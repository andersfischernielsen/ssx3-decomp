//! Staging wave 209 (oc276): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, VSlot};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;
use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0037_0058, emitter_build_descriptor, "Emitter_BuildDescriptor"),
    (0x002a_b028, stream_array_is_playing, "StreamArray_IsPlaying"),
    (0x002a_ebd0, path_plat_get_track_state, "PATH_PlatGetTrackState"),
    (0x0011_1578, rider_base_exit_sub_state, "RiderBase_ExitSubState"),
    (0x0026_afb8, path_data_find_near_ai_paths, "PathData_FindNearAIPaths"),
    (0x0034_5890, particle_modifier_base_save, "ParticleModifier_BaseSave"),
    (0x0013_7750, rider_wipeout_state_update_air, "RiderWipeoutState_UpdateAir"),
    (0x003d_d648, ea_file_queue_read, "EAFile_QueueRead"),
    (0x0029_3740, ssx_audio_system_calc_surface_loop_group1, "SSXAudioSystem_CalcSurfaceLoopGroup1"),
    (0x0031_3a70, anim_sequence_add_trigger, "AnimSequence_AddTrigger"),
];

// ---------------------------------------------------------------------------
// Emitter_BuildDescriptor.

/// `base + off` for the register `base` (32-bit address arithmetic).
#[inline(always)]
fn emitter_bd_at(c: &Cpu, base: usize, off: u32) -> u32 {
    (c.g[base][0] as u32).wrapping_add(off)
}

/// Load the three floats at `params + src` into f0, f1, f2 and store them to
/// `sp + dst` (x, y, z; the w word is left alone).
fn emitter_bd_stage_xyz(c: &mut Cpu, src: u32, dst: u32) {
    c.lwc1(0, emitter_bd_at(c, S0, src));
    c.lwc1(1, emitter_bd_at(c, S0, src + 4));
    c.lwc1(2, emitter_bd_at(c, S0, src + 8));
    c.swc1(0, emitter_bd_at(c, SP, dst));
    c.swc1(1, emitter_bd_at(c, SP, dst + 4));
    c.swc1(2, emitter_bd_at(c, SP, dst + 8));
}

/// `vf12 = M * v` for the 4x4 matrix at `s2` and the vector at `sp + vec`
/// (VU0 mula/madda/madda/madd), stored to `sp + out`.
fn emitter_bd_apply_matrix(c: &mut Cpu, vec: u32, out: u32) {
    c.lqc2(8, emitter_bd_at(c, SP, vec));
    c.lqc2(4, emitter_bd_at(c, S2, 0));
    c.lqc2(5, emitter_bd_at(c, S2, 0x10));
    c.lqc2(6, emitter_bd_at(c, S2, 0x20));
    c.lqc2(7, emitter_bd_at(c, S2, 0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, emitter_bd_at(c, SP, out));
}

/// The emitter lifetime in the parameter block.
const EMITTER_PARAM_LIFETIME: u32 = 0x0c;

/// A position-like vector of the parameter block (`params + src`, w = 0)
/// transformed by the matrix and divided by the emitter lifetime
/// (`params + 0xc`, VU0 `div`/`mul Q`), stored to `self + dst` (and to the
/// stack scratch at `sp`). `clear_w` is the compiler's redundant re-clear
/// of the staged w word.
fn emitter_bd_xform_div(c: &mut Cpu, src: u32, dst: u32, clear_w: bool) {
    emitter_bd_stage_xyz(c, src, 0x20);
    emitter_bd_apply_matrix(c, 0x20, 0x30);
    c.lq(2, emitter_bd_at(c, SP, 0x30));
    c.lwc1(0, emitter_bd_at(c, S0, EMITTER_PARAM_LIFETIME));
    c.sq(2, emitter_bd_at(c, SP, 0x10));
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, emitter_bd_at(c, SP, 0x10));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, emitter_bd_at(c, SP, 0x30));
    c.lq(2, emitter_bd_at(c, SP, 0x30));
    if clear_w {
        c.sw(emitter_bd_at(c, SP, 0x2c), 0);
    }
    c.sq(2, emitter_bd_at(c, S1, dst));
    c.sq(2, emitter_bd_at(c, SP, 0));
}

/// VU0 `vf5 = vf3 op vf4` through the stack scratch: `sqc2 vf5, tmp;
/// lq v0, tmp; sq v0, dst` (the compiler's copy of a returned vector).
fn emitter_bd_vec_copy(c: &mut Cpu, tmp: u32, dst: u32) {
    c.sqc2(5, emitter_bd_at(c, SP, tmp));
    c.lq(2, emitter_bd_at(c, SP, tmp));
    c.sq(2, emitter_bd_at(c, SP, dst));
}

/// `vf5 = vf4 * (bits of FPU reg fpr).x` with the scalar moved through the
/// GPR `gpr` (`mfc1`, `qmtc2` into vf3), the vector loaded from `sp + vec`.
fn emitter_bd_scale(c: &mut Cpu, gpr: usize, fpr: usize, vec: u32) {
    c.mfc1(gpr, fpr);
    c.lqc2(4, emitter_bd_at(c, SP, vec));
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
}

