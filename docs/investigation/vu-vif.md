# VU0/VU1 and VIF0/VIF1

Crate `crates/ssx3-ps2`, modules `vu/` and `vif/`, generated code in
`generated/vu/`. Logic is partly ported from `snuri00/ps2-web-recomp` (MIT;
`runtime/src/vu_micro.cpp`, `runtime/src/vif.cpp`,
`runtime/include/ps2/fpu.hpp`). The rest follows PCSX2 semantics.

## Execution model: microcode is statically recompiled

Game microcode is not interpreted at runtime. Here is how it runs:

1. **`vu_recomp`** (`cargo run -p ssx3-ps2 --bin vu_recomp [ELF] [OUT]`) reads
   `.vutext` from `orig/SLUS_207.72`. It finds every VIF `MPG` (CMD 0x4a, with
   64-bit-aligned data) in the DMA/VIF chains stored there, then merges
   consecutive MPGs with contiguous load addresses into one *segment*.
   `vu::recomp::translate` turns each segment into `generated/vu/vu_<hash>.rs`
   and also writes `generated/vu/mod.rs` (`PROGRAMS`, the registry).
2. Each translated segment has one function,
   `run(vu, pc, kick) -> VuExit`, which is a `loop { match pc { … } }` over
   basic blocks. Blocks start at these leaders:
   - the load address
   - branch targets
   - the pair after each branch's delay slot
   - BAL/JALR return addresses
   - the pair after each E-bit delay slot

   Delay slots are emitted inline. Every *non-leader* pair also gets a
   one-pair arm, so a `JR` to any pc still runs translated code until it
   reaches the next leader. This matters: entry 0 of several programs does
   `jr vi15` with an EE-supplied address. A pc outside the segment returns
   `VuExit::Jump(pc)`, and `Vu::run` dispatches it again.
3. **Registry key.** `Vu::write_code` (which MPG calls) tracks upload
   segments. A write that starts where the most recently written segment
   ends extends that segment. Any other write starts a new segment and
   trims the parts of older segments it overlaps. At `MSCAL`, `Vu::run`
   hashes the segment that contains the start pc. The hash is FNV-1a over the
   load address (LE) followed by the bytes, and is cached until the next
   write. The hash selects the `ProgramEntry`.
4. **No translation:** `Vu::run` logs `[vuN] no translated microcode…`,
   increments `vu.errors`, and stops the VU. Nothing is interpreted silently.
   `vu.fallback: Option<ProgramFn>` exists so you can install an executor
   explicitly. Tests install the oracle this way.
5. **Oracle:** `vu::reference` (always built; the game never installs it) is a pair interpreter. It
   uses the same decoder and the same semantic methods. The test
   `oracle_translated_matches_reference_on_vutext` runs the compiled
   translations against it on all 7 real programs, from every entry point
   (pc 0 and each post-E pair), with two seeded random states. It compares
   all registers, flags, cycle count, data memory and XGKICK packets.

Generated output: 7 segments, 7305 pairs, about 3.6 MB of Rust. A crate
build takes about 4 s in debug and 10 s in release.

| `.vutext` source | load | bytes | pairs | E bits | hash |
| --- | --- | --- | --- | --- | --- |
| 0x0042e5b4 | 0x0000 | 0x24a0 | 1172 | 15 | f4babac52752998a |
| 0x00430aa4 | 0x0000 | 0x3ea8 | 2005 | 7 | 9790369fcc84762b |
| 0x004349b4 | 0x0000 | 0x1200 | 576 | 9 | 9fa478425b3700e5 |
| 0x00435bf4 | 0x0000 | 0x3e08 | 1985 | 13 | d159716bb1281fbe |
| 0x00439a64 | 0x0000 | 0x1920 | 804 | 9 | f55ee63416fe5dcf |
| 0x0043b414 | 0x0000 | 0x0898 | 275 | 2 | e72540c9e7c434ee |
| 0x0043bd34 | 0x0000 | 0x0f40 | 488 | 17 | 27649fa55223cbbf (likely VU0) |

Decode coverage: **all 7305 pairs decode, with 0 unknown opcodes.** No
program has a branch in a delay slot, an E bit in a branch delay slot, or a
branch in an E pair. The translator would flag these as `unsupported`.

## VU semantic library (`vu/mod.rs`)

The per-pair model (what the reference interpreter and macro mode run):

```rust
vu.begin_pair(&[(fs, mask), ...]); // stall model + retire Q/P/flag pipelines
vu.madd(0b1110, 1, 2, Src::Bc(3, W)); // upper: computed from old state, staged
vu.loi(imm);                       // I-bit pairs, after the upper: it reads the OLD I
vu.lq(0b1111, 4, 1, 2);            // lower: runs against old state
vu.end_pair();                     // commit upper; it wins over a lower write to the same VF
```

Translated code (since 2026-10-14 evening) computes the same state but
resolves the bookkeeping at translate time: it checks only VF fields that
can still be in flight, retires Q/P/flags only before ops that observe them,
writes ready times and flag snapshots only when something can see them,
defers MAC/status commits within a block, and does the FMAC arithmetic four
lanes at a time (NEON on AArch64). See [`vu-performance.md`](vu-performance.md)
for the analyses, the equivalence argument and the checks.

