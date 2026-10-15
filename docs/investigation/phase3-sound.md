# Phase 3, sound: the SNDDRV / LIBSD ↔ EE protocol

Specification for a native sound driver that replaces the recompiled IOP modules (`iop-audio.md`) at the EE/IOP boundary (`phase3-boundaries.md` §2, cut A). Research pass 2026-10-15.

Scope: SSX 3 (SLUS_207.72), EA "SND Library EE Driver" SNDDRV.IRX v8.3.3 plus Sony LIBSD.IRX (libsd v1.05). The two IRX files are loaded at the project's fixed bases: LIBSD at 0x20000 and SNDDRV at 0x30000.

Markers:
- **V** means read in code: the IOP decompile listed below, or EE Ghidra at 127.0.0.1:8080.
- **I** means inferred.

**How the IOP side was read.** Both IRX files were extracted from the ISO and relocated the same way `irx.rs` does. Ghidra headless ran in a scratch project, not the shared one: SNDDRV's own symbol table was applied, the 12 import stubs got prototypes, and LIBSD functions were named from its export table. The result is a full C decompile at `(scratch, not committed) decomp.c`; the address→name list is `syms.txt` in the same directory. EE decompiles are in scratch `ee/*.c`.

IOP symbol addresses (base 0x30000):

| Function | Address |
|---|---|
| threadmain | 0x38f8c |
| SNDIOP_processtagbuf | 0x36fd8 |
| SNDIOP_updatevoices | 0x37d1c |
| SNDIOP_mix | 0x36b74 |
| SNDIOP_ee36_iop24 | 0x3510c |
| SNDIOP_ee24_iop24 | 0x34e04 |
| SNDIOP_init | 0x382f0 |
| SNDIOP_dispatchrpc | 0x38bf0 |
| SNDIOP_cmdhandler | 0x38ba0 |
| SNDIOP_dm* (DMA queue) | 0x34924–0x34d78 |
| SNDIOP_spusetvol | 0x376c0 |
| SNDIOP_checkspueffect | 0x378c0 |
| SNDIOP_updateiopvoices | 0x373b8 |
| MIX_* | 0x30fc8–0x31a50 |

---

## 0. Configuration SSX 3 actually uses

| Item | Value | Source |
|---|---|---|
| EE mix rate (RPC +0xC) | **36000** | `PS2SoundManager_SetOutputMode` 0x2a8d90 sets 36000/24000 in every output mode, and `SND_InitWithOutputMode` keeps the 36000/24000 pair. **V** for the code; **I** that it is the runtime value. |
| IOP mix rate (RPC +0xE) | **24000** | same |
| Output channels (RPC +0x10) | 2 (modes 0..2), 4 (mode 3, DTS) | V (SetOutputMode `uStack_d9`) |
| Aux planes (RPC +0x11) | 0 | I (never set) |
| IOP software voices (RPC +0x12) | 8 | V (`uStack_f1 = 8`) |
| SPU voices | 48 (0x30), i.e. 2 cores × 24 | V |
| IOP resample quality (cmd block +0x13) | 0x32 (> 0x18 → linear) | V (SetOutputMode `uStack_df`), I at runtime |
| Resulting IOP path | `SNDIOP_ee36_iop24` (channels < 4) | V (`SNDIOP_mix` dispatch) |
| EE block per tick | 0x180 frames × 2 planes s16 = 0x600 bytes | V (`SND_IopInit`: 0x100/0x180/0x200 for 24/36/48 k) |
| IOP block per tick | 256 frames (24000·512/48000) | V |
| Output per tick | 512 frames at 48 kHz, so one tick every 10.667 ms (**93.75 Hz**) | V |
| SPU reverb mode | 0 (off) for the whole game | I: `SND_SetFx` is only called from `SND_Shutdown`, and mode source 0x50ab08 is BSS |
| IOP effect RPC (fno 2) | never sent during play | I (same reason) |
| Voice groups | SPU (flag 8), IOP (0x100), EE (4) | V |
| Banks using IOP RAM | `zboard.bnk` (region B 0x40660 bytes, 8-bit PCM), loaded in WORLD per `BANKS.INF` | V (bank headers in AUDIO.BIG) |
| Banks using SPU RAM | `zBxsfx`, `zboardSPU`, `Mtn`, `Crowd`, `land`, `SSX3Menu`, `FE Menu` | V |

**IOP software voices are in use**, so a native driver needs §8 and not only the SPU model.

---

## 1. Transport

### 1.1 Messages

| # | Dir | Mechanism | When / rate | Payload | V/I |
|---|---|---|---|---|---|
| T1 | EE→IOP | `sceSifBindRpc(0x50b980, 'SND' 0x00534E44)` | once, in `SND_IopInit` | none | V |
| T2 | EE→IOP | RPC fno 0 (init), blocking | once | 0x14 bytes copied to 0x50ad00, layout in §1.2. No reply data (rbuf = 0) | V |
| T3 | EE→IOP | RPC fno 1 (shutdown) → `SNDIOP_restore` | `SND_IopShutdown` | none | V |
| T4 | EE→IOP | RPC fno 2 (IOP effect), **no-wait** (mode = fno==2) | `SND_IopSetEffect`. Not reached during play (I) | Preset blob, size = BE u32 at preset+4 (≤ 0x800, the IOP receive buffer 0x40ed0..0x416d0), or the 4-byte default at EE 0x44dd70. Size 4 → `MIX_restorereverb`, otherwise `MIX_initreverb(iopRate, buf)` | V |
| T5 | IOP→EE | `sceSifSendCmd(cid 1, pkt 0x20, src = ioptoee 0x3b098, dst = EE 0x50b740, 0x240)` | **every tick** after the tag buffer is processed | pkt +0xC (opt) = IOP tag receive address (0x3b3c4); +0x10 = type 0. Data = status block (§4) | V |
| T6 | EE side of T5 | `SND_SifCmdHandler` 0x3c1578 | interrupt | type 0: stores opt at 0x50c804 (= DMA destination) and `iSignalSema(0x50c7a4)`. type 2: `SND_SifDmaComplete(opt)`. Any other type is ignored | V |
| T7 | EE→IOP | `sceSifSetDma({src 0x512b80, dst *(0x50c804), size *(0x50c808), mode 0}, 1)` | once per EE tick, end of `SND_EeThread` | tag buffer (§2) | V |
| T8 | EE→IOP | SIF DMA EE→IOP staging buffer, then `sceSifSendCmd(cid 0, pkt 0x515780, 0x30, 0, 0, 0)` | per ≤ 0x1000-byte SPU upload piece, one in flight at a time | pkt +0x10 = 1 (type), +0x14 = upload id (token), +0x18 = IOP staging address (`sceSifAllocIopHeap(0x1000)` → 0x50bb8c), +0x1c = SPU byte destination, +0x20 = u16 size (rounded up to 0x40), +0x22 = u16 priority (u8) | V |
| T9 | IOP→EE | `isceSifSendCmd(cid 1, pkt 0x20, 0, 0, 0)` from the SPU DMA-end interrupt | per completed upload with token ≠ 0 | opt (+0xC) = token, +0x10 = type 2 | V |
| T10 | EE→IOP | direct SIF DMA, no command | IOP-RAM uploads (kind 0x100) | raw sample bytes to an IOP heap address (`sceSifAllocIopHeap(size+0x40)`) | V |
| T11 | IOP→EE | `sceSifSendCmd(1, crdts, 0x20, sndiopdtspcm, EE[RPC+8], (ch+aux)*0x400)`, type 3 | DTS mode only (channels > 3). The EE handler ignores type 3; only the data matters | — | V |

