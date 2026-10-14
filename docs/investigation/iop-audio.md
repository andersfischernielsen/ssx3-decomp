# IOP audio: recompiled LIBSD/SNDDRV on a native IOP kernel

_Written 2026-10-14._ Status: **working behind `SSX3_IOP=1`**. The game's own
sound modules run as statically recompiled Rust, drive a modelled SPU2, and
produce audio on the host.

## Why

Before this, the IOP side of EA's sound driver was faked in `sif.rs`: the
`'SND'` RPC server returned zeros, a fake per-VBlank tick (SIF user command 1,
type 0) kept the EE's sound thread alive, and sample-bank uploads (user
command 0, type 1) were acknowledged without doing anything. The game ran, but
silently. Now the disc's `DATA/MODULES/LIBSD.IRX` (Sony's sound library) and
`DATA/MODULES/SNDDRV.IRX` (EA's "SND Library EE Driver") are translated to
Rust, the same way as the EE code, and run against native firmware.

## Pieces

| piece | where | what |
| --- | --- | --- |
| IRX loader | `crates/ssx3-iop/src/irx.rs` | parses the ELF + `.iopmod`, relocates (`R_MIPS_32/26/HI16/LO16`, v1 and v2 pairing), finds import (`0x41E00000`) and export (`0x41C00000`) tables, symbols, and every code address that data or `%hi/%lo` pairs produce. Fixed load addresses (`irx::layout`): LIBSD `0x20000`, SNDDRV `0x30000`, native heap `0x50000..0x100000`. Shared by generator and runtime. |
| R3000 → Rust generator | `crates/ssx3-recomp/src/iopgen.rs`, bin `recomp-iop` | per-function block dispatchers over labels, same unwind/trampoline model as `eegen.rs`. Output: crate `ssx3-iop-gen` in `generated/iop/modules/` (gitignored) + `report.txt`. `recomp-ee` also runs this step. |
| IOP state + runtime | `crates/ssx3-iop/src/lib.rs` | `Iop`: 32-bit GPRs, HI/LO, 2 MiB RAM, scratchpad, I/O dispatch; trampoline, nested native→guest calls, scheduler loop `Iop::run(target)`. |
| kernel | `kernel.rs`, `imports.rs` | threads (priorities, FIFO), sleep/wakeup, delay, semaphores, event flags, alarms, first-fit allocator, interrupt handler table, module start. All 45 imports the two modules use (sysmem, loadcore, intrman, thbase, thsemap, thevent, sifman, sifcmd, sysclib). |
| SIF, IOP end | `sif.rs` | `sceSifAddCmdHandler`, `sceSifRegisterRpc` / `SetRpcQueue` / `GetNextRequest` / `ExecRequest`, `sceSifSendCmd`, `sceSifSetDma`. |
| hardware | `hw.rs`, `spu2.rs` | INTC (`I_STAT/I_MASK/I_CTRL`), DMA ch4/ch7 + `DICR/DICR2`, SPU2 (below). |
| host audio | `audio.rs` | cpal output on its own thread via a ~200 ms ring buffer; `SSX3_WAV=file.wav` capture (header kept valid while writing). |
| integration | `crates/ssx3-ps2/src/iop_bridge.rs` (+ 4 hooks in `sif.rs`, 1 line in `machine.rs`) | see below. |

### Generated code

`recomp-iop` reads both IRX files from the ISO, relocates them at their fixed
bases, and emits one crate (≈1.1 MB of Rust, 206 functions, 14,581
instructions, 0 invalid encodings). Function starts: SNDDRV's symbol table
(it ships one), the module entries, export entries, `jal` targets and, for
LIBSD (no symbols), `%hi/%lo` code pointers. Labels: branch targets, return
points, every relocated code reference (jump tables, handler pointers), and
targets of branches from other functions. Import stubs never become code:

* a `jal` to a stub of a native library is lowered to
  `ssx3_iop::imports::call(c, LIB, INDEX)`;
* SNDDRV's `libsd` imports are linked statically to LIBSD's export table, so
  they are direct Rust calls into the recompiled LIBSD.

The generated crate records each module's text hash; the runtime re-reads
the IRX from the disc, relocates it the same way, and refuses to run
generated code that does not match.

R3000 specifics handled by the generator: 32-bit arithmetic, HI/LO
semantics including division by zero, `lwl/lwr/swl/swr` little-endian merges,
no branch-likely, `bltzal/bgezal` link-always. **Load delay slots** are not
modelled (loads are immediate); the generator scans for loads whose result is
read by the next instruction and lists them in `report.txt`: there are **0**
in these modules (the compiler always filled the slot). `break` (division by
zero checks) and `syscall` stop the IOP with a diagnostic.

### Execution and scheduling

* `Iop::run(target)` advances the IOP clock (36.864 MHz = EE / 8). It services
  timed events (thread delays, alarms, DMA completions), the SPU2 output
  batches (every 32 samples = 24,576 IOP cycles), incoming SIF traffic and
  interrupts, then runs the current thread until the next event (slices of
  at most 4,096 cycles; generated code checks `cycles >= next_event` at
  backward branches).
* Thread switches use the EE scheme: a blocking native call saves the
  thread context with `pc = $ra`, sets `UNWIND`, and every host frame returns
  to the trampoline, which re-enters the next thread at its saved `pc`.
* Interrupt handlers and callbacks run nested (`Iop::call_guest`) on an
  interrupt stack and return to a magic address.
* `sceSifExecRequest` runs the RPC server function as a guest *tail call*
  returning to a magic `EXEC_FINISH` address (so the server may block), where
  the reply data is copied to the EE and the RPC end packet is queued.

### Integration (`iop_bridge.rs`)

* **Enable:** `SSX3_IOP=1` (and the game crate must have registered the
  generated code; `game.rs` does). Default behaviour is unchanged.
* **Module load:** LOADFILE of `LIBSD.IRX` / `SNDDRV.IRX` starts the
  recompiled module (its start thread runs to completion inside the RPC).
* **SIF:** binds/calls to RPC servers the IOP registered (`0x00534E44`
  'SND') and user commands with an IOP handler (command 0) are queued for the
  IOP. Everything else stays on the HLE servers.
* **Time:** `machine::service_events` calls `iop_bridge::sync`, which runs
  the IOP up to `ee_cycles / 8` and then applies IOP→EE DMA writes and queues
  IOP→EE packets into `sif.rs`'s outgoing queue.
* **Shared IOP RAM:** `Bus::iop` holds IOP RAM while the EE runs (EE SIF DMA
  and the HLE servers write into it); it is swapped into the `Iop` while IOP
  code runs.

### SPU2 model (`spu2.rs`)

Ported from ps2-web-recomp `spu2.cpp` (MIT) and restructured: 2 cores × 24
voices; PS-ADPCM decode with loop flags and `LSAX`; ADSR (attack/decay/
sustain/release, linear and exponential, PSX rate tables); pitch with linear
interpolation; fixed and sweep volumes; `KON/KOFF/ENDX`; `TSA`/data port;
manual DMA to/from sound RAM; **AutoDMA** (core input from IOP RAM, 512 bytes
L / 512 bytes R per 256 frames, completion when consumed); IRQ address
matching on voice fetches, DMA, data port and the output "rings"; master
(`MVOL`) and input (`BVOL`) volumes; 48 kHz stereo out.

## What SSX 3 does with it

EA's driver is a hybrid. The EE sound thread (`0x3c19a8`) mixes streams and
builds a *tag buffer* each tick, which it DMAs to SNDDRV (`0x3b3c4`); SNDDRV
(`threadmain`, `SNDIOP_processtagbuf`) mixes the EE's PCM, converts sample
rates (`SNDIOP_ee24_iop24_spu48` …) into the AutoDMA buffers at IOP
`0x50200/0x50a00` (core 0, 512 frames each), updates SPU voices
(`SNDIOP_updatevoices`) and sends the next tick (SIF command 1, type 0, with a
0x240-byte status block) to the EE. The AutoDMA completion interrupt
(LIBSD's DMA handler → `SNDIOP_autodmacb`) paces everything at 48 kHz:
≈94 ticks per second. Sample banks are uploaded with user command 0 type 1
and acknowledged with command 1 type 2 after the SPU DMA (`sceSdVoiceTrans`)
completes. One 'SND' RPC call (init) is made.

### Music needed the EE timer interrupt

The first runs had sound effects but no music, with or without the
recompiled IOP. Music is EA's interactive "path" streaming (`.mus` files in
`MUSIC.BIG`/`MUSIC2.BIG`, `SongInstance_ctor` `0x2b1c50`, stream open
`0x3d25f0`): the stream opened (title: `data/audio/music2.big|data\audio\charsel.mus`)
but never read past the BIG table of contents. The stream service is a
periodic task (`0x2ae020`, registered by `0x2ae048` in EA's timer task table
at `0x51ed98`), and that table only runs from `EATimer`'s RCNT1 compare
interrupt (INTC 10, handler `0x3e4db8`, `EATimer_Init(60)` at `0x3e4c50`).
`hw.rs` models the EE counters but never raised TIM0..3, so EA's tick
counter `0x450dd0` stayed 0 and no timer task ever ran.

`crates/ssx3-ps2/src/ee_timers.rs` adds RCNT compare interrupts (CMPE,
`EQUF`, ZRET/wrap periods, next-deadline scheduling), called from
`machine::service_events` (2 lines). It is opt-in: on with `SSX3_IOP=1` or
`SSX3_EE_TIMER_IRQ=1`, off otherwise, because it changes EE behaviour (three
other timer tasks — `0x31ad20`, `0x3e33b0`, `0x3b5450` — now run as well).

Observed (headless, `SSX3_IOP=1`):

* boot → title → main menu → race start still works.
* Title and main menu: **music** from the title screen on (≈18 s into the
  run), mixed by the EE, streamed through SNDDRV's AutoDMA input (status
  line `level: input ≈1300`). Menu cursor/confirm sound effects play from SPU
  voices on top. 
