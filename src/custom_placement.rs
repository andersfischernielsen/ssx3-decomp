//! Native result placement, with additional courses excluded from career writes.

use crate::call::{call_ptr, guest, Frame, Unwound};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0015_4ee8, submit_placement, "BEScoreInt_SubmitPlacement")];

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// Submit the original course's placement to its event, points and reward
/// interfaces. Returns whether a branch delay slot already restored S0.
fn submit_original(c: &mut Cpu, frame: Frame) -> Result<bool, Unwound> {
    call_ptr(c, 0x0014_dc80, 0x0015_4f54)?;
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_dd58, 0x0015_4f60)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[S7][0] = c.g[V0][0];
    call_ptr(c, 0x0014_a080, 0x0015_4f6c)?;
    c.g[V1][0] = c.lw(r(c, S4).wrapping_add(0x40));
    c.g[A0][0] = 0x64;
    c.g[FP][0] = c.g[V0][0];
    c.g[A1][0] = 0x0053_0000;
    c.mult(0, V1, V1, A0);
    c.g[V0][0] = 0x0044_0000;
    c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x26b0));
    c.g[A0][0] = c.g[S1][0];
    c.g[S1][0] = c.lb(r(c, A1).wrapping_add(0x5c12));
    c.g[A1][0] = c.g[S2][0];
    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
    c.g[S3][0] = c.lw(r(c, V1));
    c.g[V0][0] = c.g[S3][0] ^ 0x16;
    if c.g[V0][0] == 0 {
        c.g[S3][0] = 0;
    }
    call_ptr(c, 0x0015_5328, 0x0015_4fa8)?;
    c.g[V1][0] = sx(r(c, S1).wrapping_sub(4));
    c.g[V1][0] = (c.g[V1][0] < 2) as u64;
    let special_event = c.g[V1][0] != 0;
    c.g[S0][0] = c.g[V0][0];
    if special_event {
        c.cycles += 3;
        c.g[S0][0] = u64::MAX;
        c.g[V0][0] = 1;
        if c.g[S2][0] == 0 {
            c.g[S0][0] = c.g[V0][0];
        }
    }

    c.cycles += 3;
    c.g[V0][0] = c.lb(r(c, S4).wrapping_add(0x49));
    let suppress_event_result = c.g[V0][0] != 0;
    c.g[V0][0] = u64::MAX;
    if !suppress_event_result {
        c.cycles += 5;
        c.g[A2][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x001e_25d8, 0x0015_4fe0)?;
        c.g[V0][0] = u64::MAX;
    }
    c.cycles += 2;
    if c.g[S0][0] == c.g[V0][0] {
        frame.restore(c, S0, 0x90);
        return Ok(true);
    }

    c.cycles += 3;
    c.g[V0][0] = c.lb(r(c, S4).wrapping_add(0x49));
    if c.g[V0][0] == 0 {
        c.cycles += 5;
        call_ptr(c, 0x001e_1eb8, 0x0015_5000)?;
        c.g[V0][0] = c.lb(r(c, S4).wrapping_add(0x49));
        let suppress_points = c.g[V0][0] != 0;
        c.g[V0][0] = 4;
        if !suppress_points {
            c.cycles += 2;
            let event_four = c.g[S1][0] == c.g[V0][0];
            c.g[V0][0] = 5;
            if !event_four {
                c.cycles += 2;
                if c.g[S1][0] != c.g[V0][0] {
                    c.cycles += 2;
                    call_ptr(c, 0x001e_1550, 0x0015_5024)?;
                }
            }
        }
    }

    c.cycles += 11;
    call_ptr(c, 0x0014_dc80, 0x0015_502c)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xd;
    call_ptr(c, 0x0014_dd58, 0x0015_5038)?;
    c.g[A0][0] = 0x0053_0000;
    c.g[S4][0] = c.g[V0][0];
    c.g[V1][0] = sx(r(c, A0).wrapping_add(0x5b0));
    c.g[V0][0] = c.lb(r(c, V1).wrapping_add(0x49));
    if c.g[V0][0] != 0 {
        frame.restore(c, S0, 0x90);
        return Ok(true);
    }
    c.cycles += 4;
    c.g[V0][0] = c.lb(r(c, V1).wrapping_add(0x48));
    c.g[S2][0] = 4;
    let skip_reward = c.g[V0][0] == c.g[S2][0];
    c.g[A0][0] = c.g[S7][0];
    if skip_reward {
        return Ok(false);
    }

    c.cycles += 7;
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S1][0];
    call_ptr(c, 0x0014_56a0, 0x0015_506c)?;
    c.g[A3][0] = c.g[V0][0];
    c.g[V0][0] = u64::MAX;
    let has_record_target = c.g[A3][0] != c.g[V0][0];
    c.g[A0][0] = c.g[S4][0];
    if has_record_target {
        c.cycles += 2;
        if c.g[S1][0] == 0 {
            c.g[V0][0] = 0x3c;
            c.cycles += 2;
            c.mult(0, V0, A3, V0);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S6][0] as i64)) as u64;
        } else {
            c.cycles += 2;
            if c.g[S1][0] != c.g[S2][0] {
                c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[A3][0] as i64)) as u64;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0x3c;
                c.cycles += 2;
                c.mult(0, V0, A3, V0);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S6][0] as i64)) as u64;
            }
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[S0][0] = 0;
        }
        c.g[A0][0] = c.g[S4][0];
    }
    c.cycles += 6;
    c.g[A1][0] = c.g[S5][0];
    c.g[A2][0] = c.g[FP][0];
    c.g[A3][0] = c.g[S1][0];
    c.g[T0][0] = c.g[S3][0];
    c.g[T1][0] = c.g[S0][0];
    call_ptr(c, 0x0015_91e8, 0x0015_50b8)?;
    Ok(false)
}

/// `SubmitPlacement(self, rider, zero_based_place, score)` (0x154ee8).
///
/// All courses keep BEScoreInt's result-screen placement. Additional courses
/// stop there: their template event must not receive points, medals or rewards.
pub fn submit_placement(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x70);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S6, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S5, 0x40);
        c.g[S6][0] = c.g[A3][0];
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a0e0, 0x0015_4f2c)?;
        c.g[S5][0] = c.g[V0][0];
        c.g[V0][0] = ((c.g[S5][0] as i64) < 2) as u64;
        let eligible_profile = c.g[V0][0] != 0;
        c.g[V0][0] = 0xb0;
        let mut s0_restored = false;
        if eligible_profile {
            c.cycles += 31;
            c.g[V1][0] = 0x0053_0000;
            c.mult(0, A0, S5, V0);
            c.g[S4][0] = sx(r(c, V1).wrapping_add(0x5b0));
            c.g[V0][0] = sx(r(c, A0).wrapping_add(r(c, S1)));
            c.sw(r(c, V0).wrapping_add(0xcc), c.g[S2][0]);
            if crate::custom_tracks::active().is_none() {
                s0_restored = submit_original(c, frame)?;
            }
        }
        if !s0_restored {
            c.cycles += 1;
            frame.restore(c, S0, 0x90);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
