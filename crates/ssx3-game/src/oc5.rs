//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Hand-written, typed replacements for small generated functions, each with a
//! differential test against the generated body. Most are leaves; a few are
//! thin wrappers that forward to another guest function (through [`crate::call`]
//! so the call protocol — frame, return address, unwind — is reproduced).

use crate::abi;
use crate::call::{self, Callee, Frame, Target};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_32c0, focus_man_add_object, "FocusMan_AddObject"),
    (0x0010_33b0, focus_man_add_focus_box, "FocusMan_AddFocusBox"),
    (0x0010_4110, rider_anim_update_seq_type03, "RiderAnim_UpdateSeqType03"),
    (0x0010_4178, rider_anim_update_seq_type04, "RiderAnim_UpdateSeqType04"),
    (0x0010_4238, rider_anim_update_seq_type05, "RiderAnim_UpdateSeqType05"),
    (0x0010_42a8, rider_anim_update_seq_type06, "RiderAnim_UpdateSeqType06"),
    (0x0010_42e0, rider_anim_update_seq_type0a, "RiderAnim_UpdateSeqType0A"),
    (0x0010_45b8, rider_anim_update_seq_type0c, "RiderAnim_UpdateSeqType0C"),
    (0x0010_4660, rider_anim_update_seq_type09, "RiderAnim_UpdateSeqType09"),
    (0x0010_46b0, rider_anim_update_seq_type0f, "RiderAnim_UpdateSeqType0F"),
    (0x0010_4d38, rider_anim_set_rider, "RiderAnim_SetRider"),
    (0x0010_4d50, rider_anim_set_actor, "RiderAnim_SetActor"),
    (0x0010_8f88, computer_actor_rider_init_2, "ComputerActorRider_Init_2"),
    (0x0010_cab8, computer_actor_rider_init_difficulty_2, "ComputerActorRider_InitDifficulty_2"),
    (0x0010_d938, computer_actor_rider_is_far_behind_human, "ComputerActorRider_IsFarBehindHuman"),
    (0x0010_d990, computer_actor_rider_is_far_ahead_of_human, "ComputerActorRider_IsFarAheadOfHuman"),
    (0x0010_f398, ai_reset_rider_relations_2, "AI_ResetRiderRelations_2"),
    (0x0011_1830, rider_base_get_main_state_name, "RiderBase_GetMainStateName"),
    (0x0011_1860, rider_base_get_sub_state_name, "RiderBase_GetSubStateName"),
    (0x0011_3138, air_predictor_ctor, "AirPredictor_ctor"),
];

// ---------------------------------------------------------------------------
// Direct-call targets (the generated bodies, which dispatch through their own
// override slot when one exists).

const BLEND_SEQ2: Callee = Callee { addr: 0x0010_3cc8, f: ssx3_ee::s00::f_00103cc8 };
const BLEND_SEQ4: Callee = Callee { addr: 0x0010_3e28, f: ssx3_ee::s00::f_00103e28 };
const SCRUB_SEQ: Callee = Callee { addr: 0x0010_3be0, f: ssx3_ee::s00::f_00103be0 };
const PLAY_SEQ_THEN_LOOP: Callee = Callee { addr: 0x0010_3ff8, f: ssx3_ee::s00::f_00103ff8 };
const COMPUTER_ACTOR_RIDER_INIT: Callee = Callee { addr: 0x0010_8e88, f: ssx3_ee::s00::f_00108e88 };
const COMPUTER_ACTOR_RIDER_INIT_DIFFICULTY: Callee =
    Callee { addr: 0x0010_c758, f: ssx3_ee::s00::f_0010c758 };
const AI_RESET_RIDER_RELATIONS: Callee = Callee { addr: 0x0010_f3b8, f: ssx3_ee::s00::f_0010f3b8 };
const AIR_PREDICTOR_RESET: Callee = Callee { addr: 0x0011_3198, f: ssx3_ee::s00::f_00113198 };
const GET_HUMAN_RIDER: Callee = Callee { addr: 0x0010_deb0, f: ssx3_ee::s00::f_0010deb0 };
const GET_DIST_TO_FINISH: Callee = Callee { addr: 0x0011_3128, f: ssx3_ee::s00::f_00113128 };

