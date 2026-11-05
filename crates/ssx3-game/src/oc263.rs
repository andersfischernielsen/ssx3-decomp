//! Staging wave 208 (oc263): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002a_a068, stream_buffer_get_request_status, "StreamBuffer_GetRequestStatus"),
    (0x002b_3c28, music_player_path_cmd_a, "MusicPlayer_PathCmdA"),
    (0x002a_eaa8, path_plat_get_queued_entry_value, "PATH_PlatGetQueuedEntryValue"),
    (0x002b_3d10, music_player_set_volume_percent, "MusicPlayer_SetVolumePercent"),
    (0x003e_2490, eafile_str_icmp, "EAFile_StrICmp"),
    (0x0030_9c88, wscript_man_call_func, "WScriptMan_CallFunc"),
    (0x0031_7500, app_vsync_callback, "App_VSyncCallback"),
    (0x0022_5068, cluno_table_assign, "cLunoTable_Assign"),
    (0x003d_fe88, stream_file_classify_chunk, "StreamFile_ClassifyChunk"),
    (0x0011_2a50, rider_select_ai_path, "Rider_SelectAIPath"),
];

// ---------------------------------------------------------------------------
// StreamBuffer_GetRequestStatus (0x2aa068, streamsys.cpp)

/// `SND_Lock` / `SND_Unlock` (sound library lock, through the table).
const SBRS_SND_LOCK: u32 = 0x003b_58a0;
const SBRS_SND_UNLOCK: u32 = 0x003b_58d8;
/// `SNDSTRM_GetRequestStatus(int request, SndRequestStatus* out)` (another shard).
const SBRS_SNDSTRM_GET_REQUEST_STATUS: u32 = 0x003b_7c40;
/// `StreamBuffer_GetStatus(self, SndStreamStatus* out)`.
const SBRS_STREAM_BUFFER_GET_STATUS: Callee = Callee {
    addr: 0x002a_a020,
    f: ee::s08::StreamBuffer_GetStatus,
};
/// SndStreamStatus field: the id of the stream's current request.
const SBRS_STATUS_REQUEST: u32 = 0x4;

/// `int StreamBuffer_GetRequestStatus(StreamBuffer* self, SndRequestStatus*
/// out, SndStreamStatus* scratch)` (0x2aa068): reads the stream status into
/// `scratch` (or a frame-local buffer when null), then, under the sound
/// library lock, `SNDSTRM_GetRequestStatus(status.request, out)`; returns its
/// result.
pub fn stream_buffer_get_request_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        // movn: the caller's status buffer, else the frame's own.
        c.g[S0][0] = if c.g[A2][0] != 0 { c.g[A2][0] } else { c.g[SP][0] };
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[S0][0];
        SBRS_STREAM_BUFFER_GET_STATUS.call(c, 0x002a_a08c)?;
        call_ptr(c, SBRS_SND_LOCK, 0x002a_a094)?;
        let status = r32(c, S0);
        c.g[A0][0] = Addr(status.wrapping_add(SBRS_STATUS_REQUEST)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, SBRS_SNDSTRM_GET_REQUEST_STATUS, 0x002a_a0a0)?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, SBRS_SND_UNLOCK, 0x002a_a0a8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MusicPlayer_PathCmdA (0x2b3c28, bankmonitor.cpp)

/// MusicPlayer fields: the current song instance, the "playing" flag.
const MP_SONG: u32 = 0x408;
const MP_ACTIVE: u32 = 0x418;
/// `SongInstance_PathCmdA(SongInstance*)`.
const MP_SONG_INSTANCE_PATH_CMD_A: Callee = Callee {
    addr: 0x002b_2120,
    f: ee::s08::SongInstance_PathCmdA,
};

