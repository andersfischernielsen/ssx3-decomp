//! Staging wave 207 (oc258): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::abi;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001e_1458, message_center_update, "MessageCenter_Update"),
    (0x002e_e600, world_painter_get_weather48, "WorldPainter_GetWeather48"),
    (0x0035_7bf8, multi_particle_man_update, "MultiParticleMan_Update"),
    (0x0010_4358, rider_anim_update_seq_type07, "RiderAnim_UpdateSeqType07"),
    (0x0011_1ac0, rider_base_save_state, "RiderBase_SaveState"),
    (0x002e_edf8, world_painter_get_light_glow10, "WorldPainter_GetLightGlow10"),
    (0x002e_e6a8, world_painter_get_weather40, "WorldPainter_GetWeather40"),
    (0x0031_3ba8, anim_sequence_remove_event_trigger, "AnimSequence_RemoveEventTrigger"),
    (0x002e_30d0, lens_fx_render, "LensFX_Render"),
    (0x0012_cd20, rider_wipeout_sub_update_phase0, "RiderWipeoutSub_UpdatePhase0"),
];

// ---------------------------------------------------------------------------
// MessageCenter_Update (0x1e1458)

/// `MessageCenter_InitGroupTable` (0x1e12f0).
const MESSAGE_CENTER_INIT_GROUP_TABLE: Callee = Callee { addr: 0x001e_12f0, f: ee::s04::f_001e12f0 };
/// `MessageCenter_TriggerGroups(type)` (0x1e14c0).
const MESSAGE_CENTER_TRIGGER_GROUPS: Callee = Callee { addr: 0x001e_14c0, f: ee::s04::f_001e14c0 };

/// gp-relative message-center globals.
const MC_SUSPENDED: u32 = 0x1030; // gp - 0x1030: non-zero = updates off
const MC_TABLE_READY: u32 = 0x1028; // gp - 0x1028: group table initialised
const MC_LAST_TRIGGER: u32 = 0x1034; // gp - 0x1034: clock at the last trigger
const MC_CLOCK_OWNER: u32 = 0x2a74; // gp + 0x2a74: object whose +0x1c is the clock
const MC_CLOCK: u32 = 0x1c;
/// Ticks between two periodic trigger passes (600 = 10 s at 60 Hz).
const MC_TRIGGER_PERIOD: i64 = 600;

/// `void MessageCenter_Update(void)` (0x1e1458): unless the message center
/// is suspended, build the group table once, then every 600 clock ticks
/// fire the periodic (type 1) triggers and remember the clock.
pub fn message_center_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_sub(MC_SUSPENDED)).lw(c);
        frame.save(c, S0, 0x10);
        let suspended = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if suspended {
            c.cycles += 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(gp.wrapping_sub(MC_TABLE_READY)).lw(c);
            c.g[V1][0] = Addr(gp.wrapping_add(MC_CLOCK_OWNER)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[V0][0] = 1;
                Addr(gp.wrapping_sub(MC_TABLE_READY)).sw(c, 1);
                MESSAGE_CENTER_INIT_GROUP_TABLE.call(c, 0x001e_1484)?;
                let gp = r32(c, GP);
                c.g[V1][0] = Addr(gp.wrapping_add(MC_CLOCK_OWNER)).lw(c);
            }
            c.cycles += 6;
            let gp = r32(c, GP);
            c.g[V0][0] = Addr(gp.wrapping_sub(MC_LAST_TRIGGER)).lw(c);
            c.g[S0][0] = Addr(r32(c, V1).wrapping_add(MC_CLOCK)).lw(c);
            c.g[V0][0] = sx(r32(c, S0).wrapping_sub(r32(c, V0)));
            c.g[V0][0] = ((c.g[V0][0] as i64) < MC_TRIGGER_PERIOD + 1) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[A0][0] = 1;
                MESSAGE_CENTER_TRIGGER_GROUPS.call(c, 0x001e_14a8)?;
                let gp = r32(c, GP);
                let now = c.g[S0][0];
                Addr(gp.wrapping_sub(MC_LAST_TRIGGER)).sw(c, now);
                c.cycles += 1;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldPainter_GetWeather48 (0x2ee600)

/// The lens-effect slot array `D_004FA370` (0xf0 bytes per slot).
const LENS_FX_SLOTS: u32 = 0x004f_a370;
const LENS_FX_SLOT_SIZE: u64 = 0xf0;
/// `gp + 0xa70`: non-zero while the weather lens effects are off.
const LENS_FX_WEATHER_OFF: u32 = 0xa70;

/// `float WorldPainter_GetWeather48(int i)` (0x2ee600): 0.0 while the weather
/// effects are off (`gp + 0xa70`), else the value the lens-effect slot `i`'s
/// object returns through its virtual accessor (vtable slot 0x198/0x19c, a
/// pointer to the float).
pub fn world_painter_get_weather48(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(LENS_FX_WEATHER_OFF)).lw(c);
        let off = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if off {
            c.cycles += 3;
            c.mtc1(0, 0); // f0 = 0.0
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 14;
            c.g[V1][0] = LENS_FX_SLOT_SIZE;
            c.g[V0][0] = 0x0050_0000;
            c.mult(0, V1, A0, V1); // v1 = i * 0xf0
            c.g[V0][0] = sx(LENS_FX_SLOTS);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(LENS_FX_SLOTS));
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x20)).lw(c); // s->p20
            c.g[A1][0] = Addr(r32(c, A0)).lw(c); // obj = *s->p20
            c.g[V1][0] = Addr(r32(c, A1).wrapping_add(4)).lw(c); // obj->vt
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x198)).lh(c); // delta
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x19c)).lw(c); // fn
            let target = r32(c, V0);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x002e_e64c)?;
            let p = r32(c, V0);
            c.lwc1(0, p);
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MultiParticleMan_Update (0x357bf8)

/// `MultiParticle_Update(p)` (0x358120).
const MULTI_PARTICLE_UPDATE: Callee = Callee { addr: 0x0035_8120, f: ee::s11::f_00358120 };
/// Particle systems a `MultiParticleMan` holds.
const MULTI_PARTICLE_MAN_SLOTS: u64 = 4;

