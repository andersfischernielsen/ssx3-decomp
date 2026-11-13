# ps2-web-recomp: engineering digest

Reference: <https://github.com/snuri00/ps2-web-recomp>, commit `a02cdd56a8ab` (2026-10-13), cloned to the session scratchpad. C++20, MIT, "Copyright (c) 2026 Samir Nuri". `runtime/src/ipu/tables.hpp` is BSD-2 (adapted from Play!). The clone succeeded, so ran-j/PS2Recomp was not needed.

It plays Gran Turismo 4 (PAL) and God of War II end to end, in the browser (Emscripten + WebGPU) and natively (wgpu-native + SDL2). The EE is AOT-translated to C++. Everything else is HLE or an interpreter. All paths below are relative to the repo root.

**Bottom line for us:** this is the closest existing design to what we are building. It is not cycle-accurate. Most transfers are instantaneous and it relies on HLE. It still runs two demanding games, so its simplifications are a tested "good enough" baseline. The parts most worth porting are the EE kernel HLE, the SIF RPC HLE, the IOP kernel/library HLE, the DMAC/VIF/GIF state machines, the VU interpreter and flag pipeline, the GS swizzle tables, and the software rasterizer.

---

## 1. Execution model and how EE threads map onto AOT code

### Generated code (`recomp/codegen.cpp`)
- Guest code is split into **16 KiB units**. Each unit becomes `void unit_X(Cpu &c, uint32_t entry, uint32_t depth)` with a `switch(entry)` jump table (`dispatch_entry:`) over its basic-block labels. The `UnitInfo{base,size,function,entries[]}` table maps a PC to its unit and entry id. This is `runtime/include/ps2/generated.hpp`, and the lookup is `dispatch()` in `runtime/src/dispatch.cpp`.
- Jumps and branches inside a unit use `goto`. Jumps into another unit use `[[clang::musttail]] return unit_Y(c, id, depth)`. Unknown targets set `c.pc` and tail-call `dispatch()`.
- **`jal` is a real host call**: `if (depth < kMaxDepth) unit(c,id,depth+1); else { c.pc = target; return; }` followed by `if (c.pc != ret) return;` (`call_to`, codegen.cpp:274). `kMaxDepth = 192`. `jr $ra` becomes `c.pc = ra; return;`, so the caller's host frame checks that `c.pc` equals the expected return address. Any mismatch propagates the return upward.
- **Cycle budget**: `c.budget -= N` per block, where N is roughly the instruction count. Only **backward branches** test for preemption, with `if (c.budget < 0 && preempt(c)) { c.pc = target | kUnwind; return; }` (codegen.cpp:267).
- `syscall` emits `c.pc = pc+4; syscall(c, pc); if (c.pc != pc+4) return;` (codegen.cpp:421).

### Thread model: no host threads and no fibers. It unwinds the host stack.
- The host stack is only a **cache of guest call frames**. The trampoline is `ps2::run()` (`runtime/src/system.cpp`). It loops `run_due_events();`, then `kernel::service(c);`, then `dispatch(c,0,0)`.
- A thread switch (`kernel::switch_to`, `runtime/src/kernel.cpp`) saves the outgoing context (`save_context`: GPRs, HI/LO, SA, PC, FPRs, FCR31), loads the next one, and sets `c.pc |= kUnwind` (bit 0). Every generated frame then returns because `c.pc != ret`. The trampoline clears the bit and dispatches the new thread at its saved PC with `depth = 0`. When the resumed thread later runs `jr ra`, control returns to the trampoline, which dispatches `ra`. This is why no per-guest-thread host stack is needed.
- Idle: if no thread is ready, `c.pc = kIdle (0x1010)`. `idle()` fast-forwards time to the next event, capped at `kEeClock/2000`. It halts with "deadlock" if there are no events and the IOP is idle.
- Magic addresses: `kHandlerReturn = 0x1000` (the `ra` for handler and callback calls) and `kIdle = 0x1010`. Handlers run on a kernel stack at `0x80000`, or `0x7C000` when nested.

### Interrupt delivery
- `kernel::must_unwind()` returns true if a reschedule or alarm is pending, or if interrupts are enabled and INTC/DMAC have pending causes. Interrupts are enabled when `(Status & 0x10001) == 0x10001`, EXL=0 and ERL=0.
- `preempt()` runs due events, refills the budget, then returns `halted || must_unwind()`. Interrupts are therefore taken only at backward branches, at syscalls, and between trampoline iterations. There is no instruction-precise delivery.
- `kernel::service()` runs alarms first. It then drains up to 16 pending causes, **DMAC before INTC**, lowest bit first. Each cause is handled by `run_dmac`/`run_intc`, which call `hw::*_ack(cause)` and then run every registered handler for that cause in list order through `run_guest()`. If a handler returns a negative value, the chain stops.
- `run_guest()` is the important primitive. It saves the full context, sets `g_in_handler`, sets Status to (EXL=1, IE=0, EIE=0), sets a0..a3 and gp, uses the kernel stack, and sets `ra = kHandlerReturn`. It then loops `dispatch()` with 20000-cycle budgets until `pc == kHandlerReturn` and restores everything. Inside a handler, `reschedule()` is suppressed and `WaitSema` warns and returns. The same primitive runs alarm callbacks and custom syscalls installed through `SetSyscall`.

---

## 2. EE kernel HLE (`runtime/src/kernel.cpp`, 876 lines)

State: `std::map<int,Thread> g_threads` (ids 1..255, lowest free), semaphores (ids 1..255), `vector<Handler>` for INTC and DMAC, `map<int,Alarm>` (ids 1..63), and a `g_order` counter for FIFO order within a priority. Scheduling is strict priority (lower number wins); ties go to the lowest `order` (`best_ready()`). `make_ready()` sets `order = ++g_order`.

`handle_syscall(c, pc)`: n = `$v1`. A negative n is the interrupt variant (`n = -n`). If a custom handler was installed in the table at `0x1800 + n*4` through `SetSyscall`, it is run with `run_guest`. The full list:

