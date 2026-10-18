//! The game's VU0 microprogram, by hand: its visibility library (3904 bytes
//! of microcode uploaded at boot, segment hash 0x27649fa55223cbbf). The EE
//! calls it with VCALLMS / VCALLMSR after loading the operands into VF/VI
//! (macro mode); results come back in VI1/VI2 and VF registers.
//!
//! | entry | what |
//! | ----- | ---- |
//! | 0x000 | matrix chain: 10 matrices (VF1-16 by data qwords 16..25), each applied to 10 vectors, stored from qword 32 |
//! | 0x278 | VF9-12 = VF1-4 × VF5-8 (4×4 product) |
//! | 0x320 | box (VF15 min, VF16 max) against the clip volume of VF10-13 (corners built per lane) |
//! | 0x570 | the same box test, corners as matrix columns (`box_clip`) |
//! | 0x6e0 | segment VF10→VF13 against triangle VF10, VF11, VF12 |
//! | 0x828 | sphere (VF6: centre, w radius) inside all of planes VF1-5 |
//! | 0x948 | sphere against planes VF1-5: outside / intersecting / inside |
//! | 0xdb8 | object visibility: box test (matrix at qword 64), plane groups, a second box test (qword 68) |
//! | 0xe80 | the second box test alone |
//! | 0xef0 | plane groups alone |
//!
//! Exactness: every FMAC op uses the PS2 float model (`fast`), in program
//! order, so VF, ACC, Q, I, VI, memory and the final MAC/status/clip flags
//! equal the translated microcode's (`tests::matches_translation`). The
//! microcode is scheduled so that every flag read (FMAND, FSAND, FCAND,
//! FCGET) sees exactly the op 4 pairs before it, and no branch reads a VI
//! written in the pair before; written sequentially, each read takes the
//! flags of that op. Cycle counts are not modelled (VU0 runs to completion
//! inside VCALLMS; the EE cannot observe them).

use super::{fast, Vu, VuExit};
use crate::Qword;

pub const HASH: u64 = 0x2764_9fa5_5223_cbbf;
/// Code bytes (the registry entry's length).
pub const LEN: u32 = 0xf40;
/// The routines' entry points.
pub const ENTRIES: [u32; 10] = [0x000, 0x278, 0x320, 0x570, 0x6e0, 0x828, 0x948, 0xdb8, 0xe80, 0xef0];

type V = [u32; 4];

const XYZW: u8 = 0b1111;
const XYZ: u8 = 0b1110;
const X: u8 = 0b1000;
const W: u8 = 0b0001;

/// `[v.c; 4]`: a broadcast operand.
fn bc(v: V, c: usize) -> V {
    [v[c]; 4]
}

/// MAC flag bits of the lanes in `d` (bit 3 = x).
fn lanes(d: u8) -> u32 {
    let d = d as u32;
    d | d << 4 | d << 8 | d << 12
}

fn merge(to: &mut V, d: u8, v: V) {
    for (l, x) in to.iter_mut().enumerate() {
        if d & (8 >> l) != 0 {
            *x = v[l];
        }
    }
}

/// Where an FMAC result goes.
#[derive(Clone, Copy)]
enum To {
    Acc,
    /// VF0 discards the value (the op is for its flags).
    Vf(usize),
}
use To::{Acc, Vf};

/// Sign of x in the MAC flags (FMAND with 0x80, as the plane tests use).
const SIGN_X: u16 = 0x80;
/// The status flags' S bit (FSAND 2).
const STATUS_S: u16 = 2;

/// The VU while this program runs: the helpers below are its micro ops.
struct Vu0<'a>(&'a mut Vu);

impl std::ops::Deref for Vu0<'_> {
    type Target = Vu;
    fn deref(&self) -> &Vu {
        self.0
    }
}
impl std::ops::DerefMut for Vu0<'_> {
    fn deref_mut(&mut self) -> &mut Vu {
        self.0
    }
}

impl Vu0<'_> {
    fn f(&self, r: usize) -> V {
        self.vf[r]
    }

    /// Write an FMAC result and commit its flags; returns the MAC flags.
    fn fmac(&mut self, to: To, d: u8, (v, mac): (V, u32)) -> u32 {
        match to {
            Acc => merge(&mut self.acc, d, v),
            Vf(0) => {}
            Vf(r) => merge(&mut self.vf[r], d, v),
        }
        let mac = mac & lanes(d);
        self.set_mac(mac);
        mac
    }
    fn add(&mut self, to: To, d: u8, s: V, t: V) -> u32 {
        self.fmac(to, d, fast::add4::<XYZW, true, true>(&s, &t))
    }
    fn sub(&mut self, to: To, d: u8, s: V, t: V) -> u32 {
        self.fmac(to, d, fast::sub4::<XYZW, true, true>(&s, &t))
    }
    fn mul(&mut self, to: To, d: u8, s: V, t: V) -> u32 {
        self.fmac(to, d, fast::mul4::<XYZW, true, true>(&s, &t))
    }
    /// `to = ACC + s * t`.
    fn madd(&mut self, to: To, d: u8, s: V, t: V) -> u32 {
        let a = self.acc;
        self.fmac(to, d, fast::madd4::<XYZW, true, true, true>(&a, &s, &t))
    }
    /// `to = ACC - s * t`.
    fn msub(&mut self, to: To, d: u8, s: V, t: V) -> u32 {
        let a = self.acc;
        self.fmac(to, d, fast::msub4::<XYZW, true, true, true>(&a, &s, &t))
    }
    /// `to.xyz = s × t` (OPMULA ACC = s.yzx * t.zxy; OPMSUB to = ACC - t.yzx * s.zxy).
    fn cross(&mut self, to: usize, s: usize, t: usize) {
        let yzx = |v: V| [v[1], v[2], v[0], 0];
        let zxy = |v: V| [v[2], v[0], v[1], 0];
        let (s, t) = (self.f(s), self.f(t));
        self.mul(Acc, XYZ, yzx(s), zxy(t));
        self.msub(Vf(to), XYZ, yzx(t), zxy(s));
    }
    /// MOVE / LQ: lower-unit writes of lanes `d`.
    fn mov(&mut self, to: usize, d: u8, v: V) {
        if to != 0 {
            merge(&mut self.vf[to], d, v);
        }
    }
    /// CLIP `s.xyz` against `|w|`; returns what FCGET then reads (the low
    /// 12 bits: this judgement and the one before).
    fn clip_against(&mut self, s: V, w: u32) -> u16 {
        let c = fast::clip(s, w);
        self.clip_commit(c, false);
        (self.clip & 0xfff) as u16
    }
    fn setvi(&mut self, r: usize, v: u16) {
        if r != 0 {
            self.vi[r] = v;
        }
    }
    /// LQI: load the qword at VI `r`, then increment it.
    fn lqi(&mut self, to: usize, r: usize) {
        let a = self.vi[r];
        let v = self.ldq(a as u32);
        self.mov(to, XYZW, v);
        self.setvi(r, a.wrapping_add(1));
    }
    /// SQI: store VF `from` at VI `r`, then increment it.
    fn sqi(&mut self, from: usize, r: usize) {
        let a = self.vi[r];
        let v = self.f(from);
        self.stq::<XYZW>(a as u32, &v);
        self.setvi(r, a.wrapping_add(1));
    }
    /// BAL: the link register holds the pair after the delay slot.
    fn link(&mut self, bal_pc: u32) {
        self.setvi(15, ((bal_pc + 16) / 8) as u16);
    }
}

