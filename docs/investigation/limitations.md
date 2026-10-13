# Known limitations and open questions

## Linear sweep vs. real instruction boundaries

`Program::decode` is a **linear sweep**: it steps 4 bytes at a time across a
function body. Two real problems were observed:

1. **Embedded data inside a function.** `FUN_0042d470`'s body is 1,640 bytes but
   Ghidra disassembles only 407 instructions; the trailing 12 bytes are non-zero
   data (a jump table), which the sweep decodes as bogus instructions.
2. **Ghidra's listing has holes.** Within `FUN_0042d470` the disassembly jumps
   `0x42d580` → `0x42d588`, `0x42d5e8` → `0x42d5f0`, `0x42d860` → `0x42d868`.
   Ghidra did not disassemble those 4-byte slots at all (undefined/unreachable
   bytes); a sweep fills them in.

Consequence: for translation we need **authoritative per-instruction
boundaries**, obtained by either

- recursive-descent disassembly from entries and branch/jump targets, or
- exporting Ghidra's per-instruction addresses (the "export disassembly"
  option), or
- exporting the function's instruction-set from Ghidra.

Until then the golden test bounds the tail and compares by address, not index.

## Decoder gaps

2,123 of 801,242 instructions are unrecognised (opcode `0x11` COP1 formats:
2,089; REGIMM: 34). See [`r5900-decoder.md`](r5900-decoder.md).

## Symbol recovery

The ELF is stripped, but `-ffunction-sections` left mangled C++ names in section
names (`RCMP5CODEC`, `RCMP7DECODER`, SONY PS2 codec classes, ...). These have not
been applied to functions yet — a cheap, high-value naming pass.

## Execution and code generation

- **Control flow is scaffolding.** Generated functions use a per-function
  basic-block dispatch (`match pc`). It is correct but not the target: structured
  Rust (`loop`/`if`, no labels) must replace it.
- **FPU not modelled.** COP1 arithmetic/compares lower to no-ops.
- **`lq`/`sq`** use only the low 64 bits; **HI/LO** is `u64`, not the R5900's
  128-bit pair, so `madd`/`pmfhl` and friends are approximated.
- **`clear fsa`/`clear fs`/`clear fcsr`** (Ghidra pseudo-ops) are no-ops.
- **Indirect `jr`/`jalr`** stop execution rather than resolve a target.
- **Syscalls** that are not a compile-time constant fall back to a logging stub.

## Not yet modelled

- DMA, timers, INTC, and the GIF/GS packet path (VRAM is presented, not produced).
- VU0/VU1 microcode (`.vutext` exists; no VU model).
- The game beyond the entry call graph: only the functions `recomp-gen` has
  translated can run; everything else stops at `untranslated_call`.
