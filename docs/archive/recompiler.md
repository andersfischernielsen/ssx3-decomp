# The EE recompiler (historical notes)

_Archived 2026-11-13._ How `recomp-ee` and its inputs were built, written during phase 1
(October 2026). Several details are out of date (host-thread modes, the old `codegen.rs`
scaffolding). The generator itself is still how
`generated/ee` is made (`python3 tools/integrate_wave.py regen`); its source is
`crates/ssx3-recomp/`. The function list is `generated/functions.tsv`; names live in Ghidra.


## recomp-ee: whole-program EE → Rust (2026-10-14)

_Written 2026-10-14._ Supersedes the `codegen.rs` / `recomp-all` scaffolding
for running the game. `codegen.rs` is kept for now, but its output is not used.

### What it is

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

### Code shape

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

### Calls, returns, threads (the unwind/trampoline model)

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

### Known gaps

- VU0 macro arithmetic is still lowered to `c.vu0_macro(word)`, which decodes the
  word inside the VU library at run time. It will move to per-op calls once the
  VU library exposes them; see `vu-vif.md`.
- Integer-overflow traps (`add`, `sub`) and conditional traps (`teq` …) are not
  modelled.
- Timing is one cycle per instruction.
- The block-dispatch form is correct but not idiomatic Rust. Structured control
  flow and register promotion come later; correctness comes first.

### Structured output via host threads (implemented: `--threads`, `--structured`)

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

#### Status (2026-10-14)

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

#### Open bug in host-thread modes (2026-10-14)

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


## Function inventory (Ghidra export)

### Export

`tools/export-ghidra-functions.py` talks to the GhidraMCP backend:

- Phase 1 pages `/functions` (page cap 2000) for names.
- Phase 2 queries `/function?address=` per entry for the exact `body_end`.

Output (committed, ~600 KB):

```
generated/functions.json   [{ "name", "address", "end" }] sorted by address
generated/functions.tsv    "<hex-address>\t<hex-end>\t<name>"  (embedded by ssx3-recomp)
```

