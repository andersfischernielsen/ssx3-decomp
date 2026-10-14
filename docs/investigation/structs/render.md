# Render submission structures

How the PS2 renderer turns draw calls into DMA chains. The chains are built in main
memory, batched by a deferred-draw manager, and kicked by a dedicated thread.

Sources: Ghidra decompilation (HTTP backend), the full decompile cache
(`naming2/dec`), and live PINE reads from PCSX2. PCSX2 was paused in a Snow Jam
race: the frame had just been flipped, buffer 0 was waiting for vblank and buffer 1
was queued.

Confidence: **H** means the code reads or writes the field in an unambiguous way,
or a live value confirms it. **M** means it is inferred from use. **L** is a guess.

## Overview: one frame

```
game Render()  ->  PS2GraphicsMan virtual draw slots (71..100, 53, 55, ...)
                   each one: SprStream_BeginBlock(spr, gm+0x5a00 cursor)        build a VIF packet in scratchpad
                             SprStream_EndBlockAligned -> DMA SPR->VifBuff       packet lands in this frame's VifBuff half
                             add a DeferMan record {render state, pkt start, pkt end, matrix idx}
EndFrame (slot 20, 0x377a10)
   PS2GraphicsMan_RenderFrame(DeferMan, &vifCursor, &vif2Head, &gifCursor, &gif2Head)   [really a DeferMan method]
       DeferMan_BuildBuckets   merge records with equal state+textures into buckets; chain consecutive packets with NEXT tags
       DeferMan_SortBuckets    4-pass byte radix sort on the 32-bit bucket key
       emit VIF chain at the cursor:  per bucket  DeferMan_EmitStateChange (A+D GS regs via DIRECT)
                                                  TexMan_BindTextures (GIF upload packets into GifBuff + TEX0 in VIF)
                                                  CALL tag -> packet start, RET written at packet end
       layer <= 6 done      -> per-view FUN_0036ac00 (effects that need the depth buffer / VRAM scratch)
       layer <= 9 done      -> END tag; the second ("phase 2") chain starts; per-view FUN_0036c790 / FUN_003904a0
       remaining layers     -> phase 2 chain, END
   WaitSema(mutex); publish {vif1 tadr, gif tadr} x 2 phases into the queue slot; buffer state = 2 (queued)
   SignalSema(kick)   -> Renderer_DmaKickThread
   swap VifBuff/GifBuff halves, DeferMan_ResetFrame, TexMan per-frame hook
Renderer_DmaKickThread (0x382760, prio 5)
   state 0 & queued:  wait for DMA/GS idle, VIF1 FIFO <- MSKPATH3 1, GIF_MODE=4,
                      GIF TADR=phase1 gif, CHCR 0x105; VIF1 TADR=phase1 vif, CHCR 0x185      -> state 1
   VIF1 DMA end IRQ (PS2GraphicsMan_OnVif1DmaEnd): 1 -> 2, signal kick
   state 2:  call gm+0x6d0c(gm+0x6d10) (lens-flare hook), MSKPATH3 1, kick phase 2       -> state 3
   VIF1 DMA end IRQ: 3 -> 4
   VBlankStart IRQ (PS2GraphicsMan_OnVBlankStart): after gm+0x5ab8 vblanks, 4 -> 5, signal kick
   state 5:  buffer state = 0 (free), Renderer_SetDisplayRegs(flip)                        -> state 0
```

The GIF channel carries only texture uploads (PATH3, one packet per texture). The
VIF1 chain unmasks PATH3 just before the draws that need each texture; see
`race-textures.md`. Both chains run in two phases. Between the phases the CPU
callback at gm+0x6d0c runs (live: 0x2e3110, the LensFX/flare code; set through
slot 117 by `LensFXBase_ctor`).

---

## PS2GraphicsMan

| | |
|---|---|
| Size | 0x75e0 (`EA_MemAllocNamed(0x75e0, "PS2GraphicsMan")` in `PS2GraphicsMan_Create` 0x375a08) |
| Base | `GraphicsMan` (ctor 0x3691f8, dtor 0x3695d8, vtable 0x493950, mostly `__pure_virtual` 0x411530) |
| vptr | at **+0x10d8** (not +0). The vtable is 0x493260, with 125 slots (0..124) |
| ctor / dtor | `PS2GraphicsMan_ctor` 0x395288 (only stores the vptr), `PS2GraphicsMan_dtor` 0x395268 |
| Init | slot 2 = `Renderer_Init` 0x375a40, called from `SSXApp_Init` |
| Singleton | `gp+0x2a90` = **0x4a5b80** (set in `GraphicsMan_ctor`) and `gp-0x854` = **0x4a289c** (`iGpfffff7ac`). Both live = **0x61ba60** |

`iGpfffff7ac` / 0x4a289c is this object. Every game-side `(*(vt@+0x10d8)+8k+4)` call is a PS2GraphicsMan virtual call.

### Fields

