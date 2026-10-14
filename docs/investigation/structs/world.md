# Runtime structs: course and world objects

Scope: GameModule, the "AI" race manager, World (cWorldCache, its section streamer,
SectionTable, SectionAllocator, WorldMemman, cHullPage), SectionMan, WorldLightMan,
SkyBox, InstancesMan, OcclusionMan, GameProcess / ProcessList / WorldObj headers,
SurfPropCache, plus short notes on cWorldPainterMan, cWScriptCache, SnowFall,
cSplashManager, cTransitionMan and GameViewMan.

Method: Ghidra decompilation over the HTTP backend, plus live reads through PINE from a
paused PCSX2 Snow Jam race (single player, world file `data/worlds/BAM.*`). "Live"
values come from that snapshot. Confidence: **high** means it is read directly from
code, such as a ctor store, an allocation tag, or an unambiguous use. **med** means it is
inferred from behaviour. **low** is a guess.

Conventions: `gp = 0x4a30f0`. Every "slot k" is a gcc 2.95 vtable slot, with its fnptr at
vt+8k+4.

## Globals

| Address | gp form | Type | Meaning | Live |
|---|---|---|---|---|
| 0x4a28a8 | gp-0x848 | SSXApp* | g_App (static at 0x4c9428) | 0x4c9428 |
| 0x4c9428+0x84 | | AppModule* | current module (GameModule in game) | 0xcbfa20 |
| 0x4a2c68 | gp-0x488 | GameModule* | g_GameModule. GameModule_Load sets it to `this` at its end; the ctor and every state-object ctor clear it | 0xcbfa20 |
| 0x4a47b8 | gp+0x16c8 | World* | g_World. World_ctor sets it, the World dtor clears it, World_GetInstance (0x2d1bd8) returns it | 0x542f00 |
| 0x4a5988 | gp+0x2898 | ProcessList | the GameProcess priority list (9 priorities) | {9, 0xcbffd0} |
| 0x4a5b64 | gp+0x2a74 | ? | timer block: +0x10 ticks/s(?), +0x18 frame time, +0x1c frame counter (used by section and hull LRU timers) | |
| 0x44216c | | struct[50] {int worldSection; int state; int poolIdx?; int ?} (0x10 each) | SectionMan's logical-section table (see SectionMan) | |
| 0x442480 | | struct[] (0x28 each) | event/track -> section-set table used by SectionMan_LoadEvent | |
| 0x4a2a50 | | u32 | game pause/freeze flags read by GameModule_Update (bit0 paused, bit1 skip pri-1 world-object update, bit2 skip pri-3, bit3 skip script tick, bit4 skip pri-2) (med) | |

---

## GameModule (alloc in the module factory; vt 0x47d110; base AppModule vt 0x47d980)

ctor 0x22e968, dtor 0x243838. Slots: 1 dtor, 2 `GameModule_Load` 0x22ebc8,
3 `GameModule_Unload` 0x22fa98, 4 `GameModule_TryExit` 0x230e98, 5 `GameModule_Enter`
0x230050, 6 `GameModule_Update` 0x2306b8, 7 **Render at 0x22b008** (not a Ghidra function:
large frame 0x7c20, calls GameViewMan 0x22e8b8/0x22e920, AppModule_RenderFade,
`GameModule_RenderUI` 0x22c830, 0x22add8(World, OcclusionMan, Nash) for the world pass),
8 0x231348 and 9 0x2314d0 (state serialisation, direction unknown).

Live object 0xcbfa20. AppModule base fields: +0 vptr, +4 ?, +8 exit-request flag (Update
sets it to 1 when App+4 != 0).

