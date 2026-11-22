//! Native registration of polled EA timer callbacks.

use crate::abi;
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x003e_57f8, timer_add_callback, "EATimer_AddCallback")];

const TIMER_GLOBALS: u64 = 0x0045_0000;
const CALLBACKS: u32 = 0x0051_ed98;
const REGISTRATION_DEPTH: u32 = 0x0e00;
const TICKS: u32 = 0x0dd0;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// `void EATimer_AddCallback(callback, period, delay)` (0x3e57f8).
/// Register or update a callback in the sixteen-entry polled timer table.
/// Each entry holds the callback, period, next tick and a cleared state word.
/// This table is distinct from the eight interrupt callbacks at 0x450da8.
pub fn timer_add_callback(c: &mut Cpu) {
    c.cycles += 5;
    c.g[T1][0] = c.g[A0][0];
    c.g[T0][0] = c.g[A1][0];
    c.g[A0][0] = u64::MAX;
    let ordinary_period = c.g[T0][0] != c.g[A0][0];
    c.g[T4][0] = c.g[A2][0];
    c.cycles += 2;
    if ordinary_period {
        c.g[V0][0] = 1;
        if c.g[T0][0] == 0 {
            c.g[T0][0] = c.g[V0][0];
        }
    } else {
        c.g[T0][0] = 0;
    }

    c.cycles += 9;
    c.g[V0][0] = TIMER_GLOBALS;
    c.g[T3][0] = 0x0052_0000;
    c.g[A1][0] = c.lw(r(c, V0).wrapping_add(REGISTRATION_DEPTH));
    c.g[T5][0] = c.g[V0][0];
    c.g[A2][0] = 0;
    c.g[T2][0] = u64::MAX;
    c.g[V1][0] = sx(r(c, A1).wrapping_add(1));
    c.g[A3][0] = sx(CALLBACKS);
    c.sw(r(c, V0).wrapping_add(REGISTRATION_DEPTH), c.g[V1][0]);

    loop {
        c.cycles += 3;
        c.g[V0][0] = c.lw(r(c, A3));
        let charged_increment = if c.g[V0][0] == c.g[T1][0] {
            c.g[A0][0] = c.g[A2][0];
            true
        } else {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.g[A2][0] = sx(r(c, A2).wrapping_add(1));
                false
            } else {
                c.cycles += 2;
                if c.g[A0][0] != c.g[T2][0] {
                    c.g[A2][0] = sx(r(c, A2).wrapping_add(1));
                    false
                } else {
                    c.cycles += 2;
                    if c.g[A1][0] == 0 {
                        c.g[A0][0] = c.g[A2][0];
                    } else {
                        c.cycles += 1;
                        c.g[A1][0] = sx(r(c, A1).wrapping_sub(1));
                    }
                    true
                }
            }
        };
        if charged_increment {
            c.cycles += 1;
            c.g[A2][0] = sx(r(c, A2).wrapping_add(1));
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A2][0] as i64) < 16) as u64;
        let more_entries = c.g[V0][0] != 0;
        c.g[A3][0] = sx(r(c, A3).wrapping_add(0x10));
        if !more_entries {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003e_5840);
        }
    }

    c.cycles += 3;
    c.g[V0][0] = u64::MAX;
    let no_slot = c.g[A0][0] == c.g[V0][0];
    c.g[V1][0] = sx(r(c, T3).wrapping_sub(0x1268));
    if !no_slot {
        c.cycles += 12;
        c.g[A0][0] = sx(r(c, A0) << 4);
        c.g[A1][0] = TIMER_GLOBALS;
        c.g[A2][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
        c.g[V0][0] = c.lw(r(c, A1).wrapping_add(TICKS));
        c.g[A3][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
        c.sw(r(c, A2), c.g[T1][0]);
        c.g[A1][0] = c.g[A3][0];
        c.sw(r(c, A3).wrapping_add(4), c.g[T0][0]);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T4)));
        c.sw(r(c, A1).wrapping_add(8), c.g[V0][0]);
        c.g[V1][0] = c.g[A1][0];
        c.sw(r(c, V1).wrapping_add(0xc), 0);
    }
    c.cycles += 4;
    c.g[V0][0] = c.lw(r(c, T5).wrapping_add(REGISTRATION_DEPTH));
    c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
    c.sw(r(c, T5).wrapping_add(REGISTRATION_DEPTH), c.g[V0][0]);
    abi::ret(c);
}
