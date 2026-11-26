//! Staging module oc1401 (wave av-1): hand-written AV player constructor,
//! chunk-callback, subtitle parsing leaf and player time/state replacements,
//! each with its differential test against the generated body. Moved into the
//! source-tree units by `tools/reorg_modules.py apply oc1401`.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub(crate) use crate::game::world::wscriptcache::callee::AV_PLAYER_INIT;
pub(crate) use crate::game::world::wscriptcache::AV_PLAYER_ON_CHUNK;
pub(crate) use crate::game::world::wscriptcache::SNDSTRM_GET_REQUEST_STATUS;
pub(crate) use crate::game::world::wscriptcache::SNDSTRM_GET_STATUS;
pub(crate) use crate::game::world::wscriptcache::SND_LOCK;
pub(crate) use crate::game::world::wscriptcache::SND_UNLOCK;

/// `int AV_StrICmp(const char*, const char*)` (0x3af378): case-insensitive
/// string compare, used here to match subtitle justification names.
pub(crate) const AV_STR_ICMP: Callee = Callee {
    addr: 0x003a_f378,
    f: ee::s13::f_003af378,
};

/// `void AV_SkipWhitespace(const char**, int*)` (0x3af300): advance over
/// leading whitespace, used here before token matching.
pub(crate) const AV_SKIP_WHITESPACE: Callee = Callee {
    addr: 0x003a_f300,
    f: ee::s13::f_003af300,
};

/// `float RCMP_DECODER_GetFrameRate(RCMP_DECODER*)` (0x3b05c0): the codec's
/// frame rate (0 when it has no object yet).
pub(crate) const RCMP_DECODER_GET_FRAME_RATE: Callee = Callee {
    addr: 0x003b_05c0,
    f: ee::s13::f_003b05c0,
};

/// `int AV_Player_GetTimeMs(AV_Player*)` (0x3ae9a0): the player's media
/// clock in milliseconds (ported below; called by the frame-due test).
pub(crate) const AV_PLAYER_GET_TIME_MS: Callee = Callee {
    addr: 0x003a_e9a0,
    f: ee::s13::f_003ae9a0,
};

/// `int SND_GetOutputLatency(void)` (0x3b5f60): nonzero path calls the
/// platform latency hook, but both arms return 0.
pub(crate) const SND_GET_OUTPUT_LATENCY: Callee = Callee {
    addr: 0x003b_5f60,
    f: ee::s13::f_003b5f60,
};

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_ddc0, av_player_ctor, "AV_Player_ctor"),
    (0x003a_de58, av_player_ctor_3, "AV_Player_ctor_3"),
    (0x003a_de90, av_player_ctor_4, "AV_Player_ctor_4"),
    (
        0x003a_ecb8,
        av_player_get_chunk_callback,
        "AV_Player_GetChunkCallback",
    ),
    (0x003a_f560, av_parse_int_field, "AV_ParseIntField"),
    (0x003a_f428, av_parse_justify, "AV_ParseJustify"),
    (0x003a_f670, av_find_token, "AV_FindToken"),
    (0x003a_e780, av_player_is_frame_due, "AV_Player_IsFrameDue"),
    (0x003a_e9a0, av_player_get_time_ms, "AV_Player_GetTimeMs"),
];

