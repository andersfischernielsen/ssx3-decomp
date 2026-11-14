//! Staging module oc361 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc361`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x003b_aac0, snd_parse_patch, "SND_ParsePatch")];

// ------------------------------------------------------------ SND_ParsePatch

/// `SND_ParsePatch`'s stack frame size.
pub(crate) const PARSE_PATCH_FRAME: u32 = 0x3b0;

/// Per-tag values (tags 0..0x2a), one word each: `sp + 4 * tag`.
pub(crate) const PARSE_PATCH_VALUES: u32 = 0x0;

/// Per-tag payload pointers (tags 0..0x2a): `sp + 0xb0 + 4 * tag`.
pub(crate) const PARSE_PATCH_PAYLOADS: u32 = 0xb0;

/// Per-tag values (tags 0x80..0xad): `sp + 0x160 + 4 * (tag - 0x80)`.
pub(crate) const PARSE_PATCH_VALUES_HI: u32 = 0x160;

/// Per-tag payload pointers (tags 0x80..0xad): `sp + 0x220 + 4 * (tag - 0x80)`.
pub(crate) const PARSE_PATCH_PAYLOADS_HI: u32 = 0x220;

/// The per-channel pitch offsets (6 words, cleared by `memset`).
pub(crate) const PARSE_PATCH_PITCH: u32 = 0x2e0;

/// The tag stream record `SND_ParseNextTag` fills: read pointer, tag,
/// value, payload pointer, payload length.
pub(crate) const PARSE_PATCH_REC: u32 = 0x300;

/// Patch fields: the count of tag-0x14 entries (signed byte) and their
/// tables (payload pointer, length).
pub(crate) const PARSE_PATCH_N14: u32 = 0x17;
pub(crate) const PARSE_PATCH_T14_PTR: u32 = 0x34;
pub(crate) const PARSE_PATCH_T14_LEN: u32 = 0x44;

/// Patch fields: the channel count (signed byte), the six sample data
/// pointers (`+0x98`, tags 0x98..0x9b, 0xa4, 0xa5) and lengths (`+0xb0`,
/// halves), the per-channel pitches (`+0xbc`, halves), the stream pointer
/// copy (`+0xc8`).
pub(crate) const PARSE_PATCH_CHANNELS: u32 = 0x3;
pub(crate) const PARSE_PATCH_DATA: u32 = 0x98;
pub(crate) const PARSE_PATCH_DATA_LEN: u32 = 0xb0;
pub(crate) const PARSE_PATCH_PITCHES: u32 = 0xbc;
pub(crate) const PARSE_PATCH_SRC: u32 = 0xc8;

/// The sound state block (0x50a8e8): its output mode byte (`+0x47`, 2 =
/// a mode where stereo patches get no pitch offset) and the pitch table
/// (`+0x2e0`: per channel count, six halves).
pub(crate) const PARSE_PATCH_SND_STATE: u32 = 0x0050_a8e8;
pub(crate) const PARSE_PATCH_STATE_MODE: u32 = 0x47;
pub(crate) const PARSE_PATCH_STATE_PITCH: u32 = 0x2e0;

const PARSE_PATCH_MEMSET: u32 = 0x0041_6210;

const PARSE_PATCH_NEXT_TAG: Callee = Callee {
    addr: 0x003c_7388,
    f: ee::s13::SND_ParseNextTag,
};