| Off | Type | Name | Meaning / evidence | Live | Conf |
|---|---|---|---|---|---|
| 0x00 | vptr | | 0x47d110 | 0x47d110 | high |
| 0x08 | int | exitRequested | set in Update when g_App+4 != 0 | 0 | med |
| 0x0c | AI* | raceMan | `EA_MemAllocNamed(0xd0,"AI")` + AI_ctor; Load calls slot 2 (Init) | 0x5b3300 | high |
| 0x10 | World* | world | alloc 0x18 "World" | 0x542f00 | high |
| 0x14 | SkyBox* | skyBox | alloc 0x18 "SkyBox" in GameModule_LoadWorld | 0x542ea0 | high |
| 0x18 | cPS2FogParticleMan* | fogParticles | alloc 0x3010 "cPS2FogParticleMan", ctor 0x2dbf80, vt 0x4882e0 at +4 | 0x1464050 | high |
| 0x1c | ? | | return of 0x2ec3e0() in LoadWorld | 0x1467070 | low |
| 0x20 | "Nash"* (0xa8) | nash | alloc 0xa8 "Nash"; [2][2][2] array of 0x14-byte records, plus +0xa0/+0xa4. Passed to World load (0x3a6ae0), 0x334800 (collision range), WorldLightMan+0, and the world render. Probably a collision/hull query cache | 0x5b5b00 | tag high, meaning low |
| 0x24 | Font* | debugFont | alloc 0x84 "DebugFont", `data/fonts/debugfnt.sfn` | 0x5b3700 | high |
| 0x28 | Replay* | replay | alloc 0x688 "Replay". *replay = replay mode (Update compares it with 3 and with <10) | 0x146d8a0 | high |
| 0x2c | FXLoader* | fxLoader | alloc 1 "FXLoader" | 0x53ff10 | high |
| 0x30 | WScriptMan* | scriptMan | alloc 0x498; +0x1c4/+0x1c8 event queue read in Update | 0xb84c70 | high |
| 0x34 | PlaybackMan* | playback | alloc 0x7c; +0x70 == 1 means playback active | 0x58c580 | high |
| 0x38 | CrowdMan2d* | | alloc 0x2720; update 0x2294c8 | 0xb85110 | high |
| 0x3c | CrowdMan3d* | | alloc 0x9c4; update 0x3440c8 | 0xba8180 | high |
| 0x40 | MultiParticleMan* | | alloc 0x10; update 0x357bf8 | 0x540720 | high |
| 0x44 | SurfProp[19]* | surfProps | alloc 0xd10 "SurfPropCache" = 19 x 0xb0. See SurfPropCache | 0xbac490 | high |
| 0x48 | UIEngine* | ui | alloc 0x70; package `data/ui/ov`; +0xc OVUIInput, +0x14 OVUISound, +0x18 UIStateMgr | 0x588900 | high |
| 0x4c | cWorldSound* | worldSound | alloc 0x250 "cWorldSound"; cWorldSound_Update(ws, view+0x20, i) per view | 0xbad1b0 | high |
| 0x50 | GameProcess(0x14)* | aiVfxProc | GameProcess at priority 6, vt 0x47cb60, +0x10 = PS2AIVisualEffectsMan | 0x543ce0 | high |
| 0x54 | GameProcess(0x14)* | vfxProc | priority 3, vt 0x47d070, +0x10 = PS2VisualEffectsMan | 0x543d00 | high |
| 0x58 | GameProcess(0x14)* | statsProc | priority 4, vt 0x47cae8, +0x10 = SSXStatsMan | 0x5441a0 | high |
| 0x5c | cTransitionMan*[2] | transition | alloc 0x84 each, ctor(p, prio 3, index) | 0x5bb500, 0x5bb400 | high |
| 0x64 | cCutSceneTransitionMan* | | alloc 0x48, prio 3 | 0x58c880 | high |
| 0x68 | cSplashManager*[2] | splash | alloc 0x1030 each, prio 2 | 0x1574f90, 0x1555540 | high |
| 0x70 | cFlagManager* | | alloc 0x1718, prio 2 | 0x1556580 | high |
| 0x74 | SnowFall* | snow | alloc 0x60, prio 2 | 0x58c900 | high |
| 0x78 | SectionMan* | sectionMan | alloc 0x1d4 "SectionMan" in LoadWorld; Update checks +0x1d0 | 0x5c2a00 | high |
| 0x7c | InputSync* | inputSync | alloc 0x44, only in netplay (`DAT_00534b30 != 0`) | 0 | high |
| 0x80 | ? | | never written in the code read so far | 0x05010000 | low |
| 0x84 | GameViewMan* | views | alloc 0x38 "GameViewMan" | 0x56e140 | high |
| 0x88 | InstancesMan* | instances | alloc 0x10, ctor(p, 0x80, 0x40) | 0x540760 | high |
| 0x8c | OcclusionMan* | occlusion | alloc 0x114 | 0x5dd400 | high |
| 0x90 | WorldLightMan* | lights | alloc 0x32a0, ctor(p, this->nash) | 0xacb940 | high |
| 0x94 | LoadIndicator* | | alloc 0x10 | 0x540700 | high |
| 0x98 | GenericStreamPause* | | GenericStreamPause_Create | 0x53fd50 | high |
| 0x9c | int | worldStreamWasIdle | Update: set to 1 after a streaming pass | 0 | med |
| 0xa0 | int | flushRequested | when set, Update calls renderer slot 0x9c, GameViewMan 0x22e8b8, and 0x3a88a8 (flush hull pages) | 0 | med |
| 0xa4 | MenuSys* | menuSys | alloc 0x68; +0x58 font, +0x5c pad, +0x60 menu font | 0x58c780 | high |
| 0xa8 | BigFile* | worldBig | alloc 8; opens `data/worlds/<trk>.big` | 0x53fdf8 | high |
| 0xac | int | frameTicks | ++ per Update. GameModule_RenderUI feeds it to SSXAudioSystem_Update and then clears it | 0 | med |
| 0xb0..0x1fb | embedded state objects | | see below | | high |
| 0x1fc | GameState* | prevState | | 0xcbfb00 | high |
| 0x200 | GameState* | curState | | 0xcbfb10 (state 11) | high |
| 0x204 | GameState* | overlayState | transient state (slots 2/3/8) | 0 | high |
| 0x208 | int | pendingOverlayId | set by GameModule_RequestOverlayState | 0 | high |
| 0x20c | int | overlayId | | 0 | high |
| 0x210 | int | pendingStateId | set by GameModule_RequestState | 0 | high |
| 0x214 | int | curStateId | 3 makes Update merge pending priority 1 | 4 | high |
| 0x218 | int | prevStateId | | 3 | high |
| 0x21c | int | pendingStateArg | passed to the new state's Enter (slot 5) | 0 | high |
| 0x220 | handle | menuFont | renderer slot 0x174 on `data/fonts/menu.ssh` | 0x65b | high |
| 0x224 | int | | ProcessStateChange returns early if set (non-netplay) | 0 | low |
| 0x228 | byte | | passed to OVUIInput slot 0x164 on pause | | low |

Ctor fields: +0x0c, +0x24, +0x48, +0x7c, +0x84 = 0, +0x228 (word 0x8a) = -1.

### GameModule state objects (embedded, 0x10..0x24 bytes each)

Layout: `{int id; int ?; int ?; vptr @+0xc; ...}`. The base vt 0x47d6e8 (slot 1 dtor 0x243c80;
slots 2..9 are `jr ra` stubs at 0x243cb0+8k). Slots used by GameModule_ProcessStateChange
(0x230f40): **2 OverlayEnter, 3 OverlayUpdate, 5 Enter(arg), 6 Update, 7 Exit,
8 IsOverlayDone**.

