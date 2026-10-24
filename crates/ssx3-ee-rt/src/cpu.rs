//! R5900 architectural state and instruction semantics.
//!
//! This is **not** an interpreter: there is no fetch/decode loop here. The
//! recompiler (`ssx3-recomp`) decodes every instruction at build time and emits
//! Rust that manipulates this state directly (simple ALU ops inline) or calls
//! the semantic helpers below (loads/stores, multiply/divide, MMI, FPU, COP0).
//! Keeping the semantics in one hand-written library keeps the generated code
//! small and lets every instruction be lowered exactly.
//!
//! `Cpu` also owns the rest of the machine, as a `Box<dyn Platform>` in
//! [`Bus::plat`] plus the [`Hooks`], so generated functions take a single
//! `&mut Cpu`.

use crate::bus::Bus;
use crate::platform::{NullPlatform, Platform};
use crate::vu0::Vu0;

/// Sign-extend a 32-bit result to 64 bits (MIPS64 convention for 32-bit ops).
#[inline(always)]
pub fn sx(v: u32) -> u64 {
    v as i32 as i64 as u64
}

/// ABI names for GPR indices, used by generated code (`c.g[SP][0]`).
#[allow(missing_docs)]
pub mod reg {
    pub const ZERO: usize = 0;
    pub const AT: usize = 1;
    pub const V0: usize = 2;
    pub const V1: usize = 3;
    pub const A0: usize = 4;
    pub const A1: usize = 5;
    pub const A2: usize = 6;
    pub const A3: usize = 7;
    pub const T0: usize = 8;
    pub const T1: usize = 9;
    pub const T2: usize = 10;
    pub const T3: usize = 11;
    pub const T4: usize = 12;
    pub const T5: usize = 13;
    pub const T6: usize = 14;
    pub const T7: usize = 15;
    pub const S0: usize = 16;
    pub const S1: usize = 17;
    pub const S2: usize = 18;
    pub const S3: usize = 19;
    pub const S4: usize = 20;
    pub const S5: usize = 21;
    pub const S6: usize = 22;
    pub const S7: usize = 23;
    pub const T8: usize = 24;
    pub const T9: usize = 25;
    pub const K0: usize = 26;
    pub const K1: usize = 27;
    pub const GP: usize = 28;
    pub const SP: usize = 29;
    pub const FP: usize = 30;
    pub const RA: usize = 31;
}

/// Bit 0 of `pc` set by the kernel means "unwind to the trampoline".
pub const UNWIND: u32 = 1;

/// Machine operations that need the whole `Cpu` (they run guest code, switch
/// threads, or touch kernel state). Installed by the hardware model.
#[derive(Clone, Copy)]
pub struct Hooks {
    /// Advance hardware time and deliver interrupts (`Cpu::event`). Returns
    /// true when the current thread must unwind.
    pub event: fn(&mut Cpu) -> bool,
    /// `syscall` at `pc` (on entry `c.pc` is the return address).
    pub syscall: fn(&mut Cpu, u32),
    /// A computed jump left a function's labels: `(pc, fn start, fn end)`.
    pub jump_out: fn(&mut Cpu, u32, u32, u32),
    /// Stop the machine with a diagnostic at `pc`.
    pub fatal: fn(&mut Cpu, u32, &str),
    /// No translated code at the target of an indirect call (`pc`).
    pub miss: fn(&mut Cpu, u32),
}

fn no_event(_: &mut Cpu) -> bool {
    false
}
fn no_syscall(_: &mut Cpu, pc: u32) {
    panic!("syscall at {pc:#x} without a platform");
}
fn no_jump_out(_: &mut Cpu, pc: u32, _: u32, _: u32) {
    panic!("jump_out to {pc:#x} without a platform");
}
fn no_fatal(_: &mut Cpu, pc: u32, msg: &str) {
    panic!("{msg} at {pc:#x}");
}
fn no_miss(_: &mut Cpu, pc: u32) {
    panic!("no translated code at {pc:#x}");
}

impl Hooks {
    /// Hooks for a CPU without a machine (unit tests): events are no-ops,
    /// everything else panics.
    pub const DETACHED: Hooks = Hooks {
        event: no_event,
        syscall: no_syscall,
        jump_out: no_jump_out,
        fatal: no_fatal,
        miss: no_miss,
    };
}