| Off | Type | Name | Meaning / evidence | Live | Conf |
|---|---|---|---|---|---|
| +0x000 | float[2] | screenOffset | display position x/y. Slot 12 sets it from ints, slots 13/14 read it; `Renderer_SetDisplayRegs` builds DISPLAY1 DX/DY from it (`*p*8+641`, `50-2*p[1]`) | 0,0 | H |
| +0x008 | int | ? | zeroed in ctor | 0 | L |
| +0x00c..+0xe83 | | (unknown GraphicsMan base data) | ctor sets +0x26c=0, +0x270=1, +0x274=0 | | L |
| +0xe84 | RenderState* | curState | top of the render-state stack. Callers push inline (`+0xe84 += 0x14`, copy) and pop (`-= 0x14`); 43 functions pop | 0x61c8e8 | H |
| +0xe88 | RenderState[4] | stateStack | 4 x 0x14. Entry 0 is reset to the default 0x501420 every EndFrame | see RenderState | H |
| +0xed8 | u32[4] | colorMask | set by slot 63 (`~mask`); slot 65 returns its address | 0 | M |
| +0xee8 | short[..] | ? | ctor sets them to 0x7fff/-1 | | L |
| +0xf44 | int | forceDisplayFlip | when !=0, `SetDisplayRegs` skips the parity pick once and clears it | 0 | M |
| +0xf50 | byte[0x158] | ? | memset -1 in ctor | | L |
| +0x10d8 | void* | vptr | 0x493260 | 0x493260 | H |
| +0x10dc | int | viewDepth | index into viewStack (PushView/PopView) | 0 | H |
| +0x10e0 | Matrix4[12] | matrixStack | 0x40 each. Slots 31/32 push/pop | | H |
| +0x13e0 | int | matrixDepth | | 0 | H |
| +0x13e4 | Matrix4* | curMatrix | top of matrixStack (world matrix); init = +0x10e0 | 0x61cb40 | H |
| +0x13e8 | void* | spad | 0x70000000 scratchpad base | 0x70000000 | H |
| +0x13ec | int | numLights | slot 66 clears it, slot 67 appends | | M |
| +0x13f0 | Light[?] x 0xa0 | lights | slot 67: qword color @+0, matrix @+0x10, matrix @+0x50, float @+0x90, int @+0x94 | | M |
| +0x18f0 | DeferMan* | defer | `EA_MemAllocNamed(0x6b100,"DEFER MAN")` | 0x8095f0 | H |
| +0x18f4 | TexMan* | texMan | `EA_MemAllocNamed(0x6740,"TEX MAN")` | 0x874700 | H |
| +0x18f8 | int[2000] | texLoaded | per-texture "loaded by graphics man" flags. Slot 120 frees every nonzero entry | | M |
| +0x3838 | int[2000] | texLoadedSaved | snapshot (slot 121 copies +0x18f8 here; ResetTextureSlots clears it) | | M |
| +0x5780 | Matrix4 | world | copy of *curMatrix (UpdateViewMatrices) | | H |
| +0x57c0 | Matrix4 | view | world->view | | H |
| +0x5800 | Matrix4 | worldView | view*world | | H |
| +0x5840 | vec4 x2 | clipScale/clipOffset? | saved in the view stack | (1024,1024,-8.4e6),(2048,2048,8.4e6) | L |
| +0x5860 | Matrix4 | viewProj | world->clip without the model matrix | | H |
| +0x58a0 | Matrix4 | worldClip | viewProj*world (slot 43 returns it) | | H |
| +0x58e0 | vec4 x2 | ? | saved in the view stack | | L |
| +0x5900 | short[4] | scissor | x,y,w,h (`PS2GraphicsMan_SetScissorClamped`); pushed to `DeferMan_SetScissor` | 0,0,512,448 | H |
| +0x5908 | float | fovOrAspect | view-stack copy | 0.7539 | M |
| +0x590c | float | near/fogNear? | | 29999 | L |
| +0x5910 | float | far | | 40000 | M |
| +0x5920 | vec4 | ? | view-stack copy | (1023,0,0,0.0256) | L |
| +0x5930 | Matrix4[n] | viewMatrices | per-view screen matrix (slot 116 copies +0x6af0 here and sets +0x59c0=n+1) | | M |
| +0x59c0 | int | numViews | loop count for the per-view passes in RenderFrame (split screen) | 1 | H |
| +0x59c4 | int | numDisplayModes | 8 | 8 | H |
| +0x59c8 | int | curDisplayMode | -1 at init. SelectDisplayMode skips the call if equal | 0 | H |
| +0x59cc | DisplayMode* | displayModes | table 0x44b440, 0xa4 per entry | 0x44b440 | H |
| +0x59d0 | int | dither | written to DTHE (0x45) in SetDisplayRegs; set to 1 for 16-bit fb | 0 | H |
| +0x59e0 | u64 | dimx | DIMX (0x44) | 0x2637405137265140 | H |
| +0x59e8 | int | blitToDisplay | !=0: each flip copies the draw buffer into the display buffer with a sprite strip pass | 1 | H |
| +0x59ec | int[4] | brightnessRGBA | per-channel blit colour (scaled by +0x59fc, clamped to 255) | 0x80,0x80,0x80,0x80 | H |
| +0x59fc | float | brightness | | 1.0 | H |
| +0x5a00 | u32 | vifCursor | VifBuff write pointer of the frame being built. Draw slots append here; RenderFrame emits the chain here | 0x623080 | H |
| +0x5a04 | u32 | vifHalfStart | | 0x623080 | H |
| +0x5a08 | void* | vifBuffRaw | `operator_new_named(0x199940,"VifBuff")` | 0x623050 | H |
| +0x5a0c | u32 | vifBuffBase | raw rounded up to 128 bytes. Halves are base + i*0xccc80 | 0x623080 | H |
| +0x5a10 | int | buildIndex | half being built (0/1), toggled in EndFrame | 0 | H |
| +0x5a14 | u32 | gifCursor | GifBuff write pointer (texture upload packets) | 0x7bca00 | H |
| +0x5a18 | u32 | gifHalfStart | | 0x7bca00 | H |
| +0x5a1c | void* | gifBuffRaw | `operator_new_named(0x4cc40,"GifBuff")` | 0x7bc9a0 | H |
| +0x5a20 | u32 | gifBuffBase | halves are base + i*0x26600 | 0x7bca00 | H |
| +0x5a24 | int | gifBuildIndex | | 0 | H |
| +0x5a30 | int | fbWidth | (Renderer_SetDisplayMode param 4) | 512 | H |
| +0x5a34 | int | fbHeight | | 448 | H |
| +0x5a38 | int | displayPsm | display buffer PSM (DISPFB1) | 1 (CT24) | H |
| +0x5a3c | int | drawPsm | FRAME_1 PSM | 0 (CT32) | H |
| +0x5a40 | int | zPsm | ZBUF PSM | 0x31 (Z24) | H |
| +0x5a44 | short | interlace | `sceGsResetGraph` / SMODE2 INT | 1 | H |
| +0x5a46 | short | ffmd | field/frame mode | 0 | H |
| +0x5a48 | short | omode | 2 = NTSC (0x50 for progressive) | 2 | H |
| +0x5a4c | int | progressive | !=0: omode 0x50, EATimer mode 3. Slot 9 returns it | 0 | H |
| +0x5a50 | int | fbw | fbWidth/64 | 8 | H |
| +0x5a54 | int | dispWidth | | 512 | H |
| +0x5a58 | int | dispHeight | | 448 | H |
| +0x5a5c | int | dispFbw | | 8 | H |
| +0x5a60 | int | magv | | 0 | H |
| +0x5a64 | int | magh | from width (640->3, 512->4, 384->6, 320->7, 256->9) | 4 | H |
| +0x5a68 | int | displayPages | 8 KB pages | 0x70 | H |
| +0x5a6c | int | drawPages | | 0x70 | H |
| +0x5a70 | int | zPages | | 0x70 | H |
| +0x5a74 | int | vsyncFlipCount | incremented per flip; parity picks the front buffer; slot 114 returns it | 0xfad | H |
| +0x5a78 | int | fbp0 | VRAM page of the draw buffer (`TexMan_AllocFrameBuffer`) | 0 | H |
| +0x5a7c | int | fbp1 | display buffer page | 0x70 | H |
| +0x5a80 | int | zbp | Z buffer page | 0xe0 | H |
| +0x5a84 | int | drawFbp | FRAME_1 FBP this frame | 0 | H |
| +0x5a88 | int | dispFbp | DISPFB1 FBP this frame | 0x70 | H |
| +0x5a8c | int | kickState | 0 idle, 1 phase-1 DMA, 2 phase-1 done, 3 phase-2 DMA, 4 waiting vblank, 5 flip pending | 4 | H |
| +0x5a90 | int[2] | bufState | per VifBuff/GifBuff half: 0 free, 1 building (AcquireFrameBuffer), 2 queued, 3 in flight/displayed | 3,2 | H |
| +0x5a98 | int | inFlightIndex | | 0 | H |
| +0x5a9c | u32 | inFlightVif2 | phase-2 VIF1 TADR | 0x65c440 | H |
| +0x5aa0 | u32 | inFlightGif2 | phase-2 GIF TADR | 0x7c3d10 | H |
| +0x5aa4 | int | queuedIndex | | 1 | H |
| +0x5aa8 | u32 | queuedVif1 | phase-1 VIF1 TADR (= vifCursor before RenderFrame) | 0x720610 | H |
| +0x5aac | u32 | queuedVif2 | | 0x728c50 | H |
| +0x5ab0 | u32 | queuedGif1 | | 0x7e3000 | H |
| +0x5ab4 | u32 | queuedGif2 | | 0x7ea310 | H |
| +0x5ab8 | int | vblanksPerFlip | | 1 | H |
| +0x5abc | int | vblankCounter | | 0 | H |
| +0x5ac0 | int | intcHandlerId | INTC 2 (VBlankStart) -> 0x3825c0 | 5 | H |
| +0x5ac4 | int | dmacHandlerId | DMAC 1 (VIF1) -> 0x382650 | 6 | H |
| +0x5ac8 | int | kickSema | (max 0x10, init 0) | 0x15 | H |
| +0x5acc | int | queueMutex | (1,1) | 0x16 | H |
| +0x5ad0 | int | kickThreadId | entry `Renderer_DmaKickThreadEntry` 0x382740, prio 5 | 7 | H |
| +0x5ae0 | byte[0x1000] | kickThreadStack | | | H |
| +0x6ae0 | float | ? | used in the SetDisplayRegs blit RGBAQ | | L |
| +0x6af0 | Matrix4 | screenMatrix | clip -> GS 12.4 fixed-point coords (scale 12.8/14.93, offset 28672/29184) | | H |
| +0x6b30 | Matrix4 | screenWorld | screenMatrix*curMatrix | | H |
| +0x6b70 | vec4 | screenOffset4 | | (28672,29184,1.68e7,1) | M |
| +0x6b80 | vec4 | guardBand? | | (36864,36352,0,1) | L |
| +0x6b90 | int | matricesValid | 0 = dirty; draw slots call `PS2GraphicsMan_UpdateViewMatrices` when 0 | 0 | H |
| +0x6b94 | int | fogMode | slot 40; slot 42 returns &+0x6b94 | 0 | M |
| +0x6b98..+0x6ba4 | float | fogParams | slot 41; also clamp the viewport y range in SetViewport | 0,1,1,1 | M |
| +0x6bb0 | qword[10] | lighting | slot 69 copies 0xa0 bytes; slot 70 returns the address; slot 124 uploads it | | M |
| +0x6c50..+0x6c60 | int | ? | cleared in Init | | L |
| +0x6c64 | {..}[n] x 0x20 | perViewFx | used by FUN_0036ac00 | | L |
| +0x6d0c | fn* | phase2Callback | called by the kick thread before phase 2 (slot 117 sets it) | 0x2e3110 | H |
| +0x6d10 | void* | phase2CallbackArg | | 0x1467070 | H |
| +0x6d14 | int | ? | slot 98 sets it | 0 | L |
| +0x6d20 | ViewState[n] x 0x230 | viewStack | see below | | H |
| +0x6d3c | int | ? | cleared in Init | 0 | L |

