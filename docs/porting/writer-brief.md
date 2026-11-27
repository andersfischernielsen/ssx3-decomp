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
  `deliver_event_here`, never `c.event()` with the result ignored.
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
- Test target: `f_XXXXXXXX__generated`. The orchestrator registers a slot for
  every function of the wave before you start (`integrate_wave.py slots`), so the
  name exists; only if it does not compile as `__generated` use `f_XXXXXXXX`.
  (Older text below about `f_XXXXXXXX` applies to that case.)
  Original wording: `f_XXXXXXXX__generated` if the function already has an
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
- Bus reads take `&mut Cpu` (`c.bus.read32`): a closure or helper that
  reads guest memory takes `c: &mut Cpu`, not `&Cpu` (wave 217).
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
- A callee that allocates (`operator_new_named`, `EA_MemAlloc`, the pool
  allocator) runs the real `DLHeap_*` code: with an uninitialised heap it
  loops ("runaway", pc around `0x31ef40`). Give the fixture a real allocator
  (`rbc_test_allocator` in `game/object/` ports; `game/be/beintstat.rs`) or
  fake the allocator entry points. Keep your fixture's data out of the pool
  and arena regions.
- `fakes::covered` needs at least one fake call and one unwind: a leaf
  function with no callees uses `difftest::check`.
- A function that returns to its caller must end with `c.pc = ra` (the
  generated `c.pc = t; return;`): a script that turns it into a bare
  `return Ok(())` leaves `pc` at the last block head.
- A function with resume entries (`if c.pc != ENTRY { return ..__entries }`)
  falls back to `ee::sNN::f_XXXXXXXX__generated(c)`, which exists next to `__entries`.