* Analysis of the main-menu capture (30–39 s of the run): rms ≈5,100,
  peak ≈26,000, zero-crossing rate 0.023 (white noise ≈0.5), spectral
  flatness 0.18, onset autocorrelation peaks at 0.48 s / 0.96 s / 1.92 s
  (**125 BPM**, 4-beat bars; r = 0.56), stereo with a small side component
  (side/mid 0.05), no discontinuities at the 256-sample AutoDMA block edges.
  That is beat-structured music, not noise or garbage.
* The race (Snow Jam; the HUD's "EA RADIO BIG" box changes tracks, e.g.
  "Andy Hunter — Go") has music, ambience and effects; onset periodicity
  0.44 s / 0.89 s / 1.78 s (≈136 BPM). Loud moments reach full scale (peak
  ±32767 in a few seconds of a 78 s capture).

## Gaps

* Reverb (`ESA/EEA`, effect volumes) is not modelled; voice dry/wet routing
  (`VMIX*`, `MMIX`) is ignored (every voice is mixed dry).
* Core 0's output feeds core 1 as a plain sum (no `MMIX` gating, no `AVOL`).
* Output occasionally saturates in the race (no reverb/wet split, all voices
  dry); real hardware mixes at similar levels, but this is unverified.
* Interpolation is linear (hardware: 4-point Gaussian).
* Timing: one IOP cycle per instruction; native calls cost a flat 8 cycles.
* Only the libraries these two modules import are implemented; other
  imports log once and return 0.
* The cpal device runs free: when the game runs slower than real time the
  ring buffer underflows (gaps), when faster it drops samples. No resampling
  to emulation speed.
* Other IRX (SIO2MAN, PADMAN, MCMAN, CDVD) stay HLE.

## How to run

```sh
cargo run --release -p ssx3-recomp --bin recomp-ee    # also writes generated/iop
# or just the IOP part:
cargo run --release -p ssx3-recomp --bin recomp-iop
cargo build --profile game -p ssx3 --bin ssx3-headless
SSX3_IOP=1 SSX3_WAV=/tmp/ssx3.wav SSX3_INPUT="950:0010:8,1000:4000:8,1600:0008:8" \
  ./target/game/ssx3-headless 2400
SSX3_IOP=1 cargo run --profile game -p ssx3           # windowed, with sound
```

Env: `SSX3_IOP=1` enable (also turns on EE timer IRQs, see above); `SSX3_AUDIO=0/1` output device (headless defaults
to off); `SSX3_WAV=path` capture; `SSX3_TRACE_IOP` (thread switches, every
import call, SIF traffic); `SSX3_TRACE_SPU2` (register writes, DMA). The
headless status line prints an `iop …` summary every 60 frames.
