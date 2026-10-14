# Input structures (SSX 3, SLUS_207.72)

Scope: the pad driver (`PS2Pad`), the per-tick sample ring (`PadQueue`), the
per-player edge and repeat tracker (`InputPad`), the scripted binding language
(`input.map` → `InputMap` bytecode → `InputVM`), the controller handle used
everywhere (`InputBinding`, at App+0xb0/+0xb4), online lockstep (`InputSync`), and the
bit-packed rider control word built by `Player_GetControlInput`.

Live values are from the paused PCSX2 Snow Jam race, read through PINE. Pad 1 was
connected with nothing pressed, and pad 2 was absent.

## Data flow

```
SSXApp_Tick (VSync side)            PadQueue_Poll
   PadQueue (gp-0x850 -> 0x618b60) --> for each PS2Pad: vt slot 2 Update(this, &ring[w].pad[i].ch[0], 24)
                                        ring[w].pad[i].status = returned channel count (24 / 0)
SSXApp_NextTick (game tick)         PadQueue_AdvanceWrite; for pad 0..1:
                                        InputPad_Update(App+0xa8[i], status, data)   (edge/repeat/debounce)
game code                           InputBinding_EvalBool/EvalFloat(App+0xb0[i], actionId)
                                        -> InputMap_Eval*(map, pad, id) -> InputVM_Execute(bytecode[id])
Player (RiderBase vt slot 1)        Player_GetControlInput(this, &ctrl) -> 8-byte bit-packed word
FUN_00121068                        Rider -> GetControlInput(&ctrl); FUN_00111728(riderBase, &ctrl)
                                        -> switch(subState) substate handler(ctrl)
```

## PS2Pad  (alloc tag "PS2Pad", size 0x180)

- Factory: `PS2Pad_Create(PS2Pad** out, int n)` (0x326d60). It clamps n to 2 and allocates
  `EA_MemAllocNamed(0x180, "PS2Pad", 0x4000000, 0)` (the flag is an alignment request: the
  live objects sit 0x200 apart at 0x5c0000 / 0x5c0200). It then calls
  `PS2Pad_ctor(p, port=i, slot=0)`.
- Base class `PadDevice` (inv., vt 0x48e4b8). Its slot 2 is pure virtual (`__pure_virtual`
  0x411530). Derived vt **0x48e488**:

| slot | PS2Pad (0x48e488) | PadDevice base (0x48e4b8) | meaning | conf |
|---:|---|---|---|---|
| 1 | 0x326e50 `PS2Pad_dtor` (scePadPortClose) | 0x3277d0 `PadDevice_dtor` | dtor | high |
| 2 | 0x326eb0 `PS2Pad_Update(this, float* out, int max)` → channel count | pure | read pad into float channels | high |
| 3 | 0x327738 (not a Ghidra fn): `return 2` | 0x327800 `return 0` | GetActuatorCount | high |
| 4 | 0x327740 (not a Ghidra fn): `SetActuator(this, int motor, float v)` | 0x327808 | motor 1 → +0x30 = (v != 0); otherwise +0x38 = v ? clamp(v*205+50, 50..255) : 0 | high |

`PadDevice_StopActuators` (0x326cf0) loops `i < slot3()` calling `slot4(i, 0.0)`.

### Fields

