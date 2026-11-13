//! Staging module oc320 (wave wave213): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc320`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_4668, anim_seq_list_clear, "AnimSeqList_Clear"),
    (
        0x0012_ca30,
        rider_wipeout_sub_enter,
        "RiderWipeoutSub_Enter",
    ),
    (0x0036_1290, intersect_info_dtor_2, "IntersectInfo_dtor_2"),
    (
        0x002a_1028,
        ssx_audio_system_stop_character_speech,
        "SSXAudioSystem_StopCharacterSpeech",
    ),
    (
        0x003e_2580,
        big_file_get_header_size,
        "BigFile_GetHeaderSize",
    ),
    (0x003e_2610, big_file_get_footer, "BigFile_GetFooter"),
    (
        0x0011_8ff8,
        rider_metrix_show_trick_name,
        "RiderMetrix_ShowTrickName",
    ),
    (
        0x0031_2660,
        anim_controller_sample_anim_pose,
        "AnimController_SampleAnimPose",
    ),
];

// ---------------------------------------------------------------------------
// AnimSeqList_Clear (0x314668)

/// `AnimSeqList`: sequence count (`+0`), first sequence (`+4`).
pub(crate) const OC320_SEQLIST_COUNT: u32 = 0x0;

pub(crate) const OC320_SEQLIST_FIRST: u32 = 0x4;

/// `AnimSequence`: next sequence in the list (`+0xc8`).
pub(crate) const OC320_SEQ_NEXT: u32 = 0xc8;

/// `AnimSequence_FreeTriggers` (0x313c08).
pub(crate) const OC320_SEQ_FREE_TRIGGERS: Callee = Callee {
    addr: 0x0031_3c08,
    f: ee::s10::AnimSequence_FreeTriggers,
};

/// `operator delete` (`__builtin_delete`, 0x317e50).
pub(crate) const OC320_DELETE: Callee = Callee {
    addr: 0x0031_7e50,
    f: ee::s10::__builtin_delete,
};

