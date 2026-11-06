//! Staging wave 209 (oc275): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_6158, snd_next_voice_group, "SND_NextVoiceGroup"),
    (0x003b_6ad8, sndstrm_release_chunk_callback, "SNDSTRM_ReleaseChunkCallback"),
    (0x002a_a108, stream_buffer_get_time_remaining, "StreamBuffer_GetTimeRemaining"),
    (0x003d_e670, ea_file_alloc_request, "EAFile_AllocRequest"),
    (0x0023_3af0, game_state_save_state, "GameState_SaveState"),
    (0x0029_5628, ssx_audio_system_track_sound, "SSXAudioSystem_TrackSound"),
    (0x0029_8d90, ssx_audio_system_start_boost_sound, "SSXAudioSystem_StartBoostSound"),
    (0x003b_6820, snd_play, "SND_Play"),
];

// ---------------------------------------------------------------------------
// SND_NextVoiceGroup (0x3c6158)
// ---------------------------------------------------------------------------

/// The voice-group table `D_0050A8E8`: `u8 count` at +0x4c, `u16 masks[]` at
/// +0x4e (one capability mask per voice group).
const SND_VOICE_GROUPS: u32 = 0x0050_a8e8;
const SND_VOICE_GROUP_COUNT: u32 = 0x4c;
const SND_VOICE_GROUP_MASKS: u32 = 0x4e;
/// The request's wanted-capability mask (`e->f18`) and its byte at +3
/// (`e->b3`: 1 allows groups with the 0x80 bit).
const SND_REQ_FLAGS: u32 = 0x18;
const SND_REQ_B3: u32 = 0x3;
/// Loop head (back-edge event check).
const SND_NEXT_VOICE_GROUP_LOOP: u32 = 0x003c_6188;

/// `int SND_NextVoiceGroup(int* idx, Request* e)` (0x3c6158): walks the
/// voice-group table from `*idx`, returning the first group's mask that
/// matches the request (the 0x71c type bits, then the 0xe0 bits; groups with
/// 0x80 only when `e->b3 == 1`) and advancing `*idx` past it; 0 when the
/// table is exhausted. Register-faithful: v0/v1/a0/a2/a3/t0..t3 as the
/// original leaves them.
pub fn snd_next_voice_group(c: &mut Cpu) {
    c.cycles += 8;
    c.g[T0][0] = c.g[A0][0];
    c.g[A0][0] = sx(SND_VOICE_GROUPS);
    let mut idx_ptr = Addr(c.g[T0][0] as u32);
    let mut v1 = idx_ptr.lw(c);
    let count = Addr(SND_VOICE_GROUPS + SND_VOICE_GROUP_COUNT).lbu(c);
    let mut a2: u64 = 0;
    if (v1 as i64) < (count as i64) {
        c.cycles += 4;
        c.g[T2][0] = sx(SND_VOICE_GROUPS);
        c.g[T3][0] = 1;
        c.g[T1][0] = sx(SND_VOICE_GROUPS + SND_VOICE_GROUP_MASKS);
        let mut v0 = sx((v1 as u32) << 1);
        let mut a0 = c.g[A0][0];
        let mut a3 = c.g[A3][0];
        let mut masks = c.g[T1][0] as u32;
        let mut table = c.g[T2][0] as u32;
        let mut one = c.g[T3][0];
        let mut req = c.g[A1][0] as u32;
        loop {
            // Type bits (0x71c).
            c.cycles += 7;
            let flags = Addr(req.wrapping_add(SND_REQ_FLAGS)).lhu(c);
            v0 = sx((v0 as u32).wrapping_add(masks));
            let group = Addr(v0 as u32).lhu(c);
            a3 = flags;
            v1 = flags & 0x71c;
            a0 = group & 0x71c;
            let mut accepted = true;
            if v1 == 0 {
                c.cycles += 2;
                a2 = a0;
                v1 = idx_ptr.lw(c);
            } else {
                c.cycles += 4;
                v0 = a3 & 0xffff;
                a2 = v0 & a0;
                if a2 != 0 {
                    v1 = idx_ptr.lw(c);
                } else {
                    c.cycles += 2;
                    a3 = idx_ptr.lw(c);
                    accepted = false;
                }
            }
            if accepted {
                // Second-stage bits (0xe0).
                c.cycles += 7;
                a0 = a3 & 0xe0;
                v0 = sx(((v1 as u32) << 1).wrapping_add(masks));
                a3 = v1;
                v1 = Addr(v0 as u32).lhu(c) & 0xe0;
                if a0 == 0 {
                    c.cycles += 1;
                    a2 |= v1;
                } else {
                    c.cycles += 4;
                    v0 = Addr(req.wrapping_add(SND_REQ_FLAGS)).lhu(c) & v1;
                    a2 |= v0;
                    if v0 == 0 {
                        c.cycles += 2;
                        a2 = 0;
                        accepted = false;
                    }
                }
            }
            if accepted {
                // Groups with 0x80 only for b3 == 1.
                c.cycles += 3;
                let b3 = Addr(req.wrapping_add(SND_REQ_B3)).lb(c);
                v0 = a2 & 0x80;
                if b3 != one {
                    c.cycles += 2;
                    if v0 != 0 {
                        a2 = 0;
                        accepted = false;
                    }
                }
            }
            if accepted {
                c.cycles += 2;
                v0 = sx((a3 as u32).wrapping_add(1));
                if a2 != 0 {
                    // Found: *idx = idx + 1, return the mask.
                    c.cycles += 2;
                    idx_ptr.sw(c, v0);
                    break;
                }
            }
            // Next group.
            c.cycles += 6;
            v1 = sx((a3 as u32).wrapping_add(1));
            idx_ptr.sw(c, v1);
            v0 = Addr(table.wrapping_add(SND_VOICE_GROUP_COUNT)).lbu(c);
            let more = (v1 as i64) < (v0 as i64);
            v0 = sx((v1 as u32) << 1);
            if !more {
                break;
            }
            if event_due(c) {
                c.g[V0][0] = v0;
                c.g[V1][0] = v1;
                c.g[A0][0] = a0;
                c.g[A2][0] = a2;
                c.g[A3][0] = a3;
                deliver_event_here(c, SND_NEXT_VOICE_GROUP_LOOP);
                v0 = c.g[V0][0];
                v1 = c.g[V1][0];
                a0 = c.g[A0][0];
                a2 = c.g[A2][0];
                a3 = c.g[A3][0];
                idx_ptr = Addr(c.g[T0][0] as u32);
                masks = c.g[T1][0] as u32;
                table = c.g[T2][0] as u32;
                one = c.g[T3][0];
                req = c.g[A1][0] as u32;
            }
        }
        c.g[A0][0] = a0;
        c.g[A3][0] = a3;
    }
    c.cycles += 2;
    c.g[V1][0] = v1;
    c.g[A2][0] = a2;
    c.g[V0][0] = a2;
    ret(c);
}

