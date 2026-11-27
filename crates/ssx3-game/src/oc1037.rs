//! Staging module oc1037 (wave wave263): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1037`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002a_a260,
        stream_buffer_get_request_length,
        "StreamBuffer_GetRequestLength",
    ),
    (0x002a_a2b0, stream_buffer_pause, "StreamBuffer_Pause"),
    (0x002a_a2e8, stream_buffer_resume, "StreamBuffer_Resume"),
    (
        0x002a_a398,
        stream_buffer_queue_from_big,
        "StreamBuffer_QueueFromBig",
    ),
    (
        0x002a_a448,
        stream_buffer_set_stream_param,
        "StreamBuffer_SetStreamParam",
    ),
    (
        0x002a_a870,
        stream_array_create_stream_ex,
        "StreamArray_CreateStreamEx",
    ),
    (0x002a_ac98, stream_array_pause_all, "StreamArray_PauseAll"),
    (
        0x002a_ad08,
        stream_array_resume_all,
        "StreamArray_ResumeAll",
    ),
];

// ---------------------------------------------------------------------------
// StreamBuffer_GetRequestLength (0x2aa260)

/// The request status record `StreamBuffer_GetRequestStatus` fills: the state
/// at +0 (2 = playing) and two lengths at +4 and +8.
pub(crate) const SRL_STATE: u32 = 0x0;

pub(crate) const SRL_LENGTH_A: u32 = 0x4;

pub(crate) const SRL_LENGTH_B: u32 = 0x8;

pub(crate) const SRL_PLAYING: u64 = 2;

/// `StreamBuffer_GetRequestStatus(self, out, scratch)` (0x2aa068).
pub(crate) const SRL_GET_REQUEST_STATUS: Callee = Callee {
    addr: 0x002a_a068,
    f: ee::s08::StreamBuffer_GetRequestStatus,
};