| id | Offset | vt | Note |
|---|---|---|---|
| 1 | +0xb0 | 0x47d530 | ctor 0x234188. Overrides slots 5/6/7 (0x2341d0/0x2343b0/0x234750) |
| 2 | +0xd0 | 0x47d270 | |
| 3 | +0xe0 | 0x47d4d8 | |
| 4 | +0xf0 | 0x47d480 | |
| 5 | +0x100 | 0x47d5e0 | ctor 0x233c10 |
| 6 | +0x118 | 0x47d690 | |
| 7 | +0x128 | 0x47d218 | |
| 8 | +0x138 | 0x47d638 | |
| 9 | +0x148 | 0x47d588 | |
| 10 | +0x158 | 0x47d428 | ctor 0x234ef8, own slots 2..7 (0x234f40..0x235868). Default after Load (state 10, or 0x10 for replay) |
| 11 | +0x178 | 0x47d2c8 | ctor 0x236868. Used by SectionMan event switch (RequestState(11), overlay 10) |
| 12 | +0x18c | 0x47d1c0 | |
| 13 | +0x19c | 0x47d3d0 | |
| 14 | +0x1b8 | 0x47d320 | ctor 0x236208 |
| 15 | +0x1dc | 0x47d378 | |
| 16 | +0x1ec | 0x47d168 | |

State names are unknown (no strings). The live current state at +0x200 = 0xcbfb10, which
falls inside the id-14 object region, while +0x214 = 4. The id-to-offset map is taken from
the switch in 0x230f40. The live pointer does not line up with the table, so check
before relying on it (low).

### Update/Render order (GameModule_Update 0x2306b8)

World streaming (World 0x3a6928 -> cWorldCache tick), SectionMan_Update, then, when not
paused: UIEngine, ProcessStateChange, Replay_Update, then the process list:
`ProcessList_UpdatePriority(1)` (world objects, unless flag bit1), `(5)`, `(6)`,
`ProcessList_PostUpdatePriority(1)`, AI slot 3 (Update), `(2)`, `(3)`, WScriptMan,
CrowdMan2d/3d, MultiParticleMan, `ProcessList_MergePending(1)`, `(4)`, then
cWorldSound per view and each rider's vt(+0x6c0) slot 19. Render per view (RenderUI)
calls `ProcessList_RenderPriority(3)`.

---

## "AI" race manager (alloc 0xd0 "AI"; ctor 0x1286a0; main vt 0x458488 at +0xcc)

Slots of 0x458488: 1 dtor 0x1287b8, 2 Init 0x128818, 3 Update 0x128af0, 4 Render 0x1291e0,
5 Restart 0x129768, 6 CreatePlayers 0x1298e0, 7 SaveState 0x12b7f0, 8 LoadState 0x12b948.
Live 0x5b3300, Race state.

| Off | Type | Name | Meaning / evidence | Live | Conf |
|---|---|---|---|---|---|
| 0x00 | int | raceState | 1 GameInit .. 7 Shutdown (AI_SelectRaceState 0x113b48) | 5 (Race) | high |
| 0x04 | int | prevRaceState | AI_SetRaceState; equal after the transition | 5 | high |
| 0x08 | int | | counter | 0x256 | low |
| 0x0c | int | raceTicks? | SectionMan restart writes 0 | 0x1a2 | med |
| 0x10 | int | | Init 0 | 1 | low |
| 0x14 | int | cpuActive | Update includes CPU riders only if != 0 | 1 | med |
| 0x18 | int | | | -1 | low |
| 0x1c | int | | Init 0 | 0 | low |
| 0x20..0x24 | int[2] | | Init 0 | 0 | low |
| 0x28 | Rider*[6] | riders | Init clears 6 (0x28..0x3c); Update iterates `riders[i]->vt(+0x6c0)` for i < +0x78 | 0x14701a0, 0x18d0c40, ... | high |
| 0x40 | Player*[2] | players | AI_Update: `players[i]+0x18` = rider | 0x146f390, 0 | high |
| 0x48 | ComputerRider*[5] | cpu | Init clears 0x48..0x58 | 0x18cfcf0 .. 0x1912170 | high |
| 0x5c | ActorRider* | actors[?] | iterated for +0x84 actors | 0x5aae00 | med |
| 0x60 | float | | | 110.0 | low |
| 0x64 | float | | | 133.0 | low |
| 0x6c | int | | | 4 | low |
| 0x70 | Rider-owner* | missionRider? | Init 0; used as the 2nd camera subject in one Update mode | 0 | low |
| 0x74 | int | | ctor 0 | 1 | low |
| 0x78 | int | riderCount | Init 0 | 6 | high |
| 0x7c | int | playerCount | | 1 | high |
| 0x80 | int | cpuCount | Load: `if (+0x80 == 0) AI_SetupActorRiders` | 5 | high |
| 0x84 | int | actorCount | | 0 | high |
| 0x88 | int | | if >= 1 with players, Update takes the replay/playback path | 0 | med |
| 0x8c | int | actorsActive | Load clears it after SetupActorRiders | 0 | med |
| 0x90 | int | | Init 0 | 0 | low |
| 0x98 | RaceState* | curStateObj | AI_SelectRaceState | 0x5b33c0 (= +0xc0) | high |
| 0x9c | RaceState* | prevStateObj | Exit (slot 2 at +0x14) is called on it, then cleared | 0 | high |
| 0xa0 | char* | stateName | "GameInit"/"Freeride"/"PreRace"/"Countdown"/"Race"/"EndRace"/"Shutdown" | 0x4a0e98 "Race" | high |
| 0xa4 | FocusMan* | focus | alloc 0x8e0 (tag 0x457f00), inline-initialised | 0x146df30 | high |
| 0xa8 | DangerMan* | danger | alloc 0xb60, DangerMan_ctor(p, this) | 0x146e820 | high |
| 0xac | vptr | GameInit state | vt 0x4585c8 | | high |
| 0xb0 | vptr | Freeride | 0x4585a0 | | high |
| 0xb4 | vptr | PreRace | 0x458550 | | high |
| 0xb8 | — | (unused; 0) | | 0 | high |
| 0xbc | vptr | Countdown | 0x458578 | | high |
| 0xc0 | vptr | Race | 0x458528 | | high |
| 0xc4 | vptr | EndRace | 0x458500 | | high |
| 0xc8 | vptr | Shutdown | 0x4584d8 | | high |
| 0xcc | vptr | main | 0x458488 | | high |

