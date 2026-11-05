//! Staging wave 205 (oc241): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_7a70, sndstrm_find_request, "SNDSTRM_FindRequest"),
    (0x0031_30a8, anim_data_base_decode_chan_cache, "AnimDataBase_DecodeChanCache"),
    (0x003d_4b00, path_select_master_track, "PATH_SelectMasterTrack"),
    (0x003c_7828, snd_mix_add_ramp16, "SND_MixAddRamp16"),
    (0x0037_1940, dma_send_gif_normal, "Dma_SendGifNormal"),
    (0x0036_66f8, tex_man_reset_usage_table, "TexMan_ResetUsageTable"),
    (0x0026_c5f8, playback_man_poll_file_load, "PlaybackMan_PollFileLoad"),
    (0x0029_d290, ssx_audio_system_update_race_loop_sfx, "SSXAudioSystem_UpdateRaceLoopSfx"),
];

// ---------------------------------------------------------------------------
// SNDSTRM_FindRequest (0x3b7a70)

const SNDSTRM_GET_STREAM: Callee = Callee {
    addr: 0x003b_6a38,
    f: ee::s13::SNDSTRM_GetStream,
};

/// Stream: head of its request list.
const STREAM_REQUESTS: u32 = 0x11c;
/// Request: next request in the list (+0) and its handle (+0xc).
const REQUEST_NEXT: u32 = 0x0;
const REQUEST_HANDLE: u32 = 0xc;

/// `SndStrmRequest* SNDSTRM_FindRequest(int handle)` (0x3b7a70): the request
/// with this handle in the request list of its stream (`handle & 0xff`,
/// `SNDSTRM_GetStream`), or 0 for a negative handle, a missing stream or an
/// unknown request. The handle stays in `s0` (64-bit compare with the
/// sign-extended handle words).
pub fn sndstrm_find_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let handle = c.g[S0][0];
        'done: {
            'none: {
                if (handle as i64) < 0 {
                    break 'none;
                }
                c.cycles += 4;
                c.g[A0][0] = handle & 0xff;
                SNDSTRM_GET_STREAM.call(c, 0x003b_7a8c)?;
                let stream = r32(c, V0);
                if stream == 0 {
                    break 'none;
                }
                c.g[V1][0] = Addr(stream).off(STREAM_REQUESTS).lw(c);
                loop {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V1][0];
                    if c.g[V1][0] == 0 {
                        break 'done;
                    }
                    c.cycles += 3;
                    let req = r32(c, V1);
                    c.g[V0][0] = Addr(req).off(REQUEST_HANDLE).lw(c);
                    if c.g[V0][0] == c.g[S0][0] {
                        c.cycles += 1;
                        c.g[V0][0] = c.g[V1][0];
                        break 'done;
                    }
                    c.g[V1][0] = Addr(req).off(REQUEST_NEXT).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003b_7aa0);
                    }
                }
            }
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
// AnimDataBase_DecodeChanCache (0x3130a8)

/// Anim database: channel array (0x14 bytes per channel), data pointers
/// (+4..+0x10) and the per-type channel counters (+0x14..+0x17, types 6-9).
const ADB_CHANNELS: u32 = 0x0;
const ADB_DATA4: u32 = 0x4;
const ADB_DATA8: u32 = 0x8;
const ADB_DATA_C: u32 = 0xc;
const ADB_DATA10: u32 = 0x10;
const ADB_COUNT6: u32 = 0x14;
const ADB_COUNT7: u32 = 0x15;
const ADB_COUNT8: u32 = 0x16;
const ADB_COUNT9: u32 = 0x17;
/// Channel: type (+0), constants (+4..+0x10), index among its type (+0xc).
const CHAN_SIZE: u32 = 0x14;
const CHAN_TYPE: u32 = 0x0;
const CHAN_TYPE_INDEX: u32 = 0xc;
/// The `switch (type - 1)` jump table (in `.rodata`) and its case labels.
const DECODE_JUMP_TABLE: u32 = 0x0048_a590;
const CASE_ONE: u32 = 0x0031_3240;
const CASE_THREE: u32 = 0x0031_3270;
const CASE_FOUR: u32 = 0x0031_32f0;
const CASE_FIVE: u32 = 0x0031_3398;
const CASE_TWO: u32 = 0x0031_3434;

/// One channel-type nibble (in `a0`) for channel `a1`: types 6-9 take the
/// next index of their type (counter `self+0x14+type-6`, `t0` = self) into
/// `chan+0xc`; every type is stored at `chan+0`. Registers v0/v1 as the
/// original's compare chain against t3..a2 (6..9) leaves them.
fn decode_chan_type(c: &mut Cpu) {
    let this = r32(c, T0);
    let chan = r32(c, A1);
    let ty = c.g[A0][0];
    let counter = if ty == c.g[T3][0] {
        c.cycles += 5;
        Some(ADB_COUNT6)
    } else if {
        c.cycles += 2;
        ty == c.g[T2][0]
    } {
        c.cycles += 5;
        Some(ADB_COUNT7)
    } else if {
        c.cycles += 2;
        ty == c.g[T1][0]
    } {
        c.cycles += 5;
        Some(ADB_COUNT8)
    } else if {
        c.cycles += 2;
        ty == c.g[A2][0]
    } {
        c.cycles += 4;
        Some(ADB_COUNT9)
    } else {
        None
    };
    if let Some(off) = counter {
        let count = Addr(this).off(off).lbu(c);
        c.g[V0][0] = count;
        c.g[V1][0] = sx(1u32.wrapping_add(count as u32));
        Addr(this).off(off).sb(c, c.g[V1][0]);
        c.cycles += 2;
        Addr(chan).off(CHAN_TYPE_INDEX).sw(c, c.g[V0][0]);
    }
    Addr(chan).off(CHAN_TYPE).sw(c, ty);
}

/// The 24-bit constant at `a3+k` (bytes b0 b1 b2) as the float bits
/// `b0<<8 | b1<<16 | b2<<24`, stored from f0 at `a1+dst`; v1/v0/a0 keep the
/// assembled word and its parts.
fn decode_word24(c: &mut Cpu, k: u32, dst: u32) {
    let p = r32(c, A3);
    let b0 = Addr(p).off(k).lbu(c) as u32;
    let b1 = Addr(p).off(k + 1).lbu(c) as u32;
    let b2 = Addr(p).off(k + 2).lbu(c) as u32;
    c.g[V0][0] = sx(b1 << 16);
    c.g[A0][0] = sx(b2 << 24);
    c.g[V1][0] = sx(b0 << 8) | c.g[V0][0] | c.g[A0][0];
    c.mtc1(V1, 0);
    let chan = r32(c, A1);
    c.swc1(0, chan.wrapping_add(dst));
}

