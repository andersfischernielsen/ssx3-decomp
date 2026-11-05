//! Staging wave 205 (oc237): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_7098, c_stream_man_update, "cStreamMan_Update"),
    (0x0037_4b38, bezier_patch_cache_release_unused, "BezierPatchCache_ReleaseUnused"),
    (0x002f_6910, rider_base_match_cheat_code, "RiderBase_MatchCheatCode"),
    (0x003d_5508, path_set_volume_percent, "PATH_SetVolumePercent"),
    (
        0x002a_43b8,
        ssx_audio_system_flush_pending_speech,
        "SSXAudioSystem_FlushPendingSpeech",
    ),
    (
        0x0029_7fa0,
        ssx_audio_system_update_object_loops,
        "SSXAudioSystem_UpdateObjectLoops",
    ),
    (0x0022_a4a8, nash_classify_effects, "Nash_ClassifyEffects"),
    (0x0032_6b88, pad_queue_poll, "PadQueue_Poll"),
];

// ---------------------------------------------------------------------------
// cStreamMan_Update (0x3a7098)

/// cStreamMan fields (`src/world/streamman.cpp`).
mod stream_man {
    /// The world cache this manager streams into (`+0`).
    pub const CACHE: u32 = 0x0;
    /// The stream file handle (`+0x88`).
    pub const FILE: u32 = 0x88;
    /// The decompression buffer (`+0x8c`).
    pub const BUFFER: u32 = 0x8c;
    /// A hull is streaming (`+0x90`) and which one (`+0x94`).
    pub const ACTIVE: u32 = 0x90;
    pub const HULL: u32 = 0x94;
    /// Parsing stopped mid-buffer and must be resumed first (`+0xac`).
    pub const PENDING: u32 = 0xac;
    /// Bytes left in the buffer (`+0xb0`) and the read cursor (`+0xb4`).
    pub const LEFT: u32 = 0xb0;
    pub const CURSOR: u32 = 0xb4;
    /// Throttled streaming (`+0xd4`): a chunk every `+0xdc` audio ticks,
    /// `+0xe0` the ticks still to wait, `+0xe4` the tick last seen.
    pub const THROTTLED: u32 = 0xd4;
    pub const INTERVAL: u32 = 0xdc;
    pub const WAIT: u32 = 0xe0;
    pub const LAST_TICK: u32 = 0xe4;
    /// `gp+0x2a74`: the audio frame clock; its tick count at `+0x18`.
    pub const GP_CLOCK: u32 = 0x2a74;
    pub const CLOCK_TICKS: u32 = 0x18;
}

mod stream_callee {
    use super::{ee, Callee};
    pub const PARSE_CHUNKS: Callee = Callee {
        addr: 0x003a_7238,
        f: ee::s13::cStreamMan_ParseChunks,
    };
    pub const DECOMPRESS: Callee = Callee {
        addr: 0x003a_7218,
        f: ee::s13::cStreamMan_Decompress,
    };
    pub const SET_HULL_RESIDENT: Callee = Callee {
        addr: 0x003a_8650,
        f: ee::s13::cWorldCache_SetHullResident,
    };
    /// `StreamFile_GetChunk`, `StreamFile_ReleaseChunk`, `StreamFile_IsIdle`
    /// (called through the function table).
    pub const GET_CHUNK: u32 = 0x003e_12e0;
    pub const RELEASE_CHUNK: u32 = 0x003e_13e8;
    pub const IS_IDLE: u32 = 0x003e_1530;
}

/// `void cStreamMan_Update(cStreamMan* self)` (0x3a7098): per-frame hull
/// streaming. While a hull is streaming: resume a parse that ran out of
/// buffer; when throttled, count the audio ticks down and wait; then take
/// every chunk the stream file has, decompress it into the buffer and parse
/// its resources (stopping after one chunk when throttled, or when parsing
/// stalls, which marks it pending). When the file is idle, the hull is
/// resident: tell the world cache and stop.
pub fn c_stream_man_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(this + stream_man::ACTIVE).lw(c);
        // The paths through the block at 0x3a71fc charge one more cycle.
        let mut tail = false;
        if c.g[V0][0] != 0 {
            tail = stream_man_update_active(c, this)?;
        }
        if tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The body of [`c_stream_man_update`] for an active manager; `Ok(true)`
/// when it leaves through the block at 0x3a71fc (pending set, or hull done).
fn stream_man_update_active(c: &mut Cpu, this: u32) -> Result<bool, crate::call::Unwound> {
    use stream_callee::*;
    use stream_man::*;
    let at = |off: u32| Addr(this.wrapping_add(off));

    // Resume a parse that stopped mid-buffer (a0 is still `this`).
    c.cycles += 3;
    c.g[V0][0] = at(PENDING).lw(c);
    c.g[A1][0] = sx(this.wrapping_add(LEFT));
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[A2][0] = sx(this.wrapping_add(CURSOR));
        PARSE_CHUNKS.call(c, 0x003a_70d0)?;
        if c.g[V0][0] == 0 {
            return Ok(false);
        }
    }

    // Throttled: count the audio ticks since the last frame off the wait.
    c.cycles += 3;
    c.g[V0][0] = at(THROTTLED).lw(c);
    let throttled = c.g[V0][0] != 0;
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = Addr(gp.wrapping_add(GP_CLOCK)).lw(c);
    if throttled {
        c.cycles += 4;
        c.g[A0][0] = at(LAST_TICK).lw(c);
        let clock = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(clock.wrapping_add(CLOCK_TICKS)).lw(c);
        let same = c.g[A0][0] == c.g[V1][0];
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        if !same {
            c.cycles += 6;
            let wait = at(WAIT).read_fpu(c);
            let ticks = FpuF32::from_i32(c.g[V0][0] as u32 as i32);
            abi::set_fpr(c, 1, ticks);
            let now = c.g[V1][0];
            at(LAST_TICK).sw(c, now);
            let wait = wait - ticks;
            abi::set_fpr(c, 0, wait);
            at(WAIT).write_fpu(c, wait);
        }
        c.cycles += 6;
        let wait = at(WAIT).read_fpu(c);
        abi::set_fpr(c, 1, wait);
        abi::set_fpr(c, 0, FpuF32::ZERO);
        if abi::c_lt_s(c, FpuF32::ZERO, wait) {
            return Ok(false);
        }
    }

    // Take the chunks the stream file has.
    c.cycles += 5;
    c.g[A0][0] = at(FILE).lw(c);
    call_ptr(c, GET_CHUNK, 0x003a_712c)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[S2][0] = 1;
    if c.g[S1][0] != 0 {
        c.cycles += 2;
        let interval = at(INTERVAL).read_fpu(c);
        abi::set_fpr(c, 0, interval);
        loop {
            // One chunk: tag/size word at +4, payload at +8.
            let chunk = c.g[S1][0] as u32;
            let ch = |off: u32| Addr(chunk.wrapping_add(off));
            c.cycles += 28;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = at(BUFFER).lw(c);
            c.g[A3][0] = sx(chunk.wrapping_add(8));
            let interval = FpuF32(c.f[0]);
            at(WAIT).write_fpu(c, interval);
            let b7 = ch(7).lbu(c);
            c.g[V0][0] = sx((ch(6).lbu(c) as u32) << 16);
            c.g[V1][0] = sx((ch(5).lbu(c) as u32) << 8);
            c.g[T0][0] = ch(4).lbu(c);
            let size = sx((b7 as u32) << 24) | c.g[V0][0] | c.g[V1][0] | c.g[T0][0];
            c.g[A2][0] = sx((size as u32).wrapping_sub(8));
            DECOMPRESS.call(c, 0x003a_7180)?;
            c.g[A0][0] = at(FILE).lw(c);
            c.g[A1][0] = c.g[S1][0];
            let left = c.g[V0][0];
            at(LEFT).sw(c, left);
            call_ptr(c, RELEASE_CHUNK, 0x003a_7190)?;
            c.g[V0][0] = at(BUFFER).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(this.wrapping_add(LEFT));
            c.g[A2][0] = sx(this.wrapping_add(CURSOR));
            let buffer = c.g[V0][0];
            at(CURSOR).sw(c, buffer);
            PARSE_CHUNKS.call(c, 0x003a_71a8)?;
            if c.g[V0][0] == 0 {
                // Stalled: resume next frame.
                c.cycles += 2;
                let one = c.g[S2][0];
                at(PENDING).sw(c, one);
                return Ok(true);
            }
            c.g[V0][0] = at(THROTTLED).lw(c);
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                break;
            }
            c.cycles += 5;
            c.g[A0][0] = at(FILE).lw(c);
            call_ptr(c, GET_CHUNK, 0x003a_71c8)?;
            c.g[S1][0] = c.g[V0][0];
            if c.g[S1][0] == 0 {
                break;
            }
            let interval = at(INTERVAL).read_fpu(c);
            abi::set_fpr(c, 0, interval);
            if event_due(c) {
                deliver_event_here(c, 0x003a_7140);
            }
        }
    }

    // Everything read and parsed: the hull is resident.
    c.cycles += 4;
    c.g[A0][0] = at(FILE).lw(c);
    call_ptr(c, IS_IDLE, 0x003a_71dc)?;
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    c.cycles += 6;
    c.g[A0][0] = at(CACHE).lw(c);
    c.g[A1][0] = at(HULL).lw(c);
    SET_HULL_RESIDENT.call(c, 0x003a_71f0)?;
    c.g[V0][0] = u64::MAX;
    at(ACTIVE).sw(c, 0);
    at(HULL).sw(c, u64::MAX);
    Ok(true)
}

// ---------------------------------------------------------------------------
// BezierPatchCache_ReleaseUnused (0x374b38)

