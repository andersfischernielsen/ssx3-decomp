//! Work package A1 math: sin/cos/asin/atan, angle wrapping, 1-D noise, cubic
//! spline segments, the multi-word random generator, libm bit helpers
//! (`fabsf`, `floorf`, ...), 64-bit multiply
//! (`docs/investigation/phase2-candidates.md`, package A1).
//!
//! Everything here is COP1 (FPU) code: floats are [`FpuF32`] (truncating
//! PS2 arithmetic, no inf/NaN, two roundings per multiply-add) and every
//! expression keeps the compiled operation order, since reordering changes
//! the last bit. Beyond the result, a replacement leaves the registers the
//! differential test treats as observable (v0/v1, f0/f1, the FCC bit of
//! fcr31) exactly as the original does; those incidental values are set in
//! one place at the end of each function and marked as such.

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::{A0, A1, V0, V1};
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_be50, sin_cos, "Math_SinCos"),
    (0x0031_bf60, sin, "Math_Sin"),
    (0x0031_c040, cos, "Math_Cos"),
    (0x0031_c128, asin, "Math_Asin"),
    (0x0031_c228, atan, "Math_Atan"),
    (0x0016_bce0, wrap_angle_pi, "Math_WrapAnglePi"),
    (0x002d_18b0, noise_sample_1d, "Noise_Sample1D"),
    (
        0x0031_d5a8,
        cubic_spline_eval_segment,
        "cCSICubicSpline_EvalSegment",
    ),
    (0x0031_7a08, rand_next, "Rand_Next"),
    (0x0031_7958, rand_seed, "Rand_Seed"),
    (0x0040_f8a0, fabsf, "fabsf"),
    (0x0040_f9d0, isnanf, "isnanf"),
    (0x0040_f8c0, finitef, "finitef"),
    (0x0040_fb58, copysignf, "copysignf"),
    (0x0040_f8e8, floorf, "floorf"),
    (0x0041_14d0, muldi3, "__muldi3"),
    (
        0x002b_2690,
        song_instance_get_beat_ms,
        "SongInstance_GetBeatMs",
    ),
    (0x0023_1f80, load_indicator_advance, "LoadIndicator_Advance"),
];

// ---------------------------------------------------------------------------
// Constants. The `.lit4` literal pool (compiler-emitted float literals, never
// written) is hard-coded; values were read from the ELF.

const HALF: FpuF32 = FpuF32(0x3f00_0000);
const NEG_HALF: FpuF32 = FpuF32(0xbf00_0000);
const ONE: FpuF32 = FpuF32::ONE;
/// 2/pi (`.lit4` 0x4a0230 and copies).
const TWO_OVER_PI: FpuF32 = FpuF32(0x3f22_f983);
/// pi/2 (`.lit4` 0x4a0234 and copies).
const HALF_PI: FpuF32 = FpuF32(0x3fc9_0fdb);
/// sin(p) ~ p + p*s*(S3 + s*(S2 + s*(S1 + s*S0))), s = p^2: the Taylor-like
/// coefficients -1/6, 1/120, ... from highest to lowest power.
const SIN_POLY: [FpuF32; 4] = [
    FpuF32(0x3638_ef1f),
    FpuF32(0xb950_0d03),
    FpuF32(0x3c08_8889),
    FpuF32(0xbe2a_aaab),
];
/// cos(p) ~ 1 - s/2 + s^2*(C2 + s*(C1 + s*C0)), s = p^2.
const COS_POLY: [FpuF32; 3] = [
    FpuF32(0x37d0_0d03),
    FpuF32(0xbab6_0b62),
    FpuF32(0x3d2a_aaab),
];
/// -sqrt(1/2) / +sqrt(1/2): the switch points between the direct arcsine
/// series and the `pi/2 - asin(sqrt(1-x^2))` identity (0x4a0290 / 0x4a0298).
const NEG_SQRT_HALF: FpuF32 = FpuF32(0xbf35_04f3);
const SQRT_HALF: FpuF32 = FpuF32(0x3f35_04f3);
/// The arcsine/arctangent series coefficients: a 4-float global in `.sdata`
/// (0x4a3ee0), read from memory as the original does.
const ASIN_COEFFS: Addr = Addr(0x004a_3ee0);
/// 1/(2 pi) and 2 pi (0x49cc78, 0x49cc7c).
const INV_TWO_PI: FpuF32 = FpuF32(0x3e22_f983);
const TWO_PI: FpuF32 = FpuF32(0x40c9_0fdb);
/// The 1-D value-noise table: 160 samples plus one wrap-around entry (`.data`).
const NOISE_TABLE: Addr = Addr(0x0044_5ab0);
const NOISE_PERIOD: i32 = 160;
/// Per-frame phase step of the loading indicator, 1/60 in the executable:
/// read from its `.lit4` entry, which the tick rate rewrites (`crate::tick`).
const LOAD_INDICATOR_STEP_AT: Addr = Addr(0x0049_df48);
#[cfg(test)]
const LOAD_INDICATOR_STEP: FpuF32 = FpuF32(0x3c88_8889);
/// The "raise inexact" probe of fdlibm's floorf.
const HUGE: FpuF32 = FpuF32(0x7149_f2ca);

