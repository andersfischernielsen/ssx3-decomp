//! Staging wave 210 (oc285): hand-written replacements for the hottest
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
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002a_e7a8, path_plat_queue, "PATH_PlatQueue"),
    (0x0032_02f8, mem_pool16_alloc, "MEM_Pool16Alloc"),
    (0x0030_bb10, wscript_man_save_state, "WScriptMan_SaveState"),
    (0x0014_a0b0, be_get_rider_alt_character, "BE_GetRiderAltCharacter"),
    (0x0013_acb0, rider_air_state_save_state, "RiderAirState_SaveState"),
    (0x0012_fbf0, rider_natural_air_sub_save_state, "RiderNaturalAirSub_SaveState"),
    (0x0035_79a8, particle_save, "Particle_Save"),
    (0x0036_ccb8, emitter_randomize_desc_seeds, "Emitter_RandomizeDescSeeds"),
    (0x0035_53c0, mod_obj_dtor, "ModObj_dtor"),
];

// ---------------------------------------------------------------------------
// PATH_PlatQueue (0x2ae7a8)

/// The track monitor table (`D_004D3E98`, 24 pointers) and the bank-slot
/// table (`D_004D3EF8`, 24 entries of 0x28 bytes) of `PATH_PlatQueue`.
const PLAT_QUEUE_TRACKS: u32 = 0x004d_3e98;
const PLAT_QUEUE_BANK_SLOTS: u32 = 0x004d_3ef8;
/// Monitor slot fields used by `PATH_PlatQueue`.
mod plat_queue_mon {
    /// Stream handle (`SNDSTRM_*` handle; the bank voice in memory mode).
    pub const HANDLE: u32 = 0x0;
    pub const POS: u32 = 0x4;
    /// Optional file-offset base for `SNDSTRM_QueueFileOffset`, and its limit.
    pub const OFFSET_BASE: u32 = 0x28;
    pub const OFFSET_LIMIT: u32 = 0x2c;
    pub const VOLUME: u32 = 0x30;
    pub const IS_MEM: u32 = 0x64;
    pub const DIRTY: u32 = 0x65;
    pub const COUNT: u32 = 0x66;
    pub const SUBS: u32 = 0x6c;
}

mod plat_queue_callee {
    use crate::call::Callee;
    use ssx3_ee as ee;
    pub const SET_VOLUME: Callee = Callee {
        addr: 0x002a_f6c0,
        f: ee::s08::PATH_PlatSetVolume,
    };
    pub const FIND_BANK_SLOT: Callee = Callee {
        addr: 0x002a_e230,
        f: ee::s08::PATH_PlatFindBankSlot,
    };
    pub const GET_TIME_MS: Callee = Callee {
        addr: 0x002a_f370,
        f: ee::s08::PATH_PlatGetTimeMs,
    };
    pub const SND_LOCK: u32 = 0x003b_58a0;
    pub const SND_PLAY: u32 = 0x003b_6820;
    pub const SND_UNLOCK: u32 = 0x003b_58d8;
    pub const SNDSTRM_STOP: u32 = 0x003b_78f8;
    pub const SNDSTRM_QUEUE_FILE_OFFSET: u32 = 0x003b_7e10;
    pub const SNDSTRM_QUEUE: u32 = 0x003b_78d8;
}

