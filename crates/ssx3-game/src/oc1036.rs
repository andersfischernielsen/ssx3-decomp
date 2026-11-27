//! Staging module oc1036 (wave wave263): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1036`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002a_8fd8,
        ps2_sound_manager_alloc_spu_block,
        "PS2SoundManager_AllocSpuBlock",
    ),
    (
        0x002a_97c0,
        snd_play_params_dtor_ported,
        "SndPlayParams_dtor",
    ),
    (0x002a_9c00, stream_buffer_ctor_2, "StreamBuffer_ctor_2"),
    (
        0x002a_a0c0,
        stream_buffer_get_request_position,
        "StreamBuffer_GetRequestPosition",
    ),
    (
        0x002a_a150,
        stream_buffer_has_buffered,
        "StreamBuffer_HasBuffered",
    ),
    (
        0x002a_a188,
        stream_buffer_is_request_pending,
        "StreamBuffer_IsRequestPending",
    ),
    (
        0x002a_a1e8,
        stream_buffer_clear_prebuffer,
        "StreamBuffer_ClearPrebuffer",
    ),
    (
        0x002a_a240,
        stream_buffer_clear_prebuffer_handle,
        "StreamBuffer_ClearPrebufferHandle",
    ),
];

/// `StreamBuffer_GetStatus` (0x2aa020): fills the 3-word status record.
const GET_STATUS_OF_BUFFER: Callee = Callee {
    addr: 0x002a_a020,
    f: ee::s08::f_002aa020,
};

/// `StreamBuffer_GetRequestStatus` (0x2aa068): the current request's status.
const GET_REQUEST_STATUS_OF_BUFFER: Callee = Callee {
    addr: 0x002a_a068,
    f: ee::s08::f_002aa068,
};

/// `SNDSTRM_SetPrebuffer` (0x3b7d80), called through the function table.
const SNDSTRM_SET_PREBUFFER_ADDR: u32 = 0x003b_7d80;

/// `SndRequestStatus.state` of a request that is playing.
const REQUEST_PLAYING: u64 = 2;

