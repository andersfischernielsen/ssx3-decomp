//! Event checks in loops: how a replacement with a loop delivers interrupts
//! and thread switches at the same points as the generated code.
//!
//! The machine delivers events (interrupts, VBlank, thread switches) only at
//! `c.event()` calls, which the generated code makes at loop back edges when
//! `c.cycles >= c.next_event`. A replacement makes the same checks with the
//! same `c.pc` and the same cycle count, and copies the shape of its
//! original:
//!
//! - **Structured loops** (`'l_…: loop` in the generated code): at a back
//!   edge, `if event_due(c) { deliver_event(c, head); }`. The result of
//!   `event()` is ignored (a requested unwind is only honoured by a later
//!   check elsewhere) and the function cannot be entered in the middle.
//! - **Dispatcher loops** (`match pc`): the same check, but an unwind request
//!   makes the function return at once with `pc` still at the loop head
//!   ([`back_edge_unwinds`]). The trampoline later re-enters the function
//!   *at that head*, so the replacement must accept those resume addresses
//!   as entry points (`match c.pc { ENTRY => .., HEAD => .., pc =>
//!   c.jump_out(..) }`) and pick up its loop state from the registers. A
//!   convenient structure is one function per stage returning
//!   [`Resume<S>`], driven by a loop in the entry function.
//!
//! Either way the registers are the interface: before an event is delivered
//! every register the loop has changed holds the value the original would
//! have there (a handler or a thread switch saves the context), and after a
//! non-unwinding event the loop state is reloaded from the registers, as the
//! generated code reads them.
//!
//! Tests deliver real events mid-loop and drive re-entry with
//! `difftest::events`.

use ssx3_ee_rt::Cpu;

/// Is an event due? (`cycles >= next_event`, tested at back edges.)
#[inline]
pub fn event_due(c: &Cpu) -> bool {
    c.cycles >= c.next_event
}

/// Deliver an event with `pc` at the loop head `head`; true when the thread
/// must unwind. Callers spill their loop registers first and reload them
/// afterwards.
#[inline]
pub fn deliver_event(c: &mut Cpu, head: u32) -> bool {
    c.pc = head;
    c.event()
}

/// Deliver a due event at a back edge of a dispatcher-shaped loop (the
/// caller has spilled its registers). True when the function must return
/// now; it is re-entered at `head` later.
#[inline]
pub fn back_edge_unwinds(c: &mut Cpu, head: u32) -> bool {
    event_due(c) && deliver_event(c, head)
}

/// Where a resumable (dispatcher-shaped) function continues after a stage:
/// another stage of the original, or out.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Resume<S> {
    /// Continue with stage `S`.
    Go(S),
    /// Returned to the caller.
    Done,
    /// An event asked the thread to unwind; `pc` is the loop head to resume at.
    Unwind,
}
