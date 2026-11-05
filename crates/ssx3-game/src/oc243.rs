//! Staging wave 205 (oc243): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0031_74a8,
        ps2_exec_man_invoke_callback,
        "PS2ExecMan_InvokeCallback",
    ),
    (
        0x0010_48c0,
        rider_anim_seq_type00_update,
        "RiderAnim_UpdateSeqType00",
    ),
    (
        0x0013_5cb0,
        rider_spin_sub_get_flip_progress,
        "RiderSpinSub_GetFlipProgress",
    ),
    (
        0x0015_ec98,
        game_camera_update_stream_view,
        "GameCamera_UpdateStreamView",
    ),
    (
        0x0035_8260,
        multi_particle_emit_at_instances,
        "MultiParticle_EmitAtInstances",
    ),
    (0x0040_da10, logf, "logf"),
    (0x002e_3478, lens_fx_draw_flares, "LensFX_DrawFlares"),
    (
        0x0028_0640,
        script_audio_obj_update_sounds,
        "ScriptAudioObj_UpdateSounds",
    ),
    (
        0x0035_c040,
        rail_modifier_update_bounds,
        "RailModifier_UpdateBounds",
    ),
    (
        0x0010_0f88,
        computer_actor_rider_cruise_chase_rider,
        "ComputerActorRider_CruiseChaseRider",
    ),
];

// ------------------------------------------------ PS2ExecMan_InvokeCallback

/// `void PS2ExecMan_InvokeCallback(Callback* cb, int notify)` (0x3174a8,
/// `cExecutionMan::checkHalt`): call `cb->func(cb->arg)` if a function is
/// set; then, when `notify` is nonzero, tell the owner at `cb+8` through
/// its virtual method at +0x70 (`this` adjusted by the slot's delta, so
/// relative to `cb`).
pub fn ps2_exec_man_invoke_callback(c: &mut Cpu) {
    /// `Callback` fields.
    const FUNC: u32 = 0x0;
    const ARG: u32 = 0x4;
    const OWNER: u32 = 0x8;
    /// The owner's vtable slot `{delta: i16, method}` (embedded at +0x70).
    const SLOT_DELTA: u32 = 0x70;
    const SLOT_METHOD: u32 = 0x74;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let cb = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let func = Addr(cb).off(FUNC).lw(c);
        c.g[V0][0] = func;
        c.g[S1][0] = c.g[A1][0];
        if func == 0 {
            c.cycles += 1;
        } else {
            c.cycles += 4;
            c.g[A0][0] = Addr(cb).off(ARG).lw(c);
            call_ptr(c, func as u32, 0x0031_74d0)?;
            // `notify` is compared as the full 64-bit register.
            if c.g[S1][0] != 0 {
                c.cycles += 5;
                let owner = Addr(cb).off(OWNER).lw(c);
                c.g[V0][0] = owner;
                c.g[A0][0] = Addr(owner as u32).off(SLOT_DELTA).lh(c);
                c.g[V1][0] = Addr(owner as u32).off(SLOT_METHOD).lw(c);
                let method = r32(c, V1);
                c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
                call_ptr(c, method, 0x0031_74ec)?;
                c.cycles += 1;
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ------------------------------------------------ RiderAnim_UpdateSeqType00

/// `int RiderAnim_UpdateSeqType00(RiderAnim* anim, SeqList* list,
/// AnimSequence* seq, float dt)` (0x1048c0): a one-shot sequence. Advance
/// its track by `dt * (1/60)`; when the track has ended, or else when its
/// weight fade has finished, remove it from the list. Returns whether it
/// was removed. (Same shape as type 02, with its own `.lit4` constant.)
pub fn rider_anim_seq_type00_update(c: &mut Cpu) {
    /// `AnimSequence_AdvanceTrack(seq, 0, step)`: nonzero when it ended.
    const ADVANCE_TRACK: u32 = 0x0031_35b0;
    /// `AnimSequence_UpdateWeightFade(seq, step)`: nonzero when faded out.
    const UPDATE_WEIGHT_FADE: u32 = 0x0031_3800;
    /// `AnimSeqList_Remove(list, seq)`.
    const LIST_REMOVE: u32 = 0x0031_45f8;
    /// `gp-0x7f44`: 1/60, ticks to seconds (`.lit4`, follows the rate).
    const TICK_GP: u32 = 0x7f44;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        let k = Addr(r32(c, GP).wrapping_sub(TICK_GP)).read_fpu(c);
        c.f[0] = k.0;
        frame.save_fpr(c, 20, 0x30);
        let step = FpuF32(c.f[12]) * k;
        c.f[20] = step.0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        c.f[12] = step.0;
        call_ptr(c, ADVANCE_TRACK, 0x0010_48f4)?;
        let ended = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !ended {
            c.cycles += 3;
            c.f[12] = c.f[20];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, UPDATE_WEIGHT_FADE, 0x0010_4908)?;
        }
        c.cycles += 2;
        let remove = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S1][0];
        if remove {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, LIST_REMOVE, 0x0010_4918)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    });
}

// --------------------------------------------- RiderSpinSub_GetFlipProgress

/// `BELibrary_GetInstance()`.
const BE_LIBRARY_GET_INSTANCE: Callee = Callee {
    addr: 0x0014_dc80,
    f: ee::s01::BELibrary_GetInstance,
};
/// `BELibrary_GetInterface(lib, id)`.
const BE_LIBRARY_GET_INTERFACE: Callee = Callee {
    addr: 0x0014_dd58,
    f: ee::s01::BELibrary_GetInterface,
};
/// `float BECharInt_GetAttrib4Factor(charInt, rider, value)`.
const BE_CHAR_INT_GET_ATTRIB4_FACTOR: Callee = Callee {
    addr: 0x0014_95a8,
    f: ee::s01::BECharInt_GetAttrib4Factor,
};

/// `void RiderSpinSub_GetFlipProgress(RiderSpinSub* self, float* done,
/// float* total)` (0x135cb0): how far the current flip has turned. `done`
/// is the flip angle covered (`|end - start|`, folded below pi) over the
/// flip rate, `total` pi over the rate; both halved (times ~0.5) when the
/// rider's flip boost (+0x2ec) is positive, then scaled by
/// `1 / (1 + attrib4 * ~0.5)` from the rider's character attribute 4.
pub fn rider_spin_sub_get_flip_progress(c: &mut Cpu) {
    /// `RiderSpinSub` fields.
    const FLIP_START: u32 = 0x14;
    const FLIP_END: u32 = 0x1c;
    const FLIP_RATE: u32 = 0x38;
    const RIDER: u32 = 0x58;
    /// Rider fields.
    const RIDER_FLIP_BOOST: u32 = 0x2ec;
    const RIDER_INDEX: u32 = 0x86c;
    const RIDER_ATTRIB4: u32 = 0xb34;
    /// `.lit4` constants: pi, the boost scale, the attribute scale.
    const PI_GP: u32 = 0x7330;
    const BOOST_GP: u32 = 0x732c;
    const ATTRIB_GP: u32 = 0x7328;
    /// `BELibrary` interface id of the character interface.
    const CHAR_INTERFACE: u64 = 3;
    let lit = |c: &Cpu, off: u32| r32(c, GP).wrapping_sub(off);
    let fpr = |c: &Cpu, n: usize| FpuF32(c.f[n]);
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        let pi = Addr(lit(c, PI_GP)).read_fpu(c);
        c.f[3] = pi.0;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        let done_out = r32(c, S1);
        let total_out = r32(c, S2);
        let start = Addr(this).off(FLIP_START).read_fpu(c);
        let end = Addr(this).off(FLIP_END).read_fpu(c);
        let diff = end - start;
        c.f[0] = diff.0;
        let mut angle = diff.abs();
        let wrapped = pi.lt(angle);
        abi::set_fcc(c, wrapped);
        if wrapped {
            angle = angle - pi;
        }
        c.f[1] = angle.0;

        c.cycles += 12;
        let rate = Addr(this).off(FLIP_RATE).read_fpu(c);
        c.f[2] = 0;
        let done = angle.div(rate);
        c.f[0] = done.0;
        Addr(done_out).write_fpu(c, done);
        let rate = Addr(this).off(FLIP_RATE).read_fpu(c);
        let total = pi.div(rate);
        c.f[1] = total.0;
        Addr(total_out).write_fpu(c, total);
        let rider = Addr(this).off(RIDER).lw(c);
        c.g[V0][0] = rider;
        let boost = Addr(rider as u32).off(RIDER_FLIP_BOOST).read_fpu(c);
        c.f[0] = boost.0;
        let boosted = FpuF32(0).lt(boost);
        abi::set_fcc(c, boosted);
        if boosted {
            c.cycles += 6;
            let half = Addr(lit(c, BOOST_GP)).read_fpu(c);
            c.f[2] = half.0;
            let total = fpr(c, 1) * half;
            c.f[1] = total.0;
            Addr(total_out).write_fpu(c, total);
            let done = Addr(done_out).read_fpu(c) * half;
            c.f[0] = done.0;
            Addr(done_out).write_fpu(c, done);
        }

        c.cycles += 28;
        BE_LIBRARY_GET_INSTANCE.call(c, 0x0013_5d44)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = CHAR_INTERFACE;
        BE_LIBRARY_GET_INTERFACE.call(c, 0x0013_5d50)?;
        let rider = Addr(this).off(RIDER).lw(c);
        c.g[V1][0] = rider;
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = Addr(rider as u32).off(RIDER_ATTRIB4).lw(c);
        c.g[A1][0] = Addr(rider as u32).off(RIDER_INDEX).lw(c);
        BE_CHAR_INT_GET_ATTRIB4_FACTOR.call(c, 0x0013_5d64)?;
        let k = Addr(lit(c, ATTRIB_GP)).read_fpu(c);
        c.f[3] = k.0;
        c.g[AT][0] = 0x3f80_0000;
        let one = FpuF32(0x3f80_0000);
        let scaled = fpr(c, 0) * k;
        let total = Addr(total_out).read_fpu(c);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0);
        let denom = scaled + one;
        c.f[0] = denom.0;
        let m = one.div(denom);
        c.f[2] = m.0;
        let total = total * m;
        c.f[1] = total.0;
        Addr(total_out).write_fpu(c, total);
        frame.restore(c, S2, 0x10);
        let done = Addr(done_out).read_fpu(c) * m;
        c.f[0] = done.0;
        Addr(done_out).write_fpu(c, done);
        frame.restore(c, S1, 0x20);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------- GameCamera_UpdateStreamView

