//! `src/ai/airpredictor.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::rider::fcc_lt;
pub(crate) use crate::game::ai::rider::fr_3 as fr;
pub(crate) use crate::game::ai::rider::gp_lit;
pub(crate) use crate::game::ai::rider::set_fr;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_3198, air_predictor_reset, "AirPredictor_Reset"),
    (0x0011_35b8, air_predictor_prime, "AirPredictor_Prime"),
    (0x0011_3648, air_predictor_update, "AirPredictor_Update"),
    (
        0x0011_3998,
        air_predictor_clear_air_time,
        "AirPredictor_ClearAirTime",
    ),
    (
        0x0011_3aa0,
        air_predictor_save_state,
        "AirPredictor_SaveState",
    ),
    (
        0x0011_3ad8,
        air_predictor_load_state,
        "AirPredictor_LoadState",
    ),
    (0x0011_39a0, air_predictor_step_2, "AirPredictor_Step"),
    (0x0011_3200, air_predictor_simulate, "AirPredictor_Simulate"),
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
        let p = if i == 3 {
            1.0
        } else {
            n_pos[i] * k + s_pos[i] * (1.0 - k)
        };
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
    let g = if vel[2] > 0.0 {
        -14.166_667
    } else {
        -31.666_668
    };
    let mut v = [
        vel[0] + vel[0] * -0.003_333_333_6,
        vel[1] + vel[1] * -0.003_333_333_6,
        vel[2] + g,
        vel[3],
    ];
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

        Callee {
            addr: 0x0011_3198,
            f: ssx3_ee::s00::AirPredictor_Reset,
        }
        .call(c, 0x0011_35e4)?;
        Addr(c.g[S0][0] as u32)
            .off(0xa8)
            .write_fpu(c, FpuF32(c.f[20]));

        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        Callee {
            addr: 0x0011_3618,
            f: ssx3_ee::s00::AirPredictor_SetLaunchState,
        }
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

// ---------------------------------------------------------------------------
// AirPredictor_Step (0x001139a0)
// ---------------------------------------------------------------------------

/// `.lit4` constants (gp-relative, subtracted): seconds per tick (1/60),
/// the per-tick horizontal drag (-0.2/60), the gravity step falling
/// (-1900/60) and rising (-850/60).
pub(crate) const GP_AIR_STEP: u32 = 0x7c50;

pub(crate) const GP_AIR_DRAG: u32 = 0x7c4c;

pub(crate) const GP_AIR_GRAVITY_FALLING: u32 = 0x7c48;

pub(crate) const GP_AIR_GRAVITY_RISING: u32 = 0x7c44;

/// `AirPredictor.maxSpeed`.
pub(crate) const AIRP_MAX_SPEED: u32 = 0xa8;

/// `vf5 = vf4 * f` with `f` passed through `v0` (`mfc1`/`qmtc2` into vf3).
pub(crate) fn air_scale_vu0(c: &mut Cpu, v: u32, out: u32) {
    c.mfc1(V0, 0);
    c.lqc2(4, v);
    c.qmtc2(V0, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, out);
}