// ---------------------------------------------------------------------------
// SNDSTRM_ReleaseChunkCallback (0x3b6ad8)
// ---------------------------------------------------------------------------

/// The sound streams' stream-file handles, indexed by stream number
/// (`D_0050A0A0`: pointers to `StreamFile*`).
const SNDSTRM_FILES: u32 = 0x0050_a0a0;
/// `StreamFile_ReleaseChunk(file, chunk)` (cross-shard, through the table).
const SNDSTRM_RELEASE_CHUNK: u32 = 0x003e_13e8;

/// `void SNDSTRM_ReleaseChunkCallback(void* data)` (0x3b6ad8): the chunk
/// header sits in the word before the data; its first byte is the stream
/// number. Releases the chunk to that stream's file
/// (`StreamFile_ReleaseChunk(*files[*chunk], chunk)`).
pub fn sndstrm_release_chunk_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let chunk = Addr(r32(c, A0).wrapping_sub(4)).lw(c);
        c.g[A1][0] = chunk;
        let stream = Addr(chunk as u32).lbu(c);
        let slot = sx(((stream as u32) << 2).wrapping_add(SNDSTRM_FILES));
        let handle = Addr(slot as u32).lw(c);
        // Registers at the call.
        c.g[V0][0] = slot;
        c.g[V1][0] = handle;
        c.g[A0][0] = Addr(handle as u32).lw(c);
        call_ptr(c, SNDSTRM_RELEASE_CHUNK, 0x003b_6b04)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_GetTimeRemaining (0x2aa108)
// ---------------------------------------------------------------------------

/// `StreamBuffer_GetRequestStatus(self, status, scratch)`.
const STREAM_BUFFER_GET_REQUEST_STATUS: Callee = Callee {
    addr: 0x002a_a068,
    f: ee::s08::StreamBuffer_GetRequestStatus,
};
/// Request status record (on the stack): state (2 = playing) and the
/// remaining time at +8.
const SB_REQ_STATE: u32 = 0x0;
const SB_REQ_REMAINING: u32 = 0x8;
const SB_REQ_PLAYING: u64 = 2;

/// `float StreamBuffer_GetTimeRemaining(StreamBuffer* self)` (0x2aa108):
/// asks for the current request's status (into a stack record); while it
/// is playing (state 2) returns its remaining time as a float, else 0.
pub fn stream_buffer_get_time_remaining(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        STREAM_BUFFER_GET_REQUEST_STATUS.call(c, 0x002a_a11c)?;
        let state = Addr(frame.at(c, SB_REQ_STATE)).lw(c);
        c.g[V1][0] = state;
        c.g[V0][0] = SB_REQ_PLAYING;
        frame.restore_ra(c, 0x10);
        if state == SB_REQ_PLAYING {
            c.cycles += 3;
            let remaining = Addr(frame.at(c, SB_REQ_REMAINING)).lw(c);
            c.g[V0][0] = remaining;
            abi::set_fpr(c, 0, FpuF32::from_i32(remaining as i32));
        } else {
            c.cycles += 3;
            abi::set_fpr(c, 0, FpuF32::ZERO);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EAFile_AllocRequest (0x3de670)
// ---------------------------------------------------------------------------

/// The EAFile request pool (`0x519ab0`): slot count at +0, its mutex at
/// +0x14, the slot array (0x30 bytes each) at +0x28.
const EAFILE_POOL: u32 = 0x0051_9ab0;
const EAFILE_POOL_SLOTS: u32 = 0x28;
const EAFILE_POOL_MUTEX: u32 = 0x0051_9ac4;
const EAFILE_REQ_SIZE: u32 = 0x30;
/// Slot word 0: bits 0-19 serial, 20-23 state (0 = free, 1 = allocated);
/// byte 3 = the slot's index.
const EAFILE_REQ_INDEX: u32 = 0x3;
/// The next request serial (wraps from 0xfffff to 1).
const EAFILE_REQ_SERIAL: u32 = 0x0045_0c0c;
/// Loop heads: the slot scan, and the spin when the pool is exhausted.
const EAFILE_ALLOC_SCAN_LOOP: u32 = 0x003d_e6d0;
const EAFILE_ALLOC_EXHAUSTED_LOOP: u32 = 0x003d_e768;

const EAFILE_MUTEX_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};
const EAFILE_MUTEX_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};

/// `EAFileRequest* EAFile_AllocRequest(void)` (0x3de670): under the pool
/// mutex, takes the first free request slot, marks it allocated, stores its
/// index and the next serial (1..0xfffff) in it; returns `&slots[index]`.
/// With every slot busy the original spins forever (pc 0x3de768); so does
/// this port.
pub fn ea_file_alloc_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = 0x0052_0000;
        c.g[S2][0] = sx(EAFILE_POOL_MUTEX);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[A0][0] = c.g[S2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        EAFILE_MUTEX_LOCK.call(c, 0x003d_e69c)?;
        let count = Addr(r32(c, S2).wrapping_sub(0x14)).lw(c);
        c.g[V0][0] = count;
        c.g[S3][0] = 0x0052_0000;
        if (count as i64) > 0 {
            c.cycles += 10;
            c.g[T0][0] = 0xffff_ffff_ff0f_ffff; // clears the state bits
            c.g[A2][0] = 0xf_ffff; // serial mask
            c.g[A0][0] = sx(r32(c, S2).wrapping_sub(0x14));
            c.g[T1][0] = c.g[S3][0];
            c.g[T2][0] = 0x10_0000; // state 1
            c.g[T3][0] = 0xffff_ffff_fff0_0000; // clears the serial
            c.g[T4][0] = 1;
            c.g[A3][0] = 0;
            loop {
                c.cycles += 7;
                let pool = r32(c, A0);
                let slots = Addr(pool.wrapping_add(EAFILE_POOL_SLOTS)).lw(c);
                let slot = sx(r32(c, A3).wrapping_add(slots as u32));
                c.g[A1][0] = slot;
                let word = Addr(slot as u32).lw(c);
                c.g[V1][0] = word;
                let state = sx((word as u32) >> 20) & 0xf;
                c.g[V0][0] = Addr(r32(c, T1).wrapping_sub(0x6550)).lw(c);
                if state == 0 {
                    // Free: claim it.
                    c.cycles += 19;
                    let claimed = (word & c.g[T0][0]) | c.g[T2][0];
                    c.g[V1][0] = claimed;
                    c.g[T0][0] = 0x0045_0000;
                    Addr(slot as u32).sw(c, claimed);
                    let slots = Addr(pool.wrapping_add(EAFILE_POOL_SLOTS)).lw(c);
                    let at = sx(r32(c, A3).wrapping_add(slots as u32));
                    let index = c.g[S1][0];
                    Addr((at as u32).wrapping_add(EAFILE_REQ_INDEX)).sb(c, index);
                    let slots = Addr(pool.wrapping_add(EAFILE_POOL_SLOTS)).lw(c);
                    let serial = Addr(EAFILE_REQ_SERIAL).lw(c);
                    let req = sx(r32(c, A3).wrapping_add(slots as u32));
                    c.g[A0][0] = req;
                    let head = Addr(req as u32).lw(c);
                    let serial_bits = serial & c.g[A2][0];
                    let next = sx((serial as u32).wrapping_add(1));
                    let head = (head & c.g[T3][0]) | serial_bits;
                    let wrap = (c.g[A2][0] as i64) < (next as i64);
                    c.g[A1][0] = serial_bits;
                    c.g[V1][0] = next;
                    c.g[A2][0] = wrap as u64;
                    c.g[V0][0] = head;
                    Addr(req as u32).sw(c, head);
                    Addr(EAFILE_REQ_SERIAL).sw(c, next);
                    if wrap {
                        c.cycles += 2;
                        let one = c.g[T4][0];
                        Addr(EAFILE_REQ_SERIAL).sw(c, one);
                    }
                    break;
                }
                // Busy: next slot.
                c.cycles += 4;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                let more = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(EAFILE_REQ_SIZE));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, EAFILE_ALLOC_SCAN_LOOP);
                }
            }
        }
        c.cycles += 6;
        c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x653c));
        c.g[A0][0] = c.g[S0][0];
        EAFILE_MUTEX_UNLOCK.call(c, 0x003d_e75c)?;
        let count = Addr(r32(c, S0).wrapping_sub(0x14)).lw(c);
        c.g[V0][0] = EAFILE_REQ_SIZE as u64;
        if c.g[S1][0] == count {
            // No free slot: the original spins here forever.
            loop {
                c.cycles += 7;
                if event_due(c) {
                    deliver_event_here(c, EAFILE_ALLOC_EXHAUSTED_LOOP);
                }
            }
        }
        c.cycles += 11;
        c.g[A0][0] = sx(r32(c, S3).wrapping_sub(0x6550));
        let (index, stride) = (r32(c, S1), r32(c, V0));
        let offset = abi::mult(c, index, stride);
        c.g[V0][0] = sx(offset);
        let slots = Addr(r32(c, A0).wrapping_add(EAFILE_POOL_SLOTS)).lw(c);
        c.g[V1][0] = slots;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        c.g[V0][0] = sx((slots as u32).wrapping_add(offset));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// GameState_SaveState (0x233af0)
