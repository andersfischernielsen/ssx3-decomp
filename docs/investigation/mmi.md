# R5900 MMI (opcode `0x1C`) mnemonic table + `clear`/Coprocessor encodings

Complete reference for implementing the MMI/Special2 decoder, extracted from the
Ghidra EmotionEngine-Reloaded Sleigh (`eecore.sinc`, `mips.sinc`, `base.sinc`,
`cop1.sinc`, `cop2.sinc`, `vuupper.sinc`) and cross-checked against the
in-repo [`docs/ps2-bible.md`](../ps2-bible.md) tables and
[psi-rockin/ps2tek](https://psi-rockin.github.io/ps2tek/).

> [!IMPORTANT]
> The MMI group selectors are **not** `fct ∈ {0x08,0x09,0x0A,0x0B}`. The Sleigh,
> `ps2-bible.md`, and ps2tek all use:
> `fct 0x08 = MMI0`, `fct 0x09 = MMI2`, `fct 0x28 = MMI1`, `fct 0x29 = MMI3`.
> `fct 0x0A`/`0x0B` are unassigned. See [Ambiguities](#8-ambiguities--sleigh-vs-manual).

## 0. Encoding layout

`prime` (opcode) is `word[31:26]`; MMI/Special2 uses `0x1C` (`0b011100`).

```
31..26   25..21  20..16  15..11  10..6     5..0
opcode     rs      rt      rd     sa      funct
0x1C                       rd     mmiop    fct
```

* `mmiop = word[10:6]` (the `sa` field, 5 bits)
* `fct   = word[5:0]`  (the `funct` field, 6 bits)

Word construction: `word = (0x1C << 26) | (rs<<21) | (rt<<16) | (rd<<11) | (mmiop<<6) | fct`.

The `with : prime=28 { ... }` block in `eecore.sinc` covers **both** the EE
integer multiply/divide Special2 ops (`madd`…`divu1`) **and** the MMI
groups. Group dispatch (checked first):

| `fct` | group | sub-selector |
| --- | --- | --- |
| `0x08` | MMI0 | `mmiop` |
| `0x09` | MMI2 | `mmiop` |
| `0x28` | MMI1 | `mmiop` |
| `0x29` | MMI3 | `mmiop` |

Operand shapes below use 128-bit GPR names: `rd=word[15:11]`,
`rs=word[25:21]`, `rt=word[20:16]`, `sa=word[10:6]=mmiop`.

---

## 1. MMI0 — `fct = 0x08` (25 entries)

| `mmiop` | mnemonic | operands (printed order) | Sleigh pcode helper |
| --- | --- | --- | --- |
| `0x00` | `paddw` | `rd, rs, rt` | `_paddw(RS,RT)` |
| `0x01` | `psubw` | `rd, rs, rt` | `_psubw(RS,RT)` |
| `0x02` | `pcgtw` | `rd, rs, rt` | `_pcgtw(RS,RT)` |
| `0x03` | `pmaxw` | `rd, rs, rt` | `_pmaxw(RS,RT)` |
| `0x04` | `paddh` | `rd, rs, rt` | `_paddh(RT,RS)` |
| `0x05` | `psubh` | `rd, rs, rt` | `_psubh(RS,RT)` |
| `0x06` | `pcgth` | `rd, rs, rt` | `_pcgth(RS,RT)` |
| `0x07` | `pmaxh` | `rd, rs, rt` | `_pmaxh(RS,RT)` |
| `0x08` | `paddb` | `rd, rs, rt` | `_paddb(RT,RS)` |
| `0x09` | `psubb` | `rd, rs, rt` | `_psubb(RS,RT)` |
| `0x0A` | `pcgtb` | `rd, rs, rt` | `_pcgtb(RS,RT)` |
| `0x10` | `paddsw` | `rd, rs, rt` | `_paddsw(RS,RT)` |
| `0x11` | `psubsw` | `rd, rs, rt` | `_psubsw(RS,RT)` |
| `0x12` | `pextlw` | `rd, rs, rt` | `_pextlw(RS,RT)` |
| `0x13` | `ppacw` | `rd, rs, rt` | `_ppacw(RS,RT)` |
| `0x14` | `paddsh` | `rd, rs, rt` | `_paddsh(RS,RT)` |
| `0x15` | `psubsh` | `rd, rs, rt` | `_psubsh(RS,RT)` |
| `0x16` | `pextlh` | `rd, rs, rt` | `_pextlh(RS,RT)` |
| `0x17` | `ppach` | `rd, rs, rt` | `_ppach(RS,RT)` |
| `0x18` | `paddsb` | `rd, rs, rt` | `_paddsb(RS,RT)` |
| `0x19` | `psubsb` | `rd, rs, rt` | `_psubsb(RS,RT)` |
| `0x1A` | `pextlb` | `rd, rs, rt` | `_pextlb(RS,RT)` |
| `0x1B` | `ppacb` | `rd, rs, rt` | `_ppacb(RS,RT)` |
| `0x1E` | `pext5` | `rd, rt` | `_pext5(RT)` |
| `0x1F` | `ppac5` | `rd, rt` | `_ppac5(RT)` |

> The internal helper argument order (e.g. `_paddb(RT,RS)`) is an implementation
> detail of the Sleigh's `pcodeop`s; the **disassembly operand order is always
> `rd, rs, rt`** (or `rd, rt` for the one-source forms).

## 2. MMI1 — `fct = 0x28` (18 entries)

| `mmiop` | mnemonic | operands | helper |
| --- | --- | --- | --- |
| `0x01` | `pabsw` | `rd, rt` | `_pabsw(RT)` |
| `0x02` | `pceqw` | `rd, rs, rt` | `_pceqw(RS,RT)` |
| `0x03` | `pminw` | `rd, rs, rt` | `_pminw(RS,RT)` |
| `0x04` | `padsbh` | `rd, rs, rt` | `_padsbh(RS,RT)` |
| `0x05` | `pabsh` | `rd, rt` | `_pabsh(RT)` |
| `0x06` | `pceqh` | `rd, rs, rt` | `_pceqh(RS,RT)` |
| `0x07` | `pminh` | `rd, rs, rt` | `_pminh(RS,RT)` |
| `0x0A` | `pceqb` | `rd, rs, rt` | `_pceqb(RS,RT)` |
| `0x10` | `padduw` | `rd, rs, rt` | `_padduw(RS,RT)` |
| `0x11` | `psubuw` | `rd, rs, rt` | `_psubuw(RS,RT)` |
| `0x12` | `pextuw` | `rd, rs, rt` | `_pextuw(RS,RT)` |
| `0x14` | `padduh` | `rd, rs, rt` | `_padduh(RS,RT)` |
| `0x15` | `psubuh` | `rd, rs, rt` | `_psubuh(RS,RT)` |
| `0x16` | `pextuh` | `rd, rs, rt` | `_pextuh(RS,RT)` |
| `0x18` | `paddub` | `rd, rs, rt` | `_paddub(RS,RT)` |
| `0x19` | `psubub` | `rd, rs, rt` | `_psubub(RS,RT)` |
| `0x1A` | `pextub` | `rd, rs, rt` | `_pextub(RS,RT)` |
| `0x1B` | `qfsrv` | `rd, rs, rt` | `_qfsrv(RS,RT)` |

## 3. MMI2 — `fct = 0x09` (22 entries)

| `mmiop` | mnemonic | operands | notes |
| --- | --- | --- | --- |
| `0x00` | `pmaddw` | `rd, rs, rt` | writes lo/hi |
| `0x02` | `psllvw` | `rd, rs, rt` | **note order** |
| `0x03` | `psrlvw` | `rd, rt, rs` | **note order** |
| `0x04` | `pmsubw` | `rd, rs, rt` | |
| `0x08` | `pmfhi` | `rd` | |
| `0x09` | `pmflo` | `rd` | |
| `0x0A` | `pinth` | `rd, rs, rt` | |
| `0x0C` | `pmultw` | `rd, rs, rt` | writes lo/hi |
| `0x0D` | `pdivw` | `rs, rt` | writes lo/hi, no `rd` |
| `0x0E` | `pcpyld` | `rd, rs, rt` | |
| `0x12` | `pand` | `rd, rs, rt` | |
| `0x13` | `pxor` | `rd, rs, rt` | |
| `0x14` | `pmaddh` | `rd, rs, rt` | |
| `0x15` | `phmadh` | `rd, rs, rt` | |
| `0x18` | `pmsubh` | `rd, rs, rt` | |
| `0x19` | `phmsdh` | `rd, rs, rt` | manual spells `PHMSBH` |
| `0x1A` | `pexeh` | `rd, rt` | |
| `0x1B` | `prevh` | `rd, rt` | |
| `0x1C` | `pmulth` | `rd, rs, rt` | |
| `0x1D` | `pdivbw` | `rs, rt` | writes lo/hi, no `rd` |
| `0x1E` | `pexew` | `rd, rt` | |
| `0x1F` | `prot3w` | `rd, rt` | |

## 4. MMI3 — `fct = 0x29` (13 entries)

| `mmiop` | mnemonic | operands | notes |
| --- | --- | --- | --- |
| `0x00` | `pmadduw` | `rd, rs, rt` | |
| `0x03` | `psravw` | `rd, rt, rs` | **note order** |
| `0x08` | `pmthi` | `rs` | single source operand |
| `0x09` | `pmtlo` | `rs` | single source operand |
| `0x0A` | `pinteh` | `rd, rs, rt` | |
| `0x0C` | `pmultuw` | `rd, rs, rt` | |
| `0x0D` | `pdivuw` | `rs, rt` | writes lo/hi, no `rd` |
| `0x0E` | `pcpyud` | `rd, rs, rt` | |
| `0x12` | `por` | `rd, rs, rt` | |
| `0x13` | `pnor` | `rd, rs, rt` | |
| `0x1A` | `pexch` | `rd, rt` | |
| `0x1B` | `pcpyh` | `rd, rt` | |
| `0x1E` | `pexcw` | `rd, rt` | |

> `psllvw` is the odd one out: Sleigh prints `psllvw rd, rs, rt` but
> `psrlvw`/`psravw` print `rd, rt, rs`. See [Ambiguities](#8-ambiguities--sleigh-vs-manual).

---

## 5. Single-opcode MMI / Special2 forms (not in MMI0–3)

All are opcode `0x1C`; selected directly by `fct` (the `mmiop`/`sa` field is
either free, zero-constrained, or is the operand).

| `fct` | mnemonic | operands | notes |
| --- | --- | --- | --- |
| `0x00` | `madd` | `rs, rt` if `rd=0`; else `rd, rs, rt` | lo/hi, `sa=0` |
| `0x01` | `maddu` | `rs, rt` if `rd=0`; else `rd, rs, rt` | lo/hi, `sa=0` |
| `0x02` | `msub` | `rs, rt` if `rd=0`; else `rd, rs, rt` | **not in Sleigh** (manual/ps2tek only) |
| `0x03` | `msubu` | `rs, rt` if `rd=0`; else `rd, rs, rt` | **not in Sleigh** |
| `0x04` | `plzcw` | `rd, rs` | Sleigh requires `mmiop=0` |
| `0x10` | `mfhi1` | `rd` | |
| `0x11` | `mthi1` | `rs` | |
| `0x12` | `mflo1` | `rd` | |
| `0x13` | `mtlo1` | `rs` | |
| `0x18` | `mult1` | `rd, rs, rt`, or `rs, rt` if `rd=0` | lo1/hi1 |
| `0x19` | `multu1` | `rd, rs, rt`, or `rs, rt` if `rd=0` | lo1/hi1 |
| `0x1A` | `div1` | `rs, rt` | lo1/hi1, `sa=0` |
| `0x1B` | `divu1` | `rs, rt` | lo1/hi1, `sa=0` |
| `0x20` | `madd1` | `rs, rt` if `rd=0`; else `rd, rs, rt` | lo1/hi1 |
| `0x21` | `maddu1` | `rs, rt` if `rd=0`; else `rd, rs, rt` | lo1/hi1 |
| `0x30` | `pmfhl.lw` | `rd` | `mmiop=0x00` |
| `0x30` | `pmfhl.uw` | `rd` | `mmiop=0x01` |
| `0x30` | `pmfhl.slw` | `rd` | `mmiop=0x02` |
| `0x30` | `pmfhl.lh` | `rd` | `mmiop=0x03` |
| `0x30` | `pmfhl.sh` | `rd` | `mmiop=0x04` |
| `0x31` | `pmthl.lw` | `rs` | `mmiop=0x00` only |
| `0x34` | `psllh` | `rd, rt, sa` | `sa = mmiop` (5-bit shift) |
| `0x36` | `psrlh` | `rd, rt, sa` | `sa = mmiop` |
| `0x37` | `psrah` | `rd, rt, sa` | `sa = mmiop` |
| `0x3C` | `psllw` | `rd, rt, sa` | `sa = mmiop` |
| `0x3E` | `psrlw` | `rd, rt, sa` | `sa = mmiop` |
| `0x3F` | `psraw` | `rd, rt, sa` | `sa = mmiop` |

`div1`/`divu1` use `fct=0x1A`/`0x1B` under opcode `0x1C`. The Sleigh source
comment beside `div1` (`# 0011 10ss ...`, i.e. opcode `0x0E`) is a **typo** —
the rule itself is inside `with : prime=28` and is Special2 (`0x1C`).

### 5a. Decoding order / `rd=0` ambiguity

For `madd`/`maddu`/`madd1`/`maddu1`/`mult1`/`multu1` the assembler form with an
`rd` is the R5900 3-operand extension; when `rd=0` it matches the 2-operand
form. A decoder should read the `rd` field directly and emit the 3-operand
form when `rd != 0`.

---

## 6. `clear …` pseudo-ops

Ghidra renders certain all-zero-source instructions as `clear <dest>`. None of
these are distinct hardware encodings — they are aliases of ordinary instructions.

### 6a. `clear hi` / `clear lo` (base MIPS SPECIAL, opcode `0x00`)

| form | underlying encoding | word | operands |
| --- | --- | --- | --- |
| `clear hi` | `mthi $0` — `fct=0x11`, `rs=0, rt=0, rd=0, sa=0` | `0x00000011` | none |
| `clear lo` | `mtlo $0` — `fct=0x13`, `rs=0, rt=0, rd=0, sa=0` | `0x00000013` | none |

### 6b. `clear hi1` / `clear lo1` (MMI/Special2, opcode `0x1C`)

| form | underlying encoding | word | operands |
| --- | --- | --- | --- |
| `clear hi1` | `mthi1 $0` — `fct=0x11`, `rs=0, rt=0, rd=0, sa=0` | `0x70000011` | none |
| `clear lo1` | `mtlo1 $0` — `fct=0x13`, `rs=0, rt=0, rd=0, sa=0` | `0x70000013` | none |

### 6c. `clear fsa` (REGIMM, opcode `0x01`)

| form | underlying encoding | word | operands |
| --- | --- | --- | --- |
| `clear fsa` | `mtsah $0, 0` — `rt=0x19`, `rs=0`, `imm16=0` | `0x04190000` | none (`fsa` implicit) |

`mtsab`/`mtsah` are REGIMM `rt=0x18`/`0x19` (Sleigh `copfct=24`/`25`).
`clear fsa` is specifically the `mtsah` form with `rs=0` and `immed=0`.

### 6d. `clear rd` (whole 128-bit GPR ← 0) — 6 aliases

`rd = word[15:11]`. Each is a vector op with **both** sources `$0`. The Sleigh
defines all six; the `0x10/0x28` one is what appears at `entry`.

| underlying op | `mmiop` | `fct` | word with `rd=0` | word with `rd` |
| --- | --- | --- | --- | --- |
| `paddw` | `0x00` | `0x08` | `0x70000008` | `0x70000008 \| (rd<<11)` |
| `paddh` | `0x04` | `0x08` | `0x70000108` | `0x70000108 \| (rd<<11)` |
| `paddb` | `0x08` | `0x08` | `0x70000208` | `0x70000208 \| (rd<<11)` |
| `padduw` | `0x10` | `0x28` | `0x70000428` | `0x70000428 \| (rd<<11)` |
| `paddub` | `0x18` | `0x28` | `0x70000628` | `0x70000628 \| (rd<<11)` |
| `por` | `0x12` | `0x29` | `0x700004A9` | `0x700004A9 \| (rd<<11)` |

Example: `clear at` (`rd=1`, `mmiop=0x10`, `fct=0x28`) = `0x70000C28`.

### 6e. `moveq rd, rs` — 6 related aliases

Same six `(mmiop, fct)` pairs, but `rt=0` and `rs != 0`; the Sleigh renders
these as `moveq rd, rs` (a 128-bit quadword move). A decoder must check
`rt==0 && rs!=0` **before** the generic three-register form.

| `mmiop` | `fct` | underlying op |
| --- | --- | --- |
| `0x00` | `0x08` | `paddw` |
| `0x04` | `0x08` | `paddh` |
| `0x08` | `0x08` | `paddb` |
| `0x10` | `0x28` | `padduw` |
| `0x18` | `0x28` | `paddub` |
| `0x12` | `0x29` | `por` |

Decision table for the six special `(mmiop,fct)` pairs:

| `rs` | `rt` | rendering |
| --- | --- | --- |
| any | any | normal `p<op> rd, rs, rt` |
| `!=0` | `0` | `moveq rd, rs` |
| `0` | `0` | `clear rd` |

(All other `(mmiop,fct)` combinations never special-case on `rs`/`rt`.)

### 6f. COP1 `clear fs` / `clear fcsr`

| form | encoding | word | operands |
| --- | --- | --- | --- |
| `clear fs` | `mtc1 $0, fs` — `copop(rs)=4`, `rt=0`, `bigfunct=0` | `0x44800000 \| (fs<<11)` | FPU reg `fs` (`word[15:11]`) |
| `clear fcsr` | `ctc1 $0, $fcsr` — `copop(rs)=6`, `rt=0`, `fs=31`, `bigfunct=0` | `0x44C0F800` | none |

`clear fs` zeroes the 32-bit FPU register `fs`; `clear fcsr` zeroes `fcsr`
(COP1 control reg 31). Note `fs=0` for `clear fs` would be `clear f0`.

---

## 7. COP2 (VU0 macro mode)

### 7a. COP2 memory ops (primary opcodes, not COP2-format)

| mnemonic | opcode (`word[31:26]`) | operands | Sleigh |
| --- | --- | --- | --- |
| `lqc2` | `0x36` (`prime=54`) | `vft, offset(base)` | `vft = _lqc2(*:16 OFF_BASE)` |
| `sqc2` | `0x3E` (`prime=62`) | `vft, offset(base)` | `*:16 OFF_BASE = _sqc2(vft)` |

(`lq`/`sq` are the analogous GPR quadword ops at opcode `0x1E`/`0x1F`.)

### 7b. COP2 format field (`fmt = word[25:21]`), opcode `0x12`

| `fmt` | mnemonic | operands | notes |
| --- | --- | --- | --- |
| `0x01` | `qmfc2` | `rt, vfs` | `vfs=word[15:11]`; bit0 = interlock `.I` |
| `0x02` | `cfc2` | `rt, vic` | `vic=word[15:11]`; requires `word[10:1]=0`; bit0 = `.I` |
| `0x05` | `qmtc2` | `rt, vfs` | requires `word[10:1]=0`; bit0 = `.I` |
| `0x06` | `ctc2` | `rt, vic` | requires `word[10:1]=0`; bit0 = `.I` |
| `0x08` | `bc2f/bc2t/bc2fl/bc2tl` | `offset` | condition `word[20:16]` ∈ {0,1,2,3} |
| `0x10`–`0x1F` | VU macro upper ops | see 7c | bit25 = 1 |

`vic` selects VI0–VI15 (`0..15`) and VC0–VC15 (`16..31`).

### 7c. VU0 macro upper (arithmetic) ops

Layout for these: `bit25=1`; `dest = word[24:21]` (xyzw write mask; a
`.x/.y/.z/.w/...` suffix); `ft = word[20:16]`; `fs = word[15:11]`;
`fd = word[10:6]`; `opcode = word[10:0]` (11 bits). The Sleigh matches
`vuinst = prime=18 & vuco=1`.

Many regular ops are selected by `word[5:0]` with the broadcast lane in
`word[1:0]` (`bc` = x,y,z,w):

| `word[5:0]` | mnemonic | operands |
| --- | --- | --- |
| `0x00`–`0x03` | `vadd` + `.x/.y/.z/.w` | `vfd, vfs, vft` |
| `0x04`–`0x07` | `vsub` + lane | `vfd, vfs, vft` |
| `0x08`–`0x0B` | `vmadd` + lane | `vfd, vfs, vft` |
| `0x0C`–`0x0F` | `vmsub` + lane | `vfd, vfs, vft` |
| `0x10`–`0x13` | `vmax` + lane | `vfd, vfs, vft` |
| `0x14`–`0x17` | `vmini` + lane | `vfd, vfs, vft` |
| `0x18`–`0x1B` | `vmul` + lane | `vfd, vfs, vft` |
| `0x1C` | `vmulq` | `vfd, vfs, Q` |
| `0x1D` | `vmaxi` | `vfd, vfs, I` |
| `0x1E` | `vmuli` | `vfd, vfs, I` |
| `0x1F` | `vminii` | `vfd, vfs, I` |
| `0x20` | `vaddq` | `vfd, vfs, Q` |
| `0x21` | `vmaddq` | `vfd, vfs, Q` |
| `0x22` | `vaddi` | `vfd, vfs, I` |
| `0x23` | `vmaddi` | `vfd, vfs, I` |
| `0x24` | `vsubq` | `vfd, vfs, Q` |
| `0x25` | `vmsubq` | `vfd, vfs, Q` |
| `0x26` | `vsubi` | `vfd, vfs, I` |
| `0x27` | `vmsubi` | `vfd, vfs, I` |
| `0x28` | `vadd` | `vfd, vfs, vft` |
| `0x29` | `vmadd` | `vfd, vfs, vft` |
| `0x2A` | `vmul` | `vfd, vfs, vft` |
| `0x2B` | `vmax` | `vfd, vfs, vft` |
| `0x2C` | `vsub` | `vfd, vfs, vft` |
| `0x2D` | `vmsub` | `vfd, vfs, vft` |
| `0x2E` | `vopmsub` | `vfd, vfs, vft` |
| `0x2F` | `vmini` | `vfd, vfs, vft` |
| `0x30` | `viadd` | `vid, vis, vit` |
| `0x31` | `visub` | `vid, vis, vit` |
| `0x32` | `viaddi` | `vit, vis, imm5` |
| `0x34` | `viand` | `vid, vis, vit` |
| `0x35` | `vior` | `vid, vis, vit` |
| `0x38` | `vcallms` | `imm15` |
| `0x39` | `vcallmsr` | (uses `vi27`) |

Ops selected by the full 11-bit `word[10:0]` (not fitting `word[5:0]`):

| `word[10:0]` | mnemonic | operands |
| --- | --- | --- |
| `0x13C` | `vitof0` | `vft, vfs` |
| `0x13D` | `vitof4` | `vft, vfs` |
| `0x13E` | `vitof12` | `vft, vfs` |
| `0x13F` | `vitof15` | `vft, vfs` |
| `0x17C` | `vftoi0` | `vft, vfs` |
| `0x17D` | `vftoi4` | `vft, vfs` |
| `0x17E` | `vftoi12` | `vft, vfs` |
| `0x17F` | `vftoi15` | `vft, vfs` |
| `0x1FC` | `vmulaq` | `ACC, vfs, Q` |
| `0x1FD` | `vabs` | `vft, vfs` |
| `0x1FE` | `vmulai` | `ACC, vfs, I` |
| `0x1FF` | `vclip` | `vfs, vft` (`.w`) |
| `0x23C` | `vaddaq` | `ACC, vfs, Q` |
| `0x23D` | `vmaddaq` | `ACC, vfs, Q` |
| `0x23E` | `vaddai` | `ACC, vfs, I` |
| `0x23F` | `vmaddai` | `ACC, vfs, I` |
| `0x27C` | `vsubaq` | `ACC, vfs, Q` |
| `0x27D` | `vmsubaq` | `ACC, vfs, Q` |
| `0x27E` | `vsubai` | `ACC, vfs, I` |
| `0x27F` | `vmsubai` | `ACC, vfs, I` |
| `0x2BC` | `vadda` | `ACC, vfs, vft` |
| `0x2BD` | `vmadda` | `ACC, vfs, vft` |
| `0x2BE` | `vmula` | `ACC, vfs, vft` |
| `0x2FC` | `vsuba` | `ACC, vfs, vft` |
| `0x2FD` | `vmsuba` | `ACC, vfs, vft` |
| `0x2FE` | `vopmula` | `ACC, vfs, vft` |
| `0x2FF` | `vnop` | — |
| `0x33C` | `vmove` | `vft, vfs` |
| `0x33D` | `vmr32` | `vft, vfs` |
| `0x37C` | `vlqi` | `vft, (vis++)` |
| `0x37D` | `vsqi` | `vfs, (vit++)` |
| `0x37E` | `vlqd` | `vft, (--vis)` |
| `0x37F` | `vsqd` | `vfs, (--vit)` |
| `0x3BC` | `vdiv` | `Q, vfs.fsf, vft.ftf` |
| `0x3BD` | `vsqrt` | `Q, vft.ftf` |
| `0x3BE` | `vrsqrt` | `Q, vfs, vft` |
| `0x3BF` | `vwaitq` | — |
| `0x3FC` | `vmtir` | `vit, vfs.fsf` |
| `0x3FD` | `vmfir` | `vft, vis` |
| `0x3FE` | `vilwr` | `vit, (vis)` |
| `0x3FF` | `viswr` | `vit, (vis)` |
| `0x43C` | `vrnext` | `vft, R` |
| `0x43D` | `vrget` | `vft, R` |
| `0x43E` | `vrinit` | `R, vfs` |
| `0x43F` | `vrxor` | `R, vfs` |

Accumulator broadcast variants selected by `word[10:2]` with lane in `word[1:0]`:

| `word[10:2]` | mnemonic | operands |
| --- | --- | --- |
| `0b000001111` | `vadda` + lane | `ACC, vfs, vft` |
| `0b000011111` | `vsuba` + lane | `ACC, vfs, vft` |
| `0b000101111` | `vmadda` + lane | `ACC, vfs, vft` |
| `0b000111111` | `vmsuba` + lane | `ACC, vfs, vft` |
| `0b001101111` | `vmula` + lane | `ACC, vfs, vft` |

(`vcallms` additionally requires `dest=0`, `vcallmsr` requires
`ft=0, fs=27, fd=0`.)

---

## 8. Ambiguities / Sleigh vs manual

1. **Group selectors.** The task brief said MMI0/1/2/3 are `fct ∈
   {0x08,0x09,0x0A,0x0B}`. That is **not** what any of the three sources say.
   Sleigh, `docs/ps2-bible.md`, and ps2tek all agree on
   `MMI0=0x08, MMI1=0x28, MMI2=0x09, MMI3=0x29`; `fct=0x0A`/`0x0B` are
   unassigned. Implement the `{0x08,0x09,0x28,0x29}` set.
2. **`msub`/`msubu` missing.** Manual/ps2tek list MMI `fct=0x02` (`msub`) and
   `fct=0x03` (`msubu`), but the Sleigh defines neither (grep confirms no
   `:msub`/`:msubu` on opcode `0x1C`). A decoder should add them; Ghidra will
   leave these words un-decoded.
3. **`plzcw` `mmiop` constraint.** Sleigh requires `mmiop=0`; ps2tek keys only
   on `fct=0x04` (ignores `sa`). Treat the `sa` bits as don't-care for real
   hardware, but note the Sleigh is stricter.
4. **`psllvw` operand order.** Sleigh prints `psllvw rd, rs, rt` but
   `psrlvw`/`psravw` as `rd, rt, rs`. The manual likely uses one uniform order;
   this asymmetry looks like a Sleigh typo. Flag before trusting operand order
   for these three.
5. **`pmthl` variants.** Sleigh defines only `pmthl.lw` (`mmiop=0`); other
   `mmiop` values under `fct=0x31` are unassigned. `pmfhl` has all five
   variants (`lw/uw/slw/lh/sh`, `mmiop=0..4`).
6. **`clear fsa`.** It is the `mtsah $0, 0` encoding (`0x04190000`), not
   `mtsab`. Verify against the manual if a `clear fsa` from `mtsab` is expected.
7. **`div1` comment typo.** `eecore.sinc` annotates `div1` with
   `0011 10ss …` (opcode `0x0E`); the actual rule is under `prime=28`
   (`0x1C`, Special2), i.e. `fct=0x1A`. Use `0x1C`.
8. **Mnemonic spellings.** Sleigh `phmsdh` = manual `PHMSBH`; Sleigh `padsbh` =
   manual `PADSBH`. Same encodings (`0x15/0x09` and `0x04/0x28`).
9. **PSLLH-family `sa`.** `psllh/psrlh/psrah/psllw/psrlw/psraw` use the `mmiop`
   field as the shift amount; `sa=0` is a legal encoding (matches all values,
   no nonzero constraint in Sleigh).
10. **Current decoder mismatch (informational).**
    `crates/ssx3-recomp/src/r5900.rs::mmi_mnemonic` currently holds a
    provisional table. Several `fct` values are off by 4 versus every source
    above — e.g. it maps `mult1=0x14, multu1=0x15, div1=0x16, divu1=0x17,
    madd1=0x18, maddu1=0x19`, but the correct values are
    `0x18,0x19,0x1A,0x1B,0x20,0x21`; and it maps `fct=0x02 → "mul"` where the
    manual says `msub`. `decode_cop2` also keys `qmfc2/qmtc2/cfc2` on
    `fmt=0/1/2` instead of the correct `1/5/2` (and invents `mtc2` at `4`).
    This table supersedes it.
11. **`clear`/`moveq` ordering.** The special cases must be resolved before the
    generic 3-register form for the six `(mmiop,fct)` pairs in §6d/§6e,
    otherwise `clear`/`moveq` decode as ordinary vector ops.

---

## 9. Counts

* MMI0: **25** · MMI1: **18** · MMI2: **22** · MMI3: **13** → **78** group members.
* Single-opcode MMI/Special2 forms: **21** defined by the Sleigh
  (`madd, maddu, plzcw, mfhi1, mthi1, mflo1, mtlo1, mult1, multu1, div1, divu1,
  madd1, maddu1, pmfhl, pmthl, psllh, psrlh, psrah, psllw, psrlw, psraw`), or
  **23** if the manual-only `msub`/`msubu` are included.
* **99** Sleigh-defined MMI/Special2 encodings (**101** per the manual).
  Counting the five `pmfhl` variants separately yields **103**/**105**.
* Aliases: **6** `moveq` forms, **6** `clear rd` forms, plus `clear hi`,
  `clear lo`, `clear hi1`, `clear lo1`, `clear fsa`, and COP1 `clear fs`,
  `clear fcsr`.