| off | type | name | meaning | evidence | live pad0 / pad1 | conf |
|---|---|---|---|---|---|---|
| 0x00 | vptr | vt | 0x48e488 | ctor | 0x48e488 / same | high |
| 0x04 | int | port | libpad port | ctor arg i; every scePad* call | 0 / 1 | high |
| 0x08 | int | slot | libpad slot | ctor arg 0 | 0 / 0 | high |
| 0x0c-0x2f | - | (unused?) | zeroed by ctor memset(+0xc,0,0x24) | ctor | 0 | low |
| 0x30 | int | smallMotor | requested small-motor value (0/1) | slot 4; Update | 0 / 0 | high |
| 0x34 | int | smallMotorSent | last value sent with scePadSetActDirect (-1 = force resend) | Update | 0 / -1 | high |
| 0x38 | int | bigMotor | requested big-motor value (0 or 50..255) | slot 4 | 0 / 0 | high |
| 0x3c | int | bigMotorSent | last sent (-1 = force) | Update | 0 / -1 | high |
| 0x40 | u8[6] | actDirect | buffer for scePadSetActDirect: [0] small, [1] big | Update | 0 | high |
| 0x48 | int | state | init state machine: 0 closed → PortOpen → 1 wait stable → 2 SetMainMode(analog, lock) → 3 wait → 4 InfoPressMode → 5 EnterPressMode → 6 wait req → 7 InfoAct → 8 SetActAlign → 9 wait req → **10 reading** | Update switch | 10 / 1 | high |
| 0x4c | int | reqRetry | scePadGetReqState retry counter (gives up after 5) | cases 6, 9 | 5 / 0 | high |
| 0x50 | int | modeLocked | SetMainMode accepted | cases 2, 9 | 1 / 0 | high |
| 0x54 | int | pressureMode | DS2 pressure mode active: buttons 4-15 come from the pressure bytes | case 6; channel fill | 1 / 0 | high |
| 0x58 | int | isDualShock | InfoMode(ID) == 7 (analog): sticks are valid | case 1 | 1 / 0 | high |
| 0x5c | int | actuatorsAligned | SetActAlign completed | case 9 | 1 / 0 | high |
| 0x60-0x7f | - | pad/unused | | | 0 | low |
| 0x80 | u8[0x100] | padDmaBuf | libpad DMA buffer passed to `scePadPortOpen` (64-byte aligned) | case 0 | holds `ff ff 79 00 7f 7f 7f 7f`: DS2 mode 0x79, no buttons, sticks centred | high |

### Channel layout written by Update (24 floats, 0..1)

Raw scePadRead bytes: [2],[3] are the active-low button masks, [4..7] are RX RY LX LY, and
[8..19] are pressures.

| ch | input | digital bit (byte[2]/[3]) | pressure byte |
|---:|---|---|---|
| 0 | Select | b2.0 | - |
| 1 | Start | b2.3 | - |
| 2 | L3 | b2.1 | - |
| 3 | R3 | b2.2 | - |
| 4 | D-Right | b2.5 | [8] |
| 5 | D-Left | b2.7 | [9] |
| 6 | D-Up | b2.4 | [10] |
| 7 | D-Down | b2.6 | [11] |
| 8 | Triangle | b3.4 | [12] |
| 9 | Circle | b3.5 | [13] |
| 10 | Cross | b3.6 | [14] |
| 11 | Square | b3.7 | [15] |
| 12 | L1 | b3.2 | [16] |
| 13 | R1 | b3.3 | [17] |
| 14 | L2 | b3.0 | [18] |
| 15 | R2 | b3.1 | [19] |
| 16/17 | right stick X -/+ | [4] | half-axes |
| 18/19 | right stick Y -/+ (up/down) | [5] | |
| 20/21 | left stick X -/+ | [6] | |
| 22/23 | left stick Y -/+ (up/down) | [7] | |

Each half-axis is computed as `max(0, (0x4f - v)*255/0x4f) / 255` or
`max(0, (v - 0xb0)*255/0x4f) / 255`. This gives a dead zone of 0x4f..0xb0, and the scale is
1/255 (gp-0x2e0c and gp-0x2e08). If the pad is not a DualShock, channels 16-23 are 0.
Update returns 0x18 only when state 10 is reached and scePadRead succeeded; otherwise it
returns 0. Confidence: high.

## PadQueue  (alloc tag "PadCache", size 0x2efc; global gp-0x850 = 0x4a28a0 → live 0x618b60)

This is a 30-entry ring of raw samples, filled at VSync rate (`SSXApp_Tick` → `PadQueue_Poll`)
and consumed one entry per game tick (`SSXApp_NextTick`). Functions 0x326a28-0x326cec:
`PadQueue_Init/Shutdown/StopAllActuators/AdvanceWrite/Poll/Flush/GetPadStatus/GetPadData`.