/// Run the program from `pc` to its end.
pub fn run(vu: &mut Vu, pc: u32, _kick: &mut dyn FnMut(&[Qword])) -> VuExit {
    let vu = &mut Vu0(vu);
    let end = match pc {
        0x000 => matrix_chain(vu),
        0x278 => matrix_product(vu),
        0x320 => box_test_lanes(vu),
        0x570 => {
            box_clip(vu);
            0x6e0
        }
        0x6e0 => segment_triangle(vu),
        0x828 => sphere_inside_planes(vu),
        0x948 => sphere_against_planes(vu),
        0xdb8 => object_visibility(vu),
        0xe80 => second_box_test(vu),
        0xef0 => plane_groups_only(vu),
        _ => {
            eprintln!("[vu0lib] no routine at {pc:#06x}; stopping VU0");
            pc
        }
    };
    vu.end_program(end);
    VuExit::End
}

/// 0x000: ten passes; pass `k` builds the matrix (VF1-16 regrouped as four
/// column sets) times data qword 16 + k into VF23-26, then transforms data
/// qwords 16..25 by it (xyz; w kept from qword 16), storing the results
/// from qword 32 on (100 qwords).
fn matrix_chain(vu: &mut Vu0) -> u32 {
    vu.setvi(3, 10);
    vu.setvi(5, 16);
    vu.setvi(7, 32);
    let first = vu.ldq(vu.vi[5] as u32);
    vu.mov(18, XYZW, first);
    vu.mov(27, XYZW, first);
    loop {
        // 0x028
        let zero = vu.f(0);
        vu.sub(Vf(19), XYZW, zero, zero);
        vu.setvi(4, 16);
        vu.sub(Vf(21), XYZW, zero, zero);
        vu.lqi(28, 5);
        vu.sub(Vf(22), XYZW, zero, zero);
        vu.lqi(17, 4);
        // VF23-26: the four column sets (VF1-4, 5-8, 9-12, 13-16, xyz into
        // VF19-22) times VF28.
        for (k, out) in (23..=26).enumerate() {
            for (c, r) in (19..=22).enumerate() {
                let v = vu.f(1 + 4 * k + c);
                vu.mov(r, XYZ, v);
            }
            let m = vu.f(28);
            let (a, b, c, d) = (vu.f(19), vu.f(20), vu.f(21), vu.f(22));
            vu.mul(Acc, XYZW, a, bc(m, 0));
            vu.madd(Acc, XYZW, b, bc(m, 1));
            vu.madd(Acc, XYZW, c, bc(m, 2));
            vu.madd(Vf(out), XYZW, d, bc(m, 3));
        }
        // Ten vectors through it, alternating VF17/VF28 (loads) and
        // VF18/VF27 (results); each result is stored one step later.
        for k in 0..10 {
            let (src, dst, next) = if k % 2 == 0 { (17, 18, 28) } else { (28, 27, 17) };
            let s = vu.f(src);
            let (a, b, c, d) = (vu.f(23), vu.f(24), vu.f(25), vu.f(26));
            vu.mul(Acc, XYZW, a, bc(s, 0));
            if k < 9 {
                vu.lqi(next, 4);
            }
            vu.madd(Acc, XYZW, b, bc(s, 1));
            vu.madd(Acc, XYZW, c, bc(s, 2));
            if k > 0 {
                vu.sqi(if dst == 18 { 27 } else { 18 }, 7);
            }
            vu.madd(Vf(dst), XYZ, d, bc(s, 3));
        }
        // 0x240: count down; the delay slot stores the last result.
        vu.vi[3] = vu.vi[3].wrapping_sub(1);
        let again = (vu.vi[3] as i16) > 0;
        vu.sqi(27, 7);
        if !again {
            return 0x278;
        }
    }
}

/// 0x278: VF9-12 = VF1-4 × VF5-8 (row k of the result: VF1-4 weighted by
/// VF(5+k)).
fn matrix_product(vu: &mut Vu0) -> u32 {
    for k in 0..4 {
        let m = vu.f(5 + k);
        let (a, b, c, d) = (vu.f(1), vu.f(2), vu.f(3), vu.f(4));
        vu.mul(Acc, XYZW, a, bc(m, 0));
        vu.madd(Acc, XYZW, b, bc(m, 1));
        vu.madd(Acc, XYZW, c, bc(m, 2));
        vu.madd(Vf(9 + k), XYZW, d, bc(m, 3));
    }
    0x320
}

