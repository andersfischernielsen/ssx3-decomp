//! `src/animation/rideranim.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::oc344_at;
pub(crate) use crate::game::misc::add32;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0010_36a0,
        rider_anim_update_sequence,
        "RiderAnim_UpdateSequence",
    ),
    (0x0010_3918, rider_anim_v03, "RiderAnim_v03"),
    (
        0x0010_4110,
        rider_anim_update_seq_type03,
        "RiderAnim_UpdateSeqType03",
    ),
    (
        0x0010_4178,
        rider_anim_update_seq_type04,
        "RiderAnim_UpdateSeqType04",
    ),
    (
        0x0010_4238,
        rider_anim_update_seq_type05,
        "RiderAnim_UpdateSeqType05",
    ),
    (
        0x0010_42a8,
        rider_anim_update_seq_type06,
        "RiderAnim_UpdateSeqType06",
    ),
    (
        0x0010_42e0,
        rider_anim_update_seq_type0a,
        "RiderAnim_UpdateSeqType0A",
    ),
    (
        0x0010_45b8,
        rider_anim_update_seq_type0c,
        "RiderAnim_UpdateSeqType0C",
    ),
    (
        0x0010_4660,
        rider_anim_update_seq_type09,
        "RiderAnim_UpdateSeqType09",
    ),
    (
        0x0010_46b0,
        rider_anim_update_seq_type0f,
        "RiderAnim_UpdateSeqType0F",
    ),
    (0x0010_4d38, rider_anim_set_rider, "RiderAnim_SetRider"),
    (0x0010_4d50, rider_anim_set_actor, "RiderAnim_SetActor"),
    (
        0x0010_49c0,
        rider_anim_seq_type02_update,
        "RiderAnim_UpdateSeqType02",
    ),
    (
        0x0010_3aa0,
        rider_anim_fire_sound_events,
        "RiderAnim_FireSoundEvents",
    ),
    (0x0010_3e28, rider_anim_blend_seq4, "RiderAnim_BlendSeq4"),
    (
        0x0010_48c0,
        rider_anim_seq_type00_update,
        "RiderAnim_UpdateSeqType00",
    ),
    (0x0010_3cc8, rider_anim_blend_seq2, "RiderAnim_BlendSeq2"),
    (
        0x0010_4358,
        rider_anim_update_seq_type07,
        "RiderAnim_UpdateSeqType07",
    ),
    (
        0x0010_4940,
        rider_anim_seq_type01_update,
        "RiderAnim_UpdateSeqType01",
    ),
    (0x0010_4cf8, rider_anim_v04, "RiderAnim_v04"),
    (
        0x0010_3ff8,
        rider_anim_play_seq_then_loop,
        "RiderAnim_PlaySeqThenLoop",
    ),
    (0x0010_4d68, rider_anim_save_state, "RiderAnim_SaveState"),
    (
        0x0010_4a60,
        rider_anim_seq_end_type02,
        "RiderAnim_SeqEndType02",
    ),
    (
        0x0010_4b48,
        rider_anim_seq_end_type05,
        "RiderAnim_SeqEndType05",
    ),
    (
        0x0010_4c18,
        rider_anim_seq_end_type04,
        "RiderAnim_SeqEndType04",
    ),
    (
        0x0010_4ca0,
        rider_anim_seq_end_type00,
        "RiderAnim_SeqEndType00",
    ),
    (0x0010_3be0, rider_anim_scrub_seq, "RiderAnim_ScrubSeq"),
    (
        0x0010_43f8,
        rider_anim_update_seq_type0b,
        "RiderAnim_UpdateSeqType0B",
    ),
    (
        0x0010_45d8,
        rider_anim_update_seq_type08,
        "RiderAnim_UpdateSeqType08",
    ),
    (
        0x0010_4728,
        rider_anim_update_seq_type0e,
        "RiderAnim_UpdateSeqType0E",
    ),
    (
        0x0010_47f0,
        rider_anim_update_seq_type0d,
        "RiderAnim_UpdateSeqType0D",
    ),
    (
        0x0010_4a40,
        rider_anim_seq_end_type01,
        "RiderAnim_SeqEndType01",
    ),
    (
        0x0010_4b78,
        rider_anim_seq_end_type03,
        "RiderAnim_SeqEndType03",
    ),
    (
        0x0010_4b98,
        rider_anim_seq_end_type07,
        "RiderAnim_SeqEndType07",
    ),
    (
        0x0010_4bb8,
        rider_anim_seq_end_type08,
        "RiderAnim_SeqEndType08",
    ),
    (
        0x0010_4bd8,
        rider_anim_seq_end_type09,
        "RiderAnim_SeqEndType09",
    ),
    (
        0x0010_4c38,
        rider_anim_seq_end_type06,
        "RiderAnim_SeqEndType06",
    ),
    (
        0x0010_4c80,
        rider_anim_seq_end_type0a,
        "RiderAnim_SeqEndType0A",
    ),
    (
        0x0010_4cc8,
        rider_anim_play_event_sound,
        "RiderAnim_PlayEventSound",
    ),
    (0x0010_4de0, rider_anim_load_state, "RiderAnim_LoadState"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE00;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE01;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE02;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE03;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE04;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE05;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE06;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE07;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE08;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE09;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE0A;
}

// ---------------------------------------------------------------------------
// Direct-call targets (the generated bodies, which dispatch through their own
// override slot when one exists).

pub(crate) const BLEND_SEQ2: Callee = Callee {
    addr: 0x0010_3cc8,
    f: ssx3_ee::s00::f_00103cc8,
};

pub(crate) const BLEND_SEQ4: Callee = Callee {
    addr: 0x0010_3e28,
    f: ssx3_ee::s00::f_00103e28,
};

pub(crate) const SCRUB_SEQ: Callee = Callee {
    addr: 0x0010_3be0,
    f: ssx3_ee::s00::f_00103be0,
};

pub(crate) const PLAY_SEQ_THEN_LOOP: Callee = Callee {
    addr: 0x0010_3ff8,
    f: ssx3_ee::s00::f_00103ff8,
};

// ---------------------------------------------------------------------------
// Rider animation sequencing.

/// Blend prologue shared by the `RiderAnim_UpdateSeqTypeNN` selectors: load
/// the anim object's actor (+0x60), then its blend value at `actor + off` into
/// f13, negated (for one cycle) when `actor->0x320` is non-zero. Leaves
/// v1 = actor, v0 = the loaded word.
#[inline]
pub(crate) fn seq_prologue(c: &mut Cpu, blend_off: u32) {
    let actor = Addr(abi::arg(c, 0)).off(0x60).lw(c);
    c.g[V1][0] = actor;
    let blend = Addr(actor as u32).off(0x320).lw(c);
    c.g[V0][0] = blend;
    let raw = Addr(actor as u32).off(blend_off).read_fpu(c);
    let f13 = if blend != 0 {
        c.cycles += 1;
        -raw
    } else {
        raw
    };
    abi::set_fpr(c, 13, f13);
}

