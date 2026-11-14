//! Staging module oc363 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc363`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::ai::rider::{
    fcc, fr, gp, ldf, li_f, p, qcopy, setf, stf, v_add, v_cross, v_dot, v_length, v_normalize,
    v_scale, v_sub,
};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0011_4298, rider_launch_into_air, "Rider_LaunchIntoAir")];

// ---------------------------------------------------------------------------
// Rider_LaunchIntoAir: rider fields it touches.

/// `+0x110`: the position (nudged along the ground heading on flat take-offs).
pub(crate) const LAUNCH_POS: u32 = 0x110;
/// `+0x1c0`: the board's up vector.
pub(crate) const LAUNCH_UP: u32 = 0x1c0;
/// `+0x1e0`: the velocity (z at +0x1e8).
pub(crate) const LAUNCH_VEL: u32 = 0x1e0;
/// `+0x2d4` (halfword): flags; 0x20 keeps the speed on flat take-offs.
pub(crate) const LAUNCH_FLAGS: u32 = 0x2d4;
/// `+0x2e4`: the maximum speed.
pub(crate) const LAUNCH_MAX_SPEED: u32 = 0x2e4;
/// `+0x320`/`+0x324`: the stance words compared for `InterruptTrick`.
pub(crate) const LAUNCH_STANCE_A: u32 = 0x320;
pub(crate) const LAUNCH_STANCE_B: u32 = 0x324;
/// `+0x370`: the ground direction (vec4), its z (+0x378) the slope cosine.
pub(crate) const LAUNCH_DIR: u32 = 0x370;
/// `+0x380`: the ground heading (vec4), its z at +0x388.
pub(crate) const LAUNCH_HEADING: u32 = 0x380;
/// `+0x3a0`: the ground side vector (vec4), its z at +0x3a8.
pub(crate) const LAUNCH_SIDE: u32 = 0x3a0;
/// `+0x3c0`: the flattened, normalised heading written here.
pub(crate) const LAUNCH_FLAT_HEADING: u32 = 0x3c0;
/// `+0x434`: the sub state (11..13 skip the take-off damping).
pub(crate) const LAUNCH_SUB_STATE: u32 = 0x434;
/// `+0x5a4`: the forward air speed out; `+0x5a8`: launched with a push.
pub(crate) const LAUNCH_AIR_FORWARD: u32 = 0x5a4;
pub(crate) const LAUNCH_PUSHED: u32 = 0x5a8;
/// `+0x77c`: the owner (frame of the take-off at +0x10, jump flag +0x14).
pub(crate) const LAUNCH_OWNER: u32 = 0x77c;
/// `+0x790`: the rider's metrix.
pub(crate) const LAUNCH_METRIX: u32 = 0x790;
/// `+0x86c`: the rider id passed to the replay.
pub(crate) const LAUNCH_RIDER_ID: u32 = 0x86c;