/// 0x320: the eight corners of the box VF15 (min) .. VF16 (max), picked
/// per lane (VF1 = min, VF8 = max.xyz), transformed by VF10-13 into
/// VF20-27 and clipped. VI1 = OR of the judgements, VI2 = AND (6 bits).
fn box_test_lanes(vu: &mut Vu0) -> u32 {
    let (zero, lo, hi) = (vu.f(0), vu.f(15), vu.f(16));
    vu.add(Vf(14), XYZ, zero, bc(zero, 3));
    vu.mul(Vf(14), W, zero, bc(zero, 3));
    let one = vu.f(14);
    // Lanes from the min corner (the rest from max), per corner VF1..8.
    const FROM_MIN: [u8; 8] = [0b1111, 0b1101, 0b1011, 0b1001, 0b0111, 0b0101, 0b0011, 0b0001];
    for (k, m) in FROM_MIN.into_iter().enumerate() {
        vu.mul(Vf(1 + k), m, one, lo);
        if k > 0 {
            vu.mul(Vf(1 + k), !m & XYZW, one, hi);
        }
    }
    let transform = |vu: &mut Vu0, corner: usize, out: usize| {
        let p = vu.f(corner);
        let (a, b, c, d) = (vu.f(10), vu.f(11), vu.f(12), vu.f(13));
        vu.mul(Acc, XYZW, a, bc(p, 0));
        vu.madd(Acc, XYZW, b, bc(p, 1));
        vu.madd(Acc, XYZW, c, bc(p, 2));
        vu.madd(Vf(out), XYZW, d, bc(p, 3));
    };
    transform(vu, 1, 20);
    transform(vu, 2, 21);
    let (mut any, mut all) = (0u16, 63u16);
    for k in 0..8 {
        let v = vu.f(20 + k);
        let seen = vu.clip_against(v, v[3]);
        if k + 3 <= 8 {
            transform(vu, 3 + k, 22 + k);
        }
        vu.setvi(1, seen);
        any |= seen;
        all &= seen;
        vu.setvi(2, any);
        vu.setvi(3, all);
    }
    vu.setvi(4, 63);
    vu.setvi(1, any & 63);
    vu.setvi(2, all & 63);
    0x570
}

/// 0x570 / 0xac8: the box VF15 (min) .. VF16 (max) through the matrix
/// VF10-13 (rows: x, y, z, translation), its eight corners into VF1-8
/// (corner order: z, then y, then x from min to max) and clipped. VI1 = OR
/// of the eight judgements (6 bits), VI2 = AND;
/// VI3/VI4 hold the last two FCGET reads, VI5 = 63.
fn box_clip(vu: &mut Vu0) {
    let (lo, hi) = (vu.f(15), vu.f(16));
    let (mx, my, mz, mt) = (vu.f(10), vu.f(11), vu.f(12), vu.f(13));
    let one = vu.f(0);
    // ACC = translation + z + y of a corner pair; the two x choices finish
    // it into VF`out` (x = min) and VF`out + 1` (x = max).
    let partial = |vu: &mut Vu0, z: V, y: V| {
        vu.mul(Acc, XYZW, mt, bc(one, 3));
        vu.madd(Acc, XYZW, mz, bc(z, 2));
        vu.madd(Acc, XYZW, my, bc(y, 1));
    };
    let finish = |vu: &mut Vu0, out: usize| {
        vu.madd(Vf(out), XYZW, mx, bc(lo, 0));
        vu.madd(Vf(out + 1), XYZW, mx, bc(hi, 0));
    };
    // CLIP a corner pair; FCGET reads each judgement 4 pairs later (while
    // the next corners are computed) and sees it with the one before.
    let clip_pair = |vu: &mut Vu0, a: usize| -> (u16, u16) {
        let (p, q) = (vu.f(a), vu.f(a + 1));
        (vu.clip_against(p, p[3]), vu.clip_against(q, q[3]))
    };
    let fold = |vu: &mut Vu0, (r3, r4): (u16, u16)| {
        vu.setvi(3, r3);
        vu.setvi(4, r4);
        vu.vi[1] |= r3;
        vu.vi[2] &= r3;
        vu.vi[1] |= r4;
        vu.vi[2] &= r4;
    };
    partial(vu, lo, lo);
    finish(vu, 1);
    vu.setvi(1, 0);
    vu.setvi(2, 63);
    partial(vu, lo, hi);
    let r = clip_pair(vu, 1);
    finish(vu, 3);
    partial(vu, hi, lo);
    fold(vu, r);
    let r = clip_pair(vu, 3);
    finish(vu, 5);
    partial(vu, hi, hi);
    fold(vu, r);
    let r = clip_pair(vu, 5);
    finish(vu, 7);
    fold(vu, r);
    let r = clip_pair(vu, 7);
    vu.setvi(5, 63);
    fold(vu, r);
    vu.vi[1] &= 63;
}