// ---------------------------------------------------------------------------
// Focus manager.

/// `void FocusMan_AddObject(FocusMan *self, FocusObject *obj)` (0x1032c0):
/// tag `obj->flags` (+0x8) with 0x100; store the new flags — OR 0x300 when
/// `self->locked` (+0xd4) is zero, else the tagged value with 0x200 cleared;
/// append `obj` to the list at +0xdc (count at +0xd8) and bump the count.
pub fn focus_man_add_object(c: &mut Cpu) {
    const LOCKED: u32 = 0xd4;
    const FLAGS: u32 = 0x8;
    const COUNT: u32 = 0xd8;
    const OBJECTS: u32 = 0xdc;
    c.cycles += 6;
    let this = abi::arg(c, 0);
    let obj = Addr(abi::arg(c, 1));
    let flags = obj.off(FLAGS).lw(c);
    c.g[V1][0] = flags;
    let tagged = flags | 0x100;
    c.g[A2][0] = tagged;
    obj.off(FLAGS).sw(c, tagged);
    let locked = Addr(this).off(LOCKED).lw(c);
    c.g[V0][0] = locked;
    let stored = if locked == 0 {
        flags | 0x300
    } else {
        c.cycles += 3;
        tagged & !0x200u64
    };
    c.g[V0][0] = stored;
    c.cycles += 8;
    obj.off(FLAGS).sw(c, stored);
    let count = Addr(this).off(COUNT).lw(c);
    c.g[V1][0] = count;
    c.g[V0][0] = sx((count as u32) << 2);
    c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = sx((count as u32).wrapping_add(1));
    Addr(c.g[V0][0] as u32).off(OBJECTS).sw(c, c.g[A1][0]);
    let ret = c.g[RA][0] as u32;
    Addr(this).off(COUNT).sw(c, c.g[V1][0]);
    c.pc = ret;
}

