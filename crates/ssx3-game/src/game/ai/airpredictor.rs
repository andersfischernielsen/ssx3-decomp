//! `src/ai/airpredictor.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call;
use crate::call::Callee;
use crate::call::Frame;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::call_ptr;
use crate::call::Flow;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use crate::call::guest;
use crate::call::sp;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_3198, air_predictor_reset, "AirPredictor_Reset"),
    (0x0011_35b8, air_predictor_prime, "AirPredictor_Prime"),
    (0x0011_3648, air_predictor_update, "AirPredictor_Update"),
    (0x0011_3998, air_predictor_clear_air_time, "AirPredictor_ClearAirTime"),
    (0x0011_3aa0, air_predictor_save_state, "AirPredictor_SaveState"),
    (0x0011_3ad8, air_predictor_load_state, "AirPredictor_LoadState"),
];


/// `void AirPredictor_Update(AirPredictor *ap, Vec4 *pos, Vec4 *vel, float dt)`
/// (0x113648): advance the flight by `dt` in fixed 1/60 s steps
/// (`AirPredictor_Step`, 0x1139a0: gravity -31.67 or -14.17 and drag -0.2/60
/// per step, so the step size is not the tick step: its `.lit4` entries are
/// in `tick::NOT_A_STEP`), and write the state at the current time to
/// `pos`/`vel`: the last step, or between it and the next step when more than
/// 0.01 s is left over.
///
/// At 60 the time left over is always 0 and this is the generated body. Above
/// 60 it is often under 0.01 s (1/240, 2/240 at 240 Hz), where the original
/// would hold the last step for those ticks; the state is then interpolated
/// the same way, so the rider moves every tick.
pub fn air_predictor_update(c: &mut Cpu) {
    if crate::tick::rate(c) == 60 {
        ee::s00::f_00113648__generated(c);
        return;
    }
    let ap = abi::r32(c, A0);
    let pos = abi::r32(c, A1);
    let vel = abi::r32(c, A2);
    let ret = c.g[RA][0] as u32;
    ee::s00::f_00113648__generated(c);
    if c.pc != ret {
        return;
    }
    let f = |c: &mut Cpu, a: u32| f32::from_bits(c.bus.read32(a));
    let left = f(c, ap + 0xa0) - f(c, ap + 0xa4);
    if !(left > 0.0 && left <= 0.01) {
        return;
    }
    let s_pos: [f32; 4] = std::array::from_fn(|i| f(c, ap + 0x70 + 4 * i as u32));
    let s_vel: [f32; 4] = std::array::from_fn(|i| f(c, ap + 0x80 + 4 * i as u32));
    let (n_pos, n_vel) = air_predictor_step(s_pos, s_vel, f(c, ap + 0xa8));
    let k = left * 59.999996;
    for i in 0..4 {
        let p = if i == 3 { 1.0 } else { n_pos[i] * k + s_pos[i] * (1.0 - k) };
        let v = n_vel[i] * k + s_vel[i] * (1.0 - k);
        c.bus.write32(pos + 4 * i as u32, p.to_bits());
        c.bus.write32(vel + 4 * i as u32, v.to_bits());
    }
}

/// `AirPredictor_Step` (0x1139a0) on host floats: one 1/60 s step of the
/// position and velocity, the speed clamped to `max_speed` (`+0xa8`).
fn air_predictor_step(pos: [f32; 4], vel: [f32; 4], max_speed: f32) -> ([f32; 4], [f32; 4]) {
    const STEP: f32 = 0.016_666_668;
    let p = std::array::from_fn(|i| pos[i] + vel[i] * STEP);
    let g = if vel[2] > 0.0 { -14.166_667 } else { -31.666_668 };
    let mut v = [vel[0] + vel[0] * -0.003_333_333_6, vel[1] + vel[1] * -0.003_333_333_6, vel[2] + g, vel[3]];
    let len = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if max_speed < len {
        let k = max_speed / len;
        v.iter_mut().for_each(|x| *x *= k);
    }
    (p, v)
}


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


// ---------------------------------------------------------------------------
// Air predictor and algorithm lists.

/// `void AirPredictor_SaveState(AirPredictor *this, State *out)` (0x113aa0):
/// call `out->save` (`out+0 -> +8/+0xc`).
pub fn air_predictor_save_state(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = 0xb0;
        c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V0][0] as u32, 0x0011_3ac8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void AirPredictor_LoadState(AirPredictor *this, State *in)` (0x113ad8):
/// call `in->load` (`in+0 -> +0x10/+0x14`).
pub fn air_predictor_load_state(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = 0xb0;
        c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x10)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x14)).lw(c);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V0][0] as u32, 0x0011_3b00)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void AirPredictor_ClearAirTime(AirPredictor* p)` (0x113998): reset the
