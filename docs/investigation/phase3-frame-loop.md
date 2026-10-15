# Phase 3, frame loop: findings and host clock mode

Roadmap item: "frame loop → host timer". This doc covers how the game's frame loop works
(`phase3-boundaries.md` §3, `frame-callgraph.md`), what a host timer can and cannot change,
and the host clock mode that exists now.

## How the game paces itself

- **The logic tick is the VBlank.** At every VBlank end, the `PS2ExecMan` VSync thread runs
  `App_VSyncTick`. It counts the tick, samples the pads into `PadQueue` (`SSXApp_Tick`) and
  wakes the main thread if it is waiting.
- **The main loop** (`App_RunModuleLoop`) runs one `module->Update` per queued pad entry,
  up to 12 (`App+0x20`), then a single `module->Render`, then `PS2ExecMan_WaitFrame`. So the
  game already decouples logic from rendering: when a frame takes longer than a VBlank, it
  catches up with several fixed 1/60 s updates and renders less.
- **Other clocks:**
  - `EATimer` (RCNT1, polled timer tasks: streaming, CD tray);
  - the sound driver's tick (93.75 Hz, `phase3-sound.md`);
  - the renderer's kick thread (VBlank start, flip).

  All are emulated from the EE cycle counter, so they stay consistent with each other.

Replacing the VSync thread and the interrupts with direct calls would change *who* runs
`App_VSyncTick`, not *when*. It would also lose the game's catch-up structure or have to
re-create it. As long as rendering goes through the emulated GS and VU, there is nothing to
gain, so the threads and interrupts are kept.

## Host clock mode (`SSX3_HOST_CLOCK=1`, opt-in)

Emulated time follows wall-clock time (`machine::follow_host_clock`):

- At each event service point, if the EE cycle counter is more than 0.5 ms behind the host
  clock, it jumps ahead (at most 100 ms at a time; a longer stall, such as a debugger, is
  forgotten).
- VBlanks, audio ticks and timers then happen in real time, and the game catches up with its
  own multi-update loop.
- `run_frame` also runs VBlanks that are already late without stopping to present them, so
  the host skips frames.
- Without the variable, `run_frame` makes exactly one pass and nothing changes; the replay
  A/B stays byte-identical.

Measured (headless, race script, `SSX3_SLOW_FRAME_MS` simulating a slow present):

| Case | Wall | Emulated | VBlanks/s | Skipped | Result |
|---|---:|---:|---:|---:|---|
| no slowdown, menus | 2.2 s | 20.0 s | 540 | 0 | headless runs ahead of real time (the windowed front end paces at 59.94 Hz); the mode never triggers |
| 10 ms per presented frame, 5000 VBlanks | 83.8 s | 83.6 s | **59.8** | 71.2 s | real-time VBlanks and audio, but **loading was starved**: frame 5000 still shows "Loading 43%" (normally mid-race) |
| 30 ms per presented frame | 45 s | 44.9 s | 59.8 | 44.8 s | starved: the game never got past boot |

The limit is simple: skipping cycles keeps time real-time only if the host has spare guest
speed. Every skipped cycle is time the guest did not run, so CPU-bound phases such as loading
slow down in game terms.

On this machine the guest runs faster than real time in menus (about 9×) and races (about
1.4×), so the mode rarely triggers. When it does (a heavy moment in a race), music and the
race clock stay real-time and the game drops renders.

It stays opt-in. The point where the frame loop really changes is native rendering: once a
render no longer means emulating VU1 and GS work on the EE timeline, `Render` can be paced by
the host's display while `Update` stays on the 60 Hz tick.
