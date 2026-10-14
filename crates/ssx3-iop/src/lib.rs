//! # ssx3-iop
//!
//! The PlayStation 2 I/O processor for the SSX 3 recompilation.
//!
//! The game's sound modules (`LIBSD.IRX`, Sony's sound library, and
//! `SNDDRV.IRX`, EA's driver) are translated from R3000A machine code to Rust
//! at build time by `recomp-iop` (`ssx3-recomp/src/iopgen.rs`) into
//! `generated/iop/`. That code runs against [`Iop`]: 32-bit registers, 2 MiB of
//! IOP RAM, and the native firmware in this crate:
//!
//! * [`kernel`]: threads, semaphores, event flags, alarms, memory, interrupts;
//! * [`imports`]: the IOP library entry points the modules import (sysmem,
//!   intrman, thbase, thevent, sifcmd, sifman, sysclib, ...);
//! * [`sif`]: the IOP end of SIF (command handlers, RPC servers);
//! * [`hw`]: INTC and DMA controller; [`spu2`]: the sound processor;
//! * [`audio`]: host output (cpal) and WAV capture.
//!
//! Execution model (same as the EE side, see `docs/investigation/recompiler-v2.md`):
//! every function is a block dispatcher over its labels; a thread switch sets
//! [`UNWIND`] in `pc`, every host frame returns, and [`trampoline`] re-enters
//! the new thread at its saved `pc`. The host (`ssx3-ps2`) calls
//! [`Iop::run`] with the IOP time that corresponds to the EE clock.

pub mod audio;
pub mod ds2;
pub mod hw;
pub mod imports;
pub mod irx;
pub mod kernel;
pub mod memcard;
pub mod sif;
pub mod sio2;
pub mod spu2;

use std::collections::BTreeMap;

/// Bit 0 of `pc`: unwind host frames back to the trampoline.
pub const UNWIND: u32 = 1;
/// Return address of native → guest calls (interrupt handlers, callbacks).
pub const RETURN_MAGIC: u32 = 0xFFFF_FF00;
/// Return address of thread entry functions.
pub const THREAD_EXIT: u32 = 0xFFFF_FF10;
/// Return address of RPC server functions run by `sceSifExecRequest`.
pub const EXEC_FINISH: u32 = 0xFFFF_FF20;
/// Resume point of a thread inside `sceSifRpcLoop` (after a request, or
/// when woken while waiting for one).
pub const RPC_LOOP: u32 = 0xFFFF_FF30;
/// Length of the vertical blanking interval in IOP cycles (NTSC: ~22 lines).
pub const VBLANK_CYCLES: u64 = 52_000;

pub const RAM_SIZE: usize = 0x20_0000;
/// IOP clock: EE / 8 = 36.864 MHz.
pub const CLOCK: u64 = 36_864_000;
const MAX_DEPTH: u32 = 160;
/// Longest a thread runs before the scheduler looks at events again.
const SLICE: u64 = 4096;

pub type GuestFn = fn(&mut Iop);

#[derive(Clone, Copy)]
pub struct FnEntry {
    pub start: u32,
    pub end: u32,
    pub f: GuestFn,
}

/// What an import stub address resolves to when reached indirectly.
#[derive(Clone, Copy, Debug)]
pub enum StubKind {
    /// Native library function: `(library id, index)`.
    Native(u16, u16),
    /// Linked to another recompiled module's export.
    Link(u32),
}

/// A module the generator translated: the runtime checks the disc image
/// against `text_hash` before running the generated code.
#[derive(Clone, Copy, Debug)]
pub struct ModuleInfo {
    pub name: &'static str,
    pub base: u32,
    pub text_hash: u64,
}

pub struct Generated {
    pub functions: &'static [FnEntry],
    pub stubs: &'static [(u32, StubKind)],
    pub modules: &'static [ModuleInfo],
}

static mut GENERATED: Option<&'static Generated> = None;

