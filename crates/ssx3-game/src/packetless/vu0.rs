//! VU0 macro-mode arithmetic as the game's draw functions use it, typed:
//! each helper is the instruction sequence the compiled code runs, with the
//! PS2 float model of `ssx3_vu::vu::fast` / `ssx3_ee_rt::float`, so the
//! results are bit-identical to the generated code's.

use ssx3_ee_rt::float;
use ssx3_vu::vu::fast;

pub type V = [u32; 4];
pub type M = [V; 4];

pub const ONE: u32 = 0x3f80_0000;

pub fn mul(a: &V, b: &V) -> V {
    fast::mul4::<0b1111, true, true>(a, b).0
}

pub fn madd(acc: &V, a: &V, b: &V) -> V {
    fast::madd4::<0b1111, true, true, true>(acc, a, b).0
}

pub fn add(a: &V, b: &V) -> V {
    fast::add4::<0b1111, true, true>(a, b).0
}

pub fn sub(a: &V, b: &V) -> V {
    fast::sub4::<0b1111, true, true>(a, b).0
}

/// A lane broadcast (`.x` .. `.w`).
pub fn bc(v: &V, l: usize) -> V {
    [v[l]; 4]
}

/// `v` through `m` (`vmulax / vmadday / vmaddaz / vmaddw`):
/// `m0·v.x + m1·v.y + m2·v.z + m3·v.w`.
pub fn xform(m: &M, v: &V) -> V {
    let acc = mul(&m[0], &bc(v, 0));
    let acc = madd(&acc, &m[1], &bc(v, 1));
    let acc = madd(&acc, &m[2], &bc(v, 2));
    madd(&acc, &m[3], &bc(v, 3))
}

/// Each row of `b` through `a` (the row-vector product `b · a`).
pub fn mul_m(a: &M, b: &M) -> M {
    std::array::from_fn(|i| xform(a, &b[i]))
}

/// Row `r` of `m` times lane `r` of `s` (`vmulx / y / z / w`).
pub fn scale_rows(m: &M, s: &V) -> M {
    std::array::from_fn(|r| mul(&m[r], &bc(s, r)))
}

/// `v · rsqrt(v·v)` as the compiled code normalises (vf0 = (0, 0, 0, 1)):
/// `vaddw.x vf6, vf0, vf0w` (1.0), `vmul t = v·v`, `vaddaxy`, `vmaddaz`,
/// `vmaddw` (x lane), `vrsqrt Q = vf0w / sqrt(t.x)`, `vmulq`.
pub fn normalize4(v: &V) -> V {
    let vf0: V = [0, 0, 0, ONE];
    let vf6 = add(&vf0, &bc(&vf0, 3));
    let t = mul(v, v);
    let acc = add(&t, &bc(&t, 1));
    let acc = madd(&acc, &vf6, &bc(&t, 2));
    let d = madd(&acc, &vf6, &bc(&t, 3));
    let q = float::rsqrt(ONE, d[0]).value;
    mul(v, &[q; 4])
}

pub fn max(a: u32, b: u32) -> u32 {
    float::max(a, b)
}

pub fn min(a: u32, b: u32) -> u32 {
    float::min(a, b)
}

/// `vftoi0` of one lane.
pub fn ftoi0(a: u32) -> u32 {
    float::to_fixed(a, 0)
}
