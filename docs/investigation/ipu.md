# IPU (MPEG-2 decoder)

SSX 3's boot and attract movies (`DATA/MOVIES/*.MPC`) are played by Sony
libmpeg 2.7 (`PsIIlibmpeg 2700` / `PsIIlibipu 2700` in `SLUS_207.72`). They
need the EE's IPU. Code: `crates/ssx3-ps2/src/ipu/`.

## The MPC files

`.MPC` is an EA "Gimex" chunk container. Each chunk is `tag[4] size_le32 payload`,
and `size` includes the 8-byte header:

| tag    | contents |
|--------|----------|
| `MPCh` | MPEG-2 video elementary stream. The first chunk holds the sequence header (`00 00 01 B3`) and each later chunk holds one picture (it starts at `00 00 01 00`) |
| `SCHl`/`SCCl`/`SCDl`/`SCEl` | EA audio stream (header, count, data, end) |

`EABIG.MPC` is 512x448 at 29.97 fps, with the default quantiser matrices, frame
pictures and slices of one macroblock row each (start codes `01`..`1C`). It
has 75 pictures: 6 I, 20 P and 49 B.

## Module layout

| file | role |
|------|------|
| `ipu/mod.rs` | registers, the command engine, bitstream reader, VLC/DCT decode, dequantisation |
| `ipu/tables.rs` | VLC tables (BSD-2, from Play! via ps2-web-recomp; the licence header is kept in the file) |
| `ipu/idct.rs` | separable double-precision IDCT, saturating to [-256, 255] |
| `ipu/csc.rs` | YCbCr→RGB32 (PCSX2 fixed-point coefficients), SETTH alpha thresholds, RGB16 with dither, INDX4 vector quantisation |
| `ipu/dma.rs` | `HwPort`: ch4 pulled one qword at a time (with its own chain-tag walk), ch3 pushed into |
| `ipu/tests.rs` | unit tests and the EABIG end-to-end test |

## Interface

- `Ipu::read32(off)`: `off` is relative to `0x1000_2000`.
  - `0x00` CMD: DATA is the VDEC/FDEC result, or the next 32 bits otherwise.
  - `0x04` holds the BUSY bit.
  - `0x10` CTRL: IFC, OFC, CBP, ECD, SCD, IDP/AS/IVF/QST/MP1/PCT, BUSY.
  - `0x20` BP: BP, IFC, FP.
  - `0x30` TOP, with its BUSY bit at `0x34`.
- `Ipu::write32(off, v, dma)`: CMD starts a command. CTRL writes bits 16-29, and RST (bit 30) resets the IPU.
- `Ipu::write_fifo(&[u128], dma)` / `Ipu::read_fifo(n, dma)`: the CPU FIFOs at `0x1000_7010` / `0x1000_7000`.
- `Ipu::step(dma)`: resumes a starved command, delivers output to ch3 and tops the input FIFO back up to 8 qwords from ch4.
- `Ipu::take_irq()`: true when a command has completed. `hw.rs` then raises INTC bit 8.
- `trait IpuDma { pull, push }`: `ipu::dma::HwPort` implements it over `Hw::ch[3..=4]` and RAM/SPR, and `NoDma` is the version with no DMA attached.

The changes to `hw.rs` are small and only touch the IPU paths:
- the `ipu: Ipu` field;
- register read/write arms;
- the FIFO arms in `read128`/`write128`;
- ch3/ch4 in `dma_start`, which no longer complete instantly;
- the `with_ipu` helper, which posts D_STAT and INTC.

## Semantics

- **Pull model.** The input FIFO holds at most 8 unconsumed qwords between
  commands, and toIPU is drained only as fast as the IPU consumes data. This
  matters for libmpeg. Its `sceIpuStopDMA`/`sceIpuRestartDMA` rewind the
  bitstream with `D4_MADR -= 16*(IPU_BP.IFC+FP)` so it can interleave CSC
  (which also arrives over toIPU) with BDEC. ch4 stays STR until its chain
  ends (`refe`/`end`, or IRQ+TIE) and its last qword has been pulled. ch3
  completes when its QWC reaches 0, or immediately if it was started with
  QWC=0.
