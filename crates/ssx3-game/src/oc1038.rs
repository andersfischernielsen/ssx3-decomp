//! Staging module oc1038 (wave wave263): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1038`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `StreamArray::buffers`: the pointer to the array of `StreamBuffer*` slots.
pub(crate) const SAQ_BUFFERS: u32 = 0x8;

/// SND lock, `SNDSTRM_GetRequestStatus` and SND unlock, as the request-state
/// wrappers call them (through the function table).
pub(crate) const SAIR_SND_LOCK: u32 = 0x003b_58a0;
pub(crate) const SAIR_SND_UNLOCK: u32 = 0x003b_58d8;
pub(crate) const SAIR_REQUEST_STATUS: u32 = 0x003b_7c40;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002a_adc0,
        stream_array_queue_from_big,
        "StreamArray_QueueFromBig",
    ),
    (
        0x002a_aec0,
        stream_array_get_request_length,
        "StreamArray_GetRequestLength",
    ),
    (
        0x002a_af08,
        stream_array_has_buffered,
        "StreamArray_HasBuffered",
    ),
    (
        0x002a_af40,
        stream_array_is_request_pending,
        "StreamArray_IsRequestPending",
    ),
    (
        0x002a_af78,
        stream_array_is_request_done,
        "StreamArray_IsRequestDone",
    ),
    (
        0x002a_afd0,
        stream_array_is_request_playing,
        "StreamArray_IsRequestPlaying",
    ),
    (
        0x002a_b060,
        stream_array_clear_prebuffer,
        "StreamArray_ClearPrebuffer",
    ),
    (
        0x002a_b098,
        stream_array_clear_prebuffer_handle,
        "StreamArray_ClearPrebufferHandle",
    ),
];

