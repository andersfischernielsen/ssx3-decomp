use super::*;

// ------------------------------------------------------------- helpers

/// MSB-first bit writer for hand-built bitstreams.
#[derive(Default)]
struct BitWriter {
    bytes: Vec<u8>,
    nbits: usize,
}

impl BitWriter {
    fn put(&mut self, s: &str) -> &mut Self {
        for c in s.chars().filter(|c| *c == '0' || *c == '1') {
            if self.nbits.is_multiple_of(8) {
                self.bytes.push(0);
            }
            if c == '1' {
                *self.bytes.last_mut().unwrap() |= 0x80 >> (self.nbits % 8);
            }
            self.nbits += 1;
        }
        self
    }

    /// Pad to whole quadwords (with zero bits).
    fn qwords(&self) -> Vec<Qword> {
        let mut b = self.bytes.clone();
        b.resize(b.len().div_ceil(16).max(1) * 16, 0);
        b.chunks(16)
            .map(|c| u128::from_le_bytes(c.try_into().unwrap()))
            .collect()
    }
}

fn bytes_to_qwords(b: &[u8]) -> Vec<Qword> {
    let mut b = b.to_vec();
    b.resize(b.len().div_ceil(16) * 16, 0);
    b.chunks(16)
        .map(|c| u128::from_le_bytes(c.try_into().unwrap()))
        .collect()
}

fn raw16(out: &[Qword]) -> Vec<i16> {
    out.iter()
        .flat_map(|q| q.to_le_bytes())
        .collect::<Vec<u8>>()
        .chunks(2)
        .map(|c| i16::from_le_bytes([c[0], c[1]]))
        .collect()
}

fn words(out: &[Qword]) -> Vec<u32> {
    out.iter()
        .flat_map(|q| (0..4).map(move |i| (q >> (32 * i)) as u32))
        .collect()
}

/// Straightforward 2-D IDCT straight from the definition, for comparison.
fn idct_reference(f: &[i32; 64]) -> [i32; 64] {
    let c = |u: usize| {
        if u == 0 {
            std::f64::consts::FRAC_1_SQRT_2
        } else {
            1.0
        }
    };
    let mut out = [0; 64];
    for y in 0..8 {
        for x in 0..8 {
            let mut s = 0.0;
            for v in 0..8 {
                for u in 0..8 {
                    s += c(u)
                        * c(v)
                        * f[v * 8 + u] as f64
                        * (((2 * x + 1) * u) as f64 * std::f64::consts::PI / 16.0).cos()
                        * (((2 * y + 1) * v) as f64 * std::f64::consts::PI / 16.0).cos();
                }
            }
            out[y * 8 + x] = ((s / 4.0 + 0.5).floor() as i32).clamp(-256, 255);
        }
    }
    out
}

fn cmd_word(code: u32, rest: u32) -> u32 {
    code << 28 | rest
}

// --------------------------------------------------------------- IDCT

#[test]
fn idct_dc_only() {
    let mut f = [0i32; 64];
    f[0] = 1024;
    let mut out = [0i16; 64];
    idct::idct(&f, &mut out);
    assert!(out.iter().all(|&v| v == 128));
    f[0] = -8 * 300;
    idct::idct(&f, &mut out);
    assert!(out.iter().all(|&v| v == -256), "saturates");
}

#[test]
fn idct_matches_definition() {
    // Deterministic pseudo-random sparse blocks.
    let mut seed = 0x1234_5678u32;
    let mut rnd = || {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        seed
    };
    for _ in 0..200 {
        let mut f = [0i32; 64];
        for _ in 0..(rnd() % 12) {
            f[(rnd() % 64) as usize] = (rnd() % 600) as i32 - 300;
        }
        f[0] = (rnd() % 2048) as i32 - 1024;
        let mut out = [0i16; 64];
        idct::idct(&f, &mut out);
        let want = idct_reference(&f);
        for i in 0..64 {
            assert!(
                (out[i] as i32 - want[i]).abs() <= 1,
                "{i}: {} vs {}",
                out[i],
                want[i]
            );
        }
    }
}

// ------------------------------------------------------- VLC / BDEC

/// An intra macroblock whose six blocks carry only a zero DC difference.
fn flat_intra_mb(w: &mut BitWriter) {
    for _ in 0..4 {
        w.put("100").put("10"); // dct_dc_size_luma = 0, EOB
    }
    for _ in 0..2 {
        w.put("00").put("10"); // dct_dc_size_chroma = 0, EOB
    }
}