// ---------------------------------------------------------------------------

/// The serialiser's virtual "save state" slot (`vt[1]`: delta at +8,
/// method at +0xc), and the size argument it gets.
const GAME_STATE_SAVE_SLOT: u32 = 0x8;
const GAME_STATE_SAVE_SIZE: u64 = 4;

/// `void GameState_SaveState(GameState* self, Serialiser* s)` (0x233af0):
/// the default save of a game state: `s->vt[1](s, self, 4)` (writes the
/// state's first word through the serialiser).
pub fn game_state_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        let serialiser = c.g[A1][0];
        c.g[A3][0] = serialiser;
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = GAME_STATE_SAVE_SIZE;
        let vt = Addr(serialiser as u32).lw(c);
        c.g[V1][0] = vt;
        let slot = crate::call::VSlot::read(c, vt, GAME_STATE_SAVE_SLOT);
        c.g[V0][0] = slot.func;
        c.g[A0][0] = slot.this(serialiser);
        call_ptr(c, slot.func as u32, 0x0023_3b18)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_TrackSound (0x295628)
// ---------------------------------------------------------------------------

/// The tracked-sound ring of `int`s (5 per tracked sound): capacity, data,
/// head (write index) and tail (read index).
const TS_RING: u32 = 0x597c;
const TS_RING_DATA: u32 = 0x4;
const TS_RING_HEAD: u32 = 0x8;
const TS_RING_TAIL: u32 = 0xc;
/// The audio system's manager pointer; `*mgr + 0x1d8` is the bank monitor.
const TS_MGR: u32 = 0x118;
const TS_BANK_MONITOR: u32 = 0x1d8;

const TS_STOP_HANDLE: Callee = Callee {
    addr: 0x002a_d5f0,
    f: ee::s08::BankMonitor_StopHandle,
};

/// Drop the ring's oldest entry (pops 3..5 of a full ring): `extra` cycles
/// first, then tail = (tail + 1) wrapped at the capacity, unless empty.
fn ts_ring_drop(c: &mut Cpu, ring: u32, extra: u64) {
    c.cycles += extra;
    let head = Addr(ring + TS_RING_HEAD).lw(c);
    let tail = Addr(ring + TS_RING_TAIL).lw(c);
    c.g[A0][0] = tail;
    let next = sx((tail as u32).wrapping_add(1));
    c.g[V0][0] = next;
    if head != tail {
        c.cycles += 5;
        let cap = Addr(ring).lw(c);
        c.g[V1][0] = cap;
        Addr(ring + TS_RING_TAIL).sw(c, next);
        let fits = (next as i64) < (cap as i64);
        c.g[V0][0] = fits as u64;
        if !fits {
            Addr(ring + TS_RING_TAIL).sw(c, 0);
        }
    }
}

/// One push of `value` (`(head + 1) % cap` already in HI): unless full,
/// `data[head] = value`, head advances (wrapping). `first` is the push
/// right after the full check (head in v1, 6 setup cycles); the last push
/// does not reload v0.
fn ts_ring_push(c: &mut Cpu, ring: u32, value: u64, first: bool, last: bool) {
    // (head + 1) % cap.
    if first {
        c.cycles += 6;
        c.g[A1][0] = sx(ring);
        let cap = Addr(ring).lw(c);
        c.g[V0][0] = cap;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
        let n = r32(c, V1);
        abi::div(c, n, cap as u32);
    } else {
        c.cycles += 5;
        let cap = Addr(ring).lw(c);
        c.g[V1][0] = cap;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        let n = r32(c, V0);
        abi::div(c, n, cap as u32);
    }
    c.cycles += 4;
    let tail = Addr(ring + TS_RING_TAIL).lw(c);
    if first {
        c.g[V1][0] = tail;
    } else {
        c.g[V0][0] = tail;
    }
    let next_head = c.hi[0];
    c.g[A0][0] = next_head;
    if next_head != tail {
        c.cycles += 11;
        let ring_base = r32(c, A1);
        let head = Addr(ring_base + TS_RING_HEAD).lw(c);
        let data = Addr(ring_base + TS_RING_DATA).lw(c);
        let at = sx(((head as u32) << 2).wrapping_add(data as u32));
        Addr(at as u32).sw(c, value);
        let head = Addr(ring + TS_RING_HEAD).lw(c);
        let cap = Addr(ring).lw(c);
        let head = sx((head as u32).wrapping_add(1));
        c.g[V1][0] = head;
        let fits = (head as i64) < (cap as i64);
        c.g[V0][0] = fits as u64;
        Addr(ring + TS_RING_HEAD).sw(c, head);
        if !fits {
            c.cycles += 1;
            Addr(ring + TS_RING_HEAD).sw(c, 0);
        }
        c.cycles += 1;
    }
    if !last {
        c.g[V0][0] = Addr(ring + TS_RING_HEAD).lw(c);
    }
}

