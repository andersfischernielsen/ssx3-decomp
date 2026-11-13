# Build-time split: EE core, VU, hardware model

_Written 2026-10-15._ Goal: a change to the hardware model (GS, VU, VIF, DMA,
kernel HLE, SIF/IOP) should rebuild only the hardware crate(s) and relink,
not the ~52 MB of generated EE Rust in `generated/ee/*` (16 shard crates plus
the table crate `ssx3-ee`).

## Why everything used to rebuild

Generated EE crates depended on `ssx3-ps2`, which held the instruction
semantics *and* the whole machine (`Cpu` owned `Bus { hw: Hw }`, `Kernel`,
`Sif`, `Sched`). Cargo rebuilds every dependent whenever a dependency is
rebuilt, so any edit in `ssx3-ps2` recompiled all 17 generated crates.
Incremental compilation (the `game` profile has `incremental = true`) only
softens this: an edit that changes nothing a generated function uses (a
comment) costs ~18 s, but any real change, and especially a layout change of
a struct reachable from `Cpu`, costs 2-6 minutes.

## Layout now

```
generated/ee/s00..s15, generated/ee/game ──► ssx3-ee-rt   (no dependencies)
                                                 ▲
ssx3-vu  (Vu, timing model, fast path, microcode translator,
          generated/vu microcode) ───────────────┤
                                                 │
ssx3-ps2 (Machine: hw/DMAC/INTC/timers, VIF, GS/MTGS, IPU, kernel HLE,
          threads, SIF + IOP bridge) ──► ssx3-vu, ssx3-iop, ssx3-ee-rt
ssx3 (front ends) ──► ssx3-ps2, ssx3-ee (generated), ssx3-iop-gen
```

### `crates/ssx3-ee-rt`: the stable core

Only what generated functions reference directly:

- `Cpu`: GPRs, HI/LO, SA, FPU registers, COP0, `pc`, `cycles`,
  `next_event`, depth; `bus: Bus { ram, spr, plat }`; `hooks: Hooks`.
- Instruction semantics: `cpu.rs` (loads/stores, mult/div, COP0), `fpu.rs`,
  `mmi.rs`, `cop2.rs` (register moves), `float.rs` (PS2 float library, shared
  with the VU; everything hot is `#[inline]` because the VU calls it across the
  crate boundary).
- Guest memory: RAM and scratchpad are handled inline / in the core (`lw`/`sw`
  keep their inline RAM fast path); only I/O addresses go to the platform.
- `funcs.rs`: `FnEntry`, `register_functions`, `lookup`, `call_indirect`.
- `platform.rs`: `trait Platform: Vu0 + Any`, the vtable for I/O reads and
  writes (`read32/64/128`, `write8..128` with `ram`/`spr` for DMA),
  `set_now`, `irq_dirty`. `NullPlatform` for unit tests.
- `vu0.rs`: `trait Vu0`, one method per COP2 macro instruction (generated
  from a single op list, `for_each_vu0_op!`), plus `vf`/`set_vf`,
  `read_ctrl`/`write_ctrl`, `vcallms`, `vnop`, `macro_op`; the operand types
  `Src`, `Efu`, `X..W`.
- `Hooks`: plain `fn(&mut Cpu, ..)` pointers for what needs the whole
  machine: `event` (scheduler/interrupts), `syscall`, `jump_out`, `fatal`,
  `miss`.

The core has no dependencies and changes only when instruction semantics do.

### `ssx3-ps2`: the machine

`platform::Machine { hw, kernel, sif, sched, iop }` is the
`Box<dyn Platform>` in `c.bus.plat`; `ssx3_ps2::new_cpu()` builds a `Cpu`
with it and `platform::HOOKS`. Code in the crate reaches the machine with
`m!(c)` (`m!(c).kernel.current`, `m!(c).hw.gs`, ...), which downcasts only
the `c.bus.plat` field, so it combines with `c.g`, `c.bus.ram` etc. (`mr!`
for `&Cpu`). Front ends use `ssx3_ps2::m!` too. `ssx3_ps2::{cpu, bus, fpu,
mmi, Cpu, Qword}` and `machine::{call_indirect, FnEntry, ...}` are
re-exports, so older paths keep working.

`Machine` implements `Vu0` as "begin; per-op VU call; end" on
`hw.vu0`, exactly what the generated source did inline before
(`vu.macro_begin(); vu.add(..); vu.macro_end_upper();`) and what
`Vu::macro_op` does after decoding.

### `ssx3-vu`: the Vector Units

The `vu` module moved unchanged into its own crate (`ssx3_ps2::vu` is a
re-export). Reason: the translated microcode (`generated/vu`, 5.5 MB) was
part of `ssx3-ps2`, so even after the core split every GS or kernel edit
re-ran codegen for it (80-220 s). The VU only needs `ssx3-ee-rt` (float,
`Qword`, the operand types). `decode::Efu`'s timing moved to an extension
trait (`EfuLatency`) so the type can live in the core.

### Generator

`recomp-ee` emits against the core:

| before | now |
| --- | --- |
| `use ssx3_ps2::{Cpu, cpu::sx, cpu::reg::*, vu::{Src,..}, vu::decode::Efu}` | `use ssx3_ee_rt::{Cpu, cpu::sx, cpu::reg::*, funcs::call_indirect, vu0::{Efu, Src, ..}}` |
| `ssx3_ps2::machine::call_indirect(c, T)` | `call_indirect(c, T)` |
| `ssx3_ps2::kernel::syscall(c, PC)` | `c.syscall(PC)` |
| `ssx3_ps2::machine::jump_out(c, ..)` / `fatal(c, ..)` | `c.jump_out(..)` / `c.fatal(..)` |
| `{ c.bus.hw.vu0.macro_begin(); c.bus.hw.vu0.madd(..); c.bus.hw.vu0.macro_end_upper(); }` | `c.vu0().madd(..)` |

