//! # What the two Bezier terrain programs share (types 1 and 2)
//!
//! VU1 programs f4babac5 ([`crate::mat1`]) and 9790369f ([`crate::mat2`])
//! draw the same kind of geometry the same way: a grid of patch points is
//! transformed vertex by vertex into triangle strips, every triangle that
//! leaves the guard band gets ADC and goes through the same near/guard-band
//! clipper, and the clipped polygons are kicked at once as fans. The data
//! memory layout of that machinery is common to both programs; this module
//! holds it, with the PS2 float helpers and the VU memory accessor.
//!
//! The clipper here is not mat3's (`crate::mat3`, 0x1630): it uses other
//! buffers, interpolates from the inside end of an edge (the "leaving" case
//! from `a`, the "entering" case from `b`) and keeps the source ST's z/w on
//! the leaving side, so the last bits differ. It is identical in the two
//! Bezier programs (f4babac5 0x1cc8, 9790369f 0x36d0, same code).

use ssx3_vu::vu::{fast, float};

/// A VU vector register: four PS2 floats (or integers) as raw bits, x..w.
pub type V = [u32; 4];

pub const ONE: u32 = 0x3f80_0000;
/// `vf0` (always (0, 0, 0, 1)).
pub const VF0: V = [0, 0, 0, ONE];

// ---------------------------------------------------------------------------
// Memory layout (qword addresses), common to both programs
// ---------------------------------------------------------------------------

/// M: model → guard-clip matrix columns.
pub const MATRIX: u16 = 0;
/// guardScale (gw/2, -gh/2, -8388467.5, F scale).
pub const GUARD_SCALE: u16 = 4;
/// guardOffset (cx+2048, cy+2048, 8388467.5, F offset).
pub const GUARD_OFFSET: u16 = 5;
/// The constant vertex colour (128, 128, 128, 128).
pub const COLOUR: u16 = 6;
/// GIF tags (only .yzw are used): strip ctx1, strip ctx2, fan ctx1, fan ctx2.
pub const STRIP_TAG_CTX1: u16 = 7;
pub const STRIP_TAG_CTX2: u16 = 8;
pub const FAN_TAG_CTX1: u16 = 9;
pub const FAN_TAG_CTX2: u16 = 10;
/// .x: the fan tag the clipper used last (0x1c4 / 0x1e0, alternating).
pub const FAN_TOGGLE: u16 = 41;
/// The two strip packets: GIF tag, then {STQ, RGBA, XYZF2} per vertex.
pub const PACKET_A: u16 = 0x60;
pub const PACKET_B: u16 = 0x121;
/// Clipper polygon buffers (up to 9 vertices × {pos, ST, RGBA}).
pub const CLIP_BUF_A: u16 = 0x2a;
pub const CLIP_BUF_B: u16 = 0x45;
/// The two fan packets (GIF tag + up to 9 vertices × 3 qw).
pub const FAN_TAG_A: u16 = 0x1c4;
pub const FAN_TAG_B: u16 = 0x1e0;
/// .x: TOP of the last draw call (the positions later passes re-use).
pub const LAST_TOP: u16 = 510;
/// Saved return addresses of the subroutines (.x, and .y/.w in type 2).
pub const RETURN_SAVE: u16 = 511;

/// The NLOOP|EOP word of a strip packet tag.
pub const EOP: u16 = 0x8000;

// ---------------------------------------------------------------------------
// Memory
// ---------------------------------------------------------------------------

/// VU1 data memory: 1024 qwords, addressed by qword index (wrapping).
pub struct Mem<'a>(pub &'a mut [u8; 16384]);