/// `void SSXAudioSystem_TrackSound(SSXAudioSystem* self, int evt, int h,
/// int t, int a, int idx)` (0x295628): remembers a playing sound (five
/// ints) in the tracked-sound ring. When the ring is full the oldest entry
/// is dropped first and its sound handle (second int) stopped through the
/// bank monitor (`BankMonitor_StopHandle(bm, handle, 1, 0.0)`).
pub fn ssx_audio_system_track_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S5][0] = c.g[T1][0];
        let this = r32(c, S0);
        let ring = this.wrapping_add(TS_RING);
        let head = Addr(ring + TS_RING_HEAD).lw(c);
        let cap = Addr(ring).lw(c);
        c.g[A2][0] = head;
        c.g[A1][0] = cap;
        c.g[V0][0] = sx((head as u32).wrapping_add(1));
        let n = r32(c, V0);
        abi::div(c, n, cap as u32);

        // Full?
        c.cycles += 4;
        let tail = Addr(ring + TS_RING_TAIL).lw(c);
        c.g[A0][0] = tail;
        c.g[V1][0] = c.hi[0];
        if c.hi[0] == tail {
            // Pop 1 (the event).
            c.cycles += 2;
            let next = sx((tail as u32).wrapping_add(1));
            c.g[V0][0] = next;
            if head != tail {
                c.cycles += 3;
                let fits = (next as i64) < (cap as i64);
                c.g[V1][0] = fits as u64;
                Addr(ring + TS_RING_TAIL).sw(c, next);
                if !fits {
                    c.cycles += 1;
                    Addr(ring + TS_RING_TAIL).sw(c, 0);
                }
            }
            // Pop 2: the sound handle (-1 when empty).
            c.cycles += 7;
            let mgr = Addr(this.wrapping_add(TS_MGR)).lw(c);
            c.g[V0][0] = mgr;
            c.g[A2][0] = sx(ring);
            let head = Addr(ring + TS_RING_HEAD).lw(c);
            c.g[A0][0] = head;
            let bank_owner = Addr(mgr as u32).lw(c);
            c.g[V1][0] = bank_owner;
            let tail = Addr(ring + TS_RING_TAIL).lw(c);
            c.g[A1][0] = tail;
            let monitor = sx((bank_owner as u32).wrapping_add(TS_BANK_MONITOR));
            c.g[A3][0] = monitor;
            if head != tail {
                c.cycles += 10;
                let tail = Addr(ring + TS_RING_TAIL).lw(c);
                let next = sx((tail as u32).wrapping_add(1));
                c.g[A1][0] = next;
                let data = Addr(ring + TS_RING_DATA).lw(c);
                c.g[A0][0] = data;
                let cap = Addr(ring).lw(c);
                let at = sx(((tail as u32) << 2).wrapping_add(data as u32));
                let handle = Addr(at as u32).lw(c);
                c.g[V0][0] = handle;
                let fits = (next as i64) < (cap as i64);
                c.g[V1][0] = fits as u64;
                Addr(ring + TS_RING_TAIL).sw(c, next);
                if !fits {
                    c.cycles += 1;
                    Addr(ring + TS_RING_TAIL).sw(c, 0);
                }
                c.cycles += 1;
                c.g[A1][0] = handle;
            } else {
                c.cycles += 2;
                c.g[A1][0] = u64::MAX;
            }
            // BankMonitor_StopHandle(monitor, handle, 1, 0.0f).
            c.cycles += 8;
            abi::set_fpr(c, 12, FpuF32::ZERO);
            c.g[A0][0] = monitor;
            c.g[A2][0] = 1;
            TS_STOP_HANDLE.call(c, 0x0029_5704)?;
            // Pops 3..5.
            ts_ring_drop(c, ring, 0);
            ts_ring_drop(c, ring, 4);
            ts_ring_drop(c, ring, 4);
            c.cycles += 1;
        }
        c.g[V1][0] = Addr(ring + TS_RING_HEAD).lw(c);

        // Push the five values.
        let s1 = c.g[S1][0];
        ts_ring_push(c, ring, s1, true, false);
        let s2 = c.g[S2][0];
        ts_ring_push(c, ring, s2, false, false);
        let s3 = c.g[S3][0];
        ts_ring_push(c, ring, s3, false, false);
        let s4 = c.g[S4][0];
        ts_ring_push(c, ring, s4, false, false);
        let s5 = c.g[S5][0];
        ts_ring_push(c, ring, s5, false, true);
        frame.restore(c, S0, 0x60);
        c.cycles += 8;
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
// SSXAudioSystem_StartBoostSound (0x298d90)
// ---------------------------------------------------------------------------

/// Rider fields: boost active and boost amount (both non-zero to start).
const BOOST_RIDER_ACTIVE: u32 = 0x874;
const BOOST_RIDER_AMOUNT: u32 = 0x87c;
/// Audio system fields: the boost voice handle and the boost start count
/// (the start voice only plays on the first start).
const BOOST_HANDLE: u32 = 0x59e0;
const BOOST_COUNT: u32 = 0x59e4;
/// The boost sounds' timer callback key (8 bytes, `D_004A36A8`).
const BOOST_TIMER_KEY: u32 = 0x004a_36a8;
/// The play-record track (`*self`): write cursor at +0x1f4, the default
/// record at +0x1dc (0x18 bytes), the record arrays' descriptor at +0x1d8.
const BOOST_TRACK_CURSOR: u32 = 0x1f4;
const BOOST_TRACK_ARRAYS: u32 = 0x1d8;

const BOOST_IS_MUTED: u32 = 0x0028_8ae0;
const BOOST_KILL_TIMERS: Callee = Callee {
    addr: 0x002a_dde0,
    f: ee::s08::BankMonitor_KillTimersByCallback,
};
const BOOST_MEMSET: u32 = 0x0041_6210;
const BOOST_GET_CHANNEL_GAIN: u32 = 0x0028_7968;
const BOOST_PLAY_PUSHED: u32 = 0x0029_06b8;

