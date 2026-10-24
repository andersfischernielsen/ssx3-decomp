//! EA's IOP software voice mixer (MIX / SFILTER in SNDDRV), used for banks
//! that live in IOP RAM (`zboard.bnk`). Spec: `docs/investigation/phase3-sound.md` §8.
//!
//! Literal port of:
//! - `MIX_create` (0x30fc8), `MIX_setresamplequality` (0x3117c),
//!   `MIX_playinit` (0x3118c), `MIX_play` (0x3134c), `MIX_stop` (0x31408),
//!   `MIXI_stop` (0x313d0), `SNDMIX_setdrygain` (0x31528),
//!   `SNDMIX_setwetgain` (0x31568), `MIX_audioslice` (0x315a8),
//!   `MIX_getframe` (0x319f0), `MIX_setpitch` (0x31a50);
//! - `MIXI_interpolateto0` (0x30970), `MIXI_interpolatemix` (0x30ba8),
//!   `SNDMIXI_volramp` (0x30dcc), `mixiop` (0x336d0);
//! - `SFILTER_rs` (0x33280), `SFILTER_rsinit` (0x3364c),
//!   `SFILTER_rssetpitch` (0x33260), `SFILTERI_rs16linear` (0x33120),
//!   `SFILTERI_rs16nearest` (0x33090);
//! - decoders: `SFILTER_unpackf` (0x31f8c) / `SFILTER_unpacklf` (0x320ec)
//!   with `SNDI_decode8to32` / `SNDI_decode16to32`, `SFILTER_unpackxaf`
//!   (0x39390), `SFILTER_unpackxalf` (0x398fc), `SFILTER_unpackxalfsection`
//!   (0x39620), `unpackxafresidual` (0x39ce0), `decxa16c` (0x39ba0).
//!
//! Not ported: IOP effects (`MIX_initreverb`, `SNDMIXI_initfx`, the SFILTER
//! effect graph). The fx enable flag (`0x4174f`) is therefore always 0: wet
//! gains are stored and the wet declick residual accumulates, but nothing is
//! mixed to a wet bus, exactly as in the original with fx off.
//!
//! ## Filter chain
//! A voice's chain is the resampler node (priority 0xa0) followed by the
//! decoder node (0xf0), as built by `SFILTER_add`. The head is called with
//! `(n, scratch = 0x417a4, out = 0x417a8)`; the resampler calls the decoder
//! with its input buffer as the decoder's output. The two global scratch
//! buffers are kept here (`buf_a`, `buf_b`) and persist between calls, as in
//! the original.
//!
//! ## Call sequence and `MIX_play`
//! `SNDIOP_updateiopvoices` starts a voice with `MIX_playinit`, `MIX_setpitch`,
//! `SNDMIX_setdrygain` (each channel), `SNDMIX_setwetgain`, `MIX_play`. The
//! driver here calls [`IopMixer::play`] (= `MIX_playinit`), then
//! [`IopMixer::set_pitch`], [`IopMixer::set_dry`], [`IopMixer::set_wet`]. So
//! `play` leaves `MIX_play` pending, and it runs at the end of the next
//! `set_wet` (the last call of that sequence) or, failing that, at the start
//! of the next [`IopMixer::slice`]. The result is identical to the original
//! sequence: the voice starts with current gains = target gains, no ramp.

/// Size of the scratch buffers (the original allocates 0x20bc bytes each,
/// 64-byte aligned; anything big enough for 256 frames at pitch 4.0 works).
const SCRATCH: usize = 4096;
/// Max output channels (`+4` dry gains are 6 words: 0x4..0x1c).
const MAX_CH: usize = 6;
/// `MIXI_interpolateto0` weights.
pub const DECLICK_W: [i32; 16] = [
    0x77, 0x70, 0x68, 0x61, 0x59, 0x52, 0x4a, 0x43, 0x3b, 0x34, 0x2c, 0x25, 0x1d, 0x16, 0x0e, 0x07,
];
/// `xafp` (0x3acc0): EA-XA predictor pairs (c1, c2). Only 4 entries exist in
/// the image; indices 4..15 would read the BSS words after it (SNDabortfile,
/// ...) at runtime and are taken as 0 here (never used by valid EA-XA data).
const XAFP: [[i32; 2]; 16] = {
    let mut t = [[0i32; 2]; 16];
    t[1] = [0xf0, 0];
    t[2] = [0x1cc, -0xd0];
    t[3] = [0x188, -0xdc];
    t
};

// ---------------------------------------------------------------------------
// IOP RAM access (addresses are masked to the 2 MiB IOP RAM; reads outside
// the slice return 0) and bounds-tolerant buffer writes.

#[inline]
fn rd8(ram: &[u8], a: u32) -> u8 {
    ram.get((a & 0x1f_ffff) as usize).copied().unwrap_or(0)
}
#[inline]
fn rd16(ram: &[u8], a: u32) -> i16 {
    i16::from_le_bytes([rd8(ram, a), rd8(ram, a.wrapping_add(1))])
}
#[inline]
fn rd32(ram: &[u8], a: u32) -> u32 {
    u32::from_le_bytes([
        rd8(ram, a),
        rd8(ram, a.wrapping_add(1)),
        rd8(ram, a.wrapping_add(2)),
        rd8(ram, a.wrapping_add(3)),
    ])
}
#[inline]
fn put(buf: &mut [i32], i: usize, v: i32) {
    if let Some(x) = buf.get_mut(i) {
        *x = v;
    }
}
#[inline]
fn get(buf: &[i32], i: usize) -> i32 {
    buf.get(i).copied().unwrap_or(0)
}

/// `SNDI_decode8to32`: `out[i] = (s8)src[i] << 8`.
fn decode8to32(n: u32, src: u32, ram: &[u8], out: &mut [i32]) {
    for i in 0..n.min(out.len() as u32) {
        out[i as usize] = (rd8(ram, src.wrapping_add(i)) as i8 as i32) << 8;
    }
}
/// `SNDI_decode16to32`: `out[i] = (s16)src[i]`.
fn decode16to32(n: u32, src: u32, ram: &[u8], out: &mut [i32]) {
    for i in 0..n.min(out.len() as u32) {
        out[i as usize] = rd16(ram, src.wrapping_add(i * 2)) as i32;
    }
}

// ---------------------------------------------------------------------------
// EA-XA

