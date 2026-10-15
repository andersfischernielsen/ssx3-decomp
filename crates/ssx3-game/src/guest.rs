//! Typed views of guest (EE) memory. Accesses go through the same bus paths
//! as recompiled loads/stores, so RAM, scratchpad and I/O behave identically.

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
}