/// Bezier patch cache fields (`src/render/particle.cpp`), per patch level
/// `i` in 0..3 (word arrays indexed `4 * i`).
mod patch_cache {
    /// Entries in use (`+0`) and the entry lists (`+0x18`; 12-byte entries).
    pub const COUNT: u32 = 0x0;
    pub const ENTRIES: u32 = 0x18;
    /// Free entry-index lists (`+0x30`) and their counts (`+0x3c`).
    pub const FREE_LIST: u32 = 0x30;
    pub const FREE_COUNT: u32 = 0x3c;
    /// Free slot lists (`+0x1cc`) and their counts (`+0x1d8`).
    pub const SLOT_LIST: u32 = 0x1cc;
    pub const SLOT_COUNT: u32 = 0x1d8;
    /// Entry: owner (`+0`), flags (`+4`: 4 and 8 = used this frame / last
    /// frame), slot (`+8`, -1 none); the owner's slot index per level is a
    /// halfword at `+0x1a6 + 2 * i`.
    pub const E_OWNER: u32 = 0x0;
    pub const E_FLAGS: u32 = 0x4;
    pub const E_SLOT: u32 = 0x8;
    pub const E_SIZE: u32 = 0xc;
    pub const OWNER_SLOT: u32 = 0x1a6;
}