/// `void cAnimSequencer::expireAll(void)` (0x314668): drop every sequence of
/// the list: unlink the head, free its triggers, `delete` it, and decrement
/// the count, until the list is empty (a zeroed list leaves at once).
pub fn anim_seq_list_clear(c: &mut Cpu) {
    const LOOP_HEAD: u32 = 0x0031_4688;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S1)).off(OC320_SEQLIST_FIRST).lw(c);
        let empty = c.g[V0][0] == 0;
        // `lq s0, 0x20(sp)` sits in the branch delay slot: it always runs.
        frame.restore(c, S0, 0x20);
        if !empty {
            loop {
                c.cycles += 4;
                c.g[S0][0] = Addr(r32(c, S1)).off(OC320_SEQLIST_FIRST).lw(c);
                let next = Addr(r32(c, S0)).off(OC320_SEQ_NEXT).lw(c);
                c.g[V0][0] = next;
                let node_null = c.g[S0][0] == 0;
                // `sw` in the delay slot: unlink even for a null node.
                Addr(r32(c, S1)).off(OC320_SEQLIST_FIRST).sw(c, next);
                if !node_null {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    OC320_SEQ_FREE_TRIGGERS.call(c, 0x0031_46a0)?;
                    c.g[A0][0] = c.g[S0][0];
                    OC320_DELETE.call(c, 0x0031_46a8)?;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(r32(c, S1)).off(OC320_SEQLIST_COUNT).lw(c);
                c.g[V1][0] = Addr(r32(c, S1)).off(OC320_SEQLIST_FIRST).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                let more = c.g[V1][0] != 0;
                Addr(r32(c, S1)).off(OC320_SEQLIST_COUNT).sw(c, c.g[V0][0]);
                if !more {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, LOOP_HEAD);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderWipeoutSub_Enter (0x12ca30)

/// `RiderWipeoutSub`: the rider (`+0x80`), phase (`+0`), the two pose-pair
/// blocks `+0x10` and `+0x30`, `+0x54`, and the slide amount `+0x70`.
pub(crate) const OC320_WIPE_RIDER: u32 = 0x80;

pub(crate) const OC320_WIPE_A: u32 = 0x10;

pub(crate) const OC320_WIPE_B: u32 = 0x30;

pub(crate) const OC320_WIPE_F54: u32 = 0x54;

pub(crate) const OC320_WIPE_SLIDE: u32 = 0x70;

/// Rider: the motion state (`+0x77c`), the animation controller (`+0x784`),
/// the sequence-pair table (`+0x780`), its two indices (`+0x89c`, `+0x8a4`),
/// the animation set (`+0x86c`), the track (`+0x790`), the position
/// (`+0x9d0`), and the sound slot index (`+0x870`); `+0x2e8`/`+0x2ec` and
/// `+0x318` are reset.
pub(crate) const OC320_RIDER_MOTION: u32 = 0x77c;

pub(crate) const OC320_RIDER_ANIM: u32 = 0x784;

pub(crate) const OC320_RIDER_PAIRS: u32 = 0x780;

pub(crate) const OC320_RIDER_INDEX_A: u32 = 0x89c;

pub(crate) const OC320_RIDER_INDEX_B: u32 = 0x8a4;

pub(crate) const OC320_RIDER_POS: u32 = 0x9d0;

/// `SSXAudioSystem_GetInstance` (`lw gp+0x410`, 0x28b180).
pub(crate) const OC320_AUDIO_GET_INSTANCE: Callee = Callee {
    addr: 0x0028_b180,
    f: ee::s07::f_0028b180,
};

/// `SSXAudioSystem_StopCharacterSpeech(audio, rider)` (0x29e578): clears
/// `audio->slot[rider->0x870]`.
pub(crate) const OC320_AUDIO_STOP_SPEECH: Callee = Callee {
    addr: 0x0029_e578,
    f: ee::s08::f_0029e578,
};

/// `RiderAnimBase_GetBlendInTime(ctrl, 2)` (0x311ae8): 0 for the 0x1b6
/// "no animation" id, else the entry's blend-in time.
pub(crate) const OC320_ANIM_BLEND_IN: Callee = Callee {
    addr: 0x0031_1ae8,
    f: ee::s10::f_00311ae8,
};

/// `RiderWipeoutState_SetPose(motion, a, b, pair)` (0x136d40).
pub(crate) const OC320_MOTION_SET_POSE: Callee = Callee {
    addr: 0x0013_6d40,
    f: ee::s01::f_00136d40,
};

/// `ReplayCache_RecordKey(cache, key, id)` (0x2708f0).
pub(crate) const OC320_REPLAY_RECORD: Callee = Callee {
    addr: 0x0027_08f0,
    f: ee::s07::f_002708f0,
};

/// `WScriptMan_PushOp(man)` (0x309990): push op 0x70004.
pub(crate) const OC320_WSCRIPT_PUSH: Callee = Callee {
    addr: 0x0030_9990,
    f: ee::s10::f_00309990,
};

/// `void RiderWipeoutSub_Enter(RiderWipeoutSub* this)` (0x12ca30): start a
/// wipeout: clear the rider's wipeout state, stop its speech slot, copy the
/// two pose pairs selected by the rider's current/next pair indices (adding
/// the rider's position to both first vectors), and — when the controller's
/// layer-2 blend-in time is 0x16 — hand the wipeout motion its start pose;
/// then record the replay key and post the wipeout script op.
pub fn rider_wipeout_sub_enter(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::Src;
    guest(c, |c| {
        c.cycles += 52;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(OC320_WIPE_B));
        // Reset the rider's wipeout state.
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(OC320_WIPE_RIDER));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x2ec), 0);
        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(OC320_WIPE_RIDER));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x2e8), 0);
        c.sw(r32(c, S0).wrapping_add(OC320_WIPE_F54), 0);
        c.sw(r32(c, S0).wrapping_add(OC320_WIPE_SLIDE), 0);
        // Audio: stop the rider's character speech slot.
        OC320_AUDIO_GET_INSTANCE.call(c, 0x0012_ca64)?;
        c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(OC320_WIPE_RIDER));
        c.g[A0][0] = c.g[V0][0];
        OC320_AUDIO_STOP_SPEECH.call(c, 0x0012_ca70)?;
        // The pair for the current animation (`+0x89c`), then the one for
        // the next (`+0x8a4`): 0x20 bytes each out of the pair table.
        c.g[A2][0] = c.lw(r32(c, S0).wrapping_add(OC320_WIPE_RIDER));
        c.g[A1][0] = 0x2;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(OC320_RIDER_PAIRS));
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(OC320_RIDER_INDEX_A));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.lq(2, c.g[A0][0] as u32);
        c.sq(2, r32(c, S0).wrapping_add(OC320_WIPE_A));
        c.lq(3, (c.g[A0][0] as u32).wrapping_add(0x10));
        c.sq(3, r32(c, S0).wrapping_add(OC320_WIPE_A + 0x10));
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(OC320_RIDER_PAIRS));
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(OC320_RIDER_INDEX_B));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.lq(2, c.g[A0][0] as u32);
        c.sq(2, r32(c, S0).wrapping_add(OC320_WIPE_B));
        c.lq(3, (c.g[A0][0] as u32).wrapping_add(0x10));
        c.sq(3, r32(c, S0).wrapping_add(OC320_WIPE_B + 0x10));
        // Both pair starts get the rider's position added (VU0).
        c.lqc2(3, r32(c, S0).wrapping_add(OC320_WIPE_A));
        c.lqc2(4, (c.g[A2][0] as u32).wrapping_add(OC320_RIDER_POS));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, r32(c, S0).wrapping_add(OC320_WIPE_A));
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(OC320_WIPE_RIDER));
        c.lqc2(3, r32(c, S0).wrapping_add(OC320_WIPE_B));
        c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(OC320_RIDER_POS));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, r32(c, S0).wrapping_add(OC320_WIPE_B));
        c.sw(r32(c, S0), 0);
        // Layer 2's blend-in time picks the wipeout start pose (0x16).
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(OC320_WIPE_RIDER));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(OC320_RIDER_ANIM));
        OC320_ANIM_BLEND_IN.call(c, 0x0012_caf4)?;
        c.g[V1][0] = 0x16;
        let not_wipeout_anim = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        if !not_wipeout_anim {
            c.cycles += 11;
            c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(OC320_WIPE_RIDER));
            c.g[A2][0] = 0x500000;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0xee0));
            c.g[A3][0] = c.g[S1][0];
            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(OC320_RIDER_MOTION));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1e0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
            OC320_MOTION_SET_POSE.call(c, 0x0012_cb20)?;
            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(OC320_WIPE_RIDER));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x318), 0);
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        }
        c.cycles += 14;
        c.g[A3][0] = c.lw(r32(c, S0).wrapping_add(OC320_WIPE_RIDER));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x790));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x86c));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xfc));
        OC320_REPLAY_RECORD.call(c, 0x0012_cb48)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        OC320_WSCRIPT_PUSH.call(c, 0x0012_cb50)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// IntersectInfo_dtor_2 (0x361290)

/// The `cMeshAnimFrame` vtable-instance pointer written at `+0x84`
/// (`D_00490AF0`).
pub(crate) const OC320_MESH_ANIM_FRAME_VT: u32 = 0x0049_0af0;

