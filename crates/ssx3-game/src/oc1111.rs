//! Staging module oc1111 (wave wave269): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1111`.

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
        0x003d_3658,
        path_calc_node_repeat_count,
        "PATH_CalcNodeRepeatCount",
    ),
    (0x003d_4050, path_post_node_events, "PATH_PostNodeEvents"),
    (0x003d_4900, path_track_get_beat, "PATH_TrackGetBeat"),
    (0x003d_4a50, path_find_track_info, "PATH_FindTrackInfo"),
    (
        0x003d_53b8,
        path_find_playing_track_id,
        "PATH_FindPlayingTrackId",
    ),
];

/// The path data pointer (`DAT_0044f420`, loaded as `gp - 0xbe0`): its word
/// points at the path node table (+0x34, +0x38 and +0x50 are its fields).
const PATH_DATA_PTR: u32 = 0x0044_f420;

/// The high half of the `gp` the generated bodies keep in `s3`/`s4`.
const PATH_GP: u32 = 0x0045_0000;

/// `PATH_TrackGetBeatPos(track, out)` (0x3d32b8): the beat position of a track.
fn track_get_beat_pos() -> Callee {
    Callee {
        addr: 0x003d_32b8,
        f: ssx3_ee::s14::f_003d32b8,
    }
}

/// `PATH_QueueAppend(list, node)` (0x3d0b18).
fn queue_append() -> Callee {
    Callee {
        addr: 0x003d_0b18,
        f: ssx3_ee::s14::f_003d0b18,
    }
}

/// `PATHI_copyevent(event)` (0x3d2068): returns the copy in `v0`.
fn copy_event() -> Callee {
    Callee {
        addr: 0x003d_2068,
        f: ssx3_ee::s14::f_003d2068,
    }
}

/// `PATH_AddEventToMaps(map, event)` (0x3d1f80).
fn add_event_to_maps() -> Callee {
    Callee {
        addr: 0x003d_1f80,
        f: ssx3_ee::s14::f_003d1f80,
    }
}

/// `PATH_GetTrackStream(id)` (0x3d3030): the stream of a track id (`< 0`: none).
fn get_track_stream() -> Callee {
    Callee {
        addr: 0x003d_3030,
        f: ssx3_ee::s14::f_003d3030,
    }
}

/// `PATH_status(id, out)` (0x3d11b8): writes a status word to `out`.
fn path_status() -> Callee {
    Callee {
        addr: 0x003d_11b8,
        f: ssx3_ee::s14::f_003d11b8,
    }
}

/// The beat-position branch of [`path_calc_node_repeat_count`] (the generated
/// block at 0x3d3780): asks the track for its beat, then works out the node's
/// count from the node's mode bits (0x300 of its info word). Leaves the count
/// in `v1`; the caller adds the common tail.
fn repeat_count_beat_path(c: &mut Cpu) -> Flow {
    c.cycles += 18;
    c.g[A1][0] = c.g[S1][0];
    track_get_beat_pos().call(c, 0x003d_36d0)?;
    c.g[A1][0] = Addr(PATH_DATA_PTR).lw(c);
    c.g[A3][0] = c.g[V0][0];
    c.g[V0][0] = Addr(c.g[S0][0] as u32).lh(c);
    c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x50)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
    c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x4)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    // An 8-byte copy of the record at v0 - 8 to the stack (the generated
    // `ldl`/`ldr`/`sdl`/`sdr` run through a0).
    c.ldl(A0, (c.g[V0][0] as u32).wrapping_sub(0x1));
    c.ldr(A0, (c.g[V0][0] as u32).wrapping_sub(0x8));
    let stack = sp(c);
    c.sdl(A0, stack.wrapping_add(0x7));
    c.sdr(A0, stack);
    c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x8)).lw(c);
    c.g[V1][0] = 0x200;
    c.g[A1][0] = c.g[V0][0] & 0x300;
    let not_fraction = c.g[A1][0] != c.g[V1][0];
    c.g[V0][0] = 0x100;
    if not_fraction {
        c.cycles += 2;
        let not_repeat = c.g[A1][0] != c.g[V0][0];
        c.g[V0][0] = c.g[A1][0] ^ 0x300;
        if not_repeat {
            c.cycles += 1;
            c.g[V1][0] = (c.g[V0][0] < 0x1) as u64;
        } else {
            // Mode 0x100: `(beat + 1) % count + 1`, `count` when that wraps to 0.
            c.cycles += 6;
            c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            c.g[V0][0] &= 0xf;
            c.divu(0, 3, 2);
            c.cycles += 4;
            c.g[A0][0] = c.hi[0];
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            if c.g[V1][0] == 0 {
                c.g[V1][0] = c.g[V0][0];
            }
        }
    } else {
        // Mode 0x200: a fraction of the track, `(w * h) - (puVar4[6] / puVar4[1]) + 1`.
        c.cycles += 9;
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
        c.g[V1][0] = sx((c.g[A2][0] as u32) >> 24);
        c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x18)).lw(c);
        c.g[V0][0] = sx((c.g[A2][0] as u32) >> 20);
        c.g[V0][0] &= 0xf;
        c.g[V1][0] &= 0xf;
        c.divu(0, 5, 4);
        c.cycles += 6;
        c.g[A1][0] = c.lo[0];
        c.mult(0, 3, 2, 3);
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A1][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
    }
    Ok(())
}