/// `decxa16c` (0x39ba0) state block (node +0x1c..+0x2c): `count` remaining
/// (goes <= 0; whole 28-sample blocks are always decoded, so it overshoots),
/// predictor history `p1` (+0x20, newest) / `p2` (+0x24), source byte
/// address, destination index.
fn decxa16c(
    count: &mut i32,
    p1: &mut i32,
    p2: &mut i32,
    src: &mut u32,
    ram: &[u8],
    out: &mut [i32],
    dst: &mut usize,
) {
    while *count > 0 {
        *count -= 0x1c;
        let b = rd8(ram, *src);
        *src = src.wrapping_add(1);
        let [c1, c2] = XAFP[(b >> 4) as usize];
        let shift = ((b & 0xf) + 8) as u32;
        for _ in 0..14 {
            let byte = rd8(ram, *src) as u32;
            let s0 = ((((byte >> 4) << 28) as i32) >> shift)
                .wrapping_add(p1.wrapping_mul(c1).wrapping_add(p2.wrapping_mul(c2)))
                >> 8;
            put(out, *dst, s0);
            *p2 = s0;
            let s1 = (((byte << 28) as i32) >> shift)
                .wrapping_add(s0.wrapping_mul(c1).wrapping_add(p1.wrapping_mul(c2)))
                >> 8;
            put(out, *dst + 1, s1);
            *p1 = s1;
            *dst += 2;
            *src = src.wrapping_add(1);
        }
    }
}

/// Byte address of the block holding frame `pos`: `base + pos*15/28`.
#[inline]
fn xa_src(base: u32, pos: i32) -> u32 {
    base.wrapping_add((pos.wrapping_mul(0xf) / 0x1c) as u32)
}

/// `unpackxafresidual` (0x39ce0): copy `min(cnt, n)` residual samples to
/// `out[dst..]`, shift the rest down, return the number copied.
fn unpackxafresidual(cnt: i32, buf: &mut [i32], n: i32, out: &mut [i32], dst: &mut usize) -> i32 {
    let m = if cnt < n { cnt } else { n };
    let mut i = 0i32;
    while i < m {
        put(out, *dst, get(buf, i as usize));
        *dst += 1;
        i += 1;
    }
    let mut k = 0usize;
    while i < cnt {
        let v = get(buf, i as usize);
        put(buf, k, v);
        k += 1;
        i += 1;
    }
    m
}

/// One-shot EA-XA node (`SFILTER_unpackxafinit`, 0xb0 bytes).
#[derive(Clone, Debug)]
struct XaF {
    p1: i32,
    p2: i32,
    base: u32,        // +0x30
    total: i32,       // +0x34
    pos: i32,         // +0x38 (frames decoded so far, the reported frame)
    resid_n: i32,     // +0x3c
    resid: [i32; 28], // +0x40
}

impl XaF {
    /// `SFILTER_unpackxaf` (0x39390).
    fn process(&mut self, n: i32, ram: &[u8], out: &mut [i32]) -> i32 {
        if self.pos >= self.total {
            return -1;
        }
        let mut d = 0usize;
        if self.resid_n < n {
            for i in 0..self.resid_n.max(0) as usize {
                put(out, d, self.resid[i]);
                d += 1;
            }
            let mut avail = self.total - self.pos;
            let n2 = n - self.resid_n;
            if avail < 0 {
                avail = 0;
            }
            let take = if n2 < avail { n2 } else { avail };
            let mut count = take;
            let mut src = xa_src(self.base, self.pos);
            decxa16c(
                &mut count,
                &mut self.p1,
                &mut self.p2,
                &mut src,
                ram,
                out,
                &mut d,
            );
            let newpos = self.pos + take - count;
            self.pos = newpos;
            if avail == take {
                // End reached: zero-fill the rest of the request (after the
                // decoded overshoot), keep returning 1 this time.
                let mut z = (n2 - take) - (newpos - self.total);
                while z > 0 {
                    put(out, d, 0);
                    d += 1;
                    z -= 1;
                }
                return 1;
            }
            // Keep the overshoot of the last block for the next call.
            let mut k = 0usize;
            let mut i = count;
            while i < 0 {
                self.resid[k] = get(out, (d as isize + i as isize) as usize);
                k += 1;
                i += 1;
            }
            self.resid_n = k as i32;
        } else {
            let mut i = 0usize;
            while (i as i32) < n {
                put(out, d, self.resid[i]);
                d += 1;
                i += 1;
            }
            let mut k = 0usize;
            while (i as i32) < self.resid_n {
                self.resid[k] = self.resid[i];
                k += 1;
                i += 1;
            }
            self.resid_n = k as i32;
        }
        1
    }
}

/// Looped EA-XA node (`SFILTER_unpackxalfinit`, 0x138 bytes).
#[derive(Clone, Debug)]
struct XaLF {
    /// The node's process function: false = `SFILTER_unpackxalf` (lead-in up
    /// to the loop start), true = `SFILTER_unpackxalfsection` (the loop).
    section: bool,
    p1: i32,
    p2: i32,
    base: u32,        // +0x30
    pos: i32,         // +0x38
    loop_start: i32,  // +0x3c
    loop_end: i32,    // +0x40 (inclusive)
    resid_n: i32,     // +0x44
    saved_p1: i32,    // +0x48 (decoder state at the loop start block)
    saved_p2: i32,    // +0x4c
    resid: [i32; 56], // +0x58
}

impl XaLF {
    fn process(&mut self, n: i32, ram: &[u8], out: &mut [i32]) -> i32 {
        if self.section {
            self.section_fn(n, ram, out, 0)
        } else {
            self.lead_in(n, ram, out)
        }
    }

    /// `SFILTER_unpackxalf` (0x398fc).
    fn lead_in(&mut self, n: i32, ram: &[u8], out: &mut [i32]) -> i32 {
        let mut d = 0usize;
        if self.resid_n < n {
            for i in 0..self.resid_n.max(0) as usize {
                put(out, d, self.resid[i]);
                d += 1;
            }
            let n2 = n - self.resid_n;
            let mut take = (self.loop_start - self.pos) - 0x1b;
            if n2 < take {
                take = n2;
            }
            if take < 0 {
                take = 0;
            }
            let mut count = take;
            let mut src = xa_src(self.base, self.pos);
            decxa16c(
                &mut count,
                &mut self.p1,
                &mut self.p2,
                &mut src,
                ram,
                out,
                &mut d,
            );
            self.pos = self.pos + take - count;
            let mut k = 0usize;
            let mut i = count;
            while i < 0 {
                self.resid[k] = get(out, (d as isize + i as isize) as usize);
                k += 1;
                i += 1;
            }
            self.resid_n = k as i32;
            d -= k;
            if self.loop_start - self.pos < 0x1c {
                self.section = true;
                self.saved_p1 = self.p1;
                self.saved_p2 = self.p2;
                if n2 - take > 0 {
                    self.section_fn(n2 - take, ram, out, d);
                }
            }
        } else {
            let mut i = 0usize;
            while (i as i32) < n {
                put(out, d, self.resid[i]);
                d += 1;
                i += 1;
            }
            let mut k = 0usize;
            while (i as i32) < self.resid_n {
                self.resid[k] = self.resid[i];
                k += 1;
                i += 1;
            }
            self.resid_n = k as i32;
        }
        1
    }