/// `void Emitter_BuildDescriptor(Emitter* self, Matrix* m, EmitterParams* p,
/// Box* bounds, float time)` (0x370058): fill the emitter's particle
/// descriptor from its parameter block. Sets the two spawn curves
/// (`Emitter_SetDescCurveA/B`), the start time (`time`, or random seeds when
/// `time < 0`), resets the acceleration, transforms the spawn position and
/// two spread axes by `m` (`+0xb0..+0xd0`, the position re-centred by
/// `1.5 * (axis1 + axis2)`), the four velocity vectors divided by the
/// lifetime (`+0x70..+0xa0`, the first re-centred by `1.5 * (v1+v2+v3)`),
/// the four colours scaled by 128 (`Emitter_SetDescVelocityCurve`), the
/// time (`+0x14 + 0.5 * +0x1c` when `p+8 < 0`, else 0) and the bounds.
pub fn emitter_build_descriptor(c: &mut Cpu) {
    guest(c, |c| {
        const PARAM_TIME_SIGN: u32 = 0x08;
        const PARAM_CURVE_X: u32 = 0x10;
        const PARAM_CURVE_START: u32 = 0x14;
        const PARAM_CURVE_Z: u32 = 0x18;
        const PARAM_CURVE_SPAN: u32 = 0x1c;
        const PARAM_CURVE_Y: u32 = 0xcc;
        const SELF_START_TIME: u32 = 0x38;

        c.cycles += 30;
        let frame = Frame::open(c, 0xf0);
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        frame.save_fpr(c, 20, 0xe0);
        frame.save_ra(c, 0x90);
        c.g[S0][0] = c.g[A2][0]; // params
        c.g[S1][0] = c.g[A0][0]; // self
        c.g[S2][0] = c.g[A1][0]; // matrix
        c.g[S3][0] = c.g[A3][0]; // bounds
        c.mov_s(20, 12); // time

        // Curve A (x, y, z of the parameter curve).
        c.lwc1(12, emitter_bd_at(c, S0, PARAM_CURVE_X));
        c.lwc1(14, emitter_bd_at(c, S0, PARAM_CURVE_Z));
        c.lwc1(13, emitter_bd_at(c, S0, PARAM_CURVE_Y));
        Callee { addr: 0x0036_ce00, f: ee::s12::f_0036ce00 }.call(c, 0x0037_0098)?;

        // Curve B: start -/+ half the span.
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 0);
        c.g[A0][0] = c.g[S1][0];
        c.lwc1(12, emitter_bd_at(c, S0, PARAM_CURVE_SPAN));
        c.lwc1(1, emitter_bd_at(c, S0, PARAM_CURVE_START));
        c.mul_s(12, 12, 0);
        c.add_s(13, 1, 12);
        c.sub_s(12, 1, 12);
        Callee { addr: 0x0036_ce28, f: ee::s12::f_0036ce28 }.call(c, 0x0037_00bc)?;

        // Start time, or random seeds for a negative time.
        c.mtc1(0, 0);
        c.c_lt_s(20, 0);
        if !c.fcc() {
            c.swc1(20, emitter_bd_at(c, S1, SELF_START_TIME));
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            Callee { addr: 0x0036_ccb8, f: ee::s12::f_0036ccb8 }.call(c, 0x0037_00d8)?;
        }

        c.cycles += 305;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 20); // f20 = 0.5 from here on
        c.g[A0][0] = c.g[S1][0];
        Callee { addr: 0x0036_d008, f: ee::s12::f_0036d008 }.call(c, 0x0037_00e8)?;

        // Spawn position (w = 1) through the matrix -> self+0xb0.
        c.lwc1(2, emitter_bd_at(c, S0, 0x24));
        c.g[A0][0] = c.g[S1][0];
        c.lwc1(3, emitter_bd_at(c, S0, 0x28));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(1, emitter_bd_at(c, S0, 0x2c));
        c.g[A2][0] = sx(emitter_bd_at(c, SP, 0x20));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.g[A3][0] = sx(emitter_bd_at(c, SP, 0x40));
        c.swc1(3, emitter_bd_at(c, SP, 0x14));
        c.g[T0][0] = sx(emitter_bd_at(c, SP, 0x60));
        c.swc1(2, emitter_bd_at(c, SP, 0x10));
        c.swc1(1, emitter_bd_at(c, SP, 0x18));
        c.swc1(0, emitter_bd_at(c, SP, 0x1c));
        emitter_bd_apply_matrix(c, 0x10, 0x20);
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(AT, 3); // f3 = 1.5
        c.lq(2, emitter_bd_at(c, SP, 0x20));
        c.sw(emitter_bd_at(c, SP, 0x1c), 0);
        c.sq(2, emitter_bd_at(c, S1, 0xb0));
        c.sq(2, emitter_bd_at(c, SP, 0));
        c.g[AT][0] = 0x4300_0000;
        c.mtc1(AT, 4); // f4 = 128.0

        // The two spread axes (w = 0) -> self+0xc0, self+0xd0.
        emitter_bd_stage_xyz(c, 0x30, 0x10);
        emitter_bd_apply_matrix(c, 0x10, 0x20);
        c.lq(2, emitter_bd_at(c, SP, 0x20));
        c.sw(emitter_bd_at(c, SP, 0x1c), 0);
        c.sq(2, emitter_bd_at(c, S1, 0xc0));
        c.sq(2, emitter_bd_at(c, SP, 0));
        emitter_bd_stage_xyz(c, 0x3c, 0x10);
        emitter_bd_apply_matrix(c, 0x10, 0x20);
        c.lq(2, emitter_bd_at(c, SP, 0x20));
        c.sq(2, emitter_bd_at(c, S1, 0xd0));
        c.sq(2, emitter_bd_at(c, SP, 0));

        // position -= 1.5 * (axis1 + axis2).
        c.lqc2(3, emitter_bd_at(c, S1, 0xc0));
        c.lqc2(4, emitter_bd_at(c, S1, 0xd0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        emitter_bd_vec_copy(c, 0x20, 0x10);
        emitter_bd_scale(c, 3, 3, 0x10);
        emitter_bd_vec_copy(c, 0x20, 0);
        c.lqc2(3, emitter_bd_at(c, S1, 0xb0));
        c.lqc2(4, emitter_bd_at(c, SP, 0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, emitter_bd_at(c, S1, 0xb0));

        // The four velocity vectors / lifetime -> self+0x70..+0xa0.
        c.sw(emitter_bd_at(c, SP, 0x2c), 0);
        emitter_bd_xform_div(c, 0x48, 0x70, true);
        emitter_bd_xform_div(c, 0x54, 0x80, true);
        emitter_bd_xform_div(c, 0x60, 0x90, true);
        emitter_bd_xform_div(c, 0x6c, 0xa0, false);

        // v1 -= 1.5 * (v2 + v3 + v4).
        c.lqc2(3, emitter_bd_at(c, S1, 0x80));
        c.lqc2(4, emitter_bd_at(c, S1, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        emitter_bd_vec_copy(c, 0x30, 0x20);
        c.lqc2(3, emitter_bd_at(c, SP, 0x20));
        c.lqc2(4, emitter_bd_at(c, S1, 0xa0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        emitter_bd_vec_copy(c, 0x30, 0x10);
        emitter_bd_scale(c, 3, 3, 0x10);
        emitter_bd_vec_copy(c, 0x30, 0);
        c.lqc2(3, emitter_bd_at(c, S1, 0x70));
        c.lqc2(4, emitter_bd_at(c, SP, 0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, emitter_bd_at(c, S1, 0x70));

        // The four colours (w first in the block) * 128 -> sp, sp+0x20,
        // sp+0x40, sp+0x60 (the velocity-curve arguments).
        c.lwc1(0, emitter_bd_at(c, S0, 0x88));
        c.lwc1(1, emitter_bd_at(c, S0, 0x8c));
        c.lwc1(2, emitter_bd_at(c, S0, 0x90));
        c.lwc1(3, emitter_bd_at(c, S0, 0x84));
        c.swc1(0, emitter_bd_at(c, SP, 0x10));
        c.swc1(1, emitter_bd_at(c, SP, 0x14));
        c.swc1(2, emitter_bd_at(c, SP, 0x18));
        c.swc1(3, emitter_bd_at(c, SP, 0x1c));
        emitter_bd_scale(c, 3, 4, 0x10);
        emitter_bd_vec_copy(c, 0x20, 0);

        c.lwc1(0, emitter_bd_at(c, S0, 0x98));
        c.lwc1(1, emitter_bd_at(c, S0, 0x9c));
        c.lwc1(2, emitter_bd_at(c, S0, 0xa0));
        c.lwc1(3, emitter_bd_at(c, S0, 0x94));
        c.swc1(0, emitter_bd_at(c, SP, 0x30));
        c.swc1(1, emitter_bd_at(c, SP, 0x34));
        c.swc1(2, emitter_bd_at(c, SP, 0x38));
        c.swc1(3, emitter_bd_at(c, SP, 0x3c));
        emitter_bd_scale(c, 3, 4, 0x30);
        c.sqc2(5, emitter_bd_at(c, SP, 0x40));

        c.lwc1(0, emitter_bd_at(c, S0, 0xa8));
        c.lwc1(1, emitter_bd_at(c, S0, 0xac));
        c.lwc1(2, emitter_bd_at(c, S0, 0xb0));
        c.lwc1(3, emitter_bd_at(c, S0, 0xa4));
        c.lq(2, emitter_bd_at(c, SP, 0x40));
        c.swc1(0, emitter_bd_at(c, SP, 0x50));
        c.sq(2, emitter_bd_at(c, SP, 0x20));
        c.swc1(1, emitter_bd_at(c, SP, 0x54));
        c.swc1(2, emitter_bd_at(c, SP, 0x58));
        c.swc1(3, emitter_bd_at(c, SP, 0x5c));
        emitter_bd_scale(c, 3, 4, 0x50);
        c.sqc2(5, emitter_bd_at(c, SP, 0x60));
        c.lq(2, emitter_bd_at(c, SP, 0x60));

        c.lwc1(0, emitter_bd_at(c, S0, 0xb8));
        c.lwc1(2, emitter_bd_at(c, S0, 0xbc));
        c.sq(2, emitter_bd_at(c, SP, 0x40));
        c.swc1(0, emitter_bd_at(c, SP, 0x70));
        c.lwc1(0, emitter_bd_at(c, S0, 0xb4));
        c.lwc1(1, emitter_bd_at(c, S0, 0xc0));
        c.swc1(2, emitter_bd_at(c, SP, 0x74));
        c.swc1(1, emitter_bd_at(c, SP, 0x78));
        c.swc1(0, emitter_bd_at(c, SP, 0x7c));
        emitter_bd_scale(c, 2, 4, 0x70);
        c.sqc2(5, emitter_bd_at(c, SP, 0x80));

        // Velocity curve over start + 0.5 * span.
        c.lwc1(12, emitter_bd_at(c, S0, PARAM_CURVE_SPAN));
        c.lwc1(0, emitter_bd_at(c, S0, PARAM_CURVE_START));
        c.mul_s(12, 12, 20);
        c.lq(2, emitter_bd_at(c, SP, 0x80));
        c.sq(2, emitter_bd_at(c, SP, 0x60));
        c.add_s(12, 0, 12);
        Callee { addr: 0x0036_d318, f: ee::s12::f_0036d318 }.call(c, 0x0037_0588)?;

        // Time: start + 0.5 * span when p+8 < 0, else 0.
        c.mtc1(0, 12);
        c.lwc1(0, emitter_bd_at(c, S0, PARAM_TIME_SIGN));
        c.c_lt_s(0, 12);
        if c.fcc() {
            c.cycles += 4;
            c.lwc1(0, emitter_bd_at(c, S0, PARAM_CURVE_SPAN));
            c.lwc1(1, emitter_bd_at(c, S0, PARAM_CURVE_START));
            c.mul_s(0, 0, 20);
            c.add_s(12, 1, 0);
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S1][0];
        Callee { addr: 0x0036_d3d8, f: ee::s12::f_0036d3d8 }.call(c, 0x0037_05b4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        Callee { addr: 0x0036_d500, f: ee::s12::f_0036d500 }.call(c, 0x0037_05c0)?;

        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.restore_fpr(c, 20, 0xe0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamArray_IsPlaying.

/// `StreamArray::buffers`: the array of `StreamBuffer*` slots.
const STREAM_ARRAY_BUFFERS: u32 = 0x8;

/// `bool StreamArray_IsPlaying(StreamArray* self, int i)` (0x2ab028): whether
/// stream slot `i` holds a buffer that is playing (`StreamBuffer_IsPlaying`);
/// false for an empty slot.
pub fn stream_array_is_playing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(STREAM_ARRAY_BUFFERS)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 2;
            Callee { addr: 0x002a_a1b8, f: ee::s08::f_002aa1b8 }.call(c, 0x002a_b050)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PATH_PlatGetTrackState.

/// `PATH_PlatTrack* D_004D3E98[]`: the platform track table.
const PATH_PLAT_TRACKS: u32 = 0x004d_3e98;
/// `PATH_PlatTrack::dirty` (s8): the status needs a refresh.
const PATH_PLAT_TRACK_DIRTY: u32 = 0x65;
/// `PATH_PlatTrack::state`.
const PATH_PLAT_TRACK_STATE: u32 = 0x58;

/// `int PATH_PlatGetTrackState(int idx)` (0x2aebd0): the state word of
/// platform track `idx`, refreshing the status first
/// (`PATH_PlatRefreshStatus(0)`) when the track is marked dirty; -8 for an
/// empty slot.
pub fn path_plat_get_track_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(PATH_PLAT_TRACKS));
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = sx(-8i32 as u32);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(PATH_PLAT_TRACK_DIRTY)).lb(c);
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = 0;
                Callee { addr: 0x002a_efa8, f: ee::s08::f_002aefa8 }.call(c, 0x002a_ec0c)?;
            }
            // Re-read the slot (the refresh may replace the track).
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.cycles += 1;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(PATH_PLAT_TRACK_STATE)).lw(c);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderBase_ExitSubState.

/// `Rider::mSubState`.
const RIDER_SUB_STATE: u32 = 0xde4;
/// The `switch (mSubState)` jump table (`.rodata`, 14 entries).
const RIDER_EXIT_SUB_STATE_TABLE: u32 = 0x0045_6b90;

/// How a sub state's exit handler is called.
#[derive(Clone, Copy)]
enum RiderExitSubCall {
    Direct(Callee),
    Table(u32),
}

/// One `case` of the exit switch: its block address (the table target), the
/// sub-object offset in the rider, the handler and its return address.
struct RiderExitSubCase {
    block: u32,
    off: u32,
    handler: RiderExitSubCall,
    ret: u32,
}

/// The sub states with an exit handler (cruise, board press, prewind,
/// natural air, 5, 7, wipeout, 11), in block order.
const RIDER_EXIT_SUB_CASES: [RiderExitSubCase; 8] = [
    RiderExitSubCase {
        block: 0x0011_15ac,
        off: 0x1c0,
        handler: RiderExitSubCall::Direct(Callee { addr: 0x0013_1c30, f: ee::s00::f_00131c30 }),
        ret: 0x0011_15b4,
    },
    RiderExitSubCase {
        block: 0x0011_15bc,
        off: 0x1d0,
        handler: RiderExitSubCall::Direct(Callee { addr: 0x0012_fe98, f: ee::s00::f_0012fe98 }),
        ret: 0x0011_15c4,
    },
    RiderExitSubCase {
        block: 0x0011_15cc,
        off: 0x1f0,
        handler: RiderExitSubCall::Direct(Callee { addr: 0x0012_e9b0, f: ee::s00::f_0012e9b0 }),
        ret: 0x0011_15d4,
    },
    RiderExitSubCase {
        block: 0x0011_15dc,
        off: 0x210,
        handler: RiderExitSubCall::Direct(Callee { addr: 0x0012_fb68, f: ee::s00::f_0012fb68 }),
        ret: 0x0011_15e4,
    },
    RiderExitSubCase {
        block: 0x0011_15ec,
        off: 0x230,
        handler: RiderExitSubCall::Table(0x0013_4cb0),
        ret: 0x0011_15f4,
    },
    RiderExitSubCase {
        block: 0x0011_15fc,
        off: 0x2b0,
        handler: RiderExitSubCall::Table(0x0013_2048),
        ret: 0x0011_1604,
    },
    RiderExitSubCase {
        block: 0x0011_160c,
        off: 0x2c0,
        handler: RiderExitSubCall::Direct(Callee { addr: 0x0012_e690, f: ee::s00::f_0012e690 }),
        ret: 0x0011_1614,
    },
    RiderExitSubCase {
        block: 0x0011_161c,
        off: 0x370,
        handler: RiderExitSubCall::Table(0x0013_2f98),
        ret: 0x0011_1624,
    },
];

/// `void RiderBase_ExitSubState(Rider* self)` (0x111578): run the exit
/// handler of the rider's current sub state (`mSubState`, 0xde4) on its
/// sub-object; sub states without one (3, 6, 9, 10, 12, 13, out of range)
/// do nothing. The switch goes through the `.rodata` jump table.
pub fn rider_base_exit_sub_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(RIDER_SUB_STATE)).lw(c);
        let in_table = c.g[V1][0] < 14;
        c.g[V0][0] = 0x0045_0000;
        let mut tail = !in_table;
        if in_table {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx(RIDER_EXIT_SUB_STATE_TABLE);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let target = c.g[A0][0] as u32;
            if target == 0x0011_1624 {
                // No handler.
                tail = true;
            } else if let Some(case) = RIDER_EXIT_SUB_CASES.iter().find(|k| k.block == target) {
                let last = case.block == 0x0011_161c;
                c.cycles += if last { 2 } else { 4 };
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(case.off));
                match case.handler {
                    RiderExitSubCall::Direct(f) => f.call(c, case.ret)?,
                    RiderExitSubCall::Table(addr) => call_ptr(c, addr, case.ret)?,
                }
                if last {
                    tail = true;
                } else {
                    frame.restore_ra(c, 0);
                }
            } else {
                c.jump_out(target, 0x0011_1578, 0x0011_1630);
                return Ok(());
            }
        }
        if tail {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PathData_FindNearAIPaths.

/// `PathData::aiPathCount`.
const PATH_DATA_AI_COUNT: u32 = 0x8;
/// `PathData::aiPaths` (0x40-byte `AIPath` records).
const PATH_DATA_AI_PATHS: u32 = 0xc;
/// `AIPath::enabled` (only checked when the caller asks for enabled paths).
const AI_PATH_ENABLED: u32 = 0x3c;
/// The caller's point, spilled at `sp+0x320`.
const FIND_NEAR_AI_POINT_SLOT: u32 = 0x320;

/// `s1 = s1 + n` etc.: 32-bit add, sign-extended.
#[inline(always)]
fn find_near_ai_add(v: u64, n: u32) -> u64 {
    sx((v as u32).wrapping_add(n))
}

/// `int PathData_FindNearAIPaths(PathData* d, const Vec3* p, AIPath** out,
/// int max, bool enabledOnly)` (0x26afb8): collect up to `max` AI paths
/// near `p` into `out`. First pass: the bounds distance of every path
/// (`Path_BoundsDistance`, kept in a stack array; disabled paths count as
/// "far" (`gp-0x4a4c`) when `enabledOnly`); paths containing `p` (distance
/// 0) are taken at once. If fewer than `max` were found, the rest are the
/// nearest remaining paths in order (selection by smallest distance, each
/// pick marked used with `gp-0x4a48`). Returns the number written.
pub fn path_data_find_near_ai_paths(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x3e0);
        frame.save(c, S0, 0x3c0);
        frame.save(c, S2, 0x3a0);
        c.g[S0][0] = 0; // found
        frame.save(c, S4, 0x380);
        c.g[S2][0] = 0; // path index
        frame.save(c, S6, 0x360);
        c.g[S4][0] = c.g[A0][0]; // data
        frame.save(c, S7, 0x350);
        frame.save(c, FP, 0x340);
        c.g[S7][0] = c.g[A2][0]; // out
        frame.save(c, S1, 0x3b0);
        c.g[FP][0] = c.g[T0][0]; // enabledOnly
        frame.save(c, S3, 0x390);
        frame.save(c, S5, 0x370);
        frame.save_ra(c, 0x330);
        frame.save_fpr(c, 21, 0x3d8);
        frame.save_fpr(c, 20, 0x3d0);
        let point_slot = frame.at(c, FIND_NEAR_AI_POINT_SLOT);
        c.sw(point_slot, c.g[A1][0]);
        c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(PATH_DATA_AI_COUNT)).lw(c);
        c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
        // limit = min(count, max)
        c.g[S6][0] = if c.g[V1][0] == 0 { c.g[A3][0] } else { c.g[V0][0] };

        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 6;
            c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x4a4c)); // "far"
            c.g[S1][0] = c.g[SP][0]; // distance slot
            c.mtc1(0, 21); // 0.0
            c.g[S3][0] = 0; // path offset
            c.g[S5][0] = c.g[S7][0]; // out cursor
            loop {
                c.cycles += 6;
                c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(PATH_DATA_AI_PATHS)).lw(c);
                c.g[A1][0] = Addr(sp(c).wrapping_add(FIND_NEAR_AI_POINT_SLOT)).lw(c);
                c.g[A0][0] = find_near_ai_add(c.g[A0][0], c.g[S3][0] as u32);
                Callee { addr: 0x0026_a9b0, f: ee::s07::f_0026a9b0 }.call(c, 0x0026_b040)?;
                c.swc1(0, c.g[S1][0] as u32);
                if c.g[FP][0] != 0 {
                    c.cycles += 5;
                    c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(PATH_DATA_AI_PATHS)).lw(c);
                    c.g[V0][0] = find_near_ai_add(c.g[S3][0], c.g[V0][0] as u32);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(AI_PATH_ENABLED)).lw(c);
                    if c.g[V1][0] == 0 {
                        c.swc1(20, c.g[S1][0] as u32);
                    }
                }
                c.cycles += 5;
                c.lwc1(0, c.g[S1][0] as u32);
                c.c_eq_s(0, 21);
                if c.fcc() {
                    // Inside the bounds: take it now.
                    c.cycles += 7;
                    c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(PATH_DATA_AI_PATHS)).lw(c);
                    c.g[S0][0] = find_near_ai_add(c.g[S0][0], 1);
                    c.swc1(20, c.g[S1][0] as u32);
                    c.g[V0][0] = find_near_ai_add(c.g[V0][0], c.g[S3][0] as u32);
                    c.sw(c.g[S5][0] as u32, c.g[V0][0]);
                    let full = c.g[S0][0] == c.g[S6][0];
                    c.g[S5][0] = find_near_ai_add(c.g[S5][0], 4);
                    if full {
                        break;
                    }
                    c.cycles += 1;
                }
                c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(PATH_DATA_AI_COUNT)).lw(c);
                c.cycles += 5;
                c.g[S2][0] = find_near_ai_add(c.g[S2][0], 1);
                c.g[S1][0] = find_near_ai_add(c.g[S1][0], 4);
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = find_near_ai_add(c.g[S3][0], 0x40);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_b030);
                }
            }
        }

        // Fill the rest with the nearest remaining paths.
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S6][0] as i64)) as u64;
        let short = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S0][0];
        if short {
            c.cycles += 2;
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4a48)); // "used"
            loop {
                c.cycles += 8;
                c.g[V1][0] = Addr((c.g[S4][0] as u32).wrapping_add(PATH_DATA_AI_COUNT)).lw(c);
                c.g[S2][0] = 1;
                c.g[A3][0] = find_near_ai_add(c.g[S0][0], 1);
                c.lwc1(1, sp(c)); // best = dist[0]
                c.g[A0][0] = 0; // best index
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[A2][0] = sx((c.g[S0][0] as u32) << 2);
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[V1][0];
                    c.g[V1][0] = sx(sp(c).wrapping_add(4));
                    loop {
                        c.cycles += 5;
                        c.lwc1(0, c.g[V1][0] as u32);
                        c.c_lt_s(0, 1);
                        if c.fcc() {
                            c.cycles += 3;
                            c.mov_s(1, 0);
                            c.g[A0][0] = c.g[S2][0];
                        }
                        c.g[S2][0] = find_near_ai_add(c.g[S2][0], 1);
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[A1][0] as i64)) as u64;
                        c.g[V1][0] = find_near_ai_add(c.g[V1][0], 4);
                        if c.g[V0][0] == 0 {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0026_b0e0);
                        }
                    }
                }
                c.cycles += 11;
                c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(PATH_DATA_AI_PATHS)).lw(c);
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 6);
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[A2][0] = find_near_ai_add(c.g[A2][0], c.g[S7][0] as u32);
                c.g[V0][0] = find_near_ai_add(c.g[V0][0], c.g[V1][0] as u32);
                c.g[A1][0] = find_near_ai_add(c.g[SP][0], c.g[A0][0] as u32);
                c.sw(c.g[A2][0] as u32, c.g[V0][0]);
                c.g[S0][0] = c.g[A3][0];
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.swc1(2, c.g[A1][0] as u32);
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S0][0];
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_b0b8);
                }
            }
        }

        c.cycles += 14;
        frame.restore(c, S1, 0x3b0);
        frame.restore(c, S0, 0x3c0);
        frame.restore(c, S2, 0x3a0);
        frame.restore(c, S3, 0x390);
        frame.restore(c, S4, 0x380);
        frame.restore(c, S5, 0x370);
        frame.restore(c, S6, 0x360);
        frame.restore(c, S7, 0x350);
        frame.restore(c, FP, 0x340);
        frame.restore_ra(c, 0x330);
        frame.restore_fpr(c, 21, 0x3d8);
        frame.restore_fpr(c, 20, 0x3d0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ParticleModifier_BaseSave.

/// `ParticleModifier::type` (the word the base class saves).
const PARTICLE_MODIFIER_TYPE: u32 = 0xc;
/// The stream's "write word" virtual method slot.
const STREAM_VT_WRITE_WORD: u32 = 0x28;

/// `void ParticleModifier_BaseSave(ParticleModifier* self, Stream* s)`
/// (0x345890): the base part of a modifier's save: write the modifier's
/// type word (`+0xc`) through the stream's virtual write (slot 0x28).
pub fn particle_modifier_base_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A1][0]; // stream
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c); // vtable
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(PARTICLE_MODIFIER_TYPE)).lw(c);
        let vt = c.g[V0][0];
        let slot = VSlot::read(c, vt, STREAM_VT_WRITE_WORD);
        c.g[A0][0] = slot.delta;
        c.g[V1][0] = slot.func;
        c.g[A0][0] = slot.this(c.g[A2][0]);
        call_ptr(c, slot.func as u32, 0x0034_58b4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderWipeoutState_UpdateAir.

/// `WipeoutState::spin` (the tumble vector damped every tick).
const WIPEOUT_STATE_SPIN: u32 = 0x30;
/// `WipeoutState::rider`.
const WIPEOUT_STATE_RIDER: u32 = 0x40;
/// `Rider::timeScale` (ticks this frame).
const WIPEOUT_RIDER_TIME_SCALE: u32 = 0x300;
/// `Rider::airPredictor`.
const WIPEOUT_RIDER_AIR_PREDICTOR: u32 = 0x788;
/// `AirPredictor::mode` (1 and 3 are ballistic flights).
const WIPEOUT_AP_MODE: u32 = 0xac;

/// `void RiderWipeoutState_UpdateAir(WipeoutState* self)` (0x137750): one
/// airborne tick of a wipeout. Advances the rider's air predictor by
/// `timeScale * (1/60)` (`AirPredictor_Update` on position +0x110 and
/// velocity +0x1e0); in a ballistic flight (mode 1 or 3) re-aims the rider
/// along the predicted path (`Rider_UpdateOrientation` with the time left,
/// `timeScale / (end - now)` or `* 60` when under a tick, and the turn
/// rate `timeScale * gp-0x726c`); damps the tumble by `1 - 1.5 dt` and
/// updates the implicit orientation.
pub fn rider_wipeout_state_update_air(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7278)); // 1/60
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let rider = Addr((c.g[S0][0] as u32).wrapping_add(WIPEOUT_STATE_RIDER)).lw(c);
        c.g[A1][0] = rider;
        c.lwc1(0, (rider as u32).wrapping_add(WIPEOUT_RIDER_TIME_SCALE));
        c.g[A2][0] = sx((rider as u32).wrapping_add(0x1e0));
        c.g[A0][0] = Addr((rider as u32).wrapping_add(WIPEOUT_RIDER_AIR_PREDICTOR)).lw(c);
        c.mul_s(20, 0, 1); // dt
        c.g[A1][0] = sx((rider as u32).wrapping_add(0x110));
        c.mov_s(12, 20);
        call_ptr(c, 0x0011_3648, 0x0013_7788)?;

        // Ballistic flight (mode 1 or 3)?
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(WIPEOUT_STATE_RIDER)).lw(c);
        c.g[A0][0] = 1;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(WIPEOUT_RIDER_AIR_PREDICTOR)).lw(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(WIPEOUT_AP_MODE)).lw(c);
        c.g[A1][0] = 0;
        let mut ballistic = c.g[V1][0] == 1;
        if !ballistic {
            c.cycles += 3;
            c.g[V0][0] = 3;
            ballistic = c.g[V1][0] == 3;
        }
        if ballistic {
            c.cycles += 1;
            c.g[A1][0] = 1;
        }
        c.cycles += 2;
        if ballistic {
            c.cycles += 11;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(WIPEOUT_STATE_RIDER)).lw(c);
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x7274)); // one tick
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(WIPEOUT_RIDER_AIR_PREDICTOR)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0xa0));
            c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x98));
            c.sub_s(0, 0, 1); // time left
            c.c_le_s(2, 0);
            let long = c.fcc();
            c.lwc1(12, (c.g[V0][0] as u32).wrapping_add(WIPEOUT_RIDER_TIME_SCALE));
            if long {
                c.cycles += 3;
                c.div_s(12, 12, 0);
            } else {
                c.cycles += 3;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7270)); // 60
                c.mul_s(12, 12, 0);
            }
            c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(WIPEOUT_RIDER_TIME_SCALE));
            c.cycles += 7;
            c.g[A0][0] = c.g[A1][0];
            c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x726c));
            c.g[A1][0] = Addr((c.g[A1][0] as u32).wrapping_add(WIPEOUT_RIDER_AIR_PREDICTOR)).lw(c);
            c.mul_s(13, 0, 13);
            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
            call_ptr(c, 0x0012_1aa0, 0x0013_7818)?;
        }

        // spin *= 1 - 1.5 * dt (VU0).
        c.cycles += 18;
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(AT, 0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.mul_s(0, 20, 0);
        c.sub_s(1, 1, 0);
        c.mfc1(2, 1);
        c.lqc2(4, (c.g[S0][0] as u32).wrapping_add(WIPEOUT_STATE_SPIN));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[S0][0] as u32).wrapping_add(WIPEOUT_STATE_SPIN));
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(WIPEOUT_STATE_RIDER)).lw(c);
        call_ptr(c, 0x0011_e098, 0x0013_784c)?;

        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EAFile_QueueRead.