/// `void GameCamera_UpdateStreamView(GameCamera* cam)` (0x15ec98): point
/// the world-cache streamer slot of this camera at the camera position with
/// a radius of 1.5x the view distance (+8, clamped to [0, 15000] except in
/// mode 1 of the game module's race state), then flag the camera
/// (`+0xb4 = 1`) when the streamer reports a clearance at least as large
/// as the far limit (2/3 of the distance clamped to [15000, 30000]) or the
/// view distance itself.
pub fn game_camera_update_stream_view(c: &mut Cpu) {
    /// `GameCamera` fields.
    const VIEW_DIST: u32 = 0x8;
    const POSITION: u32 = 0x20;
    const STREAM_SLOT: u32 = 0xb0;
    const CLEARANCE_OK: u32 = 0xb4;
    /// Streamer slots: `0x50` bytes each, the active flag at +0x250 of the
    /// world cache (the streamer starts at +0x10).
    const SLOT_SIZE: u32 = 0x50;
    const SLOT_ACTIVE: u32 = 0x250;
    const STREAMER: u32 = 0x10;
    /// `cWorldCache_StreamerSetView(streamer, slot, pos, radius)`.
    const STREAMER_SET_VIEW: u32 = 0x003a_9658;
    /// `cWorldCache_StreamerGetViewClearance(streamer, slot, &pos, &t)`.
    const STREAMER_GET_VIEW_CLEARANCE: u32 = 0x003a_96e0;
    /// Globals: the app (`gp-0x848`), the world cache pointer (`gp+0x16c8`).
    const APP_GP: u32 = 0x848;
    const WORLD_CACHE_GP: u32 = 0x16c8;
    /// `.lit4`: 15000, 2/3, 15000, 30000.
    const MAX_DIST_GP: u32 = 0x6c68;
    const FAR_SCALE_GP: u32 = 0x6c64;
    const FAR_MIN_GP: u32 = 0x6c60;
    const FAR_MAX_GP: u32 = 0x6c5c;
    let lit = |c: &Cpu, off: u32| r32(c, GP).wrapping_sub(off);
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        let app = Addr(lit(c, APP_GP)).lw(c);
        c.g[A1][0] = app;
        frame.save(c, S1, 0x40);
        c.g[A2][0] = 1;
        frame.save(c, S0, 0x50);
        let cam = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 21, 0x68);
        frame.save_fpr(c, 20, 0x60);
        let module = Addr(app as u32).off(0x84).lw(c);
        c.g[V0][0] = module;
        let race = Addr(module as u32).off(0x84).lw(c);
        c.g[V1][0] = race;
        let mode = Addr(race as u32).off(0x10).lw(c);
        c.g[A0][0] = mode;
        // The view distance, clamped unless in mode 1.
        let dist = if mode != 1 {
            let raw = Addr(cam).off(VIEW_DIST).read_fpu(c);
            c.f[1] = raw.0;
            c.cycles += 6;
            c.f[0] = 0;
            let max = Addr(lit(c, MAX_DIST_GP)).read_fpu(c);
            c.f[2] = max.0;
            let positive = FpuF32(0).le(raw);
            abi::set_fcc(c, positive);
            let mut d = FpuF32(0);
            if positive {
                c.cycles += 1;
                d = raw.min(max);
            }
            c.f[3] = d.0;
            c.cycles += 1;
            d
        } else {
            c.cycles += 2;
            Addr(cam).off(VIEW_DIST).read_fpu(c)
        };
        c.f[20] = dist.0;

        // Radius and far limit.
        c.cycles += 11;
        let k = Addr(lit(c, FAR_SCALE_GP)).read_fpu(c);
        let far_min = Addr(lit(c, FAR_MIN_GP)).read_fpu(c);
        c.f[2] = far_min.0;
        let scaled = dist * k;
        c.f[0] = scaled.0;
        c.g[AT][0] = 0x3fc0_0000;
        let one_half = FpuF32(0x3fc0_0000);
        c.f[1] = one_half.0;
        let mut far = far_min;
        let far_max = Addr(lit(c, FAR_MAX_GP)).read_fpu(c);
        c.f[3] = far_max.0;
        let above = far_min.le(scaled);
        abi::set_fcc(c, above);
        c.f[12] = (dist * one_half).0;
        if above {
            c.cycles += 1;
            far = scaled.min(far_max);
        }
        c.f[21] = far.0;

        // Mark the slot active and set the view.
        c.cycles += 22;
        c.g[V1][0] = Addr(cam).off(STREAM_SLOT).lw(c);
        c.g[V0][0] = SLOT_SIZE as u64;
        let cache_ptr = Addr(r32(c, GP).wrapping_add(WORLD_CACHE_GP)).lw(c);
        c.g[S0][0] = cache_ptr;
        c.g[S2][0] = 1;
        c.mult(0, V1, V1, V0);
        c.g[A2][0] = sx(cam.wrapping_add(POSITION));
        let cache = Addr(cache_ptr as u32).lw(c);
        c.g[V0][0] = abi::addu(cache, c.g[V1][0]);
        Addr(r32(c, V0)).off(SLOT_ACTIVE).sw(c, 1);
        let cache = Addr(cache_ptr as u32).lw(c);
        c.g[A1][0] = Addr(cam).off(STREAM_SLOT).lw(c);
        c.g[A0][0] = sx((cache as u32).wrapping_add(STREAMER));
        call_ptr(c, STREAMER_SET_VIEW, 0x0015_ed60)?;
        Addr(cam).off(CLEARANCE_OK).sw(c, 0);
        c.g[A2][0] = c.g[SP][0];
        c.g[A1][0] = Addr(cam).off(STREAM_SLOT).lw(c);
        let clearance_at = frame.at(c, 0x10);
        c.g[A3][0] = sx(clearance_at);
        let cache = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = sx((cache as u32).wrapping_add(STREAMER));
        call_ptr(c, STREAMER_GET_VIEW_CLEARANCE, 0x0015_ed7c)?;
        let have = c.g[V0][0] != 0;
        let clearance = Addr(frame.at(c, 0x10)).read_fpu(c);
        c.f[0] = clearance.0;
        if !have {
            c.cycles += 1;
        } else {
            c.cycles += 4;
            let far_ok = far.le(clearance);
            abi::set_fcc(c, far_ok);
            if far_ok {
                Addr(cam).off(CLEARANCE_OK).sw(c, 1);
                c.cycles += 1;
            } else {
                c.cycles += 4;
                let near_ok = dist.le(clearance);
                abi::set_fcc(c, near_ok);
                if near_ok {
                    c.cycles += 1;
                    Addr(cam).off(CLEARANCE_OK).sw(c, 1);
                    c.cycles += 1;
                }
            }
        }
        frame.restore(c, S0, 0x50);
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    });
}

// -------------------------------------------- MultiParticle_EmitAtInstances

/// `void MultiParticle_EmitAtInstances(MultiParticle* self)` (0x358260):
/// for each of the `count` world-instance handles in `refs`, resolve the
/// handle through the world's handle table (low byte: table index, upper
/// bits: slot); for a live instance, offset the emit box (+0x20 min, +0x30
/// max) by the instance position (+0x40) and render the emitter descriptor
/// (+0x10) there, at instance position + `center` (+0x40).
pub fn multi_particle_emit_at_instances(c: &mut Cpu) {
    /// `MultiParticle` fields.
    const COUNT: u32 = 0x4;
    const REFS: u32 = 0xc;
    const DESC: u32 = 0x10;
    const BOX_MIN: u32 = 0x20;
    const BOX_MAX: u32 = 0x30;
    const CENTER: u32 = 0x40;
    /// Instance position.
    const INSTANCE_POS: u32 = 0x40;
    /// `World* World_Get()` (`*(gp+0x16c8)`).
    const WORLD_GET: u32 = 0x002d_1bd8;
    /// `Emitter_RenderAt(desc, box, pos)`.
    const EMITTER_RENDER_AT: u32 = 0x0037_0888;
    /// The loop head (event checks).
    const LOOP: u32 = 0x0035_8280;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x50);
        c.g[V0][0] = Addr(this).off(COUNT).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !empty {
            loop {
                c.cycles += 14;
                call_ptr(c, WORLD_GET, 0x0035_8288)?;
                let world = r32(c, V0);
                let i = r32(c, S1);
                // Resolve the handle.
                let refs = Addr(this).off(REFS).lw(c);
                c.g[A0][0] = refs;
                let tables = Addr(world).lw(c);
                c.g[A1][0] = tables;
                let handle = Addr((i << 2).wrapping_add(refs as u32)).lw(c);
                c.g[V1][0] = handle;
                let table = Addr(tables as u32).off(8).lw(c);
                let entry_at = ((handle as u32 & 0xff) << 2).wrapping_add(table as u32);
                let entry = Addr(entry_at).lw(c);
                c.g[A0][0] = entry;
                let slot = handle as u32 >> 8;
                c.g[V0][0] = sx(slot);
                let instance = if entry == 0 {
                    c.cycles += 2;
                    0
                } else {
                    c.cycles += 7;
                    let slots = Addr(entry as u32).off(0x1c).lw(c);
                    c.g[A0][0] = slots;
                    let word = Addr((slot << 2).wrapping_add(slots as u32)).lw(c);
                    c.g[V1][0] = word;
                    let index = word as u32 >> 8;
                    let mut p = sx(index << 2);
                    if index == 0 {
                        c.cycles += 1;
                        p = 0;
                    }
                    c.g[V0][0] = p;
                    c.cycles += 1;
                    p
                };
                c.g[A0][0] = instance;

                c.cycles += 2;
                c.g[A1][0] = c.g[SP][0];
                if instance != 0 {
                    c.cycles += 27;
                    let inst = instance as u32;
                    let at = |c: &Cpu, off: u32| sp(c).wrapping_add(off);
                    // box = self->box + pos (both corners).
                    c.lq(V0, this.wrapping_add(BOX_MIN));
                    c.lq(V1, this.wrapping_add(BOX_MAX));
                    c.g[A2][0] = sx(at(c, 0x20));
                    c.sq(V0, at(c, 0));
                    c.sq(V1, at(c, 0x10));
                    c.lq(V0, inst.wrapping_add(INSTANCE_POS));
                    c.sq(V0, at(c, 0x20));
                    c.lqc2(3, at(c, 0));
                    c.lqc2(4, at(c, 0x20));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, 0));
                    c.lq(V0, inst.wrapping_add(INSTANCE_POS));
                    c.sq(V0, at(c, 0x20));
                    c.lqc2(3, at(c, 0x10));
                    c.lqc2(4, at(c, 0x20));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, 0x10));
                    // at = pos + center.
                    c.lq(V0, inst.wrapping_add(INSTANCE_POS));
                    c.sq(V0, at(c, 0x30));
                    c.lqc2(3, at(c, 0x30));
                    c.lqc2(4, this.wrapping_add(CENTER));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, 0x40));
                    c.lq(V0, at(c, 0x40));
                    c.g[A0][0] = Addr(this).off(DESC).lw(c);
                    c.sq(V0, at(c, 0x20));
                    call_ptr(c, EMITTER_RENDER_AT, 0x0035_8358)?;
                }

                c.cycles += 5;
                let count = Addr(this).off(COUNT).lw(c);
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                let more = (c.g[S1][0] as i64) < (count as i64);
                c.g[V0][0] = more as u64;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, LOOP);
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------- logf