/// `void cMeshAnimFrame::~cMeshAnimFrame(this, int flags)` (0x361290): the
/// deleting destructor variant: install the base vtable pointer at `+0x84`,
/// and when `flags & 1` is set, `operator delete(this)`.
pub fn intersect_info_dtor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x490000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(OC320_MESH_ANIM_FRAME_VT);
        c.g[A1][0] = c.g[A1][0] & 0x1;
        let plain = c.g[A1][0] == 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x84), c.g[V0][0]);
        if !plain {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x0036_12b4)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_StopCharacterSpeech (0x2a1028)

/// `SSXAudioSystem`: the stream array (`+0x118`) and the speech system
/// (`+0x5560`, whose `+4` is the instance table).
pub(crate) const OC320_AUDIO_STREAMS: u32 = 0x118;

pub(crate) const OC320_AUDIO_SPEECH: u32 = 0x5560;

/// `SpeechSystem_GetInstanceStreamSlot(speech, index)` (0x2b1220).
pub(crate) const OC320_SPEECH_GET_STREAM_SLOT: Callee = Callee {
    addr: 0x002b_1220,
    f: ee::s08::f_002b1220,
};

/// `StreamArray_GetCurrentRequest(array, index)` (0x2ab150).
pub(crate) const OC320_STREAM_ARRAY_GET_REQUEST: Callee = Callee {
    addr: 0x002a_b150,
    f: ee::s08::f_002ab150,
};

/// `BankMonitor_FindInstance(bank, key, index)` (0x2ad550).
pub(crate) const OC320_BANK_FIND_INSTANCE: Callee = Callee {
    addr: 0x002a_d550,
    f: ee::s08::f_002ad550,
};

/// `SpeechSystem_StopInstance(speech, index)` (0x2b11b0).
pub(crate) const OC320_SPEECH_STOP_INSTANCE: Callee = Callee {
    addr: 0x002b_11b0,
    f: ee::s08::f_002b11b0,
};

/// `int SSXAudioSystem_StopCharacterSpeech(SSXAudioSystem* self, int id)`
/// (0x2a1028): stop the character speech instance whose voice carries `id`.
/// 1 when there was no current request or the instance was stopped, 0 when
/// there is no voice or the voice belongs to another id.
pub fn ssx_audio_system_stop_character_speech(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(OC320_AUDIO_SPEECH));
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S1][0];
        OC320_SPEECH_GET_STREAM_SLOT.call(c, 0x002a_1054)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(OC320_AUDIO_STREAMS));
        c.g[A1][0] = c.g[V0][0];
        OC320_STREAM_ARRAY_GET_REQUEST.call(c, 0x002a_1060)?;
        c.g[A1][0] = c.g[V0][0];
        let no_request = (c.g[A1][0] as i64) < 0;
        c.g[V0][0] = 0x1;
        if !no_request {
            c.cycles += 7;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(OC320_AUDIO_STREAMS));
            c.g[A2][0] = 0x3;
            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
            OC320_BANK_FIND_INSTANCE.call(c, 0x002a_1080)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x80));
                c.cycles += 2;
                let other_id = c.g[V0][0] != c.g[S2][0];
                c.g[V0][0] = 0;
                if !other_id {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 0;
                    OC320_SPEECH_STOP_INSTANCE.call(c, 0x002a_10a4)?;
                    c.g[V0][0] = 0x1;
                }
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
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
// BigFile_GetHeaderSize (0x3e2580)

/// `BigFile_GetType(base)` (0x3e2500).
pub(crate) const OC320_BIG_TYPE: Callee = Callee {
    addr: 0x003e_2500,
    f: ee::s14::BigFile_GetType,
};

/// `int BigFile_GetHeaderSize(BigFile* base)` (0x3e2580): the header size of
/// a BIG file: `0xC0FB` type-1 headers encode it in bytes 2-3 (`+4`); types
/// 2 and 3 ("BIGF"/"BIG0") carry a big-endian size at `+0xc`; anything else
/// is 0.
pub fn big_file_get_header_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        OC320_BIG_TYPE.call(c, 0x003e_2598)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 0x1;
        let not_type1 = c.g[V1][0] != c.g[V0][0];
        c.g[S1][0] = 0;
        if !not_type1 {
            c.cycles += 6;
            c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x2));
            c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x3));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 2;
            let no_header = (c.g[V1][0] as i64) <= 0;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 4i64) as u64;
            if !no_header {
                c.cycles += 2;
                let not_short = c.g[V0][0] == 0;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                if !not_short {
                    c.cycles += 10;
                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
                    c.g[A2][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x3));
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 8);
                    c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                    c.g[S1][0] = c.g[V0][0] | c.g[A2][0];
                }
            }
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BigFile_GetFooter (0x3e2610)

/// `BigFile_GetHeaderSize(base)` (0x3e2580).
pub(crate) const OC320_BIG_HEADER_SIZE: Callee = Callee {
    addr: 0x003e_2580,
    f: ee::s14::BigFile_GetHeaderSize,
};

