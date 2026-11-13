//! Staging module oc323 (wave wave213): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc323`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0029_0c40,
        ssx_audio_system_get_rider_speed_volume,
        "SSXAudioSystem_GetRiderSpeedVolume",
    ),
    (
        0x0037_0c60,
        dynamic_emitter_alloc_data,
        "DynamicEmitter_AllocData",
    ),
    (
        0x0031_2b18,
        anim_controller_replace_sequence,
        "AnimController_ReplaceSequence",
    ),
    (0x003c_e0b8, snd_utk_set_state, "SND_UtkSetState"),
    (
        0x003d_7d58,
        spch_purge_channel_requests,
        "SPCH_PurgeChannelRequests",
    ),
    (0x0031_78e0, rand_save_game_state, "Rand_SaveGameState"),
    (
        0x002d_19e8,
        wscript_man_try_run_kill_script,
        "WScriptMan_TryRunKillScript",
    ),
    (
        0x0035_2230,
        mesh_anim_randomize_nodes,
        "MeshAnim_RandomizeNodes",
    ),
    (
        0x0010_4a60,
        rider_anim_seq_end_type02,
        "RiderAnim_SeqEndType02",
    ),
    (
        0x002b_1348,
        speech_system_stream_wait_callback,
        "SpeechSystem_StreamWaitCallback",
    ),
];

// ---------------------------------------------------------------------------
// SSXAudioSystem_GetRiderSpeedVolume (0x290c40)

/// The rider's physics object (0x6c0 into the rider).
pub(crate) const RIDER_SPEED_VOLUME_BODY: u32 = 0x6c0;

/// `int SSXAudioSystem_GetRiderSpeedVolume(SSXAudioSystem* self, Rider* rider)`
/// (0x290c40): the magnitude of the rider's speed (the vector returned by the
/// physics object's virtual at slot 0x10) through audio curve table 3,
/// clamped to the MIDI volume range 0..127. No args, no locals beyond `ra`.
pub fn ssx_audio_system_get_rider_speed_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        // The physics object's virtual speed vector.
        c.g[V1][0] = Addr(r32(c, A1).wrapping_add(RIDER_SPEED_VOLUME_BODY)).lw(c);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(RIDER_SPEED_VOLUME_BODY));
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x10)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x14)).lw(c);
        let method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0029_0c60)?;
        // The speed's magnitude on VU0; Q into f12.
        c.lqc2(3, r32(c, V0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 12);
        // Curve table 3 at that speed.
        c.g[A0][0] = 0x3;
        Callee {
            addr: 0x0029_0c10,
            f: ee::s07::Audio_EvalCurveTable,
        }
        .call(c, 0x0029_0c90)?;
        c.cvt_w_s(1, 0);
        c.mfc1(V0, 1);
        // Clamp to 0..127 (the min/max the compiler emitted: `movn` pairs).
        c.g[A1][0] = 0x7f;
        c.g[V1][0] = u64::MAX;
        frame.restore_ra(c, 0);
        c.g[A0][0] = ((c.g[V0][0] as i64) < 128) as u64;
        if c.g[A0][0] == 0 {
            c.g[V0][0] = c.g[A1][0];
        }
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0;
        }
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// DynamicEmitter_AllocData (0x370c60)

/// Emitter: element count (+0x178), particle array (+0x1a0), state array
/// (+0x1a4).
pub(crate) const EMITTER_COUNT: u32 = 0x178;

pub(crate) const EMITTER_PARTICLES: u32 = 0x1a0;

pub(crate) const EMITTER_STATES: u32 = 0x1a4;

/// The `new (tag, 0, 0)` allocation tags of the two arrays.
pub(crate) const EMITTER_PARTICLE_TAG: u64 = 0x0049_2650;

pub(crate) const EMITTER_STATE_TAG: u64 = 0x0049_2670;

/// `operator_new_named` (0x317e30, cross-shard).
pub(crate) const EMITTER_NEW_NAMED: u32 = 0x0031_7e30;

/// `void DynamicEmitter_AllocData(DynamicEmitter* self)` (0x370c60): allocate
/// the emitter's particle array (`count * 16` bytes, tag 0x492650) and its
/// state array (tag 0x492670) at `count` elements (`+0x178`), storing them at
/// `+0x1a0`/`+0x1a4`. Frame 0x40 (s0 = self, s1 = array slot, s2 = count).
pub fn dynamic_emitter_alloc_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[A1][0] = EMITTER_PARTICLE_TAG;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x30);
        c.g[A3][0] = 0;
        c.g[S0][0] = Addr(r32(c, S1).wrapping_add(EMITTER_COUNT)).lw(c);
        c.g[S2][0] = sx(r32(c, S1).wrapping_add(EMITTER_PARTICLES));
        c.g[A0][0] = sx(r32(c, S0) << 4);
        call_ptr(c, EMITTER_NEW_NAMED, 0x0037_0c98)?;
        // The compiler's array-new tail: `count - 1` (the `n - 1` of an
        // inlined element loop), and a `-1` marker when the count is not 0.
        c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
        if r32(c, S0) != 0 {
            c.cycles += 2;
            c.g[A0][0] = u64::MAX;
        }
        c.cycles += 11;
        Addr(r32(c, S2)).sw(c, c.g[V0][0]);
        c.g[A1][0] = EMITTER_STATE_TAG;
        c.g[A2][0] = 0;
        c.g[S0][0] = Addr(r32(c, S1).wrapping_add(EMITTER_COUNT)).lw(c);
        c.g[A3][0] = 0;
        c.g[S1][0] = sx(r32(c, S1).wrapping_add(EMITTER_STATES));
        c.g[A0][0] = sx(r32(c, S0) << 4);
        call_ptr(c, EMITTER_NEW_NAMED, 0x0037_0ccc)?;
        c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
        if r32(c, S0) != 0 {
            c.cycles += 1;
            c.g[A0][0] = u64::MAX;
        }
        c.cycles += 7;
        Addr(r32(c, S1)).sw(c, c.g[V0][0]);
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

/// `void* AnimSeqList_Remove` (0x3145f8).
pub(crate) const ANIM_SEQ_LIST_REMOVE: Callee = Callee {
    addr: 0x0031_45f8,
    f: ee::s10::AnimSeqList_Remove,
};

/// `AnimController_PlayAnim` (0x3128e8).
pub(crate) const ANIM_CONTROLLER_PLAY_ANIM: Callee = Callee {
    addr: 0x0031_28e8,
    f: ee::s10::AnimController_PlayAnim,
};

/// `AnimSeqList_GetAt` (0x314760).
pub(crate) const ANIM_SEQ_LIST_GET_AT: Callee = Callee {
    addr: 0x0031_4760,
    f: ee::s10::AnimSeqList_GetAt,
};

/// `AnimSequence_SetWeightFade` (0x313a10).
pub(crate) const ANIM_SEQUENCE_SET_WEIGHT_FADE: Callee = Callee {
    addr: 0x0031_3a10,
    f: ee::s10::AnimSequence_SetWeightFade,
};