    /// `SFILTER_unpackxalfsection` (0x39620): output starts at `out[d]`.
    fn section_fn(&mut self, mut n: i32, ram: &[u8], out: &mut [i32], mut d: usize) -> i32 {
        while n > 0 {
            if self.resid_n == 0 {
                if self.pos + 0x1b < self.loop_end {
                    let mut count = 0x1c;
                    let mut src = xa_src(self.base, self.pos);
                    decxa16c(
                        &mut count,
                        &mut self.p1,
                        &mut self.p2,
                        &mut src,
                        ram,
                        out,
                        &mut d,
                    );
                    n -= 0x1c;
                    self.pos += 0x1c;
                } else {
                    // Block holding the loop end: decode into the residual,
                    // keep the samples up to loop_end, then jump back.
                    let mut count = 0x1c;
                    let mut src = xa_src(self.base, self.pos);
                    let mut rd = 0usize;
                    decxa16c(
                        &mut count,
                        &mut self.p1,
                        &mut self.p2,
                        &mut src,
                        ram,
                        &mut self.resid,
                        &mut rd,
                    );
                    let ls = self.loop_start;
                    self.pos += 0x1c;
                    self.resid_n = 0x1d - (self.pos - self.loop_end);
                    self.p1 = self.saved_p1;
                    self.p2 = self.saved_p2;
                    self.pos = (ls / 0x1c) * 0x1c;
                    if ls != self.pos {
                        let mut stack = [0i32; 0x1c];
                        let mut count = 0x1c;
                        let mut src = xa_src(self.base, self.pos);
                        let mut sd = 0usize;
                        decxa16c(
                            &mut count,
                            &mut self.p1,
                            &mut self.p2,
                            &mut src,
                            ram,
                            &mut stack,
                            &mut sd,
                        );
                        self.pos += 0x1c;
                        let k = self.pos - ls;
                        for i in 0..k.max(0) {
                            let v = get(&stack, (0x1c - k + i) as usize);
                            put(&mut self.resid, self.resid_n as usize, v);
                            self.resid_n += 1;
                        }
                    }
                }
            } else {
                let c = unpackxafresidual(self.resid_n, &mut self.resid, n, out, &mut d);
                n -= c;
                self.resid_n -= c;
            }
        }
        // Overshoot of the last full block becomes the residual.
        let mut d = d as isize + n as isize;
        while n < 0 {
            let v = get(out, d as usize);
            put(&mut self.resid, self.resid_n as usize, v);
            d += 1;
            self.resid_n += 1;
            n += 1;
        }
        1
    }
}

// ---------------------------------------------------------------------------
// PCM

/// One-shot PCM node (`SFILTER_unpackfinit`, 0x2c bytes).
#[derive(Clone, Debug)]
struct UnpackF {
    base: u32,  // +0x1c
    pos: u32,   // +0x20
    count: u32, // +0x24
    is16: bool, // +0x28
}

impl UnpackF {
    /// `SFILTER_unpackf` (0x31f8c). The final partial request is zero-filled
    /// (returns 1); the call after that returns -1. `pos` advances by the
    /// full request even past the end.
    fn process(&mut self, n: i32, ram: &[u8], out: &mut [i32]) -> i32 {
        let p = self.pos;
        if p >= self.count {
            return -1;
        }
        let n = n as u32;
        self.pos = p.wrapping_add(n);
        let k = if p.wrapping_add(n) < self.count {
            n
        } else {
            self.count - p
        };
        if self.is16 {
            decode16to32(k, self.base.wrapping_add(p.wrapping_mul(2)), ram, out);
        } else {
            decode8to32(k, self.base.wrapping_add(p), ram, out);
        }
        for i in k..n {
            put(out, i as usize, 0);
        }
        1
    }
}

/// Looped PCM node (`SFILTER_unpacklfinit`, 0x30 bytes).
#[derive(Clone, Debug)]
struct UnpackLF {
    base: u32,       // +0x1c
    pos: u32,        // +0x20
    loop_start: u32, // +0x24
    loop_end: u32,   // +0x28 (inclusive)
    is16: bool,      // +0x2c
}

impl UnpackLF {
    /// `SFILTER_unpacklf` (0x320ec): wraps from `loop_end + 1` to
    /// `loop_start`, always returns 1.
    fn process(&mut self, mut n: i32, ram: &[u8], out: &mut [i32]) -> i32 {
        let mut d = 0usize;
        while n > 0 {
            let mut c = (self.loop_end as i32)
                .wrapping_sub(self.pos as i32)
                .wrapping_add(1);
            if n < c {
                c = n;
            }
            if c <= 0 {
                // Malformed header (loop start > loop end): the original
                // would decode a negative count. Not reachable with game
                // data; fill silence instead of hanging.
                for i in 0..n as usize {
                    put(out, d + i, 0);
                }
                break;
            }
            let o = out.get_mut(d..).unwrap_or(&mut []);
            if self.is16 {
                decode16to32(
                    c as u32,
                    self.base.wrapping_add(self.pos.wrapping_mul(2)),
                    ram,
                    o,
                );
            } else {
                decode8to32(c as u32, self.base.wrapping_add(self.pos), ram, o);
            }
            d += c as usize;
            n -= c;
            self.pos = self.pos.wrapping_add(c as u32);
            if self.loop_end < self.pos {
                self.pos = self.loop_start;
            }
        }
        1
    }
}

#[derive(Clone, Debug)]
enum Decoder {
    Unpack(UnpackF),
    UnpackLoop(UnpackLF),
    Xa(XaF),
    XaLoop(Box<XaLF>),
}

impl Decoder {
    fn process(&mut self, n: i32, ram: &[u8], out: &mut [i32]) -> i32 {
        match self {
            Decoder::Unpack(d) => d.process(n, ram, out),
            Decoder::UnpackLoop(d) => d.process(n, ram, out),
            Decoder::Xa(d) => d.process(n, ram, out),
            Decoder::XaLoop(d) => d.process(n, ram, out),
        }
    }
    /// The node's getframe function: frames consumed so far.
    fn frame(&self) -> i32 {
        match self {
            Decoder::Unpack(d) => d.pos as i32,
            Decoder::UnpackLoop(d) => d.pos as i32,
            Decoder::Xa(d) => d.pos,
            Decoder::XaLoop(d) => d.pos,
        }
    }
}