/// `void AnimDataBase_DecodeChanCache(AnimDataBase* self, u8* data, int frames)`
/// (0x3130a8): decodes a channel cache header. `data[0]` is the channel
/// count n (0: nothing to do); then one type nibble per channel (high
/// nibble first) into `chan+0` (types 6-9 also count their channels:
/// `self+0x14..0x17`, the index into `chan+0xc`); then per channel its
/// constants as 24-bit floats (type 1: one, 2/6/7/8: two, 3: three, 4:
/// four, 5: three and a 16-bit integer). Finally the four data stream
/// pointers `self+4..0x10` after the header, sized from `frames` and the
/// type counts (16-bit and 32-bit alignment padding).
///
/// Register-level port (leaf): t0 = self, t7 = data, a3 = cursor, t5 =
/// frames, a1 = channel, t4 = end of channels, t3/t2/t1/a2 = 6/7/8/9,
/// LO/HI from the products.
pub fn anim_data_base_decode_chan_cache(c: &mut Cpu) {
    c.cycles += 7;
    c.g[T7][0] = c.g[A1][0];
    c.g[T0][0] = c.g[A0][0];
    c.g[A3][0] = c.g[T7][0];
    c.g[T5][0] = c.g[A2][0];
    c.g[V1][0] = Addr(r32(c, A3)).lbu(c);
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
    if c.g[V1][0] == 0 {
        c.cycles += 2;
        return ret(c);
    }
    let this = r32(c, T0);
    c.cycles += 13;
    c.g[V0][0] = CHAN_SIZE as u64;
    c.g[A1][0] = Addr(this).off(ADB_CHANNELS).lw(c);
    c.mult(0, V0, V1, V0);
    for off in [ADB_COUNT6, ADB_COUNT7, ADB_COUNT8] {
        Addr(this).off(off).sb(c, 0);
    }
    c.g[T6][0] = sx(r32(c, T5).wrapping_sub(2));
    c.g[T3][0] = 6;
    Addr(this).off(ADB_COUNT9).sb(c, 0);
    c.g[T2][0] = 7;
    c.g[T4][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
    c.g[T1][0] = 8;
    c.g[A2][0] = 9;

    // The type nibbles, two per byte.
    loop {
        c.cycles += 4;
        c.g[V0][0] = Addr(r32(c, A3)).lbu(c);
        c.g[A0][0] = sx((c.g[V0][0] as u32) >> 4);
        decode_chan_type(c);
        c.cycles += 3;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(CHAN_SIZE));
        if c.g[A1][0] == c.g[T4][0] {
            break;
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(r32(c, A3)).lbu(c);
        c.g[A0][0] = c.g[V0][0] & 0xf;
        decode_chan_type(c);
        c.cycles += 3;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(CHAN_SIZE));
        if c.g[A1][0] == c.g[T4][0] {
            break;
        }
        c.cycles += 2;
        c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
        if event_due(c) {
            deliver_event_here(c, 0x0031_30f8);
        }
    }
    c.g[A1][0] = Addr(this).off(ADB_CHANNELS).lw(c);

    // The constants of each channel.
    c.cycles += 2;
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
    c.g[V0][0] = Addr(r32(c, A1)).off(CHAN_TYPE).lw(c);
    loop {
        c.cycles += 4;
        c.g[A0][0] = sx(r32(c, V0).wrapping_sub(1));
        c.g[V1][0] = (c.g[A0][0] < 8) as u64;
        c.g[V0][0] = 0x0049_0000;
        if c.g[V1][0] != 0 {
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, A0) << 2);
            c.g[V0][0] = sx(DECODE_JUMP_TABLE);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(DECODE_JUMP_TABLE));
            c.g[A0][0] = Addr(r32(c, V1)).lw(c);
            let case = r32(c, A0);
            let size = match case {
                CASE_ONE => {
                    c.cycles += 12;
                    decode_word24(c, 0, 4);
                    3
                }
                CASE_THREE => {
                    c.cycles += 32;
                    for i in 0..3 {
                        decode_word24(c, 3 * i, 4 + 4 * i);
                    }
                    9
                }
                CASE_FOUR => {
                    c.cycles += 42;
                    for i in 0..4 {
                        decode_word24(c, 3 * i, 4 + 4 * i);
                    }
                    12
                }
                CASE_FIVE => {
                    c.cycles += 39;
                    for i in 0..3 {
                        decode_word24(c, 3 * i, 4 + 4 * i);
                    }
                    // A 16-bit integer converted to float.
                    let p = Addr(r32(c, A3));
                    c.g[V0][0] = p.off(0xa).lbu(c);
                    c.g[V1][0] = p.off(0x9).lbu(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                    c.g[V1][0] |= c.g[V0][0];
                    c.mtc1(V1, 0);
                    c.cvt_s_w(0, 0);
                    let chan = r32(c, A1);
                    c.swc1(0, chan.wrapping_add(0x10));
                    11
                }
                CASE_TWO => {
                    c.cycles += 21;
                    for i in 0..2 {
                        decode_word24(c, 3 * i, 4 + 4 * i);
                    }
                    6
                }
                t => {
                    c.jump_out(t, 0x0031_30a8, 0x0031_3508);
                    return;
                }
            };
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(size));
        }
        c.cycles += 3;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(CHAN_SIZE));
        if c.g[A1][0] == c.g[T4][0] {
            break;
        }
        c.g[V0][0] = Addr(r32(c, A1)).off(CHAN_TYPE).lw(c);
        if event_due(c) {
            deliver_event_here(c, 0x0031_3218);
        }
    }

    // The data streams after the header: type-6 channels (frames bytes
    // each), type-7 ((frames-2)/2+2 each), 16-bit aligned type-8 (2*frames
    // each), then 32-bit aligned.
    c.cycles += 27;
    let frames_less_2 = r32(c, T6);
    c.g[V0][0] = sx(frames_less_2 >> 31);
    c.g[A1][0] = Addr(this).off(ADB_COUNT6).lbu(c);
    c.g[V0][0] = sx(frames_less_2.wrapping_add(r32(c, V0)));
    c.g[A2][0] = Addr(this).off(ADB_COUNT7).lbu(c);
    c.g[V0][0] = sx(((r32(c, V0) as i32) >> 1) as u32);
    c.mult(0, A1, T5, A1);
    c.g[A0][0] = Addr(this).off(ADB_COUNT8).lbu(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(2));
    c.mult(0, V0, V0, A2);
    c.g[V1][0] = sx(r32(c, A3).wrapping_sub(r32(c, T7)));
    c.mult(0, A0, T5, A0);
    let cursor = r32(c, A3);
    Addr(this).off(ADB_DATA4).sw(c, c.g[A3][0]);
    let (size6, size7, size8) = (r32(c, A1), r32(c, V0), r32(c, A0));
    let ofs = r32(c, V1).wrapping_add(size6).wrapping_add(size7);
    let data8 = cursor.wrapping_add(size6);
    Addr(this).off(ADB_DATA8).sw(c, sx(data8));
    let pad = ofs & 1;
    let size8x2 = size8 << 1;
    let ofs2 = ofs.wrapping_add(size8x2).wrapping_add(pad) & 3;
    let data_c = data8.wrapping_add(size7).wrapping_add(pad);
    let data10 = data8.wrapping_add(size7).wrapping_add(size8x2).wrapping_add(ofs2);
    c.g[A1][0] = sx(data8);
    c.g[A2][0] = pad as u64;
    c.g[V1][0] = ofs2 as u64;
    c.g[V0][0] = sx(data_c);
    c.g[A0][0] = sx(data10);
    Addr(this).off(ADB_DATA_C).sw(c, c.g[V0][0]);
    Addr(this).off(ADB_DATA10).sw(c, c.g[A0][0]);
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// PATH_SelectMasterTrack (0x3d4b00)

