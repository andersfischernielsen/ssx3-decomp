//! Typed views of guest (EE) memory.
//!
//! Two families of accessors: `read_*`/`write_*` go straight to the bus (fine
//! for RAM and scratchpad structs); `lb`..`sd` go through the same `Cpu`
//! load/store paths as recompiled code, including the I/O timing side effects
//! (use these when an address may be I/O, or to get the sign-extended 64-bit
//! register value a load leaves).

use crate::ps2f::{Vec4, VuF32};
use ssx3_ee_rt::Cpu;

/// A guest address.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Addr(pub u32);

impl Addr {
    #[inline]
    pub fn off(self, o: u32) -> Addr {
        Addr(self.0.wrapping_add(o))
    }
    #[inline]
    pub fn read_u8(self, c: &mut Cpu) -> u8 {
        c.bus.read8(self.0)
    }
    #[inline]
    pub fn write_u8(self, c: &mut Cpu, v: u8) {
        c.bus.write8(self.0, v)
    }
    #[inline]
    pub fn read_u16(self, c: &mut Cpu) -> u16 {
        c.bus.read16(self.0)
    }
    #[inline]
    pub fn write_u16(self, c: &mut Cpu, v: u16) {
        c.bus.write16(self.0, v)
    }
    #[inline]
    pub fn read_u64(self, c: &mut Cpu) -> u64 {
        c.bus.read64(self.0)
    }
    #[inline]
    pub fn write_u64(self, c: &mut Cpu, v: u64) {
        c.bus.write64(self.0, v)
    }
    /// An FPU float (`lwc1`).
    #[inline]
    pub fn read_fpu(self, c: &mut Cpu) -> crate::ps2f::FpuF32 {
        crate::ps2f::FpuF32(self.read_u32(c))
    }
    /// `swc1`.
    #[inline]
    pub fn write_fpu(self, c: &mut Cpu, v: crate::ps2f::FpuF32) {
        self.write_u32(c, v.0)
    }
    #[inline]
    pub fn read_u32(self, c: &mut Cpu) -> u32 {
        c.bus.read32(self.0)
    }
    #[inline]
    pub fn write_u32(self, c: &mut Cpu, v: u32) {
        c.bus.write32(self.0, v)
    }
    #[inline]
    pub fn read_f32(self, c: &mut Cpu) -> VuF32 {
        VuF32(self.read_u32(c))
    }
    /// A quadword (`lq`/`lqc2`: the low 4 address bits are ignored).
    #[inline]
    pub fn read_vec4(self, c: &mut Cpu) -> Vec4 {
        Vec4::from_bits(c.bus.read128(self.0))
    }
    #[inline]
    pub fn write_vec4(self, c: &mut Cpu, v: Vec4) {
        c.bus.write128(self.0, v.to_bits())
    }

    // ------------------------------------------------ CPU load/store paths

    /// `lb` (sign-extended to 64 bits).
    #[inline]
    pub fn lb(self, c: &mut Cpu) -> u64 {
        c.lb(self.0)
    }
    /// `lbu`.
    #[inline]
    pub fn lbu(self, c: &mut Cpu) -> u64 {
        c.lbu(self.0)
    }
    /// `lh` (sign-extended).
    #[inline]
    pub fn lh(self, c: &mut Cpu) -> u64 {
        c.lh(self.0)
    }
    /// `lhu`.
    #[inline]
    pub fn lhu(self, c: &mut Cpu) -> u64 {
        c.lhu(self.0)
    }
    /// `lw` (sign-extended).
    #[inline]
    pub fn lw(self, c: &mut Cpu) -> u64 {
        c.lw(self.0)
    }
    /// `lwu`.
    #[inline]
    pub fn lwu(self, c: &mut Cpu) -> u64 {
        c.lwu(self.0)
    }
    /// `ld`.
    #[inline]
    pub fn ld(self, c: &mut Cpu) -> u64 {
        c.ld(self.0)
    }
    /// `sb` (low byte of `v`).
    #[inline]
    pub fn sb(self, c: &mut Cpu, v: u64) {
        c.sb(self.0, v)
    }
    /// `sh`.
    #[inline]
    pub fn sh(self, c: &mut Cpu, v: u64) {
        c.sh(self.0, v)
    }
    /// `sw`.
    #[inline]
    pub fn sw(self, c: &mut Cpu, v: u64) {
        c.sw(self.0, v)
    }
    /// `sd`.
    #[inline]
    pub fn sd(self, c: &mut Cpu, v: u64) {
        c.sd(self.0, v)
    }

    // ------------------------------------------------ unaligned doublewords

    /// An unaligned doubleword load as the `ldl a+7; ldr a` pair performs it
    /// (straight to the bus).
    pub fn read_u64_unaligned(self, c: &mut Cpu) -> u64 {
        let a = self.0;
        let hi = c.bus.read64(a.wrapping_add(7) & !7);
        let lo = c.bus.read64(a & !7);
        let s = a & 7;
        if s == 0 {
            lo
        } else {
            (lo >> (s * 8)) | (hi << ((8 - s) * 8))
        }
    }

    /// An unaligned doubleword store as the `sdl a+7; sdr a` pair performs
    /// it (two read-modify-writes of the aligned doublewords, high one
    /// first; straight to the bus).
    pub fn write_u64_unaligned(self, c: &mut Cpu, v: u64) {
        let a = self.0;
        let hi_at = a.wrapping_add(7);
        let s = hi_at & 7;
        let mem = c.bus.read64(hi_at & !7);
        let keep = if s == 7 { 0 } else { u64::MAX << ((s + 1) * 8) };
        c.bus
            .write64(hi_at & !7, (mem & keep) | (v >> (56 - s * 8)));
        let s = a & 7;
        let mem = c.bus.read64(a & !7);
        let keep = if s == 0 { 0 } else { u64::MAX >> ((8 - s) * 8) };
        c.bus.write64(a & !7, (mem & keep) | (v << (s * 8)));
    }
}
