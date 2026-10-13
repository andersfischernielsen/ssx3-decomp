# FPU (COP1)

The EE FPU has 32 registers; `.s` (single) values occupy the low 32 bits of each
register, `.d` (double) values use the 64-bit register (and conventionally
overlap pairs). Runtime state is `Cpu::fpr: [u64; 32]` with helpers `f32`/`set_f32`
/`f64`/`set_f64`, plus `fcr31_cc: bool` for the condition code.

## Encoding reminders

COP1 uses the *FPU* field layout, not the integer R-type layout:

```
opcode(6)=0x11 | fmt(5) | ft(5) | fs(5) | fd(5) | funct(6)
```

`fmt` (= `rs`) is 16 for `.s`, 17 for `.d`, 20 for `.w`, 21 for `.l`. The
decoder maps `fs` to bits 15..11 and `fd` to bits 10..6 (see
`docs/investigation/r5900-decoder.md` — this was a bug we hit).

## What is implemented

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

## Known approximations

- **`adda`/`suba`/`mula`/`diva`** are lowered as the non-accumulating forms.
  `entry` uses `adda.S`; verify semantics before trusting results.
- **`cvt.*`** is unimplemented: the decoder records the destination format in the
  mnemonic (`cvt.s`, `cvt.w`) but **drops the source format** (`fmt`). To do it
  right, carry `fmt` through `Operands::Cop1Arith` (or name `cvt.s.w` etc.).
- **Signaling vs quiet** compares are treated alike for the condition code.
- **FCR31** rounding/flags are not modelled.
