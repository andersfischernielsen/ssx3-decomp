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
