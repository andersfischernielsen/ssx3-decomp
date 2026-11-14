//! Staging module oc369 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc369`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x003d_41a8,
    path_track_submit_queue,
    "PATH_TrackSubmitQueue",
)];

// ---------------------------------------------------------------------------
// PATH_TrackSubmitQueue (0x3d41a8)
// ---------------------------------------------------------------------------

/// `PathSystem* g_pathSystem` (`*0x44f420`): `+0` the index of the master
/// (beat) track or -1, `+0x58` the track table, `+0x34` the node array,
/// `+0x38` the node index table (`short`s), `+0x50` the sample table
/// (8-byte entries: `{ s5 value, length }`).
pub(crate) const PTS_PATH_SYS: u32 = 0x0044_f420;
/// `int g_pathTime` (`0x44f438`), the music clock.
pub(crate) const PTS_TIME: u32 = 0x0044_f438;
/// `int g_pathLastSyncQueue` (`0x44f79c`): when a synced track last queued.
pub(crate) const PTS_LAST_SYNC: u32 = 0x0044_f79c;
/// `double` volume scale at `0x495b98`.
const PTS_VOLUME_SCALE: u32 = 0x0049_5b98;

/// Track fields.
pub(crate) const PTS_VOLUME: u32 = 0x04;
pub(crate) const PTS_FLAGS: u32 = 0x20;
pub(crate) const PTS_LOOPING: u32 = 0x2f;
pub(crate) const PTS_NEXT_NODE: u32 = 0x30;
pub(crate) const PTS_GAIN: u32 = 0x34;
pub(crate) const PTS_NODE: u32 = 0x36;
pub(crate) const PTS_LENGTH: u32 = 0x40;
pub(crate) const PTS_BEAT_LENGTH: u32 = 0x44;
pub(crate) const PTS_START: u32 = 0x48;
pub(crate) const PTS_IDLE_SINCE: u32 = 0x4c;
pub(crate) const PTS_PLAT_TRACK: u32 = 0x54;
pub(crate) const PTS_HANDLE: u32 = 0x58;
pub(crate) const PTS_QUEUE_STATE: u32 = 0x5c;
pub(crate) const PTS_HISTORY: u32 = 0xfc;
pub(crate) const PTS_RESYNC: u32 = 0x10c;

/// `PATH_PlatGetQueuedEntryValue(track, i)` (through the table).
pub(crate) const PTS_GET_QUEUED: u32 = 0x002a_eaa8;
/// `PATH_PlatGetTrackVolume(track)`.
pub(crate) const PTS_GET_VOLUME: u32 = 0x002a_f428;
/// `PATH_PlatQueue(track, ...)`.
pub(crate) const PTS_PLAT_QUEUE: u32 = 0x002a_e7a8;
const PTS_FLOATSIDF: u32 = 0x0041_2550;
const PTS_MULDF3: u32 = 0x0041_1fd8;
const PTS_FIXDFSI: u32 = 0x0041_2608;
const PTS_CALC_REPEAT: Callee = Callee {
    addr: 0x003d_3658,
    f: ee::s14::PATH_CalcNodeRepeatCount,
};
const PTS_GET_BEAT_POS: Callee = Callee {
    addr: 0x003d_32b8,
    f: ee::s14::PATH_TrackGetBeatPos,
};

#[inline(always)]
fn pts_srl(v: u64, n: u32) -> u64 {
    sx((v as u32) >> n)
}
#[inline(always)]
fn pts_sra(v: u64, n: u32) -> u64 {
    sx(((v as u32) as i32 >> n) as u32)
}
#[inline(always)]
fn pts_sll(v: u64, n: u32) -> u64 {
    sx((v as u32) << n)
}
#[inline(always)]
fn pts_addu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}
#[inline(always)]
fn pts_at(c: &Cpu, r: usize, off: u32) -> u32 {
    r32(c, r).wrapping_add(off)
}

/// The argument setup shared by the three `PATH_PlatQueue` calls (`t2`
/// already holds the track flags): `(plat track, s7 mode, &queue state,
/// s5 sample value, length, (s8)volume, repeat count, 0)`.
fn pts_queue_args(c: &mut Cpu, load_length: bool) {
    c.g[T1][0] = pts_sll(c.g[S2][0], 24);
    c.g[A1][0] = c.g[S7][0];
    c.g[T2][0] = pts_sll(c.g[T2][0], 23);
    c.g[A0][0] = c.lw(pts_at(c, S1, PTS_PLAT_TRACK));
    if load_length {
        c.g[T0][0] = c.lw(pts_at(c, S1, PTS_LENGTH));
    }
    c.g[A3][0] = c.g[S5][0];
    c.g[T1][0] = pts_sra(c.g[T1][0], 24);
    c.g[T2][0] = pts_sra(c.g[T2][0], 27);
    c.g[A2][0] = pts_addu(c.g[S1][0], PTS_QUEUE_STATE as u64);
    c.g[T3][0] = 0;
}

/// How the synced part of [`path_track_submit_queue`] ended.
enum PtsSynced {
    /// The next node does not sync: queue at once (`t2` = flags).
    Unsynced,
    /// Return `v0` (-9999: wait) now.
    Return,
    /// Too early to queue (idle time cleared).
    Wait,
    /// Queued on the voice.
    Queued,
}