/// `lw(arrays + base_off) + (cursor << shift)` with the cursor re-read, the
/// sum in v0 (`in_v0`) or v1 and the array base in the other; then the
/// store of `value` at `+field`.
fn boost_store(c: &mut Cpu, base_off: u32, shift: u32, field: u32, value: u64, in_v0: bool) {
    let track = r32(c, S0);
    let arrays = r32(c, S1);
    let cursor = Addr(track + BOOST_TRACK_CURSOR).lw(c);
    let base = Addr(arrays.wrapping_add(base_off)).lw(c);
    let at = sx(((cursor as u32) << shift).wrapping_add(base as u32));
    let (a, b) = if in_v0 { (V0, V1) } else { (V1, V0) };
    c.g[a][0] = at;
    c.g[b][0] = base;
    Addr((at as u32).wrapping_add(field)).sw(c, value);
}

/// Push a play record for `sound` on the track and play it (the inlined
/// `PushPlayParams` + `PlaySound` of the original); returns with the
/// handle in v0. `s2_sound`: the second copy keeps the sound id in s2 (set
/// first) instead of v1.
fn boost_push_and_play(c: &mut Cpu, cycles: u64, sound: u64, s2_sound: bool, rets: [u32; 3]) -> crate::call::Flow {
    c.cycles += cycles;
    let this = r32(c, S3);
    let track = Addr(this).lw(c);
    c.g[S0][0] = track;
    let track = track as u32;
    c.g[A1][0] = 0;
    c.g[A2][0] = 8;
    let cursor = Addr(track + BOOST_TRACK_CURSOR).lw(c);
    c.g[S1][0] = sx(track.wrapping_add(BOOST_TRACK_ARRAYS));
    if s2_sound {
        c.g[S2][0] = sound;
    }
    let arrays = r32(c, S1);
    // cursor++.
    let cursor = sx((cursor as u32).wrapping_add(1));
    c.g[V0][0] = cursor;
    Addr(track + BOOST_TRACK_CURSOR).sw(c, cursor);
    // records[cursor] = the default record (0x18 bytes).
    let cursor = Addr(track + BOOST_TRACK_CURSOR).lw(c);
    let records = Addr(arrays.wrapping_add(0x20)).lw(c);
    c.g[V0][0] = records;
    let stride = r32(c, T1);
    let row = abi::mult(c, cursor as u32, stride);
    let dst = sx(row.wrapping_add(records as u32));
    c.g[V1][0] = dst;
    let w0 = Addr(track + 0x1dc).read_u64_unaligned(c);
    let w1 = Addr(track + 0x1e4).read_u64_unaligned(c);
    let w2 = Addr(track + 0x1ec).read_u64_unaligned(c);
    c.g[V0][0] = w0;
    c.g[A0][0] = w1;
    c.g[A3][0] = w2;
    Addr(dst as u32).write_u64_unaligned(c, w0);
    Addr((dst as u32).wrapping_add(8)).write_u64_unaligned(c, w1);
    Addr((dst as u32).wrapping_add(0x10)).write_u64_unaligned(c, w2);
    // track->flags[cursor] = 0.
    let cursor = Addr(track + BOOST_TRACK_CURSOR).lw(c);
    let at = sx(((cursor as u32) << 2).wrapping_add(track));
    c.g[V0][0] = at;
    Addr((at as u32).wrapping_add(0x1fc)).sw(c, 0);
    boost_store(c, 0x34, 2, 0, 0, false);
    // priorities[cursor] = (s8) records[cursor].first byte.
    let cursor = Addr(track + BOOST_TRACK_CURSOR).lw(c);
    let cursor2 = Addr(track + BOOST_TRACK_CURSOR).lw(c);
    let records = Addr(arrays.wrapping_add(0x20)).lw(c);
    c.g[T0][0] = records;
    let stride = r32(c, T1);
    let row = abi::mult(c, cursor2 as u32, stride);
    let base = Addr(arrays.wrapping_add(0x38)).lw(c);
    c.g[V0][0] = base;
    let dst = sx(((cursor as u32) << 2).wrapping_add(base as u32));
    c.g[A3][0] = dst;
    let src = sx(row.wrapping_add(records as u32));
    c.g[A0][0] = src;
    let first = Addr(src as u32).lb(c);
    c.g[V1][0] = first;
    Addr(dst as u32).sw(c, first);
    boost_store(c, 0x3c, 2, 0, 0, true);
    // memset(&arrays->params[cursor], 0, 8).
    let cursor = Addr(track + BOOST_TRACK_CURSOR).lw(c);
    c.g[A0][0] = sx((((cursor as u32) << 3).wrapping_add(arrays)).wrapping_add(0x40));
    call_ptr(c, BOOST_MEMSET, rets[0])?;
    // The rest of the record: volume 100, pan 90, 50, pitch 127, 1.
    let track = r32(c, S0);
    let arrays = r32(c, S1);
    let _ = (track, arrays);
    c.g[A3][0] = 0x64;
    c.g[T0][0] = 0x5a;
    c.g[T1][0] = 0x32;
    c.g[T2][0] = 0x7f;
    c.g[T3][0] = 1;
    boost_store(c, 0x60, 2, 0, 0, false);
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = 9;
    c.g[A2][0] = 0;
    boost_store(c, 0x64, 2, 0, 0, false);
    boost_store(c, 0x68, 2, 0, 0, true);
    boost_store(c, 0x6c, 2, 0, 0, false);
    boost_store(c, 0x70, 4, 0x4, 0, true);
    let v = c.g[A3][0];
    boost_store(c, 0x70, 4, 0x0, v, false);
    let v = c.g[T0][0];
    boost_store(c, 0x70, 4, 0x8, v, true);
    let v = c.g[T1][0];
    boost_store(c, 0x70, 4, 0xc, v, false);
    boost_store(c, 0x74, 2, 0, 0, true);
    boost_store(c, 0x80, 2, 0, 0, false);
    boost_store(c, 0x84, 2, 0, 0, true);
    let v = c.g[T2][0];
    boost_store(c, 0x78, 2, 0, v, false);
    let v = c.g[T3][0];
    boost_store(c, 0x7c, 2, 0, v, true);
    // self->handles[cursor] = 0; self->sounds[cursor] = sound.
    let this = r32(c, S3);
    for (field, last) in [(0x4u32, false), (0x8, true)] {
        let track = Addr(this).lw(c);
        c.g[V1][0] = track;
        let table = Addr(this + field).lw(c);
        c.g[A3][0] = table;
        let cursor = Addr((track as u32).wrapping_add(BOOST_TRACK_CURSOR)).lw(c);
        let at = sx(((cursor as u32) << 2).wrapping_add(table as u32));
        c.g[V0][0] = at;
        if last {
            let value = if s2_sound { c.g[S2][0] } else {
                c.g[V1][0] = sound;
                sound
            };
            Addr(at as u32).sw(c, value);
        } else {
            Addr(at as u32).sw(c, 0);
        }
    }
    // gains[cursor] = SSXAudioSystem_GetChannelGainPtr(...); play it.
    c.g[S0][0] = Addr(this + 0x118).lw(c);
    call_ptr(c, BOOST_GET_CHANNEL_GAIN, rets[1])?;
    let owner = Addr(r32(c, S0)).lw(c);
    c.g[A1][0] = owner;
    c.g[A0][0] = c.g[S3][0];
    let cursor = Addr((owner as u32).wrapping_add(BOOST_TRACK_CURSOR)).lw(c);
    let gains = Addr((owner as u32).wrapping_add(0x214)).lw(c);
    c.g[A2][0] = gains;
    let at = sx(((cursor as u32) << 2).wrapping_add(gains as u32));
    c.g[V1][0] = at;
    let gain = c.g[V0][0];
    Addr(at as u32).sw(c, gain);
    call_ptr(c, BOOST_PLAY_PUSHED, rets[2])
}

