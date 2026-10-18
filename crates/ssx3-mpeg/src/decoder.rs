//! Native MPEG-2 video decoding for the game's movies (N5,
//! `docs/plans/native-rendering.md`).
//!
//! Sony's libmpeg decodes on the EE with the IPU: the IPU parses
//! macroblocks (BDEC), libmpeg forms the predictions and stores the
//! pictures, then the IPU converts them to RGB32 macroblocks (CSC). Here
//! the whole picture is decoded natively, with the IPU model's own VLC
//! tables, dequantisation, IDCT and colour conversion (this crate), so
//! the output is the bytes the emulated path produces.
//!
//! What the game's movies use (all 14 `.MPC` files: 512x448, progressive
//! frame pictures, frame prediction and frame DCT only, zigzag scan, linear
//! quantiser, DC precision 0, 4:2:0) is what is implemented; anything else
//! is reported and decoded as far as it goes.
//!
//! The decoder is driven like libmpeg: [`Decoder::picture`] decodes until
//! one picture is ready in display order and asks its [`Feed`] for more
//! bitstream whenever it runs out (libmpeg's "no data" callback).

use crate::{csc, dequantise, idct, luts, tables, DctLookup, Lookup, DEFAULT_INTRA, NON_LINEAR_SCALE};

/// Where more bitstream comes from.
pub trait Feed {
    /// Append more bytes to `out`; false if there are none.
    fn more(&mut self, out: &mut Vec<u8>) -> bool;
}

/// One decoded picture: Y, Cb, Cr planes (4:2:0).
#[derive(Clone)]
pub struct Frame {
    pub y: Vec<u8>,
    pub cb: Vec<u8>,
    pub cr: Vec<u8>,
}

impl Frame {
    fn new(w: usize, h: usize) -> Self {
        Frame { y: vec![0; w * h], cb: vec![0; w * h / 4], cr: vec![0; w * h / 4] }
    }
}

/// What a call to [`Decoder::picture`] ended with.
#[derive(Debug, PartialEq, Eq)]
pub enum Got {
    /// A picture (in display order) is ready: [`Decoder::output`].
    Picture,
    /// The sequence ended (or the bitstream ran out) with nothing to show.
    End,
}

#[derive(Default)]
struct Pic {
    /// 1 I, 2 P, 3 B.
    kind: u32,
    /// f_code[s][t]: s 0 forward, 1 backward; t 0 horizontal, 1 vertical.
    f_code: [[u32; 2]; 2],
    dc_precision: u32,
    intra_vlc: bool,
    q_scale_type: bool,
    alternate_scan: bool,
}

pub struct Decoder {
    data: Vec<u8>,
    /// Read position in bits.
    bit: usize,
    /// The feed had nothing more.
    dry: bool,
    pub width: usize,
    pub height: usize,
    mbw: usize,
    mbh: usize,
    intra_q: [u8; 64],
    nonintra_q: [u8; 64],
    pic: Pic,
    /// The reference pictures: `older` (forward reference of B pictures)
    /// and `newer` (the latest I/P picture).
    older: Option<Frame>,
    newer: Option<Frame>,
    /// `newer` has not been output yet.
    held: bool,
    cur: Frame,
    /// What [`Self::output`] returns: the B picture just decoded, else
    /// `newer`'s predecessor or `newer`.
    out: Option<Frame>,
    pub ended: bool,
    /// Pictures output so far.
    pub pictures: u32,
    /// Problems met (unsupported syntax, bad codes), for a report.
    pub errors: u32,
    /// Debugging: per macroblock of the last decoded picture, its
    /// prediction (address, forward, backward, vectors), when `log_mbs`.
    pub log_mbs: bool,
    pub mb_log: Vec<(usize, bool, bool, [[i32; 2]; 2])>,
}

