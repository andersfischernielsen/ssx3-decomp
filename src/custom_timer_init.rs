//! Native EA timer-rate initialization and video-rate calibration.

use crate::call::{call_ptr, guest, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x003e_4af0, timer_init, "EATimer_Init")];

const GLOBALS: u64 = 0x0045_0000;
const ADD_INTERRUPT: u32 = 0x0042_3a80;
const ENABLE_INTERRUPT: u32 = 0x0042_48e8;
const SET_VIDEO_MODE: u32 = 0x003e_6690;
const ADD_SHUTDOWN: u32 = 0x003e_4a30;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// `int EATimer_Init(int requested_rate)` (0x3e4af0). An existing interrupt
/// registration returns the stored rate. First use chooses a timer divisor,
/// registers the interrupt handler, measures the video rate if needed and
/// adds shutdown cleanup. This does not initialize the eight callback slots.
pub fn timer_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = GLOBALS;
        frame.save(c, S2, 0x10);
        c.g[A3][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0xddc));
        c.g[S2][0] = c.g[V1][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let initialized = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if initialized {
            c.cycles += 3;
            c.g[V1][0] = GLOBALS;
            c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0xdc8));
        } else {
            configure_rate(c);
            start_timer(c)?;
            c.cycles += 3;
            c.g[A0][0] = sx(r(c, S0).wrapping_add(0x4d68));
            call_ptr(c, ADD_SHUTDOWN, 0x003e_4d48)?;
            c.g[V0][0] = c.lw(r(c, S1).wrapping_add(0xdc8));
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Choose clock source and reload value; preserve the original division
/// results and scratch registers, including the source in A0/A1.
fn configure_rate(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 100;
    if c.g[A3][0] == 0 {
        c.g[A3][0] = c.g[V0][0];
    }
    c.g[V1][0] = ((c.g[A3][0] as i64) < 3150) as u64;
    let slow_clock = c.g[V1][0] != 0;
    c.g[V0][0] = ((c.g[A3][0] as i64) < 196) as u64;
    if slow_clock {
        c.cycles += 2;
        let slowest_clock = c.g[V0][0] != 0;
        c.g[V1][0] = sx(r(c, A3) >> 31);
        if slowest_clock {
            c.cycles += 6;
            c.g[A0][0] = 0x0008_0000;
            c.g[V1][0] = sx(r(c, A3).wrapping_add(r(c, V1)));
            c.g[A0][0] |= 0xca00;
            c.g[V1][0] = sx(((r(c, V1) as i32) >> 1) as u32);
            c.cycles += 8;
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
            c.g[A1][0] = GLOBALS;
            c.div(0, V1, A3);
            c.g[A2][0] = GLOBALS;
            c.g[S1][0] = c.g[A2][0];
            c.g[T0][0] = c.g[A1][0];
            c.g[A3][0] = 2;
        } else {
            c.cycles += 6;
            c.g[A0][0] = 0x008c_0000;
            c.g[V1][0] = sx(r(c, A3).wrapping_add(r(c, V1)));
            c.g[A0][0] |= 0xa000;
            c.g[V1][0] = sx(((r(c, V1) as i32) >> 1) as u32);
            c.cycles += 7;
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
            c.g[A1][0] = GLOBALS;
            c.div(0, V1, A3);
            c.g[A2][0] = GLOBALS;
            c.g[S1][0] = c.g[A2][0];
            c.g[T0][0] = c.g[A1][0];
            c.g[A3][0] = 1;
        }
        c.cycles += 7;
        c.g[V1][0] = c.lo[0];
        c.sh(r(c, A1).wrapping_add(0xde0), c.g[V1][0]);
        c.g[V0][0] = c.g[V1][0] & 0xffff;
        c.div(0, A0, V0);
        c.g[A0][0] = c.lo[0];
        c.sw(r(c, A2).wrapping_add(0xdc8), c.g[A0][0]);
    } else {
        c.cycles += 5;
        c.g[V1][0] = sx(r(c, A3) >> 31);
        c.g[A1][0] = 0x08ca_0000;
        c.g[V1][0] = sx(r(c, A3).wrapping_add(r(c, V1)));
        c.cycles += 14;
        c.g[V1][0] = sx(((r(c, V1) as i32) >> 1) as u32);
        c.g[A0][0] = GLOBALS;
        c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
        c.g[A2][0] = GLOBALS;
        c.div(0, V1, A3);
        c.g[S1][0] = c.g[A2][0];
        c.g[T0][0] = c.g[A0][0];
        c.g[A3][0] = 0;
        c.g[V1][0] = c.lo[0];
        c.sh(r(c, A0).wrapping_add(0xde0), c.g[V1][0]);
        c.g[V0][0] = c.g[V1][0] & 0xffff;
        c.div(0, A1, V0);
        c.g[A1][0] = c.lo[0];
        c.sw(r(c, A2).wrapping_add(0xdc8), c.g[A1][0]);
    }
}

fn start_timer(c: &mut Cpu) -> Flow {
    c.cycles += 24;
    c.g[V0][0] = 0x1000_0000;
    c.g[A2][0] = 0x1000_0000;
    c.g[V0][0] |= 0x800;
    c.g[A2][0] |= 0x820;
    c.sw(r(c, V0), 0);
    c.g[V1][0] = 0x1000_0000;
    c.g[V1][0] |= 0x810;
    c.g[A3][0] |= 0x180;
    c.g[V0][0] = c.lhu(r(c, T0).wrapping_add(0xde0));
    c.g[A1][0] = 0x003e_0000;
    c.g[A1][0] = sx(r(c, A1).wrapping_add(0x4db8));
    c.g[A0][0] = 10;
    c.sw(r(c, A2), c.g[V0][0]);
    c.g[S0][0] = 0x003e_0000;
    c.sw(r(c, V1), c.g[A3][0]);
    c.g[A2][0] = 0;
    call_ptr(c, ADD_INTERRUPT, 0x003e_4c58)?;
    c.sw(r(c, S2).wrapping_add(0xddc), c.g[V0][0]);
    c.g[A0][0] = 10;
    call_ptr(c, ENABLE_INTERRUPT, 0x003e_4c64)?;
    c.g[V1][0] = GLOBALS;
    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0xda0));
    let calibrated = c.g[V0][0] != 0;
    c.g[V1][0] = 0x1000_0000;
    if !calibrated {
        calibrate_video(c)?;
    }
    Ok(())
}