### ViewState (0x230, `PS2GraphicsMan_PushView` 0x3765b8 / `PopView` 0x376768)

| Off | Content | Conf |
|---|---|---|
| +0x00 | float x,y,w,h viewport (`PS2GraphicsMan_SetViewport`). Live entry 0: 0,0,512,448 | H |
| +0x10 | float near?, +0x14 float (1.0), +0x18 float fov (0.754). Slots 29/30 read them | M |
| +0x20 | int projection type (1 = perspective) | M |
| +0x24/+0x28/+0x2c | copies of gm+0x590c/+0x5908/+0x5910 | H |
| +0x30/+0x40/+0x50 | gm+0x6b70, +0x6b80, +0x5920 | H |
| +0x60 | screenMatrix (+0x6af0) | H |
| +0xa0/+0xe0/+0x120 | world, view, worldView | H |
| +0x160/+0x170 | gm+0x5840/+0x5850 | H |
| +0x180/+0x1c0 | viewProj, worldClip | H |
| +0x200/+0x210 | gm+0x58e0/+0x58f0 | H |
| +0x220 | scissor short[4] | H |
| +0x228 | DeferMan scissor index (+0x69cc4) | H |

### DisplayMode table entry (0xa4, table 0x44b440, 8 entries)

| Off | Content |
|---|---|
| +0x00 | char name[0x80], e.g. `"%dx%dx%d (%.2fk)"` (BuildDisplayModeTable sprintf) |
| +0x80..+0x98 | args to `Renderer_SetDisplayMode`: +0x88 width, +0x8c height (slots 10/11 return these), +0x90 psm, +0xa0 progressive flag (matched by FindDisplayMode) |

