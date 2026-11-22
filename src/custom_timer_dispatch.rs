//! Native EA timer interrupt acknowledgement and registered callbacks.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x003e_4db8, timer_dispatch, "EATimer_InterruptHandler")];

const TIMER_GLOBALS: u64 = 0x0045_0000;
const CALLBACKS: u32 = 0x0045_0da8;
const IN_CALLBACK: u32 = 0x0de4;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// Acknowledge the hardware timer, increment both tick counters and call its eight
/// registered handlers in order. The callback table accepts any nonzero word;
/// registration and lifetime management belong to the surrounding timer code.
pub fn timer_dispatch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = TIMER_GLOBALS;
        frame.save(c, S0, 0x40);
        c.g[A1][0] = 0x1000_0000;
        frame.save(c, S1, 0x30);
        c.g[A1][0] |= 0x820;
        frame.save(c, S2, 0x20);
        c.g[A0][0] = 0x1000_0000;
        frame.save(c, S3, 0x10);
        c.g[A0][0] |= 0x800;
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x1000_0000;
        c.g[V0][0] = c.lhu(r(c, V1).wrapping_add(0x0de0));
        c.g[A2][0] |= 0x810;
        c.g[V1][0] = c.lw(r(c, A1));
        c.g[T0][0] = TIMER_GLOBALS;
        c.g[A3][0] = c.lw(r(c, A0));
        c.g[T1][0] = TIMER_GLOBALS;
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.g[S2][0] = TIMER_GLOBALS;
        c.g[V0][0] = sx(r(c, V0).wrapping_sub(r(c, A3)));
        c.g[V1][0] = TIMER_GLOBALS;
        c.sw(r(c, A1), c.g[V0][0]);
        c.g[S1][0] = sx(CALLBACKS);
        c.sw(r(c, A0), 0);
        c.g[S3][0] = 1;
        c.g[S0][0] = 7;
        c.g[V1][0] = c.lw(r(c, A2));
        c.g[V1][0] |= 0x400;
        c.sw(r(c, A2), c.g[V1][0]);
        c.g[V0][0] = c.lw(r(c, T0).wrapping_add(0x0dcc));
        c.g[V0][0] = sx(r(c, V0).wrapping_add(1));
        c.sw(r(c, T0).wrapping_add(0x0dcc), c.g[V0][0]);
        c.g[V1][0] = c.lw(r(c, T1).wrapping_add(0x0dd0));
        c.g[V1][0] = sx(r(c, V1).wrapping_add(1));
        c.sw(r(c, T1).wrapping_add(0x0dd0), c.g[V1][0]);

        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r(c, S1));
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                let callback = r(c, V0);
                c.g[RA][0] = sx(0x003e_4e5c);
                c.sw(r(c, S2).wrapping_add(IN_CALLBACK), c.g[S3][0]);
                call_ptr(c, callback, 0x003e_4e5c)?;
                c.sw(r(c, S2).wrapping_add(IN_CALLBACK), 0);
            }
            c.g[S0][0] = sx(r(c, S0).wrapping_sub(1));
            c.cycles += 2;
            let more_callbacks = (c.g[S0][0] as i64) >= 0;
            c.g[S1][0] = sx(r(c, S1).wrapping_add(4));
            if !more_callbacks {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003e_4e48);
            }
        }

        c.cycles += 10;
        c.ei();
        frame.restore(c, S0, 0x40);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