Race-state objects are just a vptr (4 bytes). Their slots are 1 Enter(ai) (+0xc), 2 Exit(ai) (+0x14),
3 Update(ai) (+0x1c) (AI_UpdateRaceState 0x113c20).

---

## World (alloc 0x18 "World"; ctor 0x3a6688, dtor 0x3a6740; no vtable)

| Off | Type | Name | Meaning | Live | Conf |
|---|---|---|---|---|---|
| 0x00 | cWorldCache* | cache | alloc 0x1c00 "cWorldCache" | 0xc31590 | high |
| 0x04 | cWScriptCache* | scriptCache | alloc 0x10 | 0x540740 | high |
| 0x08 | cWorldPainterMan* | painter | alloc 8 | 0x53fe10 | high |
| 0x0c | ? | | not set in the ctor | 0xffff | low |
| 0x10 | int | | ctor 0 | 0 | high |
| 0x14 | int | | ctor 0 | 0 | high |

The ctor also calls `World_InitSphereTreeCache(10,4,2,5)`. Methods (outside the allowed
rename ranges, so proposals only): 0x3a6ae0 **World_Load(name, nash)** (cWorldCache
reset+load `|data/worlds/<name>.sdb`, cWScriptCache alloc per section, painter AllocFiles),
0x3a6928 **World_Update** (-> cWorldCache 0x3a8290), 0x3a67e0/0x3a6800 **World_ResetSections**
(sets flag bit 1 on every type-3 resource of resident sections), 0x3a6948 **World_SdbHasSections(name, n, list)**,
0x3a6b78..0x3a6d18 cWScriptCache forwarders, 0x3a88a8 **cWorldCache_FlushHullPages**.

### cWorldCache (0x1c00, "cWorldCache"; ctor 0x3a7dd0, dtor 0x3a7e38)

| Off | Type | Name | Meaning | Live | Conf |
|---|---|---|---|---|---|
| 0x000 | SdbHeader* | sdb | `.sdb` file loaded by EAFile_LoadFileEx (0x3a7fc0) | 0xbd7400 | high |
| 0x004 | SectionTable* | sharedTable | alloc 0xe8 "SharedTable" over sdb+0x18 (section id 0xff) | 0x5b4f00 | high |
| 0x008 | SectionTable** | sectionTables | `new[nSections]` "SectionTables". Entry set by 0x3a81a8 (alloc "SectionTable") on section load and freed by 0x3a8230 | 0x5b3b00 | high |
| 0x00c | WorldMemman* | memman | alloc 0x13c "WORLD MEMMAN" | 0x5c2c00 | high |
| 0x010 | SectionStreamer (0x2f0) | streamer | embedded; ctor 0x3a8f80 -> 0x3a9188, see below | | high |
| 0x300 | HullStreamer (0xe8) | hullStream | embedded; ctor 0x3a6d50, dtor 0x3a6e20. +0 owner, +4 filename `|data/worlds/BAM.ssb` | | high |
| 0x390 | int | hullStream.busy (+0x90) | a request is in flight | 0 | high |
| 0x3d4 | int | streamEnable (hullStream+0xd4) | GameModule_Update and LoadWorld write 0/1 here through `*world` | 1 | high |
| 0x3e8 | cPS2ChunkResolve* | resolver | 0x3aad98, alloc 0x14 "cPS2ChunkResolve", vt 0x495090 at +8 (base 0x495150). Slot 2 Init(nash), 3 Resolve(type, handle, data) -> ptr, 4 Release(type, handle, ptr) | 0x542e40 | high |
| 0x3ec | u32 | nHullPages | = sdb+0x10 | 0x9f (159) | high |
| 0x3f0 | HullSlot[nHull] (0x18 each) | hullSlots | {state 0 free/1 wanted/2 loading/3 resident/4 releasing; int pinned; cHullPage*; float priority; int lastUseTime; int releaseFrame} | slot 0 = {3, 1, 0x540770, 0.91, 0x997, 0} | high |
| 0x1bf0 | int | hullTimeout | = ticks/s * const (ctor) | | med |

Per-frame tick 0x3a8290: streamer update (0x3a9258), hull LRU 0x3a8668 (picks the
highest-priority wanted slot, creates a cHullPage, queues `StreamFile_QueueRange`), then
the hull stream pump 0x3a7098 (EAC_Unpack chunks, 0x3a7238 chunk parser ->
0x3a8a90 alloc / 0x3a8cd0 resolve).

Resource handle format (high): a section-table entry is a u32 `refcount:8 | (ptr>>2):24`.
A handle is `type` (0..27) plus `(index<<8) | sectionIdx`, where sectionIdx 0xff is the shared table.
Types 1,2,3,11,12 allocate in WorldMemman pool flags 0x22000000, types 9,10 in 0x45000000,
type 14 is "BxStream AIPATH", and other shared types use "BxStream SHAREDMEM".

### SdbHeader (`.sdb`, version short 0x3800)