### RenderState (0x14 bytes, packed). Default at 0x501420 = `{0, 0xb00294, 0x80, 0, 0xffffffff}`

Decoded from `DeferMan_EmitStateChange` 0x363c20 and its GS register helpers.

| Word | Bits | Meaning | Conf |
|---|---|---|---|
| w0 | 2-3 / 4-5 | CLAMP_1 / CLAMP_2 mode (0 repeat, 1 clamp T, 2 clamp S, 3 both) | H |
| w0 | 6-9 | material / packet type (0 = no texture). Selects the TEX0/microcode setup in FUN_00364cd0; draw slots set 3 | M |
| w0 | 12-19 / 20-27 | ALPHA_1 / ALPHA_2 FIX | H |
| w1 | 0-1 | sort mode (0: key 0x3ff, 1: 0x3fe, 2: explicit, from w2 bits 10-28) (FUN_00364240) | M |
| w1 | 2-6 / 7-11 | ALPHA_1 / ALPHA_2 blend preset (22 presets, jump table 0x491fb0) | H |
| w1 | 12-19 | AREF | H |
| w1 | 20-21 | ATST (2 -> LESS, 3 -> GREATER, else ALWAYS) | H |
| w1 | 22 | ZMSK (no Z write) | H |
| w1 | 23-24 | ZTST (0 -> GREATER, 1 -> GEQUAL, else ALWAYS) | H |
| w2 | 0-4 | scissor index (DeferMan scissor table), filled in when the draw is recorded | H |
| w2 | 5-9 | layer / pass. The chain splits at layer 6 and 9 (thresholds at 0x4a4058 = {6, 9}) | H |
| w2 | 10-28 | explicit sort value (sort mode 2) | M |
| w3 | 0-29 | material parameter (texture/shader handle passed to FUN_00364cd0) | M |
| +0x10 | short tex[2] | texture ids for context 1/2 (-1 = none) -> `TexMan_BindTextures` | H |

---

## DeferMan (deferred draw list)

`EA_MemAllocNamed(0x6b100, "DEFER MAN")`, `DeferMan_ctor` 0x394d50, then `DeferMan_Init` 0x3629b8. Owner gm+0x18f0, live **0x8095f0**. There is no vtable.

| Off | Type | Name | Meaning | Live | Conf |
|---|---|---|---|---|---|
| +0x00000 | int | numRecords | max 0xa28 (2600). Draw slots check `< 0xa28` inline (29 functions) | 0 (frame just flushed) | H |
| +0x00080 | DrawRecord[2600] x 0x80 | records | written through the uncached (0x30000000) alias | | H |
| +0x51480 | int | numStates | reset each frame to +0x57484 | 3 | H |
| +0x51484 | {RenderState,+4}[1024] x 0x18 | states | unique render states | | H |
| +0x57484 | int | numBaseStates | 3 predefined states built by DeferMan_Init | 3 | H |
| +0x57488/8c/90 | ptr x3 | baseStates | | | H |
| +0x57494 | int | numBuckets | | 0 | H |
| +0x57498 | Bucket[2048] x 0x20 | buckets | | | H |
| +0x674a0 | Bucket*[256] | stateHash | hash of RenderState words 0-3 (`FUN_00394ed0`) | | H |
| +0x678a0 | Bucket*[256] | bucketHash | hash of (state, textures, matrix) (`FUN_00395000`) | | H |
| +0x67ca0 | Bucket* | lastBucket | for chaining continuation records | | H |
| +0x67ca4 | int | numSorted | | 0 | H |
| +0x67ca8 | {u32 key; Bucket*}[n] | sortList | radix-sorted by `DeferMan_SortBuckets` (4 byte histograms in scratchpad) | | H |
| +0x69ca8 | RenderState | emitState | the state last emitted into the chain | default | H |
| +0x69cbc | u32 | emitMaterial | | | M |
| +0x69cc0 | int | curMatrixIndex | recorded in DrawRecord+0x1c | -1 | H |
| +0x69cc4 | int | curScissorIndex | recorded in RenderState w2[0:4] | -1 | H |
| +0x69cc8 | int | numMatrices | dedup table at +0x69db0... (`FUN_003645b8`) | 0 | M |
| +0x6aed0 | int | numScissors | | 1 | H |
| +0x6aed4 | short[4][32] | scissors | x,y,w,h | {0,0,512,448} | H |

**DrawRecord (0x80)** is written by every draw slot right after its packet:

| Off | Content | Conf |
|---|---|---|
| +0x00 | RenderState (0x14) copy of *gm->curState (w2 bits 0-4 = current scissor index) | H |
| +0x14 | u32 packet start in VifBuff (= vifCursor before the packet) | H |
| +0x18 | u32 packet end (where the RET tag is written) | H |
| +0x1c | short matrix index (+0x69cc0) | H |
| +0x1e | short continuation flag (1 = same bucket as previous record, linked with a NEXT tag) | M |
| +0x20..+0x7f | unused / scratch (live record 0 shows only the first 0x20 bytes populated) | M |

Live example (record 0, stale from the last frame): `{0xcc, 0xc14184, 0, 0, 0xffff0119}`, start 0x6efd00, end 0x6efd80 (texture 0x119 in ctx 1).

**Bucket (0x20)**: +0 state ptr, +4 textures (short[2]), +8 matrix index, +0xc first packet start (CALL target), +0x10 last packet end (patched with RET or NEXT), +0x14 chained flag, +0x18 next-in-chain, +0x1c hash next. `DeferMan_AppendPacket` 0x362978 writes `NEXT(start)` (0x20000000) at the previous end. (H)

---

## Packet building: SprStream (scratchpad double buffer)

Global pointer `gp+0x1384` = 0x4a4474 -> object **0x504f80**. It is not heap-allocated.

