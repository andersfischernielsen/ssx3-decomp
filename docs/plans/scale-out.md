# Plan: scale-out to the whole inventory

## Problem

`recomp-gen` currently translates only the direct-call closure of a start
function (≈14–103 functions). The goal is Rust for all **11,844**.

## Scale

At ~4554 instructions for ~103 functions (≈44 instructions/function), the whole
image is ≈784k instructions. Rough estimate: 10–40 MB of generated Rust. rustc
will not comfortably compile that as one crate.

## Design

1. **Deterministic sharding.** Group functions into modules by address ranges
   (e.g. 64 KiB blocks → ≈50 modules, or by call-graph cluster). Emit
   `generated/mod.rs` with `pub mod g0x0010; …` and a `dispatch`/direct-call
   table only where needed.
2. **Manifest first.** A tool walks the inventory and writes a manifest (module →
   [addresses]) without emitting bodies, so the layout is reviewable and stable.
3. **Emit, don't compile, by default.** `recomp-gen --all` writes every module;
   compiling is a separate, opt-in step (`--emit-crate` / cargo feature) because
   it is slow. CI compiles a sample (e.g. the `entry` closure) to keep the
   pipeline honest.
4. **Progress is measurable.** Report: functions emitted, instructions emitted,
   unsupported-lowering count per module, and bytes of Rust.

## Done means

- `recomp-gen --all` emits Rust for every function in the inventory.
- A sampled subset (e.g. 200 functions spread across the image) compiles and the
  reachable-set test passes.
- The unsupported-lowering list is a tracked TODO with counts.

## Risks

- Compile time/size. Mitigate with sharding, `opt-level=0` for generated code,
  and not compiling the whole crate in CI.
- Determinism: output must be reproducible from `generated/*.tsv` + the ELF.
