# Code generation (R5900 → Rust)

**This is a recompiler, not an emulator.** The intended output is portable Rust
that a compiler could have produced. Nothing fetches or interprets instructions
at runtime, and there is no cross-platform host assumption.

## What is generated today

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

## Scaffolding still in place (and why)

Control flow inside a function is currently a **per-function basic-block
dispatch** (`while c.trap.is_none() { match pc { ... } }`). It is correct — it
uses the same authoritative boundaries and branch targets — but it is scaffolding
for the real goal: recovering **structured** Rust (`loop`, `if`/`else`, no labels)
from the CFG. A real decompilation should not ship this form.

## Known approximations

- **FPU is not modelled.** COP1 arithmetic and compares lower to no-ops so the
  init path can run. Real port needs a proper `f32`/`f64` model.
- **`lq`/`sq`** (128-bit pair loads/stores) use only the low 64 bits.
- **HI/LO** is `u64`, not the R5900's 128-bit accumulator pair; `madd` and friends
  are approximated.
- **`clear fsa`/`clear fs`/`clear fcsr`** (Ghidra pseudo-ops for REGIMM/COP1
  encodings) are treated as no-ops.

## Measured execution

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