// ---------------------------------------------------------------------------
// Trigonometry.

/// Quarter-turn range reduction shared by `Math_Sin`, `Math_Cos` and
/// `Math_SinCos`: `n = round(x * 2/pi)` (ties away from zero, via truncation
/// of `x*2/pi +- 0.5`) and `p = x - n*pi/2`, so x = n quarter turns + p with
/// |p| <= pi/4. Charges the first two blocks (6, then 4 for x < 0 or 3).
fn reduce_quarter_turns(c: &mut Cpu, x: FpuF32) -> (i32, FpuF32) {
    c.cycles += 6;
    let scaled = x * TWO_OVER_PI;
    let biased = if abi::c_lt_s(c, x, FpuF32::ZERO) {
        c.cycles += 4;
        scaled - HALF
    } else {
        c.cycles += 3;
        scaled + HALF
    };
    let n = biased.to_i32();
    let p = x - FpuF32::from_i32(n) * HALF_PI;
    (n, p)
}

/// The first three Horner steps `(s*k0 + k1)*s + k2`, which both series
/// share in shape (the original leaves this partial sum in f1).
fn horner3(s: FpuF32, k: &[FpuF32]) -> FpuF32 {
    (s * k[0] + k[1]) * s + k[2]
}

/// `sin(p)` for |p| <= pi/4 as `Math_Sin`/`Math_Cos` evaluate it:
/// `((((s*S0 + S1)*s + S2)*s + S3)*s)*p + p`. Returns (value, f1 partial).
fn sin_series(p: FpuF32, s: FpuF32) -> (FpuF32, FpuF32) {
    let partial = horner3(s, &SIN_POLY);
    let v = (partial * s + SIN_POLY[3]) * s * p + p;
    (v, partial)
}

/// `cos(p)` for |p| <= pi/4: `(((s*C0 + C1)*s + C2)*s - 0.5)*s + 1`.
/// Returns (value, f1 partial).
fn cos_series(s: FpuF32) -> (FpuF32, FpuF32) {
    let partial = horner3(s, &COS_POLY);
    let v = (partial * s + NEG_HALF) * s + ONE;
    (v, partial)
}

/// `float Math_Sin(float rad)` (0x31bf60): sine by quarter-turn reduction
/// and a degree-9 odd / degree-8 even polynomial.
pub fn sin(c: &mut Cpu) {
    let x = abi::farg(c, 0);
    let (n, p) = reduce_quarter_turns(c, x);
    let s = p * p;
    c.cycles += 9;
    // Odd quadrants are the cosine of the remainder.
    let (v, partial) = if n & 1 != 0 {
        c.cycles += 16;
        cos_series(s)
    } else {
        c.cycles += 13;
        sin_series(p, s)
    };
    // Quadrants 2 and 3 are negated (sign flip, so +0 becomes -0).
    c.cycles += 3;
    let v = if n & 2 != 0 { -v } else { v };
    c.cycles += 2;
    abi::set_fret(c, v);
    abi::set_fpr(c, 1, partial);
    abi::set_reg(c, V1, sx(n as u32));
    abi::set_reg(c, V0, sx(n as u32) & 2);
    abi::ret(c);
}

/// `float Math_Cos(float rad)` (0x31c040): cosine, the same reduction as
/// [`sin`] shifted by a quarter turn.
pub fn cos(c: &mut Cpu) {
    let x = abi::farg(c, 0);
    let (n, p) = reduce_quarter_turns(c, x);
    let s = p * p;
    c.cycles += 9;
    // Odd quadrants are the sine of the remainder.
    let (v, partial) = if n & 1 != 0 {
        c.cycles += 14;
        sin_series(p, s)
    } else {
        c.cycles += 15;
        cos_series(s)
    };
    // Quadrants 1 and 2 are negated.
    c.cycles += 4;
    let sign = sx(n.wrapping_add(1) as u32) & 2;
    let v = if sign != 0 { -v } else { v };
    c.cycles += 2;
    abi::set_fret(c, v);
    abi::set_fpr(c, 1, partial);
    abi::set_reg(c, V1, sx(n as u32));
    abi::set_reg(c, V0, sign);
    abi::ret(c);
}

