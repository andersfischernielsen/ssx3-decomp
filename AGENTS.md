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
Wait with the browser build as the final step of your work (smoke test). The browser is the lowest-priority platform target.

Debugging aid: `SSX3_EVENT=<code>` (`SSX3_RIDER=<name>`) boots straight into an event (racing at about frame 400: no memory card, title or menus); codes in `docs/options.md`, `SSX3_EVENT=list`. Use it to look at or measure one course; keep the menu replay for the gate and the scenarios.

---

## Porting loop (typed Rust replacements)

Goal: replace generated EE functions with hand-written Rust, each verified by a differential test against the generated body and by the race replay gate. `generated/ee` is the authority for registers, cycles, memory and call order. The C++ decompilation in `devvydont-ssx3` (`~/Desktop/devvydont-ssx3`; ask if it is not there) and Ghidra's decompilation are readability aids.

The orchestrator (you, in the main session) runs waves: pick about 80 functions, split them into writer lists, start subagent writers, verify their modules, install, reorganise, push. Writers only write code and tests; the orchestrator does every build and run.

### Places and names

- Wave `N` is one more than the highest in `target/gate/waves/` (`waveN-*.txt`). Its working directory is `target/gate/waveN/` (wave list, dossiers in `ref/`, writer lists `list-ocM.txt`, logs).
- Staging modules `crates/ssx3-game/src/ocM.rs` continue from the highest `ocM` in `git log --oneline | grep -o 'oc[0-9]*' | sort -V | tail -1`.
- The call profile is `target/gate/edges.tsv`. The writer brief is `docs/porting/writer-brief.md`. `target/gate/` is not in git: if it is gone, rebuild the gate baseline and profile (step 1).

### Before a wave

- `main` is up to date and green. If the build stops with "generated/ee is older than overrides.txt", run `python3 tools/integrate_wave.py regen`.
- After pulling the reference (`~/Desktop/devvydont-ssx3`), rebuild its index: `python3 tools/refindex.py` (seconds; `refsrc.py` warns when it is stale).
- Ghidra is open on `SLUS_207.72` with the GhidraMCP plugin (port 8080; check with `mcp__ghidra__get_program_info`). Without it, dossiers of functions with no C++ have no pseudo-C; say so and go on.
- Long builds: wrap heavy commands in `caffeinate -s` (the machine sleeps otherwise). One heavy process at a time (a build, a test run or a game run), never two in parallel.

### Orchestrator

1. **Profile (only when `target/gate/edges.tsv` is missing or the race replay changed).** All overrides off, so every generated call counts:
   `CARGO_TARGET_DIR=target-callprof cargo build --profile game -p ssx3 --bin ssx3-headless --features callprof`, then
   `SSX3_NO_OVERRIDES=1 SSX3_INPUT="$(python3 tools/integrate_wave.py race-input)" SSX3_CALLPROF=target/gate/edges.tsv:4800:6000 target-callprof/game/ssx3-headless 6000`.
   The gate baseline (`target/gate/base/ds1.bin`) is the same replay's RAM dump: `SSX3_RAM_DUMP=target/gate/base/ds1.bin SSX3_INPUT=... target/game/ssx3-headless 6000` on a green `main`. The baseline holds on this Mac only (other platforms' maths libraries differ).
2. **Pick.** `SSX3_SCRATCH=target/gate/waves SSX3_WAVE_SIZE=80 python3 tools/pick_wave.py target/gate/edges.tsv target/gate/waveN waveN all`, then `cp target/gate/waveN/waveN-all.txt target/gate/waves/`. It ranks unported functions by `calls/frame * bytes` and skips `overrides.txt`, staged `OVERRIDES` and every earlier `target/gate/waves/wave*.txt` (so functions a writer skipped are not picked again; list those for dedicated waves). Once the profiled functions run out, the picks are functions the race calls rarely or never.
3. **Dossiers.** `python3 tools/refdossier2.py target/gate/waveN/ref $(cut -f1 target/gate/waveN/waveN-all.txt)`: the C++ body with its callees and globals mapped to addresses, or Ghidra's pseudo-C (`0xADDR.ghidra.c`) when there is no C++.
4. **Stage.** `python3 tools/stage_wave.py stage target/gate/waveN/waveN-all.txt target/gate/waveN ocM` (`ocM`: the next free number). Functions of 1300 bytes or more get a writer of their own; the rest go 10 to a writer, in wave order (`SSX3_BIG_BYTES`, `SSX3_PER_WRITER`). It writes `list-ocM.txt` per writer, creates each `ocM.rs` and wires it into `Cargo.toml` (feature `ocM`) and `lib.rs` (`pub mod`, and its `OVERRIDES` into `install()`).
5. **Writers.** One subagent per list, up to eight at a time (writers never build, so they do not load the machine). Each writer gets a fresh context with exactly this prompt, filled in:

   > You are a subagent (a writer), not the orchestrator. Read `docs/porting/writer-brief.md` and follow it. Your module: `crates/ssx3-game/src/ocM.rs`. Your functions: `target/gate/waveN/list-ocM.txt`. Port them in order, each with its differential test, then report per function and stop. Never run cargo.

   A finished writer is never given a new list (contexts that grow across lists cost far too many tokens); start a fresh one. Fix requests for its own module go back to the same writer (step 6).