Handler tables:
- IOP: `sceSifSetCmdBuffer(cmdBuf, 0x10)`, handler for cid 0. **V**
- EE: `sceSifSetCmdBuffer(0x50c880, 0x10)`, handler for cid 1. **V**

### 1.2 Init RPC block (EE 0x50acc8 → IOP buffer 0x40ed0)

| Off | Type | SSX 3 value | IOP destination | Meaning |
|---|---|---|---|---|
| 0x00 | u32 | 0x00080303 | must match, otherwise abort | version 8.3.3 (V) |
| 0x04 | u32 | 0x0050b740 | `0x4172c` | EE address the status block is sent to (V) |
| 0x08 | u32 | `*(0x50c7d8)`, 0 unless DTS (I) | `0x41730` | EE address for DTS PCM (V) |
| 0x0C | u16 | 36000 | `b398` | EE rate. IOP derives `b39c = rate*512/48000` (V) |
| 0x0E | u16 | 24000 | `b39a` | IOP mixer rate. `b39e = rate*512/48000` = 256 (V) |
| 0x10 | u8 | 2 | `b31e` | output channels. 1 = mono (plane duplicated); ≥ 4 = DTS path (V) |
| 0x11 | u8 | 0 | `b320` | aux planes (V) |
| 0x12 | u8 | 8 | `b321` | number of IOP software voices; `MIX_create` (V) |
| 0x13 | u8 | 0 (forced to 0 when channels < 4) | `b31f` | "SPU fx to mixer" flag, DTS only (V) |

The EE stores the IOP version in the abort message; there is no reply. **V**

### 1.3 Tick sequence on the IOP (V)

1. LIBSD's AutoDMA (core 0, DMA ch 4) half-buffer completion interrupt calls `SNDIOP_autodmacb`, which does `iWakeupThread(dmathread)`. dmathread has priority 9.
2. dmathread sets `b322 = sceSdBlockTransStatus(0,0) >> 24` (the half now playing), then `WakeupThread(main)` when channels < 4, then sleeps.
3. threadmain (priority 0xF) wakes and drains pending 'SND' RPC requests (`sceSifGetNextRequest`/`ExecRequest`). If `b31c` (initialised) is set:
   - `SNDIOP_copytagbuf(0x3dfc4, 0x3b3c4)`;
   - `SNDIOP_processtagbuf`;
   - [DTS send];
   - wait for the previous T5 DMA (`syncsifcmd`: `sceSifDmaStat` with `DelayThread(500)`);
   - `ioptoee[0] = ioptoee[0x23c/4] = seqnum++`;
   - send T5.
4. CPU accounting: the µs spent per tick are summed; once per second, `ioptoee+4 = sum*655/100`. This is host-time and nondeterministic; **the EE never reads it**.

Subtleties:
- **Extra ticks.** The RPC queue's thread *is* threadmain, so every RPC arrival is also an extra tick when `b31c = 1`. The **first tick after init** is produced this way, by the init RPC itself. Each extra tick mixes into the same idle half again. **V**
- **Replays.** If the EE has not DMA'd a new tag buffer before the next tick, the IOP **reprocesses the old buffer**: the PCM is replayed, and voice commands are idempotent because of the caches. The receive area is never cleared; only the *copy* gets tag 6. **V**
- Before init, threadmain writes 6 (end) at 0x3b3c4 and 0xFF to `ae08`. **V**

Rate for a native driver: one tick per 512 output frames, which is exactly 3,145,728 EE cycles at 294.912 MHz. The EE consumes the ticks as `SND_Service(94)`, which converts them to the 100 Hz `SND_Tick` counter `0x50aa68` (V). The EE thread runs at priority 6, above the game's 100, so tick timing decides where `SND_Service` interleaves with game code (I).

---

## 2. Tag buffer

### 2.1 Container

The buffer lives at EE 0x512b80 (written through the uncached alias 0x20512b80), at most 0x2c00 bytes, built once by `SND_InitIopCmdBuffer` 0x3c1638 (V). The IOP receives it at 0x3b3c4 (inside `sndiop`) and copies it to 0x3dfc4 (0x2c00 bytes apart) before processing.

Every tag starts with a u32 tag id. Tags are 16-byte aligned.