/// `EAFileRequest` fields.
const EAFILE_QR_STATUS: u32 = 0xc;
const EAFILE_QR_CALLBACK_ARG: u32 = 0x10;
const EAFILE_QR_CALLBACK: u32 = 0x14;
const EAFILE_QR_OFFSET: u32 = 0x18;
const EAFILE_QR_SIZE: u32 = 0x1c;
const EAFILE_QR_BUFFER: u32 = 0x20;
const EAFILE_QR_FILE: u32 = 0x24;
/// `EAFile::size`.
const EAFILE_QR_FILE_SIZE: u32 = 0x4;

/// `int EAFile_QueueRead(EAFile* f, int offset, void* buf, int size, int
/// cbArg, void* cb)` (0x3dd648): allocate a request (`EAFile_AllocRequest`),
/// make it a read (command 4 in bits 20-23 of word 0), store the callback;
/// a null file marks the request failed (status 6, `EAFile_QueueSignalStub`).
/// Clamp the size to the end of the file, fill the request and hand it to
/// `EAFile_ServiceQueue`; returns the request's word 0 (its handle).
pub fn ea_file_queue_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[T1][0]; // callback
        frame.save(c, S3, 0x40);
        c.g[S1][0] = c.g[T0][0]; // callback arg
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A3][0]; // size
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A0][0]; // file
        frame.save(c, S6, 0x10);
        c.g[S5][0] = c.g[A1][0]; // offset
        frame.save(c, S2, 0x50);
        frame.save_ra(c, 0);
        c.g[S6][0] = c.g[A2][0]; // buffer
        Callee { addr: 0x003d_e670, f: ee::s14::EAFile_AllocRequest }.call(c, 0x003d_d688)?;
        c.g[S2][0] = c.g[V0][0];
        let req = c.g[S2][0] as u32;
        c.g[V1][0] = 0xffff_ffff_ff0f_ffff;
        c.g[V0][0] = Addr(req).lw(c);
        c.g[A0][0] = 0x0040_0000;
        c.sw(req.wrapping_add(EAFILE_QR_CALLBACK), c.g[S0][0]);
        c.g[V0][0] &= c.g[V1][0];
        c.sw(req.wrapping_add(EAFILE_QR_CALLBACK_ARG), c.g[S1][0]);
        c.g[V0][0] |= c.g[A0][0]; // command: read
        c.sw(req, c.g[V0][0]);
        if c.g[S4][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = 6;
            c.g[A0][0] = c.g[S2][0];
            c.sw(req.wrapping_add(EAFILE_QR_STATUS), c.g[V0][0]);
            Callee { addr: 0x003d_e668, f: ee::s14::EAFile_QueueSignalStub }.call(c, 0x003d_d6c4)?;
        }

        c.cycles += 22;
        let req = c.g[S2][0] as u32;
        c.sw(req.wrapping_add(EAFILE_QR_FILE), c.g[S4][0]);
        c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S3][0] as u32)); // end
        c.g[A0][0] = c.g[S2][0];
        c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(EAFILE_QR_FILE_SIZE)).lw(c);
        c.sw(req.wrapping_add(EAFILE_QR_BUFFER), c.g[S6][0]);
        c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        c.sw(req.wrapping_add(EAFILE_QR_OFFSET), c.g[S5][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S5][0] as u32));
        if c.g[V1][0] != 0 {
            c.g[S3][0] = c.g[V0][0]; // clamp to the file end
        }
        c.sw(req.wrapping_add(EAFILE_QR_SIZE), c.g[S3][0]);
        Callee { addr: 0x003d_dfa8, f: ee::s14::EAFile_ServiceQueue }.call(c, 0x003d_d6f0)?;
        c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_CalcSurfaceLoopGroup1.