#[test]
fn bdec_flat_intra_macroblock() {
    let mut ipu = Ipu::new();
    let mut w = BitWriter::default();
    flat_intra_mb(&mut w);
    ipu.write_fifo(&w.qwords(), &mut NoDma);
    // BDEC: MBI=1, DCR=1, QSC=1.
    ipu.write32(
        reg::CMD,
        cmd_word(cmd::BDEC, 1 << 27 | 1 << 26 | 1 << 16),
        &mut NoDma,
    );
    assert!(ipu.busy(), "busy until the output is read");
    assert_eq!(ipu.out_pending(), 48);
    let out = ipu.read_fifo(48, &mut NoDma);
    assert!(!ipu.busy());
    assert!(ipu.take_irq());
    let raw = raw16(&out);
    assert_eq!(raw.len(), 384);
    assert!(raw.iter().all(|&v| v == 128), "{raw:?}");
    let c = ipu.read32(reg::CTRL);
    assert_eq!((c >> 8) & 0x3F, 0x3F, "CBP");
    assert_ne!(c & ctrl::SCD, 0, "zero padding reads as a start code");
}

#[test]
fn bdec_ac_coefficient_dequant_and_idct() {
    let mut ipu = Ipu::new();
    let mut w = BitWriter::default();
    // Y0: DC diff +3 (size 2 "01", bits "11"), then run 0 level -1 ("11" + s=1),
    // then run 1 level 2 at scan 3 (table B.14: "0110 0s"... use "0101 s" = run 2 level 1),
    // EOB.
    w.put("01").put("11"); // dc size 2, diff +3
    w.put("11").put("1"); // run 0, level -1 -> scan index 1
    w.put("0101").put("0"); // run 2, level +1 -> scan index 4
    w.put("10"); // EOB
    for _ in 0..3 {
        w.put("100").put("10");
    }
    for _ in 0..2 {
        w.put("00").put("10");
    }
    ipu.write_fifo(&w.qwords(), &mut NoDma);
    let qsc = 5;
    ipu.write32(
        reg::CMD,
        cmd_word(cmd::BDEC, 1 << 27 | 1 << 26 | qsc << 16),
        &mut NoDma,
    );
    let raw = raw16(&ipu.read_fifo(48, &mut NoDma));
    // Expected: DC = (128 + 3) * 8; scan 1 -> raster 1, scan 4 -> raster 9.
    let mut f = [0i32; 64];
    f[0] = 131 * 8;
    let deq = |level: i32, r: usize| (level * DEFAULT_INTRA[r] as i32 * (2 * qsc as i32) * 2) / 32;
    f[1] = deq(-1, 1);
    f[9] = deq(1, 9);
    let sum: i32 = f.iter().sum();
    if sum & 1 == 0 {
        f[63] ^= 1;
    }
    let want = idct_reference(&f);
    for y in 0..8 {
        for x in 0..8 {
            let got = raw[y * 16 + x] as i32;
            assert!(
                (got - want[y * 8 + x]).abs() <= 1,
                "({x},{y}) {got} vs {}",
                want[y * 8 + x]
            );
        }
    }
    // Block 1 (to the right) is flat 128: its DC predictor continued from Y0.
    assert_eq!(raw[8], 131);
}

#[test]
fn vdec_and_fdec() {
    let mut ipu = Ipu::new();
    ipu.write32(reg::CTRL, 1 << 24, &mut NoDma); // PCT = I
    let mut w = BitWriter::default();
    w.put("1")
        .put("011")
        .put("01")
        .put("0000 0001 000")
        .put("1");
    w.put("0000 0000 0000 0000 0000 0001 1011 0011");
    ipu.write_fifo(&w.qwords(), &mut NoDma);
    let vdec = |ipu: &mut Ipu, tbl: u32| {
        ipu.write32(reg::CMD, cmd_word(cmd::VDEC, tbl << 26), &mut NoDma);
        assert!(!ipu.busy());
        ipu.read32(reg::CMD)
    };
    assert_eq!(vdec(&mut ipu, 0), 0x1_0001, "MBAI 1");
    assert_eq!(vdec(&mut ipu, 0), 0x3_0002, "MBAI 2");
    assert_eq!(vdec(&mut ipu, 1), 0x2_0011, "intra+quant");
    assert_eq!(vdec(&mut ipu, 0), 0xB_0023, "escape");
    assert_eq!(vdec(&mut ipu, 0), 0x1_0001);
    // FDEC peeks without consuming; FB skips.
    ipu.write32(reg::CMD, cmd_word(cmd::FDEC, 0), &mut NoDma);
    assert_eq!(ipu.read32(reg::CMD), 0x0000_01B3);
    assert_ne!(ipu.read32(reg::CTRL) & ctrl::SCD, 0);
    assert_eq!(ipu.read32(reg::TOP), 0x0000_01B3);
    ipu.write32(reg::CMD, cmd_word(cmd::FDEC, 8), &mut NoDma);
    assert_eq!(ipu.read32(reg::CMD), 0x0001_B300);
    // A start code where a VLC is expected: VDEC returns 0 with SCD.
    let mut ipu = Ipu::new();
    ipu.write_fifo(&[0x0100_0000u128], &mut NoDma);
    assert_eq!(vdec(&mut ipu, 0), 0);
    assert_ne!(ipu.read32(reg::CTRL) & ctrl::SCD, 0);
}