| # | name | behaviour |
|---|---|---|
| 0x02 | GsSetCrt | records (interlace, mode, field); `pal = (mode == 3)` unless forced |
| 0x04 | Exit | halt |
| 0x06/0x07 | LoadExecPS2/ExecPS2 | halt ("program asked to run another executable") |
| 0x10/0x11 | Add/RemoveIntcHandler | `add_handler`: `next == 0` puts the handler at the head, `next == -1` at the tail, otherwise before handler id `next`; captures the caller's gp; returns a handler id |
| 0x12/0x13 | Add/RemoveDmacHandler | same |
| 0x14/0x1A, 0x15/0x1B | (i)Enable/DisableIntc | `hw::intc_set_mask`; returns 1 if the mask changed |
| 0x16/0x1C, 0x17/0x1D | (i)Enable/DisableDmac | sets/clears `D_STAT` bit 16+ch |
| 0x18/0x1E/0xFC/0xFE | (i)SetAlarm | due = now + (a0 & 0xFFFF) * hblank_cycles; callback(id, now/hblank, arg) with gp |
| 0x19/0x1F/0xFD/0xFF | (i)ReleaseAlarm | erase |
| 0x20 | CreateThread | param block: +4 entry, +8 stack, +12 size, +16 gp, +20 prio, +28 attr, +32 option; inherits heap |
| 0x21 | DeleteThread | erase |
| 0x22 | StartThread | ctx: pc=entry, a0=arg, gp, sp = stack+size-0x2A0, ra = kIdle; ready, reschedule |
| 0x23/0x24 | ExitThread/ExitDeleteThread | dormant (or delete), reschedule |
| 0x25/0x26 | (i)TerminateThread | dormant; removed from semaphore wait queues |
| 0x29/0x2A | (i)ChangeThreadPriority | prio & 127; returns the old prio |
| 0x2B/0x2C | (i)RotateThreadReadyQueue | bumps order |
| 0x2D/0x2E | (i)ReleaseWaitThread | waiter gets -418 |
| 0x2F | GetThreadId | |
| 0x30/0x31 | (i)ReferThreadStatus | writes a 48-byte struct |
| 0x32 | SleepThread | consumes a wakeup or blocks |
| 0x33/0x34 | (i)WakeupThread | |
| 0x35/0x36 | (i)CancelWakeupThread | |
| 0x37-0x3A | (i)Suspend/ResumeThread | counted suspends, WaitSuspend state |
| 0x3C | SetupThread (InitMainThread) | stack = (a1 == -1 ? RAM-0x1000 : a1+a2); copies boot argv into a3; returns top - 0x2A0 |
| 0x3D | SetupHeap (InitHeap) | heap = (a1 == -1 ? stack : a0+a1) |
| 0x3E | EndOfHeap | |
| 0x40 | CreateSema | +0 count, +4 max, +8 init, +16 attr, +20 option; count = init |
| 0x41 | DeleteSema | waiters get -425 |
| 0x42 | SignalSema | wakes the FIFO head (result = id) and reschedules; otherwise count++ |
| 0x43 | iSignalSema | same, but no immediate reschedule (`g_reschedule` is picked up later) |
| 0x44 | WaitSema | count-- or block (`kWaitSema`) |
| 0x45/0x46 | (i)PollSema | -1 if count is 0 |
| 0x47/0x48 | (i)ReferSemaStatus | |
| 0x4A/0x4B | Set/GetOsdConfigParam | one stored word |
| 0x5B/0x6B | GetEntryAddress / SifStopDma | return 0 |
| 0x64/0x68 | (i)FlushCache | no-op |
| 0x70/0x71 | GsGetIMR/GsPutIMR | through `gs::read/write_privileged(0x12001010)` |
| 0x73 | SetVSyncFlag | stores (flag ptr, data ptr); at the next VBlank start writes `*flag = 1` and `*data = GS_CSR` (`on_vblank_start`) |
| 0x74 | SetSyscall | table at 0x1800 (n < 0x100 only) |
| 0x76 | SifDmaStat | `sif::dma_stat` |
| 0x77 | SifSetDma | `sif::set_dma` + `request_iop_sync()` (cuts the budget to 64 so the IOP runs soon) |
| 0x78 | SifSetDChain | the interrupt variant (`isceSifSetDChain`) signals `sif::packet_processed()` |
| 0x79/0x7A | SifSetReg/SifGetReg | |
| 0x7C | Deci2Call | functions 1..6 return -1 (no DECI2) |
| 0x7E | MachineType | 0 |
| 0x7F | GetMemorySize | 32 MiB |
| other | | logs "[sys] unhandled syscall", returns 0 |

**Not implemented:** `ResetEE` (0x01), `SetGsIMR`-style aliases other than 0x71, `_ExceptionEpilogue`, `EnableCache`/`DisableCache` (they fall through to the default "return 0"), `ExecOSD`, and `SetCPUTimerHandler` (0xFC-0xFF are used as alarm aliases). SSX3 may call `ResetEE` during crt0. It is harmless if it returns 0.

COP0 (kernel.cpp, end of file):
- `mfc0 $9` (Count) returns `now()`. `$15` (PRId) returns `0x2E20`.
- `eret` clears ERL or EXL and returns ErrorEPC or EPC.
- `ei`/`di` toggle Status.EIE (bit 16).
- The boot Status is `0x70010C01`.

INTC/DMAC/timer registers (`runtime/src/hw.cpp`):
- `INTC_STAT` reads return `stat | latched`. Writes clear bits by writing 1. The latched copy keeps a cause visible to code that polls `INTC_STAT` (for example `sceGsSyncV`) after the kernel has acked it.
- `INTC_MASK` writes toggle bits (XOR).
- `D_STAT` writes clear bits 0-15 by writing 1 and toggle bits 16-31.
- EE timers 0-3 run from computed time: `count = (now - base) / div` with div = 2/32/512/hblank. Compare and overflow events are scheduled exactly and raise INTC 9+n. Timer 3 boots in mode 0x183.
- SIF registers sit at `0x1000F200 + 0x10*k`, k = 0..3, i.e. MSCOM, SMCOM, MSFLG, SMFLG (`sif::get_reg(k+1)`).
- `0x1000F520` (D_ENABLER) mirrors `0x1000F590` (D_ENABLEW).
- Any other register is backed by a `std::map`, and the first access is logged.
- 128-bit FIFO writes go to VIF0 (`0x10004000`), VIF1 (`0x10005000`), GIF (`0x10006000`) and the IPU (`0x10007010`). Reads from the VIF1 FIFO return GS download data.

INTC causes raised: 0 GS, 2 VBlank start, 3 VBlank end (22 hblanks later), 4/5 VIF0/VIF1 (the interrupt bit in a VIFcode), 9-12 timers. HBlank (cause 1) is
**never** raised: `kernel::on_hblank` is declared but has no implementation.

---

## 3. Timing model (`runtime/src/system.cpp`)

- `kEeClock = 294,912,000`. `frame_cycles = clock/50` (PAL) or `/60` (NTSC). `hblank_cycles = frame*2/625` (PAL) or `frame*2/525` (NTSC). One VBlank per "frame", i.e. per field.
- Time is `now() = g_base + (g_given - c.budget)`. The guest only advances time by burning budget, at roughly 1 cycle per instruction plus 2 per branch.
- Events live in a `vector<Event{id,at,action}>`; `schedule(id,...)` replaces any event with the same id. `refill()` gives a slice of `min(20000, next_event - now)`.
- The `vblank()` event:
  - `++frames` and input presses
  - `hw::vblank_start()`: INTC 2, `gs::vblank` (toggles the odd field and sets CSR bit 3, VSINT), schedules INTC 3 at +22 hblanks
  - `iop::vblank(true)` (IOP IRQ 0, and IRQ 11 for the end, 22 hblanks later)
  - `kernel::on_vblank_start()` (SetVSyncFlag)
  - re-arms itself after `frame_cycles`