/// `void BezierPatchCache_ReleaseUnused(BezierPatchCache* cache)` (0x374b38):
/// age the cached patch entries of the three levels: an entry used this
/// frame (flag 4) or last frame (flag 8) loses that flag; any other
/// non-empty entry is released: its owner forgets the slot (-1), its slot (if
/// any) goes back to the level's free slots, and the entry index to the free
/// entries. A leaf with a frame for s0-s6.
pub fn bezier_patch_cache_release_unused(c: &mut Cpu) {
    use patch_cache::*;
    c.cycles += 16;
    let frame = Frame::open(c, 0x70);
    let cache = c.g[A0][0] as u32;
    c.g[T2][0] = c.g[A0][0];
    frame.save(c, S2, 0x40);
    c.g[A1][0] = 0;
    frame.save(c, S3, 0x30);
    c.g[S2][0] = sx(cache.wrapping_add(FREE_COUNT));
    frame.save(c, S4, 0x20);
    c.g[S3][0] = sx(cache.wrapping_add(FREE_LIST));
    frame.save(c, S5, 0x10);
    c.g[S4][0] = sx(cache.wrapping_add(SLOT_COUNT));
    frame.save(c, S6, 0);
    c.g[S5][0] = sx(cache.wrapping_add(SLOT_LIST));
    frame.save(c, S0, 0x60);
    c.g[S6][0] = sx(cache.wrapping_add(ENTRIES));
    frame.save(c, S1, 0x50);
    c.g[A0][0] = 0;
    loop {
        // Level i = a1 (a0 = 4 * i).
        let level = c.g[A1][0] as u32;
        let i4 = c.g[A0][0] as u32;
        c.cycles += 7;
        c.g[A3][0] = 0;
        c.g[V0][0] = sx(cache.wrapping_add(i4));
        c.g[V1][0] = sx(cache.wrapping_add(ENTRIES).wrapping_add(i4));
        c.g[T3][0] = Addr(cache.wrapping_add(COUNT + i4)).lw(c);
        c.g[T7][0] = sx(level.wrapping_add(1));
        let count = c.g[T3][0] as i64;
        c.g[V0][0] = Addr(cache.wrapping_add(ENTRIES + i4)).lw(c);
        if count > 0 {
            c.cycles += 11;
            c.g[T6][0] = sx(level << 1);
            let free_count = cache.wrapping_add(FREE_COUNT).wrapping_add(i4);
            let slot_list = cache.wrapping_add(SLOT_LIST).wrapping_add(i4);
            let slot_count = cache.wrapping_add(SLOT_COUNT).wrapping_add(i4);
            let free_list = cache.wrapping_add(FREE_LIST).wrapping_add(i4);
            c.g[T1][0] = sx(free_count);
            c.g[A2][0] = c.g[V0][0];
            c.g[S1][0] = !4u64;
            c.g[S0][0] = !8u64;
            c.g[T9][0] = u64::MAX;
            c.g[T5][0] = sx(slot_list);
            c.g[T0][0] = sx(slot_count);
            c.g[T8][0] = u64::MAX;
            c.g[T4][0] = sx(free_list);
            loop {
                // Entry j = a3 at a2.
                let e = c.g[A2][0] as u32;
                c.cycles += 4;
                let flags = Addr(e.wrapping_add(E_FLAGS)).lw(c);
                c.g[V1][0] = flags;
                c.g[V0][0] = flags & !4;
                let mut tail = true;
                if flags & 4 != 0 {
                    c.cycles += 2;
                    Addr(e.wrapping_add(E_FLAGS)).sw(c, flags & !4);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = flags & !8;
                    if flags & 8 != 0 {
                        c.cycles += 2;
                        Addr(e.wrapping_add(E_FLAGS)).sw(c, flags & !8);
                    } else {
                        c.cycles += 2;
                        if flags == 0 {
                            tail = false;
                        } else {
                            // Release: the owner forgets its slot.
                            c.cycles += 7;
                            let owner = Addr(e.wrapping_add(E_OWNER)).lw(c);
                            Addr(e.wrapping_add(E_FLAGS)).sw(c, 0);
                            c.g[V0][0] = sx((owner as u32).wrapping_add(level << 1));
                            Addr((c.g[V0][0] as u32).wrapping_add(OWNER_SLOT)).sh(c, u64::MAX);
                            let slot = Addr(e.wrapping_add(E_SLOT)).lw(c);
                            c.g[A1][0] = slot;
                            if (slot as i64) >= 0 {
                                // The slot goes back to the free slots.
                                c.cycles += 9;
                                let n = Addr(slot_count).lw(c);
                                let list = Addr(slot_list).lw(c);
                                c.g[A0][0] = list;
                                c.g[V1][0] = sx(((n as u32) << 2).wrapping_add(list as u32));
                                c.g[V0][0] = sx((n as u32).wrapping_add(1));
                                Addr(c.g[V1][0] as u32).sw(c, slot);
                                let n1 = c.g[V0][0];
                                Addr(slot_count).sw(c, n1);
                                Addr(e.wrapping_add(E_SLOT)).sw(c, u64::MAX);
                            }
                            // The entry index goes to the free entries.
                            let n = Addr(free_count).lw(c);
                            c.cycles += 6;
                            let list = Addr(free_list).lw(c);
                            c.g[A0][0] = list;
                            c.g[V1][0] = sx(((n as u32) << 2).wrapping_add(list as u32));
                            c.g[V0][0] = sx((n as u32).wrapping_add(1));
                            let j = c.g[A3][0];
                            Addr(c.g[V1][0] as u32).sw(c, j);
                            let n1 = c.g[V0][0];
                            Addr(free_count).sw(c, n1);
                        }
                    }
                }
                if tail {
                    c.cycles += 1;
                }
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T3][0] as i64)) as u64;
                c.g[A2][0] = sx(e.wrapping_add(E_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0037_4bc0);
                }
            }
        }
        c.cycles += 4;
        c.g[A1][0] = c.g[T7][0];
        c.g[V0][0] = ((c.g[A1][0] as i64) < 3) as u64;
        c.g[A0][0] = sx((c.g[A1][0] as u32) << 2);
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0037_4b78);
        }
    }
    c.cycles += 9;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore(c, S6, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// RiderBase_MatchCheatCode (0x2f6910)

/// The cheat-code matcher (`src/visualfx/worldlightman.cpp`): state byte
/// (`+4`: -1 failed, 0 entering, 1 unlocked), the step in the code (`+5`),
/// presses counted for this step (`+6`), flags (`+7`: 1 = events handled,
/// 2 = toggled, 0x10/0x20/0x40 = pending button events).
mod cheat {
    pub const STATE: u32 = 0x4;
    pub const STEP: u32 = 0x5;
    pub const COUNT: u32 = 0x6;
    pub const FLAGS: u32 = 0x7;
    /// The code: presses per step (9 signed bytes at 0x445e30).
    pub const CODE: u32 = 0x0044_5e30;
    pub const STEPS: u64 = 9;
    /// `gp+0xad0`: set to 8 when the full code is entered with both buttons.
    pub const GP_UNLOCK: u32 = 0xad0;
}

mod cheat_callee {
    use super::{ee, Callee};
    pub const GET_EVENTS: Callee = Callee {
        addr: 0x002f_6b38,
        f: ee::s09::RiderBase_GetCheatCodeEvents,
    };
    pub const CLEAR_EVENTS: Callee = Callee {
        addr: 0x002f_6b28,
        f: ee::s09::RiderBase_ClearCheatCodeEvents,
    };
}

/// How [`rider_base_match_cheat_code`] leaves: straight to the epilogue
/// (`lq s0` already done), through 0x2f6a44 (+1, `lq s0`), or through
/// 0x2f6a40 (+1, store the flags in v0, then 0x2f6a44).
enum CheatExit {
    Direct,
    Done,
    StoreFlags,
}

/// `void RiderBase_MatchCheatCode(Matcher* self)` (0x2f6910): consume the
/// pending button events; on a new press (flag 1 clear) step the code
/// state machine: from failed (-1) the 0x40 button restarts; while entering
/// (0) a 0x10 press counts and a 0x20 press checks the count against the
/// code's step (9 steps unlock, a mismatch fails); once unlocked (1) both
/// buttons (0x60) set `gp+0xad0 = 8` and 0x40 toggles flag 2. With no event
/// flag 1 is cleared.
pub fn rider_base_match_cheat_code(c: &mut Cpu) {
    use cheat::*;
    use cheat_callee::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        let at = |off: u32| Addr(this.wrapping_add(off));
        GET_EVENTS.call(c, 0x002f_6928)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        CLEAR_EVENTS.call(c, 0x002f_6934)?;
        let events = c.g[S1][0];
        let exit = if events == 0 {
            c.cycles += 3;
            c.g[V0][0] = at(FLAGS).lbu(c) & 0xfe;
            CheatExit::StoreFlags
        } else {
            c.g[V1][0] = at(FLAGS).lbu(c);
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] & 1;
            if c.g[V0][0] != 0 {
                CheatExit::Direct
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V1][0] | 1;
                c.g[V1][0] = at(STATE).lb(c);
                let flags = c.g[A0][0];
                at(FLAGS).sb(c, flags);
                let state = c.g[V1][0] as i64;
                if state == 0 {
                    cheat_entering(c, this, events)
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    if state > 0 {
                        c.cycles += 2;
                        c.g[V0][0] = 0x60;
                        if state == 1 {
                            // Unlocked.
                            c.cycles += 2;
                            if events == 0x60 {
                                c.cycles += 3;
                                c.g[V0][0] = 8;
                                let gp = c.g[GP][0] as u32;
                                Addr(gp.wrapping_add(GP_UNLOCK)).sw(c, 8);
                                CheatExit::Done
                            } else {
                                c.cycles += 2;
                                let toggle = events & 0x40 != 0;
                                c.g[V0][0] = c.g[A0][0] ^ 2;
                                if toggle {
                                    CheatExit::StoreFlags
                                } else {
                                    CheatExit::Done
                                }
                            }
                        } else {
                            c.cycles += 2;
                            CheatExit::Direct
                        }
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = events & 0x40;
                        if state == -1 {
                            // Failed: the 0x40 button restarts.
                            c.cycles += 2;
                            if c.g[V0][0] == 0 {
                                CheatExit::Direct
                            } else {
                                c.cycles += 5;
                                at(FLAGS).sb(c, 0);
                                at(STATE).sb(c, 0);
                                at(COUNT).sb(c, 0);
                                at(STEP).sb(c, 0);
                                CheatExit::Done
                            }
                        } else {
                            c.cycles += 2;
                            CheatExit::Direct
                        }
                    }
                }
            }
        };
        match exit {
            CheatExit::StoreFlags => {
                c.cycles += 2;
                let flags = c.g[V0][0];
                at(FLAGS).sb(c, flags);
                frame.restore(c, S0, 0x20);
            }
            CheatExit::Done => {
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
            CheatExit::Direct => frame.restore(c, S0, 0x20),
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// State 0 of [`rider_base_match_cheat_code`]: count 0x10 presses; on 0x20
/// compare the count with the code's step.
fn cheat_entering(c: &mut Cpu, this: u32, events: u64) -> CheatExit {
    use cheat::*;
    let at = |off: u32| Addr(this.wrapping_add(off));
    c.cycles += 3;
    c.g[V0][0] = events & 0x20;
    if events & 0x10 != 0 {
        c.cycles += 4;
        c.g[V0][0] = sx((at(COUNT).lbu(c) as u32).wrapping_add(1));
        let n = c.g[V0][0];
        at(COUNT).sb(c, n);
        return CheatExit::Done;
    }
    c.cycles += 2;
    if c.g[V0][0] == 0 {
        c.g[V0][0] = 0x44_0000;
        return CheatExit::Done;
    }
    c.cycles += 7;
    let step = at(STEP).lb(c);
    c.g[A0][0] = at(COUNT).lb(c);
    c.g[V1][0] = sx((step as u32).wrapping_add(CODE));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lb(c);
    c.g[A1][0] = at(STEP).lbu(c);
    if c.g[A0][0] != c.g[V0][0] {
        // Wrong count: failed.
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        at(STATE).sb(c, u64::MAX);
        return CheatExit::Done;
    }
    c.cycles += 6;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
    at(COUNT).sb(c, 0);
    c.g[A0][0] = c.g[V0][0] & 0xff;
    c.g[V1][0] = STEPS;
    let next = c.g[V0][0];
    at(STEP).sb(c, next);
    if c.g[A0][0] == STEPS {
        // The whole code: unlocked.
        c.cycles += 5;
        c.g[V0][0] = 1;
        at(STEP).sb(c, 0);
        at(STATE).sb(c, 1);
        at(COUNT).sb(c, 0);
    }
    CheatExit::Done
}

// ---------------------------------------------------------------------------
// GameUI_Update (0x20cfd0)

/// Globals and fields used by [`game_ui_update`] (`src/fe/ovtemplatedialog.cpp`).
mod game_ui {
    /// `gp-0x848`: the world; its game module (`+0x84`) and BE library
    /// (`+0x78`).
    pub const GP_WORLD: u32 = 0x848;
    pub const W_GAME: u32 = 0x84;
    pub const W_BE: u32 = 0x78;
    /// `gp-0x9f4`: the pending UI action (0 none, 1 menu result, 2 leave the
    /// replay, 3 close the PDA template, 4 request game state 0xe).
    pub const GP_ACTION: u32 = 0x9f4;
    /// Game module: replay (`+0x28`), UI manager (`+0x48`, its state stack at
    /// `+0x18`), the HUD block (`+0x200`, its "dirty" word at `+8`), the game
    /// mode (`+0x214`).
    pub const G_REPLAY: u32 = 0x28;
    pub const G_UI: u32 = 0x48;
    pub const UI_STACK: u32 = 0x18;
    pub const G_HUD: u32 = 0x200;
    pub const G_MODE: u32 = 0x214;
    /// UI state id (`+0x98`); 8 is the pause menu.
    pub const S_ID: u32 = 0x98;
    /// The action jump table.
    pub const JUMP_TABLE: u32 = 0x0047_1ae0;
    /// The "online" flag word the pause-menu exit checks.
    pub const ONLINE: u32 = 0x0053_4b30;
}

mod game_ui_callee {
    use super::{ee, Callee};
    pub const HANDLE_MENU_RESULT: Callee = Callee {
        addr: 0x0020_ccf8,
        f: ee::s05::GameUI_HandleMenuResult,
    };
    pub const PDA_TEMPLATE_CLOSE: Callee = Callee {
        addr: 0x0020_d190,
        f: ee::s05::OVScreenPDATemplate_Close,
    };
    /// Called through the function table.
    pub const GET_ACTIVE_STATE: u32 = 0x0039_fa88;
    pub const REPLAY_FINISH: u32 = 0x0027_06f0;
    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const CLOSE_ALL: u32 = 0x0039_f840;
    pub const REQUEST_STATE: u32 = 0x0023_1250;
}

/// `void GameUI_Update(void)` (0x20cfd0): run the pending UI action
/// (`gp-0x9f4`, through a jump table): 1 handles the menu result; 2 leaves
/// the replay (stop it, mark the HUD dirty and, online in the pause menu of
/// mode 2, close every UI state); 3 closes the PDA template and every UI
/// state; 4 requests game state 0xe. Actions 2-4 clear the pending action.
#[allow(dead_code)]
pub fn game_ui_update(c: &mut Cpu) {
    use game_ui::*;
    use game_ui_callee::*;
    guest(c, |c| {
        let gp = c.g[GP][0] as u32;
        let gpa = |off: u32| Addr(gp.wrapping_sub(off));
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = gpa(GP_WORLD).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(W_GAME)).lw(c);
        c.g[S1][0] = Addr((c.g[V0][0] as u32).wrapping_add(G_UI)).lw(c);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(UI_STACK));
        c.g[S0][0] = 0;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, GET_ACTIVE_STATE, 0x0020_d000)?;
            c.g[S0][0] = c.g[V0][0];
        }

        c.cycles += 4;
        c.g[V1][0] = gpa(GP_ACTION).lw(c);
        c.g[V0][0] = (c.g[V1][0] < 5) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[V0][0] = 0x0047_0000;
        // Whether the exit passes the block at 0x20d0f8 (+1 cycle).
        let mut via_d0f8 = true;
        if in_range {
            c.cycles += 6;
            c.g[V1][0] = sx(((c.g[V1][0] as u32) << 2).wrapping_add(JUMP_TABLE));
            c.g[V0][0] = sx(JUMP_TABLE);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let target = c.g[A0][0] as u32;
            match target {
                0x0020_d02c => {
                    // 1: the menu result.
                    c.cycles += 4;
                    HANDLE_MENU_RESULT.call(c, 0x0020_d034)?;
                    via_d0f8 = false;
                }
                0x0020_d03c => {
                    // 2: leave the replay.
                    c.cycles += 11;
                    c.g[V1][0] = gpa(GP_WORLD).lw(c);
                    gpa(GP_ACTION).sw(c, 0);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(W_GAME)).lw(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(G_REPLAY)).lw(c);
                    call_ptr(c, REPLAY_FINISH, 0x0020_d050)?;
                    c.g[A1][0] = gpa(GP_WORLD).lw(c);
                    let world = c.g[A1][0] as u32;
                    c.g[A0][0] = 1;
                    c.g[V0][0] = Addr(world.wrapping_add(W_GAME)).lw(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(G_HUD)).lw(c);
                    Addr((c.g[V1][0] as u32).wrapping_add(8)).sw(c, 1);
                    // The state is read from s0 again, as the original does
                    // (a callee that misbehaves on test data can change it).
                    if c.g[S0][0] != 0 {
                        via_d0f8 = false;
                        c.cycles += 5;
                        c.g[V0][0] = Addr(world.wrapping_add(W_GAME)).lw(c);
                        c.g[A0][0] = 2;
                        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(G_MODE)).lw(c);
                        if c.g[V1][0] == 2 {
                            c.cycles += 4;
                            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(S_ID)).lw(c);
                            c.g[V0][0] = 8;
                            if c.g[V1][0] == 8 {
                                c.cycles += 7;
                                c.g[A0][0] = Addr(world.wrapping_add(W_BE)).lw(c);
                                c.g[A1][0] = 7;
                                call_ptr(c, BE_GET_INTERFACE, 0x0020_d098)?;
                                c.g[V1][0] = 0x0053_0000;
                                c.g[V0][0] = Addr(ONLINE).lw(c);
                                if c.g[V0][0] != 0 {
                                    c.cycles += 4;
                                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(UI_STACK));
                                    call_ptr(c, CLOSE_ALL, 0x0020_d0b0)?;
                                }
                            }
                        }
                    }
                }
                0x0020_d0b8 => {
                    // 3: close the PDA template and every UI state.
                    c.cycles += 9;
                    gpa(GP_ACTION).sw(c, 0);
                    PDA_TEMPLATE_CLOSE.call(c, 0x0020_d0c0)?;
                    c.g[V0][0] = gpa(GP_WORLD).lw(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(W_GAME)).lw(c);
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(G_UI)).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(UI_STACK));
                    call_ptr(c, CLOSE_ALL, 0x0020_d0d4)?;
                    via_d0f8 = false;
                }
                0x0020_d0dc => {
                    // 4: request game state 0xe.
                    c.cycles += 7;
                    c.g[V0][0] = gpa(GP_WORLD).lw(c);
                    c.g[A1][0] = 0xe;
                    gpa(GP_ACTION).sw(c, 0);
                    c.g[A2][0] = 2;
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(W_GAME)).lw(c);
                    c.g[A3][0] = 0;
                    call_ptr(c, REQUEST_STATE, 0x0020_d0f8)?;
                }
                0x0020_d0f8 => {}
                t => {
                    c.jump_out(t, 0x0020_cfd0, 0x0020_d10c);
                    return Ok(());
                }
            }
        }
        if via_d0f8 {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PATH_SetVolumePercent (0x3d5508)

/// PATH (interactive music) globals (`ealib`).
mod path_music {
    /// Initialised flag (byte), the master volume (byte), a map selection
    /// in progress (word: the selected map's flags), the selected map.
    pub const INITED: u32 = 0x0044_f42c;
    pub const MASTER_VOLUME: u32 = 0x0044_f42e;
    pub const SELECTED: u32 = 0x0044_f430;
    pub const CURRENT_MAP: u32 = 0x0044_f420;
    /// The four maps (0x928 bytes each): flags (`+0x30`, high nibbles
    /// matched against the mask), loaded (`+0x34`); a map's 24 tracks at
    /// `+0x58`.
    pub const MAPS: u32 = 0x0051_7610;
    pub const MAP_SIZE: u64 = 0x928;
    pub const MAP_FLAGS: u32 = 0x30;
    pub const MAP_LOADED: u32 = 0x34;
    pub const MAP_TRACKS: u32 = 0x58;
    pub const TRACKS: u32 = 24;
}

mod path_callee {
    use super::{ee, Callee};
    pub const SELECT_MAP: Callee = Callee {
        addr: 0x003d_5450,
        f: ee::s14::PATH_SelectMap,
    };
    pub const TRACK_SET_VOLUME: Callee = Callee {
        addr: 0x003d_62c8,
        f: ee::s14::PATH_TrackSetVolume,
    };
}

/// `void PATH_SetVolumePercent(u32 mask, s8 percent)` (0x3d5508): clamp
/// the volume to 0..100 (mask -1 also sets the master volume) and give it
/// to every track whose bit is set in `mask`, of every loaded map whose
/// flag nibbles match the mask. Does nothing before init or while a map is
/// selected.
pub fn path_set_volume_percent(c: &mut Cpu) {
    use path_callee::*;
    use path_music::*;
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x0045_0000;
        frame.save(c, S1, 0x80);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        frame.save(c, S2, 0x70);
        c.g[S1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
        c.g[V1][0] = Addr(INITED).lbu(c);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        let mask = c.g[S2][0];
        // Whether the exit passes the block at 0x3d5668 (+1 cycle).
        let mut tail = true;
        'body: {
            if c.g[V1][0] == 0 {
                break 'body;
            }
            c.cycles += 4;
            c.g[V1][0] = 0x0045_0000;
            c.g[V0][0] = Addr(SELECTED).lw(c);
            if c.g[V0][0] != 0 {
                tail = false;
                break 'body;
            }
            c.cycles += 4;
            c.g[A0][0] = u64::MAX;
            SELECT_MAP.call(c, 0x003d_5564)?;
            c.g[V1][0] = 100;
            if c.g[V0][0] == 0 {
                break 'body;
            }
            // Clamp to 0..100.
            c.cycles += 8;
            c.g[A0][0] = ((c.g[S1][0] as i64) < 101) as u64;
            if c.g[A0][0] == 0 {
                c.g[S1][0] = 100;
            }
            c.g[V0][0] = (-1 < c.g[S1][0] as i64) as u64;
            c.g[V1][0] = u64::MAX;
            if c.g[V0][0] == 0 {
                c.g[S1][0] = 0;
            }
            if mask == u64::MAX {
                c.cycles += 2;
                c.g[V0][0] = 0x0045_0000;
                let vol = c.g[S1][0];
                Addr(MASTER_VOLUME).sb(c, vol);
            }
            c.cycles += 9;
            c.g[V0][0] = 0x0051_0000;
            c.g[A0][0] = 0;
            c.g[S5][0] = sx(MAPS + MAP_LOADED);
            c.g[S4][0] = 0x0045_0000;
            c.g[FP][0] = MAP_SIZE;
            c.g[S7][0] = 0xffff_ffff_f000_0000;
            c.g[S6][0] = 0x0f00_0000;
            c.mult(0, A1, A0, FP);
            loop {
                // Map i = a0 (a1 = i * 0x928).
                let map = MAPS.wrapping_add(c.g[A1][0] as u32);
                c.cycles += 4;
                c.g[V0][0] = sx(map.wrapping_add(MAP_LOADED));
                c.g[V1][0] = Addr(map.wrapping_add(MAP_LOADED)).lw(c);
                c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                let mut skip_tail = true;
                'map: {
                    if c.g[V1][0] == 0 {
                        break 'map;
                    }
                    c.cycles += 8;
                    c.g[V0][0] = sx(MAPS + MAP_FLAGS);
                    c.g[V1][0] = sx(map.wrapping_add(MAP_FLAGS));
                    c.g[V1][0] = Addr(map.wrapping_add(MAP_FLAGS)).lw(c);
                    let flags = c.g[V1][0];
                    let high = flags & c.g[S7][0] & mask;
                    c.g[V0][0] = flags & c.g[S6][0];
                    if high == 0 {
                        break 'map;
                    }
                    c.cycles += 3;
                    c.g[V0][0] &= mask;
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = c.g[S3][0];
                        skip_tail = false;
                        break 'map;
                    }
                    c.cycles += 4;
                    SELECT_MAP.call(c, 0x003d_55fc)?;
                    let selected = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S3][0];
                    if !selected {
                        skip_tail = false;
                        break 'map;
                    }
                    // The selected map's tracks in the mask.
                    c.cycles += 3;
                    c.g[S0][0] = 0;
                    c.g[V0][0] = Addr(CURRENT_MAP).lw(c);
                    loop {
                        let j = c.g[S0][0] as u32;
                        c.cycles += 5;
                        c.g[V1][0] = sx(j << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(j << 2));
                        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(MAP_TRACKS)).lw(c);
                        let track = c.g[A0][0];
                        c.g[V0][0] = sx((mask as u32) >> (j & 31));
                        let mut step_tail = true;
                        if track != 0 {
                            c.cycles += 4;
                            c.g[V0][0] = (c.g[V0][0] ^ 1) & 1;
                            if c.g[V0][0] != 0 {
                                step_tail = false;
                            } else {
                                c.cycles += 2;
                                c.g[A1][0] = c.g[S1][0];
                                TRACK_SET_VOLUME.call(c, 0x003d_563c)?;
                            }
                        }
                        if step_tail {
                            c.cycles += 1;
                        }
                        c.g[S0][0] = sx(j.wrapping_add(1));
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < TRACKS as i64) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[V0][0] = Addr(CURRENT_MAP).lw(c);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_5610);
                        }
                    }
                }
                if skip_tail {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S3][0];
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < 4) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.mult(0, A1, A0, FP);
                if event_due(c) {
                    deliver_event_here(c, 0x003d_55b8);
                }
            }
            // Done: forget the selection.
            c.cycles += 3;
            c.g[V0][0] = 0x0045_0000;
            Addr(CURRENT_MAP).sw(c, 0);
            Addr(SELECTED).sw(c, 0);
        }
        if tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x90);
        c.cycles += 11;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_FlushPendingSpeech (0x2a43b8)