/// Register the generated code (called once at startup by the game crate).
pub fn register_generated(g: &'static Generated) {
    // SAFETY: written once before any IOP code runs, single-threaded.
    unsafe { GENERATED = Some(g) };
}

pub fn generated() -> Option<&'static Generated> {
    // SAFETY: written once at startup.
    unsafe { GENERATED }
}

pub fn lookup(pc: u32) -> Option<GuestFn> {
    let t = generated()?.functions;
    let i = t.partition_point(|e| e.start <= pc);
    if i == 0 {
        return None;
    }
    let e = &t[i - 1];
    (pc < e.end).then_some(e.f)
}

fn stub_at(pc: u32) -> Option<StubKind> {
    let s = generated()?.stubs;
    s.binary_search_by_key(&pc, |e| e.0).ok().map(|i| s[i].1)
}

/// Bytes the IOP wants written into EE memory, and SIF packets for the EE,
/// collected during a run and applied by the host afterwards.
#[derive(Default)]
pub struct Outbox {
    pub ee_writes: Vec<(u32, Vec<u8>)>,
    pub packets: Vec<Vec<u8>>,
}

pub struct Iop {
    pub r: [u32; 32],
    pub hi: u32,
    pub lo: u32,
    pub pc: u32,
    pub cop0: [u32; 16],
    pub cycles: u64,
    pub next_event: u64,
    pub depth: u32,
    pub ram: Vec<u8>,
    pub scratch: Vec<u8>,
    pub k: kernel::Kernel,
    pub hw: hw::Hw,
    pub spu: spu2::Spu2,
    pub sio2: sio2::Sio2,
    pub sif: sif::IopSif,
    pub out: Outbox,
    pub halted: Option<String>,
    /// Set by the scheduler hooks: the top-level trampoline returns to [`Iop::run`].
    pub yield_now: bool,
    /// Nesting depth of native → guest calls (interrupt context when > 0).
    pub in_handler: u32,
    handler_deadline: u64,
    pub trace: bool,
    /// Unknown I/O addresses touched (address → count).
    pub unknown_io: BTreeMap<u32, u64>,
    io_regs: BTreeMap<u32, u32>,
    /// Per-function-start call statistics are not kept; instructions retired.
    pub instructions: u64,
    /// VBlank starts signalled by the host (EE VBlank) and not yet delivered.
    pub vblank_pending: u32,
    pub vblanks: u64,
    /// EE main RAM, readable while the IOP runs (`sceSifGetOtherData`); set by
    /// the host around [`Iop::run`], null otherwise.
    pub ee_ram: *const u8,
    pub ee_ram_len: usize,
}

// SAFETY: `ee_ram` is only dereferenced while the host lends EE RAM to a
// running IOP (single-threaded, see `ssx3-ps2/src/iop_bridge.rs`).
unsafe impl Send for Iop {}