pub struct Cpu {
    /// 32 × 128-bit GPRs as `[low64, high64]`. Non-MMI ops touch only `[0]`.
    pub g: [[u64; 2]; 32],
    /// HI/LO: `[0]` is HI/LO, `[1]` is HI1/LO1 (the pipeline-1 pair).
    pub hi: [u64; 2],
    pub lo: [u64; 2],
    /// Shift amount register (in bits) used by QFSRV.
    pub sa: u32,
    /// FPU registers (raw single-precision bits), accumulator, and FCR31.
    pub f: [u32; 32],
    pub facc: u32,
    pub fcr31: u32,
    /// COP0 registers (Status, Cause, EPC, Count, ...).
    pub cop0: [u32; 32],
    /// Address of the next guest instruction to run, as seen by the trampoline.
    pub pc: u32,
    /// Elapsed EE cycles (approximate: one per instruction).
    pub cycles: u64,
    /// When `cycles` reaches this, generated code calls [`Cpu::event`].
    pub next_event: u64,
    /// Host call depth; capped so deep guest recursion unwinds to the trampoline.
    pub depth: u32,
    /// The caller of [`Cpu::event`] cannot unwind (a hand-written loop that
    /// continues after its event check): the machine delivers interrupts
    /// but keeps a thread switch or a finished frame for the next check.
    /// Only the unwind form needs it (host threads block instead).
    pub no_unwind: bool,
    /// RAM, scratchpad, and the hardware model (`bus.plat`).
    pub bus: Bus,
    pub hooks: Hooks,
}

const MAX_DEPTH: u32 = 160;

impl Cpu {
    /// A CPU attached to `plat`, with `hooks` for syscalls and events.
    pub fn new(plat: Box<dyn Platform>, hooks: Hooks) -> Box<Self> {
        Box::new(Self {
            g: [[0; 2]; 32],
            hi: [0; 2],
            lo: [0; 2],
            sa: 0,
            f: [0; 32],
            facc: 0,
            fcr31: 0,
            cop0: [0; 32],
            pc: 0,
            cycles: 0,
            next_event: 0,
            depth: 0,
            no_unwind: false,
            bus: Bus::new(plat),
            hooks,
        })
    }

    /// A CPU with no hardware ([`NullPlatform`], [`Hooks::DETACHED`]), for
    /// unit tests of instruction semantics.
    pub fn detached() -> Box<Self> {
        Self::new(Box::new(NullPlatform::default()), Hooks::DETACHED)
    }

    /// VU0 (COP2 macro mode) on the hardware model.
    #[inline(always)]
    pub fn vu0(&mut self) -> &mut dyn Vu0 {
        &mut *self.bus.plat
    }

    /// `syscall` at `pc` (generated code sets `c.pc` to the return address first).
    #[inline(always)]
    pub fn syscall(&mut self, pc: u32) {
        (self.hooks.syscall)(self, pc)
    }

    /// Stop the machine with a diagnostic.
    #[inline(never)]
    pub fn fatal(&mut self, pc: u32, msg: &str) {
        (self.hooks.fatal)(self, pc, msg)
    }

    /// A computed jump (`jr`) left the current function's labels.
    #[inline(never)]
    pub fn jump_out(&mut self, pc: u32, start: u32, end: u32) {
        (self.hooks.jump_out)(self, pc, start, end)
    }

    // ---------------------------------------------------------------- calls

    /// Called by generated code before a direct call. Returns false if the
    /// host stack is too deep; the caller then unwinds and lets the trampoline
    /// make the call at depth 0.
    #[inline(always)]
    pub fn enter(&mut self, target: u32) -> bool {
        self.pc = target;
        if self.depth >= MAX_DEPTH {
            return false;
        }
        self.depth += 1;
        true
    }

    #[inline(always)]
    pub fn leave(&mut self) {
        self.depth -= 1;
    }

    /// Generated code calls this when `cycles >= next_event` (at backward
    /// branches and calls). Returns true if the current thread must unwind.
    #[inline(always)]
    pub fn event(&mut self) -> bool {
        (self.hooks.event)(self)
    }

    // ------------------------------------------------------------- memory