/// `void RiderAnim_UpdateSeqType03(RiderAnim *self, void *a1, int *seq)`
/// (0x104110): pick the blend arguments (a3/t0/t1) for sequence types 7 and
/// 0xa, then `RiderAnim_BlendSeq2`.
pub fn rider_anim_update_seq_type03(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        seq_prologue(c, 0x1fc);
        let seq = Addr(abi::arg(c, 2)).lw(c);
        c.g[V1][0] = seq;
        c.cycles += 4;
        c.g[V0][0] = 0xa;
        if seq == 0x7 {
            c.cycles += 4;
            c.g[A3][0] = 0x11;
            c.g[T0][0] = 0xf;
            c.g[T1][0] = 0x10;
        } else if seq == 0xa {
            c.cycles += 4;
            c.g[A3][0] = 0x1a;
            c.g[T0][0] = 0x17;
            c.g[T1][0] = 0x19;
        } else {
            c.cycles += 5;
            c.g[A3][0] = 0x1a;
            c.g[T0][0] = 0x18;
            c.g[T1][0] = 0x19;
        }
        c.cycles += 5;
        BLEND_SEQ2.call(c, 0x0010_416c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderAnim_UpdateSeqType04(RiderAnim *self, void *a1, int *seq)`
/// (0x104178): pick a3/t0..t3 for sequence types 6, 8, 0xe and 0xf, then
/// `RiderAnim_BlendSeq4`.
pub fn rider_anim_update_seq_type04(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        seq_prologue(c, 0x1fc);
        let seq = Addr(abi::arg(c, 2)).lw(c);
        c.g[V1][0] = seq;
        c.cycles += 4;
        if seq == 0x6 {
            c.cycles += 6;
            c.g[V0][0] = 0x8;
            c.g[A3][0] = 0xe;
            c.g[T0][0] = 0xd;
            c.g[T1][0] = 0xa;
            c.g[T2][0] = 0xb;
            c.g[T3][0] = 0xc;
        } else {
            c.cycles += 2;
            if seq == 0x8 {
                c.cycles += 6;
                c.g[V0][0] = 0xe;
                c.g[A3][0] = 0x16;
                c.g[T0][0] = 0x15;
                c.g[T1][0] = 0x12;
                c.g[T2][0] = 0x13;
                c.g[T3][0] = 0x14;
            } else {
                c.cycles += 2;
                if seq == 0xe {
                    c.cycles += 4;
                    c.g[V0][0] = 0xf;
                    c.g[A3][0] = 0x20;
                    c.g[T0][0] = 0x1f;
                    c.g[T1][0] = 0x1b;
                    c.cycles += 2;
                    c.g[T2][0] = 0x1d;
                    c.g[T3][0] = 0x1e;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0xf;
                    if seq == 0xf {
                        c.g[A3][0] = 0x20;
                        c.g[T0][0] = 0x1f;
                        c.g[T1][0] = 0x1c;
                        c.cycles += 3;
                        c.cycles += 2;
                        c.g[T2][0] = 0x1d;
                        c.g[T3][0] = 0x1e;
                    } else {
                        c.g[A3][0] = 0x9;
                        c.cycles += 5;
                        c.g[T0][0] = 0x8;
                        c.g[T1][0] = 0x5;
                        c.g[T2][0] = 0x6;
                        c.g[T3][0] = 0x7;
                    }
                }
            }
        }
        c.cycles += 5;
        BLEND_SEQ4.call(c, 0x0010_422c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderAnim_UpdateSeqType05(RiderAnim *self, void *a1, int *seq)`
/// (0x104238): blend value from `actor->0x238`; pick a3/t0/t1 for sequence
/// types 0x12 and 0x13, then `RiderAnim_BlendSeq2`.
pub fn rider_anim_update_seq_type05(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        seq_prologue(c, 0x238);
        let seq = Addr(abi::arg(c, 2)).lw(c);
        c.g[V1][0] = seq;
        c.cycles += 4;
        c.g[V0][0] = 0x13;
        if seq == 0x12 {
            c.cycles += 4;
            c.g[A3][0] = 0x41;
            c.g[T0][0] = 0x3f;
            c.g[T1][0] = 0x40;
        } else if seq == 0x13 {
            c.cycles += 5;
            c.g[A3][0] = 0x44;
            c.g[T0][0] = 0x42;
            c.g[T1][0] = 0x43;
        } else {
            c.cycles += 5;
            c.g[A3][0] = 0x47;
            c.g[T0][0] = 0x45;
            c.g[T1][0] = 0x46;
        }
        c.cycles += 5;
        BLEND_SEQ2.call(c, 0x0010_429c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderAnim_UpdateSeqType06(RiderAnim *self, void *a1, int *seq)`
/// (0x1042a8): blend value from `actor->0x214`, negated when `actor->0x320`
/// is zero, then `RiderAnim_BlendSeq2` with a3=0x24, t0=0x21, t1=0x22.
pub fn rider_anim_update_seq_type06(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let actor = Addr(abi::arg(c, 0)).off(0x60).lw(c);
        c.g[V1][0] = actor;
        let blend = Addr(actor as u32).off(0x320).lw(c);
        c.g[V0][0] = blend;
        let raw = Addr(actor as u32).off(0x214).read_fpu(c);
        let f13 = if blend == 0 {
            c.cycles += 1;
            -raw
        } else {
            raw
        };
        abi::set_fpr(c, 13, f13);
        c.cycles += 7;
        c.g[A3][0] = 0x24;
        c.g[T0][0] = 0x21;
        c.g[T1][0] = 0x22;
        BLEND_SEQ2.call(c, 0x0010_42d4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderAnim_UpdateSeqType0A(RiderAnim *self, void *a1, int *seq)`
/// (0x1042e0): choose a3/t0 for sequence types 0xda, 0xe2 and 0xea, load f13
/// from the actor's +0x238, then `RiderAnim_ScrubSeq`.
pub fn rider_anim_update_seq_type0a(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        let seq = Addr(abi::arg(c, 2)).lw(c);
        c.g[V1][0] = seq;
        c.g[V0][0] = 0xda;
        frame.save_ra(c, 0);
        let (extra, a3, t0) = if seq == 0xda {
            (4, 0x16a, 0x16b)
        } else if seq == 0xe2 {
            (7, 0x173, 0x174)
        } else if seq == 0xea {
            (8, 0x17c, 0x17d)
        } else {
            (8, 0x185, 0x186)
        };
        c.cycles += extra;
        let actor = Addr(abi::arg(c, 0)).off(0x60).lw(c);
        c.g[V0][0] = actor;
        c.g[A3][0] = a3;
        c.g[T0][0] = t0;
        c.cycles += 5;
        let f13 = Addr(actor as u32).off(0x238).read_fpu(c);
        abi::set_fpr(c, 13, f13);
        SCRUB_SEQ.call(c, 0x0010_4348)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderAnim_UpdateSeqType0C(RiderAnim *self, void *a1, int *seq)`
/// (0x1045b8): play sequence 0x89 then loop 0x8a
/// (`RiderAnim_PlaySeqThenLoop`).
pub fn rider_anim_update_seq_type0c(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = 0x89;
        frame.save_ra(c, 0);
        c.g[T0][0] = 0x8a;
        PLAY_SEQ_THEN_LOOP.call(c, 0x0010_45cc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderAnim_UpdateSeqType09(RiderAnim *self, void *a1, int *seq)`
/// (0x104660): blend value from `actor->0x244`; pick a3/t0 for sequence type
/// 0x2d, then `RiderAnim_ScrubSeq` with a0 = self.
pub fn rider_anim_update_seq_type09(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = 0x2d;
        let seq = Addr(abi::arg(c, 2)).lw(c);
        c.g[A0][0] = seq;
        let actor = Addr(c.g[A3][0] as u32).off(0x60).lw(c);
        c.g[V0][0] = actor;
        let f13 = Addr(actor as u32).off(0x244).read_fpu(c);
        abi::set_fpr(c, 13, f13);
        let (extra, t0, a3) = if seq == 0x2d {
            (3, 0x160, 0x161)
        } else {
            (4, 0x15a, 0x15b)
        };
        c.cycles += extra;
        c.g[A0][0] = c.g[A3][0];
        c.g[T0][0] = t0;
        c.g[A3][0] = a3;
        c.cycles += 5;
        SCRUB_SEQ.call(c, 0x0010_46a4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderAnim_UpdateSeqType0F(RiderAnim *self, void *a1, int *seq)`
/// (0x1046b0): f13 = min(2*x - 1, 1) clamped from below at -1 for
/// `x = actor->0x274`; pick a3/t0/t1 for sequence type 0x1c, then
/// `RiderAnim_BlendSeq2`.
pub fn rider_anim_update_seq_type0f(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        abi::set_fpr(c, 2, FpuF32(0x3f80_0000));
        frame.save_ra(c, 0);
        abi::set_fpr(c, 1, FpuF32(0xbf80_0000));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        let actor = Addr(abi::arg(c, 0)).off(0x60).lw(c);
        c.g[V0][0] = actor;
        let one = FpuF32(0x3f80_0000);
        let neg_one = FpuF32(0xbf80_0000);
        let x = Addr(actor as u32).off(0x274).read_fpu(c);
        let f0 = x + x - one;
        abi::set_fpr(c, 0, f0);
        let le = neg_one.le(f0);
        abi::set_fcc(c, le);
        let f13 = if le {
            c.cycles += 1;
            f0.min(one)
        } else {
            neg_one
        };
        abi::set_fpr(c, 13, f13);
        let seq = Addr(abi::arg(c, 2)).lw(c);
        c.g[V1][0] = seq;
        c.cycles += 4;
        c.g[V0][0] = 0x1c;
        if seq == 0x1c {
            c.cycles += 3;
            c.g[A3][0] = 0x31;
            c.g[T0][0] = 0x30;
            c.g[T1][0] = 0x32;
        } else {
            c.cycles += 3;
            c.g[A3][0] = 0x3c;
            c.g[T0][0] = 0x3a;
            c.g[T1][0] = 0x3b;
        }
        c.cycles += 5;
        BLEND_SEQ2.call(c, 0x0010_471c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider animation object.

/// `void RiderAnim_SetRider(RiderAnim *self, Rider *rider)` (0x104d38): clear
/// `self->+0x64`, set `self->rider` (+0x60), copy `rider->+0x780` to
/// `self->+0x54`.
pub fn rider_anim_set_rider(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    let rider = c.g[A1][0];
    this.off(0x64).sw(c, 0);
    this.off(0x60).sw(c, rider);
    c.g[V0][0] = Addr(rider as u32).off(0x780).lw(c);
    let ret = c.g[RA][0] as u32;
    this.off(0x54).sw(c, c.g[V0][0]);
    c.pc = ret;
}

/// `void RiderAnim_SetActor(RiderAnim *self, Actor *actor)` (0x104d50): set
/// `self->actor` (+0x64), clear `self->rider` (+0x60), copy `actor->+0x8` to
/// `self->+0x54`.
pub fn rider_anim_set_actor(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    let actor = c.g[A1][0];
    this.off(0x64).sw(c, actor);
    this.off(0x60).sw(c, 0);
    c.g[V0][0] = Addr(actor as u32).off(0x8).lw(c);
    let ret = c.g[RA][0] as u32;
    this.off(0x54).sw(c, c.g[V0][0]);
    c.pc = ret;
}

/// `call_indirect` targets the generated code reaches through the function
/// table (cross-shard `jalr`/`call_indirect`).
pub(crate) const ANIM_TABLE_GET_ENTRY_ADDR: u32 = 0x0031_15c8;

// ---------------------------------------------------------------------------
// RiderAnim_v03: the rider animation sequencer's per-sequence-end step. The
// sequence-end kind (0..10) selects one of ten handlers through a `.rodata`
// table, then the handler's return value is stored into the rider's animation
// slot array.

/// The ten sequence-end handlers and the return address each call resumes at.
pub(crate) const RIDER_ANIM_END_HANDLERS: [(u32, u32, Callee); 10] = [
    (0x0010_3978, 0x0010_3988, callee::RIDER_ANIM_SEQ_END_TYPE00),
    (0x0010_3990, 0x0010_39a0, callee::RIDER_ANIM_SEQ_END_TYPE01),
    (0x0010_39a8, 0x0010_39b8, callee::RIDER_ANIM_SEQ_END_TYPE02),
    (0x0010_39c0, 0x0010_39d0, callee::RIDER_ANIM_SEQ_END_TYPE03),
    (0x0010_39d8, 0x0010_39e8, callee::RIDER_ANIM_SEQ_END_TYPE04),
    (0x0010_39f0, 0x0010_3a00, callee::RIDER_ANIM_SEQ_END_TYPE05),
    (0x0010_3a08, 0x0010_3a18, callee::RIDER_ANIM_SEQ_END_TYPE06),
    (0x0010_3a20, 0x0010_3a30, callee::RIDER_ANIM_SEQ_END_TYPE07),
    (0x0010_3a38, 0x0010_3a48, callee::RIDER_ANIM_SEQ_END_TYPE08),
    (0x0010_3a50, 0x0010_3a60, callee::RIDER_ANIM_SEQ_END_TYPE09),
];

/// `void RiderAnim_v03(RiderAnim* anim, int seqEnd, int* out)`
/// (0x103918): on a sequence end, look the sequence up in the animation table
/// (0x446990, 28-byte entries), run that end kind's handler and write its
/// result to `out[0]`. The port accepts all `__entries` resume labels; the
/// sequence-end table entry that points back at the function loops.
pub fn rider_anim_v03(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x0010_3918 => {
                    c.cycles += 18;
                    let frame = Frame::open(c, 0x50);
                    frame.save(c, 16, 0x40);
                    frame.save(c, 17, 0x30);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, 18, 0x20);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, 19, 0x10);
                    c.g[S2][0] = c.g[A2][0];
                    frame.save_ra(c, 0);
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x50).lw(c);
                    c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[S3][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                    call_ptr(c, ANIM_TABLE_GET_ENTRY_ADDR, 0x0010_3950)?;
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
                    c.g[V0][0] = (c.g[V1][0] < 0xbu64) as u64;
                    let out_of_range = c.g[V0][0] == 0;
                    // The generated code loads the table base into v0 before
                    // branching (the dispatch block below adds 0x6990 to it).
                    c.g[V0][0] = 0x45_0000;
                    pc = if out_of_range {
                        0x0010_3a84
                    } else {
                        0x0010_3960
                    };
                }
                0x0010_3960 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6990));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    if target == 0x0010_3918 {
                        if back_edge_unwinds(c, 0x0010_3918) {
                            return Ok(());
                        }
                        pc = 0x0010_3918;
                    } else if let Some(k) =
                        RIDER_ANIM_END_HANDLERS.iter().position(|h| h.0 == target)
                    {
                        pc = RIDER_ANIM_END_HANDLERS[k].0;
                    } else if target == 0x0010_3a68 {
                        pc = 0x0010_3a68;
                    } else {
                        c.jump_out(target, 0x0010_3918, 0x0010_3aa0);
                        return Ok(());
                    }
                }
                0x0010_3978 | 0x0010_3990 | 0x0010_39a8 | 0x0010_39c0 | 0x0010_39d8
                | 0x0010_39f0 | 0x0010_3a08 | 0x0010_3a20 | 0x0010_3a38 | 0x0010_3a50 => {
                    c.cycles += 6;
                    let k = RIDER_ANIM_END_HANDLERS
                        .iter()
                        .position(|h| h.0 == pc)
                        .unwrap();
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S1][0];
                    let (_, ret, f) = RIDER_ANIM_END_HANDLERS[k];
                    f.call(c, ret)?;
                    c.g[V1][0] = sx((c.g[S3][0] as u32) << 2);
                    pc = 0x0010_3a7c;
                }
                0x0010_3a68 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S1][0];
                    callee::RIDER_ANIM_SEQ_END_TYPE0A.call(c, 0x0010_3a78)?;
                    c.g[V1][0] = sx((c.g[S3][0] as u32) << 2);
                    pc = 0x0010_3a7c;
                }
                0x0010_3a7c => {
                    c.cycles += 2;
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let a = c.g[V1][0] as u32;
                    let v = c.g[V0][0];
                    Addr(a).sw(c, v);
                    pc = 0x0010_3a84;
                }
                0x0010_3a84 => {
                    c.cycles += 7;
                    let frame = Frame { size: 0x50 };
                    frame.restore(c, 16, 0x40);
                    frame.restore(c, 17, 0x30);
                    frame.restore(c, 18, 0x20);
                    frame.restore(c, 19, 0x10);
                    frame.restore_ra(c, 0);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0010_3918, 0x0010_3aa0);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Direct-call targets (the generated bodies, which dispatch through their own
// override slot when one exists).

pub(crate) const ANIM_TABLE_GET_ENTRY: Callee = Callee {
    addr: 0x0031_15c8,
    f: ee::s10::AnimTable_GetEntry,
};

pub(crate) const FIRE_SOUND_EVENTS: Callee = Callee {
    addr: 0x0010_3aa0,
    f: ee::s00::RiderAnim_FireSoundEvents,
};

/// The sixteen per-type sequence updates, in sequence-type order.
pub(crate) const SEQ_TYPES: [Callee; 16] = [
    Callee {
        addr: 0x0010_48c0,
        f: ee::s00::RiderAnim_UpdateSeqType00,
    },
    Callee {
        addr: 0x0010_4940,
        f: ee::s00::RiderAnim_UpdateSeqType01,
    },
    Callee {
        addr: 0x0010_49c0,
        f: ee::s00::RiderAnim_UpdateSeqType02,
    },
    Callee {
        addr: 0x0010_4110,
        f: ee::s00::RiderAnim_UpdateSeqType03,
    },
    Callee {
        addr: 0x0010_4178,
        f: ee::s00::RiderAnim_UpdateSeqType04,
    },
    Callee {
        addr: 0x0010_4238,
        f: ee::s00::RiderAnim_UpdateSeqType05,
    },
    Callee {
        addr: 0x0010_42a8,
        f: ee::s00::RiderAnim_UpdateSeqType06,
    },
    Callee {
        addr: 0x0010_4358,
        f: ee::s00::RiderAnim_UpdateSeqType07,
    },
    Callee {
        addr: 0x0010_45d8,
        f: ee::s00::RiderAnim_UpdateSeqType08,
    },
    Callee {
        addr: 0x0010_4660,
        f: ee::s00::RiderAnim_UpdateSeqType09,
    },
    Callee {
        addr: 0x0010_42e0,
        f: ee::s00::RiderAnim_UpdateSeqType0A,
    },
    Callee {
        addr: 0x0010_43f8,
        f: ee::s00::RiderAnim_UpdateSeqType0B,
    },
    Callee {
        addr: 0x0010_45b8,
        f: ee::s00::RiderAnim_UpdateSeqType0C,
    },
    Callee {
        addr: 0x0010_47f0,
        f: ee::s00::RiderAnim_UpdateSeqType0D,
    },
    Callee {
        addr: 0x0010_4728,
        f: ee::s00::RiderAnim_UpdateSeqType0E,
    },
    Callee {
        addr: 0x0010_46b0,
        f: ee::s00::RiderAnim_UpdateSeqType0F,
    },
];

/// Return addresses of the sixteen type calls (they differ by two
/// instructions, and all land on the shared tail at `0x1038bc`).
pub(crate) const SEQ_RETS: [u32; 16] = [
    0x0010_3714,
    0x0010_3730,
    0x0010_374c,
    0x0010_3768,
    0x0010_3784,
    0x0010_37a0,
    0x0010_37bc,
    0x0010_37d8,
    0x0010_37f4,
    0x0010_3810,
    0x0010_382c,
    0x0010_3848,
    0x0010_3864,
    0x0010_3880,
    0x0010_389c,
    0x0010_38b8,
];

/// `int RiderAnim_UpdateSequence(RiderAnim *anim, float frame, AnimSequence *seq)`
/// (0x1036a0): look the sequence's animation type up in the animation table
/// and run the matching per-type update; the returned flag (non-zero when the
/// sequence finished) is kept in `s1` and the tail fires the sequence's sound
/// events when it is clear and the sequence's owner has sounds pending.
///
/// Also accepts every resume entry of the generated `__entries` twin: the
/// trampoline can re-enter this function at each type call's return address
/// (and at the shared tail blocks) after an event unwinds a callee.
pub fn rider_anim_update_sequence(c: &mut Cpu) {
    let mut pc = c.pc as u32;
    loop {
        match pc {
            // Entry: frame, then the animation-table lookup.
            0x0010_36a0 => {
                c.cycles += 18;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                let sp = c.g[SP][0] as u32;
                c.sq(16, sp.wrapping_add(0x40));
                c.sq(17, sp.wrapping_add(0x30));
                c.g[S0][0] = c.g[A1][0];
                c.sq(18, sp.wrapping_add(0x20));
                c.g[S1][0] = 0;
                c.sq(19, sp.wrapping_add(0x10));
                c.g[S2][0] = c.g[A0][0];
                c.swc1(20, sp.wrapping_add(0x50));
                c.g[S3][0] = c.g[A2][0];
                c.sd(sp, c.g[RA][0]);
                c.mov_s(20, 12);
                c.g[A0][0] = Addr(c.g[S3][0] as u32).lw(c);
                if ANIM_TABLE_GET_ENTRY.call(c, 0x0010_36d8).is_err() {
                    return;
                }
                let ty = Addr(c.g[V0][0] as u32).off(4).lw(c);
                c.g[V1][0] = ty;
                c.g[V0][0] = (ty < 0x10) as u64;
                let known = ty < 0x10;
                c.g[V0][0] = 0x45_0000;
                pc = if known { 0x0010_36e8 } else { 0x0010_38bc };
            }
            // Animation-type dispatch through the jump table at 0x456950.
            0x0010_36e8 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6950));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let target = c.g[A0][0] as u32;
                pc = match target {
                    0x0010_36a0 => 0x0010_36a0,
                    0x0010_3700 | 0x0010_371c | 0x0010_3738 | 0x0010_3754 | 0x0010_3770
                    | 0x0010_378c | 0x0010_37a8 | 0x0010_37c4 | 0x0010_37e0 | 0x0010_37fc
                    | 0x0010_3818 | 0x0010_3834 | 0x0010_3850 | 0x0010_386c | 0x0010_3888
                    | 0x0010_38a4 => target,
                    _ => {
                        c.jump_out(target, 0x0010_36a0, 0x0010_3914);
                        return;
                    }
                };
            }
            // The sixteen per-type updates; only the charge and return address
            // differ (the last one is a six-cycle block).
            0x0010_3700 | 0x0010_371c | 0x0010_3738 | 0x0010_3754 | 0x0010_3770 | 0x0010_378c
            | 0x0010_37a8 | 0x0010_37c4 | 0x0010_37e0 | 0x0010_37fc | 0x0010_3818 | 0x0010_3834
            | 0x0010_3850 | 0x0010_386c | 0x0010_3888 => {
                let kind = ((pc - 0x0010_3700) / 0x1c) as usize;
                c.cycles += 7;
                c.g[A1][0] = c.g[S0][0];
                c.mov_s(12, 20);
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S3][0];
                if SEQ_TYPES[kind].call(c, SEQ_RETS[kind]).is_err() {
                    return;
                }
                c.g[S1][0] = c.g[V0][0];
                pc = 0x0010_38bc;
            }
            0x0010_38a4 => {
                c.cycles += 6;
                c.g[A1][0] = c.g[S0][0];
                c.mov_s(12, 20);
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S3][0];
                if SEQ_TYPES[15].call(c, SEQ_RETS[15]).is_err() {
                    return;
                }
                c.g[S1][0] = c.g[V0][0];
                pc = 0x0010_38bc;
            }
            // Shared tail: fire sound events if the update asked for them.
            0x0010_38bc => {
                c.cycles += 2;
                let finished = c.g[S1][0] != 0;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                pc = if finished { 0x0010_38f8 } else { 0x0010_38c4 };
            }
            0x0010_38c4 => {
                c.cycles += 3;
                let owner = Addr(c.g[S2][0] as u32).off(0x60).lw(c);
                c.g[V1][0] = owner;
                if owner == 0 {
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    pc = 0x0010_38fc;
                } else {
                    pc = 0x0010_38d0;
                }
            }
            0x0010_38d0 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x874).lw(c);
                let no_pending = c.g[V0][0] == 0;
                c.g[A0][0] = 0;
                pc = if no_pending { 0x0010_38e4 } else { 0x0010_38dc };
            }
            0x0010_38dc => {
                c.cycles += 2;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x87c).lw(c);
                c.g[A0][0] = (0u64 < c.g[V0][0]) as u64;
                pc = 0x0010_38e4;
            }
            0x0010_38e4 => {
                c.cycles += 2;
                let silent = c.g[A0][0] == 0;
                c.g[A0][0] = c.g[S2][0];
                pc = if silent { 0x0010_38f4 } else { 0x0010_38ec };
            }
            0x0010_38ec => {
                c.cycles += 2;
                c.g[A1][0] = c.g[S3][0];
                if FIRE_SOUND_EVENTS.call(c, 0x0010_38f4).is_err() {
                    return;
                }
                pc = 0x0010_38f4;
            }
            0x0010_38f4 => {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                pc = 0x0010_38f8;
            }
            0x0010_38f8 => {
                c.cycles += 1;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                pc = 0x0010_38fc;
            }
            // Epilogue.
            0x0010_38fc => {
                c.cycles += 6;
                let sp = c.g[SP][0] as u32;
                c.lq(18, sp.wrapping_add(0x20));
                c.lq(19, sp.wrapping_add(0x10));
                c.g[RA][0] = c.ld(sp);
                c.lwc1(20, sp.wrapping_add(0x50));
                let ra = c.g[RA][0] as u32;
                c.g[SP][0] = sx(sp.wrapping_add(0x60));
                c.pc = ra;
                return;
            }
            _ => {
                c.jump_out(pc, 0x0010_36a0, 0x0010_3914);
                return;
            }
        }
    }
}

// ------------------------------------------------ RiderAnim_UpdateSeqType02

/// `int RiderAnim_UpdateSeqType02(RiderAnim* anim, SeqList* list,
/// AnimSequence* seq, float dt)` (0x1049c0): a one-shot sequence. Advance
/// its track by `dt * (1/60)`; when the track has ended, or else when its
/// weight fade has finished, remove it from the list. Returns whether it
/// was removed.
pub fn rider_anim_seq_type02_update(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    /// `AnimSequence_AdvanceTrack(seq, 0, step)`: nonzero when it ended.
    const ADVANCE_TRACK: u32 = 0x0031_35b0;
    /// `AnimSequence_UpdateWeightFade(seq, step)`: nonzero when faded out.
    const UPDATE_WEIGHT_FADE: u32 = 0x0031_3800;
    /// `AnimSeqList_Remove(list, seq)`.
    const LIST_REMOVE: u32 = 0x0031_45f8;
    /// `gp-0x7f3c`: 1/60, ticks to seconds.
    const TICK_GP: u32 = 0x7f3c;
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
        call_ptr(c, ADVANCE_TRACK, 0x0010_49f4)?;
        let ended = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !ended {
            c.cycles += 3;
            c.f[12] = c.f[20];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, UPDATE_WEIGHT_FADE, 0x0010_4a08)?;
        }
        c.cycles += 2;
        let remove = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S1][0];
        if remove {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, LIST_REMOVE, 0x0010_4a18)?;
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

// ------------------------------------------------------- RiderAnim_BlendSeq4

/// `AnimSequence_SetTrackAnim(seq, track, anim)`.
pub(crate) const ANIM_SEQ_SET_TRACK_ANIM: u32 = 0x0031_3c50;

/// `AnimSequence_SetTrackTime(seq, track, time)`.
pub(crate) const ANIM_SEQ_SET_TRACK_TIME: u32 = 0x0031_3cf0;

/// `AnimSequence_SetTrackWeight(seq, track, weight)`.
pub(crate) const ANIM_SEQ_SET_TRACK_WEIGHT: u32 = 0x0031_3d28;

/// `AnimSequence_SetTrackLoop(seq, track, loop)`.
pub(crate) const ANIM_SEQ_SET_TRACK_LOOP: u32 = 0x0031_3d40;

/// `AnimSequence_AdvanceTrack(seq, track, step)`.
pub(crate) const ANIM_SEQ_ADVANCE_TRACK: u32 = 0x0031_35b0;

/// `AnimSequence_UpdateWeightFade(seq, step)`: nonzero when faded out.
pub(crate) const ANIM_SEQ_UPDATE_WEIGHT_FADE: u32 = 0x0031_3800;

/// `AnimSeqList_Remove(list, seq)`.
pub(crate) const ANIM_SEQ_LIST_REMOVE: u32 = 0x0031_45f8;

/// `int RiderAnim_BlendSeq4(RiderAnim* self, SeqList* list, AnimSequence*
/// seq, int a0, int a1, int a2, int a3, int a4, float dt, float v)`
/// (0x103e28): a five-key blend along `v` in [-1, 1] (e.g. lean or carve
/// amount). `v` picks the two neighbouring keys (`from`, `to`) of
/// `a0..a4` at -1, -0.5, 0, 0.5, 1 and the weight `w` of `from`. If track 0
/// does not already play `from`'s animation, switch it, keeping its
/// relative time. Track 1 plays `to` with weight `1 - w`, synchronised to
/// track 0's phase. Then advance by `dt / 60` and, when the weight fade
/// ends, remove the sequence from `list`. Returns whether it was removed.
pub fn rider_anim_blend_seq4(c: &mut Cpu) {
    /// `gp+0xd8c`: the rider animation bank (animation ids at +0x1030).
    const ANIM_BANK_GP: u32 = 0xd8c;
    const BANK_IDS: u32 = 0x1030;
    /// `gp-0x7f68`: 1/60, ticks to seconds.
    const TICK_GP: u32 = 0x7f68;
    /// AnimSequence fields: track 0's animation id, time, length; track 1's
    /// speed.
    const SEQ_ANIM0: u32 = 0x4;
    const SEQ_TIME0: u32 = 0x8;
    const SEQ_LENGTH0: u32 = 0x10;
    const SEQ_SPEED1: u32 = 0x2c;
    guest(c, |c| {
        c.cycles += 15;
        let half = FpuF32(0x3f00_0000);
        c.g[AT][0] = 0x3f00_0000;
        c.f[0] = half.0;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x30);
        let v = FpuF32(c.f[13]);
        let above_half = abi::c_lt_s(c, half, v);
        frame.save(c, S2, 0x10);
        frame.save_fpr(c, 22, 0x50);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.f[22] = c.f[12];
        frame.save_fpr(c, 21, 0x48);
        frame.save_fpr(c, 20, 0x40);
        // The key pair: `from` in t3, `to` in s1; the weight in f21.
        let w;
        if above_half {
            // (0.5, 1]: a3 -> a4, w = (v - 0.5) * 2 (from is already t3).
            c.cycles += 4;
            let d = v - half;
            c.f[0] = d.0;
            c.g[S1][0] = c.g[T2][0];
            w = d + d;
        } else {
            c.cycles += 5;
            c.f[0] = 0;
            let above_zero = abi::c_lt_s(c, FpuF32::ZERO, v);
            c.g[T3][0] = c.g[T2][0];
            if above_zero {
                // (0, 0.5]: a2 -> a3, w = v * 2.
                c.cycles += 3;
                c.g[S1][0] = c.g[T1][0];
                w = v + v;
            } else {
                c.cycles += 6;
                c.g[AT][0] = 0xffff_ffff_bf00_0000;
                let neg_half = FpuF32(0xbf00_0000);
                c.f[0] = neg_half.0;
                let above_neg_half = abi::c_lt_s(c, neg_half, v);
                c.g[T3][0] = c.g[T1][0];
                c.g[AT][0] = 0x3f80_0000;
                c.f[0] = FpuF32::ONE.0;
                if above_neg_half {
                    // (-0.5, 0]: a1 -> a2, w = v * 2 + 1.
                    c.cycles += 6;
                    let twice = v + v;
                    c.f[1] = twice.0;
                    c.g[S1][0] = c.g[T0][0];
                    w = twice + FpuF32::ONE;
                } else {
                    // [-1, -0.5]: a0 -> a1, w = (v + 1) * 2.
                    c.cycles += 6;
                    c.g[T3][0] = c.g[T0][0];
                    c.g[S1][0] = c.g[A3][0];
                    let d = v + FpuF32::ONE;
                    c.f[0] = d.0;
                    w = d + d;
                }
            }
        }
        c.f[21] = w.0;

        // Track 0 plays `from`: switch it unless it already does.
        c.cycles += 7;
        let seq = r32(c, S0);
        let bank = Addr(r32(c, GP).wrapping_add(ANIM_BANK_GP)).lw(c);
        c.g[V0][0] = bank;
        let entry = sx((r32(c, T3) << 2).wrapping_add(bank as u32));
        c.g[V1][0] = entry;
        c.g[A0][0] = Addr(seq + SEQ_ANIM0).lw(c);
        c.g[A2][0] = Addr((entry as u32).wrapping_add(BANK_IDS)).lw(c);
        let same = c.g[A2][0] == c.g[A0][0];
        c.g[V1][0] = Addr(r32(c, GP).wrapping_add(ANIM_BANK_GP)).lw(c);
        if !same {
            c.cycles += 12;
            let phase = Addr(seq + SEQ_TIME0)
                .read_fpu(c)
                .div(Addr(seq + SEQ_LENGTH0).read_fpu(c));
            c.f[0] = Addr(seq + SEQ_LENGTH0).read_fpu(c).0;
            c.f[20] = phase.0;
            c.g[A0][0] = sx(seq);
            c.g[A1][0] = 0;
            call_ptr(c, ANIM_SEQ_SET_TRACK_ANIM, 0x0010_3f0c)?;
            let time = Addr(seq + SEQ_LENGTH0).read_fpu(c) * FpuF32(c.f[20]);
            c.g[A0][0] = sx(seq);
            c.g[A1][0] = 0;
            c.f[12] = time.0;
            call_ptr(c, ANIM_SEQ_SET_TRACK_TIME, 0x0010_3f20)?;
            c.g[V1][0] = Addr(r32(c, GP).wrapping_add(ANIM_BANK_GP)).lw(c);
        }

        // Track 1 plays `to` with the complementary weight.
        c.cycles += 39;
        let to = sx((r32(c, S1) << 2).wrapping_add(r32(c, V1)));
        c.g[V0][0] = to;
        c.g[A0][0] = sx(seq);
        c.g[A1][0] = 1;
        c.g[A2][0] = Addr((to as u32).wrapping_add(BANK_IDS)).lw(c);
        call_ptr(c, ANIM_SEQ_SET_TRACK_ANIM, 0x0010_3f3c)?;
        c.f[12] = c.f[21];
        c.g[A0][0] = sx(seq);
        c.g[A1][0] = 0;
        call_ptr(c, ANIM_SEQ_SET_TRACK_WEIGHT, 0x0010_3f4c)?;
        c.g[AT][0] = 0x3f80_0000;
        c.g[A0][0] = sx(seq);
        c.g[A1][0] = 1;
        c.f[12] = (FpuF32::ONE - FpuF32(c.f[21])).0;
        call_ptr(c, ANIM_SEQ_SET_TRACK_WEIGHT, 0x0010_3f64)?;
        c.g[A2][0] = 1;
        c.g[A0][0] = sx(seq);
        c.g[A1][0] = 0;
        call_ptr(c, ANIM_SEQ_SET_TRACK_LOOP, 0x0010_3f74)?;

        // Advance by dt seconds.
        let tick = Addr(r32(c, GP).wrapping_sub(TICK_GP)).read_fpu(c);
        let step = FpuF32(c.f[22]) * tick;
        c.f[20] = step.0;
        c.g[A0][0] = sx(seq);
        c.g[A1][0] = 0;
        c.f[12] = step.0;
        call_ptr(c, ANIM_SEQ_ADVANCE_TRACK, 0x0010_3f8c)?;

        // Track 1 follows track 0's phase at its own speed.
        let time0 = Addr(seq + SEQ_TIME0).read_fpu(c);
        let length0 = Addr(seq + SEQ_LENGTH0).read_fpu(c);
        let speed1 = Addr(seq + SEQ_SPEED1).read_fpu(c);
        c.f[1] = length0.0;
        c.f[0] = speed1.0;
        c.g[A0][0] = sx(seq);
        c.g[A1][0] = 1;
        c.f[12] = (time0.div(length0) * speed1).0;
        call_ptr(c, ANIM_SEQ_SET_TRACK_TIME, 0x0010_3fac)?;
        c.f[12] = c.f[20];
        c.g[A0][0] = sx(seq);
        call_ptr(c, ANIM_SEQ_UPDATE_WEIGHT_FADE, 0x0010_3fb8)?;
        let faded = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S2][0];
        if faded {
            c.cycles += 3;
            c.g[A1][0] = sx(seq);
            call_ptr(c, ANIM_SEQ_LIST_REMOVE, 0x0010_3fd0)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x50);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Rider animation events.

/// `AnimBankSet* g_animBanks` (`gp+0xd08`): bank pointers by `id & 0xff`. A
/// bank: `+8` its sequences (0x14 bytes; `+0x10` first event, `+0x12` event
/// count), `+0x10` its events (4 bytes; `+2` the event code, bit 15 = sound).
pub(crate) const ANIM_BANKS: u32 = 0xd08;

/// The animation object (`a1`): `+4` its sequence id (bank | seq << 8),
/// `+0xb0` the per-event flags (two 64-bit masks: reached, and armed).
pub(crate) const ANIM_SEQ_ID: u32 = 0x4;

pub(crate) const ANIM_EVENT_FLAGS: u32 = 0xb0;

pub(crate) const SEQ_SIZE: u64 = 0x14;

pub(crate) const FLAGS64_CLEAR: u32 = 0x0014_4670;

pub(crate) const FLAGS64_TEST: u32 = 0x0014_46a0;

pub(crate) const FLAGS64_TEST_SECOND: u32 = 0x0014_46b8;

pub(crate) const RIDER_ANIM_PLAY_EVENT_SOUND: Callee = Callee {
    addr: 0x0010_4cc8,
    f: ee::s00::f_00104cc8,
};

/// `void RiderAnim_FireSoundEvents(RiderAnim* self, Anim* anim)` (0x103aa0):
/// for each event of the animation's current sequence that is set in both
/// event masks and whose code has the sound bit, clear its flag and play the
/// event sound (`code & 0x7fff`).
pub fn rider_anim_fire_sound_events(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x90);
        c.g[A2][0] = SEQ_SIZE;
        frame.save(c, S1, 0x70);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x80);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save_ra(c, 0);
        let anim = r32(c, S4);
        let gp = c.g[GP][0] as u32;
        // The sequence's event count.
        c.g[A1][0] = Addr(gp + ANIM_BANKS).lw(c);
        c.g[V0][0] = Addr(anim + ANIM_SEQ_ID).lw(c);
        c.g[V1][0] = sx(((c.g[V0][0] & 0xff) as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
        c.g[A1][0] = add32(c.g[A1][0], c.g[V1][0]);
        c.mult(0, V0, V0, A2);
        c.g[A0][0] = Addr(r32(c, A1)).lw(c);
        c.g[V1][0] = Addr(r32(c, A0) + 8).lw(c);
        c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
        c.g[S3][0] = Addr(r32(c, V0) + 0x12).lhu(c);
        c.g[S1][0] = 0;
        if c.g[S3][0] != 0 {
            c.cycles += 4;
            c.g[S2][0] = sx(anim + ANIM_EVENT_FLAGS);
            c.g[S7][0] = 1;
            c.g[S6][0] = SEQ_SIZE;
            c.g[S0][0] = 0;
            loop {
                // Head 0x103b18: s1 = event index, s0 = 0.
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, FLAGS64_TEST, 0x0010_3b24)?;
                let reached = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S2][0];
                if reached {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, FLAGS64_TEST_SECOND, 0x0010_3b34)?;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.g[S7][0];
                    }
                }
                c.cycles += 2;
                let fire = c.g[S0][0] != 0;
                c.g[A1][0] = Addr(gp + ANIM_BANKS).lw(c);
                let mut played = !fire;
                if fire {
                    // The event's code.
                    c.cycles += 18;
                    c.g[V1][0] = Addr(anim + ANIM_SEQ_ID).lw(c);
                    c.g[V0][0] = sx(((c.g[V1][0] & 0xff) as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 8);
                    c.g[A1][0] = add32(c.g[A1][0], c.g[V0][0]);
                    c.mult(0, V1, V1, S6);
                    c.g[A0][0] = Addr(r32(c, A1)).lw(c);
                    c.g[A2][0] = Addr(r32(c, A0) + 8).lw(c);
                    c.g[A1][0] = Addr(r32(c, A0) + 0x10).lw(c);
                    c.g[V1][0] = add32(c.g[V1][0], c.g[A2][0]);
                    c.g[V0][0] = Addr(r32(c, V1) + 0x10).lhu(c);
                    c.g[V0][0] = add32(c.g[V0][0], c.g[S1][0]);
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = add32(c.g[V0][0], c.g[A1][0]);
                    c.g[S0][0] = Addr(r32(c, V0) + 2).lhu(c);
                    c.g[V1][0] = c.g[S0][0] & 0x8000;
                    if c.g[V1][0] != 0 {
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, FLAGS64_CLEAR, 0x0010_3b94)?;
                        c.g[A1][0] = c.g[S0][0] & 0x7fff;
                        c.g[A0][0] = c.g[S5][0];
                        RIDER_ANIM_PLAY_EVENT_SOUND.call(c, 0x0010_3ba0)?;
                        played = true;
                    }
                }
                if played {
                    c.cycles += 1;
                }
                c.g[S1][0] = add32(c.g[S1][0], 1);
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = 0;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_3b18);
                }
            }
        }
        c.cycles += 11;
        for (reg, off) in [
            (S0, 0x80),
            (S1, 0x70),
            (S2, 0x60),
            (S3, 0x50),
            (S4, 0x40),
            (S5, 0x30),
            (S6, 0x20),
            (S7, 0x10),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
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

// ---------------------------------------------------------------------------
// RiderAnim_BlendSeq2 (0x103cc8)

/// `gp+0xd8c`: the rider animation bank (animation ids at +0x1030).
pub(crate) const ANIM_BANK_GP: u32 = 0xd8c;

pub(crate) const BANK_IDS: u32 = 0x1030;

/// `gp-0x7f6c`: 1/60, ticks to seconds (`.lit4`).
pub(crate) const SEQ2_TICK_GP: u32 = 0x7f6c;

/// AnimSequence: track 0's time and length, track 1's speed.
pub(crate) const SEQ_TIME0: u32 = 0x8;

pub(crate) const SEQ_LENGTH0: u32 = 0x10;

pub(crate) const SEQ_SPEED1: u32 = 0x2c;

/// AnimSequence methods (through the function table).
pub(crate) const ANIM_SEQ_SET_TRACK_ANIM_2: u32 = 0x0031_3c50;

pub(crate) const ANIM_SEQ_SET_TRACK_TIME_2: u32 = 0x0031_3cf0;

pub(crate) const ANIM_SEQ_SET_TRACK_WEIGHT_2: u32 = 0x0031_3d28;

pub(crate) const ANIM_SEQ_SET_TRACK_LOOP_2: u32 = 0x0031_3d40;

pub(crate) const ANIM_SEQ_ADVANCE_TRACK_2: u32 = 0x0031_35b0;

pub(crate) const ANIM_SEQ_UPDATE_WEIGHT_FADE_2: u32 = 0x0031_3800;

pub(crate) const ANIM_SEQ_LIST_REMOVE_2: u32 = 0x0031_45f8;

/// `int RiderAnim_BlendSeq2(Rider* r, AnimList* list, AnimSequence* seq,
/// int key_neg, int key_mid, float dt, float v, int key_pos)` (0x103cc8):
/// a two-track blend: track 0 plays `key_mid`, track 1 plays `key_pos` for
/// `v > 0` or `key_neg` otherwise, weighted `1 - |v|` and `|v|`; track 0
/// loops and advances by `dt` seconds, track 1 follows its phase at its own
/// speed. When the weight fade finishes the sequence leaves `list`
/// (returns 1), else 0. (`a3` key_neg, `t0` key_mid, `t1` key_pos, `f12`
/// dt, `f13` v.)
pub fn rider_anim_blend_seq2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x60);
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(ANIM_BANK_GP)).lw(c);
        frame.save(c, S0, 0x40);
        c.g[T0][0] = sx(r32(c, T0) << 2);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[T0][0] = sx(r32(c, T0).wrapping_add(r32(c, V0)));
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A3][0];
        frame.save_fpr(c, 21, 0x58);
        c.g[S3][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x50);
        c.f[21] = c.f[12];
        frame.save_ra(c, 0);
        c.f[20] = c.f[13];
        c.g[S1][0] = c.g[T1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = Addr(r32(c, T0).wrapping_add(BANK_IDS)).lw(c);
        c.g[A1][0] = 0;
        call_ptr(c, ANIM_SEQ_SET_TRACK_ANIM_2, 0x0010_3d1c)?;

        // Track 1: key_pos for v > 0, else key_neg with v negated.
        c.f[0] = 0;
        let v = FpuF32(c.f[20]);
        let positive = abi::c_lt_s(c, FpuF32::ZERO, v);
        let gp = r32(c, GP);
        c.g[V1][0] = Addr(gp.wrapping_add(ANIM_BANK_GP)).lw(c);
        let ret = if positive {
            c.cycles += 8;
            c.g[V0][0] = sx(r32(c, S1) << 2);
            0x0010_3d48
        } else {
            c.cycles += 7;
            c.g[V0][0] = sx(r32(c, S2) << 2);
            c.f[20] = (-FpuF32(c.f[20])).0;
            0x0010_3d6c
        };
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[A1][0] = 1;
        c.g[A2][0] = Addr(r32(c, V0).wrapping_add(BANK_IDS)).lw(c);
        call_ptr(c, ANIM_SEQ_SET_TRACK_ANIM_2, ret)?;

        // Weights 1 - w and w, track 0 loops.
        c.cycles += 33;
        c.g[AT][0] = 0x3f80_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.f[12] = (FpuF32::ONE - FpuF32(c.f[20])).0;
        call_ptr(c, ANIM_SEQ_SET_TRACK_WEIGHT_2, 0x0010_3d84)?;
        c.f[12] = c.f[20];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, ANIM_SEQ_SET_TRACK_WEIGHT_2, 0x0010_3d94)?;
        c.g[A2][0] = 1;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, ANIM_SEQ_SET_TRACK_LOOP_2, 0x0010_3da4)?;

        // Advance track 0 by dt seconds.
        let gp = r32(c, GP);
        let tick = Addr(gp.wrapping_sub(SEQ2_TICK_GP)).read_fpu(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        let step = FpuF32(c.f[21]) * tick;
        c.f[20] = step.0;
        c.f[12] = step.0;
        call_ptr(c, ANIM_SEQ_ADVANCE_TRACK_2, 0x0010_3dbc)?;

        // Track 1 follows track 0's phase at its own speed.
        let seq = r32(c, S0);
        let time0 = Addr(seq.wrapping_add(SEQ_TIME0)).read_fpu(c);
        c.g[A0][0] = c.g[S0][0];
        let length0 = Addr(seq.wrapping_add(SEQ_LENGTH0)).read_fpu(c);
        c.f[1] = length0.0;
        c.g[A1][0] = 1;
        let speed1 = Addr(seq.wrapping_add(SEQ_SPEED1)).read_fpu(c);
        c.f[0] = speed1.0;
        c.f[12] = (time0.div(length0) * speed1).0;
        call_ptr(c, ANIM_SEQ_SET_TRACK_TIME_2, 0x0010_3ddc)?;
        c.f[12] = c.f[20];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, ANIM_SEQ_UPDATE_WEIGHT_FADE_2, 0x0010_3de8)?;
        let faded = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S3][0];
        if faded {
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, ANIM_SEQ_LIST_REMOVE_2, 0x0010_3e00)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x58);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderAnim_UpdateSeqType01 (0x104940)

