//! PS2 GIF packet interpreter.
//!
//! The GIF (Graphics Interface) moves display lists from EE RAM to the GS. A
//! GIF *packet* is a 128-bit [`GifTag`] followed by payload whose shape depends
//! on the tag's `FLG` field:
//!
//! * `PACKED` (0): `NLOOP * NREG` 128-bit "packed" register words.
//! * `REGLIST` (1): `NLOOP * NREG` 64-bit register values; the register list is
//!   carried in the tag itself.
//! * `IMAGE` (2) / `IMAGE2` (3): `NLOOP` 128-bit words of raw image data.
//!
//! [`Gs::write_gif`] walks a byte stream of such packets and returns how many
//! bytes it safely consumed. The parser never panics: a tag or payload that runs
//! past the end of the buffer simply stops parsing at that point and reports the
//! bytes handled so far.

use crate::regs;
use crate::{Gs, VRAM_SIZE};

/// Size of a [`GifTag`] in bytes.
pub const TAG_SIZE: usize = 16;

/// Number of 4-bit register descriptors packed into a tag's `REGS` field.
pub const MAX_REGS: usize = 16;

/// A decoded 128-bit GIFtag.
///
/// The bit layout follows the GS manual (and PCSX2's `GIFTag` union):
///
/// | Bits    | Field  |
/// |---------|--------|
/// | 0..14   | NLOOP  |
/// | 15      | EOP    |
/// | 46      | PRE    |
/// | 47..57  | PRIM   |
/// | 58..59  | FLG    |
/// | 60..63  | NREG   |
/// | 64..127 | REGS   |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GifTag {
    /// Number of times the register list is repeated. `0` means the tag
    /// produces no output.
    pub nloop: u32,
    /// End of packet: this is the last tag of the transfer.
    pub eop: bool,
    /// When set (and `flg == PACKED`) latch [`GifTag::prim`] into the GS first.
    pub pre: bool,
    /// `PRIM` value latched when `pre` is set.
    pub prim: u32,
    /// Transfer format; see [`regs::flg`].
    pub flg: u32,
    /// Raw `NREG` field. `0` is interpreted as 16 by [`GifTag::reg_count`].
    pub nreg: u32,
    /// The 16 packed 4-bit register descriptors from `REGS`.
    pub regs: [u8; MAX_REGS],
}

impl GifTag {
    /// Decode a little-endian 128-bit tag.
    pub fn parse(raw: &[u8; TAG_SIZE]) -> Self {
        let lo = u64::from_le_bytes(raw[0..8].try_into().expect("8 bytes"));
        let hi = u64::from_le_bytes(raw[8..16].try_into().expect("8 bytes"));

        let mut regs = [0u8; MAX_REGS];
        for (i, r) in regs.iter_mut().enumerate() {
            *r = ((hi >> (4 * i)) & 0xf) as u8;
        }

        Self {
            nloop: (lo & 0x7fff) as u32,
            eop: (lo >> 15) & 1 != 0,
            pre: (lo >> 46) & 1 != 0,
            prim: ((lo >> 47) & 0x7ff) as u32,
            flg: ((lo >> 58) & 0x3) as u32,
            nreg: ((lo >> 60) & 0xf) as u32,
            regs,
        }
    }

    /// Number of registers present. The 4-bit `NREG` field encodes `0` as 16.
    pub fn reg_count(&self) -> u32 {
        if self.nreg == 0 {
            MAX_REGS as u32
        } else {
            self.nreg
        }
    }
}

/// Decoded `BITBLTBUF` register (`0x50`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BitbltBuf {
    /// Source base pointer, in units of 256 bytes.
    pub sbp: u32,
    /// Source buffer width, in units of 64 pixels.
    pub sbw: u32,
    /// Source pixel-storage format.
    pub spsm: u32,
    /// Destination base pointer, in units of 256 bytes.
    pub dbp: u32,
    /// Destination buffer width, in units of 64 pixels.
    pub dbw: u32,
    /// Destination pixel-storage format.
    pub dpsm: u32,
}

impl BitbltBuf {
    /// Split a 64-bit register value into its fields.
    pub fn from_u64(v: u64) -> Self {
        Self {
            sbp: (v & 0x3fff) as u32,
            sbw: ((v >> 16) & 0x3f) as u32,
            spsm: ((v >> 24) & 0x3f) as u32,
            dbp: ((v >> 32) & 0x3fff) as u32,
            dbw: ((v >> 48) & 0x3f) as u32,
            dpsm: ((v >> 56) & 0x3f) as u32,
        }
    }

