//! Staging wave 211 (oc289): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

// ---------------------------------------------------------------------------
// Rider_StartWipeout.

use crate::game::ai::rider::{q_rotate, quat_mul_vu0};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::vu0::{Src, X};

/// Rider fields used by the wipeout start.
const WIPE_VTABLE: u32 = 0x6c0;
/// `vibrationBump(float)` slot of the rider vtable: `{delta (lh), fn}`.
const WIPE_VT_VIBRATION_BUMP: u32 = 0x88;
const WIPE_POSITION: u32 = 0x110;
const WIPE_ORIENT: u32 = 0x120;
const WIPE_UP: u32 = 0x1c0;
const WIPE_SLEW0_TARGET: u32 = 0x1f0;
const WIPE_SLEW0_CUR: u32 = 0x1f4;
const WIPE_SLEW0_RATE: u32 = 0x1f8;
const WIPE_SLEW2_TARGET: u32 = 0x208;
const WIPE_SLEW2_CUR: u32 = 0x20c;
const WIPE_SLEW2_RATE: u32 = 0x210;
const WIPE_SLEW8_TARGET: u32 = 0x250;
const WIPE_SLEW8_CUR: u32 = 0x254;
const WIPE_SLEW8_RATE: u32 = 0x258;
/// Grind/rail facing: 3 and 4 are the sideways stances (a quarter turn).
const WIPE_RAIL_STYLE: u32 = 0x328;
const WIPE_PRESS_TYPE: u32 = 0x330;
const WIPE_HANDLER: u32 = 0x77c;
const WIPE_ANIM_MODEL: u32 = 0x780;
const WIPE_ANIM: u32 = 0x784;
const WIPE_METRIX: u32 = 0x790;
const WIPE_BONE_HIPS: u32 = 0x89c;

/// Impact info: contact point (+0x20) and strength (+0x30).
const WIPE_EVT_POINT: u32 = 0x20;
const WIPE_EVT_STRENGTH: u32 = 0x30;

/// Animation model: bone translation (+0x24) and rotation (+0x28) tables,
/// per-axis bone scale (+0x140).
const WIPE_MODEL_BONE_POS: u32 = 0x24;
const WIPE_MODEL_BONE_ROT: u32 = 0x28;
const WIPE_MODEL_SCALE: u32 = 0x140;
/// Animation controller: heading offset quaternion (+0x30) and its
/// rotation (+0x40).
const WIPE_ANIM_OFFSET_A: u32 = 0x30;
const WIPE_ANIM_OFFSET_B: u32 = 0x40;

/// `.bss` vectors: the zero quadword (0x4ff130) and the up axis (0x4ff160),
/// addressed from 0x500000 as the original does.
const WIPE_HI: u32 = 0x0050_0000;
const WIPE_ZERO: u32 = WIPE_HI - 0xed0;
const WIPE_AXIS: u32 = WIPE_HI - 0xea0;

/// `.lit4` (gp-relative, subtracted): +pi/2 and -pi/2.
const WIPE_GP_HALF_PI: u32 = 0x7cb8;
const WIPE_GP_NEG_HALF_PI: u32 = 0x7cb4;
/// gp+0x1b18: the default blend id passed to the animation setup.
const WIPE_GP_BLEND: u32 = 0x1b18;

/// Stack frame (0x1e0) layout.
const WIPE_FRAME: u32 = 0x1e0;
const WIPE_TMP: u32 = 0x00;
const WIPE_SEQ_BUF: u32 = 0x10;
const WIPE_ANIM_XF: u32 = 0x60;
const WIPE_NODE_POS: u32 = 0x80;
const WIPE_NODE_ROT: u32 = 0x90;
const WIPE_Y_POS: u32 = 0xa0;
const WIPE_Y_ROT: u32 = 0xb0;
const WIPE_XF_POS: u32 = 0xc0;
const WIPE_XF_ROT: u32 = 0xd0;
const WIPE_XF_COPY: u32 = 0xe0;
const WIPE_Q: u32 = 0xf0;
const WIPE_Q_COPY: u32 = 0x100;
const WIPE_V0: u32 = 0x110;
const WIPE_V1: u32 = 0x120;
const WIPE_V2: u32 = 0x130;
const WIPE_V3: u32 = 0x140;
const WIPE_SINCOS_A: u32 = 0x150;
const WIPE_SINCOS_B: u32 = 0x158;

const WIPE_METRIX_ON_WIPEOUT: Callee = Callee {
    addr: 0x0011_9b08,
    f: ee::s00::RiderMetrix_OnWipeout,
};
const WIPE_ADD_BOOST: Callee = Callee {
    addr: 0x0010_e098,
    f: ee::s00::Rider_AddBoost,
};
const WIPE_DYNAMIC_SPRAY: Callee = Callee {
    addr: 0x0011_1aa0,
    f: ee::s00::RiderBase_TriggerDynamicSpray,
};
const WIPE_SET_SUB_STATE: Callee = Callee {
    addr: 0x0011_fec8,
    f: ee::s00::Rider_SetSubState,
};
const WIPE_SET_MAIN_STATE: Callee = Callee {
    addr: 0x0011_fe78,
    f: ee::s00::Rider_SetMainState,
};
const WIPE_ROTATE_AROUND_AXIS: Callee = Callee {
    addr: 0x0011_dfe0,
    f: ee::s00::Rider_RotateAroundAxis,
};
const WIPE_EMPTY_STANCE_HOOK: Callee = Callee {
    addr: 0x0011_6930,
    f: ee::s00::Rider_EmptyStanceHook,
};
const WIPE_UPDATE_BOARD_LEAN: Callee = Callee {
    addr: 0x0011_fa10,
    f: ee::s00::Rider_UpdateBoardLean,
};
const WIPE_BUILD_WORLD_MATRIX: Callee = Callee {
    addr: 0x0011_e098,
    f: ee::s00::Rider_BuildWorldMatrix,
};