- The IOP runs at **EE/8 = 36.864 MHz** (`iop::kClockDivider`). `sync_iop()` runs it up to the EE's `now()` every time `run_due_events()` runs, i.e. at every preempt check, then calls `sif::pump()`.
- The IOP interpreter itself slices at 512 cycles (`iop/kernel.cpp kSlice`), 1 cycle per instruction.
- Practical granularity: the EE and IOP interleave at most every 20k EE cycles, and earlier whenever an event or SIF DMA is due.

---

## 4. DMAC (`runtime/src/dmac.cpp`, 469 lines)

**Every transfer completes synchronously on the CHCR.STR write** (`start(ch)`). Then `finish(ch)` clears STR and calls `hw::dmac_raise(ch)`, which sets `D_STAT.CIS`. The handler runs if CIM is set. There is no bandwidth model, no cycle stealing, and no D_PCR priority.

| ch | name | handling |
|---|---|---|
| 0 | VIF0 | normal/chain → `vif::transfer(0, ...)` |
| 1 | VIF1 | normal/chain → `vif::transfer(1, ...)`. A normal transfer with DIR=0 (from device) reads GS image data with `gs::read_image`. MFIFO-aware |
| 2 | GIF | normal/chain → `gif::transfer(3, ...)` (PATH3) |
| 3 | fromIPU | `ipu::dma_started`, pulled through `ipu_push` |
| 4 | toIPU | the IPU pulls with `ipu_pull` (does its own chain-tag walk: cnt/next/ref/refs/refe/end) |
| 5,6,7 | SIF0/1/2 | **finish immediately**. SIF is fully HLE'd through syscalls, so the EE's own SIF DMA register writes are no-ops |
| 8 | fromSPR | normal or chain (tags read from SPR at SADR, `run_from_spr_chain`); writes into the MFIFO ring when `D_CTRL.MFD == 2` |
| 9 | toSPR | normal: RAM → SPR at SADR |

Chain tags (`run_chain`), with `addr = tag[63:32] & 0x7FFFFFF0`, bit 63 = SPR select:
- 0 **refe**: data at addr, end.
- 1 **cnt**: data follows the tag; next tag after the data.
- 2 **next**: data follows the tag; next tag at addr.
- 3/4 **ref/refs**: data at addr; next tag at tadr+16.
- 5 **call**: pushes data+qwc*16 onto ASR0/1 (CHCR.ASP); next tag at addr.
- 6 **ret**: pops ASP, or ends if ASP = 0.
- 7 **end**: data follows the tag, then stops.
- `TIE && tag.IRQ` also ends the chain.
- `TTE` (VIF0/1 only) sends the tag's upper 64 bits to the VIF as two VIFcodes.
- Stall: if `vif::stalled(ch)` is set after a delivery (VIFcode i-bit or MARK stall), the channel pauses with `paused/pause_end`. `vif::write(FBRST.STC)` later calls `dmac::resume(ch)`.
- MFIFO: `ring(a) = RBOR | (a & RBSR)`. VIF1 waits (`g_mfifo_waiting`) when `ring(tadr) == ring(ch8.madr)`. `finish(8)` resumes it.

Unsupported: VIF1/GIF interleave and stall control (D_CTRL.STS/STD), `D_SQWC` interleave, and any timing.

---

## 5. VIF (`runtime/src/vif.cpp`, 405 lines)

A `Unit` holds: STAT, ERR, MARK, CYCLE, MODE, NUM, MASK, CODE, ITOPS, BASE, OFST, TOPS, ITOP, TOP, ROW[4], COL[4], DBF, stalled, and a `pending` word queue for commands split across DMA packets. `run()` executes a command only once all of its data words are present (`data_words()`).

| op | command | implementation |
|---|---|---|
| 00 | NOP | |
| 01 | STCYCL | cycle = imm (CL = low byte, WL = high byte) |
| 02 | OFFSET | ofst = imm & 0x3FF, DBF = 0, TOPS = BASE |
| 03 | BASE | |
| 04 | ITOP | itops |
| 05 | STMOD | mode = imm & 3 |
| 06 | MSKPATH3 | `gif::set_path3_masked` (only reported in GIF_STAT.M3P; PATH3 is **not** actually held back) |
| 07 | MARK | |
| 10/11/13 | FLUSHE/FLUSH/FLUSHA | **no-ops**, because the VU runs synchronously |
| 14/15 | MSCAL/MSCALF | `run_vu(unit, imm*8)` |
| 17 | MSCNT | run from `vu::tpc` |
| 20 | STMASK | |
| 30/31 | STROW/STCOL | |
| 4A | MPG | copies `num` 64-bit words to micro memory at imm*8; `vu::code_changed()` only when bytes actually differ |
| 50/51 | DIRECT/DIRECTHL | `gif::transfer(2, ...)` (PATH2), imm qwords (0 means 65536) |
| 60-7F | UNPACK | see below |

The VIFcode IRQ bit (bit 31) sets STAT.VIS|INT (`0xC00`) and raises INTC 4+unit unless ERR.MII is set.

`run_vu` for VIF1: `top = tops; itop = itops; dbf = !dbf; tops = base + (dbf ? ofst : 0)`, then `vu::set_tops` and `vu::run`.

**UNPACK** (`unpack()`):
- vn = bits 26-27 (1-4 components), vl = bits 24-25 (32/16/8 bits, or 5551 when vn = 3 and vl = 3).
- Bit 14 is USN (zero-extend; otherwise sign-extend). Bit 15 adds TOPS (VIF1 only). Bit 28 is MASK.
- num 0 means 256. Addresses wrap at 256 qwords (VU0) or 1024 qwords (VU1).
- S-x formats broadcast the value. V2 sets z = x and w = y. V3 sets w = 0.
- V4-5 expands to `r<<3, g<<3, b<<3, a<<7`.
- Skipping write (CL >= WL): `addr += CL - WL` after every WL writes. Filling write (WL > CL): cycles at or beyond CL write ROW/COL through a forced mask of 1.
- Mask: 2 bits per lane, row = min(cycle, 3). The values are 0 data, 1 ROW, 2 COL, 3 protect.
- MODE 1 is offset (+ROW). MODE 2 is difference (ROW += value).
- Fast path: templated `unpack_fast<Bits, Components, Usn>` when there is no mask, MODE 0, no fill, and not V4-5.
- Word count: `ceil(data_elems * (vn+1) * bits / 32)`, where `data_elems` accounts for the WL > CL fill.

Registers: FBRST bit 0 resets the unit. Bit 3 (STC) clears the stall, re-runs the pending words, and resumes DMA.

**Threading trick:** with the VU thread active, VIF1 data is posted to a ring (`gs::queue::kVif1`). The EE thread runs a "shadow" VIF1 that only tracks register state, does UNPACK only for MODE 2 (into the `g_dry_run` buffer, to keep ROW correct), and scans DIRECT data for SIGNAL/FINISH/LABEL (`gif::scan_signals`). The real VIF1/VU1/GS work happens on another thread.

---

