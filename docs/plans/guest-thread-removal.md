# Guest thread removal

Goal (roadmap, "Final state: no guest threads"): the game runs as one loop the
port drives. No guest thread is created; the kernel's thread, semaphore and
alarm services are unused; `ssx3-ps2/src/threads.rs`, `corosensei` and the
`StackSwitcher` are deleted; every front end (the browser included) runs the
same code with no stack switching.

Work happens in a separate worktree (`../ssx3-decomp-threads`, branch
`feat/no-guest-threads`), based on `main` after the SCE library port (#78), so
`libcdvd`, `libpad`, `libmc`, `libdma`, `libipu` and the kernel/SIF units are
already hand-written Rust. Merge to `main` in steps; each step keeps the gate
(`GATE: race replay RAM identical to the baseline`) green.

## Why it is possible

The EE has one core and the game's threads run one at a time, deterministically
(`kernel.rs` implements the priority scheduler; `threads.rs` gives each thread a
host stack). A thread's work is a function; the thread is only the place where
it waits. Where each thread waits in one spot (the top of a `while (1)` loop on
a semaphore, an ISR signal, a delay) the thread can become a function the
signalling side calls, as long as the order of effects stays what the scheduler
produced.

## Known threads (from `docs/investigation/boot-path.md` §4)

| thread | entry | prio | waits on |
| --- | --- | --- | --- |
| main (`App_Run_MainLoop` 0x316d60) | crt0 | 1 then 100 | `PS2ExecMan_WaitFrame` (sema em+0x18) |
| `libkernl` helper | 0x424aa0 | 0 | sema (library internal) |
| power-off | 0x3198a0 | 1 | sema |
| `PS2ExecMan_WorkerThread` | 0x31ac60 | P+1 | sema em+0x14044 (jobs posted by the main thread, 0x31acd8) |
| `PS2ExecMan_VSyncThread` | 0x31ac08 | P-1 | sema em+0x4034, signalled from the VBlank-End ISR; runs `App_VSyncTick` -> `SSXApp_Tick` |
| `Renderer_DmaKickThread` | 0x382760 | 5 | semas r+0x5ac8 (flip, from VBlank-Start and VIF1 ISRs) and r+0x5acc |
| `SND` pump | 0x3c19a8 | 6 | VBlank / timer |
| libcdvd callback thread | (SDK) | 0xb | CD events |
| `EAFile_CdReadThread` | 0x3e3be0 | 0xc | sema 0x519c4c (read requests) |
| SIF-cmd/DMA device client | 0x3f44f8 | 10 | SIF command handshake |
| `EAThread_Create` threads | various | 0x61-0x67 | movie / streaming / misc (to inventory) |

Interrupt handlers that wake them: VBlank End (3) -> VSync sema; VBlank Start (2)
-> renderer flip sema; DMAC ch1 (VIF1) -> renderer; TIMER1 (10) -> `EATimer`
(callbacks run on the main loop via `EATimer_ServiceCallbacks`); IPU/SIF DMAC.

## Step 1: inventory (tooling first)

Tool: a syscall trace in the kernel HLE, `SSX3_THREAD_TRACE=file`, one line per
thread-relevant event: frame, cycle, running thread id, syscall (CreateThread,
StartThread, ExitThread, ChangeThreadPriority, Sleep/Wakeup/iWakeup,
Wait/Signal/iSignal/PollSema, DelayThread, RotateThreadReadyQueue,
SetAlarm/iSetAlarm), arguments, the guest caller (`ra` and the enclosing
function name), and every context switch (from, to, reason). `tools/thread_inventory.py`
turns a trace into `docs/investigation/thread-inventory.md`:

- per thread: creator, entry, priority, stack, lifetime (boot / menu / race / movie);
- per blocking site: the function that calls the wait, what it waits for (a
  semaphore with its signal sites, an ISR, a delay, a CD event), how often per frame;
- the per-frame switch sequence of a race frame (the schedule the conversion has to reproduce);
- the call tree each thread runs, from the call profile (`callprof`), with the
  generated (not yet hand-written) functions on it: **this is the port list for step 2**.

Runs: boot, the menu replay, one race (`SSX3_EVENT`), a movie, a save/load.

## Step 2: port what is on the thread paths

Every function a thread runs, and every function that signals it, must be
hand-written Rust before the thread can go: a blocking call inside a generated
function keeps a host stack alive. Mostly the "front end, graphics, sound" and
"engine, libraries, online" groups: `PS2ExecMan_*`, `EAFile_*` (queue, CD read,
BIG lookup), `EAThread_*`, `EATimer_*`, `Renderer_*` kick and flip, the `SND`
driver pump, libkernl wrappers. Ordinary porting loop (waves, writers,
differential tests), with the per-thread lists from the inventory instead of
the hot-function ranking.

## Step 3: convert thread by thread

Order: easiest first.

1. **Pure workers** (wait on a semaphore at the top of a loop, never block
   inside): power-off, `PS2ExecMan_WorkerThread`, `EAFile_CdReadThread` (native
   files are synchronous: the request is served where it is signalled or at the
   next scheduling point), SND pump.
2. **ISR-driven**: `PS2ExecMan_VSyncThread` (the VBlank-End ISR calls
   `App_VSyncTick` directly at the point the scheduler would have run it),
   `Renderer_DmaKickThread` (kick on the flip signal; the native renderer does
   not need the DMA state machine).
3. **Main thread and the rest**: `App_Run_MainLoop` becomes the frame function
   the port calls; `WaitFrame` returns after the tick ran; libkernl helper,
   libcdvd callback thread, SIF client disappear with their libraries' native
   replacements; `EAThread_Create` users case by case.

Mechanism for a converted thread: the thread's entry function is split into
`init` (once) and `step` (the loop body). Its create/start syscalls stop
creating a thread; the semaphore it waited on keeps a counter, and signalling it
queues a call of `step` that runs at exactly the point the priority scheduler
would have switched to it (immediately in the signaller if the signalled thread
had higher priority and interrupts were enabled; at the signaller's next block
or end of frame otherwise). The gate (RAM identical) and a per-frame order
trace (`SSX3_THREAD_TRACE` on both builds, compared) prove the order is kept.
Once all threads are converted, the semaphore / thread / alarm syscalls are
unreachable and are removed with `threads.rs`, `threads_switcher.rs`,
`corosensei` and the generated trampoline code only they need.

## What the trace says (decided design)

Measured on the gate replay (`../investigation/thread-inventory.md`,
`SSX3_TRACE_SYSCALLS=1 SSX3_TRACE_THREADS=1`, which now also logs every block):

- Seven threads are created (ids 2-8) plus the main thread. In a race frame
  only three run: the VSync thread 6 (the game tick, woken by the VBlank-End ISR),
  the SND thread 8 (woken by the SIF command ISR, twice a frame) and main (1).
  Threads 2, 3, 4 and 7 run once at boot and never again; worker 5 runs the
  loading jobs.
- **Nesting holds.** Every one of the 1.27 million switches in the replay is
  last-in-first-out: a thread is preempted by a higher-priority one that runs to
  its block point, then the preempted thread continues. The worker and main
  share priority 100 and switch only cooperatively (`EAThread_Yield`).
  A simulation over the trace finds no switch that resumes a preempted thread
  other than the top of the stack.
- **Mutexes block rarely, and once it matters.** Almost all `EAMutex_Lock`/`SND_MutexLock`
  calls find the semaphore free. The SND mutex (semaphore 24) blocks 7 times in the
  replay: the SND thread (8) preempts main inside main's SND critical section, blocks on
  the mutex, main finishes and unlocks, and thread 8 continues in the middle of its body.
  That is the one non-LIFO case (see "Mid-body blocks").
- These are all the blocks: the thread loop semaphores (VSync 18, SND 25, main's
  frame sema 16, worker 17), main's `EAFile_DelayTicks` alarm waits (sema 13,
  20 times, nothing else runs meanwhile) and the SND mutex contention above
  (sema 24, 7 times, main runs meanwhile).

