# Porting writer brief

You are a **subagent** (a writer) in the SSX 3 decompilation repository. The
orchestrator gave you a staging module (`crates/ssx3-game/src/ocN.rs`) and a
list of functions (`list-ocN.txt`: addresses, names, dossier paths). Port each
function, in the order given, to hand-written Rust in that module, each with
its own differential test. Then report and stop.

This brief is binding. It adds lessons from earlier waves to the writer rules
in `AGENTS.md` ("Porting loop"). The orchestrator adds new lessons at the end
after each wave.

## Scope

- Only the functions on your list, then stop. Read only what they need. Finish
  each one (port and test) before the next. If one is too hard, skip it and
  say why rather than leave it half done.
- Read `docs/investigation/phase2-worker-guide.md` first, completely, then
  each function's dossier.
- Edit **only** your module file. Never edit `overrides.txt`, `Cargo.toml`,
  `lib.rs` or generated code. **Never run cargo** (no build, test or check);
  the orchestrator builds, one heavy process at a time. Never commit.
- The module may already hold other writers' functions: append yours (the
  function, its `OVERRIDES` entry, its `#[test]` inside the existing
  `mod tests`) and leave theirs alone.
- Keep your module compiling at every step: all staging modules build
  together, and one broken module blocks everyone's tests.
- Other writers run at the same time and share the scratch directory: put
  any scratch files in a subdirectory named after your module (`ocN/`),
  never at its top level (a shared `body.rs` was overwritten mid-wave).
- Skip the huge top-level drivers (a whole subsystem's per-frame `*_Update`, a
  bytecode interpreter, qsort-like dispatcher bodies with resume twins) and
  say so, instead of spending your context on them. A function the
  orchestrator gave you alone (a large one) is meant to be ported, though.

## Sources

- `generated/ee` is the authority for registers, cycles, memory and call
  order.
- The reference C/C++ (the dossier; `python3 tools/refsrc.py <name>`, the
  `devvydont-ssx3` checkout) is a readability aid only.
