//! The interface between the EE core and the hardware/firmware model.
//!
//! [`Cpu`](crate::Cpu) owns one `Box<dyn Platform>` (in [`Bus::plat`]
//! (crate::bus::Bus::plat)). Ordinary RAM and scratchpad accesses never
//! touch it; only memory-mapped I/O, COP2/VU0 and a few status queries go
//! through this vtable. Operations that need the whole machine state
//! (syscalls, scheduled events, fatal errors) are plain function pointers in
//! [`Hooks`](crate::Hooks), since they take `&mut Cpu`.

use crate::vu0::Vu0;
use std::any::Any;

pub trait Platform: Vu0 + Any {
    /// Physical I/O reads (`0x1000_0000..`, kseg I/O aliases): the bus has
    /// already decided the address is not RAM or scratchpad. 8/16-bit reads
    /// are derived from `read32` by the bus.
    fn read32(&mut self, p: u32) -> u32;
    fn read64(&mut self, p: u32) -> u64;
    fn read128(&mut self, p: u32) -> u128;
    /// Physical I/O writes. `ram`/`spr` are passed for DMA kicked by the
    /// write.
    fn write8(&mut self, p: u32, v: u8, ram: &mut [u8], spr: &mut [u8]);
    fn write16(&mut self, p: u32, v: u16, ram: &mut [u8], spr: &mut [u8]);
    fn write32(&mut self, p: u32, v: u32, ram: &mut [u8], spr: &mut [u8]);
    fn write64(&mut self, p: u32, v: u64, ram: &mut [u8], spr: &mut [u8]);
    fn write128(&mut self, p: u32, v: u128, ram: &mut [u8], spr: &mut [u8]);
    /// Tell the hardware the current EE cycle count (before `lw`/`lwu`/`sw`
    /// I/O accesses, as the hardware model expects).
    fn set_now(&mut self, now: u64);
    /// An interrupt source changed: the CPU should service events now.
    fn irq_dirty(&self) -> bool;
    /// Current guest thread id, for diagnostics only.
    fn debug_thread(&self) -> u32 {
        0
    }
}

/// A platform with no hardware: I/O reads 0, writes are ignored, VU0 is a
/// bare register file. For unit tests of the instruction semantics.
#[derive(Default)]
pub struct NullPlatform {
    pub vf: [[u32; 4]; 32],
}

macro_rules! null_vu0 {
    ($($kind:ident $name:ident($($a:ident: $t:ty),*);)*) => {
        impl Vu0 for NullPlatform {
            $(fn $name(&mut self, $($a: $t),*) { $(let _ = $a;)* })*
            fn vnop(&mut self) {}
            fn macro_op(&mut self, _word: u32) {}
            fn vf(&self, r: usize) -> [u32; 4] {
                self.vf[r]
            }
            fn set_vf(&mut self, r: usize, v: [u32; 4]) {
                self.vf[r] = v;
            }
            fn read_ctrl(&mut self, _reg: u32) -> u32 {
                0
            }
            fn write_ctrl(&mut self, _reg: u32, _val: u32) {}
            fn vcallms(&mut self, _addr: u32) {}
        }
    };
}
use crate::vu0::{Efu, Src};
crate::for_each_vu0_op!(null_vu0);

impl Platform for NullPlatform {
    fn read32(&mut self, _p: u32) -> u32 {
        0
    }
    fn read64(&mut self, _p: u32) -> u64 {
        0
    }
    fn read128(&mut self, _p: u32) -> u128 {
        0
    }
    fn write8(&mut self, _p: u32, _v: u8, _ram: &mut [u8], _spr: &mut [u8]) {}
    fn write16(&mut self, _p: u32, _v: u16, _ram: &mut [u8], _spr: &mut [u8]) {}
    fn write32(&mut self, _p: u32, _v: u32, _ram: &mut [u8], _spr: &mut [u8]) {}
    fn write64(&mut self, _p: u32, _v: u64, _ram: &mut [u8], _spr: &mut [u8]) {}
    fn write128(&mut self, _p: u32, _v: u128, _ram: &mut [u8], _spr: &mut [u8]) {}
    fn set_now(&mut self, _now: u64) {}
    fn irq_dirty(&self) -> bool {
        false
    }
}