So one host stack suffices: a thread is a function run nested, to its next block
point, at the moment the scheduler would have switched to it.

### Mechanism: native threads on the scheduler's decisions

The kernel keeps making every scheduling decision (priorities, ready order, wait
queues, alarms, ISR timing) exactly as now, so the order of effects and the
cycle counts do not change. What changes is how a switch is carried out:

1. A thread whose body is hand-written (loop head plus resume entries) is
   **native**: switching to it runs it nested on the current host stack
   (`run_guest`-style, `kernel::run_native`), and switching away from it is an
   unwind to the runner (the `Flow`/`Unwound` protocol the ports already follow:
   `call_ptr(WAIT_SEMA)` returns `Err`). Its context is saved at the wait and
   restored on the next resume; the port re-enters at the wait's return address
   (an arm of its `match pc`).
2. Preemption of a native thread by a higher-priority one nests the new one on
   top; when it blocks, the preempted one continues. A switch that would need to
   resume a thread lower on the stack while a native thread is above it (a
   non-LIFO switch) is a fatal error: it never happens in the replay, and the
   scenarios will show if it does anywhere else.
3. Blocking mid-body (the alarm waits, the RPC waits) becomes "run the event
   loop until the semaphore is signalled" when nothing else may run meanwhile;
   the SND-init RPC waits (main runs meanwhile) are completed synchronously by
   the SIF HLE instead, which moves a few boot events (the gate baseline is
   rebuilt for that step only, after comparing screenshots and the race).
4. When every thread is native, no coroutine is created: `threads.rs`,
   `corosensei`, the `StackSwitcher`, and the frame driver's baton handoff go. The
   main loop becomes the step function the frontends call once per VBlank.

## Port the thread paths first (wave 237)