/// `PATH_CalcNodeRepeatCount(node, track, count)` (0x3d3658): how many times a
/// node repeats. The node's word at +8 (bits 20 and up) picks a record of the
/// path node table; its mode bits (0x300) decide the count:
/// - no track (or the track's word at +0x30 is 0): 1 when the mode is 0, else 0;
/// - 0x100: `(beat + 1) % n + 1`, with `n` (the count's low nibble) when that is 0;
/// - 0x200: a fraction of the track, `w * h - count[6] / count[1] + 1`;
/// - 0x300: 1; anything else: 0.
pub fn path_calc_node_repeat_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x60);
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S3][0] = PATH_GP as u64;
        c.g[V0][0] = Addr(PATH_DATA_PTR).lw(c);
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x38)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) >> 20);
        c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x34)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lh(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[S2][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));

        let track = c.g[A3][0];
        let has_track = if track == 0 {
            false
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr((track as u32).wrapping_add(0x30)).lw(c);
            c.g[A0][0] = track;
            c.g[V0][0] != 0
        };
        if has_track {
            repeat_count_beat_path(c)?;
            c.cycles += 1;
            c.g[V0][0] = c.g[V1][0];
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x8)).lw(c);
            c.g[V0][0] &= 0x300;
            c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
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

/// The memset-like clear the event code runs on its two 16-byte buffers
/// (`call_indirect 0x416210(buf, 0, 16)`), returning to `ret`.
fn clear_16(c: &mut Cpu, buf: u64, ret: u32) -> Flow {
    c.g[A0][0] = buf;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x10;
    call_ptr(c, 0x0041_6210, ret)
}

/// The event block of [`path_post_node_events`] (0x3d4134..0x3d4180): clears
/// two buffers, and, unless the track's first word is negative, asks the track
/// for its beat and queues a node; then copies the event, adds it to the maps.
fn post_node_events_body(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 18;
    c.g[V1][0] = 0x1100_0000;
    c.g[V0][0] = c.g[V0][0] << (c.g[A0][0] & 63);
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[V0][0] |= c.g[V1][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0;
    c.g[S1][0] = c.g[V0][0] << 32;
    c.g[S1][0] = ((c.g[S1][0] as i64) >> 32) as u64;
    let buf = c.g[S0][0];
    clear_16(c, buf, 0x003d_40e0)?;
    let stack = c.g[SP][0];
    clear_16(c, stack, 0x003d_40f0)?;

    c.g[V1][0] = Addr(PATH_DATA_PTR).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    let negative = (c.g[V0][0] as i64) < 0;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    if !negative {
        c.cycles += 13;
        c.g[A1][0] = sx(sp(c).wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x58)).lw(c);
        track_get_beat_pos().call(c, 0x003d_4110)?;
        c.g[V1][0] = Addr(sp(c).wrapping_add(0x38)).lw(c);
        c.g[V0][0] = 0x2;
        let kind = c.g[V0][0];
        Addr(sp(c).wrapping_add(0x9)).sb(c, kind);
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xa));
        let link = c.g[S1][0];
        Addr(sp(c)).sw(c, link);
        let slot = c.g[V1][0];
        Addr(sp(c).wrapping_add(0xc)).sw(c, slot);
        c.g[A1][0] = c.g[SP][0];
        queue_append().call(c, 0x003d_4134)?;
    }

    // The 19-cycle tail, reached on both paths.
    c.cycles += 19;
    c.g[V0][0] = 0x4;
    let event_list = c.g[S1][0];
    Addr(sp(c)).sw(c, event_list);
    let kind = c.g[V0][0];
    Addr(sp(c).wrapping_add(0x9)).sb(c, kind);
    c.g[V1][0] = u64::MAX;
    c.g[A1][0] = c.g[SP][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
    let node = c.g[S3][0];
    Addr(sp(c).wrapping_add(0xc)).sh(c, node);
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 21);
    let neg = c.g[V1][0];
    Addr(sp(c).wrapping_add(0xe)).sb(c, neg);
    c.g[V0][0] &= 0x3f;
    let key = c.g[V0][0];
    Addr(sp(c).wrapping_add(0x8)).sb(c, key);
    queue_append().call(c, 0x003d_4168)?;
    c.g[A0][0] = c.g[S0][0];
    copy_event().call(c, 0x003d_4170)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S0][0];
    add_event_to_maps().call(c, 0x003d_4180)?;
    let _ = frame;
    Ok(())
}

