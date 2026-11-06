//! Staging wave 209 (oc272): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::call::Unwound;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::abi;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003d_fed0, stream_file_parse_chunks, "StreamFile_ParseChunks"),
    (0x002a_c180, bank_monitor_instance_fade_out, "BankMonitorInstance_FadeOut"),
    (0x002f_e2c0, wscript_fn_add_uv_scroll_modifier, "WScriptFn_AddUVScrollModifier"),
    (0x0031_4518, anim_seq_list_push_anim, "AnimSeqList_PushAnim"),
    (0x0031_d8b0, c_csi_cubic_spline_solve, "cCSICubicSpline_Solve"),
    (0x0022_4c00, luno_vm_register_write_ptr, "LunoVMRegister_WritePtr"),
    (0x003e_0270, stream_file_start_next_request, "StreamFile_StartNextRequest"),
    (0x0034_fbf0, world_obj_dtor, "WorldObj_dtor"),
    (0x0011_6950, trick_build_name, "Trick_BuildName"),
];

// ---------------------------------------------------------------------------
// StreamFile_ParseChunks (0x3dfed0, seg_2DCAE8.c)

const SFPC_ENTRY: u32 = 0x003d_fed0;
/// The loop head the trampoline re-enters at after an unwind (0x3dff08).
const SFPC_HEAD: u32 = 0x003d_ff08;
const SFPC_END: u32 = 0x003e_00d8;
/// StreamFile fields: its mutex (+4), the per-class chunk tallies (+0x24,
/// 16-byte entries `{.., .., bytes +8, first chunk +0xc}` for class 1..),
/// the low-water mark (+0x44), the "needs refill" flag (+0x48), the parsed
/// byte count (+0x4c), the parse cursor (+0x54), the fill cursor (+0x58)
/// and the stream header (+0x60).
const SFPC_MUTEX: u32 = 0x4;
const SFPC_TALLIES: u32 = 0x24;
const SFPC_LOW_WATER: u32 = 0x44;
const SFPC_NEEDS_REFILL: u32 = 0x48;
const SFPC_PARSED: u32 = 0x4c;
const SFPC_CURSOR: u32 = 0x54;
const SFPC_FILL: u32 = 0x58;
const SFPC_HEADER: u32 = 0x60;
/// Header fields: the stream state (+4; 4 = shutting down) and the id of
/// the end-of-stream chunk (+0x11c).
const SFPC_HDR_STATE: u32 = 0x4;
const SFPC_HDR_END_ID: u32 = 0x11c;
const SFPC_STATE_CLOSING: u64 = 4;
/// Chunk header: `{id, bytes | class << 24}`.
const SFPC_CHUNK_BYTES: u32 = 0x4;
/// Tally entry fields.
const SFPC_TALLY_BYTES: u32 = 0x8;
const SFPC_TALLY_FIRST: u32 = 0xc;

const SFPC_CLASSIFY: Callee = Callee {
    addr: 0x003d_fe88,
    f: ee::s14::StreamFile_ClassifyChunk,
};
const SFPC_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};
const SFPC_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};

/// Where the parse loop goes after one chunk.
enum SfpcNext {
    /// Back to the "enough bytes for a header?" test (v0 = fill cursor).
    Top,
    /// Return v0.
    Exit,
}

/// `int StreamFile_ParseChunks(StreamFile* self)` (0x3dfed0): walks the
/// chunks between the parse cursor and the fill cursor. Each complete chunk
/// is classified; under the stream's mutex (and unless the stream is shutting
/// down) an unknown chunk is marked dead (id -2), a known one is tagged with
/// its class (`bytes | class << 24`), added to its class tally and to the
/// parsed byte count (clearing the "needs refill" flag when that crosses the
/// low-water mark), and the cursor moves past it. The end-of-stream chunk (or
/// one whose length already carries a class tag) is padded to 64-byte
/// alignment and ends the walk with 1; a shutdown or an incomplete chunk ends
/// it with 0. Frame 0x90 (ra at 0, s7..s0 at 0x10..0x80). Dispatcher-shaped:
/// re-entered at [`SFPC_HEAD`] after an unwind at the loop's event check.
pub fn stream_file_parse_chunks(c: &mut Cpu) {
    guest(c, |c| {
        let mut at_head = match c.pc {
            SFPC_ENTRY => {
                c.cycles += 14;
                let frame = Frame::open(c, 0x90);
                frame.save(c, S0, 0x80);
                frame.save(c, S7, 0x10);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S1, 0x70);
                c.g[S7][0] = sx(0xffff_ffc0);
                frame.save(c, S2, 0x60);
                frame.save(c, S3, 0x50);
                frame.save(c, S4, 0x40);
                frame.save(c, S5, 0x30);
                frame.save_ra(c, 0);
                frame.save(c, S6, 0x20);
                c.g[S6][0] = c.lw(r32(c, S0).wrapping_add(SFPC_HEADER));
                // 0x3e0054
                c.cycles += 1;
                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(SFPC_FILL));
                false
            }
            SFPC_HEAD => true,
            pc => {
                c.jump_out(pc, SFPC_ENTRY, SFPC_END);
                return Ok(());
            }
        };
        loop {
            if !at_head {
                // 0x3e0058: at least a chunk header (8 bytes) buffered?
                c.cycles += 5;
                let chunk = c.lw(r32(c, S0).wrapping_add(SFPC_CURSOR));
                c.g[S2][0] = chunk;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(chunk as u32));
                let short = (c.g[V0][0] as i64) < 8;
                c.g[V0][0] = 0;
                if short {
                    break;
                }
                // 0x3e006c: a length that already carries a class tag turns
                // the chunk into an 8-byte end-of-stream chunk.
                c.cycles += 5;
                c.g[S1][0] = c.lw(r32(c, S2).wrapping_add(SFPC_CHUNK_BYTES));
                c.g[V0][0] = c.g[S1][0] & 0xffff_ffff_ff00_0000;
                if c.g[V0][0] != 0 {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(r32(c, S6).wrapping_add(SFPC_HDR_END_ID));
                    c.g[S1][0] = 8;
                    c.sw(r32(c, S2).wrapping_add(SFPC_CHUNK_BYTES), 8);
                    c.sw(r32(c, S2), c.g[V0][0]);
                }
                c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(SFPC_CURSOR));
                // 0x3e0094: the whole chunk buffered?
                c.cycles += 5;
                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(SFPC_FILL));
                c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    break;
                }
                c.g[V1][0] = c.lw(r32(c, S2));
                if back_edge_unwinds(c, SFPC_HEAD) {
                    return Err(Unwound);
                }
            }
            at_head = false;
            match sfpc_parse_one(c)? {
                SfpcNext::Top => continue,
                SfpcNext::Exit => break,
            }
        }
        // 0x3e00ac
        c.cycles += 11;
        let frame = Frame { size: 0x90 };
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

/// `len` padded so that `cursor + len` ends 64-byte aligned (`s7` = -64).
fn sfpc_align64(c: &Cpu, len: u32, cursor_low: u64) -> (u64, u64) {
    let low = cursor_low & 0x3f;
    let padded = sx(len.wrapping_add(low as u32));
    let padded = sx((padded as u32).wrapping_add(0x3f)) & c.g[S7][0];
    (low, padded)
}

/// One complete chunk at s2 (length s1, id in v1): 0x3dff08..0x3e0050.
fn sfpc_parse_one(c: &mut Cpu) -> Result<SfpcNext, Unwound> {
    // 0x3dff08: the end chunk is padded to 64-byte alignment first.
    c.cycles += 3;
    c.g[V0][0] = c.lw(r32(c, S6).wrapping_add(SFPC_HDR_END_ID));
    if c.g[V1][0] == c.g[V0][0] {
        c.cycles += 6;
        let (len, cursor) = (r32(c, S1), c.g[A0][0]);
        let (low, padded) = sfpc_align64(c, len, cursor);
        c.g[V1][0] = low;
        c.g[V0][0] = padded;
        c.g[S1][0] = sx((padded as u32).wrapping_sub(low as u32));
    }
    c.g[A0][0] = c.g[S0][0];
    // 0x3dff2c
    c.cycles += 5;
    c.g[A1][0] = c.g[S2][0];
    SFPC_CLASSIFY.call(c, 0x003d_ff34)?;
    c.g[S5][0] = c.g[V0][0];
    c.g[V0][0] = sx(r32(c, S5) << 24);
    if (c.g[S5][0] as i64) < 0 {
        // 0x3dff40: unknown chunk: mark it dead and skip it.
        c.cycles += 8;
        c.g[S3][0] = sx(r32(c, S0).wrapping_add(SFPC_MUTEX));
        c.g[A0][0] = c.g[S3][0];
        SFPC_LOCK.call(c, 0x003d_ff4c)?;
        c.g[V0][0] = c.lw(r32(c, S6).wrapping_add(SFPC_HDR_STATE)) ^ SFPC_STATE_CLOSING;
        c.g[S4][0] = (c.g[V0][0] < 1) as u64;
        c.g[V1][0] = sx(0xffff_fffe);
        if c.g[S4][0] == 0 {
            c.cycles += 4;
            c.sw(r32(c, S2), c.g[V1][0]);
            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(SFPC_CURSOR));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
            c.sw(r32(c, S0).wrapping_add(SFPC_CURSOR), c.g[V0][0]);
        }
        // 0x3dff70
        c.cycles += 4;
        c.g[A0][0] = c.g[S3][0];
        SFPC_UNLOCK.call(c, 0x003d_ff78)?;
    } else {
        // 0x3dff80: tag the chunk with its class and account for it.
        c.cycles += 9;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(SFPC_MUTEX));
        c.g[V0][0] |= c.g[S1][0];
        c.sw(r32(c, S2).wrapping_add(SFPC_CHUNK_BYTES), c.g[V0][0]);
        SFPC_LOCK.call(c, 0x003d_ff90)?;
        c.g[V0][0] = c.lw(r32(c, S6).wrapping_add(SFPC_HDR_STATE)) ^ SFPC_STATE_CLOSING;
        c.g[S4][0] = (c.g[V0][0] < 1) as u64;
        c.g[A2][0] = sx(r32(c, S0).wrapping_add(SFPC_MUTEX));
        if c.g[S4][0] == 0 {
            // 0x3dffa4: tallies[class - 1].bytes += len (first chunk noted).
            c.cycles += 8;
            let tallies = c.lw(r32(c, S0).wrapping_add(SFPC_TALLIES));
            c.g[V1][0] = sx(r32(c, S5) << 4);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x10));
            c.g[V0][0] = sx((tallies as u32).wrapping_add(r32(c, V1)));
            let tally = r32(c, V0);
            c.g[A0][0] = c.lw(tally.wrapping_add(SFPC_TALLY_BYTES));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
            c.sw(tally.wrapping_add(SFPC_TALLY_BYTES), c.g[A0][0]);
            if c.g[A0][0] == c.g[S1][0] {
                c.cycles += 1;
                c.sw(tally.wrapping_add(SFPC_TALLY_FIRST), c.g[S2][0]);
            }
            // 0x3dffc8: parsed count and cursor; low-water crossing.
            c.cycles += 9;
            let this = r32(c, S0);
            c.g[V1][0] = c.lw(this.wrapping_add(SFPC_PARSED));
            c.g[V0][0] = c.lw(this.wrapping_add(SFPC_CURSOR));
            c.g[A0][0] = c.lw(this.wrapping_add(SFPC_LOW_WATER));
            c.g[A1][0] = sx(r32(c, V1).wrapping_add(r32(c, S1)));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
            c.sw(this.wrapping_add(SFPC_PARSED), c.g[A1][0]);
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            c.sw(this.wrapping_add(SFPC_CURSOR), c.g[V0][0]);
            if c.g[V1][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.sw(this.wrapping_add(SFPC_NEEDS_REFILL), 0);
                }
            }
        }
        // 0x3dfff8
        c.cycles += 2;
        c.g[A0][0] = c.g[A2][0];
        SFPC_UNLOCK.call(c, 0x003e_0000)?;
    }
    // 0x3e0000
    c.cycles += 2;
    if c.g[S4][0] == 0 {
        c.g[V1][0] = c.lw(r32(c, S2));
        // 0x3e0040: stop after the end chunk.
        c.cycles += 3;
        c.g[V0][0] = c.lw(r32(c, S6).wrapping_add(SFPC_HDR_END_ID));
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(SFPC_FILL));
            return Ok(SfpcNext::Top);
        }
        c.cycles += 2;
        c.g[V0][0] = 1;
        return Ok(SfpcNext::Exit);
    }
    // 0x3e0008: shutting down: pad a non-end chunk and stop with 0.
    c.cycles += 4;
    c.g[V1][0] = c.lw(r32(c, S6).wrapping_add(SFPC_HDR_END_ID));
    c.g[V0][0] = c.lw(r32(c, S2));
    let is_end = c.g[V0][0] == c.g[V1][0];
    c.g[V0][0] = 0;
    if is_end {
        return Ok(SfpcNext::Exit);
    }
    c.cycles += 10;
    let cursor = c.lw(r32(c, S0).wrapping_add(SFPC_CURSOR));
    c.g[A0][0] = sx(r32(c, S5) << 24);
    let len = r32(c, S1);
    let (low, padded) = sfpc_align64(c, len, cursor);
    c.g[V0][0] = low;
    c.g[V1][0] = padded;
    c.g[S1][0] = sx((padded as u32).wrapping_sub(low as u32));
    c.g[A0][0] |= c.g[S1][0];
    c.sw(r32(c, S2).wrapping_add(SFPC_CHUNK_BYTES), c.g[A0][0]);
    // 0x3e00a8
    c.cycles += 1;
    c.g[V0][0] = 0;
    Ok(SfpcNext::Exit)
}