/// 0x6e0: does the segment VF10 → VF13 hit the triangle VF10, VF11, VF12?
/// VI2 = 1 if it does (barycentrics u, v >= 0 and u + v < 1 and a
/// non-degenerate determinant, |det| > 1e-10), else 0.
fn segment_triangle(vu: &mut Vu0) -> u32 {
    const END: u32 = 0x828;
    vu.setvi(2, 0);
    let (a, b, c, p) = (vu.f(10), vu.f(11), vu.f(12), vu.f(13));
    vu.sub(Vf(1), XYZ, b, a); // e1
    vu.sub(Vf(2), XYZ, c, a); // e2
    vu.sub(Vf(7), XYZ, p, a); // d
    vu.cross(3, 1, 2); // n = e1 × e2
    vu.cross(15, 7, 1); // d × e1
    vu.cross(8, 3, 2); // n × e2
    let (n, de1, ne2, e1, d) = (vu.f(3), vu.f(15), vu.f(8), vu.f(1), vu.f(7));
    vu.mul(Vf(16), XYZ, n, de1);
    vu.mul(Vf(4), XYZ, e1, ne2);
    let zero = vu.f(0);
    vu.add(Vf(5), X, zero, bc(zero, 3)); // VF5.x = 1
    vu.mul(Vf(9), XYZ, d, ne2);
    vu.loi(0x2edb_e6ff); // 1e-10
    vu.mul(Vf(6), W, zero, [vu.i; 4]);
    // Horizontal sums (x + y + 1·z) of the three products.
    let hsum = |vu: &mut Vu0, r: usize| {
        let (v, one) = (vu.f(r), vu.f(5));
        vu.add(Acc, X, v, bc(v, 1));
        vu.madd(Vf(r), X, one, bc(v, 2));
    };
    hsum(vu, 4); // det = e1 · (n × e2)
    hsum(vu, 9);
    let (det, eps) = (vu.f(4), vu.f(6)[3]);
    let clip = vu.clip_against(det, eps);
    vu.div(0, 3, 4, 0); // Q = 1 / det
    vu.waitq();
    hsum(vu, 16);
    let big = clip & 3 != 0;
    vu.setvi(1, big as u16);
    if !big {
        return END;
    }
    let q = [vu.q; 4];
    let (u, v) = (vu.f(9), vu.f(16));
    let s_u = vu.mul(Vf(9), X, u, q);
    let s_v = vu.mul(Vf(16), X, v, q);
    let s = |mac: u32| if mac & 0xf0 != 0 { STATUS_S } else { 0 };
    let (u, v, one) = (vu.f(9), vu.f(16), vu.f(5));
    let s_u1 = vu.sub(Vf(14), X, u, bc(zero, 3)); // u - 1
    vu.setvi(1, s(s_u));
    vu.sub(Acc, X, u, one);
    // (The branch's delay slot: u + v - 1, and v's sign.)
    let s_uv = vu.madd(Vf(17), X, v, bc(zero, 3));
    vu.setvi(3, s(s_v));
    if s(s_u) != 0 {
        return END; // u < 0
    }
    vu.setvi(1, s(s_u1));
    if s(s_v) != 0 {
        return END; // v < 0
    }
    vu.setvi(3, s(s_uv));
    if s(s_u1) == 0 {
        return END; // u >= 1
    }
    if s(s_uv) == 0 {
        return END; // u + v >= 1
    }
    vu.setvi(2, 1);
    END
}

/// The steps of a sphere/plane distance, which the microcode interleaves
/// across planes: `a`, `b`, `c` sum ACC.x = n·c + d for plane VF`r` (its
/// xyz already multiplied by the centre, componentwise; w the distance;
/// VF7.x = 1), then `minus_r` / `plus_r` give the flags of ACC.x ∓ the
/// radius (VF6.w). A negative x (MAC 0x80) is "behind the plane".
struct Plane;

impl Plane {
    fn a(vu: &mut Vu0, r: usize) {
        let p = vu.f(r);
        vu.add(Acc, X, p, bc(p, 1));
    }
    fn b(vu: &mut Vu0, r: usize) {
        let (p, one) = (vu.f(r), vu.f(7));
        vu.madd(Acc, X, one, bc(p, 2));
    }
    fn c(vu: &mut Vu0, r: usize) {
        let (p, one) = (vu.f(r), vu.f(7));
        vu.madd(Acc, X, one, bc(p, 3));
    }
    /// ACC.x - radius (flags only).
    fn minus_r(vu: &mut Vu0) -> u32 {
        let (one, s) = (vu.f(7), vu.f(6));
        vu.msub(Vf(0), X, one, bc(s, 3))
    }
    /// ACC.x + radius (flags only).
    fn plus_r(vu: &mut Vu0) -> u32 {
        let (one, s) = (vu.f(7), vu.f(6));
        vu.madd(Vf(0), X, one, bc(s, 3))
    }
}

/// FMAND VI2 = MAC & VI1 (VI1 = 0x80: x negative).
fn fmand(vu: &mut Vu0, to: usize, mac: u32) -> u16 {
    let v = mac as u16 & vu.vi[1];
    vu.setvi(to, v);
    v
}

/// The five-plane test of 0x828 and of each plane group (0xc18): VF1-5 are
/// the planes, VF6 the sphere. True if the sphere is not entirely behind
/// any plane. The early exits leave the next plane's second step done (the
/// branch delay slot).
fn sphere_in_five_planes(vu: &mut Vu0) -> bool {
    let zero = vu.f(0);
    vu.add(Vf(7), X, zero, bc(zero, 3)); // VF7.x = 1
    vu.setvi(1, SIGN_X);
    let s = vu.f(6);
    for r in 1..=5 {
        let p = vu.f(r);
        vu.mul(Vf(r), XYZ, p, s);
    }
    Plane::a(vu, 1);
    Plane::b(vu, 1);
    Plane::c(vu, 1);
    let mut prev = Plane::minus_r(vu);
    Plane::a(vu, 2);
    Plane::b(vu, 2);
    Plane::c(vu, 2);
    let m = Plane::minus_r(vu);
    fmand(vu, 2, prev);
    prev = m;
    for r in 3..=5 {
        Plane::a(vu, r);
        let behind = vu.vi[2] == vu.vi[1];
        Plane::b(vu, r);
        if behind {
            return false;
        }
        Plane::c(vu, r);
        let m = Plane::minus_r(vu);
        fmand(vu, 2, prev);
        prev = m;
    }
    if vu.vi[2] == vu.vi[1] {
        return false;
    }
    fmand(vu, 2, prev);
    vu.vi[2] != vu.vi[1]
}