## 6. VU microcode (`runtime/src/vu_micro.cpp` 997 lines, `include/ps2/vu_ops.hpp` 325, `include/ps2/vu.hpp` 172)

Execution is **synchronous and run-to-completion** inside MSCAL. It is capped at 1,000,000 steps and logs "did not end" if the cap is hit. There are three tiers:

1. **AOT C++** (`recomp/vurecomp.cpp`, 632 lines). During exploring runs (`PS2_VU_RECORD`), each VU1 program reached is dumped as a 16 KiB code image named `<key>_<entry>.bin`. `key` is an FNV-1a hash over the PCs reachable from the entry and their instruction words (`vu::reachable`, `vu::program_key` in vu_ops.hpp). vurecomp emits one C++ function per program, and `register_programs()` registers them. At run time, `lookup(pc)` caches up to 8 candidates per entry; each is validated by comparing the current code words at its PCs (`candidate_matches`). A generation counter is bumped on MPG changes.
2. **WASM JIT** (`runtime/src/vu_jit.cpp`, 1496 lines). This emits a WebAssembly module at run time for programs not seen in the AOT set. It is browser-only and not useful to us. `check_jit` samples 1 of every 32 calls and compares the result against the AOT version.
3. **Interpreter** `run_fast()`. Decodes are cached per PC (`Decoded` keyed on the lo/hi words). Flags are computed only when `scan_flags` found a flag reader. `run()` is the slower exact variant (`PS2_VU_EXACT`), and VU0 micromode (`vcallms`) always uses `run()`.

Semantics:
- Per bundle: compute the **upper result first into a temporary** (`UpperOut`). Then execute the lower op, which still sees the pre-upper registers. Then commit the upper result.
- The I-bit (hi bit 31) loads the lo word into `I` instead of running a lower op. The E-bit (hi bit 30) ends the program after the **next** bundle (the delay slot), and `tpc` is saved.
- After every bundle, VF0 is forced to (0,0,0,1) and VI0 to 0.
- Branches have a single delay slot (`npc`), with targets `pc + 8 + imm11*8`. `JR`/`JALR` jump to `vi*8`, and links store `(pc+16)/8`. VI registers are read immediately; the hardware's VI-after-load delay is not modelled.
- **No pipeline stalls** and no Q/P latency. DIV, SQRT, RSQRT and all EFU ops write `Q`/`P` immediately. WAITQ (0x3B) and WAITP (0x7B) are no-ops. M, D and T bits are ignored.
- **The flag pipeline is modelled** (`vuops::FlagPipe`): an 8-entry history of MAC and status values and of CLIP values, each timestamped by bundle count. `FMxxx`/`FSxxx` read the newest MAC/status at least **4** bundles old. `FCxxx` read the newest clip at least **3** bundles old. The GoW2 fix was 4 to 3. `FSSET` and `FCSET` are back-dated with `t-4` so they are visible immediately.
- Float rules (`vuops::fin/fout`): inputs are denormal → ±0 and Inf/NaN → ±max. Outputs flush the same way. Arithmetic uses host IEEE, so round-to- nearest instead of the PS2's truncation, and the multiplier result is inexact. MADD and MSUB normalise the product before the add.
- MAC flags per lane are bits Z (0-3), S (4-7) and O (12-15). Status is Z, S, U, O plus sticky copies <<6.
- CLIP: `vuops::clip_value` shifts in 6 new bits.
- Upper ops: ADD/SUB/MADD/MSUB/MAX/MINI/MUL in bc/q/i/vector forms, OPMULA/OPMSUB, ITOF/FTOI with 0/4/12/15 fraction bits, ABS, CLIP and the ACC-target variants (`upper()`, vu_micro.cpp around line 90).
- Lower ops (`lower()`): IADD, ISUB, IADDI, IAND, IOR, IADDIU, ISUBIU, MOVE, MR32, LQI, SQI, LQD, SQD, LQ, SQ, ILW, ISW, ILWR, ISWR, DIV, SQRT, RSQRT, MTIR, MFIR, RNEXT, RGET, RINIT, RXOR, MFP, XTOP, XITOP, XGKICK, the EFU set (ESADD, ERSADD, ELENG, ERLENG, EATANxy, EATANxz, ESUM, ESQRT, ERSQRT, ERCPR, ESIN, EATAN, EEXP, implemented with host libm), the FC*/FM*/FS* family, B, BAL, JR, JALR, IBEQ/NE/LTZ/GTZ/LEZ/GEZ.
- **XGKICK** (`xgkick()`) walks GIF tags in VU1 memory up to EOP and calls `gif::transfer(1, buf, qwc)` (PATH1) synchronously. There is no PATH arbitration.
- VU0 macro mode (COP2 in EE code) is inlined from `vu.hpp`/`vu_ops.hpp` into the generated C++. `vcallms` runs VU0 micro code synchronously. `bc2t/f` use `vu0_busy`.

Threading: when enabled, VIF1 commands and therefore VU1 run on a dedicated VU thread (`run_vu_thread`, system.cpp). `PS2_ONE_THREAD=1` disables it. Save states use a fence (`kVuFence`).

---

## 7. GIF (`runtime/src/gs/gif.cpp`, 215 lines)

- `Path{active, nloop, loop, nreg, reg, flg, regs, eop}` is kept per path, for path indices 1, 2 and 3 (index 0 unused).
- `walk()` parses GIFtags. NREG = 0 means 16. PRE (bit 46) writes PRIM from tag bits 47-57, PACKED mode only.
- PACKED goes to `gs::write_packed(reg, q)`. REGLIST sends two 64-bit writes per qword. IMAGE and the disabled mode (flg 2/3) go to `gs::write_image`.
- A tag with NLOOP = 0 is consumed with no data. RGBAQ.Q is reset to 1.0 on each tag.
- `write_packed`:
  - 0x0 PRIM
  - 0x1 RGBAQ, using the Q latched by the last ST
  - 0x2 ST, which latches Q from word 2
  - 0x3 UV (14-bit)
  - 0x4 XYZF2: Z from `w[2] >> 4`, F from `w[3] >> 4`; ADC (bit 111) selects XYZF3
  - 0x5 XYZ2/3
  - 0xA FOG
  - 0xC/0xD XYZF3/XYZ3
  - 0xE A+D
  - 0xF NOP
- **PATH arbitration and masking:** transfers are processed immediately in call order. `GIF_STAT` returns only M3P plus a fake FQC = 16 while PATH3 is masked, because games poll FQC. `GIF_CTRL.RST` resets the paths.
- When the GS runs on its render thread, the EE side only `scan()`s tags for SIGNAL/FINISH/LABEL (A+D registers 0x60-0x62) and transfer-register writes (0x50-0x53, used for download caching), then posts the data to the ring.

---

## 8. GS (`runtime/src/gs/gs.cpp` 1148, `raster.cpp` 474, `gpu/gpu.cpp` 1860, `gpu/gs.wgsl` 1144)