| off | type | name | meaning | live | conf |
|---|---|---|---|---|---|
| 0x0000 | Frame[30] | ring | Frame = 4 × PadSample (0x190 bytes) | | high |
| (PadSample) +0 | int | status | channel count returned by Update (24 or 0) | | high |
| (PadSample) +4 | float[24] | ch | channels, see above (0x64 bytes per pad) | | high |
| 0x2ee0 | int | readIdx | consumer index, advanced by AdvanceWrite (fails when it would reach writeIdx) | 7 | high |
| 0x2ee4 | int | writeIdx | producer index (Poll) | 8 | high |
| 0x2ee8 | int | padCount | from PS2Pad_Create (max 2) | 2 | high |
| 0x2eec | PS2Pad*[4] | pads | | 0x5c0000, 0x5c0200, 0, 0 | high |

Note: the names "AdvanceWrite" and "readIdx" are inherited and look swapped. Poll writes
at +0x2ee4, and NextTick reads `ring[+0x2ee0]` after the advance, so +0x2ee0 is the
consumer cursor.

## InputPad  (alloc tag "InputPad", size 0x2a4; App+0xa8 / +0xac)

This is the per-player debounced state, updated by `InputPad_Update(this, status, float* ch)`
(0x321298, renamed). SSXApp_Init allocates two of them. Live objects: 0x9cf3e0 (P1) and
0x9cf690 (P2).

| off | type | name | meaning | live P1 / P2 | conf |
|---|---|---|---|---|---|
| 0x00 | int | count | channel count (= status; 0 = no pad) | 24 / 0 | high |
| 0x04 | Chan[24] | ch | 0x1c bytes each | | high |

Chan (0x1c):

| off | type | name | meaning | conf |
|---|---|---|---|---|
| +0x00 | float | value | raw channel value | high |
| +0x04 | int | pressed | 1 on the tick it went to >0 | high |
| +0x08 | int | released | 1 on the tick it went to 0 | high |
| +0x0c | int | held | current boolean (value > 0) | high |
| +0x10 | int | repeat | auto-repeat pulse while held: first after 24 ticks, then every 12 | high |
| +0x14 | int | repeatTimer | countdown for repeat | high |
| +0x18 | int | stableTicks | debounce: a state change is accepted only when ≥3 ticks have passed since the last change (counter saturates at 3) | high |

Live P1: every channel is `{0.0, 0,0,0,0,0, 3}`. Nothing is held and every channel is stable.

## InputBinding  (alloc tag "Input", size 8; App+0xb0 / +0xb4)

This is the controller handle that the game, cameras, FE and Player use (`Player+0xdf0`,
`GameCamera` controllers +0xc, and FE wrappers that hold it at +0x18).

| off | type | name | meaning | live 0x53fcf8 / 0x53fd00 | conf |
|---|---|---|---|---|---|
| 0x0 | InputPad* | pad | set by FUN_00244400 (SetPad) | 0x9cf3e0 / 0x9cf690 | high |
| 0x4 | InputMap* | map | set by FUN_00244408 (SetMap); defaults to App+0xb8 (input.map) | 0x542560 / 0x542560 | high |

API (0x320bf0-0x320c70): `InputBinding_EvalFloat(b, id)`, `EvalBool(b, id)` (≠ 0.0),
`EvalFloatArgs/EvalBoolArgs` (float args f12-f19 → VM `arg1..arg8`).

## Input map language

`data/config/input.map` and `data/config/input2.map` are loaded in `SSXApp_BootLoad` by
FUN_00226b60. The results go to App+0xb8 (live 0x542560) and App+0xbc (live 0x542580), the
alternate controller configuration. The pipeline is:

1. **InputPreproc** (0x325f48-0x3266e8). A tiny C-style preprocessor with `#include`, line
   markers, and `/* */` and `//` comments. It writes into a growable 0x800-byte buffer
   `{int len; int cap; char* buf}`.
