//! Native race-result snapshots, with separate policy for additional courses.
//!
//! The result-screen snapshot belongs to BEScoreInt; high-score tables and
//! character profiles are separate writes later in the submission path.

use crate::call::{call_ptr, guest, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0015_4ab8,
    submit_race_result,
    "BEScoreInt_SubmitRaceResult",
)];

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn load_unaligned(c: &mut Cpu, to: usize, from: usize, offset: u32) {
    c.g[to][0] = Addr(r(c, from).wrapping_add(offset)).read_u64_unaligned(c);
}

#[inline]
fn store_unaligned(c: &mut Cpu, to: usize, offset: u32, from: usize) {
    let address = Addr(r(c, to).wrapping_add(offset));
    let value = c.g[from][0];
    address.write_u64_unaligned(c, value);
}

/// Copy the first 0xa0 bytes of the result while preserving both alignment
/// paths' original scratch registers and event boundaries.
fn copy_result(c: &mut Cpu) {
    c.cycles += 7;
    c.mult(0, V0, S2, V1);
    c.g[V1][0] = sx(r(c, V0).wrapping_add(r(c, S1)));
    c.g[V1][0] = sx(r(c, V1).wrapping_add(0x20));
    c.g[V0][0] = (c.g[S5][0] | c.g[V1][0]) & 7;
    let unaligned = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S5][0];
    c.cycles += 8;
    if unaligned {
        c.g[A2][0] = sx(r(c, S2) << 2);
        c.g[A3][0] = sx(r(c, S1).wrapping_add(0x10));
        c.sw(r(c, SP).wrapping_add(0x14), c.g[A2][0]);
        c.g[V0][0] = sx(r(c, S5).wrapping_add(0xa0));
        c.sw(r(c, SP).wrapping_add(0x10), c.g[A3][0]);
    } else {
        c.g[T0][0] = sx(r(c, S2) << 2);
        c.g[T1][0] = sx(r(c, S1).wrapping_add(0x10));
        c.sw(r(c, SP).wrapping_add(0x14), c.g[T0][0]);
        c.g[V0][0] = sx(r(c, S5).wrapping_add(0xa0));
        c.sw(r(c, SP).wrapping_add(0x10), c.g[T1][0]);
    }
    c.g[A1][0] = sx(r(c, S1).wrapping_add(0xcc));
    c.g[S6][0] = 0x0044_0000;
    c.g[FP][0] = sx(r(c, S1).wrapping_add(0x18));
    loop {
        if unaligned {
            c.cycles += 19;
            load_unaligned(c, T0, A0, 0);
            load_unaligned(c, T1, A0, 8);
            load_unaligned(c, A2, A0, 0x10);
            load_unaligned(c, A3, A0, 0x18);
            store_unaligned(c, V1, 0, T0);
            store_unaligned(c, V1, 8, T1);
            store_unaligned(c, V1, 0x10, A2);
            store_unaligned(c, V1, 0x18, A3);
        } else {
            c.cycles += 11;
            c.g[A2][0] = c.ld(r(c, A0));
            c.g[A3][0] = c.ld(r(c, A0).wrapping_add(8));
            c.g[T0][0] = c.ld(r(c, A0).wrapping_add(0x10));
            c.g[T1][0] = c.ld(r(c, A0).wrapping_add(0x18));
            c.sd(r(c, V1), c.g[A2][0]);
            c.sd(r(c, V1).wrapping_add(8), c.g[A3][0]);
            c.sd(r(c, V1).wrapping_add(0x10), c.g[T0][0]);
            c.sd(r(c, V1).wrapping_add(0x18), c.g[T1][0]);
        }
        c.g[A0][0] = sx(r(c, A0).wrapping_add(0x20));
        let more = c.g[A0][0] != c.g[V0][0];
        c.g[V1][0] = sx(r(c, V1).wrapping_add(0x20));
        if !more {
            c.cycles += if unaligned { 2 } else { 1 };
            c.g[V0][0] = 0xb0;
            break;
        }
        if event_due(c) {
            deliver_event_here(c, if unaligned { 0x0015_4b48 } else { 0x0015_4bbc });
        }
    }
}