/// `int AnimController_ReplaceSequence(RiderAnimBase* self,
/// cAnimSequencer* seq, AnimNode* node, int anim)` (0x312b18): take the
/// node's current weight, fade weight and fade target, drop `node` from the
/// sequencer's list, start `anim` at weight -1 (keep the current weight),
/// then give the new sequence (index 0) the node's weight and, if the node's
/// target is not 0, fade it from the node's fade weight to that target.
/// Frame 0x60 (s0 = seq, s1 = self, s2 = anim, f20..f22 saved).
pub fn anim_controller_replace_sequence(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A3][0];
        frame.save_fpr(c, 22, 0x50);
        c.g[A1][0] = c.g[A2][0];
        frame.save_fpr(c, 21, 0x48);
        c.g[A0][0] = c.g[S0][0];
        frame.save_fpr(c, 20, 0x40);
        c.lwc1(20, r32(c, A2).wrapping_add(ANIM_NODE_WEIGHT));
        c.lwc1(21, r32(c, A2).wrapping_add(ANIM_NODE_FADE_TARGET));
        c.lwc1(22, r32(c, A2).wrapping_add(ANIM_NODE_FADE_WEIGHT));
        ANIM_SEQ_LIST_REMOVE.call(c, 0x0031_2b5c)?;
        // `cRiderAnimBase_play(self, anim, 0, -1.0f)`.
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 12);
        c.g[A2][0] = 0;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        ANIM_CONTROLLER_PLAY_ANIM.call(c, 0x0031_2b74)?;
        // The new sequence (index 0) takes the node's weight.
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        ANIM_SEQ_LIST_GET_AT.call(c, 0x0031_2b80)?;
        c.mtc1(0, 0);
        c.swc1(20, r32(c, V0).wrapping_add(ANIM_NODE_WEIGHT));
        c.c_eq_s(21, 0);
        let no_target = c.fcc();
        c.swc1(20, r32(c, V0).wrapping_add(ANIM_NODE_FADE_WEIGHT));
        if !no_target {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.mov_s(12, 22);
            c.mov_s(13, 21);
            ANIM_SEQUENCE_SET_WEIGHT_FADE.call(c, 0x0031_2ba8)?;
        }
        c.cycles += 10;
        c.g[V0][0] = c.g[S2][0];
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

// ---------------------------------------------------------------------------
// SND_UtkSetState (0x3ce0b8)

/// The SND state word (+0x4) and the two state fields (+0xd5c, +0xd44).
pub(crate) const UTK_STATE: u32 = 0x4;

pub(crate) const UTK_FIELD_D5C: u32 = 0xd5c;

pub(crate) const UTK_FIELD_D44: u32 = 0xd44;

/// `void SND_UtkSetState(Snd* snd, const SndUtkState* st)` (0x3ce0b8): copy
/// the toolkit's state block (three words at +0/+4/+8) into the sound
/// system's state fields.
pub fn snd_utk_set_state(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = Addr(r32(c, A1)).lw(c);
    Addr(r32(c, A0).wrapping_add(UTK_STATE)).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(r32(c, A1).wrapping_add(0x8)).lw(c);
    Addr(r32(c, A0).wrapping_add(UTK_FIELD_D5C)).sw(c, c.g[V1][0]);
    c.g[V0][0] = Addr(r32(c, A1).wrapping_add(0x4)).lw(c);
    Addr(r32(c, A0).wrapping_add(UTK_FIELD_D44)).sw(c, c.g[V0][0]);
    ret(c);
}

// ---------------------------------------------------------------------------
// SPCH_PurgeChannelRequests (0x3d7d58)

/// The channel slot table `D_0044FFF8`: 16 slots 0x60 bytes apart at
/// `+0x64 + 0x60*i` (so the fields sit at `0x64..0x70` of each slot record,
/// i.e. `0x00..0x0c` of the element). The word at `+0x60` is the "another
/// request is pending" flag the purge sets.
pub(crate) const SPCH_SLOT_TABLE: u32 = 0x0044_fff8;

pub(crate) const SPCH_SLOT_STRIDE: u32 = 0x60;

/// Slot: the request's serial (`f0`), the tie-break halfword, the active byte,
/// the channel byte, the 16-bit state (0 = idle), the 16-bit link handle and
/// the queue entry pointer.
pub(crate) const SPCH_SLOT_F0: u32 = 0x64;

pub(crate) const SPCH_SLOT_H: u32 = 0x68;

pub(crate) const SPCH_SLOT_ACTIVE: u32 = 0x6a;

pub(crate) const SPCH_SLOT_CHANNEL: u32 = 0x6b;

pub(crate) const SPCH_SLOT_S8: u32 = 0x6c;

#[cfg(test)]
pub(crate) const SPCH_SLOT_LINK: u32 = 0x6e;

pub(crate) const SPCH_SLOT_QUEUE: u32 = 0x70;

/// The pending flag, right before element 0.
pub(crate) const SPCH_PENDING: u32 = 0x60;

/// Queue entry: the signed status byte at +9 (100 = held) and the flags byte
/// at +0xa (bit 2 = queued).
pub(crate) const SPCH_QUEUE_STATUS: u32 = 0x9;

pub(crate) const SPCH_QUEUE_FLAGS: u32 = 0xa;

/// `SPCH_FreeQueueSlot` (0x3d7c38).
pub(crate) const SPCH_FREE_QUEUE_SLOT: Callee = Callee {
    addr: 0x003d_7c38,
    f: ee::s14::SPCH_FreeQueueSlot,
};

