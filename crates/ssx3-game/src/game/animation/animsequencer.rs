//! `src/animation/animsequencer.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::world::wscriptcache::DELETE_2 as DELETE;
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
        0x0031_3d28,
        anim_sequence_set_track_weight,
        "AnimSequence_SetTrackWeight",
    ),
    (
        0x0031_3d40,
        anim_sequence_set_track_loop,
        "AnimSequence_SetTrackLoop",
    ),
    (0x0031_42e8, anim_seq_list_init, "AnimSeqList_Init"),
    (0x0031_4368, anim_seq_list_append, "AnimSeqList_Append"),
    (
        0x0031_4418,
        anim_seq_list_collect_tracks,
        "AnimSeqList_CollectTracks",
    ),
    (0x0031_4978, mdf_file_ctor, "MdfFile_ctor"),
    (0x0031_4988, mdf_file_dtor, "MdfFile_dtor"),
    (0x0031_49d0, mdf_file_free, "MdfFile_Free"),
    (0x0031_4aa0, mdf_file_parse_header, "MdfFile_ParseHeader"),
    (
        0x0031_4c00,
        mdf_file_create_model_by_name,
        "MdfFile_CreateModelByName",
    ),
    (0x0031_3dd0, anim_sequence_save, "AnimSequence_Save"),
    (
        0x0031_3ba8,
        anim_sequence_remove_event_trigger,
        "AnimSequence_RemoveEventTrigger",
    ),
    (0x0031_47f0, anim_seq_list_save, "AnimSeqList_Save"),
    (
        0x0031_3c08,
        anim_sequence_free_triggers,
        "AnimSequence_FreeTriggers",
    ),
    (0x0031_42f8, anim_seq_list_push_new, "AnimSeqList_PushNew"),
    (
        0x0031_4588,
        anim_seq_list_push_looping_anim,
        "AnimSeqList_PushLoopingAnim",
    ),
    (
        0x0031_3a70,
        anim_sequence_add_trigger,
        "AnimSequence_AddTrigger",
    ),
    (0x0031_4518, anim_seq_list_push_anim, "AnimSeqList_PushAnim"),
    (
        0x0031_3ad8,
        anim_sequence_add_event_trigger,
        "AnimSequence_AddEventTrigger",
    ),
    (
        0x0031_46d0,
        anim_seq_list_fade_out_all,
        "AnimSeqList_FadeOutAll",
    ),
    (
        0x0031_4718,
        anim_seq_list_remove_event_trigger,
        "AnimSeqList_RemoveEventTrigger",
    ),
    (0x0031_4668, anim_seq_list_clear, "AnimSeqList_Clear"),
];

// ---------------------------------------------------------------- AnimSequence

/// Direct-call targets of the animation functions (shard s10).
pub(crate) mod anim_callee {

    pub(crate) use crate::game::animation::animmodel::anim_callee::GET_TRACK_BLEND_WEIGHT;
}

/// `AnimSequence` layout used here (tracks of 0x1c bytes from +0).
pub(crate) mod seq {

    pub const TRACK_SIZE: u32 = 0x1c;

    /// Track fields: anim id, time, rate, length, loop mode (+0x1c, the
    /// word after the track's active flag).
    pub const T_ANIM: u32 = 0x4;

    pub const T_TIME: u32 = 0x8;

    pub const T_RATE: u32 = 0xc;

    pub const T_LENGTH: u32 = 0x10;

    pub const T_ACTIVE: u32 = 0x18;

    pub const T_MODE: u32 = 0x1c;

    /// Sequence fields.
    pub const BLEND_A: u32 = 0x60;

    pub const BLEND_B: u32 = 0x70;

    pub const LAYER: u32 = 0x80;

    pub const FLAGS: u32 = 0x84;

    pub const USER: u32 = 0x88;

    pub const RATE: u32 = 0x90;

    pub const FIRED_MASK: u32 = 0xb8;

    pub const END_FLAG: u32 = 0xc0;

    pub const DIRTY: u32 = 0xc4;

    pub const NEXT: u32 = 0xc8;
}

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;

    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;

    pub(crate) use crate::game::world::wscriptcache::callee::EA_MEM_FREE_2 as EA_MEM_FREE;

    pub(crate) use crate::game::world::wscriptcache::callee::MDF_FILE_FREE;
}

// ---------------------------------------------------------------- anim sequence

/// Size of one animation track (`AnimTrack`).
pub(crate) const TRACK_SIZE: u32 = 0x1c;

/// `AnimTrack+0x14`: the track's blend weight.
pub(crate) const TRACK_WEIGHT: u32 = 0x14;

/// `AnimTrack+0x1c`: the track's loop flag.
pub(crate) const TRACK_LOOP: u32 = 0x1c;

/// `void AnimSequence_SetTrackWeight(AnimSequence *seq, int track, float w)`
/// (0x313d28): set track `track`'s blend weight (`seq + track*0x1c + 0x14`)
/// to `w`. The index multiply also sets `LO`/`HI`.
pub fn anim_sequence_set_track_weight(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = TRACK_SIZE as u64;
    let offset = abi::mult(c, abi::arg(c, 1), TRACK_SIZE);
    c.g[V1][0] = sx(offset);
    let seq = abi::arg(c, 0);
    let track = Addr(seq.wrapping_add(offset));
    c.g[A0][0] = sx(track.0);
    track.off(TRACK_WEIGHT).write_fpu(c, abi::farg(c, 0));
    abi::ret(c);
}

/// `void AnimSequence_SetTrackLoop(AnimSequence *seq, int track, uint loop)`
/// (0x313d40): store the loop flag at `seq + track*0x1c + 0x1c`. The index
/// multiply also sets `LO`/`HI`.
pub fn anim_sequence_set_track_loop(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = TRACK_SIZE as u64;
    let offset = abi::mult(c, abi::arg(c, 1), TRACK_SIZE);
    c.g[V1][0] = sx(offset);
    let seq = abi::arg(c, 0);
    let track = Addr(seq.wrapping_add(offset));
    c.g[A0][0] = sx(track.0);
    track.off(TRACK_LOOP).sw(c, c.g[A2][0]);
    abi::ret(c);
}

/// `void AnimSeqList_Init(AnimSeqList *this)` (0x3142e8): clear the head and
/// count.
pub fn anim_seq_list_init(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    Addr(this).off(0x4).sw(c, 0);
    Addr(this).sw(c, 0);
    abi::ret(c);
}

/// `MdfFile *MdfFile_ctor(MdfFile *this)` (0x314978): clear the header word.
pub fn mdf_file_ctor(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr(abi::arg(c, 0)).sw(c, 0);
    abi::ret(c);
}