impl Mem<'_> {
    fn off(a: u16) -> usize {
        (a as usize & 0x3ff) * 16
    }
    pub fn ld(&self, a: u16) -> V {
        let o = Self::off(a);
        std::array::from_fn(|l| u32::from_le_bytes(self.0[o + 4 * l..o + 4 * l + 4].try_into().unwrap()))
    }
    /// Store the lanes of `v` selected by `mask` (bit 3 = x .. bit 0 = w).
    pub fn st_mask(&mut self, a: u16, v: V, mask: u8) {
        let o = Self::off(a);
        for (l, x) in v.iter().enumerate() {
            if mask & (8 >> l) != 0 {
                self.0[o + 4 * l..o + 4 * l + 4].copy_from_slice(&x.to_le_bytes());
            }
        }
    }
    pub fn st(&mut self, a: u16, v: V) {
        self.st_mask(a, v, 0b1111);
    }
    /// ILW: the low 16 bits of one lane.
    pub fn ilw(&self, a: u16, lane: usize) -> u16 {
        self.ld(a)[lane] as u16
    }
    /// ISW: a VI register, zero-extended, into one lane.
    pub fn isw(&mut self, a: u16, lane: usize, v: u16) {
        self.st_mask(a, [v as u32; 4], 8 >> lane);
    }
    /// The matrix columns at `a..a+4`.
    pub fn matrix(&self, a: u16) -> [V; 4] {
        std::array::from_fn(|i| self.ld(a.wrapping_add(i as u16)))
    }
    /// The GIF packet XGKICK sends from qword `a`: tags up to EOP.
    pub fn gif_packet(&self, a: u16) -> Vec<u128> {
        let n = 1024u32;
        let mut q = a as u32 & (n - 1);
        let mut out = Vec::new();
        let read = |q: u32| -> u128 { u128::from_le_bytes(self.0[q as usize * 16..q as usize * 16 + 16].try_into().unwrap()) };
        loop {
            let tag = read(q);
            out.push(tag);
            q = (q + 1) & (n - 1);
            let lo = tag as u64;
            let nloop = (lo & 0x7fff) as u32;
            let eop = (lo >> 15) & 1 != 0;
            let flg = (lo >> 58) & 3;
            let nreg = match ((lo >> 60) & 0xf) as u32 {
                0 => 16,
                r => r,
            };
            let body = match flg {
                0 => nloop * nreg,
                1 => (nloop * nreg).div_ceil(2),
                _ => nloop,
            };
            for _ in 0..body {
                if out.len() as u32 >= n {
                    break;
                }
                out.push(read(q));
                q = (q + 1) & (n - 1);
            }
            if eop || out.len() as u32 >= n {
                break;
            }
        }
        out
    }
}

// ---------------------------------------------------------------------------
// PS2 float vector ops (the translated code's helpers)
// ---------------------------------------------------------------------------

pub fn add(a: V, b: V) -> V {
    fast::add4::<0b1111, true, true>(&a, &b).0
}
pub fn sub(a: V, b: V) -> V {
    fast::sub4::<0b1111, true, true>(&a, &b).0
}
pub fn mul(a: V, b: V) -> V {
    fast::mul4::<0b1111, true, true>(&a, &b).0
}
/// `acc + a * b`.
pub fn madd(acc: V, a: V, b: V) -> V {
    fast::madd4::<0b1111, true, true, true>(&acc, &a, &b).0
}
/// `acc - a * b`.
pub fn msub(acc: V, a: V, b: V) -> V {
    fast::msub4::<0b1111, true, true, true>(&acc, &a, &b).0
}
pub fn ftoi0(a: V) -> V {
    fast::ftoi4::<0>(&a)
}
pub fn ftoi4(a: V) -> V {
    fast::ftoi4::<4>(&a)
}
pub fn itof0(a: V) -> V {
    fast::itof4::<0>(&a)
}
pub fn bc(x: u32) -> V {
    [x; 4]
}
/// `dst` with the lanes in `mask` (bit 3 = x) taken from `src`.
pub fn merge(dst: V, src: V, mask: u8) -> V {
    std::array::from_fn(|l| if mask & (8 >> l) != 0 { src[l] } else { dst[l] })
}
/// DIV: `a / b` as the DIV unit computes it.
pub fn div(a: u32, b: u32) -> u32 {
    float::div(a, b).value
}

/// `M·p` as MULAx / MADDAy / MADDAz / MADDw (columns `m[0..4]`).
pub fn xform(m: &[V; 4], p: V) -> V {
    let acc = mul(m[0], bc(p[0]));
    let acc = madd(acc, m[1], bc(p[1]));
    let acc = madd(acc, m[2], bc(p[2]));
    madd(acc, m[3], bc(p[3]))
}

/// The CLIP flag register: the judgement bits of the last 4 CLIPs, newest
/// in bits 0-5 (+x -x +y -y +z -z).
#[derive(Clone, Copy, Default)]
pub struct ClipFlags(u32);