/// `void SPCH_PurgeChannelRequests(int idx)` (0x3d7d58): clear the pending
/// flag, then for all 16 channel slots except `idx`'s, free the queue slot of
/// every idle entry (same channel, not playing) whose request is older than
/// `idx`'s (serial, then tie-break halfword) and not marked queued; when an
/// entry is not older and its queue entry is held (status 100) on the same
/// channel, set the pending flag instead. Frame 0xa0 (s0-s7, fp).
pub fn spch_purge_channel_requests(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S6, 0x60);
        c.g[V1][0] = SPCH_SLOT_STRIDE as u64;
        c.g[V0][0] = 0x0045_0000;
        c.g[S6][0] = c.g[A0][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(8));
        // `v1 = idx * 0x60`: the compiled form of `&r[idx]`.
        c.mult(0, V1, S6, V1);
        frame.save(c, FP, 0x80);
        frame.save(c, S5, 0x50);
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(SPCH_SLOT_F0));
        frame.save(c, S2, 0x20);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[S2][0] = sx(r32(c, V0).wrapping_add(SPCH_SLOT_QUEUE));
        frame.save(c, S0, 0);
        c.g[A1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        frame.save(c, RA, 0x90);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
        frame.save(c, S7, 0x70);
        c.g[S1][0] = 0;
        frame.save(c, S4, 0x40);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x30);
        c.g[FP][0] = 1;
        // The reference slot's channel, serial and tie-break.
        c.g[S3][0] = Addr(r32(c, A1).wrapping_add(SPCH_SLOT_CHANNEL)).lbu(c);
        c.g[S4][0] = Addr(r32(c, V1)).lw(c);
        c.g[S7][0] = Addr(r32(c, A1).wrapping_add(SPCH_SLOT_H)).lhu(c);
        Addr(r32(c, V0).wrapping_add(SPCH_PENDING)).sw(c, 0);
        // s1 = i.
        loop {
            c.cycles += 2;
            c.g[V0][0] = 0x0045_0000;
            // The two exits that rejoin after the block charge one more
            // cycle: `i == idx`, and the "pending" case.
            let mut late_exit = false;
            if c.g[S1][0] == c.g[S6][0] {
                late_exit = true;
            } else {
                c.cycles += 5;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(8));
                c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(SPCH_SLOT_ACTIVE)).lbu(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(SPCH_SLOT_CHANNEL)).lbu(c);
                    if c.g[V0][0] == c.g[S3][0] {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(SPCH_SLOT_S8)).lh(c);
                        if c.g[V0][0] == 0 {
                            // Older than the reference request?
                            c.cycles += 4;
                            c.g[A0][0] = Addr(r32(c, S5)).lw(c);
                            c.g[V0][0] = (c.g[A0][0] < c.g[S4][0]) as u64;
                            let older = c.g[V0][0] != 0;
                            c.g[V1][0] = Addr(r32(c, V1).wrapping_add(SPCH_SLOT_H)).lhu(c);
                            let free = if older {
                                true
                            } else {
                                c.cycles += 2;
                                if c.g[A0][0] != c.g[S4][0] {
                                    c.g[V1][0] = Addr(r32(c, S2)).lw(c);
                                    false
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = (c.g[V1][0] < c.g[S7][0]) as u64;
                                    if c.g[V0][0] == 0 {
                                        c.g[V1][0] = Addr(r32(c, S2)).lw(c);
                                        false
                                    } else {
                                        true
                                    }
                                }
                            };
                            if free {
                                c.cycles += 9;
                                // `v1 = 0x450000 + 0x68`: `&r[i].queue`.
                                c.g[V1][0] = (SPCH_SLOT_TABLE + SPCH_SLOT_QUEUE) as u64;
                                c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
                                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                                c.g[V1][0] = Addr(r32(c, A0).wrapping_add(SPCH_QUEUE_FLAGS)).lbu(c);
                                c.g[V1][0] = sx(r32(c, V1) >> 2);
                                c.g[V1][0] &= 1;
                                if c.g[V1][0] == 0 {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S1][0];
                                    SPCH_FREE_QUEUE_SLOT.call(c, 0x003d_7e4c)?;
                                }
                            } else {
                                // Not older: is it held on this channel?
                                c.cycles += 6;
                                c.g[V0][0] = 0x0045_0000;
                                c.g[A0][0] = sx(r32(c, V0).wrapping_sub(8));
                                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(SPCH_QUEUE_STATUS)).lb(c);
                                c.g[V1][0] = 0x64;
                                if c.g[V0][0] == c.g[V1][0] {
                                    c.cycles += 4;
                                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                                    c.g[V1][0] =
                                        Addr(r32(c, V0).wrapping_add(SPCH_SLOT_CHANNEL)).lbu(c);
                                    if c.g[V1][0] == c.g[S3][0] {
                                        Addr(r32(c, A0).wrapping_add(SPCH_PENDING))
                                            .sw(c, c.g[FP][0]);
                                    }
                                    late_exit = true;
                                }
                            }
                        }
                    }
                }
            }
            if late_exit {
                c.cycles += 1;
            }
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            // i++ over the three parallel pointers.
            c.cycles += 5;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(SPCH_SLOT_STRIDE));
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(SPCH_SLOT_STRIDE));
            c.g[V0][0] = ((c.g[S1][0] as i64) < 16) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S5][0] = sx(r32(c, S5).wrapping_add(SPCH_SLOT_STRIDE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_7dc8);
            }
        }
        c.cycles += 12;
        frame.restore(c, RA, 0x90);
        frame.restore(c, FP, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rand_SaveGameState (0x3178e0)

/// The game RNG's 0x18-byte state block.
pub(crate) const RAND_STATE: u32 = 0x004f_f030;

pub(crate) const RAND_STATE_BYTES: u64 = 0x18;

/// `EA_MemCopy(dst, src, n)` (0x3e6574, cross-shard).
pub(crate) const RAND_MEM_COPY: u32 = 0x003e_6574;

/// `void Rand_SaveGameState(void* dst)` (0x3178e0): copy the game RNG's
/// 0x18-byte state (`0x4ff030`) into `dst`. Frame 0x10.
pub fn rand_save_game_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        // `a1 = 0x500000 - 0xfd0`.
        c.g[A1][0] = RAND_STATE as u64;
        frame.save_ra(c, 0);
        c.g[A2][0] = RAND_STATE_BYTES;
        call_ptr(c, RAND_MEM_COPY, 0x0031_78f8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptMan_TryRunKillScript (0x2d19e8)

/// The script manager pointer global (`D_004A3DD8`), at `gp + 0xce8`.
pub(crate) const WSCRIPT_MANAGER_GP: u32 = 0xce8;

/// `WScriptMan_HasKillScript` (0x30a598, cross-shard).
pub(crate) const WSCRIPT_HAS_KILL_SCRIPT: u32 = 0x0030_a598;

/// `WScriptMan_RunKillScript` (0x30a5c0, cross-shard).
pub(crate) const WSCRIPT_RUN_KILL_SCRIPT: u32 = 0x0030_a5c0;

/// `int WScriptMan_TryRunKillScript(int id)` (0x2d19e8): when the script
/// manager (`*(gp + 0xce8)`) has a kill script for `id`, run it and return 1,
/// else return 0. Frame 0x20 (s0 = id).
pub fn wscript_man_try_run_kill_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(WSCRIPT_MANAGER_GP)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, WSCRIPT_HAS_KILL_SCRIPT, 0x002d_1a04)?;
        let has_script = c.g[V0][0] != 0;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(WSCRIPT_MANAGER_GP)).lw(c);
        if has_script {
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, WSCRIPT_RUN_KILL_SCRIPT, 0x002d_1a1c)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MeshAnim_RandomizeNodes (0x352230)

/// The debris/mesh-anim instance: model at +0x18, spin scale at +0x20, the
/// per-axis spin array at +0x330, the per-axis (normalized) axes at +0x1b0,
/// vmax[3] at +0x390, vbase at +0x3a0 and the per-axis velocities at +0x530.
pub(crate) const MESH_MODEL: u32 = 0x18;

pub(crate) const MESH_SPIN_SCALE: u32 = 0x20;

pub(crate) const MESH_AXIS: u32 = 0x1b0;

pub(crate) const MESH_SPIN: u32 = 0x330;

