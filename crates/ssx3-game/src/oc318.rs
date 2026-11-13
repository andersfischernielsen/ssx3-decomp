//! Staging module oc318 (wave wave213): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc318`.

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
    (0x0013_fad8, obj_ptr_array_sort, "ObjPtrArray_Sort"),
    (
        0x0035_5cb0,
        mod_obj_add_particle_modifier,
        "ModObj_AddParticleModifier",
    ),
    (
        0x002b_1520,
        speech_system_apply_event_context,
        "SpeechSystem_ApplyEventContext",
    ),
    (
        0x0026_e6e0,
        replay_highlight_slot_begin,
        "Replay_HighlightSlotBegin",
    ),
    (0x003c_7600, snd_mix_declick16, "SND_MixDeclick16"),
    (
        0x0012_fb68,
        rider_natural_air_sub_exit,
        "RiderNaturalAirSub_Exit",
    ),
    (
        0x0030_c6c8,
        wscript_man_collect_set_save,
        "WScriptMan_CollectSetSave",
    ),
];

// ---------------------------------------------------------------------------
// ObjPtrArray_Sort (0x13fad8)

/// `ObjPtrArray_MergeSort(first, last, buf, cmp)` (0x13f8f8).
pub(crate) const OBJ_PTR_ARRAY_MERGE_SORT: Callee = Callee {
    addr: 0x0013_f8f8,
    f: ee::s01::ObjPtrArray_MergeSort,
};

/// The stack frame's saved-`ra` slot.
pub(crate) const OBJ_PTR_ARRAY_SORT_FRAME: u32 = 0x10;

