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
| Worktree on `main` with #78 | ⬜ |
| Trace tool + inventory | ⬜ |
| Port the thread paths | ⬜ |
| Convert workers | ⬜ |
| Convert ISR-driven threads | ⬜ |
| Convert main + the rest, delete `threads.rs` | ⬜ |
