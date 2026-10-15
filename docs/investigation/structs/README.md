# Runtime data structures (phase 2 groundwork)

Field-level maps of the structures SSX 3 uses during a race, written so they can
become typed Rust structs (roadmap phase 2, step 2). Mapped 2026-10-15 from the
Ghidra project, a paused PCSX2 race read over PINE (`tools/pine.py`) and RAM
dumps of our headless runner one frame apart.

| file | contents | depth |
|---|---|---|
| [rider.md](rider.md) | `Rider` (virtual base, 0xb40), `RiderBase` state-machine/FX mixins (0xdf0), `Player` (0x1950), `ComputerActorRider` (0x1a90), AirPredictor, RiderModel, RiderMetrix, rider vtables, the per-frame rider update order | full layout; ~80% of Rider fields named (round 2: tricks, slews, boost, FX, CPU AI), all hot fields high confidence |
| [camera.md](camera.md) | `GameViewMan`, `CameraBase`/`GameCamera` (0x4b0), `CamController` + Chase/Manual/Script controllers, `AlgorithmList`, `CameraAlgorithm` → `ChaseCameraAlgorithm` (0x390) and the algorithm-id table, `RiderCameraTarget` | full layout for camera/controllers; chase tuning constants and filter fields mapped in round 2 |
| [world.md](world.md) | `GameModule` (all subsystem pointers, state machine, update order), the "AI" race manager, `World`, `cWorldCache` (+ `.sdb` header, section streamer, SectionTable, WorldMemman, SectionAllocator, cHullPage), `SectionMan`, `GameProcess`/`WorldObj` headers and the process list, `WorldLightMan`, `SkyBox`, `InstancesMan`, `OcclusionMan`, `SurfPropCache` | main objects mapped; many scalar fields med/low |
| [render.md](render.md) | `PS2GraphicsMan`, the DMA kick path (`Renderer_DmaKickThread`), packet/VIF builders, `TexMan` | see file |
| [app.md](app.md) | `App`/`SSXApp` static object, module switching (FE / Game / debug menu), the gp globals that root everything | short pass, main fields high |
| [input.md](input.md) | `PS2Pad`, input bindings / `InputParser`, the rider control-input record | see file |

**Round 2 (2026-10-15):** closed most open questions: the rider trick system
(Spin sub-state object, grab state machine, trick and score tables, scoring → boost),
the 19 slewed parameters, boost meter / timers / uber level and the target-speed
table (the distance unit is confirmed as the centimetre), the four unnamed rider FX
mixins, RiderMetrix, the ComputerActorRider AI block and CPU tuning table, the AI
race-manager fields, the 16 GameModule state names, the chase-camera tuning (all
compiled-in constants, five rebuilt splines), App+0x08 (`PS2ExecMan`) and the .big
mount handles, and the render draw-slot packet formats (render.md). Each file now
also ends with a "Proposed names (round 2)" table; nothing was renamed in Ghidra in
round 2. Extra headless dumps: frames 5000-5003, 5010/5011, 5030, 5040-5042, 5060,
5120, 5240.

Every table gives offset, type, name, meaning, evidence (function and/or live
value) and confidence (high / med / low). Each file ends with open questions and
the Ghidra renames it made.

## Shared conventions

- **gp = 0x4a30f0.** Ghidra prints gp-relative globals as `iGpfffffXXX`; the address
  is `0x4a30f0 - 0x10000 + 0xXXXX` (e.g. `iGpfffff7b8` → 0x4a28a8).
- **Root pointers:** `*(0x4a28a8)` = static `SSXApp` at 0x4c9428; `App+0x84` =
  current `AppModule` (in a race the `GameModule`, also at `*(0x4a2c68)`);
  `GameModule+0x0c` = "AI" race manager → `Rider*[6]` at +0x28.
- **Z-up world**, rider-local Y = forward; velocities in units/s at 60 Hz.
- **gcc 2.95 object model:** vptr placed after the first polymorphic base's data
  (Rider's vptr at +0x6c0, CameraBase's at +0x90); vtable entries are 8 bytes
  `{i16 delta, i16 index, fnptr}` with slot 0 empty; virtual bases are reached
  through stored pointers (RiderBase keeps ~30 copies of `Rider*`). Many small
  virtual-method bodies are not Ghidra functions (no names, but readable with a
  disassembler).
- **Allocation tags:** `EA_MemAllocNamed(size, "Tag", flags, 0)` gives the size and
  usually the class name of almost every heap object.

## How these become Rust