impl Iop {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            r: [0; 32],
            hi: 0,
            lo: 0,
            pc: 0,
            cop0: [0; 16],
            cycles: 0,
            next_event: 0,
            depth: 0,
            ram: vec![0; RAM_SIZE],
            scratch: vec![0; 0x400],
            k: kernel::Kernel::new(),
            hw: hw::Hw::default(),
            spu: spu2::Spu2::new(),
            sio2: sio2::Sio2::new(),
            sif: sif::IopSif::default(),
            out: Outbox::default(),
            halted: None,
            yield_now: false,
            in_handler: 0,
            handler_deadline: 0,
            trace: std::env::var("SSX3_TRACE_IOP").is_ok(),
            unknown_io: BTreeMap::new(),
            io_regs: BTreeMap::new(),
            instructions: 0,
            vblank_pending: 0,
            vblanks: 0,
            ee_ram: std::ptr::null(),
            ee_ram_len: 0,
        })
    }

    /// Copy `n` bytes of EE RAM (zeros if the host did not provide it).
    pub fn ee_read(&self, a: u32, n: usize) -> Vec<u8> {
        let mut v = vec![0u8; n];
        if self.ee_ram.is_null() {
            return v;
        }
        let o = (a & 0x01FF_FFFF) as usize;
        let end = (o + n).min(self.ee_ram_len);
        if o < end {
            // SAFETY: the host keeps `ee_ram` valid while the IOP runs.
            let src = unsafe { std::slice::from_raw_parts(self.ee_ram.add(o), end - o) };
            v[..end - o].copy_from_slice(src);
        }
        v
    }

    // ------------------------------------------------------------ calls

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

    /// Generated code calls this at backward branches when
    /// `cycles >= next_event`. Returns true if the frame must unwind.
    pub fn event(&mut self) -> bool {
        if self.in_handler > 0 {
            if self.cycles > self.handler_deadline {
                let pc = self.pc;
                self.fatal(pc, "interrupt handler / callback does not return");
                return true;
            }
            self.next_event = self.cycles + 100_000;
            return false;
        }
        self.yield_now = true;
        self.pc |= UNWIND;
        true
    }

    /// Stop the IOP with a diagnostic (the EE keeps running).
    pub fn fatal(&mut self, pc: u32, msg: &str) {
        if self.halted.is_none() {
            eprintln!(
                "[iop] fatal: {msg} at {pc:#010x} (ra {:#010x}, thread {}, cycles {})",
                self.r[31], self.k.current, self.cycles
            );
            self.halted = Some(format!("{msg} at {pc:#010x}"));
        }
        self.yield_now = true;
        self.pc = RETURN_MAGIC | UNWIND;
    }

    // ------------------------------------------------------------ memory

    #[inline(always)]
    fn ram_off(a: u32) -> Option<usize> {
        let p = a & 0x1FFF_FFFF;
        (p < 0x0080_0000).then_some((p & 0x1F_FFFF) as usize)
    }

    #[inline(always)]
    pub fn lw(&mut self, a: u32) -> u32 {
        match Self::ram_off(a) {
            Some(o) => {
                let o = o & !3;
                u32::from_le_bytes(self.ram[o..o + 4].try_into().unwrap())
            }
            None => self.io_read(a & 0x1FFF_FFFC, 4),
        }
    }
    #[inline(always)]
    pub fn lhu(&mut self, a: u32) -> u32 {
        match Self::ram_off(a) {
            Some(o) => {
                let o = o & !1;
                u16::from_le_bytes([self.ram[o], self.ram[o + 1]]) as u32
            }
            None => self.io_read(a & 0x1FFF_FFFE, 2) & 0xFFFF,
        }
    }
    #[inline(always)]
    pub fn lh(&mut self, a: u32) -> u32 {
        self.lhu(a) as u16 as i16 as i32 as u32
    }
    #[inline(always)]
    pub fn lbu(&mut self, a: u32) -> u32 {
        match Self::ram_off(a) {
            Some(o) => self.ram[o] as u32,
            None => self.io_read(a & 0x1FFF_FFFF, 1) & 0xFF,
        }
    }
    #[inline(always)]
    pub fn lb(&mut self, a: u32) -> u32 {
        self.lbu(a) as u8 as i8 as i32 as u32
    }
    #[inline(always)]
    pub fn sw(&mut self, a: u32, v: u32) {
        match Self::ram_off(a) {
            Some(o) => {
                let o = o & !3;
                self.ram[o..o + 4].copy_from_slice(&v.to_le_bytes())
            }
            None => self.io_write(a & 0x1FFF_FFFC, v, 4),
        }
    }
    #[inline(always)]
    pub fn sh(&mut self, a: u32, v: u32) {
        match Self::ram_off(a) {
            Some(o) => {
                let o = o & !1;
                self.ram[o..o + 2].copy_from_slice(&(v as u16).to_le_bytes())
            }
            None => self.io_write(a & 0x1FFF_FFFE, v & 0xFFFF, 2),
        }
    }
    #[inline(always)]
    pub fn sb(&mut self, a: u32, v: u32) {
        match Self::ram_off(a) {
            Some(o) => self.ram[o] = v as u8,
            None => self.io_write(a & 0x1FFF_FFFF, v & 0xFF, 1),
        }
    }
    pub fn lwl(&mut self, rt: usize, a: u32) {
        const MASK: [u32; 4] = [0x00FF_FFFF, 0x0000_FFFF, 0x0000_00FF, 0];
        const SHIFT: [u32; 4] = [24, 16, 8, 0];
        let s = (a & 3) as usize;
        let m = self.lw(a & !3);
        if rt != 0 {
            self.r[rt] = (self.r[rt] & MASK[s]) | (m << SHIFT[s]);
        }
    }
    pub fn lwr(&mut self, rt: usize, a: u32) {
        const MASK: [u32; 4] = [0, 0xFF00_0000, 0xFFFF_0000, 0xFFFF_FF00];
        const SHIFT: [u32; 4] = [0, 8, 16, 24];
        let s = (a & 3) as usize;
        let m = self.lw(a & !3);
        if rt != 0 {
            self.r[rt] = (self.r[rt] & MASK[s]) | (m >> SHIFT[s]);
        }
    }
    pub fn swl(&mut self, rt: usize, a: u32) {
        const MASK: [u32; 4] = [0xFFFF_FF00, 0xFFFF_0000, 0xFF00_0000, 0];
        const SHIFT: [u32; 4] = [24, 16, 8, 0];
        let s = (a & 3) as usize;
        let m = self.lw(a & !3);
        let v = (m & MASK[s]) | (self.r[rt] >> SHIFT[s]);
        self.sw(a & !3, v);
    }
    pub fn swr(&mut self, rt: usize, a: u32) {
        const MASK: [u32; 4] = [0, 0x0000_00FF, 0x0000_FFFF, 0x00FF_FFFF];
        const SHIFT: [u32; 4] = [0, 8, 16, 24];
        let s = (a & 3) as usize;
        let m = self.lw(a & !3);
        let v = (m & MASK[s]) | (self.r[rt] << SHIFT[s]);
        self.sw(a & !3, v);
    }

    // multiply / divide (HI/LO)
    pub fn mult(&mut self, rs: usize, rt: usize) {
        let p = (self.r[rs] as i32 as i64) * (self.r[rt] as i32 as i64);
        self.lo = p as u32;
        self.hi = (p >> 32) as u32;
    }
    pub fn multu(&mut self, rs: usize, rt: usize) {
        let p = (self.r[rs] as u64) * (self.r[rt] as u64);
        self.lo = p as u32;
        self.hi = (p >> 32) as u32;
    }
    pub fn div(&mut self, rs: usize, rt: usize) {
        let (n, d) = (self.r[rs] as i32, self.r[rt] as i32);
        if d == 0 {
            self.hi = n as u32;
            self.lo = if n >= 0 { 0xFFFF_FFFF } else { 1 };
        } else if n == i32::MIN && d == -1 {
            self.lo = n as u32;
            self.hi = 0;
        } else {
            self.lo = (n / d) as u32;
            self.hi = (n % d) as u32;
        }
    }
    pub fn divu(&mut self, rs: usize, rt: usize) {
        let (n, d) = (self.r[rs], self.r[rt]);
        if d == 0 {
            self.hi = n;
            self.lo = 0xFFFF_FFFF;
        } else {
            self.lo = n / d;
            self.hi = n % d;
        }
    }

    pub fn mfc0(&mut self, rd: usize) -> u32 {
        self.cop0[rd & 15]
    }
    pub fn mtc0(&mut self, rd: usize, v: u32) {
        self.cop0[rd & 15] = v;
    }
    pub fn brk(&mut self, pc: u32, code: u32) {
        let msg = format!("break {code}");
        self.fatal(pc, &msg);
    }
    pub fn syscall(&mut self, pc: u32) {
        self.fatal(pc, "syscall in module code");
    }

    // ------------------------------------------------------------ I/O

    pub fn io_read(&mut self, p: u32, size: u32) -> u32 {
        if (0x1F80_0000..0x1F80_0400).contains(&p) {
            let o = (p & 0x3FF) as usize;
            let mut v = 0u32;
            for i in 0..size as usize {
                v |= (self.scratch[(o + i) & 0x3FF] as u32) << (8 * i);
            }
            return v;
        }
        if (0x1F90_0000..0x1F90_0800).contains(&p) {
            let lo = self.spu.read(p);
            return if size == 4 { lo | (self.spu.read(p + 2) << 16) } else { lo };
        }
        if let Some(v) = hw::read(self, p, size) {
            return v;
        }
        *self.unknown_io.entry(p).or_default() += 1;
        self.io_regs.get(&p).copied().unwrap_or(0)
    }

    pub fn io_write(&mut self, p: u32, v: u32, size: u32) {
        if (0x1F80_0000..0x1F80_0400).contains(&p) {
            let o = (p & 0x3FF) as usize;
            for i in 0..size as usize {
                self.scratch[(o + i) & 0x3FF] = (v >> (8 * i)) as u8;
            }
            return;
        }
        if (0x1F90_0000..0x1F90_0800).contains(&p) {
            spu2::write_reg(self, p, v & 0xFFFF);
            if size == 4 {
                spu2::write_reg(self, p + 2, v >> 16);
            }
            return;
        }
        if hw::write(self, p, v, size) {
            return;
        }
        *self.unknown_io.entry(p).or_default() += 1;
        self.io_regs.insert(p, v);
    }

    // ------------------------------------------------------------ RAM helpers for natives

    pub fn read_bytes(&self, a: u32, n: usize) -> Vec<u8> {
        let o = (a & 0x1F_FFFF) as usize;
        let end = (o + n).min(RAM_SIZE);
        let mut v = self.ram[o..end].to_vec();
        v.resize(n, 0);
        v
    }
    pub fn write_bytes(&mut self, a: u32, data: &[u8]) {
        let o = (a & 0x1F_FFFF) as usize;
        let end = (o + data.len()).min(RAM_SIZE);
        self.ram[o..end].copy_from_slice(&data[..end - o]);
    }
    pub fn read_cstr(&self, a: u32, max: usize) -> String {
        let o = (a & 0x1F_FFFF) as usize;
        self.ram[o..(o + max).min(RAM_SIZE)].iter().take_while(|&&b| b != 0).map(|&b| b as char).collect()
    }
    pub fn ram32(&self, a: u32) -> u32 {
        let o = (a & 0x1F_FFFC) as usize;
        u32::from_le_bytes(self.ram[o..o + 4].try_into().unwrap())
    }
    pub fn set_ram32(&mut self, a: u32, v: u32) {
        let o = (a & 0x1F_FFFC) as usize;
        self.ram[o..o + 4].copy_from_slice(&v.to_le_bytes());
    }
    /// Argument `i` of the current call (a0..a3, then the stack).
    pub fn arg(&self, i: usize) -> u32 {
        if i < 4 {
            self.r[4 + i]
        } else {
            self.ram32(self.r[29].wrapping_add(16 + 4 * (i as u32 - 4)))
        }
    }

    // ------------------------------------------------------------ native → guest

    /// Call guest code at `addr` from native code (interrupt handlers and
    /// callbacks): runs nested on the interrupt stack until it returns.
    pub fn call_guest(&mut self, addr: u32, args: &[u32]) -> u32 {
        if self.halted.is_some() {
            return 0;
        }
        let saved_r = self.r;
        let (hi, lo, pc, depth, next) = (self.hi, self.lo, self.pc, self.depth, self.next_event);
        let saved_deadline = self.handler_deadline;
        self.in_handler += 1;
        for (i, &a) in args.iter().take(4).enumerate() {
            self.r[4 + i] = a;
        }
        self.r[29] = irx::layout::INTR_STACK - (self.in_handler - 1) * 0x800;
        self.r[31] = RETURN_MAGIC;
        self.pc = addr;
        self.next_event = self.cycles + 100_000;
        self.handler_deadline = self.cycles + 50_000_000;
        trampoline(self, Some(RETURN_MAGIC));
        let v = self.r[2];
        self.r = saved_r;
        self.hi = hi;
        self.lo = lo;
        self.pc = pc;
        self.depth = depth;
        self.next_event = next;
        self.handler_deadline = saved_deadline;
        self.in_handler -= 1;
        v
    }

    // ------------------------------------------------------------ scheduling

    /// Run the IOP until its clock reaches `target` (IOP cycles).
    pub fn run(&mut self, target: u64) {
        while self.cycles < target && self.halted.is_none() {
            self.service();
            if self.halted.is_some() {
                break;
            }
            let next = self.k.next_timed().min(self.spu.next_tick).min(target);
            if self.k.current == 0 {
                self.cycles = self.cycles.max(next);
                continue;
            }
            self.next_event = next.max(self.cycles + 1).min(self.cycles + SLICE);
            self.yield_now = false;
            trampoline(self, None);
        }
    }

    /// Timed events, SPU2 output, incoming SIF traffic, interrupts, and the
    /// thread switch they cause.
    fn service(&mut self) {
        while let Some(ev) = self.k.pop_due(self.cycles) {
            kernel::timed_event(self, ev);
        }
        while self.cycles >= self.spu.next_tick {
            spu2::tick(self);
        }
        sif::service(self);
        if !self.k.intr_suspended || self.k.current == 0 {
            kernel::vblank_start(self);
            hw::deliver_interrupts(self);
        }
        kernel::reschedule(self);
    }
}