| Tag | Header | Present (SSX 3) | IOP handling (`processtagbuf`) | V/I |
|---|---|---|---|---|
| 0 | +0 = 0, **+4 = seq** (EE `0x50ac18`, the `SND_Service` tick count), **+8 = total bytes**, +0xC pad | always, at offset 0 | skip 0x10 | V |
| 3 | +0 = 3, +4 = payload size (0x2a8 including pad), +8 = **command block** (0x2a0 bytes, §2.2). EE pointer `*(0x50b9b0)` = buf+0x18 | always, at 0x10 | `SNDIOP_updatevoices(tag+8)`; next = tag + 8 + size | V |
| 2 | +0 = 2, then 0xC pad, then `b39c*4` bytes EE wet buffer | only if config 0x50a92a ≠ 0 (default 0); absent | `memcpy(MIX_getwetbuffer(), data, b39c*4)` | V / I (absent) |
| 1 | +0 = 1, **+4 = u16 PCM bytes**, pad to 0x10, then PCM | always | `SNDIOP_mix` (§3). next = tag + 0x10 + bytes | V |
| 4 | +0 = 4, then 0x800 bytes DTS | only if config flags & 2 (DTS) | `ptagbufdts = tag+0x10` | V |
| 5 | +0 = 5, **+4 = seq** (same value as tag 0 +4) | always | skip 0x10 | V |
| 6 | end | always, last | stop. The copy gets `*buf = 6` | V |

**Consistency check** (`SNDIOP_copytagbuf`, V):
```
loop { if src[0]==0 { memcpy(dst, src, src[2]) } else if src[0]==6 { dst[0]=6; return }
       if dst[1] == *(u32*)(dst + dst[2] - 0x1c) { break } }   // header seq == tag-5 seq
```

**SSX 3 stereo layout** (I; derived from the V code with rates 36k/24k):

| Range | Contents |
|---|---|
| 0x000 | tag 0 (seq at 0x004, total at 0x008) |
| 0x010 | tag 3: size 0x2a8 at 0x014, cmd block 0x018..0x2b7, pad to 0x2c0 |
| 0x2c0 | tag 1: bytes = 0x600 at 0x2c4 |
| 0x2d0 | PCM plane 0, 384 × s16 (= **Right**) |
| 0x5d0 | PCM plane 1, 384 × s16 (= **Left**) |
| 0x8d0 | tag 5 (seq at 0x8d4) |
| 0x8e0 | tag 6 |
| total | **0x8f0** |

### 2.2 Command block (tag 3 payload, 0x2a0 bytes; offsets from `*(0x50b9b0)`)

| Off | Type | EE writer | IOP action (`updatevoices`, in this order) | V/I |
|---|---|---|---|---|
| 0x00 | u64 | wet (reverb-send) bits, bit v = SPU voice v (`SND_ChannelApplySend`) | on change: `setwetbusbits(lo, hi)` → VMIXEL/VMIXER: core 0 = lo & 0xFFFFFF; core 1 = lo>>24 \| hi<<8 | V |
| 0x08 | u64 | dry bits, initialised to 0xFFFFFFFFFFFF (`SND_ChannelApplyFxEnable`) | on change: `setdrybusbits` → VMIXL/VMIXR, same split | V |
| 0x10 | s8 | SPU reverb mode 0..9 (`SND_SetSpuReverbMode`: 5→1, 10→2, 20→3, 30→4, 40→5, 50→6, 120→7, 110→8, 100→9) | `checkspueffect` state machine (§6.3) | V |
| 0x11 | s8 | reverb delay (0x50ab0c) | effect-attr delay | V |
| 0x12 | s8 | reverb feedback (0x50ab10) | effect-attr feedback | V |
| 0x13 | u8 | IOP resample quality (config 0x50a929) | `MIX_setresamplequality` on change; > 0x18 → linear for resamplers created later | V |
| 0x14 | s16 | EVOL L target (= fx vol float × 32767, `SND_SetSpuFxVolume`) | ramp ±0x800 per tick, see §6.3 | V |
| 0x16 | s16 | EVOL R target | same | V |
| 0x18 | — | unused | — | I |
| 0x20 + v·8 (v 0..47) | **SPU voice record** | §2.3 | §2.3 | V |
| 0x1a0 + i·0x10 (i 0..15; 8 used) | **IOP voice record** | §8 | §8 | V |

Voice index v: 0..23 is core 0 voice v; 24..47 is core 1 voice v−24. The LIBSD entry is `((v%24)<<1) | (v>=24)`. **V**

### 2.3 SPU voice record (8 bytes)

| Off | Type | Meaning | EE writer | V/I |
|---|---|---|---|---|
| +0 | u32 | bits 0..22 = SPU **byte** start address (0 = stop). Bits 23..31 = start counter: EE `0x50c7b8 += 0x800000` on every start, so the word changes even when the address repeats | `SND_ChannelStartVoices` (start), `SND_ChannelStopVoices` / `SND_ReleaseFinishedVoices` (0) | V |
| +4 | u16 | SPU pitch (0x1000 = 48 kHz): `((srcRate<<12)/48000)*mul>>12`, clamped to 0x3FFF | `SND_ChannelApplyPitch` | V |
| +6 | s8 | VOLL target: −128..127 | `SND_ChannelApplyVolume`: `(char)(gain[ch1]*vol*127)` | V |
| +7 | s8 | VOLR target | `(char)(gain[ch0]*vol*127)` | V |

In DTS (config flags == 2) the EE mirrors every record and bit to voice v+24. **V**

Per-tick IOP algorithm for each v in 0..47 (V; cached state in §7):

1. If `pitch != cache.pitch`: `SetParam(PITCH)`, update cache.
2. If `volL != cache.volL || ramp[v].L`: `spusetvol(v, L)` (§6.2). Same for R.
3. If `word != cache.word`, let `a = word & 0x7FFFFF`:
   - `a == 0`: add v to the KOFF mask; `cache.word = word`.
   - `a != 0` and `GetParam(ENVX) == 0`:
     - set SSA = a, LSAX = 0x5000, ADSR1 = 0x000F, ADSR2 = 0x0005;
     - VOLL/VOLR = `vfix(volL/R)` (fixed);
     - clear ramp flags, update the cached vols;
     - add v to the KON mask; `cache.word = word`; `keyed = true`.
   - `a != 0` and ENVX ≠ 0 (still sounding): add v to the KOFF mask, **do not** update the cache (retried next tick); `keyed = true`.
4. Status (§4):
   - `echo[v] = word`;
   - `pos[v] = GetAddr(NAX)`, read **before** this tick's KON;
   - if `keyed`: `pos[v] = a`; else if `pos[v] < 0x5040`: `pos[v] = 0`.

After the loop:
- `updateiopvoices` (§8);
- `KON` (`SetSwitch 0x1500/0x1501`) if the mask is non-zero;
- then `KOFF` (0x1600/0x1601).