#[test]
fn starved_command_resumes() {
    let mut w = BitWriter::default();
    for _ in 0..3 {
        w.put("1010 1010 1010 1010 1010 1010 1010 1010");
    }
    w.put("1010 1010 1010 1010 1010 1010 1010 10");
    // 126 bits of noise to skip, so the first macroblock straddles a quadword.
    for _ in 0..20 {
        flat_intra_mb(&mut w);
    }
    let qs = w.qwords();
    let mut ipu = Ipu::new();
    ipu.write_fifo(&qs[..1], &mut NoDma);
    // Skip 63 + 63 bits: FDEC 63 then BDEC with FB 63.
    ipu.write32(reg::CMD, cmd_word(cmd::FDEC, 63), &mut NoDma);
    assert!(!ipu.busy());
    ipu.write32(
        reg::CMD,
        cmd_word(cmd::BDEC, 1 << 27 | 1 << 26 | 1 << 16 | 63),
        &mut NoDma,
    );
    assert!(ipu.busy());
    assert_eq!(ipu.out_pending(), 0, "starved, nothing emitted");
    ipu.write_fifo(&qs[1..], &mut NoDma);
    let raw = raw16(&ipu.read_fifo(48, &mut NoDma));
    assert!(raw.iter().all(|&v| v == 128));
    assert!(!ipu.busy());
    // BP reflects consumption: 126 + 28 bits.
    assert_eq!(ipu.read32(reg::BP) & 0x7F, 26);
}

#[test]
fn setiq_applies_matrix() {
    let mut ipu = Ipu::new();
    // Non-intra matrix of all 32 (sent in zigzag order).
    ipu.write_fifo(&[u128::from_le_bytes([32; 16]); 4], &mut NoDma);
    ipu.write32(reg::CMD, cmd_word(cmd::SETIQ, 1 << 27), &mut NoDma);
    assert!(!ipu.busy());
    assert!(ipu.nonintra_iq.iter().all(|&v| v == 32));
    // Intra matrix 0..63 in zigzag order lands at raster positions.
    let m: Vec<u8> = (0..64).collect();
    ipu.write_fifo(&bytes_to_qwords(&m), &mut NoDma);
    ipu.write32(reg::CMD, cmd_word(cmd::SETIQ, 0), &mut NoDma);
    for k in 0..64 {
        assert_eq!(ipu.intra_iq[tables::ZIGZAG[k] as usize], k as u8);
    }
}

// ----------------------------------------------------------------- CSC

fn raw8_uniform(y: u8, cb: u8, cr: u8) -> Vec<u8> {
    let mut v = vec![y; 256];
    v.extend(std::iter::repeat_n(cb, 64));
    v.extend(std::iter::repeat_n(cr, 64));
    v
}

#[test]
fn csc_known_colours() {
    let px = |y, cb, cr| {
        csc::ycbcr_to_rgb32(&raw8_uniform(y, cb, cr).try_into().unwrap(), 0, 0, false)[0]
    };
    assert_eq!(px(235, 128, 128), 0x80FF_FFFF, "white");
    assert_eq!(px(16, 128, 128), 0x8000_0000, "black");
    assert_eq!(px(128, 128, 128), 0x8082_8282, "mid grey");
    let red = px(81, 90, 240);
    assert!(
        red & 0xFF >= 250 && (red >> 8) & 0xFF <= 2 && (red >> 16) & 0xFF <= 2,
        "red {red:08X}"
    );
}