fn calibrate_video(c: &mut Cpu) -> Flow {
    c.cycles += 14;
    c.g[V0][0] = 0x1000_0000;
    c.g[A2][0] = c.lw(r(c, V1));
    c.g[V0][0] |= 0x10;
    c.g[A3][0] = c.lw(r(c, V0));
    c.g[V1][0] = 0x83;
    c.sw(r(c, V0), c.g[V1][0]);
    c.g[A1][0] = GLOBALS;
    c.g[A0][0] = c.lw(r(c, A1).wrapping_add(0xdcc));
    c.g[V1][0] = c.lw(r(c, S1).wrapping_add(0xdc8));
    c.g[V0][0] = c.lw(r(c, A1).wrapping_add(0xdcc));
    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
    let wait = c.g[V0][0] != 0;
    c.g[V1][0] = c.g[A1][0];
    if wait {
        c.cycles += 1;
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0xdcc));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003e_4cb0);
            }
        }
    }
    c.cycles += 7;
    c.g[V0][0] = 0x1000_0000;
    c.g[V1][0] = c.lw(r(c, V0));
    c.sw(r(c, V0), c.g[A2][0]);
    c.sw(r(c, V0), c.g[A3][0]);
    c.g[V1][0] = sx(r(c, V1).wrapping_sub(r(c, A2)));
    let positive_delta = (c.g[V1][0] as i64) >= 0;
    c.g[V0][0] = sx(r(c, V1).wrapping_sub(0x3d55));
    if !positive_delta {
        c.cycles += 3;
        c.g[V0][0] = 0x0001_0000;
        c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
        c.g[V0][0] = sx(r(c, V1).wrapping_sub(0x3d55));
    }
    c.cycles += 3;
    c.g[V0][0] = (c.g[V0][0] < 0x63) as u64;
    let video_mode_one = c.g[V0][0] != 0;
    c.g[V0][0] = sx(r(c, V1).wrapping_sub(0x3cd8));
    if video_mode_one {
        c.cycles += 4;
        c.g[A0][0] = 1;
        call_ptr(c, SET_VIDEO_MODE, 0x003e_4d08)?;
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = (c.g[V0][0] < 0x63) as u64;
    let video_mode_two = c.g[V0][0] != 0;
    c.g[V0][0] = sx(r(c, V1).wrapping_sub(0x7081));
    if video_mode_two {
        c.cycles += 4;
        c.g[A0][0] = 2;
        call_ptr(c, SET_VIDEO_MODE, 0x003e_4d24)?;
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = (c.g[V0][0] < 0xc7f) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[A0][0] = 3;
        call_ptr(c, SET_VIDEO_MODE, 0x003e_4d40)?;
    }
    Ok(())
}