Normalising those spellings, the regenerated output is identical to the
previous one (all 16 shards, 0 differing lines). `recomp-ee` and `recomp-iop`
now write files only when their contents change (`write_if_changed`) and no
longer delete `generated/ee` first, so a regeneration that touches a few
shards recompiles only those.

## Cost at run time

The RAM fast paths stay inline; I/O accesses, COP2 and the hooks are one
indirect call each (VU0 macro ops were already out-of-line calls). Scripted
run to frame 9000 (fixed RTC, GPU GS, MTGS), same machine, back to back:

| | full run | race frames 4320-4680 (5-6 runs) |
| --- | --- | --- |
| before (e201c33) | 125 s | 37.1-40.4 fps, mean ~38.5 |
| after | 118-127 s | 37.8-40.4 fps, mean ~39.5 |

All 18 screenshots (frames 500-9000) are byte-identical to the
`overnight-playable` baseline, at every step of the split. With another heavy
process on the machine the numbers drop for both builds (~25 fps); interleave
runs when measuring.

## Rebuild times

`cargo build --profile game -p ssx3`, `CARGO_BUILD_JOBS=6`, `nice -n 10`,
Apple Silicon, after a previous build of the same tree. Each probe is a small
real edit (a statement in a function body, or a new field in a struct and its
constructor); the probes run in sequence, so each build also reverts the
previous probe (same sequence before and after).

| change | before: time (crates) | core split only | final (core + VU split) |
| --- | --- | --- | --- |
| GS function body (`gs/raster.rs`) | 129 s (19) | 81 s (2) | **3 s** (2) |
| GS struct field (`Mtgs`) | 344 s (19) | 84 s (2) | **3 s** (2) |
| kernel function body (`syscall`) | 342 s (19) | 156 s (2) | **3 s** (2) |
| kernel struct field (`Kernel`) | (not run) | 223 s (2) | **3 s** (2) |
| VU function body (`Vu::flush`) | 342 s (19) | 153 s (2) | 155 s (3) |
| VU struct field (`Vu`) | (not run) | 183 s (2) | 182 s (3) |
| IOP (SPU2) body / field | n/a | 86 / 87 s (4) | **5 / 19 s** (4) |
| EE core (`ssx3-ee-rt`) body | n/a | 194 s (22) | not re-measured (recompiles all generated code) |
| clean build of the game | 462 s | 590 s (after regenerating) | |

"Crates" counts the crates cargo recompiled: before, every hardware change
recompiled `ssx3-ps2`, all 16 shards, `ssx3-ee` and `ssx3`; now a GS or
kernel change recompiles `ssx3-ps2` and `ssx3` (the 3 s are mostly the
link), a VU change `ssx3-vu` + `ssx3-ps2` + `ssx3`. A manual GS check (new
`Mtgs` field used in `sync`) rebuilt in 3.3 s and changed the binary.

Remaining slow cases:

- **VU changes** (~2.5-3 min) recompile the translated microcode, which
  inlines the VU fast path. That is inherent to static translation; splitting
  the microcode from the `Vu` runtime would not help (cargo rebuilds
  dependents of any changed crate).
- **Core changes** recompile all generated code. The core only holds
  instruction semantics, so this should be rare.
- **Regeneration** (`recomp-ee`) recompiles the shards whose output changed.
  After the merge (2026-10-15, main checkout) every shard changed (new call
  spellings), so this was effectively a clean build: `recomp-ee` 19 min (almost
  all of it compiling `ssx3-vu` with the translated microcode in release mode,
  because the generator depends on it), then the game 39 min on a loaded
  machine. Later regenerations only rebuild shards whose output changed.

The IOP side was checked as well: `generated/iop/modules` (3.4 MB) depends on
`ssx3-iop`, but rebuilding it runs in parallel with `ssx3-ps2` and costs
5-19 s in total, so no split was needed there.

## Rules for keeping it fast

- Don't add anything to `ssx3-ee-rt` that is not referenced by generated
  code. New hardware state belongs in `Machine` (or `Hw`, `Kernel`, ...).
- A new kind of generated call into the machine goes through `Platform`,
  `Vu0` or `Hooks` (one indirect call), never through a direct path into
  `ssx3-ps2`.
- `ssx3-vu` must not depend on `ssx3-ps2`.
- To see what a change costs: `cargo build --profile game -p ssx3 -v` lists
  the crates being recompiled; after a hardware-only change it should not
  list any `ssx3-ee-s*`.

## Verification after merging into overnight-playable (2026-10-15)

- Merged as e5b2d28 (one conflict: the headless pad-2 input line), regenerated,
  rebuilt.
- Scripted race to frame 9000 with the progressed memory card: all 18
  screenshots byte-identical between the pre-merge binary and the merged one
  (107 s vs 113 s, back to back), no halts or misses.
- `touch crates/ssx3-ps2/src/gs/raster.rs` + `cargo build --profile game -p
  ssx3`: **2.4 s**, recompiling only `ssx3_ps2`, `ssx3` and `ssx3_headless`.
- Generator decoupled (same day): `ssx3-recomp` no longer depends on
  `ssx3-ps2`. The translated microcode in `ssx3-vu` is behind the default
  `microcode` feature and the recompiler uses `ssx3-vu` with
  `default-features = false`; the ISO reader moved to `ssx3_core::iso`
  (re-exported as `ssx3_ps2::iso`). `cargo build --release --bin recomp-ee`
  now takes **4.9 s** (was ~19 min cold), regeneration output is unchanged,
  and a GS edit rebuilds the game in 1.5 s.