There is one public method per operation:

- **Upper:** `add/sub/mul/madd/msub/max/mini(dest, fd, fs, Src)` with
  `Src::{Vec(ft), Bc(ft, X..W), Q, I}`. The ACC forms are `adda/suba/mula/madda/msuba`.
  Also `opmula`, `opmsub`, `itof/ftoi(frac ∈ {0,4,12,15})`, `abs`, `clipw`.
- **Lower:** `lq sq ilw isw ilwr iswr lqi lqd sqi sqd iaddiu isubiu iadd isub
  iaddi iand ior fceq fcset fcand fcor fcget fseq fsset fsand fsor fmeq fmand
  fmor vmove mr32 mtir mfir div sqrt rsqrt waitq rnext rget rinit rxor mfp
  xtop xitop xgkick efu_vec(Efu::…) efu_scalar(Efu::…) waitp`.
- **Branches:** `ibeq/ibne/ibltz/ibgtz/iblez/ibgez(..) -> bool`,
  `jr_target(is)` and `link(it, ret_pc)`. The translator turns these into
  control flow.

Macro mode (`macro_op`) decodes the COP2 word with the same decoder. It runs
the same methods and then flushes all pipelines, so results are visible
immediately. Supported: all upper ops, VIADD, VISUB, VIADDI, VIAND, VIOR,
VMOVE, VMR32, VLQI, VSQI, VLQD, VSQD, VDIV, VSQRT, VRSQRT, VWAITQ, VMTIR,
VMFIR, VILWR, VISWR, VRNEXT, VRGET, VRINIT, VRXOR. `macro_op` ignores
VCALLMS and VCALLMSR.

### Float rules (`vu/float.rs`)

- Denormal inputs are read as ±0. Inputs with exponent 255 are read as ±max
  (`0x7f7fffff`).
- add/sub/mul use integer emulation with truncation: one guard bit, no sticky
  bit. MADD/MSUB truncate the product first.
- Overflow clamps to ±max and sets O. Underflow flushes to ±0 and sets U and Z.
- **MAC layout:** Z bits 3..0, S 7..4, U 11..8, O 15..12, with x as the high
  bit of each nibble. Lanes outside the dest mask read as 0.
- **Status:** Z S U O come from the MAC, and their sticky copies are bits 6..9.
  The DIV unit sets I (bit 4) and D (bit 5), with sticky copies in bits 10 and 11.
- **FTOI:** truncates and saturates to i32. **ITOF:** truncates.
- **MAX/MINI:** compare the raw bit patterns as sign-magnitude integers and
  return the raw operand: no denormal flush, no exponent-255 clamp (PCSX2
  microVU `MIN_MAX_PS`). SSX 3 copies integer vertex colours with
  `MAX.xyzw vfX, vfY, vfY`.
- **CLIP:** compares |w| against x, y and z, producing 6 bits that shift into
  the 24-bit clip flags.

### Timing model (approximation of PCSX2)

- Each pair takes 1 cycle.
- **Stalls:** `begin_pair` stalls until every VF field the pair reads has been
  written back. An FMAC result is ready 4 cycles after issue. Lower VF writes
  (LQ, MOVE, MFIR, …) also use 4 cycles. ACC and VI are not tracked.
- **Q:** DIV and SQRT take 7 cycles, RSQRT takes 13. Until then, an upper op
  reading Q sees the old value. WAITQ, or a new DIV-unit op, stalls until the
  result lands. The stall happens *before* the pair (`vu.stall_q()`, emitted
  between `begin_pair` and the upper op), so an upper op reading Q in the
  WAITQ/DIV pair sees the new Q, as in PCSX2. The course clipper depends on
  it (`race-geometry.md`).
- **P:** the EFU ops use the manual's latencies, for example ESADD 11,
  ERSQRT 18, EATAN 54 and EEXP 44. WAITP stalls until P is ready.
- **Flags:** MAC, status and clip flags become visible to FMxxx/FSxxx/FCxxx
  4 cycles after the op that sets them, as in PCSX2 (microVU `mVUsetFlags`,
  interpreter FMAC pipe). Clip used to be 3; that made VU0's bounding-box
  test read half the box corners and cull visible meshes
  (`race-geometry.md`). The latency is a constant: the translator's flag
  analysis (3-pair window) needs it to be at most 4. This uses a snapshot
  pipeline.
  FSSET and FCSET rewrite both the in-flight snapshots and the current flags.
- **VI-backup quirk (PCSX2 `VI_BACKUP`):** suppose the pair just before a
  conditional branch writes a VI the branch reads, with IADD/IADDI/IADDIU/
  ISUB/ISUBIU/IAND/IOR, MTIR or LQI/LQD/SQI/SQD. The branch then sees the
  old value, unless the branch pair stalls (microVU). JR/JALR read the
  current VI. On by default (`SSX3_VU_VI_BACKUP=0` disables); no SSX 3
  program contains the pattern.