2. **InputParser** (alloc "InputParser", size 0x109ec; ctor 0x321428). This is a
   recursive-descent parser for C-like expressions: `?:`, `||`, `&&`, comparisons,
   `+ -`, `* /`, unary, 1- and 2-argument functions, and statements. Operands are
   `button<N>[.value|.held|.pressed|.released|.repeat]`, `padtype`, `arg1..8` and
   `config0..22`.
   - +0x00 cursor, +0x04 length, +0x08 text start, +0x0c token buffer, +0x8c/+0x90 line
     and column (init 1), +0xe0 free-node list head.
   - +0xe4: a pool of 2048 nodes `{int type; int a; int b; int c}`. Node types 5..0x21
     are the expression kinds (5 = ternary with 3 children, 6-0x15 binary, 0x12-0x21 unary
     or leaf).
   - +0x108ec int[63] free VM-register stack, +0x109e8 count. Confidence: med.
3. **InputMap** (alloc "InputMap", size 0x18): `InputMap_Init(map, 205)`, then
   `InputMap_CompileEntry(map, id, "ActionName")` × 205, then `InputMap_FinalizeOpcodes`
   (frees the parser and shrinks the code buffer).

| off | type | name | meaning | live 0x542560 | conf |
|---|---|---|---|---|---|
| 0x00 | int | numActions | 205 (0xcd) | 0xcd | high |
| 0x04 | int* | entryOffset | per action, a word offset into code (-1 = unbound); alloc "InputMapTable" | 0xa1b880 | high |
| 0x08 | InputParser* | parser | only during compile | 0 | high |
| 0x0c | int | codeCap | capacity in words | 0x31b | high |
| 0x10 | int | codeLen | words used | 0x31b | high |
| 0x14 | u32* | code | bytecode | 0xa18bd0 | high |

4. **InputVM** (stack object, 0x190 bytes; `InputVM_Execute(vm, InputPad*, code)` 0x325450):

| off | name | meaning |
|---|---|---|
| +0x000 | defaultAttr | attribute used when an operand has no explicit `.attr` (0 normally) |
| +0x004 | pad | InputPad* being evaluated |
| +0x008 | regs[64] | float registers (operands 0x00-0x3f) |
| +0x108 | builtins[32] | operands 0x40-0x5f: 0 padtype (reads 0), 1-8 arg1..8, 9-31 config0..22 (`InputVM_ReadBuiltin`) |

Operands 0x60-0x3fb are pad channels: `ch = (op-0x60)/6` and `attr = (op-0x60)%6`, read by
`InputVM_ReadPadChannel` (renamed). The attributes are 0 value, 1 held, 2 pressed,
3 released, 4 repeat, and 5 = use `defaultAttr`. A channel ≥ pad.count reads 0.

Instruction word layout: `op = bits 0-5, dst = bits 6-11, srcA = bits 12-21, srcB = bits 22-31`.
Opcodes:

| op | meaning |
|---|---|
| 0 | load immediate (next word) |
| 1 | mov |
| 2 | jz (operand in bits 6-15, offset in the high 16 bits) |
| 3 | jmp |
| 4 | or |
| 5 | and |
| 6-11 | > ≥ < ≤ == != |
| 0xc-0xf | + − × ÷ |
| 0x10/0x11 | select max/min-style conditionals |
| 0x12 | log-scaled power |
| 0x13 | atan2 |
| 0x14-0x1e | unary math (exp, log, floor/ceil, round, ...) |
| 0x1f | not |
| 0x20 | end |

Confidence: high for the operand space and the layout; med for the mnemonics of 0x10-0x1e.

### Action ids (InputMap entries, from FUN_00226b60; high)