    /// Byte offset of the destination base pointer inside VRAM.
    pub fn dest_base(&self) -> usize {
        (self.dbp as usize) * 256
    }

    /// Destination row stride in bytes (the buffer width is in 64-pixel units).
    pub fn dest_row_bytes(&self) -> usize {
        self.dbw as usize * 64 * bits_per_pixel(self.dpsm) as usize / 8
    }
}

/// Decoded `TRXPOS` register (`0x51`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TrxPos {
    /// Source upper-left X.
    pub ssax: u32,
    /// Source upper-left Y.
    pub ssay: u32,
    /// Destination upper-left X.
    pub dsax: u32,
    /// Destination upper-left Y.
    pub dsay: u32,
    /// Pixel order, packed as `DIRY | (DIRX << 1)`.
    pub dir: u32,
}

impl TrxPos {
    /// Split a 64-bit register value into its fields.
    pub fn from_u64(v: u64) -> Self {
        Self {
            ssax: (v & 0x7ff) as u32,
            ssay: ((v >> 16) & 0x7ff) as u32,
            dsax: ((v >> 32) & 0x7ff) as u32,
            dsay: ((v >> 48) & 0x7ff) as u32,
            dir: ((v >> 59) & 0x3) as u32,
        }
    }
}

/// Decoded `TRXREG` register (`0x52`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TrxReg {
    /// Width of the transfer area, in pixels.
    pub rrw: u32,
    /// Height of the transfer area, in pixels.
    pub rrh: u32,
}

impl TrxReg {
    /// Split a 64-bit register value into its fields.
    pub fn from_u64(v: u64) -> Self {
        Self {
            rrw: (v & 0xfff) as u32,
            rrh: ((v >> 32) & 0xfff) as u32,
        }
    }
}

/// GIF state the interpreter keeps across packets.
///
/// This is deliberately small: enough to complete an ordinary host-to-local
/// image upload. The full GS drawing pipeline is not modelled yet.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GifState {
    /// `PRIM` register (`0x00`); only the low 11 bits are meaningful.
    pub prim: u32,
    /// `BITBLTBUF` (`0x50`).
    pub bitbltbuf: BitbltBuf,
    /// `TRXPOS` (`0x51`).
    pub trxpos: TrxPos,
    /// `TRXREG` (`0x52`).
    pub trxreg: TrxReg,
    /// `TRXDIR.XDIR` (`0x53`); see [`regs::TRXDIR_*`](regs::TRXDIR_HOST_TO_LOCAL).
    pub trxdir: u32,
    /// Bytes already written into VRAM by the active host-to-local transfer.
    pub written: usize,
}

impl GifState {
    /// VRAM byte offset where the next image payload byte belongs.
    ///
    /// The GS stores pixels with a block/interleave swizzle that we do not model
    /// yet; the common linear case reduces to `base + dst_y * stride + dst_x`,
    /// which is what this computes, plus the progress of the active transfer.
    pub fn image_offset(&self) -> usize {
        let bpp = bits_per_pixel(self.bitbltbuf.dpsm) as usize;
        let row_pixels = self.bitbltbuf.dbw as usize * 64;
        let linear = (self.trxpos.dsay as usize * row_pixels + self.trxpos.dsax as usize) * bpp / 8;
        (self.bitbltbuf.dest_base() + linear + self.written) % VRAM_SIZE
    }
}

/// Color depth in bits for a GS pixel-storage format (`PSM`).
///
/// Unknown formats fall back to 32 bits, the most common framebuffer depth.
pub fn bits_per_pixel(psm: u32) -> u32 {
    match psm {
        0x00 | 0x30 => 32,               // PSMCT32 / PSMZ32
        0x01 | 0x31 => 24,               // PSMCT24 / PSMZ24
        0x02 | 0x0a | 0x32 | 0x3a => 16, // PSMCT16(S) / PSMZ16(S)
        0x13 | 0x1b => 8,                // PSMT8 / PSMT8H
        0x14 | 0x24 | 0x2c => 4,         // PSMT4 / PSMT4HL / PSMT4HH
        _ => 32,
    }
}

