//! Host-thread execution mode: guest threads as stackful coroutines.
//!
//! Every EE thread runs on its own stack (a `corosensei` coroutine), all of
//! them on the one host thread that drives frames. Exactly one of them (or the
//! frame driver, token 0) runs at any time; it holds the **baton**. A guest
//! thread switch inside a syscall saves the outgoing context, loads the
//! incoming one into the shared [`Cpu`] and *suspends* the outgoing
//! coroutine; when it is resumed the syscall simply returns. No host frame is
//! ever unwound, so recompiled functions need no resume labels or unwind
//! checks, and the generator can emit plain structured calls (`f_X(c);`).
//!
//! The frame driver (`run_frame`, token 0) resumes the current guest thread
//! and runs a small scheduler loop: a suspending thread names the token to run
//! next, which the loop resumes, until a thread hands back to token 0 (a
//! VBlank, idle, or a halt). A switch is a user-space stack switch, not an OS
//! thread handoff.
//!
//! Safety: the single shared `Cpu` is used only by the running coroutine. Each
//! suspended coroutine keeps a `&mut Cpu` on its stack while parked inside a
//! syscall; it re-reads all state after the suspension returns (the call
//! takes `&mut Cpu`, so the compiler cannot cache across it).

use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use corosensei::stack::DefaultStack;
use corosensei::{Coroutine, CoroutineResult, Yielder};

use crate::cpu::Cpu;

/// A guest thread: resumed with `()`, suspends with the token to run next.
type GuestCoroutine = Coroutine<(), i32, (), DefaultStack>;

thread_local! {
    /// Token of the running coroutine (0 = the frame driver).
    static CURRENT: Cell<i32> = const { Cell::new(0) };
    /// The coroutines by token (leaked boxes: a parked guest thread is never
    /// dropped, which would unwind its stack).
    static COROUTINES: RefCell<HashMap<i32, *mut GuestCoroutine>> = RefCell::new(HashMap::new());
    /// Each coroutine's yielder, for suspending from deep inside a syscall.
    static YIELDERS: RefCell<HashMap<i32, *const Yielder<(), i32>>> = RefCell::new(HashMap::new());
}

/// The running context's baton token (0 = frame driver).
pub fn my_token() -> i32 {
    CURRENT.with(|t| t.get())
}

/// Hand the baton from `me` to `to` and return when it comes back to `me`.
pub fn handoff(me: i32, to: i32) {
    if me == to {
        return;
    }
    if me == 0 {
        run_from_driver(to);
    } else {
        // Suspend this coroutine; the driver's loop runs `to` (or returns to
        // the frame driver when `to` is 0) and resumes us later.
        let y = YIELDERS.with(|m| m.borrow().get(&me).copied()).expect("running coroutine has a yielder");
        // SAFETY: the yielder lives on this coroutine's stack, which is live
        // while it runs.
        unsafe { (*y).suspend(to) };
    }
}

/// The frame driver's scheduler loop: run `first`, then whatever each
/// suspending thread names, until one hands back to the driver.
fn run_from_driver(first: i32) {
    let mut next = first;
    while next != 0 {
        let co = COROUTINES.with(|m| m.borrow().get(&next).copied());
        let Some(co) = co else {
            panic!("no guest thread with baton token {next}");
        };
        CURRENT.with(|t| t.set(next));
        // SAFETY: coroutines are only touched on this host thread, one at a
        // time; the box is leaked, so the pointer stays valid.
        next = match unsafe { (*co).resume(()) } {
            CoroutineResult::Yield(to) => to,
            CoroutineResult::Return(()) => 0,
        };
    }
    CURRENT.with(|t| t.set(0));
}

#[derive(Clone, Copy)]
struct CpuPtr(*mut Cpu);

/// Create the coroutine for guest thread token `id`. It runs when first
/// resumed: the guest entry point (the context was loaded by whoever switched
/// to it) and, if the entry function returns, ExitThread.
pub fn spawn(c: &mut Cpu, id: i32) {
    let p = CpuPtr(c as *mut Cpu);
    let stack = DefaultStack::new(64 << 20).expect("guest thread stack");
    let co: GuestCoroutine = Coroutine::with_stack(stack, move |y: &Yielder<(), i32>, ()| {
        let p = p;
        YIELDERS.with(|m| m.borrow_mut().insert(id, y as *const _));
        // SAFETY: only the running coroutine uses the Cpu.
        let c = unsafe { &mut *p.0 };
        // Base trampoline: a frame whose return address doesn't match
        // returns early (longjmp-like returns, shared epilogues); resume at
        // `c.pc` until the thread returns to its exit sentinel.
        loop {
            if c.cycles >= c.next_event {
                c.event();
            }
            let pc = c.pc;
            crate::machine::call_indirect(c, pc);
            if c.pc == crate::kernel::THREAD_EXIT || m!(c).kernel.halted.is_some() {
                break;
            }
        }
        // Falling off the entry function = ExitThread. A dormant thread is
        // never resumed again; a restart gets a new coroutine.
        c.g[3][0] = 0x23;
        crate::kernel::syscall(c, crate::kernel::THREAD_EXIT);
    });
    let co = Box::into_raw(Box::new(co));
    COROUTINES.with(|m| m.borrow_mut().insert(id, co));
}