/// Other-shard callees (through the function table).
const WIPE_AUDIO_INSTANCE: u32 = 0x0028_b180;
const WIPE_AUDIO_ON_WIPEOUT: u32 = 0x0029_6310;
const WIPE_AUDIO_GRUNT: u32 = 0x0029_f660;
const WIPE_CROWD_FALL: u32 = 0x0029_61f0;
const WIPE_HEADING_OFFSET: u32 = 0x0031_1b48;
const WIPE_SIN_COS: u32 = 0x0031_be50;
const WIPE_ANIM_SETUP: u32 = 0x0031_2660;
const WIPE_ANIM_SAMPLE_BONE: u32 = 0x0030_ecd8;
const WIPE_ORIENT_OFFSET: u32 = 0x0031_1bf0;
const WIPE_ANIM_PLAY: u32 = 0x0031_28e8;

/// `lwc1 f, at`, returning the value.
fn wipe_ldf(c: &mut Cpu, f: usize, at: u32) -> FpuF32 {
    let v = Addr(at).read_fpu(c);
    c.f[f] = v.0;
    v
}

/// Set FPU register `f` to `v`, returning it.
fn wipe_setf(c: &mut Cpu, f: usize, v: FpuF32) -> FpuF32 {
    c.f[f] = v.0;
    v
}

/// `lq reg, from; sq reg, to...` through a GPR.
fn wipe_copy(c: &mut Cpu, reg: usize, from: u32, to: &[u32]) {
    c.lq(reg, from);
    for &t in to {
        c.sq(reg, t);
    }
}

/// `vf5 = a - b` (all lanes) stored at `out`.
fn wipe_vsub(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// `vf5 = a + b` (all lanes) stored at `out`.
fn wipe_vadd(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// `vf5 = v * f20` (f20 = -1: the negated vector) stored at `out`; the
/// scale passes through v1 and vf3 as in the original.
fn wipe_vscale_f20(c: &mut Cpu, v: u32, out: u32) {
    c.mfc1(V1, 20);
    c.lqc2(4, v);
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, out);
}

/// `vf8 = a * b` (quaternion product) stored at `out`.
fn wipe_qmul(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(4, a);
    c.lqc2(5, b);
    quat_mul_vu0(c);
    c.sqc2(8, out);
}

/// The quarter turn of a sideways (rail style 3: +pi/2) stance: rotate the
/// rider about its up vector, turn the animation heading the same way,
/// then reset the controller's offset quaternion to `{axis * sin, cos}`
/// of `-0` (the sincos results). Registers as the original's arm.
fn wipe_quarter_turn_left(c: &mut Cpu, rider: u32) -> Flow {
    c.cycles += 38;
    let s = sp(c);
    c.lq(V0, rider + WIPE_UP);
    c.g[A1][0] = sx(s);
    let lit = r32(c, GP).wrapping_sub(WIPE_GP_HALF_PI);
    let half_pi = wipe_ldf(c, 20, lit);
    c.g[A0][0] = c.g[S3][0];
    c.sq(V0, s + WIPE_TMP);
    c.g[S0][0] = WIPE_HI as u64;
    wipe_setf(c, 12, half_pi);
    c.g[S4][0] = c.g[S0][0];
    WIPE_ROTATE_AROUND_AXIS.call(c, 0x0010_ec4c)?;
    c.g[S1][0] = WIPE_HI as u64;
    c.g[A0][0] = Addr(r32(c, S3).wrapping_add(WIPE_ANIM)).lw(c);
    let f20 = FpuF32(c.f[20]);
    wipe_setf(c, 12, f20);
    c.g[S2][0] = sx(WIPE_AXIS);
    call_ptr(c, WIPE_HEADING_OFFSET, 0x0010_ec60)?;
    let s = sp(c);
    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0xed0));
    c.f[12] = 0;
    c.lq(V0, r32(c, S0));
    c.g[A0][0] = sx(s + WIPE_SINCOS_A);
    let anim = Addr(r32(c, S3).wrapping_add(WIPE_ANIM)).lw(c);
    c.g[S0][0] = anim;
    c.f[12] ^= 0x8000_0000;
    c.g[A1][0] = sx(s + WIPE_SINCOS_A + 4);
    c.sq(V0, (anim as u32).wrapping_add(WIPE_ANIM_OFFSET_A));
    call_ptr(c, WIPE_SIN_COS, 0x0010_ec84)?;
    wipe_store_offset_quat(c, WIPE_SINCOS_A, S2, S1, S0);
    Ok(())
}

/// Rail style 4: the same quarter turn by -pi/2 (the original's second
/// arm, with its own register assignment).
fn wipe_quarter_turn_right(c: &mut Cpu, rider: u32) -> Flow {
    c.cycles += 35;
    let s = sp(c);
    c.lq(V0, rider + WIPE_UP);
    c.g[A1][0] = sx(s);
    let lit = r32(c, GP).wrapping_sub(WIPE_GP_NEG_HALF_PI);
    let neg_half_pi = wipe_ldf(c, 20, lit);
    c.g[A0][0] = c.g[S3][0];
    c.sq(V0, s + WIPE_TMP);
    c.g[S0][0] = WIPE_HI as u64;
    wipe_setf(c, 12, neg_half_pi);
    c.g[S1][0] = sx(WIPE_AXIS);
    WIPE_ROTATE_AROUND_AXIS.call(c, 0x0010_ecec)?;
    c.g[A0][0] = Addr(r32(c, S3).wrapping_add(WIPE_ANIM)).lw(c);
    let f20 = FpuF32(c.f[20]);
    wipe_setf(c, 12, f20);
    call_ptr(c, WIPE_HEADING_OFFSET, 0x0010_ecf8)?;
    let s = sp(c);
    c.f[12] = 0;
    c.g[V0][0] = sx(r32(c, S4).wrapping_sub(0xed0));
    c.lq(V1, r32(c, V0));
    c.g[A0][0] = sx(s + WIPE_SINCOS_B);
    let anim = Addr(r32(c, S3).wrapping_add(WIPE_ANIM)).lw(c);
    c.g[S2][0] = anim;
    c.f[12] ^= 0x8000_0000;
    c.g[A1][0] = sx(s + WIPE_SINCOS_B + 4);
    c.sq(V1, (anim as u32).wrapping_add(WIPE_ANIM_OFFSET_A));
    call_ptr(c, WIPE_SIN_COS, 0x0010_ed1c)?;
    wipe_store_offset_quat(c, WIPE_SINCOS_B, S1, S0, S2);
    Ok(())
}