impl ClipFlags {
    /// CLIPw.xyz v, v.w.
    pub fn clip(&mut self, v: V) {
        self.0 = ((self.0 << 6) | fast::clip(v, v[3])) & 0xff_ffff;
    }
    pub fn fcand(self, imm: u32) -> bool {
        self.0 & imm & 0xff_ffff != 0
    }
    pub fn fcor(self, imm: u32) -> bool {
        (self.0 | imm) & 0xff_ffff == 0xff_ffff
    }
    /// Some vertex of the last 3 is outside the guard band.
    pub fn triangle_out(self) -> bool {
        self.fcand(0x3_ffff)
    }
    /// The triangle test of the clip handlers for the last 3 vertices:
    /// true = drop it outright (any vertex beyond the far plane, or all
    /// three outside one plane); false = it needs the clipper.
    pub fn triangle_dropped(self) -> bool {
        self.fcand(0x01_0410)
            || self.fcor(0xfd_f7df)
            || self.fcor(0xff_7df7)
            || self.fcor(0xff_befb)
            || self.fcor(0xff_df7d)
            || self.fcor(0xff_efbe)
    }
}

// ---------------------------------------------------------------------------
// Drawing state and strip emission
// ---------------------------------------------------------------------------

/// What the strip and fan code reads from the header: M, guard scale and
/// offset (qw0-5).
pub struct Raster {
    pub mat: [V; 4],
    pub gs: V,
    pub go: V,
}

impl Raster {
    pub fn load(m: &Mem) -> Raster {
        Raster { mat: m.matrix(MATRIX), gs: m.ld(GUARD_SCALE), go: m.ld(GUARD_OFFSET) }
    }
    /// Screen position (before FTOI4): `gO·1 + (c·q)·gS`.
    fn screen(&self, cq: V) -> V {
        madd(mul(self.go, bc(ONE)), cq, self.gs)
    }
}

/// One strip vertex: where its position and its UV are in VU memory.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vertex {
    pub pos: u16,
    pub uv: u16,
}

/// How a strip routine writes the RGBA slot.
#[derive(Clone, Copy)]
pub enum Rgba {
    /// Store this colour (type 1: qw6).
    Store(V),
    /// Leave the slot alone (type 2 fills the slots beforehand).
    Keep,
}

/// Emit one triangle strip into the output at `*out` (STQ, RGBA, XYZF2 per
/// vertex). Per vertex: `c = M·P`, `q = 1/c.w`, STQ = UV·q, XYZF2 =
/// FTOI4(gO + c·q·gS), CLIP of `c`. The first two vertices get ADC; from the
/// third on, a triangle with a vertex outside the guard band gets ADC on its
/// last vertex and, unless the test drops it outright, is clipped and
/// kicked as a fan before the next vertex.
///
/// `mark` is the value the clip handler of a vertex saves at qw511.w (only
/// the type-2 zipper handlers do; it is stored whenever the guard test
/// fails, before the triangle test).
#[allow(clippy::too_many_arguments)]
pub fn emit_strip(
    m: &mut Mem,
    r: &Raster,
    clip: &mut ClipFlags,
    verts: &[Vertex],
    out: &mut u16,
    rgba: Rgba,
    mark: &dyn Fn(usize) -> Option<u16>,
    kick: &mut dyn FnMut(&[u128]),
) {
    let mut cs: Vec<V> = Vec::with_capacity(verts.len());
    for (i, v) in verts.iter().enumerate() {
        let c = xform(&r.mat, m.ld(v.pos));
        let q = div(ONE, c[3]);
        let stq = mul(m.ld(v.uv), bc(q));
        clip.clip(c);
        let mut xyz = ftoi4(r.screen(mul(c, bc(q))));
        let o = *out;
        m.st(o, stq);
        if let Rgba::Store(col) = rgba {
            m.st(o.wrapping_add(1), col);
        }
        cs.push(c);
        if i < 2 {
            xyz[3] = 0xffff_ffff;
        } else if clip.triangle_out() {
            xyz[3] = 0xffff_ffff;
            if let Some(code) = mark(i) {
                m.isw(RETURN_SAVE, 3, code);
            }
            if !clip.triangle_dropped() {
                // The triangle (i-2, i-1, i): positions in clip space, raw
                // UVs, and the colours already in the output slots.
                let mut tri = [[0; 4]; 9];
                for j in 0..3 {
                    let k = i - 2 + j;
                    let slot = o.wrapping_sub(3 * (2 - j) as u16);
                    tri[3 * j] = cs[k];
                    tri[3 * j + 1] = m.ld(verts[k].uv);
                    tri[3 * j + 2] = itof0(m.ld(slot.wrapping_add(1)));
                }
                clip_triangle(m, clip, r, tri, kick);
                clip.clip(cs[i - 1]);
                clip.clip(cs[i]);
            }
        }
        m.st(o.wrapping_add(2), xyz);
        *out = o.wrapping_add(3);
    }
}