/// `void Rider_LaunchIntoAir(Rider *r, float push)` (0x114298): the jump
/// impulse at take-off. A negative `push` is an ollie-less drop (the owner's
/// jump flag +0x14 = -1, a fixed small impulse); otherwise the impulse is
/// `push² * f(speed)` clamped to 361..708. On a rail (main state 4) it goes
/// along the board's up vector; otherwise along the ground direction plus a
/// fifth of the side vector, and on steep enough lips the velocity is also
/// steered toward the side by a slope factor. Then the replay, the world
/// script (jump event), the trick metrix (interrupt) and the boost meter are
/// told. On steep lips the backward part of the change is undone with speed;
/// on flat ground the lateral part of the velocity is removed (and slowed),
/// the speed is capped at the maximum, early take-offs are damped, and the
/// forward air speed (+0x5a4) and the pushed flag (+0x5a8) are stored.
pub fn rider_launch_into_air(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x130);
        setf(c, 0, FpuF32(0));
        frame.save(c, S0, 0xe0);
        frame.save(c, S1, 0xd0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 26, 0x120);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0xc0);
        let push = setf(c, 26, fr(c, 12));
        frame.save_fpr(c, 27, 0x128);
        let negative = fcc(c, push.lt(fr(c, 0)));
        for (f, off) in [
            (25, 0x118),
            (24, 0x110),
            (23, 0x108),
            (22, 0x100),
            (21, 0xf8),
            (20, 0xf0),
        ] {
            frame.save_fpr(c, f, off);
        }
        let r = r32(c, S0);
        let s = sp(c);
        // f23 = |velocity|; the old velocity at sp.
        v_length(c, p(r, LAUNCH_VEL), V0, 23);
        qcopy(c, V0, p(r, LAUNCH_VEL), s);

        if negative {
            c.cycles += 5;
            c.g[V1][0] = c.lw(p(r, LAUNCH_OWNER));
            c.g[V0][0] = u64::MAX;
            ldf(c, 21, gp(c, -0x7bf0));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        } else {
            // f21 = push² * (speed < 972 ? speed * k + 361 : 708), clamped.
            c.cycles += 5;
            let knee = ldf(c, 0, gp(c, -0x7bec));
            let slow = fcc(c, fr(c, 23).lt(knee));
            setf(c, 21, push * push);
            if slow {
                c.cycles += 5;
                let k = ldf(c, 0, gp(c, -0x7be8));
                let base = ldf(c, 1, gp(c, -0x7be4));
                let v = fr(c, 23) * k;
                setf(c, 0, v + base);
            } else {
                c.cycles += 1;
                ldf(c, 0, gp(c, -0x7be0));
            }
            c.cycles += 7;
            let scaled = setf(c, 21, fr(c, 21) * fr(c, 0));
            let lo = ldf(c, 0, gp(c, -0x7bdc));
            let hi = ldf(c, 1, gp(c, -0x7bd8));
            if fcc(c, lo.le(scaled)) {
                setf(c, 0, scaled.min(hi));
            }
            c.cycles += 1;
            setf(c, 21, fr(c, 0));
        }

        // f20 = cos(lo angle), f27 = cos(hi angle).
        c.cycles += 12;
        ldf(c, 12, gp(c, -0x7bd4));
        call_ptr(c, 0x0031_c040, 0x0011_437c)?;
        ldf(c, 12, gp(c, -0x7bd0));
        setf(c, 20, fr(c, 0));
        call_ptr(c, 0x0031_c040, 0x0011_4388)?;
        setf(c, 27, fr(c, 0));
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x0011_fe98,
            f: ee::s00::Rider_GetMainState,
        }
        .call(c, 0x0011_4394)?;
        c.g[V1][0] = 0x4;
        let r = r32(c, S0);
        let s = sp(c);
        if c.g[V0][0] == c.g[V1][0] {
            // On a rail: velocity += up * f21.
            c.cycles += 15;
            qcopy(c, V0, p(r, LAUNCH_UP), p(s, 0x20));
            v_scale(c, V1, 21, p(s, 0x20), p(s, 0x30));
            qcopy(c, V0, p(s, 0x30), p(s, 0x10));
            v_add(c, p(r, LAUNCH_VEL), p(s, 0x10), p(r, LAUNCH_VEL));
            c.g[V0][0] = c.lw(gp(c, -0x848));
        } else {
            launch_ground_push(c)?;
        }

        // Tell the replay, the world script, the metrix and the boost meter.
        c.cycles += 26;
        let r = r32(c, S0);
        c.g[A2][0] = c.lw(p(r, LAUNCH_METRIX));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[A1][0] = c.lw(p(r, LAUNCH_RIDER_ID));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xfc));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
        call_ptr(c, 0x0027_07e0, 0x0011_47b8)?;
        c.g[A0][0] = c.lw(gp(c, 0xce8));
        call_ptr(c, 0x0030_9848, 0x0011_47c0)?;
        let r = r32(c, S0);
        c.g[V0][0] = c.lw(p(r, LAUNCH_STANCE_B));
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = c.lw(p(r, LAUNCH_STANCE_A));
        c.g[A0][0] = c.lw(p(r, LAUNCH_METRIX));
        c.g[A1][0] ^= c.g[V0][0];
        c.g[A1][0] = (0u64 < c.g[A1][0]) as u64;
        Callee {
            addr: 0x0011_9e38,
            f: ee::s00::RiderMetrix_InterruptTrick,
        }
        .call(c, 0x0011_47dc)?;
        setf(c, 12, fr(c, 0));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x1;
        Callee {
            addr: 0x0010_e098,
            f: ee::s00::Rider_AddBoost,
        }
        .call(c, 0x0011_47ec)?;

        let r = r32(c, S0);
        let s = sp(c);
        let margin = ldf(c, 0, gp(c, -0x7bc0));
        let slope_cos = ldf(c, 1, p(r, LAUNCH_DIR + 8));
        let limit = setf(c, 0, fr(c, 27) + margin);
        if fcc(c, limit.lt(slope_cos)) {
            launch_undo_backward(c, r, s);
        }

        launch_flat_ground(c, r, s);
        launch_cap_and_damp(c)?;

        // Forward air speed: (velocity · dir) * |(axis × velocity)^ × dir|.
        let r = r32(c, S0);
        let s = sp(c);
        c.cycles += 23;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
        let axis = c.g[V0][0] as u32;
        v_cross(c, axis, p(r, LAUNCH_VEL), p(s, 0x30));
        qcopy(c, V0, p(s, 0x30), p(s, 0x20));
        let cl = v_length(c, p(s, 0x20), V1, 1);
        if !fcc(c, fr(c, 0).lt(cl)) {
            stf(c, 0, p(r, LAUNCH_AIR_FORWARD));
        } else {
            c.cycles += 37;
            let one = li_f(c, 0, 0x3f80_0000);
            setf(c, 0, one.div(cl));
            v_scale(c, V0, 0, p(s, 0x20), p(s, 0x20));
            v_cross(c, p(s, 0x20), p(r, LAUNCH_DIR), p(s, 0x40));
            c.lq(V0, p(s, 0x40));
            v_dot(c, p(r, LAUNCH_VEL), p(r, LAUNCH_DIR));
            c.qmfc2(V1, 4);
            c.mtc1(V1, 0);
            c.sq(V0, p(s, 0x30));
            let len = v_length(c, p(s, 0x30), V1, 1);
            setf(c, 0, fr(c, 0) * len);
            stf(c, 0, p(r, LAUNCH_AIR_FORWARD));
        }

        c.cycles += 5;
        setf(c, 0, FpuF32(0));
        let pushed = fcc(c, fr(c, 0).lt(fr(c, 26)));
        c.g[V0][0] = 0x1;
        if !pushed {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 14;
        c.sw(p(r, LAUNCH_PUSHED), c.g[V0][0]);
        frame.restore(c, S0, 0xe0);
        frame.restore(c, S1, 0xd0);
        frame.restore_ra(c, 0xc0);
        for (f, off) in [
            (27, 0x128),
            (26, 0x120),
            (25, 0x118),
            (24, 0x110),
            (23, 0x108),
            (22, 0x100),
            (21, 0xf8),
            (20, 0xf0),
        ] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}