/// 0x828: VI1 = 1 if the sphere VF6 is in front of all planes VF1-5.
fn sphere_inside_planes(vu: &mut Vu0) -> u32 {
    if sphere_in_five_planes(vu) {
        vu.setvi(1, 1);
        0x938
    } else {
        vu.setvi(1, 0);
        0x948
    }
}

/// 0x948: the sphere VF6 against planes VF1-5 (both sides). VI1 = 1 if it
/// is entirely behind one of them, 0 if entirely in front of all, 2 if it
/// crosses one.
fn sphere_against_planes(vu: &mut Vu0) -> u32 {
    let s = vu.f(6);
    let p = vu.f(1);
    vu.mul(Vf(1), XYZ, p, s);
    vu.setvi(1, SIGN_X);
    let zero = vu.f(0);
    vu.add(Vf(7), X, zero, bc(zero, 3));
    vu.setvi(2, 0);
    for r in [2, 5] {
        let p = vu.f(r);
        vu.mul(Vf(r), XYZ, p, s);
    }
    Plane::a(vu, 1);
    Plane::b(vu, 1);
    Plane::c(vu, 1);
    let mut far = Plane::plus_r(vu);
    let p = vu.f(3);
    vu.mul(Vf(3), XYZ, p, s);
    let mut near = Plane::minus_r(vu);
    // Plane k's two signs are read while plane k+1 is evaluated: "far"
    // (centre + radius behind: VI = 0x80 → outside) and "near" (centre -
    // radius behind: the sphere crosses it), ORed into VI2/VI3 in turn.
    let (mut t, mut o) = (3, 2);
    for r in 2..=5 {
        Plane::a(vu, r);
        Plane::b(vu, r);
        fmand(vu, t, far);
        Plane::c(vu, r);
        let out = vu.vi[t] == vu.vi[1];
        let f = Plane::plus_r(vu);
        fmand(vu, t, near);
        if out {
            vu.setvi(1, 1);
            return 0xac8;
        }
        if r == 2 {
            let p = vu.f(4);
            vu.mul(Vf(4), XYZ, p, s);
        }
        let n = Plane::minus_r(vu);
        let v = vu.vi[o] | vu.vi[t];
        vu.setvi(t, v);
        far = f;
        near = n;
        std::mem::swap(&mut t, &mut o);
    }
    if fmand(vu, 3, far) == vu.vi[1] {
        fmand(vu, 1, near);
        vu.setvi(1, 1);
        return 0xac8;
    }
    let crossing = fmand(vu, 1, near);
    let v = vu.vi[2] | crossing;
    vu.setvi(2, v);
    if v == 0 {
        vu.setvi(1, 0);
        0xab8
    } else {
        vu.setvi(1, 2);
        0xaa8
    }
}

/// 0xc18: VI14 is a mask of plane groups (bit k: the five planes at data
/// qword VI10 + 5k). VI1 = 1 if the sphere VF6 is inside all five planes
/// of one of the selected groups, else 0. Consumes VI14; VI11 = the bit
/// after the last group tried.
fn plane_groups(vu: &mut Vu0) {
    if vu.vi[14] == 0 {
        vu.setvi(11, 1);
        vu.setvi(1, 0);
        return;
    }
    vu.setvi(11, 1);
    loop {
        let bit = vu.vi[14] & vu.vi[11];
        vu.setvi(1, bit);
        vu.vi[14] = vu.vi[14].wrapping_sub(bit);
        if bit != 0 {
            let base = vu.vi[10] as u32;
            for k in 0..5 {
                let v = vu.ldq(base + k);
                vu.mov(1 + k as usize, XYZW, v);
            }
            if sphere_in_five_planes(vu) {
                vu.setvi(1, 1);
                return;
            }
        }
        vu.vi[10] = vu.vi[10].wrapping_add(5);
        let more = vu.vi[14] != 0;
        vu.vi[11] = vu.vi[11].wrapping_add(vu.vi[11]);
        if !more {
            vu.setvi(1, 0);
            return;
        }
    }
}

/// The box test of 0xdb8 / 0xe80 with the matrix at data qword `m`.
fn box_test_at(vu: &mut Vu0, m: u32, bal: u32) {
    for k in 0..4 {
        let v = vu.ldq(m + k);
        vu.mov(10 + k as usize, XYZW, v);
    }
    let (lo, hi) = (vu.f(20), vu.f(21));
    vu.mov(15, XYZW, lo);
    vu.mov(16, XYZW, hi);
    vu.link(bal);
    box_clip(vu);
}

/// 0xdb8: an object's visibility. The box VF20 (min) .. VF21 (max) is
/// tested against the matrix at qword 64; then the sphere VF22 against the
/// plane groups VI14 (from qword 80); if the box crossed the clip volume,
/// the box again against the matrix at qword 68. VI1: 1 = box outside, 2 =
/// in a plane group, 0 = visible inside, 3 = second box test crosses, 4 =
/// second box test passes (see `second_box_test`).
fn object_visibility(vu: &mut Vu0) -> u32 {
    box_test_at(vu, 64, 0xde8);
    let culled = vu.vi[2] != 0;
    let crossing = vu.vi[1];
    vu.setvi(13, crossing);
    if culled {
        vu.setvi(1, 1);
        return 0xe18;
    }
    if vu.vi[14] != 0 {
        let s = vu.f(22);
        vu.mov(6, XYZW, s);
        vu.link(0xe28);
        vu.setvi(10, 80);
        plane_groups(vu);
        if vu.vi[1] != 0 {
            vu.setvi(1, 2);
            return 0xe58;
        }
    } else {
        let s = vu.f(22);
        vu.mov(6, XYZW, s);
    }
    if vu.vi[13] == 0 {
        vu.setvi(1, 0);
        return 0xe80;
    }
    second_box_test(vu)
}