#[test]
fn csc_command_rgb32_rgb16_and_thresholds() {
    let mut ipu = Ipu::new();
    // SETTH: TH0 = 0x10, TH1 = 0x90.
    ipu.write32(
        reg::CMD,
        cmd_word(cmd::SETTH, 0x90 << 16 | 0x10),
        &mut NoDma,
    );
    let mut data = raw8_uniform(16, 128, 128); // black -> below TH0
    data.extend(raw8_uniform(128, 128, 128)); // grey 0x82 -> below TH1
    data.extend(raw8_uniform(235, 128, 128)); // white
    ipu.write_fifo(&bytes_to_qwords(&data), &mut NoDma);
    ipu.write32(reg::CMD, cmd_word(cmd::CSC, 3), &mut NoDma);
    let out = words(&ipu.read_fifo(64 * 3, &mut NoDma));
    assert!(!ipu.busy());
    assert_eq!(out[0], 0);
    assert_eq!(out[256], 0x4082_8282);
    assert_eq!(out[512], 0x80FF_FFFF);

    // RGB16 without dither: white -> 0x7FFF, grey (alpha 0x40) has A set.
    let mut data = raw8_uniform(235, 128, 128);
    data.extend(raw8_uniform(128, 128, 128));
    ipu.write_fifo(&bytes_to_qwords(&data), &mut NoDma);
    ipu.write32(reg::CMD, cmd_word(cmd::CSC, 1 << 27 | 2), &mut NoDma);
    let out = ipu.read_fifo(64, &mut NoDma);
    let h: Vec<u16> = out
        .iter()
        .flat_map(|q| q.to_le_bytes())
        .collect::<Vec<u8>>()
        .chunks(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect();
    assert_eq!(h[0], 0x7FFF);
    let g = 0x82 >> 3;
    assert_eq!(h[256], 0x8000 | g | g << 5 | g << 10);
}

#[test]
fn dither_and_pack_indx4() {
    let mut rgb = [0x8080_8080u32; 256];
    let plain = csc::rgb32_to_rgb16(&rgb, false);
    let dith = csc::rgb32_to_rgb16(&rgb, true);
    assert!(plain.iter().all(|&p| p == 0x4210));
    // Matrix entry (0,0) = -4: 0x80 - 4 = 0x7C -> 15.
    assert_eq!(dith[0] & 31, 15);
    assert_eq!(dith[1] & 31, 16);
    // PACK to INDX4 against a 2-colour CLUT (black, white).
    let mut ipu = Ipu::new();
    let mut clut = [0u16; 16];
    clut[1] = 0x7FFF;
    let cb: Vec<u8> = clut.iter().flat_map(|c| c.to_le_bytes()).collect();
    ipu.write_fifo(&bytes_to_qwords(&cb), &mut NoDma);
    ipu.write32(reg::CMD, cmd_word(cmd::SETVQ, 0), &mut NoDma);
    assert_eq!(ipu.vqclut[1], 0x7FFF);
    rgb[1] = 0x80FF_FFFF; // second pixel white
    let bytes: Vec<u8> = rgb.iter().flat_map(|p| p.to_le_bytes()).collect();
    ipu.write_fifo(&bytes_to_qwords(&bytes), &mut NoDma);
    ipu.write32(reg::CMD, cmd_word(cmd::PACK, 1), &mut NoDma);
    let out: Vec<u8> = ipu
        .read_fifo(8, &mut NoDma)
        .iter()
        .flat_map(|q| q.to_le_bytes())
        .collect();
    assert!(!ipu.busy());
    // Grey 0x80 (16) is nearer white (31) than black (0) only if > 15.5: 16 -> white.
    assert_eq!(out[0], 0x11);
}

// ------------------------------------------------- DMA through `Hw`

#[test]
fn dma_to_and_from_ipu_through_hw() {
    use crate::hw::{int, Hw};
    let mut hw = Hw::new();
    let mut ram = vec![0u8; 32 << 20];
    let mut spr = vec![0u8; 16 << 10];
    // Bitstream of 3 flat macroblocks at 0x10000, reached via a chain:
    // ref tag -> 0x10000 (n qwords), then end.
    let mut w = BitWriter::default();
    for _ in 0..3 {
        flat_intra_mb(&mut w);
    }
    let qs = w.qwords();
    for (i, q) in qs.iter().enumerate() {
        ram[0x10000 + i * 16..0x10010 + i * 16].copy_from_slice(&q.to_le_bytes());
    }
    let tag = |id: u64, qwc: u64, addr: u64| ((addr << 32) | (id << 28) | qwc) as u128;
    ram[0x2000..0x2010].copy_from_slice(&tag(3, qs.len() as u64, 0x10000).to_le_bytes());
    ram[0x2010..0x2020].copy_from_slice(&tag(7, 0, 0).to_le_bytes());
    hw.i_mask = 1 << int::IPU;
    // fromIPU: 3 * 48 qwords to 0x20000.
    hw.write32(0x1000_B010, 0x20000, &mut ram, &mut spr);
    hw.write32(0x1000_B020, 3 * 48, &mut ram, &mut spr);
    hw.write32(0x1000_B000, 0x100, &mut ram, &mut spr);
    // toIPU: source chain at 0x2000.
    hw.write32(0x1000_B430, 0x2000, &mut ram, &mut spr);
    hw.write32(0x1000_B420, 0, &mut ram, &mut spr);
    hw.write32(0x1000_B400, 0x104, &mut ram, &mut spr);
    for i in 0..3 {
        let dcr = if i == 0 { 1 << 26 } else { 0 };
        hw.write32(
            0x1000_2000,
            cmd_word(cmd::BDEC, 1 << 27 | dcr | 1 << 16),
            &mut ram,
            &mut spr,
        );
        assert_eq!(hw.read32(0x1000_2010) & ctrl::BUSY, 0, "mb {i} done");
    }
    let out = &ram[0x20000..0x20000 + 3 * 768];
    for c in out.chunks(2) {
        assert_eq!(i16::from_le_bytes([c[0], c[1]]), 128);
    }
    assert_eq!(hw.ch[3].chcr & 0x100, 0, "fromIPU complete");
    assert_eq!(hw.ch[4].chcr & 0x100, 0, "toIPU complete");
    assert_eq!(hw.d_stat & 0x18, 0x18);
    assert_ne!(hw.i_stat & (1 << int::IPU), 0);
}

// ---------------------------------------------- end to end: EABIG.MPC

/// A libmpeg-style driver that parses headers through FDEC/VDEC and decodes
/// macroblocks with BDEC (or IDEC), all through the IPU's register interface.
struct Driver {
    ipu: Ipu,
}

struct Frame {
    width: usize,
    height: usize,
    rgba: Vec<u32>,
}

impl Driver {
    fn command(&mut self, v: u32) {
        self.ipu.write32(reg::CMD, v, &mut NoDma);
    }

    fn fdec(&mut self, fb: u32) -> u32 {
        self.command(cmd_word(cmd::FDEC, fb));
        assert!(!self.ipu.busy(), "out of data");
        self.ipu.read32(reg::CMD)
    }

    fn skip(&mut self, mut n: u32) {
        while n > 0 {
            let k = n.min(32);
            self.fdec(k);
            n -= k;
        }
    }

    fn bits(&mut self, n: u32) -> u32 {
        let v = self.fdec(0) >> (32 - n);
        self.skip(n);
        v
    }

    fn vdec(&mut self, tbl: u32) -> u32 {
        self.command(cmd_word(cmd::VDEC, tbl << 26));
        assert!(!self.ipu.busy());
        self.ipu.read32(reg::CMD) & 0xFFFF
    }

    fn next_start_code(&mut self) -> u8 {
        let bp = self.ipu.read32(reg::BP) & 0x7F;
        self.skip((8 - bp % 8) % 8);
        loop {
            let v = self.fdec(0);
            if v >> 8 == 1 {
                self.skip(32);
                return v as u8;
            }
            self.skip(8);
        }
    }

    fn load(&mut self, data: &[u8]) {
        self.command(cmd_word(cmd::BCLR, 0));
        self.ipu.write_fifo(&bytes_to_qwords(data), &mut NoDma);
    }

    fn set_default_matrices(&mut self) {
        let zz: Vec<u8> = (0..64)
            .map(|k| DEFAULT_INTRA[tables::ZIGZAG[k] as usize])
            .collect();
        self.load(&zz);
        self.command(cmd_word(cmd::SETIQ, 0));
        self.load(&[16; 64]);
        self.command(cmd_word(cmd::SETIQ, 1 << 27));
    }

    /// Decode one I-picture (`data` starts at its picture start code).
    /// `idec`: use IDEC per slice instead of BDEC + CSC.
    fn decode_i_picture(
        &mut self,
        data: &[u8],
        width: usize,
        height: usize,
        idec: bool,
    ) -> Option<Frame> {
        let mut stream = data.to_vec();
        stream.extend_from_slice(&[0, 0, 1, 0xB7]); // sequence_end terminator
        stream.extend_from_slice(&[0; 32]);
        self.load(&stream);
        if self.next_start_code() != 0x00 {
            return None;
        }
        self.skip(10); // temporal_reference
        if self.bits(3) != 1 {
            return None;
        }
        let mbw = width / 16;
        let mut y_plane = vec![0u8; width * height];
        let mut cb_plane = vec![0u8; width * height / 4];
        let mut cr_plane = vec![0u8; width * height / 4];
        let mut rgba = vec![0u32; width * height];
        let mut fpfd = true;
        let mut code = self.next_start_code();
        let mut slices = 0;
        loop {
            match code {
                0xB5 => {
                    if self.bits(4) == 8 {
                        self.skip(16); // f_codes
                        let dc_prec = self.bits(2);
                        let structure = self.bits(2);
                        assert_eq!(structure, 3, "frame picture");
                        self.skip(1);
                        fpfd = self.bits(1) != 0;
                        self.skip(1);
                        let qst = self.bits(1);
                        let ivf = self.bits(1);
                        let alt = self.bits(1);
                        let c = dc_prec << 16 | alt << 20 | ivf << 21 | qst << 22 | 1 << 24;
                        self.ipu.write32(reg::CTRL, c, &mut NoDma);
                    }
                }
                0x01..=0xAF => {
                    slices += 1;
                    let row = code as usize - 1;
                    let mut qsc = self.bits(5);
                    while self.bits(1) == 1 {
                        self.skip(8);
                    }
                    let mut mb_x: isize = -1;
                    let mut dcr = true;
                    loop {
                        let mut inc = 0;
                        loop {
                            match self.vdec(0) {
                                0x23 => inc += 33,
                                0x22 => {}
                                0 => panic!("bad MBAI"),
                                v => {
                                    inc += v as isize;
                                    break;
                                }
                            }
                        }
                        mb_x += inc;
                        if idec {
                            let dtd = if fpfd { 0 } else { 1 << 24 };
                            self.command(cmd_word(cmd::IDEC, dtd | qsc << 16));
                            let n = self.ipu.out_pending();
                            assert_eq!(n % 64, 0);
                            let out = words(&self.ipu.read_fifo(n, &mut NoDma));
                            assert!(!self.ipu.busy());
                            assert_eq!(self.ipu.read32(reg::CTRL) & ctrl::ECD, 0);
                            for (i, mb) in out.chunks(256).enumerate() {
                                let mx = mb_x as usize + i;
                                for y in 0..16 {
                                    for x in 0..16 {
                                        rgba[(row * 16 + y) * width + mx * 16 + x] = mb[y * 16 + x];
                                    }
                                }
                            }
                            break;
                        }
                        let ty = self.vdec(1);
                        assert_ne!(ty & 1, 0, "intra");
                        let dct = if fpfd { 0 } else { self.bits(1) };
                        if ty & 0x10 != 0 {
                            qsc = self.bits(5);
                        }
                        let d = if dcr { 1 << 26 } else { 0 };
                        self.command(cmd_word(cmd::BDEC, 1 << 27 | d | dct << 25 | qsc << 16));
                        dcr = false;
                        let raw = raw16(&self.ipu.read_fifo(48, &mut NoDma));
                        assert!(!self.ipu.busy());
                        assert_eq!(self.ipu.read32(reg::CTRL) & ctrl::ECD, 0);
                        let mx = mb_x as usize;
                        for y in 0..16 {
                            for x in 0..16 {
                                y_plane[(row * 16 + y) * width + mx * 16 + x] =
                                    raw[y * 16 + x].clamp(0, 255) as u8;
                            }
                        }
                        for y in 0..8 {
                            for x in 0..8 {
                                let o = (row * 8 + y) * (width / 2) + mx * 8 + x;
                                cb_plane[o] = raw[256 + y * 8 + x].clamp(0, 255) as u8;
                                cr_plane[o] = raw[320 + y * 8 + x].clamp(0, 255) as u8;
                            }
                        }
                        let scd = self.ipu.read32(reg::CTRL) & ctrl::SCD != 0;
                        if self.fdec(0) >> 9 == 0 {
                            assert!(scd, "BDEC flags the start code ending the slice");
                            break;
                        }
                        assert!(!scd);
                    }
                }
                _ => break,
            }
            code = self.next_start_code();
        }
        assert_eq!(slices, height / 16);
        if !idec {
            // CSC every macroblock, as libmpeg does after motion compensation.
            for my in 0..height / 16 {
                for mx in 0..mbw {
                    let mut raw8 = Vec::with_capacity(384);
                    for y in 0..16 {
                        let o = (my * 16 + y) * width + mx * 16;
                        raw8.extend_from_slice(&y_plane[o..o + 16]);
                    }
                    for plane in [&cb_plane, &cr_plane] {
                        for y in 0..8 {
                            let o = (my * 8 + y) * (width / 2) + mx * 8;
                            raw8.extend_from_slice(&plane[o..o + 8]);
                        }
                    }
                    self.load(&raw8);
                    self.command(cmd_word(cmd::CSC, 1));
                    let out = words(&self.ipu.read_fifo(64, &mut NoDma));
                    for y in 0..16 {
                        for x in 0..16 {
                            rgba[(my * 16 + y) * width + mx * 16 + x] = out[y * 16 + x];
                        }
                    }
                }
            }
        }
        Some(Frame {
            width,
            height,
            rgba,
        })
    }
}

/// EABIG.MPC from the mounted disc, or from the ISO found above the crate.
fn load_eabig() -> Option<Vec<u8>> {
    if let Ok(d) = std::fs::read("/Volumes/SSX3/DATA/MOVIES/EABIG.MPC") {
        return Some(d);
    }
    let iso = std::env::var_os("SSX3_ISO")
        .map(std::path::PathBuf::from)
        .or_else(|| {
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .ancestors()
                .map(|a| a.join("SSX 3 (USA).iso"))
                .find(|p| p.exists())
        })?;
    let mut disc = crate::iso::Disc::open(&iso).ok()?;
    let e = disc.find("DATA/MOVIES/EABIG.MPC")?;
    let mut d = disc.read_vec(e.sector, e.size.div_ceil(crate::iso::SECTOR as u32));
    d.truncate(e.size as usize);
    Some(d)
}

/// Split the EA Gimex container into its `MPCh` (video) chunk payloads.
fn mpc_video_chunks(d: &[u8]) -> Vec<&[u8]> {
    let mut v = Vec::new();
    let mut o = 0;
    while o + 8 <= d.len() {
        let size = u32::from_le_bytes(d[o + 4..o + 8].try_into().unwrap()) as usize;
        if size < 8 || o + size > d.len() {
            break;
        }
        if &d[o..o + 4] == b"MPCh" {
            v.push(&d[o + 8..o + size]);
        }
        o += size;
    }
    v
}

fn write_png(path: &std::path::Path, f: &Frame) {
    fn crc32(data: &[u8]) -> u32 {
        let mut c = 0xFFFF_FFFFu32;
        for b in data {
            c ^= *b as u32;
            for _ in 0..8 {
                c = if c & 1 != 0 {
                    0xEDB8_8320 ^ (c >> 1)
                } else {
                    c >> 1
                };
            }
        }
        !c
    }
    let mut raw = Vec::new();
    for y in 0..f.height {
        raw.push(0);
        for x in 0..f.width {
            let p = f.rgba[y * f.width + x] | 0xFF00_0000;
            raw.extend_from_slice(&p.to_le_bytes());
        }
    }
    let mut z = vec![0x78, 0x01];
    let n = raw.chunks(65535).count();
    for (i, block) in raw.chunks(65535).enumerate() {
        z.push((i + 1 == n) as u8);
        z.extend_from_slice(&(block.len() as u16).to_le_bytes());
        z.extend_from_slice(&(!(block.len() as u16)).to_le_bytes());
        z.extend_from_slice(block);
    }
    let (mut a, mut b) = (1u32, 0u32);
    for x in &raw {
        a = (a + *x as u32) % 65521;
        b = (b + a) % 65521;
    }
    z.extend_from_slice(&((b << 16) | a).to_be_bytes());
    let mut out = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
    let mut chunk = |kind: &[u8; 4], data: &[u8]| {
        out.extend_from_slice(&(data.len() as u32).to_be_bytes());
        let mut c = kind.to_vec();
        c.extend_from_slice(data);
        out.extend_from_slice(&c);
        out.extend_from_slice(&crc32(&c).to_be_bytes());
    };
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&(f.width as u32).to_be_bytes());
    ihdr.extend_from_slice(&(f.height as u32).to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
    chunk(b"IHDR", &ihdr);
    chunk(b"IDAT", &z);
    chunk(b"IEND", &[]);
    std::fs::write(path, out).unwrap();
}

#[test]
fn eabig_i_frames_decode() {
    let Some(mpc) = load_eabig() else {
        eprintln!("EABIG.MPC not available (mount the disc or set SSX3_ISO); skipping");
        return;
    };
    let chunks = mpc_video_chunks(&mpc);
    assert!(chunks.len() > 10);
    // Sequence header (first chunk): 512x448, no custom matrices.
    let seq = chunks[0];
    assert_eq!(&seq[..4], &[0, 0, 1, 0xB3]);
    let width = ((seq[4] as usize) << 4) | (seq[5] as usize >> 4);
    let height = (((seq[5] & 0xF) as usize) << 8) | seq[6] as usize;
    assert_eq!((width, height), (512, 448));
    assert_eq!(seq[11] & 3, 0, "default quantiser matrices");
    let out_dir = std::env::var_os("SSX3_IPU_PNG_DIR").map(std::path::PathBuf::from);
    let max = std::env::var("SSX3_IPU_FRAMES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2usize);

    let mut drv = Driver { ipu: Ipu::new() };
    drv.set_default_matrices();
    let mut decoded = 0;
    for (n, c) in chunks.iter().enumerate() {
        let Some(pic) = c.windows(4).position(|w| w == [0, 0, 1, 0]) else {
            continue;
        };
        if (c[pic + 5] >> 3) & 7 != 1 {
            continue; // not an I-picture
        }
        let bdec = drv
            .decode_i_picture(&c[pic..], width, height, false)
            .expect("I-picture");
        let idec = drv
            .decode_i_picture(&c[pic..], width, height, true)
            .expect("I-picture");
        assert!(
            bdec.rgba == idec.rgba,
            "IDEC and BDEC+CSC agree on chunk {n}"
        );
        // Plausibility: not a flat image, and mostly in-gamut luma variety.
        let mut hist = [0usize; 16];
        for p in &bdec.rgba {
            let l = ((p & 0xFF) + ((p >> 8) & 0xFF) + ((p >> 16) & 0xFF)) / 3;
            hist[(l >> 4) as usize] += 1;
        }
        eprintln!("chunk {n}: I-picture, luma histogram {hist:?}");
        if let Some(dir) = &out_dir {
            write_png(&dir.join(format!("eabig_chunk{n:03}.png")), &bdec);
        }
        decoded += 1;
        if decoded >= max {
            break;
        }
    }
    assert!(decoded > 0);
    assert_eq!(drv.ipu.stats.bad_codes, 0);
}

/// Chunks fed the way the game's codec does (`FeedBitstream`): one per
/// request, zero-padded to a quadword plus one more, then the sequence end.
struct ChunkFeed<'a> {
    chunks: Vec<&'a [u8]>,
    next: usize,
    ended: bool,
}

impl ssx3_mpeg::decoder::Feed for ChunkFeed<'_> {
    fn more(&mut self, out: &mut Vec<u8>) -> bool {
        if let Some(c) = self.chunks.get(self.next) {
            self.next += 1;
            let n = (c.len() + 15) / 16 * 16 + 16;
            out.extend_from_slice(c);
            out.resize(out.len() + n - c.len(), 0);
            return true;
        }
        if !self.ended {
            self.ended = true;
            for _ in 0..4 {
                out.extend_from_slice(&[0, 0, 1, 0xb7]);
            }
            return true;
        }
        false
    }
}

