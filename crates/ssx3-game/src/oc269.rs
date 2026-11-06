//! Staging wave 209 (oc269): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0022_4df0, c_luno_table_dtor, "cLunoTable_dtor"),
    (0x003b_b820, snd_parse_stream_header, "SND_ParseStreamHeader"),
    (0x002b_82b8, world_sound_compute_falloff, "WorldSound_ComputeFalloff"),
    (0x0026_b178, path_data_find_near_track_paths, "PathData_FindNearTrackPaths"),
    (0x003b_49d0, refpack_decode, "RefPack_Decode"),
    (0x003b_8530, snd_fade_volume, "SND_FadeVolume"),
    (0x003b_a740, snd_channel_calc_send, "SND_ChannelCalcSend"),
];
// ---------------------------------------------------------------------------
// cLunoTable_dtor (0x00224df0)

/// `cLunoTableRef::~cLunoTableRef(int flags)` (0x00224df0): drop one
/// reference to the shared Luno table (`self->table->refCount`, +0x8); the
/// last reference frees the table's data (`cLunoTable_FreeData(self)`).
/// `flags & 1` also deletes the reference itself (`operator delete`).
pub fn c_luno_table_dtor(c: &mut Cpu) {
    const TABLE_REFCOUNT: u32 = 0x8;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let this = r32(c, S0);
        let table = Addr(this).lw(c);
        c.g[V1][0] = table;
        let refs = sx((Addr(table as u32).off(TABLE_REFCOUNT).lw(c) as u32).wrapping_sub(1));
        c.g[V0][0] = refs;
        Addr(table as u32).off(TABLE_REFCOUNT).sw(c, refs);
        if refs == 0 {
            // Last reference: a0 is still `this`.
            c.cycles += 2;
            LUNO_TABLE_FREE_DATA.call(c, 0x0022_4e24)?;
        }
        c.cycles += 3;
        let delete_self = c.g[S1][0] & 1;
        c.g[V0][0] = delete_self;
        if delete_self != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0022_4e38)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cLunoTable_FreeData` (0x00224f30).
const LUNO_TABLE_FREE_DATA: Callee = Callee {
    addr: 0x0022_4f30,
    f: ee::s05::cLunoTable_FreeData,
};


// ---------------------------------------------------------------------------
// SND_ParseStreamHeader (0x003bb820)

/// The sound library's global state block (`D_0050A8E8`).
const PSH_SND_STATE: u32 = 0x0050_a8e8;
/// `u8` in the state block: the output channel layout (2 = stereo).
const PSH_STATE_OUT_CHANNELS: u32 = 0x47;
/// `u16 [channels][6]` in the state block: per-layout channel offsets.
const PSH_STATE_CHAN_TABLE: u32 = 0x2e0;

/// Stream header (`hdr`, s3): `u16 rate`, `u8 channels`, `u8 codec`.
const PSH_HDR_RATE: u32 = 0x0;
const PSH_HDR_CHANNELS: u32 = 0x2;
/// Patch params (`out`, s0): `u16 flags` at +8, channel offsets (`u16`) at
/// +0xc, marker tables at +0x48 / +0x58.
const PSH_OUT_FLAGS: u32 = 0x8;
const PSH_OUT_CHAN_OFF: u32 = 0xc;
const PSH_OUT_MARKER_DATA: u32 = 0x48;
const PSH_OUT_MARKER_SIZE: u32 = 0x58;
/// Extra info (`extra`, s2): word 0, then per-channel data offsets.
const PSH_EXTRA_OFFS: u32 = 0x4;

/// The tag record `SND_ParseNextTag` fills (at sp): read pointer, id, value,
/// data pointer, data size.
const PSH_REC_ID: u32 = 0x4;
const PSH_REC_VALUE: u32 = 0x8;
const PSH_REC_DATA: u32 = 0xc;
const PSH_REC_SIZE: u32 = 0x10;
/// The six per-channel base offsets (`vals`) on the stack.
const PSH_VALS: u32 = 0x20;

/// Where a tag's value goes.
#[derive(Clone, Copy)]
enum PshTag {
    /// `sb` of the value's low byte to `base + off`.
    Byte(usize, u32),
    /// `sh` of the value's low half.
    Half(usize, u32),
    /// `sw` of the value.
    Word(usize, u32),
    /// A copied data block: `out[data] = SND_MemAlloc(size)`, copy, `out[size]
    /// = size`; the block's address (for the return addresses).
    Data(u32, u32, u32),
    /// The reference offset pointer (`ref`, s5 = the data pointer).
    Ref,
    /// The output flags (s7 = the value).
    Flags,
    /// A marker (data pointer, size) appended to the marker tables (s6 = n).
    Marker,
}

/// The tag ids in the order the compiled compare chain tests them.
const PSH_TAGS: [(u64, PshTag); 29] = [
    (0xa0, PshTag::Byte(S3, 0x3)),
    (0x9c, PshTag::Word(SP, PSH_VALS)),
    (0x9d, PshTag::Word(SP, PSH_VALS + 0x4)),
    (0x9e, PshTag::Word(SP, PSH_VALS + 0x8)),
    (0x9f, PshTag::Word(SP, PSH_VALS + 0xc)),
    (0xa6, PshTag::Word(SP, PSH_VALS + 0x10)),
    (0xa7, PshTag::Word(SP, PSH_VALS + 0x14)),
    (0x98, PshTag::Data(0x18, 0x30, 0x003b_b9f4)),
    (0x99, PshTag::Data(0x1c, 0x34, 0x003b_ba24)),
    (0x9a, PshTag::Data(0x20, 0x38, 0x003b_ba54)),
    (0x9b, PshTag::Data(0x24, 0x3c, 0x003b_ba84)),
    (0xa4, PshTag::Data(0x28, 0x40, 0x003b_bab4)),
    (0xa5, PshTag::Data(0x2c, 0x44, 0x003b_bae4)),
    (0x80, PshTag::Byte(S0, 0x7)),
    (0x82, PshTag::Byte(S3, PSH_HDR_CHANNELS)),
    (0x84, PshTag::Half(S3, PSH_HDR_RATE)),
    (0x85, PshTag::Word(S2, 0x0)),
    (0x8a, PshTag::Ref),
    (0x13, PshTag::Byte(S0, 0x5)),
    (0x0a, PshTag::Byte(S0, 0x6)),
    (0x06, PshTag::Byte(S0, 0x2)),
    (0x8c, PshTag::Flags),
    (0x88, PshTag::Word(S2, PSH_EXTRA_OFFS)),
    (0x89, PshTag::Word(S2, PSH_EXTRA_OFFS + 0x4)),
    (0x94, PshTag::Word(S2, PSH_EXTRA_OFFS + 0x8)),
    (0x95, PshTag::Word(S2, PSH_EXTRA_OFFS + 0xc)),
    (0xa2, PshTag::Word(S2, PSH_EXTRA_OFFS + 0x10)),
    (0xa3, PshTag::Word(S2, PSH_EXTRA_OFFS + 0x14)),
    (0x14, PshTag::Marker),
];

/// `addu` of two registers' low words.
#[inline]
fn psh_addu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// Stages of [`snd_parse_stream_header`] (labels of the original).
#[derive(Clone, Copy)]
enum PshStage {
    /// 0x3bbc54: parse the next tag.
    NextTag,
    /// 0x3bb958: store the tag in v1 (loop head: a resume entry).
    Dispatch,
    /// 0x3bbc64: tags done, fix up the channels.
    Channels,
    /// 0x3bbcb0: one channel, counting down in a1 (a resume entry).
    Channel,
    /// 0x3bbd6c: epilogue.
    Exit,
}

