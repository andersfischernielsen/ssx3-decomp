# Phase 2 candidates, round 3: the next catalogue

_Compiled 2026-10-15, after round 2 (147 replacements, byte-identical race
replay, 140 tests). Mechanism, contract and the loop/call patterns are in
[`phase2-overrides.md`](phase2-overrides.md); the round-1 catalogue, its rules
and the Tier A/B/C definitions are in
[`phase2-candidates.md`](phase2-candidates.md). This file does not repeat the
rules: every package below follows "Rules that apply to every package" there._

Round 1 could only offer leaves without loops (Tier A) because loops and calls
had no pattern yet. Both exist now (`util.rs`: structured loops with
back-edge event checks and re-enterable `match pc` dispatchers;
`dispatch.rs`: `Flow`, `Callee`, `call_ptr`, `VSlot`, `Frame`). The naming
pass 3 (`naming.md`) took the named functions from 4,473 to 9,715 of 11,844
Ghidra functions. So this catalogue mixes leaves, loops and calls, and groups
functions by subsystem so that each package defines one coherent set of typed
views.

## How the data was made

The round-1 classifier, rebuilt (throwaway script, about 150 lines; see
§ Reproducing). Differences from round 1:

- The generated code now calls through the readable aliases
  (`c.pc = 0x00111408; RiderBase_UpdateMainState(c);`), so direct calls are
  recognised by the `c.pc = 0xTARGET; <ident>(c);` pair, not by `f_X(c)`. The
  target address is the `c.pc` value.
- Function bodies are cut by brace counting. Structured bodies are not
  indented, so "the first `}` in column 0" (an easy mistake) ends a function
  at its first labelled block.
- Replaced functions (the 147 in `crates/ssx3-game/overrides.txt`) are
  classified from their `__generated` body and then **excluded** from every
  list below; their callers are flagged instead ("replaced callees").
- New per-function flags: VU0 ACC use (`adda/suba/mula/madda/msuba/opmula`),
  `cfc2 x, vi22` (Q read-back), `vcallms` (VU0 micro program), stores to the
  stack (`sp`) in leaves, number of event checks.
- Names: `generated/functions.tsv` plus the `///` doc comments (pass 3).
- Profile: the same race profile (`scratchpad/prof/sample.txt`, 2026-10-14,
  macOS `sample` at 1 ms, scripted Snow Jam race). Inclusive samples per guest
  function, recursion counted once. Race flag P / c / c2 as in round 1. The
  numbers reproduce round 1 exactly (677 profiled functions, 365 `__entries`
  twins, 13,306 functions), which validates the rebuilt script.
- Semantics: Ghidra decompilation (read-only) and the generated Rust, one
  researcher per package; field names from `structs/*.md`.

## Summary statistics (after round 2)

Counts exclude the 147 replaced functions. **P** = in the race profile;
**P/c/c2** = in it or up to two call levels below a profiled function.

| set | functions | named | P | P/c/c2 |
|---|---:|---:|---:|---:|
| all remaining generated EE functions | 13159 | 9568 | 643 | 2107 |
| with an `__entries` twin (not overridable today) | 365 | 271 | 35 | 73 |
| leaf (no calls of any kind) | 3306 | 1365 | 89 | 477 |
| leaf, no loop | 2730 | 926 | 44 | 323 |
| leaf with loops | 576 | 439 | 45 | 154 |
| non-leaf | 9853 | 8203 | 554 | 1630 |
| non-leaf, direct calls only, no loop | 5583 | 4481 | 222 | 800 |
| non-leaf with loops | 2546 | 2218 | 235 | 520 |
| with an indirect call (`jalr`) | 2434 | 2133 | 199 | 467 |
| with a syscall | 84 | 82 | 6 | 26 |
| using VU0 (macro ops or COP2 moves) | 574 | 433 | 154 | 272 |
| … using the VU0 accumulator (ACC) | 358 | 274 | 97 | 176 |
| … reading Q through `cfc2 vi22` | 154 | 122 | 48 | 91 |
| … starting a VU0 micro program (`vcallms`) | 13 | 8 | 10 | 12 |
| using MMI (128-bit integer) | 40 | 34 | 11 | 18 |
| using the FPU | 2472 | 1832 | 327 | 856 |
| calling at least one replaced function | 2939 | 2479 | 181 | 550 |
| **all direct callees replaced, no `jalr`, no syscall** | 672 | 508 | 20 | 115 |
| named, ≤ 256 B, no `__entries`, on the race path (P/c/c2) | 1038 | 1038 | 248 | 1038 |
| 8-byte stubs (`jr ra; li v0, K` and similar) | 1057 | 71 | 7 | 39 |

What changed since round 1:

- Naming is no longer the bottleneck: 1,038 named functions of ≤ 256 bytes sit
  on the race path, against ~100 in round 1. The packages below take ~200 of
  them, chosen for coherence and for what they unlock.
- 672 functions now call only replaced functions (plus arithmetic). They are
  the cheapest "calls" candidates: every callee already has a typed Rust
  signature.
- The VU0 accumulator is the most common remaining VU0 obstacle (358
  functions, 97 profiled). ACC needs a small `ps2f` addition (§ Shared
  prerequisites); package D3 builds it.
- `__entries` blocks only 365 functions, but they include the main rider
  dispatchers and `AI_Update`. § `__entries` shows that 340 of them are false
  positives the generator can remove.

## Package overview

Each package is about 25 functions in one subsystem, with one owner. Kinds:
**leaf**, **loop** (event checks, `util.rs` pattern), **calls N** (N direct
call sites, `dispatch.rs` pattern), **virtual N** (`jalr` through a vtable or
function pointer: `VSlot`/`call_ptr`), **VU0** (+ **ACC** / **Q** when the
accumulator or a `cfc2 vi22` read-back is involved), **FPU**, **MMI**.
**cycles**: one number = one block; `≤N (K)` = sum of K blocks
(path-dependent; the per-function gotchas give the paths). **prof**:
inclusive race-profile samples. **callers**: direct call sites.
**replaced callees**: how many of its callees are already hand-written.

| package | functions | leaf | loop | calls | virtual | VU0 | FPU | in race profile | first typed views |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| D1 Rider frame doors & state handlers | 25 | 2 | 0 | 23 | 2 | 0 | 7 | 10 | `Rider`, `RiderBase`, state ids |
| D2 Skeleton, animation & VU0 math | 24 | 15 | 9 | 9 | 1 | 11 | 13 | 14 | `RiderModel`, bone table, `AnimController`, `Mat4`/`Quat` |
| D3 Chase camera (round-1 Tier B leftovers) | 24 | 3 | 3 | 18 | 9 | 15 | 19 | 8 | `ChaseCameraAlgorithm`, `ChaseCamController`, VU0 `Acc` |
| D4 AI, BE interfaces & race logic | 24 | 5 | 4 | 18 | 4 | 1 | 4 | 8 | `BELibrary` interfaces, `AI`, `ComputerActorRider` |
| D5 World, sections & process lists | 25 | 7 | 13 | 14 | 12 | 0 | 3 | 22 | `ProcessList`, `ModObj`, `ModifierBlock`, `World` |
| D6 Frontend UI, fonts & HUD | 25 | 8 | 13 | 15 | 6 | 0 | 13 | 13 | `UIStateMgr`, `UIState`, `Font`, HUD element |
| D7 Audio system | 25 | 5 | 7 | 19 | 3 | 0 | 4 | 22 | SND channel/handle, `BankMonitor`, `SSXAudioSystem` |
| D8 Memory, containers, replay & script | 24 | 9 | 5 | 15 | 0 | 0 | 0 | 10 | `VramHeap` node, MEM heap, `EAMutex`, `Replay` |

The packages are independent: each depends only on round-1/2 code. D3 adds
the VU0 accumulator helper, which D2 can use too. Suggested start order when
fewer than eight people are available: D5 and D1 (hottest doors; D5 has 22
profiled functions), then D2 (the hottest VU0 leaf left), D3, D7, D6, D4, D8.

> Assembly note (2026-10-15): the catalogue agent's researchers completed
> semantic notes for D1, D4 and D7; for the other packages the function lists
> below come from the classifier (kind, block costs, callees, callers, flags)
> and package authors research semantics themselves, as in rounds 1-2. Since
> commit 10794a5 functions with `__entries` twins **can** be overridden (the
> stub routes non-start entries to the generated twin), so the `__entries`
> restriction mentioned above no longer applies.


<!-- D1 Rider frame doors & state handlers -->
| addr | name | size | cycles | kind | race | prof | callers | replaced callees |
|---|---|---:|---|---|---|---:|---:|---:|
| 0x11eb60 | `Rider_TickAnimation` | 52 | 13 | calls 2 | P | 136 | 5 | 0 |
| 0x121700 | `Rider_UpdateAnimation` | 40 | ≤10 (3) | calls 1, FPU | P | 136 | 1 | 0 |
| 0x121728 | `Rider_UpdatePose` | 40 | ≤10 (3) | calls 1 | P | 106 | 1 | 0 |
| 0x1217f8 | `Rider_UpdateModel` | 28 | 7 | calls 1 | P | 41 | 1 | 0 |
| 0x1225f0 | `Rider_RenderShadow` | 40 | ≤10 (3) | calls 1 | P | 21 | 1 | 0 |
| 0x1225c0 | `Rider_PrepareLighting` | 44 | ≤11 (3) | calls 1 | P | 8 | 1 | 0 |
| 0x122638 | `Rider_RenderAttachedFX` | 28 | 7 | calls 1 | c | 0 | 1 | 0 |
| 0x120e88 | `Rider_UpdateAttachedFX` | 48 | 12 | calls 2 | P | 1 | 1 | 0 |
| 0x1218d0 | `Rider_UpdateSurfaceFX` | 128 | ≤32 (3) | calls 3, virtual 1, FPU | P | 28 | 1 | 0 |
| 0x121818 | `Rider_UpdateProgress` | 180 | ≤45 (9) | calls 7, virtual 1 | P | 24 | 1 | 1 |
| 0x1112b8 | `RiderBase_SetMainState` | 64 | 16 | calls 2 | c2 | 0 | 1 | 0 |
| 0x111538 | `RiderBase_SetSubState` | 64 | 16 | calls 2 | c2 | 0 | 1 | 0 |
| 0x136e98 | `RiderWipeoutState_Update` | 72 | ≤18 (4) | calls 3 | c | 0 | 1 | 0 |
| 0x136ee0 | `RiderWipeoutState_PostUpdate` | 72 | ≤18 (4) | calls 3 | c | 0 | 1 | 0 |
| 0x139528 | `RiderHandplantState_PostUpdate` | 32 | 8 | calls 1 | c | 0 | 1 | 0 |
| 0x131cc0 | `RiderCruiseSub_CheckAirborne` | 68 | ≤17 (3) | calls 2 | c | 0 | 1 | 2 |
| 0x1231a8 | `Rider_IsOnGround` | 100 | ≤25 (6) | calls 2 | c | 0 | 7 | 1 |
| 0x116378 | `Rider_TryEnterFinishLine` | 56 | ≤14 (3) | calls 1, FPU | c | 0 | 4 | 1 |
| 0x116120 | `Rider_CheckResetPath` | 172 | ≤43 (7) | calls 6 | c | 0 | 20 | 2 |
| 0x1162c8 | `Rider_TryEnterPrewind` | 176 | ≤44 (10) | calls 4, FPU | c | 0 | 3 | 2 |
| 0x120ed8 | `Rider_RefreshPose` | 72 | ≤18 (3) | calls 3, FPU | c | 0 | 1 | 1 |
| 0x122088 | `Rider_FindNearbyLights` | 76 | 19 | calls 2 | P | 1 | 4 | 1 |
| 0x122658 | `Rider_ClearNearbyLights` | 36 | 9 | calls 1 | c2 | 0 | 1 | 1 |
| 0x120090 | `Rider_SetAnimTimeScale` | 60 | ≤15 (4) | leaf, FPU | c | 0 | 1 | 0 |
| 0x1360c8 | `RiderSpinSub_GetHUDAngle` | 52 | ≤13 (5) | leaf, FPU | c | 0 | 1 | 0 |

summary D1 Rider frame doors & state handlers: n=25 leaf=2 loop=0 calls=23 virt=2 vu=0 fpu=7 P=10 prof=502

<!-- D2 Skeleton, animation & VU0 math -->
| addr | name | size | cycles | kind | race | prof | callers | replaced callees |
|---|---|---:|---|---|---|---:|---:|---:|
| 0x310120 | `RiderModel_BuildBoneMatrix` | 220 | 55 | leaf, VU0 ACC | P | 37 | 1 | 0 |
| 0x3103f0 | `RiderModel_BuildBoneMatrices` | 100 | ≤25 (4) | loop, calls 1 | P | 41 | 6 | 0 |
| 0x310200 | `RiderModel_ApplyPose` | 492 | ≤123 (10) | leaf, loop, VU0 ACC, FPU | P | 62 | 6 | 0 |
| 0x310530 | `RiderModel_Translate` | 272 | ≤68 (7) | leaf, loop, VU0 | c | 0 | 1 | 0 |
| 0x3123c0 | `AnimController_Update` | 204 | ≤51 (7) | loop, calls 2, virtual 1, FPU | P | 134 | 2 | 0 |
| 0x312598 | `AnimController_EvaluatePose` | 196 | ≤49 (5) | loop, calls 2 | P | 132 | 7 | 0 |
| 0x37de88 | `FUN_0037de88` | 88 | ≤22 (3) | leaf, VU0 | P | 52 | 0 | 0 |
| 0x37dee0 | `PS2GraphicsMan_ProjectPoint` | 96 | 24 | leaf, VU0 ACC | P | 1 | 1 | 0 |
| 0x31b6c8 | `Mat4_Inverse` | 124 | 31 | calls 2, VU0, FPU | P | 2 | 1 | 0 |
| 0x31b748 | `Mat4_ToPosQuat` | 96 | 24 | calls 1, FPU | c | 0 | 9 | 0 |
| 0x31b088 | `Math_Det2x2` | 16 | 4 | leaf, FPU | c2 | 0 | 3 | 0 |
| 0x31b098 | `Math_Det3x3` | 224 | 56 | calls 3, FPU | c | 0 | 20 | 0 |
| 0x31bb30 | `Quat_FromTwoVectors` | 380 | ≤95 (7) | calls 3, VU0 ACC Q, FPU | P | 2 | 1 | 2 |
| 0x31b7a8 | `Quat_FromMatrix` | 520 | ≤130 (10) | leaf, FPU | c2 | 0 | 1 | 0 |
| 0x11dfe0 | `Rider_RotateAroundAxis` | 184 | 46 | calls 2, VU0 ACC, FPU | P | 1 | 10 | 2 |
| 0x36d400 | `Vu0_RandNext` | 36 | 9 | leaf, VU0, FPU | c2 | 0 | 9 | 0 |
| 0x32c590 | `SphereTree_GetRadius` | 24 | 6 | leaf, FPU | - | 0 | 6 | 0 |
| 0x32c630 | `SphereTree_Translate` | 24 | 6 | leaf, VU0 | - | 0 | 1 | 0 |
| 0x32c5e0 | `SphereTree_SetPosition` | 80 | 20 | leaf, VU0, FPU | - | 0 | 0 | 0 |
| 0x329910 | `RiderSphereTree_ctor` | 36 | 9 | leaf | P | 1 | 3 | 0 |
| 0x329ae0 | `RiderSphereTree_UpdateFromBones` | 96 | ≤24 (4) | leaf, loop | P | 1 | 1 | 0 |
| 0x314760 | `AnimSeqList_GetAt` | 36 | ≤9 (5) | leaf, loop | c | 0 | 11 | 0 |
| 0x3145f8 | `AnimSeqList_Remove` | 112 | ≤28 (7) | loop, calls 2 | P | 1 | 15 | 1 |
| 0x26ac48 | `Path_GetLength` | 60 | ≤15 (4) | leaf, loop, FPU | P | 3 | 5 | 0 |

summary D2 Skeleton, animation & VU0 math: n=24 leaf=15 loop=9 calls=9 virt=1 vu=11 fpu=13 P=14 prof=470

<!-- D3 Chase camera (Tier B leftovers) -->
| addr | name | size | cycles | kind | race | prof | callers | replaced callees |
|---|---|---:|---|---|---|---:|---:|---:|
| 0x168150 | `ChaseCameraAlgorithm_CheckDistanceReset` | 328 | ≤82 (5) | virtual 1, VU0 ACC Q, FPU | P | 3 | 14 | 0 |
| 0x166550 | `ChaseCameraAlgorithm_InitFromRider` | 236 | 59 | virtual 1, VU0 ACC Q, FPU | - | 0 | 15 | 0 |
| 0x15d880 | `CameraBase_GetPitch` | 164 | 41 | calls 1, VU0 ACC Q, FPU | - | 0 | 0 | 1 |
| 0x163010 | `ChaseCameraAlgorithm_ApplyFollowDistance` | 328 | ≤82 (7) | calls 1, VU0 ACC Q, FPU | P | 2 | 7 | 0 |
| 0x162b90 | `ChaseCameraAlgorithm_ApplyTargetHeight` | 228 | ≤57 (6) | calls 1, VU0, FPU | P | 1 | 7 | 0 |
| 0x163158 | `ChaseCameraAlgorithm_ApplySideOffset` | 280 | ≤70 (3) | calls 1, VU0 ACC Q, FPU | - | 0 | 3 | 0 |
| 0x163270 | `ChaseCameraAlgorithm_ApplyCameraHeight` | 316 | ≤79 (11) | calls 1, VU0, FPU | c | 0 | 7 | 0 |
| 0x1633b0 | `ChaseCameraAlgorithm_ApplyFov` | 160 | ≤40 (5) | calls 1, FPU | c | 0 | 7 | 0 |
| 0x163450 | `ChaseCameraAlgorithm_ApplyHandplantHold` | 364 | ≤91 (12) | calls 1, VU0, FPU | c | 0 | 5 | 0 |
| 0x1641c0 | `ChaseCameraAlgorithm_ApplyRailPullback` | 484 | ≤121 (5) | calls 1, virtual 1, VU0 ACC Q, FPU | c | 0 | 5 | 0 |
| 0x1643a8 | `ChaseCameraAlgorithm_ApplyLowSpeedHold` | 760 | ≤190 (28) | calls 1, VU0 ACC Q, FPU | P | 1 | 6 | 0 |
| 0x1646a0 | `ChaseCameraAlgorithm_ApplyRailAlign` | 452 | ≤113 (6) | calls 1, VU0 ACC Q, FPU | P | 2 | 6 | 0 |
| 0x162998 | `ChaseCameraAlgorithm_UpdateHandplantFade` | 136 | ≤34 (9) | leaf, FPU | c | 0 | 6 | 0 |
| 0x162a20 | `ChaseCameraAlgorithm_UpdateAirTransitions` | 348 | ≤87 (16) | calls 2, virtual 1, VU0 ACC Q, FPU | c | 0 | 7 | 0 |
| 0x166228 | `ChaseCameraAlgorithm_PostUpdate` | 116 | ≤29 (3) | calls 4, virtual 1, FPU | P | 2 | 0 | 0 |
| 0x166640 | `ChaseCameraAlgorithm_ComputeAngles` | 632 | ≤158 (15) | calls 3, VU0 ACC Q, FPU | c | 0 | 4 | 2 |
| 0x161bb8 | `ChaseCamController_Update` | 672 | ≤168 (37) | loop, calls 3, virtual 2, FPU | P | 13 | 0 | 0 |
| 0x161ab0 | `ChaseCamController_UpdateAutoMode` | 260 | ≤65 (10) | calls 4, virtual 1, VU0 ACC Q, FPU | c | 0 | 1 | 1 |
| 0x15d020 | `ChaseCamController_IsScriptCameraActive` | 44 | ≤11 (4) | leaf | c | 0 | 1 | 0 |
| 0x162060 | `ChaseCamController_SetOverrideAlgorithm` | 112 | ≤28 (8) | calls 1 | c | 0 | 5 | 0 |
| 0x22e840 | `GameViewMan_UpdateCameras` | 116 | ≤29 (4) | loop, virtual 1 | P | 33 | 9 | 0 |
| 0x22e920 | `GameViewMan_AllViewsReady` | 68 | ≤17 (6) | leaf, loop | c | 0 | 6 | 0 |
| 0x15e460 | `GameCamera_UpdateShake` | 520 | ≤130 (18) | calls 5, virtual 2, VU0 ACC Q, FPU | c | 0 | 1 | 1 |
| 0x15e030 | `GameCamera_ResetDefaultChase` | 28 | 7 | calls 1 | - | 0 | 3 | 1 |

summary D3 Chase camera (Tier B leftovers): n=24 leaf=3 loop=3 calls=18 virt=9 vu=15 fpu=19 P=8 prof=57

<!-- D4 AI, BE interfaces & race logic -->
| addr | name | size | cycles | kind | race | prof | callers | replaced callees |
|---|---|---:|---|---|---|---:|---:|---:|
| 0x14dc80 | `BELibrary_GetInstance` | 200 | ≤50 (3) | calls 6, virtual 1 | P | 1 | 722 | 1 |
| 0x1448d8 | `BENewRaceInt_GetInstance` | 76 | ≤19 (3) | calls 1 | c | 0 | 7 | 1 |
| 0x145970 | `BENewPlayerInt_GetInstance` | 68 | ≤17 (3) | calls 2 | c | 0 | 13 | 1 |
| 0x147f78 | `BECharInt_GetInstance` | 92 | ≤23 (3) | calls 1, virtual 1 | P | 1 | 9 | 1 |
| 0x155a50 | `BEAggressionInt_GetInstance` | 76 | ≤19 (3) | calls 1 | c | 0 | 17 | 1 |
| 0x154948 | `BEScoreInt_GetInstance` | 92 | ≤23 (3) | calls 2 | c | 0 | 7 | 1 |
| 0x14f250 | `BEOptionInt_GetInstance` | 84 | ≤21 (3) | calls 2 | P | 1 | 5 | 1 |
| 0x146e98 | `BENewPlayerInt_GetHumanRider` | 168 | ≤42 (9) | loop, calls 1 | P | 1 | 87 | 1 |
| 0x147398 | `BENewPlayerInt_GetCharacter` | 52 | 13 | calls 1 | c | 0 | 32 | 1 |
| 0x147290 | `BENewPlayerInt_GetPadIndex` | 52 | 13 | calls 1 | c | 0 | 3 | 1 |
| 0x144c98 | `BENewRaceInt_GetTrackPeak` | 36 | 9 | leaf | c | 0 | 8 | 0 |
| 0x148680 | `BECharInt_GetAttrib3` | 96 | 24 | calls 2 | P | 1 | 2 | 2 |
| 0x148d80 | `BECharInt_GetAttrib3Factor` | 228 | ≤57 (7) | calls 3, FPU | P | 1 | 1 | 2 |
| 0x155b50 | `BEAggressionInt_GetRivalLevel` | 160 | ≤40 (4) | calls 5 | P | 2 | 6 | 2 |
| 0x12a250 | `AI_AllPlayersFinished` | 92 | ≤23 (8) | leaf, loop, FPU | c | 0 | 15 | 0 |
| 0x12a180 | `AI_AreAllRidersLoaded` | 112 | ≤28 (4) | loop, calls 1 | c | 0 | 5 | 1 |
| 0x1298c8 | `AI_GetFrameCounter` | 20 | 5 | leaf | c | 0 | 32 | 0 |
| 0x113c20 | `AI_UpdateRaceState` | 148 | ≤37 (5) | virtual 3 | c | 0 | 1 | 0 |
| 0x10f878 | `AI_UpdateRivals` | 288 | ≤72 (14) | loop, calls 2 | c | 0 | 1 | 0 |
| 0x10a960 | `ComputerActorRider_ThinkCruise` | 272 | ≤68 (14) | calls 3, virtual 1 | P | 7 | 1 | 0 |
| 0x10b750 | `ComputerActorRider_ThinkWipeout` | 64 | 16 | calls 1 | c | 0 | 1 | 0 |
| 0x100610 | `ComputerActorRider_GetSteerTarget` | 108 | 27 | leaf, VU0, FPU | c | 0 | 8 | 0 |
| 0x10deb0 | `ComputerActorRider_GetHumanRider` | 64 | ≤16 (4) | calls 1 | c2 | 0 | 3 | 0 |
| 0x113618 | `AirPredictor_SetLaunchState` | 48 | 12 | leaf, FPU | c | 0 | 3 | 0 |

summary D4 AI, BE interfaces & race logic: n=24 leaf=5 loop=4 calls=18 virt=4 vu=1 fpu=4 P=8 prof=15