/// 0xe80: the box VF20 .. VF21 against the matrix at qword 68. VI1 = 4 if
/// any corner is outside a plane (the OR of judgements is non-zero), else 3.
fn second_box_test(vu: &mut Vu0) -> u32 {
    box_test_at(vu, 68, 0xeb0);
    if vu.vi[1] != 0 {
        vu.setvi(1, 4);
        0xef0
    } else {
        vu.setvi(1, 3);
        0xee0
    }
}

/// 0xef0: VI1 = 2 if the sphere VF22 is inside one of the plane groups
/// VI14 (from qword 80), else 0.
fn plane_groups_only(vu: &mut Vu0) -> u32 {
    let s = vu.f(22);
    vu.mov(6, XYZW, s);
    if vu.vi[14] == 0 {
        vu.setvi(1, 0);
        return 0xf40;
    }
    vu.link(0xf00);
    vu.setvi(10, 80);
    plane_groups(vu);
    if vu.vi[1] == 0 {
        vu.setvi(1, 0);
        0xf40
    } else {
        vu.setvi(1, 2);
        0xf30
    }
}

// ---------------------------------------------------------------------------
// Typed entry points: the routines' arithmetic on plain values, for callers
// whose EE side is hand-written too (no microprogram runs, no VU state is
// touched). They return everything the routine leaves in VF, ACC and VI,
// which such a caller writes back; `typed_tests` checks them against the
// routines above. The flags are not modelled (VCALLMS callers read results
// from VI).

fn mul(s: V, t: V) -> (V, u32) {
    fast::mul4::<XYZW, true, true>(&s, &t)
}
fn add(s: V, t: V) -> (V, u32) {
    fast::add4::<XYZW, true, true>(&s, &t)
}
fn madd(a: V, s: V, t: V) -> (V, u32) {
    fast::madd4::<XYZW, true, true, true>(&a, &s, &t)
}
fn msub(a: V, s: V, t: V) -> (V, u32) {
    fast::msub4::<XYZW, true, true, true>(&a, &s, &t)
}
/// `to` with the lanes `d` of `v`.
fn with(mut to: V, d: u8, v: V) -> V {
    merge(&mut to, d, v);
    to
}

/// What the box test (0x570) leaves: the corners in VF1-8, ACC, VI1-5 and
/// the clip flags.
#[derive(Debug, PartialEq)]
pub struct BoxClip {
    pub corners: [V; 8],
    pub acc: V,
    /// VI1 (OR of the corners' clip judgements), VI2 (AND), VI3/VI4 (the
    /// last two FCGET reads), VI5 (63).
    pub vi: [u16; 5],
    pub clip: u32,
}

/// 0x570 on values: the box `lo`..`hi` through `m` (rows x, y, z,
/// translation; VF10-13), `vf0` = VF0, `clip` = the clip flags before.
pub fn box_clip_typed(lo: V, hi: V, m: [V; 4], vf0: V, clip: u32) -> BoxClip {
    let [mx, my, mz, mt] = m;
    let mut acc = [0; 4];
    let mut corners = [[0; 4]; 8];
    for (k, (z, y)) in [(lo, lo), (lo, hi), (hi, lo), (hi, hi)].into_iter().enumerate() {
        acc = mul(mt, bc(vf0, 3)).0;
        acc = madd(acc, mz, bc(z, 2)).0;
        acc = madd(acc, my, bc(y, 1)).0;
        corners[2 * k] = madd(acc, mx, bc(lo, 0)).0;
        corners[2 * k + 1] = madd(acc, mx, bc(hi, 0)).0;
    }
    let (mut clip, mut any, mut all, mut reads) = (clip, 0u16, 63u16, [0u16; 8]);
    for (k, p) in corners.iter().enumerate() {
        clip = ((clip << 6) | fast::clip(*p, p[3])) & 0xff_ffff;
        reads[k] = (clip & 0xfff) as u16;
        any |= reads[k];
        all &= reads[k];
    }
    BoxClip { corners, acc, vi: [any & 63, all, reads[6], reads[7], 63], clip }
}

/// What the plane-set test (0x948) leaves: VF1-5 (the planes, xyz scaled
/// by the sphere's as far as the test got), VF7, ACC and VI1-3.
#[derive(Debug, PartialEq)]
pub struct SpherePlanes {
    pub planes: [V; 5],
    pub vf7: V,
    pub acc: V,
    /// VI1: 1 if the sphere is entirely in front of one of the planes, 2 if
    /// it crosses one, 0 if it is behind all five (inside the volume).
    pub vi: [u16; 3],
}

/// 0x948 on values: the sphere `s` (VF6: centre, w radius) against
/// `planes` (VF1-5); `vf7`, `acc` the registers before, `vf0` = VF0.
pub fn sphere_against_planes_typed(planes: [V; 5], s: V, vf7: V, acc: V, vf0: V) -> SpherePlanes {
    let mut p = planes;
    let mut acc = acc;
    let scale = |p: &mut [V; 5], r: usize| p[r - 1] = with(p[r - 1], XYZ, mul(p[r - 1], s).0);
    // Plane r's distance in ACC.x: x + y, + 1·z, + 1·w.
    let dist = |acc: &mut V, p: &[V; 5], vf7: V, r: usize| {
        let q = p[r - 1];
        *acc = with(*acc, X, add(q, bc(q, 1)).0);
        *acc = with(*acc, X, madd(*acc, vf7, bc(q, 2)).0);
        *acc = with(*acc, X, madd(*acc, vf7, bc(q, 3)).0);
    };
    // ACC.x ± radius: the sign of x.
    let plus_r = |acc: V, vf7: V| (madd(acc, vf7, bc(s, 3)).1 as u16) & SIGN_X;
    let minus_r = |acc: V, vf7: V| (msub(acc, vf7, bc(s, 3)).1 as u16) & SIGN_X;
    scale(&mut p, 1);
    let vf7 = with(vf7, X, add(vf0, bc(vf0, 3)).0);
    let mut vi = [0, SIGN_X, 0u16, 0u16];
    scale(&mut p, 2);
    scale(&mut p, 5);
    dist(&mut acc, &p, vf7, 1);
    let mut far = plus_r(acc, vf7);
    scale(&mut p, 3);
    let mut near = minus_r(acc, vf7);
    let (mut t, mut o) = (3, 2);
    let done = |p: [V; 5], acc: V, vi: [u16; 4]| SpherePlanes { planes: p, vf7, acc, vi: [vi[1], vi[2], vi[3]] };
    for r in 2..=5 {
        dist(&mut acc, &p, vf7, r);
        vi[t] = far & vi[1];
        let out = vi[t] == vi[1];
        let f = plus_r(acc, vf7);
        vi[t] = near & vi[1];
        if out {
            vi[1] = 1;
            return done(p, acc, vi);
        }
        if r == 2 {
            scale(&mut p, 4);
        }
        let n = minus_r(acc, vf7);
        vi[t] |= vi[o];
        far = f;
        near = n;
        std::mem::swap(&mut t, &mut o);
    }
    vi[3] = far & vi[1];
    if vi[3] == vi[1] {
        vi[1] = 1;
        return done(p, acc, vi);
    }
    vi[1] &= near;
    vi[2] |= vi[1];
    vi[1] = if vi[2] == 0 { 0 } else { 2 };
    done(p, acc, vi)
}