pub(crate) const MESH_VMAX: u32 = 0x390;

pub(crate) const MESH_VBASE: u32 = 0x3a0;

pub(crate) const MESH_VEL: u32 = 0x530;

/// The model's node-list header (+0x80); its +4 is the node count.
pub(crate) const MESH_NODE_LIST: u32 = 0x80;

pub(crate) const MESH_NODE_COUNT: u32 = 0x4;

/// The frame-rate struct `D_004A5B64` at `gp + 0x2a74`: the per-frame divisor
/// at +0x10 and the velocity scale at +0x14.
pub(crate) const MESH_FRAME_RATE_GP: u32 = 0x2a74;

/// `Rand_NextMain` / `BXrand` (0x3177f0).
pub(crate) const MESH_RAND: u32 = 0x0031_77f0;

/// `void MeshAnim_RandomizeNodes(MeshAnim* self)` (0x352230): for every node
/// of the model, give the debris a random spin (`spinScale * rand[0.5,1)`)
/// and a random unit axis (`rand[-1,1)` in each of x, y, z, normalized); then
/// for every node give it a velocity: `vbase * scale` plus a random
/// `[-vmax, vmax) / frameCount` in each axis. Frame 0xd0 (s0-s6, f20-f24).
pub fn mesh_anim_randomize_nodes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S2, 0x70);
        frame.save(c, S5, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S6, 0x30);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 24, 0xc0);
        frame.save_fpr(c, 23, 0xb8);
        frame.save_fpr(c, 22, 0xb0);
        frame.save_fpr(c, 21, 0xa8);
        frame.save_fpr(c, 20, 0xa0);
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(MESH_MODEL)).lw(c);
        c.g[S6][0] = Addr(r32(c, V0).wrapping_add(MESH_NODE_LIST)).lw(c);
        c.g[V1][0] = Addr(r32(c, S6).wrapping_add(MESH_NODE_COUNT)).lw(c);
        let no_nodes = (c.g[V1][0] as i64) <= 0;
        c.g[S5][0] = 0;
        if !no_nodes {
            // f24 = 0.5, f22 = 1.0, f23 = -1.0; s1 = 23-bit mask, s4 = 1.0f.
            c.cycles += 12;
            c.g[S1][0] = 0x7f_0000;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(AT, 24);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 22);
            c.g[S1][0] |= 0xffff;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(AT, 23);
            c.g[S4][0] = 0x3f80_0000;
            c.g[S0][0] = c.g[S2][0];
            c.g[S3][0] = sx(r32(c, S2).wrapping_add(MESH_SPIN));
            loop {
                c.cycles += 56;
                c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
                call_ptr(c, MESH_RAND, 0x0035_22b8)?;
                // spin[i] = spinScale * (0.5 + 0.5 * rand[0,1)).
                c.g[V0][0] &= c.g[S1][0];
                c.lwc1(1, r32(c, S2).wrapping_add(MESH_SPIN_SCALE));
                c.g[V0][0] |= c.g[S4][0];
                c.mtc1(V0, 2);
                c.sub_s(0, 2, 22);
                c.mul_s(0, 0, 24);
                c.add_s(0, 0, 24);
                c.mul_s(1, 1, 0);
                c.swc1(1, r32(c, S3));
                call_ptr(c, MESH_RAND, 0x0035_22e0)?;
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(4));
                // y = 2 * rand[0,1) - 1.
                c.g[V0][0] &= c.g[S1][0];
                c.g[V0][0] |= c.g[S4][0];
                c.mtc1(V0, 3);
                c.sub_s(21, 3, 22);
                c.add_s(21, 21, 21);
                c.add_s(21, 21, 23);
                call_ptr(c, MESH_RAND, 0x0035_2300)?;
                // z.
                c.g[V0][0] &= c.g[S1][0];
                c.g[V0][0] |= c.g[S4][0];
                c.mtc1(V0, 0);
                c.sub_s(20, 0, 22);
                c.add_s(20, 20, 20);
                c.add_s(20, 20, 23);
                call_ptr(c, MESH_RAND, 0x0035_231c)?;
                // x; the vector goes to the stack in (y, z, x, 0) order.
                c.g[V0][0] &= c.g[S1][0];
                c.swc1(21, r32(c, SP));
                c.g[V0][0] |= c.g[S4][0];
                c.swc1(20, r32(c, SP).wrapping_add(4));
                c.mtc1(V0, 1);
                c.sw(r32(c, SP).wrapping_add(0xc), 0);
                c.sub_s(0, 1, 22);
                c.add_s(0, 0, 0);
                c.add_s(0, 0, 23);
                c.swc1(0, r32(c, SP).wrapping_add(8));
                c.lq(2, r32(c, SP));
                c.sq(2, r32(c, S0).wrapping_add(MESH_AXIS));
                // Normalize on VU0 and store back.
                c.lqc2(3, r32(c, S0).wrapping_add(MESH_AXIS));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, r32(c, SP).wrapping_add(0x10));
                c.lq(2, r32(c, SP).wrapping_add(0x10));
                c.sq(2, r32(c, S0).wrapping_add(MESH_AXIS));
                c.sq(2, r32(c, SP));
                c.g[V0][0] = Addr(r32(c, S6).wrapping_add(MESH_NODE_COUNT)).lw(c);
                c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_22b0);
                }
            }
        }
        // Velocities.
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S6).wrapping_add(MESH_NODE_COUNT)).lw(c);
        let no_nodes = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = 0;
        if !no_nodes {
            c.cycles += 7;
            c.g[S1][0] = 0x7f_0000;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 23);
            c.g[S1][0] |= 0xffff;
            c.g[S4][0] = 0x3f80_0000;
            c.g[S0][0] = c.g[S2][0];
            loop {
                // vx = -vmax[0] .. vmax[0], scaled to a per-frame step.
                c.cycles += 16;
                c.lwc1(20, r32(c, S2).wrapping_add(MESH_VMAX));
                call_ptr(c, MESH_RAND, 0x0035_23c0)?;
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(MESH_FRAME_RATE_GP)).lw(c);
                c.g[V0][0] &= c.g[S1][0];
                c.g[V0][0] |= c.g[S4][0];
                c.lwc1(0, r32(c, A0).wrapping_add(0x10));
                c.cvt_s_w(0, 0);
                c.mtc1(V0, 2);
                c.div_s(0, 23, 0);
                c.sub_s(1, 2, 23);
                c.mul_s(20, 20, 1);
                c.mul_s(22, 20, 0);
                call_ptr(c, MESH_RAND, 0x0035_23ec)?;
                c.g[V0][0] &= 1;
                if c.g[V0][0] != 0 {
                    c.neg_s(22, 22);
                }
                // vy.
                c.cycles += 16;
                c.lwc1(20, r32(c, S2).wrapping_add(MESH_VMAX + 4));
                call_ptr(c, MESH_RAND, 0x0035_2400)?;
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(MESH_FRAME_RATE_GP)).lw(c);
                c.g[V0][0] &= c.g[S1][0];
                c.g[V0][0] |= c.g[S4][0];
                c.lwc1(0, r32(c, A0).wrapping_add(0x10));
                c.cvt_s_w(0, 0);
                c.mtc1(V0, 3);
                c.div_s(0, 23, 0);
                c.sub_s(1, 3, 23);
                c.mul_s(20, 20, 1);
                c.mul_s(21, 20, 0);
                call_ptr(c, MESH_RAND, 0x0035_242c)?;
                c.g[V0][0] &= 1;
                if c.g[V0][0] != 0 {
                    c.neg_s(21, 21);
                }
                // vz, then vel[j] = vbase * scale + (vx, vy, vz, 0).
                c.cycles += 34;
                c.lwc1(20, r32(c, S2).wrapping_add(MESH_VMAX + 8));
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                call_ptr(c, MESH_RAND, 0x0035_2444)?;
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(MESH_FRAME_RATE_GP)).lw(c);
                c.g[V0][0] &= c.g[S1][0];
                c.g[V0][0] |= c.g[S4][0];
                c.lwc1(1, r32(c, V1).wrapping_add(0x10));
                c.cvt_s_w(1, 1);
                c.lwc1(0, r32(c, V1).wrapping_add(0x14));
                c.mtc1(V0, 3);
                c.mfc1(V1, 0);
                c.lqc2(4, r32(c, S2).wrapping_add(MESH_VBASE));
                c.qmtc2(V1, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, r32(c, SP).wrapping_add(0x10));
                c.lq(2, r32(c, SP).wrapping_add(0x10));
                c.sub_s(2, 3, 23);
                c.div_s(1, 23, 1);
                c.sq(2, r32(c, SP));
                c.sq(2, r32(c, S0).wrapping_add(MESH_VEL));
                c.mul_s(20, 20, 2);
                c.swc1(22, r32(c, SP));
                c.swc1(21, r32(c, SP).wrapping_add(4));
                c.sw(r32(c, SP).wrapping_add(0xc), 0);
                c.mul_s(20, 20, 1);
                c.swc1(20, r32(c, SP).wrapping_add(8));
                c.lqc2(3, r32(c, S0).wrapping_add(MESH_VEL));
                c.lqc2(4, r32(c, SP));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, r32(c, S0).wrapping_add(MESH_VEL));
                c.g[V0][0] = Addr(r32(c, S6).wrapping_add(MESH_NODE_COUNT)).lw(c);
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_23b8);
                }
            }
        }
        c.cycles += 15;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 24, 0xc0);
        frame.restore_fpr(c, 23, 0xb8);
        frame.restore_fpr(c, 22, 0xb0);
        frame.restore_fpr(c, 21, 0xa8);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

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
fn seq_end_replace(c: &mut Cpu, ret: u32, anim: u32) -> crate::call::Flow {
    c.cycles += 5;
    c.g[A0][0] = c.g[A3][0];
    c.g[A3][0] = anim as u64;
    call_ptr(c, SEQ_END_REPLACE, ret)?;
    c.g[RA][0] = c.ld(sp(c));
    Ok(())
}