#[test]
fn eabig_native_decoder() {
    let Some(mpc) = load_eabig() else {
        eprintln!("EABIG.MPC not available (mount the disc or set SSX3_ISO); skipping");
        return;
    };
    let chunks = mpc_video_chunks(&mpc);
    let (width, height) = (512usize, 448usize);
    // The first I-picture through the IPU.
    let first_i = chunks
        .iter()
        .find_map(|c| {
            let pic = c.windows(4).position(|w| w == [0, 0, 1, 0])?;
            ((c[pic + 5] >> 3) & 7 == 1).then(|| &c[pic..])
        })
        .unwrap();
    let mut drv = Driver { ipu: Ipu::new() };
    drv.set_default_matrices();
    let ipu = drv
        .decode_i_picture(first_i, width, height, false)
        .expect("I-picture");
    // The whole movie natively.
    let mut feed = ChunkFeed {
        chunks,
        next: 0,
        ended: false,
    };
    let mut d = ssx3_mpeg::decoder::Decoder::new();
    let mut out = vec![0u8; width * height * 4];
    let mut pictures = 0;
    let t = std::time::Instant::now();
    while d.picture(&mut feed) == ssx3_mpeg::decoder::Got::Picture {
        if pictures == 0 {
            d.output_rgb32(&mut out);
            // libmpeg's macroblock order (columns) -> raster.
            let mbh = height / 16;
            for (i, &p) in ipu.rgba.iter().enumerate() {
                let (x, y) = (i % width, i / width);
                let mb = (x / 16) * mbh + y / 16;
                let o = mb * 1024 + ((y % 16) * 16 + x % 16) * 4;
                let n = u32::from_le_bytes(out[o..o + 4].try_into().unwrap());
                assert_eq!(n, p, "pixel ({x}, {y}) of the first picture");
            }
        }
        pictures += 1;
    }
    eprintln!(
        "EABIG natively: {pictures} pictures, {} errors, {:?}",
        d.errors,
        t.elapsed()
    );
    assert_eq!(d.errors, 0);
    assert_eq!(pictures, 75);
    assert!(d.ended);
}