| Off | Type | Meaning | Live | Conf |
|---|---|---|---|---|
| +0x00 | int | mode 1..7 (`SprStream_SetMode` 0x38f300) | 3 | H |
| +0x04 | int | number of SPR buffers (1/2/3) | 2 | H |
| +0x08 | int | next buffer to begin | 0 | H |
| +0x0c | int | next buffer to end | 0 | H |
| +0x10 | int | buffer size in qwords (mode: 1->0x400, 2->0x200, 3->0x1c0, 4->0x180, 5/7->0x100, 6->0x140) | 0x1c0 | H |
| +0x14 | u32[3] | scratchpad buffer addresses | 0x70000000, 0x70001c00 | H |
| +0x20 | u32[3] | destination main-memory address per buffer | | H |
| +0x2c | int[3] | size limit per buffer | | H |

API: `SprStream_BeginBlock(s, dst, size=-1, flags)` returns an SPR write pointer. `SprStream_EndBlock[Aligned](s, end, flags)` starts DMA SPR->dst and returns `dst+len`, which becomes the new `gm+0x5a00`. `BeginBlockUpload`, `Upload` and `Download` move data the other way. RenderFrame switches the stream to mode 5 while it emits and then restores the previous mode.

### Typical draw packet (slot 71, `0x379860`, triangle strip)

```
qw0  DMAtag CNT qwc=3n+4                 | VIF NOP, NOP
qw1  VIF  STROW/NOP..., UNPACK V4-32 num=3n+2 addr=0 FLG (0x6c008000|(3n+2)<<16)
qw2  GIFtag NLOOP=n EOP PRE PRIM=0x5c (tristrip, Gouraud, textured, alpha) / 0x4c untextured, NREG=3 REGS=0x412 (ST,RGBAQ,XYZ2)
qw3  VU header (count=3n)
...  3 qwords per vertex
qwN  VIF MSCAL 0x12b8/0x12b0 (0x14000000 | addr>>3)
```

Batches are split at 0x48 vertices and the SPR block is flushed when full. Other draw slots use other VIF UNPACK forms (`0x6c15...`, `0x6c98...`, `0x6c6e...`, `0x6c0b...`). Mesh slots (95, 99, 100) emit `CALL` tags (0x50000000) into prebuilt model packets and `UNPACK|STCYCL 0x1000101` headers.

### GS register helpers (A+D into a VIF DIRECT block, renamed)

`GsPkt_AddScissor` (0x40/0x41), `GsPkt_AddZbuf` (0x4e/0x4f), `GsPkt_AddTest` (0x47/0x48), `GsPkt_AddAlpha` (0x42/0x43), `GsPkt_AddClamp` (0x08/0x09). The finisher `0x3627a8` wraps the block as DMA CNT + VIF DIRECT (mode 1), DIRECT through `CALL` to the static 0x505a80 (mode 0), or UNPACK 0x6c0003f2 (mode 2).

Live chain head (queued frame, phase 1 at 0x720610): `CNT 1 [NOP NOP NOP FLUSHE]`, `CNT 10 [... DIRECT 9]`, then GIFtag A+D x8 (CLAMP_1, ALPHA_1, ZBUF_1, ...). Phase 2 (0x728c50) starts with `REF 0x44b200 qwc 0x22` (static setup packet). The GIF chain (0x7e3000) starts with `CNT 6` A+D BITBLTBUF... (texture upload).

---

## TexMan

`EA_MemAllocNamed(0x6740, "TEX MAN")` in Renderer_Init. The caller first stores the two heap-table pointers (+0x3f50 = +0x1f60, +0x6340 = +0x4350), then calls `TexMan_ctor` 0x366fe8. Owner gm+0x18f4, live **0x874700**. There is no vtable.

| Off | Type | Name | Meaning | Live | Conf |
|---|---|---|---|---|---|
| +0x0000 | int | numBTex | bank textures allocated by index (`TexMan_AllocBTexInfo`) | 0x99 | H |
| +0x0004 | int | numNBTex | dynamic textures (`AllocNBTexInfo`) | 0x87 | H |
| +0x0008 | TexInfo*[2000] | tex | ids 0-1499 are bank slots (caller chooses the index); 1500-1999 are a free list | 288 non-null | H |
| +0x1f48 | void** | nbFreeHead | free list threaded through tex[1500..1999] | 0x875f24 | H |
| +0x1f4c | int[2] | boundTex | texture bound to ctx 1/2 this frame (`TexMan_InvalidateBound` -> -1) | -1 | M |
| +0x1f54 | int | boundClut | | -1 | M |
| +0x1f58 | u32 | allocFlags | OR'd into the pixel-copy allocations (`| 0x5000000`); slot 45 / ResetTextureSlots set it | | H |
| +0x1f60 | VramHeap | vram | whole GS memory, bytes 0..0x400000 | | H |
| +0x4350 | VramHeap | tempVram | heap over the display-buffer area, in *word* units (base = fbp<<11, size = w*h). Used for format-9 (8-bit) textures (`TexMan_InitTempHeap`, `Renderer_SetDisplayMode`) | base 0x38000, size 0x70000 | M |

**TexInfo (0x58, tags "NBTexInfo"/"BTexInfo")**, filled by `TexMan_LoadTexture` 0x367440:

| Off | Type | Meaning | Live (0x58d300) | Conf |
|---|---|---|---|---|
| +0x00 | int | ownsPixels (1 = copied into an allocation of its own) | 0 | H |
| +0x04 | char[8] | name | "strm_tex" | H |
| +0x0c | int | format (2 = 16-bit, 8 = 24/32?, 9/10 = 8-bit pal, 0xb/0xc = 4-bit pal) | 0xc | H |
| +0x10 | int | pixel data size (all mips, each at least 256) | 0x2a00 | H |
| +0x14 | int | VRAM size (256-byte aligned) | 0x6000 | H |
| +0x18 | int | bits per pixel | 4 | H |
| +0x1c | void* | pixels | 0xbdf500 | H |
| +0x20 | void* | clut (0x400 bytes, or 0x40 for 4-bit) | 0xbe1f80 | H |
| +0x24 | int | mip count | 2 | H |
| +0x28 | int | TBP (VRAM block address, 256-byte units; -1 = not resident) | -1 | H |
| +0x2c | int | CBP (clut block; -1 = not resident) | -1 | H |
| +0x30 | int | VramHeap block index (image) | -1 | H |
| +0x34 | int | VramHeap block index (clut) | -1 | H |
| +0x38 | u64 | TEX0 (TBW, PSM, TW, TH, TCC, CLD) | 0x20000005dd408000 | H |
| +0x40 | u64 | TEX1 (LCM, MXL, MMIN/MAG, K) | 0xffffff47_00000168 | M |
| +0x48 | u64 | MIPTBP1 | | H |
| +0x50 | u64 | MIPTBP2 | | H |

