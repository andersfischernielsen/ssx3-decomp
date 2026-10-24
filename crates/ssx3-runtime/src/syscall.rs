//! EE kernel syscall dispatch.
//!
//! Called from generated code when it executes `syscall`. The number is in
//! `$v1` (register 3); arguments are in `$a0..$a3`; the result is written to
//! `$v0` (register 2). This implements the syscalls the crt0/init path uses and
//! records the rest rather than failing.

use crate::{Cpu, REG_NAMES};

impl Cpu {
    /// Handle the current `syscall` instruction.
    pub fn syscall(&mut self) {
        let n = self.reg(3) as u32;
        if self.syscalls.len() < self.syscall_limit {
            self.syscalls.push(n);
        }
        let a0 = self.reg(4) as u32;
        let a1 = self.reg(5) as u32;
        let _ = (a0, a1);

        match n {
            0x01 => { /* ResetEE */ }
            0x02 | 0x03 => { /* SetGsCrt / SetGsCrtKn */ }
            0x04 => {
                self.stop(self.pc, 0, "Exit (syscall 0x04)");
            }
            0x3c => {
                // InitMainThread(gp, stack, stack_size, args) -> stack pointer.
                let stack = self.reg(5) as u32;
                self.set_reg(2, stack as u64);
            }
            0x3d => {
                // InitHeap(heap, size) -> heap end.
                let base = self.reg(4) as u32;
                let size = self.reg(5) as u32;
                self.heap_base = base;
                self.heap_end = base.wrapping_add(size);
                self.set_reg(2, self.heap_end as u64);
            }
            0x3e | 0x3f => {
                // EndOfHeap / GetHeapEnd -> heap base.
                self.set_reg(2, self.heap_base as u64);
            }
            0x70 => {
                // GsGetIMR -> current mask.
                self.set_reg(2, self.gs_imr as u64);
            }
            0x71 => {
                // GsPutIMR(mask).
                self.gs_imr = a0;
            }
            0x64 => { /* FlushCache */ }
            0x74 => { /* SetSyscall */ }
            0x79 | 0x7a | 0x7e | 0x7f => { /* OSD config: leave the buffer zeroed */ }
            0x83 => {
                /* FindAddress */
                self.set_reg(2, 0);
            }
            0x85 => {
                // SetMemoryMode(mode) -> previous mode.
                let prev = self.memory_mode;
                self.memory_mode = a0;
                self.set_reg(2, prev as u64);
            }
            _ => {
                // Unknown syscalls return 0; the list is visible via `syscalls`.
                self.set_reg(2, 0);
            }
        }
    }

    /// Human-readable name of a syscall number (see `ssx3_core::syscall`).
    pub fn syscall_name(n: u32) -> &'static str {
        ssx3_core::syscall::name(n).unwrap_or("?")
    }

    /// Format a register for diagnostics.
    pub fn dump_gpr(&self, n: u8) -> String {
        format!("{}={:016x}", REG_NAMES[(n & 31) as usize], self.reg(n))
    }
}