/// `float StreamBuffer_GetRequestPosition(StreamBuffer* self)` (0x2aa0c0): the
/// current request's status into a frame-local record; while it is playing
/// (state 2), the record's `a` word as a float, else 0.0. The state test leaves
/// `v0` at 2 on the zero path, as the generated body does.
pub fn stream_buffer_get_request_position(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        GET_REQUEST_STATUS_OF_BUFFER.call(c, 0x002a_a0d4)?;
        let state = Addr(frame.at(c, 0)).lw(c);
        c.g[V1][0] = state;
        c.g[V0][0] = REQUEST_PLAYING;
        frame.restore_ra(c, 0x10);
        c.cycles += 3;
        if state == REQUEST_PLAYING {
            c.g[V0][0] = Addr(frame.at(c, 0x4)).lw(c);
            abi::set_fpr(c, 0, FpuF32::from_i32(c.g[V0][0] as u32 as i32));
        } else {
            abi::set_fpr(c, 0, FpuF32::ZERO);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `bool StreamBuffer_HasBuffered(StreamBuffer* self, int minVal)` (0x2aa150):
/// the status record's `b` word is at least `minVal` (signed).
pub fn stream_buffer_has_buffered(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        GET_STATUS_OF_BUFFER.call(c, 0x002a_a168)?;
        c.g[V0][0] = Addr(frame.at(c, 0x8)).lw(c);
        frame.restore_ra(c, 0x10);
        let below = (c.g[V0][0] as i64) < (c.g[S0][0] as i64);
        c.g[V0][0] = (below as u64) ^ 1;
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `bool StreamBuffer_IsRequestPending(StreamBuffer* self)` (0x2aa188): the
/// current request's state is 1.
pub fn stream_buffer_is_request_pending(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        GET_REQUEST_STATUS_OF_BUFFER.call(c, 0x002a_a19c)?;
        let state = Addr(frame.at(c, 0)).lw(c);
        frame.restore_ra(c, 0x10);
        c.g[V0][0] = ((state ^ 1) < 1) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `void StreamBuffer_ClearPrebuffer(StreamBuffer* self)` (0x2aa1e8): the
/// stream's status record gives the stream handle `a`; the prebuffer of that
/// handle is cleared (`SNDSTRM_SetPrebuffer(handle, 0)`).
pub fn stream_buffer_clear_prebuffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        GET_STATUS_OF_BUFFER.call(c, 0x002a_a1f8)?;
        c.g[A0][0] = Addr(frame.at(c, 0x4)).lw(c);
        c.g[A1][0] = 0;
        call_ptr(c, SNDSTRM_SET_PREBUFFER_ADDR, 0x002a_a204)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void StreamBuffer_ClearPrebufferHandle(void* self, int handle)` (0x2aa240):
/// `SNDSTRM_SetPrebuffer(handle, 0)`.
pub fn stream_buffer_clear_prebuffer_handle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        call_ptr(c, SNDSTRM_SET_PREBUFFER_ADDR, 0x002a_a254)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *PS2SoundManager_AllocSpuBlock(PS2SoundManager *self, int size)` (0x002a8fd8):
/// takes an SPU block of `size` from the in-use list, walking it for a block
/// of exactly that size, and otherwise from the free list (after
/// `SND_SpuMemGetFree` / `SND_SpuMemSetBase`). Converted from the generated
/// `match pc` body: every register, cycle and call kept.
pub fn ps2_sound_manager_alloc_spu_block(c: &mut Cpu) {
    if c.pc != 0x002a8fd8 {
        return ee::s08::f_002a8fd8__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x002a8fd8;
        loop {
            match pc {
                0x002a8fd8 => {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x318));
                    let t = (c.g[V1][0] as i64) < 0;
                    c.g[S2][0] = c.g[A1][0];
                    if t {
                        pc = 0x002a9060;
                        continue;
                    }
                    pc = 0x002a8ffc;
                    continue;
                }
                0x002a8ffc => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x18;
                    c.g[A0][0] = 0u64;
                    c.mult(0, 2, 3, 2);
                    c.g[A1][0] = 0x2;
                    c.g[A2][0] = 0xffffffffffffffff;
                    let t = true;
                    c.g[V1][0] = 0x18;
                    if t {
                        pc = 0x002a901c;
                        continue;
                    }
                    pc = 0x002a9018;
                    continue;
                }
                0x002a9018 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S0][0];
                    pc = 0x002a901c;
                    continue;
                }
                0x002a901c => {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));

                    if c.g[V0][0] != c.g[S2][0] {
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        pc = 0x002a904c;
                        continue;
                    }
                    pc = 0x002a9030;
                    continue;
                }
                0x002a9030 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] == 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                    if t {
                        pc = 0x002a9040;
                        continue;
                    }
                    pc = 0x002a9038;
                    continue;
                }
                0x002a9038 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    pc = 0x002a9040;
                    continue;
                }
                0x002a9040 => {
                    c.cycles += 3;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[A2][0]);
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                    if t {
                        pc = 0x002a9110;
                        continue;
                    }
                    pc = 0x002a904c;
                    continue;
                }
                0x002a904c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        pc = 0x002a90d0;
                        continue;
                    }
                    pc = 0x002a9054;
                    continue;
                }
                0x002a9054 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.mult(0, 2, 2, 3);
                    if t {
                        if back_edge_unwinds(c, 0x002a9018) {
                            return Ok(());
                        }
                        pc = 0x002a9018;
                        continue;
                    }
                    pc = 0x002a9060;
                    continue;
                }
                0x002a9060 => {
                    c.cycles += 6;
                    c.sw((c.g[SP][0] as u32), 0u64);
                    c.g[RA][0] = sx(0x002a906c);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x003b5b68, 0x002a906c)?;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x002a9110;
                        continue;
                    }
                    pc = 0x002a9078;
                    continue;
                }
                0x002a9078 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x31c));

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = 0xfffffffffffffffe;
                        pc = 0x002a9110;
                        continue;
                    }
                    pc = 0x002a9084;
                    continue;
                }
                0x002a9084 => {
                    c.cycles += 19;
                    c.g[RA][0] = sx(0x002a908c);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x002a92b8,
                        f: ee::s08::PS2SoundManager_PopFreeSpuBlock,
                    }
                    .call(c, 0x002a908c)?;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30c));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = 0x2;
                    c.g[A1][0] = 0xffffffffffffffff;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[S2][0] as u32)));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[A0][0] = 0xffffffffffffffff;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[A1][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[S2][0]);
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30c));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[S2][0] as u32)));
                    c.g[RA][0] = sx(0x002a90c8);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x30c), c.g[A1][0]);
                    call_ptr(c, 0x003b5b20, 0x002a90c8)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                    if t {
                        pc = 0x002a9110;
                        continue;
                    }
                    pc = 0x002a90d0;
                    continue;
                }
                0x002a90d0 => {
                    c.cycles += 16;
                    c.g[RA][0] = sx(0x002a90d8);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x002a92b8,
                        f: ee::s08::PS2SoundManager_PopFreeSpuBlock,
                    }
                    .call(c, 0x002a90d8)?;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = 0x2;
                    c.g[A0][0] = 0xffffffffffffffff;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[S2][0] as u32)));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[S2][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[S2][0] as u32)));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                    pc = 0x002a9110;
                    continue;
                }
                0x002a9110 => {
                    c.cycles += 6;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002a8fd8, 0x002a9128);
                    return Ok(());
                }
            }
        }
    });
}