/// `PATH_PostNodeEvents(node)` (0x3d4050): posts a node's events to the track's
/// queue. Returns the event copy in `v0` (0 when the node has none).
pub fn path_post_node_events(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S4, 0x50);
        c.g[S4][0] = PATH_GP as u64;
        frame.save(c, S0, 0x90);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S1, 0x80);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x40);
        c.g[V1][0] = sx((c.g[S3][0] as u32) << 1);
        c.g[A2][0] = Addr(PATH_DATA_PTR).lw(c);
        c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x38)).lw(c);
        c.g[A1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x34)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lh(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[S2][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = 0;
        if c.g[S2][0] != 0 {
            c.cycles += 7;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x2)).lhu(c);
            c.g[A0][0] = c.g[V0][0] & 0x1f;
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x58)).lw(c);
            let has_events = c.g[V0][0] != 0;
            c.g[V0][0] = 0x1;
            if has_events {
                post_node_events_body(c, frame)?;
            }
        }

        c.cycles += 9;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x80);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `PATH_TrackGetBeat(track, out)` (0x3d4900): the track's beat position. A
/// negative position stores 0 to `*out` (when `out` is not null) and returns -1.
pub fn path_track_get_beat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        c.g[A1][0] = c.g[SP][0];
        track_get_beat_pos().call(c, 0x003d_4918)?;
        if (c.g[V0][0] as i64) < 0 {
            c.cycles += 4;
            Addr(sp(c).wrapping_add(0x4)).sw(c, 0);
            c.g[V0][0] = u64::MAX;
            let minus_one = c.g[V0][0];
            Addr(sp(c).wrapping_add(0x10)).sw(c, minus_one);
        }
        c.cycles += 2;
        let store = c.g[S0][0] != 0;
        c.g[V0][0] = Addr(sp(c).wrapping_add(0x4)).lw(c);
        if store {
            c.cycles += 1;
            let value = c.g[V0][0];
            Addr(c.g[S0][0] as u32).sw(c, value);
        }
        c.cycles += 5;
        c.g[V0][0] = Addr(sp(c).wrapping_add(0x10)).lw(c);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `PATH_FindTrackInfo(key)` (0x3d4a50): searches the four 0x928-byte track