/// `float AirPredictor_Step(AirPredictor* self, const Vec4* pos,
/// const Vec4* vel, Vec4* outPos, Vec4* outVel)` (0x001139a0): one tick of
/// the airborne path prediction: `outPos = pos + vel / 60`; `outVel = vel`
/// with horizontal drag and gravity (weaker while rising), clamped to the
/// predictor's maximum speed. Returns the new speed (f0).
pub fn air_predictor_step_2(c: &mut Cpu) {
    c.cycles += 27;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
    let s = sp(c);
    let this = r32(c, A0);
    let pos = r32(c, A1);
    let vel = r32(c, A2);
    let out_pos = r32(c, A3);
    let out_vel = r32(c, T0);
    let step = gp_lit(c, GP_AIR_STEP);
    set_fr(c, 0, step);
    air_scale_vu0(c, vel, s + 0x20);
    set_fr(c, 5, FpuF32::ZERO);
    c.lq(V0, s + 0x20);
    let drag = gp_lit(c, GP_AIR_DRAG);
    set_fr(c, 3, drag);
    c.sq(V0, s + 0x10);
    // outPos = pos + vel * step
    c.lqc2(3, pos);
    c.lqc2(4, s + 0x10);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s + 0x20);
    let g = gp_lit(c, GP_AIR_GRAVITY_FALLING);
    set_fr(c, 4, g);
    c.lq(V0, s + 0x20);
    c.sq(V0, out_pos);
    c.sq(V0, s);
    let vx = Addr(vel).read_fpu(c);
    let vz = Addr(vel + 8).read_fpu(c);
    let vy = Addr(vel + 4).read_fpu(c);
    set_fr(c, 0, vz);
    set_fr(c, 2, vx * drag);
    let rising = fcc_lt(c, FpuF32::ZERO, vz);
    set_fr(c, 1, vy * drag);
    if rising {
        c.cycles += 1;
        let g = gp_lit(c, GP_AIR_GRAVITY_RISING);
        set_fr(c, 4, g);
    }

    // 0x00113a10: outVel = vel + (drag x, drag y, gravity, 0).
    c.cycles += 25;
    Addr(s + 0x10).write_fpu(c, fr(c, 2));
    Addr(s + 0x14).write_fpu(c, fr(c, 1));
    Addr(s + 0x18).write_fpu(c, fr(c, 4));
    Addr(s + 0x1c).write_fpu(c, fr(c, 5));
    c.lqc2(3, vel);
    c.lqc2(4, s + 0x10);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s + 0x20);
    c.lq(V0, s + 0x20);
    c.sq(V0, out_vel);
    c.sq(V0, s);
    // |outVel| (xyzw) on VU0.
    c.lqc2(3, out_vel);
    {
        let v = c.vu0();
        v.add(0b1000, 6, 0, Src::Bc(0, W));
        v.mul(0b1111, 4, 3, Src::Vec(3));
        v.adda(0b1000, 4, Src::Bc(4, Y));
        v.madda(0b1000, 6, Src::Bc(4, Z));
        v.madd(0b1000, 4, 6, Src::Bc(4, W));
        v.sqrt(4, 0);
        v.waitq();
    }
    c.cfc2(V1, 22);
    c.mtc1(V1, 0);
    let speed = fr(c, 0);
    let max = Addr(this + AIRP_MAX_SPEED).read_fpu(c);
    set_fr(c, 1, max);
    if fcc_lt(c, max, speed) {
        c.cycles += 7;
        set_fr(c, 0, max.div(speed));
        air_scale_vu0(c, out_vel, out_vel);
        set_fr(c, 0, max);
    }
    c.cycles += 2;
    c.g[SP][0] = sx(s.wrapping_add(0x30));
    abi::ret(c);
}

const GP_FORECAST_DISTANCE: u32 = 0x7c74;
const GP_FORECAST_STEP: u32 = 0x7c70;
const GP_FORECAST_HEADING: u32 = 0x7c6c;
const GP_FORECAST_EXTENSION: u32 = 0x7c68;
const GP_FORECAST_BACKTRACK: u32 = 0x7c64;

