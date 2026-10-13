# Plan: structured control flow

## Problem

Generated functions currently use a per-function basic-block dispatcher:

```rust
pub fn f_00100008(c: &mut Cpu) {
    let mut pc: u32 = 0x00100008;
    while c.trap.is_none() { match pc { /* arms */ } }
}
```

It is *not* an emulator — no instruction is fetched — but it is not what a Rust
developer would write. The target is structured Rust: `if`/`else`, `loop`,
`while`, `break`, and early `return`, with no `pc` variable.

## Design

Work from the function's CFG, built on the **authoritative boundaries** (so
embedded data is already excluded) and branch/jump targets.

1. **Basic blocks.** Split at the entry, at every branch/jump target, and after
   every control transfer (+ its delay slot). Record `(start, end, successors)`.

2. **Structure (Relooper-lite).**
   - Chain blocks with a single successor (`A -> B` with `A` falling through).
   - A conditional branch (`beq`/`bne`/`blez`/…) becomes
     `if cond { <taken region> } else { <fallthrough region> }` followed by the
     join region.
   - A backward edge (`target <= block`) becomes a `loop { … }` with the loop
     header as the entry; exits use `break`.
   - `jal` → direct call; `jr $ra` → `return`; `j` → structure the target as a
     fall-through region.
   - Compute join points with a simple post-dominator / reachability check; when
     a clean structure cannot be proven, **fall back** to a labeled form
     (`'flow: loop { … }` with `continue 'flow`) rather than a `pc` state machine.

3. **Verification.** For every function, walk the *structured* output back to a
   reachable-block set (or keep the CFG alongside) and assert it equals Ghidra's
   boundaries. Add a round-trip test over the corpus and a sample of the image.

## Done means

- No `pc` variable and no `match pc` in generated code.
   - The golden corpus functions (`entry`, …) compile and behave identically.
   - A test asserts the structured output's reachable set matches Ghidra's
     boundaries for the corpus and ≥ N sampled functions.

## Status

Implemented (in `codegen.rs`):

- CFG construction from the authoritative boundaries (leaders, successors).
- Immediate post-dominators (bitset post-dominance + minimal candidate).
- Recursive emitter: fall-through chains, forward `j`, `jal` (direct call),
  `jr $ra` (`return`), and 2-way branches as `if/else` with the ip-dom join.

Measured: **43/300** functions (14.3%) emit fully structured Rust; the rest fall
back to the dispatcher. `entry` falls back because it contains a BSS-clear loop.

**Next:** loop support. Detect back edges (target ≤ source), wrap the natural
loop in `loop { … }`, turn back edges into `continue` and loop exits into
`break`. That should lift coverage substantially (most compiled functions loop).


## Risks

- Irreducible CFGs (rare from GCC but possible) need the labeled fallback, which
  must still be structured Rust, not a state machine.
- Delay slots must be folded into the branch arm exactly once.