/// `void Math_SinCos(float rad, float *out_sin, float *out_cos)` (0x31be50):
/// sine and cosine together. The sine of the remainder uses its own
/// polynomial form `(((s*S0 + S1)*s + S2)*s + S3)*s + 1) * p` (not the one
/// of [`sin`]; they differ in the last bit), the cosine is
/// `sqrt(1 - sin^2)` (always >= +0), and the quadrant permutes and negates
/// the pair.
pub fn sin_cos(c: &mut Cpu) {
    let x = abi::farg(c, 0);
    let out_sin = Addr(abi::arg(c, 0));
    let out_cos = Addr(abi::arg(c, 1));
    let (n, p) = reduce_quarter_turns(c, x);
    c.cycles += 29;
    let s = p * p;
    let factor = ((horner3(s, &SIN_POLY) * s + SIN_POLY[3]) * s) + ONE;
    let sin_p = factor * p;
    let cos_p = (ONE - sin_p * sin_p).sqrt();

    // Stores in the original order: sine slot first.
    let quadrant = n & 3;
    let (sin_x, cos_x, f0) = match quadrant {
        0 => {
            c.cycles += 2 + 2 + 2;
            (sin_p, cos_p, factor)
        }
        1 => {
            c.cycles += 4;
            (cos_p, -sin_p, -sin_p)
        }
        2 => {
            c.cycles += 2 + 2 + 5;
            (-sin_p, -cos_p, -sin_p)
        }
        _ => {
            c.cycles += 2 + 2 + 2 + 3;
            (-cos_p, sin_p, -cos_p)
        }
    };
    out_sin.write_fpu(c, sin_x);
    out_cos.write_fpu(c, cos_x);
    // Incidental registers: f0/f1 hold the last values computed per branch,
    // v1 the quadrant, v0 the last immediate of the quadrant dispatch.
    abi::set_fpr(c, 0, f0);
    abi::set_fpr(c, 1, if quadrant == 2 { -cos_p } else { cos_p });
    abi::set_reg(c, V1, quadrant as u64);
    abi::set_reg(c, V0, if quadrant < 2 { 1 } else { 3 });
    abi::ret(c);
}

/// The arcsine/arctangent series tail `t*(K0 + t*(K1 + t*(K2 + t*K3)))` over
/// the `.sdata` coefficients, in the original order. Returns (value, the
/// partial `t*(K2 + t*K3)` the original leaves in f1).
fn asin_series(c: &mut Cpu, t: FpuF32) -> (FpuF32, FpuF32) {
    let k: [FpuF32; 4] = std::array::from_fn(|i| ASIN_COEFFS.off(4 * i as u32).read_fpu(c));
    let partial = t * (k[2] + t * k[3]);
    let v = t * (k[0] + t * (k[1] + partial));
    (v, partial)
}

/// `float Math_Asin(float x)` (0x31c128): arcsine. Near +-1 it uses
/// asin(x) = +-(pi/2 - asin(sqrt(1 - x^2))) with the series evaluated at
/// t = 1 - x^2, in the middle the odd series `x + x*s*Q(s)`, s = x^2.
/// Out-of-range inputs don't fault (the PS2 sqrt takes |t|).
pub fn asin(c: &mut Cpu) {
    let x = abi::farg(c, 0);
    c.cycles += 5;
    let x2 = x * x;
    let (v, partial) = if abi::c_lt_s(c, x, NEG_SQRT_HALF) {
        c.cycles += 20;
        let t = ONE - x2;
        let root = t.sqrt();
        let (q, partial) = asin_series(c, t);
        ((root * (q + ONE)) + (-HALF_PI), partial)
    } else {
        c.cycles += 5;
        if abi::c_lt_s(c, SQRT_HALF, x) {
            c.cycles += 20;
            let t = ONE - x2;
            let root = t.sqrt();
            let (q, partial) = asin_series(c, t);
            (HALF_PI - root * (q + ONE), partial)
        } else {
            c.cycles += 14;
            let (q, partial) = asin_series(c, x2);
            (q * x + x, partial)
        }
    };
    abi::set_fret(c, v);
    abi::set_fpr(c, 1, partial);
    abi::ret(c);
}

/// `float Math_Atan(float x)` (0x31c228): arctangent through the arcsine
/// series: atan(x) = asin(x / sqrt(1 + x^2)), evaluated on u = x^2/(1+x^2)
/// for |x| <= 1 and as +-(pi/2 - ...) on u = 1/(1+x^2) beyond.
pub fn atan(c: &mut Cpu) {
    let x = abi::farg(c, 0);
    c.cycles += 6;
    let (v, partial) = if abi::c_lt_s(c, x, -ONE) {
        c.cycles += 21;
        let u = ONE.div(x * x + ONE);
        let root = u.sqrt();
        let (q, partial) = asin_series(c, u);
        (root * (q + ONE) + (-HALF_PI), partial)
    } else {
        c.cycles += 6;
        let x2 = x * x;
        if abi::c_lt_s(c, ONE, x) {
            c.cycles += 19;
            let u = ONE.div(x2 + ONE);
            let root = u.sqrt();
            let (q, partial) = asin_series(c, u);
            (HALF_PI - root * (q + ONE), partial)
        } else {
            c.cycles += 19;
            let u = x2.div(x2 + ONE);
            let root = u.sqrt();
            let (q, partial) = asin_series(c, u);
            let r = root * (q + ONE);
            // Odd symmetry; -0.0 is not below 0, so it keeps +0.
            let r = if abi::c_lt_s(c, x, FpuF32::ZERO) {
                c.cycles += 1;
                -r
            } else {
                r
            };
            c.cycles += 2;
            (r, partial)
        }
    };
    abi::set_fret(c, v);
    abi::set_fpr(c, 1, partial);
    abi::ret(c);
}

/// `floor(t)` as the game computes it in FPU code: truncate with
/// `cvt.w.s`/`cvt.s.w`, then step down by one when that rounded up (negative
/// non-integers). Charges `fixup_cycles` for the step. Sets FCC.
fn fpu_floor(c: &mut Cpu, t: FpuF32, fixup_cycles: u64) -> FpuF32 {
    let fl = FpuF32::from_i32(t.to_i32());
    if abi::c_lt_s(c, t, fl) {
        c.cycles += fixup_cycles;
        fl - ONE
    } else {
        fl
    }
}