/// `void SND_ParseStreamHeader(int base, const u8* src, StreamHdr* hdr,
/// PatchParams* out, StreamExtra* extra, u8* isGstr)` (0x003bb820): parse
/// an EA stream header ("SCHl"-style tag list after a 4-byte magic) into
/// the header, the patch params and the extra info. A "GSTR" header (not
/// "PT") goes to `SND_ParsePtHeader` instead and sets `*isGstr`. After the
/// tags, every channel gets its output offset from the layout table, and
/// its data offset made absolute (`base + `) or relative to the reference
/// offset (`*ref`), depending on the output flags.
///
/// Dispatcher-shaped: an event at the tag loop's or the channel loop's back
/// edge may unwind; the trampoline re-enters at 0x3bb958 / 0x3bbcb0 with
/// the loop state in the registers.
pub fn snd_parse_stream_header(c: &mut Cpu) {
    const START: u32 = 0x003b_b820;
    let frame = Frame { size: 0xe0 };
    guest(c, |c| {
        let mut stage = match c.pc {
            START => psh_prologue(c, frame)?,
            0x003b_b958 => PshStage::Dispatch,
            0x003b_bcb0 => PshStage::Channel,
            pc => {
                c.jump_out(pc, START, 0x003b_bd9c);
                return Ok(());
            }
        };
        loop {
            stage = match stage {
                PshStage::NextTag => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[SP][0];
                    SND_PARSE_NEXT_TAG.call(c, 0x003b_bc5c)?;
                    let more = c.g[V0][0] != 0;
                    c.g[V1][0] = Addr(sp(c)).off(PSH_REC_ID).lw(c);
                    if !more {
                        PshStage::Channels
                    } else if back_edge_unwinds(c, 0x003b_b958) {
                        return Ok(());
                    } else {
                        PshStage::Dispatch
                    }
                }
                PshStage::Dispatch => {
                    psh_store_tag(c)?;
                    PshStage::NextTag
                }
                PshStage::Channels => {
                    c.cycles += 5;
                    let out = r32(c, S0);
                    let flags = c.g[S7][0];
                    Addr(out).off(PSH_OUT_FLAGS).sh(c, flags);
                    let channels = Addr(r32(c, S3)).off(PSH_HDR_CHANNELS).lbu(c);
                    let ch = sx((channels as u32).wrapping_sub(1));
                    c.g[A1][0] = ch;
                    c.g[V0][0] = 0x51_0000;
                    if (ch as i64) < 0 {
                        PshStage::Exit
                    } else {
                        // Loop registers: a1 = ch, a3 = &vals[ch], a2 =
                        // &extra->offs[ch], t1 = &out->chanOff[ch], t2 = the
                        // state block, t6 = its channel table.
                        c.cycles += 14;
                        c.g[A0][0] = sx((ch as u32) << 2);
                        c.g[T2][0] = sx(PSH_SND_STATE);
                        c.g[V1][0] = sx((ch as u32) << 1);
                        c.g[V0][0] = psh_addu(c.g[A0][0], 4);
                        c.g[V1][0] = psh_addu(c.g[V1][0], PSH_OUT_CHAN_OFF as u64);
                        c.g[A0][0] = psh_addu(c.g[A0][0], c.g[SP][0]);
                        c.g[T6][0] = sx(PSH_SND_STATE + PSH_STATE_CHAN_TABLE);
                        c.g[A3][0] = psh_addu(c.g[A0][0], PSH_VALS as u64);
                        c.g[A2][0] = psh_addu(c.g[V0][0], c.g[S2][0]);
                        c.g[T1][0] = psh_addu(c.g[V1][0], c.g[S0][0]);
                        c.g[T5][0] = 2;
                        c.g[T4][0] = 0xc;
                        c.g[T3][0] = psh_addu(c.g[S2][0], PSH_EXTRA_OFFS as u64);
                        PshStage::Channel
                    }
                }
                PshStage::Channel => {
                    psh_channel(c);
                    c.cycles += 4;
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(2));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                    let more = (c.g[A1][0] as i64) >= 0;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(4));
                    if !more {
                        PshStage::Exit
                    } else if back_edge_unwinds(c, 0x003b_bcb0) {
                        return Ok(());
                    } else {
                        PshStage::Channel
                    }
                }
                PshStage::Exit => {
                    c.cycles += 12;
                    frame.restore(c, S0, 0xd0);
                    frame.restore(c, S1, 0xc0);
                    frame.restore(c, S2, 0xb0);
                    frame.restore(c, S3, 0xa0);
                    frame.restore(c, S4, 0x90);
                    frame.restore(c, S5, 0x80);
                    frame.restore(c, S6, 0x70);
                    frame.restore(c, S7, 0x60);
                    frame.restore(c, FP, 0x50);
                    frame.restore_ra(c, 0x40);
                    frame.close(c);
                    return Ok(());
                }
            };
        }
    });
}

/// `memset` (0x416210, through the function table).
const PSH_MEMSET: u32 = 0x0041_6210;
const SND_PARSE_NEXT_TAG: Callee = Callee {
    addr: 0x003c_7388,
    f: ee::s13::SND_ParseNextTag,
};
const SND_PARSE_PT_HEADER: Callee = Callee {
    addr: 0x003b_bda0,
    f: ee::s13::SND_ParsePtHeader,
};
const SND_INIT_PATCH_PARAMS: Callee = Callee {
    addr: 0x003c_7218,
    f: ee::s13::SND_InitPatchParams,
};
const SND_MEM_ALLOC: Callee = Callee {
    addr: 0x003b_5c60,
    f: ee::s13::SND_MemAlloc,
};
const SND_MEM_COPY: Callee = Callee {
    addr: 0x003c_51e8,
    f: ee::s13::SND_MemCopy,
};