// ---------------------------------------------------------------------------
// Resampler node

/// `SFILTER_rs` node (0x40 bytes). `+0x35` ("pre") is always 0 after
/// `SFILTER_rsinit` and is folded out.
#[derive(Clone, Debug)]
struct Rs {
    p1: i32,        // +0x20 pitch (16.16)
    p2: i32,        // +0x24
    p3: i32,        // +0x28
    p4: i32,        // +0x2c
    phase: u32,     // +0x30
    primed: bool,   // +0x34
    nhist: usize,   // +0x36: 2 linear, 1 nearest
    hist: [i32; 2], // +0x38
    linear: bool,   // +0x1c
}

impl Rs {
    /// `SFILTER_rsinit` (0x3364c).
    fn new(linear: bool) -> Self {
        Rs {
            p1: 0,
            p2: 0,
            p3: 0,
            p4: 0,
            phase: 0,
            primed: false,
            nhist: if linear { 2 } else { 1 },
            hist: [0; 2],
            linear,
        }
    }

    /// `SFILTER_rssetpitch` (0x33260).
    fn set_pitch(&mut self, p: i32) {
        self.p2 = p.wrapping_mul(2);
        self.p1 = p;
        self.p3 = p.wrapping_mul(3);
        self.p4 = p << 2;
    }

    /// `SFILTERI_rs16linear` (0x33120), 4 outputs per step.
    fn rs16linear(&mut self, n: i32, inp: &[i32], out: &mut [i32]) {
        let mut o = 0i32;
        while o < n {
            let ph = self.phase;
            for (k, step) in [0, self.p1, self.p2, self.p3].into_iter().enumerate() {
                let q = ph.wrapping_add(step as u32);
                let i = (q >> 16) as usize;
                let a = get(inp, i);
                let b = get(inp, i + 1);
                let v = (b.wrapping_sub(a).wrapping_mul((q & 0xffff) as i32) >> 16).wrapping_add(a);
                put(out, o as usize + k, v);
            }
            self.phase = self.phase.wrapping_add(self.p4 as u32);
            o += 4;
        }
    }

    /// `SFILTERI_rs16nearest` (0x33090), 4 outputs per step. (The original is
    /// a do-while that needs `n % 4 == 0`; n is always 16, 240 or 256.)
    fn rs16nearest(&mut self, n: i32, inp: &[i32], out: &mut [i32]) {
        let mut ph = self.phase;
        let mut o = 0i32;
        loop {
            put(out, o as usize, get(inp, (ph >> 16) as usize));
            put(
                out,
                o as usize + 1,
                get(inp, (ph.wrapping_add(self.p1 as u32) >> 16) as usize),
            );
            put(
                out,
                o as usize + 2,
                get(inp, (ph.wrapping_add(self.p2 as u32) >> 16) as usize),
            );
            put(
                out,
                o as usize + 3,
                get(inp, (ph.wrapping_add(self.p3 as u32) >> 16) as usize),
            );
            ph = ph.wrapping_add(self.p4 as u32);
            o += 4;
            if o >= n {
                break;
            }
        }
        self.phase = ph;
    }

    /// `SFILTER_rs` (0x33280): `n` output frames into `b` (param_4) using
    /// `a` (param_3) as the input buffer; `dec` is the next node.
    fn process(
        &mut self,
        dec: Option<&mut Decoder>,
        n: i32,
        ram: &[u8],
        a: &mut [i32],
        b: &mut [i32],
    ) -> i32 {
        let h = self.nhist;
        if self.p1 == 0x10000 {
            // Pass-through, with the same h-sample delay as the history.
            let (n2, off) = if !self.primed {
                (n + h as i32, 0usize)
            } else {
                b[..h].copy_from_slice(&self.hist[..h]);
                (n, h)
            };
            let mut r = 1;
            if let Some(d) = dec {
                r = d.process(n2, ram, &mut b[off..]);
            }
            if r < 1 {
                self.primed = true;
                self.hist = [0; 2];
                return r;
            }
            self.primed = true;
            // Unprimed: b[n + i]; primed: (b + h)[n + i - h] = b[n + i].
            for i in 0..h {
                self.hist[i] = get(b, n as usize + i);
            }
            n
        } else {
            let mut need = (self.p1.wrapping_mul(n).wrapping_add(self.phase as i32) as u32) >> 16;
            let mut off = 0usize;
            if !self.primed {
                need = need.wrapping_add(h as u32);
            } else {
                a[..h].copy_from_slice(&self.hist[..h]);
                off = h;
            }
            if (need as i32) > 0 {
                if let Some(d) = dec {
                    // The decoder writes into our input buffer; our output
                    // buffer is its scratch.
                    let r = d.process(need as i32, ram, &mut a[off..]);
                    if r < 1 {
                        self.primed = true;
                        self.hist = [0; 2];
                        return r;
                    }
                }
            }
            self.primed = true;
            if self.linear {
                self.rs16linear(n, a, b);
            } else {
                self.rs16nearest(n, a, b);
            }
            let ip = ((self.phase >> 16) & 0xffff) as usize;
            for i in 0..h {
                self.hist[i] = get(a, ip + i);
            }
            self.phase &= 0xffff;
            n
        }
    }
}

// ---------------------------------------------------------------------------
// Mixer

/// One MIX voice record (0x50 bytes in the original).
#[derive(Clone, Debug, Default)]
struct Voice {
    /// +0: 0 idle, 1 initialised (`MIX_playinit`), 2 playing (`MIX_play`).
    state: u8,
    /// +1: gains changed since the last slice.
    dirty: bool,
    cur_dry: [i32; MAX_CH], // +0x04
    tgt_dry: [i32; MAX_CH], // +0x1c
    cur_wet: i32,           // +0x34
    tgt_wet: i32,           // +0x38
    /// +0x3c: last output sample of the chain (for the stop declick).
    last: i32,
    /// Chain membership (+0x40 list). The node objects below survive
    /// `MIX_stop` (the original frees them but keeps +0x48/+0x4c/+0x44).
    chain_dec: bool,
    chain_rs: bool,
    dec: Option<Decoder>, // +0x48 (+0x44 getframe is `dec.is_some()`)
    rs: Option<Rs>,       // +0x4c
    /// SNDDRV's `b328[v]`: set by `play`, cleared by `stop` and by the
    /// end-of-sample callback (`SNDIOP_mixvoicefree`).
    active: bool,
    /// `MIX_play` still to run (see the module docs).
    pending_play: bool,
}

