//! The PS2 machine behind the EE core's [`Platform`] trait object.
//!
//! Generated EE code is compiled against `ssx3-ee-rt` only. Everything it
//! cannot reach inline (memory-mapped I/O, VU0 in macro mode, syscalls,
//! scheduled events) arrives here: [`Machine`] is the `Box<dyn Platform>` in
//! `c.bus.plat`, and [`HOOKS`] are the `fn(&mut Cpu, ..)` entry points the
//! core calls for operations that need the whole machine. Code in this crate
//! reaches the machine state with [`m!`](crate::m) (`m!(c).kernel`,
//! `m!(c).hw.gs`, ...), which borrows only the `c.bus.plat` field, so it can
//! be combined with `c.g`, `c.bus.ram` and the other `Cpu` fields.
//!
//! See `docs/investigation/crate-split.md`.

use ssx3_ee_rt::vu0::{Efu, Src, Vu0};
use ssx3_ee_rt::{Cpu, Hooks, Platform};

use crate::hw::Hw;
use crate::kernel::Kernel;
use crate::machine::Sched;
use crate::sif::Sif;

/// Everything of the PS2 except the EE core: hardware, kernel HLE, SIF/IOP
/// services, the frame scheduler, and IOP RAM.
pub struct Machine {
    pub hw: Hw,
    pub kernel: Kernel,
    pub sif: Sif,
    pub sched: Sched,
    /// IOP RAM (2 MiB), visible to the EE at 0x1C00_0000.
    pub iop: Vec<u8>,
}

impl Machine {
    pub fn new() -> Self {
        Self { hw: Hw::new(), kernel: Kernel::new(), sif: Sif::new(), sched: Sched::default(), iop: vec![0; 0x20_0000] }
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}

/// The core's hooks into this crate.
pub const HOOKS: Hooks = Hooks {
    event: crate::machine::service_events,
    syscall: crate::kernel::syscall,
    jump_out: crate::machine::jump_out,
    fatal: crate::machine::fatal,
    miss: crate::machine::miss,
};

/// A new EE attached to a fresh PS2 machine.
pub fn new_cpu() -> Box<Cpu> {
    Cpu::new(Box::new(Machine::new()), HOOKS)
}

/// The [`Machine`] behind a CPU's platform box. Panics if the CPU was not
/// created by [`new_cpu`].
#[inline]
pub fn machine_of(p: &mut Box<dyn Platform>) -> &mut Machine {
    let any: &mut dyn std::any::Any = &mut **p;
    any.downcast_mut::<Machine>().expect("Cpu platform is not an ssx3_ps2 Machine")
}

/// Shared-reference form of [`machine_of`].
#[inline]
pub fn machine_ref(p: &Box<dyn Platform>) -> &Machine {
    let any: &dyn std::any::Any = &**p;
    any.downcast_ref::<Machine>().expect("Cpu platform is not an ssx3_ps2 Machine")
}

/// `m!(c)`: `&mut Machine` of `c: &mut Cpu`, borrowing only `c.bus.plat`.
#[macro_export]
macro_rules! m {
    ($c:expr) => {
        $crate::platform::machine_of(&mut $c.bus.plat)
    };
}

/// `mr!(c)`: `&Machine` of `c: &Cpu`.
#[macro_export]
macro_rules! mr {
    ($c:expr) => {
        $crate::platform::machine_ref(&$c.bus.plat)
    };
}

impl Platform for Machine {
    fn read32(&mut self, p: u32) -> u32 {
        self.hw.read32(p)
    }
    fn read64(&mut self, p: u32) -> u64 {
        self.hw.read64(p)
    }
    fn read128(&mut self, p: u32) -> u128 {
        self.hw.read128(p)
    }
    fn write8(&mut self, p: u32, v: u8, ram: &mut [u8], spr: &mut [u8]) {
        self.hw.write8(p, v, ram, spr)
    }
    fn write16(&mut self, p: u32, v: u16, ram: &mut [u8], spr: &mut [u8]) {
        self.hw.write16(p, v, ram, spr)
    }
    fn write32(&mut self, p: u32, v: u32, ram: &mut [u8], spr: &mut [u8]) {
        self.hw.write32(p, v, ram, spr)
    }
    fn write64(&mut self, p: u32, v: u64, ram: &mut [u8], spr: &mut [u8]) {
        self.hw.write64(p, v, ram, spr)
    }
    fn write128(&mut self, p: u32, v: u128, ram: &mut [u8], spr: &mut [u8]) {
        self.hw.write128(p, v, ram, spr)
    }
    fn set_now(&mut self, now: u64) {
        self.hw.now = now;
    }
    fn irq_dirty(&self) -> bool {
        self.hw.irq_dirty
    }
    fn debug_thread(&self) -> u32 {
        self.kernel.current as u32
    }
}

// One macro op = the VU's per-op semantics bracketed by the macro-mode
// pipeline begin/end, exactly what `Vu::macro_op` does after decoding.
macro_rules! vu0_end {
    (upper, $v:expr) => {
        $v.macro_end_upper()
    };
    (lower, $v:expr) => {
        $v.macro_end_lower()
    };
}

macro_rules! impl_vu0 {
    ($($kind:ident $name:ident($($a:ident: $t:ty),*);)*) => {
        impl Vu0 for Machine {
            $(
                fn $name(&mut self, $($a: $t),*) {
                    let v = &mut self.hw.vu0;
                    v.macro_begin();
                    v.$name($($a),*);
                    vu0_end!($kind, v);
                }
            )*
            fn vnop(&mut self) {
                let v = &mut self.hw.vu0;
                v.macro_begin();
                v.macro_end_upper();
            }
            fn macro_op(&mut self, word: u32) {
                self.hw.vu0.macro_op(word);
            }
            fn vf(&self, r: usize) -> [u32; 4] {
                self.hw.vu0.vf[r]
            }
            fn set_vf(&mut self, r: usize, v: [u32; 4]) {
                self.hw.vu0.vf[r] = v;
            }
            fn acc(&self) -> [u32; 4] {
                self.hw.vu0.acc
            }
            fn set_acc(&mut self, v: [u32; 4]) {
                self.hw.vu0.acc = v;
            }
            fn read_ctrl(&mut self, reg: u32) -> u32 {
                self.hw.vu0.read_ctrl(reg)
            }
            fn write_ctrl(&mut self, reg: u32, val: u32) {
                self.hw.vu0.write_ctrl(reg, val);
            }
            fn vcallms(&mut self, addr: u32) {
                let vu = &mut self.hw.vu0;
                vu.start(addr);
                let cap = crate::vu::capture::begin(vu, vu.tpc);
                vu.run(1_000_000, &mut |_| {});
                if let Some(c) = cap {
                    crate::vu::capture::end(c, vu);
                }
            }
        }
    };
}

ssx3_ee_rt::for_each_vu0_op!(impl_vu0);