/// [`rider_launch_into_air`] off a rail (the `+0x378` slope cosine loaded
/// into f2): the push goes along `normalize(dir + side * 0.2)` (at sp+0x10);
/// when the side's z, the slope factor `t` (f25) and the velocity's z are all
/// non-negative, the velocity is steered toward the side instead
/// ([`launch_steer`]). Leaves `v0` = the world.
fn launch_ground_push(c: &mut Cpu) -> crate::call::Flow {
    let r = r32(c, S0);
    let s = sp(c);
    ldf(c, 2, p(r, LAUNCH_DIR + 8));
    c.cycles += 37;
    let span = setf(c, 1, fr(c, 27) - fr(c, 20));
    ldf(c, 0, gp(c, -0x7bcc));
    v_scale(c, V0, 0, p(r, LAUNCH_SIDE), p(s, 0x40));
    let one = li_f(c, 24, 0x3f80_0000);
    c.lq(V0, p(s, 0x40));
    let rel = setf(c, 2, fr(c, 2) - fr(c, 20));
    let inv = setf(c, 1, one.div(span));
    c.sq(V0, p(s, 0x30));
    v_add(c, p(r, LAUNCH_DIR), p(s, 0x30), p(s, 0x40));
    let zero = setf(c, 22, FpuF32(0));
    qcopy(c, V0, p(s, 0x40), p(s, 0x20));
    v_normalize(c, p(s, 0x20), p(s, 0x40));
    let t = setf(c, 25, rel * inv);
    let side_z = ldf(c, 0, p(r, LAUNCH_SIDE + 8));
    qcopy(c, V0, p(s, 0x40), p(s, 0x10));
    let mut steer = false;
    if !fcc(c, side_z.lt(zero)) {
        c.cycles += 3;
        if !fcc(c, t.lt(zero)) {
            c.cycles += 5;
            let vz = ldf(c, 0, p(r, LAUNCH_VEL + 8));
            steer = !fcc(c, vz.lt(zero));
        }
    }
    if steer {
        return launch_steer(c);
    }
    // velocity += dir * f21.
    c.cycles += 13;
    v_scale(c, V1, 21, p(s, 0x10), p(s, 0x30));
    qcopy(c, V0, p(s, 0x30), p(s, 0x20));
    v_add(c, p(r, LAUNCH_VEL), p(s, 0x20), p(r, LAUNCH_VEL));
    c.g[V0][0] = c.lw(gp(c, -0x848));
    Ok(())
}