impl Gs {
    /// Parse a stream of GIF packets from `data`.
    ///
    /// Returns the number of bytes consumed. Parsing stops at the first
    /// truncated packet (reporting everything processed up to it), at a tag
    /// with `NLOOP == 0`, or after a completed packet whose `EOP` is set. An
    /// `EOP` marks the end of the GIF transfer, so bytes after it are not
    /// interpreted.
    pub fn write_gif(&mut self, data: &[u8]) -> usize {
        let mut pos = 0;
        while data.len() - pos >= TAG_SIZE {
            let mut raw = [0u8; TAG_SIZE];
            raw.copy_from_slice(&data[pos..pos + TAG_SIZE]);
            let tag = GifTag::parse(&raw);
            pos += TAG_SIZE;

            if tag.nloop == 0 {
                // "When NLOOP is 0 the GIF does not output anything."
                if tag.eop {
                    break;
                }
                continue;
            }

            if tag.pre && tag.flg == regs::flg::PACKED {
                self.gif.prim = tag.prim & 0x7ff;
            }

            let (consumed, complete) = self.gif_payload(&tag, &data[pos..]);
            pos += consumed;
            if !complete || tag.eop {
                break;
            }
        }
        pos
    }

    /// Apply one tag's payload. Returns `(bytes_consumed, packet_complete)`.
    fn gif_payload(&mut self, tag: &GifTag, data: &[u8]) -> (usize, bool) {
        match tag.flg {
            regs::flg::PACKED => self.gif_packed(tag, data),
            regs::flg::REGLIST => self.gif_reglist(tag, data),
            regs::flg::IMAGE | regs::flg::IMAGE2 => self.gif_image(tag, data),
            _ => (0, true),
        }
    }

    /// `PACKED`: `NLOOP * NREG` 128-bit packed register words.
    fn gif_packed(&mut self, tag: &GifTag, data: &[u8]) -> (usize, bool) {
        let nreg = tag.reg_count() as usize;
        let mut pos = 0;
        for _ in 0..tag.nloop {
            for r in 0..nreg {
                if pos + 16 > data.len() {
                    return (pos, false);
                }
                self.gif_packed_reg(tag.regs[r], &data[pos..pos + 16]);
                pos += 16;
            }
        }
        (pos, true)
    }

    /// `REGLIST`: `NLOOP * NREG` 64-bit register values.
    fn gif_reglist(&mut self, tag: &GifTag, data: &[u8]) -> (usize, bool) {
        let nreg = tag.reg_count() as usize;
        let mut pos = 0;
        for _ in 0..tag.nloop {
            for r in 0..nreg {
                if pos + 8 > data.len() {
                    return (pos, false);
                }
                let value = u64::from_le_bytes(data[pos..pos + 8].try_into().expect("8 bytes"));
                self.gif_reg_write(tag.regs[r], value);
                pos += 8;
            }
        }
        (pos, true)
    }

    /// `IMAGE`: `NLOOP` 128-bit words of raw data for the active transfer.
    fn gif_image(&mut self, tag: &GifTag, data: &[u8]) -> (usize, bool) {
        let want = tag.nloop as usize * 16;
        let take = want.min(data.len());
        if self.gif.trxdir == regs::TRXDIR_HOST_TO_LOCAL {
            self.write_image(&data[..take]);
        }
        (take, take == want)
    }

    /// Copy image bytes into VRAM and advance the transfer cursor.
    fn write_image(&mut self, bytes: &[u8]) {
        let mut off = self.gif.image_offset();
        for &b in bytes {
            self.vram[off % VRAM_SIZE] = b;
            off += 1;
        }
        self.gif.written = self.gif.written.wrapping_add(bytes.len());
    }

    /// Dispatch one 128-bit packed register word.
    fn gif_packed_reg(&mut self, reg: u8, word: &[u8]) {
        match reg {
            regs::PRIM => {
                let lo = u32::from_le_bytes(word[0..4].try_into().expect("4 bytes"));
                self.gif.prim = lo & 0x7ff;
            }
            regs::A_D => {
                // Packed A+D: 64-bit data followed by an 8-bit register address.
                let data = u64::from_le_bytes(word[0..8].try_into().expect("8 bytes"));
                self.gif_reg_write(word[8], data);
            }
            _ => {}
        }
    }

