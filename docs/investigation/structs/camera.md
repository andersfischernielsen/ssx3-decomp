# Camera structures (GameViewMan, GameCamera, controllers, algorithms)

_Mapped 2026-10-15 from Ghidra and the paused PCSX2 Snow Jam race (PINE). Most
camera functions were already named in naming pass 2; this file adds the data
layouts. Round 2 (same day) added the chase-camera tuning pipeline, the spline
curves and the filter fields, checked against headless RAM dumps of frames
5000/5001/5002 of the scripted race. Confidence: high / med / low as in `rider.md`._

World is Z-up (see `rider.md`). Camera orientation is stored as a quaternion and
turned into a view matrix by `CameraBase_UpdateMatrices` 0x15d928.

## Ownership

```
GameModule+0x84  GameViewMan (0x38, alloc 0x47b618)        live 0x56e140
  +0x04/+0x08    GameCamera* [2] (0x4b0, alloc "GameCamera")  live 0x1597fd0 / 0
                   +0xa0 active CamController*  (= one of the three below)
                   +0xa4 ManualCamController* (0x20)  "Manual Cam Controller"
                   +0xa8 ChaseCamController*  (0x44)  "Chase Cam Controller"
                   +0xac ScriptCamController* (0x20)  "Script Cam Controller"
                   +0xc0 Default_3ChaseCameraAlgorithm (embedded, 0x390) = blended output
CamController+0x08  AlgorithmList* (8)  -> AlgorithmListNode (0x1c) -> CameraAlgorithm*
ChaseCamController+0x20  RiderCameraTarget* (8: vptr 0x45b7a8, Rider*)
```

Per frame `GameCamera_Update` 0x15df98 calls the active controller's slot 2
(Update: updates every algorithm in its list and the blend weights), then
`GameCamera_ComputeView` 0x15e668, which blends the views of the list (weights
+0x0c of each node), runs collision avoidance and shake on the embedded
algorithm at +0xc0, and builds the view matrix at +0x40.

## GameViewMan (0x38)

`GameViewMan_ctor` 0x22e3b8, dtor 0x22e730. Live: `{1, 0x1597fd0, 0, 4, 1, 0, 0, 0, 512.0, 448.0}`.

| off | type | meaning | conf |
|---|---|---|---|
| 0x00 | u32 | number of views/cameras (1, or 2 in split screen) | high |
| 0x04 / 0x08 | `GameCamera*[2]` | one per view; created with slot 2 `Init(fovDefault, 30.0, far, cam, index)` and controller 2 (chase) | high |
| 0x0c / 0x10 / 0x14 | i32 | view mode (4) / 1 / 0 | low |
| 0x18 / 0x1c | i32 | 0 | low |
| 0x20 / 0x24 | f32 | display width / height from the graphics object (`*(0x4a289c)` vt+0x10d8 slots 0x50/0x58): 512 × 448 | high |

## CameraBase / GameCamera (0x4b0)

`CameraBase_ctor` 0x15d698, `GameCamera_ctor` 0x15dc70, `GameCamera_dtor` 0x15dd88.

Vtables (vptr at **+0x90**): CameraBase 0x45b8b0 / GameCamera 0x45b888 —
1 dtor, 2 `Init`, 3 `Update`.

