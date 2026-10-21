# Goals:
- Get the entire game ported to native Rust with no PS2 machine emulation in the codebase.

# General rules:
Keep the decompilation work:
  - as native as possible
  - as good performance as possible
  - avoid too many heavy validation runs
  - prefer unit tests to compare GS vs. native behavior

---

## Porting loop (typed Rust replacements)

Goal: replace generated EE functions with hand-written Rust, one verified function at a time. `generated/ee` is the authority for registers, cycles, memory and call order; the matching C++ decompilation in `devvydont-ssx3` (if path not known, ask where it is cloned to) is a readability aid (`tools/refsrc.py`, `refbody.py`, `refindex.py`, `refdossier2.py`, `integrate_wave.py`). Also use `overlap-functions.tsv`. 

### Orchestrator

1. **Pick a wave.** Take up to 50 functions per writer, preferring small (<= 600 bytes) functions that have a readable C++ body and *no* `__entries` dispatcher fragment, and that are not in `overrides.txt`, in any `oc*.rs` `OVERRIDES`, or in an earlier `wave*.txt`. Write `waveN-ocM.txt` and per-function dossiers (`refN/`).
2. **Start at most four writers** (respect the machine limits). Give each one module file, its assignment file, the dossier directory, and the subagent rules below. Tell them: code and tests only, never run cargo. Make it clear to the subagent that it's a subagent (with its rules below).
3. **Verify each report yourself.** Run `cargo test --profile game -p ssx3-game --features ocM --lib ocM::`. Fix mechanical compile errors directly (wrong shard, u32/u64, double borrow, `?` outside `guest`, missing `restore_ra`). Send semantic or fixture failures back to the same agent with the exact generated-vs-hand diff; repeat until the module is green.
4. **Install only green modules** (or green subsets; never leave an OVERRIDES entry whose test is absent/failing). Use `python3 tools/integrate_wave.py prep ocM`, `regen` (both generated forms), `repoint ocM`, `test ocM`, `build`, `gate`. The gate must print `GATE: race replay RAM identical to the baseline`. Commit.
5. **Re-check the browser form** after installs: `CARGO_TARGET_DIR=target-unwind cargo build --profile game -p ssx3 --bin ssx3-headless --no-default-features --features ee-unwind`, then run the same 6000-frame replay and `cmp` its RAM dump. No halt, empty `misses`, identical RAM.
6. One heavy process at a time (a build or a game run). Writers must not build. Bisect a bad gate with `SSX3_SKIP_OVERRIDES=addr,...`.
7. If a fixture cannot be bounded, drop that function's OVERRIDES entry and test and say so; keep the port with a comment if it is otherwise exact.

### Subagent (writer)

- Read `docs/investigation/phase2-worker-guide.md` and the wave brief first. Edit only your module file. Never edit `overrides.txt`, `Cargo.toml`, `lib.rs` or generated code, and never run cargo.
- Reproduce every register and cycle the generated body leaves, including `v0`/`v1` and scratch registers. Leaves end with `abi::ret(c)`; callers restore `RA` from the frame (`frame.restore_ra`). Wrap call-making overrides in `call::guest(c, |c| { ...; Ok(()) })` (or use the `if ... .is_err() { return; }` form).
- Loops with back-edge event checks: `events::event_due` + `deliver_event_here`, never `c.event()` with the result ignored (the browser/unwind build halts otherwise).
- VU0: issue the original's `c.vu0()`/`lqc2`/`sqc2` macro ops; do not recompute with `VuF32`.
- Fixtures (most "failures" are fixtures, not ports): set the BELibrary dummy at `gp-0x1f04` and the world at `gp-0x848`; use the pool allocator (`gp+0xdd8` + pool heads) and/or `DLHeap_InitArena` for allocation; keep real callees on short paths. Known runaway PCs: `0x3de768` lazy disc load, `0x31ef40` zeroed-heap consolidate. A 1-cycle diff is usually a missed block charge; a wrong `pc`/`ra` is a missing return restore.
- Add one `#[test]` per function comparing against the generated body (`f_XXXXXXXX__generated` if a slot exists, else `f_XXXXXXXX`), with a fixture that reaches every branch. Report functions written, skipped and why, and anything uncertain.