// ---------------------------------------------------------------------------
// BankMonitorInstance_FadeOut (0x2ac180, streamsys.cpp)

/// Bank monitor instance fields: kind (+0; 1 = patch voice, 2 = stream), the
/// sound handle (+4), the playing flag (+8) and the fading flag (+0x54).
const BMI_KIND: u32 = 0x0;
const BMI_HANDLE: u32 = 0x4;
const BMI_PLAYING: u32 = 0x8;
const BMI_FADING: u32 = 0x54;
/// Fade length and target volume passed to the fade calls.
const BMI_FADE_TICKS: u64 = 0x19;
/// `SND_Lock` / `SND_Unlock` and the two fade functions (cross-shard
/// direct calls, emitted as table calls).
const BMI_SND_LOCK: u32 = 0x003b_58a0;
const BMI_SND_UNLOCK: u32 = 0x003b_58d8;
const BMI_FADE_VOICE: u32 = 0x003b_8530;
const BMI_FADE_STREAM: u32 = 0x003b_9f38;

/// `void BankMonitorInstance_FadeOut(BankMonitorInstance* self)`
/// (0x2ac180): once only (the fading flag), clears the playing flag and
/// fades the instance's voice (kind 1, `SND_FadeVolume`) or stream (kind 2)
/// to silence over 25 ticks, under the sound lock. Frame 0x20 (ra at 0, s0
/// at 0x10).
pub fn bank_monitor_instance_fade_out(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let inst = r32(c, S0);
        c.g[V0][0] = c.lw(inst.wrapping_add(BMI_FADING));
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = 1;
            c.g[V1][0] = c.lw(inst.wrapping_add(BMI_KIND));
            c.sw(inst.wrapping_add(BMI_PLAYING), 0);
            c.sw(inst.wrapping_add(BMI_FADING), 1);
            // (fade function, its three return addresses)
            let fade = if c.g[V1][0] == 1 {
                c.cycles += 10;
                Some((BMI_FADE_VOICE, [0x002a_c1cc, 0x002a_c1dc, 0x002a_c1e4]))
            } else {
                c.cycles += 3;
                c.g[V0][0] = 2;
                if c.g[V1][0] == 2 {
                    c.cycles += 9;
                    Some((BMI_FADE_STREAM, [0x002a_c1f4, 0x002a_c204, 0x002a_c20c]))
                } else {
                    c.cycles += 2;
                    None
                }
            };
            if let Some((target, rets)) = fade {
                call_ptr(c, BMI_SND_LOCK, rets[0])?;
                c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(BMI_HANDLE));
                c.g[A1][0] = BMI_FADE_TICKS;
                c.g[A2][0] = u64::MAX;
                call_ptr(c, target, rets[1])?;
                call_ptr(c, BMI_SND_UNLOCK, rets[2])?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_AddUVScrollModifier (0x2fe2c0, seg_1FBE38.cpp)

/// The default UV-scroll parameter block (52 bytes, a guarded static local:
/// `{object id = -1, 5, 0, 0, 1/60, 1/60, 1, 0, 0, 0, 0, 1, tag}`) and its guard.
const WSUV_DEFAULTS: u32 = 0x004f_b7e0;
const WSUV_DEFAULTS_HI: u64 = 0x0050_0000;
const WSUV_GUARD: u32 = 0x252c;
/// `1/60` in `.lit4` (gp-0x3384).
const WSUV_SIXTIETH: u32 = 0x3384;
/// Script argument entries: `{param index, value, .., type}` (16 bytes);
/// the parameters' declared types (`D_00446238`, 2 = float).
const WSUV_PARAM_TYPES: u64 = 0x0044_6238;
const WSUV_TYPE_FLOAT: u64 = 2;
/// The world (`gp+0xce8`, its own object at +0x290) and the object table
/// (`gp+0x16c8`) the object id resolves through.
const WSUV_WORLD: u32 = 0xce8;
const WSUV_WORLD_SELF: u32 = 0x290;
const WSUV_OBJ_TABLE: u32 = 0x16c8;
/// The script's void result (`D_004C9098`, 12 bytes).
const WSUV_RESULT_HI: u64 = 0x004d_0000;
const WSUV_RESULT_LO: u32 = 0x6f68;
const WSUV_MEMCPY: u32 = 0x003e_6574;
const WSUV_ADD_MODIFIER: u32 = 0x0035_5b90;

/// `ScriptValue WScriptFn_AddUVScrollModifier(int argc, Arg* args)`
/// (0x2fe2c0, result returned through a0): fills a 13-word UV-scroll
/// parameter block from the defaults and the script arguments (an integer
/// passed for a float parameter is converted), resolves the object id in
/// word 0 (-1: the world itself) and, if the object has a model instance
/// whose virtual `+0x80` check passes, adds the modifier to it
/// (`ModObj_AddUVScrollModifier`). Returns the void script value. Frame 0xa0
/// (block at 0x10, ra at 0x50, s3..s0 at 0x60..0x90).
pub fn wscript_fn_add_uv_scroll_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(WSUV_GUARD));
        frame.save(c, S0, 0x90);
        c.g[A3][0] = WSUV_DEFAULTS_HI;
        frame.save(c, S2, 0x70);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x60);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S1, 0x80);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x50);
        if c.g[V0][0] == 0 {
            // First use: build the defaults.
            c.cycles += 25;
            let d = WSUV_DEFAULTS;
            c.g[V0][0] = sx(d);
            c.g[V1][0] = u64::MAX;
            c.f[0] = 0;
            c.g[A2][0] = c.lbu(d.wrapping_add(0x30));
            c.g[A0][0] = 5;
            c.lwc1(2, r32(c, GP).wrapping_sub(WSUV_SIXTIETH));
            c.g[A1][0] = 1;
            c.g[AT][0] = 0x3f80_0000;
            c.f[1] = 0x3f80_0000;
            c.sw(d, c.g[V1][0]);
            c.sb(d.wrapping_add(0x30), c.g[A2][0]);
            c.sw(d.wrapping_add(0x4), 5);
            c.swc1(2, d.wrapping_add(0x14));
            c.swc1(0, d.wrapping_add(0x28));
            c.swc1(1, d.wrapping_add(0x2c));
            c.sw(r32(c, GP).wrapping_add(WSUV_GUARD), 1);
            c.swc1(0, d.wrapping_add(0x8));
            c.swc1(0, d.wrapping_add(0xc));
            c.swc1(2, d.wrapping_add(0x10));
            c.swc1(1, d.wrapping_add(0x18));
            c.swc1(0, d.wrapping_add(0x1c));
            c.swc1(0, d.wrapping_add(0x20));
            c.swc1(0, d.wrapping_add(0x24));
            c.sb(sp(c), c.g[A2][0]);
        }
        // 0x2fe354: copy the defaults into the frame's block.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        let block = sp(c).wrapping_add(0x10);
        c.g[S1][0] = sx(block);
        c.g[A2][0] = 0x34;
        c.g[A1][0] = sx(r32(c, A3).wrapping_sub(0x4820));
        c.sw(block, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, WSUV_MEMCPY, 0x002f_e374)?;
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S0][0] as i64) > 0 {
            // Apply the script arguments.
            c.cycles += 5;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = WSUV_PARAM_TYPES;
            c.g[A3][0] = WSUV_TYPE_FLOAT;
            loop {
                // 0x2fe390
                c.cycles += 8;
                let arg = r32(c, A2);
                c.g[V0][0] = c.lw(arg);
                c.g[V1][0] = c.lw(arg.wrapping_add(0xc));
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T0)));
                c.g[V0][0] = c.lw(r32(c, V0));
                let convert = if c.g[V1][0] == c.g[V0][0] {
                    false
                } else {
                    c.cycles += 2;
                    c.g[V0][0] == c.g[A3][0]
                };
                if convert {
                    c.cycles += 4;
                    c.lwc1(0, arg.wrapping_add(0x4));
                    c.cvt_s_w(0, 0);
                    c.swc1(0, r32(c, A0));
                } else {
                    c.g[V0][0] = c.lw(arg.wrapping_add(0x4));
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_e390);
                }
            }
        }
        // 0x2fe3d8: resolve the object id.
        c.cycles += 4;
        c.g[A2][0] = c.lw(r32(c, S1));
        c.g[V0][0] = !c.g[A2][0];
        c.g[A1][0] = c.lw(r32(c, GP).wrapping_add(WSUV_OBJ_TABLE));
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(WSUV_WORLD));
            c.g[V0][0] = c.lw(r32(c, V0).wrapping_add(WSUV_WORLD_SELF));
        } else {
            c.cycles += 8;
            let id = r32(c, A2);
            c.g[V0][0] = sx((id & 0xff) << 2);
            c.g[V1][0] = c.lw(r32(c, A1));
            c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x8));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = c.lw(r32(c, V0));
            c.g[V0][0] = sx(id >> 8);
            let mut found = false;
            if c.g[V1][0] != 0 {
                c.cycles += 7;
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                c.g[V1][0] = c.lw(r32(c, V0));
                c.g[V0][0] = sx(r32(c, V1) >> 8);
                found = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, V0) << 2);
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        // 0x2fe434: the object's model instance.
        c.cycles += 3;
        c.g[S0][0] = c.lw(r32(c, V0).wrapping_add(0xc));
        c.g[V1][0] = WSUV_RESULT_HI;
        if c.g[S0][0] != 0 {
            c.cycles += 7;
            let inst = r32(c, S0);
            c.g[V1][0] = c.lw(inst.wrapping_add(0xc));
            let vt = r32(c, V1);
            c.g[A0][0] = c.lh(vt.wrapping_add(0x80));
            c.g[V0][0] = c.lw(vt.wrapping_add(0x84));
            let method = r32(c, V0);
            c.g[A0][0] = sx(inst.wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x002f_e454)?;
            let ok = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if ok {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, WSUV_ADD_MODIFIER, 0x002f_e464)?;
            }
            c.cycles += 1;
            c.g[V1][0] = WSUV_RESULT_HI;
        }
        // 0x2fe468: return the void script value through a0.
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx(r32(c, V1).wrapping_sub(WSUV_RESULT_LO));
        let src = r32(c, A3);
        let dst = r32(c, S3);
        c.ldl(4, src.wrapping_add(7));
        c.ldr(4, src);
        c.g[A1][0] = c.lw(src.wrapping_add(8));
        c.sdl(4, dst.wrapping_add(7));
        c.sdr(4, dst);
        c.sw(dst.wrapping_add(8), c.g[A1][0]);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimSeqList_PushAnim (0x314518, animsequencer.cpp)