/// Rewrite the ST of already-emitted vertices for a second pass:
/// `STQ.xy = UV2.xy · STQ.w` (z, w unchanged), one vertex per UV2 address,
/// from output slot `out` on.
pub fn rewrite_st(m: &mut Mem, out: u16, uv2: impl IntoIterator<Item = u16>) {
    for (i, a) in uv2.into_iter().enumerate() {
        let slot = out.wrapping_add(3 * i as u16);
        let w = m.ld(slot)[3];
        m.st_mask(slot, mul(m.ld(a), bc(w)), 0b1100);
    }
}

/// Transform `count` points from `src` with `mat` and store (x, y, 1, 1)
/// from `dst` on (the projector / shine UV generators: f4babac5 0x1b98,
/// 9790369f 0x0800).
pub fn project_xy(m: &mut Mem, mat: &[V; 4], src: u16, dst: u16, count: u16) {
    let zw = [0, 0, ONE, ONE];
    for i in 0..count {
        let p = xform(mat, m.ld(src.wrapping_add(i)));
        m.st(dst.wrapping_add(i), merge(zw, p, 0b1100));
    }
}

// ---------------------------------------------------------------------------
// The clipper (f4babac5 0x1cc8 / 9790369f 0x36d0)
// ---------------------------------------------------------------------------

/// One clip plane. For the "minus" planes (z-, x-, y-) the program forms
/// `d(v) = v[lane] + v.w`, `t = d(a) / ((a-b).w + (a-b)[lane])` and
/// interpolates `x - (a-b)·t`; for the "plus" planes (x+, y+)
/// `d(v) = v[lane] - v.w`, `t = d(a) / ((a-b).w - (a-b)[lane])` and
/// `x + (a-b)·t`.
struct Plane {
    lane: usize,
    minus: bool,
}

impl Plane {
    /// CLIP judgement bit of this plane for the newest vertex.
    fn bit(&self) -> u32 {
        1 << (self.lane * 2 + self.minus as usize)
    }
    fn num(&self, v: V) -> u32 {
        if self.minus {
            float::add(v[self.lane], v[3]).0
        } else {
            float::sub(v[self.lane], v[3]).0
        }
    }
    fn den(&self, d: V) -> u32 {
        if self.minus {
            float::add(d[3], d[self.lane]).0
        } else {
            float::sub(d[3], d[self.lane]).0
        }
    }
    /// `from ∓ d·t` (all lanes).
    fn lerp(&self, from: V, d: V, t: u32) -> V {
        let acc = mul(from, bc(ONE));
        if self.minus {
            msub(acc, d, bc(t))
        } else {
            madd(acc, d, bc(t))
        }
    }
}

/// Plane order of the original: z-, x-, x+, y+, y-.
const PLANES: [Plane; 5] = [
    Plane { lane: 2, minus: true },
    Plane { lane: 0, minus: true },
    Plane { lane: 0, minus: false },
    Plane { lane: 1, minus: false },
    Plane { lane: 1, minus: true },
];