Residency (`TexMan_LoadTexture` param 10): 1 = permanent VRAM block, 3 = permanent plus 0x2000 for the CLUT, 0 = streamed. `TexMan_BindTextures` 0x368660 allocates on demand per draw bucket (the `VramHeap_Alloc` arguments include the draw index, used for eviction). It emits the upload into GifBuff (`FUN_00368170` image, `FUN_003684f0` CLUT) and TEX0/MIPTBP for ctx 1/2 into the VIF chain (`FUN_00368970`).

**VramHeap (0x23f0)**, `VramHeap_Init` 0x365e40:

| Off | Meaning | Conf |
|---|---|---|
| +0x0000 | Block[0x123] x 0x1c: +0 `addr | flags` (low byte: bit0 used, 0x2 resident, 0x4 clut, 0x20 reserved), +4 size, +8 owner tex id (0x7fffffff = frame buffer), +0xc prev, +0x10 next (address order), +0x14/+0x18 free-list links | H/M |
| +0x1ff0 | Block* table (also uploaded to scratchpad for the frame: `FUN_00365f68`/`FUN_00365fa8`) | H |
| +0x1ff4 | heap size | H |
| +0x200c..+0x23c8 | free-list / size-class heads (-1) | M |
| +0x23e8 | TexInfo** (= &texMan->tex) | H |

Live main heap: block 0 = draw FB (0, 0xe0000), 1 = display FB (0xe0000), 2 = Z (0x1c0000), then texture blocks (for example 0x2a0000, size 0x400, tex 0xfa).

---

## cPS2GridMesh (if time: partial)

`PS2GraphicsMan_CreateGridMesh` (slot 76) -> `EA_MemAllocNamed(0x1cc,"cPS2GridMesh")`, `cPS2GridMesh_ctor` 0x385138. The vptr is at **+4**: 0x493208 (base 0x493938).

Slots: 1 dtor, 2 IsValid (+0x1c8), 3 SetPositions (vec3[]), 4 SetUVs (vec2[]), 5/6 per-vertex colour/normal updates (M), 7 Draw(matrix, texObj) -> 8 `cPS2GridMesh_Draw`(matrix, texId, flag), 9 SetScale(f).

| Off | Meaning | Conf |
|---|---|---|
| +0x0c | batch[32] x 0xc {vuQwords, rows, firstVert}. A batch is limited to 0xdb qwords of VU memory | H |
| +0x18c | numBatches | H |
| +0x190 / +0x192 | short width, height (vertices) | H |
| +0x194 / +0x198 | scale, 1/scale (slot 9) | H |
| +0x19c | data allocation | H |
| +0x1a0 | front buffer index (0/1); vertex writes go to the other one | H |
| +0x1a4 | flip-pending flag | H |
| +0x1a8 | strips built | M |
| +0x1ac / +0x1b0 | packet start per buffer (CALL targets) | H |
| +0x1b4 / +0x1b8 | per-row vertex pointers per buffer | H |
| +0x1bc | rows (h-1) | H |
| +0x1c0 | verts per strip (2w) | H |
| +0x1c4 | PS2GraphicsMan* | H |
| +0x1c8 | valid | H |

---

## PS2GraphicsMan vtable 0x493260 (slot k fnptr at vt+8k+4)

"calls" is the number of functions that call the slot (from the decompile cache). Names in *italics* are descriptive (med/low) and were not applied in Ghidra. Slots not listed are `jr ra` stubs.