/// accumulated air time.
pub fn air_predictor_clear_air_time(c: &mut Cpu) {
    c.cycles += 2;
    let p = abi::arg(c, 0);
    Addr(p.wrapping_add(0x98)).sw(c, 0);
    abi::ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::elf_image;
    use crate::difftest::put_words;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;
    pub(crate) use crate::game::lib::sif::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::ai::ai::tests::rng;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::leaf;


    /// The predictor's `.lit4` constants (the test machine has no data image).
    fn predictor_pool(c: &mut Cpu) {
        c.g[GP][0] = sx(crate::difftest::GAME_GP);
        for (a, v) in [
            (0x0049_b47c, 0x4883_d600u32),
            (0x0049_b480, crate::tick::STEP_60),
            (0x0049_b484, 0x3f7d_70a4),
            (0x0049_b490, (-0.2f32).to_bits()),
            (0x0049_b494, crate::tick::STEP_60),
            (0x0049_b498, 0.01f32.to_bits()),
            (0x0049_b49c, 59.999996f32.to_bits()),
            (0x0049_b4a0, crate::tick::STEP_60),
            (0x0049_b4a4, (-0.0033333336f32).to_bits()),
            (0x0049_b4a8, (-31.666668f32).to_bits()),
            (0x0049_b4ac, 0xc162_aaab),
        ] {
            c.bus.write32(a, v);
        }
    }

    #[test]
    fn air_predictor_update_matches() {
        crate::difftest::check(ee::s00::f_00113648__generated, air_predictor_update, 64, &|c, seed| {
            let mut r = crate::difftest::Rng(0x5eed_3648 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            let ap = crate::difftest::DATA;
            for o in (0..0xb0).step_by(4) {
                c.bus.write32(ap + o, 0);
            }
            // A flight in progress: state at +0x70/+0x80, timers in seconds.
            let f = |r: &mut crate::difftest::Rng, lo: f32, hi: f32| lo + (r.next_u32() % 1000) as f32 / 1000.0 * (hi - lo);
            for i in 0..3 {
                c.bus.write32(ap + 0x70 + 4 * i, f(&mut r, -1000.0, 1000.0).to_bits());
                c.bus.write32(ap + 0x80 + 4 * i, f(&mut r, -500.0, 500.0).to_bits());
            }
            c.bus.write32(ap + 0x7c, 1.0f32.to_bits());
            c.bus.write32(ap + 0xa8, 2000.0f32.to_bits());
            c.bus.write32(ap + 0xac, 3);
            let t = (seed % 7) as f32 / 60.0;
            c.bus.write32(ap + 0xa0, t.to_bits());
            c.bus.write32(ap + 0xa4, t.to_bits());
            c.g[A0][0] = sx(ap);
            c.g[A1][0] = sx(ap + 0x100);
            c.g[A2][0] = sx(ap + 0x110);
            c.f[12] = (1.0f32 / 60.0).to_bits();
            predictor_pool(c);
        });
    }

    /// Above 60 Hz the flight follows the 60 Hz steps in real time: after one
    /// second (60, 120, 240 calls) the velocity is the one 60 steps give, and
    /// between steps the state moves every tick.
    #[test]
    fn air_predictor_update_rate() {
        let fly = |hz: u32| {
            let mut c = crate::difftest::fresh();
            crate::tick::set_for_test(&mut c, hz);
            predictor_pool(&mut c);
            let ap = crate::difftest::DATA;
            for o in (0..0xb0).step_by(4) {
                c.bus.write32(ap + o, 0);
            }
            c.bus.write32(ap + 0x7c, 1.0f32.to_bits());
            c.bus.write32(ap + 0x80, 100.0f32.to_bits());
            c.bus.write32(ap + 0x88, 600.0f32.to_bits());
            c.bus.write32(ap + 0xa8, 5000.0f32.to_bits());
            c.bus.write32(ap + 0xac, 3);
            let mut zs = Vec::new();
            for _ in 0..hz {
                c.g[A0][0] = sx(ap);
                c.g[A1][0] = sx(ap + 0x100);
                c.g[A2][0] = sx(ap + 0x110);
                c.g[RA][0] = crate::difftest::RA as u64;
                c.f[12] = (1.0f32 / hz as f32).to_bits();
                c.pc = 0x0011_3648;
                air_predictor_update(&mut c);
                zs.push(f32::from_bits(c.bus.read32(ap + 0x108)));
            }
            (f32::from_bits(c.bus.read32(ap + 0x118)), zs)
        };
        let (vz60, _) = fly(60);
        for hz in [120u32, 240] {
            let (vz, zs) = fly(hz);
            assert!((vz - vz60).abs() < 1.0, "{hz} Hz: vz {vz} vs {vz60}");
            assert!(zs.windows(2).all(|w| w[1] != w[0]), "{hz} Hz: height held for a tick");
        }
    }

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


    // ------------------------------------- air predictor / algorithm lists

    #[test]
    fn air_predictor_state_match() {
        for (gen, hand, slot, salt) in [
            (ee::s00::f_00113aa0 as fn(&mut Cpu), air_predictor_save_state as fn(&mut Cpu), 8u32, 0x3aa0u64),
            (ee::s00::f_00113ad8, air_predictor_load_state, 0x10, 0x3ad8),
        ] {
            covered(gen, hand, SEEDS, &|c, seed| {
                let mut r = machine(c, seed, salt, 1);
                let obj = ptr(&mut r);
                let vt = vtable(c, &mut r, &[slot]);
                c.bus.write32(obj, vt);
                c.g[A0][0] = ptr(&mut r) as u64;
                c.g[A1][0] = obj as u64;
            });
        }
    }


    #[test]
    fn air_predictor_clear_air_time_matches() {
        leaf(ssx3_ee::s00::f_00113998__generated, air_predictor_clear_air_time, 22);
    }
}
