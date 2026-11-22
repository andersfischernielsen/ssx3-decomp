//! Host-thread execution mode where the host provides the stacks (wasm32).
//!
//! The same contract as the native coroutines (`threads.rs`): every guest
//! thread runs on a stack of its own, exactly one context (or the frame
//! driver, token 0) holds the baton, and a thread switch inside a syscall
//! parks the outgoing stack and returns when the baton comes back. WebAssembly
//! has no stack switching of its own that Rust can use, so the front end
//! supplies it through [`StackSwitcher`] (the browser: JSPI, one suspended
//! wasm stack per guest thread, `ssx3-web`).
//!
//! The front end must:
//! - run [`thread_main`]`(id)` on a fresh stack when `switch` first hands the
//!   baton to a token created by `spawn`;
//! - give each such stack its own linear-memory shadow stack (Rust's
//!   `__stack_pointer` is one global, shared by all wasm stacks): swap it at
//!   every switch;
//! - park `me` in `switch` until a later `switch(_, me)` names it.

use ssx3_core::hash::HashMap;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::cpu::Cpu;

/// The front end's stack switching.
pub trait StackSwitcher {
    /// Guest thread token `id` was created. Its stack starts on the first
    /// `switch` to it, with [`thread_main`]`(id)`.
    fn spawn(&self, id: i32);
    /// Hand the baton from `me` to `to` (0 = the frame driver): start `to`
    /// if it has not run yet, resume it otherwise, and park the calling
    /// stack until the baton comes back to `me`.
    fn switch(&self, me: i32, to: i32);
}

thread_local! {
    static SWITCHER: RefCell<Option<Rc<dyn StackSwitcher>>> = const { RefCell::new(None) };
    /// Token of the running context (0 = the frame driver).
    static CURRENT: Cell<i32> = const { Cell::new(0) };
    /// Threads created but not started: their Cpu.
    static PENDING: RefCell<HashMap<i32, CpuPtr>> = RefCell::new(HashMap::default());
}

#[derive(Clone, Copy)]
struct CpuPtr(*mut Cpu);

/// Install the front end's stack switcher (before the first frame).
pub fn set_switcher(s: Rc<dyn StackSwitcher>) {
    SWITCHER.with(|w| *w.borrow_mut() = Some(s));
}

fn switcher() -> Rc<dyn StackSwitcher> {
    SWITCHER
        .with(|w| w.borrow().clone())
        .expect("no stack switcher installed (ssx3_ps2::threads::set_switcher)")
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
    let s = switcher();
    CURRENT.with(|t| t.set(to));
    s.switch(me, to);
    CURRENT.with(|t| t.set(me));
}

/// Create guest thread token `id`; it runs when first switched to.
pub fn spawn(c: &mut Cpu, id: i32) {
    PENDING.with(|m| m.borrow_mut().insert(id, CpuPtr(c as *mut Cpu)));
    switcher().spawn(id);
}

/// The body of guest thread `id`, run by the front end on the thread's own
/// stack: the guest entry point (its context was loaded by whoever switched
/// to it) and, if the entry function returns, ExitThread.
pub fn thread_main(id: i32) {
    let p = PENDING
        .with(|m| m.borrow_mut().remove(&id))
        .expect("thread_main for a token that was not spawned");
    // SAFETY: only the running context uses the Cpu.
    let c = unsafe { &mut *p.0 };
    // Base trampoline: see `threads.rs`.
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
    c.g[3][0] = 0x23;
    crate::kernel::syscall(c, crate::kernel::THREAD_EXIT);
    // As the coroutine returning: the baton goes to the frame driver and
    // this stack stays parked.
    handoff(id, 0);
}