/// `int BigFile_GetFooter(void* base, int* num, int* tag)` (0x3e2610):
/// recognise the 8-byte footer an EA "big file" build tool appends before
/// the header: a letter and three digits at `header-8`, and the tag bytes at
/// `header-4`. Returns 8 and fills `num`/`tag` (each optional) when the
/// footer is there, 0 otherwise.
pub fn big_file_get_footer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        OC320_BIG_HEADER_SIZE.call(c, 0x003e_2638)?;
        c.g[S3][0] = 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_sub(0x8));
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x41));
        c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
        let lower = c.g[V0][0] != 0;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x8));
        let letter_ok;
        if lower {
            letter_ok = true;
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x61));
            c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
            letter_ok = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S3][0];
        }
        let mut digits_ok = false;
        if letter_ok {
            c.cycles += 5;
            c.g[A1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30));
            c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
            let d1 = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S3][0];
            if d1 {
                c.cycles += 5;
                c.g[V1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
                c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
                let d2 = c.g[V0][0] != 0;
                c.g[V0][0] = c.g[S3][0];
                if d2 {
                    c.cycles += 5;
                    c.g[A2][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x3));
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
                    digits_ok = c.g[V0][0] != 0;
                    c.g[V0][0] = c.g[S3][0];
                }
            }
        }
        if digits_ok {
            c.cycles += 2;
            let no_num = c.g[S1][0] == 0;
            c.g[S3][0] = 0x8;
            if !no_num {
                c.cycles += 16;
                c.g[V0][0] = sx((c.g[A1][0] as u32) << 24);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 24);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
                c.g[A0][0] = 0x64;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
                c.g[A1][0] = 0xa;
                c.mult(0, 2, 2, 4);
                c.mult(0, 3, 3, 5);
                c.g[A0][0] = sx((c.g[A2][0] as u32) << 24);
                c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            }
            c.cycles += 2;
            let no_tag = c.g[S2][0] == 0;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4));
            if !no_tag {
                c.cycles += 11;
                c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_sub(0x4));
                c.g[V1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
                c.g[A2][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x3));
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32) << 8);
                c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                c.sw(c.g[S2][0] as u32, c.g[V0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
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

// ---------------------------------------------------------------------------
// RiderMetrix_ShowTrickName (0x118ff8)

/// `Trick_BuildName(out, id)` (0x116950).
pub(crate) const OC320_TRICK_BUILD_NAME: Callee = Callee {
    addr: 0x0011_6950,
    f: ee::s00::Trick_BuildName,
};

/// `RiderMetrix_ShowHUDText(self, 0, text, 3.0f, repeat)` (0x117ae8).
pub(crate) const OC320_SHOW_HUD_TEXT: Callee = Callee {
    addr: 0x0011_7ae8,
    f: ee::s00::RiderMetrix_ShowHUDText,
};

/// `void RiderMetrix_ShowTrickName(RiderMetrix* self, TrickId* id, int
/// repeat)` (0x118ff8): build the trick's display name into a 0x100-byte
/// stack buffer and show it as a HUD effect at scale 3, then post the trick
/// to the script manager.
pub fn rider_metrix_show_trick_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x140);
        frame.save(c, S0, 0x130);
        frame.save(c, S1, 0x120);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x110);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x100);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = 0xff;
        OC320_TRICK_BUILD_NAME.call(c, 0x0011_9024)?;
        c.g[AT][0] = 0x40400000;
        c.mtc1(1, 12);
        c.g[A3][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = c.g[SP][0];
        OC320_SHOW_HUD_TEXT.call(c, 0x0011_9040)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0030_9918, 0x0011_904c)?;
        frame.restore(c, S0, 0x130);
        frame.restore(c, S1, 0x120);
        frame.restore(c, S2, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimController_SampleAnimPose (0x312660)

/// `AnimSeqList_Init(list)` (0x3142e8).
pub(crate) const OC320_SEQLIST_INIT: Callee = Callee {
    addr: 0x0031_42e8,
    f: ee::s10::AnimSeqList_Init,
};

/// `AnimController_PlayAnimImpl(ctrl, seq, anim, key, 1, 0, 0.0f)` (0x311f00).
pub(crate) const OC320_PLAY_ANIM_IMPL: Callee = Callee {
    addr: 0x0031_1f00,
    f: ee::s10::AnimController_PlayAnimImpl,
};

/// `AnimSeqList_GetAt(list, i)` (0x314760).
pub(crate) const OC320_SEQLIST_GET_AT: Callee = Callee {
    addr: 0x0031_4760,
    f: ee::s10::AnimSeqList_GetAt,
};

/// `AnimSequence_SetTrackTime(seq, track, time)` (0x313cf0).
pub(crate) const OC320_SEQ_SET_TRACK_TIME: Callee = Callee {
    addr: 0x0031_3cf0,
    f: ee::s10::AnimSequence_SetTrackTime,
};

/// `AnimSeqList_CollectTracks(list, out, max)` (0x314418).
pub(crate) const OC320_SEQ_COLLECT_TRACKS: Callee = Callee {
    addr: 0x0031_4418,
    f: ee::s10::AnimSeqList_CollectTracks,
};

/// `AnimSeqList_Clear(list)` (0x314668).
pub(crate) const OC320_SEQ_CLEAR: Callee = Callee {
    addr: 0x0031_4668,
    f: ee::s10::AnimSeqList_Clear,
};

/// `AnimBlendFrame AnimController_SampleAnimPose(void* out,
/// AnimController* ctrl, int anim, int key, float time)` (0x312660): play the
/// animation on a scratch sequencer, set its track time, sample the tracks
/// and copy the 0x50-byte blend frame to `out`; the sequencer is emptied
/// again.
pub fn anim_controller_sample_anim_pose(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x1490);
        c.g[V0][0] = 0x3f;
        frame.save(c, S0, 0x1470);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        frame.save(c, S1, 0x1460);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x1450);
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1400));
        frame.save(c, S3, 0x1440);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x1430);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x1420);
        c.g[S4][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x1480);
        c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        frame.save_ra(c, 0x1410);
        c.mov_s(20, 12);
        c.g[A0][0] = c.g[S1][0];
        OC320_SEQLIST_INIT.call(c, 0x0031_26b0)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2860));
        let direct = c.g[S0][0] != c.g[V0][0];
        c.g[A1][0] = c.g[S3][0];
        if !direct {
            // The AI anim id: a virtual call on the controller's interface.
            c.cycles += 9;
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x58));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0031_26d0)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xd8c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1030));
        }
        c.cycles += 23;
        c.mtc1(0, 12);
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = 0x1;
        c.g[T1][0] = 0;
        c.g[A2][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        OC320_PLAY_ANIM_IMPL.call(c, 0x0031_2700)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        OC320_SEQLIST_GET_AT.call(c, 0x0031_270c)?;
        c.g[A0][0] = c.g[V0][0];
        c.mov_s(12, 20);
        c.g[A1][0] = 0;
        OC320_SEQ_SET_TRACK_TIME.call(c, 0x0031_271c)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x40;
        OC320_SEQ_COLLECT_TRACKS.call(c, 0x0031_272c)?;
        c.g[A0][0] = c.g[S1][0];
        OC320_SEQ_CLEAR.call(c, 0x0031_2734)?;
        // Two 0x20-byte samples and a final 0x10-byte block to `out`.
        c.g[A0][0] = c.g[S4][0];
        c.g[V1][0] = c.g[SP][0];
        loop {
            c.cycles += 7;
            c.lq(2, c.g[V1][0] as u32);
            c.lq(5, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.sq(2, c.g[A0][0] as u32);
            c.sq(5, (c.g[A0][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[S5][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
            if !more {
                c.cycles += 13;
                c.g[V0][0] = c.g[S4][0];
                c.lq(8, c.g[V1][0] as u32);
                c.sq(8, c.g[A0][0] as u32);
                frame.restore(c, S0, 0x1470);
                frame.restore(c, S1, 0x1460);
                frame.restore(c, S2, 0x1450);
                frame.restore(c, S3, 0x1440);
                frame.restore(c, S4, 0x1430);
                frame.restore(c, S5, 0x1420);
                frame.restore_ra(c, 0x1410);
                frame.restore_fpr(c, 20, 0x1480);
                frame.close(c);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_273c);
            }
        }
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::machine;
    use crate::difftest::fill;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL;

    /// A list of `count` sequences (linked through `+0xc8`), each with 0..2
    /// triggers (linked through `+0xc`) that the real
    /// `AnimSequence_FreeTriggers` pops, and the real `__builtin_delete`
    /// (allocator mutex held, fast path).
    pub(crate) fn oc320_clear_setup(c: &mut Cpu, seed: u32, count: u32) {
        use crate::game::ai::aianimeventmap::tests::alloc_setup;
        let mut r = machine(c, seed, 0x31_4668, 0);
        alloc_setup(c, &mut r);
        let list = DATA;
        let mut head = 0;
        for k in (0..count).rev() {
            let seq = DATA + 0x1_0000 + 0x400 * k;
            fill(c, &mut r, seq, 0x20);
            let mut trig = 0;
            for t in (0..r.next_u32() % 3).rev() {
                let node = DATA + 0x8_0000 + 0x4000 * k + 0x40 * t;
                c.bus.write32(node + 0x0, r.next_u32());
                c.bus.write32(node + 0xc, trig);
                trig = node;
            }
            c.bus.write32(seq + 0xac, trig);
            c.bus.write32(seq + OC320_SEQ_NEXT, head);
            head = seq;
        }
        c.bus.write32(list + OC320_SEQLIST_COUNT, count);
        c.bus.write32(list + OC320_SEQLIST_FIRST, head);
        c.g[A0][0] = sx(list);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0031_4668;
    }

    driven!(oc320_gen_clear, ee::s10::f_00314668__generated);

    driven!(oc320_hand_clear, anim_seq_list_clear);

    /// Empty list and lists of 1..4 sequences; then the same with events at
    /// the loop's back edge (0x314688).
    #[test]
    fn anim_seq_list_clear_matches() {
        check(
            ee::s10::f_00314668__generated,
            anim_seq_list_clear,
            200,
            &|c, seed| oc320_clear_setup(c, seed, seed % 5),
        );
        check_events(oc320_gen_clear, oc320_hand_clear, 60, &|c, seed| {
            oc320_clear_setup(c, seed, 2 + seed / 3 % 3);
            let mut r = Rng(0x3146_68 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, 0x0031_4668);
            c.next_event = c.cycles + 4 + (r.next_u32() % 24) as u64;
            mode
        });
    }

    // -- 0x0012ca30 RiderWipeoutSub_Enter ------------------------------------

    const OC320_ENTER_THIS: u32 = DATA + 0x2_0000;
    const OC320_ENTER_RIDER: u32 = DATA + 0x3_0000;
    const OC320_ENTER_ANIMBASE: u32 = DATA + 0x4_0000;
    const OC320_ENTER_PAIRS: u32 = DATA + 0x4_8000;
    const OC320_ENTER_MOTION: u32 = DATA + 0x5_0000;
    const OC320_ENTER_SUB: u32 = DATA + 0x5_8000;
    const OC320_ENTER_AUDIO: u32 = DATA + 0x6_0000;
    const OC320_ENTER_WSM: u32 = DATA + 0x7_0000;
    const OC320_ENTER_PTR1: u32 = DATA + 0x7_8000;
    const OC320_ENTER_PTR2: u32 = DATA + 0x7_9000;
    const OC320_ENTER_REPLAY: u32 = DATA + 0x7_a000;

    /// `AnimTable_GetEntry`: `0x446990 + anim`, whose first word is the
    /// animation's blend-in time.
    const OC320_ANIM_TABLE: u32 = 0x0044_6990;

    /// A live wipeout sub-state: an audio system, a rider with a pair table
    /// (two indices), a motion state (pose flag on and off), a replay cache
    /// whose first condition holds (the record call returns at once), a
    /// script manager, and a blend-in time of 0x16 or not.
    pub(crate) fn oc320_enter_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0012_ca30, 0);
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        w(c, OC320_ENTER_THIS + OC320_WIPE_RIDER, OC320_ENTER_RIDER);
        fill(c, &mut r, OC320_ENTER_RIDER, 0x300);
        w(c, OC320_ENTER_RIDER + 0x870, r.next_u32() % 4);
        w(c, OC320_ENTER_RIDER + OC320_RIDER_PAIRS, OC320_ENTER_PAIRS);
        w(c, OC320_ENTER_RIDER + OC320_RIDER_INDEX_A, r.next_u32() % 4);
        w(c, OC320_ENTER_RIDER + OC320_RIDER_INDEX_B, r.next_u32() % 4);
        w(c, OC320_ENTER_RIDER + OC320_RIDER_POS + 0x0, r.next_u32());
        w(c, OC320_ENTER_RIDER + OC320_RIDER_POS + 0x4, r.next_u32());
        w(c, OC320_ENTER_PAIRS + 0x2c, OC320_ENTER_PAIRS + 0x100);
        fill(c, &mut r, OC320_ENTER_PAIRS + 0x100, 0x80);
        // The animation controller's layer 2: 0x1b6 (no blend-in) or an id
        // whose table word is 0x16 (the wipeout start pose) or not.
        let anim = if seed % 5 == 0 {
            0x1b6
        } else {
            (r.next_u32() % 0x40) * 4
        };
        w(
            c,
            OC320_ENTER_RIDER + OC320_RIDER_ANIM,
            OC320_ENTER_ANIMBASE,
        );
        w(c, OC320_ENTER_ANIMBASE + 8, anim);
        if anim != 0x1b6 {
            let blend = if seed % 3 == 0 {
                0x16
            } else {
                r.next_u32() % 0x40
            };
            w(c, OC320_ANIM_TABLE + anim, blend);
        }
        // The motion state: pose flag (both ways) and its sub-object.
        w(
            c,
            OC320_ENTER_RIDER + OC320_RIDER_MOTION,
            OC320_ENTER_MOTION,
        );
        w(
            c,
            OC320_ENTER_MOTION + 0x30,
            if seed % 3 == 0 { 0 } else { 1 },
        );
        w(c, OC320_ENTER_MOTION + 0x70, OC320_ENTER_SUB);
        fill(c, &mut r, OC320_ENTER_SUB, 0x100);
        // Globals: audio system, replay cache, script manager.
        w(c, GAME_GP + 0x410, OC320_ENTER_AUDIO);
        w(c, GAME_GP - 0x848, OC320_ENTER_PTR1);
        w(c, OC320_ENTER_PTR1 + 0x84, OC320_ENTER_PTR2);
        w(c, OC320_ENTER_PTR2 + 0x28, OC320_ENTER_REPLAY);
        w(c, OC320_ENTER_REPLAY + 0x61c, 1);
        w(c, GAME_GP + 0xce8, OC320_ENTER_WSM);
        c.g[A0][0] = sx(OC320_ENTER_THIS);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0012_ca30;
    }

    /// The real audio, animation-controller, wipeout-motion, replay-cache
    /// (early return) and script-manager callees; the 0x16 branch both ways.
    #[test]
    fn rider_wipeout_sub_enter_matches() {
        check(
            ee::s00::f_0012ca30__generated,
            rider_wipeout_sub_enter,
            200,
            &|c, seed| oc320_enter_setup(c, seed),
        );
    }

    // -- 0x00361290 IntersectInfo_dtor_2 -------------------------------------

    /// A `cMeshAnimFrame` at DATA; the deleting destructor variant (`flags
    /// & 1`) calls `operator delete`, the plain one does not. Other flag bits
    /// are masked away (and stay masked in `a1`).
    #[test]
    fn intersect_info_dtor_2_matches() {
        use crate::game::ai::aianimeventmap::tests::alloc_setup;
        check(
            ee::s12::f_00361290__generated,
            intersect_info_dtor_2,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x36_1290, 0);
                alloc_setup(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = match seed % 4 {
                    0 => 0u64,
                    1 => 1,
                    2 => 0xfffe | 1,
                    _ => (r.next_u32() & !1) as u64,
                };
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0036_1290;
            },
        );
    }

    // -- 0x002a1028 SSXAudioSystem_StopCharacterSpeech ------------------------

    const OC320_STOP_SELF: u32 = DATA + 0x1_0000;
    const OC320_STOP_SLOTTBL: u32 = DATA + 0x1_1000;
    const OC320_STOP_SLOTENTRY: u32 = DATA + 0x1_2000;
    const OC320_STOP_ARRAY: u32 = DATA + 0x1_3000;
    const OC320_STOP_ELEMS: u32 = DATA + 0x1_4000;
    const OC320_STOP_BUFFER: u32 = DATA + 0x1_5000;
    const OC320_STOP_BANK: u32 = DATA + 0x2_0000;
    const OC320_STOP_VOICES: u32 = DATA + 0x2_1000;
    const OC320_STOP_STREAM: u32 = DATA + 0x3_0000;
    const OC320_STOP_REQ: u32 = DATA + 0x3_1000;
    const OC320_STOP_BUFOBJ: u32 = DATA + 0x3_2000;
    /// `SNDSTRM_GetStream`: the stream table (`0x50a0a0 + id*4`) and its
    /// count byte.
    const OC320_STREAM_TABLE: u32 = 0x0050_a0a0;
    const OC320_STREAM_COUNT: u32 = 0x0050_a92e;
    /// `SNDSTRM_GetBufferedBytes`: the request table.
    const OC320_STREAM_REQ_TABLE: u32 = 0x0051_5abc;

    /// Modes: no slot entry (-1), a null array element (-1), no matching
    /// voice, and a matching voice (id equal or not, so the stop instance
    /// call runs or not). The stream lookup runs the real library code with
    /// one controlled entry in the global stream table.
    pub(crate) fn oc320_stop_speech_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x002a_1028, 0);
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        let w8 = |c: &mut Cpu, a: u32, v: u8| c.bus.write8(a, v);
        let mode = seed % 4;
        let id = 0x40 + r.next_u32() % 4;
        w(c, OC320_STOP_SELF + OC320_AUDIO_STREAMS, OC320_STOP_ARRAY);
        w(
            c,
            OC320_STOP_SELF + OC320_AUDIO_SPEECH + 4,
            OC320_STOP_SLOTTBL,
        );
        if mode == 0 {
            w(c, OC320_STOP_SLOTTBL, 0);
        } else {
            w(c, OC320_STOP_SLOTTBL, OC320_STOP_SLOTENTRY);
        }
        let slot = r.next_u32() % 4;
        w(c, OC320_STOP_SLOTENTRY + 4, slot);
        w(c, OC320_STOP_ARRAY + 8, OC320_STOP_ELEMS);
        // The stream lookup: array element -> buffer -> stream id.
        let stream_id = r.next_u32() % 4;
        w(
            c,
            OC320_STOP_ELEMS + 4 * slot,
            if mode == 1 { 0 } else { OC320_STOP_BUFFER },
        );
        w(c, OC320_STOP_BUFFER + 8, stream_id);
        w(c, OC320_STREAM_TABLE + 4 * stream_id, OC320_STOP_STREAM);
        w8(c, OC320_STREAM_COUNT, 8);
        w(c, OC320_STOP_STREAM + 0x124, 7);
        w(c, OC320_STOP_STREAM + 0x11c, OC320_STOP_REQ);
        let request = r.next_u32() % 4;
        w(c, OC320_STOP_REQ + 0xc, request);
        c.bus.write16(OC320_STOP_STREAM + 0x14, 1);
        let bb_id = r.next_u32() % 4;
        w(c, OC320_STOP_STREAM + 8, bb_id);
        w(c, OC320_STREAM_REQ_TABLE + 4 * bb_id, OC320_STOP_BUFOBJ);
        w(c, OC320_STOP_BUFOBJ + 0x48, 5);
        // The bank: `lw(lw(self+0x118)) + 0x1d8`, voice slots at `+0x8ec`.
        w(c, OC320_STOP_ARRAY, OC320_STOP_BANK);
        w(c, OC320_STOP_BANK + 0x8ec + 0x1d8, OC320_STOP_VOICES);
        if mode >= 2 {
            let k = r.next_u32() % 4;
            let voice = OC320_STOP_VOICES + k * 0xc0;
            w(c, voice, 3);
            w(c, voice + 4, request);
            w(c, voice + 0x9c, 0);
            w(
                c,
                voice + 0x80,
                if mode == 3 && seed % 8 < 4 {
                    id
                } else {
                    id ^ 1
                },
            );
        }
        c.g[A0][0] = sx(OC320_STOP_SELF);
        c.g[A1][0] = sx(id);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x002a_1028;
    }

    /// The real speech-slot, stream-array, bank-monitor and stop-instance
    /// callees; both return values and every early exit.
    #[test]
    fn ssx_audio_system_stop_character_speech_matches() {
        check(
            ee::s08::f_002a1028__generated,
            ssx_audio_system_stop_character_speech,
            240,
            &|c, seed| oc320_stop_speech_setup(c, seed),
        );
    }

    // -- 0x00118ff8 RiderMetrix_ShowTrickName --------------------------------

    /// A trick id (random bit fields) and name-part tables whose slots point
    /// at empty or short strings, so the real `Trick_BuildName` copies both
    /// shapes; the real `RiderMetrix_ShowHUDText` stops at its null slot
    /// (self+0x1b0), and the real script-manager post runs.
    #[test]
    fn rider_metrix_show_trick_name_matches() {
        check(
            ee::s00::f_00118ff8__generated,
            rider_metrix_show_trick_name,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0011_8ff8, 0);
                let this = DATA + 0x1000;
                let id = DATA + 0x2000;
                let strings = DATA + 0x3000;
                c.bus.write32(this + 0x1b0, 0);
                c.bus.write32(id, r.next_u32());
                c.bus.write32(id + 4, r.next_u32());
                for k in 0..4u32 {
                    let s = strings + 0x10 * k;
                    let len = if k == 0 { 0 } else { k + r.next_u32() % 6 };
                    for i in 0..len {
                        c.bus.write8(s + i, b'a' + (r.next_u32() % 26) as u8);
                    }
                    c.bus.write8(s + len, 0);
                }
                for t in [
                    0x0043_d118u32,
                    0x0043_d130,
                    0x0043_cf40,
                    0x0043_cf60,
                    0x0043_cf70,
                    0x0043_cfb0,
                    0x0043_cfc8,
                    0x0043_cfe0,
                    0x0043_d0c0,
                    0x0043_d160,
                    0x0043_d2b8,
                    0x0043_d2d0,
                    0x0043_d2e0,
                    0x0043_d100,
                    0x0043_d320,
                    GAME_GP - 0x2100,
                ] {
                    for slot in 0..0x80u32 {
                        c.bus
                            .write32(t + 4 * slot, strings + 0x10 * (r.next_u32() % 4));
                    }
                }
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x4000);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(id);
                c.g[A2][0] = sx(r.next_u32() % 4);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0011_8ff8;
            },
        );
    }

    // -- 0x003e2580 BigFile_GetHeaderSize ------------------------------------

    /// A "big file" header of every type (`0xC0FB` with bytes 2-3, "BIGF"
    /// and "BIG0" with a big-endian size at +0xc, and an unknown one) plus
    /// the out-of-range type values the type code can return.
    #[test]
    fn big_file_get_header_size_matches() {
        check(
            ee::s14::f_003e2580__generated,
            big_file_get_header_size,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_2580, 0);
                let base = DATA + 0x1000;
                let size = (r.next_u32() & 0xff) | ((r.next_u32() & 0xff) << 8);
                match seed % 5 {
                    0 => {
                        // `0xC0FB` type 1: the size is only bytes 2-3 + 4.
                        c.bus.write8(base, 0xc0);
                        c.bus.write8(base + 1, 0xfb);
                        c.bus.write16(base + 2, size as u16);
                    }
                    1 => {
                        c.bus.write32(base, 0x4249_4746);
                        c.bus.write32(base + 0xc, r.next_u32());
                    }
                    2 => {
                        c.bus.write32(base, 0x4249_4700 | (r.next_u32() & 0xff));
                        c.bus.write32(base + 0xc, r.next_u32());
                    }
                    _ => {
                        c.bus.write32(base, r.next_u32());
                        c.bus.write32(base + 0xc, r.next_u32());
                    }
                }
                c.g[A0][0] = sx(base);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x003e_2580;
            },
        );
    }

    // -- 0x003e2610 BigFile_GetFooter ----------------------------------------

    const OC320_FOOTER_NUM: u32 = DATA + 0x800;
    const OC320_FOOTER_TAG: u32 = DATA + 0x804;

    /// Every header type (the footer sits right before the header), a footer
    /// letter in all three ranges and digits that are valid or not; `num`
    /// and `tag` present or null.
    #[test]
    fn big_file_get_footer_matches() {
        check(
            ee::s14::f_003e2610__generated,
            big_file_get_footer,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_2610, 0);
                let base = DATA + 0x2000;
                let hdr = match seed % 4 {
                    0 => {
                        c.bus.write32(base, r.next_u32() & 0x00ff_ffff);
                        0
                    }
                    1 => {
                        c.bus.write8(base, 0xc0);
                        c.bus.write8(base + 1, 0xfb);
                        c.bus.write16(base + 2, 0x1c);
                        0x20
                    }
                    2 => {
                        c.bus.write32(base, 0x4249_4746);
                        c.bus.write32(base + 0xc, 0x20);
                        0x20
                    }
                    _ => {
                        c.bus.write32(base, 0x4249_4700 | (r.next_u32() & 0xff));
                        c.bus.write32(base + 0xc, 0x20);
                        0x20
                    }
                };
                let q = base + hdr - 8;
                let letter = match seed % 6 {
                    0 => b'Z' - (r.next_u32() % 26) as u8,
                    1 => b'z' - (r.next_u32() % 26) as u8,
                    2 => r.next_u32() as u8,
                    3 => b'0' + (r.next_u32() % 10) as u8,
                    4 => b'A',
                    _ => 0x40,
                };
                c.bus.write8(q, letter);
                for d in 0..3u32 {
                    c.bus.write8(
                        q + 1 + d,
                        if seed % 5 == 0 && d == 2 {
                            b'/' - (r.next_u32() % 10) as u8
                        } else {
                            b'0' + (r.next_u32() % 10) as u8
                        },
                    );
                }
                c.bus.write32(q + 4, r.next_u32());
                c.g[A0][0] = sx(base);
                c.g[A1][0] = if seed % 7 == 0 {
                    0
                } else {
                    sx(OC320_FOOTER_NUM)
                };
                c.g[A2][0] = if seed % 7 == 3 {
                    0
                } else {
                    sx(OC320_FOOTER_TAG)
                };
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x003e_2610;
            },
        );
    }

    // -- 0x00312660 AnimController_SampleAnimPose ----------------------------

    const OC320_SAMPLE_BUF: u32 = DATA + 0x1_0000;
    const OC320_SAMPLE_SELF: u32 = DATA + 0x2_0000;
    const OC320_SAMPLE_CHANTBL: u32 = DATA + 0x3_0000;
    const OC320_SAMPLE_CHANSET: u32 = DATA + 0x3_1000;
    const OC320_SAMPLE_CHANBASE: u32 = DATA + 0x3_2000;

    /// `AnimTable_GetEntry`: `0x446990 + anim`.
    const OC320_SAMPLE_ANIM_TABLE: u32 = 0x0044_6990;

    /// The game image (anim name tables and the size-class pools), an
    /// animation-table entry with one plain sequence and no triggers or
    /// weight fade, a channel set with no animation triggers, and controller
    /// data for the callee to copy. The AI-anim-id path (a virtual call) is
    /// not taken: the key is not the sentinel.
    #[test]
    fn anim_controller_sample_anim_pose_matches() {
        use crate::difftest::elf_image;
        use crate::game::bx::bxstring::tests::pool;
        check(
            ee::s10::f_00312660__generated,
            anim_controller_sample_anim_pose,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_2660, 0);
                elf_image(c);
                pool(c);
                let anim = 0x100 + 4 * (r.next_u32() % 8);
                let e = OC320_SAMPLE_ANIM_TABLE + anim;
                c.bus.write32(e + 0x4, 1);
                c.bus.write32(e + 0x8, 0);
                c.bus.write32(e + 0x14, 0);
                c.bus.write32(e + 0x18, r.next_u32());
                c.bus.write32(GAME_GP + 0xd08, OC320_SAMPLE_CHANTBL);
                c.bus.write32(OC320_SAMPLE_CHANTBL, OC320_SAMPLE_CHANSET);
                c.bus
                    .write32(OC320_SAMPLE_CHANSET + 0x8, OC320_SAMPLE_CHANBASE);
                c.bus.write16(OC320_SAMPLE_CHANBASE + 0x12, 0);
                c.bus.write32(GAME_GP + 0x2860, 0xdead_beef);
                fill(c, &mut r, OC320_SAMPLE_SELF, 0x40);
                c.f[12] = if seed % 3 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(10.0)
                };
                c.g[A0][0] = sx(OC320_SAMPLE_BUF);
                c.g[A1][0] = sx(OC320_SAMPLE_SELF);
                c.g[A2][0] = sx(anim);
                c.g[A3][0] = 0;
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0031_2660;
            },
        );
    }
}