/// `void MdfFile_ParseHeader(MdfFile *this, const u8 *data)` (0x314aa0):
/// store the data pointer and resolve the geometry/vertex sections.
pub fn mdf_file_parse_header(c: &mut Cpu) {
    c.cycles += 8;
    let this = abi::arg(c, 0);
    let data = c.g[A1][0] as u32;
    Addr(this).off(0x4).sw(c, c.g[A1][0]);
    c.g[V0][0] = Addr(data).off(0x6).lh(c);
    c.g[V0][0] = sx(data.wrapping_add(c.g[V0][0] as u32));
    Addr(this).off(0x8).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(data).off(0x8).lw(c);
    c.g[A1][0] = sx(data.wrapping_add(c.g[V1][0] as u32));
    Addr(this).off(0xc).sw(c, c.g[A1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// AnimSeqList.

/// `AnimSeqList` layout.
pub(crate) const LIST_HEAD: u32 = 0x4;

pub(crate) const SEQ_NEXT: u32 = 0xc8;

/// `void AnimSeqList_Append(AnimSeqList *list, AnimSequence *seq)`
/// (0x314368): append `seq` to the singly-linked list at +0x4 (following
/// +0xc8), or set it as the head when the list is empty; clear the new node's
/// link and bump the count at +0. Back-edge event check at 0x314388.
pub fn anim_seq_list_append(c: &mut Cpu) {
    const HEAD: u32 = 0x0031_4388;
    c.cycles += 3;
    let list = Addr(abi::arg(c, 0));
    let seq = abi::arg(c, 1);
    let head = list.off(LIST_HEAD).lw(c);
    c.g[V0][0] = head;
    c.g[V1][0] = head;
    if head == 0 {
        c.cycles += 1;
        list.off(LIST_HEAD).sw(c, seq as u64);
    } else {
        c.cycles += 3;
        let first_link = Addr(head as u32).off(SEQ_NEXT).lw(c);
        c.g[V0][0] = first_link;
        if first_link == 0 {
            Addr(head as u32).off(SEQ_NEXT).sw(c, seq as u64);
        } else {
            c.cycles += 2;
            let mut node = first_link;
            c.g[V1][0] = node;
            loop {
                c.cycles += 7;
                let link = Addr(node as u32).off(SEQ_NEXT).lw(c);
                c.g[V0][0] = link;
                if link != 0 {
                    node = link;
                    c.g[V1][0] = link;
                    if event_due(c) {
                        deliver_event_here(c, HEAD);
                    }
                } else {
                    c.cycles += 2;
                    Addr(node as u32).off(SEQ_NEXT).sw(c, seq as u64);
                    break;
                }
            }
        }
    }
    c.cycles += 5;
    Addr(seq).off(SEQ_NEXT).sw(c, 0);
    let count = list.lw(c) as u32;
    c.g[V0][0] = sx(count.wrapping_add(1));
    list.sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `int AnimSeqList_CollectTracks(AnimSeqList* list, TrackSample* out)`
/// (0x314418): for every sequence in the list (`+4` first, linked by
/// `+0xc8`) and each of its three tracks that is active, append a 0x50-byte
/// sample: anim id, time, the sequence's layer word (+0x84 → +8), the track
/// blend weight (`AnimSequence_GetTrackBlendWeight`, +0xc), the sequence's
/// user doubleword (+0x88 → +0x10), its two blend quadwords (+0x60/+0x70 →
/// +0x20/+0x30) and +0x80 (→ +0x40). Returns the number of samples.
///
/// The loop state lives in s0..s7 as in the original (event checks at
/// 0x314468 per track and 0x314458 per sequence).
pub fn anim_seq_list_collect_tracks(c: &mut Cpu) {
    const TRACK_HEAD: u32 = 0x0031_4468;
    const SEQ_HEAD: u32 = 0x0031_4458;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save(c, S0, 0x80);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x70);
        // s6 = count, s7 = out, s1 = sequence, s5 = out offset of the
        // sequence's first sample, s4 = track, s2/s3 = the track's time /
        // active field, s0 = next sample.
        c.g[S6][0] = 0;
        c.g[S7][0] = c.g[A1][0];
        let first = Addr(abi::arg(c, 0)).off(4).lw(c);
        c.g[S1][0] = first;
        if first != 0 {
            c.cycles += 2;
            c.g[S5][0] = 0;
            c.g[S4][0] = 0;
            loop {
                c.cycles += 4;
                let s = c.g[S1][0] as u32;
                c.g[S2][0] = sx(s.wrapping_add(seq::T_TIME));
                c.g[S3][0] = sx(s.wrapping_add(seq::T_ACTIVE));
                c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S7][0] as u32));
                loop {
                    c.cycles += 3;
                    let s = c.g[S1][0] as u32;
                    let active = Addr(c.g[S3][0] as u32).lw(c);
                    c.g[V0][0] = active;
                    c.g[A0][0] = c.g[S1][0];
                    if active != 0 {
                        c.cycles += 20;
                        let time_at = Addr(c.g[S2][0] as u32);
                        let out = Addr(c.g[S0][0] as u32);
                        let anim = time_at
                            .off(0u32.wrapping_sub(seq::T_TIME - seq::T_ANIM))
                            .lw(c);
                        c.g[V0][0] = anim;
                        c.g[A1][0] = c.g[S4][0];
                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x50));
                        out.sw(c, anim);
                        c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(1));
                        let time = time_at.read_fpu(c);
                        c.f[0] = time.0;
                        out.off(4).write_fpu(c, time);
                        c.lq(V0, s.wrapping_add(seq::BLEND_A));
                        c.sq(V0, out.0.wrapping_add(0x20));
                        c.lq(V1, s.wrapping_add(seq::BLEND_B));
                        c.sq(V1, out.0.wrapping_add(0x30));
                        let layer = Addr(s).off(seq::LAYER).lw(c);
                        c.g[V0][0] = layer;
                        out.off(0x40).sw(c, layer);
                        let flags = Addr(s).off(seq::FLAGS).lw(c);
                        c.g[V1][0] = flags;
                        out.off(8).sw(c, flags);
                        anim_callee::GET_TRACK_BLEND_WEIGHT.call(c, 0x0031_44b4)?;
                        let weight = FpuF32(c.f[0]);
                        out.off(0xc).write_fpu(c, weight);
                        let user = Addr(s).off(seq::USER).ld(c);
                        c.g[V0][0] = user;
                        out.off(0x10).sd(c, user);
                        c.g[S0][0] = sx(out.0.wrapping_add(0x50));
                    }
                    c.cycles += 5;
                    let track = (c.g[S4][0] as u32).wrapping_add(1);
                    c.g[S4][0] = sx(track);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(seq::TRACK_SIZE));
                    let more = (c.g[S4][0] as i64) < 3;
                    c.g[V0][0] = more as u64;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(seq::TRACK_SIZE));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, TRACK_HEAD);
                    }
                }
                c.cycles += 3;
                let next = Addr(c.g[S1][0] as u32).off(seq::NEXT).lw(c);
                c.g[S1][0] = next;
                c.g[S4][0] = 0;
                if next == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, SEQ_HEAD);
                }
            }
        }
        c.cycles += 12;
        c.g[V0][0] = c.g[S6][0];
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Foo_dtor(void* self, int flags)` (0x30 frame): run the destructor
/// `base`, then `__builtin_delete(self)` when bit 0 of `flags` is set.
///
/// The original reloads `s0` (`lq 16, 0x20(sp)`) *before* the delete branch, so
/// the delete call already sees the caller's `s0` — and so does its own frame
/// save. Reproduce that order.
pub(crate) fn dtor(c: &mut Cpu, base: Callee, ret_base: u32, ret_delete: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S1, 0x10);
    frame.save(c, S0, 0x20);
    c.g[S1][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A1][0];
    base.call(c, ret_base)?;
    c.g[S0][0] &= 1;
    let keep = c.g[S0][0] == 0;
    frame.restore(c, S0, 0x20);
    if !keep {
        c.cycles += 3;
        c.g[A0][0] = c.g[S1][0];
        DELETE.call(c, ret_delete)?;
        frame.restore(c, S0, 0x20);
    }
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    c.cycles += 4;
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// Destructor thunks and vtable forwarders.

/// `void MdfFile_dtor(MdfFile* f, int flags)` (0x314988): free the file and,
/// when `flags & 1`, the object.
pub fn mdf_file_dtor(c: &mut Cpu) {
    guest(c, |c| {
        dtor(c, callee::MDF_FILE_FREE, 0x0031_49a4, 0x0031_49b8)
    })
}

/// `void MdfFile_Free(MdfFile* f)` (0x3149d0): release the buffer (`+0`) and
/// zero the four list fields.
pub fn mdf_file_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(r32(c, S0));
        if c.g[A0][0] == 0 {
            c.sw(r32(c, S0).wrapping_add(0xc), 0);
        } else {
            c.cycles += 3;
            callee::EA_MEM_FREE.call(c, 0x0031_49f4)?;
            c.sw(r32(c, S0).wrapping_add(0xc), 0);
        }
        c.cycles += 7;
        c.sw(r32(c, S0), 0);
        c.sw(r32(c, S0).wrapping_add(4), 0);
        c.sw(r32(c, S0).wrapping_add(8), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MdfFile_CreateModelByName` (0x314c00): find the model whose name matches
