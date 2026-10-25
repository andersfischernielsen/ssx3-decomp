//! # Type 3 (`d159716b`) natively: the general VU1 program, bit-exact
//!
//! A readable Rust implementation of the VU1 microprogram with hash
//! `0xd159716bb1281fbe` (`generated/vu/vu_d159716bb1281fbe.rs` is the literal
//! translation; `docs/investigation/phase3-render.md` §1.2 describes it).
//! [`run`] takes the program's VU1 data memory and produces the same XGKICK
//! packets **and** the same data memory as the translated program, bit for
//! bit (checked on real frames by `SSX3_VERIFY_MAT3=1`,
//! `crates/ssx3-ps2/src/mat3_verify.rs`).
//!
//! Exactness rules followed throughout:
//! - every float op goes through the PS2 float helpers the translated code
//!   uses (`ssx3_vu::vu::fast`, `float`): no IEEE inf/NaN, denormals flushed,
//!   truncating rounding;
//! - the original evaluation order is kept (e.g. the strip loop transforms
//!   `w, z, y, x` while its prologue and the clipper go `x, y, z, w`; the
//!   results differ in the last bit), including the accumulator round trips
//!   the microcode makes;
//! - the DIV unit is pipelined: each `q` below is the value the original
//!   instruction sees (the program's software pipelining makes this the
//!   "intended" reciprocal everywhere it was checked);
//! - the CLIP flag register is a 4-vertex shift register ([`ClipFlags`]);
//!   FCAND/FCOR read it exactly as in the program;
//! - memory side effects are reproduced, including scratch (the clipper
//!   buffers at 0x392/0x3ad, the fan tag toggle at qw33.w, the accumulator
//!   spill at qw37), so the data memory after a call is identical too.
//!
//! Entries implemented: all the game uses (see [`implemented`]):
//! - 0x12b8 float strips (slot 71) and 0x3a08 ribbons (wake, shadow) with
//!   the guard-band cull and the Sutherland-Hodgman clipper (0x1630);
//! - 0x2270 / 0x22c8 / 0x2320 / 0x2358 meshes, both passes (ITOP == 0:
//!   ctx1, ITOP != 0: ST rewrite for ctx2), lit or unlit, clipped or not,
//!   every texgen except the pass-1 sphere map;
//! - 0x2400 (PRIM-only kick), 0x3bf0 (cubic line shape).
//!
//! Anything else returns `false` without touching memory, and the caller
//! runs the translated program: unknown entries, the pass-1 sphere map (it
//! reads positions through a pointer register left over from the previous
//! call, a game bug never triggered in play), and inputs on which the
//! original's do-while loops would not end (fewer than 3 vertices in a
//! strip, counts that are not multiples of 3).
//!
//! Notes for the wgpu port (what these functions show):
//! - every vertex is computed independently; the software pipelining of the
//!   original (2-3 vertices in flight, DIV/RSQRT a vertex ahead) never makes
//!   a vertex see another vertex's Q, so a shader per vertex reproduces it;
//! - the only order-dependent state is the CLIP flag history (the triangle
//!   test of a strip vertex looks at the 2 before it, inside one strip) and
//!   the fan tag toggle; clipped fans are kicked before their strip;
//! - outputs depend on op order at the last bit (FMA-free PS2 rounding), so
//!   a GPU version will be close but not bit-exact; this module is the
//!   bit-exact CPU reference.

use ssx3_vu::vu::{fast, float};

/// A VU vector register: four PS2 floats (or integers) as raw bits, x..w.
pub type V = [u32; 4];

const ONE: u32 = 0x3f80_0000;
/// `vf0` (always (0, 0, 0, 1)).
const VF0: V = [0, 0, 0, ONE];
const ZERO: V = [0; 4];

/// Hash of the program this module implements.
pub const HASH: u64 = 0xd159_716b_b128_1fbe;

/// Register state the program carries from one call to the next.
///
/// Type 3 keeps everything that crosses calls in data memory (qw33 fan tag
/// toggle and return address, qw34 mesh flags, qw6-13 mesh matrices, the
/// previous output buffer for pass 2). No VU register is read before it is
/// written in the implemented entries, so this is empty; it exists so the
/// API does not change when an entry that needs one is added.
#[derive(Clone, Debug, Default)]
pub struct VuState {}

/// Entries [`run`] implements natively (MSCAL byte addresses).
pub fn implemented(entry: u32) -> bool {
    matches!(
        entry,
        0x12b8 | 0x3a08 | 0x2400 | 0x3bf0 | 0x2270 | 0x22c8 | 0x2320 | 0x2358
    )
}