/// The steered push: `flat` = the ground direction without z (sp+0x20),
/// `side` = the side vector without its `flat` part (sp+0x30), `vdir` = the
/// velocity without its `flat` part (sp+0x40), all normalised; `f` =
/// `t * (side.z - sin 0) / (sin 20° - sin 0)` clamped to 0..0.95; the push
/// direction becomes `normalize(dir * (1-f) + side * f)` and the velocity
/// `v * (1-f) + vdir * f * speed + dir * push * 0.5`. Sets `s1` (steered).
fn launch_steer(c: &mut Cpu) -> crate::call::Flow {
    let r = r32(c, S0);
    let s = sp(c);
    ldf(c, 1, p(r, LAUNCH_DIR));
    c.cycles += 101;
    setf(c, 12, fr(c, 22));
    ldf(c, 0, p(r, LAUNCH_DIR + 4));
    stf(c, 1, p(s, 0x30));
    stf(c, 0, p(s, 0x34));
    stf(c, 22, p(s, 0x38));
    stf(c, 22, p(s, 0x3c));
    v_normalize(c, p(s, 0x30), p(s, 0x40));
    qcopy(c, V0, p(s, 0x40), p(s, 0x20));
    // side - flat * (side · flat), normalised.
    v_dot(c, p(r, LAUNCH_SIDE), p(s, 0x20));
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
    v_scale(c, A0, 0, p(s, 0x20), p(s, 0x60));
    qcopy(c, V0, p(s, 0x60), p(s, 0x50));
    v_sub(c, p(r, LAUNCH_SIDE), p(s, 0x50), p(s, 0x60));
    qcopy(c, V0, p(s, 0x60), p(s, 0x40));
    v_normalize(c, p(s, 0x40), p(s, 0x60));
    qcopy(c, V0, p(s, 0x60), p(s, 0x30));
    // velocity - flat * (velocity · flat), normalised.
    v_dot(c, p(r, LAUNCH_VEL), p(s, 0x20));
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
    v_scale(c, A0, 0, p(s, 0x20), p(s, 0x70));
    qcopy(c, V0, p(s, 0x70), p(s, 0x60));
    v_sub(c, p(r, LAUNCH_VEL), p(s, 0x60), p(s, 0x70));
    qcopy(c, V0, p(s, 0x70), p(s, 0x50));
    v_normalize(c, p(s, 0x50), p(s, 0x70));
    qcopy(c, V0, p(s, 0x70), p(s, 0x40));
    call_ptr(c, 0x0031_bf60, 0x0011_4620)?;
    ldf(c, 12, gp(c, -0x7bc8));
    setf(c, 20, fr(c, 0));
    call_ptr(c, 0x0031_bf60, 0x0011_462c)?;
    let r = r32(c, S0);
    let s = sp(c);
    let range = setf(c, 0, fr(c, 0) - fr(c, 20));
    let side_z = ldf(c, 1, p(s, 0x38));
    let cap = ldf(c, 4, gp(c, -0x7bc4));
    let rel = setf(c, 1, side_z - fr(c, 20));
    let inv = setf(c, 0, fr(c, 24).div(range));
    let tf = setf(c, 1, fr(c, 25) * rel);
    let f = setf(c, 3, tf * inv);
    let positive = fcc(c, fr(c, 22).le(f));
    setf(c, 2, fr(c, 22));
    if positive {
        c.cycles += 1;
        setf(c, 2, f.min(cap));
    }

    c.cycles += 81;
    let f = setf(c, 3, fr(c, 2));
    c.g[S1][0] = 0x1;
    setf(c, 0, fr(c, 24) - f);
    li_f(c, 2, 0x3f00_0000);
    // dir = normalize(dir * (1-f) + side * f), at sp+0x10 and sp+0x50.
    v_scale(c, V0, 0, p(s, 0x10), p(s, 0x80));
    setf(c, 1, f * fr(c, 23));
    qcopy(c, V0, p(s, 0x80), p(s, 0x70));
    v_scale(c, V1, 3, p(s, 0x30), p(s, 0x90));
    qcopy(c, V0, p(s, 0x90), p(s, 0x80));
    v_add(c, p(s, 0x70), p(s, 0x80), p(s, 0x90));
    qcopy(c, V0, p(s, 0x90), p(s, 0x60));
    v_normalize(c, p(s, 0x60), p(s, 0x90));
    c.lq(V0, p(s, 0x90));
    c.sq(V0, p(s, 0x10));
    c.sq(V0, p(s, 0x50));
    // velocity = v * (1-f) + vdir * f * speed + dir * push * 0.5.
    v_scale(c, V1, 0, p(r, LAUNCH_VEL), p(s, 0x80));
    qcopy(c, V0, p(s, 0x80), p(s, 0x70));
    v_scale(c, V1, 1, p(s, 0x40), p(s, 0x90));
    qcopy(c, V0, p(s, 0x90), p(s, 0x80));
    v_add(c, p(s, 0x70), p(s, 0x80), p(s, 0x90));
    qcopy(c, V0, p(s, 0x90), p(s, 0x60));
    v_scale(c, V1, 21, p(s, 0x10), p(s, 0xb0));
    qcopy(c, V0, p(s, 0xb0), p(s, 0xa0));
    v_scale(c, V1, 2, p(s, 0xa0), p(s, 0xb0));
    qcopy(c, V0, p(s, 0xb0), p(s, 0x90));
    v_add(c, p(s, 0x60), p(s, 0x90), p(s, 0xb0));
    c.lq(V0, p(s, 0xb0));
    c.sq(V0, p(r, LAUNCH_VEL));
    c.sq(V0, p(s, 0x50));
    c.g[V0][0] = c.lw(gp(c, -0x848));
    Ok(())
}

