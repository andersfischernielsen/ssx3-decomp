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