/// `void SSXAudioSystem_StartBoostSound(SSXAudioSystem* self, Rider* rider,
/// int withVoice)` (0x298d90): unless the audio is muted (0x288ae0) or the
/// rider's boost is off, kill the boost timers, play the boost start sound
/// (0x7a, only on the first start; its handle kept at `+0x59e0`), the
/// boost voice (0x4a) when asked, and count the start.
pub fn ssx_audio_system_start_boost_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, BOOST_IS_MUTED, 0x0029_8db8)?;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            let rider = r32(c, S0);
            let active = Addr(rider + BOOST_RIDER_ACTIVE).lw(c);
            c.g[V0][0] = active;
            c.g[A1][0] = 0;
            if active != 0 {
                c.cycles += 2;
                let amount = Addr(rider + BOOST_RIDER_AMOUNT).lw(c);
                c.g[V0][0] = amount;
                c.g[A1][0] = (0 < amount) as u64;
            }
            c.cycles += 2;
            c.g[V0][0] = 0x004a_0000;
            if c.g[A1][0] != 0 {
                c.cycles += 9;
                let this = r32(c, S3);
                let mgr = Addr(this + 0x118).lw(c);
                c.g[V1][0] = mgr;
                c.g[A1][0] = Addr(BOOST_TIMER_KEY).read_u64_unaligned(c);
                let owner = Addr(mgr as u32).lw(c);
                c.g[A0][0] = sx((owner as u32).wrapping_add(0x1d8));
                BOOST_KILL_TIMERS.call(c, 0x0029_8df4)?;
                let count = Addr(r32(c, S3) + BOOST_COUNT).lw(c);
                c.g[V0][0] = count;
                c.g[T1][0] = 0x18;
                if count == 0 {
                    boost_push_and_play(c, 149, 0x7a, false, [0x0029_8ed0, 0x0029_9030, 0x0029_9050])?;
                    let handle = c.g[V0][0];
                    Addr(r32(c, S3) + BOOST_HANDLE).sw(c, handle);
                }
                c.cycles += 2;
                c.g[T1][0] = 0x18;
                if c.g[S2][0] != 0 {
                    boost_push_and_play(c, 148, 0x4a, true, [0x0029_9130, 0x0029_928c, 0x0029_92ac])?;
                }
                c.cycles += 3;
                let this = r32(c, S3);
                let count = sx((Addr(this + BOOST_COUNT).lw(c) as u32).wrapping_add(1));
                c.g[V0][0] = count;
                Addr(this + BOOST_COUNT).sw(c, count);
            }
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_Play (0x3b6820)
// ---------------------------------------------------------------------------

/// The loaded banks (`D_0050AADC`: 16-byte entries, the bank pointer
/// first); a bank's patch count is the `u16` at +6.
const SND_PLAY_BANKS: u32 = 0x0050_aadc;
const SND_PLAY_BANK_PATCHES: u32 = 0x6;
/// SND error: bad bank or patch index.
const SND_PLAY_EBADARG: u64 = 0xffff_ffff_ffff_fff8;

const SND_PLAY_VALIDATE: Callee = Callee {
    addr: 0x003b_64d8,
    f: ee::s13::SND_BankValidate,
};
const SND_PLAY_GET_PATCH: Callee = Callee {
    addr: 0x003c_6540,
    f: ee::s13::SND_BankGetPatch,
};
const SND_PLAY_PLAY_PATCH: Callee = Callee {
    addr: 0x003b_b588,
    f: ee::s13::SND_PlayPatch,
};

/// `int SND_Play(int bank, int patch, PlayParams* params)` (0x3b6820):
/// validates the bank and the patch index (-8 when bad), then plays the
/// bank's patch (`SND_PlayPatch(bank, SND_BankGetPatch(bank, patch), bank
/// index, patch index, params)`) and returns its handle.
pub fn snd_play(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A2][0];
        SND_PLAY_VALIDATE.call(c, 0x003b_6848)?;
        let valid = (c.g[V0][0] as i64) >= 0;
        c.g[V0][0] = 0x0051_0000;
        let mut ok = false;
        if valid {
            c.cycles += 5;
            let banks = Addr(SND_PLAY_BANKS).lw(c);
            c.g[A0][0] = banks;
            let entry = sx((r32(c, S2) << 4).wrapping_add(banks as u32));
            c.g[V1][0] = entry;
            let bank = Addr(entry as u32).lw(c);
            c.g[S0][0] = bank;
            if (c.g[S1][0] as i64) >= 0 {
                c.cycles += 4;
                let count = Addr((bank as u32).wrapping_add(SND_PLAY_BANK_PATCHES)).lhu(c);
                let inside = (c.g[S1][0] as i64) < (count as i64);
                c.g[V0][0] = inside as u64;
                c.g[A0][0] = bank;
                ok = inside;
            }
        }
        if ok {
            c.cycles += 8;
            c.g[A1][0] = c.g[S1][0];
            SND_PLAY_GET_PATCH.call(c, 0x003b_6884)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S1][0];
            c.g[T0][0] = c.g[S3][0];
            SND_PLAY_PLAY_PATCH.call(c, 0x003b_689c)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = SND_PLAY_EBADARG;
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
mod tests {
    use super::*;