Masks are 48 bits as a (lo, hi) pair: core 0 = lo & 0xFFFFFF, core 1 = lo>>24 | hi<<8.

`vfix(b) = b < 0 ? (u8)b << 7 : b * 0x81` gives a 15-bit two's-complement fixed volume: 127 → 0x3FFF, −1 → 0x7F80 (= −0x80), −128 → 0x4000 (= −0x4000). **V**

---

## 3. EE PCM path

### 3.1 Formats

| Item | Value | V/I |
|---|---|---|
| EE block | planar s16. Plane p (p < ch+aux) at `data + p*N*2`, N = 0x100/0x180/0x200 for 24/36/48 kHz. Header u16 = `N*(ch+aux)*2` (+N*2 extra when mono, plane 1 = copy of plane 0, done by the EE thread) | V |
| Plane order | **plane 0 = Right, plane 1 = Left.** The IOP sends plane 1 to the first (L) AutoDMA block; the EE puts pan-gain index 1 in VOLL (+6) and index 0 in VOLR (+7) | V (mapping), I (L/R naming) |
| IOP mix bus | `b3a0[ch]`: 0x800-byte int32 buffers, 256 frames at 24 kHz, scale ×128 (mix = sample × (gain>>8), gain ≈ ±0x7FFF). Channel 1 = L | V |
| AutoDMA ring | `b3c0 = SNDMEMI_allocz(0x2000)` (0x50200 in the current runtime, I). `sceSdBlockTrans(0, 0x10 /*write+loop*/, b3c0, 0x1000)`, so only 0x1000 bytes are used: 2 halves × 0x800. Half layout: `[L 256 s16][R 256][L 256][R 256]` = 512 frames | V |
| Write target | `b3c0 + (1 - b322) * 0x800`, the half **not** playing | V |
| SPU side | core 0 input, BVOL L/R = 0 at init and **0x7FFF from the 3rd tick** (`b2f4 == 3`, counted on every `updatevoices`, never reset). MMIX core 0 = 0x0FC0, core 1 = 0x0FCC (input dry only, no input wet) | V |

### 3.2 `SNDIOP_mix` (tag 1)

```
if b31f { addspufxtomixer() }            // DTS only
MIX_audioslice(b3a0, b39e=256)           // IOP software voices -> int32 buses (§8)
if ch==1 { memcpy(b3a4, b3a0, 256*4) }
dispatch on (eeRate, iopRate) and ch<4:
  (36000,24000) ee36_iop24   (36000,36000) ee36_iop36   (48000,48000) ee48_iop48   (24000,24000) ee24_iop24
  ch>=4 -> *_spu48 variants (DTS; not used in stereo)
```
**V**

### 3.3 `SNDIOP_ee36_iop24`: the SSX 3 path, exact (V)

Per sub-block s ∈ {0,1} and output channel c ∈ {0 = L, 1 = R}:
- EE pointer e = `eePcm + {s0: L→384, R→0; s1: L→576, R→192}` samples.
- IOP pointer p = `b3a0[{L→1, R→0}] + s*128` ints.
- Persistent state per c: `pe[c]` s16 at 0x3b2f8 and `pp[c]` i32 at 0x3b304.
- `clamp(x) = min(max(x, -32768), 32767)`; `>>` is arithmetic.

With `e[-1] = pe[c]` and `p[-1] = pp[c]`, for j = 0..63 (k = 4j):
```
out[k  ] = clamp( e[3j-1]                     + (p[2j-1] >> 7) )
out[k+1] = clamp( ((e[3j-1] + 3*e[3j]) >> 2)   + ((p[2j-1] + p[2j]) >> 8) )
out[k+2] = clamp( ((e[3j] + e[3j+1]) >> 1)     + (p[2j] >> 7) )
out[k+3] = clamp( ((3*e[3j+1] + e[3j+2]) >> 2) + ((p[2j] + p[2j+1]) >> 8) )
```
Then `pe[c] = e[191]` (raw, unclamped s16), `pp[c] = p[127]`, and p[0..127] = 0 (the IOP bus is cleared after it is read). The output is the 256 s16 of the L or R block of sub-block s.

This is linear interpolation with a one-input-sample delay: 3:4 for the EE input, 1:2 for the IOP bus. Output block order: s0 L, s0 R, s1 L, s1 R.

### 3.4 Other variants (V, for completeness)

- **ee24_iop24.** Tables: EE {s0: L 256, R 0; s1: L 384, R 128}; IOP {L 1, R 0}. With `x[i] = e[i] + (p[i] >> 7)` (unclamped) and `x[-1] = pe` (stored **clamped** combined):
  - `out[2i] = clamp(x[i-1])`
  - `out[2i+1] = clamp((x[i-1] + x[i]) >> 1)`
  - Then `pe = clamp(x[127])`.
- **ee48_iop48.** Tables: EE {s0: L 512, R 0; s1: L 768, R 256}. `out[i] = clamp(e[i] + (p[i] >> 7))`, with no interpolation and no state.
- **ee36_iop36.** Not decoded in detail (unused).

---

## 4. Status block (`ioptoee`, IOP 0x3b098, 0x240 bytes → EE 0x50b740; EE copies it to 0x50b500 under lock each tick)

