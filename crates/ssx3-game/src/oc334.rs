//! Staging module oc334 (wave wave214): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc334`.

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
    (
        0x0037_d090,
        ps2_graphics_man_set_texture_filter,
        "PS2GraphicsMan_SetTextureFilter",
    ),
    (0x003e_0118, stream_file_on_close, "StreamFile_OnClose"),
    (0x003b_7fb8, sndstrm_set_pitch, "SNDSTRM_SetPitch"),
    (0x0035_9f30, spline_modifier_save, "SplineModifier_Save"),
    (
        0x001e_1548,
        message_center_empty_trigger,
        "MessageCenter_EmptyTrigger",
    ),
    (
        0x003d_e8c0,
        ea_file_free_file_handle,
        "EAFile_FreeFileHandle",
    ),
    (0x003d_76f0, spch_queue_event, "SPCH_QueueEvent"),
    (0x003d_8040, spch_service_channel, "SPCH_ServiceChannel"),
    (0x003c_d1a8, snd_utk_alloc, "SND_UtkAlloc"),
    (
        0x0010_4ca0,
        rider_anim_seq_end_type00,
        "RiderAnim_SeqEndType00",
    ),
];

// ---------------------------------------------------------------------------
// PS2GraphicsMan_SetTextureFilter (0x37d090)

/// The graphics manager's texture slot 1 filter field.
pub(crate) const PS2GM_FILTER: u32 = 0x18f4;

/// `TexMan_SetTex1Filter(TexMan* man, int index, ...)` (0x369130).
const PS2GM_SET_TEX1_FILTER: Callee = Callee {
    addr: 0x0036_9130,
    f: ee::s12::TexMan_SetTex1Filter,
};