/// `sp + off`.
#[inline(always)]
fn parse_patch_at(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// `sw v, off(sp)`.
#[inline(always)]
fn parse_patch_sw(c: &mut Cpu, off: u32, v: u64) {
    let a = parse_patch_at(c, off);
    c.sw(a, v);
}

/// `lw off(sp)`.
#[inline(always)]
fn parse_patch_lw(c: &mut Cpu, off: u32) -> u64 {
    let a = parse_patch_at(c, off);
    c.lw(a)
}

/// `lbu off(sp)`.
#[inline(always)]
fn parse_patch_lbu(c: &mut Cpu, off: u32) -> u64 {
    let a = parse_patch_at(c, off);
    c.lbu(a)
}

/// `lhu off(sp)`.
#[inline(always)]
fn parse_patch_lhu(c: &mut Cpu, off: u32) -> u64 {
    let a = parse_patch_at(c, off);
    c.lhu(a)
}

/// `addu` of two words.
#[inline(always)]
fn parse_patch_addu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// A countdown loop of the prologue: `do { *p = 0 (word); n--; p -= 4 }
/// while (n >= 0)` with `n` in t8 and `p` in `reg`, events at the back edge.
fn parse_patch_clear_down(c: &mut Cpu, reg: usize, head: u32) {
    loop {
        c.cycles += 7;
        let p = r32(c, reg);
        c.sw(p, 0);
        c.g[T8][0] = sx((c.g[T8][0] as u32).wrapping_sub(1));
        let more = (c.g[T8][0] as i64) >= 0;
        c.g[reg][0] = sx(p.wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `int SND_ParsePatch(u8** src, Patch* patch)` (0x3baac0): parse a patch's
/// tag stream (`SND_ParseNextTag`) into `patch`. Tags 0..0x2a and
/// 0x80..0xad keep their value and payload pointer in tables on the stack
/// (preset with the patch defaults: volume 0x7f, key 0x3c, pan 0x40, one
/// channel, rate 0x5622, ...); tag 0x14 appends its payload to the patch's
/// list, tags 0x98..0x9b, 0xa4 and 0xa5 set the six channel data pointers
/// and lengths. The stream ends at 0xff (returns 0) or at the 0xfe marker
/// (returns 1). Then the tables go into the patch fields, each channel's
/// pitch is the table pitch for the channel count plus the patch's offset
/// (zero in output mode 2 for stereo patches), and `*src` advances past the
/// tags.
pub fn snd_parse_patch(c: &mut Cpu) {
    let frame = Frame {
        size: PARSE_PATCH_FRAME,
    };
    guest(c, |c| {
        c.cycles += 26;
        Frame::open(c, PARSE_PATCH_FRAME);
        c.g[A2][0] = 0x18;
        frame.save(c, S6, 0x340);
        frame.save(c, S4, 0x360);
        c.g[S6][0] = c.g[A0][0]; // src
        frame.save(c, S7, 0x330);
        c.g[S4][0] = c.g[A1][0]; // patch
        c.g[A0][0] = sx(parse_patch_at(c, PARSE_PATCH_PITCH));
        frame.save(c, S0, 0x3a0);
        frame.save(c, S1, 0x390);
        c.g[A1][0] = 0;
        frame.save(c, S2, 0x380);
        frame.save(c, S3, 0x370);
        frame.save(c, S5, 0x350);
        frame.save_ra(c, 0x320);
        c.g[S7][0] = c.g[A0][0];
        // memset(pitch, 0, 0x18)
        call_ptr(c, PARSE_PATCH_MEMSET, 0x003b_ab04)?;

        let src = r32(c, S6);
        c.g[V0][0] = c.lw(src);
        c.g[S2][0] = sx(parse_patch_at(c, PARSE_PATCH_PAYLOADS));
        c.g[S0][0] = sx(parse_patch_at(c, PARSE_PATCH_PAYLOADS_HI));
        c.g[S1][0] = sx(parse_patch_at(c, PARSE_PATCH_REC));
        let patch = r32(c, S4);
        c.sw(patch.wrapping_add(PARSE_PATCH_SRC), c.g[V0][0]);
        c.g[T8][0] = 0x2a;
        parse_patch_sw(c, PARSE_PATCH_REC, c.g[V0][0]);
        c.g[V1][0] = sx(parse_patch_at(c, 0x158));
        // Clear the payload pointers (both tables, top down).
        parse_patch_clear_down(c, V1, 0x003b_ab28);
        c.cycles += 5;
        c.g[A0][0] = sx(patch.wrapping_add(PARSE_PATCH_DATA));
        c.g[V1][0] = sx(patch.wrapping_add(PARSE_PATCH_DATA_LEN));
        c.g[T8][0] = 0x2d;
        c.g[V0][0] = parse_patch_addu(c.g[S0][0], 0xb4);
        parse_patch_clear_down(c, V0, 0x003b_ab58);
        // Clear the six channel data pointers and lengths.
        c.cycles += 3;
        c.g[V0][0] = c.g[A0][0];
        c.g[T8][0] = 5;
        loop {
            c.cycles += 7;
            c.sw(r32(c, V0), 0);
            c.g[T8][0] = sx((c.g[T8][0] as u32).wrapping_sub(1));
            c.sh(r32(c, V1), 0);
            c.g[V0][0] = parse_patch_addu(c.g[V0][0], 4);
            let more = (c.g[T8][0] as i64) >= 0;
            c.g[V1][0] = parse_patch_addu(c.g[V1][0], 2);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_ab80);
            }
        }

        // The defaults.
        c.cycles += 68;
        c.g[V1][0] = 0x45_0000;
        c.g[A1][0] = 0x7f;
        c.g[A3][0] = u64::MAX;
        c.g[A2][0] = 1;
        c.g[V0][0] = 0x3c;
        c.g[V1][0] = sx(0x0044_c580);
        c.g[A0][0] = 0x40;
        parse_patch_sw(c, 0x1c, c.g[V0][0]);
        parse_patch_sw(c, 0x30, c.g[A0][0]);
        c.g[T0][0] = 0x5622;
        parse_patch_sw(c, 0x64, c.g[V1][0]);
        c.g[T1][0] = 5;
        parse_patch_sw(c, 0x70, c.g[A1][0]);
        c.g[V0][0] = 8;
        parse_patch_sw(c, 0x4, 0);
        c.g[S5][0] = 0; // result: the stream ended without 0xfe
        parse_patch_sw(c, 0x8, c.g[A1][0]);
        parse_patch_sw(c, 0xc, 0);
        parse_patch_sw(c, 0x10, c.g[A1][0]);
        parse_patch_sw(c, 0x18, 0);
        parse_patch_sw(c, 0x20, c.g[A3][0]);
        parse_patch_sw(c, 0x24, c.g[A2][0]);
        parse_patch_sw(c, 0x28, 0);
        parse_patch_sw(c, 0x34, 0);
        parse_patch_sw(c, 0x38, c.g[A1][0]);
        for off in [0x3c, 0x40, 0x44, 0x48, 0x4c] {
            parse_patch_sw(c, off, 0);
        }
        c.sb(patch.wrapping_add(PARSE_PATCH_N14), 0);
        for off in [
            0x5c, 0x68, 0x98, 0x9c, 0xa0, 0xa4, 0xa8, 0x74, 0x78, 0x7c, 0x80, 0x84, 0x88, 0x8c,
        ] {
            parse_patch_sw(c, off, 0);
        }
        parse_patch_sw(c, 0x168, c.g[A2][0]); // one channel
        parse_patch_sw(c, 0x170, c.g[T0][0]); // rate
        parse_patch_sw(c, 0x17c, c.g[A3][0]);
        parse_patch_sw(c, 0x1e0, c.g[T1][0]);
        parse_patch_sw(c, 0x190, c.g[V0][0]);
        parse_patch_sw(c, 0x90, 0);
        parse_patch_sw(c, 0x94, c.g[A2][0]);
        parse_patch_sw(c, 0x160, 0);
        parse_patch_sw(c, 0x174, 0);
        parse_patch_sw(c, 0x178, c.g[A3][0]);
        for off in [
            0x180, 0x1d0, 0x184, 0x1d4, 0x1b0, 0x1d8, 0x1b4, 0x1dc, 0x1e8, 0x1f8, 0x1ec, 0x1fc,
        ] {
            parse_patch_sw(c, off, 0);
        }

        // The tag loop.
        'parse: loop {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            PARSE_PATCH_NEXT_TAG.call(c, 0x003b_ae1c)?;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = parse_patch_lw(c, PARSE_PATCH_REC + 0x4);
            if !more {
                break 'parse;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_acb0);
            }
            let sp_ = sp(c);
            let patch = r32(c, S4);
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 43) as u64;
            c.g[A0][0] = parse_patch_lw(c, PARSE_PATCH_REC + 0x8);
            if c.g[V0][0] != 0 {
                // Tags 0..0x2a: value and payload pointer.
                c.cycles += 12;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx(sp_.wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = 0x14;
                c.sw(r32(c, V1).wrapping_add(PARSE_PATCH_VALUES), c.g[A0][0]);
                c.g[V0][0] = parse_patch_lw(c, PARSE_PATCH_REC + 0x4);
                c.g[A0][0] = parse_patch_lw(c, PARSE_PATCH_REC + 0xc);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = parse_patch_addu(c.g[S2][0], c.g[V0][0]);
                c.sw(r32(c, V0), c.g[A0][0]);
                c.g[V1][0] = parse_patch_lw(c, PARSE_PATCH_REC + 0x4);
                c.g[A0][0] = parse_patch_lw(c, PARSE_PATCH_REC + 0xc);
                if c.g[V1][0] != c.g[A1][0] {
                    continue 'parse;
                }
                // Tag 0x14: append (payload, length) to the patch's list.
                c.cycles += 13;
                c.g[V1][0] = c.lb(patch.wrapping_add(PARSE_PATCH_N14));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[A1][0] = parse_patch_lw(c, PARSE_PATCH_REC + 0x10);
                c.g[V1][0] = parse_patch_addu(c.g[S4][0], c.g[V1][0]);
                c.sw(r32(c, V1).wrapping_add(PARSE_PATCH_T14_PTR), c.g[A0][0]);
                c.g[V0][0] = c.lb(patch.wrapping_add(PARSE_PATCH_N14));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = parse_patch_addu(c.g[S4][0], c.g[V0][0]);
                c.sw(r32(c, V0).wrapping_add(PARSE_PATCH_T14_LEN), c.g[A1][0]);
                c.g[V1][0] = c.lbu(patch.wrapping_add(PARSE_PATCH_N14));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                c.sb(patch.wrapping_add(PARSE_PATCH_N14), c.g[V1][0]);
                continue 'parse;
            }
            c.cycles += 4;
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x80));
            let high = c.g[A0][0] < 0x2e;
            c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
            if !high {
                c.cycles += 3;
                c.g[V0][0] = 0xfe;
                if c.g[V1][0] != c.g[V0][0] {
                    continue 'parse;
                }
                // The end marker.
                c.cycles += 2;
                c.g[S5][0] = 1;
                break 'parse;
            }
            // Tags 0x80..0xad: value and payload pointer.
            c.cycles += 13;
            c.g[A1][0] = 0x98;
            c.g[A0][0] = parse_patch_lw(c, PARSE_PATCH_REC + 0x8);
            c.g[V1][0] = sx(sp_.wrapping_add(c.g[V0][0] as u32));
            c.sw(r32(c, V1).wrapping_add(PARSE_PATCH_VALUES_HI), c.g[A0][0]);
            c.g[V0][0] = parse_patch_lw(c, PARSE_PATCH_REC + 0x4);
            c.g[V1][0] = parse_patch_lw(c, PARSE_PATCH_REC + 0xc);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x80));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = parse_patch_addu(c.g[S0][0], c.g[V0][0]);
            c.sw(r32(c, V0), c.g[V1][0]);
            c.g[V1][0] = parse_patch_lw(c, PARSE_PATCH_REC + 0x4);
            c.g[V0][0] = 0x99;
            if c.g[V1][0] == c.g[A1][0] {
                c.cycles += 5;
                c.g[V0][0] = parse_patch_lw(c, PARSE_PATCH_REC + 0xc);
                c.g[V1][0] = parse_patch_lhu(c, PARSE_PATCH_REC + 0x10);
                c.sw(patch.wrapping_add(PARSE_PATCH_DATA), c.g[V0][0]);
                c.sh(patch.wrapping_add(PARSE_PATCH_DATA_LEN), c.g[V1][0]);
                continue 'parse;
            }
            // The other channel data tags (compared against v0, which
            // holds the next id): 0x99, 0x9a, 0x9b, 0xa4.
            for (next, k) in [(0x9au64, 1u32), (0x9b, 2), (0xa4, 3), (0xa5, 4)] {
                c.cycles += 2;
                let hit = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = next;
                if hit {
                    c.cycles += 5;
                    c.g[V0][0] = parse_patch_lw(c, PARSE_PATCH_REC + 0xc);
                    c.g[V1][0] = parse_patch_lhu(c, PARSE_PATCH_REC + 0x10);
                    c.sw(patch.wrapping_add(PARSE_PATCH_DATA + 4 * k), c.g[V0][0]);
                    c.sh(patch.wrapping_add(PARSE_PATCH_DATA_LEN + 2 * k), c.g[V1][0]);
                    continue 'parse;
                }
            }
            // 0xa5.
            c.cycles += 2;
            let hit = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = parse_patch_lw(c, PARSE_PATCH_REC + 0xc);
            if hit {
                c.cycles += 4;
                c.g[V1][0] = parse_patch_lhu(c, PARSE_PATCH_REC + 0x10);
                c.sw(patch.wrapping_add(PARSE_PATCH_DATA + 4 * 5), c.g[V0][0]);
                c.sh(patch.wrapping_add(PARSE_PATCH_DATA_LEN + 2 * 5), c.g[V1][0]);
            }
        }

        // The tables into the patch fields (the compiled order of loads and
        // stores; the registers they leave are part of the result).
        parse_patch_store_fields(c);

        if (c.g[S3][0] as i64) > 0 {
            // Per-channel pitch: table[channels - 1][i] + offset[i].
            c.cycles += 8;
            c.g[V0][0] = 0x51_0000;
            c.g[A1][0] = c.g[S7][0];
            c.g[A3][0] = sx(PARSE_PATCH_SND_STATE);
            c.g[T2][0] = 2;
            c.g[T1][0] = sx(PARSE_PATCH_SND_STATE + PARSE_PATCH_STATE_PITCH);
            c.g[T0][0] = 0xc;
            c.g[A2][0] = sx(r32(c, S4).wrapping_add(PARSE_PATCH_PITCHES));
            loop {
                let patch = r32(c, S4);
                c.cycles += 3;
                c.g[V1][0] = c.lbu(r32(c, A3).wrapping_add(PARSE_PATCH_STATE_MODE));
                c.g[V0][0] = c.lb(patch.wrapping_add(PARSE_PATCH_CHANNELS));
                if c.g[V1][0] == c.g[T2][0] {
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[V1][0] {
                        c.sw(r32(c, A1), 0);
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lb(patch.wrapping_add(PARSE_PATCH_CHANNELS));
                }
                c.cycles += 15;
                c.g[V1][0] = sx((c.g[T8][0] as u32) << 1);
                c.g[A0][0] = c.lhu(r32(c, A1));
                c.g[T8][0] = parse_patch_addu(c.g[T8][0], 1);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.g[A1][0] = parse_patch_addu(c.g[A1][0], 4);
                c.mult(0, V0, V0, T0);
                c.g[V1][0] = parse_patch_addu(c.g[V1][0], c.g[V0][0]);
                c.g[V1][0] = parse_patch_addu(c.g[V1][0], c.g[T1][0]);
                c.g[V0][0] = c.lhu(r32(c, V1));
                c.g[V0][0] = parse_patch_addu(c.g[V0][0], c.g[A0][0]);
                c.sh(r32(c, A2), c.g[V0][0]);
                c.g[V0][0] = c.lb(patch.wrapping_add(PARSE_PATCH_CHANNELS));
                c.g[V0][0] = ((c.g[T8][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = parse_patch_addu(c.g[A2][0], 2);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_b038);
                }
            }
        }

        // *src = the read pointer; return s5.
        c.cycles += 14;
        c.g[V1][0] = parse_patch_lw(c, PARSE_PATCH_REC);
        c.g[V0][0] = c.g[S5][0];
        frame.restore(c, S0, 0x3a0);
        c.sw(r32(c, S6), c.g[V1][0]);
        frame.restore(c, S1, 0x390);
        frame.restore(c, S2, 0x380);
        frame.restore(c, S3, 0x370);
        frame.restore(c, S4, 0x360);
        frame.restore(c, S5, 0x350);
        frame.restore(c, S6, 0x340);
        frame.restore(c, S7, 0x330);
        frame.restore_ra(c, 0x320);
        frame.close(c);
        Ok(())
    });
}