/// `a1` (case-insensitive) in the file's 0x60-byte records and create it.
pub fn mdf_file_create_model_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x10);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(this, 4)).lw(c);
        c.g[V0][0] = Addr(p(r(c, V1), 4)).lh(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = c.g[A1][0];
        if empty {
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore(c, S0, 0x40);
        } else {
            c.cycles += 2;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 6;
                c.g[A1][0] = Addr(p(this, 8)).lw(c);
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x0031_4c48);
                c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, S2)));
                call_ptr(c, callee::STRCASECMP, 0x0031_4c48)?;
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = Addr(p(this, 4)).lw(c);
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0031_4c5c);
                    c.g[A1][0] = c.g[S0][0];
                    callee::MDF_FILE_CREATE_MODEL.call(c, 0x0031_4c5c)?;
                    frame.restore(c, S0, 0x40);
                    break;
                }
                c.cycles += 5;
                c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                c.g[V0][0] = Addr(p(r(c, V1), 4)).lh(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx(r(c, S2).wrapping_add(0x60));
                if !more {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    frame.restore(c, S0, 0x40);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_4c38);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------- AnimSequence_Save

/// The fixed part of an `AnimSequence` `AnimSequence_Save` writes, in
/// order: (offset, size). The key list head is at +0xac.
pub(crate) const ANIM_SEQUENCE_FIELDS: [(u32, u64); 16] = [
    (0x0, 0x4),
    (0x4, 0x54),
    (0x60, 0x20),
    (0x80, 0x4),
    (0x84, 0x4),
    (0x88, 0x8),
    (0x90, 0x4),
    (0x94, 0x4),
    (0x98, 0x4),
    (0x9c, 0x4),
    (0xa0, 0x4),
    (0xa4, 0x4),
    (0xa8, 0x4),
    (0xb0, 0x10),
    (0xc0, 0x4),
    (0xc4, 0x4),
];

/// `void AnimSequence_Save(AnimSequence* seq, cAsStream* s)` (0x313dd0):
/// writes a sequence to a stream through the stream's virtual
/// `Write(data, size)` (vtable slot +0x8): the sixteen fixed fields, the
/// number of keys in the list at +0xac (linked through +0xc), then each
/// key's 16 bytes. Frame 0x50 (s0-s2; the count at +0).
pub fn anim_sequence_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 122;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0x4;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        let stream = c.g[A1][0];
        c.g[S1][0] = stream;
        frame.save_ra(c, 0x10);
        let seq = c.g[A0][0];
        c.g[S2][0] = seq;
        let seq = seq as u32;
        for (i, &(off, size)) in ANIM_SEQUENCE_FIELDS.iter().enumerate() {
            let vt = c.lw(stream as u32);
            c.g[V1][0] = vt;
            c.g[A1][0] = sx(seq.wrapping_add(off));
            c.g[A2][0] = size;
            let slot = crate::call::VSlot::read(c, vt, 0x8);
            c.g[A0][0] = slot.delta;
            c.g[V0][0] = slot.func;
            c.g[A0][0] = slot.this(stream);
            call_ptr(c, slot.func as u32, 0x0031_3e08 + 0x1c * i as u32)?;
        }

        // The key count.
        let head = c.lw(seq.wrapping_add(0xac));
        c.g[S0][0] = head;
        let s = sp(c);
        c.sw(s, 0);
        if head != 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            loop {
                c.cycles += 7;
                c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                if c.g[S0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_3fc0);
                }
            }
            c.cycles += 1;
            c.sw(sp(c), c.g[V0][0]);
        }
        c.cycles += 10;
        let vt = c.lw(stream as u32);
        c.g[V0][0] = vt;
        c.g[A1][0] = sx(sp(c));
        c.g[A2][0] = 0x4;
        let slot = crate::call::VSlot::read(c, vt, 0x8);
        c.g[A0][0] = slot.delta;
        c.g[V1][0] = slot.func;
        c.g[A0][0] = slot.this(stream);
        call_ptr(c, slot.func as u32, 0x0031_3ffc)?;

        // The keys.
        let head = c.lw(seq.wrapping_add(0xac));
        c.g[S0][0] = head;
        if head != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw(stream as u32);
            loop {
                c.cycles += 9;
                let key = c.g[S0][0];
                c.g[A1][0] = key;
                c.g[A2][0] = 0x10;
                let vt = c.g[V0][0];
                let slot = crate::call::VSlot::read(c, vt, 0x8);
                c.g[A0][0] = slot.delta;
                c.g[V1][0] = slot.func;
                c.g[A0][0] = slot.this(stream);
                call_ptr(c, slot.func as u32, 0x0031_4028)?;
                c.g[S0][0] = c.lw((key as u32).wrapping_add(0xc));
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V0][0] = c.lw(stream as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0031_4010);
                }
            }
        }
        frame.restore(c, S0, 0x40);

        c.cycles += 5;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimSeqList_Save (0x3147f0)

/// AnimSeqList `int`: number of sequences.
pub(crate) const SEQLIST_COUNT: u32 = 0x0;

/// AnimSeqList: first sequence node.
pub(crate) const SEQLIST_FIRST: u32 = 0x4;

/// AnimSequence: next node in the list.
pub(crate) const SEQLIST_NODE_NEXT: u32 = 0xc8;

/// `AnimSequence_Save(seq, stream)`.
pub(crate) const SEQLIST_SEQUENCE_SAVE: Callee = Callee {
    addr: 0x0031_3dd0,
    f: ee::s10::AnimSequence_Save,
};

