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
        std::array::from_fn(|l| {
            u32::from_le_bytes(self.0[o + 4 * l..o + 4 * l + 4].try_into().unwrap())
        })
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
        let read = |q: u32| -> u128 {
            u128::from_le_bytes(
                self.0[q as usize * 16..q as usize * 16 + 16]
                    .try_into()
                    .unwrap(),
            )
        };
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
        Raster {
            mat: m.matrix(MATRIX),
            gs: m.ld(GUARD_SCALE),
            go: m.ld(GUARD_OFFSET),
        }
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
/// The direction the game's rider shadows are cast along this frame (the
/// projector passes' matrices; sun shadows follow it): the cross product of
/// the projector's U and V gradients over world positions, oriented up.
static PROJECTOR: std::sync::Mutex<Option<[f32; 3]>> = std::sync::Mutex::new(None);

/// A projector pass's matrix (columns, `project_xy` over world positions).
pub fn note_projector(mat: &[V; 4]) {
    let f = |c: usize, l: usize| f32::from_bits(mat[c][l]) as f64;
    let (gu, gv) = ([f(0, 0), f(1, 0), f(2, 0)], [f(0, 1), f(1, 1), f(2, 1)]);
    let d = [
        gu[1] * gv[2] - gu[2] * gv[1],
        gu[2] * gv[0] - gu[0] * gv[2],
        gu[0] * gv[1] - gu[1] * gv[0],
    ];
    let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    if l > 0.0 && l.is_finite() {
        let s = if d[2] < 0.0 { -1.0 / l } else { 1.0 / l };
        *PROJECTOR.lock().unwrap() = Some(d.map(|x| (x * s) as f32));
    }
}