// ---------------------------------------------------------------------------
// SpeechSystem_StreamWaitCallback (0x2b1348)

/// `SND` request message: the entry pointer at +0x18, the request id at
/// +0x1c and the timer id at +0x20.
pub(crate) const STREAM_MSG_ENTRY: u32 = 0x18;

pub(crate) const STREAM_MSG_REQUEST: u32 = 0x1c;

pub(crate) const STREAM_MSG_TIMER: u32 = 0x20;

/// The request entry: the stream slot at +0, the stream index at +4.
#[cfg(test)]
pub(crate) const STREAM_ENTRY_SLOT: u32 = 0;

pub(crate) const STREAM_ENTRY_INDEX: u32 = 0x4;

/// `D_004A37B4` (the SND manager's `**`) at `gp + 0x6c4`.
pub(crate) const STREAM_SND_GP: u32 = 0x6c4;

/// The bank monitor's timer object at +0x1d8 of the manager.
pub(crate) const STREAM_MONITOR: u32 = 0x1d8;

/// The timer descriptor `D_004A37D8` (8 bytes, loaded unaligned).
pub(crate) const STREAM_TIMER_DESC: u32 = 0x004a_37d8;

/// Callees.
pub(crate) const STREAM_ARRAY_GET_STATE: u32 = 0x002a_ae08;

pub(crate) const STREAM_ARRAY_IS_PLAYING: u32 = 0x002a_b028;

pub(crate) const STREAM_ARRAY_TIME_LEFT: u32 = 0x002a_ae78;

pub(crate) const STREAM_ARRAY_CURRENT: u32 = 0x002a_b150;

pub(crate) const STREAM_MONITOR_ADD_TIMER: u32 = 0x002a_dca0;

pub(crate) const STREAM_INSTANCE_SLOT: u32 = 0x002b_1220;

pub(crate) const STREAM_START_CALLBACK: u32 = 0x002b_1268;

/// `int SpeechSystem_StreamWaitCallback(SpeechSystem* self, SndMsg* msg)`
/// (0x2b1348): when the stream of the message's entry is beyond its first
/// state: if it is playing, add a bank-monitor timer for its remaining time
/// and return 1; otherwise, when the current request is the message's, run
/// the start callback. In every other case run the start callback (the first
/// state) or return 0. Frame 0x40 (s0 = self, s1 = entry, s2 = msg).
pub fn speech_system_stream_wait_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = Addr(r32(c, S2).wrapping_add(STREAM_MSG_ENTRY)).lw(c);
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = Addr(r32(c, S1).wrapping_add(STREAM_ENTRY_INDEX)).lw(c);
        call_ptr(c, STREAM_ARRAY_GET_STATE, 0x002b_1374)?;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        let first_state = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = c.g[S0][0];
        if !first_state {
            // Past the first state: playing (timer) or pending (compare)?
            c.cycles += 5;
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            c.g[A1][0] = Addr(r32(c, S1).wrapping_add(STREAM_ENTRY_INDEX)).lw(c);
            call_ptr(c, STREAM_ARRAY_IS_PLAYING, 0x002b_138c)?;
            if c.g[V0][0] == 0 {
                c.g[A1][0] = Addr(r32(c, S1)).lw(c);
            } else {
                c.cycles += 20;
                c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                c.g[A1][0] = Addr(r32(c, S1).wrapping_add(STREAM_ENTRY_INDEX)).lw(c);
                call_ptr(c, STREAM_ARRAY_TIME_LEFT, 0x002b_13a0)?;
                // `BankMonitor_AddTimer(**(gp+0x6c4) + 0x1d8, (int)time,
                // D_004A37D8[0].v, entry, msg->request, msg->timer, 0, 0)`.
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(STREAM_SND_GP)).lw(c);
                c.cvt_w_s(1, 0);
                c.mfc1(5, 1);
                c.g[V0][0] = 0x004a_0000;
                c.g[T1][0] = Addr(r32(c, S2).wrapping_add(STREAM_MSG_TIMER)).lw(c);
                c.g[T0][0] = Addr(r32(c, V1)).lw(c);
                c.g[A3][0] = c.g[S1][0];
                c.ldl(6, STREAM_TIMER_DESC + 7);
                c.ldr(6, STREAM_TIMER_DESC);
                c.g[T2][0] = 0;
                c.g[A0][0] = Addr(r32(c, T0)).lw(c);
                c.g[T3][0] = 0;
                c.g[T0][0] = Addr(r32(c, S2).wrapping_add(STREAM_MSG_REQUEST)).lw(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(STREAM_MONITOR));
                call_ptr(c, STREAM_MONITOR_ADD_TIMER, 0x002b_13dc)?;
                c.g[V0][0] = 1;
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                c.cycles += 6;
                frame.close(c);
                return Ok(());
            }
            // The request is still pending: is it the message's?
            c.cycles += 8;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(STREAM_SND_GP)).lw(c);
            call_ptr(c, STREAM_INSTANCE_SLOT, 0x002b_13ec)?;
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, STREAM_ARRAY_CURRENT, 0x002b_13f8)?;
            c.g[V1][0] = Addr(r32(c, S2).wrapping_add(STREAM_MSG_REQUEST)).lw(c);
            let not_ours = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 0;
            if !not_ours {
                c.cycles += 1;
                c.g[A0][0] = c.g[S0][0];
                speech_stream_start(c, frame)?;
                return Ok(());
            }
        } else {
            speech_stream_start(c, frame)?;
            return Ok(());
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        c.cycles += 6;
        frame.close(c);
        Ok(())
    })
}