- When DevvyDont has a function only as asm, the dossier may take its C++
  from the fallback reference, the `peak3project/ssx3` fork (`peak3-ssx3`;
  the dossier says "peak3project fork", with the fork's name for it).
- Dossiers also give the SSX On Tour dev-build name and DWARF signature when
  the reference knows one (`docs/ontour_names.tsv` there): good for naming
  arguments and fields.
- Functions with no C++ come with Ghidra's decompilation
  (`0xADDR.ghidra.c` next to the dossier). Use it for structure: control
  flow, calls, which fields are read and written. Its types, names and
  float/int guesses can be wrong. If Ghidra is open, its MCP tools load with
  ToolSearch (query `+ghidra decompile`, then
  `mcp__ghidra__decompile_function_by_address`). In your report, say how
  useful the pseudo-C was: what it got right, what misled you.

## The port

- Reproduce every register and cycle the generated body leaves, including
  `v0`/`v1` and scratch registers.
- Leaves end with `abi::ret(c)`. Callers restore `RA` from the frame
  (`frame.restore_ra`). Wrap call-making overrides in
  `call::guest(c, |c| { ...; Ok(()) })` (or the `if ... .is_err() { return; }`
  form).
- Callee-saved restores (`lq s0, ..(sp)`) the original does between calls
  must happen at the same point: the next callee's prologue saves that
  register to its stack.
- After a call, read `self`/state from the callee-saved register (s0..s7),
  not from a Rust copy taken before the call: a misbehaving callee can change
  it.
- Loops with back-edge event checks: `events::event_due` +
  `deliver_event_here`, never `c.event()` with the result ignored (the
  browser/unwind build halts otherwise).
- VU0: issue the original's `c.vu0()`/`lqc2`/`sqc2` macro ops; do not
  recompute with `VuF32`. FPU: `ps2f::FpuF32` operations as the guide says,
  never native `f32` maths.
- Recursive self-calls: call the generated alias (`f: ee::sNN::Name`), never
  the Rust function directly. After install, the generated body's recursion
  goes through the override table, and the call logs must match.
- A branch delay slot after a `let t = ...` loop test still executes: don't
  drop the store in it.
- Common compile errors: VU0 register numbers in `Src::Bc`/`Src::Vec` and
  `madd` destinations are `u8`, not `usize`; don't pass `c` twice in one call
  (`set_fr(c, n, gp_lit(c, x))`: bind to a `let` first); `c.cycles` is `u64`
  (no u32/u64 mix-ups); no unary minus on `u32`; register 3 is `v1` and 7 is
  `a3` (check `c.g[N]` indices).

## The test

- One `#[test]` per function, comparing against the generated body, with a
  fixture that reaches every branch. Every `OVERRIDES` entry must have its
  test: add the entry only together with the finished test.
- Test target: `f_XXXXXXXX__generated` if the function already has an
  override slot, else `f_XXXXXXXX`. `f_XXXXXXXX` dispatches on `c.pc`: set
  `c.pc` to the function's entry address in the fixture, or the generated side
  jumps to `__entries` and does nothing. (`integrate_wave.py repoint` switches
  the tests to `__generated` at install.)
- Make it readable: a doc comment saying what the function does in game
  terms, named offsets.
- Look at existing tests in `crates/ssx3-game/src/game/**.rs` for fixture
  helpers (`crate::difftest::*`) and copy their patterns.

## Fixtures (most "failures" are fixtures, not ports)

- Set the BELibrary dummy at `gp-0x1f04` and the world at `gp-0x848`. A real
  callee that reaches `BELibrary_GetInstance` without it loads from disc and
  runs away at pc `0x3de768`.
- Allocation: the pool allocator (`gp+0xdd8` + pool heads) and/or
  `DLHeap_InitArena`. Zeroed-heap consolidate runs away at `0x31ef40`.
- Keep real callees on short paths. History/file loaders such as
  `Avalanche_BakeAll` reach the lazy disc load with no short path: keep their
  trigger flags at 0.
- `SpatialHash`/`SpatialKey_FromAABB` loops forever on zeroed RAM: write the
  grid constants (`gp-0x2df8` = 1.0f, `gp-0x2df4` = 0.25f) as
  `spatial_key_from_aabb_matches` does.
- Spline/curve data must be valid (sorted times, t inside the range) or
  `cCSICubicSpline_Evaluate`'s search never ends.
- Set every argument register the function reads: a scrambled `a0` is a random
  pointer.
- Fakes return forced results zero-extended, so you cannot force -1 through
  them: use a real leaf that returns 0 (e.g. `0x1406c0`) or a real callee.
- Never place fixture data at `DATA+0x100000` (= `0x900000`): that is the
  fakes' call log. Use `DATA..DATA+0xfffff` or `DATA+0x200000` and up.
- A seed that starts at a resume entry inside a frame must write
  `difftest::RA` into the frame's saved-ra slot.
- Callees that are `match pc` dispatchers (memset, memcpy, `SND_MemCopy`, ...)
  can unwind mid-call: in `check_events` tests keep the first due event past
  such calls (see `psh_setup`).
- A 1-cycle difference is usually a missed block charge; a wrong `pc`/`ra` is
  a missing return restore.

## Lesson: global callee fakes must be gated

A test that installs a fake in a callee's `OVERRIDE_` slot changes that
function for every test in the binary, and the suite runs its tests in
parallel. A fake that always logs runs another unit's test into the fake's
pointer return (rider simulations then spin for minutes or forever), so:

- Gate the fake on a magic word in the *machine's* RAM: `if c.bus.read32(CTRL)
  != MAGIC { return <the body it replaced>(c) }`. Write `MAGIC` at `CTRL` in
  your own fixture (both sides of the diff get their own machine).
- Install with `crate::difftest::fakes::hook(addr_of_mut!(slot), fake,
  &PREV)` and, with the gate off, run the body in `PREV`. `hook` installs each
  fake once per process. Never re-record `PREV` per test and never put the old
  body back on `Drop`: two fakes on one slot then fall back to each other and
  a rider test loops forever (wave 213: `bow_fake_anim_current` and
  `oc318_fake_air_get_anim` on `0x311ae8`).
- A fake made by a macro that cannot name a static of its own uses
  `fakes::prev_cell(fake)` as its `PREV` (both in `hook` and `fall_back`).
  Never fall back straight to `__generated`: that skips the fakes other
  tests chained before yours, and their tests then run real callees on
  fixture data (wave 215: the handplant test's allocator ran away).
- The same rule goes for `c.pc` fakes and for helpers you put in another
  unit's `mod tests`: state whose absence hangs a rider test is worse than a
  failing test.

## Lesson: picked lists can name functions already ported

The wave list comes from `edges.tsv` and a snapshot of `overrides.txt`, so it
can contain functions an earlier wave installed. Check
`grep '^0xADDR' crates/ssx3-game/overrides.txt` before writing; skip those,
say so in the report, and never add a second `OVERRIDES` entry for the address.

## Lesson: the generated body is authoritative, so read the delay slots

Three ports in one wave dropped an unconditional store that sits in a branch
delay slot (`c.g[A2][0] = c.g[S3][0]` after a `call_ptr`); the loop then read
its entries from the wrong register (it looked like a callee-argument bug) and
the diff showed `f0`, the output buffer and 3-5 cycles off. Transcribe every
store after a call in the generated order, and take registers from the
generated body, not from what the C++ suggests.

Also: a `break` inside a labeled block (`'skip: { ... break; }`) must name the
loop (`'scan: loop` ... `break 'scan;`), and a `macro_rules!` that emits
`static $prev` puts the static at the *invocation* site's scope: do not also
declare it by hand.

## Lesson: keep the generated block structure (wave 214)

- An exit block that wraps a loop in the generated body (`'b_X: { 'l_Y: loop
  { ... break 'b_X; } }`) must wrap it in the port too. Put inside the loop,
  every exit restarts the loop and the function never returns (the test hits
  the cycle budget).
- A branch that jumps straight to the epilogue skips everything between: the
  `c.cycles += 1; v0 = s2` of the normal path must not run on the early exit.
- API shapes writers got wrong: `Frame::open(c, size)`,
  `frame.restore(c, reg, off)`; bind `vtable(c, ...)` to a `let` before
  passing it to `c.bus.write32`.
- Constants only your tests read: mark them `#[cfg(test)]` (otherwise the
  game build warns that they are never used, and it must build warning-free).

## Lesson: shared helpers for sibling functions (wave 215)

Siblings that look alike (`Rider_ReportRiderHit1`/`Hit2`) can differ in one
argument order: Hit1 passes `(a, b)` to its sound call, Hit2 `(b, a)`. A
shared helper written from the first sibling then passes the second one's
arguments swapped, which only `SSX3_DIFF_STRICT=args` catches. Diff the
siblings' generated bodies line by line before sharing a helper, and make
every difference a parameter.

## Lesson: wave 216

- `addr_of_mut!(ee::sNN::OVERRIDE_X)` and `fakes::hook(...)` are safe: no
  `unsafe { }` around them (the build warns, and it must be warning-free).
- Take every return address from the generated `c.g[RA][0] = sx(0x...)`
  line of that call. A formula (`head + 0x3c + 0x14*i`) was off by 0x3c for
  one call kind, and only the callee's saved `ra` on the stack showed it.
- A test whose target has no override slot calls `f_XXXXXXXX` (with `c.pc`
  set), never `f_XXXXXXXX__generated`: that name only exists once the
  function is installed.

## Names (the reorg merges your module into shared unit files)

- Give module-local helpers function-specific names, not `lt`, `at`, `gp`,
  `slot`, `f`: generic names clash after the merge.
- Test-only constants your tests take from the port (offsets etc.): keep them
  in the module body as `pub(crate) const`, named after the function. The
  reorg may put a port's tests in a different unit file than its constants;
  full paths still resolve.
- Never name a test constant `SP`: it shadows the stack-pointer register index
  from `reg::*`.

## Report

Per function: written (or skipped, and why), its test, and anything uncertain
(branches the fixture may not reach, assumptions). For functions with Ghidra
pseudo-C, how useful it was.