/// `{axis.x, axis.y, axis.z} * sin, cos` (sin/cos at `sincos(sp)`) built in
/// the frame's first quadword and stored at `anim + 0x40`. `axis_reg`
/// holds the axis address, `hi_reg` 0x500000 (its x is read through it),
/// `anim_reg` the controller.
fn wipe_store_offset_quat(c: &mut Cpu, sincos: u32, axis_reg: usize, hi_reg: usize, anim_reg: usize) {
    let s = sp(c);
    let axis = r32(c, axis_reg);
    let az = wipe_ldf(c, 1, axis + 8);
    let sin = wipe_ldf(c, 0, s + sincos);
    let axis_x = r32(c, hi_reg).wrapping_sub(0xea0);
    let ax = wipe_ldf(c, 2, axis_x);
    let ay = wipe_ldf(c, 4, axis + 4);
    let qz = wipe_setf(c, 1, sin * az);
    let qx = wipe_setf(c, 2, sin * ax);
    let cos = wipe_ldf(c, 3, s + sincos + 4);
    let qy = wipe_setf(c, 0, sin * ay);
    Addr(s + 0xc).write_fpu(c, cos);
    Addr(s + 8).write_fpu(c, qz);
    Addr(s).write_fpu(c, qx);
    Addr(s + 4).write_fpu(c, qy);
    c.lq(V0, s);
    c.sq(V0, r32(c, anim_reg).wrapping_add(WIPE_ANIM_OFFSET_B));
}

/// Hips bone node of the sampled pose: `{pos * model scale, rot}` into
/// `NODE_POS`/`NODE_ROT`, and the conjugate of the sampled animation
/// rotation (`ANIM_XF + 0x10`) into `V0`, then `Q`/`Q_COPY`. Leaves a1 =
/// bone index * 16 and v1 = the bone rotation quadword as the original.
fn wipe_hips_node(c: &mut Cpu, rider: u32) {
    let s = sp(c);
    let model = Addr(rider + WIPE_ANIM_MODEL).lw(c);
    c.g[V1][0] = model;
    let model = model as u32;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = Addr(rider + WIPE_BONE_HIPS).lw(c);
    let pos_table = Addr(model + WIPE_MODEL_BONE_POS).lw(c);
    let bone = (r32(c, A1)) << 4;
    c.g[A1][0] = sx(bone);
    let sx_ = wipe_ldf(c, 4, model + WIPE_MODEL_SCALE);
    let pos = (pos_table as u32).wrapping_add(bone);
    c.g[V0][0] = sx(pos);
    let sy = wipe_ldf(c, 6, model + WIPE_MODEL_SCALE + 4);
    let px = wipe_ldf(c, 0, pos);
    let sz = wipe_ldf(c, 3, model + WIPE_MODEL_SCALE + 8);
    let px = wipe_setf(c, 0, px * sx_);
    let pw = wipe_ldf(c, 5, pos + 0xc);
    let sw = wipe_ldf(c, 4, model + WIPE_MODEL_SCALE + 0xc);
    let py = wipe_ldf(c, 1, pos + 4);
    let pz = wipe_ldf(c, 2, pos + 8);
    let pw = wipe_setf(c, 5, pw * sw);
    let py = wipe_setf(c, 1, py * sy);
    Addr(s).write_fpu(c, px);
    let pz = wipe_setf(c, 2, pz * sz);
    let rz = wipe_ldf(c, 0, s + WIPE_ANIM_XF + 0x18);
    Addr(s + 0xc).write_fpu(c, pw);
    Addr(s + 4).write_fpu(c, py);
    let rz = wipe_setf(c, 0, -rz);
    Addr(s + 8).write_fpu(c, pz);
    let rx = wipe_ldf(c, 3, s + WIPE_ANIM_XF + 0x10);
    c.lq(V0, s);
    let ry = wipe_ldf(c, 1, s + WIPE_ANIM_XF + 0x14);
    let rx = wipe_setf(c, 3, -rx);
    c.sq(V0, s + WIPE_NODE_POS);
    let ry = wipe_setf(c, 1, -ry);
    let rw = wipe_ldf(c, 2, s + WIPE_ANIM_XF + 0x1c);
    let rot_table = Addr(model + WIPE_MODEL_BONE_ROT).lw(c);
    let rot = (rot_table as u32).wrapping_add(bone);
    c.g[V0][0] = sx(rot);
    c.lq(V1, rot);
    Addr(s + WIPE_V0 + 0xc).write_fpu(c, rw);
    Addr(s + WIPE_V0).write_fpu(c, rx);
    Addr(s + WIPE_V0 + 4).write_fpu(c, ry);
    Addr(s + WIPE_V0 + 8).write_fpu(c, rz);
    c.sq(V1, s + WIPE_NODE_ROT);
    wipe_copy(c, V0, s + WIPE_V0, &[s + WIPE_Q, s + WIPE_Q_COPY]);
}