/// records (from 0x517644) for the first one whose mask word (at -4) matches
/// `key`'s high nibbles, then returns the entry of the table at +0x8f0 for the
/// lowest set bit of `key` (bit 0..23). Returns 0 when nothing matches.
pub fn path_find_track_info(c: &mut Cpu) {
    c.cycles += 12;
    c.g[A1][0] = sx(0x0051_0000u32.wrapping_add(0x7610));
    c.g[A2][0] = 0x00ff_0000;
    c.g[V0][0] = 0xffff_ffff_f000_0000;
    c.g[V1][0] = 0x0f00_0000;
    c.g[T2][0] = c.g[A0][0] & c.g[V0][0];
    c.g[T1][0] = c.g[A0][0] & c.g[V1][0];
    c.g[A2][0] |= 0xffff;
    c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x924));
    c.g[A0][0] &= c.g[A2][0];
    c.g[A3][0] = 0;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x34));
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8f0)).lw(c);
        let no_table = c.g[V0][0] == 0;
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_sub(0x4)).lw(c);
        // `skip_count`: the record was rejected before any cycle of the
        // "next record" step, so the counter moves without its extra cycle.
        let mut skip_count = false;
        'record: {
            if no_table {
                break 'record;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
            let no_base = c.g[V0][0] == 0;
            c.g[V0][0] = c.g[V1][0] & c.g[T2][0];
            if no_base {
                break 'record;
            }
            c.cycles += 2;
            let no_high = c.g[V0][0] == 0;
            c.g[V0][0] = c.g[V1][0] & c.g[T1][0];
            if no_high {
                break 'record;
            }
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                skip_count = true;
                break 'record;
            }
            // The low-bit search: `key >> i` for i = 0.., up to 24 bits.
            c.cycles += 4;
            c.g[V1][0] = 0;
            c.g[A2][0] = c.g[T0][0];
            c.g[V0][0] = sx((c.g[A0][0] as u32) >> ((c.g[V1][0] as u32) & 31));
            loop {
                c.cycles += 3;
                c.g[V0][0] &= 0x1;
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.cycles += 3;
                    let more = c.g[V1][0] < 0x18;
                    c.g[V0][0] = more as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[A0][0] as u32) >> ((c.g[V1][0] as u32) & 31));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_4ab8);
                        }
                        continue;
                    }
                    break 'record;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                    ret(c);
                    return;
                }
            }
        }
        if !skip_count {
            c.cycles += 1;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
        }
        c.cycles += 4;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x928));
        c.g[V0][0] = (c.g[A3][0] < 0x4) as u64;
        let again = c.g[V0][0] != 0;
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x928));
        if again {
            if event_due(c) {
                deliver_event_here(c, 0x003d_4a80);
            }
            continue;
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }
}

