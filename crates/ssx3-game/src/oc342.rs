//! Staging module oc342 (wave wave215): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc342`.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
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
];

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
fn oc342_fade_tail(c: &mut Cpu, list_reg: usize, rets: [u32; 2]) -> Flow {
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
fn oc342_bank_id(c: &mut Cpu, bank_reg: usize, addr_reg: usize, key: u32) {
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
fn oc342_c_le_s(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
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
fn oc342_seq_end_replace(c: &mut Cpu, anim_id: u64, ret: u32) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, elf_image, Rng, DATA, GAME_GP};
    use crate::game::animation::rideranim::tests::{
        install_seq_end_fakes, replace_sequence_setup, CTRL, MAGIC,
    };

    /// A random 16-byte aligned pointer in the low half of the test data
    /// area (far from the fakes' call log at DATA+0x100000 and from CTRL).
    fn oc342_ptr(r: &mut Rng) -> u32 {
        DATA + (r.next_u32() & 0x7_fff0)
    }

    /// A float that is 0, an edge case or in [-range, range].
    fn oc342_float(r: &mut Rng, range: f32) -> u32 {
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
    fn oc342_seq_fixture(c: &mut Cpu, r: &mut Rng, keys: std::ops::Range<u32>) -> (u32, u32) {
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
}
