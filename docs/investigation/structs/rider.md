# Rider structures (Rider, RiderBase, Player, ComputerActorRider)

_Mapped 2026-10-15 from Ghidra (`SLUS_207.72`), a paused PCSX2 Snow Jam race (PINE)
and two RAM dumps of our own headless runner one frame apart (frames 5000/5001
of the scripted race in `plans/playable-push.md`)._

Confidence: **high** = read from code and confirmed by a live value or by an
unambiguous use; **med** = read from code, meaning inferred from how it is used;
**low** = a guess from one use or from values only. Offsets are hex, sizes in bytes.

## Conventions worth knowing first

- **World is Z-up.** The ground normal of a rider on a slope is mostly +Z
  (`(-0.23, 0.01, 0.97)` live), and a rider descending moves along -Z.
- **Rider local axes:** X = side, **Y = forward**, Z = up. The rider's world
  matrix rows are `x_axis`, `y_axis` (forward, matches the velocity direction),
  `z_axis` (up), `translation`.
- **Units:** distance units per second for velocities; the per-frame position
  delta equals `velocity / 60` (frames 5000→5001: Δpos = (-33.3, 0.51, -30.0),
  velocity = (-1988, 30.8, -1830)). Rider physics runs once per 60 Hz field.
  Typical race speeds are 2000-3300 units/s; the unit is probably the centimetre
  (unconfirmed).
- **Vectors** are 16-byte aligned `vec4` (VU0 `lqc2/sqc2`); positions carry w = 1,
  directions w = 0. Orientations are quaternions `(x, y, z, w)`.
- gcc 2.95 multiple and virtual inheritance: a class's vptr sits **after** the
  base data members, so `Rider`'s vptr is at +0x6c0. The class with a virtual base
  stores a *pointer* to it in each sub-object (`RiderBase+0x18`, `+0x24`, `+0x70`, ...
  all hold `this + 0xdf0`, or `+0xe10` in Player, `+0xf50` in ComputerActorRider).
  Virtual calls are `vt[k] = {short delta; short idx; fnptr}`, slot k's pointer at
  `vt + 8k + 4`, `this` adjusted by `delta`.

## Class layout overview

```
Player (0x1950, alloc "Player")                ComputerActorRider (0x1a90, "ComputerRider"/"ComputerActorRider")
 +0x000 RiderBase (0xdf0)                       +0x000 RiderBase (0xdf0)
 +0xdf0 Player fields (0x20)                    +0xdf0 ComputerActorRider fields (0x160)
 +0xe10 Rider (virtual base, 0xb40)             +0xf50 Rider (virtual base, 0xb40)

RiderBase (0xdf0 non-virtual part)
 +0x000 .. +0x1c0  main-state objects (Ground, Air, Wipeout, Static, Railslide, Handplant)
 +0x1c0 .. +0x3b0  sub-state objects (Cruise ... UberGrind, +1 unnamed)
 +0x3b0 .. +0xde0  rider FX mixins (BoardWake, BoardSpark, BoardTrail, SpeedBoostStreamer, ...)
 +0xde0 main state, +0xde4 sub state, +0xde8 RiderBase vptr
 (every mixin holds a Rider* to the shared virtual base)

Rider (0xb40)
 +0x000 .. +0x6c0  physics/gameplay data (no vptr; this is a base class whose vptr is at +0x6c0)
 +0x6c0 Rider vptr (primary)
 +0x6d0 scriptable-object base (vptr +0x6d0, ScriptObj at +0x6dc with vptr +0x6e8, matrix +0x700)
 +0x750 .. +0xb40  owner pointer, sub-objects, skeleton bone indices, IK, collision, flags
```

Allocation (`AI_CreatePlayers` 0x1298e0, `AI_CreateComputerRiders` 0x129e20,
`AI_CreateActorRiders` 0x129ff8):

| class | size | alloc tag | ctor | Rider at |
|---|---|---|---|---|
| Player | 0x1950 | "Player" (0x4a1108) | `Player_ctor` 0x125c70 | +0xe10 |
| ComputerActorRider (CPU racer) | 0x1a90 | "ComputerRider" (0x457f30) | `ComputerActorRider_ctor(p, 1, 0)` 0x108c80 | +0xf50 |
| ComputerActorRider (scripted actor) | 0x1a90 | "ComputerActorRider" (0x457f40) | `ComputerActorRider_ctor(p, 1, 1)` | +0xf50 |

The second ctor argument is gcc 2.95's "construct virtual bases" flag; the third
(actor) sets `Rider+0x878 = 1`.

## Global access paths (live in the paused PCSX2 race)

| path | value | meaning |
|---|---|---|
| `*(0x4a28a8)` (gp-0x848) | 0x4c9428 | static `SSXApp` object |
| `App+0x84` = `*(0x4a2c68)` | 0xcbfa20 | `GameModule` |
| `GameModule+0x0c` | 0x5b3300 | "AI" race manager (alloc "AI", 0xd0 bytes, `AI_ctor` 0x1286a0) |
| `AI+0x28 + 4*i` | 0x14701a0, 0x18d0c40, ... | `Rider*` for rider index i (6 in a race) |
| `AI+0x40 + 4*i` | 0x146f390 | `Player*` (owner, i < 2) |
| `AI+0x48 + 4*i` | 0x18cfcf0, ... | CPU `ComputerActorRider*` |
| `AI+0x5c + 4*i` | | actor `ComputerActorRider*` |
| `AI+0x78/+0x7c/+0x80/+0x84` | 6 / 1 / 5 / 0 | rider count / players / CPUs / actors |
| `Rider+0x77c` | 0x146f390 | back pointer to the owning `RiderBase` (Player/CAR) |