/// `int StreamArray_QueueFromBig(StreamArray* self, int idx, int a, int b, int c)`
/// (0x2aadc0): queue the request on the stream in slot `idx`
/// (`StreamBuffer_QueueFromBig`), or return -1 when the slot is empty.
pub fn stream_array_queue_from_big(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A3][0];
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(SAQ_BUFFERS)).lw(c);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = Addr(r32(c, A1)).lw(c);
        let has_stream = c.g[A0][0] != 0;
        c.g[A3][0] = c.g[T0][0];
        if has_stream {
            c.cycles += 3;
            c.g[A1][0] = c.g[A2][0];
            c.g[A2][0] = c.g[V1][0];
            Callee {
                addr: 0x002a_a398,
                f: ee::s08::f_002aa398,
            }
            .call(c, 0x002a_adf8)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float StreamArray_GetRequestLength(StreamArray* self, int idx)` (0x2aaec0):
/// the length of the request on slot `idx` (`StreamBuffer_GetRequestLength`),
/// or 0.0 for a negative index or an empty slot. The result is in f0.
pub fn stream_array_get_request_length(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let index_negative = (c.g[A1][0] as i64) < 0;
        frame.save_ra(c, 0);
        let mut has_stream = false;
        if !index_negative {
            c.cycles += 6;
            c.g[V1][0] = Addr(r32(c, A0).wrapping_add(SAQ_BUFFERS)).lw(c);
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[A0][0] = Addr(r32(c, V0)).lw(c);
            has_stream = c.g[A0][0] != 0;
        }
        c.cycles += 3;
        if has_stream {
            Callee {
                addr: 0x002a_a260,
                f: ee::s08::f_002aa260,
            }
            .call(c, 0x002a_aef8)?;
            frame.restore_ra(c, 0);
        } else {
            c.mtc1(0, 0);
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `bool StreamArray_HasBuffered(StreamArray* self, int idx, int min)` (0x2aaf08):
/// whether the stream in slot `idx` has at least `min` buffered
/// (`StreamBuffer_HasBuffered`); false for an empty slot.
pub fn stream_array_has_buffered(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(SAQ_BUFFERS)).lw(c);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = Addr(r32(c, A1)).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 2;
            c.g[A1][0] = c.g[A2][0];
            Callee {
                addr: 0x002a_a150,
                f: ee::s08::f_002aa150,
            }
            .call(c, 0x002a_af30)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool StreamArray_IsRequestPending(StreamArray* self, int idx)` (0x2aaf40):
/// whether the stream in slot `idx` has a request pending
/// (`StreamBuffer_IsRequestPending`); false for an empty slot.
pub fn stream_array_is_request_pending(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(SAQ_BUFFERS)).lw(c);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = Addr(r32(c, A1)).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 2;
            Callee {
                addr: 0x002a_a188,
                f: ee::s08::f_002aa188,
            }
            .call(c, 0x002a_af68)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool StreamArray_IsRequestDone(StreamArray* self, int handle)` (0x2aaf78,
/// streamsys.cpp `cStreamSys_STREAM_IsRequestedDequeued`): true when the
/// request `handle` reports state 3 (done) or the status query failed. The
/// query runs under the SND lock: `SNDSTRM_GetRequestStatus(handle, st)`
/// fills `st[0]` with the state and returns the result in s0.
pub fn stream_array_is_request_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, SAIR_SND_LOCK, 0x002a_af8c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, SAIR_REQUEST_STATUS, 0x002a_af98)?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, SAIR_SND_UNLOCK, 0x002a_afa0)?;
        c.g[V1][0] = Addr(sp(c)).lw(c);
        c.g[V0][0] = 3;
        let done = c.g[V1][0] == c.g[V0][0];
        c.g[A0][0] = 0;
        c.cycles += 2;
        if done {
            c.g[A0][0] = 1;
        } else {
            // The status query's result: below zero means failure, which
            // also counts as done.
            let failed = (c.g[S0][0] as i64) < 0;
            if failed {
                c.cycles += 2;
                c.g[A0][0] = 1;
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        c.g[V0][0] = c.g[A0][0];
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `bool StreamArray_IsRequestPlaying(StreamArray* self, int handle)`
/// (0x2aafd0): true when the request `handle` reports state 2 (playing) and
/// the status query did not fail. Same query as `StreamArray_IsRequestDone`.
pub fn stream_array_is_request_playing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, SAIR_SND_LOCK, 0x002a_afe4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, SAIR_REQUEST_STATUS, 0x002a_aff0)?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, SAIR_SND_UNLOCK, 0x002a_aff8)?;
        c.g[V1][0] = Addr(sp(c)).lw(c);
        c.g[V0][0] = 2;
        c.g[A0][0] = 0;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 2;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 0) as u64;
            c.g[A0][0] = c.g[V0][0] ^ 0x1;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = c.g[A0][0];
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void StreamArray_ClearPrebuffer(StreamArray* self, int idx)` (0x2ab060):
/// clear the prebuffer of the stream in slot `idx` (`StreamBuffer_ClearPrebuffer`);
/// nothing for an empty slot.
pub fn stream_array_clear_prebuffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(SAQ_BUFFERS)).lw(c);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = Addr(r32(c, A1)).lw(c);
        let empty = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !empty {
            c.cycles += 3;
            Callee {
                addr: 0x002a_a1e8,
                f: ee::s08::f_002aa1e8,
            }
            .call(c, 0x002a_b088)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void StreamArray_ClearPrebufferHandle(StreamArray* self, int idx, int handle)`
/// (0x2ab098): clear the prebuffer of request `handle` on slot `idx`
/// (`StreamBuffer_ClearPrebufferHandle`); nothing for an empty slot.
pub fn stream_array_clear_prebuffer_handle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(SAQ_BUFFERS)).lw(c);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = Addr(r32(c, A1)).lw(c);
        let empty = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !empty {
            c.cycles += 3;
            c.g[A1][0] = c.g[A2][0];
            Callee {
                addr: 0x002a_a240,
                f: ee::s08::f_002aa240,
            }
            .call(c, 0x002a_b0c0)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, Rng, DATA};

    /// The stream array of every fixture (its list at `LIST`, the buffer
    /// every slot points at, one SND stream with one request).
    const ARRAY: u32 = DATA + 0x20_0000;
    const LIST: u32 = ARRAY + 0x40;
    const BUFFER: u32 = ARRAY + 0x400;
    const STREAM: u32 = ARRAY + 0x1000;
    const REQ: u32 = ARRAY + 0x2000;
    /// `StreamBuffer::stream` (the SND stream id) and `StreamBuffer::dec`
    /// (the decoder: null takes the short path of `QueueFromBig`).
    const BUFFER_STREAM: u32 = 0x8;
    const BUFFER_DEC: u32 = 0x74;

    /// The stream fixture of the `StreamBuffer_IsPlaying` test: the SND lock
    /// hooks are fakes, one live SND stream with one request (handle at
    /// `REQ + 0xc`), and every array slot holds `BUFFER` except `empty`.
    fn stream_fixture(c: &mut Cpu, r: &mut Rng, seed: u32, empty: Option<u32>) {
        c.bus.write32(ARRAY + SAQ_BUFFERS, LIST);
        for i in 0..4u32 {
            let entry = if empty == Some(i) { 0 } else { BUFFER };
            c.bus.write32(LIST + 4 * i, entry);
        }
        let id = if seed % 3 == 0 {
            1 + r.next_u32() % 4
        } else {
            0
        };
        c.bus.write32(BUFFER + BUFFER_STREAM, id);
        c.bus.write32(BUFFER + BUFFER_DEC, 0);
        c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
        c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
        c.bus.write8(0x0051_0000 - 0x56d2, 1);
        c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
        c.bus.write32(
            STREAM + 0x124,
            if seed % 5 == 1 { 0 } else { r.next_u32() | 1 },
        );
        c.bus.write32(STREAM + 0x11c, REQ);
        c.bus.write32(
            REQ + 0xc,
            if seed % 6 < 3 {
                0x8000_0000 | r.next_u32()
            } else {
                r.next_u32() % 4
            },
        );
        c.bus.write16(STREAM + 0x14, 0);
        c.bus
            .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
    }

    /// A request handle: the fixture's request (found) or a random one.
    fn handle(c: &mut Cpu, r: &mut Rng, seed: u32) -> u64 {
        let h = if seed % 4 == 0 {
            r.next_u32()
        } else {
            c.bus.read32(REQ + 0xc)
        };
        h as i32 as i64 as u64
    }

    #[test]
    fn stream_array_queue_from_big_matches() {
        check(
            ee::s08::f_002aadc0__generated,
            stream_array_queue_from_big,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_adc0, 0);
                c.pc = 0x002a_adc0;
                let idx = seed % 4;
                stream_fixture(c, &mut r, seed, (seed % 3 == 1).then_some(idx));
                c.g[A0][0] = sx(ARRAY);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn stream_array_get_request_length_matches() {
        check(
            ee::s08::f_002aaec0__generated,
            stream_array_get_request_length,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_aec0, 0);
                c.pc = 0x002a_aec0;
                let idx = seed % 4;
                stream_fixture(c, &mut r, seed, (seed % 3 == 1).then_some(idx));
                c.g[A0][0] = sx(ARRAY);
                c.g[A1][0] = if seed % 7 == 0 { u64::MAX } else { idx as u64 };
            },
        );
    }

    #[test]
    fn stream_array_has_buffered_matches() {
        check(
            ee::s08::f_002aaf08__generated,
            stream_array_has_buffered,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_af08, 0);
                c.pc = 0x002a_af08;
                let idx = seed % 4;
                stream_fixture(c, &mut r, seed, (seed % 3 == 1).then_some(idx));
                c.g[A0][0] = sx(ARRAY);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = (r.next_u32() % 64) as u64;
            },
        );
    }

    #[test]
    fn stream_array_is_request_pending_matches() {
        check(
            ee::s08::f_002aaf40__generated,
            stream_array_is_request_pending,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_af40, 0);
                c.pc = 0x002a_af40;
                let idx = seed % 4;
                stream_fixture(c, &mut r, seed, (seed % 3 == 1).then_some(idx));
                c.g[A0][0] = sx(ARRAY);
                c.g[A1][0] = idx as u64;
            },
        );
    }

    #[test]
    fn stream_array_is_request_done_matches() {
        check(
            ee::s08::f_002aaf78__generated,
            stream_array_is_request_done,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_af78, 4);
                c.pc = 0x002a_af78;
                stream_fixture(c, &mut r, seed, None);
                c.g[A0][0] = sx(ARRAY);
                c.g[A1][0] = handle(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn stream_array_is_request_playing_matches() {
        check(
            ee::s08::f_002aafd0__generated,
            stream_array_is_request_playing,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_afd0, 4);
                c.pc = 0x002a_afd0;
                stream_fixture(c, &mut r, seed, None);
                c.g[A0][0] = sx(ARRAY);
                c.g[A1][0] = handle(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn stream_array_clear_prebuffer_matches() {
        check(
            ee::s08::f_002ab060__generated,
            stream_array_clear_prebuffer,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_b060, 0);
                c.pc = 0x002a_b060;
                let idx = seed % 4;
                stream_fixture(c, &mut r, seed, (seed % 3 == 1).then_some(idx));
                c.g[A0][0] = sx(ARRAY);
                c.g[A1][0] = idx as u64;
            },
        );
    }

    #[test]
    fn stream_array_clear_prebuffer_handle_matches() {
        check(
            ee::s08::f_002ab098__generated,
            stream_array_clear_prebuffer_handle,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_b098, 0);
                c.pc = 0x002a_b098;
                let idx = seed % 4;
                stream_fixture(c, &mut r, seed, (seed % 3 == 1).then_some(idx));
                c.g[A0][0] = sx(ARRAY);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = handle(c, &mut r, seed);
            },
        );
    }
}