use crate::game::animation::animsequencer::{
    ASL_PUSH_NEW, ASL_SET_TRACK_ANIM, ASL_SET_TRACK_LOOP, ASL_SET_TRACK_TIME,
};

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
// cCSICubicSpline_Solve (0x31d8b0, cubicspline.cpp)

/// Spline header word: knot count in bits 11..18, bit 0 = clamped ends (the
/// start slope in knot 0's +0x10, the end slope at +4). Knots at +8, 20 bytes
/// each: on input `{x, y, ..}` (the last knot's `{x, y}` at +8/+0xc), on
/// output `{d, c, b, y, x}`.
const CSP_KNOTS: u32 = 0x8;
const CSP_END_SLOPE: u32 = 0x4;
const CSP_KNOT_SIZE: u32 = 0x14;
/// Frame arrays (256 floats each) relative to sp.
const CSP_X: u32 = 0x000;
const CSP_Y: u32 = 0x400;
const CSP_H: u32 = 0x800;
const CSP_ALPHA: u32 = 0xc00;
const CSP_L: u32 = 0x1000;
const CSP_MU: u32 = 0x1400;
const CSP_Z: u32 = 0x1800;
const CSP_C: u32 = 0x2000;
/// `.lit4` constants (1/3) for the natural and clamped back-substitution.
const CSP_THIRD_NATURAL: u32 = 0x2e48;
const CSP_THIRD_CLAMPED: u32 = 0x2e44;
const CSP_THREE: u32 = 0x4040_0000;

#[inline]
fn csp_fset(c: &mut Cpu, regs: &[(usize, FpuF32)]) {
    for &(r, v) in regs {
        c.f[r] = v.0;
    }
}
#[inline]
fn csp_add(c: &mut Cpu, r: usize, k: u32) {
    c.g[r][0] = sx(r32(c, r).wrapping_add(k));
}
#[inline]
fn csp_lt(c: &Cpu, a: usize, b: usize) -> u64 {
    ((c.g[a][0] as i64) < (c.g[b][0] as i64)) as u64
}

/// `void cCSICubicSpline_Solve(sCubicSpline* self)` (0x31d8b0): computes the
/// cubic spline coefficients through the knots (the classic tridiagonal
/// solve: h, alpha, l/mu/z forward sweep, c/b/d back substitution), natural
/// or clamped, in ten 256-float arrays on a 0x2830-byte stack frame, and
/// writes `{d, c, b, y, x}` back into each knot; the last knot's x goes to
/// +4 and the header's bits 19..26 are cleared. A leaf (s0..s2 saved at
/// 0x2800..0x2820).
pub fn c_csi_cubic_spline_solve(c: &mut Cpu) {
    c.cycles += 10;
    let frame = Frame::open(c, 0x2830);
    c.g[T3][0] = c.g[A0][0];
    frame.save(c, S0, 0x2820);
    frame.save(c, S1, 0x2810);
    frame.save(c, S2, 0x2800);
    let this = r32(c, T3);
    c.g[V0][0] = c.lw(this);
    c.g[T2][0] = sx(r32(c, V0) >> 11) & 0xff;
    let base = sp(c);
    let ys = base.wrapping_add(CSP_Y);
    c.g[T7][0] = sx(ys);
    let n = r32(c, T2);
    // Copy the knots' x and y into the frame.
    if n != 0 {
        c.cycles += 4;
        c.g[V0][0] = c.lw(this.wrapping_add(CSP_KNOTS));
        c.g[A0][0] = sx(ys);
        c.g[V1][0] = sx(base);
        c.g[T0][0] = n as u64;
        loop {
            c.cycles += 9;
            let knot = r32(c, V0);
            let x = Addr(knot).read_fpu(c);
            csp_add(c, T0, u32::MAX);
            Addr(r32(c, V1)).write_fpu(c, x);
            csp_add(c, V1, 4);
            let y = Addr(knot.wrapping_add(4)).read_fpu(c);
            c.g[V0][0] = sx(knot.wrapping_add(CSP_KNOT_SIZE));
            Addr(r32(c, A0)).write_fpu(c, y);
            c.f[0] = y.0;
            let more = c.g[T0][0] != 0;
            csp_add(c, A0, 4);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_d8e8);
            }
        }
    }
    // 0x31d8f0: the last knot's {x, y}, then h[i] = x[i+1] - x[i].
    c.cycles += 14;
    c.g[V1][0] = CSP_KNOT_SIZE as u64;
    c.g[A1][0] = c.lw(this.wrapping_add(CSP_KNOTS));
    c.mult(0, V1, T2, V1);
    let n4 = n << 2;
    c.g[V0][0] = sx(n4);
    c.g[A0][0] = sx(base.wrapping_add(n4));
    c.g[T6][0] = sx(n4);
    c.g[V0][0] = sx(ys.wrapping_add(n4));
    c.g[T0][0] = 0;
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    let last = r32(c, V1);
    let x = Addr(last.wrapping_sub(0xc)).read_fpu(c);
    Addr(r32(c, A0)).write_fpu(c, x);
    let y = Addr(last.wrapping_sub(0x8)).read_fpu(c);
    Addr(r32(c, V0)).write_fpu(c, y);
    csp_fset(c, &[(0, x), (1, y)]);
    if n != 0 {
        c.cycles += 1;
        c.g[V1][0] = sx(base);
        loop {
            c.cycles += 8;
            let p = r32(c, V1);
            let x1 = Addr(p.wrapping_add(4)).read_fpu(c);
            csp_add(c, T0, 1);
            let x0 = Addr(p).read_fpu(c);
            c.g[V0][0] = csp_lt(c, T0, T2);
            let h = x1 - x0;
            Addr(p.wrapping_add(CSP_H)).write_fpu(c, h);
            c.g[V1][0] = sx(p.wrapping_add(4));
            csp_fset(c, &[(0, h), (1, x0)]);
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_d948);
            }
        }
    }
    // 0x31d968: natural or clamped ends.
    c.cycles += 4;
    c.g[V0][0] = c.lw(this);
    c.g[V1][0] = c.g[V0][0] & 1;
    let clamped = c.g[V1][0] != 0;
    c.g[V0][0] = 1;
    if clamped {
        c.cycles += 2;
        c.f[0] = Addr(ys).read_fpu(c).0;
        csp_solve_clamped(c, base);
    } else {
        csp_solve_natural(c, base);
    }
    // 0x31de30: write {d, c, b, y, x} back into the knots.
    c.cycles += 2;
    c.g[A0][0] = sx(base);
    if c.g[T2][0] != 0 {
        c.cycles += 4;
        c.g[A2][0] = c.g[T7][0];
        c.g[T0][0] = c.g[T2][0];
        c.g[A1][0] = 0;
        loop {
            c.cycles += 25;
            let this = r32(c, T3);
            let p = r32(c, A0);
            let off = r32(c, A1);
            csp_add(c, T0, u32::MAX);
            // d, c, b into the knot's +0, +4, +8 (v1, v0, v1 hold the knot).
            for (k, src, reg) in [(0u32, 0x2400u32, V1), (4, CSP_C, V0), (8, 0x1c00, V1)] {
                let knots = c.lw(this.wrapping_add(CSP_KNOTS));
                let val = Addr(p.wrapping_add(src)).read_fpu(c);
                c.g[reg][0] = sx(off.wrapping_add(knots as u32));
                Addr(r32(c, reg).wrapping_add(k)).write_fpu(c, val);
            }
            let knots = c.lw(this.wrapping_add(CSP_KNOTS));
            let y = Addr(r32(c, A2)).read_fpu(c);
            c.g[V0][0] = sx(off.wrapping_add(knots as u32));
            csp_add(c, A2, 4);
            Addr(r32(c, V0).wrapping_add(0xc)).write_fpu(c, y);
            let knots = c.lw(this.wrapping_add(CSP_KNOTS));
            let x = Addr(p).read_fpu(c);
            c.g[V1][0] = sx(off.wrapping_add(knots as u32));
            c.g[A0][0] = sx(p.wrapping_add(4));
            Addr(r32(c, V1).wrapping_add(0x10)).write_fpu(c, x);
            c.f[0] = x.0;
            let more = c.g[T0][0] != 0;
            c.g[A1][0] = sx(off.wrapping_add(CSP_KNOT_SIZE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_de48);
            }
        }
    }
    // 0x31deac
    c.cycles += 13;
    let this = r32(c, T3);
    c.g[V1][0] = c.lw(this);
    c.g[A0][0] = sx(base.wrapping_add(r32(c, T6)));
    c.g[V0][0] = 0xffff_ffff_f807_ffff;
    let xn = Addr(r32(c, A0)).read_fpu(c);
    c.f[0] = xn.0;
    frame.restore(c, S0, 0x2820);
    c.g[V1][0] &= c.g[V0][0];
    frame.restore(c, S1, 0x2810);
    frame.restore(c, S2, 0x2800);
    Addr(this.wrapping_add(CSP_END_SLOPE)).write_fpu(c, xn);
    c.sw(this, c.g[V1][0]);
    frame.close(c);
}

/// alpha[i] = 3/h[i] (y[i+1]-y[i]) - 3/h[i-1] (y[i]-y[i-1]) for i in
/// 1..n, from `A0` (alpha) / `V1` (y) / `A1`-or-`A0` cursor registers
/// (the natural and clamped loops at 0x31d9b8 / 0x31dc18 differ only in
/// their register use).
fn csp_alpha_step(c: &mut Cpu, out: u32, hp: u32, yp: u32) -> [FpuF32; 6] {
    let h1 = Addr(hp).read_fpu(c);
    csp_add(c, T0, 1);
    let h0 = Addr(hp.wrapping_sub(4)).read_fpu(c);
    c.g[V0][0] = csp_lt(c, T0, T2);
    let y1 = Addr(yp).read_fpu(c);
    let k = FpuF32(c.f[5]);
    let r1 = k.div(h1);
    let y2 = Addr(yp.wrapping_add(4)).read_fpu(c);
    let r0 = k.div(h0);
    let y0 = Addr(yp.wrapping_sub(4)).read_fpu(c);
    let dy1 = y2 - y1;
    let dy0 = y1 - y0;
    let p1 = r1 * dy1;
    let p0 = r0 * dy0;
    let alpha = p1 - p0;
    Addr(out).write_fpu(c, alpha);
    [dy1, alpha, dy0, p0, y0, p1]
}