| Off | Meaning | Live (BAM) |
|---|---|---|
| 0x00 | u16 version 0x3800 (checked in 0x3a6948) | 0x3800 |
| 0x04 | float | 0.89 |
| 0x08 | nSections | 0x31 (49) |
| 0x0c | nNodes (0x60-byte BSP/AABB nodes) | 0xb7 |
| 0x10 | nHulls (0x44-byte hull descriptors) | 0x9f |
| 0x14 | Section* (fixed up to sdb+0x50) | |
| 0x18 | short[28] shared-table counts | |
| 0x50 | Section[nSections] (0x58 each): +0 name[16] ("TRANSP", "A", "A_ABA1", ...), +0x18 hull index, +0x1c root node*, +0x20 short[28] per-type counts. Then nodes (16-aligned, 0x60 each: +0 AABB min, +0x10 max, +0x20/+0x30 split planes, +0x50/+0x54 children, +0x58 hull*). Then hulls (0x44 each) | |

### SectionStreamer (embedded at cWorldCache+0x10, 0x2f0; class name unknown)

| Off (rel) | Meaning | Conf |
|---|---|---|
| 0x00 | owner cWorldCache* | high |
| 0x04 | SdbHeader* | high |
| 0x08 | Section* array | high |
| 0x0c | Node* array | high |
| 0x10 | Hull* array (0x44 each; 0x3a9a98) | high |
| 0x14 | {int state; float priority}[64] per section. States: 0 unloaded, 1 load requested, 2 loading (hull pending), 3 loaded/idle, 4 unloading, 5 resident, 6 resident+active (0x3a9258, 0x3a98c8, 0x3a9958, 0x3a99d8). Live: sections show 6 with priority 0.9-0.92 | high |
| 0x220 | world AABB min/max (copied from section 0 root node in 0x3a9000). Live bytes here look like garbage: unresolved | low |
| 0x240 | View[2] (0x50 each): +0 active, +4 dirty, +8 time, +0x10 position, +0x20 min, +0x30 max, +0x40 radius, +0x44/+0x48 nearest-section distances (0x3a9658, 0x3a9d60) | high |
| 0x2e0 | int residentCount, 0x2e4 activeCount | med |

Section names starting with ASKY..ESKY get WorldMemman pool 0, all others pool 2.

### SectionTable (0xe8; ctor 0x3a7c88, dtor 0x3a7d80)

`+0 u32* storage; +4 {u32* entries; int count}[28]; +0xe4 short* counts`. Total
storage is `sum(counts)*4` ("WorldCacheTableMem"). High.

### WorldMemman (0x13c "WORLD MEMMAN"; ctor 0x3a7a20, dtor 0x3a7aa8)

Three BlockPools of 0x14 each at +0, +0x14, +0x28 (0x3a7598:
`{blockSize, count, freeCount, Block* blocks, Block* freeHead}`, blocks of 0x18:
`{void* mem, size, used, padding, inUse, next}`, "WORLD BLOCK ARRAY"/"WORLD MEMBLOCK").
Parameter table at 0x44c1e4 (live): pool0 = 2 x 0x4000, pool1 = 0 x 0x8000, pool2 =
0x35 x 0x20000. +0x3c SectionAllocator*[64] per section (0x3a7b18 allocates
"Section Allocator", 0x3a7b98 frees). High for layout, med for which pool is which.

### SectionAllocator (0x14; ctor 0x3a77b0)

`{BlockPool* pool; int sectionIdx?; int nBlocks; Block* head; Block* cur}`. Alloc 0x3a7878
(alignment `4 << ((flags>>24)&0xf)`), free-all 0x3a7818. High.

### cHullPage (0x10 "cHullPage"; ctor 0x3a7bd8, dtor 0x3a7c30)

`{Hull* hull; int capacity (= *(u16*)hull); int nHandles; u32* handles ("cHullPage Entries")}`.
It records every resource handle resolved while the hull streams in, so the page can be released
(0x3a8528 walks the list backwards, 0x3a8e20 releases each one). High.

---

## SectionMan (0x1d4 "SectionMan"; ctor 0x22cce8, dtor 0x22cd18; no vtable)