/// `int ObjPtrArray_Sort(T** first, T** last, T** out, const InstanceCompare& cmp)`
/// (0x13fad8): sort the object-pointer range `[first, last)` into `*out`.
/// Empty (or inverted) range: nothing, returns 1; one element: copies it and
/// returns the pointer; two or more: the merge sort fills `*out` and its
/// result is returned. Frame 0x10 (`ra` +0).
pub fn obj_ptr_array_sort(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        let frame = Frame::open(c, OBJ_PTR_ARRAY_SORT_FRAME);
        // n = (last - first) / 4, an arithmetic shift as the original does.
        c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        let fewer_than_two = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if fewer_than_two {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let not_one = c.g[V1][0] != c.g[V0][0];
            frame.restore_ra(c, 0);
            if !not_one {
                c.cycles += 2;
                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                c.sw(c.g[A2][0] as u32, c.g[V0][0]);
            }
        } else {
            c.cycles += 4;
            OBJ_PTR_ARRAY_MERGE_SORT.call(c, 0x0013_faf8)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModObj_AddParticleModifier (0x355cb0)

/// `EA_MemAllocNamed(size, name, flags, align)` (0x317d70).
pub(crate) const MOD_OBJ_ADD_PARTICLE_ALLOC: u32 = 0x0031_7d70;
/// `ParticleModifier_ctor(self, desc, link)` (0x3458c0).
pub(crate) const MOD_OBJ_ADD_PARTICLE_CTOR: Callee = Callee {
    addr: 0x0034_58c0,
    f: ee::s11::ParticleModifier_ctor,
};
/// `ModObj_LinkParticleModifier(self, mod)` (0x355658).
pub(crate) const MOD_OBJ_ADD_PARTICLE_LINK: Callee = Callee {
    addr: 0x0035_5658,
    f: ee::s11::ModObj_LinkParticleModifier,
};

/// `void ModObj_AddParticleModifier(cMoveNode* self, void* desc)` (0x355cb0):
/// allocate a 0x1f0-byte particle modifier (named `D_0048e978`), construct it
/// with `desc` and `self->field_18`, and link it into the node's modifier
/// list. Frame 0x30 (ra +0, s0 +0x20, s1 +0x10).
pub fn mod_obj_add_particle_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A1][0]; // desc
        c.g[S0][0] = c.g[A0][0]; // self
        c.g[A1][0] = 0x0049_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x1f0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1688)); // D_0048e978
        call_ptr(c, MOD_OBJ_ADD_PARTICLE_ALLOC, 0x0035_5ce0)?;
        // ParticleModifier_ctor(mod, desc, self->field_18)
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        MOD_OBJ_ADD_PARTICLE_CTOR.call(c, 0x0035_5cf0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        MOD_OBJ_ADD_PARTICLE_LINK.call(c, 0x0035_5cfc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SpeechSystem_ApplyEventContext (0x2b1520)

/// `sBMon.ents` entry `j`, as the body addresses it: `self + j*0x20`, with the
/// fields at these absolute offsets (`ents[0]` is `self + 0x18`, i.e. the
/// `used` word of entry 0).
#[cfg(test)]
pub(crate) const SPEECH_EVENT_ENT_SIZE: u32 = 0x20;
/// `sBEnt_1520` fields, relative to `self + j*0x20`.
pub(crate) const SPEECH_EVENT_ENT_USED: u32 = 0x18;
pub(crate) const SPEECH_EVENT_ENT_ID: u32 = 0x20;
pub(crate) const SPEECH_EVENT_ENT_KEY: u32 = 0x24;
pub(crate) const SPEECH_EVENT_ENT_OBJ: u32 = 0x28;
pub(crate) const SPEECH_EVENT_ENT_W: u32 = 0x2c;
pub(crate) const SPEECH_EVENT_ENT_T: u32 = 0x30;
pub(crate) const SPEECH_EVENT_ENT_K: u32 = 0x34;
/// `sBMon` fields.
pub(crate) const SPEECH_EVENT_SLOTS: u32 = 0x04;
pub(crate) const SPEECH_EVENT_CUR_K: u32 = 0x0c;
pub(crate) const SPEECH_EVENT_CUR_ID: u32 = 0x10;
/// `sBMObj_1520`: the voice (a `cBMVoice_1520`) and the stored `z`.
pub(crate) const SPEECH_EVENT_OBJ_VOICE: u32 = 0x6c0;
pub(crate) const SPEECH_EVENT_OBJ_Z: u32 = 0x870;
/// Virtual slots of the voice: `v05()` at +0x28 and `v08()` at +0x40.
pub(crate) const SPEECH_EVENT_V05: u32 = 0x28;
pub(crate) const SPEECH_EVENT_V08: u32 = 0x40;
/// Where the found entry writes the voice's state (`slots[id]` fields).
pub(crate) const SPEECH_EVENT_SLOT_V05: u32 = 0x8cc;
pub(crate) const SPEECH_EVENT_SLOT_V08: u32 = 0x8f8;
pub(crate) const SPEECH_EVENT_SLOT_Z: u32 = 0x8fc;
pub(crate) const SPEECH_EVENT_SLOT_T: u32 = 0x8f4;
pub(crate) const SPEECH_EVENT_SLOT_W: u32 = 0x8d0;

/// `int SpeechSystem_ApplyEventContext(sBMon* self, int id, int key)`
/// (0x2b1520): find the used entry of the ten whose `(id, key)` matches; copy
/// its voice's `v05`/`v08`/`z`/`t`/`w` into `self->slots[id]`, remember its
/// `k` (or clear the current key when `k` is 0) and return 1. No match: 0.
/// Frame 0x70.
pub fn speech_system_apply_event_context(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S5, 0x10);
        c.g[S0][0] = c.g[A0][0]; // self
        frame.save(c, S3, 0x30);
        c.g[S5][0] = c.g[A1][0]; // id
        frame.save(c, S1, 0x50);
        c.g[S3][0] = sx((c.g[S5][0] as u32) << 2); // id * 4 (slots index)
        frame.save(c, S2, 0x40);
        c.g[A0][0] = 0; // j
        frame.save(c, S4, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((c.g[A0][0] as u32) << 5); // j * 0x20
        'search: loop {
            'b_2b167c: {
                'b_2b166c: {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(SPEECH_EVENT_ENT_USED));
                    if c.g[V1][0] == 0 {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                        break 'b_2b166c;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(SPEECH_EVENT_ENT_ID));
                    if c.g[V1][0] != c.g[S5][0] {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                        break 'b_2b166c;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(SPEECH_EVENT_ENT_KEY));
                    if c.g[V1][0] != c.g[A2][0] {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                        break 'b_2b166c;
                    }
                    // The entry matches: read the voice's v05/v08 and the z.
                    'b_2b15e0: {
                        c.cycles += 7;
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_ENT_OBJ));
                        c.g[S4][0] = 0;
                        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        c.g[A3][0] = 0;
                        c.g[A1][0] = c.lw(c.g[S2][0] as u32); // the entry's object
                        let no_object = c.g[A1][0] == 0;
                        c.g[T0][0] = sx(0xffff_ffff);
                        if no_object {
                            break 'b_2b15e0;
                        }
                        c.cycles += 17;
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(SPEECH_EVENT_OBJ_VOICE));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(SPEECH_EVENT_OBJ_VOICE));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_V05));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_V05 + 4));
                        let v05 = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, v05, 0x002b_15b4)?;
                        c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                        c.g[S4][0] = c.g[V0][0]; // v = voice->v05()
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(SPEECH_EVENT_OBJ_VOICE));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(SPEECH_EVENT_OBJ_VOICE));
                        c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_V08));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_V08 + 4));
                        let v08 = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        call_ptr(c, v08, 0x002b_15d4)?;
                        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                        c.g[A3][0] = c.g[V0][0]; // u = voice->v08()
                        c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_OBJ_Z));
                        break 'b_2b15e0;
                    }
                    'b_2b1664: {
                        'b_2b165c: {
                            c.cycles += 28;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_SLOTS));
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                            c.g[A2][0] = c.g[A0][0];
                            c.g[A1][0] = c.g[A0][0];
                            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            c.sw(
                                (c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_SLOT_V05),
                                c.g[S4][0],
                            );
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_SLOTS));
                            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            c.sw(
                                (c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_SLOT_V08),
                                c.g[A3][0],
                            );
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_SLOTS));
                            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            c.sw(
                                (c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_SLOT_Z),
                                c.g[T0][0],
                            );
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_SLOTS));
                            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(SPEECH_EVENT_ENT_T));
                            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_SLOT_T));
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_SLOTS));
                            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(SPEECH_EVENT_ENT_W));
                            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            c.sw(
                                (c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_SLOT_W),
                                c.g[A0][0],
                            );
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(SPEECH_EVENT_ENT_K));
                            if c.g[V0][0] == 0 {
                                c.g[V0][0] = sx(0xffff_ffff);
                                break 'b_2b165c;
                            }
                            c.cycles += 3;
                            c.sw(
                                (c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_CUR_ID),
                                c.g[S5][0],
                            );
                            c.sw(
                                (c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_CUR_K),
                                c.g[V0][0],
                            );
                            break 'b_2b1664;
                        }
                        c.cycles += 2;
                        c.sw((c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_CUR_K), 0);
                        c.sw(
                            (c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_CUR_ID),
                            c.g[V0][0],
                        );
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    break 'b_2b167c;
                }
                // Next entry, or out of entries.
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < 10) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[A0][0] as u32) << 5);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_1558);
                    }
                    continue 'search;
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'b_2b167c;
            }
            c.cycles += 9;
            break 'search;
        }
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay_HighlightSlotBegin (0x26e6e0)

