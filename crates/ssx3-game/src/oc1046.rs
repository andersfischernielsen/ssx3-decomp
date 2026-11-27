//! Staging module oc1046 (wave wave264): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1046`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002a_dac8, bank_monitor_base_dtor, "BankMonitor_BaseDtor"),
    (
        0x002a_df80,
        sound_manager_ref_stop_all,
        "SoundManagerRef_StopAll",
    ),
    (0x002a_dfc8, path_plat_alloc, "PATH_PlatAlloc"),
    (0x002a_e000, path_plat_free, "PATH_PlatFree"),
    (
        0x002a_ea38,
        path_plat_get_track_position,
        "PATH_PlatGetTrackPosition",
    ),
    (
        0x002a_eb40,
        path_plat_find_queued_by_id,
        "PATH_PlatFindQueuedById",
    ),
    (
        0x002a_f480,
        path_plat_get_track_handle,
        "PATH_PlatGetTrackHandle",
    ),
    (0x002a_f4e0, path_plat_set_aux_send, "PATH_PlatSetAuxSend"),
];

/// `D_004D3E98`: the sound paths' player table (`tPathPlayer*[]`), 4-byte slots.
#[cfg(test)]
const PATH_PLAYER_TABLE: u32 = 0x004d_3e98;

/// `PATH_PlatRefreshStatus` (0x2aefa8): refreshes a dirty player.
const PATH_PLAT_REFRESH_STATUS: Callee = Callee {
    addr: 0x002a_efa8,
    f: ee::s08::f_002aefa8,
};

/// `SndPlayParams_dtor` (0x2a97c0).
const SND_PLAY_PARAMS_DTOR: Callee = Callee {
    addr: 0x002a_97c0,
    f: ee::s08::f_002a97c0,
};

/// `BankMonitor_TimerDtor` (0x2ad860): one monitor instance's destructor.
const BANK_MONITOR_TIMER_DTOR: Callee = Callee {
    addr: 0x002a_d860,
    f: ee::s08::f_002ad860,
};

/// The sound system's destructor (`cSoundSys` dtor, 0x2adee8).
const SOUND_SYS_DTOR: Callee = Callee {
    addr: 0x002a_dee8,
    f: ee::s08::f_002adee8,
};

/// `BankMonitor` layout: the first word is its sound system (`mpVSoundSys`);
/// the monitor instances run from `+0x98` up to `+0x8d8` in 0x2c steps
/// (48 of them); `mVtbl` is at `+0x8d8`.
const BM_INSTANCES: u32 = 0x98;
const BM_INSTANCES_END: u32 = 0x8d8;
const BM_INSTANCE_SIZE: u32 = 0x2c;
/// `gMonitor` (gp + 0x688).
const BM_G_MONITOR: u32 = 0x688;