/// `void PS2GraphicsMan_SetTextureFilter(GraphicsMan* gs)` (0x37d090): apply
/// the filter of the texture-function descriptor kept at `gs + 0x18f4`: the
/// 7-cycle forwarding wrapper for `TexMan_SetTex1Filter(gs->descriptor)`.
pub fn ps2_graphics_man_set_texture_filter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, A0).wrapping_add(PS2GM_FILTER)).lw(c);
        PS2GM_SET_TEX1_FILTER.call(c, 0x0037_d0a0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamFile_OnClose (0x3e0118)

/// Stream file fields OnClose touches: the pending request (`+0x174`), the
/// refill priority (`+0x40`) and its name buffer (`+0x6c`).
pub(crate) const SFOC_REQUEST: u32 = 0x174;

pub(crate) const SFOC_PRIORITY: u32 = 0x40;

pub(crate) const SFOC_NAME: u32 = 0x6c;

/// The callback an opened stream sets on its request (`func_003E00D8`).
pub(crate) const SFOC_READ_CALLBACK: u32 = 0x003e_00d8;

const SFOC_RELEASE_REQUEST: Callee = Callee {
    addr: 0x003d_d310,
    f: ee::s14::EAFile_ReleaseRequest,
};

const SFOC_QUEUE_OPEN: Callee = Callee {
    addr: 0x003d_d4e8,
    f: ee::s14::EAFile_QueueOpen,
};

const SFOC_SET_REQUEST_CALLBACK: Callee = Callee {
    addr: 0x003d_cf70,
    f: ee::s14::EAFile_SetRequestCallback,
};

/// `void StreamFile_OnClose(int, int, StreamFile* st)` (0x3e0118): release
/// the current file request, then queue a fresh open of the stream's name
/// (`EAFile_QueueOpen(st + 0x6c, 1, st->priority, st)`) and, when that
/// produced a request, set the stream's read callback on it. The request
/// handle lives at `st + 0x174`.
pub fn stream_file_on_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(SFOC_REQUEST)).lw(c);
        SFOC_RELEASE_REQUEST.call(c, 0x003e_0130)?;
        c.g[A2][0] = Addr(r32(c, S0).wrapping_add(SFOC_PRIORITY)).lw(c);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(SFOC_NAME));
        c.g[A1][0] = 1;
        c.g[A3][0] = c.g[S0][0];
        SFOC_QUEUE_OPEN.call(c, 0x003e_0144)?;
        let failed = c.g[V0][0] == 0;
        Addr(r32(c, S0)).off(SFOC_REQUEST).sw(c, c.g[V0][0]);
        if !failed {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = sx(SFOC_READ_CALLBACK);
            SFOC_SET_REQUEST_CALLBACK.call(c, 0x003e_015c)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SNDSTRM_SetPitch (0x3b7fb8)

/// Stream object fields: the voice handle the stream plays on (`+4`) and the
/// requested pitch scale (`+0xf8`, a halfword).
pub(crate) const SNDSTRM_HANDLE: u32 = 0x4;

pub(crate) const SNDSTRM_PITCH: u32 = 0xf8;

/// The largest pitch scale a stream accepts.
pub(crate) const SNDSTRM_MAX_PITCH: u64 = 0x4000;

const SNDSTRM_GET_STREAM: Callee = Callee {
    addr: 0x003b_6a38,
    f: ee::s13::SNDSTRM_GetStream,
};

const SNDSTRM_SET_PITCH: Callee = Callee {
    addr: 0x003b_8790,
    f: ee::s13::SND_SetPitch,
};

/// `int SNDSTRM_SetPitch(int stream, int pitch)` (0x3b7fb8): set the stream's
/// pitch scale (clamped to 0x4000) and apply it to its voice handle; -8 when
/// there is no such stream.
pub fn sndstrm_set_pitch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        SNDSTRM_GET_STREAM.call(c, 0x003b_7fcc)?;
        c.g[A2][0] = c.g[V0][0];
        let no_stream = c.g[A2][0] == 0;
        c.g[V1][0] = ((c.g[S0][0] as i64) < 16385i64) as u64;
        if no_stream {
            c.cycles += 1;
            c.g[V0][0] = sx(0xffff_fff8);
        } else {
            c.cycles += 8;
            c.g[V0][0] = SNDSTRM_MAX_PITCH;
            if c.g[V1][0] == 0 {
                c.g[S0][0] = c.g[V0][0];
            }
            c.g[A0][0] = Addr(r32(c, A2).wrapping_add(SNDSTRM_HANDLE)).lw(c);
            c.g[A1][0] = c.g[S0][0];
            Addr(r32(c, A2)).off(SNDSTRM_PITCH).sh(c, c.g[S0][0]);
            SNDSTRM_SET_PITCH.call(c, 0x003b_7ff0)?;
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
// SplineModifier_Save (0x359f30)

const SPLINE_SAVE_PATH: Callee = Callee {
    addr: 0x0034_5538,
    f: ee::s11::SplineModifier_SavePath,
};

/// `void cSplineModifier::writeToReplayFrame(cSplineModifier* self, void*
/// stream)` (0x359f30): write `self + 0x10` (0x50 bytes) through the stream's
/// second virtual method (`stream->v02(ptr, len)`, slot at vtable `+8`), then
/// the spline path (`self + 0xd8`).
pub fn spline_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x50;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x10));
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x8)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0xc)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0035_9f64)?;
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0xd8));
        c.g[A1][0] = c.g[S0][0];
        SPLINE_SAVE_PATH.call(c, 0x0035_9f70)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MessageCenter_EmptyTrigger (0x1e1548)

/// `void MessageCenter_EmptyTrigger(void)` (0x1e1548): the trigger slot the
/// message center leaves empty — it only consumes its two-cycle block and
/// returns.
pub fn message_center_empty_trigger(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// EAFile_FreeFileHandle (0x3de8c0)

/// The EAFile file-handle pool mutex (`0x520000 - 0x653c`).
pub(crate) const EAF_HANDLE_MUTEX: u32 = 0x0051_9ac4;

/// Size of one EAFile file-handle record.
pub(crate) const EAF_HANDLE_SIZE: u64 = 0x110;

const EAF_MUTEX_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};