/// The shared tail of `SpeechSystem_StreamWaitCallback`: run the start
/// callback and return its result.
fn speech_stream_start(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 2;
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, STREAM_START_CALLBACK, 0x002b_1410)?;
    c.cycles += 6;
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ssx_audio_system_get_rider_speed_volume_matches() {
        use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
        use crate::difftest::{check, put_vec, put_words, DATA};
        /// `D_00445898[3]`: the game's audio curve table 3 (byte-indexed;
        /// the 32-bit loads mask the low address bits, as the bus does).
        const CURVE: u32 = 0x0044_5898;
        check(
            ssx3_ee::s07::f_00290c40__generated,
            ssx_audio_system_get_rider_speed_volume,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0029_0c40, 1);
                let base = DATA + 0x2_0000;
                let (rider, vt) = (base, base + 0x1000);
                let (speed_vec, curve) = (base + 0x2000, CURVE);
                // The physics object's table slot: delta at +0x10, method at
                // +0x14 (a fake returning `speed_vec`).
                c.bus.write32(rider + RIDER_SPEED_VOLUME_BODY, vt);
                let delta = (r.next_u32() % 4) * 4;
                c.bus.write16(vt + 0x10, delta as u16);
                c.bus.write32(vt + 0x14, FAKE);
                c.bus.write32(LOG_RESULTS, speed_vec);
                // Curve table 3: times 0..4, values 0, -20, 40, 200, 300.
                put_words(
                    c,
                    curve,
                    &[
                        0.0f32.to_bits(),
                        1.0f32.to_bits(),
                        2.0f32.to_bits(),
                        3.0f32.to_bits(),
                        4.0f32.to_bits(),
                        0.0f32.to_bits(),
                        (-20.0f32).to_bits(),
                        40.0f32.to_bits(),
                        200.0f32.to_bits(),
                        300.0f32.to_bits(),
                    ],
                );
                // The speed's magnitude comes out of the VU0 reduction as the
                // vector's y (VF6 cleared); pick values that reach every
                // clamp: negative, in range and over 127.
                let speed = [0.0f32, 0.5, 1.5, 2.5, 3.5, 7.0][(seed % 6) as usize];
                c.bus.plat.set_vf(6, [0, 0, 0, 0]);
                put_vec(c, speed_vec, [0, speed.to_bits(), 0, 0]);
                c.g[A1][0] = rider as u64;
            },
        );
    }

    #[test]
    fn dynamic_emitter_alloc_data_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        use crate::game::be::beintreward::tests::world;
        use crate::game::bx::bxstring::tests::heap_init_4;
        use crate::game::bx::bxstringctor::tests::be_library;
        check(
            ssx3_ee::s12::f_00370c60__generated,
            dynamic_emitter_alloc_data,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0037_0c60, 0);
                be_library(c);
                world(c);
                heap_init_4(c);
                // The emitter: a junk element count and junk array slots
                // (some seeds empty, which allocates nothing).
                let emitter = DATA + 0x100;
                fill(c, &mut r, emitter + 0x170, 0x40);
                let count = if seed % 4 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 8
                };
                c.bus.write32(emitter + EMITTER_COUNT, count);
                c.g[A0][0] = emitter as u64;
            },
        );
    }

    #[test]
    fn snd_utk_set_state_matches() {
        use crate::difftest::{check, fill, scramble, Rng, DATA};
        check(
            ssx3_ee::s14::f_003ce0b8__generated,
            snd_utk_set_state,
            200,
            &|c, seed| {
                let mut r = Rng(0x3ce0b8 ^ seed as u64);
                scramble(c, &mut r);
                let (snd, state) = (DATA + 0x100, DATA + 0x200);
                // Junk in both structs, then the three source words.
                fill(c, &mut r, snd, 0x400);
                fill(c, &mut r, state, 4);
                c.g[A0][0] = snd as u64;
                c.g[A1][0] = state as u64;
            },
        );
    }

    #[test]
    fn spch_purge_channel_requests_matches() {
        use crate::difftest::{check, Rng, DATA};
        check(
            ssx3_ee::s14::f_003d7d58__generated,
            spch_purge_channel_requests,
            200,
            &|c, seed| {
                let mut r = Rng(0x003d_7d58 ^ seed as u64);
                // One slot is the purge's reference (`idx`, in a0); the other
                // 15 are compared against it.
                let idx = r.next_u32() % 16;
                let f0_ref = r.next_u32() & 0x0fff_ffff;
                let h_ref = r.next_u32() % 0x8000;
                let channel = (r.next_u32() % 4) as u8;
                let entries = DATA + 0x2000;
                for i in 0..16u32 {
                    let slot = SPCH_SLOT_TABLE + SPCH_SLOT_STRIDE * i;
                    let same = i == idx;
                    // Serials and tie-breaks straddle the reference (older,
                    // equal, newer).
                    let f0 = if same {
                        f0_ref
                    } else {
                        (f0_ref as i32 + (r.next_u32() % 3) as i32 - 1) as u32
                    };
                    let h = if same {
                        h_ref
                    } else {
                        (h_ref as i32 + (r.next_u32() % 3) as i32 - 1) as u32 & 0xffff
                    };
                    let chan = if r.next_u32() % 2 == 0 {
                        channel
                    } else {
                        (r.next_u32() % 4) as u8
                    };
                    // Some slots idle (active 0) or busy (state != 0).
                    let active = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 255
                    };
                    let state = if r.next_u32() % 3 == 0 { 1u32 } else { 0 };
                    c.bus.write32(slot + SPCH_SLOT_F0, f0);
                    c.bus.write16(slot + SPCH_SLOT_H, h as u16);
                    c.bus.write8(slot + SPCH_SLOT_ACTIVE, active as u8);
                    c.bus.write8(slot + SPCH_SLOT_CHANNEL, chan);
                    c.bus.write16(slot + SPCH_SLOT_S8, state as u16);
                    // Freeing the slot walks this link when it is not -1;
                    // -1 takes `SPCH_FreeQueueSlot`'s short path.
                    c.bus.write16(slot + SPCH_SLOT_LINK, 0xffff);
                    let entry = entries + 0x20 * i;
                    c.bus.write32(slot + SPCH_SLOT_QUEUE, entry);
                    c.bus.write8(
                        entry + SPCH_QUEUE_STATUS,
                        if r.next_u32() % 2 == 0 {
                            100
                        } else {
                            (r.next_u32() % 256) as u8
                        },
                    );
                    c.bus.write8(
                        entry + SPCH_QUEUE_FLAGS,
                        if r.next_u32() % 2 == 0 { 0x04 } else { 0 },
                    );
                }
                c.g[A0][0] = idx as u64;
            },
        );
    }

    #[test]
    fn rand_save_game_state_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        check(
            ssx3_ee::s10::f_003178e0__generated,
            rand_save_game_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_78e0, 0);
                // The RNG state and a junk destination (sometimes unaligned,
                // which takes EA_MemCopy's byte head).
                fill(c, &mut r, RAND_STATE, 6);
                let dst = DATA + 0x100 + (seed as u32 % 4);
                fill(c, &mut r, dst & !3, 8);
                c.g[A0][0] = dst as u64;
            },
        );
    }

    // -- WScriptMan_TryRunKillScript fake callees -------------------------
    //
    // `HasKillScript` is faked (logging through `difftest::fakes::fake` so the
    // argument dance is compared) to pick the "no script" and "run it" paths;
    // the world's script lookup is faked to "not found" (-1, which a forced
    // fake result cannot express) so `RunKillScript` takes its short path.

    fn wscript_fake_has_kill_script(c: &mut Cpu) {
        let ret = c.g[RA][0] as u32;
        crate::difftest::fakes::fake(c, WSCRIPT_HAS_KILL_SCRIPT, ret);
        // The control word: 0 = no kill script, else one.
        let control = c.bus.read32(crate::difftest::fakes::LOG_RESULTS);
        c.g[V0][0] = (control != 0) as u64;
    }

    fn wscript_fake_world_object(c: &mut Cpu) {
        c.g[V0][0] = u64::MAX;
        c.pc = c.g[RA][0] as u32;
    }

    /// Installs the fakes and puts the previous bodies back on drop.
    struct WscriptFakeSlots {
        has_kill_script: fn(&mut Cpu),
        world_object: fn(&mut Cpu),
    }

    impl WscriptFakeSlots {
        fn install() -> Self {
            unsafe {
                let saved = Self {
                    has_kill_script: ssx3_ee::s10::OVERRIDE_0030A598,
                    world_object: ssx3_ee::s13::OVERRIDE_003A6B78,
                };
                ssx3_ee::s10::OVERRIDE_0030A598 = wscript_fake_has_kill_script;
                ssx3_ee::s13::OVERRIDE_003A6B78 = wscript_fake_world_object;
                saved
            }
        }
    }

    impl Drop for WscriptFakeSlots {
        fn drop(&mut self) {
            unsafe {
                ssx3_ee::s10::OVERRIDE_0030A598 = self.has_kill_script;
                ssx3_ee::s13::OVERRIDE_003A6B78 = self.world_object;
            }
        }
    }

    #[test]
    fn wscript_man_try_run_kill_script_matches() {
        use crate::difftest::fakes::{machine, LOG_RESULTS};
        use crate::difftest::{check, fill, DATA, GAME_GP};
        let _slots = WscriptFakeSlots::install();
        check(
            ssx3_ee::s09::f_002d19e8__generated,
            wscript_man_try_run_kill_script,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002d_19e8, 1);
                // The script manager, with junk around the two fields the
                // callee touches.
                let manager = DATA + 0x100;
                fill(c, &mut r, manager + 0x280, 8);
                c.bus.write32(GAME_GP + WSCRIPT_MANAGER_GP, manager);
                c.bus.write32(manager + 0x28c, DATA + 0x200);
                // Half the seeds have no kill script for the id.
                c.bus.write32(LOG_RESULTS, (seed % 2) as u32);
                c.g[A0][0] = (r.next_u32() % 64) as u64;
            },
        );
    }

    // -- MeshAnim_RandomizeNodes fake callee ------------------------------
    //
    // The RNG (`Rand_NextMain`) is faked, logging every register at each of
    // the five calls per node so the caller's argument dance is compared.

    fn mesh_anim_fake_rand(c: &mut Cpu) {
        let ret = c.g[RA][0] as u32;
        crate::difftest::fakes::fake(c, MESH_RAND, ret);
    }

    /// Installs the RNG fake and puts the previous body back on drop.
    struct MeshAnimFakeSlots {
        rand: fn(&mut Cpu),
    }

    impl MeshAnimFakeSlots {
        fn install() -> Self {
            unsafe {
                let saved = Self {
                    rand: ssx3_ee::s10::OVERRIDE_003177F0,
                };
                ssx3_ee::s10::OVERRIDE_003177F0 = mesh_anim_fake_rand;
                saved
            }
        }
    }

    impl Drop for MeshAnimFakeSlots {
        fn drop(&mut self) {
            unsafe {
                ssx3_ee::s10::OVERRIDE_003177F0 = self.rand;
            }
        }
    }

    #[test]
    fn mesh_anim_randomize_nodes_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA, GAME_GP};
        let _slots = MeshAnimFakeSlots::install();
        check(
            ssx3_ee::s11::f_00352230__generated,
            mesh_anim_randomize_nodes,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_2230, 5);
                let (this, model, nodes, rate) =
                    (DATA + 0x1000, DATA + 0x2000, DATA + 0x3000, DATA + 0x4000);
                // No nodes at all on some seeds (negative and zero counts).
                let count = match seed % 6 {
                    0 => 0i32,
                    1 => -1,
                    n => 1 + (n as i32 % 3),
                };
                c.bus.write32(this + MESH_MODEL, model);
                c.bus.write32(model + MESH_NODE_LIST, nodes);
                c.bus.write32(nodes + MESH_NODE_COUNT, count as u32);
                c.bus.write32(this + MESH_SPIN_SCALE, r.f32(4.0));
                for k in 0..3u32 {
                    c.bus.write32(this + MESH_VMAX + 4 * k, r.f32(20.0));
                }
                for k in 0..4u32 {
                    c.bus.write32(this + MESH_VBASE + 4 * k, r.f32(2.0));
                }
                // The frame-rate struct: divisor and velocity scale.
                c.bus.write32(GAME_GP + MESH_FRAME_RATE_GP, rate);
                c.bus.write32(
                    rate + 0x10,
                    match seed % 4 {
                        0 => 30u32,
                        1 => 60,
                        2 => 120,
                        _ => 1 + r.next_u32() % 200,
                    },
                );
                c.bus.write32(rate + 0x14, r.f32(2.0));
                c.g[A0][0] = this as u64;
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

    // -- SpeechSystem_StreamWaitCallback fake callees ---------------------
    //
    // The stream-array getters and the bank timer are faked (logging every
    // register at the call); their control values live at a fixed address.

    const STREAM_FAKE_CONTROL: u32 = 0x0080_5000;

    fn stream_fake_log(c: &mut Cpu, target: u32) {
        let ret = c.g[RA][0] as u32;
        crate::difftest::fakes::fake(c, target, ret);
    }

    fn stream_fake_get_state(c: &mut Cpu) {
        stream_fake_log(c, STREAM_ARRAY_GET_STATE);
        c.g[V0][0] = c.bus.read32(STREAM_FAKE_CONTROL) as u64;
    }

    fn stream_fake_is_playing(c: &mut Cpu) {
        stream_fake_log(c, STREAM_ARRAY_IS_PLAYING);
        c.g[V0][0] = c.bus.read32(STREAM_FAKE_CONTROL + 4) as u64;
    }

    fn stream_fake_time_left(c: &mut Cpu) {
        stream_fake_log(c, STREAM_ARRAY_TIME_LEFT);
        c.f[0] = c.bus.read32(STREAM_FAKE_CONTROL + 8);
    }

    fn stream_fake_add_timer(c: &mut Cpu) {
        stream_fake_log(c, STREAM_MONITOR_ADD_TIMER);
    }

    fn stream_fake_instance_slot(c: &mut Cpu) {
        stream_fake_log(c, STREAM_INSTANCE_SLOT);
        c.g[V0][0] = c.bus.read32(STREAM_FAKE_CONTROL + 12) as u64;
    }

    /// Installs the fakes and puts the previous bodies back on drop.
    struct StreamFakeSlots {
        get_state: fn(&mut Cpu),
        is_playing: fn(&mut Cpu),
        time_left: fn(&mut Cpu),
        add_timer: fn(&mut Cpu),
        instance_slot: fn(&mut Cpu),
    }

    impl StreamFakeSlots {
        fn install() -> Self {
            unsafe {
                let saved = Self {
                    get_state: ssx3_ee::s08::OVERRIDE_002AAE08,
                    is_playing: ssx3_ee::s08::OVERRIDE_002AB028,
                    time_left: ssx3_ee::s08::OVERRIDE_002AAE78,
                    add_timer: ssx3_ee::s08::OVERRIDE_002ADCA0,
                    instance_slot: ssx3_ee::s08::OVERRIDE_002B1220,
                };
                ssx3_ee::s08::OVERRIDE_002AAE08 = stream_fake_get_state;
                ssx3_ee::s08::OVERRIDE_002AB028 = stream_fake_is_playing;
                ssx3_ee::s08::OVERRIDE_002AAE78 = stream_fake_time_left;
                ssx3_ee::s08::OVERRIDE_002ADCA0 = stream_fake_add_timer;
                ssx3_ee::s08::OVERRIDE_002B1220 = stream_fake_instance_slot;
                saved
            }
        }
    }

    impl Drop for StreamFakeSlots {
        fn drop(&mut self) {
            unsafe {
                ssx3_ee::s08::OVERRIDE_002AAE08 = self.get_state;
                ssx3_ee::s08::OVERRIDE_002AB028 = self.is_playing;
                ssx3_ee::s08::OVERRIDE_002AAE78 = self.time_left;
                ssx3_ee::s08::OVERRIDE_002ADCA0 = self.add_timer;
                ssx3_ee::s08::OVERRIDE_002B1220 = self.instance_slot;
            }
        }
    }

    #[test]
    fn speech_system_stream_wait_callback_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA, GAME_GP};
        let _slots = StreamFakeSlots::install();
        check(
            ssx3_ee::s08::f_002b1348__generated,
            speech_system_stream_wait_callback,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_1348, 12);
                let (this, array, table) = (DATA + 0x1000, DATA + 0x1100, DATA + 0x1200);
                let (msg, ent) = (DATA + 0x2000, DATA + 0x2100);
                // Control values for the faked callees: the stream state, the
                // playing flag, the remaining time and the instance slot.
                c.bus.write32(
                    STREAM_FAKE_CONTROL,
                    match seed % 3 {
                        0 => 0,
                        1 => 1,
                        _ => 2 + r.next_u32() % 4,
                    },
                );
                c.bus.write32(STREAM_FAKE_CONTROL + 4, (seed % 2) as u32);
                c.bus.write32(STREAM_FAKE_CONTROL + 8, r.f32(120.0));
                c.bus.write32(STREAM_FAKE_CONTROL + 12, 0);
                // self -> array; the array's buffer table is empty, so the
                // current-request lookup returns -1 at once.
                c.bus.write32(this, array);
                c.bus.write32(array + 8, table);
                for i in 0..8u32 {
                    c.bus.write32(table + 4 * i, 0);
                }
                // The message and its request entry.
                c.bus.write32(msg + STREAM_MSG_ENTRY, ent);
                c.bus.write32(
                    msg + STREAM_MSG_REQUEST,
                    if seed % 4 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() & 0x7fff
                    },
                );
                c.bus.write32(msg + STREAM_MSG_TIMER, u32::MAX);
                c.bus.write32(ent + STREAM_ENTRY_SLOT, r.next_u32() % 4);
                c.bus.write32(ent + STREAM_ENTRY_INDEX, r.next_u32() % 8);
                // The SND manager chain `**(gp+0x6c4) + 0x1d8`.
                let (p1, p2) = (DATA + 0x3000, DATA + 0x3100);
                c.bus.write32(GAME_GP + STREAM_SND_GP, p1);
                c.bus.write32(p1, p2);
                c.bus.write32(p2, DATA + 0x3200);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = msg as u64;
            },
        );
    }

    /// The animation chain `AnimController_ReplaceSequence` runs, kept on
    /// its short paths (no global fake slots, so the tests stay independent):
    /// an empty sequencer list, and `AnimController_PlayAnim`'s virtual
    /// returning the "no animation" id 0x207.
    fn replace_sequence_setup(c: &mut Cpu, seed: u32, salt: u64) {
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

    #[test]
    fn anim_controller_replace_sequence_matches() {
        use crate::difftest::check;
        check(
            ssx3_ee::s10::f_00312b18__generated,
            anim_controller_replace_sequence,
            200,
            &|c, seed| replace_sequence_setup(c, seed, 0x0031_2b18),
        );
    }
}