    /// Apply a 64-bit write to a GS general register.
    ///
    /// Unknown registers are ignored. This is the single funnel for both the
    /// `REGLIST` path and the `A+D` path, so it names the registers the crate
    /// actually models.
    pub fn gif_reg_write(&mut self, addr: u8, value: u64) {
        match addr {
            regs::PRIM => self.gif.prim = value as u32 & 0x7ff,
            regs::BITBLTBUF => self.gif.bitbltbuf = BitbltBuf::from_u64(value),
            regs::TRXPOS => self.gif.trxpos = TrxPos::from_u64(value),
            regs::TRXREG => self.gif.trxreg = TrxReg::from_u64(value),
            regs::TRXDIR => {
                self.gif.trxdir = value as u32 & 0x3;
                if self.gif.trxdir == regs::TRXDIR_HOST_TO_LOCAL {
                    // (Re)starting a host-to-local transfer rewinds the cursor.
                    self.gif.written = 0;
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regs;

    /// Build a 128-bit GIFtag from decoded fields, packing `regs` into `REGS`.
    ///
    /// The number of entries in `regs` becomes the `NREG` field, so a 16-entry
    /// slice naturally encodes the hardware's "0 means 16" representation.
    fn make_tag(nloop: u32, eop: bool, pre: bool, prim: u32, flg: u32, regs: &[u8]) -> [u8; 16] {
        let lo: u64 = (nloop as u64 & 0x7fff)
            | ((eop as u64) << 15)
            | ((pre as u64) << 46)
            | ((prim as u64 & 0x7ff) << 47)
            | ((flg as u64 & 0x3) << 58)
            | ((regs.len() as u64 & 0xf) << 60);
        let mut hi: u64 = 0;
        for (i, &r) in regs.iter().enumerate() {
            hi |= (r as u64 & 0xf) << (4 * i);
        }

        let mut out = [0u8; 16];
        out[0..8].copy_from_slice(&lo.to_le_bytes());
        out[8..16].copy_from_slice(&hi.to_le_bytes());
        out
    }

    /// Packed `A+D` word: 64-bit payload followed by the register address.
    fn ad_word(data: u64, addr: u8) -> [u8; 16] {
        let mut w = [0u8; 16];
        w[0..8].copy_from_slice(&data.to_le_bytes());
        w[8] = addr;
        w
    }

    fn push(out: &mut Vec<u8>, bytes: &[u8]) {
        out.extend_from_slice(bytes);
    }

    #[test]
    fn gif_tag_decodes_all_fields() {
        // nloop=3, eop, prim=0x2a, packed, regs=[0x0e, 0x05, 0x0f]
        let tag = GifTag::parse(&make_tag(
            3,
            true,
            false,
            0x2a,
            regs::flg::PACKED,
            &[0x0e, 0x05, 0x0f],
        ));
        assert_eq!(tag.nloop, 3);
        assert!(tag.eop);
        assert!(!tag.pre);
        assert_eq!(tag.prim, 0x2a);
        assert_eq!(tag.flg, regs::flg::PACKED);
        assert_eq!(tag.nreg, 3);
        assert_eq!(tag.reg_count(), 3);
        assert_eq!(tag.regs[0], 0x0e);
        assert_eq!(tag.regs[1], 0x05);
        assert_eq!(tag.regs[2], 0x0f);
        assert_eq!(tag.regs[3], 0);
    }

    #[test]
    fn gif_tag_nreg_zero_means_sixteen() {
        let tag = GifTag::parse(&make_tag(1, true, false, 0, regs::flg::PACKED, &[0u8; 16]));
        assert_eq!(tag.nreg, 0);
        assert_eq!(tag.reg_count(), 16);
    }

    #[test]
    fn reglist_writes_a_general_register() {
        let mut gs = Gs::new();
        let mut stream = Vec::new();
        // REGLIST tag listing PRIM (0x00), one loop, EOP.
        push(
            &mut stream,
            &make_tag(1, true, false, 0, regs::flg::REGLIST, &[regs::PRIM]),
        );
        // PRIM value 0x06 (sprite), sent as a 64-bit register value.
        push(&mut stream, &6u64.to_le_bytes());

        let consumed = gs.write_gif(&stream);
        assert_eq!(consumed, stream.len());
        assert_eq!(gs.gif.prim, 6);
    }

    #[test]
    fn packed_reglist_pre_latches_prim() {
        // A PRE tag latches PRIM before any packed data is processed.
        let mut gs = Gs::new();
        let mut stream = Vec::new();
        push(
            &mut stream,
            &make_tag(1, true, true, 0x03, regs::flg::PACKED, &[regs::NOP]),
        );
        push(&mut stream, &[0u8; 16]); // one NOP packed word

        assert_eq!(gs.write_gif(&stream), stream.len());
        assert_eq!(gs.gif.prim, 0x03);
    }

    #[test]
    fn image_transfer_writes_vram() {
        let mut gs = Gs::new();
        let mut stream = Vec::new();

        // --- Setup packet (PACKED, four A+D writes) ---
        let regs = [regs::A_D; 4];
        push(
            &mut stream,
            &make_tag(1, false, false, 0, regs::flg::PACKED, &regs),
        );

        // BITBLTBUF: DBP=0x10 (byte 0x1000), DBW=1 (64 px), DPSM=0 (CT32).
        let bitbltbuf: u64 = (0x10 << 32) | (1 << 48);
        push(&mut stream, &ad_word(bitbltbuf, regs::BITBLTBUF));
        // TRXPOS: destination (0, 0).
        push(&mut stream, &ad_word(0, regs::TRXPOS));
        // TRXREG: 64x1 pixels.
        push(&mut stream, &ad_word((1 << 32) | 64, regs::TRXREG));
        // TRXDIR: host -> local.
        push(
            &mut stream,
            &ad_word(regs::TRXDIR_HOST_TO_LOCAL as u64, regs::TRXDIR),
        );

        // --- Image packet (16 bytes) ---
        push(
            &mut stream,
            &make_tag(1, true, false, 0, regs::flg::IMAGE, &[]),
        );
        let payload: [u8; 16] = std::array::from_fn(|i| i as u8 + 1);
        push(&mut stream, &payload);

        assert_eq!(gs.write_gif(&stream), stream.len());
        assert_eq!(gs.gif.trxdir, regs::TRXDIR_HOST_TO_LOCAL);
        assert_eq!(gs.gif.written, 16);
        assert_eq!(&gs.vram[0x1000..0x1000 + 16], &payload[..]);
    }

    #[test]
    fn truncated_tag_is_safe() {
        let mut gs = Gs::new();
        // Five bytes cannot hold a tag.
        assert_eq!(gs.write_gif(&[0u8; 5]), 0);
        assert_eq!(gs.gif, GifState::default());
    }

    #[test]
    fn truncated_packed_payload_is_safe() {
        let mut gs = Gs::new();
        let mut stream = Vec::new();
        // Tag promises two NOP words but only one is present.
        push(
            &mut stream,
            &make_tag(
                1,
                true,
                false,
                0,
                regs::flg::PACKED,
                &[regs::NOP, regs::NOP],
            ),
        );
        push(&mut stream, &[0u8; 16]);

        let consumed = gs.write_gif(&stream);
        // The tag and the one available word are reported as consumed.
        assert_eq!(consumed, 32);
    }

    #[test]
    fn truncated_image_payload_is_safe() {
        let mut gs = Gs::new();
        gs.gif.trxdir = regs::TRXDIR_HOST_TO_LOCAL;
        gs.gif.bitbltbuf = BitbltBuf::from_u64((0x10 << 32) | (1 << 48));

        let mut stream = Vec::new();
        // Tag promises two 16-byte words but only ten bytes follow.
        push(
            &mut stream,
            &make_tag(2, true, false, 0, regs::flg::IMAGE, &[]),
        );
        push(&mut stream, &[0x5au8; 10]);

        let consumed = gs.write_gif(&stream);
        assert_eq!(consumed, 16 + 10);
        assert_eq!(gs.gif.written, 10);
        assert!(gs.vram[0x1000..0x1000 + 10].iter().all(|&b| b == 0x5a));
    }

    #[test]
    fn unknown_registers_are_ignored() {
        let mut gs = Gs::new();
        let mut stream = Vec::new();
        push(
            &mut stream,
            &make_tag(1, true, false, 0, regs::flg::REGLIST, &[regs::TEX0_1]),
        );
        push(&mut stream, &0xdead_beef_u64.to_le_bytes());

        assert_eq!(gs.write_gif(&stream), stream.len());
        assert_eq!(gs.gif, GifState::default());
    }

    #[test]
    fn empty_nloop_produces_no_output() {
        let mut gs = Gs::new();
        // NLOOP == 0 is a no-op; parsing continues if EOP is clear.
        let stream = make_tag(0, false, false, 0, regs::flg::PACKED, &[regs::NOP]);
        assert_eq!(gs.write_gif(&stream), stream.len());
        assert_eq!(gs.gif, GifState::default());
    }

    #[test]
    fn bits_per_pixel_covers_known_formats() {
        assert_eq!(bits_per_pixel(0x00), 32);
        assert_eq!(bits_per_pixel(0x01), 24);
        assert_eq!(bits_per_pixel(0x02), 16);
        assert_eq!(bits_per_pixel(0x13), 8);
        assert_eq!(bits_per_pixel(0x14), 4);
    }
}