/// `void MultiParticleMan_Update(MultiParticle** slots)` (0x357bf8): update
/// each of the four multi-particle systems that is present.
pub fn multi_particle_man_update(c: &mut Cpu) {
    const HEAD: u32 = 0x0035_7c18;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = MULTI_PARTICLE_MAN_SLOTS - 1; // slots left after this one
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        loop {
            c.cycles += 4;
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            c.g[A0][0] = c.g[V0][0];
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                MULTI_PARTICLE_UPDATE.call(c, 0x0035_7c30)?;
            }
            c.cycles += 2;
            if (c.g[S1][0] as i64) < 0 {
                break;
            }
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderAnim_UpdateSeqType07 (0x104358)

/// `AnimSequence_SetTrackTime(seq, track, time)` (0x313cf0).
const SEQ07_SET_TRACK_TIME: u32 = 0x0031_3cf0;
/// `AnimSequence_UpdateWeightFade(seq, step)` (0x313800): nonzero when faded out.
const SEQ07_UPDATE_WEIGHT_FADE: u32 = 0x0031_3800;
/// `AnimSeqList_Remove(list, seq)` (0x3145f8).
const SEQ07_LIST_REMOVE: u32 = 0x0031_45f8;
/// `gp - 0x7f58`: 1/60 (ticks to seconds).
const SEQ07_TICK_GP: u32 = 0x7f58;
/// `RiderAnim + 0x60`: the rider actor; its two speed-like floats.
const SEQ07_ACTOR: u32 = 0x60;
const SEQ07_ACTOR_A: u32 = 0x2b0;
const SEQ07_ACTOR_B: u32 = 0x2a4;
/// `AnimSequence + 0x10`: the track-time scale.
const SEQ07_SEQ_SCALE: u32 = 0x10;

/// `int RiderAnim_UpdateSeqType07(RiderAnim* self, SeqList* list,
/// AnimSequence* seq, float dt)` (0x104358): a scrubbed sequence. Its track 0
/// time is `max(|actor+0x2b0|, |actor+0x2a4|) * seq->scale`; then the weight
/// fade advances by `dt / 60`, and when it has finished the sequence is
/// removed from the list. Returns whether it was removed.
pub fn rider_anim_update_seq_type07(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.f[20] = c.f[12]; // dt
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(SEQ07_ACTOR)).lw(c);
        let actor = r32(c, V0);
        let a = Addr(actor.wrapping_add(SEQ07_ACTOR_A)).read_fpu(c).abs();
        let b = Addr(actor.wrapping_add(SEQ07_ACTOR_B)).read_fpu(c).abs();
        abi::set_fpr(c, 0, a);
        abi::set_fpr(c, 1, b);
        let a_lt_b = a.lt(b);
        abi::set_fcc(c, a_lt_b);
        let m = if a_lt_b {
            c.cycles += 1;
            b
        } else {
            a
        };
        abi::set_fpr(c, 2, m);

        c.cycles += 11;
        let scale = Addr(r32(c, S0).wrapping_add(SEQ07_SEQ_SCALE)).read_fpu(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        abi::set_fpr(c, 12, m * scale);
        call_ptr(c, SEQ07_SET_TRACK_TIME, 0x0010_43b4)?;
        let tick = Addr(r32(c, GP).wrapping_sub(SEQ07_TICK_GP)).read_fpu(c);
        c.g[A0][0] = c.g[S0][0];
        let dt = FpuF32(c.f[20]);
        abi::set_fpr(c, 12, dt * tick);
        call_ptr(c, SEQ07_UPDATE_WEIGHT_FADE, 0x0010_43c4)?;
        let faded = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S1][0];
        if faded {
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, SEQ07_LIST_REMOVE, 0x0010_43dc)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderBase_SaveState (0x111ac0)

/// `RiderBase + 0x18`: the rider (physics/model side) whose parts are saved.
const RB_RIDER: u32 = 0x18;
/// Rider fields saved by pointer.
const RB_RIDER_ANIM: u32 = 0x784;
const RB_RIDER_MODEL: u32 = 0x780;
const RB_RIDER_AIR_PREDICTOR: u32 = 0x788;
const RB_RIDER_METRIX: u32 = 0x790;
const RB_RIDER_TARGET0: u32 = 0x864;
const RB_RIDER_TARGET1: u32 = 0x868;
const RB_RIDER_BLOCK0: u32 = 0xabc;
const RB_RIDER_BLOCK1: u32 = 0xac0;
const RB_RIDER_TRACK_PATH: u32 = 0xab4;
const RB_RIDER_AI_PATH: u32 = 0xab8;
/// `target + 0x150`: the id saved for a non-null target object.
const RB_TARGET_ID: u32 = 0x150;
/// `D_004D33A0`: the course path data.
const RB_PATH_DATA: u32 = 0x004d_33a0;

/// `RiderAnim_SaveState(anim, stream)` (0x104d68).
const RB_RIDER_ANIM_SAVE: Callee = Callee { addr: 0x0010_4d68, f: ee::s00::f_00104d68 };
/// `AirPredictor_SaveState` (0x113aa0).
const RB_AIR_PREDICTOR_SAVE: Callee = Callee { addr: 0x0011_3aa0, f: ee::s00::f_00113aa0 };
/// `RiderMetrix_SaveState` (0x1174b0).
const RB_RIDER_METRIX_SAVE: Callee = Callee { addr: 0x0011_74b0, f: ee::s00::f_001174b0 };
/// `RiderModel_SaveState` (0x310ea8).
const RB_RIDER_MODEL_SAVE: u32 = 0x0031_0ea8;
/// `PathData_GetTrackPathIndex(paths, path)` (0x26b508).
const RB_PATH_TRACK_INDEX: u32 = 0x0026_b508;
/// `PathData_GetAIPathIndex(paths, path)` (0x26b550).
const RB_PATH_AI_INDEX: u32 = 0x0026_b550;

/// How a rider sub-state's `SaveState(self + off, stream)` is reached.
#[derive(Clone, Copy)]
enum RbSub {
    /// `jal` through the generated function.
    Direct(Callee),
    /// The generator's cross-shard `call_indirect`.
    Table(u32),
}

/// The sub-states embedded in `RiderBase`, saved in order:
/// (offset, saver, return address).
const RB_SUBSTATES: [(u32, RbSub, u32); 16] = [
    (0x000, RbSub::Table(0x0013_f848), 0x0011_1c3c), // RiderGroundState
    (0x020, RbSub::Table(0x0013_acb0), 0x0011_1c48),
    (0x030, RbSub::Table(0x0013_8ad8), 0x0011_1c54),
    (0x0b0, RbSub::Table(0x0013_c5e0), 0x0011_1c60),
    (0x110, RbSub::Table(0x0013_9970), 0x0011_1c6c),
    (0x1d0, RbSub::Direct(Callee { addr: 0x0013_1598, f: ee::s00::f_00131598 }), 0x0011_1c78), // BoardPress
    (0x210, RbSub::Direct(Callee { addr: 0x0012_fbf0, f: ee::s00::f_0012fbf0 }), 0x0011_1c84), // NaturalAir
    (0x230, RbSub::Table(0x0013_61f8), 0x0011_1c90),
    (0x290, RbSub::Direct(Callee { addr: 0x0012_c558, f: ee::s00::f_0012c558 }), 0x0011_1c9c), // Gate
    (0x2b0, RbSub::Table(0x0013_2840), 0x0011_1ca8),
    (0x2c0, RbSub::Direct(Callee { addr: 0x0012_e708, f: ee::s00::f_0012e708 }), 0x0011_1cb4), // Wipeout
    (0x350, RbSub::Direct(Callee { addr: 0x0012_f5b0, f: ee::s00::f_0012f5b0 }), 0x0011_1cc0), // Reset
    (0x360, RbSub::Direct(Callee { addr: 0x0012_c9c0, f: ee::s00::f_0012c9c0 }), 0x0011_1ccc), // FinishLine
    (0x370, RbSub::Table(0x0013_30b8), 0x0011_1cd8),
    (0x390, RbSub::Table(0x0013_68e8), 0x0011_1ce4), // UberGrind
    (0xd20, RbSub::Table(0x002f_6a58), 0x0011_1cf0), // cheat code
];

/// `stream->Write(a1, a2)` (vtable slot 0x8/0xc of the stream in s1): the
/// vtable pointer is already in `vt_reg`, the method pointer goes to `fn_reg`.
fn rb_stream_write(c: &mut Cpu, vt_reg: usize, fn_reg: usize, ret_addr: u32) -> crate::call::Flow {
    let vt = r32(c, vt_reg);
    c.g[A0][0] = Addr(vt.wrapping_add(0x8)).lh(c);
    c.g[fn_reg][0] = Addr(vt.wrapping_add(0xc)).lw(c);
    let target = r32(c, fn_reg);
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ret_addr)
}

/// `rider = self->rider` (`lw s2+0x18`), into `reg`.
fn rb_rider(c: &mut Cpu, reg: usize) -> u32 {
    c.g[reg][0] = Addr(r32(c, S2).wrapping_add(RB_RIDER)).lw(c);
    r32(c, reg)
}