/// The IOP software mixer (`MIX_create` state).
#[derive(Clone, Debug)]
pub struct IopMixer {
    voices: Vec<Voice>,
    /// Output channels (`DAT_41744` byte 1 = channels + aux).
    channels: usize,
    /// `0x4174e` resample quality (> 0x18 → linear).
    quality: u8,
    /// `0x417c4[ch]`: dry declick residuals.
    resid: [i32; MAX_CH],
    /// `0x417dc`: wet declick residual (accumulates, never applied: fx off).
    wet_resid: i32,
    buf_a: Vec<i32>, // 0x417a4
    buf_b: Vec<i32>, // 0x417a8
}

impl IopMixer {
    /// `MIX_create` (0x30fc8) with `voices` voices and `channels` output
    /// buses (at most 6).
    pub fn new(voices: usize, channels: usize) -> Self {
        IopMixer {
            voices: vec![Voice::default(); voices],
            channels: channels.min(MAX_CH),
            quality: 0,
            resid: [0; MAX_CH],
            wet_resid: 0,
            buf_a: vec![0; SCRATCH],
            buf_b: vec![0; SCRATCH],
        }
    }

    /// `MIX_setresamplequality` (0x3117c). Read when a voice's resampler
    /// node is created (first `set_pitch` after `play`).
    pub fn set_quality(&mut self, q: u8) {
        self.quality = q;
    }

    /// `MIX_playinit` (0x3118c) as called by `SNDIOP_updateiopvoices` for the
    /// sample header at IOP address `header_addr` (§8.2: +1 codec, +4 loop
    /// start, +8 loop end (> 0 → looped), +0xC frame count, +0x10 data).
    /// Marks the voice active; `MIX_play` follows (see module docs).
    /// Unknown codecs get no decoder node (the original indexes outside its
    /// init table there).
    pub fn play(&mut self, v: usize, header_addr: u32, ram: &[u8]) {
        let codec = rd8(ram, header_addr.wrapping_add(1));
        let loop_start = rd32(ram, header_addr.wrapping_add(4));
        let loop_end = rd32(ram, header_addr.wrapping_add(8));
        let count = rd32(ram, header_addr.wrapping_add(0xc));
        let data = header_addr.wrapping_add(0x10);
        let looped = (loop_end as i32) > 0;
        let vo = &mut self.voices[v];
        vo.chain_dec = false;
        vo.chain_rs = false;
        vo.dec = None;
        vo.rs = None;
        let dec = match (codec, looped) {
            (8 | 9, false) => Some(Decoder::Unpack(UnpackF {
                base: data,
                pos: 0,
                count,
                is16: codec == 8,
            })),
            (8 | 9, true) => Some(Decoder::UnpackLoop(UnpackLF {
                base: data,
                pos: 0,
                loop_start,
                loop_end,
                is16: codec == 8,
            })),
            (10, false) => Some(Decoder::Xa(XaF {
                p1: 0,
                p2: 0,
                base: data,
                total: count as i32,
                pos: 0,
                resid_n: 0,
                resid: [0; 28],
            })),
            (10, true) => Some(Decoder::XaLoop(Box::new(XaLF {
                section: false,
                p1: 0,
                p2: 0,
                base: data,
                pos: 0,
                loop_start: loop_start as i32,
                loop_end: loop_end as i32,
                resid_n: 0,
                saved_p1: 0,
                saved_p2: 0,
                resid: [0; 56],
            }))),
            _ => None,
        };
        vo.chain_dec = dec.is_some();
        vo.dec = dec;
        vo.state = 1;
        vo.active = true;
        vo.pending_play = true;
    }

    /// `MIX_play` (0x3134c): current gains = targets, no ramp, state 2.
    fn mix_play(&mut self, v: usize) {
        let ch = self.channels;
        let vo = &mut self.voices[v];
        vo.pending_play = false;
        vo.last = 0;
        vo.dirty = false;
        vo.cur_wet = vo.tgt_wet;
        vo.cur_dry[..ch].copy_from_slice(&vo.tgt_dry[..ch]);
        vo.state = 2;
    }

    /// `MIX_stop` (0x31408): adds the declick residual
    /// `((cur_gain >> 8) * last) >> 7` per bus, frees the chain, state 0.
    /// Also clears the active flag (the driver's `b328 = 0`).
    pub fn stop(&mut self, v: usize) {
        let ch = self.channels;
        let vo = &mut self.voices[v];
        if vo.pending_play {
            // MIX_play always precedes a stop in the original sequence.
            vo.pending_play = false;
            vo.last = 0;
            vo.cur_wet = vo.tgt_wet;
            vo.cur_dry[..ch].copy_from_slice(&vo.tgt_dry[..ch]);
        }
        self.wet_resid = self
            .wet_resid
            .wrapping_add((vo.cur_wet >> 8).wrapping_mul(vo.last) >> 7);
        for k in 0..ch {
            self.resid[k] =
                self.resid[k].wrapping_add((vo.cur_dry[k] >> 8).wrapping_mul(vo.last) >> 7);
        }
        vo.chain_dec = false;
        vo.chain_rs = false;
        vo.state = 0;
        vo.active = false;
    }

    /// `MIX_setpitch` (0x31a50): `step` is 16.16 (the driver passes
    /// `pitch << 4`), clamped to 0x40000. Creates the resampler node on first
    /// use after `play` (linear when quality > 0x18, else nearest).
    pub fn set_pitch(&mut self, v: usize, step: u32) {
        let mut p = step as i32;
        if 0x40000 < p {
            p = 0x40000;
        }
        let linear = self.quality > 0x18;
        let vo = &mut self.voices[v];
        if vo.rs.is_none() {
            vo.rs = Some(Rs::new(linear));
            vo.chain_rs = true;
        }
        vo.rs.as_mut().unwrap().set_pitch(p);
    }

    /// `SNDMIX_setdrygain` (0x31528).
    pub fn set_dry(&mut self, v: usize, ch: usize, gain: i32) {
        let vo = &mut self.voices[v];
        if ch < MAX_CH {
            vo.tgt_dry[ch] = gain;
        }
        vo.dirty = true;
    }

    /// `SNDMIX_setwetgain` (0x31568), then the pending `MIX_play` if `play`
    /// started this voice.
    pub fn set_wet(&mut self, v: usize, gain: i32) {
        let vo = &mut self.voices[v];
        vo.tgt_wet = gain;
        vo.dirty = true;
        if vo.pending_play {
            self.mix_play(v);
        }
    }

    /// The driver's `b328[v]`.
    pub fn is_active(&self, v: usize) -> bool {
        self.voices[v].active
    }

    /// `MIX_getframe` (0x319f0): the decoder's frames consumed so far
    /// (runs ahead of playback), -1 without a decoder. The driver reports
    /// `max(frame, 0)` while active.
    pub fn frame(&self, v: usize) -> i32 {
        match &self.voices[v].dec {
            Some(d) => d.frame(),
            None => -1,
        }
    }

