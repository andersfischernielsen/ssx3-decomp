//! Staging module oc325 (wave wave213): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc325`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002a_10c0,
        ssx_audio_system_is_character_speaking,
        "SSXAudioSystem_IsCharacterSpeaking",
    ),
    (0x0012_e690, rider_wipeout_sub_exit, "RiderWipeoutSub_Exit"),
    (
        0x002b_8190,
        world_sound_get_trigger_ids,
        "cWorldSound_GetTriggerIds",
    ),
    (0x003d_7c38, spch_free_queue_slot, "SPCH_FreeQueueSlot"),
    (
        0x0029_ab08,
        ssx_audio_system_reset_boost_tick_pitch,
        "SSXAudioSystem_ResetBoostTickPitch",
    ),
    (
        0x003d_e800,
        ea_file_alloc_file_handle,
        "EAFile_AllocFileHandle",
    ),
    (
        0x0023_6fa8,
        game_state_finish_cinematic_save_state,
        "GameStateFinishCinematic_SaveState",
    ),
    (
        0x0023_6ec8,
        game_state_results_save_state,
        "GameStateResults_SaveState",
    ),
    (
        0x0023_6d60,
        game_state_pre_race_menu_save_state,
        "GameStatePreRaceMenu_SaveState",
    ),
    (
        0x0023_6b70,
        game_state_reset_ai_save_state,
        "GameStateResetAI_SaveState",
    ),
];

// ---------------------------------------------------------------------------
// 0x002a10c0 SSXAudioSystem_IsCharacterSpeaking

/// SSXAudioSystem: `+0x118` the stream array, `+0x5560` the speech system.
pub(crate) const OC325_AUDIO_STREAMS: u32 = 0x118;
pub(crate) const OC325_AUDIO_SPEECH: u32 = 0x5560;
/// Audio object: the bank monitor.
pub(crate) const OC325_AUDIO_BANK_MONITOR: u32 = 0x1d8;
/// Bank monitor instance: the id of the voice.
pub(crate) const OC325_INSTANCE_ID: u32 = 0x80;

pub(crate) const OC325_SPEECH_GET_STREAM_SLOT: Callee = Callee {
    addr: 0x002b_1220,
    f: ee::s08::SpeechSystem_GetInstanceStreamSlot,
};

pub(crate) const OC325_STREAM_ARRAY_GET_REQUEST: Callee = Callee {
    addr: 0x002a_b150,
    f: ee::s08::StreamArray_GetCurrentRequest,
};

pub(crate) const OC325_BANK_FIND_INSTANCE: Callee = Callee {
    addr: 0x002a_d550,
    f: ee::s08::BankMonitor_FindInstance,
};

/// `int SSXAudioSystem_IsCharacterSpeaking(SSXAudioSystem* self, int id)`
/// (0x2a10c0): 1 when the speech stream slot's current request has a voice
/// whose id is `id`. 0 when there is no current request, no voice, or the
/// voice belongs to another id.
pub fn ssx_audio_system_is_character_speaking(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(OC325_AUDIO_SPEECH));
        c.g[A1][0] = 0;
        OC325_SPEECH_GET_STREAM_SLOT.call(c, 0x002a_10e4)?;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(OC325_AUDIO_STREAMS)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        OC325_STREAM_ARRAY_GET_REQUEST.call(c, 0x002a_10f0)?;
        c.g[A1][0] = c.g[V0][0];
        let no_request = (c.g[A1][0] as i64) < 0;
        c.g[V0][0] = 0;
        if !no_request {
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(OC325_AUDIO_STREAMS)).lw(c);
            c.g[A2][0] = 0x3;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(OC325_AUDIO_BANK_MONITOR));
            OC325_BANK_FIND_INSTANCE.call(c, 0x002a_1110)?;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(OC325_INSTANCE_ID)).lw(c);
                c.g[V0][0] = c.g[V0][0] ^ c.g[S1][0];
                c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
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
// 0x0012e690 RiderWipeoutSub_Exit

/// Rider: `+0x80` its scripting object, `+0x6c0` the wipeout sub-state's
/// vtable slot (the sub-state object is at `+0x6c0`'s target).
pub(crate) const OC325_RIDER_SCRIPT: u32 = 0x80;
pub(crate) const OC325_RIDER_SUB: u32 = 0x6c0;
/// The sub-state's virtual "is the wipeout finishing?" slot (entry 8 of a
/// 0x10-byte table: delta at +0x40, method at +0x44).
pub(crate) const OC325_SUB_EXIT_SLOT: u32 = 0x40;

pub(crate) const OC325_AI_GET_FRAME_COUNTER: Callee = Callee {
    addr: 0x0012_98c8,
    f: ee::s00::AI_GetFrameCounter,
};

pub(crate) const OC325_RIDER_SET_TIMED_EVENT: Callee = Callee {
    addr: 0x0010_e028,
    f: ee::s00::Rider_SetTimedEvent,
};

