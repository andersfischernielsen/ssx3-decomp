# Single loop: the game without guest threads

Branch `feat/single-loop` (worktree `../ssx3-decomp-loop`), from `main` after
#83. Follows `guest-thread-removal.md` and `docs/investigation/native-thread-attempts.md`:
five of the seven guest threads are native; the SND thread (8), main (1) and
the worker (5) block in the middle of their bodies, which a single stack cannot
represent without making every function on those paths resumable.

Decision (owner, 2026-11-25): be as modern as possible. The end state is one
deterministic step per frame that the frontends call, with threads only where
they buy throughput. That gives up the byte-identical guest schedule, so
equivalence is defined anew (step 1) before anything changes.

## Modern shape

- One fixed-step `frame()`: tick, render, end-frame, sound pump, in a defined order.
- Sound: `ssx3-snd` driven from the frame step through a command queue; no guest
  SND thread, no mutex contention.
- Loading: async requests that complete into a queue (native files are already on).
- Waits (`DelayTicks`, frame wait, RPC replies): timers and synchronous completion.
- Real threads (job pool, render, audio) only where profiling shows a gain.

## Steps

1. **Acceptance first.** Define "equivalent" once the byte-identical RAM gate is
   gone: the race result, a per-frame hash of the game-logic state (excluding
   scheduler and stack memory), and screenshots at fixed frames. Record it on the
   old tree as the new baseline. Keep the old byte-identical baseline for the last
   commit before the change, so a regression can be bisected.
2. **A `frame()` step.** The tick, main's render and `EndFrame` become one function
   the frontends call per VBlank; the frame-semaphore waits go away.
3. **SND.** Drive `ssx3-snd` from the frame step through a command queue; remove the
   guest SND thread and the mutex contention.
4. **Worker and loading.** The worker's jobs become async requests that complete into
   a queue.
5. **Main's waits.** Replace the `DelayTicks` alarm waits with timers and synchronous
   completion.
6. **Delete.** Remove `threads.rs`, `corosensei`, the `StackSwitcher` and the guest
   kernel scheduler.
7. **Threads for real.** A job pool and a render and audio thread only where profiling
   shows a gain.

## Rules

- Steps 2 to 6 change behaviour; each one lands only after the step-1 checks agree
  with the step-1 baseline (race result, state hashes, screenshots).
- The byte-identical gate stays as the check up to the last step that keeps it.
- Browser smoke test last. Never push this branch without the owner's OK to merge.

## Status

| step | status |
| --- | --- |
| Worktree on `main` | ✅ |
| 1 Acceptance | ✅ `tools/accept.py record/compare`, `SSX3_STATE_LOG` |
| 2 `frame()` step | ✅ `Engine::step` runs the main thread on the driver's stack; it unwinds at `PS2ExecMan_WaitFrame` and re-enters there; boundaries inside in-place waits are made up (`Sched::host_frames`) |
| 3 SND | 🟡 the SND thread is native (unwinds at its mixer wait); a lock holder is not preempted, so the SND mutex never blocks mid-body. Still guest code; turning it into a plain step call is open |
| 4 Worker and loading | 🟡 the worker is native (unwinds at its job wait); jobs are still guest functions |
| 5 Main's waits | ✅ `DelayTicks` etc. wait in place (the machine idles on main's frames) |
| 6 Delete | 🟡 `threads.rs`, `threads_switcher.rs`, `corosensei` and the `StackSwitcher` deleted; the kernel scheduler stays (it is the guest syscall layer) |
| 7 Threads for real | ⬜ |

Result: the byte-identical gate still passes (the final RAM of the race replay is
identical to the old baseline) with every thread native.

## How the native kernel works

- Every guest thread (main included) runs on the frame driver's host stack, nested:
  `run_native` runs a thread until it blocks, exits, or is replaced.
- A thread that blocks at a registered **resume site** (`NATIVE_THREAD_ENTRIES`: the
  entry and the syscall return addresses where it may unwind) unwinds; its frames are
  rebuilt by re-entering at the return address. That needs every function between the
  entry and the wait to be a dispatcher-shaped port or generated code with a resume
  label (`generated/entries.txt`: return points of the calls on the main thread's
  chain, and the four loop-top waits).
- A block anywhere else is a **wait in place**: the thread stays on the stack
  (`Kernel::nest`) and the machine idles on its frames (`idle_in_place`) until an event
  wakes it; other threads run nested above it.
- A thread can only continue when everything above it has unwound
  (`Kernel::best_eligible`); a native thread that holds a lock (a semaphore created with
  count 1) is not preempted, so a thread that preempts cannot block on a lock whose
  holder is buried below it.
- `run_frame` returns at a VBlank; boundaries that pass inside an in-place wait are
  made up with immediate frames so the host's frame count stays the guest's.
- Adding the resume labels moved 3 cycles of block cost to after the wait call in the
  generated VSync and worker thread bodies; their ports follow it.

## Step 3 design: the SND round as a frame step

Today `SND_EeThread` (0x3c19a8) is a native guest thread: it waits on the mixer
semaphore (`state + 0x1aa4`), which the SIF command handler (`SND_SifCmdHandler`,
0x3c1578, command 0) signals once per audio frame through `iSignalSema`; it then
runs one mix round and waits again.

Plan:

1. Split `snd_ee_thread` into the setup (the constants it loads into the saved
   registers) and `snd_round(c)`, one round from the mix-flag test to the
   post-mix hooks. The thread body becomes `loop { wait; snd_round }`, so the
   gate stays identical while this is done.
2. Register the entry as a *step entry* in the kernel: StartThread of it creates
   no runnable thread; the signal on its semaphore only counts a pending round.
3. The frame step (`run_frame`, after the frame's guest work) runs the pending
   rounds on the driver's stack (`call_guest` with the sound stack as sp), unless
   a thread holds the SND lock (`Kernel.held`); then the round is deferred to the
   next frame step. No thread, no mutex wait, no mid-body block.
4. The byte-identical gate stops holding at this step (the mixer runs at a
   different point of the frame). Acceptance is `tools/accept.py` (race result,
   per-frame state hash, screenshots) and the audio output; the old baseline
   stays for bisecting.

## Step 1: acceptance (done)

`SSX3_STATE_LOG=path` (headless) writes, every `SSX3_STATE_EVERY` frames, a hash of each
4 KiB page of EE RAM with the guest thread stacks and `InitMainThread`'s stack left
out (`crates/ssx3/src/state_log.rs`). `tools/accept.py record DIR` runs the gate replay
with it, takes screenshots every 250 frames and keeps the final RAM; `compare A B`
says "RAM identical", or reports the first differing frame and pages, how the number of
differing pages evolves, and the screenshot differences. A change of schedule is
accepted when the last logged frame differs in at most `PAGE_TOL` pages and every
screenshot is within `SHOT_TOL` (defaults 64 pages, 2.0). Both runs of the unchanged
tree compare identical, and a corrupted log is found.

Baseline of the byte-identical tree: `target/accept/base0` (not in git; record it again
with `tools/accept.py record` on the last commit before a behaviour change).
