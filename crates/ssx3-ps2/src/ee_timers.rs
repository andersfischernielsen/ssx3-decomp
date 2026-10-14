//! EE timer (RCNT0..3) compare interrupts.
//!
//! `hw.rs` models the counters but never raises INTC causes TIM0..TIM3. SSX 3
//! needs them: `EATimer_Init(60)` (0x3e4c50) programs RCNT1 with a compare
//! target, CMPE and ZRET off, and installs `0x3e4db8` on INTC 10. That
//! handler advances EA's tick counter (`0x450dd0`) and runs the periodic task
//! table at `0x51ed98`; among those tasks is the sound-stream service
//! (registered by `0x2ae048`), so without this interrupt music streams are
//! opened but never read.
//!
//! Enabled with `SSX3_IOP=1` (the recompiled sound driver depends on it) or
//! `SSX3_EE_TIMER_IRQ=1`; off by default until it has been validated on the
//! rest of the game.

use std::sync::OnceLock;

use crate::cpu::Cpu;
use crate::hw::int;

/// Per-timer bookkeeping: counter base we last saw, compare events raised.
#[derive(Default, Clone, Copy)]
pub struct TimerIrq {
    base: u64,
    count_at_base: u32,
    fired: u64,
}

static STATE: std::sync::Mutex<[TimerIrq; 4]> = std::sync::Mutex::new([TimerIrq { base: 0, count_at_base: 0, fired: 0 }; 4]);

pub fn enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| {
        let on = |k: &str| std::env::var(k).map(|v| v != "0").unwrap_or(false);
        on("SSX3_EE_TIMER_IRQ") || on("SSX3_IOP")
    })
}

/// EE cycles per counter tick (same table as `hw.rs`).
fn div(mode: u32) -> u64 {
    match mode & 3 {
        0 => 2,
        1 => 32,
        2 => 512,
        _ => 18_743,
    }
}

/// Raise TIMn for every compare match since the last call. Returns the EE
/// cycle of the next match, if any timer is armed.
pub fn service(c: &mut Cpu) -> Option<u64> {
    if !enabled() {
        return None;
    }
    let now = c.bus.hw.now;
    let mut st = STATE.lock().unwrap();
    let mut next: Option<u64> = None;
    for i in 0..4 {
        let t = c.bus.hw.timers[i];
        // Count enabled (CUE) and compare interrupt enabled (CMPE).
        if t.mode & 0x80 == 0 || t.mode & 0x100 == 0 {
            continue;
        }
        let s = &mut st[i];
        if s.base != t.base || s.count_at_base != t.count_at_base {
            *s = TimerIrq { base: t.base, count_at_base: t.count_at_base, fired: 0 };
        }
        let comp = t.comp as u64;
        let cab = t.count_at_base as u64;
        // ZRET: the counter returns to 0 after the compare value.
        let period = if t.mode & 0x40 != 0 { comp + 1 } else { 0x1_0000 };
        let mut first = (comp + period - cab % period) % period;
        if first == 0 {
            first = period;
        }
        let d = div(t.mode);
        let ticks = now.saturating_sub(t.base) / d;
        let events = if ticks >= first { 1 + (ticks - first) / period } else { 0 };
        if events > s.fired {
            s.fired = events;
            c.bus.hw.timers[i].mode |= 0x400; // EQUF
            c.bus.hw.raise(int::TIM0 + i as u32);
        }
        let due = t.base + (first + s.fired * period) * d;
        next = Some(next.map_or(due, |n: u64| n.min(due)));
    }
    next
}