/// `void BankMonitor_BaseDtor(BankMonitor* self, int flags)` (0x2adac8):
/// the sound system's vtable is pointed at the bank monitor's (on flags 0, a
/// copy on the stack with its `this` adjusted), the 100 Hz client is removed,
/// the monitor instances are destroyed from the end down, then the play
/// options (`SndPlayParams_dtor`), the sound system if flags bit 1 is set, and
/// the object itself (`__builtin_delete`) if bit 0 is set.
pub fn bank_monitor_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, S0, 0x80);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3ba8));
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S5, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        c.g[A0][0] = 0x48_0000;
        frame.save(c, S3, 0x50);
        c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3bc0));
        frame.save(c, S4, 0x40);
        c.g[S5][0] = sx((c.g[S2][0] as u32).wrapping_add(BM_INSTANCES_END));
        frame.save_ra(c, 0x20);
        let this = c.g[S2][0] as u32;
        let monitor_vtbl = c.g[V0][0]; // cMonitor_vtbl (0x483ba8)
        Addr(this.wrapping_add(BM_INSTANCES_END)).sw(c, monitor_vtbl);
        c.g[V1][0] = Addr(this).lw(c);
        let sound_sys_vtbl = c.g[A3][0];
        Addr((c.g[V1][0] as u32).wrapping_add(0x4)).sw(c, sound_sys_vtbl);
        if c.g[S0][0] == 0 {
            // flags == 0: the sound system's vtable is a stack copy whose
            // entry 1 has `this` adjusted by the offset of the monitor.
            c.cycles += 16;
            c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x3bc0)).ld(c);
            c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x8)).ld(c);
            c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x10)).ld(c);
            c.g[A0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x18)).ld(c);
            c.g[A1][0] = Addr(this).lw(c);
            let (v0, v1, a2, a0) = (c.g[V0][0], c.g[V1][0], c.g[A2][0], c.g[A0][0]);
            Addr(frame.at(c, 0x8)).sd(c, v0);
            Addr(frame.at(c, 0x10)).sd(c, v1);
            Addr(frame.at(c, 0)).sd(c, a2);
            Addr(frame.at(c, 0x18)).sd(c, a0);
            let sp_val = c.g[SP][0];
            Addr((c.g[A1][0] as u32).wrapping_add(0x4)).sw(c, sp_val);
            c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x8)).lhu(c);
            c.g[V0][0] = Addr(this).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8dc));
            c.g[V0][0] = sx(this.wrapping_sub(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let delta = c.g[V1][0];
            Addr(frame.at(c, 0x8)).sh(c, delta);
        }

        // The 100 Hz client, then the monitor instances (end first).
        c.cycles += 8;
        c.g[A0][0] = 0x2b_0000;
        c.g[S4][0] = c.g[S0][0] & 0x1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2668));
        c.g[S1][0] = sx(this.wrapping_add(BM_INSTANCES));
        call_ptr(c, 0x003b_5948, 0x002a_db68)?;
        c.g[S3][0] = c.g[S0][0] & 0x2;
        let instances_null = c.g[S1][0] == 0;
        Addr((c.g[GP][0] as u32).wrapping_add(BM_G_MONITOR)).sw(c, 0);
        // `tail` is the fall-through of the instance block: a0 = self, +1 cycle.
        let mut tail = true;
        if !instances_null {
            c.cycles += 3;
            c.g[S0][0] = c.g[S5][0];
            let empty = c.g[S1][0] == c.g[S0][0];
            c.g[A0][0] = c.g[S2][0];
            if empty {
                tail = false;
            } else {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(BM_INSTANCE_SIZE));
                loop {
                    c.cycles += 5;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    BANK_MONITOR_TIMER_DTOR.call(c, 0x002a_db94)?;
                    if c.g[S1][0] != c.g[S0][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(BM_INSTANCE_SIZE));
                        if event_due(c) {
                            deliver_event_here(c, 0x002a_db88);
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        if tail {
            c.cycles += 1;
            c.g[A0][0] = c.g[S2][0];
        }

        // The play options, then the sound system (flags bit 1).
        c.cycles += 4;
        c.g[A1][0] = 0;
        SND_PLAY_PARAMS_DTOR.call(c, 0x002a_dba8)?;
        if c.g[S3][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(this).lw(c);
            c.g[A1][0] = 0;
            SOUND_SYS_DTOR.call(c, 0x002a_dbbc)?;
        }

        // The object itself (flags bit 0): `__builtin_delete(self)`.
        c.cycles += 2;
        let no_delete = c.g[S4][0] == 0;
        frame.restore(c, S0, 0x80);
        if !no_delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0031_7e50, 0x002a_dbcc)?;
            frame.restore(c, S0, 0x80);
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void SoundManagerRef_StopAll(SoundManagerRef* self, int a)` (0x2adf80):
/// calls the object's vtable slot `+0x14` (adjusted by its `+0x10` delta) with
/// the object as `a0`.
pub fn sound_manager_ref_stop_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x10)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x14)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002a_dfa0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *PATH_PlatAlloc(int size)` (0x2adfc8): `size` bytes from the sound
/// path allocator (`gp + 0x69c`, called with the name `str_pathxSND`, `size`,
/// 0), or null when no allocator is set.
pub fn path_plat_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x69c)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[V0][0] = 0;
        if c.g[V1][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = 0x48_0000;
            c.g[A2][0] = 0;
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3288));
            call_ptr(c, target, 0x002a_dff0)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PATH_PlatFree(void* block)` (0x2ae000): `MEM_Free`, the block passed
/// through in `a0`.
pub fn path_plat_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0025_23a8, 0x002a_e010)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PATH_PlatGetTrackPosition(int idx)` (0x2aea38): the player's buffered
/// amount (`+0x50`), after refreshing it if it is dirty. A null player
/// (`idx` slot empty) gives the address of its `+0x3e98` slot, as the generated
/// body does (the read through it is the caller's problem).
pub fn path_plat_get_track_position(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = 0x4d_0000;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[V1][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3e98));
        c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3e98));
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x65)).lb(c);
            let clean = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3e98));
            if !clean {
                c.cycles += 3;
                c.g[A0][0] = 0;
                PATH_PLAT_REFRESH_STATUS.call(c, 0x002a_ea80)?;
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3e98));
            }
        }
        c.cycles += 9;
        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.restore(c, S0, 0x20);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        frame.restore(c, S1, 0x10);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x50)).lw(c);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PATH_PlatFindQueuedById(int idx, int key)` (0x2aeb40): the position of
/// the request whose handle (`+4`) is `key`, in the player's request list
/// (`+0x6c`, 12-byte records, count `+0x66`), after refreshing the player if it
/// is dirty; -8 when there is none. The list walk tests the count as signed.
pub fn path_plat_find_queued_by_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x4d_0000;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e98));
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = Addr(c.g[A0][0] as u32).lw(c);
        let null = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        // A clean player's count is read at once; otherwise (null or dirty)
        // the count is read after the (dirty) refresh.
        let mut count_read = false;
        if !null {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x65)).lb(c);
            if c.g[V0][0] == 0 {
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x66)).lb(c);
                count_read = true;
            } else {
                c.cycles += 2;
                c.g[A0][0] = 0;
                PATH_PLAT_REFRESH_STATUS.call(c, 0x002a_eb80)?;
            }
        }
        if !count_read {
            c.cycles += 1;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x66)).lb(c);
        }

        // The walk over the requests: found → v0 = its position (+8).
        let mut found = false;
        'walk: {
            c.cycles += 2;
            let no_requests = (c.g[V0][0] as i64) <= 0;
            c.g[V1][0] = 0;
            if no_requests {
                break 'walk;
            }
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x6c)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4)).lw(c);
                if c.g[V0][0] != c.g[S1][0] {
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x8)).lw(c);
                    found = true;
                    break 'walk;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x002a_eb98);
                    }
                } else {
                    break;
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_fff8;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PATH_PlatGetTrackHandle(int idx)` (0x2af480): the player's word at
/// `+0x14` (the reference calls it the dry level), or -8 for an empty slot.
/// A leaf: no calls.
pub fn path_plat_get_track_handle(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x4d_0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e98));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
    let empty = c.g[A0][0] == 0;
    c.g[V0][0] = 0xffff_ffff_ffff_fff8;
    if empty {
        c.cycles += 2;
    } else {
        c.cycles += 2;
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x14)).lw(c);
    }
    abi::ret(c);
}