/// `void AirPredictor_Simulate(AirPredictor *ap)` (0x113200): extend the
/// fixed-step flight forecast until its distance/heading limit, then sweep
/// the segment and backtrack its predicted time and velocity to a contact.
/// The forecast step and contact backtrack are separate gp-relative loads;
/// both are `tick::NOT_A_STEP`.
pub fn air_predictor_simulate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        let frame = Frame::open(c, 0x280);
        c.f[1] = (gp_lit(c, GP_FORECAST_DISTANCE)).0; // forecast distance budget
        frame.save(c, S0, 0x240);
        frame.save(c, S1, 0x230);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x220);
        c.g[S1][0] = sx(r32(c, S0).wrapping_add(0x60)); // velocity
        frame.save(c, S3, 0x210);
        c.g[S2][0] = sx(r32(c, S0).wrapping_add(0x50)); // position
        frame.save(c, S4, 0x200);
        c.g[S3][0] = sx(frame.at(c, 0xf0)); // segment query
        frame.save(c, S5, 0x1f0);
        c.g[S4][0] = sx(frame.at(c, 0x70)); // contact result
        frame.save(c, S6, 0x1e0);
        c.g[S5][0] = sx(frame.at(c, 0x40)); // extended end
        frame.save_fpr(c, 25, 0x278);
        c.g[S6][0] = sx(frame.at(c, 0x30)); // extended start
        for (f, off) in [(24, 0x270), (23, 0x268), (22, 0x260), (21, 0x258)] {
            frame.save_fpr(c, f, off);
        }
        frame.save_ra(c, 0x1d0);
        frame.save_fpr(c, 20, 0x250);
        c.f[21] = (FpuF32::ZERO).0; // internal steps, not game ticks
        c.lq(V0, r32(c, S0) + 0x50);
        c.lq(V1, r32(c, S0) + 0x60);
        c.sq(V0, sp(c));
        c.sq(V1, sp(c) + 0x10);
        // Initial direction, with the original VU0 reduction order.
        c.lqc2(3, r32(c, S0) + 0x60);
        {
            let v = c.vu0();
            v.add(0b1000, 6, 0, Src::Bc(0, W));
            v.mul(0b1111, 4, 3, Src::Vec(3));
            v.adda(0b1000, 4, Src::Bc(4, Y));
            v.madda(0b1000, 6, Src::Bc(4, Z));
            v.madd(0b1000, 4, 6, Src::Bc(4, W));
            v.rsqrt(0, 3, 4, 0);
            v.waitq();
            v.mul(0b1111, 5, 3, Src::Q);
        }
        c.sqc2(5, sp(c) + 0x30);
        c.f[25] = (gp_lit(c, GP_FORECAST_STEP)).0; // forecast quantum
        c.f[0] = (Addr(r32(c, S0) + AIRP_MAX_SPEED).read_fpu(c)).0;
        c.lq(V0, sp(c) + 0x30);
        c.f[22] = (fr(c, 1).div(fr(c, 0))).0;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 24);
        c.sq(V0, sp(c) + 0x20);
        c.f[23] = (gp_lit(c, GP_FORECAST_HEADING)).0; // heading cosine limit
        c.f[0] = (Addr(r32(c, S0) + 0x98).read_fpu(c)).0;
        loop {
            c.cycles += 15;
            c.g[A0][0] = c.g[S0][0];
            c.f[20] = (Addr(r32(c, S0) + 0x58).read_fpu(c)).0;
            c.g[A1][0] = c.g[S2][0];
            c.f[0] = (fr(c, 0) + fr(c, 25)).0;
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S2][0];
            c.g[T0][0] = c.g[S1][0];
            c.f[21] = (fr(c, 21) + fr(c, 24)).0;
            Addr(r32(c, S0) + 0x98).write_fpu(c, fr(c, 0));
            Callee {
                addr: 0x0011_39a0,
                f: ee::s00::AirPredictor_Step,
            }
            .call(c, 0x0011_32e8)?;
            c.f[1] = (fr(c, 0)).0;
            c.f[0] = (Addr(r32(c, S0) + 0x58).read_fpu(c)).0;
            let rising = fr(c, 20).le(fr(c, 0));
            abi::set_fcc(c, rising);
            if rising {
                c.cycles += 4;
                c.f[0] = (Addr(r32(c, S0) + 0x98).read_fpu(c)).0;
                c.lq(V0, r32(c, S0) + 0x50);
                Addr(r32(c, S0) + 0x9c).write_fpu(c, fr(c, 0));
                c.sq(V0, r32(c, S0) + 0x40);
            }
            c.cycles += 4;
            if fcc_lt(c, fr(c, 22), fr(c, 21)) {
                c.lq(V1, r32(c, S0) + 0x50);
                break;
            }
            c.cycles += 14;
            c.f[0] = (fr(c, 1) * fr(c, 23)).0;
            c.lqc2(3, sp(c) + 0x20);
            c.lqc2(5, r32(c, S0) + 0x60);
            simulate_dot(c);
            c.qmfc2(V0, 4);
            c.mtc1(V0, 1);
            if fcc_lt(c, fr(c, 1), fr(c, 0)) {
                c.cycles += 1;
                c.lq(V1, r32(c, S0) + 0x50);
                break;
            }
            c.f[0] = (Addr(r32(c, S0) + 0x98).read_fpu(c)).0;
            if event_due(c) {
                deliver_event_here(c, 0x0011_32c0);
            }
        }
        // Extend both ends of the sweep by the same fraction of its delta.
        c.cycles += 34;
        c.g[A1][0] = c.g[S5][0];
        c.lq(V0, sp(c));
        c.g[A2][0] = c.g[S6][0];
        c.sq(V1, sp(c) + 0x40);
        c.g[A0][0] = c.g[S3][0];
        c.sq(V0, sp(c) + 0x30);
        c.g[A3][0] = 1;
        c.lqc2(3, sp(c) + 0x40);
        c.lqc2(4, sp(c) + 0x30);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp(c) + 0x60);
        c.f[0] = (gp_lit(c, GP_FORECAST_EXTENSION)).0;
        c.lq(V0, sp(c) + 0x60);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 12);
        c.sq(V0, sp(c) + 0x50);
        c.mfc1(V1, 0);
        c.lqc2(4, sp(c) + 0x50);
        c.qmtc2(V1, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp(c) + 0x50);
        c.lqc2(3, sp(c) + 0x30);
        c.lqc2(4, sp(c) + 0x50);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp(c) + 0x30);
        c.lqc2(3, sp(c) + 0x40);
        c.lqc2(4, sp(c) + 0x50);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp(c) + 0x40);
        call_ptr(c, 0x0032_e100, 0x0011_33d4)?;
        c.g[V1][0] = Addr(r32(c, S0) + 0xac).lw(c);
        let active = c.g[V1][0] != 0;
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        c.cycles += if active { 7 } else { 8 };
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S4][0];
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(r32(c, V1) + 0x84).lw(c);
        c.g[A0][0] = Addr(r32(c, V0) + 0x20).lw(c);
        if active {
            call_ptr(c, 0x0033_78c0, 0x0011_3418)?;
        } else {
            call_ptr(c, 0x0033_6850, 0x0011_33f8)?;
        }
        c.f[3] = (fr(c, 0)).0;
        c.cycles += 5;
        c.f[1] = (FpuF32::ZERO).0;
        let hit = fr(c, 1).le(fr(c, 3));
        abi::set_fcc(c, hit);
        if hit {
            c.cycles += 13;
            c.lqc2(3, sp(c) + 0x50);
            c.lqc2(5, sp(c) + 0x80);
            simulate_dot(c);
            c.qmfc2(V0, 4);
            c.mtc1(V0, 0);
            let approaching = fcc_lt(c, fr(c, 0), fr(c, 1));
            c.g[A0][0] = 2;
            if approaching {
                c.cycles += 34;
                c.f[0] = (gp_lit(c, GP_FORECAST_BACKTRACK)).0; // contact time backtrack
                c.f[2] = (Addr(r32(c, S0) + 0x98).read_fpu(c)).0;
                c.f[0] = (fr(c, 3) * fr(c, 0)).0;
                c.lq(V0, sp(c) + 0x70);
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 1);
                c.sq(V0, r32(c, S0));
                c.f[0] = (fr(c, 21) * fr(c, 0)).0;
                c.f[1] = (fr(c, 1) - fr(c, 3)).0;
                c.f[2] = (fr(c, 2) - fr(c, 0)).0;
                Addr(r32(c, S0) + 0x98).write_fpu(c, fr(c, 2));
                c.mfc1(V1, 3);
                c.lqc2(4, sp(c) + 0x10);
                c.qmtc2(V1, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp(c) + 0x1b0);
                c.lq(V0, sp(c) + 0x1b0);
                c.sq(V0, sp(c) + 0x1a0);
                c.mfc1(V1, 1);
                c.lqc2(4, r32(c, S0) + 0x60);
                c.qmtc2(V1, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp(c) + 0x1c0);
                c.lq(V0, sp(c) + 0x1c0);
                c.sq(V0, sp(c) + 0x1b0);
                c.lqc2(3, sp(c) + 0x1a0);
                c.lqc2(4, sp(c) + 0x1b0);
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c) + 0x1c0);
                c.lq(V0, sp(c) + 0x1c0);
                c.g[V1][0] = Addr(r32(c, S0) + 0xac).lw(c);
                c.sq(V0, r32(c, S0) + 0x10);
                let mode_two = c.g[V1][0] == c.g[A0][0];
                c.sq(V0, sp(c) + 0x60);
                if mode_two {
                    c.cycles += 9;
                    c.g[V1][0] = 3;
                    c.g[V0][0] = 0x50_0000;
                    Addr(r32(c, S0) + 0xac).sw(c, c.g[V1][0]);
                    c.g[V0][0] = sx(0x4f_f160);
                    c.g[A0][0] = 13;
                    c.lq(V1, r32(c, V0));
                    Addr(r32(c, S0) + 0x90).sw(c, c.g[A0][0]);
                    c.sq(V1, r32(c, S0) + 0x20);
                } else {
                    c.cycles += 6;
                    c.lq(V0, sp(c) + 0x80);
                    c.g[V1][0] = 1;
                    c.g[A0][0] = Addr(sp(c) + 0xbc).lw(c);
                    Addr(r32(c, S0) + 0xac).sw(c, c.g[V1][0]);
                    c.sq(V0, r32(c, S0) + 0x20);
                    Addr(r32(c, S0) + 0x90).sw(c, c.g[A0][0]);
                }
                c.cycles += 3;
                c.g[A0][0] = Addr(sp(c) + 0xc4).lw(c);
                let has_object = c.g[A0][0] != 0;
                c.g[V0][0] = 1;
                let mut object_contact = false;
                if has_object {
                    c.cycles += 3;
                    c.g[V1][0] = Addr(r32(c, S0) + 0xac).lw(c);
                    object_contact = c.g[V1][0] == c.g[V0][0];
                    c.f[1] = (Addr(sp(c) + 0xdc).read_fpu(c)).0;
                }
                if object_contact {
                    c.cycles += 8;
                    c.g[V0][0] = Addr(r32(c, A0) + 0xa).lhu(c);
                    Addr(r32(c, S0) + 0x94).sh(c, c.g[V0][0]);
                    c.f[0] = (Addr(sp(c) + 0xe0).read_fpu(c)).0;
                    c.g[V0][0] = Addr(r32(c, A0) + 0x150).lw(c);
                    Addr(r32(c, S0) + 0x38).write_fpu(c, fr(c, 0));
                    Addr(r32(c, S0) + 0x30).sw(c, c.g[V0][0]);
                    Addr(r32(c, S0) + 0x34).write_fpu(c, fr(c, 1));
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = u64::MAX;
                    c.g[V1][0] = u64::MAX;
                    Addr(r32(c, S0) + 0x94).sh(c, c.g[V1][0]);
                    Addr(r32(c, S0) + 0x30).sw(c, c.g[V0][0]);
                }
            }
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x240);
        c.cycles += 15;
        for (r, off) in [
            (S1, 0x230),
            (S2, 0x220),
            (S3, 0x210),
            (S4, 0x200),
            (S5, 0x1f0),
            (S6, 0x1e0),
        ] {
            frame.restore(c, r, off);
        }
        frame.restore_ra(c, 0x1d0);
        for (f, off) in [
            (25, 0x278),
            (24, 0x270),
            (23, 0x268),
            (22, 0x260),
            (21, 0x258),
            (20, 0x250),
        ] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}