/// `void* AV_Player_ctor(AV_Player* self, int a1, int a2, int a3, int a4)`
/// (0x3addc0): clear the word at +0x60, then initialise with
/// `(a1, 0, a2, 0, 0, 0, a3, a4)` (the last word travels on the stack).
/// Returns `self`.
pub fn av_player_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[T3][0] = c.g[A3][0];
        frame.save(c, S0, 0x20);
        c.g[A3][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let stacked = c.g[T0][0];
        c.sw(frame.at(c, 0), stacked);
        let this = c.g[S0][0] as u32;
        Addr(this).off(0x60).sw(c, 0);
        c.g[A2][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        AV_PLAYER_INIT.call(c, 0x003a_ddf4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void* AV_Player_ctor_3(AV_Player* self, ...)` (0x3ade58): clear the word
/// at +0x60 and forward all eight constructor arguments to
/// `AV_Player_Init` (the eighth arrives on the caller's stack). Returns
/// `self`.
pub fn av_player_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[V0][0] = Addr(frame.at(c, 0x30)).lw(c);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        let this = c.g[S0][0] as u32;
        Addr(this).off(0x60).sw(c, 0);
        let stacked = c.g[V0][0];
        c.sw(frame.at(c, 0), stacked);
        AV_PLAYER_INIT.call(c, 0x003a_de78)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void* AV_Player_ctor_4(AV_Player* self, ...)` (0x3ade90): clear the word
/// at +0x60, forward the stacked seventh argument, and initialise with a
/// constant 2 in the seventh register slot. Returns `self`.
pub fn av_player_ctor_4(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let stacked = c.g[T3][0];
        c.sw(frame.at(c, 0), stacked);
        let this = c.g[S0][0] as u32;
        Addr(this).off(0x60).sw(c, 0);
        c.g[T3][0] = 0x2;
        AV_PLAYER_INIT.call(c, 0x003a_deb0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void AV_Player_GetChunkCallback(AV_Player* this, void* user)` (0x3aecb8):
/// `AV_Player_OnChunk(*user, this)`. Adopted verbatim from the draft in
/// `game/world/wscriptcache.rs`: the generated body does the same loads in
/// the same order (9 cycles, one 0x10 frame, `v0` as the spill).
pub fn av_player_get_chunk_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A1][0] = c.g[V0][0];
        AV_PLAYER_ON_CHUNK.call(c, 0x003a_ecd0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AV_ParseIntField(char** pp, int* plen, int* out)` (0x3af560): scan
/// `*pp` for a tab/LF within `*plen` bytes, NUL it, parse the head with
/// `strtol` base 10, shrink `*plen` past the consumed bytes, restore the
/// delimiter unless it landed exactly on the new end, and store the value.
/// Returns 1 when digits parsed, 0 when the head is empty.
pub fn av_parse_int_field(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = 0x9;
        frame.save_ra(c, 0x10);
        // `i` (S0) scans `*pp` (S2) for the first tab/LF; `delim` (S3) remembers
        // it (9 when the scan runs out). The NUL goes in the delay slot.
        'scan: {
            c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
            let empty_len = c.g[V0][0] == 0;
            // Delay slot of the length test: stash `*out` on both paths.
            c.g[S4][0] = c.g[A2][0];
            if empty_len {
                break 'scan;
            }
            'found: {
                'first: {
                    c.cycles += 7;
                    c.g[V1][0] = Addr(c.g[S2][0] as u32).lw(c);
                    c.g[A1][0] = c.g[V1][0];
                    c.g[V1][0] = Addr(c.g[V1][0] as u32).lbu(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x9));
                    c.g[V0][0] = (c.g[V0][0] < 0x2) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                        break 'first;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                    Addr(c.g[A1][0] as u32).sb(c, 0);
                    break 'found;
                }
                'search: loop {
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    if c.g[V0][0] == 0 {
                        break 'scan;
                    }
                    c.cycles += 5;
                    c.g[V1][0] = Addr(c.g[A0][0] as u32).lbu(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x9));
                    c.g[V0][0] = (c.g[V0][0] < 0x2) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_f5c0);
                        }
                        continue 'search;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                    Addr(c.g[A0][0] as u32).sb(c, 0);
                    break 'found;
                }
            }
            // The delimiter byte, sign-extended (0 when no delimiter was NULled,
            // 9/0xA for tab/LF).
            c.cycles += 1;
            c.g[S3][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
        }
        // Parse the (possibly NUL-shortened) head; `end` lands on the stack.
        c.cycles += 12;
        c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0xa;
        call_ptr(c, 0x0041_e9e0, 0x003a_f600)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V1][0] = c.lw(frame.at(c, 0));
        c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        let new_len = c.g[A0][0];
        Addr(c.g[S1][0] as u32).sw(c, new_len);
        if c.g[S0][0] != c.g[A0][0] {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            let delim = c.g[S3][0];
            Addr(c.g[V0][0] as u32).sb(c, delim);
        }
        // Empty head: 0, leaving `*out` alone. Otherwise store the value, 1.
        c.cycles += 4;
        c.g[V1][0] = Addr(c.g[S2][0] as u32).lw(c);
        c.g[V0][0] = c.lw(frame.at(c, 0));
        let at_end = c.g[V0][0] == c.g[V1][0];
        c.g[V0][0] = 0x1;
        if !at_end {
            c.cycles += 4;
            c.g[V1][0] = c.g[A1][0] << 32;
            c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
            let r = c.g[V1][0];
            Addr(c.g[S4][0] as u32).sw(c, r);
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 8;
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

/// `int AV_ParseJustify(char* s, int len)` (0x3af428): NUL the first
/// tab/CR in `s` (remembering it), look `s` up in the five justification
/// names (`AV_StrICmp`, values from the table beside them), restore the
/// byte unless it sat exactly on the end, and return the value (-1 for no
/// match).
pub fn av_parse_justify(c: &mut Cpu) {
    let entry = c.pc;
    guest(c, |c| av_parse_justify_body(c, entry))
}

pub(crate) fn av_parse_justify_body(c: &mut Cpu, mut pc: u32) -> Flow {
    let frame = Frame { size: 0xc0 };
    loop {
        match pc {
            // Entry: spill registers, point at the name/value tables.
            0x003a_f428 => {
                c.cycles += 16;
                c.g[SP][0] = sx(sp(c).wrapping_sub(0xc0));
                frame.save(c, S3, 0x80);
                frame.save(c, S4, 0x70);
                c.g[S3][0] = 0;
                frame.save(c, S5, 0x60);
                c.g[S4][0] = c.g[A0][0];
                frame.save(c, S6, 0x50);
                c.g[S5][0] = c.g[A1][0];
                frame.save(c, S0, 0xb0);
                c.g[S6][0] = 0x9;
                frame.save(c, S1, 0xa0);
                frame.save(c, S2, 0x90);
                frame.save_ra(c, 0x40);
                c.g[A1][0] = 0x490000;
                c.g[A2][0] = 0x490000;
                pc = 0x003a_f468;
            }
            // Scan head: `s[i]` while `i < len`.
            0x003a_f468 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[S3][0] < c.g[S5][0]) as u64;
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S3][0] as u32));
                if c.g[V0][0] == 0 {
                    pc = 0x003a_f494;
                } else {
                    pc = 0x003a_f474;
                }
            }
            // Tab or CR ends the scan (its byte is remembered, NULled next).
            0x003a_f474 => {
                c.cycles += 4;
                c.g[V0][0] = 0x9;
                c.g[V1][0] = Addr(c.g[A0][0] as u32).lb(c);
                let is_tab = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0xd;
                if is_tab {
                    pc = 0x003a_f48c;
                } else {
                    pc = 0x003a_f484;
                }
            }
            // Any other byte keeps scanning.
            0x003a_f484 => {
                c.cycles += 2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003a_f468) {
                        return Ok(());
                    }
                    pc = 0x003a_f468;
                } else {
                    pc = 0x003a_f48c;
                }
            }
            // Found: NUL the delimiter, remember it.
            0x003a_f48c => {
                c.cycles += 2;
                Addr(c.g[A0][0] as u32).sb(c, 0);
                c.g[S6][0] = c.g[V1][0];
                pc = 0x003a_f494;
            }
            // Copy the five name pointers and values onto the stack.
            0x003a_f494 => {
                c.cycles += 27;
                c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5290));
                c.ldl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
                c.ldr(2, c.g[A3][0] as u32);
                c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
                c.sdl(2, frame.at(c, 0x7));
                c.sdr(2, frame.at(c, 0));
                c.sdl(3, frame.at(c, 0xf));
                c.sdr(3, frame.at(c, 0x8));
                c.sw(frame.at(c, 0x10), c.g[A0][0]);
                c.g[S1][0] = 0xffffffffffff0000;
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x52a8));
                c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.ldr(2, c.g[A1][0] as u32);
                c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                c.sdl(2, frame.at(c, 0x27));
                c.sdr(2, frame.at(c, 0x20));
                c.sdl(3, frame.at(c, 0x2f));
                c.sdr(3, frame.at(c, 0x28));
                c.sw(frame.at(c, 0x30), c.g[A0][0]);
                c.g[S1][0] |= 0xffff;
                c.g[S2][0] = 0;
                c.g[S0][0] = c.g[SP][0];
                pc = 0x003a_f504;
            }
            // Next table entry.
            0x003a_f500 => {
                c.cycles += 1;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                pc = 0x003a_f504;
            }
            // All five tried: give up with -1.
            0x003a_f504 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[S2][0] < 0x5) as u64;
                if c.g[V0][0] == 0 {
                    pc = 0x003a_f528;
                } else {
                    pc = 0x003a_f510;
                }
            }
            // Compare one table name; on mismatch advance to the next.
            0x003a_f510 => {
                c.cycles += 5;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A1][0] = c.g[S4][0];
                AV_STR_ICMP.call(c, 0x003a_f51c)?;
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    if back_edge_unwinds(c, 0x003a_f500) {
                        return Ok(());
                    }
                    pc = 0x003a_f500;
                } else {
                    pc = 0x003a_f524;
                }
            }
            // Match: the value beside the name.
            0x003a_f524 => {
                c.cycles += 1;
                c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x20)).lw(c);
                pc = 0x003a_f528;
            }
            // Restore the delimiter unless it sat on the end.
            0x003a_f528 => {
                c.cycles += 2;
                let at_end = c.g[S3][0] == c.g[S5][0];
                c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S3][0] as u32));
                if !at_end {
                    c.cycles += 1;
                    let at = c.g[V0][0];
                    let delim = c.g[S6][0];
                    Addr(at as u32).sb(c, delim);
                    pc = 0x003a_f534;
                } else {
                    pc = 0x003a_f534;
                }
            }
            // Return the value.
            0x003a_f534 => {
                c.cycles += 11;
                c.g[V0][0] = c.g[S1][0];
                frame.restore(c, S0, 0xb0);
                frame.restore(c, S1, 0xa0);
                frame.restore(c, S2, 0x90);
                frame.restore(c, S3, 0x80);
                frame.restore(c, S4, 0x70);
                frame.restore(c, S5, 0x60);
                frame.restore(c, S6, 0x50);
                frame.restore_ra(c, 0x40);
                frame.close(c);
                return Ok(());
            }
            _ => {
                c.jump_out(pc, 0x003a_f428, 0x003a_f560);
                return Ok(());
            }
        }
    }
}

/// `void AV_FindToken(char** ps, int* plen, ...)` (0x3af670): skip leading
/// whitespace, then match `*ps` against the `nwords` (word, length) pairs:
/// on a full word match advance `*ps` past it, shrink `*plen`, and store the
/// word index; otherwise clear the outputs (`*ps` = 0, `*plen` = 0,
/// index = -1).
pub fn av_find_token(c: &mut Cpu) {
    let entry = c.pc;
    guest(c, |c| av_find_token_body(c, entry))
}