/// `void FocusMan_AddFocusBox(FocusMan *self, FocusBox *box)` (0x1033b0):
/// clear `self->active` (+0xd0) to -1, store `box` in slot `self->count`
/// (stride 0x20, +0x10), copy the box's quad to the next slot (+0x20) and to
/// the just-opened stack scratch, then bump the count.
pub fn focus_man_add_focus_box(c: &mut Cpu) {
    const COUNT: u32 = 0x0;
    const ACTIVE: u32 = 0xd0;
    const SLOT: u32 = 0x10;
    const QUAD: u32 = 0x20;
    c.cycles += 18;
    let this = abi::arg(c, 0);
    let box_ptr = abi::arg(c, 1);
    let count = Addr(this).off(COUNT).lw(c);
    c.g[V0][0] = count;
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    Addr(this).off(ACTIVE).sw(c, c.g[V1][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[V0][0] = sx((count as u32) << 5);
    c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    Addr(c.g[V0][0] as u32).off(SLOT).sw(c, c.g[A1][0]);
    let count = Addr(this).off(COUNT).lw(c);
    c.g[V1][0] = count;
    c.lq(6, box_ptr);
    c.g[V1][0] = sx((count as u32) << 5);
    c.g[V1][0] = sx(this.wrapping_add(c.g[V1][0] as u32));
    c.sq(6, c.g[SP][0] as u32);
    c.sq(6, (c.g[V1][0] as u32).wrapping_add(QUAD));
    let count = Addr(this).off(COUNT).lw(c);
    c.g[V0][0] = count;
    c.g[V0][0] = sx((count as u32).wrapping_add(1));
    Addr(this).off(COUNT).sw(c, c.g[V0][0]);
    let ret = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ret;
}

// ---------------------------------------------------------------------------
// Rider animation sequencing.

/// Blend prologue shared by the `RiderAnim_UpdateSeqTypeNN` selectors: load
/// the anim object's actor (+0x60), then its blend value at `actor + off` into
/// f13, negated (for one cycle) when `actor->0x320` is non-zero. Leaves
/// v1 = actor, v0 = the loaded word.
#[inline]
fn seq_prologue(c: &mut Cpu, blend_off: u32) {
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
        let (extra, t0, a3) = if seq == 0x2d { (3, 0x160, 0x161) } else { (4, 0x15a, 0x15b) };
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

// ---------------------------------------------------------------------------
// Computer actor riders.

/// `void ComputerActorRider_Init_2(ComputerActorRider *self)` (0x108f88):
/// tail wrapper around `ComputerActorRider_Init`.
pub fn computer_actor_rider_init_2(c: &mut Cpu) {
    call::tail_wrapper(c, None, Target::Direct(COMPUTER_ACTOR_RIDER_INIT), 0x0010_8f98);
}

/// `void ComputerActorRider_InitDifficulty_2(ComputerActorRider *self)`
/// (0x10cab8): tail wrapper around `ComputerActorRider_InitDifficulty`.
pub fn computer_actor_rider_init_difficulty_2(c: &mut Cpu) {
    call::tail_wrapper(
        c,
        None,
        Target::Direct(COMPUTER_ACTOR_RIDER_INIT_DIFFICULTY),
        0x0010_cac8,
    );
}

/// `bool ComputerActorRider_IsFarBehindHuman(ComputerActorRider *self)`
/// (0x10d938): true when the human rider's finish distance minus this actor's
/// rider's (+0x4d0) is below the rider's threshold (+0xdc).
pub fn computer_actor_rider_is_far_behind_human(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        GET_HUMAN_RIDER.call(c, 0x0010_d94c)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 10;
            c.g[A0][0] = c.g[V0][0];
            GET_DIST_TO_FINISH.call(c, 0x0010_d95c)?;
            let rider = Addr((c.g[S0][0] as u32).wrapping_add(0x18)).lw(c);
            c.g[V1][0] = rider;
            let limit = Addr(rider as u32).off(0x4d0).read_fpu(c);
            let threshold = Addr(rider as u32).off(0xdc).read_fpu(c);
            abi::set_fpr(c, 1, limit);
            abi::set_fpr(c, 2, threshold);
            let diff = FpuF32(c.f[0]) - limit;
            abi::set_fpr(c, 0, diff);
            let lt = abi::c_lt_s(c, diff, threshold);
            c.g[V0][0] = 0x1;
            if !lt {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool ComputerActorRider_IsFarAheadOfHuman(ComputerActorRider *self)`
/// (0x10d990): true when the this actor's rider's threshold (+0xe0) is below
/// the human rider's finish distance minus the actor's rider's distance
/// (+0x4d0).
pub fn computer_actor_rider_is_far_ahead_of_human(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        GET_HUMAN_RIDER.call(c, 0x0010_d9a4)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 10;
            c.g[A0][0] = c.g[V0][0];
            GET_DIST_TO_FINISH.call(c, 0x0010_d9b4)?;
            let rider = Addr((c.g[S0][0] as u32).wrapping_add(0x18)).lw(c);
            c.g[V1][0] = rider;
            let limit = Addr(rider as u32).off(0x4d0).read_fpu(c);
            let threshold = Addr(rider as u32).off(0xe0).read_fpu(c);
            abi::set_fpr(c, 1, limit);
            abi::set_fpr(c, 2, threshold);
            let diff = FpuF32(c.f[0]) - limit;
            abi::set_fpr(c, 0, diff);
            let lt = abi::c_lt_s(c, threshold, diff);
            c.g[V0][0] = 0x1;
            if !lt {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_ResetRiderRelations_2(void *self)` (0x10f398): tail wrapper around
/// `AI_ResetRiderRelations`.
pub fn ai_reset_rider_relations_2(c: &mut Cpu) {
    call::tail_wrapper(c, None, Target::Direct(AI_RESET_RIDER_RELATIONS), 0x0010_f3a8);
}

// ---------------------------------------------------------------------------
// Rider base state names.

/// `const char *RiderBase_GetMainStateName(RiderBase *self)` (0x111830):
/// index `self->+0xde0` into the 7-entry main-state name table, or a default.
pub fn rider_base_get_main_state_name(c: &mut Cpu) {
    const MAIN_STATE: u32 = 0xde0;
    const NAMES: u32 = 0x0045_9d70; // 0x460000 - 0x6290
    const DEFAULT: u32 = 0x0045_6c48; // 0x450000 + 0x6c48
    c.cycles += 4;
    let this = abi::arg(c, 0);
    let state = Addr(this).off(MAIN_STATE).lw(c);
    c.g[V1][0] = state;
    let in_range = state < 7;
    c.g[V0][0] = in_range as u64;
    if !in_range {
        c.cycles += 2;
        c.g[V0][0] = sx(DEFAULT);
        c.pc = c.g[RA][0] as u32;
    } else {
        c.cycles += 6;
        c.g[V1][0] = sx((state as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(NAMES));
        let ret = c.g[RA][0] as u32;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.pc = ret;
    }
}

/// `const char *RiderBase_GetSubStateName(RiderBase *self)` (0x111860):
/// index `self->+0xde4` into the 15-entry sub-state name table, or a default.
pub fn rider_base_get_sub_state_name(c: &mut Cpu) {
    const SUB_STATE: u32 = 0xde4;
    const NAMES: u32 = 0x0045_9dc8; // 0x460000 - 0x6238
    const DEFAULT: u32 = 0x0045_6c48; // 0x450000 + 0x6c48
    c.cycles += 4;
    let this = abi::arg(c, 0);
    let state = Addr(this).off(SUB_STATE).lw(c);
    c.g[V1][0] = state;
    let in_range = state < 15;
    c.g[V0][0] = in_range as u64;
    if !in_range {
        c.cycles += 2;
        c.g[V0][0] = sx(DEFAULT);
        c.pc = c.g[RA][0] as u32;
    } else {
        c.cycles += 6;
        c.g[V1][0] = sx((state as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(NAMES));
        let ret = c.g[RA][0] as u32;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.pc = ret;
    }
}

// ---------------------------------------------------------------------------
// Air predictor.

/// `AirPredictor *AirPredictor_ctor(AirPredictor *self)` (0x113138): set
/// `self->+0x30` to -1, run `AirPredictor_Reset`, and return `self`.
pub fn air_predictor_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] |= 0xffff;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(c.g[S0][0] as u32).off(0x30).sw(c, c.g[V0][0]);
        AIR_PREDICTOR_RESET.call(c, 0x0011_3158)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// Common setup for the `RiderAnim_UpdateSeqTypeNN` selectors: an anim
    /// object at DATA whose actor (+0x60) is at DATA+0x400, the actor's blend
    /// words filled in, and the sequence descriptor at DATA+0x200.
    fn seq_setup(c: &mut Cpu, seed: u32, blend_off: u32, seqs: &[u32]) {
        let mut r = Rng(0x5e9_0000 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = (DATA + 0x100) as u64;
        c.g[A2][0] = (DATA + 0x200) as u64;
        let actor = DATA + 0x400;
        c.bus.write32(DATA + 0x60, actor);
        c.bus.write32(actor + 0x320, if seed % 2 == 0 { 0 } else { r.next_u32() });
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
    fn focus_man_add_object_matches() {
        check(ssx3_ee::s00::f_001032c0, focus_man_add_object, 300, &|c, seed| {
            let mut r = Rng(0x1032_c000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            let count = (seed % 5) as u32;
            c.bus.write32(DATA + 0xd4, if seed % 3 == 0 { 0 } else { r.next_u32() });
            c.bus.write32(DATA + 0xd8, count);
            c.bus.write32(DATA + 0x400 + 0x8, r.next_u32());
            c.bus.write32(DATA + count * 4 + 0xdc, r.next_u32());
        });
    }

    #[test]
    fn focus_man_add_focus_box_matches() {
        check(ssx3_ee::s00::f_001033b0, focus_man_add_focus_box, 200, &|c, seed| {
            let mut r = Rng(0x1033_b000 ^ seed as u64);
            scramble(c, &mut r);
            let count = (seed % 4) as u32;
            c.g[A0][0] = DATA as u64;
            // Odd seeds use a misaligned box to cover `lq` masking.
            c.g[A1][0] = (DATA + 0x600 + (seed % 2) * 8) as u64;
            c.bus.write32(DATA, count);
            for i in 0..4 {
                c.bus.write32(DATA + 0x600 + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn rider_anim_update_seq_type03_matches() {
        check(ssx3_ee::s00::f_00104110, rider_anim_update_seq_type03, 80, &|c, seed| {
            seq_setup(c, seed, 0x1fc, &[0x7, 0xa, 0x1234]);
        });
    }

    #[test]
    fn rider_anim_update_seq_type04_matches() {
        check(ssx3_ee::s00::f_00104178, rider_anim_update_seq_type04, 80, &|c, seed| {
            seq_setup(c, seed, 0x1fc, &[0x6, 0x8, 0xe, 0xf, 0x55]);
        });
    }

    #[test]
    fn rider_anim_update_seq_type05_matches() {
        check(ssx3_ee::s00::f_00104238, rider_anim_update_seq_type05, 80, &|c, seed| {
            seq_setup(c, seed, 0x238, &[0x12, 0x13, 0x77]);
        });
    }

    #[test]
    fn rider_anim_update_seq_type06_matches() {
        check(ssx3_ee::s00::f_001042a8, rider_anim_update_seq_type06, 80, &|c, seed| {
            let mut r = Rng(0x1042_a800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = (DATA + 0x200) as u64;
            let actor = DATA + 0x400;
            c.bus.write32(DATA + 0x60, actor);
            c.bus.write32(actor + 0x320, if seed % 2 == 0 { 0 } else { r.next_u32() });
            c.bus.write32(actor + 0x214, r.edge_f32());
        });
    }

    #[test]
    fn rider_anim_update_seq_type0a_matches() {
        check(ssx3_ee::s00::f_001042e0, rider_anim_update_seq_type0a, 80, &|c, seed| {
            seq_setup(c, seed, 0x238, &[0xda, 0xe2, 0xea, 0x33]);
        });
    }

    #[test]
    fn rider_anim_update_seq_type0c_matches() {
        check(ssx3_ee::s00::f_001045b8, rider_anim_update_seq_type0c, 80, &|c, seed| {
            let mut r = Rng(0x1045_b800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = (DATA + 0x200) as u64;
        });
    }

    #[test]
    fn rider_anim_update_seq_type09_matches() {
        check(ssx3_ee::s00::f_00104660, rider_anim_update_seq_type09, 80, &|c, seed| {
            seq_setup(c, seed, 0x244, &[0x2d, 0x99]);
        });
    }

    #[test]
    fn rider_anim_update_seq_type0f_matches() {
        check(ssx3_ee::s00::f_001046b0, rider_anim_update_seq_type0f, 80, &|c, seed| {
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
        });
    }

    #[test]
    fn rider_anim_set_rider_matches() {
        check(ssx3_ee::s00::f_00104d38, rider_anim_set_rider, 200, &|c, seed| {
            let mut r = Rng(0x104d_3800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            c.bus.write32(DATA + 0x400 + 0x780, r.next_u32());
        });
    }

    #[test]
    fn rider_anim_set_actor_matches() {
        check(ssx3_ee::s00::f_00104d50, rider_anim_set_actor, 200, &|c, seed| {
            let mut r = Rng(0x104d_5000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            c.bus.write32(DATA + 0x400 + 0x8, r.next_u32());
        });
    }

    #[test]
    fn computer_actor_rider_init_2_matches() {
        check(ssx3_ee::s00::f_00108f88, computer_actor_rider_init_2, 60, &|c, seed| {
            let mut r = Rng(0x108f_8800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..16 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn computer_actor_rider_init_difficulty_2_matches() {
        check(ssx3_ee::s00::f_0010cab8, computer_actor_rider_init_difficulty_2, 60, &|c, seed| {
            let mut r = Rng(0x10ca_b800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..16 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    /// Set up the world chain so `ComputerActorRider_GetHumanRider` returns a
    /// rider object, plus the rider/list floats the two "far" predicates read.
    fn far_setup(c: &mut Cpu, seed: u32, threshold_off: u32) {
        let mut r = Rng(0xfab0_0000 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        c.g[GP][0] = GAME_GP as u64;
        let world = GAME_GP.wrapping_sub(0x848);
        let w = DATA + 0x700;
        let x = DATA + 0x740;
        let y = DATA + 0x780;
        let v = DATA + 0x7c0;
        let human = DATA + 0x400;
        let list = DATA + 0x900;
        c.bus.write32(world, w);
        c.bus.write32(w + 0x84, x);
        c.bus.write32(x + 0xc, y);
        c.bus.write32(y, v);
        c.bus.write32(v + 0x28, human);
        c.bus.write32(DATA + 0x18, list);
        // A zero first word makes FindHumanRiderIndex fail -> GetHumanRider 0.
        c.bus.write32(list, if seed % 4 == 0 { 0 } else { 1 });
        c.bus.write32(list + 4, if seed % 4 == 0 { 0 } else { 1 });
        c.bus.write32(human + 0x4d0, r.edge_f32());
        c.bus.write32(list + 0x4d0, r.edge_f32());
        c.bus.write32(list + threshold_off, r.edge_f32());
    }

    #[test]
    fn computer_actor_rider_is_far_behind_human_matches() {
        check(ssx3_ee::s00::f_0010d938, computer_actor_rider_is_far_behind_human, 120, &|c, seed| {
            far_setup(c, seed, 0xdc);
        });
    }

    #[test]
    fn computer_actor_rider_is_far_ahead_of_human_matches() {
        check(ssx3_ee::s00::f_0010d990, computer_actor_rider_is_far_ahead_of_human, 120, &|c, seed| {
            far_setup(c, seed, 0xe0);
        });
    }

    #[test]
    fn ai_reset_rider_relations_2_matches() {
        check(ssx3_ee::s00::f_0010f398, ai_reset_rider_relations_2, 60, &|c, seed| {
            let mut r = Rng(0x10f3_9800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..16 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn rider_base_get_main_state_name_matches() {
        check(ssx3_ee::s00::f_00111830, rider_base_get_main_state_name, 200, &|c, seed| {
            let mut r = Rng(0x1118_3000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let state = match seed % 5 {
                0 => 0,
                1 => 6,
                2 => 7,
                3 => 0xffff_ffff,
                _ => r.next_u32(),
            };
            c.bus.write32(DATA + 0xde0, state);
            for i in 0..7 {
                c.bus.write32(0x0045_9d70 + i * 4, r.next_u32());
            }
            c.bus.write32(0x0045_6c48, r.next_u32());
        });
    }

    #[test]
    fn rider_base_get_sub_state_name_matches() {
        check(ssx3_ee::s00::f_00111860, rider_base_get_sub_state_name, 200, &|c, seed| {
            let mut r = Rng(0x1118_6000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let state = match seed % 5 {
                0 => 0,
                1 => 14,
                2 => 15,
                3 => 0xffff_ffff,
                _ => r.next_u32(),
            };
            c.bus.write32(DATA + 0xde4, state);
            for i in 0..15 {
                c.bus.write32(0x0045_9dc8 + i * 4, r.next_u32());
            }
            c.bus.write32(0x0045_6c48, r.next_u32());
        });
    }

    #[test]
    fn air_predictor_ctor_matches() {
        check(ssx3_ee::s00::f_00113138, air_predictor_ctor, 60, &|c, seed| {
            let mut r = Rng(0x1131_3800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..8 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }
}