/// Frame, clears, the magic check (GSTR goes to `SND_ParsePtHeader`) and
/// the header defaults; returns where the tag loop starts.
fn psh_prologue(c: &mut Cpu, frame: Frame) -> Result<PshStage, crate::call::Unwound> {
    c.cycles += 40;
    Frame::open(c, frame.size);
    frame.save(c, S1, 0xc0);
    frame.save(c, S3, 0xa0);
    frame.save(c, FP, 0x50);
    frame.save(c, S0, 0xd0);
    frame.save(c, S2, 0xb0);
    frame.save(c, S4, 0x90);
    frame.save(c, S5, 0x80);
    frame.save(c, S6, 0x70);
    frame.save(c, S7, 0x60);
    frame.save_ra(c, 0x40);
    c.g[S1][0] = c.g[A1][0]; // src
    c.g[S3][0] = c.g[A2][0]; // hdr
    c.g[FP][0] = c.g[A0][0]; // base
    c.g[S0][0] = c.g[A3][0]; // out
    c.g[S2][0] = c.g[T0][0]; // extra
    c.g[S4][0] = c.g[T1][0]; // isGstr
    // memset(vals, 0, 0x18); memset(hdr, 0, 4); memset(out, 0, 0x68);
    // memset(extra, 0, 0x1c). n = 0, flags = 8, ref = 0.
    c.g[A0][0] = psh_addu(c.g[SP][0], PSH_VALS as u64);
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x18;
    call_ptr(c, PSH_MEMSET, 0x003b_b874)?;
    c.g[S6][0] = 0;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x4;
    c.g[S7][0] = 0x8;
    call_ptr(c, PSH_MEMSET, 0x003b_b88c)?;
    c.g[S5][0] = 0;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x68;
    call_ptr(c, PSH_MEMSET, 0x003b_b8a0)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x1c;
    call_ptr(c, PSH_MEMSET, 0x003b_b8b0)?;

    // "PT" is a stream header; "GSTR" (checked as the compiled chain does)
    // is a PT header.
    let src = Addr(r32(c, S1));
    let b0 = src.lbu(c);
    c.g[A0][0] = b0;
    c.g[V0][0] = 0x47;
    let gstr = 'magic: {
        if b0 == b'P' as u64 {
            c.cycles += 4;
            let b1 = src.off(1).lbu(c);
            c.g[V1][0] = b1;
            if b1 == b'T' as u64 {
                break 'magic false;
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0x53;
        if b0 != b'G' as u64 {
            break 'magic false;
        }
        for (i, (want, next)) in [(b'S', 0x54u64), (b'T', 0x52), (b'R', 0)]
            .into_iter()
            .enumerate()
        {
            c.cycles += 3;
            let b = src.off(1 + i as u32).lbu(c);
            c.g[V1][0] = b;
            if i < 2 {
                c.g[V0][0] = next;
            } else {
                c.g[A1][0] = c.g[S1][0];
            }
            if b != want as u64 {
                break 'magic false;
            }
        }
        true
    };
    if gstr {
        c.cycles += 7;
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = c.g[S2][0];
        c.g[A0][0] = 0;
        SND_PARSE_PT_HEADER.call(c, 0x003b_b910)?;
        c.g[V0][0] = 1;
        if c.g[S4][0] != 0 {
            c.cycles += 2;
            Addr(r32(c, S4)).sb(c, 1);
        }
        return Ok(PshStage::Exit);
    }
    c.cycles += 2;
    if c.g[S4][0] != 0 {
        Addr(r32(c, S4)).sb(c, 0);
    }
    c.cycles += 12;
    c.g[A0][0] = c.g[S0][0];
    SND_INIT_PATCH_PARAMS.call(c, 0x003b_b930)?;
    // Defaults: 22050 Hz, mono, codec 5; the tag reader starts after the
    // magic; extra->f0 = 0.
    c.g[V1][0] = 0x5622;
    c.g[V0][0] = 0x1;
    c.g[A0][0] = 0x5;
    let hdr = Addr(r32(c, S3));
    hdr.off(PSH_HDR_RATE).sh(c, 0x5622);
    hdr.off(PSH_HDR_CHANNELS).sb(c, 1);
    let p = psh_addu(c.g[S1][0], 4);
    c.g[A1][0] = p;
    hdr.off(0x3).sb(c, 5);
    Addr(sp(c)).sw(c, p);
    Addr(r32(c, S2)).sw(c, 0);
    Ok(PshStage::NextTag)
}

/// The compare chain on the tag id (v1) and the store it selects (0x3bb958).
fn psh_store_tag(c: &mut Cpu) -> crate::call::Flow {
    let id = c.g[V1][0];
    c.g[A0][0] = id;
    let rec = Addr(sp(c));
    for (k, &(tag, action)) in PSH_TAGS.iter().enumerate() {
        c.cycles += if k < 2 { 3 } else { 2 };
        // The delay slot loads the next id (the first block's moves a0).
        c.g[V0][0] = match k {
            0 => 0xa0,
            28 => sx((c.g[S6][0] as u32) << 2),
            _ => PSH_TAGS[k + 1].0,
        };
        if id != tag {
            continue;
        }
        match action {
            PshTag::Byte(base, off) => {
                c.cycles += 3;
                let v = rec.off(PSH_REC_VALUE).lbu(c);
                c.g[V0][0] = v;
                Addr(r32(c, base)).off(off).sb(c, v);
            }
            PshTag::Half(base, off) => {
                c.cycles += 3;
                let v = rec.off(PSH_REC_VALUE).lhu(c);
                c.g[V0][0] = v;
                Addr(r32(c, base)).off(off).sh(c, v);
            }
            PshTag::Word(base, off) => {
                c.cycles += 3;
                let v = rec.off(PSH_REC_VALUE).lw(c);
                c.g[V0][0] = v;
                Addr(r32(c, base)).off(off).sw(c, v);
            }
            PshTag::Data(data, size, block) => {
                c.cycles += 10;
                c.g[A0][0] = rec.off(PSH_REC_SIZE).lw(c);
                SND_MEM_ALLOC.call(c, block + 0x8)?;
                let rec = Addr(sp(c));
                c.g[A1][0] = rec.off(PSH_REC_DATA).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = rec.off(PSH_REC_SIZE).lw(c);
                let mem = c.g[V0][0];
                Addr(r32(c, S0)).off(data).sw(c, mem);
                SND_MEM_COPY.call(c, block + 0x1c)?;
                let n = Addr(sp(c)).off(PSH_REC_SIZE).lw(c);
                c.g[V0][0] = n;
                Addr(r32(c, S0)).off(size).sw(c, n);
            }
            PshTag::Ref => {
                c.cycles += 2;
                c.g[S5][0] = rec.off(PSH_REC_DATA).lw(c);
            }
            PshTag::Flags => {
                c.cycles += 2;
                c.g[S7][0] = rec.off(PSH_REC_VALUE).lw(c);
            }
            PshTag::Marker => {
                c.cycles += 7;
                let data = rec.off(PSH_REC_DATA).lw(c);
                c.g[A0][0] = data;
                let entry = psh_addu(c.g[S0][0], c.g[V0][0]);
                c.g[V1][0] = entry;
                let size = rec.off(PSH_REC_SIZE).lw(c);
                c.g[A1][0] = size;
                Addr(entry as u32).off(PSH_OUT_MARKER_DATA).sw(c, data);
                c.g[V0][0] = entry;
                Addr(entry as u32).off(PSH_OUT_MARKER_SIZE).sw(c, size);
                c.g[S6][0] = psh_addu(c.g[S6][0], 1);
            }
        }
        return Ok(());
    }
    Ok(())
}

/// One channel (a1 = ch) of the fix-up loop, 0x3bbcb0..0x3bbd5c (without
/// the counter step): the output offset from the layout table plus the
/// channel's base (zeroed for stereo output of a stereo stream), and the
/// data offset made absolute or relative to `*ref`.
fn psh_channel(c: &mut Cpu) {
    let state = Addr(r32(c, T2));
    let hdr = Addr(r32(c, S3));
    let out = Addr(r32(c, S0));
    c.cycles += 3;
    let layout = state.off(PSH_STATE_OUT_CHANNELS).lbu(c);
    c.g[V1][0] = layout;
    c.g[T0][0] = sx((c.g[A1][0] as u32) << 2);
    if layout == c.g[T5][0] {
        c.cycles += 3;
        let channels = hdr.off(PSH_HDR_CHANNELS).lbu(c);
        c.g[V0][0] = channels;
        if channels == layout {
            Addr(r32(c, A3)).sw(c, 0);
        }
    }
    // out->chanOff[ch] = table[channels - 1][ch] + (u16)vals[ch]
    c.cycles += 14;
    c.g[V0][0] = hdr.off(PSH_HDR_CHANNELS).lbu(c);
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 1);
    c.g[A0][0] = Addr(r32(c, A3)).lhu(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    c.mult(0, V0, V0, T4);
    c.g[V1][0] = psh_addu(c.g[V1][0], c.g[V0][0]);
    c.g[V1][0] = psh_addu(c.g[V1][0], c.g[T6][0]);
    c.g[V0][0] = Addr(r32(c, V1)).lhu(c);
    c.g[V0][0] = psh_addu(c.g[V0][0], c.g[A0][0]);
    let off = c.g[V0][0];
    Addr(r32(c, T1)).sh(c, off);
    let flags = out.off(PSH_OUT_FLAGS).lhu(c);
    c.g[V1][0] = flags;
    c.g[V0][0] = flags & 0x14;
    // Absolute offsets (flags & 0x14, or no flags): offs[ch] += base.
    let absolute = if c.g[V0][0] != 0 {
        c.g[V1][0] = psh_addu(c.g[T3][0], c.g[T0][0]);
        true
    } else {
        c.cycles += 2;
        if flags != 0 {
            c.g[V0][0] = out.off(PSH_OUT_FLAGS).lhu(c);
            false
        } else {
            c.cycles += 1;
            c.g[V1][0] = psh_addu(c.g[T3][0], c.g[T0][0]);
            true
        }
    };
    if absolute {
        c.cycles += 4;
        let slot = Addr(r32(c, V1));
        let v = psh_addu(c.g[FP][0], slot.lw(c));
        c.g[V0][0] = v;
        slot.sw(c, v);
        c.g[V0][0] = out.off(PSH_OUT_FLAGS).lhu(c);
    }
    // Relative to the reference (flags & 0x308, ref set).
    c.cycles += 3;
    c.g[V0][0] &= 0x308;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        if c.g[S5][0] != 0 {
            c.cycles += 2;
            let extra = Addr(r32(c, S2));
            let reference = Addr(r32(c, S5));
            if c.g[A1][0] != 0 {
                c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                c.cycles += 5;
                c.g[A0][0] = extra.off(PSH_EXTRA_OFFS).lw(c);
                c.g[V1][0] = reference.lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                c.g[V1][0] = psh_addu(c.g[V1][0], c.g[V0][0]);
                let v = c.g[V1][0];
                Addr(r32(c, A2)).sw(c, v);
            } else {
                c.cycles += 3;
                let v = reference.lw(c);
                c.g[V0][0] = v;
                extra.off(PSH_EXTRA_OFFS).sw(c, v);
            }
            c.cycles += 1;
        }
    }
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(4));
}


// ---------------------------------------------------------------------------
// WorldSound_ComputeFalloff (0x002b82b8)

/// The falloff curve jump table (`.rodata`, six case addresses).
const FALLOFF_CURVES: u32 = 0x0048_35e0;
/// `.lit4` (gp-relative): the "hold full volume" knee (0.7) and the slope
/// after it (1 / 0.3).
const FALLOFF_KNEE: u32 = 0x4534;
const FALLOFF_KNEE_SLOPE: u32 = 0x4530;

