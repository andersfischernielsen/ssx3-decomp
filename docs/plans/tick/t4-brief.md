# T4 conversion brief (per-tick updates with no 1/60 marker)

The classification is in `docs/plans/tick/t4-a.tsv` .. `t4-d.tsv` (`class` =
PER_TICK rows, with the site, the constant and a proposed `conversion`).
Read `docs/plans/configurable-tick-rate.md` ("Conversion rules") and
`crates/ssx3-game/src/tick.rs` first.

Rules (AGENTS.md, tick-rate writer and porting writer):

- You are a subagent. Edit only the files you were given. Never edit
  `overrides.txt`, `Cargo.toml`, `lib.rs`, `tick.rs` or generated code, and
  never run cargo (the orchestrator builds and tests).
- At 60 Hz the code must do exactly what it did: same values, registers,
  cycles, calls. When a converted expression is not bit-exact at 60, write
  `if tick::rate(c) == 60 { original } else { converted }` and put the value
  where the original put it (same register or memory).
- Tools: `crate::tick::rate(c)`, `tick::dt(c)`, `tick::ticks_from_frames60(c,
  n)`, `tick::frame60_due(c, key, frame)` (a per-object 60 Hz clock: run a
  60 Hz timeline step only when it returns true), and the helpers
  `crate::game::ai::rider::{rate_step_60, per_tick_60}` (k * 60/rate) and
  any existing `smoothing_60` / per-tick-factor helper (k ^ (60/rate)); grep
  for them before writing your own.
- `.lit4` 1/60 loads already follow the rate: leave them alone. Constants
  in `tick::PER_TICK` are already rewritten for the rate: do not convert
  those sites again.
- Random chances rolled per tick: `p' = 1 - (1 - p)^(60/rate)`, or roll
  only when `frame60_due` says a 60 Hz frame passed.
- Porting a generated function (listed as such in your assignment): its
  slot exists, the generated body is `ee::sNN::f_<addr>__generated` (shard
  given). Port it into your unit file following
  `docs/investigation/phase2-worker-guide.md` and the surrounding code's
  idiom; reproduce every register, cycle and call of the generated body;
  add it to the unit's `OVERRIDES` table `(0x00XX_XXXX, fn_name,
  "Name")`; add one differential `#[test]` against the generated body
  with a fixture reaching every branch (copy fixture helpers from the same
  file's tests). Then convert it like the hand-written ones.
- Each converted function: keep its differential test unchanged (it runs at
  60) and add one rate test: `tick::set_for_test(c, 120 or 240)`, run it
  rate/60 times (or the needed number of ticks), compare the timing
  quantity with one run at 60 (exact for counters, small tolerance for
  floats).
- Report: per function what you changed, tests added, anything left and
  why, anything uncertain.