pub(crate) fn av_find_token_body(c: &mut Cpu, mut pc: u32) -> Flow {
    let frame = Frame { size: 0x80 };
    loop {
        match pc {
            // Entry: spill registers; empty input clears the outputs.
            0x003a_f670 => {
                c.cycles += 17;
                c.g[SP][0] = sx(sp(c).wrapping_sub(0x80));
                frame.save(c, S0, 0x70);
                frame.save(c, S1, 0x60);
                c.g[S0][0] = c.g[A1][0];
                frame.save(c, S2, 0x50);
                c.g[S1][0] = c.g[T0][0];
                frame.save(c, S3, 0x40);
                c.g[S2][0] = c.g[A0][0];
                frame.save(c, S4, 0x30);
                c.g[S3][0] = c.g[T1][0];
                frame.save(c, S5, 0x20);
                c.g[S4][0] = c.g[A3][0];
                frame.save_ra(c, 0x10);
                c.g[S5][0] = c.g[A2][0];
                c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                let empty = c.g[V0][0] == 0;
                c.sw(frame.at(c, 0), c.g[V0][0]);
                if !empty {
                    pc = 0x003a_f6e0;
                } else {
                    pc = 0x003a_f6b4;
                }
            }
            // Null input: `*ps` = 0, then the shared failure exit.
            0x003a_f6b4 => {
                c.cycles += 2;
                Addr(c.g[S2][0] as u32).sw(c, 0);
                pc = 0x003a_f7b8;
            }
            // Match: advance `*ps` past the word, shrink `*plen`, store the
            // word index; then the shared epilogue.
            0x003a_f6bc => {
                c.cycles += 9;
                c.g[V0][0] = c.lw(frame.at(c, 0));
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let start = c.g[V0][0];
                Addr(c.g[S2][0] as u32).sw(c, start);
                c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                let left = c.g[V1][0];
                Addr(c.g[S0][0] as u32).sw(c, left);
                let word = c.g[T3][0];
                Addr(c.g[S3][0] as u32).sw(c, word);
                pc = 0x003a_f7c8;
            }
            // Skip whitespace; an empty remainder fails.
            0x003a_f6e0 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S0][0];
                AV_SKIP_WHITESPACE.call(c, 0x003a_f6ec)?;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                let empty = c.g[V0][0] == 0;
                c.g[A3][0] = 0;
                if empty {
                    pc = 0x003a_f7b4;
                } else {
                    pc = 0x003a_f6f8;
                }
            }
            // Remember the (post-skip) length.
            0x003a_f6f8 => {
                c.cycles += 2;
                c.g[T5][0] = c.g[V0][0];
                pc = 0x003a_f700;
            }
            // Next start position; no words left means advance.
            0x003a_f700 => {
                c.cycles += 2;
                let no_words = c.g[S1][0] == 0;
                c.g[T3][0] = 0;
                if no_words {
                    pc = 0x003a_f7a4;
                } else {
                    pc = 0x003a_f708;
                }
            }
            // Point at this position's string, word list and length list.
            0x003a_f708 => {
                c.cycles += 4;
                c.g[T4][0] = c.lw(frame.at(c, 0));
                c.g[T1][0] = c.g[S4][0];
                c.g[T0][0] = c.g[S5][0];
                pc = 0x003a_f718;
            }
            // Next word: empty words match immediately.
            0x003a_f718 => {
                c.cycles += 4;
                c.g[T2][0] = Addr(c.g[T1][0] as u32).lw(c);
                c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T4][0] as u32));
                let empty_word = c.g[T2][0] == 0;
                c.g[A1][0] = 0;
                if empty_word {
                    pc = 0x003a_f788;
                } else {
                    pc = 0x003a_f728;
                }
            }
            // Past the end: this word cannot match here.
            0x003a_f728 => {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V0][0] = (c.g[A3][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
                    pc = 0x003a_f78c;
                } else {
                    pc = 0x003a_f738;
                }
            }
            // First bytes differ: this word cannot match here.
            0x003a_f738 => {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[T0][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[A2][0] as u32).lb(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
                if c.g[V1][0] != c.g[A0][0] {
                    c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
                    pc = 0x003a_f78c;
                } else {
                    pc = 0x003a_f74c;
                }
            }
            // One more byte matches.
            0x003a_f74c => {
                c.cycles += 1;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                pc = 0x003a_f750;
            }
            // Whole word matched when the count reaches its length.
            0x003a_f750 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[A1][0] < c.g[T2][0]) as u64;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                if c.g[V0][0] == 0 {
                    pc = 0x003a_f788;
                } else {
                    pc = 0x003a_f75c;
                }
            }
            // Still inside the remainder: compare the next byte pair.
            0x003a_f75c => {
                c.cycles += 5;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
                    pc = 0x003a_f78c;
                } else {
                    pc = 0x003a_f770;
                }
            }
            // Equal so far: count the byte and keep matching.
            0x003a_f770 => {
                c.cycles += 6;
                c.g[V0][0] = Addr(c.g[T0][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[A2][0] as u32).lb(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
                if c.g[V1][0] == c.g[A0][0] {
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003a_f750) {
                        return Ok(());
                    }
                    pc = 0x003a_f750;
                } else {
                    pc = 0x003a_f788;
                }
            }
            // Length reached: recheck the count against this word's length.
            0x003a_f788 => {
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
                pc = 0x003a_f78c;
            }
            // Full match: consume the word. Otherwise try the next word.
            0x003a_f78c => {
                c.cycles += 2;
                let matched = c.g[A1][0] == c.g[V0][0];
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x4));
                if matched {
                    if back_edge_unwinds(c, 0x003a_f6bc) {
                        return Ok(());
                    }
                    pc = 0x003a_f6bc;
                } else {
                    pc = 0x003a_f794;
                }
            }
            // Next word.
            0x003a_f794 => {
                c.cycles += 4;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[T3][0] < c.g[S1][0]) as u64;
                let more_words = c.g[V0][0] != 0;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x4));
                if more_words {
                    if back_edge_unwinds(c, 0x003a_f718) {
                        return Ok(());
                    }
                    pc = 0x003a_f718;
                } else {
                    pc = 0x003a_f7a4;
                }
            }
            // Next start position.
            0x003a_f7a4 => {
                c.cycles += 4;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[A3][0] < c.g[T5][0]) as u64;
                let more = c.g[V0][0] != 0;
                if more {
                    if back_edge_unwinds(c, 0x003a_f700) {
                        return Ok(());
                    }
                    pc = 0x003a_f700;
                } else {
                    pc = 0x003a_f7b4;
                }
            }
            // No match anywhere: `*ps` = 0, then the shared failure exit.
            0x003a_f7b4 => {
                c.cycles += 1;
                Addr(c.g[S2][0] as u32).sw(c, 0);
                pc = 0x003a_f7b8;
            }
            // Failure exit: `*plen` = 0, index = -1.
            0x003a_f7b8 => {
                c.cycles += 4;
                c.g[V0][0] = 0xffffffffffff0000;
                Addr(c.g[S0][0] as u32).sw(c, 0);
                c.g[V0][0] |= 0xffff;
                let none = c.g[V0][0];
                Addr(c.g[S3][0] as u32).sw(c, none);
                pc = 0x003a_f7c8;
            }
            // Shared epilogue.
            0x003a_f7c8 => {
                c.cycles += 9;
                frame.restore(c, S0, 0x70);
                frame.restore(c, S1, 0x60);
                frame.restore(c, S2, 0x50);
                frame.restore(c, S3, 0x40);
                frame.restore(c, S4, 0x30);
                frame.restore(c, S5, 0x20);
                frame.restore_ra(c, 0x10);
                frame.close(c);
                return Ok(());
            }
            _ => {
                c.jump_out(pc, 0x003a_f670, 0x003a_f7ec);
                return Ok(());
            }
        }
    }
}