/// `Replay_SnapshotSetHighlightValue(rider, index, 0)` (0x26d730).
pub(crate) const REPLAY_HIGHLIGHT_SET_VALUE: u64 = 0x0026_d730;
/// `sReplayObj` fields: the slot index, its data block, the attached rider.
pub(crate) const REPLAY_HIGHLIGHT_INDEX: u32 = 0x00;
pub(crate) const REPLAY_HIGHLIGHT_DATA: u32 = 0x04;
pub(crate) const REPLAY_HIGHLIGHT_RIDER: u32 = 0xb0;
/// The data block: 0xa0 bytes copied in 0x20-byte chunks, then 0xc more.
pub(crate) const REPLAY_HIGHLIGHT_CHUNK: u32 = 0x20;
pub(crate) const REPLAY_HIGHLIGHT_BULK: u32 = 0xa0;
/// The rider's highlight flag byte (bit per slot).
pub(crate) const REPLAY_HIGHLIGHT_FLAGS: u32 = 0x1e;

/// `void Replay_HighlightSlotBegin(sReplayObj* self, char* rider, sReplayData* src)`
/// (0x26e6e0): mark the rider's per-slot highlight bit, tell the snapshot
/// system, attach the rider to the slot and copy its 0xac-byte data block.
/// Frame 0x40.
pub fn replay_highlight_slot_begin(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0]; // rider
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0]; // self
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0]; // src
        c.g[A2][0] = 0;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_INDEX));
        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_FLAGS));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A0][0] as u32) & 31));
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.sb(
            (c.g[S0][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_FLAGS),
            c.g[V1][0],
        );
        c.g[RA][0] = sx(0x0026_e728);
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_INDEX));
        call_ptr(c, REPLAY_HIGHLIGHT_SET_VALUE as u32, 0x0026_e728)?;
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_DATA));
        c.g[V0][0] = c.g[S2][0] | c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] & 0x7;
        let aligned = c.g[V0][0] == 0;
        c.sw(
            (c.g[S1][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_RIDER),
            c.g[S0][0],
        );
        if aligned {
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_BULK));
            loop {
                c.cycles += 11;
                c.g[A0][0] = c.ld(c.g[S2][0] as u32);
                c.g[A1][0] = c.ld((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = c.ld((c.g[S2][0] as u32).wrapping_add(0x10));
                c.g[A3][0] = c.ld((c.g[S2][0] as u32).wrapping_add(0x18));
                c.sd(c.g[V1][0] as u32, c.g[A0][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_CHUNK));
                let more = c.g[S2][0] != c.g[V0][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_CHUNK));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_e798);
                    }
                    continue;
                }
                break;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_BULK));
            loop {
                c.cycles += 19;
                c.ldl(4, (c.g[S2][0] as u32).wrapping_add(0x7));
                c.ldr(4, c.g[S2][0] as u32);
                c.ldl(5, (c.g[S2][0] as u32).wrapping_add(0xf));
                c.ldr(5, (c.g[S2][0] as u32).wrapping_add(0x8));
                c.ldl(6, (c.g[S2][0] as u32).wrapping_add(0x17));
                c.ldr(6, (c.g[S2][0] as u32).wrapping_add(0x10));
                c.ldl(7, (c.g[S2][0] as u32).wrapping_add(0x1f));
                c.ldr(7, (c.g[S2][0] as u32).wrapping_add(0x18));
                c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.sdr(4, c.g[V1][0] as u32);
                c.sdl(5, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.sdr(5, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x17));
                c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x1f));
                c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x18));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_CHUNK));
                let more = c.g[S2][0] != c.g[V0][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(REPLAY_HIGHLIGHT_CHUNK));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_e740);
                    }
                    continue;
                }
                c.cycles += 2;
                break;
            }
        }
        // The tail: the data block's last 0xc bytes.
        c.cycles += 12;
        c.ldl(2, (c.g[S2][0] as u32).wrapping_add(0x7));
        c.ldr(2, c.g[S2][0] as u32);
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.sdr(2, c.g[V1][0] as u32);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_MixDeclick16 (0x3c7600)

/// The 16 ramp weights, `(16 - i) / 17`, as the game's float bits.
pub(crate) const SND_DECLICK_K: [u32; 16] = [
    0x3f70_f0f1, // 16/17
    0x3f61_e1e2, // 15/17
    0x3f52_d2d3, // 14/17
    0x3f43_c3c4, // 13/17
    0x3f34_b4b5, // 12/17
    0x3f25_a5a6, // 11/17
    0x3f16_9697, // 10/17
    0x3f07_8788, // 9/17
    0x3ef0_f0f1, // 8/17
    0x3ed2_d2d3, // 7/17
    0x3eb4_b4b5, // 6/17
    0x3e96_9697, // 5/17
    0x3e70_f0f1, // 4/17
    0x3e34_b4b5, // 3/17
    0x3df0_f0f1, // 2/17
    0x3d70_f0f1, // 1/17
];