/// `float logf(float x)` (0x40da10): newlib's `w_logf.c` wrapper. Returns
/// `__ieee754_logf(x)`; unless `_LIB_VERSION` is `_IEEE_` or `x` is NaN or
/// positive, it builds a `struct exception` on the stack ("log", `x` as
/// both doubles, retval `-HUGE` for `_SVID_` else `-HUGE_VAL`), sets type
/// `SING` for 0 / `DOMAIN` for negatives, sets `errno` (ERANGE / EDOM) for
/// `_POSIX_` or when `matherr` declines, copies the struct's `err` to
/// `errno` when nonzero, and returns the retval as a float.
pub fn logf(c: &mut Cpu) {
    const IEEE754_LOGF: Callee = Callee {
        addr: 0x0040_e550,
        f: ee::s15::__ieee754_logf,
    };
    const ISNANF: Callee = Callee {
        addr: 0x0040_f9d0,
        f: ee::s15::isnanf,
    };
    const EXTENDSFDF2: Callee = Callee {
        addr: 0x0041_3af8,
        f: ee::s15::__extendsfdf2,
    };
    const SUBDF3: Callee = Callee {
        addr: 0x0041_1f70,
        f: ee::s15::__subdf3,
    };
    const GETREENT: Callee = Callee {
        addr: 0x0041_5fb8,
        f: ee::s15::__getreent,
    };
    const MATHERR: Callee = Callee {
        addr: 0x0040_f878,
        f: ee::s15::matherr,
    };
    const TRUNCDFSF2: Callee = Callee {
        addr: 0x0041_27a8,
        f: ee::s15::__truncdfsf2,
    };
    /// `_LIB_VERSION` (`0x4a0000 - 0x6978`): -1 `_IEEE_`, 0 `_SVID_`, 2 `_POSIX_`.
    const HI: u32 = 0x004a_0000;
    const LIB_VERSION: u32 = HI - 0x6978;
    const IEEE: u64 = u64::MAX;
    const SVID: u32 = 0;
    const POSIX: u64 = 2;
    /// The name "log", the `-HUGE` (SVID) double, `HUGE_VAL` (+inf).
    const NAME_LOG: u32 = HI - 0x6f58;
    const MINUS_HUGE: u32 = HI - 0x6f50;
    const HUGE_VAL: u32 = HI - 0x6980;
    /// `struct exception` on the stack.
    const EXC_TYPE: u32 = 0x0;
    const EXC_NAME: u32 = 0x4;
    const EXC_ARG1: u32 = 0x8;
    const EXC_ARG2: u32 = 0x10;
    const EXC_RETVAL: u32 = 0x18;
    const EXC_ERR: u32 = 0x20;
    /// Exception types and errno values.
    const DOMAIN: u64 = 1;
    const SING: u64 = 2;
    const EDOM: u64 = 0x21;
    const ERANGE: u64 = 0x22;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save_fpr(c, 20, 0x60);
        let x = FpuF32(c.f[12]);
        c.f[20] = x.0;
        let a = frame.at(c, 0x40);
        c.sd(a, c.g[S1][0]);
        let a = frame.at(c, 0x30);
        c.sd(a, c.g[S0][0]);
        frame.save_fpr(c, 21, 0x68);
        frame.save_ra(c, 0x50);
        c.g[S1][0] = HI as u64;
        IEEE754_LOGF.call(c, 0x0040_da34)?;
        let version = Addr(LIB_VERSION).lw(c);
        c.g[S0][0] = version;
        c.g[V0][0] = IEEE;
        c.f[21] = c.f[0];
        let mut error = false;
        if version != IEEE {
            c.cycles += 4;
            c.f[12] = x.0;
            ISNANF.call(c, 0x0040_da4c)?;
            let nan = c.g[V0][0] != 0;
            c.f[0] = c.f[21];
            if nan {
                return finish(c, frame);
            }
            c.cycles += 5;
            c.f[0] = 0;
            let positive = FpuF32(0).lt(x);
            abi::set_fcc(c, positive);
            c.g[V0][0] = HI as u64;
            error = !positive;
        }
        if !error {
            c.cycles += 2;
            c.f[0] = c.f[21];
            return finish(c, frame);
        }

        // Fill the exception record.
        c.cycles += 8;
        let exc = |c: &Cpu, off: u32| sp(c).wrapping_add(off);
        Addr(exc(c, EXC_ERR)).sw(c, 0);
        c.g[V0][0] = sx(NAME_LOG);
        c.f[12] = x.0;
        Addr(exc(c, EXC_NAME)).sw(c, NAME_LOG as u64);
        EXTENDSFDF2.call(c, 0x0040_da84)?;
        let a = exc(c, EXC_ARG1);
        c.sd(a, c.g[V0][0]);
        let a = exc(c, EXC_ARG2);
        c.sd(a, c.g[V0][0]);
        if c.g[S0][0] == SVID as u64 {
            c.cycles += 4;
            c.g[AT][0] = HI as u64;
            c.g[V0][0] = c.ld(MINUS_HUGE);
        } else {
            c.cycles += 5;
            c.g[V0][0] = HI as u64;
            c.g[A0][0] = 0;
            c.g[A1][0] = c.ld(HUGE_VAL);
            SUBDF3.call(c, 0x0040_dab0)?;
        }
        let a = exc(c, EXC_RETVAL);
        c.sd(a, c.g[V0][0]);

        // Classify, then errno.
        c.cycles += 5;
        c.f[0] = 0;
        let zero = x.eq_s(FpuF32(0));
        abi::set_fcc(c, zero);
        c.g[V0][0] = SING;
        let errno_value;
        let mut ask_matherr = false;
        if zero {
            c.cycles += 3;
            c.g[V1][0] = Addr(LIB_VERSION).lw(c);
            Addr(exc(c, EXC_TYPE)).sw(c, SING);
            if c.g[V1][0] != POSIX {
                ask_matherr = true;
                errno_value = 0;
            } else {
                c.cycles += 4;
                GETREENT.call(c, 0x0040_dadc)?;
                errno_value = ERANGE;
            }
        } else {
            c.cycles += 5;
            c.g[V1][0] = DOMAIN;
            c.g[V0][0] = Addr(LIB_VERSION).lw(c);
            c.g[A0][0] = POSIX;
            Addr(exc(c, EXC_TYPE)).sw(c, DOMAIN);
            ask_matherr = c.g[V0][0] != POSIX;
            errno_value = EDOM;
        }
        let mut set_errno = !ask_matherr || errno_value == ERANGE;
        if ask_matherr {
            c.cycles += 4;
            c.g[A0][0] = c.g[SP][0];
            MATHERR.call(c, 0x0040_db00)?;
            let handled = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(exc(c, EXC_ERR)).lw(c);
            set_errno = !handled;
        }
        if set_errno {
            let code = if errno_value == ERANGE {
                ERANGE
            } else {
                // `DOMAIN`, or a declined `SING` (EDOM here as well).
                c.cycles += 3;
                GETREENT.call(c, 0x0040_db10)?;
                EDOM
            };
            c.g[V1][0] = code;
            c.cycles += 2;
            Addr(r32(c, V0)).sw(c, code);
            c.g[V0][0] = Addr(exc(c, EXC_ERR)).lw(c);
        }
        // A nonzero `err` from matherr goes to errno.
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            GETREENT.call(c, 0x0040_db2c)?;
            let err = Addr(exc(c, EXC_ERR)).lw(c);
            c.g[V1][0] = err;
            Addr(r32(c, V0)).sw(c, err);
        }
        c.cycles += 2;
        let a = exc(c, EXC_RETVAL);
        c.g[A0][0] = c.ld(a);
        TRUNCDFSF2.call(c, 0x0040_db3c)?;
        finish(c, frame)
    });

    /// The epilogue.
    fn finish(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
        c.cycles += 7;
        frame.restore_ra(c, 0x50);
        let a = frame.at(c, 0x40);
        c.g[S1][0] = c.ld(a);
        let a = frame.at(c, 0x30);
        c.g[S0][0] = c.ld(a);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    }
}

// -------------------------------------------------------- LensFX_DrawFlares