`AI_Update` (0x128af0) collects the active `Rider*`s and runs one phase over all
riders before the next (see "Per-frame update" below).

## Rider (virtual base, 0xb40 bytes)

Ctor `Rider_ctor` 0x11b698 (calls the data-base ctor 0x140738 for +0x000..+0x6c0,
then the scriptable base 0x27f968 at +0x6d0); dtor `Rider_dtor` 0x11b978.
Loading is incremental: `Rider_Load` 0x11c230 / `Rider_LoadStep` 0x11c298
(state at +0x880, 7 = loaded; `Rider_IsLoaded` 0x11d640).

### Fields

| off | type | name | meaning | evidence | conf |
|---|---|---|---|---|---|
| 0x000 | `RiderRelation[6]` (0x24 each) | `relations` | one record per *other* rider index: +0x00 valid, +0x08 2-D distance, +0x0c angle to that rider, +0x14 frame stamp, +0x1c "aggressive" flag | `AI_UpdateRiderRelations` 0x10f560 (every 6th frame), stamps live 0x2ec/0x2ed | med |
| 0x0d8 | i32 | | -1 on reset | `Rider_Reset` | low |
| 0x0dc / 0x0e0 | f32 | | written by the CPU AI (`0x10c4f8`, `0x10c758`: ±100, ±50000 live) — likely relation window (nearest ahead/behind) | AI code | low |
| 0x0f0 / 0x0f4 / 0x0f8 | i32 | | AI target bookkeeping; 0xf8 = index of another rider (-1 none) | `0x10d410`, `0x10f878`, `Rider_Reset` | low |
| 0x100 | i32 | | 1 after reset | `Rider_Reset` | low |
| **0x110** | vec4 | `pos` | world position (w = 1) | `Rider_Translate`, `Rider_Place`, vslot 5 `GetPosition` returns `&+0x110`; Δ/frame = vel/60 | **high** |
| **0x120** | quat | `rot` | orientation quaternion (unit) | `Rider_Place` (yaw from heading), normalised in `Rider_BuildWorldMatrix`; camera target slot 2 | **high** |
| 0x130 | vec4 | `rootPoseOverridePos` | root-bone position used when +0x150 != 0 | skeleton update 0x11eb98 | med |
| 0x140 | quat | `rootPoseOverrideRot` | root-bone rotation used when +0x150 != 0 | 0x11eb98 | med |
| 0x150 | i32 | `useRootPoseOverride` | | `Rider_Place` clears, 0x11eb98 | med |
| 0x160 | mat4 | `renderMatrix` | model matrix after pose/lean adjustments (equals +0x1a0 when upright) | written by 0x11eb98, read by FX/collision | med |
| **0x1a0** | mat4 | `world` | rotation rows x / **y = forward** / z = up, +0x1d0 = translation | `Rider_BuildWorldMatrix` 0x11e098; camera target slots 4/5/6 return +0x1b0/+0x1a0/+0x1c0 | **high** |
| **0x1e0** | vec4 | `vel` | velocity, units/s | `Rider_AddVelocity`, vslot 2 `GetVelocity` returns `&+0x1e0`; frame diff | **high** |
| 0x1f0 | `Slew[19]` (12 bytes: `cur, maxStep, target`) | `slewed` | 19 scalars moved toward `target` by at most `maxStep` per frame (lean, crouch, board flex ...). #11 (+0x274) and #16 (+0x2b0) are read by tricks/camera; #3 (+0x214) feeds +0x750 | `Rider_UpdateSlewedParams` 0x1211f8 | high (structure) / low (meaning per slot) |
| 0x2dc / 0x2e0 | f32 | | air/spin timers cleared on state changes (Spin sub-state writes +0x2e0) | `RiderSpinSub_*`, `RiderWipeoutState_Enter` | low |
| **0x2e4** | f32 | `targetSpeed` | speed the rider is pulled toward (3333 for every airborne rider in the headless dump): from the surface table (`SurfPropCache` at `GameModule+0x44`, record +0x438 × 0xb0), slope, stats; capped to +0x30c while boosting. Equals \|vel\| at terminal speed (2207 live), 3234 vs \|vel\| 2702 in the headless dump | `Rider_UpdateTargetSpeed` 0x11b3f8 | med |
| 0x2e8 / 0x2ec | f32 | | accumulators added through Rider vslots 13/14 (`0x10e770`/`0x10e7d0`, which also trigger audio) and decayed in 0x1200d0; probably trick/aggression feedback | | low |
| 0x2f0 | f32 | `uberTimer` | 20 s-style timer set when the boost meter fills (`max(+0x2f0, 20)`), 60 in another path | `Rider_AddBoost` 0x10e098, 0x10e910 | med |
| 0x2f4 | i32 | `boostLevel` | number of meter fills (capped ~10); >10 lets boost exceed target speed | `Rider_AddBoost`, `Rider_UpdateTargetSpeed` | med |
| **0x2f8** | f32 | `boostMeter` | 0..1 | `Rider_AddBoost` clamps to [0,1]; HUD (`RiderMetrix_UpdateTrickHUD`) | **high** |
| 0x2fc | f32 | `boostActive` | strength of the active boost (0, 0.25, 0.625, 1.0 by meter level at activation); 0 = not boosting | 0x114130, `Rider_UpdateTargetSpeed` | med |
| 0x300 | f32 | `animTimeScale` | passed to the anim controller each frame (1.0) | 0x121700 → 0x11eb60 | med |
| 0x304 | i32 | `boostMode` | 3 = boost disabled, 0 = meter starts full, 1 default | `Rider_ResetBoostMeter` 0x120d58, ctor | med |
| 0x308 | i32 | | 1; Ground-state flag | ctor, `RiderGroundState_Update` | low |
| 0x30c | f32 | `boostMaxSpeed` | 4000.0 | ctor, `Rider_UpdateTargetSpeed` | med |
| 0x310 / 0x314 | f32 | | 0.18 / 0.015 (gp constants); HUD shows 0x314/0x310 | ctor, HUD | low |
| 0x318 / 0x31c | f32 | | IK blend weights for legs (0x318 → `0x11f3d8` with the thigh bones +0x8e0/+0x910) | 0x11eb98 | low |
| 0x320 / 0x324 | i32 | | compared (`!=` → "different") when scoring tricks; 0x324 = value from BE interface 2 per rider | `Rider_Reset`, trick code | low |
| 0x328 | i32 | `trickPhase` | 0..4, state of the current trick sequence | 0x106848, 0x115640 | low |
| 0x32c | i32 | | handplant phase (1/2/3) | `RiderHandplantSub_*` | low |
| 0x330 | i32 | | trick-related flag | | low |
| 0x340 | vec4 | | saved axis (copy of +0x1a0) during spins | 0x1163b0 | low |
| 0x350 | f32/i32 | | spin accumulator; also set to 1 by 0x1235f8 *through the owner* — overlapping meaning, unresolved | 0x1163b0 | low |
| 0x364 | u32 | `characterBit` | one-hot bit of the character (BE character id → 1,4,0x20,2,8,0x100,0x200,0x80,0x10,0x40) | `Rider_LoadStep` | high |
| **0x370** | vec4 | `groundNormal` | surface normal under the rider (unit, mostly +Z) | `Rider_Place` (from the ground ray), camera target slot 8 | **high** |
| 0x380 / 0x390 | vec4 | | previous ground normals (0x380 = last frame, 0x390 = smoothed/older) | frame diff shows 0x380(t+1) = 0x370(t) | med |
| 0x3a0 | vec4 | `groundForward` | forward projected onto the ground plane, normalised | `Rider_Place` | high |
| 0x3b0 | vec4 | `groundSide` | normal × groundForward | `Rider_Place` | high |
| 0x3c0 | vec4 | | written by air/trick code; camera target slot 16 returns `&+0x3c0` | | low |
| 0x3d0 | vec4 | | contact/push-out vector (zeroed on place) | 0x1242b0 | low |
| 0x3e0 | vec4 | `impulse` | collision impulse, decayed each frame | `0x1210b0` | med |
| 0x3f0 / 0x3f4 | f32 | `impactAccum` / `impactAccum2` | decayed; >4.5 (or >5 with conditions) forces a wipeout (`0x116120(r,0,2)`) | `0x1210b0` | med |
| 0x3fc | i32 | | per-frame flag, cleared each frame | `0x120f20` | low |
| 0x400 / 0x410 | vec4 | `boundsMin` / `boundsMax` | AABB of the rider (swept), fed to the OverlapList (+0x860) | 0x11e150, `Rider_Translate` moves both | med |
| 0x420 | vec4 | `boundSphere` | centre = pos, w = radius (250.0) | 0x11e150 | med |
| 0x430 | i32 | `groundPolyId` | id of the ground surface/section under the rider (-1 none), live 0x28308 / 0x14608 | 0x1242b0, `RiderAirState_PostUpdate` | med |
| 0x434 | i32 | `groundMaterial` | looked up from +0x430 via `GameModule+0x78` (`0x22e0e0`); 0x31 default; 0xb..0xd special (checked by camera) | 0x1218d0 | med |
| 0x438 | i32 | `surfaceType` | index into SurfPropCache (19 × 0xb0 records at `GameModule+0x44`, see world.md) (friction, speed); 2, 3 and 0xd change camera behaviour | camera target slot 20, `RiderGroundState_Enter` | med |
| 0x450 | i32 | | -1 | ctor | low |
| 0x454 | f32 | | ground contact distance/time | 0x1242b0 | low |
| 0x460 | vec4 | `groundContactPoint` | hit point of the ground ray | 0x1242b0 (≈ pos live) | med |
| 0x470 | f32 | `stateTimer` | -1 on reset, then advances by gp constant each frame while ≥ 0 | `0x120f20`, `Rider_Reset` | low |
| 0x474..0x480 | i32 | | race bookkeeping (start frame = `AI+0x08` at reset; 0x47c = finish time slot `0xe10*(n+1)`; 0x480 = finished flag) | `Rider_Reset`, 0x125108, 0x125228 | low |
| 0x490 | vec4 | `aiPathNearest` | nearest point on the AI path | `Rider_UpdateAIPathProgress` 0x1125c0 | med |
| 0x4a0 / 0x4b0 | vec4 | `aiPathLookAhead` / previous | look-ahead point on the AI path (distance + 796 / variable) | 0x1125c0, `Rider_InitTrackPaths` | med |
| 0x4c0 / 0x4c4 | f32 | `aiPathDist` prev / cur | distance along the AI path (+33/frame at 2000 u/s) | frame diff | high |
| 0x4c8 | f32 | `aiPathLateral` | lateral distance from the AI path | 0x1125c0; AI compares to 500/540/600/1000 | med |
| 0x4cc | f32 | `aiPathHeading` | yaw (atan2) from nearest point to look-ahead | 0x1125c0 | med |
| 0x4d0 / 0x4d4 | f32 | `distToFinish` cur / prev | remaining distance on the race path (decreasing, 338,690 live). For all six riders in the headless dump `aiPathDist + distToFinish` ≈ 346,000 (the Snow Jam race length) | `Rider_InitTrackPaths`, `Rider_UpdateRaceProgress` | high |
| 0x4dc | `{i32 a, i32 b, ...}[12]` (16 bytes) | `overlaps` | volumes/triggers currently overlapped; +0x59c = count | `Rider_UpdateRaceProgress` diffing against 0x4a5d60 | med |
| 0x59c | i32 | `overlapCount` | | | med |
| 0x5a0..0x5ac | f32/i32 | | air trick timing (0x5a4 = air time × factor, 0x5a8 = going up, 0x5ac = rail flag); camera target slots 21/22/24 | 0x114298 | low |
| 0x5b0 | i32 | | ground/turn state flag | 0x122cf0 | low |
| 0x5b4 | f32 | `smoothedHeading` | low-passed yaw of the ground velocity | `0x121950` | med |
| 0x5b8 | i32[64] | `visitedIds` | list of ids (-1 = empty) tested by vslot 3 (`0x108c28`, compares with `arg+0x78`) — likely gates/triggers already passed | base ctor 0x140738, `0x120f20` clears [0] | med |
| 0x6b8 | | | padding before the vptr | | |
| **0x6c0** | ptr | vptr | `Rider` primary vtable (see below) | | high |
| 0x6d0 | ptr | vptr | scriptable-object base (`0x27f968`): vt 0x459b90 | ctor | high |
| 0x6dc | | `ScriptObj` | script-engine object (`ScriptObj_ctor`), vptr at +0x6e8, registered with `ScriptEngine` | 0x283298 | high |
| 0x700 | mat4 | `scriptMatrix` | identity at load | 0x27f968 | med |
| 0x750 / 0x758 / 0x75c | f32 | | board flex / edge values (0x750 = \|slew #3\|) | 0x1242b0, 0x13f178 | low |
| 0x764 | i32 | | 3 live | | low |
| 0x774 / 0x778 | i32 | | -1 | ctor | low |
| **0x77c** | `RiderBase*` | `owner` | Player / ComputerActorRider that contains this Rider | `RiderBase_ctor`, `Rider_GetMainState` | **high** |
| **0x780** | `RiderModel*` | `model` | 0x160 bytes, alloc "RiderModel" | `Rider_LoadStep` | **high** |
| 0x784 | `RiderAnim*` | `anim` | anim controller (alloc "RiderAnim" 0x70, vt at +0x58 = 0x458458) | `Rider_LoadStep` | high |
| 0x788 | `AirPredictor*` | `airPredictor` | 0xb0 bytes | `Rider_LoadStep` | high |
| 0x78c | ptr | `focusBox` | FocusMan box (0x10, alloc 0x457960) holding a copy of pos | `Rider_CreateFocusBox`, `Rider_UpdateFocusBox` | high |
| 0x790 | `RiderMetrix*` | `metrix` | stats/HUD (0x1cc) | `Rider_CreateRiderMetrix` | high |
| 0x794 | ptr[8] | | handles cleared by `EA_MemSet(+0x794,0,0x20)`, iterated by 0x1220d8 (pointers into 0xd5xxxx live) | | low |
| 0x7c0..0x85c | f32[40] | | morph / blend weights written as a block by 0x1220d8 | | low |
| 0x860 | `OverlapList*` | | alloc "OverlapList" 0xa14; updated with the AABB +0x400 | ctor, `0x120f20` | high |
| 0x864 | ptr | `lineHint` | "Line Intersect Hint" (0x10); saved in replay state | ctor, `RiderBase_SaveState` | high |
| 0x868 | ptr | `wallHint` | "Wall Intersect Hint" (0x10) | ctor | high |
| **0x86c** | i32 | `index` | rider slot (0..5); vslot 7 returns it | `AI_Create*`, `0x140b80` | **high** |
| 0x870 | i32 | `viewIndex` | player/view index (0 = P1), -1 CPU, -2 in some net modes | `AI_CreatePlayers` | high |
| 0x874 | i32 | `isHuman` | 1 for Player; vslot 8 returns it, vslot 9 its inverse | `Player_ctor` | high |
| 0x878 | i32 | `isActor` | 1 for scripted actor riders | `ComputerActorRider_ctor` | high |
| 0x87c | i32 | `isLocal` | 1; 0 for the remote rider in a network race | `AI_CreatePlayers` | med |
| 0x880 | i32 | `loadStep` | 0..7, 7 = loaded | `Rider_LoadStep`, `Rider_Reset` | high |
| 0x884 | i32 | `visible` | render enable | `AI_Render` | med |
| 0x888 | i32 | `renderSlot` | -1 until assigned in `AI_Render` | | med |
| 0x88c | `AttachedEmitterFX*` | | 0xa8 | ctor | high |
| 0x890 | `cBoltOnFXInterface*` | | 200 | ctor | high |
| 0x894 | ptr | `loadList` | "LoadList" (0x2948) during loading, freed after | `Rider_LoadStep` | high |
| 0x89c..0x918 | i32 | bone indices | `+0x89c hips`, `+0x8a0 board_rootg`, `+0x8a4 board_childg`, `+0x8a8 head`, `+0x8ac middlespine`, `+0x8b0 handleft`, `+0x8b4 forearmleft`, `+0x8b8 handright`, `+0x8bc forearmright`, `+0x8e0 thighright`, `+0x8e4 shinright`, `+0x8e8 footright`, `+0x910 thighleft`, `+0x914 shinleft`, `+0x918 footleft` | `Rider_LoadStep` (`0x310c48(model, set, "name")`) | high |
| 0x8c0 / 0x8c8 / 0x8d0 | u64 | bone masks | upper-body / spine / head bone sets (+ "morph" bits) | `Rider_LoadStep` (`0x310ce8`) | high |
| 0x8f0..0x9cc | | IK data | leg IK targets and rest offsets (two 0x30 blocks at +0x8f0/+0x920 for the feet, quats at +0x950.., +0x9a0 config) | `Rider_LoadStep`, 0x120378 | med |
| 0x9d0 | vec4 | `modelOffset` | translation applied to the model (zero normally) | `Rider_Translate`, `Rider_PostUpdateMainState` | med |
| 0x9e0..0xa9c | | shadow / attach matrices | 0x9e0 and 0xa60 = position + direction pairs written by 0x105398 | | low |
| 0xa30 | ptr | | object the rider is grinding/attached to (has its own vtable; else world) | 0x121818 | low |
| 0xaa0 | ptr | | collision sphere-tree / hull object ("RiderSphereTree"?), moved by `Rider_Translate` | | med |
| 0xaa4 | i32 | | per-frame flag | | low |
| 0xaa8 | `BoaderShadow*` | `shadow` | 0x1a0 | ctor | high |
| 0xaac / 0xab0 | f32 | | ground contact values (live 0.47, 0.005) | 0x1242b0 | low |
| 0xab4 | ptr | `racePath` | race (finish-distance) path | `Rider_InitTrackPaths` | med |
| 0xab8 | ptr | `aiPath` | AI line path | `Rider_InitTrackPaths` | med |
| 0xabc | ptr | | "AI path intersect info" (0x20) | ctor | high |
| 0xac0 | ptr | | "Track path intersect info" (0x20) | ctor | high |
| 0xac4 | i32 | `isPuppet` | when set the rider skips physics/anim (replay or script control) | many `if (+0xac4 == 0)` guards in `AI_Update` phases | med |
| 0xac8 / 0xae0 / 0xafc / 0xb04 / 0xb08..0xb14 | | | attach-to-object (scripted or moving platform): 0xafc = use board bone, 0xb08 4 ids | Rider vt+0x6d0 slots 4/5 (0x123640/0x123b48) | low |
| 0xb18 / 0xb1c | i32 | | wall-hit result / flag | 0x122278, 0x122448 | low |
| 0xb20 | i32 | | 1, 2 for players | `0x12b498` | low |
| 0xb24 | f32 | `boostMeterStart` | | `Rider_ResetBoostMeter` | med |
| 0xb28 | u32 | `boostSourceMask` | 0x1f; 2 for some modes | `Rider_AddBoost` | med |
| 0xb2c | u32 | | flags (bit 0 tested by AI) | | low |
| 0xb30 | i32 | `aiSkill` | CPU difficulty value | `0x12b498`, `0x12c028` | low |
| 0xb34 | i32 | `statsIndex` | index passed with the rider index to the BE stats interface (3) | `Rider_UpdateTargetSpeed`, `Rider_AddBoost` | med |

### Rider vtable (primary, at +0x6c0)

Plain `Rider`: 0x459c18. In a Player the slots are 0x4583a8 (same slots 2-15,
own 1 and 16-19); in a ComputerActorRider 0x458660 (live; ScriptObj vt at +0x6e8 = 0x458618). The RiderBase
mixins are built with construction vtables (0x4599e8, 0x459700, ...).

| slot | target | meaning | conf |
|---|---|---|---|
| 1 | `Rider_dtor` / `Player_dtor` | dtor | high |
| 2 | 0x140910 | `GetVelocity()` → `&+0x1e0` | high |
| 3 | 0x108c28 | `HasVisited(obj)` — searches `+0x5b8[64]` for `obj+0x78` | med |
| 4 | `Rider_AddVelocity` | add impulse (updates AirPredictor in Air) | high |
| 5 | 0x1408f0 | `GetPosition()` → `&+0x110` | high |
| 6 | 0x1406e0 | returns a constant vec (0x4ff1a0) | low |
| 7 | 0x140b80 | `GetIndex()` → +0x86c | high |
| 8 / 9 | 0x140bc0 / 0x140bc8 | `IsHuman()` / `IsComputer()` (+0x874) | high |
| 10 | `Rider_ResetAtMatrix` 0x123210 | respawn at a matrix (grid offset per index) | med |
| 11 | 0x140710 | no-op | |
| 12 | `Rider_PlaceAtMatrix` | | high |
| 13 / 14 | 0x10e770 / 0x10e7d0 | add to +0x2e8 / +0x2ec (+ audio) | low |
| 15 | 0x10e830 | add trick score (calls RiderMetrix 0x119448, `Rider_AddBoost`) | low |
| 16-19 | stubs (Rider) / 0x1278c0.. (Player) | Player: clear / add / set its +0xdfc/+0xe00 (rumble or score accumulators) | low |
| 20 | `__pure_virtual` | | |

Rider vt+0x6d0 (0x459b90, scriptable base): slots 1/2 register/unregister with
`ScriptEngine` (0x282fb8 / 0x283000), 3/4 (0x123e30 / 0x123da8) and 12/13
(0x123640 / 0x123b48) attach/detach the rider to a script object.

## RiderBase (0xdf0 non-virtual part)

`RiderBase_ctor` 0x10fcd8, `RiderBase_dtor` 0x142a50. RiderBase is assembled from
one mixin class per state and per FX; each mixin derives virtually from Rider and
keeps its own `Rider*` (`vbase`). States are entered by
`RiderBase_SetMainState` 0x1112b8 / `RiderBase_SetSubState` 0x111538, which call
Exit on the old and Enter on the new state object.

| off | field | value / meaning | conf |
|---|---|---|---|
| 0xde0 | i32 `mainState` | 0 Ground, 1 Air, 2 Wipeout, 3 Static, 4 Railslide, 5 Handplant (names table 0x459d70); 6 = none (ctor) | high |
| 0xde4 | i32 `subState` | 0 Cruise, 1 BoardPress, 2 Prewind, 3 Bump, 4 NaturalAir, 5 Spin, 6 Gate, 7 Railslide, 8 Wipeout, 9 Reset, 0xa FinishLine, 0xb Handplant, 0xc UberGrind, 0xd Void (table 0x459dc8); 0xe = none (ctor) | high |
| 0xde8 | vptr | RiderBase vt 0x458710 (Player 0x458338, CAR 0x4585f0): slot 1 `GetControlInput(out)` (pure in RiderBase), 2 `SaveState`, 3 `LoadState` | high |

Live: Player main 0 / sub 0 (Ground/Cruise), CPU rider 1 main 1 / sub 5 (Air/Spin).

### Main-state objects

| state | object at | vbase ptr at | Enter | Exit | Update | PostUpdate |
|---|---|---|---|---|---|---|
| 0 Ground | +0x000 (base ctor 0x13c650) | +0x018 | 0x13c7a8 | 0x13f410 | 0x13d818 | 0x13f178 |
| 1 Air | +0x020 | +0x024 | 0x1399e0 | 0x139a18 | 0x139a20 | 0x139c88 |
| 2 Wipeout | +0x030 (ctor 0x136980) | +0x070 | 0x136c40 | 0x136f28 | 0x136e98 | 0x136ee0 |
| 3 Static | +0x0a0 | +0x0a0 | — | — | 0x136958 | 0x136978 |
| 4 Railslide | +0x0b0 | +0x100 | 0x13ad20 | 0x13c5a0 | 0x13af28 | 0x13bfa8 |
| 5 Handplant | +0x110 (ctor 0x141480) | +0x1b0 | 0x138b48 | 0x139178 | 0x1391a8 | 0x139528 |

(All renamed `Rider<State>State_<Op>`.) Examples of state-local data: Ground +0x04/+0x08 =
friction pair from the surface table × `RiderModel+0x140`; Wipeout +0x44 =
"BoardSphereTree" (0x2b0) allocated on first entry, +0x50/+0x60 = start positions;
Handplant +0x30 = target position, +0x84 = duration (velocity = Δ/duration).

### Sub-state objects

| sub | object at | vbase at | Enter | Exit | HandleInput |
|---|---|---|---|---|---|
| 0 Cruise | +0x1c0 | +0x1c0 | 0x131608 | 0x131c30 | 0x131620 |
| 1 BoardPress | +0x1d0 | +0x1e4 | 0x12fc60 | 0x12fe98 | 0x12fc80 |
| 2 Prewind | +0x1f0 | +0x1f0 | 0x12e980 | 0x12e9b0 | 0x12e9b8 |
| 3 Bump | +0x200 | +0x200 | — | — | 0x12e778 |
| 4 NaturalAir | +0x210 | +0x224 | 0x12f620 | 0x12fb68 | 0x12f730 |
| 5 Spin | +0x230 | +0x288 | 0x133128 | 0x134cb0 | 0x133308 |
| 6 Gate | +0x290 | +0x2a4 | 0x12be20 | — | 0x12bf68 |
| 7 Railslide | +0x2b0 | +0x2b8 | 0x131d08 | 0x132048 | 0x131d30 |
| 8 Wipeout | +0x2c0 (ctor 0x142388) | +0x340 | 0x12ca30 | 0x12e690 | 0x12cb68 |
| 9 Reset | +0x350 | +0x358 | 0x12f230 | — | 0x12f398 |
| 0xa FinishLine | +0x360 | +0x364 | 0x12c5c8 | — | 0x12c678 |
| 0xb Handplant | +0x370 | +0x384 | 0x1328b0 | 0x132f98 | 0x132a30 |
| 0xc UberGrind | +0x390 | +0x398 | 0x136268 | — | 0x136508 |
| (0xd Void) | +0x3a0? | +0x3a0 | — | — | — |

(Renamed `Rider<Sub>Sub_<Op>`.) Gate live: +0x290 = 1, +0x294 = 2.7, +0x29c = 555.6.

### Rider FX mixins

Updated per frame by `AI_Update` (called with `owner + off`), reset by 0x111890,
updated in replays by 0x111948.

| off | class | ctor | update |
|---|---|---|---|
| 0x3b0 | cBoardWakeFX | 0x2dcb88 | `cBoardWakeFX_Update` |
| 0x470 | cBoardSparkFX | 0x2da390 | `cBoardSparkFX_Update` |
| 0x520 | cBoardTrailFX | 0x2e81e8 | `cBoardTrailFX_Update` |
| 0x610 | cSpeedBoostStreamerFX | 0x2e6320 | `cSpeedBoostStreamerFX_Update` |
| 0x9c0 | unnamed FX (0x110 bytes) | 0x2eaae0 | 0x2eadd0 |
| 0xad0 | cAirStreamerFX | 0x2ef3b0 | `cAirStreamerFX_Update` |
| 0xaf0 | cAngerFX | 0x2d48f0 | `cAngerFX_Update` |
| 0xb00 | unnamed FX (0x40) | 0x2e3680 | 0x2e39d8 |
| 0xb40 | cDynamicSprayFX | 0x2de4a8 | `cDynamicSprayFX_Update` |
| 0xc70 | unnamed FX (0xb0) | 0x2f0e88 | 0x2f1150 |
| 0xd20 | unnamed FX (0xc0; +0x10 = sound/emitter passed to 0x392d18) | 0x2f6238 | 0x2f6518 |

## Player (0x1950)

`Player_ctor` 0x125c70, dtor 0x125eb8. RiderBase vt 0x458338: 1 `Player_GetControlInput`
0x127998, 2 `Player_SaveState`, 3 `Player_LoadState`.

| off | type | meaning | evidence | conf |
|---|---|---|---|---|
| 0xdf0 | ptr | input binding object for this player's pad (`App+0xb0+4*pad`, live 0x53fcf8); read with `InputBinding_EvalBool/EvalFloat(binding, id)` | `AI_CreatePlayers`, `Player_GetControlInput` | high |
| 0xdf4 | ptr/i32 | per-pad value from `App2+0x2eec` (gp-0x850 object), live 0x5c0000; gates vslot 19 | `AI_CreatePlayers` | low |
| 0xdf8 | ptr | replay cache (`Player_AllocReplayCache`) — also read as a float (>50) by 0x100680, unresolved | `Player_ctor` | med |
| 0xdfc / 0xe00 | f32 | accumulators driven by Rider vslots 16-18 (rumble / feedback) | 0x1278c0.. | low |

The control-input record filled by `GetControlInput` is 8 bytes of bitfields whose
layout depends on the sub-state (buttons in bits 12-19, analog axes as 6-bit
values `×31`); see `input.md`.

## ComputerActorRider (0x1a90)

`ComputerActorRider_ctor` 0x108c80, dtor 0x108fa8, init 0x108e88. RiderBase vt
0x4585f0: 1 `ComputerActorRider_GetControlInput` 0x10a768, 2/3 Save/LoadState.
Own fields 0xdf0-0xf50 (AI brain): +0xdf4 = 1.0 (skill scale), +0xe0c = -1,
+0xe50/+0xe60 vec4 targets, +0xe70 = index of the rider being targeted (-1)
used as `Rider+0x00[idx]` relation lookup, +0xe7c/+0xeb8/+0xef4 three 0x3c
blocks, +0xf38 u16 = 1, +0xf44 u64 constant. Low confidence beyond these.

## Owned sub-objects

### AirPredictor (0xb0, alloc "AirPredictor", ctor 0x113138)

Simulates the current jump forward to predict the landing. Primed by
`RiderAirState_Enter` and `Rider_AddVelocity` (in Air) through 0x1135b8(targetSpeed,
ap, &pos, &vel), which resets it (0x113198), stores +0xa8 and runs the simulation
(0x113618). Values below are from CPU rider 1, airborne in the paused PCSX2 race
(rider at (-144,300, 11,930, -235,000), vel (-1980, 18.8, 67.6)).

| off | type | meaning | live | conf |
|---|---|---|---|---|
| 0x00 | vec4 | predicted landing position | (-147,400, 11,950, -238,100) | med |
| 0x10 | vec4 | predicted velocity at landing | (-1208, 11.5, -3073) | med |
| 0x20 | vec4 | normal of the landing surface | (-0.68, 0.11, 0.73) | med |
| 0x30 | i32 | landing surface/poly id (same id space as `Rider+0x430`); -1 none | 0x14608 | med |
| 0x34 / 0x38 | f32 | | 0.278 / 0.278 | low |
| 0x3c | ptr | | | low |
| 0x40 | vec4 | position when the prediction was made (camera target slot 14) | ≈ pos | med |
| 0x50 / 0x60 | vec4 | end point / velocity of the last simulated step | | low |
| 0x70 / 0x80 | vec4 | rider position / velocity copy | = pos / vel | high |
| 0x94 | i32 | simulation step count | 0x49 | low |
| 0x98 | f32 | predicted total air time (s) | 2.52 | med |
| 0x9c | f32 | | 0.717 | low |
| 0xa0 / 0xa4 | f32 | air time elapsed (s) | 0.633 | med |
| 0xa8 | f32 | target speed passed in (`Rider+0x2e4`) | 3333 | high |
| 0xac | i32 | phase; 1 or 3 = landing predicted on track (camera slot 19) | 1 | med |

On the ground the first words are list links (live player: 0x5b1800, 0x5b1700, ...).

### RiderModel (0x160, alloc "RiderModel", ctor 0x30d4b8)

+0x00 bone count (13 live), +0x24 / +0x28 per-bone arrays (16 bytes per bone:
translation and rotation overrides), **+0x2c bone transforms** (0x20 per bone:
vec4 translation + quat), +0x140 vec4 colour/tint scale (0.85, 0.85, 0.85, 1),
+0x130 ptr. Bones are addressed with the Rider's bone indices (+0x89c...).
`RiderModel_Setup` 0x30dbd0, `RiderModel_LoadVariant` 0x30d8b8. Confidence med.

### RiderMetrix (0x1cc, alloc 0x457970, ctor 0x117248)

Stats and HUD messages for one rider. Live (player): +0x15c max speed 2209,
+0x160 current speed 2207 (`RiderMetrix_Update` keeps the max), +0x164 distance
travelled 756,000, +0x168 598 (frames in the current run?), +0x1a8 speed again,
+0x1ac `Rider*`, +0x1b0 HUD state object, +0x1b4 0x2c, +0x1c4 1.0. Combo/trick
score fields in +0x00..+0x150 (`RiderMetrix_AddComboPoints`, `_BankCombo`,
`_FinishTrick`; -1.0 = empty slots). Confidence med for the speed fields.

## Per-frame update (AI_Update 0x128af0)

For each phase, all riders are processed before the next phase:

1. `0x120f20` — reset per-frame flags, update the OverlapList, ground probe, AI-path
   intersection (every 3rd frame, staggered by rider index), `Rider_UpdateTargetSpeed`.
2. `Rider_ProcessControlInput` 0x121068 — owner vslot 1 `GetControlInput`, then
   `RiderBase_HandleSubStateInput`.
3. `0x1210b0` — decay impulse/impact accumulators, forced wipeout on hard impacts,
   then 0x1200d0 / 0x120d90.
4. `Rider_UpdateSlewedParams` 0x1211f8.
5. `Rider_UpdateMainState` 0x1216e0 (physics lives in the state Update functions).
6. 0x121700 / 0x121728 — animation controller update / pose evaluation (skipped if `+0xac4`).
7. `Rider_PostUpdateMainState` 0x121750, 0x1217f8 (RiderModel update).
8. 0x121818 — attachment matrix, 0x125ad0, `Rider_UpdateRaceProgress`,
   `Rider_UpdateAIPathProgress`, 0x125228 (finish), `RiderMetrix_Update`,
   `Rider_UpdateFocusBox`.
9. 0x1218d0 — surface material lookup, ground FX/sound.
10. 0x121950 — audio, `smoothedHeading`; 0x122cf0.
11. RiderBase FX mixins (table above), then `Rider_UpdateAttachedFX` per rider.

Every 6th frame `AI_UpdateRiderRelations` 0x10f560 refreshes `Rider+0x00`.

## Open questions

- Exact meaning of each of the 19 slewed parameters at +0x1f0 (which is lean,
  crouch, edge, board flex ...). The slots used by tricks/camera are #3, #11, #16.
- 0x2e8/0x2ec/0x320/0x324/0x328..0x350: trick-system fields; needs a pass over
  0x105000-0x10f000 and 0x113000-0x116000 (trick/score code).
- The CPU AI block (CAR +0xdf0..+0xf50) and Rider +0x0d8..+0x100.
- Whether 0x160 is "previous" or "render" matrix (identical while riding straight;
  written by the skeleton update 0x11eb98).
- The unit of distance (cm is the guess) and the meaning of `Player+0xdf4/+0xdf8`.
- Four unnamed FX mixins (0x9c0, 0xb00, 0xc70, 0xd20).

## Renames applied in Ghidra (2026-10-15)

78 functions, all previously `FUN_*`. Confidence as in the table.

| address | new name | conf | evidence |
|---|---|---|---|
| 0x111830 | RiderBase_GetMainStateName | high | name table 0x459d70 indexed by +0xde0 |
| 0x111860 | RiderBase_GetSubStateName | high | name table 0x459dc8 indexed by +0xde4 |
| 0x11fea8 | Rider_GetMainStateName | high | wrapper via +0x77c |
| 0x11fef8 | Rider_GetSubStateName | high | wrapper via +0x77c |
| 0x111408 | RiderBase_UpdateMainState | high | switch on +0xde0, per-frame |
| 0x1114a0 | RiderBase_PostUpdateMainState | med | second per-frame switch on +0xde0 |
| 0x111728 | RiderBase_HandleSubStateInput | high | switch on +0xde4 with the control-input record |
| (54 functions) | Rider{Ground,Air,Wipeout,Static,Railslide,Handplant}State_{Enter,Exit,Update,PostUpdate}, Rider{Cruise,BoardPress,Prewind,Bump,NaturalAir,Spin,Gate,Railslide,Wipeout,Reset,FinishLine,Handplant,UberGrind}Sub_{Enter,Exit,HandleInput} | high (PostUpdate med) | addresses in the two state tables above; each is the target of exactly one case of the Enter/Exit/Update dispatchers |
| 0x121068 | Rider_ProcessControlInput | high | AI_Update phase 2 |
| 0x1211f8 | Rider_UpdateSlewedParams | high | 19 slews |
| 0x1216e0 | Rider_UpdateMainState | high | wrapper |
| 0x121750 | Rider_PostUpdateMainState | med | wrapper + model offset |
| 0x120e88 | Rider_UpdateAttachedFX | high | +0x88c/+0x890 |
| 0x120e30 | Rider_UpdateFocusBox | high | copies +0x110 to +0x78c |
| 0x11e098 | Rider_BuildWorldMatrix | high | quat → +0x1a0..+0x1d0 |
| 0x11ff48 | Rider_GetHipsPosition | high | bone +0x89c ("hips") |
| 0x112180 | Rider_InitTrackPaths | med | +0xab4/+0xab8, distances |
| 0x1125c0 | Rider_UpdateAIPathProgress | med | +0x490..+0x4cc |
| 0x112338 | Rider_UpdateRaceProgress | med | +0x4d0/+0x4d4, overlaps |
| 0x11b3f8 | Rider_UpdateTargetSpeed | med | +0x2e4 |
| 0x10e098 | Rider_AddBoost | med | +0x2f8 clamp, +0x2f4 |
| 0x120d58 | Rider_ResetBoostMeter | med | +0x2f8 = +0xb24 |
| 0x10f560 | AI_UpdateRiderRelations | med | pairwise records every 6th frame |
| 0x123210 | Rider_ResetAtMatrix | med | Rider vslot 10 |

Decompiler comments were added at `Rider_ctor` and `RiderBase_ctor` summarising
the layout.