    /// Pull `n` frames through voice `v`'s chain into `buf_b`.
    fn pull(&mut self, v: usize, n: i32, ram: &[u8]) -> i32 {
        let vo = &mut self.voices[v];
        let dec = if vo.chain_dec { vo.dec.as_mut() } else { None };
        if vo.chain_rs {
            vo.rs
                .as_mut()
                .unwrap()
                .process(dec, n, ram, &mut self.buf_a, &mut self.buf_b)
        } else if let Some(d) = dec {
            d.process(n, ram, &mut self.buf_b)
        } else {
            // Empty chain: the original would call through a null node.
            -1
        }
    }

    /// `SNDMIXI_volramp` (0x30dcc): 16 frames with a gain ramp.
    fn volramp(&mut self, v: usize, ram: &[u8], buses: &mut [Vec<i32>]) -> usize {
        let ch = self.channels;
        let r = self.pull(v, 0x10, ram);
        let vo = &mut self.voices[v];
        if r < 1 {
            vo.cur_wet = vo.tgt_wet;
            vo.cur_dry[..ch].copy_from_slice(&vo.tgt_dry[..ch]);
            return 0;
        }
        vo.last = self.buf_b[15];
        // (wet ramp only with fx enabled: skipped, cur_wet left as is)
        for k in 0..ch {
            let cur = vo.cur_dry[k];
            let tgt = vo.tgt_dry[k];
            let Some(bus) = buses.get_mut(k) else {
                continue;
            };
            if cur == tgt {
                if cur != 0 {
                    mixiop(16, cur, &self.buf_b, bus);
                }
            } else {
                interpolatemix(cur, tgt, &self.buf_b, bus);
                vo.cur_dry[k] = tgt;
            }
        }
        16
    }