/// `PATH_TrackGetBeatPos` failed: no beat (`pos.beat = 0`, `pos.next = -1`).
fn pts_no_beat(c: &mut Cpu) {
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.sw(sp(c).wrapping_add(0x24), 0);
    c.g[V0][0] |= 0xffff;
    c.sw(pts_at(c, S0, 0x10), c.g[V0][0]);
}

/// A looping track that follows the master track `s4` (`a0` = 0x100): if
/// the next node syncs (mode 0x100/0x200/0x300 in its `+8`), split the
/// lengths by the node's beats and bars, line the start (`+0x48`) up with
/// the master's beat position, and queue once half a length has passed
/// since the last synced queue.
fn pts_synced(c: &mut Cpu) -> Result<PtsSynced, Unwound> {
    c.cycles += 5;
    c.g[V0][0] = c.lw(pts_at(c, S1, PTS_NEXT_NODE));
    c.g[V1][0] = c.lw(pts_at(c, V0, 0x8));
    c.g[V1][0] &= 0x300;
    let mut synced = c.g[V1][0] == c.g[A0][0];
    c.g[V0][0] = 0x200;
    if !synced {
        c.cycles += 2;
        synced = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x300;
        if !synced {
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[T2][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                return Ok(PtsSynced::Unsynced);
            }
        }
    }

    // Beat length /= beats (node +4 bits 20..23); length /= beats * bars.
    c.cycles += 5;
    c.g[V1][0] = c.lw(pts_at(c, S3, 0x4));
    c.g[V0][0] = 0xf00000;
    c.g[V0][0] &= c.g[V1][0];
    let no_beats = c.g[V0][0] == 0;
    c.g[V0][0] = pts_srl(c.g[V1][0], 20);
    if !no_beats {
        c.cycles += 4;
        c.g[V1][0] = c.lw(pts_at(c, S1, PTS_BEAT_LENGTH));
        c.g[V0][0] &= 0xf;
        c.cycles += 4;
        c.divu(0, V1, V0);
        c.g[V1][0] = c.lo[0];
        c.sw(pts_at(c, S1, PTS_BEAT_LENGTH), c.g[V1][0]);
        c.g[V1][0] = c.lw(pts_at(c, S3, 0x4));
    }
    c.cycles += 4;
    c.g[V0][0] = 0xf000000;
    c.g[V0][0] &= c.g[V1][0];
    let no_bars = c.g[V0][0] == 0;
    c.g[A0][0] = pts_srl(c.g[V1][0], 20);
    if !no_bars {
        c.cycles += 8;
        c.g[V0][0] = pts_srl(c.g[V1][0], 24) & 0xf;
        c.g[A0][0] &= 0xf;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = c.lw(pts_at(c, S1, PTS_LENGTH));
        c.divu(0, V1, V0);
        c.cycles += 2;
        c.g[V1][0] = c.lo[0];
        c.sw(pts_at(c, S1, PTS_LENGTH), c.g[V1][0]);
    }

    'queue_check: {
        'set_start: {
            'repeated: {
                c.cycles += 2;
                if c.g[FP][0] != 0 {
                    c.g[V0][0] = c.lw(pts_at(c, S1, PTS_NEXT_NODE));
                    break 'repeated;
                }
                'fresh: {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(pts_at(c, S1, PTS_RESYNC));
                    c.g[V1][0] = 0x450000;
                    if c.g[A0][0] == 0 {
                        break 'fresh;
                    }
                    // A pending resync: wait until it is due.
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(PTS_TIME);
                    c.g[V0][0] = pts_addu(c.g[V0][0], 5);
                    c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
                    c.g[FP][0] = c.g[V1][0];
                    if c.g[V0][0] == 0 {
                        c.cycles += 2;
                        c.g[V0][0] = 0xffff_ffff_ffff_d8f1;
                        return Ok(PtsSynced::Return);
                    }
                    c.cycles += 7;
                    c.g[S0][0] = pts_addu(c.g[SP][0], 0x20);
                    c.sw(pts_at(c, S1, PTS_RESYNC), 0);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[S0][0];
                    PTS_GET_BEAT_POS.call(c, 0x003d_4510)?;
                    let found = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x24));
                    if !found {
                        c.cycles += 5;
                        pts_no_beat(c);
                        c.g[V0][0] = c.lw(sp(c).wrapping_add(0x24));
                    }
                    // Next beat, or the one after if past half a beat.
                    c.cycles += 11;
                    c.g[A1][0] = c.lw(sp(c).wrapping_add(0x30));
                    c.g[V1][0] = pts_srl(c.g[V0][0], 31);
                    c.g[A0][0] = c.lw(r32(c, FP).wrapping_sub(0xbc8));
                    c.g[V1][0] = pts_addu(c.g[V0][0], c.g[V1][0]);
                    c.g[V1][0] = pts_sra(c.g[V1][0], 1);
                    c.g[V0][0] = pts_addu(c.g[A1][0], c.g[V0][0]);
                    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    if c.g[V1][0] == 0 {
                        c.g[A1][0] = c.g[V0][0];
                    }
                    c.g[A0][0] = pts_addu(c.g[A1][0], c.g[A0][0]);
                    c.sw(pts_at(c, S1, PTS_START), c.g[A0][0]);
                    break 'queue_check;
                }
                c.cycles += 6;
                c.g[S0][0] = pts_addu(c.g[SP][0], 0x20);
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S0][0];
                PTS_GET_BEAT_POS.call(c, 0x003d_4568)?;
                if (c.g[V0][0] as i64) < 0 {
                    c.cycles += 5;
                    pts_no_beat(c);
                }
                c.g[V0][0] = c.lw(pts_at(c, S1, PTS_LENGTH));
                c.cycles += 7;
                c.g[A1][0] = c.lw(sp(c).wrapping_add(0x30));
                c.g[V1][0] = pts_srl(c.g[V0][0], 31);
                c.g[V0][0] = pts_addu(c.g[V0][0], c.g[V1][0]);
                c.g[V0][0] = pts_sra(c.g[V0][0], 1);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let late = c.g[V0][0] != 0;
                c.g[A0][0] = c.lw(sp(c).wrapping_add(0x24));
                if !late {
                    c.cycles += 2;
                    c.g[A1][0] = pts_addu(c.g[A1][0], c.g[A0][0]);
                }
                break 'set_start;
            }
            // Repeated node (v0 = next node): by its sync mode.
            'mode_200: {
                'free: {
                    c.cycles += 5;
                    c.g[A0][0] = 0x100;
                    c.g[V1][0] = c.lw(pts_at(c, V0, 0x8));
                    c.g[V1][0] &= 0x300;
                    let m100 = c.g[V1][0] == c.g[A0][0];
                    c.g[V0][0] = 0x300;
                    if m100 {
                        break 'free;
                    }
                    c.cycles += 2;
                    let not300 = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x200;
                    if not300 {
                        break 'mode_200;
                    }
                }
                // Resync later: at the repeat's end, rounded down to 10.
                c.cycles += 11;
                c.g[V0][0] = 0x450000;
                c.g[A1][0] = c.lw(sp(c).wrapping_add(0x10));
                c.g[V1][0] = c.lw(PTS_TIME);
                c.g[A0][0] = 0xa;
                c.sw(pts_at(c, S1, PTS_START), 0);
                c.g[V0][0] = 0xffff_ffff_ffff_d8f1;
                c.g[V1][0] = pts_addu(c.g[A1][0], c.g[V1][0]);
                c.divu(0, V1, A0);
                c.sw(pts_at(c, S1, PTS_RESYNC), c.g[V1][0]);
                c.cycles += 4;
                c.g[A1][0] = c.hi[0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                c.sw(pts_at(c, S1, PTS_RESYNC), c.g[V1][0]);
                return Ok(PtsSynced::Return);
            }
            c.cycles += 2;
            let not200 = c.g[V1][0] != c.g[V0][0];
            c.g[FP][0] = 0x450000;
            if not200 {
                break 'queue_check;
            }
            c.cycles += 2;
            let ahead = (c.g[S6][0] as i64) >= 0;
            c.g[A1][0] = c.lw(sp(c).wrapping_add(0x10));
            if !ahead {
                c.cycles += 5;
                c.g[V0][0] = c.lw(sp(c).wrapping_add(0x4));
                c.g[S6][0] = sx(0u32.wrapping_sub(c.g[S6][0] as u32));
                c.g[V1][0] = c.lw(sp(c).wrapping_add(0x10));
                c.mult(0, V0, S6, V0);
                c.g[A1][0] = pts_addu(c.g[V1][0], c.g[V0][0]);
            }
        }
        // start = now + a1
        c.cycles += 5;
        c.g[V1][0] = 0x450000;
        c.g[V0][0] = c.lw(PTS_TIME);
        c.g[FP][0] = c.g[V1][0];
        c.g[V0][0] = pts_addu(c.g[V0][0], c.g[A1][0]);
        c.sw(pts_at(c, S1, PTS_START), c.g[V0][0]);
    }

    // Queue once half a length has passed since the last synced queue.
    c.cycles += 9;
    c.g[S0][0] = 0x450000;
    c.g[V0][0] = c.lw(r32(c, FP).wrapping_sub(0xbc8));
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_sub(0x864));
    c.g[T0][0] = c.lw(pts_at(c, S1, PTS_LENGTH));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    c.g[V1][0] = pts_srl(c.g[T0][0], 1);
    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
    if c.g[V0][0] != 0 {
        c.sw(pts_at(c, S1, PTS_IDLE_SINCE), 0);
        return Ok(PtsSynced::Wait);
    }
    c.cycles += 17;
    c.g[T2][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
    c.g[V0][0] = 1;
    pts_queue_args(c, false);
    c.sw(sp(c).wrapping_add(0x44), c.g[V0][0]);
    call_ptr(c, PTS_PLAT_QUEUE, 0x003d_4690)?;
    c.g[V1][0] = c.lw(r32(c, FP).wrapping_sub(0xbc8));
    c.sw(pts_at(c, S1, PTS_HANDLE), c.g[V0][0]);
    c.sw(r32(c, S0).wrapping_sub(0x864), c.g[V1][0]);
    Ok(PtsSynced::Queued)
}