The unported functions the threads run (117 functions, 30 KB: SND driver and its
mixer init, `PS2GraphicsMan`/`Renderer`, `EAFile`, `EAMutex`, libkernl, SIF/FS
library, `ssx_*` boot) are one writer wave, `oc688`-`oc699`. Thread entries
`PS2ExecMan_WorkerThread` and `PS2ExecMan_VSyncThread` are already hand-written.

## Open: where the outer event service ends

Found while converting the SND thread (`SSX3_TRACE_THREADS=1` now also logs every
`service_events` call, forced events and frame boundaries). In coroutine mode the
`service_events` call that wakes a thread from the frame driver's idle loop runs the
thread inline (`handoff(0, token)`), and the tail that recomputes `next_event`
runs when the baton comes back. For the VSync thread (wakes, runs, hands to main,
main idles) nested execution reproduces this exactly. For the SND thread (wakes,
runs, blocks, idle) the original shows no tail after the slice, nested execution
does, and the next idle tick moves by the cycles between the last event check and
the block (31). Until that is explained (what makes the original skip the tail:
how `handoff` and `run_from_driver` return when the thread blocks straight to
idle), the SND thread stays a coroutine. The explanation decides how the main
loop and the worker (which also run across frame boundaries) can be converted
without moving host-visible events; if exactness cannot be kept, the fallback is
a re-based gate for that step only (compare race outcome and screenshots).

## Mid-body blocks (what stops a thread from being native)

A native thread unwinds at its wait and is re-entered at the wait's return address.
That works when the wait is a direct syscall call of a resumable (dispatcher-shaped)
body: the loop-top wait of a thread. A thread that blocks deeper (inside `SND_Service`
-> `SND_MutexLock`) leaves frames of functions that cannot re-enter at a return
address, and it cannot stay nested on top of the thread it waits for.

| thread | blocks mid-body | consequence |
| --- | --- | --- |
| 6 VSync | no | native, gate identical (done) |
| 8 SND | SND mutex held by the thread it preempted (7 in the replay) | needs resumable frames from the thread entry down to `SND_MutexLock`, or a deferral that reproduces the 39-cycle bounce, or an accepted timing change |
| 1 main | `EAFile_DelayTicks` (alarm + semaphore: wait inline with the event loop, nothing else runs) and the frame wait in `PS2ExecMan_WaitFrame` | main becomes the base context the driver runs; its waits become "pump events until signalled" |
| 5 worker | loading jobs wait on IOP RPC replies and CD reads (sema 8, 17, ...) | job-by-job conversion |
| 2, 3, 4, 7 | no: they run once to a wait and never resume | native, gate identical (done; see `../investigation/native-thread-attempts.md`) |

Frame boundaries: the host regains control at each VBlank (`run_frame` returns). In
coroutine mode a thread that sees the boundary hands the baton to the driver
mid-slice. A native slice sits on the driver's own stack and cannot, so the
boundary is deferred until the stack unwinds; `Sched::boundary_in_native` keeps the
outermost `service_events` tail identical to the original. Nothing in the VSync
thread's slices observes the host's work between frames, and the gate agrees.

### Decision needed for the rest

Exact (gate-identical) conversion of the SND thread, main and the worker needs either
(a) every function on their blocking paths to be re-entrant at its return addresses
(large, mechanical, and the code that results is worse Rust than a plain call), or
(b) a deliberate, bounded timing change: replace "preempt and block on the mutex"
by "defer the thread's slice to the unlock" and the SND-init and `DelayTicks`
waits by synchronous completion, then rebuild the gate baseline once and check
equivalence by comparing screenshots and the race result instead of RAM. (b) is
what the roadmap's end state ("the game as one loop the port drives") implies,
because it removes the guest scheduler's exact cycle interleaving, which only
the emulation of the PS2 kernel had; it should be an owner decision.

Every attempt, with how to retry, is in `../investigation/native-thread-attempts.md`.

## Verification

- The gate after every thread.
- Per-frame schedule trace identical before/after (same sequence of
  step calls and their cycle counts), until the last thread is gone.
- Race replays, menus, movie, save/load scenarios (`tools/scenarios.py`) once per
  milestone.
- Browser: smoke test last, without JSPI.

## Status

| step | status |
| --- | --- |
| Worktree on `main` with #78/#79 | ✅ |
| Trace tool + inventory | ✅ (`thread-inventory.md`, `tools/thread_inventory.py`) |
| Port the thread paths | 🟡 wave 237 (`oc688`-`oc699`) |
| Native-thread mechanism in the kernel (nested run, unwind at the wait) | ✅ gate identical with the VSync thread native (`SSX3_NATIVE_THREADS=0` turns it off) |
| Park-once threads 2, 3, 4, 7 native | ✅ gate identical |
| Convert workers | ⬜ |
| Convert ISR-driven threads | 🟡 VSync (6) done and exact. SND (8): schedule exact, but the outer `service_events` that wakes it ends at a different point than in coroutine mode (one idle tick 31 cycles later): see "Open: where the outer event service ends" |
| Convert main + the rest, delete `threads.rs` | ⬜ |
