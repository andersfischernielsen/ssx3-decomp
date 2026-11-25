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
| 1 Acceptance | ⬜ |
| 2-7 | ⬜ |