/// `void RiderBase_SaveState(RiderBase* self, Stream* s)` (0x111ac0): write
/// the rider's 0x6c0-byte block, the anim/model/air-predictor/metrix states,
/// both target ids (-1 for none) with their 16-byte records, the two words at
/// +0xde0/+0xde4, every embedded sub-state, the two 32-byte blocks and the
/// track/AI path indices.
pub fn rider_base_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x60);
        c.g[A2][0] = 0x6c0;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        c.g[S2][0] = c.g[A0][0];
        c.g[S0][0] |= 0xffff; // -1: "no target"
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        let rider = Addr(r32(c, S2).wrapping_add(RB_RIDER)).lw(c);
        c.g[A1][0] = rider;
        rb_stream_write(c, V1, V0, 0x0011_1b00)?;

        let rider = rb_rider(c, V1);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = Addr(rider.wrapping_add(RB_RIDER_ANIM)).lw(c);
        RB_RIDER_ANIM_SAVE.call(c, 0x0011_1b10)?;
        let rider = rb_rider(c, V0);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = Addr(rider.wrapping_add(RB_RIDER_MODEL)).lw(c);
        call_ptr(c, RB_RIDER_MODEL_SAVE, 0x0011_1b20)?;
        let rider = rb_rider(c, V0);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = Addr(rider.wrapping_add(RB_RIDER_AIR_PREDICTOR)).lw(c);
        RB_AIR_PREDICTOR_SAVE.call(c, 0x0011_1b30)?;
        let rider = rb_rider(c, V0);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = Addr(rider.wrapping_add(RB_RIDER_METRIX)).lw(c);
        RB_RIDER_METRIX_SAVE.call(c, 0x0011_1b40)?;

        // Target 0: its id (or -1) at sp+0x10, then the 16-byte record.
        rb_target_id(c, RB_RIDER_TARGET0, 0x10);
        c.cycles += 20;
        c.g[A1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = 4;
        rb_stream_write(c, V1, V0, 0x0011_1b7c)?;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = 0x10;
        let rider = rb_rider(c, V0);
        c.g[A1][0] = Addr(rider.wrapping_add(RB_RIDER_TARGET0)).lw(c);
        rb_stream_write(c, V1, A3, 0x0011_1b9c)?;

        // Target 1 at sp+0x14.
        rb_target_id(c, RB_RIDER_TARGET1, 0x14);
        c.cycles += 117;
        c.g[A1][0] = sx(sp(c).wrapping_add(0x14));
        c.g[A2][0] = 4;
        rb_stream_write(c, V1, V0, 0x0011_1bd8)?;
        c.g[A3][0] = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = 0x10;
        let rider = rb_rider(c, V0);
        c.g[A1][0] = Addr(rider.wrapping_add(RB_RIDER_TARGET1)).lw(c);
        rb_stream_write(c, A3, V1, 0x0011_1bf8)?;

        c.g[A3][0] = Addr(r32(c, S1)).lw(c);
        c.g[A1][0] = sx(r32(c, S2).wrapping_add(0xde0));
        c.g[A2][0] = 4;
        rb_stream_write(c, A3, V0, 0x0011_1c14)?;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = 4;
        c.g[A1][0] = sx(r32(c, S2).wrapping_add(0xde4));
        rb_stream_write(c, V1, V0, 0x0011_1c30)?;

        for (off, saver, ret_addr) in RB_SUBSTATES {
            c.g[A0][0] = if off == 0 {
                c.g[S2][0]
            } else {
                sx(r32(c, S2).wrapping_add(off))
            };
            c.g[A1][0] = c.g[S1][0];
            match saver {
                RbSub::Direct(f) => f.call(c, ret_addr)?,
                RbSub::Table(addr) => call_ptr(c, addr, ret_addr)?,
            }
        }

        c.g[A3][0] = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = 0x20;
        let rider = rb_rider(c, V0);
        c.g[A1][0] = Addr(rider.wrapping_add(RB_RIDER_BLOCK0)).lw(c);
        rb_stream_write(c, A3, V1, 0x0011_1d10)?;
        c.g[A3][0] = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = 0x20;
        let rider = rb_rider(c, V0);
        c.g[A1][0] = Addr(rider.wrapping_add(RB_RIDER_BLOCK1)).lw(c);
        rb_stream_write(c, A3, V1, 0x0011_1d30)?;

        // The path indices go to sp+0 / sp+4.
        let rider = rb_rider(c, V0);
        c.g[S0][0] = sx(RB_PATH_DATA);
        c.g[A1][0] = Addr(rider.wrapping_add(RB_RIDER_TRACK_PATH)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, RB_PATH_TRACK_INDEX, 0x0011_1d48)?;
        let track = c.g[V0][0];
        Addr(sp(c)).sw(c, track);
        c.g[A0][0] = c.g[S0][0];
        let rider = rb_rider(c, V0);
        c.g[A1][0] = Addr(rider.wrapping_add(RB_RIDER_AI_PATH)).lw(c);
        call_ptr(c, RB_PATH_AI_INDEX, 0x0011_1d5c)?;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A1][0] = c.g[SP][0];
        let ai = c.g[V0][0];
        Addr(sp(c).wrapping_add(4)).sw(c, ai);
        c.g[A2][0] = 8;
        rb_stream_write(c, V1, V0, 0x0011_1d7c)?;

        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `sp[slot] = -1 (s0); if (*rider->field) sp[slot] = (*rider->field)->id;`
/// then `v1 = stream->vtable`.
fn rb_target_id(c: &mut Cpu, field: u32, slot: u32) {
    let none = c.g[S0][0];
    Addr(sp(c).wrapping_add(slot)).sw(c, none);
    let rider = rb_rider(c, V0);
    c.g[V1][0] = Addr(rider.wrapping_add(field)).lw(c);
    c.g[V1][0] = Addr(r32(c, V1)).lw(c);
    if c.g[V1][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(RB_TARGET_ID)).lw(c);
        let id = c.g[V0][0];
        Addr(sp(c).wrapping_add(slot)).sw(c, id);
    }
    c.g[V1][0] = Addr(r32(c, S1)).lw(c);
}

// ---------------------------------------------------------------------------
// WorldPainter_GetLightGlow10 (0x2eedf8), WorldPainter_GetWeather40 (0x2ee6a8)

