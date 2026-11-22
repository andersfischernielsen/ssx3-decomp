//! Native finish notification, timestamps and rider-local post-finish timer.
//!
//! Retained as an uninstalled port. No differential tests were added or run.

use crate::call::{call_ptr, guest, Callee, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0012_5108, rider_on_finish_race, "Rider_OnFinishRace")];

const FINISHED_TIMER: u32 = 0x470;
const FINISH_TICK: u32 = 0x478;
const PENALTY_TICKS: u32 = 0x47c;
const DID_NOT_FINISH: u32 = 0x480;
const RIDER_VPTR: u32 = 0x6c0;
const METRIX: u32 = 0x790;
const VIEW_INDEX: u32 = 0x870;
const IS_HUMAN: u32 = 0x874;
const IS_LOCAL: u32 = 0x87c;

#[inline]
fn at(c: &Cpu, reg: usize, offset: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(offset)
}

/// `void Rider_OnFinishRace(Rider *rider)` (0x125108).
/// Notify metrix, the event manager, the local HUD and audio. Capture the
/// race clock plus penalty ticks at +0x478. +0x470 is NOT the race duration:
/// this routine only changes a negative value to zero, starting the timer
/// that runs after crossing the finish line.
pub fn rider_on_finish_race(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = c.lw(at(c, GP, 0u32.wrapping_sub(0x204)));
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);

        // Only successful human finishes update this session's flags.
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, S1, DID_NOT_FINISH));
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = c.lw(at(c, S1, IS_HUMAN));
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, S1, IS_LOCAL));
                    let local = c.g[V0][0] != 0;
                    c.g[V0][0] = 1;
                    if local {
                        c.cycles += 2;
                        c.sw(at(c, V1, 0xa0), c.g[V0][0]);
                    } else {
                        c.cycles += 1;
                        c.sw(at(c, V1, 0xa4), c.g[V0][0]);
                    }
                }
            }
        }

        c.cycles += 13;
        c.g[A0][0] = c.lw(at(c, S1, METRIX));
        Callee {
            addr: 0x0011_93e0,
            f: ssx3_ee::s00::RiderMetrix_OnRaceFinished,
        }
        .call(c, 0x0012_515c)?;
        c.g[V0][0] = c.lw(at(c, GP, 0u32.wrapping_sub(0x848)));
        c.mtc1(ZERO, 1);
        c.g[A0][0] = c.lw(at(c, V0, 0x84));
        c.lwc1(0, at(c, S1, FINISHED_TIMER));
        c.g[V1][0] = c.lw(at(c, A0, 0xc));
        c.g[A1][0] = c.lw(at(c, S1, PENALTY_TICKS));
        c.c_lt_s(0, 1);
        c.g[V0][0] = c.lw(at(c, V1, 0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        let start_finished_timer = c.fcc();
        c.sw(at(c, S1, FINISH_TICK), c.g[V0][0]);
        if start_finished_timer {
            c.cycles += 1;
            c.swc1(1, at(c, S1, FINISHED_TIMER));
        }

        // The rider interface supplies its AI index for the finish submission.
        c.cycles += 17;
        c.g[V1][0] = c.lw(at(c, S1, RIDER_VPTR));
        c.g[A1][0] = sx(at(c, S1, RIDER_VPTR));
        c.g[A2][0] = c.lw(at(c, GP, 0u32.wrapping_sub(0x848)));
        c.g[A0][0] = c.lh(at(c, V1, 0x38));
        c.g[V0][0] = c.lw(at(c, V1, 0x3c));
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let get_index = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0012_51ac);
        c.g[S0][0] = c.lw(at(c, A2, 0xc0));
        call_ptr(c, get_index, 0x0012_51ac)?;
        c.g[A2][0] = c.lw(at(c, S1, METRIX));
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = sx(at(c, A2, 0xfc));
        call_ptr(c, 0x0023_8358, 0x0012_51c0)?;
        c.g[A1][0] = c.lw(at(c, S1, VIEW_INDEX));
        c.g[V0][0] = (c.g[A1][0] < 2) as u64;
        let has_local_view = c.g[V0][0] != 0;
        c.g[A0][0] = c.lw(at(c, GP, 0u32.wrapping_sub(0x848)));
        if has_local_view {
            c.cycles += 7;
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[V1][0] = c.lw(at(c, A0, 0x84));
            c.g[V0][0] = c.lw(at(c, V1, 0x84));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = c.lw(at(c, V0, 4));
            c.g[A0][0] = c.lw(at(c, V1, 0xa8));
            call_ptr(c, 0x0016_2258, 0x0012_51ec)?;
        }

        c.cycles += 9;
        call_ptr(c, 0x0028_b180, 0x0012_51f4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0028_6ea0, 0x0012_5200)?;
        c.g[V1][0] = c.lw(at(c, S1, DID_NOT_FINISH));
        c.g[V0][0] = 1;
        if c.g[V1][0] == c.g[V0][0] {
            c.sw(at(c, S1, 0x100), 0);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