/// Clip the triangle `tri` (3 × {clip-space pos, raw ST, RGBA as float})
/// against the 5 planes (Sutherland-Hodgman, ping-ponging between the two
/// buffers), then kick the result as a fan through the fan packet not used
/// last time.
///
/// Per edge a→b (the first edge of a plane runs from the last vertex to the
/// first): both inside → emit b; both outside → nothing; leaving (b out) →
/// emit `a ∓ (a-b)·t(a)`; entering (a out) → emit `b ∓ (a-b)·t(b)`, then b.
/// ST: b's z, w are forced to 1 when b is emitted or interpolated from; the
/// leaving intersection keeps a's z, w. RGBA is interpolated as float.
pub fn clip_triangle(m: &mut Mem, clip: &mut ClipFlags, r: &Raster, tri: [V; 9], kick: &mut dyn FnMut(&[u128])) {
    for (i, v) in tri.iter().enumerate() {
        m.st(CLIP_BUF_A + i as u16, *v);
    }
    let mut src_buf = CLIP_BUF_A;
    let mut dst_buf = CLIP_BUF_B;
    let mut count = 9u16; // qwords in the source polygon
    for plane in &PLANES {
        let mut ia = src_buf.wrapping_add(count).wrapping_sub(3);
        let mut ib = src_buf;
        let mut a = m.ld(ia);
        let mut b = m.ld(ib);
        clip.clip(a);
        clip.clip(b);
        if count == 0 {
            // Everything was clipped away by the previous plane.
            return;
        }
        let bit_b = plane.bit();
        let bit_a = bit_b << 6;
        let mut d = dst_buf;
        let mut left = count;
        loop {
            let b_st = m.ld(ib.wrapping_add(1));
            let a_st = m.ld(ia.wrapping_add(1));
            let b_col = m.ld(ib.wrapping_add(2));
            let a_col = m.ld(ia.wrapping_add(2));
            let b_st1 = [b_st[0], b_st[1], ONE, ONE];
            if !clip.fcor(0xff_ffff & !(bit_a | bit_b)) {
                let at = d;
                if !clip.fcand(bit_a | bit_b) {
                    // Both inside: emit b.
                    m.st(at, b);
                    m.st(at.wrapping_add(1), b_st1);
                    m.st(at.wrapping_add(2), b_col);
                    d = d.wrapping_add(3);
                } else {
                    let ab = sub(a, b);
                    let d_col = sub(a_col, b_col);
                    // (a - b) on xy, a's own z, w.
                    let d_st = merge(a_st, sub(a_st, b_st1), 0b1100);
                    let (p, s, c) = if clip.fcand(bit_a) {
                        // Entering: from b, and b itself goes after.
                        let t = div(plane.num(b), plane.den(ab));
                        m.st(at.wrapping_add(3), b);
                        m.st(at.wrapping_add(4), b_st1);
                        m.st(at.wrapping_add(5), b_col);
                        d = d.wrapping_add(3);
                        let s = merge(b_st1, plane.lerp(b_st1, d_st, t), 0b1100);
                        (plane.lerp(b, ab, t), s, plane.lerp(b_col, d_col, t))
                    } else {
                        // Leaving: from a (the program also stores a's ST
                        // and colour at +1/+2 first; overwritten below).
                        let t = div(plane.num(a), plane.den(ab));
                        let s = merge(a_st, plane.lerp(a_st, d_st, t), 0b1100);
                        (plane.lerp(a, ab, t), s, plane.lerp(a_col, d_col, t))
                    };
                    m.st(at, p);
                    m.st(at.wrapping_add(1), s);
                    m.st(at.wrapping_add(2), c);
                    d = d.wrapping_add(3);
                }
            }
            left = left.wrapping_sub(3);
            ia = ib;
            a = m.ld(ib);
            b = m.ld(ib.wrapping_add(3));
            ib = ib.wrapping_add(3);
            if left == 0 {
                break;
            }
            clip.clip(b);
        }
        count = d.wrapping_sub(dst_buf);
        std::mem::swap(&mut src_buf, &mut dst_buf);
    }
    if count == 0 {
        return;
    }
    emit_fan(m, src_buf, count, r, kick);
}

/// Kick the clipped polygon at `buf` (`qw` qwords) as a fan through the
/// fan packet not used last time: STQ = ST·q, RGBA = FTOI0, XYZF2 = FTOI4
/// (gO + p·q·gS) with q = 1/p.w.
fn emit_fan(m: &mut Mem, buf: u16, qw: u16, r: &Raster, kick: &mut dyn FnMut(&[u128])) {
    let tag = if m.ilw(FAN_TOGGLE, 0) == FAN_TAG_A { FAN_TAG_B } else { FAN_TAG_A };
    m.isw(FAN_TOGGLE, 0, tag);
    let mut o = tag.wrapping_add(1);
    let mut n = 0u16;
    for i in (0..qw).step_by(3) {
        let p = m.ld(buf.wrapping_add(i));
        let st = m.ld(buf.wrapping_add(i + 1));
        let col = m.ld(buf.wrapping_add(i + 2));
        let q = div(ONE, p[3]);
        m.st(o, mul(st, bc(q)));
        m.st(o.wrapping_add(1), ftoi0(col));
        m.st(o.wrapping_add(2), ftoi4(r.screen(mul(p, bc(q)))));
        o = o.wrapping_add(3);
        n += 1;
    }
    m.isw(tag, 0, EOP | n);
    kick(&m.gif_packet(tag));
}

/// Replay of `SSX3_VERIFY_BEZIER_DUMP` files (written by
/// `crates/ssx3-ps2/src/bezier_verify.rs`): input memory, the translated
/// program's kicks and output memory per sampled call.
#[cfg(test)]
pub mod dump {
    pub struct Call {
        pub entry: u32,
        pub top: u32,
        pub itop: u32,
        /// 0 = type 1, 1 = type 2.
        pub program: u32,
        pub input: Box<[u8; 16384]>,
        pub output: Box<[u8; 16384]>,
        pub kicks: Vec<Vec<u128>>,
    }