/// Pending speech requests of the audio system (`src/sound/soundsys.cpp`):
/// one word per line, set by game events and played (then cleared) here,
/// at most one per frame in this priority order.
mod pending_speech {
    pub const DJ_20E5: u32 = 0x5768;
    pub const DJ_212B: u32 = 0x5784;
    pub const DJ_20BA: u32 = 0x576c;
    pub const DJ_212C: u32 = 0x577c;
    pub const DJ_212D: u32 = 0x5770;
    pub const RIDER_20D3: u32 = 0x5788;
    /// DJ line 0x20cf with its argument at `+0x5778`.
    pub const DJ_20CF: u32 = 0x5774;
    pub const DJ_20CF_ARG: u32 = 0x5778;
    /// DJ chatter (kind 0, 0, 1).
    pub const CHATTER_A: u32 = 0x574c;
    pub const CHATTER_B: u32 = 0x5750;
    pub const CHATTER_C: u32 = 0x5754;
    /// DJ line 0x20cc with its argument at `+0x575c`.
    pub const DJ_20CC: u32 = 0x5758;
    pub const DJ_20CC_ARG: u32 = 0x575c;
    pub const DJ_20E6: u32 = 0x5740;
    /// Rider line 0x20b9 (independent of the others), for the race
    /// manager's rider number `+0x5748` (riders at race `+0x28`).
    pub const RIDER_20B9: u32 = 0x5744;
    pub const RIDER_20B9_INDEX: u32 = 0x5748;
    pub const RACE_RIDERS: u32 = 0x28;
}

mod speech_callee {
    use super::{ee, Callee};
    macro_rules! callee {
        ($name:ident, $addr:expr, $f:ident) => {
            pub const $name: Callee = Callee {
                addr: $addr,
                f: ee::s08::$f,
            };
        };
    }
    callee!(DJ_SAY_20E5, 0x002a_26f0, SSXAudioSystem_DJSay20E5);
    callee!(DJ_SAY_212B, 0x002a_2ad0, SSXAudioSystem_DJSay212B);
    callee!(DJ_SAY_20BA, 0x002a_2638, SSXAudioSystem_DJSay20BA);
    callee!(DJ_SAY_212C, 0x002a_2b88, SSXAudioSystem_DJSay212C);
    callee!(DJ_SAY_212D, 0x002a_2c30, SSXAudioSystem_DJSay212D);
    callee!(RIDER_SAY_20D3, 0x002a_1138, SSXAudioSystem_RiderSay20D3);
    callee!(DJ_SAY_20CF, 0x002a_2860, SSXAudioSystem_DJSay20CF);
    callee!(PLAY_DJ_CHATTER, 0x002a_2e50, SSXAudioSystem_PlayDJChatter);
    callee!(DJ_SAY_20CC, 0x002a_2568, SSXAudioSystem_DJSay20CC_2);
    callee!(DJ_SAY_20E6, 0x002a_27d8, SSXAudioSystem_DJSay20E6);
    callee!(RIDER_SAY_20B9, 0x002a_1400, SSXAudioSystem_RiderSay20B9);
    /// `SSXAudioSystem_GetRaceMan` (through the function table).
    pub const GET_RACE_MAN: u32 = 0x0028_b1d8;
}