    #[inline(always)]
    pub fn lb(&mut self, a: u32) -> u64 {
        self.bus.read8(a) as i8 as i64 as u64
    }
    #[inline(always)]
    pub fn lbu(&mut self, a: u32) -> u64 {
        self.bus.read8(a) as u64
    }
    #[inline(always)]
    pub fn lh(&mut self, a: u32) -> u64 {
        self.bus.read16(a) as i16 as i64 as u64
    }
    #[inline(always)]
    pub fn lhu(&mut self, a: u32) -> u64 {
        self.bus.read16(a) as u64
    }
    #[inline(always)]
    pub fn lw(&mut self, a: u32) -> u64 {
        if a < 0x0200_0000 {
            let o = (a & !3) as usize;
            return sx(u32::from_le_bytes(
                self.bus.ram[o..o + 4].try_into().unwrap(),
            ));
        }
        self.bus.plat.set_now(self.cycles);
        sx(self.bus.read32(a))
    }
    #[inline(always)]
    pub fn lwu(&mut self, a: u32) -> u64 {
        self.bus.plat.set_now(self.cycles);
        self.bus.read32(a) as u64
    }
    #[inline(always)]
    pub fn ld(&mut self, a: u32) -> u64 {
        self.bus.read64(a)
    }
    #[inline(always)]
    pub fn lq(&mut self, rt: usize, a: u32) {
        let v = self.bus.read128(a);
        if rt != 0 {
            self.g[rt] = [v as u64, (v >> 64) as u64];
        }
    }
    #[inline(always)]
    pub fn sb(&mut self, a: u32, v: u64) {
        self.bus.write8(a, v as u8)
    }
    #[inline(always)]
    pub fn sh(&mut self, a: u32, v: u64) {
        self.bus.write16(a, v as u16)
    }
    #[inline(always)]
    pub fn sw(&mut self, a: u32, v: u64) {
        if a < 0x0200_0000 {
            let o = (a & !3) as usize;
            self.bus.mark(o);
            self.bus.ram[o..o + 4].copy_from_slice(&(v as u32).to_le_bytes());
            return;
        }
        self.bus.plat.set_now(self.cycles);
        self.bus.write32(a, v as u32);
        if self.bus.plat.irq_dirty() {
            self.next_event = self.cycles;
        }
    }
    #[inline(always)]
    pub fn sd(&mut self, a: u32, v: u64) {
        self.bus.write64(a, v)
    }
    #[inline(always)]
    pub fn sq(&mut self, rt: usize, a: u32) {
        let v = (self.g[rt][0] as u128) | ((self.g[rt][1] as u128) << 64);
        self.bus.write128(a, v)
    }

    pub fn lwl(&mut self, rt: usize, a: u32) {
        const MASK: [u32; 4] = [0x00ff_ffff, 0x0000_ffff, 0x0000_00ff, 0];
        const SHIFT: [u32; 4] = [24, 16, 8, 0];
        let s = (a & 3) as usize;
        let mem = self.bus.read32(a & !3);
        let v = (self.g[rt][0] as u32 & MASK[s]) | (mem << SHIFT[s]);
        if rt != 0 {
            self.g[rt][0] = sx(v);
        }
    }
    pub fn lwr(&mut self, rt: usize, a: u32) {
        const MASK: [u32; 4] = [0, 0xff00_0000, 0xffff_0000, 0xffff_ff00];
        const SHIFT: [u32; 4] = [0, 8, 16, 24];
        let s = (a & 3) as usize;
        let mem = self.bus.read32(a & !3);
        if rt == 0 {
            return;
        }
        if s == 0 {
            self.g[rt][0] = sx(mem);
        } else {
            let v = (self.g[rt][0] as u32 & MASK[s]) | (mem >> SHIFT[s]);
            self.g[rt][0] = (self.g[rt][0] & 0xffff_ffff_0000_0000) | v as u64;
        }
    }
    pub fn ldl(&mut self, rt: usize, a: u32) {
        let s = (a & 7) as u32;
        let mem = self.bus.read64(a & !7);
        let mask = if s == 7 { 0 } else { u64::MAX >> ((s + 1) * 8) };
        let shift = 56 - s * 8;
        if rt != 0 {
            self.g[rt][0] = (self.g[rt][0] & mask) | (mem << shift);
        }
    }
    pub fn ldr(&mut self, rt: usize, a: u32) {
        let s = (a & 7) as u32;
        let mem = self.bus.read64(a & !7);
        let mask = if s == 0 { 0 } else { u64::MAX << ((8 - s) * 8) };
        if rt != 0 {
            self.g[rt][0] = (self.g[rt][0] & mask) | (mem >> (s * 8));
        }
    }
    pub fn swl(&mut self, rt: usize, a: u32) {
        const MASK: [u32; 4] = [0xffff_ff00, 0xffff_0000, 0xff00_0000, 0];
        const SHIFT: [u32; 4] = [24, 16, 8, 0];
        let s = (a & 3) as usize;
        let mem = self.bus.read32(a & !3);
        let v = (mem & MASK[s]) | ((self.g[rt][0] as u32) >> SHIFT[s]);
        self.bus.write32(a & !3, v);
    }
    pub fn swr(&mut self, rt: usize, a: u32) {
        const MASK: [u32; 4] = [0, 0x0000_00ff, 0x0000_ffff, 0x00ff_ffff];
        const SHIFT: [u32; 4] = [0, 8, 16, 24];
        let s = (a & 3) as usize;
        let mem = self.bus.read32(a & !3);
        let v = (mem & MASK[s]) | ((self.g[rt][0] as u32) << SHIFT[s]);
        self.bus.write32(a & !3, v);
    }
    pub fn sdl(&mut self, rt: usize, a: u32) {
        let s = (a & 7) as u32;
        let mem = self.bus.read64(a & !7);
        let mask = if s == 7 { 0 } else { u64::MAX << ((s + 1) * 8) };
        let v = (mem & mask) | (self.g[rt][0] >> (56 - s * 8));
        self.bus.write64(a & !7, v);
    }
    pub fn sdr(&mut self, rt: usize, a: u32) {
        let s = (a & 7) as u32;
        let mem = self.bus.read64(a & !7);
        let mask = if s == 0 { 0 } else { u64::MAX >> ((8 - s) * 8) };
        let v = (mem & mask) | (self.g[rt][0] << (s * 8));
        self.bus.write64(a & !7, v);
    }
    pub fn lwc1(&mut self, ft: usize, a: u32) {
        self.f[ft] = self.bus.read32(a);
    }
    pub fn swc1(&mut self, ft: usize, a: u32) {
        let v = self.f[ft];
        self.bus.write32(a, v);
    }
    pub fn lqc2(&mut self, ft: usize, a: u32) {
        let v = self.bus.read128(a);
        if ft != 0 {
            let r = [
                v as u32,
                (v >> 32) as u32,
                (v >> 64) as u32,
                (v >> 96) as u32,
            ];
            self.bus.plat.set_vf(ft, r);
        }
    }
    pub fn sqc2(&mut self, ft: usize, a: u32) {
        let r = self.bus.plat.vf(ft);
        let v = (r[0] as u128) | (r[1] as u128) << 32 | (r[2] as u128) << 64 | (r[3] as u128) << 96;
        self.bus.write128(a, v);
    }