/// `void SND_MixDeclick16(float* ramp, float* samples)` (0x3c7600): fade 16
/// samples in by adding `*ramp * (16 - i)/17` to sample `i`, then consume the
/// ramp. Frame 0x20 (f20..f22 spilled at +0, +8, +0x10).
///
/// The register schedule (which weight sits in which `f` register, and the
/// interleaving of the loads, the sixteen adds and the stores) is the
/// original's: a straightforward loop leaves a different `f` state.
pub fn snd_mix_declick16(c: &mut Cpu) {
    c.cycles += 138;
    let ramp = c.g[A0][0] as u32;
    let out = c.g[A1][0] as u32;
    c.lwc1(0, ramp);
    let frame = Frame::open(c, 0x20);
    c.g[AT][0] = SND_DECLICK_K[0] as u64;
    c.mtc1(1, 2); // f2 = 16/17
    c.lwc1(1, out);
    c.mul_s(0, 0, 2);
    c.g[AT][0] = SND_DECLICK_K[1] as u64;
    c.mtc1(1, 5); // f5 = 15/17
    frame.save_fpr(c, 22, 0x10);
    frame.save_fpr(c, 21, 0x8);
    c.add_s(1, 1, 0);
    frame.save_fpr(c, 20, 0);
    c.g[AT][0] = SND_DECLICK_K[2] as u64;
    c.mtc1(1, 8); // f8 = 14/17
    c.lwc1(2, out.wrapping_add(0x4));
    c.swc1(1, out);
    c.lwc1(3, out.wrapping_add(0x8));
    c.lwc1(0, ramp);
    c.g[AT][0] = SND_DECLICK_K[3] as u64;
    c.mtc1(1, 11); // f11 = 13/17
    c.mul_s(0, 0, 5);
    c.lwc1(4, out.wrapping_add(0xc));
    c.g[AT][0] = SND_DECLICK_K[4] as u64;
    c.mtc1(1, 12); // f12 = 12/17
    c.lwc1(1, out.wrapping_add(0x10));
    c.add_s(2, 2, 0);
    c.g[AT][0] = SND_DECLICK_K[5] as u64;
    c.mtc1(1, 13); // f13 = 11/17
    c.lwc1(5, out.wrapping_add(0x14));
    c.g[AT][0] = SND_DECLICK_K[6] as u64;
    c.mtc1(1, 14); // f14 = 10/17
    c.swc1(2, out.wrapping_add(0x4));
    c.lwc1(6, out.wrapping_add(0x18));
    c.lwc1(0, ramp);
    c.g[AT][0] = SND_DECLICK_K[7] as u64;
    c.mtc1(1, 15); // f15 = 9/17
    c.mul_s(0, 0, 8);
    c.lwc1(7, out.wrapping_add(0x1c));
    c.g[AT][0] = SND_DECLICK_K[8] as u64;
    c.mtc1(1, 16); // f16 = 8/17
    c.lwc1(2, out.wrapping_add(0x20));
    c.add_s(3, 3, 0);
    c.g[AT][0] = SND_DECLICK_K[9] as u64;
    c.mtc1(1, 17); // f17 = 7/17
    c.lwc1(8, out.wrapping_add(0x24));
    c.lwc1(9, out.wrapping_add(0x28));
    c.swc1(3, out.wrapping_add(0x8));
    c.lwc1(10, out.wrapping_add(0x2c));
    c.lwc1(0, ramp);
    c.g[AT][0] = SND_DECLICK_K[10] as u64;
    c.mtc1(1, 18); // f18 = 6/17
    c.mul_s(0, 0, 11);
    c.g[AT][0] = SND_DECLICK_K[11] as u64;
    c.mtc1(1, 19); // f19 = 5/17
    c.g[AT][0] = SND_DECLICK_K[12] as u64;
    c.mtc1(1, 20); // f20 = 4/17
    c.g[AT][0] = SND_DECLICK_K[13] as u64;
    c.mtc1(1, 21); // f21 = 3/17
    c.add_s(4, 4, 0);
    c.g[AT][0] = SND_DECLICK_K[14] as u64;
    c.mtc1(1, 22); // f22 = 2/17
    c.g[AT][0] = SND_DECLICK_K[15] as u64;
    c.mtc1(1, 11); // f11 = 1/17
    c.swc1(4, out.wrapping_add(0xc));
    c.lwc1(0, ramp);
    c.mul_s(0, 0, 12);
    c.add_s(1, 1, 0);
    c.swc1(1, out.wrapping_add(0x10));
    c.lwc1(0, ramp);
    c.mul_s(0, 0, 13);
    c.add_s(5, 5, 0);
    c.swc1(5, out.wrapping_add(0x14));
    c.lwc1(0, ramp);
    c.mul_s(0, 0, 14);
    c.add_s(6, 6, 0);
    c.swc1(6, out.wrapping_add(0x18));
    c.lwc1(0, ramp);
    c.mul_s(0, 0, 15);
    c.add_s(7, 7, 0);
    c.swc1(7, out.wrapping_add(0x1c));
    c.lwc1(0, ramp);
    c.mul_s(0, 0, 16);
    c.add_s(2, 2, 0);
    c.swc1(2, out.wrapping_add(0x20));
    c.lwc1(0, ramp);
    c.mul_s(0, 0, 17);
    c.add_s(8, 8, 0);
    c.swc1(8, out.wrapping_add(0x24));
    c.lwc1(0, ramp);
    c.mul_s(0, 0, 18);
    c.add_s(9, 9, 0);
    c.swc1(9, out.wrapping_add(0x28));
    c.lwc1(0, ramp);
    c.mul_s(0, 0, 19);
    c.add_s(10, 10, 0);
    c.swc1(10, out.wrapping_add(0x2c));
    c.lwc1(0, ramp);
    c.lwc1(1, out.wrapping_add(0x30));
    c.mul_s(0, 0, 20);
    c.lwc1(2, out.wrapping_add(0x34));
    c.lwc1(3, out.wrapping_add(0x38));
    c.lwc1(4, out.wrapping_add(0x3c));
    c.add_s(1, 1, 0);
    frame.restore_fpr(c, 20, 0);
    c.swc1(1, out.wrapping_add(0x30));
    c.lwc1(0, ramp);
    c.mul_s(0, 0, 21);
    frame.restore_fpr(c, 21, 0x8);
    c.add_s(2, 2, 0);
    c.swc1(2, out.wrapping_add(0x34));
    c.lwc1(0, ramp);
    c.mul_s(0, 0, 22);
    frame.restore_fpr(c, 22, 0x10);
    c.add_s(3, 3, 0);
    c.swc1(3, out.wrapping_add(0x38));
    c.lwc1(0, ramp);
    c.mul_s(0, 0, 11);
    c.add_s(4, 4, 0);
    c.swc1(4, out.wrapping_add(0x3c));
    c.sw(ramp, 0);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// RiderNaturalAirSub_Exit (0x12fb68)

/// `AnimController_GetLayerAnimEntry(ctrl, layer)` (0x311ae8).
pub(crate) const RIDER_AIR_EXIT_GET_ANIM: u32 = 0x0031_1ae8;
/// `AnimController_FadeOutLayer(ctrl, layer, duration)` (0x311e88).
pub(crate) const RIDER_AIR_EXIT_FADE_OUT: u32 = 0x0031_1e88;
/// `AnimController_GetLayerSequence(ctrl, layer)` (0x311b20).
pub(crate) const RIDER_AIR_EXIT_GET_SEQ: u32 = 0x0031_1b20;
/// The rider's animation controller pointer.
pub(crate) const RIDER_AIR_EXIT_CONTROLLER: u32 = 0x14;
/// `AnimController`'s render object.
pub(crate) const RIDER_AIR_EXIT_ANIMBASE: u32 = 0x784;
/// The sequence's blend weight.
pub(crate) const RIDER_AIR_EXIT_BLEND: u32 = 0x90;
/// The fade-out duration the original passes in f12.
pub(crate) const RIDER_AIR_EXIT_FADE_F12: u32 = 0x76e0;

/// `void RiderNaturalAirSub_Exit(self, int release)` (0x12fb68): when the
/// natural-air skill ends with a release and the controller's layer 0 is
/// playing the wind-up (3) or the air (0xd) animation, fade that layer out
/// and force its blend to 1. Frame 0x20 (ra +0, s0 +0x10).
pub fn rider_natural_air_sub_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let quiet = c.g[A1][0] == 0;
        c.g[S0][0] = c.g[A0][0]; // self
        if !quiet {
            c.cycles += 7;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_AIR_EXIT_CONTROLLER));
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x0012_fb8c);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(RIDER_AIR_EXIT_ANIMBASE));
            call_ptr(c, RIDER_AIR_EXIT_GET_ANIM, 0x0012_fb8c)?;
            c.g[V1][0] = 3;
            let windup = c.g[V0][0] == c.g[V1][0];
            c.g[A1][0] = 1;
            if !windup {
                // The original asks again (the call's result is not kept).
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_AIR_EXIT_CONTROLLER));
                c.g[RA][0] = sx(0x0012_fba4);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(RIDER_AIR_EXIT_ANIMBASE));
                call_ptr(c, RIDER_AIR_EXIT_GET_ANIM, 0x0012_fba4)?;
                c.g[V1][0] = 0xd;
                if c.g[V0][0] != c.g[V1][0] {
                    frame.restore(c, S0, 0x10);
                    c.cycles += 3;
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            }
            c.cycles += 12;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_AIR_EXIT_CONTROLLER));
            c.g[A1][0] = 0;
            c.lwc1(
                12,
                (c.g[GP][0] as u32).wrapping_sub(RIDER_AIR_EXIT_FADE_F12),
            );
            c.g[RA][0] = sx(0x0012_fbc4);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(RIDER_AIR_EXIT_ANIMBASE));
            call_ptr(c, RIDER_AIR_EXIT_FADE_OUT, 0x0012_fbc4)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_AIR_EXIT_CONTROLLER));
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x0012_fbd4);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(RIDER_AIR_EXIT_ANIMBASE));
            call_ptr(c, RIDER_AIR_EXIT_GET_SEQ, 0x0012_fbd4)?;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(RIDER_AIR_EXIT_BLEND));
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptMan_CollectSetSave (0x30c6c8)