Logical course-section manager (a peak/event's set of world sections, plus skies). It
works on the global table 0x44216c[50] `{worldSecIdx (-1 = absent), state, ?, poolIdx}`,
with states 0 unloaded, 1 loaded, 2 loaded+active, 3 load queued, 4 load+activate queued,
5 unload pending, 6 loading, 7 unloading, 8 loading-to-active. Logical ids 0x2c..0x30 are
the five skies.

| Off | Type | Meaning | Live | Conf |
|---|---|---|---|---|
| 0x000 | {int id; Node* next}[50] | node pool (SectionMan_InitNodePool 0x22e228) | | high |
| 0x190 | int | pool sentinel id 0x31 | 0x31 | med |
| 0x198 | Node* | free list head | 0x5c2a10 | high |
| 0x19c | Node* | queued-section list head (QueueSection 0x22e298 / DequeueSection 0x22e360) | 0 | high |
| 0x1a0 | int | queued count | 0 | high |
| 0x1a4 | int | skyEnabled; LoadWorld sets it to 1 | 0 | high |
| 0x1a8 | int | skyChangePending | 0 | high |
| 0x1ac | int | pendingSkyId (0x2c..0x30) | -1 | high |
| 0x1b0 | GameModule* | owner | 0xcbfa20 | high |
| 0x1b4 | int | event-switch phase 0 idle / 1 start / 2 / 3 / 4 (Update) | 0 | high |
| 0x1b8 | int | event-switch arg | 0 | med |
| 0x1bc | int | current event/track id (Init 0x16). 0xe..0x10 are special-cased in 0x22d6c8 | 0 | high |
| 0x1c0 | int | eventLoaded | 1 | high |
| 0x1c4 | int | stream status 0 ok / 1 / 2 loading / 3 not ready | 0 | med |
| 0x1c8 | int | | 0 | low |
| 0x1cc | int | restartPending; on completion calls AI slot 5 (Restart) | 0 | med |
| 0x1d0 | int | enabled; GameModule_Update runs SectionMan_Update only when 0 (!) | 1 | med |

Methods: SectionMan_Init 0x22cd40 (maps logical names from BE interface 0 `0x144d38` to
world section indices), SectionMan_LoadEvent 0x22d088 (synchronous, used by
GameModule_LoadWorld), SectionMan_RequestEvent 0x22cea8 (async), SectionMan_IsEventLoaded
0x22d278, SectionMan_Update 0x22d8d8, 0x22d3d8/0x22d478/0x22d598 request load / activate / unload
per logical section, SectionMan_RequestSky 0x22de58 / ApplySky 0x22de98 (calls
SkyBox_SetSection), 0x22df50/0x22dfd0/0x22dff0 event-switch steps.

---

## GameProcess / TypedProcess / SyncProcess / WorldObj

| Class | vt | ctor / stream ctor / dtor | Size | Added fields |
|---|---|---|---|---|
| GameProcess | 0x491f00 | 0x354648 / 0x354688 / 0x3546c8 | 0x10 | +0 ? (not set by the ctor), +4 next, +8 prev, +0xc vptr |
| TypedProcess | 0x491e80 | 0x354720 / 0x354788 / 0x354808 | 0x14 | +0x10 short type (per-type count at 0x44aff0[type]), +0x12 short (0) |
| SyncProcess | 0x491e00 | 0x354850 / 0x3548c0 / 0x354920 | 0x18 | +0x14 int syncId (counter gp+0x2a8c for type 6, else gp+0x2a88). Slot 6 IdLessEqual 0x360800, 7 IdEquals 0x360840 (used by the sorted merge) |
| WorldObj | 0x491c80 | 0x34fb00 / 0x34fb90 / 0x34fbf0 | 0x1c+ | +0x18 Instance* (back-link instance+0xc = this). Ctor registers with DangerMan via 0x2d1bf0 unless instance flag 0x100 or type 6/0x10/0x16 |

The ctor's 2nd argument is the priority. GameProcess slots: 1 dtor, 2 Update, 3 PostUpdate,
4 Render, 6/7 sort compare, 12 Save. SaveState writes type (+0x10) and syncId (+0x14) as 4
bytes each.

**ProcessList** `{int nPriorities; Bucket* buckets}`. Bucket is 0x44 bytes:
`{u32 flags (bit0 = insert into pending list); GameProcess activeHead @+4,
activeTail @+0x14, pendingHead @+0x24, pendingTail @+0x34}`. The sentinels are real
GameProcess objects with vt 0x491f00. Live contents (Snow Jam):

| Pri | flags | Objects (vt -> class) |
|---|---|---|
| 1 | 1 | 158 world objects: 77 Hide (0x491800), 36 Object (0x490e80), 20 AnimObject (0x490b10), 15 Flag (0x48fc10), 8 Particle (0x48ee60), 1 AnimTeeter (0x4908f8), 1 RestoreNode (0x491680) |
| 2 | 0 | SnowFall, cFlagManager, cSplashManager x2 |
| 3 | 0 | VisualEffectsMan proc (0x47d070), cTransitionMan x2, cCutSceneTransitionMan |
| 4 | 0 | SSXStatsMan proc (0x47cae8) |
| 6 | 0 | AIVisualEffectsMan proc (0x47cb60) |
| 8 | 0 | 81 DeadNode (0x491b00), never ticked by GameModule_Update |
| 0, 5, 7 | 0 | empty |

---

## WorldLightMan (0x32a0 "WorldLightMan"; ctor 0x2f5180, dtor 0x2f5230; no vtable)

Per-instance irradiance (SH) lighting cache plus a real-time-light overlap cache.

| Off | Type | Meaning | Live | Conf |
|---|---|---|---|---|
| 0x0000 | Nash* | from GameModule+0x20 | 0x5b5b00 | high |
| 0x0004 | WorldPainterQuery* | `WorldPainterQuery_Create(world->painter, gp+0x24a8)`; vt slot 2 queries painter regions (Lighting) at a position | 0x542ec0 | high |
| 0x0010 | OverlapRegion[8] (0x40 each) | `{int key (-1 free); int lastFrame; ...; vec4 min @+0x10; vec4 max @+0x20; RTLOverlapList* @+0x30}`. Box = pos +/- 1000. List alloc 0xa14 "RTLOverlapList", filled by 0x331450 | [0] = {0x7708, 0xf3a, ..}, list 0xc2e410 | high |
| 0x0210 | InstLight[32] (0x180 each) | `{u32 instanceHandle; int lastFrame; ..; vec4 sh[10] @+0x10 (lit result); vec4 pos @+0xb0; OverlapRegion* @+0xc0; vec4 baseSh[10] @+0xd0; InstLight* hashNext @+0x170; hashPrev @+0x174}` | [0] = {0x7708, 0xf6d, sh0 = (0.50, 0.52, 0.61)} | high |
| 0x3210 | InstLight*[32] | hash buckets on `(handle>>8)&0x1f` | | high |
| 0x3290 | u32 | used InstLight count (<= 32, then LRU evict 0x2f59d0) | | high |

WorldLightMan_GetInstanceLighting 0x2f5400 (wlm, Instance*, moving) uses instance+0x78
handle and +0x40 position (or WorldObj matrix 0x34fed8 when instance+0xc != 0). It
re-samples when the instance has moved more than 10 units (and re-queries the painter
past 500). Base SH comes from the Irradiance table at 0x5047f8 (`Irradiance_Load` in
GameModule_Load), or 0x2ee2f8 when gp+0xa74 != 0. It then adds the up to 4 strongest
type-6 (point) lights from the region's overlap list (0x2f5b68, 0x38a6a8).

---

## SkyBox (0x18 "SkyBox"; ctor 0x353ac0, dtor 0x353ae8; vt 0x48f008)

It has the GameProcess layout but the ctor never registers it in the process list (+4/+8 = 0).
Slots: 1 dtor, 4 SkyBox_Render 0x353b10, others `jr ra` stubs 0x360720+.

| Off | Meaning | Live | Conf |
|---|---|---|---|
| 0x0c | vptr | 0x48f008 | high |
| 0x10 | world section index of the active sky (-1 none) | 0xa | high |
| 0x14 | first type-3 resource (model) of that section, from the section's table | 0xbb22b0 | high |

---

## InstancesMan (0x10 "InstancesMan"; ctor 0x351030)

`{int param0 (0x80); int capacity (0x40); Instance* freeHead; Instance* pool}`. The pool
holds capacity x 0xb0 ("Instance"). Ctor init per record: +0x78 = -1 (handle),
+0x7c = 0, +0x7d = 0xff, +0x7e = -1. Free list through +0. Live {0x80, 0x40, 0xbab280,
0xba8b50}. The same 0xb0 "Instance" layout is used by WorldObj+0x18 and WorldLightMan:
+8 flags (0x100 = no DangerMan, bit 1 = dead), +0xc WorldObj*, +0x40 position,
+0x78 handle. Methods 0x351120..0x351660 not analysed. Med.

---

## OcclusionMan (0x114 "OcclusionMan"; ctor 0x242500, dtor 0x242518)

| Off | Meaning | Live | Conf |
|---|---|---|---|
| 0x000 | Occluder* list head | 0x12fd820 | high |
| 0x004 | occluder count | 0x13 | high |
| 0x008..0x107 | per-frame scratch (camera-space occluder data, BuildVisible 0x2425c0) | | med |
| 0x108 | int, BuildVisible clears it | 2 | med |
| 0x10c | int visible count A (cleared at frame start) | 2 | med |
| 0x110 | int visible count B | 3 | med |

Occluders are added from the chunk resolver (0x3ab410 -> OcclusionMan_AddOccluder) when
sections stream in. Occluder record: +0 bounding sphere, +0x10 4 corner vec4s, +0x50 plane,
+0xa0 AABB min/max, +0xb8 enabled, +0xbc next, +0xc0 prev. High for the list links, med
for the geometry.

---

## SurfPropCache (GameModule+0x44; 19 x 0xb0, alloc "SurfPropCache")

Per-surface-type physics/FX parameters, indexed by `rider+0x438` (current surface),
`[...]+0x90`, etc. Defaults come from SurfPropCache_InitDefaults (0x2c2f18). Live values
are later overwritten from world data. Readers include cBoardSparkFX_Update 0x2dabc8 (+0x14/+0x18),
0x105d98 and 0x114db8 (+0x44 flag). Live row 0: 1300, 0, 3.5, 45, 2.74, 0.5, 2.5, 5.0, 51,
2, 1.1, 0.1. Field meanings are unknown (low). Rows 14..18 share one parameter set, and
rows 4 and 11 match.

---

## Brief: other world-side objects

- **cWorldPainterMan** (8, ctor 0x2bb130): `{int nFiles (= nSections, 0x31); File* files}`.
  Methods AllocFiles/LoadFile/SetFileData/Update 0x2bb2b8. tWPIGD_* region data types:
  MusicTrigger, Mix, Ambience, Speech, Camera, Fog, LightGlow, ScreenTint, SkyBox, Sun,
  Surface, Lighting, Weather, Danger (ctor/Blend/Save/Load per type, 0x2bc6f8..0x2c0308).
- **cWScriptCache** (0x10, ctor 0x3ac7c8): `{World*; Entry(0xc)* perSection; int n (0x31); ?}`.
- **SnowFall** (0x60, ctor 0x2e5920, vt 0x487fd8, prio 2): +0x10..+0x24 six box ptrs, +0x40
  camera position vec4 (Update 0x2e5da0, Render 0x2e6008).
- **cSplashManager** (0x1030, ctor 0x2f3550, vt 0x488230, prio 2, x2 per view).
- **cTransitionMan** (0x84, ctor 0x2e4228, vt 0x488158, prio 3, x2): +0x10 = 2, +0x14 = 7/8,
  +0x1c vt 0x488100 sub-object, +0x20 ptr, +0x24 -1, +0x28 9 (low).
- **GameViewMan** (0x38, ctor 0x22e3b8): +0 ?, +4 GameCamera*[2] (GameCamera_SetTargetRider
  on +4), +0xc 4, +0x10 view count, +0x14 current view index, +0x20/+0x24 512.0/448.0
  (viewport). Live {1, 0x1597fd0, 0, 4, 1, 0, .., 512, 448}. Med.

## Open questions

- GameModule state-object names (ids 1..16) and the mismatch between the live +0x200
  pointer and the offset table.
- "Nash" (0xa8) object semantics.
- SectionStreamer +0x220 bounds hold unexpected live bytes.
- SurfPropCache field meanings. Who loads the live values (probably tWPIGD_Surface or a
  world file)?
- AI manager fields 0x08..0x24, 0x60..0x74, 0x88.
- GameModule+0x80 and +0x1c.
- SectionMan +0x1d0 polarity: Update runs SectionMan_Update only when +0x1d0 == 0, yet the
  live value is 1 in-race.

## Renames

Applied through `mcp__ghidra__rename_function_by_address` (all were `FUN_*` except
`SkyBox_v01`):

| Address | Old | New | Conf | Evidence |
|---|---|---|---|---|
| 0x231250 | FUN_00231250 | GameModule_RequestState | high | writes +0x210/+0x21c, optional immediate ProcessStateChange |
| 0x231278 | FUN_00231278 | GameModule_RequestOverlayState | high | writes +0x208 |
| 0x2312d8 | FUN_002312d8 | GameModule_IsOverlayStateDone | med | +0x208 == 0 and overlay slot 8 |
| 0x230f40 | FUN_00230f40 | GameModule_ProcessStateChange | high | switch id -> embedded state, Exit/Enter/Update slots |
| 0x22f6b0 | FUN_0022f6b0 | GameModule_LoadWorld | high | `data/worlds/` .big, World load, SkyBox, SectionMan, SurfPropCache, cWorldSound, fog man |
| 0x22c830 | FUN_0022c830 | GameModule_RenderUI | med | per-view render priority 3, UIEngine_Render, MenuSys_Draw, LoadIndicator render |
| 0x22cd18 | FUN_0022cd18 | SectionMan_dtor | high | deleting dtor next to the ctor |
| 0x22d088 | FUN_0022d088 | SectionMan_LoadEvent | med | event table 0x442480, sets +0x1c0 = 1 |
| 0x22cea8 | FUN_0022cea8 | SectionMan_RequestEvent | med | async twin of LoadEvent |
| 0x22d278 | FUN_0022d278 | SectionMan_IsEventLoaded | med | polled in LoadWorld |
| 0x22d8d8 | FUN_0022d8d8 | SectionMan_Update | high | called every frame from GameModule_Update and the LoadWorld wait loop |
| 0x22e228 | FUN_0022e228 | SectionMan_InitNodePool | med | builds the 50-node free list |
| 0x22e298 | FUN_0022e298 | SectionMan_QueueSection | med | list +0x19c, count +0x1a0 |
| 0x22e348 | FUN_0022e348 | SectionMan_NextQueued | med | iterator |
| 0x22e360 | FUN_0022e360 | SectionMan_DequeueSection | med | unlink + count-- |
| 0x22de58 | FUN_0022de58 | SectionMan_RequestSky | med | +0x1ac/+0x1a8 |
| 0x22de98 | FUN_0022de98 | SectionMan_ApplySky | med | unloads other skies 0x2c..0x30, SkyBox_SetSection |
| 0x354f98 | FUN_00354f98 | ProcessList_UpdatePriority | high | calls slot 2 on every process in a bucket |
| 0x355028 | FUN_00355028 | ProcessList_PostUpdatePriority | high | slot 3 |
| 0x3550a0 | FUN_003550a0 | ProcessList_RenderPriority | high | slot 4 |
| 0x354c98 | FUN_00354c98 | ProcessList_MergePending | med | sorted merge of pending list with slots 6/7 |
| 0x355260 | FUN_00355260 | ProcessList_SetSortedInsert | med | sets bucket flag bit 0 |
| 0x353ae8 | SkyBox_v01 | SkyBox_dtor | high | slot 1, GameProcess_dtor |
| 0x353cf0 | FUN_00353cf0 | SkyBox_SetSection | high | writes +0x10/+0x14 from World section table |
| 0x353d98 | FUN_00353d98 | SkyBox_Clear | high | +0x14 = 0, +0x10 = -1 |
| 0x2d1bd8 | FUN_002d1bd8 | World_GetInstance | high | returns gp+0x16c8 (set in World_ctor) |
| 0x2f53b0 | FUN_002f53b0 | WorldLightMan_FreeOverlapLists | high | frees the 8 RTLOverlapLists; called from the dtor |
| 0x2f5400 | FUN_002f5400 | WorldLightMan_GetInstanceLighting | med | instance cache + SH + point lights |
| 0x2f59d0 | FUN_002f59d0 | WorldLightMan_EvictOldestInstance | med | min lastFrame over 32, hash unlink |
| 0x2f5a70 | FUN_002f5a70 | WorldLightMan_GetOverlapRegion | med | free or oldest of 8 regions |
| 0x2f5998 | FUN_002f5998 | WorldLightMan_ResetRegions | med | sets 8 region keys to -1 |
| 0x242518 | FUN_00242518 | OcclusionMan_dtor | high | deleting dtor after the ctor |
| 0x242540 | FUN_00242540 | OcclusionMan_AddOccluder | high | push-front on +0, count +4 |
| 0x242570 | FUN_00242570 | OcclusionMan_RemoveOccluder | high | unlink, count-- |
| 0x2425c0 | FUN_002425c0 | OcclusionMan_BuildVisible | med | per-frame camera transform, clears +0x10c/+0x110 |
| 0x2c2f18 | FUN_002c2f18 | SurfPropCache_InitDefaults | med | fills 19 x 0xb0 defaults; only caller is LoadWorld right after the "SurfPropCache" alloc |

Proposed (outside the allowed rename ranges, not applied): 0x3a6ae0 World_Load,
0x3a6928 World_Update, 0x3a6800 World_ResetSections, 0x3a6740 World_dtor, 0x3a7e38
cWorldCache_dtor, 0x3a7fc0 cWorldCache_Load, 0x3a8290 cWorldCache_Update, 0x3a8668
cWorldCache_UpdateHullLRU, 0x3a88a8 cWorldCache_FlushHullPages, 0x3a8a90
cWorldCache_AllocResource, 0x3a8cd0 cWorldCache_ResolveResource, 0x3a8e20
cWorldCache_ReleaseResource, 0x3a9000 SectionStreamer_Bind, 0x3a9258
SectionStreamer_Update, 0x3a7598 WorldBlockPool_Init, 0x3a7878 SectionAllocator_Alloc,
0x3aad98 cPS2ChunkResolve_Create, 0x22b008 (create function) GameModule_Render.