| Off | Type | IOP writer | EE reader | V/I |
|---|---|---|---|---|
| 0x000 | u32 | seqnum (starts 0, +1 per sent tick) | none | V |
| 0x004 | u32 | CPU load (µs/s × 6.55); host-time, nondeterministic | none | V |
| 0x008..0x03b | — | never written (0 from `start`'s memset) | none | V (no writer found) |
| 0x03c + v·8 | u32 | **echo** = SPU cmd word seen this tick | `SND_ReleaseFinishedVoices` (0x50b53c) | V |
| 0x040 + v·8 | u32 | **pos**: NAX byte address; or the new start address on the key/retry tick; or **0 if NAX < 0x5040** (voice parked in the safe loop = finished) | `ReleaseFinishedVoices` (0x50b540), `SND_ChannelGetPosition`: samples = (pos − start)·28/16 | V |
| 0x1bc + i·8 | u32 | **echo** of the IOP voice cmd word | `ReleaseFinishedVoices` (0x50b6bc) | V |
| 0x1c0 + i·8 | s32 | IOP voice frame = decoder read index (`MIX_getframe`); 0 on start; **−1 when stopped/ended** (`SNDIOP_mixvoicefree`, `MIX_stop` path) | `ReleaseFinishedVoices` (0x50b6c0), `GetPosition` | V |
| 0x23c | u32 | seqnum (copy) | none | V |

EE semantics (V):

- **SPU voices**, for each v with `cmd[v] != 0` on a master channel (+0x28 == −1):
  - if `echo != cmd`, then `pos := cmd & 0x7FFFFF` (stale status → "just started");
  - if `pos == 0`: `cmd[v] = 0` and `SND_ChannelRelease` (the channel is done). This is **the only voice-end signal; ENDX is never read** (LIBSD `GetSwitch` is not even imported).
- **IOP voices:** if `echo != cmd`, `pos := 0`; if `pos == −1`, release.
- `SND_ChannelStartVoices` also pre-writes `0x50b540+v*8 = start` (and `0x50b6c0+i*8 = 0`) into the *copy*.

Bit-exactness requirement: only the 48 + 8 (echo, pos) pairs matter. Their values depend on:
- NAX at the moment the tick runs (§6.4);
- the echo/override rules above;
- the IOP decoder read index (§8.3).

---

## 5. Sample uploads

| Kind | Meaning | Path | Destination | Completion | V/I |
|---|---|---|---|---|---|
| 8 | SPU RAM | `SND_BankAdd` → `SND_IopUpload(8, ee, spu, ≤0x1000)` → `SND_SifDmaQueue`: EE queue of 28 entries {id, src, dst, u16 size, u8 prio at +0xE}, highest prio then lowest id; one in flight (`0x50b9c2`). SIF DMA to the IOP staging buffer, then T8 | SPU byte address from `SND_SpuMemAlloc`: 64-byte units in [0x141, 0x8000) = **[0x5040, 0x200000)** (`SNDI_SpuMemSetBase`) | IOP `dmqueue`, `sceSdVoiceTrans(ch 1, write, staging, spu, size)`; on the DMA-end IRQ T9 (type 2, opt = id) → `SND_SifDmaComplete` → next. EE `SND_IopUploadWait` spins; restarts the DMA if 500 `SND_Tick`s (5 s) pass | V |
| 0x100 | IOP RAM | `SND_SifSetDma(src, dst, size, 0)` directly; returns the id but is never queued, so the poll is immediately done | `sceSifAllocIopHeap(size+0x40)` | none (EE assumes done) | V |

Bank image header (V):
- +6 u16 patch count;
- +8 offset of sample data;
- +0xC SPU region size (located at data + 0x10-size; region B comes first);
- +0x10 IOP region size;
- +0x14 patch offsets.

`SND_BankRelocatePatches` writes the absolute SPU/IOP address into each patch, and those addresses are what goes into the voice words. Banks with both sizes = 0 hold EE-voice samples (EE memory). **V**

IOP DMA queue (`SNDIOP_dmqueue`, V):
- 32 slots of 0x18 bytes at 0x40bd0: {id, token, a, b, u16 size (rounded up to 0x40), s16 prio, u8 dir}.
- The IOP's own users:
  - safe-loop upload (prio 100);
  - reverb zero-fill (prio 0x14, 0x1000-byte chunks of `sndiopzerobuf`);
  - DTS core-output reads (prio 0x28, dir 1).
- The highest prio is served first, ties go to the lowest id.

---

## 6. SPU state the driver controls

### 6.1 Init (`SNDIOP_init`, channels < 4) (V)

| Register (LIBSD entry) | Value |
|---|---|
| `sceSdInit(0)` | — |
| MVOLL/R both cores (0x980/0xA80 \| core) | 0, later **0x3FFF** |
| MMIX core 0 / core 1 | 0x0FC0 / 0x0FCC |
| EVOLL/R both cores | 0 |
| EEA both cores (`SetAddr 0x1D00/0x1D01`, 0x1E0000) | end = 0x1FFFFF |
| VMIXL/VMIXR (dry) | all 48 voices on |
| VMIXEL/ER (wet) | none |
| Effect attr | mode 0, depth 0, both cores; core attr EFFECT_ENABLE (2) = 1 |
| Trans IRQ handler ch 1 | `SNDIOP_dmcallback` |
| Safe loop | 0x40 bytes uploaded to SPU **0x5000**: 4 blocks, header bytes `00 07`, zero data, i.e. a silent self-loop |
| All 48 voices | PITCH 0, SSA 0x5000, LSAX 0x5000, ADSR1 0x000F, ADSR2 0x0005, VOLL/R 0; then **KON all** (every voice idles in the safe loop) |
| BVOL core 0 | 0 (0x7FFF on the 3rd tick) |
| SPDIF mode (core attr 0xA) | 0 |
| AutoDMA | `sceSdBlockTrans(0, 0x10, b3c0, 0x1000)`, handler ch 0 = `SNDIOP_autodmacb`, `b31c = 1` |

### 6.2 Volume ramps (`SNDIOP_spusetvol`) (V)

- `cur = (s8)(GetParam(VOLX{L,R}) >> 8)` (VOLX 0x600/0x700); target t = s8; `d = |t - cur|`.
- If `d>>1 < 2`: write fixed `vfix(t)`, `cache = t`, clear the ramp flag.
- Else: `rate = volramplut[d>>1]`, with `volramplut` (128 bytes) = `0,55,51,49,47,46,45,44,43,43,42,42,41,40,40,40,39×4,37×8,36×4,35×8,34×8,33×8,32×8,31×16,30×16,29×16,28×16`. Let c16 = (s16)VOLX and t16 = t<<8:
  - `c16 < t16`: `c16 < 0` → `0xB000|rate`, but `0xB01B` if `t16 >= 0`; otherwise `0x8000|rate`.
  - else: `c16 <= 0` → `0x9000|rate`; otherwise `0xA000|rate`, but `0xA01B` if `t16 <= 0`.
- In the ramp branch: write the sweep value, `cache = cur`, set the ramp flag. The flag forces re-evaluation every tick until the target is reached.
- Sweep bits used: 0x8000 sweep, 0x2000 decrease, 0x1000 **phase (negative)**. **Exponential (0x4000) is never used.**

### 6.3 Reverb (`checkspueffect`) (V; unused in practice, I)

States `b31d`:
- **0 (normal).** EVOL L/R step ±0x800 per tick toward cmd +0x14/+0x16. Writes EVOLL to cores 0 and 1 and EVOLR to cores 0 and 1, or 0 when `b31f`.
- **1 (mode changed).** Ramp both EVOLs to 0 by 0x800 per tick. At 0:
  - EEA core 1 = 0x1E0000 (mode 0) or 0x1C0000 (end 0x1DFFFF);
  - `SetEffectAttr(core 0 and core 1, {mode, depth 0/0, delay = +0x11, feedback = +0x12})`;
  - clear targets [ESA0, 0x200000) and [ESA1, 0x1E0000).
- **2.** Zero-fill DMA, 0x1000 bytes per tick per core, top-down; when both are done → 0.

### 6.4 What a native SPU must model

| Feature | Needed | Detail | V/I |
|---|---|---|---|
| 48 voices, PS-ADPCM, flags 1/2/4 | yes | Sony encoding is used: loop start = 6, inside = 2, loop end = 3, one-shot end = **1** (block counts in zBxsfx: 1:39, 2:12950, 3:3520, 6:6) | V (data) |
| SSA, LSAX write, loop-start flag overriding LSAX | yes | LSAX = 0x5000 is written before every KON, so a one-shot ending with flag 1 jumps to the **safe loop**; looped samples override LSAX with their own flag 4 | V |
| **NAX read** (byte address, as LIBSD `GetAddr`: (hi<<17)\|(lo<<1)) | yes, **observable** | Sampling point and granularity determine status pos and `SND_GetTimeRemaining`. A voice that ended with flag 1 must keep fetching from 0x5000 (muted) so that NAX < 0x5040 | V/I |
| ENVX read | yes | `== 0` gates KON (retrigger delay of one tick when not 0) | V |
| ADSR | fixed | ADSR1 0x000F (attack linear rate 0, decay 0, sustain level 15), ADSR2 0x0005 (sustain linear-increase rate 0, release linear rate 5, about 64 samples). Flag-1 end forces release and level 0 | V |
| Pitch ≤ 0x3FFF, interpolation | yes | HW: 4-point Gaussian; current model: linear | V/I |
| VOL fixed (15-bit signed, negatives used for pan) and linear sweeps with the phase bit | yes | | V |
| KON / KOFF | yes | applied after all parameter writes of the tick; KON before KOFF | V |
| VMIX/VMIXE per voice, MMIX, MVOL, BVOL, AutoDMA core 0 | yes | | V |
| EVOL, reverb modes 0..9, ESA/EEA, work-area zeroing | only if mode ≠ 0 | | I |
| Voice DMA write ch 1 (uploads) | yes | | V |
| Unused | — | PMON, NON (only the DTS calibration), ENDX, IRQA, exponential sweeps, core 1 AutoDMA, TSA/PIO, AVOL/external input, ADSR variants | V (no such LIBSD calls in SNDDRV) |

---

## 7. IOP state carried between ticks

| Address | Contents | Init | V |
|---|---|---|---|
| ae18 + v·8 | {u32 lastWord, u16 pitch, s8 volL, s8 volR} per SPU voice | 0 | V |
| b338 + 2v, b339 + 2v | ramp-pending flags L/R | 0 | V |
| adf8 / adfc, ae00 / ae04 | cached wet / dry 64-bit masks | 0 / (0xFFFFFFFF, 0xFFFF) | V |
| ae08 | cached reverb mode (s8) | 0xFF before init, 0 after | V |
| ae0b | cached resample quality | 0 | V |
| ae0c, ae0e | current EVOL L/R | 0 | V |
| b31d | effect state; b2dc/b2e0 clear cursors; `sndiop[0]` last zero-fill DMA id | 0 | V |
| b2f4 | updatevoices count (BVOL on at 3) | 0 | V |
| b2f8[2] s16, b304[2] i32 | resampler history (EE, IOP) | 0 | V |
| b322 | AutoDMA half (from BlockTransStatus) | — | V |
| af98 + i·0x10 | IOP voice caches {lastWord, pitch, gains[6..], wet} | 0 | V |
| b328[i] | IOP voice active flag | 0 | V |
| b254 / b258 + i·8 | IOP voice status echo / frame (b258 = −1 when idle) | −1 | V |
| seqnum (0x3aca8) | tick counter | 0 | V |
| 0x3b3c4 receive area | last EE tag buffer (replayed if not refreshed) | 6 | V |
| MIX voice table (0x50 bytes each), filter chains, declick residuals 417c4/417dc | §8 | — | V |
| DMA queue 0x40bd0 | 32 slots | empty | V |

---

## 8. IOP software voices (MIX, needed for zboard.bnk)

### 8.1 IOP voice record (cmd block 0x1a0 + i·0x10)

| Off | Type | Meaning | V |
|---|---|---|---|
| +0 | u32 | bits 0..22 = IOP address of the sample header (0 = stop); bits 23..31 = start counter | V |
| +4 | u16 | pitch, `((rate<<12)/iopRate)*mul>>12`. IOP: `MIX_setpitch(i, p<<4)`, 16.16, max 0x40000 | V |
| +6 + k | s8 | dry gain for output channel k (k < ch+aux; 0 = R, 1 = L): `g = v<0 ? v<<8 : v*0x102` | V |
| +0xC | s8 | wet gain → `SNDMIX_setwetgain(i, 0, v<<8)`; ignored unless the IOP fx bus is on (never, I) | V |

The EE writes +6.. as `(char)(fvol*pan*0.007874*gain*127)`.

### 8.2 Sample header in IOP RAM (V; zboard's first sample in brackets)

| Off | Type | Value |
|---|---|---|
| +0 | u8 | passed as the last `MIX_playinit` argument (sub-format; 0) |
| +1 | u8 | codec: **8 = PCM16, 9 = PCM8 (s8 << 8), 10 = EA-XA** [9] |
| +4 | u32 | loop start frame [5] |
| +8 | u32 | loop end frame, inclusive, > 0 means looped [0x1c263] |
| +0xC | u32 | frame count [0x1c267] |
| +0x10 | — | data |

### 8.3 Behaviour (V)

**Tick logic** (`updateiopvoices`, run after the 48 SPU voices):
- echo = word.
- On pitch change (if active): `MIX_setpitch`.
- On gain change: `setdrygain` for all channels.
- On wet change: `setwetgain`.
- On word change:
  - non-zero: if active, `MIX_stop`; then `MIX_playinit` (decoder node: unpack8/16 one-shot or looped, or XA), `setpitch` (creates the resampler node: linear when quality > 0x18, nearest otherwise), gains, `MIX_play` (current gains = targets, no ramp); status frame = 0.
  - zero: `MIX_stop`, frame = −1.
- If active: frame = `max(MIX_getframe, 0)`.

**Mixing** (`MIX_audioslice(bus, 256)`):
1. Clear the buses.
2. Add the pending declick residual r over the first 16 samples: `bus[i] += r*w[i]`, `w = 0x77,0x70,0x68,0x61,0x59,0x52,0x4a,0x43,0x3b,0x34,0x2c,0x25,0x1d,0x16,0x0e,0x07`.
3. For each voice in state 2:
   - If gains are dirty, render 16 samples with a ramp: gain_i = g0 + (i+1)·((g1−g0)/17), i = 0..15.
   - Then pull n frames from the chain head and mix: `bus += s * (g>>8)` (`mixiop`).
   - Remember the last sample at +0x3c.
4. A pull returning < 0 means the end: `MIX_stop`, then the callback sets frame −1.
5. `MIX_stop` declick: `r += (g>>8)*last >> 7`.

**Resampler** (`SFILTER_rs`):
- 16.16 phase with 2-sample history, pass-through at 0x10000.
- Requests `(pitch*n + phase) >> 16` input frames.
- Linear: `y = s0 + ((s1 - s0)*frac >> 16)`.

**Decoders:**
- Decoder position = **frames consumed so far**, which runs ahead of playback; this is what the status reports.
- Ends: one-shot `unpackf` zero-fills the last partial request (returns 1), then returns −1. The looped variant wraps from loopEnd+1 to loopStart.
- EA-XA (`SFILTER_unpackxaf`, `decxa16c`) is the same format as the EE's `SND_EaxaDecodeBlocks`.

For bit-exact status, port `MIX_audioslice`, `SNDMIXI_volramp`, `SFILTER_rs`, `rs16linear` and the unpackers literally (≈ 1.5k lines of decompile).

IOP effects (`MIX_initreverb`, `SNDMIXI_initfx`, the SFILTER graph: lpfRC, amplf, mixer, splitter, echo; FIR8/reson are "not supported on IOP") apply only after RPC fno 2, which is not used during play (I).

---

## 9. Proposed native driver layout (signatures only)

```rust
// crates/ssx3-snd/src/lib.rs
pub mod proto;   // wire formats (§1, §2, §4, §5)
pub mod driver;  // SNDDRV logic (tick, uploads, rpc)
pub mod spu;     // SPU2 subset (§6.4)
pub mod iopmix;  // EA MIX/SFILTER port (§8)
pub mod output;  // ee36_iop24 & friends (§3)

// proto.rs
pub const SND_RPC_ID: u32 = 0x0053_4E44;
pub const SND_VERSION: u32 = 0x0008_0303;
pub const STATUS_EE_ADDR_DEFAULT: u32 = 0x0050_B740;
pub const STATUS_LEN: usize = 0x240;
pub const SAFE_LOOP_ADDR: u32 = 0x5000;
pub const SAFE_LOOP_END: u32 = 0x5040;
#[repr(C)] pub struct InitParams { pub version: u32, pub status_ee: u32, pub dts_ee: u32,
    pub ee_rate: u16, pub iop_rate: u16, pub channels: u8, pub aux: u8, pub iop_voices: u8, pub spu_fx_to_mixer: u8 }
#[repr(C)] pub struct SpuVoiceCmd { pub word: u32, pub pitch: u16, pub vol_l: i8, pub vol_r: i8 }
#[repr(C)] pub struct IopVoiceCmd { pub word: u32, pub pitch: u16, pub dry: [i8; 6], pub wet: i8, pub _pad: [u8; 3] }
#[repr(C)] pub struct CommandBlock { pub wet_bits: u64, pub dry_bits: u64, pub rev_mode: i8, pub rev_delay: i8,
    pub rev_feedback: i8, pub resample_quality: u8, pub evol: [i16; 2], pub _pad: [u8; 8],
    pub spu: [SpuVoiceCmd; 48], pub iop: [IopVoiceCmd; 16] }          // size 0x2a0
#[repr(C)] pub struct StatusBlock { pub seq: u32, pub cpu: u32, pub _r: [u32; 13],
    pub spu: [[u32; 2]; 48] /* echo, pos */, pub iop: [[i32; 2]; 16] /* echo, frame */, pub seq2: u32 } // 0x240
pub enum Tag<'a> { Header { seq: u32, total: u32 }, Voices(&'a CommandBlock), Pcm(&'a [i16]),
    EeWet(&'a [u8]), Dts(&'a [u8]), Trailer { seq: u32 }, End }
pub fn parse_tags(buf: &[u8]) -> Result<Vec<Tag<'_>>, TagError>;
pub fn tag_buffer_consistent(buf: &[u8]) -> bool;           // header seq == trailer seq
#[repr(C)] pub struct UploadCmd { pub hdr: [u32; 4], pub kind: u32 /*1*/, pub token: u32,
    pub iop_src: u32, pub spu_dst: u32, pub size: u16, pub prio: u16, pub _pad: [u32; 3] }

// driver.rs
pub trait EeLink {                                           // implemented by ssx3-ps2 (sif.rs/iop_bridge seam)
    fn write_ee(&mut self, ee_addr: u32, data: &[u8]);
    fn send_cmd1(&mut self, kind: u32 /*0 tick, 2 upload done*/, opt: u32);
    fn iop_ram(&mut self) -> &mut [u8];                      // where EE SIF DMA lands (tag buffer, staging, IOP banks)
}
pub struct SndDriver { pub cfg: Option<InitParams>, spu: spu::Spu, mix: iopmix::IopMixer, out: output::Resampler,
    cache: VoiceCaches, fx: EffectState, uploads: UploadQueue, ticks: u32, seq: u32, tag_rx_addr: u32 }
impl SndDriver {
    pub fn new(tag_rx_addr: u32) -> Self;
    pub fn rpc(&mut self, link: &mut dyn EeLink, fno: u32, data: &[u8]) -> Vec<u8>; // 0 init, 1 restore, 2 iop fx
    pub fn user_cmd0(&mut self, link: &mut dyn EeLink, pkt: &UploadCmd);           // queue SPU upload
    pub fn service_uploads(&mut self, link: &mut dyn EeLink);                      // complete -> cmd1 type 2
    /// One AutoDMA half: process tag buffer, render 512 frames, send status + tick.
    pub fn tick(&mut self, link: &mut dyn EeLink, out: &mut [[i16; 2]; 512]);
}

// spu.rs
pub struct Voice { /* ssa, lsax, nax, adpcm state, env, vol[2] incl. sweep+phase, pitch, kon/koff */ }
pub struct Spu { pub ram: Box<[u16; 0x10_0000]>, pub voices: [[Voice; 24]; 2], /* vmix, vmixe, mvol, bvol, evol, mmix, rev */ }
impl Spu {
    pub fn set_param(&mut self, entry: u16, value: u16);        // LIBSD entry encoding
    pub fn get_param(&self, entry: u16) -> u16;
    pub fn set_addr(&mut self, entry: u16, byte_addr: u32);
    pub fn get_addr(&self, entry: u16) -> u32;
    pub fn set_switch(&mut self, entry: u16, mask: u32);       // KON/KOFF/VMIX*
    pub fn dma_write(&mut self, byte_addr: u32, data: &[u8]);
    pub fn render(&mut self, input: &[[i16; 2]], out: &mut [[i16; 2]]); // core0 input = AutoDMA half
}

// iopmix.rs
pub struct IopMixer { /* voices, chains, declick residuals, quality */ }
impl IopMixer {
    pub fn play(&mut self, v: usize, hdr: &IopSampleHeader, ram: &[u8]);
    pub fn stop(&mut self, v: usize);
    pub fn set_pitch(&mut self, v: usize, step_16_16: u32);
    pub fn set_dry(&mut self, v: usize, ch: usize, gain: i32);
    pub fn frame(&self, v: usize) -> i32;                       // -1 when idle
    pub fn slice(&mut self, ram: &[u8], buses: &mut [[i32; 256]; 2]);
}

// output.rs
pub struct Resampler { pe: [i16; 2], pp: [i32; 2] }
impl Resampler {
    pub fn ee36_iop24(&mut self, ee: &[i16; 768], iop: &mut [[i32; 256]; 2], half: &mut [i16; 1024]);
    pub fn ee24_iop24(&mut self, ee: &[i16; 512], iop: &mut [[i32; 256]; 2], half: &mut [i16; 1024]);
    pub fn ee48_iop48(&mut self, ee: &[i16; 1024], iop: &mut [[i32; 512]; 2], half: &mut [i16; 2048]);
}
```

Seam: replace `iop_bridge::command`/`sync` for 'SND' and cmd 0 with `SndDriver`.
- Deliver `tick` at fixed EE-cycle points: every 3,145,728 EE cycles, plus one immediately after the init RPC to match the IOP.
- Read the tag buffer from the IOP RAM address you advertise in the tick's opt.

---

## 10. Open questions and findings to verify

1. **Possible SPU2-model bug (I, high impact on status).** `spu2.rs::decode_block` handles a flag-1 (end + mute) block by setting env = 0 and phase Release→Off, and `render_voice` then stops advancing. NAX freezes inside the sample instead of moving to LSAX 0x5000. Hardware (and SNDDRV's design) expects the muted voice to keep fetching the safe loop, so status pos becomes 0 and the EE releases the channel. If this is right, one-shot SPU voices are never reported finished under `SSX3_IOP=1`, and channels leak or are only stolen. The flag is also processed one block early: the last 28 samples are muted.
   - To check: trace status +0x40 for a voice after a menu click; it should reach 0 within a few ticks.
2. Confirm the runtime RPC init block (expected `03 03 08 00 | 40 b7 50 00 | 00000000 | a0 8c c0 5d | 02 00 08 00`) with `SSX3_TRACE_IOP` (the bridge logs RPC send words).
3. Does any IOP voice actually play? Check cmd +0x1a0 words or MIX activity in a race with board sounds (zboard.bnk vs zboardSPU.bnk selection).
4. NAX sampling point and granularity. Real HW NAX granularity and the exact sample at which the IOP thread reads it (IRQ latency) differ from the model, and these values are game-visible (`SND_GetTimeRemaining`, release timing). Pick a deterministic rule (for example, NAX at the block boundary) and accept divergence from the emulated IOP.
5. The sweep **phase bit 0x1000** is ignored by `spu2.rs::volume`. Negative (phase-inverted) voice volumes occur whenever the EE pan gain is negative. Verify whether stereo mode ever produces them (`SND_PanFixQuadrant`).
6. Reverb mode: is 0x50ab08 ever non-zero (`SND_GetFxParams` writes through a pointer)? If not, reverb, EVOL and the work-area clear can be dropped.
7. EE upload priority argument: `SND_BankAdd` calls `SND_IopUpload` with 4 arguments, so the 5th (t0, priority) is whatever is in the register. Harmless only because the EE serialises uploads; check the disassembly at 0x3b6098.
8. Replay or extra ticks (§1.3) are timing artefacts of the real IOP. A synchronous native driver should not reproduce them, but that changes PCM relative to `SSX3_IOP=1` when the EE thread is late.
9. Bytes 0x08..0x3b of the status block and cmd block +0x18..+0x1f have no writer or reader found (Ghidra xrefs); confirm nothing indexes them computationally.
10. LIBSD `BlockTransStatus >> 24` half semantics. The model must report the half being *played* at the IRQ (the IOP writes the other half); verify against `spu2.rs` ADMA bookkeeping.
11. The HLE fallback in `sif.rs::vblank` sends ticks with opt = 0, so the EE DMAs its tag buffer to IOP address 0. That is harmless today but worth knowing when switching seams.

Supporting artefacts are all in `(scratch, not committed) `:
- `decomp.c`: full IOP decompile, 263 functions;
- `syms.txt`, `iop.bin`: relocated LIBSD + SNDDRV image at 0x20000;
- `ee/*.c`: EE decompiles;
- `reloc.py`, `gh/Setup.java`: how the IOP image and decompile were produced.