/// `void LensFX_DrawFlares(LensFX* fx, int view)` (0x2e3478): draw the sun
/// of this view (when the sun is enabled, `gp+0xa28`), then, between the
/// lens FX's push / pop render-state methods (vtable at +0x649c, slots
/// +0x10 and +0x18), every flare quad of the view whose "on" flag is set.
/// Per view: a flare count at `+0x60 + 4*view`, `0x3000` bytes of flares
/// (`0x30` each, the "on" halfword at flare +0x2c) from +0x68, and a sun
/// record of `0x210` bytes at +0x6070.
pub fn lens_fx_draw_flares(c: &mut Cpu) {
    const DRAW_SUN: Callee = Callee {
        addr: 0x002f_4a08,
        f: ee::s09::LensFX_DrawSun,
    };
    const DRAW_FLARE_QUAD: Callee = Callee {
        addr: 0x002e_2868,
        f: ee::s09::LensFX_DrawFlareQuad,
    };
    /// `gp+0xa28`: the sun is enabled.
    const SUN_ENABLED_GP: u32 = 0xa28;
    /// `LensFX` layout.
    const COUNTS: u32 = 0x60;
    const FLARES: u32 = 0x68;
    const FLARES_PER_VIEW: u32 = 0x3000;
    const FLARE_SIZE: u32 = 0x30;
    /// The flare's "on" halfword (`FLARES + 0x2c`), relative to the lens FX.
    const FLARE_ON: u32 = 0x94;
    const SUNS: u32 = 0x6070;
    const SUN_SIZE: u32 = 0x210;
    const VTABLE: u32 = 0x649c;
    /// Vtable slots `{delta: i16, method}`: push / pop render state.
    const PUSH_STATE: u32 = 0x10;
    const POP_STATE: u32 = 0x18;
    /// Loop head (event checks).
    const LOOP: u32 = 0x002e_3510;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        let sun_on = Addr(r32(c, GP).wrapping_add(SUN_ENABLED_GP)).lw(c);
        c.g[V0][0] = sun_on;
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        let fx = r32(c, A0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        let view = r32(c, A1);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        if sun_on != 0 {
            c.cycles += 7;
            c.g[V0][0] = sx(fx.wrapping_add(SUNS));
            let off = abi::mult(c, view, SUN_SIZE);
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S4][0];
            c.g[A0][0] = sx(fx.wrapping_add(SUNS).wrapping_add(off));
            DRAW_SUN.call(c, 0x002e_34c4)?;
        }

        // Push the render state.
        c.cycles += 12;
        let vt = Addr(fx).off(VTABLE).lw(c);
        c.g[V0][0] = vt;
        c.g[S3][0] = 0;
        let delta = Addr(vt as u32).off(PUSH_STATE).lh(c);
        let method = Addr(vt as u32).off(PUSH_STATE + 4).lw(c);
        c.g[V1][0] = method;
        c.g[A0][0] = abi::addu(c.g[S2][0], delta);
        call_ptr(c, method as u32, 0x002e_34dc)?;
        c.g[A0][0] = sx(view << 2);
        c.g[V0][0] = sx(fx.wrapping_add(COUNTS));
        let count_at = fx.wrapping_add(COUNTS).wrapping_add(view << 2);
        c.g[A1][0] = sx(count_at);
        let count = Addr(count_at).lw(c);
        c.g[V1][0] = count;
        c.g[V0][0] = FLARES_PER_VIEW as u64;
        if (count as i64) > 0 {
            c.cycles += 7;
            c.g[A0][0] = sx(fx.wrapping_add(FLARES));
            let block = abi::mult(c, view, FLARES_PER_VIEW);
            c.g[V0][0] = sx(block);
            c.g[S5][0] = c.g[A1][0];
            c.g[V1][0] = sx(block.wrapping_add(FLARE_ON));
            c.g[S1][0] = sx(fx.wrapping_add(FLARES).wrapping_add(block));
            c.g[S0][0] = sx(block.wrapping_add(FLARE_ON).wrapping_add(fx));
            loop {
                c.cycles += 3;
                let on = Addr(r32(c, S0)).lhu(c);
                c.g[V0][0] = on;
                c.g[A0][0] = c.g[S1][0];
                if on != 0 {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S4][0];
                    DRAW_FLARE_QUAD.call(c, 0x002e_3524)?;
                }
                c.cycles += 6;
                let count = Addr(r32(c, S5)).lw(c);
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(FLARE_SIZE));
                let more = (c.g[S3][0] as i64) < (count as i64);
                c.g[V0][0] = more as u64;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(FLARE_SIZE));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, LOOP);
                }
            }
        }

        // Pop the render state.
        c.cycles += 14;
        let vt = Addr(fx).off(VTABLE).lw(c);
        c.g[V0][0] = vt;
        let delta = Addr(vt as u32).off(POP_STATE).lh(c);
        let method = Addr(vt as u32).off(POP_STATE + 4).lw(c);
        c.g[V1][0] = method;
        c.g[A0][0] = abi::addu(c.g[S2][0], delta);
        call_ptr(c, method as u32, 0x002e_3550)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------- ScriptAudioObj_UpdateSounds