/// Natural ends (0x31d988..0x31db60): alpha, the forward sweep with
/// l[0] = 1, mu[0] = z[0] = 0, then c/b/d back substitution.
fn csp_solve_natural(c: &mut Cpu, base: u32) {
    let ys = base.wrapping_add(CSP_Y);
    c.cycles += 7;
    c.g[T0][0] = 1;
    c.g[T8][0] = sx(base.wrapping_add(CSP_L));
    c.g[V0][0] = csp_lt(c, T0, T2);
    c.g[T4][0] = sx(base.wrapping_add(CSP_Z));
    c.g[T9][0] = sx(base.wrapping_add(CSP_C));
    c.g[S0][0] = sx(r32(c, T2).wrapping_sub(1));
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.g[AT][0] = CSP_THREE as u64;
        c.f[5] = CSP_THREE;
        c.g[A0][0] = sx(base.wrapping_add(CSP_ALPHA + 4));
        c.g[V1][0] = sx(ys.wrapping_add(4));
        loop {
            c.cycles += 18;
            let a = r32(c, A0);
            let v = r32(c, V1);
            let [dy1, alpha, dy0, p0, y0, _] =
                csp_alpha_step(c, a, a.wrapping_sub(0x400), v);
            c.g[V1][0] = sx(v.wrapping_add(4));
            csp_fset(c, &[(0, dy1), (1, alpha), (2, dy0), (3, p0), (4, y0)]);
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx(a.wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_d9b8);
            }
        }
    }
    // 0x31da00: forward sweep.
    c.cycles += 8;
    c.g[AT][0] = 0x3f80_0000;
    c.f[0] = 0x3f80_0000;
    c.g[T0][0] = 1;
    c.sw(base.wrapping_add(CSP_MU), 0);
    c.g[V0][0] = csp_lt(c, T0, T2);
    Addr(base.wrapping_add(CSP_L)).write_fpu(c, FpuF32::ONE);
    c.sw(base.wrapping_add(CSP_Z), 0);
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[A1][0] = sx(r32(c, T4).wrapping_add(4));
        c.g[V1][0] = sx(base.wrapping_add(CSP_H));
        c.g[A0][0] = sx(r32(c, T8).wrapping_add(4));
        loop {
            c.cycles += 26;
            let v = r32(c, V1);
            let a0 = r32(c, A0);
            let a1 = r32(c, A1);
            let x0 = Addr(v.wrapping_sub(0x800)).read_fpu(c);
            csp_add(c, T0, 1);
            let x2 = Addr(v.wrapping_sub(0x7f8)).read_fpu(c);
            c.g[V0][0] = csp_lt(c, T0, T2);
            let mu0 = Addr(v.wrapping_add(0xc00)).read_fpu(c);
            let dx = x2 - x0;
            let h0 = Addr(v).read_fpu(c);
            let hm = h0 * mu0;
            let l = (dx + dx) - hm;
            Addr(a0).write_fpu(c, l);
            let h1 = Addr(v.wrapping_add(4)).read_fpu(c);
            let h0b = Addr(v).read_fpu(c);
            let mu = h1.div(l);
            let alpha = Addr(v.wrapping_add(0x404)).read_fpu(c);
            Addr(v.wrapping_add(0xc04)).write_fpu(c, mu);
            c.g[V1][0] = sx(v.wrapping_add(4));
            let z0 = Addr(a1.wrapping_sub(4)).read_fpu(c);
            let lr = Addr(a0).read_fpu(c);
            let hz = h0b * z0;
            c.g[A0][0] = sx(a0.wrapping_add(4));
            let z = (alpha - hz).div(lr);
            Addr(a1).write_fpu(c, z);
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = sx(a1.wrapping_add(4));
            csp_fset(c, &[(0, z0), (1, lr), (2, z), (3, hz)]);
            if !more {
                c.cycles += 2;
                c.g[AT][0] = 0x3f80_0000;
                c.f[0] = 0x3f80_0000;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_da30);
            }
        }
    }
    // 0x31daa0: l[n] = 1, z[n] = c[n] = 0; back substitution.
    c.cycles += 8;
    let t6 = r32(c, T6);
    c.g[V0][0] = sx(r32(c, T8).wrapping_add(t6));
    c.g[V1][0] = sx(r32(c, T4).wrapping_add(t6));
    c.g[A0][0] = sx(r32(c, T9).wrapping_add(t6));
    let f0 = FpuF32(c.f[0]);
    Addr(r32(c, V0)).write_fpu(c, f0);
    c.g[T0][0] = c.g[S0][0];
    c.sw(r32(c, V1), 0);
    c.sw(r32(c, A0), 0);
    if (c.g[T0][0] as i64) < 0 {
        return;
    }
    c.cycles += 10;
    let j4 = r32(c, T0) << 2;
    c.g[V0][0] = sx(j4);
    c.g[A0][0] = sx(base.wrapping_add(CSP_H).wrapping_add(j4));
    c.g[A1][0] = sx(j4.wrapping_add(r32(c, T4)));
    c.lwc1(6, r32(c, GP).wrapping_sub(CSP_THIRD_NATURAL));
    c.g[AT][0] = CSP_THREE as u64;
    c.f[5] = CSP_THREE;
    c.g[V1][0] = sx(j4.wrapping_add(4).wrapping_add(r32(c, T9)));
    c.g[V0][0] = sx(j4.wrapping_add(r32(c, T7)));
    loop {
        c.cycles += 29;
        let v1 = r32(c, V1);
        let a0 = r32(c, A0);
        let a1 = r32(c, A1);
        let v0 = r32(c, V0);
        let c_next = Addr(v1).read_fpu(c);
        csp_add(c, T0, u32::MAX);
        let mu = Addr(a0.wrapping_add(0xc00)).read_fpu(c);
        let z = Addr(a1).read_fpu(c);
        c.g[A1][0] = sx(a1.wrapping_sub(4));
        let cj = z - mu * c_next;
        Addr(v1.wrapping_sub(4)).write_fpu(c, cj);
        let y0 = Addr(v0).read_fpu(c);
        let y1 = Addr(v0.wrapping_add(4)).read_fpu(c);
        let s = c_next + (cj + cj);
        let h = Addr(a0).read_fpu(c);
        c.g[V0][0] = sx(v0.wrapping_sub(4));
        let h3 = h * FpuF32(c.f[5]);
        let hs6 = (h * s) * FpuF32(c.f[6]);
        let b = (y1 - y0).div(h) - hs6;
        Addr(a0.wrapping_add(0x1400)).write_fpu(c, b);
        let cn2 = Addr(v1).read_fpu(c);
        let cj2 = Addr(v1.wrapping_sub(4)).read_fpu(c);
        c.g[V1][0] = sx(v1.wrapping_sub(4));
        let d = (cn2 - cj2).div(h3);
        Addr(a0.wrapping_add(0x1c00)).write_fpu(c, d);
        let more = (c.g[T0][0] as i64) >= 0;
        c.g[A0][0] = sx(a0.wrapping_sub(4));
        csp_fset(c, &[(0, d), (1, cj2), (2, h), (3, hs6), (4, h3)]);
        if !more {
            c.cycles += 2;
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0031_dae8);
        }
    }
}