#[cfg(test)]
mod typed_tests {
    use super::*;

    fn rng(seed: &mut u64) -> u64 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    }
    /// Mostly moderate floats, sometimes zero or PS2 specials.
    fn float(seed: &mut u64) -> u32 {
        match rng(seed) % 16 {
            0 => 0,
            1 => (rng(seed) as u32 & 0x807f_ffff) | 0x7f80_0000,
            2 => rng(seed) as u32 & 0x807f_ffff,
            _ => ((rng(seed) % 2001) as f32 - 1000.0).to_bits(),
        }
    }
    fn vec(seed: &mut u64) -> V {
        [float(seed), float(seed), float(seed), float(seed)]
    }
    fn state(seed: &mut u64) -> Vu {
        let mut vu = Vu::new(0);
        for k in 1..32 {
            vu.vf[k] = vec(seed);
        }
        vu.acc = vec(seed);
        for k in 1..16 {
            vu.vi[k] = rng(seed) as u16;
        }
        vu.clip = rng(seed) as u32 & 0xff_ffff;
        // Planes and spheres that get past the early exits.
        if rng(seed) % 2 == 0 {
            for p in 1..=5 {
                vu.vf[p][3] = ((rng(seed) % 100) as f32 + 1.0).to_bits();
            }
            vu.vf[6][3] = ((rng(seed) % 4) as f32).to_bits();
        }
        vu
    }
    fn run_routine(vu: &mut Vu, pc: u32) {
        vu.start(pc);
        let VuExit::End = run(vu, pc, &mut |_| {}) else { panic!() };
    }

    #[test]
    fn box_clip_typed_matches_routine() {
        let mut seed = 0x0b0c_5eed_u64;
        for _ in 0..20000 {
            let mut vu = state(&mut seed);
            let t = box_clip_typed(vu.vf[15], vu.vf[16], [vu.vf[10], vu.vf[11], vu.vf[12], vu.vf[13]], vu.vf[0], vu.clip);
            run_routine(&mut vu, 0x570);
            let want = BoxClip {
                corners: std::array::from_fn(|k| vu.vf[1 + k]),
                acc: vu.acc,
                vi: [vu.vi[1], vu.vi[2], vu.vi[3], vu.vi[4], vu.vi[5]],
                clip: vu.clip,
            };
            assert_eq!(t, want);
        }
    }

    #[test]
    fn sphere_against_planes_typed_matches_routine() {
        let mut seed = 0x5fe7_e5ee_u64;
        let mut results = [0u32; 3];
        for _ in 0..20000 {
            let mut vu = state(&mut seed);
            let t = sphere_against_planes_typed(std::array::from_fn(|k| vu.vf[1 + k]), vu.vf[6], vu.vf[7], vu.acc, vu.vf[0]);
            run_routine(&mut vu, 0x948);
            let want = SpherePlanes {
                planes: std::array::from_fn(|k| vu.vf[1 + k]),
                vf7: vu.vf[7],
                acc: vu.acc,
                vi: [vu.vi[1], vu.vi[2], vu.vi[3]],
            };
            assert_eq!(t, want);
            results[(t.vi[0] as usize).min(2)] += 1;
        }
        assert!(results.iter().all(|&n| n > 100), "outcomes: {results:?}");
    }
}