/// The saved object's vtable and the stream/collector interface slot.
pub(crate) const WSCRIPT_SET_SAVE_VTABLE: u32 = 0x00;
pub(crate) const WSCRIPT_SET_SAVE_SLOT: u32 = 0x08;

/// `int WScriptMan_CollectSetSave(void* stream, cObj* obj)` (0x30c6c8):
/// `obj->v01(stream, 0x10c)`, i.e. the collector's save entry with the
/// `SaveSet` tag. Frame 0x10 (ra +0).
pub fn wscript_man_collect_set_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0]; // obj
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0]; // stream
        c.g[A2][0] = 0x10c;
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(WSCRIPT_SET_SAVE_VTABLE));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(WSCRIPT_SET_SAVE_SLOT));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(WSCRIPT_SET_SAVE_SLOT + 4));
        let save = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0030_c6f0);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, save, 0x0030_c6f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake, machine, vtable};
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Once;

    /// Gate words: a fake only acts when its own machine carries its magic
    /// (the slots are process-global, other tests' machines must fall
    /// through to whatever was installed before).
    const CTRL_SORT: u32 = DATA + 0x6_0000;
    const MAGIC_SORT: u32 = 0x13fa_d801;
    const CTRL_PARTICLE: u32 = DATA + 0x6_0010;
    const MAGIC_PARTICLE: u32 = 0x355c_b001;
    const CTRL_HIGHLIGHT: u32 = DATA + 0x6_0020;
    const MAGIC_HIGHLIGHT: u32 = 0x26e6_e001;
    const CTRL_AIR: u32 = DATA + 0x6_0030;
    const MAGIC_AIR: u32 = 0x12fb_6801;

    /// A call-logging fake for `addr`, gated on `ctrl == magic` and chaining
    /// to the override it replaced (the slots are shared across test modules).
    macro_rules! oc318_gated {
        ($name:ident, $prev:ident, $ctrl:expr, $magic:expr, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32($ctrl) == $magic {
                    let ra = c.g[RA][0] as u32;
                    return fake(c, $addr, ra);
                }
                let p = $prev.load(Ordering::Relaxed);
                if p == 0 || p == $name as *const () as usize {
                    return $gen(c);
                }
                // SAFETY: `p` was read from the same slot (a `fn(&mut Cpu)`).
                let f = unsafe { std::mem::transmute::<usize, fn(&mut Cpu)>(p) };
                f(c)
            }
        };
    }

    // ------------------------------------------------------- ObjPtrArray_Sort

    oc318_gated!(
        oc318_fake_merge_sort,
        OC318_PREV_MERGE_SORT,
        CTRL_SORT,
        MAGIC_SORT,
        0x0013_f8f8,
        ee::s01::f_0013f8f8__generated
    );

    fn obj_ptr_array_sort_install() {
        static ONCE: Once = Once::new();
        ONCE.call_once(|| unsafe {
            OC318_PREV_MERGE_SORT.store(ee::s01::OVERRIDE_0013F8F8 as usize, Ordering::Relaxed);
            ee::s01::OVERRIDE_0013F8F8 = oc318_fake_merge_sort;
        });
    }

    /// `ObjPtrArray_Sort` over empty, inverted, single and 2/3/16-element
    /// ranges (the merge sort is a logging fake).
    #[test]
    fn obj_ptr_array_sort_matches() {
        obj_ptr_array_sort_install();
        covered(
            ee::s01::f_0013fad8__generated,
            obj_ptr_array_sort,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0013_fad8, 1);
                c.pc = 0x0013_fad8;
                c.bus.write32(CTRL_SORT, MAGIC_SORT);
                let first = DATA + 0x100;
                let last = match seed % 8 {
                    0 => first,                 // empty
                    1 => first.wrapping_sub(8), // inverted
                    2 => first + 4,             // one element
                    3 => first + 8,             // two
                    4 => first + 0xc,           // three
                    5 => first + 0x10,          // four
                    _ => first + 0x40,          // sixteen
                };
                for i in 0..16 {
                    let v = if seed % 2 == 0 {
                        r.next_u32()
                    } else {
                        first + 4 * (15 - i)
                    };
                    c.bus.write32(first + 4 * i, v);
                }
                c.g[A0][0] = first as u64;
                c.g[A1][0] = last as u64;
                c.g[A2][0] = (DATA + 0x300) as u64;
            },
        );
    }

    // ------------------------------------------- ModObj_AddParticleModifier

    fn oc318_fall_through(c: &mut Cpu, prev: &AtomicUsize, gen: fn(&mut Cpu)) {
        let p = prev.load(Ordering::Relaxed);
        if p == 0 {
            return gen(c);
        }
        // SAFETY: `p` was read from the same slot (a `fn(&mut Cpu)`).
        let f = unsafe { std::mem::transmute::<usize, fn(&mut Cpu)>(p) };
        f(c)
    }

    static OC318_PREV_PARTICLE_ALLOC: AtomicUsize = AtomicUsize::new(0);
    static OC318_PREV_PARTICLE_CTOR: AtomicUsize = AtomicUsize::new(0);
    oc318_gated!(
        oc318_fake_particle_link,
        OC318_PREV_PARTICLE_LINK,
        CTRL_PARTICLE,
        MAGIC_PARTICLE,
        0x0035_5658,
        ee::s11::f_00355658__generated
    );

    /// `EA_MemAllocNamed`: logs the request, returns the fixture's block.
    fn oc318_fake_particle_alloc(c: &mut Cpu) {
        if c.bus.read32(CTRL_PARTICLE) != MAGIC_PARTICLE {
            return oc318_fall_through(
                c,
                &OC318_PREV_PARTICLE_ALLOC,
                ee::s10::f_00317d70__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let block = c.bus.read32(CTRL_PARTICLE + 4);
        fake(c, 0x0031_7d70, ra);
        if c.pc == ra {
            c.g[V0][0] = block as u64;
        }
    }

    /// `ParticleModifier_ctor`: logs its arguments and returns the modifier.
    fn oc318_fake_particle_ctor(c: &mut Cpu) {
        if c.bus.read32(CTRL_PARTICLE) != MAGIC_PARTICLE {
            return oc318_fall_through(
                c,
                &OC318_PREV_PARTICLE_CTOR,
                ee::s11::f_003458c0__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let this = c.g[A0][0];
        fake(c, 0x0034_58c0, ra);
        if c.pc == ra {
            c.g[V0][0] = this;
        }
    }

    fn mod_obj_add_particle_install() {
        // Put the engine-allocator fake in first, then chain on top of it.
        crate::game::animation::animsequencer::tests::asl_install_fakes();
        static ONCE: Once = Once::new();
        ONCE.call_once(|| unsafe {
            let hook = |slot: *mut fn(&mut Cpu), f: fn(&mut Cpu), prev: &AtomicUsize| {
                let cur = *slot;
                if cur as usize != f as usize {
                    prev.store(cur as usize, Ordering::Relaxed);
                    *slot = f;
                }
            };
            hook(
                addr_of_mut!(ee::s10::OVERRIDE_00317D70),
                oc318_fake_particle_alloc,
                &OC318_PREV_PARTICLE_ALLOC,
            );
            hook(
                addr_of_mut!(ee::s11::OVERRIDE_003458C0),
                oc318_fake_particle_ctor,
                &OC318_PREV_PARTICLE_CTOR,
            );
            hook(
                addr_of_mut!(ee::s11::OVERRIDE_00355658),
                oc318_fake_particle_link,
                &OC318_PREV_PARTICLE_LINK,
            );
        });
    }

    /// `ModObj_AddParticleModifier`: allocator, constructor and linker are
    /// logging fakes (some calls unwind).
    #[test]
    fn mod_obj_add_particle_modifier_matches() {
        mod_obj_add_particle_install();
        covered(
            ee::s11::f_00355cb0__generated,
            mod_obj_add_particle_modifier,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x0035_5cb0, 3);
                c.pc = 0x0035_5cb0;
                let ctrl = CTRL_PARTICLE;
                c.bus.write32(ctrl, MAGIC_PARTICLE);
                c.bus.write32(ctrl + 4, DATA + 0x8000);
                let this = DATA + 0x100;
                c.bus.write32(this + 0x18, 0x0035_5000 + seed);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
            },
        );
    }

    // --------------------------------- SpeechSystem_ApplyEventContext

    /// The ten-entry table: entry 0 is another id, entry 1 another key,
    /// entry 2 unused, entry 3 the match (with or without an object).
    fn speech_event_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x002b_1520 ^ (seed as u64) << 8);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.hooks.miss = crate::difftest::fakes::fake_call;
        c.hooks.jump_out = crate::difftest::fakes::fake_jump_out;
        c.hooks.syscall = crate::difftest::fakes::fake_syscall;
        // Unwind at the first voice call for one seed in four (those carry an
        // object on the matching entry, see below), so `covered` sees a
        // requested unwind inside the matched path.
        let unwind = if seed % 4 == 1 { 0 } else { u32::MAX };
        c.bus.write32(crate::difftest::fakes::LOG_UNWIND_AT, unwind);

        let this = DATA + 0x100;
        let id = (seed % 3) as u32;
        let key = ((seed / 3) % 2) as u32;
        let no_match = seed % 5 == 4;
        c.g[A0][0] = this as u64;
        c.g[A1][0] = id as u64;
        c.g[A2][0] = key as u64;
        c.pc = 0x002b_1520;

        // self->slots: an array of four slot records.
        c.bus.write32(this + SPEECH_EVENT_SLOTS, DATA + 0x40);
        for i in 0..4u32 {
            c.bus
                .write32(DATA + 0x40 + 4 * i, DATA + 0x2000 + 0x1000 * i);
        }
        // The ten entries: the body indexes entry `j` as `self + (j << 5)` and
        // then uses the absolute offsets above, so write each entry's field
        // block there (`ents[0]` is `self + 0x18`, the `used` word of j = 0).
        for j in 0..10u32 {
            let e = this + j * SPEECH_EVENT_ENT_SIZE;
            fill(c, &mut r, e + SPEECH_EVENT_ENT_USED, 8);
            let (used, e_id, e_key) = match j {
                0 => (1u32, id ^ 1, key),
                1 => (1, id, key ^ 1),
                2 => (0, id, key),
                3 => (1, id, if no_match { key ^ 1 } else { key }),
                _ => (0, r.next_u32() % 3, r.next_u32() % 2),
            };
            c.bus.write32(e + SPEECH_EVENT_ENT_USED, used);
            c.bus.write32(e + SPEECH_EVENT_ENT_ID, e_id);
            c.bus.write32(e + SPEECH_EVENT_ENT_KEY, e_key);
            c.bus.write32(e + SPEECH_EVENT_ENT_W, r.next_u32());
            c.bus.write32(e + SPEECH_EVENT_ENT_T, r.edge_f32());
            c.bus.write32(
                e + SPEECH_EVENT_ENT_K,
                if seed % 3 == 0 { 0 } else { 1 + seed % 7 },
            );
            c.bus.write32(e + SPEECH_EVENT_ENT_OBJ, 0);
        }
        // The matching entry's object, with a fake voice vtable.
        if !no_match && seed % 2 == 1 {
            let e = this + 3 * SPEECH_EVENT_ENT_SIZE;
            let obj = DATA + 0x4000;
            let vt = vtable(c, &mut r, &[SPEECH_EVENT_V05, SPEECH_EVENT_V08]);
            c.bus.write32(obj + SPEECH_EVENT_OBJ_VOICE, vt);
            c.bus.write32(obj + SPEECH_EVENT_OBJ_Z, r.next_u32());
            c.bus.write32(e + SPEECH_EVENT_ENT_OBJ, obj);
        }
    }

    /// `SpeechSystem_ApplyEventContext` over a ten-entry table: used/id/key
    /// mismatches, the end of the table, and a match whose object is absent
    /// or has a voice whose `v05`/`v08` are logging fakes.
    #[test]
    fn speech_system_apply_event_context_matches() {
        covered(
            ee::s08::f_002b1520__generated,
            speech_system_apply_event_context,
            200,
            &speech_event_setup,
        );
    }

    // --------------------------------------- Replay_HighlightSlotBegin

    oc318_gated!(
        oc318_fake_highlight_set_value,
        OC318_PREV_HIGHLIGHT,
        CTRL_HIGHLIGHT,
        MAGIC_HIGHLIGHT,
        0x0026_d730,
        ee::s07::f_0026d730__generated
    );

    fn replay_highlight_install() {
        static ONCE: Once = Once::new();
        ONCE.call_once(|| unsafe {
            OC318_PREV_HIGHLIGHT.store(ee::s07::OVERRIDE_0026D730 as usize, Ordering::Relaxed);
            ee::s07::OVERRIDE_0026D730 = oc318_fake_highlight_set_value;
        });
    }

    /// A slot object, a rider and a data block: aligned and unaligned
    /// blocks, so both copy loops run.
    fn replay_highlight_setup(c: &mut Cpu, seed: u32) {
        replay_highlight_install();
        let mut r = machine(c, seed, 0x0026_e6e0, 1);
        c.pc = 0x0026_e6e0;
        c.bus.write32(CTRL_HIGHLIGHT, MAGIC_HIGHLIGHT);
        // `self + 4` must be 8-aligned for the aligned path.
        let this = if seed % 2 == 0 {
            DATA + 0x404
        } else {
            DATA + 0x400
        };
        let rider = DATA + 0x800;
        let src = DATA + 0x1000 + 4 * (seed % 2);
        c.g[A0][0] = this as u64;
        c.g[A1][0] = rider as u64;
        c.g[A2][0] = src as u64;
        c.bus.write32(this + REPLAY_HIGHLIGHT_INDEX, seed % 8);
        fill(c, &mut r, rider, 8); // 0x20 bytes
        fill(c, &mut r, this + REPLAY_HIGHLIGHT_DATA, 0xac / 4);
        fill(c, &mut r, src, 0xac / 4);
    }

    /// `Replay_HighlightSlotBegin`: both copy loops (aligned and not) and the
    /// snapshot call (a logging fake).
    #[test]
    fn replay_highlight_slot_begin_matches() {
        covered(
            ee::s07::f_0026e6e0__generated,
            replay_highlight_slot_begin,
            200,
            &replay_highlight_setup,
        );
    }

    /// The copy loops' back-edge event checks, with events delivered and
    /// rescheduled mid-copy.
    #[test]
    fn replay_highlight_slot_begin_events_match() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s07::f_0026e6e0__generated,
            replay_highlight_slot_begin,
            60,
            &|c, seed| {
                replay_highlight_setup(c, seed);
                let mut r = Rng(0x0026_e601 ^ seed as u64);
                schedule(c, &mut r, seed, 0x0026_e6e0)
            },
        );
    }

    // ------------------------------------------------- SND_MixDeclick16

    /// `SND_MixDeclick16`: a ramp value and sixteen samples at both alignments
    /// (edge-case float bits so NaNs and infinities go through the same ops).
    #[test]
    fn snd_mix_declick16_matches() {
        check(
            ee::s14::f_003c7600__generated,
            snd_mix_declick16,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003c_7600, 0);
                let ramp = DATA + 0x100;
                let out = DATA + 0x200 + 4 * (seed % 2);
                c.bus.write32(ramp, r.edge_f32());
                for i in 0..16 {
                    c.bus.write32(out + 4 * i, r.edge_f32());
                }
                c.g[A0][0] = ramp as u64;
                c.g[A1][0] = out as u64;
            },
        );
    }

    // --------------------------------------- RiderNaturalAirSub_Exit

    static OC318_PREV_AIR_ANIM: AtomicUsize = AtomicUsize::new(0);
    static OC318_PREV_AIR_SEQ: AtomicUsize = AtomicUsize::new(0);
    oc318_gated!(
        oc318_fake_air_fade_out,
        OC318_PREV_AIR_FADE,
        CTRL_AIR,
        MAGIC_AIR,
        0x0031_1e88,
        ee::s10::f_00311e88__generated
    );

    /// `AnimController_GetLayerSequence`: logs the call and returns the buffer
    /// the fixture picks at `CTRL_AIR + 8` (the caller writes its blend).
    fn oc318_fake_air_get_seq(c: &mut Cpu) {
        if c.bus.read32(CTRL_AIR) != MAGIC_AIR {
            return oc318_fall_through(c, &OC318_PREV_AIR_SEQ, ee::s10::f_00311b20__generated);
        }
        let ra = c.g[RA][0] as u32;
        let seq = c.bus.read32(CTRL_AIR + 8);
        fake(c, 0x0031_1b20, ra);
        if c.pc == ra {
            c.g[V0][0] = seq as u64;
        }
    }

    /// `AnimController_GetLayerAnimEntry`: logs the call and returns the
    /// animation the fixture picks at `CTRL_AIR + 4` (3, 0xd or another).
    fn oc318_fake_air_get_anim(c: &mut Cpu) {
        if c.bus.read32(CTRL_AIR) != MAGIC_AIR {
            return oc318_fall_through(c, &OC318_PREV_AIR_ANIM, ee::s10::f_00311ae8__generated);
        }
        let ra = c.g[RA][0] as u32;
        let anim = c.bus.read32(CTRL_AIR + 4);
        fake(c, 0x0031_1ae8, ra);
        if c.pc == ra {
            c.g[V0][0] = anim as u64;
        }
    }

    fn rider_natural_air_install() {
        crate::game::misc::tests::bow_install_fakes();
        static ONCE: Once = Once::new();
        ONCE.call_once(|| unsafe {
            let hook = |slot: *mut fn(&mut Cpu), f: fn(&mut Cpu), prev: &AtomicUsize| {
                let cur = *slot;
                if cur as usize != f as usize {
                    prev.store(cur as usize, Ordering::Relaxed);
                    *slot = f;
                }
            };
            hook(
                addr_of_mut!(ee::s10::OVERRIDE_00311AE8),
                oc318_fake_air_get_anim,
                &OC318_PREV_AIR_ANIM,
            );
            hook(
                addr_of_mut!(ee::s10::OVERRIDE_00311E88),
                oc318_fake_air_fade_out,
                &OC318_PREV_AIR_FADE,
            );
            hook(
                addr_of_mut!(ee::s10::OVERRIDE_00311B20),
                oc318_fake_air_get_seq,
                &OC318_PREV_AIR_SEQ,
            );
        });
    }

    /// `RiderNaturalAirSub_Exit`: released or not, the layer entry at
    /// 0x3/0xd/other → fade and blend, or return.
    #[test]
    fn rider_natural_air_sub_exit_matches() {
        rider_natural_air_install();
        covered(
            ee::s00::f_0012fb68__generated,
            rider_natural_air_sub_exit,
            200,
            &|c, seed| {
                machine(c, seed, 0x0012_fb68, 3);
                c.pc = 0x0012_fb68;
                c.bus.write32(CTRL_AIR, MAGIC_AIR);
                c.bus
                    .write32(CTRL_AIR + 4, [3u32, 0xd, 7][(seed % 3) as usize]);
                c.bus.write32(CTRL_AIR + 8, DATA + 0x3000);
                let this = DATA + 0x100;
                let ctrl = DATA + 0x200;
                c.bus.write32(this + RIDER_AIR_EXIT_CONTROLLER, ctrl);
                c.bus.write32(ctrl + RIDER_AIR_EXIT_ANIMBASE, DATA + 0x400);
                let mut fr = Rng(seed as u64 + 7);
                c.bus
                    .write32(GAME_GP.wrapping_sub(RIDER_AIR_EXIT_FADE_F12), fr.edge_f32());
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    // --------------------------------------- WScriptMan_CollectSetSave

    /// `WScriptMan_CollectSetSave`: a fake vtable slot 0x8 method.
    #[test]
    fn wscript_man_collect_set_save_matches() {
        covered(
            ee::s10::f_0030c6c8__generated,
            wscript_man_collect_set_save,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_c6c8, 1);
                c.pc = 0x0030_c6c8;
                let obj = DATA + 0x100;
                let vt = vtable(c, &mut r, &[WSCRIPT_SET_SAVE_SLOT]);
                c.bus.write32(obj, vt);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = obj as u64;
            },
        );
    }
}