/// `void SndPlayParams_dtor(SndPlayParams *p, int flags)` (0x002a97c0): frees the
/// pointer members (`flags` gates the sound-manager reference and the object).
/// Converted from the generated labelled-block body: every register, cycle and
/// call kept.
pub fn snd_play_params_dtor_ported(c: &mut Cpu) {
    if c.pc != 0x002a97c0 {
        return ee::s08::f_002a97c0__generated(c);
    }
    guest(c, |c| {
        'b_2a982c: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.g[V1][0] = 0x480000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3be0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            c.g[V0][0] = c.lw((c.g[S0][0] as u32));
            let t = c.g[S1][0] != 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A3][0]);
            if t {
                break 'b_2a982c;
            } else {
                c.cycles += 16;
                c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x3be0));
                c.g[V0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd((c.g[SP][0] as u32), c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[V1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x88));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                break 'b_2a982c;
            }
        }
        'b_2a9844: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                break 'b_2a9844;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002a9840);
                call_ptr(c, 0x00317e98, 0x002a9840)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                break 'b_2a9844;
            }
        }
        'b_2a9858: {
            c.cycles += 2;
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                break 'b_2a9858;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002a9854);
                call_ptr(c, 0x00317e98, 0x002a9854)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                break 'b_2a9858;
            }
        }
        'b_2a986c: {
            c.cycles += 2;
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                break 'b_2a986c;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002a9868);
                call_ptr(c, 0x00317e98, 0x002a9868)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                break 'b_2a986c;
            }
        }
        'b_2a9880: {
            c.cycles += 2;
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                break 'b_2a9880;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002a987c);
                call_ptr(c, 0x00317e98, 0x002a987c)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                break 'b_2a9880;
            }
        }
        'b_2a9894: {
            c.cycles += 2;
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
                break 'b_2a9894;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002a9890);
                call_ptr(c, 0x00317e98, 0x002a9890)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
                break 'b_2a9894;
            }
        }
        'b_2a98a8: {
            c.cycles += 2;
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                break 'b_2a98a8;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002a98a4);
                call_ptr(c, 0x00317e98, 0x002a98a4)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                break 'b_2a98a8;
            }
        }
        'b_2a98bc: {
            c.cycles += 2;
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                break 'b_2a98bc;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002a98b8);
                call_ptr(c, 0x00317e98, 0x002a98b8)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                break 'b_2a98bc;
            }
        }
        'b_2a98d0: {
            c.cycles += 2;
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                break 'b_2a98d0;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002a98cc);
                call_ptr(c, 0x00317e98, 0x002a98cc)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                break 'b_2a98d0;
            }
        }
        'b_2a98e4: {
            c.cycles += 2;
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                break 'b_2a98e4;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002a98e0);
                call_ptr(c, 0x00317e98, 0x002a98e0)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                break 'b_2a98e4;
            }
        }
        'b_2a98f8: {
            c.cycles += 2;
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                break 'b_2a98f8;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002a98f4);
                call_ptr(c, 0x00317e98, 0x002a98f4)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                break 'b_2a98f8;
            }
        }
        'b_2a990c: {
            c.cycles += 2;
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                break 'b_2a990c;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002a9908);
                call_ptr(c, 0x00317e98, 0x002a9908)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                break 'b_2a990c;
            }
        }
        'b_2a9920: {
            c.cycles += 2;
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                break 'b_2a9920;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002a991c);
                call_ptr(c, 0x00317e98, 0x002a991c)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                break 'b_2a9920;
            }
        }
        'b_2a9934: {
            c.cycles += 2;
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                break 'b_2a9934;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002a9930);
                call_ptr(c, 0x00317e98, 0x002a9930)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                break 'b_2a9934;
            }
        }
        'b_2a9948: {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;
            c.g[V0][0] = c.g[S1][0] & 0x2;
            if t {
                break 'b_2a9948;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002a9944);
                call_ptr(c, 0x00317e98, 0x002a9944)?;
                c.g[V0][0] = c.g[S1][0] & 0x2;
                break 'b_2a9948;
            }
        }
        'b_2a9960: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = c.g[S1][0] & 0x1;
            if t {
                break 'b_2a9960;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                c.g[RA][0] = sx(0x002a995c);
                c.g[A1][0] = 0u64;
                Callee {
                    addr: 0x002adee8,
                    f: ee::s08::SoundManagerRef_dtor,
                }
                .call(c, 0x002a995c)?;
                c.g[V0][0] = c.g[S1][0] & 0x1;
                break 'b_2a9960;
            }
        }
        'b_2a9974: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_2a9974;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002a9970);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00317e50, 0x002a9970)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_2a9974;
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `StreamBuffer* StreamBuffer_ctor_2(StreamBuffer *self, SsPlayer *p, char *mem,
/// int size, int n, int m)` (0x002a9c00): fills the stream fields from the
/// player's tables (float curve values for the volume and pitch), then creates the
/// SND stream. Converted from the generated labelled-block body: every register,
/// cycle and call kept.
pub fn stream_buffer_ctor_2(c: &mut Cpu) {
    if c.pc != 0x002a9c00 {
        return ee::s08::f_002a9c00__generated(c);
    }
    guest(c, |c| {
        'b_2a9d8c: {
            c.cycles += 21;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.g[T2][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[T3][0] = c.g[A2][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[T4][0] = c.g[A3][0];
            c.g[A2][0] = c.g[T1][0];
            c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
            c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x3c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[A1][0] = c.lw((c.g[V0][0] as u32));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
            c.g[A0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x38));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
            let t = c.g[A1][0] == 0u64;
            c.sb((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            if t {
                break 'b_2a9d8c;
            } else {
                'b_2a9d14: {
                    'b_2a9d10: {
                        'b_2a9cc0: {
                            c.cycles += 16;
                            c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                            c.g[A0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x38));
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.lwc1(2, (c.g[A1][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[AT][0] = 0x42fe0000;
                            c.mtc1(1, 3);
                            c.lwc1(0, (c.g[V0][0] as u32));
                            c.cvt_s_w(0, 0);
                            c.mul_s(0, 0, 2);
                            c.c_lt_s(0, 3);
                            let t = !c.fcc();
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            if t {
                                break 'b_2a9cc0;
                            } else {
                                c.cycles += 11;
                                c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                                c.mtc1(0, 1);
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                c.lwc1(0, (c.g[V0][0] as u32));
                                c.cvt_s_w(0, 0);
                                c.mul_s(0, 0, 2);
                                c.c_le_s(1, 0);
                                let t = !c.fcc();
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_2a9d10;
                                } else {
                                    break 'b_2a9cc0;
                                }
                            }
                        }
                        c.cycles += 10;
                        c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.lwc1(0, (c.g[V0][0] as u32));
                        c.cvt_s_w(0, 0);
                        c.mul_s(0, 0, 2);
                        c.c_lt_s(0, 3);
                        let t = !c.fcc();
                        c.g[V0][0] = 0x7f;
                        if t {
                            break 'b_2a9d10;
                        } else {
                            c.cycles += 10;
                            c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.lwc1(0, (c.g[V0][0] as u32));
                            c.cvt_s_w(0, 0);
                            c.mul_s(0, 0, 2);
                            c.cvt_w_s(1, 0);
                            c.mfc1(2, 1);
                            let t = true;
                            c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                            if t {
                                break 'b_2a9d14;
                            } else {
                                break 'b_2a9d10;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                    break 'b_2a9d14;
                }
                'b_2a9d88: {
                    'b_2a9d5c: {
                        c.cycles += 12;
                        c.g[V0][0] = 0x18;
                        c.g[A0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                        c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                        c.g[A3][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x38));
                        c.mult(0, 4, 4, 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[A1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x20));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                        if t {
                            break 'b_2a9d5c;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                            let t = (c.g[V1][0] as i64) < 0;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_2a9d88;
                            } else {
                                break 'b_2a9d5c;
                            }
                        }
                    }
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
                    if c.g[V1][0] == 0u64 {
                        c.g[V0][0] = 0x7f;
                        break 'b_2a9d88;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                        c.g[V0][0] = c.lbu((c.g[V0][0] as u32));
                        break 'b_2a9d88;
                    }
                }
                c.cycles += 1;
                c.sb((c.g[A0][0] as u32), c.g[V0][0]);
                break 'b_2a9d8c;
            }
        }
        c.cycles += 18;
        c.g[V0][0] = 0x1;
        c.sw((c.g[S0][0] as u32), c.g[T3][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[V1][0] = 0x18;
        c.g[A1][0] = c.g[T0][0];
        c.g[A3][0] = c.g[T3][0];
        c.g[A0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x1c));
        c.g[T0][0] = c.g[T4][0];
        c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x20));
        c.mult(0, 4, 4, 3);
        c.g[RA][0] = sx(0x002a9dbc);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, 0x003b7818, 0x002a9dbc)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, Rng, DATA};
    use crate::game::lib::snd::tests::snd_world;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of this module's fakes: a word in the machine's RAM, written by
    /// the fixtures that want the fakes to answer (else the previous body runs).
    const OC1036_CTRL: u32 = DATA + 0x2e_8000;
    const OC1036_MAGIC: u32 = 0x1036_c0de;

    static OC1036_PREV_FREE: AtomicUsize = AtomicUsize::new(0);
    static OC1036_PREV_DELETE: AtomicUsize = AtomicUsize::new(0);
    static OC1036_PREV_SNDSTRM_CREATE: AtomicUsize = AtomicUsize::new(0);

    /// `EA_MemFree` (0x317e98): records the call and returns.
    fn oc1036_fake_free(c: &mut Cpu) {
        if c.bus.read32(OC1036_CTRL) != OC1036_MAGIC {
            return fakes::fall_back(c, &OC1036_PREV_FREE, ee::s10::f_00317e98__generated);
        }
        fakes::fake_call(c, 0x0031_7e98);
    }

    /// `__builtin_delete` (0x317e50): records the call and returns.
    fn oc1036_fake_delete(c: &mut Cpu) {
        if c.bus.read32(OC1036_CTRL) != OC1036_MAGIC {
            return fakes::fall_back(c, &OC1036_PREV_DELETE, ee::s10::f_00317e50__generated);
        }
        fakes::fake_call(c, 0x0031_7e50);
    }

    /// `SNDSTRM_Create` (0x3b7818): records the call and returns.
    fn oc1036_fake_sndstrm_create(c: &mut Cpu) {
        if c.bus.read32(OC1036_CTRL) != OC1036_MAGIC {
            return fakes::fall_back(
                c,
                &OC1036_PREV_SNDSTRM_CREATE,
                ee::s13::f_003b7818__generated,
            );
        }
        fakes::fake_call(c, 0x003b_7818);
    }

    /// Install the gated fakes (once per process).
    fn oc1036_install() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1036_fake_free,
            &OC1036_PREV_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1036_fake_delete,
            &OC1036_PREV_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B7818),
            oc1036_fake_sndstrm_create,
            &OC1036_PREV_SNDSTRM_CREATE,
        );
    }

    fn oc1036_gate_on(c: &mut Cpu) {
        c.bus.write32(OC1036_CTRL, OC1036_MAGIC);
    }

    /// The stream object of the status readers, its stream slot and request.
    const SELF: u32 = DATA + 0x20_0000;
    const STREAM: u32 = SELF + 0x1000;
    const REQ: u32 = SELF + 0x2000;

    /// The status readers' world (as the GetStatus / GetRequestStatus tests):
    /// SND lock hooks, one stream slot whose request list holds `REQ`. The
    /// request is playing on even seeds (state byte 2), else another state.
    fn stream_world(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
        c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
        c.bus.write8(0x0051_0000 - 0x56d2, 1);
        c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
        let id = if seed % 3 == 0 {
            1 + r.next_u32() % 4
        } else {
            0
        };
        c.bus.write32(SELF + 0x8, id);
        c.bus.write32(
            STREAM + 0x124,
            if seed % 5 == 1 { 0 } else { r.next_u32() | 1 },
        );
        c.bus.write32(STREAM + 0x11c, REQ);
        c.bus.write32(REQ, 0);
        c.bus.write32(
            REQ + 0xc,
            if seed % 4 < 2 {
                0x8000_0000 | r.next_u32()
            } else {
                r.next_u32() & 0x7fff_ffff
            },
        );
        c.bus.write8(
            REQ + 0x24,
            if seed % 2 == 0 {
                2
            } else {
                1 + (r.next_u32() % 4) as u8
            },
        );
        c.bus
            .write16(STREAM + 0x14, 1 + (r.next_u32() % 48000) as u16);
        c.bus
            .write16(STREAM + 0x18, 1 + (r.next_u32() % 48000) as u16);
        let done = r.next_u32() & 0xf_ffff;
        c.bus.write32(REQ + 0x14, done);
        c.bus.write32(REQ + 0x18, done + (r.next_u32() & 0xf_ffff));
        c.bus.write32(REQ + 0x1c, r.next_u32() & 0xffff);
        c.bus
            .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
    }

    #[test]
    fn stream_buffer_get_request_position_matches() {
        check(
            ee::s08::f_002aa0c0__generated,
            stream_buffer_get_request_position,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a0c0, 4);
                c.pc = 0x002a_a0c0;
                c.g[A0][0] = sx(SELF);
                stream_world(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn stream_buffer_has_buffered_matches() {
        check(
            ee::s08::f_002aa150__generated,
            stream_buffer_has_buffered,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a150, 4);
                c.pc = 0x002a_a150;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(if seed % 2 == 0 {
                    r.next_u32() % 4
                } else {
                    r.next_u32()
                });
                stream_world(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn stream_buffer_is_request_pending_matches() {
        check(
            ee::s08::f_002aa188__generated,
            stream_buffer_is_request_pending,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a188, 4);
                c.pc = 0x002a_a188;
                c.g[A0][0] = sx(SELF);
                stream_world(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn stream_buffer_clear_prebuffer_matches() {
        oc1036_install();
        check(
            ee::s08::f_002aa1e8__generated,
            stream_buffer_clear_prebuffer,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a1e8, 4);
                oc1036_gate_on(c);
                c.pc = 0x002a_a1e8;
                c.g[A0][0] = sx(SELF);
                stream_world(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn stream_buffer_clear_prebuffer_handle_matches() {
        oc1036_install();
        check(
            ee::s08::f_002aa240__generated,
            stream_buffer_clear_prebuffer_handle,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a240, 2);
                oc1036_gate_on(c);
                c.pc = 0x002a_a240;
                snd_world(c, &mut r, seed % 2 == 0);
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { sx(r.next_u32()) };
            },
        );
    }

    #[test]
    fn ps2_sound_manager_alloc_spu_block_matches() {
        const SM: u32 = SELF;
        check(
            ee::s08::f_002a8fd8__generated,
            ps2_sound_manager_alloc_spu_block,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_8fd8, 3);
                c.pc = 0x002a_8fd8;
                c.g[A0][0] = sx(SM);
                c.g[A1][0] = if seed % 9 == 0 {
                    u64::MAX
                } else {
                    sx(r.next_u32() % 64)
                };
                // Head of the in-use list: negative (none) or a block index.
                c.bus.write32(
                    SM + 0x318,
                    if seed % 5 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 4
                    },
                );
                c.bus.write32(SM + 0x30c, r.next_u32());
                c.bus.write32(SM + 0x310, 0);
                c.bus.write32(SM + 0x31c, if seed % 7 == 0 { 1 } else { 0 });
                // Four 0x18-byte blocks: size at +0xc, next index at +0x14.
                for i in 0..4u32 {
                    let block = SM + 0xc + i * 0x18;
                    c.bus.write32(block + 0xc, r.next_u32() % 64);
                    let next = if i < 3 && seed % 3 != 0 {
                        i + 1
                    } else {
                        u32::MAX
                    };
                    c.bus.write32(block + 0x14, next);
                }
            },
        );
    }

    #[test]
    fn snd_play_params_dtor_ported_matches() {
        const OBJ: u32 = SELF;
        const FIRST: u32 = DATA + 0x21_0000;
        const MEMBER: u32 = DATA + 0x22_0000;
        oc1036_install();
        check(
            ee::s08::f_002a97c0__generated,
            snd_play_params_dtor_ported,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_97c0, 3);
                oc1036_gate_on(c);
                c.pc = 0x002a_97c0;
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = (seed % 8) as u64;
                c.bus.write32(OBJ, FIRST);
                // The member pointers: null or a live-looking block.
                for off in [
                    0x20, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80,
                    0x84,
                ] {
                    let v = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        MEMBER + off
                    };
                    c.bus.write32(OBJ + off, v);
                }
            },
        );
    }

    #[test]
    fn stream_buffer_ctor_2_matches() {
        const P: u32 = SELF + 0x4000;
        const TABLE_A: u32 = SELF + 0x5000;
        const TABLE_B: u32 = SELF + 0x5100;
        oc1036_install();
        check(
            ee::s08::f_002a9c00__generated,
            stream_buffer_ctor_2,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_9c00, 3);
                oc1036_gate_on(c);
                c.pc = 0x002a_9c00;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(P);
                c.g[A2][0] = sx(SELF + 0x6000);
                c.g[A3][0] = sx(r.next_u32() % 4096);
                c.g[T0][0] = sx(r.next_u32() % 8);
                c.g[T1][0] = sx(r.next_u32() % 8);
                c.bus.write32(P + 0x1c, r.next_u32() % 16);
                c.bus.write32(P + 0x20, r.next_u32() % 16);
                c.bus.write32(P + 0x38, TABLE_B);
                c.bus.write32(P + 0x3c, TABLE_A);
                for k in 0..64u32 {
                    let a = if k % 5 == 0 {
                        u32::MAX - r.next_u32() % 5
                    } else {
                        r.next_u32() % 300
                    };
                    c.bus.write32(TABLE_A + k * 4, a);
                    c.bus.write32(TABLE_B + k * 4, r.next_u32() % 300);
                }
            },
        );
    }
}