/// `AnimSequence_AdvanceTrack(seq, 0, step)`: nonzero when the track ended.
pub(crate) const SEQ01_ADVANCE_TRACK: u32 = 0x0031_35b0;

/// `AnimSequence_UpdateWeightFade(seq, step)`: nonzero when faded out.
pub(crate) const SEQ01_UPDATE_WEIGHT_FADE: u32 = 0x0031_3800;

/// `AnimSeqList_Remove(list, seq)`.
pub(crate) const SEQ01_LIST_REMOVE: u32 = 0x0031_45f8;

/// `gp-0x7f40`: 1/60, ticks to seconds (this type's own `.lit4` entry).
pub(crate) const SEQ01_TICK_GP: u32 = 0x7f40;

/// `int RiderAnim_UpdateSeqType01(RiderAnim* anim, SeqList* list,
/// AnimSequence* seq, float dt)` (0x104940): a one-shot sequence. Advance
/// its track by `dt * (1/60)`; when the track has ended, or else when its
/// weight fade has finished, remove it from the list. Returns whether it
/// was removed. (Same shape as type 02, with its own step constant.)
pub fn rider_anim_seq_type01_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        let k = Addr(r32(c, GP).wrapping_sub(SEQ01_TICK_GP)).read_fpu(c);
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
        call_ptr(c, SEQ01_ADVANCE_TRACK, 0x0010_4974)?;
        let ended = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !ended {
            c.cycles += 3;
            c.f[12] = c.f[20];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, SEQ01_UPDATE_WEIGHT_FADE, 0x0010_4988)?;
        }
        c.cycles += 2;
        let remove = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S1][0];
        if remove {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, SEQ01_LIST_REMOVE, 0x0010_4998)?;
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