- At the end of a program (after the E delay slot), every pipeline drains.
  `tpc` is set to the pair after the delay slot.
- XGKICK runs immediately. It reads GIF tags from `vi[is]*16` and keeps going
  until it reaches EOP. The packet size is PACKED nloop·nreg, REGLIST
  ⌈nloop·nreg/2⌉, or IMAGE nloop qwords. Reads wrap at 16 KiB. The packet goes
  to `kick` (VIF1 sends it to `gs.gif_write(1, ..)`).

### Control registers

| reg | meaning |
| --- | --- |
| 0–15 | VI. Reads are 16-bit; vi0 is hardwired. |
| 16 | status. A write only sets the sticky bits 6..11. |
| 17 | MAC. Read-only. |
| 18 | clip |
| 20 | R. Stored as `0x3f800000 \| lfsr`; reads return 23 bits. |
| 21 | I |
| 22 | Q |
| 23 | P. Read-only. |
| 26 | TPC. Reads return `tpc/8` (pair units). |
| 27 | CMSAR0 |
| 28 | FBRST. This instance acts only on its own RS and FB bits. The machine forwards the VU1 half. |
| 29 | VPU-STAT. Bit 0 is VU0 running and bit 8 is VU1. The machine sets `peer_running` for the other VU. |
| 31 | CMSAR1. Records `cmsar1_request = Some(val*8)`. The machine then calls `vu1.start()`. |

## VIF (`vif/mod.rs`)

- Data arrives as 32-bit words in a FIFO. A command waits until all of its
  data has arrived, so commands and data can straddle `write` calls.
- **Registers:** STAT (DBF, FQC, VPS, MRK, VIS, INT, ER1), FBRST (RST, STP,
  STC), ERR, MARK, CYCLE, MODE, NUM, MASK, CODE, ITOPS, BASE, OFST, TOPS,
  ITOP, TOP, R0–R3 and C0–C3, at the documented offsets.
- **Commands:** NOP, STCYCL, OFFSET, BASE, ITOP, STMOD, MSKPATH3 (VIF1: masks
  PATH3 in the GS, `race-textures.md`),
  MARK, FLUSHE, FLUSH, FLUSHA (no-ops), MSCAL, MSCALF, MSCNT, STMASK, STROW,
  STCOL, MPG (calls `Vu::write_code`), DIRECT/DIRECTHL (`gs.gif_write(2, ..)`,
  VIF1 only) and UNPACK.
- **UNPACK:** supports all vn/vl formats, including V4-5. USN controls sign
  versus zero extension. FLG adds TOPS (VIF1 only). The write mask (`m` bit
  plus MASK) selects data, ROW, COL[cycle], or write-protect. STMOD modes
  offset and difference are supported; difference also updates ROW. For
  CYCLE, CL≥WL skips memory and CL<WL fills, and filling writes use ROW.
- **Double buffering:** on MSCAL/MSCALF/MSCNT, TOP = TOPS and ITOP = ITOPS,
  DBF toggles, and TOPS = BASE (+ OFST when DBF is set). OFFSET clears DBF and
  sets TOPS = BASE.
- **MSCAL:** runs the VU to completion synchronously, with a budget of
  2,000,000 pairs. If the program does not finish, the VIF logs it and stops
  the VU.
- **i bit:** if a command has the i bit and ERR.MII is clear, then after it
  runs the VIF sets STAT.INT|VIS, stalls, and `take_irq()` returns true once.
  `write_reg(FBRST, STC)` clears the stall. Buffered data resumes on the next
  `write`, or when you call `vif.resume(vu, gs)`.

## Gaps and approximations

- **E bit in a branch delay slot, or a branch in a delay slot:** not
  translated faithfully. `vu.unknown_op` logs it. It does not occur in SSX 3.
- **Not modelled:**
  - the M, D and T upper bits (debug breaks)
  - VU0's view of VU1 registers at 0x4000
  - XGKICK/PATH3 arbitration (MSKPATH3 masking is modelled; packet-level
    interleaving is not)
  - integer-register latencies
  - stalls on ACC
- **Timing differences from PCSX2:** the stall model is per VF field with
  4-cycle latency. There are no branch penalties, and the P (EFU) wait
  happens inside the pair instead of before it (only lower ops read P, so
  this changes no values). Code that depends on exact
  cycle counts can still see different Q, P or flag timing than PCSX2.
- **Budgets:** translated code charges the pair budget once per block, at the
  block's start. The interpreter charges per pair.
- **Registry key:** the hash covers the whole contiguous upload. If the game
  uploads a program differently from the chain in `.vutext`, there is no
  translation and the error log shows it. Examples: a partial re-upload, or a
  different combination of chunks.
- **VIF:** MARK does not stall. STAT.VEW is never set, because VU runs are
  synchronous. Invalid commands set ER1 and are skipped, and the VIF does
  not stall on them. VIF0 DIRECT is rejected.
- **Differential testing** against ps2-web-recomp's interpreter on captured
  game invocations: [`vu-differential.md`](vu-differential.md).
- **Tests skip without the ELF:** the decode-coverage and oracle tests are
  skipped when `orig/SLUS_207.72` is absent.