- **Starvation.** Every macroblock (and every SETIQ/SETVQ/FDEC/VDEC/CSC/PACK
  unit) runs atomically. If input runs out, the bit position, DC predictors
  and output are rolled back, BUSY stays set, and the work is retried on the
  next FIFO write or DMA start.
- **BUSY** stays set until all of the output has been delivered (to ch3 or to
  FIFO reads). The command then completes and raises INTC IPU. This is done
  for every command, including BCLR and SETTH.
- **BDEC** follows the PCSX2/ps2-web-recomp layout: FB, QSC, DT (field DCT),
  DCR (reset DC predictors), MBI. Output is RAW16 (Y 16x16, Cb 8x8, Cr 8x8),
  48 qwords. CTRL.CBP is set. SCD is set if 23 zero bits follow.
- **IDEC** starts from `macroblock_type`, so the driver consumes the first
  address increment. It decodes intra macroblocks until a start code, CSCs
  each one (RGB32 or RGB16, with DTE dither, SGN XORs 0x808080, and SETTH
  thresholds apply), leaves the stream at `00 00 01`, and sets SCD.
- **VDEC** returns `length<<16 | value`. On an invalid code it returns 0 and
  sets SCD if a start code follows, or ECD otherwise. **FDEC** skips FB bits
  and peeks 32 bits without consuming them. It sets SCD for `00 00 01 xx`.
- **SETIQ** takes the matrix in zigzag order and stores it by raster
  position, which is correct for alternate scan too. **SETVQ** takes 16
  little-endian halfwords.
- **Dequantisation** handles the MPEG-2 linear and non-linear `quantiser_scale`
  and mismatch control. In MPEG-1 mode (MP1) it uses MPEG-1 escape levels and
  oddification instead.

## Verification

`cargo test -p ssx3-ps2 ipu` runs these tests:
- an IDCT check against the 2-D definition;
- hand-built bitstreams covering DC/AC VLC, dequantisation and the DC predictor;
- VDEC/FDEC;
- resuming after starvation, and IPU_BP;
- SETIQ;
- CSC known colours and thresholds;
- RGB16, dither and PACK/INDX4;
- a full DMA round trip through `Hw` (ch4 source chain → BDEC → ch3, D_STAT and INTC);
- `eabig_i_frames_decode`.

`eabig_i_frames_decode` reads `EABIG.MPC` from `/Volumes/SSX3`, `$SSX3_ISO`
or `SSX 3 (USA).iso` in a parent directory, and skips if none is found. A
libmpeg-style driver parses headers through FDEC/VDEC only, decodes each
I-picture with BDEC + CSC and again with IDEC, and asserts that both
produce identical RGB.
- `SSX3_IPU_FRAMES=n` sets how many I-pictures are decoded.
- `SSX3_IPU_PNG_DIR=dir` writes PNGs.

Chunk 58 shows the full "EA SPORTS BIG" logo on orange with "EASPORTSBIG.COM".
Chunk 28 shows the logo mid-animation, and chunk 0 is a flat orange frame. All
6 I-pictures decode with no VLC errors in about 0.3 s each in a debug build.

## Gaps

- P/B pictures are untested end to end. The IPU only decodes their residuals
  (non-intra BDEC, VDEC motion codes), and the motion compensation is libmpeg's
  work on the EE. Non-intra paths are covered by code review only.
- The bit-exactness of the IDCT against real hardware is not modelled (PCSX2 uses
  an integer IDCT). The output range and intra clamping of BDEC RAW16 to [-256, 255] follow
  ps2-web-recomp.
- When to raise the IPU interrupt, and whether to set SCD/ECD after VDEC/BDEC,
  are best guesses (PCSX2 behaviour as far as known). We have not checked
  against libmpeg's handlers in the game.
- If a macroblock starves, more than 8 qwords can be buffered, and IPU_BP.IFC
  is then clamped to 15. A rewind at that exact moment would be off.
- With the CPU reading `0x1000_7000` there is no DMA connection, so a command
  that starves during such a read only resumes on the next register write,
  FIFO write or DMA start.
- D_CTRL stall control (STS/STD), MFIFO and fromIPU chain modes are ignored. Reads through `IPU_TOP` do not pull DMA data.
- The game-side movie path (`RCMP::mpegbuff`, audio, GS upload) has not been
  exercised in the recompiled game yet.