    // ------------------------------------------------------ multiply/divide

    #[inline]
    fn set_rd(&mut self, rd: usize, v: u64) {
        if rd != 0 {
            self.g[rd][0] = v;
        }
    }

    pub fn mult(&mut self, p: usize, rd: usize, rs: usize, rt: usize) {
        let r = (self.g[rs][0] as i32 as i64) * (self.g[rt][0] as i32 as i64);
        self.lo[p] = sx(r as u32);
        self.hi[p] = sx((r >> 32) as u32);
        self.set_rd(rd, self.lo[p]);
    }
    pub fn multu(&mut self, p: usize, rd: usize, rs: usize, rt: usize) {
        let r = (self.g[rs][0] as u32 as u64) * (self.g[rt][0] as u32 as u64);
        self.lo[p] = sx(r as u32);
        self.hi[p] = sx((r >> 32) as u32);
        self.set_rd(rd, self.lo[p]);
    }
    pub fn div(&mut self, p: usize, rs: usize, rt: usize) {
        let n = self.g[rs][0] as i32;
        let d = self.g[rt][0] as i32;
        let (q, r) = if d == 0 {
            (if n < 0 { 1 } else { -1 }, n)
        } else if n == i32::MIN && d == -1 {
            (i32::MIN, 0)
        } else {
            (n / d, n % d)
        };
        self.lo[p] = sx(q as u32);
        self.hi[p] = sx(r as u32);
    }
    pub fn divu(&mut self, p: usize, rs: usize, rt: usize) {
        let n = self.g[rs][0] as u32;
        let d = self.g[rt][0] as u32;
        let (q, r) = if d == 0 {
            (u32::MAX, n)
        } else {
            (n / d, n % d)
        };
        self.lo[p] = sx(q);
        self.hi[p] = sx(r);
    }
    pub fn madd(&mut self, p: usize, rd: usize, rs: usize, rt: usize) {
        let acc = ((self.hi[p] as u32 as u64) << 32 | self.lo[p] as u32 as u64) as i64;
        let r = acc.wrapping_add((self.g[rs][0] as i32 as i64) * (self.g[rt][0] as i32 as i64));
        self.lo[p] = sx(r as u32);
        self.hi[p] = sx((r >> 32) as u32);
        self.set_rd(rd, self.lo[p]);
    }
    pub fn maddu(&mut self, p: usize, rd: usize, rs: usize, rt: usize) {
        let acc = (self.hi[p] as u32 as u64) << 32 | self.lo[p] as u32 as u64;
        let r = acc.wrapping_add((self.g[rs][0] as u32 as u64) * (self.g[rt][0] as u32 as u64));
        self.lo[p] = sx(r as u32);
        self.hi[p] = sx((r >> 32) as u32);
        self.set_rd(rd, self.lo[p]);
    }

    // ------------------------------------------------------------- COP0

    pub fn mfc0(&mut self, rt: usize, rd: usize) {
        let v = match rd {
            9 => self.cycles as u32,
            15 => 0x2e20,
            _ => self.cop0[rd],
        };
        self.set_rd(rt, sx(v));
    }
    pub fn mtc0(&mut self, rt: usize, rd: usize) {
        self.cop0[rd] = self.g[rt][0] as u32;
    }

    // ------------------------------------------------------------- traps

    /// `break` / unhandled trap: log and stop the machine.
    pub fn brk(&mut self, pc: u32, code: u32) {
        self.fatal(pc, &format!("break {code:#x}"));
    }
}