    pub fn read(path: &str) -> Vec<Call> {
        let b = std::fs::read(path).unwrap();
        let mut p = 0usize;
        let u32_at = |p: &mut usize| {
            let v = u32::from_le_bytes(b[*p..*p + 4].try_into().unwrap());
            *p += 4;
            v
        };
        let mut out = Vec::new();
        while p < b.len() {
            let entry = u32_at(&mut p);
            let top = u32_at(&mut p);
            let itop = u32_at(&mut p);
            let program = u32_at(&mut p);
            let input: Box<[u8; 16384]> = b[p..p + 16384].to_vec().into_boxed_slice().try_into().unwrap();
            p += 16384;
            let output: Box<[u8; 16384]> = b[p..p + 16384].to_vec().into_boxed_slice().try_into().unwrap();
            p += 16384;
            let nk = u32_at(&mut p);
            let mut kicks = Vec::new();
            for _ in 0..nk {
                let n = u32_at(&mut p) as usize;
                kicks.push((0..n).map(|i| u128::from_le_bytes(b[p + 16 * i..p + 16 * i + 16].try_into().unwrap())).collect());
                p += 16 * n;
            }
            out.push(Call { entry, top, itop, program, input, output, kicks });
        }
        out
    }

    pub type Run<'a> = &'a dyn Fn(u32, u32, u32, &mut [u8; 16384], &mut dyn FnMut(&[u128])) -> bool;

    /// Replay every call of `program` in the files listed in `BEZIER_DUMP`
    /// through `run`; prints per-entry counts and the first mismatches.
    pub fn replay(program: u32, run: Run) {
        let Ok(paths) = std::env::var("BEZIER_DUMP") else {
            eprintln!("BEZIER_DUMP not set; skipping");
            return;
        };
        let mut stats: std::collections::BTreeMap<u32, (u64, u64, u64)> = Default::default();
        let mut shown = 0;
        for path in paths.split(',') {
            for c in read(path).into_iter().filter(|c| c.program == program) {
                let mut mem = c.input.clone();
                let mut kicks = Vec::new();
                let ok = run(c.entry, c.top, c.itop, &mut mem, &mut |k| kicks.push(k.to_vec()));
                let e = stats.entry(c.entry).or_default();
                if !ok {
                    e.2 += 1;
                    continue;
                }
                if kicks == c.kicks && mem[..] == c.output[..] {
                    e.0 += 1;
                    continue;
                }
                e.1 += 1;
                if shown < 5 {
                    shown += 1;
                    eprintln!("mismatch: entry {:#06x} top {:#x}: kicks native {} translated {}", c.entry, c.top, kicks.len(), c.kicks.len());
                    if let Some(i) = (0..kicks.len().max(c.kicks.len())).find(|&i| kicks.get(i) != c.kicks.get(i)) {
                        let (a, b) = (kicks.get(i), c.kicks.get(i));
                        eprintln!("  kick {i}: native {} qw, translated {} qw", a.map_or(0, |k| k.len()), b.map_or(0, |k| k.len()));
                        if let (Some(a), Some(b)) = (a, b) {
                            for (j, (x, y)) in a.iter().zip(b).enumerate().filter(|(_, (x, y))| x != y).take(6) {
                                eprintln!("    qw {j}: native {x:032x} translated {y:032x}");
                            }
                        }
                    }
                    let diff: Vec<usize> = (0..1024).filter(|q| mem[q * 16..q * 16 + 16] != c.output[q * 16..q * 16 + 16]).collect();
                    for &q in diff.iter().take(6) {
                        let rd = |m: &[u8], q: usize| u128::from_le_bytes(m[q * 16..q * 16 + 16].try_into().unwrap());
                        eprintln!("  mem qw {q:#05x}: in {:032x} native {:032x} translated {:032x}", rd(&c.input[..], q), rd(&mem[..], q), rd(&c.output[..], q));
                    }
                    if !diff.is_empty() {
                        eprintln!("  data memory: {} qwords differ", diff.len());
                    }
                }
            }
        }
        for (pc, (ok, bad, ni)) in &stats {
            eprintln!("entry {pc:#06x}: ok {ok} mismatch {bad} not-implemented {ni}");
        }
        assert!(stats.values().all(|s| s.1 == 0));
    }
}
