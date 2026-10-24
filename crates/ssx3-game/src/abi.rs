//! The MIPS EABI/o32-style calling convention the game uses, for
//! replacements: integer arguments in a0..a3 (then t0..t3), results in v0,
//! return to `ra`.

use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::Cpu;

/// Integer/pointer argument `i` (0-based; a0..a3, t0..t3).
#[inline]
pub fn arg(c: &Cpu, i: usize) -> u32 {
    const R: [usize; 8] = [A0, A1, A2, A3, T0, T1, T2, T3];
    c.g[R[i]][0] as u32
}

/// Set the 32-bit result (sign-extended into the 64-bit register, as
/// compiled code does).
#[inline]
pub fn set_ret(c: &mut Cpu, v: u32) {
    c.g[V0][0] = sx(v);
}

/// Return to the caller (`jr ra`).
#[inline]
pub fn ret(c: &mut Cpu) {
    c.pc = c.g[RA][0] as u32;
}

/// Float argument `i` (0-based; f12, f13, ... as the game's EABI passes them).
#[inline]
pub fn farg(c: &Cpu, i: usize) -> crate::ps2f::FpuF32 {
    crate::ps2f::FpuF32(c.f[12 + i])
}

/// Set the float result (f0).
#[inline]
pub fn set_fret(c: &mut Cpu, v: crate::ps2f::FpuF32) {
    c.f[0] = v.0;
}

/// The full 64-bit value of integer argument `i` (the game sometimes compares
/// whole registers, e.g. against -1, so the upper half matters).
#[inline]
pub fn arg64(c: &Cpu, i: usize) -> u64 {
    const R: [usize; 8] = [A0, A1, A2, A3, T0, T1, T2, T3];
    c.g[R[i]][0]
}

/// Leave a 32-bit value in a GPR as a `lw`/ALU result does (sign-extended,
/// upper 64 bits untouched).
#[inline]
pub fn set_word(c: &mut Cpu, reg: usize, v: u32) {
    c.g[reg][0] = sx(v);
}

/// Leave a full 128-bit value in a GPR (what `lq` leaves behind).
#[inline]
pub fn set_gpr128(c: &mut Cpu, reg: usize, v: u128) {
    c.g[reg] = [v as u64, (v >> 64) as u64];
}

/// The FPU condition bit as a `c.*.s` compare leaves it.
#[inline]
pub fn set_fcc(c: &mut Cpu, cond: bool) {
    if cond {
        c.fcr31 |= ssx3_ee_rt::fpu::FCC;
    } else {
        c.fcr31 &= !ssx3_ee_rt::fpu::FCC;
    }
}

/// `mult` (pipeline 0): signed 32x32 multiply, LO/HI set as the hardware
/// does; returns the low word.
#[inline]
pub fn mult(c: &mut Cpu, a: u32, b: u32) -> u32 {
    let r = (a as i32 as i64) * (b as i32 as i64);
    c.lo[0] = sx(r as u32);
    c.hi[0] = sx((r >> 32) as u32);
    r as u32
}

/// `multu` (pipeline 0); returns the low word.
#[inline]
pub fn multu(c: &mut Cpu, a: u32, b: u32) -> u32 {
    let r = (a as u64) * (b as u64);
    c.lo[0] = sx(r as u32);
    c.hi[0] = sx((r >> 32) as u32);
    r as u32
}

// ---------------------------------------------------------------------------
// Whole registers.

/// The full 64-bit value of a GPR (lane 0), for the 64-bit compares and
/// moves the game does on whole registers.
#[inline]
pub fn reg(c: &Cpu, r: usize) -> u64 {
    c.g[r][0]
}

/// Set the full 64-bit value of a GPR (lane 0; the upper 64 bits are left
/// alone, as non-MMI instructions do). Also for incidental values an
/// original leaves behind (v0/v1 scratch).
#[inline]
pub fn set_reg(c: &mut Cpu, r: usize, v: u64) {
    c.g[r][0] = v;
}

/// The low 32 bits of a GPR (a pointer or a 32-bit count).
#[inline]
pub fn r32(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// A 128-bit GPR as one value (lane 0 in the low bits).
#[inline]
pub fn gpr128(c: &Cpu, r: usize) -> u128 {
    c.g[r][0] as u128 | (c.g[r][1] as u128) << 64
}

/// `addu`: 32-bit add, sign-extended (address arithmetic on registers).
#[inline]
pub fn addu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

// ---------------------------------------------------------------------------
// FPU registers.

/// Set an FPU register (an incidental value the original leaves behind, or
/// a result in a register other than f0).
#[inline]
pub fn set_fpr(c: &mut Cpu, r: usize, v: crate::ps2f::FpuF32) {
    c.f[r] = v.0;
}

/// The bits of a float register as `mfc1` delivers them (sign-extended).
#[inline]
pub fn mfc1(v: crate::ps2f::FpuF32) -> u64 {
    sx(v.0)
}

/// `c.lt.s a, b`: the compare result, also latched in the FCC bit of fcr31
/// (the original leaves it there; later code may branch on a stale value).
#[inline]
pub fn c_lt_s(c: &mut Cpu, a: crate::ps2f::FpuF32, b: crate::ps2f::FpuF32) -> bool {
    let lt = a.lt(b);
    set_fcc(c, lt);
    lt
}

// ---------------------------------------------------------------------------
// Division (pipeline 0), LO/HI as `Cpu::div`/`Cpu::divu` leave them.

/// `div n, d`: signed 32-bit division; LO = quotient, HI = remainder
/// (sign-extended). Hardware results for the special cases: `d == 0` gives
/// quotient `n < 0 ? 1 : -1` and remainder `n`; `MIN / -1` gives `MIN`
/// remainder 0. Returns `(quotient, remainder)`.
#[inline]
pub fn div(c: &mut Cpu, n: u32, d: u32) -> (u32, u32) {
    let (n, d) = (n as i32, d as i32);
    let (q, r) = if d == 0 {
        (if n < 0 { 1 } else { -1 }, n)
    } else if n == i32::MIN && d == -1 {
        (i32::MIN, 0)
    } else {
        (n / d, n % d)
    };
    c.lo[0] = sx(q as u32);
    c.hi[0] = sx(r as u32);
    (q as u32, r as u32)
}

/// `divu n, d`: unsigned 32-bit division; LO = quotient, HI = remainder
/// (sign-extended). `d == 0` gives quotient `0xffffffff` and remainder `n`.
/// Returns `(quotient, remainder)`.
#[inline]
pub fn divu(c: &mut Cpu, n: u32, d: u32) -> (u32, u32) {
    let (q, r) = if d == 0 {
        (u32::MAX, n)
    } else {
        (n / d, n % d)
    };
    c.lo[0] = sx(q);
    c.hi[0] = sx(r);
    (q, r)
}
