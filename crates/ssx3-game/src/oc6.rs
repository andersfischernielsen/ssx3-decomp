//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Hand-written, typed replacements for small generated functions, each with a
//! differential test against the generated body.

use crate::abi;
use crate::call::{self, Callee, Frame};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_3198, air_predictor_reset, "AirPredictor_Reset"),
    (0x0011_35b8, air_predictor_prime, "AirPredictor_Prime"),
    (0x0011_3cc0, race_state_game_init_update, "RaceStateGameInit_Update"),
    (0x0011_3cf0, race_state_freeride_update, "RaceStateFreeride_Update"),
    (0x0011_3d48, race_state_countdown_update, "RaceStateCountdown_Update"),
    (0x0011_fea8, rider_get_main_state_name, "Rider_GetMainStateName"),
    (0x0011_3f38, rider_set_slew5_target, "Rider_SetSlew5Target"),
    (0x0011_6938, trick_get_grab_name, "Trick_GetGrabName"),
    (0x0011_6fa8, chudfx_ctor, "cHUDFX_ctor"),
    (0x0011_7520, rider_metrix_set_base_score, "RiderMetrix_SetBaseScore"),
    (0x0011_7948, rider_metrix_get_rounded_trick_points, "RiderMetrix_GetRoundedTrickPoints"),
];

