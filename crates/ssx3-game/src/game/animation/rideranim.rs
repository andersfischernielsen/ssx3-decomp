//! `src/animation/rideranim.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Target;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
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
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::ai::callee::EA_FILE_FREE_FILE_HANDLE;
    pub(crate) use crate::game::ai::ai::callee::EA_FILE_FREE_REQUEST;
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
    pub(crate) use crate::game::ai::ai::callee::RIDER_WIPEOUT_UPDATE_ANIM_SPEED;
    pub(crate) use crate::game::ai::ai::callee::SPCH_FIND_BANK_TYPE;
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::put_vec;
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
}
