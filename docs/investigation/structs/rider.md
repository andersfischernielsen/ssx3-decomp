# Rider structures (Rider, RiderBase, Player, ComputerActorRider)

_Mapped 2026-10-15 from Ghidra (`SLUS_207.72`), a paused PCSX2 Snow Jam race (PINE)
and two RAM dumps of our own headless runner one frame apart (frames 5000/5001
of the scripted race in `plans/playable-push.md`)._ Round 2 (same day) added the
trick system, slews, boost/target speed, the four unnamed FX mixins, RiderMetrix and
the CPU AI block, using further dumps at frames 5002/5003/5030/5040/5060/5120/5240 and
PINE reads of a CPU rider mid-trick.

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
  Typical race speeds are 2000-3300 units/s. **The unit is the centimetre**
  (confirmed in round 2): the target-speed table at 0x4a6310 holds km/h values
  (70..120) that `Rider_UpdateTargetSpeed` multiplies by 27.78 = 100000/3600, the
  airborne target 3333.3 is 120 km/h, and the camera constants use the same
  conversion (camera.md).
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
| 0x0d8 | i32 | | -1 on reset; no in-race reader found (the record array ends at +0xd8, so this is the first field after it) | `Rider_Reset` | low |
| 0x0dc | f32 | `rubberBandBehind` | negative distance window (cm): when this rider is further than this behind the human rider (Δ = human.distToFinish − own), catch-up mode scales speed up to ×1.1. Per-slot values come from `0x10c4f8` (−49,670 .. −92,500 by CPU slot), but `AI_UpdateRiderRelations` rewrites ±50,000 (gp-0x7cac/-0x7ca8) every 6th frame, so live it is always −50,000 | `0x10def0`, `0x10d938`, `AI_UpdateRiderRelations`; live −50000 in PCSX2 and headless, all riders | high (value) / med (meaning) |
| 0x0e0 | f32 | `rubberBandAhead` | positive window (cm): when ahead of the human by more than this, slow-down mode scales speed by 0.909/(1+(Δ−w)/w) | same; live 50,000 | high (value) / med |
| 0x0e4 | i32 | `rubberBandMode` | bit 0 = slow down when far ahead, bit 1 = catch up when far behind (0 none, 1, 2, 3). `0x10c4f8`: CPU slot 1 → 0, slots 2-5 → 1; a debug/mode flag (`0x5308d0` bit 2) forces 2. Only applied while `distToFinish` ≥ 50,000 and race type != 2 | `0x10def0`; live R0/R1 0, R2-R5 1 | high |
| 0x0e8 | i32 | `prevRacePosition` | copy of +0xec taken by `0x10e558` (trick/HUD event), -1 after `AI_ResetRiderRelations` | `0x10e558`, `0x10f3b8`; live -1 | med |
| **0x0ec** | i32 | `racePosition` | 0-based place, recomputed every 6th frame by `0x10f998` when `AI+0x74` != 0: mode 1 sorts by `distToFinish + 20·oldPos` (actors last), mode 2 by RiderMetrix+0x198 (score); a change fires a HUD event (`0x299e28`) | f5000: dists 338686/339715/337778/338772/342428/338076 → positions 2/4/0/3/5/1 | **high** |
| 0x0f0 | i32 | `hasRival` | 1 if `BEAggressionInt` level (`0x155b50(me, other)`) toward some rider is 4 | `0x10f878` (every 6th frame), `0x10d9e8` | med |
| 0x0f4 | i32 | `lastShoveFrame` | `AI+0x08` frame of the last CPU shove/punch; shoves are spaced ≥120 frames | `0x10dbf0`; live 540 on one CPU | med |
| 0x0f8 | i32 | `rivalIndex` | rider index with aggression level 4 (-1 none) | `0x10f878`, `Rider_Reset`, `0x10d410` | med |
| 0x0fc | i32 | `lastAggression` | aggression level toward the last rider examined in `0x10f878` (effectively the highest index) | `0x10f878`; live 0/3 | low |
| 0x100 | i32 | | 1 after reset; no AI reader found | `Rider_Reset` | low |
| **0x110** | vec4 | `pos` | world position (w = 1) | `Rider_Translate`, `Rider_Place`, vslot 5 `GetPosition` returns `&+0x110`; Δ/frame = vel/60 | **high** |
| **0x120** | quat | `rot` | orientation quaternion (unit) | `Rider_Place` (yaw from heading), normalised in `Rider_BuildWorldMatrix`; camera target slot 2 | **high** |
| 0x130 | vec4 | `rootPoseOverridePos` | root-bone position used when +0x150 != 0 | skeleton update 0x11eb98 | med |
| 0x140 | quat | `rootPoseOverrideRot` | root-bone rotation used when +0x150 != 0 | 0x11eb98 | med |
| 0x150 | i32 | `useRootPoseOverride` | | `Rider_Place` clears, 0x11eb98 | med |
| 0x160 | mat4 | `renderMatrix` | model matrix after pose/lean adjustments (equals +0x1a0 when upright) | written by 0x11eb98, read by FX/collision | med |
| **0x1a0** | mat4 | `world` | rotation rows x / **y = forward** / z = up, +0x1d0 = translation | `Rider_BuildWorldMatrix` 0x11e098; camera target slots 4/5/6 return +0x1b0/+0x1a0/+0x1c0 | **high** |
| **0x1e0** | vec4 | `vel` | velocity, units/s | `Rider_AddVelocity`, vslot 2 `GetVelocity` returns `&+0x1e0`; frame diff | **high** |
| 0x1f0 | `Slew[19]` (12 bytes: `cur, maxStep, target`) | `slewed` | 19 scalars moved toward `target` by at most `maxStep` per frame. Per-slot meanings in "Slewed parameters" below: #0 steer, #1 body lean, #3 brake, #4 crouch, #5 rail balance, #13/#14 air pitch/roll adjust, #15/#16 prewind spin/flip | `Rider_UpdateSlewedParams` 0x1211f8; writers listed in the sub-table | high (structure) / per slot in the sub-table |
| 0x2dc | f32 | `spinRate` | current yaw-spin rate in rad/s, set each frame by the Spin sub-state from the spin object's base rate × the BE "spin" stat factor; ×1.x while the trick power-up (+0x2ec) runs; 0 when the spin target is reached. Live CPU rider 1 mid-trick: 7.224 | `RiderSpinSub_HandleInput` (writes, then `spin.cur += clamp(target-cur, ±rate/60)`), PINE | high |
| 0x2e0 | f32 | `flipRate` | same for the flip (pitch) axis; live 4.816 | same | high |
| **0x2e4** | f32 | `targetSpeed` | speed the rider is pulled toward, cm/s. Ground: bilinear lookup in the speed table at 0x4a6310 (km/h; see "Target speed and boost") × 27.78 (km/h → cm/s), ×0.98 when riding switch and not boosting, then low-passed (0.90 when slowing, 0.97 when speeding up). Air: constant 3333.3 (120 km/h, gp-0x79d0). With `uberLevel` > 10 and boosting it is raised to at least `boostMaxSpeed` | `Rider_UpdateTargetSpeed` 0x11b3f8; frame diffs 5000..5003 (player in Prewind: 3235 → 3212 → 3190 → 3168, the 0.97/0.90 low-pass converging) | high |
| 0x2e8 | f32 | `powerUpBoostTimer` | seconds of "free boost": while > 0, holding boost does not drain the meter. Decays by `timeScale/60` per frame; cleared by ResetPath. Added through Rider vslot 13 (0x10e770) from the script command at 0x2ff850 (`FUN_0010f1c0` type 1); starts the 0x70 sound cue | 0x1200d0, 0x114130, 0x116120 | med |
| 0x2ec | f32 | `powerUpTrickTimer` | seconds of the trick power-up: spin/flip rates ×k, anim playback faster, and the rider FX at RiderBase+0x9c0 draws its board ribbon while > 0. Decays only on the ground (or when > 1/60); zeroed when a trick lands with boost earned. Added through vslot 14 (0x10e7d0, type 2, cue 0x71) | 0x1200d0, `RiderSpinSub_HandleInput`, 0x10e910, 0x2eadd0 | med |
| 0x2f0 | f32 | `uberTimer` | seconds of "uber" (full-meter) time: set to max(+0x2f0, 20) when the meter fills, 60 when `uberLevel` reaches 10. Held while airborne or in UberGrind, decays on the ground; reaching 0 shows HUD message 6. At level 10 the meter is pinned to 1.0 and, when the timer runs out, the level drops to 5 with 20 s | `Rider_AddBoost`, 0x10e910, 0x1200d0 | med |
| 0x2f4 | i32 | `uberLevel` | number of uber tricks landed in this run (0..10): 0x10e910 adds the increase of `RiderMetrix+0x114` (cumulative uber count), capped at 10; a full meter raises it from 0 to 1. ≥5 selects the "super uber" animation variants (FUN_00150198 with `level > 4`), ≥6 counts super ubers (`RiderMetrix+0x58`), 10 = unlimited-boost mode, 11 = cheat mode (set by the RiderBase+0xd20 code detector). >9 stops the meter draining | `Rider_AddBoost`, 0x10e910, 0x1197d8, 0x2f6518 | med |
| **0x2f8** | f32 | `boostMeter` | 0..1. Filled by `Rider_AddBoost` (trick boost from `RiderMetrix`), drained by 0.00150680 per frame while boosting (≈ 11 s from full), leaks passively by 0.005/s in boostMode 1 (frames 5030→5120: 0.4154→0.4078 = 8.4e-5 per frame, measured) or 0.024/s in mode 2, pinned to 1 in mode 0 | `Rider_AddBoost` 0x10e098, 0x114130, 0x1200d0; dump diffs | **high** |
| 0x2fc | f32 | `boostStrength` | strength of the active boost, latched on the press: meter > 2/3 → 1.0, > 1/3 → 0.625, else 0.25; 0 = not boosting. Picks the boost column of the speed table (<0.1 / <0.4 / <0.8 / ≥0.8 → tier 0..3) | 0x114130 (`Rider_UpdateBoost`), `Rider_UpdateTargetSpeed` | high |
| 0x300 | f32 | `timeScale` | rubber-band time/speed scale: slewed toward 0.909..1.1 at 0.05/s by 0x120090 (CPU catch-up), read by every state Update and passed to the anim controller each frame (1.0 for the player); also the time step of the timers above (`dt = scale/60`) | 0x121700 → 0x11eb60, 0x1200d0, 0x120090 (AI fork, round 2) | med |
| 0x304 | i32 | `boostMode` | 0 = meter pinned full ("unlimited boost"), 1 = normal (slow leak), 2 = fast leak (0.024/s), 3 = boost disabled (`Rider_AddBoost` returns). Probably the event options `kT_OVRCMNUnlimBoost` / `kT_11NoBoost` / `kT_11StartEventFullBoost` | `Rider_ResetBoostMeter` 0x120d58, ctor, 0x1200d0 | med |
| 0x308 | i32 | | 1; Ground-state flag; the cheat detector sets it from its option bit 1 | ctor, `RiderGroundState_Update`, 0x2f6518 | low |
| 0x30c | f32 | `boostMaxSpeed` | 4000.0 cm/s (144 km/h) | ctor, `Rider_UpdateTargetSpeed`, 0x2f6518 | med |
| 0x310 / 0x314 | f32 | `cheatSteerAlign` / `cheatSpeedLerp` | 0.18 / 0.015; used only in cheat mode (`uberLevel` 11, boosting): speed is lerped toward `boostMaxSpeed` by 0x314 and the velocity direction toward `groundForward` by 0x310 | 0x2f6518 | med |
| 0x318 / 0x31c | f32 | | IK blend weights for legs (0x318 → `0x11f3d8` with the thigh bones +0x8e0/+0x910) | 0x11eb98 | low |
| **0x320** | i32 | `stance` | current stance (0/1). Flipped by an odd number of half spins: CPU rider 1 lands a 540 between frames 5003 and 5030 and +0x320 goes 1 → 0. `stance != naturalStance` = riding switch | dump diff, `RiderMetrix_FinishTrick(…, switch)`, Cruise/Spin anim selection, 0x1163b0 | high |
| 0x324 | i32 | `naturalStance` | the character's natural stance (regular/goofy) from BE interface 2; constant per rider | `Rider_Reset`, trick code | med |
| 0x328 | i32 | `railStyle` | how the board meets the rail, set when a rail is caught: 0 none (cleared on jump), 1 straight (board along the rail), 2 straight but reversed, 3 / 4 sideways (boardslide, board turned 90°; HUD and anim use the rotated root). Sub-state 1 (BoardPress) forces 1 or 2 by stance | `Rider_TryCatchRail` 0x106848 (angle between board bone axis and rail direction), `RiderPrewindSub_HandleInput`, 0x1161d0 | med |
| 0x32c | i32 | | handplant phase (1/2/3) | `RiderHandplantSub_*` | low |
| 0x330 | i32 | `pressType` | board press: 0 none, 1 nose (anims 0x18 / rail 0x1a), 2 tail (0x20 / rail 0x22). Set from the BoardPress axis (`FUN_001161d0`) or the Spin sub-state's tri-state field; when set, catching a rail enters sub-state BoardPress instead of Railslide | 0x1161d0, `RiderSpinSub_HandleInput`, 0x106848 | high |
| 0x340 | vec4 | `attackDir` | side axis of the rider (+0x1a0 row, negated for a right-hand attack) saved when an attack starts | `Rider_UpdateAttack` 0x1163b0 (AttackLeft/AttackRight bits 18/19) | med |
| 0x350 | f32 | `attackBlend` | 0 → 1 blend weight of the attack (punch) animation on the base layer; reset to 0 at the start of an attack. The FX at RiderBase+0xc70 is active while > 0. (The "set to 1 through the owner" write seen in round 1 is to `RiderBase+0x350`, the Reset sub-state object, not to this field) | 0x1163b0, 0x2f1150, 0x116120 | med |
| 0x354 / 0x358 | i32 | `hudMsgExpire` / `hudMsgId` | rider HUD message: expiry frame (`AI` frame + seconds×60, -1 = none) and id (2 = landed trick, 3 = uber landed, 5 = meter full, 6 = uber time over) | `FUN_0010e028` (proposed `Rider_SetHudMessage`) | med |
| 0x35c | f32 | | 1/60 after a landing, 0 in the air | dumps 5003→5030 | low |
| 0x360 | f32 | `jumpRearm` | 1.0 while the jump button is up; a new prewind needs it set (prevents auto-repeat) | `FUN_001162c8` (proposed `Rider_TryPrewind`) | med |
| 0x364 | u32 | `characterBit` | one-hot bit of the character (BE character id → 1,4,0x20,2,8,0x100,0x200,0x80,0x10,0x40) | `Rider_LoadStep` | high |
| **0x370** | vec4 | `groundNormal` | surface normal under the rider (unit, mostly +Z) | `Rider_Place` (from the ground ray), camera target slot 8 | **high** |
| 0x380 / 0x390 | vec4 | | previous ground normals (0x380 = last frame, 0x390 = smoothed/older) | frame diff shows 0x380(t+1) = 0x370(t) | med |
| 0x3a0 | vec4 | `groundForward` | forward projected onto the ground plane, normalised | `Rider_Place` | high |
| 0x3b0 | vec4 | `groundSide` | normal × groundForward | `Rider_Place` | high |
| 0x3c0 | vec4 | `jumpDir` | horizontal take-off direction: xy of the ground normal at the jump (normalised; default vector if flat). Camera target slot 16 returns `&+0x3c0` | `Rider_Jump` 0x114298 | med |
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
| 0x470 | f32 | `finishTimer` | -1 while racing; ≥ 0 after the finish line (advances each frame). Any sub-state handler sends the rider to FinishLine (sub 10) once it is ≥ 0 | `FUN_00116378` (proposed `Rider_CheckFinished`), `0x120f20`, `Rider_Reset`; speech trigger 0x2a3b18 tests `< 0` | med |
| 0x474..0x480 | i32 | | race bookkeeping (start frame = `AI+0x08` at reset; 0x47c = finish time slot `0xe10*(n+1)`; 0x480 = finished flag) | `Rider_Reset`, 0x125108, 0x125228 | low |
| 0x490 | vec4 | `aiPathNearest` | nearest point on the AI path | `Rider_UpdateAIPathProgress` 0x1125c0 | med |
| 0x4a0 / 0x4b0 | vec4 | `aiPathLookAhead` / previous | look-ahead point on the AI path (distance + 796 / variable) | 0x1125c0, `Rider_InitTrackPaths` | med |
| 0x4c0 / 0x4c4 | f32 | `aiPathDist` prev / cur | distance along the AI path (+33/frame at 2000 u/s) | frame diff | high |
| 0x4c8 | f32 | `aiPathLateral` | lateral distance from the AI path | 0x1125c0; AI compares to 500/540/600/1000 | med |
| 0x4cc | f32 | `aiPathHeading` | yaw (atan2) from nearest point to look-ahead | 0x1125c0 | med |
| 0x4d0 / 0x4d4 | f32 | `distToFinish` cur / prev | remaining distance on the race path (decreasing, 338,690 live). For all six riders in the headless dump `aiPathDist + distToFinish` ≈ 346,000 (the Snow Jam race length) | `Rider_InitTrackPaths`, `Rider_UpdateRaceProgress` | high |
| 0x4dc | `{i32 a, i32 b, ...}[12]` (16 bytes) | `overlaps` | volumes/triggers currently overlapped; +0x59c = count | `Rider_UpdateRaceProgress` diffing against 0x4a5d60 | med |
| 0x59c | i32 | `overlapCount` | | | med |
| 0x5a0..0x5ac | f32/i32 | | take-off record: 0x5a0 cleared on the ground, 0x5a4 = take-off "pop" (velocity along the ground normal × the horizontal speed factor), 0x5a8 = 1 for an ollie (button jump), 0 for natural air off a lip, 0x5ac rail flag; camera target slots 21/22/24 | `Rider_Jump` 0x114298, `Rider_UpdateTargetSpeed` | med |
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
| 0xb2c | u32 | `uberFlags` | bit 0 = uber tricks allowed: CPU trick choice only tries uber tricks when set (`0x10c258`, `0x10cd20`); `0x125958(r, v)` stores v and, when 0, clears `uberTimer` +0x2f0 and `boostLevel` +0x2f4. Set per player from the rules table `0x5308b8[pad]` bit 4 / `0x5308d0` bit 1 (`0x12b498`) | live 3 on all riders | med |
| 0xb30 | i32 | `startHandicap` | players only: `max(rule values 0x5308c0/0x5308d4/0x5308cc) − own value` (`0x14f7a8`); `0x12c028` returns true once `AI+0x0c (race ticks) × k ≥ +0xb30`, used by the gate push-off code `0x12c230`. Not a CPU skill value (the CPU skill lives in ComputerActorRider +0xdf8/+0xdfc) | `0x12b498`, `0x12c028`; live 0 | med |
| 0xb34 | i32 | `statsIndex` | index passed with the rider index to the BE stats interface (3) | `Rider_UpdateTargetSpeed`, `Rider_AddBoost` | med |