/// `int PATH_PlatQueue(int track, a1, char* name, u32 sectorInfo, int tag,
/// signed char volume, int pos, int restart)` (0x2ae7a8): queue a track on
/// its monitor slot. A non-negative `volume` (t1) is stored and applied
/// first. In-memory slots look up the bank slot, play the patch under the
/// sound lock and stamp the bank slot's time; streaming slots optionally stop
/// the stream (`restart`, t3), clear the sub-stream ids, then queue the file
/// (at a file offset when the slot has one and the position is below its
/// limit). On success the sub-stream list is shifted up by one and the new
/// handle and `tag` (t0) go in front; the slot is marked dirty. Returns the
/// handle (or -1 when no bank slot exists).
pub fn path_plat_queue(c: &mut Cpu) {
    use plat_queue_callee as callee;
    use plat_queue_mon as mon;
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x4d_0000;
        frame.save(c, S2, 0x70);
        c.g[V0][0] = sx(PLAT_QUEUE_TRACKS);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        c.g[V1][0] = sx(r32(c, S2) << 2);
        frame.save(c, S4, 0x50);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        frame.save(c, S5, 0x40);
        c.g[T1][0] = sx(r32(c, T1) << 24);
        frame.save(c, S6, 0x30);
        c.g[T1][0] = sx(((r32(c, T1) as i32) >> 24) as u32);
        frame.save(c, S7, 0x20);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, FP, 0x10);
        c.g[S7][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S0, 0x90);
        c.g[FP][0] = c.g[T0][0]; // tag
        c.g[S0][0] = c.lw(r32(c, V1)); // the monitor slot
        c.g[S1][0] = c.g[T2][0]; // pos
        c.g[S5][0] = c.g[T3][0]; // restart
        c.g[S4][0] = u64::MAX; // bank: -1
        if (c.g[T1][0] as i64) >= 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[T1][0];
            c.sb(r32(c, S0).wrapping_add(mon::VOLUME), c.g[T1][0]);
            callee::SET_VOLUME.call(c, 0x002a_e820)?;
        }
        'out: {
            c.cycles += 3;
            c.g[V0][0] = c.lb(r32(c, S0).wrapping_add(mon::IS_MEM));
            let in_memory = c.g[V0][0] != 0;
            c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[S1][0] as i64)) as u64;
            if in_memory {
                // Bank mode: find the bank slot and play the patch.
                c.cycles += 7;
                c.g[A0][0] = c.g[S2][0];
                if c.g[V0][0] == 0 {
                    c.g[S1][0] = 0; // pos < 0 -> 0
                }
                c.g[A1][0] = sx(r32(c, S3) >> 16);
                callee::FIND_BANK_SLOT.call(c, 0x002a_e83c)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V1][0] = 0x28;
                if (c.g[S2][0] as i64) >= 0 {
                    c.cycles += 5;
                    c.g[V0][0] = 0x4d_0000;
                    c.mult(0, V1, S2, V1);
                    c.g[V0][0] = sx(PLAT_QUEUE_BANK_SLOTS);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                    c.g[S4][0] = c.lw(r32(c, V0).wrapping_add(0x4));
                }
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                if (c.g[S4][0] as i64) < 0 {
                    break 'out; // no bank: return -1
                }
                c.cycles += 26;
                c.g[V0][0] = c.g[S3][0] & 0xffff;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                c.sw(r32(c, S0).wrapping_add(mon::POS), c.g[V0][0]);
                call_ptr(c, callee::SND_LOCK, 0x002a_e878)?;
                c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(mon::POS));
                c.g[A0][0] = c.g[S4][0];
                c.g[A2][0] = sx(r32(c, S0).wrapping_add(mon::VOLUME));
                call_ptr(c, callee::SND_PLAY, 0x002a_e888)?;
                c.g[S1][0] = c.g[V0][0];
                call_ptr(c, callee::SND_UNLOCK, 0x002a_e890)?;
                callee::GET_TIME_MS.call(c, 0x002a_e898)?;
                c.g[A0][0] = 0x28;
                c.g[V1][0] = 0x4d_0000;
                c.mult(0, A0, S2, A0);
                c.g[V1][0] = sx(PLAT_QUEUE_BANK_SLOTS);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                c.sw(r32(c, V1).wrapping_add(0x24), c.g[V0][0]); // slot time
                let m = r32(c, S0);
                c.sw(m.wrapping_add(mon::HANDLE), c.g[S4][0]);
                c.g[V0][0] = c.lw(m.wrapping_add(mon::SUBS));
                c.sw(r32(c, V0).wrapping_add(0x4), c.g[S1][0]);
                c.g[V0][0] = 1;
                c.g[V1][0] = c.lw(m.wrapping_add(mon::SUBS));
                c.sw(r32(c, V1), c.g[FP][0]);
            } else {
                // Stream mode.
                c.cycles += 2;
                let restart = c.g[S5][0] != 0;
                c.g[S1][0] = sx(r32(c, S3) << 7);
                if restart {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(mon::HANDLE));
                    call_ptr(c, callee::SNDSTRM_STOP, 0x002a_e8dc)?;
                    c.g[V1][0] = c.lb(r32(c, S0).wrapping_add(mon::COUNT));
                    c.g[A0][0] = sx(r32(c, V1).wrapping_sub(1));
                    c.g[V0][0] = 0xc;
                    if (c.g[A0][0] as i64) > 0 {
                        // Clear the ids of sub-streams 1..count (from the top).
                        c.cycles += 3;
                        c.g[A2][0] = u64::MAX;
                        c.mult(0, A1, A0, V0);
                        loop {
                            c.cycles += 9;
                            let m = r32(c, S0);
                            c.g[V1][0] = c.lw(m.wrapping_add(mon::SUBS));
                            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
                            c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
                            c.sw(r32(c, V1).wrapping_sub(0x8), c.g[A2][0]);
                            c.g[V0][0] = c.lw(m.wrapping_add(mon::SUBS));
                            c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                            c.sw(r32(c, V0).wrapping_sub(0xc), c.g[A2][0]);
                            let more = (c.g[A0][0] as i64) > 0;
                            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0xc));
                            if !more {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x002a_e8f8);
                            }
                        }
                    }
                }
                // Queue the file: at an offset when the slot has one and the
                // position is inside it.
                c.cycles += 3;
                let m = r32(c, S0);
                c.g[A2][0] = c.lw(m.wrapping_add(mon::OFFSET_BASE));
                let mut queued = false;
                if c.g[A2][0] == 0 {
                    c.g[A0][0] = c.lw(m.wrapping_add(mon::HANDLE));
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(m.wrapping_add(mon::OFFSET_LIMIT));
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    c.g[A3][0] = c.g[S1][0];
                    if c.g[V0][0] != 0 {
                        c.cycles += 5;
                        c.g[A0][0] = c.lw(m.wrapping_add(mon::HANDLE));
                        c.g[A1][0] = c.g[S6][0];
                        call_ptr(c, callee::SNDSTRM_QUEUE_FILE_OFFSET, 0x002a_e944)?;
                        c.g[S1][0] = c.g[V0][0];
                        queued = true;
                    } else {
                        c.cycles += 1;
                        c.g[A0][0] = c.lw(m.wrapping_add(mon::HANDLE));
                    }
                }
                if !queued {
                    c.cycles += 5;
                    c.g[A3][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S6][0];
                    c.g[A2][0] = c.g[S7][0];
                    call_ptr(c, callee::SNDSTRM_QUEUE, 0x002a_e960)?;
                    c.g[S1][0] = c.g[V0][0];
                }
                c.cycles += 2;
                c.g[V0][0] = 1;
                if (c.g[S1][0] as i64) >= 0 {
                    c.cycles += 4;
                    let m = r32(c, S0);
                    c.g[V0][0] = c.lb(m.wrapping_add(mon::COUNT));
                    c.g[A0][0] = sx(r32(c, V0).wrapping_sub(1));
                    c.g[V0][0] = 0xc;
                    if (c.g[A0][0] as i64) > 0 {
                        // Shift the 12-byte sub-stream records up by one.
                        c.cycles += 1;
                        c.mult(0, V1, A0, V0);
                        loop {
                            c.cycles += 11;
                            let m = r32(c, S0);
                            c.g[V0][0] = c.lw(m.wrapping_add(mon::SUBS));
                            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
                            c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                            let rec = r32(c, V0);
                            c.ldl(A1, rec.wrapping_sub(0x5));
                            c.ldr(A1, rec.wrapping_sub(0xc));
                            c.g[A2][0] = c.lw(rec.wrapping_sub(0x4));
                            c.sdl(A1, rec.wrapping_add(0x7));
                            c.sdr(A1, rec);
                            c.sw(rec.wrapping_add(0x8), c.g[A2][0]);
                            let more = (c.g[A0][0] as i64) > 0;
                            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0xc));
                            if !more {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x002a_e980);
                            }
                        }
                    }
                    c.cycles += 5;
                    let m = r32(c, S0);
                    c.g[V0][0] = c.lw(m.wrapping_add(mon::SUBS));
                    c.sw(r32(c, V0).wrapping_add(0x4), c.g[S1][0]);
                    c.g[V1][0] = c.lw(m.wrapping_add(mon::SUBS));
                    c.sw(r32(c, V1), c.g[FP][0]);
                    c.g[V0][0] = 1;
                }
            }
            // Mark the slot dirty and return the handle.
            c.cycles += 2;
            c.sb(r32(c, S0).wrapping_add(mon::DIRTY), c.g[V0][0]);
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x90);
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
// MEM_Pool16Alloc (0x3202f8)