/// `void Math_WrapAnglePi(float *angle)` (0x16bce0): wraps an angle in
/// place into [-pi, pi): `a - floor(a/(2 pi) + 1/2) * 2 pi`.
pub fn wrap_angle_pi(c: &mut Cpu) {
    let angle = Addr(abi::arg(c, 0));
    c.cycles += 11;
    let a = angle.read_fpu(c);
    let turns = a * INV_TWO_PI + HALF;
    let whole = fpu_floor(c, turns, 3);
    c.cycles += 5;
    let r = a - whole * TWO_PI;
    angle.write_fpu(c, r);
    abi::set_fpr(c, 0, r);
    abi::set_fpr(c, 1, whole);
    abi::ret(c);
}

/// `void LoadIndicator_Advance(float *phase)` (0x231f80): advances the
/// loading-screen animation phase by one frame step and wraps it into
/// [0, 1). The unwrapped sum is stored first, then the wrapped value.
pub fn load_indicator_advance(c: &mut Cpu) {
    let phase = Addr(abi::arg(c, 0));
    c.cycles += 9;
    let s = phase.read_fpu(c) + LOAD_INDICATOR_STEP_AT.read_fpu(c);
    let whole = fpu_floor(c, s, 3);
    phase.write_fpu(c, s);
    c.cycles += 3;
    let r = s - whole;
    phase.write_fpu(c, r);
    abi::set_fpr(c, 0, r);
    abi::set_fpr(c, 1, whole);
    abi::ret(c);
}

/// `float Noise_Sample1D(float x)` (0x2d18b0): periodic 1-D value noise,
/// linear interpolation between table samples, 160 samples per unit of x.
/// The index is a C remainder, so negative x reads before the table (as the
/// original does).
pub fn noise_sample_1d(c: &mut Cpu) {
    let x = abi::farg(c, 0);
    c.cycles += 10 + 19;
    let y = x * FpuF32(0x4320_0000); // 160.0
    let n = y.to_i32();
    let i = n.wrapping_rem(NOISE_PERIOD);
    let frac = y - FpuF32::from_i32(n);
    let at = |i: i32| NOISE_TABLE.off((i as u32) << 2);
    let (a_at, b_at) = (at(i), at(i.wrapping_add(1)));
    let a = a_at.read_fpu(c) * (ONE - frac);
    let b = b_at.read_fpu(c) * frac;
    abi::set_fret(c, a + b);
    abi::set_fpr(c, 1, a);
    abi::set_reg(c, V1, sx(a_at.0));
    abi::set_reg(c, V0, sx(b_at.0));
    abi::ret(c);
}

/// One segment of a `cCSICubicSpline`: five floats {a, b, c, d, x0}.
#[derive(Clone, Copy, Debug)]
pub struct CubicSegment(pub Addr);

impl CubicSegment {
    /// Coefficient `k` (0 = cubic, 1 = quadratic, 2 = linear, 3 = constant).
    pub fn coeff(self, c: &mut Cpu, k: u32) -> FpuF32 {
        self.0.off(4 * k).read_fpu(c)
    }
    /// The knot the segment starts at.
    pub fn x0(self, c: &mut Cpu) -> FpuF32 {
        self.0.off(0x10).read_fpu(c)
    }
}

/// `float cCSICubicSpline_EvalSegment(float x, const Segment *seg)`
/// (0x31d5a8): `a t^3 + b t^2 + c t + d` with t = x - x0, evaluated as
/// `(t*a)*t^2 + ((c*t + d) + b*t^2)` (original order).
pub fn cubic_spline_eval_segment(c: &mut Cpu) {
    let x = abi::farg(c, 0);
    let seg = CubicSegment(Addr(abi::arg(c, 0)));
    c.cycles += 15;
    let t = x - seg.x0(c);
    let t2 = t * t;
    let (a, b, lin, d) = (
        seg.coeff(c, 0),
        seg.coeff(c, 1),
        seg.coeff(c, 2),
        seg.coeff(c, 3),
    );
    let quad = b * t2;
    let low = (lin * t + d) + quad;
    let v = (t * a) * t2 + low;
    abi::set_fret(c, v);
    abi::set_fpr(c, 1, quad);
    abi::ret(c);
}