/// Insert into the five-entry, 20-byte high-score table. The selected rank
/// is also written into BEScoreInt's per-profile result-screen fields.
fn submit_high_score(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, 0x0015_57e0, 0x0015_4d1c)?;
    let allowed = c.g[V0][0] != 0;
    c.g[A1][0] = c.lw(r(c, SP));
    if !allowed {
        return Ok(());
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S6][0];
    call_ptr(c, 0x0014_ab20, 0x0015_4d2c)?;
    c.g[A2][0] = c.g[V0][0];
    let has_table = c.g[A2][0] != 0;
    c.g[V0][0] = 1;
    if !has_table {
        return Ok(());
    }
    c.cycles += 8;
    c.g[S0][0] = 0;
    c.g[V0][0] = sx(r(c, V0).wrapping_sub(r(c, S2)));
    c.g[S1][0] = sx(r(c, A2).wrapping_add(8));
    c.g[V0][0] = sx(r(c, V0) << 2);
    c.g[A1][0] = c.g[A2][0];
    c.sw(r(c, SP).wrapping_add(0xc), c.g[V0][0]);
    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x14));
    c.g[S2][0] = sx(r(c, FP).wrapping_add(r(c, V0)));

    loop {
        c.cycles += 3;
        c.g[V1][0] = c.lw(r(c, SP).wrapping_add(8));
        let compare_existing_first = c.g[V1][0] != 0;
        c.g[V0][0] = c.lw(r(c, A1));
        if compare_existing_first {
            c.cycles += 2;
            c.g[V0][0] = (c.g[V0][0] < c.g[S3][0]) as u64;
        } else {
            c.cycles += 1;
            c.g[V0][0] = (c.g[S3][0] < c.g[V0][0]) as u64;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 4) as u64;
            let shift = c.g[V0][0] != 0;
            c.g[A0][0] = 4;
            if shift {
                c.cycles += 1;
                c.g[V1][0] = sx(r(c, A2).wrapping_add(0x50));
                loop {
                    c.cycles += 14;
                    c.g[A0][0] = sx(r(c, A0).wrapping_sub(1));
                    load_unaligned(c, A2, V1, 0u32.wrapping_sub(0x14));
                    load_unaligned(c, A3, V1, 0u32.wrapping_sub(0xc));
                    c.g[T0][0] = c.lw(r(c, V1).wrapping_sub(4));
                    store_unaligned(c, V1, 0, A2);
                    store_unaligned(c, V1, 8, A3);
                    c.sw(r(c, V1).wrapping_add(0x10), c.g[T0][0]);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V1][0] = sx(r(c, V1).wrapping_sub(0x14));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_4d88);
                    }
                }
            }

            c.cycles += 19;
            c.sw(r(c, A1).wrapping_add(4), c.g[S7][0]);
            c.sw(r(c, A1), c.g[S3][0]);
            call_ptr(c, 0x0014_dc80, 0x0015_4dcc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0014_dd58, 0x0015_4dd8)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S4][0];
            call_ptr(c, 0x0014_7170, 0x0015_4de4)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0041_66f4, 0x0015_4df0)?;
            c.sw(r(c, S2), c.g[S0][0]);
            c.g[V1][0] = 0x0053_0000;
            c.g[V1][0] = sx(r(c, V1).wrapping_add(0x5bc8));
            c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x34));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
            let fewer_than_two = c.g[V0][0] != 0;
            c.g[A0][0] = c.lw(r(c, SP).wrapping_add(0xc));
            if !fewer_than_two {
                c.cycles += 5;
                c.g[A1][0] = sx(r(c, FP).wrapping_add(r(c, A0)));
                c.g[V1][0] = c.lw(r(c, A1));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S0][0] as i64)) as u64;
                let earlier_rank = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r(c, V1).wrapping_add(1));
                if !earlier_rank {
                    c.cycles += 3;
                    c.g[V1][0] = ((c.g[V0][0] as i64) < 5) as u64;
                    c.sw(r(c, A1), c.g[V0][0]);
                    if c.g[V1][0] == 0 {
                        c.cycles += 3;
                        c.g[A2][0] = u64::MAX;
                        c.sw(r(c, A1), c.g[A2][0]);
                    }
                }
            }
            return Ok(());
        }

        c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
        c.cycles += 4;
        c.g[S1][0] = sx(r(c, S1).wrapping_add(0x14));
        c.g[V0][0] = ((c.g[S0][0] as i64) < 5) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = sx(r(c, A1).wrapping_add(0x14));
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x0015_4d58);
        }
    }
}

