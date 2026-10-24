//! A small GIF packet reader for the streams the VIF1 chain sends through
//! DIRECT/DIRECTHL (PATH2): it follows GIF tags across PACKED, REGLIST and
//! IMAGE data and reports the register writes that matter for draw state.
//!
//! Only PACKED A+D (register descriptor 0xe) writes are reported as
//! `(register, value)`; the other PACKED descriptors (PRIM, RGBAQ, ST, UV,
//! XYZ..) and REGLIST data are vertex data and are skipped. A tag's data may
//! continue in the next DIRECT block, so the reader keeps its position.

/// GS register ids used by the draw state.
pub mod reg {
    pub const PRIM: u8 = 0x00;
    pub const TEX0_1: u8 = 0x06;
    pub const TEX0_2: u8 = 0x07;
    pub const CLAMP_1: u8 = 0x08;
    pub const CLAMP_2: u8 = 0x09;
    pub const TEX1_1: u8 = 0x14;
    pub const TEX1_2: u8 = 0x15;
    pub const TEX2_1: u8 = 0x16;
    pub const TEX2_2: u8 = 0x17;
    pub const MIPTBP1_1: u8 = 0x34;
    pub const MIPTBP1_2: u8 = 0x35;
    pub const MIPTBP2_1: u8 = 0x36;
    pub const MIPTBP2_2: u8 = 0x37;
    pub const SCISSOR_1: u8 = 0x40;
    pub const SCISSOR_2: u8 = 0x41;
    pub const ALPHA_1: u8 = 0x42;
    pub const ALPHA_2: u8 = 0x43;
    pub const TEST_1: u8 = 0x47;
    pub const TEST_2: u8 = 0x48;
    pub const FRAME_1: u8 = 0x4c;
    pub const FRAME_2: u8 = 0x4d;
    pub const ZBUF_1: u8 = 0x4e;
    pub const ZBUF_2: u8 = 0x4f;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Mode {
    /// Expecting a GIF tag.
    #[default]
    Tag,
    /// PACKED: `left` qwords to go; `k` = current register index.
    Packed { left: u32, k: u32 },
    /// REGLIST or IMAGE: `left` qwords of data to skip.
    Skip { left: u32 },
}

/// Streaming GIF reader.
#[derive(Clone, Debug, Default)]
pub struct GifReader {
    mode: Mode,
    nreg: u32,
    regs: u64,
}

impl GifReader {
    /// Feed qwords; `out` gets every A+D write `(register, value)` in order.
    pub fn feed(&mut self, data: &[u128], out: &mut dyn FnMut(u8, u64)) {
        for &q in data {
            match self.mode {
                Mode::Tag => {
                    let nloop = (q & 0x7fff) as u32;
                    let flg = ((q >> 58) & 3) as u32;
                    let nreg = match ((q >> 60) & 0xf) as u32 {
                        0 => 16,
                        n => n,
                    };
                    self.nreg = nreg;
                    self.regs = (q >> 64) as u64;
                    let qwords = match flg {
                        0 => nloop * nreg,
                        1 => (nloop * nreg).div_ceil(2),
                        _ => nloop,
                    };
                    if qwords > 0 {
                        self.mode = if flg == 0 {
                            Mode::Packed { left: qwords, k: 0 }
                        } else {
                            Mode::Skip { left: qwords }
                        };
                    }
                }
                Mode::Packed { left, k } => {
                    let desc = ((self.regs >> (4 * k)) & 0xf) as u8;
                    if desc == 0xe {
                        out(((q >> 64) & 0xff) as u8, q as u64);
                    }
                    let k = if k + 1 == self.nreg { 0 } else { k + 1 };
                    self.mode = if left > 1 {
                        Mode::Packed { left: left - 1, k }
                    } else {
                        Mode::Tag
                    };
                }
                Mode::Skip { left } => {
                    self.mode = if left > 1 {
                        Mode::Skip { left: left - 1 }
                    } else {
                        Mode::Tag
                    };
                }
            }
        }
    }

    /// True between packets (no tag data pending).
    pub fn at_tag(&self) -> bool {
        self.mode == Mode::Tag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tag(nloop: u32, flg: u32, nreg: u32, regs: u64) -> u128 {
        nloop as u128
            | (1u128 << 15)
            | (flg as u128) << 58
            | (nreg as u128) << 60
            | (regs as u128) << 64
    }

    #[test]
    fn packed_ad_across_blocks_and_skips_reglist() {
        let mut g = GifReader::default();
        let mut seen = Vec::new();
        // REGLIST 3 x 1 reg = 2 qwords to skip, then A+D x2 split over two feeds.
        g.feed(
            &[tag(3, 1, 1, 0x5), 0x47 << 64 | 0xdead, 0x1234],
            &mut |r, v| seen.push((r, v)),
        );
        g.feed(
            &[tag(2, 0, 1, 0xe), (0x47u128 << 64) | 0x5_0001],
            &mut |r, v| seen.push((r, v)),
        );
        assert!(!g.at_tag());
        g.feed(&[(0x06u128 << 64) | 0x99], &mut |r, v| seen.push((r, v)));
        assert!(g.at_tag());
        assert_eq!(seen, vec![(0x47, 0x5_0001), (0x06, 0x99)]);
    }

    #[test]
    fn packed_mixed_registers_only_reports_ad() {
        let mut g = GifReader::default();
        let mut seen = Vec::new();
        // NLOOP 2, NREG 2: {PRIM, A+D}.
        g.feed(
            &[
                tag(2, 0, 2, 0xe0),
                1,
                (0x42u128 << 64) | 0x44,
                2,
                (0x43u128 << 64) | 0x48,
            ],
            &mut |r, v| seen.push((r, v)),
        );
        assert_eq!(seen, vec![(0x42, 0x44), (0x43, 0x48)]);
        // IMAGE: NLOOP qwords of data.
        g.feed(&[tag(2, 2, 0, 0), 5, 6], &mut |r, v| seen.push((r, v)));
        assert!(g.at_tag());
        assert_eq!(seen.len(), 2);
    }
}