### Register state
- `gs::State` holds PRIM, PRMODE, PRMODECONT, RGBAQ, ST, UV, FOG, TEXCLUT, SCANMSK, TEXA, FOGCOL, DIMX, DTHE, COLCLAMP, PABE, BITBLTBUF, TRXPOS, TRXREG, TRXDIR, a `Context ctx[2]` with TEX0/CLAMP/TEX1/TEX2/XYOFFSET/MIPTBP1/2/ SCISSOR/ALPHA/TEST/FBA/FRAME/ZBUF, a 3-vertex queue, and `clut[512]`.
- The vertex kick (`kick()`) needs 1, 2 or 3 vertices for point, line/strip or triangle. Sprites take 2.
- Strips shift the queue (`q0 = q1; q1 = q2`). Fans keep q0 (`q1 = q2`).
- XYZ2/XYZF2 draw. XYZ3/XYZF3 only push a vertex.
- TEX2 merges into TEX0 under mask `0xFFFFFFE0_03F00000` and reloads the CLUT.
- TEXFLUSH flushes the GPU path.
- SIGNAL, FINISH and LABEL set CSR bits 0, 1 and LABEL, and raise INTC 0 unless IMR masks them (`gs::raise`).
- Privileged registers handled: PMODE, SMODE1/2, SRFSH, SYNCH1/2, SYNCV, DISPFB1/2, DISPLAY1/2, EXTBUF, EXTDATA, EXTWRITE, BGCOLOR, CSR (write-1-to-clear the low 5 bits, RESET → `0x551B4000`, FIELD bit 13 toggles per VBlank), IMR (reset `0x7F00`), BUSDIR, SIGLBLID.

### VRAM swizzle (`build_tables`, gs.cpp:77)
- Precomputed per-page offset tables:
  - `g_off32[32][64]`, `g_offz32`
  - `g_off16[64][64]`, `g_off16s`, `g_offz16`, `g_offz16s`
  - `g_off8[64][128]`, `g_off4[128][128]` (nibble index)
- They are built from the block tables `kBlock32`, `kBlockZ32`, `kBlock16`, `kBlock16S`, `kBlockZ16`, `kBlockZ16S` and the column tables `kColumn32`, `kColumn16`, `kColumnWord`.
- Page sizes: 32-bit formats 64x32, 16-bit 64x64, 8-bit 128x64, 4-bit 128x128. The page address is `bp*256 + ((x/pw) + (y/ph) * max(1, bw*64/pw)) * 8192`, in bytes.
- `read_pixel`/`write_pixel` (gs.cpp:365/388) cover:
  - PSMCT32, CT24 (keeps the high byte), CT16, CT16S
  - PSMZ32, Z24, Z16, Z16S
  - PSMT8, PSMT4 (nibble)
  - PSMT8H (bits 24-31 of a CT32 word), PSMT4HL (24-27), PSMT4HH (28-31)
- These tables are self-contained and **directly portable**. They also drive the WGSL shader (`swizzle_tables()` uploads them).

### Transfers
- TRXDIR 0 (host → local) runs through `feed_image`/`transfer_pixel`, bit-packed per PSM, with 24-bit carry handling.
- TRXDIR 1 (local → host) runs through `render_read_image`, which VIF1/ch1 reads use.
- TRXDIR 2 (local → local) is `local_copy`.
- The GPU path keeps a download cache keyed on BITBLTBUF/TRXPOS/TRXREG/offset so the EE does not block on GPU readback (`read_image`).

### CLUT (`load_clut`, gs.cpp:296)
- CLD 0-5 semantics, with CBP0/CBP1 compare for CLD 4/5.
- CSM1: an 8-bit CLUT is read as a 16x16 block with the bit-3/bit-4 swap (`j = (i & ~0x18) | ((i & 8) << 1) | ((i & 0x10) >> 1)`). A 4-bit CLUT is an 8x2 block. The CLUT buffer holds 512 entries, and for 4-bit formats CSA selects a 16-entry slice.
- CSM2: reads linearly from (COU*16 + i, COV) with CBW from TEXCLUT.
- CPSM is CT32, CT16 or CT16S. 16-bit entries expand through TEXA at sample time.

### Software rasterizer (`raster.cpp`; the reference the GPU path is diffed against with `PS2_GS_COMPARE`)
- `make_setup` decodes PRIM, or PRMODE when PRMODECONT = 0, plus the context.
- Triangles:
  - Bounding box clipped to SCISSOR. Edge functions use a top-left fill rule (`edge_in`).
  - Sample points are integer pixel centres at x, y (no +0.5). Coordinates are `(X - OFX) / 16`.
  - Attributes are linear barycentric. Perspective comes only from dividing S/Q and T/Q per pixel. Z is interpolated in double. Flat shading uses the last vertex's colour.
- Sprites: axis-aligned, with S/T interpolated across. Lines use a DDA. Points are supported.
- Texturing: wrap modes REPEAT, CLAMP, REGION_CLAMP and REGION_REPEAT (`(v & min) | max`). Nearest or bilinear (TEX1 bit 5 only, which is MMAG). **No mipmapping or LOD** (MIPTBP is stored, not used).
- TFX MODULATE, DECAL, HIGHLIGHT and HIGHLIGHT2 (`(t*c) >> 7`), with TCC. TEXA handles AEM, TA0 and TA1 for 24- and 16-bit textures.
- Fog: `(f*c + (255-f)*fogcol) >> 8`.
- Alpha test: ATST 0-7 and AFAIL KEEP / FB_ONLY / ZB_ONLY / RGB_ONLY.
- Destination alpha test: DATE/DATM on bit 31, or bit 15 for 16-bit formats.
- Depth test: ZTST NEVER/ALWAYS/GEQUAL/GREATER, with Z clamped to the format max.
- Alpha blend: `((A - B) * C >> 7) + D` with A/B/D in {Cs, Cd, 0} and C in {As, Ad, FIX}. PABE gates it on As bit 7. COLCLAMP clamps, otherwise the result is masked with `& 0xFF`.
- FBA ORs 0x80 into alpha. FBMSK is honoured, including the 16-bit pack.
- SCANMSK: modes 2 and 3 skip even or odd rows.
- **Not implemented:** dithering (DTHE is stored, unused), mipmaps, TEX1.MMIN and LCM, DIMX, AA1 (antialiasing), and FST rounding nuances.
- The software path is slow: it uses per-pixel `read_pixel` and floats, and is only usable for menus.

### GPU rasterizer (WebGPU compute)
- 4 MiB of GS memory lives in a GPU storage buffer in native swizzled layout, with `scale`x supersampling (1-3).
- Pipelines: `upload`, `copy_read`, `copy_write`, `load_clut`, `display`, `raster`, `resolve`, `snapshot`, `decode` (gs.wgsl).
- Triangles are binned on the CPU into **16x16 tiles** (`flush_batch`, gpu.cpp:524). One workgroup of 16x16 threads runs per tile (`raster`, gs.wgsl:1106). The shader `plot()` mirrors `raster.cpp`.
- Batches are split on read-after-write conflicts (texture reads of a frame being rendered, `batch_conflict`). Uploads that do not conflict are hoisted before the batch. A 16 MB upload content cache (`cache_find`) serves repeated uploads.