/// Dot of vf3 and vf5, leaving the original reduction temporaries in VU0.
fn simulate_dot(c: &mut Cpu) {
    let v = c.vu0();
    v.add(0b1000, 6, 0, Src::Bc(0, W));
    v.mul(0b1111, 4, 3, Src::Vec(5));
    v.adda(0b1000, 4, Src::Bc(4, Y));
    v.madda(0b1000, 6, Src::Bc(4, Z));
    v.madd(0b1000, 4, 6, Src::Bc(4, W));
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::elf_image;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_UNWIND_AT;

    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::leaf;

    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;

    /// Only the object's virtual collision delegate is fake. The predictor,
    /// segment constructor, grid traversal and closest-contact query run real.
    fn simulate_collision(c: &mut Cpu, target: u32) {
        use crate::difftest::fakes::{fake_call, FAKE};
        let out = r32(c, A2);
        let case = c.bus.read32(DATA + 0x300);
        fake_call(c, target);
        if c.pc & 0xffff_0000 == 0xdead_0000 {
            return;
        }
        if target == FAKE {
            c.g[V0][0] = 1;
            return;
        }
        assert_eq!(target, FAKE + 0x10);
        for off in (0..0x80).step_by(4) {
            c.bus.write32(out + off, 0);
        }
        let normal = if case / 16 % 2 == 0 { 1.0f32 } else { -1.0 };
        c.bus.write32(out + 0x10, normal.to_bits());
        c.bus.write32(out, 33.0f32.to_bits());
        c.bus.write32(
            out + 0x40,
            [0.0f32, 0.25, 1.0][(case / 4 % 3) as usize].to_bits(),
        );
        c.bus.write32(out + 0x4c, 7);
        c.bus
            .write32(out + 0x54, if case % 3 == 0 { 0 } else { DATA + 0x400 });
        c.bus.write32(out + 0x6c, 0.75f32.to_bits());
        c.bus.write32(out + 0x70, 0.125f32.to_bits());
        c.g[V0][0] = if case % 5 == 0 { 0 } else { 1 };
    }

    fn simulate_event(c: &mut Cpu) -> bool {
        // Structured back-edge event: preserve the thread and resume the loop.
        c.bus.write32(DATA + 0x1800, c.pc);
        c.bus.write64(DATA + 0x1808, c.cycles);
        for r in 0..32 {
            c.bus.write64(DATA + 0x1810 + 16 * r as u32, c.g[r][0]);
            c.bus.write64(DATA + 0x1818 + 16 * r as u32, c.g[r][1]);
        }
        c.next_event = u64::MAX;
        false
    }

    fn simulate_fixture(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::FAKE;
        elf_image(c);
        machine(c, seed, 0x1132_00ab, 0);
        predictor_pool(c);
        let case = seed;
        // Independently varied literal-pool inputs, not tick-rate coverage.
        for (address, value) in [
            (0x49b47c, [0.0f32, 10_000.0, 100_000.0][(case % 3) as usize]),
            (0x49b480, [0.01, 0.025, 0.04][(case / 3 % 3) as usize]),
            (0x49b484, [0.95, 0.9999, 1.01][(case / 9 % 3) as usize]),
            (0x49b488, [0.01, 0.03, 0.05][(case / 27 % 3) as usize]),
            (0x49b48c, [0.007, 0.02, 0.03][(case / 81 % 3) as usize]),
        ] {
            c.bus.write32(address, value.to_bits());
        }
        c.hooks.miss = simulate_collision;
        c.bus
            .write32(LOG_UNWIND_AT, if case % 11 == 10 { 1 } else { u32::MAX });
        for o in (0..0xb0).step_by(4) {
            c.bus.write32(DATA + o, 0);
        }
        c.bus.write32(DATA + 0x5c, 1.0f32.to_bits());
        c.bus.write32(DATA + 0x60, 5000.0f32.to_bits());
        c.bus.write32(
            DATA + 0x68,
            if case % 2 == 0 { 600.0f32 } else { -600.0f32 }.to_bits(),
        );
        c.bus.write32(DATA + 0xa8, 5000.0f32.to_bits());
        c.bus.write32(DATA + 0xac, case % 4);
        c.bus.write32(DATA + 0x300, case);
        c.bus.write16(DATA + 0x40a, 0x1234);
        c.bus.write32(DATA + 0x550, 0x8765_4321);
        c.bus.write32(GAME_GP - 0x848, DATA + 0x600);
        c.bus.write32(DATA + 0x684, DATA + 0x700);
        c.bus.write32(DATA + 0x720, DATA + 0x800);
        for (i, x) in [0u32, 0, 1.0f32.to_bits(), 0].into_iter().enumerate() {
            c.bus.write32(0x4ff160 + 4 * i as u32, x);
        }
        c.g[A0][0] = sx(DATA);
        c.pc = 0x0011_3200;
        if case % 2 == 0 {
            c.next_event = 0;
            c.hooks.event = simulate_event;
        }
        // One huge cell and one bounded object; all other cell trees are empty.
        let (grid, tree, object, sub, vt) = (
            DATA + 0x800,
            DATA + 0x900,
            DATA + 0xa00,
            DATA + 0xc00,
            DATA + 0xd00,
        );
        c.bus.write32(GAME_GP - 0x2dfc, 0.5f32.to_bits());
        c.bus.write32(grid, 20);
        c.bus.write32(grid + 0x10, tree);
        c.bus.write32(tree + 0x20, object);
        c.bus.write32(object + 8, 0x20);
        c.bus.write32(object + 0xc, sub);
        c.bus.write32(object + 0x88, DATA + 0xf00);
        for k in 0..3 {
            c.bus
                .write32(object + 0x60 + 4 * k, (-100_000.0f32).to_bits());
            c.bus.write32(object + 0x6c + 4 * k, 100_000.0f32.to_bits());
        }
        c.bus.write32(sub + 0xc, vt);
        c.bus.write32(vt + 0x134, FAKE);
        c.bus.write32(vt + 0x13c, FAKE + 0x10);
    }

    #[test]
    fn air_predictor_simulate_matches() {
        covered(
            ee::s00::f_00113200__generated,
            air_predictor_simulate,
            243,
            &simulate_fixture,
        );
        // Seeds 16/11: rise/fall, distance/heading exits, event/no event,
        // both grid queries and approaching/receding contacts. Seeds 18/22:
        // mode 2 -> 3 and object absent/present; seed 5: no contact.
        crate::difftest::require_paths(
            ee::s00::f_00113200__generated,
            5,
            &[1624, 2010, 1879, 3499, 1901],
            &|c, seed| simulate_fixture(c, [16, 11, 18, 5, 22][seed as usize]),
        );
    }

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
        crate::difftest::check(
            ee::s00::f_00113648__generated,
            air_predictor_update,
            64,
            &|c, seed| {
                let mut r = crate::difftest::Rng(0x5eed_3648 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                let ap = crate::difftest::DATA;
                for o in (0..0xb0).step_by(4) {
                    c.bus.write32(ap + o, 0);
                }
                // A flight in progress: state at +0x70/+0x80, timers in seconds.
                let f = |r: &mut crate::difftest::Rng, lo: f32, hi: f32| {
                    lo + (r.next_u32() % 1000) as f32 / 1000.0 * (hi - lo)
                };
                for i in 0..3 {
                    c.bus
                        .write32(ap + 0x70 + 4 * i, f(&mut r, -1000.0, 1000.0).to_bits());
                    c.bus
                        .write32(ap + 0x80 + 4 * i, f(&mut r, -500.0, 500.0).to_bits());
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
            },
        );
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
            assert!(
                zs.windows(2).all(|w| w[1] != w[0]),
                "{hz} Hz: height held for a tick"
            );
        }
    }

    #[test]
    fn air_predictor_reset_matches() {
        check(
            ssx3_ee::s00::f_00113198,
            air_predictor_reset,
            200,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn air_predictor_prime_matches() {
        check(
            ssx3_ee::s00::f_001135b8,
            air_predictor_prime,
            200,
            &|c, seed| {
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
            },
        );
    }

    // ------------------------------------- air predictor / algorithm lists

    #[test]
    fn air_predictor_state_match() {
        for (gen, hand, slot, salt) in [
            (
                ee::s00::f_00113aa0 as fn(&mut Cpu),
                air_predictor_save_state as fn(&mut Cpu),
                8u32,
                0x3aa0u64,
            ),
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
        leaf(
            ssx3_ee::s00::f_00113998__generated,
            air_predictor_clear_air_time,
            22,
        );
    }

    use crate::difftest::put_vec;

    /// `AirPredictor_Step`: random positions and velocities (rising and
    /// falling, edge floats), maximum speeds above and below the result.
    #[test]
    fn air_predictor_step_matches() {
        use crate::difftest::elf_image;
        check(
            ee::s00::f_001139a0__generated,
            air_predictor_step_2,
            300,
            &|c, seed| {
                let mut r = Rng(0x0011_39a0 ^ seed as u64);
                scramble(c, &mut r);
                elf_image(c);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x0011_39a0;
                let this = DATA;
                let pos = DATA + 0x100;
                let vel = DATA + 0x110;
                let out_pos = DATA + 0x200;
                let out_vel = DATA + 0x210;
                put_vec(
                    c,
                    pos,
                    [r.f32(5000.0), r.f32(5000.0), r.f32(5000.0), r.f32(2.0)],
                );
                let z = if seed % 9 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(60.0)
                };
                put_vec(c, vel, [r.f32(60.0), r.f32(60.0), z, r.f32(1.0)]);
                let max = if seed % 2 == 0 {
                    r.f32(30.0) & 0x7fff_ffff
                } else {
                    0x4479_c000
                };
                put_words(c, this + AIRP_MAX_SPEED, &[max]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(pos);
                c.g[A2][0] = sx(vel);
                c.g[A3][0] = sx(out_pos);
                c.g[T0][0] = sx(out_vel);
            },
        );
    }
}