/// Debugging aid: `SSX3_MPEG_REF_DIR=<dir>` with `pic<k>.rgba` dumps of the
/// emulated path (`SSX3_MOVIE_DUMP`): report where the native pictures
/// differ, with the macroblocks' predictions.
#[test]
fn eabig_native_vs_dumps() {
    let (Some(dir), Some(mpc)) = (std::env::var_os("SSX3_MPEG_REF_DIR"), load_eabig()) else {
        return;
    };
    let chunks = mpc_video_chunks(&mpc);
    let mut feed = ChunkFeed {
        chunks,
        next: 0,
        ended: false,
    };
    let mut d = ssx3_mpeg::decoder::Decoder::new();
    d.log_mbs = true;
    let mut out = vec![0u8; 512 * 448 * 4];
    for k in 0..8 {
        assert_eq!(d.picture(&mut feed), ssx3_mpeg::decoder::Got::Picture);
        let Ok(r) = std::fs::read(std::path::Path::new(&dir).join(format!("pic{k}.rgba"))) else {
            continue;
        };
        d.output_rgb32(&mut out);
        let mut bad = std::collections::BTreeMap::new();
        for (i, (a, b)) in out.iter().zip(&r).enumerate() {
            if a != b {
                *bad.entry(i / 1024).or_insert(0) += 1;
            }
        }
        eprintln!("picture {k}: {} macroblocks differ", bad.len());
        // Bidirectional macroblocks by half-pel kind of each vector
        // (0 full, 1 x, 2 y, 3 xy): (count, differing).
        let mut cat = std::collections::BTreeMap::new();
        for &(mb, f, b, mv) in &d.mb_log {
            let kind = |v: [i32; 2]| (v[0] & 1 | (v[1] & 1) << 1) as u8;
            let key = (
                f,
                b,
                if f { kind(mv[0]) } else { 9 },
                if b { kind(mv[1]) } else { 9 },
            );
            let (mx, my) = (mb % 32, mb / 32);
            let e = cat.entry(key).or_insert((0, 0));
            e.0 += 1;
            if bad.contains_key(&(mx * 28 + my)) {
                e.1 += 1;
            }
        }
        eprintln!("  (fwd, bwd, fkind, bkind) -> (count, differing): {cat:?}");
        for (&mb, &n) in bad.iter().take(6) {
            let (mx, my) = (mb / 28, mb % 28);
            let info = d.mb_log.iter().find(|x| x.0 == my * 32 + mx);
            eprintln!("  mb ({mx}, {my}): {n} bytes; prediction {info:?}");
        }
    }
}