// ---------------------------------------------------------------------------
// RiderAnim_v04 (seg_1218.cpp).

/// `AIAnimEventMap_PickAnim` (0x311710; called through the table).
pub(crate) const AI_ANIM_EVENT_MAP_PICK_ANIM_207: u32 = 0x0031_1710;

/// `RiderAnim_v04(RiderAnim* self, int event)` (0x104cf8): pick the
/// animation for `event` from the global event map (`gp+0xd0c`) with the
/// rider's filter flags: the board's (`self+0x64` → `+0xcd8`) if there is a
/// board, else the rider's (`self+0x60` → `+0x364`), else 0. Returns what
/// `AIAnimEventMap_PickAnim(map, event, flags)` returns.
pub fn rider_anim_v04(c: &mut Cpu) {
    const RET: u32 = 0x0010_4d28;
    const BOARD: u32 = 0x64;
    const BOARD_FLAGS: u32 = 0xcd8;
    const RIDER: u32 = 0x60;
    const RIDER_FLAGS: u32 = 0x364;
    /// `gp`-relative: the AI animation event map.
    const GP_EVENT_MAP: u32 = 0xd0c;
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0)).off(BOARD).lw(c);
        c.g[A2][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[A2][0] = Addr(r32(c, V0)).off(BOARD_FLAGS).lw(c);
        } else {
            c.cycles += 3;
            c.g[A0][0] = Addr(r32(c, A0)).off(RIDER).lw(c);
            if c.g[A0][0] != 0 {
                c.g[A2][0] = Addr(r32(c, A0)).off(RIDER_FLAGS).lw(c);
            }
        }
        c.cycles += 5;
        c.g[A0][0] = Addr(r32(c, GP)).off(GP_EVENT_MAP).lw(c);
        call_ptr(c, AI_ANIM_EVENT_MAP_PICK_ANIM_207, RET)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderAnim_UpdateSeqType07 (0x104358)

/// `AnimSequence_SetTrackTime(seq, track, time)` (0x313cf0).
pub(crate) const SEQ07_SET_TRACK_TIME: u32 = 0x0031_3cf0;

/// `AnimSequence_UpdateWeightFade(seq, step)` (0x313800): nonzero when faded out.
pub(crate) const SEQ07_UPDATE_WEIGHT_FADE: u32 = 0x0031_3800;

/// `AnimSeqList_Remove(list, seq)` (0x3145f8).
pub(crate) const SEQ07_LIST_REMOVE: u32 = 0x0031_45f8;

/// `gp - 0x7f58`: 1/60 (ticks to seconds).
pub(crate) const SEQ07_TICK_GP: u32 = 0x7f58;

/// `RiderAnim + 0x60`: the rider actor; its two speed-like floats.
pub(crate) const SEQ07_ACTOR: u32 = 0x60;

pub(crate) const SEQ07_ACTOR_A: u32 = 0x2b0;

pub(crate) const SEQ07_ACTOR_B: u32 = 0x2a4;

/// `AnimSequence + 0x10`: the track-time scale.
pub(crate) const SEQ07_SEQ_SCALE: u32 = 0x10;

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
// RiderAnim_PlaySeqThenLoop (0x103ff8)

/// `gp+0xd8c`: the rider animation bank (animation ids at +0x1030).
pub(crate) const PSTL_ANIM_BANK_GP: u32 = 0xd8c;

/// Animation ids in the bank, indexed by key.
pub(crate) const PSTL_BANK_IDS: u32 = 0x1030;

/// `gp-0x7f64` / `gp-0x7f60` / `gp-0x7f5c`: 1/60 (`.lit4`), ticks to seconds
/// for the intro advance, the plain advance and the weight fade.
pub(crate) const PSTL_TICK_INTRO_GP: u32 = 0x7f64;

pub(crate) const PSTL_TICK_PLAIN_GP: u32 = 0x7f60;

pub(crate) const PSTL_TICK_FADE_GP: u32 = 0x7f5c;

/// AnimSequence: track 0's animation id, time and length.
pub(crate) const PSTL_SEQ_ANIM0: u32 = 0x4;

pub(crate) const PSTL_SEQ_TIME0: u32 = 0x8;

pub(crate) const PSTL_SEQ_LENGTH0: u32 = 0x10;

/// AnimSequence methods (cross-shard direct calls, through the table).
pub(crate) const PSTL_SET_TRACK_ANIM: u32 = 0x0031_3c50;

pub(crate) const PSTL_SET_TRACK_TIME: u32 = 0x0031_3cf0;

pub(crate) const PSTL_SET_TRACK_LOOP: u32 = 0x0031_3d40;

pub(crate) const PSTL_ADVANCE_TRACK: u32 = 0x0031_35b0;

pub(crate) const PSTL_UPDATE_WEIGHT_FADE: u32 = 0x0031_3800;

pub(crate) const PSTL_LIST_REMOVE: u32 = 0x0031_45f8;

/// `int RiderAnim_PlaySeqThenLoop(RiderAnim* anim, AnimSeqList* list,
/// AnimSequence* seq, int intro_key, float dt, int loop_key)` (0x103ff8):
/// while track 0 still plays the intro animation (`bank[intro_key]`), play
/// it once (no loop) and advance it by `dt/60`; when it reaches its end,
/// switch the track to `bank[loop_key]`, looping, from time 0. Otherwise
/// just advance the track. Then run the weight fade; when it has finished,
/// remove the sequence from `list` and return 1, else 0. (`a3` intro_key,
/// `t0` loop_key, `f12` dt.)
pub fn rider_anim_play_seq_then_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(PSTL_ANIM_BANK_GP)).lw(c);
        frame.save(c, S0, 0x30);
        c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
        frame.save_fpr(c, 20, 0x40);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.f[20] = c.f[12];
        let seq = r32(c, S0);
        c.g[V1][0] = Addr(seq.wrapping_add(PSTL_SEQ_ANIM0)).lw(c);
        c.g[V0][0] = Addr(r32(c, A3).wrapping_add(PSTL_BANK_IDS)).lw(c);
        let intro = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = c.g[T0][0];
        if intro {
            c.cycles += 15;
            // Play the intro once.
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, PSTL_SET_TRACK_LOOP, 0x0010_4048)?;
            let k = Addr(r32(c, GP).wrapping_sub(PSTL_TICK_INTRO_GP)).read_fpu(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.f[12] = (FpuF32(c.f[20]) * k).0;
            call_ptr(c, PSTL_ADVANCE_TRACK, 0x0010_405c)?;
            let seq = r32(c, S0);
            let time = Addr(seq.wrapping_add(PSTL_SEQ_TIME0)).read_fpu(c);
            c.f[1] = time.0;
            let length = Addr(seq.wrapping_add(PSTL_SEQ_LENGTH0)).read_fpu(c);
            c.f[0] = length.0;
            let ended = length.le(time);
            abi::set_fcc(c, ended);
            c.g[V1][0] = Addr(r32(c, GP).wrapping_add(PSTL_ANIM_BANK_GP)).lw(c);
            if ended {
                // Switch to the looping animation from the start.
                c.cycles += 16;
                c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[A2][0] = Addr(r32(c, V0).wrapping_add(PSTL_BANK_IDS)).lw(c);
                call_ptr(c, PSTL_SET_TRACK_ANIM, 0x0010_408c)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 1;
                call_ptr(c, PSTL_SET_TRACK_LOOP, 0x0010_409c)?;
                c.f[12] = 0;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                call_ptr(c, PSTL_SET_TRACK_TIME, 0x0010_40ac)?;
            }
        } else {
            c.cycles += 5;
            let k = Addr(r32(c, GP).wrapping_sub(PSTL_TICK_PLAIN_GP)).read_fpu(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.f[12] = (FpuF32(c.f[20]) * k).0;
            call_ptr(c, PSTL_ADVANCE_TRACK, 0x0010_40c8)?;
        }
        c.cycles += 6;
        let k = Addr(r32(c, GP).wrapping_sub(PSTL_TICK_FADE_GP)).read_fpu(c);
        c.g[A0][0] = c.g[S0][0];
        c.f[12] = (FpuF32(c.f[20]) * k).0;
        call_ptr(c, PSTL_UPDATE_WEIGHT_FADE, 0x0010_40d8)?;
        let faded = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S2][0];
        if faded {
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, PSTL_LIST_REMOVE, 0x0010_40f0)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// RiderAnim_SaveState (0x104d68)

/// Rider animation state: 0x50 bytes of plain state, then (+0x50) a pointer
/// to six 8-byte animation sequence lists.
pub(crate) const RAS_STATE_BYTES: u64 = 0x50;

pub(crate) const RAS_SEQ_LISTS: u32 = 0x50;

pub(crate) const RAS_SEQ_LIST_COUNT: u32 = 6;

/// cStream vtable slot "write(buf, n)" (this-adjust halfword +8, method +0xc).
pub(crate) const RAS_STREAM_WRITE: u32 = 0x8;

pub(crate) const RAS_ANIM_SEQ_LIST_SAVE: u32 = 0x0031_47f0;

/// `void RiderAnim_SaveState(RiderAnim* self, cStream* s)` (0x104d68): write
/// the first 0x50 bytes of the animation state to the stream, then save
/// each of the six animation sequence lists (`AnimSeqList_Save`).
pub fn rider_anim_save_state(c: &mut Cpu) {
    use crate::guest::Addr;
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = RAS_STATE_BYTES;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S2][0];
        // s->write(self, 0x50)
        let stream = r32(c, S1);
        let vtable = Addr(stream).lw(c);
        c.g[V0][0] = vtable;
        let adjust = Addr((vtable as u32).wrapping_add(RAS_STREAM_WRITE)).lh(c);
        let method = Addr((vtable as u32).wrapping_add(RAS_STREAM_WRITE + 4)).lw(c);
        c.g[V1][0] = method;
        c.g[A0][0] = sx(stream.wrapping_add(adjust as u32));
        call_ptr(c, method as u32, 0x0010_4da4)?;
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(RAS_SEQ_LISTS)).lw(c);
        loop {
            c.cycles += 8;
            let i = r32(c, S0);
            c.g[A1][0] = c.g[S1][0];
            c.g[S0][0] = sx(i.wrapping_add(1));
            c.g[A0][0] = sx(r32(c, V0).wrapping_add(i << 3));
            call_ptr(c, RAS_ANIM_SEQ_LIST_SAVE, 0x0010_4dbc)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < RAS_SEQ_LIST_COUNT as i64) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr(r32(c, S2).wrapping_add(RAS_SEQ_LISTS)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0010_4da8);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimController_ReplaceSequence (0x312b18)

/// A sequence node's current weight (+0x94), fade weight (+0x98) and fade
/// target (+0x9c), all floats.
pub(crate) const ANIM_NODE_WEIGHT: u32 = 0x94;

pub(crate) const ANIM_NODE_FADE_WEIGHT: u32 = 0x98;

pub(crate) const ANIM_NODE_FADE_TARGET: u32 = 0x9c;

/// `cAnimSequencer`'s sequence list head (+4).
#[cfg(test)]
pub(crate) const ANIM_SEQ_LIST_HEAD: u32 = 0x4;

// ---------------------------------------------------------------------------
// RiderAnim_SeqEndType02 (0x104a60)

/// The `.rodata` jump table at `0x4569e0` (`0x450000 + 0x69e0`), indexed by
/// `state - 0x10d` (states 0x10d..=0x11e).
pub(crate) const SEQ_END_TABLE: u32 = 0x69e0;

/// `AnimController_ReplaceSequence` (0x312b18, cross-shard).
pub(crate) const SEQ_END_REPLACE: u32 = 0x0031_2b18;

/// `int RiderAnim_SeqEndType02(RiderAnim* a, cAnimSequencer* b, int* state,
/// ...)` (0x104a60): the sequence-end handler for states 0x10d..=0x11e; each
/// state maps (through the `.rodata` jump table) to a call of
/// `AnimController_ReplaceSequence(a, b, state, anim)` with the state's
/// animation id, or 0x1b6 for the two states without one.
///
/// Also accepts every entry of the generated `__entries` twin (the trampoline
/// re-enters at a case block after an event unwinds the callee).
pub fn rider_anim_seq_end_type02(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                // Entry: the state's index and the frame.
                0x0010_4a60 => {
                    c.cycles += 7;
                    c.g[V1][0] = Addr(r32(c, A2)).lw(c);
                    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
                    let frame = sp(c);
                    c.sd(frame, c.g[RA][0]);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x10d));
                    c.g[V0][0] = (c.g[V1][0] < 0x12) as u64;
                    let in_table = c.g[V0][0] != 0;
                    c.g[A3][0] = c.g[A0][0];
                    pc = if in_table { 0x0010_4a7c } else { 0x0010_4b38 };
                }
                // The jump-table read.
                0x0010_4a7c => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x0045_0000;
                    c.g[V1][0] = sx(r32(c, V1) << 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(SEQ_END_TABLE));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
                    pc = r32(c, A0);
                    let known = matches!(
                        pc,
                        0x0010_4a98
                            | 0x0010_4aac
                            | 0x0010_4ac0
                            | 0x0010_4ad4
                            | 0x0010_4ae8
                            | 0x0010_4afc
                            | 0x0010_4b10
                            | 0x0010_4b24
                            | 0x0010_4b38
                    );
                    if !known {
                        c.jump_out(pc, 0x0010_4a60, 0x0010_4b48);
                        return Ok(());
                    }
                }
                // The nine cases: replace the sequence with the state's anim.
                0x0010_4a98 => {
                    seq_end_replace(c, 0x0010_4aa4, 0x123)?;
                    pc = 0x0010_4b40;
                }
                0x0010_4aac => {
                    seq_end_replace(c, 0x0010_4ab8, 0x124)?;
                    pc = 0x0010_4b40;
                }
                0x0010_4ac0 => {
                    seq_end_replace(c, 0x0010_4acc, 0x121)?;
                    pc = 0x0010_4b40;
                }
                0x0010_4ad4 => {
                    seq_end_replace(c, 0x0010_4ae0, 0x122)?;
                    pc = 0x0010_4b40;
                }
                0x0010_4ae8 => {
                    seq_end_replace(c, 0x0010_4af4, 0x125)?;
                    pc = 0x0010_4b40;
                }
                0x0010_4afc => {
                    seq_end_replace(c, 0x0010_4b08, 0x126)?;
                    pc = 0x0010_4b40;
                }
                0x0010_4b10 => {
                    seq_end_replace(c, 0x0010_4b1c, 0x127)?;
                    pc = 0x0010_4b40;
                }
                0x0010_4b24 => {
                    seq_end_replace(c, 0x0010_4b30, 0x128)?;
                    pc = 0x0010_4b40;
                }
                // No animation for this state.
                0x0010_4b38 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x1b6;
                    c.g[RA][0] = c.ld(sp(c));
                    pc = 0x0010_4b40;
                }
                // Epilogue.
                0x0010_4b40 => {
                    c.cycles += 2;
                    let ra = r32(c, RA);
                    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
                    c.pc = ra;
                    return Ok(());
                }
                other => {
                    c.jump_out(other, 0x0010_4a60, 0x0010_4b48);
                    return Ok(());
                }
            }
        }
    })
}

/// One case of `RiderAnim_SeqEndType02`: `AnimController_ReplaceSequence(a,
/// b, state, anim)`, then reload `ra` from the frame.
pub(crate) fn seq_end_replace(c: &mut Cpu, ret: u32, anim: u32) -> crate::call::Flow {
    c.cycles += 5;
    c.g[A0][0] = c.g[A3][0];
    c.g[A3][0] = anim as u64;
    call_ptr(c, SEQ_END_REPLACE, ret)?;
    c.g[RA][0] = c.ld(sp(c));
    Ok(())
}

// ---------------------------------------------------------------------------
// RiderAnim sequence-end handlers (0x104b48, 0x104c18)

/// `RiderAnim_ClearFlagsAndPlayAnim` (0x312bd0): cross-shard, so the generated
/// code calls it through the function table.
pub(crate) const RIDER_ANIM_CLEAR_FLAGS_AND_PLAY_ANIM: u32 = 0x0031_2bd0;

/// The animation id used when the rider has an animation chain.
pub(crate) const SEQ_END_ANIM_CHAIN: u64 = 0x1b4;

/// The animation id used when the rider has no animation chain.
pub(crate) const SEQ_END_ANIM_NO_CHAIN: u64 = 0x11f;