/// The PATH module's state pointer (`0x44f420`): the master track selection
/// (+0 index, +4 length, +8 kind) and 24 track pointers from +0x58.
const PATH_STATE: u32 = 0x0044_f420;
const PATH_MASTER_INDEX: u32 = 0x0;
const PATH_MASTER_LENGTH: u32 = 0x4;
const PATH_MASTER_KIND: u32 = 0x8;
const PATH_TRACKS: u32 = 0x58;
const PATH_TRACK_COUNT: i64 = 24;
/// Track: disabled byte (+3), length (+0x24), header pointer (+0x30, its
/// first word holds the kind in bits 21-26), signed halfword (+0x36,
/// negative: not a candidate).
const TRACK_DISABLED: u32 = 0x3;
const TRACK_LENGTH: u32 = 0x24;
const TRACK_HEADER: u32 = 0x30;
const TRACK_FLAGS: u32 = 0x36;

/// `void PATH_SelectMasterTrack(void)` (0x3d4b00): picks the longest of the
/// 24 PATH tracks that is present, enabled (`+3 == 0`), has a header and a
/// non-negative `+0x36`: index, length (unsigned compare, strictly longer
/// wins) and kind (`header[0] >> 21 & 0x3f`) into the state's master
/// selection (reset to -1/0/0 first).
///
/// Registers as the original: a1 = state, a2 = track slot cursor, a3 =
/// index, t0 = 1, a0/v0/v1 scratch.
pub fn path_select_master_track(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0x0045_0000;
    c.g[A0][0] = u64::MAX;
    c.g[V1][0] = Addr(PATH_STATE).lw(c);
    c.g[A3][0] = 0;
    c.g[T0][0] = 1;
    c.g[A1][0] = c.g[V1][0];
    let state = Addr(r32(c, A1));
    state.off(PATH_MASTER_INDEX).sw(c, u64::MAX);
    state.off(PATH_MASTER_LENGTH).sw(c, 0);
    c.g[A2][0] = sx(state.0.wrapping_add(PATH_TRACKS));
    state.off(PATH_MASTER_KIND).sw(c, 0);
    c.g[A0][0] = Addr(r32(c, A2)).lw(c);
    loop {
        c.cycles += 2;
        let track = r32(c, A0);
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
        // Through the shared `a3++` with its extra cycle, or straight to the
        // loop test (the original's two increments).
        let mut tail_cycle = true;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(track).off(TRACK_DISABLED).lbu(c);
            c.g[V1][0] = 0;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(track).off(TRACK_FLAGS).lh(c);
                if (c.g[V0][0] as i64) >= 0 {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(track).off(TRACK_HEADER).lw(c);
                    c.g[V1][0] = if c.g[V0][0] == 0 { 0 } else { c.g[T0][0] };
                }
                c.cycles += 2;
                if c.g[V1][0] == 0 {
                    tail_cycle = false;
                } else {
                    c.cycles += 5;
                    c.g[V1][0] = Addr(track).off(TRACK_LENGTH).lw(c);
                    c.g[V0][0] = state.off(PATH_MASTER_LENGTH).lw(c);
                    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    if c.g[V0][0] == 0 {
                        tail_cycle = false;
                    } else {
                        // The longest so far.
                        c.cycles += 7;
                        state.off(PATH_MASTER_LENGTH).sw(c, c.g[V1][0]);
                        state.off(PATH_MASTER_INDEX).sw(c, c.g[A3][0]);
                        c.g[V1][0] = Addr(track).off(TRACK_HEADER).lw(c);
                        c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                        c.g[V0][0] = sx(r32(c, V0) >> 21) & 0x3f;
                        state.off(PATH_MASTER_KIND).sw(c, c.g[V0][0]);
                    }
                }
            }
        }
        if tail_cycle {
            c.cycles += 1;
        }
        c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A3][0] as i64) < PATH_TRACK_COUNT) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[A0][0] = Addr(r32(c, A2)).lw(c);
        if event_due(c) {
            deliver_event_here(c, 0x003d_4b30);
        }
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// SND_MixAddRamp16 (0x3c7828)

/// `1/17` (0x3d70f0f1): the ramp step over a 16-sample block.
const RAMP16_STEP_SCALE: FpuF32 = FpuF32(0x3d70_f0f1);