6. **Verify each module** as its writer reports, one at a time: `SSX3_DIFF_STRICT=args cargo test --profile game -p ssx3-game --features ocM --lib ocM::` (`args` also compares every call's arguments). Put the command in a script file and run that: an inline `sh -c` once lost the test filter and ran the whole suite. Fix mechanical compile errors yourself (wrong shard, u32/u64, double borrow, `?` outside `guest`, missing `restore_ra`). Send semantic and fixture failures back to the same writer with the exact generated-vs-hand difference; repeat until green. A fixture that cannot be bounded: drop that function's `OVERRIDES` entry and its test, keep the port with a comment if it is otherwise exact, and say so. Six camera tests fail only under `SSX3_DIFF_STRICT=args`; they predate the waves.
7. **Install the green modules** (or green subsets: never leave an `OVERRIDES` entry without a passing test): `python3 tools/integrate_wave.py prep ocM ...` (adds the functions to `overrides.txt` and the modules to the default features), `regen` (both generated forms), `repoint ocM ...` (tests compare against `__generated`), `test ocM ...`, `build`, `gate`. The gate must print `GATE: race replay RAM identical to the baseline`. Bisect a failing gate with `SSX3_SKIP_OVERRIDES=addr,...`. Commit (`Port N wave-N functions (names...)`).
8. **Reorganise.** `python3 tools/reorg_modules.py apply ocM ...` moves each function and its tests into its source-tree unit (`crates/ssx3-game/src/game/<dir>/<unit>.rs`, named after the C++ file) and deletes the `ocM.rs` files; then `python3 tools/stage_wave.py unstage ocM ...` removes their `Cargo.toml` and `lib.rs` lines. Clean up what the merge breaks:
   - duplicate imports: `cargo test --profile game -p ssx3-game --lib --no-run > LOG 2>&1; python3 tools/dedupe_uses.py LOG`, repeated until no `E0252` is left;
   - by hand: self-imports, test constants that moved to another unit (use the full path), `SP` shadowing the register index, `driven!` wrappers separated from their test, helper name clashes, modules emptied by `cargo fix`.
   Then `cargo fmt --all`, the whole suite (`cargo test --profile game -p ssx3-game --lib`, about 20 minutes; the moved modules again with `SSX3_DIFF_STRICT=args` and their unit filters), `build`, `gate`.
9. **No warnings:** `cargo test --profile game -p ssx3-game --lib --no-run` and `cargo build --profile game -p ssx3 --bins` must report none (CI denies warnings in the crates it can build; it cannot build `ssx3-game`).
10. **Browser form:** `CARGO_TARGET_DIR=target-unwind cargo build --profile game -p ssx3 --bin ssx3-headless --no-default-features --features ee-unwind`, then the gate's replay with that binary (`SSX3_RAM_DUMP=... SSX3_INPUT="$(python3 tools/integrate_wave.py race-input)" target-unwind/game/ssx3-headless 6000`) and `cmp` its dump with `target/gate/base/ds1.bin`: no halt, empty `misses`, identical RAM.
11. **Finish.** `python3 tools/progress.py` (the README chart), commit (`Reorg the ocM-ocK ports into the source-tree units ...`), push `main`. Add new writer lessons (pitfalls more than one writer hit, fixes you had to make) to `docs/porting/writer-brief.md` in the same commit. If the wave ported functions with per-frame timing, follow with the tick-rate pass below.

Then start the next wave. Stop at a pushed point when the session is running out of context or tokens.

### Writer rules (summary; the full brief is `docs/porting/writer-brief.md`)

- You are a subagent. Port only the functions on your list, in order, each with its test; report and stop.
- Edit only your module file. Never edit `overrides.txt`, `Cargo.toml`, `lib.rs` or generated code. Never run cargo. Never commit.
- Reproduce every register and cycle the generated body leaves (`v0`/`v1` and scratch registers too); `generated/ee` is the authority.
- One `#[test]` per function against the generated body (`f_XXXXXXXX__generated` if a slot exists, else `f_XXXXXXXX` with `c.pc` set to the entry), with a fixture that reaches every branch. An `OVERRIDES` entry only together with its finished test.
- Report per function: written or skipped (and why), the test, anything uncertain.

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