<!-- D5 World, sections & process lists -->
| addr | name | size | cycles | kind | race | prof | callers | replaced callees |
|---|---|---:|---|---|---|---:|---:|---:|
| 0x3550a0 | `ProcessList_RenderPriority` | 116 | ≤29 (5) | loop, calls 2, virtual 1 | P | 90 | 2 | 0 |
| 0x354f98 | `ProcessList_UpdatePriority` | 144 | ≤36 (5) | loop, calls 3, virtual 1 | P | 50 | 16 | 0 |
| 0x355028 | `ProcessList_PostUpdatePriority` | 116 | ≤29 (5) | loop, calls 2, virtual 1 | P | 9 | 2 | 0 |
| 0x354ef8 | `ProcessList_Next` | 36 | 9 | leaf | P | 2 | 10 | 0 |
| 0x354f48 | `ProcessList_PendingFirst` | 36 | 9 | leaf | P | 1 | 1 | 0 |
| 0x354c98 | `ProcessList_MergePending` | 428 | ≤107 (15) | loop, calls 4, virtual 3 | P | 2 | 10 | 0 |
| 0x230640 | `GameModule_RenderProcessLists` | 88 | ≤22 (3) | loop, calls 1 | P | 90 | 1 | 0 |
| 0x22c078 | `GameModule_RenderModels` | 312 | ≤78 (7) | loop, virtual 2 | P | 91 | 1 | 0 |
| 0x22c620 | `GameModule_RenderBezierPatches` | 228 | ≤57 (9) | loop, calls 4, virtual 1 | P | 53 | 1 | 0 |
| 0x22c3b8 | `GameModule_EndBezierPass` | 88 | ≤22 (3) | calls 1, virtual 1 | P | 41 | 1 | 1 |
| 0x352dd0 | `ModifierBlock_Render` | 124 | ≤31 (6) | loop, virtual 2 | P | 38 | 1 | 0 |
| 0x352d20 | `ModifierBlock_PostUpdate` | 172 | ≤43 (9) | loop, virtual 3 | P | 17 | 1 | 0 |
| 0x352c70 | `ModifierBlock_Update` | 176 | ≤44 (10) | calls 3, virtual 3 | P | 2 | 1 | 0 |
| 0x352b88 | `ModifierBlock_GetTransformBounds` | 56 | ≤14 (4) | virtual 1 | P | 1 | 5 | 0 |
| 0x356198 | `ModObj_Update` | 252 | ≤63 (11) | calls 5, virtual 5 | P | 28 | 1 | 0 |
| 0x355748 | `ModObj_PostUpdateModifiers` | 40 | ≤10 (3) | calls 1 | P | 17 | 1 | 0 |
| 0x3556f8 | `ModObj_UpdateModifiers` | 80 | ≤20 (4) | calls 2 | P | 2 | 1 | 0 |
| 0x3a6928 | `World_Update` | 28 | 7 | calls 1 | P | 3 | 5 | 0 |
| 0x3a8290 | `cWorldCache_Update` | 56 | 14 | calls 3 | P | 3 | 1 | 0 |
| 0x2d1be0 | `World_GetNash` | 16 | 4 | leaf | c | 0 | 23 | 0 |
| 0x2d1c70 | `World_GetSecondsPerTick` | 12 | 3 | leaf, FPU | c | 0 | 16 | 0 |
| 0x22e0e0 | `SectionMan_FindLogicalSection` | 56 | ≤14 (5) | leaf, loop | c | 0 | 5 | 0 |
| 0x2c1cd8 | `QuadTree_FindLeaf` | 208 | ≤52 (7) | leaf, loop, FPU | P | 1 | 1 | 0 |
| 0x2d1928 | `Noise_Fractal1D` | 144 | ≤36 (4) | loop, calls 1, FPU | P | 1 | 2 | 1 |
| 0x22a5a0 | `Nash_CollectInstances` | 244 | ≤61 (15) | leaf, loop | P | 11 | 1 | 0 |

summary D5 World, sections & process lists: n=25 leaf=7 loop=13 calls=14 virt=12 vu=0 fpu=3 P=22 prof=553

<!-- D6 Frontend UI, fonts & HUD -->
| addr | name | size | cycles | kind | race | prof | callers | replaced callees |
|---|---|---:|---|---|---|---:|---:|---:|
| 0x39f9d8 | `UIStateMgr_FindState` | 172 | ≤43 (12) | loop, calls 4 | P | 1 | 20 | 2 |
| 0x39fa88 | `UIStateMgr_GetActiveState` | 96 | ≤24 (6) | loop, calls 2 | P | 3 | 11 | 2 |
| 0x39ebe0 | `UIStateMgr_Render` | 196 | ≤49 (12) | loop, calls 4, virtual 2 | P | 57 | 1 | 2 |
| 0x39f100 | `UIStateMgr_Update` | 144 | ≤36 (9) | loop, calls 5, virtual 1 | P | 3 | 1 | 2 |
| 0x39e868 | `UIState_UpdateScreens` | 76 | ≤19 (4) | loop, calls 3 | c | 0 | 4 | 2 |
| 0x39eb60 | `UIStateMgr_HasStateInPhase` | 128 | ≤32 (8) | loop, calls 2 | c | 0 | 1 | 2 |
| 0x39ea90 | `UIStateMgr_RestartStateInPhase` | 204 | ≤51 (8) | loop, calls 2 | c | 0 | 2 | 2 |
| 0x39f840 | `UIStateMgr_CloseAll` | 136 | ≤34 (5) | loop, calls 2 | c | 0 | 10 | 2 |
| 0x39f7b8 | `UIStateMgr_SetParentChainVisible` | 136 | ≤34 (10) | leaf, loop | c | 0 | 2 | 0 |
| 0x398038 | `UIEngine_Update` | 64 | 16 | calls 1, virtual 1 | P | 4 | 10 | 0 |
| 0x397df8 | `UIEngine_Render` | 540 | 135 | calls 1, virtual 6, FPU | P | 57 | 6 | 0 |
| 0x391cb0 | `Font_DrawText` | 384 | ≤96 (4) | virtual 2, FPU | P | 10 | 50 | 0 |
| 0x391fb0 | `Font_MeasureText` | 572 | ≤143 (24) | leaf, loop, FPU | P | 1 | 60 | 0 |
| 0x392908 | `Font_DrawTextCentered` | 120 | 30 | calls 2, FPU | P | 2 | 15 | 0 |
| 0x1e9290 | `HUD_GetElemScale` | 20 | 5 | leaf, FPU | P | 1 | 38 | 0 |
| 0x1e91a8 | `HUD_GetElemColor` | 36 | 9 | leaf, FPU | c | 0 | 40 | 0 |
| 0x1e91f8 | `HUD_GetElemPos` | 36 | 9 | leaf, FPU | c | 0 | 42 | 0 |
| 0x1e9220 | `HUD_GetElemSize` | 108 | ≤27 (8) | leaf, FPU | c | 0 | 32 | 0 |
| 0x1f10f8 | `HUD_AlignElemPos` | 148 | 37 | calls 2, FPU | P | 1 | 37 | 0 |
| 0x1f1190 | `HUD_DrawSprite` | 420 | ≤105 (7) | loop, virtual 1, FPU | P | 1 | 25 | 0 |
| 0x117008 | `cHUDFX_SetText` | 60 | ≤15 (5) | leaf, loop | c | 0 | 4 | 0 |
| 0x117048 | `cHUDFX_Show` | 96 | 24 | calls 1, FPU | c2 | 0 | 1 | 0 |
| 0x1171a8 | `cHUDFX_ShowNumberDelayed` | 156 | 39 | calls 2, FPU | P | 3 | 24 | 0 |
| 0x1179e0 | `RiderMetrix_RemoveHUDMessage` | 120 | ≤30 (8) | leaf, loop | c | 0 | 43 | 0 |
| 0x117b88 | `RiderMetrix_ShowHUDNumber` | 156 | ≤39 (4) | calls 2, FPU | c | 0 | 29 | 0 |

summary D6 Frontend UI, fonts & HUD: n=25 leaf=8 loop=13 calls=15 virt=6 vu=0 fpu=13 P=13 prof=144

<!-- D7 Audio system -->
| addr | name | size | cycles | kind | race | prof | callers | replaced callees |
|---|---|---:|---|---|---|---:|---:|---:|
| 0x3b8790 | `SND_SetPitch` | 164 | ≤41 (8) | loop, calls 4 | P | 4 | 4 | 1 |
| 0x3b8c00 | `SND_SetPan` | 200 | ≤50 (9) | loop, calls 3 | P | 1 | 4 | 1 |
| 0x3baa00 | `SND_NextHandleChannel` | 192 | ≤48 (14) | leaf, loop | P | 1 | 14 | 0 |
| 0x3ba938 | `SND_ChannelCalcPitch` | 196 | ≤49 (9) | calls 1 | P | 1 | 5 | 0 |
| 0x3b5860 | `SND_DefaultLock` | 28 | 7 | calls 1 | P | 3 | 0 | 0 |
| 0x3b5880 | `SND_DefaultUnlock` | 28 | 7 | calls 1 | P | 3 | 0 | 0 |
| 0x3c3380 | `SND_MutexLock` | 96 | ≤24 (3) | calls 2 | P | 3 | 1 | 0 |
| 0x3c33e0 | `SND_MutexUnlock` | 112 | ≤28 (6) | calls 2 | P | 2 | 1 | 0 |
| 0x3b5540 | `SND_Service` | 172 | ≤43 (9) | loop, calls 1 | P | 17 | 1 | 0 |
| 0x2ab958 | `SSXAudioSystem_UpdateBanksAndStreams` | 136 | 34 | calls 5, virtual 1, FPU | P | 35 | 1 | 0 |
| 0x2ab7a0 | `SSXAudioSystem_UpdateBankMonitor` | 132 | ≤33 (5) | loop, calls 1, FPU | P | 30 | 0 | 0 |
| 0x2ab6b0 | `SSXAudioSystem_UpdateBankMonitorInstance` | 236 | ≤59 (14) | calls 2, virtual 1, FPU | P | 29 | 1 | 0 |
| 0x2ab8e8 | `SSXAudioSystem_UpdateTimers` | 112 | ≤28 (4) | loop, calls 2 | P | 1 | 0 | 0 |
| 0x2abb80 | `BankMonitorInstance_PollFree` | 88 | ≤22 (6) | calls 2 | P | 6 | 8 | 0 |
| 0x2abc18 | `BankMonitor_CalcFalloffVolume` | 100 | ≤25 (5) | leaf, FPU | P | 1 | 1 | 0 |
| 0x2abc80 | `BankMonitorInstance_GetFalloffVolume` | 28 | 7 | calls 1 | P | 1 | 1 | 0 |
| 0x2ad550 | `BankMonitor_FindInstance` | 156 | ≤39 (9) | loop, calls 1 | P | 1 | 12 | 0 |
| 0x2abe78 | `BankMonitorInstance_SetPitch` | 232 | ≤58 (11) | calls 7 | P | 8 | 8 | 2 |
| 0x2ad998 | `BankMonitor_SndTickCallback` | 44 | 11 | virtual 1 | P | 2 | 0 | 0 |
| 0x2b4388 | `MusicPlayer_Update` | 76 | ≤19 (4) | calls 2 | P | 2 | 3 | 0 |
| 0x2b2488 | `SongInstance_GetStreamHandle` | 52 | ≤13 (3) | calls 2 | P | 1 | 8 | 0 |
| 0x287968 | `SSXAudioSystem_GetChannelGainPtr` | 52 | ≤13 (5) | leaf | c | 0 | 76 | 0 |
| 0x28b1d8 | `SSXAudioSystem_GetRaceMan` | 16 | 4 | leaf | c | 0 | 51 | 0 |
| 0x28b1c8 | `SSXAudioSystem_GetGameModule` | 12 | 3 | leaf | c | 0 | 24 | 0 |
| 0x288ae0 | `SSXAudioSystem_IsReplayPlaying` | 96 | ≤24 (7) | calls 3 | P | 2 | 60 | 0 |

summary D7 Audio system: n=25 leaf=5 loop=7 calls=19 virt=3 vu=0 fpu=4 P=22 prof=154

<!-- D8 Memory, containers, replay & script -->
| addr | name | size | cycles | kind | race | prof | callers | replaced callees |
|---|---|---:|---|---|---|---:|---:|---:|
| 0x366ce0 | `VramHeap_Alloc` | 248 | ≤62 (7) | loop, calls 4 | P | 14 | 6 | 0 |
| 0x3660a8 | `VramHeap_LinkFreeBlock` | 396 | ≤99 (23) | leaf, loop, MMI | P | 6 | 5 | 0 |
| 0x366238 | `VramHeap_UnlinkSizeList` | 148 | ≤37 (8) | leaf | P | 1 | 4 | 0 |
| 0x3667f8 | `VramHeap_FindFreeBlock` | 140 | ≤35 (10) | leaf, loop, MMI | c | 0 | 1 | 0 |
| 0x3662d0 | `VramHeap_SplitBlock` | 260 | ≤65 (7) | calls 4 | P | 5 | 1 | 0 |
| 0x3663d8 | `VramHeap_FreeBlock` | 368 | ≤92 (14) | loop, calls 6 | P | 3 | 9 | 0 |
| 0x3668f8 | `VramHeap_EvictCheapestBlock` | 248 | ≤62 (15) | loop, calls 1, MMI | P | 8 | 1 | 0 |
| 0x365fe8 | `VramHeap_PopSpareNode` | 32 | 8 | leaf | c | 0 | 1 | 0 |
| 0x366008 | `VramHeap_RemoveNode` | 160 | ≤40 (8) | leaf | c | 0 | 1 | 0 |
| 0x319a90 | `MEM_AllocInternal` | 184 | ≤46 (13) | calls 4 | P | 1 | 2 | 0 |
| 0x319b48 | `MEM_FreeInternal` | 80 | ≤20 (6) | calls 3 | c | 0 | 3 | 0 |
| 0x317e98 | `EA_MemFree` | 72 | 18 | calls 3 | c | 0 | 345 | 0 |
| 0x2523a8 | `MEM_Free` | 36 | ≤9 (3) | calls 1 | c | 0 | 34 | 0 |
| 0x3e5700 | `EAMutex_Lock` | 96 | ≤24 (4) | calls 3 | c | 0 | 49 | 0 |
| 0x3e5760 | `EAMutex_Unlock` | 108 | ≤27 (8) | calls 3 | c | 0 | 50 | 0 |
| 0x3e6220 | `EA_MemZero` | 32 | 8 | calls 1 | c | 0 | 7 | 1 |
| 0x26d760 | `Replay_RecordSnapshot` | 112 | 28 | calls 3 | P | 4 | 4 | 0 |
| 0x26e9d0 | `ReplayStream_Write` | 76 | 19 | calls 1 | P | 4 | 0 | 1 |
| 0x26d740 | `Replay_AddSnapshot` | 28 | 7 | calls 1 | c | 0 | 4 | 1 |
| 0x270628 | `Replay_CancelInstantReplay` | 48 | ≤12 (3) | leaf | c | 0 | 3 | 0 |
| 0x26fa50 | `Replay_StopRecording` | 36 | ≤9 (3) | leaf | c2 | 0 | 3 | 0 |
| 0x309c88 | `WScriptMan_CallFunc` | 148 | 37 | calls 5 | P | 2 | 7 | 0 |
| 0x30c820 | `WScriptMan_EventQueuePop` | 80 | ≤20 (3) | leaf | c | 0 | 5 | 0 |
| 0x30c870 | `WScriptMan_EventQueuePeek` | 40 | ≤10 (3) | leaf | c | 0 | 3 | 0 |

summary D8 Memory, containers, replay & script: n=24 leaf=9 loop=5 calls=15 virt=0 vu=0 fpu=0 P=10 prof=48

## D1: function list (classifier)

```text
# D1 Rider frame doors & state handlers

0x0011eb60 Rider_TickAnimation shard=s00 size=52 race=P prof=136 callers=5
  kind=C2 blocks=[13] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['AnimController_Update', 'AnimController_DispatchEndEvents']
  direct callers: ['Rider_Place', 'Rider_RefreshPose', 'Rider_UpdateAnimation', 'FUN_00124788', 'AI_UpdateRiderPoses']

0x00121700 Rider_UpdateAnimation shard=s00 size=40 race=P prof=136 callers=1
  kind=C1/F1 blocks=[5, 3, 2] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=1 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Rider_TickAnimation']
  direct callers: ['AI_Update']

0x00121728 Rider_UpdatePose shard=s00 size=40 race=P prof=106 callers=1
  kind=C1 blocks=[5, 3, 2] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Rider_UpdateSkeleton']
  direct callers: ['AI_Update']

0x001217f8 Rider_UpdateModel shard=s00 size=28 race=P prof=41 callers=1
  kind=C1 blocks=[7] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['RiderModel_BuildBoneMatrices']
  direct callers: ['AI_Update']

0x001225f0 Rider_RenderShadow shard=s00 size=40 race=P prof=21 callers=1
  kind=C1 blocks=[5, 3, 2] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Rider_DrawShadow']
  direct callers: ['AI_Render']

0x001225c0 Rider_PrepareLighting shard=s00 size=44 race=P prof=8 callers=1
  kind=C1 blocks=[5, 3, 3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Rider_BuildLightSet']
  direct callers: ['AI_Render']

0x00122638 Rider_RenderAttachedFX shard=s00 size=28 race=c prof=0 callers=1
  kind=C1 blocks=[7] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['AttachedEmitterFX_Render']
  direct callers: ['AI_Render']

0x00120e88 Rider_UpdateAttachedFX shard=s00 size=48 race=P prof=1 callers=1
  kind=C2 blocks=[12] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['AttachedEmitterFX_Update', 'cBoltOnFXInterface_Update']
  direct callers: ['AI_Update']

0x001218d0 Rider_UpdateSurfaceFX shard=s00 size=128 race=P prof=28 callers=1
  kind=C3/I1/F3 blocks=[8, 5, 19] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=3 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['SectionMan_FindLogicalSection', 'WorldPainter_UpdateView', 'FUN_00392d18']
  direct callers: ['AI_Update']

0x00121818 Rider_UpdateProgress shard=s00 size=180 race=P prof=24 callers=1
  kind=C7/I1 blocks=[7, 3, 9, 4, 5, 2, 7, 2, 6] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['FUN_00125ad0', 'Rider_UpdateRaceProgress', 'Rider_UpdateAIPathProgress', 'Rider_CheckRaceFinish', 'RiderMetrix_Update', 'Rider_UpdateFocusBox*', 'WScriptMan_OnObjectTrigger']
  direct callers: ['AI_Update']

0x001112b8 RiderBase_SetMainState shard=s00 size=64 race=c2 prof=0 callers=1
  kind=C2 blocks=[16] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['RiderBase_ExitMainState', 'RiderBase_EnterMainState']
  direct callers: ['Rider_SetMainState*']

0x00111538 RiderBase_SetSubState shard=s00 size=64 race=c2 prof=0 callers=1
  kind=C2 blocks=[16] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['RiderBase_ExitSubState', 'RiderBase_EnterSubState']
  direct callers: ['Rider_SetSubState*']

0x00136e98 RiderWipeoutState_Update shard=s01 size=72 race=c prof=0 callers=1
  kind=C3 blocks=[8, 4, 3, 3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['RiderWipeoutState_UpdateRootPose', 'RiderWipeoutState_UpdateGround', 'RiderWipeoutState_UpdateAir']
  direct callers: ['RiderBase_UpdateMainState']

0x00136ee0 RiderWipeoutState_PostUpdate shard=s01 size=72 race=c prof=0 callers=1
  kind=C3 blocks=[8, 4, 3, 3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['RiderWipeoutState_PostUpdateRootPose', 'RiderWipeoutState_PostUpdateGround', 'RiderWipeoutState_PostUpdateAir']
  direct callers: ['RiderBase_PostUpdateMainState']

0x00139528 RiderHandplantState_PostUpdate shard=s01 size=32 race=c prof=0 callers=1
  kind=C1 blocks=[8] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Rider_UpdateBounds']
  direct callers: ['RiderBase_PostUpdateMainState']

0x00131cc0 RiderCruiseSub_CheckAirborne shard=s00 size=68 race=c prof=0 callers=1
  kind=C2 blocks=[9, 4, 4] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Rider_GetMainState*', 'Rider_SetSubState*']
  direct callers: ['RiderCruiseSub_HandleInput']

0x001231a8 Rider_IsOnGround shard=s00 size=100 race=c prof=0 callers=7
  kind=C2 blocks=[9, 5, 4, 1, 1, 5] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Rider_GetMainState*']
  direct callers: ['Rider_HandleWallCollision', 'FUN_00106f78', 'Rider_CheckRiderCollisions', 'Rider_ResolveRiderCollision', 'Rider_UpdateHeadingAndAudio', 'FUN_00122cd0']

0x00116378 Rider_TryEnterFinishLine shard=s00 size=56 race=c prof=0 callers=4
  kind=C1/F3 blocks=[8, 3, 3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=3 fcmp=1 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Rider_SetSubState*']
  direct callers: ['RiderBumpSub_HandleInput', 'RiderPrewindSub_HandleInput', 'RiderBoardPressSub_HandleInput', 'RiderCruiseSub_HandleInput']

0x00116120 Rider_CheckResetPath shard=s00 size=172 race=c prof=0 callers=20
  kind=C6 blocks=[7, 2, 4, 3, 3, 19, 5] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['RiderMetrix_ShowWrongWay', 'Rider_SetSubState*', 'Rider_SetMainState*', 'SSXAudioSystem_GetInstance', 'SSXAudioSystem_OnRiderReset', 'Replay_OnRiderReset']
  direct callers: ['FUN_00105d98', 'FUN_0010f1c0', 'Rider_UpdateImpacts', 'RiderWipeoutSub_HandleInput', 'RiderWipeoutSub_UpdatePhase2', 'RiderWipeoutSub_UpdatePhase1', 'RiderWipeoutSub_UpdatePhase4', 'RiderBumpSub_HandleInput'] ...

0x001162c8 Rider_TryEnterPrewind shard=s00 size=176 race=c prof=0 callers=3
  kind=C4/F5 blocks=[9, 4, 5, 2, 2, 5, 4, 4, 3, 6] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=5 fcmp=1 mmi=0 sp_stores=4 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Rider_GetSubState*', 'FUN_00131348', 'Rider_SetSubState*', 'RiderBase_RecordCheatCodeInput']
  direct callers: ['RiderBoardPressSub_HandleInput', 'RiderCruiseSub_HandleInput', 'RiderRailslideSub_HandleInput']

0x00120ed8 Rider_RefreshPose shard=s00 size=72 race=c prof=0 callers=1
  kind=C3/F1 blocks=[8, 7, 3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=1 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Rider_BuildWorldMatrix*', 'Rider_TickAnimation', 'Rider_UpdateSkeleton']
  direct callers: ['AI_Update']

0x00122088 Rider_FindNearbyLights shard=s00 size=76 race=P prof=1 callers=4
  kind=C2 blocks=[19] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Rider_GetHipsPosition*', 'WorldLightMan_FindStrongestLights']
  direct callers: ['Rider_LoadStep', 'Rider_Place', 'Rider_RefreshOverlaps', 'Rider_BeginFrame']

0x00122658 Rider_ClearNearbyLights shard=s00 size=36 race=c2 prof=0 callers=1
  kind=C1 blocks=[9] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['EA_MemSet*']
  direct callers: ['World_ResetLights']

0x00120090 Rider_SetAnimTimeScale shard=s00 size=60 race=c prof=0 callers=1
  kind=leaf/F9 blocks=[7, 2, 4, 2] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=9 fcmp=2 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['ComputerActorRider_GetControlInput']

0x001360c8 RiderSpinSub_GetHUDAngle shard=s01 size=52 race=c prof=0 callers=1
  kind=leaf/F5 blocks=[6, 1, 3, 1, 2] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=5 fcmp=1 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['RiderMetrix_UpdateTrickHUD']
```

### D1: research notes

# D1 Rider frame doors & state handlers — research notes

Conventions: `Rider*` = the gameplay rider (vptr at +0x6c0); `RiderBase*` = owner (`Rider+0x77c`), main state +0xde0, sub state +0xde4. `g_App = *(gp-0x848)` (0x4a28a8, `SSXApp*`), `App+0x84` = `GameModule*` (+0x28 Replay*, +0x78 SectionMan*, +0x90 WorldLightMan*). "Table" = the generated code uses `call_indirect(c, addr)` (callee in another shard); "Direct" = a generated fn in the same shard. All frames: `sd ra` at the frame base unless stated otherwise.

#### `Rider_TickAnimation` — 0x0011eb60
*52 B · kind: calls · cycles: 13 · race P · callers 5*

`void Rider_TickAnimation(Rider *r /*a0*/, float dt /*f12, passed through*/)`: `AnimController_Update(r->anim (+0x784), *(r+0xb1c) (unmapped flag/int, "wall-hit flag" in rider.md))` with f12 untouched, then `AnimController_DispatchEndEvents(r->anim)`. No return value. Callees: AnimController_Update (0x3123c0, Table), AnimController_DispatchEndEvents (0x312490, Table).