/// `void SSXAudioSystem_FlushPendingSpeech(SSXAudioSystem* self)`
/// (0x2a43b8): play the first pending DJ/rider line (in priority order) and
/// clear its request; then, independently, the pending rider line 0x20b9
/// for the race manager's rider (if that rider exists).
pub fn ssx_audio_system_flush_pending_speech(c: &mut Cpu) {
    use pending_speech::*;
    use speech_callee::*;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        let at = |off: u32| Addr(this.wrapping_add(off));
        let me = c.g[S0][0];
        // Play one line: the call, then clear its request (through s0 as
        // the original does, not a saved copy).
        let say = |c: &mut Cpu, callee: Callee, ret: u32, flag: u32| -> Flow {
            callee.call(c, ret)?;
            let s0 = c.g[S0][0] as u32;
            Addr(s0.wrapping_add(flag)).sw(c, 0);
            Ok(())
        };
        c.g[V0][0] = at(DJ_20E5).lw(c);
        c.g[A1][0] = 0;
        let played = 'played: {
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A2][0] = 0;
                say(c, DJ_SAY_20E5, 0x002a_43dc, DJ_20E5)?;
                break 'played true;
            }
            c.cycles += 3;
            c.g[V0][0] = at(DJ_212B).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = me;
                say(c, DJ_SAY_212B, 0x002a_43f8, DJ_212B)?;
                break 'played true;
            }
            // Lines whose word the previous check loaded (delay slot).
            for (flag, callee, ret) in [
                (DJ_20BA, DJ_SAY_20BA, 0x002a_4410),
                (DJ_212C, DJ_SAY_212C, 0x002a_4428),
                (DJ_212D, DJ_SAY_212D, 0x002a_4440),
            ] {
                c.g[V0][0] = at(flag).lw(c);
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    c.g[A0][0] = me;
                    say(c, callee, ret, flag)?;
                    break 'played true;
                }
            }
            c.g[V0][0] = at(RIDER_20D3).lw(c);
            c.cycles += 2;
            c.g[A0][0] = me;
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                say(c, RIDER_SAY_20D3, 0x002a_445c, RIDER_20D3)?;
                break 'played true;
            }
            c.cycles += 3;
            c.g[V0][0] = at(DJ_20CF).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                c.g[A1][0] = at(DJ_20CF_ARG).lw(c);
                c.g[A0][0] = me;
                say(c, DJ_SAY_20CF, 0x002a_447c, DJ_20CF)?;
                break 'played true;
            }
            c.g[V0][0] = at(CHATTER_A).lw(c);
            c.cycles += 2;
            c.g[A0][0] = me;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = 0;
                say(c, PLAY_DJ_CHATTER, 0x002a_4494, CHATTER_A)?;
                break 'played true;
            }
            c.cycles += 3;
            c.g[V0][0] = at(CHATTER_B).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = 0;
                say(c, PLAY_DJ_CHATTER, 0x002a_44b0, CHATTER_B)?;
                break 'played true;
            }
            c.g[V0][0] = at(CHATTER_C).lw(c);
            c.cycles += 2;
            c.g[A0][0] = me;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = 1;
                say(c, PLAY_DJ_CHATTER, 0x002a_44c8, CHATTER_C)?;
                break 'played true;
            }
            c.cycles += 3;
            c.g[V0][0] = at(DJ_20CC).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                c.g[A1][0] = at(DJ_20CC_ARG).lw(c);
                c.g[A0][0] = me;
                say(c, DJ_SAY_20CC, 0x002a_44e8, DJ_20CC)?;
                break 'played true;
            }
            c.g[V0][0] = at(DJ_20E6).lw(c);
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = me;
                say(c, DJ_SAY_20E6, 0x002a_4500, DJ_20E6)?;
                break 'played true;
            }
            false
        };
        // A line played: the block at 0x2a4504.
        if played {
            c.cycles += 1;
        }
        // From here on `self` is s0 as the original reads it after the call.
        let this = c.g[S0][0] as u32;
        let at = |off: u32| Addr(this.wrapping_add(off));
        c.g[V0][0] = at(RIDER_20B9).lw(c);

        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            call_ptr(c, GET_RACE_MAN, 0x002a_4518)?;
            c.g[V1][0] = at(RIDER_20B9_INDEX).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(RACE_RIDERS)).lw(c);
            if c.g[A1][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                RIDER_SAY_20B9.call(c, 0x002a_4538)?;
            }
            let s0 = c.g[S0][0] as u32;
            Addr(s0.wrapping_add(RIDER_20B9)).sw(c, 0);
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_UpdateObjectLoops (0x297fa0)

/// Object loop slots of the audio system (`src/sound/soundsys.cpp`): 30
/// slots of 0x30 bytes at `+0x5a00`.
mod object_loop {
    pub const SLOTS: u32 = 0x5a00;
    pub const SLOT_SIZE: u64 = 0x30;
    pub const COUNT: i64 = 30;
    /// Slot: active (`+0`, 1), the sound object (`+4`; its scene node at
    /// `+0xc`, its own position at `+0x40`), the node's bone (`+8`, -1 the
    /// node itself), the emitter position (`+0x10`, a quadword), the loop
    /// handle (`+0x24`).
    pub const ACTIVE: u32 = 0x0;
    pub const OBJECT: u32 = 0x4;
    pub const BONE: u32 = 0x8;
    pub const POSITION: u32 = 0x10;
    pub const HANDLE: u32 = 0x24;
    pub const OBJ_NODE: u32 = 0xc;
    pub const OBJ_POSITION: u32 = 0x40;
    /// Node vtable (`+0xc`) slots: the world matrix (`+0xc0`), "has bones"
    /// (`+0xd0`), a bone's matrix (`+0xe8`); matrices hold the translation
    /// at `+0x30`.
    pub const NODE_VTABLE: u32 = 0xc;
    pub const VT_MATRIX: u32 = 0xc0;
    pub const VT_HAS_BONES: u32 = 0xd0;
    pub const VT_BONE_MATRIX: u32 = 0xe8;
    pub const TRANSLATION: u32 = 0x30;
    /// The audio system's channel table (`+0x118`; the bank monitor at
    /// `*table + 0x1d8`).
    pub const CHANNELS: u32 = 0x118;
    pub const BANK_MONITOR: u32 = 0x1d8;
}

mod object_loop_callee {
    use super::{ee, Callee};
    pub const STOP_HANDLE: Callee = Callee {
        addr: 0x002a_d5f0,
        f: ee::s08::BankMonitor_StopHandle,
    };
}