```
0 CruiseTurn 1 CruiseCrouch 2 CruiseBrake 3 BoardPress 4 BoardPivot 5 AttackLeft 6 AttackRight
7 PrewindTurn 8 PrewindSpin 9 PrewindFlip 10 AirAdjRotFB 11 AirAdjRotLR 12 LateSpin 13 RailSpin
14 Spin 15 Flip 16 WipeoutRecover 17 JumpPressed 18 JumpHeld 19 BoostPressed 20 BoostHeld
21 Tweak 22 OllieHeld 23 RailBalance 24 HandplantBalance 25 Handplant 26 GateAnticipate
27-30 UberGrind1-4 31-45 Trick1-15 46 ResetPath 47 CameraActivate 48 CameraTurn 49 CameraPitch
50 CameraMove 51 CameraStrafe 52 CameraBoost 53 CameraInGameTurn 54 CameraInGamePitch
55 CameraInGameZoom 56 CameraInGameNext 57 CameraNextStatCam 58 CameraPrevStatCam
59 CameraGoToStatCam 60 MenuButton 61 MenuKey 62-65 MenuLeft/Right/Up/Down 66 MenuLeftShift
67 MenuRightShift 68-85 Editor* (Help, CursorSnap, Rotate L/R/U/D/Hold, ZoomIn/Out,
ModeOrient/Slow/Fast/Faster/Left/Right/Up/Down, TeleportAI) 86-90 PE* (AddEvent, StartPath,
MoveNode, DeleteNode, WaitForNoInput) 91-100 CV* (model viewer) 101 FEAdvance 102-110 Replay*
(Exit, Play, FForward, FBackward, StepForward, StepSlow, CycleCamera, DebugLoad, DebugSave)
111 NISSkip 112 UIStart 113 UISelect 114-117 UIUp/Down/Left/Right 118-121 UIUp2..UIRight2
122 UINext 123 UIPrevious 124 UIOption 125 UIMisc 126-129 UIRShift1/2 UILShift1/2
130-133 ...Held 134 UICheatMode 135-138 UICheatUp/Down/Left/Right 139-142 UICheatButton1-4
143 UICheatAnyKey 144-146 UICamZoom/Rotate/Pan 147-154 UI*Held 155-158 UI*Released
159 ScriptMenuKey 160-179 ScriptEdit* 180 ScreenCapture 181 VideoCapture
182 NetRiderAnalyzerToggle 183 MemCardKey 184-186 TestCamera* 187-189 ChaseCamZoom/LR/ChaseCam
190-194 ChaseManualCam* 195 TestTerrainType 196-199 MissionDebug* 200 VoipOnOff
201 ConsoleReset 202 ReplayMenuUp 203 ReplayMenuDown 204 ReplayTimeLine
```

The list above is complete (205 ids); the names come from the InputMap parser's string table.

## Rider control word  (8 bytes; `Player_GetControlInput` 0x127998 / `ComputerActorRider_GetControlInput` 0x10a768)

RiderBase vt slot 1 (Player vt 0x458338, ComputerActorRider vt 0x4585f0, abstract base
0x458710). The caller is `Rider_ProcessControlInput` (0x121068): it fills a stack `ctrl`, then `RiderBase_HandleSubStateInput` (0x111728) dispatches
it to the input handler of the current sub-state (`RiderBase+0xde4`; handler objects at
RiderBase+0x1c0...+0x390).

After the call, `Player+0xdf8` (`ReplayCache`, if any) records the word
(`ReplayCache_Update`). The AI version zeroes the word and uses per-substate think functions
(0x10a960 ...) that fill the same layout.

The word is zeroed first. Bits 0-11 of word 0 are never written (reserved). Common fields
in every substate: **w0 bit 12 = ResetPath (46)**. Analog fields are signed 6-bit
`(int)(v*31)` values, decoded as `((x<<k)>>26) * (1/31)` (gp-0x7554 = 0.032258). The rest is
a union keyed by sub-state. Fields are listed as word.bits = action.