/// `void SND_MixAddRamp16(const float* src, float* dst, float g0, float g1)`
/// (0x3c7828): mixes 16 samples into `dst` with a linear gain ramp,
/// `dst[i] += src[i] * (g0 + step * (i + 1))`, `step = (g1 - g0) / 17`
/// (gain 2 as `step + step`, gains 3..16 as `step * k`).
///
/// The original keeps the gains and partly mixed samples in f1..f25 (f20..f25
/// saved on a 0x30 stack frame) and interleaves the loads and stores; the
/// memory order is kept (`dst[0..12]` are read before the first stores, each
/// `src[k]` after `dst[k-1]` is written), and so are the registers it leaves.
pub fn snd_mix_add_ramp16(c: &mut Cpu) {
    c.cycles += 159;
    let src = Addr(abi::arg(c, 0));
    let dst = Addr(abi::arg(c, 1));
    let g0 = FpuF32(c.f[12]);
    let step = (FpuF32(c.f[13]) - g0) * RAMP16_STEP_SCALE;
    let k = |n: u32| FpuF32((n as f32).to_bits());
    let mut gain = [FpuF32::ZERO; 17];
    gain[1] = g0 + step;
    gain[2] = g0 + (step + step);
    for n in 3..=16 {
        gain[n as usize] = g0 + step * k(n);
    }
    // dst[i] = d + src[i] * gain; returns (new dst[i], the product).
    let mix = |c: &mut Cpu, d: FpuF32, i: u32| -> (FpuF32, FpuF32) {
        let product = src.off(4 * i).read_fpu(c) * gain[i as usize + 1];
        let out = d + product;
        dst.off(4 * i).write_fpu(c, out);
        (out, product)
    };

    let s0 = src.read_fpu(c);
    let d0 = dst.read_fpu(c);
    // Save f20..f25 (restored below).
    let frame = Frame::open(c, 0x30);
    for (i, f) in (20..=25).enumerate() {
        frame.save_fpr(c, f, 8 * i as u32);
    }
    let mut d = [FpuF32::ZERO; 16];
    d[0] = d0;
    for i in 1..4 {
        d[i] = dst.off(4 * i as u32).read_fpu(c);
    }
    let out0 = d0 + s0 * gain[1];
    dst.write_fpu(c, out0);
    for i in 4..8 {
        d[i] = dst.off(4 * i as u32).read_fpu(c);
    }
    let (out1, _) = mix(c, d[1], 1);
    for i in 8..12 {
        d[i] = dst.off(4 * i as u32).read_fpu(c);
    }
    let mut out = [FpuF32::ZERO; 16];
    out[0] = out0;
    out[1] = out1;
    for i in 2..12 {
        out[i] = mix(c, d[i], i as u32).0;
    }
    for (i, f) in (20..=25).enumerate() {
        frame.restore_fpr(c, f, 8 * i as u32);
    }
    // The last four: src[12] first, then dst[12..16].
    let s12 = src.off(0x30).read_fpu(c);
    for i in 12..16 {
        d[i] = dst.off(4 * i as u32).read_fpu(c);
    }
    out[12] = d[12] + s12 * gain[13];
    dst.off(0x30).write_fpu(c, out[12]);
    let mut last_product = FpuF32::ZERO;
    for i in 13..16 {
        (out[i], last_product) = mix(c, d[i], i as u32);
    }

    // Registers as the original leaves them.
    c.g[AT][0] = 0x4180_0000;
    let set = |c: &mut Cpu, f: usize, v: FpuF32| c.f[f] = v.0;
    set(c, 0, last_product);
    set(c, 1, out[12]);
    set(c, 2, out[13]);
    set(c, 3, out[14]);
    set(c, 4, out[15]);
    set(c, 5, gain[4]);
    set(c, 6, gain[11]);
    set(c, 7, gain[12]);
    set(c, 8, gain[14]);
    set(c, 9, gain[5]);
    set(c, 10, gain[6]);
    set(c, 11, gain[7]);
    set(c, 12, gain[16]);
    set(c, 13, step * k(16));
    set(c, 14, gain[8]);
    set(c, 15, gain[15]);
    set(c, 16, out[2]);
    set(c, 17, out[11]);
    set(c, 18, out[8]);
    set(c, 19, out[9]);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Dma_SendGifNormal (0x371940)

/// DMA channel 2 (GIF) registers and `CHCR.STR`.
const GIF_CHCR: u32 = 0x1000_a000;
const GIF_MADR: u32 = 0x1000_a010;
const GIF_QWC: u32 = 0x1000_a020;
const DMA_CHCR_STR: u64 = 0x100;
/// The VIF1 FIFO and `GIF_MODE`.
const VIF1_FIFO: u32 = 0x1000_5000;
const GIF_MODE: u32 = 0x1000_3010;
/// The quadword pushed into the VIF1 FIFO first (a `MSKPATH3` VIF code).
const GIF_SEND_VIF_PREFIX: u32 = 0x0044_b430;
/// `flags`: wait until the channel is idle before / after the transfer.
const GIF_WAIT_BEFORE: u64 = 4;
const GIF_WAIT_AFTER: u64 = 2;

/// Spin while the GIF channel is busy (v1 = CHCR, v0 = the busy bit); the
/// poll goes through `Cpu::lw`, so the DMA model sees the original's cycles.
fn gif_spin(c: &mut Cpu, head: u32) {
    c.g[V1][0] = GIF_CHCR as u64;
    loop {
        c.cycles += 7;
        c.g[V0][0] = c.lw(GIF_CHCR) & DMA_CHCR_STR;
        if c.g[V0][0] == 0 {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `void Dma_SendGifNormal(void* madr, int qwc, int flags)` (0x371940):
/// sends `qwc` quadwords at `madr` to the GIF through DMA channel 2 in
/// normal mode. `flags & 4`: wait for the channel first; then the VIF1
/// FIFO gets the `MSKPATH3` quadword at 0x44b430, `GIF_MODE = 4`, MADR (a
/// scratchpad address above 0x6fffffff becomes `madr & 0x3ff0 | SPR`), QWC,
/// `CHCR = 0x101` (start, from memory); `flags & 2`: wait for completion.
pub fn dma_send_gif_normal(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A3][0] = c.g[A0][0];
    c.g[T0][0] = c.g[A1][0];
    c.g[V0][0] = c.g[A2][0] & GIF_WAIT_BEFORE;
    c.g[A0][0] = 0x0045_0000;
    if c.g[V0][0] != 0 {
        c.cycles += 6;
        c.g[V0][0] = c.lw(GIF_CHCR) & DMA_CHCR_STR;
        c.g[V1][0] = 0x1000_0000;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            gif_spin(c, 0x0037_1970);
        }
    }

    c.cycles += 14;
    c.g[V0][0] = GIF_SEND_VIF_PREFIX as u64;
    c.lq(A1, GIF_SEND_VIF_PREFIX);
    c.sq(A1, VIF1_FIFO);
    c.g[A0][0] = GIF_MODE as u64;
    c.sw(GIF_MODE, 4);
    c.g[V1][0] = 4;
    let madr = c.g[A3][0];
    c.g[V0][0] = madr & 0x3ff0;
    if 0x6fff_ffffu64 < madr {
        // Scratchpad source.
        c.cycles += 7;
        c.g[V1][0] = GIF_MADR as u64;
        c.g[A0][0] = 0xffff_ffff_8000_0000;
        c.g[V0][0] |= c.g[A0][0];
        c.sw(GIF_MADR, c.g[V0][0]);
    } else {
        c.cycles += 4;
        c.sw(GIF_MADR, madr);
    }
    c.g[V0][0] = 0x1000_0000;

    c.cycles += 9;
    c.g[A0][0] = GIF_CHCR as u64;
    c.g[V0][0] = GIF_QWC as u64;
    c.sw(GIF_QWC, c.g[T0][0]);
    c.g[V1][0] = 0x101;
    c.sw(GIF_CHCR, 0x101);
    c.g[V0][0] = c.g[A2][0] & GIF_WAIT_AFTER;
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.g[V0][0] = c.lw(GIF_CHCR) & DMA_CHCR_STR;
        c.g[V1][0] = 0x1000_0000;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            gif_spin(c, 0x0037_1a30);
        }
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// TexMan_ResetUsageTable (0x3666f8)

const SPR_STREAM_DOWNLOAD: Callee = Callee {
    addr: 0x0038_f738,
    f: ee::s12::SprStream_Download,
};
/// `InvalidDCache` (libkernel; a cross-shard call through the table).
const INVALID_DCACHE: u32 = 0x0042_47d8;
/// The texture usage table: 2000 8-byte entries.
const TEX_USAGE_TABLE: u32 = 0x0053_ab80;
const TEX_USAGE_TABLE_LAST: u32 = 0x0053_e9ff;
const TEX_USAGE_ENTRIES: i64 = 2000;
/// `gp`-relative: the scratchpad stream's free pointer (`0x7000_0000`-based;
/// the scratchpad ends at `0x7000_4000`).
const GP_SPR_FREE: u32 = 0x2a94;
const SPR_END: u32 = 0x7000_4000;

/// `void TexMan_ResetUsageTable(void)` (0x3666f8): sets all 2000 entries of
/// the texture usage table (0x53ab80) to `0xffffffff` words: invalidates its
/// data cache lines, fills the free scratchpad (from the stream's free
/// pointer to the end) with the pattern quadword, and DMAs that scratchpad
/// block over the table in chunks of `free / 8` entries
/// (`SprStream_Download`, waiting for each).
///
/// The pattern quadword is built on the stack and kept in s0; s1 = entries
/// done, s0 = chunk, s2 = 2000, s3 = the table.
pub fn tex_man_reset_usage_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0xffff_ffff;
        let at = frame.at(c, 8);
        c.sd(at, c.g[V0][0]);
        c.g[A0][0] = 0x0054_0000;
        let at = frame.at(c, 0);
        c.sd(at, c.g[V0][0]);
        c.g[A1][0] = 0x0054_0000;
        frame.save(c, S0, 0x50);
        c.g[A0][0] = TEX_USAGE_TABLE as u64;
        frame.save(c, S1, 0x40);
        c.g[A1][0] = TEX_USAGE_TABLE_LAST as u64;
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        frame.restore(c, S0, 0);
        call_ptr(c, INVALID_DCACHE, 0x0036_6738)?;

        // Fill the free scratchpad with the pattern.
        let gp = r32(c, GP);
        c.g[V1][0] = Addr(gp).off(GP_SPR_FREE).lw(c);
        c.g[V0][0] = 0x7000_0000;
        if c.g[V1][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx(SPR_END.wrapping_sub(r32(c, V1)));
        }
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, V0) >> 4);
        if c.g[V0][0] != 0 {
            loop {
                c.cycles += 7;
                let at = r32(c, V1);
                c.sq(S0, at);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x10));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0036_6760);
                }
            }
            c.cycles += 1;
        }
        c.g[V1][0] = Addr(gp).off(GP_SPR_FREE).lw(c);

        // The free size in table entries per chunk.
        c.cycles += 2;
        c.g[V0][0] = 0x7000_0000;
        if c.g[V1][0] == 0 {
            c.cycles += 1;
            c.g[A1][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V0][0] = SPR_END as u64;
            c.g[A1][0] = sx(SPR_END.wrapping_sub(r32(c, V1)));
        }
        c.cycles += 6;
        c.g[V0][0] = 0x0054_0000;
        c.g[S0][0] = sx(r32(c, A1) >> 3);
        c.g[S3][0] = TEX_USAGE_TABLE as u64;
        c.g[S1][0] = 0;
        c.g[S2][0] = TEX_USAGE_ENTRIES as u64;
        c.g[V1][0] = sx(r32(c, S2).wrapping_sub(r32(c, S1)));
        loop {
            c.cycles += 11;
            c.g[A0][0] = sx(r32(c, S1) << 3);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S0][0] as i64)) as u64;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
            if c.g[V0][0] != 0 {
                // The last, shorter chunk.
                c.g[S0][0] = c.g[V1][0];
            }
            c.g[A2][0] = 2;
            c.g[A1][0] = sx(r32(c, S0) << 3);
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
            SPR_STREAM_DOWNLOAD.call(c, 0x0036_67d0)?;
            c.g[V0][0] = ((c.g[S1][0] as i64) < TEX_USAGE_ENTRIES) as u64;
            c.g[V1][0] = sx(r32(c, S2).wrapping_sub(r32(c, S1)));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_67b0);
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PlaybackMan_PollFileLoad (0x26c5f8)