/// `void RiderWipeoutSub_Exit(RiderWipeoutSub* m, int a1)` (0x12e690): every
/// other frame, ask the wipeout sub-state whether it is done; when it is,
/// set the rider's timed event 4 (the "wipeout over" boost timer), then
/// notify the speech system of the wipeout recovery.
pub fn rider_wipeout_sub_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        OC325_AI_GET_FRAME_COUNTER.call(c, 0x0012_e6a4)?;
        c.g[V0][0] &= 1;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 9;
            let rider = Addr(r32(c, S0).wrapping_add(OC325_RIDER_SCRIPT)).lw(c);
            c.g[A0][0] = rider;
            let vt = Addr((rider as u32).wrapping_add(OC325_RIDER_SUB)).lw(c);
            c.g[V1][0] = vt;
            c.g[A0][0] = sx((rider as u32).wrapping_add(OC325_RIDER_SUB));
            c.g[A1][0] = Addr((vt as u32).wrapping_add(OC325_SUB_EXIT_SLOT)).lh(c);
            c.g[V0][0] = Addr((vt as u32).wrapping_add(OC325_SUB_EXIT_SLOT + 4)).lw(c);
            let method = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, method, 0x0012_e6cc)?;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = Addr(r32(c, S0).wrapping_add(OC325_RIDER_SCRIPT)).lw(c);
                c.mtc1(0, 12);
                c.g[A1][0] = 0x4;
                OC325_RIDER_SET_TIMED_EVENT.call(c, 0x0012_e6e4)?;
            }
            c.cycles += 6;
            call_ptr(c, 0x0028_b180, 0x0012_e6ec)?;
            c.g[A1][0] = Addr(r32(c, S0).wrapping_add(OC325_RIDER_SCRIPT)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x002a_02d8, 0x0012_e6f8)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002b8190 cWorldSound_GetTriggerIds

/// A world sound object: `+0x8c` its trigger data, whose `+0x10` is the
/// trigger header `{word offset to the records, record count}`.
pub(crate) const OC325_SOUND_TDATA: u32 = 0x8c;
pub(crate) const OC325_TRIGGER_HDR: u32 = 0x10;
/// Record strides by type (0, 1, 2, 3).
pub(crate) const OC325_TRIGGER_STRIDE: [u32; 4] = [0x1c, 0x30, 0x30, 0x18];

/// `void cWorldSound_GetTriggerIds(cWorldSound* self, WorldSound* obj,
/// int* o0, int* o1, int* o2, int* o3)` (0x2b8190): walk the trigger records
/// (`{type, id}` with a type-dependent stride) and copy the id of each of
/// the first four into the four output words (0 when there are fewer).
pub fn world_sound_get_trigger_ids(c: &mut Cpu) {
    c.cycles += 9;
    let obj = c.g[A1][0] as u32;
    c.g[V0][0] = Addr(obj.wrapping_add(OC325_SOUND_TDATA)).lw(c);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(OC325_TRIGGER_HDR)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(8));
    c.g[T3][0] = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    let no_records = (c.g[T3][0] as i64) <= 0;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    if !no_records {
        // Record pointers in the frame's first four words.
        c.cycles += 7;
        c.g[T6][0] = 0x1;
        c.g[T5][0] = 0x2;
        c.g[T4][0] = 0x3;
        c.g[T2][0] = c.g[SP][0];
        c.g[A1][0] = c.g[T3][0];
        c.sw(c.g[T2][0] as u32, c.g[A0][0]);
        loop {
            c.cycles += 3;
            c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
            let is_one = c.g[V1][0] == c.g[T6][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
            if is_one {
                c.cycles += 1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(OC325_TRIGGER_STRIDE[1]));
                c.cycles += 1;
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    // type 0
                    c.cycles += 2;
                    if c.g[V1][0] == 0 {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(OC325_TRIGGER_STRIDE[0]));
                    } else {
                        c.cycles += 2;
                    }
                    c.cycles += 1;
                } else {
                    c.cycles += 2;
                    if c.g[V1][0] == c.g[T5][0] {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(OC325_TRIGGER_STRIDE[2]));
                        c.cycles += 1;
                    } else {
                        c.cycles += 2;
                        if c.g[V1][0] == c.g[T4][0] {
                            c.g[A0][0] =
                                sx((c.g[A0][0] as u32).wrapping_add(OC325_TRIGGER_STRIDE[3]));
                            c.cycles += 1;
                        } else {
                            c.cycles += 2;
                        }
                    }
                }
            }
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(4));
            c.cycles += 3;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            if c.g[A1][0] == 0 {
                break;
            }
            c.sw(c.g[T2][0] as u32, c.g[A0][0]);
            if event_due(c) {
                deliver_event_here(c, 0x002b_81d0);
            }
        }
    }
    // The four outputs, zeroed, then filled from the records' id words.
    c.cycles += 6;
    c.sw(c.g[A2][0] as u32, 0);
    c.g[A0][0] = 0;
    c.sw(c.g[A3][0] as u32, 0);
    c.sw(c.g[T0][0] as u32, 0);
    let no_records = (c.g[T3][0] as i64) <= 0;
    c.sw(c.g[T1][0] as u32, 0);
    if !no_records {
        c.cycles += 6;
        c.g[T6][0] = Addr(c.g[SP][0] as u32).lw(c);
        c.g[T5][0] = 0x1;
        c.g[T4][0] = 0x2;
        c.g[T2][0] = 0x3;
        c.g[A1][0] = c.g[SP][0];
        loop {
            c.cycles += 3;
            c.g[V1][0] = Addr(c.g[T6][0] as u32).lw(c);
            if c.g[V1][0] != c.g[T5][0] {
                c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
            let second = c.g[A0][0] == c.g[T5][0];
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(4)).lw(c);
            if second {
                c.cycles += 1;
                c.sw(c.g[A3][0] as u32, c.g[V1][0]);
                c.cycles += 1;
            } else {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < 2) as u64;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    if c.g[A0][0] == 0 {
                        c.sw(c.g[A2][0] as u32, c.g[V1][0]);
                    } else {
                        c.cycles += 2;
                    }
                    c.cycles += 1;
                } else {
                    c.cycles += 2;
                    if c.g[A0][0] == c.g[T4][0] {
                        c.sw(c.g[T0][0] as u32, c.g[V1][0]);
                        c.cycles += 1;
                    } else {
                        c.cycles += 2;
                        if c.g[A0][0] == c.g[T2][0] {
                            c.sw(c.g[T1][0] as u32, c.g[V1][0]);
                            c.cycles += 1;
                        } else {
                            c.cycles += 2;
                        }
                    }
                }
            }
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[T3][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002b_8250);
            }
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
    ret(c);
}

