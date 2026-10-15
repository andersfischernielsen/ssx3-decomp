# Phase 2 worker guide: replacing small generated functions

This guide is for workers (OpenCode or other agents) who rewrite small
generated functions of the SSX 3 decompilation as hand-written, typed Rust.
Read it completely before starting.

## What you are doing

The game's code was translated mechanically into Rust in
`generated/ee/sNN/src/lib.rs` (one `pub fn f_XXXXXXXX(c: &mut Cpu)` per game
function, where XXXXXXXX is its address; named ones have a doc comment
`/// \`Name\``). You rewrite a function as readable Rust in the module file you
are given (e.g. `crates/ssx3-game/src/oc1.rs`). Your version must behave
**exactly** like the generated one: same memory writes, same register results,
same cycle count. A differential test proves it.

## Rules

1. Edit ONLY your module file. Never edit other files (shared helpers,
   `lib.rs`, `Cargo.toml`, `overrides.txt`, generated code). Never commit.
2. Copy the generated function's logic exactly, including:
   - every `c.cycles += N;` at the same points (sum them per branch);
   - every register the generated code leaves set at the end, notably
     `v0` (`c.g[V0][0]`), `v1` (`c.g[V1][0]`) — even "scratch" values the
     generated code writes into v1 must be reproduced — and FPU `c.f[0]`,
     `c.f[1]`, and the FPU condition bit (`abi::set_fcc` after compares);
   - loads/stores with the same widths and sign extension: use
     `Addr(x).lw(c)` (sign-extended 32-bit load, returns u64),
     `Addr(x).lwu(c)`, `.lh/.lhu/.lb/.lbu/.ld`, `.sw(c, v)`, `.sh`, `.sb`,
     `.sd`;
   - 64-bit vs 32-bit comparisons exactly as written in the generated code
     (`c.g[A0][0]` is a u64; `as u32` means 32-bit);
   - return with `abi::ret(c)` (sets `c.pc = ra`).
3. Floats: FPU code (`c.add_s`, `c.mul_s`, `c.lwc1`...) must use
   `ps2f::FpuF32` (`+ - *` operators, `.div`, `.sqrt`, `.abs`, `.max`, `.min`,
   `FpuF32::madd` = acc + a*b rounded twice, `.lt/.le/.eq_s` compares, then
   `abi::set_fcc(c, cond)`), never native `f32` arithmetic.
4. Make it readable: a doc comment saying what the function does in game terms
   (and its signature), meaningful variable names, typed field accessors with
   named offsets where the meaning is clear (`const STATE: u32 = 0x54;`).
5. Write one differential test per function and run it. Only add a function
   to `OVERRIDES` when its test passes with no differences. If you cannot
   make one match, leave it out and say why in your final message.

## Helpers (crate::...)

- `abi::arg(c, i) -> u32` argument i (a0..a3,t0..t3); `abi::arg64(c, i)`;
  `abi::set_ret(c, v: u32)` (v0 = sign-extended); `abi::set_word(c, reg, v)`;
  `abi::ret(c)`; `abi::farg(c, i) -> FpuF32` (f12..); `abi::set_fret(c, v)`
  (f0); `abi::set_fcc(c, bool)`; `abi::mult(c, a, b)` (sets LO/HI).
- `guest::Addr(u32)` with `.off(n)`, the loads/stores above, `.read_fpu(c)`,
  `.write_fpu(c, v)`.
- `ssx3_ee_rt::cpu::{reg::*, sx}`: register names `V0, V1, A0.., T0..,
  RA, SP, GP`; `sx(u32) -> u64` sign extension.

## Worked example

Generated code (`generated/ee/s09/src/lib.rs`, `MenuSys_IsActive`, 0x2cc098):

```rust
pub fn f_002cc098(c: &mut Cpu) {
'b_2cc0ac: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0u64;
if t {
break 'b_2cc0ac;
} else {
c.cycles += 2;
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x54));
c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
break 'b_2cc0ac;
}
}
c.cycles += 2;
c.g[V0][0] = c.g[V1][0];
let t = c.g[RA][0] as u32;
c.pc = t; return;
}
```

Replacement:

```rust
/// `bool MenuSys_IsActive(MenuSys* m)` (0x2cc098): false if the first word
/// is 0, else whether the word at +0x54 is non-zero.
pub fn menu_sys_is_active(c: &mut Cpu) {
    c.cycles += 3;
    let menu = Addr(abi::arg(c, 0));
    let mut active = 0;
    if menu.lw(c) != 0 {
        c.cycles += 2;
        active = (menu.off(0x54).lw(c) != 0) as u64;
    }
    c.cycles += 2;
    c.g[V1][0] = active;
    c.g[V0][0] = active;
    abi::ret(c);
}
```

Test (inside `#[cfg(test)] mod tests { use super::*; ... }` at the end of your
module):

```rust
use crate::difftest::{check, scramble, Rng, DATA};

#[test]
fn menu_sys_is_active_matches() {
    check(ssx3_ee::s09::f_002cc098, menu_sys_is_active, 300, &|c, seed| {
        let mut r = Rng(0x1234_5678 ^ seed as u64);
        scramble(c, &mut r);          // random junk in all registers first
        c.g[A0][0] = sx(DATA);        // the object lives at DATA
        let first = if seed % 3 == 0 { 0 } else { r.next_u32() };
        c.bus.write32(DATA, first);   // cover both branches
        c.bus.write32(DATA + 0x54, if seed % 4 == 0 { 0 } else { r.next_u32() });
    });
}
```

`check(generated, yours, seeds, setup)` runs both from the same state and
fails with a list of differences (registers, cycles, memory). Make the setup
cover every branch (choose values by `seed % n`). For FPU arguments set
`c.f[12]`, `c.f[13]`... with `r.edge_f32()` (edge cases) or `r.f32(range)`.
Pointer arguments: point them at `DATA` (or `DATA + k`), and write the fields
the function reads with `c.bus.write32`/`write16`/`write8`.

The module starts as:

```rust
use ssx3_ee_rt::funcs::GuestFn;
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];
```

Add `use crate::{abi, guest::Addr}; use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::Cpu;` (and `crate::ps2f::FpuF32` if needed) and list each
finished function: `(0x002c_c098, menu_sys_is_active, "MenuSys_IsActive"),`.

## Test speed (gate)

Integration accepts a function only if its test passes **and finishes within
5 seconds** (run per test by `cargo nextest`, see `.config/nextest.toml`).
So: keep seeds at 200-300, build fixtures cheaply, and don't let real
callees run long loops on test data (prefer setups that take the short
path, or fakes). A test that times out rejects the function.

## Commands

Run your tests (replace `ocN` with your module/feature name):

```sh
export CARGO_BUILD_JOBS=2
nice -n 10 cargo test --profile game -p ssx3-game --features ocN ocN:: 2>&1 | tail -40
```

It may wait for a build lock ("Blocking waiting for file lock") while other
workers build; that is normal, just wait. Do not run any other cargo command,
do not run `recomp-ee`, do not build the game.

Finish with a short report: done functions, skipped ones and why.