| off | type | name | meaning | evidence (live) | conf |
|---|---|---|---|---|---|
| 0x00 | f32 | `fov` | current vertical FOV, radians (0.754 = 43.2°) | `CameraBase_Init`; 0x0c is the default | high |
| 0x04 | f32 | `near` | 30.0 | | high |
| 0x08 | f32 | `far` | 30000.0 | | high |
| 0x0c / 0x10 / 0x14 | f32 | `fovDefault`, `nearDefault`, `farDefault` | π/4, 30, 30000 | `CameraBase_Init` copies [0..2] to [3..5]; `GameCamera_Init` takes them as args | high |
| 0x18 | i32 | `viewIndex` | 0 for player 1 | `GameCamera_Init` | high |
| **0x20** | vec4 | `eye` | camera position (-141,900, 12,300, -234,900; ~400 units from the rider) | `CameraBase_UpdateMatrices` uses -eye as translation; `ComputeView` writes the blended position | **high** |
| **0x30** | quat | `rot` | camera orientation (unit quaternion) | quat → matrix in `CameraBase_UpdateMatrices` | **high** |
| **0x40** | mat4 | `view` | world→camera matrix (rotation rows, translation row +0x70) | `CameraBase_UpdateMatrices`, `ComputeView` | **high** |
| 0x80 | vec4 | `forward` | view direction (-0.833, -0.11, -0.54) = third rotation column | ctor default (0x4ff140), live | med |
| 0x90 | vptr | | 0x45b888 | | high |
| 0xa0 | `CamController*` | `active` | = +0xa8 in a race (`GameCamera_SetController(cam, 1/2/3)` picks manual/chase/script) | `GameCamera_SetController` 0x15dfd8 | high |
| 0xa4 / 0xa8 / 0xac | `CamController*` | manual / chase / script | | ctor | high |
| 0xb0 | i32 | `playerIndex` | 0 / 1, else -1 | `GameCamera_Init` | high |
| 0xb4 | i32 | | 1 live, cleared by Init | | low |
| 0xc0 | `Default_3ChaseCameraAlgorithm` (0x390) | `blend` | working algorithm the blended view is written into; collision and shake are applied to it (`ChaseCameraAlgorithm_StartShake/ApplyShake(..., cam+0xc0)`) | `GameCamera_ctor`, `ComputeView` | high |
| 0xe0 | vec4 | (`blend`+0x20) | look-at point (≈ rider position) | live | high |
| 0x100 | vec4 | (`blend`+0x40) | desired eye before collision | `GameCamera_AvoidCollision` | med |
| 0x120 / 0x130 | vec4 / quat | (`blend`+0x60/+0x70) | output eye / orientation (copied to +0x20/+0x30) | `ChaseCameraAlgorithm_GetView` | high |
| 0x450 | i32 | `shakePreset` | 0..4, index into the shake preset table 0x4c53e0 (16 bytes each: amplitude, frequency, strength, range) | `GameCamera_AddShake`, `GameCamera_InitShakePresets` | high |
| 0x454 | f32 | `shakeStrength` | strongest request this frame (cleared after apply) | `GameCamera_AddShake` / `UpdateShake` | high |
| 0x458 / 0x45c | i32 | `shakeRequested` / previous | edge-detects the start of a shake | `GameCamera_UpdateShake` | high |
| 0x460 | f32 | `collisionPull` | how far the eye is pulled in by geometry; decays by a gp factor (4.4 live) | `GameCamera_AvoidCollision` 0x15ee00 | med |
| 0x470 | vec4 | `collisionNormal` | normal of the last camera ray hit (-0.33, 0.008, 0.94) | `GameCamera_AvoidCollision` | med |
| 0x480 | struct (0x10) | `collisionHint` | intersect hint for the camera sweep: passed as the 4th argument of `FUN_00336850` (proposed `World_SweepClosestHit`), which forwards it to the world query `FUN_00335d78`; the result goes into a stack record, not here. Same role as `Rider+0x864` "Line Intersect Hint". Live {0x111d400 ptr, 0x70003, 1, 2}; headless {0x10f8a60, 5→4, 0→1, 2} | `GameCamera_AvoidCollision` | med |
| 0x490 | vec4 | `lastClearEye` | last eye position that passed the look-at→eye visibility test; restored when the test fails and +0x4a4 == 0 | AvoidCollision | med |
| 0x4a0 | i32 | `occluded` | 1 while the line from look-at+100 up to the eye is blocked (only tested while a script camera is active, `FUN_0015d020`) | AvoidCollision | med |
| 0x4a4 | i32 | `snapRequest` | one-shot: on an occluded frame, place the eye 180 above / 180 along +X of the look-at instead of restoring +0x490; cleared every frame | AvoidCollision | med |