/// The group-1 pitch and volume curves (`.rodata`, five points each, copied
/// to the stack).
const SURFACE_LOOP1_PITCH_CURVE: u32 = 0x0048_2bc8;
const SURFACE_LOOP1_VOLUME_CURVE: u32 = 0x0048_2bf0;
/// The smoothed loop level per surface (`SSXAudioSystem + 0x59b8`).
const SURFACE_LOOP1_LEVELS: u32 = 0x59b8;

/// Copy one unaligned doubleword (`ldl/ldr` into GPR `reg`, `sdl/sdr`).
fn surface_loop1_copy_dword(c: &mut Cpu, reg: usize, src: u32, dst: u32) {
    c.ldl(reg, src.wrapping_add(7));
    c.ldr(reg, src);
    c.sdl(reg, dst.wrapping_add(7));
    c.sdr(reg, dst);
}

/// `void SSXAudioSystem_CalcSurfaceLoopGroup1(SSXAudioSystem* a, Rider* r,
/// int* pitch, int* volume)` (0x293740): the board-surface loop of group 1.
/// The level is the rider's speed factor (`+0x758`) times
/// `1 - (+0x750 + +0x754)`, boosted (`* +0x75c * 10`) when `+0x75c` is
/// under the threshold `gp-0x45c0`. The surface index comes from the
/// rider's virtual (`+0x6c0`, slot 0x38); a jump of more than the
/// threshold from that surface's previous level moves a quarter of the way
/// instead. The stored level is evaluated on the pitch and volume curves
/// (`Audio_EvalCurve5`): `pitch = int(curve)`, `volume = int(curve * 4096)`.
pub fn ssx_audio_system_calc_surface_loop_group1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xb0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0); // 1.0
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = c.g[A0][0]; // audio system
        frame.save(c, S2, 0x70);
        c.g[S1][0] = c.g[A2][0]; // pitch out
        frame.save_fpr(c, 21, 0xa8);
        c.g[S2][0] = c.g[A3][0]; // volume out
        frame.save_ra(c, 0x60);
        frame.save_fpr(c, 20, 0xa0);
        let rider = c.g[A1][0] as u32;
        c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x45c0)); // threshold
        c.lwc1(1, rider.wrapping_add(0x754));
        c.lwc1(2, rider.wrapping_add(0x750));
        c.lwc1(12, rider.wrapping_add(0x75c));
        c.add_s(1, 1, 2);
        c.lwc1(20, rider.wrapping_add(0x758));
        c.c_lt_s(12, 21);
        c.sub_s(0, 0, 1);
        let boost = c.fcc();
        c.mul_s(20, 20, 0); // level
        if boost {
            c.cycles += 4;
            c.mul_s(1, 20, 12);
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(AT, 0); // 10.0
            c.mul_s(20, 1, 0);
        }

        // Surface index from the rider's virtual.
        c.cycles += 18;
        c.g[V1][0] = Addr(rider.wrapping_add(0x6c0)).lw(c);
        c.g[A1][0] = sx(rider.wrapping_add(0x6c0));
        let vt = c.g[V1][0];
        let slot = VSlot::read(c, vt, 0x38);
        c.g[A0][0] = slot.delta;
        c.g[V0][0] = slot.func;
        c.g[A0][0] = slot.this(c.g[A1][0]);
        call_ptr(c, slot.func as u32, 0x0029_37c0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(SURFACE_LOOP1_LEVELS));
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.lwc1(2, c.g[V0][0] as u32); // previous level
        c.sub_s(1, 2, 20);
        c.abs_s(0, 1);
        c.c_lt_s(21, 0);
        let jump = c.fcc();
        c.g[V1][0] = 0x0048_0000;
        if jump {
            c.cycles += 3;
            c.c_lt_s(20, 2);
            if c.fcc() {
                // Falling: move down a quarter of the way.
                c.cycles += 5;
                c.g[AT][0] = 0x3e80_0000;
                c.mtc1(AT, 0);
                c.mul_s(0, 1, 0);
                c.sub_s(20, 2, 0);
            } else {
                c.cycles += 4;
                c.c_lt_s(2, 20);
                c.g[V1][0] = 0x0048_0000;
                if c.fcc() {
                    // Rising: move up a quarter of the way.
                    c.cycles += 5;
                    c.sub_s(0, 20, 2);
                    c.g[AT][0] = 0x3e80_0000;
                    c.mtc1(AT, 1);
                    c.mul_s(0, 0, 1);
                    c.add_s(20, 2, 0);
                }
            }
        }

        // Store the level, copy both curves to the stack, evaluate them.
        c.cycles += 68;
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2bc8));
        let pitch_src = c.g[T0][0] as u32;
        let base = sp(c);
        for (i, reg) in [4usize, 6, 7, 4, 6].into_iter().enumerate() {
            let o = 8 * i as u32;
            surface_loop1_copy_dword(c, reg, pitch_src + o, base + o);
        }
        c.swc1(20, c.g[V0][0] as u32);
        c.g[A1][0] = 0x0048_0000;
        c.g[S0][0] = sx(base.wrapping_add(0x30));
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = sx(SURFACE_LOOP1_VOLUME_CURVE);
        for (i, reg) in [2usize, 3, 6, 2, 3].into_iter().enumerate() {
            let o = 8 * i as u32;
            surface_loop1_copy_dword(c, reg, SURFACE_LOOP1_VOLUME_CURVE + o, base + 0x30 + o);
        }
        c.mov_s(12, 20);
        Callee { addr: 0x0029_0b58, f: ee::s07::Audio_EvalCurve5 }.call(c, 0x0029_38fc)?;
        c.cvt_w_s(1, 0);
        c.swc1(1, c.g[S1][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(12, 20);
        Callee { addr: 0x0029_0b58, f: ee::s07::Audio_EvalCurve5 }.call(c, 0x0029_3910)?;
        c.g[AT][0] = 0x4580_0000;
        c.mtc1(AT, 1); // 4096.0
        c.mul_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        c.swc1(1, c.g[S2][0] as u32);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 21, 0xa8);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimSequence_AddTrigger.

/// `AnimSequence::triggers` (singly linked list head).
const ANIM_SEQUENCE_TRIGGERS: u32 = 0xac;
/// The allocation name `D_0048A5B0`.
const ANIM_TRIGGER_ALLOC_NAME: u32 = 0x0048_a5b0;

/// `void AnimSequence_AddTrigger(AnimSequence* self, int key, float time)`
/// (0x313a70): allocate a 0x10-byte trigger node (`EA_MemAllocNamed(0x10,
/// "...", 0x20000000, 0)`) and push it on the sequence's trigger list:
/// `{ key, fired = 0, time, next = old head }`.
pub fn anim_sequence_add_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S1, 0x10);
        c.g[A3][0] = 0;
        c.g[S1][0] = c.g[A1][0]; // key
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0]; // sequence
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(20, 12); // time
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(ANIM_TRIGGER_ALLOC_NAME);
        c.g[A0][0] = 0x10;
        Callee { addr: 0x0031_7d70, f: ee::s10::EA_MemAllocNamed }.call(c, 0x0031_3aa8)?;
        let node = c.g[V0][0] as u32;
        let list = (c.g[S0][0] as u32).wrapping_add(ANIM_SEQUENCE_TRIGGERS);
        c.g[V1][0] = Addr(list).lw(c); // old head
        c.sw(list, c.g[V0][0]);
        c.sw(node, c.g[S1][0]);
        c.swc1(20, node.wrapping_add(0x8));
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        c.sw(node.wrapping_add(0xc), c.g[V1][0]);
        c.sw(node.wrapping_add(0x4), 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// Emitter_BuildDescriptor with real callees: random emitter (`a0`,
    /// rate at `+0x4c`), matrix (`a1`), parameter block (`a2`) and bounds
    /// (`a3`); the time `f12` is negative on some seeds (random seeds through
    /// `Rand_NextMain`, whose state at `0x4ff018` is random too) and `p+8`
    /// is negative on some seeds (the time branch).
    #[test]
    fn emitter_build_descriptor_matches() {
        const SELF: u32 = DATA;
        const PARAMS: u32 = DATA + 0x200;
        const MATRIX: u32 = DATA + 0x400;
        const BOUNDS: u32 = DATA + 0x500;
        const RAND_STATE: u32 = 0x0050_0000 - 0xfe8;
        check(ee::s12::f_00370058__generated, emitter_build_descriptor, 200, &|c, seed| {
            let mut r = Rng(0x0037_0058 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0037_0058;
            c.g[GP][0] = sx(GAME_GP);
            c.g[A0][0] = sx(SELF);
            c.g[A1][0] = sx(MATRIX);
            c.g[A2][0] = sx(PARAMS);
            c.g[A3][0] = sx(BOUNDS);
            for i in 0..0x50u32 {
                let v = if i % 7 == 0 { r.edge_f32() } else { r.f32(50.0) };
                c.bus.write32(SELF + 4 * i, v);
            }
            for i in 0..0x34u32 {
                c.bus.write32(PARAMS + 4 * i, r.f32(50.0));
            }
            for i in 0..0x10u32 {
                c.bus.write32(MATRIX + 4 * i, r.f32(4.0));
            }
            for i in 0..8u32 {
                c.bus.write32(BOUNDS + 4 * i, r.f32(100.0));
            }
            for i in 0..6u32 {
                c.bus.write32(RAND_STATE + 4 * i, r.next_u32());
            }
            let base = (GAME_GP as i32 - 0x2868) as u32;
            for i in 0..24u32 {
                c.bus.write32(base + 4 * i, r.f32(10.0));
            }
            // Lifetime / rate non-zero on most seeds.
            if seed % 5 != 0 {
                c.bus.write32(PARAMS + 0xc, 1.5f32.to_bits());
                c.bus.write32(SELF + 0x4c, 0.25f32.to_bits());
            }
            let time = r.f32(10.0) & 0x7fff_ffff;
            c.f[12] = if seed % 2 == 0 { time | 0x8000_0000 } else { time };
            let sign = c.bus.read32(PARAMS + 8) & 0x7fff_ffff;
            let sign = if seed % 3 == 0 { sign | 0x8000_0000 } else { sign };
            c.bus.write32(PARAMS + 8, sign);
        });
    }

    /// StreamArray_IsPlaying: an empty slot (false) or a buffer whose real
    /// StreamBuffer_IsPlaying -> GetRequestStatus chain runs on one SND
    /// stream slot (unknown or known stream, request state random), with
    /// fake lock hooks that unwind on some seeds.
    #[test]
    fn stream_array_is_playing_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        const ARRAY: u32 = DATA + 0x20_0000;
        const LIST: u32 = ARRAY + 0x40;
        const BUFFER: u32 = ARRAY + 0x400;
        const STREAM: u32 = ARRAY + 0x1000;
        const REQ: u32 = ARRAY + 0x2000;
        check(ee::s08::f_002ab028__generated, stream_array_is_playing, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x2a_b028, 4);
            c.pc = 0x002a_b028;
            let i = seed % 8;
            c.g[A0][0] = sx(ARRAY);
            c.g[A1][0] = i as u64;
            c.bus.write32(ARRAY + STREAM_ARRAY_BUFFERS, LIST);
            c.bus.write32(LIST + 4 * i, if seed % 4 == 0 { 0 } else { BUFFER });
            // SND lock / unlock hooks.
            c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
            c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
            // One stream slot.
            c.bus.write8(0x0051_0000 - 0x56d2, 1);
            c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
            let id = if seed % 3 == 0 { 1 + r.next_u32() % 4 } else { 0 };
            c.bus.write32(BUFFER + 0x8, id);
            c.bus.write32(STREAM + 0x124, if seed % 5 == 1 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(STREAM + 0x11c, REQ);
            c.bus.write32(REQ + 0xc, if seed % 6 < 3 { 0x8000_0000 | r.next_u32() } else { r.next_u32() % 4 });
            c.bus.write16(STREAM + 0x14, 0);
            c.bus.write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
        });
    }

    /// PATH_PlatGetTrackState: an empty slot (-8), a clean track, or a dirty
    /// one (the real PATH_PlatRefreshStatus runs first); fake hooks unwind on
    /// some seeds.
    #[test]
    fn path_plat_get_track_state_matches() {
        use crate::difftest::fakes::machine;
        check(ee::s08::f_002aebd0__generated, path_plat_get_track_state, 120, &|c, seed| {
            let _ = machine(c, seed, 0x2a_ebd0, 2);
            c.pc = 0x002a_ebd0;
            for i in 0..4u32 {
                let e = PATH_PLAT_TRACKS + 4 * i;
                if (seed / 4 + i) % 3 == 0 {
                    c.bus.write32(e, 0);
                } else {
                    let track = DATA + 0x400 + 0x80 * i;
                    c.bus.write32(e, track);
                    c.bus.write8(track + PATH_PLAT_TRACK_DIRTY, ((seed + i) % 3 == 0) as u8);
                    c.bus.write32(track + 0x48, 0x10 + i);
                    c.bus.write32(track + 0x4c, 0x20 + i);
                    c.bus.write32(track + PATH_PLAT_TRACK_STATE, 0x50 + i);
                    c.bus.write32(track + 0x5c, if seed % 2 == 0 { 0 } else { 0x30 + i });
                    c.bus.write32(track + 0x60, if seed % 3 == 0 { 0x40 + i } else { 0 });
                }
            }
            c.g[A0][0] = (seed % 4) as u64;
        });
    }

    /// RiderBase_ExitSubState: every sub state 0..13 plus out-of-range ones,
    /// through the game's jump table (ELF image), with the real exit
    /// handlers on short paths: every sub-object points at one actor whose
    /// animation controller has layer 1 idle (`0x1b6`: the cruise and
    /// natural-air exits stop there, the spin exit's fades find empty
    /// lists), and the world's frame counter is odd (the wipeout exit
    /// returns at once). Some seeds zero the table (fake jump out, which
    /// unwinds on some of them).
    #[test]
    fn rider_base_exit_sub_state_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::machine;
        const RIDER: u32 = DATA;
        const ACTOR: u32 = DATA + 0x1_0000;
        const CTRL: u32 = DATA + 0x2_0000;
        const WORLD: u32 = DATA + 0x3_0000;
        check(ee::s00::f_00111578__generated, rider_base_exit_sub_state, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x11_1578, 1);
            elf_image(c);
            c.pc = 0x0011_1578;
            let states = [0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0xffff_ffff];
            c.bus.write32(RIDER + RIDER_SUB_STATE, states[(seed as usize) % states.len()]);
            if seed % 17 == 3 {
                for i in 0..14 {
                    c.bus.write32(RIDER_EXIT_SUB_STATE_TABLE + 4 * i, 0);
                }
            }
            // Each handler's link to the rider actor.
            c.bus.write32(RIDER + 0x1c0, ACTOR); // cruise: +0
            c.bus.write32(RIDER + 0x1d0 + 0x14, ACTOR); // board press
            c.bus.write32(RIDER + 0x210 + 0x14, ACTOR); // natural air
            c.bus.write32(RIDER + 0x230 + 0x58, ACTOR); // spin
            c.bus.write32(RIDER + 0x2b0 + 0x8, ACTOR); // 7
            c.bus.write32(RIDER + 0x2c0 + 0x80, ACTOR); // wipeout
            c.bus.write32(RIDER + 0x370 + 0x14, ACTOR); // 11
            c.bus.write32(ACTOR + 0x784, CTRL);
            c.bus.write32(CTRL + 4, 0x1b6);
            // World -> game -> race: an odd frame counter.
            c.bus.write32(GAME_GP - 0x848, WORLD);
            c.bus.write32(WORLD + 0x84, WORLD + 0x100);
            c.bus.write32(WORLD + 0x100 + 0xc, WORLD + 0x200);
            c.bus.write32(WORLD + 0x200 + 8, 2 * (r.next_u32() & 0xffff) + 1);
            c.g[A0][0] = sx(RIDER);
            c.g[A1][0] = (r.next_u32() % 6) as u64;
        });
    }

    /// PathData_FindNearAIPaths: 0..12 AI paths whose boxes contain the
    /// point (taken in the first pass, possibly filling `max`) or lie at
    /// random distances (equal ones too), some disabled; `max` from -1 to 8,
    /// `enabledOnly` on half the seeds; the real Path_BoundsDistance.
    #[test]
    fn path_data_find_near_ai_paths_matches() {
        const PATHDATA: u32 = DATA;
        const POINT: u32 = DATA + 0x80;
        const PATHS: u32 = DATA + 0x100;
        const OUT: u32 = DATA + 0x1000;
        check(ee::s07::f_0026afb8__generated, path_data_find_near_ai_paths, 300, &|c, seed| {
            let mut r = Rng(0x0026_afb8 ^ (seed as u64) << 4);
            scramble(c, &mut r);
            c.pc = 0x0026_afb8;
            c.g[GP][0] = sx(GAME_GP);
            c.bus.write32(GAME_GP - 0x4a4c, 1.0e30f32.to_bits());
            c.bus.write32(GAME_GP - 0x4a48, 2.0e30f32.to_bits());
            let n = seed % 13;
            c.bus.write32(PATHDATA + PATH_DATA_AI_COUNT, n);
            c.bus.write32(PATHDATA + PATH_DATA_AI_PATHS, PATHS);
            for k in 0..3 {
                c.bus.write32(POINT + 4 * k, r.f32(4.0));
            }
            let point = [0, 1, 2].map(|k| f32::from_bits(c.bus.read32(POINT + 4 * k)));
            for i in 0..n {
                let path = PATHS + 0x40 * i;
                let mode = r.next_u32() % 4;
                for k in 0..3u32 {
                    let p = point[k as usize];
                    let (lo, hi) = match mode {
                        0 => (p - 1.0, p + 1.0), // contains the point
                        1 => (p + 1.0 + (i % 3) as f32, p + 5.0), // same distance per i%3
                        _ => (p + 0.5 + r.next_u32() as f32 / 1.0e9, p + 9.0),
                    };
                    c.bus.write32(path + 0x1c + 4 * k, lo.to_bits());
                    c.bus.write32(path + 0x28 + 4 * k, hi.to_bits());
                }
                c.bus.write32(path + AI_PATH_ENABLED, (r.next_u32() % 3 != 0) as u32);
            }
            c.g[A0][0] = sx(PATHDATA);
            c.g[A1][0] = sx(POINT);
            c.g[A2][0] = sx(OUT);
            c.g[A3][0] = sx(((seed / 13 % 10) as i32 - 1) as u32);
            c.g[T0][0] = (seed % 2) as u64;
        });
    }

    /// ParticleModifier_BaseSave: a stream whose vtable slot 0x28 is a fake
    /// (random `this` delta) that unwinds on some seeds.
    #[test]
    fn particle_modifier_base_save_matches() {
        use crate::difftest::fakes::{machine, vtable};
        check(ee::s11::f_00345890__generated, particle_modifier_base_save, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x34_5890, 1);
            c.pc = 0x0034_5890;
            let modifier = DATA + 0x10;
            let stream = DATA + 0x200;
            c.bus.write32(modifier + PARTICLE_MODIFIER_TYPE, r.next_u32());
            let vt = vtable(c, &mut r, &[STREAM_VT_WRITE_WORD]);
            c.bus.write32(stream, vt);
            c.g[A0][0] = sx(modifier);
            c.g[A1][0] = sx(stream);
        });
    }

    /// RiderWipeoutState_UpdateAir with the real AirPredictor_Update,
    /// Rider_UpdateOrientation and orientation update (game literals from
    /// the ELF image): the predictor in a flight with mode 1, 3 or another
    /// (no re-aim), the time left above or below one tick.
    #[test]
    fn rider_wipeout_state_update_air_matches() {
        use crate::difftest::{elf_image, put_vec};
        const STATE: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const AP: u32 = DATA + 0x3000;
        check(ee::s01::f_00137750__generated, rider_wipeout_state_update_air, 120, &|c, seed| {
            let mut r = Rng(0x0013_7750 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            elf_image(c);
            c.g[GP][0] = sx(GAME_GP);
            c.pc = 0x0013_7750;
            let small = |r: &mut Rng, k: f32| f32::from_bits(r.f32(k));
            // The wipeout state: spin vector, rider.
            put_vec(c, STATE + WIPEOUT_STATE_SPIN, [r.f32(3.0), r.f32(3.0), r.f32(3.0), r.f32(1.0)]);
            c.bus.write32(STATE + WIPEOUT_STATE_RIDER, RIDER);
            // The rider: time scale ~1, unit orientation, position/velocity.
            c.bus.write32(RIDER + WIPEOUT_RIDER_TIME_SCALE, (1.0 + small(&mut r, 0.5)).to_bits());
            c.bus.write32(RIDER + WIPEOUT_RIDER_AIR_PREDICTOR, AP);
            put_vec(c, RIDER + 0x110, [r.f32(100.0), r.f32(100.0), r.f32(100.0), 0x3f80_0000]);
            put_vec(c, RIDER + 0x1e0, [r.f32(10.0), r.f32(10.0), r.f32(10.0), 0]);
            for o in [0x100u32, 0x120, 0x130, 0x140] {
                put_vec(c, RIDER + o, [0, 0, 0, 0x3f80_0000]);
            }
            // The predictor: a flight in progress (as AirPredictor_Update's test).
            for o in (0..0xb0).step_by(4) {
                c.bus.write32(AP + o, 0);
            }
            for i in 0..3 {
                c.bus.write32(AP + 0x70 + 4 * i, small(&mut r, 1000.0).to_bits());
                c.bus.write32(AP + 0x80 + 4 * i, small(&mut r, 500.0).to_bits());
                c.bus.write32(AP + 0x10 + 4 * i, small(&mut r, 1.0).to_bits());
                c.bus.write32(AP + 0x20 + 4 * i, small(&mut r, 1.0).to_bits());
            }
            c.bus.write32(AP + 0x7c, 1.0f32.to_bits());
            c.bus.write32(AP + 0xa8, 2000.0f32.to_bits());
            c.bus.write32(AP + WIPEOUT_AP_MODE, [1u32, 3, 0, 2][(seed % 4) as usize]);
            let now = (seed % 7) as f32 / 60.0;
            let end = now + if seed % 3 == 0 { 0.005 } else { 0.5 + (seed % 5) as f32 };
            c.bus.write32(AP + 0xa0, now.to_bits());
            c.bus.write32(AP + 0xa4, now.to_bits());
            c.bus.write32(AP + 0x98, end.to_bits());
            c.g[A0][0] = sx(STATE);
        });
    }

    /// EAFile_QueueRead: the real EAFile_AllocRequest on a pool with a free
    /// slot (EAFile_AllocRequest's own fixture: mutex with fake syscalls,
    /// some unwinding), a null file or one whose size clamps the read or
    /// not, and the real EAFile_ServiceQueue on its short path (a request
    /// already active at pool+0x24: insert into the priority queue of 0..2
    /// requests, unlock, return).
    #[test]
    fn ea_file_queue_read_matches() {
        use crate::difftest::fakes::machine;
        const POOL: u32 = 0x0051_9ab0;
        const MUTEX: u32 = POOL + 0x14;
        const SLOTS: u32 = DATA + 0x1000;
        const FILE: u32 = DATA + 0x2000;
        const QUEUED: u32 = DATA + 0x3000;
        check(ee::s14::f_003dd648__generated, ea_file_queue_read, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x3d_d648, 8);
            c.pc = 0x003d_d648;
            let count = 1 + r.next_u32() % 6;
            c.bus.write32(POOL, count);
            c.bus.write32(POOL + 0x28, SLOTS);
            c.bus.write32(MUTEX + 0x4, 0x7700_0000 | r.next_u32() & 0xffff);
            c.bus.write32(MUTEX + 0x8, r.next_u32() & 3);
            c.bus.write32(MUTEX + 0xc, r.next_u32() & 0xff);
            c.bus.write32(0x0045_0de4, r.next_u32() & 1);
            let free = r.next_u32() % count;
            for i in 0..8u32 {
                let mut w = r.next_u32();
                if i == free {
                    w &= !0x00f0_0000;
                } else if i < free && w & 0x00f0_0000 == 0 {
                    w |= 0x0010_0000;
                }
                c.bus.write32(SLOTS + 0x30 * i, w);
            }
            c.bus.write32(0x0045_0c0c, r.next_u32() & 0xf_ffff);
            // The service queue: a request active; 0..2 queued.
            c.bus.write32(POOL + 0x24, QUEUED + 0x100);
            let queued = seed % 3;
            c.bus.write32(POOL + 0x30, if queued == 0 { 0 } else { QUEUED });
            for k in 0..queued {
                let q = QUEUED + 0x40 * k;
                c.bus.write32(q + 0x10, r.next_u32() % 8);
                c.bus.write32(q + 0x2c, if k + 1 < queued { q + 0x40 } else { 0 });
            }
            c.bus.write32(POOL + 0xc, r.next_u32() & 0xff);
            c.bus.write32(FILE + EAFILE_QR_FILE_SIZE, r.next_u32() & 0xf_ffff);
            c.g[A0][0] = if seed % 5 == 0 { 0 } else { sx(FILE) };
            c.g[A1][0] = sx(r.next_u32() & 0x7_ffff);
            c.g[A2][0] = sx(DATA + 0x8000);
            c.g[A3][0] = sx(r.next_u32() & 0x7_ffff);
            c.g[T0][0] = sx(r.next_u32() % 8);
            c.g[T1][0] = sx(r.next_u32());
        });
    }

    /// SSXAudioSystem_CalcSurfaceLoopGroup1: rider factors below / above the
    /// threshold, a surface getter that is a fake virtual (forced index
    /// 0..7 via the fake's result; unwinds on some seeds), previous levels
    /// far below / above / near the new level; the real Audio_EvalCurve5 on
    /// sorted five-point curves.
    #[test]
    fn ssx_audio_system_calc_surface_loop_group1_matches() {
        use crate::difftest::fakes::{machine, vtable, LOG_RESULTS};
        const AUDIO: u32 = DATA;
        const RIDER: u32 = DATA + 0x8000;
        const OUT: u32 = DATA + 0xa000;
        check(ee::s07::f_00293740__generated, ssx_audio_system_calc_surface_loop_group1, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x29_3740, 1);
            c.pc = 0x0029_3740;
            c.bus.write32(GAME_GP - 0x45c0, 0.3f32.to_bits());
            // Curves: x ascending 0..1, then y values.
            for (base, k) in [(SURFACE_LOOP1_PITCH_CURVE, 1.0f32), (SURFACE_LOOP1_VOLUME_CURVE, 0.5)] {
                for i in 0..5u32 {
                    c.bus.write32(base + 4 * i, (i as f32 * 0.25).to_bits());
                    c.bus.write32(base + 0x14 + 4 * i, (k * (i * i) as f32 + 0.1).to_bits());
                }
            }
            let f = |r: &mut Rng, lo: f32, hi: f32| lo + (r.next_u32() % 1000) as f32 / 1000.0 * (hi - lo);
            c.bus.write32(RIDER + 0x750, f(&mut r, 0.0, 0.4).to_bits());
            c.bus.write32(RIDER + 0x754, f(&mut r, 0.0, 0.4).to_bits());
            c.bus.write32(RIDER + 0x758, f(&mut r, 0.0, 1.0).to_bits());
            c.bus.write32(RIDER + 0x75c, f(&mut r, 0.0, 0.6).to_bits());
            let vt = vtable(c, &mut r, &[0x38]);
            c.bus.write32(RIDER + 0x6c0, vt);
            let index = r.next_u32() % 8;
            // The fake's forced result (zero would mean "a pointer", so the
            // index is 1..8).
            c.bus.write32(LOG_RESULTS, index + 1);
            for i in 0..10u32 {
                let prev = match (seed + i) % 3 {
                    0 => f(&mut r, -2.0, -1.0),
                    1 => f(&mut r, 1.5, 3.0),
                    _ => f(&mut r, 0.0, 1.0),
                };
                c.bus.write32(AUDIO + SURFACE_LOOP1_LEVELS + 4 * i, prev.to_bits());
            }
            c.g[A0][0] = sx(AUDIO);
            c.g[A1][0] = sx(RIDER);
            c.g[A2][0] = sx(OUT);
            c.g[A3][0] = sx(OUT + 4);
        });
    }

    /// AnimSequence_AddTrigger: the real EA_MemAllocNamed on the engine's
    /// pool allocator; an empty or non-empty trigger list.
    #[test]
    fn anim_sequence_add_trigger_matches() {
        use crate::difftest::fakes::machine;
        use crate::game::ui::uianimation::tests::pool_allocator;
        check(ee::s10::f_00313a70__generated, anim_sequence_add_trigger, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x31_3a70, 0);
            c.pc = 0x0031_3a70;
            pool_allocator(c);
            let seq = DATA + 0x6_0000;
            let head = if seed % 2 == 0 { 0 } else { DATA + 0x6_1000 };
            c.bus.write32(seq + ANIM_SEQUENCE_TRIGGERS, head);
            c.g[A0][0] = sx(seq);
            c.g[A1][0] = sx(r.next_u32());
            c.f[12] = r.f32(5.0);
        });
    }
}
