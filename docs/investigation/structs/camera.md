# Camera structures (GameViewMan, GameCamera, controllers, algorithms)

_Mapped 2026-10-15 from Ghidra and the paused PCSX2 Snow Jam race (PINE). Most
camera functions were already named in naming pass 2; this file adds the data
layouts. Confidence: high / med / low as in `rider.md`._

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
| 0x480 | struct (0x10) | | collision query result (ptr, flags) | `0x336850(..., cam+0x480)` | low |
| 0x490 | vec4 | | saved eye used when +0x4a4 == 0 | AvoidCollision | low |
| 0x4a0 / 0x4a4 | i32 | | collision state flags (0 / 1) | | low |

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
| 0x30 | u32/ptr | flags; bit 1 forces cuts (blend rate 1.0) in `SetAlgorithm` | 0xcdce88 (looks like a pointer, unresolved) | low |
| 0x38 | f32 | override timer (seconds, >10 restores +0x24) | 0 | med |
| 0x3c | i32 | spoke mode (entered when the rider's main state is 5 Handplant, left above 35 speed units) | 0 | med |
| 0x40 | ptr | | 0xcdcedc | low |

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
| 0x1d0 | f32 | smoothed follow distance (200.0 at ctor) | `ApplyFollowDistance` | med |
| 0x1d4 | f32 | smoothed camera height (reset to slot 13's value) | `ApplyCameraHeight` | med |
| 0x1dc | f32 | smoothed FOV scale; `fov = base × const × this` | `ApplyFov` | med |
| 0x1e0 | f32 | 10.0 | ctor, reset | low |
| 0x1e4 | f32 | smoothed look-at height above the rider | `ApplyTargetHeight` | med |
| 0x1e8 / 0x1ec | f32 | extra follow distance that decays / last speed value it was computed from | `ApplyFollowDistance` | med |
| 0x204 / 0x208 | f32 | height bonus for the eye / look-at (held while airborne) | `ApplyCameraHeight` / `ApplyTargetHeight` | med |
| 0x1f0..0x250 (rest) | f32 | other filter state zeroed on reset; +0x224 gp constant, +0x228 = 1.0, +0x248 = 0.5 | `ResetForRider` | low |
| 0x2c0 | i32 | 5000 (distance-reset threshold?) | ctor, reset | low |
| 0x2c4 | i32 | -1 | reset | low |
| 0x2e4 | i32 | 1 after reset | | low |
| 0x2f0 | i32 | "needs reset" flag, slot 11 | `CheckDistanceReset`, `Tick` | med |
| 0x2f4..0x2fc | | written by `GatherRiderState` | | low |
| 0x300 | i32 | 1 after reset | | low |
| 0x304..0x33c | `cCSICubicSpline[5]` (12 bytes each) | tuning curves (by speed) | ctor | med |
| 0x340 | f32[5]×3 | ring of recent values (cleared on reset) | `ResetForRider` | low |
| 0x37c | i32 | | reset | low |
| 0x380 / 0x384 / 0x388 | f32 | gp tuning constants set by the ctor | ctor | low |

Live DEFAULT_3 instance (chase controller's list, 0xcdbc50): fov 0.754, near 10,
far 30000, id 0x3d.

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

- Meaning of most ChaseCameraAlgorithm filter fields (+0x1e4..+0x250) and of the
  five tuning splines; `ApplyFollowDistance` etc. are named but not field-mapped.
- ChaseCamController +0x30/+0x40 hold pointers, not flags, in the live object.
- GameCamera +0x480 collision record.
- Split-screen: second GameCamera absent in the 1-player race, not inspected.

## Renames

None in this group (the camera functions were already named by pass 2).