| sub | w0 | w1 | handler (ctrl) | sub-state (name table 0x459dc8, see rider.md) | conf |
|---:|---|---|---|---|---|
| 0 | 13 Handplant, 14 JumpPressed, 15 JumpHeld, 16 BoostPressed, 17 BoostHeld, 18 AttackLeft, 19 AttackRight, 20-25 CruiseTurn, 26-31 CruiseCrouch | 0-5 CruiseBrake, 6-11 BoardPress | 0x131620 (+0x1c0) | Cruise (ground/cruise) | layout high, name med |
| 1 | 13 BoostPressed, 14 BoostHeld, 15 JumpHeld, 16 JumpPressed, 17 OllieHeld, 18-23 CruiseTurn, 24-29 RailBalance | 0-5 BoardPress, 6-11 BoardPivot | 0x12fc80 (+0x1d0) | BoardPress (board press/pivot) | layout high |
| 2 | 13 JumpHeld, 14 BoostHeld, 15-20 PrewindSpin, 21-26 PrewindFlip | 0-5 PrewindTurn, 6-11 RailBalance(23) | 0x12e9b8 (+0x1f0) | Prewind (jump wind-up) | med |
| 3 | 13 BoostPressed, 14 BoostHeld, 15-20 CruiseTurn, 21-26 RailBalance | - | 0x12e778 (+0x200) | Bump | layout high |
| 4 | 13 Handplant, 14 AttackLeft, 15 AttackRight, byte 2 (bits 16-23) trick index 0-14 / -1 (Trick1-15, FUN_001276f0), 24-29 CruiseTurn | 0-5 CruiseCrouch | 0x12f730 (+0x210) | NaturalAir (air) | med |
| 5 | 13 Handplant, 14 Tweak, 15 LateSpin, byte 2 trick index, 24-29 Spin | 0-5 Flip, 6-11 AirAdjRotFB, 12-17 AirAdjRotLR, 18-19 BoardPress tri-state (-1/0/1 at ±0.5) | 0x133308 (+0x230) | Spin (air trick / spin) | med |
| 6 | 12-17 GateAnticipate (only field; no ResetPath) | - | 0x12bf68 (+0x290) | Gate (start gate) | med |
| 7 | 13 JumpPressed, 14 JumpHeld, 15 BoostPressed, 16 BoostHeld, 17-24 uber index 0-3 / -1 (UberGrind1-4, FUN_00127848), 25-30 RailBalance | 0-5 RailSpin, 6-11 BoardPress(3) | 0x131d30 (+0x2b0) | Railslide (rail grind) | med |
| 8 | 13 WipeoutRecover | - | 0x12cb68 (+0x2c0) | Wipeout | med |
| 9 | (nothing) | - | 0x12f398 (+0x350) | Reset (scripted / no input) | low |
| 10 | 12-17 CruiseTurn (only field; no ResetPath) | - | 0x12c678 (+0x360) | FinishLine | low |
| 11 | 13 Handplant, 14-19 HandplantBalance | - | 0x132a30 (+0x370) | Handplant | med |
| 12 | 13 BoostPressed, 14 BoostHeld, 15-22 uber index, 23-28 RailBalance | - | 0x136508 (+0x390) | UberGrind | med |

Cross-checks: the sub-state 0 handler reads w0 bits 12, 13, 14, 15, 18, 19, then
`(w0<<6)>>26` (bits 20-25), `w0>>26`, `(w1<<26)>>26` and `(w1<<20)>>26`. The sub-state 3
handler reads bits 12-14, `(w0<<11)>>26` (15-20) and `(w0<<5)>>26` (21-26). The sub-state 12
handler compares `(w0<<9)>>24` (bits 15-22) to its current uber id. FUN_00116120(rider,
bit12, 0) (ResetPath) is the first call in every handler.

## InputSync  (alloc tag at 0x47b5f8, size 0x44; GameModule+0x7c; online only)

Created in `GameModule_Load` only when a net game is active:
`InputSync_ctor(p, delayTicks = latency/tick + 2, step = cGameComm+0x120)`. It is null in
the live offline race (GameModule 0xcbfa20, +0x7c = 0). It is a lockstep delay line:
local samples are sent and remote ones are received through `cGameComm_PopPacket` (0x1c-byte
packets), then re-expanded into a private InputPad.

| off | type | name | meaning | conf |
|---|---|---|---|---|
| 0x00 | int | delay | delayTicks × step | med |
| 0x04 | int | ringLen | (60/step)*step | med |
| 0x08 | int | step | ticks per net frame | med |
| 0x0c | int | ? | 0 | low |
| 0x10 | int | numChannels | ≤ 2 | high |
| 0x14 | Chan[2] | chan | 0x18 each | high |

