# Native guest threads: what was tried, and how to retry

Plan: `../plans/guest-thread-removal.md`. Inventory: `thread-inventory.md`.
This page records each conversion attempt so the next one starts where this ended.

## Method (same for every thread)

1. Add the thread's entry address to `NATIVE_THREAD_ENTRIES` (`crates/ssx3-game/src/lib.rs`).
2. `python3 tools/integrate_wave.py build`, then `... gate`. It must print
   `GATE: race replay RAM identical to the baseline`.
3. Check it ran natively: `SSX3_TRACE_THREADS=1 target/game/ssx3-headless 300 2>&1 | grep "native slice"`.
4. If the gate fails: trace both modes (`SSX3_NATIVE_THREADS=0` is the coroutine baseline)
   with `SSX3_TRACE_SYSCALLS=1 SSX3_TRACE_THREADS=1`, and diff the switch lines
   (`[thread] a -> b at ... cyc N`, `[block] ...`, syscall lines with `ra` and `cyc`).
   The first line that differs names the thread, the semaphore and the cycle.
   `tools/thread_inventory.py` summarises a trace.

A thread can be native when its body (and every function it calls before its wait) is a
plain port whose syscall calls propagate the unwind (`?`/`is_err()` on `call_ptr`), and,
if it is ever resumed, is dispatcher-shaped so it can re-enter at the wait's return
address (see `ps2_exec_man_vsync_thread`, arm `0x0031_ac30`).

## Results

| id | thread | result |
| --- | --- | --- |
| 6 | `PS2ExecMan_VSyncThread` 0x31ac08 | native, gate identical. Needed a resume arm at the wait's return address and the frame-boundary handling (`Sched::boundary_in_native`). |
| 2 | `libkernl_IntrHelperThread` 0x424aa0 | native, gate identical. Runs once to its wait and never resumes (the generated body unwinds correctly). |
| 3 | `EAFile_CdReadThread` 0x3e3be0 | native, gate identical (parks once). |
| 4 | `libcdvd_CallbackThread` 0x4009a8 | native, gate identical (parks once). |
| 7 | `Renderer_DmaKickThreadEntry` 0x382740 | native, gate identical (parks once). |
| 8 | `SND_EeThread` 0x3c19a8 | **failed** (below). Reverted to a coroutine. |
| 1, 5 | main, worker | not attempted: blocks mid-body (below). |

## Why the SND thread failed (the one to retry)

Entry 0x3c19a8, loop-top `WaitSema(0x19)` (signalled by `SND_SifCmdHandler`, an ISR). The
loop-top wait is resumable (an `after_wait` arm exists in `snd_ee_thread`,
`game/lib/snd.rs`), and the schedule was correct for most slices. The gate failed
(RAM diff) because 7 times in the replay the SND thread preempts the main thread while
main holds the SND mutex (semaphore 24), then blocks on that mutex **in the middle of
`SND_Service`** and only continues after main unlocks.

- That is a non-LIFO switch: thread 8 is nested above main on the host stack, but must
  wait for main to run. A nested single-stack run cannot represent it.
- Unwinding thread 8 at the mutex wait loses its `SND_Service` frames (not
  re-enterable at a return address); the guest then resumes at the wrong place.
- An earlier note claimed "mutexes never block"; that was wrong (sema 24 was misread as
  an RPC wait). The block count comes from the `[block] thread 8 sema 24` lines.

Ways forward (also in the plan, "Decision needed for the rest"):

- (a) exact: make the functions between the entry and `SND_MutexLock` resumable
  (dispatcher-shaped `match pc` arms), so the unwind can re-enter at the mutex wait.
- (b) accept a bounded timing change: defer the SND slice until main unlocks, rebuild
  the gate baseline once, and compare screenshots and race results.

## Main and the worker

- Main (1): `EAFile_DelayTicks` waits on an alarm semaphore (sema 13, 20 times; nothing
  else runs), and `PS2ExecMan_WaitFrame` waits for the frame. Both are "pump events until
  signalled" in the end state.
- Worker (5, same priority as main, cooperative via `EAThread_Yield`): the loading jobs
  wait on IOP RPC replies and CD reads (sema 8, 17, ...), job by job.

## Gotchas found

- The `UNWIND` bit stays in `c.pc` after `run_native`: clear it, or `StartThread` loops.
- A coroutine thread that preempts a native thread must hand the baton back to
  `host_token` (the nested run is not LIFO otherwise).
- A resumed native thread first runs the syscall stub's tail event check (`resume_tail`).
- A frame boundary inside a native slice cannot hand the baton to the host: the
  outermost `service_events` tail is skipped when `service_depth == 1 && boundary_in_native`.