/// The shared body of the 18-cycle lens-slot accessors: `slot = &D_004FA370[i]`,
/// `obj = *slot->field`, return `*obj->vt[vt_off].fn(obj + delta)` in f0.
fn lens_slot_virtual_float(c: &mut Cpu, field: u32, vt_off: u32, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = LENS_FX_SLOT_SIZE;
        let frame = Frame::open(c, 0x10);
        c.mult(0, A0, A0, V1); // a0 = i * 0xf0
        c.g[V0][0] = 0x0050_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(LENS_FX_SLOTS);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(LENS_FX_SLOTS));
        c.g[V1][0] = Addr(r32(c, A0).wrapping_add(field)).lw(c);
        c.g[A2][0] = Addr(r32(c, V1)).lw(c); // obj
        c.g[A1][0] = Addr(r32(c, A2).wrapping_add(4)).lw(c); // obj->vt
        c.g[A0][0] = Addr(r32(c, A1).wrapping_add(vt_off)).lh(c); // delta
        c.g[V0][0] = Addr(r32(c, A1).wrapping_add(vt_off + 4)).lw(c); // fn
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, target, ret_addr)?;
        let p = r32(c, V0);
        c.lwc1(0, p);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetLightGlow10(int i)` (0x2eedf8): the light-glow
/// value of lens slot `i` (`*slot->p08`, vtable slot 0x70/0x74).
pub fn world_painter_get_light_glow10(c: &mut Cpu) {
    lens_slot_virtual_float(c, 0x8, 0x70, 0x002e_ee30);
}

/// `float WorldPainter_GetWeather40(int i)` (0x2ee6a8): the weather value of
/// lens slot `i` (`*slot->p20`, vtable slot 0x190/0x194).
pub fn world_painter_get_weather40(c: &mut Cpu) {
    lens_slot_virtual_float(c, 0x20, 0x190, 0x002e_e6e0);
}

// ---------------------------------------------------------------------------
// AnimSequence_RemoveEventTrigger (0x313ba8)

/// `operator delete` (`__builtin_delete`, 0x317e50).
const ANIM_TRIGGER_DELETE: Callee = Callee { addr: 0x0031_7e50, f: ee::s10::f_00317e50 };
/// `AnimSequence + 0xac`: head of the event-trigger list.
const ANIM_SEQ_TRIGGERS: u32 = 0xac;
/// Trigger node: `{ int key; int active; ...; Trigger* next (+0xc) }`.
const ANIM_TRIGGER_ACTIVE: u32 = 0x4;
const ANIM_TRIGGER_NEXT: u32 = 0xc;

/// `void AnimSequence_RemoveEventTrigger(AnimSequence* seq, int key)`
/// (0x313ba8): unlink and delete the first active trigger with `key`.
pub fn anim_sequence_remove_event_trigger(c: &mut Cpu) {
    const HEAD: u32 = 0x0031_3bc0;
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(ANIM_SEQ_TRIGGERS)).lw(c);
        c.g[A2][0] = sx(r32(c, A0).wrapping_add(ANIM_SEQ_TRIGGERS)); // link
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            loop {
                c.cycles += 4;
                c.g[V1][0] = Addr(r32(c, A2)).lw(c); // node = *link
                c.g[V0][0] = Addr(r32(c, V1)).lw(c); // node->key
                c.g[A0][0] = c.g[V1][0];
                if c.g[V0][0] == c.g[A1][0] {
                    c.cycles += 3;
                    let active = Addr(r32(c, A0).wrapping_add(ANIM_TRIGGER_ACTIVE)).lw(c);
                    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(ANIM_TRIGGER_NEXT)).lw(c);
                    if active != 0 {
                        c.cycles += 4;
                        let next = c.g[V0][0];
                        Addr(r32(c, A2)).sw(c, next); // *link = node->next
                        ANIM_TRIGGER_DELETE.call(c, 0x0031_3be4)?;
                        frame.restore_ra(c, 0);
                        c.cycles += 2;
                        frame.close(c);
                        return Ok(());
                    }
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(ANIM_TRIGGER_NEXT)).lw(c);
                }
                c.cycles += 2;
                c.g[A2][0] = sx(r32(c, A0).wrapping_add(ANIM_TRIGGER_NEXT));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// LensFX_Render (0x2e30d0)

/// `LensFX_PrepareView(fx, view)` (0x2e3338).
const LENS_FX_PREPARE_VIEW: Callee = Callee { addr: 0x002e_3338, f: ee::s09::f_002e3338 };
/// `LensFX_DrawFlares(fx, view)` (0x2e3478).
const LENS_FX_DRAW_FLARES: Callee = Callee { addr: 0x002e_3478, f: ee::s09::f_002e3478 };

/// `void LensFX_Render(LensFX* fx, int view)` (0x2e30d0): prepare the view
/// (camera rect/matrix/FOV, sun, flare projection), then draw its flares.
pub fn lens_fx_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        LENS_FX_PREPARE_VIEW.call(c, 0x002e_30ec)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        LENS_FX_DRAW_FLARES.call(c, 0x002e_30f8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderWipeoutSub_UpdatePhase0 (0x12cd20)

/// `RiderWipeoutSub + 0x80`: the rider.
const WP0_RIDER: u32 = 0x80;
/// Rider fields: model (+0x780, its `+0x2c` the root-node pose array, 0x20
/// bytes per node: position, rotation quaternion), anim controller (+0x784),
/// wipeout state (+0x77c), the two tracked node indices.
const WP0_RIDER_STATE: u32 = 0x77c;
const WP0_RIDER_MODEL: u32 = 0x780;
const WP0_RIDER_ANIM: u32 = 0x784;
const WP0_RIDER_NODE1: u32 = 0x89c;
const WP0_RIDER_NODE2: u32 = 0x8a4;
/// Anim queries on track 2: `AnimController_GetTrackPhase` (f0),
/// `..._IsTrackDone`, `..._GetTrackSeq`, `..._GetTrackAnimType`.
const WP0_TRACK_PHASE: u32 = 0x0031_2ab0;
const WP0_TRACK_DONE: u32 = 0x0031_2ae8;
const WP0_TRACK_SEQ: u32 = 0x0031_1b20;
const WP0_TRACK_TYPE: u32 = 0x0031_1ae8;
/// `RiderWipeoutState_SetVelocity(state + 0x30, rider + 0x1e0, &angvel)`.
const WP0_SET_VELOCITY: u32 = 0x0013_6de0;
/// `RiderWipeoutState_EnableRootPoseOverride(state + 0x30, &vel, &angvel, node2)`.
const WP0_ROOT_OVERRIDE: u32 = 0x0013_6d40;
/// `RiderWipeoutSub_SelectTumbleAnim(self)`.
const WP0_SELECT_TUMBLE: Callee = Callee { addr: 0x0012_da88, f: ee::s00::f_0012da88 };
/// `SSXAudioSystem_GetInstance()`, `..._StartWipeoutLoop(audio, self)`,
/// `..._StartWipeoutLoop2(audio, self)`.
const WP0_AUDIO_INSTANCE: u32 = 0x0028_b180;
const WP0_AUDIO_LOOP: u32 = 0x0029_6868;
const WP0_AUDIO_LOOP2: u32 = 0x0029_6e80;
/// `gp - 0x77c4`: 0.95, the phase up to which the start poses are recorded.
const WP0_RECORD_LIMIT_GP: u32 = 0x77c4;
/// Anim types that end the tumble phase specially.
const WP0_TYPE_ROOT_OVERRIDE: u64 = 0x17;
const WP0_TYPE_LOOP_A: u64 = 0x1d;
const WP0_TYPE_LOOP_B: u64 = 0x19;

/// `v = vf[src] * f[f].x` through the stack: `mfc1 v1, f; lqc2 vf4, src(sp);
/// qmtc2 v1, vf3; vmulx vf5, vf4, vf3; sqc2 vf5, dst(sp); lq v0, dst(sp)`.
fn wp0_scale(c: &mut Cpu, f: usize, src: u32, dst: u32) {
    let s = sp(c);
    c.mfc1(V1, f);
    c.lqc2(4, s.wrapping_add(src));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(dst));
    c.lq(V0, s.wrapping_add(dst));
}

/// `vf5 = [a] - [b]` stored at `dst(sp)`.
fn wp0_sub(c: &mut Cpu, a: u32, b: u32, dst: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    let s = sp(c);
    c.sqc2(5, s.wrapping_add(dst));
}

/// The quaternion product `vf4 * vf5` (xyz cross/dot form) into vf8.
fn wp0_quat_mul(c: &mut Cpu) {
    c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().mula(0b1110, 4, Src::Bc(5, W));
    c.vu0().madda(0b1110, 5, Src::Bc(4, W));
    c.vu0().suba(0b0001, 7, Src::Bc(7, X));
    c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
    c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
    c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
}

/// The conjugate of the node quaternion at `node + 0x10` written to
/// `conj(sp)..+0xc` (`(-x, -y, -z, w)`, stored in the original's order).
fn wp0_store_conj(c: &mut Cpu, node: u32, conj: u32) {
    let s = sp(c).wrapping_add(conj);
    let w = Addr(node.wrapping_add(0x1c)).read_fpu(c);
    abi::set_fpr(c, 0, w);
    Addr(s.wrapping_add(0xc)).write_fpu(c, w);
    let x = Addr(node.wrapping_add(0x10)).read_fpu(c);
    abi::set_fpr(c, 2, x);
    Addr(s).write_fpu(c, x);
    let y = Addr(node.wrapping_add(0x14)).read_fpu(c);
    abi::set_fpr(c, 1, y);
    abi::set_fpr(c, 2, -x);
    Addr(s.wrapping_add(0x4)).write_fpu(c, y);
    abi::set_fpr(c, 1, -y);
    let z = Addr(node.wrapping_add(0x18)).read_fpu(c);
    abi::set_fpr(c, 0, z);
    Addr(s).write_fpu(c, -x);
    abi::set_fpr(c, 0, -z);
    Addr(s.wrapping_add(0x4)).write_fpu(c, -y);
    Addr(s.wrapping_add(0x8)).write_fpu(c, -z);
}

/// Copy the xyz of the quadword at `src(sp)` to `dst(sp)` through f0..f2.
fn wp0_copy_xyz(c: &mut Cpu, src: u32, dst: u32) {
    let s = sp(c);
    c.lwc1(0, s.wrapping_add(src));
    c.lwc1(1, s.wrapping_add(src + 4));
    c.lwc1(2, s.wrapping_add(src + 8));
    c.swc1(0, s.wrapping_add(dst));
    c.swc1(1, s.wrapping_add(dst + 4));
    c.swc1(2, s.wrapping_add(dst + 8));
}