/// `float WorldSound_ComputeFalloff(float curve, float t)` (0x002b82b8): the
/// volume of a world sound at normalised distance `t` (f13) for falloff
/// curve `(int)curve` (f12), dispatched through a jump table:
/// 0: `1 - t*t`; 1: `1 - t/(1.5 - t/2)`; 2: `1 - t`; 3: `u/(1.5 - u/2)` with
/// `u = 1 - t`; 4: `(1 - t)^2`; 5: 1 up to the knee, then `(1 - t) / 0.3`;
/// any other curve: 0. The result is in f0 (and f13).
pub fn world_sound_compute_falloff(c: &mut Cpu) {
    const ONE: u64 = 0x3f80_0000;
    const HALF: u64 = 0x3f00_0000;
    const ONE_AND_HALF: u64 = 0x3fc0_0000;
    c.cycles += 5;
    let curve = abi::farg(c, 0).to_i32() as u32;
    c.f[0] = curve;
    c.g[V1][0] = sx(curve);
    let in_range = c.g[V1][0] < 6;
    c.g[V0][0] = in_range as u64;
    c.g[V0][0] = 0x48_0000;
    let mut t = abi::farg(c, 1);
    if !in_range {
        c.cycles += 1;
        t = FpuF32::ZERO;
    } else {
        c.cycles += 6;
        c.g[V1][0] = sx(curve << 2);
        c.g[V0][0] = sx(FALLOFF_CURVES);
        c.g[V1][0] = sx((curve << 2).wrapping_add(FALLOFF_CURVES));
        c.g[A0][0] = Addr((curve << 2).wrapping_add(FALLOFF_CURVES)).lw(c);
        let target = c.g[A0][0] as u32;
        match target {
            // 0: 1 - t*t
            0x002b_82e4 => {
                c.cycles += 2 + 4;
                t = t * t;
                c.g[AT][0] = ONE;
                c.f[0] = ONE as u32;
                t = FpuF32::ONE - t;
            }
            // 1: 1 - t / (1.5 - t/2)
            0x002b_82ec => {
                c.cycles += 11;
                c.g[AT][0] = ONE_AND_HALF;
                let half_t = t * FpuF32(HALF as u32);
                c.f[0] = half_t.0;
                c.g[AT][0] = ONE;
                c.f[2] = ONE as u32;
                let d = FpuF32(ONE_AND_HALF as u32) - half_t;
                c.f[1] = d.0;
                t = FpuF32::ONE - t.div(d);
            }
            // 2: 1 - t
            0x002b_8318 => {
                c.cycles += 4;
                c.g[AT][0] = ONE;
                c.f[0] = ONE as u32;
                t = FpuF32::ONE - t;
            }
            // 3: u / (1.5 - u/2), u = 1 - t (returns directly).
            0x002b_8328 => {
                c.cycles += 12;
                c.f[2] = ONE as u32;
                let u = FpuF32::ONE - t;
                c.g[AT][0] = ONE_AND_HALF;
                let half_u = u * FpuF32(HALF as u32);
                c.f[1] = half_u.0;
                let d = FpuF32(ONE_AND_HALF as u32) - half_u;
                let r = u.div(d);
                c.f[13] = r.0;
                c.f[0] = r.0;
                abi::ret(c);
                return;
            }
            // 4: (1 - t)^2
            0x002b_8358 => {
                c.cycles += 5;
                c.g[AT][0] = ONE;
                c.f[0] = ONE as u32;
                let u = FpuF32::ONE - t;
                t = u * u;
            }
            // 5: full volume up to the knee, then a linear ramp to 0.
            0x002b_836c => {
                c.cycles += 4;
                let knee = Addr(r32(c, GP).wrapping_sub(FALLOFF_KNEE)).read_fpu(c);
                c.f[0] = knee.0;
                let held = t.le(knee);
                abi::set_fcc(c, held);
                c.g[AT][0] = ONE;
                if held {
                    // (returns directly)
                    c.cycles += 4;
                    c.f[13] = ONE as u32;
                    c.f[0] = ONE as u32;
                    abi::ret(c);
                    return;
                }
                c.cycles += 6;
                c.f[0] = ONE as u32;
                let slope = Addr(r32(c, GP).wrapping_sub(FALLOFF_KNEE_SLOPE)).read_fpu(c);
                c.f[1] = slope.0;
                t = (FpuF32::ONE - t) * slope;
            }
            _ => {
                c.jump_out(target, 0x002b_82b8, 0x002b_83b4);
                return;
            }
        }
    }
    c.cycles += 3;
    c.f[13] = t.0;
    c.f[0] = t.0;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// PathData_FindNearTrackPaths (0x0026b178)

/// Path data: track path count and the path array (0x3c bytes per path,
/// bounds at +0x1c).
const NEAR_PATHS_COUNT: u32 = 0x10;
const NEAR_PATHS_ITEMS: u32 = 0x14;
const NEAR_PATHS_STRIDE: u32 = 0x3c;
/// `.lit4` (gp-relative): the "already taken" distance (1e37), loaded twice.
const NEAR_PATHS_TAKEN_LIT: u32 = 0x4a44;
const NEAR_PATHS_TAKEN_LIT2: u32 = 0x4a40;

/// `addu` of two registers' low words.
#[inline]
fn near_paths_addu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

const PATH_BOUNDS_DISTANCE: Callee = Callee {
    addr: 0x0026_a9b0,
    f: ee::s07::Path_BoundsDistance,
};

/// `int PathData_FindNearTrackPaths(PathData* self, const Vec3* pos,
/// TrackPath** out, int max)` (0x0026b178): collect up to `min(count, max)`
/// track paths near `pos`: first every path whose bounds contain `pos`
/// (distance 0) in order, then the remaining slots by increasing bounds
/// distance (selection by repeated minimum search over a distance array on
/// the stack; a taken path's distance becomes 1e37). Returns the number
/// written.
pub fn path_data_find_near_track_paths(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x180);
        frame.save(c, S0, 0x160);
        frame.save(c, S2, 0x140);
        frame.save(c, S5, 0x110);
        frame.save(c, S6, 0x100);
        frame.save(c, S7, 0xf0);
        frame.save(c, FP, 0xe0);
        frame.save(c, S1, 0x150);
        frame.save(c, S3, 0x130);
        frame.save(c, S4, 0x120);
        frame.save_ra(c, 0xd0);
        frame.save_fpr(c, 21, 0x178);
        frame.save_fpr(c, 20, 0x170);
        c.g[S0][0] = 0; // found
        c.g[S2][0] = 0; // i
        c.g[S5][0] = c.g[A0][0]; // self
        c.g[S7][0] = c.g[A2][0]; // out
        c.g[FP][0] = c.g[A1][0]; // pos
        let count = Addr(r32(c, S5)).off(NEAR_PATHS_COUNT).lw(c);
        c.g[V0][0] = count;
        let max = abi::arg64(c, 3);
        c.g[V1][0] = ((count as i64) < (max as i64)) as u64;
        // want = min(count, max) (s6)
        c.g[S6][0] = if (count as i64) < (max as i64) { count } else { max };

        // Pass 1: distances; paths containing pos are taken in order.
        if (count as i64) > 0 {
            c.cycles += 5;
            c.f[21] = 0;
            c.g[S1][0] = c.g[SP][0]; // &dist[i]
            let taken = Addr(r32(c, GP).wrapping_sub(NEAR_PATHS_TAKEN_LIT)).read_fpu(c);
            c.f[20] = taken.0;
            c.g[S4][0] = 0; // i * stride
            c.g[S3][0] = c.g[S7][0]; // &out[found]
            loop {
                c.cycles += 8;
                let items = Addr(r32(c, S5)).off(NEAR_PATHS_ITEMS).lw(c);
                c.g[A1][0] = c.g[FP][0];
                c.g[A0][0] = near_paths_addu(items, c.g[S4][0]);
                PATH_BOUNDS_DISTANCE.call(c, 0x0026_b1f8)?;
                let dist = FpuF32(c.f[0]);
                let inside = dist.eq_s(FpuF32(c.f[21]));
                abi::set_fcc(c, inside);
                Addr(r32(c, S1)).write_fpu(c, dist);
                if inside {
                    c.cycles += 7;
                    let items = Addr(r32(c, S5)).off(NEAR_PATHS_ITEMS).lw(c);
                    c.g[S0][0] = near_paths_addu(c.g[S0][0], 1);
                    let taken = FpuF32(c.f[20]);
                    Addr(r32(c, S1)).write_fpu(c, taken);
                    let path = near_paths_addu(items, c.g[S4][0]);
                    c.g[V0][0] = path;
                    Addr(r32(c, S3)).sw(c, path);
                    let full = c.g[S0][0] == c.g[S6][0];
                    c.g[S3][0] = near_paths_addu(c.g[S3][0], 4);
                    if full {
                        break;
                    }
                }
                c.cycles += 6;
                let count = Addr(r32(c, S5)).off(NEAR_PATHS_COUNT).lw(c);
                c.g[S2][0] = near_paths_addu(c.g[S2][0], 1);
                c.g[S1][0] = near_paths_addu(c.g[S1][0], 4);
                let more = (c.g[S2][0] as i64) < (count as i64);
                c.g[V0][0] = more as u64;
                c.g[S4][0] = near_paths_addu(c.g[S4][0], NEAR_PATHS_STRIDE as u64);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_b1e8);
                }
            }
        }

        // Pass 2: fill the remaining slots with the nearest untaken paths.
        c.cycles += 3;
        let short = (c.g[S0][0] as i64) < (c.g[S6][0] as i64);
        c.g[V0][0] = c.g[S0][0];
        if short {
            c.cycles += 2;
            let taken = Addr(r32(c, GP).wrapping_sub(NEAR_PATHS_TAKEN_LIT2)).read_fpu(c);
            c.f[2] = taken.0;
            c.g[T1][0] = NEAR_PATHS_STRIDE as u64;
            loop {
                c.cycles += 8;
                let count = Addr(r32(c, S5)).off(NEAR_PATHS_COUNT).lw(c);
                c.g[V1][0] = count;
                c.g[S2][0] = 1;
                c.g[T0][0] = near_paths_addu(c.g[S0][0], 1);
                c.f[1] = Addr(sp(c)).read_fpu(c).0; // best = dist[0]
                c.g[A3][0] = 0; // best index
                c.g[V0][0] = (1 < count as i64) as u64;
                c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                if 1 < count as i64 {
                    c.cycles += 2;
                    c.g[A0][0] = count;
                    c.g[V1][0] = near_paths_addu(c.g[SP][0], 4);
                    loop {
                        c.cycles += 5;
                        let d = Addr(r32(c, V1)).read_fpu(c);
                        c.f[0] = d.0;
                        let nearer = d.lt(FpuF32(c.f[1]));
                        abi::set_fcc(c, nearer);
                        if nearer {
                            c.cycles += 3;
                            c.f[1] = d.0;
                            c.g[A3][0] = c.g[S2][0];
                        }
                        c.g[S2][0] = near_paths_addu(c.g[S2][0], 1);
                        c.cycles += 3;
                        let more = (c.g[S2][0] as i64) < (c.g[A0][0] as i64);
                        c.g[V0][0] = more as u64;
                        c.g[V1][0] = near_paths_addu(c.g[V1][0], 4);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0026_b278);
                        }
                    }
                }
                // out[found] = &items[best]; dist[best] = taken.
                c.cycles += 11;
                let best = r32(c, A3);
                let offset = abi::mult(c, best, NEAR_PATHS_STRIDE);
                c.g[A2][0] = sx(offset);
                let items = Addr(r32(c, S5)).off(NEAR_PATHS_ITEMS).lw(c);
                c.g[V0][0] = sx(best << 2);
                c.g[A1][0] = near_paths_addu(c.g[A1][0], c.g[S7][0]);
                c.g[V1][0] = near_paths_addu(c.g[SP][0], c.g[V0][0]);
                c.g[S0][0] = c.g[T0][0];
                let taken = FpuF32(c.f[2]);
                Addr(r32(c, V1)).write_fpu(c, taken);
                let more = (c.g[S0][0] as i64) < (c.g[S6][0] as i64);
                c.g[V0][0] = more as u64;
                let path = near_paths_addu(items, c.g[A2][0]);
                c.g[A0][0] = path;
                Addr(r32(c, A1)).sw(c, path);
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S0][0];
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_b250);
                }
            }
        }

        c.cycles += 14;
        frame.restore(c, S1, 0x150);
        frame.restore(c, S0, 0x160);
        frame.restore(c, S2, 0x140);
        frame.restore(c, S3, 0x130);
        frame.restore(c, S4, 0x120);
        frame.restore(c, S5, 0x110);
        frame.restore(c, S6, 0x100);
        frame.restore(c, S7, 0xf0);
        frame.restore(c, FP, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.restore_fpr(c, 21, 0x178);
        frame.restore_fpr(c, 20, 0x170);
        frame.close(c);
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// RefPack_Decode (0x003b49d0)

/// `sx(a + b)` on the low words (`addu`/`addiu`).
#[inline]
fn refpack_addu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// One RefPack byte copy loop: `do { *dst++ (t1) = *src++; } while (--v1
/// != end)` with `end` holding -1, an event check at `head` per byte.
#[inline]
fn refpack_copy(c: &mut Cpu, src: usize, end: usize, head: u32) {
    loop {
        c.cycles += 7;
        let b = Addr(r32(c, src)).lbu(c);
        c.g[V0][0] = b;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.g[src][0] = refpack_addu(c.g[src][0], 1);
        Addr(r32(c, T1)).sb(c, b);
        let more = c.g[V1][0] != c.g[end][0];
        c.g[T1][0] = refpack_addu(c.g[T1][0], 1);
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `int RefPack_Decode(u8* dst, const u8* src, int* consumed)`
/// (0x003b49d0): EA RefPack (QFS) decompression. The header gives the
/// unpacked size (3 or 4 big-endian bytes, after an optional compressed
/// size when flag 0x100 is set); then control bytes: `0xxxxxxx` (2-byte
/// form: 0-3 literals, a 3-10 byte match up to 1 KiB back), `10xxxxxx`
/// (3-byte form: 4-67 bytes up to 16 KiB back), `110xxxxx` (4-byte form:
/// 5-1028 bytes up to 128 KiB back), `111xxxxx` below 0xfc (4-112
/// literals), and 0xfc-0xff (0-3 literals, end). Returns the unpacked size
/// (0 for a null `src`) and stores the bytes read to `*consumed` if given.
/// Registers as compiled: t1 = dst, a3 = src, t0 = control byte, t7 =
/// size, t6 = -1.
pub fn refpack_decode(c: &mut Cpu) {
    c.cycles += 4;
    c.g[T1][0] = c.g[A0][0];
    c.g[A3][0] = c.g[A1][0];
    c.g[T7][0] = 0;
    if c.g[A1][0] != 0 {
        refpack_decode_stream(c);
    }
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[A1][0] as u32));
    if c.g[A2][0] != 0 {
        c.cycles += 1;
        let consumed = c.g[V0][0];
        Addr(r32(c, A2)).sw(c, consumed);
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T7][0];
    abi::ret(c);
}

/// The header and the control-byte loop of [`refpack_decode`].
fn refpack_decode_stream(c: &mut Cpu) {
    // Header: 16-bit flags, an optional packed size, the unpacked size.
    c.cycles += 7;
    let src = r32(c, A1);
    let h0 = Addr(src).lbu(c);
    c.g[V0][0] = Addr(src).off(1).lbu(c);
    let flags = refpack_addu(h0 << 8, c.g[V0][0]);
    c.g[V1][0] = flags;
    c.g[A0][0] = flags & 0x8000;
    c.g[A3][0] = sx(src.wrapping_add(2));
    let four = c.g[A0][0] != 0;
    c.cycles += if four { 18 } else { 13 };
    c.g[V1][0] = flags & 0x100;
    c.g[V0][0] = sx(src.wrapping_add(if four { 6 } else { 5 }));
    if c.g[V1][0] != 0 {
        c.g[A3][0] = c.g[V0][0];
    }
    // t7 = big-endian size (v1/a0 alternate as the byte registers).
    let mut p = r32(c, A3);
    let mut size = Addr(p).lbu(c);
    p = p.wrapping_add(1);
    for (i, reg) in [V1, A0, V1].into_iter().enumerate() {
        if i == 2 && !four {
            break;
        }
        let b = Addr(p).lbu(c);
        c.g[reg][0] = b;
        c.g[V0][0] = sx((size as u32) << 8);
        p = p.wrapping_add(1);
        size = refpack_addu(c.g[V0][0], b);
    }
    c.g[T7][0] = size;
    c.g[A3][0] = sx(p);

    c.cycles += 4;
    c.g[T6][0] = u64::MAX;
    c.g[T2][0] = refpack_addu(c.g[A3][0], 1);
    'stream: loop {
        c.cycles += 1;
        c.g[T0][0] = Addr(r32(c, A3)).lbu(c);
        loop {
            // t0 = control byte at a3; t2 = a3 + 1.
            c.cycles += 3;
            let ctl = c.g[T0][0];
            c.g[V0][0] = ctl & 0x80;
            c.g[A3][0] = c.g[T2][0];
            if ctl & 0x80 == 0 {
                // 0ooccrrr oooooooo: r literals, then c+3 bytes from o+1 back.
                c.cycles += 7;
                c.g[A0][0] = Addr(r32(c, A3)).lbu(c);
                c.g[V1][0] = ctl & 0x3;
                c.g[T3][0] = ctl & 0x1c;
                c.g[A3][0] = refpack_addu(c.g[A3][0], 1);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                c.g[T2][0] = ctl & 0x60;
                if c.g[V1][0] != c.g[T6][0] {
                    c.cycles += 3;
                    c.g[T0][0] = u64::MAX;
                    refpack_copy(c, A3, T0, 0x003b_4ac0);
                }
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[T2][0] as u32) << 3);
                c.g[V1][0] = sx((c.g[T3][0] as u32) >> 2);
                c.g[V0][0] = !(c.g[A0][0] | c.g[V0][0]);
                c.g[A0][0] = refpack_addu(c.g[V0][0], c.g[T1][0]);
                c.g[V1][0] = refpack_addu(c.g[V1][0], 2);
                c.g[T2][0] = refpack_addu(c.g[A3][0], 1);
                c.g[T0][0] = u64::MAX;
                refpack_copy(c, A0, T0, 0x003b_4b00);
            } else {
                c.cycles += 3;
                c.g[V0][0] = ctl & 0x20;
                if ctl & 0x40 == 0 {
                    // 10cccccc rroooooo oooooooo: r literals, c+4 bytes.
                    c.cycles += 8;
                    c.g[A0][0] = Addr(r32(c, A3)).lbu(c);
                    c.g[T2][0] = ctl & 0x3f;
                    c.g[A3][0] = refpack_addu(c.g[A3][0], 1);
                    c.g[T3][0] = Addr(r32(c, A3)).lbu(c);
                    c.g[V1][0] = sx((c.g[A0][0] as u32) >> 6);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                    let none = c.g[V1][0] == c.g[T6][0];
                    c.g[A3][0] = refpack_addu(c.g[A3][0], 1);
                    if !none {
                        c.cycles += 2;
                        c.g[T0][0] = u64::MAX;
                        refpack_copy(c, A3, T0, 0x003b_4b58);
                    }
                    c.cycles += 9;
                    c.g[V0][0] = c.g[A0][0] & 0x3f;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                    c.g[V1][0] = refpack_addu(c.g[T2][0], 3);
                    c.g[V0][0] = !(c.g[T3][0] | c.g[V0][0]);
                    c.g[T2][0] = refpack_addu(c.g[A3][0], 1);
                    c.g[A0][0] = refpack_addu(c.g[V0][0], c.g[T1][0]);
                    c.g[T0][0] = u64::MAX;
                    refpack_copy(c, A0, T0, 0x003b_4b98);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = ctl & 0x1f;
                    if ctl & 0x20 == 0 {
                        // 110occrr oooooooo oooooooo cccccccc.
                        c.cycles += 11;
                        c.g[A0][0] = Addr(r32(c, A3)).lbu(c);
                        c.g[V1][0] = ctl & 0x3;
                        c.g[A3][0] = refpack_addu(c.g[A3][0], 1);
                        c.g[T5][0] = ctl & 0xc;
                        c.g[T3][0] = Addr(r32(c, A3)).lbu(c);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                        c.g[A3][0] = refpack_addu(c.g[A3][0], 1);
                        c.g[T2][0] = ctl & 0x10;
                        c.g[T4][0] = Addr(r32(c, A3)).lbu(c);
                        let none = c.g[V1][0] == c.g[T6][0];
                        c.g[A3][0] = refpack_addu(c.g[A3][0], 1);
                        if !none {
                            c.cycles += 2;
                            c.g[T0][0] = u64::MAX;
                            refpack_copy(c, A3, T0, 0x003b_4bf8);
                        }
                        c.cycles += 13;
                        c.g[V0][0] = (c.g[T2][0] != 0) as u64;
                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 8);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                        c.g[V1][0] = sx((c.g[T5][0] as u32) << 6);
                        c.g[V0][0] = refpack_addu(c.g[V0][0], c.g[A0][0]);
                        c.g[V1][0] = refpack_addu(c.g[V1][0], c.g[T4][0]);
                        c.g[V0][0] = !(c.g[T3][0] | c.g[V0][0]);
                        c.g[A0][0] = refpack_addu(c.g[V0][0], c.g[T1][0]);
                        c.g[V1][0] = refpack_addu(c.g[V1][0], 4);
                        c.g[T2][0] = refpack_addu(c.g[A3][0], 1);
                        c.g[T0][0] = u64::MAX;
                        refpack_copy(c, A0, T0, 0x003b_4c48);
                    } else {
                        // 111rrrrr: (r+1)*4 literals, or the end code.
                        c.cycles += 5;
                        c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V1][0] = refpack_addu(c.g[A0][0], 4);
                        let literals = c.g[V1][0] < 0x71;
                        c.g[V0][0] = literals as u64;
                        c.g[V1][0] = refpack_addu(c.g[A0][0], 3);
                        if !literals {
                            break 'stream;
                        }
                        c.cycles += 2;
                        let none = c.g[V1][0] == c.g[T6][0];
                        c.g[T2][0] = refpack_addu(c.g[A3][0], 1);
                        if !none {
                            c.cycles += 2;
                            c.g[A0][0] = u64::MAX;
                            refpack_copy(c, A3, A0, 0x003b_4c90);
                            c.cycles += 2;
                            c.g[T2][0] = refpack_addu(c.g[A3][0], 1);
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003b_4a88);
                        }
                        continue 'stream;
                    }
                }
            }
            // After a match: the next control byte.
            c.cycles += 2;
            c.g[T0][0] = Addr(r32(c, A3)).lbu(c);
            if event_due(c) {
                deliver_event_here(c, 0x003b_4a8c);
            }
        }
    }
    // End code: 0-3 trailing literals.
    c.cycles += 4;
    c.g[V1][0] = c.g[T0][0] & 0x3;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
    if c.g[V1][0] != c.g[T6][0] {
        c.cycles += 3;
        c.g[A0][0] = u64::MAX;
        refpack_copy(c, A3, A0, 0x003b_4cd0);
    }
}