/// `void SSXAudioSystem_UpdateObjectLoops(SSXAudioSystem* self)`
/// (0x297fa0): for every active object loop, follow its emitter: a sound
/// object whose scene node is gone stops the loop; otherwise the position
/// is the node's translation (bone -1), the bone's translation (node with
/// bones) or the object's own position.
pub fn ssx_audio_system_update_object_loops(c: &mut Cpu) {
    use object_loop::*;
    use object_loop_callee::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[V0][0] = SLOT_SIZE;
        let this = c.g[S2][0] as u32;
        loop {
            // Slot i = s1 at s0 - 0x5a00.
            c.cycles += 6;
            c.g[A0][0] = 1;
            c.mult(0, V1, S1, V0);
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(this));
            let slot = (c.g[S0][0] as u32).wrapping_add(SLOTS);
            let at = |off: u32| Addr(slot.wrapping_add(off));
            c.g[V1][0] = at(ACTIVE).lw(c);
            if c.g[V1][0] == 1 {
                c.cycles += 4;
                c.g[V0][0] = at(OBJECT).lw(c);
                c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(OBJ_NODE)).lw(c);
                let node = c.g[A1][0] as u32;
                if node == 0 {
                    // The node is gone: stop the loop.
                    c.cycles += 9;
                    c.g[V0][0] = Addr(this.wrapping_add(CHANNELS)).lw(c);
                    c.g[A2][0] = 1;
                    c.g[A1][0] = at(HANDLE).lw(c);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                    abi::set_fpr(c, 12, FpuF32::ZERO);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(BANK_MONITOR));
                    STOP_HANDLE.call(c, 0x0029_8004)?;
                    at(ACTIVE).sw(c, 0);
                } else {
                    c.g[V1][0] = at(BONE).lw(c);
                    c.cycles += 3;
                    let bone = c.g[V1][0];
                    c.g[V0][0] = u64::MAX;
                    c.g[V1][0] = Addr(node.wrapping_add(NODE_VTABLE)).lw(c);
                    let vt = c.g[V1][0] as u32;
                    // Where the translation is read (`None`: the object's own
                    // position, through v1 = the object).
                    let matrix = if bone == u64::MAX {
                        c.cycles += 6;
                        virtual_call(c, node, vt, VT_MATRIX, 0x0029_8028)?;
                        true
                    } else {
                        c.cycles += 6;
                        virtual_call(c, node, vt, VT_HAS_BONES, 0x0029_8040)?;
                        if c.g[V0][0] == 0 {
                            c.g[V1][0] = at(OBJECT).lw(c);
                            false
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = at(OBJECT).lw(c);
                            c.g[A1][0] = at(BONE).lw(c);
                            c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(OBJ_NODE)).lw(c);
                            let node = c.g[A2][0] as u32;
                            c.g[V1][0] = Addr(node.wrapping_add(NODE_VTABLE)).lw(c);
                            let vt = c.g[V1][0] as u32;
                            virtual_call(c, node, vt, VT_BONE_MATRIX, 0x0029_8068)?;
                            true
                        }
                    };
                    let sp0 = sp(c);
                    if matrix {
                        c.lq(V1, (c.g[V0][0] as u32).wrapping_add(TRANSLATION));
                        c.cycles += 3;
                        c.sq(V1, at(POSITION).0);
                        c.sq(V1, sp0);
                    } else {
                        c.cycles += 3;
                        c.lq(V0, (c.g[V1][0] as u32).wrapping_add(OBJ_POSITION));
                        c.sq(V0, at(POSITION).0);
                        c.sq(V0, sp0);
                    }
                }
                c.cycles += 1;
            }
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < COUNT) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = SLOT_SIZE;
            if event_due(c) {
                deliver_event_here(c, 0x0029_7fc0);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// A virtual call `node->vt[slot](node + delta)` (`{s16 delta; fn}` at
/// `vt + slot`), as the original sets it up: a0 = node + delta, v0 = fn.
fn virtual_call(c: &mut Cpu, node: u32, vt: u32, slot: u32, ret: u32) -> Flow {
    c.g[A0][0] = Addr(vt.wrapping_add(slot)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(slot + 4)).lw(c);
    let f = c.g[V0][0] as u32;
    c.g[A0][0] = sx(node.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, f, ret)
}

// ---------------------------------------------------------------------------
// Nash_ClassifyEffects (0x22a4a8)

/// Nash (effect) registration (`src/main/ssxapp.cpp`).
mod nash {
    /// The effect-node count (`+0x7bc0`) and array (`+0x7bc4`) of type-8
    /// nodes; the registration head (`+0x7ab8`).
    pub const NODE_COUNT: u32 = 0x7bc0;
    pub const NODES: u32 = 0x7bc4;
    pub const HEAD: u32 = 0x7ab8;
    /// An item (8 bytes): its model (`+0`, node list at `+0x28`) and an
    /// argument (`+4`). A node: next (`+0`), type (`+8`: 7 an effect, 8 a
    /// plain node).
    pub const ITEM_SIZE: u32 = 8;
    pub const ITEM_ARG: u32 = 4;
    pub const MODEL_NODES: u32 = 0x28;
    pub const NODE_TYPE: u32 = 8;
    pub const EFFECT: u64 = 7;
    pub const PLAIN: u64 = 8;
}

mod nash_callee {
    use super::{ee, Callee};
    pub const CLASSIFY_EFFECT: Callee = Callee {
        addr: 0x0022_a270,
        f: ee::s05::Nash_ClassifyEffect,
    };
}

/// `void Nash_ClassifyEffects(Nash* self, Item* items, int n)` (0x22a4a8):
/// walk the node lists of `n` items: effect nodes (type 7) are classified
/// with the item's argument, plain nodes (type 8) are appended to the node
/// array. Then, if anything is registered, classify a null effect (flush).
pub fn nash_classify_effects(c: &mut Cpu) {
    use nash::*;
    use nash_callee::*;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x80);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[S1][0] as u32;
        if (c.g[A2][0] as i64) > 0 {
            c.cycles += 1;
            loop {
                // Item at s2; s5 = items left after it.
                let item = c.g[S2][0] as u32;
                c.cycles += 5;
                c.g[V0][0] = Addr(item).lw(c);
                c.g[S5][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(MODEL_NODES)).lw(c);
                c.g[S4][0] = sx(item.wrapping_add(ITEM_SIZE));
                if c.g[S0][0] != 0 {
                    c.cycles += 5;
                    c.g[S7][0] = EFFECT;
                    c.g[S6][0] = PLAIN;
                    c.g[S3][0] = sx(this.wrapping_add(NODES));
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(NODE_TYPE)).lw(c);
                    loop {
                        let node = c.g[S0][0] as u32;
                        let kind = c.g[V0][0];
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S1][0];
                        if kind == EFFECT {
                            c.cycles += 5;
                            c.g[A2][0] = Addr(item.wrapping_add(ITEM_ARG)).lw(c);
                            c.g[A1][0] = c.g[S0][0];
                            CLASSIFY_EFFECT.call(c, 0x0022_a51c)?;
                        } else {
                            c.cycles += 2;
                            if kind == PLAIN {
                                c.cycles += 7;
                                c.g[V1][0] = Addr(this.wrapping_add(NODE_COUNT)).lw(c);
                                let n = c.g[V1][0] as u32;
                                c.g[V0][0] = sx(this.wrapping_add(NODES).wrapping_add(n << 2));
                                c.g[V1][0] = sx(n.wrapping_add(1));
                                Addr(c.g[V0][0] as u32).sw(c, node as i32 as i64 as u64);
                                let count = c.g[V1][0];
                                Addr(this.wrapping_add(NODE_COUNT)).sw(c, count);
                            }
                        }
                        c.g[S0][0] = Addr(node).lw(c);
                        c.cycles += 2;
                        if c.g[S0][0] == 0 {
                            break;
                        }
                        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(NODE_TYPE)).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x0022_a508);
                        }
                    }
                }
                c.cycles += 3;
                c.g[A2][0] = c.g[S5][0];
                c.g[S2][0] = c.g[S4][0];
                if (c.g[A2][0] as i64) <= 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_a4e0);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(this.wrapping_add(HEAD)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            CLASSIFY_EFFECT.call(c, 0x0022_a574)?;
        }
        c.cycles += 11;
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

// ---------------------------------------------------------------------------
// PadQueue_Poll (0x326b88)

/// The pad input queue (`src/input/inputparse.cpp`): 30 frames of 0x190
/// bytes (one 0x64-byte record per device: result word, then 0x18 bytes
/// of data) from `+0`, the write index (`+0x2ee4`), the device count
/// (`+0x2ee8`) and devices (`+0x2eec`).
mod pad_queue {
    pub const TAIL: u32 = 0x2ee4;
    pub const COUNT: u32 = 0x2ee8;
    pub const DEVICES: u32 = 0x2eec;
    pub const FRAME_SIZE: u64 = 0x190;
    pub const RECORD_SIZE: u32 = 0x64;
    pub const READ_SIZE: u64 = 0x18;
    pub const FRAMES: u64 = 30;
    /// Device vtable: `read(data, size)` at `+0x10`.
    pub const VT_READ: u32 = 0x10;
}