    /// `MIX_audioslice` (0x315a8): clears `frames` ints of each bus, adds the
    /// pending declick residuals, then mixes every playing voice
    /// (`bus += s * (gain >> 8)`). A voice whose chain ends is stopped
    /// (`MIXI_stop`: `MIX_stop` + `SNDIOP_mixvoicefree`, so `is_active`
    /// becomes false).
    pub fn slice(&mut self, ram: &[u8], buses: &mut [Vec<i32>], frames: usize) {
        for v in 0..self.voices.len() {
            if self.voices[v].pending_play {
                self.mix_play(v);
            }
        }
        for k in 0..self.channels {
            let Some(bus) = buses.get_mut(k) else {
                continue;
            };
            let m = frames.min(bus.len());
            bus[..m].iter_mut().for_each(|x| *x = 0);
            if self.resid[k] != 0 {
                interpolateto0(&mut self.resid[k], bus);
            }
        }
        for v in 0..self.voices.len() {
            if self.voices[v].state != 2 {
                continue;
            }
            let mut off = 0usize;
            let mut n = frames as i32;
            if self.voices[v].dirty {
                off = self.volramp(v, ram, buses);
                self.voices[v].dirty = false;
                n = frames as i32 - off as i32;
            }
            if n == 0 {
                continue;
            }
            let r = self.pull(v, n, ram);
            if r < 0 {
                self.stop(v);
            } else if r != 0 {
                let vo = &mut self.voices[v];
                vo.last = get(&self.buf_b, r as usize - 1);
                for k in 0..self.channels {
                    let g = vo.tgt_dry[k];
                    if g != 0 {
                        if let Some(bus) = buses.get_mut(k) {
                            if let Some(dst) = bus.get_mut(off..) {
                                mixiop(r as u32, g, &self.buf_b, dst);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// `mixiop` (0x336d0): `dst[i] += src[i] * (gain >> 8)` for `count` samples
/// (the count is split as `count & 3` then `count & 0xfffc`).
fn mixiop(count: u32, gain: i32, src: &[i32], dst: &mut [i32]) {
    let g = gain >> 8;
    let n = ((count & 3) + (count & 0xfffc)) as usize;
    for i in 0..n.min(dst.len()) {
        dst[i] = dst[i].wrapping_add(get(src, i).wrapping_mul(g));
    }
}

/// `MIXI_interpolatemix` (0x30ba8): 16 samples with gains
/// `g0 + (i+1) * ((g1 - g0) / 17)`.
fn interpolatemix(g0: i32, g1: i32, src: &[i32], dst: &mut [i32]) {
    let d = g1.wrapping_sub(g0) / 0x11;
    let mut g = g0;
    for i in 0..16.min(dst.len()) {
        g = g.wrapping_add(d);
        dst[i] = dst[i].wrapping_add(get(src, i).wrapping_mul(g >> 8));
    }
}

/// `MIXI_interpolateto0` (0x30970): `dst[i] += r * w[i]`, then `r = 0`.
fn interpolateto0(r: &mut i32, dst: &mut [i32]) {
    for i in 0..16.min(dst.len()) {
        dst[i] = dst[i].wrapping_add(r.wrapping_mul(DECLICK_W[i]));
    }
    *r = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    const HDR: u32 = 0x1000;

    fn ram_with(codec: u8, loop_start: u32, loop_end: u32, count: u32, data: &[u8]) -> Vec<u8> {
        let mut ram = vec![0u8; 0x10000];
        let h = HDR as usize;
        ram[h + 1] = codec;
        ram[h + 4..h + 8].copy_from_slice(&loop_start.to_le_bytes());
        ram[h + 8..h + 12].copy_from_slice(&loop_end.to_le_bytes());
        ram[h + 12..h + 16].copy_from_slice(&count.to_le_bytes());
        ram[h + 16..h + 16 + data.len()].copy_from_slice(data);
        ram
    }

    fn pcm16(v: &[i16]) -> Vec<u8> {
        v.iter().flat_map(|x| x.to_le_bytes()).collect()
    }

    fn start(m: &mut IopMixer, v: usize, ram: &[u8], step: u32, dry: [i32; 2]) {
        m.play(v, HDR, ram);
        m.set_pitch(v, step);
        m.set_dry(v, 0, dry[0]);
        m.set_dry(v, 1, dry[1]);
        m.set_wet(v, 0);
    }

    fn buses() -> [Vec<i32>; 2] {
        [vec![0; 256], vec![0; 256]]
    }

    #[test]
    fn pcm16_one_shot_to_end_passthrough() {
        // 300 frames of value 100, pitch 1.0 (pass-through, 2-sample delay).
        let ram = ram_with(8, 0, 0, 300, &pcm16(&[100; 300]));
        let mut m = IopMixer::new(8, 2);
        m.set_quality(0x32);
        start(&mut m, 0, &ram, 0x10000, [0x100 * 2, 0]);
        let mut b = buses();
        m.slice(&ram, &mut b, 256);
        // Unprimed: decodes 258 frames, outputs 256 of them.
        assert!(b[0].iter().all(|&x| x == 200));
        assert!(b[1].iter().all(|&x| x == 0));
        assert_eq!(m.frame(0), 258);
        assert!(m.is_active(0));
        // Second slice: 2 history samples + request 256 > 42 left: 42 decoded,
        // the rest zero-filled; frame = 514.
        m.slice(&ram, &mut b, 256);
        assert!(b[0][..44].iter().all(|&x| x == 200));
        assert!(b[0][44..].iter().all(|&x| x == 0));
        assert_eq!(m.frame(0), 514);
        assert!(m.is_active(0));
        // Third slice: decoder returns -1, voice stops. last was 0 → no declick.
        m.slice(&ram, &mut b, 256);
        assert!(!m.is_active(0));
        assert!(b[0].iter().all(|&x| x == 0));
    }

    #[test]
    fn pcm8_looped_wraps() {
        // 8-bit, frames 0..6 = 1..=6, loop 2..=5.
        let data: Vec<u8> = (1..=6).collect();
        let ram = ram_with(9, 2, 5, 6, &data);
        let mut d = Decoder::UnpackLoop(UnpackLF {
            base: HDR + 16,
            pos: 0,
            loop_start: 2,
            loop_end: 5,
            is16: false,
        });
        let mut out = vec![0i32; 16];
        assert_eq!(d.process(10, &ram, &mut out), 1);
        let want: Vec<i32> = [1, 2, 3, 4, 5, 6, 3, 4, 5, 6]
            .iter()
            .map(|x| x << 8)
            .collect();
        assert_eq!(&out[..10], &want[..]);
        assert_eq!(d.frame(), 2); // wrapped twice: 6 -> 2
                                  // Through the mixer: header loop end > 0 selects the looped node.
        let mut m = IopMixer::new(1, 2);
        m.play(0, HDR, &ram);
        assert!(matches!(m.voices[0].dec, Some(Decoder::UnpackLoop(_))));
    }

    #[test]
    fn unpackf_end_semantics() {
        let ram = ram_with(8, 0, 0, 5, &pcm16(&[1, 2, 3, 4, 5]));
        let mut d = UnpackF {
            base: HDR + 16,
            pos: 0,
            count: 5,
            is16: true,
        };
        let mut out = vec![9i32; 8];
        assert_eq!(d.process(3, &ram, &mut out), 1);
        assert_eq!(&out[..3], &[1, 2, 3]);
        assert_eq!(d.process(4, &ram, &mut out), 1);
        assert_eq!(&out[..4], &[4, 5, 0, 0]);
        assert_eq!(d.pos, 7);
        assert_eq!(d.process(4, &ram, &mut out), -1);
    }

    #[test]
    fn linear_resampler_half_speed() {
        // Ramp 0, 10, 20, ... at pitch 0.5.
        let v: Vec<i16> = (0..400).map(|i| (i * 10) as i16).collect();
        let ram = ram_with(8, 0, 0, 400, &pcm16(&v));
        let mut m = IopMixer::new(1, 2);
        m.set_quality(0x19);
        start(&mut m, 0, &ram, 0x8000, [0x100, 0x100]);
        let mut b = buses();
        m.slice(&ram, &mut b, 256);
        // need = (0x8000*256)>>16 + 2 = 130 frames consumed.
        assert_eq!(m.frame(0), 130);
        // y[k] = s[k/2] + (s[k/2+1]-s[k/2]) * frac >> 16; gain>>8 = 1.
        for k in 0..256 {
            assert_eq!(b[0][k], (k as i32) * 5, "k={k}");
        }
        // Continuity across slices (history carries the next samples).
        m.slice(&ram, &mut b, 256);
        for k in 0..256 {
            assert_eq!(b[1][k], (256 + k as i32) * 5, "k={k}");
        }
        assert_eq!(m.frame(0), 258);
    }

    #[test]
    fn nearest_resampler_when_quality_low() {
        let v: Vec<i16> = (0..400).map(|i| i as i16).collect();
        let ram = ram_with(8, 0, 0, 400, &pcm16(&v));
        let mut m = IopMixer::new(1, 1);
        m.set_quality(0x18);
        start(&mut m, 0, &ram, 0x18000, [0x100, 0]);
        assert!(!m.voices[0].rs.as_ref().unwrap().linear);
        let mut b = buses();
        m.slice(&ram, &mut b, 256);
        for k in 0..256 {
            assert_eq!(b[0][k], ((k as u32 * 0x18000) >> 16) as i32);
        }
        assert_eq!(m.frame(0), 384 + 1);
    }

    #[test]
    fn gain_ramp_values() {
        let ram = ram_with(8, 0, 0, 2000, &pcm16(&[1000; 2000]));
        let mut m = IopMixer::new(1, 2);
        start(&mut m, 0, &ram, 0x10000, [0, 0]);
        let mut b = buses();
        m.slice(&ram, &mut b, 256); // silent, gains 0
        assert!(b[0].iter().all(|&x| x == 0));
        m.set_dry(0, 0, 0x7fff);
        m.set_dry(0, 1, 0); // unchanged, stays silent
        m.slice(&ram, &mut b, 256);
        let d = 0x7fff / 17; // 1927
        for i in 0..16 {
            assert_eq!(b[0][i], 1000 * (((i as i32 + 1) * d) >> 8), "i={i}");
        }
        assert!(b[0][16..].iter().all(|&x| x == 1000 * (0x7fff >> 8)));
        assert!(b[1].iter().all(|&x| x == 0));
        assert_eq!(m.voices[0].cur_dry[0], 0x7fff);
    }

    #[test]
    fn declick_residual_on_stop() {
        let ram = ram_with(8, 0, 0, 2000, &pcm16(&[1000; 2000]));
        let mut m = IopMixer::new(1, 2);
        start(&mut m, 0, &ram, 0x10000, [0x4000, 0]);
        let mut b = buses();
        m.slice(&ram, &mut b, 256);
        assert_eq!(m.voices[0].last, 1000);
        m.stop(0);
        // r = ((0x4000 >> 8) * 1000) >> 7 = 500
        assert_eq!(m.resid[0], 500);
        m.slice(&ram, &mut b, 256);
        for i in 0..16 {
            assert_eq!(b[0][i], 500 * DECLICK_W[i]);
        }
        assert!(b[0][16..].iter().all(|&x| x == 0));
        assert_eq!(m.resid[0], 0);
        assert!(!m.is_active(0));
    }

    /// Reference EA-XA decoder written from the format, not the port.
    fn xa_ref(block: &[u8], p1: &mut i32, p2: &mut i32) -> Vec<i32> {
        let coef = [[0, 0], [240, 0], [460, -208], [392, -220]];
        let (c1, c2) = (
            coef[(block[0] >> 4) as usize][0],
            coef[(block[0] >> 4) as usize][1],
        );
        let sh = (block[0] & 15) as u32 + 8;
        let mut v = vec![];
        for &by in &block[1..15] {
            for nib in [by >> 4, by & 15] {
                let n = ((nib as i32) << 28) >> sh;
                let s = (n + *p1 * c1 + *p2 * c2) >> 8;
                *p2 = *p1;
                *p1 = s;
                v.push(s);
            }
        }
        v
    }

    fn xa_blocks(nblocks: usize) -> Vec<u8> {
        let mut seed = 7u32;
        let mut d = vec![];
        for b in 0..nblocks {
            d.push(((b as u8 % 4) << 4) | 3);
            for _ in 0..14 {
                seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12345);
                d.push((seed >> 16) as u8);
            }
        }
        d
    }

    #[test]
    fn xa_known_block() {
        // One block, coef 2 (460, -208), shift 12; nibbles 7,-8,1,0,...
        let mut blk = vec![0x24u8, 0x78, 0x10];
        blk.extend_from_slice(&[0; 12]);
        let (mut p1, mut p2, mut src) = (0, 0, 0u32);
        let mut count = 28;
        let mut out = vec![0; 28];
        let mut d = 0;
        decxa16c(
            &mut count, &mut p1, &mut p2, &mut src, &blk, &mut out, &mut d,
        );
        assert_eq!(count, 0);
        assert_eq!(d, 28);
        // s0 = (7<<28 >> 12) >> 8 = 0x70000 >> 8 = 1792
        assert_eq!(out[0], 1792);
        // s1 = ((-8<<28)>>12 + 1792*460) >> 8 = (-524288 + 824320) >> 8 = 1172
        assert_eq!(out[1], 1172);
        // s2 = (65536 + 1172*460 - 1792*208) >> 8
        assert_eq!(out[2], (65536 + 1172 * 460 - 1792 * 208) >> 8);
        let (mut q1, mut q2) = (0, 0);
        assert_eq!(out, xa_ref(&blk, &mut q1, &mut q2));
        assert_eq!((p1, p2), (out[27], out[26]));
    }

    #[test]
    fn xa_one_shot_matches_reference_and_ends() {
        let data = xa_blocks(4); // 112 frames; sample says 100
        let ram = ram_with(10, 0, 0, 100, &data);
        let (mut p1, mut p2) = (0, 0);
        let mut want = vec![];
        for b in 0..4 {
            want.extend(xa_ref(&data[b * 15..b * 15 + 15], &mut p1, &mut p2));
        }
        let mut d = XaF {
            p1: 0,
            p2: 0,
            base: HDR + 16,
            total: 100,
            pos: 0,
            resid_n: 0,
            resid: [0; 28],
        };
        let mut got = vec![];
        let mut out = vec![0; 64];
        // 10 frames: one block decoded, 18 kept as residual, frame = 28.
        assert_eq!(d.process(10, &ram, &mut out), 1);
        got.extend_from_slice(&out[..10]);
        assert_eq!((d.pos, d.resid_n), (28, 18));
        // 5 frames: from the residual only.
        assert_eq!(d.process(5, &ram, &mut out), 1);
        got.extend_from_slice(&out[..5]);
        assert_eq!((d.pos, d.resid_n), (28, 13));
        assert_eq!(d.process(40, &ram, &mut out), 1);
        got.extend_from_slice(&out[..40]);
        assert_eq!(&got[..], &want[..55]);
        // 13 from the residual + 27 decoded (one block, 1 frame of overshoot kept).
        assert_eq!((d.pos, d.resid_n), (56, 1));
        // 50 more: residual 1, avail 44 -> two blocks (56 frames). The
        // overshoot past the 100-frame end covers the request (z < 1), so
        // nothing is zero-filled and frames 100..105 come from the block.
        assert_eq!(d.process(50, &ram, &mut out), 1);
        assert_eq!(&out[..50], &want[55..105]);
        assert_eq!(d.pos, 112);
        assert_eq!(d.process(50, &ram, &mut out), -1);
    }

    #[test]
    fn xa_looped_matches_reference() {
        // 6 blocks = 168 frames; loop 40..=150.
        let data = xa_blocks(6);
        let ram = ram_with(10, 40, 150, 168, &data);
        let decode_from = |start_block: usize, p1: &mut i32, p2: &mut i32| -> Vec<i32> {
            let mut v = vec![];
            for b in start_block..6 {
                v.extend(xa_ref(&data[b * 15..b * 15 + 15], p1, p2));
            }
            v
        };
        let (mut p1, mut p2) = (0, 0);
        let lin = decode_from(0, &mut p1, &mut p2);
        // Expected stream: 0..=150, then 40..=150 again (the loop restarts the
        // predictor from the state saved at block 1 = frame 28), ...
        let mut want: Vec<i32> = lin[..151].to_vec();
        // State at the start of block 1:
        let (mut s1, mut s2) = (0, 0);
        xa_ref(&data[0..15], &mut s1, &mut s2);
        let (mut r1, mut r2) = (s1, s2);
        let again = decode_from(1, &mut r1, &mut r2);
        for _ in 0..3 {
            want.extend_from_slice(&again[40 - 28..151 - 28]);
        }
        let mut m = IopMixer::new(1, 1);
        m.play(0, HDR, &ram);
        let mut got = vec![];
        let mut out = vec![0; 512];
        let dec = m.voices[0].dec.as_mut().unwrap();
        for n in [7, 33, 64, 3, 100, 61, 90, 28, 1, 29] {
            assert_eq!(dec.process(n, &ram, &mut out), 1);
            got.extend_from_slice(&out[..n as usize]);
        }
        assert_eq!(&got[..], &want[..got.len()]);
    }

    #[test]
    fn rs_pass_through_ignores_quality_and_ramp_from_play_is_off() {
        let ram = ram_with(8, 0, 0, 2000, &pcm16(&[64; 2000]));
        let mut m = IopMixer::new(2, 2);
        start(&mut m, 1, &ram, 0x10000, [0x100, 0x200]);
        assert!(!m.voices[1].dirty);
        assert_eq!(m.voices[1].state, 2);
        let mut b = buses();
        m.slice(&ram, &mut b, 256);
        assert!(b[0].iter().all(|&x| x == 64));
        assert!(b[1].iter().all(|&x| x == 128));
        assert_eq!(m.frame(0), -1);
        assert!(!m.is_active(0));
    }
}
