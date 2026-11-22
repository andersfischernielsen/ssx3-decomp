//! Native dispatch for a rider finishing the current event.
//!
//! Additional courses keep the native finish handler and results screen while
//! their score/placement consumers suppress official career writes.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0023_8358,
    on_rider_finished,
    "GameModeMan_OnRiderFinished",
)];

fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

pub fn on_rider_finished(c: &mut Cpu) {
    crate::custom_finish_trace::observe(c, "rider-finished", 0, r(c, A1));
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x90);
        c.g[V1][0] = 0x53_0000;
        frame.save(c, S1, 0x70);
        c.g[V1][0] = sx(r(c, V1).wrapping_add(0x6668));
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S7, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x80);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save_ra(c, 0);

        // The active game-mode handler records the finish before score submission.
        c.g[V0][0] = c.lw(r(c, S7).wrapping_add(4));
        c.g[V0][0] = sx(r(c, V0) << 2);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.g[A2][0] = c.lw(r(c, V0));
        c.g[V1][0] = c.lw(r(c, A2));
        c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0x40));
        c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x44));
        let handler = r(c, V0);
        c.g[A0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
        call_ptr(c, handler, 0x0023_83b8)?;
        crate::custom_finish_trace::observe(c, "handler-returned", 0, handler);
        call_ptr(c, 0x0014_dc80, 0x0023_83c0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_83cc)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_4bc0, 0x0023_83d4)?;
        c.g[V1][0] = c.lw(r(c, V0));
        c.g[V1][0] = ((c.g[V1][0] as i64) < 17) as u64;
        let submit_track = c.g[V1][0] != 0;
        c.g[V0][0] = c.lw(r(c, GP).wrapping_sub(0x848));

        if submit_track {
            c.cycles += 5;
            c.g[V1][0] = c.lw(r(c, V0).wrapping_add(0x84));
            c.g[A0][0] = c.lw(r(c, V1).wrapping_add(0x28));
            c.g[A0][0] = c.lw(r(c, A0));
            let zero_state = c.g[A0][0] == 0;
            c.g[A1][0] = 0;
            if !zero_state {
                c.cycles += 1;
                c.g[A1][0] = ((c.g[A0][0] as i64) < 10) as u64;
            }
            c.cycles += 2;
            let skip_submission = c.g[A1][0] != 0;
            frame.restore(c, S0, 0x80);

            if !skip_submission {
                c.cycles += 18;
                c.g[S0][0] = sx(r(c, S2) << 2);
                call_ptr(c, 0x0014_dc80, 0x0023_840c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 8;
                call_ptr(c, 0x0014_dd58, 0x0023_8418)?;
                c.g[S6][0] = c.g[S0][0];
                c.g[S5][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[V0][0] = 0x53_0000;
                c.g[A0][0] = c.g[S5][0];
                c.g[V0][0] = sx(r(c, V0).wrapping_add(0x6640));
                c.g[A1][0] = c.g[S2][0];
                c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
                c.g[S4][0] = c.lw(r(c, S0));
                c.g[A3][0] = c.g[S4][0];
                call_ptr(c, 0x0015_4ab8, 0x0023_8444)?;

                if c.g[S2][0] == 0 && crate::custom_tracks::active().is_none() {
                    c.cycles += 17;
                    c.g[A0][0] = 0;
                    call_ptr(c, 0x0014_a0e0, 0x0023_8454)?;
                    c.g[A0][0] = 0;
                    c.g[S1][0] = c.g[V0][0];
                    call_ptr(c, 0x0014_a080, 0x0023_8460)?;
                    c.g[S3][0] = c.g[V0][0];
                    call_ptr(c, 0x0014_dc80, 0x0023_8468)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0xd;
                    call_ptr(c, 0x0014_dd58, 0x0023_8474)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x0015_77e0, 0x0023_8488)?;
                    let peak_complete = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    let mut reward_already_granted = false;
                    if peak_complete {
                        c.cycles += 6;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[S1][0];
                        c.g[A3][0] = 0;
                        call_ptr(c, 0x0015_a2e0, 0x0023_84a0)?;
                        reward_already_granted = c.g[V0][0] != 0;
                        if !reward_already_granted {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A2][0] = c.g[S3][0];
                            c.g[A3][0] = 0;
                            call_ptr(c, 0x0015_9cd0, 0x0023_84bc)?;
                        }
                    }
                    if !reward_already_granted {
                        c.cycles += 1;
                    }
                }
                c.g[V0][0] = c.lw(r(c, S7).wrapping_add(0x9c));

                c.cycles += 2;
                let submit_placement = c.g[V0][0] != 0;
                c.g[V0][0] = 0x53_0000;
                if submit_placement {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(0x6730));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[V0][0] = sx(r(c, S6).wrapping_add(r(c, V0)));
                    c.g[A3][0] = c.g[S4][0];
                    c.g[A2][0] = c.lw(r(c, V0));
                    call_ptr(c, 0x0015_4ee8, 0x0023_84e4)?;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x80);
            }
        } else {
            c.cycles += 1;
            frame.restore(c, S0, 0x80);
        }

        c.cycles += 10;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