### Presentation (`gs::display`, `present`, `copy_frame`)
- Circuit 2 is used if PMODE.EN2, otherwise circuit 1. Only **one circuit**; there is no merge circuit, alpha blend, or BGCOLOR.
- From DISPFB: FBP*32 (blocks), FBW, PSM, DBX, DBY. From DISPLAY: `width = (DW+1)/(MAGH+1)` and `height = (DH+1)/(MAGV+1)`.
- If SMODE2 has INT = 1 and FFMD = 1 (`(smode2 & 3) == 3`), the frame is read at half height (`y/2`) and `copy_frame` blends the current frame with the previous one (`(a & b) + ((a ^ b) & 0xFEFEFEFE) >> 1`) as a deinterlace.
- CT16 is expanded to RGB888. Output alpha is forced to 0xFF.

---

## 9. IOP

### CPU (`runtime/src/iop/cpu.cpp`, 210 lines)
- A plain R3000A interpreter (`execute(Cpu&, int64_t &budget)`) with branch delay through `pc/npc`. It does **not** model load delay slots.
- Opcode **0x3F** is the HLE trap: `call_hle(c, imm26)`.
- RAM is 2 MiB (`iop::g_ram`). IOP scratchpad is at `0x1F800000`.

### Kernel and module loading (`runtime/src/iop/kernel.cpp`, 814 lines)
- **No rom0 modules are emulated as code.** The IOP kernel and every BIOS library is C++ HLE. `kernel::boot()` installs the libraries (`install_libraries()` in libs.cpp, then `install_io/cdvd/sif`).
- The game's own **disc** IRX modules run on the interpreter: sound driver, SIO2MAN, PADMAN, MCMAN, MCSERV, LIBSD, cdvd streaming.
- Loading (`load_module_buffer`): parses the ELF and the `.iopmod` program header (type 0x70000080, with entry, gp and text/data/bss sizes). It allocates with `kernel::alloc` (first fit, user memory from 0x20000, 256-byte granularity), relocates (`relocate`: R_MIPS_32, 26, HI16/LO16, with IRX v2 0xFF81 paired HI/LO), and links imports (`link_imports`).
- Import linking scans for the import table magic `0x41E00000`. For each stub pair `jr ra; addiu zero, zero, N`:
  - If the library was exported by a guest module through `RegisterLibraryEntries`, the stub is patched to `j export[N]`.
  - Otherwise it becomes `0xFC000000 | hle_id`, auto-registering a "missing `<lib>` #N" logger that returns 0 if no HLE exists.
