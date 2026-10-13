//! # ssx3-runtime
//!
//! Execution state for recompiled code: the R5900 register file, memory, and
//! syscall dispatch. Generated functions (see [`generated`]) take `&mut Cpu` and
//! mutate it directly.
//!
//! The EE kernel is entered with `syscall`; the **syscall number is in `$v1`**
//! (register 3), arguments in `$a0..$a3`, and the result returns in `$v0`
//! (register 2). Confirmed from the real image: `SetGsCrt` does
//! `li v1, 0x2; syscall`.

pub mod generated;
pub mod sdk;
pub mod syscall;

use ssx3_core::{memory_map, Memory};

/// ABI register names, index 0..31.
pub const REG_NAMES: [&str; 32] = [
    "zero", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5", "t6",
    "t7", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "t8", "t9", "k0", "k1", "gp", "sp",
    "fp", "ra",
];

/// Why execution stopped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trap {
    pub pc: u32,
    pub word: u32,
    pub reason: String,
}

/// R5900 execution state.
pub struct Cpu {
    /// 32 general-purpose registers (64-bit).
    pub gpr: [u64; 32],
    pub hi: u64,
    pub lo: u64,
    /// Current program counter (address of the instruction being executed).
    pub pc: u32,
    /// EE main memory and scratchpad.
    pub mem: Memory,
    /// Minimal CP0 state we model.
    pub cp0_status: u32,
    /// Count of executed instructions.
    pub steps: u64,
    /// Instruction budget; execution traps when exceeded.
    pub step_limit: u64,
    /// Set once execution has stopped.
    pub trap: Option<Trap>,
    /// Optional instruction trace (`(pc, word)`), bounded by `trace_limit`.
    pub trace: Vec<(u32, u32)>,
    pub trace_limit: usize,
    /// Syscall numbers seen, in order.
    pub syscalls: Vec<u32>,
    pub syscall_limit: usize,
    /// Kernel heap state maintained by `InitHeap`.
    pub heap_base: u32,
    pub heap_end: u32,
    /// GS interrupt mask, set via `GsPutIMR`.
    pub gs_imr: u32,
    /// EE memory mode, set via `SetMemoryMode`.
    pub memory_mode: u32,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        let mut mem = Memory::new();
        mem.unmapped_reads = 0;
        Self {
            gpr: [0; 32],
            hi: 0,
            lo: 0,
            pc: memory_map::ENTRY_POINT,
            mem,
            cp0_status: 0,
            steps: 0,
            step_limit: 50_000_000,
            trap: None,
            trace: Vec::new(),
            trace_limit: 0,
            syscalls: Vec::new(),
            syscall_limit: 4096,
            heap_base: 0,
            heap_end: 0,
            gs_imr: 0,
            memory_mode: 0,
        }
    }

    pub fn reg(&self, n: u8) -> u64 {
        self.gpr[(n & 31) as usize]
    }

    pub fn set_reg(&mut self, n: u8, v: u64) {
        // Register 0 is hard-wired to zero.
        if n & 31 != 0 {
            self.gpr[(n & 31) as usize] = v;
        }
    }

    /// Fetch and consume the current `pc`, checking the step budget.
    #[inline]
    pub fn tick(&mut self, pc: u32, word: u32) -> bool {
        if self.trap.is_some() {
            return false;
        }
        self.steps += 1;
        self.pc = pc;
        if self.steps > self.step_limit {
            self.stop(pc, word, "step limit exceeded");
            return false;
        }
        if self.trace.len() < self.trace_limit {
            self.trace.push((pc, word));
        }
        true
    }

    pub fn stop(&mut self, pc: u32, word: u32, reason: impl Into<String>) {
        if self.trap.is_none() {
            self.trap = Some(Trap {
                pc,
                word,
                reason: reason.into(),
            });
        }
    }

    /// Record a syscall number (diagnostic; bounded by `syscall_limit`).
    #[inline]
    pub fn note_syscall(&mut self, n: u32) {
        if self.syscalls.len() < self.syscall_limit {
            self.syscalls.push(n);
        }
    }

    /// Record an instruction the translator could not lower and stop.
    #[inline]
    pub fn unimplemented(&mut self, pc: u32, word: u32) {
        self.stop(pc, word, format!("unimplemented instruction {word:08x}"));
    }

    /// Record a call to a function that was not translated and stop.
    #[inline]
    pub fn untranslated_call(&mut self, pc: u32, target: u32) {
        self.stop(pc, 0, format!("call to untranslated function {target:08x}"));
    }

    // -- memory helpers -----------------------------------------------------

    pub fn load_u8(&mut self, addr: u32) -> u64 {
        self.mem.read_u8(addr) as u64
    }
    pub fn load_u16(&mut self, addr: u32) -> u64 {
        self.mem.read_u16(addr) as u64
    }
    pub fn load_u32(&mut self, addr: u32) -> u64 {
        self.mem.read_u32(addr) as u64
    }
    pub fn load_i8(&mut self, addr: u32) -> u64 {
        self.mem.read_u8(addr) as i8 as i64 as u64
    }
    pub fn load_i16(&mut self, addr: u32) -> u64 {
        self.mem.read_u16(addr) as i16 as i64 as u64
    }
    pub fn load_i32(&mut self, addr: u32) -> u64 {
        self.mem.read_u32(addr) as i32 as i64 as u64
    }
    pub fn load_u64(&mut self, addr: u32) -> u64 {
        let lo = self.mem.read_u32(addr);
        let hi = self.mem.read_u32(addr.wrapping_add(4));
        ((hi as u64) << 32) | lo as u64
    }
    pub fn store_u8(&mut self, addr: u32, v: u64) {
        self.mem.write_u8(addr, v as u8);
    }
    pub fn store_u16(&mut self, addr: u32, v: u64) {
        self.mem.write_u16(addr, v as u16);
    }
    pub fn store_u32(&mut self, addr: u32, v: u64) {
        self.mem.write_u32(addr, v as u32);
    }
    pub fn store_u64(&mut self, addr: u32, v: u64) {
        self.mem.write_u32(addr, v as u32);
        self.mem.write_u32(addr.wrapping_add(4), (v >> 32) as u32);
    }
}
