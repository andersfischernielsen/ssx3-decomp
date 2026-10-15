# Recompiler v2: whole-program EE → Rust (`recomp-ee`)

_Written 2026-10-14._ Supersedes the `codegen.rs` / `recomp-all` scaffolding
for running the game. `codegen.rs` is kept for now, but its output is not used.

## What it is

`crates/ssx3-recomp/src/eegen.rs` translates **every instruction of every
function** in the inventory (11,844 functions, 783,966 instructions) into Rust.
It does this at build time from the raw words, so every R5900 encoding gets an
explicit lowering. Latest run: **0 invalid encodings**.

```sh
cargo run --release -p ssx3-recomp --bin recomp-ee      # → generated/ee/ (≈73 MB)
cargo build --profile game -p ssx3                      # compiles 16 shard crates
cargo run --profile game -p ssx3 --bin ssx3-headless -- 600 --shot-every 60
cargo run --profile game -p ssx3                        # windowed
```

The output is gitignored and derived only from `orig/SLUS_207.72`,
`generated/functions.tsv`, `generated/boundaries.tsv` and the optional
`generated/entries.txt` (extra entry points found at run time).

## Code shape

- **One crate per shard.** There are 16 shards (`ssx3-ee-s00..s15`), each a contiguous
  address range balanced by instruction count, so `rustc` compiles them in
  parallel. The crate `ssx3-ee` holds the sorted address → `fn` table
  (`FUNCTIONS`).
- **Functions.** Each function is `pub fn f_XXXXXXXX(c: &mut Cpu)`: a `loop { match pc
  { label => { ... } } }` over its *labels*. Labels are:
  - the entry and every branch target;
  - every return point after `jal`/`jalr`/`bal`;
  - the instruction after each `syscall`;
  - every in-function address that data refers to (scanned `.data`..`.sbss`;
    this covers jump tables);
  - every address listed in `generated/entries.txt`.

  Any label can be an entry point. That's what lets a thread resume at the exact
  instruction where it was switched out.
- **Inline ALU.** ALU code is emitted inline (`c.g[rd][0] = sx(...)`), with the MIPS64
  convention that 32-bit ops sign-extend.
- **Semantics library.** Everything else calls the semantics library in
  `ssx3-ee-rt` (moved out of `ssx3-ps2`, see `crate-split.md`): `cpu.rs`
  (memory, mult/div, COP0), `mmi.rs` (all MMI), `fpu.rs`
  (PS2 float rules) and `cop2.rs`.
- **Delay slots.** The branch condition is evaluated before the delay slot. For
  likely branches the delay slot runs only when the branch is taken.

## Calls, returns, threads (the unwind/trampoline model)

Ported from ps2-web-recomp's design (MIT):

- **`jal T`.** Set `ra`, run the delay slot, then
  `if !c.enter(T) { return; } f_T(c); c.leave(); if c.pc != RET { return; }`.
  Calls within a shard are direct Rust calls. Calls to another shard use
  `machine::call_indirect` (a binary search over the table).
- **`jr $ra`.** `c.pc = ra; return;`
- **Other `jr`.** Set `pc = t; continue`, a jump-table dispatch within the
  function. If `t` isn't a label, the code calls `machine::jump_out`, which does a
  tail jump to other code or reports a fatal "not a label".
- **Thread switch.** A thread switch inside a syscall saves the context and sets
  `UNWIND` in `c.pc`. Every frame then sees `c.pc != expected` and returns. The
  trampoline (`machine::trampoline`) re-enters at the new thread's `pc`
  through the table.
- **Host stack.** Host stack depth is capped (`MAX_DEPTH` = 160). Deeper guest calls
  unwind, and the trampoline makes the call at depth 0.
- **Interrupts.** Interrupts, alarms and VBlank are delivered at backward branches and
  after syscalls (`c.cycles >= c.next_event` → `c.event()`). Handlers run
  nested (`kernel::run_guest`) on a kernel stack and return to the magic
  address `HANDLER_RETURN`.

## Known gaps

- VU0 macro arithmetic is still lowered to `c.vu0_macro(word)`, which decodes the
  word inside the VU library at run time. It will move to per-op calls once the
  VU library exposes them; see `vu-vif.md`.
- Integer-overflow traps (`add`, `sub`) and conditional traps (`teq` …) are not
  modelled.
- Timing is one cycle per instruction.
- The block-dispatch form is correct but not idiomatic Rust. Structured control
  flow and register promotion come later; correctness comes first.

## Structured output via host threads (implemented: `--threads`, `--structured`)

The block-dispatch form exists because a thread switch must be able to resume
any function at any return point after the host stack unwinds. If each EE
thread instead ran on its own host thread (PS2Recomp's approach, one runnable
at a time, handed off with a condvar baton), no frame would ever unwind.
Functions could then be emitted as **structured Rust** (loops and ifs, with
GPRs promoted to typed locals across calls by the ABI), which is a much more
readable decompilation:

- Interrupt handlers run nested on the current host thread.
- The depth cap goes away.
- Thread switches cost a condvar handoff, a few microseconds each, at a few
  thousand switches per second.

Prerequisites are a structuring pass over the CFG (the old `codegen.rs` reached
86% structured functions) and an ABI-aware register promotion. The current
form stays the correctness baseline to diff against.

### Status (2026-10-14)

Both modes are implemented and run the game. Default output is unchanged
until the structured build has passed the full race path:

```sh
cargo run --release -p ssx3-recomp --bin recomp-ee -- --threads      # host threads, dispatch form
cargo run --release -p ssx3-recomp --bin recomp-ee -- --structured   # host threads + structured Rust
```

- **Runtime** (`ssx3-ps2/src/threads.rs`):
  - one host thread per EE thread, plus a baton. Each host thread knows its own
    token through a thread-local, which fixed a deadlock when a thread exits
    with removal;
  - `machine::run_frame` is the token-0 frame driver and idle loop;
  - `kernel::switch_to` hands the baton over instead of unwinding.
- **Generator** (`ssx3-recomp/src/structure.rs`, Ramsey's algorithm):
  - **95% of functions are structured** (12,622 of 13,306), and the output
    shrinks from 79 MB to 56 MB;
  - the rest have jump tables (`jr` through a register other than `$ra`) or
    irreducible control flow, and keep the dispatch form;
  - a function that can be entered other than at its start gets a dispatch
    twin, `f_X__entries`, for those entries.
- **Result:** it reaches character select with identical GS output (primitive
  counts match).

### Open bug in host-thread modes (2026-10-14)

Both `--threads` and `--structured` reach character select. They then
derail around frame ~2750 of the scripted path, just after confirming board
selection:

- **First symptom:** a return to `0x39cd7c` cascades up. That address is the
  return point after the virtual call `jalr v1` in `FUN_0039cd30`, and that
  frame is *not* on the stack when the mismatch appears. With "mismatched
  returns propagate to the base trampoline" (current code), thread 1 then spins
  at `0x39cd7c`. In thread mode, return points are not labels, so the base
  trampoline can't resume there.
- **Suspects:**
  - an entry into a function at a non-start address that the generator doesn't
    know about. Structured functions without an `__entries` twin always start
    at their entry; dispatch functions would report `bad_entry`;
  - jump tables whose case code lives in separate Ghidra functions (the
    `0x3a0158` and `0x3a3808` dispatchers tail-jump into them), interacting
    with the shared epilogue.
- **Next step:** log every `call_indirect` whose target is not a function start
  in thread mode, and compare call/return pairs against the unwind build on the
  same input. RTC is now deterministic.

The unwind-mode build (default) runs the full race path.