// ---------------------------------------------------------------------------
// SND_FadeVolume (0x003b8530)

/// The sound state block and its channel table (`+0x1f0`, 0x8c bytes per
/// channel: `+0x30` volume step, `+0x34` target volume, `+0x38` volume, all
/// `volume << 16`).
const FADE_SND_STATE: u32 = 0x0050_a8e8;
const FADE_CHANNELS: u32 = 0x1f0;
const FADE_CHANNEL_SIZE: u64 = 0x8c;
const FADE_STEP: u32 = 0x30;
const FADE_TARGET: u32 = 0x34;
const FADE_VOLUME: u32 = 0x38;

const FADE_HANDLE_TO_CHANNEL: Callee = Callee {
    addr: 0x003b_a6e8,
    f: ee::s13::SND_HandleToChannel,
};
const FADE_NEXT_HANDLE_CHANNEL: Callee = Callee {
    addr: 0x003b_aa00,
    f: ee::s13::SND_NextHandleChannel,
};

/// `int SND_FadeVolume(int handle, int frames, unsigned volume)`
/// (0x003b8530): start a volume fade on every channel of a handle: the
/// target becomes `volume << 16` and the per-frame step `(target - volume)
/// / frames` (at least one frame). Returns the handle's first channel, or
/// the error from `SND_HandleToChannel`.
///
/// Dispatcher-shaped: an unwind at the channel loop's back edge re-enters
/// at 0x3b8588 (registers: s0 frames, s1 target, s2 first channel, s3
/// 0x8c, s4 the state block, v1 the channel; the cursor also at `0(sp)`).
pub fn snd_fade_volume(c: &mut Cpu) {
    const START: u32 = 0x003b_8530;
    const HEAD: u32 = 0x003b_8588;
    let frame = Frame { size: 0x70 };
    let resumed = match c.pc {
        START => false,
        HEAD => true,
        pc => {
            c.jump_out(pc, START, 0x003b_85ec);
            return;
        }
    };
    guest(c, |c| {
        let mut more = true;
        if !resumed {
            c.cycles += 13;
            Frame::open(c, frame.size);
            frame.save(c, S0, 0x60);
            frame.save(c, S1, 0x50);
            frame.save(c, S2, 0x40);
            frame.save(c, S3, 0x30);
            frame.save(c, S4, 0x20);
            frame.save_ra(c, 0x10);
            c.g[S0][0] = c.g[A1][0]; // frames
            c.g[S1][0] = c.g[A2][0]; // volume
            FADE_HANDLE_TO_CHANNEL.call(c, 0x003b_8558)?;
            c.g[S2][0] = c.g[V0][0];
            let positive = 0 < c.g[S0][0] as i64;
            c.g[V1][0] = positive as u64;
            if (c.g[S2][0] as i64) < 0 {
                more = false;
            } else {
                c.cycles += 9;
                c.g[A1][0] = 1;
                c.g[A0][0] = u64::MAX;
                c.g[V0][0] = 0x51_0000;
                c.g[S4][0] = sx(FADE_SND_STATE);
                if !positive {
                    c.g[S0][0] = 1;
                }
                c.sw(sp(c), u64::MAX); // cursor = -1
                c.g[S1][0] = sx((c.g[S1][0] as u32) << 16);
                c.g[S3][0] = FADE_CHANNEL_SIZE;
                more = fade_next_channel(c)?;
                if more && back_edge_unwinds(c, HEAD) {
                    return Ok(());
                }
            }
        }
        while more {
            // The cursor's channel (v1): target and step.
            c.cycles += 8;
            let table = Addr(r32(c, S4)).off(FADE_CHANNELS).lw(c);
            c.mult(0, V1, V1, S3);
            let ch = sx((table as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = ch;
            let ch = Addr(ch as u32);
            let volume = ch.off(FADE_VOLUME).lw(c);
            let target = c.g[S1][0];
            ch.off(FADE_TARGET).sw(c, target);
            c.g[V0][0] = sx((target as u32).wrapping_sub(volume as u32));
            c.cycles += 3;
            c.div(0, V0, S0);
            c.g[V0][0] = c.lo[0];
            let step = c.g[V0][0];
            ch.off(FADE_STEP).sw(c, step);
            more = fade_next_channel(c)?;
            if more && back_edge_unwinds(c, HEAD) {
                return Ok(());
            }
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `SND_NextHandleChannel(first, &cursor)` for [`snd_fade_volume`]: true
/// when there is another channel (its index then in v1).
fn fade_next_channel(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    c.cycles += 5;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[SP][0];
    FADE_NEXT_HANDLE_CHANNEL.call(c, 0x003b_85c0)?;
    let more = c.g[V0][0] != 0;
    c.g[V1][0] = Addr(sp(c)).lw(c);
    Ok(more)
}


// ---------------------------------------------------------------------------
// SND_ChannelCalcSend (0x003ba740)

/// The channel table pointer (`SND state + 0x1f0`, 0x8c bytes per channel:
/// `+0x24` voice flags, per send slot `+0x60/+0x61` signed send levels and
/// `+0x62` the combined level, `u16` per slot).
const CALC_SEND_CHANNELS: u32 = 0x0050_a8e8 + 0x1f0;
const CALC_SEND_FLAGS: u32 = 0x24;

const SND_GET_FX_PARAMS: Callee = Callee {
    addr: 0x003b_8218,
    f: ee::s13::SND_GetFxParams,
};

/// `void SND_ChannelCalcSend(int slot, int channel)` (0x003ba740): the
/// effect send level of send slot `slot` of a channel: the product of the
/// channel's two signed send bytes for that slot and the effect's own
/// level (`SND_GetFxParams(slot, flags)[2]`), divided by 64, stored as the
/// slot's `s16` send.
pub fn snd_channel_calc_send(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        c.g[V0][0] = 0x8c;
        let frame = Frame::open(c, 0x30);
        c.mult(0, A1, A1, V0); // channel * 0x8c
        frame.save(c, S0, 0x20);
        c.g[V1][0] = 0x51_0000;
        frame.save(c, S1, 0x10);
        let table = Addr(CALC_SEND_CHANNELS).lw(c);
        c.g[S1][0] = c.g[A0][0]; // slot
        frame.save_ra(c, 0);
        let ch = sx((table as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[S0][0] = ch;
        c.g[A1][0] = Addr(ch as u32).off(CALC_SEND_FLAGS).lhu(c);
        SND_GET_FX_PARAMS.call(c, 0x003b_a770)?;
        // The slot's two send bytes (channel + slot).
        let sends = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[A2][0] = sends;
        frame.restore_ra(c, 0);
        let b = Addr(sends as u32).off(0x61).lb(c);
        c.g[A1][0] = b;
        let a = Addr(sends as u32).off(0x60).lb(c);
        c.g[V1][0] = a;
        c.g[S1][0] = sx((c.g[S1][0] as u32) << 1);
        c.g[A0][0] = Addr(r32(c, V0)).off(2).lb(c); // effect level
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.mult(0, V1, V1, A1);
        frame.restore(c, S1, 0x10);
        c.mult(0, V1, V1, A0);
        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 6) as u32);
        let level = c.g[V1][0];
        Addr(r32(c, S0)).off(0x62).sh(c, level);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    });
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::{check, elf_image, put_words, Rng, DATA, GAME_GP};
    use crate::game::be::beintreward::tests::world;
    use crate::game::bx::bxstring::tests::{alloc, heap};
    use crate::game::bx::bxstring2::tests::be_library;

    /// cLunoTable_dtor: a heap-allocated reference to a heap-allocated table
    /// with no data. Refcount 1 (last reference: the table is freed) or more,
    /// and `flags & 1` (delete the reference) on or off.
    #[test]
    fn c_luno_table_dtor_matches() {
        check(ee::s05::f_00224df0__generated, c_luno_table_dtor, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x224df0, 0);
            be_library(c);
            world(c);
            heap(c);
            let this = alloc(c, 0x10);
            let table = alloc(c, 0x20);
            for o in (0..0x20).step_by(4) {
                c.bus.write32(table + o, 0);
            }
            c.bus.write32(this, table);
            let refs = if seed % 2 == 0 { 1 } else { 2 + r.next_u32() % 5 };
            c.bus.write32(table + 0x8, refs);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = ((r.next_u32() & !1) | (seed >> 1) & 1) as u64;
            c.g[RA][0] = crate::difftest::RA as u64;
            c.pc = 0x0022_4df0;
        });
    }

    /// The sound library's state block for SND_ParseStreamHeader: an empty
    /// SND heap (`STATE+0x234`) for the data tags, the output layout byte and
    /// the per-layout channel offset table.
    fn psh_sound_state(c: &mut Cpu, r: &mut Rng, seed: u32) {
        const HEAP: u32 = DATA + 0x8000;
        put_words(c, PSH_SND_STATE + 0x234, &[HEAP]);
        // base, block table (empty, grows down), limit, low water, lowest.
        put_words(c, HEAP, &[DATA + 0x20_0000, DATA + 0x9000, 0, 0x4000, 0x40, 0]);
        let layout = if seed % 3 == 0 { r.next_u32() % 4 } else { 2 };
        c.bus.write8(PSH_SND_STATE + PSH_STATE_OUT_CHANNELS, layout as u8);
        for i in 0..36 {
            c.bus
                .write16(PSH_SND_STATE + PSH_STATE_CHAN_TABLE + 2 * i, r.next_u32() as u16);
        }
    }

    /// A stream header at `src`: the magic, then random tags of the
    /// parser's (id, length, big-endian value / data) form, padding (0xfc)
    /// and length-less ids (0xfd/0xfe), ended by 0xff.
    ///
    /// `events`: a plain "SCHl" stream of at least 8 tags without the data
    /// tags, so no callee that can unwind on its own (memset/
    /// `SND_ParsePtHeader`/`SND_MemCopy` dispatchers) runs once the event is
    /// due; the test harness re-enters only the function under test.
    fn psh_stream(c: &mut Cpu, r: &mut Rng, seed: u32, src: u32, events: bool) {
        let mut b: Vec<u8> = Vec::new();
        match if events { 7 } else { seed % 8 } {
            0 => b.extend_from_slice(b"GSTR\0\0\0\0"),
            1 => b.extend_from_slice(b"PTxx"),
            2 => b.extend_from_slice(b"GSTx"),
            3 => b.extend_from_slice(b"GSxR"),
            4 => b.extend_from_slice(b"PxTR"),
            _ => b.extend_from_slice(b"SCHl"),
        }
        let ids: Vec<u8> = PSH_TAGS.iter().map(|t| t.0 as u8).collect();
        let n = if events { 8 + r.next_u32() % 8 } else { r.next_u32() % 14 };
        let mut markers = 0;
        for _ in 0..n {
            match r.next_u32() % 12 {
                0 => b.push(0xfc),
                1 => b.push(0xfd + (r.next_u32() % 2) as u8),
                _ => {
                    let mut id = if r.next_u32() % 6 == 0 {
                        r.next_u32() as u8 % 0xf0
                    } else {
                        ids[(r.next_u32() as usize) % ids.len()]
                    };
                    if events && matches!(id, 0x98..=0x9b | 0xa4 | 0xa5) {
                        id = 0x88;
                    }
                    if id == 0x14 {
                        markers += 1;
                        if markers > 4 {
                            id = 0x13;
                        }
                    }
                    let len = 1 + r.next_u32() % 4;
                    let mut data: Vec<u8> = (0..len).map(|_| r.next_u32() as u8).collect();
                    match id {
                        // Channels: 1..6, sometimes 0 (no channel loop).
                        0x82 => {
                            data = vec![(r.next_u32() % 7) as u8];
                        }
                        // Output flags: the bits the channel loop tests.
                        0x8c => {
                            let f = [0u16, 0x14, 0x10, 0x4, 0x308, 0x100, 0x8, 0x1c];
                            let v = f[(r.next_u32() as usize) % f.len()];
                            data = vec![(v >> 8) as u8, v as u8];
                        }
                        _ => {}
                    }
                    b.push(id);
                    b.push(data.len() as u8);
                    b.extend_from_slice(&data);
                }
            }
        }
        b.push(0xff);
        for (i, &x) in b.iter().enumerate() {
            c.bus.write8(src + i as u32, x);
        }
    }

    /// Arguments of SND_ParseStreamHeader(base, src, hdr, out, extra, isGstr).
    fn psh_setup(c: &mut Cpu, r: &mut Rng, seed: u32, events: bool) {
        psh_sound_state(c, r, seed);
        let src = DATA + 0x1000;
        psh_stream(c, r, seed, src, events);
        c.g[A0][0] = sx(r.next_u32() % 0x10_0000);
        c.g[A1][0] = sx(src);
        c.g[A2][0] = sx(DATA + 0x200);
        c.g[A3][0] = sx(DATA + 0x300);
        c.g[T0][0] = sx(DATA + 0x400);
        c.g[T1][0] = if seed % 5 == 0 { 0 } else { sx(DATA + 0x500) };
        c.pc = 0x003b_b820;
    }

    /// SND_ParseStreamHeader: random tag streams (every tag of the chain,
    /// unknown ids, padding), the PT / GSTR / near-miss magics, channel
    /// counts 0..6, stereo and other layouts, and the absolute / relative
    /// offset flags.
    #[test]
    fn snd_parse_stream_header_matches() {
        check(ee::s13::f_003bb820__generated, snd_parse_stream_header, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3bb820, 0);
            psh_setup(c, &mut r, seed, false);
        });
    }

    driven!(psh_gen_driven, ee::s13::f_003bb820__generated);
    driven!(psh_hand_driven, snd_parse_stream_header);

    /// SND_ParseStreamHeader with events at the tag loop's and the channel
    /// loop's back edges (delivered, or unwinding and re-entered there).
    #[test]
    fn snd_parse_stream_header_events_match() {
        check_events(psh_gen_driven, psh_hand_driven, 150, &|c, seed| {
            let mut r = machine(c, seed, 0x3bb821, 0);
            psh_setup(c, &mut r, seed, true);
            let mode = schedule(c, &mut r, seed, 0x003b_b820);
            // The first due event lands after the prologue's memsets (~250
            // cycles), in the tag loop (its own resume head 0x3bb958) or the
            // non-unwinding InitPatchParams / ParseNextTag loops; a memset
            // that unwinds would resume at a return address, which neither
            // the original nor the port has as an entry.
            if mode != Events::Off {
                c.next_event += 400;
            }
            mode
        });
    }

    /// WorldSound_ComputeFalloff: every curve of the jump table (and
    /// fractional, negative and out-of-range curve numbers), distances
    /// around the 0.7 knee and edge-case floats. The game image supplies the
    /// jump table and the `.lit4` constants.
    #[test]
    fn world_sound_compute_falloff_matches() {
        check(ee::s08::f_002b82b8__generated, world_sound_compute_falloff, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2b82b8, 0);
            elf_image(c);
            let curve = match seed % 10 {
                k @ 0..=5 => (k as f32 + (r.next_u32() % 4) as f32 * 0.25).to_bits(),
                6 => 6.0f32.to_bits(),
                7 => (-1.0f32).to_bits(),
                8 => r.edge_f32(),
                _ => (((seed / 10) % 6) as f32).to_bits(),
            };
            c.f[12] = curve;
            c.f[13] = match r.next_u32() % 4 {
                0 => r.edge_f32(),
                1 => 0.7f32.to_bits(),
                _ => (r.next_u32() as f32 / u32::MAX as f32).to_bits(),
            };
            c.pc = 0x002b_82b8;
        });
    }

    /// PathData_FindNearTrackPaths: 0..9 track paths whose bounds contain
    /// the position (distance 0, taken first) or lie around it, `max` below,
    /// at or above the count (sometimes with garbage in the upper half of
    /// a3), so both passes, the early "full" exit and the minimum search run.
    #[test]
    fn path_data_find_near_track_paths_matches() {
        check(
            ee::s07::f_0026b178__generated,
            path_data_find_near_track_paths,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26b178, 0);
                let this = DATA;
                let items = DATA + 0x1000;
                let pos = DATA + 0x800;
                let out = DATA + 0x900;
                let taken = 1e37f32.to_bits();
                put_words(c, GAME_GP - NEAR_PATHS_TAKEN_LIT, &[taken]);
                put_words(c, GAME_GP - NEAR_PATHS_TAKEN_LIT2, &[taken]);
                let count = r.next_u32() % 10;
                put_words(c, this + NEAR_PATHS_COUNT, &[count, items]);
                let p = [r.f32(50.0), r.f32(50.0), r.f32(50.0)];
                put_words(c, pos, &p);
                for i in 0..count {
                    let box_at = items + NEAR_PATHS_STRIDE * i;
                    let inside = r.next_u32() % 3 == 0;
                    for k in 0..3u32 {
                        let centre = f32::from_bits(p[k as usize]);
                        let (lo, hi) = if inside {
                            (centre - 1.0 - (r.next_u32() % 8) as f32, centre + 1.0)
                        } else {
                            let a = centre + (r.next_u32() % 64) as f32 - 32.0;
                            (a, a + (r.next_u32() % 16) as f32)
                        };
                        c.bus.write32(box_at + 0x1c + 4 * k, lo.to_bits());
                        c.bus.write32(box_at + 0x28 + 4 * k, hi.to_bits());
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(pos);
                c.g[A2][0] = sx(out);
                let max = r.next_u32() % 12;
                c.g[A3][0] = if seed % 7 == 0 {
                    (r.next_u32() as u64) << 32 | max as u64
                } else {
                    max as u64
                };
                c.pc = 0x0026_b178;
            },
        );
    }

    /// A random RefPack stream: header (3- or 4-byte sizes, with or without
    /// the packed-size field), then every command form with random
    /// literal counts, lengths and offsets, and an end code with 0-3
    /// trailing literals.
    fn refpack_stream(r: &mut Rng, seed: u32) -> Vec<u8> {
        let mut s = Vec::new();
        let four = seed % 2 == 1;
        let skip = seed % 4 >= 2;
        let flags: u16 = 0x10fb | (if four { 0x8000 } else { 0 }) | (if skip { 0x100 } else { 0 });
        s.extend_from_slice(&flags.to_be_bytes());
        let width = if four { 4 } else { 3 };
        for _ in 0..if skip { 2 } else { 1 } {
            let v = r.next_u32();
            s.extend_from_slice(&v.to_be_bytes()[4 - width..]);
        }
        let lit = |r: &mut Rng, s: &mut Vec<u8>, n: u32| {
            for _ in 0..n {
                s.push(r.next_u32() as u8);
            }
        };
        let n = r.next_u32() % 9;
        for _ in 0..n {
            let rl = r.next_u32() % 4;
            match r.next_u32() % 4 {
                0 => {
                    let len = 3 + r.next_u32() % 8;
                    let off = r.next_u32() % 1024;
                    s.push((((off >> 8) << 5) | ((len - 3) << 2) | rl) as u8);
                    s.push(off as u8);
                }
                1 => {
                    let len = 4 + r.next_u32() % 64;
                    let off = r.next_u32() % 0x4000;
                    s.push((0x80 | (len - 4)) as u8);
                    s.push(((rl << 6) | (off >> 8)) as u8);
                    s.push(off as u8);
                }
                2 => {
                    let len = 5 + r.next_u32() % 300;
                    let off = r.next_u32() % 0x2_0000;
                    s.push((0xc0 | ((off >> 16) << 4) | (((len - 5) >> 8) << 2) | rl) as u8);
                    s.push((off >> 8) as u8);
                    s.push(off as u8);
                    s.push((len - 5) as u8);
                }
                _ => {
                    let k = r.next_u32() % 28;
                    s.push((0xe0 | k) as u8);
                    lit(r, &mut s, (k + 1) * 4);
                    continue;
                }
            }
            lit(r, &mut s, rl);
        }
        let rl = r.next_u32() % 4;
        s.push((0xfc | rl) as u8);
        lit(r, &mut s, rl);
        s
    }

    /// RefPack_Decode: random streams through every command form, the
    /// header variants, a null source, and with or without `consumed`.
    #[test]
    fn refpack_decode_matches() {
        check(ee::s13::f_003b49d0__generated, refpack_decode, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3b49d0, 0);
            let src = DATA + 0x1000;
            for (i, b) in refpack_stream(&mut r, seed).into_iter().enumerate() {
                c.bus.write8(src + i as u32, b);
            }
            c.g[A0][0] = sx(DATA + 0x4_0000);
            c.g[A1][0] = if seed % 13 == 0 { 0 } else { sx(src) };
            c.g[A2][0] = if seed % 5 == 0 { 0 } else { sx(DATA + 0x800) };
            c.pc = 0x003b_49d0;
        });
    }

    /// SND_FadeVolume setup: SND_SetVolume's channel table and handle
    /// (bad handles, one or more channels), then `frames` (positive, zero
    /// or negative: one frame) and the target volume.
    fn fade_setup(c: &mut Cpu, r: &mut Rng) {
        crate::game::lib::snd::tests::set_volume_setup(c, r);
        c.g[A2][0] = c.g[A1][0];
        c.g[A1][0] = match r.next_u32() % 4 {
            0 => 0,
            1 => (-(1 + (r.next_u32() % 50) as i64)) as u64,
            _ => (1 + r.next_u32() % 300) as u64,
        };
    }

    driven!(fade_gen_driven, ee::s13::f_003b8530__generated);
    driven!(fade_hand_driven, snd_fade_volume);

    /// SND_FadeVolume: plain runs, and events at the channel loop's back
    /// edge (delivered, or unwinding and re-entered at the loop head).
    #[test]
    fn snd_fade_volume_matches() {
        check(ee::s13::f_003b8530__generated, snd_fade_volume, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3b8530, 0);
            fade_setup(c, &mut r);
            c.pc = 0x003b_8530;
        });
        check_events(fade_gen_driven, fade_hand_driven, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3b8531, 0);
            fade_setup(c, &mut r);
            schedule(c, &mut r, seed, 0x003b_8530)
        });
    }

    /// SND_ChannelCalcSend: a random channel table (flags selecting the
    /// different effect parameter blocks, signed send bytes), send slots
    /// 0..3 and channels 0..7.
    #[test]
    fn snd_channel_calc_send_matches() {
        check(ee::s13::f_003ba740__generated, snd_channel_calc_send, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x3ba740, 0);
            // Effect parameter blocks in the state area, then the table.
            for i in 0..0x100u32 {
                c.bus.write32(0x0050_aa00 + 4 * i, r.next_u32());
            }
            let table = DATA + 0x2_0000;
            put_words(c, CALC_SEND_CHANNELS, &[table]);
            for i in 0..8 * 0x8c / 4 {
                c.bus.write32(table + 4 * i, r.next_u32());
            }
            for ch in 0..8u32 {
                let flags = [0x8u16, 0x100, 0x4, 0x10, 0x200, r.next_u32() as u16];
                c.bus.write16(
                    table + ch * 0x8c + CALC_SEND_FLAGS,
                    flags[(r.next_u32() % 6) as usize],
                );
            }
            c.g[A0][0] = (r.next_u32() % 4) as u64;
            c.g[A1][0] = (r.next_u32() % 8) as u64;
            c.pc = 0x003b_a740;
        });
    }
}