- The module entry runs as a new thread (prio 8, 16 KiB stack) with argc/argv. On exit, a result of `(result & 3) == 1` (NO_RESIDENT_END) frees the module.
- Loading a `rom0:` path with no disc data fails with -1 unless a `register_hle_module(name, ...)` exists, and **none are registered**. So `rom0:SIO2MAN`, `rom0:PADMAN` and similar would fail. GT4, GoW2 and **SSX3** load their pad, memcard and sound IRX from `cdrom0:\` (SSX3's ELF strings include SIO2MAN.IRX, PADMAN.IRX, MCMAN.IRX, MCSERV.IRX, LIBSD.IRX, SNDDRV.IRX and MSIFRPC.IRX).
- IOP reboot (EE `SifIopReset`, a SIF packet to dst 0) is just logged by `sif.cpp reboot()`. An `IOPRP` image switches fileio to the async (v2.x) protocol, and the LOADFILE version is read from the image (`PsIIloadfile`).
- Scheduler: same priority/order scheme as the EE. Context switches copy `Cpu`. `call_guest()` runs handlers and callbacks nested on interrupt stacks at 0x1F000 - 0x1000*depth, until the return stub runs. Interrupts are delivered via `deliver_interrupts()` between slices, unless the current thread is inside `CpuSuspendIntr`.
- Events: a priority queue of `Job{kind,a,b,c,d}` (serialisable) or closures. `usec_to_cycles(us) = us * 36864 / 1000`.

HLE libraries in `runtime/src/iop/libs.cpp` (1174 lines), plus `sif.cpp`, `cdvd.cpp` and `io.cpp`. Listed as export index:function:
- **sysmem**: 4 AllocSysMemory, 5 FreeSysMemory, 6 QueryMemSize, 7 QueryMaxFreeMemSize, 8 QueryTotalFreeMemSize, 9 QueryBlockTopAddress, 10 QueryBlockSize, 14 Kprintf
- **loadcore**: 4 FlushIcache, 5 FlushDcache, 6 RegisterLibraryEntries, 7 ReleaseLibraryEntries, 12 QueryBootMode, 27 SetRebootTimeLibraryHandlingMode
- **intrman**: 4 RegisterIntrHandler, 5 ReleaseIntrHandler, 6 EnableIntr, 7 DisableIntr, 8/9 CpuDisable/EnableIntr, 17/18 CpuSuspend/ResumeIntr, 23 QueryIntrContext
- **thbase**: 4-43 (Create/Delete/Start/StartArgs/Exit/ExitDelete/Terminate Thread, Disable/EnableDispatchThread, ChangeThreadPriority, Rotate, Release, GetThreadId, CheckThreadStack, ReferThreadStatus, Sleep/Wakeup/ CancelWakeup, Suspend/Resume, DelayThread, GetSystemTime, Set/CancelAlarm, USec2SysClock, SysClock2USec, GetSystemStatusFlag, GetSystemTimeLow)
- **thsemap** 4-12, **thevent** 4-14 (event flags with AND/OR/clear modes), **thmsgbx** 4-9, **thfpool** 4-9
- **sysclib**: setjmp/longjmp, ctype, mem*/str*, sprintf/vsprintf, strtol/ul, _wmemcopy/_wmemset
- **stdio**: printf/putchar/puts
- **modload**: 13, 16, 17, 18, 22
- **vblank**: 4-9 (WaitVblankStart/End, WaitVblank, WaitNonVblank, Register/ReleaseVblankHandler)
- **timrman**: 4-24 (hard timers driving IOP root counters; needed for GoW2's sound tick)
- **dmacman**: 16, 17, 28, 32-35
- **heaplib**: 4-7
- **secrman**: 4/5 return 0, 6 SecrAuthCard returns 1
- **ioman**: 4-8 and 16 (open/close/read/write/lseek/getstat), devices `cdrom0:` and `rom0:ROMVER` (returns "0220EC20060210")
- **cdvdman**: 4 Init, 5 Standby, 6 Read, 7 Seek, 8 GetError, 10 SearchFile, 11 Sync, 12 GetDiskType (returns 0x14, PS2 DVD), 13 DiskReady (returns 2), 14 TrayReq, 15 Stop, 22 RI, 24 ReadClock (BCD, JST = UTC+9), 28 Status, 37 Callback, 38 Pause, 39 Break, 74 PowerOff, 75 Mmode, 78 POffCallback, 83 ReadDvdDualInfo, 84 LayerSearchFile
- **sifman** and **sifcmd**: see the next subsection

### SIF (`runtime/src/iop/sif.cpp`, 855 lines): the most reusable protocol code
The EE side is **the game's own libkernel SIF code, recompiled**. Only the IOP side and the SIF hardware are HLE.
- **EE → IOP**: `SifSetDma` (syscall 0x77) → `sif::set_dma(list, n)`. Each `SifDmaTransfer{src, dst, size, attr}` is copied immediately into IOP RAM, except for three magic destinations:
  - `dst == 0xDEADBEF0` (`kRpcRecvAddr`, which bind replies hand out as the server's receive buffer) records `g_data_addr = src`: the RPC argument payload.
  - `dst == 0x1E000` (`kCmdBuffer`, returned for SIF reg 2 / SMCOM) means a SIF command packet, handled by `command(src)`.
  - `dst == 0` means an IOP reboot packet.
  - `SifDmaStat` reports busy until `256 + 2*bytes` cycles have passed.
- **IOP → EE**: packets are queued in `g_outgoing`. `pump()` delivers one packet at a time: memcpy into the EE receive buffer `g_ee_recv` (set by EE command 0x80000002, SET_EE_RECV_ADDR with opt 0), then `hw::dmac_raise(5)`, which raises the SIF0 DMAC interrupt. The game's EE SIF0 handler processes the packet and calls `isceSifSetDChain` (syscall -0x78), which calls `packet_processed()` and lets the next packet out.
- Registers: MSCOM (1) = `g_mainaddr`, SMCOM (2) = `0x1E000`, MSFLG (3) is OR-set, SMFLG (4) starts at `0x60000` (the SIF-init and CMD-init bits). Reads of `0x80000000` return 0 and `0x80000002` return 1.
- Command IDs handled:
  - 0x80000001 SET_SREG
  - 0x80000002 SET_EE_RECV_ADDR (opt != 0 makes the IOP send a SET_SREG command back)
  - 0x80000008 RPC_END (for IOP-side clients: writes the client struct, runs the end function, wakes the waiter)
  - 0x80000009 BIND: replies with RPC_END cmd 0x80000009 and `server_data = sid ^ 0xACACACAC`, buffer 0xDEADBEF0, cbuffer 0xDEADCAFE. If the server is not registered yet, the reply is held until `sceSifRegisterRpc`, or times out after 4 EE-seconds with server_data = 0
  - 0x8000000A CALL
  - 0x8000000C GET_OTHER_DATA
  - anything else goes to a guest handler from `sceSifAddCmdHandler`, with the packet copied to IOP 0x1E000
- **RPC call path**: `call()` decodes `server = server_data ^ 0xACACACAC`.
  - HLE server (`register_hle_server(id, fn)`): `fn(method, args_ee_addr, size, recv_ee_addr, recv_size)` returns true when finished, or false to finish later via `complete_call(id)`. The reply is RPC_END with cmd 0x8000000A.
  - Guest server: the argument bytes from EE `g_data_addr` are queued on the server's queue and wake the thread blocked in `sceSifRpcLoop`. The loop stub (`rpc_loop`) copies the args into the server buffer and calls `func(fn, buf, size)` with `ra = finish_stub`. `rpc_finish` copies `recv_size` bytes from the returned pointer into EE `recv`, replies, and loops.
- **HLE servers** (`install_servers`):
  - 0x80000006 LOADFILE: method 0 loads a module by path, 6 loads from an IOP buffer, 9 returns -202, 0xFF returns the version.
  - 0x80000001 FILEIO, in both sync and async/IOPRP variants: open, close, read, write (stub), lseek, getstat, devctl, using `iop::io::*` on the disc.
  - 0x80000003 IOP heap: alloc/free/query.
  - In `cdvdfsv.cpp`:
    - 0x80000592 CD init, which reports ready.
    - 0x80000593 S-commands: ReadClock, GetDiskType, error, trays, status, DVD dual info.
    - 0x80000595 N-commands: 1 sceCdRead into **EE RAM**, completed by a delayed job; 4/5/8; 9 streaming (`sceCdSt*`: start, read, stop, seek).
- IOP-side clients: `sceSifBindRpc`/`sceSifCallRpc` send packets the other way. They always ask for rmode 1 so the EE replies with RPC_END. The calling thread blocks unless `mode & 1`.

### CD/DVD (`runtime/src/disc.cpp`, `iop/cdvd.cpp`, `iop/cdvdfsv.cpp`)
- A raw 2048-byte-sector ISO through a `Reader(offset, out, size)` callback (`fopen`/`fread` natively, HTTP Range requests on the web).
- The ISO9660 directory tree is walked once (`walk`, `index_layer`), including DVD layer 1 found by probing for a second PVD (`layer1_start`). `disc::find(path)` looks up a path case-insensitively, ignoring `;1`.
- Reads finish after `400us + 250us/sector` (cdvdman) or `200us/sector` (cdvdfsv), as scheduled IOP jobs. The read then runs the callback and wakes `sceCdSync` waiters.

### Pad and memcard (`runtime/src/iop/sio2.cpp`, 439 lines)
- **SIO2 is emulated at the register level** (`0x1F808200-0x1F808280`, IOP DMA ch 11/12), so the game's own SIO2MAN and PADMAN/MCMAN from the disc drive it.
- Pad commands: 0x40, 0x41, 0x42 poll (buttons, analog, pressure, motors), 0x43 config, 0x44 set mode, 0x45 query model (DualShock 2), 0x46, 0x47, 0x4C, 0x4D motor map, 0x4F pressure mode. Modes are 0x41 digital, 0x73 analog, 0x79 pressure, 0xF3 config.
- Memory card: 528-byte pages, 0x4000 pages (8 MB, PCSX2 `.ps2` layout), commands 0x42/0x43 and friends, flushed to disk on VBlank.

### SPU2 (`iop/spu2.cpp`, 614 lines) and IPU (`ipu/ipu.cpp`, 716 lines)
- SPU2: 48 voices with ADPCM, ADSR, IRQ address, and core DMA for IOP ch 4/7 including AutoDMA streaming, ticked by IOP jobs.
- IPU: MPEG-2 I/P/B decode with BSD-2 VLC tables from Play!. SSX3 movies would need it.

---

## 10. Game bootstrap and per-game configuration

- `host/main.cpp`: `ps2run <ELF> --disc image.iso [--memcard f] [--pal|--ntsc] [--frames N] [--press F:btn] [--misses file] [--window]`, plus trace flags (`--trace-syscalls`, `--trace-iop`, `--trace-iop-hle`, `--trace-threads`, `--trace-io`).
- `load_elf()` copies PT_LOAD segments to physical addresses.
- `boot()`:
  - `hw::reset`, `kernel::reset`, `disc::open`, `iop::reset` (HLE IOP kernel boot; no IOP modules yet)
  - writes argv at EE 0x800 with `argv[0] = "cdrom0:\NAME;1"`
  - `kernel::start_main_thread(entry, gp = 0, argc, argv)`: thread 1 runs at `entry` with Status `0x70010C01`
  - schedules the first VBlank
- There is no BIOS and no OSDSYS. crt0 calls `SetupThread` and `SetupHeap` itself.
- **Game-specific code: none in the runtime.** The README says GoW2 needed "no game-specific code". The only per-game artefacts are:
  1. `games/<name>/entries.txt`: extra EE function entry points discovered at run time. `ps2run --misses` logs a PC with no translated code, and `scripts/port.sh` loops translate → run headless → append misses, for up to 8 rounds.
  2. `games/<name>/vu1/*.bin`: recorded VU1 programs for vurecomp.
  3. `scripts/gt4_core.py`: GT4 ships its real executable deflated in `CORE.GT4`; this script unpacks it into an ELF.
  4. SNR2 relocatable EE overlays (`recomp/snr2.cpp`, `--modules`). These are recompiled position-independently, and `dispatch.cpp instance_for()` recognises them in RAM by the magic `0x32524E53` plus checksum words.
- Exploring input defaults: start at frame 300, then cross every 300 frames. `PS2_*` environment variables are debug knobs only (`PS2_VU_INTERPRET`, `PS2_TRACE_DMA`, `PS2_GS=cpu|gpu|both`, `PS2_ONE_THREAD`, ...).

---

## 11. Size per subsystem (lines, from `wc -l` on .cpp/.hpp/.wgsl)

| subsystem | files | lines |
|---|---|---|
| EE recompiler (analysis, codegen, ELF, R5900 decoder, SNR2) | `recomp/*` | 1941 |
| VU1 AOT recompiler | `recomp/vurecomp.cpp` | 664 |
| EE kernel HLE | `runtime/src/kernel.*` | 905 |
| Scheduler, events, dispatch, save state glue | `system.cpp`, `dispatch.cpp`, `system.hpp` | 831 |
| EE hardware (INTC, timers, MMIO routing) | `runtime/src/hw.*` | 368 |
| DMAC | `dmac.*` | 491 |
| VIF | `vif.*` | 426 |
| VU interpreter, ops, flag pipe | `vu_micro.*`, `vu.hpp`, `vu_ops.hpp` | 1517 |
| VU WASM JIT (skip) | `vu_jit*` | 1663 |
| FPU, MMI, COP0 helpers | `fpu.hpp`, `mmi.hpp`, `cop.cpp` | 559 |
| GIF | `gs/gif.*` | 240 |
| GS core (regs, swizzle, transfers, CLUT, display) | `gs/gs.*` | 1241 |
| GS software rasterizer | `gs/raster.*` | 484 |
| GS WebGPU renderer (C++ and WGSL) | `gs/gpu/*` | 3112 |
| render/VU thread rings | `gs/queue.*` | 209 |
| IOP CPU interpreter | `iop/cpu.cpp` | 210 |
| IOP kernel, loader, scheduler | `iop/kernel.*`, `iop.hpp` | 1019 |
| IOP HLE libraries | `iop/libs.cpp` | 1174 |
| SIF (cmd, RPC, servers) | `iop/sif.*` | 883 |
| CD/DVD, cdvdfsv, ioman, ISO | `cdvd`, `cdvdfsv`, `io`, `disc` | 742 |
| SIO2 (pad and memcard) | `iop/sio2.*` | 463 |
| IOP hardware (INTC, DMA, OHCI stub) | `iop/hw.*` | 314 |
| SPU2 | `iop/spu2.*` | 637 |
| IPU (MPEG) | `ipu/*` | 907 |
| host window, audio, web glue | `host/*.cpp` | 1044 |
| **total** | | **~21.8k** |

The largest and most complex pieces are the GS GPU renderer (3.1k, not portable as-is), the VU (1.5k interpreter plus 0.66k AOT), the GS core (1.2k), the IOP HLE libraries (1.2k), the IOP kernel (1.0k), and SIF (0.9k). The whole non-GPU, non-JIT emulation core is about **13k lines**, a small and tractable port.

---

## 12. What ports directly to Rust (for `ssx3-runtime`, `ssx3-ps2`, `ssx3-gs`)

**Port nearly 1:1. These are pure state machines with no C++-isms beyond `std::map` and `std::function`:**
- The `kernel.cpp` syscall table and thread/semaphore/alarm/handler semantics, including the add_handler ordering, -418/-425 results, the 0x2A0 context reserve, and the `SetupThread` argv copy.
- The `run_guest` nested-handler pattern.
- `dmac.cpp` chain walker plus MFIFO, and `vif.cpp` (UNPACK including mask, mode and fill; the double-buffer TOPS logic).
- The `gif.cpp` walker and `write_packed` packing.
- The GS swizzle tables (`build_tables`, `read_pixel`/`write_pixel`), `load_clut`, and `raster.cpp` as a first-pixels CPU rasterizer.
- `vu_micro.cpp` `upper()`/`lower()` plus `vuops::FlagPipe` and `vuops::fin/fout`. Port these as an interpreter first; AOT later, using the same "record program → hash → compiled fn" keying.
- IOP: `cpu.cpp` (trivial), `kernel.cpp` (loader, relocations, import linking with the 0x3F HLE trap opcode), `libs.cpp`, `sif.cpp`, `cdvd*.cpp`, `io.cpp`, `disc.cpp`, `sio2.cpp`.
  - Interpreting the IOP is the right call. This repo shows that interpreting the disc IRX modules at 36.8 MHz is fast enough, and it makes "IOP recompile" unnecessary for us.

**Reuse the architecture, not the code:**
- Thread model: adopt the "host stack is a cache, `kUnwind` bit plus trampoline" design. It needs only three things from our Rust codegen:
  1. every call site checks `cpu.pc == expected_ret` after the callee returns and otherwise returns;
  2. `jr $ra` returns to the caller rather than dispatching;
  3. backward branches and syscalls poll a budget.

  In Rust without `musttail`, cross-unit jumps should go back through the dispatcher, or use a loop-plus-match per unit. Depth is capped at 192.
- Time: a single `now()` derived from the budget, a small event list, the IOP run lazily up to `now()` with EE/8, and VBlank every clock/60 (NTSC for SSX3).

**Skip or replace:** the WASM VU JIT, the Emscripten and browser glue, and the WebGPU renderer. Our `ssx3-gs` would either port `raster.cpp` (correctness first) or write a wgpu compute renderer modelled on gs.wgsl. gs.wgsl is WGSL and works with wgpu as is, so it could be reused almost verbatim, under MIT with attribution.

**Known gaps to expect if we copy them:**
- No HBlank interrupt.
- DMA, VU and GIF are instantaneous.
- No PATH3 masking or arbitration.
- No VU Q/P latency or stalls.
- No dithering or mipmapping.
- Single display circuit.
- No `ResetEE`.
- rom0 IRX unsupported (SSX3 loads its from disc).
- FLUSH* are no-ops.
- No IOP load-delay slots.