/// On a lip steeper than the high angle (+0.02): when the change of the
/// velocity's xy points backward (`old · dv < 0`), take `dv * k` back off,
/// `k` ramping 0..1 as the old speed goes from half to 70% of the maximum.
fn launch_undo_backward(c: &mut Cpu, r: u32, s: u32) {
    c.cycles += 24;
    v_sub(c, p(r, LAUNCH_VEL), s, p(s, 0x20));
    c.lq(V0, s);
    c.lq(V1, p(s, 0x20));
    c.sq(V0, p(s, 0x20));
    c.sw(p(s, 0x28), 0);
    c.sq(V1, p(s, 0x10));
    let zero = ldf(c, 4, p(s, 0x28));
    c.sw(p(s, 0x18), 0);
    v_dot(c, p(s, 0x20), p(s, 0x10));
    c.qmfc2(V0, 4);
    c.mtc1(V0, 0);
    if !fcc(c, fr(c, 0).lt(zero)) {
        return;
    }
    c.cycles += 9;
    let max = ldf(c, 0, p(r, LAUNCH_MAX_SPEED));
    let half = li_f(c, 1, 0x3f00_0000);
    let k70 = ldf(c, 2, gp(c, -0x7bbc));
    let a = setf(c, 3, max * half);
    let speed = fr(c, 23);
    let above = fcc(c, a.le(speed));
    let b = setf(c, 0, max * k70);
    if !above {
        c.cycles += 1;
        setf(c, 0, fr(c, 4));
    } else {
        c.cycles += 3;
        if !fcc(c, speed.le(b)) {
            c.cycles += 4;
            li_f(c, 0, 0x3f80_0000);
        } else {
            c.cycles += 5;
            let span = setf(c, 1, b - a);
            let rel = setf(c, 0, speed - a);
            setf(c, 0, rel.div(span));
        }
    }
    c.cycles += 12;
    v_scale(c, V1, 0, p(s, 0x10), p(s, 0x40));
    qcopy(c, V0, p(s, 0x40), p(s, 0x30));
    v_sub(c, p(r, LAUNCH_VEL), p(s, 0x30), p(r, LAUNCH_VEL));
}