/// `void MusicPlayer_PathCmdA(MusicPlayer* self)` (0x2b3c28): when the
/// player is active and has a song instance, forwards PATH command A to it.
pub fn music_player_path_cmd_a(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[V0][0] = Addr(this.wrapping_add(MP_ACTIVE)).lw(c);
        frame.restore_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(this.wrapping_add(MP_SONG)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                MP_SONG_INSTANCE_PATH_CMD_A.call(c, 0x002b_3c50)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PATH_PlatGetQueuedEntryValue (0x2aeaa8, bankmonitor.cpp)

/// The 24 sound-bank monitor slots (`BankMonitor*[24]`).
const PQ_MON_TABLE: u32 = 0x004d_3e98;
/// BankMonitor fields: whole-bank value, "status stale" flag, sub-entry
/// count (`s8`), sub-entry array (12-byte `{?, handle, value}`).
const PQ_MON_VALUE: u32 = 0x54;
const PQ_MON_STALE: u32 = 0x65;
const PQ_MON_SUB_COUNT: u32 = 0x66;
const PQ_MON_SUBS: u32 = 0x6c;
const PQ_SUB_SIZE: u32 = 0xc;
const PQ_SUB_HANDLE: u32 = 0x4;
const PQ_SUB_VALUE: u32 = 0x8;
/// `PATH_PlatRefreshStatus(int reset)`.
const PQ_PATH_PLAT_REFRESH_STATUS: Callee = Callee {
    addr: 0x002a_efa8,
    f: ee::s08::PATH_PlatRefreshStatus,
};

/// `int PATH_PlatGetQueuedEntryValue(int slot, int sub)` (0x2aeaa8): looks
/// up monitor `slot` (refreshing every monitor first when its status is
/// stale); `sub < 0` returns the monitor's own value, otherwise sub-entry
/// `sub`'s value, or -8 when `sub` is out of range or the entry has no
/// handle.
pub fn path_plat_get_queued_entry_value(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let index = (c.g[A0][0] as u32) << 2;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(PQ_MON_TABLE);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(index.wrapping_add(PQ_MON_TABLE));
        c.g[S0][0] = Addr(index.wrapping_add(PQ_MON_TABLE)).lw(c);
        c.g[S1][0] = c.g[A1][0];
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            let mon = r32(c, S0);
            c.g[V0][0] = Addr(mon.wrapping_add(PQ_MON_STALE)).lb(c);
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[A0][0] = 0;
                PQ_PATH_PLAT_REFRESH_STATUS.call(c, 0x002a_eae8)?;
            }
        }
        c.cycles += 2;
        let mon = r32(c, S0);
        let sub = c.g[S1][0];
        if (sub as i64) >= 0 {
            c.g[V0][0] = Addr(mon.wrapping_add(PQ_MON_SUB_COUNT)).lb(c);
            c.cycles += 3;
            let in_range = ((sub as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[V0][0] = PQ_SUB_SIZE as u64;
            if in_range == 0 {
                c.cycles += 1;
                c.g[V0][0] = sx(-8i32 as u32);
            } else {
                c.cycles += 6;
                c.g[A0][0] = Addr(mon.wrapping_add(PQ_MON_SUBS)).lw(c);
                let off = crate::abi::mult(c, sub as u32, PQ_SUB_SIZE);
                let entry = off.wrapping_add(c.g[A0][0] as u32);
                c.g[V0][0] = sx(entry);
                c.g[V1][0] = Addr(entry.wrapping_add(PQ_SUB_HANDLE)).lw(c);
                if (c.g[V1][0] as i64) < 0 {
                    c.g[V0][0] = sx(-8i32 as u32);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(entry.wrapping_add(PQ_SUB_VALUE)).lw(c);
                }
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(mon.wrapping_add(PQ_MON_VALUE)).lw(c);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MusicPlayer_SetVolumePercent (0x2b3d10, bankmonitor.cpp)

/// `SongInstance_SetVolumePercent(SongInstance*, s8 percent)`.
const MP_SONG_INSTANCE_SET_VOLUME_PERCENT: Callee = Callee {
    addr: 0x002b_23e8,
    f: ee::s08::SongInstance_SetVolumePercent,
};

/// `void MusicPlayer_SetVolumePercent(MusicPlayer* self, s8 percent)`
/// (0x2b3d10): when the player is active and has a song instance, sets the
/// song's PATH volume percentage.
pub fn music_player_set_volume_percent(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[V0][0] = Addr(this.wrapping_add(MP_ACTIVE)).lw(c);
        c.g[A1][0] = sx(c.g[A1][0] as u8 as i8 as i32 as u32);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(this.wrapping_add(MP_SONG)).lw(c);
            frame.restore_ra(c, 0);
            if c.g[A0][0] == 0 {
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 2;
            MP_SONG_INSTANCE_SET_VOLUME_PERCENT.call(c, 0x002b_3d3c)?;
        }
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EAFile_StrICmp (0x3e2490, seg_2DCAE8.c)

/// The ctype table indexed with a signed byte (0x4a0000 - 0x637f); bit 0 is
/// "uppercase".
const EAF_CTYPE: u32 = 0x0049_9c81;

/// `int EAFile_StrICmp(const char* a, const char* b)` (0x3e2490): compares
/// two strings ignoring case (uppercase letters per the ctype table are
/// lowered by +0x20); returns the first non-zero difference, else 0.
pub fn eafile_str_icmp(c: &mut Cpu) {
    c.cycles += 7;
    c.g[T2][0] = c.g[A0][0];
    c.g[T1][0] = c.g[A1][0];
    c.g[T0][0] = Addr(c.g[T2][0] as u32).lb(c);
    c.g[V0][0] = 0x4a_0000;
    c.g[A3][0] = Addr(c.g[T1][0] as u32).lb(c);
    c.g[T3][0] = sx(EAF_CTYPE);
    loop {
        // 0x3e24bc: lowered characters and their difference.
        c.cycles += 13;
        let ca = c.g[T0][0] as u32;
        let cb = c.g[A3][0] as u32;
        let table = c.g[T3][0] as u32;
        c.g[A0][0] = sx(ca.wrapping_add(table));
        c.g[A2][0] = sx(ca.wrapping_add(0x20));
        c.g[A1][0] = sx(cb.wrapping_add(table));
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lbu(c) & 1;
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lbu(c) & 1;
        c.g[A0][0] = sx(cb.wrapping_add(0x20));
        if c.g[V0][0] == 0 {
            c.g[A2][0] = c.g[T0][0];
        }
        if c.g[V1][0] == 0 {
            c.g[A0][0] = c.g[A3][0];
        }
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        if c.g[V0][0] != 0 {
            break;
        }
        // 0x3e24f0: equal so far; stop at the end of `a`.
        c.cycles += 2;
        let more = c.g[T0][0] != 0;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003e_24b0);
        }
        // 0x3e24b0: next pair.
        c.cycles += 3;
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(1));
        c.g[T0][0] = Addr(c.g[T2][0] as u32).lb(c);
        c.g[A3][0] = Addr(c.g[T1][0] as u32).lb(c);
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// WScriptMan_CallFunc (0x309c88, wscriptman.cpp)

/// WScriptMan fields: the world (script cache owner), the per-section
/// script tables (`cLunoValue*[]`, indexed by the handle's low byte).
const WSM_WORLD: u32 = 0x28c;
const WSM_SECTION_TABLES: u32 = 0x3cc;
/// Callees (all through the function table).
const WSM_WORLD_GET_SECTION_SCRIPT_FUNC: u32 = 0x003a_6d18;
const WSM_LUNO_VM_REGISTER_SET_RAW: u32 = 0x0022_6600;
const WSM_LUNO_VM_PASS_THROUGH: u32 = 0x0022_24a8;
const WSM_LUNO_VM_CALL: u32 = 0x0022_24b8;
const WSM_LUNO_VM_REGISTER_DTOR: u32 = 0x0022_5b90;

/// `void WScriptMan_CallFunc(WScriptMan* self, ScriptHandle* handle)`
/// (0x309c88): looks up the scripted function `handle` names in the world's
/// script cache, wraps it in a Luno value and calls it on the section's
/// script table with an empty result value, which is destroyed afterwards.
pub fn wscript_man_call_func(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x30);
        // func = World_GetSectionScriptFunc(self->world, handle)
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(WSM_WORLD)).lw(c);
        call_ptr(c, WSM_WORLD_GET_SECTION_SCRIPT_FUNC, 0x0030_9cac)?;
        // arg = {func, 0} at sp
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0;
        call_ptr(c, WSM_LUNO_VM_REGISTER_SET_RAW, 0x0030_9cbc)?;
        let section = Addr(r32(c, S1)).lbu(c);
        c.g[V0][0] = section;
        c.g[S2][0] = sx(sp(c).wrapping_add(0x20));
        Addr(sp(c).wrapping_add(0x18)).sw(c, 0);
        c.g[A0][0] = c.g[S2][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, 0);
        c.g[V0][0] = sx((section as u32) << 2);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(c.g[V0][0] as u32));
        c.g[S1][0] = Addr(r32(c, S0).wrapping_add(WSM_SECTION_TABLES)).lw(c);
        call_ptr(c, WSM_LUNO_VM_PASS_THROUGH, 0x0030_9ce0)?;
        // LunoVM_Call(&call, &arg, table, &result)
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A0][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, WSM_LUNO_VM_CALL, 0x0030_9cf8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        call_ptr(c, WSM_LUNO_VM_REGISTER_DTOR, 0x0030_9d04)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// App_VSyncCallback (0x317500, appman.cpp)

/// `App*` singleton (gp + 0x2a74).
const APP_INSTANCE_GP: u32 = 0x2a74;
/// `App_VSyncTick(App*)`.
const APP_VSYNC_TICK: Callee = Callee {
    addr: 0x0031_7348,
    f: ee::s10::App_VSyncTick,
};

/// `void App_VSyncCallback()` (0x317500): the VBlank handler hook; runs
/// `App_VSyncTick` on the application singleton.
pub fn app_vsync_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(r32(c, GP).wrapping_add(APP_INSTANCE_GP)).lw(c);
        frame.save_ra(c, 0);
        APP_VSYNC_TICK.call(c, 0x0031_7514)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cLunoTable_Assign (0x225068, lunovm.cpp)

/// cLunoTable field: the reference count.
const LTA_REFCOUNT: u32 = 0x8;
/// `cLunoTable_FreeData(cLunoTableRef*)`: destroys and frees the table.
const LTA_FREE_DATA: Callee = Callee {
    addr: 0x0022_4f30,
    f: ee::s05::cLunoTable_FreeData,
};

/// `cLunoTableRef* cLunoTable_Assign(cLunoTableRef* self, cLunoTableRef*
/// other)` (0x225068): reference assignment. When the tables differ, drops
/// the old reference (freeing the table at refcount 0), takes `other`'s
/// table and increments its refcount; returns `self`.
pub fn cluno_table_assign(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        let same = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = c.g[S0][0];
        if !same {
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                // Drop the old reference.
                c.cycles += 4;
                let old = c.g[V1][0] as u32;
                c.g[V0][0] = sx((Addr(old.wrapping_add(LTA_REFCOUNT)).lw(c) as u32).wrapping_sub(1));
                let live = c.g[V0][0] != 0;
                Addr(old.wrapping_add(LTA_REFCOUNT)).sw(c, c.g[V0][0]);
                if !live {
                    c.cycles += 2;
                    LTA_FREE_DATA.call(c, 0x0022_50b0)?;
                }
                c.cycles += 1;
            }
            c.g[V1][0] = Addr(r32(c, S1)).lw(c);
            // Take the new one.
            c.cycles += 5;
            let new = c.g[V1][0] as u32;
            Addr(r32(c, S0)).sw(c, c.g[V1][0]);
            c.g[V0][0] = sx((Addr(new.wrapping_add(LTA_REFCOUNT)).lw(c) as u32).wrapping_add(1));
            Addr(new.wrapping_add(LTA_REFCOUNT)).sw(c, c.g[V0][0]);
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamFile_ClassifyChunk (0x3dfe88, seg_2DCAE8.c)

/// StreamFile fields: the chunk classifier table and its entry count.
const SFC_ENTRIES: u32 = 0x1c;
const SFC_COUNT: u32 = 0x20;
/// Classifier entry (12 bytes): `{mask, value, result}`.
const SFC_ENTRY_SIZE: u32 = 0xc;
const SFC_ENTRY_VALUE: u32 = 0x4;
const SFC_ENTRY_RESULT: u32 = 0x8;

/// `int StreamFile_ClassifyChunk(StreamFile* self, const int* chunk_id)`
/// (0x3dfe88): the result of the first classifier entry whose
/// `(id & mask) == value`, or -2 when none matches.
pub fn stream_file_classify_chunk(c: &mut Cpu) {
    c.cycles += 4;
    let this = r32(c, A0);
    c.g[V1][0] = Addr(this.wrapping_add(SFC_COUNT)).lw(c);
    c.g[A2][0] = 0;
    c.g[A1][0] = Addr(r32(c, A1)).lw(c);
    if (c.g[V1][0] as i64) > 0 {
        c.cycles += 2;
        c.g[A0][0] = Addr(this.wrapping_add(SFC_ENTRIES)).lw(c);
        c.g[A3][0] = c.g[V1][0];
        loop {
            // 0x3dfea0
            c.cycles += 5;
            let entry = r32(c, A0);
            c.g[V0][0] = Addr(entry).lw(c);
            c.g[V1][0] = Addr(entry.wrapping_add(SFC_ENTRY_VALUE)).lw(c);
            c.g[V0][0] &= c.g[A1][0];
            let matched = c.g[V0][0] == c.g[V1][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            if matched {
                c.cycles += 2;
                c.g[V0][0] = Addr(entry.wrapping_add(SFC_ENTRY_RESULT)).lw(c);
                ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
            c.g[A0][0] = sx(entry.wrapping_add(SFC_ENTRY_SIZE));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_fea0);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = sx(-2i32 as u32);
    ret(c);
}

// ---------------------------------------------------------------------------
// Rider_SelectAIPath (0x112a50, computer.cpp)

/// Rider fields: position, velocity, the AI controller (vtable at +0),
/// the followed AI path and its search hint, the path follow state.
const RSA_POS: u32 = 0x110;
const RSA_VEL: u32 = 0x1e0;
const RSA_CLOSEST: u32 = 0x490;
const RSA_AHEAD_POINT: u32 = 0x4a0;
const RSA_PATH_DIST_PREV: u32 = 0x4c0;
const RSA_PATH_DIST: u32 = 0x4c4;
const RSA_CLOSEST_DIST: u32 = 0x4c8;
const RSA_AI: u32 = 0x6c0;
const RSA_AI_PATH: u32 = 0xab8;
const RSA_AI_PATH_HINT: u32 = 0xabc;
/// Path hint field: the current segment (-1: search the whole path).
const RSA_HINT_SEGMENT: u32 = 0x14;
/// AI controller vtable slots: "uses path weights" (int) and the weight of
/// a path (float).
const RSA_VT_USES_WEIGHTS: u32 = 0x48;
const RSA_VT_PATH_WEIGHT: u32 = 0xa0;
/// `PathData` holding the AI paths.
const RSA_PATH_DATA: u32 = 0x004d_33a0;
/// `.lit4` gp-0x7c7c: the initial best distance (~1e37).
const RSA_LIT_FAR: u32 = 0x7c7c;
/// Callees (another shard, through the table).
const RSA_FIND_NEAR_AI_PATHS: u32 = 0x0026_afb8;
const RSA_PATH_GET_CLOSEST_POINT: u32 = 0x0026_a428;
const RSA_PATH_GET_LENGTH: u32 = 0x0026_ac48;
const RSA_PATH_GET_POINT_AT_DISTANCE: u32 = 0x0026_ab20;
const RSA_PATH_GET_CLOSEST_POINT_FROM: u32 = 0x0026_a638;
/// Look-ahead along a path (796.0f), the end margin (200.0f), -1.0f.
const RSA_LOOK_AHEAD: u64 = 0x4447_0000;
const RSA_END_MARGIN: u64 = 0x4348_0000;
const RSA_MINUS_ONE: u64 = 0xffff_ffff_bf80_0000;

/// `|v|²` of the quadword at `addr` (x²+y²+z²+w² through ACC) into `gpr`
/// with `qmfc2`, as the original's inlined `vu0Dot`.
fn rsa_vu0_dot_self(c: &mut Cpu, addr: u32, gpr: usize) {
    c.lqc2(3, addr);
    c.lqc2(5, addr);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(gpr, 4);
}

/// `vf5 = [a] - [b]`, stored through the frame's temporary at sp+0x60 and
/// reloaded into r2 (the original's by-value vector return).
fn rsa_vu0_sub_to_v0(c: &mut Cpu, a: u32, b: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x60));
    c.lq(2, sp(c).wrapping_add(0x60));
}

/// `void Rider_SelectAIPath(Rider* self, bool keep_current)` (0x112a50):
/// the AI's path choice. Looks 796 units ahead along the velocity, finds up
/// to six AI paths near that point and scores each one that has at least
/// 200 units left past the rider: by the controller's path weight when it
/// uses weights (highest wins), else by the summed squared distances of the
/// rider to the path and of the look-ahead point to the path point 796
/// further on (lowest wins). With `keep_current`, the current path is not a
/// candidate; a winner equal to the current path counts as none. A new path
/// becomes current: its closest point and distance are recomputed from a
/// fresh search and the point 796 ahead becomes the steering target.
pub fn rider_select_ai_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x120);
        c.mtc1(0, 0);
        frame.save(c, S0, 0xf0);
        frame.save(c, S2, 0xd0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S5, 0xa0);
        c.g[S2][0] = 0;
        frame.save_fpr(c, 22, 0x110);
        c.g[S5][0] = c.g[A1][0];
        frame.save_fpr(c, 21, 0x108);
        frame.save(c, S1, 0xe0);
        frame.save(c, S3, 0xc0);
        frame.save(c, S4, 0xb0);
        frame.save(c, S6, 0x90);
        frame.save_ra(c, 0x80);
        frame.save_fpr(c, 20, 0x100);
        // |vel| (VU0 length).
        let this = r32(c, S0);
        c.lqc2(3, this.wrapping_add(RSA_VEL));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 1);
        c.lwc1(22, r32(c, GP).wrapping_sub(RSA_LIT_FAR)); // best distance
        c.c_eq_s(1, 0);
        c.g[AT][0] = RSA_MINUS_ONE;
        c.mtc1(1, 21); // best weight
        if c.fcc() {
            // Standing still: look from the rider's position.
            c.cycles += 3;
            c.lq(2, this.wrapping_add(RSA_POS));
            c.sq(2, sp(c).wrapping_add(0x30));
        } else {
            // look = pos + vel * (796 / |vel|)
            c.cycles += 17;
            c.g[AT][0] = RSA_LOOK_AHEAD;
            c.mtc1(1, 0);
            c.div_s(0, 0, 1);
            c.mfc1(3, 0);
            c.lqc2(4, this.wrapping_add(RSA_VEL));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp(c).wrapping_add(0x60));
            c.lq(2, sp(c).wrapping_add(0x60));
            c.sq(2, sp(c).wrapping_add(0x50));
            c.lqc2(3, this.wrapping_add(RSA_POS));
            c.lqc2(4, sp(c).wrapping_add(0x50));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp(c).wrapping_add(0x60));
            c.lq(2, sp(c).wrapping_add(0x60));
            c.sq(2, sp(c).wrapping_add(0x30));
            c.sq(2, sp(c).wrapping_add(0x40));
        }
        // n = PathData_FindNearAIPaths(&ai_paths, &look, paths, 6, 0)
        c.cycles += 10;
        c.g[A1][0] = sx(sp(c).wrapping_add(0x30));
        c.g[S6][0] = c.g[A1][0];
        c.g[A0][0] = sx(RSA_PATH_DATA);
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = 6;
        c.g[T0][0] = 0;
        call_ptr(c, RSA_FIND_NEAR_AI_PATHS, 0x0011_2b40)?;
        c.g[S3][0] = c.g[V0][0];
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 2;
            c.g[S4][0] = sx(r32(c, S0).wrapping_add(RSA_AI));
            c.g[S1][0] = c.g[SP][0];
            loop {
                // 0x112b50: candidate *s1.
                'candidate: {
                    c.cycles += 4;
                    c.g[A0][0] = Addr(r32(c, S1)).lw(c);
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(RSA_AI_PATH)).lw(c);
                    let is_current = c.g[A0][0] == c.g[V0][0];
                    c.g[A1][0] = c.g[S6][0];
                    if is_current {
                        c.cycles += 2;
                        if c.g[S5][0] != 0 {
                            c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                            break 'candidate;
                        }
                        c.cycles += 1;
                        c.g[A1][0] = c.g[S6][0];
                    }
                    // t = Path_GetClosestPoint(path, &look, &out, &t_out, 1)
                    c.cycles += 14;
                    c.g[A2][0] = sx(sp(c).wrapping_add(0x20));
                    c.g[A3][0] = sx(sp(c).wrapping_add(0x70));
                    c.g[T0][0] = 1;
                    call_ptr(c, RSA_PATH_GET_CLOSEST_POINT, 0x0011_2b7c)?;
                    c.g[A0][0] = Addr(r32(c, S1)).lw(c);
                    c.mov_s(20, 0);
                    call_ptr(c, RSA_PATH_GET_LENGTH, 0x0011_2b88)?;
                    c.g[AT][0] = RSA_END_MARGIN;
                    c.mtc1(1, 1);
                    c.sub_s(0, 0, 1);
                    c.c_lt_s(0, 20);
                    if c.fcc() {
                        // Too close to the end of this path.
                        c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                        break 'candidate;
                    }
                    // ahead = Path_GetPointAtDistance(path, 0, t + 796)
                    c.cycles += 46;
                    c.g[AT][0] = RSA_LOOK_AHEAD;
                    c.mtc1(1, 12);
                    c.g[A0][0] = sx(sp(c).wrapping_add(0x50));
                    c.g[A1][0] = Addr(r32(c, S1)).lw(c);
                    c.g[A2][0] = 0;
                    c.add_s(12, 20, 12);
                    call_ptr(c, RSA_PATH_GET_POINT_AT_DISTANCE, 0x0011_2bc0)?;
                    // d1 = look - ahead (sp+0x40), d0 = pos - out (sp+0x50)
                    let base = sp(c);
                    rsa_vu0_sub_to_v0(c, base.wrapping_add(0x30), base.wrapping_add(0x50));
                    c.sq(2, base.wrapping_add(0x40));
                    rsa_vu0_sub_to_v0(c, r32(c, S0).wrapping_add(RSA_POS), base.wrapping_add(0x20));
                    c.g[A1][0] = Addr(r32(c, S0).wrapping_add(RSA_AI)).lw(c);
                    c.sq(2, base.wrapping_add(0x50));
                    rsa_vu0_dot_self(c, base.wrapping_add(0x40), 2);
                    c.mtc1(2, 0);
                    rsa_vu0_dot_self(c, base.wrapping_add(0x50), 3);
                    c.mtc1(3, 1);
                    // ai->UsesPathWeights()
                    let vt = r32(c, A1);
                    c.g[A0][0] = Addr(vt.wrapping_add(RSA_VT_USES_WEIGHTS)).lh(c);
                    c.add_s(20, 1, 0); // dist = |d0|² + |d1|²
                    c.g[V0][0] = Addr(vt.wrapping_add(RSA_VT_USES_WEIGHTS + 4)).lw(c);
                    let method = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(r32(c, S4).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0011_2c50)?;
                    c.g[V1][0] = 1;
                    if c.g[V0][0] == c.g[V1][0] {
                        // weight = ai->PathWeight(path); highest wins.
                        c.cycles += 11;
                        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(RSA_AI)).lw(c);
                        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
                        let vt = r32(c, V0);
                        c.g[A0][0] = Addr(vt.wrapping_add(RSA_VT_PATH_WEIGHT)).lh(c);
                        c.g[V1][0] = Addr(vt.wrapping_add(RSA_VT_PATH_WEIGHT + 4)).lw(c);
                        let method = c.g[V1][0] as u32;
                        c.g[A0][0] = sx(r32(c, S0).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, method, 0x0011_2c74)?;
                        c.mov_s(20, 0);
                        c.c_lt_s(21, 20);
                        if !c.fcc() {
                            c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                            break 'candidate;
                        }
                        c.cycles += 3;
                        c.g[S2][0] = Addr(r32(c, S1)).lw(c);
                        c.mov_s(21, 20);
                    } else {
                        // Lowest distance wins.
                        c.cycles += 4;
                        c.c_lt_s(20, 22);
                        if !c.fcc() {
                            c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                            break 'candidate;
                        }
                        c.cycles += 2;
                        c.g[S2][0] = Addr(r32(c, S1)).lw(c);
                        c.mov_s(22, 20);
                    }
                    // 0x112cac: the current path is no new choice.
                    c.cycles += 4;
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(RSA_AI_PATH)).lw(c);
                    c.g[V0][0] ^= c.g[S2][0];
                    if c.g[V0][0] == 0 {
                        c.g[S2][0] = 0;
                    }
                    c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                }
                // 0x112cbc
                c.cycles += 2;
                let more = c.g[S3][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0011_2b50);
                }
            }
        }
        // 0x112cc4
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
        if c.g[S2][0] != 0 {
            // Follow the new path from a fresh closest-point search.
            c.cycles += 22;
            let this = r32(c, S0);
            c.g[V1][0] = Addr(this.wrapping_add(RSA_AI_PATH_HINT)).lw(c);
            Addr(this.wrapping_add(RSA_AI_PATH)).sw(c, c.g[S2][0]);
            c.g[A1][0] = sx(this.wrapping_add(RSA_POS));
            Addr(r32(c, V1).wrapping_add(RSA_HINT_SEGMENT)).sw(c, c.g[V0][0]);
            c.g[A2][0] = sx(this.wrapping_add(RSA_CLOSEST));
            c.g[A3][0] = sx(this.wrapping_add(RSA_CLOSEST_DIST));
            c.g[T1][0] = 1;
            c.g[A0][0] = Addr(this.wrapping_add(RSA_AI_PATH)).lw(c);
            c.g[T0][0] = Addr(this.wrapping_add(RSA_AI_PATH_HINT)).lw(c);
            call_ptr(c, RSA_PATH_GET_CLOSEST_POINT_FROM, 0x0011_2cf4)?;
            let this = r32(c, S0);
            c.g[AT][0] = RSA_LOOK_AHEAD;
            c.mtc1(1, 12);
            c.g[A0][0] = sx(sp(c).wrapping_add(0x40));
            c.g[A1][0] = Addr(this.wrapping_add(RSA_AI_PATH)).lw(c);
            c.g[A2][0] = 0;
            c.add_s(12, 0, 12);
            c.swc1(0, this.wrapping_add(RSA_PATH_DIST));
            call_ptr(c, RSA_PATH_GET_POINT_AT_DISTANCE, 0x0011_2d14)?;
            let this = r32(c, S0);
            c.lq(2, sp(c).wrapping_add(0x40));
            c.lwc1(0, this.wrapping_add(RSA_PATH_DIST));
            c.sq(2, this.wrapping_add(RSA_AHEAD_POINT));
            c.swc1(0, this.wrapping_add(RSA_PATH_DIST_PREV));
        }
        c.cycles += 13;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore_ra(c, 0x80);
        frame.restore_fpr(c, 22, 0x110);
        frame.restore_fpr(c, 21, 0x108);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, fill, put_vec, put_words, scramble, Rng, DATA};

    /// StreamBuffer_GetRequestStatus: caller's status buffer or the frame's,
    /// an unknown SND stream (status zeroed: request 0) or a known one with
    /// a request id that is negative (-8 early out) or unknown; fake lock
    /// hooks, real StreamBuffer_GetStatus / SNDSTRM_GetRequestStatus.
    #[test]
    fn stream_buffer_get_request_status_matches() {
        const SELF: u32 = DATA + 0x20_0000;
        const OUT: u32 = SELF + 0x100;
        const SCRATCH: u32 = SELF + 0x200;
        const STREAM: u32 = SELF + 0x1000;
        const REQ: u32 = SELF + 0x2000;
        check(ee::s08::f_002aa068__generated, stream_buffer_get_request_status, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x2a_a068, 4);
            c.pc = 0x002a_a068;
            c.g[A0][0] = sx(SELF);
            c.g[A1][0] = sx(OUT);
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { sx(SCRATCH) };
            // SND lock / unlock hooks.
            c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
            c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
            // One stream slot.
            c.bus.write8(0x0051_0000 - 0x56d2, 1);
            c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
            let id = if seed % 3 == 0 { 1 + r.next_u32() % 4 } else { 0 };
            c.bus.write32(SELF + 0x8, id);
            c.bus.write32(STREAM + 0x124, if seed % 5 == 1 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(STREAM + 0x11c, REQ);
            c.bus.write32(REQ + 0xc, if seed % 4 < 2 { 0x8000_0000 | r.next_u32() } else { r.next_u32() & 0x7fff_ffff });
            c.bus.write16(STREAM + 0x14, 0);
            c.bus.write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
        });
    }

    /// MusicPlayer_PathCmdA: inactive player, no song, or a song whose PATH
    /// command runs the real SongInstance_PathCmdA / PATH_SetTrackControlValue
    /// (PATH player not initialised: its early out).
    #[test]
    fn music_player_path_cmd_a_matches() {
        const SELF: u32 = DATA + 0x20_0000;
        check(ee::s08::f_002b3c28__generated, music_player_path_cmd_a, 240, &|c, seed| {
            let mut r = Rng(0x002b_3c28 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x002b_3c28;
            c.g[A0][0] = sx(SELF);
            c.bus.write32(SELF + MP_ACTIVE, if seed % 3 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(SELF + MP_SONG, if seed % 3 == 1 { 0 } else { SELF + 0x1000 });
            c.bus.write8(0x0044_f42c, 0); // PATH player not initialised
        });
    }

    /// PATH_PlatGetQueuedEntryValue: empty and filled monitor slots, stale
    /// status (the real PATH_PlatRefreshStatus over 24 slots whose monitors
    /// have no stream; fake SND lock hooks), negative / out-of-range /
    /// in-range sub indices with and without a handle.
    #[test]
    fn path_plat_get_queued_entry_value_matches() {
        const BANKS: u32 = DATA + 0x20_0000;
        const SUBS: u32 = DATA + 0x20_1000;
        check(ee::s08::f_002aeaa8__generated, path_plat_get_queued_entry_value, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x2a_eaa8, 0);
            c.pc = 0x002a_eaa8;
            c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
            c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
            for i in 0..4u32 {
                let e = PQ_MON_TABLE + 4 * i;
                if (seed / 4 + i) % 5 == 0 {
                    c.bus.write32(e, 0);
                    continue;
                }
                let bank = BANKS + 0x100 * i;
                let subs = SUBS + 0x100 * i;
                c.bus.write32(e, bank);
                c.bus.write32(bank, u32::MAX); // no stream: refresh skips it
                c.bus.write8(bank + PQ_MON_STALE, ((seed + i) % 4 == 0) as u8);
                c.bus.write8(bank + PQ_MON_SUB_COUNT, (r.next_u32() % 6) as u8);
                c.bus.write32(bank + PQ_MON_SUBS, subs);
                c.bus.write32(bank + PQ_MON_VALUE, r.next_u32());
                for j in 0..8u32 {
                    let h = if r.next_u32() % 3 == 0 { 0x8000_0000 | r.next_u32() } else { r.next_u32() >> 1 };
                    c.bus.write32(subs + 0xc * j + PQ_SUB_HANDLE, h);
                    c.bus.write32(subs + 0xc * j + PQ_SUB_VALUE, r.next_u32());
                }
            }
            c.g[A0][0] = (seed % 4) as u64;
            c.g[A1][0] = match seed % 5 {
                0 => sx(0x8000_0000 | r.next_u32()),
                1 => sx(r.next_u32() >> 1),
                _ => (r.next_u32() % 7) as u64,
            };
        });
    }

    /// MusicPlayer_SetVolumePercent: inactive player, no song, or a song
    /// with and without a PATH track (+0x54) through the real
    /// SongInstance_SetVolumePercent / PATH call (PATH player not
    /// initialised: its early out); any percent byte (sign-extended).
    #[test]
    fn music_player_set_volume_percent_matches() {
        const SELF: u32 = DATA + 0x20_0000;
        const SONG: u32 = SELF + 0x1000;
        check(ee::s08::f_002b3d10__generated, music_player_set_volume_percent, 240, &|c, seed| {
            let mut r = Rng(0x002b_3d10 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x002b_3d10;
            c.g[A0][0] = sx(SELF);
            c.bus.write32(SELF + MP_ACTIVE, if seed % 3 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(SELF + MP_SONG, if seed % 3 == 1 { 0 } else { SONG });
            c.bus.write32(SONG + 0x54, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write8(0x0044_f42c, 0); // PATH player not initialised
        });
    }

    /// EAFile_StrICmp: strings of 0..5 characters over a mixed-case
    /// alphabet, often `b` a case-flipped copy of `a` (equal, end reached),
    /// sometimes a prefix; uppercase bits set in the ctype table.
    #[test]
    fn eafile_str_icmp_matches() {
        check(ee::s14::f_003e2490__generated, eafile_str_icmp, 300, &|c, seed| {
            let mut r = Rng(0x003e_2490 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x003e_2490;
            for ch in b'A'..=b'Z' {
                c.bus.write8(EAF_CTYPE + ch as u32, 1);
            }
            const ALPHABET: [u8; 7] = [b'a', b'A', b'b', b'B', b'1', b'_', 0xc1];
            let a = DATA + 0x1000;
            let b = DATA + 0x2000;
            let n = seed % 6;
            for i in 0..n {
                let ch = ALPHABET[(r.next_u32() % 7) as usize];
                c.bus.write8(a + i, ch);
            }
            c.bus.write8(a + n, 0);
            let m = if seed % 7 == 0 { n.saturating_sub(1) } else { n };
            for i in 0..m {
                let x = c.bus.read8(a + i);
                let ch = if seed % 3 != 2 {
                    if x.is_ascii_lowercase() && r.next_u32() % 2 == 0 { x - 32 } else { x }
                } else {
                    ALPHABET[(r.next_u32() % 7) as usize]
                };
                c.bus.write8(b + i, ch);
            }
            c.bus.write8(b + m, 0);
            c.g[A0][0] = sx(a);
            c.g[A1][0] = sx(b);
        });
    }

    /// WScriptMan_CallFunc: a handle naming section 0..3 and script id
    /// 0..3 resolved through a world script cache (table entries of kind 3)
    /// to a Luno script whose function returns at once; the real
    /// LunoVM_PassThrough / LunoVM_Call / LunoVMRegister_dtor run on the pool
    /// allocator and a dlmalloc arena (as `luno_vm_call_matches` sets them up).
    #[test]
    fn wscript_man_call_func_matches() {
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        const SELF: u32 = DATA + 0x20_0000;
        const HANDLE: u32 = SELF + 0x1000;
        const WORLD: u32 = SELF + 0x1100;
        const CACHE: u32 = SELF + 0x1200;
        const ENTRIES: u32 = SELF + 0x1300;
        const FUNCS: u32 = SELF + 0x1400;
        const SCRIPT: u32 = SELF + 0x2000;
        const TABLES: u32 = SELF + 0x3000;
        check(ee::s10::f_00309c88__generated, wscript_man_call_func, 120, &|c, seed| {
            let mut r = alloc_machine(c, seed, 0x30_9c88);
            c.pc = 0x0030_9c88;
            let section = seed % 4;
            let id = (seed / 4) % 4;
            c.bus.write32(HANDLE, id << 8 | section);
            c.g[A0][0] = sx(SELF);
            c.g[A1][0] = sx(HANDLE);
            // World_GetSectionScriptFunc -> tWScriptCache_GetTableEntry.
            c.bus.write32(SELF + WSM_WORLD, WORLD);
            c.bus.write32(WORLD + 4, CACHE);
            c.bus.write32(CACHE + 4, ENTRIES);
            let entry = ENTRIES + 0xc * section;
            c.bus.write16(entry, 3);
            c.bus.write32(entry + 8, FUNCS);
            c.bus.write32(FUNCS + 0x3c, FUNCS + 0x40);
            c.bus.write32(FUNCS + 0x40 + 4 * id, SCRIPT);
            // The script: code offset 0x10, start pc 0, n registers, 0 locals.
            c.bus.write32(SCRIPT + 4, 0x10);
            c.bus.write32(SCRIPT + 0x14, 0);
            c.bus.write32(SCRIPT + 0x18, (seed / 16) % 4);
            c.bus.write32(SCRIPT + 0x1c, 0);
            // Per-section tables: a cLunoValue whose word 0 is a table
            // (refcount at +8).
            for i in 0..4u32 {
                let value = TABLES + 0x40 * i;
                let table = TABLES + 0x400 + 0x40 * i;
                c.bus.write32(SELF + WSM_SECTION_TABLES + 4 * i, value);
                c.bus.write32(value, table);
                c.bus.write32(table + 8, 100);
            }
            // LunoVM_Call's default value: a plain word.
            let d = 0x004c_9098;
            c.bus.write32(d, r.next_u32());
            c.bus.write32(d + 4, r.next_u32());
            c.bus.write32(d + 8, 2);
        });
    }

    /// App_VSyncCallback: the real App_VSyncTick on an App singleton (as in
    /// `app_vsync_tick_matches`: zero / non-zero frame step, fake vtable
    /// methods that sometimes unwind).
    #[test]
    fn app_vsync_callback_matches() {
        use crate::difftest::fakes::covered;
        const APP: u32 = DATA + 0x1000;
        const VT: u32 = DATA + 0x2000;
        const SUBOBJ: u32 = DATA + 0x3000;
        const SUBVT: u32 = DATA + 0x4000;
        covered(ee::s10::f_00317500__generated, app_vsync_callback, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x31_7500, 6);
            c.pc = 0x0031_7500;
            c.bus.write32(crate::difftest::GAME_GP + APP_INSTANCE_GP, APP);
            let acc = (r.next_u32() % 1000) as f32 / 1000.0;
            let step = if seed % 4 == 0 { 0.0 } else { (r.next_u32() % 3500) as f32 / 1000.0 };
            c.bus.write32(APP + 0x18, r.next_u32());
            c.bus.write32(APP + 0x24, step.to_bits());
            c.bus.write32(APP + 0x28, acc.to_bits());
            c.bus.write32(APP + 0x5c, VT);
            c.bus.write32(APP + 0x8, SUBOBJ);
            c.bus.write16(VT + 0x38, ((r.next_u32() % 64) as u16).wrapping_sub(32) & !1);
            c.bus.write32(VT + 0x3c, FAKE);
            c.bus.write32(SUBOBJ + 0x8, SUBVT);
            c.bus.write16(SUBVT + 0x28, ((r.next_u32() % 64) as u16).wrapping_sub(32) & !1);
            c.bus.write32(SUBVT + 0x2c, FAKE + 0x10);
        });
    }

    /// cLunoTable_Assign: the same table, a null old reference, an old
    /// reference that stays alive (refcount > 1) and one whose last
    /// reference is dropped (a real zeroed heap table freed by the real
    /// cLunoTable_FreeData on the pool allocator / dlmalloc arena).
    #[test]
    fn cluno_table_assign_matches() {
        use crate::game::dirtysock::tags2::tests::zero;
        use crate::game::fe::festaterewards::tests::heap_block;
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        const SELF: u32 = DATA + 0x400;
        const OTHER: u32 = DATA + 0x440;
        const OLD: u32 = DATA + 0x800;
        const NEW: u32 = DATA + 0x900;
        check(ee::s05::f_00225068__generated, cluno_table_assign, 120, &|c, seed| {
            let mut r = alloc_machine(c, seed, 0x22_5068);
            c.pc = 0x0022_5068;
            zero(c, SELF, 0x80);
            c.g[A0][0] = sx(SELF);
            c.g[A1][0] = sx(OTHER);
            c.bus.write32(NEW + LTA_REFCOUNT, r.next_u32() % 50);
            c.bus.write32(OTHER, NEW);
            match seed % 4 {
                0 => c.bus.write32(SELF, NEW),
                1 => c.bus.write32(SELF, 0),
                2 => {
                    c.bus.write32(SELF, OLD);
                    c.bus.write32(OLD + LTA_REFCOUNT, 2 + r.next_u32() % 50);
                }
                _ => {
                    let table = heap_block(c, 0x300);
                    zero(c, table, 0x300);
                    c.bus.write32(table + LTA_REFCOUNT, 1);
                    c.bus.write32(SELF, table);
                }
            }
        });
    }

    /// StreamFile_ClassifyChunk: entry counts -1..5, chunk ids that match
    /// the first, a later or no entry (masks of all, some or no bits).
    #[test]
    fn stream_file_classify_chunk_matches() {
        const SELF: u32 = DATA + 0x1000;
        const KEY: u32 = DATA + 0x1100;
        const ENTRIES: u32 = DATA + 0x1200;
        check(ee::s14::f_003dfe88__generated, stream_file_classify_chunk, 300, &|c, seed| {
            let mut r = Rng(0x003d_fe88 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x003d_fe88;
            let n = (seed % 7) as i32 - 1;
            c.bus.write32(SELF + SFC_COUNT, n as u32);
            c.bus.write32(SELF + SFC_ENTRIES, ENTRIES);
            let key = r.next_u32();
            c.bus.write32(KEY, key);
            let hit = if n > 0 && seed % 3 != 0 { (r.next_u32() % n as u32) as i32 } else { -1 };
            for i in 0..6u32 {
                let e = ENTRIES + SFC_ENTRY_SIZE * i;
                let mask = [u32::MAX, 0xffff_0000, 0xff, 0][(r.next_u32() % 4) as usize];
                let value = if i as i32 == hit { key & mask } else { (key & mask) ^ (1 + r.next_u32() % 0xff) };
                c.bus.write32(e, mask);
                c.bus.write32(e + SFC_ENTRY_VALUE, value);
                c.bus.write32(e + SFC_ENTRY_RESULT, r.next_u32());
            }
            c.g[A0][0] = sx(SELF);
            c.g[A1][0] = sx(KEY);
        });
    }

    /// Rider_SelectAIPath: standing still or moving (look-ahead point), 0..4
    /// AI paths of 1..4 segments around the rider (real
    /// PathData_FindNearAIPaths / Path_GetClosestPoint / Path_GetLength /
    /// Path_GetPointAtDistance / Path_GetClosestPointFrom on the game's
    /// `.lit4` constants), the current path among them or not, keep flag on
    /// and off, and the AI controller's two virtual methods as fakes ("uses
    /// weights" forced to 1 or not, a random weight), some unwinding.
    #[test]
    fn rider_select_ai_path_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::LOG_RESULTS;
        use crate::game::worldpainter::quadtree::tests::put_vtable;
        const SELF: u32 = DATA + 0x20_0000;
        const VT: u32 = DATA + 0x20_2000;
        const HINT: u32 = DATA + 0x20_2100;
        const PATHS: u32 = DATA + 0x20_4000;
        const SEGS: u32 = DATA + 0x20_5000;
        check(ee::s00::f_00112a50__generated, rider_select_ai_path, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x11_2a50, 4);
            elf_image(c);
            c.pc = 0x0011_2a50;
            let f = |v: f32| v.to_bits();
            let uf = |r: &mut Rng, lo: f32, hi: f32| lo + (r.next_u32() % 10_000) as f32 / 10_000.0 * (hi - lo);
            // The rider.
            let pos = [uf(&mut r, -100.0, 100.0), uf(&mut r, -100.0, 100.0), uf(&mut r, -10.0, 10.0)];
            put_vec(c, SELF + RSA_POS, [f(pos[0]), f(pos[1]), f(pos[2]), f(1.0)]);
            let vel = if seed % 4 == 0 {
                [0; 4]
            } else {
                [f(uf(&mut r, -30.0, 30.0)), f(uf(&mut r, -30.0, 30.0)), f(uf(&mut r, -3.0, 3.0)), 0]
            };
            put_vec(c, SELF + RSA_VEL, vel);
            c.bus.write32(SELF + RSA_AI, VT);
            put_vtable(c, &mut r, VT, &[RSA_VT_USES_WEIGHTS, RSA_VT_PATH_WEIGHT]);
            c.bus.write32(SELF + RSA_AI_PATH_HINT, HINT);
            for n in 0..8 {
                c.bus.write32(LOG_RESULTS + 4 * n, r.next_u32() % 2);
            }
            // The AI paths.
            let count = seed % 5;
            c.bus.write32(RSA_PATH_DATA + 8, count);
            c.bus.write32(RSA_PATH_DATA + 0xc, PATHS);
            for i in 0..4u32 {
                let path = PATHS + 0x40 * i;
                let segs = SEGS + 0x100 * i;
                let n = 1 + r.next_u32() % 4;
                c.bus.write32(path + 8, n);
                let start = [pos[0] + uf(&mut r, -150.0, 150.0), pos[1] + uf(&mut r, -150.0, 150.0), pos[2]];
                for k in 0..3 {
                    c.bus.write32(path + 0xc + 4 * k, f(start[k as usize]));
                }
                c.bus.write32(path + 0x18, segs);
                // Bounds (min, max): around the start, often holding the rider.
                let reach = if r.next_u32() % 2 == 0 { 400.0 } else { 50.0 };
                for k in 0..3 {
                    c.bus.write32(path + 0x1c + 4 * k, f(start[k as usize] - reach));
                    c.bus.write32(path + 0x28 + 4 * k, f(start[k as usize] + reach));
                }
                for k in 0..n {
                    let a = uf(&mut r, 0.0, 6.28);
                    put_vec(c, segs + 0x10 * k, [f(a.cos()), f(a.sin()), f(uf(&mut r, -0.2, 0.2)), f(uf(&mut r, 100.0, 700.0))]);
                }
            }
            let current = match seed % 3 {
                0 => 0,
                1 => PATHS + 0x40 * (r.next_u32() % 4),
                _ => PATHS,
            };
            c.bus.write32(SELF + RSA_AI_PATH, current);
            c.g[A0][0] = sx(SELF);
            c.g[A1][0] = ((seed / 3) % 2) as u64;
        });
    }
}