    /// SND_NextVoiceGroup: random group masks and request flags (both stage
    /// bits zero or set, b3 = 1 or not), start index before, inside or past
    /// the table; a third of the seeds deliver events at the loop head.
    #[test]
    fn snd_next_voice_group_matches() {
        use crate::difftest::events::schedule;
        use crate::difftest::{check, scramble, Rng, DATA, RA as SENTINEL};
        check(ssx3_ee::s13::f_003c6158__generated, snd_next_voice_group, 300, &|c, seed| {
            let mut r = Rng(0x3c61_5800 ^ seed as u64);
            scramble(c, &mut r);
            let idx_ptr = DATA;
            let req = DATA + 0x200;
            let count = 1 + r.next_u32() % 12;
            c.bus.write8(SND_VOICE_GROUPS + SND_VOICE_GROUP_COUNT, count as u8);
            for i in 0..16 {
                let m = match r.next_u32() % 4 {
                    0 => 0,
                    1 => r.next_u32() & 0x71c,
                    2 => r.next_u32() & 0xe0,
                    _ => r.next_u32(),
                };
                c.bus.write16(SND_VOICE_GROUPS + SND_VOICE_GROUP_MASKS + 2 * i, m as u16);
            }
            let start = match seed % 5 {
                0 => count + r.next_u32() % 3,
                1 => (r.next_u32() % 3).wrapping_neg(),
                _ => r.next_u32() % count,
            };
            c.bus.write32(idx_ptr, start);
            let flags = match (seed / 5) % 4 {
                0 => 0,
                1 => r.next_u32() & 0x71c,
                2 => r.next_u32() & 0xe0,
                _ => r.next_u32(),
            };
            c.bus.write16(req + SND_REQ_FLAGS, flags as u16);
            c.bus.write8(req + SND_REQ_B3, if seed % 2 == 0 { 1 } else { r.next_u32() as u8 });
            c.g[A0][0] = sx(idx_ptr);
            c.g[A1][0] = sx(req);
            c.pc = 0x003c_6158;
            if seed % 7 < 3 {
                c.g[RA][0] = SENTINEL as u64;
                schedule(c, &mut r, seed, 0x003c_6158);
            }
        });
    }