/// `int PATH_TrackSubmitQueue(PathTrack* track)` (0x3d41a8): queues the
/// track's current node on its platform voice. A track with no node is
/// reset (`-1` handle, `0` returned). Otherwise it sums the voice's queued
/// entries, works out the repeat count of the node (looping tracks), the
/// volume (`track->volume * scale * track->gain`, or the voice's own when
/// negative), the node's length and sample, and either queues at once or,
/// for a looping track synced to the master track, lines the start up with
/// the master's beat (returning -9999 when it must wait). A queued entry is
/// recorded in the track's 16-entry history ring. Returns the voice handle.
pub fn path_track_submit_queue(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xf0);
        c.g[V0][0] = 0x450000;
        frame.save(c, S1, 0xd0);
        frame.save(c, S4, 0xa0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S6, 0x80);
        c.g[S4][0] = 0;
        frame.save(c, S7, 0x70);
        c.g[S6][0] = 0;
        frame.save(c, FP, 0x60);
        c.g[S7][0] = 0;
        frame.save(c, S0, 0xe0);
        c.g[FP][0] = 0;
        frame.save(c, S2, 0xc0);
        frame.save(c, S3, 0xb0);
        frame.save(c, S5, 0x90);
        frame.save_ra(c, 0x50);
        // The master track (if any).
        c.g[V1][0] = c.lw(PTS_PATH_SYS);
        c.g[V0][0] = c.lw(r32(c, V1));
        c.sw(sp(c).wrapping_add(0x44), 0);
        if (c.g[V0][0] as i64) >= 0 {
            c.cycles += 3;
            c.g[V0][0] = pts_sll(c.g[V0][0], 2);
            c.g[V0][0] = pts_addu(c.g[V1][0], c.g[V0][0]);
            c.g[S4][0] = c.lw(pts_at(c, V0, 0x58));
        }

        'ret: {
            'tail: {
                c.cycles += 3;
                c.g[V0][0] = c.lh(pts_at(c, S1, PTS_NODE));
                c.g[S2][0] = 0;
                if (c.g[V0][0] as i64) < 0 {
                    // No node: reset the track.
                    c.cycles += 10;
                    c.g[V0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                    c.g[V1][0] = 0xffff_ffff_ffff_fe0f;
                    c.g[A0][0] = u64::MAX;
                    c.sw(pts_at(c, S1, PTS_NEXT_NODE), 0);
                    c.g[V0][0] &= c.g[V1][0];
                    c.sh(pts_at(c, S1, PTS_NODE), c.g[A0][0]);
                    c.sw(pts_at(c, S1, PTS_FLAGS), c.g[V0][0]);
                    c.g[V0][0] = 0;
                    c.sw(pts_at(c, S1, PTS_HANDLE), c.g[A0][0]);
                    break 'ret;
                }

                // Sum the voice's queued entries.
                c.cycles += 5;
                c.g[V0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                c.g[V0][0] = pts_srl(c.g[V0][0], 14) & 0x1f;
                c.g[S0][0] = 0;
                if c.g[V0][0] != 0 {
                    loop {
                        c.cycles += 12;
                        c.g[A0][0] = c.lw(pts_at(c, S1, PTS_PLAT_TRACK));
                        c.g[A1][0] = c.g[S0][0];
                        c.g[S0][0] = pts_addu(c.g[S0][0], 1);
                        call_ptr(c, PTS_GET_QUEUED, 0x003d_4238)?;
                        c.g[V1][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                        c.g[A0][0] = ((c.g[V0][0] as i64) < 0) as u64;
                        c.g[V0][0] = pts_addu(c.g[S2][0], c.g[V0][0]);
                        c.g[V1][0] = pts_srl(c.g[V1][0], 14) & 0x1f;
                        c.g[V1][0] = (c.g[S0][0] < c.g[V1][0]) as u64;
                        if c.g[A0][0] == 0 {
                            c.g[S2][0] = c.g[V0][0];
                        }
                        if c.g[V1][0] == 0 {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_4228);
                        }
                    }
                }

                // The current node (s3) and its repeat count.
                'reload_done: {
                    'reload: {
                        'store: {
                            c.cycles += 12;
                            c.g[V1][0] = 0x450000;
                            c.g[A1][0] = c.lbu(pts_at(c, S1, PTS_LOOPING));
                            c.g[V0][0] = c.lw(PTS_PATH_SYS);
                            c.g[V1][0] = c.lh(pts_at(c, S1, PTS_NODE));
                            c.g[A0][0] = c.lw(pts_at(c, V0, 0x38));
                            c.g[V1][0] = pts_sll(c.g[V1][0], 1);
                            c.g[A2][0] = c.lw(pts_at(c, V0, 0x34));
                            c.g[V1][0] = pts_addu(c.g[V1][0], c.g[A0][0]);
                            c.g[V0][0] = c.lh(r32(c, V1));
                            c.g[V0][0] = pts_sll(c.g[V0][0], 2);
                            c.g[S3][0] = pts_addu(c.g[A2][0], c.g[V0][0]);
                            if c.g[A1][0] == 0 {
                                // One-shot: nothing queued means idle since now.
                                c.cycles += 2;
                                if (c.g[S2][0] as i64) > 0 {
                                    c.g[V0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw(pts_at(c, S1, PTS_IDLE_SINCE));
                                    c.g[S7][0] = u64::MAX;
                                    if c.g[V0][0] == 0 {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x450000;
                                        c.g[V1][0] = c.lw(PTS_TIME);
                                        c.sw(pts_at(c, S1, PTS_IDLE_SINCE), c.g[V1][0]);
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                                }
                                c.cycles += 2;
                                c.g[V0][0] |= 0x1f0;
                                break 'store;
                            }
                            c.cycles += 5;
                            c.g[V0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                            c.g[V0][0] = pts_sra(pts_sll(c.g[V0][0], 23), 27);
                            if (c.g[V0][0] as i64) > 0 {
                                c.g[V0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                                break 'reload_done;
                            }
                            c.cycles += 17;
                            c.g[A0][0] = c.g[S3][0];
                            c.g[A1][0] = c.g[S4][0];
                            c.g[A2][0] = c.g[SP][0];
                            c.g[FP][0] = 1;
                            PTS_CALC_REPEAT.call(c, 0x003d_42dc)?;
                            c.g[A0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                            c.g[V1][0] = c.g[V0][0] & 0x1f;
                            c.g[A1][0] = 0xffff_ffff_ffff_fe0f;
                            c.g[V0][0] = pts_sll(c.g[V0][0], 27);
                            c.g[V1][0] = pts_sll(c.g[V1][0], 4);
                            c.g[A0][0] &= c.g[A1][0];
                            c.g[A0][0] |= c.g[V1][0];
                            c.g[S6][0] = pts_sra(c.g[V0][0], 27);
                            c.g[V0][0] = pts_sra(pts_sll(c.g[A0][0], 23), 27);
                            c.sw(pts_at(c, S1, PTS_FLAGS), c.g[A0][0]);
                            if (c.g[V0][0] as i64) > 0 {
                                break 'reload;
                            }
                            c.cycles += 2;
                            c.g[V0][0] = (c.g[A0][0] & c.g[A1][0]) | 0x10;
                        }
                        c.cycles += 1;
                        c.sw(pts_at(c, S1, PTS_FLAGS), c.g[V0][0]);
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                }

                // Volume: (int)(volume * scale * gain), else the voice's.
                c.cycles += 20;
                c.g[A0][0] = c.lb(pts_at(c, S1, PTS_VOLUME));
                c.g[V0][0] = pts_sra(pts_sll(c.g[V0][0], 23), 27);
                c.sw(sp(c).wrapping_add(0x40), c.g[V0][0]);
                call_ptr(c, PTS_FLOATSIDF, 0x003d_4330)?;
                c.g[AT][0] = 0x490000;
                c.g[A1][0] = c.ld(PTS_VOLUME_SCALE);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, PTS_MULDF3, 0x003d_4340)?;
                c.g[A0][0] = c.lb(pts_at(c, S1, PTS_GAIN));
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, PTS_FLOATSIDF, 0x003d_434c)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, PTS_MULDF3, 0x003d_4358)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, PTS_FIXDFSI, 0x003d_4360)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = 0x450000;
                if (c.g[S2][0] as i64) < 0 {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(pts_at(c, S1, PTS_PLAT_TRACK));
                    call_ptr(c, PTS_GET_VOLUME, 0x003d_4374)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V0][0] = 0x450000;
                }

                // Next node, length and the sample value (s5).
                c.cycles += 21;
                c.g[V1][0] = c.lw(pts_at(c, S3, 0x8));
                c.g[A2][0] = c.lw(PTS_PATH_SYS);
                c.g[A3][0] = 0;
                c.g[V1][0] = pts_srl(c.g[V1][0], 20);
                c.g[T0][0] = c.lh(pts_at(c, S1, PTS_NODE));
                c.g[A1][0] = c.lw(pts_at(c, A2, 0x38));
                c.g[V1][0] = pts_sll(c.g[V1][0], 1);
                c.g[A0][0] = c.lw(pts_at(c, A2, 0x34));
                c.g[V1][0] = pts_addu(c.g[V1][0], c.g[A1][0]);
                c.g[V0][0] = c.lh(r32(c, V1));
                c.g[V0][0] = pts_sll(c.g[V0][0], 2);
                c.g[A0][0] = pts_addu(c.g[A0][0], c.g[V0][0]);
                c.sw(pts_at(c, S1, PTS_NEXT_NODE), c.g[A0][0]);
                c.g[V0][0] = c.lh(r32(c, S3));
                c.g[A0][0] = c.lw(pts_at(c, A2, 0x50));
                c.g[V0][0] = pts_sll(c.g[V0][0], 3);
                c.g[V0][0] = pts_addu(c.g[V0][0], c.g[A0][0]);
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_sub(4));
                c.sw(pts_at(c, S1, PTS_LENGTH), c.g[V1][0]);
                c.sw(pts_at(c, S1, PTS_BEAT_LENGTH), c.g[V1][0]);
                let mut in_range = false;
                if (c.g[T0][0] as i64) >= 0 {
                    c.cycles += 5;
                    c.g[A1][0] = c.lw(pts_at(c, A2, 0x34));
                    c.g[V0][0] = c.lhu(pts_at(c, A1, 0x12));
                    c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = c.lw(pts_at(c, A2, 0x38));
                        in_range = true;
                    }
                }
                if in_range {
                    c.cycles += 8;
                    c.g[V1][0] = pts_sll(c.g[T0][0], 1);
                    c.g[V1][0] = pts_addu(c.g[V1][0], c.g[V0][0]);
                    c.g[A0][0] = c.lh(r32(c, V1));
                    c.g[A0][0] = pts_sll(c.g[A0][0], 2);
                    c.g[A0][0] = pts_addu(c.g[A1][0], c.g[A0][0]);
                    c.g[V0][0] = c.lh(r32(c, A0));
                    let sample = c.g[V0][0] as i64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    if sample > 0 {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(pts_at(c, A2, 0x50));
                        c.g[V0][0] = pts_sll(c.g[V0][0], 3);
                        c.g[V0][0] = pts_addu(c.g[V0][0], c.g[V1][0]);
                        c.g[A3][0] = c.lw(r32(c, V0));
                    }
                    c.cycles += 1;
                    c.g[S5][0] = c.g[A3][0];
                } else {
                    c.cycles += 2;
                    c.g[S5][0] = 0;
                }

                // Queue (t0: the handle to record, or < 0).
                'have_handle: {
                    'reload_handle: {
                        'plain: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lbu(pts_at(c, S1, PTS_LOOPING));
                            if c.g[V0][0] == 0 {
                                c.g[T2][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                                break 'plain;
                            }
                            'counted: {
                                'clear_idle: {
                                    'unsynced: {
                                        c.cycles += 2;
                                        if c.g[S4][0] == 0 {
                                            c.g[T2][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                                            break 'unsynced;
                                        }
                                        'is_master: {
                                            c.cycles += 2;
                                            let master = c.g[S4][0] == c.g[S1][0];
                                            c.g[A0][0] = 0x100;
                                            if master {
                                                break 'is_master;
                                            }
                                            match pts_synced(c)? {
                                                PtsSynced::Unsynced => break 'unsynced,
                                                PtsSynced::Return => break 'ret,
                                                PtsSynced::Wait => break 'counted,
                                                PtsSynced::Queued => break 'clear_idle,
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[T2][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                                    }
                                    // Queue now, start time 0.
                                    c.cycles += 13;
                                    c.sw(pts_at(c, S1, PTS_START), 0);
                                    pts_queue_args(c, true);
                                    call_ptr(c, PTS_PLAT_QUEUE, 0x003d_46d4)?;
                                    c.sw(pts_at(c, S1, PTS_HANDLE), c.g[V0][0]);
                                }
                                c.cycles += 1;
                                c.sw(pts_at(c, S1, PTS_IDLE_SINCE), 0);
                            }
                            // Advance the repeat counter if this queued.
                            c.cycles += 3;
                            c.g[V1][0] = c.lw(sp(c).wrapping_add(0x44));
                            c.g[A2][0] = 0xffff_ffff_ffff_fe0f;
                            if c.g[V1][0] == 0 {
                                break 'reload_handle;
                            }
                            c.cycles += 18;
                            c.g[V0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                            c.g[A0][0] = pts_sll(c.g[V0][0], 23);
                            c.g[A1][0] = c.g[V0][0] & c.g[A2][0];
                            c.g[A0][0] = pts_sra(c.g[A0][0], 27);
                            c.g[V0][0] = pts_addu(c.g[A0][0], 1) & 0x1f;
                            c.g[V0][0] = pts_sll(c.g[V0][0], 4);
                            c.g[A1][0] |= c.g[V0][0];
                            c.sw(pts_at(c, S1, PTS_FLAGS), c.g[A1][0]);
                            c.g[V0][0] = c.lw(pts_at(c, S3, 0x4));
                            c.g[V1][0] = pts_srl(c.g[V0][0], 20);
                            c.g[V0][0] = pts_srl(c.g[V0][0], 24);
                            c.g[V1][0] &= 0xf;
                            c.g[V0][0] &= 0xf;
                            c.mult(0, V0, V0, V1);
                            c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let done = c.g[A0][0] == 0;
                            c.g[V0][0] = c.g[A1][0] & c.g[A2][0];
                            if !done {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw(pts_at(c, S3, 0xc));
                                c.g[V0][0] &= 2;
                                if c.g[V0][0] != 0 {
                                    c.g[T0][0] = c.lw(pts_at(c, S1, PTS_HANDLE));
                                    break 'have_handle;
                                }
                                c.cycles += 1;
                                c.g[V0][0] = c.g[A1][0] & c.g[A2][0];
                            }
                            c.cycles += 2;
                            c.sw(pts_at(c, S1, PTS_FLAGS), c.g[V0][0]);
                            break 'reload_handle;
                        }
                        // One-shot: queue now.
                        c.cycles += 12;
                        pts_queue_args(c, true);
                        call_ptr(c, PTS_PLAT_QUEUE, 0x003d_4778)?;
                        c.sw(pts_at(c, S1, PTS_HANDLE), c.g[V0][0]);
                    }
                    c.cycles += 1;
                    c.g[T0][0] = c.lw(pts_at(c, S1, PTS_HANDLE));
                }

                // Record the queued node in the history ring.
                c.cycles += 2;
                c.g[V0][0] = 0x450000;
                if (c.g[T0][0] as i64) < 0 {
                    break 'tail;
                }
                c.cycles += 42;
                c.g[A0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                c.g[A2][0] = c.lw(PTS_PATH_SYS);
                c.g[A3][0] = 0xffff_ffff_ff07_0000;
                c.g[V1][0] = c.lh(pts_at(c, S1, PTS_NODE));
                c.g[A0][0] = pts_srl(c.g[A0][0], 15);
                c.g[V0][0] = c.lw(pts_at(c, A2, 0x38));
                c.g[A0][0] &= 0x1f0;
                c.g[A1][0] = c.lw(pts_at(c, S1, PTS_HISTORY));
                c.g[V1][0] = pts_sll(c.g[V1][0], 1);
                c.g[V1][0] = pts_addu(c.g[V1][0], c.g[V0][0]);
                c.g[A3][0] |= 0xffff;
                c.g[A1][0] = pts_addu(c.g[A1][0], c.g[A0][0]);
                c.g[V0][0] = c.lh(r32(c, V1));
                c.g[A0][0] = c.lw(pts_at(c, A2, 0x34));
                c.sw(pts_at(c, A1, 0xc), c.g[T0][0]);
                c.g[V0][0] = pts_sll(c.g[V0][0], 2);
                c.g[S3][0] = pts_addu(c.g[A0][0], c.g[V0][0]);
                c.g[V1][0] = c.lh(pts_at(c, S1, PTS_NODE));
                c.sw(r32(c, A1), c.g[V1][0]);
                c.g[V0][0] = c.lh(r32(c, S3));
                c.g[V1][0] = c.lw(pts_at(c, A2, 0x50));
                c.g[V0][0] = pts_sll(c.g[V0][0], 3);
                c.g[V0][0] = pts_addu(c.g[V0][0], c.g[V1][0]);
                c.g[A0][0] = c.lw(r32(c, V0).wrapping_sub(4));
                c.g[V1][0] = c.lw(sp(c).wrapping_add(0x40));
                c.sw(pts_at(c, A1, 0x4), c.g[A0][0]);
                c.sw(pts_at(c, A1, 0x8), c.g[V1][0]);
                c.g[V1][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                c.g[V0][0] = pts_srl(c.g[V1][0], 19);
                c.g[A0][0] = c.g[V1][0] & c.g[A3][0];
                c.g[V0][0] &= 0x1f;
                c.g[V0][0] = pts_addu(c.g[V0][0], 1) & 0x1f;
                c.g[V0][0] = pts_sll(c.g[V0][0], 19);
                c.g[A0][0] |= c.g[V0][0];
                c.g[V1][0] = pts_srl(c.g[A0][0], 19) & 0x1f;
                c.g[V0][0] = pts_srl(c.g[A0][0], 14) & 0x1f;
                c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                c.sw(pts_at(c, S1, PTS_FLAGS), c.g[A0][0]);
                if c.g[V1][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A0][0] & c.g[A3][0];
                    c.sw(pts_at(c, S1, PTS_FLAGS), c.g[V0][0]);
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw(pts_at(c, S1, PTS_HANDLE));
        }

        c.cycles += 12;
        frame.restore(c, S0, 0xe0);
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore(c, FP, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{elf_image, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Gate of PATH_TrackSubmitQueue's fakes (off: the slot's previous body).
    pub(crate) const PTS_CTRL: u32 = DATA + 0x7_0000;
    pub(crate) const PTS_MAGIC: u32 = 0x3d41_a8c5;

    static PTS_PREV_QUEUED: AtomicUsize = AtomicUsize::new(0);
    static PTS_PREV_VOLUME: AtomicUsize = AtomicUsize::new(0);
    static PTS_PREV_QUEUE: AtomicUsize = AtomicUsize::new(0);

    /// A logged fake returning the sign-extended word at `result`.
    fn pts_fake_with(c: &mut Cpu, target: u32, result: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc == ra {
            c.g[V0][0] = sx(c.bus.read32(result));
        }
    }

    /// `PATH_PlatGetQueuedEntryValue(track, i)`: the value at `PTS_CTRL +
    /// 0x10 + 4 * (i & 3)`.
    fn pts_fake_queued(c: &mut Cpu) {
        if c.bus.read32(PTS_CTRL) != PTS_MAGIC {
            return fakes::fall_back(c, &PTS_PREV_QUEUED, ee::s08::f_002aeaa8__generated);
        }
        let i = c.g[A1][0] as u32 & 3;
        pts_fake_with(c, PTS_GET_QUEUED, PTS_CTRL + 0x10 + 4 * i);
    }

    /// `PATH_PlatGetTrackVolume(track)`: the word at `PTS_CTRL + 4`.
    fn pts_fake_volume(c: &mut Cpu) {
        if c.bus.read32(PTS_CTRL) != PTS_MAGIC {
            return fakes::fall_back(c, &PTS_PREV_VOLUME, ee::s08::f_002af428__generated);
        }
        pts_fake_with(c, PTS_GET_VOLUME, PTS_CTRL + 4);
    }

    /// `PATH_PlatQueue(...)`: the handle at `PTS_CTRL + 8`.
    fn pts_fake_queue(c: &mut Cpu) {
        if c.bus.read32(PTS_CTRL) != PTS_MAGIC {
            return fakes::fall_back(c, &PTS_PREV_QUEUE, ee::s08::f_002ae7a8__generated);
        }
        pts_fake_with(c, PTS_PLAT_QUEUE, PTS_CTRL + 8);
    }

    #[allow(unused_unsafe)]
    fn pts_install_fakes() {
        use std::ptr::addr_of_mut;
        unsafe {
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002AEAA8),
                pts_fake_queued,
                &PTS_PREV_QUEUED,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002AF428),
                pts_fake_volume,
                &PTS_PREV_VOLUME,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002AE7A8),
                pts_fake_queue,
                &PTS_PREV_QUEUE,
            );
        }
    }

    /// PATH_TrackSubmitQueue: no node (reset); 0-3 queued entries with
    /// negative and positive values; one-shot (idle time set or kept) and
    /// looping tracks (repeat count positive, or recomputed by the real
    /// `PATH_CalcNodeRepeatCount`); volume positive or negative (voice
    /// volume); the node index in and out of range and samples 0/-1/n; no
    /// master, the track itself as master, or another master with every
    /// sync mode, beat/bar nibbles zero or not, a pending resync early or
    /// due, the real `PATH_TrackGetBeatPos` (its voice idle), the last
    /// synced queue recent or old; queue handles negative or not and the
    /// repeat/history counters wrapping. The voice calls are gated logged
    /// fakes; one may unwind.
    #[test]
    fn path_track_submit_queue_matches() {
        const TRACK: u32 = DATA + 0x1000;
        const MASTER: u32 = DATA + 0x2000;
        const SYS: u32 = DATA + 0x3000;
        const NODES: u32 = DATA + 0x4000;
        const HIST: u32 = DATA + 0x5000;
        const IDX: u32 = DATA + 0x8000;
        const SAMPLES: u32 = DATA + 0xc000;
        /// `PATH_Plat` voice table (`0x4d3e98`): empty, so the beat
        /// position's status query returns at once.
        const VOICES: u32 = 0x004d_3e98;
        pts_install_fakes();
        covered(
            ee::s14::f_003d41a8__generated,
            path_track_submit_queue,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x003d_41a8, 6);
                c.pc = 0x003d_41a8;
                c.bus.write32(PTS_CTRL, PTS_MAGIC);
                for i in 0..4 {
                    let v = match r.next_u32() % 3 {
                        0 => (-8i32) as u32,
                        _ => r.next_u32() % 100,
                    };
                    c.bus.write32(PTS_CTRL + 0x10 + 4 * i, v);
                }
                c.bus.write32(PTS_CTRL + 4, r.next_u32() % 128);
                let handle = if r.next_u32() % 4 == 0 {
                    u32::MAX
                } else {
                    r.next_u32() % 64
                };
                c.bus.write32(PTS_CTRL + 8, handle);

                let now = 100 + r.next_u32() % 1000;
                c.bus.write32(PTS_TIME, now);
                c.bus
                    .write32(PTS_LAST_SYNC, now.wrapping_sub(r.next_u32() % 700));
                c.bus.write32(PTS_PATH_SYS, SYS);
                for i in 0..4 {
                    c.bus.write32(VOICES + 4 * i, 0);
                }

                // The path system: master index -1..3 over {0, track, master, master}.
                let master = (r.next_u32() % 5) as i32 - 1;
                c.bus.write32(SYS, master as u32);
                c.bus.write32(SYS + 0x58, 0);
                c.bus.write32(SYS + 0x5c, TRACK);
                c.bus.write32(SYS + 0x60, MASTER);
                c.bus.write32(SYS + 0x64, MASTER);
                c.bus.write32(SYS + 0x34, NODES);
                c.bus.write32(SYS + 0x38, IDX);
                c.bus.write32(SYS + 0x50, SAMPLES);
                // Node indices: eight 16-byte nodes.
                for i in 0..0x1000u32 {
                    c.bus.write16(IDX + 2 * i, ((r.next_u32() % 8) * 4) as u16);
                }
                for k in 0..8u32 {
                    let n = NODES + 16 * k;
                    let sample = match r.next_u32() % 6 {
                        0 => 0,
                        1 => 0xffff,
                        _ => 1 + r.next_u32() % 8,
                    };
                    c.bus.write32(n, (r.next_u32() % 10) << 16 | sample);
                    let mut beats = r.next_u32();
                    if r.next_u32() % 3 == 0 {
                        beats &= 0xff0f_ffff;
                    }
                    if r.next_u32() % 3 == 0 {
                        beats &= 0xf0ff_ffff;
                    }
                    c.bus.write32(n + 4, beats);
                    c.bus.write32(n + 8, r.next_u32());
                    c.bus.write32(n + 0xc, r.next_u32());
                }
                for i in 0..0x18u32 {
                    let v = if r.next_u32() % 5 == 0 {
                        r.next_u32()
                    } else {
                        r.next_u32() % 2000
                    };
                    c.bus.write32(SAMPLES - 0x10 + 4 * i, v);
                }

                // The track.
                for i in 0..0x44u32 {
                    c.bus.write32(TRACK + 4 * i, r.next_u32());
                }
                let node = if seed % 8 == 0 {
                    0xffff
                } else {
                    r.next_u32() % 8
                };
                c.bus.write16(TRACK + PTS_NODE, node as u16);
                let flags = r.next_u32() & !(0x1f << 14) | (r.next_u32() % 4) << 14;
                c.bus.write32(TRACK + PTS_FLAGS, flags);
                let looping = if r.next_u32() % 3 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 255
                };
                c.bus.write8(TRACK + PTS_LOOPING, looping as u8);
                if r.next_u32() % 2 == 0 {
                    c.bus.write32(TRACK + PTS_IDLE_SINCE, 0);
                }
                c.bus.write32(TRACK + PTS_PLAT_TRACK, r.next_u32() % 4);
                c.bus.write32(TRACK + PTS_HISTORY, HIST);
                let resync = match r.next_u32() % 3 {
                    0 => 0,
                    1 => now + 5 + r.next_u32() % 100,
                    _ => r.next_u32() % (now + 5),
                };
                c.bus.write32(TRACK + PTS_RESYNC, resync);

                // The master track.
                for i in 0..0x44u32 {
                    c.bus.write32(MASTER + 4 * i, r.next_u32());
                }
                c.bus.write16(MASTER + PTS_NODE, (r.next_u32() % 8) as u16);
                c.bus.write32(MASTER + PTS_PLAT_TRACK, r.next_u32() % 4);
                if r.next_u32() % 4 == 0 {
                    c.bus.write32(MASTER + PTS_NEXT_NODE, 0);
                }

                c.g[A0][0] = sx(TRACK);
            },
        );
    }
}