/// Clamped ends (0x31db64..0x31de2c; f0 = y[0]): alpha with the end slopes,
/// the forward sweep with l[0] = 2 h[0], mu[0] = 1/2, then back substitution.
fn csp_solve_clamped(c: &mut Cpu, base: u32) {
    let ys = base.wrapping_add(CSP_Y);
    let this = r32(c, T3);
    let t6 = r32(c, T6);
    let three = FpuF32(CSP_THREE);
    c.cycles += 40;
    c.g[A1][0] = sx(r32(c, T2).wrapping_sub(1));
    let y_first = FpuF32(c.f[0]);
    let y1 = Addr(base.wrapping_add(CSP_Y + 4)).read_fpu(c);
    let m4 = r32(c, A1) << 2;
    c.g[A0][0] = sx(m4);
    c.g[AT][0] = CSP_THREE as u64;
    c.g[A2][0] = sx(ys.wrapping_add(m4));
    let h0 = Addr(base.wrapping_add(CSP_H)).read_fpu(c);
    c.g[V0][0] = c.lw(this.wrapping_add(CSP_KNOTS));
    c.g[A3][0] = sx(ys.wrapping_add(t6));
    let end_slope = Addr(this.wrapping_add(CSP_END_SLOPE)).read_fpu(c);
    c.g[V1][0] = sx(base.wrapping_add(CSP_H));
    let start_slope = Addr(r32(c, V0).wrapping_add(0x10)).read_fpu(c);
    c.g[S1][0] = c.g[V1][0];
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(m4));
    let alpha0 = ((y1 - y_first) * three).div(h0) - start_slope * three;
    c.g[S2][0] = sx(base.wrapping_add(CSP_ALPHA));
    c.g[T0][0] = 1;
    c.g[V0][0] = sx(base.wrapping_add(CSP_ALPHA).wrapping_add(t6));
    c.g[S0][0] = c.g[A1][0];
    c.g[A0][0] = csp_lt(c, T0, T2);
    c.g[T8][0] = sx(base.wrapping_add(CSP_L));
    c.g[T5][0] = sx(base.wrapping_add(CSP_MU));
    c.g[T4][0] = sx(base.wrapping_add(CSP_Z));
    c.g[T9][0] = sx(base.wrapping_add(CSP_C));
    Addr(base.wrapping_add(CSP_ALPHA)).write_fpu(c, alpha0);
    let yn = Addr(r32(c, A3)).read_fpu(c);
    let yp = Addr(r32(c, A2)).read_fpu(c);
    let hl = Addr(r32(c, V1)).read_fpu(c);
    let q = ((yn - yp) * three).div(hl);
    let alpha_n = end_slope * three - q;
    Addr(r32(c, V0)).write_fpu(c, alpha_n);
    csp_fset(c, &[(0, q), (1, yp), (2, hl), (3, alpha_n), (4, three)]);
    if c.g[A0][0] != 0 {
        c.cycles += 5;
        c.f[5] = c.f[4];
        c.g[A1][0] = sx(base.wrapping_add(CSP_ALPHA + 4));
        c.g[V1][0] = sx(ys.wrapping_add(4));
        c.g[A0][0] = sx(base.wrapping_add(CSP_H + 4));
        loop {
            c.cycles += 19;
            let a0 = r32(c, A0);
            let v = r32(c, V1);
            let a1 = r32(c, A1);
            let [dy1, alpha, dy0, p0, y0, _] = csp_alpha_step(c, a1, a0, v);
            c.g[A0][0] = sx(a0.wrapping_add(4));
            c.g[V1][0] = sx(v.wrapping_add(4));
            csp_fset(c, &[(0, alpha), (1, dy1), (2, dy0), (3, p0), (4, y0)]);
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = sx(a1.wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_dc18);
            }
        }
    }
    // 0x31dc64: forward sweep.
    c.cycles += 12;
    let h0 = Addr(base.wrapping_add(CSP_H)).read_fpu(c);
    c.g[T0][0] = 1;
    let a0v = Addr(base.wrapping_add(CSP_ALPHA)).read_fpu(c);
    c.g[V0][0] = csp_lt(c, T0, T2);
    let l0 = h0 + h0;
    c.g[AT][0] = 0x3f00_0000;
    let half = FpuF32(0x3f00_0000);
    Addr(base.wrapping_add(CSP_MU)).write_fpu(c, half);
    let z0 = a0v.div(l0);
    Addr(base.wrapping_add(CSP_L)).write_fpu(c, l0);
    Addr(base.wrapping_add(CSP_Z)).write_fpu(c, z0);
    csp_fset(c, &[(0, half), (1, l0), (2, z0)]);
    if c.g[V0][0] != 0 {
        c.cycles += 7;
        c.g[A3][0] = sx(r32(c, T4).wrapping_add(4));
        c.g[T1][0] = sx(r32(c, S2).wrapping_add(4));
        c.g[A2][0] = sx(r32(c, T5).wrapping_add(4));
        c.g[V1][0] = c.g[S1][0];
        c.g[A1][0] = sx(base);
        c.g[A0][0] = sx(r32(c, T8).wrapping_add(4));
        loop {
            c.cycles += 29;
            let a1 = r32(c, A1);
            let v1 = r32(c, V1);
            let a2 = r32(c, A2);
            let a3 = r32(c, A3);
            let t1 = r32(c, T1);
            let a0 = r32(c, A0);
            let x0 = Addr(a1).read_fpu(c);
            csp_add(c, T0, 1);
            let x2 = Addr(a1.wrapping_add(8)).read_fpu(c);
            c.g[V0][0] = csp_lt(c, T0, T2);
            let hm1 = Addr(v1).read_fpu(c);
            c.g[A1][0] = sx(a1.wrapping_add(4));
            let dx = x2 - x0;
            let mu0 = Addr(a2.wrapping_sub(4)).read_fpu(c);
            let hm = hm1 * mu0;
            let l = (dx + dx) - hm;
            Addr(a0).write_fpu(c, l);
            let h = Addr(v1.wrapping_add(4)).read_fpu(c);
            let mu = h.div(l);
            Addr(a2).write_fpu(c, mu);
            c.g[A2][0] = sx(a2.wrapping_add(4));
            let hm1b = Addr(v1).read_fpu(c);
            let zp = Addr(a3.wrapping_sub(4)).read_fpu(c);
            c.g[V1][0] = sx(v1.wrapping_add(4));
            let alpha = Addr(t1).read_fpu(c);
            let hz = hm1b * zp;
            let lr = Addr(a0).read_fpu(c);
            c.g[T1][0] = sx(t1.wrapping_add(4));
            c.g[A0][0] = sx(a0.wrapping_add(4));
            let z = (alpha - hz).div(lr);
            Addr(a3).write_fpu(c, z);
            let more = c.g[V0][0] != 0;
            c.g[A3][0] = sx(a3.wrapping_add(4));
            csp_fset(c, &[(0, z), (1, hz), (2, zp), (3, lr)]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_dcb0);
            }
        }
    }
    // 0x31dd24: l[n], z[n] = c[n]; back substitution.
    c.cycles += 25;
    let m4 = r32(c, S0) << 2;
    c.g[A2][0] = sx(m4);
    c.g[AT][0] = 0x4000_0000;
    let two = FpuF32(0x4000_0000);
    c.g[T5][0] = sx(r32(c, T5).wrapping_add(m4));
    c.g[A3][0] = sx(r32(c, S1).wrapping_add(m4));
    let mu = Addr(r32(c, T5)).read_fpu(c);
    c.g[V0][0] = sx(r32(c, T8).wrapping_add(t6));
    let h = Addr(r32(c, A3)).read_fpu(c);
    c.g[T1][0] = sx(r32(c, T4).wrapping_add(m4));
    c.g[V1][0] = sx(r32(c, S2).wrapping_add(t6));
    c.g[A0][0] = sx(r32(c, T4).wrapping_add(t6));
    c.g[A1][0] = sx(r32(c, T9).wrapping_add(t6));
    c.g[T0][0] = c.g[S0][0];
    let ln = h * (two - mu);
    Addr(r32(c, V0)).write_fpu(c, ln);
    let hb = Addr(r32(c, A3)).read_fpu(c);
    let zp = Addr(r32(c, T1)).read_fpu(c);
    let alpha = Addr(r32(c, V1)).read_fpu(c);
    let hz = hb * zp;
    let zn = (alpha - hz).div(ln);
    Addr(r32(c, A0)).write_fpu(c, zn);
    Addr(r32(c, A1)).write_fpu(c, zn);
    csp_fset(c, &[(0, zn), (1, hz), (2, zp), (3, ln)]);
    if (c.g[T0][0] as i64) < 0 {
        return;
    }
    c.cycles += 10;
    c.g[V0][0] = sx(m4.wrapping_add(4));
    c.g[A1][0] = sx(base.wrapping_add(0x2400).wrapping_add(m4));
    c.lwc1(6, r32(c, GP).wrapping_sub(CSP_THIRD_CLAMPED));
    c.g[V1][0] = c.g[A3][0];
    c.g[AT][0] = CSP_THREE as u64;
    c.f[5] = CSP_THREE;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T9)));
    c.g[A3][0] = c.g[T5][0];
    c.g[A0][0] = sx(m4.wrapping_add(r32(c, T7)));
    loop {
        c.cycles += 32;
        let a3 = r32(c, A3);
        let v0 = r32(c, V0);
        let t1 = r32(c, T1);
        let a0 = r32(c, A0);
        let v1 = r32(c, V1);
        let a1 = r32(c, A1);
        let mu = Addr(a3).read_fpu(c);
        csp_add(c, T0, u32::MAX);
        let c_next = Addr(v0).read_fpu(c);
        c.g[A3][0] = sx(a3.wrapping_sub(4));
        let z = Addr(t1).read_fpu(c);
        c.g[T1][0] = sx(t1.wrapping_sub(4));
        let cj = z - mu * c_next;
        Addr(v0.wrapping_sub(4)).write_fpu(c, cj);
        let y1 = Addr(a0.wrapping_add(4)).read_fpu(c);
        let y0 = Addr(a0).read_fpu(c);
        let s = c_next + (cj + cj);
        let h = Addr(v1).read_fpu(c);
        c.g[A0][0] = sx(a0.wrapping_sub(4));
        let hs6 = (h * s) * FpuF32(c.f[6]);
        let b = (y1 - y0).div(h) - hs6;
        Addr(a1.wrapping_sub(0x800)).write_fpu(c, b);
        let cn2 = Addr(v0).read_fpu(c);
        let cj2 = Addr(v0.wrapping_sub(4)).read_fpu(c);
        let hh = Addr(v1).read_fpu(c);
        c.g[V0][0] = sx(v0.wrapping_sub(4));
        c.g[V1][0] = sx(v1.wrapping_sub(4));
        let h3 = hh * FpuF32(c.f[5]);
        let d = (cn2 - cj2).div(h3);
        Addr(a1).write_fpu(c, d);
        let more = (c.g[T0][0] as i64) >= 0;
        c.g[A1][0] = sx(a1.wrapping_sub(4));
        csp_fset(c, &[(0, h3), (1, cj2), (2, d), (3, h), (4, y0)]);
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0031_ddb0);
        }
    }
}

// ---------------------------------------------------------------------------
// LunoVMRegister_WritePtr (0x224c00, lunovm.cpp)

/// The output stream's vtable slot `Write(this, const void* data, int n)`
/// (`{delta +8, fn +0xc}`).
const LVW_WRITE_SLOT: u32 = 0x8;

/// `void LunoVMRegister_WritePtr(LunoVMRegister* self, Stream* out, int base)`
/// (0x224c00): writes a pointer register as two words through the stream's
/// virtual `Write`: the pointer relative to `base`, then the register's
/// second word (+4). Frame 0x40 (the two words at 0 / 4, ra at 0x10, s1 /
/// s0 at 0x20 / 0x30).
pub fn luno_vm_register_write_ptr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[SP][0];
        let out = r32(c, S0);
        c.g[V1][0] = c.lw(out);
        c.g[V0][0] = c.lw(r32(c, S1));
        let vt = r32(c, V1);
        c.g[A0][0] = c.lh(vt.wrapping_add(LVW_WRITE_SLOT));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A2)));
        c.g[A3][0] = c.lw(vt.wrapping_add(LVW_WRITE_SLOT + 4));
        c.g[A0][0] = sx(out.wrapping_add(r32(c, A0)));
        c.sw(sp(c), c.g[V0][0]);
        c.g[A2][0] = 4;
        let method = r32(c, A3);
        call_ptr(c, method, 0x0022_4c40)?;
        let out = r32(c, S0);
        c.g[V1][0] = c.lw(out);
        c.g[A1][0] = c.g[SP][0] | 4;
        c.g[A3][0] = c.lw(r32(c, S1).wrapping_add(4));
        c.g[A2][0] = 4;
        let vt = r32(c, V1);
        c.g[A0][0] = c.lh(vt.wrapping_add(LVW_WRITE_SLOT));
        c.g[V0][0] = c.lw(vt.wrapping_add(LVW_WRITE_SLOT + 4));
        c.g[A0][0] = sx(out.wrapping_add(r32(c, A0)));
        let method = r32(c, V0);
        c.sw(sp(c).wrapping_add(4), c.g[A3][0]);
        call_ptr(c, method, 0x0022_4c64)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamFile_StartNextRequest (0x3e0270, seg_2DCAE8.c)

/// StreamFile fields: its mutex (+4), the read state (+0x38), the parse /
/// fill cursors (+0x54 / +0x58), the current request (+0x60), the open
/// file's name (+0x6c), its pending close/open file request (+0x16c / +0x174)
/// and the bytes left to read (+0x170).
const SNR_MUTEX: u32 = 0x4;
const SNR_READ_STATE: u32 = 0x38;
const SNR_CURSOR: u32 = 0x54;
const SNR_FILL: u32 = 0x58;
const SNR_REQUEST: u32 = 0x60;
const SNR_NAME: u32 = 0x6c;
const SNR_FILE: u32 = 0x16c;
const SNR_LEFT: u32 = 0x170;
const SNR_FILE_REQUEST: u32 = 0x174;
/// Request fields: state (+4; 1 = ready, 2 = started), next (+0xc), kind
/// (+0x10; 1 = in-memory), file name (+0x14), size (+0x118), the cursor at
/// its start (+0x120).
const SNR_REQ_STATE: u32 = 0x4;
const SNR_REQ_NEXT: u32 = 0xc;
const SNR_REQ_KIND: u32 = 0x10;
const SNR_REQ_NAME: u32 = 0x14;
const SNR_REQ_SIZE: u32 = 0x118;
const SNR_REQ_START: u32 = 0x120;
const SNR_STRCMP: u32 = 0x0041_65a8;
const SNR_STRCPY: u32 = 0x0041_66f4;
/// `StreamFile_OnOpen` / `StreamFile_OnClose` (request callbacks).
const SNR_ON_OPEN: u64 = 0x003e_00d8;
const SNR_ON_CLOSE: u64 = 0x003e_0118;
const SNR_QUEUE_OPEN: Callee = Callee {
    addr: 0x003d_d4e8,
    f: ee::s14::EAFile_QueueOpen,
};
const SNR_QUEUE_CLOSE: Callee = Callee {
    addr: 0x003d_d5a0,
    f: ee::s14::EAFile_QueueClose,
};
const SNR_SET_CALLBACK: Callee = Callee {
    addr: 0x003d_cf70,
    f: ee::s14::EAFile_SetRequestCallback,
};
const SNR_ISSUE_READ: Callee = Callee {
    addr: 0x003e_03d0,
    f: ee::s14::StreamFile_IssueRead,
};