/// Cross-shard calls through the function table.
const ASYNC_FILE_IS_DONE: u32 = 0x003d_f980;
const ASYNC_FILE_GET_RESULT: u32 = 0x003d_f9d8;
const OPERATOR_NEW_NAMED: u32 = 0x0031_7e30;
const EA_MEM_COPY: u32 = 0x003e_6574;
const MEM_FREE: u32 = 0x0025_23a8;
/// The allocation name passed to `operator_new_named` and its flags.
const PLAYBACK_ALLOC_NAME: u64 = 0x0048_1560;
const PLAYBACK_ALLOC_FLAGS: u64 = 0x2000_0000;
/// PlaybackMan: entry array (+0x58, 12 bytes each: +4 buffer, +8 size),
/// current entry (+0x74) and the pending async file handle (+0x78, -1:
/// none).
const PBM_ENTRIES: u32 = 0x58;
const PBM_CURRENT: u32 = 0x74;
const PBM_HANDLE: u32 = 0x78;
const PBE_SIZE: u64 = 0xc;
const PBE_BUFFER: u32 = 0x4;
const PBE_BYTES: u32 = 0x8;

/// `void PlaybackMan_PollFileLoad(PlaybackMan* self)` (0x26c5f8): when the
/// pending async load (`+0x78`) has finished, takes its result
/// (`AsyncFile_GetResult(handle, &data, &size)`); if it holds data, copies
/// it into a new buffer (`operator_new_named(size, ..)`) of the current
/// entry (`entries[cur]`: buffer, size) and frees the loaded block. The
/// handle is then cleared (-1).
///
/// s2 = self, s0 = the entry, s1 = 12 (the entry size; LO/HI from the
/// index products), `data`/`size` on the stack at 0/4.
pub fn playback_man_poll_file_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0x10);
        let this = Addr(r32(c, S2));
        c.g[A0][0] = this.off(PBM_HANDLE).lw(c);
        'done: {
            if (c.g[A0][0] as i64) < 0 {
                break 'done;
            }
            c.cycles += 5;
            call_ptr(c, ASYNC_FILE_IS_DONE, 0x0026_c624)?;
            c.g[V1][0] = 1;
            if c.g[V0][0] != 1 {
                break 'done;
            }
            c.cycles += 9;
            c.g[A0][0] = this.off(PBM_HANDLE).lw(c);
            let data_at = Addr(sp(c));
            let size_at = data_at.off(4);
            c.g[A1][0] = c.g[SP][0];
            data_at.sw(c, 0);
            c.g[A2][0] = c.g[SP][0] | 4;
            size_at.sw(c, 0);
            call_ptr(c, ASYNC_FILE_GET_RESULT, 0x0026_c648)?;
            'clear: {
                c.g[V1][0] = data_at.lw(c);
                c.g[V0][0] = u64::MAX;
                if c.g[V1][0] == 0 {
                    break 'clear;
                }
                c.cycles += 3;
                c.g[A0][0] = size_at.lw(c);
                c.g[S1][0] = PBE_SIZE;
                if (c.g[A0][0] as i64) <= 0 {
                    break 'clear;
                }
                // entries[cur].buffer = operator_new_named(size, ..)
                c.cycles += 27;
                c.g[S0][0] = this.off(PBM_CURRENT).lw(c);
                c.g[V0][0] = this.off(PBM_ENTRIES).lw(c);
                c.mult(0, S0, S0, S1);
                c.g[A1][0] = PLAYBACK_ALLOC_NAME;
                c.g[A2][0] = PLAYBACK_ALLOC_FLAGS;
                c.g[A3][0] = 0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                call_ptr(c, OPERATOR_NEW_NAMED, 0x0026_c684)?;
                let entry = Addr(r32(c, S0));
                entry.off(PBE_BUFFER).sw(c, c.g[V0][0]);
                // entries[cur].size = size
                c.g[V0][0] = this.off(PBM_CURRENT).lw(c);
                c.g[V1][0] = this.off(PBM_ENTRIES).lw(c);
                c.mult(0, V0, V0, S1);
                c.g[A0][0] = size_at.lw(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                Addr(r32(c, V0)).off(PBE_BYTES).sw(c, c.g[A0][0]);
                // EA_MemCopy(entries[cur].buffer, data, size)
                c.g[V1][0] = this.off(PBM_CURRENT).lw(c);
                c.g[V0][0] = this.off(PBM_ENTRIES).lw(c);
                c.mult(0, V1, V1, S1);
                c.g[A2][0] = size_at.lw(c);
                c.g[A1][0] = data_at.lw(c);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = Addr(r32(c, V1)).off(PBE_BUFFER).lw(c);
                call_ptr(c, EA_MEM_COPY, 0x0026_c6c0)?;
                c.g[A0][0] = data_at.lw(c);
                call_ptr(c, MEM_FREE, 0x0026_c6c8)?;
                c.g[V0][0] = u64::MAX;
            }
            c.cycles += 2;
            this.off(PBM_HANDLE).sw(c, c.g[V0][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_UpdateRaceLoopSfx (0x29d290)

/// `SSXAudioSystem_GetRaceMan` (cross-shard, through the table).
const SSX_AUDIO_GET_RACE_MAN: u32 = 0x0028_b1d8;
const SSX_AUDIO_START_LOOP_SFX: Callee = Callee {
    addr: 0x0029_d370,
    f: ee::s08::SSXAudioSystem_StartLoopSfx,
};
const SSX_AUDIO_STOP_LOOP_SFX: Callee = Callee {
    addr: 0x0029_d678,
    f: ee::s08::SSXAudioSystem_StopLoopSfx,
};
const BANK_MONITOR_STOP_BY_BANK: Callee = Callee {
    addr: 0x002a_d650,
    f: ee::s08::BankMonitor_StopByBank,
};
/// Race manager: its race state (+0x28) with the loop sound id (+0x430,
/// -1: none) and the race phase (+0x434; below 22 the loop plays).
const RACE_MAN_STATE: u32 = 0x28;
const RACE_LOOP_ID: u32 = 0x430;
const RACE_PHASE: u32 = 0x434;
const RACE_PHASE_LOOPING: i64 = 22;
/// Audio system: the loop id last seen (+0x5fc0, compared by its low byte)
/// and the sound engine pointer (+0x118 -> object with the bank monitor at
/// +0x1d8 and the event table at +0xacc: two events at +0x300/+0x360, each
/// "on" word followed by its bank id).
const AUDIO_LOOP_SEEN: u32 = 0x5fc0;
const AUDIO_ENGINE: u32 = 0x118;
const ENGINE_BANK_MONITOR: u32 = 0x1d8;
const ENGINE_EVENTS: u32 = 0xacc;
/// `gp`-relative fade times (`.lit4`): the loop stop and the two bank stops.
const GP_LOOP_FADE: u32 = 0x4594;
const GP_BANK_A_FADE: u32 = 0x4590;
const GP_BANK_B_FADE: u32 = 0x458c;

/// `BankMonitor_StopByBank(engine.bank_monitor, id, fade)` for the event at
/// `events + on`: id = the bank id after the "on" word if that word is 1,
/// else -1. `fade_gp` is the gp offset of the fade literal; `extra` the
/// cycles charged with the call.
fn stop_event_bank(c: &mut Cpu, on: u32, fade_gp: u32, extra: u64, ret: u32) -> crate::call::Flow {
    let engine = Addr(r32(c, S0)).off(AUDIO_ENGINE).lw(c);
    c.g[V0][0] = engine;
    c.g[V1][0] = Addr(engine as u32).lw(c);
    let object = r32(c, V1);
    c.g[A1][0] = Addr(object).off(ENGINE_EVENTS).lw(c);
    let events = r32(c, A1);
    c.g[V0][0] = Addr(events).off(on).lw(c) ^ 1;
    c.g[A0][0] = sx(object.wrapping_add(ENGINE_BANK_MONITOR));
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.g[A1][0] = u64::MAX;
    } else {
        c.cycles += 2;
        c.g[A1][0] = Addr(events).off(on + 4).lw(c);
    }
    c.cycles += extra;
    let gp = r32(c, GP);
    c.lwc1(12, gp.wrapping_sub(fade_gp));
    BANK_MONITOR_STOP_BY_BANK.call(c, ret)
}

/// `void SSXAudioSystem_UpdateRaceLoopSfx(SSXAudioSystem* self)` (0x29d290):
/// follows the race's loop sound id (`raceMan->state+0x430`). When it is
/// set (not -1) and its low byte differs from the one last seen
/// (`self+0x5fc0`, then updated): in the looping phases (`+0x434 < 22`)
/// starts the loop sfx; afterwards stops it (`StopLoopSfx` with a fade) and
/// stops the banks of the two race events (`BankMonitor_StopByBank`).
pub fn ssx_audio_system_update_race_loop_sfx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, SSX_AUDIO_GET_RACE_MAN, 0x0029_d2a4)?;
        let this = Addr(r32(c, S0));
        c.g[A1][0] = Addr(r32(c, V0)).off(RACE_MAN_STATE).lw(c);
        let race = Addr(r32(c, A1));
        c.g[A0][0] = race.off(RACE_LOOP_ID).lw(c);
        c.g[V0][0] = c.g[A0][0] & 0xff;
        'out: {
            if c.g[A0][0] == u64::MAX {
                c.cycles += 1;
                break 'out;
            }
            c.cycles += 3;
            c.g[V1][0] = this.off(AUDIO_LOOP_SEEN).lbu(c);
            if c.g[V0][0] == c.g[V1][0] {
                break 'out;
            }
            c.cycles += 5;
            this.off(AUDIO_LOOP_SEEN).sw(c, c.g[A0][0]);
            c.g[V0][0] = race.off(RACE_PHASE).lw(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < RACE_PHASE_LOOPING) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                SSX_AUDIO_START_LOOP_SFX.call(c, 0x0029_d2e0)?;
                break 'out;
            }
            c.cycles += 10;
            let gp = r32(c, GP);
            c.lwc1(12, gp.wrapping_sub(GP_LOOP_FADE));
            c.g[A0][0] = c.g[S0][0];
            SSX_AUDIO_STOP_LOOP_SFX.call(c, 0x0029_d2f4)?;
            stop_event_bank(c, 0x300, GP_BANK_A_FADE, 10, 0x0029_d328)?;
            stop_event_bank(c, 0x360, GP_BANK_B_FADE, 3, 0x0029_d35c)?;
            c.cycles += 1;
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, scramble, Rng, DATA};

    /// SNDSTRM_FindRequest: four stream slots (one empty), each with a
    /// two-request list; handles that are negative, name the empty slot or an
    /// unknown request, or match the first or second request.
    #[test]
    fn sndstrm_find_request_matches() {
        check(ee::s13::f_003b7a70__generated, sndstrm_find_request, 300, &|c, seed| {
            const B: u32 = 0x00b0_0000;
            const STREAM_COUNT: u32 = 0x0050_a92e;
            const STREAMS: u32 = 0x0050_a0a0;
            let mut r = Rng(0x003b_7a70 ^ (seed as u64) << 5);
            scramble(c, &mut r);
            c.bus.write8(STREAM_COUNT, 4);
            for i in 0..4u32 {
                let stream = B + 0x1000 * i;
                c.bus.write32(STREAMS + 4 * i, if i == 3 { 0 } else { stream });
                let (first, second) = (stream + 0x200, stream + 0x300);
                c.bus.write32(stream + STREAM_REQUESTS, first);
                c.bus.write32(first, second);
                c.bus.write32(second, 0);
                c.bus.write32(first + REQUEST_HANDLE, i | 0x100);
                c.bus.write32(second + REQUEST_HANDLE, i | 0x200);
            }
            let stream = r.next_u32() % 4;
            let handle = match seed % 5 {
                0 => 0x8000_0000 | r.next_u32(),
                1 => stream | 0x300,
                2 => stream | 0x100,
                _ => stream | 0x200,
            };
            c.g[A0][0] = handle as i32 as i64 as u64;
            c.pc = 0x003b_7a70;
        });
    }

    /// AnimDataBase_DecodeChanCache: an anim database at DATA with its
    /// channel array at DATA+0x1000; a header of 0..=12 channels (0: the
    /// early return) with random type nibbles (0-15: every table case, the
    /// counted types 6-9 and the skipped ones), random constant bytes and
    /// frame counts (also small and negative).
    #[test]
    fn anim_data_base_decode_chan_cache_matches() {
        check(
            ee::s10::f_003130a8__generated,
            anim_data_base_decode_chan_cache,
            300,
            &|c, seed| {
                let mut r = Rng(0x0031_30a8 ^ (seed as u64) << 7);
                scramble(c, &mut r);
                crate::difftest::put_words(
                    c,
                    DECODE_JUMP_TABLE,
                    &[
                        CASE_ONE, CASE_TWO, CASE_THREE, CASE_FOUR, CASE_FIVE, CASE_TWO,
                        CASE_TWO, CASE_TWO,
                    ],
                );
                let this = DATA;
                let chans = DATA + 0x1000;
                let data = DATA + 0x2000 + (r.next_u32() & 3);
                fill(c, &mut r, this, 8);
                c.bus.write32(this + ADB_CHANNELS, chans);
                fill(c, &mut r, chans, 0x50);
                let n = if seed % 10 == 0 { 0 } else { 1 + r.next_u32() % 12 };
                c.bus.write8(data, n as u8);
                for i in 0..0x100 {
                    c.bus.write8(data + 1 + i, r.next_u32() as u8);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(data);
                let frames = match seed % 4 {
                    0 => r.next_u32() % 4,
                    1 => (r.next_u32() % 8).wrapping_sub(4),
                    _ => r.next_u32() % 400,
                };
                c.g[A2][0] = sx(frames);
                c.pc = 0x0031_30a8;
            },
        );
    }

    /// PATH_SelectMasterTrack: the state at DATA (pointer at 0x44f420) with
    /// 24 track slots, each empty, disabled, with a negative `+0x36`, no
    /// header, or a candidate with a random length (sometimes equal to an
    /// earlier one, sometimes with the top bit set) and header word.
    #[test]
    fn path_select_master_track_matches() {
        check(ee::s14::f_003d4b00__generated, path_select_master_track, 300, &|c, seed| {
            let mut r = Rng(0x003d_4b00 ^ (seed as u64) << 9);
            scramble(c, &mut r);
            let state = DATA;
            c.bus.write32(PATH_STATE, state);
            fill(c, &mut r, state, 4);
            let lengths = [r.next_u32() % 100, r.next_u32()];
            for i in 0..24u32 {
                let track = DATA + 0x1000 + 0x100 * i;
                let header = DATA + 0x4000 + 0x10 * i;
                let empty = r.next_u32() % 6 == 0;
                c.bus.write32(state + PATH_TRACKS + 4 * i, if empty { 0 } else { track });
                fill(c, &mut r, track, 0x10);
                c.bus.write8(track + TRACK_DISABLED, (r.next_u32() % 5 == 0) as u8);
                let flags = if r.next_u32() % 5 == 0 { 0x8000 | r.next_u32() } else { r.next_u32() & 0x7fff };
                c.bus.write16(track + TRACK_FLAGS, flags as u16);
                c.bus.write32(track + TRACK_HEADER, if r.next_u32() % 5 == 0 { 0 } else { header });
                c.bus.write32(header, r.next_u32());
                let len = match r.next_u32() % 4 {
                    0 => lengths[0],
                    1 => lengths[1],
                    _ => r.next_u32() % (100 + 10 * seed),
                };
                c.bus.write32(track + TRACK_LENGTH, len);
            }
            c.pc = 0x003d_4b00;
        });
    }

    /// SND_MixAddRamp16: 16 random (edge-case or ordinary) source and
    /// destination samples in separate buffers, or the same buffer (in-place
    /// mixing), and random start/end gains.
    #[test]
    fn snd_mix_add_ramp16_matches() {
        check(ee::s14::f_003c7828__generated, snd_mix_add_ramp16, 300, &|c, seed| {
            let mut r = Rng(0x003c_7828 ^ (seed as u64) << 11);
            scramble(c, &mut r);
            let src = DATA;
            let dst = if seed % 5 == 0 { DATA } else { DATA + 0x100 };
            for i in 0..16 {
                let v = if seed % 3 == 0 { r.edge_f32() } else { r.f32(2.0) };
                c.bus.write32(src + 4 * i, v);
                if dst != src {
                    let v = if seed % 3 == 0 { r.edge_f32() } else { r.f32(2.0) };
                    c.bus.write32(dst + 4 * i, v);
                }
            }
            c.g[A0][0] = sx(src);
            c.g[A1][0] = sx(dst);
            c.f[12] = if seed % 7 == 0 { r.edge_f32() } else { r.f32(1.0) };
            c.f[13] = if seed % 7 == 1 { r.edge_f32() } else { r.f32(1.0) };
            c.pc = 0x003c_7828;
        });
    }

    /// Dma_SendGifNormal: a GIF packet of 0..=16 empty end-of-packet GIF
    /// tags in main memory (or a scratchpad address above 0x6fffffff), the
    /// `MSKPATH3` quadword at 0x44b430, and every combination of the wait
    /// flags (with the channel busy or idle beforehand: a first transfer
    /// started by the setup).
    #[test]
    fn dma_send_gif_normal_matches() {
        check(ee::s12::f_00371940__generated, dma_send_gif_normal, 300, &|c, seed| {
            let mut r = Rng(0x0037_1940 ^ (seed as u64) << 13);
            scramble(c, &mut r);
            crate::difftest::put_words(c, GIF_SEND_VIF_PREFIX, &[0x0600_0000, 0, 0, 0]);
            let qwc = r.next_u32() % 17;
            for i in 0..16 {
                crate::difftest::put_vec(c, DATA + 0x10 * i, [0x8000, 0, 0, 0]);
                c.bus.spr[0x10 * i as usize..0x10 * i as usize + 4]
                    .copy_from_slice(&0x8000u32.to_le_bytes());
            }
            let madr = if seed % 4 == 0 { 0x7000_0000 | (r.next_u32() % 4) << 4 } else { DATA };
            c.g[A0][0] = sx(madr);
            c.g[A1][0] = sx(qwc);
            c.g[A2][0] = sx(r.next_u32() % 8 | r.next_u32() & 0xff00);
            if seed % 3 == 0 {
                // A transfer in flight: the channel is busy at the start.
                c.sw(GIF_MADR, DATA as u64);
                c.sw(GIF_QWC, 16);
                c.sw(GIF_CHCR, 0x101);
            }
            c.pc = 0x0037_1940;
        });
    }

    /// TexMan_ResetUsageTable: the scratchpad stream's free pointer at
    /// `gp+0x2a94` leaving 0x400..0x4000 free bytes (16..2 chunks, the last
    /// one shorter when 2000 is not a multiple), random table and scratchpad
    /// contents. The real InvalidDCache and SprStream_Download run (DMA from
    /// the scratchpad, waiting). A null free pointer is not covered: the
    /// chunk size would be 0 and the original would loop forever.
    #[test]
    fn tex_man_reset_usage_table_matches() {
        check(ee::s12::f_003666f8__generated, tex_man_reset_usage_table, 200, &|c, seed| {
            let mut r = Rng(0x0036_66f8 ^ (seed as u64) << 17);
            scramble(c, &mut r);
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            let free = 0x400 + (r.next_u32() % 0x3c1) * 0x10;
            let ptr = SPR_END - free;
            c.bus.write32(crate::difftest::GAME_GP + GP_SPR_FREE, ptr);
            fill(c, &mut r, TEX_USAGE_TABLE, 4000);
            for i in 0..0x1000 {
                c.bus.spr[i * 4..i * 4 + 4].copy_from_slice(&r.next_u32().to_le_bytes());
            }
            c.pc = 0x0036_66f8;
        });
    }

    /// PlaybackMan_PollFileLoad: the engine allocator and fake syscalls
    /// (`feasyncfile::tests::base_2`), four async file slots (0x450c34,
    /// count 0x519bf0) and a PlaybackMan whose handle is negative, unknown,
    /// a slot still loading, or a finished slot whose result has no data,
    /// a zero/negative size, or a real allocated block (copied into a new
    /// buffer of the current entry, then freed).
    #[test]
    fn playback_man_poll_file_load_matches() {
        use crate::game::fe::feasyncfile::tests::base_2;
        const RET: u64 = 0x0012_3450;
        check(ee::s07::f_0026c5f8__generated, playback_man_poll_file_load, 300, &|c, seed| {
            let mut r = base_2(c, seed, 0x0026_c5f8);
            let slots = DATA + 0x10000;
            let this = DATA + 0x11000;
            let entries = DATA + 0x11100;
            c.bus.write32(0x0045_0c34, slots);
            c.bus.write32(0x0051_9bf0, 4);
            fill(c, &mut r, slots, 4 * 12);
            for i in 0..4u32 {
                // Not done, no error, by default.
                let file = slots + 0x30 * i;
                c.bus.write32(file, 0x100 | i);
                c.bus.write32(file + 0x1c, 1);
                c.bus.write32(file + 0x10, 0);
            }
            fill(c, &mut r, this, 0x20);
            fill(c, &mut r, entries, 0x30);
            c.bus.write32(this + PBM_ENTRIES, entries);
            c.bus.write32(this + PBM_CURRENT, r.next_u32() % 4);
            let slot = r.next_u32() % 4;
            let file = slots + 0x30 * slot;
            let size = 1 + r.next_u32() % 0x180;
            let handle = match seed % 6 {
                0 => 0x8000_0000 | r.next_u32(),
                1 => 0x200 | slot,
                _ => 0x100 | slot,
            };
            if seed % 6 != 2 {
                c.bus.write32(file + 0x1c, 0);
            }
            match seed % 6 {
                3 => {
                    // Finished without data.
                    c.bus.write32(file + 0x8, size);
                    c.bus.write32(file + 0x2c, size);
                }
                4 => {
                    // Finished with an empty or negative size.
                    let n = if r.next_u32() % 2 == 0 { 0 } else { 0x8000_0000 | r.next_u32() };
                    c.bus.write32(file + 0x8, n);
                    c.bus.write32(file + 0x2c, (DATA + 0x12000).wrapping_add(n));
                }
                _ => {
                    // A loaded block from the engine allocator.
                    c.g[A0][0] = size as u64;
                    c.g[A1][0] = PLAYBACK_ALLOC_NAME;
                    c.g[A2][0] = PLAYBACK_ALLOC_FLAGS;
                    c.g[A3][0] = 0;
                    c.g[RA][0] = RET;
                    c.pc = OPERATOR_NEW_NAMED;
                    ee::s10::f_00317e30(c);
                    assert_eq!(c.pc, RET as u32, "operator_new_named did not return");
                    let data = c.g[V0][0] as u32;
                    assert!(data != 0, "no block");
                    fill(c, &mut r, data, size / 4);
                    c.bus.write32(file + 0x8, size);
                    c.bus.write32(file + 0x2c, data + size);
                }
            }
            c.bus.write32(this + PBM_HANDLE, handle);
            c.g[A0][0] = sx(this);
            c.g[SP][0] = crate::difftest::SP as u64;
            c.pc = 0x0026_c5f8;
        });
    }

    /// SSXAudioSystem_UpdateRaceLoopSfx: the world at `gp-0x848`
    /// (`feasyncfile::tests::base_2`) with its race manager's state holding
    /// a loop id that is -1, the same (low byte) as the one seen, or new;
    /// phases below and from 22. The real StartLoopSfx (no loop bank:
    /// `+0x6c74 = 0`), StopLoopSfx (nothing playing: `+0x5fc4 = 0`) and
    /// BankMonitor_StopByBank (63 idle instances, the events on or off)
    /// run.
    #[test]
    fn ssx_audio_system_update_race_loop_sfx_matches() {
        use crate::game::fe::feasyncfile::tests::base_2;
        check(
            ee::s08::f_0029d290__generated,
            ssx_audio_system_update_race_loop_sfx,
            300,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x0029_d290);
                let gp = crate::difftest::GAME_GP;
                let app = c.bus.read32(gp - 0x848);
                let man = c.bus.read32(app + 0x84);
                let race_man = DATA + 0x10000;
                let race = DATA + 0x10100;
                let this = DATA + 0x11000;
                let engine_ptr = DATA + 0x18000;
                let object = DATA + 0x18100;
                let events = DATA + 0x19000;
                let instances = DATA + 0x1a000;
                c.bus.write32(man + 0xc, race_man);
                c.bus.write32(race_man + RACE_MAN_STATE, race);
                for off in [GP_LOOP_FADE, GP_BANK_A_FADE, GP_BANK_B_FADE] {
                    let v = r.f32(8.0);
                    c.bus.write32(gp - off, v);
                }
                let seen = r.next_u32();
                c.bus.write32(this + AUDIO_LOOP_SEEN, seen);
                let id = match seed % 4 {
                    0 => u32::MAX,
                    1 => (r.next_u32() & !0xff) | (seen & 0xff),
                    _ => r.next_u32() | 1,
                };
                c.bus.write32(race + RACE_LOOP_ID, id);
                let phase = if seed % 2 == 0 { r.next_u32() % 22 } else { 22 + r.next_u32() % 10 };
                c.bus.write32(race + RACE_PHASE, phase);
                c.bus.write32(this + 0x6c74, 0);
                c.bus.write32(this + 0x5fc4, 0);
                c.bus.write32(this + AUDIO_ENGINE, engine_ptr);
                c.bus.write32(engine_ptr, object);
                c.bus.write32(object + ENGINE_EVENTS, events);
                for on in [0x300u32, 0x360] {
                    let state = if r.next_u32() % 2 == 0 { 1 } else { r.next_u32() % 4 };
                    c.bus.write32(events + on, state);
                    c.bus.write32(events + on + 4, r.next_u32() % 3);
                }
                c.bus.write32(object + ENGINE_BANK_MONITOR + 0x8ec, instances);
                for i in 0..64u32 {
                    let inst = instances + i * 0xc0;
                    c.bus.write32(inst, 0);
                    c.bus.write32(inst + 0xc, r.next_u32() % 3);
                }
                c.g[A0][0] = sx(this);
                c.pc = 0x0029_d290;
            },
        );
    }
}