| Slot | Impl | Name | Calls | Notes |
|---|---|---|---|---|
| 1 | 0x395268 | PS2GraphicsMan_dtor | 1 | |
| 2 | 0x375a40 | Renderer_Init (= PS2GraphicsMan_Init) | 1 | SSXApp_Init |
| 3 | 0x376268 | PS2GraphicsMan_Shutdown | 1 | removes handlers, thread, semas, Defer/TexMan |
| 4 | 0x3764c0 | PS2GraphicsMan_SelectDisplayMode(idx) | 1 | |
| 5 | 0x395328 (stub) | *GetDisplayModeIndex* | 0 | returns +0x59c8 |
| 6 | 0x376560 (stub) | *GetDisplayModeInfo(idx)* | 0 | table+idx*0xa4 |
| 7 | 0x395318 (stub) | *GetDisplayModeCount* | 0 | +0x59c4 |
| 8 | 0x376578 | PS2GraphicsMan_SetDefaultDisplayMode | 3 | slot 122 (512,448,24,32,24) |
| 9 | 0x395320 (stub) | *IsProgressive* | 1 | +0x5a4c |
| 10 / 11 | 0x395330 / 0x395350 (stubs) | *GetScreenWidth / GetScreenHeight* | 15 / 13 | mode +0x88 / +0x8c |
| 12 / 13 / 14 | 0x393fb8 / fd8 / ff0 | *SetScreenOffset / GetScreenOffsetX / Y* | 1 / 0 / 0 | +0 / +4 |
| 15 | 0x389098 | PS2GraphicsMan_SetClearColor | 13 | |
| 17 | 0x395f60 | **PS2GraphicsMan_TryAcquireFrame** | 10 | -> AcquireFrameBuffer (begin frame; 0 = both buffers busy) |
| 18 | 0x395f80 | **PS2GraphicsMan_IsIdle** | 2 | -> IsRenderIdle |
| 19 | 0x369c28 | PS2GraphicsMan_WaitForRenderIdle | 8 | |
| 20 | 0x377a10 | PS2GraphicsMan_EndFrame | 11 | |
| 21 / 22 | 0x3765b8 / 0x376768 | PS2GraphicsMan_PushView / PopView | 11 / 11 | |
| 23 | 0x376a70 | **PS2GraphicsMan_SetViewport**(x,y,w,h) | 1 | |
| 24 | 0x395370 | **PS2GraphicsMan_SetViewportRect**(float[4]) | 2 | |
| 25 / 26 | 0x376c58 / 0x377458 | SetPerspective / SetOrtho | 3 / 15 | |
| 27 | 0x377278 | *ProjectPoint/billboard scale* | 1 | uses worldClip |
| 28 / 29 / 30 | stubs | *GetViewport / GetFov / GetNearFarAspect* | | view entry |
| 31 / 32 | stubs | *PushMatrix / PopMatrix* | 16 / 15 | +0x13e4 +-0x40 |
| 33 | 0x395750 | *(matrix op)* | 0 | |
| 34 / 35 / 36 / 37 / 38 | stubs | *LoadMatrix / GetMatrix / MultMatrix / LoadIdentity (0x4ff1a0) / Translate* | 16/9/0/4/0 | all clear +0x6b90 |
| 39 | 0x395d60 | **PS2GraphicsMan_RotateAxis**(angle, axis) | 0 | |
| 40 / 41 / 42 | stubs | *SetFogMode / SetFogParams / GetFogState* | 4/3/4 | +0x6b94.. |
| 43 | 0x3953b8 | **PS2GraphicsMan_GetWorldToClipMatrix** | 8 | |
| 44 | 0x37c7c0 | PS2GraphicsMan_ResetTextureSlots | 7 | |
| 45 | 0x37c8a0 | **PS2GraphicsMan_SetTexAllocFlags** | 0 | |
| 46 | 0x37c8c0 | *LoadTextureFromShape* (SSH/FSH image header) | 17 | |
| 47 | 0x37caf8 | PS2GraphicsMan_LoadTexture | 1 | |
| 48 | 0x37d0b0 | *LoadPackedShape* ("unpack") | 2 | |
| 49 | 0x37d318 | *(TexMan 0x367d20)* | 2 | |
| 50 | 0x37cb50 | PS2GraphicsMan_FreeTexture | 22 | |
| 51 | stub | *IsTextureValid(id<2000)* | 0 | |
| 52 | 0x37d738 | PS2GraphicsMan_LockTexture | 2 | |
| 53 / 55 | 0x37d348 / 0x37d800 | *DrawTextureUpload / DrawTexturedSprite* | 1 / 1 | TexMan_Build{Upload,Sprite}Packet + record |
| 54 | 0x37d450 (stub) | *DrawClear/scissored fill* | 2 | EndFrame |
| 56 / 57 | stubs | *GetTextureWidth / Height* (TEX0 TW/TH) | 1 / 1 | |
| 58 | 0x3889f0 | ? | 1 | |
| 59 / 60 | 0x37cbe8 / 0x37cc30 | *AcquireTextureRef / ReleaseTextureRef* | 1 / 1 | |
| 61 | 0x37cc98 | *DrawTransition* | 1 | cTransitionMan_Render |
| 63 | 0x369610 | *SetColorMask* | 0 | +0xed8 |
| 65 | stub | *GetColorMask* | | |
| 66 / 67 | stubs | *ClearLights / AddLight* | 0 / 1 | +0x13ec |
| 69 / 70 | stubs | *SetLighting / GetLighting* | 3 / 0 | +0x6bb0 |
| 71 | 0x379860 | *DrawTriStrip(n, verts)* | 38 | most-used draw |
| 72 | 0x379bd0 | *DrawVectorShape* | 1 | UIVector_Render |
| 73 | 0x381ad0 | *DrawLines/Points* | 8 | |
| 75 | 0x381f10 | *(draw)* | 0 | |
| 76 | 0x37a540 | PS2GraphicsMan_CreateGridMesh | 2 | |
| 77 | 0x36a190 | *DestroyGridMesh* | 3 | |
| 78 / 79 | 0x377cf0 / 0x3781a0 | *DrawQuad / DrawBillboard* | 7 / 4 | |
| 80 / 81 | 0x378808 / 0x379028 | *DrawGlyphs / DrawGlyphsW* | 1 / 1 | Font_DrawText(W) |
| 82 / 83 / 84 | 0x380518 / 0x3807a0 / 0x380ce0 | *DrawParticles* (Emitter / DynamicEmitter / mEmitter) | 2/1/1 | |
| 85 / 86 | 0x381310 / 0x3816f0 | *DrawTrail* variants | 1 / 1 | |
| 87 | 0x382170 | *DrawStreaks* | 1 | |
| 89 | 0x386e78 | *DrawShadow/decal* | 1 | |
| 90 | 0x3883b8 | *DrawBoardWake* | 1 | cBoardWakeFX |
| 91 / 92 / 93 | 0x37dbe8 / 0x37dd20 / stub | *IsSphereVisible / ProjectToScreen / ClipTest* | 3/3/3 | |
| 94 | 0x36a2c0 | *(model helper)* | 1 | |
| 95 | 0x37e238 | *DrawModel* | 5 | MeshAnim/SkyBox/ModObj |
| 96 | 0x369cb0 | *DrawModelSimple* -> 95 | 5 | |
| 97 | stub 0x380380 | *(model setup)* | 3 | |
| 99 / 100 | 0x37a610 / 0x37b548 | *DrawSkinnedModel / DrawSkinnedModelShadow* | 1 / 1 | rider |
| 101 | stub 0x37ba50 | *DrawMatrixPacket* | 1 | |
| 109 / 110 / 111 / 112 | 0x3866f0 / stub / 0x386bd0 / stub | *Rider model setup / free* | | RiderModel_* |
| 113 | stub | *(GameModule_Unload hook)* | 1 | |
| 114 | stub | *GetFlipCount* (+0x5a74) | 12 | |
| 115 | 0x386640 | *UpdateLensEffects* | 1 | |
| 116 | stub 0x386688 | *AddView(i)* | 0 | +0x5930, +0x59c0 |
| 117 | stub 0x3866e0 | *SetPhase2Callback(fn,arg)* | 3 | +0x6d0c/+0x6d10 |
| 119 | 0x389118 | PS2GraphicsMan_RemoveInterruptHandlers | 1 | |
| 120 | 0x37c830 | **PS2GraphicsMan_FreeAllTextures** | 1 | |
| 121 | 0x37c808 | **PS2GraphicsMan_SnapshotTextureTable** | 1 | |
| 122 | 0x376468 | **PS2GraphicsMan_SelectMatchingDisplayMode** | 1 | |
| 123 | 0x3762f8 | **PS2GraphicsMan_FindDisplayMode** | 1 | |
| 124 | stub 0x396b40 | *EmitLightingPacket* | 1 | |