**Gotchas:**
- Single block `+13`, both calls charged in it; no cycles after the calls.
- **f12 is an implicit argument**: callers load it (UpdateAnimation: `Rider+0x300 timeScale`; RefreshPose: 1.0) and it must arrive unchanged at AnimController_Update. Do not touch FPU regs.
- Frame 0x20: `sq s0` @+0x10, `sd ra` @+0; s0 = r. Unwind checks: ret 0x11eb7c, 0x11eb84. a0 reloaded from `+0x784` for the second call (re-read memory, the first call may change nothing but keep the `lw`).

#### `Rider_UpdateAnimation` — 0x00121700
*40 B · kind: calls/FPU · cycles: 7 (puppet) or 10 · race P · callers 1*

`void Rider_UpdateAnimation(Rider *r)`: if `r->isPuppet (+0xac4) == 0`, `f12 = r->timeScale (+0x300)` (lwc1) and `Rider_TickAnimation(r)` (Direct 0x11eb60). Callees: Rider_TickAnimation.

**Gotchas:**
- Blocks: `+5` entry, `+3` call arm, `+2` epilogue → 7 skip / 10 call.
- Frame 0x10 (only ra). The compiled code reloads `ra` from the stack in the branch delay slot before the call path sets it again — harmless.
- f12 is set *only* on the call path (lwc1, raw bits; no arithmetic). Return ret 0x12171c. If TickAnimation gets replaced, call it through the Callee anyway (unwind).

#### `Rider_UpdatePose` — 0x00121728
*40 B · kind: calls · cycles: 7 or 10 · race P · callers 1*

`void Rider_UpdatePose(Rider *r)`: if `isPuppet (+0xac4) == 0` → `Rider_UpdateSkeleton(r)` (Direct 0x11eb98). Same shape as UpdateAnimation without the f12 load.

**Gotchas:**
- `+5`, `+3` (call), `+2`; ret 0x121744; frame 0x10. Identical template to UpdateAnimation/RenderShadow — write one helper `guarded_wrapper(field, cond, callee, ret, tail_cost)`.

#### `Rider_UpdateModel` — 0x001217f8
*28 B · kind: calls · cycles: 7 · race P · callers 1*

`void Rider_UpdateModel(Rider *r)`: `RiderModel_BuildBoneMatrices(r->model (+0x780))` (0x3103f0, Table).

**Gotchas:**
- Exactly `call::tail_wrapper(c, Some(0x780), Target::Table(0x3103f0), 0x121808)`.

#### `Rider_RenderShadow` — 0x001225f0
*40 B · kind: calls · cycles: 7 (skip) or 10 · race P · callers 1*

`void Rider_RenderShadow(Rider *r)`: if `*(r+0xb18) != 0` (unmapped; rider.md calls 0xb18 "wall-hit result" with low confidence — here it gates shadow/lighting, likely "visible/loaded") → `Rider_DrawShadow(r)` (Direct 0x122898).

**Gotchas:**
- `+5`, `+3` call arm, `+2`. Condition is the opposite polarity of UpdatePose (calls when non-zero). ret 0x12260c, frame 0x10.

#### `Rider_PrepareLighting` — 0x001225c0
*44 B · kind: calls · cycles: 8 (skip) or 11 · race P · callers 1*

`int Rider_PrepareLighting(Rider *r)`: if `*(r+0xb18) != 0` → `Rider_BuildLightSet(r)` (Direct 0x1220d8). Always returns v0 = 1.

