//! Host-thread execution mode.
//!
//! In this mode every EE thread runs on its own host thread, and exactly one
//! of them (or the main thread, id 0) holds the **baton** at any time. A
//! guest thread switch inside a syscall saves the outgoing context, loads the
//! incoming one into the shared [`Cpu`], passes the baton and *blocks*; when the
//! baton comes back the syscall simply returns. No host frame is ever unwound,
//! so recompiled functions need no resume labels or unwind checks, and the
//! generator can emit plain structured calls (`f_X(c);`).
//!
//! The main host thread (baton id 0) drives frames: `run_frame` hands the baton
//! to the current guest thread and waits until a VBlank (or idle) returns it.
//!
//! Safety: the single shared `Cpu` is accessed only by the baton holder. Each
//! host thread keeps a `&mut Cpu` on its stack while parked inside a syscall;
//! it re-reads all state after the blocking call returns (the call takes
//! `&mut Cpu`, so the compiler cannot cache across it).

use std::sync::{Condvar, Mutex, OnceLock};

use crate::cpu::Cpu;

struct Baton {
    holder: Mutex<i32>,
    cv: Condvar,
}

fn baton() -> &'static Baton {
    static B: OnceLock<Baton> = OnceLock::new();
    B.get_or_init(|| Baton { holder: Mutex::new(0), cv: Condvar::new() })
}

thread_local! {
    /// Baton token of the calling host thread (0 = main/frame driver).
    static MY_TOKEN: std::cell::Cell<i32> = const { std::cell::Cell::new(0) };
}

/// The calling host thread's baton token.
pub fn my_token() -> i32 {
    MY_TOKEN.with(|t| t.get())
}

#[derive(Clone, Copy)]
struct CpuPtr(*mut Cpu);
// SAFETY: access is serialized by the baton.
unsafe impl Send for CpuPtr {}

/// Give the baton to `to` (without waiting).
pub fn pass(to: i32) {
    let b = baton();
    *b.holder.lock().unwrap() = to;
    b.cv.notify_all();
}

/// Block until the baton is `me`.
pub fn wait_for(me: i32) {
    let b = baton();
    let mut h = b.holder.lock().unwrap();
    while *h != me {
        h = b.cv.wait(h).unwrap();
    }
}

/// Hand the baton from `me` to `to` and block until it returns to `me`.
pub fn handoff(me: i32, to: i32) {
    if me == to {
        return;
    }
    pass(to);
    wait_for(me);
}

/// Spawn the host thread for guest thread `id`. It waits for the baton, then
/// runs the guest entry point (the context was loaded by whoever switched to
/// it) and, if the entry function returns, performs ExitThread.
pub fn spawn(c: &mut Cpu, id: i32) {
    let p = CpuPtr(c as *mut Cpu);
    let h = std::thread::Builder::new()
        .name(format!("ee-thread-{id}"))
        .stack_size(64 << 20)
        .spawn(move || {
            let p = p;
            MY_TOKEN.with(|t| t.set(id));
            wait_for(id);
            // SAFETY: we hold the baton.
            let c = unsafe { &mut *p.0 };
            // Base trampoline: a frame whose return address doesn't match
            // returns early (longjmp-like returns, shared epilogues); resume
            // at `c.pc` until the thread returns to its exit sentinel.
            loop {
                let pc = c.pc;
                crate::machine::call_indirect(c, pc);
                if c.pc == crate::kernel::THREAD_EXIT || c.kernel.halted.is_some() {
                    break;
                }
            }
            // Falling off the entry function = ExitThread.
            c.g[3][0] = 0x23;
            crate::kernel::syscall(c, crate::kernel::THREAD_EXIT);
            // A dormant thread's host thread never gets the baton again unless
            // restarted, in which case a new host thread is spawned.
        })
        .expect("spawn ee thread");
    drop(h);
}