/// The patch fields from the tag tables (0x3bae24, 125 cycles), in the
/// compiled order: leaves t8 = 0 (the pitch loop's channel) and s3 = the
/// channel count << 24.
fn parse_patch_store_fields(c: &mut Cpu) {
    let p = r32(c, S4);
    c.cycles += 125;
    c.g[V0][0] = parse_patch_lw(c, 0x48);
    c.g[T8][0] = 0;
    c.g[V1][0] = parse_patch_lw(c, 0xf8);
    c.g[S0][0] = parse_patch_lbu(c, 0x4);
    c.g[V1][0] = parse_patch_addu(c.g[V1][0], c.g[V0][0]);
    c.g[S1][0] = parse_patch_lbu(c, 0x8);
    c.g[A0][0] = parse_patch_lbu(c, 0xc);
    c.g[V0][0] = parse_patch_lbu(c, 0x10);
    c.g[A1][0] = parse_patch_lbu(c, 0x18);
    c.g[A2][0] = parse_patch_lbu(c, 0x1c);
    c.g[A3][0] = parse_patch_lbu(c, 0x20);
    c.g[T0][0] = parse_patch_lbu(c, 0x24);
    c.g[T1][0] = parse_patch_lbu(c, 0x28);
    c.g[T2][0] = parse_patch_lbu(c, 0x30);
    c.g[T3][0] = parse_patch_lbu(c, 0x34);
    c.g[T4][0] = parse_patch_lbu(c, 0x38);
    c.g[T5][0] = parse_patch_lbu(c, 0x3c);
    c.g[T6][0] = parse_patch_lhu(c, 0x40);
    c.g[T7][0] = parse_patch_lhu(c, 0x44);
    c.g[S2][0] = parse_patch_lbu(c, 0x4c);
    c.sb(p.wrapping_add(0x8), c.g[S0][0]);
    c.sb(p.wrapping_add(0x9), c.g[S1][0]);
    c.sb(p.wrapping_add(0xa), c.g[A0][0]);
    c.sb(p.wrapping_add(0xb), c.g[V0][0]);
    c.sb(p.wrapping_add(0xc), c.g[A1][0]);
    c.sb(p.wrapping_add(0xd), c.g[A2][0]);
    c.sb(p.wrapping_add(0xe), c.g[A3][0]);
    c.sb(p.wrapping_add(0xf), c.g[T0][0]);
    c.sb(p.wrapping_add(0x10), c.g[T1][0]);
    c.sb(p.wrapping_add(0x11), c.g[T2][0]);
    c.sb(p.wrapping_add(0x12), c.g[T3][0]);
    c.sb(p.wrapping_add(0x14), c.g[T4][0]);
    c.sb(p.wrapping_add(0x15), c.g[T5][0]);
    c.sh(p.wrapping_add(0x4), c.g[T6][0]);
    c.sh(p.wrapping_add(0x6), c.g[T7][0]);
    c.sw(p.wrapping_add(0x20), c.g[V1][0]);
    c.sb(p.wrapping_add(0x16), c.g[S2][0]);
    c.g[V0][0] = parse_patch_lw(c, 0x5c);
    c.g[V1][0] = parse_patch_lw(c, 0x64);
    c.g[A1][0] = parse_patch_lw(c, 0x10c);
    c.g[A0][0] = parse_patch_lw(c, 0x114);
    c.g[A1][0] = parse_patch_addu(c.g[A1][0], c.g[V0][0]);
    c.g[A2][0] = parse_patch_lw(c, 0x124);
    c.g[A0][0] = parse_patch_addu(c.g[A0][0], c.g[V1][0]);
    c.g[V0][0] = parse_patch_lw(c, 0x74);
    c.g[V1][0] = parse_patch_lw(c, 0x80);
    c.g[A3][0] = parse_patch_lw(c, 0x130);
    c.g[A2][0] = parse_patch_addu(c.g[A2][0], c.g[V0][0]);
    c.g[T3][0] = parse_patch_lw(c, 0x68);
    c.g[A3][0] = parse_patch_addu(c.g[A3][0], c.g[V1][0]);
    c.g[T4][0] = parse_patch_lw(c, 0x98);
    c.g[V0][0] = parse_patch_lw(c, 0x9c);
    c.g[V1][0] = parse_patch_lw(c, 0xa0);
    c.g[T0][0] = parse_patch_lw(c, 0xa4);
    c.g[T1][0] = parse_patch_lw(c, 0xa8);
    c.g[T2][0] = parse_patch_lbu(c, 0x70);
    c.g[T5][0] = parse_patch_lbu(c, 0x78);
    c.g[T6][0] = parse_patch_lbu(c, 0x7c);
    c.g[T7][0] = parse_patch_lbu(c, 0x84);
    c.g[S0][0] = parse_patch_lhu(c, 0x88);
    c.sw(p.wrapping_add(0x24), c.g[A1][0]);
    c.sw(p.wrapping_add(0x28), c.g[A0][0]);
    c.sw(p.wrapping_add(0x80), c.g[T3][0]);
    c.sw(p.wrapping_add(0x84), c.g[T4][0]);
    c.sw(p.wrapping_add(0x88), c.g[V0][0]);
    c.sw(p.wrapping_add(0x8c), c.g[V1][0]);
    c.sw(p.wrapping_add(0x90), c.g[T0][0]);
    c.sw(p.wrapping_add(0x94), c.g[T1][0]);
    c.sb(p.wrapping_add(0x1a), c.g[T2][0]);
    c.sw(p.wrapping_add(0x2c), c.g[A2][0]);
    c.sb(p.wrapping_add(0x1b), c.g[T5][0]);
    c.sb(p.wrapping_add(0x1c), c.g[T6][0]);
    c.sw(p.wrapping_add(0x30), c.g[A3][0]);
    c.sb(p.wrapping_add(0x1d), c.g[T7][0]);
    c.sh(p.wrapping_add(0x54), c.g[S0][0]);
    c.g[V0][0] = parse_patch_lbu(c, 0x94);
    c.g[V1][0] = parse_patch_lbu(c, 0x160);
    c.g[A0][0] = parse_patch_lbu(c, 0x1e0);
    c.g[A1][0] = parse_patch_lhu(c, 0x170);
    c.g[A2][0] = parse_patch_lw(c, 0x174);
    c.g[A3][0] = parse_patch_lw(c, 0x178);
    c.g[T0][0] = parse_patch_lw(c, 0x17c);
    c.g[T7][0] = parse_patch_lbu(c, 0x8c);
    c.g[S0][0] = parse_patch_lhu(c, 0x90);
    c.g[T1][0] = parse_patch_lhu(c, 0x190);
    c.g[T2][0] = parse_patch_lw(c, 0x248);
    c.g[T3][0] = parse_patch_lw(c, 0x180);
    c.g[S2][0] = parse_patch_lbu(c, 0x168);
    c.g[T4][0] = parse_patch_lw(c, 0x1d0);
    c.g[T6][0] = parse_patch_lw(c, 0x1d4);
    c.g[S3][0] = sx((c.g[S2][0] as u32) << 24);
    c.g[T5][0] = parse_patch_lw(c, 0x184);
    c.g[S1][0] = parse_patch_lw(c, 0x1b0);
    c.sb(p.wrapping_add(0x1f), c.g[V0][0]);
    c.sb(p.wrapping_add(0x2), c.g[V1][0]);
    c.sb(p.wrapping_add(0x13), c.g[A0][0]);
    c.sh(p.wrapping_add(0x56), c.g[A1][0]);
    c.sw(p.wrapping_add(0x58), c.g[A2][0]);
    c.sw(p.wrapping_add(0x5c), c.g[A3][0]);
    c.sw(p.wrapping_add(0x60), c.g[T0][0]);
    c.sb(p.wrapping_add(0x1e), c.g[T7][0]);
    c.sh(p, c.g[S0][0]);
    c.sh(p.wrapping_add(0x18), c.g[T1][0]);
    c.sw(p.wrapping_add(0x64), c.g[T2][0]);
    c.sw(p.wrapping_add(0x68), c.g[T3][0]);
    parse_patch_sw(c, 0x2e0, c.g[T4][0]);
    c.sw(p.wrapping_add(0x6c), c.g[T5][0]);
    parse_patch_sw(c, 0x2e4, c.g[T6][0]);
    c.sb(p.wrapping_add(PARSE_PATCH_CHANNELS), c.g[S2][0]);
    c.sw(p.wrapping_add(0x70), c.g[S1][0]);
    c.g[V0][0] = parse_patch_lw(c, 0x1d8);
    c.g[V1][0] = parse_patch_lw(c, 0x1b4);
    c.g[A0][0] = parse_patch_lw(c, 0x1dc);
    c.g[A2][0] = parse_patch_lw(c, 0x1f8);
    c.g[T0][0] = parse_patch_lw(c, 0x1fc);
    c.g[A1][0] = parse_patch_lw(c, 0x1e8);
    c.g[A3][0] = parse_patch_lw(c, 0x1ec);
    parse_patch_sw(c, 0x2e8, c.g[V0][0]);
    c.sw(p.wrapping_add(0x74), c.g[V1][0]);
    parse_patch_sw(c, 0x2ec, c.g[A0][0]);
    c.sw(p.wrapping_add(0x78), c.g[A1][0]);
    parse_patch_sw(c, 0x2f0, c.g[A2][0]);
    c.sw(p.wrapping_add(0x7c), c.g[A3][0]);
    parse_patch_sw(c, 0x2f4, c.g[T0][0]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fill, Rng, DATA};

    /// One tag of a patch stream: id, payload length (0..4: a big-endian
    /// value; 5..: payload only), payload bytes.
    fn parse_patch_put_tag(c: &mut Cpu, r: &mut Rng, at: &mut u32, tag: u8, value: Option<u32>) {
        c.bus.write8(*at, tag);
        *at += 1;
        if tag == 0xfd || tag == 0xfe || tag == 0xff {
            return;
        }
        let len = match value {
            Some(_) => 1 + r.next_u32() % 4,
            None => r.next_u32() % 9,
        };
        c.bus.write8(*at, len as u8);
        *at += 1;
        let v = value.unwrap_or_else(|| r.next_u32());
        for i in 0..len {
            let b = if value.is_some() {
                // Big endian: the low `len` bytes of the value.
                (v >> (8 * (len - 1 - i))) as u8
            } else {
                r.next_u32() as u8
            };
            c.bus.write8(*at, b);
            *at += 1;
        }
    }

    /// `SND_ParsePatch`: random tag streams (values for tags 0..0x2a with
    /// the 0x14 list, the 0x80..0xad tags with every channel data tag,
    /// ignored ids, bare 0xfd markers, long payloads), ending at 0xff
    /// (result 0) or 0xfe (result 1); channel counts 0..6 and negative,
    /// the output mode byte 2 or not (stereo patches lose their pitch
    /// offsets in mode 2).
    #[test]
    fn snd_parse_patch_matches() {
        check(
            ee::s13::f_003baac0__generated,
            snd_parse_patch,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3baac0, 0);
                c.pc = 0x003b_aac0;
                let (src, patch, stream) = (DATA, DATA + 0x100, DATA + 0x1000);
                fill(c, &mut r, patch, 0x40);
                fill(c, &mut r, stream, 0x100);
                // The sound state: output mode and the pitch table.
                let mode = if seed % 2 == 0 { 2 } else { r.next_u32() % 4 };
                c.bus
                    .write8(PARSE_PATCH_SND_STATE + PARSE_PATCH_STATE_MODE, mode as u8);
                fill(
                    c,
                    &mut r,
                    PARSE_PATCH_SND_STATE + PARSE_PATCH_STATE_PITCH,
                    0x20,
                );
                let mut at = stream;
                let n = r.next_u32() % 24;
                for _ in 0..n {
                    let (tag, value) = match r.next_u32() % 12 {
                        0 => (0x14, None),
                        1 | 2 => ((r.next_u32() % 43) as u8, None),
                        3 => (
                            0x82,
                            Some(match r.next_u32() % 5 {
                                0 => 0x80 | r.next_u32() % 0x80,
                                1 => 2,
                                _ => r.next_u32() % 7,
                            }),
                        ),
                        4 | 5 => {
                            let ids = [0x98u8, 0x99, 0x9a, 0x9b, 0xa4, 0xa5];
                            (ids[(r.next_u32() % 6) as usize], None)
                        }
                        6 | 7 => ((0x80 + r.next_u32() % 0x2e) as u8, None),
                        8 => ((0x2b + r.next_u32() % 0x55) as u8, None),
                        9 => ((0xae + r.next_u32() % 0x4e) as u8, None),
                        10 => (0xfd, None),
                        _ => ((r.next_u32() % 43) as u8, Some(r.next_u32())),
                    };
                    parse_patch_put_tag(c, &mut r, &mut at, tag, value);
                }
                // Sometimes a stereo patch for the mode-2 path.
                if seed % 3 == 0 {
                    parse_patch_put_tag(c, &mut r, &mut at, 0x82, Some(2));
                }
                let end = if seed % 4 == 1 { 0xff } else { 0xfe };
                c.bus.write8(at, end);
                c.bus.write32(src, stream);
                c.g[A0][0] = sx(src);
                c.g[A1][0] = sx(patch);
            },
        );
    }
}