/// `int AV_Player_IsFrameDue(AV_Player* this)` (0x3ae780): the media time
/// plus the base at +0x40, scaled by the codec frame rate: 1 while the
/// scaled presentation time has not passed one frame, else 0. Negative
/// intermediate values take a halving path through the float conversion.
pub fn av_player_is_frame_due(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        AV_PLAYER_GET_TIME_MS.call(c, 0x003a_e798)?;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x64).lw(c);
        c.g[S0][0] = c.g[V0][0];
        RCMP_DECODER_GET_FRAME_RATE.call(c, 0x003a_e7a4)?;
        // Clock plus base; negative sums halve into the float conversion.
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x40).lw(c);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let negative = (c.g[V0][0] as i64) < 0;
        c.mov_s(2, 0);
        if negative {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[V0][0] &= 0x1;
            c.g[V0][0] |= c.g[V1][0];
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.add_s(1, 1, 1);
        } else {
            c.cycles += 4;
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
        }
        // Scale by the rate and the second threshold word.
        c.cycles += 8;
        c.g[AT][0] = 0x3a830000;
        c.g[AT][0] |= 0x126f;
        c.mtc1(1, 0);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x44).lw(c);
        c.mul_s(0, 1, 0);
        c.mul_s(0, 0, 2);
        let negative2 = (c.g[V0][0] as i64) < 0;
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x48));
        if negative2 {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[V0][0] &= 0x1;
            c.g[V0][0] |= c.g[V1][0];
            c.mtc1(2, 2);
            c.cvt_s_w(2, 2);
            c.add_s(2, 2, 2);
        } else {
            c.cycles += 4;
            c.mtc1(2, 2);
            c.cvt_s_w(2, 2);
        }
        // Due when a full frame's worth has accumulated.
        c.cycles += 8;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x48));
        c.add_s(0, 2, 0);
        c.c_lt_s(1, 0);
        let over = c.fcc();
        c.g[V0][0] = 0;
        if !over {
            c.cycles += 1;
            c.g[V0][0] = 0x1;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AV_Player_GetTimeMs(AV_Player* this)` (0x3ae9a0): the media clock in
/// milliseconds (`accum * 1000 / tick_rate`, minus the drift offset). While
/// enabled, the sound locks are held to sample the stream status, and the
/// drift tracker folds the output-latency-corrected error back into the
/// offset, resetting it once it grows past 264 ms.
pub fn av_player_get_time_ms(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x5c).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0042_e508, 0x003a_e9c4)?;
        call_ptr(c, 0x003e_5008, 0x003a_e9cc)?;
        // The tick rate is re-tested here; both arms fall through.
        c.g[A0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x8));
        // Milliseconds from the 1000x-scaled accumulator.
        c.cycles += 12;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x4c).lw(c);
        c.g[V1][0] = c.g[A0][0] << 5;
        c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0x54).lw(c);
        c.g[V1][0] = c.g[V1][0].wrapping_sub(c.g[A0][0]);
        c.g[V1][0] = c.g[V1][0] << 2;
        c.g[V1][0] = c.g[V1][0].wrapping_add(c.g[A0][0]);
        c.g[V1][0] = c.g[V1][0] << 35;
        c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
        c.divu(0, V1, V0);
        c.g[S2][0] = c.lo[0];
        let sound_off = c.g[A1][0] == 0;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[A2][0] as u32));
        if !sound_off {
            c.cycles += 19;
            SND_LOCK.call(c, 0x003a_ea10)?;
            c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
            c.g[A1][0] = sx(frame.at(c, 0x10));
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            SNDSTRM_GET_STATUS.call(c, 0x003a_ea20)?;
            c.g[A0][0] = c.lw(frame.at(c, 0x14));
            c.g[A1][0] = c.g[SP][0];
            SNDSTRM_GET_REQUEST_STATUS.call(c, 0x003a_ea2c)?;
            SND_UNLOCK.call(c, 0x003a_ea34)?;
            SND_GET_OUTPUT_LATENCY.call(c, 0x003a_ea3c)?;
            // Behind only when the stream's last position lags the sample.
            c.g[A0][0] = c.lw(frame.at(c, 0x4));
            c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x50).lw(c);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            let behind = (c.g[V1][0] as i64) < (c.g[V0][0] as i64);
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S0][0] as u32));
            if behind {
                c.cycles += 15;
                c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x58).lw(c);
                c.g[A2][0] = u64::MAX;
                let fresh = c.g[V0][0];
                Addr(c.g[S1][0] as u32).off(0x50).sw(c, fresh);
                let nonneg = (c.g[A2][0] as i64) < (c.g[A0][0] as i64);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7));
                if nonneg {
                    c.g[V0][0] = c.g[A0][0];
                }
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A0][0] as u32));
                let negative = (c.g[V0][0] as i64) < (c.g[A0][0] as i64);
                if negative {
                    c.g[V0][0] = c.g[A0][0];
                }
                let settled = (c.g[V0][0] as i64) < 265;
                let drift = c.g[A0][0];
                Addr(c.g[S1][0] as u32).off(0x58).sw(c, drift);
                if !settled {
                    c.cycles += 9;
                    let nonneg2 = (c.g[A2][0] as i64) < (c.g[A0][0] as i64);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7));
                    if nonneg2 {
                        c.g[V1][0] = c.g[A0][0];
                    }
                    Addr(c.g[S1][0] as u32).off(0x58).sw(c, 0);
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x4c).lw(c);
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    let offset = c.g[A0][0];
                    Addr(c.g[S1][0] as u32).off(0x4c).sw(c, offset);
                    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x40);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, check_lohi, require_paths, DATA};
    use crate::difftest::elf_image;
    use crate::game::wscript::wscriptman::tests::alloc_ready;
    use crate::game::wscript::wscriptman::tests::clear_3;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Player base for the ctor tests: 16-aligned, nonzero low byte, cleared
    /// like the sibling ctor_2 test (`AV_Player_Init` dereferences it).
    pub(crate) const AVPLAYER_THIS: u32 = DATA + 0x1410;

    #[test]
    fn av_player_ctor_matches() {
        check(
            ee::s13::f_003addc0__generated,
            av_player_ctor,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3add_c0, 0);
                alloc_ready(c);
                clear_3(c, AVPLAYER_THIS, 0x100);
                c.g[A0][0] = AVPLAYER_THIS as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[A2][0] = if seed / 2 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[A3][0] = if seed / 4 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T0][0] = if seed / 8 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    #[test]
    fn av_player_ctor_3_matches() {
        check(
            ee::s13::f_003ade58__generated,
            av_player_ctor_3,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3ade_58, 0);
                alloc_ready(c);
                clear_3(c, AVPLAYER_THIS, 0x100);
                c.g[A0][0] = AVPLAYER_THIS as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[A2][0] = if seed / 2 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[A3][0] = if seed / 4 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T0][0] = if seed / 8 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T1][0] = if seed / 16 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T2][0] = if seed / 32 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T3][0] = if seed / 64 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                // The eighth argument arrives on the caller's stack.
                c.bus.write32(
                    crate::difftest::SP + 0x30,
                    if seed / 128 % 2 == 0 { 0 } else { r.next_u32() },
                );
            },
        );
    }

    #[test]
    fn av_player_ctor_4_matches() {
        check(
            ee::s13::f_003ade90__generated,
            av_player_ctor_4,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3ade_90, 0);
                alloc_ready(c);
                clear_3(c, AVPLAYER_THIS, 0x100);
                c.g[A0][0] = AVPLAYER_THIS as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[A2][0] = if seed / 2 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[A3][0] = if seed / 4 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T0][0] = if seed / 8 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T1][0] = if seed / 16 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T2][0] = if seed / 32 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T3][0] = if seed / 64 % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    /// Gate for the stream-file fakes below: they answer only while this word
    /// holds `AVCHUNK_CTRL_KEY` (otherwise the slot body they replaced runs).
    pub(crate) const AVCHUNK_CTRL: u32 = DATA + 0x3000;
    pub(crate) const AVCHUNK_CTRL_KEY: u32 = 0x4156_5052;

    static AVCHUNK_PREV_GETCHUNK: AtomicUsize = AtomicUsize::new(0);
    static AVCHUNK_PREV_ISIDLE: AtomicUsize = AtomicUsize::new(0);

    fn avchunk_gated(c: &mut Cpu) -> bool {
        c.bus.read32(AVCHUNK_CTRL) == AVCHUNK_CTRL_KEY
    }

    /// `StreamFile_GetChunk`: no chunk queued.
    fn avchunk_fake_getchunk(c: &mut Cpu) {
        if !avchunk_gated(c) {
            return fakes::fall_back(c, &AVCHUNK_PREV_GETCHUNK, ee::s14::f_003e12e0__generated);
        }
        fakes::fake_call(c, 0x003e_12e0);
        c.g[V0][0] = 0;
    }

    /// `StreamFile_IsIdle`: the stream is idle, so the chunk loop exits.
    fn avchunk_fake_isidle(c: &mut Cpu) {
        if !avchunk_gated(c) {
            return fakes::fall_back(c, &AVCHUNK_PREV_ISIDLE, ee::s14::f_003e1530__generated);
        }
        fakes::fake_call(c, 0x003e_1530);
        c.g[V0][0] = 1;
    }

    fn avchunk_install_stream_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E12E0),
            avchunk_fake_getchunk,
            &AVCHUNK_PREV_GETCHUNK,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E1530),
            avchunk_fake_isidle,
            &AVCHUNK_PREV_ISIDLE,
        );
    }

    /// Chunk-callback fixture pieces (all 16-aligned, nonzero low byte).
    pub(crate) const AVCHUNK_PLAYER: u32 = DATA + 0x1410;
    pub(crate) const AVCHUNK_HOLDER: u32 = DATA + 0x1450;
    pub(crate) const AVCHUNK_CHUNK: u32 = DATA + 0x1460;
    pub(crate) const AVCHUNK_STREAM: u32 = DATA + 0x1500;
    pub(crate) const AVCHUNK_MAGIC: u32 = DATA + 0x1510;
    pub(crate) const AVCHUNK_SCRATCH: u32 = DATA + 0x1700;

    #[test]
    fn av_player_get_chunk_callback_matches() {
        // A valid stream with no chunk queued: `GetChunk` returns 0 and
        // `IsIdle` reports idle, so `OnChunk` takes its short path. (Its
        // deeper arms belong to `OnChunk`'s own future test.)
        check(
            ee::s13::f_003aecb8__generated,
            av_player_get_chunk_callback,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aec_b8, 0);
                avchunk_install_stream_fakes();
                c.bus.write32(AVCHUNK_CTRL, AVCHUNK_CTRL_KEY);
                c.bus.write32(AVCHUNK_MAGIC, 0x4d52_5453);
                c.bus.write32(AVCHUNK_MAGIC + 0x38, 1);
                for i in 0..8 {
                    c.bus.write32(AVCHUNK_PLAYER + 4 * i, 0xdead_beef);
                    c.bus.write32(AVCHUNK_CHUNK + 4 * i, r.next_u32());
                }
                c.bus.write32(AVCHUNK_CHUNK + 0x30, AVCHUNK_STREAM);
                c.bus.write32(AVCHUNK_HOLDER, AVCHUNK_CHUNK);
                c.bus.write32(AVCHUNK_STREAM, AVCHUNK_MAGIC);
                c.bus.write32(AVCHUNK_STREAM + 0x8, 0);
                c.bus.write32(AVCHUNK_SCRATCH, 0xdead_beef);
                c.g[A0][0] = AVCHUNK_PLAYER as u64;
                c.g[A1][0] = AVCHUNK_HOLDER as u64;
                c.g[A2][0] = AVCHUNK_SCRATCH as u64;
            },
        );
    }

    /// Int-field fixture: the pointer, length and output words plus the
    /// 16-byte head scratch.
    pub(crate) const AVINT_PP: u32 = DATA + 0x1410;
    pub(crate) const AVINT_PLEN: u32 = DATA + 0x1414;
    pub(crate) const AVINT_OUT: u32 = DATA + 0x1418;
    pub(crate) const AVINT_STR: u32 = DATA + 0x1420;

    fn av_int_setup(c: &mut Cpu, seed: u32) {
        // `seed % 4`: 0 empty (returns 0, output untouched), 1 delimiter
        // mid-string (restore + value), 2 delimiter first (empty head,
        // returns 0), 3 digits-or-nothing to the end. A short length stops
        // the scan before the delimiter.
        machine(c, seed, 0x3af5_60, 0);
        for i in 0..8 {
            c.bus.write32(AVINT_STR + 4 * i, 0xdead_beef);
        }
        c.bus.write32(AVINT_PP, AVINT_STR);
        c.bus.write32(AVINT_OUT, 0xdead_beef);
        let (text, len): (&[u8], u32) = match seed % 4 {
            0 => (b"", 0),
            1 => (b"12\n34", 5),
            2 => (b"\n789", 4),
            _ => match seed / 4 % 3 {
                0 => (b"456", 3),
                1 => (b"-12", 3),
                _ => (b"abc", 3),
            },
        };
        for (i, &b) in text.iter().enumerate() {
            c.bus.write8(AVINT_STR + i as u32, b);
        }
        c.bus.write8(AVINT_STR + text.len() as u32, 0);
        let len = if seed % 4 == 1 && seed / 16 % 2 == 0 {
            2
        } else {
            len
        };
        c.bus.write32(AVINT_PLEN, 0xdead_beef);
        c.bus.write32(AVINT_PLEN, len);
        c.g[A0][0] = AVINT_PP as u64;
        c.g[A1][0] = AVINT_PLEN as u64;
        c.g[A2][0] = AVINT_OUT as u64;
    }

    #[test]
    fn av_parse_int_field_matches() {
        check(
            ee::s13::f_003af560__generated,
            av_parse_int_field,
            200,
            &av_int_setup,
        );
    }

    /// Vertical-tab bytes (0x0b): first-byte and mid-string VT hit the
    /// `(byte-9) < 0x2` class boundary (V0==2) that tab/LF/digits never do.
    pub(crate) fn av_int_vt_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x3af5_60, 0);
        for i in 0..8 {
            c.bus.write32(AVINT_STR + 4 * i, 0xdead_beef);
        }
        c.bus.write32(AVINT_PP, AVINT_STR);
        c.bus.write32(AVINT_OUT, 0xdead_beef);
        // VT first (empty-class hit at once), VT mid-string (scan finds
        // it), VT only (scan to end after VT? no: VT ends the scan).
        let (text, len): (&[u8], u32) = match seed % 3 {
            0 => (b"\x0b789", 4),
            1 => (b"45\x0b67", 5),
            _ => (b"\x0b", 1),
        };
        for (i, &b) in text.iter().enumerate() {
            c.bus.write8(AVINT_STR + i as u32, b);
        }
        c.bus.write8(AVINT_STR + text.len() as u32, 0);
        c.bus.write32(AVINT_PLEN, 0xdead_beef);
        c.bus.write32(AVINT_PLEN, len);
        c.g[A0][0] = AVINT_PP as u64;
        c.g[A1][0] = AVINT_PLEN as u64;
        c.g[A2][0] = AVINT_OUT as u64;
    }

    #[test]
    fn av_parse_int_field_vt_matches() {
        check(
            ee::s13::f_003af560__generated,
            av_parse_int_field,
            96,
            &av_int_vt_setup,
        );
    }

    /// Store-path fixture: the game image for `_strtol_r`'s ctype table at
    /// `0x499c81` (without it no byte is ever a digit and `at_end` stays
    /// true), then digit texts that all take the `!at_end` store in
    /// generated block `0x3af64c` (hand wscriptcache.rs:9577-9581).
    /// `seed % 4`: 0 "456", 1 "-7", 2 "0", 3 "12x" (stops mid-way, still
    /// stores 12). Outputs are prefilled `0xdead_beef` so every store is
    /// checked.
    pub(crate) fn av_int_store_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x3af5_60, 0);
        elf_image(c);
        for i in 0..8 {
            c.bus.write32(AVINT_STR + 4 * i, 0xdead_beef);
        }
        c.bus.write32(AVINT_PP, AVINT_STR);
        c.bus.write32(AVINT_OUT, 0xdead_beef);
        let (text, len): (&[u8], u32) = match seed % 4 {
            0 => (b"456", 3),
            1 => (b"-7", 2),
            2 => (b"0", 1),
            _ => (b"12x", 3),
        };
        for (i, &b) in text.iter().enumerate() {
            c.bus.write8(AVINT_STR + i as u32, b);
        }
        c.bus.write8(AVINT_STR + text.len() as u32, 0);
        c.bus.write32(AVINT_PLEN, 0xdead_beef);
        c.bus.write32(AVINT_PLEN, len);
        c.g[A0][0] = AVINT_PP as u64;
        c.g[A1][0] = AVINT_PLEN as u64;
        c.g[A2][0] = AVINT_OUT as u64;
    }

    #[test]
    fn av_parse_int_field_store_matches() {
        check(
            ee::s13::f_003af560__generated,
            av_parse_int_field,
            32,
            &av_int_store_setup,
        );
        // Stored-value proof that generated block `0x3af64c`'s `!at_end`
        // store ran on the hand body: each value differs from the
        // `0xdead_beef` prefill (including "0", which stores zero) and the
        // return is 1.
        for (seed, want) in [(0u32, 456u32), (1, (-7i32) as u32), (2, 0), (3, 12)] {
            let mut c = crate::difftest::fresh();
            av_int_store_setup(&mut c, seed);
            av_parse_int_field(&mut c);
            assert_eq!(c.bus.read32(AVINT_OUT), want, "seed {seed}");
            assert_eq!(c.g[V0][0], 1, "seed {seed} return");
        }
    }

    #[test]
    fn av_parse_int_field_events() {
        // The scan loop's bare event check delivers without unwinding, so
        // both sides stay in lockstep through the driver. `strtol` runs
        // real (leaf, no event checks) on both sides.
        driven!(av_int_gen_driven, ee::s13::f_003af560__generated);
        driven!(av_int_hand_driven, av_parse_int_field);
        check_events(av_int_gen_driven, av_int_hand_driven, 96, &|c, seed| {
            let mut r = machine(c, seed, 0x3af5_60, 0);
            av_int_setup(c, seed);
            c.pc = 0x003a_f560;
            schedule(c, &mut r, seed, 0x003a_f560)
        });
    }

    /// Justify fixture: the game-fixed name/value tables plus subject and
    /// dictionary scratch.
    pub(crate) const AVJUSTIFY_NAMES: u32 = 0x0049_5290;
    pub(crate) const AVJUSTIFY_VALS: u32 = 0x0049_52a8;
    pub(crate) const AVJUSTIFY_S: u32 = DATA + 0x1410;
    pub(crate) const AVJUSTIFY_DICT: u32 = DATA + 0x1500;
    pub(crate) const AVJUSTIFY_CTYPE: u32 = 0x0049_9c81;

    fn av_justify_setup(c: &mut Cpu, seed: u32) {
        // `seed % 6`: 0 empty (miss), 1 exact (first entry), 2 case-folded
        // (second entry), 3 tabbed (miss + restore), 4 CR (fourth entry +
        // restore), 5 unknown (miss, no restore).
        machine(c, seed, 0x3af4_28, 0);
        for ch in b'A'..=b'Z' {
            c.bus.write8(AVJUSTIFY_CTYPE + ch as u32, 1);
        }
        const WORDS: [&[u8]; 5] = [b"left", b"right", b"center", b"top", b"up"];
        const VALS: [u32; 5] = [10, 20, 30, 40, 50];
        for (i, w) in WORDS.iter().enumerate() {
            let at = AVJUSTIFY_DICT + i as u32 * 0x10;
            for (j, &b) in w.iter().enumerate() {
                c.bus.write8(at + j as u32, b);
            }
            c.bus.write8(at + w.len() as u32, 0);
            c.bus.write32(AVJUSTIFY_NAMES + 4 * i as u32, at);
            c.bus.write32(AVJUSTIFY_VALS + 4 * i as u32, VALS[i]);
        }
        for i in 0..8 {
            c.bus.write32(AVJUSTIFY_S + 4 * i, 0xdead_beef);
        }
        let (text, len): (&[u8], u32) = match seed % 6 {
            0 => (b"", 0),
            1 => (b"left", 4),
            2 => (b"RIGHT", 5),
            3 => (b"cen\tter", 7),
            4 => (b"top\r", 4),
            _ => (b"zzz", 3),
        };
        for (i, &b) in text.iter().enumerate() {
            c.bus.write8(AVJUSTIFY_S + i as u32, b);
        }
        c.bus.write8(AVJUSTIFY_S + text.len() as u32, 0);
        c.pc = 0x003a_f428;
        c.g[A0][0] = AVJUSTIFY_S as u64;
        c.g[A1][0] = len as u64;
    }

    #[test]
    fn av_parse_justify_matches() {
        check(
            ee::s13::f_003af428__generated,
            av_parse_justify,
            200,
            &av_justify_setup,
        );
    }

    #[test]
    fn av_parse_justify_paths() {
        // One total per arm (seed % 6): empty 182, exact 210, case-folded
        // 281, tabbed 348, CR 304, unknown 284. Inputs are fixed per arm,
        // so the `AV_StrICmp` costs are stable.
        require_paths(
            ee::s13::f_003af428__generated,
            200,
            &[182, 210, 281, 348, 304, 284],
            &av_justify_setup,
        );
    }

    #[test]
    fn av_parse_justify_events() {
        // Both back edges unwind through `drive` and resume at their heads;
        // `AV_StrICmp` runs real (leaf, no event checks) on both sides.
        driven!(av_justify_gen_driven, ee::s13::f_003af428__generated);
        driven!(av_justify_hand_driven, av_parse_justify);
        check_events(
            av_justify_gen_driven,
            av_justify_hand_driven,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3af4_28, 0);
                av_justify_setup(c, seed);
                c.pc = 0x003a_f428;
                schedule(c, &mut r, seed, 0x003a_f428)
            },
        );
    }

    /// Token fixture: subject/pointer/length/output words, the word and
    /// length lists, and dictionary scratch.
    pub(crate) const AVTOKEN_PS: u32 = DATA + 0x1410;
    pub(crate) const AVTOKEN_PLEN: u32 = DATA + 0x1414;
    pub(crate) const AVTOKEN_OUT: u32 = DATA + 0x1418;
    pub(crate) const AVTOKEN_WORDS: u32 = DATA + 0x1420;
    pub(crate) const AVTOKEN_LENS: u32 = DATA + 0x1430;
    pub(crate) const AVTOKEN_S: u32 = DATA + 0x1440;
    pub(crate) const AVTOKEN_DICT: u32 = DATA + 0x1500;
    pub(crate) const AVTOKEN_CTYPE: u32 = 0x0049_9c81;

    fn av_find_token_setup(c: &mut Cpu, seed: u32) {
        // `seed % 7`: 0 null subject, 1 empty remainder, 2 match at (0,0),
        // 3 empty-word match, 4 match at (3,1), 5 leading whitespace, 6 no
        // match (with and without words).
        machine(c, seed, 0x3af6_70, 0);
        for ch in [0x20u32, 0x09, 0x0a, 0x0d] {
            c.bus.write8(AVTOKEN_CTYPE + ch, 8);
        }
        for ch in b'A'..=b'Z' {
            c.bus.write8(AVTOKEN_CTYPE + ch as u32, 1);
        }
        const WORDS: [&[u8]; 3] = [b"alpha", b"beta", b""];
        for (i, w) in WORDS.iter().enumerate() {
            let at = AVTOKEN_DICT + i as u32 * 0x10;
            for (j, &b) in w.iter().enumerate() {
                c.bus.write8(at + j as u32, b);
            }
            c.bus.write8(at + w.len() as u32, 0);
            c.bus.write32(AVTOKEN_WORDS + 4 * i as u32, at);
            c.bus.write32(AVTOKEN_LENS + 4 * i as u32, w.len() as u32);
        }
        for i in 0..8 {
            c.bus.write32(AVTOKEN_S + 4 * i, 0xdead_beef);
        }
        let (text, len, nwords): (Option<&[u8]>, u32, u32) = match seed % 7 {
            0 => (None, 0, 3),
            1 => (Some(b""), 0, 2),
            2 => (Some(b"alpha rest"), 10, 3),
            3 => (Some(b"zz beta"), 7, 3),
            4 => (Some(b"zz beta"), 7, 2),
            5 => (Some(b"  alpha"), 7, 3),
            _ => (Some(b"zzz"), 3, if seed / 7 % 2 == 0 { 2 } else { 0 }),
        };
        if let Some(text) = text {
            for (i, &b) in text.iter().enumerate() {
                c.bus.write8(AVTOKEN_S + i as u32, b);
            }
            c.bus.write8(AVTOKEN_S + text.len() as u32, 0);
            c.bus.write32(AVTOKEN_PS, AVTOKEN_S);
        } else {
            c.bus.write32(AVTOKEN_PS, 0);
        }
        c.bus.write32(AVTOKEN_PLEN, 0xdead_beef);
        c.bus.write32(AVTOKEN_PLEN, len);
        c.bus.write32(AVTOKEN_OUT, 0xdead_beef);
        c.pc = 0x003a_f670;
        c.g[A0][0] = AVTOKEN_PS as u64;
        c.g[A1][0] = AVTOKEN_PLEN as u64;
        c.g[A2][0] = AVTOKEN_WORDS as u64;
        c.g[A3][0] = AVTOKEN_LENS as u64;
        c.g[T0][0] = nwords as u64;
        c.g[T1][0] = AVTOKEN_OUT as u64;
    }

    #[test]
    fn av_find_token_matches() {
        check(
            ee::s13::f_003af670__generated,
            av_find_token,
            200,
            &av_find_token_setup,
        );
    }

    #[test]
    fn av_find_token_paths() {
        // One total per arm (seed % 7): null 32, empty 47, match-(0,0) 143,
        // empty-word 112, match-(3,1) 292, whitespace 163, no-match 201
        // (two words) or 75 (no words). Inputs are fixed per arm, so the
        // `AV_SkipWhitespace` costs are stable.
        require_paths(
            ee::s13::f_003af670__generated,
            200,
            &[32, 47, 143, 112, 292, 163, 201, 75],
            &av_find_token_setup,
        );
    }

    #[test]
    fn av_find_token_events() {
        // All four back edges unwind through `drive` and resume at their
        // heads; `AV_SkipWhitespace` runs real (bare check only, it never
        // unwinds) on both sides.
        driven!(av_token_gen_driven, ee::s13::f_003af670__generated);
        driven!(av_token_hand_driven, av_find_token);
        check_events(av_token_gen_driven, av_token_hand_driven, 96, &|c, seed| {
            let mut r = machine(c, seed, 0x3af6_70, 0);
            av_find_token_setup(c, seed);
            c.pc = 0x003a_f670;
            schedule(c, &mut r, seed, 0x003a_f670)
        });
    }

    /// Frame-due fixture: player, timer and decoder scratch.
    pub(crate) const AVFRAMEDUE_PLAYER: u32 = DATA + 0x1410;
    pub(crate) const AVFRAMEDUE_TIMER: u32 = DATA + 0x1500;
    pub(crate) const AVFRAMEDUE_DECODER: u32 = DATA + 0x1540;
    /// The EATimer tick-rate word both time tests sample.
    pub(crate) const AVTIME_TICK_RATE: u32 = 0x0045_0dc8;

    #[test]
    fn av_player_is_frame_due_matches() {
        // The accumulator stays 0, so the sum sign comes from `base`:
        // -5 (negative), 0 and 1 (positive, under a frame), 5 (positive,
        // over a frame); the threshold word covers both float arms.
        check(
            ee::s13::f_003ae780__generated,
            av_player_is_frame_due,
            200,
            &|c, seed| {
                machine(c, seed, 0x3ae7_80, 0);
                c.bus.write32(AVTIME_TICK_RATE, 1000);
                for i in 0..0x20 {
                    c.bus.write32(AVFRAMEDUE_PLAYER + 4 * i, 0xdead_beef);
                }
                let base: u32 = [-5i32 as u32, 0, 1, 5][seed as usize % 4];
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x40, base);
                c.bus.write32(
                    AVFRAMEDUE_PLAYER + 0x44,
                    if seed / 4 % 2 == 0 { 0xffff_ffff } else { 1 },
                );
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x4c, 0);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x54, 0);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x58, 0);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x5c, AVFRAMEDUE_TIMER);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x64, AVFRAMEDUE_DECODER);
                for i in 0..8 {
                    c.bus.write32(AVFRAMEDUE_TIMER + 4 * i, 0xdead_beef);
                }
                c.bus.write64(AVFRAMEDUE_TIMER + 0x8, 0);
                c.bus.write32(AVFRAMEDUE_TIMER + 0x10, 0x1000);
                c.bus.write32(AVFRAMEDUE_DECODER + 0x14, 0);
                c.g[A0][0] = AVFRAMEDUE_PLAYER as u64;
            },
        );
    }

    /// Edge values: a base that zeroes the clock sum (X=0, halving
    /// boundary), a very negative base (X<0, halving taken), a zero
    /// threshold word (second-float boundary), and a huge accumulator
    /// (nonzero stored product, GetTimeMs >= 0x800000 so the stale-f2 bits
    /// are not a denormal). GetTimeMs returns 0xa92fc on the stock timer,
    /// so base = -0xa92fc zeroes X.
    #[test]
    fn av_player_is_frame_due_edges_matches() {
        check(
            ee::s13::f_003ae780__generated,
            av_player_is_frame_due,
            32,
            &|c, seed| {
                machine(c, seed, 0x3ae7_80, 0);
                c.bus.write32(AVTIME_TICK_RATE, 1000);
                for i in 0..0x20 {
                    c.bus.write32(AVFRAMEDUE_PLAYER + 4 * i, 0xdead_beef);
                }
                let (base, thresh, accum): (u32, u32, u64) = match seed % 4 {
                    0 => (0xa92fc_u32.wrapping_neg(), 1, 0),
                    1 => (0xf0000000, 0xffff_ffff, 0),
                    2 => (5, 0, 0),
                    _ => (5, 1, 0x900000),
                };
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x40, base);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x44, thresh);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x4c, 0);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x54, 0);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x58, 0);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x5c, AVFRAMEDUE_TIMER);
                c.bus.write32(AVFRAMEDUE_PLAYER + 0x64, AVFRAMEDUE_DECODER);
                for i in 0..8 {
                    c.bus.write32(AVFRAMEDUE_TIMER + 4 * i, 0xdead_beef);
                }
                c.bus.write64(AVFRAMEDUE_TIMER + 0x8, accum);
                c.bus.write32(AVFRAMEDUE_TIMER + 0x10, 0x1000);
                c.bus.write32(AVFRAMEDUE_DECODER + 0x14, 0);
                c.g[A0][0] = AVFRAMEDUE_PLAYER as u64;
            },
        );
    }

    /// Time fixture: player, timer and owner scratch plus the SND globals
    /// the status path samples (null stream table, latency flag byte).
    pub(crate) const AVTIME_PLAYER: u32 = DATA + 0x1410;
    pub(crate) const AVTIME_TIMER: u32 = DATA + 0x1500;
    pub(crate) const AVTIME_OWNER: u32 = DATA + 0x1540;
    pub(crate) const AVTIME_STREAM_COUNT: u32 = 0x0050_a92e;
    pub(crate) const AVTIME_STREAM_TABLE: u32 = 0x0050_a0a0;
    /// 0x510000 - 0x55a4: the flag byte `SND_GetOutputLatency` samples.
    pub(crate) const AVTIME_LATENCY: u32 = 0x0051_0000 - 0x55a4;

    #[test]
    fn av_player_get_time_ms_matches() {
        // `seed % 2` enables the sound block; `seed / 2 % 2` lags the
        // stream; `seed / 4 % 3` settles the drift or pushes it past the
        // reset; `seed / 12 % 2` drives both latency-flag arms.
        check_lohi(
            ee::s13::f_003ae9a0__generated,
            av_player_get_time_ms,
            200,
            &|c, seed| {
                machine(c, seed, 0x3ae9_a0, 0);
                c.bus.write32(AVTIME_TICK_RATE, 1000);
                c.bus.write32(AVTIME_STREAM_COUNT, 1);
                c.bus.write32(AVTIME_STREAM_TABLE, 0);
                c.bus.write8(AVTIME_LATENCY, (seed / 12 % 2) as u8);
                for i in 0..0x20 {
                    c.bus.write32(AVTIME_PLAYER + 4 * i, 0xdead_beef);
                }
                c.bus.write32(AVTIME_PLAYER, AVTIME_OWNER);
                let (drift, offset) = match seed / 4 % 3 {
                    0 => (0, 0),
                    1 => (100, 0),
                    _ => (0, 500),
                };
                c.bus.write32(AVTIME_PLAYER + 0x4c, offset);
                c.bus.write32(
                    AVTIME_PLAYER + 0x50,
                    if seed / 2 % 2 == 0 { 0 } else { 0xffff_ffff },
                );
                c.bus.write32(AVTIME_PLAYER + 0x54, (seed % 2) as u32);
                c.bus.write32(AVTIME_PLAYER + 0x58, drift);
                c.bus.write32(AVTIME_PLAYER + 0x5c, AVTIME_TIMER);
                for i in 0..8 {
                    c.bus.write32(AVTIME_TIMER + 4 * i, 0xdead_beef);
                }
                c.bus.write64(AVTIME_TIMER + 0x8, 0);
                c.bus.write32(AVTIME_TIMER + 0x10, 0x1000);
                c.bus.write32(AVTIME_OWNER + 0xc, 0);
                c.g[A0][0] = AVTIME_PLAYER as u64;
            },
        );
    }

    /// Drift-boundary values: drift -1 (the `nonneg` A2=-1 boundary),
    /// drift 7 and 8 (the (A0+7)>>3 shift boundary), and a lagging stream
    /// with drift that propagates past both shifts to the stored fields.
    /// Outputs are prefilled nonzero so every store is checked.
    #[test]
    fn av_player_get_time_ms_drift_matches() {
        check_lohi(
            ee::s13::f_003ae9a0__generated,
            av_player_get_time_ms,
            32,
            &|c, seed| {
                machine(c, seed, 0x3ae9_a0, 0);
                c.bus.write32(AVTIME_TICK_RATE, 1000);
                c.bus.write32(AVTIME_STREAM_COUNT, 1);
                c.bus.write32(AVTIME_STREAM_TABLE, 0);
                c.bus.write8(AVTIME_LATENCY, (seed % 2) as u8);
                for i in 0..0x20 {
                    c.bus.write32(AVTIME_PLAYER + 4 * i, 0xdead_beef);
                }
                c.bus.write32(AVTIME_PLAYER, AVTIME_OWNER);
                let drift: u32 = match seed % 4 {
                    0 => 0xffff_ffff,
                    1 => 7,
                    2 => 8,
                    _ => 100,
                };
                c.bus.write32(AVTIME_PLAYER + 0x4c, 0x1111_1111);
                c.bus.write32(AVTIME_PLAYER + 0x50, 0xffff_ffff);
                c.bus.write32(AVTIME_PLAYER + 0x54, 1);
                c.bus.write32(AVTIME_PLAYER + 0x58, drift);
                c.bus.write32(AVTIME_PLAYER + 0x5c, AVTIME_TIMER);
                for i in 0..8 {
                    c.bus.write32(AVTIME_TIMER + 4 * i, 0xdead_beef);
                }
                c.bus.write64(AVTIME_TIMER + 0x8, 0);
                c.bus.write32(AVTIME_TIMER + 0x10, 0x1000);
                c.bus.write32(AVTIME_OWNER + 0xc, 0);
                c.g[A0][0] = AVTIME_PLAYER as u64;
            },
        );
    }

    #[test]
    fn av_parse_int_field_paths() {
        // Arm totals, one per branch shape (the "456"/"abc" coincidence is
        // real: `strtol` costs the same digits-or-nothing run; RAM differs).
        require_paths(
            ee::s13::f_003af560__generated,
            200,
            &[411, 425, 431, 442, 440, 461],
            &av_int_setup,
        );
    }

    /// Resume frame base for the justify tests (entry opens 0xc0).
    pub(crate) const AVJUSTIFY_FRAME: u32 = DATA + 0x1600;
    /// Resume frame base for the token tests (entry opens 0x80).
    pub(crate) const AVTOKEN_FRAME: u32 = DATA + 0x1600;

    /// Direct-entry resume for `av_parse_justify`'s two used-form heads: the
    /// frame as the entry block leaves it, loop registers at mid-scan
    /// values, compared head by head against the generated body.
    #[test]
    fn av_parse_justify_resume_matches() {
        use crate::difftest::compare;
        for head in [0x003a_f468u32, 0x003a_f500] {
            let r = compare(ee::s13::f_003af428__generated, av_parse_justify, &|c| {
                machine(c, 0x3af4_28, head as u64, 0);
                // Tables and dictionary, as in the matches fixture.
                for ch in b'A'..=b'Z' {
                    c.bus.write8(AVJUSTIFY_CTYPE + ch as u32, 1);
                }
                const WORDS: [&[u8]; 5] = [b"left", b"right", b"center", b"top", b"up"];
                for (i, w) in WORDS.iter().enumerate() {
                    let at = AVJUSTIFY_DICT + i as u32 * 0x10;
                    for (j, b) in w.iter().enumerate() {
                        c.bus.write8(at + j as u32, *b);
                    }
                    c.bus.write8(at + w.len() as u32, 0);
                    c.bus.write32(AVJUSTIFY_NAMES + 4 * i as u32, at);
                    c.bus.write32(AVJUSTIFY_VALS + 4 * i as u32, 10 * i as u32);
                }
                // Subject "cen\tter": the tab at index 3 ends the scan.
                for (i, b) in b"cen\tter".iter().enumerate() {
                    c.bus.write8(AVJUSTIFY_S + i as u32, *b);
                }
                c.bus.write8(AVJUSTIFY_S + 7, 0);
                // Frame as the entry block leaves it; saved slots get
                // distinctive garbage so a missed restore changes RAM.
                let sp = AVJUSTIFY_FRAME;
                c.g[SP][0] = sp as u64;
                for i in 0..0x30 {
                    c.bus.write32(sp + 4 * i as u32, 0x5a5a_5a00 + i as u32);
                }
                c.bus.write32(sp + 0x40, crate::difftest::RA);
                c.g[RA][0] = crate::difftest::RA as u64;
                if head == 0x003a_f468 {
                    // Mid-scan: index 2 of 7, tab remembered.
                    c.g[S3][0] = 2;
                    c.g[S5][0] = 7;
                    c.g[S4][0] = AVJUSTIFY_S as u64;
                    c.g[S6][0] = 9;
                } else {
                    // Mid-table: third entry; stack names/values forged as
                    // the copy block lays them out (names at +0x0, values
                    // at +0x20).
                    let names = DATA + 0x1700;
                    for i in 0..WORDS.len() {
                        let at = AVJUSTIFY_DICT + i as u32 * 0x10;
                        c.bus.write32(names + 4 * i as u32, at);
                        c.bus.write32(names + 0x20 + 4 * i as u32, 0xdead_beef);
                    }
                    c.g[S2][0] = 2;
                    c.g[S0][0] = names as u64;
                    c.g[S1][0] = 0xffff_ffff;
                    c.g[S4][0] = AVJUSTIFY_S as u64;
                    c.g[S3][0] = 3;
                    c.g[S5][0] = 7;
                    c.g[S6][0] = 9;
                }
                c.pc = head;
            });
            assert!(r.hard.is_empty(), "head {head:#x}: {:#?}", r.hard);
        }
    }

    /// Direct-entry resume for `av_find_token`'s four used-form heads: the
    /// frame as the entry block leaves it, loop registers at mid-match
    /// values (a full "alpha" match from each head), compared head by head.
    #[test]
    fn av_find_token_resume_matches() {
        use crate::difftest::compare;
        for head in [0x003a_f6bcu32, 0x003a_f700, 0x003a_f718, 0x003a_f750] {
            let r = compare(ee::s13::f_003af670__generated, av_find_token, &|c| {
                machine(c, 0x3af6_70, head as u64, 0);
                for ch in [0x20u32, 0x09, 0x0a, 0x0d] {
                    c.bus.write8(AVTOKEN_CTYPE + ch, 8);
                }
                for ch in b'A'..=b'Z' {
                    c.bus.write8(AVTOKEN_CTYPE + ch as u32, 1);
                }
                // One word, "alpha": every head below runs to its match.
                const WORD: &[u8; 5] = b"alpha";
                let at = AVTOKEN_DICT;
                for (j, b) in WORD.iter().enumerate() {
                    c.bus.write8(at + j as u32, *b);
                }
                c.bus.write8(at + WORD.len() as u32, 0);
                c.bus.write32(AVTOKEN_WORDS, at);
                c.bus.write32(AVTOKEN_LENS, 5);
                for (i, b) in b"alpha rest".iter().enumerate() {
                    c.bus.write8(AVTOKEN_S + i as u32, *b);
                }
                c.bus.write8(AVTOKEN_S + 10, 0);
                c.bus.write32(AVTOKEN_PS, AVTOKEN_S);
                c.bus.write32(AVTOKEN_PLEN, 10);
                c.bus.write32(AVTOKEN_OUT, 0xdead_beef);
                // Frame as the entry block leaves it (subject pointer word
                // at +0x0, saved slots garbage).
                let sp = AVTOKEN_FRAME;
                c.g[SP][0] = sp as u64;
                c.bus.write32(sp, AVTOKEN_S);
                for i in 1..0x20 {
                    c.bus.write32(sp + 4 * i as u32, 0x5a5a_5a00 + i as u32);
                }
                c.bus.write32(sp + 0x10, crate::difftest::RA);
                c.g[RA][0] = crate::difftest::RA as u64;
                c.g[S0][0] = AVTOKEN_PLEN as u64;
                c.g[S2][0] = AVTOKEN_PS as u64;
                c.g[S3][0] = AVTOKEN_OUT as u64;
                c.g[S4][0] = AVTOKEN_LENS as u64;
                c.g[S5][0] = AVTOKEN_WORDS as u64;
                c.g[T5][0] = 10;
                c.g[T1][0] = AVTOKEN_LENS as u64;
                c.g[T0][0] = AVTOKEN_WORDS as u64;
                c.g[T4][0] = AVTOKEN_S as u64;
                match head {
                    // Match exit: start 0, five bytes matched, word 0.
                    0x003a_f6bc => {
                        c.g[A3][0] = 0;
                        c.g[A1][0] = 5;
                        c.g[T3][0] = 0;
                        c.g[A0][0] = 0;
                        c.g[A2][0] = 0;
                    }
                    // Next start: fresh scan of the full remainder.
                    0x003a_f700 => {
                        c.g[S1][0] = 1;
                        c.g[T3][0] = 0;
                        c.g[A3][0] = 0;
                        c.g[A1][0] = 0;
                        c.g[A0][0] = 0;
                        c.g[A2][0] = 0;
                    }
                    // Next word: same, positioned one byte in.
                    0x003a_f718 => {
                        c.g[S1][0] = 1;
                        c.g[T3][0] = 0;
                        c.g[A3][0] = 1;
                        c.g[A1][0] = 0;
                        c.g[A0][0] = 0;
                        c.g[A2][0] = 0;
                    }
                    // Whole word matched when the count reaches its length:
                    // three bytes in ("alp"), two to go ("ha").
                    _ => {
                        c.g[S1][0] = 1;
                        c.g[T3][0] = 0;
                        c.g[A3][0] = 0;
                        c.g[A1][0] = 3;
                        c.g[A2][0] = (AVTOKEN_S + 2) as u64;
                        c.g[T2][0] = 5;
                        c.g[V0][0] = 0;
                        c.g[V1][0] = 0;
                    }
                }
                c.pc = head;
            });
            assert!(r.hard.is_empty(), "head {head:#x}: {:#?}", r.hard);
        }
    }
}