// ---------------------------------------------------------------------------
// 0x003d7c38 SPCH_FreeQueueSlot

/// The SPCH queue table (`D_0044FFF8`): 8 counters (`z`) at +0, 8 list heads
/// (`a`) at +0x20, 8 (`b`) at +0x40, then 16 queue entries of 0x60 bytes.
#[cfg(test)]
pub(crate) const OC325_SPCH_QUEUES: u32 = 0x0044_fff8;
pub(crate) const OC325_SPCH_QUEUE_ENTRY: u32 = 0x60;
pub(crate) const OC325_SPCH_HEADS_A: u32 = 0x20;
pub(crate) const OC325_SPCH_HEADS_B: u32 = 0x40;
/// Entry fields: the bound flag (`c`, byte), the queue index (`d`, byte),
/// the "in use" halfword (`s8`) and the next entry (`next`).
pub(crate) const OC325_SPCH_FLAG: u32 = 0x6a;
pub(crate) const OC325_SPCH_QUEUE_ID: u32 = 0x6b;
pub(crate) const OC325_SPCH_IN_USE: u32 = 0x6c;
pub(crate) const OC325_SPCH_NEXT: u32 = 0x6e;

/// `void SPCH_FreeQueueSlot(unsigned slot)` (0x3d7c38): release a speech
/// request slot: clear its bound flag and unlink it from its queue's lists
/// (heads and the chain of bound entries), then drop its queue's count by
/// one (clamped at 0).
pub fn spch_free_queue_slot(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = (c.g[A0][0] < 0x10) as u64;
    let out_of_range = c.g[V0][0] == 0;
    c.g[A2][0] = u64::MAX;
    if !out_of_range {
        c.cycles += 9;
        c.g[V1][0] = OC325_SPCH_QUEUE_ENTRY as u64;
        c.g[V0][0] = 0x0045_0000;
        c.mult(0, V1, A0, V1); // v1 = 0x60 * slot
        c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_sub(8)); // 0x44fff8
        c.g[T4][0] = c.g[V0][0]; // 0x450000
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T1][0] as u32));
        c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(OC325_SPCH_IN_USE)).lh(c);
        let in_use = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(OC325_SPCH_HEADS_B));
        if !in_use {
            // Unbind: clear the flag, drop it from the b[] and a[] heads.
            c.cycles += 7;
            c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(OC325_SPCH_QUEUE_ID)).lbu(c);
            Addr((c.g[A3][0] as u32).wrapping_add(OC325_SPCH_FLAG)).sb(c, 0);
            c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            if c.g[A0][0] == c.g[V0][0] {
                c.sw(c.g[V1][0] as u32, c.g[A2][0]);
            }
            c.cycles += 5;
            c.g[T2][0] = sx((c.g[T1][0] as u32).wrapping_add(OC325_SPCH_HEADS_A));
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T2][0] as u32));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            if c.g[A0][0] == c.g[V0][0] {
                c.sw(c.g[V1][0] as u32, c.g[A2][0]);
            }
            // Walk the chain of bound entries after this one.
            c.cycles += 4;
            c.g[A2][0] = Addr((c.g[A3][0] as u32).wrapping_add(OC325_SPCH_NEXT)).lh(c);
            c.g[V0][0] = u64::MAX;
            let no_next = c.g[A2][0] == c.g[V0][0];
            c.g[T0][0] = c.g[A1][0];
            let mut tail_charge = no_next;
            if !no_next {
                c.cycles += 3;
                c.g[V0][0] = (c.g[A2][0] < 0x10) as u64;
                let far = c.g[V0][0] == 0;
                c.g[V0][0] = OC325_SPCH_QUEUE_ENTRY as u64;
                if !far {
                    c.cycles += 7;
                    c.g[T5][0] = c.g[T1][0];
                    c.g[T3][0] = c.g[T2][0];
                    c.g[T2][0] = OC325_SPCH_QUEUE_ENTRY as u64;
                    c.g[T1][0] = c.g[T0][0];
                    c.g[A3][0] = u64::MAX;
                    c.mult(0, V0, A2, T2); // v0 = next * 0x60
                    loop {
                        c.cycles += 4;
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T5][0] as u32));
                        c.g[V0][0] =
                            Addr((c.g[A1][0] as u32).wrapping_add(OC325_SPCH_IN_USE)).lh(c);
                        if c.g[V0][0] == 0 {
                            c.g[A2][0] = u64::MAX;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] =
                                Addr((c.g[A1][0] as u32).wrapping_add(OC325_SPCH_FLAG)).lbu(c);
                            let unbound = c.g[V0][0] == 0;
                            c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T3][0] as u32));
                            if unbound {
                                c.cycles += 1;
                                c.g[A2][0] = u64::MAX;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                                if c.g[A2][0] == c.g[V0][0] {
                                    c.sw(c.g[V1][0] as u32, c.g[A3][0]);
                                }
                                c.cycles += 3;
                                c.g[A2][0] =
                                    Addr((c.g[A1][0] as u32).wrapping_add(OC325_SPCH_NEXT)).lh(c);
                                Addr((c.g[A1][0] as u32).wrapping_add(OC325_SPCH_FLAG)).sb(c, 0);
                            }
                        }
                        c.cycles += 2;
                        let finished = c.g[A2][0] == c.g[A3][0];
                        c.g[V0][0] = (c.g[A2][0] < 0x10) as u64;
                        if finished {
                            break;
                        }
                        c.cycles += 2;
                        let more = c.g[V0][0] != 0;
                        c.mult(0, V0, A2, T2);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_7cd0);
                        }
                    }
                    tail_charge = true;
                }
            }
            if tail_charge {
                c.cycles += 1;
                c.g[V0][0] = OC325_SPCH_QUEUE_ENTRY as u64;
            }
            // Drop the queue's count (clamped at 0).
            c.cycles += 6;
            c.g[A1][0] = sx((c.g[T4][0] as u32).wrapping_sub(8));
            c.mult(0, V0, A0, V0); // v0 = slot * 0x60
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(OC325_SPCH_IN_USE)).lh(c);
            let still_in_use = c.g[V1][0] != 0;
            c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            if !still_in_use {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            if (c.g[V0][0] as i64) < 0 {
                c.sw(c.g[V1][0] as u32, 0);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// 0x0029ab08 SSXAudioSystem_ResetBoostTickPitch

/// Rider: `+0x870` the boost-tick channel index, `+0x874`/`+0x87c` its two
/// enable words.
pub(crate) const OC325_RIDER_TICK_CHANNEL: u32 = 0x870;
pub(crate) const OC325_RIDER_TICK_A: u32 = 0x874;
pub(crate) const OC325_RIDER_TICK_B: u32 = 0x87c;
/// SSXAudioSystem: per-channel pitch settings (halfwords).
pub(crate) const OC325_CHANNEL_PITCH: u32 = 0x5fe8;

/// `void SSXAudioSystem_ResetBoostTickPitch(SSXAudioSystem* mgr, Rider*
/// rider)` (0x29ab08): when the rider's boost-tick sound is enabled, reset
/// its channel's pitch to 1.0 (0x1000 in 4.12 fixed point).
pub fn ssx_audio_system_reset_boost_tick_pitch(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, A1).wrapping_add(OC325_RIDER_TICK_A)).lw(c);
    let disabled = c.g[V0][0] == 0;
    c.g[V1][0] = 0;
    if !disabled {
        c.cycles += 2;
        c.g[V0][0] = Addr(r32(c, A1).wrapping_add(OC325_RIDER_TICK_B)).lw(c);
        c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
    }
    c.cycles += 2;
    let off = c.g[V1][0] == 0;
    c.g[V1][0] = 0x1000;
    if !off {
        c.cycles += 4;
        c.g[V0][0] = Addr(r32(c, A1).wrapping_add(OC325_RIDER_TICK_CHANNEL)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sh(
            (c.g[V0][0] as u32).wrapping_add(OC325_CHANNEL_PITCH),
            c.g[V1][0],
        );
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// 0x003de800 EAFile_AllocFileHandle

/// The file-handle manager (`D_00519AB0`): `+4` the handle count, `+0x14`
/// its mutex (0x519ac4), `+0x2c` the handle array. Handles are 0x110 bytes;
/// their first word is the in-use flag.
#[cfg(test)]
pub(crate) const OC325_FILE_MANAGER: u32 = 0x0051_9ab0;
pub(crate) const OC325_FILE_COUNT: u32 = 0x4;
pub(crate) const OC325_FILE_MUTEX: u32 = 0x14;
pub(crate) const OC325_FILE_ARRAY: u32 = 0x2c;
pub(crate) const OC325_FILE_HANDLE: u32 = 0x110;
/// The manager's own mutex (`D_00519AC4`).
#[cfg(test)]
pub(crate) const OC325_FILE_MANAGER_MUTEX: u32 = 0x0051_9ac4;

pub(crate) const OC325_MUTEX_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};

pub(crate) const OC325_MUTEX_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};

/// `EAFileHandle* EAFile_AllocFileHandle(void)` (0x3de800): take the file
/// manager's lock, claim the first free handle (marking it in use) and
/// return it; spin forever when every handle is taken.
pub fn ea_file_alloc_file_handle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[A0][0] = 0x0052_0000;
        frame.save(c, S1, 0x20);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x653c));
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        OC325_MUTEX_LOCK.call(c, 0x003d_e824)?;
        c.g[S2][0] = 0x0052_0000;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6550));
        loop {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(OC325_FILE_COUNT)).lw(c);
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let all_taken = c.g[V0][0] == 0;
            c.g[V0][0] = OC325_FILE_HANDLE as u64;
            if all_taken {
                break;
            }
            c.cycles += 6;
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(OC325_FILE_ARRAY)).lw(c);
            c.mult(0, V0, S1, V0); // v0 = 0x110 * i
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
            if c.g[V1][0] != 0 {
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                if event_due(c) {
                    deliver_event_here(c, 0x003d_e830);
                }
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                break;
            }
        }
        c.cycles += 5;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(OC325_FILE_MUTEX));
        OC325_MUTEX_UNLOCK.call(c, 0x003d_e868)?;
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(OC325_FILE_COUNT)).lw(c);
        let no_free = c.g[S1][0] != c.g[V0][0];
        c.g[V0][0] = OC325_FILE_HANDLE as u64;
        if !no_free {
            // Every handle is in use: the original spins here forever.
            loop {
                c.cycles += 7;
                if event_due(c) {
                    deliver_event_here(c, 0x003d_e878);
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6550));
        c.mult(0, V0, S1, V0); // v0 = 0x110 * i
        c.g[V1][0] = Addr(r32(c, A0).wrapping_add(OC325_FILE_ARRAY)).lw(c);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// GameState_*_SaveState wrappers (0x236fa8, 0x236ec8, 0x236d60, 0x236b70)

/// `GameState_SaveState` (0x233af0, through its override slot).
pub(crate) const OC325_GAME_STATE_SAVE_STATE: Callee = Callee {
    addr: 0x0023_3af0,
    f: ee::s06::GameState_SaveState,
};

/// A per-state `SaveState` wrapper: save the base game state, then return.
pub(crate) fn oc325_gamestate_save_wrapper(c: &mut Cpu, ret_addr: u32) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC325_GAME_STATE_SAVE_STATE),
        ret_addr,
    );
}