/// `void StreamFile_StartNextRequest(StreamFile* self, int priority)`
/// (0x3e0270): under the stream's mutex, picks the request to stream (the
/// current one if ready, else the next queued one); with none, the stream
/// goes idle (read state 0). Otherwise the request is marked started at the
/// parse cursor, the fill cursor is reset to it and, for an in-memory
/// request or one on the already open file, reading starts at once
/// (`StreamFile_IssueRead`); a different file is remembered by name and
/// opened (or the old one closed first) through the file queue, with
/// `StreamFile_OnOpen` / `OnClose` as callbacks. Frame 0x50 (ra at 0, s3..s0
/// at 0x10..0x40).
pub fn stream_file_start_next_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(SNR_MUTEX));
        frame.save(c, S1, 0x30);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = 0;
        SFPC_LOCK.call(c, 0x003e_029c)?;
        let this = r32(c, S0);
        c.g[A0][0] = c.lw(this.wrapping_add(SNR_REQUEST));
        c.g[V0][0] = 1;
        // s1: no request to start.
        let mut idle = true;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            let req = r32(c, A0);
            c.g[V1][0] = c.lw(req.wrapping_add(SNR_REQ_STATE));
            c.g[S1][0] = 0;
            if c.g[V1][0] == c.g[V0][0] {
                idle = false;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(req.wrapping_add(SNR_REQ_NEXT));
                if c.g[V0][0] != 0 {
                    c.sw(this.wrapping_add(SNR_REQUEST), c.g[V0][0]);
                    idle = false;
                }
            }
        }
        if idle {
            c.cycles += 1;
            c.g[S1][0] = 1;
        }
        // 0x3e02c4
        c.cycles += 2;
        if c.g[S1][0] == 0 {
            c.g[S2][0] = c.lw(this.wrapping_add(SNR_REQUEST));
            c.cycles += 4;
            c.g[V0][0] = 2;
            c.g[V1][0] = c.lw(this.wrapping_add(SNR_CURSOR));
            let req = r32(c, S2);
            c.sw(req.wrapping_add(SNR_REQ_STATE), 2);
            c.sw(req.wrapping_add(SNR_REQ_START), c.g[V1][0]);
        } else {
            c.cycles += 3;
            c.sw(this.wrapping_add(SNR_READ_STATE), 0);
        }
        // 0x3e02e8
        c.cycles += 4;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(SNR_MUTEX));
        SFPC_UNLOCK.call(c, 0x003e_02f0)?;
        if c.g[S1][0] == 0 {
            c.cycles += 6;
            let this = r32(c, S0);
            c.g[V0][0] = c.lw(this.wrapping_add(SNR_CURSOR));
            c.g[A0][0] = 1;
            c.sw(this.wrapping_add(SNR_FILL), c.g[V0][0]);
            let req = r32(c, S2);
            c.g[V1][0] = c.lw(req.wrapping_add(SNR_REQ_KIND));
            let mut read_now = true;
            if c.g[V1][0] != c.g[A0][0] {
                // A file request: open its file unless it is already open.
                c.g[V0][0] = c.lw(req.wrapping_add(SNR_REQ_SIZE));
                c.cycles += 8;
                c.g[S1][0] = sx(this.wrapping_add(SNR_NAME));
                c.g[S2][0] = sx(req.wrapping_add(SNR_REQ_NAME));
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S2][0];
                c.sw(this.wrapping_add(SNR_LEFT), c.g[V0][0]);
                call_ptr(c, SNR_STRCMP, 0x003e_0330)?;
                let same = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S1][0];
                if !same {
                    read_now = false;
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, SNR_STRCPY, 0x003e_0340)?;
                    let this = r32(c, S0);
                    c.g[A0][0] = c.lw(this.wrapping_add(SNR_FILE));
                    c.g[A1][0] = c.g[S3][0];
                    let (queued, callback, ret) = if c.g[A0][0] == 0 {
                        c.cycles += 7;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[S3][0];
                        c.g[A1][0] = 1;
                        c.g[A3][0] = c.g[S0][0];
                        SNR_QUEUE_OPEN.call(c, 0x003e_0360)?;
                        (c.g[V0][0], SNR_ON_OPEN, 0x003e_0378)
                    } else {
                        c.cycles += 4;
                        c.g[A2][0] = c.g[S0][0];
                        SNR_QUEUE_CLOSE.call(c, 0x003e_0388)?;
                        (c.g[V0][0], SNR_ON_CLOSE, 0x003e_03a0)
                    };
                    c.sw(r32(c, S0).wrapping_add(SNR_FILE_REQUEST), queued);
                    if queued != 0 {
                        c.cycles += 6;
                        c.g[A1][0] = 0x003e_0000;
                        c.g[A0][0] = queued;
                        c.g[A1][0] = callback;
                        SNR_SET_CALLBACK.call(c, ret)?;
                    } else {
                        c.cycles += 1;
                    }
                }
            } else {
                c.cycles += 2;
                c.sw(this.wrapping_add(SNR_LEFT), 0);
            }
            if read_now {
                // 0x3e03a8
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S3][0];
                SNR_ISSUE_READ.call(c, 0x003e_03b4)?;
                c.cycles += 1;
            }
        }
        // 0x3e03b8
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
// WorldObj_dtor (0x34fbf0, instancenode.cpp)

/// WorldObj fields: vtable (+0xc) and model instance (+0x18; its flags at
/// +8). `D_00491C80` is the WorldObj vtable.
const WOD_VTABLE: u32 = 0xc;
const WOD_INSTANCE: u32 = 0x18;
const WOD_INST_FLAGS: u32 = 0x8;
const WOD_VTABLE_ADDR: u64 = 0x0049_1c80;
const WOD_UNLINK: Callee = Callee {
    addr: 0x0034_fc80,
    f: ee::s11::WorldObj_UnlinkInstance,
};
const WOD_BASE_DTOR: Callee = Callee {
    addr: 0x0035_4920,
    f: ee::s11::SyncProcess_dtor,
};

/// `WorldObj::~WorldObj(WorldObj* self, int flags)` (0x34fbf0): unlinks the
/// model instance from the object, resets its flags (keeps the high half
/// and bits 8-9, then ORs the high half into the low half and sets bit 1),
/// and chains to `SyncProcess_dtor`. Frame 0x30 (ra at 0, s1 / s0 at
/// 0x10 / 0x20).
pub fn world_obj_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = WOD_VTABLE_ADDR;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let this = r32(c, S0);
        c.sw(this.wrapping_add(WOD_VTABLE), c.g[V0][0]);
        c.g[A1][0] = c.lw(this.wrapping_add(WOD_INSTANCE));
        WOD_UNLINK.call(c, 0x0034_fc1c)?;
        let this = r32(c, S0);
        c.g[A3][0] = c.lw(this.wrapping_add(WOD_INSTANCE));
        c.g[V1][0] = 0xffff_ffff_ffff_0300;
        c.g[A0][0] = c.g[S0][0];
        let inst = r32(c, A3).wrapping_add(WOD_INST_FLAGS);
        c.g[V0][0] = c.lw(inst);
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] &= c.g[V1][0];
        c.sw(inst, c.g[V0][0]);
        c.g[A2][0] = c.lw(this.wrapping_add(WOD_INSTANCE));
        let inst = r32(c, A2).wrapping_add(WOD_INST_FLAGS);
        c.g[V0][0] = c.lw(inst);
        c.g[V1][0] = sx((r32(c, V0) as i32 >> 16) as u32);
        c.g[V0][0] |= c.g[V1][0];
        c.sw(inst, c.g[V0][0]);
        c.g[V1][0] = c.lw(this.wrapping_add(WOD_INSTANCE));
        let inst = r32(c, V1).wrapping_add(WOD_INST_FLAGS);
        c.g[V0][0] = c.lw(inst) | 2;
        c.sw(inst, c.g[V0][0]);
        WOD_BASE_DTOR.call(c, 0x0034_fc64)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Trick_BuildName (0x116950, seg_14B10.cpp `gGenTrickName`)

/// Where a trick-id field is read from (byte offset in the 8-byte id).
#[derive(Clone, Copy)]
enum TbnLoad {
    Word(u32),
    Half(u32),
    Byte(u32),
}

/// The string table a field indexes.
#[derive(Clone, Copy)]
enum TbnTable {
    Abs(u32),
    /// `t0 - 0x2ea0` (t0 = 0x440000: `D_0043D160`).
    T0,
    /// `gp - 0x2100` (`D_004A0FF0`).
    Gp,
}

/// One name part: `table[(load >> shift) & mask (<< 2)]` (the mask and
/// shift already produce a byte offset unless `shl2`), its header cycles
/// (10-cycle headers pay one more cycle before the copy loop), and the copy
/// loop's head.
#[derive(Clone, Copy)]
struct TbnPart {
    cycles: u64,
    load: TbnLoad,
    table: TbnTable,
    shift: u32,
    mask: Option<u64>,
    shl2: bool,
    head: u32,
}

const fn tbn(cycles: u64, load: TbnLoad, table: TbnTable, shift: u32, mask: Option<u64>, shl2: bool, head: u32) -> TbnPart {
    TbnPart { cycles, load, table, shift, mask, shl2, head }
}

/// Name parts 2..17 (part 1, the stance from bits 0-2, has its own shape).
const TBN_PARTS: [TbnPart; 16] = {
    use TbnLoad::*;
    use TbnTable::*;
    [
        tbn(10, Word(0), Abs(0x0043_d130), 1, Some(0x3c), false, 0x0011_69c8),
        tbn(10, Word(0), Abs(0x0043_cf40), 5, Some(0x1c), false, 0x0011_6a10),
        tbn(10, Word(0), Abs(0x0043_cf60), 8, Some(0xc), false, 0x0011_6a58),
        tbn(10, Word(0), Abs(0x0043_cf70), 10, Some(0x3c), false, 0x0011_6aa0),
        tbn(10, Half(2), Abs(0x0043_cfb0), 0, Some(0x7), true, 0x0011_6ae8),
        tbn(10, Word(0), Abs(0x0043_cfc8), 17, Some(0x1c), false, 0x0011_6b30),
        tbn(10, Word(0), Abs(0x0043_cfe0), 20, Some(0xfc), false, 0x0011_6b78),
        tbn(10, Word(0), Abs(0x0043_d0c0), 28, None, true, 0x0011_6bc0),
        tbn(9, Word(4), T0, 1, Some(0x1fc), false, 0x0011_6c00),
        tbn(9, Word(4), Gp, 8, Some(0x4), false, 0x0011_6c40),
        tbn(9, Word(4), T0, 9, Some(0x1fc), false, 0x0011_6c80),
        tbn(10, Word(4), Abs(0x0043_d2b8), 0, Some(0x7), true, 0x0011_6cc8),
        tbn(9, Half(6), Abs(0x0043_d2d0), 0, Some(0xc), false, 0x0011_6d08),
        tbn(10, Word(4), Abs(0x0043_d2e0), 18, Some(0x3c), false, 0x0011_6d50),
        tbn(10, Byte(7), Abs(0x0043_d100), 0, Some(0x7), true, 0x0011_6d98),
        tbn(10, Word(4), Abs(0x0043_d320), 27, None, true, 0x0011_6de0),
    ]
};