/// The 16-byte pool's free-list head (`pool + 0x2000`).
const MEM_POOL16_FREE: u32 = 0x2000;
const MEM_POOL16_FALLBACK: Callee = Callee {
    addr: 0x0031_9e48,
    f: ee::s10::f_00319e48,
};

/// `void* MEM_Pool16Alloc(Pool16* pool)` (0x3202f8): pop a block off the
/// 16-byte pool's free list (`pool + 0x2000`); when it is empty fall back
/// to `MEM_PoolAlloc(0x14, a1)` (a1 is whatever the caller left there).
pub fn mem_pool16_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let head = r32(c, A0).wrapping_add(MEM_POOL16_FREE);
        c.g[V0][0] = c.lw(head);
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.lw(r32(c, V0)); // next free block
            c.cycles += 2;
            c.sw(head, c.g[V1][0]);
        } else {
            c.cycles += 4;
            c.g[A0][0] = 0x14;
            MEM_POOL16_FALLBACK.call(c, 0x0032_0314)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptMan_SaveState (0x30bb10)

/// `WScriptMan` fields saved by `WScriptMan_SaveState`.
mod wsm_save {
    /// The plain state block written first (0x28c bytes from `self`).
    pub const STATE_SIZE: u64 = 0x28c;
    /// The world (`world + 4` is its script cache).
    pub const WORLD: u32 = 0x28c;
    /// The two process lists.
    pub const LIST_A: u32 = 0x2b8;
    pub const LIST_B: u32 = 0x2bc;
    /// The collect-set table (one, 0x10c bytes).
    pub const COLLECT_SET: u32 = 0x2c0;
    pub const COLLECT_SET_SIZE: u32 = 0x10c;
    /// The 50 Luno object tables.
    pub const OBJS: u32 = 0x3cc;
    pub const OBJ_COUNT: u64 = 50;
}

const WSM_COLLECT_SET_SAVE: Callee = Callee {
    addr: 0x0030_c6c8,
    f: ee::s10::WScriptMan_CollectSetSave,
};
const WSM_SAVE_PROCESS_LIST: Callee = Callee {
    addr: 0x0030_bc80,
    f: ee::s10::WScriptMan_SaveProcessList,
};
/// `cWScriptCache_GetData` and `cLunoTable_Write` (cross-shard: the table).
const WSM_CACHE_GET_DATA: u32 = 0x003a_ca38;
const WSM_LUNO_TABLE_WRITE: u32 = 0x0022_57e0;

/// `stream->Write(...)` through the stream's vtable slot 8 (`{delta, fn}`):
/// the vtable is loaded into `vt`, the method into `func`, `a0 = stream +
/// delta`.
fn wsm_save_stream_write(c: &mut Cpu, vt: usize, func: usize, ret: u32) -> crate::call::Flow {
    let stream = r32(c, S2);
    c.g[vt][0] = c.lw(stream);
    c.g[A0][0] = c.lh(r32(c, vt).wrapping_add(0x8));
    c.g[func][0] = c.lw(r32(c, vt).wrapping_add(0xc));
    c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
    let target = r32(c, func);
    call_ptr(c, target, ret)
}

/// `void WScriptMan_SaveState(WScriptMan* self, cPbStream* stream)`
/// (0x30bb10): write the script manager's state to a save stream: its first
/// 0x28c bytes, the collect set, both process lists, then the number of
/// live Luno object tables and, for each, its index and the table (with the
/// script cache entry of that index).
pub fn wscript_man_save_state(c: &mut Cpu) {
    use wsm_save as f;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x60);
        c.g[A2][0] = f::STATE_SIZE;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0]; // stream
        frame.save_ra(c, 0x10);
        c.g[S3][0] = c.g[A0][0]; // self
        c.g[A1][0] = c.g[S3][0];
        c.g[S0][0] = sx(r32(c, S3).wrapping_add(f::COLLECT_SET));
        wsm_save_stream_write(c, V1, V0, 0x0030_bb54)?;

        // The collect sets (a loop over one).
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        WSM_COLLECT_SET_SAVE.call(c, 0x0030_bb64)?;
        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(f::COLLECT_SET_SIZE));

        // Both process lists.
        c.cycles += 14;
        let this = r32(c, S3);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = sx(this.wrapping_add(f::LIST_A));
        c.g[A2][0] = c.g[S2][0];
        c.g[S0][0] = sx(this.wrapping_add(f::OBJS));
        WSM_SAVE_PROCESS_LIST.call(c, 0x0030_bb84)?;
        let this = r32(c, S3);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = sx(this.wrapping_add(f::LIST_B));
        c.g[A2][0] = c.g[S2][0];
        WSM_SAVE_PROCESS_LIST.call(c, 0x0030_bb94)?;

        // count = number of live object tables (local at sp+0).
        c.sw(sp(c), 0);
        c.g[V1][0] = c.g[S0][0];
        c.g[A0][0] = f::OBJ_COUNT - 1;
        c.g[V0][0] = c.lw(r32(c, V1));
        loop {
            c.cycles += 3;
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
            let empty = c.g[V0][0] == 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
            if !empty {
                c.cycles += 3;
                c.g[V0][0] = c.lw(sp(c));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                c.sw(sp(c), c.g[V0][0]);
            }
            c.cycles += 2;
            if (c.g[A0][0] as i64) < 0 {
                break;
            }
            c.g[V0][0] = c.lw(r32(c, V1));
            if event_due(c) {
                deliver_event_here(c, 0x0030_bba8);
            }
        }
        c.cycles += 10;
        c.g[V0][0] = c.lw(r32(c, S2));
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 4;
        wsm_save_stream_write(c, V0, V1, 0x0030_bbe4)?;

        // Each live table: its index j (local at sp+4), then the table.
        c.sw(sp(c).wrapping_add(4), 0);
        c.g[A0][0] = 0;
        loop {
            c.cycles += 6;
            c.g[V0][0] = sx(r32(c, A0) << 2);
            c.g[A1][0] = sx(sp(c).wrapping_add(4));
            c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
            c.g[V1][0] = c.lw(r32(c, V0));
            c.g[A2][0] = 4;
            if c.g[V1][0] != 0 {
                c.cycles += 16;
                wsm_save_stream_write(c, V0, V1, 0x0030_bc1c)?;
                c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(f::WORLD));
                c.g[A1][0] = c.lw(sp(c).wrapping_add(4));
                c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(4));
                call_ptr(c, WSM_CACHE_GET_DATA, 0x0030_bc2c)?;
                c.g[V1][0] = c.lw(sp(c).wrapping_add(4));
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[V1][0] = sx(r32(c, V1) << 2);
                c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
                c.g[A0][0] = c.lw(r32(c, V1));
                call_ptr(c, WSM_LUNO_TABLE_WRITE, 0x0030_bc48)?;
            }
            c.cycles += 6;
            c.g[V0][0] = c.lw(sp(c).wrapping_add(4));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = ((c.g[A0][0] as i64) < f::OBJ_COUNT as i64) as u64;
            let more = c.g[V1][0] != 0;
            c.sw(sp(c).wrapping_add(4), c.g[V0][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0030_bbf0);
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
// BE_GetRiderAltCharacter (0x14a0b0)

/// The rider -> character id table (`D_005305B0`, one word per rider) and
/// the character table (`D_00535B20`, 0x1c bytes each; +0x12 is the signed
/// alternate-character byte).
const BE_RIDER_CHARACTER_IDS: u32 = 0x0053_05b0;
const BE_CHARACTERS: u32 = 0x0053_5b20;
const BE_CHARACTER_SIZE: u64 = 0x1c;
const BE_CHARACTER_ALT: u32 = 0x12;

/// `int BE_GetRiderAltCharacter(int rider)` (0x14a0b0): the alternate
/// character byte (signed) of the rider's character entry.
pub fn be_get_rider_alt_character(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = sx(BE_RIDER_CHARACTER_IDS);
    c.g[A0][0] = sx(r32(c, A0) << 2);
    c.g[A2][0] = BE_CHARACTER_SIZE;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(BE_RIDER_CHARACTER_IDS));
    c.g[V1][0] = c.lw(r32(c, A0)); // character id
    c.g[A1][0] = sx(BE_CHARACTERS);
    c.mult(0, V1, V1, A2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(BE_CHARACTERS));
    c.g[V0][0] = c.lb(r32(c, V1).wrapping_add(BE_CHARACTER_ALT));
    ret(c);
}