/// `void RiderWipeoutSub_UpdatePhase0(RiderWipeoutSub* self)` (0x12cd20):
/// the tumble start. While the track-2 animation plays, record its phase and
/// the two tracked root-node poses (until phase 0.95). When it is done, turn
/// the pose change since the record into a linear and angular velocity for
/// the wipeout state (and, for anim type 0x17, a root-pose override from the
/// second node), then select the tumble animation: phase 1 if the state is in
/// mode 1, else phase 2 and start the wipeout audio loop (type 0x1d/0x19 or
/// the other one).
pub fn rider_wipeout_sub_update_phase0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0xe0);
        c.g[A1][0] = 2;
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0xa0);
        frame.save_fpr(c, 20, 0xd0);
        frame.save(c, S3, 0x90);
        frame.save_ra(c, 0x80);
        frame.save_fpr(c, 21, 0xd8);
        let rider = Addr(r32(c, S0).wrapping_add(WP0_RIDER)).lw(c);
        c.g[V1][0] = rider;
        let rider = rider as u32;
        c.g[A0][0] = Addr(rider.wrapping_add(WP0_RIDER_MODEL)).lw(c);
        c.g[A2][0] = Addr(rider.wrapping_add(WP0_RIDER_NODE1)).lw(c);
        c.g[V0][0] = Addr(rider.wrapping_add(WP0_RIDER_NODE2)).lw(c);
        c.g[A3][0] = Addr(r32(c, A0).wrapping_add(0x2c)).lw(c); // nodes
        c.g[A2][0] = sx(r32(c, A2) << 5);
        c.g[V0][0] = sx(r32(c, V0) << 5);
        c.g[A0][0] = Addr(rider.wrapping_add(WP0_RIDER_ANIM)).lw(c);
        c.g[S2][0] = sx(r32(c, A3).wrapping_add(r32(c, V0))); // node 2
        c.g[S1][0] = sx(r32(c, A3).wrapping_add(r32(c, A2))); // node 1
        call_ptr(c, WP0_TRACK_PHASE, 0x0012_cd74)?;
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(WP0_RIDER)).lw(c);
        c.f[20] = c.f[0]; // phase
        c.g[A1][0] = 2;
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(WP0_RIDER_ANIM)).lw(c);
        call_ptr(c, WP0_TRACK_DONE, 0x0012_cd88)?;
        let done = c.g[V0][0] != 0;
        c.g[A1][0] = 2;

        if !done {
            // Still playing: record the phase and both node poses up to 0.95.
            c.cycles += 5;
            let limit = Addr(r32(c, GP).wrapping_sub(WP0_RECORD_LIMIT_GP)).read_fpu(c);
            abi::set_fpr(c, 0, limit);
            let record = FpuF32(c.f[20]).lt(limit);
            abi::set_fcc(c, record);
            if record {
                c.cycles += 10;
                let (this, n1, n2) = (r32(c, S0), r32(c, S1), r32(c, S2));
                c.swc1(20, this.wrapping_add(0x50));
                c.lq(V0, n1);
                c.sq(V0, this.wrapping_add(0x10));
                c.lq(V1, n1.wrapping_add(0x10));
                c.sq(V1, this.wrapping_add(0x20));
                c.lq(V0, n2);
                c.sq(V0, this.wrapping_add(0x30));
                c.lq(V1, n2.wrapping_add(0x10));
                c.sq(V1, this.wrapping_add(0x40));
            }
        } else {
            c.cycles += 96;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(WP0_RIDER)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(WP0_RIDER_ANIM)).lw(c);
            call_ptr(c, WP0_TRACK_SEQ, 0x0012_cd9c)?;
            let seq = r32(c, V0);
            let (this, n1) = (r32(c, S0), r32(c, S1));
            let length = Addr(seq.wrapping_add(0x10)).read_fpu(c);
            abi::set_fpr(c, 0, length);
            c.g[S3][0] = sx(sp(c).wrapping_add(0x30));
            let rate = Addr(seq.wrapping_add(0x90)).read_fpu(c);
            abi::set_fpr(c, 2, rate);
            c.g[A2][0] = c.g[S3][0];
            let start = Addr(this.wrapping_add(0x50)).read_fpu(c);
            abi::set_fpr(c, 1, start);
            // Linear: (node1.pos - start pos) * anim scale * s.
            wp0_sub(c, n1, this.wrapping_add(0x10), 0x30);
            let s0 = sp(c);
            c.lq(V0, s0.wrapping_add(0x30));
            let elapsed = FpuF32(c.f[20]) - start;
            c.g[V1][0] = Addr(this.wrapping_add(WP0_RIDER)).lw(c);
            c.sq(V0, s0.wrapping_add(0x20));
            let denom = elapsed * length;
            abi::set_fpr(c, 1, denom);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(WP0_RIDER_ANIM)).lw(c);
            let scale = Addr(r32(c, V0).wrapping_add(0x1c)).read_fpu(c);
            abi::set_fpr(c, 0, scale);
            let speed = rate.div(denom); // s
            abi::set_fpr(c, 20, speed);
            wp0_scale(c, 0, 0x20, 0x30);
            c.sq(V0, s0.wrapping_add(0x10));
            wp0_scale(c, 20, 0x10, 0x30);
            abi::set_fpr(c, 21, speed + speed);
            c.sq(V0, s0);
            // Angular: (node1.rot - start rot) * conj(node1.rot), xyz, * 2s.
            wp0_sub(c, n1.wrapping_add(0x10), this.wrapping_add(0x20), 0x20);
            c.lq(V0, s0.wrapping_add(0x20));
            c.sq(V0, s0.wrapping_add(0x10));
            wp0_store_conj(c, n1, 0x40);
            c.lq(V0, s0.wrapping_add(0x40));
            c.sq(V0, s0.wrapping_add(0x30));
            c.lqc2(4, s0.wrapping_add(0x10));
            c.lqc2(5, s0.wrapping_add(0x30));
            wp0_quat_mul(c);
            c.sqc2(8, s0.wrapping_add(0x40));
            c.lq(V0, s0.wrapping_add(0x40));
            Addr(s0.wrapping_add(0x4c)).sw(c, 0);
            c.sq(V0, s0.wrapping_add(0x20));
            wp0_copy_xyz(c, 0x20, 0x40);
            wp0_scale(c, 21, 0x40, 0x50);
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(WP0_RIDER)).lw(c);
            c.sq(V0, s0.wrapping_add(0x30));
            c.g[A1][0] = sx(r32(c, V1).wrapping_add(0x1e0));
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(WP0_RIDER_STATE)).lw(c);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x30));
            call_ptr(c, WP0_SET_VELOCITY, 0x0012_cef4)?;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(WP0_RIDER)).lw(c);
            c.g[A1][0] = 2;
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(WP0_RIDER_ANIM)).lw(c);
            call_ptr(c, WP0_TRACK_TYPE, 0x0012_cf04)?;
            c.g[V1][0] = WP0_TYPE_ROOT_OVERRIDE;
            if c.g[V0][0] == c.g[V1][0] {
                // The same from node 2, handed to the root-pose override.
                c.cycles += 84;
                let (this, n2) = (r32(c, S0), r32(c, S2));
                let s0 = sp(c);
                wp0_sub(c, n2, this.wrapping_add(0x30), 0x70);
                c.g[A2][0] = c.g[S3][0];
                c.lq(V1, s0.wrapping_add(0x70));
                c.g[A3][0] = c.g[S2][0];
                c.g[A0][0] = Addr(this.wrapping_add(WP0_RIDER)).lw(c);
                c.g[A1][0] = c.g[SP][0];
                c.sq(V1, s0.wrapping_add(0x60));
                c.g[V0][0] = Addr(r32(c, A0).wrapping_add(WP0_RIDER_ANIM)).lw(c);
                let scale = Addr(r32(c, V0).wrapping_add(0x1c)).read_fpu(c);
                abi::set_fpr(c, 0, scale);
                wp0_scale(c, 0, 0x60, 0x70);
                c.sq(V0, s0.wrapping_add(0x50));
                wp0_scale(c, 20, 0x50, 0x70);
                c.sq(V0, s0.wrapping_add(0x40));
                c.sq(V0, s0);
                wp0_sub(c, n2.wrapping_add(0x10), this.wrapping_add(0x40), 0x50);
                c.lq(V0, s0.wrapping_add(0x50));
                c.sq(V0, s0.wrapping_add(0x10));
                c.sq(V0, s0.wrapping_add(0x40));
                wp0_store_conj(c, n2, 0x60);
                c.lq(V0, s0.wrapping_add(0x60));
                c.sq(V0, s0.wrapping_add(0x50));
                c.lqc2(4, s0.wrapping_add(0x10));
                c.lqc2(5, s0.wrapping_add(0x50));
                wp0_quat_mul(c);
                c.sqc2(8, s0.wrapping_add(0x60));
                c.lq(V0, s0.wrapping_add(0x60));
                Addr(s0.wrapping_add(0x5c)).sw(c, 0);
                c.sq(V0, s0.wrapping_add(0x20));
                c.sq(V0, s0.wrapping_add(0x40));
                wp0_copy_xyz(c, 0x20, 0x50);
                wp0_scale(c, 21, 0x50, 0x60);
                c.g[V1][0] = Addr(r32(c, S0).wrapping_add(WP0_RIDER)).lw(c);
                c.sq(V0, s0.wrapping_add(0x30));
                c.sq(V0, s0.wrapping_add(0x40));
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(WP0_RIDER_STATE)).lw(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x30));
                call_ptr(c, WP0_ROOT_OVERRIDE, 0x0012_d05c)?;
            }
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(WP0_RIDER)).lw(c);

            // Next phase: 1 when the wipeout state is in mode 1, else 2.
            c.cycles += 5;
            c.g[A0][0] = 1;
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(WP0_RIDER_STATE)).lw(c);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x30)).lw(c);
            if c.g[V0][0] == c.g[A0][0] {
                c.cycles += 5;
                let phase = c.g[V0][0];
                Addr(r32(c, S0)).sw(c, phase);
                c.g[A0][0] = c.g[S0][0];
                WP0_SELECT_TUMBLE.call(c, 0x0012_d080)?;
            } else {
                c.g[V0][0] = 2;
                c.cycles += 10;
                c.g[A0][0] = c.g[S0][0];
                Addr(r32(c, S0)).sw(c, 2);
                WP0_SELECT_TUMBLE.call(c, 0x0012_d094)?;
                c.g[V0][0] = Addr(r32(c, S0).wrapping_add(WP0_RIDER)).lw(c);
                c.g[A1][0] = 2;
                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(WP0_RIDER_ANIM)).lw(c);
                call_ptr(c, WP0_TRACK_TYPE, 0x0012_d0a4)?;
                c.g[V1][0] = WP0_TYPE_LOOP_A;
                let mut loop_a = c.g[V0][0] == c.g[V1][0];
                c.g[A1][0] = 2;
                if !loop_a {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(WP0_RIDER)).lw(c);
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(WP0_RIDER_ANIM)).lw(c);
                    call_ptr(c, WP0_TRACK_TYPE, 0x0012_d0bc)?;
                    c.g[V1][0] = WP0_TYPE_LOOP_B;
                    loop_a = c.g[V0][0] == c.g[V1][0];
                }
                c.cycles += 7;
                let (ret1, start, ret2) = if loop_a {
                    (0x0012_d0d0, WP0_AUDIO_LOOP, 0x0012_d0dc)
                } else {
                    (0x0012_d0ec, WP0_AUDIO_LOOP2, 0x0012_d0f8)
                };
                call_ptr(c, WP0_AUDIO_INSTANCE, ret1)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, start, ret2)?;
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore_ra(c, 0x80);
        frame.restore_fpr(c, 21, 0xd8);
        frame.restore_fpr(c, 20, 0xd0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable};
    use crate::difftest::{check, DATA, GAME_GP};

    fn oc258_w32(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// The BE-library dummy at gp-0x1f04 and a dummy world at gp-0x848.
    fn oc258_env(c: &mut Cpu) {
        oc258_w32(c, GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
        let wld = DATA + 0x1000;
        oc258_w32(c, GAME_GP.wrapping_sub(0x848), wld);
        oc258_w32(c, wld + 0x84, DATA + 0x1200);
        oc258_w32(c, DATA + 0x1200 + 0x20, DATA + 0x3000);
        oc258_w32(c, DATA + 0x1200 + 0x28, DATA + 0x1400);
        oc258_w32(c, wld + 0x7c, DATA + 0x1500);
    }

    #[test]
    fn message_center_update_matches() {
        check(ee::s04::f_001e1458__generated, message_center_update, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1e1458, 0);
            oc258_env(c);
            c.pc = 0x001e_1458;
            let gp = GAME_GP;
            // Suspended in 1 of 5 seeds.
            oc258_w32(c, gp - MC_SUSPENDED, if seed % 5 == 0 { r.next_u32() | 1 } else { 0 });
            oc258_w32(c, gp - MC_TABLE_READY, (seed / 2 % 2) as u32);
            let owner = DATA + 0x2000;
            oc258_w32(c, gp + MC_CLOCK_OWNER, owner);
            let clock = r.next_u32() >> 2;
            oc258_w32(c, owner + MC_CLOCK, clock);
            // Elapsed time: below, at and past the 600-tick period.
            let elapsed = match seed % 4 {
                0 => 600,
                1 => 601,
                2 => r.next_u32() % 600,
                _ => 601 + r.next_u32() % 5000,
            };
            oc258_w32(c, gp - MC_LAST_TRIGGER, clock.wrapping_sub(elapsed));
            // Trigger table types (0x441630, 0x14 per group).
            for i in 0..0x40u32 {
                let kind = r.next_u32() % 7;
                oc258_w32(c, 0x0044_1630 + 0x14 * i, kind);
            }
        });
    }

    /// Weather48: weather off (f0 = 0) or the slot object's virtual accessor.
    #[test]
    fn world_painter_get_weather48_matches() {
        check(ee::s09::f_002ee600__generated, world_painter_get_weather48, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x2ee600, 1);
            c.pc = 0x002e_e600;
            oc258_w32(c, GAME_GP + LENS_FX_WEATHER_OFF, if seed % 4 == 0 { r.next_u32() | 1 } else { 0 });
            let i = seed % 8;
            c.g[A0][0] = i as u64;
            let slot = LENS_FX_SLOTS + 0xf0 * i;
            let obj = DATA + 0x200;
            let vt = vtable(c, &mut r, &[0x198]);
            oc258_w32(c, slot + 0x20, DATA + 0x100);
            oc258_w32(c, DATA + 0x100, obj);
            oc258_w32(c, obj + 4, vt);
        });
    }

    /// MultiParticleMan_Update: any mix of empty and present slots; present
    /// systems have a null or zeroed particle system (Emitter_Update's short exit).
    #[test]
    fn multi_particle_man_update_matches() {
        check(ee::s11::f_00357bf8__generated, multi_particle_man_update, 48, &|c, seed| {
            let mut r = machine(c, seed, 0x357bf8, 0);
            oc258_env(c);
            c.pc = 0x0035_7bf8;
            let man = DATA + 0x100;
            c.g[A0][0] = sx(man);
            c.bus.write32(GAME_GP.wrapping_sub(0x2a80), r.f32(1.0));
            for k in 0..4u32 {
                let present = (seed >> k) & 1 != 0 || seed % 7 == 3;
                let p = DATA + 0x4000 + 0x400 * k;
                oc258_w32(c, man + 4 * k, if present { p } else { 0 });
                let sys = DATA + 0x8000 + 0x400 * k;
                oc258_w32(c, p + 0x10, if (seed + k) % 2 == 0 { 0 } else { sys });
                oc258_w32(c, sys, 0);
            }
        });
    }

    /// RiderAnim_UpdateSeqType07: either actor float the larger, weight fades
    /// that finish (list removal) or not; the real track setter, fade and
    /// list removal run.
    #[test]
    fn rider_anim_update_seq_type07_matches() {
        use crate::difftest::fakes::ptr;
        check(ee::s00::f_00104358__generated, rider_anim_update_seq_type07, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0010_4358, 0);
            crate::difftest::elf_image(c);
            c.pc = 0x0010_4358;
            let seq = ptr(&mut r);
            let list = ptr(&mut r);
            let anim = ptr(&mut r);
            let actor = DATA + 0x20_0000;
            c.g[A0][0] = sx(anim);
            c.g[A1][0] = sx(list);
            c.g[A2][0] = sx(seq);
            oc258_w32(c, anim + SEQ07_ACTOR, actor);
            for off in [SEQ07_ACTOR_A, SEQ07_ACTOR_B] {
                let v = if r.next_u32() % 5 == 0 { r.edge_f32() } else { r.f32(30.0) };
                oc258_w32(c, actor + off, v);
            }
            for off in [0x8, 0xc, 0x10, 0x90, 0x94, 0x98, 0x9c, 0xa4, 0xa8] {
                let v = match r.next_u32() % 4 {
                    0 => 0,
                    1 => r.edge_f32(),
                    _ => r.f32(3.0),
                };
                oc258_w32(c, seq + off, v);
            }
            oc258_w32(c, seq + 0x1c, r.next_u32() & 1);
            oc258_w32(c, seq + 0xa0, r.next_u32() & 1);
            let mut at = list + 4;
            for _ in 0..seed % 3 {
                let other = ptr(&mut r) + 0x8;
                oc258_w32(c, at, other);
                at = other + 200;
            }
            c.f[12] = if seed % 4 == 0 { r.edge_f32() } else { r.f32(0.05) };
        });
    }

    /// RiderBase_SaveState: a stream whose methods are fakes; the real
    /// sub-state savers run on zeroed objects (their writes go to the fake
    /// stream); targets present or not; path indices found or not.
    #[test]
    fn rider_base_save_state_matches() {
        check(ee::s00::f_00111ac0__generated, rider_base_save_state, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x0011_1ac0, 40);
            c.pc = 0x0011_1ac0;
            let stream = DATA + 0x10;
            let vt = vtable(c, &mut r, &[0x8]);
            oc258_w32(c, stream, vt);
            let this = DATA + 0x20_0000;
            crate::difftest::fill(c, &mut r, this, 0xe00 / 4);
            let rider = DATA + 0x21_0000;
            oc258_w32(c, this + RB_RIDER, rider);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(stream);
            // Zeroed part objects (anim, model, air predictor, metrix).
            for (k, f) in [RB_RIDER_ANIM, RB_RIDER_MODEL, RB_RIDER_AIR_PREDICTOR, RB_RIDER_METRIX]
                .into_iter()
                .enumerate()
            {
                oc258_w32(c, rider + f, DATA + 0x22_0000 + 0x1000 * k as u32);
            }
            // The anim's six sequence lists (8 bytes each, `count <= 0`).
            oc258_w32(c, DATA + 0x22_0000 + 0x50, DATA + 0x22_8000);
            // Targets: `*field` null or an object with an id.
            for (k, f) in [RB_RIDER_TARGET0, RB_RIDER_TARGET1].into_iter().enumerate() {
                let cell = DATA + 0x23_0000 + 0x400 * k as u32;
                let obj = cell + 0x100;
                oc258_w32(c, rider + f, cell);
                let present = (seed >> k) & 1 != 0;
                oc258_w32(c, cell, if present { obj } else { 0 });
                oc258_w32(c, obj + RB_TARGET_ID, r.next_u32());
            }
            oc258_w32(c, rider + RB_RIDER_BLOCK0, r.next_u32());
            oc258_w32(c, rider + RB_RIDER_BLOCK1, r.next_u32());
            // Path data: n track paths of 0x3c bytes (count +0x10, array +0x14)
            // and n AI paths of 0x40 bytes (count +0x8, array +0xc); the
            // rider's paths are one of them, a stranger, or null.
            let n = seed % 5;
            let tracks = DATA + 0x24_0000;
            let ais = DATA + 0x25_0000;
            oc258_w32(c, RB_PATH_DATA + 0x10, n);
            oc258_w32(c, RB_PATH_DATA + 0x14, tracks);
            oc258_w32(c, RB_PATH_DATA + 0x8, n);
            oc258_w32(c, RB_PATH_DATA + 0xc, ais);
            for (f, base, stride) in [(RB_RIDER_TRACK_PATH, tracks, 0x3c), (RB_RIDER_AI_PATH, ais, 0x40)] {
                let v = match r.next_u32() % 3 {
                    0 => 0,
                    1 => base + stride * (r.next_u32() % 5),
                    _ => r.next_u32(),
                };
                oc258_w32(c, rider + f, v);
            }
        });
    }

    /// A lens slot `i` whose `field` points at an object with a fake
    /// virtual accessor at vtable `vt_off`.
    fn oc258_lens_slot(c: &mut Cpu, seed: u32, salt: u64, field: u32, vt_off: u32) {
        let mut r = machine(c, seed, salt, 1);
        let i = seed % 8;
        c.g[A0][0] = i as u64;
        let slot = LENS_FX_SLOTS + 0xf0 * i;
        let obj = DATA + 0x200;
        let vt = vtable(c, &mut r, &[vt_off]);
        oc258_w32(c, slot + field, DATA + 0x100);
        oc258_w32(c, DATA + 0x100, obj);
        oc258_w32(c, obj + 4, vt);
    }

    #[test]
    fn world_painter_get_light_glow10_matches() {
        check(ee::s09::f_002eedf8__generated, world_painter_get_light_glow10, 32, &|c, seed| {
            oc258_lens_slot(c, seed, 0x2eedf8, 0x8, 0x70);
            c.pc = 0x002e_edf8;
        });
    }

    #[test]
    fn world_painter_get_weather40_matches() {
        check(ee::s09::f_002ee6a8__generated, world_painter_get_weather40, 32, &|c, seed| {
            oc258_lens_slot(c, seed, 0x2ee6a8, 0x20, 0x190);
            c.pc = 0x002e_e6a8;
        });
    }

    /// A setup call into a generated function (returns to a sentinel).
    fn oc258_run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        const BACK: u32 = 0x0000_7770;
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2][i]][0] = *a;
        }
        c.g[RA][0] = BACK as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, BACK, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// AnimSequence_RemoveEventTrigger: an empty list, or 1..6 heap-allocated
    /// triggers with keys from a small set and random active flags, so the
    /// key is missing, found inactive, or found active (unlinked, deleted).
    #[test]
    fn anim_sequence_remove_event_trigger_matches() {
        const MEM_HEAP: u32 = 0x004f_f250;
        check(ee::s10::f_00313ba8__generated, anim_sequence_remove_event_trigger, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x0031_3ba8, 0);
            oc258_env(c);
            let ra = c.g[RA][0];
            oc258_run(
                c,
                0x0031_ff60,
                ee::s10::DLHeap_InitArena,
                &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
            );
            let seq = DATA + 0x100;
            let n = if seed % 6 == 0 { 0 } else { 1 + r.next_u32() % 6 };
            let mut link = seq + ANIM_SEQ_TRIGGERS;
            for _ in 0..n {
                let node =
                    oc258_run(c, 0x0031_e818, ee::s10::DLHeap_Malloc, &[MEM_HEAP as u64, 0x10]) as u32;
                oc258_w32(c, link, node);
                oc258_w32(c, node, r.next_u32() % 4);
                oc258_w32(c, node + ANIM_TRIGGER_ACTIVE, r.next_u32() % 2);
                oc258_w32(c, node + 8, r.next_u32());
                link = node + ANIM_TRIGGER_NEXT;
            }
            oc258_w32(c, link, 0);
            c.g[RA][0] = ra;
            c.g[A0][0] = sx(seq);
            c.g[A1][0] = (r.next_u32() % 4) as u64;
            c.pc = 0x0031_3ba8;
        });
    }

    /// LensFX_Render: the `LensFX_PrepareView` fixture (camera interface
    /// slots and the projector's occlusion slot are fakes, the matrix call
    /// returns a random matrix, the painter's sun texture is -1) combined
    /// with the `LensFX_DrawFlares` one (sun on or off, push/pop render-state
    /// fakes, the camera doubling as the flare-quad renderer); views 0 and 1,
    /// zero, negative and 1-4 flares; some fake calls unwind.
    #[test]
    fn lens_fx_render_matches() {
        use crate::difftest::fakes::{FAKE, LOG_RESULTS};
        use crate::difftest::fill;
        // Camera / renderer (`*(gp + 0x2a90)`) and its interface vtable.
        const CAMERA_GP: u32 = 0x2a90;
        const CAM_VT: u32 = 0x10d8;
        // LensFX layout.
        const COUNTS: u32 = 0x60;
        const FLARES: u32 = 0x68;
        const FLARE_VIEW: u32 = 0x3000;
        const FLARE: u32 = 0x30;
        const FX_VT: u32 = 0x649c;
        check(ee::s09::f_002e30d0__generated, lens_fx_render, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x002e_30d0, 8);
            c.pc = 0x002e_30d0;
            let fx = DATA + 0x4_0000;
            let cam = DATA + 0x5_0000;
            let cvt = DATA + 0x5_1000;
            let painter_vt = DATA + 0x5_3000;
            let holder = DATA + 0x5_2000;
            let no_sun = DATA + 0x5_4000;
            let src = DATA + 0x5_5000;
            let matrix = DATA + 0x5_6000;
            let top = DATA + 0x5_7000;
            let view = seed % 2;
            c.g[A0][0] = sx(fx);
            c.g[A1][0] = view as u64;
            // Camera interface: view rect, matrix, FOV, occlusion, sprite draw.
            oc258_w32(c, GAME_GP + CAMERA_GP, cam);
            oc258_w32(c, cam + CAM_VT, cvt);
            for (i, off) in [0xe0u32, 0x118, 0xf0, 0x2d8, 0x278].into_iter().enumerate() {
                let delta = (r.next_u32() as i16 as i32 % 64) & !3;
                c.bus.write16(cvt + off, delta as u16);
                oc258_w32(c, cvt + off + 4, FAKE + 0x10 * i as u32);
            }
            oc258_w32(c, cam + 0xe84, top);
            fill(c, &mut r, top, 8);
            oc258_w32(c, cam + 0x1024, r.next_u32());
            oc258_w32(c, cam + 0x1028, r.next_u32());
            fill(c, &mut r, matrix, 16);
            oc258_w32(c, LOG_RESULTS + 4, matrix);
            // Painter views 6 and 7: sun texture -1 (fake call 3).
            for v in [6u32, 7] {
                let entry = LENS_FX_SLOTS + v * 0xf0;
                let obj = holder + 0x40;
                oc258_w32(c, entry + 0x14, holder);
                oc258_w32(c, holder, obj);
                oc258_w32(c, obj + 4, painter_vt);
            }
            let delta = (r.next_u32() as i16 as i32 % 64) & !3;
            c.bus.write16(painter_vt + 0x108, delta as u16);
            oc258_w32(c, painter_vt + 0x10c, FAKE);
            oc258_w32(c, no_sun, u32::MAX);
            oc258_w32(c, LOG_RESULTS + 12, no_sun);
            // Sun drawing on or off; push/pop render-state fakes.
            oc258_w32(c, GAME_GP + 0xa28, if seed % 3 == 0 { 0 } else { r.next_u32() | 1 });
            let fvt = DATA + 0x5_8000;
            for (i, off) in [0x10u32, 0x18].into_iter().enumerate() {
                let delta = (r.next_u32() as i16 as i32 & !3) as u16;
                c.bus.write16(fvt + off, delta);
                oc258_w32(c, fvt + off + 4, FAKE + 0x40 + 0x10 * i as u32);
            }
            oc258_w32(c, fx + FX_VT, fvt);
            // The lens FX header, flare count and flares sharing one source.
            fill(c, &mut r, fx, 0x18);
            fill(c, &mut r, src, 0x10);
            for j in 0..3 {
                let v = if r.next_u32() % 7 == 3 { 0 } else { r.f32(50.0) };
                oc258_w32(c, src + 0x10 + 4 * j, v);
                oc258_w32(c, src + 0x1c + 4 * j, r.f32(1000.0));
            }
            for j in 0..4 {
                let a = match r.next_u32() % 4 {
                    0 => 0,
                    1 => r.f32(1.0) | 0x8000_0000,
                    _ => r.f32(1.0) & !0x8000_0000,
                };
                oc258_w32(c, src + 0x40 + 4 * j, a);
            }
            let kind = [0x10, 0x20, 0x40, 0x00, 0x30, 0x50, 0x60, 0x70][(r.next_u32() % 8) as usize];
            oc258_w32(c, src + 0xc, (r.next_u32() & !0x70) | kind);
            let count = match seed % 4 {
                0 => 0,
                1 => (r.next_u32() % 5 + 1).wrapping_neg(),
                _ => 1 + r.next_u32() % 4,
            };
            oc258_w32(c, fx + COUNTS + 4 * view, count);
            for k in 0..4 {
                let flare = fx + FLARES + view * FLARE_VIEW + k * FLARE;
                fill(c, &mut r, flare, 0xc);
                oc258_w32(c, flare + 0x20, src);
                oc258_w32(c, flare + 0x28, r.f32(6.3));
                let on = if r.next_u32() % 2 == 0 { 0 } else { 1 + r.next_u32() as u16 % 3 };
                c.bus.write16(flare + 0x2c, on);
                c.bus.write16(flare + 0x2e, (r.next_u32() % 2) as u16);
            }
        });
    }

    /// RiderWipeoutSub_UpdatePhase0: the track-2 anim still playing (phase
    /// below or above 0.95) or done; done with anim types 0x17 (root-pose
    /// override), 0x1d, 0x19 or another; the wipeout state in mode 1 or not.
    /// The real anim queries, velocity setter (and its air-predictor prime),
    /// root-pose override, tumble selection (an anim id outside its table:
    /// straight to its epilogue) and audio loops (the rider's loop handles
    /// are non-negative: their early exit) run.
    #[test]
    fn rider_wipeout_sub_update_phase0_matches() {
        check(ee::s00::f_0012cd20__generated, rider_wipeout_sub_update_phase0, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x0012_cd20, 0);
            c.pc = 0x0012_cd20;
            let this = DATA + 0x100;
            let rider = DATA + 0x1000;
            let model = DATA + 0x3000;
            let nodes = DATA + 0x3100;
            let anim = DATA + 0x4000;
            let lists = DATA + 0x4100;
            let seq = DATA + 0x4200;
            let state = DATA + 0x5000;
            let predictor = DATA + 0x6000;
            c.g[A0][0] = sx(this);
            // The recorded start: phase at +0x50, node poses at +0x10..+0x50.
            for k in 0..0x10u32 {
                oc258_w32(c, this + 0x10 + 4 * k, r.f32(2.0));
            }
            oc258_w32(c, this + 0x50, r.f32(0.5));
            oc258_w32(c, this, r.next_u32() % 4);
            oc258_w32(c, this + WP0_RIDER, rider);
            // Rider: model nodes, anim controller, wipeout state.
            oc258_w32(c, rider + WP0_RIDER_MODEL, model);
            oc258_w32(c, model + 0x2c, nodes);
            for k in 0..0x20u32 {
                oc258_w32(c, nodes + 4 * k, r.f32(2.0));
            }
            oc258_w32(c, rider + WP0_RIDER_NODE1, r.next_u32() % 4);
            oc258_w32(c, rider + WP0_RIDER_NODE2, r.next_u32() % 4);
            oc258_w32(c, rider + WP0_RIDER_ANIM, anim);
            oc258_w32(c, rider + WP0_RIDER_STATE, state);
            oc258_w32(c, rider + 0x774, 0);
            oc258_w32(c, rider + 0x778, 0);
            oc258_w32(c, rider + 0x788, predictor);
            for k in 0..4u32 {
                oc258_w32(c, rider + 0x370 + 4 * k, r.f32(1.0));
                oc258_w32(c, rider + 0x1e0 + 4 * k, r.f32(10.0));
            }
            // Anim controller: track-2 sequence list -> the playing sequence.
            oc258_w32(c, anim + 0x50, lists);
            oc258_w32(c, lists + 0x10 + 4, seq);
            oc258_w32(c, anim + 0x1c, r.f32(2.0));
            let length = 0.5 + (r.next_u32() % 100) as f32 / 50.0;
            let phase = match seed % 3 {
                0 => 0.97,
                _ => (r.next_u32() % 100) as f32 / 100.0,
            };
            oc258_w32(c, seq + 0x10, length.to_bits());
            oc258_w32(c, seq + 0x8, (length * phase).to_bits());
            oc258_w32(c, seq + 0x90, r.f32(3.0));
            oc258_w32(c, seq + 0xc0, if seed % 4 == 0 { 0 } else { 1 });
            // Track-2 anim id and its type in the anim table (0x446990, 0x1c each).
            let id = 1 + seed % 7;
            oc258_w32(c, anim + 0x8, id);
            let kind = [0x17u32, 0x1d, 0x19, 0x5][(seed / 4 % 4) as usize];
            oc258_w32(c, 0x0044_6990 + 0x1c * id, kind);
            // Wipeout state: mode at +0x30 (1 or not), its rider at +0x70.
            oc258_w32(c, state + 0x30, if seed / 16 % 2 == 0 { 1 } else { 0 });
            oc258_w32(c, state + 0x70, rider);
            oc258_w32(c, GAME_GP.wrapping_sub(WP0_RECORD_LIMIT_GP), 0.95f32.to_bits());
        });
    }
}
