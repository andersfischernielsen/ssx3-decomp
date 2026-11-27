#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, scramble, Rng, DATA};

    /// Slots holding the encoder state pointers (D_0050AC68 and D_0050AC6C).
    const OC1096_Q_SLOT: u32 = 0x0050_ac68;
    const OC1096_P_SLOT: u32 = 0x0050_ac6c;
    /// D_0050AC6C: subband state, with the work buffer after it.
    const OC1096_P: u32 = DATA + 0x1000;
    /// D_0050AC68: frame state (mode +0, channels +1, size +4, bit pos +6,
    /// out +8, band factors +0xc, channel block pointers +0x84).
    const OC1096_Q: u32 = DATA + 0x4000;
    const OC1096_BLK: u32 = DATA + 0x5000;
    const OC1096_OUT: u32 = DATA + 0x20000;
    const OC1096_PEAKS: u32 = DATA + 0x3000;
    const OC1096_SPR: u32 = 0x7000_0000;

    fn oc1096_floats(c: &mut Cpu, r: &mut Rng, base: u32, n: u32, scale: f32) {
        for i in 0..n {
            let v = (r.next_u32() % 2000) as f32 / 1000.0 - 1.0;
            c.bus.write32(base + i * 4, (v * scale).to_bits());
        }
    }

    fn oc1096_words(c: &mut Cpu, r: &mut Rng, base: u32, n: u32) {
        for i in 0..n {
            let v = r.next_u32();
            c.bus.write32(base + i * 4, v);
        }
    }

    /// Writes the DTS header bits for every mode / channel count.
    #[test]
    fn sndenc_write_header_matches() {
        check(
            ee::s13::f_003bcba0__generated,
            sndenc_write_header,
            24,
            &|c, seed| {
                let mut r = Rng(0x1096_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(OC1096_Q_SLOT, OC1096_Q);
                c.bus.write8(OC1096_Q, [1, 2, 0][(seed % 3) as usize]);
                c.bus.write8(OC1096_Q + 1, (seed / 3 % 4 + 1) as u8);
                c.bus.write32(OC1096_Q + 8, OC1096_OUT);
                c.bus.write16(OC1096_Q + 6, (r.next_u32() % 64) as u16);
                oc1096_words(c, &mut r, OC1096_OUT, 0x800);
                c.pc = 0x003b_cba0;
            },
        );
    }

    /// Shared fixture of the analysis filterbank (see the two tests below).
    fn oc1096_analysis_setup(c: &mut Cpu, seed: u32, spr: bool) {
        let mut r = Rng(0x1096_0002 ^ seed as u64);
        scramble(c, &mut r);
        elf_image(c);
        let size = [0xe0u32, 0x160, 0x1e0][(seed % 3) as usize];
        c.bus.write32(OC1096_Q_SLOT, OC1096_Q);
        c.bus.write32(OC1096_P_SLOT, OC1096_P);
        c.bus.write16(OC1096_Q + 4, size as u16);
        let work = if spr { OC1096_SPR } else { DATA + 0x30000 };
        c.bus.write32(OC1096_P + 0xa20, work);
        c.bus.write32(OC1096_P + 0xa24, DATA + 0x40000);
        for (i, off) in [0xa28u32, 0xa2c, 0xa30, 0xa34, 0xa38].iter().enumerate() {
            c.bus.write32(OC1096_P + off, DATA + 0x44000 + 0x400 * i as u32);
        }
        c.bus.write8(OC1096_P + 0xa3c, spr as u8);
        oc1096_floats(c, &mut r, DATA + 0x40000, 0x400, 1.0);
        oc1096_floats(c, &mut r, DATA + 0x44000, 0x800, 1.0);
        oc1096_words(c, &mut r, DATA + 0x50000, 0x400); // pcm
        oc1096_words(c, &mut r, DATA + 0x52000, 0x400); // history
        oc1096_floats(c, &mut r, OC1096_P + 0x600, 0x100, 1.0);
        c.g[A0][0] = sx(DATA + 0x50000);
        c.g[A1][0] = sx(DATA + 0x54000);
        c.g[A2][0] = sx(DATA + 0x52000);
    }

    /// Analysis filterbank, plain (non-scratchpad) path with the three
    /// window sizes.
    #[test]
    fn sndenc_analysis_filterbank_matches() {
        check(
            ee::s13::f_003bd220__generated,
            sndenc_analysis_filterbank,
            12,
            &|c, seed| oc1096_analysis_setup(c, seed, false),
        );
    }

    /// Analysis filterbank through the scratchpad DMA path, and with the
    /// pcm or history buffer itself in the scratchpad (which turns it off).
    #[test]
    fn sndenc_analysis_filterbank_spr_matches() {
        check(
            ee::s13::f_003bd220__generated,
            sndenc_analysis_filterbank,
            18,
            &|c, seed| {
                oc1096_analysis_setup(c, seed, true);
                match seed / 3 % 3 {
                    1 => c.g[A0][0] = sx(OC1096_SPR + 0x2000),
                    2 => c.g[A2][0] = sx(OC1096_SPR + 0x3000),
                    _ => {}
                }
            },
        );
    }

    /// Mode-1 channel encode: scale factors from the subband peaks, 14 groups
    /// of 10-bit and 10 of 8-bit quantised code words packed into the frame.
    #[test]
    fn sndenc_encode_channel_mode1_matches() {
        check(
            ee::s13::f_003bd5b8__generated,
            sndenc_encode_channel_mode1,
            40,
            &|c, seed| {
                let mut r = Rng(0x1096_0003 ^ seed as u64);
                scramble(c, &mut r);
                elf_image(c);
                let ch = seed & 1;
                c.g[A0][0] = ch as u64;
                c.bus.write32(OC1096_P_SLOT, OC1096_P);
                c.bus.write32(OC1096_Q_SLOT, OC1096_Q);
                c.bus.write32(OC1096_P + 0xa20, OC1096_PEAKS);
                c.bus.write32(OC1096_Q + 8, OC1096_OUT);
                c.bus.write32(OC1096_Q + 0x84 + 4 * ch, OC1096_BLK);
                let scale = [0.5f32, 4.0, 100.0, 1000.0][(seed % 4) as usize];
                oc1096_floats(c, &mut r, OC1096_P, 24 * 16, scale);
                for i in 0..24u32 {
                    let f = 0.1 + (r.next_u32() % 3000) as f32 / 1000.0;
                    c.bus.write32(OC1096_Q + 0xc + 4 * i, f.to_bits());
                }
            },
        );
    }

    /// Mode-1 header: packs the 5-bit scale factors of each channel pair into
    /// the frame header at out+0x8d.
    #[test]
    fn sndenc_pack_header_mode1_matches() {
        check(
            ee::s13::f_003bd9c0__generated,
            sndenc_pack_header_mode1,
            30,
            &|c, seed| {
                let mut r = Rng(0x1096_0004 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(OC1096_Q_SLOT, OC1096_Q);
                c.bus.write8(OC1096_Q + 1, (seed % 5) as u8);
                c.bus.write32(OC1096_Q + 8, OC1096_OUT);
                for i in 0..8u32 {
                    let blk = OC1096_BLK + 0x100 * i;
                    c.bus.write32(OC1096_Q + 0x84 + 4 * i, blk);
                    for j in 0..0x20u32 {
                        let v = r.next_u32() as u8;
                        c.bus.write8(blk + 4 + j, v);
                    }
                }
            },
        );
    }

    /// Whole frame encode, mode 1 or 2, one to three channels, a repeated key
    /// (no header rewrite) and the odd-pointer / misaligned-data error exits.
    fn oc1096_frame_setup(c: &mut Cpu, seed: u32, spr: bool) {
        let mut r = Rng(0x1096_0005 ^ seed as u64);
        scramble(c, &mut r);
        elf_image(c);
        let size = [0xe0u32, 0x160, 0x1e0][(seed % 3) as usize];
        let nch = (seed / 3 % 3 + 1) as u8;
        let mode = if seed / 9 % 2 == 0 { 1u8 } else { 2 };
        let key = DATA + 0x20000 + if seed % 11 == 10 { 1 } else { 0 };
        c.bus.write32(OC1096_Q_SLOT, OC1096_Q);
        c.bus.write8(OC1096_Q, mode);
        c.bus.write8(OC1096_Q + 1, nch);
        c.bus.write16(OC1096_Q + 4, size as u16);
        c.bus.write32(OC1096_Q + 8, if seed % 5 == 4 { key } else { 0 });
        let data = DATA + 0x60000;
        for i in 0..3u32 {
            let blk = OC1096_BLK + 0x100 * i;
            c.bus.write32(OC1096_Q + 0x84 + 4 * i, blk);
            // *ptrs[i] is the history buffer
            c.bus.write32(blk, DATA + 0x70000 + 0x2000 * i);
            let pcm = DATA + 0x80000 + 0x2000 * i + if seed % 13 == 12 && i == 1 { 8 } else { 4 * 4 };
            c.bus.write32(data + 4 * i, pcm);
        }
        oc1096_floats(c, &mut r, OC1096_Q + 0xc, 24, 2.0);
        oc1096_words(c, &mut r, DATA + 0x80000, 0x1800);
        oc1096_words(c, &mut r, DATA + 0x70000, 0x1800);
        c.g[A0][0] = sx(data);
        c.g[A1][0] = sx(key);
        c.g[A2][0] = sx(if spr { OC1096_SPR } else { DATA + 0x10000 });
        c.pc = 0x003b_f0f0;
    }

    #[test]
    fn sndenc_encode_frame_matches() {
        check(
            ee::s13::f_003bf0f0__generated,
            sndenc_encode_frame,
            36,
            &|c, seed| oc1096_frame_setup(c, seed, false),
        );
    }

    /// The same with the work buffer in the scratchpad (DMA copies of the
    /// window tables).
    #[test]
    fn sndenc_encode_frame_spr_matches() {
        check(
            ee::s13::f_003bf0f0__generated,
            sndenc_encode_frame,
            18,
            &|c, seed| oc1096_frame_setup(c, seed, true),
        );
    }
}