// ---------------------------------------------------------------------------
// Rider state SaveState forwarders (0x13acb0, 0x12fbf0)

/// The shape of the rider states' `SaveState(self, stream)`: `stream->Write(
/// self, size)` through the stream's vtable slot 8 (`{delta, fn}`), in a
/// 0x10 frame holding `ra`; `ret` is the return address of the call.
fn rider_state_save_forward(c: &mut Cpu, ret: u32, size: u64) -> crate::call::Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x10);
    c.g[A3][0] = c.g[A1][0];
    let stream = c.g[A3][0];
    frame.save_ra(c, 0);
    c.g[A1][0] = c.g[A0][0];
    c.g[A2][0] = size;
    c.g[V1][0] = c.lw(stream as u32);
    let vt = c.g[V1][0];
    let slot = crate::call::VSlot::read(c, vt, 0x8);
    c.g[A0][0] = slot.delta;
    c.g[V0][0] = slot.func;
    c.g[A0][0] = slot.this(stream);
    call_ptr(c, slot.func as u32, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void RiderAirState_SaveState(RiderAirState* self, cPbStream* stream)`
/// (0x13acb0): write the state's 4 bytes.
pub fn rider_air_state_save_state(c: &mut Cpu) {
    guest(c, |c| rider_state_save_forward(c, 0x0013_acd8, 0x4))
}

/// `void RiderNaturalAirSub_SaveState(RiderNaturalAirSub* self, cPbStream*
/// stream)` (0x12fbf0): write the sub-state's 0x14 bytes.
pub fn rider_natural_air_sub_save_state(c: &mut Cpu) {
    guest(c, |c| rider_state_save_forward(c, 0x0012_fc18, 0x14))
}

// ---------------------------------------------------------------------------
// Particle_Save (0x3579a8)

/// `void Particle_Save(Particle* self, Stream* stream)` (0x3579a8): a
/// particle saves exactly what its modifier-object base saves
/// (`ModObj_Save`).
pub fn particle_save(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(Callee {
            addr: 0x0035_6b30,
            f: ee::s11::ModObj_Save,
        }),
        0x0035_79b8,
    )
}

// ---------------------------------------------------------------------------
// Emitter_RandomizeDescSeeds (0x36ccb8)

/// `Rand_NextMain` (cross-shard: through the table).
const EMITTER_RAND_NEXT_MAIN: u32 = 0x0031_77f0;
/// The emitter descriptor's nine random seeds (floats in [1, 2)).
const EMITTER_SEED_FIELDS: [u32; 9] = [0x38, 0x3c, 0x40, 0x44, 0x48, 0x50, 0x54, 0x58, 0x5c];
/// Return addresses of the nine `Rand_NextMain` calls.
const EMITTER_SEED_RETS: [u32; 9] = [
    0x0036_cce4,
    0x0036_cd08,
    0x0036_cd24,
    0x0036_cd40,
    0x0036_cd5c,
    0x0036_cd78,
    0x0036_cd94,
    0x0036_cdb0,
    0x0036_cdcc,
];

/// `void Emitter_RandomizeDescSeeds(EmitterDesc* self)` (0x36ccb8): fill the
/// descriptor's nine seed floats (+0x38..+0x48, +0x50..+0x5c) with random
/// values in [1, 2): a random mantissa under exponent 0 (`(r & 0x7fffff) |
/// 1.0f`), then `(x - 1) + 1` as the original computes it.
pub fn emitter_randomize_desc_seeds(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    guest(c, |c| {
        c.cycles += 82;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = 0x7f_0000;
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] |= 0xffff; // mantissa mask
        frame.save_fpr(c, 20, 0x40);
        c.g[S1][0] = 0x3f80_0000; // 1.0f
        call_ptr(c, EMITTER_RAND_NEXT_MAIN, EMITTER_SEED_RETS[0])?;
        c.g[AT][0] = 0x3f80_0000;
        c.f[20] = 0x3f80_0000;
        for (i, &field) in EMITTER_SEED_FIELDS.iter().enumerate() {
            let last = i == EMITTER_SEED_FIELDS.len() - 1;
            c.g[V0][0] &= c.g[S0][0];
            if last {
                frame.restore_ra(c, 0);
            }
            c.g[V0][0] |= c.g[S1][0];
            if last {
                frame.restore(c, S0, 0x30);
            }
            c.f[1] = r32(c, V0);
            if last {
                frame.restore(c, S1, 0x20);
            }
            let one = FpuF32(c.f[20]);
            c.f[0] = (FpuF32(c.f[1]) - one).0;
            c.f[0] = (FpuF32(c.f[0]) + one).0;
            if last {
                frame.restore_fpr(c, 20, 0x40);
                c.swc1(0, r32(c, S2).wrapping_add(field));
                frame.restore(c, S2, 0x10);
                frame.close(c);
            } else {
                c.swc1(0, r32(c, S2).wrapping_add(field));
                call_ptr(c, EMITTER_RAND_NEXT_MAIN, EMITTER_SEED_RETS[i + 1])?;
            }
        }
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModObj_dtor (0x3553c0)

/// `ModObj` fields used by its destructor.
const MOD_OBJ_VTABLE: u32 = 0xc;
const MOD_OBJ_BLOCK: u32 = 0x1c;
/// The `ModObj` vtable (`D_00491028`).
const MOD_OBJ_VTABLE_ADDR: u32 = 0x0049_1028;
const MOD_OBJ_V49: Callee = Callee {
    addr: 0x0035_67e0,
    f: ee::s11::ModObj_v49,
};
const MOD_OBJ_BLOCK_DTOR: Callee = Callee {
    addr: 0x0035_2ae8,
    f: ee::s11::ModifierBlock_dtor,
};
const MOD_OBJ_BASE_DTOR: Callee = Callee {
    addr: 0x0034_fbf0,
    f: ee::s11::WorldObj_dtor,
};

/// `ModObj* ModObj_dtor(ModObj* self, int flags)` (0x3553c0): restore the
/// ModObj vtable, refresh the bounds one last time (`ModObj_v49`), delete
/// the modifier block if there is one (`ModifierBlock_dtor(block, 3)`), then
/// run the `WorldObj` destructor with the caller's flags.
pub fn mod_obj_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(MOD_OBJ_VTABLE_ADDR);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0]; // self
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0]; // flags
        c.sw(r32(c, S0).wrapping_add(MOD_OBJ_VTABLE), c.g[V0][0]);
        MOD_OBJ_V49.call(c, 0x0035_53e8)?;
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(MOD_OBJ_BLOCK));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3; // destroy and free
            MOD_OBJ_BLOCK_DTOR.call(c, 0x0035_53fc)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.cycles += 7;
        c.g[A1][0] = c.g[S1][0];
        MOD_OBJ_BASE_DTOR.call(c, 0x0035_5408)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// PATH_PlatQueue: in-memory slots with no bank slot, a negative bank and
    /// a played patch; streaming slots with a stop, the file-offset queue,
    /// the plain queue failing (no stream) and succeeding (a real
    /// SNDSTRM_QueueRequest -> StreamFile_QueueRange chain on a fake stream
    /// file), with 0..4 sub-streams; volume set or not.
    #[test]
    fn path_plat_queue_matches() {
        use crate::difftest::fakes::{covered, machine};
        use crate::difftest::DATA;
        const MON: u32 = DATA + 0x2000;
        const SUBS: u32 = DATA + 0x2400;
        const STREAM: u32 = DATA + 0x3000;
        const FILE: u32 = DATA + 0x3400;
        const NODE: u32 = DATA + 0x3600;
        const HEADER: u32 = DATA + 0x3800;
        const REQ: u32 = DATA + 0x4000;
        const NAME: u32 = DATA + 0x5000;
        /// SNDSTRM stream table and count (`0x510000 - 0x5f60`, `- 0x56d2`).
        const STRM_TABLE: u32 = 0x0050_a0a0;
        const STRM_COUNT: u32 = 0x0050_a92e;
        covered(
            ssx3_ee::s08::f_002ae7a8__generated,
            path_plat_queue,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ae7a8, 3);
                c.pc = 0x002a_e7a8;
                let track = r.next_u32() % 24;
                let case = seed % 8;
                c.bus.write32(PLAT_QUEUE_TRACKS + 4 * track, MON);
                c.g[A0][0] = track as u64;
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(NAME);
                c.bus.write32(NAME, 0x006b_7274); // "trk"
                let info = r.next_u32();
                c.g[A3][0] = sx(info);
                c.g[T0][0] = sx(r.next_u32()); // tag
                c.g[T1][0] = sx(if (seed / 8) % 2 == 0 {
                    r.next_u32() & 0x7f
                } else {
                    0x80 | r.next_u32()
                });
                c.g[T2][0] = sx(match (seed / 16) % 3 {
                    0 => (r.next_u32() % 4).wrapping_neg(),
                    _ => r.next_u32() % 0x1000,
                });
                c.g[T3][0] = (case == 3 || (case == 5 && seed % 3 == 0)) as u64;
                // Timer for PATH_PlatGetTimeMs.
                c.bus.write32(0x0045_0dc8, 1000);
                c.bus.write32(0x0045_0dcc, r.next_u32() % 100_000);
                // Bank slots: none owned by the track unless the case says so.
                for i in 0..24 {
                    c.bus.write32(PLAT_QUEUE_BANK_SLOTS + 0x28 * i + 8, u32::MAX);
                }
                let count = r.next_u32() % 5;
                c.bus.write8(MON + 0x66, count as u8);
                c.bus.write32(MON + 0x6c, SUBS);
                for i in 0..16 {
                    c.bus.write32(SUBS + 4 * i, r.next_u32());
                }
                c.bus.write8(MON + 0x64, (case < 3) as u8);
                let handle = r.next_u32() % 4;
                c.bus.write32(MON, handle);
                match case {
                    0 => {}
                    1 | 2 => {
                        let k = r.next_u32() % 24;
                        let e = PLAT_QUEUE_BANK_SLOTS + 0x28 * k;
                        c.bus.write32(e + 8, track);
                        c.bus.write32(e, info >> 16);
                        let bank = if case == 1 {
                            0x8000_0000 | r.next_u32()
                        } else {
                            r.next_u32() % 4
                        };
                        c.bus.write32(e + 4, bank);
                    }
                    _ => {
                        let pos = info << 7;
                        let (base, limit) = match case {
                            4 => (1 | r.next_u32(), pos.wrapping_add(1).max(1)),
                            5 => (1 | r.next_u32(), pos.min(r.next_u32())),
                            _ => (0, 0),
                        };
                        c.bus.write32(MON + 0x28, base);
                        c.bus.write32(MON + 0x2c, limit);
                        if case >= 6 {
                            // A live stream whose queue request succeeds.
                            c.bus.write8(STRM_COUNT, 4);
                            c.bus.write32(STRM_TABLE + 4 * handle, STREAM);
                            c.bus.write32(STREAM, FILE);
                            c.bus.write32(STREAM + 0xc, 0);
                            c.bus.write32(STREAM + 0x128, NODE);
                            c.bus.write32(STREAM + 0x12c, NODE);
                            c.bus.write32(STREAM + 0x130, 1);
                            c.bus.write32(NODE, 0);
                            c.bus.write32(FILE, HEADER);
                            c.bus.write32(HEADER, 0x4d52_5453); // "STRM"
                            c.bus.write32(HEADER + 0x38, 1);
                            c.bus.write32(HEADER + 0x64, 0);
                            c.bus.write32(HEADER + 0x68, REQ);
                            c.bus.write32(REQ + 0xc, 0);
                        }
                    }
                }
            },
        );
    }

    /// MEM_Pool16Alloc: a free block (last or not) is popped; an empty list
    /// falls back to the real MEM_PoolAlloc with pools ready or not (the
    /// fixture of `mem_pool64_alloc_matches`).
    #[test]
    fn mem_pool16_alloc_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA, GAME_GP};
        use crate::game::bx::cubicspline::tests::oc238_run;
        check(
            ee::s10::f_003202f8__generated,
            mem_pool16_alloc,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32_02f8, 0);
                let pool16 = DATA + 0x20_0000;
                if seed % 3 != 0 {
                    let block = DATA + 0x30_0000 + 0x10 * (r.next_u32() % 16);
                    c.bus.write32(pool16 + MEM_POOL16_FREE, block);
                    c.bus
                        .write32(block, if seed % 2 == 0 { 0 } else { block + 0x10 });
                } else {
                    c.bus.write32(pool16 + MEM_POOL16_FREE, 0);
                    oc238_run(
                        c,
                        0x0031_ff60,
                        ee::s10::f_0031ff60,
                        &[0x004f_f250, (DATA + 0x40000) as u64, 0x40000],
                    );
                    if seed % 2 == 0 {
                        c.bus.write32(GAME_GP + 0xdd8, 1);
                        const POOLS: [(u32, u32, u32); 7] = [
                            (0x2a70, 0x800, 0x40),
                            (0x2a60, 0x2000, 0x40),
                            (0x2a6c, 0x6000, 0x40),
                            (0x2a7c, 0x32000, 0x40),
                            (0x2a84, 0x25800, 0x80),
                            (0x2a78, 0x20000, 0x100),
                            (0x2a68, 0x20000, 0x200),
                        ];
                        let mut pool = DATA + 0xc_0000;
                        let mut base = DATA + 0x8_0000;
                        for (slot, head, stride) in POOLS {
                            for n in 0..64u32 {
                                let node = base + stride * n;
                                c.bus.write32(node, if n == 63 { 0 } else { node + stride });
                            }
                            c.bus.write32(pool + head, base);
                            c.bus.write32(GAME_GP + slot, pool);
                            pool += 0x100;
                            base += 0x8000;
                        }
                    } else {
                        c.bus.write32(GAME_GP + 0xdd8, 0);
                    }
                }
                c.g[A0][0] = sx(pool16);
                c.g[A1][0] = [0u64, 4, 8, 16, 32, 64, 128][(r.next_u32() % 7) as usize];
                c.pc = 0x0032_02f8;
            },
        );
    }

    /// WScriptMan_SaveState: a fake stream (its Write unwinds at some calls),
    /// empty process lists (the real WScriptMan_SaveProcessList), and 0..50
    /// live object tables whose real cLunoTable_Write takes its no-hash path;
    /// the real cWScriptCache_GetData reads the world's cache.
    #[test]
    fn wscript_man_save_state_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::DATA;
        const SELF: u32 = DATA;
        const WORLD: u32 = DATA + 0x1000;
        const CACHE: u32 = DATA + 0x1100;
        const CACHE_DATA: u32 = DATA + 0x1200;
        const OBJ: u32 = DATA + 0x2000;
        const TABLE: u32 = DATA + 0x2100;
        const STREAM: u32 = DATA + 0x3000;
        covered(
            ee::s10::f_0030bb10__generated,
            wscript_man_save_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_bb10, 8);
                c.pc = 0x0030_bb10;
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(STREAM, vt);
                c.bus.write32(SELF + wsm_save::WORLD, WORLD);
                c.bus.write32(SELF + wsm_save::LIST_A, 0);
                c.bus.write32(SELF + wsm_save::LIST_B, 0);
                c.bus.write32(WORLD + 4, CACHE);
                c.bus.write32(CACHE + 4, CACHE_DATA);
                for i in 0..50 {
                    c.bus.write32(CACHE_DATA + 0xc * i + 8, r.next_u32());
                }
                // Each live object is a reference to a table without a hash
                // part (modulus -1).
                c.bus.write32(OBJ, TABLE);
                c.bus.write32(TABLE + 0xc, u32::MAX);
                let density = [0, 1, 4, 16][(seed % 4) as usize];
                for i in 0..50 {
                    let live = density != 0 && r.next_u32() % 16 < density;
                    c.bus.write32(SELF + wsm_save::OBJS + 4 * i, if live { OBJ } else { 0 });
                }
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(STREAM);
            },
        );
    }

    /// BE_GetRiderAltCharacter: riders 0..15 with character ids 0..39 and
    /// alternate bytes of both signs.
    #[test]
    fn be_get_rider_alt_character_matches() {
        use crate::difftest::{check, scramble, Rng};
        check(
            ee::s01::f_0014a0b0__generated,
            be_get_rider_alt_character,
            200,
            &|c, seed| {
                let mut r = Rng(0x0014_a0b0 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x0014_a0b0;
                let rider = r.next_u32() % 16;
                let id = r.next_u32() % 40;
                c.bus.write32(BE_RIDER_CHARACTER_IDS + 4 * rider, id);
                c.bus.write8(
                    BE_CHARACTERS + 0x1c * id + BE_CHARACTER_ALT,
                    r.next_u32() as u8,
                );
                c.g[A0][0] = if seed % 2 == 0 {
                    rider as u64
                } else {
                    ((r.next_u32() as u64) << 32) | rider as u64
                };
            },
        );
    }

    /// The rider-state SaveState forwarders: `a1` a stream whose vtable slot
    /// 8 is a fake (random delta; some calls unwind), `a0` the state.
    fn rider_state_save_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        use crate::difftest::fakes::{machine, vtable};
        use crate::difftest::DATA;
        let mut r = machine(c, seed, salt, 1);
        c.pc = entry;
        let stream = DATA + 0x800;
        c.g[A0][0] = sx(DATA + 0x400);
        c.g[A1][0] = sx(stream);
        let vt = vtable(c, &mut r, &[8]);
        c.bus.write32(stream, vt);
    }

    #[test]
    fn rider_air_state_save_state_matches() {
        crate::difftest::fakes::covered(
            ee::s01::f_0013acb0__generated,
            rider_air_state_save_state,
            60,
            &|c, seed| rider_state_save_setup(c, seed, 0x13_acb0, 0x0013_acb0),
        );
    }

    #[test]
    fn rider_natural_air_sub_save_state_matches() {
        crate::difftest::fakes::covered(
            ee::s00::f_0012fbf0__generated,
            rider_natural_air_sub_save_state,
            60,
            &|c, seed| rider_state_save_setup(c, seed, 0x12_fbf0, 0x0012_fbf0),
        );
    }

    /// `Particle_Save`: the real `ModObj_Save` with and without a modifier
    /// block (the fixture of `mod_obj_save_matches`); the stream's write
    /// slots are fakes, some of which unwind.
    #[test]
    fn particle_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::{fill, DATA};
        covered(
            ee::s11::f_003579a8__generated,
            particle_save,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_79a8, 7);
                c.pc = 0x0035_79a8;
                let (this, stream, block) = (DATA + 0x100, DATA + 0x400, DATA + 0x800);
                fill(c, &mut r, this, 0x10);
                c.bus
                    .write32(this + 0x1c, if seed % 3 == 0 { 0 } else { block });
                let vt = vtable(c, &mut r, &[8, 0x30]);
                c.bus.write32(stream, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
            },
        );
    }

    /// Emitter_RandomizeDescSeeds: the real Rand_NextMain on a random
    /// generator state; random f20 (saved and restored).
    #[test]
    fn emitter_randomize_desc_seeds_matches() {
        use crate::difftest::{check, fill, scramble, Rng, DATA};
        check(
            ee::s12::f_0036ccb8__generated,
            emitter_randomize_desc_seeds,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_ccb8 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                c.pc = 0x0036_ccb8;
                // Rand_NextMain's generator (0x500000 - 0xfe8).
                fill(c, &mut r, 0x004f_f018, 0x10);
                c.f[20] = r.edge_f32();
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    /// ModObj_dtor: no modifier block, or an empty one (no implementation,
    /// so the real ModObj_v49 does nothing) allocated from a dlmalloc arena
    /// and freed by the real ModifierBlock_dtor; the real WorldObj_dtor runs
    /// on the fixture of `world_obj_dtor_matches` (flags without the delete
    /// bit). The mutex syscalls are fakes; some unwind.
    #[test]
    fn mod_obj_dtor_matches() {
        use crate::difftest::fakes::{machine, LOG_COUNT, LOG_UNWIND_AT};
        use crate::difftest::{check, fill, DATA, GAME_GP};
        use crate::game::bx::cubicspline::tests::oc238_run;
        check(
            ee::s11::f_003553c0__generated,
            mod_obj_dtor,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_53c0, 0);
                let obj = DATA + 0x1000;
                let inst = DATA + 0x1100;
                fill(c, &mut r, obj, 8);
                c.bus.write16(obj + 0x10, (r.next_u32() % 8) as u16);
                if seed % 3 == 0 {
                    c.bus.write32(obj + 4, 0);
                } else {
                    c.bus.write32(obj + 4, DATA + 0x1200);
                    c.bus
                        .write32(obj + 8, if seed % 3 == 1 { 0 } else { DATA + 0x1300 });
                }
                c.bus
                    .write32(obj + 0x18, if seed % 7 == 0 { 0 } else { inst });
                c.bus.write32(inst + 0x8, r.next_u32());
                c.bus
                    .write32(inst + 0xc, if seed % 2 == 0 { obj } else { r.next_u32() });
                let block = if seed % 4 == 0 {
                    0
                } else {
                    // The game heap over test memory, pools off.
                    c.bus.write32(GAME_GP + 0xdd8, 0);
                    oc238_run(
                        c,
                        0x0031_ff60,
                        ee::s10::f_0031ff60,
                        &[0x004f_f250, (DATA + 0x40000) as u64, 0x40000],
                    );
                    let b = oc238_run(c, 0x0031_9e48, ee::s10::f_00319e48, &[0x40, 0]) as u32;
                    for i in 0..0x10 {
                        c.bus.write32(b + 4 * i, 0);
                    }
                    b
                };
                c.bus.write32(obj + MOD_OBJ_BLOCK, block);
                // Unwind at one of the next calls (the setup made some).
                if seed % 3 == 1 {
                    let n = c.bus.read32(LOG_COUNT);
                    c.bus.write32(LOG_UNWIND_AT, n + r.next_u32() % 6);
                }
                c.pc = 0x0035_53c0;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = (r.next_u32() & !1) as u64;
            },
        );
    }
}