/// `Invert(xf)` with the conjugate rotation already at `Q`: the position
/// `-(rotate(Q, pos - 0) ) + 0` into `XF_POS`, the rotation into `XF_ROT`
/// (copies in `Q_COPY`, `XF_COPY`; temporaries `t`/`u` as the original).
fn wipe_invert(c: &mut Cpu, pos: u32, t: u32, u: u32) {
    let s = sp(c);
    wipe_vsub(c, s + pos, WIPE_ZERO, s + t);
    wipe_copy(c, V0, s + t, &[s + u]);
    q_rotate(c, s + WIPE_Q, s + u, s + t);
    wipe_copy(c, V0, s + t, &[s + WIPE_V1]);
    wipe_vscale_f20(c, s + WIPE_V1, s + t);
    wipe_copy(c, V0, s + t, &[s + WIPE_V0]);
    wipe_vadd(c, s + WIPE_V0, WIPE_ZERO, s + t);
}

/// `Combine(a, b)`: rotation `a.rot * b.rot` into `Q`/`Q_COPY`, position
/// `rotate(a.rot, b.pos - 0) + a.pos` left at `V2` (the caller copies it
/// out).
fn wipe_combine(c: &mut Cpu, a_rot: u32, a_pos: u32, b_pos: u32, b_rot: u32) {
    let s = sp(c);
    wipe_qmul(c, a_rot, b_rot, s + WIPE_V0);
    wipe_copy(c, V0, s + WIPE_V0, &[s + WIPE_Q, s + WIPE_Q_COPY]);
    wipe_vsub(c, b_pos, WIPE_ZERO, s + WIPE_V2);
    wipe_copy(c, V0, s + WIPE_V2, &[s + WIPE_V1]);
    q_rotate(c, a_rot, s + WIPE_V1, s + WIPE_V2);
    wipe_copy(c, V0, s + WIPE_V2, &[s + WIPE_V0]);
    wipe_vadd(c, s + WIPE_V0, a_pos, s + WIPE_V2);
}

