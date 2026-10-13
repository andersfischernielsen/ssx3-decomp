//! R5900 architectural state and instruction semantics.
//!
//! This is **not** an interpreter: there is no fetch/decode loop here. The
//! recompiler (`ssx3-recomp`) decodes every instruction at build time and emits
//! Rust that manipulates this state directly (simple ALU ops inline) or calls
//! the semantic helpers below (loads/stores, multiply/divide, MMI, FPU, COP0).
//! Keeping the semantics in one hand-written library keeps the generated code
//! small and lets every instruction be lowered exactly.
//!
//! `Cpu` also owns the rest of the machine (bus, kernel, VUs), so generated
//! functions take a single `&mut Cpu`.

use crate::bus::Bus;
use crate::kernel::Kernel;
use crate::machine::Sched;
use crate::sif::Sif;

/// Sign-extend a 32-bit result to 64 bits (MIPS64 convention for 32-bit ops).
#[inline(always)]
pub fn sx(v: u32) -> u64 {
    v as i32 as i64 as u64
}

/// Bit 0 of `pc` set by the kernel means "unwind to the trampoline".
pub const UNWIND: u32 = 1;

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
    pub bus: Bus,
    pub kernel: Kernel,
    pub sif: Sif,
    pub sched: Sched,
}

const MAX_DEPTH: u32 = 160;

impl Cpu {
    pub fn new() -> Box<Self> {
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
            bus: Bus::new(),
            kernel: Kernel::new(),
            sif: Sif::new(),
            sched: Sched::default(),
        })
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
    pub fn event(&mut self) -> bool {
        crate::machine::service_events(self)
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
            return sx(u32::from_le_bytes(self.bus.ram[o..o + 4].try_into().unwrap()));
        }
        sx(self.bus.read32(a))
    }
    #[inline(always)]
    pub fn lwu(&mut self, a: u32) -> u64 {
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
            self.bus.ram[o..o + 4].copy_from_slice(&(v as u32).to_le_bytes());
            return;
        }
        self.bus.write32(a, v as u32)
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
            for i in 0..4 {
                self.bus.hw.vu0.vf[ft][i] = (v >> (32 * i)) as u32;
            }
        }
    }
    pub fn sqc2(&mut self, ft: usize, a: u32) {
        let r = self.bus.hw.vu0.vf[ft];
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
        let (q, r) = if d == 0 { (u32::MAX, n) } else { (n / d, n % d) };
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
        crate::machine::fatal(self, pc, &format!("break {code:#x}"));
    }
}