    /// SNDSTRM_ReleaseChunkCallback: chunk header before the data, stream
    /// number byte, handle table; the real StreamFile_ReleaseChunk takes its
    /// short paths (null file, bad magic, chunk outside the buffer, chunk
    /// already released).
    #[test]
    fn sndstrm_release_chunk_callback_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        check(ssx3_ee::s13::f_003b6ad8__generated, sndstrm_release_chunk_callback, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x3b6ad8, 0);
            c.pc = 0x003b_6ad8;
            let data = DATA + 0x10;
            let chunk = DATA + 0x100;
            let case = seed % 4;
            // The stream byte is also the low byte of the chunk's status
            // word: 0xfe with case 3 makes the status "released" (-2).
            let stream = if case == 3 { 0xfe } else { r.next_u32() % 8 };
            let handle_ptr = DATA + 0x200 + 0x10 * (stream & 7);
            let handle = DATA + 0x400;
            let header = DATA + 0x1000;
            c.bus.write32(data - 4, chunk);
            c.bus.write32(SNDSTRM_FILES + 4 * stream, handle_ptr);
            c.bus.write32(handle_ptr, if case == 0 { 0 } else { handle });
            c.bus.write32(handle, header);
            let magic = 0x4d52_5453;
            c.bus.write32(header, if case == 1 { magic ^ (1 + r.next_u32() % 0xff) } else { magic });
            // Buffer: past the chunk (case 2) or around it with the chunk
            // already released (case 3).
            let (start, end) = if case == 2 { (DATA + 0x2000, DATA + 0x3000) } else { (DATA, DATA + 0x3000) };
            c.bus.write32(header + 0x30, start);
            c.bus.write32(header + 0x34, end);
            c.bus.write32(chunk, 0xffff_ff00 | stream);
            c.g[A0][0] = sx(data);
        });
    }

    /// StreamBuffer_GetTimeRemaining: the real StreamBuffer_GetRequestStatus
    /// chain (fake SND lock/unlock hooks, some unwinding) on one SND stream
    /// whose idle status hands request id 0 on: the request playing (head
    /// of the list: state 2, remaining time from its byte counters and the
    /// sample rate), queued behind another (state 1), inactive (state 0) or
    /// not found (state 3).
    #[test]
    fn stream_buffer_get_time_remaining_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, DATA};
        const SELF: u32 = DATA + 0x20_0000;
        const STREAM: u32 = SELF + 0x1000;
        const HEAD: u32 = SELF + 0x2000;
        const REQ: u32 = SELF + 0x2100;
        check(ssx3_ee::s08::f_002aa108__generated, stream_buffer_get_time_remaining, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x2a_a108, 4);
            c.pc = 0x002a_a108;
            c.g[A0][0] = sx(SELF);
            // SND lock / unlock hooks.
            c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
            c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
            // One stream slot (SND stream 0), idle: status 0, request id 0.
            c.bus.write8(0x0051_0000 - 0x56d2, 1);
            c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
            c.bus.write32(SELF + 0x8, 0);
            c.bus.write32(STREAM + 0x124, 0);
            c.bus.write16(STREAM + 0x14, 1 + (r.next_u32() % 48000) as u16);
            c.bus.write16(STREAM + 0x18, 1 + (r.next_u32() % 48000) as u16);
            c.bus.write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
            // Request list: REQ (id 0) first, behind HEAD (id 1), or absent.
            let case = seed % 4;
            let first = if case == 1 { HEAD } else { REQ };
            c.bus.write32(STREAM + 0x11c, first);
            c.bus.write32(HEAD, REQ);
            c.bus.write32(HEAD + 0xc, 1);
            c.bus.write32(REQ, 0);
            c.bus.write32(REQ + 0xc, if case == 3 { 7 } else { 0 });
            c.bus.write8(REQ + 0x24, if case == 2 { 0 } else { 1 + r.next_u32() as u8 % 4 });
            let done = r.next_u32() & 0xf_ffff;
            c.bus.write32(REQ + 0x14, done);
            c.bus.write32(REQ + 0x18, done + (r.next_u32() & 0xf_ffff));
            c.bus.write32(REQ + 0x1c, r.next_u32() & 0xffff);
        });
    }

    /// EAFile_AllocRequest: pools of 1..6 slots with the first free one
    /// anywhere (or a negative count: no scan), serials near the wrap; the
    /// real EAMutex lock/unlock with fake thread-id/semaphore syscalls (some
    /// unwinding). The all-busy case spins forever in the original and is
    /// not run.
    #[test]
    fn ea_file_alloc_request_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        const SLOTS: u32 = DATA + 0x1000;
        check(ssx3_ee::s14::f_003de670__generated, ea_file_alloc_request, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x3d_e670, 6);
            c.pc = 0x003d_e670;
            let count: i32 = if seed % 7 == 0 { -1 - (r.next_u32() % 3) as i32 } else { 1 + (r.next_u32() % 6) as i32 };
            c.bus.write32(EAFILE_POOL, count as u32);
            c.bus.write32(EAFILE_POOL + EAFILE_POOL_SLOTS, SLOTS);
            // Mutex: owner never a fake thread id, depth, semaphore.
            c.bus.write32(EAFILE_POOL_MUTEX + 0x4, 0x7700_0000 | r.next_u32() & 0xffff);
            c.bus.write32(EAFILE_POOL_MUTEX + 0x8, r.next_u32() & 3);
            c.bus.write32(EAFILE_POOL_MUTEX + 0xc, r.next_u32() & 0xff);
            c.bus.write32(0x0045_0de4, r.next_u32() & 1);
            let free = if count > 0 { r.next_u32() % count as u32 } else { 0 };
            for i in 0..8u32 {
                let mut w = r.next_u32();
                if i == free {
                    w &= !0x00f0_0000;
                } else if i < free && w & 0x00f0_0000 == 0 {
                    w |= 0x0010_0000 << (r.next_u32() % 4);
                }
                c.bus.write32(SLOTS + EAFILE_REQ_SIZE * i, w);
            }
            let serial = match seed % 3 {
                0 => 0xf_fffe + r.next_u32() % 3,
                1 => r.next_u32(),
                _ => r.next_u32() & 0xf_ffff,
            };
            c.bus.write32(EAFILE_REQ_SERIAL, serial);
        });
    }

    /// GameState_SaveState: a serialiser whose vtable slot 1 is a fake
    /// method (random this-delta; some calls unwind).
    #[test]
    fn game_state_save_state_matches() {
        use crate::difftest::fakes::{covered, machine, ptr, vtable};
        use crate::difftest::DATA;
        covered(ssx3_ee::s06::f_00233af0__generated, game_state_save_state, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x23_3af0, 1);
            c.pc = 0x0023_3af0;
            let serialiser = DATA + 0x20_0000 + (r.next_u32() & 0xfff0);
            let vt = vtable(c, &mut r, &[GAME_STATE_SAVE_SLOT]);
            c.bus.write32(serialiser, vt);
            c.g[A0][0] = sx(ptr(&mut r));
            c.g[A1][0] = sx(serialiser);
        });
    }

    /// SSXAudioSystem_TrackSound: ring capacities 0..9 (0: the division's
    /// hardware result), random head/tail or a full ring (capacity 1-2: the
    /// handle pop finds it empty), indices past the end. The real
    /// BankMonitor_StopHandle finds no instance (all 64 free).
    #[test]
    fn ssx_audio_system_track_sound_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        const THIS: u32 = DATA + 0x20_0000;
        const MGR: u32 = DATA + 0x21_0000;
        const BANK_OWNER: u32 = DATA + 0x21_0100;
        const INSTANCES: u32 = DATA + 0x22_0000;
        const RING_DATA: u32 = DATA + 0x21_8000;
        check(ssx3_ee::s08::f_00295628__generated, ssx_audio_system_track_sound, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x29_5628, 0);
            c.pc = 0x0029_5628;
            c.g[A0][0] = sx(THIS);
            c.bus.write32(THIS + TS_MGR, MGR);
            c.bus.write32(MGR, BANK_OWNER);
            c.bus.write32(BANK_OWNER + TS_BANK_MONITOR + 0x8ec, INSTANCES);
            let ring = THIS + TS_RING;
            let cap = r.next_u32() % 10;
            c.bus.write32(ring, cap);
            c.bus.write32(ring + TS_RING_DATA, RING_DATA);
            fill(c, &mut r, RING_DATA, 16);
            let m = cap.max(1);
            let head = r.next_u32() % m;
            let tail = match seed % 4 {
                0 | 1 => (head + 1) % m,
                2 => r.next_u32() % m,
                _ => r.next_u32() % 16,
            };
            c.bus.write32(ring + TS_RING_HEAD, if seed % 9 == 8 { cap + r.next_u32() % 3 } else { head });
            c.bus.write32(ring + TS_RING_TAIL, tail);
        });
    }

    /// SSXAudioSystem_StartBoostSound: the MIDI-bank fixture of the
    /// PlaySound tests (real PushPlayParams-style records, real
    /// GetChannelGainPtr / Audio_PlayPushedSound), rider boost on or off,
    /// first start or not (start sound), voice flag on or off. The timer key
    /// matches no timer (the real KillTimersByCallback only scans).
    #[test]
    fn ssx_audio_system_start_boost_sound_matches() {
        use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
        use crate::game::sound::soundsys::tests::midi_bank;
        check(ssx3_ee::s08::f_00298d90__generated, ssx_audio_system_start_boost_sound, 200, &|c, seed| {
            let mut r = Rng(0x0029_8d90 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0029_8d90;
            c.g[GP][0] = GAME_GP as u64;
            let this = DATA + 0x100;
            let rider = DATA + 0x3000;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(rider);
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { sx(1 + r.next_u32() % 4) };
            midi_bank(c, &mut r, seed);
            c.bus.write32(rider + BOOST_RIDER_ACTIVE, (seed / 2) % 3);
            c.bus.write32(rider + BOOST_RIDER_AMOUNT, [0, 1, 0x8000_0000, 7][(seed as usize / 6) % 4]);
            c.bus.write32(this + BOOST_COUNT, if seed % 5 < 2 { 0 } else { r.next_u32() % 9 });
            c.bus.write32(BOOST_TIMER_KEY, 0x5a5a_a5a5);
            c.bus.write32(BOOST_TIMER_KEY + 4, 0x7e7e_e7e7);
        });
    }

    /// SND_Play: bank index negative / past the count / unloaded / loading
    /// (the real SND_BankValidate), patch index negative, past the bank's
    /// count or inside it; inside, the real SND_BankGetPatch finds an empty
    /// patch slot and the real SND_PlayPatch returns -8 at once.
    #[test]
    fn snd_play_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        const STATE: u32 = 0x0050_a8e8;
        const VALIDATE_TABLE: u32 = DATA + 0x1000;
        const BANKS: u32 = DATA + 0x2000;
        const BANK: u32 = DATA + 0x3000;
        check(ssx3_ee::s13::f_003b6820__generated, snd_play, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x3b_6820, 0);
            c.pc = 0x003b_6820;
            let nbanks = 1 + r.next_u32() % 6;
            c.bus.write16(STATE + 0x28, nbanks as u16);
            c.bus.write32(STATE + 0x1f4, VALIDATE_TABLE);
            c.bus.write32(SND_PLAY_BANKS, BANKS);
            let bank = match seed % 6 {
                0 => (r.next_u32() % 3).wrapping_neg(),
                1 => nbanks + r.next_u32() % 3,
                _ => r.next_u32() % nbanks,
            };
            for i in 0..8u32 {
                let e = VALIDATE_TABLE + 16 * i;
                c.bus.write32(e, if seed % 6 == 2 { 0 } else { 1 + r.next_u32() });
                c.bus.write8(e + 0xc, if seed % 6 == 3 { 1 + r.next_u32() as u8 % 4 } else { 0 });
                c.bus.write32(BANKS + 16 * i, BANK);
            }
            let npatches = r.next_u32() % 6;
            c.bus.write16(BANK + SND_PLAY_BANK_PATCHES, npatches as u16);
            let patch = match (seed / 6) % 4 {
                0 => (1 + r.next_u32() % 3).wrapping_neg(),
                1 => npatches + r.next_u32() % 3,
                _ => r.next_u32() % npatches.max(1),
            };
            c.g[A0][0] = sx(bank);
            c.g[A1][0] = sx(patch);
            c.g[A2][0] = sx(DATA + 0x4000);
        });
    }
}