/// `void AnimSeqList_Save(AnimSeqList* list, cAsStream* s)` (0x3147f0):
/// write the list's count word (`s->Write(list, 4)`, vtable slot +0x8), then
/// save each of its `count` sequences (linked through +0xc8).
pub fn anim_seq_list_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0x4;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let stream = c.g[A1][0];
        c.g[S3][0] = stream;
        c.g[A1][0] = c.g[S2][0];
        // s->Write(list, 4).
        let vt = Addr(stream as u32).lw(c);
        c.g[V1][0] = vt;
        let slot = crate::call::VSlot::read(c, vt, 0x8);
        c.g[A0][0] = slot.delta;
        c.g[V0][0] = slot.func;
        c.g[A0][0] = sx((stream as u32).wrapping_add(slot.delta as u32));
        call_ptr(c, slot.func as u32, 0x0031_4830)?;
        let list = r32(c, S2);
        let count = Addr(list + SEQLIST_COUNT).lw(c);
        c.g[V1][0] = count;
        c.g[S0][0] = Addr(list + SEQLIST_FIRST).lw(c);
        if (count as i64) > 0 {
            c.cycles += 1;
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S3][0];
                SEQLIST_SEQUENCE_SAVE.call(c, 0x0031_484c)?;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                let count = Addr(r32(c, S2) + SEQLIST_COUNT).lw(c);
                let more = (c.g[S1][0] as i64) < (count as i64);
                c.g[V0][0] = more as u64;
                c.g[S0][0] = Addr(r32(c, S0).wrapping_add(SEQLIST_NODE_NEXT)).lw(c);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_4840);
                }
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
    });
}

// ---------------------------------------------------------------------------
// AnimSequence_RemoveEventTrigger (0x313ba8)

/// `operator delete` (`__builtin_delete`, 0x317e50).
pub(crate) const ANIM_TRIGGER_DELETE: Callee = Callee {
    addr: 0x0031_7e50,
    f: ee::s10::f_00317e50,
};

/// `AnimSequence + 0xac`: head of the event-trigger list.
pub(crate) const ANIM_SEQ_TRIGGERS: u32 = 0xac;

/// Trigger node: `{ int key; int active; ...; Trigger* next (+0xc) }`.
pub(crate) const ANIM_TRIGGER_ACTIVE: u32 = 0x4;

pub(crate) const ANIM_TRIGGER_NEXT: u32 = 0xc;

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
// AnimSeqList_PushNew (0x3142f8)

/// `EA_MemAllocNamed(size, name, flags, align)`.
pub(crate) const PUSH_NEW_ALLOC: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::EA_MemAllocNamed,
};

/// `AnimSequence_ctor(seq, type)`.
pub(crate) const PUSH_NEW_SEQ_CTOR: Callee = Callee {
    addr: 0x0031_3508,
    f: ee::s10::AnimSequence_ctor,
};

/// Size of an AnimSequence, its allocation name (0x48a5c0) and flags.
pub(crate) const PUSH_NEW_SEQ_SIZE: u64 = 0xd0;

pub(crate) const PUSH_NEW_ALLOC_FLAGS: u64 = 0x2000_0000;

/// AnimSeqList: `count` at +0, `first` at +4; AnimSequence `next` at +0xc8.
pub(crate) const PUSH_NEW_LIST_COUNT: u32 = 0x0;

pub(crate) const PUSH_NEW_LIST_FIRST: u32 = 0x4;

pub(crate) const PUSH_NEW_SEQ_NEXT: u32 = 0xc8;