/// `SubmitRaceResult(self, rider, result_0xac_bytes, score)` (0x154ab8).
/// The original implementation accepts the two persistent player profiles.
/// Additional courses retain the native result snapshot without submitting
/// their template course's records or character-profile scores.
pub fn submit_race_result(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S1, 0xa0);
        frame.save(c, S4, 0x70);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x80);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S5, 0x60);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S2, 0x90);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S0, 0xb0);
        frame.save(c, S6, 0x50);
        frame.save(c, S7, 0x40);
        frame.save(c, FP, 0x30);
        frame.save_ra(c, 0x20);
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x0014_a0e0, 0x0015_4afc)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V0][0] = ((c.g[S2][0] as i64) < 2) as u64;
        let eligible_profile = c.g[V0][0] != 0;
        c.g[V1][0] = 0xb0;

        let mut restore_s0_again = true;
        if eligible_profile {
            copy_result(c);
            c.cycles += 53;
            c.g[S0][0] = u64::MAX;
            c.mult(0, A2, S2, V0);
            c.g[V0][0] = sx(r(c, A2).wrapping_add(r(c, A1)));
            load_unaligned(c, A3, A0, 0);
            c.g[T0][0] = c.lw(r(c, A0).wrapping_add(8));
            store_unaligned(c, V1, 0, A3);
            c.sw(r(c, V1).wrapping_add(8), c.g[T0][0]);
            c.sw(r(c, SP).wrapping_add(8), 0);
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S5][0];
            c.sw(r(c, V0), c.g[S0][0]);
            if crate::custom_tracks::active().is_some() {
                let profile = r(c, S2);
                let scores = r(c, S1);
                c.sw(scores + 0x10 + profile * 4, 0);
                c.sw(scores + 0x18 + profile * 4, u64::MAX);
                // The snapshot and placement belong to the current results
                // screen. Everything below this point can write the official
                // template's high scores, records or saved character profile.
                frame.restore(c, S0, 0xb0);
                frame.restore(c, S1, 0xa0);
                frame.restore(c, S2, 0x90);
                frame.restore(c, S3, 0x80);
                frame.restore(c, S4, 0x70);
                frame.restore(c, S5, 0x60);
                frame.restore(c, S6, 0x50);
                frame.restore(c, S7, 0x40);
                frame.restore(c, FP, 0x30);
                frame.restore_ra(c, 0x20);
                frame.close(c);
                return Ok(());
            }
            call_ptr(c, 0x0015_4a58, 0x0015_4c24)?;
            call_ptr(c, 0x0014_dc80, 0x0015_4c2c)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dd58, 0x0015_4c38)?;
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x0014_a080, 0x0015_4c40)?;

            c.g[A2][0] = 0x0053_0000;
            c.g[S7][0] = c.g[V0][0];
            c.g[T1][0] = sx(r(c, A2).wrapping_add(0x5b0));
            c.g[A3][0] = 0x0053_0000;
            c.g[A0][0] = c.lw(r(c, T1).wrapping_add(0x40));
            c.g[V1][0] = 0x64;
            c.g[V0][0] = sx(r(c, A3).wrapping_add(0x5bc8));
            c.g[T0][0] = sx(r(c, S6).wrapping_sub(0x26b0));
            c.g[V0][0] = c.lb(r(c, V0).wrapping_add(0x4a));
            c.mult(0, A0, A0, V1);
            c.g[V1][0] = 0xf88;
            c.g[A3][0] = c.lw(r(c, SP).wrapping_add(0x14));
            c.sw(r(c, SP), c.g[V0][0]);
            c.g[A1][0] = 0x9b50;
            c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x10));
            c.mult(0, V1, S7, V1);
            c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, T0)));
            c.mult(0, A1, S2, A1);
            c.g[A2][0] = sx(r(c, V0).wrapping_add(r(c, A3)));
            c.g[T0][0] = 0x004a_0000;
            c.g[S6][0] = c.lw(r(c, A0));
            c.g[V0][0] = sx(r(c, T0).wrapping_add(0x6ca8));
            c.sw(r(c, A2), 0);
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
            c.g[A3][0] = sx(r(c, FP).wrapping_add(r(c, A3)));
            c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, V1)));
            c.sw(r(c, A3), c.g[S0][0]);
            c.g[V0][0] = c.g[S6][0] ^ 0x16;
            c.sw(r(c, SP).wrapping_add(4), c.g[A1][0]);
            c.g[V1][0] = c.lb(r(c, T1).wrapping_add(0x48));
            let mut convert_score = c.g[V1][0] == 0;
            if c.g[V0][0] == 0 {
                c.g[S6][0] = 0;
            }
            if !convert_score {
                c.cycles += 3;
                c.g[V0][0] = 5;
                convert_score = c.g[V1][0] == c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
            }
            if convert_score {
                c.cycles += 3;
                c.g[T1][0] = 1;
                c.sw(r(c, SP).wrapping_add(8), c.g[T1][0]);
                if (c.g[S3][0] as i64) < 0 {
                    c.cycles += 6;
                    c.g[V1][0] = sx(r(c, S3) >> 1);
                    c.g[V0][0] = (c.g[S3][0] & 1) | c.g[V1][0];
                    c.mtc1(V0, 1);
                    c.cvt_s_w(1, 1);
                    c.add_s(1, 1, 1);
                } else {
                    c.cycles += 4;
                    c.mtc1(S3, 1);
                    c.cvt_s_w(1, 1);
                }
                c.cycles += 5;
                c.lwc1(0, r(c, GP).wrapping_sub(0x6dec));
                c.mul_s(0, 1, 0);
                c.cvt_w_s(1, 0);
                c.mfc1(S3, 1);
                c.g[A0][0] = c.g[S1][0];
            }

            submit_high_score(c)?;
            c.cycles += 3;
            c.g[A3][0] = c.lw(r(c, SP).wrapping_add(8));
            let converted_score = c.g[A3][0] != 0;
            c.g[A0][0] = c.g[S4][0];
            if converted_score {
                c.cycles += 6;
                c.g[A1][0] = c.g[S5][0];
                c.g[A2][0] = u64::MAX;
                c.g[A3][0] = c.g[S3][0];
                call_ptr(c, 0x0015_5420, 0x0015_4e64)?;
            } else {
                c.cycles += 5;
                c.g[A1][0] = c.g[S5][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = u64::MAX;
                call_ptr(c, 0x0015_5420, 0x0015_4e7c)?;
            }
            c.g[T0][0] = 0x0053_0000;
            c.cycles += 4;
            c.g[V0][0] = sx(r(c, T0).wrapping_add(0x5b0));
            c.g[V1][0] = c.lb(r(c, V0).wrapping_add(0x49));
            let skip_profile_record = c.g[V1][0] != 0;
            frame.restore(c, S0, 0xb0);
            if skip_profile_record {
                restore_s0_again = false;
            } else {
                c.cycles += 9;
                c.g[A0][0] = c.lw(r(c, SP).wrapping_add(4));
                c.g[A1][0] = c.g[S6][0];
                c.g[A2][0] = c.lw(r(c, SP));
                c.g[A3][0] = c.g[S3][0];
                call_ptr(c, 0x0015_2460, 0x0015_4ea4)?;
                c.g[T1][0] = c.lw(r(c, SP).wrapping_add(0x10));
                c.g[A0][0] = c.lw(r(c, SP).wrapping_add(0x14));
                c.g[V1][0] = sx(r(c, T1).wrapping_add(r(c, A0)));
                c.sw(r(c, V1), c.g[V0][0]);
            }
        }

        if restore_s0_again {
            c.cycles += 1;
            frame.restore(c, S0, 0xb0);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore(c, FP, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}