/// `float SongInstance_GetBeatMs(SongInstance *song)` (0x2b2690): the beat
/// length in milliseconds, `(60 / bpm) * 1000`, with the tempo read from the
/// song's info record (+0x90).
pub fn song_instance_get_beat_ms(c: &mut Cpu) {
    let song = Addr(abi::arg(c, 0));
    c.cycles += 9;
    let info = song.read_u32(c);
    let bpm = Addr(info).off(0x90).read_fpu(c);
    let ms = FpuF32(0x4270_0000).div(bpm) * FpuF32(0x447a_0000); // 60.0, 1000.0
    abi::set_fret(c, ms);
    abi::set_fpr(c, 1, bpm);
    abi::set_reg(c, V0, sx(info));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Random numbers.

/// The state of the game's random generator: six 32-bit words, s[0] most
/// significant, forming a 192-bit number.
#[derive(Clone, Copy, Debug)]
pub struct RandState(pub Addr);

impl RandState {
    pub fn word(self, c: &mut Cpu, i: u32) -> u32 {
        self.0.off(4 * i).read_u32(c)
    }
    pub fn set_word(self, c: &mut Cpu, i: u32, v: u32) {
        self.0.off(4 * i).write_u32(c, v)
    }
}

/// Offsets added to the seed for each state word (cumulative sums of the
/// constants the original adds one after another).
const RAND_SEED_OFFSETS: [u32; 6] = {
    let steps = [
        0xf22d_0e56u32,
        0x9604_1893,
        0x3df3_b646,
        0x40dd_e76d,
        0x9732_7ae1,
        0xd1a9_fbe7,
    ];
    let mut out = [0u32; 6];
    let mut acc = 0u32;
    let mut i = 0;
    while i < 6 {
        acc = acc.wrapping_add(steps[i]);
        out[i] = acc;
        i += 1;
    }
    out
};

/// `void Rand_Seed(RandState *state, u32 seed)` (0x317958): state[i] =
/// seed + fixed offset i.
pub fn rand_seed(c: &mut Cpu) {
    let state = RandState(Addr(abi::arg(c, 0)));
    let seed = abi::arg(c, 1);
    c.cycles += 43;
    let words = RAND_SEED_OFFSETS.map(|k| seed.wrapping_add(k));
    for (i, w) in words.iter().enumerate() {
        state.set_word(c, i as u32, *w);
    }
    abi::set_reg(c, V0, sx(words[4]));
    abi::set_reg(c, V1, sx(words[3]));
    abi::ret(c);
}

/// `int Rand_Next(RandState *state)` (0x317a08): one step of the additive
/// generator. The low two words are added into the upper four as a 192-bit
/// sum (s[0..4] += s[4..5] shifted, with the original's simplified carries),
/// s[5] is incremented as a counter rippling upward, and the new s[0] is
/// returned.
pub fn rand_next(c: &mut Cpu) {
    let st = RandState(Addr(abi::arg(c, 0)));
    let s: [u32; 6] = std::array::from_fn(|i| st.word(c, i as u32));

    // s4 += s5, with carry out (tested against either addend).
    c.cycles += 7;
    let u4 = s[5].wrapping_add(s[4]);
    let carry = if u4 < s[5] {
        c.cycles += 2;
        1
    } else {
        c.cycles += 3;
        if u4 < s[4] {
            c.cycles += 2;
            1
        } else {
            0
        }
    };
    // Ripple u4 into s3, s2, s1, s0; each carry is tested as `sum < old word`
    // on the carry-included sum.
    c.cycles += 23;
    let u3 = u4.wrapping_add(s[3]).wrapping_add(carry);
    let u2 = u3.wrapping_add(s[2]).wrapping_add((u3 < s[3]) as u32);
    let u2_plus_s1 = u2.wrapping_add(s[1]);
    let u1 = u2_plus_s1.wrapping_add((u2 < s[2]) as u32);
    let mut u0 = u1.wrapping_add(s[0]).wrapping_add((u1 < s[1]) as u32);
    st.set_word(c, 4, u4);
    st.set_word(c, 3, u3);
    st.set_word(c, 2, u2);
    st.set_word(c, 1, u1);
    let counter = s[5].wrapping_add(1);
    st.set_word(c, 5, counter);
    st.set_word(c, 0, u0);

    // The counter wrapped: increment s4, s3, s2, s1 in turn until one doesn't
    // wrap; if all do, s0 too.
    if counter == 0 {
        let mut all_wrapped = true;
        for i in (1..=4).rev() {
            c.cycles += 4;
            let w = st.word(c, i).wrapping_add(1);
            st.set_word(c, i, w);
            if w != 0 {
                all_wrapped = false;
                break;
            }
        }
        if all_wrapped {
            c.cycles += 3;
            u0 = u0.wrapping_add(1);
            st.set_word(c, 0, u0);
        }
    }
    c.cycles += 2;
    abi::set_ret(c, u0);
    abi::set_reg(c, V1, sx(u2_plus_s1));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// libm / libgcc bit helpers.

/// `float fabsf(float)` (0x40f8a0): clears the sign bit (no FPU rules).
pub fn fabsf(c: &mut Cpu) {
    c.cycles += 7;
    let r = abi::farg(c, 0).abs();
    abi::set_fret(c, r);
    abi::set_reg(c, V0, 0x7fff_ffff);
    abi::set_reg(c, V1, r.0 as u64);
    abi::ret(c);
}

/// `int isnanf(float)` (0x40f9d0): 1 for an IEEE NaN bit pattern (exponent
/// 255, mantissa != 0), else 0. An integer test; PS2 floats have no NaN.
pub fn isnanf(c: &mut Cpu) {
    c.cycles += 9;
    let mag = abi::farg(c, 0).0 & 0x7fff_ffff;
    abi::set_ret(c, 0x7f80_0000u32.wrapping_sub(mag) >> 31);
    abi::set_reg(c, A0, 0x7fff_ffff);
    abi::set_reg(c, V1, 0x7f80_0000);
    abi::ret(c);
}

/// `int finitef(float)` (0x40f8c0): 1 unless the exponent field is 255.
pub fn finitef(c: &mut Cpu) {
    c.cycles += 9;
    let mag = abi::farg(c, 0).0 & 0x7fff_ffff;
    abi::set_ret(c, mag.wrapping_add(0x8080_0000) >> 31);
    abi::set_reg(c, A0, 0x7fff_ffff);
    abi::set_reg(c, V1, sx(0x8080_0000));
    abi::ret(c);
}

/// `float copysignf(float x, float y)` (0x40fb58): |x| with y's sign (bits).
pub fn copysignf(c: &mut Cpu) {
    c.cycles += 12;
    let (x, y) = (abi::farg(c, 0).0, abi::farg(c, 1).0);
    let sign = abi::mfc1(FpuF32(y)) & 0xffff_ffff_8000_0000;
    let r = (abi::mfc1(FpuF32(x)) & 0x7fff_ffff) | sign;
    abi::set_fret(c, FpuF32(r as u32));
    abi::set_reg(c, A0, 0xffff_ffff_8000_0000);
    abi::set_reg(c, A1, r);
    abi::set_reg(c, V0, 0x7fff_ffff);
    abi::set_reg(c, V1, sign);
    abi::ret(c);
}

/// `float floorf(float x)` (0x40f8e8): fdlibm's bit-level floor. Clears the
/// fraction bits below the binary point, after adding one unit at the point
/// for negative non-integers; |x| < 1 gives +0, -1 or -0. The `x + 1e30 > 0`
/// probe only raises "inexact" in IEEE; it is always true here but sets FCC.
pub fn floorf(c: &mut Cpu) {
    let x = abi::farg(c, 0);
    c.cycles += 8;
    let i0 = abi::mfc1(x);
    let biased_exp = ((i0 as u32 as i32 >> 23) & 0xff) as u64;
    let j0 = biased_exp as i32 - 0x7f;
    let (v0, v1): (u64, u64);
    let mut bits = i0;
    // Probe `x + huge > 0`; returns whether the rounding path continues.
    let probe = |c: &mut Cpu| {
        c.cycles += 8;
        let sum = x + HUGE;
        abi::set_fpr(c, 0, sum);
        abi::set_fpr(c, 1, FpuF32::ZERO);
        abi::c_lt_s(c, FpuF32::ZERO, sum)
    };
    if j0 >= 23 {
        // Already integral (or exponent 255: x + x as the original returns).
        c.cycles += 2 + 2;
        let r = if j0 == 0x80 { x + x } else { x };
        abi::set_fret(c, r);
        abi::set_reg(c, V0, 0x80);
        abi::set_reg(c, V1, 0);
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    if j0 < 0 {
        // |x| < 1.
        if probe(c) {
            c.cycles += 2;
            if (i0 as i64) < 0 {
                c.cycles += 5;
                v1 = sx(0xbf80_0000); // -1.0
                v0 = i0 & 0x7fff_ffff;
                if v0 != 0 {
                    bits = v1;
                }
            } else {
                c.cycles += 2;
                v1 = 1;
                v0 = 0x7fff_0000;
                bits = 0;
            }
        } else {
            v1 = 1;
            v0 = biased_exp;
        }
    } else {
        c.cycles += 5;
        let frac_mask = sx(((0x007f_ffff_u32 as i32) >> (j0 & 31)) as u32);
        let frac = i0 & frac_mask;
        v1 = frac;
        if frac == 0 {
            // Integral already.
            c.cycles += 2;
            abi::set_fret(c, x);
            abi::set_reg(c, V0, 0x7f_ffff);
            abi::set_reg(c, V1, v1);
            abi::ret(c);
            return;
        }
        if probe(c) {
            c.cycles += 2;
            if (i0 as i64) < 0 {
                c.cycles += 4;
                let unit = sx(((0x0080_0000_u32 as i32) >> (j0 & 31)) as u32);
                bits = sx((bits as u32).wrapping_add(unit as u32));
            }
            c.cycles += 2;
            v0 = !frac_mask;
            bits &= v0;
        } else {
            v0 = 0x7f_ffff;
        }
    }
    c.cycles += 3;
    let r = FpuF32(bits as u32);
    c.f[12] = r.0;
    abi::set_fret(c, r);
    abi::set_reg(c, V0, v0);
    abi::set_reg(c, V1, v1);
    abi::ret(c);
}

/// `u64 __muldi3(u64 a, u64 b)` (0x4114d0): libgcc's 64-bit multiply from
/// 32-bit halves; the low 64 bits of the product.
pub fn muldi3(c: &mut Cpu) {
    c.cycles += 24;
    let (a, b) = (c.g[A0][0], c.g[A1][0]);
    let (a_lo, a_hi, b_lo, b_hi) = (a as u32, (a >> 32) as u32, b as u32, (b >> 32) as u32);
    // The cross terms the original sums in v1 (low words only).
    let cross = a_lo
        .wrapping_mul(b_hi)
        .wrapping_add(a_hi.wrapping_mul(b_lo));
    c.g[V0][0] = a.wrapping_mul(b);
    abi::set_reg(c, V1, sx(cross));
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, put_words, Rng, DATA, GAME_GP as GP};

    /// Put the game's constants where the generated code reads them (the
    /// test machine's RAM starts empty): `.lit4` literals, the arcsine
    /// coefficients, and gp.
    fn load_constants(c: &mut Cpu) {
        c.g[28][0] = GP as u64;
        const LIT_4A0230: [u32; 30] = [
            0x3f22f983, 0x3fc90fdb, 0x3638ef1f, 0xb9500d03, 0x3c088889, 0xbe2aaaab, // SinCos
            0x3f22f983, 0x3fc90fdb, 0x37d00d03, 0xbab60b62, 0x3d2aaaab, 0x3638ef1f, // Sin
            0xb9500d03, 0x3c088889, 0xbe2aaaab, 0x3f22f983, 0x3fc90fdb, 0x3638ef1f, // Sin/Cos
            0xb9500d03, 0x3c088889, 0xbe2aaaab, 0x37d00d03, 0xbab60b62, 0x3d2aaaab, // Cos
            0xbf3504f3, 0xbfc90fdb, 0x3f3504f3, 0x3fc90fdb, 0xbfc90fdb,
            0x3fc90fdb, // Asin/Atan
        ];
        put_words(c, 0x004a_0230, &LIT_4A0230);
        put_words(
            c,
            0x004a_3ee0,
            &[0x3e2bba25, 0x3d8a908b, 0x3d55033a, 0x3d6137ab],
        );
        put_words(c, 0x0049_cc78, &[INV_TWO_PI.0, TWO_PI.0]);
        put_words(c, 0x0049_df48, &[LOAD_INDICATOR_STEP.0]);
    }

    /// A float bit pattern biased toward edge cases: zeros, denormals,
    /// exponent 255, integers, values near the functions' switch points,
    /// and plain random values over several ranges.
    fn interesting(r: &mut Rng) -> u32 {
        let sign = r.next_u32() & 0x8000_0000;
        match r.next_u32() % 16 {
            0 => sign,
            1 => sign | (r.next_u32() & 0x007f_ffff), // denormal
            2 => sign | 0x7f80_0000 | (r.next_u32() & 0x007f_ffff), // exp 255
            3 => sign | 0x7f7f_ffff,
            4 => {
                // Small integers and half-integers (floor/wrap boundaries).
                let k = (r.next_u32() % 2001) as f32 - 1000.0;
                (if r.next_u32() % 2 == 0 { k } else { k + 0.5 }).to_bits()
            }
            5 => {
                sign | [
                    0x3f80_0000,
                    0x3f35_04f3,
                    0x3fc9_0fdb,
                    0x4049_0fdb,
                    0x3f00_0000,
                ][r.next_u32() as usize % 5]
            }
            6 => {
                // One ulp around a switch point.
                let base = [0x3f80_0000u32, 0x3f35_04f3, 0x3f00_0000, 0x3fc9_0fdb]
                    [r.next_u32() as usize % 4];
                sign | base.wrapping_add(r.next_u32() % 3).wrapping_sub(1)
            }
            7 => r.next_u32(), // any bits
            8 => r.f32(1.0),
            9 => r.f32(4.0),
            10 => r.f32(100.0),
            11 => r.f32(1e6),
            12 => r.f32(1e30),
            13 => sign | ((r.next_u32() % 40 + 0x67) << 23) | (r.next_u32() & 0x007f_ffff),
            14 => ((r.next_u32() as i32 % 100000) as f32).to_bits(),
            _ => r.f32(10.0),
        }
    }

    /// Random float argument in f12 (and f13), constants in RAM, random
    /// leftovers in the scratch registers so stale values can't match by
    /// accident.
    fn float_setup(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = Rng(0x2545_f491_4f6c_dd1d ^ (seed as u64).wrapping_mul(0x9e37_79b9));
        load_constants(c);
        for f in 0..20 {
            c.f[f] = r.next_u32();
        }
        c.fcr31 = r.next_u32() & ssx3_ee_rt::fpu::FCC;
        for g in [1usize, 2, 3, 6, 7, 8, 9, 10, 11] {
            c.g[g][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
        }
        c.f[12] = interesting(&mut r);
        c.f[13] = interesting(&mut r);
        r
    }

    const SEEDS: u32 = 400;

    fn check_float1(generated: GuestFn, hand: GuestFn) {
        check(generated, hand, SEEDS, &|c, seed| {
            float_setup(c, seed);
        });
    }

    #[test]
    fn math_sin() {
        check_float1(ssx3_ee::s10::f_0031bf60__generated, sin);
    }
    #[test]
    fn math_cos() {
        check_float1(ssx3_ee::s10::f_0031c040__generated, cos);
    }
    #[test]
    fn math_asin() {
        check_float1(ssx3_ee::s10::f_0031c128__generated, asin);
    }
    #[test]
    fn math_atan() {
        check_float1(ssx3_ee::s10::f_0031c228__generated, atan);
    }
    #[test]
    fn math_sin_cos() {
        check(
            ssx3_ee::s10::f_0031be50__generated,
            sin_cos,
            SEEDS,
            &|c, seed| {
                let mut r = float_setup(c, seed);
                c.g[4][0] = DATA as u64;
                // Aliased outputs on some seeds (the store order then matters).
                c.g[5][0] = if r.next_u32() % 8 == 0 {
                    DATA as u64
                } else {
                    DATA as u64 + 0x40
                };
            },
        );
    }
    #[test]
    fn math_fabsf() {
        check_float1(ssx3_ee::s15::f_0040f8a0__generated, fabsf);
    }
    #[test]
    fn math_isnanf() {
        check_float1(ssx3_ee::s15::f_0040f9d0__generated, isnanf);
    }
    #[test]
    fn math_finitef() {
        check_float1(ssx3_ee::s15::f_0040f8c0__generated, finitef);
    }
    #[test]
    fn math_copysignf() {
        check_float1(ssx3_ee::s15::f_0040fb58__generated, copysignf);
    }
    #[test]
    fn math_floorf() {
        check_float1(ssx3_ee::s15::f_0040f8e8__generated, floorf);
    }

    #[test]
    fn math_wrap_angle_pi() {
        check(
            ssx3_ee::s02::f_0016bce0__generated,
            wrap_angle_pi,
            SEEDS,
            &|c, seed| {
                let mut r = float_setup(c, seed);
                c.g[4][0] = DATA as u64;
                let v = interesting(&mut r);
                c.bus.write32(DATA, v);
            },
        );
    }
    #[test]
    fn math_load_indicator_advance() {
        check(
            ssx3_ee::s06::f_00231f80__generated,
            load_indicator_advance,
            SEEDS,
            &|c, seed| {
                let mut r = float_setup(c, seed);
                c.g[4][0] = DATA as u64;
                let v = interesting(&mut r);
                c.bus.write32(DATA, v);
            },
        );
    }
    #[test]
    fn math_noise_sample_1d() {
        check(
            ssx3_ee::s09::f_002d18b0__generated,
            noise_sample_1d,
            SEEDS,
            &|c, seed| {
                let mut r = float_setup(c, seed);
                // The table and the words before it (negative indices).
                for i in -160i32..=161 {
                    let v = r.f32(1.0);
                    c.bus.write32(NOISE_TABLE.0.wrapping_add((i * 4) as u32), v);
                }
                if seed % 4 == 0 {
                    c.f[12] = r.f32(3.0);
                }
            },
        );
    }
    #[test]
    fn math_cubic_spline_eval_segment() {
        check(
            ssx3_ee::s10::f_0031d5a8__generated,
            cubic_spline_eval_segment,
            SEEDS,
            &|c, seed| {
                let mut r = float_setup(c, seed);
                c.g[4][0] = DATA as u64;
                for k in 0..5 {
                    let v = if seed % 3 == 0 {
                        interesting(&mut r)
                    } else {
                        r.f32(1000.0)
                    };
                    c.bus.write32(DATA + 4 * k, v);
                }
            },
        );
    }
    #[test]
    fn math_song_instance_get_beat_ms() {
        check(
            ssx3_ee::s08::f_002b2690__generated,
            song_instance_get_beat_ms,
            SEEDS,
            &|c, seed| {
                let mut r = float_setup(c, seed);
                c.g[4][0] = DATA as u64;
                let info = DATA + 0x100 + (r.next_u32() % 64) * 4;
                c.bus.write32(DATA, info);
                let bpm = if seed % 2 == 0 {
                    r.f32(300.0)
                } else {
                    interesting(&mut r)
                };
                c.bus.write32(info + 0x90, bpm);
            },
        );
    }

    fn rand_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0xdead_beef_0bad_f00d ^ (seed as u64).wrapping_mul(0x9e37_79b9));
        c.g[4][0] = DATA as u64;
        for g in [1usize, 2, 3, 5, 6, 7, 8, 9] {
            c.g[g][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
        }
        for i in 0..6 {
            // Mostly random, often all-ones / zero to hit carries and wraps.
            let w = match r.next_u32() % 4 {
                0 => 0xffff_ffff,
                1 => 0,
                _ => r.next_u32(),
            };
            c.bus.write32(DATA + 4 * i, w);
        }
        if seed % 16 == 0 {
            for i in 0..6 {
                c.bus.write32(DATA + 4 * i, 0xffff_ffff);
            }
        }
        if seed % 16 == 1 {
            // The counter wraps and the ripple reaches s0.
            for i in 1..5 {
                c.bus.write32(DATA + 4 * i, 0);
            }
            c.bus.write32(DATA + 0x14, 0xffff_ffff);
        }
    }
    #[test]
    fn math_rand_next() {
        check(
            ssx3_ee::s10::f_00317a08__generated,
            rand_next,
            SEEDS,
            &rand_setup,
        );
    }
    #[test]
    fn math_rand_seed() {
        check(
            ssx3_ee::s10::f_00317958__generated,
            rand_seed,
            SEEDS,
            &rand_setup,
        );
    }
    #[test]
    fn math_muldi3() {
        check(
            ssx3_ee::s15::f_004114d0__generated,
            muldi3,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x1234_5678_9abc_def0 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                let mut w = || match r.next_u32() % 5 {
                    0 => 0xffff_ffff,
                    1 => 0x8000_0000,
                    2 => 0,
                    _ => r.next_u32(),
                } as u64;
                c.g[4][0] = (w() << 32) | w();
                c.g[5][0] = (w() << 32) | w();
                c.g[3][0] = 0x5555;
            },
        );
    }
}