/// `void AirPredictor_Reset(AirPredictor *ap)` (0x113198): zero the timers at
/// `+0x98..+0xa4`, clear the state words at `+0x90`/`+0x30`, then copy three
/// constant vectors (loaded from the data segment) into the struct at `+0x10`,
/// `+0x20` and the unaligned quadword at `+0x34`.
pub fn air_predictor_reset(c: &mut Cpu) {
    let ap = Addr(abi::arg(c, 0));
    c.cycles += 26;

    // Zero the four scalar fields.
    ap.off(0xac).sw(c, 0);
    c.g[V1][0] = 0x50_0000;
    ap.off(0xa0).sw(c, 0);
    // Address of the first constant vector (0x500000 - 0xea0).
    c.g[V1][0] = sx(0x50_0000u32.wrapping_sub(0xea0));
    ap.off(0x98).sw(c, 0);
    c.g[A2][0] = 0x50_0000;
    ap.off(0x9c).sw(c, 0);
    // Address of the second constant vector (0x500000 - 0xee0).
    c.g[A2][0] = sx(0x50_0000u32.wrapping_sub(0xee0));
    ap.off(0xa4).sw(c, 0);

    c.g[A1][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = c.g[A1][0] | 0xffff;
    c.g[A3][0] = 0xffff_ffff_ffff_ffff;

    // lq v0, 0x4ff160 ; sq v0, ap+0x20
    let q2 = Addr(c.g[V1][0] as u32).read_vec4(c);
    abi::set_gpr128(c, V0, q2.to_bits());
    c.g[T0][0] = 0x4a_0000;
    ap.off(0x20).write_vec4(c, q2);

    // lq v1, 0x4ff120 ; sw -1, ap+0x90 ; sq v1, ap+0x10 ; sw -1, ap+0x30
    let q3 = Addr(c.g[A2][0] as u32).read_vec4(c);
    abi::set_gpr128(c, V1, q3.to_bits());
    ap.off(0x90).sw(c, c.g[A3][0]);
    ap.off(0x10).write_vec4(c, q3);
    ap.off(0x30).sw(c, c.g[A1][0]);

    // ldl/ldr v0, 0x4a5960 (unaligned doubleword) then sdl/sdr to ap+0x34.
    c.g[A2][0] = sx(0x4a_0000u32.wrapping_add(0x5960));
    let d = Addr(c.g[A2][0] as u32).read_u64_unaligned(c);
    c.g[V0][0] = d;
    ap.off(0x34).write_u64_unaligned(c, d);

    abi::ret(c);
}

/// `void AirPredictor_Prime(AirPredictor *ap, LaunchState *state, Vec4 *pos)`
/// (0x1135b8): save the frame and callee-saved registers, stash `f12` at
/// `ap+0xa8`, reset the predictor through 0x113198, then forward the three
/// arguments to `AirPredictor_SetLaunchState` (0x113618).
pub fn air_predictor_prime(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x40);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.f[20] = c.f[12]; // mov.s f20, f12

        Callee { addr: 0x0011_3198, f: ssx3_ee::s00::AirPredictor_Reset }.call(c, 0x0011_35e4)?;
        Addr(c.g[S0][0] as u32).off(0xa8).write_fpu(c, FpuF32(c.f[20]));

        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        Callee { addr: 0x0011_3618, f: ssx3_ee::s00::AirPredictor_SetLaunchState }
            .call(c, 0x0011_35f8)?;

        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void RaceStateGameInit_Update(AIRaceState *state, AIRaceState *next)`
/// (0x113cc0): set the object in `a1` to race state 2.
pub fn race_state_game_init_update(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x2;
        Callee { addr: 0x0011_3b10, f: ssx3_ee::s00::AI_SetRaceState }
            .call(c, 0x0011_3cd4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RaceStateFreeride_Update(AIRaceState *state, AIRaceState *next)`
/// (0x113cf0): set the object in `a1` to race state 5.
pub fn race_state_freeride_update(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x5;
        Callee { addr: 0x0011_3b10, f: ssx3_ee::s00::AI_SetRaceState }
            .call(c, 0x0011_3d04)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RaceStateCountdown_Update(AIRaceState *state, AIRaceState *next)`
/// (0x113d48): tick the countdown at `+0x1c`; when it expires, move the
/// object in `a1` to race state 5.
pub fn race_state_countdown_update(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let timer = Addr(c.g[A0][0] as u32).off(0x1c);
        let remaining = timer.lw(c);
        c.g[V0][0] = sx(remaining.wrapping_sub(1) as u32);
        if (remaining as i64) > 0 {
            c.cycles += 2;
            timer.sw(c, c.g[V0][0]);
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 4;
            c.g[A1][0] = 0x5;
            Callee { addr: 0x0011_3b10, f: ssx3_ee::s00::AI_SetRaceState }
                .call(c, 0x0011_3d68)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void Rider_SetSlew5Target(Rider *rider, float target)` (0x113f38): slew the
/// "slew 5" value (`+0x22c`) toward `target` (f12), rate limited by the two gp
/// constants (a per-frame cap and a multiplier), store the unscaled target at
/// `+0x234` and the new value at `+0x230`.
pub fn rider_set_slew5_target(c: &mut Cpu) {
    let rider = Addr(abi::arg(c, 0));
    let target = abi::farg(c, 0); // f12

    c.cycles += 13;
    let cur = rider.off(0x22c).read_fpu(c);
    c.g[AT][0] = 0x40e0_0000; // 7.0
    c.f[2] = c.g[AT][0] as u32; // mtc1 f2, at
    let diff = target - cur;
    let cap = Addr((c.g[GP][0] as u32).wrapping_sub(0x7c28)).read_fpu(c); // f1
    c.f[1] = cap.0;
    let scaled = diff.abs() * FpuF32(c.f[2]);
    c.f[0] = scaled.0;
    c.g[AT][0] = 0x4100_0000; // 8.0
    c.f[3] = c.g[AT][0] as u32; // mtc1 f3, at

    // c.le.s f1, f0: cap <= scaled.
    let within = cap.le(scaled);
    abi::set_fcc(c, within);
    c.f[2] = cap.0; // mov.s f2, f1
    if within {
        c.cycles += 1;
        c.f[2] = scaled.min(FpuF32(c.f[3])).0; // min.s f2, f0, f3
    }

    c.cycles += 5;
    let mul = Addr((c.g[GP][0] as u32).wrapping_sub(0x7c24)).read_fpu(c); // f0
    c.f[0] = mul.0;
    rider.off(0x234).write_fpu(c, target);
    let new = FpuF32(c.f[2]) * mul; // mul.s f0, f2, f0
    c.f[0] = new.0;
    rider.off(0x230).write_fpu(c, new);

    abi::ret(c);
}

/// `char *Trick_GetGrabName(int grab)` (0x116938): look up the grab-name
/// pointer in the 4-byte-entry table based at 0x43d160 (0x440000 - 0x2ea0).
pub fn trick_get_grab_name(c: &mut Cpu) {
    c.cycles += 6;
    let index4 = abi::arg(c, 0).wrapping_shl(2);
    c.g[V0][0] = sx(0x44_0000u32.wrapping_sub(0x2ea0)); // table base
    let slot = Addr(index4.wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx(slot.0);
    c.g[V0][0] = slot.lw(c);
    abi::ret(c);
}

/// `cHUDFX *cHUDFX_ctor(cHUDFX *this)` (0x116fa8): set the state word at +0 to
/// 0x34 and return `this`.
pub fn chudfx_ctor(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = this.0 as u64;
    c.g[V1][0] = 0x34;
    Addr(c.g[V0][0] as u32).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `char *Rider_GetMainStateName(Rider *rider)` (0x11fea8): tail-call
/// `RiderBase_GetMainStateName` (0x111830) on the base object at
/// `rider+0x77c`.
pub fn rider_get_main_state_name(c: &mut Cpu) {
    call::tail_wrapper(
        c,
        Some(0x77c),
        call::Target::Direct(Callee { addr: 0x0011_1830, f: ssx3_ee::s00::RiderBase_GetMainStateName }),
        0x0011_feb8,
    );
}

/// `void RiderMetrix_SetBaseScore(RiderMetrix *m, int base)` (0x117520):
/// `delta = base - m->base (+0x1c8)`, store the new base, and add `delta` to
/// the running total at +0x198.
pub fn rider_metrix_set_base_score(c: &mut Cpu) {
    c.cycles += 7;
    let m = Addr(abi::arg(c, 0));
    let old_base = m.off(0x1c8).lw(c);
    c.g[V0][0] = old_base;
    let total = m.off(0x198).lw(c);
    c.g[V1][0] = total;
    let delta = abi::arg(c, 1).wrapping_sub(old_base as u32);
    c.g[V0][0] = sx(delta);
    m.off(0x1c8).sw(c, c.g[A1][0]);
    let new_total = (total as u32).wrapping_add(delta);
    c.g[V1][0] = sx(new_total);
    m.off(0x198).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `int RiderMetrix_GetRoundedTrickPoints(RiderMetrix *m)` (0x117948): compute
/// `round_down((m->pts (+0x1c4) * m->scale (+0x14) * k + 5.0), 10)`, the
/// division being a signed 32-bit `div` whose remainder is subtracted.
pub fn rider_metrix_get_rounded_trick_points(c: &mut Cpu) {
    let m = Addr(abi::arg(c, 0));

    c.cycles += 5;
    let scale = m.off(0x14).read_fpu(c); // f2
    c.f[2] = scale.0;
    c.g[A1][0] = 0xa;
    let points = m.off(0x1c4).read_fpu(c); // f0
    c.f[0] = points.0;
    // (the original's `a1 == 0` guard is dead: a1 is 10)

    c.cycles += 12;
    let k = Addr((c.g[GP][0] as u32).wrapping_sub(0x7b08)).read_fpu(c); // f3
    c.f[3] = k.0;
    let scaled = points * scale;
    c.f[0] = scaled.0;
    c.g[AT][0] = 0x40a0_0000; // 5.0
    c.f[1] = c.g[AT][0] as u32; // mtc1 f1, at
    let scaled = scaled * k;
    c.f[0] = scaled.0;
    let biased = scaled + FpuF32(c.f[1]); // add.s f0, f0, f1
    c.f[0] = biased.0;

    let v = biased.to_i32(); // cvt.w.s f1, f0
    c.f[1] = v as u32;
    let vx = sx(v as u32); // mfc1 v0, f1
    c.g[V0][0] = vx;

    let (_, rem) = abi::div(c, v as u32, 10); // div v0, a1
    c.g[V1][0] = sx(rem);
    c.g[V0][0] = sx((vx as u32).wrapping_sub(rem));
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn air_predictor_reset_matches() {
        check(ssx3_ee::s00::f_00113198, air_predictor_reset, 200, &|c, seed| {
            let mut r = Rng(0x1234_5678 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            // the three constant sources the body copies from.
            for i in 0..4 {
                c.bus.write32(0x4ff160 + i * 4, r.next_u32());
                c.bus.write32(0x4ff120 + i * 4, r.next_u32());
            }
            for i in 0..2 {
                c.bus.write32(0x4a5960 + i * 4, r.next_u32());
            }
            // pre-fill the object so the unaligned stores are meaningful.
            for i in 0..16 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn air_predictor_prime_matches() {
        check(ssx3_ee::s00::f_001135b8, air_predictor_prime, 200, &|c, seed| {
            let mut r = Rng(0x1357_9bdf ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            c.g[A2][0] = (DATA + 0x300) as u64;
            c.f[12] = r.edge_f32();
            for i in 0..64 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
            for i in 0..8 {
                c.bus.write32(DATA + 0x200 + i * 4, r.next_u32());
                c.bus.write32(DATA + 0x300 + i * 4, r.next_u32());
            }
            for i in 0..4 {
                c.bus.write32(0x4ff160 + i * 4, r.next_u32());
                c.bus.write32(0x4ff120 + i * 4, r.next_u32());
            }
            for i in 0..2 {
                c.bus.write32(0x4a5960 + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn race_state_game_init_update_matches() {
        check(ssx3_ee::s00::f_00113cc0, race_state_game_init_update, 100, &|c, seed| {
            let mut r = Rng(0x3141_5926 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = DATA as u64;
            // state at +0 already equals the 2 the wrapper installs, so the
            // callee (AI_SetRaceState) short-circuits without dispatching.
            c.bus.write32(DATA, 2);
        });
    }

    #[test]
    fn race_state_freeride_update_matches() {
        check(ssx3_ee::s00::f_00113cf0, race_state_freeride_update, 100, &|c, seed| {
            let mut r = Rng(0x2718_2818 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = DATA as u64;
            c.bus.write32(DATA, 5);
        });
    }

    #[test]
    fn race_state_countdown_update_matches() {
        check(ssx3_ee::s00::f_00113d48, race_state_countdown_update, 200, &|c, seed| {
            let mut r = Rng(0x1618_0339 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = DATA as u64;
            // Half the seeds tick a live countdown, half expire into the
            // AI_SetRaceState(5) branch (which short-circuits).
            let countdown = if seed % 2 == 0 { 1 + r.next_u32() % 1000 } else { 0 };
            c.bus.write32(DATA + 0x1c, countdown);
            c.bus.write32(DATA, 5);
        });
    }

    #[test]
    fn rider_set_slew5_target_matches() {
        check(ssx3_ee::s00::f_00113f38, rider_set_slew5_target, 300, &|c, seed| {
            let mut r = Rng(0x2233_4455 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x22c, r.edge_f32());
            c.f[12] = r.edge_f32();
            // Alternate a small cap (compare true -> min.s path) and a huge one
            // (compare false -> cap path) so both branches are taken.
            let cap = if seed % 2 == 0 {
                r.f32(1.0)
            } else {
                0x4f00_0000 | (r.next_u32() & 0x007f_ffff)
            };
            c.bus.write32(GAME_GP - 0x7c28, cap);
            c.bus.write32(GAME_GP - 0x7c24, r.edge_f32());
            c.bus.write32(DATA + 0x230, r.next_u32());
            c.bus.write32(DATA + 0x234, r.next_u32());
        });
    }

    #[test]
    fn trick_get_grab_name_matches() {
        check(ssx3_ee::s00::f_00116938, trick_get_grab_name, 200, &|c, seed| {
            let mut r = Rng(0x3344_5566 ^ seed as u64);
            scramble(c, &mut r);
            // Low 32 bits index the table; the upper half is junk (ignored).
            let idx = (seed % 64) as u32;
            c.g[A0][0] = idx as u64 | ((r.next_u32() as u64) << 32);
            let base = 0x43d160u32;
            for i in 0..64 {
                c.bus.write32(base + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn chudfx_ctor_matches() {
        check(ssx3_ee::s00::f_00116fa8, chudfx_ctor, 50, &|c, seed| {
            let mut r = Rng(0x4455_6677 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn rider_get_main_state_name_matches() {
        check(ssx3_ee::s00::f_0011fea8, rider_get_main_state_name, 200, &|c, seed| {
            let mut r = Rng(0x2468_ace0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let base = DATA + 0x1000;
            c.bus.write32(DATA + 0x77c, base);
            // Cover both branches of the callee's range test (< 7 vs >= 7).
            let state = if seed % 2 == 0 {
                r.next_u32() % 7
            } else {
                7 + r.next_u32() % 8
            };
            c.bus.write32(base + 0xde0, state);
            // name tables the callee indexes.
            for i in 0..8 {
                c.bus.write32(0x459d70 + i * 4, r.next_u32());
                c.bus.write32(0x456c48 + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn rider_metrix_set_base_score_matches() {
        check(ssx3_ee::s00::f_00117520, rider_metrix_set_base_score, 200, &|c, seed| {
            let mut r = Rng(0x5566_7788 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(DATA + 0x1c8, r.next_u32());
            c.bus.write32(DATA + 0x198, r.next_u32());
        });
    }

    #[test]
    fn rider_metrix_get_rounded_trick_points_matches() {
        check_lohi(ssx3_ee::s00::f_00117948, rider_metrix_get_rounded_trick_points, 300, &|c, seed| {
            let mut r = Rng(0x6677_8899 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x14, r.edge_f32());
            c.bus.write32(DATA + 0x1c4, r.edge_f32());
            c.bus.write32(GAME_GP - 0x7b08, r.edge_f32());
        });
    }
}