const DEFAULT_NONINTRA: [u8; 64] = [16; 64];

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder {
    pub fn new() -> Self {
        Decoder {
            data: Vec::new(),
            bit: 0,
            dry: false,
            width: 0,
            height: 0,
            mbw: 0,
            mbh: 0,
            intra_q: DEFAULT_INTRA,
            nonintra_q: DEFAULT_NONINTRA,
            pic: Pic::default(),
            older: None,
            newer: None,
            held: false,
            cur: Frame::new(0, 0),
            out: None,
            ended: false,
            pictures: 0,
            errors: 0,
            log_mbs: false,
            mb_log: Vec::new(),
        }
    }

    /// Back to the start of a stream (libmpeg `sceMpegReset`): the
    /// bitstream and the reference pictures go.
    pub fn reset(&mut self) {
        let (errors, log) = (self.errors, self.log_mbs);
        *self = Self::new();
        self.errors = errors;
        self.log_mbs = log;
    }

    /// Bitstream added outside a feed (`sceMpegAddBs` before decoding).
    pub fn add(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
        self.dry = false;
    }

    // ------------------------------------------------------------ bits

    fn avail(&self) -> usize {
        self.data.len() * 8 - self.bit
    }

    /// At least `n` bits buffered, if the feed can give them.
    #[inline]
    fn need(&mut self, n: usize, feed: &mut dyn Feed) -> bool {
        while self.avail() < n {
            if self.dry || !self.refill(feed) {
                return false;
            }
        }
        true
    }

    #[cold]
    fn refill(&mut self, feed: &mut dyn Feed) -> bool {
        // (Drop what was read, a megabyte at a time.)
        let byte = self.bit / 8;
        if byte >= 1 << 20 {
            self.data.drain(..byte);
            self.bit -= byte * 8;
        }
        let before = self.data.len();
        if !feed.more(&mut self.data) || self.data.len() == before {
            self.dry = true;
            return false;
        }
        true
    }

    /// The next `n` (<= 32) bits, zeros past the end of the data.
    #[inline]
    fn peek(&mut self, n: u32, feed: &mut dyn Feed) -> u32 {
        if n == 0 {
            return 0;
        }
        self.need(n as usize, feed);
        let byte = self.bit / 8;
        let mut w = 0u64;
        for i in 0..8 {
            w = w << 8 | *self.data.get(byte + i).unwrap_or(&0) as u64;
        }
        ((w << (self.bit % 8)) >> (64 - n)) as u32
    }

    #[inline]
    fn skip(&mut self, n: u32) {
        self.bit = (self.bit + n as usize).min(self.data.len() * 8);
    }

    #[inline]
    fn get(&mut self, n: u32, feed: &mut dyn Feed) -> u32 {
        let v = self.peek(n, feed);
        self.skip(n);
        v
    }

    fn vlc(&mut self, l: &Lookup, feed: &mut dyn Feed) -> Option<u32> {
        let i = self.peek(l.bits, feed);
        let (len, v) = l.entries[i as usize];
        if len == 0 {
            return None;
        }
        self.skip(len as u32);
        Some(v)
    }

    /// Byte-align and find the next start code `00 00 01 xx`; the position
    /// stays at it. None when the bitstream ends.
    fn next_start_code(&mut self, feed: &mut dyn Feed) -> Option<u8> {
        self.bit = self.bit.div_ceil(8) * 8;
        loop {
            if !self.need(32, feed) {
                return None;
            }
            let b = self.bit / 8;
            let d = &self.data[b..b + 4];
            if d[0] == 0 && d[1] == 0 && d[2] == 1 {
                return Some(d[3]);
            }
            // (Skip quickly over a run of non-zero bytes.)
            self.bit += if d[2] > 1 { 24 } else if d[1] > 1 { 16 } else { 8 };
        }
    }

    // ---------------------------------------------------------- headers

    fn sequence_header(&mut self, feed: &mut dyn Feed) {
        let w = self.get(12, feed) as usize;
        let h = self.get(12, feed) as usize;
        self.skip(4 + 4); // aspect ratio, frame rate
        self.get(18, feed); // bit rate
        self.skip(1 + 10 + 1); // marker, vbv buffer size, constrained
        if self.get(1, feed) != 0 {
            for k in 0..64 {
                self.intra_q[tables::ZIGZAG[k] as usize] = self.get(8, feed) as u8;
            }
        } else {
            self.intra_q = DEFAULT_INTRA;
        }
        if self.get(1, feed) != 0 {
            for k in 0..64 {
                self.nonintra_q[tables::ZIGZAG[k] as usize] = self.get(8, feed) as u8;
            }
        } else {
            self.nonintra_q = DEFAULT_NONINTRA;
        }
        if (w, h) != (self.width, self.height) {
            self.width = w;
            self.height = h;
            self.mbw = w.div_ceil(16);
            self.mbh = h.div_ceil(16);
            self.older = None;
            self.newer = None;
            self.held = false;
        }
    }

    fn extension(&mut self, feed: &mut dyn Feed) {
        match self.get(4, feed) {
            // Quant matrix extension.
            3 => {
                if self.get(1, feed) != 0 {
                    for k in 0..64 {
                        self.intra_q[tables::ZIGZAG[k] as usize] = self.get(8, feed) as u8;
                    }
                }
                if self.get(1, feed) != 0 {
                    for k in 0..64 {
                        self.nonintra_q[tables::ZIGZAG[k] as usize] = self.get(8, feed) as u8;
                    }
                }
            }
            // Picture coding extension.
            8 => {
                let bits = self.peek(32, feed);
                let mut f = [[0u32; 2]; 2];
                f[0][0] = bits >> 28;
                f[0][1] = bits >> 24 & 15;
                f[1][0] = bits >> 20 & 15;
                f[1][1] = bits >> 16 & 15;
                let p = &mut self.pic;
                p.f_code = f;
                p.dc_precision = bits >> 14 & 3;
                let structure = bits >> 12 & 3;
                let frame_pred_frame_dct = bits >> 10 & 1;
                let concealment = bits >> 9 & 1;
                p.q_scale_type = bits >> 8 & 1 != 0;
                p.intra_vlc = bits >> 7 & 1 != 0;
                p.alternate_scan = bits >> 6 & 1 != 0;
                if structure != 3 || frame_pred_frame_dct == 0 || concealment != 0 {
                    self.errors += 1;
                }
                self.skip(32);
            }
            _ => {}
        }
    }

    fn picture_header(&mut self, feed: &mut dyn Feed) {
        self.skip(10); // temporal reference
        self.pic.kind = self.get(3, feed);
        self.skip(16); // vbv delay
        if self.pic.kind == 2 || self.pic.kind == 3 {
            self.skip(4); // full_pel_forward, forward_f_code (MPEG-1)
        }
        if self.pic.kind == 3 {
            self.skip(4);
        }
        while self.get(1, feed) != 0 {
            self.skip(8);
        }
    }

    // ----------------------------------------------------------- pictures

    /// Decode until a picture is ready in display order. Its RGB32
    /// macroblocks: [`Self::output_rgb32`].
    pub fn picture(&mut self, feed: &mut dyn Feed) -> Got {
        loop {
            let Some(code) = self.next_start_code(feed) else {
                return self.finish();
            };
            self.skip(32);
            match code {
                0xb3 => self.sequence_header(feed),
                0xb5 => self.extension(feed),
                0xb7 => return self.finish(),
                0x00 => {
                    if self.width == 0 {
                        self.errors += 1;
                        continue;
                    }
                    if let Some(g) = self.decode_picture(feed) {
                        return g;
                    }
                }
                _ => {}
            }
        }
    }

    /// The end of the sequence: the held reference picture, else nothing.
    fn finish(&mut self) -> Got {
        self.ended = true;
        if self.held {
            self.held = false;
            self.out = self.newer.clone();
            self.pictures += 1;
            return Got::Picture;
        }
        Got::End
    }

    /// A picture header was read: its extensions and slices. Some when a
    /// picture is ready for output.
    fn decode_picture(&mut self, feed: &mut dyn Feed) -> Option<Got> {
        self.picture_header(feed);
        // Defaults when no picture coding extension follows (MPEG-1).
        self.pic.dc_precision = 0;
        let (w, h) = (self.width, self.height);
        if self.cur.y.len() != w * h {
            self.cur = Frame::new(w, h);
        }
        // Extensions and user data up to the first slice.
        let first = loop {
            let code = self.next_start_code(feed)?;
            match code {
                0xb5 => {
                    self.skip(32);
                    self.extension(feed);
                }
                0x01..=0xaf => break code,
                0xb2 => self.skip(32),
                _ => return None,
            }
        };
        let kind = self.pic.kind;
        if kind == 2 && self.newer.is_none() || kind == 3 && (self.older.is_none() || self.newer.is_none()) {
            // (A predicted picture with no reference: skipped.)
            self.errors += 1;
            return None;
        }
        self.mb_log.clear();
        let mut code = first;
        loop {
            self.skip(32);
            self.slice(code as usize, feed);
            match self.next_start_code(feed) {
                Some(c @ 0x01..=0xaf) => code = c,
                _ => break,
            }
        }
        let pic = std::mem::replace(&mut self.cur, Frame::new(0, 0));
        if kind == 3 {
            let old = self.out.replace(pic);
            self.cur = old.filter(|f| f.y.len() == w * h).unwrap_or_else(|| Frame::new(w, h));
            self.pictures += 1;
            return Some(Got::Picture);
        }
        // A reference picture: the previous one is shown now.
        let prev_held = self.held;
        let recycled = self.older.take();
        self.older = self.newer.take();
        self.newer = Some(pic);
        self.held = true;
        self.cur = recycled.unwrap_or_else(|| Frame::new(w, h));
        if prev_held {
            self.out = self.older.clone();
            self.pictures += 1;
            return Some(Got::Picture);
        }
        None
    }

    /// The picture to show (after [`Got::Picture`]) as RGB32 macroblocks
    /// (`0xAABBGGRR`, 16x16 each, row-major) in libmpeg's order: column by
    /// column (top to bottom, then left to right), as its CSC delivers.
    pub fn output_rgb32(&self, out: &mut [u8]) {
        let Some(f) = &self.out else { return };
        let (w, mbh) = (self.width, self.mbh);
        for mb in 0..self.mbw * self.mbh {
            let (mx, my) = (mb / mbh, mb % mbh);
            let mut raw: csc::Raw8 = [0; 384];
            for y in 0..16 {
                let row = (my * 16 + y) * w + mx * 16;
                raw[y * 16..y * 16 + 16].copy_from_slice(&f.y[row..row + 16]);
            }
            for y in 0..8 {
                let row = (my * 8 + y) * (w / 2) + mx * 8;
                raw[256 + y * 8..256 + y * 8 + 8].copy_from_slice(&f.cb[row..row + 8]);
                raw[320 + y * 8..320 + y * 8 + 8].copy_from_slice(&f.cr[row..row + 8]);
            }
            let rgb = csc::ycbcr_to_rgb32(&raw, 0, 0, false);
            let Some(dst) = out.get_mut(mb * 1024..mb * 1024 + 1024) else { return };
            for (i, p) in rgb.iter().enumerate() {
                dst[i * 4..i * 4 + 4].copy_from_slice(&p.to_le_bytes());
            }
        }
    }

    // ------------------------------------------------------------ slices

    fn slice(&mut self, vpos: usize, feed: &mut dyn Feed) {
        let mut qsc = self.get(5, feed);
        if self.peek(1, feed) != 0 {
            self.skip(1 + 1 + 7);
            while self.peek(1, feed) != 0 {
                self.skip(9);
            }
        }
        self.skip(1);
        let reset_dc = 128 << self.pic.dc_precision;
        let mut dc = [reset_dc; 3];
        let mut pmv = [[0i32; 2]; 2];
        // The previous macroblock's prediction directions (B skips).
        let mut prev_dirs = (false, false);
        let mut addr: isize = ((vpos - 1) * self.mbw) as isize - 1;
        let total = (self.mbw * self.mbh) as isize;
        let mut first = true;
        loop {
            // Address increment (with escapes).
            let mut inc = 0isize;
            loop {
                match self.vlc(&luts().mbai, feed) {
                    Some(v) => match v & 0xffff {
                        0x23 => inc += 33,
                        0x22 => {}
                        n => {
                            inc += n as isize;
                            break;
                        }
                    },
                    None => {
                        self.errors += 1;
                        return;
                    }
                }
            }
            if !first {
                // Skipped macroblocks.
                for _ in 1..inc {
                    addr += 1;
                    if addr >= total {
                        return;
                    }
                    dc = [reset_dc; 3];
                    match self.pic.kind {
                        2 => {
                            pmv = [[0; 2]; 2];
                            self.predict(addr as usize, (true, false), &pmv);
                        }
                        3 => self.predict(addr as usize, prev_dirs, &pmv),
                        _ => self.errors += 1,
                    }
                }
                addr += 1;
            } else {
                addr += inc;
                first = false;
            }
            if addr < 0 || addr >= total {
                self.errors += 1;
                return;
            }
            let table = match self.pic.kind {
                1 => &luts().mb_i,
                2 => &luts().mb_p,
                _ => &luts().mb_b,
            };
            let Some(t) = self.vlc(table, feed) else {
                self.errors += 1;
                return;
            };
            let (intra, pattern, bwd, fwd, quant) = (t & 1 != 0, t & 2 != 0, t & 4 != 0, t & 8 != 0, t & 16 != 0);
            if quant {
                qsc = self.get(5, feed);
            }
            if fwd {
                self.motion_vector(0, &mut pmv[0], feed);
            }
            if bwd {
                self.motion_vector(1, &mut pmv[1], feed);
            }
            let mb = addr as usize;
            if intra {
                pmv = [[0; 2]; 2];
                let mut blocks = [[0i16; 64]; 6];
                for (b, blk) in blocks.iter_mut().enumerate() {
                    if !self.block(blk, Some((&mut dc[b.saturating_sub(3)], b >= 4)), qsc, feed) {
                        return;
                    }
                }
                self.store(mb, &blocks, 0x3f, false);
            } else {
                dc = [reset_dc; 3];
                // A P macroblock without motion: zero vector, forward.
                let dirs = if self.pic.kind == 2 && !fwd {
                    pmv = [[0; 2]; 2];
                    (true, false)
                } else {
                    (fwd, bwd)
                };
                prev_dirs = dirs;
                self.predict(mb, dirs, &pmv);
                let cbp = if pattern {
                    match self.vlc(&luts().cbp, feed) {
                        Some(v) => v & 0x3f,
                        None => {
                            self.errors += 1;
                            return;
                        }
                    }
                } else {
                    0
                };
                if cbp != 0 {
                    let mut blocks = [[0i16; 64]; 6];
                    for (b, blk) in blocks.iter_mut().enumerate() {
                        if cbp & (32 >> b) != 0 && !self.block(blk, None, qsc, feed) {
                            return;
                        }
                    }
                    self.store(mb, &blocks, cbp, true);
                }
            }
            // The slice ends where 23 zero bits follow.
            if self.peek(23, feed) == 0 {
                return;
            }
        }
    }

    /// The prediction of macroblock `mb` into the picture being decoded:
    /// forward and/or backward (frame prediction, half-pel), averaged when
    /// both.
    fn predict(&mut self, mb: usize, (fwd, bwd): (bool, bool), pmv: &[[i32; 2]; 2]) {
        if self.log_mbs {
            self.mb_log.push((mb, fwd, bwd, *pmv));
        }
        let (w, h, mbw) = (self.width, self.height, self.mbw);
        let (mx, my) = (mb % mbw, mb / mbw);
        let fwd_ref = if self.pic.kind == 2 { self.newer.as_ref() } else { self.older.as_ref() };
        let bwd_ref = self.newer.as_ref();
        let mut parts: Vec<(&Frame, [i32; 2])> = Vec::with_capacity(2);
        if fwd {
            match fwd_ref {
                Some(f) => parts.push((f, pmv[0])),
                None => self.errors += 1,
            }
        }
        if bwd {
            match bwd_ref {
                Some(f) => parts.push((f, pmv[1])),
                None => self.errors += 1,
            }
        }
        if parts.is_empty() {
            return;
        }
        let cur = &mut self.cur;
        // Luma 16x16.
        let mut acc = [[0u16; 256]; 2];
        for (k, (f, mv)) in parts.iter().enumerate() {
            pred(&f.y, w, h, mx * 16, my * 16, mv[0], mv[1], 16, &mut acc[k]);
        }
        put(&mut cur.y, w, mx * 16, my * 16, 16, &acc, parts.len());
        // Chroma 8x8: the luma vector halved (towards zero).
        let (cw, ch) = (w / 2, h / 2);
        for plane in 0..2 {
            let mut acc = [[0u16; 256]; 2];
            for (k, (f, mv)) in parts.iter().enumerate() {
                let src = if plane == 0 { &f.cb } else { &f.cr };
                pred(src, cw, ch, mx * 8, my * 8, mv[0] / 2, mv[1] / 2, 8, &mut acc[k]);
            }
            let dst = if plane == 0 { &mut cur.cb } else { &mut cur.cr };
            put(dst, cw, mx * 8, my * 8, 8, &acc, parts.len());
        }
    }

    /// The residual blocks of macroblock `mb` (`cbp`: which are coded):
    /// stored (intra) or added to the prediction, saturated to 0..255.
    fn store(&mut self, mb: usize, blocks: &[[i16; 64]; 6], cbp: u32, add: bool) {
        let (w, mbw) = (self.width, self.mbw);
        let (mx, my) = (mb % mbw, mb / mbw);
        for (b, blk) in blocks.iter().enumerate() {
            if cbp & (32 >> b) == 0 {
                continue;
            }
            let (plane, stride, x0, y0) = match b {
                0..=3 => (&mut self.cur.y, w, mx * 16 + (b & 1) * 8, my * 16 + (b >> 1) * 8),
                4 => (&mut self.cur.cb, w / 2, mx * 8, my * 8),
                _ => (&mut self.cur.cr, w / 2, mx * 8, my * 8),
            };
            for y in 0..8 {
                let row = &mut plane[(y0 + y) * stride + x0..(y0 + y) * stride + x0 + 8];
                for (x, p) in row.iter_mut().enumerate() {
                    let r = blk[y * 8 + x] as i32;
                    *p = if add { (*p as i32 + r).clamp(0, 255) } else { r.clamp(0, 255) } as u8;
                }
            }
        }
    }

    /// One motion vector (frame prediction): `pmv` updated in place.
    fn motion_vector(&mut self, s: usize, pmv: &mut [i32; 2], feed: &mut dyn Feed) {
        for (t, p) in pmv.iter_mut().enumerate() {
            let f_code = self.pic.f_code[s][t].max(1);
            let code = match self.vlc(&luts().motion, feed) {
                Some(v) => (v & 0xffff) as i16 as i32,
                None => {
                    self.errors += 1;
                    0
                }
            };
            let r_size = f_code - 1;
            let delta = if f_code == 1 || code == 0 {
                code
            } else {
                let residual = self.get(r_size, feed) as i32;
                let d = ((code.abs() - 1) << r_size) + residual + 1;
                if code < 0 {
                    -d
                } else {
                    d
                }
            };
            let f = 1i32 << r_size;
            let (high, low, range) = (16 * f - 1, -16 * f, 32 * f);
            let mut v = *p + delta;
            if v > high {
                v -= range;
            }
            if v < low {
                v += range;
            }
            *p = v;
        }
    }

    /// One 8x8 block: coefficients, dequantisation, IDCT. `dc`: the intra
    /// DC predictor of the block's component (None: non-intra).
    fn block(&mut self, out: &mut [i16; 64], dc: Option<(&mut i32, bool)>, qsc: u32, feed: &mut dyn Feed) -> bool {
        let intra = dc.is_some();
        let mut coef = [0i32; 64];
        let mut index = 0usize;
        let table_one = intra && self.pic.intra_vlc;
        let l: &DctLookup = if table_one { &luts().dct1 } else { &luts().dct0 };
        if let Some((dc, chroma)) = dc {
            let size = match self.vlc(if chroma { &luts().dc_chroma } else { &luts().dc_luma }, feed) {
                Some(v) => v & 0xffff,
                None => {
                    self.errors += 1;
                    return false;
                }
            };
            let diff = if size == 0 {
                0
            } else {
                let v = self.get(size, feed) as i32;
                if v < 1 << (size - 1) {
                    v + 1 - (1 << size)
                } else {
                    v
                }
            };
            *dc += diff;
            coef[0] = *dc;
            index = 1;
        } else if self.peek(1, feed) != 0 {
            coef[0] = if self.get(2, feed) & 1 != 0 { -1 } else { 1 };
            index = 1;
        }
        loop {
            if index > 0 {
                let eob = if table_one { self.peek(4, feed) == 0b0110 } else { self.peek(2, feed) == 0b10 };
                if eob {
                    self.skip(if table_one { 4 } else { 2 });
                    break;
                }
            }
            let i = self.peek(l.bits, feed);
            let (len, run, level) = l.entries[i as usize];
            if len == 0 {
                self.errors += 1;
                return false;
            }
            self.skip(len as u32);
            let (run, level) = if run == tables::ESCAPE {
                let run = self.get(6, feed) as usize;
                let v = self.get(12, feed) as i32;
                (run, if v & 0x800 != 0 { v - 0x1000 } else { v })
            } else {
                let level = level as i32;
                (run as usize, if self.get(1, feed) != 0 { -level } else { level })
            };
            index += run;
            if index >= 64 {
                self.errors += 1;
                return false;
            }
            coef[index] = level;
            index += 1;
        }
        let scan = if self.pic.alternate_scan { &tables::ALTERNATE } else { &tables::ZIGZAG };
        let weights = if intra { &self.intra_q } else { &self.nonintra_q };
        let scale = if self.pic.q_scale_type { NON_LINEAR_SCALE[(qsc & 31) as usize] as i32 } else { qsc as i32 * 2 };
        let f = dequantise(&coef, scan, weights, intra, scale, 8 >> self.pic.dc_precision, false);
        idct::idct(&f, out);
        true
    }
}