Shake: while the rider boosts (target slot 17 = `Rider+0x2fc` above a gp
threshold) `GameCamera_UpdateShake` requests preset 1, 2 or 3 by boost strength,
scaled by speed (0 below 60, 1 above 120 in the target's speed units), then applies
the strongest request of the frame through the embedded algorithm.

## CamController (base, 0x18) and subclasses

`CamController_ctor` 0x15cba0 (vt 0x45b938: 1 dtor, 2 `Update` (stub), 3 `ResetForRider`, 4 `Reset`).

| off | type | meaning | live (chase) | conf |
|---|---|---|---|---|
| 0x00 | i32 | controller type: 1 manual, 2 chase, 3 script | 2 | high |
| 0x04 | i32 | | 1 | low |
| 0x08 | `AlgorithmList*` | algorithms being blended (alloc "Algorithm List") | 0x53ff30 | high |
| 0x0c | ptr | input binding of the player's pad (`App+0xb0+4*index`) | 0x53fcf8 | high |
| 0x10 | `GameCamera*` | owner | 0x1597fd0 | high |
| 0x14 | vptr | | 0x45b908 | high |

**ChaseCamController (0x44)** — `ChaseCamController_ctor` 0x1619a8, vt 0x45b908
(2 `ChaseCamController_Update` 0x161bb8). `ChaseCamController_SetAlgorithm`
0x15d078 is the factory from algorithm id to class.

| off | type | meaning | live | conf |
|---|---|---|---|---|
| 0x18 | i32 | current algorithm id | 0x3d | high |
| 0x1c | i32 | auto/default algorithm id (forced to 0x3d unless 0x4a) | 0x3d | high |
| 0x20 | `RiderCameraTarget*` | target (vptr 0x45b7a8 + `Rider*`); `GameCamera_SetTargetRider` writes +4 | 0x53ff48 | high |
| 0x24 | i32 | algorithm to restore after an override | 0x3d | med |
| 0x28 | i32 | | 0x3d | low |
| 0x2c | i32 | override algorithm id (0x5d = timed override) | 0x3d | med |
| 0x30 | u32 | flags: bit 0 = override active (`SetOverrideAlgorithm`, cleared by id 0x4c), bit 1 = script camera active (`BeginScriptCamera` / `EndScriptCamera`; while set, `SetAlgorithm` cuts and overrides are deferred unless +0x2c == 0x5d), bit 2 = spoke mode (`EnterSpokeMode` / `ExitSpokeMode`). Bits 3-31 are never initialised: the reset (`FUN_00161950`) only does `&= ~7`, so the upper bits hold stale heap data (the live 0xcdce88 has bits 0-2 clear; headless 0) | 0xcdce88 / 0 | high |
| 0x34 | f32 | blend rate of the pending override (3rd arg of `SetOverrideAlgorithm`, reused by `BeginScriptCamera(0x5d)`) | 0 | med |
| 0x38 | f32 | override timer (seconds, >10 restores +0x24) | 0 | med |
| 0x3c | i32 | spoke mode (entered when the rider's main state is 5 Handplant, left above 35 km/h: \|vel\| × gp 0.036) | 0 | med |
| 0x40 | i32 | zeroed by the ctor reset and `ResetState`; no other access found (unused) | 0 | med |

The "pointer" seen at +0x40 in round 1 was the word *after* the object (+0x44 =
0xcdcedc, next heap block); +0x40 itself reads 0 live and headless.
`FUN_0015d020` = `ChaseCamController_IsScriptCameraActive` (bit 1 set and +0x2c not
0x5d / 0x3c); `GameCamera_AvoidCollision` uses it.

**ManualCamController (0x20)** — vt 0x45b8d8; +0x18 ManualCamAlgo* ("Manual Cam Algo",
0x40), +0x1c EditorCamAlgo* ("Editor Cam Algo", 0x40). Default selection 0x51.

**ScriptCamController (0x20)** — vt 0x45c428; +0x18 current script camera, +0x1c
array of 2 `ScriptObjCam*` (0x80 each, script-addressable). Update eases each
node's weight from the script object's +0x18 value.

### AlgorithmList (8) and node (0x1c, alloc 0x45b1f0)

List: +0x00 head node, +0x04 count. Node:

| off | meaning |
|---|---|
| 0x00 | `CameraAlgorithm*` |
| 0x04 | blend weight 0..1, advanced by +0x08 per update |
| 0x08 | blend rate (1.0 = cut) |
| 0x0c | eased weight `3w² - 2w³` (used by `ComputeView`) |
| 0x10 / 0x14 | prev / next |
| 0x18 | owns the algorithm (delete on removal) |

Live: one node, algorithm 0xcdbc50 (DEFAULT_3), weight 1.0.

## Camera algorithms

All algorithms start with a common header:

| off | type | meaning | conf |
|---|---|---|---|
| 0x00 | f32 | FOV (radians) | high |
| 0x04 | f32 | near (10.0 for chase cams) | high |
| 0x08 | f32 | far | high |
| 0x0c | i32 | algorithm id | high |
| 0x10 | vptr | | high |

`CameraAlgorithm` vt 0x45bde0 slots: 1 dtor, 2 `GetView(out {vec4 eye; quat rot})`
(pure), 3 `ResetForRider`, 4 `Reset`, 5 `Update`, 6 `GetLookAt(out vec4)`, 7
`GetTuning(out)` (Default_3: four gp floats + flag), 8 `PostUpdate`, 9 `Tick`, 10
no-op, 11 `GetResetFlag` (chase: +0x2f0), 12 `SetTarget`, 13 a float parameter
(Default_3: gp constant). `RiderCameraAlgorithm` (vt 0x45bc38, ctor 0x1622b0) adds
+0x14.. and target support; `ChaseCameraAlgorithm` vt 0x45bbc0; leaf classes
override 1/3/5/7/13 (Default_3 vt 0x45ca38).

### ChaseCameraAlgorithm (0x390; DirectionNe/Matrix/Spoke 0x3a0, Superpipe/Post_Race 0x3b0, Relative 0x3c0, Spline 0x3d0)

`ChaseCameraAlgorithm_ctor` 0x162318; state set by `ChaseCameraAlgorithm_ResetForRider`
0x166c60 and `ChaseCameraAlgorithm_InitFromRider` 0x166550; per-frame steps
`GatherRiderState` 0x162568 → `ApplyTargetHeight` / `ApplyFollowDistance` /
`ApplySideOffset` / `ApplyCameraHeight` / `ApplyFov` / `ApplyShake` → `PostUpdate`.
`GatherRiderState` fills a stack record the Apply steps read (`RiderCamState`):
+0x00 look-at base, +0x10 rider position, +0x40 main state, +0x44 speed term
(probably \|vel\| scaled), +0x4c boost strength (target slot 17 = `Rider+0x2fc`;
widens the FOV and the follow distance), +0x54 target slot 18 (`Rider+0x220`),
+0x58/+0x5c/+0x60/+0x64 = "is Wipeout / Handplant / Air / Railslide" flags (main
state 2/5/1/4).

| off | type | meaning | evidence | conf |
|---|---|---|---|---|
| 0x20 | vec4 | look-at point | slot 6 returns it; `SetLookAtFromRider` | high |
| 0x30 | `RiderCameraTarget*` | target | `RiderCameraAlgorithm_SetTarget` (slot 12) | high |
| 0x40 | vec4 | desired eye | `GatherRiderState`, `CheckDistanceReset` | med |
| 0x50 / 0x54 / 0x58 | f32 | 0 | ctor | low |
| 0x60 | vec4 | output eye | `ChaseCameraAlgorithm_GetView` | high |
| 0x70 | quat | output orientation | `GetView` | high |
| 0x80 | vec4 | rider forward at reset | `ResetForRider` (target slot 4) | high |
| 0x90 | vec4 | | ctor | low |
| 0xc0 / 0xd0 / 0xe0 | vec4 | smoothing state, zeroed on reset | | low |
| 0xf0 | vec4 | rider velocity at reset, minus its component along up | `ResetForRider` | med |
| 0x100 | vec4 | smoothed horizontal velocity (z zeroed) | `GatherRiderState`, `ResetForRider` | med |
| 0x170 | vec4 | zeroed on reset | | low |
| 0x190 | vec4 | rider velocity at reset | `ResetForRider` (target slot 3) | med |
| 0x1a0 | vec4 | forward × 250 | `ResetForRider` | med |
| 0x1b0 | vec4 | smoothed rider forward (kept when the new value is degenerate) | `GatherRiderState` | high |
| 0x1c0 | vec4 | forward × 2 | `ResetForRider` | low |
| 0x1d0 | f32 | `followDist`: smoothed eye distance behind the look-at (200.0 at ctor; 439→440→441 in the headless frames 5000-5002) | `ApplyFollowDistance` | high |
| 0x1d4 | f32 | `eyeDrop`: smoothed amount subtracted from the eye along world up (reset to slot 13's value) | `ApplyCameraHeight` | high |
| 0x1d8 | f32 | changes slowly (0.469→0.471→0.473); writer not identified (not one of the DEFAULT_3 steps) | dumps | low |
| 0x1dc | f32 | `fovScale`: smoothed; `fov = base × π/4 × this` (1.0 live) | `ApplyFov` | high |
| 0x1e0 | f32 | `handplantBlendTime`: seconds since the handplant ended, +1/60 per frame; the eye eases back from the saved offset +0xb0 while < blend time; ≥ 5 = idle (10.0 at ctor) | `FUN_00163450` | med |
| 0x1e4 | f32 | `lookAtHeight`: smoothed look-at lift along world up (40.4 headless) | `ApplyTargetHeight` | high |
| 0x1e8 / 0x1ec | f32 | `boostDistBonus` (= gain × √boost when boost rises, then decays by a fixed step per frame) / last boost value seen | `ApplyFollowDistance` | high |
| 0x1f0 | f32 | 0 | | low |
| 0x1f4 | f32 | `airCurveTime`: x at which the air-arc splines are evaluated; walks toward +0x200 by 1/60 per frame | `FUN_001635f8` | med |
| 0x1f8 / 0x1fc | f32 | `jumpCurveGain` / `jumpCurveTimeScale` (1.0 / 1.0 when a curve is built) | `FUN_001635f8` | med |
| 0x200 | f32 | `airCurveEnd`: predicted air time (target slot 12) when the air-arc curve was built, 4.0 for a takeoff kick | `FUN_001635f8` | med |
| 0x204 | f32 | `eyeSlopeBonus`: speed² × gain × (slot 18 − 0.5), held while airborne (4.667 headless) | `ApplyCameraHeight` | med |
| 0x208 | f32 | `lookAtSlopeTerm`: slot 18 (`Rider+0x220`), held while airborne (1.0 headless) | `ApplyTargetHeight` | med |
| 0x20c / 0x210 | f32 | air-arc spline outputs for look-at / eye (spline 0 / spline 1) | `FUN_001635f8` | med |
| 0x214 | f32 | `railWeight`: target slot 24 (`Rider+0x5ac` rail flag) low-passed with factor 0.98 | `FUN_001641c0` | med |
| 0x218 | f32 | `kickCurveTime`: x for the takeoff-kick splines, +1/60 per frame up to 4.0 (start 1/60) | `FUN_001635f8` | med |
| 0x21c / 0x220 | f32 | takeoff-kick spline outputs for look-at / eye (spline 2 / spline 3); after landing multiplied by +0x224 every frame (headless: −0.2483 → −0.2309 → −0.2148, ratio 0.93) | `FUN_001635f8`, dumps | high |
| 0x224 | f32 | `landingDecay`: per-frame decay of all four jump offsets after landing; 0.93 by default, raised toward 1 for long air times (slot 23 between 1.05 and 1.4) | `FUN_001635f8` | high |
| 0x228 | f32 | `kickSize`: jump-size factor from target slot 21 (`Rider+0x5a4`) mapped 300..500 → 0..1 (0.516 headless); scales the air-arc amplitude | `FUN_001635f8` | med |
| 0x22c | f32 | air-swing elapsed air time (+1/60 per frame, cleared at takeoff of a drop) | `FUN_00162a20`, `FUN_00164878` | med |
| 0x230 / 0x234 | f32 | air-swing duration (remaining predicted air time, min 2.3 s) / elapsed | `FUN_00164878` | med |
| 0x238 | f32 | air-swing angle (π/2 − asin of eye-dir · swing-target dir) | `FUN_00164878` | med |
| 0x23c | f32 | air-swing blend-out timer (set to the 1.527 s argument, counts down 1/60) | `FUN_00164878` | med |
| 0x240 / 0x244 | f32 | post-swing blend-back timer / its start value | `FUN_00164878` | med |
| 0x248 | f32 | `lowSpeedBlend`: 0..1 weight of the low-speed hold (0.5 at reset) | `FUN_001643a8` | med |
| 0x24c / 0x250 | f32 | 0 in every dump; no writer found in the DEFAULT_3 steps | | low |
| 0x2b8 | f32 | smoothed extra eye height added during/after a handplant | `FUN_00163450` | low |
| 0x2c0 | i32 | `framesSinceHandplant`: counts up to 820 when not in Handplant; 5000 at ctor (= never); drives a fade weight (`RiderCamState+0x48`) that is 1 for 120 frames then falls by 1/700 per frame | `FUN_00162998` | med |
| 0x2c4 | i32 | `takeoffCountdown`: set to 15 on takeoff when no air-arc curve is running and the rider was grounded for more than 30 frames, decremented per airborne frame; at 14 the takeoff-kick splines are built, at 0 (or early, when a landing is predicted) the air-arc splines; keeps going negative in the air (−91 headless) | `FUN_00162a20`, `FUN_001635f8` | med |
| 0x2cc | i32 | AI frame stamp (`AI+0x08`, via `FUN_001298c8`) of the last airborne frame (0x2e8 headless) | `FUN_00162a20` | high |
| 0x2d0 | i32 | `isDrop`: set at takeoff when the velocity points steeply down (z/\|v\| > 0.85) and the previous ground normal is near-vertical (z < 0.1): use the air swing instead of the jump curves | `FUN_00162a20` | med |
| 0x2d4 | i32 | `wasAirborne` | `FUN_00162a20` | high |
| 0x2d8 | i32 | air-swing set up | `FUN_00164878` | med |
| 0x2e0 / 0x2e8 | i32 | air-arc curve active / takeoff-kick curve active | `FUN_001635f8` | med |
| 0x2e4 | i32 | landed: jump offsets are now decaying by +0x224 (1 after reset) | `FUN_001635f8` | med |
| 0x2f0 | i32 | "needs reset" flag, slot 11 | `CheckDistanceReset`, `Tick` | med |
| 0x2f4 / 0x2f8 / 0x2fc | f32 / f32 / i32 | copies from `GatherRiderState`: boost (slot 17), (boost + 1) / 2 or 0, Handplant flag | `GatherRiderState` | high |
| 0x300 | i32 | `lowSpeedState`: 0 blending in, 1 blending out, 2 held, 3 off (3 live) | `FUN_001643a8` | med |
| 0x304 / 0x310 | `cCSICubicSpline` | spline 0 / 1: air-arc look-at / eye height, 13 knots, x = air time × {0, 0.33, 0.66, 1 .. 4}/4, y = amplitude × {−121, −218, −255, −251, −193, −63, 111, 307, 496, 690, 821, 849} (look-at) and {−22, −35, 14, 92, 213, 383, 624, 813, 978, 1071, 1105, 1131} (eye) | `FUN_001635f8` | high |
| 0x31c / 0x328 | `cCSICubicSpline` | spline 2 / 3: takeoff-kick look-at / eye, same x/y tables but x in seconds 0..4 and amplitude = 1.811 × 0.136 × `kickSize` | `FUN_001635f8` | high |
| 0x334 | `cCSICubicSpline` | spline 4: air-swing easing profile 0→1, 9 knots at x = 0, 0.125 .. 1 with y = 0, 0.064, 0.308, 0.553, 0.658, 0.729, 0.827, 0.941, 1 (start slope 0) | `FUN_00164878` | high |
| 0x340 / 0x354 / 0x368 | f32[5] each | chase-heading PID ring: error, output and (error − output) history, indexed by +0x37c (frames 5000-5002 show index 3→4→0) | `FUN_00162c78`, dumps | high |
| 0x37c | i32 | ring index 0..4 | | high |
| 0x380 / 0x384 / 0x388 | f32 | PID gains P = 0.0114, I = 0, D = 0.00128 (getters `FUN_00167d88/98/a8` return gp defaults; the ctor then overwrites with gp 0x9500 / 0 / gp 0x94fc) | ctor, live | high |

Live DEFAULT_3 instance (chase controller's list, 0xcdbc50): fov 0.754, near 10,
far 30000, id 0x3d.

`cCSICubicSpline` (12 bytes): +0x00 bitfield (bit 2 = solved, bits 3-10 = knot
count, bits 11-18 = capacity), +0x08 knot array (0x14 per knot: x, y, three
coefficients). `cCSICubicSpline_AddPoint` solves when the last knot is added.

### Where the chase tuning comes from

There is **no tuning file**. Every chase parameter is a compiled-in constant in
gp-relative `.sdata` (identical in the PCSX2 RAM and the headless dump, no
writers in Ghidra), passed as arguments by each leaf class's `Update`. Speeds are
written in km/h converted to cm/s (27.78 = 1 km/h, 833.3 = 30, 1388.9 = 50,
1666.7 = 60), which also confirms that the rider's distance unit is the
centimetre. Times are in seconds with a 1/60 step. The five splines are rebuilt at
runtime from these constants on every jump. `ChaseCameraAlgorithm_StaticInit`
0x1673f8 fills a block of 0..1 records at 0x4c5020..0x4c539c (read only through
computed indices, purpose unknown) and the axis-remap matrix 0x4c53a0
(world Z-up → view: used by `CameraBase_UpdateMatrices`), then the shake presets.

`Default_3ChaseCameraAlgorithm_Update` 0x176e10 (the race camera) runs, in
order, with these arguments (gp offset → value):

| step | function | arguments (DEFAULT_3) | effect |
|---|---|---|---|
| 1 | `GatherRiderState` | 0.271, 0.9, 0.294, 0.851 (normal ground) / 0.85, 0.97, 0.1, 0.6 (surface types 2, 3, 0xd) | low-passes the horizontal velocity into +0x100 with a factor between a and b by forward-vector steepness \|fwd.z\| in [c, d]; fills `RiderCamState` |
| 2 | `FUN_00162998` | | handplant fade weight (+0x2c0) |
| 3 | `FUN_00162a20` | | takeoff / landing bookkeeping (+0x2c4, +0x2cc, +0x2d0, +0x2d4) |
| 4 | `SetLookAtFromRider` | | look-at = rider hips |
| 5 | `FUN_00162c78` | | chase heading +0x80: PID on the angle between +0x80 and the smoothed forward +0x1c0 (gains +0x380..), blend factor 0.0395 × (1 − PID) + 0.956, fourth-rooted on ground materials 0xb..0xd |
| 6 | `ApplyFollowDistance` | base 300.9, speed gain 11.55, speed offset 76.7, boost gain 10.76, boost decay 1.77/frame, smoothing 0.924 (growing) / 0.907 (shrinking) | `dist = min(5 × base, base + gain × (v − off)^1.5 × 1e-4) + boostBonus`; eye = look-at − chaseDir × dist |
| 7 | `ApplyCameraHeight` | 11.68, 31.2, boost 11.39, slope 9.33, smoothing 0.978 | drop = 11.68 × (v − 1388.9)^1.5 × 1e-4 − 31.2 + max(slope bonus +0x204, 11.39 × √boost) |
| 8 | `ApplyFov` | speed² term 0, base 0.96, boost widen 1.0, smoothing 0.95 (0.999 outside Ground/Air) | `fov = 0.96 × π/4 × fovScale` (0.754 rad live) |
| 9 | `FUN_00163450` | 348.6, 66.3, 162.3, 2.0, 0.98, 0.96, 0.455 | handplant eye hold and blend back (+0x1e0, +0xb0, +0x2b8) |
| 10 | `ApplyTargetHeight` | base 31.1, speed 0.375, slope 12.33, boost 10.59, smoothing 0.924 | lift = base + clamp01((v − 1388.9) × 5.1e-4) × 0.375 + boost × 10.59 + 12.33 × slot 18 |
| 11 | `FUN_001635f8` | kick 1.811, air blend 2.495 s, amplitude 0.136, landing 0.854 | jump curves (splines 0-3) |
| 12 | `FUN_00164878` | blend-out 1.527 s | air swing for drops (spline 4) |
| 13 | `FUN_001646a0` | up 146.4, smoothing 0.97 | rail alignment offset +0xc0 (only while Railslide) |
| 14 | `FUN_001641c0` | side 600, back 300, up 100, rate 0.98 | rail pull-back weighted by +0x214 |
| 15 | `FUN_001643a8` | below 15 km/h (v cm/s × 0.036 = km/h), not airborne, not in Handplant; in 1.5 s, out 1.5 s | low-speed hold: look-at moves to the rider, distance halves toward +0x1a0 |

Each step ends with `ChaseCameraAlgorithm_CheckDistanceReset`. Other leaf classes
(DEFAULT_2/4, Idle, Mid, Lazy, ...) call the same steps with their own constant
sets.

### Algorithm ids (`ChaseCamController_SetAlgorithm`)

| id | class (alloc tag) |
|---|---|
| 0x3d and most ids 0..0x4a | DEFAULT_3 Chase Camera Algorithm (race default) |
| 1, 0x3e | DEFAULT_4 |
| 2, 0x3c | DEFAULT_2 |
| 7 | Idle |
| 9 / 0xa | Mid Right / Mid Left |
| 0xb | Manual Chase |
| 0xd | Lazy |
| 0x12..0x19 | Direction N (8 directions, arg 0..7) |
| 0x20 | Look Back |
| 0x21..0x28, 0x5e..0x61 | Relative (angle = (id-0x21) × gp constant, or fixed gp constants) |
| 0x29 | Matrix |
| 0x42 | SPOKE (handplant) |
| 0x43 | none |
| 0x44 | POST_RACE_1 |
| 0x5b | SUPERPIPE (trigger type 2) or Bounded Camera Algorithm (0x90) |
| 0x5c | Spline Camera Algorithm (4 spline points from a camera trigger) |
| 0x4c | override set by `GameCamera_Init` |
| 0x51 | manual controller default |

### RiderCameraTarget (8 bytes, vptr 0x45b7a8)

The camera reads the rider only through this interface (`+0x04` = `Rider*`). Its
slots are the clearest list of rider fields the camera depends on:

| slot | returns | rider field |
|---|---|---|
| 1 | position | hips bone (`Rider_GetHipsPosition`) |
| 2 | orientation | `+0x120` |
| 3 | velocity | Rider vslot 2 (`+0x1e0`) |
| 4 / 5 / 6 | forward / side / up | `+0x1b0` / `+0x1a0` / `+0x1c0` |
| 7 | f32 | `+0x1f0` (slewed #0) |
| 8 / 9 | ground normal / previous normal | `+0x370` / `+0x380` |
| 10 / 11 | handplant data | `owner+0x130` / `owner+0x140` |
| 12 / 13 / 15 | air times | total `AirPredictor+0x98`, remaining `+0x98-+0xa0`, `+0x9c-+0xa0` |
| 14 | jump prediction origin | `&AirPredictor+0x40` |
| 16 | vec | `&+0x3c0` |
| 17 | f32 | `+0x2fc` (boost) |
| 18 | f32 | `+0x220` (slewed #2 target) |
| 19 | bool | AirPredictor phase is 1 or 3 |
| 20 | i32 | `+0x438` surface type |
| 21 / 22 / 24 | air trick values | `+0x5a4` / `+0x5a8` / `+0x5ac` |
| 23 | f32 | computed from `AirPredictor+0x10/+0x20` |
| 25 | landing-surface normal | `&AirPredictor+0x20` |

(Slots 2-25 are not Ghidra functions; read with a disassembler.)

## Open questions

- Resolved in round 2: the +0x1d0..+0x250 filter fields, the five splines, the
  tuning source (compiled-in constants), ChaseCamController +0x30 (flag bits with
  stale upper bits) and +0x40 (unused), GameCamera +0x480 (intersect hint).
- Still open: ChaseCameraAlgorithm +0x1d8 and +0x24c/+0x250 writers (probably
  other leaf classes); the exact float-argument order of `FUN_00163450` (the
  decompiler loses f15-f18); the 0..1 record block 0x4c5020..0x4c539c.
- Need an airborne dump to confirm the jump-curve fields live (in the headless
  frames 5000-5002 the player is grounded; only the landing decay was seen).
- Split-screen: second GameCamera absent in the 1-player race, not inspected.

## Renames

None in this group (the camera functions were already named by pass 2).

### Proposed names (round 2, not applied: renames are owned by another agent)

| address | proposed name | evidence | conf |
|---|---|---|---|
| 0x162998 | ChaseCameraAlgorithm_UpdateHandplantFade | +0x2c0 frame counter reset by the Handplant flag, writes `RiderCamState+0x48` | med |
| 0x162a20 | ChaseCameraAlgorithm_UpdateAirTransitions | +0x2d4 was-airborne, +0x2d0 drop test, +0x2c4 = 15 at takeoff, +0x2cc frame stamp | med |
| 0x162c78 | ChaseCameraAlgorithm_UpdateChaseHeading | 5-entry PID ring +0x340..+0x37c with gains +0x380..+0x388, updates +0x80 | high |
| 0x163450 | ChaseCameraAlgorithm_ApplyHandplantHold | saves look-at − eye in +0xb0 during Handplant, eases back over +0x1e0 | med |
| 0x1635f8 | ChaseCameraAlgorithm_ApplyJumpCurves | builds/evaluates splines +0x304..+0x328 | high |
| 0x1641c0 | ChaseCameraAlgorithm_ApplyRailPullback | target slot 24 (rail flag) → +0x214, offsets the eye by (600, 300, 100) × weight | med |
| 0x1643a8 | ChaseCameraAlgorithm_ApplyLowSpeedHold | state machine +0x300 / weight +0x248, triggers below 15 km/h (v × 0.036) | med |
| 0x1646a0 | ChaseCameraAlgorithm_ApplyRailAlign | smooths an eye correction into +0xc0 while Railslide | med |
| 0x164878 | ChaseCameraAlgorithm_ApplyAirSwing | spline +0x334, swing fields +0x22c..+0x244, +0x110..+0x180 | high |
| 0x161950 | ChaseCamController_Reset | called by the ctor; sets ids to 0x3d, clears +0x34..+0x40, `+0x30 &= ~7` | high |
| 0x15d020 | ChaseCamController_IsScriptCameraActive | flag bit 1 and +0x2c not 0x5d/0x3c | high |
| 0x1298c8 | AI_GetFrameCount | returns `GameModule->AI+0x08` | high |
| 0x167d88 / 0x167d98 / 0x167da8 | ChaseCameraAlgorithm_DefaultHeadingP / I / D | return gp constants stored into +0x380/+0x384/+0x388 | med |
| 0x167db8 | ChaseCameraAlgorithm_ClearHeadingRing | fills the +0x340 ring (5 × error/output, zero integral) | high |
| 0x336850 | World_SweepClosestHit | runs the world sweep (`FUN_00335d78`, 64 hits), keeps the closest (ties by object/poly order), copies the 0x80-byte hit, returns its fraction or −1 | med |
| 0x32e100 | SweepQuery_Init | builds the sweep record (radius 0.5, from, to, flags 2) consumed by 0x336850 | low |