### Slewed parameters (+0x1f0, 19 × `{cur, maxStep, target}`)

Writers were found by scanning for stores to each `target` offset and reading the
writers; the sub-state handlers set `target` (and usually `maxStep`) from the
control word, `Rider_UpdateSlewedParams` moves `cur`.

| # | cur at | meaning | writers of `target` | readers / evidence | conf |
|---:|---|---|---|---|---|
| 0 | 0x1f0 | `steer`: CruiseTurn axis (-1..1), scaled down below a reference speed; maxStep smaller on surface types 2, 3, 0xd | `FUN_00113e80` (proposed `Rider_SetSteerTarget`) from Cruise, Prewind, NaturalAir | Cruise anim choice (\|#0\|), #1 target | high |
| 1 | 0x1fc | `bodyLean`: lagged copy of #0 (target = #0.cur) | Cruise, Bump, FinishLine, NaturalAir handlers | Cruise carve anims 0x10/0x11 when \|#1\| large | med |
| 2 | 0x208 | ground-state value | `RiderGroundState_Update`/`_Exit`, `Rider_LoadStep` | | low |
| 3 | 0x214 | `brake`: CruiseBrake axis, signed by lean side (powerslide); 0 = not braking | `FUN_00113f88` (proposed `Rider_SetCrouchBrakeTargets`) | Cruise brake anims 0xb/0xc/0xd; `Rider+0x750 = \|#3\|` | high |
| 4 | 0x220 | `crouch`: 0..1 tuck (CruiseCrouch; 1.0 in Prewind) | `FUN_00113f88` | `Rider_UpdateTargetSpeed` interpolates the speed table with it (70 → 75 km/h etc.); `Rider_Jump(#4, r)` uses it as ollie strength | high |
| 5 | 0x22c | `railBalance` | `FUN_00113f38` (Prewind on a rail), `RiderRailslideState_PostUpdate` | | med |
| 6 | 0x238 | rail / grind lean | `RiderRailslideSub_HandleInput`/`_Exit`, `RiderUberGrindSub_HandleInput`, 0x132060 | | low |
| 7 | 0x244 | `handplantBalance` | `RiderHandplantSub_HandleInput`/`_Exit` | | med |
| 8 | 0x250 | | `RiderGroundState_Update`, `RiderHandplantSub_HandleInput`, `RiderRailslideState_Enter` | | low |
| 9 | 0x25c | rail-state value | `RiderRailslideState_Enter`/`_Exit` | | low |
| 10 | 0x268 | `boardPress` amount | BoardPress helpers 0x130228/0x1303e0/0x1304e0, `RiderBoardPressSub_Exit` | | med |
| 11 | 0x274 | `boardPivot` | BoardPress helpers 0x1308d8/0x130dd0/0x131200, `RiderBoardPressSub_Exit` | read by tricks/camera (round 1) | med |
| 12 | 0x280 | | (offset 0x288 is too common to scan) | | low |
| 13 | 0x28c | `airPitchAdjust`: spin object +0x2c / max (AirAdjRotFB, or the BoardPress tri-state) | `RiderSpinSub_HandleInput`/`_Exit` | Spin picks one of 8 "tweak" anims 0x129..0x130 from atan2(#13, #14) | med |
| 14 | 0x298 | `airRollAdjust`: spin object +0x28 / max (AirAdjRotLR) | `RiderSpinSub_HandleInput`/`_Exit` | same | med |
| 15 | 0x2a4 | `prewindSpin`: -1/0/1 from PrewindSpin | `RiderPrewindSub_HandleInput`, `RiderNaturalAirSub_Enter`, `RiderSpinSub_Exit` | passed to `FUN_001158b8` (seeds the spin target at take-off) | high |
| 16 | 0x2b0 | `prewindFlip`: -1/0/1 from PrewindFlip | same | same; read by tricks/camera | high |
| 17 | 0x2bc | ground-state value | `RiderGroundState_Update`/`_Exit` | | low |
| 18 | 0x2c8 | ground-state value | `RiderGroundState_Update`/`_Exit`, 0x124788, 0x12c230/0x12c408 (FinishLine) | | low |

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
| 13 / 14 | 0x10e770 / 0x10e7d0 | `AddPowerUpBoost(sec)` / `AddPowerUpTrick(sec)`: add to +0x2e8 / +0x2ec, sound cue 0x70 / 0x71, speech event | med |
| 15 | 0x10e830 | `AddTrickMultiplier(x)`: only in Air or Railslide; `RiderMetrix+0x130` count, `+0x18` = best multiplier (HUD 4), cue 0x75 (2/3/5/10 → speech variants) | med |

Slots 13-15 have exactly one caller, `FUN_0010f1c0(rider, type, value)` (jump table
0x456af0, types 1/2/3 → slots 13/14/15, 4..6 other effects), called only from the
script native at 0x2ff850 (entry in the native table at 0x441fa4). The power-ups are
therefore script/event driven (debug menu "PowerUpFXToggles"), not part of a normal race.
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

### Trick system (round 2)

Air tricks live in the **Spin sub-state object** (RiderBase+0x230, sub-state 5),
which `RiderPrewindSub_HandleInput` enters at take-off after seeding the spin/flip
targets from slews #15/#16 (`FUN_001158b8`). `RiderSpinSub_HandleInput` 0x133308 runs
every frame in the air: ResetPath check, handplant check, `Rider_TryCatchRail`
(0x106848), the grab state machine (`FUN_001352a8`), boost (`Rider_UpdateBoost`
0x114130), then the spin/flip integrators, the scoring feed and the anim choice.

Spin object layout (offsets from RiderBase+0x230; live values from PCSX2 CPU rider 1
mid-trick and headless frames 5000/5040):

| off | type | name | meaning | live | conf |
|---|---|---|---|---|---|
| +0x00 | i32 | `inputMode` | 0 = stick drives spin/flip, 2 = Tweak held with a stick direction, 1 = Tweak released (latched), 3 = grab started; reset to 0 after 0.3 s without stick input | 3 / 0 | low |
| +0x04 | i32 | `grabPhase` | 0 idle, 1 grab start anim playing, 2 holding (points given), 3/4 uber variant playing, 5 releasing (back to 0 at the end of the anim) | 5 | med |
| +0x08 | i32 | `grabButton` | trick button index 0..14 (`Trick1..15`, control-word byte 2), -1 none | 1 / -1 | high |
| +0x0c | i32 | `spinPhase` | 0 accepting input, 1 adding half turns while the stick is held, 2 unwinding to the target, 3 done/idle | 0 / 3 | med |
| +0x10 / +0x14 | f32 | `flipTarget` / `spinTarget` | target rotation in rad, a multiple of π (one half turn per step, `fGp-0x74ec` = π) | -2π / -2π | high |
| +0x18 / +0x1c | f32 | `flipCur` / `spinCur` | current rotation; moves toward the target by `Rider+0x2e0` / `+0x2dc` × 1/60 per frame | -2.97 / -4.455 | high |
| +0x20 / +0x24 | f32 | `spinSum` / `flipSum` | running sums of the per-frame deltas (reset on Enter) | | med |
| +0x28 / +0x2c | f32 | `rollAdjust` / `pitchAdjust` | slewed air-adjust values, written to slews #14 / #13 as fractions of their maximum | | med |
| +0x30 / +0x34 | f32 | `spinTotal` / `flipTotal` | totals for scoring; negated when `naturalStance` ≠ 0 and pushed to `RiderMetrix+0x34/+0x38` each frame (`FUN_00119898` / `FUN_001198d8`) | 9.425 / -6.283 (= a 540 with a back flip) | high |
| +0x38 / +0x3c | f32 | `spinBaseRate` / `flipBaseRate` | rad/s; seeded 4.5 / 3.0 on take-off, ×0.8 per extra half turn (floor 3.0) | 6.91 / 4.607 | high |
| +0x40 | f32 | | decays ~3.4 %/frame in the air (0.299 → 0.270 over frames 5000..5003) | | low |
| +0x44 | i32 | `turnCommitted` | set when a half turn is added | | low |
| +0x48 / +0x4c | f32 | `spinHold` / `flipHold` | seconds the spin / flip stick has been held | 0.567 | med |
| +0x50 | f32 | `stickAngle` | atan2 of the stick (1e30 = centred) | -2.356 | med |
| +0x54 | f32 | `idleTime` | seconds without stick input | | med |
| +0x58 | `Rider*` | vbase | | | high |

Rates: the per-frame rate written to `Rider+0x2dc/+0x2e0` is the base rate × (1 +
0.5 × BE stat (interface 3, `FUN_001495a8`)), ×2 while the trick power-up runs;
the unwinding rate is capped by 2 × 1/(1 + 0.35 × stat).

**Grab state machine** (`FUN_001352a8`, proposed `RiderSpinSub_UpdateGrab`; returns
"grab anim playing" = layer-2 anim entry 0x12..0x14):

1. Trick button pressed (phase 0): play the start anim (`ButtonTrick.startAnim`) on
   anim layer 2, store the button in +0x08.
2. Anim event 0 → phase 1; event 1 → award the trick (`ButtonTrick.trickId` →
   `FUN_00119708` adds `points × 1e-4` to `RiderMetrix+0x14` and sets the hold rate),
   play the hold anim, phase 2.
3. Holding the same button while `uberTimer` > 0 and `Rider+0xb2c` bit 0 is set
   starts an uber variant (per-character table, see below), phase 3/4; the uber
   trick id (≥ 0x23) is awarded the same way.
4. Release → `FUN_001197d8` (proposed `RiderMetrix_EndGrab`: counts the trick by
   class, accumulates hold time), phase 5 → 0.

**Trick tables:**

| address | layout | meaning | conf |
|---|---|---|---|
| 0x45aeb8 | 15 × 0x14 `ButtonTrick {u16 startAnim, u16 holdAnim, u16 trickId, u16 endAnim, u16 uberHoldAnim, u16 pad, u16 uberStartAnim?, u16 uberCount, ButtonUber* ubers}` | one record per trick button; 0x1b6 = no anim. trickIds 3..17 | med |
| `ubers` | 8-byte `{u16 anim, u16 ?, u16 trickId, u16 ?}` | uber variants of a button trick, chosen per character | med |
| 0x530ec0 | `u8 [profile][character 0x1fe][button 6]{normal, super}` | index into `ubers` for this rider (`FUN_0014fea8`); profile/character from `BE_GetRiderProfile`/`BE_GetRiderCharacter` | med |
| 0x530600 | `{i32 points, i32 holdRate}[trickId]` (RAM, loaded from BE data) | ids 0..18 grabs (200..350 pts), 19..34 second class (200 pts, hold 500), ≥ 35 uber tricks (1000..2500 pts, hold 3000) | high (layout) / med (classes) |

Classes as counted by `FUN_001197d8`: id < 0x13 → `RiderMetrix+0x4c`, < 0x23 →
`+0x50`, else uber → `+0x54` (and `+0x58` "super uber" when `uberLevel` ≥ 6). The CPU
rider 1 trick in the dumps (grab id 21) counted in `+0x50`.

**Scoring and boost interplay:**

- During the trick `RiderMetrix+0x14` (`trickBoost`) collects the boost value of the
  trick: grab points × 1e-4, hold time × holdRate × 1.67e-6/frame, and
  |spin| / |flip| × 1/(20π) per radian (`FUN_00119898` replaces the previous spin
  contribution each frame).
- Landing (any of `FUN_00119d40` from the ground states, 0x10e910 when the trick
  ends on a rail, `FUN_00119c98` for spin-only landings) calls
  `RiderMetrix_FinishTrick` (points shown, statistics) and returns the boost to add:
  `Rider_AddBoost(trickBoost, rider, 1)`. CPU rider 1: `trickBoost` 0.4155 at frame
  5003, meter 0.4154 at 5030 after landing a 540 back flip with a grab.
- `Rider_AddBoost` only accepts positive amounts whose source bit (1 = trick,
  4 = multiplier/script) is in `Rider+0xb28`; a full meter starts the 20 s uber timer.
- Riding switch (`stance != naturalStance`) is passed to FinishTrick as a flag and
  costs 2 % target speed.

### Target speed and boost (round 2)

`Rider_UpdateTargetSpeed` 0x11b3f8, per frame:

```
stat   = BE interface 3 FUN_001494c0(rider index, statsIndex)        # 0..1 speed stat
surf   = SurfProp[surfaceType].speed (+4)  (ground only, else 1.0)     # -1..1
plane  = surf > 0 ? 1 : 0 ; s = surf > 0 ? surf : surf + 1
tier   = boostStrength < 0.1 ? 0 : < 0.4 ? 1 : < 0.8 ? 2 : 3
c      = slew #4 crouch (ground) else 1.0
for v in {0,1}: T[v] = bilerp(table[v][plane..plane+1][tier][0..1], s, c)
target = lerp(T[0], T[1], stat) × 27.78            # km/h → cm/s
if switch and not boosting: target ×= 0.98
targetSpeed = lerp(old, target, target < old ? 0.90 : 0.97)   # ground
targetSpeed = 3333.3                                          # air
if uberLevel > 10 and boosting: targetSpeed = max(targetSpeed, boostMaxSpeed)
```

Speed table 0x4a6310: `float [stat 2][plane 3][tier 4][crouch 2]` in km/h:
stat-low plane 0 = 70/75, 75/80, 80/85, 90/95 per tier (upright/tucked), up to
115/120 (stat-high plane 2, full boost). Confidence high (constants read from RAM,
code read in full).

`Rider_UpdateBoost` 0x114130 (proposed; args `held`, `pressed`): on a press with
meter > 0 latch `boostStrength`, start the boost sound; while held drain the meter
unless `powerUpBoostTimer` > 0, `boostMode` = 0 or `uberLevel` > 9; on release clear
`boostStrength`. `FUN_001200d0` (proposed `Rider_UpdateBoostTimers`) runs the timers
and the passive leak each frame (phase 3 of `AI_Update`).

### Rider FX mixins

Updated per frame by `AI_Update` (called with `owner + off`), reset by 0x111890,
updated in replays by 0x111948.

| off | class | ctor | update |
|---|---|---|---|
| 0x3b0 | cBoardWakeFX | 0x2dcb88 | `cBoardWakeFX_Update` |
| 0x470 | cBoardSparkFX | 0x2da390 | `cBoardSparkFX_Update` |
| 0x520 | cBoardTrailFX | 0x2e81e8 | `cBoardTrailFX_Update` |
| 0x610 | cSpeedBoostStreamerFX | 0x2e6320 | `cSpeedBoostStreamerFX_Update` |
| 0x9c0 | proposed `cTrickPowerUpFX` (0x110 bytes, vt 0x488458): while `Rider+0x2ec` > 0 in the air, keeps a 3-entry history of board-bone (`+0x8a4` board_childg) segments for a ribbon and enables the emitter at RiderBase+0xd30 | 0x2eaae0 | 0x2eadd0 |
| 0xad0 | cAirStreamerFX | 0x2ef3b0 | `cAirStreamerFX_Update` |
| 0xaf0 | cAngerFX | 0x2d48f0 | `cAngerFX_Update` |
| 0xb00 | proposed `cPlayerPulseFX` (0x40, vt 0x4889c8): only for human riders (vslot 8); advances a phase and blends two RGBA colours (+0x08 / +0x18) by `cos` into +0x28 (a pulsing highlight colour) | 0x2e3680 | 0x2e39d8 |
| 0xb40 | cDynamicSprayFX | 0x2de4a8 | `cDynamicSprayFX_Update` |
| 0xc70 | proposed `cAttackFX` (0xb0, vt 0x4887d8): active while `Rider+0x350` (attack blend) > 0 and an attack anim (0x143/0x144/0x146/0x147) plays; spawns 4 jittered points at the striking hand bone (+0x8b0 / +0x8b8 by stance), sizes scaled by the blend | 0x2f0e88 | 0x2f1150 |
| 0xd20 | proposed `cCheatCodeDetector` (0xc0, vt 0x4886e0; +0x10 = emitter used by 0x9c0): human riders only. Records boost-press (bit 0x10, from `Rider_UpdateBoost`) / jump-press (0x20, from `Rider_TryPrewind`) / 0x40 events in byte +7; counts boost presses between jumps and compares them with the 9-entry table 0x445e30 (5,1,0,4,1,0,3,1,0); a full match sets +4 = 1, which forces `uberLevel` 11, meter 1, all timers 60 s, boostMode 0, and in that mode steers/accelerates the rider toward `boostMaxSpeed` | 0x2f6238 | 0x2f6518 |

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

`ComputerActorRider_ctor` 0x108c80, dtor 0x108fa8, init 0x108e88 (round 2: clears
the brain, then `0x10cab8` → `CAR_SetupTuning` 0x10c758). RiderBase vt 0x4585f0:
1 `ComputerActorRider_GetControlInput` 0x10a768, 2/3 Save/LoadState. The CAR's
Rider vtable 0x458660 adds slot 20 = `CAR_ScorePathBranch` 0x10d410.

**How the CPU drives (round 2).** The CPU fills the same 8-byte control-input
record as a pad (`input.md`). `GetControlInput` first runs the rubber band
(`0x10def0` → `0x120090` slews `Rider+0x300` toward 0.909..1.1 by 0.05/s; +0x300
is read by every state Update, so it is a rider time/speed scale, not only an
anim rate) and then switches on the sub-state:

| sub | handler | what it does |
|---|---|---|
| 0 Cruise | `0x10a960` | reset request (`0x10d1a0`), shoves (`0x10dbf0`/`0x10da10` → bits 0x40000/0x80000/0xc0000), then calls the current **behaviour** through the pointer-to-member at +0xf44 |
| 1 BoardPress | `0x10ad78` | steer to the AI line; sidestep markers |
| 2 Prewind | `0x10aa70` | tuck (bit 0x4000) while slower than +0xdf0; jump (0x2000) at a type-0x10 marker or at the end of the +0xe50→+0xe60 line |
| 4 NaturalAir | `0x10b590` | on take-off: pick spin/flip axes from the AirPredictor's remaining air time (>1 spin, >3 flip, random sign) |
| 5 Spin | `0x10b250` | choose and time the trick (below); after 30 frames of long predicted air, bit 0x1000 |
| 6 Gate | `0x10b790` | push off the start gate at `180 − AI+0x08` frames, earlier for lower CPU slots; random wobble in +0xdf0 every 20 frames |
| 7 Railslide | `0x10aed8` | balance on the rail, read the next marker, set up the dismount trick |
| 8 Wipeout | `0x10b750` | |

Behaviours (+0xf38 id, +0xf44 = `{i16 delta, i16 -1, fn}` copied from the table at
0x43cf20):

| id | function | when | does |
|---|---|---|---|
| 0 | `0x1009e0` | a type-0x10 trick marker is within 300 cm ahead on the AI path | follow the marker line at the marker speed; at the marker call `CAR_PlanAirTrick` and press 0xc000 |
| 1 | `0x100680` | default | follow the AI line: steer to `0x100610` (look-ahead +0x4b0 plus 0.x × (nearest − pos)); hold speed at a type-0x11 marker's speed (+0xdf0); sidestep at type-0x13 markers (60-frame pulse, +0xe40); use boost (0x20000) when meter ≥ 0.9, uber timer > 0 and +0xdf8 > 50; tuck via `0x10c0a8`; every 12 frames pick a nearby rider (`0x10bd10`, +0xe70) |
| 2 | `0x100f88` | picked rider is not the rival and not human | pace/block it: match its heading cone; if it is a CPU on the same path, `0x112a50` switches path |
| 3 | `0x100b90` | picked rider is the rival (`Rider+0xf8`) | chase: steer at a point 300 cm ahead of the rival along its velocity, tuck to catch up |

AI-path marker types (from the path query `aiPath+0x34` vslot 1): **0x10** trick/jump
marker (word 1 bits 0-3 = flip (→ +0xe3c), spin (+0xe38), no-late-trick (+0xe20), jump-line (+0xe1c); bits 4+ = speed in km/h),
**0x11** speed marker (km/h), **0x13** sidestep (left/right). Speeds are converted with
27.778 = 100 000/3600, which makes the distance unit the **centimetre** (cm/s).

| off | type | name | meaning | evidence | conf |
|---|---|---|---|---|---|
| 0xdf0 | f32 | `desiredSpeed` | speed to hold, cm/s, from the last marker (1916.7 = 69 km/h, 2138.9 = 77, 2500 = 90 live); reused as the random wobble value in the Gate sub-state | `0x10b980`, `0x10bb18`, `0x10aa70`, `0x100680`, `0x10b790` | high |
| 0xdf4 | f32 | `steerGain` | 1.0; when != 1 the steering output of `0x100250` is scaled by it | init, `0x100250` | med |
| 0xdf8 | f32 | `aggression` | 0..100 per CPU slot (100 / 80.4 / 51.0 / 46.6 / 14.0 for slots 1-5 at the live difficulty): > 50 uses boost, < 20 never tucks | `CAR_SetupTuning`/`0x10c4f8` table at 0x49b31c; `0x100680`, `0x10c0a8` | high (values) / med (use) |
| 0xdfc | f32 | `trickSkill` | 0..1 (0.869 / 0.796 / 0.689 / 0.568 / 0.424); probability gates: uber `p = s`, held/tweaked trick `0.3+0.7s`, basic trick `0.6+0.4s`; `1−s` = chance of a random (unfitted) trick. Tracks 2/3/4 add 10%/10%/25% (capped at 1) | `0x10c1d0`, `0x10c258`, `0x10c320`, `0x10c3b8`, `CAR_SetupTuning` | high |
| 0xe00 | i16 | `pathClass` | 0/1/2: which flagged AI-path branches this CPU may take (`0x10c450(slot, App+0xc0 setting)`; live 2,2,1,1,0); race types 4/5 force 0/2 | `0x10d870` (path-branch score) | med |
| 0xe04 | i32 | `allowExpertBranches` | slot 1 when the App+0xc0 setting is 3; lets `CAR_ScorePathBranch` take branches with flag bit 0 | `CAR_SetupTuning`, `0x10d410`; live 1 for slot 1 | med |
| 0xe08 | i32 | `randomBranches` | race types 1/5/6: add a random term to the branch score | `CAR_SetupTuning`, `0x10d410` | med |
| 0xe0c | i32 | `trickId` | 0..14 chosen trick (-1 none); copied into control byte 2 | `0x10b250`, `0x10cad8`, `0x10bfa8` | high |
| 0xe10 | i32 | `airTrickPlanned` | spin or flip planned for this jump | `0x10b590`, `0x10bfa8`, `0x10b0e8` | med |
| 0xe14 | i32 | `randomTrick` | 1 with probability `1−trickSkill`: the Spin handler picks `rand%15` instead of a trick that fits the air time | `0x10bfa8`, `0x10b250` | med |
| 0xe18 | i32 | `firstAirFrame` | 1 on take-off; first Spin frame stores the remaining air time in +0xe34 | `0x10b590`, `0x10b250` | med |
| 0xe1c | i32 | `useJumpLine` | marker flag: jump at the end of the +0xe50→+0xe60 line instead of at a marker | `0x10bfa8`, `0x10aa70` | med |
| 0xe20 | i32 | `noLateTrick` | marker flag: abort tricks when < 1 unit of air remains; sets the 0x21000 bits in word 1 | `0x10bfa8`, `0x10b250`, `0x10cad8` | low |
| 0xe24 | i32 | `uberTrick` | chosen trick is an uber (`0x10cf68`; needs `Rider+0x2f0` > 0); forces the 0x4000 modifier bit | `0x10cf68`, `0x10b250` | med |
| 0xe28 | i32 | `tweakTrick` | chosen trick is the held/tweaked variant (`0x10cd20`); also forces 0x4000 | `0x10cd20`, `0x10b250` | med |
| 0xe2c | f32 | `trickHoldTime` | anim length / anim speed of the chosen trick (0.26 live) | `0x10cbe0`.. | med |
| 0xe30 | f32 | `trickAirAtStart` | remaining air when the trick was chosen | `0x10cbe0`.. | med |
| 0xe34 | f32 | `trickSlack` | air left after the trick (15.0 = "unset"); cleared outside Spin | `0x10b250`, `GetControlInput` | med |
| 0xe38 | f32 | `spinAxis` | -1/0/+1 analog spin input | `0x10b590`, `0x10bfa8`, `0x10b0e8` | high |
| 0xe3c | f32 | `flipAxis` | -1/0/+1 analog flip input | same | high |
| 0xe40 | i32 | `sidestepTimer` | 60-frame steering pulse at a type-0x13 marker (-1 done) | `0x100680` | med |
| 0xe44..0xe4c | | | padding (zero) | | |
| 0xe50 / 0xe60 | vec4 | `jumpLineStart` / `jumpLineEnd` | world points of the current marker's line (from the AI path) | `0x10b980`; live ≈ 2000 cm ahead of the riders | high |
| 0xe70 | i32 | `targetRider` | rider index picked by `0x10bd10` (relation distance ≤ 700, own lateral < 540; dropped beyond 600 lateral or the gp limit) | `0x10bd10`, `0x100b90`, `0x100f88`; headless CPU slot 3 → 0 (the player) | high |
| 0xe74 | i32 | `stuckFrames` | frames in Wipeout or ≥ 1000 cm off the AI line; > 222 → request reset (bit 0x1000) | `0x10d1a0`; headless 9 → 39 → 99 while in Wipeout | high |
| 0xe78 | i32 | `wrongHeadingFrames` | frames with heading far from the AI-path heading; > 60 → reset | `0x10d1a0` | med |
| 0xe7c | i32[15] | `basicTrickUses` | per trick id, how often chosen; the chooser prefers the least used | `0x10cbe0` | high |
| 0xeb8 | i32[15] | `uberTrickUses` | same for uber tricks | `0x10cf68` | high |
| 0xef4 | i32[15] | `tweakTrickUses` | same for tweaked tricks; live [0,1,1,0...] | `0x10cd20` | high |
| 0xf30 | f32 | `shoveCooldown` | seconds (2.0) after seeing a rider in anim 0xd (shove); decremented by 1/60 | `0x10da10` | med |
| 0xf34 | i32 | `shoveBack` | 25% chance to shove back while the cooldown runs | `0x10da10` | med |
| 0xf38 | i16 | `behaviour` | 0 marker line, 1 race line, 2 pace, 3 chase (table above) | all four behaviours; live 0/1 | high |
| 0xf44 | ptmf | `behaviourFn` | `{i16 delta, i16 -1, fn}`: 0x1009e0 / 0x100680 / 0x100f88 / 0x100b90 (table 0x43cf20 + 8·id) | `0x10a960`; live `(0,-1,0x1009e0)`/`(0,-1,0x100680)` | high |

### CPU tuning (`CAR_SetupTuning` 0x10c758 / `0x10c4f8`)

Indexed by the Rider slot (vslot 7, 1..5); the second argument is the difficulty
from `BE interface 1` (`0x147cb8`). With difficulty 1 (live) the table at 0x49b31c gives
`(aggression, trickSkill×100, rubberBehind, rubberAhead)`: slot 1 (100, 86.9, −49,670,
47,500, mode 0), 2 (80.4, 79.6, −70,000, 45,000), 3 (51.0, 68.9, −81,571, 40,000),
4 (46.6, 56.8, −80,321, 37,500), 5 (14.0, 42.4, −92,500, 32,500); difficulties 0 and 2
use the tables at 0x49b368 / 0x49b3a0. The slot thus acts as the CPU's personality:
slot 1 is the fast, expert-line, no-rubber-band leader. The rubber-band windows are
overwritten with ±50,000 every 6 frames (see `Rider+0xdc`), so the per-slot window
values have no effect in-race (possibly a shipped bug).

### Proposed names (round 2)

Not renamed in Ghidra (renames are owned by another agent).

| address | proposed name | evidence | conf |
|---|---|---|---|
| 0x108e88 | `CAR_Init` | called last in the ctor; clears +0xdf0..+0xf44, runs tuning | high |
| 0x10c758 | `CAR_SetupTuning` | per-slot aggression/trickSkill/pathClass, track bonus | high |
| 0x10c4f8 | `CAR_ApplySlotTuningTable` | switch on slot × difficulty over 0x49b31c.. | high |
| 0x10c450 | `CAR_GetPathClass` | (slot, setting) → 0/1/2 | med |
| 0x10cab8 | `CAR_ResetTuning` | thunk to `CAR_SetupTuning` | high |
| 0x10def0 | `CAR_GetRubberBandScale` | ±window vs the human's distToFinish, 0.909/1.1 | high |
| 0x120090 | `Rider_SlewTimeScale` | moves +0x300 toward the value by 0.000833/frame | high |
| 0x10deb0 | `CAR_GetHumanRider` | first relation with valid && isHuman → AI rider | high |
| 0x10d8f8 | `CAR_FindHumanRelation` | scans the 6 relation records | high |
| 0x113128 | `Rider_GetDistToFinish` | returns +0x4d0 | high |
| 0x1298c8 | `AI_GetFrameCount` | returns `AI+0x08` | high |
| 0x10a960 | `CAR_CruiseInput` | sub 0 handler | high |
| 0x10ad78 | `CAR_BoardPressInput` | sub 1 | high |
| 0x10aa70 | `CAR_PrewindInput` | sub 2 | high |
| 0x10b590 | `CAR_NaturalAirInput` | sub 4 | high |
| 0x10b250 | `CAR_SpinInput` | sub 5 | high |
| 0x10b790 | `CAR_GateInput` | sub 6 | high |
| 0x10aed8 | `CAR_RailslideInput` | sub 7 | high |
| 0x10b750 | `CAR_WipeoutInput` | sub 8 | med |
| 0x1009e0 | `CAR_BehaveFollowMarker` | behaviour 0 | high |
| 0x100680 | `CAR_BehaveRaceLine` | behaviour 1 | high |
| 0x100f88 | `CAR_BehavePace` | behaviour 2 | med |
| 0x100b90 | `CAR_BehaveChaseRival` | behaviour 3 | med |
| 0x100610 | `CAR_GetSteerTarget` | look-ahead + correction toward the nearest path point | med |
| 0x100348 | `CAR_SteerToward` | target → −1..1 steer | med |
| 0x100250 | `CAR_SteerError` | lateral error × +0xdf4 | low |
| 0x10b980 | `CAR_FindTrickMarker` | path query for type 0x10, fills +0xe50/+0xe60 | high |
| 0x10bb18 | `CAR_FindSpeedMarker` | type 0x11 within 600 | high |
| 0x10bbf8 | `CAR_FindSidestepMarker` | type 0x13 within 50 | high |
| 0x10bfa8 | `CAR_PlanAirTrick` | sets +0xe10..+0xe3c from marker flags | high |
| 0x10b0e8 | `CAR_CancelAxesIfNoTime` | clears spin/flip when the trick no longer fits | med |
| 0x10cad8 | `CAR_ChooseTrick` | uber → tweak → basic | high |
| 0x10cbe0 | `CAR_ChooseBasicTrick` | random id, fits air time, counts +0xe7c | high |
| 0x10cd20 | `CAR_ChooseTweakTrick` | least used, +0xef4 | high |
| 0x10cf68 | `CAR_ChooseUberTrick` | least used, +0xeb8, needs uber timer | high |
| 0x10c1d0 / 0x10c258 / 0x10c320 / 0x10c3b8 | `CAR_RollRandomTrick` / `CAR_RollUber` / `CAR_RollTweak` / `CAR_RollBasic` | probability gates on +0xdfc | high |
| 0x10c0a8 | `CAR_SetTuck` | analog crouch from speed, off when aggression < 20 | med |
| 0x10c140 | `CAR_SetTuckBoost` | crouch + 0x30000 | low |
| 0x10bd10 | `CAR_PickTargetRider` | every 12 frames, nearest by distance × angle weight | high |
| 0x10d9e8 | `CAR_IsRival` | `Rider+0xf0 && Rider+0xf8 == idx` | high |
| 0x10dbf0 | `CAR_TryShove` | every 6 frames, aggression ≥ 3, 120-frame cooldown (`Rider+0xf4`) | med |
| 0x10da10 | `CAR_TryShoveBack` | reacts to a rider in anim 0xd within 200 cm | med |
| 0x10d1a0 | `CAR_ShouldRequestReset` | +0xe74 / +0xe78 counters | high |
| 0x10d410 | `CAR_ScorePathBranch` | Rider vslot 20 on the CAR vtable; branch flags in path+0x38 (speed class bits 7-9 in km/h steps of 20, difficulty bits 4-6) | med |
| 0x10d870 | `CAR_PathBranchWeight` | flag bits vs +0xe00 | med |
| 0x10d938 | `CAR_IsFarBehindHuman` | Δ < `Rider+0xdc` | high |

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

Stats, trick scoring and HUD messages for one rider. Round 2 mapped the trick part
from `RiderMetrix_ResetTrick` 0x117838, `RiderMetrix_FinishTrick` 0x11a228 and the
small adders, and checked it against CPU rider 1 before (frame 5003, in the air)
and after (5030) landing a 540 back flip with a grab.

| off | type | name | meaning | 5003 → 5030 | conf |
|---|---|---|---|---|---|
| 0x00 | i32 | `switchFlag` | stance ≠ natural stance at landing (FinishTrick arg) | 0 → 1 | med |
| 0x04 / 0x0c / 0x10 | i32 | | FinishTrick args: landing kind, `railStyle`, `pressType` | | low |
| 0x08 | i32 | | 1 after a spin-only landing (`FUN_00119c98`) | | low |
| 0x14 | f32 | `trickBoost` | boost value of the trick in progress (see Trick system) | 0.4155 → 0 | med |
| 0x18 | f32 | `multiplier` | 1.0; best trick multiplier collected (vslot 15) | 1 | med |
| 0x20 / 0x24, 0x28 / 0x2c | i32 / f32 | | rail style / press type of the trick and their timers (-1 = unused) | | low |
| 0x30 | f32 | `airTime` | seconds in the air (+1/60 per frame), -1 on the ground | 1.75, 1.80 (+3 frames) → -1 | high |
| 0x34 / 0x38 | f32 | `spin` / `flip` | spin / flip totals in rad (from the Spin object) | -9.425 / -6.283 → 0 | high |
| 0x3c | f32 | `holdRate` | grab hold rate (holdRate × 1.67e-6) | 0.000833 | med |
| 0x40 | f32 | `grabTime` | current grab hold time, -1 none | | med |
| 0x44 / 0x48 | f32 | `grabTimeSum` / `grabTimeMax` | | 0.983 | med |
| 0x4c / 0x50 / 0x54 / 0x58 | i32 | trick counts | grabs (id < 19) / second class (19..34) / ubers / super ubers in this trick | 0x50: 1 → 0 | med |
| 0x5c | i32 | `uberInProgress` | | | low |
| 0x60..0x6b | i32[3] | `trickIds` | ids of the tricks in the current trick (memset on reset) | 21 | med |
| 0x9c..0xac | | landing HUD record | 0xa0 points shown (2282), 0xa4 HUD timer (1.467 s), 0xa8 text ptr, 0xac 168 | | low |
| 0xf8 | i32 | | tricks landed | 0 → 1 | med |
| 0xfc / 0x100 | i32 | `totalSpinDeg` / `totalFlipDeg` | sums of landed rotations, rounded to 180° | 0 → 540 / 360 | high |
| 0x104 / 0x108 | i32 | | totals of +0x4c / +0x80 | | low |
| 0x110 | i32 | `tricksLanded` | | 0 → 1 | high |
| 0x114 / 0x118 / 0x11c | i32 | `ubersLanded` / `superUbersLanded` / | cumulative; 0x10e910 turns the increase of 0x114 into `Rider+0x2f4` | | med |
| 0x130 | i32 | `multiplierCount` | | | med |
| 0x144 | f32 | | total grab time | 0 → 0.983 | med |
| 0x14c / 0x150 / 0x154 / 0x158 | f32 | records | max air time / max grab time / max +0x24 / max +0x78 | 0 → 2.217 / 0.983 | med |
| 0x15c | f32 | `maxSpeed` | | 3333 | high |
| 0x160 | f32 | `speed` | | | high |
| 0x164 | f32 | `distance` | distance travelled, cm | 993,200 → 1,072,000 | high |
| 0x168 | i32 | | +1 per frame (race time in frames) | 749, 752 | med |
| 0x16c | i32[8] | `tierCounts` | landed tricks per score tier (`RiderMetrix_GetScoreTier`) | [3]: 0 → 1 | med |
| 0x18c | i32 | `bestTrickScore` | -1 none | -1 → 4150 | high |
| 0x190 | u64 | `bestTrick` | | | med |
| 0x198 | i32 | `totalScore` | | 0 → 4150 | high |
| 0x19c | i32 | | | | low |
| 0x1a8 | f32 | | speed again | | med |
| 0x1ac | `Rider*` | | | | high |
| 0x1b0 | ptr | `hud` | HUD state object (null for CPU riders without HUD) | | med |

Speed fields: live (player) +0x15c max speed 2209, +0x160 current speed 2207
(`RiderMetrix_Update` keeps the max).

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

- Slews #2, #8, #9, #12, #17, #18: writers are known (ground/rail state code) but
  not their meaning.
- Spin object +0x00 (input mode) and +0x40: semantics only partly read.
- Exact boost formula for spins/flips (`FUN_001198d8` not read; assumed symmetric to
  `FUN_00119898`) and the meaning of the score tiers.
- `ButtonTrick` fields +0x08..+0x0e and the second u16 of the uber variant records.
- `FUN_0010f1c0` types 4..6 (0x10f238 / 0x10f24c / 0x10f264).
- The CPU AI block (CAR +0xdf0..+0xf50) and Rider +0x0d8..+0x100: see the
  ComputerActorRider section.
- Whether 0x160 is "previous" or "render" matrix (identical while riding straight;
  written by the skeleton update 0x11eb98).
- The meaning of `Player+0xdf4/+0xdf8`.
- Resolved in round 2: the unit (centimetre), the trick fields 0x2dc..0x360, the
  four unnamed FX mixins, the boost meter and target speed.

## Proposed names (round 2, rider/trick; not applied)

| address | current | proposed | evidence | conf |
|---|---|---|---|---|
| 0x106848 | FUN_00106848 | `Rider_TryCatchRail` | finds a rail (`FUN_00108a48`), projects the velocity onto it, sets `railStyle`, banks the air trick, enters Railslide/BoardPress | high |
| 0x107578 | FUN_00107578 | `Rider_TryHandplant` | called on the Handplant bit (0x2000) from Cruise/Spin | med |
| 0x10e028 | FUN_0010e028 | `Rider_SetHudMessage` | writes +0x354 expiry frame / +0x358 id | high |
| 0x10e770 | FUN_0010e770 | `Rider_AddPowerUpBoost` | vslot 13, +0x2e8 += x | high |
| 0x10e7d0 | FUN_0010e7d0 | `Rider_AddPowerUpTrick` | vslot 14, +0x2ec += x | high |
| 0x10e830 | FUN_0010e830 | `Rider_AddTrickMultiplier` | vslot 15 | med |
| 0x10e910 | FUN_0010e910 | `Rider_BankAirTrickOnRail` | FinishTrick from Air when a rail is caught, raises `uberLevel` | med |
| 0x10f1c0 | FUN_0010f1c0 | `Rider_ApplyScriptEffect` | switch(type) → vslots 13/14/15...; only caller the script native 0x2ff850 | med |
| 0x113e80 | FUN_00113e80 | `Rider_SetSteerTarget` | slew #0 target/step | high |
| 0x113f38 | FUN_00113f38 | `Rider_SetRailBalanceTarget` | slew #5 | med |
| 0x113f88 | FUN_00113f88 | `Rider_SetCrouchBrakeTargets` | slews #4 / #3 | high |
| 0x114130 | FUN_00114130 | `Rider_UpdateBoost` | +0x2fc latch, meter drain | high |
| 0x114298 | FUN_00114298 | `Rider_Jump` | take-off impulse from crouch, +0x3c0, +0x5a4/+0x5a8 | med |
| 0x1158b8 | FUN_001158b8 | `Rider_SeedSpinFromPrewind` | args slews #16/#15 at take-off | low |
| 0x116120 | FUN_00116120 | `Rider_TryResetPath` | ResetPath bit → Static/Reset states, wrong-way HUD | high |
| 0x1161d0 | FUN_001161d0 | `Rider_TryBoardPress` | sets +0x330, sub-state 1 | high |
| 0x1162c8 | FUN_001162c8 | `Rider_TryPrewind` | jump bits → sub-state 2, +0x360 | high |
| 0x116378 | FUN_00116378 | `Rider_CheckFinished` | +0x470 ≥ 0 → sub-state 10 | high |
| 0x1163b0 | FUN_001163b0 | `Rider_UpdateAttack` | AttackLeft/Right bits, anims 0x142..0x147, +0x340/+0x350 | high |
| 0x119448 | FUN_00119448 | `RiderMetrix_AddMultiplier` | +0x130, +0x18 | med |
| 0x119708 | FUN_00119708 | `RiderMetrix_AddTrickPoints` | points table 0x530600 → +0x14, +0x3c | high |
| 0x1197d8 | FUN_001197d8 | `RiderMetrix_EndGrab` | class counters +0x4c..+0x58 | med |
| 0x119898 | FUN_00119898 | `RiderMetrix_SetSpin` | +0x34, +0x14 delta | high |
| 0x1198d8 | FUN_001198d8 | `RiderMetrix_SetFlip` | +0x38 (by symmetry) | med |
| 0x119c98 | FUN_00119c98 | `RiderMetrix_LandSpinOnly` | | low |
| 0x119d40 | FUN_00119d40 | `RiderMetrix_LandTrick` | FinishTrick + ResetTrick + store landing args | high |
| 0x1200d0 | FUN_001200d0 | `Rider_UpdateBoostTimers` | decays +0x2e8/+0x2ec/+0x2f0, meter leak by boostMode | high |
| 0x1352a8 | FUN_001352a8 | `RiderSpinSub_UpdateGrab` | grab state machine (+0x04/+0x08) | high |
| 0x14fea8 | FUN_0014fea8 | `TrickDB_GetUberVariant` | 0x530ec0 lookup | med |
| 0x1500d8 / 0x150118 | FUN_… | `TrickDB_GetStartAnim` / `TrickDB_GetTrickId` | 0x45aeb8 +0 / +4 | high |
| 0x150528 / 0x150540 | FUN_… | `TrickDB_GetPoints` / `TrickDB_GetHoldRate` | 0x530600 stride 8 | high (first) / med |
| 0x2eaae0 / 0x2eadd0 | FUN_… | `cTrickPowerUpFX_ctor` / `_Update` | RiderBase+0x9c0 | med |
| 0x2e3680 / 0x2e39d8 | FUN_… | `cPlayerPulseFX_ctor` / `_Update` | RiderBase+0xb00 | low |
| 0x2f0e88 / 0x2f1150 | FUN_… | `cAttackFX_ctor` / `_Update` | RiderBase+0xc70 | med |
| 0x2f6238 / 0x2f6518 / 0x2f6910 / 0x2f6ac8 | FUN_… | `cCheatCodeDetector_ctor` / `_Update` / `_Step` / `_PostEvent` | RiderBase+0xd20 | med |

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