const EAF_MUTEX_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};

const EAF_MEMSET: Callee = Callee {
    addr: 0x003e_6448,
    f: ee::s14::EA_MemSet,
};

/// `void EAFile_FreeFileHandle(EAFileHandle* handle)` (0x3de8c0): under the
/// file-handle pool mutex, clear the 0x110-byte handle record (a free handle
/// is an all-zero record).
pub fn ea_file_free_file_handle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0x0052_0000;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x653c));
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        EAF_MUTEX_LOCK.call(c, 0x003d_e8e4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = EAF_HANDLE_SIZE;
        EAF_MEMSET.call(c, 0x003d_e8f4)?;
        c.g[A0][0] = c.g[S0][0];
        EAF_MUTEX_UNLOCK.call(c, 0x003d_e8fc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SPCH_QueueEvent (0x3d76f0)

/// The speech queue's "servicing" flag, `gp + 0x1758` (read by
/// `SPCH_QueueEventArgs`).
#[cfg(test)]
pub(crate) const SPCH_QUEUE_BUSY: u32 = 0x1758;

/// The loop back edge of the vararg copy.
pub(crate) const SPCH_QUEUE_EVENT_LOOP: u32 = 0x003d_7730;

const SPCH_QUEUE_EVENT_ARGS: Callee = Callee {
    addr: 0x003d_7418,
    f: ee::s14::SPCH_QueueEventArgs,
};

/// `void SPCH_QueueEvent(int id, int n, ...)` (0x3d76f0): copy the event id
/// and its `n` varargs into the 20-word argument buffer on the frame (the
/// original's `__builtin_next_arg` walk over the home area) and queue the
/// event. The loop back edge checks events.
pub fn spch_queue_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xd0);
        Addr(frame.at(c, 0xa0)).sd(c, c.g[A2][0]);
        frame.save(c, 31, 0x50);
        c.g[A2][0] = sx(frame.at(c, 0xa0));
        Addr(frame.at(c, 0xa8)).sd(c, c.g[A3][0]);
        Addr(frame.at(c, 0xb0)).sd(c, c.g[T0][0]);
        Addr(frame.at(c, 0xb8)).sd(c, c.g[T1][0]);
        Addr(frame.at(c, 0xc0)).sd(c, c.g[T2][0]);
        Addr(frame.at(c, 0xc8)).sd(c, c.g[T3][0]);
        c.swc1(12, frame.at(c, 0x90));
        c.swc1(14, frame.at(c, 0x94));
        c.swc1(16, frame.at(c, 0x98));
        c.swc1(18, frame.at(c, 0x9c));
        let empty = (c.g[A1][0] as i64) <= 0;
        Addr(frame.at(c, 0)).sw(c, c.g[A0][0]);
        if !empty {
            c.cycles += 1;
            c.g[V1][0] = sx(frame.at(c, 0x4));
            loop {
                c.cycles += 7;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x8));
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1));
                c.g[V0][0] = Addr(r32(c, A2).wrapping_sub(0x8)).lw(c);
                Addr(r32(c, V1)).sw(c, c.g[V0][0]);
                let more = c.g[A1][0] != 0;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, SPCH_QUEUE_EVENT_LOOP);
                }
            }
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[SP][0];
        SPCH_QUEUE_EVENT_ARGS.call(c, 0x003d_7754)?;
        frame.restore(c, 31, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SPCH_ServiceChannel (0x3d8040)

/// The speech queue's "being serviced" flag, `gp + 0x175c`.
pub(crate) const SPCH_SERVICE_BUSY: u32 = 0x175c;

const SPCH_HAS_PENDING_SENTENCE: Callee = Callee {
    addr: 0x003d_a3f0,
    f: ee::s14::f_003da3f0,
};

const SPCH_DEQUEUE_CHANNEL: Callee = Callee {
    addr: 0x003d_7ec8,
    f: ee::s14::f_003d7ec8,
};

const SPCH_PLAY_SENTENCE: Callee = Callee {
    addr: 0x003d_9bd8,
    f: ee::s14::f_003d9bd8,
};

/// `void* SPCH_ServiceChannel(SpchChannel* ch)` (0x3d8040): start the next
/// sentence of a speech channel unless another service run is already in
/// progress (the busy flag at `gp + 0x175c`). A sentence is started when one
/// is pending, or when dequeuing this channel produced a request; the flag
/// is cleared again on the way out.
pub fn spch_service_channel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(SPCH_SERVICE_BUSY)).lw(c);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0);
        c.g[S1][0] = 0;
        frame.save(c, 31, 0x20);
        let busy = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if busy {
            c.cycles += 1;
            c.g[V0][0] = c.g[S1][0];
        } else {
            c.cycles += 5;
            c.g[V0][0] = 1;
            Addr(r32(c, GP)).off(SPCH_SERVICE_BUSY).sw(c, c.g[V0][0]);
            SPCH_HAS_PENDING_SENTENCE.call(c, 0x003d_806c)?;
            let pending = c.g[V0][0] != 0;
            let play = if pending {
                true
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                SPCH_DEQUEUE_CHANNEL.call(c, 0x003d_807c)?;
                let nothing = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[S1][0];
                !nothing
            };
            if play {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                SPCH_PLAY_SENTENCE.call(c, 0x003d_808c)?;
                c.g[S1][0] = c.g[V0][0];
                c.cycles += 1;
                c.g[V0][0] = c.g[S1][0];
            }
        }
        c.cycles += 6;
        frame.restore(c, 31, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0);
        Addr(r32(c, GP)).off(SPCH_SERVICE_BUSY).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_UtkAlloc (0x3cd1a8)