/// `void Rider_StartWipeout(Rider* rider, int anim, int metrixFlag,
/// int sprayArg, const ImpactInfo* evt)` (0x0010eb30): the rider wipes
/// out. A controller rumble scaled by the impact strength; the metrics
/// note the wipeout (its score as boost), the audio system the wipeout,
/// grunt and crowd fall; a dynamic snow spray at the impact point; sub
/// state 0xd. A sideways rail stance (3 / 4) is turned a quarter back
/// first. Then the stance and slew state are cleared, the wipeout
/// animation `anim` is set up and its first frame's hips transform `y`
/// (sampled hips node combined with the inverse of the sampled root)
/// is applied to the rider (`rider.xf = rider.xf * y`), the animation's
/// orientation offset set to `inverse(y)`, and the animation played; sub
/// state 8, main state 2.
pub fn rider_start_wipeout(c: &mut Cpu) {
    guest(c, |c| {
        // Frame and saves; the rumble.
        c.cycles += 62;
        let frame = Frame::open(c, WIPE_FRAME);
        c.g[AT][0] = 0x40a0_0000;
        let five = wipe_setf(c, 12, FpuF32(0x40a0_0000));
        frame.save(c, S0, 0x1c0);
        frame.save(c, S1, 0x1b0);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x1a0);
        c.g[S1][0] = c.g[T0][0];
        frame.save(c, S3, 0x190);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S5, 0x170);
        c.g[S3][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x1d0);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S4, 0x180);
        frame.save_ra(c, 0x160);
        let rider = r32(c, S3);
        let evt = r32(c, S1);
        let vt = Addr(rider + WIPE_VTABLE).lw(c);
        c.g[V1][0] = vt;
        let vt = vt as u32;
        let strength = wipe_ldf(c, 0, evt + WIPE_EVT_STRENGTH);
        let delta = Addr(vt + WIPE_VT_VIBRATION_BUMP).lh(c);
        c.g[A0][0] = delta;
        wipe_setf(c, 12, strength * five);
        let method = Addr(vt + WIPE_VT_VIBRATION_BUMP + 4).lw(c);
        c.g[V0][0] = method;
        c.g[A0][0] = sx(rider.wrapping_add(delta as u32));
        call_ptr(c, method as u32, 0x0010_eb8c)?;

        // Metrics, boost, audio.
        c.g[A0][0] = Addr(r32(c, S3).wrapping_add(WIPE_METRIX)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        WIPE_METRIX_ON_WIPEOUT.call(c, 0x0010_eb98)?;
        c.f[20] = c.f[0];
        c.g[A0][0] = c.g[S3][0];
        c.f[12] = c.f[20];
        c.g[A1][0] = 1;
        WIPE_ADD_BOOST.call(c, 0x0010_ebac)?;
        call_ptr(c, WIPE_AUDIO_INSTANCE, 0x0010_ebb4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, WIPE_AUDIO_ON_WIPEOUT, 0x0010_ebc0)?;
        call_ptr(c, WIPE_AUDIO_INSTANCE, 0x0010_ebc8)?;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, WIPE_AUDIO_GRUNT, 0x0010_ebdc)?;
        call_ptr(c, WIPE_AUDIO_INSTANCE, 0x0010_ebe4)?;
        c.g[A0][0] = c.g[V0][0];
        c.f[12] = c.f[20];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, WIPE_CROWD_FALL, 0x0010_ebf4)?;

        // Snow spray at the impact point; sub state 0xd.
        c.g[A1][0] = c.g[S1][0];
        let evt = r32(c, A1);
        c.g[A0][0] = Addr(r32(c, S3).wrapping_add(WIPE_HANDLER)).lw(c);
        wipe_ldf(c, 12, evt + WIPE_EVT_STRENGTH);
        c.g[A2][0] = sx(evt.wrapping_add(WIPE_EVT_POINT));
        c.g[A3][0] = c.g[S2][0];
        WIPE_DYNAMIC_SPRAY.call(c, 0x0010_ec0c)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0xd;
        WIPE_SET_SUB_STATE.call(c, 0x0010_ec18)?;

        // Sideways rail stances turn back a quarter.
        let rider = r32(c, S3);
        let style = Addr(rider + WIPE_RAIL_STYLE).lw(c);
        c.g[V1][0] = style;
        c.g[V0][0] = 4;
        if style == 3 {
            wipe_quarter_turn_left(c, rider)?;
        } else {
            c.cycles += 2;
            c.g[S4][0] = WIPE_HI as u64;
            if style == 4 {
                wipe_quarter_turn_right(c, rider)?;
            }
        }

        // Clear the stance and the slews; set up the wipeout animation.
        c.cycles += 283;
        let rider = r32(c, S3);
        Addr(rider + WIPE_RAIL_STYLE).sw(c, 0);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.f[20] = 0xbf80_0000;
        c.g[A0][0] = c.g[S3][0];
        WIPE_EMPTY_STANCE_HOOK.call(c, 0x0010_ed68)?;
        c.g[A1][0] = sx(r32(c, S3).wrapping_add(WIPE_POSITION));
        c.g[A0][0] = c.g[S3][0];
        WIPE_UPDATE_BOARD_LEAN.call(c, 0x0010_ed74)?;
        c.g[A0][0] = c.g[S3][0];
        WIPE_BUILD_WORLD_MATRIX.call(c, 0x0010_ed7c)?;
        let rider = r32(c, S3);
        let s = sp(c);
        Addr(rider + WIPE_SLEW0_RATE).sw(c, 0);
        c.g[S0][0] = sx(s + WIPE_SEQ_BUF);
        Addr(rider + WIPE_SLEW0_TARGET).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        Addr(rider + WIPE_SLEW0_CUR).sw(c, 0);
        c.g[A2][0] = c.g[S5][0];
        for off in [
            WIPE_SLEW2_RATE,
            WIPE_SLEW2_TARGET,
            WIPE_SLEW2_CUR,
            WIPE_SLEW8_RATE,
            WIPE_SLEW8_TARGET,
            WIPE_SLEW8_CUR,
            WIPE_PRESS_TYPE,
        ] {
            Addr(rider + off).sw(c, 0);
        }
        wipe_ldf(c, 12, rider + WIPE_SLEW0_RATE);
        c.g[A1][0] = Addr(rider + WIPE_ANIM).lw(c);
        c.g[A3][0] = Addr(r32(c, GP).wrapping_add(WIPE_GP_BLEND)).lw(c);
        call_ptr(c, WIPE_ANIM_SETUP, 0x0010_edc0)?;
        let s = sp(c);
        c.g[A1][0] = Addr(r32(c, S3).wrapping_add(WIPE_ANIM_MODEL)).lw(c);
        c.g[A0][0] = sx(s + WIPE_ANIM_XF);
        c.g[A3][0] = 0;
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, WIPE_ANIM_SAMPLE_BONE, 0x0010_edd4)?;

        // y = node * inverse(sampled root); rider.xf = rider.xf * y.
        let rider = r32(c, S3);
        let s = sp(c);
        wipe_hips_node(c, rider);
        wipe_invert(c, WIPE_ANIM_XF, WIPE_V3, WIPE_V2);
        c.lq(V0, s + WIPE_V3);
        c.lq(V1, s + WIPE_Q);
        c.sq(V0, s + WIPE_Q_COPY);
        c.sq(V0, s + WIPE_XF_POS);
        c.sq(V1, s + WIPE_XF_ROT);
        c.sq(V0, s + WIPE_XF_COPY);
        wipe_combine(c, s + WIPE_NODE_ROT, s + WIPE_NODE_POS, s + WIPE_XF_POS, s + WIPE_XF_ROT);
        c.lq(V0, s + WIPE_V2);
        c.lq(V1, s + WIPE_Q);
        c.sq(V0, s + WIPE_Q_COPY);
        c.sq(V0, s + WIPE_Y_POS);
        c.sq(V1, s + WIPE_Y_ROT);
        c.sq(V0, s + WIPE_XF_COPY);
        wipe_combine(c, rider + WIPE_ORIENT, rider + WIPE_POSITION, s + WIPE_Y_POS, s + WIPE_Y_ROT);
        c.lq(V0, s + WIPE_V2);
        c.lq(V1, s + WIPE_Q);
        c.sq(V0, rider + WIPE_POSITION);
        c.sq(V1, rider + WIPE_ORIENT);
        c.sq(V0, s + WIPE_Q_COPY);
        c.sq(V0, s + WIPE_XF_COPY);
        c.sq(V0, s + WIPE_XF_POS);
        c.g[RA][0] = sx(0x0010_f090);
        c.sq(V1, s + WIPE_XF_ROT);
        WIPE_BUILD_WORLD_MATRIX.call(c, 0x0010_f090)?;

        // The animation's orientation offset = inverse(y).
        let s = sp(c);
        let qx = wipe_ldf(c, 3, s + WIPE_Y_ROT);
        c.g[A1][0] = sx(s + WIPE_XF_POS);
        let qy = wipe_ldf(c, 0, s + WIPE_Y_ROT + 4);
        let qz = wipe_ldf(c, 1, s + WIPE_Y_ROT + 8);
        let qx = wipe_setf(c, 3, -qx);
        let qy = wipe_setf(c, 0, -qy);
        let qw = wipe_ldf(c, 2, s + WIPE_Y_ROT + 0xc);
        let qz = wipe_setf(c, 1, -qz);
        Addr(s + WIPE_V0 + 0xc).write_fpu(c, qw);
        Addr(s + WIPE_V0).write_fpu(c, qx);
        Addr(s + WIPE_V0 + 4).write_fpu(c, qy);
        Addr(s + WIPE_V0 + 8).write_fpu(c, qz);
        wipe_copy(c, V0, s + WIPE_V0, &[s + WIPE_Q, s + WIPE_Q_COPY]);
        wipe_invert(c, WIPE_Y_POS, WIPE_V3, WIPE_V2);
        c.lq(V0, s + WIPE_V3);
        c.sq(V0, s + WIPE_Q_COPY);
        c.lq(V1, s + WIPE_Q);
        c.g[A0][0] = Addr(r32(c, S3).wrapping_add(WIPE_ANIM)).lw(c);
        c.sq(V0, s + WIPE_XF_POS);
        c.sq(V1, s + WIPE_XF_ROT);
        c.g[RA][0] = sx(0x0010_f16c);
        c.sq(V0, s + WIPE_XF_COPY);
        call_ptr(c, WIPE_ORIENT_OFFSET, 0x0010_f16c)?;

        // Play it; sub state 8, main state 2.
        c.g[A0][0] = Addr(r32(c, S3).wrapping_add(WIPE_ANIM)).lw(c);
        c.f[12] = c.f[20];
        c.g[A1][0] = c.g[S5][0];
        c.g[A2][0] = 0;
        call_ptr(c, WIPE_ANIM_PLAY, 0x0010_f180)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 8;
        WIPE_SET_SUB_STATE.call(c, 0x0010_f18c)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 2;
        WIPE_SET_MAIN_STATE.call(c, 0x0010_f198)?;
        frame.restore(c, S0, 0x1c0);
        frame.restore(c, S1, 0x1b0);
        frame.restore(c, S2, 0x1a0);
        frame.restore(c, S3, 0x190);
        frame.restore(c, S4, 0x180);
        frame.restore(c, S5, 0x170);
        frame.restore_ra(c, 0x160);
        frame.restore_fpr(c, 20, 0x1d0);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_eb30, rider_start_wipeout, "Rider_StartWipeout"),
];

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{put_vec, Rng, DATA, GAME_GP};

    // ------------------------------------------------------ Rider_StartWipeout

    /// Fixture area (clear of the fakes' pointer results at DATA..+0x40000
    /// and their log at DATA+0x100000).
    const WT_CTRL: u32 = DATA + 0x20_0000;
    const WT_MAGIC: u32 = 0x10eb_30a5;
    const WT_RIDER: u32 = WT_CTRL + 0x1000;
    const WT_EVT: u32 = WT_CTRL + 0x2000;
    const WT_METRIX: u32 = WT_CTRL + 0x2400;
    const WT_ANIM: u32 = WT_CTRL + 0x2800;
    const WT_LAYERS: u32 = WT_CTRL + 0x2900;
    const WT_MODEL: u32 = WT_CTRL + 0x2c00;
    const WT_BONE_POS: u32 = WT_CTRL + 0x3000;
    const WT_BONE_ROT: u32 = WT_CTRL + 0x3100;
    const WT_ANIM_TABLE: u32 = WT_CTRL + 0x3400;
    const WT_ANIM_SET: u32 = WT_CTRL + 0x3500;
    const WT_ANIM_DEFS: u32 = WT_CTRL + 0x3600;
    const WT_ENTRY: u32 = WT_CTRL + 0x3800;
    const WT_SEQ: u32 = WT_CTRL + 0x3a00;

    /// A logged fake (which may unwind); the animation table entry and the
    /// pushed sequence answer fixture objects.
    fn wipe_fake(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        match target {
            // AnimTable_GetEntry
            0x0031_15c8 => c.g[V0][0] = WT_ENTRY as u64,
            // AnimSeqList_PushAnim / PushLoopingAnim
            0x0031_4518 | 0x0031_4588 => c.g[V0][0] = WT_SEQ as u64,
            _ => {}
        }
    }

    /// A slot replacement: the fake while this fixture is loaded, else the
    /// generated body.
    macro_rules! wipe_gated {
        ($name:ident, $addr:expr, $gen:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(WT_CTRL) != WT_MAGIC {
                    return $gen(c);
                }
                wipe_fake(c, $addr)
            }
        };
    }

    wipe_gated!(wipe_fake_0010e098, 0x0010_e098, ee::s00::f_0010e098__generated);
    wipe_gated!(wipe_fake_00111aa0, 0x0011_1aa0, ee::s00::f_00111aa0__generated);
    wipe_gated!(wipe_fake_0011fec8, 0x0011_fec8, ee::s00::f_0011fec8__generated);
    wipe_gated!(wipe_fake_0011dfe0, 0x0011_dfe0, ee::s00::f_0011dfe0__generated);
    wipe_gated!(wipe_fake_00116930, 0x0011_6930, ee::s00::f_00116930__generated);
    wipe_gated!(wipe_fake_0011fa10, 0x0011_fa10, ee::s00::f_0011fa10__generated);
    wipe_gated!(wipe_fake_0011e098, 0x0011_e098, ee::s00::f_0011e098__generated);
    wipe_gated!(wipe_fake_0011fe78, 0x0011_fe78, ee::s00::f_0011fe78__generated);
    wipe_gated!(wipe_fake_00117838, 0x0011_7838, ee::s00::f_00117838__generated);
    wipe_gated!(wipe_fake_00117948, 0x0011_7948, ee::s00::f_00117948__generated);
    wipe_gated!(wipe_fake_00117b88, 0x0011_7b88, ee::s00::f_00117b88__generated);
    wipe_gated!(wipe_fake_001179e0, 0x0011_79e0, ee::s00::f_001179e0__generated);
    wipe_gated!(wipe_fake_00117908, 0x0011_7908, ee::s00::f_00117908__generated);
    wipe_gated!(wipe_fake_0028b180, 0x0028_b180, ee::s07::f_0028b180__generated);
    wipe_gated!(wipe_fake_00287920, 0x0028_7920, ee::s07::f_00287920__generated);
    wipe_gated!(wipe_fake_00288ae0, 0x0028_8ae0, ee::s07::f_00288ae0__generated);
    wipe_gated!(wipe_fake_00285d98, 0x0028_5d98, ee::s07::f_00285d98__generated);
    wipe_gated!(wipe_fake_00296310, 0x0029_6310, ee::s08::f_00296310__generated);
    wipe_gated!(wipe_fake_00311b48, 0x0031_1b48, ee::s10::f_00311b48__generated);
    wipe_gated!(wipe_fake_0031be50, 0x0031_be50, ee::s10::f_0031be50__generated);
    wipe_gated!(wipe_fake_003128e8, 0x0031_28e8, ee::s10::f_003128e8__generated);
    wipe_gated!(wipe_fake_003142e8, 0x0031_42e8, ee::s10::f_003142e8__generated);
    wipe_gated!(wipe_fake_00313cf0, 0x0031_3cf0, ee::s10::f_00313cf0__generated);
    wipe_gated!(wipe_fake_00314418, 0x0031_4418, ee::s10::f_00314418__generated);
    wipe_gated!(wipe_fake_00314760, 0x0031_4760, ee::s10::f_00314760__generated);
    wipe_gated!(wipe_fake_003115c8, 0x0031_15c8, ee::s10::f_003115c8__generated);
    wipe_gated!(wipe_fake_00314518, 0x0031_4518, ee::s10::f_00314518__generated);
    wipe_gated!(wipe_fake_00314588, 0x0031_4588, ee::s10::f_00314588__generated);
    wipe_gated!(wipe_fake_00313ad8, 0x0031_3ad8, ee::s10::f_00313ad8__generated);
    wipe_gated!(wipe_fake_00313a10, 0x0031_3a10, ee::s10::f_00313a10__generated);
    wipe_gated!(wipe_fake_00313a70, 0x0031_3a70, ee::s10::f_00313a70__generated);
    wipe_gated!(wipe_fake_00313ba8, 0x0031_3ba8, ee::s10::f_00313ba8__generated);
    wipe_gated!(wipe_fake_00313a20, 0x0031_3a20, ee::s10::f_00313a20__generated);
    wipe_gated!(wipe_fake_00313c50, 0x0031_3c50, ee::s10::f_00313c50__generated);
    wipe_gated!(wipe_fake_00313d28, 0x0031_3d28, ee::s10::f_00313d28__generated);
    wipe_gated!(wipe_fake_003145f8, 0x0031_45f8, ee::s10::f_003145f8__generated);
    wipe_gated!(wipe_fake_00313c08, 0x0031_3c08, ee::s10::f_00313c08__generated);
    wipe_gated!(wipe_fake_00317e50, 0x0031_7e50, ee::s10::f_00317e50__generated);
    wipe_gated!(wipe_fake_00311318, 0x0031_1318, ee::s10::f_00311318__generated);

    #[allow(unused_unsafe)]
    fn wipe_install_fakes() {
        unsafe {
            ee::s00::OVERRIDE_0010E098 = wipe_fake_0010e098;
            ee::s00::OVERRIDE_00111AA0 = wipe_fake_00111aa0;
            ee::s00::OVERRIDE_0011FEC8 = wipe_fake_0011fec8;
            ee::s00::OVERRIDE_0011DFE0 = wipe_fake_0011dfe0;
            ee::s00::OVERRIDE_00116930 = wipe_fake_00116930;
            ee::s00::OVERRIDE_0011FA10 = wipe_fake_0011fa10;
            ee::s00::OVERRIDE_0011E098 = wipe_fake_0011e098;
            ee::s00::OVERRIDE_0011FE78 = wipe_fake_0011fe78;
            ee::s00::OVERRIDE_00117838 = wipe_fake_00117838;
            ee::s00::OVERRIDE_00117948 = wipe_fake_00117948;
            ee::s00::OVERRIDE_00117B88 = wipe_fake_00117b88;
            ee::s00::OVERRIDE_001179E0 = wipe_fake_001179e0;
            ee::s00::OVERRIDE_00117908 = wipe_fake_00117908;
            ee::s07::OVERRIDE_0028B180 = wipe_fake_0028b180;
            ee::s07::OVERRIDE_00287920 = wipe_fake_00287920;
            ee::s07::OVERRIDE_00288AE0 = wipe_fake_00288ae0;
            ee::s07::OVERRIDE_00285D98 = wipe_fake_00285d98;
            ee::s08::OVERRIDE_00296310 = wipe_fake_00296310;
            ee::s10::OVERRIDE_00311B48 = wipe_fake_00311b48;
            ee::s10::OVERRIDE_0031BE50 = wipe_fake_0031be50;
            ee::s10::OVERRIDE_003128E8 = wipe_fake_003128e8;
            ee::s10::OVERRIDE_003142E8 = wipe_fake_003142e8;
            ee::s10::OVERRIDE_00313CF0 = wipe_fake_00313cf0;
            ee::s10::OVERRIDE_00314418 = wipe_fake_00314418;
            ee::s10::OVERRIDE_00314760 = wipe_fake_00314760;
            ee::s10::OVERRIDE_003115C8 = wipe_fake_003115c8;
            ee::s10::OVERRIDE_00314518 = wipe_fake_00314518;
            ee::s10::OVERRIDE_00314588 = wipe_fake_00314588;
            ee::s10::OVERRIDE_00313AD8 = wipe_fake_00313ad8;
            ee::s10::OVERRIDE_00313A10 = wipe_fake_00313a10;
            ee::s10::OVERRIDE_00313A70 = wipe_fake_00313a70;
            ee::s10::OVERRIDE_00313BA8 = wipe_fake_00313ba8;
            ee::s10::OVERRIDE_00313A20 = wipe_fake_00313a20;
            ee::s10::OVERRIDE_00313C50 = wipe_fake_00313c50;
            ee::s10::OVERRIDE_00313D28 = wipe_fake_00313d28;
            ee::s10::OVERRIDE_003145F8 = wipe_fake_003145f8;
            ee::s10::OVERRIDE_00313C08 = wipe_fake_00313c08;
            ee::s10::OVERRIDE_00317E50 = wipe_fake_00317e50;
            ee::s10::OVERRIDE_00311318 = wipe_fake_00311318;
        }
    }

    fn wipe_vec(c: &mut Cpu, r: &mut Rng, at: u32, range: f32) {
        put_vec(c, at, [r.f32(range), r.f32(range), r.f32(range), r.f32(range)]);
    }

    /// Rider_StartWipeout: rail style 3, 4 or another (the two quarter-turn
    /// arms and none); the metrics flag zero or not (RiderMetrix_OnWipeout
    /// runs for real either way: score banking or the HUD number). The
    /// unslotted audio callees run for real to their early exits (their
    /// queries are fakes answering non-zero), the animation setup
    /// (`0x312660`, with the real `AnimController_PlayAnimImpl` and
    /// `AnimSeqList_Clear` on an empty list: fp = 0 is what the speech call
    /// leaves in that stack slot), the bone sampler (no channels: the
    /// identity transform at 0x4ff230, randomised) and the orientation
    /// offset (six empty layers) also run for real. Every slotted callee is
    /// a logged fake, one of which may unwind.
    #[test]
    fn rider_start_wipeout_matches() {
        wipe_install_fakes();
        covered(ee::s00::f_0010eb30__generated, rider_start_wipeout, 200, &|c, seed| {
            wipe_install_fakes();
            let mut r = machine(c, seed, 0x0010_eb30, 50);
            c.pc = 0x0010_eb30;
            c.bus.write32(WT_CTRL, WT_MAGIC);
            c.g[30] = [0, 0];

            // gp: +-pi/2, the blend id (0, != gp+0x2860), the anim table.
            let gp = GAME_GP;
            c.bus.write32(gp - WIPE_GP_HALF_PI, std::f32::consts::FRAC_PI_2.to_bits());
            c.bus.write32(gp - WIPE_GP_NEG_HALF_PI, (-std::f32::consts::FRAC_PI_2).to_bits());
            c.bus.write32(gp + WIPE_GP_BLEND, 0);
            c.bus.write32(gp + 0x2860, 1);
            c.bus.write32(gp + 0xd08, WT_ANIM_TABLE);
            c.bus.write32(WT_ANIM_TABLE, WT_ANIM_SET);
            c.bus.write32(WT_ANIM_SET + 8, WT_ANIM_DEFS);

            // .bss vectors: zero quad, up axis, identity transform.
            wipe_vec(c, &mut r, WIPE_ZERO, 2.0);
            wipe_vec(c, &mut r, WIPE_AXIS, 1.0);
            wipe_vec(c, &mut r, 0x004f_f230, 2.0);
            wipe_vec(c, &mut r, 0x004f_f240, 1.0);

            // The rider.
            let rider = WT_RIDER;
            let vt = fakes::vtable(c, &mut r, &[WIPE_VT_VIBRATION_BUMP]);
            c.bus.write32(rider + WIPE_VTABLE, vt);
            wipe_vec(c, &mut r, rider + WIPE_POSITION, 100.0);
            wipe_vec(c, &mut r, rider + WIPE_ORIENT, 1.0);
            wipe_vec(c, &mut r, rider + WIPE_UP, 1.0);
            for off in (0x1f0..0x260).step_by(4) {
                c.bus.write32(rider + off, r.f32(5.0));
            }
            let style = match seed % 3 {
                0 => 3,
                1 => 4,
                _ => [0, 1, 2, 5][(r.next_u32() % 4) as usize],
            };
            c.bus.write32(rider + WIPE_RAIL_STYLE, style);
            c.bus.write32(rider + WIPE_PRESS_TYPE, r.next_u32());
            c.bus.write32(rider + WIPE_HANDLER, r.next_u32());
            c.bus.write32(rider + WIPE_ANIM_MODEL, WT_MODEL);
            c.bus.write32(rider + WIPE_ANIM, WT_ANIM);
            c.bus.write32(rider + WIPE_METRIX, WT_METRIX);
            let bone = r.next_u32() % 8;
            c.bus.write32(rider + WIPE_BONE_HIPS, bone);

            // Impact info, metrics.
            for i in 0..0x10 {
                c.bus.write32(WT_EVT + 4 * i, r.f32(50.0));
            }
            for i in 0..0x80 {
                c.bus.write32(WT_METRIX + 4 * i, r.next_u32() % 2000);
            }

            // Animation controller (layer list of six empty layers), model.
            for i in 0..0x14 {
                c.bus.write32(WT_ANIM + 4 * i, r.f32(3.0));
            }
            c.bus.write32(WT_ANIM + 0x50, WT_LAYERS);
            c.bus.write32(WT_MODEL + WIPE_MODEL_BONE_POS, WT_BONE_POS);
            c.bus.write32(WT_MODEL + WIPE_MODEL_BONE_ROT, WT_BONE_ROT);
            wipe_vec(c, &mut r, WT_MODEL + WIPE_MODEL_SCALE, 2.0);
            for b in 0..8 {
                wipe_vec(c, &mut r, WT_BONE_POS + 16 * b, 50.0);
                wipe_vec(c, &mut r, WT_BONE_ROT + 16 * b, 1.0);
            }
            c.bus.write32(WT_ENTRY + 0x18, r.f32(2.0));

            // Arguments.
            c.g[A0][0] = sx(rider);
            c.g[A1][0] = (r.next_u32() % 0x200) as u64;
            let flag = if seed % 2 == 0 { 0 } else { 1 + r.next_u32() % 50 };
            c.g[A2][0] = flag as u64;
            c.g[A3][0] = r.next_u32() as u64;
            c.g[T0][0] = sx(WT_EVT);
        });
    }
}