/// `int PATH_PlatSetAuxSend(int idx, int effectBus, int sendLevel)` (0x2af4e0):
/// the player's send to `effectBus` at `sendLevel`, inside the sound critical
/// section: the bank player's (`+0x64`) sends go through the bank function
/// (0x3b88c8), the others through 0x3b8050; the level is kept at `+0x10`.
/// Returns the send's result, or -8 for an empty slot.
pub fn path_plat_set_aux_send(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x4d_0000;
        frame.save(c, S1, 0x30);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        frame.save(c, S2, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e98));
        frame.save(c, S3, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A2][0];
        c.g[S0][0] = Addr(c.g[A0][0] as u32).lw(c);
        let null = c.g[S0][0] == 0;
        c.g[S1][0] = 0xffff_ffff_ffff_fff8;
        if !null {
            c.cycles += 5;
            call_ptr(c, 0x003b_58a0, 0x002a_f524)?;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x64)).lb(c);
            let plain = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[S3][0];
            let mut bank_send = false;
            if !plain {
                c.cycles += 5;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x003b_88c8, 0x002a_f53c)?;
                bank_send = true;
            }
            if !bank_send {
                c.cycles += 3;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x003b_8050, 0x002a_f550)?;
            }
            c.cycles += 3;
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, 0x003b_58d8, 0x002a_f558)?;
            let level = c.g[S2][0];
            Addr((c.g[S0][0] as u32).wrapping_add(0x10)).sw(c, level);
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate of this module's fakes: a word in the machine's RAM, written by
    /// the fixtures that want the fakes to answer (else the previous body runs).
    const OC1046_CTRL: u32 = DATA + 0x2f_0000;
    const OC1046_MAGIC: u32 = 0x1046_c0de;

    fn oc1046_gate_on(c: &mut Cpu) {
        c.bus.write32(OC1046_CTRL, OC1046_MAGIC);
    }

    /// A callee answered by a gated fake: logs the call and returns, or runs
    /// the body the slot held before (`fakes::prev_cell` of the fake) when the
    /// gate is off.
    fn oc1046_gated(c: &mut Cpu, me: fn(&mut Cpu), generated: fn(&mut Cpu), target: u32) {
        if c.bus.read32(OC1046_CTRL) != OC1046_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(me), generated);
        }
        fakes::fake_call(c, target);
    }

    fn oc1046_fake_mem_free(c: &mut Cpu) {
        oc1046_gated(
            c,
            oc1046_fake_mem_free,
            ee::s06::f_002523a8__generated,
            0x0025_23a8,
        );
    }

    fn oc1046_fake_timer_dtor(c: &mut Cpu) {
        oc1046_gated(
            c,
            oc1046_fake_timer_dtor,
            ee::s08::f_002ad860__generated,
            0x002a_d860,
        );
    }

    fn oc1046_fake_sndplay_dtor(c: &mut Cpu) {
        oc1046_gated(
            c,
            oc1046_fake_sndplay_dtor,
            ee::s08::f_002a97c0__generated,
            0x002a_97c0,
        );
    }

    fn oc1046_fake_sound_sys_dtor(c: &mut Cpu) {
        oc1046_gated(
            c,
            oc1046_fake_sound_sys_dtor,
            ee::s08::f_002adee8__generated,
            0x002a_dee8,
        );
    }

    fn oc1046_fake_refresh(c: &mut Cpu) {
        oc1046_gated(
            c,
            oc1046_fake_refresh,
            ee::s08::f_002aefa8__generated,
            0x002a_efa8,
        );
    }

    fn oc1046_fake_remove_100hz(c: &mut Cpu) {
        oc1046_gated(
            c,
            oc1046_fake_remove_100hz,
            ee::s13::f_003b5948__generated,
            0x003b_5948,
        );
    }

    fn oc1046_fake_enter_critical(c: &mut Cpu) {
        oc1046_gated(
            c,
            oc1046_fake_enter_critical,
            ee::s13::f_003b58a0__generated,
            0x003b_58a0,
        );
    }

    fn oc1046_fake_leave_critical(c: &mut Cpu) {
        oc1046_gated(
            c,
            oc1046_fake_leave_critical,
            ee::s13::f_003b58d8__generated,
            0x003b_58d8,
        );
    }

    fn oc1046_fake_bank_send(c: &mut Cpu) {
        oc1046_gated(
            c,
            oc1046_fake_bank_send,
            ee::s13::f_003b88c8__generated,
            0x003b_88c8,
        );
    }

    fn oc1046_fake_track_send(c: &mut Cpu) {
        oc1046_gated(
            c,
            oc1046_fake_track_send,
            ee::s13::f_003b8050__generated,
            0x003b_8050,
        );
    }

    fn oc1046_fake_delete(c: &mut Cpu) {
        oc1046_gated(
            c,
            oc1046_fake_delete,
            ee::s10::f_00317e50__generated,
            0x0031_7e50,
        );
    }

    /// Install the gated fakes (once per process).
    fn oc1046_install() {
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002523A8),
            oc1046_fake_mem_free,
            fakes::prev_cell(oc1046_fake_mem_free),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AD860),
            oc1046_fake_timer_dtor,
            fakes::prev_cell(oc1046_fake_timer_dtor),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002A97C0),
            oc1046_fake_sndplay_dtor,
            fakes::prev_cell(oc1046_fake_sndplay_dtor),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ADEE8),
            oc1046_fake_sound_sys_dtor,
            fakes::prev_cell(oc1046_fake_sound_sys_dtor),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AEFA8),
            oc1046_fake_refresh,
            fakes::prev_cell(oc1046_fake_refresh),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B5948),
            oc1046_fake_remove_100hz,
            fakes::prev_cell(oc1046_fake_remove_100hz),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B58A0),
            oc1046_fake_enter_critical,
            fakes::prev_cell(oc1046_fake_enter_critical),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B58D8),
            oc1046_fake_leave_critical,
            fakes::prev_cell(oc1046_fake_leave_critical),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B88C8),
            oc1046_fake_bank_send,
            fakes::prev_cell(oc1046_fake_bank_send),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B8050),
            oc1046_fake_track_send,
            fakes::prev_cell(oc1046_fake_track_send),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1046_fake_delete,
            fakes::prev_cell(oc1046_fake_delete),
        );
    }

    /// The BankMonitor under test, its sound system, and the gp-relative
    /// `gMonitor` word lives in the game image.
    const MONITOR: u32 = DATA + 0x20_0000;
    const SOUND_SYS: u32 = DATA + 0x2f_1800;

    #[test]
    fn bank_monitor_base_dtor_matches() {
        oc1046_install();
        check(
            ee::s08::f_002adac8__generated,
            bank_monitor_base_dtor,
            240,
            &|c, seed| {
                machine(c, seed, 0x2a_dac8, 0);
                oc1046_gate_on(c);
                c.g[A0][0] = sx(MONITOR);
                // Flags 0..3: the copy path (0), the sound system's dtor (bit
                // 1) and the delete (bit 0) each on and off.
                c.g[A1][0] = sx(seed % 4);
                c.bus.write32(MONITOR, SOUND_SYS);
            },
        );
    }

    /// The sound manager object of `SoundManagerRef_StopAll`: its first word
    /// is the vtable object, whose slot `+0x10` is the delta and `+0x14` the
    /// function. The object lives outside the random vtable area.
    const SOUND_MANAGER_REF: u32 = DATA + 0x2f_1000;
    const SOUND_MANAGER_OBJ: u32 = DATA + 0x2f_1100;

    #[test]
    fn sound_manager_ref_stop_all_matches() {
        check(
            ee::s08::f_002adf80__generated,
            sound_manager_ref_stop_all,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_df80, 0);
                oc1046_gate_on(c);
                let vt = fakes::vtable(c, &mut r, &[0x10]);
                c.bus.write32(SOUND_MANAGER_OBJ, vt);
                c.bus.write32(SOUND_MANAGER_REF, SOUND_MANAGER_OBJ);
                c.g[A0][0] = sx(SOUND_MANAGER_REF);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    /// The sound path allocator pointer at `gp + 0x69c`: empty on even seeds
    /// (null result), else a fake (logged) allocator.
    #[test]
    fn path_plat_alloc_matches() {
        oc1046_install();
        check(
            ee::s08::f_002adfc8__generated,
            path_plat_alloc,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_dfc8, 0);
                oc1046_gate_on(c);
                c.g[A0][0] = sx(r.next_u32() & 0xffff);
                let alloc = if seed % 2 == 0 { 0 } else { FAKE + 0x80 };
                c.bus.write32(GAME_GP + 0x69c, alloc);
            },
        );
    }

    #[test]
    fn path_plat_free_matches() {
        oc1046_install();
        check(
            ee::s08::f_002ae000__generated,
            path_plat_free,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_e000, 0);
                oc1046_gate_on(c);
                c.g[A0][0] = sx(DATA + 0x2f_2000 + 0x10 * (r.next_u32() & 0xf));
            },
        );
    }

    /// The sound path player table: slot `k` holds a player (or null on some
    /// seeds). Each player: bank flag `+0x64`, dirty flag `+0x65`, request
    /// count `+0x66` (signed), request list `+0x6c`, buffered `+0x50`, dry
    /// level `+0x14`. Requests are 12 bytes: handle `+4`, position `+8`.
    const PLAYER_BASE: u32 = DATA + 0x2f_2000;
    const REQUESTS: u32 = DATA + 0x2f_4000;

    fn player_world(c: &mut Cpu, r: &mut Rng, seed: u32) {
        for k in 0..8u32 {
            let p = PLAYER_BASE + 0x100 * k;
            let null = k == seed % 8 && seed % 5 == 0;
            c.bus
                .write32(PATH_PLAYER_TABLE + 4 * k, if null { 0 } else { p });
            c.bus.write8(p + 0x64, (seed as u8) & 1);
            c.bus.write8(p + 0x65, if seed % 3 == 0 { 1 } else { 0 });
            let count = if seed % 11 == 10 {
                0x80
            } else {
                (seed % 7) as u8
            };
            c.bus.write8(p + 0x66, count);
            let list = REQUESTS + 0x100 * k;
            c.bus.write32(p + 0x6c, list);
            for i in 0..8u32 {
                let handle = 0x100 + 0x10 * k + i;
                c.bus.write32(list + 12 * i + 4, handle);
                c.bus.write32(list + 12 * i + 8, r.next_u32());
            }
            c.bus.write32(p + 0x50, r.next_u32());
            c.bus.write32(p + 0x14, r.next_u32());
        }
    }

    #[test]
    fn path_plat_get_track_position_matches() {
        oc1046_install();
        check(
            ee::s08::f_002aea38__generated,
            path_plat_get_track_position,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_ea38, 0);
                oc1046_gate_on(c);
                player_world(c, &mut r, seed);
                c.g[A0][0] = sx(seed % 8);
            },
        );
    }

    #[test]
    fn path_plat_find_queued_by_id_matches() {
        oc1046_install();
        check(
            ee::s08::f_002aeb40__generated,
            path_plat_find_queued_by_id,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_eb40, 0);
                oc1046_gate_on(c);
                player_world(c, &mut r, seed);
                let k = seed % 8;
                c.g[A0][0] = sx(k);
                // Key: a request of this player (found) or not (-8).
                let key = if seed % 4 == 3 {
                    0x0fff
                } else {
                    0x100 + 0x10 * k + (seed % 8)
                };
                c.g[A1][0] = sx(key);
            },
        );
    }

    #[test]
    fn path_plat_get_track_handle_matches() {
        check(
            ee::s08::f_002af480__generated,
            path_plat_get_track_handle,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_f480, 0);
                player_world(c, &mut r, seed);
                c.g[A0][0] = sx(seed % 8);
            },
        );
    }

    #[test]
    fn path_plat_set_aux_send_matches() {
        oc1046_install();
        check(
            ee::s08::f_002af4e0__generated,
            path_plat_set_aux_send,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_f4e0, 0);
                oc1046_gate_on(c);
                player_world(c, &mut r, seed);
                c.g[A0][0] = sx(seed % 8);
                c.g[A1][0] = sx(r.next_u32() & 0xff);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }
}