/// The utility-allocator function pointer in the sound library's table
/// (`0x510000 + 0x7604`).
pub(crate) const SND_UTK_ALLOC_FN: u32 = 0x7604;

/// `void* SND_UtkAlloc(int size)` (0x3cd1a8): forward to the utility
/// allocator installed at `0x517604`, with the arguments untouched.
pub fn snd_utk_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V1][0] = 0x0051_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(SND_UTK_ALLOC_FN)).lw(c);
        frame.save_ra(c, 0);
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x003c_d1c0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderAnim_SeqEndType00 (0x104ca0)

/// `AnimSeqList_Remove` (0x3145f8), reached through the function table.
pub(crate) const RIDER_ANIM_LIST_REMOVE: u32 = 0x0031_45f8;

/// The sequence-end result the rider animation dispatcher expects.
pub(crate) const RIDER_ANIM_SEQ_END_RESULT: u64 = 0x1b6;

/// `int RiderAnim_SeqEndType00(RiderAnim* anim, AnimSeqList* list, AnimSeq*
/// seq)` (0x104ca0): unlink and free `seq` from `list` (`AnimSeqList_Remove`,
/// through the function table) and return 0x1b6, the "sequence ended"
/// selector.
pub fn rider_anim_seq_end_type00(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        call_ptr(c, RIDER_ANIM_LIST_REMOVE, 0x0010_4cb4)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = RIDER_ANIM_SEQ_END_RESULT;
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    // ---------------------------------------------- PS2GraphicsMan_SetTextureFilter

    /// The manager and the texture-function table are fixture data; the real
    /// `TexMan_SetTex1Filter` rewrites the descriptor's word at `+0x40`.
    #[test]
    fn ps2_graphics_man_set_texture_filter_matches() {
        check(
            ee::s12::f_0037d090__generated,
            ps2_graphics_man_set_texture_filter,
            200,
            &|c, seed| {
                let mut r = Rng(0x0037_d090 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0037_d090;
                let man = DATA + 0x40;
                let table = DATA + 0x200;
                let descriptor = DATA + 0x1000;
                c.g[A0][0] = sx(man);
                c.bus.write32(man + PS2GM_FILTER, table);
                let index = (seed % 4) as u32;
                c.bus.write32(table + index * 4 + 8, descriptor);
                c.g[A1][0] = sx(index);
                c.g[A2][0] = sx(r.next_u32());
                c.g[A3][0] = sx(r.next_u32());
                c.g[T0][0] = sx(r.next_u32());
                c.g[T1][0] = sx(r.next_u32());
            },
        );
    }

    // ------------------------------------------------------- StreamFile_OnClose

    /// A real EAFile request pool, handle pool, mutex and service queue on
    /// their short paths, a stream whose old request releases cleanly
    /// (state 0 in the request table) and a NUL-terminated name.
    #[test]
    fn stream_file_on_close_matches() {
        const POOL: u32 = 0x0051_9ab0;
        const MUTEX: u32 = POOL + 0x14;
        const SLOTS: u32 = DATA + 0x1000;
        const HANDLES: u32 = DATA + 0x4000;
        const QUEUED: u32 = DATA + 0x8000;
        const FILE: u32 = DATA + 0x1_0000;
        const NAME: u32 = FILE + SFOC_NAME;
        // The request hash table (0x519ad8): zeroed entries mean "no state",
        // so the released request has nothing to free first.
        const TABLE: u32 = DATA + 0x2_0000;
        const OLD_REQUEST: u32 = DATA + 0x3_0000;
        const HANDLE_COUNT: u32 = POOL + 0x4;
        const HANDLE_SLOTS: u32 = POOL + 0x2c;
        check(
            ee::s14::f_003e0118__generated,
            stream_file_on_close,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_0118, 0);
                c.pc = 0x003e_0118;
                // The request pool: 1..6 slots, the first free one anywhere.
                let count = 1 + r.next_u32() % 6;
                c.bus.write32(POOL, count);
                c.bus.write32(POOL + 0x28, SLOTS);
                // The handle pool: 1..6 slots, all free.
                let handles = 1 + r.next_u32() % 6;
                c.bus.write32(HANDLE_COUNT, handles);
                c.bus.write32(HANDLE_SLOTS, HANDLES);
                for i in 0..6u32 {
                    c.bus.write32(HANDLES + 0x110 * i, 0);
                }
                // Mutex: owner never a fake thread id, depth, semaphore.
                c.bus
                    .write32(MUTEX + 0x4, 0x7700_0000 | r.next_u32() & 0xffff);
                c.bus.write32(MUTEX + 0x8, r.next_u32() & 3);
                c.bus.write32(MUTEX + 0xc, r.next_u32() & 0xff);
                c.bus.write32(0x0045_0de4, r.next_u32() & 1);
                let free = r.next_u32() % count;
                for i in 0..8u32 {
                    let mut w = r.next_u32();
                    if i == free {
                        w &= !0x00f0_0000;
                    } else if i < free && w & 0x00f0_0000 == 0 {
                        w |= 0x0010_0000;
                    }
                    c.bus.write32(SLOTS + 0x30 * i, w);
                }
                c.bus.write32(0x0045_0c0c, r.next_u32() & 0xf_ffff);
                // The service queue: a request active; 0..2 queued.
                c.bus.write32(POOL + 0x24, QUEUED + 0x100);
                let queued = seed % 3;
                c.bus
                    .write32(POOL + 0x30, if queued == 0 { 0 } else { QUEUED });
                for k in 0..queued {
                    let q = QUEUED + 0x40 * k;
                    c.bus.write32(q + 0x10, r.next_u32() % 8);
                    c.bus
                        .write32(q + 0x2c, if k + 1 < queued { q + 0x40 } else { 0 });
                }
                c.bus.write32(POOL + 0xc, r.next_u32() & 0xff);
                // The stream file: its old request, priority and name.
                // 0x519ad8 is the request table the EAFile release path
                // indexes by the request pointer's top byte (0 here).
                c.bus.write32(0x0051_9ad8, TABLE);
                for i in 0..0x10u32 {
                    c.bus.write32(OLD_REQUEST + 4 * i, 0);
                }
                c.bus.write32(FILE + SFOC_REQUEST, OLD_REQUEST);
                c.bus.write32(FILE + SFOC_PRIORITY, r.next_u32() & 0x7f);
                let len = r.next_u32() % 0x40;
                for i in 0..len {
                    c.bus.write8(NAME + i, b'a' + (r.next_u32() % 26) as u8);
                }
                c.bus.write8(NAME + len, 0);
                c.g[A2][0] = sx(FILE);
            },
        );
    }

    // ------------------------------------------------------- SNDSTRM_SetPitch

    /// Streams exist (count 4) or the index is out of range; pitches below,
    /// at and above the 0x4000 clamp. The voice handle is invalid (negative),
    /// so the real `SND_SetPitch` takes its error path.
    #[test]
    fn sndstrm_set_pitch_matches() {
        const STREAM_COUNT: u32 = 0x0050_a92e;
        const STREAMS: u32 = 0x0050_a0a0;
        check(ee::s13::f_003b7fb8__generated, sndstrm_set_pitch, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x3b_7fb8, 0);
            c.pc = 0x003b_7fb8;
            let stream = DATA + 0x400;
            c.bus.write8(STREAM_COUNT, 4);
            for i in 0..4u32 {
                c.bus.write32(STREAMS + 4 * i, stream + 0x40 * i);
            }
            let out_of_range = seed % 3 == 0;
            let index = if out_of_range { 6 } else { seed % 4 };
            c.g[A0][0] = sx(index);
            let st = stream + 0x40 * (index & 3);
            let pitch = match seed % 4 {
                0 => r.next_u32() & 0x3fff,
                1 => 0x4000,
                2 => 0x4001 + (r.next_u32() & 0xffff),
                _ => r.next_u32() | 0x8000_0000,
            };
            c.g[A1][0] = sx(pitch);
            c.bus
                .write32(st + SNDSTRM_HANDLE, 0x8000_0000 | r.next_u32());
            c.bus.write32(st + SNDSTRM_PITCH, r.next_u32());
        });
    }

    // ------------------------------------------------------ SplineModifier_Save

    /// A stream whose vtable slot 1 (offset 8) is a fake byte writer; both
    /// the direct call and `SplineModifier_SavePath`'s write go through it.
    #[test]
    fn spline_modifier_save_matches() {
        covered(
            ee::s11::f_00359f30__generated,
            spline_modifier_save,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_9f30, 3);
                c.pc = 0x0035_9f30;
                let stream = DATA + 0x1000;
                let vt = DATA + 0x2000;
                c.bus.write32(stream, vt);
                let delta = (r.next_u32() as i16 & !3) as u32;
                c.bus.write16(vt + 8, delta as u16);
                c.bus.write32(vt + 0xc, FAKE);
                c.g[A0][0] = sx(DATA + (r.next_u32() & 0xff0));
                c.g[A1][0] = sx(stream);
            },
        );
    }

    // ------------------------------------------------ MessageCenter_EmptyTrigger

    #[test]
    fn message_center_empty_trigger_matches() {
        check(
            ee::s04::f_001e1548__generated,
            message_center_empty_trigger,
            40,
            &|c, seed| {
                let mut r = Rng(0x001e_1548 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x001e_1548;
            },
        );
    }

    // --------------------------------------------------- EAFile_FreeFileHandle

    /// A real handle pool mutex (short path) and a handle record to clear.
    #[test]
    fn ea_file_free_file_handle_matches() {
        check(
            ee::s14::f_003de8c0__generated,
            ea_file_free_file_handle,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_e8c0, 4);
                c.pc = 0x003d_e8c0;
                let owner = 0x40 + seed % 3;
                c.bus.write32(EAF_HANDLE_MUTEX + 4, owner);
                c.bus.write32(EAF_HANDLE_MUTEX + 8, 1 + seed % 2);
                c.bus.write32(EAF_HANDLE_MUTEX, r.next_u32() & 0xff);
                let handle = DATA + 0x100 + (r.next_u32() % 0x40) * 4;
                for i in 0..(EAF_HANDLE_SIZE / 4) as u32 {
                    c.bus.write32(handle + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(handle);
            },
        );
    }

    // --------------------------------------------------------- SPCH_QueueEvent

    /// 0..7 varargs around the event id; the real `SPCH_QueueEventArgs` walks
    /// its zeroed event table (no match) and returns on the short path.
    #[test]
    fn spch_queue_event_matches() {
        check(ee::s14::f_003d76f0__generated, spch_queue_event, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x3d_76f0, 0);
            c.pc = 0x003d_76f0;
            c.bus.write32(GAME_GP + SPCH_QUEUE_BUSY, 0);
            c.g[A0][0] = sx(1 + r.next_u32() % 0x40);
            c.g[A1][0] = sx((seed % 8) as u32);
            c.g[A2][0] = sx(r.next_u32());
            c.g[A3][0] = sx(r.next_u32());
            c.g[T0][0] = sx(r.next_u32());
            c.g[T1][0] = sx(r.next_u32());
            c.g[T2][0] = sx(r.next_u32());
            c.g[T3][0] = sx(r.next_u32());
        });
    }

    // ----------------------------------------------------- SPCH_ServiceChannel

    /// Busy (the flag was already set: nothing happens) or idle with nothing
    /// pending and a real `SPCH_DequeueChannel` that finds no request.
    #[test]
    fn spch_service_channel_matches() {
        const PENDING_CHANNEL: u32 = 0x0045_0700;
        const PENDING_COUNT: u32 = 0x0045_0704;
        check(
            ee::s14::f_003d8040__generated,
            spch_service_channel,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_8040, 0);
                c.pc = 0x003d_8040;
                let channel = DATA + 0x400;
                c.g[A0][0] = sx(channel);
                if seed % 2 == 0 {
                    c.bus
                        .write32(GAME_GP + SPCH_SERVICE_BUSY, 1 + (r.next_u32() & 3));
                } else {
                    c.bus.write32(GAME_GP + SPCH_SERVICE_BUSY, 0);
                }
                // No sentence pending for this channel.
                c.bus.write32(PENDING_CHANNEL, channel ^ 4);
                c.bus.write32(PENDING_COUNT, 0);
            },
        );
    }

    // ----------------------------------------------------------- SND_UtkAlloc

    /// The allocator slot points at a fake; the call may unwind.
    #[test]
    fn snd_utk_alloc_matches() {
        covered(ee::s14::f_003cd1a8__generated, snd_utk_alloc, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x3c_d1a8, 1);
            c.pc = 0x003c_d1a8;
            c.bus.write32(0x0051_7604, FAKE + 0x10 * (seed % 4));
            c.g[A0][0] = sx(r.next_u32());
        });
    }

    // -------------------------------------------------- RiderAnim_SeqEndType00

    /// Lists of 0..3 sequences, none of them the one to remove, so the real
    /// `AnimSeqList_Remove` only walks the chain.
    #[test]
    fn rider_anim_seq_end_type00_matches() {
        check(
            ee::s00::f_00104ca0__generated,
            rider_anim_seq_end_type00,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_4ca0, 0);
                c.pc = 0x0010_4ca0;
                let list = DATA + 0x400;
                let seq = DATA + 0x9000;
                let n = seed % 4;
                for i in 0..4u32 {
                    let node = list + 0x100 * (i + 1);
                    c.bus
                        .write32(node + 0xc8, if i + 1 < n { node + 0x100 } else { 0 });
                }
                c.bus
                    .write32(list + 4, if n == 0 { 0 } else { list + 0x100 });
                c.bus.write32(list, n ^ (r.next_u32() & 0xf));
                c.g[A1][0] = sx(list);
                c.g[A2][0] = sx(seq);
            },
        );
    }
}