/// `RiderAnim_SeqEndType05` (0x104b48): the sequence-end handler for state
/// type 05. It plays animation 0x1b4 when the rider's animation chain
/// (`RiderAnim+0x64`) is present, 0x11f otherwise, through
/// `RiderAnim_ClearFlagsAndPlayAnim(a, b, c, anim)`.
pub fn rider_anim_seq_end_type05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x64)).lw(c);
        let has_chain = c.g[V0][0] != 0;
        c.g[A3][0] = SEQ_END_ANIM_CHAIN;
        if !has_chain {
            c.cycles += 1;
            c.g[A3][0] = SEQ_END_ANIM_NO_CHAIN;
        }
        c.cycles += 5;
        call_ptr(c, RIDER_ANIM_CLEAR_FLAGS_AND_PLAY_ANIM, 0x0010_4b68)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderAnim_SeqEndType04` (0x104c18): the sequence-end handler for state
/// type 04; always plays animation 5.
pub fn rider_anim_seq_end_type04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = 0x5;
        call_ptr(c, RIDER_ANIM_CLEAR_FLAGS_AND_PLAY_ANIM, 0x0010_4c28)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderAnim_SeqEndType00 (0x104ca0)

/// `AnimSeqList_Remove` (0x3145f8), reached through the function table.
pub(crate) const RIDER_ANIM_LIST_REMOVE: u32 = 0x0031_45f8;

/// The sequence-end result the rider animation dispatcher expects.
pub(crate) const RIDER_ANIM_SEQ_END_RESULT: u64 = 0x1b6;

/// `int RiderAnim_SeqEndType00(RiderAnim* anim, AnimSeqList* list, AnimSeq*
/// seq)` (0x104ca0): unlink and free `seq` from `list` (`AnimSeqList_Remove`,
/// through the function table) and return 0x1b6, the "sequence ended"
/// selector.
pub fn rider_anim_seq_end_type00(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        call_ptr(c, RIDER_ANIM_LIST_REMOVE, 0x0010_4cb4)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = RIDER_ANIM_SEQ_END_RESULT;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------- RiderAnim_SeqEndType03

/// `int RiderAnim_SeqEndType03(RiderAnim* a, cAnimSequencer* b, AnimNode* n,
/// ...)` (0x104b78): the sequence-end handler for state type 03: replace the
/// sequence with animation 5 (`AnimController_ReplaceSequence(a, b, n, 5)`).
pub fn rider_anim_seq_end_type03(c: &mut Cpu) {
    use crate::game::animation::rideranim::SEQ_END_REPLACE;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = 0x5;
        call_ptr(c, SEQ_END_REPLACE, 0x0010_4b88)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Callees shared by the sequence updaters (reached through the function
// table, as the generated bodies do).

/// `AnimSequence_SetTrackAnim(seq, track, anim_id)`.
pub(crate) const OC342_SET_TRACK_ANIM: u32 = 0x0031_3c50;

/// `AnimSequence_SetTrackTime(seq, track, t)` (t in f12).
pub(crate) const OC342_SET_TRACK_TIME: u32 = 0x0031_3cf0;

/// `AnimSequence_SetTrackWeight(seq, track, w)` (w in f12).
pub(crate) const OC342_SET_TRACK_WEIGHT: u32 = 0x0031_3d28;

/// `AnimSequence_SetTrackLoop(seq, track, loop)`.
pub(crate) const OC342_SET_TRACK_LOOP: u32 = 0x0031_3d40;

/// `AnimSequence_AdvanceTrack(seq, track, step)`.
pub(crate) const OC342_ADVANCE_TRACK: u32 = 0x0031_35b0;

/// `AnimSequence_UpdateWeightFade(seq, step)`: nonzero when faded out.
pub(crate) const OC342_UPDATE_WEIGHT_FADE: u32 = 0x0031_3800;

/// `AnimSeqList_Remove(list, seq)`.
pub(crate) const OC342_LIST_REMOVE: u32 = 0x0031_45f8;

/// `AnimController_ReplaceSequence(anim, list, seq, anim_id)`.
pub(crate) const OC342_REPLACE_SEQUENCE: u32 = 0x0031_2b18;

/// `RiderAnim_ClearFlagsAndPlayAnim(anim, list, seq, anim_id)`.
pub(crate) const OC342_CLEAR_FLAGS_AND_PLAY_ANIM: u32 = 0x0031_2bd0;

/// `gp+0xd8c`: the rider animation bank; its animation ids are at +0x1030.
pub(crate) const OC342_ANIM_BANK_GP: u32 = 0xd8c;

pub(crate) const OC342_BANK_IDS: u32 = 0x1030;

/// The anim object's actor pointer.
pub(crate) const OC342_ACTOR: u32 = 0x60;

/// The sequence's playback speed (scales the track time).
pub(crate) const OC342_SEQ_SPEED: u32 = 0x10;

/// The shared tail of the sequence updaters: `AnimSequence_UpdateWeightFade
/// (s0, f12)`; when it reports the sequence faded out, unlink it from the list
/// in `list_reg` and return 1, else 0.
#[inline]
pub(crate) fn oc342_fade_tail(c: &mut Cpu, list_reg: usize, rets: [u32; 2]) -> Flow {
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, OC342_UPDATE_WEIGHT_FADE, rets[0])?;
    let faded = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[list_reg][0];
    if faded {
        c.cycles += 3;
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, OC342_LIST_REMOVE, rets[1])?;
        c.g[V0][0] = 1;
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    Ok(())
}

/// The bank's animation id for `key` (`bank->ids[key]`) into a2, with
/// a0 = s0 and a1 = 0. Leaves `bank_reg` = the bank and `addr_reg` = the
/// id's address minus 0x1030, as the generated code's v0/v1 pair does.
#[inline]
pub(crate) fn oc342_bank_id(c: &mut Cpu, bank_reg: usize, addr_reg: usize, key: u32) {
    let gp = r32(c, GP);
    c.g[bank_reg][0] = Addr(gp.wrapping_add(OC342_ANIM_BANK_GP)).lw(c);
    c.g[addr_reg][0] = sx(key << 2);
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0;
    c.g[addr_reg][0] = sx(r32(c, addr_reg).wrapping_add(r32(c, bank_reg)));
    c.g[A2][0] = Addr(r32(c, addr_reg).wrapping_add(OC342_BANK_IDS)).lw(c);
}

// ---------------------------------------------------------------------------
// RiderAnim_ScrubSeq (0x103be0)

/// `gp-0x7f70`: 1/60 (`.lit4`), ticks to seconds for the weight fade.
pub(crate) const SCRUB_SEQ_TICK_GP: u32 = 0x7f70;

/// `int RiderAnim_ScrubSeq(RiderAnim* self, AnimSeqList* list, AnimSeq* seq,
/// int key_pos, int key_neg, float dt /*f12*/, float v /*f13*/)` (0x103be0):
/// play the bank animation `key_pos` (v >= 0) or `key_neg` (v < 0, with v
/// negated) on track 0, positioned at `|v| * seq->speed`; then fade the
/// sequence's weight by `dt` ticks and drop it from `list` once it has faded
/// out (returns 1, else 0).
pub fn rider_anim_scrub_seq(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        c.f[0] = 0;
        frame.save_fpr(c, 20, 0x30);
        c.f[20] = c.f[13];
        frame.save(c, S0, 0x20);
        let v = FpuF32(c.f[20]);
        let backwards = abi::c_lt_s(c, v, FpuF32::ZERO);
        frame.save(c, S1, 0x10);
        frame.save_fpr(c, 21, 0x38);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.f[21] = c.f[12];
        if backwards {
            c.cycles += 15;
            let key = r32(c, T0);
            oc342_bank_id(c, V1, V0, key);
            call_ptr(c, OC342_SET_TRACK_ANIM, 0x0010_3c30)?;
            c.f[0] = (-FpuF32(c.f[20])).0;
            let speed = Addr(r32(c, S0).wrapping_add(OC342_SEQ_SPEED)).read_fpu(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.f[12] = (FpuF32(c.f[0]) * speed).0;
            call_ptr(c, OC342_SET_TRACK_TIME, 0x0010_3c48)?;
        } else {
            c.cycles += 12;
            let key = r32(c, A3);
            oc342_bank_id(c, V0, V1, key);
            call_ptr(c, OC342_SET_TRACK_ANIM, 0x0010_3c6c)?;
            let speed = Addr(r32(c, S0).wrapping_add(OC342_SEQ_SPEED)).read_fpu(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.f[12] = (FpuF32(c.f[20]) * speed).0;
            call_ptr(c, OC342_SET_TRACK_TIME, 0x0010_3c80)?;
        }
        c.cycles += 6;
        let tick = Addr(r32(c, GP).wrapping_sub(SCRUB_SEQ_TICK_GP)).read_fpu(c);
        c.f[12] = (FpuF32(c.f[21]) * tick).0;
        oc342_fade_tail(c, S1, [0x0010_3c90, 0x0010_3ca8])?;
        c.cycles += 7;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderAnim_UpdateSeqType0B (0x1043f8)

/// First sequence state of the 0B group (0x129..=0x130).
pub(crate) const SEQ0B_FIRST_STATE: u32 = 0x129;

/// The `.rodata` jump table over the eight states.
pub(crate) const SEQ0B_TABLE: u32 = 0x0045_69c0;

/// The actor's two lean values; the larger magnitude drives the blend.
pub(crate) const SEQ0B_ACTOR_A: u32 = 0x28c;

pub(crate) const SEQ0B_ACTOR_B: u32 = 0x298;

/// `gp-0x7f54`: 1/60 (`.lit4`), ticks to seconds for track 1's advance.
pub(crate) const SEQ0B_TICK_GP: u32 = 0x7f54;

/// `int RiderAnim_UpdateSeqType0B(RiderAnim* self, AnimSeqList* list,
/// AnimSeq* seq, float dt /*f12*/)` (0x1043f8): put the state's bank
/// animation (one of keys 0xa5..0xac, by `seq->state - 0x129`) on looping
/// track 1, weigh it `0.25 + 0.75 * m` against track 0 (`1 - that`), where
/// `m` is the actor's larger lean magnitude clamped to [0, 1]; set track 0's
/// time to `m * seq->speed`, advance track 1 by dt ticks, then fade the
/// sequence and unlink it once faded (returns 1, else 0).
pub fn rider_anim_update_seq_type0b(c: &mut Cpu) {
    if c.pc != 0x0010_43f8 {
        return ee::s00::f_001043f8__entries(c);
    }
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_fpr(c, 22, 0x50);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.f[22] = c.f[12];
        frame.save_fpr(c, 21, 0x48);
        frame.save_fpr(c, 20, 0x40);
        let state = Addr(r32(c, S0)).lw(c);
        c.g[V1][0] = sx((state as u32).wrapping_sub(SEQ0B_FIRST_STATE));
        c.g[V0][0] = (c.g[V1][0] < 8) as u64;
        c.g[A0][0] = 0xa5;
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            c.g[V0][0] = sx(SEQ0B_TABLE);
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = Addr(r32(c, V1)).lw(c);
            let target = r32(c, A0);
            let (cycles, key) = match target {
                0x0010_4458 => (2, 0xa5),
                0x0010_4460 => (2, 0xab),
                0x0010_4468 => (2, 0xa8),
                0x0010_4470 => (2, 0xac),
                0x0010_4478 => (2, 0xa6),
                0x0010_4480 => (2, 0xaa),
                0x0010_4488 => (2, 0xa7),
                0x0010_4490 => (1, 0xa9),
                _ => {
                    c.jump_out(target, 0x0010_43f8, 0x0010_45b8);
                    return Ok(());
                }
            };
            c.cycles += cycles;
            c.g[A0][0] = key;
        }

        // Track 1: the state's animation, looping.
        c.cycles += 20;
        let key = r32(c, A0);
        oc342_bank_id(c, V1, V0, key);
        c.g[A1][0] = 1;
        call_ptr(c, OC342_SET_TRACK_ANIM, 0x0010_44b0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.g[A2][0] = 1;
        call_ptr(c, OC342_SET_TRACK_LOOP, 0x0010_44c0)?;

        // m = max(|a|, |b|), clamped to [0, 1].
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(OC342_ACTOR)).lw(c);
        let actor = r32(c, V0);
        let a = Addr(actor.wrapping_add(SEQ0B_ACTOR_A)).read_fpu(c).abs();
        let b = Addr(actor.wrapping_add(SEQ0B_ACTOR_B)).read_fpu(c).abs();
        c.f[0] = a.0;
        c.f[1] = b.0;
        let mut m = a;
        if abi::c_lt_s(c, a, b) {
            c.cycles += 1;
            m = b;
        }
        c.cycles += 7;
        c.g[AT][0] = 0x3f80_0000;
        c.f[12] = FpuF32::ONE.0;
        let clamped = if oc342_c_le_s(c, FpuF32::ZERO, m) {
            m.min(FpuF32::ONE)
        } else {
            FpuF32::ZERO
        };

        // Weights: track 0 gets 1 - w, track 1 gets w = 0.25 + 0.75 * m.
        c.cycles += 31;
        c.f[21] = clamped.0;
        c.f[0] = 0x3e80_0000;
        c.g[AT][0] = 0x3e80_0000;
        c.g[A0][0] = c.g[S0][0];
        let w = clamped * FpuF32(0x3f40_0000) + FpuF32(0x3e80_0000);
        c.f[20] = w.0;
        c.g[A1][0] = 0;
        c.f[12] = (FpuF32::ONE - w).0;
        call_ptr(c, OC342_SET_TRACK_WEIGHT, 0x0010_4530)?;
        c.f[12] = c.f[20];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, OC342_SET_TRACK_WEIGHT, 0x0010_4540)?;

        // Track 0 is positioned by m, track 1 advances by dt.
        let speed = Addr(r32(c, S0).wrapping_add(OC342_SEQ_SPEED)).read_fpu(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.f[12] = (FpuF32(c.f[21]) * speed).0;
        call_ptr(c, OC342_SET_TRACK_TIME, 0x0010_4554)?;
        let tick = Addr(r32(c, GP).wrapping_sub(SEQ0B_TICK_GP)).read_fpu(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.f[20] = (FpuF32(c.f[22]) * tick).0;
        c.f[12] = c.f[20];
        call_ptr(c, OC342_ADVANCE_TRACK, 0x0010_456c)?;
        c.f[12] = c.f[20];
        oc342_fade_tail(c, S2, [0x0010_4578, 0x0010_4590])?;

        c.cycles += 9;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x50);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `c.le.s a, b`: the compare result, also latched in the FCC bit.
#[inline]
pub(crate) fn oc342_c_le_s(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let le = a.le(b);
    abi::set_fcc(c, le);
    le
}

// ---------------------------------------------------------------------------
// RiderAnim_UpdateSeqType08 (0x1045d8)

/// The actor's board/rider data pointer and its speed scale at +0x2a0.
pub(crate) const SEQ08_ACTOR_DATA: u32 = 0x77c;

pub(crate) const SEQ08_DATA_SCALE: u32 = 0x2a0;

/// `gp-0x7f50`: 1/60 (`.lit4`), ticks to seconds for the weight fade.
pub(crate) const SEQ08_TICK_GP: u32 = 0x7f50;

/// `int RiderAnim_UpdateSeqType08(RiderAnim* self, AnimSeqList* list,
/// AnimSeq* seq, float dt /*f12*/)` (0x1045d8): set track 0's time to
/// `seq->speed * actor->data->scale`, then fade the sequence by dt ticks and
/// unlink it once faded (returns 1, else 0).
pub fn rider_anim_update_seq_type08(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.f[20] = c.f[12];
        c.g[A1][0] = 0;
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(OC342_ACTOR)).lw(c);
        let speed = Addr(r32(c, S0).wrapping_add(OC342_SEQ_SPEED)).read_fpu(c);
        c.f[0] = speed.0;
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(SEQ08_ACTOR_DATA)).lw(c);
        let scale = Addr(r32(c, V1).wrapping_add(SEQ08_DATA_SCALE)).read_fpu(c);
        c.f[12] = (speed * scale).0;
        call_ptr(c, OC342_SET_TRACK_TIME, 0x0010_4618)?;
        let tick = Addr(r32(c, GP).wrapping_sub(SEQ08_TICK_GP)).read_fpu(c);
        c.f[12] = (FpuF32(c.f[20]) * tick).0;
        oc342_fade_tail(c, S1, [0x0010_4628, 0x0010_4640])?;
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
// RiderAnim_UpdateSeqType0E (0x104728)

/// The two sequence states that play the value inverted (`1 - v`, clamped).
pub(crate) const SEQ0E_INVERTED_A: u64 = 0x19;

pub(crate) const SEQ0E_INVERTED_B: u64 = 0x21;

/// The actor's value that positions track 0.
pub(crate) const SEQ0E_ACTOR_VALUE: u32 = 0x268;

/// `gp-0x7f4c`: 1/60 (`.lit4`), ticks to seconds for the weight fade.
pub(crate) const SEQ0E_TICK_GP: u32 = 0x7f4c;

/// `int RiderAnim_UpdateSeqType0E(RiderAnim* self, AnimSeqList* list,
/// AnimSeq* seq, float dt /*f12*/)` (0x104728): position track 0 at
/// `t * seq->speed`, with t the actor's value (+0x268), or `1 - value`
/// clamped to [0, 1] for states 0x19 and 0x21; then fade the sequence by dt
/// ticks and unlink it once faded (returns 1, else 0).
pub fn rider_anim_update_seq_type0e(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = SEQ0E_INVERTED_A;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.f[20] = c.f[12];
        let mut inverted = c.g[V1][0] == SEQ0E_INVERTED_A;
        if !inverted {
            c.cycles += 3;
            c.g[V0][0] = SEQ0E_INVERTED_B;
            inverted = c.g[V1][0] == SEQ0E_INVERTED_B;
            if !inverted {
                c.g[V0][0] = Addr(r32(c, A0).wrapping_add(OC342_ACTOR)).lw(c);
            }
        }
        if inverted {
            c.cycles += 10;
            c.g[V0][0] = Addr(r32(c, A0).wrapping_add(OC342_ACTOR)).lw(c);
            c.g[AT][0] = 0x3f80_0000;
            c.f[2] = FpuF32::ONE.0;
            let value = Addr(r32(c, V0).wrapping_add(SEQ0E_ACTOR_VALUE)).read_fpu(c);
            c.f[1] = 0;
            let inv = FpuF32::ONE - value;
            c.f[0] = inv.0;
            let mut t = FpuF32::ZERO;
            if oc342_c_le_s(c, FpuF32::ZERO, inv) {
                c.cycles += 1;
                t = inv.min(FpuF32::ONE);
            }
            c.cycles += 2;
            c.f[12] = t.0;
            c.f[1] = t.0;
        } else {
            c.cycles += 1;
            let value = Addr(r32(c, V0).wrapping_add(SEQ0E_ACTOR_VALUE)).read_fpu(c);
            c.f[1] = value.0;
        }
        c.cycles += 11;
        let speed = Addr(r32(c, S0).wrapping_add(OC342_SEQ_SPEED)).read_fpu(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.f[12] = (FpuF32(c.f[1]) * speed).0;
        call_ptr(c, OC342_SET_TRACK_TIME, 0x0010_47ac)?;
        let tick = Addr(r32(c, GP).wrapping_sub(SEQ0E_TICK_GP)).read_fpu(c);
        c.f[12] = (FpuF32(c.f[20]) * tick).0;
        oc342_fade_tail(c, S1, [0x0010_47bc, 0x0010_47d4])?;
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
// RiderAnim_UpdateSeqType0D (0x1047f0)

/// The two sequence states that play the value inverted (`1 - |v|`).
pub(crate) const SEQ0D_INVERTED_A: u64 = 0x25;

pub(crate) const SEQ0D_INVERTED_B: u64 = 0x26;

/// The actor's value that positions track 0.
pub(crate) const SEQ0D_ACTOR_VALUE: u32 = 0x280;

/// `gp-0x7f48`: 1/60 (`.lit4`), ticks to seconds for the weight fade.
pub(crate) const SEQ0D_TICK_GP: u32 = 0x7f48;

/// `int RiderAnim_UpdateSeqType0D(RiderAnim* self, AnimSeqList* list,
/// AnimSeq* seq, float dt /*f12*/)` (0x1047f0): position track 0 at
/// `t * seq->speed`, t = |actor value (+0x280)| (or `1 - |value|` for states
/// 0x25 and 0x26) clamped to [0, 1]; then fade the sequence by dt ticks and
/// unlink it once faded (returns 1, else 0).
pub fn rider_anim_update_seq_type0d(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = SEQ0D_INVERTED_A;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.f[20] = c.f[12];
        let mut inverted = c.g[V1][0] == SEQ0D_INVERTED_A;
        if !inverted {
            c.cycles += 3;
            c.g[V0][0] = SEQ0D_INVERTED_B;
            inverted = c.g[V1][0] == SEQ0D_INVERTED_B;
            if !inverted {
                c.g[V0][0] = Addr(r32(c, A0).wrapping_add(OC342_ACTOR)).lw(c);
            }
        }
        let t = if inverted {
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, A0).wrapping_add(OC342_ACTOR)).lw(c);
            c.g[AT][0] = 0x3f80_0000;
            let mag = Addr(r32(c, V0).wrapping_add(SEQ0D_ACTOR_VALUE))
                .read_fpu(c)
                .abs();
            c.f[0] = mag.0;
            FpuF32::ONE - mag
        } else {
            c.cycles += 2;
            let value = Addr(r32(c, V0).wrapping_add(SEQ0D_ACTOR_VALUE)).read_fpu(c);
            c.f[0] = value.0;
            value.abs()
        };
        c.f[1] = t.0;

        // Clamp to [0, 1].
        c.cycles += 7;
        c.g[AT][0] = 0x3f80_0000;
        c.f[2] = FpuF32::ONE.0;
        let clamped = if oc342_c_le_s(c, FpuF32::ZERO, t) {
            t.min(FpuF32::ONE)
        } else {
            FpuF32::ZERO
        };
        c.f[0] = clamped.0;

        c.cycles += 11;
        let speed = Addr(r32(c, S0).wrapping_add(OC342_SEQ_SPEED)).read_fpu(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.f[12] = (clamped * speed).0;
        call_ptr(c, OC342_SET_TRACK_TIME, 0x0010_487c)?;
        let tick = Addr(r32(c, GP).wrapping_sub(SEQ0D_TICK_GP)).read_fpu(c);
        c.f[12] = (FpuF32(c.f[20]) * tick).0;
        oc342_fade_tail(c, S1, [0x0010_488c, 0x0010_48a4])?;
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
// RiderAnim_SeqEndType01/07/08 (0x104a40, 0x104b98, 0x104bb8)

/// `AnimController_ReplaceSequence(a0, a1, a2, anim_id)` with a fixed id, as
/// the one-line sequence-end handlers do.
#[inline]
pub(crate) fn oc342_seq_end_replace(c: &mut Cpu, anim_id: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = anim_id;
        call_ptr(c, OC342_REPLACE_SEQUENCE, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderAnim_SeqEndType01(RiderAnim*, AnimSeqList*, AnimSeq*)`
/// (0x104a40): replace the ended sequence with animation 0x11f.
pub fn rider_anim_seq_end_type01(c: &mut Cpu) {
    oc342_seq_end_replace(c, 0x11f, 0x0010_4a50);
}

/// `void RiderAnim_SeqEndType07(RiderAnim*, AnimSeqList*, AnimSeq*)`
/// (0x104b98): replace the ended sequence with animation 3.
pub fn rider_anim_seq_end_type07(c: &mut Cpu) {
    oc342_seq_end_replace(c, 0x3, 0x0010_4ba8);
}

/// `void RiderAnim_SeqEndType08(RiderAnim*, AnimSeqList*, AnimSeq*)`
/// (0x104bb8): replace the ended sequence with animation 0x13.
pub fn rider_anim_seq_end_type08(c: &mut Cpu) {
    oc342_seq_end_replace(c, 0x13, 0x0010_4bc8);
}

// ---------------------------------------------------------------------------
// RiderAnim_SeqEndType09 (0x104bd8)

/// The actor's mode word: 1 picks animation 0x1c, anything else 0x24.
pub(crate) const SEQ_END09_ACTOR_MODE: u32 = 0x330;

/// `int RiderAnim_SeqEndType09(RiderAnim* anim, AnimSeqList*, AnimSeq*)`
/// (0x104bd8): `RiderAnim_ClearFlagsAndPlayAnim` with animation 0x1c when
/// the actor's mode (+0x330) is 1, else 0x24.
pub fn rider_anim_seq_end_type09(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = 1;
        c.g[A0][0] = Addr(r32(c, A3).wrapping_add(OC342_ACTOR)).lw(c);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(SEQ_END09_ACTOR_MODE)).lw(c);
        let mode_one = c.g[V0][0] == c.g[V1][0];
        c.g[A0][0] = c.g[A3][0];
        if mode_one {
            c.cycles += 1;
            c.g[A3][0] = 0x1c;
        } else {
            c.cycles += 2;
            c.g[A3][0] = 0x24;
        }
        c.cycles += 5;
        call_ptr(c, OC342_CLEAR_FLAGS_AND_PLAY_ANIM, 0x0010_4c0c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderAnim_SeqEndType06 (0x104c38)

/// `int RiderAnim_SeqEndType06(RiderAnim*, AnimSeqList*, int* state)`
/// (0x104c38): `RiderAnim_ClearFlagsAndPlayAnim` with animation 0x14 for
/// state 0x45, 0x13 for 0x46, else 0x12.
pub fn rider_anim_seq_end_type06(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr(r32(c, A2)).lw(c);
        c.g[V0][0] = 0x45;
        let first = c.g[V1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if first {
            c.cycles += 2;
            c.g[A3][0] = 0x14;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x46;
            let second = c.g[V1][0] == c.g[V0][0];
            c.g[A3][0] = 0x12;
            if second {
                c.cycles += 1;
                c.g[A3][0] = 0x13;
            } else {
                c.cycles += 2;
            }
        }
        c.cycles += 5;
        call_ptr(c, OC342_CLEAR_FLAGS_AND_PLAY_ANIM, 0x0010_4c74)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderAnim sequence-end handler 0A, event sound, LoadState
// (0x104c80, 0x104cc8, 0x104de0; src/seg/seg_1218.cpp)

/// `RiderAnim_ClearFlagsAndPlayAnim` (0x312bd0, through the function table).
pub(crate) const RIDER_ANIM_SEQ_END_0A_PLAY: u32 = 0x0031_2bd0;

/// The animation the type-0A sequence end plays.
pub(crate) const RIDER_ANIM_SEQ_END_0A_ANIM: u64 = 0x1b2;

/// `void RiderAnim_SeqEndType0A(RiderAnim* a, AnimSeqList* list, AnimSeq*
/// seq)` (0x104c80): the sequence-end handler for state type 0A; plays
/// animation 0x1b2 through `RiderAnim_ClearFlagsAndPlayAnim(a, list, seq,
/// 0x1b2)`.
pub fn rider_anim_seq_end_type0a(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = RIDER_ANIM_SEQ_END_0A_ANIM;
        call_ptr(c, RIDER_ANIM_SEQ_END_0A_PLAY, 0x0010_4c90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The rider's sound owner (`RiderAnim+0x60`).
pub(crate) const RIDER_ANIM_SOUND_OWNER: u32 = 0x60;

/// `SSXAudioSystem_OnAnimSoundEvent` (0x289b18, through the function table).
pub(crate) const RIDER_ANIM_ON_SOUND_EVENT: u32 = 0x0028_9b18;

/// `void RiderAnim_PlayEventSound(RiderAnim* a, u16 event)` (0x104cc8): when
/// the animation has a sound owner (`+0x60`), pass the event to the audio
/// system (`gp+0x410`): `OnAnimSoundEvent(audio, owner, event & 0xffff)`.
pub fn rider_anim_play_event_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(oc344_at(c, A0, RIDER_ANIM_SOUND_OWNER));
        c.g[A2][0] = c.g[A1][0] & 0xffff;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.lw(oc344_at(c, GP, 0x410));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, RIDER_ANIM_ON_SOUND_EVENT, 0x0010_4ce8)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The six animation sequence lists (`RiderAnim+0x50`, 8 bytes each).
pub(crate) const RIDER_ANIM_SEQ_LISTS: u32 = 0x50;

/// `AnimSeqList_Clear` (0x314668) and `AnimSeqList_LoadState` (0x314880),
/// both through the function table.
pub(crate) const RIDER_ANIM_LIST_CLEAR: u32 = 0x0031_4668;

pub(crate) const RIDER_ANIM_LIST_LOAD: u32 = 0x0031_4880;

/// `void RiderAnim_LoadState(RiderAnim* self, Stream* s)` (0x104de0): read
/// the first 0x50 bytes of the animation state (`s->Read(self, 0x50)`, slot
/// 0x10 of the stream's vtable), then clear and reload each of the six
/// sequence lists at `*(self+0x50) + 8*i`.
pub fn rider_anim_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0x50;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[S2][0];
        c.g[V0][0] = c.lw(r32(c, S3));
        let vt = c.g[V0][0];
        let read = VSlot::read(c, vt, 0x10);
        c.g[V1][0] = read.func;
        c.g[A0][0] = sx(r32(c, S3).wrapping_add(read.delta as u32));
        call_ptr(c, read.func as u32, 0x0010_4e20)?;
        c.g[A0][0] = c.lw(oc344_at(c, S2, RIDER_ANIM_SEQ_LISTS));
        loop {
            c.cycles += 11;
            c.g[S0][0] = sx(r32(c, S1) << 3);
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S0)));
            call_ptr(c, RIDER_ANIM_LIST_CLEAR, 0x0010_4e38)?;
            c.g[A0][0] = c.lw(oc344_at(c, S2, RIDER_ANIM_SEQ_LISTS));
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S0)));
            call_ptr(c, RIDER_ANIM_LIST_LOAD, 0x0010_4e48)?;
            let more = (c.g[S1][0] as i64) < 6;
            c.g[V0][0] = more as u64;
            if !more {
                break;
            }
            c.g[A0][0] = c.lw(oc344_at(c, S2, RIDER_ANIM_SEQ_LISTS));
            if event_due(c) {
                deliver_event_here(c, 0x0010_4e28);
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::elf_image;

    use crate::difftest::fakes::machine;

    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::SP as STACK;

    /// Common setup for the `RiderAnim_UpdateSeqTypeNN` selectors: an anim
    /// object at DATA whose actor (+0x60) is at DATA+0x400, the actor's blend
    /// words filled in, and the sequence descriptor at DATA+0x200.
    pub(crate) fn seq_setup(c: &mut Cpu, seed: u32, blend_off: u32, seqs: &[u32]) {
        let mut r = Rng(0x5e9_0000 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = (DATA + 0x100) as u64;
        c.g[A2][0] = (DATA + 0x200) as u64;
        let actor = DATA + 0x400;
        c.bus.write32(DATA + 0x60, actor);
        c.bus
            .write32(actor + 0x320, if seed % 2 == 0 { 0 } else { r.next_u32() });
        for off in [0x1fc, 0x214, 0x238, 0x244, 0x274] {
            c.bus.write32(actor + off, r.edge_f32());
        }
        c.bus.write32(actor + blend_off, r.edge_f32());
        let seq = if (seed as usize) % seqs.len() == seqs.len() - 1 {
            r.next_u32()
        } else {
            seqs[(seed as usize) % seqs.len()]
        };
        c.bus.write32(DATA + 0x200, seq);
    }

    #[test]
    fn rider_anim_update_seq_type03_matches() {
        check(
            ssx3_ee::s00::f_00104110,
            rider_anim_update_seq_type03,
            80,
            &|c, seed| {
                seq_setup(c, seed, 0x1fc, &[0x7, 0xa, 0x1234]);
            },
        );
    }

    #[test]
    fn rider_anim_update_seq_type04_matches() {
        check(
            ssx3_ee::s00::f_00104178,
            rider_anim_update_seq_type04,
            80,
            &|c, seed| {
                seq_setup(c, seed, 0x1fc, &[0x6, 0x8, 0xe, 0xf, 0x55]);
            },
        );
    }

    #[test]
    fn rider_anim_update_seq_type05_matches() {
        check(
            ssx3_ee::s00::f_00104238,
            rider_anim_update_seq_type05,
            80,
            &|c, seed| {
                seq_setup(c, seed, 0x238, &[0x12, 0x13, 0x77]);
            },
        );
    }

    #[test]
    fn rider_anim_update_seq_type06_matches() {
        check(
            ssx3_ee::s00::f_001042a8,
            rider_anim_update_seq_type06,
            80,
            &|c, seed| {
                let mut r = Rng(0x1042_a800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = (DATA + 0x200) as u64;
                let actor = DATA + 0x400;
                c.bus.write32(DATA + 0x60, actor);
                c.bus
                    .write32(actor + 0x320, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(actor + 0x214, r.edge_f32());
            },
        );
    }

    #[test]
    fn rider_anim_update_seq_type0a_matches() {
        check(
            ssx3_ee::s00::f_001042e0,
            rider_anim_update_seq_type0a,
            80,
            &|c, seed| {
                seq_setup(c, seed, 0x238, &[0xda, 0xe2, 0xea, 0x33]);
            },
        );
    }

    #[test]
    fn rider_anim_update_seq_type0c_matches() {
        check(
            ssx3_ee::s00::f_001045b8,
            rider_anim_update_seq_type0c,
            80,
            &|c, seed| {
                let mut r = Rng(0x1045_b800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = (DATA + 0x200) as u64;
            },
        );
    }

    #[test]
    fn rider_anim_update_seq_type09_matches() {
        check(
            ssx3_ee::s00::f_00104660,
            rider_anim_update_seq_type09,
            80,
            &|c, seed| {
                seq_setup(c, seed, 0x244, &[0x2d, 0x99]);
            },
        );
    }

    #[test]
    fn rider_anim_update_seq_type0f_matches() {
        check(
            ssx3_ee::s00::f_001046b0,
            rider_anim_update_seq_type0f,
            80,
            &|c, seed| {
                seq_setup(c, seed, 0x274, &[0x1c, 0x88]);
                // Force both sides of the `x >= 0` FPU clamp.
                let actor = DATA + 0x400;
                let x = match seed % 4 {
                    0 => 0.0f32.to_bits(),
                    1 => 1.5f32.to_bits(),
                    2 => (-1.5f32).to_bits(),
                    _ => (-100.0f32).to_bits(),
                };
                c.bus.write32(actor + 0x274, x);
            },
        );
    }

    #[test]
    fn rider_anim_set_rider_matches() {
        check(
            ssx3_ee::s00::f_00104d38,
            rider_anim_set_rider,
            200,
            &|c, seed| {
                let mut r = Rng(0x104d_3800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
                c.bus.write32(DATA + 0x400 + 0x780, r.next_u32());
            },
        );
    }

    #[test]
    fn rider_anim_set_actor_matches() {
        check(
            ssx3_ee::s00::f_00104d50,
            rider_anim_set_actor,
            200,
            &|c, seed| {
                let mut r = Rng(0x104d_5000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
                c.bus.write32(DATA + 0x400 + 0x8, r.next_u32());
            },
        );
    }

    // -------------------------------------------------------------- RiderAnim

    /// `RiderAnim_v03` fixture. The real descriptor table at `.rodata
    /// 0x446990` (28-byte entries, loaded from the ELF image) supplies the
    /// sequence-end kind at `entry + 8`; `*out` is the entry index. These
    /// indices have kinds 0..10 and one whose kind is >= 0xb (the epilogue
    /// fall-through). The kind indexes the real handler table at 0x456990.
    pub(crate) const ANIM_ENTRY_FOR_KIND: [u32; 12] =
        [0, 27, 269, 21, 2, 71, 49, 4, 220, 23, 435, 437];

    pub(crate) fn anim_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        machine(c, seed, salt, 1);
        elf_image(c);
        c.g[SP][0] = (STACK - 0x200) as u64;
        for i in 0..0x100 {
            c.bus.write32(DATA + i * 4, 0);
        }
        c.g[A0][0] = DATA as u64; // RiderAnim
        c.g[A2][0] = (DATA + 0x100) as u64; // out: *out = descriptor index
        c.bus.write32(DATA + 0x50, 0x1000); // first sequence-end address
        let k = (seed % 12) as usize;
        let idx = ANIM_ENTRY_FOR_KIND[k];
        c.bus.write32(DATA + 0x100, idx);
        // The real descriptor really has the kind that selects this arm (and
        // the last one is out of the handler table's range).
        let kind = c.bus.read32(0x0044_6990 + idx * 0x1c + 8);
        assert!(
            if k < 11 {
                kind == k as u32
            } else {
                kind >= 0xb
            },
            "descriptor {idx} has kind {kind}, wanted {k}"
        );
        // Sequence-end address: the slot written is (seqEnd - 0x1000) >> 3.
        c.g[A1][0] = (0x1000 + (seed % 4) * 8) as u64;
    }

    #[test]
    fn rider_anim_v03_matches() {
        check(
            ssx3_ee::s00::f_00103918__generated,
            rider_anim_v03,
            36,
            &|c, seed| anim_fixture(c, seed, 47),
        );
    }

    /// The static animation table (`AnimTable_GetEntry`'s base; 28-byte
    /// entries, the sequence type at +4).
    pub(crate) const ANIM_TABLE: u32 = 0x0044_6990;

    #[test]
    fn rider_anim_update_sequence_matches() {
        check(
            ssx3_ee::s00::f_001036a0__generated,
            rider_anim_update_sequence,
            24,
            &|c, seed| {
                let mut r = Rng(0x1036_a0d1 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let anim = DATA;
                let seq = DATA + 0x200;
                c.g[A0][0] = anim as u64;
                c.g[A1][0] = (DATA + 0x1800) as u64;
                c.g[A2][0] = seq as u64;
                c.f[12] = r.edge_f32();
                // The animation-type jump table (0x456950) selects the
                // per-type update; the test machine's RAM starts empty.
                put_words(
                    c,
                    0x0045_6950,
                    &[
                        0x0010_3700,
                        0x0010_371c,
                        0x0010_3738,
                        0x0010_3754,
                        0x0010_3770,
                        0x0010_378c,
                        0x0010_37a8,
                        0x0010_37c4,
                        0x0010_37e0,
                        0x0010_37fc,
                        0x0010_3818,
                        0x0010_3834,
                        0x0010_3850,
                        0x0010_386c,
                        0x0010_3888,
                        0x0010_38a4,
                    ],
                );
                // The sequence's animation-table index (seed covers all 20
                // entries: 0..15 run a per-type update, 16..19 skip it).
                let index = seed % 20;
                c.bus.write32(seq, index);
                c.bus.write32(ANIM_TABLE + index * 0x1c + 4, seed % 20);
                // The optional sound-event owner at anim+0x60.
                let owner = anim + 0x300;
                c.bus
                    .write32(anim + 0x60, if seed % 2 == 0 { owner } else { 0 });
                c.bus
                    .write32(owner + 0x874, if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(owner + 0x87c, if seed % 4 == 0 { 0 } else { 1 });
                // `RiderAnim_FireSoundEvents` indexes a table of sound owners
                // through gp+0xd08; give it a table whose first entry is an
                // object with no events (it returns after the first load).
                c.bus.write32(GAME_GP + 0xd08, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, DATA + 0x1100);
                c.bus.write32(DATA + 0x1108, 0);
                c.bus.write16(DATA + 0x12, 0);
            },
        );
    }

    // ------------------------------------------------ RiderAnim_UpdateSeqType02

    /// `RiderAnim_UpdateSeqType02`: the real track advance, weight fade and
    /// list removal on a random sequence (ended / faded / still running).
    #[test]
    fn rider_anim_seq_type02_update_matches() {
        use crate::difftest::fakes::{machine, ptr};
        check(
            ssx3_ee::s00::f_001049c0__generated,
            rider_anim_seq_type02_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_49c0, 0);
                crate::difftest::elf_image(c);
                c.pc = 0x0010_49c0;
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

    // ------------------------------------------------------- RiderAnim_BlendSeq4

    /// `RiderAnim_BlendSeq4`: a sequence whose track 0 sometimes already
    /// plays the `from` key, `v` in each of the four key intervals (and on
    /// their edges), and a weight fade that sometimes ends (list removal).
    /// The real track setters, advance, fade and list removal run.
    #[test]
    fn rider_anim_blend_seq4_matches() {
        use crate::difftest::fakes::{machine, ptr};
        use crate::difftest::{check, elf_image, GAME_GP};
        check(
            ee::s00::f_00103e28__generated,
            rider_anim_blend_seq4,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_3e28, 0);
                elf_image(c);
                c.pc = 0x0010_3e28;
                // The animation bank: ids 0x100 + key at +0x1030.
                let bank = ptr(&mut r);
                c.bus.write32(GAME_GP + 0xd8c, bank);
                for k in 0..8 {
                    c.bus.write32(bank + 0x1030 + 4 * k, 0x100 + k * 0x101);
                }
                let keys: Vec<u32> = (0..5).map(|_| r.next_u32() % 8).collect();
                for (reg, k) in [A3, T0, T1, T2, T3].into_iter().zip(&keys) {
                    c.g[reg][0] = *k as u64;
                }
                let seq = ptr(&mut r);
                let list = ptr(&mut r);
                c.g[A0][0] = sx(ptr(&mut r));
                c.g[A1][0] = sx(list);
                c.g[A2][0] = sx(seq);
                for off in [0x8, 0xc, 0x10, 0x2c, 0x90, 0x94, 0x98, 0x9c, 0xa4, 0xa8] {
                    let v = match r.next_u32() % 4 {
                        0 => 0,
                        1 => r.edge_f32(),
                        _ => r.f32(3.0),
                    };
                    c.bus.write32(seq + off, v);
                }
                // Track 0 already plays one of the keys (or something else).
                let cur = if seed % 3 == 0 {
                    r.next_u32()
                } else {
                    0x100 + keys[(r.next_u32() % 5) as usize] * 0x101
                };
                c.bus.write32(seq + 0x4, cur);
                c.bus.write32(seq + 0x1c, r.next_u32() & 1);
                c.bus.write32(seq + 0xa0, r.next_u32() & 1);
                let mut at = list + 4;
                for _ in 0..seed % 3 {
                    let other = ptr(&mut r) + 0x8;
                    c.bus.write32(at, other);
                    at = other + 200;
                }
                let vs = [-1.0f32, -0.75, -0.5, -0.25, 0.0, 0.25, 0.5, 0.75, 1.0];
                c.f[13] = match seed % 11 {
                    9 => r.edge_f32(),
                    10 => r.f32(1.0),
                    i => vs[i as usize].to_bits(),
                };
                c.f[12] = if seed % 4 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(3.0)
                };
            },
        );
    }

    /// `RiderAnim_FireSoundEvents` over sequences of 0..8 events with random
    /// reached/armed masks and codes with and without the sound bit. The
    /// rider is null, so the event sound call takes its short path.
    #[test]
    fn rider_anim_fire_sound_events_matches() {
        check(
            ee::s00::f_00103aa0__generated,
            rider_anim_fire_sound_events,
            150,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x10_3aa0, 0);
                let (this, anim, banks, bank, seqs, events) = (
                    DATA,
                    DATA + 0x100,
                    DATA + 0x400,
                    DATA + 0x500,
                    DATA + 0x600,
                    DATA + 0x800,
                );
                c.bus.write32(GAME_GP + ANIM_BANKS, banks);
                let b = r.next_u32() % 4;
                let seq = r.next_u32() % 6;
                c.bus.write32(banks + 4 * b, bank);
                c.bus.write32(bank + 8, seqs);
                c.bus.write32(bank + 0x10, events);
                let first = r.next_u32() % 8;
                c.bus.write16(seqs + 0x14 * seq + 0x10, first as u16);
                c.bus.write16(seqs + 0x14 * seq + 0x12, (seed % 9) as u16);
                for k in 0..20 {
                    let code = r.next_u32() as u16;
                    c.bus.write16(events + 4 * k + 2, code);
                }
                c.bus.write32(anim + ANIM_SEQ_ID, b | seq << 8);
                for k in 0..4 {
                    c.bus
                        .write32(anim + ANIM_EVENT_FLAGS + 4 * k, r.next_u32() | r.next_u32());
                }
                c.bus.write32(this + 0x60, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(anim);
                c.pc = 0x0010_3aa0;
            },
        );
    }

    use crate::difftest::fakes::ptr;

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

    /// `RiderAnim_BlendSeq2`: random keys into an id bank, v around 0
    /// (positive, zero, negative, edge cases), random sequence times and
    /// track states; the real AnimSequence methods (from the game image)
    /// run, including the list removal when the fade finishes.
    #[test]
    fn rider_anim_blend_seq2_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::ptr;
        check(
            ee::s00::f_00103cc8__generated,
            rider_anim_blend_seq2,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_3cc8, 0);
                elf_image(c);
                c.pc = 0x0010_3cc8;
                let bank = ptr(&mut r);
                c.bus.write32(GAME_GP + ANIM_BANK_GP, bank);
                for k in 0..8 {
                    c.bus.write32(bank + BANK_IDS + 4 * k, 0x100 + k * 0x101);
                }
                for reg in [A3, T0, T1] {
                    c.g[reg][0] = (r.next_u32() % 8) as u64;
                }
                let seq = ptr(&mut r);
                let list = ptr(&mut r);
                c.g[A0][0] = sx(ptr(&mut r));
                c.g[A1][0] = sx(list);
                c.g[A2][0] = sx(seq);
                for off in [0x8, 0xc, 0x10, 0x2c, 0x90, 0x94, 0x98, 0x9c, 0xa4, 0xa8] {
                    let v = match r.next_u32() % 4 {
                        0 => 0,
                        1 => r.edge_f32(),
                        _ => r.f32(3.0),
                    };
                    c.bus.write32(seq + off, v);
                }
                c.bus.write32(seq + 0x4, 0x100 + (r.next_u32() % 8) * 0x101);
                c.bus.write32(seq + 0x1c, r.next_u32() & 1);
                c.bus.write32(seq + 0xa0, r.next_u32() & 1);
                let mut at = list + 4;
                for _ in 0..seed % 3 {
                    let other = ptr(&mut r) + 0x8;
                    c.bus.write32(at, other);
                    at = other + 200;
                }
                let vs = [-1.0f32, -0.5, -0.25, 0.0, 0.25, 0.5, 1.0];
                c.f[13] = match seed % 9 {
                    7 => r.edge_f32(),
                    8 => r.f32(1.0),
                    i => vs[i as usize].to_bits(),
                };
                c.f[12] = if seed % 4 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(3.0)
                };
            },
        );
    }

    pub(crate) use crate::game::ai::ai::tests::oc258_w32;

    /// `RiderAnim_UpdateSeqType01`: the real track advance, weight fade and
    /// list removal on a random sequence (ended / faded / still running).
    #[test]
    fn rider_anim_seq_type01_update_matches() {
        use crate::difftest::fakes::ptr;
        check(
            ee::s00::f_00104940__generated,
            rider_anim_seq_type01_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_4940, 0);
                crate::difftest::elf_image(c);
                c.pc = 0x0010_4940;
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

    // ------------------------------------------------------- rider anim

    /// RiderAnim_v04: a board (its flags), no board but a rider (its
    /// flags), or neither (0); the real AIAnimEventMap_PickAnim takes its
    /// single-choice path (`entry+2 == 1`) for events 0..3.
    #[test]
    fn rider_anim_v04_matches() {
        const ANIM: u32 = DATA;
        const BOARD: u32 = DATA + 0x1000;
        const RIDER: u32 = DATA + 0x2000;
        const MAP: u32 = DATA + 0x3000;
        check(
            ee::s00::f_00104cf8__generated,
            rider_anim_v04,
            200,
            &|c, seed| {
                let mut r = Rng(0x0010_4cf8 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0010_4cf8;
                c.g[A0][0] = sx(ANIM);
                c.g[A1][0] = (seed % 4) as u64;
                let (board, rider) = match seed % 3 {
                    0 => (BOARD, 0),
                    1 => (0, RIDER),
                    _ => (0, 0),
                };
                c.bus.write32(ANIM + 0x64, board);
                c.bus.write32(ANIM + 0x60, rider);
                let v = r.next_u32();
                c.bus.write32(BOARD + 0xcd8, v);
                let v = r.next_u32();
                c.bus.write32(RIDER + 0x364, v);
                c.bus.write32(GAME_GP + 0xd0c, MAP);
                for i in 0..4 {
                    let pick = (r.next_u32() % 8) as u16;
                    c.bus.write16(MAP + 4 * i, pick);
                    c.bus.write16(MAP + 4 * i + 2, 1);
                }
                for k in 0..8 {
                    let v = r.next_u32();
                    c.bus.write32(0x0044_9960 + 12 * k, v);
                }
            },
        );
    }

    /// RiderAnim_UpdateSeqType07: either actor float the larger, weight fades
    /// that finish (list removal) or not; the real track setter, fade and
    /// list removal run.
    #[test]
    fn rider_anim_update_seq_type07_matches() {
        use crate::difftest::fakes::ptr;
        check(
            ee::s00::f_00104358__generated,
            rider_anim_update_seq_type07,
            200,
            &|c, seed| {
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
                    let v = if r.next_u32() % 5 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(30.0)
                    };
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
                c.f[12] = if seed % 4 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(0.05)
                };
            },
        );
    }

    /// RiderAnim_PlaySeqThenLoop: track 0 plays the intro key (or not), the
    /// intro reaches its end (or not) after the real AdvanceTrack, the real
    /// weight fade finishes (or not) and the real list removal runs.
    #[test]
    fn rider_anim_play_seq_then_loop_matches() {
        use crate::difftest::fakes::{machine, ptr};
        use crate::difftest::{check, elf_image, GAME_GP};
        check(
            ee::s00::f_00103ff8__generated,
            rider_anim_play_seq_then_loop,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_3ff8, 0);
                elf_image(c);
                c.pc = 0x0010_3ff8;
                let bank = ptr(&mut r);
                c.bus.write32(GAME_GP + PSTL_ANIM_BANK_GP, bank);
                for k in 0..8 {
                    c.bus
                        .write32(bank + PSTL_BANK_IDS + 4 * k, 0x100 + k * 0x101);
                }
                let intro = r.next_u32() % 8;
                c.g[A3][0] = intro as u64;
                c.g[T0][0] = (r.next_u32() % 8) as u64;
                let seq = ptr(&mut r);
                let list = ptr(&mut r);
                c.g[A0][0] = sx(ptr(&mut r));
                c.g[A1][0] = sx(list);
                c.g[A2][0] = sx(seq);
                for off in [0x8, 0xc, 0x10, 0x2c, 0x90, 0x94, 0x98, 0x9c, 0xa4, 0xa8] {
                    let v = match r.next_u32() % 4 {
                        0 => 0,
                        1 => r.edge_f32(),
                        _ => r.f32(3.0),
                    };
                    c.bus.write32(seq + off, v);
                }
                let cur = if seed % 3 == 0 {
                    0x100 + (r.next_u32() % 8) * 0x101
                } else {
                    0x100 + intro * 0x101
                };
                c.bus.write32(seq + PSTL_SEQ_ANIM0, cur);
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
                    r.f32(3.0)
                };
            },
        );
    }

    /// RiderAnim_SaveState: the stream's write is a fake (some calls
    /// unwind); the real AnimSeqList_Save runs on six lists with no items
    /// (count 0 or negative), so it only writes its count through the fake.
    #[test]
    fn rider_anim_save_state_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::DATA;
        const SELF: u32 = DATA + 0x20_0000;
        const STREAM: u32 = DATA + 0x20_0100;
        const LISTS: u32 = DATA + 0x20_0200;
        covered(
            ee::s00::f_00104d68__generated,
            rider_anim_save_state,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_4d68, 7);
                c.pc = 0x0010_4d68;
                let vt = vtable(c, &mut r, &[RAS_STREAM_WRITE]);
                c.bus.write32(STREAM, vt);
                c.bus.write32(SELF + RAS_SEQ_LISTS, LISTS);
                for i in 0..RAS_SEQ_LIST_COUNT {
                    let count = if r.next_u32() % 3 == 0 {
                        -(r.next_u32() as i32 % 4)
                    } else {
                        0
                    };
                    c.bus.write32(LISTS + 8 * i, count as u32);
                    c.bus.write32(LISTS + 8 * i + 4, r.next_u32());
                }
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(STREAM);
            },
        );
    }

    #[test]
    fn rider_anim_seq_end_type02_matches() {
        use crate::difftest::{check, put_words, DATA};
        check(
            ssx3_ee::s00::f_00104a60__generated,
            rider_anim_seq_end_type02,
            200,
            &|c, seed| {
                // The same animation chain the replacement call runs.
                replace_sequence_setup(c, seed, 0x0010_4a60);
                // The game's `.rodata` jump table for states 0x10d..=0x11e
                // (the two states 0x115 and 0x11a have no entry block). The
                // 0x115 entry is corrupt to exercise the `jump_out` arm.
                put_words(
                    c,
                    0x0045_69e0,
                    &[
                        0x0010_4ac0,
                        0x0010_4ad4,
                        0x0010_4a98,
                        0x0010_4aac,
                        0x0010_4ae8,
                        0x0010_4afc,
                        0x0010_4b10,
                        0x0010_4b24,
                        0xdead_1234,
                        0x0010_4ac0,
                        0x0010_4ad4,
                        0x0010_4aac,
                        0x0010_4aac,
                        0x0010_4b38,
                        0x0010_4ac0,
                        0x0010_4ad4,
                        0x0010_4a98,
                        0x0010_4aac,
                    ],
                );
                // Every state, plus out-of-table ones.
                let state = if seed % 5 == 0 {
                    0x108 + seed % 0x60
                } else {
                    0x10d + seed % 18
                };
                // The node doubles as the state word (a2 = the int* argument).
                c.bus.write32(DATA + 0x300, state);
                c.pc = 0x0010_4a60;
            },
        );
    }

    /// The animation chain `AnimController_ReplaceSequence` runs, kept on
    /// its short paths (no global fake slots, so the tests stay independent):
    /// an empty sequencer list, and `AnimController_PlayAnim`'s virtual
    /// returning the "no animation" id 0x207.
    pub(crate) fn replace_sequence_setup(c: &mut Cpu, seed: u32, salt: u64) {
        use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
        use crate::difftest::DATA;
        let mut r = machine(c, seed, salt, 0);
        let (self_, sub, seq, node) = (DATA + 0x100, DATA + 0x180, DATA + 0x200, DATA + 0x300);
        // `PlayAnim`'s virtual: a fake that returns 0x207.
        c.bus.write32(self_ + 0x58, sub);
        c.bus.write16(sub + 0x20, ((r.next_u32() % 4) * 4) as u16);
        c.bus.write32(sub + 0x24, FAKE);
        c.bus.write32(LOG_RESULTS, 0x207);
        // An empty sequencer list: both list walks return at once.
        c.bus.write32(seq + ANIM_SEQ_LIST_HEAD, 0);
        // The node's weight, fade weight and fade target (no fade on every
        // third seed, edge floats otherwise).
        c.bus.write32(node + ANIM_NODE_WEIGHT, r.edge_f32());
        c.bus.write32(node + ANIM_NODE_FADE_WEIGHT, r.edge_f32());
        let target = if seed % 3 == 0 {
            0.0f32.to_bits()
        } else {
            r.edge_f32()
        };
        c.bus.write32(node + ANIM_NODE_FADE_TARGET, target);
        c.g[A0][0] = self_ as u64;
        c.g[A1][0] = seq as u64;
        c.g[A2][0] = node as u64;
        c.g[A3][0] = (r.next_u32() % 8) as u64;
    }

    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    /// Magic word that turns the gated test fakes on. It lives in each test
    /// machine's own RAM (`DATA + 0xf0000`, below the fakes' call log), so the
    /// rest of the suite reaches the fakes' chained-to bodies instead.
    pub(crate) const CTRL: u32 = DATA + 0x0f_0000;

    pub(crate) const MAGIC: u32 = 0x4f43_3332;

    /// The bodies the slots held before a test fake was installed.
    pub(crate) static PREV: [AtomicUsize; 16] = [const { AtomicUsize::new(0) }; 16];

    /// Replace a generated slot with `fake` (once), remembering what it held.
    pub(crate) fn install(slot: *mut fn(&mut Cpu), fake: fn(&mut Cpu), idx: usize) {
        crate::difftest::fakes::hook(slot, fake, &PREV[idx]);
    }

    /// Run the body a slot held before our fake (if any).
    pub(crate) fn chain(c: &mut Cpu, idx: usize, mine: fn(&mut Cpu)) {
        let p = PREV[idx].load(Ordering::Relaxed);
        if p != 0 && p != mine as usize {
            let f: GuestFn = unsafe { std::mem::transmute(p) };
            f(c);
        }
    }

    pub(crate) fn active(c: &mut Cpu) -> bool {
        c.bus.read32(CTRL) == MAGIC
    }

    /// Return to the caller after `cycles`, with `v0`/`v1` clobbered.
    pub(crate) fn fake_ret(c: &mut Cpu, cycles: u64) {
        c.g[V0][0] = c.bus.read32(CTRL + 8) as u64;
        c.g[V1][0] = c.bus.read32(CTRL + 0xc) as u64;
        c.cycles += cycles;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
    }

    // -- slot 0: RiderAnim_ClearFlagsAndPlayAnim (0x312bd0) ------------------

    pub(crate) fn fake_clear_flags_and_play_anim(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 0, fake_clear_flags_and_play_anim);
        }
        if c.bus.read32(CTRL + 4) != 0 {
            // Ask the caller to unwind.
            c.pc = 0xdead_0000;
            return;
        }
        fake_ret(c, 5);
    }

    pub(crate) fn install_seq_end_fakes() {
        install(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00312BD0),
            fake_clear_flags_and_play_anim,
            0,
        );
    }

    #[test]
    fn rider_anim_seq_end_type05_matches() {
        install_seq_end_fakes();
        check(
            ee::s00::f_00104b48__generated,
            rider_anim_seq_end_type05,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_4b48, 1);
                let anim = DATA + 0x1000;
                c.g[A0][0] = anim as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (DATA + 0x3000) as u64;
                // The rider's animation chain, absent on every third seed.
                c.bus
                    .write32(anim + 0x64, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(CTRL, MAGIC);
                c.bus.write32(CTRL + 4, (seed % 4 == 0) as u32);
                c.pc = 0x0010_4b48;
            },
        );
    }

    #[test]
    fn rider_anim_seq_end_type04_matches() {
        install_seq_end_fakes();
        check(
            ee::s00::f_00104c18__generated,
            rider_anim_seq_end_type04,
            200,
            &|c, seed| {
                machine(c, seed, 0x0010_4c18, 1);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (DATA + 0x3000) as u64;
                c.bus.write32(CTRL, MAGIC);
                c.bus.write32(CTRL + 4, (seed % 4 == 0) as u32);
                c.pc = 0x0010_4c18;
            },
        );
    }

    // -------------------------------------------------- RiderAnim_SeqEndType00

    /// Lists of 0..3 sequences, none of them the one to remove, so the real
    /// `AnimSeqList_Remove` only walks the chain.
    #[test]
    fn rider_anim_seq_end_type00_matches() {
        check(
            ee::s00::f_00104ca0__generated,
            rider_anim_seq_end_type00,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_4ca0, 0);
                c.pc = 0x0010_4ca0;
                let list = DATA + 0x400;
                let seq = DATA + 0x9000;
                let n = seed % 4;
                for i in 0..4u32 {
                    let node = list + 0x100 * (i + 1);
                    c.bus
                        .write32(node + 0xc8, if i + 1 < n { node + 0x100 } else { 0 });
                }
                c.bus
                    .write32(list + 4, if n == 0 { 0 } else { list + 0x100 });
                c.bus.write32(list, n ^ (r.next_u32() & 0xf));
                c.g[A1][0] = sx(list);
                c.g[A2][0] = sx(seq);
            },
        );
    }

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fill;

    use crate::difftest::SP as TEST_SP;

    pub(crate) use crate::game::ai::computer::tests::oc344_install;
    pub(crate) use crate::game::ai::computer::tests::oc344_machine;

    /// `RiderAnim_SeqEndType03` over the real `AnimController_ReplaceSequence`
    /// chain on its short paths (`RiderAnim_SeqEndType02`'s fixture).
    #[test]
    fn rider_anim_seq_end_type03_matches() {
        use crate::game::animation::rideranim::tests::replace_sequence_setup;
        check(
            ee::s00::f_00104b78__generated,
            rider_anim_seq_end_type03,
            200,
            &|c, seed| {
                replace_sequence_setup(c, seed, 0x0010_4b78);
                c.pc = 0x0010_4b78;
            },
        );
    }

    /// A random 16-byte aligned pointer in the low half of the test data
    /// area (far from the fakes' call log at DATA+0x100000 and from CTRL).
    pub(crate) fn oc342_ptr(r: &mut Rng) -> u32 {
        DATA + (r.next_u32() & 0x7_fff0)
    }

    /// A float that is 0, an edge case or in [-range, range].
    pub(crate) fn oc342_float(r: &mut Rng, range: f32) -> u32 {
        match r.next_u32() % 4 {
            0 => 0,
            1 => r.edge_f32(),
            _ => r.f32(range),
        }
    }

    /// The sequence updaters' world: the anim bank at `gp+0xd8c` with ids for
    /// `keys`, a random sequence (two tracks, weight fade state) in a2 and a
    /// list of 0..2 other sequences in a1 (the real callees run on it), an
    /// anim object in a0 whose actor (+0x60) is returned, and dt in f12.
    pub(crate) fn oc342_seq_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        keys: std::ops::Range<u32>,
    ) -> (u32, u32) {
        elf_image(c);
        let bank = oc342_ptr(r);
        c.bus.write32(GAME_GP + OC342_ANIM_BANK_GP, bank);
        for k in keys {
            c.bus
                .write32(bank + OC342_BANK_IDS + 4 * k, 0x100 + (k % 8) * 0x101);
        }
        let seq = oc342_ptr(r);
        let list = oc342_ptr(r);
        for off in [
            0x8, 0xc, 0x10, 0x24, 0x28, 0x2c, 0x90, 0x94, 0x98, 0x9c, 0xa4, 0xa8,
        ] {
            let v = oc342_float(r, 3.0);
            c.bus.write32(seq + off, v);
        }
        c.bus.write32(seq + 0x4, 0x100 + (r.next_u32() % 8) * 0x101);
        c.bus
            .write32(seq + 0x20, 0x100 + (r.next_u32() % 8) * 0x101);
        c.bus.write32(seq + 0x1c, r.next_u32() & 1);
        c.bus.write32(seq + 0x38, r.next_u32() & 1);
        c.bus.write32(seq + 0xa0, r.next_u32() & 1);
        let mut at = list + 4;
        for _ in 0..r.next_u32() % 3 {
            let other = oc342_ptr(r) + 0x8;
            c.bus.write32(at, other);
            at = other + 200;
        }
        c.bus.write32(at, 0);
        let anim = oc342_ptr(r);
        let actor = oc342_ptr(r);
        c.bus.write32(anim + OC342_ACTOR, actor);
        c.g[A0][0] = sx(anim);
        c.g[A1][0] = sx(list);
        c.g[A2][0] = sx(seq);
        c.f[12] = if r.next_u32() % 4 == 0 {
            r.edge_f32()
        } else {
            r.f32(3.0)
        };
        (seq, actor)
    }

    /// `RiderAnim_ScrubSeq`: forward, backward and edge scrub values, both
    /// key registers, sequences that fade out or not.
    #[test]
    fn rider_anim_scrub_seq_matches() {
        check(
            ee::s00::f_00103be0__generated,
            rider_anim_scrub_seq,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_3be0, 0);
                c.pc = 0x0010_3be0;
                oc342_seq_fixture(c, &mut r, 0..8);
                c.g[A3][0] = (r.next_u32() % 8) as u64;
                c.g[T0][0] = (r.next_u32() % 8) as u64;
                let vs = [-1.0f32, -0.5, -0.0, 0.0, 0.25, 1.0];
                c.f[13] = match seed % 8 {
                    6 => r.edge_f32(),
                    7 => r.f32(2.0),
                    i => vs[i as usize].to_bits(),
                };
            },
        );
    }

    /// `RiderAnim_UpdateSeqType0B`: every state of the table, states outside
    /// it, and a corrupt table entry (the `jump_out` arm); lean values on
    /// both sides of the [0, 1] clamp.
    #[test]
    fn rider_anim_update_seq_type0b_matches() {
        check(
            ee::s00::f_001043f8__generated,
            rider_anim_update_seq_type0b,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_43f8, 0);
                c.pc = 0x0010_43f8;
                let (seq, actor) = oc342_seq_fixture(c, &mut r, 0xa5..0xad);
                for off in [SEQ0B_ACTOR_A, SEQ0B_ACTOR_B] {
                    let v = oc342_float(&mut r, 1.5);
                    c.bus.write32(actor + off, v);
                }
                let k = r.next_u32() % 8;
                let state = match seed % 10 {
                    8 => r.next_u32(),
                    9 => {
                        c.bus.write32(SEQ0B_TABLE + 4 * k, 0xdead_1234);
                        SEQ0B_FIRST_STATE + k
                    }
                    i => SEQ0B_FIRST_STATE + (i % 8),
                };
                c.bus.write32(seq, state);
            },
        );
    }

    /// `RiderAnim_UpdateSeqType08`: random speed and scale, fade or not.
    #[test]
    fn rider_anim_update_seq_type08_matches() {
        check(
            ee::s00::f_001045d8__generated,
            rider_anim_update_seq_type08,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_45d8, 0);
                c.pc = 0x0010_45d8;
                let (_seq, actor) = oc342_seq_fixture(c, &mut r, 0..0);
                let data = oc342_ptr(&mut r);
                c.bus.write32(actor + SEQ08_ACTOR_DATA, data);
                let v = oc342_float(&mut r, 2.0);
                c.bus.write32(data + SEQ08_DATA_SCALE, v);
            },
        );
    }

    /// `RiderAnim_UpdateSeqType0E`: both inverted states and plain ones,
    /// values below 0, inside and above 1.
    #[test]
    fn rider_anim_update_seq_type0e_matches() {
        check(
            ee::s00::f_00104728__generated,
            rider_anim_update_seq_type0e,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_4728, 0);
                c.pc = 0x0010_4728;
                let (seq, actor) = oc342_seq_fixture(c, &mut r, 0..0);
                let v = oc342_float(&mut r, 2.0);
                c.bus.write32(actor + SEQ0E_ACTOR_VALUE, v);
                let state = match seed % 4 {
                    0 => SEQ0E_INVERTED_A as u32,
                    1 => SEQ0E_INVERTED_B as u32,
                    _ => r.next_u32() % 0x40,
                };
                c.bus.write32(seq, state);
            },
        );
    }

    /// `RiderAnim_UpdateSeqType0D`: both inverted states and plain ones,
    /// values on both sides of the clamp.
    #[test]
    fn rider_anim_update_seq_type0d_matches() {
        check(
            ee::s00::f_001047f0__generated,
            rider_anim_update_seq_type0d,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_47f0, 0);
                c.pc = 0x0010_47f0;
                let (seq, actor) = oc342_seq_fixture(c, &mut r, 0..0);
                let v = oc342_float(&mut r, 2.0);
                c.bus.write32(actor + SEQ0D_ACTOR_VALUE, v);
                let state = match seed % 4 {
                    0 => SEQ0D_INVERTED_A as u32,
                    1 => SEQ0D_INVERTED_B as u32,
                    _ => r.next_u32() % 0x40,
                };
                c.bus.write32(seq, state);
            },
        );
    }

    /// `RiderAnim_SeqEndType01`: the real `AnimController_ReplaceSequence`
    /// on the short-path chain the SeqEndType02 test uses.
    #[test]
    fn rider_anim_seq_end_type01_matches() {
        check(
            ee::s00::f_00104a40__generated,
            rider_anim_seq_end_type01,
            200,
            &|c, seed| {
                replace_sequence_setup(c, seed, 0x0010_4a40);
                c.pc = 0x0010_4a40;
            },
        );
    }

    /// `RiderAnim_SeqEndType07`: as SeqEndType01, animation 3.
    #[test]
    fn rider_anim_seq_end_type07_matches() {
        check(
            ee::s00::f_00104b98__generated,
            rider_anim_seq_end_type07,
            200,
            &|c, seed| {
                replace_sequence_setup(c, seed, 0x0010_4b98);
                c.pc = 0x0010_4b98;
            },
        );
    }

    /// `RiderAnim_SeqEndType08`: as SeqEndType01, animation 0x13.
    #[test]
    fn rider_anim_seq_end_type08_matches() {
        check(
            ee::s00::f_00104bb8__generated,
            rider_anim_seq_end_type08,
            200,
            &|c, seed| {
                replace_sequence_setup(c, seed, 0x0010_4bb8);
                c.pc = 0x0010_4bb8;
            },
        );
    }

    /// `RiderAnim_SeqEndType09`: actor mode 1 or other, with the gated
    /// `RiderAnim_ClearFlagsAndPlayAnim` fake (unwinding on some seeds).
    #[test]
    fn rider_anim_seq_end_type09_matches() {
        install_seq_end_fakes();
        check(
            ee::s00::f_00104bd8__generated,
            rider_anim_seq_end_type09,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_4bd8, 1);
                let anim = DATA + 0x1000;
                let actor = DATA + 0x4000;
                c.g[A0][0] = anim as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (DATA + 0x3000) as u64;
                c.bus.write32(anim + OC342_ACTOR, actor);
                let mode = match seed % 3 {
                    0 => 1,
                    1 => 0,
                    _ => r.next_u32(),
                };
                c.bus.write32(actor + SEQ_END09_ACTOR_MODE, mode);
                c.bus.write32(CTRL, MAGIC);
                c.bus.write32(CTRL + 4, (seed % 4 == 0) as u32);
                c.pc = 0x0010_4bd8;
            },
        );
    }

    /// `RiderAnim_SeqEndType06`: states 0x45, 0x46 and others, with the
    /// gated `RiderAnim_ClearFlagsAndPlayAnim` fake (unwinding on some seeds).
    #[test]
    fn rider_anim_seq_end_type06_matches() {
        install_seq_end_fakes();
        check(
            ee::s00::f_00104c38__generated,
            rider_anim_seq_end_type06,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_4c38, 1);
                let state = DATA + 0x3000;
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = state as u64;
                let v = match seed % 3 {
                    0 => 0x45,
                    1 => 0x46,
                    _ => r.next_u32(),
                };
                c.bus.write32(state, v);
                c.bus.write32(CTRL, MAGIC);
                c.bus.write32(CTRL + 4, (seed % 4 == 0) as u32);
                c.pc = 0x0010_4c38;
            },
        );
    }

    /// `RiderAnim_SeqEndType0A`: animation 0x1b2 through the (faked)
    /// `ClearFlagsAndPlayAnim`, which unwinds on some seeds.
    #[test]
    fn rider_anim_seq_end_type0a_matches() {
        oc344_install();
        covered(
            ee::s00::f_00104c80__generated,
            rider_anim_seq_end_type0a,
            200,
            &|c, seed| {
                oc344_machine(c, seed, 0x0010_4c80, 1);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = sx(DATA + 0x3000);
                c.pc = 0x0010_4c80;
            },
        );
    }

    /// `RiderAnim_PlayEventSound`: with and without a sound owner.
    #[test]
    fn rider_anim_play_event_sound_matches() {
        oc344_install();
        covered(
            ee::s00::f_00104cc8__generated,
            rider_anim_play_event_sound,
            200,
            &|c, seed| {
                let mut r = oc344_machine(c, seed, 0x0010_4cc8, 1);
                let anim = DATA + 0x1000;
                c.g[A0][0] = sx(anim);
                c.bus.write32(
                    anim + RIDER_ANIM_SOUND_OWNER,
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
                c.bus.write32(GAME_GP + 0x410, r.next_u32());
                c.pc = 0x0010_4cc8;
            },
        );
    }

    /// `RiderAnim_LoadState`: the stream read (faked through the vtable),
    /// then the six lists, cleared (faked) and reloaded by the real
    /// `AnimSeqList_LoadState` (count 0 from the stack, so no allocation).
    #[test]
    fn rider_anim_load_state_matches() {
        oc344_install();
        covered(
            ee::s00::f_00104de0__generated,
            rider_anim_load_state,
            120,
            &|c, seed| {
                let mut r = oc344_machine(c, seed, 0x0010_4de0, 13);
                let anim = DATA + 0x1000;
                let stream = DATA + 0x2000;
                let lists = DATA + 0x3000;
                fill(c, &mut r, anim, 0x20);
                c.bus.write32(anim + RIDER_ANIM_SEQ_LISTS, lists);
                fill(c, &mut r, lists, 12);
                let vt = vtable(c, &mut r, &[0x10]);
                c.bus.write32(stream, vt);
                // AnimSeqList_LoadState's count word (its sp+0).
                c.bus.write32(TEST_SP - 0x50 - 0x80, 0);
                c.g[A0][0] = sx(anim);
                c.g[A1][0] = sx(stream);
                c.pc = 0x0010_4de0;
            },
        );
    }
}