`end` is **exclusive** (Ghidra's inclusive `body_end` + 1).

### Shape

| | |
| --- | --- |
| functions | **11,844** |
| entry range | `00100008` .. `0042e508` |
| bodies | 8 B .. 40,536 B; 3,204,968 B total |
| recovered names | **75** (`entry`, `SetGsCrt`, `GsPutIMR`, `AddIntcHandler`, `_Exit`, `_LoadExecPS2`, `SetVTLBRefillHandler`, ...) |
| auto placeholders | 11,769 (`FUN_…`) |

All function entries are within `.text` (11,843) plus one in a linkonce section.

### Why extents come from Ghidra, not neighbours

Deriving a function's end from the next entry's address **overestimates**: e.g.
`FUN_001001c8` is only 8 bytes while the next entry is 16 bytes later (there is
alignment/other data between functions). Ghidra's `body_end` is authoritative and
is what the inventory stores.

### Query tools

`ssx3-recomp` embeds the TSV; `FunctionIndex` offers `by_address`, `containing`
(entry-inclusive, end-exclusive), `search` and named-vs-auto classification. The
`recomp-stats` CLI wraps it:

```sh
recomp-stats                 # summary + named list
recomp-stats 0x0012ab34      # address lookup (or containing function)
recomp-stats SetGsCrt        # name search
recomp-stats --coverage      # decode the whole image, report coverage
recomp-stats --disasm entry  # disassemble a function
```


## Instruction boundaries

### Why they are needed

A function body is **not** a contiguous run of instructions:

- it can contain **embedded data** (jump tables);
- Ghidra may leave **holes** where it never disassembled bytes.

A blind linear sweep misdecodes both — e.g. it turns `FUN_0042d470`'s jump table
into bogus instructions, and it cannot know that `beq $0,$0,off` (an
unconditional `b`) has no fall-through, so it walks into the data that follows.

### Authoritative map (Ghidra)

`tools/export-ghidra-boundaries.py` records, per function, the contiguous
instruction runs from Ghidra's listing:

```
generated/boundaries.tsv
<hex-func-addr>\t<hex-start>-<hex-end>[,<start>-<end>...]     (end exclusive)
```

Measured over all 11,844 functions:

| | |
| --- | --- |
| instructions | **783,966** |
| naive linear-sweep total | 801,242 |
| difference (data/padding) | 17,276 |
| functions with holes | **224** |

`Program::decode` decodes against these runs, so data is skipped. `FUN_0042d470`
decodes to 407 instructions over 4 runs.

### Recursive descent (our own)

`ssx3-recomp::flow::disassemble` reconstructs boundaries from the bytes alone,
following fall-through, both edges of branches (after the delay slot), direct
`j`/`jal` targets and `jal`/`jalr` returns. It does **not** follow indirect `jr`
targets (jump tables) and stops at `eret`. It correctly treats `beq $0,$0,off`
as an unconditional branch.

Compared with Ghidra over the whole image:

| | |
| --- | --- |
| functions exact | **8,388 (70.8%)** |
| instructions matched | 573,030 |
| extra (decoded data) | 9,019 |
| missing | 210,936 |

The `missing` set is code Ghidra reached through jump tables / pointer analysis
that recursive descent cannot follow. This is why **Ghidra's map stays
authoritative** and recursive descent is a cross-check and a fallback for code
Ghidra has not seen. Closing the gap would need jump-table recovery.

### Commands

```sh
tools/export-ghidra-boundaries.py                 # regenerate boundaries.tsv
recomp-stats --flow                               # recursive descent vs Ghidra
recomp-stats --coverage                           # decoder coverage (boundaries-aware)
```


## R5900 decoder

`ssx3-recomp::r5900` decodes little-endian words into a structured
`Instruction { word, mnemonic, operands }`. Mnemonics are canonical MIPS, not
Ghidra's pseudo-instructions.

### Coverage (whole image)

Measured with `recomp-stats --coverage` over all 11,844 functions:

| | |
| --- | --- |
| instructions | **801,242** |
| decoded | 799,119 (**99.74%**) |
| unrecognised | 2,123 |

Remaining gaps, by opcode:

| Opcode | Count | Meaning |
| --- | --- | --- |
| `0x11` | 2,089 | COP1 arithmetic formats we do not name yet (e.g. long/`.ps` formats) |
| `0x01` | 34 | REGIMM `rt` sub-ops not in the table |

Unrecognised encodings are preserved as `Operands::Word` with the raw `word`, so
nothing is dropped — they are a to-do list, not lost data.

### Encoding notes that bit us

- **Integer R-type**: `rs[25:21] rt[20:16] rd[15:11] sa[10:6] funct[5:0]`.
- **COP1 R-type is different**: `fmt[25:21] ft[20:16] fs[15:11] fd[10:6]
  funct[5:0]`. The `fs`/`fd` fields are swapped relative to integer R-type. Our
  first attempt got this wrong; `decode_cop1` now reads `fs` from bits 15..11 and
  `fd` from bits 10..6.
- **MMI** (opcode `0x1C`): `mmiop` is the `sa` field (`[10:6]`) and `fct` is the
  funct field (`[5:0]`). Confirmed against the emotion-engine Sleigh.
- **`clear rd`** is `mmiop = 0x10`, `fct = 0x28`, with `rs = rt = 0`. It is the
  first instruction of `entry` (`0x70000c28` → `clear at`). Ghidra also renders
  `mthi $0` as `clear hi`, `mthi1` as `clear hi1`, etc. — those are pseudo-forms
  of ordinary instructions, which we decode normally.
- **COP0 control ops**: `rs = 0x10`; `funct 0x18` = `eret`, `0x38` = `ei`,
  `0x39` = `di`. This is how `entry`'s `EI` decodes.
- **`adda.S`** in `entry` is COP1 `fmt = s` with `funct 0x18` (`adda`).

### Validation

- `tests/decoder.rs` — 18 encoding tests, every expected word hand-assembled
  from the ISA (not from Ghidra).
- `tests/ghidra_golden.rs` — compares against Ghidra's disassembly of 7 functions
  (`generated/ghidra_disasm/`, 600 instructions). We require that we cover
  Ghidra's instructions word-for-word and that any extra tail is small. Ghidra's
  *mnemonics* are not compared because of its pseudo-instruction rendering.

### Regenerating the golden corpus

```sh
tools/export-ghidra-disasm.py                 # default 7-function spread
tools/export-ghidra-disasm.py 0x00100008 ...  # specific functions
```


## FPU (COP1) lowering

The EE FPU has 32 registers; `.s` (single) values occupy the low 32 bits of each
register, `.d` (double) values use the 64-bit register (and conventionally
overlap pairs). Runtime state is `Cpu::fpr: [u64; 32]` with helpers `f32`/`set_f32`
/`f64`/`set_f64`, plus `fcr31_cc: bool` for the condition code.

### Encoding reminders

COP1 uses the *FPU* field layout, not the integer R-type layout:

```
opcode(6)=0x11 | fmt(5) | ft(5) | fs(5) | fd(5) | funct(6)
```

`fmt` (= `rs`) is 16 for `.s`, 17 for `.d`, 20 for `.w`, 21 for `.l`. The
decoder maps `fs` to bits 15..11 and `fd` to bits 10..6 (see
"R5900 decoder" above — this was a bug we hit).

### What is implemented

`ssx3-runtime`: `fpr`, `fcr31_cc`, `f32/f64` accessors.
`ssx3-recomp::codegen`:

| Mnemonic(s) | Lowering |
| --- | --- |
| `mtc1`/`mfc1` | move between GPR and FPU low 32 bits |
| `cfc1`/`ctc1` | FCR31 condition code (approximate) |
| `add/sub/mul/div/abs/neg/mov/sqrt` `.s`/`.d` | native `f32`/`f64` ops |
| `c.f/c.un/c.eq/c.ueq/c.olt/c.ult/c.ole/c.ule/c.sf/c.ngle/c.seq/c.ngl` `.s`/`.d` | set `fcr31_cc` |
| `bc1f`/`bc1t` | branch on `fcr31_cc` |
| `clear fs` / `clear fcsr` | no-ops (Ghidra pseudo-ops) |

### Known approximations

- **`adda`/`suba`/`mula`/`diva`** are lowered as the non-accumulating forms.
  `entry` uses `adda.S`; verify semantics before trusting results.
- **`cvt.*`** is unimplemented: the decoder records the destination format in the
  mnemonic (`cvt.s`, `cvt.w`) but **drops the source format** (`fmt`). To do it
  right, carry `fmt` through `Operands::Cop1Arith` (or name `cvt.s.w` etc.).
- **Signaling vs quiet** compares are treated alike for the condition code.
- **FCR31** rounding/flags are not modelled.

## The first code generator (`codegen.rs`, superseded by recomp-ee)

**This is a recompiler, not an emulator.** The intended output is portable Rust
that a compiler could have produced. Nothing fetches or interprets instructions
at runtime, and there is no cross-platform host assumption.

### What is generated today

`ssx3-recomp::codegen` turns a function's authoritatively-bounded instructions
into a Rust function over the runtime state:

- **Syscalls → native SDK calls.** The generator constant-folds the `$v1` number
  set by the preceding `li v1, N` (e.g. `li v1,0x3c` → `syscall`). Instead of a
  runtime syscall dispatcher it emits a direct call to a Rust function in
  `ssx3-runtime::sdk`:
  ```rust
  let a0 = c.reg(4); let a1 = c.reg(5); /* ... */
  let r = crate::sdk::init_heap(c, a0, a1);
  c.set_reg(2, r);
  ```
  Only a non-constant syscall number falls back to `sdk::dynamic_syscall`, which
  exists to log an unrecognised kernel call rather than miscompile it.
- **Direct calls.** A `jal` to a translated function becomes a direct Rust call
  (`f_0042c300(c);`), never a dispatch on an address. A `jal` to an untranslated
  function stops cleanly via `c.untranslated_call`.
- **Instruction lowering** for the integer/load-store/branch/MMI-hi-lo subset,
  with FPU and remaining MMI preserved.

### Scaffolding still in place (and why)

Control flow inside a function is currently a **per-function basic-block
dispatch** (`while c.trap.is_none() { match pc { ... } }`). It is correct — it
uses the same authoritative boundaries and branch targets — but it is scaffolding
for the real goal: recovering **structured** Rust (`loop`, `if`/`else`, no labels)
from the CFG. A real decompilation should not ship this form.

### Known approximations

- **FPU** arithmetic/compares/moves and `lwc1/swc1/ldc1/sdc1` are modelled;
  `cvt.s`/`cvt.d` are not (the decoder drops the source format), and
  `adda`/`mula` etc. are approximated. See "FPU (COP1) lowering" above.
- **MMI vector ops and COP2/VU0** are decoded but not lowered (they need 128-bit
  GPRs). `recomp-stats --todo` lists them.- **`lq`/`sq`** (128-bit pair loads/stores) use only the low 64 bits.
- **HI/LO** is `u64`, not the R5900's 128-bit accumulator pair; `madd` and friends
  are approximated.
- **`clear fsa`/`clear fs`/`clear fcsr`** (Ghidra pseudo-ops for REGIMM/COP1
  encodings) are treated as no-ops.

### Measured execution

`recomp-gen` translates the direct-call closure of `entry`; `ssx3-run` executes
it. At depth 2 (14 functions, 675 instructions):

```
steps      : 276657
registers  : v0=...01 v1=...3d gp=000000004a30f0 ra=0000000042c0fc
stopped    : 0x0042c0f4: call to untranslated function 00423da0
```

At depth 4 (103 functions, 4554 instructions) it executes the C runtime for tens
of millions of instructions, invoking `InitMainThread(0x3c)`, `InitHeap(0x3d)`,
`CreateSema(0x40)`, `SetSyscall(0x74)` and `FindAddress(0x83)`, and `gp` reaches
its real value `0x004a30f0`.

```sh
cargo run -p ssx3-recomp --bin recomp-gen -- --depth 4 --limit 400
cargo run -p ssx3-runtime --bin ssx3-run
```