#[cfg(all(test, feature = "microcode", not(target_arch = "wasm32")))]
mod tests {
    use super::super::capture::{ArchState, Record};
    use super::super::generated::vu_27649fa55223cbbf as translated;
    use super::*;

    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }
        fn below(&mut self, n: u64) -> u64 {
            self.next() % n
        }
        /// Mostly moderate floats, sometimes zero, tiny, huge or PS2
        /// specials (exponent 255).
        fn float(&mut self) -> u32 {
            match self.below(32) {
                0 => 0,
                1 => 0x8000_0000,
                2 => (self.next() as u32 & 0x807f_ffff) | 0x7f80_0000,
                3 => self.next() as u32 & 0x807f_ffff,
                4 => (self.next() as u32 & 0x80ff_ffff) | 0x7e00_0000,
                _ => {
                    let v = (self.below(2001) as f32 - 1000.0) / 100.0 * (1u32 << self.below(4)) as f32;
                    v.to_bits()
                }
            }
        }
        fn vec(&mut self) -> V {
            [self.float(), self.float(), self.float(), self.float()]
        }
    }

    fn random_state(r: &mut Rng, entry: u32) -> Vu {
        let mut vu = Vu::new(0);
        for k in 1..32 {
            vu.vf[k] = r.vec();
        }
        vu.acc = r.vec();
        for k in 1..16 {
            vu.vi[k] = r.next() as u16;
        }
        vu.vi[14] = r.below(16) as u16;
        vu.q = r.float();
        vu.p = r.float();
        vu.i = r.float();
        vu.clip = r.next() as u32 & 0xff_ffff;
        vu.status = r.next() as u32 & 0xfff;
        vu.mac = r.next() as u32 & 0xffff;
        for b in vu.data.chunks_exact_mut(4) {
            b.copy_from_slice(&r.float().to_le_bytes());
        }
        // Push the plane tests past their early exits: planes the sphere is
        // in front of, small radii, sometimes.
        if r.below(2) == 0 {
            let planes: Vec<usize> = match entry {
                0x828 | 0x948 => (1..=5).collect(),
                _ => vec![],
            };
            for p in planes {
                vu.vf[p][3] = (r.below(100) as f32 + 1.0).to_bits();
            }
            for q in 80..100 {
                let o = q * 16 + 12;
                vu.data[o..o + 4].copy_from_slice(&(r.below(100) as f32 + 1.0).to_bits().to_le_bytes());
            }
            vu.vf[6][3] = (r.below(4) as f32).to_bits();
            vu.vf[22][3] = (r.below(4) as f32).to_bits();
        }
        // Triangles the segment can hit.
        if entry == 0x6e0 && r.below(2) == 0 {
            let p = |x: f32, y: f32, z: f32| [x.to_bits(), y.to_bits(), z.to_bits(), 1f32.to_bits()];
            let f = |r: &mut Rng| (r.below(200) as f32 - 100.0) / 10.0;
            vu.vf[10] = p(f(r), f(r), 0.0);
            vu.vf[11] = p(f(r), f(r), 0.0);
            vu.vf[12] = p(f(r), f(r), 0.0);
            vu.vf[13] = p(f(r), f(r), f(r));
        }
        // Boxes around the origin, in view.
        if matches!(entry, 0x320 | 0x570 | 0xdb8 | 0xe80) && r.below(2) == 0 {
            let s = |x: f32| x.to_bits();
            let a = r.below(5) as f32;
            vu.vf[15] = [s(-a), s(-a), s(-a), s(1.0)];
            vu.vf[16] = [s(a), s(a), s(a), s(1.0)];
            vu.vf[20] = vu.vf[15];
            vu.vf[21] = vu.vf[16];
            let id = |vu: &mut Vu, k: usize, row: usize| {
                vu.vf[k] = [0; 4];
                vu.vf[k][row] = s(1.0);
            };
            for (k, row) in [(10, 0), (11, 1), (12, 2), (13, 3)] {
                id(&mut vu, k, row);
                vu.vf[13][3] = s(r.below(8) as f32);
            }
        }
        vu
    }

    fn run_translated(vu: &mut Vu, pc: u32) {
        vu.start(pc);
        vu.budget = u64::MAX;
        let mut pc = vu.tpc;
        loop {
            match translated::run(vu, pc, &mut |_| {}) {
                VuExit::End => break,
                VuExit::Jump(t) => pc = t,
                VuExit::Budget(_) => panic!("budget"),
            }
        }
    }

    fn run_hand(vu: &mut Vu, pc: u32) {
        vu.start(pc);
        let VuExit::End = run(vu, pc, &mut |_| {}) else { panic!() };
    }

    fn compare(mut a: Vu, mut b: Vu, pc: u32, what: &str) {
        run_translated(&mut a, pc);
        run_hand(&mut b, pc);
        let (sa, sb) = (ArchState::of(&a), ArchState::of(&b));
        if sa != sb || a.data != b.data {
            let diff: Vec<String> = (0..32)
                .filter(|&k| sa.vf[k] != sb.vf[k])
                .map(|k| format!("vf{k} {:08x?} != {:08x?}", sa.vf[k], sb.vf[k]))
                .chain((0..16).filter(|&k| sa.vi[k] != sb.vi[k]).map(|k| format!("vi{k} {:#x} != {:#x}", sa.vi[k], sb.vi[k])))
                .chain([("acc", sa.acc != sb.acc), ("q", sa.q != sb.q), ("i", sa.i != sb.i)].iter().filter(|x| x.1).map(|x| x.0.to_string()))
                .chain([("mac", sa.mac, sb.mac), ("status", sa.status, sb.status), ("clip", sa.clip, sb.clip), ("tpc", sa.tpc, sb.tpc)].iter().filter(|x| x.1 != x.2).map(|x| format!("{} {:#x} != {:#x}", x.0, x.1, x.2)))
                .chain((a.data != b.data).then(|| "data".to_string()))
                .collect();
            panic!("{what} at {pc:#05x}: {}", diff.join(", "));
        }
    }

    #[test]
    fn matches_translation() {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15);
        for &pc in &ENTRIES {
            for n in 0..3000 {
                let vu = random_state(&mut r, pc);
                let copy = clone(&vu);
                compare(vu, copy, pc, &format!("random state {n}"));
            }
        }
    }

    fn clone(vu: &Vu) -> Vu {
        let mut c = Vu::new(0);
        ArchState::of(vu).apply(&mut c);
        c.data = vu.data.clone();
        c
    }

    /// Real calls: `SSX3_VU0_CAPTURE=<file>` (`SSX3_VU_CAPTURE` output of
    /// a game run).
    #[test]
    fn matches_translation_on_captures() {
        let Ok(path) = std::env::var("SSX3_VU0_CAPTURE") else { return };
        let bytes = std::fs::read(path).unwrap();
        let mut n = 0;
        for rec in Record::read_all(&bytes).into_iter().filter(|r| r.unit == 0 && r.hash == HASH) {
            let mut vu = Vu::new(0);
            rec.start.apply(&mut vu);
            vu.data = rec.data_in.clone();
            let copy = clone(&vu);
            compare(vu, copy, rec.pc, &format!("capture {}", rec.seq));
            n += 1;
        }
        eprintln!("{n} captured calls match");
    }
}