- Constants used only by tests: `#[cfg(test)]`, or the game build warns.
- `f_XXXXXXXX__generated` exists only once the function is in `overrides.txt`
  (after the orchestrator's `prep` + `regen`); a staging module cannot name
  it. Until then name `f_XXXXXXXX` / `__entries` in the normal form, and tell
  the orchestrator which resume fallback to switch at install: calling the
  function's own wrapper from its resume path recurses forever.
- A "runaway" in the function's *own* loop (pc inside the function) is usually
  a port bug: a converted `c.pc = t; return;` epilogue without its `return`
  falls out of the block and wraps around the `loop`.
- Test helpers made by a macro or helper fn in your module (`xx_install`,
  hooks, layout structs with private fields) are used by tests that the reorg
  moves to other unit files: make them `pub(crate)`, fields included.
- Fake and `PREV_*` names must be unique per module (`oc434_get_be`,
  `OC434_PREV_GET_BE`): the reorg moves tests into units that already have
  old fakes with the same name, and a fake paired with another module's
  `PREV_*` cell makes the hook chain loop.
- A big driver whose callees are mostly unported cannot be faked per slot: a
  child-process test with a private function table works (`oc421`'s
  `GameModule_Load`); if even that fails, drop the entry and test and keep the
  port with a comment.

- `machine` is `difftest::fakes::machine`, not `difftest::machine`; `sx` takes
  a `u32` here (`sx(v)`, not `sx(v as u64)`).
- A single function over about 5000 bytes that is a resumable dispatcher with
  60+ call sites (`Player_dtor`, `RiderBase_dtor`) is skipped by writers: give
  it a dedicated wave, split by segment.

- Generated bodies close nested blocks with a bare `}` at column 0: a body
  extractor must stop at a blank line, not at `^}$`.
- `c.mult(0, rd, rs, rt)` writes `rd` too: `c.mult(0, 2, 2, 4)` is v0 = v0 * a0.
- `Addr(x).sw(c, c.g[..])` double-borrows `c`: bind the value first.
- A real file loader as callee runs away on test data: drop its entry and test.

## Report

Per function: written (or skipped, and why), its test, and anything uncertain
(branches the fixture may not reach, assumptions). For functions with Ghidra
pseudo-C, how useful it was.

- Resumable dispatcher bodies (`match pc` blocks, or labelled blocks with an
  `__entries` twin) are not ported by writers: the orchestrator converts them
  with `tools/port_dispatcher.py ADDR SHARD Name fn_name` (run it after
  `prep` + `regen`: it reads `f_X__generated`). The port is the generated body
  with its calls through `call_ptr` / `Callee::call`, `?` for unwinds, and an
  `if c.pc != ENTRY { return ..::f_X__generated(c) }` guard for resumes. Its
  test fixture must reach the body (a bare `A0 = DATA` exits at once): build
  the structures and add a path-count test (`fresh` + `run_generated`, the set
  of cycle totals over the seeds).

- A shared helper body for "similar" functions must keep each function's own
  register allocation (which `s` register holds which argument): the call
  logs compare every register, so two siblings that differ only in S0/S2 fail.
  Check the generated prologue of each sibling before sharing a body.
- A port whose real callee is unported and unbounded on test data (runaway at
  the cycle budget) is dropped from `OVERRIDES` with its test (`#[allow(dead_code)]`
  on the function, a one-line reason); port the callee first.
- Labelled-block bodies converted with a script lose the closing braces that
  the generated text leaves at column 0: count `{` against `}` before handing
  a module over (the brace balance must be 0 after the epilogue).
- Flat static-init tables (hundreds of stores) are best generated from the
  generated text by script with the cycle/store counts checked, not retyped.

- Extracting a generated body with a script: stop at the next `pub fn`, `///` or
  `#[` line, not at the first blank line (the bodies contain none).
- Before porting a function, grep every `oc*.rs` module for its address with and
  without the digit separator (`0x0030_2680`): lists built before earlier writers
  report overlap, and two modules must never hold the same `OVERRIDES` address.
- `fakes::covered` counts only fakes installed through the machine's miss hook:
  gated `fakes::hook` fakes do not register, so a test whose callees are all
  gated or real uses `check`. A `covered` test needs at least one unwind: pass
  `machine(c, seed, salt, calls)` a `calls` count inside the real call count.
- A helper that adds the gp itself (a prologue taking an offset) must not be
  passed `gp + offset` by its callers (oc732: the guard word was read from
  `2*gp + off`).
- Tests that run a real callee which has no override slot cannot be faked: if it
  runs away, shrink the fixture (empty lists, count 0) or drop the entry and test.
- Long skipped functions go to a fresh writer on a stronger model with a list of
  at most six; short leaf and sibling families work on a small model with eight.

Lessons from the front-end, graphics and sound waves (248-250):

- Know the runaway signatures: a test that stops at the cycle budget with pc
  `0x3de768`/`0x3de878` is the `EAFile_AllocRequest` spin (slot 0x3de670 must be
  faked); `0x31ef40`/`0x31ea40` are list and allocator walks (empty lists, count 0);
  `0x39d888` is the inner loop of `UIScreen_FindObject` (fake the finder, gated
  fakes included). Bound the fixture first, then drop the entry and test.
- The generated body leaves scratch registers and stores that a port easily
  drops: the `v0`/`v1` of a compare, its result store, the 8-byte tail of a copy,
  the unsigned `sltu` of a pointer compare (not `slt`), and the cycle of a branch
  delay slot (`V0` off by one on the exit path). Read every branch of the
  generated body against the port before the first test run.
- A port that is written and tested but missing from its unit's `OVERRIDES` table
  (and from `overrides.txt`) is dead code that looks done: register it with the
  test pointing at `f_X__generated`, then gate. Never leave two ports of one
  address in two units; the second is deleted, not kept.
- Slot-only lines (`0x........    Name` in `overrides.txt`, no `OVERRIDES` entry yet)
  make `f_X__generated` exist for a port that is still being written; every such
  line is either registered or removed before the wave ends.
- After the reorg into the source-tree units: macro-generated
  `ocNNN_install*` helpers become `pub(crate)` and the consuming unit re-imports
  them in its `mod tests`; test code that indexes registers uses
  `c.g[ssx3_ee_rt::cpu::reg::SP]` (a local `SP` constant shadows the index);
  fields of structs used by other units' tests become `pub(crate)`.
  `cargo fix --lib` strips imports that only the tests use: run it with `--tests`
  or restore them behind `#[cfg(test)]`.

Lessons from the verification-speed pass:

- Fixtures whose real callees allocate (constructors, popups, string tables,
  the BELibrary) use `fakes::machine_rt(c, seed, salt, calls)` instead of
  `fakes::machine`: it adds the size-class allocator, the dlmalloc arena and the
  BELibrary dummy. Without it the test spins in `EAFile_AllocRequest` (pc
  `0x3de768`, `0x31ef40`, `0x31e968`) and the orchestrator has to bounce it back.
  Tests run with a 5-million-cycle budget during verification.
- Check your module with `python3 tools/lint_port.py ocM` (no build, one second)
  before you report: it flags hex literals with more than 8 digits
  (`0x001b_babcc`), addresses that the generated body never uses, return
  addresses that are missing and cycle totals that differ.
- Return-address and string-offset literals are the commonest transcription
  bugs of all: copy them from the generated text by script, never by eye, and
  keep them full (`0x001b_abcc`, never `0x434c` plus a base that a helper may
  add twice).
- Failure logs: in `"x vs y"` lines x is the generated body and y your port.
- Dispatcher lists carry a draft from `port_dispatcher.py`
  (`target/gate/waveN/draft/0xADDR.rs`): start from it and spend your time on the
  fixture (a path-count test), not on the transcription.

Lessons from the cleanup wave (254):

- Jump tables and `isButton` name strings live in the game image, which is
  empty in a bare test machine: every table read returns 0 and every event
  falls into the same `jump_out` arm (3 distinct call sequences instead of 12).
  Load it with `crate::difftest::elf_image(c)` right after `machine`/`machine_rt`
  (and before your own writes), or write the table words you need. Never lower a
  path-count threshold to make a test pass: raise the coverage.
- A jump-table entry equal to the function's own entry address loops forever
  (the draft's self-case retries through `event_due`): leave it out of the table.
- A test fake that you add to a shared override slot must use ITS OWN gate word
  and forced-result mailbox in the machine's RAM (a unique `DATA+0x...` address
  and magic, no `static` mutable state) and fall back with `fakes::fall_back`
  when the gate is off. Two tests sharing one gate word, or a fake that answers
  with the gate off, break each other once the modules sit in one unit file
  (found after the reorg: `OnAuthPopupEvent` broke `OnChildEvent` through
  `FA_CTRL`).
- A TextOK-style case that "never matched" is usually a port bug, not a
  fixture problem: the generated body compared `s0 != v0` BEFORE the delay-slot
  `lq(16)` restored `s0`, the port restored first. Delay slots execute before the
  next block: check compare operands against the restore they sit beside. Never
  weaken a fixture so a case stops failing.
- A resumable dispatcher without an `__entries` twin (e.g. 0x1a9cc0) is not
  detected by `stage_wave.py`; if a list entry has no draft, run
  `tools/port_dispatcher.py ADDR SHARD Name fn` yourself.