/// Run guest code from `c.pc` until `stop` is reached (nested calls) or the
/// scheduler asks for a yield (top level).
pub fn trampoline(c: &mut Iop, stop: Option<u32>) {
    let base = c.depth;
    loop {
        c.pc &= !UNWIND;
        if c.halted.is_some() {
            return;
        }
        if Some(c.pc) == stop {
            return;
        }
        if stop.is_none() && c.yield_now {
            c.yield_now = false;
            return;
        }
        match c.pc {
            RETURN_MAGIC => {
                c.fatal(RETURN_MAGIC, "handler return outside a handler");
                return;
            }
            THREAD_EXIT => {
                kernel::exit_thread(c);
                continue;
            }
            EXEC_FINISH => {
                sif::exec_finish(c);
                continue;
            }
            RPC_LOOP => {
                sif::rpc_loop_resume(c);
                continue;
            }
            _ => {}
        }
        c.depth = base;
        let pc = c.pc;
        match lookup(pc) {
            Some(f) => f(c),
            None => call_stub_or_miss(c, pc),
        }
    }
}

fn call_stub_or_miss(c: &mut Iop, pc: u32) {
    match stub_at(pc) {
        Some(StubKind::Native(lib, idx)) => imports::call(c, lib as usize, idx as u32),
        Some(StubKind::Link(t)) => {
            c.pc = t;
            if let Some(f) = lookup(t) {
                f(c)
            }
        }
        None => c.fatal(pc, "no translated IOP code at target"),
    }
}

/// Indirect call (`jalr`, or a tail jump to another function).
#[inline(never)]
pub fn call_indirect(c: &mut Iop, target: u32) {
    match lookup(target) {
        Some(f) => f(c),
        None => call_stub_or_miss(c, target),
    }
}

/// `jr` to an address that is not a label of the current function.
pub fn jump_out(c: &mut Iop, pc: u32, start: u32, end: u32) {
    if pc >= start && pc < end && lookup(pc).is_some() {
        c.fatal(pc, "jump to a non-label inside the function");
        return;
    }
    if !c.enter(pc) {
        return;
    }
    call_indirect(c, pc);
    c.leave();
}