/// The projector direction noted since the last call (and forget it).
pub fn take_projector() -> Option<[f32; 3]> {
    PROJECTOR.lock().unwrap().take()
}

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
    Plane {
        lane: 2,
        minus: true,
    },
    Plane {
        lane: 0,
        minus: true,
    },
    Plane {
        lane: 0,
        minus: false,
    },
    Plane {
        lane: 1,
        minus: false,
    },
    Plane {
        lane: 1,
        minus: true,
    },
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
pub fn clip_triangle(
    m: &mut Mem,
    clip: &mut ClipFlags,
    r: &Raster,
    tri: [V; 9],
    kick: &mut dyn FnMut(&[u128]),
) {
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
    let tag = if m.ilw(FAN_TOGGLE, 0) == FAN_TAG_A {
        FAN_TAG_B
    } else {
        FAN_TAG_A
    };
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

// ---------------------------------------------------------------------------
// The strip draws on the GPU (`crate::gpu`, plan step N1)
// ---------------------------------------------------------------------------

/// The strip emission of both programs as GPU draws (`vs_bezier` in
/// `ssx3-gfx/src/gpuvu/bezier.wgsl`).
///
/// A draw's data is {position, UV} per strip vertex in emission order
/// (packet A's strips, then packet B's). A triangle is named by its last
/// vertex `i` (vertices `i-2 .. i`; no triangle ends at a strip's first two
/// vertices, which have ADC). Every shader invocation computes the three
/// clip-space positions of its triangle, so all agree on the program's
/// per-triangle rules:
///
/// - a triangle with a vertex outside the guard band (CLIP, against |w|)
///   gets ADC in the strip: the strip call ([`STRIPS`], index `(i << 2) |
///   corner`) draws the others;
/// - unless dropped (a vertex beyond the far plane, or all three outside
///   one plane), such a triangle goes through the program's clipper and is
///   kicked as a fan with the fan tag: the fan call ([`FANS`], index
///   `(i << 5) | (3·k + corner)`, fan triangle `k` < 7) draws it. With
///   all three vertices in front of the eye (w > 0) the program's planes
///   are the true ones and the GPU clips the triangle itself (x/y at the
///   viewport, inside the guard band; z at the near plane, the program's
///   z- plane): numerically closer to the program than its clipper in IEEE
///   floats. Otherwise the shader runs the program's clipper: its planes
///   test against |w| and it divides by a signed w, so there its polygons
///   are not the true ones, and the game shows that;
/// - the pass-2 "flag 0" ST rewrite re-kicks the strip packets only (no
///   fan call).
///
/// The fans go first (the program kicks them as it meets them, before the
/// packet).
pub mod gpu {
    use super::*;
    use crate::gpu::{GpuCall, Link, Program, UNIFORM_QWORDS};

    /// Uniform qwords of a call.
    pub mod uni {
        /// M (columns).
        pub const MATRIX: usize = 0;
        pub const GUARD_SCALE: usize = 4;
        pub const GUARD_OFFSET: usize = 5;
        /// The RGBA every vertex gets (integers).
        pub const COLOUR: usize = 6;
        /// x: [`super::STRIPS`] / [`super::FANS`], y: 1 = pass-2 ST rewrite
        /// (own data: UV2 per vertex), z: vertex count, w: 1 = positions and
        /// UVs are the source call's (`Link::Reuse`).
        pub const INFO: usize = 7;
    }
    /// The triangles inside the guard band (the strip packets').
    pub const STRIPS: u32 = 1;
    /// The clipped ones (the fans).
    pub const FANS: u32 = 2;
    /// Fan triangles per clipped triangle (the clipper's polygon has at
    /// most 9 vertices).
    pub const FAN_TRIS: u32 = 7;

    /// `Link` key of the strip packets' vertex data (the latest draw).
    pub const KEY: u32 = 0x8000_0000 | PACKET_A as u32;
    /// What a GPU-planned draw leaves in the first vertex slot of packet A
    /// (the CPU program's STQ there): {MARK, vertex count, grid n, program},
    /// then its colour in the next slot. A later pass-2 ST rewrite on the
    /// CPU's packets is planned for the GPU only with this mark.
    const MARK: u32 = 0x315a_5042;

    /// A strip draw: vertices in emission order and strip lengths, per
    /// packet.
    #[derive(Default)]
    pub struct Strips {
        pub verts: Vec<Vertex>,
        /// (packet slot, first strip, strips end) into `lens`.
        packets: Vec<(u16, usize, usize)>,
        lens: Vec<u32>,
    }

    impl Strips {
        pub fn packet(&mut self, slot: u16) {
            let n = self.lens.len();
            self.packets.push((slot, n, n));
        }
        pub fn strip(&mut self, s: &[Vertex]) {
            self.verts.extend_from_slice(s);
            self.lens.push(s.len() as u32);
            self.packets.last_mut().expect("packet first").2 += 1;
        }
        /// The strips (first vertex, length) of packets `ps`.
        fn strips(&self, ps: std::ops::Range<usize>) -> impl Iterator<Item = (u32, u32)> + '_ {
            ps.flat_map(move |p| {
                let (_, a, b) = self.packets[p];
                let first: u32 = self.lens[..a].iter().sum();
                self.lens[a..b].iter().scan(first, |f, &len| {
                    let s = *f;
                    *f += len;
                    Some((s, len))
                })
            })
        }
        /// The triangles (last vertex) of the strips of packets `ps`.
        fn triangles(&self, ps: std::ops::Range<usize>) -> impl Iterator<Item = u32> + '_ {
            self.strips(ps)
                .flat_map(|(s, len)| (2..len).map(move |k| s + k))
        }
        /// Runs of packets with the same tag (one call each).
        fn runs(&self, m: &Mem) -> Vec<(V, std::ops::Range<usize>)> {
            let mut out: Vec<(V, std::ops::Range<usize>)> = Vec::new();
            for (p, &(slot, ..)) in self.packets.iter().enumerate() {
                let t = m.ld(slot);
                match out.last_mut() {
                    Some((lt, r)) if lt[1..] == t[1..] => r.end = p + 1,
                    _ => out.push((t, p..p + 1)),
                }
            }
            out
        }
    }

    /// Triangle lists: index `i << 2 | corner` per strip triangle,
    /// `i << 5 | k` (k < 3 · FAN_TRIS) per fan of a clipped one.
    fn strip_indices(tris: impl Iterator<Item = u32>) -> (Vec<u32>, u32) {
        (tris.collect(), crate::gpu::tri_mode(3, 2))
    }

    fn fan_indices(tris: impl Iterator<Item = u32>) -> (Vec<u32>, u32) {
        (tris.collect(), crate::gpu::tri_mode(3 * FAN_TRIS, 5))
    }

    fn tag(v: V) -> u128 {
        v[0] as u128 | (v[1] as u128) << 32 | (v[2] as u128) << 64 | (v[3] as u128) << 96
    }

    fn untag(t: u128) -> V {
        std::array::from_fn(|i| (t >> (32 * i)) as u32)
    }

    /// Whether a fan tag draws the clipped triangles as such: a triangle
    /// primitive (PRE) with the strip's register list.
    fn fans_draw(strip: V, fan: V) -> bool {
        let prim = (fan[1] >> 15) & 7;
        fan[1] & (1 << 14) != 0
            && (3..=5).contains(&prim)
            && fan[1] >> 26 == strip[1] >> 26
            && fan[2..] == strip[2..]
    }

    fn uniforms(r: &Raster, colour: V, info: [u32; 4]) -> [[u32; 4]; UNIFORM_QWORDS] {
        let mut u = [[0u32; 4]; UNIFORM_QWORDS];
        u[uni::MATRIX..uni::MATRIX + 4].copy_from_slice(&r.mat);
        u[uni::GUARD_SCALE] = r.gs;
        u[uni::GUARD_OFFSET] = r.go;
        u[uni::COLOUR] = colour;
        u[uni::INFO] = info;
        u
    }

    /// The draws of a strip emission (in place of [`emit_strip`] for every
    /// strip and the packet kicks): the tags are read now, as kicked. Leaves
    /// the mark in the packet (the tags keep their NLOOP: a type-1 re-draw
    /// on the CPU kicks with them).
    pub fn draw(m: &mut Mem, s: &Strips, colour: V, n: u16, program: u32, out: &mut Vec<GpuCall>) {
        let data = s
            .verts
            .iter()
            .flat_map(|v| [m.ld(v.pos), m.ld(v.uv)])
            .collect::<Vec<_>>();
        let nv = s.verts.len() as u32;
        draw_data(m, s, data, nv, colour, n, program, out);
    }

    /// `m·p` (`m` by columns), summed in `bz_clip`'s order: the CPU clip
    /// transform of the fan triage and of [`eval`] (`eval` relies on the
    /// order; the triage's bound holds for any).
    fn clip(m: &[F4; 4], p: F4) -> F4 {
        // (By columns, so the four components vectorise.)
        let term = |c: usize| m[c].map(|x| x * p[c]);
        let add = |a: F4, b: F4| -> F4 { std::array::from_fn(|l| a[l] + b[l]) };
        add(add(add(term(0), term(1)), term(2)), term(3))
    }

    // The fan triage's bound. The shader sets a flag when `h[l] > |h[3]|`
    // (or `-h[l] > |h[3]|`) in `f32` (`bz_flags`). An `f32` dot of 4 terms
    // (in any order, with or without FMA, rounding to nearest or toward
    // zero) is within 8·2^-24·S of the exact one, S = Σ|terms| (Higham's
    // γ₄ for u = 2^-23). So the shader certainly sets the flag when the
    // exact `h[l] - |h[3]|` is above 8·2^-24·(S[l] + S[3]), and certainly
    // leaves it clear at or below the negative. The first test, in `f32`,
    // adds 4·2^-24·S for this CPU's dot and 2^-24·S for its own margin
    // arithmetic: 13·2^-24 per unit of S. It retests the vertices it
    // leaves undecided in `f64`, where the products are exact: 8·2^-24,
    // times `1 + 2^-19` for γ₄'s higher terms and the `f64` sums. `floor`
    // covers what the shader's flush to zero may cost. Valid
    // while every S is below `f32::MAX / 2`, so that no partial sum in any
    // order overflows or saturates; beyond that every flag is possible.

    /// The two tests' margins per unit of Σ|terms|: 13·2^-24 and
    /// 8·2^-24·(1 + 2^-19).
    const F32_MARGIN: f32 = 6.5 * f32::EPSILON;
    const F64_MARGIN: f64 = 4.0 * f32::EPSILON as f64 * (1.0 + 1.0 / (1u32 << 19) as f64);
    /// Results the shader may flush to zero per flag: 4 products and 3
    /// sums per component, two components (14), and 2 more for both
    /// sides' gradual underflow (under 2^-149 per rounding).
    const FLUSHED: f64 = 16.0;

    /// The six clip flags (+x -x +y -y +z -z).
    const ALL: u32 = 0x3f;

    /// A vertex's clip flags as the fan shader may compute them: those it
    /// may set (`possible`) and those it must set (`certain`).
    #[derive(Clone, Copy)]
    struct VertexFlags {
        possible: u32,
        certain: u32,
    }

    /// Every flag possible, none certain.
    const UNKNOWN: VertexFlags = VertexFlags {
        possible: ALL,
        certain: 0,
    };

    /// [`VertexFlags`] for a clip position within `k·(s[l] + s[3]) +
    /// floor` of `h` in each component pair: the `f64` test.
    fn decide(h: [f64; 4], s: [f64; 4], k: f64, floor: f64) -> VertexFlags {
        let a = h[3].abs();
        // (Lane 3, w against itself, goes with the others and is masked
        // off.)
        let (mut clear, mut certain) = (0, 0);
        for l in 0..4 {
            let margin = (s[l] + s[3]) * k + floor;
            let (up, down) = (h[l] - a, -h[l] - a);
            clear |=
                u32::from(up <= -margin) << (2 * l) | u32::from(down <= -margin) << (2 * l + 1);
            certain |= u32::from(up > margin) << (2 * l) | u32::from(down > margin) << (2 * l + 1);
        }
        VertexFlags {
            possible: !clear & ALL,
            certain: certain & ALL,
        }
    }

    /// [`decide`] in `f32` with [`F32_MARGIN`], the first test (`f32`
    /// arithmetic is twice as wide in SIMD; its margin covers its own
    /// rounding).
    fn decide_f32(h: F4, s: F4, floor: f32) -> VertexFlags {
        let a = h[3].abs();
        let (mut clear, mut certain) = (0, 0);
        for l in 0..4 {
            let margin = (s[l] + s[3]) * F32_MARGIN + floor;
            let (up, down) = (h[l] - a, -h[l] - a);
            clear |=
                u32::from(up <= -margin) << (2 * l) | u32::from(down <= -margin) << (2 * l + 1);
            certain |= u32::from(up > margin) << (2 * l) | u32::from(down > margin) << (2 * l + 1);
        }
        VertexFlags {
            possible: !clear & ALL,
            certain: certain & ALL,
        }
    }

    /// The fan shader's flags for a position under the draw matrix `mat`
    /// (the uniforms' columns).
    fn clip_flags(mat: &[V; 4]) -> impl Fn(F4) -> VertexFlags {
        let mat = mat.map(|c| c.map(f32::from_bits));
        let abs = mat.map(|c| c.map(f32::abs));
        // A flushed position component costs |m|·2^-126 per term; a
        // flushed matrix entry has no bound (no triage for that draw).
        let entries = || abs.iter().flatten();
        let floor = if entries().any(|&x| x != 0.0 && x < f32::MIN_POSITIVE) {
            f64::INFINITY
        } else {
            (FLUSHED + entries().map(|&x| f64::from(x)).sum::<f64>()) * f64::from(f32::MIN_POSITIVE)
        };
        let floor32 = floor as f32;
        move |p| {
            let s = clip(&abs, p.map(f32::abs));
            // (Also NaN and infinity.)
            if !s.iter().all(|&x| x < f32::MAX / 2.0) {
                return UNKNOWN;
            }
            let f = decide_f32(clip(&mat, p), s, floor32);
            if f.possible == f.certain {
                return f;
            }
            let dot = |m: &[F4; 4], p: F4| -> [f64; 4] {
                std::array::from_fn(|l| (0..4).map(|c| f64::from(m[c][l]) * f64::from(p[c])).sum())
            };
            decide(dot(&mat, p), dot(&abs, p.map(f32::abs)), F64_MARGIN, floor)
        }
    }

    /// Whether the fan shader may draw a strip triangle with these vertex
    /// flags: it kills one with no flag (inside the guard band; the strip
    /// call draws it) and one [`fan_dropped`] drops.
    fn fan_draws([f0, f1, f2]: [VertexFlags; 3]) -> bool {
        (f0.possible | f1.possible | f2.possible) != 0
            && !fan_dropped([f0.certain, f1.certain, f2.certain])
    }

    /// The triangles of packets `ps` the fan shader may draw, from the
    /// draw's matrix and input data ({position, UV} per vertex; a vertex
    /// without data, which [`draw`] never gives, is kept).
    fn fan_triangles(
        mat: &[V; 4],
        data: &[[u32; 4]],
        s: &Strips,
        ps: std::ops::Range<usize>,
    ) -> Vec<u32> {
        let flags = clip_flags(mat);
        let flags = |v: u32| {
            data.get(2 * v as usize)
                .map_or(UNKNOWN, |q| flags(q.map(f32::from_bits)))
        };
        let mut out = Vec::new();
        for (first, len) in s.strips(ps) {
            let mut f = [UNKNOWN; 3];
            for v in first..first + len {
                f = [f[1], f[2], flags(v)];
                if v >= first + 2 && fan_draws(f) {
                    out.push(v);
                }
            }
        }
        out
    }

    /// [`draw`] with the vertex data given ({position, UV} per strip
    /// vertex), and the vertex count the mark records (the program's).
    #[allow(clippy::too_many_arguments)]
    pub fn draw_data(
        m: &mut Mem,
        s: &Strips,
        data: Vec<[u32; 4]>,
        mark_nv: u32,
        colour: V,
        n: u16,
        program: u32,
        out: &mut Vec<GpuCall>,
    ) {
        let r = Raster::load(m);
        let nv = s.verts.len() as u32;
        // (Smooth normals only for the renderer's lighting: without it a
        // zero third qword, the layout unchanged.)
        let mut source = Some(if crate::world_casters::enabled() {
            with_normals(s, &data)
        } else {
            without_normals(s, &data)
        });
        let alpha_max = colour[3] as i32 as f32;
        let fan = m.ld(FAN_TAG_A);
        let mut call =
            |mode: u32, t: V, (indices, index_mode): (Vec<u32>, u32), out: &mut Vec<GpuCall>| {
                let (data, link, reuse) = match source.take() {
                    Some(d) => (d, Link::Source(KEY), 0),
                    None => (Vec::new(), Link::Reuse(KEY), 1),
                };
                out.push(GpuCall {
                    program: Program::Bezier,
                    pre: Vec::new(),
                    tag: tag(t),
                    uniforms: uniforms(&r, colour, [mode, 0, nv, reuse]),
                    data,
                    indices,
                    index_mode,
                    alpha_max,
                    link,
                    split: None,
                    mesh_key: None,
                });
            };
        for (t, ps) in s.runs(m) {
            // A fan slot holding another program's data (the program does
            // not always write its fan tags) would make the GS read that
            // data as registers. Here the clipped triangles are drawn with
            // the program's own fan tag for the strip's context instead:
            // what the slot was meant to hold (renderer only: no game state
            // depends on it).
            let fan = if fans_draw(t, fan) {
                Some(fan)
            } else {
                [m.ld(FAN_TAG_CTX1), m.ld(FAN_TAG_CTX2)]
                    .into_iter()
                    .find(|&f| fans_draw(t, f))
                    .map(|f| [fan[0], f[1], f[2], f[3]])
            };
            if let Some(fan) = fan {
                // The fan shader kills every other triangle: listing only
                // these, or no fan call, leaves the drawn ones unchanged.
                let crossing = fan_triangles(&r.mat, &data, s, ps.clone());
                if !crossing.is_empty() {
                    call(FANS, fan, fan_indices(crossing.into_iter()), out);
                }
            }
            call(STRIPS, t, strip_indices(s.triangles(ps)), out);
        }
        m.st(PACKET_A + 1, [MARK, mark_nv, n as u32, program]);
        m.st(PACKET_A + 2, colour);
    }

    /// The draw's vertex data with a smooth normal per vertex: {position,
    /// UV} per strip vertex in, {position, UV, normal} out (`DATA_STRIDE`).
    /// The normal sums the faces around each grid point (keyed by its
    /// position, so the strips sharing a row agree), consistently wound
    /// (strip triangles alternate) and each strip turned up (its faces'
    /// sum facing +z: strips of opposite winding would cancel where they
    /// meet, leaving short normals in any direction that change as the
    /// draws do). For the renderer's lighting; the game has none.
    fn with_normals(s: &Strips, data: &[[u32; 4]]) -> Vec<[u32; 4]> {
        let nv = s.verts.len().min(data.len() / 2);
        let pos =
            |v: usize| -> [f32; 3] { std::array::from_fn(|l| f32::from_bits(data[2 * v][l])) };
        let key = |v: usize| -> [u32; 3] { [data[2 * v][0], data[2 * v][1], data[2 * v][2]] };
        let mut acc: std::collections::HashMap<[u32; 3], [f32; 3]> = Default::default();
        let mut start = 0usize;
        let mut faces: Vec<([usize; 3], [f32; 3])> = Vec::new();
        for &len in &s.lens {
            let len = len as usize;
            faces.clear();
            let mut up = 0f32;
            for k in 2..len {
                let (a, b, c) = (start + k - 2, start + k - 1, start + k);
                if c >= nv {
                    break;
                }
                let (pa, pb, pc) = (pos(a), pos(b), pos(c));
                let (u, w) = (
                    [pb[0] - pa[0], pb[1] - pa[1], pb[2] - pa[2]],
                    [pc[0] - pa[0], pc[1] - pa[1], pc[2] - pa[2]],
                );
                let sg = if k % 2 == 1 { -1.0 } else { 1.0 };
                let n = [
                    sg * (u[1] * w[2] - u[2] * w[1]),
                    sg * (u[2] * w[0] - u[0] * w[2]),
                    sg * (u[0] * w[1] - u[1] * w[0]),
                ];
                if !n.iter().all(|x| x.is_finite()) {
                    continue;
                }
                up += n[2];
                faces.push(([a, b, c], n));
            }
            let sg = if up < 0.0 { -1.0 } else { 1.0 };
            for &(vs, n) in &faces {
                for v in vs {
                    let e = acc.entry(key(v)).or_default();
                    for l in 0..3 {
                        e[l] += sg * n[l];
                    }
                }
            }
            start += len;
        }
        let mut out = Vec::with_capacity(nv * DATA_STRIDE);
        for v in 0..nv {
            let n = acc.get(&key(v)).copied().unwrap_or_default();
            let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            let n = if l > 0.0 && l.is_finite() {
                n.map(|x| x / l)
            } else {
                [0.0; 3]
            };
            out.extend_from_slice(&[
                data[2 * v],
                data[2 * v + 1],
                [n[0].to_bits(), n[1].to_bits(), n[2].to_bits(), 0],
            ]);
        }
        out
    }

    /// [`with_normals`]' layout with zero normals.
    fn without_normals(s: &Strips, data: &[[u32; 4]]) -> Vec<[u32; 4]> {
        let nv = s.verts.len().min(data.len() / 2);
        let mut out = Vec::with_capacity(nv * DATA_STRIDE);
        for v in 0..nv {
            out.extend_from_slice(&[data[2 * v], data[2 * v + 1], [0; 4]]);
        }
        out
    }

    /// Qwords per vertex in a draw's data: position, UV, normal.
    pub const DATA_STRIDE: usize = 3;

    #[cfg(test)]
    mod fan_tests {
        use super::{
            clip, clip_flags, decide, draw_data, eval, fan_draws, fan_indices, fan_triangles,
            flags, uni, Strips, F4, FANS, STRIPS, V,
        };
        use crate::bezier::{Mem, Vertex, FAN_TAG_A, ONE, PACKET_A};
        use crate::gpu::GpuCall;

        const IDENT: [V; 4] = [
            [ONE, 0, 0, 0],
            [0, ONE, 0, 0],
            [0, 0, ONE, 0],
            [0, 0, 0, ONE],
        ];

        fn ident() -> [F4; 4] {
            IDENT.map(|c| c.map(f32::from_bits))
        }

        fn bits(m: [F4; 4]) -> [V; 4] {
            m.map(|c| c.map(f32::to_bits))
        }

        /// The (possible, certain) flags the triage gives position `p`
        /// under matrix `m`.
        fn triaged(m: [F4; 4], p: F4) -> (u32, u32) {
            let f = clip_flags(&bits(m))(p);
            (f.possible, f.certain)
        }

        /// A xorshift generator of numbers in [-1, 1).
        fn rng(mut seed: u64) -> impl FnMut() -> f64 {
            move || {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                (seed >> 11) as f64 / (1u64 << 52) as f64 - 1.0
            }
        }

        /// The fan shader's rule on clip positions, with a margin of `2·s`
        /// on every plane: whether a triangle may be drawn as a fan.
        #[test]
        fn fan_draws_keeps_what_the_shader_may_draw() {
            let i = [0.0, 0.0, 0.0, 1.0];
            let px = [2.0, 0.0, 0.0, 1.0];
            let neg_w = [0.0, 0.0, 0.0, -1.0];
            #[rustfmt::skip]
            let rows: &[(&str, [F4; 3], f64, bool)] = &[
                ("inside the guard band", [i, i, i], 0.0, false),
                ("one corner past +x", [px, i, i], 0.0, true),
                ("a corner beyond the far plane", [[0.0, 0.0, 2.0, 1.0], i, i], 0.0, false),
                ("past -z, not the far plane", [[0.0, 0.0, -2.0, 1.0], i, i], 0.0, true),
                ("past +x and -y, no common plane", [px, [0.0, -2.0, 0.0, 1.0], i], 0.0, true),
                ("w < 0, against |w|", [[2.0, 0.0, 0.0, -1.0], neg_w, neg_w], 0.0, true),
                ("all zero", [[0.0; 4]; 3], 0.0, false),
                ("w = 0 and x > 0", [[1.0, 0.0, 0.0, 0.0], [0.0; 4], [0.0; 4]], 0.0, true),
                ("NaN", [[f32::NAN, 0.0, 0.0, 1.0], i, i], 0.0, true),
                ("just inside +x, in the margin", [[0.99, 0.0, 0.0, 1.0]; 3], 0.02, true),
                ("just past +x, in the margin", [[1.01, 0.0, 0.0, 1.0]; 3], 0.02, true),
                ("just past the far plane, in the margin", [[0.0, 0.0, 1.01, 1.0], i, i], 0.02, true),
                ("past +x beyond the margin", [[1.05, 0.0, 0.0, 1.0]; 3], 0.02, false),
                ("past the far plane beyond the margin", [[0.0, 0.0, 1.05, 1.0], i, i], 0.02, false),
            ];
            for &(what, h, s, want) in rows {
                let f = h.map(|h| decide(h.map(f64::from), [s; 4], 1.0, 0.0));
                assert_eq!(fan_draws(f), want, "{what}");
            }
            for lane in 0..3 {
                for sign in [2.0, -2.0] {
                    let mut h = i;
                    h[lane] = sign;
                    let f = decide(h.map(f64::from), [0.0; 4], 1.0, 0.0);
                    assert!(!fan_draws([f; 3]), "all outside, lane {lane} sign {sign}");
                }
            }
        }

        /// The triage's flags at its edges: matrix entries changed from the
        /// identity (column, lane, value), the position, the flags looked
        /// at and the (possible, certain) ones among them.
        #[test]
        fn triage_flags_at_the_edges() {
            let e = f32::EPSILON; // 2·2^-24
            let all = super::ALL;
            #[rustfmt::skip]
            let rows: &[(&str, &[(usize, usize, f32)], F4, u32, (u32, u32))] = &[
                // Σ|terms| is x + 1: the exact test's margin is about 16·2^-24.
                ("18·2^-24 past +x", &[], [1.0 + 9.0 * e, 0.0, 0.0, 1.0], 1, (1, 1)),
                ("14·2^-24 past +x", &[], [1.0 + 7.0 * e, 0.0, 0.0, 1.0], 1, (1, 0)),
                ("14·2^-24 inside +x", &[], [1.0 - 7.0 * e, 0.0, 0.0, 1.0], 1, (1, 0)),
                ("18·2^-24 inside +x", &[], [1.0 - 9.0 * e, 0.0, 0.0, 1.0], 1, (0, 0)),
                // 1e8 + 1 - 1e8: 0 in this CPU's f32, 1 exactly.
                ("cancelling terms", &[(0, 0, 1e8), (1, 0, 1.0), (2, 0, -1e8)], [1.0; 4], 1, (1, 0)),
                // The shader may flush x and w to zero, or not.
                ("subnormal position", &[], [2e-39, 0.0, 0.0, 1e-39], 3, (3, 0)),
                ("subnormal matrix entry", &[(1, 0, 1e-40)], [2.0, 0.0, 0.0, 1.0], all, (all, 0)),
                ("NaN position", &[], [f32::NAN, 0.0, 0.0, 1.0], all, (all, 0)),
                ("infinite position", &[], [f32::INFINITY, 0.0, 0.0, 1.0], all, (all, 0)),
                ("NaN matrix entry", &[(3, 0, f32::NAN)], [0.0, 0.0, 0.0, 1.0], all, (all, 0)),
                // x is 0 exactly; the shader's sum overflows (+x or -x set,
                // by rounding mode).
                ("overflowing sum",
                    &[(1, 0, 1.0), (2, 0, -1.0), (3, 0, -1.0), (1, 1, 0.0), (2, 2, 0.0), (3, 3, 1e-4)],
                    [3e38; 4], all, (all, 0)),
                // x > w exactly; the shader's products overflow (inf - inf: no
                // flag).
                ("overflowing products", &[(0, 0, 1e30), (1, 0, -1e30)],
                    [1e30, 0.99e30, 0.0, 1.0], all, (all, 0)),
            ];
            for &(what, edits, p, mask, want) in rows {
                let mut m = ident();
                for &(c, l, x) in edits {
                    m[c][l] = x;
                }
                let (possible, certain) = triaged(m, p);
                assert_eq!((possible & mask, certain & mask), want, "{what}");
            }
        }

        /// Against the exact dot (in `f64`, where the `f32` products are
        /// exact): a certain flag is set and a flag left out of `possible`
        /// is clear for every shader `f32` dot within γ₄(2^-23)·Σ|terms|
        /// of it, near each plane of random transforms with large
        /// translations (cancelling terms).
        #[test]
        fn flags_hold_for_every_shader_rounding() {
            let mut rnd = rng(0x2545_f491_4f6c_dd1d);
            let u = f64::from(f32::EPSILON) / 2.0;
            // The shader's γ₄, plus this check's own `f64` sums.
            let gamma = 8.0 * u / (1.0 - 8.0 * u) + 4.0 * f64::EPSILON;
            let exact = |m: &[[f32; 4]; 4], p: F4, l: usize| -> (f64, f64) {
                let t = (0..4).map(|c| f64::from(m[c][l]) * f64::from(p[c]));
                (t.clone().sum(), t.map(f64::abs).sum())
            };
            let mut seen = [0; 3]; // certain, undecided, clear
            for _ in 0..20_000 {
                let mut m = [[0f32; 4]; 4];
                for c in 0..3 {
                    for l in 0..3 {
                        m[c][l] = rnd() as f32;
                    }
                    m[c][3] = (0.01 * rnd()) as f32;
                }
                m[3][3] = (50.0 + 49.0 * rnd()) as f32;
                let p = [1000.0 * rnd(), 1000.0 * rnd(), 1000.0 * rnd(), 1.0].map(|x| x as f32);
                let (w, _) = exact(&m, p, 3);
                for l in 0..3 {
                    // A translation putting `h[l]` about k·2^-24·Σ|terms|
                    // from `±|w|`, k in ±20.
                    let (part, sum) = exact(&m, p, l);
                    let side = if rnd() < 0.0 { -1.0 } else { 1.0 };
                    let k = 20.0 * rnd() * u * 2.0 * (sum + w.abs());
                    m[3][l] = (side * (w.abs() + k) - part) as f32;
                }
                let (possible, certain) = triaged(m, p);
                let (hw, sw) = exact(&m, p, 3);
                for l in 0..3 {
                    let (h, s) = exact(&m, p, l);
                    let e = gamma * (s + sw);
                    for (bit, d) in [(2 * l, h - hw.abs()), (2 * l + 1, -h - hw.abs())] {
                        if certain >> bit & 1 != 0 {
                            assert!(d > e, "certain but within the shader's bound: {m:?} {p:?}");
                            seen[0] += 1;
                        } else if possible >> bit & 1 != 0 {
                            seen[1] += 1;
                        } else {
                            assert!(d <= -e, "clear but within the shader's bound: {m:?} {p:?}");
                            seen[2] += 1;
                        }
                    }
                }
            }
            assert!(seen.iter().all(|&n| n > 1000), "{seen:?}");
        }

        /// The calls [`draw_data`] plans for these strips (one packet,
        /// matrix `mat`).
        fn planned(mat: [V; 4], strips: &[&[F4]]) -> Vec<GpuCall> {
            let mut bytes = [0u8; 16384];
            let mut m = Mem(&mut bytes);
            for (q, c) in mat.into_iter().enumerate() {
                m.st(q as u16, c);
            }
            let tag = [0, (1 << 14) | (4 << 15), 7, 8];
            m.st(PACKET_A, tag);
            m.st(FAN_TAG_A, tag);
            let mut s = Strips::default();
            s.packet(PACKET_A);
            let mut data = Vec::new();
            let mut nv = 0u32;
            for strip in strips {
                let verts: Vec<Vertex> = strip.iter().map(|_| Vertex { pos: 0, uv: 0 }).collect();
                nv += verts.len() as u32;
                for &p in strip.iter() {
                    data.push(p.map(f32::to_bits));
                    data.push([0; 4]);
                }
                s.strip(&verts);
            }
            let mut out = Vec::new();
            draw_data(&mut m, &s, data, nv, [128; 4], 4, 0, &mut out);
            out
        }

        /// Through the planner: a run with no crossing triangle emits no
        /// fan call; a mixed run lists only the crossing triangle in its
        /// fan call, which still precedes the strip call.
        #[test]
        fn draw_data_lists_only_crossing_fans() {
            let inside = [[0.0, 0.0, 0.0, 1.0]; 3];
            let out = planned(IDENT, &[&inside]);
            assert_eq!(out.len(), 1);
            assert_eq!(out[0].uniforms[uni::INFO][0], STRIPS);
            assert_eq!(out[0].indices, vec![2]);

            let crossing = [
                [2.0, 0.0, 0.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
            ];
            let out = planned(IDENT, &[&inside, &crossing]);
            assert_eq!(out.len(), 2);
            assert_eq!(out[0].uniforms[uni::INFO][0], FANS);
            assert_eq!(out[0].indices, vec![5]);
            assert_eq!(out[1].uniforms[uni::INFO][0], STRIPS);
            assert_eq!(out[1].indices, vec![2, 5]);

            // A vertex without data (which `draw` never plans) is kept.
            let mut s = Strips::default();
            s.packet(PACKET_A);
            s.strip(&[Vertex { pos: 0, uv: 0 }; 3]);
            assert_eq!(fan_triangles(&IDENT, &[], &s, 0..1), vec![2]);
        }

        /// The plan against [`eval`], the shader's formulas in this CPU's
        /// `f32` (one of the roundings the triage covers): the fan call
        /// lists every fan `eval` draws among all the strip's triangles,
        /// and every vertex's `eval` flags lie between the triage's certain
        /// and possible ones. Random strips with vertices near the planes,
        /// at w = 1, with and without large translations.
        #[test]
        fn fan_call_draws_what_eval_draws() {
            let mut rnd = rng(0x9e37_79b9_7f4a_7c15);
            let u = f64::from(f32::EPSILON) / 2.0;
            let mut seen = [0; 3]; // fans drawn, triangles not listed, undecided vertices
            for case in 0..400 {
                let t: [f64; 3] = if case % 2 == 0 {
                    [0.0; 3]
                } else {
                    std::array::from_fn(|_| 1e4 * rnd())
                };
                let mut m = ident();
                for l in 0..3 {
                    m[3][l] = t[l] as f32;
                }
                let strip: Vec<F4> = (0..8)
                    .map(|_| {
                        let mut h: [f64; 3] = std::array::from_fn(|_| 1.2 * rnd());
                        if rnd() < 0.0 {
                            // About k·2^-24·Σ|terms| from a plane, k in ±24.
                            let l = ((rnd() + 1.0) * 1.5) as usize;
                            let side = if rnd() < 0.0 { -1.0 } else { 1.0 };
                            let s = (side - t[l]).abs() + t[l].abs() + 1.0;
                            h[l] = side * (1.0 + 24.0 * rnd() * u * s);
                        }
                        [h[0] - t[0], h[1] - t[1], h[2] - t[2], 1.0].map(|x| x as f32)
                    })
                    .collect();
                let out = planned(bits(m), &[&strip]);
                let src = &out[0].data;
                let strips = out.last().expect("a strip call");
                let mut all = strips.clone();
                all.uniforms[uni::INFO][0] = FANS;
                (all.indices, all.index_mode) = fan_indices(strips.indices.iter().copied());
                let want: Vec<u32> = eval(&all, src).fans.iter().map(|f| f.0).collect();
                let listed: Vec<&GpuCall> = out
                    .iter()
                    .filter(|c| c.uniforms[uni::INFO][0] == FANS)
                    .collect();
                let got: Vec<u32> = listed
                    .iter()
                    .flat_map(|c| eval(c, src).fans)
                    .map(|f| f.0)
                    .collect();
                assert_eq!(got, want, "{m:?} {strip:?}");
                let n_listed: usize = listed.iter().map(|c| c.indices.len()).sum();
                seen[0] += want.len();
                seen[1] += strips.indices.len() - n_listed;
                let triage = clip_flags(&bits(m));
                for &p in &strip {
                    let (f, mirror) = (triage(p), flags(clip(&m, p)));
                    assert_eq!(f.certain & !mirror, 0, "{m:?} {p:?}");
                    assert_eq!(mirror & !f.possible, 0, "{m:?} {p:?}");
                    seen[2] += usize::from(f.possible != f.certain);
                }
            }
            assert!(seen.iter().all(|&n| n > 50), "{seen:?}");
        }
    }

    #[cfg(test)]
    mod normal_tests {
        use super::*;

        /// Two strips over a flat 3×3 grid (z = 5), wound opposite ways:
        /// every vertex gets the same unit normal, up, the row the strips
        /// share included.
        #[test]
        fn flat_grid_has_one_normal() {
            let at = |r: usize, c: usize| {
                [
                    (c as f32 * 10.0).to_bits(),
                    (r as f32 * 10.0).to_bits(),
                    5f32.to_bits(),
                    1f32.to_bits(),
                ]
            };
            let mut s = Strips::default();
            s.packet(PACKET_A);
            let mut data = Vec::new();
            // (The second strip wound the other way: its rows swapped.)
            for r in 0..2 {
                let verts: Vec<Vertex> = (0..6).map(|_| Vertex { pos: 0, uv: 0 }).collect();
                for k in 0..6 {
                    data.push(at(r + if r == 0 { k % 2 } else { 1 - k % 2 }, k / 2));
                    data.push([0; 4]);
                }
                s.strip(&verts);
            }
            let out = with_normals(&s, &data);
            assert_eq!(out.len(), 12 * DATA_STRIDE);
            let n0 = out[2];
            for v in 0..12 {
                let n = out[DATA_STRIDE * v + 2].map(f32::from_bits);
                assert!(
                    (n[2] - 1.0).abs() < 1e-6 && n[0] == 0.0 && n[1] == 0.0,
                    "{n:?}"
                );
                assert_eq!(out[DATA_STRIDE * v + 2], n0);
            }
        }
    }

    /// The mark of the last GPU-planned draw: (vertex count, n, program,
    /// colour).
    pub fn mark(m: &Mem) -> Option<(u32, u16, u32, V)> {
        let v = m.ld(PACKET_A + 1);
        (v[0] == MARK).then(|| (v[1], v[2] as u16, v[3], m.ld(PACKET_A + 2)))
    }

    /// The pass-2 ST rewrite of the last draw's packets (in place of
    /// [`rewrite_st`] and the re-kicks): `s` gives the strips (as drawn)
    /// and the UV2 address of each vertex in `.uv`.
    pub fn rewrite_st(m: &Mem, s: &Strips, out: &mut Vec<GpuCall>) {
        let data: Vec<[u32; 4]> = s.verts.iter().map(|v| m.ld(v.uv)).collect();
        rewrite_st_data(m, s, data, out);
    }

    /// [`rewrite_st`] with the UV2s given (one per strip vertex).
    pub fn rewrite_st_data(m: &Mem, s: &Strips, data: Vec<[u32; 4]>, out: &mut Vec<GpuCall>) {
        let r = Raster::load(m);
        let (_, _, _, colour) = mark(m).expect("rewrite after a GPU draw");
        let nv = s.verts.len() as u32;
        for (t, ps) in s.runs(m) {
            // (Each call carries the UV2s; positions are the source's.)
            let (indices, index_mode) = strip_indices(s.triangles(ps));
            out.push(GpuCall {
                program: Program::Bezier,
                pre: Vec::new(),
                tag: tag(t),
                uniforms: uniforms(&r, colour, [STRIPS, 1, nv, 1]),
                data: data.clone(),
                indices,
                index_mode,
                alpha_max: 0.0,
                link: Link::Reuse(KEY),
                split: None,
                mesh_key: None,
            });
        }
    }

    /// Whether the fan tags a draw finds in memory (when it does not write
    /// them) are fan tags: other programs use the slots, and a clipped
    /// triangle kicked with what they leave there makes the GS read their
    /// data as registers (texture setup among them). The CPU programs kick
    /// exactly that; the GPU draws use the program's own fan tag instead
    /// ([`draw_data`]).
    pub fn fan_tags_ok(m: &Mem) -> bool {
        let (a, b) = (m.ld(FAN_TAG_A), m.ld(FAN_TAG_B));
        a[1..] == b[1..] && fans_draw(m.ld(STRIP_TAG_CTX1), a)
    }

    /// The patch memory a GPU-planned call reads from `a` (positions, UVs,
    /// UV2s, coefficients and LODs) must not overlap what the strip code
    /// writes (clip buffers, packets) or the program's tables.
    pub fn region_ok(a: u16) -> bool {
        (0x22b..=0x3fe - 0xd1).contains(&a)
    }

    // -----------------------------------------------------------------------
    // CPU mirror of the shader (`SSX3_GPU_CHECK`, tests)
    // -----------------------------------------------------------------------

    type F4 = [f32; 4];

    /// CLIP judgement bits (+x -x +y -y +z -z) against |w|.
    pub fn flags(c: F4) -> u32 {
        let a = c[3].abs();
        (0..3)
            .map(|l| ((c[l] > a) as u32) << (2 * l) | ((c[l] < -a) as u32) << (2 * l + 1))
            .sum()
    }

    /// The far plane's flag (+z).
    const FAR: u32 = 16;

    /// Whether the fan shader drops a clipped triangle with these vertex
    /// flags: a vertex beyond the far plane, or all three outside one
    /// plane (the clipper would keep at most an edge).
    fn fan_dropped([f0, f1, f2]: [u32; 3]) -> bool {
        (f0 | f1 | f2) & FAR != 0 || f0 & f1 & f2 != 0
    }

    /// The program's clipper ([`clip_triangle`]) in IEEE floats, as the
    /// shader runs it (for triangles reaching behind the eye): the polygon
    /// (clip-space position, ST) of the triangle `(p, st)`; empty when
    /// everything is clipped away.
    pub fn clip_polygon(p: [F4; 3], st: [F4; 3]) -> Vec<(F4, F4)> {
        let mut poly: Vec<(F4, F4)> = (0..3).map(|k| (p[k], st[k])).collect();
        for plane in &PLANES {
            let (l, minus) = (plane.lane, plane.minus);
            let bit = 1u32 << (2 * l + minus as usize);
            let out = |v: F4| flags(v) & bit != 0;
            let num = |v: F4| if minus { v[l] + v[3] } else { v[l] - v[3] };
            let den = |d: F4| if minus { d[3] + d[l] } else { d[3] - d[l] };
            let lerp = |from: F4, d: F4, t: f32| -> F4 {
                std::array::from_fn(|k| {
                    if minus {
                        from[k] - d[k] * t
                    } else {
                        from[k] + d[k] * t
                    }
                })
            };
            let n = poly.len();
            let mut next = Vec::with_capacity(9);
            for k in 0..n {
                let (a, a_st) = poly[(k + n - 1) % n];
                let (b, b_st) = poly[k];
                let (ao, bo) = (out(a), out(b));
                if ao && bo {
                    continue;
                }
                let b_st1 = [b_st[0], b_st[1], 1.0, 1.0];
                if !ao && !bo {
                    next.push((b, b_st1));
                    continue;
                }
                let d: F4 = std::array::from_fn(|k| a[k] - b[k]);
                let d_st = [a_st[0] - b_st1[0], a_st[1] - b_st1[1], 0.0, 0.0];
                if ao {
                    let t = num(b) / den(d);
                    let s = lerp(b_st1, d_st, t);
                    next.push((lerp(b, d, t), [s[0], s[1], 1.0, 1.0]));
                    next.push((b, b_st1));
                } else {
                    let t = num(a) / den(d);
                    let s = lerp(a_st, d_st, t);
                    next.push((lerp(a, d, t), [s[0], s[1], a_st[2], a_st[3]]));
                }
            }
            next.truncate(9);
            poly = next;
            if poly.is_empty() {
                break;
            }
        }
        poly
    }

    /// What a call draws, as the shader computes it: per strip vertex its
    /// {STQ, RGBA, screen XYZ}; the strip triangles drawn (last vertex);
    /// the fans (triangle, polygon vertices {STQ, screen XYZ}) as the
    /// program's clipper makes them (in front of the eye the shader lets
    /// the GPU clip instead: the same polygon, but for rounding).
    pub struct Eval {
        pub verts: Vec<[f32; 10]>,
        pub strips: Vec<u32>,
        pub fans: Vec<(u32, Vec<[f32; 6]>)>,
    }

    pub fn eval(call: &GpuCall, src: &[[u32; 4]]) -> Eval {
        let u = &call.uniforms;
        let f = |q: usize| -> F4 { u[q].map(f32::from_bits) };
        let info = u[uni::INFO];
        let m = [f(0), f(1), f(2), f(3)];
        let (gs, go) = (f(uni::GUARD_SCALE), f(uni::GUARD_OFFSET));
        let nv = info[2] as usize;
        let h = |v: usize| -> F4 { clip(&m, src[DATA_STRIDE * v].map(f32::from_bits)) };
        let uv = |v: usize| -> F4 { src[DATA_STRIDE * v + 1].map(f32::from_bits) };
        let screen = |c: F4| -> [f32; 3] {
            let q = 1.0 / c[3];
            std::array::from_fn(|l| go[l] + c[l] * q * gs[l])
        };
        let col = u[uni::COLOUR].map(|x| x as i32 as f32);
        let verts = (0..nv)
            .map(|v| {
                let c = h(v);
                let q = 1.0 / c[3];
                let uv = uv(v);
                let stq = if info[1] != 0 {
                    let uv2 = call.data[v].map(f32::from_bits);
                    [uv2[0] * (uv[3] * q), uv2[1] * (uv[3] * q), uv[2] * q]
                } else {
                    [uv[0] * q, uv[1] * q, uv[2] * q]
                };
                let xyz = screen(c);
                [
                    stq[0], stq[1], stq[2], col[0], col[1], col[2], col[3], xyz[0], xyz[1], xyz[2],
                ]
            })
            .collect();
        let step = if info[0] == FANS {
            3 * FAN_TRIS as usize
        } else {
            3
        };
        let shift = if info[0] == FANS { 5 } else { 2 };
        let (mut strips, mut fans) = (Vec::new(), Vec::new());
        for t in call.expanded_indices().chunks(step) {
            let i = (t[0] >> shift) as usize;
            let c = [h(i - 2), h(i - 1), h(i)];
            let fl = c.map(flags);
            let any = fl[0] | fl[1] | fl[2];
            let dropped = fan_dropped(fl);
            if info[0] == STRIPS {
                if any == 0 {
                    strips.push(i as u32);
                }
            } else if any != 0 && !dropped {
                let poly = clip_polygon(c, [uv(i - 2), uv(i - 1), uv(i)]);
                if !poly.is_empty() {
                    let vs = poly
                        .iter()
                        .map(|&(p, st)| {
                            let q = 1.0 / p[3];
                            let xyz = screen(p);
                            [st[0] * q, st[1] * q, st[2] * q, xyz[0], xyz[1], xyz[2]]
                        })
                        .collect();
                    fans.push((i as u32, vs));
                }
            }
        }
        Eval {
            verts,
            strips,
            fans,
        }
    }

    /// `crate::gpu::check` for both programs: the program's output (`gif`,
    /// all its kicks) against [`eval`] of the planned calls, pass by pass
    /// (a pass: the strip packets A and B and the fans kicked with them).
    /// Per strip vertex STQ / RGBA / XYZ, the strip triangles drawn (no
    /// ADC), and the fans one by one. Rewrites of an earlier call's
    /// packets are not checked.
    pub fn check(calls: &[GpuCall], gif: &[u128]) -> Option<String> {
        let words = |v: &[u128]| -> Vec<[u32; 12]> {
            v.chunks(3)
                .map(|v| std::array::from_fn(|i| (v[i / 4] >> (32 * (i % 4))) as u32))
                .collect()
        };
        // The program's passes: (strip vertices, fans, fan tags).
        let mut passes: Vec<(Vec<[u32; 12]>, Vec<Vec<[u32; 12]>>, Vec<V>)> = Vec::new();
        let (mut verts, mut fans, mut fan_tags, mut strips) =
            (Vec::new(), Vec::new(), Vec::new(), 0);
        let mut p = 0;
        while p < gif.len() {
            let head = gif[p];
            let lo = head as u64;
            let nloop = (lo & 0x7fff) as usize;
            let body = &gif[p + 1..(p + 1 + 3 * nloop).min(gif.len())];
            p += 1 + 3 * nloop;
            // (Strip packets: PRE, triangle strip.)
            if lo >> 46 & 1 == 0 || (lo >> 47) & 7 != 4 {
                fans.push(words(body));
                fan_tags.push(untag(head));
                continue;
            }
            verts.extend(words(body));
            strips += 1;
            if strips == 2 {
                passes.push((
                    std::mem::take(&mut verts),
                    std::mem::take(&mut fans),
                    std::mem::take(&mut fan_tags),
                ));
                strips = 0;
            }
        }
        // The planned passes: (draw source data, calls).
        let mut planned: Vec<(Option<&[[u32; 4]]>, Vec<&GpuCall>)> = Vec::new();
        let mut src: Option<&[[u32; 4]]> = None;
        for c in calls {
            let st = c.uniforms[uni::INFO][1] != 0;
            let source = matches!(c.link, Link::Source(_));
            if source {
                src = Some(&c.data);
            }
            let new = planned
                .last()
                .is_none_or(|(_, l)| source || st != (l[0].uniforms[uni::INFO][1] != 0));
            if new {
                planned.push((src, Vec::new()));
            }
            planned.last_mut().unwrap().1.push(c);
        }
        if planned.len() != passes.len() {
            return Some(format!(
                "bezier: {} planned passes, the program kicked {}",
                planned.len(),
                passes.len()
            ));
        }
        let mut bad = Vec::new();
        let close = |a: f32, b: f32, tol: f32| (a - b).abs() <= tol * (1.0 + b.abs());
        // Screen XYZ against FTOI4 (z: gO.z + gS.z·z/w cancels, the PS2's
        // rounding shows there).
        let xyz_ok = |g: &[f32], w: &[u32]| {
            let xy = (0..2).all(|l| {
                (g[l] * 16.0 - w[l] as i32 as f32).abs() <= 2.0 + 1e-4 * (w[l] as i32 as f32).abs()
            });
            let z = w[2] as i32 as f32;
            xy && (g[2] * 16.0 - z).abs() <= 16.0 * 256.0 + 1e-3 * z.abs()
        };
        for (k, ((src, cs), (want, want_fans, fan_tags))) in planned.iter().zip(&passes).enumerate()
        {
            let Some(src) = src else { continue };
            let evals: Vec<Eval> = cs.iter().map(|c| eval(c, src)).collect();
            let v = &evals[0].verts;
            if v.len() != want.len() {
                bad.push(format!(
                    "pass {k}: {} vertices, the program {}",
                    v.len(),
                    want.len()
                ));
                continue;
            }
            let mut strip_tris: Vec<u32> = evals
                .iter()
                .flat_map(|e| e.strips.iter().copied())
                .collect();
            strip_tris.sort();
            // The program's strip triangles: no ADC on the last vertex.
            let prog: Vec<u32> = (0..want.len() as u32)
                .filter(|&i| want[i as usize][11] >> 15 & 1 == 0)
                .collect();
            if prog != strip_tris {
                let d: Vec<_> = prog
                    .iter()
                    .filter(|i| !strip_tris.contains(i))
                    .chain(strip_tris.iter().filter(|i| !prog.contains(i)))
                    .take(4)
                    .collect();
                bad.push(format!(
                    "pass {k}: {} strip triangles, the program {} (differ at {d:?})",
                    strip_tris.len(),
                    prog.len()
                ));
            }
            // No fan call: the plan found no triangle the fan shader draws,
            // so the program must kick no fans (unless its fan tags are not
            // ones the plan would draw them with).
            if !cs.iter().any(|c| c.uniforms[uni::INFO][0] == FANS) {
                let drawable = cs
                    .iter()
                    .any(|c| fan_tags.iter().any(|&f| fans_draw(untag(c.tag), f)));
                if drawable && !want_fans.is_empty() {
                    bad.push(format!(
                        "pass {k}: no fan call, the program {} fans",
                        want_fans.len()
                    ));
                }
            } else {
                let fans: Vec<&(u32, Vec<[f32; 6]>)> =
                    evals.iter().flat_map(|e| e.fans.iter()).collect();
                if fans.len() != want_fans.len() {
                    bad.push(format!(
                        "pass {k}: {} fans, the program {}",
                        fans.len(),
                        want_fans.len()
                    ));
                }
                for ((i, g), w) in fans.iter().zip(want_fans) {
                    let ok = g.len() == w.len()
                        && g.iter().zip(w).all(|(g, w)| {
                            (0..3).all(|l| close(g[l], f32::from_bits(w[l]), 1e-2))
                                && xyz_ok(&g[3..], &w[8..11])
                        });
                    if !ok && bad.len() < 4 {
                        let ws: Vec<_> = w
                            .iter()
                            .map(|w| {
                                (
                                    w[0..3]
                                        .iter()
                                        .map(|&x| f32::from_bits(x))
                                        .collect::<Vec<_>>(),
                                    &w[8..11],
                                )
                            })
                            .collect();
                        bad.push(format!("pass {k} fan of {i}: {g:?} want {ws:?}"));
                    }
                }
            }
            for (i, (g, w)) in v.iter().zip(want).enumerate() {
                let ok_stq = (0..3).all(|l| close(g[l], f32::from_bits(w[l]), 1e-3));
                let ok_col = (0..4).all(|l| (g[3 + l] - w[4 + l] as f32).abs() <= 0.5);
                if !(ok_stq && ok_col && xyz_ok(&g[7..], &w[8..11])) && bad.len() < 4 {
                    bad.push(format!("pass {k} v{i}: {g:?} want {:?}", &w[..11]));
                }
            }
        }
        (!bad.is_empty()).then(|| format!("bezier ({} calls): {}", calls.len(), bad.join("; ")))
    }
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
            let input: Box<[u8; 16384]> = b[p..p + 16384]
                .to_vec()
                .into_boxed_slice()
                .try_into()
                .unwrap();
            p += 16384;
            let output: Box<[u8; 16384]> = b[p..p + 16384]
                .to_vec()
                .into_boxed_slice()
                .try_into()
                .unwrap();
            p += 16384;
            let nk = u32_at(&mut p);
            let mut kicks = Vec::new();
            for _ in 0..nk {
                let n = u32_at(&mut p) as usize;
                kicks.push(
                    (0..n)
                        .map(|i| {
                            u128::from_le_bytes(b[p + 16 * i..p + 16 * i + 16].try_into().unwrap())
                        })
                        .collect(),
                );
                p += 16 * n;
            }
            out.push(Call {
                entry,
                top,
                itop,
                program,
                input,
                output,
                kicks,
            });
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
                let ok = run(c.entry, c.top, c.itop, &mut mem, &mut |k| {
                    kicks.push(k.to_vec())
                });
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
                    eprintln!(
                        "mismatch: entry {:#06x} top {:#x}: kicks native {} translated {}",
                        c.entry,
                        c.top,
                        kicks.len(),
                        c.kicks.len()
                    );
                    if let Some(i) = (0..kicks.len().max(c.kicks.len()))
                        .find(|&i| kicks.get(i) != c.kicks.get(i))
                    {
                        let (a, b) = (kicks.get(i), c.kicks.get(i));
                        eprintln!(
                            "  kick {i}: native {} qw, translated {} qw",
                            a.map_or(0, |k| k.len()),
                            b.map_or(0, |k| k.len())
                        );
                        if let (Some(a), Some(b)) = (a, b) {
                            for (j, (x, y)) in a
                                .iter()
                                .zip(b)
                                .enumerate()
                                .filter(|(_, (x, y))| x != y)
                                .take(6)
                            {
                                eprintln!("    qw {j}: native {x:032x} translated {y:032x}");
                            }
                        }
                    }
                    let diff: Vec<usize> = (0..1024)
                        .filter(|q| mem[q * 16..q * 16 + 16] != c.output[q * 16..q * 16 + 16])
                        .collect();
                    for &q in diff.iter().take(6) {
                        let rd = |m: &[u8], q: usize| {
                            u128::from_le_bytes(m[q * 16..q * 16 + 16].try_into().unwrap())
                        };
                        eprintln!(
                            "  mem qw {q:#05x}: in {:032x} native {:032x} translated {:032x}",
                            rd(&c.input[..], q),
                            rd(&mem[..], q),
                            rd(&c.output[..], q)
                        );
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