/// The ground heading flattened (z = 0) and normalised (or the default
/// 0x4ff140 when x and y are 0) goes to +0x3c0; when the heading's z is
/// within ±0.05 (flat ground) the velocity loses its part along it, the
/// position moves 4 units along it, and unless flag 0x20 is set the
/// velocity is scaled by ~0.8.
fn launch_flat_ground(c: &mut Cpu, r: u32, s: u32) {
    c.lq(V0, p(r, LAUNCH_HEADING));
    c.cycles += 10;
    let hz = ldf(c, 2, p(r, LAUNCH_HEADING + 8));
    c.sq(V0, p(s, 0x10));
    c.sw(p(s, 0x18), 0);
    let hx = ldf(c, 0, p(s, 0x10));
    let zero = ldf(c, 1, p(s, 0x18));
    let ax = setf(c, 0, hx.abs());
    let nonzero_x = fcc(c, zero.lt(ax));
    let hy = ldf(c, 0, p(s, 0x14));
    let mut nonzero = nonzero_x;
    if !nonzero_x {
        c.cycles += 5;
        let ay = setf(c, 0, hy.abs());
        nonzero = fcc(c, zero.lt(ay));
        c.g[V0][0] = 0x50_0000;
    }
    if nonzero {
        c.cycles += 19;
        v_length(c, p(s, 0x10), V0, 0);
        c.mfc1(V1, 0);
        c.qmtc2(V1, 3);
        c.vu0().div(0, 3, 3, 0);
        c.lqc2(4, p(s, 0x10));
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 4, Src::Q);
        c.sqc2(5, p(s, 0x10));
    } else {
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xec0));
        c.lq(V1, c.g[V0][0] as u32);
        c.sq(V1, p(s, 0x10));
    }

    c.cycles += 6;
    let lim = ldf(c, 0, gp(c, -0x7bb8));
    c.lq(V0, p(s, 0x10));
    let below = fcc(c, hz.lt(lim));
    c.sq(V0, p(r, LAUNCH_FLAT_HEADING));
    if !below {
        c.cycles += 1;
        return;
    }
    c.cycles += 5;
    let neg_lim = ldf(c, 0, gp(c, -0x7bb4));
    if !fcc(c, neg_lim.lt(hz)) {
        return;
    }
    c.cycles += 47;
    v_dot(c, p(r, LAUNCH_VEL), p(s, 0x10));
    c.qmfc2(V0, 4);
    c.mtc1(V0, 0);
    v_dot(c, p(s, 0x10), p(s, 0x10));
    c.qmfc2(V1, 4);
    c.mtc1(V1, 1);
    setf(c, 0, fr(c, 0).div(fr(c, 1)));
    li_f(c, 2, 0x4080_0000);
    v_scale(c, V1, 0, p(s, 0x10), p(s, 0x30));
    qcopy(c, V0, p(s, 0x30), p(s, 0x20));
    v_sub(c, p(r, LAUNCH_VEL), p(s, 0x20), p(r, LAUNCH_VEL));
    v_scale(c, V1, 2, p(s, 0x10), p(s, 0x40));
    qcopy(c, V0, p(s, 0x40), p(s, 0x30));
    v_add(c, p(r, LAUNCH_POS), p(s, 0x30), p(r, LAUNCH_POS));
    c.g[V0][0] = c.lh(p(r, LAUNCH_FLAGS));
    c.g[V0][0] &= 0x20;
    if c.g[V0][0] != 0 {
        return;
    }
    c.cycles += 6;
    ldf(c, 0, gp(c, -0x7bb0));
    v_scale(c, V0, 0, p(r, LAUNCH_VEL), p(r, LAUNCH_VEL));
    c.cycles += 1;
}