/// `PATH_FindPlayingTrackId(track, bit)` (0x3d53b8): looks for a playing track
/// among the stream ids `(0x10000000 << k) | (0x1000000 << track) + bit` for
/// k = 1, 0. Returns the first id whose status word is non-negative, else 0.
pub fn path_find_playing_track_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x0100_0000;
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = 1;
        frame.save(c, S3, 0x50);
        c.g[S2][0] = sx((c.g[S1][0] as u32) << ((c.g[A1][0] as u32) & 31));
        frame.save(c, S4, 0x40);
        c.g[S3][0] = sx((c.g[V0][0] as u32) << ((c.g[A0][0] as u32) & 31));
        frame.save(c, S0, 0x80);
        c.g[S4][0] = 0x1000_0000;
        frame.save_ra(c, 0x30);
        c.g[V0][0] = sx((c.g[S4][0] as u32) << ((c.g[S1][0] as u32) & 31));
        loop {
            c.cycles += 6;
            c.g[V0][0] |= c.g[S3][0];
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            c.g[A0][0] = c.g[S0][0];
            get_track_stream().call(c, 0x003d_5400)?;
            let negative = (c.g[V0][0] as i64) < 0;
            c.g[A0][0] = c.g[S0][0];
            if !negative {
                c.cycles += 5;
                c.g[A1][0] = c.g[SP][0];
                path_status().call(c, 0x003d_5410)?;
                c.g[V1][0] = Addr(sp(c)).lw(c);
                c.g[V0][0] = c.g[S0][0];
                if (c.g[V1][0] as i64) >= 0 {
                    break;
                }
            }
            c.cycles += 3;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[S4][0] as u32) << ((c.g[S1][0] as u32) & 31));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003d_53f0);
                }
                continue;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
            break;
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S0, 0x80);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, machine_rt};
    use crate::difftest::{check, fill, Rng, DATA};

    /// The path node table the lookups read through [`PATH_DATA_PTR`].
    pub(crate) const PATH_TEST_TBL: u32 = DATA + 0x2000;

    /// Lays out the path table: its fields (+0x34, +0x38, +0x50), the 32
    /// record words at +0x58, and the head word (`*tbl`, sign picks a branch).
    pub(crate) fn path_test_tables(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let tbl = PATH_TEST_TBL;
        c.bus.write32(PATH_DATA_PTR, tbl);
        let base1 = if seed % 5 == 0 { 0 } else { DATA + 0x3000 };
        c.bus.write32(tbl + 0x34, base1);
        c.bus.write32(tbl + 0x38, DATA + 0x4000);
        c.bus.write32(tbl + 0x50, DATA + 0x5000);
        for i in 0..32 {
            let word = if seed % 4 == 0 { 0 } else { r.next_u32() };
            c.bus.write32(tbl + 0x58 + 4 * i, word);
        }
        let head = r.next_u32() & 0x1f;
        c.bus.write32(
            tbl,
            if seed % 2 == 0 {
                0x8000_0000 | head
            } else {
                head
            },
        );
    }

    /// Argument-side fixture of the node tests: the node at `DATA+0x6000`
    /// with its selector word, the track at `DATA+0x7000`, the count words at
    /// `DATA+0x8000`.
    pub(crate) fn path_test_node(c: &mut Cpu, r: &mut Rng, seed: u32) {
        fill(c, r, DATA + 0x7000, 16);
        let idx = seed % 16;
        c.bus
            .write32(DATA + 0x6008, (idx << 20) | (r.next_u32() & 0xf_ffff));
        c.bus.write32(DATA + 0x6004, r.next_u32());
        c.bus.write16(DATA + 0x4000 + 2 * idx, (seed % 6) as u16);
        let base1 = if seed % 5 == 0 { 0 } else { DATA + 0x3000 };
        let info = base1.wrapping_add(4 * (seed % 6));
        c.bus.write32(info, r.next_u32());
        c.bus
            .write16(info.wrapping_add(2), (r.next_u32() & 0x1f) as u16);
        c.bus.write32(
            info.wrapping_add(8),
            ((seed % 4) << 8) | (r.next_u32() & !0x300),
        );
        c.bus
            .write32(DATA + 0x7030, if seed % 3 == 2 { 1 } else { 0 });
        c.bus.write32(DATA + 0x8000, 1 + (seed % 15));
        c.bus.write32(DATA + 0x8004, 1 + (seed % 7));
        c.bus.write32(DATA + 0x8018, r.next_u32());
        c.g[A0][0] = sx(DATA + 0x6000);
        c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x7000) };
        c.g[A2][0] = sx(DATA + 0x8000);
    }

    #[test]
    fn path_calc_node_repeat_count_matches() {
        check(
            ssx3_ee::s14::f_003d3658__generated,
            path_calc_node_repeat_count,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x3658, 6);
                path_test_tables(c, &mut r, seed);
                path_test_node(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn path_post_node_events_matches() {
        check(
            ssx3_ee::s14::f_003d4050__generated,
            path_post_node_events,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x4050, 8);
                path_test_tables(c, &mut r, seed);
                let k = seed % 8;
                c.g[A0][0] = sx(k);
                c.bus.write16(DATA + 0x4000 + 2 * k, (seed % 6) as u16);
                let base1 = if seed % 5 == 0 { 0 } else { DATA + 0x3000 };
                let info = base1.wrapping_add(4 * (seed % 6));
                c.bus.write32(info, r.next_u32());
                c.bus
                    .write16(info.wrapping_add(2), (r.next_u32() & 0x1f) as u16);
                c.bus.write32(DATA + 0x7030, 1);
                c.bus.write32(DATA + 0x7000, r.next_u32());
            },
        );
    }

    #[test]
    fn path_track_get_beat_matches() {
        check(
            ssx3_ee::s14::f_003d4900__generated,
            path_track_get_beat,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x4900, 2);
                fill(c, &mut r, DATA + 0x7000, 16);
                c.g[A0][0] = sx(DATA + 0x7000);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x2000) };
            },
        );
    }

    #[test]
    fn path_find_track_info_matches() {
        check(
            ssx3_ee::s14::f_003d4a50__generated,
            path_find_track_info,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x4a50, 0);
                let key = r.next_u32();
                c.g[A0][0] = sx(key);
                let ptr = DATA + 0x1000;
                for i in 0..4u32 {
                    let base = 0x0051_7644 + i * 0x928;
                    let mask = if (seed + i) % 4 == 0 {
                        r.next_u32()
                    } else {
                        key ^ (r.next_u32() & 0x0f0f_0f0f)
                    };
                    c.bus.write32(base - 4, mask);
                    c.bus.write32(base, if seed % 7 == 0 { 0 } else { 1 });
                    c.bus
                        .write32(base + 0x8f0, if (seed + i) % 9 == 0 { 0 } else { ptr });
                }
                for k in 0..24 {
                    c.bus.write32(ptr + 4 * k, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn path_find_playing_track_id_matches() {
        check(
            ssx3_ee::s14::f_003d53b8__generated,
            path_find_playing_track_id,
            300,
            &|c, seed| {
                machine_rt(c, seed, 0x53b8, 4);
                c.g[A0][0] = (seed % 32) as u64;
                c.g[A1][0] = ((seed / 3) % 32) as u64;
            },
        );
    }
}