/// One `n`x`n` block predicted from `src` (a `w`x`h` plane) at (`x`, `y`)
/// moved by the half-pel vector (`mvx`, `mvy`), into `out` (row-major) at
/// four times the pixel scale (rounded by [`put`]).
#[allow(clippy::too_many_arguments)]
fn pred(src: &[u8], w: usize, h: usize, x: usize, y: usize, mvx: i32, mvy: i32, n: usize, out: &mut [u16; 256]) {
    let (hx, hy) = ((mvx & 1) as usize, (mvy & 1) as usize);
    let sx = (x as i32 + (mvx >> 1)).clamp(0, (w - n - hx) as i32) as usize;
    let sy = (y as i32 + (mvy >> 1)).clamp(0, (h - n - hy) as i32) as usize;
    let at = |xx: usize, yy: usize| src[yy * w + xx] as u16;
    for j in 0..n {
        for i in 0..n {
            let (px, py) = (sx + i, sy + j);
            out[j * n + i] = match (hx, hy) {
                (0, 0) => 4 * at(px, py),
                (1, 0) => 2 * (at(px, py) + at(px + 1, py)),
                (0, 1) => 2 * (at(px, py) + at(px, py + 1)),
                _ => at(px, py) + at(px + 1, py) + at(px, py + 1) + at(px + 1, py + 1),
            };
        }
    }
}

/// The prediction (one part, or the average of two) into the `n`x`n`
/// block at (`x`, `y`) of `dst`, rounded as MPEG-2 specifies: each part to
/// the nearest (half-pel averages rounded up), then their average rounded
/// up. (The emulated path, libmpeg over the IPU model, differs by 1-2 in a
/// few bidirectional macroblocks whose backward vector is diagonal; its
/// arithmetic is the same, so the difference is in what it fetches.)
fn put(dst: &mut [u8], w: usize, x: usize, y: usize, n: usize, acc: &[[u16; 256]; 2], parts: usize) {
    for j in 0..n {
        let row = &mut dst[(y + j) * w + x..(y + j) * w + x + n];
        for (i, p) in row.iter_mut().enumerate() {
            let k = j * n + i;
            let f = (acc[0][k] + 2) >> 2;
            *p = if parts == 2 { (f + ((acc[1][k] + 2) >> 2) + 1) >> 1 } else { f } as u8;
        }
    }
}
