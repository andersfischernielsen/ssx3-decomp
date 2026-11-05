//! `src/ealib/seg_2dcae8.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::misc::add32;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x003e_5928,
    ea_timer_service_callbacks,
    "EATimer_ServiceCallbacks",
)];

// ---------------------------------------------------------------------------
// EA timer.

/// The current timer tick (`0x450dd0`) and its copy taken by the service
/// pass (`0x450e04`).
pub(crate) const EA_TIMER_NOW: u32 = 0x0045_0dd0;

pub(crate) const EA_TIMER_SERVICED_AT: u32 = 0x0045_0e04;

/// Sixteen timer callbacks of 0x10 bytes: `+0` the function, `+4` its
/// period, `+8` the tick it is next due, `+0xc` set while it runs.
pub(crate) const EA_TIMERS: u32 = 0x0051_ed98;

pub(crate) const EA_TIMER_COUNT: u64 = 16;

/// `int EATimer_ServiceCallbacks(void* arg)` (0x3e5928): run every timer
/// callback that is due and not already running, as `fn(arg, now - due)`,
/// and schedule it one period later (from the tick after the call). Returns
/// the callbacks' results OR'ed together. Walks the table from the end.
pub fn ea_timer_service_callbacks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = 0x45_0000;
        frame.save(c, S3, 0x40);
        c.g[A1][0] = 0x45_0000;
        c.g[S3][0] = c.g[V0][0];
        c.g[V1][0] = Addr(EA_TIMER_NOW).lw(c);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[V0][0] = sx(EA_TIMERS);
        frame.save(c, S2, 0x50);
        // s0 = &timer.running.
        c.g[S0][0] = sx(EA_TIMERS + 0xc);
        frame.save(c, S4, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S6, 0x10);
        c.g[S5][0] = 1;
        frame.save_ra(c, 0);
        c.g[S6][0] = c.g[S3][0];
        Addr(EA_TIMER_SERVICED_AT).sw(c, c.g[V1][0]);
        c.g[S1][0] = EA_TIMER_COUNT - 1;
        loop {
            // Head 0x3e5980.
            let running = r32(c, S0);
            c.cycles += 3;
            c.g[A2][0] = Addr(running - 0xc).lw(c);
            if c.g[A2][0] != 0 {
                c.cycles += 5;
                c.g[V0][0] = Addr(EA_TIMER_NOW).lw(c);
                c.g[V1][0] = Addr(running - 4).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(running).lw(c);
                    if c.g[V0][0] == 0 {
                        c.cycles += 12;
                        c.g[A1][0] = Addr(EA_TIMER_NOW).lw(c);
                        c.g[A0][0] = c.g[S4][0];
                        Addr(running).sw(c, c.g[S5][0]);
                        let callback = r32(c, A2);
                        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(r32(c, V1)));
                        call_ptr(c, callback, 0x003e_59c0)?;
                        c.g[S2][0] |= c.g[V0][0];
                        c.g[V1][0] = Addr(EA_TIMER_NOW).lw(c);
                        c.g[V0][0] = Addr(running - 8).lw(c);
                        Addr(running).sw(c, 0);
                        c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
                        Addr(running - 4).sw(c, c.g[V1][0]);
                    }
                }
            }
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = add32(c.g[S0][0], 0x10);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003e_5980);
            }
        }
        c.cycles += 11;
        c.g[V0][0] = c.g[S2][0];
        for (reg, off) in [
            (S0, 0x70),
            (S1, 0x60),
            (S2, 0x50),
            (S3, 0x40),
            (S4, 0x30),
            (S5, 0x20),
            (S6, 0x10),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::driven;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::put_words;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    /// `EATimer_ServiceCallbacks` over sixteen timers: empty, not yet due,
    /// already running, and due ones (fake callbacks, some of which unwind).
    #[test]
    fn ea_timer_service_callbacks_matches() {
        check(
            ee::s14::f_003e5928__generated,
            ea_timer_service_callbacks,
            150,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x3e_5928, 16);
                let now = r.next_u32() % 1000;
                c.bus.write32(EA_TIMER_NOW, now);
                for i in 0..16u32 {
                    let t = EA_TIMERS + 0x10 * i;
                    let kind = r.next_u32() % 4;
                    c.bus
                        .write32(t, if kind == 0 { 0 } else { FAKE + 0x10 * i });
                    c.bus.write32(t + 4, r.next_u32() % 100);
                    c.bus
                        .write32(t + 8, now.wrapping_add(r.next_u32() % 40).wrapping_sub(20));
                    c.bus.write32(t + 0xc, (kind == 1) as u32);
                }
                c.g[A0][0] = r.next_u32() as u64;
                c.pc = 0x003e_5928;
            },
        );
    }
}
