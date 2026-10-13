//! Native Rust implementations of the EE kernel / SDK boundary.
//!
//! In the recompiled game these are ordinary Rust functions: a `syscall` site
//! becomes a direct call to one of these, not a runtime syscall-number switch.
//! The generator constant-folds the `li v1, N` that precedes a `syscall` and
//! picks the matching function here.
//!
//! Arguments arrive in `$a0..$a3` and the result is returned (the caller writes
//! it to `$v0`). Where a real EE function has side effects on machine state we
//! keep them on [`Cpu`]; the goal is that the generated code reads like normal
//! Rust calling normal Rust.

use crate::Cpu;

/// `SetGsCrt(interlace, mode, field)` — initialise the PCRTC display mode.
pub fn set_gs_crt(_c: &mut Cpu, _interlace: u64, _mode: u64, _field: u64) {}

/// `ResetEE(flags)`.
pub fn reset_ee(_c: &mut Cpu, _flags: u64) {}

/// `InitMainThread(gp, stack, stack_size, args) -> stack_ptr`.
pub fn init_main_thread(_c: &mut Cpu, _gp: u64, stack: u64, _stack_size: u64, _args: u64) -> u64 {
    stack
}

/// `InitHeap(heap, size) -> heap_end`.
pub fn init_heap(c: &mut Cpu, heap: u64, size: u64) -> u64 {
    c.heap_base = heap as u32;
    c.heap_end = (heap as u32).wrapping_add(size as u32);
    c.heap_end as u64
}

/// `EndOfHeap() -> heap_base`.
pub fn end_of_heap(c: &mut Cpu) -> u64 {
    c.heap_base as u64
}

/// `GetHeapEnd() -> heap_base`.
pub fn get_heap_end(c: &mut Cpu) -> u64 {
    c.heap_base as u64
}

/// `Exit(status)` — terminates the game.
pub fn exit(c: &mut Cpu, status: u64) -> ! {
    c.stop(c.pc, 0, format!("Exit({status})"));
    // A Rust program would end here; the harness observes `c.trap`.
    std::panic::panic_any(ExitReached(status));
}

/// Marker thrown by [`exit`] so the host can unwind cleanly.
#[derive(Debug)]
pub struct ExitReached(pub u64);

/// `FlushCache(mode)` — no-op with a coherent memory model.
pub fn flush_cache(_c: &mut Cpu, _mode: u64) {}

/// `GsGetIMR() -> mask`.
pub fn gs_get_imr(c: &mut Cpu) -> u64 {
    c.gs_imr as u64
}

/// `GsPutIMR(mask)`.
pub fn gs_put_imr(c: &mut Cpu, mask: u64) {
    c.gs_imr = mask as u32;
}

/// `SetMemoryMode(mode) -> previous`.
pub fn set_memory_mode(c: &mut Cpu, mode: u64) -> u64 {
    let prev = c.memory_mode;
    c.memory_mode = mode as u32;
    prev as u64
}

/// `FindAddress(id) -> address`.
pub fn find_address(_c: &mut Cpu, _id: u64) -> u64 {
    0
}

/// `SetSyscall(index, address)`.
pub fn set_syscall(_c: &mut Cpu, _index: u64, _address: u64) {}

/// `_kprintf(fmt, ...)` — debug print.
pub fn kprintf(_c: &mut Cpu, _fmt: u64) {}

/// Fallback when the syscall number was not a compile-time constant. This is the
/// *only* place that inspects `$v1` at runtime, and it exists solely so an
/// unrecognised kernel call can be logged rather than miscompiled.
pub fn dynamic_syscall(c: &mut Cpu) -> u64 {
    let n = c.reg(3) as u32;
    if c.syscalls.len() < c.syscall_limit {
        c.syscalls.push(n);
    }
    c.set_reg(2, 0);
    0
}
