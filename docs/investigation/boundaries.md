# Instruction boundaries

## Why they are needed

A function body is **not** a contiguous run of instructions:

- it can contain **embedded data** (jump tables);
- Ghidra may leave **holes** where it never disassembled bytes.

A blind linear sweep misdecodes both — e.g. it turns `FUN_0042d470`'s jump table
into bogus instructions, and it cannot know that `beq $0,$0,off` (an
unconditional `b`) has no fall-through, so it walks into the data that follows.

## Authoritative map (Ghidra)

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

## Recursive descent (our own)

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

## Commands

```sh
tools/export-ghidra-boundaries.py                 # regenerate boundaries.tsv
recomp-stats --flow                               # recursive descent vs Ghidra
recomp-stats --coverage                           # decoder coverage (boundaries-aware)
```