/// `void ScriptAudioObj_UpdateSounds(ScriptAudioObj* self)` (0x280640):
/// for each of the four sound triggers (`+0xbc`, `0xc` bytes each: sound
/// id, stop request, target), a trigger with a sound either stops it now
/// (stop requested) or is cleared once the bank monitor no longer has an
/// instance playing it; a cleared trigger gets id and target -1 and no
/// stop request. Then, if a speech cue is pending (`+0xf4`), clear it and,
/// when speech instance 0 is ready, skip one entry of the script engine's
/// current list.
pub fn script_audio_obj_update_sounds(c: &mut Cpu) {
    const AUDIO_GET_INSTANCE: Callee = Callee {
        addr: 0x0028_b180,
        f: ee::s07::SSXAudioSystem_GetInstance,
    };
    const LIST_SKIP: Callee = Callee {
        addr: 0x0027_9070,
        f: ee::s07::ScriptEngine_ListSkip,
    };
    /// `BankMonitor_StopHandle(bm, id, kind, fade)`.
    const BANK_MONITOR_STOP_HANDLE: u32 = 0x002a_d5f0;
    /// `BankMonitor_FindInstance(bm, id, kind)`.
    const BANK_MONITOR_FIND_INSTANCE: u32 = 0x002a_d550;
    /// `SpeechSystem_IsInstanceReady(speech, index)`.
    const SPEECH_IS_INSTANCE_READY: u32 = 0x002b_0e28;
    /// `ScriptAudioObj` fields.
    const TRIGGERS: u32 = 0xbc;
    const TRIGGER_SIZE: u32 = 0xc;
    const TRIGGER_ID: u32 = 0x0;
    const TRIGGER_STOP: u32 = 0x4;
    const TRIGGER_TARGET: u32 = 0x8;
    const SPEECH_PENDING: u32 = 0xf4;
    /// `SSXAudioSystem`: the sound root (`**root + 0x1d8` is the bank
    /// monitor) and the speech system.
    const ROOT: u32 = 0x118;
    const BANK_MONITOR: u32 = 0x1d8;
    const SPEECH: u32 = 0x5560;
    /// The script engine (`gp-0x84c`) and its current list index.
    const ENGINE_GP: u32 = 0x84c;
    const ENGINE_LIST: u32 = 0x550;
    /// Bank-monitor instance kind 1: an SND handle.
    const KIND_SND: u64 = 1;
    /// Loop head (event checks).
    const LOOP: u32 = 0x0028_0678;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S4, 0x10);
        frame.save(c, S0, 0x50);
        let this = r32(c, A0);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx(this.wrapping_add(TRIGGERS));
        frame.save(c, S2, 0x30);
        c.g[S1][0] = u64::MAX;
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        c.g[S3][0] = 3;
        AUDIO_GET_INSTANCE.call(c, 0x0028_0670)?;
        c.g[S2][0] = c.g[V0][0];
        let audio = r32(c, S2);
        let monitor = |c: &mut Cpu| {
            let root = Addr(audio).off(ROOT).lw(c);
            c.g[V0][0] = root;
            let snd = Addr(root as u32).lw(c);
            c.g[A0][0] = sx((snd as u32).wrapping_add(BANK_MONITOR));
        };
        loop {
            c.cycles += 3;
            let trigger = r32(c, S0);
            let id = Addr(trigger).off(TRIGGER_ID).lw(c);
            c.g[A1][0] = id;
            let mut clear = false;
            if (id as i64) >= 0 {
                c.cycles += 3;
                let stop = Addr(trigger).off(TRIGGER_STOP).lw(c);
                c.g[V0][0] = stop;
                c.g[A2][0] = KIND_SND;
                if stop != 0 {
                    c.cycles += 7;
                    c.f[12] = 0;
                    monitor(c);
                    call_ptr(c, BANK_MONITOR_STOP_HANDLE, 0x0028_06a4)?;
                    Addr(r32(c, S0)).off(TRIGGER_TARGET).sw(c, u64::MAX);
                    clear = true;
                } else {
                    c.cycles += 6;
                    monitor(c);
                    call_ptr(c, BANK_MONITOR_FIND_INSTANCE, 0x0028_06bc)?;
                    if c.g[V0][0] == 0 {
                        c.cycles += 1;
                        Addr(r32(c, S0)).off(TRIGGER_TARGET).sw(c, u64::MAX);
                        clear = true;
                    }
                }
            }
            if clear {
                c.cycles += 3;
                let trigger = r32(c, S0);
                Addr(trigger).off(TRIGGER_ID).sw(c, u64::MAX);
                Addr(trigger).off(TRIGGER_STOP).sw(c, 0);
            }
            c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
            c.cycles += 2;
            let more = (c.g[S3][0] as i64) >= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(TRIGGER_SIZE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, LOOP);
            }
        }

        c.cycles += 3;
        let pending = Addr(this).off(SPEECH_PENDING).lw(c);
        c.g[V0][0] = pending;
        c.g[A0][0] = sx(audio.wrapping_add(SPEECH));
        if pending != 0 {
            c.cycles += 5;
            Addr(this).off(SPEECH_PENDING).sw(c, 0);
            c.g[A1][0] = 0;
            call_ptr(c, SPEECH_IS_INSTANCE_READY, 0x0028_06f4)?;
            let ready = c.g[V0][0] != 0;
            let engine = Addr(r32(c, GP).wrapping_sub(ENGINE_GP)).lw(c);
            c.g[A0][0] = engine;
            if ready {
                c.cycles += 4;
                c.g[A2][0] = 1;
                c.g[A3][0] = 1;
                c.g[A1][0] = Addr(engine as u32).off(ENGINE_LIST).lw(c);
                LIST_SKIP.call(c, 0x0028_070c)?;
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ------------------------------------------------ RailModifier_UpdateBounds

/// `void RailModifier_UpdateBounds(RailModifier* self)` (0x35c040): if the
/// owner's object (`self+0x40 -> +0xc`) says it has bounds (virtual at
/// vtable +0x160), copy its bounding box (virtual at +0x168: min, max) to
/// `self+0x10` / `self+0x20`, grown by 100 units on x, y and z.
pub fn rail_modifier_update_bounds(c: &mut Cpu) {
    /// `RailModifier` fields.
    const OWNER: u32 = 0x40;
    const BOX_MIN: u32 = 0x10;
    const BOX_MAX: u32 = 0x20;
    /// The owner's object and its vtable slots `{delta: i16, method}`.
    const OWNER_OBJECT: u32 = 0xc;
    const VTABLE: u32 = 0xc;
    const HAS_BOUNDS: u32 = 0x160;
    const GET_BOUNDS: u32 = 0x168;
    /// 100.0, the margin.
    const MARGIN: u64 = 0x42c8_0000;
    /// `object->vtable[slot](object)`.
    fn call_object(c: &mut Cpu, this: u32, slot: u32, ret: u32) -> crate::call::Flow {
        let owner = Addr(this).off(OWNER).lw(c);
        c.g[V0][0] = owner;
        let object = Addr(owner as u32).off(OWNER_OBJECT).lw(c);
        c.g[A1][0] = object;
        let vt = Addr(object as u32).off(VTABLE).lw(c);
        c.g[V1][0] = vt;
        let delta = Addr(vt as u32).off(slot).lh(c);
        let method = Addr(vt as u32).off(slot + 4).lw(c);
        c.g[V0][0] = method;
        c.g[A0][0] = abi::addu(object, delta);
        call_ptr(c, method as u32, ret)
    }
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        call_object(c, this, HAS_BOUNDS, 0x0035_c06c)?;
        if c.g[V0][0] != 0 {
            c.cycles += 26;
            call_object(c, this, GET_BOUNDS, 0x0035_c090)?;
            let bounds = r32(c, V0);
            c.lq(V1, bounds);
            c.g[AT][0] = MARGIN;
            c.f[0] = MARGIN as u32;
            c.sq(V1, this.wrapping_add(BOX_MIN));
            c.lq(A0, bounds.wrapping_add(0x10));
            // The margin vector (100, 100, 100, 0) on the stack.
            let pad = sp(c);
            Addr(pad).off(0x8).write_fpu(c, FpuF32(MARGIN as u32));
            c.sq(A0, this.wrapping_add(BOX_MAX));
            Addr(pad).write_fpu(c, FpuF32(MARGIN as u32));
            Addr(pad).off(0x4).write_fpu(c, FpuF32(MARGIN as u32));
            Addr(pad).off(0xc).sw(c, 0);
            c.lqc2(3, this.wrapping_add(BOX_MIN));
            c.lqc2(4, pad);
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, this.wrapping_add(BOX_MIN));
            c.lqc2(3, this.wrapping_add(BOX_MAX));
            c.lqc2(4, pad);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, this.wrapping_add(BOX_MAX));
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

// -------------------------------------- ComputerActorRider_CruiseChaseRider

/// `void ComputerActorRider_CruiseChaseRider(ComputerActorRider* self,
/// ControlInput* pad)` (0x100f88): the "chase a rider" cruise step. With
/// the target rider (AI rider slot `self+0xe70`) and the record of that
/// slot in the model (`model + slot*0x24`: distance +8, heading +0xc):
///
/// - heading difference to the target wrapped to [-pi, pi]; own and
///   target speeds (the target's velocity from its path interface, vtable
///   +0x10 at `+0x6c0`);
/// - behind (|angle| > pi/2): steer at the path, boost when slower than
///   the target, else cruise-crouch;
/// - otherwise the "close enough" angle `atan(150 / d)` (pi/2 at d == 0,
///   +pi when d < 0): outside it, or not faster than the target, steer and
///   crouch; else when the target is ready (vtable +0x48), re-select the AI
///   path if both follow the same one, then steer and crouch;
/// - pack the steering (`* 31`, 6 bits at bit 20) into the pad flags, then
///   resolve the jump state as the other cruise steps do.
///
/// The original's blocks are laid out in address order and entered by
/// falling out of the previous one; each `'b_…` label is one of them.
pub fn computer_actor_rider_cruise_chase_rider(c: &mut Cpu) {
    use crate::game::ai::computer::{cruise, jump_desc_copy, vu0_length_sq};
    const SELECT_AI_PATH: Callee = Callee {
        addr: 0x0011_2a50,
        f: ee::s00::Rider_SelectAIPath,
    };
    /// `float Math_Atan(float)`.
    const MATH_ATAN: u32 = 0x0031_c228;
    /// Rider fields.
    const R_MODEL: u32 = 0x18;
    const R_MARKER_DIST: u32 = 0xdf0;
    const R_TARGET: u32 = 0xe70;
    const R_JUMP_STATE: u32 = 0xf38;
    /// Rider-slot record in the model: `0x24` bytes, distance and heading.
    const REC_SIZE: u64 = 0x24;
    const REC_DIST: u32 = 0x8;
    const REC_HEADING: u32 = 0xc;
    /// Model fields: heading, velocity, followed AI path.
    const M_HEADING: u32 = 0x4cc;
    const M_VELOCITY: u32 = 0x1e0;
    const M_AI_PATH: u32 = 0xab8;
    /// The target rider entry's path interface (`{delta, fn}` slots).
    const E_IFACE: u32 = 0x6c0;
    const IFACE_GET_VELOCITY: u32 = 0x10;
    const IFACE_READY: u32 = 0x48;
    /// The jump descriptors (`0x450000 - 0x30e0`).
    const JUMP_DESCS: u32 = 0x0044_0000 - 0x30e0;
    let gp_lit = |c: &Cpu, off: u32| r32(c, GP).wrapping_sub(off);
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0xc0);
        let a = gp_lit(c, 0x7fa8);
        c.lwc1(0, a); // 1 / 2pi
        frame.save(c, S0, 0x80);
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S4, 0x40);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x70);
        c.g[S4][0] = REC_SIZE;
        frame.save(c, S3, 0x50);
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 24, 0xb0);
        frame.save_fpr(c, 23, 0xa8);
        frame.save_fpr(c, 22, 0xa0);
        frame.save_fpr(c, 21, 0x98);
        frame.save_fpr(c, 20, 0x90);
        let this = r32(c, S0);
        c.g[V0][0] = Addr(gp_lit(c, 0x848)).lw(c);
        c.g[A0][0] = Addr(this).off(R_TARGET).lw(c);
        c.g[A2][0] = Addr(this).off(R_MODEL).lw(c);
        c.mult(0, A0, A0, S4);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 3); // 0.5
        c.lwc1(2, r32(c, A2).wrapping_add(M_HEADING));
        c.g[A1][0] = Addr(r32(c, V0)).off(0x84).lw(c);
        c.g[V1][0] = Addr(this).off(R_TARGET).lw(c);
        c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
        c.g[V0][0] = Addr(r32(c, A1)).off(0xc).lw(c);
        c.lwc1(1, r32(c, A0).wrapping_add(REC_HEADING));
        c.g[V1][0] = sx(r32(c, V1) << 2);
        c.g[V0][0] = abi::addu(c.g[V0][0], c.g[V1][0]);
        c.sub_s(1, 1, 2);
        c.g[S1][0] = Addr(r32(c, V0)).off(0x28).lw(c);
        // floor(diff / 2pi + 0.5)
        c.mul_s(0, 1, 0);
        c.add_s(0, 0, 3);
        c.cvt_w_s(2, 0);
        c.cvt_s_w(2, 2);
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.cycles += 3;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.sub_s(2, 2, 0);
        }

        'b_101200: {
            'b_1011a4: {
                'b_101190: {
                    'b_1010fc: {
                        c.cycles += 37;
                        let a = gp_lit(c, 0x7fa4);
                        c.lwc1(0, a); // 2pi
                        c.lqc2(3, r32(c, A2).wrapping_add(M_VELOCITY));
                        vu0_length_sq(c);
                        c.cfc2(V0, 22);
                        c.mtc1(V0, 24); // own speed
                        let entry = r32(c, S1);
                        c.g[V1][0] = Addr(entry).off(E_IFACE).lw(c);
                        c.g[A1][0] = sx(entry.wrapping_add(E_IFACE));
                        c.mul_s(0, 2, 0);
                        c.g[S3][0] = c.g[A1][0];
                        let vt = r32(c, V1);
                        c.g[A0][0] = Addr(vt).off(IFACE_GET_VELOCITY).lh(c);
                        c.g[V0][0] = Addr(vt).off(IFACE_GET_VELOCITY + 4).lw(c);
                        c.sub_s(22, 1, 0); // the wrapped angle
                        let target = r32(c, V0);
                        c.g[A0][0] = abi::addu(c.g[A1][0], c.g[A0][0]);
                        call_ptr(c, target, 0x0010_1080)?;
                        c.mtc1(0, 21); // steer = 0
                        let a = gp_lit(c, 0x7fa0);
                        c.lwc1(1, a); // pi/2
                        c.abs_s(0, 22);
                        c.lqc2(3, r32(c, V0));
                        vu0_length_sq(c);
                        c.cfc2(V1, 22);
                        c.mtc1(V1, 23); // target speed
                        c.c_lt_s(1, 0);
                        let behind = c.fcc();
                        c.g[A0][0] = c.g[SP][0];
                        if !behind {
                            break 'b_1010fc;
                        }
                        c.cycles += 9;
                        c.g[A1][0] = c.g[S0][0];
                        cruise::GET_STEER_TARGET.call(c, 0x0010_10cc)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[SP][0];
                        cruise::COMPUTE_STEER.call(c, 0x0010_10d8)?;
                        c.c_lt_s(24, 23);
                        let slower = c.fcc();
                        c.mov_s(21, 0);
                        if !slower {
                            break 'b_101190;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S2][0];
                        cruise::CROUCH_AND_BOOST.call(c, 0x0010_10f4)?;
                        break 'b_101200;
                    }
                    // Ahead: the "close enough" angle from the distance.
                    'b_101178: {
                        'b_101154: {
                            'b_101128: {
                                c.cycles += 8;
                                c.g[V0][0] = Addr(this).off(R_TARGET).lw(c);
                                c.g[V1][0] = Addr(this).off(R_MODEL).lw(c);
                                c.mult(0, V0, V0, S4);
                                c.g[V1][0] = abi::addu(c.g[V1][0], c.g[V0][0]);
                                c.lwc1(20, r32(c, V1).wrapping_add(REC_DIST));
                                c.c_eq_s(20, 21);
                                if !c.fcc() {
                                    break 'b_101128;
                                }
                                c.cycles += 3;
                                let a = gp_lit(c, 0x7f9c);
                                c.lwc1(1, a); // pi/2
                                break 'b_101154;
                            }
                            c.cycles += 9;
                            c.g[AT][0] = 0x4316_0000; // 150.0
                            c.mtc1(1, 12);
                            c.div_s(12, 12, 20);
                            call_ptr(c, MATH_ATAN, 0x0010_113c)?;
                            c.c_lt_s(20, 21);
                            let negative = c.fcc();
                            c.mov_s(1, 0);
                            if negative {
                                c.cycles += 2;
                                let a = gp_lit(c, 0x7f98);
                                c.lwc1(0, a); // pi
                                c.add_s(1, 1, 0);
                            }
                        }
                        c.cycles += 5;
                        c.abs_s(0, 22);
                        c.c_lt_s(1, 0);
                        let outside = c.fcc();
                        c.g[A0][0] = c.g[SP][0];
                        if outside {
                            break 'b_101178;
                        }
                        c.cycles += 4;
                        c.c_le_s(24, 23);
                        if !c.fcc() {
                            // Faster than the target: ask whether it is ready.
                            c.g[V1][0] = Addr(r32(c, S1)).off(E_IFACE).lw(c);
                            break 'b_1011a4;
                        }
                    }
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S0][0];
                    cruise::GET_STEER_TARGET.call(c, 0x0010_1180)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[SP][0];
                    cruise::COMPUTE_STEER.call(c, 0x0010_118c)?;
                    c.mov_s(21, 0);
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S2][0];
                cruise::SET_CRUISE_CROUCH.call(c, 0x0010_119c)?;
                break 'b_101200;
            }
            c.cycles += 6;
            let vt = r32(c, V1);
            c.g[A0][0] = Addr(vt).off(IFACE_READY).lh(c);
            c.g[V0][0] = Addr(vt).off(IFACE_READY + 4).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = abi::addu(c.g[S3][0], c.g[A0][0]);
            call_ptr(c, target, 0x0010_11b4)?;
            if c.g[V0][0] == 0 {
                break 'b_101200;
            }
            c.cycles += 5;
            c.g[A0][0] = Addr(this).off(R_MODEL).lw(c);
            c.g[V1][0] = Addr(r32(c, S1)).off(M_AI_PATH).lw(c);
            c.g[V0][0] = Addr(r32(c, A0)).off(M_AI_PATH).lw(c);
            if c.g[V1][0] == c.g[V0][0] {
                // Both follow the same AI path: pick another.
                c.cycles += 3;
                c.g[A1][0] = 0;
                SELECT_AI_PATH.call(c, 0x0010_11d8)?;
            }
            c.g[A0][0] = c.g[SP][0];
            c.cycles += 9;
            c.g[A1][0] = c.g[S0][0];
            cruise::GET_STEER_TARGET.call(c, 0x0010_11e4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            cruise::COMPUTE_STEER.call(c, 0x0010_11f0)?;
            c.mov_s(21, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S2][0];
            cruise::SET_CRUISE_CROUCH.call(c, 0x0010_1200)?;
        }

        // Pack the steering, then the jump state.
        'b_1012dc: {
            c.cycles += 24;
            c.g[AT][0] = 0x41f8_0000; // 31.0
            c.mtc1(1, 0);
            c.g[V0][0] = 0xffff_ffff_fc0f_0000;
            let pad = r32(c, S2);
            c.g[V1][0] = Addr(pad).lw(c);
            c.g[V0][0] |= 0xffff;
            c.mul_s(0, 21, 0);
            let sp0 = sp(c);
            Addr(sp0).off(0x24).sw(c, 0);
            c.g[V1][0] &= c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(this.wrapping_add(R_MARKER_DIST));
            c.g[A2][0] = sx(sp0.wrapping_add(0x10));
            c.cvt_w_s(1, 0);
            c.mfc1(V0, 1);
            c.g[T0][0] = sx(sp0.wrapping_add(0x18));
            c.g[A3][0] = sx(sp0.wrapping_add(0x14));
            c.g[T1][0] = sx(sp0.wrapping_add(0x1c));
            c.g[T2][0] = sx(sp0.wrapping_add(0x20));
            c.g[V0][0] &= 0x3f;
            c.g[V0][0] = sx(r32(c, V0) << 20);
            c.g[V1][0] |= c.g[V0][0];
            let flags = c.g[V1][0];
            Addr(pad).sw(c, flags);
            cruise::FIND_JUMP_MARKER.call(c, 0x0010_1258)?;
            let none = c.g[V0][0] == 0;
            c.g[V0][0] = u64::MAX;
            if !none {
                c.cycles += 9;
                c.g[V1][0] = 0x44_0000;
                Addr(this).off(R_TARGET).sw(c, u64::MAX);
                c.g[A2][0] = sx(JUMP_DESCS);
                jump_desc_copy(c, V0, JUMP_DESCS, 7, 0);
                Addr(this).off(R_JUMP_STATE).sh(c, 0);
                break 'b_1012dc;
            }
            'b_1012d8: {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx(sp(c).wrapping_add(0x24));
                cruise::SELECT_TARGET_RIDER.call(c, 0x0010_1290)?;
                let selected = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(sp(c)).off(0x24).lw(c);
                if !selected {
                    c.cycles += 8;
                    c.g[V0][0] = sx(JUMP_DESCS);
                    jump_desc_copy(c, A3, JUMP_DESCS, 0xf, 0x8);
                    c.g[V1][0] = 1;
                    break 'b_1012d8;
                }
                c.cycles += 2;
                let none = c.g[V0][0] == 0;
                c.g[V0][0] = 0x44_0000;
                if none {
                    break 'b_1012dc;
                }
                c.cycles += 6;
                c.g[V1][0] = 3;
                c.g[V0][0] = sx(JUMP_DESCS);
                jump_desc_copy(c, A3, JUMP_DESCS, 0x1f, 0x18);
            }
            c.cycles += 1;
            let state = c.g[V1][0];
            Addr(this).off(R_JUMP_STATE).sh(c, state);
        }
        c.cycles += 13;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 24, 0xb0);
        frame.restore_fpr(c, 23, 0xa8);
        frame.restore_fpr(c, 22, 0xa0);
        frame.restore_fpr(c, 21, 0x98);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, ptr, FAKE};
    use crate::difftest::{check, elf_image, fill, put_words, scramble, Rng, DATA, GAME_GP};

    // ------------------------------------------------ PS2ExecMan_InvokeCallback

    /// `PS2ExecMan_InvokeCallback`: a callback with or without a function,
    /// `notify` zero, nonzero in the low word, or nonzero only in the upper
    /// word (64-bit compare); both calls go to fakes (some unwind).
    #[test]
    fn ps2_exec_man_invoke_callback_matches() {
        covered(
            ee::s10::f_003174a8__generated,
            ps2_exec_man_invoke_callback,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_74a8, 2);
                c.pc = 0x0031_74a8;
                let cb = ptr(&mut r);
                let owner = cb + 0x100;
                c.bus.write32(cb, if seed % 4 == 0 { 0 } else { FAKE });
                c.bus.write32(cb + 4, r.next_u32());
                c.bus.write32(cb + 8, owner);
                c.bus
                    .write16(owner + 0x70, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(owner + 0x74, FAKE + 0x10);
                c.g[A0][0] = sx(cb);
                c.g[A1][0] = match seed % 3 {
                    0 => 0,
                    1 => r.next_u32() as u64 | 1,
                    _ => (r.next_u32() as u64 | 1) << 32,
                };
            },
        );
    }

    // ------------------------------------------------ RiderAnim_UpdateSeqType00

    /// `RiderAnim_UpdateSeqType00`: the real track advance, weight fade and
    /// list removal on a random sequence (ended / faded / still running).
    #[test]
    fn rider_anim_seq_type00_update_matches() {
        check(
            ee::s00::f_001048c0__generated,
            rider_anim_seq_type00_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_48c0, 0);
                elf_image(c);
                c.pc = 0x0010_48c0;
                let seq = ptr(&mut r);
                let list = ptr(&mut r);
                c.g[A0][0] = sx(ptr(&mut r));
                c.g[A1][0] = sx(list);
                c.g[A2][0] = sx(seq);
                for off in [0x8, 0xc, 0x10, 0x90, 0x94, 0x98, 0x9c, 0xa4, 0xa8] {
                    let v = match r.next_u32() % 4 {
                        0 => 0,
                        1 => r.edge_f32(),
                        _ => r.f32(3.0),
                    };
                    c.bus.write32(seq + off, v);
                }
                c.bus.write32(seq + 0x1c, r.next_u32() & 1);
                c.bus.write32(seq + 0xa0, r.next_u32() & 1);
                let mut at = list + 4;
                for _ in 0..seed % 3 {
                    let other = ptr(&mut r) + 0x8;
                    c.bus.write32(at, other);
                    at = other + 200;
                }
                c.f[12] = if seed % 4 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(0.05)
                };
            },
        );
    }

    // --------------------------------------------- RiderSpinSub_GetFlipProgress

    /// The BE tables `BECharInt_GetAttrib4Factor` reads (as in `ai`'s
    /// `be_tables`): attribute and character-max noise, profile ids, the
    /// player slots (human or CPU), rider records and the BE singletons.
    fn be_attrib_tables(c: &mut Cpu, r: &mut Rng) {
        const ATTRIBS: u32 = 0x0053_5538;
        const CHARACTER_MAX: u32 = 0x0053_08d8;
        const PLAYERS: u32 = 0x0053_4fe0;
        fill(c, r, ATTRIBS - 0x400, 0x400);
        fill(c, r, CHARACTER_MAX - 0x800, 0x400);
        for p in 0..8 {
            c.bus.write32(0x0053_05b0 + 4 * p, r.next_u32() % 8);
        }
        fill(c, r, PLAYERS, 8 * 0x1c / 4);
        for s in 0..8 {
            let pad = if r.next_u32() % 2 == 0 {
                r.next_u32() | 0x8000_0000
            } else {
                r.next_u32() % 4
            };
            c.bus.write32(PLAYERS + s * 0x1c + 0xc, pad);
        }
        fill(c, r, 0x0053_5b20, 16 * 0x1c / 4);
        for s in 0..16 {
            let rec = 0x0053_5b20 + s * 0x1c;
            if r.next_u32() % 2 == 0 {
                c.bus.write32(rec + 0xc, u32::MAX);
            }
            c.bus.write8(rec + 0x11, (r.next_u32() % 10) as u8);
        }
        for ch in 0..10 {
            for n in 0..7 {
                if r.next_u32() % 6 == 0 {
                    c.bus.write8(CHARACTER_MAX + ch * 15 + 8 + n, 0);
                }
            }
        }
        c.bus.write32(0x0053_5c08, r.next_u32() % 40);
        fill(c, r, 0x0043_d950, 40 * 0x64 / 4);
        for t in 0..40 {
            c.bus
                .write32(0x0043_d950 + t * 0x64 + 0x54, r.next_u32() % 4);
        }
        // NEW_RACE_INT, NEW_PLAYER_INT, AGGRESSION_INT, CHAR_INT exist.
        for g in [0x1f3c, 0x1f30, 0x1ea4, 0x1f2c] {
            c.bus.write32(GAME_GP - g, DATA + 0xe_0000 + g * 0x10);
        }
    }

    /// `RiderSpinSub_GetFlipProgress`: flip angles below and above pi,
    /// edge-case rates, boost positive / zero / negative; the BELibrary
    /// singleton exists and its interface lookup is a fake (some unwind);
    /// the real attribute-4 factor runs on random BE tables.
    #[test]
    fn rider_spin_sub_get_flip_progress_matches() {
        use crate::difftest::fakes::vtable;
        covered(
            ee::s01::f_00135cb0__generated,
            rider_spin_sub_get_flip_progress,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0013_5cb0, 1);
                elf_image(c);
                c.pc = 0x0013_5cb0;
                // The BELibrary singleton; interface slot 0 is an object
                // whose vtable slot +8 is a fake.
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x1000);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(0x0053_08a8, DATA + 0x1100);
                c.bus.write32(DATA + 0x1104, vt);
                be_attrib_tables(c, &mut r);
                let this = DATA + 0x30_0000;
                let rider = DATA + 0x30_1000;
                let done = DATA + 0x30_2000;
                let total = DATA + 0x30_2010;
                let pi = std::f32::consts::PI;
                let a = r.f32(4.0);
                let b = match seed % 3 {
                    0 => r.f32(1.0),
                    1 => (f32::from_bits(a) + if seed % 2 == 0 { 4.0 } else { -4.0 }).to_bits(),
                    _ => r.edge_f32(),
                };
                c.bus.write32(this + 0x14, a);
                c.bus.write32(this + 0x1c, b);
                let rate = if seed % 7 == 0 {
                    r.edge_f32()
                } else {
                    (pi * 2.0).to_bits()
                };
                c.bus.write32(this + 0x38, rate);
                c.bus.write32(this + 0x58, rider);
                let boost = match seed % 4 {
                    0 => 0,
                    1 => 1.0f32.to_bits(),
                    2 => (-1.0f32).to_bits(),
                    _ => r.edge_f32(),
                };
                c.bus.write32(rider + 0x2ec, boost);
                c.bus.write32(rider + 0x86c, r.next_u32() % 8);
                c.bus.write32(
                    rider + 0xb34,
                    match r.next_u32() % 3 {
                        0 => 0,
                        1 => r.next_u32() % 40,
                        _ => r.next_u32(),
                    },
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(done);
                c.g[A2][0] = sx(total);
            },
        );
    }

    // ---------------------------------------------- GameCamera_UpdateStreamView

    /// `GameCamera_UpdateStreamView`: race mode 1 or not, view distances
    /// negative / small / mid / huge / edge (both clamps, both clearance
    /// tests, and the no-clearance result of a non-positive radius); the
    /// real streamer view setter and clearance query run on a world cache
    /// with no streamer nodes.
    #[test]
    fn game_camera_update_stream_view_matches() {
        check(
            ee::s01::f_0015ec98__generated,
            game_camera_update_stream_view,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_ec98, 0);
                elf_image(c);
                c.pc = 0x0015_ec98;
                // App -> game module -> race state (mode at +0x10).
                let app = DATA + 0x1000;
                let module = DATA + 0x1100;
                let race = DATA + 0x1200;
                c.bus.write32(GAME_GP - 0x848, app);
                c.bus.write32(app + 0x84, module);
                c.bus.write32(module + 0x84, race);
                c.bus.write32(
                    race + 0x10,
                    if seed % 4 == 0 { 1 } else { r.next_u32() % 3 },
                );
                // The world cache: streamer at +0x10 with no nodes
                // (`streamer+4 -> {+8: 0}`); the frame clock at gp+0x2a74.
                let cache_ptr = DATA + 0x2000;
                let cache = DATA + 0x3000;
                let nodes = DATA + 0x2100;
                let clock = DATA + 0x2200;
                c.bus.write32(GAME_GP + 0x16c8, cache_ptr);
                c.bus.write32(cache_ptr, cache);
                c.bus.write32(cache + 0x10 + 4, nodes);
                c.bus.write32(nodes + 8, 0);
                c.bus.write32(GAME_GP + 0x2a74, clock);
                c.bus.write32(clock + 0x18, r.next_u32());
                let cam = DATA + 0x8000;
                let dist: u32 = match seed % 6 {
                    0 => (-((r.next_u32() % 500) as f32)).to_bits(),
                    1 => r.f32(2.0),
                    2 => r.f32(12000.0),
                    3 => r.f32(80000.0),
                    4 => r.edge_f32(),
                    _ => ((r.next_u32() % 40000) as f32).to_bits(),
                };
                c.bus.write32(cam + 0x8, dist);
                for k in 0..4 {
                    c.bus.write32(cam + 0x20 + 4 * k, r.f32(5000.0));
                }
                c.bus.write32(cam + 0xb0, r.next_u32() % 4);
                c.bus.write32(cam + 0xb4, r.next_u32());
                c.g[A0][0] = sx(cam);
            },
        );
    }

    // -------------------------------------------- MultiParticle_EmitAtInstances

    /// `MultiParticle_EmitAtInstances`: 0..4 handles, each with an empty
    /// table entry, an empty slot (index 0) or a live instance; the real
    /// `Emitter_RenderAt` runs on a descriptor with nothing pending (its two
    /// graphics-manager hooks are fakes, some unwind).
    #[test]
    fn multi_particle_emit_at_instances_matches() {
        use crate::difftest::put_vec;
        check(
            ee::s11::f_00358260__generated,
            multi_particle_emit_at_instances,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_8260, 8);
                c.pc = 0x0035_8260;
                let this = DATA + 0x1000;
                let refs = DATA + 0x1100;
                let desc = DATA + 0x2000;
                let world = DATA + 0x3000;
                let tables = DATA + 0x3100;
                let table = DATA + 0x3200;
                fill(c, &mut r, desc, 0x80);
                c.bus.write32(desc + 0xc, 0);
                c.bus.write32(GAME_GP + 0x16c8, world);
                c.bus.write32(world, tables);
                c.bus.write32(tables + 8, table);
                let count = seed % 5;
                c.bus.write32(
                    this + 4,
                    if seed % 11 == 0 {
                        (-(seed as i32)) as u32
                    } else {
                        count
                    },
                );
                c.bus.write32(this + 0xc, refs);
                c.bus.write32(this + 0x10, desc);
                for off in [0x20, 0x30, 0x40] {
                    put_vec(
                        c,
                        this + off,
                        [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.f32(1.0)],
                    );
                }
                for i in 0..count {
                    let index = (r.next_u32() % 16) as u32;
                    let slot = r.next_u32() % 32;
                    c.bus.write32(refs + 4 * i, slot << 8 | index);
                    let entry = DATA + 0x4000 + 0x400 * i;
                    let slots = entry + 0x100;
                    let kind = r.next_u32() % 4;
                    c.bus
                        .write32(table + 4 * index, if kind == 0 { 0 } else { entry });
                    c.bus.write32(entry + 0x1c, slots);
                    let inst = DATA + 0x8000 + 0x100 * i;
                    let word = if kind == 1 {
                        r.next_u32() & 0xff
                    } else {
                        (inst >> 2) << 8 | (r.next_u32() & 0xff)
                    };
                    c.bus.write32(slots + 4 * slot, word);
                    put_vec(
                        c,
                        inst + 0x40,
                        [r.f32(500.0), r.f32(500.0), r.f32(500.0), r.f32(1.0)],
                    );
                }
            },
        );
    }

    // ---------------------------------------------------------------------- logf

    /// `logf` under every `_LIB_VERSION` (-1 `_IEEE_`, 0 `_SVID_`, 1
    /// `_XOPEN_`, 2 `_POSIX_`) with positive, NaN, +-0, negative and
    /// infinite inputs; the real `__ieee754_logf`, soft-float helpers,
    /// `matherr` and `__getreent` run on the ELF image.
    #[test]
    fn logf_matches() {
        check(ee::s15::f_0040da10__generated, logf, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0040_da10, 0);
            elf_image(c);
            c.pc = 0x0040_da10;
            let version = match seed % 5 {
                0 => u32::MAX,
                1 => 0,
                2 => 1,
                _ => 2,
            };
            c.bus.write32(0x004a_0000 - 0x6978, version);
            c.f[12] = match seed / 5 % 7 {
                0 => r.f32(1000.0) & 0x7fff_ffff,
                1 => 0x7fc0_0000 | (r.next_u32() & 0x803f_ffff),
                2 => (r.next_u32() & 1) << 31,
                3 => r.f32(1000.0) | 0x8000_0000,
                4 => 0xff80_0000,
                5 => 0x7f80_0000,
                _ => r.edge_f32(),
            };
        });
    }

    // -------------------------------------------------------- LensFX_DrawFlares

    /// `LensFX_DrawFlares`: view 0 or 1, sun on or off, 0..5 flares (or a
    /// negative count), each off or on; the real sun draw returns at once
    /// (its "on" halfword is 0), the real flare-quad draw runs on random
    /// particles of every type (its sprite draw is a fake), and the push /
    /// pop render-state methods are fakes (some calls unwind).
    #[test]
    fn lens_fx_draw_flares_matches() {
        covered(ee::s09::f_002e3478__generated, lens_fx_draw_flares, 300, &|c, seed| {
            use crate::difftest::fakes::vtable;
            let mut r = machine(c, seed, 0x002e_3478, 7);
            c.pc = 0x002e_3478;
            let fx = DATA + 0x30_0000;
            let view = seed % 2;
            c.bus.write32(
                GAME_GP + 0xa28,
                if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
            );
            let vt = vtable(c, &mut r, &[0x10, 0x18]);
            c.bus.write32(fx + 0x649c, vt);
            let count = if seed % 13 == 0 {
                r.next_u32() | 0x8000_0000
            } else {
                seed / 2 % 6
            };
            c.bus.write32(fx + 0x60 + 4 * view, count);
            // The renderer the flare quads draw with (as in the
            // `LensFX_DrawFlareQuad` test).
            let gfx = DATA + 0x20_0000;
            let top = DATA + 0x20_3000;
            let gvt = DATA + 0x20_4000;
            c.bus.write32(GAME_GP + 0x2a90, gfx);
            c.bus.write32(gfx + 0xe84, top);
            fill(c, &mut r, top, 8);
            c.bus.write32(gfx + 0x1024, r.next_u32());
            c.bus.write32(gfx + 0x1028, r.next_u32());
            c.bus.write32(gfx + 0x10d8, gvt);
            c.bus
                .write16(gvt + 0x278, (r.next_u32() as i16 & !3) as u16);
            c.bus.write32(gvt + 0x27c, FAKE + 0x20);
            for k in 0..6 {
                let flare = fx + 0x68 + view * 0x3000 + k * 0x30;
                let part = DATA + 0x21_0000 + k * 0x100;
                let on = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    1 + r.next_u32() as u16 % 3
                };
                c.bus.write16(flare + 0x2c, on);
                c.bus.write16(flare + 0x2e, (r.next_u32() % 2) as u16);
                c.bus.write32(flare + 0x20, part);
                c.bus.write32(flare + 0x28, r.f32(6.3));
                for j in 0..3 {
                    let v = if r.next_u32() % 7 == 3 {
                        0
                    } else {
                        r.f32(50.0)
                    };
                    c.bus.write32(part + 0x10 + 4 * j, v);
                    c.bus.write32(part + 0x1c + 4 * j, r.f32(1000.0));
                }
                for j in 0..4 {
                    let a = match r.next_u32() % 4 {
                        0 => 0,
                        1 => r.f32(1.0) | 0x8000_0000,
                        _ => r.f32(1.0) & !0x8000_0000,
                    };
                    c.bus.write32(part + 0x40 + 4 * j, a);
                }
                let kind =
                    [0x10, 0x20, 0x40, 0x00, 0x30, 0x50, 0x60, 0x70][(r.next_u32() % 8) as usize];
                c.bus.write32(part + 0xc, (r.next_u32() & !0x70) | kind);
            }
            c.g[A0][0] = sx(fx);
            c.g[A1][0] = view as u64;
        });
    }

    // ---------------------------------------------- ScriptAudioObj_UpdateSounds

    /// `ScriptAudioObj_UpdateSounds`: four triggers each without a sound,
    /// with a stop request, or polling an id the bank monitor may or may
    /// not still play (64 busy / free instances that are already fading, so
    /// the real stop resets them without SND calls); a pending speech cue
    /// with speech instance 0 missing or ready (the real list skip runs on
    /// a list in state 3 that returns at once).
    #[test]
    fn script_audio_obj_update_sounds_matches() {
        check(
            ee::s07::f_00280640__generated,
            script_audio_obj_update_sounds,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0028_0640, 0);
                c.pc = 0x0028_0640;
                let this = DATA + 0x1_0000;
                let audio = DATA + 0x1000;
                let root = DATA + 0x2000;
                let snd = DATA + 0x3000;
                let insts = DATA + 0x20_0000;
                c.bus.write32(GAME_GP + 0x410, audio);
                c.bus.write32(audio + 0x118, root);
                c.bus.write32(root, snd);
                c.bus.write32(snd + 0x1d8 + 0x8ec, insts);
                for i in 0..64 {
                    let inst = insts + i * 0xc0;
                    c.bus.write32(inst, r.next_u32() % 3);
                    c.bus.write32(inst + 4, r.next_u32() % 6);
                    c.bus.write32(inst + 0x54, 1);
                    c.bus.write32(inst + 0x9c, 0);
                }
                for k in 0..4 {
                    let t = this + 0xbc + 0xc * k;
                    let id = match r.next_u32() % 4 {
                        0 => r.next_u32() | 0x8000_0000,
                        _ => r.next_u32() % 6,
                    };
                    c.bus.write32(t, id);
                    c.bus.write32(
                        t + 4,
                        if r.next_u32() % 3 == 0 {
                            r.next_u32() | 1
                        } else {
                            0
                        },
                    );
                    c.bus.write32(t + 8, r.next_u32());
                }
                c.bus.write32(
                    this + 0xf4,
                    if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
                );
                // Speech: instance 0 missing or with a stream slot whose
                // stream is gone (ready).
                let speech = audio + 0x5560;
                let spch_insts = DATA + 0x4000;
                let spch = DATA + 0x4100;
                let arrays = DATA + 0x4200;
                let array = DATA + 0x4300;
                let table = DATA + 0x4400;
                c.bus.write32(speech + 4, spch_insts);
                c.bus
                    .write32(spch_insts, if seed % 2 == 0 { 0 } else { spch });
                c.bus.write32(spch + 4, r.next_u32() % 4);
                c.bus.write32(GAME_GP + 0x6c4, arrays);
                c.bus.write32(arrays, array);
                c.bus.write32(array + 8, table);
                // The script engine and its list (state 3, nothing queued).
                let engine = DATA + 0x5000;
                let lists = DATA + 0x6000;
                let index = r.next_u32() % 4;
                let list = lists + 0xcc * index;
                c.bus.write32(GAME_GP - 0x84c, engine);
                c.bus.write32(engine + 0x550, index);
                c.bus.write32(engine + 0x54c, lists);
                c.bus.write32(list + 0xa4, 3);
                c.bus.write32(list + 0xa8, 1);
                c.bus.write32(list + 0xbc, 0);
                c.bus.write32(list + 0xc0, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ------------------------------------------------ RailModifier_UpdateBounds

    /// `RailModifier_UpdateBounds`: "has bounds" is a fake (nonzero) or the
    /// real leaf at 0x1406c0 (returns 0); "get bounds" is a fake whose
    /// forced result points at a random box. Some calls unwind.
    #[test]
    fn rail_modifier_update_bounds_matches() {
        use crate::difftest::fakes::LOG_RESULTS;
        use crate::difftest::put_vec;
        check(
            ee::s11::f_0035c040__generated,
            rail_modifier_update_bounds,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_c040, 2);
                c.pc = 0x0035_c040;
                let this = DATA + 0x1000;
                let owner = DATA + 0x1100;
                let object = DATA + 0x1200;
                let vt = DATA + 0x1400;
                let bounds = DATA + 0x2000;
                c.bus.write32(this + 0x40, owner);
                c.bus.write32(owner + 0xc, object);
                c.bus.write32(object + 0xc, vt);
                for slot in [0x160, 0x168] {
                    c.bus
                        .write16(vt + slot, (r.next_u32() as i16 & 0x7c) as u16);
                }
                c.bus
                    .write32(vt + 0x164, if seed % 4 == 0 { 0x0014_06c0 } else { FAKE });
                c.bus.write32(vt + 0x16c, FAKE + 0x10);
                c.bus.write32(LOG_RESULTS + 4, bounds);
                for k in 0..2 {
                    put_vec(
                        c,
                        bounds + 0x10 * k,
                        [r.f32(5000.0), r.f32(5000.0), r.f32(5000.0), r.next_u32()],
                    );
                    put_vec(
                        c,
                        this + 0x10 + 0x10 * k,
                        [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                    );
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    // -------------------------------------- ComputerActorRider_CruiseChaseRider

    /// `ComputerActorRider_CruiseChaseRider` on the cruise fixture of
    /// `ComputerActorRider_CruiseFollowPath` (same steering, crouch, boost,
    /// jump-marker and target-rider callees, all real): the chased rider is
    /// AI slot 1..3, whose path interface returns a random velocity (fake)
    /// and is ready or not (fake / the real zero leaf); headings behind and
    /// ahead, record distances zero / positive / negative / edge, the same
    /// or another AI path than the model (real path re-selection).
    #[test]
    fn computer_actor_rider_cruise_chase_rider_matches() {
        use crate::difftest::fakes::LOG_RESULTS;
        use crate::difftest::put_vec;
        use crate::game::ai::computer::tests::{
            cruise_setup, MODEL, PMF, RIDER_2, RIDER_ENTRY, WORLD,
        };
        check(
            ee::s00::f_00100f88__generated,
            computer_actor_rider_cruise_chase_rider,
            240,
            &|c, seed| {
                cruise_setup(c, seed);
                let mut r = Rng(0x0010_0f88 ^ (seed as u64) << 20);
                c.pc = 0x0010_0f88;
                // The chased rider: AI slot 1..3 (`ai + 0x28 + 4*slot`).
                let slot = 1 + seed % 3;
                let ai = WORLD + 0x200;
                c.bus.write32(RIDER_2 + 0xe70, slot);
                for i in 1..4u32 {
                    c.bus
                        .write32(ai + 0x28 + 4 * i, RIDER_ENTRY + 0x100 * (i - 1));
                }
                let entry = RIDER_ENTRY + 0x100 * (slot - 1);
                c.bus.write32(entry + 0x6c0, PMF);
                c.bus.write32(
                    entry + 0xab8,
                    if seed % 5 == 0 { 0 } else { r.next_u32() | 1 },
                );
                // Its path interface: velocity (first fake call; the forced
                // result points at a random vector) and "ready".
                let vel = DATA + 0x9_0000;
                c.bus.write16(PMF + 0x10, 0);
                c.bus.write32(PMF + 0x14, FAKE + 0x30);
                c.bus.write16(PMF + 0x48, 0);
                c.bus.write32(
                    PMF + 0x4c,
                    if seed % 4 == 0 {
                        0x0014_06c0
                    } else {
                        FAKE + 0x40
                    },
                );
                c.bus.write32(LOG_RESULTS, vel);
                put_vec(c, vel, [r.f32(150.0), r.f32(150.0), r.f32(150.0), 0]);
                // Headings and the slot record (distance, heading).
                c.bus.write32(MODEL + 0x4cc, r.f32(3.2));
                let rec = MODEL + 0x24 * slot;
                c.bus.write32(
                    rec + 0x8,
                    match seed % 5 {
                        0 => 0,
                        1 => r.f32(400.0) & 0x7fff_ffff,
                        2 => r.f32(400.0) | 0x8000_0000,
                        3 => r.f32(40.0),
                        _ => r.edge_f32(),
                    },
                );
                c.bus.write32(rec + 0xc, r.f32(6.4));
            },
        );
    }
}