Chan:

| off | name | meaning |
|---|---|---|
| +0 | samples | ringLen × 100-byte PadSample, alloc "InputSamples", status preset to 24 |
| +4 | readIdx | |
| +8 | writeIdx | |
| +0xc | srcPad | local InputPad* (0 = remote player) |
| +0x10 | syncedPad | InputPad 0x2a4, "SyncedPad" |
| +0x14 | binding | InputBinding, "SyncedInput" |

Add functions: FUN_00237140 (local) and FUN_00237280 (remote). Update is FUN_002373b8, and
the dtor is FUN_002370a0.

## Debounce

The `Debounce` class (0x3614d0 `Debounce_v01`, world `ModObj` subclass) is a world-object
trigger, not pad input. Pad debouncing is the 3-tick `stableTicks` rule in InputPad.

## Global instances (live)

| what | where | live |
|---|---|---|
| PadQueue* | gp-0x850 (0x4a28a0) | 0x618b60 |
| PS2Pad[0..1] | PadQueue+0x2eec | 0x5c0000, 0x5c0200 |
| InputPad P1/P2 | App(0x4c9428)+0xa8/+0xac | 0x9cf3e0, 0x9cf690 |
| InputBinding P1/P2 | App+0xb0/+0xb4 | 0x53fcf8, 0x53fd00 |
| InputMap input.map / input2.map | App+0xb8/+0xbc | 0x542560, 0x542580 |
| InputSync | GameModule+0x7c | 0 (offline) |

## Open questions

- PadQueue index naming: confirm which of +0x2ee0/+0x2ee4 is the producer when the queue is
  empty (Poll writes at +0x2ee4, but the existing function names say otherwise).
- Bits 0-11 of control word 0 are never written by either GetControlInput. Check whether
  ReplayCache or net code packs a frame tag there.
- Sub-state names are now known from the table at 0x459dc8 (rider.md): 1 BoardPress,
  3 Bump, 9 Reset, 10 FinishLine; the handlers were renamed `Rider<Sub>Sub_HandleInput`.
- When game code swaps `InputBinding.map` to input2.map (App+0xbc). This is probably the
  controller-config option. Who calls FUN_00244408 besides BootLoad, PlaybackMan (replay
  pads) and InputSync?
- PS2Pad +0x0c..+0x2f is zeroed but never read in Update. Is it the old button state of a
  base version?
- InputVM opcodes 0x10-0x1e: exact semantics (min/max/clamp/round) not verified.
- Renames are blocked outside the allowed range for: FUN_00226b60 (InputMap_LoadFromFile),
  FUN_00244400/00244408 (InputBinding_SetPad/SetMap), FUN_0011ff18 (Rider_GetControlInput thunk),
  FUN_001276f0 (Player_GetTrickButton), FUN_00127848 (Player_GetUberButton), and
  FUN_0017fe80... (FE `InputBinding_EvalBool(+0x18, UI*)` wrappers).

## Renames

| address | old | new | evidence | conf |
|---|---|---|---|---|
| 0x321298 | FUN_00321298 | InputPad_Update | Called from SSXApp_NextTick with (App+0xa8[i], PadQueue status, data). Writes the 24 × 0x1c channel records {value, pressed, released, held, repeat, timer, stable}. The object is "InputPad" 0x2a4 = 4 + 24×0x1c. | high |
| 0x3252f8 | FUN_003252f8 | InputVM_ReadPadChannel | Called from InputVM_ReadOperand for operands 0x60..0x3fb as (vm, ch, attr). The jump table reads InputPad channel value/held/pressed/released/repeat. | high |
| 0x325430 | FUN_00325430 | InputVM_ReadBuiltin | Operands 0x40..0x5f return vm+0x108+4i. EvalArgs stores arg1..8 at vm+0x10c.. | high |

There is also a decompiler comment at 0x326eb0 (PS2Pad_Update channel map).