Guest memory stays the source of truth during phase 2 (bit-exact replay must keep
working), so the first form is *typed views over guest RAM*, not owned Rust
structs:

```rust
/// View of a guest `Rider` (0xb40 bytes). Field accessors are generated from
/// the tables in docs/investigation/structs/rider.md.
#[derive(Clone, Copy)]
pub struct RiderPtr(GuestPtr);           // GuestPtr = u32 EE address
impl RiderPtr {
    pub fn pos(self, m: &Mem) -> Vec4 { m.read_vec4(self.0 + 0x110) }
    pub fn vel(self, m: &Mem) -> Vec4 { m.read_vec4(self.0 + 0x1e0) }
    pub fn owner(self, m: &Mem) -> RiderBasePtr { RiderBasePtr(m.read_u32(self.0 + 0x77c)) }
    // ...
}
```

Once a subsystem is fully native (phase 3), the same names become owned
`#[repr(C)]` structs with `const _: () = assert!(size_of::<Rider>() == 0xb40);`
and `offset_of!` checks against the tables.

Proposed names:

| guest class | Rust type | notes |
|---|---|---|
| Rider | `Rider` | split into `RiderPhysics` (+0x000..+0x6c0), `ScriptObjectBase` (+0x6d0..+0x750) and the tail; the 19 slews as `[Slew; 19]` with `struct Slew { cur: f32, max_step: f32, target: f32 }`; `RiderRelation { valid, _, dist, angle, .., stamp, .., aggressive }` |
| RiderBase / Player / ComputerActorRider | `RiderController` + `enum RiderKind { Player(PlayerData), Cpu(CpuBrain), Actor(CpuBrain) }` | the ~25 mixins become `enum MainState` / `enum SubState` with per-state data, plus `RiderFx` fields; the virtual base becomes a plain `Rider` member |
| AirPredictor, RiderModel, RiderMetrix | `AirPredictor`, `RiderModel`, `RiderStats` | |
| GameCamera / CameraBase | `GameCamera` | `CameraView { eye: Vec4, rot: Quat }` is the value algorithms return |
| CamController (+3 subclasses) | `enum CamController { Manual, Chase, Script }` | |
| CameraAlgorithm hierarchy | `trait CameraAlgorithm` + `ChaseCamState` | algorithm ids as `enum ChaseCamId` |
| AlgorithmList / node | `Vec<BlendedAlgorithm { alg, weight, rate, eased, owned }>` | |
| GameModule | `GameModule` | subsystem pointers → fields |
| AI race manager | `RaceManager` | |
| World / cWorldCache / SectionMan | `World`, `WorldCache`, `SectionManager` | |

### Hot fields in a race

Read or written every frame by the rider update, the camera or the renderer;
type these first:

- **Rider:** `pos` +0x110, `rot` +0x120, `world` +0x1a0..+0x1d0, `vel` +0x1e0,
  `slewed[19]` +0x1f0, `targetSpeed` +0x2e4, boost (+0x2f4/+0x2f8/+0x2fc),
  `groundNormal` +0x370 (and +0x380/+0x3a0/+0x3b0), impulse +0x3e0/+0x3f0,
  bounds +0x400/+0x410/+0x420, surface ids +0x430/+0x434/+0x438, path progress
  +0x490..+0x4d4, `owner` +0x77c, sub-object pointers +0x780..+0x790, `index` +0x86c,
  bone indices +0x89c..+0x918, `isPuppet` +0xac4.
- **RiderBase:** `mainState` +0xde0, `subState` +0xde4.
- **GameCamera:** `fov/near/far` +0x00, `eye` +0x20, `rot` +0x30, `view` +0x40,
  `active` +0xa0, embedded blend algorithm +0xc0 (its +0x20/+0x60/+0x70), shake
  +0x450..+0x45c, collision +0x460/+0x470.
- **ChaseCameraAlgorithm:** header +0x00..+0x10, +0x20, +0x30, +0x40, +0x60, +0x70,
  +0x100, +0x1b0.
- **World/GameModule:** see `world.md` (process list, section streamer, light cache).

## Tools used

Scratch helpers (not committed) wrapped the GhidraMCP HTTP backend
(`/decompile_function`, `/xrefs_to`, `/read_memory`, `/strings`,
`POST /rename_function_by_address`, `POST /set_decompiler_comment`) and PINE.
A census script grouped `*(T *)(var + 0xNN)` accesses by variable across cached
decompilations to find all users of an offset; the frame-to-frame diff of two
headless RAM dumps (`SSX3_RAM_DUMP`, frames 5000 and 5001 of the scripted race)
separated per-frame state from constants.
