//! The call protocol: how a replacement calls other guest functions.
//!
//! A replacement that calls guest code has to look, from the outside,
//! exactly like the compiled function:
//!
//! - the same stack frame: `sp -= size`, `ra` saved with `sd`, callee-saved
//!   registers saved with `sq` (all 128 bits) at the same offsets ([`Frame`]);
//! - the same registers at the moment of each call: arguments, `ra = return
//!   address`, `pc = callee`, and also the scratch values the original
//!   happens to leave in v0/v1/a2.. (a callee may not touch them, so they can
//!   survive to the caller's caller). Write them in a marked "registers at
//!   the call" block before each call;
//! - the call itself through the generated function path ([`Callee`]: the
//!   shard's `f_X` or its readable alias, which goes through the override
//!   slot, so a replaced callee is reached too) or through the function
//!   table ([`call_ptr`]: `call_indirect`, for `jalr` and for the cross-shard
//!   direct calls the generator emitted as `call_indirect`);
//! - the unwind check (below);
//! - cycles charged block by block at the same points relative to the calls
//!   (a callee sees `c.cycles`);
//! - the epilogue reloading saved registers through the current `sp`.
//!
//! Callee-saved values (`this` in s0 ...) can be kept in Rust locals after a
//! call: every guest function preserves s0-s7 and sp.
//!
//! # Host-thread unwind
//!
//! An event delivered inside a callee (a thread switch, a VBlank that
//! completes a frame) can ask the host thread to unwind back to the
//! trampoline, which later continues the guest thread at its saved `pc`. A
//! callee that unwinds returns with `pc` somewhere other than the return
//! address. The compiled caller then returns at once, leaving every register
//! as it is, and so must a replacement: the calling helpers return [`Flow`]
//! (`Err(Unwound)` when `pc != ret`), bodies propagate it with `?`, and
//! [`guest`] turns such a body into a plain `fn(&mut Cpu)`. Nothing after
//! the call runs in the replacement. (Loops that check events themselves
//! follow the rules in [`crate::events`].)
//!
//! Tests drive these paths with fake callees that unwind at chosen call
//! indices (`difftest::fakes`).

use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

use crate::guest::Addr;

/// A callee did not return to the expected address: the caller must return
/// immediately (host-thread unwind).
#[derive(Debug)]
pub struct Unwound;

/// Result of a body that makes calls.
pub type Flow = Result<(), Unwound>;

/// Run a body written with `?` on calls as a guest function.
#[inline(always)]
pub fn guest(c: &mut Cpu, body: impl FnOnce(&mut Cpu) -> Flow) {
    let _ = body(c);
}

/// A direct-call target: guest address and the generated function
/// (`ssx3_ee::sNN::f_X` or its readable alias, which dispatches through its
/// override slot).
#[derive(Clone, Copy)]
pub struct Callee {
    pub addr: u32,
    pub f: GuestFn,
}

impl Callee {
    /// `jal addr` returning to `ret`.
    #[inline]
    pub fn call(self, c: &mut Cpu, ret: u32) -> Flow {
        c.g[RA][0] = sx(ret);
        c.pc = self.addr;
        (self.f)(c);
        returned(c, ret)
    }
}

/// `jalr target` returning to `ret` (through the function table).
#[inline]
pub fn call_ptr(c: &mut Cpu, target: u32, ret: u32) -> Flow {
    c.g[RA][0] = sx(ret);
    c.pc = target;
    call_indirect(c, target);
    returned(c, ret)
}

/// `Ok` if the callee came back to `ret`.
#[inline(always)]
pub fn returned(c: &Cpu, ret: u32) -> Flow {
    if c.pc == ret {
        Ok(())
    } else {
        Err(Unwound)
    }
}

/// What a one-call wrapper calls.
#[derive(Clone, Copy)]
pub enum Target {
    /// A direct call through the generated function.
    Direct(Callee),
    /// A call through the function table (`call_indirect`).
    Table(u32),
}

/// The shape of gcc's 7-cycle forwarding wrappers: a 0x10 frame holding
/// `ra`, optionally `a0 = *(a0 + off)` (`lw`), one call returning to `ret`,
/// return what the callee returned.
pub fn tail_wrapper(c: &mut Cpu, a0_field: Option<u32>, target: Target, ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        if let Some(off) = a0_field {
            c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(off)).lw(c);
        }
        match target {
            Target::Direct(f) => f.call(c, ret)?,
            Target::Table(addr) => call_ptr(c, addr, ret)?,
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The function's stack frame. Offsets are relative to the current `sp`
/// register, read each time as the compiled code does.
#[derive(Clone, Copy)]
pub struct Frame {
    pub size: u32,
}

impl Frame {
    /// `addiu sp, sp, -size`.
    #[inline]
    pub fn open(c: &mut Cpu, size: u32) -> Frame {
        c.g[SP][0] = sx(sp(c).wrapping_sub(size));
        Frame { size }
    }
    /// The address `off(sp)`.
    #[inline]
    pub fn at(self, c: &Cpu, off: u32) -> u32 {
        sp(c).wrapping_add(off)
    }
    /// `sd ra, off(sp)`.
    #[inline]
    pub fn save_ra(self, c: &mut Cpu, off: u32) {
        let a = self.at(c, off);
        c.sd(a, c.g[RA][0]);
    }
    /// `sq reg, off(sp)` (callee-saved GPR, all 128 bits).
    #[inline]
    pub fn save(self, c: &mut Cpu, reg: usize, off: u32) {
        let a = self.at(c, off);
        c.sq(reg, a);
    }
    /// `lq reg, off(sp)`.
    #[inline]
    pub fn restore(self, c: &mut Cpu, reg: usize, off: u32) {
        let a = self.at(c, off);
        c.lq(reg, a);
    }
    /// `ld ra, off(sp)`.
    #[inline]
    pub fn restore_ra(self, c: &mut Cpu, off: u32) {
        let a = self.at(c, off);
        c.g[RA][0] = c.ld(a);
    }
    /// `addiu sp, sp, size; jr ra`.
    #[inline]
    pub fn close(self, c: &mut Cpu) {
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(self.size));
        c.pc = ra;
    }
}

/// The current stack pointer.
#[inline(always)]
pub fn sp(c: &Cpu) -> u32 {
    c.g[SP][0] as u32
}

/// A gcc 2.9x vtable entry: `{ short delta; short; void (*fn)(); }`. The
/// method is called with `this + delta`.
#[derive(Clone, Copy)]
pub struct VSlot {
    /// `lh vt+off` (sign-extended).
    pub delta: u64,
    /// `lw vt+off+4` (sign-extended).
    pub func: u64,
}

impl VSlot {
    /// Read slot `off` of the vtable at `vt` (`lh`, then `lw`).
    #[inline]
    pub fn read(c: &mut Cpu, vt: u64, off: u32) -> VSlot {
        let base = (vt as u32).wrapping_add(off);
        let delta = Addr(base).lh(c);
        let func = Addr(base.wrapping_add(4)).lw(c);
        VSlot { delta, func }
    }
    /// The adjusted `this` (`addu this, delta`).
    #[inline]
    pub fn this(self, obj: u64) -> u64 {
        crate::abi::addu(obj, self.delta)
    }
}