/// `AnimSequence* AnimSeqList_PushNew(AnimSeqList* list, int type)`
/// (0x3142f8): allocate and construct a new sequence of `type`, link it at
/// the head of the list and bump the count; returns the sequence.
pub fn anim_seq_list_push_new(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = PUSH_NEW_ALLOC_FLAGS;
        frame.save(c, S1, 0x10);
        c.g[A3][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = PUSH_NEW_SEQ_SIZE;
        c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(0x5a40));
        PUSH_NEW_ALLOC.call(c, 0x0031_4328)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        PUSH_NEW_SEQ_CTOR.call(c, 0x0031_4334)?;
        let list = r32(c, S0);
        c.g[V1][0] = Addr(list.wrapping_add(PUSH_NEW_LIST_FIRST)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        frame.restore(c, S1, 0x10);
        let seq = r32(c, A0);
        let old_first = c.g[V1][0];
        Addr(seq.wrapping_add(PUSH_NEW_SEQ_NEXT)).sw(c, old_first);
        Addr(list.wrapping_add(PUSH_NEW_LIST_FIRST)).sw(c, seq as u64);
        c.g[V1][0] = Addr(list.wrapping_add(PUSH_NEW_LIST_COUNT)).lw(c);
        frame.restore_ra(c, 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
        let count = c.g[V1][0];
        Addr(list.wrapping_add(PUSH_NEW_LIST_COUNT)).sw(c, count);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// AnimSeqList_PushLoopingAnim (0x314588)

pub(crate) const ASL_PUSH_NEW: Callee = Callee {
    addr: 0x0031_42f8,
    f: ee::s10::AnimSeqList_PushNew,
};

pub(crate) const ASL_SET_TRACK_ANIM: Callee = Callee {
    addr: 0x0031_3c50,
    f: ee::s10::AnimSequence_SetTrackAnim,
};

pub(crate) const ASL_SET_TRACK_TIME: Callee = Callee {
    addr: 0x0031_3cf0,
    f: ee::s10::AnimSequence_SetTrackTime,
};

pub(crate) const ASL_SET_TRACK_LOOP: Callee = Callee {
    addr: 0x0031_3d40,
    f: ee::s10::AnimSequence_SetTrackLoop,
};

/// `AnimSequence *AnimSeqList_PushLoopingAnim(AnimSeqList *list, int arg,
/// int anim, float time)` (0x314588): push a new sequence (`PushNew(list,
/// arg)`) whose track 0 plays `anim` from `time`, looping. Returns the
/// sequence.
pub fn anim_seq_list_push_looping_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x10);
        frame.save_fpr(c, 20, 0x30);
        c.g[S1][0] = c.g[A2][0]; // anim
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.mov_s(20, 12); // time
        ASL_PUSH_NEW.call(c, 0x0031_45a8)?;
        c.g[S0][0] = c.g[V0][0]; // the new sequence
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        ASL_SET_TRACK_ANIM.call(c, 0x0031_45bc)?;
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        ASL_SET_TRACK_TIME.call(c, 0x0031_45cc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 1;
        ASL_SET_TRACK_LOOP.call(c, 0x0031_45dc)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void AnimSequence_FreeTriggers(AnimSequence* self)` (0x313c08): pop and
/// `delete` every node of the trigger list at `+0xac` (linked through
/// `+0xc`), leaving the list empty.
pub fn anim_sequence_free_triggers(c: &mut Cpu) {
    const TRIGGERS: u32 = 0xac;
    const NEXT: u32 = 0xc;
    const LOOP_HEAD: u32 = 0x0031_3c28;
    const BUILTIN_DELETE: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::__builtin_delete,
    };
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(TRIGGERS).lw(c);
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 1;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(TRIGGERS).lw(c);
            loop {
                c.cycles += 6;
                let next = Addr(c.g[A0][0] as u32).off(NEXT).lw(c);
                c.g[V0][0] = next;
                Addr(c.g[S0][0] as u32).off(TRIGGERS).sw(c, next);
                BUILTIN_DELETE.call(c, 0x0031_3c34)?;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(TRIGGERS).lw(c);
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                    break;
                }
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(TRIGGERS).lw(c);
                if event_due(c) {
                    deliver_event_here(c, LOOP_HEAD);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AnimSequence* AnimSeqList_PushAnim(AnimSeqList* list, int arg, int anim,
/// float time)` (0x314518): push a new sequence (`PushNew(list, arg)`) whose
/// track 0 plays `anim` once from `time` (not looping). Returns the
/// sequence. Frame 0x40 (ra at 0, s1 at 0x10, s0 at 0x20, f20 at 0x30).
pub fn anim_seq_list_push_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x10);
        frame.save_fpr(c, 20, 0x30);
        c.g[S1][0] = c.g[A2][0]; // anim
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.mov_s(20, 12); // time
        ASL_PUSH_NEW.call(c, 0x0031_4538)?;
        c.g[S0][0] = c.g[V0][0]; // the new sequence
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        ASL_SET_TRACK_ANIM.call(c, 0x0031_454c)?;
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        ASL_SET_TRACK_TIME.call(c, 0x0031_455c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        ASL_SET_TRACK_LOOP.call(c, 0x0031_456c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimSequence_AddTrigger.

/// `AnimSequence::triggers` (singly linked list head).
pub(crate) const ANIM_SEQUENCE_TRIGGERS: u32 = 0xac;

/// The allocation name `D_0048A5B0`.
pub(crate) const ANIM_TRIGGER_ALLOC_NAME: u32 = 0x0048_a5b0;

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
        Callee {
            addr: 0x0031_7d70,
            f: ee::s10::EA_MemAllocNamed,
        }
        .call(c, 0x0031_3aa8)?;
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

/// `EA_MemAllocNamed` (0x317d70).
pub(crate) const OC286_MEM_ALLOC_NAMED: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::EA_MemAllocNamed,
};

/// The allocation name of event triggers (0x48a5b0) and the flags passed.
pub(crate) const EVENT_TRIGGER_ALLOC_NAME: u32 = 0x0049_0000 - 0x5a50;

pub(crate) const EVENT_TRIGGER_ALLOC_FLAGS: u64 = 0x2000_0000;

/// `AnimSequence` field: head of its event-trigger list.
pub(crate) const ANIM_SEQ_TRIGGERS_2: u32 = 0xac;

/// Event-trigger node (0x10 bytes): key, active flag, a cleared word, next.
pub(crate) const TRIGGER_KEY: u32 = 0x0;

pub(crate) const TRIGGER_ACTIVE: u32 = 0x4;

pub(crate) const TRIGGER_CLEARED: u32 = 0x8;

pub(crate) const TRIGGER_NEXT: u32 = 0xc;

pub(crate) const TRIGGER_SIZE: u64 = 0x10;

/// `void AnimSequence_AddEventTrigger(AnimSequence* self, int key)`
/// (0x313ad8): allocates a 16-byte trigger node `{key, active = 1, 0,
/// next}` and pushes it on the front of the sequence's trigger list.
pub fn anim_sequence_add_event_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = EVENT_TRIGGER_ALLOC_FLAGS;
        frame.save(c, S1, 0x10);
        c.g[A3][0] = 0;
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = TRIGGER_SIZE;
        c.g[A1][0] = sx(EVENT_TRIGGER_ALLOC_NAME);
        OC286_MEM_ALLOC_NAMED.call(c, 0x0031_3b08)?;
        let seq = Addr(r32(c, S0));
        let node = Addr(r32(c, V0));
        c.g[A0][0] = seq.off(ANIM_SEQ_TRIGGERS_2).lw(c);
        c.g[V1][0] = 1;
        seq.off(ANIM_SEQ_TRIGGERS_2).sw(c, c.g[V0][0]);
        node.off(TRIGGER_KEY).sw(c, c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        node.off(TRIGGER_ACTIVE).sw(c, c.g[V1][0]);
        node.off(TRIGGER_NEXT).sw(c, c.g[A0][0]);
        node.off(TRIGGER_CLEARED).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimSeqList_RemoveEventTrigger (0x314718)

/// AnimSeqList -> first sequence.
pub(crate) const SEQ_LIST_HEAD: u32 = 0x4;

/// AnimSequence -> next sequence in the list.
pub(crate) const SEQ_NEXT_2: u32 = 0xc8;

pub(crate) const SEQ_REMOVE_EVENT_TRIGGER: Callee = Callee {
    addr: 0x0031_3ba8,
    f: ee::s10::AnimSequence_RemoveEventTrigger,
};

/// `void AnimSeqList::RemoveEventTrigger(int id)` (0x314718): removes the
/// event trigger `id` from every sequence of the list
/// (`AnimSequence_RemoveEventTrigger(seq, id)`, following `seq+0xc8`).
/// Frame 0x30 (s0 = sequence, s1 = id).
pub fn anim_seq_list_remove_event_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        let list = c.g[A0][0] as u32;
        c.g[S0][0] = Addr(list.wrapping_add(SEQ_LIST_HEAD)).lw(c);
        c.g[S1][0] = c.g[A1][0];
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
            loop {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                SEQ_REMOVE_EVENT_TRIGGER.call(c, 0x0031_4740)?;
                let seq = c.g[S0][0] as u32;
                c.g[S0][0] = Addr(seq.wrapping_add(SEQ_NEXT_2)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                if c.g[S0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_4738);
                }
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
// AnimSeqList_FadeOutAll (0x3146d0)

/// `AnimSeqList`: first sequence node (`+4`); `AnimSequence`: next node
/// (`+0xc8`).
pub(crate) const OC300_SEQLIST_FIRST: u32 = 0x4;

pub(crate) const OC300_SEQ_NEXT: u32 = 0xc8;

/// `AnimSequence_FadeOut` (0x313a20).
pub(crate) const OC300_SEQ_FADE_OUT: Callee = Callee {
    addr: 0x0031_3a20,
    f: ee::s10::AnimSequence_FadeOut,
};

/// `void AnimSeqList_FadeOutAll(AnimSeqList* list, float time)` (0x3146d0):
/// fades out every sequence of the list (`AnimSequence_FadeOut(seq, time)`
/// along the `+0xc8` links); `time` is kept in f20 across the calls.
pub fn anim_seq_list_fade_out_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save_fpr(c, 20, 0x20);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = Addr(r32(c, A0)).off(OC300_SEQLIST_FIRST).lw(c);
        c.f[20] = c.f[12];
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
            loop {
                c.cycles += 5;
                c.f[12] = c.f[20];
                OC300_SEQ_FADE_OUT.call(c, 0x0031_46f8)?;
                c.g[S0][0] = Addr(r32(c, S0)).off(OC300_SEQ_NEXT).lw(c);
                c.g[A0][0] = c.g[S0][0];
                if c.g[S0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_46f0);
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::bx::appman::tests::ptr as ptr_2;
    pub(crate) use crate::game::bx::appman::tests::rng;
    pub(crate) use crate::game::bx::appman::tests::SEEDS;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;

    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_4 as put_str;

    pub(crate) use crate::game::fe::feasyncfile::tests::w;

    pub(crate) use crate::game::fe::feasyncfile::tests::wh;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_2;

    pub(crate) use crate::game::visualfx::worldlightman::tests::f;
    pub(crate) use crate::game::visualfx::worldlightman::tests::pick;
    use ssx3_ee as ee;

    use ssx3_ee::s10;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn anim_sequence_set_track_weight_matches() {
        check(
            s10::f_00313d28__generated,
            anim_sequence_set_track_weight,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 6);
                scramble(c, &mut r);
                let seq = ptr_2(&mut r, 0);
                c.g[A0][0] = sx(seq);
                c.g[A1][0] = (r.next_u32() % 6) as u64;
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn anim_sequence_set_track_loop_matches() {
        check(
            s10::f_00313d40__generated,
            anim_sequence_set_track_loop,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 7);
                scramble(c, &mut r);
                let seq = ptr_2(&mut r, 0);
                c.g[A0][0] = sx(seq);
                c.g[A1][0] = (r.next_u32() % 6) as u64;
                c.g[A2][0] = if seed % 2 == 0 { 0 } else { sx(r.next_u32()) };
            },
        );
    }

    #[test]
    fn anim_seq_list_init_matches() {
        check(
            ssx3_ee::s10::f_003142e8,
            anim_seq_list_init,
            100,
            &|c, seed| {
                let mut r = Rng(0x200d ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, r.next_u32());
                c.bus.write32(DATA + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn mdf_file_ctor_matches() {
        check(ssx3_ee::s10::f_00314978, mdf_file_ctor, 100, &|c, seed| {
            let mut r = Rng(0x200e ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn mdf_file_parse_header_matches() {
        check(
            ssx3_ee::s10::f_00314aa0,
            mdf_file_parse_header,
            200,
            &|c, seed| {
                let mut r = Rng(0x200f ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.bus.write16(DATA + 0x106, (r.next_u32() & 0xffff) as u16);
                c.bus.write32(DATA + 0x108, r.next_u32());
            },
        );
    }

    #[test]
    fn anim_seq_list_append_matches() {
        check_events(
            ssx3_ee::s10::f_00314368,
            anim_seq_list_append,
            80,
            &|c, seed| {
                let mut r = Rng(0x3143_6800 ^ seed as u64);
                scramble(c, &mut r);
                let list = DATA;
                c.g[A0][0] = list as u64;
                let seq = DATA + 0x800;
                c.g[A1][0] = seq as u64;
                // Always three or more links so every loop reaches the
                // back-edge event check.
                let n = 3 + (r.next_u32() % 3);
                let mut node = DATA + 0x1000;
                c.bus.write32(list + LIST_HEAD, node);
                for i in 0..n {
                    c.bus
                        .write32(node + SEQ_NEXT, if i + 1 < n { node + 0x10 } else { 0 });
                    node += 0x10;
                }
                c.bus.write32(list, r.next_u32());
                c.bus.write32(seq + SEQ_NEXT, r.next_u32());
                let mode = schedule(c, &mut r, seed, 0x0031_4368);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn anim_seq_list_append_short_matches() {
        check(
            ssx3_ee::s10::f_00314368,
            anim_seq_list_append,
            80,
            &|c, seed| {
                let mut r = Rng(0x3143_68f0 ^ seed as u64);
                scramble(c, &mut r);
                let list = DATA;
                c.g[A0][0] = list as u64;
                let seq = DATA + 0x800;
                c.g[A1][0] = seq as u64;
                let n = r.next_u32() % 4;
                let mut node = DATA + 0x1000;
                c.bus
                    .write32(list + LIST_HEAD, if n == 0 { 0 } else { node });
                for i in 0..n {
                    c.bus
                        .write32(node + SEQ_NEXT, if i + 1 < n { node + 0x10 } else { 0 });
                    node += 0x10;
                }
                c.bus.write32(list, r.next_u32());
                c.bus.write32(seq + SEQ_NEXT, r.next_u32());
            },
        );
    }

    // ------------------------------------------------------------ AnimSequence

    /// A sequence at DATA (random fields, no triggers so the trigger
    /// callees take their short paths).
    pub(crate) fn sequence(c: &mut Cpu, r: &mut Rng, s: u32) {
        fill(c, r, s, 0x34);
        c.bus.write32(s + 0xac, 0);
        for k in 0..3 {
            let t = s + k * 0x1c;
            c.bus.write32(t + 0x8, r.f32(4.0));
            c.bus.write32(t + 0x10, f(pick(r, &[0.5, 1.0, 2.0, 3.0])));
            c.bus.write32(t + 0x14, r.f32(1.0));
            c.bus
                .write32(t + 0x18, if r.next_u32() % 3 == 0 { 0 } else { 1 });
        }
        c.bus.write32(s + 0x94, r.f32(1.0));
        c.bus.write32(s + 0xa4, f(pick(r, &[0.0, 0.25, 0.5])));
        c.bus.write32(s + 0xa8, f(pick(r, &[0.0, 0.25, 0.5])));
    }

    #[test]
    fn anim_seq_list_collect_tracks_matches() {
        check(
            ee::s10::f_00314418__generated,
            anim_seq_list_collect_tracks,
            200,
            &|c, seed| {
                let mut r = Rng(0xc011_ec75 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                let list = DATA;
                let out = DATA + 0x8000;
                let mut link = list + 4;
                for k in 0..seed % 4 {
                    let s = DATA + 0x1000 + k * 0x400;
                    sequence(c, &mut r, s);
                    c.bus.write32(link, s);
                    link = s + 0xc8;
                }
                c.bus.write32(link, 0);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = sx(out);
            },
        );
    }

    #[test]
    fn mdf_file_dtor_matches() {
        check(
            ee::s10::f_00314988__generated,
            mdf_file_dtor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31_4988, 0);
                let obj = DATA + 0x1000;
                c.bus
                    .write32(obj, if seed % 3 == 0 { DATA + 0x2000 } else { 0 });
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = (seed % 4 == 0) as u64;
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn mdf_file_free_matches() {
        check(
            ee::s10::f_003149d0__generated,
            mdf_file_free,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31_49d0, 0);
                let obj = DATA + 0x1000;
                c.bus.write32(
                    obj,
                    match seed % 3 {
                        0 => 0,
                        1 => DATA + 0x2000,
                        _ => 4,
                    },
                );
                c.bus.write32(obj + 0xc, r.next_u32());
                c.g[A0][0] = sx(obj);
            },
        );
    }

    #[test]
    fn mdf_file_create_model_by_name_matches() {
        check(
            ee::s10::f_00314c00__generated,
            mdf_file_create_model_by_name,
            SEEDS_2,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let records = DATA + 0x1000;
                let names = DATA + 0x1100;
                let count = seed % 3;
                w(c, this + 4, records);
                wh(c, records + 4, count as u16);
                w(c, this + 8, names);
                let query = DATA + 0x9000;
                let words: [&[u8]; 3] = [b"alpha", b"beta", b"gamma"];
                put_str(c, query, words[(seed % 3) as usize]);
                for i in 0..count {
                    let s = names + 0x60 * i;
                    put_str(c, s, words[i as usize]);
                    if seed & 1 == 0 && i == 0 {
                        put_str(c, s, words[(seed % 3) as usize]);
                    }
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = query as u64;
            }),
        );
    }

    /// AnimSequence_Save: a stream whose Write is a fake (some seeds
    /// unwind, at any of the up to 21 calls); key lists of 0..4 keys.
    #[test]
    fn anim_sequence_save_matches() {
        covered(
            ee::s10::f_00313dd0__generated,
            anim_sequence_save,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31_3dd0, 21);
                c.pc = 0x0031_3dd0;
                let seq = DATA + 0x1000;
                let stream = DATA + 0x2000;
                let vt = DATA + 0x3000;
                fill(c, &mut r, seq, 0x34);
                c.bus.write32(stream, vt);
                c.bus.write16(vt + 0x8, ((r.next_u32() & 0x3f) * 4) as u16);
                c.bus.write32(vt + 0xc, FAKE);
                let n = seed % 5;
                let mut next = 0;
                for k in (0..n).rev() {
                    let key = DATA + 0x4000 + 0x40 * k;
                    fill(c, &mut r, key, 4);
                    c.bus.write32(key + 0xc, next);
                    next = key;
                }
                c.bus.write32(seq + 0xac, next);
                c.g[A0][0] = seq as u64;
                c.g[A1][0] = stream as u64;
            },
        );
    }

    pub(crate) use crate::game::ai::ai::tests::oc258_w32;

    /// `AnimSeqList_Save`: a stream whose Write is a fake (some seeds
    /// unwind, at any call), lists of -1..3 sequences (each with 0-1 keys)
    /// saved by the real `AnimSequence_Save`.
    #[test]
    fn anim_seq_list_save_matches() {
        use crate::difftest::fakes::{covered, FAKE};
        use crate::difftest::fill;
        covered(
            ee::s10::f_003147f0__generated,
            anim_seq_list_save,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_47f0, 40);
                c.pc = 0x0031_47f0;
                let list = DATA + 0x1000;
                let stream = DATA + 0x2000;
                let vt = DATA + 0x3000;
                c.bus.write32(stream, vt);
                c.bus.write16(vt + 0x8, ((r.next_u32() & 0x3f) * 4) as u16);
                c.bus.write32(vt + 0xc, FAKE);
                let count = (seed % 5) as i32 - 1;
                let mut next = 0;
                for k in (0..count.max(0) as u32).rev() {
                    let seq = DATA + 0x4000 + 0x200 * k;
                    fill(c, &mut r, seq, 0x34);
                    let key = if r.next_u32() & 1 != 0 {
                        let key = seq + 0x100;
                        fill(c, &mut r, key, 4);
                        c.bus.write32(key + 0xc, 0);
                        key
                    } else {
                        0
                    };
                    c.bus.write32(seq + 0xac, key);
                    c.bus.write32(seq + SEQLIST_NODE_NEXT, next);
                    next = seq;
                }
                c.bus.write32(list + SEQLIST_COUNT, count as u32);
                c.bus.write32(list + SEQLIST_FIRST, next);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = sx(stream);
            },
        );
    }

    /// The BE-library dummy at gp-0x1f04 and a dummy world at gp-0x848.
    pub(crate) fn oc258_env(c: &mut Cpu) {
        oc258_w32(c, GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
        let wld = DATA + 0x1000;
        oc258_w32(c, GAME_GP.wrapping_sub(0x848), wld);
        oc258_w32(c, wld + 0x84, DATA + 0x1200);
        oc258_w32(c, DATA + 0x1200 + 0x20, DATA + 0x3000);
        oc258_w32(c, DATA + 0x1200 + 0x28, DATA + 0x1400);
        oc258_w32(c, wld + 0x7c, DATA + 0x1500);
    }

    /// A setup call into a generated function (returns to a sentinel).
    pub(crate) fn oc258_run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
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
        check(
            ee::s10::f_00313ba8__generated,
            anim_sequence_remove_event_trigger,
            120,
            &|c, seed| {
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
                let n = if seed % 6 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 6
                };
                let mut link = seq + ANIM_SEQ_TRIGGERS;
                for _ in 0..n {
                    let node = oc258_run(
                        c,
                        0x0031_e818,
                        ee::s10::DLHeap_Malloc,
                        &[MEM_HEAP as u64, 0x10],
                    ) as u32;
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
            },
        );
    }

    use crate::difftest::fakes::{self};

    /// AnimSeqList_PushNew: the real allocator (size-class pools ready) and
    /// the real AnimSequence_ctor; an empty or non-empty list.
    #[test]
    fn anim_seq_list_push_new_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, elf_image, DATA};
        use crate::game::bx::bxstring::tests::pool;
        check(
            ee::s10::f_003142f8__generated,
            anim_seq_list_push_new,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_42f8, 0);
                elf_image(c);
                pool(c);
                c.pc = 0x0031_42f8;
                let list = DATA + 0x100;
                c.bus.write32(list + PUSH_NEW_LIST_COUNT, seed % 5);
                let first = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
                c.bus.write32(list + PUSH_NEW_LIST_FIRST, first);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = (r.next_u32() % 16) as u64;
            },
        );
    }

    // ------------------------------------------------ AnimSeqList_PushLoopingAnim

    /// Gate of the plain logged fakes below (without it they run the
    /// generated body).
    pub(crate) const ASL_CTRL: u32 = DATA + 0x5_9000;

    pub(crate) const ASL_MAGIC: u32 = 0x3145_88a5;

    macro_rules! asl_gated {
        ($name:ident, $addr:expr, $gen:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(ASL_CTRL) != ASL_MAGIC {
                    return $gen(c);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
            }
        };
    }

    asl_gated!(asl_fake_alloc, 0x0031_7d70, ee::s10::f_00317d70__generated);

    asl_gated!(asl_fake_ctor, 0x0031_3508, ee::s10::f_00313508__generated);

    asl_gated!(
        asl_fake_track_anim,
        0x0031_3c50,
        ee::s10::f_00313c50__generated
    );

    asl_gated!(
        asl_fake_track_time,
        0x0031_3cf0,
        ee::s10::f_00313cf0__generated
    );

    asl_gated!(
        asl_fake_track_loop,
        0x0031_3d40,
        ee::s10::f_00313d40__generated
    );

    pub(crate) fn asl_install_fakes() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| unsafe {
            ee::s10::OVERRIDE_00317D70 = asl_fake_alloc;
            ee::s10::OVERRIDE_00313508 = asl_fake_ctor;
            ee::s10::OVERRIDE_00313C50 = asl_fake_track_anim;
            ee::s10::OVERRIDE_00313CF0 = asl_fake_track_time;
            ee::s10::OVERRIDE_00313D40 = asl_fake_track_loop;
        });
    }

    /// A list with a few sequences; the real `PushNew` links the new one in
    /// (its allocator and constructor are logged fakes returning a node),
    /// the three track setters are logged fakes (one may unwind).
    #[test]
    fn anim_seq_list_push_looping_anim_matches() {
        asl_install_fakes();
        covered(
            ee::s10::f_00314588__generated,
            anim_seq_list_push_looping_anim,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_4588, 5);
                c.pc = 0x0031_4588;
                c.bus.write32(ASL_CTRL, ASL_MAGIC);
                let list = DATA + 0x5_0000;
                c.bus.write32(list, r.next_u32() % 8);
                c.bus
                    .write32(list + 4, if seed % 3 == 0 { 0 } else { DATA + 0x5_1000 });
                c.g[A0][0] = sx(list);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = sx(r.next_u32() % 0x400);
                c.f[12] = r.edge_f32();
            },
        );
    }

    /// AnimSequence_FreeTriggers: a trigger list of 0..4 nodes (the
    /// allocator's mutex set up as in the other list-freeing tests).
    #[test]
    fn anim_sequence_free_triggers_matches() {
        use crate::game::ai::aianimeventmap::tests::alloc_setup;
        check(
            ee::s10::f_00313c08__generated,
            anim_sequence_free_triggers,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x31_3c08, 0);
                alloc_setup(c, &mut r);
                let seq = DATA;
                let count = seed % 5;
                let mut head = 0;
                for k in (0..count).rev() {
                    let node = DATA + 0x1000 + 0x100 * k;
                    fill(c, &mut r, node, 4);
                    c.bus.write32(node + 0xc, head);
                    head = node;
                }
                c.bus.write32(seq + 0xac, head);
                c.g[A0][0] = sx(seq);
                c.pc = 0x0031_3c08;
            },
        );
    }

    /// AnimSeqList_PushAnim: a list with a few sequences; the real `PushNew`
    /// links the new one in (its allocator and constructor are logged fakes
    /// returning a node), the three track setters are logged fakes (one may
    /// unwind). Same fixture as `anim_seq_list_push_looping_anim_matches`.
    #[test]
    fn anim_seq_list_push_anim_matches() {
        use crate::difftest::fakes::{covered, machine};
        use crate::game::animation::animsequencer::tests::{
            asl_install_fakes, ASL_CTRL, ASL_MAGIC,
        };
        asl_install_fakes();
        covered(
            ee::s10::f_00314518__generated,
            anim_seq_list_push_anim,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_4518, 5);
                c.pc = 0x0031_4518;
                c.bus.write32(ASL_CTRL, ASL_MAGIC);
                let list = DATA + 0x5_0000;
                c.bus.write32(list, r.next_u32() % 8);
                c.bus
                    .write32(list + 4, if seed % 3 == 0 { 0 } else { DATA + 0x5_1000 });
                c.g[A0][0] = sx(list);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = sx(r.next_u32() % 0x400);
                c.f[12] = r.edge_f32();
            },
        );
    }

    /// AnimSequence_AddTrigger: the real EA_MemAllocNamed on the engine's
    /// pool allocator; an empty or non-empty trigger list.
    #[test]
    fn anim_sequence_add_trigger_matches() {
        use crate::difftest::fakes::machine;
        use crate::game::ui::uianimation::tests::pool_allocator;
        check(
            ee::s10::f_00313a70__generated,
            anim_sequence_add_trigger,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31_3a70, 0);
                c.pc = 0x0031_3a70;
                pool_allocator(c);
                let seq = DATA + 0x6_0000;
                let head = if seed % 2 == 0 { 0 } else { DATA + 0x6_1000 };
                c.bus.write32(seq + ANIM_SEQUENCE_TRIGGERS, head);
                c.g[A0][0] = sx(seq);
                c.g[A1][0] = sx(r.next_u32());
                c.f[12] = r.f32(5.0);
            },
        );
    }

    /// A real allocation from the pools/heap, pushed on an empty or
    /// non-empty trigger list.
    #[test]
    fn anim_sequence_add_event_trigger_matches() {
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        check(
            ee::s10::f_00313ad8__generated,
            anim_sequence_add_event_trigger,
            100,
            &|c, seed| {
                let mut r = alloc_machine(c, seed, 0x31_3ad8);
                let seq = DATA + 0x100;
                let head = if seed % 3 == 0 { 0 } else { DATA + 0x200 };
                c.bus.write32(seq + ANIM_SEQ_TRIGGERS_2, head);
                c.g[A0][0] = sx(seq);
                c.g[A1][0] = sx(r.next_u32());
                c.pc = 0x0031_3ad8;
            },
        );
    }

    use crate::difftest::events::driven;
    use crate::difftest::RA as SENTINEL;

    /// AnimSeqList_RemoveEventTrigger: empty lists and lists of 1-4
    /// sequences, each with no triggers or a short chain of triggers whose
    /// ids never match (the real callee walks them and frees nothing).
    #[test]
    fn anim_seq_list_remove_event_trigger_matches() {
        use crate::difftest::fakes::machine;
        check(
            ee::s10::f_00314718__generated,
            anim_seq_list_remove_event_trigger,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31_4718, 0);
                let list = DATA + 0x20_0000;
                let seqs = list + 0x100;
                let id = r.next_u32() % 4;
                let n = seed % 5;
                c.bus
                    .write32(list + SEQ_LIST_HEAD, if n == 0 { 0 } else { seqs });
                for i in 0..n {
                    let seq = seqs + 0x200 * i;
                    let next = if i + 1 == n { 0 } else { seq + 0x200 };
                    c.bus.write32(seq + SEQ_NEXT_2, next);
                    // Triggers {id, flag, -, next}, never the removed id.
                    let trig = seq + 0x100;
                    let count = r.next_u32() % 3;
                    c.bus.write32(seq + 0xac, if count == 0 { 0 } else { trig });
                    for t in 0..count {
                        let tr = trig + 0x20 * t;
                        let tid = id + 1 + r.next_u32() % 8;
                        c.bus.write32(tr, tid);
                        c.bus.write32(tr + 4, r.next_u32() % 2);
                        c.bus
                            .write32(tr + 0xc, if t + 1 == count { 0 } else { tr + 0x20 });
                    }
                }
                c.g[A0][0] = sx(list);
                c.g[A1][0] = id as u64;
                c.pc = 0x0031_4718;
            },
        );
    }

    /// A list of 0..4 sequences (linked through `+0xc8`) with fade fields
    /// that reach every path of the real `AnimSequence_FadeOut` (`+0x98`
    /// and `+0x9c` zero or not, the new time below or above the old one).
    pub(crate) fn oc300_fade_setup(c: &mut Cpu, seed: u32, count: u32) {
        let mut r = Rng(0x3146_d000 ^ seed as u64);
        scramble(c, &mut r);
        let list = DATA + 0x100;
        let mut next = 0;
        for k in (0..count).rev() {
            let seq = DATA + 0x1000 + 0x100 * k;
            crate::difftest::fill(c, &mut r, seq, 0x40);
            let fading = match r.next_u32() % 3 {
                0 => 0,
                _ => r.f32(5.0),
            };
            c.bus.write32(seq + 0x98, fading);
            let old = match r.next_u32() % 3 {
                0 => 0,
                _ => r.f32(5.0),
            };
            c.bus.write32(seq + 0x9c, old);
            c.bus.write32(seq + OC300_SEQ_NEXT, next);
            next = seq;
        }
        c.bus.write32(list + OC300_SEQLIST_FIRST, next);
        c.g[A0][0] = sx(list);
        c.f[12] = if seed % 7 == 3 {
            r.edge_f32()
        } else {
            r.f32(5.0)
        };
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0031_46d0;
    }

    driven!(oc300_gen_fade_out_all, ee::s10::f_003146d0__generated);

    driven!(oc300_hand_fade_out_all, anim_seq_list_fade_out_all);

    /// Empty list and lists of 1..4 sequences with the real
    /// `AnimSequence_FadeOut`; then with events at the loop's back edge.
    #[test]
    fn anim_seq_list_fade_out_all_matches() {
        use crate::difftest::events::{check_events, schedule};
        check(
            ee::s10::f_003146d0__generated,
            anim_seq_list_fade_out_all,
            300,
            &|c, seed| oc300_fade_setup(c, seed, seed % 5),
        );
        check_events(
            oc300_gen_fade_out_all,
            oc300_hand_fade_out_all,
            90,
            &|c, seed| {
                oc300_fade_setup(c, seed, 2 + seed / 3 % 3);
                let mut r = Rng(0x3146_d0 ^ seed as u64);
                let mode = schedule(c, &mut r, seed, 0x0031_46d0);
                c.next_event = c.cycles + 4 + (r.next_u32() % 16) as u64;
                mode
            },
        );
    }

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
}