## Open questions

- `PS2GraphicsMan_RenderFrame` (0x363490) is a **DeferMan** method (DeferMan_Flush/BuildChain); it could not be renamed under the rules, so there is a decompiler comment.
- RenderState w0 bits 6-9 / w3 (material handle) need FUN_00364cd0 decoded: which VU1 microprogram and TEX0 setup each type selects.
- What the per-view passes do: FUN_0036ac00 after layer 6 (allocates a 0x400 VRAM block, probably depth readback / flare occlusion, see `flare-occlusion.md`), and FUN_0036c790/FUN_003904a0 after layer 9.
- GraphicsMan base fields +0x10..+0xe83 are unmapped.
- Draw slots 72-100 names are descriptive (from callers); the primitive/VIF formats of each are not decoded.
- VramHeap free-list layout (+0x200c..) and the eviction policy in `VramHeap_Alloc`.
- Units of the temp heap (+0x4350): word units versus byte units in the main heap. This is inferred from `Renderer_SetDisplayMode` (`page<<11`) and from `TexMan_BindTextures` shifting format-9 TBP by 2.

## Renames

All are high confidence and were applied with `rename_function_by_address`. Every old name was `FUN_<addr>`.

| Address | New name | Evidence |
|---|---|---|
| 0x382740 | Renderer_DmaKickThreadEntry | thread entry passed to CreateThread in Renderer_Init; only calls Renderer_DmaKickThread |
| 0x395f60 | PS2GraphicsMan_TryAcquireFrame | vt slot 17, wrapper of AcquireFrameBuffer, called at start of every module Render |
| 0x395f80 | PS2GraphicsMan_IsIdle | vt slot 18, wrapper of IsRenderIdle |
| 0x376a70 | PS2GraphicsMan_SetViewport | slot 23: stores x,y,w,h in viewStack[depth], clamps scissor, DeferMan_SetScissor |
| 0x395370 | PS2GraphicsMan_SetViewportRect | slot 24: unpacks float[4] into slot 23 |
| 0x376938 | PS2GraphicsMan_SetScissorClamped | clamps the rect to screen w/h, writes +0x5900 short[4] |
| 0x3953b8 | PS2GraphicsMan_GetWorldToClipMatrix | slot 43: updates matrices, copies +0x58a0 to out |
| 0x395d60 | PS2GraphicsMan_RotateAxis | slot 39: SinCos + axis-angle rotation applied to *curMatrix |
| 0x37c8a0 | PS2GraphicsMan_SetTexAllocFlags | slot 45: TexMan +0x1f58 |
| 0x37c830 | PS2GraphicsMan_FreeAllTextures | slot 120: FreeTexture(i) for every flagged i<2000 |
| 0x37c808 | PS2GraphicsMan_SnapshotTextureTable | slot 121: memcpy +0x18f8 -> +0x3838, 8000 bytes |
| 0x376468 | PS2GraphicsMan_SelectMatchingDisplayMode | slot 122: slot 123 then slot 4 |
| 0x3762f8 | PS2GraphicsMan_FindDisplayMode | slot 123: scans mode table by w,h,bpp,progressive |
| 0x3629b8 | DeferMan_Init | called once after DeferMan_ctor; builds 3 base states |
| 0x362cc8 | DeferMan_ResetFrame | EndFrame: count=0, states reset, hash tables cleared |
| 0x362de8 | DeferMan_BuildBuckets | first step of RenderFrame: hashes records into buckets |
| 0x364050 | DeferMan_SortBuckets | 4 byte-histograms + prefix sums (radix sort) of +0x67ca8 |
| 0x363c20 | DeferMan_EmitStateChange | diffs RenderState against +0x69ca8, emits SCISSOR/ZBUF/TEST/CLAMP/ALPHA |
| 0x364b88 | DeferMan_SetScissor | dedups the rect into the +0x6aed4 table, sets +0x69cc4 |
| 0x362978 | DeferMan_AppendPacket | sets bucket first/last; links with NEXT tag 0x20000000 |
| 0x3623d8 | GsPkt_AddScissor | writes regs 0x40/0x41 |
| 0x362660 | GsPkt_AddZbuf | regs 0x4e/0x4f, ZBP/PSM/ZMSK |
| 0x3626d8 | GsPkt_AddTest | regs 0x47/0x48, ATE/ATST/AREF/ZTE/ZTST |
| 0x362478 | GsPkt_AddAlpha | regs 0x42/0x43, 22-entry preset table |
| 0x3625c0 | GsPkt_AddClamp | regs 0x08/0x09 |
| 0x38f300 | SprStream_SetMode | sets nbuf/size per mode on the SprStream object |
| 0x368660 | TexMan_BindTextures | per bucket: VRAM alloc + upload for ctx1/ctx2 textures, TEX0 emit |
| 0x366fe0 | TexMan_SetAllocFlags | stores +0x1f58 |
| 0x367078 | TexMan_ResetVram | resets both VramHeaps and bound ids |
| 0x3673d8 | TexMan_AllocFrameBuffer | VramHeap_Alloc owner 0x7fffffff, returns page (>>13); used for FB0/FB1/Z |
| 0x367340 | TexMan_InitTempHeap | VramHeap_Init on +0x4350 |
| 0x396128 | TexMan_InvalidateBound | +0x1f4c/+0x1f50/+0x1f54 = -1 at frame start |

Decompiler comment added at 0x363490 (PS2GraphicsMan_RenderFrame is a DeferMan method).