/// `float StreamBuffer_GetRequestLength(StreamBuffer* self)` (0x2aa260):
/// asks for the current request's status (into a stack record); while it is
/// playing (state 2) returns the sum of its two lengths as a float, else 0.
pub fn stream_buffer_get_request_length(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        SRL_GET_REQUEST_STATUS.call(c, 0x002a_a274)?;
        let state = Addr(frame.at(c, SRL_STATE)).lw(c);
        c.g[V1][0] = state;
        c.g[V0][0] = SRL_PLAYING;
        let playing = state == SRL_PLAYING;
        frame.restore_ra(c, 0x10);
        if playing {
            c.cycles += 5;
            c.g[V0][0] = Addr(frame.at(c, SRL_LENGTH_A)).lw(c);
            c.g[V1][0] = Addr(frame.at(c, SRL_LENGTH_B)).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            let length = crate::ps2f::FpuF32::from_i32(c.g[V0][0] as u32 as i32);
            crate::abi::set_fpr(c, 0, length);
        } else {
            c.cycles += 3;
            crate::abi::set_fpr(c, 0, crate::ps2f::FpuF32::ZERO);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_Pause (0x2aa2b0) and StreamBuffer_Resume (0x2aa2e8)

/// `SND_Lock` / `SND_Unlock` (the sound library lock, through the table).
pub(crate) const SBP_SND_LOCK: u32 = 0x003b_58a0;

pub(crate) const SBP_SND_UNLOCK: u32 = 0x003b_58d8;

/// `SNDSTRM_SetPitch(handle, pitch)` (0x3b7fb8): pause sets pitch 0.
pub(crate) const SBP_SNDSTRM_SET_PITCH: u32 = 0x003b_7fb8;

/// StreamBuffer field: the SND stream handle.
pub(crate) const SBP_HANDLE: u32 = 0x8;

/// The pitch `StreamBuffer_Resume` sets.
pub(crate) const SBP_RESUME_PITCH: u64 = 0x1000;

/// `void StreamBuffer_Pause(StreamBuffer* self)` (0x2aa2b0): under the sound
/// lock, `SNDSTRM_SetPitch(self->stream, 0)`.
pub fn stream_buffer_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, SBP_SND_LOCK, 0x002a_a2c4)?;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(SBP_HANDLE)).lw(c);
        c.g[A1][0] = 0;
        call_ptr(c, SBP_SNDSTRM_SET_PITCH, 0x002a_a2d0)?;
        call_ptr(c, SBP_SND_UNLOCK, 0x002a_a2d8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void StreamBuffer_Resume(StreamBuffer* self)` (0x2aa2e8): as
/// `StreamBuffer_Pause`, with the pitch `SBP_RESUME_PITCH`.
pub fn stream_buffer_resume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, SBP_SND_LOCK, 0x002a_a2fc)?;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(SBP_HANDLE)).lw(c);
        c.g[A1][0] = SBP_RESUME_PITCH;
        call_ptr(c, SBP_SNDSTRM_SET_PITCH, 0x002a_a308)?;
        call_ptr(c, SBP_SND_UNLOCK, 0x002a_a310)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_QueueFromBig (0x2aa398)

/// `SNDSTRM` big-file decoder lookup, `BigFile_FindEntry` (0x3e2ce0).
pub(crate) const SQB_BIG_FIND_ENTRY: u32 = 0x003e_2ce0;

/// `SNDSTRM_Queue(handle, a, b, offset)` (0x3b78d8).
pub(crate) const SQB_SNDSTRM_QUEUE: u32 = 0x003b_78d8;

/// StreamBuffer fields: the big-file decoder, the SND stream handle and the
/// base of the queued offset.
pub(crate) const SQB_DECODER: u32 = 0x74;

pub(crate) const SQB_HANDLE: u32 = 0x8;

pub(crate) const SQB_OFFSET: u32 = 0x14;

/// `int StreamBuffer_QueueFromBig(StreamBuffer* self, int a, int b, int off)`
/// (0x2aa398): with a big-file decoder (+0x74), looks up the decoder entry
/// (its size goes to a stack slot) and queues the request on the SND stream
/// at `off` plus that size; without a decoder, returns 0.
pub fn stream_buffer_queue_from_big(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(SQB_DECODER)).lw(c);
        let has_decoder = c.g[A0][0] != 0;
        c.g[S2][0] = c.g[A3][0];
        if has_decoder {
            c.cycles += 10;
            c.g[A3][0] = c.g[SP][0];
            c.g[A1][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, SQB_BIG_FIND_ENTRY, 0x002a_a3d8)?;
            c.g[A3][0] = Addr(frame.at(c, 0)).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(SQB_HANDLE)).lw(c);
            c.g[A2][0] = sx(r32(c, S0).wrapping_add(SQB_OFFSET));
            c.g[A3][0] = sx(r32(c, S2).wrapping_add(r32(c, A3)));
            call_ptr(c, SQB_SNDSTRM_QUEUE, 0x002a_a3f0)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_SetStreamParam (0x2aa448)

/// The SND stream setter the other shard holds (0x3b80e0): `(handle, arg)`.
pub(crate) const SBSP_SNDSTRM_SET: u32 = 0x003b_80e0;

/// `int StreamBuffer_SetStreamParam(StreamBuffer* self, int arg)` (0x2aa448):
/// under the sound lock, `SNDSTRM_Set(self->stream, arg)`; returns its result.
pub fn stream_buffer_set_stream_param(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, SBP_SND_LOCK, 0x002a_a464)?;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(SBP_HANDLE)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, SBSP_SNDSTRM_SET, 0x002a_a470)?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, SBP_SND_UNLOCK, 0x002a_a478)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamArray_CreateStreamEx (0x2aa870)

/// `cMemMan_alloc(size, name, heap, 0)` (0x317d70) and the stream object's
/// size and name string (0x483228).
pub(crate) const SCE_ALLOC: u32 = 0x0031_7d70;

pub(crate) const SCE_NAME: u32 = 0x0048_3228;

pub(crate) const SCE_OBJ_SIZE: u64 = 0x84;

/// StreamArray fields: the buffer pointer array (+8); the sound player the
/// object's constructor takes is at `*self + 0x1d8`.
pub(crate) const SCE_BUFFERS: u32 = 0x8;

pub(crate) const SCE_PLAYER_OFF: u32 = 0x1d8;

/// `StreamBuffer_ctor_2(obj, player, mem, size, n, m)` (0x2a9c00).
pub(crate) const SCE_CTOR: Callee = Callee {
    addr: 0x002a_9c00,
    f: ee::s08::StreamBuffer_ctor_2,
};

/// `StreamBuffer* StreamArray_CreateStreamEx(StreamArray* self, int idx,
/// void* heap, void* mem, int size, int n, int m)` (0x2aa870): allocates a
/// 0x84-byte stream buffer, constructs it on the player at `*self + 0x1d8`
/// and stores it in the array slot `idx`; returns the constructor's result.
pub fn stream_array_create_stream_ex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        c.g[S0][0] = sx(r32(c, A1) << 2);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[T0][0];
        c.g[S4][0] = c.g[T2][0];
        frame.save_ra(c, 0);
        c.g[S5][0] = c.g[A3][0];
        c.g[S3][0] = c.g[T1][0];
        c.g[A1][0] = sx(SCE_NAME);
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SCE_BUFFERS)).lw(c);
        c.g[A0][0] = SCE_OBJ_SIZE;
        c.g[A3][0] = 0;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        call_ptr(c, SCE_ALLOC, 0x002a_a8c4)?;
        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = c.g[S5][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[T0][0] = c.g[S3][0];
        c.g[T1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(SCE_PLAYER_OFF));
        SCE_CTOR.call(c, 0x002a_a8e4)?;
        let slot = r32(c, S0);
        let stream = c.g[V0][0];
        Addr(slot).sw(c, stream);
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
// StreamArray_PauseAll (0x2aac98) and StreamArray_ResumeAll (0x2aad08)

/// StreamArray fields: the buffer count and the buffer pointer array.
pub(crate) const SAP_COUNT: u32 = 0x4;

pub(crate) const SAP_BUFFERS: u32 = 0x8;

/// `StreamBuffer_Pause` (0x2aa2b0) and `StreamBuffer_Resume` (0x2aa2e8).
pub(crate) const SAP_STREAM_BUFFER_PAUSE: Callee = Callee {
    addr: 0x002a_a2b0,
    f: ee::s08::StreamBuffer_Pause,
};

pub(crate) const SAP_STREAM_BUFFER_RESUME: Callee = Callee {
    addr: 0x002a_a2e8,
    f: ee::s08::StreamBuffer_Resume,
};

/// The loop `StreamArray_PauseAll` and `StreamArray_ResumeAll` share: for
/// each non-null buffer in the array (count at +4, pointers at +8), calls
/// `op` with the buffer in a0 (returning to `ret`); `head` is the loop head
/// for the back-edge event check.
fn sap_for_each_buffer(c: &mut Cpu, op: Callee, ret: u32, head: u32) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SAP_COUNT)).lw(c);
    let count = c.g[V0][0] as i64;
    c.g[S0][0] = 0;
    if count > 0 {
        c.cycles += 2;
        c.g[V1][0] = Addr(r32(c, S1).wrapping_add(SAP_BUFFERS)).lw(c);
        loop {
            c.cycles += 5;
            c.g[V0][0] = sx(r32(c, S0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[A0][0] = Addr(r32(c, V0)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                op.call(c, ret)?;
            }
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SAP_COUNT)).lw(c);
            c.cycles += 4;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V1][0] = Addr(r32(c, S1).wrapping_add(SAP_BUFFERS)).lw(c);
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void StreamArray_PauseAll(StreamArray* self)` (0x2aac98): pauses every
/// non-null stream buffer.
pub fn stream_array_pause_all(c: &mut Cpu) {
    guest(c, |c| {
        sap_for_each_buffer(c, SAP_STREAM_BUFFER_PAUSE, 0x002a_acdc, 0x002a_acc0)
    })
}

/// `void StreamArray_ResumeAll(StreamArray* self)` (0x2aad08): resumes every
/// non-null stream buffer.
pub fn stream_array_resume_all(c: &mut Cpu) {
    guest(c, |c| {
        sap_for_each_buffer(c, SAP_STREAM_BUFFER_RESUME, 0x002a_ad4c, 0x002a_ad30)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{covered, machine, machine_rt, FAKE};
    use crate::difftest::DATA;
    use crate::game::lib::snd::tests::snd_world;

    /// StreamBuffer_GetRequestLength: the request status on one SND stream
    /// slot (as the time-remaining test sets it up), with the request's state
    /// 2 (playing) on about a quarter of the seeds and idle, unknown or
    /// zero-state requests on the others: the sum of the two lengths, or 0.
    #[test]
    fn stream_buffer_get_request_length_matches() {
        const SELF: u32 = DATA + 0x20_0000;
        const STREAM: u32 = SELF + 0x1000;
        const REQ: u32 = SELF + 0x2100;
        check(
            ee::s08::f_002aa260__generated,
            stream_buffer_get_request_length,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a260, 4);
                c.pc = 0x002a_a260;
                c.g[A0][0] = sx(SELF);
                // SND lock / unlock hooks.
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                // One stream slot (SND stream 0).
                c.bus.write8(0x0051_0000 - 0x56d2, 1);
                c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
                c.bus.write32(SELF + SBP_HANDLE, 0);
                c.bus.write32(STREAM + 0x124, 0);
                c.bus
                    .write16(STREAM + 0x14, 1 + (r.next_u32() % 48000) as u16);
                c.bus
                    .write16(STREAM + 0x18, 1 + (r.next_u32() % 48000) as u16);
                c.bus
                    .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
                c.bus.write32(STREAM + 0x11c, REQ);
                let case = seed % 4;
                c.bus.write32(REQ, 0);
                c.bus.write32(REQ + 0xc, if case == 3 { 7 } else { 0 });
                let state = match case {
                    1 => 2,
                    2 => 0,
                    _ => 1 + r.next_u32() as u8 % 4,
                };
                c.bus.write8(REQ + 0x24, state);
                let done = r.next_u32() & 0xf_ffff;
                c.bus.write32(REQ + 0x14, done);
                c.bus.write32(REQ + 0x18, done + (r.next_u32() & 0xf_ffff));
                c.bus.write32(REQ + 0x1c, r.next_u32() & 0xffff);
            },
        );
    }

    /// StreamBuffer_Pause: the sound lock hooks are fakes, the stream table
    /// has no streams, so SNDSTRM_SetPitch takes its early return; the handle
    /// is unknown or random.
    #[test]
    fn stream_buffer_pause_matches() {
        covered(
            ee::s08::f_002aa2b0__generated,
            stream_buffer_pause,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a2b0, 2);
                c.pc = 0x002a_a2b0;
                snd_world(c, &mut r, false);
                let sb = DATA + 0x100;
                c.g[A0][0] = sx(sb);
                let handle = if seed % 3 == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32()
                };
                c.bus.write32(sb + SBP_HANDLE, handle);
            },
        );
    }

    /// StreamBuffer_Resume: as the pause test, with the pitch restore.
    #[test]
    fn stream_buffer_resume_matches() {
        covered(
            ee::s08::f_002aa2e8__generated,
            stream_buffer_resume,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a2e8, 2);
                c.pc = 0x002a_a2e8;
                snd_world(c, &mut r, false);
                let sb = DATA + 0x100;
                c.g[A0][0] = sx(sb);
                let handle = if seed % 3 == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32()
                };
                c.bus.write32(sb + SBP_HANDLE, handle);
            },
        );
    }

    /// StreamBuffer_QueueFromBig, the no-decoder path (+0x74 zero): returns 0
    /// after the frame is restored. The decoder path runs the big-file lookup
    /// (BigFile_FindEntry), which cannot be bounded on zeroed data (as in
    /// stream_buffer_queue_matches), so it is not exercised here.
    #[test]
    fn stream_buffer_queue_from_big_matches() {
        check(
            ee::s08::f_002aa398__generated,
            stream_buffer_queue_from_big,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a398, 0);
                c.pc = 0x002a_a398;
                let sb = DATA + 0x100;
                c.bus.write32(sb + SQB_DECODER, 0);
                c.bus.write32(sb + SQB_HANDLE, r.next_u32());
                c.bus.write32(sb + SQB_OFFSET, r.next_u32());
                c.g[A0][0] = sx(sb);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = (r.next_u32() % 0x4000) as u64;
            },
        );
    }

    /// StreamBuffer_SetStreamParam: the lock hooks are fakes, no SND streams
    /// (the setter's early return), a random handle and argument.
    #[test]
    fn stream_buffer_set_stream_param_matches() {
        covered(
            ee::s08::f_002aa448__generated,
            stream_buffer_set_stream_param,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a448, 2);
                c.pc = 0x002a_a448;
                snd_world(c, &mut r, false);
                let sb = DATA + 0x100;
                c.g[A0][0] = sx(sb);
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(sb + SBP_HANDLE, r.next_u32());
            },
        );
    }

    /// StreamArray_CreateStreamEx: the allocator is the real pool allocator
    /// (machine_rt), the constructor runs on a zeroed player; each seed fills
    /// a different array slot, and the heap and size arguments are random.
    #[test]
    fn stream_array_create_stream_ex_matches() {
        check(
            ee::s08::f_002aa870__generated,
            stream_array_create_stream_ex,
            120,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x2a_a870, 0);
                c.pc = 0x002a_a870;
                let this = DATA + 0x20_0000;
                let player = this + 0x1000;
                let buffers = this + 0x100;
                c.bus.write32(this, player);
                c.bus.write32(this + SCE_BUFFERS, buffers);
                c.bus.write32(buffers + 4 * (seed % 4), 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(seed % 4);
                c.g[A2][0] = sx(r.next_u32());
                c.g[A3][0] = sx(r.next_u32());
                c.g[T0][0] = sx(r.next_u32());
                c.g[T1][0] = sx(r.next_u32());
                c.g[T2][0] = sx(r.next_u32());
            },
        );
    }

    /// StreamArray_PauseAll: counts -1..4, null and real buffers; each real
    /// buffer runs the real StreamBuffer_Pause (fake lock hooks, no streams).
    #[test]
    fn stream_array_pause_all_matches() {
        covered(
            ee::s08::f_002aac98__generated,
            stream_array_pause_all,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_ac98, 3);
                c.pc = 0x002a_ac98;
                snd_world(c, &mut r, false);
                sap_fixture(c, &mut r, seed);
            },
        );
    }

    /// StreamArray_ResumeAll: the same fixture as PauseAll.
    #[test]
    fn stream_array_resume_all_matches() {
        covered(
            ee::s08::f_002aad08__generated,
            stream_array_resume_all,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_ad08, 3);
                c.pc = 0x002a_ad08;
                snd_world(c, &mut r, false);
                sap_fixture(c, &mut r, seed);
            },
        );
    }

    /// The array fixture both PauseAll and ResumeAll tests use.
    fn sap_fixture(c: &mut Cpu, r: &mut crate::difftest::Rng, seed: u32) {
        const ARRAY: u32 = DATA + 0x20_0000;
        const SLOTS: u32 = ARRAY + 0x100;
        const BUFS: u32 = ARRAY + 0x200;
        c.g[A0][0] = sx(ARRAY);
        let n = (seed % 6) as i32 - 1;
        c.bus.write32(ARRAY + SAP_COUNT, n as u32);
        c.bus.write32(ARRAY + SAP_BUFFERS, SLOTS);
        for i in 0..5u32 {
            let b = BUFS + 0x100 * i;
            c.bus
                .write32(SLOTS + 4 * i, if r.next_u32() % 4 == 0 { 0 } else { b });
            c.bus.write32(b + SBP_HANDLE, r.next_u32());
        }
    }
}
