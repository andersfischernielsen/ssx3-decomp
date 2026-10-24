# Port while you look:
- Whenever you study a function (in Ghidra, `generated/ee` or the C++ reference) for any task, also port it to hand-written Rust while you are there, with its differential test (porting loop below), if it is still generated. Use subagents for this, if possible.

# Goals:
- Get the entire game ported to native Rust with no PS2 machine emulation in the codebase.

# General rules:
Keep the decompilation work:
  - as native as possible
  - as good performance as possible
  - avoid too many heavy validation runs
  - prefer unit tests to compare GS vs. native behavior

Never co-author commits.

---

## Porting loop (typed Rust replacements)

Goal: replace generated EE functions with hand-written Rust, one verified function at a time. `generated/ee` is the authority for registers, cycles, memory and call order; the matching C++ decompilation in `devvydont-ssx3` (if path not known, ask where it is cloned to) is a readability aid (`tools/refsrc.py`, `refbody.py`, `refindex.py`, `refdossier2.py`, `integrate_wave.py`). Also use `overlap-functions.tsv`. 

### Orchestrator

1. **Pick a wave, hottest first.** Take the wave (up to 50 functions, one writer each, at most four at a time) from a *fresh* call graph (`SSX3_CALLPROF=<out>:<from>:<to>` on a race replay, built with `--features callprof`), ranked by `calls/frame * bytes` (`tools/pick_wave.py`). Do not order by size: size is only a tractability hint when choosing between equally hot functions. Exclude everything in `overrides.txt`, any `OVERRIDES` (`crates/ssx3-game/src/game/`, staging `oc*.rs`), and earlier `wave*.txt`. Write `waveN-ocM.txt` and per-function dossiers (`refN/`). Functions with no readable C++ body are still valid hot targets: port them from `generated/ee` (and Ghidra when the body is unclear).
2. **Start at most four writers** (respect the machine limits). **One function per writer:** each writer gets exactly one function, ports it with its test, reports and ends; start a fresh writer for the next function (contexts that grow over many functions cost far too many tokens). Never continue a finished writer with a new function. Give each one module file, its function, the dossier, and the subagent rules below. Tell them: code and tests only, never run cargo. Make it clear to the subagent that it's a subagent (with its rules below).
3. **Verify each report yourself.** Run `SSX3_DIFF_STRICT=args cargo test --profile game -p ssx3-game --features ocM --lib ocM::` (`args` also compares every call's arguments). Fix mechanical compile errors directly (wrong shard, u32/u64, double borrow, `?` outside `guest`, missing `restore_ra`). Send semantic or fixture failures back to the same agent with the exact generated-vs-hand diff; repeat until the module is green.
4. **Install only green modules** (or green subsets; never leave an OVERRIDES entry whose test is absent/failing). Use `python3 tools/integrate_wave.py prep ocM`, `regen` (both generated forms), `repoint ocM`, `test ocM`, `build`, `gate`. The gate must print `GATE: race replay RAM identical to the baseline`. Commit. Then move the staging module into the source-tree layout: `python3 tools/reorg_modules.py apply ocM` (each function into `crates/ssx3-game/src/game/<dir>/<unit>.rs` after the C++ unit, merged into existing units), delete `ocM.rs` and its `lib.rs`/`Cargo.toml` lines, test and gate again, commit.
5. **Re-check the browser form** after installs: `CARGO_TARGET_DIR=target-unwind cargo build --profile game -p ssx3 --bin ssx3-headless --no-default-features --features ee-unwind`, then run the same 6000-frame replay and `cmp` its RAM dump. No halt, empty `misses`, identical RAM.
6. One heavy process at a time (a build or a game run). Writers must not build. Bisect a bad gate with `SSX3_SKIP_OVERRIDES=addr,...`.
7. If a fixture cannot be bounded, drop that function's OVERRIDES entry and test and say so; keep the port with a comment if it is otherwise exact.

### Subagent (writer)

- **One function, then stop.** Port only the single function you were given, with its test, report, and end your work. Do not pick up other functions; read only what that function needs.
- Read `docs/investigation/phase2-worker-guide.md` and the wave brief first. Edit only your module file. Never edit `overrides.txt`, `Cargo.toml`, `lib.rs` or generated code, and never run cargo.
- Reproduce every register and cycle the generated body leaves, including `v0`/`v1` and scratch registers. Leaves end with `abi::ret(c)`; callers restore `RA` from the frame (`frame.restore_ra`). Wrap call-making overrides in `call::guest(c, |c| { ...; Ok(()) })` (or use the `if ... .is_err() { return; }` form).
- Loops with back-edge event checks: `events::event_due` + `deliver_event_here`, never `c.event()` with the result ignored (the browser/unwind build halts otherwise).
- VU0: issue the original's `c.vu0()`/`lqc2`/`sqc2` macro ops; do not recompute with `VuF32`.
- Fixtures (most "failures" are fixtures, not ports): set the BELibrary dummy at `gp-0x1f04` and the world at `gp-0x848`; use the pool allocator (`gp+0xdd8` + pool heads) and/or `DLHeap_InitArena` for allocation; keep real callees on short paths. Known runaway PCs: `0x3de768` lazy disc load, `0x31ef40` zeroed-heap consolidate. A 1-cycle diff is usually a missed block charge; a wrong `pc`/`ra` is a missing return restore.
- Add one `#[test]` per function comparing against the generated body (`f_XXXXXXXX__generated` if a slot exists, else `f_XXXXXXXX`), with a fixture that reaches every branch. Report functions written, skipped and why, and anything uncertain.

---

## Tick-rate pass (configurable game rate)

Goal: the game runs correctly at any tick rate (`SSX3_TICK_HZ`, settings "Game rate"), and stays bit-exact at 60. Read `docs/plans/configurable-tick-rate.md` first: the rate switch, the conversion rules, the work order. The inventory is `docs/plans/tick-rate-functions.tsv` (`tools/scan_framerate.py`; the `pool` column maps each load site to its `.lit4` entry). The rate lives in `crates/ssx3-game/src/tick.rs` only.

### Orchestrator

1. **Waves follow the plan's work order** (T1, T2, then T3 by system). One writer per output file; at most four writers; the same machine limits and "one heavy process at a time" as the porting loop.
2. **T1 (classification, no code).** Give each writer up to 50 `.lit4` entries (rows of the inventory, grouped by unit) and one output file `docs/plans/tick/t1-<writer>.tsv` with the columns `entry  site  function  class  reason`. `class` is `STEP` (seconds per tick: follows the rate, the default), `NOT_A_STEP` (1/60 meaning something else, e.g. seconds to minutes for a clock display) or `UNSURE`. Check every `NOT_A_STEP` and `UNSURE` yourself against the code; add confirmed `NOT_A_STEP` entries to `tick::NOT_A_STEP` with the reason as a comment.
3. **T2/T3 (conversion).** Give each writer one unit file (`crates/ssx3-game/src/game/<dir>/<unit>.rs`) and the functions of that unit to convert; never two writers in one file. A function that is still generated is ported first with the normal porting loop (staging `ocM.rs`), installed, then converted in a later wave.
4. **Verify each report yourself:** `SSX3_DIFF_STRICT=args cargo test --profile game -p ssx3-game --lib game::<dir>::<unit>::` (the 60 Hz differential tests must pass unchanged, and the new rate tests), then `cargo build --profile game -p ssx3 --bins`, `python3 tools/integrate_wave.py gate` (must print `GATE: race replay RAM identical to the baseline`), commit.
5. **End of each wave:** `python3 tools/tick_ab.py 120` (two headless runs) and look at the sheets: race clock and course % must agree up to a constant offset; note anything that runs fast or slow in the plan's "Measured" table and turn it into T3 work. Update the plan's status, commit and push.

### Subagent (tick-rate writer)

- You are a subagent. Edit only the file you were given. Never edit `overrides.txt`, `Cargo.toml`, `lib.rs`, `tick.rs` or generated code, and never run cargo.
- Read `docs/plans/configurable-tick-rate.md` ("Conversion rules") and `crates/ssx3-game/src/tick.rs`.
- T1: for each entry, read the function at the site (the hand-written port if `ported` is `yes`, else `generated/ee` and `python3 tools/refsrc.py <name>`) and decide what the loaded `1/60` means there. Write one row per entry. Give a concrete reason for anything that is not `STEP`.
- T2/T3: use only `tick::rate(c)`, `tick::dt(c)`, `tick::ticks_from_frames60(c, n)`. At 60 the code must do exactly what it did: same values, same registers, same cycles, same calls. When the converted expression is not bit-exact at 60, write `if tick::rate(c) == 60 { original } else { converted }`, and put the converted value where the original put its value (same register or memory). Loads from `.lit4` already follow the rate: leave them alone.
- Look for what the scan cannot see in your functions: frame counters and thresholds in frames, `v *= k` decay, `x += (t - x) * a` smoothing, `+= c` per tick, random chances per tick. Leave `n % 3`-style load spreading in ticks.
- Keep each function's differential test unchanged. Add one rate test per converted function: `tick::set_for_test(c, 120)`, write `tick::step_bits(120)` into the `.lit4` entries the function loads, run it twice, and compare the timing quantity with one run at 60 (exact for counters, a small tolerance for floats).
- Report: per function the class or conversion, the tests added, anything left unconverted and why, and anything uncertain.
