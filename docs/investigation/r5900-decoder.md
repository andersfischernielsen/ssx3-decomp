# R5900 decoder

`ssx3-recomp::r5900` decodes little-endian words into a structured
`Instruction { word, mnemonic, operands }`. Mnemonics are canonical MIPS, not
Ghidra's pseudo-instructions.

## Coverage (whole image)

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

## Encoding notes that bit us

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

## Validation

- `tests/decoder.rs` — 18 encoding tests, every expected word hand-assembled
  from the ISA (not from Ghidra).
- `tests/ghidra_golden.rs` — compares against Ghidra's disassembly of 7 functions
  (`generated/ghidra_disasm/`, 600 instructions). We require that we cover
  Ghidra's instructions word-for-word and that any extra tail is small. Ghidra's
  *mnemonics* are not compared because of its pseudo-instruction rendering.

## Regenerating the golden corpus

```sh
tools/export-ghidra-disasm.py                 # default 7-function spread
tools/export-ghidra-disasm.py 0x00100008 ...  # specific functions
```