/// Copy the string at `a1` to `a0` (both advanced). The header loads the
/// first byte into `signed` (`lb`, tested) and `unsigned` (`lbu`, stored);
/// the loop tests `v0` and stores `unsigned`. `pre`: cycles before the loop.
fn tbn_copy(c: &mut Cpu, head: u32, signed: usize, unsigned: usize, pre: u64) {
    let a1 = r32(c, A1);
    c.g[signed][0] = c.lb(a1);
    c.g[unsigned][0] = c.lbu(a1);
    if c.g[signed][0] == 0 {
        return;
    }
    c.cycles += pre;
    loop {
        c.cycles += 7;
        c.sb(r32(c, A0), c.g[unsigned][0]);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
        let p = r32(c, A1);
        c.g[V0][0] = c.lb(p);
        c.g[unsigned][0] = c.lbu(p);
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `void Trick_BuildName(char* out, const TrickId* id)` (0x116950): builds
/// a trick's display name by concatenating 17 strings, each picked from a
/// name-part table by a bit field of the 8-byte trick id (stance, rotation,
/// flip, grab, modifiers, ...), then NUL-terminates it. A leaf.
pub fn trick_build_name(c: &mut Cpu) {
    // Part 1: the stance (bits 0-2 of the first word, `D_0043D118`).
    c.cycles += 12;
    c.g[A3][0] = c.g[A1][0];
    c.g[V0][0] = 0x0044_0000;
    let id = r32(c, A3);
    c.g[V1][0] = c.lw(id);
    c.g[V0][0] = sx(0x0043_d118);
    c.g[T0][0] = 0x0044_0000;
    c.g[V1][0] &= 7;
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.g[A1][0] = c.lw(r32(c, V1));
    tbn_copy(c, 0x0011_6980, V0, V1, 0);
    for part in TBN_PARTS {
        c.cycles += part.cycles;
        let id = r32(c, A3);
        c.g[V0][0] = match part.load {
            TbnLoad::Word(off) => c.lw(id.wrapping_add(off)),
            TbnLoad::Half(off) => c.lhu(id.wrapping_add(off)),
            TbnLoad::Byte(off) => c.lbu(id.wrapping_add(off)),
        };
        c.g[V1][0] = match part.table {
            TbnTable::Abs(a) => sx(a),
            TbnTable::T0 => sx(r32(c, T0).wrapping_sub(0x2ea0)),
            TbnTable::Gp => sx(r32(c, GP).wrapping_sub(0x2100)),
        };
        if part.shift != 0 {
            c.g[V0][0] = sx(r32(c, V0) >> part.shift);
        }
        if let Some(mask) = part.mask {
            c.g[V0][0] &= mask;
        }
        if part.shl2 {
            c.g[V0][0] = sx(r32(c, V0) << 2);
        }
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[A1][0] = c.lw(r32(c, V0));
        tbn_copy(c, part.head, V1, A2, (part.cycles == 10) as u64);
    }
    c.cycles += 2;
    c.sb(r32(c, A0), 0);
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{DATA, RA as RET_ADDR, SP as STACK_TOP};

    /// StreamFile_ParseChunks: a stream at DATA (header DATA+0x1000, three
    /// classifier entries at DATA+0x2000, class tallies at DATA+0x3000) with
    /// 0-4 chunks of known/unknown classes in a buffer at an unaligned
    /// DATA+0x4000+k, optionally ending in an end-of-stream chunk or a chunk
    /// whose length already carries a class tag, and a fill cursor that cuts
    /// the last chunk short or not. The header's state is sometimes 4
    /// (shutting down). The mutex syscalls go to the fakes (some unwind); one
    /// seed in seven re-enters at the loop head with the frame already open.
    #[test]
    fn stream_file_parse_chunks_matches() {
        use crate::difftest::fakes::{covered, machine};
        covered(ee::s14::f_003dfed0__generated, stream_file_parse_chunks, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3dfed0, 12);
            let this = DATA;
            let header = DATA + 0x1000;
            let classes = DATA + 0x2000;
            let tallies = DATA + 0x3000;
            let end_id = 0x454e_4421u32;
            let ids = [0x4141_0001u32, 0x4242_0002, end_id];
            c.g[A0][0] = this as u64;
            c.bus.write32(this + SFPC_HEADER, header);
            c.bus.write32(header + SFPC_HDR_STATE, if seed % 5 == 0 { 4 } else { r.next_u32() % 4 });
            c.bus.write32(header + SFPC_HDR_END_ID, end_id);
            // Classifier: {mask, value, class}; the end id is class 3.
            c.bus.write32(this + 0x1c, classes);
            c.bus.write32(this + 0x20, 3);
            for (i, id) in ids.iter().enumerate() {
                let e = classes + 12 * i as u32;
                c.bus.write32(e, 0xffff_ffff);
                c.bus.write32(e + 4, *id);
                c.bus.write32(e + 8, i as u32 + 1);
            }
            c.bus.write32(this + SFPC_TALLIES, tallies);
            for k in 0..3 {
                let n = if r.next_u32() % 2 == 0 { 0 } else { r.next_u32() & 0xfff };
                c.bus.write32(tallies + 16 * k + SFPC_TALLY_BYTES, n);
            }
            c.bus.write32(this + SFPC_PARSED, r.next_u32() & 0xfff);
            c.bus.write32(this + SFPC_LOW_WATER, r.next_u32() & 0xfff);
            c.bus.write32(this + SFPC_NEEDS_REFILL, 1);
            // Mutex: owner, depth, semaphore (owner never the fake thread id).
            c.bus.write32(this + 0x8, 0x7700_0000 | r.next_u32() & 0xffff);
            c.bus.write32(this + 0xc, r.next_u32() & 3);
            c.bus.write32(this + 0x10, r.next_u32() & 0xff);
            // Chunks.
            let start = DATA + 0x4000 + 4 * (r.next_u32() % 16);
            let mut p = start;
            for _ in 0..r.next_u32() % 5 {
                let id = match r.next_u32() % 3 {
                    0 => ids[0],
                    1 => ids[1],
                    _ => r.next_u32(),
                };
                let len = 8 + 4 * (r.next_u32() % 16);
                c.bus.write32(p, id);
                c.bus.write32(p + 4, len);
                p += len;
            }
            match r.next_u32() % 3 {
                0 => {
                    c.bus.write32(p, end_id);
                    c.bus.write32(p + 4, 8 + 4 * (r.next_u32() % 8));
                    p += 0x80;
                }
                1 => {
                    c.bus.write32(p, r.next_u32());
                    c.bus.write32(p + 4, (1 + r.next_u32() % 3) << 24 | 0x10);
                    p += 0x80;
                }
                _ => {}
            }
            c.bus.write32(this + SFPC_CURSOR, start);
            let fill = match r.next_u32() % 3 {
                0 => p.wrapping_sub(r.next_u32() % 0x40),
                _ => p,
            };
            c.bus.write32(this + SFPC_FILL, fill.max(start));
            c.pc = SFPC_ENTRY;
            if seed % 7 == 3 && fill >= start + 8 {
                // Re-entry at the loop head: frame open, first chunk loaded.
                let frame = STACK_TOP - 0x90;
                c.g[SP][0] = frame as u64;
                c.bus.write64(frame, RET_ADDR as u64);
                c.g[S0][0] = this as u64;
                c.g[S6][0] = header as u64;
                c.g[S7][0] = sx(0xffff_ffc0);
                c.g[S2][0] = start as u64;
                c.g[A0][0] = start as u64;
                c.g[S1][0] = c.bus.read32(start + 4) as u64 & 0xff_ffff;
                c.g[V1][0] = sx(c.bus.read32(start));
                c.pc = SFPC_HEAD;
            }
        });
    }

    /// BankMonitorInstance_FadeOut: already fading, or kind 1 / 2 / other.
    /// SND_Lock/SND_Unlock's lock callbacks are null (fakes, some unwind);
    /// the handle is negative or out of range, so the fade calls find no
    /// channel and return at once.
    #[test]
    fn bank_monitor_instance_fade_out_matches() {
        use crate::difftest::fakes::{covered, machine};
        covered(ee::s08::f_002ac180__generated, bank_monitor_instance_fade_out, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x2ac180, 4);
            c.pc = 0x002a_c180;
            let inst = DATA;
            c.g[A0][0] = inst as u64;
            c.bus.write32(inst + BMI_FADING, if seed % 5 == 0 { 1 + r.next_u32() % 3 } else { 0 });
            let kind = match seed % 4 {
                0 | 1 => 1,
                2 => 2,
                _ => r.next_u32() % 6,
            };
            c.bus.write32(inst + BMI_KIND, kind);
            c.bus.write32(inst + BMI_PLAYING, r.next_u32());
            let handle = if r.next_u32() % 2 == 0 { 0xffff_ffff } else { r.next_u32() & 0x7fff_ffff };
            c.bus.write32(inst + BMI_HANDLE, handle);
        });
    }

    /// WScriptFn_AddUVScrollModifier: defaults built or not (guard), 0-4
    /// script arguments (raw or converted to float; parameter types at
    /// 0x446238), object id -1 (the world), a null table slot, a zero handle
    /// or a real object; no model instance, or one whose virtual check is a
    /// fake (non-zero, some unwind) or a real leaf returning 0. The passing
    /// check runs the real `ModObj_AddUVScrollModifier` on the heap arena.
    #[test]
    fn wscript_fn_add_uv_scroll_modifier_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::{fill, GAME_GP};
        covered(ee::s10::f_002fe2c0__generated, wscript_fn_add_uv_scroll_modifier, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2fe2c0, 1);
            crate::game::be::bedbreward::tests::alloc_ready(c);
            c.pc = 0x002f_e2c0;
            let result = DATA + 0x10;
            let args = DATA + 0x100;
            let world = DATA + 0x1000;
            let obj = DATA + 0x2000;
            let table = DATA + 0x3000;
            let inst = DATA + 0x4000;
            let vt = DATA + 0x5000;
            let block = DATA + 0x6000;
            c.bus.write32(GAME_GP + WSUV_GUARD, (seed % 3 == 0) as u32);
            fill(c, &mut r, WSUV_DEFAULTS, 13);
            // (A built block's object id is always -1.)
            c.bus.write32(WSUV_DEFAULTS, 0xffff_ffff);
            fill(c, &mut r, 0x004c_9098, 3);
            for i in 0..13u32 {
                let t = if i == 0 { 1 } else { r.next_u32() % 4 };
                c.bus.write32(WSUV_PARAM_TYPES as u32 + 4 * i, t);
            }
            // Arguments.
            let n = match seed % 6 {
                0 => 0,
                1 => -(r.next_u32() as i32 % 3) - 1,
                _ => (1 + r.next_u32() % 4) as i32,
            };
            let id = match (seed / 2) % 5 {
                0 => None,
                1 => Some(r.next_u32() % 4 << 8),
                2 => Some((r.next_u32() % 4) << 8 | 1),
                _ => Some((1 + r.next_u32() % 3) << 8 | (1 + r.next_u32() % 3)),
            };
            for k in 0..4u32 {
                let a = args + 16 * k;
                let idx = if k == 0 && id.is_some() { 0 } else { 1 + r.next_u32() % 12 };
                c.bus.write32(a, idx);
                c.bus.write32(a + 4, if idx == 0 { id.unwrap() } else { r.next_u32() });
                c.bus.write32(a + 8, r.next_u32());
                c.bus.write32(a + 12, r.next_u32() % 4);
            }
            c.g[A0][0] = result as u64;
            c.g[A1][0] = sx(n as u32);
            c.g[A2][0] = args as u64;
            // Object resolution.
            c.bus.write32(GAME_GP + WSUV_WORLD, world);
            c.bus.write32(world + WSUV_WORLD_SELF, obj);
            c.bus.write32(GAME_GP + WSUV_OBJ_TABLE, table);
            c.bus.write32(table, table + 0x100);
            c.bus.write32(table + 0x108, table + 0x200);
            for j in 0..4u32 {
                let slot = if j == 0 { 0 } else { table + 0x300 + 0x10 * j };
                c.bus.write32(table + 0x200 + 4 * j, slot);
                if slot != 0 {
                    c.bus.write32(slot + 0x1c, table + 0x400);
                }
            }
            for k in 0..4u32 {
                let handle = if k == 0 { 0x12 } else { (obj >> 2) << 8 | (r.next_u32() & 0xff) };
                c.bus.write32(table + 0x400 + 4 * k, handle);
            }
            c.bus.write32(obj + 0xc, if seed % 7 == 0 { 0 } else { inst });
            c.bus.write32(inst + 0xc, vt);
            c.bus.write32(inst + 0x1c, block);
            c.bus.write32(block + 4, 0);
            c.bus.write16(vt + 0x80, (r.next_u32() % 4 * 4) as u16);
            c.bus.write32(vt + 0x84, if seed % 4 == 1 { 0x0014_06c0 } else { FAKE });
        });
    }

    /// AnimSeqList_PushAnim: a list with a few sequences; the real `PushNew`
    /// links the new one in (its allocator and constructor are logged fakes
    /// returning a node), the three track setters are logged fakes (one may
    /// unwind). Same fixture as `anim_seq_list_push_looping_anim_matches`.
    #[test]
    fn anim_seq_list_push_anim_matches() {
        use crate::difftest::fakes::{covered, machine};
        use crate::game::animation::animsequencer::tests::{asl_install_fakes, ASL_CTRL, ASL_MAGIC};
        asl_install_fakes();
        covered(ee::s10::f_00314518__generated, anim_seq_list_push_anim, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0031_4518, 5);
            c.pc = 0x0031_4518;
            c.bus.write32(ASL_CTRL, ASL_MAGIC);
            let list = DATA + 0x5_0000;
            c.bus.write32(list, r.next_u32() % 8);
            c.bus.write32(list + 4, if seed % 3 == 0 { 0 } else { DATA + 0x5_1000 });
            c.g[A0][0] = sx(list);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = sx(r.next_u32() % 0x400);
            c.f[12] = r.edge_f32();
        });
    }

    /// cCSICubicSpline_Solve: 0-12 knots (sometimes 40) with increasing x
    /// and random y, natural or clamped ends (random slopes), random other
    /// header bits; the two 1/3 `.lit4` entries set. A leaf.
    #[test]
    fn c_csi_cubic_spline_solve_matches() {
        use crate::difftest::{check, scramble, Rng, GAME_GP};
        check(ee::s10::f_0031d8b0__generated, c_csi_cubic_spline_solve, 300, &|c, seed| {
            let mut r = Rng(0x31d8_b000 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0031_d8b0;
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP - CSP_THIRD_NATURAL, (1.0f32 / 3.0).to_bits());
            c.bus.write32(GAME_GP - CSP_THIRD_CLAMPED, (1.0f32 / 3.0).to_bits());
            let spline = DATA;
            let knots = DATA + 0x100;
            let n = if seed % 11 == 0 { 40 } else { seed % 13 };
            let header = (r.next_u32() & !(0xff << 11) & !1) | n << 11 | (seed / 2) % 2;
            c.bus.write32(spline, header);
            c.bus.write32(spline + CSP_END_SLOPE, r.f32(4.0));
            c.bus.write32(spline + CSP_KNOTS, knots);
            let mut x = (r.next_u32() % 100) as f32 * 0.1;
            for k in 0..=n {
                let e = knots + CSP_KNOT_SIZE * k;
                let y = (r.next_u32() % 2000) as f32 * 0.01 - 10.0;
                // Knot k's {x, y} at +0/+4; the last knot's also at +8/+0xc.
                c.bus.write32(e, x.to_bits());
                c.bus.write32(e + 4, y.to_bits());
                if k + 1 == n {
                    c.bus.write32(e + 8, (x + 1.5).to_bits());
                    c.bus.write32(e + 0xc, y.to_bits());
                }
                c.bus.write32(e + 0x10, r.f32(2.0));
                x += 0.25 + (r.next_u32() % 300) as f32 * 0.01;
            }
            c.g[A0][0] = spline as u64;
        });
    }

    /// LunoVMRegister_WritePtr: a register at DATA+0x200000, a stream whose vtable's
    /// Write slot is a fake (logs both calls' registers and the stacked
    /// words; some unwind).
    #[test]
    fn luno_vm_register_write_ptr_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        covered(ee::s05::f_00224c00__generated, luno_vm_register_write_ptr, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x224c00, 2);
            c.pc = 0x0022_4c00;
            // (Away from the vtable's random spot in DATA..DATA+0xfffff.)
            let reg = DATA + 0x20_0000;
            let out = DATA + 0x20_0100;
            c.bus.write32(reg, r.next_u32());
            c.bus.write32(reg + 4, r.next_u32());
            let vt = vtable(c, &mut r, &[LVW_WRITE_SLOT]);
            c.bus.write32(out, vt);
            c.g[A0][0] = reg as u64;
            c.g[A1][0] = out as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }

    /// StreamFile_StartNextRequest: no current request, a ready one, a
    /// queued next one or none; in-memory or file requests whose file name
    /// matches the open file. The mutex syscalls go to the fakes (some
    /// unwind); the real `StreamFile_IssueRead` stalls at once (the ring
    /// cursors all at the buffer start). The name-change branch (strcpy and
    /// the file queue's open/close) is not reached: `EAFile_QueueOpen` /
    /// `QueueClose` need the whole file-request pool.
    #[test]
    fn stream_file_start_next_request_matches() {
        use crate::difftest::fakes::{covered, machine};
        covered(ee::s14::f_003e0270__generated, stream_file_start_next_request, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3e0270, 6);
            c.pc = 0x003e_0270;
            let this = DATA;
            let req = DATA + 0x1000;
            let next = DATA + 0x1400;
            let ring = DATA + 0x2200;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (r.next_u32() % 8) as u64;
            // Mutex: owner, depth, semaphore (owner never the fake thread id).
            c.bus.write32(this + 0x8, 0x7700_0000 | r.next_u32() & 0xffff);
            c.bus.write32(this + 0xc, r.next_u32() & 3);
            c.bus.write32(this + 0x10, r.next_u32() & 0xff);
            c.bus.write32(this + SNR_READ_STATE, 1 + r.next_u32() % 2);
            c.bus.write32(this + SNR_REQUEST, if seed % 5 == 0 { 0 } else { req });
            c.bus.write32(req + SNR_REQ_STATE, if seed % 3 == 0 { 1 } else { 2 + r.next_u32() % 3 });
            c.bus.write32(req + SNR_REQ_NEXT, if seed % 4 == 1 { 0 } else { next });
            for q in [req, next] {
                c.bus.write32(q + SNR_REQ_KIND, if r.next_u32() % 2 == 0 { 1 } else { 2 });
                c.bus.write32(q + SNR_REQ_SIZE, r.next_u32() & 0xffff);
                c.bus.write32(q + SNR_REQ_START, r.next_u32());
                for (i, b) in b"track01.big\0".iter().enumerate() {
                    c.bus.write8(q + SNR_REQ_NAME + i as u32, *b);
                }
            }
            for (i, b) in b"track01.big\0".iter().enumerate() {
                c.bus.write8(this + SNR_NAME + i as u32, *b);
            }
            c.bus.write32(this + SNR_LEFT, r.next_u32());
            c.bus.write32(this + SNR_FILE, 0);
            // StreamFile_IssueRead stall path.
            c.bus.write32(this + 0x30, ring);
            c.bus.write32(this + 0x34, DATA + 0x3000);
            c.bus.write32(this + 0x50, ring);
            c.bus.write32(this + SNR_CURSOR, ring);
            c.bus.write32(this + SNR_FILL, r.next_u32());
            c.bus.write32(this + 0x5c, DATA + 0x4000);
            c.bus.write32(DATA + 0x4000 + 0xc, 0);
            c.bus.write32(this + 0x17c, 0x1000);
        });
    }

    /// WorldObj_dtor: an object at DATA whose instance (DATA+0x100, random
    /// flags) points back at it or not; the real base destructors run
    /// (process type counter, process-list unlink with/without neighbours;
    /// flags without the delete bit, so nothing is freed).
    #[test]
    fn world_obj_dtor_matches() {
        use crate::difftest::{check, fill, scramble, Rng, GAME_GP};
        check(ee::s11::f_0034fbf0__generated, world_obj_dtor, 300, &|c, seed| {
            let mut r = Rng(0x34_fbf0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x0034_fbf0;
            c.g[GP][0] = GAME_GP as u64;
            let obj = DATA;
            let inst = DATA + 0x100;
            fill(c, &mut r, obj, 8);
            c.bus.write16(obj + 0x10, (r.next_u32() % 8) as u16);
            if seed % 3 == 0 {
                c.bus.write32(obj + 4, 0);
            } else {
                c.bus.write32(obj + 4, DATA + 0x200);
                c.bus.write32(obj + 8, if seed % 3 == 1 { 0 } else { DATA + 0x300 });
            }
            c.bus.write32(obj + WOD_INSTANCE, if seed % 7 == 0 { 0 } else { inst });
            c.bus.write32(inst + WOD_INST_FLAGS, r.next_u32());
            c.bus.write32(inst + 0xc, if seed % 2 == 0 { obj } else { r.next_u32() });
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (r.next_u32() & !1) as u64;
        });
    }

    /// Trick_BuildName: random trick ids into name-part tables
    /// whose slots point at empty or short strings (both copy shapes), and
    /// one seed in five on the game image's real tables with an all-zero id.
    #[test]
    fn trick_build_name_matches() {
        use crate::difftest::{check, elf_image, scramble, Rng, GAME_GP};
        check(ee::s00::f_00116950__generated, trick_build_name, 300, &|c, seed| {
            let mut r = Rng(0x11_6950 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x0011_6950;
            c.g[GP][0] = GAME_GP as u64;
            let id = DATA;
            let out = DATA + 0x100;
            let strings = DATA + 0x1000;
            c.bus.write32(id, r.next_u32());
            c.bus.write32(id + 4, r.next_u32());
            // Tables: every part's slots (largest index 0x1fc / 4).
            let tables = [
                0x0043_d118u32, 0x0043_d130, 0x0043_cf40, 0x0043_cf60, 0x0043_cf70,
                0x0043_cfb0, 0x0043_cfc8, 0x0043_cfe0, 0x0043_d0c0, 0x0043_d160,
                0x0043_d2b8, 0x0043_d2d0, 0x0043_d2e0, 0x0043_d100, 0x0043_d320,
                GAME_GP - 0x2100,
            ];
            for k in 0..4u32 {
                let s = strings + 0x10 * k;
                let len = if k == 0 { 0 } else { k + r.next_u32() % 6 };
                for i in 0..len {
                    c.bus.write8(s + i, b'a' + (r.next_u32() % 26) as u8);
                }
                c.bus.write8(s + len, 0);
            }
            if seed % 5 == 0 {
                // The real tables, at index 0 of every part.
                elf_image(c);
                c.bus.write32(id, 0);
                c.bus.write32(id + 4, 0);
            } else {
                for t in tables {
                    for slot in 0..0x80u32 {
                        c.bus.write32(t + 4 * slot, strings + 0x10 * (r.next_u32() % 4));
                    }
                }
            }
            c.g[A0][0] = out as u64;
            c.g[A1][0] = id as u64;
        });
    }
}