/// `void PadQueue_Poll(PadQueue* q)` (0x326b88): read every device into
/// the queue's current frame (`record.result = dev->read(record.data,
/// 0x18)`), then advance the write index modulo 30.
pub fn pad_queue_poll(c: &mut Cpu) {
    use pad_queue::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        let q = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(q.wrapping_add(COUNT)).lw(c);
        c.g[S3][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[S4][0] = FRAME_SIZE;
            c.g[S2][0] = 0;
            c.g[S1][0] = sx(q.wrapping_add(DEVICES));
            loop {
                // Device s3 (s1 its pointer, s2 its record offset).
                c.cycles += 23;
                c.g[A1][0] = Addr(q.wrapping_add(TAIL)).lw(c);
                c.g[A2][0] = READ_SIZE;
                c.g[A3][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.mult(0, V0, A1, S4);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
                let dev = c.g[A3][0] as u32;
                c.g[V1][0] = Addr(dev).lw(c);
                let vt = c.g[V1][0] as u32;
                c.g[A0][0] = Addr(vt.wrapping_add(VT_READ)).lh(c);
                let record = (c.g[V0][0] as u32)
                    .wrapping_add(q)
                    .wrapping_add(c.g[S2][0] as u32);
                c.g[A1][0] = sx(record);
                c.g[V0][0] = Addr(vt.wrapping_add(VT_READ + 4)).lw(c);
                c.g[A0][0] = sx(dev.wrapping_add(c.g[A0][0] as u32));
                let read = c.g[V0][0] as u32;
                c.g[A1][0] = sx(record.wrapping_add(4));
                call_ptr(c, read, 0x0032_6bf8)?;
                // The record of the (re-read) current frame gets the result.
                c.g[V1][0] = Addr(q.wrapping_add(TAIL)).lw(c);
                c.mult(0, A0, V1, S4);
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[V1][0] = sx(q.wrapping_add(c.g[V1][0] as u32));
                let result = c.g[V0][0];
                Addr(c.g[V1][0] as u32).sw(c, result);
                c.g[V0][0] = Addr(q.wrapping_add(COUNT)).lw(c);
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(RECORD_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_6bc0);
                }
            }
        }
        c.cycles += 4;
        c.g[V1][0] = Addr(q.wrapping_add(TAIL)).lw(c);
        c.g[V0][0] = FRAMES;
        c.cycles += 12;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.divu(0, V1, V0);
        c.g[A0][0] = c.hi[0];
        let tail = c.g[A0][0];
        Addr(q.wrapping_add(TAIL)).sw(c, tail);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x50);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ssx3_ee as ee;

    /// cStreamMan_Update over its paths: inactive; a pending parse resumed
    /// (no resource, a resource allocated, or the allocation failing so the
    /// update stops); throttled waits (ticks counted down or still waiting);
    /// 0-3 chunks of junk (decompress to nothing) or of a RefPack stream
    /// holding one resource header (allocated, or failing so the update
    /// marks the parse pending); a throttled run stopping after one chunk;
    /// the stream file idle or not (and an invalid handle). The real
    /// StreamFile/ParseChunks/AllocResource callees run; their mutex kernel
    /// calls are fakes (some unwind).
    #[test]
    fn c_stream_man_update_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA, GAME_GP};
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        check(ee::s13::f_003a7098__generated, c_stream_man_update, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3a_7098, 6);
            c.pc = 0x003a_7098;
            let this = DATA;
            let cache = DATA + 0x1000;
            let (table, res, refc) = (DATA + 0x1400, DATA + 0x1500, DATA + 0x1600);
            let (allocs, alloc, block) = (DATA + 0x1700, DATA + 0x1800, DATA + 0x1900);
            let (queue, queue_items) = (DATA + 0x1a00, DATA + 0x1b00);
            let (h, man, ring) = (DATA + 0x2000, DATA + 0x2100, DATA + 0x2800);
            let buffer = DATA + 0x3800;
            let clock = DATA + 0x3c00;
            for a in (this..this + 0x100).step_by(4) {
                w(c, a, 0);
            }
            // The manager.
            w(c, this, cache);
            w(c, this + 0x88, if seed % 17 == 0 { 0 } else { h });
            w(c, this + 0x8c, buffer);
            w(c, this + 0x90, (seed % 11 != 0) as u32);
            let hull = r.next_u32() % 4;
            w(c, this + 0x94, hull);
            w(c, this + 0x98, r.next_u32() % 2);
            // A pending parse: an empty buffer and a resource of type 1 to
            // allocate (size 0: nothing to allocate).
            w(c, this + 0xac, (seed % 3 == 0) as u32);
            w(c, this + 0xb8, 1);
            w(c, this + 0xbc, 0);
            w(c, this + 0xc0, if seed % 2 == 0 { 0 } else { 0x40 });
            // Throttling: the audio clock and the wait.
            let throttled = seed % 5 == 1 || seed % 5 == 2;
            w(c, this + 0xd4, throttled as u32);
            w(c, this + 0xdc, (1.0f32 + (r.next_u32() % 4) as f32).to_bits());
            let ticks = r.next_u32() % 8;
            w(c, GAME_GP + 0x2a74, clock);
            w(c, clock + 0x18, ticks);
            w(c, this + 0xe4, if seed % 4 == 0 { ticks } else { r.next_u32() % 8 });
            w(c, this + 0xe0, ((r.next_u32() % 6) as f32 - 2.0).to_bits());
            // The world cache: resource table -> refcounts; allocator for
            // type 1 -> one block whose base 0 makes the allocation fail.
            w(c, cache + 8, table);
            w(c, table, res);
            w(c, res + 4 + 8, refc);
            for a in (refc..refc + 0x10).step_by(4) {
                w(c, a, 0);
            }
            w(c, cache + 0xc, allocs);
            w(c, allocs + 0x3c, alloc);
            w(c, alloc + 8, 1);
            w(c, alloc + 0x10, block);
            let base = if seed % 7 == 3 { 0 } else { DATA + 0x5000 };
            w(c, block, base);
            w(c, block + 4, 0x1_0000);
            w(c, block + 8, 0);
            w(c, block + 0xc, 0);
            for i in 0..4 {
                w(c, cache + 0x3f8 + 0x18 * i, queue);
            }
            w(c, queue + 8, 0);
            w(c, queue + 0xc, queue_items);
            // The stream file: handle -> manager ("STRM", mutex, ring).
            w(c, h, man);
            w(c, man, 0x4d52_5453);
            w(c, man + 8, r.next_u32() % 4);
            w(c, man + 0xc, r.next_u32() % 3);
            w(c, man + 0x10, r.next_u32() % 64);
            w(c, man + 0x30, ring);
            // Chunks lie past the ring end: ReleaseChunk leaves them alone.
            w(c, man + 0x34, ring + 4);
            w(c, man + 0x38, (seed % 6 == 5) as u32);
            let tag = 1 + r.next_u32() % 255;
            w(c, h + 4, tag);
            let k = r.next_u32() % 4;
            let mut at = ring;
            for _ in 0..k {
                let size = 0x20;
                w(c, at, 0);
                w(c, at + 4, tag << 24 | size);
                let p = at + 8;
                if r.next_u32() % 2 == 0 {
                    // RefPack: 8 literal bytes = a resource header (type 1,
                    // size 0x40, id 0).
                    let stream: [u8; 14] =
                        [0x10, 0xfb, 0, 0, 8, 0xe1, 1, 0x40, 0, 0, 0, 0, 0, 0];
                    for (i, &b) in stream.iter().enumerate() {
                        c.bus.write8(p + i as u32, b);
                    }
                    c.bus.write8(p + 14, 0xfc);
                } else {
                    w(c, p, r.next_u32());
                    c.bus.write8(p + 1, 0);
                }
                at += size;
            }
            w(c, h + 8, 0x20 * k);
            w(c, h + 0xc, ring);
            c.g[A0][0] = sx(this);
        });
    }

    /// BezierPatchCache_ReleaseUnused: three levels of 0-7 entries with
    /// flags 0, 4, 8, both, or other non-zero bits, slots -1 or >= 0.
    #[test]
    fn bezier_patch_cache_release_unused_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        check(
            ee::s12::f_00374b38__generated,
            bezier_patch_cache_release_unused,
            300,
            &|c, seed| {
                let mut r = Rng(0x374b_3800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0037_4b38;
                let cache = DATA;
                let owners = DATA + 0x1000;
                for lv in 0..3u32 {
                    let n = if seed % 9 == 0 { 0 } else { r.next_u32() % 8 };
                    let entries = DATA + 0x400 + 0x100 * lv;
                    let (free, slots) = (DATA + 0x800 + 0x80 * lv, DATA + 0xa00 + 0x80 * lv);
                    c.bus.write32(cache + 4 * lv, n);
                    c.bus.write32(cache + 0x18 + 4 * lv, entries);
                    c.bus.write32(cache + 0x30 + 4 * lv, free);
                    c.bus.write32(cache + 0x3c + 4 * lv, r.next_u32() % 4);
                    c.bus.write32(cache + 0x1cc + 4 * lv, slots);
                    c.bus.write32(cache + 0x1d8 + 4 * lv, r.next_u32() % 4);
                    for j in 0..n {
                        let e = entries + 0xc * j;
                        let flags = match r.next_u32() % 6 {
                            0 => 0,
                            1 => 4,
                            2 => 8,
                            3 => 12,
                            4 => r.next_u32() & !0xc,
                            _ => r.next_u32(),
                        };
                        c.bus.write32(e, owners + 0x10 * (r.next_u32() % 16));
                        c.bus.write32(e + 4, flags);
                        let slot = if r.next_u32() % 2 == 0 {
                            u32::MAX
                        } else {
                            r.next_u32() % 64
                        };
                        c.bus.write32(e + 8, slot);
                    }
                }
                c.g[A0][0] = sx(cache);
            },
        );
    }

    /// RiderBase_MatchCheatCode: every state (-2, -1, 0, 1, 2), flag bytes
    /// with and without flag 1 and the event bits (0x60 exactly too), counts
    /// that match the code's step or not, the last step.
    #[test]
    fn rider_base_match_cheat_code_matches() {
        use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
        check(
            ee::s09::f_002f6910__generated,
            rider_base_match_cheat_code,
            400,
            &|c, seed| {
                let mut r = Rng(0x2f69_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x002f_6910;
                c.g[GP][0] = GAME_GP as u64;
                let code = [3u8, 1, 4, 1, 5, 9, 2, 6, 5];
                for (i, &n) in code.iter().enumerate() {
                    c.bus.write8(0x0044_5e30 + i as u32, n);
                }
                let this = DATA;
                let state = [0xffu8, 0, 1, 2, 0xfe][(seed % 5) as usize];
                let step = if seed % 3 == 0 { 8 } else { (r.next_u32() % 9) as u8 };
                let count = if seed % 2 == 0 {
                    code[step as usize]
                } else {
                    (r.next_u32() % 10) as u8
                };
                let flags = match (seed / 5) % 6 {
                    0 => 0x60,
                    1 => 0x60 | (r.next_u32() as u8 & 0x8e),
                    2 => r.next_u32() as u8 & 0x8e,
                    3 => r.next_u32() as u8 & !1,
                    _ => r.next_u32() as u8,
                };
                c.bus.write32(this, r.next_u32());
                c.bus.write8(this + 4, state);
                c.bus.write8(this + 5, step);
                c.bus.write8(this + 6, count);
                c.bus.write8(this + 7, flags);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// GameUI_Update: actions 0-6 and -1 (jump table written in), no UI
    /// manager or a state stack with none / an inactive / an active state
    /// (pause menu id 8 or other), game mode 2 or not, online or not. The
    /// real callees run; the BE interface lookup is a fake.
    #[test]
    #[ignore = "fixture unbounded: real callees jump through jump tables the test machine never loads; port parked, not installed"]
    fn game_ui_update_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA, GAME_GP};
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        check(ee::s05::f_0020cfd0, game_ui_update, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x20_cfd0, 3);
            c.pc = 0x0020_cfd0;
            let gp = GAME_GP;
            let (world, game, mgr) = (DATA, DATA + 0x1000, DATA + 0x2000);
            let (replay, hud) = (DATA + 0x3000, DATA + 0x3800);
            let (sentinel, n1, n2) = (DATA + 0x4000, DATA + 0x4100, DATA + 0x4200);
            // BELibrary dummy (GameUI_HandleMenuResult gets its instance).
            w(c, gp - 0x1f04, DATA + 0x3_0000);
            w(c, gp - 0x848, world);
            w(c, gp - 0x9e4, 0);
            let action = [0u32, 1, 2, 3, 4, 5, 6, u32::MAX][(seed % 8) as usize];
            w(c, gp - 0x9f4, action);
            for (i, t) in [0x0020_d0f8u32, 0x0020_d02c, 0x0020_d03c, 0x0020_d0b8, 0x0020_d0dc]
                .into_iter()
                .enumerate()
            {
                w(c, 0x0047_1ae0 + 4 * i as u32, t);
            }
            w(c, 0x0053_4b30, (seed % 3 != 0) as u32);
            w(c, world + 0x84, game);
            w(c, world + 0x78, r.next_u32());
            w(c, world + 0xc0, 0);
            w(c, game + 0x28, replay);
            w(c, replay + 0x61c, 0);
            w(c, game + 0x200, hud);
            w(c, game + 0x214, if seed % 5 == 0 { 1 } else { 2 });
            w(c, game + 0x48, if seed % 13 == 0 { 0xffff_ffe8 } else { mgr });
            // The UI state stack at mgr+0x18: nodes {prev, next, .., flags
            // +0x1c (bit 5 active), id +0x98}, ending at a sentinel.
            w(c, sentinel, sentinel);
            w(c, sentinel + 4, sentinel);
            let id = if seed % 4 == 3 { 3 } else { 8 };
            for (n, next) in [(n1, n2), (n2, sentinel)] {
                w(c, n, mgr + 0x18);
                w(c, n + 4, next);
                w(c, n + 0x98, id);
            }
            w(c, n1 + 0x1c, if seed % 6 == 1 { 0x20 } else { r.next_u32() & !0x20 });
            w(c, n2 + 0x1c, r.next_u32() | if seed % 9 == 2 { 0 } else { 0x20 });
            w(c, mgr + 0x18 + 4, match (seed / 3) % 3 {
                0 => sentinel,
                1 => n2,
                _ => n1,
            });
        });
    }

    /// PATH_SetVolumePercent: not initialised, a selection in progress, or
    /// the four maps (loaded or not, flag nibbles matching the mask or not)
    /// with 24 tracks (present or not); masks -1 or random, volumes in and
    /// out of 0..100.
    #[test]
    fn path_set_volume_percent_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        check(ee::s14::f_003d5508__generated, path_set_volume_percent, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3d_5508, 0);
            c.pc = 0x003d_5508;
            c.bus.write8(0x0044_f42c, (seed % 11 != 0) as u8);
            c.bus.write8(0x0044_f42e, r.next_u32() as u8);
            w(c, 0x0044_f430, if seed % 13 == 1 { r.next_u32() | 1 } else { 0 });
            w(c, 0x0044_f420, r.next_u32());
            let mask = if seed % 4 == 0 { u32::MAX } else { r.next_u32() };
            for i in 0..4u32 {
                let map = 0x0051_7610 + 0x928 * i;
                w(c, map + 0x34, (r.next_u32() % 4 != 0) as u32);
                let flags = match r.next_u32() % 3 {
                    0 => r.next_u32(),
                    1 => mask & 0xff00_0000 | 1,
                    _ => mask & 0xf000_0000,
                };
                w(c, map + 0x30, flags);
                for j in 0..24u32 {
                    let t = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x40 * (j + 24 * i)
                    };
                    w(c, map + 0x58 + 4 * j, t);
                }
            }
            c.g[A0][0] = sx(mask);
            c.g[A1][0] = match seed % 5 {
                0 => 200,
                1 => sx(0xffff_ff80),
                _ => r.next_u32() as u64 | (r.next_u32() as u64) << 32,
            };
        });
    }

    /// SSXAudioSystem_FlushPendingSpeech: no request, each single request,
    /// several at once (the first in priority order plays), and the rider
    /// line 0x20b9 with its rider present or not. The real speech callees
    /// run on the zeroed audio system of the DJ-line tests.
    #[test]
    fn ssx_audio_system_flush_pending_speech_matches() {
        use crate::difftest::{check, DATA};
        use crate::game::fe::feasyncfile::tests::W_6;
        use crate::game::sound::soundsys::tests::dj_setup;
        const FLAGS: [u32; 13] = [
            0x5768, 0x5784, 0x576c, 0x577c, 0x5770, 0x5788, 0x5774, 0x574c, 0x5750,
            0x5754, 0x5758, 0x5740, 0,
        ];
        check(
            ee::s08::f_002a43b8__generated,
            ssx_audio_system_flush_pending_speech,
            150,
            &|c, seed| {
                dj_setup(c, seed, 0x2a_43b8);
                c.pc = 0x002a_43b8;
                let this = DATA;
                let first = FLAGS[(seed % 13) as usize];
                if first != 0 {
                    c.bus.write32(this + first, 1 + seed);
                    if seed % 3 == 0 {
                        // A lower-priority request too.
                        c.bus.write32(this + 0x5740, 1);
                    }
                }
                c.bus.write32(this + 0x5778, seed % 7);
                c.bus.write32(this + 0x575c, seed % 5);
                c.bus.write32(this + 0x5744, (seed % 4 != 0) as u32);
                let rider = seed % 4;
                c.bus.write32(this + 0x5748, rider);
                let race = W_6 + 0x2000;
                c.bus.write32(
                    race + 0x28 + 4 * rider,
                    if seed % 2 == 0 { 0 } else { DATA + 0x2_0000 },
                );
            },
        );
    }

    /// SSXAudioSystem_UpdateObjectLoops: 30 slots, active or not; a gone
    /// node (the loop stops through the real BankMonitor_StopHandle on the
    /// StopAllObjectLoops fixture), the node's matrix (bone -1), a bone
    /// matrix, or no bones (a real leaf returning 0) and the object's own
    /// position. The node methods are fakes (some unwind).
    #[test]
    fn ssx_audio_system_update_object_loops_matches() {
        use crate::difftest::fakes::{FAKE, LOG_RESULTS};
        use crate::difftest::{check, fill, DATA};
        use crate::game::main::gamestate::tests::base_3;
        const RETURN_ZERO: u32 = 0x0014_06c0;
        check(
            ee::s08::f_00297fa0__generated,
            ssx_audio_system_update_object_loops,
            120,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x29_7fa0, 8);
                c.pc = 0x0029_7fa0;
                let s = DATA + 0x100;
                let chans = DATA + 0x800;
                let gain = DATA + 0x840;
                c.g[A0][0] = sx(s);
                c.bus.write32(s + 0x118, chans);
                c.bus.write32(chans, gain);
                // Two node vtables: with bones (fake) and without (0).
                let (vt_bones, vt_flat) = (DATA + 0x6800, DATA + 0x6900);
                for (vt, has_bones) in [(vt_bones, FAKE + 0x10), (vt_flat, RETURN_ZERO)] {
                    for (off, f) in [(0xc0, FAKE), (0xd0, has_bones), (0xe8, FAKE + 0x20)] {
                        c.bus.write16(vt + off, (r.next_u32() as i16 as i32 & !3) as u16);
                        c.bus.write32(vt + off + 4, f);
                    }
                }
                // Fake matrices with a translation.
                for k in 0..4u32 {
                    let m = DATA + 0x7000 + 0x100 * k;
                    c.bus.write32(LOG_RESULTS + 4 * k, m);
                    fill(c, &mut r, m + 0x30, 4);
                }
                for i in 0..30u32 {
                    let slot = s + 0x5a00 + 0x30 * i;
                    let obj = DATA + 0x2000 + 0x80 * i;
                    let node = DATA + 0x4000 + 0x40 * i;
                    let active = if (i + seed) % 3 == 0 { 1 } else { r.next_u32() % 3 };
                    c.bus.write32(slot, active);
                    c.bus.write32(slot + 4, obj);
                    c.bus.write32(
                        slot + 8,
                        if r.next_u32() % 2 == 0 { u32::MAX } else { r.next_u32() % 20 },
                    );
                    c.bus.write32(slot + 0x24, r.next_u32() % 8);
                    c.bus.write32(obj + 0xc, if r.next_u32() % 5 == 0 { 0 } else { node });
                    fill(c, &mut r, obj + 0x40, 4);
                    c.bus.write32(node + 0xc, if r.next_u32() % 2 == 0 { vt_bones } else { vt_flat });
                }
            },
        );
    }

    /// Nash_ClassifyEffects: 0-3 items whose models have no nodes or lists
    /// of effect (7), plain (8) and other nodes; the registration head set
    /// or not. Nash_ClassifyEffect runs for real on the fixture of its own
    /// test (effect-kind table, VI1, small list counts).
    #[test]
    fn nash_classify_effects_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, put_words, DATA};
        check(ee::s05::f_0022a4a8__generated, nash_classify_effects, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x22_a4a8, 2);
            c.pc = 0x0022_a4a8;
            let this = DATA;
            fill(c, &mut r, this, 0x400);
            c.bus.plat.write_ctrl(1, seed % 5);
            put_words(
                c,
                0x0047_b380,
                &[0x0022_a30c, 0x0022_a358, 0x0022_a358, 0x0022_a30c, 0x0022_a334],
            );
            c.bus.write32(this + 0x7ab8, if seed % 2 == 0 { 0 } else { this });
            c.bus.write32(this + 0x78b4, r.next_u32() & 3);
            c.bus.write32(this + 0x7abc, r.next_u32() & 3);
            c.bus.write32(this + 0x7bc0, r.next_u32() & 3);
            let items = DATA + 0x9000;
            let n = if seed % 7 == 0 { 0 } else { 1 + r.next_u32() % 3 };
            let mut node_at = DATA + 0xa000;
            for i in 0..n {
                let item = items + 8 * i;
                let model = DATA + 0x9800 + 0x40 * i;
                c.bus.write32(item, model);
                c.bus.write32(item + 4, r.next_u32() % 4);
                let len = r.next_u32() % 5;
                let mut next = 0;
                for _ in 0..len {
                    let node = node_at;
                    node_at += 0x100;
                    fill(c, &mut r, node, 0x40);
                    c.bus.write32(node, next);
                    let kind = [7, 8, 7, 8, r.next_u32() % 16][(r.next_u32() % 5) as usize];
                    c.bus.write32(node + 8, kind);
                    next = node;
                }
                c.bus.write32(model + 0x28, next);
            }
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(items);
            c.g[A2][0] = if seed % 11 == 3 { sx(0xffff_fffe) } else { n as u64 };
        });
    }

    /// PadQueue_Poll: 0-4 devices (and a negative count), write index 0-29
    /// (29 wraps); the devices' read methods are fakes (some unwind).
    #[test]
    fn pad_queue_poll_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, DATA};
        check(ee::s10::f_00326b88__generated, pad_queue_poll, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x32_6b88, 4);
            c.pc = 0x0032_6b88;
            let q = DATA;
            let n = if seed % 9 == 0 { u32::MAX } else { seed % 5 };
            c.bus.write32(q + 0x2ee8, n);
            c.bus.write32(q + 0x2ee4, if seed % 3 == 0 { 29 } else { r.next_u32() % 30 });
            for i in 0..4u32 {
                let dev = DATA + 0x4000 + 0x40 * i;
                let vt = DATA + 0x5000 + 0x40 * i;
                c.bus.write32(q + 0x2eec + 4 * i, dev);
                c.bus.write32(dev, vt);
                c.bus.write16(vt + 0x10, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x14, FAKE + 0x10 * i);
            }
            c.g[A0][0] = sx(q);
        });
    }
}