**Gotchas:**
- `+5`, `+3` call arm, epilogue `+3` (one more than the others because of `li v0,1`). ret 0x1225dc, frame 0x10.
- v0 = 1 on both paths (overwrites the callee's v0).

#### `Rider_RenderAttachedFX` — 0x00122638
*28 B · kind: calls · cycles: 7 · race c · callers 1*

`void Rider_RenderAttachedFX(Rider *r)`: `AttachedEmitterFX_Render(*(r+0x88c) AttachedEmitterFX*)` (0x2f2088, Table).

**Gotchas:**
- `tail_wrapper(c, Some(0x88c), Target::Table(0x2f2088), 0x122648)`. Not in the race profile directly (c), so the A/B replay exercises it only via AI_Render.

#### `Rider_UpdateAttachedFX` — 0x00120e88
*48 B · kind: calls · cycles: 12 · race P · callers 1*

`void Rider_UpdateAttachedFX(Rider *r)`: `AttachedEmitterFX_Update(*(r+0x88c))` (0x2f1a08, Table), then `cBoltOnFXInterface_Update(*(r+0x890))` (0x2ecf78, Table).

**Gotchas:**
- One block `+12` (both calls). Frame 0x20: `sq s0` @+0x10, `sd ra` @+0. Rets 0x120ea0, 0x120ea8. Second `lw +0x890` after the first call (read after, not before).

#### `Rider_UpdateSurfaceFX` — 0x001218d0
*128 B · kind: calls/virtual/FPU · cycles: 27 (no ground poly) or 32 · race P · callers 1*

`void Rider_UpdateSurfaceFX(Rider *r)`: (1) if `r->groundPolyId (+0x430) != -1`: `r->groundMaterial (+0x434) = SectionMan_FindLogicalSection(GameModule->sectionMan (+0x78), lbu(r+0x430))` (0x22e0e0, Table; second arg is the **low byte** of the id, `lbu`). (2) `idx = r->vt[7].GetIndex()` through the Rider vtable at +0x6c0 (entry at vt+0x38: `lh` this-delta = -0x6c0, fn at vt+0x3c = 0x140b80; this = r+0x6c0+delta). (3) `WorldPainter_UpdateView(a0 = idx, f12 = groundContactPoint.x (+0x460), f13 = .y (+0x464), f14 = -99999.0 (0xc7c34f80 at gp-0x7910))` (0x2ed490, Table). (4) `FUN_00392d18(owner ? owner+0xd30 : 0)` (0x392d18, Table) — clears an emitter state (cCheatCodeDetector+0x10 emitter): words +0, +4, +8, +0xc, +0xa0, +0xa4 = 0.

**Gotchas:**
- Blocks: `+8` entry, `+5` lookup arm, `+19` tail (all three later calls in it).
- Test is `nor` on the 64-bit sign-extended word: equal to -1 ⇔ word == 0xffffffff.
- `v1 = g_App` is loaded in the entry block (before the branch) — scratch.
- Virtual call: keep `lh` (sign-extended 16-bit delta) + `lw` fn ptr; use `call_ptr(c, fn, 0x12191c)`. Other rets: 0x121900, 0x121930, 0x121940.
- f12/f13/f14 are raw `lwc1` bits (no arithmetic). The null-check on owner is the C++ base-cast idiom: a0 = 0 when owner == 0, else owner+0xd30.
- Frame 0x20: `sq s0` @+0x10, `sd ra` @+0.

#### `Rider_UpdateProgress` — 0x00121818
*180 B · kind: calls/virtual · cycles: ≤45 over 9 blocks (longest path 41; typical non-puppet, nothing attached: 29) · race P · callers 1*

`void Rider_UpdateProgress(Rider *r)`: (1) if `obj = *(r+0xa30)` (attached/grind object) != 0: `t = *(obj+0xc)`; if t != 0 → virtual `t->vt(+0xc)` entry at +0x140/+0x144 (`lh` delta, `lw` fn) called with `(t+delta, r+0xa60, r+0x9e0, r+0x6c0)`; else `WScriptMan_OnObjectTrigger(*(gp+0xce8) (0x4a3dd8 WScriptMan instance), r+0xa60, r+0x9e0, r+0x6c0)` (0x30a060, Table). (2) `FUN_00125ad0(r)` (Direct; a `jr ra` stub, 2 cycles). (3) if !isPuppet: `Rider_UpdateRaceProgress(r)` (0x112338). (4) `Rider_UpdateAIPathProgress(r)` (0x1125c0), `Rider_CheckRaceFinish(r)` (0x125228). (5) if !isPuppet (re-read +0xac4): `RiderMetrix_Update(r->metrix (+0x790))` (0x117c28). (6) `Rider_UpdateFocusBox(r)` (0x120e30, *already replaced*).

**Gotchas:**
- Blocks: `+7` entry; attached: `+3`, then `+9` (virtual) **or** `+4` (WScript); `+5` (stub call + puppet test), `+2` RaceProgress arm; `+7` (AIPath + CheckRaceFinish + test), `+2` Metrix arm; `+6` FocusBox + epilogue.
- Totals: nothing attached + puppet 25, non-puppet 29; +12 (virtual) or +7 (WScript) when attached.
- `a1 = r+0xa60` is set in the delay slot before the `t==0` test (both arms use it). +0xac4 is read twice (after CheckRaceFinish), not cached.
- Rets: 0x12185c (virtual, `call_ptr`), 0x121874, 0x12187c, 0x121890, 0x121898, 0x1218a0, 0x1218b4, 0x1218bc. FocusBox: call through its Callee (address 0x120e30) so the unwind check stays.
- Frame 0x20: `sq s0` @+0x10, `sd ra` @+0.

#### `RiderBase_SetMainState` — 0x001112b8
*64 B · kind: calls · cycles: 16 · race c2 · callers 1*

`void RiderBase_SetMainState(RiderBase *b /*a0*/, int newState /*a1*/)`: `RiderBase_ExitMainState(b, newState)` (Direct 0x1112f8; a0/a1 passed through unchanged), then `old = b->mainState (+0xde0)`; `b->mainState = newState`; `RiderBase_EnterMainState(b, old)` (Direct 0x111380).

**Gotchas:**
- Single block `+16`. Frame 0x30: `sq s0` @+0x20, `sq s1` @+0x10, `sd ra` @+0. s1 = newState (full 64-bit a1 saved, `sw` stores low 32).
- The `lw` of the old value precedes the `sw` (read old, then write). Rets 0x1112d4, 0x1112e4.
- Caller `Rider_SetMainState` is replaced; this is the next layer down — pair it with SetSubState.

#### `RiderBase_SetSubState` — 0x00111538
*64 B · kind: calls · cycles: 16 · race c2 · callers 1*

Same as SetMainState on `+0xde4 subState`: `RiderBase_ExitSubState(b, new)` (0x111578), swap, `RiderBase_EnterSubState(b, old)` (0x111630).

**Gotchas:**
- Identical frame/ordering; rets 0x111554, 0x111564. Write one generic `set_state(c, off, exit, enter, rets)`.

#### `RiderWipeoutState_Update` — 0x00136e98
*72 B · kind: calls · cycles: 15 (ground) or 14 (air) · race c · callers 1*

`void RiderWipeoutState_Update(WipeoutState *s /*a0 = RiderBase+0x30*/)`: `RiderWipeoutState_UpdateRootPose(s)` (0x136f30); then if `*(s+0) == 0` (first word of the Wipeout state: 0 = on ground, else airborne; RiderBase+0x30 — `Rider_IsOnGround` uses the same flag) → `RiderWipeoutState_UpdateGround(s)` (0x137d18) else `RiderWipeoutState_UpdateAir(s)` (0x137750). All Direct (s01).

**Gotchas:**
- Blocks `+8` entry, `+4` ground arm **or** `+3` air arm, `+3` epilogue.
- The flag is read *after* UpdateRootPose returns. Rets 0x136eac, 0x136ec0 (ground), 0x136ed0 (air).
- Frame 0x20: `sq s0` @+0x10, `sd ra` @+0; `lq s0` happens inside each arm (before the shared epilogue) — order irrelevant for state.

#### `RiderWipeoutState_PostUpdate` — 0x00136ee0
*72 B · kind: calls · cycles: 15 (ground) or 14 (air) · race c · callers 1*

Same shape: `PostUpdateRootPose(s)` (0x137138), then `*(s) == 0` → `PostUpdateGround(s)` (0x138640) else `PostUpdateAir(s)` (0x137860).

**Gotchas:**
- Same costs `8 / +4|+3 / +3`. Rets 0x136ef4, 0x136f08, 0x136f18. Share one helper with Update (three Callees + three rets).

#### `RiderHandplantState_PostUpdate` — 0x00139528
*32 B · kind: calls · cycles: 8 · race c · callers 1*

`void RiderHandplantState_PostUpdate(HandplantState *s /*RiderBase+0x110*/)`: `Rider_UpdateBounds(*(s+0xa0) Rider* (vbase ptr), 0)` (0x11e150, Table; 2572 B body).

**Gotchas:**
- Not a tail_wrapper (8 cycles, sets `a1 = 0`). Single block `+8`, frame 0x10, ret 0x13953c.

#### `RiderCruiseSub_CheckAirborne` — 0x00131cc0
*68 B · kind: calls · cycles: 13 (not air) or 17 · race c · callers 1*

`bool RiderCruiseSub_CheckAirborne(CruiseSub *s /*RiderBase+0x1c0*/)`: `r = *(s+0)` (Rider*); if `Rider_GetMainState(r) == 1 (Air)` → `Rider_SetSubState(r, 4 NaturalAir)`, return 1; else return 0. Both callees already replaced.

**Gotchas:**
- Blocks `+9`, `+4` (SetSubState arm), `+4` epilogue. Rets 0x131cd8, 0x131cf0.
- `r` is re-read from `*(s0)` for the second call (memory re-read). v1 = 1 left as scratch.
- Frame 0x20: `sq s0` @+0x10, `sd ra` @+0. v0 = 0/1 written after the compare.

#### `Rider_IsOnGround` — 0x001231a8
*100 B · kind: calls · cycles: 15 / 19 / 23 / 25 · race c · callers 7*

`bool Rider_IsOnGround(Rider *r)`: returns 1 if `mainState == 0 (Ground)`, or `mainState == 2 (Wipeout)` and the Wipeout state flag `*(owner+0x30) == 0` (on ground); else 0. Calls `Rider_GetMainState` (replaced) **twice** (no CSE).

**Gotchas:**
- Paths: Ground: 9 + 1 + 5 = 15 (ret 1). Not ground, not wipeout: 9 + 5 + 5 = 19 (ret 0). Wipeout airborne: 9 + 5 + 4 + 5 = 23 (ret 0). Wipeout grounded: 9 + 5 + 4 + 1 + 1 + 5 = 25 (ret 1).
- The first call uses the incoming a0 (no copy); the second copies a0 = s0. Rets 0x1231c4, 0x1231d4.
- Frame 0x30: `sq s0` @+0x20, `sq s1` @+0x10, `sd ra` @+0. s1 holds the result. v1 left = 2 or the flag word (scratch).

#### `Rider_TryEnterFinishLine` — 0x00116378
*56 B · kind: calls/FPU · cycles: 11 (no) or 14 (yes) · race c · callers 4*

`bool Rider_TryEnterFinishLine(Rider *r)` (rider.md: `Rider_CheckFinished`): `f1 = 0.0`, `f0 = r->finishTimer (+0x470)`; `c.le.s f1, f0` (0 ≤ finishTimer); if true → `Rider_SetSubState(r, 0xa FinishLine)` (replaced), return 1; else return 0.

**Gotchas:**
- Blocks `+8`, `+3` (call arm), `+3` epilogue. Ret 0x1163a0. a0 is passed through unchanged.
- Compare must use the FPU sign-magnitude key (`c_le_s`), e.g. finishTimer = -0.0 → ≤ test vs +0.0 key; use the shared helper, not host `<=`.
- Leftovers: f0 = finishTimer bits, f1 = 0, FCC = compare result (scratch; reproduce cheaply if the helper sets them). Frame 0x10.

#### `Rider_CheckResetPath` — 0x00116120
*172 B · kind: calls · cycles: 14 (no-op) / 38–43 · race c · callers 20*

`bool Rider_CheckResetPath(Rider *r, int force /*a1*/, int reason /*a2*/)`: if `force == 0 && reason == 0` return 0. Else: `r->powerUpBoostTimer (+0x2e8) = 0`, `r->powerUpTrickTimer (+0x2ec) = 0`; if reason == 1 or 4 → `RiderMetrix_ShowWrongWay(r->metrix (+0x790))` (0x11a088); `*(owner+0x350)` (Reset sub-state object word 0) `= (reason != 0)` (`sltu 0 < reason`, unsigned 64-bit); `Rider_SetSubState(r, 9 Reset)` (replaced); `Rider_SetMainState(r, 3 Static)` (replaced); `SSXAudioSystem_OnRiderReset(SSXAudioSystem_GetInstance(), r)` (0x28b180, 0x29a220, Table); `Replay_OnRiderReset(GameModule->replay (+0x28), r->index (+0x86c))` (0x270970, Table); return 1.

**Gotchas:**
- Blocks: `+7` entry; `+2` only when force == 0 (reason test); `+4` (stores + reason==1 test); reason ≠ 1: `+3` (reason==4 test); ShowWrongWay arm `+3`; `+19` tail (all 4 later calls); `+5` epilogue. Totals (force≠0): reason 1 → 38, 4 → 41, other → 38; add 2 when force == 0.
- Order: the two `sw 0` stores happen before ShowWrongWay. `owner` (`lw +0x77c`) is re-read after ShowWrongWay (or in the delay slot otherwise).
- The `sltu` is on the full 64-bit a2 (s1) — `reason as u64 != 0`.
- Rets: 0x116168, 0x116180, 0x11618c, 0x116194, 0x1161a0, 0x1161b4. Frame 0x30: `sq s0` @+0x20, `sq s1` @+0x10, `sd ra` @+0.

#### `Rider_TryEnterPrewind` — 0x001162c8
*176 B · kind: calls/FPU · cycles: ≤44 over 10 blocks (longest path 41) · race c · callers 3*

`bool Rider_TryEnterPrewind(Rider *r, int jumpPressed /*a1*/, int cheatBit /*a2*/)` (rider.md: `Rider_TryPrewind`): if a2 != 0 → `RiderBase_RecordCheatCodeInput(owner+0xd20 cCheatCodeDetector, 1)` (0x2f6ac8, Table). Then if `(r->jumpRearm (+0x360) == 0.0 && a1 != 0) || a2 != 0`: if `Rider_GetSubState(r) == 1 (BoardPress)` → `FUN_00131348(owner+0x1d0)` (BoardPress sub-state release: adds boost via Rider_AddBoost when `+0x330` ∈ {1,2}; Direct); `Rider_SetSubState(r, 2 Prewind)`; return 1. Else `r->jumpRearm = 1.0 (0x3f800000)`, return 0.

**Gotchas:**
- Blocks: `+9` entry, `+4` cheat arm, `+5` (load/compare +0x360), `+2` (eq → test a1), `+2` (test a2), `+5` (GetSubState), `+4` (FUN_00131348 arm), `+4` (SetSubState), `+3` (store 1.0), `+6` epilogue.
- Paths: fail (no cheat): 9+5+2+3+6 = 25 if jumpRearm ≠ 0, 27 if jumpRearm == 0 with a1 == 0. Success via a1: 9+5+2+5+4+6 = 31 (+4 if from BoardPress). Success via a2: 9+4+5+[2 if ==0.0]+2+5+[4]+4+6.
- Wording in the generated code: `c_eq_s(f1, f0)` with f0 = 0.0 via `mtc1 zero` (key compare: -0.0 vs +0.0 — use the helper). The fail-path store goes through `at = 0x3f800000; mtc1 f0; swc1` → f0 and AT leftovers.
- Rets: 0x1162fc, 0x116328, 0x116340, 0x11634c. Frame 0x40: `sq s0` @+0x30, `sq s1` @+0x20, `sq s2` @+0x10, `sd ra` @+0 (s1 = a2, s2 = a1).

#### `Rider_RefreshPose` — 0x00120ed8
*72 B · kind: calls/FPU · cycles: 11 (puppet) or 18 · race c · callers 1*

`void Rider_RefreshPose(Rider *r)`: `Rider_BuildWorldMatrix(r)` (replaced, 0x11e098); if `!isPuppet (+0xac4)`: `f12 = 1.0`; `Rider_TickAnimation(r)`; `Rider_UpdateSkeleton(r)` (0x11eb98).

**Gotchas:**
- Blocks `+8`, `+7` (both calls), `+3` epilogue. Rets 0x120eec, 0x120f08, 0x120f10.
- f12 = 1.0 is the implicit dt to TickAnimation (see there); AT = 0x3f800000 leftover. f12 set only on the call path.
- Frame 0x20: `sq s0` @+0x10, `sd ra` @+0.

#### `Rider_FindNearbyLights` — 0x00122088
*76 B · kind: calls · cycles: 19 · race P · callers 4*

`void Rider_FindNearbyLights(Rider *r)`: `Vec4 hips; Rider_GetHipsPosition(&hips /*sp+0*/, r)` (replaced, 0x11ff48); `WorldLightMan_FindStrongestLights(GameModule->lights (+0x90), &hips, r->overlapList (+0x860), r+0x794 (ptr[8] light handles), 8 /*t0*/)` (0x2f5b68, Table).

**Gotchas:**
- Single block `+19`. Frame 0x30: **`sd ra` @+0x10** (not +0), `sq s0` @+0x20, the 16-byte hips vec at sp+0..0x10 is a real guest stack buffer (the callee writes it, FindStrongestLights reads it through a1) — must be the guest `sp`, and it stays in RAM (rule 9 — naturally reproduced because the callee writes it).
- 5th arg in **t0** (EABI: a0–a3, t0–t3). Rets 0x1220a4, 0x1220c4.

#### `Rider_ClearNearbyLights` — 0x00122658
*36 B · kind: calls · cycles: 9 · race c2 · callers 1*

`void Rider_ClearNearbyLights(Rider *r)`: `EA_MemSet(r+0x794, 0, 0x20)` (0x3e6448, replaced, Table call in generated code).

**Gotchas:**
- Single block `+9`, frame 0x10, ret 0x122670. Not a tail_wrapper (`addiu` not `lw`, plus a1/a2 set). Call EA_MemSet through its address (it is replaced, but keep the unwind check/`call_ptr`).

#### `Rider_SetAnimTimeScale` — 0x00120090
*60 B · kind: leaf/FPU · cycles: 11 or 13 · race c · callers 1*

`void Rider_SetAnimTimeScale(Rider *r /*a0*/, float target /*f12*/)` (rider.md: `Rider_SlewTimeScale`): slews `r->timeScale (+0x300)` toward `target` by at most `K = 0x3a5a740f` (≈8.333e-4 = 0.05/60, at gp-0x795c = 0x49b794): `cur = +0x300; if (target + K < cur) cur' = cur - K; else if (cur < target - K) cur' = cur + K; else cur' = target;` store `cur'` to +0x300.

**Gotchas:**
- Blocks: `+7` entry, `+2` (decrease arm) **or** `+4` (second compare), `+2` store/return → 11 (decrease) / 13 (others).
- FPU semantics: `add.s`/`sub.s` with clamp/flush (FpuF32), and `c.lt.s` with the sign-magnitude key (two compares: `c.lt.s f0(target+K), f1(cur)` then `c.lt.s f1(cur), f0(target-K)`). Keep exact operand order: `target+K`, `target-K`, `cur-K`, `cur+K`.
- f0 differs per path (target+K on decrease path, target-K otherwise); f12 = result, f1 = cur, f2 = K; FCC left set. All scratch.
- No frame, no stack writes.

#### `RiderSpinSub_GetHUDAngle` — 0x001360c8
*52 B · kind: leaf/FPU · cycles: 11 (centred) or 13 · race c · callers 1*

`float RiderSpinSub_GetHUDAngle(SpinSub *s /*a0, RiderBase+0x230*/, int *outCentred /*a1*/)`: if `s->stickAngle (+0x50) == 1e30 (0x7149f2ca at gp-0x7308 = 0x49bde8, "stick centred")` → `*a1 = 1`, return 0.0; else `*a1 = 0`, return stickAngle. Result in f0.

**Gotchas:**
- Blocks: `+6`, `+1` (not-equal → v0 = 0), `+3` (store), `+1` (reload angle), `+2` → 11 / 13.
- `c.eq.s` through the key helper. v0 left = flag (1/0) — set it too (cheap). f1 = 1e30 bits, FCC leftover.
- On the not-centred path f0 is loaded **twice** from +0x50 (the second `lwc1` after `mtc1 f0, zero`); result is the raw bits (no arithmetic). `sw` of the flag is a 32-bit store of v0.

### Package notes

**Typed views/helpers first:**
- `Rider` view: `owner()` +0x77c, `model()` +0x780, `anim()` +0x784, `metrix()` +0x790, `lights()` +0x794 (ptr[8]), `overlap_list()` +0x860, `index()` +0x86c, `attached_fx()` +0x88c, `bolt_on_fx()` +0x890, `attached_obj()` +0xa30, `is_puppet()` +0xac4, `shadow_flag()` +0xb18 (unmapped), `anim_flag()` +0xb1c (unmapped), `ground_poly_id()` +0x430, `ground_material()` +0x434, `ground_contact()` +0x460, `finish_timer()` +0x470, `time_scale()` +0x300, `jump_rearm()` +0x360, `power_up_boost/trick_timer()` +0x2e8/+0x2ec, vptr +0x6c0. `RiderBase`: `main_state` +0xde0, `sub_state` +0xde4, `wipeout_in_air` +0x30, `reset_sub_flag` +0x350, `cheat_detector` +0xd20, `board_press_sub` +0x1d0. `GameModule` (via `g_App`+0x84): +0x28 replay, +0x78 sectionMan, +0x90 lights; `WScriptMan` at `*(0x4a3dd8)`.
- One GCC-2 vtable-call helper: `vcall(c, obj_with_vptr_at, vptr_off, entry_off, ret)` doing `lh delta @entry`, `lw fn @entry+4`, `a0 = obj+vptr_off+delta`, `call_ptr` (used by UpdateSurfaceFX slot 7 and UpdateProgress +0x140).
- Templates: `guarded_wrapper` (UpdateAnimation/UpdatePose/RenderShadow/PrepareLighting: 5 + 3 + 2/3), `set_state` (both RiderBase setters), `wipeout_dispatch` (Update/PostUpdate). UpdateModel and RenderAttachedFX are `tail_wrapper` one-liners.
- Constants: K_SLEW 0x3a5a740f, NO_HEIGHT -99999.0 0xc7c34f80, STICK_CENTRED 1e30 0x7149f2ca, ONE 0x3f800000.

**Suggested order:** UpdateModel, RenderAttachedFX (tail_wrapper) → UpdateAnimation, UpdatePose, RenderShadow, PrepareLighting → TickAnimation (f12 passthrough), RefreshPose → UpdateAttachedFX, ClearNearbyLights, HandplantState_PostUpdate, FindNearbyLights → RiderBase_Set{Main,Sub}State → Wipeout Update/PostUpdate → CheckAirborne, IsOnGround, TryEnterFinishLine → leaves SetAnimTimeScale, GetHUDAngle (need `FpuF32` + key compares from A1) → UpdateSurfaceFX, UpdateProgress (vcall helper) → CheckResetPath, TryEnterPrewind.

**Cross-package deps:** A1 `FpuF32`/`c_lt_s`/`c_le_s`/`c_eq_s` helpers (SetAnimTimeScale, GetHUDAngle, TryEnterFinishLine, TryEnterPrewind); already-replaced Callees from A2/B (Rider_Get{Main,Sub}State, Rider_Set{Main,Sub}State, Rider_GetHipsPosition, Rider_UpdateFocusBox, Rider_BuildWorldMatrix, EA_MemSet) — call them through their guest addresses to keep unwind checks.

**Drops:** none required; all are small and overridable. Lowest value: `Rider_ClearNearbyLights` (c2, one EA_MemSet call) and `RiderHandplantState_PostUpdate` (rarely executed in the race) — keep only if the package needs bulk. Mild caveat: `Rider_TryEnterPrewind`/`Rider_CheckResetPath` have many paths (10/7 blocks) and are race "c" only, so the A/B replay may exercise few paths.

**Suggested additions (same subsystem):**
- 0x0011fea8 `Rider_GetMainStateName` — 7-cycle wrapper `owner → RiderBase_GetMainStateName` (0x111830, name table 0x459d70); likely a `tail_wrapper` with `+0x77c`.
- 0x0011fef8 `Rider_GetSubStateName` — same for sub state (table 0x459dc8).
- 0x00125ad0 (stub called by UpdateProgress, `jr ra`, 2 cycles) — trivial; or 0x00111830 `RiderBase_GetMainStateName` leaf (table lookup by +0xde0).

## D2: function list (classifier)

```text
# D2 Skeleton, animation & VU0 math

0x00310120 RiderModel_BuildBoneMatrix shard=s10 size=220 race=P prof=37 callers=1
  kind=leaf/VU31acc blocks=[55] loops=0 matchpc=False event_checks=0 vu0=31 vu0_acc=10 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['RiderModel_BuildBoneMatrices']

0x003103f0 RiderModel_BuildBoneMatrices shard=s10 size=100 race=P prof=41 callers=6
  kind=L/C1 blocks=[8, 2, 10, 5] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['RiderModel_BuildBoneMatrix']
  direct callers: ['RiderBase_LoadState', 'Rider_Place', 'Rider_UpdateModel', 'AI_UpdateRiderPoses', 'FEModule_RiderPreviewApplyTransform']

0x00310200 RiderModel_ApplyPose shard=s10 size=492 race=P prof=62 callers=6
  kind=leaf/L/VU21acc/F16/sp18 blocks=[6, 4, 7, 5, 3, 3, 3, 85, 5, 2] loops=2 matchpc=False event_checks=2 vu0=21 vu0_acc=11 cfc2_vi22=0 fpu=16 fcmp=0 mmi=0 sp_stores=18 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['Rider_LoadStep', 'Rider_UpdateSkeleton', 'FEModule_RiderPreviewSetupModel', 'FEModule_RiderPreviewApplyTransform']

0x00310530 RiderModel_Translate shard=s10 size=272 race=c prof=0 callers=1
  kind=leaf/L/VU3/sp6 blocks=[6, 4, 6, 2, 43, 5, 2] loops=2 matchpc=False event_checks=2 vu0=3 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=6 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['Rider_PostUpdateMainState*']

0x003123c0 AnimController_Update shard=s10 size=204 race=P prof=134 callers=2
  kind=L/C2/I1/F4 blocks=[14, 9, 1, 11, 1, 3, 12] loops=2 matchpc=False event_checks=2 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=4 fcmp=0 mmi=0 sp_stores=7 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['AnimSeqList_GetAt', 'AnimController_EvaluatePose']
  direct callers: ['Rider_TickAnimation', 'FEModule_RiderPreviewUpdate']

0x00312598 AnimController_EvaluatePose shard=s10 size=196 race=P prof=132 callers=7
  kind=L/C2 blocks=[14, 7, 3, 11, 14] loops=2 matchpc=False event_checks=2 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=7 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['AnimSeqList_CollectTracks', 'RiderModel_BlendAnimPose']
  direct callers: ['RiderBase_LoadState', 'Rider_LoadStep', 'Rider_Place', 'FUN_00124788', 'FEModule_RiderPreviewSetupModel', 'AnimController_Update']

0x0037de88 FUN_0037de88 shard=s12 size=88 race=P prof=52 callers=0
  kind=leaf/VU2 blocks=[18, 2, 2] loops=0 matchpc=False event_checks=0 vu0=2 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: []

0x0037dee0 PS2GraphicsMan_ProjectPoint shard=s12 size=96 race=P prof=1 callers=1
  kind=leaf/VU8acc/sp1 blocks=[24] loops=0 matchpc=False event_checks=0 vu0=8 vu0_acc=4 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['PS2GraphicsMan_v92']

0x0031b6c8 Mat4_Inverse shard=s10 size=124 race=P prof=2 callers=1
  kind=C2/VU4/F6 blocks=[31] loops=0 matchpc=False event_checks=0 vu0=4 vu0_acc=0 cfc2_vi22=0 fpu=6 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Mat4_Determinant', 'Mat4_Adjoint']
  direct callers: ['FUN_0022b008']

0x0031b748 Mat4_ToPosQuat shard=s10 size=96 race=c prof=0 callers=9
  kind=C1/F8 blocks=[24] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=8 fcmp=0 mmi=0 sp_stores=6 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Quat_FromMatrix']
  direct callers: ['ManualCamAlgoBase_GetView', 'ChaseCameraAlgorithm_BuildOutputView', 'ManualScriptCameraAlgo_GetView', 'TargetScriptCameraAlgo_GetView', 'SubjectScriptCameraAlgo_GetView', 'AlgorithmicScriptCameraAlgo_GetView', 'BoundedCameraAlgorithmBase_GetView', 'MeshAnim_ctor_2'] ...

0x0031b088 Math_Det2x2 shard=s10 size=16 race=c2 prof=0 callers=3
  kind=leaf/F3 blocks=[4] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=3 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['Math_Det3x3']

0x0031b098 Math_Det3x3 shard=s10 size=224 race=c prof=0 callers=20
  kind=C3/F48 blocks=[56] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=48 fcmp=0 mmi=0 sp_stores=11 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Math_Det2x2']
  direct callers: ['Mat4_Determinant', 'Mat4_Adjoint']

0x0031bb30 Quat_FromTwoVectors shard=s10 size=380 race=P prof=2 callers=1
  kind=C3/VU25accq22/F25 blocks=[27, 12, 12, 5, 2, 34, 3] loops=0 matchpc=False event_checks=0 vu0=25 vu0_acc=10 cfc2_vi22=1 fpu=25 fcmp=3 mmi=0 sp_stores=11 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Math_Asin*', 'Math_SinCos*']
  direct callers: ['FUN_00121aa0']

0x0031b7a8 Quat_FromMatrix shard=s10 size=520 race=c2 prof=0 callers=1
  kind=leaf/F68/sp1 blocks=[12, 25, 4, 1, 8, 5, 29, 1, 42, 3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=68 fcmp=4 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['Mat4_ToPosQuat']

0x0011dfe0 Rider_RotateAroundAxis shard=s00 size=184 race=P prof=1 callers=10
  kind=C2/VU9acc/F14 blocks=[46] loops=0 matchpc=False event_checks=0 vu0=9 vu0_acc=5 cfc2_vi22=0 fpu=14 fcmp=0 mmi=0 sp_stores=9 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Rider_BuildWorldMatrix*', 'Math_SinCos*']
  direct callers: ['Rider_TryStartRailslide', 'Rider_StartWipeout', 'FUN_00115358', 'FUN_00115640', 'RiderGroundState_Update']

0x0036d400 Vu0_RandNext shard=s12 size=36 race=c2 prof=0 callers=9
  kind=leaf/VU2/F4 blocks=[9] loops=0 matchpc=False event_checks=0 vu0=2 vu0_acc=0 cfc2_vi22=0 fpu=4 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['Emitter_UpdateDescriptor']

0x0032c590 SphereTree_GetRadius shard=s10 size=24 race= prof=0 callers=6
  kind=leaf/F3 blocks=[6] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=3 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['FUN_00328030', 'FUN_003303f0', 'FUN_00330540', 'FUN_003306d8']

0x0032c630 SphereTree_Translate shard=s10 size=24 race= prof=0 callers=1
  kind=leaf/VU1 blocks=[6] loops=0 matchpc=False event_checks=0 vu0=1 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['RollerModifier_Translate']

0x0032c5e0 SphereTree_SetPosition shard=s10 size=80 race= prof=0 callers=0
  kind=leaf/VU1/F8/sp6 blocks=[20] loops=0 matchpc=False event_checks=0 vu0=1 vu0_acc=0 cfc2_vi22=0 fpu=8 fcmp=0 mmi=0 sp_stores=6 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: []

0x00329910 RiderSphereTree_ctor shard=s10 size=36 race=P prof=1 callers=3
  kind=leaf blocks=[9] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['Rider_LoadStep', 'RiderWipeoutState_Enter', 'FUN_0032f650']

0x00329ae0 RiderSphereTree_UpdateFromBones shard=s10 size=96 race=P prof=1 callers=1
  kind=leaf/L blocks=[4, 2, 11, 7] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['FUN_00106828']

0x00314760 AnimSeqList_GetAt shard=s10 size=36 race=c prof=0 callers=11
  kind=leaf/L blocks=[2, 1, 2, 2, 2] loops=1 matchpc=True event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['AnimController_GetLayerSequence', 'AnimController_TransformRoots', 'FUN_00311d88', 'AnimController_PlayAnimImpl', 'AnimController_Update', 'AnimController_DispatchEndEvents', 'AnimController_SampleAnimPose', 'AnimController_GetLayerProgress'] ...

0x003145f8 AnimSeqList_Remove shard=s10 size=112 race=P prof=1 callers=15
  kind=L/C2 blocks=[8, 3, 2, 4, 4, 2, 5] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['AnimSequence_FreeTriggers', '__builtin_delete*']
  direct callers: ['RiderAnim_ScrubSeq', 'RiderAnim_BlendSeq2', 'RiderAnim_BlendSeq4', 'RiderAnim_PlaySeqThenLoop', 'RiderAnim_UpdateSeqType07', 'RiderAnim_UpdateSeqType0B', 'RiderAnim_UpdateSeqType08', 'RiderAnim_UpdateSeqType0E'] ...

0x0026ac48 Path_GetLength shard=s07 size=60 race=P prof=3 callers=5
  kind=leaf/L/F4 blocks=[4, 2, 7, 2] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=4 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['Rider_UpdateRaceProgress', 'Rider_UpdateAIPathProgress', 'Rider_SelectAIPath', 'Rider_SnapToNearestAIPath', 'FUN_0026a0b8']
```

## D3: function list (classifier)

```text
# D3 Chase camera (Tier B leftovers)

0x00168150 ChaseCameraAlgorithm_CheckDistanceReset shard=s02 size=328 race=P prof=3 callers=14
  kind=I1/VU11accq22/F9 blocks=[26, 5, 1, 47, 3] loops=0 matchpc=False event_checks=0 vu0=11 vu0_acc=2 cfc2_vi22=1 fpu=9 fcmp=2 mmi=0 sp_stores=13 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): []
  direct callers: ['ChaseCameraAlgorithm_ApplyTargetHeight', 'ChaseCameraAlgorithm_ApplyFollowDistance', 'ChaseCameraAlgorithm_ApplySideOffset', 'ChaseCameraAlgorithm_ApplyCameraHeight', 'ChaseCameraAlgorithm_ApplyFov', 'ChaseCameraAlgorithm_ApplyHandplantHold', 'ChaseCameraAlgorithm_ApplyJumpCurves', 'ChaseCameraAlgorithm_ApplyRailPullback'] ...

0x00166550 ChaseCameraAlgorithm_InitFromRider shard=s02 size=236 race= prof=0 callers=15
  kind=I1/VU11accq22/F10 blocks=[59] loops=0 matchpc=False event_checks=0 vu0=11 vu0_acc=2 cfc2_vi22=1 fpu=10 fcmp=0 mmi=0 sp_stores=8 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): []
  direct callers: ['ManualChaseCameraAlgorithm_ResetForRider', 'MatrixChaseCameraAlgorithm_ResetForRider', 'RelativeChaseCameraAlgorithm_ResetForRider', 'IdleChaseCameraAlgorithm_ResetForRider', 'LazyChaseCameraAlgorithm_ResetForRider', 'LookBackChaseCameraAlgorithm_ResetForRider', 'MidRightChaseCameraAlgorithm_ResetForRider', 'MidLeftChaseCameraAlgorithm_ResetForRider'] ...

0x0015d880 CameraBase_GetPitch shard=s01 size=164 race= prof=0 callers=0
  kind=C1/VU6accq22/F23 blocks=[41] loops=0 matchpc=False event_checks=0 vu0=6 vu0_acc=2 cfc2_vi22=1 fpu=23 fcmp=0 mmi=0 sp_stores=5 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Math_Asin*']
  direct callers: []

0x00163010 ChaseCameraAlgorithm_ApplyFollowDistance shard=s01 size=328 race=P prof=2 callers=7
  kind=C1/VU8accq22/F43 blocks=[14, 1, 15, 3, 12, 1, 36] loops=0 matchpc=False event_checks=0 vu0=8 vu0_acc=2 cfc2_vi22=1 fpu=43 fcmp=3 mmi=0 sp_stores=7 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['ChaseCameraAlgorithm_CheckDistanceReset']
  direct callers: ['LookBackChaseCameraAlgorithm_Update', 'MidRightChaseCameraAlgorithm_Update', 'MidLeftChaseCameraAlgorithm_Update', 'Default_2ChaseCameraAlgorithm_Update', 'Default_3ChaseCameraAlgorithm_Update', 'Default_4ChaseCameraAlgorithm_Update', 'DirectionNeChaseCameraAlgorithm_Update']

0x00162b90 ChaseCameraAlgorithm_ApplyTargetHeight shard=s01 size=228 race=P prof=1 callers=7
  kind=C1/VU2/F29 blocks=[11, 1, 7, 2, 1, 35] loops=0 matchpc=False event_checks=0 vu0=2 vu0_acc=0 cfc2_vi22=0 fpu=29 fcmp=1 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['ChaseCameraAlgorithm_CheckDistanceReset']
  direct callers: ['LookBackChaseCameraAlgorithm_Update', 'MidRightChaseCameraAlgorithm_Update', 'MidLeftChaseCameraAlgorithm_Update', 'Default_2ChaseCameraAlgorithm_Update', 'Default_3ChaseCameraAlgorithm_Update', 'Default_4ChaseCameraAlgorithm_Update', 'DirectionNeChaseCameraAlgorithm_Update']

0x00163158 ChaseCameraAlgorithm_ApplySideOffset shard=s01 size=280 race= prof=0 callers=3
  kind=C1/VU20accq22/F9 blocks=[33, 32, 5] loops=0 matchpc=False event_checks=0 vu0=20 vu0_acc=5 cfc2_vi22=2 fpu=9 fcmp=1 mmi=0 sp_stores=8 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['ChaseCameraAlgorithm_CheckDistanceReset']
  direct callers: ['LookBackChaseCameraAlgorithm_Update', 'MidRightChaseCameraAlgorithm_Update', 'MidLeftChaseCameraAlgorithm_Update']

0x00163270 ChaseCameraAlgorithm_ApplyCameraHeight shard=s01 size=316 race=c prof=0 callers=7
  kind=C1/VU2/F42 blocks=[11, 1, 5, 4, 4, 4, 1, 2, 2, 6, 39] loops=0 matchpc=False event_checks=0 vu0=2 vu0_acc=0 cfc2_vi22=0 fpu=42 fcmp=3 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['ChaseCameraAlgorithm_CheckDistanceReset']
  direct callers: ['LookBackChaseCameraAlgorithm_Update', 'MidRightChaseCameraAlgorithm_Update', 'MidLeftChaseCameraAlgorithm_Update', 'Default_2ChaseCameraAlgorithm_Update', 'Default_3ChaseCameraAlgorithm_Update', 'Default_4ChaseCameraAlgorithm_Update', 'DirectionNeChaseCameraAlgorithm_Update']

0x001633b0 ChaseCameraAlgorithm_ApplyFov shard=s01 size=160 race=c prof=0 callers=7
  kind=C1/F24 blocks=[18, 2, 2, 1, 17] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=24 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['ChaseCameraAlgorithm_CheckDistanceReset']
  direct callers: ['LookBackChaseCameraAlgorithm_Update', 'MidRightChaseCameraAlgorithm_Update', 'MidLeftChaseCameraAlgorithm_Update', 'Default_2ChaseCameraAlgorithm_Update', 'Default_3ChaseCameraAlgorithm_Update', 'Default_4ChaseCameraAlgorithm_Update', 'DirectionNeChaseCameraAlgorithm_Update']

0x00163450 ChaseCameraAlgorithm_ApplyHandplantHold shard=s01 size=364 race=c prof=0 callers=5
  kind=C1/VU5/F32 blocks=[5, 8, 7, 6, 14, 27, 1, 1, 4, 8, 1, 9] loops=0 matchpc=False event_checks=0 vu0=5 vu0_acc=0 cfc2_vi22=0 fpu=32 fcmp=3 mmi=0 sp_stores=10 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['ChaseCameraAlgorithm_CheckDistanceReset']
  direct callers: ['MidRightChaseCameraAlgorithm_Update', 'MidLeftChaseCameraAlgorithm_Update', 'Default_2ChaseCameraAlgorithm_Update', 'Default_3ChaseCameraAlgorithm_Update', 'Default_4ChaseCameraAlgorithm_Update']

0x001641c0 ChaseCameraAlgorithm_ApplyRailPullback shard=s01 size=484 race=c prof=0 callers=5
  kind=C1/I1/VU26accq22/F34 blocks=[26, 2, 55, 17, 21] loops=0 matchpc=False event_checks=0 vu0=26 vu0_acc=7 cfc2_vi22=1 fpu=34 fcmp=1 mmi=0 sp_stores=18 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['ChaseCameraAlgorithm_CheckDistanceReset']
  direct callers: ['MidRightChaseCameraAlgorithm_Update', 'MidLeftChaseCameraAlgorithm_Update', 'Default_2ChaseCameraAlgorithm_Update', 'Default_3ChaseCameraAlgorithm_Update', 'Default_4ChaseCameraAlgorithm_Update']

0x001643a8 ChaseCameraAlgorithm_ApplyLowSpeedHold shard=s01 size=760 race=P prof=1 callers=6
  kind=C1/VU22accq22/F45 blocks=[9, 3, 2, 1, 3, 2, 10, 2, 2, 5, 2, 2, 3, 3, 2, 1, 2, 11, 2, 2, 10, 1, 1, 1, 3, 14, 86, 5] loops=0 matchpc=False event_checks=0 vu0=22 vu0_acc=4 cfc2_vi22=2 fpu=45 fcmp=4 mmi=0 sp_stores=20 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['ChaseCameraAlgorithm_CheckDistanceReset']
  direct callers: ['LookBackChaseCameraAlgorithm_Update', 'MidRightChaseCameraAlgorithm_Update', 'MidLeftChaseCameraAlgorithm_Update', 'Default_2ChaseCameraAlgorithm_Update', 'Default_3ChaseCameraAlgorithm_Update', 'Default_4ChaseCameraAlgorithm_Update']

0x001646a0 ChaseCameraAlgorithm_ApplyRailAlign shard=s01 size=452 race=P prof=2 callers=6
  kind=C1/VU22accq22/F12 blocks=[8, 55, 2, 9, 2, 37] loops=0 matchpc=False event_checks=0 vu0=22 vu0_acc=4 cfc2_vi22=2 fpu=12 fcmp=1 mmi=0 sp_stores=22 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['ChaseCameraAlgorithm_CheckDistanceReset']
  direct callers: ['MidRightChaseCameraAlgorithm_Update', 'MidLeftChaseCameraAlgorithm_Update', 'Default_2ChaseCameraAlgorithm_Update', 'Default_3ChaseCameraAlgorithm_Update', 'Default_4ChaseCameraAlgorithm_Update', 'DirectionNeChaseCameraAlgorithm_Update']

0x00162998 ChaseCameraAlgorithm_UpdateHandplantFade shard=s01 size=136 race=c prof=0 callers=6
  kind=leaf/F14 blocks=[3, 2, 4, 1, 6, 8, 5, 2, 3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=14 fcmp=1 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['MidRightChaseCameraAlgorithm_Update', 'MidLeftChaseCameraAlgorithm_Update', 'Default_2ChaseCameraAlgorithm_Update', 'Default_3ChaseCameraAlgorithm_Update', 'Default_4ChaseCameraAlgorithm_Update', 'DirectionNeChaseCameraAlgorithm_Update']

0x00162a20 ChaseCameraAlgorithm_UpdateAirTransitions shard=s01 size=348 race=c prof=0 callers=7
  kind=C2/I1/VU8accq22/F14 blocks=[9, 3, 7, 23, 12, 2, 1, 8, 2, 2, 3, 3, 1, 2, 4, 5] loops=0 matchpc=False event_checks=0 vu0=8 vu0_acc=2 cfc2_vi22=1 fpu=14 fcmp=3 mmi=0 sp_stores=6 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['AI_GetFrameCounter']
  direct callers: ['LookBackChaseCameraAlgorithm_Update', 'MidRightChaseCameraAlgorithm_Update', 'MidLeftChaseCameraAlgorithm_Update', 'Default_2ChaseCameraAlgorithm_Update', 'Default_3ChaseCameraAlgorithm_Update', 'Default_4ChaseCameraAlgorithm_Update', 'DirectionNeChaseCameraAlgorithm_Update']

0x00166228 ChaseCameraAlgorithm_PostUpdate shard=s02 size=116 race=P prof=2 callers=0
  kind=C4/I1/F4 blocks=[16, 5, 8] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=4 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['ChaseCameraAlgorithm_ComputeAngles', 'FUN_001662a0', 'FUN_00166530', 'ChaseCameraAlgorithm_BuildOutputView']
  direct callers: []

0x00166640 ChaseCameraAlgorithm_ComputeAngles shard=s02 size=632 race=c prof=0 callers=4
  kind=C3/VU42accq22/F47 blocks=[34, 15, 25, 16, 3, 5, 24, 2, 16, 3, 4, 2, 2, 1, 6] loops=0 matchpc=False event_checks=0 vu0=42 vu0_acc=16 cfc2_vi22=1 fpu=47 fcmp=6 mmi=0 sp_stores=7 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Math_Atan*', 'Math_Asin*']
  direct callers: ['GameCamera_SnapToDefaultChaseView', 'GameCamera_ComputeView', 'ChaseCameraAlgorithm_PostUpdate']

0x00161bb8 ChaseCamController_Update shard=s01 size=672 race=P prof=13 callers=0
  kind=L/C3/I2/F58 blocks=[12, 20, 2, 4, 1, 13, 1, 4, 5, 4, 2, 5, 9, 2, 5, 6, 5, 1, 4, 7, 2, 1, 6, 1, 2, 2, 2, 3, 1, 2, 3, 4, 9, 8, 1, 1, 8] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=58 fcmp=9 mmi=0 sp_stores=7 sp_adjust=True indirect_calls=2
  callees (in order of first call; * = already overridden): ['ChaseCamController_UpdateAutoMode', 'AlgorithmList_Remove', 'ChaseCamController_SetOverrideAlgorithm']
  direct callers: []

0x00161ab0 ChaseCamController_UpdateAutoMode shard=s01 size=260 race=c prof=0 callers=1
  kind=C4/I1/VU6accq22/F7 blocks=[10, 4, 3, 26, 2, 5, 6, 3, 1, 5] loops=0 matchpc=False event_checks=0 vu0=6 vu0_acc=2 cfc2_vi22=1 fpu=7 fcmp=1 mmi=0 sp_stores=4 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['ChaseCamController_SetDefaultAlgorithm', 'ChaseCamController_EnterSpokeMode', 'ChaseCamController_ExitSpokeMode', 'Rider_GetMainState*']
  direct callers: ['ChaseCamController_Update']

0x0015d020 ChaseCamController_IsScriptCameraActive shard=s01 size=44 race=c prof=0 callers=1
  kind=leaf blocks=[4, 4, 1, 2] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['GameCamera_AvoidCollision']

0x00162060 ChaseCamController_SetOverrideAlgorithm shard=s01 size=112 race=c prof=0 callers=5
  kind=C1 blocks=[4, 5, 4, 5, 3, 2, 3, 2] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['ChaseCamController_SetAlgorithm']
  direct callers: ['GameCamera_Init', 'ChaseCamController_Update', 'CameraTrigger_ApplyAction']

0x0022e840 GameViewMan_UpdateCameras shard=s05 size=116 race=P prof=33 callers=9
  kind=L/I1 blocks=[9, 3, 11, 6] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=4 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): []
  direct callers: ['GameModule_Update', 'FUN_00244a40', 'FUN_00244e60', 'WorldState_Read', 'Replay_UpdateCameraInput', 'Replay_ProcessCommand', 'Replay_StepFrame']

0x0022e920 GameViewMan_AllViewsReady shard=s05 size=68 race=c prof=0 callers=6
  kind=leaf/L blocks=[3, 3, 4, 3, 2, 2] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['FUN_0022b008', 'GameModule_Update', 'FUN_00244888', 'FUN_00244a40']

0x0015e460 GameCamera_UpdateShake shard=s01 size=520 race=c prof=0 callers=1
  kind=C5/I2/VU6accq22/F36 blocks=[21, 5, 22, 5, 4, 4, 1, 5, 5, 5, 5, 3, 1, 16, 3, 5, 1, 19] loops=0 matchpc=False event_checks=0 vu0=6 vu0_acc=2 cfc2_vi22=1 fpu=36 fcmp=5 mmi=0 sp_stores=7 sp_adjust=True indirect_calls=2
  callees (in order of first call; * = already overridden): ['GameCamera_AddShake*', 'ChaseCameraAlgorithm_StartShake', 'ChaseCameraAlgorithm_ApplyShake']
  direct callers: ['GameCamera_ComputeView']

0x0015e030 GameCamera_ResetDefaultChase shard=s01 size=28 race= prof=0 callers=3
  kind=C1 blocks=[7] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['ChaseCameraAlgorithm_Reset*']
  direct callers: ['CamController_ResetForRider', 'CamController_Reset', 'GameCamera_Init']
```

## D4: function list (classifier)

```text
# D4 AI, BE interfaces & race logic

0x0014dc80 BELibrary_GetInstance shard=s01 size=200 race=P prof=1 callers=722
  kind=C6/I1 blocks=[6, 38, 6] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['BELibrary_ctor', 'BEPreRace_GetInstance', 'BERace_GetInstance', 'BEPostRace_GetInstance', 'EA_MemAllocNamed*']
  direct callers: ['Rider_CheckRiderCollisions', 'ComputerActorRider_RollUberTrick', 'ComputerActorRider_InitDifficulty', 'FUN_0010c9a8', 'ComputerActorRider_PickRandomTrick', 'ComputerActorRider_PickFittedTrick', 'ComputerActorRider_PickUberTrick', 'ComputerActorRider_ComputeRubberBand'] ...

0x001448d8 BENewRaceInt_GetInstance shard=s01 size=76 race=c prof=0 callers=7
  kind=C1 blocks=[4, 11, 4] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['EA_MemAllocNamed*']
  direct callers: ['BELibrary_ctor', 'BELibrary_DeserializeSave', 'BERace_GetInterface', 'BEAggressionInt_GetRivalState', 'BEAggressionInt_GetRivalLevel', 'BEPostRace_GetInterface', 'BEPreRace_GetInterface']

0x00145970 BENewPlayerInt_GetInstance shard=s01 size=68 race=c prof=0 callers=13
  kind=C2 blocks=[4, 9, 4] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['BENewPlayerInt_ctor', 'EA_MemAllocNamed*']
  direct callers: ['BENewRaceInt_SetEventType', 'FUN_00145750', 'BELibrary_ctor', 'BELibrary_ResetPlayerProfile', 'BELibrary_DeserializeSave', 'BESaveInt_BuildProfileSave', 'BESaveInt_LoadSystemSave', 'BESaveInt_LoadProfileSave'] ...

0x00147f78 BECharInt_GetInstance shard=s01 size=92 race=P prof=1 callers=9
  kind=C1/I1 blocks=[4, 15, 4] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['EA_MemAllocNamed*']
  direct callers: ['BELibrary_ctor', 'BELibrary_ResetPlayerProfile', 'BELibrary_DeserializeSave', 'BEEconInt_BuyAttribute', 'BESaveInt_BuildProfileSave', 'BESaveInt_LoadProfileSave', 'BERace_GetInterface', 'BEPostRace_GetInterface'] ...

0x00155a50 BEAggressionInt_GetInstance shard=s01 size=76 race=c prof=0 callers=17
  kind=C1 blocks=[4, 11, 4] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['EA_MemAllocNamed*']
  direct callers: ['ComputerActorRider_FindAttackTarget', 'FUN_0010e228', 'FUN_0010e2e8', 'FUN_0010e3a8', 'FUN_0010e468', 'AI_UpdateRiderRelations', 'AI_UpdateRivals', 'BELibrary_ctor'] ...

0x00154948 BEScoreInt_GetInstance shard=s01 size=92 race=c prof=0 callers=7
  kind=C2 blocks=[4, 15, 4] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['BEScoreInt_ctor', 'EA_MemAllocNamed*']
  direct callers: ['BELibrary_ctor', 'BELibrary_DeserializeSave', 'BESaveInt_BuildProfileSave', 'BESaveInt_LoadSystemSave', 'BERace_GetInterface', 'BEPostRace_GetInterface', 'BEPreRace_GetInterface']

0x0014f250 BEOptionInt_GetInstance shard=s01 size=84 race=P prof=1 callers=5
  kind=C2 blocks=[4, 13, 4] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['BEOptionInt_ctor', 'EA_MemAllocNamed*']
  direct callers: ['BELibrary_ctor', 'BESaveInt_LoadSystemSave', 'BERace_GetInterface', 'BEPostRace_GetInterface', 'BEPreRace_GetInterface']

0x00146e98 BENewPlayerInt_GetHumanRider shard=s01 size=168 race=P prof=1 callers=87
  kind=L/C1 blocks=[16, 7, 2, 2, 2, 1, 3, 1, 8] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=7 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['BENewPlayerInt_GetSlot*']
  direct callers: ['FUN_00145750', 'cFEStateBraggingRights_OnWidgetCreated', 'FUN_001a34b8', 'FUN_001e1550', 'MessageCenter_PostForeshadow', 'FUN_001e2648', 'FUN_001e2828', 'FUN_001e2a08'] ...

0x00147398 BENewPlayerInt_GetCharacter shard=s01 size=52 race=c prof=0 callers=32
  kind=C1 blocks=[13] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['BENewPlayerInt_GetSlot*']
  direct callers: ['Rider_GetCharacter', 'AI_CreateComputerRiders', 'FUN_00145750', 'FUN_001e1550', 'MessageCenter_PostForeshadow', 'FUN_001e2648', 'FUN_001e2828', 'FUN_001e2a08'] ...

0x00147290 BENewPlayerInt_GetPadIndex shard=s01 size=52 race=c prof=0 callers=3
  kind=C1 blocks=[13] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['BENewPlayerInt_GetSlot*']
  direct callers: ['AI_CreatePlayers', 'GameModule_CheckPauseRequest']

0x00144c98 BENewRaceInt_GetTrackPeak shard=s01 size=36 race=c prof=0 callers=8
  kind=leaf blocks=[9] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['FUN_0010c9a8', 'cFEStateChallengePopup_FillChallengeText', 'cGameLoadState_SetupMPWidget', 'cGameLoadState_SetupQPWidget', 'World_GetPeakNumber', 'WScriptMission_v18', 'WScriptMan_CollectItem']

0x00148680 BECharInt_GetAttrib3 shard=s01 size=96 race=P prof=1 callers=2
  kind=C2 blocks=[24] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['BE_GetRiderProfile*', 'BE_GetRiderCharacter*']
  direct callers: ['BECharInt_GetOverallRating', 'cGameLoadState_UpdatePlayerRanking']

0x00148d80 BECharInt_GetAttrib3Factor shard=s01 size=228 race=P prof=1 callers=1
  kind=C3/F6 blocks=[18, 4, 2, 9, 6, 13, 5] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=6 fcmp=0 mmi=0 sp_stores=5 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['BE_GetRiderProfile*', 'BE_GetRiderCharacter*', 'FUN_001477e8']
  direct callers: ['FUN_0013ccf0']

0x00155b50 BEAggressionInt_GetRivalLevel shard=s01 size=160 race=P prof=2 callers=6
  kind=C5 blocks=[21, 12, 1, 6] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=4 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['BE_GetRiderProfile*', 'BE_GetRiderCharacter*', 'BENewRaceInt_GetInstance', 'FUN_00145750']
  direct callers: ['ComputerActorRider_FindAttackTarget', 'AI_UpdateRiderRelations', 'AI_UpdateRivals', 'SSXAudioSystem_GetRiderRivalry', 'cAngerFX_Update']

0x0012a250 AI_AllPlayersFinished shard=s00 size=92 race=c prof=0 callers=15
  kind=leaf/L/F3 blocks=[3, 3, 7, 1, 2, 3, 2, 2] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=3 fcmp=1 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['RaceStateRace_Update', 'RiderMetrix_FinishTrick', 'AI_UpdateRaceTimeLimit', 'cOVStateHUD1P_Update', 'GMHandler0_OnRiderFinished', 'GMHandler9_OnRiderFinished', 'GMHandler1_OnRiderFinished', 'GMHandler8_OnRiderFinished'] ...

0x0012a180 AI_AreAllRidersLoaded shard=s00 size=112 race=c prof=0 callers=5
  kind=L/C1 blocks=[11, 1, 8, 8] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=5 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Rider_IsLoaded*']
  direct callers: ['AI_UpdateRacePositions', 'GameModule_Load', 'GameStatePreRaceLoad_Update', 'GameStateReset_OverlayUpdate', 'GameStateRestartRace_Update']

0x001298c8 AI_GetFrameCounter shard=s00 size=20 race=c prof=0 callers=32
  kind=leaf blocks=[5] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['FocusMan_Update', 'Rider_CheckRiderCollisions', 'ComputerActorRider_ThinkGate', 'ComputerActorRider_SelectTargetRider', 'FUN_0010da10', 'ComputerActorRider_FindAttackTarget', 'FUN_0010e028', 'Rider_LaunchIntoAir'] ...

0x00113c20 AI_UpdateRaceState shard=s00 size=148 race=c prof=0 callers=1
  kind=I3 blocks=[8, 3, 5, 11, 10] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=3
  callees (in order of first call; * = already overridden): []
  direct callers: ['AI_Update']

0x0010f878 AI_UpdateRivals shard=s00 size=288 race=c prof=0 callers=1
  kind=L/C2 blocks=[15, 3, 4, 9, 3, 2, 9, 4, 2, 4, 1, 4, 1, 11] loops=2 matchpc=False event_checks=2 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=10 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['BEAggressionInt_GetInstance', 'BEAggressionInt_GetRivalLevel']
  direct callers: ['AI_UpdateRiderRelations']

0x0010a960 ComputerActorRider_ThinkCruise shard=s00 size=272 race=P prof=7 callers=1
  kind=C3/I1 blocks=[21, 2, 3, 4, 4, 4, 1, 3, 8, 2, 2, 4, 1, 9] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=4 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['ComputerActorRider_ShouldResetPath', 'ComputerActorRider_FindAttackTarget', 'FUN_0010da10']
  direct callers: ['ComputerActorRider_GetControlInput']

0x0010b750 ComputerActorRider_ThinkWipeout shard=s00 size=64 race=c prof=0 callers=1
  kind=C1 blocks=[16] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['ComputerActorRider_ShouldResetPath']
  direct callers: ['ComputerActorRider_GetControlInput']

0x00100610 ComputerActorRider_GetSteerTarget shard=s00 size=108 race=c prof=0 callers=8
  kind=leaf/VU3/F2/sp6 blocks=[27] loops=0 matchpc=False event_checks=0 vu0=3 vu0_acc=0 cfc2_vi22=0 fpu=2 fcmp=0 mmi=0 sp_stores=6 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['ComputerActorRider_CruiseFollowPath', 'ComputerActorRider_CruiseApproachJump', 'FUN_00100b90', 'FUN_00100f88', 'ComputerActorRider_ThinkBoardPress', 'ComputerActorRider_ThinkNaturalAir']

0x0010deb0 ComputerActorRider_GetHumanRider shard=s00 size=64 race=c2 prof=0 callers=3
  kind=C1 blocks=[6, 6, 1, 3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['ComputerActorRider_FindHumanRiderIndex']
  direct callers: ['ComputerActorRider_IsFarBehindHuman', 'ComputerActorRider_IsFarAheadOfHuman', 'ComputerActorRider_ComputeRubberBand']

0x00113618 AirPredictor_SetLaunchState shard=s00 size=48 race=c prof=0 callers=3
  kind=leaf/F2 blocks=[12] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=2 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['AirPredictor_Prime', 'AirPredictor_Update']
```

### D4: research notes

The BE ("back end") layer holds lazily created singletons with one global
pointer each, all in the gp area (gp = 0x4a30f0). The race code reaches them
through hundreds of call sites. The AI functions loop over the race's rider
array (`AI+0x28`, count at `AI+0x78`, world.md). Every function was read from
the generated Rust (block costs, order) and checked against Ghidra.

**The `*_GetInstance` shape (seven functions).** Each one has a 16-byte frame
that saves `ra` with `sd` at `sp+0`. It loads `inst = *(gp-K)`. If `inst != 0`,
it goes to the shared tail. Otherwise it calls
`EA_MemAllocNamed(size, tag, 0, 0)` (replaced; `a2`/`a3` are explicitly 0),
stores the result in `*(gp-K)` and initialises the object, in one of three
ways:
- it writes `obj+8 = 0` and `obj+0xc = vtable` itself;
- it calls `X_ctor(obj)`;
- it makes a virtual call through the new vtable, `vt+0x14` with `this + (short)vt[+0x10]`.

The shared tail then reloads `v0 = *(gp-K)` (it does not reuse the
register), restores `ra` and returns. Cycles: first block, then
either the init block or nothing, then the tail block. A generic
`lazy_singleton(c, slot, size, tag, init: Init)` helper covers all seven. The
store order differs per function (some store the global before the vtable
words, some after), and that order matters if the allocator's callee
observes memory, so keep it.

#### `BELibrary_GetInstance` — 0x0014dc80
*200 B · kind: calls 6 + virtual 1 · cycles: ≤50 over 3 blocks (6 / 38 / 6) · race P · callers 722*

`BELibrary *BELibrary_GetInstance(void)`. The global is at `gp-0x1f04` (0x4a11ec).
On first use it builds the whole BE library, in this order:
1. `EA_MemAllocNamed(4, "…" @0x4a1200, 0, 0)` (replaced), result stored to `gp-0x1f04`.
2. `EA_MemAllocNamed(1, @0x45a638, 0, 0)` (replaced), then `BELibrary_ctor(obj)`; the result is stored to `gp-0x1ef8`.
3. The three race-phase interfaces: `BEPreRace_GetInstance(0)` → `*(0x5308a8)`, `BERace_GetInstance(1)` → `0x5308ac`, `BEPostRace_GetInstance(2)` → `0x5308b0`. 0x5308a8 is the `g_beLibrary` table that the replaced `BELibrary_GetInterface` indexes.
4. `*(gp-0x1efc) = 0` (current phase index).
5. A virtual call on the phase-0 interface: `vt = *(iface+4)`, `fn = vt[+0x24]`, `this = iface + (short)vt[+0x20]`.
6. `*(gp-0x1f00) = 1`, `*(gp-0x1ea0) = *(gp-0x1f04)`.

It returns `*(gp-0x1f04)`, reloaded in the tail block.

**Gotchas:**
- Frame 0x30: `sq s0` at +0x20, `sq s1` at +0x10, `sd ra` at +0. The full 128-bit `s0`/`s1` are saved and restored (`lq`), so a replacement must use `Frame` with 128-bit saves.
- The hot path (already built) costs 6 + 6 = 12 cycles with no calls. That is the case worth optimising for the 722 callers; the slow path runs once per boot. It is still worth replacing in full, because a replay starts from boot.
- The call sites set `s0 = 0x530000` and `s1 = 0x5308a8` before the calls (callee-saved). The 128-bit restore makes the restored values identical anyway.
- Return addresses: 0x14dcb4, 0x14dcd4, 0x14dcdc, 0x14dce8, 0x14dcf4, 0x14dd00, 0x14dd20.

#### `BENewRaceInt_GetInstance` — 0x001448d8
*76 B · kind: calls 1 · cycles: ≤19 over 3 blocks (4 / 11 / 4) · race c · callers 7*

Lazy singleton at `gp-0x1f3c`. Init: `EA_MemAllocNamed(0x10, @0x45a0f0)`,
then `obj+8 = 0`, `obj+0xc = 0x45ae58` (vtable), then the global. Returns the
global.

**Gotchas:** the shape above. Hot path 8 cycles. Return address 0x144900.

#### `BENewPlayerInt_GetInstance` — 0x00145970
*68 B · kind: calls 2 · cycles: ≤17 (4 / 9 / 4) · race c · callers 13*

Lazy singleton at `gp-0x1f30`. Init: `EA_MemAllocNamed(0x10, @0x45a1d8)` →
`BENewPlayerInt_ctor(obj)` → the global is set from the ctor's `v0`, not from
the allocation.

**Gotchas:** the global holds the ctor's return value. Two return addresses.

#### `BECharInt_GetInstance` — 0x00147f78
*92 B · kind: calls 1 + virtual 1 · cycles: ≤23 (4 / 15 / 4) · race P · callers 9*

Lazy singleton at `gp-0x1f2c`. Init:
1. `EA_MemAllocNamed(0x10, @0x45a280)`; the global is stored immediately.
2. `obj+0xc = 0x45ad68` (vtable), `obj+8 = 0`.
3. A virtual call through the static vtable (not through the object):
   `fn = *(0x45ad68+0x14)`, `this = obj + (short)*(0x45ad68+0x10)`. It is a
   ctor-like init slot.

**Gotchas:** the indirect call needs `call_ptr` and its unwind check. The vtable
address is a constant, so `VSlot` can be resolved statically, but it should
still go through `call_indirect` (the function table) so that overrides apply.

#### `BEAggressionInt_GetInstance` — 0x00155a50
*76 B · kind: calls 1 · cycles: ≤19 (4 / 11 / 4) · race c · callers 17*

Lazy singleton at `gp-0x1ea4`, alloc `(0x10, @0x45a898)`, `obj+8 = 0`,
`obj+0xc = 0x45ac48`. Called once per rider pair from `AI_UpdateRivals` (this
package).

**Gotchas:** identical in shape to BENewRaceInt. Do both with the generic helper.

#### `BEScoreInt_GetInstance` — 0x00154948
*92 B · kind: calls 2 · cycles: ≤23 (4 / 15 / 4) · race c · callers 7*

Lazy singleton at `gp-0x1ea8`. Init: `EA_MemAllocNamed(0x230, @0x45a858)` →
`BEScoreInt_ctor(obj)` → the global is set from `v0`. Then it writes the
fields `+0x1c = -1`, `+0x8 = 0`, `+0x10 = 0`, `+0x14 = 0`, `+0x18 = -1`.

**Gotchas:** the field stores come after the ctor call and use the ctor's
return value as the base.

#### `BEOptionInt_GetInstance` — 0x0014f250
*84 B · kind: calls 2 · cycles: ≤21 (4 / 13 / 4) · race P · callers 5*

Lazy singleton at `gp-0x1ee4`. Init: alloc `(0x10, @0x45a6b0)`,
`obj+0xc = 0x45adc8`, global = obj, `obj+8 = 0`, then `BEOptionInt_ctor(obj)`.
Its return value is ignored, and the tail reloads the global.

**Gotchas:** the ctor runs after the global is published. A re-entrant
`GetInstance` from inside the ctor would see the object, so keep the order.

#### `BENewPlayerInt_GetHumanRider` — 0x00146e98
*168 B · kind: loop + calls 1 · cycles: ≤42 over 9 blocks · race P · callers 87*

`int GetHumanRider(void *this /*a0, unused*/, int second /*a1*/)`. Scans the
six player slots `i = 0..5`:
- `s = BENewPlayerInt_GetSlot(i)` (replaced);
- `entry = 0x534fec + s*0x1c` (`mult` with 0x1c);
- if `*(int*)entry < 0`, the slot is not human and the scan moves on;
- otherwise, if `second == 0`, it returns `i`; if not, it skips the first
  human (the counter `s1` is compared against the constant 1, `s3`) and
  returns the second human's index.

It returns -1 if none is found. The player table at 0x534fe0 has 0x1c-byte
records: `+0xc` is the pad index and `+0x11` the character (see
GetPadIndex/GetCharacter); `+0xc` doubles as the "human" test, < 0 = CPU.

**Gotchas:**
- Frame 0x70: `sq s0..s5` at +0x60..+0x10, `sd ra` at +0. A `Frame` with six 128-bit saves.
- The loop's back edge uses the **old shape**: `if c.cycles >= c.next_event { c.pc = 0x146ed8; c.event(); }` and ignores the result. Reproduce it as is: no return on unwind.
- `mult` writes LO/HI (`abi::mult`).
- The block path per slot: 7 (call + test), then 2 + 2 (human, first one skipped) or 1 (non-human), then 3 (loop test). Exit: 2 + 1 or 8 (epilogue). Note the `lq s0` placed early in the exit blocks.
- `v0 = -1` is set in the loop-test block before the branch, so the "not found" return relies on that leftover.

#### `BENewPlayerInt_GetCharacter` — 0x00147398
*52 B · kind: calls 1 · cycles: 13 (one block) · race c · callers 32*

`int GetCharacter(this, int player /*a1*/)`: `s = GetSlot(player)` (replaced),
then returns `(s8)*(0x534fe0 + s*0x1c + 0x11)`.

**Gotchas:** one block charged up front, before the call. A 16-byte frame with `ra`. `a0 = 0x1c` and LO/HI from the `mult` are leftovers.

#### `BENewPlayerInt_GetPadIndex` — 0x00147290
*52 B · kind: calls 1 · cycles: 13 · race c · callers 3*

The same as GetCharacter, but returns the word `*(0x534fe0 + s*0x1c + 0xc)`.
**Gotchas:** write both from one `PlayerRecord` view (0x1c bytes at 0x534fe0).

#### `BENewRaceInt_GetTrackPeak` — 0x00144c98
*36 B · kind: leaf · cycles: 9 · race c · callers 8*

`int GetTrackPeak(void)`: `idx = *(0x535c08)` (current track), returns
`*(0x43d950 + idx*0x64 + 0x54)`. 0x43d950 is the static track table, with
0x64-byte records.

**Gotchas:** `mult` (LO/HI) and the leftovers `a0 = 0x530000`, `a1 = 0x64`.
Pairs with the replaced `BENewRaceInt_GetTrackInfo` (same table).

#### `BECharInt_GetAttrib3` — 0x00148680
*96 B · kind: calls 2 · cycles: 24 · race P · callers 2*

`int GetAttrib3(this, int player /*a1*/)`:
`prof = BE_GetRiderProfile(player)`, `ch = BE_GetRiderCharacter(player)` (both
replaced), then returns `(s8)*(0x535538 + prof*0x46 + ch*7 + 3)`. It is the
attribute table: 0x46 bytes per profile, 7 bytes per character. Its siblings
`GetAttrib0..6` (0x148410…0x1488f0, 96 B each) differ only in the final byte
offset (+0…+6). **Write one function with a parameter and register all seven**:
seven more overrides for little extra work.

**Gotchas:** frame 0x30 (`sq s1` +0x10, `sq s0` +0x20, `sd ra`). One block
charged before both calls. `v1 = 0x46` and `mult` leftovers.

#### `BECharInt_GetAttrib3Factor` — 0x00148d80
*228 B · kind: calls 3, FPU · cycles: ≤57 over 7 blocks · race P · callers 1*

`float GetAttrib3Factor(this, int player /*a1*/, int override /*a2*/)`, result
in f0. Steps:
1. `prof = BE_GetRiderProfile(player)`, `ch = BE_GetRiderCharacter(player)`.
2. If `f_001477e8(player)` (unnamed, probably "is the attribute capped/maxed")
   is true, it returns 0.5 (`0x3f000000`).
3. Otherwise `v = override > 0 ? override : attrib3(prof, ch) / 5` (an integer
   `div` by the constant 5, with the generated divide-by-zero guard taken both
   ways).
4. It returns `float(v) / float((s8)*(0x5308d8 + ch*15 + 0xb))`, the
   character's maximum. The conversion is `cvt.s.w` and the division `div.s`
   (FPU).

Seven siblings (`GetAttrib{0,1,4,5,6}Factor`, the `_2` copies, 228 B each)
differ in the byte offset. `GetAttrib5Factor` has an `__entries` twin and is
excluded.

**Gotchas:**
- Block path: 18, then 4 (the 0.5 path) or 2 → 9 → 6 → 13 (the computed path), then 5.
- `div` writes LO/HI.
- `div.s` uses `FpuF32::div` (a zero divisor gives ±max).
- Frame 0x50 with four `sq` saves.
- The 0.5 path loads `at = 0x3f000000`, `mtc1` into f0, and leaves `at` behind.

#### `BEAggressionInt_GetRivalLevel` — 0x00155b50
*160 B · kind: calls 5 · cycles: ≤40 over 4 blocks (21 / 12 / 1 / 6) · race P · callers 6*

`int GetRivalLevel(this, int a /*a1*/, int b /*a2*/)`. Steps:
1. `profB = BE_GetRiderProfile(b)`, `chA = BE_GetRiderCharacter(a)`,
   `chB = BE_GetRiderCharacter(b)`.
2. `race = BENewRaceInt_GetInstance()`.
3. If `f_00145750(race) == chA` (unnamed; probably "the race's featured/boss
   character"), it returns 3.
4. Otherwise it returns
   `(s8)*(0x4a6ca8 + 0xbc2 + chA*3 + chB*0xf88 + profB*0x9b50)`, a 3-D
   aggression table.

**Gotchas:**
- Two `mult`s (the LO/HI end state is from `profB*0x9b50`).
- Frame 0x40, `sq s0..s2`.
- Five return addresses.
- Call order: profile(b) **first**, then character(a), then character(b).

#### `AI_AllPlayersFinished` — 0x0012a250
*92 B · kind: leaf + loop, FPU · cycles: ≤23 over 8 blocks · race c · callers 15*

`bool AI_AllPlayersFinished(AI *ai)`. For `i < ai+0x7c` (player count), it
takes `rider = *(ai+0x40+4i)`, `st = *(rider+0x18)` (Rider→state block) and
`t = *(float*)(st+0x470)` (finish time, < 0 = not finished). If `!(0.0 <= t)`,
it returns 0. It returns 1 after the loop, or at once if the count is ≤ 0.

**Gotchas:**
- `c.le.s f1(=0.0), f0` sets FCC; the last compare's FCC, `f0 = t`, `f1 = 0.0` (`mtc1 zero`) and `a1/a2/a3` are leftovers.
- The compare uses the FPU key (`FpuF32::le`).
- The back edge uses the old shape: `c.pc = 0x12a268; c.event();` with the result ignored.
- Paths: 3 → 2 (count ≤ 0); else 3, then per iteration 7 + (1 when not finished) + 2 + 3, and the exit adds 2 or 2.

#### `AI_AreAllRidersLoaded` — 0x0012a180
*112 B · kind: loop + calls 1 · cycles: ≤28 over 4 blocks · race c · callers 5*

`bool AreAllRidersLoaded(AI *ai)`: `ok = 1`; for `i < ai+0x78`, if
`!Rider_IsLoaded(ai[0x28+4i])` (replaced), then `ok = 0`. It returns `ok`. It
does **not** break early: every rider is queried.

**Gotchas:**
- The count `ai+0x78` is reloaded after every call.
- The loop's back edge is the old shape (`c.pc = 0x12a1b0; c.event();`).
- Frame 0x50 with `sq s0..s3`.
- `s1` is incremented and `s0` advanced *before* the call, so the call sees them already updated (callee-saved, invisible to the callee). Only the order of the cycle charge matters: 8 per iteration, charged before the call.

#### `AI_GetFrameCounter` — 0x001298c8
*20 B · kind: leaf · cycles: 5 · race c · callers 32*

Returns `*(*(*(*(gp-0x848)+0x84)+0xc)+0x8)`: the global `AI*` at 0x4a28a8 →
`+0x84` race → `+0xc` → `+8` frame counter.

**Gotchas:** `v1` and `a0` keep the intermediate pointers (leftovers).

#### `AI_UpdateRaceState` — 0x00113c20
*148 B · kind: virtual 3 · cycles: ≤37 over 5 blocks · race c · callers 1*

A small state-machine driver. `this+0` is the requested state id, `+4` the
current id, `+0x98` the new state object and `+0x9c` the old state object.
- If `+0 != +4`: if `old(+0x9c)` is set, it calls `old->Exit(this)` (vtable
  slot at `vt+0x10/+0x14`). Then it calls `new(+0x98)->Enter(this)`
  (`vt+0x8/+0xc`), clears `+0x9c` and sets `+4 = +0`.
- It always calls `(+0x98)->Update(this)` (`vt+0x18/+0x1c`).

Every call passes `a0 = obj + (short)vt[slot]` and `a1 = this`.

**Gotchas:**
- Three `VSlot` calls, each with its unwind check.
- The `if (+4 == +0)` test has `a2 = *(this+0x98)` in its delay slot (the load happens on both paths).
- Frame 0x20 (`sq s0` +0x10).

#### `AI_UpdateRivals` — 0x0010f878
*288 B · kind: 2 nested loops + calls 2 · cycles: ≤72 over 14 blocks · race c · callers 1*

`void AI_UpdateRivals(AI *ai)`. It returns at once if `ai+0x80` (rivals
enabled) ≤ 0 or `ai+0x78` (rider count) ≤ 0. For each rider `i`:
- it sets `r = ai[0x28+4i]`, `r+0xf0 = 0` (has rival) and `r+0xf8 = -1`
  (rival index);
- for each `j != i` it computes
  `lvl = BEAggressionInt_GetRivalLevel(BEAggressionInt_GetInstance(), i, j)`
  (both in this package; cross-shard `call_indirect`). If `lvl == 4`, it sets
  `r+0xf0 = 1` and `r+0xf8 = j`. It always sets `r+0xfc = lvl`, so the last
  `j` wins.

The fields `+0xf0..+0xfc` are unmapped in rider.md. Add them as
`rival_flag/rival_idx/rival_level`.

**Gotchas:**
- Two loops, both with the old-shape back edge (`c.event()` result ignored), at 0x10f900 (inner) and 0x10f8d0 (outer).
- The large frame saves `s0..s7` and `fp` (`fp = -1` is used as a constant).
- The rider count is reloaded after each inner iteration.

#### `ComputerActorRider_ThinkCruise` — 0x0010a960
*272 B · kind: calls 3 + virtual 1 (pointer to member) · cycles: ≤68 over 14 blocks · race P · callers 1*

`void ThinkCruise(CAR *this, Control *ctl /*a1*/)`. Steps:
1. `ctl->flags bit 12 = ComputerActorRider_ShouldResetPath(this) & 1`.
2. `this+0xe20 = 0`.
3. If `ComputerActorRider_FindAttackTarget(this, &tmp)` finds a target, it
   ORs `ctl->flags` with 0x40000 (when `tmp == 0`) or 0x80000. Otherwise, if
   `FUN_0010da10(this)` is true, it ORs 0xc0000.
4. It finishes with a **gcc 2.9x pointer-to-member call** stored at
   `this+0xf44`: `delta = (short)+0xf44`, `index = (short)+0xf46`, and at
   `+0xf48` either the function pointer (`index < 0`) or the vptr offset. The
   virtual case loads the 8-byte vtable entry `vt[index-1]`. The call is
   `fn(this + delta [+ entry delta], ctl)`.

**Gotchas:**
- The PMF dispatch is a new pattern. Add a `Pmf` helper next to `VSlot` (the 8-byte `ld` of the vtable entry: `delta` in the low half and `fn` in the high word).
- `tmp` is a 16-byte stack slot passed by address (the frame write is visible).
- Flags are read-modify-write words.

#### `ComputerActorRider_ThinkWipeout` — 0x0010b750
*64 B · kind: calls 1 · cycles: 16 · race c · callers 1*

`void ThinkWipeout(this, Control *ctl)`: it sets bit 12 of `ctl->flags` to
`ShouldResetPath(this) & 1`. This is the first statement of ThinkCruise; share
the helper.

**Gotchas:** one block charged before the call. The mask `0xffffffffffffefff` is
applied to the 64-bit register, but the result is stored with `sw`.

#### `ComputerActorRider_GetSteerTarget` — 0x00100610
*108 B · kind: leaf, VU0 · cycles: 27 · race c · callers 8*

`Vec4 *GetSteerTarget(Vec4 *out /*a0*/, CAR *this /*a1*/)`, with
`st = *(this+0x18)`:
`out = st[+0x4b0] + (st[+0x490] - st[+0x110]) * 0.1f`.
- `+0x110` is the rider position (rider.md); `+0x490`/`+0x4b0` are unmapped
  path points.
- 0.1 comes from `gp-0x7fd0` = 0x49b120 (0x3dcccccd).
- It is a struct return: `v0 = a0`.

**Gotchas:**
- A 0x30-byte frame in a leaf. The compiler bounces every VU0 value through the stack (`sqc2` → `lq` → `sq` → `lqc2`), so `sp+0`, `+0x10` and `+0x20` end up holding the intermediate vec4s. Reproduce those stores: they are below the caller's `sp` but stay in RAM.
- The VU0 scratch registers vf3/vf4/vf5 are left with their final values (`ps2f::vu0_scratch`).
- GPRs `v1`/`a2`/`a3` hold the 128-bit `lq` values (`set_gpr128`).
- `f0` = 0.1, and `a2`/`a3` are left as `mfc1` results.
- No ACC.

#### `ComputerActorRider_GetHumanRider` — 0x0010deb0
*64 B · kind: calls 1 · cycles: ≤16 over 4 blocks (6 / 6 or 1 / 3) · race c2 · callers 3*

`Rider *GetHumanRider(this)`: `i = ComputerActorRider_FindHumanRiderIndex(this)`.
If `i < 0`, it returns 0. Otherwise it returns
`*( *(*(*(gp-0x848)+0x84)+0xc) + i*4 + 0x28 )`: the AI global → race → rider
array.

**Gotchas:** the `a0` load of the AI global happens in the branch delay slot,
on both paths.

#### `AirPredictor_SetLaunchState` — 0x00113618
*48 B · kind: leaf, FPU · cycles: 12 · race c · callers 3*

It copies the predictor's state for a new jump: `+0xac = 0`, `+0x98 = +0xa0`
(float), and the remaining moves (`+0x9c`… from the vec at `+0x80`); see the
generated body (12 instructions, all moves).

**Gotchas:** `lwc1/swc1` moves leave `f0` holding the last moved float.
Bit-exact moves, no arithmetic.

### Package notes

- Views:
  - `PlayerRecord` (0x1c bytes at 0x534fe0: `+0xc` pad index / human flag, `+0x11` character);
  - `AttribTable` (0x535538, 0x46 per profile, 7 per character);
  - the character max table (0x5308d8, 15-byte records, `+0xb`);
  - `AI` (`+0x28` riders[], `+0x40` players[], `+0x78` rider count, `+0x7c` player count, `+0x80` rivals enabled, `+0x84` race);
  - the rider rival fields `+0xf0/+0xf8/+0xfc`.
- New helpers:
  - `lazy_singleton` for the `GetInstance` family;
  - `Pmf` (gcc 2.9x pointer-to-member call) for ThinkCruise;
  - the old-shape loop back edge (`event()` result ignored), shared with D6/D7 loops.
- Order:
  1. the GetInstance family and the table getters (GetCharacter, GetPadIndex, GetTrackPeak, GetAttrib3 plus the 6 siblings);
  2. GetHumanRider and the AI loops;
  3. GetRivalLevel, then AI_UpdateRivals;
  4. the ComputerActorRider functions.
- Cheap extras: the six `BECharInt_GetAttribN` siblings and six of the seven
  `GetAttribNFactor` siblings (`GetAttrib5Factor` has `__entries`), 0x148410…0x149208.
- No drops.

## D5: function list (classifier)

```text
# D5 World, sections & process lists

0x003550a0 ProcessList_RenderPriority shard=s11 size=116 race=P prof=90 callers=2
  kind=L/C2/I1 blocks=[11, 1, 11, 1, 5] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=4 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['ProcessList_First', 'ProcessList_Next']
  direct callers: ['GameModule_RenderUI', 'GameModule_RenderProcessLists']

0x00354f98 ProcessList_UpdatePriority shard=s11 size=144 race=P prof=50 callers=16
  kind=L/C3/I1 blocks=[12, 2, 12, 1, 9] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=5 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['ProcessList_First', 'ProcessList_Next']
  direct callers: ['GameModule_Update', 'FUN_00244e60']

0x00355028 ProcessList_PostUpdatePriority shard=s11 size=116 race=P prof=9 callers=2
  kind=L/C2/I1 blocks=[11, 1, 11, 1, 5] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=4 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['ProcessList_First', 'ProcessList_Next']
  direct callers: ['GameModule_Update', 'FUN_00244e60']

0x00354ef8 ProcessList_Next shard=s11 size=36 race=P prof=2 callers=10
  kind=leaf blocks=[9] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['WorldState_WriteObjects', 'WorldState_WriteDeadObjects', 'FUN_0026de58', 'ProcessList_UpdatePriority', 'ProcessList_PostUpdatePriority', 'ProcessList_RenderPriority', 'ProcessList_DeletePriority', 'ProcessList_DeleteSectionObjects']

0x00354f48 ProcessList_PendingFirst shard=s11 size=36 race=P prof=1 callers=1
  kind=leaf blocks=[9] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['ProcessList_MergePending']

0x00354c98 ProcessList_MergePending shard=s11 size=428 race=P prof=2 callers=10
  kind=L/C4/I3 blocks=[14, 2, 16, 2, 7, 7, 10, 8, 6, 6, 2, 1, 15, 2, 9] loops=2 matchpc=False event_checks=2 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=7 sp_adjust=True indirect_calls=3
  callees (in order of first call; * = already overridden): ['ProcessList_PendingFirst', 'ProcessList_PendingNext', 'ProcessList_Last', 'ProcessList_Prev']
  direct callers: ['GameModule_Unload', 'GameModule_ResetScriptsAndWorld', 'GameModule_ResetRace', 'GameModule_OnSectionUnload', 'GameModule_Update', 'FUN_00244e60', 'WorldState_Read', 'WorldState_ReadObjects']

0x00230640 GameModule_RenderProcessLists shard=s06 size=88 race=P prof=90 callers=1
  kind=L/C1 blocks=[10, 6, 6] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=4 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['ProcessList_RenderPriority']
  direct callers: ['FUN_0022b008']

0x0022c078 GameModule_RenderModels shard=s05 size=312 race=P prof=91 callers=1
  kind=L/I2 blocks=[34, 4, 13, 3, 4, 13, 7] loops=2 matchpc=False event_checks=2 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=5 sp_adjust=True indirect_calls=2
  callees (in order of first call; * = already overridden): []
  direct callers: ['FUN_0022b008']

0x0022c620 GameModule_RenderBezierPatches shard=s05 size=228 race=P prof=53 callers=1
  kind=L/C4/I1 blocks=[13, 3, 7, 3, 4, 7, 4, 7, 9] loops=2 matchpc=False event_checks=2 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=5 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['GameModule_BeginBezierPass', 'GameModule_DrawBezierPatch', 'GameModule_EndBezierPass']
  direct callers: ['FUN_0022b008']

0x0022c3b8 GameModule_EndBezierPass shard=s05 size=88 race=P prof=41 callers=1
  kind=C1/I1 blocks=[15, 3, 4] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['__builtin_delete*']
  direct callers: ['GameModule_RenderBezierPatches']

0x00352dd0 ModifierBlock_Render shard=s11 size=124 race=P prof=38 callers=1
  kind=L/I2 blocks=[8, 2, 8, 3, 6, 4] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=2
  callees (in order of first call; * = already overridden): []
  direct callers: ['ModObj_Render']

0x00352d20 ModifierBlock_PostUpdate shard=s11 size=172 race=P prof=17 callers=1
  kind=L/I3 blocks=[8, 2, 8, 3, 1, 8, 3, 6, 4] loops=2 matchpc=False event_checks=2 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=3
  callees (in order of first call; * = already overridden): []
  direct callers: ['ModObj_PostUpdateModifiers']

0x00352c70 ModifierBlock_Update shard=s11 size=176 race=P prof=2 callers=1
  kind=C3/I3 blocks=[7, 6, 2, 3, 2, 3, 2, 13, 3, 3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=3
  callees (in order of first call; * = already overridden): ['ModifierBlock_FreeBoost', 'UVScrollModifier_Update', 'TexFlipModifier_Update']
  direct callers: ['ModObj_UpdateModifiers']

0x00352b88 ModifierBlock_GetTransformBounds shard=s11 size=56 race=P prof=1 callers=5
  kind=I1 blocks=[5, 2, 4, 3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): []
  direct callers: ['ModObj_v49', 'ModObj_v50', 'ModObj_v44', 'ModObj_v45', 'Floating_v45']

0x00356198 ModObj_Update shard=s11 size=252 race=P prof=28 callers=1
  kind=C5/I5 blocks=[7, 1, 7, 7, 4, 7, 9, 5, 4, 9, 3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=5
  callees (in order of first call; * = already overridden): ['FUN_0034fce0', 'ModObj_UpdateModifiers', 'ModObj_PostUpdateModifiers', 'ModifierBlock_IsTransformFinished', 'ModifierBlock_ClearTransformFinished']
  direct callers: ['Particle_Update']

0x00355748 ModObj_PostUpdateModifiers shard=s11 size=40 race=P prof=17 callers=1
  kind=C1 blocks=[5, 3, 2] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['ModifierBlock_PostUpdate']
  direct callers: ['ModObj_Update']

0x003556f8 ModObj_UpdateModifiers shard=s11 size=80 race=P prof=2 callers=1
  kind=C2 blocks=[7, 6, 4, 3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['ModifierBlock_Update', 'ModifierBlock_HasTransformChanged']
  direct callers: ['ModObj_Update']

0x003a6928 World_Update shard=s13 size=28 race=P prof=3 callers=5
  kind=C1 blocks=[7] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['cWorldCache_Update']
  direct callers: ['GameModule_LoadWorld', 'GameModule_Update', 'FUN_00244a40']

0x003a8290 cWorldCache_Update shard=s13 size=56 race=P prof=3 callers=1
  kind=C3 blocks=[14] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['cWorldCache_StreamerUpdate', 'cWorldCache_UpdateHullLRU', 'cStreamMan_Update']
  direct callers: ['World_Update']

0x002d1be0 World_GetNash shard=s09 size=16 race=c prof=0 callers=23
  kind=leaf blocks=[4] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['FlexBridge_dtor', 'FlexBridge_BuildMesh', 'FlexBridge_UpdateVerts', 'FlexRail_UpdateSpaceNode', 'FlexRail_RegisterSpaceNode', 'FlexRail_UnregisterSpaceNode', 'Floating_v50', 'Floating_v49'] ...

0x002d1c70 World_GetSecondsPerTick shard=s09 size=12 race=c prof=0 callers=16
  kind=leaf/F1 blocks=[3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=1 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['Boost_v48', 'Boost_v47', 'AnimTeeter_v15', 'BoostModifier_v08', 'BoostModifier_v07', 'FloatRail_v47', 'SpringRail_v47', 'DeadFade_ctor_2'] ...

0x0022e0e0 SectionMan_FindLogicalSection shard=s05 size=56 race=c prof=0 callers=5
  kind=leaf/L blocks=[4, 3, 3, 2, 2] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['Rider_UpdateSurfaceFX', 'AI_SetTrackPathsBuffer', 'FUN_0012a490', 'FUN_00278a10', 'FUN_00278a58']

0x002c1cd8 QuadTree_FindLeaf shard=s08 size=208 race=P prof=1 callers=1
  kind=leaf/L/F11 blocks=[15, 2, 10, 1, 17, 5, 2] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=11 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['WPData_FindCellIndex']

0x002d1928 Noise_Fractal1D shard=s09 size=144 race=P prof=1 callers=2
  kind=L/C1/F22 blocks=[15, 3, 8, 10] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=22 fcmp=0 mmi=0 sp_stores=7 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Noise_Sample1D*']
  direct callers: ['cBoardWakeFX_BuildTrail', 'AttachedEmitterFX_Update']

0x0022a5a0 Nash_CollectInstances shard=s05 size=244 race=P prof=11 callers=1
  kind=leaf/L blocks=[2, 5, 7, 3, 5, 7, 7, 1, 2, 2, 7, 7, 2, 2, 2] loops=2 matchpc=False event_checks=2 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['Nash_GatherVisible']
```

## D6: function list (classifier)

```text
# D6 Frontend UI, fonts & HUD

0x0039f9d8 UIStateMgr_FindState shard=s13 size=172 race=P prof=1 callers=20
  kind=L/C4 blocks=[11, 2, 1, 2, 1, 4, 5, 2, 3, 2, 4, 6] loops=1 matchpc=True event_checks=2 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=4 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['UIList_First*', 'UIListNode_IsSentinel*']
  direct callers: ['cFEStateAudioOptions_Init', 'cFEStateAudioOptions_Shutdown', 'cFEStateRequestLine_Init', 'cFEStateRequestLine_Shutdown', 'MessageCenter_PostMessage', 'cOVStateOverlay_Init', 'cOVStateOverlay_Shutdown', 'GameUI_IsReplayMenuOpen'] ...

0x0039fa88 UIStateMgr_GetActiveState shard=s13 size=96 race=P prof=3 callers=11
  kind=L/C2 blocks=[8, 2, 5, 1, 4, 4] loops=1 matchpc=True event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['UIList_First*', 'UIListNode_IsSentinel*']
  direct callers: ['NetFEState_OnDisconnectedPopupEvent', 'NetFEState_OnNetInterruptedPopupEvent', 'cFEStateRelogin_v33', 'GameUI_PushPopupScreen', 'GameUI_CanOpenPauseMenu', 'GameUI_HandleMenuResult', 'GameUI_Update', 'FUN_002579c8'] ...

0x0039ebe0 UIStateMgr_Render shard=s13 size=196 race=P prof=57 callers=1
  kind=L/C4/I2 blocks=[9, 1, 4, 6, 4, 5, 1, 4, 6, 4, 1, 4] loops=2 matchpc=False event_checks=2 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=2
  callees (in order of first call; * = already overridden): ['UIList_First*', 'UIListNode_IsSentinel*']
  direct callers: ['UIEngine_Render']

0x0039f100 UIStateMgr_Update shard=s13 size=144 race=P prof=3 callers=1
  kind=L/C5/I1 blocks=[7, 2, 5, 2, 4, 5, 7, 1, 3] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['UIStateMgr_ProcessStates', 'UIList_First*', 'UIState_UpdateScreens', 'UIListNode_IsSentinel*']
  direct callers: ['UIEngine_Update']

0x0039e868 UIState_UpdateScreens shard=s13 size=76 race=c prof=0 callers=4
  kind=L/C3 blocks=[8, 7, 1, 3] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['UIList_First*', 'UIScreen_Update', 'UIListNode_IsSentinel*']
  direct callers: ['UIStateMgr_ProcessStates', 'UIStateMgr_Update']

0x0039eb60 UIStateMgr_HasStateInPhase shard=s13 size=128 race=c prof=0 callers=1
  kind=L/C2 blocks=[10, 2, 4, 4, 2, 4, 1, 5] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['UIList_First*', 'UIListNode_IsSentinel*']
  direct callers: ['UIStateMgr_ProcessStates']

0x0039ea90 UIStateMgr_RestartStateInPhase shard=s13 size=204 race=c prof=0 callers=2
  kind=L/C2 blocks=[14, 4, 4, 4, 11, 4, 1, 9] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=7 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['UIList_First*', 'UIListNode_IsSentinel*']
  direct callers: ['UIStateMgr_ProcessStates']

0x0039f840 UIStateMgr_CloseAll shard=s13 size=136 race=c prof=0 callers=10
  kind=L/C2 blocks=[11, 3, 9, 4, 7] loops=1 matchpc=True event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=5 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['UIList_First*', 'UIListNode_IsSentinel*']
  direct callers: ['OVScreenFinishOv_Update', 'OVScreenFinishOv_OnWidgetAction', 'OVScreenFinishOv2_Update', 'OVScreenFinishOv2_OnWidgetAction', 'GameUI_Update', 'GameStateFinish_Enter', 'GameStateCountdown_Enter', 'GameStateRace_Enter'] ...

0x0039f7b8 UIStateMgr_SetParentChainVisible shard=s13 size=136 race=c prof=0 callers=2
  kind=leaf/L blocks=[2, 3, 3, 5, 2, 4, 2, 6, 5, 2] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['UIStateMgr_ProcessStates']

0x00398038 UIEngine_Update shard=s13 size=64 race=P prof=4 callers=10
  kind=C1/I1 blocks=[16] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['UIStateMgr_Update']
  direct callers: ['ssx_BootMoviesUpdate', 'GameModule_Update', 'StartScreen_Update', 'cGameLoadScreen_Update', 'cFELoadScreen_Update', 'GameModule_UpdatePauseAndUI']

0x00397df8 UIEngine_Render shard=s13 size=540 race=P prof=57 callers=6
  kind=C1/I6/F6 blocks=[135] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=6 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=6
  callees (in order of first call; * = already overridden): ['UIStateMgr_Render']
  direct callers: ['FEModule_Render', 'FUN_0022b008', 'GameModule_RenderUI', 'StartScreen_Render', 'cGameLoadScreen_Render', 'cFELoadScreen_Render']

0x00391cb0 Font_DrawText shard=s12 size=384 race=P prof=10 callers=50
  kind=I2/F33 blocks=[29, 5, 40, 22] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=33 fcmp=2 mmi=0 sp_stores=10 sp_adjust=True indirect_calls=2
  callees (in order of first call; * = already overridden): []
  direct callers: ['DebugMenu_Render', 'LobbyMenu_Render', 'cOVStateHUD1P_Render', 'FUN_001f14b0', 'HUD_DrawTime', 'HUD_DrawTextElem', 'FUN_001f1e28', 'HUD_DrawWrappedText'] ...

0x00391fb0 Font_MeasureText shard=s12 size=572 race=P prof=1 callers=60
  kind=leaf/L/F69 blocks=[13, 3, 4, 2, 4, 7, 5, 6, 3, 1, 3, 1, 2, 31, 10, 3, 4, 3, 14, 4, 7, 8, 2, 3] loops=2 matchpc=False event_checks=2 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=69 fcmp=2 mmi=0 sp_stores=0 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['cFEStateNetConfig_LoadConfigList', 'cFEStateCreateStep1_SetTruncatedText', 'cFEStateNetQuickMessages_SetTextEllipsized', 'cFEPopupConfirm_MeasureDesc', 'cFEPopupConfirm_MeasureLine', 'cFEPopupConfirm_CheckAndFixTextEntry', 'cKeyboardPopup_UpdateScrollWindow', 'cKeyboardPopup_UpdateCursor'] ...

0x00392908 Font_DrawTextCentered shard=s12 size=120 race=P prof=2 callers=15
  kind=C2/F12 blocks=[30] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=12 fcmp=0 mmi=0 sp_stores=5 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Font_MeasureText', 'Font_DrawText']
  direct callers: ['DebugMenu_Render', 'LobbyMenu_Render', 'cOVStateHUD1P_Render', 'cOVStateHUD1P_DrawProgressMeter', 'cOVStateHUD1P_DrawRaceTime', 'FUN_002200c0', 'Font_DrawTextCenteredF']

0x001e9290 HUD_GetElemScale shard=s04 size=20 race=P prof=1 callers=38
  kind=leaf/F4 blocks=[5] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=4 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['cOVStateHUD1P_Init', 'cOVStateHUD1P_Update', 'cOVStateHUD1P_Render', 'HUD_DrawTimeElem', 'HUD_DrawTextElem', 'FUN_001f1e28', 'HUD_DrawTextElemW', 'FUN_001f25e0'] ...

0x001e91a8 HUD_GetElemColor shard=s04 size=36 race=c prof=0 callers=40
  kind=leaf/F8 blocks=[9] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=8 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['cOVStateHUD1P_Render', 'HUD_DrawTimeElem', 'HUD_DrawTextElem', 'HUD_DrawTextElemW', 'HUD_DrawWrappedText', 'HUD_DrawWrappedTextW', 'FUN_0021d9a0', 'FUN_0021e1b0'] ...

0x001e91f8 HUD_GetElemPos shard=s04 size=36 race=c prof=0 callers=42
  kind=leaf/F6 blocks=[9] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=6 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['cOVStateHUD1P_Init', 'cOVStateHUD1P_Render', 'HUD_DrawTimeElem', 'HUD_DrawTextElem', 'FUN_001f1e28', 'HUD_DrawTextElemW', 'FUN_001f25e0', 'HUD_DrawWrappedText'] ...

0x001e9220 HUD_GetElemSize shard=s04 size=108 race=c prof=0 callers=32
  kind=leaf/F16 blocks=[8, 2, 2, 7, 2, 2, 2, 2] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=16 fcmp=2 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['cOVStateHUD1P_Init', 'cOVStateHUD1P_Render', 'HUD_DrawWrappedText', 'HUD_DrawWrappedTextW', 'FUN_0021d1a0', 'FUN_0021d9a0', 'FUN_0021e1b0', 'FUN_0021ed48'] ...

0x001f10f8 HUD_AlignElemPos shard=s04 size=148 race=P prof=1 callers=37
  kind=C2/F8 blocks=[37] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=8 fcmp=0 mmi=0 sp_stores=7 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['FUN_0021e750', 'FUN_0021e7a8']
  direct callers: ['cOVStateHUD1P_Init', 'cOVStateHUD1P_Render', 'HUD_DrawTime', 'HUD_DrawTextElem', 'FUN_001f1e28', 'HUD_DrawTextElemW', 'FUN_001f25e0', 'FUN_0021d1a0'] ...

0x001f1190 HUD_DrawSprite shard=s04 size=420 race=P prof=1 callers=25
  kind=L/I1/F49 blocks=[6, 6, 7, 23, 8, 52, 3] loops=2 matchpc=False event_checks=2 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=49 fcmp=0 mmi=0 sp_stores=24 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): []
  direct callers: ['cOVStateHUD1P_Render', 'FUN_0021e1b0', 'FUN_0021ed48', 'FUN_0021f338', 'cOVStateHUD1P_DrawRaceTime', 'cOVStateHUD1P_DrawCollectedCount', 'FUN_0021fd38']

0x00117008 cHUDFX_SetText shard=s00 size=60 race=c prof=0 callers=4
  kind=leaf/L blocks=[2, 3, 1, 7, 2] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['cHUDFX_Show', 'cHUDFX_ShowNumber', 'cHUDFX_ShowDelayed', 'cHUDFX_ShowNumberDelayed']

0x00117048 cHUDFX_Show shard=s00 size=96 race=c2 prof=0 callers=1
  kind=C1/F4 blocks=[24] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=4 fcmp=0 mmi=0 sp_stores=5 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['cHUDFX_SetText']
  direct callers: ['RiderMetrix_ShowHUDText']

0x001171a8 cHUDFX_ShowNumberDelayed shard=s00 size=156 race=P prof=3 callers=24
  kind=C2/F7 blocks=[39] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=7 fcmp=0 mmi=0 sp_stores=7 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['cHUDFX_SetText', 'sprintf']
  direct callers: ['RiderMetrix_AddComboPoints', 'RiderMetrix_UpdateTrickHUD', 'FUN_00119448']

0x001179e0 RiderMetrix_RemoveHUDMessage shard=s00 size=120 race=c prof=0 callers=43
  kind=leaf/L blocks=[4, 2, 4, 2, 5, 5, 6, 2] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['RiderMetrix_ResetCombo', 'RiderMetrix_UpdateTrickHUD', 'FUN_00119210', 'RiderMetrix_FinishTrick', 'FUN_0011a7a8']

0x00117b88 RiderMetrix_ShowHUDNumber shard=s00 size=156 race=c prof=0 callers=29
  kind=C2/F4 blocks=[15, 14, 1, 9] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=4 fcmp=0 mmi=0 sp_stores=7 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['RiderMetrix_FindHUDSlot', 'cHUDFX_ShowNumber']
  direct callers: ['RiderMetrix_BankCombo', 'RiderMetrix_UpdateTrickHUD', 'FUN_00119210', 'FUN_00119400', 'FUN_001194c0', 'FUN_001195a8', 'FUN_001195d8', 'FUN_00119608'] ...
```

## D7: function list (classifier)

```text
# D7 Audio system

0x003b8790 SND_SetPitch shard=s13 size=164 race=P prof=4 callers=4
  kind=L/C4 blocks=[11, 5, 6, 2, 4, 5, 1, 7] loops=1 matchpc=True event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=6 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['SND_HandleToChannel*', 'SND_ChannelCalcPitch', 'SND_ChannelApplyPitch', 'SND_NextHandleChannel']
  direct callers: ['BankMonitorInstance_SetPitch', 'PATH_PlatSetPaused', 'SNDSTRM_SetPitch', 'SNDMIDI_SetPitch']

0x003b8c00 SND_SetPan shard=s13 size=200 race=P prof=1 callers=4
  kind=L/C3 blocks=[11, 2, 4, 5, 5, 1, 8, 5, 9] loops=1 matchpc=True event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=7 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['SND_HandleToChannel*', 'SND_ChannelApplyPan', 'SND_NextHandleChannel']
  direct callers: ['BankMonitorInstance_SetPan', 'SNDSTRM_SetPan', 'SNDMIDI_ControlChange', 'SNDMIDI_SetPan']

0x003baa00 SND_NextHandleChannel shard=s13 size=192 race=P prof=1 callers=14
  kind=leaf/L blocks=[10, 2, 1, 6, 1, 4, 7, 3, 3, 1, 4, 2, 2, 2] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['SND_FadeVolume', 'SND_SetVolume', 'FUN_003b8700', 'SND_SetPitch', 'FUN_003b8838', 'SND_SetAuxSend', 'SND_SetLowpass', 'SND_Stop'] ...

0x003ba938 SND_ChannelCalcPitch shard=s13 size=196 race=P prof=1 callers=5
  kind=C1 blocks=[11, 4, 3, 2, 6, 3, 8, 4, 8] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['SND_CentsToPitch']
  direct callers: ['SND_Tick', 'SND_SetPitch', 'SND_PlayPatchVoices', 'SND_SetPitchBend', 'SNDSTRM_StartChannel']

0x003b5860 SND_DefaultLock shard=s13 size=28 race=P prof=3 callers=0
  kind=C1 blocks=[7] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['SND_MutexLock']
  direct callers: []

0x003b5880 SND_DefaultUnlock shard=s13 size=28 race=P prof=3 callers=0
  kind=C1 blocks=[7] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['SND_MutexUnlock']
  direct callers: []

0x003c3380 SND_MutexLock shard=s13 size=96 race=P prof=3 callers=1
  kind=C2 blocks=[12, 4, 8] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['GetThreadId', 'WaitSema']
  direct callers: ['SND_DefaultLock']

0x003c33e0 SND_MutexUnlock shard=s13 size=112 race=P prof=2 callers=1
  kind=C2 blocks=[10, 6, 3, 3, 4, 2] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['GetThreadId', 'SignalSema']
  direct callers: ['SND_DefaultUnlock']

0x003b5540 SND_Service shard=s13 size=172 race=P prof=17 callers=1
  kind=L/C1 blocks=[9, 5, 3, 6, 8, 1, 6, 1, 4] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['SND_Tick']
  direct callers: ['SND_EeThread']

0x002ab958 SSXAudioSystem_UpdateBanksAndStreams shard=s08 size=136 race=P prof=35 callers=1
  kind=C5/I1/F4 blocks=[34] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=4 fcmp=0 mmi=0 sp_stores=4 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['CrowdAudio_Update', 'MusicPlayer_Update', 'SpeechSystem_Update', 'StreamArray_UpdateAll', 'SndLoadQueue_Update']
  direct callers: ['SSXAudioSystem_Update']

0x002ab7a0 SSXAudioSystem_UpdateBankMonitor shard=s08 size=132 race=P prof=30 callers=0
  kind=L/C1/F5 blocks=[12, 1, 3, 10, 7] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=5 fcmp=0 mmi=0 sp_stores=5 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['SSXAudioSystem_UpdateBankMonitorInstance']
  direct callers: []

0x002ab6b0 SSXAudioSystem_UpdateBankMonitorInstance shard=s08 size=236 race=P prof=29 callers=1
  kind=C2/I1/F7 blocks=[12, 5, 3, 2, 8, 2, 2, 4, 1, 4, 2, 4, 4, 6] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=7 fcmp=1 mmi=0 sp_stores=5 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): ['BankMonitorInstance_PollFree', 'BankMonitorInstance_Update3D']
  direct callers: ['SSXAudioSystem_UpdateBankMonitor']

0x002ab8e8 SSXAudioSystem_UpdateTimers shard=s08 size=112 race=P prof=1 callers=0
  kind=L/C2 blocks=[11, 3, 8, 6] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=4 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['BankMonitor_Tick', 'BankMonitor_TimerUpdate']
  direct callers: []

0x002abb80 BankMonitorInstance_PollFree shard=s08 size=88 race=P prof=6 callers=8
  kind=C2 blocks=[7, 2, 2, 4, 3, 4] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['BankMonitorInstance_IsDone', 'BankMonitorInstance_Reset']
  direct callers: ['SSXAudioSystem_UpdateBankMonitorInstance', 'BankMonitorInstance_Pause', 'BankMonitorInstance_Resume', 'BankMonitor_FindFreeInstance', 'BankMonitor_FindInstance', 'BankMonitor_StopByBank', 'BankMonitor_CountPlaying', 'BankMonitor_CountActive']

0x002abc18 BankMonitor_CalcFalloffVolume shard=s08 size=100 race=P prof=1 callers=1
  kind=leaf/F15 blocks=[8, 4, 3, 8, 2] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=15 fcmp=3 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['BankMonitorInstance_GetFalloffVolume']

0x002abc80 BankMonitorInstance_GetFalloffVolume shard=s08 size=28 race=P prof=1 callers=1
  kind=C1 blocks=[7] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['BankMonitor_CalcFalloffVolume']
  direct callers: ['BankMonitorInstance_Update3D']

0x002ad550 BankMonitor_FindInstance shard=s08 size=156 race=P prof=1 callers=12
  kind=L/C1 blocks=[12, 5, 5, 3, 2, 1, 2, 1, 8] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=6 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['BankMonitorInstance_PollFree']
  direct callers: ['FUN_00125448', 'FUN_00280640', 'SSXAudioSystem_IsTrackedSoundPlaying', 'SSXAudioSystem_UpdateTrackedSounds', 'SSXAudioSystem_StopCharacterSpeech', 'SSXAudioSystem_IsCharacterSpeaking', 'CrowdAudio_UpdateInstance', 'FUN_002ad358'] ...

0x002abe78 BankMonitorInstance_SetPitch shard=s08 size=232 race=P prof=8 callers=8
  kind=C7 blocks=[17, 4, 3, 2, 2, 3, 2, 9, 9, 3, 4] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['StreamBuffer_SetPitch', 'SND_Lock*', 'SND_SetPitch', 'SND_Unlock*', 'SNDMIDI_SetPitch']
  direct callers: ['SSXAudioSystem_UpdateBoardBaseLoop', 'SSXAudioSystem_UpdateBoardSurfaceLoop', 'SSXAudioSystem_UpdateBoardSurfaceLoopRemote', 'SSXAudioSystem_UpdateBoardEdgeLoop', 'CrowdAudio_UpdateInstance', 'BankMonitorInstance_Update3D', 'BankMonitorInstance_Pause', 'BankMonitorInstance_Resume']

0x002ad998 BankMonitor_SndTickCallback shard=s08 size=44 race=P prof=2 callers=0
  kind=I1 blocks=[11] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=1
  callees (in order of first call; * = already overridden): []
  direct callers: []

0x002b4388 MusicPlayer_Update shard=s08 size=76 race=P prof=2 callers=3
  kind=C2 blocks=[7, 6, 3, 3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['SongInstance_Update', 'MusicPlayer_AttachLoopBank']
  direct callers: ['SSXAudioSystem_SetChannelVolume', 'SSXAudioSystem_UpdateBanksAndStreams']

0x002b2488 SongInstance_GetStreamHandle shard=s08 size=52 race=P prof=1 callers=8
  kind=C2 blocks=[7, 3, 3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['PATH_GetTrackData', 'PATH_GetTrackStream']
  direct callers: ['SongInstance_Pause', 'SongInstance_Resume', 'SongInstance_SetVolume', 'SongInstance_Update', 'SongInstance_FadeOut', 'SongInstance_GetStreamStatus', 'MusicPlayer_GetSongStreamHandle']

0x00287968 SSXAudioSystem_GetChannelGainPtr shard=s07 size=52 race=c prof=0 callers=76
  kind=leaf blocks=[3, 3, 3, 1, 3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['FUN_00125448', 'FUN_002808e8', 'SSXAudioSystem_ctor', 'SSXAudioSystem_LoadFrontEndAudio', 'FUN_0028c8c8', 'SSXAudioSystem_PlayPeakTransitionMusic', 'SSXAudioSystem_PlayCharacterTune', 'FUN_0028cf98'] ...

0x0028b1d8 SSXAudioSystem_GetRaceMan shard=s07 size=16 race=c prof=0 callers=51
  kind=leaf blocks=[4] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['SSXAudioSystem_FindRider', 'SSXAudioSystem_EnterGame', 'FUN_00286ea0', 'FUN_002871b0', 'SSXAudioSystem_HandleMusicEvent', 'SSXAudioSystem_RestartAllBoardLoops', 'FUN_002947b0', 'SSXAudioSystem_OnRacePositionChange'] ...

0x0028b1c8 SSXAudioSystem_GetGameModule shard=s07 size=12 race=c prof=0 callers=24
  kind=leaf blocks=[3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['FUN_00285930', 'SSXAudioSystem_Update', 'FUN_00286e20', 'FUN_002872a8', 'SSXAudioSystem_IsReplayPlaying', 'SSXAudioSystem_UpdateBoardBaseLoop', 'SSXAudioSystem_UpdateBoardSurfaceLoop', 'SSXAudioSystem_UpdateBoardSurfaceLoopRemote'] ...

0x00288ae0 SSXAudioSystem_IsReplayPlaying shard=s07 size=96 race=P prof=2 callers=60
  kind=C3 blocks=[6, 5, 6, 1, 2, 1, 3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['SSXAudioSystem_IsInGame', 'SSXAudioSystem_GetGameModule']
  direct callers: ['FUN_002854f8', 'SSXAudioSystem_FindRider', 'FUN_00286ea0', 'FUN_0028c8c8', 'SSXAudioSystem_PlayCharacterTune', 'FUN_0028cf98', 'SSXAudioSystem_StartBoardBaseLoop', 'SSXAudioSystem_StartBoardSurfaceLoop'] ...
```

### D7: research notes

The SND_* functions run on the sound EE thread (`SND_EeThread`) and on game
threads behind `SND_Lock`. The SSXAudioSystem/BankMonitor layer runs on the
main thread. The channel table is `*(0x50a8e8+0x1f0)` (= `*(0x50aad8)`),
0x8c bytes per channel; the channel count is `(s16)*(0x50a8e8+0x17e)`.
Semantics come from Ghidra and block costs from the generated code. Entries
marked *(outline)* were not cross-checked block by block.

#### `SND_SetPitch` — 0x003b8790
*164 B · kind: loop + calls 4 (`match pc` dispatcher) · cycles: ≤41 over 8 blocks (11/5/6/2/4/5/1/7) · race P · callers 4*

`int SND_SetPitch(int handle /*a0*/, u16 pitch /*a1*/)`:
- `ch = SND_HandleToChannel(handle)` (replaced). If `ch < 0`, it returns `ch`.
- With the cursor `it = -1` on the stack (`sp+0`), it loops while
  `SND_NextHandleChannel(ch, &it)` returns nonzero:
  - `c = chan[it]`;
  - if `c+0x88` (the u16 requested pitch) already equals `pitch`, it
    **returns 0 at once**; this is the early exit, the remaining channels are
    not visited;
  - otherwise it stores `c+0x88 = pitch`, then calls
    `SND_ChannelCalcPitch(it)` and `SND_ChannelApplyPitch(it)` (`a0` is
    reloaded from `sp+0`).
- It returns `ch`.

**Gotchas:**
- The generator emitted a `match pc` dispatcher, so the replacement must be **re-enterable at the loop head 0x3b87d0** (`util.rs` pattern). The event check is the new shape (it returns on unwind) and sets `c.pc = 0x3b87d0`.
- The first iteration jumps straight to the Next call (block 0x3b8800). The loop-head block computes `chan[it]` with `mult` (`s2 = 0x8c`), so LO/HI are left over.
- Frame 0x60: `sq s0..s3` at +0x50..+0x20, `sd ra` at +0x10; the cursor is at `sp+0`.
- Return addresses: 0x3b87b0, 0x3b87f8, 0x3b8800, 0x3b880c.
- No `SND_Lock` inside; callers such as `BankMonitorInstance_SetPitch` take the lock.

#### `SND_SetPan` — 0x003b8c00
*200 B · kind: loop + calls 3 · cycles: ≤50 over 9 blocks · race P · callers 4*

`int SND_SetPan(int handle, s16 pan /*a1*/, int spread /*a2*/)`:
- It clamps `spread` to [-0x4000, 0x3fff].
- `ch = SND_HandleToChannel(handle)`; if `ch >= 0`, then for every channel `it`
  of the handle (same cursor loop as SetPitch, no early exit) it sets
  `c+0x1e = spread`, `c+0x1c = c+0x4c + pan` (the patch pan plus the offset)
  and calls `SND_ChannelApplyPan(it)`.
- It returns `ch`.

**Gotchas:** it is the same loop shape as SetPitch; write both with one
"for each channel of a handle" iterator that reproduces the cursor in `sp+0`
and the `NextHandleChannel` calls. Frame 0x70 (`aiStack_70`). The clamp
compares are integer `slti`s.

#### `SND_NextHandleChannel` — 0x003baa00
*192 B · kind: leaf + loop · cycles: ≤48 over 14 blocks · race P · callers 14*

`bool NextHandleChannel(int ch /*a0*/, int *it /*a1*/)`, with
`grp = chan[ch].+0x26` (u8 voice group):
- If `grp == 0` (a single-channel handle), it returns 1 once: when
  `*it < 0` it sets `*it = ch` and returns 1, otherwise it returns 0.
- Otherwise it pre-increments `*it` and scans while `*it < count`. A channel
  matches when `chan[*it].+0x26 == grp`, `(s8)+0x69 == 1` (allocated) and the
  word at `+0` is ≥ 0. On a match it returns 1 with `*it` at that index;
  otherwise it advances (`*it = idx+1`, stored every step).
- It returns 0 at the end.

**Gotchas:**
- `*it` is written on every iteration, not only at the end: each store is visible.
- The back edge uses the **old shape** (`c.pc = 0x3baa50; c.event();`, result ignored).
- `mult` by 0x8c leaves LO/HI.
- `t0/t1/t2/a2/a3` are leftovers.

#### `SND_ChannelCalcPitch` — 0x003ba938 *(outline)*
*196 B · kind: calls 1 · cycles: ≤49 over 9 blocks · race P · callers 5*

`void CalcPitch(int idx)`, with `c = chan[idx]`. If the cached pitch
`c+0x86 == 0`, it computes cents:
- `cents = (s16)c+0x84`;
- if the bend range `(s16)c+0x82 != 0`, it adds `((b - 0x40) * range) >> 6`,
  where `b = (s8)c+0x64`, remapped through the table at `c+0x74` when that is
  non-null;
- if the table at `c+0x7c` is non-null, it adds
  `((tbl[(u8)c+0x68] - 0x40) * (s16)c+0x80) >> 6`;
- then `c+0x86 = SND_CentsToPitch(cents)`.

It always stores `c+0x8a = ((u16)c+0x86 * (u16)c+0x88) >> 12`.

**Gotchas:** signed arithmetic shifts (`sra`); `mult` LO/HI. The call to
`SND_CentsToPitch` only happens on the uncached path.

#### `SND_DefaultLock` / `SND_DefaultUnlock` — 0x003b5860 / 0x003b5880
*28 B each · kind: calls 1 · cycles: 7 · race P · callers 0 (function pointer at 0x44c46c)*

Each one tail-wraps `SND_MutexLock()` / `SND_MutexUnlock()`. It is the default
lock hook that the replaced `SND_Lock` calls through `*(0x44c46c)`.

**Gotchas:** a 16-byte frame with `ra`. This is the `tail_wrapper` shape (D1
notes). They are reached only indirectly, so a replacement goes through the
function table (the override slot covers it).

#### `SND_MutexLock` — 0x003c3380
*96 B · kind: calls 2 (both syscall wrappers) · cycles: ≤24 over 3 blocks (12/4/8) · race P · callers 1*

It is a recursive mutex:
- `tid = GetThreadId()`;
- if the owner `*(0x50c814) != tid`, it calls `WaitSema(*(0x50c810))` and then
  sets `owner = tid`;
- it increments the count at `*(0x50c818)`.

**Gotchas:**
- `WaitSema` is the syscall wrapper that can **switch threads**. In host-thread mode the call may not return to 0x3c33xx immediately, so the unwind check after it is essential (`Flow`/`?`).
- The cycle charge order around the syscall (12 before `GetThreadId`, 4 before `WaitSema`, 8 after) is observable by the other thread.

#### `SND_MutexUnlock` — 0x003c33e0
*112 B · kind: calls 2 · cycles: ≤28 over 6 blocks · race P · callers 1*

- `tid = GetThreadId()`.
- If `owner == tid`, it decrements the count; if the count reaches 0, it sets
  `owner = -1`.
- Then, **independently**, if `count < 1`, it calls `SignalSema(*(0x50c810))`.
  It signals even when the caller was not the owner: a quirk to keep.

**Gotchas:** `SignalSema` can wake a higher-priority thread, so the unwind
check is needed. Keep the two separate tests, not an else-if.

#### `SND_Service` — 0x003b5540
*172 B · kind: loop + calls 1 · cycles: ≤43 over 9 blocks · race P (17) · callers 1*

`void SND_Service(int rate /*a0*/)`. If the initialised flag `*(u8*)0x50aa5c`
is set:
- if `rate` differs from the stored `(s16)0x50ac1c`, it stores `rate` and
  resets the tick counter `0x50aa68` and the call counter `0x50ac18`;
- it increments `0x50ac18`;
- it calls `SND_Tick()` while `*(u32*)0x50aa68 <= (calls*100)/rate`. It is a
  catch-up loop that keeps the sound ticks at `100 * calls / rate`.

**Gotchas:**
- The integer `div` with the `teq`-style trap when `rate == 0` (`trap(7)` in Ghidra): reproduce the generated code's behaviour for 0, which is a break/trap path.
- The loop calls `SND_Tick` (it has calls inside) with an event check at its head, so it is a loop-with-calls: new shape, return on unwind.
- The division is recomputed once before the loop (the quotient is kept in a register), so the bound does not change inside the loop even though `SND_Tick` increments `0x50aa68`.

#### `SSXAudioSystem_UpdateBanksAndStreams` — 0x002ab958
*136 B · kind: calls 5 + virtual 1, FPU arg · cycles: 34 (one block) · race P (35) · callers 1*

`void UpdateBanksAndStreams(float dt /*f12*/, SSXAudio *sys /*a1*/)`, with
`snd = *(sys+0x118)`. It calls, in order:
1. `SndLoadQueue_Update(*snd)`;
2. `CrowdAudio_Update(sys)`;
3. a virtual call on the bank-monitor manager `m = *(*snd+0xab0)`: `m->vt+0x2c`
   with `this = *snd + 0x1d8 + (short)vt[+0x28]`. It passes `dt`, so f12 must
   survive (f20 is used to keep it);
4. `MusicPlayer_Update(sys+0x118)`;
5. `SpeechSystem_Update(sys+0x5560)`;
6. `StreamArray_UpdateAll(*snd)`.

**Gotchas:**
- One block, charged up front.
- `dt` is held in callee-saved **f20**, saved with `swc1` in the frame. Reproduce the FPR save/restore and keep f12 = dt at each call that needs it.
- Six return addresses.

#### `SSXAudioSystem_UpdateBankMonitor` — 0x002ab7a0
*132 B · kind: loop + calls 1, FPU · cycles: ≤33 over 5 blocks (12 / 1 / 3 / 10·64 / 7) · race P (30) · callers 0 (virtual)*

`void UpdateBankMonitor(float dt /*f12*/, SSXAudio *sys /*a0*/)`.
- If the pause flag `*(*sys+0x118)+0xac0 != 0`, then `dt = 0`.
- For 64 instances (the counter `s0 = 0x3f` counts down to −1), it calls
  `UpdateBankMonitorInstance(dt, inst = *(**(sys+0x118)+0xac4) + i*0xc0, sys)`.

**Gotchas:**
- `f20` is saved with `swc1` at `sp+0x40` and holds `dt` (`mov.s f20, f12`; zeroed with `mtc1 zero` when paused). f12 is reset from f20 before every call.
- 64 iterations, each with an event check at 0x2ab7e0 (verify the shape in the generated loop).
- The instance array base is reloaded every iteration.

#### `SSXAudioSystem_UpdateBankMonitorInstance` — 0x002ab6b0
*236 B · kind: calls 2 + PMF call, FPU · cycles: ≤59 over 14 blocks · race P (29) · callers 1*

`void UpdateBankMonitorInstance(float dt, BMI *inst /*a1*/, SSXAudio *sys /*a2*/)`.
- If `BankMonitorInstance_PollFree(inst) == 0` and `0.0 < dt`, then:
  - if `inst+0xaa` is nonzero, it makes a **gcc 2.9x pointer-to-member call**
    stored at `inst+0xa8` (`delta = +0xa8`, `index = +0xaa`, function or vptr
    offset at `+0xac`) on `sys`, with `(dt, sys+delta, inst)`;
  - then it calls `BankMonitorInstance_Update3D(dt, inst, listener)`, with
    `listener = (sys && *sys+0x118) ? **(sys+0x118) + 0x1d8 : 0`.

**Gotchas:**
- The `Pmf` helper from D4 (`ComputerActorRider_ThinkCruise` uses the same shape).
- `c.lt.s 0.0, dt` sets FCC.
- f20 holds `dt` across the calls.

#### `SSXAudioSystem_UpdateTimers` — 0x002ab8e8
*112 B · kind: loop + calls 2 · cycles: ≤28 over 4 blocks · race P · callers 0*

`void UpdateTimers(SSXAudio *sys)`, with `snd = **(sys+0x118)`. If
`snd+0x26c == 0`, it calls `BankMonitor_Tick(snd+0x1d8)`, then
`BankMonitor_TimerUpdate(**(sys+0x118) + 0x270 + i*0x2c, sys)` for 48 timers
(`i` counts down from 0x2f).

**Gotchas:** the base pointer is reloaded in each iteration; event check at the
loop head.

#### `BankMonitorInstance_PollFree` — 0x002abb80
*88 B · kind: calls 2 · cycles: ≤22 over 6 blocks · race P · callers 8*

`bool PollFree(BMI *i)`:
- if the state `i+0 == 0`, it returns 1 (free);
- if `i[0x27]` (`+0x9c`, the "stopping" flag) is set and
  `BankMonitorInstance_IsDone(i)`, it calls `BankMonitorInstance_Reset(i)` and
  returns 1;
- otherwise it returns 0.

**Gotchas:** at most two calls. The `v0 = 0` leftovers depend on the path.

#### `BankMonitor_CalcFalloffVolume` — 0x002abc18
*100 B · kind: leaf, FPU · cycles: ≤25 over 5 blocks · race P · callers 1*

`int CalcFalloffVolume(float dist /*f12*/, float radius /*f13*/, int vol /*a0*/)`:
- `d = max(dist - 0.5, 0.0)`;
- if `d < radius && 0.0 < radius`, it returns
  `(int)(((radius - d)/radius)^2 * (float)vol)` (`cvt.w.s` truncates);
- otherwise it returns 0 (Ghidra drops that path).

**Gotchas:** the order is `t = (r-d)/r; t*t*float(vol)`, as separate `mul.s`.
The compares use the FPU key; FCC is left over; `div.s` uses the FPU
divide-by-zero rule; `cvt.w.s` saturates.

#### `BankMonitorInstance_GetFalloffVolume` — 0x002abc80
*28 B · kind: calls 1 · cycles: 7 · race P · callers 1*

A tail wrapper that calls `CalcFalloffVolume` with the float args already in
f12/f13 and `a0 = a1`. Use the `tail_wrapper` shape.

#### `BankMonitor_FindInstance` — 0x002ad550
*156 B · kind: loop + calls 1 · cycles: ≤39 over 9 blocks · race P · callers 12*

`BMI *FindInstance(BankMonitor *bm, int id /*a1*/, int kind /*a2*/)`. It scans
the 64 instances at `*(bm+0x8ec) + i*0xc0` and calls `PollFree(inst)` on
**each** one first, which can reset finished instances: a side effect. It
returns the first non-free instance with `inst+0 == kind` and `inst+4 == id`,
or 0.

**Gotchas:** the scan stops at a match, so `PollFree` is not called on later
instances; keep that. The event check sits at the loop head.

#### `BankMonitorInstance_SetPitch` — 0x002abe78
*232 B · kind: calls 7 · cycles: ≤58 over 11 blocks · race P · callers 8*

`void SetPitch(BMI *i, int pitch)`:
- it clamps `pitch` to [0, 0x3fff];
- it acts only if `|pitch - i[0x2c]| > 0xff`, computed with the unsigned trick
  `(p - old + 0xff) > 0x1fe`; then it stores `i[0x2c] = p` (`+0xb0`) and
  dispatches on the state `i+0`:
  - 1 → `SND_Lock(); SND_SetPitch(i[1], p); SND_Unlock();`
  - 2 → the same with `SNDMIDI_SetPitch`;
  - 3 → `StreamBuffer_SetPitch(i[3], p)`.

`SND_Lock`/`SND_Unlock` are replaced.

**Gotchas:** the unsigned compare (`sltu`). The state branches are compiled as
a compare tree (2 first, then < 3, then 1, then 3), and their block costs differ.

#### `BankMonitor_SndTickCallback` — 0x002ad998
*44 B · kind: virtual 1 · cycles: 11 · race P · callers 0 (registered callback)*

`m = *(gp+0x688)` (the audio system), `obj = m+0x8d8`'s vtable. It calls
`vt[+0xc](m + (short)vt[+8])`.

**Gotchas:** a `VSlot` call with an unwind check; return address 0x2ad9b8. The
callback is invoked from the sound thread's tick.

#### `MusicPlayer_Update` — 0x002b4388
*76 B · kind: calls 2 · cycles: ≤19 over 4 blocks · race P · callers 3*

`song = mp+0x408`. If it is set, it calls `SongInstance_Update(song, 0)`;
then, if `*(mp+0x408)+0x50 != 0` (reloaded), it calls
`MusicPlayer_AttachLoopBank(mp)`.

**Gotchas:** `a1 = 0` explicitly. f12 is passed through untouched (check the
callers: `SongInstance_Update` takes a float `dt`).

#### `SongInstance_GetStreamHandle` — 0x002b2488
*52 B · kind: calls 2 · cycles: ≤13 over 3 blocks · race P · callers 8*

`int GetStreamHandle(void)`: `s = PATH_GetTrackStream(0x11000001)`. If
`s < 0`, it returns −1; otherwise it returns `*PATH_GetTrackData(s)`.

**Gotchas:** the constant 0x11000001 is the music stream id.

#### `SSXAudioSystem_GetChannelGainPtr` — 0x00287968
*52 B · kind: leaf · cycles: ≤13 over 5 blocks · race c · callers 76*

`float *GetChannelGainPtr(SSXAudio *s, int kind, int idx)`: returns
`s+0x636c+idx*4` if `kind == 4 && idx < 10`, else `s+0x62c8+kind*16` (round 1
"Name first" entry).

**Gotchas:** the paths cost 3+3+3, 3+1+3 or 3+3+1+3; follow the generated
block order. The `idx < 10` test is signed.

#### `SSXAudioSystem_GetRaceMan` — 0x0028b1d8
*16 B · kind: leaf · cycles: 4 · race c · callers 51*

Returns `*(*(*(gp-0x848)+0x84)+0xc)`, the same chain as `AI_GetFrameCounter`
(D4) without the last load.

**Gotchas:** v1 is left holding `*(gp-0x848)+0x84`'s value.

#### `SSXAudioSystem_GetGameModule` — 0x0028b1c8
*12 B · kind: leaf · cycles: 3 · race c · callers 24*

Returns `*(*(gp-0x848)+0x84)`. **Gotchas:** v1 is a leftover (the global
pointer).

#### `SSXAudioSystem_IsReplayPlaying` — 0x00288ae0
*96 B · kind: calls 3 · cycles: ≤24 over 7 blocks · race P · callers 60*

- If `!SSXAudioSystem_IsInGame()`, it returns 0.
- `gm = GetGameModule()`. If `gm+0x28` (the replay object) is null, it returns 0.
- `gm = GetGameModule()` again (a second call). It returns
  `1 <= *(gm+0x28)->state <= 9`.

**Gotchas:** three calls, including the duplicate `GetGameModule`; keep both.
The range test is unsigned (`sltiu`-style). Once this package is done all
three callees are hand-written, which makes this a fully typed chain.

### Package notes

- Views:
  - `SndChannel` (0x8c bytes; `+0x1c` pan, `+0x1e` spread, `+0x26` group, `+0x4c` patch pan, `+0x64/+0x68` controller bytes, `+0x69` allocated, `+0x74/+0x7c` tables, `+0x80..+0x8a` pitch fields);
  - the `SndGlobals` block at 0x50a8e8 (`+0x17e` count, `+0x1f0` channels);
  - `SndMutex` at 0x50c810 (sema, owner, count);
  - `BankMonitorInstance` (0xc0 bytes: `+0` state, `+4` id, `+0xc` stream, `+0x9c` stopping, `+0xa8` PMF, `+0xb0` pitch);
  - `SSXAudioSystem` (`+0x118` snd root, `+0x5560` speech, `+0x62c8/+0x636c` gains).
- Helpers: a "channels of a handle" iterator (with the `sp+0` cursor), the
  `Pmf` call (shared with D4), FPR save/restore (`swc1 f20`) in `Frame`, and
  `tail_wrapper` from D1.
- Order:
  1. getters (GetRaceMan, GetGameModule, GetChannelGainPtr) and CalcFalloffVolume;
  2. PollFree, FindInstance;
  3. NextHandleChannel, then SetPitch/SetPan and CalcPitch;
  4. the mutex pair and the default lock hooks;
  5. the SSXAudioSystem update chain and SND_Service.
- Risk: the mutex pair and SND_Service are where thread switches happen.
  Test them with real VBlank/thread events in difftest, as `util.rs` does.

## D8: function list (classifier)

```text
# D8 Memory, containers, replay & script

0x00366ce0 VramHeap_Alloc shard=s12 size=248 race=P prof=14 callers=6
  kind=L/C4 blocks=[15, 3, 6, 5, 8, 7, 18] loops=1 matchpc=False event_checks=2 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=8 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['VramHeap_FindFreeBlock', 'VramHeap_EvictCheapestBlock', 'VramHeap_EvictRange', 'VramHeap_SplitBlock']
  direct callers: ['TexMan_AllocFrameBuffer', 'TexMan_LoadTexture', 'TexMan_BindTextures', 'PS2GraphicsMan_RenderDepthFog']

0x003660a8 VramHeap_LinkFreeBlock shard=s12 size=396 race=P prof=6 callers=5
  kind=leaf/L/MMI/sp1 blocks=[11, 2, 2, 14, 5, 3, 2, 9, 6, 2, 5, 2, 4, 1, 3, 3, 4, 4, 3, 7, 1, 3, 3] loops=1 matchpc=True event_checks=2 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=1 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['VramHeap_Init', 'VramHeap_SplitBlock', 'VramHeap_FreeBlock']

0x00366238 VramHeap_UnlinkSizeList shard=s12 size=148 race=P prof=1 callers=4
  kind=leaf blocks=[7, 5, 6, 6, 1, 5, 5, 2] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['VramHeap_SplitBlock', 'VramHeap_FreeBlock']

0x003667f8 VramHeap_FindFreeBlock shard=s12 size=140 race=c prof=0 callers=1
  kind=leaf/L/MMI blocks=[10, 4, 3, 3, 5, 2, 1, 2, 3, 2] loops=2 matchpc=False event_checks=2 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=1 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['VramHeap_Alloc']

0x003662d0 VramHeap_SplitBlock shard=s12 size=260 race=P prof=5 callers=1
  kind=C4 blocks=[19, 3, 2, 17, 4, 6, 14] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=7 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['VramHeap_UnlinkSizeList', 'VramHeap_PopSpareNode', 'VramHeap_LinkFreeBlock']
  direct callers: ['VramHeap_Alloc']

0x003663d8 VramHeap_FreeBlock shard=s12 size=368 race=P prof=3 callers=9
  kind=L/C6 blocks=[17, 7, 4, 3, 13, 2, 6, 3, 1, 2, 6, 21, 1, 6] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=5 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['VramHeap_UnlinkSizeList', 'VramHeap_LinkFreeBlock', 'VramHeap_RemoveNode']
  direct callers: ['VramHeap_FreeUnlocked', 'VramHeap_FreeAll', 'VramHeap_EvictCheapestBlock', 'VramHeap_EvictRange', 'VramHeap_FreeTextureBlocks', 'TexMan_FreeTexture', 'PS2GraphicsMan_RenderDepthFog']

0x003668f8 VramHeap_EvictCheapestBlock shard=s12 size=248 race=P prof=8 callers=1
  kind=L/C1/MMI blocks=[17, 5, 5, 5, 3, 2, 5, 2, 3, 2, 1, 4, 2, 3, 3] loops=1 matchpc=False event_checks=1 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=1 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['VramHeap_FreeBlock']
  direct callers: ['VramHeap_Alloc']

0x00365fe8 VramHeap_PopSpareNode shard=s12 size=32 race=c prof=0 callers=1
  kind=leaf blocks=[8] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['VramHeap_SplitBlock']

0x00366008 VramHeap_RemoveNode shard=s12 size=160 race=c prof=0 callers=1
  kind=leaf blocks=[8, 5, 6, 6, 1, 5, 5, 4] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['VramHeap_FreeBlock']

0x00319a90 MEM_AllocInternal shard=s10 size=184 race=P prof=1 callers=2
  kind=C4 blocks=[8, 2, 2, 4, 1, 4, 6, 5, 2, 2, 4, 1, 5] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=4 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['MEM_PoolAlloc', 'MEM_HeapMemalignOffset', 'MEM_HeapMemalign', 'EA_MemZero']
  direct callers: ['EA_MemAllocNamed*', 'MEM_PoolAlloc']

0x00319b48 MEM_FreeInternal shard=s10 size=80 race=c prof=0 callers=3
  kind=C3 blocks=[5, 4, 4, 3, 1, 3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['MEM_IsPoolPointer', 'MEM_PoolFree', 'MEM_HeapFree']
  direct callers: ['__builtin_delete*', 'EA_MemFree', 'MEM_ResizeBlock']

0x00317e98 EA_MemFree shard=s10 size=72 race=c prof=0 callers=345
  kind=C3 blocks=[18] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['MEM_FreeInternal', 'EAMutex_Lock', 'EAMutex_Unlock']
  direct callers: ['RiderMetrix_dtor', 'BELibrary_LoadCharDB', 'Bolts_FreeDB', 'FUN_00151368', 'BESaveInt_FreeSaveBuffer', 'FUN_00152728', 'BESaveInt_Destroy', 'RewardsFile_Unload'] ...

0x002523a8 MEM_Free shard=s06 size=36 race=c prof=0 callers=34
  kind=C1 blocks=[3, 2, 4] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['EA_MemFree']
  direct callers: ['Rider_LoadStep', 'FUN_0017a378', 'MEM_HeapFreeAll', 'VidEngine_MemFree', 'PlaybackMan_PollFileLoad', 'ScriptEngine_UnloadMaster', 'ScriptEngine_UnloadWorldScripts', 'SndBank_Load'] ...

0x003e5700 EAMutex_Lock shard=s14 size=96 race=c prof=0 callers=49
  kind=C3 blocks=[8, 4, 8, 4] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['GetThreadId', 'WaitSema']
  direct callers: ['BoltFile_UnpackSlot', 'BoltFile_LoadSlot', 'MEM_HeapFreeBlock', 'MEM_GetBlockSizeLocked', 'MEM_ResizeBlockLocked', 'MEM_HeapShrinkBlock', 'MEM_HeapAllocAligned', 'cCommSystem_PoolAlloc'] ...

0x003e5760 EAMutex_Unlock shard=s14 size=108 race=c prof=0 callers=50
  kind=C3 blocks=[8, 4, 2, 3, 4, 2, 1, 3] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=2 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['GetThreadId', 'iSignalSema', 'SignalSema']
  direct callers: ['BoltFile_UnpackSlot', 'BoltFile_LoadSlot', 'MEM_HeapFreeBlock', 'MEM_GetBlockSizeLocked', 'MEM_ResizeBlockLocked', 'MEM_HeapShrinkBlock', 'MEM_HeapAllocAligned', 'cCommSystem_PoolAlloc'] ...

0x003e6220 EA_MemZero shard=s14 size=32 race=c prof=0 callers=7
  kind=C1 blocks=[8] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['EA_MemSet*']
  direct callers: ['MEM_AllocInternal', 'EAFile_Shutdown', 'EAFile_GetSettings', 'EAFile_SetSettings', 'StreamFile_Create', 'EAFile_CdInit', 'EAThread_Shutdown']

0x0026d760 Replay_RecordSnapshot shard=s07 size=112 race=P prof=4 callers=4
  kind=C3 blocks=[28] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=4 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['Replay_AllocSnapshot', 'WorldState_Write', 'FUN_0026ed88']
  direct callers: ['Replay_Update', 'Replay_SaveResumeSnapshot']

0x0026e9d0 ReplayStream_Write shard=s07 size=76 race=P prof=4 callers=0
  kind=C1 blocks=[19] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=3 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['EA_MemCopy*']
  direct callers: []

0x0026d740 Replay_AddSnapshot shard=s07 size=28 race=c prof=0 callers=4
  kind=C1 blocks=[7] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=1 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['SnapshotList_PushBack*']
  direct callers: ['Replay_Update', 'Replay_SaveResumeSnapshot']

0x00270628 Replay_CancelInstantReplay shard=s07 size=48 race=c prof=0 callers=3
  kind=leaf blocks=[4, 6, 2] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['Replay_Update', 'Replay_EndPlayback', 'Replay_OnRiderReset']

0x0026fa50 Replay_StopRecording shard=s07 size=36 race=c2 prof=0 callers=3
  kind=leaf blocks=[6, 1, 2] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['FUN_0012b340', 'AI_UpdateRaceTimeLimit', 'GameStateFinish_Enter']

0x00309c88 WScriptMan_CallFunc shard=s10 size=148 race=P prof=2 callers=7
  kind=C5 blocks=[37] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=6 sp_adjust=True indirect_calls=0
  callees (in order of first call; * = already overridden): ['World_GetSectionScriptFunc', 'FUN_00226600', 'FUN_002224a8', 'LunoVM_Call', 'LunoVMRegister_dtor']
  direct callers: ['WScriptMan_Shutdown', 'WScriptMan_OnObjectTrigger', 'WScriptMan_RunObjScript1', 'WScriptMan_RunObjScript3', 'WScriptMan_RunKillScript', 'WScriptMan_RunObjScript0', 'WScriptMan_RunObjScript5']

0x0030c820 WScriptMan_EventQueuePop shard=s10 size=80 race=c prof=0 callers=5
  kind=leaf blocks=[5, 2, 13] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['FUN_001f7548', 'FUN_001f7738', 'GameModule_Update', 'GameModule_UpdatePauseAndUI', 'WScriptMan_PopEvent']

0x0030c870 WScriptMan_EventQueuePeek shard=s10 size=40 race=c prof=0 callers=3
  kind=leaf blocks=[4, 2, 4] loops=0 matchpc=False event_checks=0 vu0=0 vu0_acc=0 cfc2_vi22=0 fpu=0 fcmp=0 mmi=0 sp_stores=0 sp_adjust=False indirect_calls=0
  callees (in order of first call; * = already overridden): []
  direct callers: ['GameModule_Update', 'GameModule_UpdatePauseAndUI', 'WScriptMan_PeekEvent']
```