/// Cap the speed at the maximum (f1, +0x2e4); then, for a real push outside
/// sub states 11..13, scale the velocity by the take-off damping: 0.694 for
/// the first 0.539 s after the owner's take-off frame (+0x10), then ramping
/// to 1. Leaves `v0` = 0x500000 and f0 = 0.
fn launch_cap_and_damp(c: &mut Cpu) -> crate::call::Flow {
    let r = r32(c, S0);
    let max = ldf(c, 1, p(r, LAUNCH_MAX_SPEED));
    c.cycles += 13;
    let len = v_length(c, p(r, LAUNCH_VEL), V0, 0);
    if fcc(c, max.lt(len)) {
        c.cycles += 6;
        setf(c, 0, max.div(len));
        v_scale(c, V0, 0, p(r, LAUNCH_VEL), p(r, LAUNCH_VEL));
    }

    c.cycles += 5;
    let zero = setf(c, 0, FpuF32(0));
    let pushing = fcc(c, zero.le(fr(c, 26)));
    c.g[V0][0] = 0x50_0000;
    if !pushing {
        return Ok(());
    }
    c.cycles += 5;
    c.g[V0][0] = c.lw(p(r, LAUNCH_SUB_STATE));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb));
    c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
    let skip = c.g[V0][0] != 0;
    c.g[V0][0] = 0x50_0000;
    if skip {
        return Ok(());
    }
    c.cycles += 14;
    Callee {
        addr: 0x0012_98c8,
        f: ee::s00::AI_GetFrameCounter,
    }
    .call(c, 0x0011_4b04)?;
    let r = r32(c, S0);
    c.g[A0][0] = c.lw(p(r, LAUNCH_OWNER));
    let per_frame = ldf(c, 1, gp(c, -0x7bac));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
    let early = ldf(c, 2, gp(c, -0x7ba8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    c.mtc1(V0, 0);
    let frames = FpuF32::from_i32(c.f[0] as i32);
    let t = setf(c, 0, frames * per_frame);
    if !fcc(c, t.le(early)) {
        let since = setf(c, 1, t - early);
        c.cycles += 9;
        let floor = ldf(c, 3, gp(c, -0x7ba0));
        let rate = ldf(c, 2, gp(c, -0x7b9c));
        let one = li_f(c, 0, 0x3f80_0000);
        let u = setf(c, 1, since * rate);
        let u = setf(c, 1, u.min(one));
        let rest = setf(c, 0, one - u);
        let rest = setf(c, 0, rest * floor);
        setf(c, 0, u + rest);
    } else {
        c.cycles += 3;
        ldf(c, 0, gp(c, -0x7ba4));
    }
    c.cycles += 7;
    v_scale(c, V0, 0, p(r, LAUNCH_VEL), p(r, LAUNCH_VEL));
    c.g[V0][0] = 0x50_0000;
    setf(c, 0, FpuF32(0));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, put_vec, scramble, Rng, DATA, GAME_GP};

    /// `Rider_LaunchIntoAir` against the generated body: negative and
    /// positive pushes over both speed ranges and the clamp, rail and ground
    /// take-offs, the steered push (all three gates and the 0..0.95 clamp),
    /// steep lips with forward and backward changes (all three `k` ranges),
    /// zero and non-zero headings on flat and sloped ground with flag 0x20,
    /// the speed cap, sub states 11..13, early and late damping, and a zero
    /// forward cross product. The real callees take their short paths: the
    /// replay is busy, the metrix is idle (no points, so no boost).
    #[test]
    fn rider_launch_into_air_matches() {
        check(
            ee::s00::f_00114298__generated,
            rider_launch_into_air,
            250,
            &|c, seed| {
                let mut r = Rng(0x5eed_1142_98aa ^ (seed as u64) << 12);
                scramble(c, &mut r);
                elf_image(c);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0011_4298;
                let rider = DATA;
                let owner = DATA + 0x2000;
                let metrix = DATA + 0x3000;
                let world = DATA + 0x4000;
                let sys = DATA + 0x4100;
                let clock = DATA + 0x4200;
                let replay = DATA + 0x5000;
                let wscript = DATA + 0x6000;
                c.g[A0][0] = sx(rider);
                let fl = |v: f32| v.to_bits();

                // Push: negative, zero, or 0..1.6.
                c.f[12] = match seed % 6 {
                    0 => fl(-0.5),
                    1 => 0,
                    _ => r.f32(1.6) & 0x7fff_ffff,
                };
                // Velocity: slow or fast; z of either sign, sometimes all 0.
                let range = if seed % 2 == 0 { 600.0 } else { 1500.0 };
                let vel = if seed % 37 == 5 {
                    [0, 0, 0, 0]
                } else {
                    [r.f32(range), r.f32(range), r.f32(range * 0.5), 0]
                };
                put_vec(c, rider + LAUNCH_VEL, vel);
                put_vec(
                    c,
                    rider + LAUNCH_UP,
                    [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0],
                );
                put_vec(
                    c,
                    rider + LAUNCH_POS,
                    [r.f32(1e4), r.f32(1e4), r.f32(1e3), fl(1.0)],
                );
                // Ground direction, its z the slope cosine (both sides of
                // cos 50° and cos 70° + 0.02).
                let slope = [fl(0.9), fl(0.5), fl(0.3), fl(0.36), fl(0.7)][(seed / 3 % 5) as usize];
                put_vec(c, rider + LAUNCH_DIR, [r.f32(1.0), r.f32(1.0), slope, 0]);
                let side_z = if seed % 4 == 0 {
                    r.f32(1.0)
                } else {
                    r.f32(0.4) & 0x7fff_ffff
                };
                put_vec(c, rider + LAUNCH_SIDE, [r.f32(1.0), r.f32(1.0), side_z, 0]);
                // Heading: zero xy sometimes; z flat or sloped.
                let (hx, hy) = match seed % 7 {
                    0 => (0, 0),
                    1 => (0, r.f32(1.0)),
                    _ => (r.f32(1.0), r.f32(1.0)),
                };
                let hz = if seed % 3 == 0 {
                    r.f32(0.5)
                } else {
                    r.f32(0.06)
                };
                put_vec(c, rider + LAUNCH_HEADING, [hx, hy, hz, r.f32(1.0)]);
                c.bus
                    .write16(rider + LAUNCH_FLAGS, (r.next_u32() & 0x30) as u16);
                c.bus.write32(
                    rider + LAUNCH_MAX_SPEED,
                    fl(200.0 + (r.next_u32() % 2400) as f32),
                );
                c.bus.write32(rider + LAUNCH_SUB_STATE, r.next_u32() % 16);
                c.bus
                    .write32(rider + 0x304, if seed % 5 == 0 { 3 } else { 0 });
                let stance = r.next_u32() % 3;
                c.bus.write32(rider + LAUNCH_STANCE_A, stance);
                c.bus.write32(rider + LAUNCH_STANCE_B, r.next_u32() % 3);
                c.bus.write32(rider + LAUNCH_RIDER_ID, r.next_u32() % 8);
                c.bus.write32(rider + LAUNCH_OWNER, owner);
                c.bus.write32(rider + LAUNCH_METRIX, metrix);

                // Owner: main state (4 = rail), take-off frame.
                let state = if seed % 4 == 1 { 4 } else { r.next_u32() % 4 };
                c.bus.write32(owner + 0xde0, state);
                let now = 1000 + r.next_u32() % 1000;
                c.bus.write32(owner + 0x10, now - r.next_u32() % 90);
                // Metrix idle (InterruptTrick banks nothing).
                c.bus.write32(metrix + 0x70, 1);
                // World -> system: replay (busy) and frame clock.
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, sys);
                c.bus.write32(sys + 0x28, replay);
                c.bus.write32(sys + 0xc, clock);
                c.bus.write32(clock + 8, now);
                for off in [0x0, 0x60c, 0x610, 0x61c, 0x620] {
                    c.bus.write32(replay + off, 1);
                }
                // World script with a few queued events.
                c.bus.write32(GAME_GP + 0xce8, wscript);
                c.bus.write32(wscript + 0x40, r.next_u32() % 4);
                // Default heading and the cross axis.
                put_vec(c, 0x004f_f140, [fl(1.0), 0, 0, 0]);
                let axis = if seed % 11 == 3 {
                    [0, 0, 0, 0]
                } else {
                    [0, 0, fl(1.0), 0]
                };
                put_vec(c, 0x004f_f160, axis);
            },
        );
    }
}