/// `void GameStateFinishCinematic_SaveState(GameState* self,
/// Serialiser* s)` (0x236fa8): the base game state's save.
pub fn game_state_finish_cinematic_save_state(c: &mut Cpu) {
    oc325_gamestate_save_wrapper(c, 0x0023_6fb8);
}

/// `void GameStateResults_SaveState(GameState* self, Serialiser* s)`
/// (0x236ec8): the base game state's save.
pub fn game_state_results_save_state(c: &mut Cpu) {
    oc325_gamestate_save_wrapper(c, 0x0023_6ed8);
}

/// `void GameStatePreRaceMenu_SaveState(GameState* self, Serialiser* s)`
/// (0x236d60): the base game state's save.
pub fn game_state_pre_race_menu_save_state(c: &mut Cpu) {
    oc325_gamestate_save_wrapper(c, 0x0023_6d70);
}

/// `void GameStateResetAI_SaveState(GameState* self, Serialiser* s)`
/// (0x236b70): the base game state's save.
pub fn game_state_reset_ai_save_state(c: &mut Cpu) {
    oc325_gamestate_save_wrapper(c, 0x0023_6b80);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{covered, machine, ptr, vtable, FAKE, LOG_RESULTS};
    use crate::difftest::{check, check_lohi, Rng, DATA, GAME_GP, RA as SENTINEL};

    // -- 0x002a10c0 SSXAudioSystem_IsCharacterSpeaking ------------------------

    const OC325_SP_SELF: u32 = DATA + 0x1_0000;
    const OC325_SP_SLOTTBL: u32 = DATA + 0x1_1000;
    const OC325_SP_SLOTENTRY: u32 = DATA + 0x1_2000;
    const OC325_SP_ARRAY: u32 = DATA + 0x1_3000;
    const OC325_SP_ELEMS: u32 = DATA + 0x1_4000;
    const OC325_SP_BUFFER: u32 = DATA + 0x1_5000;
    const OC325_SP_BANK: u32 = DATA + 0x2_0000;
    const OC325_SP_VOICES: u32 = DATA + 0x2_1000;
    const OC325_SP_STREAM: u32 = DATA + 0x3_0000;
    const OC325_SP_REQ: u32 = DATA + 0x3_1000;
    const OC325_SP_BUFOBJ: u32 = DATA + 0x3_2000;
    /// `SNDSTRM_GetStream`: the stream table (`0x50a0a0 + id*4`) and its
    /// count byte.
    const OC325_STREAM_TABLE: u32 = 0x0050_a0a0;
    const OC325_STREAM_COUNT: u32 = 0x0050_a92e;
    /// `SNDSTRM_GetBufferedBytes`: the request table.
    const OC325_STREAM_REQ_TABLE: u32 = 0x0051_5abc;

    /// Modes: no slot entry (-1), a null array element (-1), no matching
    /// voice, and a matching voice (id equal or not). The stream lookup runs
    /// the real library code with one controlled entry in the global stream
    /// table.
    fn oc325_speaking_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x002a_10c0, 0);
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        let w8 = |c: &mut Cpu, a: u32, v: u8| c.bus.write8(a, v);
        let mode = seed % 4;
        let id = 0x40 + r.next_u32() % 4;
        w(c, OC325_SP_SELF + OC325_AUDIO_STREAMS, OC325_SP_ARRAY);
        w(c, OC325_SP_SELF + OC325_AUDIO_SPEECH + 4, OC325_SP_SLOTTBL);
        if mode == 0 {
            w(c, OC325_SP_SLOTTBL, 0);
        } else {
            w(c, OC325_SP_SLOTTBL, OC325_SP_SLOTENTRY);
        }
        let slot = r.next_u32() % 4;
        w(c, OC325_SP_SLOTENTRY + 4, slot);
        w(c, OC325_SP_ARRAY + 8, OC325_SP_ELEMS);
        // The stream lookup: array element -> buffer -> stream id.
        let stream_id = r.next_u32() % 4;
        w(
            c,
            OC325_SP_ELEMS + 4 * slot,
            if mode == 1 { 0 } else { OC325_SP_BUFFER },
        );
        w(c, OC325_SP_BUFFER + 8, stream_id);
        w(c, OC325_STREAM_TABLE + 4 * stream_id, OC325_SP_STREAM);
        w8(c, OC325_STREAM_COUNT, 8);
        w(c, OC325_SP_STREAM + 0x124, 7);
        w(c, OC325_SP_STREAM + 0x11c, OC325_SP_REQ);
        let request = r.next_u32() % 4;
        w(c, OC325_SP_REQ + 0xc, request);
        c.bus.write16(OC325_SP_STREAM + 0x14, 1);
        let bb_id = r.next_u32() % 4;
        w(c, OC325_SP_STREAM + 8, bb_id);
        w(c, OC325_STREAM_REQ_TABLE + 4 * bb_id, OC325_SP_BUFOBJ);
        w(c, OC325_SP_BUFOBJ + 0x48, 5);
        // The bank: `lw(lw(self+0x118)) + 0x1d8`, voice slots at `+0x8ec`.
        w(c, OC325_SP_ARRAY, OC325_SP_BANK);
        w(
            c,
            OC325_SP_BANK + 0x8ec + OC325_AUDIO_BANK_MONITOR,
            OC325_SP_VOICES,
        );
        if mode >= 2 {
            let k = r.next_u32() % 4;
            let voice = OC325_SP_VOICES + k * 0xc0;
            w(c, voice, 3);
            w(c, voice + 4, request);
            w(c, voice + 0x9c, 0);
            w(
                c,
                voice + OC325_INSTANCE_ID,
                if mode == 3 && seed % 8 < 4 {
                    id
                } else {
                    id ^ 1
                },
            );
        }
        c.g[A0][0] = sx(OC325_SP_SELF);
        c.g[A1][0] = sx(id);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x002a_10c0;
    }

    /// The real speech-slot, stream-array and bank-monitor callees; both
    /// ids and every early exit.
    #[test]
    fn ssx_audio_system_is_character_speaking_matches() {
        check(
            ee::s08::f_002a10c0__generated,
            ssx_audio_system_is_character_speaking,
            240,
            &|c, seed| oc325_speaking_setup(c, seed),
        );
    }

    // -- 0x0012e690 RiderWipeoutSub_Exit -------------------------------------

    const OC325_EXIT_THIS: u32 = DATA + 0x5_0000;
    const OC325_EXIT_RIDER: u32 = DATA + 0x5_1000;
    const OC325_EXIT_VT: u32 = DATA + 0x5_2000;
    /// A real leaf that returns 0 (`RiderIface_*` stub at 0x1406c0).
    const OC325_RETURNS_ZERO: u32 = 0x0014_06c0;

    /// The frame counter's parity (odd: exit at once), and the wipeout
    /// sub-state's virtual "done" method: a real zero-returning leaf, or a
    /// fake (which returns a pointer, so the timed event and the speech
    /// recovery run; the audio system is null, so the recovery is silent and
    /// returns at once).
    fn oc325_wipeout_exit_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0012_e690, 0);
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        const APP: u32 = DATA + 0x4_0000;
        const GAME: u32 = DATA + 0x4_1000;
        const AI: u32 = DATA + 0x4_2000;
        w(c, GAME_GP + 0x848, APP);
        w(c, APP + 0x84, GAME);
        w(c, GAME + 0xc, AI);
        w(c, AI + 8, (seed % 2) as u32);
        w(c, OC325_EXIT_THIS + OC325_RIDER_SCRIPT, OC325_EXIT_RIDER);
        w(c, OC325_EXIT_RIDER + OC325_RIDER_SUB, OC325_EXIT_VT);
        let method = if seed % 3 == 0 {
            OC325_RETURNS_ZERO
        } else {
            FAKE
        };
        c.bus.write16(OC325_EXIT_VT + OC325_SUB_EXIT_SLOT, 0);
        w(c, OC325_EXIT_VT + OC325_SUB_EXIT_SLOT + 4, method);
        c.g[A0][0] = sx(OC325_EXIT_THIS);
        c.g[A1][0] = sx(r.next_u32());
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0012_e690;
    }

    /// Both parities, both method results (the fake returns a pointer, so
    /// the timed event and the recovery run), all three calls.
    #[test]
    fn rider_wipeout_sub_exit_matches() {
        check(
            ee::s00::f_0012e690__generated,
            rider_wipeout_sub_exit,
            200,
            &|c, seed| oc325_wipeout_exit_setup(c, seed),
        );
    }

    // -- 0x002b8190 cWorldSound_GetTriggerIds --------------------------------

    const OC325_TRIG_OBJ: u32 = DATA + 0x1_0000;
    const OC325_TRIG_DATA: u32 = DATA + 0x1_0100;
    const OC325_TRIG_HDR: u32 = DATA + 0x1_0200;
    const OC325_TRIG_RECS: u32 = DATA + 0x1_1000;
    const OC325_TRIG_OUT: u32 = DATA + 0x1_2000;

    /// A trigger header with 0..4 records whose types are 0..4 (4 = unknown,
    /// stride 0) and random ids, and four output words.
    fn oc325_trigger_ids_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x002b_8190, 0);
        let n = seed % 5;
        c.bus
            .write32(OC325_TRIG_OBJ + OC325_SOUND_TDATA, OC325_TRIG_DATA);
        c.bus
            .write32(OC325_TRIG_DATA + OC325_TRIGGER_HDR, OC325_TRIG_HDR);
        c.bus.write32(
            OC325_TRIG_HDR,
            (OC325_TRIG_RECS - (OC325_TRIG_HDR + 8)) >> 2,
        );
        c.bus.write32(OC325_TRIG_HDR + 4, n);
        let mut p = OC325_TRIG_RECS;
        for _ in 0..n {
            let ty = r.next_u32() % 5;
            c.bus.write32(p, ty);
            c.bus.write32(p + 4, r.next_u32());
            p += match ty {
                0 => OC325_TRIGGER_STRIDE[0],
                1 | 2 => OC325_TRIGGER_STRIDE[1],
                3 => OC325_TRIGGER_STRIDE[3],
                _ => 0,
            };
        }
        for i in 0..4u32 {
            c.bus.write32(OC325_TRIG_OUT + 4 * i, r.next_u32());
        }
        c.g[A1][0] = sx(OC325_TRIG_OBJ);
        c.g[A2][0] = sx(OC325_TRIG_OUT);
        c.g[A3][0] = sx(OC325_TRIG_OUT + 4);
        c.g[T0][0] = sx(OC325_TRIG_OUT + 8);
        c.g[T1][0] = sx(OC325_TRIG_OUT + 0xc);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x002b_8190;
    }

    driven!(oc325_gen_trigger_ids, ee::s08::f_002b8190__generated);
    driven!(oc325_hand_trigger_ids, world_sound_get_trigger_ids);

    /// Zero to four records of every type, and events at both loop back
    /// edges.
    #[test]
    fn world_sound_get_trigger_ids_matches() {
        check(
            ee::s08::f_002b8190__generated,
            world_sound_get_trigger_ids,
            220,
            &|c, seed| oc325_trigger_ids_setup(c, seed),
        );
        check_events(
            oc325_gen_trigger_ids,
            oc325_hand_trigger_ids,
            60,
            &|c, seed| {
                let mut r = Rng(0x2b81_90 ^ seed as u64);
                oc325_trigger_ids_setup(c, seed);
                let mode = schedule(c, &mut r, seed, 0x002b_8190);
                c.next_event = c.cycles + 2 + (r.next_u32() % 40) as u64;
                mode
            },
        );
    }

    // -- 0x003d7c38 SPCH_FreeQueueSlot ---------------------------------------

    /// A queue entry's address for `slot`.
    fn oc325_spch_entry(slot: u32) -> u32 {
        OC325_SPCH_QUEUES + OC325_SPCH_QUEUE_ENTRY * slot
    }

    /// A slot (0..15, or out of range) in a queue `d`: its in-use halfword,
    /// its queue's count, heads and next entry (a chain of live entries
    /// that always terminates).
    fn oc325_free_slot_setup(c: &mut Cpu, seed: u32, force_chain: bool) {
        let mut r = machine(c, seed, 0x003d_7c38, 0);
        let mode = if force_chain { 4 } else { seed % 6 };
        let slot = if !force_chain && seed % 7 == 0 {
            u64::MAX
        } else {
            (r.next_u32() % 16) as u64
        };
        c.g[A0][0] = slot;
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x003d_7c38;
        if slot >= 16 {
            return;
        }
        let s = slot as u32;
        let entry = oc325_spch_entry(s);
        let d = r.next_u32() % 8;
        c.bus.write8(entry + OC325_SPCH_QUEUE_ID, d as u8);
        c.bus.write8(entry + OC325_SPCH_FLAG, 0);
        c.bus.write32(
            OC325_SPCH_QUEUES + 4 * d,
            if seed % 3 == 0 { 0 } else { r.next_u32() % 5 },
        );
        c.bus.write32(
            OC325_SPCH_QUEUES + OC325_SPCH_HEADS_A + 4 * d,
            if seed % 4 == 0 { s } else { r.next_u32() % 16 },
        );
        c.bus.write32(
            OC325_SPCH_QUEUES + OC325_SPCH_HEADS_B + 4 * d,
            if seed % 5 == 0 { s } else { r.next_u32() % 16 },
        );
        c.bus.write16(
            entry + OC325_SPCH_IN_USE,
            if force_chain || seed % 2 == 0 { 0 } else { 1 },
        );
        let next: u16 = match mode {
            0 => 0xffff,
            1 => (16 + r.next_u32() % 16) as u16,
            _ => 0xffff,
        };
        c.bus.write16(entry + OC325_SPCH_NEXT, next);
        if mode >= 2 {
            // A chain of 3..5 distinct entries (never the slot itself), each
            // live and bound; it ends at -1, at an out-of-range index, at an
            // entry that is not in use, or at one with the flag cleared.
            let len = if force_chain {
                3 + (r.next_u32() % 3) as usize
            } else {
                1 + (r.next_u32() % 5) as usize
            };
            let base = (s.wrapping_add(1).wrapping_add(r.next_u32() % 16)) % 16;
            let mut nodes = Vec::with_capacity(len);
            for k in 0..len {
                nodes.push((base + k as u32) % 16);
            }
            for (k, &j) in nodes.iter().enumerate() {
                let je = oc325_spch_entry(j);
                let live = !(mode == 3 && k == 0);
                c.bus
                    .write16(je + OC325_SPCH_IN_USE, if live { 1 } else { 0 });
                c.bus.write8(
                    je + OC325_SPCH_FLAG,
                    if mode == 2 && k + 1 == len { 0 } else { 1 },
                );
                let nxt: u16 = match k + 1 {
                    k1 if k1 < len => nodes[k1] as u16,
                    _ if mode == 5 => (16 + r.next_u32() % 16) as u16,
                    _ => 0xffff,
                };
                c.bus.write16(je + OC325_SPCH_NEXT, nxt);
            }
            // Point a[] at the first node so the chain unlink runs too.
            if seed % 2 == 0 {
                c.bus
                    .write32(OC325_SPCH_QUEUES + OC325_SPCH_HEADS_A + 4 * d, nodes[0]);
            }
            c.bus.write16(entry + OC325_SPCH_NEXT, nodes[0] as u16);
        }
    }

    driven!(oc325_gen_free_slot, ee::s14::f_003d7c38__generated);
    driven!(oc325_hand_free_slot, spch_free_queue_slot);

    /// Every guard (out of range, in use), both head unlinks, and chains
    /// that end at -1, past 16, at a dead entry and at an unbound entry.
    #[test]
    fn spch_free_queue_slot_matches() {
        check_lohi(
            ee::s14::f_003d7c38__generated,
            spch_free_queue_slot,
            240,
            &|c, seed| oc325_free_slot_setup(c, seed, false),
        );
        check_events(oc325_gen_free_slot, oc325_hand_free_slot, 60, &|c, seed| {
            let mut r = Rng(0x3d7c_38 ^ seed as u64);
            oc325_free_slot_setup(c, seed, true);
            let mode = schedule(c, &mut r, seed, 0x003d_7c38);
            c.next_event = c.cycles + 2 + (r.next_u32() % 40) as u64;
            mode
        });
    }

    // -- 0x0029ab08 SSXAudioSystem_ResetBoostTickPitch -----------------------

    /// Both enable words (clear and set) and channel indices.
    fn oc325_boost_pitch_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0029_ab08, 0);
        let mgr = DATA + 0x1_0000;
        let rider = DATA + 0x1_1000;
        c.bus.write32(
            rider + OC325_RIDER_TICK_A,
            if seed % 3 == 0 { 0 } else { 1 },
        );
        c.bus.write32(
            rider + OC325_RIDER_TICK_B,
            if seed % 4 == 0 { 0 } else { 1 },
        );
        c.bus
            .write32(rider + OC325_RIDER_TICK_CHANNEL, r.next_u32() % 4);
        c.bus
            .write16(mgr + OC325_CHANNEL_PITCH, r.next_u32() as u16);
        c.g[A0][0] = sx(mgr);
        c.g[A1][0] = sx(rider);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0029_ab08;
    }

    /// Both enable flags; the pitch halfword is written only when both are
    /// set.
    #[test]
    fn ssx_audio_system_reset_boost_tick_pitch_matches() {
        check(
            ee::s08::f_0029ab08__generated,
            ssx_audio_system_reset_boost_tick_pitch,
            120,
            &|c, seed| oc325_boost_pitch_setup(c, seed),
        );
    }

    // -- 0x003de800 EAFile_AllocFileHandle -----------------------------------

    /// The file manager with 1..4 handles, at least one free (the original
    /// spins forever when every handle is taken), and its mutex owned by the
    /// thread id the fakes return (0x42), so both mutex calls take the fast
    /// path.
    fn oc325_file_alloc_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x003d_e800, 3);
        for i in 0..8u32 {
            c.bus.write32(LOG_RESULTS + 4 * i, 0x42);
        }
        c.bus.write32(OC325_FILE_MANAGER_MUTEX + 4, 0x42);
        c.bus.write32(OC325_FILE_MANAGER_MUTEX + 8, 1);
        c.bus.write32(OC325_FILE_MANAGER_MUTEX + 0xc, 32);
        c.bus.write32(0x0045_0de4, 0);
        let count = 1 + r.next_u32() % 4;
        let array = DATA + 0x1_0000;
        c.bus.write32(OC325_FILE_MANAGER + OC325_FILE_COUNT, count);
        c.bus.write32(OC325_FILE_MANAGER + OC325_FILE_ARRAY, array);
        let free = r.next_u32() % count;
        for i in 0..count {
            let handle = array + OC325_FILE_HANDLE * i;
            c.bus.write32(handle, if i == free { 0 } else { 1 });
            c.bus.write32(handle + 4, r.next_u32());
        }
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x003d_e800;
    }

    /// The mutex fast path on both calls and a free handle at a random
    /// index; some mutex calls unwind (propagated).
    #[test]
    fn ea_file_alloc_file_handle_matches() {
        covered(
            ee::s14::f_003de800__generated,
            ea_file_alloc_file_handle,
            200,
            &|c, seed| oc325_file_alloc_setup(c, seed),
        );
    }

    // -- GameState_*_SaveState wrappers --------------------------------------

    /// A serialiser whose `Save(size)` slot (vtable +8) is a fake; the
    /// wrapper's call to `GameState_SaveState` passes it through.
    fn oc325_save_state_setup(c: &mut Cpu, seed: u32, entry: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 1);
        c.pc = entry;
        let serialiser = DATA + 0x20_0000 + (r.next_u32() & 0xfff0);
        let vt = vtable(c, &mut r, &[0x8]);
        c.bus.write32(serialiser, vt);
        c.g[A0][0] = sx(ptr(&mut r));
        c.g[A1][0] = sx(serialiser);
        c.g[RA][0] = SENTINEL as u64;
    }

    #[test]
    fn game_state_finish_cinematic_save_state_matches() {
        covered(
            ee::s06::f_00236fa8__generated,
            game_state_finish_cinematic_save_state,
            100,
            &|c, seed| oc325_save_state_setup(c, seed, 0x0023_6fa8, 0x236f_a8),
        );
    }

    #[test]
    fn game_state_results_save_state_matches() {
        covered(
            ee::s06::f_00236ec8__generated,
            game_state_results_save_state,
            100,
            &|c, seed| oc325_save_state_setup(c, seed, 0x0023_6ec8, 0x236e_c8),
        );
    }

    #[test]
    fn game_state_pre_race_menu_save_state_matches() {
        covered(
            ee::s06::f_00236d60__generated,
            game_state_pre_race_menu_save_state,
            100,
            &|c, seed| oc325_save_state_setup(c, seed, 0x0023_6d60, 0x236d_60),
        );
    }

    #[test]
    fn game_state_reset_ai_save_state_matches() {
        covered(
            ee::s06::f_00236b70__generated,
            game_state_reset_ai_save_state,
            100,
            &|c, seed| oc325_save_state_setup(c, seed, 0x0023_6b70, 0x236b_70),
        );
    }
}