/// Run the program at `entry` with VIF `top`/`itop` on VU1 data memory
/// `mem`, handing every XGKICK packet to `kick`. Returns `false` (and does
/// nothing) when the entry, or the variant selected by the input, is not
/// implemented natively.
pub fn run(
    entry: u32,
    top: u32,
    itop: u32,
    mem: &mut [u8; 16384],
    _regs: &mut VuState,
    kick: &mut dyn FnMut(&[u128]),
) -> bool {
    let top = (top & 0x3ff) as u16;
    let mut m = Mem(mem);
    match entry {
        0x12b8 => float_strip(&mut m, top, kick),
        0x3a08 => ribbon(&mut m, top, kick),
        0x2400 => {
            // PRIM-only kick: the GIF tag the EE left at qw 0x1f.
            kick(&m.gif_packet(0x1f));
            true
        }
        0x3bf0 => line_shape(&mut m, top, kick),
        0x2270 | 0x22c8 | 0x2320 | 0x2358 => mesh_entry(&mut m, entry, top, itop, kick),
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Memory
// ---------------------------------------------------------------------------

/// VU1 data memory: 1024 qwords, addressed by qword index (wrapping).
struct Mem<'a>(&'a mut [u8; 16384]);

impl Mem<'_> {
    fn off(a: u16) -> usize {
        (a as usize & 0x3ff) * 16
    }
    fn ld(&self, a: u16) -> V {
        let o = Self::off(a);
        std::array::from_fn(|l| {
            u32::from_le_bytes(self.0[o + 4 * l..o + 4 * l + 4].try_into().unwrap())
        })
    }
    /// Store the lanes of `v` selected by `mask` (bit 3 = x .. bit 0 = w).
    fn st_mask(&mut self, a: u16, v: V, mask: u8) {
        let o = Self::off(a);
        for (l, x) in v.iter().enumerate() {
            if mask & (8 >> l) != 0 {
                self.0[o + 4 * l..o + 4 * l + 4].copy_from_slice(&x.to_le_bytes());
            }
        }
    }
    fn st(&mut self, a: u16, v: V) {
        self.st_mask(a, v, 0b1111);
    }
    /// ILW: the low 16 bits of one lane.
    fn ilw(&self, a: u16, lane: usize) -> u16 {
        self.ld(a)[lane] as u16
    }
    /// ISW: a VI register, zero-extended, into one lane.
    fn isw(&mut self, a: u16, lane: usize, v: u16) {
        self.st_mask(a, [v as u32; 4], 8 >> lane);
    }
    /// The GIF packet XGKICK sends from qword `a`: tags up to EOP (as
    /// `Vu::gif_packet`).
    fn gif_packet(&self, a: u16) -> Vec<u128> {
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

fn add(a: V, b: V) -> V {
    fast::add4::<0b1111, true, true>(&a, &b).0
}
fn sub(a: V, b: V) -> V {
    fast::sub4::<0b1111, true, true>(&a, &b).0
}
fn mul(a: V, b: V) -> V {
    fast::mul4::<0b1111, true, true>(&a, &b).0
}
/// `acc + a * b`.
fn madd(acc: V, a: V, b: V) -> V {
    fast::madd4::<0b1111, true, true, true>(&acc, &a, &b).0
}
/// `acc - a * b`.
fn msub(acc: V, a: V, b: V) -> V {
    fast::msub4::<0b1111, true, true, true>(&acc, &a, &b).0
}
fn max(a: V, b: V) -> V {
    fast::max4(&a, &b)
}
fn min(a: V, b: V) -> V {
    fast::min4(&a, &b)
}
fn ftoi0(a: V) -> V {
    fast::ftoi4::<0>(&a)
}
fn ftoi4(a: V) -> V {
    fast::ftoi4::<4>(&a)
}
fn ftoi12(a: V) -> V {
    fast::ftoi4::<12>(&a)
}
fn itof0(a: V) -> V {
    fast::itof4::<0>(&a)
}
fn itof12(a: V) -> V {
    fast::itof4::<12>(&a)
}
fn itof15(a: V) -> V {
    fast::itof4::<15>(&a)
}
fn bc(x: u32) -> V {
    [x; 4]
}
/// `dst` with the lanes in `mask` (bit 3 = x) taken from `src`.
fn merge(dst: V, src: V, mask: u8) -> V {
    std::array::from_fn(|l| if mask & (8 >> l) != 0 { src[l] } else { dst[l] })
}
/// DIV: `a / b` as the DIV unit computes it.
fn div(a: u32, b: u32) -> u32 {
    float::div(a, b).value
}

/// `M·p` as MULAx / MADDAy / MADDAz / MADDw (columns `m[0..4]`).
fn xform_xyzw(m: &[V; 4], p: V) -> V {
    let acc = mul(m[0], bc(p[0]));
    let acc = madd(acc, m[1], bc(p[1]));
    let acc = madd(acc, m[2], bc(p[2]));
    madd(acc, m[3], bc(p[3]))
}

/// The CLIP flag register: the judgement bits of the last 4 CLIPs, newest
/// in bits 0-5 (+x -x +y -y +z -z).
#[derive(Clone, Copy, Default)]
struct ClipFlags(u32);

impl ClipFlags {
    /// CLIPw.xyz v, v.w.
    fn clip(&mut self, v: V) {
        self.0 = ((self.0 << 6) | fast::clip(v, v[3])) & 0xff_ffff;
    }
    fn fcand(self, imm: u32) -> bool {
        self.0 & imm & 0xff_ffff != 0
    }
    fn fcor(self, imm: u32) -> bool {
        (self.0 | imm) & 0xff_ffff == 0xff_ffff
    }
    /// The strip triangle test (0x14d0 / 0x3198) for the last 3 vertices:
    /// true = drop it outright (any vertex beyond the far plane, or all
    /// three outside one plane); false = it needs the clipper.
    fn triangle_dropped(self) -> bool {
        self.fcand(0x01_0410)
            || self.fcor(0xfd_f7df)
            || self.fcor(0xff_7df7)
            || self.fcor(0xff_befb)
            || self.fcor(0xff_df7d)
            || self.fcor(0xff_efbe)
    }
}

// ---------------------------------------------------------------------------
// Memory layout (qword addresses)
// ---------------------------------------------------------------------------

/// M: model → guard-clip matrix columns (strips).
const MATRIX: u16 = 0;
/// guardScale (gw/2, -gh/2, -8388467.5, 0).
const GUARD_SCALE: u16 = 4;
/// guardOffset (cx+2048, cy+2048, 8388467.5, 0).
const GUARD_OFFSET: u16 = 5;
/// .w: the fan tag the clipper used last (0x3c8 / 0x3e4, alternating).
const FAN_TOGGLE: u16 = 33;
/// The strip loop spills its accumulator here around the clipper.
const ACC_SPILL: u16 = 37;
/// Clipper polygon buffers (up to 9 vertices × {pos, ST, RGBA}).
const CLIP_BUF_A: u16 = 0x392;
const CLIP_BUF_B: u16 = 0x3ad;
/// The two fan packets (GIF tag + up to 9 vertices × 3 qw).
const FAN_TAG_A: u16 = 0x3c8;
const FAN_TAG_B: u16 = 0x3e4;
/// Output buffer offset from TOP.
const OUT: u16 = 0xdb;

fn guard(m: &Mem) -> (V, V) {
    (m.ld(GUARD_SCALE), m.ld(GUARD_OFFSET))
}

// ---------------------------------------------------------------------------
// 0x12b8: float triangle strip (slot 71, ribbons)
// ---------------------------------------------------------------------------

/// Input at TOP: qw0 GIF tag, qw1.x = 3n, then n × {ST (S,T,1,0) f32,
/// RGBA i32, XYZ1 f32}. Output at TOP+0xdb: the tag, then per vertex
/// {STQ, RGBA, XYZ2 (ADC for the first two and for culled triangles)};
/// triangles crossing the guard band are clipped and kicked first as fans.
fn float_strip(m: &mut Mem, top: u16, kick: &mut dyn FnMut(&[u128])) -> bool {
    let n3 = m.ilw(top.wrapping_add(1), 0);
    // The loop is a do-while ending when the input pointer reaches the end:
    // fewer than 3 vertices or a count not a multiple of 3 never ends.
    if n3 < 9 || !n3.is_multiple_of(3) {
        return false;
    }
    let mat = [
        m.ld(MATRIX),
        m.ld(MATRIX + 1),
        m.ld(MATRIX + 2),
        m.ld(MATRIX + 3),
    ];
    let (gs, go) = guard(m);
    let out = top.wrapping_add(OUT);
    let end = top.wrapping_add(n3).wrapping_add(9);

    // Prologue: the first two vertices transformed x, y, z, w.
    let mut cur = xform_xyzw(&mat, m.ld(top.wrapping_add(4))); // vf12
    let mut next = xform_xyzw(&mat, m.ld(top.wrapping_add(7))); // vf13
    let mut st = m.ld(top.wrapping_add(2)); // vf10: ST of the current vertex
    let mut col = m.ld(top.wrapping_add(3)); // vf8: RGBA of the current vertex
    let mut q = div(ONE, cur[3]);
    m.st(out, m.ld(top));

    let mut clip = ClipFlags::default();
    let mut src = top.wrapping_add(9); // vi4
    let mut dst = out.wrapping_add(1); // vi6
    let mut k = 0u32;
    loop {
        let p2 = m.ld(src.wrapping_add(1)); // position two vertices ahead
        let cq = mul(cur, bc(q));
        let stq = mul(st, bc(q));
        st = m.ld(src.wrapping_sub(4));
        src = src.wrapping_add(3);
        clip.clip(cur);
        let q_next = div(ONE, next[3]);
        let screen = madd(mul(go, bc(ONE)), cq, gs);
        dst = dst.wrapping_add(2);
        m.st(dst.wrapping_sub(2), stq);
        cur = next;
        m.st(dst.wrapping_sub(1), col);
        // The next-but-one vertex, transformed w, z, y, x (with w = 1).
        let mut acc = mul(mat[3], bc(ONE));
        acc = madd(acc, mat[2], bc(p2[2]));
        let mut xyz = if k < 2 {
            let f = ftoi4(screen);
            [f[0], f[1], f[2], 0xffff_ffff]
        } else {
            ftoi4(screen)
        };
        if k >= 2 && clip.fcand(0x3_ffff) {
            // Some vertex of this triangle is outside the guard band: no
            // drawing kick for it (ADC), and clip it unless it is dropped.
            xyz[3] = 0xffff_ffff;
            if !clip.triangle_dropped() {
                // Spill the accumulator (ACC + vf0·0) around the clipper.
                let spill = madd(acc, VF0, bc(VF0[0]));
                m.st(ACC_SPILL, spill);
                let pos = |d: u16| m.ld(src.wrapping_sub(d));
                let (pa, pb, pc) = (pos(14), pos(11), pos(8));
                let (sa, sb, sc) = (pos(16), pos(13), pos(10));
                let ca = itof0(m.ld(dst.wrapping_sub(7)));
                let cb = itof0(m.ld(dst.wrapping_sub(4)));
                let cc = itof0(m.ld(dst.wrapping_sub(1)));
                let a = xform_xyzw(&mat, pa);
                let b = xform_xyzw(&mat, pb);
                let c = xform_xyzw(&mat, pc);
                for (i, v) in [a, sa, ca, b, sb, cb, c, sc, cc].into_iter().enumerate() {
                    m.st(CLIP_BUF_A + i as u16, v);
                }
                clip_triangle(m, &mut clip, gs, go, kick);
                clip.clip(b);
                clip.clip(c);
                acc = mul(m.ld(ACC_SPILL), bc(ONE));
            }
        }
        col = m.ld(src.wrapping_sub(6));
        acc = madd(acc, mat[1], bc(p2[1]));
        next = madd(acc, mat[0], bc(p2[0]));
        m.st(dst, xyz);
        dst = dst.wrapping_add(1);
        q = q_next;
        k += 1;
        if k >= 3 && src == end {
            break;
        }
    }
    kick(&m.gif_packet(out));
    true
}

/// [`float_strip`] without its output: the clip side only, for the GPU
/// strips. The same transforms in the same order (the CLIP flags decide
/// which triangles are culled, to the last bit), the same clipper calls
/// with their memory effects (polygon buffers, fan packets, fan tag toggle,
/// accumulator spill) and fan kicks; returns, per vertex, its clip-space
/// position (as the program computes it) and whether the program gives it
/// ADC (the first two, and the last vertex of a culled triangle). The
/// clipper takes the triangle's colours from the program's output, which
/// are the input's (`n` ≤ 72 vertices: the output never reaches unread
/// input).
fn float_strip_clip(
    m: &mut Mem,
    top: u16,
    n: u16,
    kick: &mut dyn FnMut(&[u128]),
    fanned: &mut Vec<bool>,
) -> Vec<(V, bool)> {
    fanned.clear();
    fanned.resize(n as usize, false);
    let mat = [
        m.ld(MATRIX),
        m.ld(MATRIX + 1),
        m.ld(MATRIX + 2),
        m.ld(MATRIX + 3),
    ];
    let (gs, go) = guard(m);
    let mut cur = xform_xyzw(&mat, m.ld(top.wrapping_add(4)));
    let mut next = xform_xyzw(&mat, m.ld(top.wrapping_add(7)));
    let mut clip = ClipFlags::default();
    let mut src = top.wrapping_add(9);
    let mut out = Vec::with_capacity(n as usize);
    for k in 0..n {
        let p2 = m.ld(src.wrapping_add(1));
        src = src.wrapping_add(3);
        clip.clip(cur);
        out.push((cur, true));
        cur = next;
        let mut acc = mul(mat[3], bc(ONE));
        acc = madd(acc, mat[2], bc(p2[2]));
        if k >= 2 {
            let culled = clip.fcand(0x3_ffff);
            out[k as usize].1 = culled;
            if culled && !clip.triangle_dropped() {
                fanned[k as usize] = true;
                let spill = madd(acc, VF0, bc(VF0[0]));
                m.st(ACC_SPILL, spill);
                let pos = |d: u16| m.ld(src.wrapping_sub(d));
                let (pa, pb, pc) = (pos(14), pos(11), pos(8));
                let (sa, sb, sc) = (pos(16), pos(13), pos(10));
                let (ca, cb, cc) = (itof0(pos(15)), itof0(pos(12)), itof0(pos(9)));
                let a = xform_xyzw(&mat, pa);
                let b = xform_xyzw(&mat, pb);
                let c = xform_xyzw(&mat, pc);
                for (i, v) in [a, sa, ca, b, sb, cb, c, sc, cc].into_iter().enumerate() {
                    m.st(CLIP_BUF_A + i as u16, v);
                }
                clip_triangle(m, &mut clip, gs, go, kick);
                clip.clip(b);
                clip.clip(c);
                acc = mul(m.ld(ACC_SPILL), bc(ONE));
            }
        }
        acc = madd(acc, mat[1], bc(p2[1]));
        next = madd(acc, mat[0], bc(p2[0]));
    }
    out
}

// ---------------------------------------------------------------------------
// 0x1630: the near/guard-band clipper
// ---------------------------------------------------------------------------

/// One clip plane `d(v) = v.w ± v[lane] >= 0`.
struct Plane {
    lane: usize,
    /// true: `w + v[lane]` (the minus side), false: `w - v[lane]`.
    minus: bool,
}

impl Plane {
    /// CLIP judgement bit of this plane for the newest vertex.
    fn bit(&self) -> u32 {
        1 << (self.lane * 2 + self.minus as usize)
    }
    /// `d(v)`: ADDz.w / ADDx.w / SUBx.w / SUBy.w / ADDy.w of the original.
    fn dist(&self, v: V) -> u32 {
        if self.minus {
            float::add(v[3], v[self.lane]).0
        } else {
            float::sub(v[3], v[self.lane]).0
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

/// Clip the triangle in CLIP_BUF_A (3 × {pos, ST, RGBA float}) against the
/// 5 planes (Sutherland-Hodgman, ping-ponging between the two buffers),
/// then kick the result as a fan through one of the two fan packets.
///
/// Per edge a→b (the first edge of a plane runs from the last vertex to the
/// first): both inside → emit b; leaving → emit the intersection; entering →
/// emit the intersection, then b. The intersection is
/// `a + (b - a)·t`, `t = d(a) / d(a - b)`; ST is interpolated raw (its z, w
/// forced to 1, so Q comes out as 1/w at the fan), RGBA as float.
fn clip_triangle(m: &mut Mem, clip: &mut ClipFlags, gs: V, go: V, kick: &mut dyn FnMut(&[u128])) {
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
        let mut d = dst_buf; // output pointer
        let mut left = count;
        loop {
            let b_st = m.ld(ib.wrapping_add(1));
            let a_st = m.ld(ia.wrapping_add(1));
            let b_col = m.ld(ib.wrapping_add(2));
            let a_col = m.ld(ia.wrapping_add(2));
            let b_st1 = merge(b_st, max(VF0, bc(VF0[3])), 0b0011);
            if !clip.fcor(0xff_ffff & !(bit_a | bit_b)) {
                let at = d;
                if !clip.fcand(bit_a | bit_b) {
                    // Both inside: emit b.
                    d = d.wrapping_add(3);
                    m.st(at, b);
                    m.st(at.wrapping_add(1), b_st1);
                    m.st(at.wrapping_add(2), b_col);
                } else {
                    let t = div(plane.dist(a), plane.dist(sub(a, b)));
                    let d_col = sub(b_col, a_col);
                    let d_st = merge(b_st1, sub(b_st1, a_st), 0b1100);
                    if clip.fcand(bit_a) {
                        // Entering: b goes after the intersection.
                        m.st(d.wrapping_add(5), b_col);
                        m.st(d.wrapping_add(4), b_st1);
                        m.st(d.wrapping_add(3), b);
                        d = d.wrapping_add(3);
                    }
                    let p = madd(mul(a, bc(ONE)), sub(b, a), bc(t));
                    let s = merge(d_st, madd(mul(a_st, bc(ONE)), d_st, bc(t)), 0b1100);
                    let c = madd(mul(a_col, bc(ONE)), d_col, bc(t));
                    d = d.wrapping_add(3);
                    m.st(at, p);
                    m.st(at.wrapping_add(1), s);
                    m.st(at.wrapping_add(2), c);
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
    emit_fan(m, src_buf, count, gs, go, kick);
}

/// Kick the clipped polygon at `buf` (`qw` qwords) as a fan through the
/// fan packet not used last time.
fn emit_fan(m: &mut Mem, buf: u16, qw: u16, gs: V, go: V, kick: &mut dyn FnMut(&[u128])) {
    let tag = if m.ilw(FAN_TOGGLE, 3) == FAN_TAG_A {
        FAN_TAG_B
    } else {
        FAN_TAG_A
    };
    m.isw(FAN_TOGGLE, 3, tag);
    let mut o = tag.wrapping_add(1);
    let mut n = 0u16;
    for i in (0..qw).step_by(3) {
        let p = m.ld(buf.wrapping_add(i));
        let st = m.ld(buf.wrapping_add(i + 1));
        let col = m.ld(buf.wrapping_add(i + 2));
        let q = div(ONE, p[3]);
        let screen = madd(mul(go, bc(ONE)), mul(p, bc(q)), gs);
        m.st(o, mul(st, bc(q)));
        m.st(o.wrapping_add(1), ftoi0(col));
        m.st(o.wrapping_add(2), ftoi4(screen));
        o = o.wrapping_add(3);
        n += 1;
    }
    m.isw(tag, 0, 0x8000 | n);
    kick(&m.gif_packet(tag));
}

// ---------------------------------------------------------------------------
// 0x3a08: ribbon rebuild (wake, shadow strips) → 0x12b8
// ---------------------------------------------------------------------------

/// Input at TOP: qw0 GIF tag, qw1 {3m, m, Δα, α0}, then edge A and edge B
/// (m × {ST, RGBA, XYZ} each). Interleaves them in place into the strip
/// A0 B0 A1 B1 ... with alpha_k = a · clamp(α0 + kΔα, 0, 1), patches the
/// count to 6m and runs the float strip.
fn ribbon(m: &mut Mem, top: u16, kick: &mut dyn FnMut(&[u128])) -> bool {
    if ribbon_pairs(m, top).is_none() {
        return false;
    }
    ribbon_rebuild(m, top);
    float_strip(m, top, kick)
}

/// The ribbon's pair count m, when the input is one [`ribbon_rebuild`]
/// handles: as the game uses it, count3 = 3m, 2 ≤ m ≤ 36 (the strip that
/// follows needs ≥ 3 vertices; edge A's copy at TOP+0xdb must not reach
/// edge B; with these, the in-place interleave never overwrites unread
/// input).
fn ribbon_pairs(m: &Mem, top: u16) -> Option<u16> {
    let hdr = m.ld(top.wrapping_add(1));
    let (count3, pairs) = (hdr[0] as u16, hdr[1] as u16);
    ((2..=36).contains(&pairs) && count3 == 3 * pairs).then_some(pairs)
}

/// The ribbon's rebuild (0x3a08 up to the float strip): the interleave in
/// place with the alpha ramp, the count patched to 6m (a valid input,
/// [`ribbon_pairs`]).
fn ribbon_rebuild(m: &mut Mem, top: u16) {
    let hdr = m.ld(top.wrapping_add(1));
    let count3 = hdr[0] as u16; // vi3 = 3m
    let pairs = hdr[1] as u16; // vi4 = m
    let (d_alpha, alpha0) = (hdr[2], hdr[3]);
    let scratch = top.wrapping_add(OUT);

    // Copy edge A out of the way (to the output buffer area).
    for i in 0..pairs {
        for j in 0..3 {
            let v = m.ld(top.wrapping_add(2 + 3 * i + j));
            m.st(scratch.wrapping_add(3 * i + j), v);
        }
    }

    let mut alpha = alpha0; // vf1.w
    let clamp = |x: u32| min(max(bc(x), ZERO), bc(ONE))[0];
    let mut factor = clamp(max(bc(alpha), bc(alpha))[0]);
    let scale = |rgba: V, f: u32| -> V {
        let a = itof0(rgba)[3];
        let a = float::mul(a, f).0;
        [rgba[0], rgba[1], rgba[2], ftoi0(bc(a))[0]]
    };
    let mut o = top.wrapping_add(2);
    for i in 0..pairs {
        let a_st = m.ld(scratch.wrapping_add(3 * i));
        let a_col = m.ld(scratch.wrapping_add(3 * i + 1));
        let a_xyz = m.ld(scratch.wrapping_add(3 * i + 2));
        let b = top.wrapping_add(count3).wrapping_add(3 * i);
        let b_st = m.ld(b.wrapping_add(2));
        let b_col = m.ld(b.wrapping_add(3));
        let b_xyz = m.ld(b.wrapping_add(4));
        alpha = float::add(alpha, d_alpha).0;
        m.st(o, a_st);
        m.st(o.wrapping_add(2), a_xyz);
        m.st(o.wrapping_add(3), b_st);
        m.st(o.wrapping_add(5), b_xyz);
        m.st(o.wrapping_add(1), scale(a_col, factor));
        m.st(o.wrapping_add(4), scale(b_col, factor));
        factor = clamp(alpha);
        o = o.wrapping_add(6);
    }
    m.isw(top.wrapping_add(1), 0, count3.wrapping_add(count3));
}

// ---------------------------------------------------------------------------
// 0x2270 / 0x22c8 / 0x2320 / 0x2358: meshes (models, sky, grid meshes)
// ---------------------------------------------------------------------------

/// Mesh flags (qw34.w, copied from header qw4.w by the setup entries).
mod flag {
    /// Bit 0: SH lighting (colour computed from the normal).
    pub const LIT: u16 = 0x01;
    /// Bits 1-4: pass-1 texgen (2 UV matrix, 4 planar xy, 8 normal → UV,
    /// 0x10 sphere map).
    pub const TEXGEN1: u16 = 0x1e;
    /// Bit 5: the mesh may cross the guard band (cull + clip per triangle).
    pub const CLIP: u16 = 0x20;
    /// Bits 6-9: pass-2 texgen (0x40 UV matrix, 0x80 planar, 0x100 normal →
    /// UV, 0x200 sphere map).
    pub const TEXGEN2: u16 = 0x3c0;
}

/// The mesh header (setup entries, at TOP): model matrix columns qw0-3,
/// flags at qw4.w, the texgen matrix ("matrix 2") qw5-8. The mesh itself
/// follows at TOP+0x1b (setup entries) or TOP (the others).
const MESH_AFTER_HEADER: u16 = 0x1b;
/// M × model (written by the setup, read by every mesh call).
const MESH_MATRIX: u16 = 6;
/// The texgen matrix: rows A0, A1 (qw10, 11), A2/b0 (qw12), b (qw13).
const TEXGEN_MATRIX: u16 = 10;
/// SH lighting coefficients K0..K9 ×128 (qw15.w = 128: the alpha).
const LIGHT_COEFFS: u16 = 15;
/// Lighting basis B (3 rows).
const LIGHT_BASIS: u16 = 25;
/// .y: the pass-1 / pass-2 strip PRIM (GIF tag word 1).
const PASS2_TAG_Y: u16 = 31;
/// .y: the PRIM the pass-2 fans use.
const PASS2_FAN_TAG_Y: u16 = 32;
/// .x: the saved return address of the mesh routines.
const MESH_LINK: u16 = 33;
/// .w: the mesh flags.
const MESH_FLAGS: u16 = 34;
/// .y: the fan tags' PRIM words, saved while pass 2 patches them.
const FAN_TAG_A_SAVE: u16 = 35;
const FAN_TAG_B_SAVE: u16 = 36;

/// A mesh in VU memory: tag, {nLen, _, n}, nLen strip lengths, then n
/// positions (int16 → ITOF15), n UVs (4.12), n RGBA or normals (ITOF15).
#[derive(Clone, Copy)]
struct Mesh {
    base: u16,
    /// Number of strips (one length qword each).
    strips: u16,
    /// Number of vertices.
    n: u16,
}

impl Mesh {
    fn read(m: &Mem, base: u16) -> Mesh {
        let h = m.ld(base.wrapping_add(1));
        Mesh {
            base,
            strips: h[0] as u16,
            n: h[2] as u16,
        }
    }
    fn lengths(&self) -> u16 {
        self.base.wrapping_add(2)
    }
    fn xyz(&self) -> u16 {
        self.lengths().wrapping_add(self.strips)
    }
    fn uv(&self) -> u16 {
        self.xyz().wrapping_add(self.n)
    }
    fn colour(&self) -> u16 {
        self.uv().wrapping_add(self.n)
    }
    /// Every loop in the mesh code is a do-while ending on pointer
    /// equality: they need ≥ 3 vertices per strip and in total.
    fn well_formed(&self, m: &Mem) -> bool {
        if self.n < 3 || self.strips == 0 || self.strips > 0x400 {
            return false;
        }
        (0..self.strips).all(|i| m.ilw(self.lengths().wrapping_add(i), 0) >= 3)
    }
}

/// Which mesh routine an entry runs (`link` = the BAL return address the
/// routine saves at qw33.x).
/// Which matrix a call's vertices go through: `Some(true)` a mesh setup
/// (qw6-9 = qw0-3 × the model matrix at TOP qw0-3, then drawn with it),
/// `Some(false)` a mesh call drawn with the qw6-9 an earlier setup left
/// (or the EE uploaded), `None` the strip entries (qw0-3).
pub fn mesh_matrix_use(entry: u32, itop: u32) -> Option<bool> {
    match (entry, itop != 0) {
        (0x2270, false) | (0x2320, _) => Some(true),
        (0x22c8, _) | (0x2358, _) | (0x2270, true) => Some(false),
        _ => None,
    }
}

fn mesh_entry(m: &mut Mem, entry: u32, top: u16, itop: u32, kick: &mut dyn FnMut(&[u128])) -> bool {
    let after_header = top.wrapping_add(MESH_AFTER_HEADER);
    match (entry, itop != 0) {
        (0x2270, false) | (0x2320, _) => mesh_pass1(m, top, after_header, true, 0x468, kick),
        (0x22c8, false) | (0x2358, _) => mesh_pass1(m, top, top, false, 0x46e, kick),
        (0x2270, true) => mesh_pass2(m, top, after_header, 0x476, kick),
        (0x22c8, true) => mesh_pass2(m, top, top, 0x47c, kick),
        _ => false,
    }
}

/// Pass 1 (ITOP == 0): texgen, transform, ctx1 strips (+ clipped fans).
fn mesh_pass1(
    m: &mut Mem,
    top: u16,
    base: u16,
    setup: bool,
    link: u16,
    kick: &mut dyn FnMut(&[u128]),
) -> bool {
    let flags = if setup {
        m.ilw(base.wrapping_sub(23), 3)
    } else {
        m.ilw(MESH_FLAGS, 3)
    };
    let mesh = Mesh::read(m, base);
    // Not implemented natively: the pass-1 sphere map. (It reads the
    // positions through vi11, which pass 1 never sets: a pointer left over
    // from the previous call.)
    if flags & flag::TEXGEN1 == 0x10 || !mesh.well_formed(m) {
        return false;
    }
    let lit = flags & flag::LIT != 0;
    if setup {
        m.isw(MESH_FLAGS, 3, flags);
        if lit {
            lighting_setup(m, top);
        }
        // qw6-9 = M × model, qw10-13 = the texgen matrix.
        let mat = [
            m.ld(MATRIX),
            m.ld(MATRIX + 1),
            m.ld(MATRIX + 2),
            m.ld(MATRIX + 3),
        ];
        let model: [V; 4] = std::array::from_fn(|i| m.ld(top.wrapping_add(i as u16)));
        let tex: [V; 4] = std::array::from_fn(|i| m.ld(top.wrapping_add(5 + i as u16)));
        for i in 0..4 {
            m.st(MESH_MATRIX + i as u16, xform_xyzw(&mat, model[i]));
            m.st(TEXGEN_MATRIX + i as u16, tex[i]);
        }
    }
    m.isw(MESH_LINK, 0, link);
    // Pass-1 texgen (0x1f40); other values of the field mean none.
    match flags & flag::TEXGEN1 {
        2 => texgen_uv_matrix(m, mesh.uv(), mesh.uv(), mesh.n),
        4 => texgen_uv_matrix(m, mesh.xyz(), mesh.uv(), mesh.n),
        8 => texgen_normal(m, mesh.colour(), mesh.uv(), mesh.n),
        _ => {}
    }
    let out = top.wrapping_add(OUT);
    if lit {
        // Normals (in the colour slot) → RGBA, straight into the output.
        light(m, mesh.colour(), out.wrapping_add(2), mesh.n);
    }

    m.st(out, m.ld(base));
    let mat = [
        m.ld(MESH_MATRIX),
        m.ld(MESH_MATRIX + 1),
        m.ld(MESH_MATRIX + 2),
        m.ld(MESH_MATRIX + 3),
    ];
    let (gs, mut go) = guard(m);
    go[3] = ONE; // MOVE.w vf6, vf0
    if flags & flag::CLIP != 0 {
        mesh_strips_clipped(m, &mesh, out.wrapping_add(1), &mat, gs, go, !lit, kick);
    } else {
        mesh_strips(m, &mesh, out.wrapping_add(1), &mat, gs, go, !lit);
    }
    kick(&m.gif_packet(out));
    true
}

/// Pass 2 (ITOP != 0): new ST from the pass-2 texgen into the pass-1
/// output, ctx2 tag, re-kick. Clipped meshes are re-run through the strip
/// clipper (fans patched to the ctx2 PRIM).
fn mesh_pass2(m: &mut Mem, top: u16, base: u16, link: u16, kick: &mut dyn FnMut(&[u128])) -> bool {
    let flags = m.ilw(MESH_FLAGS, 3);
    let mesh = Mesh::read(m, base);
    if !mesh.well_formed(m) {
        return false;
    }
    m.isw(MESH_LINK, 0, link);
    // Pass-2 texgen (0x2010), into the UV array in place.
    match flags & flag::TEXGEN2 {
        0x40 => texgen_uv_matrix(m, mesh.uv(), mesh.uv(), mesh.n),
        0x80 => texgen_uv_matrix(m, mesh.xyz(), mesh.uv(), mesh.n),
        0x100 => texgen_normal(m, mesh.colour(), mesh.uv(), mesh.n),
        0x200 => texgen_sphere(m, mesh.xyz(), mesh.colour(), mesh.uv(), mesh.n),
        _ => {}
    }
    let out = top.wrapping_add(OUT);
    let tag = m.ld(base);
    m.st(out, [tag[0], m.ld(PASS2_TAG_Y)[1], tag[2], tag[3]]);
    if flags & flag::CLIP != 0 {
        let mat = [
            m.ld(MESH_MATRIX),
            m.ld(MESH_MATRIX + 1),
            m.ld(MESH_MATRIX + 2),
            m.ld(MESH_MATRIX + 3),
        ];
        let (gs, go) = guard(m);
        let fan_y = m.ld(PASS2_FAN_TAG_Y)[1];
        let (save_a, save_b) = (m.ld(FAN_TAG_A)[1], m.ld(FAN_TAG_B)[1]);
        m.st_mask(FAN_TAG_A, bc(fan_y), 0b0100);
        m.st_mask(FAN_TAG_B, bc(fan_y), 0b0100);
        m.st_mask(FAN_TAG_A_SAVE, bc(save_a), 0b0100);
        m.st_mask(FAN_TAG_B_SAVE, bc(save_b), 0b0100);
        mesh_strips_clipped(m, &mesh, out.wrapping_add(1), &mat, gs, go, false, kick);
        let (save_a, save_b) = (m.ld(FAN_TAG_A_SAVE)[1], m.ld(FAN_TAG_B_SAVE)[1]);
        kick(&m.gif_packet(out));
        m.st_mask(FAN_TAG_A, bc(save_a), 0b0100);
        m.st_mask(FAN_TAG_B, bc(save_b), 0b0100);
    } else {
        // STQ = (u, v, 1, 1) · q, with q from the pass-1 STQ.w.
        for i in 0..mesh.n {
            let slot = out.wrapping_add(1 + 3 * i);
            let q = m.ld(slot)[3];
            let uv = m.ld(mesh.uv().wrapping_add(i));
            let u = merge([0, 0, ONE, ONE], itof12(uv), 0b1100);
            m.st(slot, mul(u, bc(q)));
        }
        kick(&m.gif_packet(out));
    }
    true
}

/// Texgen "UV matrix" (0x0be8): `uv' = A·uv + b`, A = rows qw10.xy,
/// qw11.xy, b = qw12.xy + qw13.xy; written back as 4.12 with zw = 1.0.
/// "Planar" is the same routine fed with the positions' xy (as 4.12).
fn texgen_uv_matrix(m: &mut Mem, src: u16, dst: u16, n: u16) {
    let a0 = m.ld(TEXGEN_MATRIX);
    let a1 = m.ld(TEXGEN_MATRIX + 1);
    let b = add(m.ld(TEXGEN_MATRIX + 2), m.ld(TEXGEN_MATRIX + 3));
    for i in 0..n {
        let uv = itof12(m.ld(src.wrapping_add(i)));
        let r = madd(madd(mul(a0, bc(uv[0])), a1, bc(uv[1])), b, bc(ONE));
        let f = ftoi12(r);
        m.st(dst.wrapping_add(i), [f[0], f[1], ONE, ONE]);
    }
}

/// Texgen "normal → UV" (0x0ce8): `uv' = A·n + b`, A = qw10-12.xy,
/// b = qw13.xy; 4.12 with zw = 1.0.
fn texgen_normal(m: &mut Mem, src: u16, dst: u16, n: u16) {
    let a: [V; 4] = std::array::from_fn(|i| m.ld(TEXGEN_MATRIX + i as u16));
    for i in 0..n {
        let nrm = itof15(m.ld(src.wrapping_add(i)));
        let r = madd(
            madd(
                madd(mul(a[0], bc(nrm[0])), a[1], bc(nrm[1])),
                a[2],
                bc(nrm[2]),
            ),
            a[3],
            bc(ONE),
        );
        let f = ftoi12(r);
        m.st(dst.wrapping_add(i), [f[0], f[1], ONE, ONE]);
    }
}

/// Texgen "sphere map" (0x3400): with the eye-space position
/// `e = S·p + t·p.w` (S = qw10-12.xyz scaled per column by qw10-12.w,
/// t = qw13.xyz) and normal `N = A·n` (A = qw10-12.xyz): `I = e/|e|`,
/// `R = I - 2N(N·I)`, `R' = R + (0, 0, 1)`, `uv = 0.5·R'.xy/|R'| + 0.5`;
/// 4.12 with zw = 1.0.
fn texgen_sphere(m: &mut Mem, pos: u16, normals: u16, dst: u16, n: u16) {
    let a: [V; 3] = std::array::from_fn(|i| m.ld(TEXGEN_MATRIX + i as u16));
    let t = m.ld(TEXGEN_MATRIX + 3);
    let scale = [
        float::add(VF0[0], a[0][3]).0,
        float::add(VF0[1], a[1][3]).0,
        float::add(VF0[2], a[2][3]).0,
        0,
    ];
    let s: [V; 3] = std::array::from_fn(|i| mul(a[i], scale));
    let half = bc(0x3f00_0000);
    // |v|² as the program forms it: (x² + y²) + 1·z².
    let len2 = |sq: V| float::madd(float::add(sq[0], sq[1]).0, ONE, sq[2]).0;
    for i in 0..n {
        let p = m.ld(pos.wrapping_add(i));
        let pf = itof15(p);
        let e = madd(
            madd(madd(mul(s[0], bc(pf[0])), s[1], bc(pf[1])), s[2], bc(pf[2])),
            t,
            bc(p[3]),
        );
        let eye = mul(e, bc(float::rsqrt(ONE, len2(mul(e, e))).value));
        let nf = itof15(m.ld(normals.wrapping_add(i)));
        let nrm = madd(madd(mul(a[0], bc(nf[0])), a[1], bc(nf[1])), a[2], bc(nf[2]));
        let dot = len2(mul(nrm, eye));
        let r = msub(add(ZERO, eye), add(nrm, nrm), bc(dot));
        let r = [r[0], r[1], float::add(r[2], ONE).0, r[3]];
        let q = float::rsqrt(ONE, len2(mul(r, r))).value;
        let uv = add(mul(mul(r, half), bc(q)), half);
        let f = ftoi12(uv);
        m.st(dst.wrapping_add(i), [f[0], f[1], ONE, ONE]);
    }
}

/// Lighting setup (0x2170, lit meshes): from the header at TOP, the basis
/// B (qw9-11) goes to qw25-27 and the 10 SH coefficient vectors K0..K9
/// (qw15-24, xyz) are stored ×128 to qw15-24 with w = 0, except K0.w = 128
/// (the alpha).
fn lighting_setup(m: &mut Mem, top: u16) {
    let i128 = bc(0x4300_0000);
    for i in 0..3u16 {
        let b = m.ld(top.wrapping_add(9 + i));
        m.st(LIGHT_BASIS + i, b);
    }
    for i in 0..10u16 {
        let k = mul(m.ld(top.wrapping_add(15 + i)), i128);
        let w = if i == 0 {
            max(VF0, i128)[3]
        } else {
            min(VF0, bc(VF0[0]))[3]
        };
        m.st(LIGHT_COEFFS + i, [k[0], k[1], k[2], w]);
    }
}

/// Lighting (0x08b8): per vertex `d = B·n`, then the 2nd-order SH
/// irradiance `K0 + K1 dx² + K2 dy² + K3 dz² + K4 dx dy + K5 dz dx +
/// K6 dy dz + K7 dx + K8 dy + K9 dz`, clamped to 0..255, as RGBA ints (alpha
/// 128) into every 3rd qword from `dst`.
fn light(m: &mut Mem, normals: u16, dst: u16, n: u16) {
    let b: [V; 3] = std::array::from_fn(|i| m.ld(LIGHT_BASIS + i as u16));
    let k: [V; 10] = std::array::from_fn(|i| m.ld(LIGHT_COEFFS + i as u16));
    for i in 0..n {
        let nrm = itof15(m.ld(normals.wrapping_add(i)));
        let d = madd(
            madd(mul(b[0], bc(nrm[0])), b[1], bc(nrm[1])),
            b[2],
            bc(nrm[2]),
        );
        let sq = mul(d, d);
        // OPMULA d × d, then + vf0·vf0: (dy dz, dz dx, dx dy).
        let cross = madd(mul([d[1], d[2], d[0], 0], [d[2], d[0], d[1], 0]), VF0, VF0);
        let mut acc = mul(k[0], bc(ONE));
        acc = madd(acc, k[1], bc(sq[0]));
        acc = madd(acc, k[2], bc(sq[1]));
        acc = madd(acc, k[3], bc(sq[2]));
        acc = madd(acc, k[4], bc(cross[2]));
        acc = madd(acc, k[5], bc(cross[1]));
        acc = madd(acc, k[6], bc(cross[0]));
        acc = madd(acc, k[7], bc(d[0]));
        acc = madd(acc, k[8], bc(d[1]));
        let col = madd(acc, k[9], bc(d[2]));
        let col = min(max(col, bc(VF0[0])), bc(0x437f_0000));
        m.st(dst.wrapping_add(3 * i), ftoi0(col));
    }
}

/// Unclipped mesh strips (0x2908): the matrix is premultiplied into screen
/// space (`M' = gS ⊙ M + gO ⊗ M.w`), positions taken with w = 1.
/// Per vertex {STQ, RGBA, XYZ2}; the first two of each strip get ADC.
fn mesh_strips(m: &mut Mem, mesh: &Mesh, out: u16, mat: &[V; 4], gs: V, go: V, with_rgba: bool) {
    let screen: [V; 4] = std::array::from_fn(|i| madd(mul(gs, mat[i]), go, bc(mat[i][3])));
    let (mut xyz, mut uv, mut col) = (mesh.xyz(), mesh.uv(), mesh.colour());
    let mut o = out;
    for s in 0..mesh.strips {
        let len = m.ilw(mesh.lengths().wrapping_add(s), 0);
        let strip_out = o;
        for _ in 0..len {
            let p = itof15(m.ld(xyz));
            let c = madd(
                madd(
                    madd(mul(screen[0], bc(p[0])), screen[1], bc(p[1])),
                    screen[2],
                    bc(p[2]),
                ),
                screen[3],
                bc(ONE),
            );
            let q = div(ONE, c[3]);
            let t = itof12(m.ld(uv));
            m.st(o, mul([t[0], t[1], ONE, ONE], bc(q)));
            if with_rgba {
                m.st(o.wrapping_add(1), m.ld(col));
            }
            m.st(o.wrapping_add(2), ftoi4(mul(c, bc(q))));
            xyz = xyz.wrapping_add(1);
            uv = uv.wrapping_add(1);
            col = col.wrapping_add(1);
            o = o.wrapping_add(3);
        }
        // ADC on the strip's first two vertices (ISW.w of vi1 = -1).
        m.isw(strip_out.wrapping_add(2), 3, 0xffff);
        m.isw(strip_out.wrapping_add(5), 3, 0xffff);
    }
}

/// Clipped mesh strips (0x2d28 with RGBA, 0x2f60 without: lit colours or
/// pass 2 keep the RGBA slot as it is). Like the float strip: CLIP per
/// vertex, triangles outside the guard band get ADC and go to the clipper.
#[allow(clippy::too_many_arguments)]
fn mesh_strips_clipped(
    m: &mut Mem,
    mesh: &Mesh,
    out: u16,
    mat: &[V; 4],
    gs: V,
    go: V,
    with_rgba: bool,
    kick: &mut dyn FnMut(&[u128]),
) {
    let pos = |m: &Mem, a: u16| -> V {
        let r = m.ld(a);
        let f = itof15(r);
        [f[0], f[1], f[2], r[3]]
    };
    let tex = |m: &Mem, a: u16| -> V {
        let r = m.ld(a);
        merge(r, itof12(r), 0b1100)
    };
    let (mut xyz, mut uv, mut col) = (mesh.xyz(), mesh.uv(), mesh.colour());
    let mut o = out;
    let mut clip = ClipFlags::default();
    for s in 0..mesh.strips {
        let len = m.ilw(mesh.lengths().wrapping_add(s), 0);
        for k in 0..len {
            let c = xform_xyzw(mat, pos(m, xyz.wrapping_add(k)));
            let q = div(ONE, c[3]);
            let stq = mul(tex(m, uv.wrapping_add(k)), bc(q));
            clip.clip(c);
            let screen = madd(mul(go, bc(ONE)), mul(c, bc(q)), gs);
            m.st(o, stq);
            if with_rgba {
                m.st(o.wrapping_add(1), m.ld(col.wrapping_add(k)));
            }
            let mut v = ftoi4(screen);
            if k < 2 {
                v[3] = 0xffff_ffff;
            } else if clip.fcand(0x3_ffff) {
                v[3] = 0xffff_ffff;
                if !clip.triangle_dropped() {
                    // The triangle (k-2, k-1, k), re-read from the input
                    // (colours from the output).
                    let mut buf = [ZERO; 9];
                    for j in 0..3u16 {
                        let i = k - 2 + j;
                        buf[3 * j as usize] = xform_xyzw(mat, pos(m, xyz.wrapping_add(i)));
                        buf[3 * j as usize + 1] = tex(m, uv.wrapping_add(i));
                        buf[3 * j as usize + 2] =
                            itof0(m.ld(o.wrapping_sub(3 * (2 - j)).wrapping_add(1)));
                    }
                    for (i, v) in buf.iter().enumerate() {
                        m.st(CLIP_BUF_A + i as u16, *v);
                    }
                    clip_triangle(m, &mut clip, gs, go, kick);
                    clip.clip(buf[3]);
                    clip.clip(buf[6]);
                }
            }
            m.st(o.wrapping_add(2), v);
            o = o.wrapping_add(3);
        }
        xyz = xyz.wrapping_add(len);
        uv = uv.wrapping_add(len);
        col = col.wrapping_add(len);
    }
}

// ---------------------------------------------------------------------------
// 0x3bf0: cubic line shape
// ---------------------------------------------------------------------------

/// Input at TOP: qw0-2 copied out (A+D RGBAQ setup and the line-strip tag),
/// cubic coefficients C3..C0 at qw3-6, qw7.x = n. P(t) = M·(C3 t³ + C2 t² +
/// C1 t + C0), t = i/(n-1); XYZF2 with F = 0xff, ADC when the vertex or the
/// previous one is outside the guard band.
fn line_shape(m: &mut Mem, top: u16, kick: &mut dyn FnMut(&[u128])) -> bool {
    let n_raw = m.ld(top.wrapping_add(7))[0];
    let n = n_raw as u16;
    if n == 0 {
        return false;
    }
    let mat = [
        m.ld(MATRIX),
        m.ld(MATRIX + 1),
        m.ld(MATRIX + 2),
        m.ld(MATRIX + 3),
    ];
    // Coefficients in clip space (x, y, z, w order, each with its own w).
    let c0 = xform_xyzw(&mat, m.ld(top.wrapping_add(6)));
    let c1 = xform_xyzw(&mat, m.ld(top.wrapping_add(5)));
    let c2 = xform_xyzw(&mat, m.ld(top.wrapping_add(4)));
    let c3 = xform_xyzw(&mat, m.ld(top.wrapping_add(3)));
    let dt = div(ONE, float::sub(itof0(bc(n_raw))[0], ONE).0);
    let (gs, go) = guard(m);

    // t powers as the program forms them: (t·t·t, 1·t·t, 1·t) from (t, 1, 1).
    let poly = |t: u32| -> V {
        let tv = [t, ONE, ONE, 0];
        let p = mul(tv, bc(t));
        let p = merge(p, mul(p, bc(t)), 0b1100);
        let acc = mul(c3, bc(p[0]));
        let acc = madd(acc, c2, bc(p[1]));
        let acc = madd(acc, c1, bc(p[2]));
        madd(acc, c0, bc(ONE))
    };
    let out = top.wrapping_add(OUT);
    m.st(out.wrapping_add(1), m.ld(top.wrapping_add(1)));
    m.st(out, m.ld(top));
    m.st(out.wrapping_add(2), m.ld(top.wrapping_add(2)));

    let mut clip = ClipFlags(0x3f);
    let mut t = ZERO[0];
    for i in 0..n {
        let p = poly(t);
        t = float::add(t, dt).0;
        let pq = mul(p, bc(div(ONE, p[3])));
        clip.clip(pq);
        let screen = madd(mul(go, bc(ONE)), pq, gs);
        let f = ftoi4(screen);
        let adc = clip.fcand(0xfff) as u16;
        let w = adc.wrapping_add(0x7fff) as i16 as i32 as u32;
        m.st(out.wrapping_add(3 + i), [f[0], f[1], f[2], w]);
    }
    kick(&m.gif_packet(out));
    true
}

// ---------------------------------------------------------------------------
// GPU meshes (`crate::gpu`, `vs_mat3_mesh`)
// ---------------------------------------------------------------------------

/// What a GPU-planned pass 1 leaves at its output buffer (the CPU program
/// writes its GIF tag there): pass 2 is planned for the GPU only after it,
/// so a pass 2 on the CPU always finds a pass-1 output it can read.
const GPU_PASS1_MARK: u32 = 0x4750_5531;

/// Uniform qwords of a GPU mesh call (`vs_mat3_mesh`).
pub mod gpu_mesh {
    /// The screen matrix columns: `guardScale · M + guardOffset · M.w`.
    pub const SCREEN: usize = 0;
    /// x: lit, y: vertex count, z: pass (1 or 2), w: the mesh may cross
    /// the guard band (the program clips; otherwise it draws what it
    /// projects, even behind the eye).
    pub const INFO: usize = 4;
    /// The lighting basis (3 rows) and coefficients (10), as the program's.
    pub const BASIS: usize = 5;
    pub const COEFFS: usize = 8;
    /// x, y: guardScale.z, guardOffset.z (the clip z of a screen z).
    pub const GUARD_Z: usize = 18;
}

/// Plan a mesh call for the GPU (`crate::gpu::plan`): the memory effects of
/// [`mesh_entry`] without the vertex loop (setup matrices, flags, lighting
/// setup, in-place texgen); the vertices go to the shader. Data: per vertex
/// {position, UV (after texgen), colour or normal}. `None` (memory
/// untouched) for what the CPU program keeps: other entries, the pass-1
/// sphere map, malformed meshes. `kept`: the renderer keeps this mesh's data
/// (`GpuCall::mesh_key`), whose alpha maximum is this; the data is left out.
pub fn plan_gpu(
    entry: u32,
    top: u32,
    itop: u32,
    kept: Option<f32>,
    mem: &mut [u8; 16384],
) -> Option<crate::gpu::GpuCall> {
    let top = (top & 0x3ff) as u16;
    let mut m = Mem(mem);
    let m = &mut m;
    let after_header = top.wrapping_add(MESH_AFTER_HEADER);
    let (pass1, setup, base, link) = match (entry, itop != 0) {
        (0x2270, false) | (0x2320, _) => (true, true, after_header, 0x468),
        (0x22c8, false) | (0x2358, _) => (true, false, top, 0x46e),
        (0x2270, true) => (false, false, after_header, 0x476),
        (0x22c8, true) => (false, false, top, 0x47c),
        _ => return None,
    };
    let flags = if setup {
        m.ilw(base.wrapping_sub(23), 3)
    } else {
        m.ilw(MESH_FLAGS, 3)
    };
    let mesh = Mesh::read(m, base);
    if (pass1 && flags & flag::TEXGEN1 == 0x10) || !mesh.well_formed(m) {
        return None;
    }
    let out = top.wrapping_add(OUT);
    if !pass1 && m.ld(out)[..2] != [GPU_PASS1_MARK, mesh.n as u32] {
        return None;
    }
    let lit = flags & flag::LIT != 0;
    let tag = m.ld(base);
    let tag = if pass1 {
        tag
    } else {
        [tag[0], m.ld(PASS2_TAG_Y)[1], tag[2], tag[3]]
    };
    if pass1 {
        if setup {
            m.isw(MESH_FLAGS, 3, flags);
            if lit {
                lighting_setup(m, top);
            }
            let mat = [
                m.ld(MATRIX),
                m.ld(MATRIX + 1),
                m.ld(MATRIX + 2),
                m.ld(MATRIX + 3),
            ];
            let model: [V; 4] = std::array::from_fn(|i| m.ld(top.wrapping_add(i as u16)));
            let tex: [V; 4] = std::array::from_fn(|i| m.ld(top.wrapping_add(5 + i as u16)));
            for i in 0..4 {
                m.st(MESH_MATRIX + i as u16, xform_xyzw(&mat, model[i]));
                m.st(TEXGEN_MATRIX + i as u16, tex[i]);
            }
        }
        m.isw(MESH_LINK, 0, link);
        match flags & flag::TEXGEN1 {
            2 => texgen_uv_matrix(m, mesh.uv(), mesh.uv(), mesh.n),
            4 => texgen_uv_matrix(m, mesh.xyz(), mesh.uv(), mesh.n),
            8 => texgen_normal(m, mesh.colour(), mesh.uv(), mesh.n),
            _ => {}
        }
    } else {
        m.isw(MESH_LINK, 0, link);
        match flags & flag::TEXGEN2 {
            0x40 => texgen_uv_matrix(m, mesh.uv(), mesh.uv(), mesh.n),
            0x80 => texgen_uv_matrix(m, mesh.xyz(), mesh.uv(), mesh.n),
            0x100 => texgen_normal(m, mesh.colour(), mesh.uv(), mesh.n),
            0x200 => texgen_sphere(m, mesh.xyz(), mesh.colour(), mesh.uv(), mesh.n),
            _ => {}
        }
    }
    let mut u = [[0u32; 4]; crate::gpu::UNIFORM_QWORDS];
    let mat = [
        m.ld(MESH_MATRIX),
        m.ld(MESH_MATRIX + 1),
        m.ld(MESH_MATRIX + 2),
        m.ld(MESH_MATRIX + 3),
    ];
    let (gs, mut go) = guard(m);
    go[3] = ONE;
    for i in 0..4 {
        u[gpu_mesh::SCREEN + i] = madd(mul(gs, mat[i]), go, bc(mat[i][3]));
    }
    u[gpu_mesh::INFO] = [
        lit as u32,
        mesh.n as u32,
        if pass1 { 1 } else { 2 },
        (flags & flag::CLIP != 0) as u32,
    ];
    u[gpu_mesh::GUARD_Z] = [gs[2], go[2], 0, 0];
    if lit {
        for i in 0..3 {
            u[gpu_mesh::BASIS + i] = m.ld(LIGHT_BASIS + i as u16);
        }
        for i in 0..10 {
            u[gpu_mesh::COEFFS + i] = m.ld(LIGHT_COEFFS + i as u16);
        }
    }
    // (Pass 1 without in-place texgen: the data is the input's as it came.)
    let keyed = pass1 && flags & flag::TEXGEN1 == 0;
    let kept = kept.filter(|_| keyed);
    // Pass 1: {position, UV, colour or normal, flags} per vertex (flags.x:
    // ADC, the strip's first two vertices); pass 2: its UVs (positions,
    // colours and flags are pass 1's, `Link::Reuse`).
    let n = if kept.is_some() { 0 } else { mesh.n };
    let mut data = Vec::with_capacity(if pass1 { 4 } else { 1 } * n as usize);
    let mut alpha_max = kept.unwrap_or(if lit { 128.0f32 } else { 0.0 });
    let mut adc = Vec::with_capacity(n as usize);
    if n > 0 {
        for s in 0..mesh.strips {
            let len = m.ilw(mesh.lengths().wrapping_add(s), 0);
            adc.extend((0..len).map(|k| (k < 2) as u32));
        }
    }
    for i in 0..n {
        if pass1 {
            let c = m.ld(mesh.colour().wrapping_add(i));
            if !lit {
                alpha_max = alpha_max.max(c[3] as i32 as f32);
            }
            data.push(m.ld(mesh.xyz().wrapping_add(i)));
            data.push(m.ld(mesh.uv().wrapping_add(i)));
            data.push(c);
            data.push([adc.get(i as usize).copied().unwrap_or(0), 0, 0, 0]);
        } else {
            data.push(m.ld(mesh.uv().wrapping_add(i)));
        }
    }
    if pass1 {
        m.st(out, [GPU_PASS1_MARK, mesh.n as u32, 0, 0]);
    } else {
        // (Pass 2's alpha is pass 1's: `Gs::gpu_draw` takes it from there.)
        alpha_max = 0.0;
    }
    let mut indices = Vec::new();
    let mut first = 0u32;
    for s in 0..mesh.strips {
        let len = m.ilw(mesh.lengths().wrapping_add(s), 0) as u32;
        for k in 2..len {
            indices.extend_from_slice(&[first + k - 2, first + k - 1, first + k]);
        }
        first += len;
    }
    let tag =
        tag[0] as u128 | (tag[1] as u128) << 32 | (tag[2] as u128) << 64 | (tag[3] as u128) << 96;
    let link = if pass1 {
        crate::gpu::Link::Source(out as u32)
    } else {
        crate::gpu::Link::Reuse(out as u32)
    };
    let mesh_key = keyed.then_some(0);
    Some(crate::gpu::GpuCall {
        program: crate::gpu::Program::Mat3Mesh,
        pre: Vec::new(),
        tag,
        uniforms: u,
        data,
        indices,
        index_mode: 0,
        alpha_max,
        link,
        split: None,
        mesh_key,
    })
}

// ---------------------------------------------------------------------------
// GPU float strips and ribbons (`crate::gpu`, `vs_mat3_strip`)
// ---------------------------------------------------------------------------

/// Uniform qwords of a GPU strip call (`vs_mat3_strip`).
pub mod gpu_strip {
    /// guardScale, guardOffset (qw4, qw5 of the program's memory).
    pub const GUARD_SCALE: usize = 0;
    pub const GUARD_OFFSET: usize = 1;
    /// x: the XYZF2 fog of a drawing vertex (bits 4..11 of FTOI4 of the
    /// screen w, i.e. of guardOffset.w), as f32.
    pub const FOG: usize = 2;
    /// Data qwords per vertex: ST (S, T, Q factor; f32), RGBA (ints), the
    /// clip-space position (f32, as the program computes it), flags (x:
    /// ADC, y: the vertex's index in the strip).
    pub const VERTEX_QWORDS: usize = 4;
}

/// The strip tag a GPU strip draws: PACKED, PRE, NLOOP = the vertex count,
/// REGS {ST (or NOP when untextured), RGBAQ, XYZ2/XYZF2}, a triangle strip
/// with STQ coordinates. Returns whether it is flat shaded (`None`: the CPU
/// program keeps it).
fn gpu_strip_tag(tag: V, n: u16) -> Option<bool> {
    let lo = tag[0] as u64 | (tag[1] as u64) << 32;
    let bits = |at: u32, n: u32| (lo >> at) & ((1 << n) - 1);
    let prim = bits(47, 11);
    let tme = prim & 0x10 != 0;
    let regs = tag[2];
    let ok = bits(0, 15) == n as u64
        && bits(46, 1) == 1
        && bits(58, 2) == 0
        && bits(60, 4) == 3
        && prim & 7 == 4
        && !(tme && prim & 0x100 != 0)
        && (regs >> 4) & 0xf == 1
        && matches!((regs >> 8) & 0xf, 4 | 5)
        && (regs & 0xf == 2 || (regs & 0xf == 0xf && !tme));
    ok.then_some(prim & 8 == 0)
}

/// Plan a float strip (0x12b8) or ribbon (0x3a08) for the GPU
/// (`crate::gpu::plan`).
///
/// The CPU does the program's memory effects and its culling decisions,
/// exactly: the ribbon's rebuild (in-place interleave with the alpha ramp,
/// the count patched to 6m); the strip's tag at its output buffer (so no
/// mesh pass 2 mistakes it for a GPU pass 1); the CLIP test of every vertex
/// and, for the triangles that cross the guard band, the clipper with all
/// its effects (polygon buffers, fan packets, the fan tag toggle at qw33.w,
/// the accumulator spill at qw37) and its fans, kicked first (`pre`) as the
/// program does ([`float_strip_clip`]). Not written: the per-vertex output
/// (STQ, RGBA, XYZ from TOP+0xdc), which no call reads (the next call at
/// this TOP uploads new input; a ribbon writes its scratch there before
/// reading it).
///
/// The CLIP test needs the clip-space positions, which the CPU so computes
/// exactly (the program's transform, PS2 floats, its op order); they go to
/// the GPU as they are: the wakes and shadows lie on the snow at large world
/// coordinates, where the transform's last bits move the depth by
/// thousands of GS units (an IEEE transform makes them z-fight the snow
/// differently). The GPU does the rest per vertex (`vs_mat3_strip`):
/// perspective divide, STQ, the screen position. The triangle list holds the strip's triangles
/// the program draws (not culled); a flat-shaded strip gets 3 vertices per
/// triangle, each with the colour of the triangle's last vertex. Data: per
/// vertex [`gpu_strip::VERTEX_QWORDS`] qwords.
///
/// `None` (memory untouched) for what the CPU program keeps: inputs it
/// would not end on, tags other than [`gpu_strip_tag`]'s, a nonzero
/// guardScale.w, more than 72 vertices or an output reaching the clipper's
/// buffers.
static HW_CLIP: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Hardware clipping for the GPU strips (the renderer's lighting is on):
/// the triangles the program clips at the guard band and the near plane
/// are drawn by the GPU, which clips them itself, instead of as the
/// program's fans (CPU-drawn: without the GPU program they missed the
/// lighting, e.g. a board trail's end nearest the camera).
pub fn set_hw_clip(on: bool) {
    HW_CLIP.store(on, std::sync::atomic::Ordering::Relaxed);
}

fn hw_clip() -> bool {
    HW_CLIP.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn plan_gpu_strip(entry: u32, top: u32, mem: &mut [u8; 16384]) -> Option<crate::gpu::GpuCall> {
    let top = (top & 0x3ff) as u16;
    let mut m = Mem(mem);
    let m = &mut m;
    let n = match entry {
        0x12b8 => {
            let n3 = m.ilw(top.wrapping_add(1), 0);
            if n3 < 9 || !n3.is_multiple_of(3) {
                return None;
            }
            n3 / 3
        }
        0x3a08 => 2 * ribbon_pairs(m, top)?,
        _ => return None,
    };
    if n > 72 || top as u32 + OUT as u32 + 1 + 3 * n as u32 > CLIP_BUF_A as u32 {
        return None;
    }
    let tag = m.ld(top);
    let flat = gpu_strip_tag(tag, n)?;
    let (gs, go) = guard(m);
    // (The screen w is guardOffset.w + q·w·guardScale.w: with a zero
    // guardScale.w, a constant.)
    if gs[3] & 0x7f80_0000 != 0 {
        return None;
    }
    if entry == 0x3a08 {
        ribbon_rebuild(m, top);
    }
    m.st(top.wrapping_add(OUT), tag);
    let mut pre = Vec::new();
    let mut fanned = Vec::new();
    let hw = hw_clip();
    let clip = float_strip_clip(
        m,
        top,
        n,
        &mut |p| {
            if !hw {
                pre.extend_from_slice(p)
            }
        },
        &mut fanned,
    );

    let mut u = [[0u32; 4]; crate::gpu::UNIFORM_QWORDS];
    u[gpu_strip::GUARD_SCALE] = gs;
    u[gpu_strip::GUARD_OFFSET] = go;
    let w = madd(mul(go, bc(ONE)), bc(ONE), gs)[3];
    let fog = (ftoi4(bc(w))[3] >> 4) & 0xff;
    u[gpu_strip::FOG] = [(fog as f32).to_bits(), 0, 0, 0];

    let vertex = |m: &Mem, k: u16| -> [V; 4] {
        let a = top.wrapping_add(2 + 3 * k);
        let (c, adc) = clip[k as usize];
        [
            m.ld(a),
            m.ld(a.wrapping_add(1)),
            c,
            [adc as u32, k as u32, hw as u32, 0],
        ]
    };
    let verts: Vec<[V; 4]> = (0..n).map(|k| vertex(m, k)).collect();
    // (With hardware clipping the triangles the program clips into fans
    // are drawn whole.)
    let drawn = (2..n as usize).filter(|&k| !clip[k].1 || (hw && fanned[k]));
    // (Over the drawn triangles' vertices, as the GS side takes it.)
    let alpha_of = |k: usize| verts[k][1][3] & 0xff;
    let alpha_max = drawn
        .clone()
        .map(|k| {
            if flat {
                alpha_of(k)
            } else {
                alpha_of(k - 2).max(alpha_of(k - 1)).max(alpha_of(k))
            }
        })
        .max()
        .unwrap_or(0) as f32;
    let (data, indices): (Vec<V>, Vec<u32>) = if flat {
        let mut data = Vec::with_capacity(3 * n as usize * gpu_strip::VERTEX_QWORDS);
        for k in drawn {
            let col = verts[k][1];
            for v in &verts[k - 2..=k] {
                data.extend_from_slice(&[v[0], col, v[2], v[3]]);
            }
        }
        let count = (data.len() / gpu_strip::VERTEX_QWORDS) as u32;
        (data, (0..count).collect())
    } else {
        let data = verts.iter().flatten().copied().collect();
        (
            data,
            drawn
                .flat_map(|k| [k as u32 - 2, k as u32 - 1, k as u32])
                .collect(),
        )
    };
    let tag =
        tag[0] as u128 | (tag[1] as u128) << 32 | (tag[2] as u128) << 64 | (tag[3] as u128) << 96;
    Some(crate::gpu::GpuCall {
        program: crate::gpu::Program::Mat3Strip,
        pre,
        tag,
        uniforms: u,
        data,
        indices,
        index_mode: 0,
        alpha_max,
        link: crate::gpu::Link::None,
        split: None,
        mesh_key: None,
    })
}

/// A strip captured at `PS2GraphicsMan_DrawTriStrip` (`crate::direct`):
/// each batch as [`plan_gpu_strip`] (or [`float_strip`] where that keeps
/// the CPU) would plan it from the packet the game built, on a memory with
/// what the program reads: the header (qw0-5), the fan packets' tags (`fans`,
/// static: the material setup's) and the batch's input at its TOP (`tops`).
pub fn plan_strip_cmd(
    entry: u32,
    prim: u32,
    verts: &[[u128; 3]],
    tops: &[u32],
    header: &[u128; 6],
    fans: [u128; 2],
) -> Vec<crate::direct::Out> {
    use crate::direct::Out;
    thread_local! {
        static SCRATCH: std::cell::RefCell<Box<[u8; 16384]>> = std::cell::RefCell::new(Box::new([0; 16384]));
    }
    // (0x12b0, after gp+0x1290 was set: no native program, as for the
    // packet's call.)
    if entry != 0x12b8 {
        return Vec::new();
    }
    let regs: u128 = if prim == 0x5c { 0x412 } else { 0x41f };
    SCRATCH.with_borrow_mut(|mem| {
        let mut outs = Vec::new();
        for (r, &top) in crate::direct::strip_batches(verts.len()).zip(tops) {
            let k = r.len() as u128;
            let mut m = Mem(&mut **mem);
            for (i, &q) in header.iter().enumerate() {
                m.st(i as u16, q4(q));
            }
            m.st(FAN_TAG_A, q4(fans[0]));
            m.st(FAN_TAG_B, q4(fans[1]));
            let t = (top & 0x3ff) as u16;
            m.st(
                t,
                q4(k | 0x8000 | (prim as u128) << 47 | 1 << 46 | 3 << 60 | regs << 64),
            );
            m.st(t.wrapping_add(1), q4(3 * k));
            for (j, v) in verts[r].iter().enumerate() {
                for (l, &q) in v.iter().enumerate() {
                    m.st(t.wrapping_add(2 + 3 * j as u16 + l as u16), q4(q));
                }
            }
            match plan_gpu_strip(0x12b8, top, mem) {
                Some(c) => outs.push(Out::Gpu(c)),
                None => {
                    let mut gif = Vec::new();
                    if float_strip(&mut Mem(&mut **mem), t, &mut |p| gif.extend_from_slice(p)) {
                        outs.push(Out::Gif(gif));
                    }
                }
            }
        }
        outs
    })
}

/// Ribbons captured at the game's emitters (`crate::direct::Cmd::Ribbon`):
/// each program start as [`plan_gpu_strip`] (or [`ribbon`] where that keeps
/// the CPU) plans it, on a memory holding the header, the fan tags and the
/// start's input at its TOP.
pub fn plan_ribbon_cmd(
    entry: u32,
    calls: &[Vec<(u16, u128)>],
    tops: &[u32],
    header: &[u128; 6],
    fans: [u128; 2],
) -> Vec<crate::direct::Out> {
    use crate::direct::Out;
    thread_local! {
        static SCRATCH: std::cell::RefCell<Box<[u8; 16384]>> = std::cell::RefCell::new(Box::new([0; 16384]));
    }
    // (0x3a00: no native program, as for the packet's call.)
    if entry != 0x3a08 {
        return Vec::new();
    }
    SCRATCH.with_borrow_mut(|mem| {
        let mut outs = Vec::new();
        for (writes, &top) in calls.iter().zip(tops) {
            let mut m = Mem(&mut **mem);
            for (i, &q) in header.iter().enumerate() {
                m.st(i as u16, q4(q));
            }
            m.st(FAN_TAG_A, q4(fans[0]));
            m.st(FAN_TAG_B, q4(fans[1]));
            let t = (top & 0x3ff) as u16;
            for &(a, q) in writes {
                m.st(t.wrapping_add(a), q4(q));
            }
            match plan_gpu_strip(0x3a08, top, mem) {
                Some(c) => outs.push(Out::Gpu(c)),
                None => {
                    let mut gif = Vec::new();
                    if ribbon(&mut Mem(&mut **mem), t, &mut |p| gif.extend_from_slice(p)) {
                        outs.push(Out::Gif(gif));
                    }
                }
            }
        }
        outs
    })
}

fn q4(q: u128) -> V {
    std::array::from_fn(|l| (q >> (32 * l)) as u32)
}

/// Debug (`crate::gpu::check`): the strip shader's formulas
/// (`vs_mat3_strip`, IEEE) against the program's strip packet (the last
/// packet of `gif`: clipped fans come first), and the fans (`pre`).
pub fn check_gpu_strip(call: &crate::gpu::GpuCall, gif: &[u128]) -> Option<String> {
    use gpu_strip as g;
    let f = |q: usize| -> [f32; 4] { call.uniforms[q].map(f32::from_bits) };
    let n = (call.tag & 0x7fff) as usize;
    let Some(strip) = gif
        .len()
        .checked_sub(1 + 3 * n)
        .filter(|&s| gif[..s] == call.pre[..])
    else {
        return Some(format!(
            "mat3 strip: {n} vertices, {} qwords of fans; the program kicked {} qwords",
            call.pre.len(),
            gif.len()
        ));
    };
    let strip = strip + 1;
    let (gs, go) = (f(g::GUARD_SCALE), f(g::GUARD_OFFSET));
    let flat = (call.tag >> 47) & 8 == 0;
    let mut bad = Vec::new();
    // The triangles: the strip's whose last vertex has no ADC.
    let want: Vec<u32> = (2..n)
        .filter(|&k| (gif[strip + 3 * k + 2] >> 111) & 1 == 0)
        .map(|k| k as u32)
        .collect();
    let got: Vec<u32> = call
        .indices
        .chunks(3)
        .map(|t| call.data[g::VERTEX_QWORDS * t[2] as usize + 3][1])
        .collect();
    if got != want {
        bad.push(format!("triangles {got:?} want {want:?}"));
    }
    let mut used = vec![false; call.data.len() / g::VERTEX_QWORDS];
    for &i in &call.indices {
        used[i as usize] = true;
    }
    for (v, _) in call
        .data
        .chunks(g::VERTEX_QWORDS)
        .zip(&used)
        .filter(|(_, &u)| u)
    {
        let k = v[3][1] as usize;
        let w: [u32; 12] =
            std::array::from_fn(|i| (gif[strip + 3 * k + i / 4] >> (32 * (i % 4))) as u32);
        if (w[11] >> 15) & 1 != v[3][0] {
            bad.push(format!("v{k}: ADC {} want {}", v[3][0], (w[11] >> 15) & 1));
        }
        let st = v[0].map(f32::from_bits);
        let hc = v[2].map(f32::from_bits);
        let q = 1.0 / hc[3];
        let sx: [f32; 3] = std::array::from_fn(|l| (go[l] + gs[l] * hc[l] * q) * 16.0);
        let stq = [st[0] * q, st[1] * q, st[2] * q];
        let got_xyz = [w[8] as f32, w[9] as f32, w[10] as i32 as f32];
        let got_stq = [
            f32::from_bits(w[0]),
            f32::from_bits(w[1]),
            f32::from_bits(w[2]),
        ];
        let col = v[1].map(|c| c & 0xff);
        let got_col = [w[4] & 0xff, w[5] & 0xff, w[6] & 0xff, w[7] & 0xff];
        let close = |a: f32, b: f32, tol: f32| (a - b).abs() <= tol * (1.0 + b.abs());
        // (x, y within 1/8 pixel; Z within 2 GS units.)
        let ok_xyz =
            (0..2).all(|l| (sx[l] - got_xyz[l]).abs() <= 2.0) && (sx[2] - got_xyz[2]).abs() <= 32.0;
        let ok_stq = (0..3).all(|l| close(stq[l], got_stq[l], 1e-3));
        // (Flat: a triangle's vertices carry its last vertex's colour.)
        let ok_col = flat || col == got_col;
        if !(ok_xyz && ok_stq && ok_col) && bad.len() < 3 {
            bad.push(format!("v{k}: xyz {sx:?} want {got_xyz:?} | stq {stq:?} want {got_stq:?} | rgba {col:?} want {got_col:?}"));
        }
    }
    (!bad.is_empty()).then(|| format!("mat3 strip ({n} vertices, flat {flat}): {}", bad.join("; ")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ssx3_vu::vu::capture::Record;

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    /// A hand-made 3-vertex float strip: identity matrix, one triangle.
    fn strip_input(z2: f32) -> Box<[u8; 16384]> {
        let mut mem = Box::new([0u8; 16384]);
        let mut m = Mem(&mut mem);
        for i in 0..4 {
            let mut c = [0; 4];
            c[i] = f(1.0);
            m.st(MATRIX + i as u16, c);
        }
        m.st(GUARD_SCALE, [f(320.0), f(-224.0), f(-8388467.5), 0]);
        m.st(GUARD_OFFSET, [f(2368.0), f(2272.0), f(8388467.5), 0]);
        m.st(FAN_TOGGLE, [0, 0, 0, FAN_TAG_A as u32]);
        let fan_tag = [0, 3 << 28, 0x412, 0];
        m.st(FAN_TAG_A, fan_tag);
        m.st(FAN_TAG_B, fan_tag);
        let top = 0x26u16;
        // GIF tag: NLOOP 3, EOP, PACKED, NREG 3 (ST RGBAQ XYZ2).
        m.st(top, [3 | 0x8000, 3 << 28, 0x412, 0]);
        m.st(top + 1, [9, 0, 0, 0]);
        let verts = [(0.0f32, 0.5f32, 0.5f32), (0.5, -0.5, 0.5), (-0.5, -0.5, z2)];
        for (k, (x, y, z)) in verts.into_iter().enumerate() {
            let a = top + 2 + 3 * k as u16;
            m.st(a, [f(0.5), f(0.5), f(1.0), 0]);
            m.st(a + 1, [0x80, 0x40, 0x20, 0x80]);
            m.st(a + 2, [f(x), f(y), f(z), f(1.0)]);
        }
        mem
    }

    #[test]
    fn strip_inside_guard_band() {
        let mut mem = strip_input(0.5);
        let mut kicks = Vec::new();
        assert!(run(
            0x12b8,
            0x26,
            0,
            &mut mem,
            &mut VuState::default(),
            &mut |p| kicks.push(p.to_vec())
        ));
        assert_eq!(kicks.len(), 1);
        let k = &kicks[0];
        assert_eq!(k.len(), 10);
        let adc = |v: u128| (v >> 111) & 1;
        assert_eq!((adc(k[3]), adc(k[6]), adc(k[9])), (1, 1, 0));
        // XY of vertex 0: (0 · 320 + 2368, 0.5 · -224 + 2272) in 12.4.
        assert_eq!(k[3] as u32, 2368 * 16);
        assert_eq!((k[3] >> 32) as u32, 2160 * 16);
        // RGBA passes through.
        assert_eq!(k[2], 0x80 << 96 | 0x20 << 64 | 0x40 << 32 | 0x80);
    }

    /// Random float strips and ribbons (a perspective matrix, positions
    /// crossing the guard band and the near plane, both tag kinds the game
    /// uses): [`plan_gpu_strip`] must leave the memory the program leaves
    /// (but its per-vertex output), kick the program's fans first, give the
    /// program's ADC, draw the triangles it draws, and its shader formulas
    /// must give the program's vertices ([`check_gpu_strip`]).
    #[test]
    fn gpu_strip_matches_program() {
        let mut seed = 0x9e37_79b9_7f4a_7c15u64;
        let mut rnd = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let (mut clipped, mut culled, mut flat_calls) = (0, 0, 0);
        for case in 0..400 {
            let ribbon = case % 2 == 1;
            let flat = case % 4 >= 2;
            let mut mem = Box::new([0u8; 16384]);
            let mut m = Mem(&mut mem);
            // Perspective: w = 2 + z (the eye at z = -2), small screen.
            m.st(MATRIX, [f(1.0), 0, 0, 0]);
            m.st(MATRIX + 1, [0, f(1.0), 0, 0]);
            m.st(MATRIX + 2, [0, 0, f(1.0), f(1.0)]);
            m.st(MATRIX + 3, [0, 0, 0, f(2.0)]);
            m.st(GUARD_SCALE, [f(1024.0), f(-1024.0), f(-8388467.5), 0]);
            m.st(GUARD_OFFSET, [f(2047.5), f(2047.5), f(8388467.5), 0]);
            m.st(
                FAN_TOGGLE,
                [0, 0, 0, if case % 3 == 0 { FAN_TAG_B as u32 } else { 0 }],
            );
            m.st(FAN_TAG_A, [0x8003, 0x302e_c000, 0x412, 0]);
            m.st(FAN_TAG_B, [0x8004, 0x302e_c000, 0x412, 0]);
            let top = if case % 5 == 0 { 0x1b6u16 } else { 0x26 };
            let (prim, regs) = if flat {
                (0x04u64, 0x41fu32)
            } else {
                (0x5c, 0x412)
            };
            let pairs = 2 + (rnd() % 20) as u16;
            let n = if ribbon {
                2 * pairs
            } else {
                3 + (rnd() % 30) as u16
            };
            let lo = n as u64 | 1 << 15 | 1 << 46 | prim << 47 | 3 << 60;
            m.st(top, [lo as u32, (lo >> 32) as u32, regs, 0]);
            let scale = [0.5, 1.5, 3.0][case % 3];
            if ribbon {
                m.st(top + 1, [3 * pairs as u32, pairs as u32, f(-0.05), f(1.25)]);
            } else {
                m.st(top + 1, [3 * n as u32, 0, 0, 0]);
            }
            for i in 0..n {
                let a = top + 2 + 3 * i;
                let r: [u64; 7] = std::array::from_fn(|_| rnd());
                let unit = |x: u64| (x % 20001) as f32 / 10000.0 - 1.0;
                m.st(a, [f(unit(r[0])), f(unit(r[1])), f(1.0), 0]);
                m.st(
                    a + 1,
                    [(r[2] % 256) as u32, 0x40, 0x20, (r[3] % 256) as u32],
                );
                m.st(
                    a + 2,
                    [
                        f(unit(r[4]) * scale),
                        f(unit(r[5]) * scale),
                        f(unit(r[6]) * scale),
                        f(1.0),
                    ],
                );
            }
            drop(m);
            let entry = if ribbon { 0x3a08 } else { 0x12b8 };

            let mut want_mem = mem.clone();
            let mut kicks: Vec<Vec<u128>> = Vec::new();
            assert!(run(
                entry,
                top as u32,
                0,
                &mut want_mem,
                &mut VuState::default(),
                &mut |p| kicks.push(p.to_vec())
            ));
            let call = plan_gpu_strip(entry, top as u32, &mut mem).expect("planned");
            // Memory: all but the per-vertex output.
            let out = (top + OUT + 1) as usize..(top + OUT + 1 + 3 * n) as usize;
            for q in (0..1024).filter(|q| !out.contains(q)) {
                assert_eq!(
                    mem[q * 16..q * 16 + 16],
                    want_mem[q * 16..q * 16 + 16],
                    "case {case}: qw {q:#x}"
                );
            }
            // The fans, then the strip.
            let (strip, fans) = kicks.split_last().unwrap();
            assert_eq!(call.pre, fans.concat(), "case {case}");
            clipped += fans.len();
            let adc = |k: usize| (strip[1 + 3 * k + 2] >> 111) & 1 == 1;
            let mut want_tris: Vec<u32> = (2..n as usize)
                .filter(|&k| !adc(k))
                .map(|k| k as u32)
                .collect();
            culled += n as usize - 2 - want_tris.len();
            let mut got_tris: Vec<u32> = call
                .indices
                .chunks(3)
                .map(|t| call.data[gpu_strip::VERTEX_QWORDS * t[2] as usize + 3][1])
                .collect();
            want_tris.sort();
            got_tris.sort();
            assert_eq!(got_tris, want_tris, "case {case}");
            flat_calls += flat as u32;
            let gif: Vec<u128> = kicks.concat();
            assert_eq!(check_gpu_strip(&call, &gif), None, "case {case}");
        }
        assert!(
            clipped > 50 && culled > 100 && flat_calls > 0,
            "clipped {clipped} culled {culled}"
        );
    }

    #[test]
    fn strip_crossing_near_plane_is_clipped() {
        let mut mem = strip_input(-3.0);
        let mut kicks = Vec::new();
        assert!(run(
            0x12b8,
            0x26,
            0,
            &mut mem,
            &mut VuState::default(),
            &mut |p| kicks.push(p.to_vec())
        ));
        // The fan (through the other fan tag) comes before the strip.
        assert_eq!(kicks.len(), 2);
        assert_eq!(Mem(&mut mem).ilw(FAN_TOGGLE, 3), FAN_TAG_B);
        let fan = &kicks[0];
        let n = (fan[0] & 0x7fff) as usize;
        assert_eq!(fan.len(), 1 + 3 * n);
        assert!(n >= 3);
        // The strip's third vertex no longer draws (ADC).
        assert_eq!((kicks[1][9] >> 111) & 1, 1);
    }

    /// Replays VU captures (`SSX3_VU_CAPTURE` files, comma-separated in
    /// `MAT3_CAPTURES`) of this program: every implemented call must give
    /// the captured kicks and data memory.
    #[test]
    fn captures() {
        let Ok(paths) = std::env::var("MAT3_CAPTURES") else {
            eprintln!("MAT3_CAPTURES not set; skipping");
            return;
        };
        let mut stats: std::collections::BTreeMap<u32, (u64, u64, u64)> = Default::default();
        let mut shown = 0;
        for path in paths.split(',') {
            let bytes = std::fs::read(path).unwrap();
            for r in Record::read_all(&bytes) {
                if r.hash != HASH || r.unit != 1 {
                    continue;
                }
                if let Ok(seq) = std::env::var("MAT3_SEQ") {
                    if r.seq.to_string() != seq {
                        continue;
                    }
                }
                // Reference: the current translated program on the same input.
                let mut r = r;
                let mut vu = r.setup();
                let mut ref_kicks: Vec<Vec<u128>> = Vec::new();
                if !vu.run(5_000_000, &mut |p| ref_kicks.push(p.to_vec())) {
                    eprintln!(
                        "(record seq {} entry {:#x}: the translated program does not end; skipped)",
                        r.seq, r.pc
                    );
                    continue;
                }
                r.kicks = ref_kicks;
                r.data_out = vu.data.clone();
                let mut mem: Box<[u8; 16384]> =
                    r.data_in.clone().into_boxed_slice().try_into().unwrap();
                let mut kicks: Vec<Vec<u128>> = Vec::new();
                let ok = run(
                    r.pc,
                    r.top,
                    r.itop,
                    &mut mem,
                    &mut VuState::default(),
                    &mut |p| kicks.push(p.to_vec()),
                );
                let e = stats.entry(r.pc).or_default();
                if !ok {
                    e.2 += 1;
                    continue;
                }
                let kicks_ok = kicks == r.kicks;
                let mem_ok = mem[..] == r.data_out[..];
                if kicks_ok && mem_ok {
                    e.0 += 1;
                } else {
                    e.1 += 1;
                    if shown < 6 {
                        shown += 1;
                        report(&r, &kicks, &mem[..]);
                    }
                }
            }
        }
        for (pc, (ok, bad, ni)) in &stats {
            eprintln!("entry {pc:#06x}: ok {ok} mismatch {bad} not-implemented {ni}");
        }
        assert!(stats.values().all(|s| s.1 == 0));
    }

    /// Compare native and translated on one input; prints the first few
    /// mismatches. Returns None when the native side declines the entry.
    fn check(r: &Record, shown: &mut u32) -> Option<bool> {
        let mut vu = r.setup();
        let mut ref_kicks: Vec<Vec<u128>> = Vec::new();
        if !vu.run(5_000_000, &mut |p| ref_kicks.push(p.to_vec())) {
            return None;
        }
        let mut mem: Box<[u8; 16384]> = r.data_in.clone().into_boxed_slice().try_into().unwrap();
        let mut kicks: Vec<Vec<u128>> = Vec::new();
        if !run(
            r.pc,
            r.top,
            r.itop,
            &mut mem,
            &mut VuState::default(),
            &mut |p| kicks.push(p.to_vec()),
        ) {
            return None;
        }
        let ok = kicks == ref_kicks && mem[..] == vu.data[..];
        if !ok && *shown < 4 {
            *shown += 1;
            let mut rr = r.clone();
            rr.kicks = ref_kicks;
            rr.data_out = vu.data.clone();
            report(&rr, &kicks, &mem[..]);
        }
        Some(ok)
    }

    /// Captured strip inputs with the positions scrambled (extrapolated
    /// between random vertices of the same strip), so that many triangles
    /// cross the guard band and go through the clipper.
    #[test]
    fn fuzz_strips() {
        let Ok(paths) = std::env::var("MAT3_CAPTURES") else {
            return;
        };
        let mut seed = 0x1234_5678_9abc_def0u64;
        let mut rnd = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let (mut ok, mut bad, mut clipped) = (0, 0, 0);
        let mut shown = 0;
        for path in paths.split(',') {
            let bytes = std::fs::read(path).unwrap();
            for r in Record::read_all(&bytes) {
                if r.hash != HASH || r.unit != 1 || r.pc != 0x12b8 {
                    continue;
                }
                let top = r.top as usize & 0x3ff;
                let n = (u32::from_le_bytes(
                    r.data_in[(top + 1) * 16..(top + 1) * 16 + 4]
                        .try_into()
                        .unwrap(),
                ) / 3) as usize;
                for _ in 0..20 {
                    let mut rr = r.clone();
                    let pos = |d: &[u8], k: usize| -> [f32; 3] {
                        let o = (top + 4 + 3 * k) * 16;
                        std::array::from_fn(|l| {
                            f32::from_le_bytes(d[o + 4 * l..o + 4 * l + 4].try_into().unwrap())
                        })
                    };
                    for k in 0..n {
                        let a = pos(&r.data_in, (rnd() as usize) % n);
                        let b = pos(&r.data_in, (rnd() as usize) % n);
                        let t = ((rnd() % 1000) as f32 / 1000.0) * 12.0 - 6.0;
                        let o = (top + 4 + 3 * k) * 16;
                        for l in 0..3 {
                            let v = a[l] + (b[l] - a[l]) * t;
                            rr.data_in[o + 4 * l..o + 4 * l + 4].copy_from_slice(&v.to_le_bytes());
                        }
                    }
                    match check(&rr, &mut shown) {
                        Some(true) => ok += 1,
                        Some(false) => bad += 1,
                        None => {}
                    }
                    let mut vu = rr.setup();
                    let mut nk = 0;
                    vu.run(5_000_000, &mut |_| nk += 1);
                    if nk > 1 {
                        clipped += 1;
                    }
                }
            }
        }
        eprintln!("fuzz strips: ok {ok} mismatch {bad} (inputs with clipped fans: {clipped})");
        assert_eq!(bad, 0);
    }

    /// Captured mesh inputs with the clip flag forced on and the positions
    /// scrambled, through every mesh entry/pass.
    #[test]
    fn fuzz_meshes() {
        let Ok(paths) = std::env::var("MAT3_CAPTURES") else {
            return;
        };
        let mut seed = 0x0bad_cafe_1234_5678u64;
        let mut rnd = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let (mut ok, mut bad, mut clipped, mut ni, mut ok2) = (0, 0, 0, 0, 0);
        let mut shown = 0;
        let rd = |d: &[u8], q: usize, l: usize| {
            u32::from_le_bytes(d[q * 16 + 4 * l..q * 16 + 4 * l + 4].try_into().unwrap())
        };
        for path in paths.split(',') {
            let bytes = std::fs::read(path).unwrap();
            for r in Record::read_all(&bytes) {
                if r.hash != HASH
                    || r.unit != 1
                    || !matches!(r.pc, 0x2270 | 0x22c8 | 0x2320 | 0x2358)
                {
                    continue;
                }
                let top = r.top as usize & 0x3ff;
                let setup = matches!((r.pc, r.itop != 0), (0x2270, false) | (0x2320, _));
                let base = if matches!(r.pc, 0x2270 | 0x2320) {
                    top + 0x1b
                } else {
                    top
                };
                let strips = rd(&r.data_in, base + 1, 0) as usize & 0xffff;
                let n = rd(&r.data_in, base + 1, 2) as usize & 0xffff;
                let xyz = base + 2 + strips;
                for variant in 0..16u32 {
                    let mut rr = r.clone();
                    let flags_at = if setup { top + 4 } else { 34 };
                    let o = flags_at * 16 + 12;
                    let f = rd(&rr.data_in, flags_at, 3);
                    let tex1 = [0u32, 2, 4, 8, 6][(rnd() % 5) as usize];
                    let tex2 = [0u32, 0x40, 0x80, 0x100, 0xc0, 0x200][(rnd() % 6) as usize];
                    let f = (f & !0x3ff)
                        | if variant & 1 != 0 { 0x20 } else { 0 }
                        | (variant >> 1) & 1
                        | tex1
                        | tex2;
                    rr.data_in[o..o + 4].copy_from_slice(&f.to_le_bytes());
                    if variant >= 4 {
                        for k in 0..n {
                            let a = (rnd() as usize) % n;
                            let b = (rnd() as usize) % n;
                            let t = ((rnd() % 1000) as f32 / 1000.0) * 6.0 - 3.0;
                            for l in 0..3 {
                                let va = rd(&r.data_in, xyz + a, l) as i32 as f32;
                                let vb = rd(&r.data_in, xyz + b, l) as i32 as f32;
                                let v = (va + (vb - va) * t) as i32;
                                let o = ((xyz + k) & 0x3ff) * 16 + 4 * l;
                                rr.data_in[o..o + 4].copy_from_slice(&v.to_le_bytes());
                            }
                        }
                    }
                    match check(&rr, &mut shown) {
                        Some(true) => ok += 1,
                        Some(false) => bad += 1,
                        None => ni += 1,
                    }
                    let mut vu = rr.setup();
                    let mut nk = 0;
                    vu.run(5_000_000, &mut |_| nk += 1);
                    if nk > 1 {
                        clipped += 1;
                    }
                    // Pass 2 on top of that pass 1 (same TOP, ITOP != 0),
                    // with each pass-2 texgen.
                    if r.itop == 0 && matches!(r.pc, 0x2270 | 0x22c8) {
                        let mut r2 = rr.clone();
                        r2.data_in = vu.data.clone();
                        r2.start = ssx3_vu::vu::capture::ArchState::of(&vu);
                        r2.itop = 1;
                        let tex2 = [0u32, 0x40, 0x80, 0x100, 0x200][(variant as usize) % 5];
                        let o = 34 * 16 + 12;
                        let f = (rd(&r2.data_in, 34, 3) & !0x3c0) | tex2;
                        r2.data_in[o..o + 4].copy_from_slice(&f.to_le_bytes());
                        match check(&r2, &mut shown) {
                            Some(true) => ok2 += 1,
                            Some(false) => bad += 1,
                            None => ni += 1,
                        }
                    }
                }
            }
        }
        eprintln!("fuzz meshes: ok {ok} (+ {ok2} chained pass 2) mismatch {bad} not-implemented {ni} (inputs with clipped fans: {clipped})");
        assert_eq!(bad, 0);
    }

    fn report(r: &Record, kicks: &[Vec<u128>], mem: &[u8]) {
        eprintln!(
            "--- mismatch: entry {:#06x} top {:#x} itop {} frame {} seq {}",
            r.pc, r.top, r.itop, r.frame, r.seq
        );
        if kicks.len() != r.kicks.len() {
            eprintln!(
                "  kick count native {} translated {}",
                kicks.len(),
                r.kicks.len()
            );
        }
        for (i, (a, b)) in kicks.iter().zip(&r.kicks).enumerate() {
            if a != b {
                eprintln!("  kick {i}: len native {} translated {}", a.len(), b.len());
                for (j, (x, y)) in a.iter().zip(b).enumerate() {
                    if x != y {
                        eprintln!("    qw {j}: native {x:032x} translated {y:032x}");
                    }
                }
                break;
            }
        }
        let mut n = 0;
        for q in 0..1024 {
            let a = &mem[q * 16..q * 16 + 16];
            let b = &r.data_out[q * 16..q * 16 + 16];
            if a != b {
                if n < 8 {
                    eprintln!("  mem qw {q:#05x}: native {:02x?} translated {:02x?}", a, b);
                }
                n += 1;
            }
        }
        if n > 0 {
            eprintln!("  {n} qwords of data memory differ");
        }
    }
}
