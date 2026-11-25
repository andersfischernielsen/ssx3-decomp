//! `src/lib/cdvd.cpp`: hand-written CD/DVD (`libcdvd_*`, `sceCd*`) replacements.

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0040_0810, libcdvd_alarm_handler, "libcdvd_AlarmHandler"),
    (0x0040_0838, libcdvd_delay_thread, "libcdvd_DelayThread"),
    (0x0040_08a0, sce_cd_callback, "sceCdCallback"),
    (
        0x0040_0908,
        libcdvd_callback_cmd_handler,
        "libcdvd_CallbackCmdHandler",
    ),
    (0x0040_0a78, sce_cd_init_ee_cb, "sceCdInitEeCB"),
    (0x0040_0c00, libcdvd_sema_init, "libcdvd_SemaInit"),
    (0x0040_0cb8, libcdvd_exit, "libcdvd_Exit"),
    (0x0040_0d58, sce_cd_poff_callback, "sceCdPOffCallback"),
    (
        0x0040_0dd8,
        libcdvd_poff_cmd_handler,
        "libcdvd_POffCmdHandler",
    ),
    (
        0x0040_0e30,
        libcdvd_install_poff_handler,
        "libcdvd_InstallPOffHandler",
    ),
    (
        0x0040_0ea8,
        sce_cd_layer_search_file,
        "sceCdLayerSearchFile",
    ),
    (0x0040_11c0, sce_cd_search_file, "sceCdSearchFile"),
    (0x0040_11e0, libcdvd_ncmd_pre_check, "libcdvd_NCmdPreCheck"),
    (
        0x0040_1350,
        libcdvd_ncmd_disk_ready,
        "libcdvd_NCmdDiskReady",
    ),
    (0x0040_13e8, sce_cd_sync, "sceCdSync"),
    (0x0040_1488, sce_cd_sync_s, "sceCdSyncS"),
    (0x0040_14f8, libcdvd_scmd_pre_check, "libcdvd_SCmdPreCheck"),
    (0x0040_1718, sce_cd_init, "sceCdInit"),
    (0x0040_19f8, libcdvd_disk_ready_old, "libcdvd_DiskReadyOld"),
    (0x0040_1bf0, sce_cd_disk_ready, "sceCdDiskReady"),
    (0x0040_1df8, sce_cd_read, "sceCdRead"),
    (0x0040_1fd8, sce_cd_get_toc, "sceCdGetToc"),
    (0x0040_2268, sce_cd_get_disk_type, "sceCdGetDiskType"),
    (0x0040_2300, sce_cd_get_error, "sceCdGetError"),
    (0x0040_2398, sce_cd_power_off, "sceCdPowerOff"),
    (0x0040_2450, sce_cd_mmode, "sceCdMmode"),
    (0x0040_2520, sce_cd_read_clock, "sceCdReadClock"),
    (0x0040_2618, sce_cd_tray_req, "sceCdTrayReq"),
];

// ---------------------------------------------------------------------------
// Shared direct-call targets (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `libcdvd_DelayThread` (`0x0040_0838`).
    pub(crate) const CD_DELAY_THREAD: Callee = Callee {
        addr: 0x0040_0838,
        f: ee::s15::libcdvd_DelayThread,
    };

    /// `libcdvd_SemaInit` (`0x0040_0c00`).
    pub(crate) const SEMA_INIT: Callee = Callee {
        addr: 0x0040_0c00,
        f: ee::s15::libcdvd_SemaInit,
    };

    /// `libcdvd_Exit` (`0x0040_0cb8`).
    pub(crate) const CD_EXIT: Callee = Callee {
        addr: 0x0040_0cb8,
        f: ee::s15::libcdvd_Exit,
    };

    /// `libcdvd_InstallPOffHandler` (`0x0040_0e30`).
    pub(crate) const INSTALL_POFF: Callee = Callee {
        addr: 0x0040_0e30,
        f: ee::s15::libcdvd_InstallPOffHandler,
    };

    /// `sceCdLayerSearchFile` (`0x0040_0ea8`).
    pub(crate) const LAYER_SEARCH_FILE: Callee = Callee {
        addr: 0x0040_0ea8,
        f: ee::s15::sceCdLayerSearchFile,
    };

    /// `libcdvd_NCmdPreCheck` (`0x0040_11e0`).
    pub(crate) const NCMD_PRE_CHECK: Callee = Callee {
        addr: 0x0040_11e0,
        f: ee::s15::libcdvd_NCmdPreCheck,
    };

    /// `libcdvd_NCmdDiskReady` (`0x0040_1350`).
    pub(crate) const NCMD_DISK_READY: Callee = Callee {
        addr: 0x0040_1350,
        f: ee::s15::libcdvd_NCmdDiskReady,
    };

    /// `sceCdSync` (`0x0040_13e8`).
    pub(crate) const CD_SYNC: Callee = Callee {
        addr: 0x0040_13e8,
        f: ee::s15::sceCdSync,
    };

    /// `sceCdSyncS` (`0x0040_1488`).
    pub(crate) const SYNC_S: Callee = Callee {
        addr: 0x0040_1488,
        f: ee::s15::sceCdSyncS,
    };

    /// `libcdvd_SCmdPreCheck` (`0x0040_14f8`).
    pub(crate) const SCMD_PRE_CHECK: Callee = Callee {
        addr: 0x0040_14f8,
        f: ee::s15::libcdvd_SCmdPreCheck,
    };

    /// `libcdvd_DiskReadyOld` (`0x0040_19f8`).
    pub(crate) const DISK_READY_OLD: Callee = Callee {
        addr: 0x0040_19f8,
        f: ee::s15::libcdvd_DiskReadyOld,
    };

    /// `SetAlarm` (`0x0042_3b20`).
    pub(crate) const SET_ALARM: Callee = Callee {
        addr: 0x0042_3b20,
        f: ee::s15::SetAlarm,
    };

    /// `CreateThread` (`0x0042_3ba0`).
    pub(crate) const CREATE_THREAD: Callee = Callee {
        addr: 0x0042_3ba0,
        f: ee::s15::CreateThread,
    };

    /// `_StartThread` (`0x0042_3bc0`).
    pub(crate) const START_THREAD: Callee = Callee {
        addr: 0x0042_3bc0,
        f: ee::s15::_StartThread,
    };

    /// `ChangeThreadPriority` (`0x0042_3c30`).
    pub(crate) const CHANGE_THREAD_PRIORITY: Callee = Callee {
        addr: 0x0042_3c30,
        f: ee::s15::ChangeThreadPriority,
    };

    pub(crate) use crate::game::lib::gcc::GET_THREAD_ID;

    /// `ReferThreadStatus` (`0x0042_3ca0`).
    pub(crate) const REFER_THREAD_STATUS: Callee = Callee {
        addr: 0x0042_3ca0,
        f: ee::s15::ReferThreadStatus,
    };

    pub(super) use crate::game::lib::kernel::callee::CREATE_SEMA;

    pub(super) use crate::game::lib::sif::callee::DELETE_SEMA;

    pub(super) use crate::game::lib::mc::callee::SIGNAL_SEMA;

    /// `iSignalSema` (`0x0042_3dd0`).
    pub(crate) const ISIGNAL_SEMA: Callee = Callee {
        addr: 0x0042_3dd0,
        f: ee::s15::iSignalSema,
    };

    pub(super) use crate::game::lib::usbkb::callee::WAIT_SEMA;

    pub(super) use crate::game::lib::mc::callee::POLL_SEMA;

    pub(super) use crate::game::lib::sif::callee::PRINTF;

    /// `sceSifAddCmdHandler` (`0x0042_6020`).
    pub(crate) const ADD_CMD_HANDLER: Callee = Callee {
        addr: 0x0042_6020,
        f: ee::s15::sceSifAddCmdHandler,
    };

    /// `sceSifRemoveCmdHandler` (`0x0042_6050`).
    pub(crate) const REMOVE_CMD_HANDLER: Callee = Callee {
        addr: 0x0042_6050,
        f: ee::s15::sceSifRemoveCmdHandler,
    };

    pub(super) use crate::game::lib::sif::callee::WRITEBACK;

    pub(super) use crate::game::lib::sif::callee::INIT_RPC;

    pub(super) use crate::game::lib::sif::callee::BIND_RPC;

    pub(super) use crate::game::lib::sif::callee::CALL_RPC;

    /// `sceSifCheckStatRpc` (`0x0042_6f08`).
    pub(crate) const CHECK_STAT: Callee = Callee {
        addr: 0x0042_6f08,
        f: ee::s15::sceSifCheckStatRpc,
    };

    pub(crate) use crate::game::lib::kernel::DINTR;

    pub(super) use crate::game::lib::sif::callee::EINTR;
}

/// `libcdvd_AlarmHandler(sema, ...)` (0x400810): the delay alarm: signals
/// `a2` at interrupt level and returns its result.
pub fn libcdvd_alarm_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.sd(frame.at(c, 0), c.g[RA][0]);
        c.g[A0][0] = c.g[A2][0];
        callee::ISIGNAL_SEMA.call(c, 0x0040_0820)?;
        c.ei();
        c.g[RA][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `libcdvd_CallbackCmdHandler(cmd)` (0x400908): the CDVD RPC callback
/// dispatcher. With `*cmd == 0xb` it clears the pending-callback words;
/// otherwise it signals the callback sema, and when a callback is
/// registered (`[0x5264c0]`), its sema too.
pub fn libcdvd_callback_cmd_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0xb;
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[S0][0] = 0x45_0000;
        c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[A0][0] = 0x45_0000;
        Addr((c.g[S0][0] as u32).wrapping_add(0x1658)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x1658)).lw(c);
        Addr((c.g[A0][0] as u32).wrapping_add(0x165c)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x1658)).lw(c);
        // V0 is 0x450000 on both arms (the generated body sets it before
        // testing cmd == 0xb); the signal arm reads its sema id from
        // [V0 + 0x1628] = [0x451628].
        c.g[V0][0] = 0x45_0000;
        if c.g[V1][0] == c.g[A1][0] {
            c.cycles += 5;
            Addr((c.g[S0][0] as u32).wrapping_add(0x1658)).sw(c, 0);
            c.g[V0][0] = 0x45_0000;
            Addr((c.g[V0][0] as u32).wrapping_add(0x1634)).sw(c, 0);
            c.g[RA][0] = c.ld(frame.at(c, 0x10));
        } else {
            c.cycles += 7;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1628)).lw(c);
            callee::ISIGNAL_SEMA.call(c, 0x0040_095c)?;
            c.g[V1][0] = 0x45_0000;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1614)).lw(c);
            // Only the double-signal path skips the `[0x451634] = 0`
            // store below.
            let mut second = false;
            let pending = c.g[V0][0] != 0;
            // Fixed base 0x520000 (delay slot, both arms): this reads
            // [0x5264c0], not [cb + 0x64c0].
            c.g[V0][0] = 0x52_0000;
            if pending {
                c.cycles += 3;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x64c0)).lw(c);
                c.g[V0][0] = 0x45_0000;
                if c.g[V1][0] != 0 {
                    c.cycles += 5;
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1620)).lw(c);
                    callee::ISIGNAL_SEMA.call(c, 0x0040_0984)?;
                    second = true;
                }
            }
            if !second {
                c.cycles += 2;
                c.g[V0][0] = 0x45_0000;
                Addr((c.g[V0][0] as u32).wrapping_add(0x1634)).sw(c, 0);
            }
            c.cycles += 2;
            Addr((c.g[S0][0] as u32).wrapping_add(0x1658)).sw(c, 0);
            c.g[RA][0] = c.ld(frame.at(c, 0x10));
        }
        c.cycles += 3;
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `libcdvd_DelayThread(usec)` (0x400838): blocks `usec & 0xffff`
/// microseconds with an alarm plus a semaphore (create, set, wait,
/// delete). Returns the `DeleteSema` result.
pub fn libcdvd_delay_thread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 1;
        c.sd(frame.at(c, 0x30), c.g[S1][0]);
        c.g[S1][0] = c.g[A0][0] & 0xffff;
        c.sd(frame.at(c, 0x20), c.g[S0][0]);
        Addr(frame.at(c, 4)).sw(c, c.g[V0][0]);
        Addr(frame.at(c, 8)).sw(c, 0);
        Addr(frame.at(c, 0x14)).sw(c, 0);
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        c.g[A0][0] = c.g[SP][0];
        callee::CREATE_SEMA.call(c, 0x0040_0864)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0x40_0000;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x810));
        callee::SET_ALARM.call(c, 0x0040_087c)?;
        c.g[A0][0] = c.g[S0][0];
        callee::WAIT_SEMA.call(c, 0x0040_0884)?;
        c.g[A0][0] = c.g[S0][0];
        callee::DELETE_SEMA.call(c, 0x0040_088c)?;
        c.g[RA][0] = c.ld(frame.at(c, 0x40));
        c.g[S1][0] = c.ld(frame.at(c, 0x30));
        c.g[S0][0] = c.ld(frame.at(c, 0x20));
        frame.close(c);
        Ok(())
    })
}

/// `int libcdvd_DiskReadyOld(int mode)` (0x4019f8): the older disk-ready
/// path used before the N-command client exists. Syncs, binds the old RPC
/// client and issues the request. Returns the reply word, 6 while the drive
/// is not ready (-1 when `mode == 8` and the drive reports not-ready).
pub fn libcdvd_disk_ready_old(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0040_19f8 => {
                    c.cycles += 14;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S6][0]);
                    c.g[S6][0] = 0x450000;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S2][0]);
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x1610));
                    c.g[S2][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S7][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S5][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S4][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S3][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S1][0]);
                    let debug = (c.g[V0][0] as i64) <= 0;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                    if debug {
                        pc = 0x0040_1a3c;
                    } else {
                        pc = 0x0040_1a30;
                    }
                }
                0x0040_1a30 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x490000;
                    c.g[RA][0] = sx(0x0040_1a3c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7b48));
                    callee::PRINTF.call(c, 0x0040_1a3c)?;
                    pc = 0x0040_1a3c;
                }
                0x0040_1a3c => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x0040_1a44);
                    c.g[S3][0] = 0x450000;
                    callee::SEMA_INIT.call(c, 0x0040_1a44)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x162c));
                    c.g[RA][0] = sx(0x0040_1a50);
                    callee::POLL_SEMA.call(c, 0x0040_1a50)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x162c));
                    let sema_busy = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x6;
                    if sema_busy {
                        pc = 0x0040_1bc4;
                    } else {
                        pc = 0x0040_1a5c;
                    }
                }
                0x0040_1a5c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0040_1a64);
                    c.g[A0][0] = 0x1;
                    callee::SYNC_S.call(c, 0x0040_1a64)?;
                    let sync_fail = c.g[V0][0] != 0;
                    c.g[A0][0] = 0u64;
                    if sync_fail {
                        pc = 0x0040_1b7c;
                    } else {
                        pc = 0x0040_1a6c;
                    }
                }
                0x0040_1a6c => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0040_1a74);
                    c.g[S1][0] = 0x450000;
                    callee::INIT_RPC.call(c, 0x0040_1a74)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1648));
                    let bound = (c.g[V0][0] as i64) >= 0;
                    c.g[S5][0] = 0x520000;
                    if bound {
                        pc = 0x0040_1b2c;
                    } else {
                        pc = 0x0040_1a80;
                    }
                }
                0x0040_1a80 => {
                    c.cycles += 3;
                    c.g[S4][0] = 0x520000;
                    c.g[S7][0] = 0x450000;
                    pc = 0x0040_1ab4;
                }
                0x0040_1a8c => {
                    c.cycles += 1;
                    pc = 0x0040_1a90;
                }
                0x0040_1a90 => {
                    c.cycles += 2;
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    pc = 0x0040_1a98;
                }
                0x0040_1a98 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let more = c.g[V0][0] != c.g[V1][0];
                    if more {
                        if back_edge_unwinds(c, 0x0040_1a98) {
                            return Ok(());
                        }
                        pc = 0x0040_1a98;
                    } else {
                        pc = 0x0040_1ab4;
                    }
                }
                0x0040_1ab4 => {
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x6710));
                    pc = 0x0040_1ab8;
                }
                0x0040_1ab8 => {
                    c.cycles += 7;
                    c.g[A1][0] = 0xffff_ffff_8000_0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] |= 0x59a;
                    c.g[RA][0] = sx(0x0040_1acc);
                    c.g[A2][0] = 0u64;
                    callee::BIND_RPC.call(c, 0x0040_1acc)?;
                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                        pc = 0x0040_1b1c;
                    } else {
                        pc = 0x0040_1ad4;
                    }
                }
                0x0040_1ad4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x1610));
                    let debug = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = 0x100000;
                    if debug {
                        pc = 0x0040_1af0;
                    } else {
                        pc = 0x0040_1ae0;
                    }
                }
                0x0040_1ae0 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x490000;
                    c.g[RA][0] = sx(0x0040_1aec);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7b60));
                    callee::PRINTF.call(c, 0x0040_1aec)?;
                    c.g[V0][0] = 0x100000;
                    pc = 0x0040_1af0;
                }
                0x0040_1af0 => {
                    c.cycles += 2;
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    pc = 0x0040_1af8;
                }
                0x0040_1af8 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let more = c.g[V0][0] != c.g[V1][0];
                    if more {
                        if back_edge_unwinds(c, 0x0040_1af8) {
                            return Ok(());
                        }
                        pc = 0x0040_1af8;
                    } else {
                        pc = 0x0040_1b14;
                    }
                }
                0x0040_1b14 => {
                    c.cycles += 2;
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x6710));
                    if back_edge_unwinds(c, 0x0040_1ab8) {
                        return Ok(());
                    }
                    pc = 0x0040_1ab8;
                }
                0x0040_1b1c => {
                    c.cycles += 2;
                    let bound_done = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x100000;
                    if bound_done {
                        if back_edge_unwinds(c, 0x0040_1a90) {
                            return Ok(());
                        }
                        pc = 0x0040_1a90;
                    } else {
                        pc = 0x0040_1b24;
                    }
                }
                0x0040_1b24 => {
                    c.cycles += 2;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1648), 0u64);
                    pc = 0x0040_1b34;
                }
                0x0040_1b2c => {
                    c.cycles += 2;
                    c.g[S4][0] = 0x520000;
                    c.g[S7][0] = 0x450000;
                    pc = 0x0040_1b34;
                }
                0x0040_1b34 => {
                    c.cycles += 18;
                    c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6750));
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x6750), c.g[S2][0]);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0040_1b48);
                    c.g[A1][0] = 0x4;
                    callee::WRITEBACK.call(c, 0x0040_1b48)?;
                    c.g[S1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x2800));
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x6710));
                    c.g[A3][0] = c.g[S0][0];
                    c.sw(c.g[SP][0] as u32, 0u64);
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    c.g[T0][0] = 0x4;
                    c.g[T1][0] = c.g[S1][0];
                    c.g[T2][0] = 0x4;
                    c.g[RA][0] = sx(0x0040_1b74);
                    c.g[T3][0] = 0u64;
                    callee::CALL_RPC.call(c, 0x0040_1b74)?;
                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x1610));
                        pc = 0x0040_1b9c;
                    } else {
                        pc = 0x0040_1b7c;
                    }
                }
                0x0040_1b7c => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x162c));
                    c.g[RA][0] = sx(0x0040_1b88);
                    callee::SIGNAL_SEMA.call(c, 0x0040_1b88)?;
                    c.g[A0][0] = c.g[S2][0] ^ 0x8;
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    c.g[V0][0] = 0x6;
                    if c.g[A0][0] == 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    pc = 0x0040_1bc4;
                }
                0x0040_1b9c => {
                    c.cycles += 2;
                    let quiet = (c.g[V0][0] as i64) <= 0;
                    c.g[A0][0] = 0x490000;
                    if quiet {
                        pc = 0x0040_1bac;
                    } else {
                        pc = 0x0040_1ba4;
                    }
                }
                0x0040_1ba4 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_1bac);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7b80));
                    callee::PRINTF.call(c, 0x0040_1bac)?;
                    pc = 0x0040_1bac;
                }
                0x0040_1bac => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x2000_0000;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x162c));
                    c.g[V0][0] |= c.g[S1][0];
                    c.g[RA][0] = sx(0x0040_1bc0);
                    c.g[S0][0] = c.lw(c.g[V0][0] as u32);
                    callee::SIGNAL_SEMA.call(c, 0x0040_1bc0)?;
                    c.g[V0][0] = c.g[S0][0];
                    pc = 0x0040_1bc4;
                }
                0x0040_1bc4 => {
                    c.cycles += 11;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0040_19f8, 0x0040_1bf0);
                    return Ok(());
                }
            }
        }
    })
}

/// `void libcdvd_Exit(void)` (0x400cb8): tear the lib down — delete the
/// four semaphores and remove the power-off command handler, re-enabling
/// interrupts when they were on.
pub fn libcdvd_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[V0][0] = 0x450000;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1614));
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        let already_down = c.g[V1][0] == 0;
        c.sd(frame.at(c, 0), c.g[S0][0]);
        if !already_down {
            c.cycles += 8;
            c.g[V1][0] = 0x450000;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1658), c.g[V0][0]);
            c.g[S0][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_0ce8);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1620));
            callee::SIGNAL_SEMA.call(c, 0x0040_0ce8)?;
            c.g[V0][0] = 0x450000;
        } else {
            c.cycles += 2;
            c.g[S0][0] = 0x450000;
            c.g[V0][0] = 0x450000;
        }
        c.cycles += 18;
        c.g[RA][0] = sx(0x0040_0d00);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1628));
        callee::DELETE_SEMA.call(c, 0x0040_0d00)?;
        c.g[V1][0] = 0x450000;
        c.g[RA][0] = sx(0x0040_0d0c);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x162c));
        callee::DELETE_SEMA.call(c, 0x0040_0d0c)?;
        c.g[V1][0] = 0x450000;
        c.g[RA][0] = sx(0x0040_0d18);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1630));
        callee::DELETE_SEMA.call(c, 0x0040_0d18)?;
        c.g[RA][0] = sx(0x0040_0d20);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1620));
        callee::DELETE_SEMA.call(c, 0x0040_0d20)?;
        c.g[RA][0] = sx(0x0040_0d28);
        callee::DINTR.call(c, 0x0040_0d28)?;
        c.g[A0][0] = 0xffff_ffff_8000_0000;
        c.g[S0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0040_0d38);
        c.g[A0][0] |= 0x12;
        callee::REMOVE_CMD_HANDLER.call(c, 0x0040_0d38)?;
        let ints_were_off = c.g[S0][0] == 0;
        frame.restore_ra(c, 0x10);
        if ints_were_off {
            c.cycles += 3;
            c.g[S0][0] = c.ld(frame.at(c, 0));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[S0][0] = c.ld(frame.at(c, 0));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        c.pc = callee::EINTR.addr;
        (callee::EINTR.f)(c);
        Ok(())
    })
}

/// `int libcdvd_InstallPOffHandler(void)` (0x400e30): raise the
/// power-off handler flag and install `libcdvd_POffCmdHandler` as the SIF
/// command handler, re-enabling interrupts when they were on. Returns 1.
pub fn libcdvd_install_poff_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S2][0] = 0x1;
        c.g[S1][0] = 0x450000;
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x30), c.g[RA][0]);
        c.g[RA][0] = sx(0x0040_0e54);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1624), c.g[S2][0]);
        callee::DINTR.call(c, 0x0040_0e54)?;
        c.g[A1][0] = 0x400000;
        c.g[A0][0] = 0xffff_ffff_8000_0000;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xdd8));
        c.g[A0][0] |= 0x12;
        c.g[RA][0] = sx(0x0040_0e70);
        c.g[A2][0] = 0u64;
        callee::ADD_CMD_HANDLER.call(c, 0x0040_0e70)?;
        let ints_were_off = c.g[S0][0] == 0;
        c.g[V0][0] = 0x450000;
        if !ints_were_off {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0040_0e80);
            callee::EINTR.call(c, 0x0040_0e80)?;
            c.g[V0][0] = 0x450000;
        }
        c.cycles += 9;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1624), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1640), c.g[S2][0]);
        frame.restore_ra(c, 0x30);
        c.g[V0][0] = 0x1;
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int libcdvd_NCmdDiskReady(void)` (0x401350): run the N-command
/// precheck, then issue the disk-ready RPC and return its reply word.
/// Returns 0 when the precheck or the RPC fails.
pub fn libcdvd_ncmd_disk_ready(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x2;
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        c.g[RA][0] = sx(0x0040_1364);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        callee::NCMD_PRE_CHECK.call(c, 0x0040_1364)?;
        let pre_ok = c.g[V0][0] != 0;
        c.g[V0][0] = 0x450000;
        if !pre_ok {
            c.cycles += 2;
            c.g[V0][0] = 0u64;
            c.cycles += 4;
            frame.restore_ra(c, 0x20);
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 14;
        c.g[A0][0] = 0x450000;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1680));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x27d0));
        c.sw(frame.at(c, 0), 0u64);
        c.g[A1][0] = 0xe;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = 0x4;
        c.g[RA][0] = sx(0x0040_13a4);
        c.g[T3][0] = 0u64;
        callee::CALL_RPC.call(c, 0x0040_13a4)?;
        let rpc_ok = (c.g[V0][0] as i64) >= 0;
        c.g[V1][0] = 0x450000;
        if !rpc_ok {
            c.cycles += 5;
            c.g[V0][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_13b8);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1628));
            callee::SIGNAL_SEMA.call(c, 0x0040_13b8)?;
            c.g[V0][0] = 0u64;
            c.cycles += 4;
            frame.restore_ra(c, 0x20);
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 6;
        c.g[V0][0] = 0x2000_0000;
        c.g[V0][0] |= c.g[S0][0];
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1628));
        c.g[RA][0] = sx(0x0040_13d4);
        c.g[S0][0] = c.lw(c.g[V0][0] as u32);
        callee::SIGNAL_SEMA.call(c, 0x0040_13d4)?;
        c.g[V0][0] = c.g[S0][0];
        c.cycles += 4;
        frame.restore_ra(c, 0x20);
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int libcdvd_NCmdPreCheck(int cmd)` (0x4011e0): latch `cmd`, poll the
/// N-command semaphore and sync once. Returns 1 when the RPC client is
/// ready (binding it first when the `0x45163c` word says it is not).
pub fn libcdvd_ncmd_pre_check(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0040_11e0 => {
                    c.cycles += 14;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                    c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                    c.g[S1][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.g[S0][0] = 0x450000;
                    c.g[RA][0] = sx(0x0040_1200);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
                    callee::SEMA_INIT.call(c, 0x0040_1200)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1628));
                    c.g[RA][0] = sx(0x0040_120c);
                    callee::POLL_SEMA.call(c, 0x0040_120c)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1628));
                    let sema_free = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x450000;
                    if sema_free {
                        pc = 0x0040_1240;
                    } else {
                        pc = 0x0040_1218;
                    }
                }
                0x0040_1218 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1610));
                    let debug = (c.g[V1][0] as i64) <= 0;
                    c.g[V0][0] = 0x450000;
                    if debug {
                        pc = 0x0040_1278;
                    } else {
                        pc = 0x0040_1224;
                    }
                }
                0x0040_1224 => {
                    c.cycles += 7;
                    c.g[A0][0] = 0x490000;
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x161c));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7a78));
                    c.g[RA][0] = sx(0x0040_1238);
                    c.g[A1][0] = c.g[S1][0];
                    callee::PRINTF.call(c, 0x0040_1238)?;
                    c.g[V0][0] = 0u64;
                    pc = 0x0040_1338;
                }
                0x0040_1240 => {
                    c.cycles += 11;
                    c.g[V0][0] = 0x520000;
                    c.g[V1][0] = 0x450000;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64d8));
                    c.g[A1][0] = 0x520000;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x161c), c.g[S1][0]);
                    c.g[RA][0] = sx(0x0040_125c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x64e0));
                    callee::REFER_THREAD_STATUS.call(c, 0x0040_125c)?;
                    c.g[RA][0] = sx(0x0040_1264);
                    c.g[A0][0] = 0x1;
                    callee::CD_SYNC.call(c, 0x0040_1264)?;
                    let synced = c.g[V0][0] == 0;
                    c.g[A0][0] = 0u64;
                    if synced {
                        pc = 0x0040_1280;
                    } else {
                        pc = 0x0040_126c;
                    }
                }
                0x0040_126c => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1628));
                    c.g[RA][0] = sx(0x0040_1278);
                    callee::SIGNAL_SEMA.call(c, 0x0040_1278)?;
                    pc = 0x0040_1278;
                }
                0x0040_1278 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0u64;
                    pc = 0x0040_1338;
                }
                0x0040_1280 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0040_1288);
                    c.g[S2][0] = 0x450000;
                    callee::INIT_RPC.call(c, 0x0040_1288)?;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x163c));
                    let bound = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = 0x1;
                    if bound {
                        pc = 0x0040_1338;
                    } else {
                        pc = 0x0040_1294;
                    }
                }
                0x0040_1294 => {
                    c.cycles += 2;
                    c.g[S1][0] = 0x450000;
                    pc = 0x0040_12c4;
                }
                0x0040_129c => {
                    c.cycles += 1;
                    pc = 0x0040_12a0;
                }
                0x0040_12a0 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x100000;
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    pc = 0x0040_12a8;
                }
                0x0040_12a8 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let more = c.g[V0][0] != c.g[V1][0];
                    if more {
                        if back_edge_unwinds(c, 0x0040_12a8) {
                            return Ok(());
                        }
                        pc = 0x0040_12a8;
                    } else {
                        pc = 0x0040_12c4;
                    }
                }
                0x0040_12c4 => {
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x27d0));
                    pc = 0x0040_12c8;
                }
                0x0040_12c8 => {
                    c.cycles += 7;
                    c.g[A1][0] = 0xffff_ffff_8000_0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] |= 0x595;
                    c.g[RA][0] = sx(0x0040_12dc);
                    c.g[A2][0] = 0u64;
                    callee::BIND_RPC.call(c, 0x0040_12dc)?;
                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                        pc = 0x0040_132c;
                    } else {
                        pc = 0x0040_12e4;
                    }
                }
                0x0040_12e4 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x450000;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1610));
                    let debug = (c.g[V1][0] as i64) <= 0;
                    c.g[V0][0] = 0x100000;
                    if debug {
                        pc = 0x0040_1304;
                    } else {
                        pc = 0x0040_12f4;
                    }
                }
                0x0040_12f4 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x490000;
                    c.g[RA][0] = sx(0x0040_1300);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7aa0));
                    callee::PRINTF.call(c, 0x0040_1300)?;
                    c.g[V0][0] = 0x100000;
                    pc = 0x0040_1304;
                }
                0x0040_1304 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    pc = 0x0040_1308;
                }
                0x0040_1308 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let more = c.g[V0][0] != c.g[V1][0];
                    if more {
                        if back_edge_unwinds(c, 0x0040_1308) {
                            return Ok(());
                        }
                        pc = 0x0040_1308;
                    } else {
                        pc = 0x0040_1324;
                    }
                }
                0x0040_1324 => {
                    c.cycles += 2;
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x27d0));
                    if back_edge_unwinds(c, 0x0040_12c8) {
                        return Ok(());
                    }
                    pc = 0x0040_12c8;
                }
                0x0040_132c => {
                    c.cycles += 2;
                    let bound_done = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x1;
                    if bound_done {
                        if back_edge_unwinds(c, 0x0040_12a0) {
                            return Ok(());
                        }
                        pc = 0x0040_12a0;
                    } else {
                        pc = 0x0040_1334;
                    }
                }
                0x0040_1334 => {
                    c.cycles += 1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x163c), 0u64);
                    pc = 0x0040_1338;
                }
                0x0040_1338 => {
                    c.cycles += 6;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0040_11e0, 0x0040_1350);
                    return Ok(());
                }
            }
        }
    })
}

/// `void libcdvd_POffCmdHandler(void)` (0x400dd8): the power-off SIF command
/// handler. When a callback is installed (`*0x5264c8`) and the `0x451624`
/// word is clear, calls it with `*0x5264d0` as its argument and the saved
/// `gp` (`*0x5264cc`) loaded, restoring `gp` after. Returns no value of its
/// own: `v0` is the callback's result or scratch.
pub fn libcdvd_poff_cmd_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[V0][0] = 0x520000;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64c8));
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        let no_callback = c.g[A1][0] == 0;
        c.sd(frame.at(c, 0), c.g[S0][0]);
        if no_callback {
            c.cycles += 1;
            frame.restore_ra(c, 0x10);
            c.cycles += 3;
            c.g[S0][0] = c.ld(frame.at(c, 0));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        c.g[V0][0] = 0x450000;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1624));
        let suppress = c.g[V1][0] != 0;
        frame.restore_ra(c, 0x10);
        if suppress {
            c.cycles += 3;
            c.g[S0][0] = c.ld(frame.at(c, 0));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 7;
        c.g[V0][0] = 0x520000;
        c.g[S0][0] = c.g[GP][0];
        c.g[GP][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64cc));
        c.g[V1][0] = 0x520000;
        let target = c.g[A1][0] as u32;
        c.g[RA][0] = sx(0x0040_0e18);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64d0));
        call_ptr(c, target, 0x0040_0e18)?;
        c.g[GP][0] = c.g[S0][0];
        c.cycles += 1;
        frame.restore_ra(c, 0x10);
        c.cycles += 3;
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int libcdvd_SCmdPreCheck(int cmd)` (0x4014f8): latch `cmd`, poll the
/// S-command semaphore and sync once. Returns 1 when the RPC client is
/// ready (binding it first when the `0x45164c` word says it is not).
pub fn libcdvd_scmd_pre_check(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0040_14f8 => {
                    c.cycles += 14;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                    c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                    c.g[S1][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.g[S0][0] = 0x450000;
                    c.g[RA][0] = sx(0x0040_1518);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
                    callee::SEMA_INIT.call(c, 0x0040_1518)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x162c));
                    c.g[RA][0] = sx(0x0040_1524);
                    callee::POLL_SEMA.call(c, 0x0040_1524)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x162c));
                    let sema_free = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x450000;
                    if sema_free {
                        pc = 0x0040_1558;
                    } else {
                        pc = 0x0040_1530;
                    }
                }
                0x0040_1530 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1610));
                    let debug = (c.g[V1][0] as i64) <= 0;
                    c.g[V0][0] = 0x450000;
                    if debug {
                        pc = 0x0040_1590;
                    } else {
                        pc = 0x0040_153c;
                    }
                }
                0x0040_153c => {
                    c.cycles += 7;
                    c.g[A0][0] = 0x490000;
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1618));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7ad8));
                    c.g[RA][0] = sx(0x0040_1550);
                    c.g[A1][0] = c.g[S1][0];
                    callee::PRINTF.call(c, 0x0040_1550)?;
                    c.g[V0][0] = 0u64;
                    pc = 0x0040_1650;
                }
                0x0040_1558 => {
                    c.cycles += 11;
                    c.g[V0][0] = 0x520000;
                    c.g[V1][0] = 0x450000;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64d8));
                    c.g[A1][0] = 0x520000;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x1618), c.g[S1][0]);
                    c.g[RA][0] = sx(0x0040_1574);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x64e0));
                    callee::REFER_THREAD_STATUS.call(c, 0x0040_1574)?;
                    c.g[RA][0] = sx(0x0040_157c);
                    c.g[A0][0] = 0x1;
                    callee::SYNC_S.call(c, 0x0040_157c)?;
                    let synced = c.g[V0][0] == 0;
                    c.g[A0][0] = 0u64;
                    if synced {
                        pc = 0x0040_1598;
                    } else {
                        pc = 0x0040_1584;
                    }
                }
                0x0040_1584 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x162c));
                    c.g[RA][0] = sx(0x0040_1590);
                    callee::SIGNAL_SEMA.call(c, 0x0040_1590)?;
                    pc = 0x0040_1590;
                }
                0x0040_1590 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0u64;
                    pc = 0x0040_1650;
                }
                0x0040_1598 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0040_15a0);
                    c.g[S2][0] = 0x450000;
                    callee::INIT_RPC.call(c, 0x0040_15a0)?;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x164c));
                    let bound = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = 0x1;
                    if bound {
                        pc = 0x0040_1650;
                    } else {
                        pc = 0x0040_15ac;
                    }
                }
                0x0040_15ac => {
                    c.cycles += 2;
                    c.g[S1][0] = 0x450000;
                    pc = 0x0040_15dc;
                }
                0x0040_15b4 => {
                    c.cycles += 1;
                    pc = 0x0040_15b8;
                }
                0x0040_15b8 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x100000;
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    pc = 0x0040_15c0;
                }
                0x0040_15c0 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let more = c.g[V0][0] != c.g[V1][0];
                    if more {
                        if back_edge_unwinds(c, 0x0040_15c0) {
                            return Ok(());
                        }
                        pc = 0x0040_15c0;
                    } else {
                        pc = 0x0040_15dc;
                    }
                }
                0x0040_15dc => {
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3180));
                    pc = 0x0040_15e0;
                }
                0x0040_15e0 => {
                    c.cycles += 7;
                    c.g[A1][0] = 0xffff_ffff_8000_0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] |= 0x593;
                    c.g[RA][0] = sx(0x0040_15f4);
                    c.g[A2][0] = 0u64;
                    callee::BIND_RPC.call(c, 0x0040_15f4)?;
                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                        pc = 0x0040_1644;
                    } else {
                        pc = 0x0040_15fc;
                    }
                }
                0x0040_15fc => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x450000;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1610));
                    let debug = (c.g[V1][0] as i64) <= 0;
                    c.g[V0][0] = 0x100000;
                    if debug {
                        pc = 0x0040_161c;
                    } else {
                        pc = 0x0040_160c;
                    }
                }
                0x0040_160c => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x490000;
                    c.g[RA][0] = sx(0x0040_1618);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7b00));
                    callee::PRINTF.call(c, 0x0040_1618)?;
                    c.g[V0][0] = 0x100000;
                    pc = 0x0040_161c;
                }
                0x0040_161c => {
                    c.cycles += 1;
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    pc = 0x0040_1620;
                }
                0x0040_1620 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let more = c.g[V0][0] != c.g[V1][0];
                    if more {
                        if back_edge_unwinds(c, 0x0040_1620) {
                            return Ok(());
                        }
                        pc = 0x0040_1620;
                    } else {
                        pc = 0x0040_163c;
                    }
                }
                0x0040_163c => {
                    c.cycles += 2;
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3180));
                    if back_edge_unwinds(c, 0x0040_15e0) {
                        return Ok(());
                    }
                    pc = 0x0040_15e0;
                }
                0x0040_1644 => {
                    c.cycles += 2;
                    let bound_done = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x1;
                    if bound_done {
                        if back_edge_unwinds(c, 0x0040_15b8) {
                            return Ok(());
                        }
                        pc = 0x0040_15b8;
                    } else {
                        pc = 0x0040_164c;
                    }
                }
                0x0040_164c => {
                    c.cycles += 1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x164c), 0u64);
                    pc = 0x0040_1650;
                }
                0x0040_1650 => {
                    c.cycles += 6;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0040_14f8, 0x0040_1668);
                    return Ok(());
                }
            }
        }
    })
}

/// `int libcdvd_SemaInit(void)` (0x400c00): create the four libcdvd
/// semaphores when the cached ids are no longer valid. Returns nonzero
/// once they exist.
pub fn libcdvd_sema_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.sd(frame.at(c, 0x30), c.g[S1][0]);
        c.g[S1][0] = 0x450000;
        c.sd(frame.at(c, 0x50), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1628));
        c.sd(frame.at(c, 0x40), c.g[S2][0]);
        let first_invalid = c.g[V0][0] == c.g[V1][0];
        c.sd(frame.at(c, 0x20), c.g[S0][0]);
        if first_invalid {
            c.cycles += 1;
            c.g[S0][0] = 0x450000;
            c.cycles += 2;
            c.g[S2][0] = 0x450000;
            c.g[V0][0] = 0x1;
        } else {
            c.cycles += 4;
            c.g[S0][0] = 0x450000;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x162c));
            let second_invalid = c.g[V0][0] == c.g[V1][0];
            c.g[S2][0] = 0x450000;
            if second_invalid {
                c.cycles += 2;
                c.g[S2][0] = 0x450000;
                c.g[V0][0] = 0x1;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1630));
                let third_valid = c.g[V0][0] != c.g[V1][0];
                frame.restore_ra(c, 0x50);
                if third_valid {
                    c.cycles += 5;
                    c.g[S2][0] = c.ld(frame.at(c, 0x40));
                    c.g[S1][0] = c.ld(frame.at(c, 0x30));
                    c.g[S0][0] = c.ld(frame.at(c, 0x20));
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 2;
                c.g[V0][0] = 0x1;
            }
        }
        c.cycles += 20;
        c.sw(frame.at(c, 0x14), 0u64);
        c.sw(frame.at(c, 0x4), c.g[V0][0]);
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x0040_0c68);
        c.sw(frame.at(c, 0x8), c.g[V0][0]);
        callee::CREATE_SEMA.call(c, 0x0040_0c68)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x0040_0c74);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1628), c.g[V0][0]);
        callee::CREATE_SEMA.call(c, 0x0040_0c74)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x0040_0c80);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x162c), c.g[V0][0]);
        callee::CREATE_SEMA.call(c, 0x0040_0c80)?;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x1630), c.g[V0][0]);
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x0040_0c90);
        c.sw(frame.at(c, 0x8), 0u64);
        callee::CREATE_SEMA.call(c, 0x0040_0c90)?;
        c.g[V1][0] = 0x450000;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1620), c.g[V0][0]);
        c.g[V0][0] = 0x450000;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1634), 0u64);
        frame.restore_ra(c, 0x50);
        c.cycles += 5;
        c.g[S2][0] = c.ld(frame.at(c, 0x40));
        c.g[S1][0] = c.ld(frame.at(c, 0x30));
        c.g[S0][0] = c.ld(frame.at(c, 0x20));
        frame.close(c);
        Ok(())
    })
}

/// `sceCdCallback(cb)` (0x4008a0): installs `cb` as the CD callback.
/// Busy (`sceCdSync(1)` nonzero) returns 0; otherwise, with interrupts
/// held, it swaps `[0x5264c0]` and records `gp` next to it, re-enables
/// interrupts when they were on, and returns the previous callback.
pub fn sce_cd_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[A0][0] = 1;
        callee::CD_SYNC.call(c, 0x0040_08bc)?;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 9;
            callee::DINTR.call(c, 0x0040_08cc)?;
            c.g[V1][0] = 0x52_0000;
            c.g[A0][0] = 0x52_0000;
            c.g[S0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x64c0)).lw(c);
            Addr((c.g[V1][0] as u32).wrapping_add(0x64c0)).sw(c, c.g[S1][0]);
            Addr((c.g[A0][0] as u32).wrapping_add(0x64c4)).sw(c, c.g[GP][0]);
            let was_enabled = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S0][0];
            if was_enabled {
                c.cycles += 3;
                callee::EINTR.call(c, 0x0040_08f0)?;
                c.g[V0][0] = c.g[S0][0];
            }
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int sceCdDiskReady(int mode)` (0x401bf0): poll the N-command semaphore,
/// bind the N-command RPC client, and issue the disk-ready RPC. Returns the
/// reply word, 6 while the drive is not ready (-1 when `mode == 8` and the
/// drive reports not-ready), falling back to `libcdvd_DiskReadyOld` after
/// repeated bind retries.
pub fn sce_cd_disk_ready(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0040_1bf0 => {
                    c.cycles += 13;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S6][0]);
                    c.g[S6][0] = 0x450000;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S2][0]);
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x1610));
                    c.g[S2][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[RA][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S5][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S4][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S3][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S1][0]);
                    let debug = (c.g[V0][0] as i64) <= 0;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                    if debug {
                        pc = 0x0040_1c30;
                    } else {
                        pc = 0x0040_1c24;
                    }
                }
                0x0040_1c24 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x490000;
                    c.g[RA][0] = sx(0x0040_1c30);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7b98));
                    callee::PRINTF.call(c, 0x0040_1c30)?;
                    pc = 0x0040_1c30;
                }
                0x0040_1c30 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x0040_1c38);
                    c.g[S4][0] = 0x450000;
                    callee::SEMA_INIT.call(c, 0x0040_1c38)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1630));
                    c.g[RA][0] = sx(0x0040_1c44);
                    callee::POLL_SEMA.call(c, 0x0040_1c44)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1630));
                    let sema_busy = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = c.g[S2][0] ^ 0x8;
                    if sema_busy {
                        pc = 0x0040_1d78;
                    } else {
                        pc = 0x0040_1c50;
                    }
                }
                0x0040_1c50 => {
                    c.cycles += 6;
                    c.g[A0][0] = 0u64;
                    c.g[RA][0] = sx(0x0040_1c5c);
                    c.g[S3][0] = 0x450000;
                    callee::INIT_RPC.call(c, 0x0040_1c5c)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1648));
                    let bound = (c.g[V0][0] as i64) >= 0;
                    c.g[S1][0] = 0u64;
                    if bound {
                        pc = 0x0040_1d14;
                    } else {
                        pc = 0x0040_1c68;
                    }
                }
                0x0040_1c68 => {
                    c.cycles += 2;
                    c.g[S5][0] = 0x520000;
                    pc = 0x0040_1ca0;
                }
                0x0040_1c70 => {
                    c.cycles += 2;
                    let retries_done = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if retries_done {
                        pc = 0x0040_1d88;
                    } else {
                        pc = 0x0040_1c78;
                    }
                }
                0x0040_1c78 => {
                    c.cycles += 2;
                    c.g[V1][0] = 0x100000;
                    c.g[A0][0] = 0xffff_ffff_ffff_ffff;
                    pc = 0x0040_1c80;
                }
                0x0040_1c80 => {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let more = c.g[V1][0] != c.g[A0][0];
                    if more {
                        if back_edge_unwinds(c, 0x0040_1c80) {
                            return Ok(());
                        }
                        pc = 0x0040_1c80;
                    } else {
                        pc = 0x0040_1c9c;
                    }
                }
                0x0040_1c9c => {
                    c.cycles += 1;
                    c.g[S1][0] = c.g[V0][0];
                    pc = 0x0040_1ca0;
                }
                0x0040_1ca0 => {
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x6710));
                    pc = 0x0040_1ca4;
                }
                0x0040_1ca4 => {
                    c.cycles += 7;
                    c.g[A1][0] = 0xffff_ffff_8000_0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] |= 0x59c;
                    c.g[RA][0] = sx(0x0040_1cb8);
                    c.g[A2][0] = 0u64;
                    callee::BIND_RPC.call(c, 0x0040_1cb8)?;
                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                        pc = 0x0040_1d04;
                    } else {
                        pc = 0x0040_1cc0;
                    }
                }
                0x0040_1cc0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x1610));
                    let debug = (c.g[V0][0] as i64) <= 0;
                    c.g[V1][0] = 0x100000;
                    if debug {
                        pc = 0x0040_1cdc;
                    } else {
                        pc = 0x0040_1ccc;
                    }
                }
                0x0040_1ccc => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x490000;
                    c.g[RA][0] = sx(0x0040_1cd8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7b60));
                    callee::PRINTF.call(c, 0x0040_1cd8)?;
                    c.g[V1][0] = 0x100000;
                    pc = 0x0040_1cdc;
                }
                0x0040_1cdc => {
                    c.cycles += 1;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    pc = 0x0040_1ce0;
                }
                0x0040_1ce0 => {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let more = c.g[V1][0] != c.g[V0][0];
                    if more {
                        if back_edge_unwinds(c, 0x0040_1ce0) {
                            return Ok(());
                        }
                        pc = 0x0040_1ce0;
                    } else {
                        pc = 0x0040_1cfc;
                    }
                }
                0x0040_1cfc => {
                    c.cycles += 2;
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x6710));
                    if back_edge_unwinds(c, 0x0040_1ca4) {
                        return Ok(());
                    }
                    pc = 0x0040_1ca4;
                }
                0x0040_1d04 => {
                    c.cycles += 2;
                    let bound_done = c.g[V0][0] == 0u64;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 17i64) as u64;
                    if bound_done {
                        if back_edge_unwinds(c, 0x0040_1c70) {
                            return Ok(());
                        }
                        pc = 0x0040_1c70;
                    } else {
                        pc = 0x0040_1d0c;
                    }
                }
                0x0040_1d0c => {
                    c.cycles += 2;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1648), 0u64);
                    pc = 0x0040_1d18;
                }
                0x0040_1d14 => {
                    c.cycles += 1;
                    c.g[S5][0] = 0x520000;
                    pc = 0x0040_1d18;
                }
                0x0040_1d18 => {
                    c.cycles += 20;
                    c.g[V0][0] = 0x520000;
                    c.g[A1][0] = 0x4;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6750));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6750), c.g[S2][0]);
                    c.g[RA][0] = sx(0x0040_1d30);
                    c.g[A0][0] = c.g[S0][0];
                    callee::WRITEBACK.call(c, 0x0040_1d30)?;
                    c.g[V0][0] = 0x450000;
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x6710));
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3080));
                    c.g[A3][0] = c.g[S0][0];
                    c.sw(c.g[SP][0] as u32, 0u64);
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    c.g[T0][0] = 0x4;
                    c.g[T1][0] = c.g[S1][0];
                    c.g[T2][0] = 0x4;
                    c.g[RA][0] = sx(0x0040_1d60);
                    c.g[T3][0] = 0u64;
                    callee::CALL_RPC.call(c, 0x0040_1d60)?;
                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x1610));
                        pc = 0x0040_1da4;
                    } else {
                        pc = 0x0040_1d68;
                    }
                }
                0x0040_1d68 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1630));
                    c.g[RA][0] = sx(0x0040_1d74);
                    callee::SIGNAL_SEMA.call(c, 0x0040_1d74)?;
                    c.g[A0][0] = c.g[S2][0] ^ 0x8;
                    pc = 0x0040_1d78;
                }
                0x0040_1d78 => {
                    c.cycles += 4;
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    c.g[V0][0] = 0x6;
                    if c.g[A0][0] == 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    pc = 0x0040_1dcc;
                }
                0x0040_1d88 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1630));
                    c.g[RA][0] = sx(0x0040_1d94);
                    callee::SIGNAL_SEMA.call(c, 0x0040_1d94)?;
                    c.g[RA][0] = sx(0x0040_1d9c);
                    c.g[A0][0] = c.g[S2][0];
                    callee::DISK_READY_OLD.call(c, 0x0040_1d9c)?;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
                    pc = 0x0040_1dd0;
                }
                0x0040_1da4 => {
                    c.cycles += 2;
                    let quiet = (c.g[V0][0] as i64) <= 0;
                    c.g[A0][0] = 0x490000;
                    if quiet {
                        pc = 0x0040_1db4;
                    } else {
                        pc = 0x0040_1dac;
                    }
                }
                0x0040_1dac => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_1db4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7b80));
                    callee::PRINTF.call(c, 0x0040_1db4)?;
                    pc = 0x0040_1db4;
                }
                0x0040_1db4 => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x2000_0000;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1630));
                    c.g[V0][0] |= c.g[S1][0];
                    c.g[RA][0] = sx(0x0040_1dc8);
                    c.g[S0][0] = c.lw(c.g[V0][0] as u32);
                    callee::SIGNAL_SEMA.call(c, 0x0040_1dc8)?;
                    c.g[V0][0] = c.g[S0][0];
                    pc = 0x0040_1dcc;
                }
                0x0040_1dcc => {
                    c.cycles += 1;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
                    pc = 0x0040_1dd0;
                }
                0x0040_1dd0 => {
                    c.cycles += 9;
                    c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0040_1bf0, 0x0040_1df4);
                    return Ok(());
                }
            }
        }
    })
}

/// `int sceCdGetDiskType(void)` (0x402268): run the S-command precheck,
/// then issue the disk-type RPC and return its reply word. Returns 0 when
/// the precheck or the RPC fails.
pub fn sce_cd_get_disk_type(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x1;
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        c.g[RA][0] = sx(0x0040_227c);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        callee::SCMD_PRE_CHECK.call(c, 0x0040_227c)?;
        let pre_ok = c.g[V0][0] != 0;
        c.g[V0][0] = 0x450000;
        if !pre_ok {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 4;
            frame.restore_ra(c, 0x20);
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 14;
        c.g[A0][0] = 0x450000;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2800));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3180));
        c.sw(frame.at(c, 0), 0);
        c.g[A1][0] = 0x3;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = 0x4;
        c.g[RA][0] = sx(0x0040_22bc);
        c.g[T3][0] = 0;
        callee::CALL_RPC.call(c, 0x0040_22bc)?;
        let rpc_ok = (c.g[V0][0] as i64) >= 0;
        c.g[V1][0] = 0x450000;
        if !rpc_ok {
            c.cycles += 5;
            c.g[V0][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_22d0);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x162c));
            callee::SIGNAL_SEMA.call(c, 0x0040_22d0)?;
            c.g[V0][0] = 0;
            c.cycles += 4;
            frame.restore_ra(c, 0x20);
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 6;
        c.g[V0][0] = 0x2000_0000;
        c.g[V0][0] |= c.g[S0][0];
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x162c));
        c.g[RA][0] = sx(0x0040_22ec);
        c.g[S0][0] = c.lw(c.g[V0][0] as u32);
        callee::SIGNAL_SEMA.call(c, 0x0040_22ec)?;
        c.g[V0][0] = c.g[S0][0];
        c.cycles += 4;
        frame.restore_ra(c, 0x20);
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceCdGetError(void)` (0x402300): fetch the last CD/DVD error
/// word through the error RPC. Returns the error word, -1 when the
/// precheck/RPC fails.
pub fn sce_cd_get_error(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x3;
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        c.g[RA][0] = sx(0x0040_2314);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        callee::SCMD_PRE_CHECK.call(c, 0x0040_2314)?;
        let pre_ok = c.g[V0][0] != 0;
        c.g[V0][0] = 0x450000;
        if !pre_ok {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            c.cycles += 4;
            frame.restore_ra(c, 0x20);
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 14;
        c.g[A0][0] = 0x450000;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2800));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3180));
        c.sw(frame.at(c, 0), 0);
        c.g[A1][0] = 0x4;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = 0x4;
        c.g[RA][0] = sx(0x0040_2354);
        c.g[T3][0] = 0;
        callee::CALL_RPC.call(c, 0x0040_2354)?;
        let rpc_ok = (c.g[V0][0] as i64) >= 0;
        c.g[V1][0] = 0x450000;
        if !rpc_ok {
            c.cycles += 5;
            c.g[V0][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_2368);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x162c)).lw(c);
            callee::SIGNAL_SEMA.call(c, 0x0040_2368)?;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            c.cycles += 4;
            frame.restore_ra(c, 0x20);
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 6;
        c.g[V0][0] = 0x2000_0000;
        c.g[V0][0] |= c.g[S0][0];
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x162c)).lw(c);
        c.g[RA][0] = sx(0x0040_2384);
        c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
        callee::SIGNAL_SEMA.call(c, 0x0040_2384)?;
        c.g[V0][0] = c.g[S0][0];
        c.cycles += 4;
        frame.restore_ra(c, 0x20);
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceCdGetToc(u8 *toc)` (0x401fd8): run the N-command precheck, then
/// issue the TOC RPC into the `0x526780` buffer and copy the table to `toc`.
/// A zero reply word copies the first 0x400 bytes (aligned or unaligned loop
/// by pointer alignment); a nonzero reply replays the whole 0x800 bytes plus
/// a 16-byte tail. Returns the reply word, 0 when the precheck/RPC fails.
pub fn sce_cd_get_toc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x80);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x50), c.g[S4][0]);
        c.g[S0][0] = 0x450000;
        c.sd(frame.at(c, 0x70), c.g[RA][0]);
        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1700));
        c.sd(frame.at(c, 0x60), c.g[S5][0]);
        c.g[A0][0] = 0x8;
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[RA][0] = sx(0x0040_200c);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        callee::NCMD_PRE_CHECK.call(c, 0x0040_200c)?;
        let pre_ok = c.g[V0][0] != 0;
        c.g[V0][0] = 0x520000;
        if !pre_ok {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 9;
            frame.restore_ra(c, 0x70);
            c.g[S5][0] = c.ld(frame.at(c, 0x60));
            c.g[S4][0] = c.ld(frame.at(c, 0x50));
            c.g[S3][0] = c.ld(frame.at(c, 0x40));
            c.g[S2][0] = c.ld(frame.at(c, 0x30));
            c.g[S1][0] = c.ld(frame.at(c, 0x20));
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 26;
        c.g[A1][0] = 0x810;
        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6780));
        c.g[S5][0] = 0x450000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1700), c.g[S1][0]);
        c.g[RA][0] = sx(0x0040_2034);
        c.g[A0][0] = c.g[S1][0];
        callee::WRITEBACK.call(c, 0x0040_2034)?;
        c.g[S3][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1680));
        c.g[A0][0] = c.g[S4][0];
        c.g[RA][0] = sx(0x0040_2044);
        c.g[A1][0] = 0xc;
        callee::WRITEBACK.call(c, 0x0040_2044)?;
        c.g[S0][0] = 0x450000;
        c.g[V0][0] = 0x3;
        c.g[A0][0] = 0x450000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1658), c.g[V0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x27d0));
        c.g[A3][0] = c.g[S4][0];
        c.sw(frame.at(c, 0), 0);
        c.g[A1][0] = 0x4;
        c.g[A2][0] = 0;
        c.g[T0][0] = 0xc;
        c.g[T1][0] = c.g[S3][0];
        c.g[T2][0] = 0x8;
        c.g[RA][0] = sx(0x0040_207c);
        c.g[T3][0] = 0;
        callee::CALL_RPC.call(c, 0x0040_207c)?;
        if (c.g[V0][0] as i64) >= 0 {
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 6;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1658), 0u64);
            c.g[V0][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_2094);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1628));
            callee::SIGNAL_SEMA.call(c, 0x0040_2094)?;
            c.g[V0][0] = 0;
            c.cycles += 9;
            frame.restore_ra(c, 0x70);
            c.g[S5][0] = c.ld(frame.at(c, 0x60));
            c.g[S4][0] = c.ld(frame.at(c, 0x50));
            c.g[S3][0] = c.ld(frame.at(c, 0x40));
            c.g[S2][0] = c.ld(frame.at(c, 0x30));
            c.g[S1][0] = c.ld(frame.at(c, 0x20));
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        'b_402238: {
            'b_402164: {
                c.cycles += 3;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1658), 0u64);
                let replay = c.g[V0][0] != 0;
                c.g[S0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x1680));
                if replay {
                    break 'b_402164;
                }
                // First 0x400 bytes; the loop shape follows the alignment
                // of the source/destination pair.
                c.cycles += 7;
                c.g[V1][0] = 0x2000_0000;
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S1][0] | c.g[V1][0];
                c.g[V0][0] = c.g[A0][0] | c.g[A1][0];
                c.g[V0][0] &= 0x7;
                let aligned = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x400));
                if !aligned {
                    c.cycles += 1;
                    c.g[V1][0] = 0x450000;
                    'l_4020c8: loop {
                        c.cycles += 21;
                        c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                        c.ldr(6, c.g[A0][0] as u32);
                        c.ldl(7, (c.g[A0][0] as u32).wrapping_add(0xf));
                        c.ldr(7, (c.g[A0][0] as u32).wrapping_add(0x8));
                        c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x17));
                        c.ldr(8, (c.g[A0][0] as u32).wrapping_add(0x10));
                        c.ldl(9, (c.g[A0][0] as u32).wrapping_add(0x1f));
                        c.ldr(9, (c.g[A0][0] as u32).wrapping_add(0x18));
                        c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                        c.sdr(6, c.g[A1][0] as u32);
                        c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
                        c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                        c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
                        c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
                        c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
                        c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        let more = c.g[A0][0] != c.g[V0][0];
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x0040_20c8);
                            }
                            continue 'l_4020c8;
                        } else {
                            c.cycles += 2;
                            break 'b_402238;
                        }
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = 0x450000;
                'l_402128: loop {
                    c.cycles += 13;
                    c.g[A2][0] = c.ld(c.g[A0][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                    c.g[T1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[A2][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let more = c.g[A0][0] != c.g[V0][0];
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0040_2128);
                        }
                        continue 'l_402128;
                    } else {
                        c.cycles += 2;
                        break 'b_402238;
                    }
                }
            }
            // Nonzero reply: replay the whole 0x800 bytes plus a 16-byte
            // tail from the RPC buffer.
            'b_402218: {
                {
                    c.cycles += 7;
                    c.g[V1][0] = 0x2000_0000;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S1][0] | c.g[V1][0];
                    c.g[V0][0] = c.g[A0][0] | c.g[A1][0];
                    c.g[V0][0] &= 0x7;
                    let replay_aligned = c.g[V0][0] == 0;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
                    if !replay_aligned {
                        c.cycles += 1;
                        c.g[V1][0] = 0x450000;
                        'l_402184: loop {
                            c.cycles += 21;
                            c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                            c.ldr(6, c.g[A0][0] as u32);
                            c.ldl(7, (c.g[A0][0] as u32).wrapping_add(0xf));
                            c.ldr(7, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x17));
                            c.ldr(8, (c.g[A0][0] as u32).wrapping_add(0x10));
                            c.ldl(9, (c.g[A0][0] as u32).wrapping_add(0x1f));
                            c.ldr(9, (c.g[A0][0] as u32).wrapping_add(0x18));
                            c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.sdr(6, c.g[A1][0] as u32);
                            c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            let more = c.g[A0][0] != c.g[V0][0];
                            if more {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0040_2184);
                                }
                                continue 'l_402184;
                            } else {
                                c.cycles += 2;
                                break 'b_402218;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = 0x450000;
                'l_4021e4: loop {
                    c.cycles += 13;
                    c.g[A2][0] = c.ld(c.g[A0][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                    c.g[T1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[A2][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let more = c.g[A0][0] != c.g[V0][0];
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0040_21e4);
                        }
                        continue 'l_4021e4;
                    } else {
                        break 'b_402218;
                    }
                }
            }
            c.cycles += 8;
            c.ldl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
            c.ldr(2, c.g[A0][0] as u32);
            c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
            c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
            c.sdl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.sdr(2, c.g[A1][0] as u32);
            c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0xf));
            c.sdr(6, (c.g[A1][0] as u32).wrapping_add(0x8));
            break 'b_402238;
        }
        c.cycles += 3;
        c.g[RA][0] = sx(0x0040_2240);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1628));
        callee::SIGNAL_SEMA.call(c, 0x0040_2240)?;
        c.g[V0][0] = c.g[S0][0];
        c.cycles += 9;
        frame.restore_ra(c, 0x70);
        c.g[S5][0] = c.ld(frame.at(c, 0x60));
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceCdInit(int mode)` (0x401718): sync once, init the SIF RPC,
/// bind the CDVD RPC client and probe the drive type. `mode == 5` tears
/// the lib down (exit path); other modes bring the semaphores and the
/// power-off handler up. Returns the drive class (1, or 2 for old drives),
/// 0 when the sync or the RPC fails.
pub fn sce_cd_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xb0);
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[S3][0] = c.g[A0][0];
        c.sd(frame.at(c, 0xa0), c.g[RA][0]);
        c.sd(frame.at(c, 0x90), c.g[FP][0]);
        c.g[A0][0] = 0x1;
        c.sd(frame.at(c, 0x80), c.g[S7][0]);
        c.sd(frame.at(c, 0x70), c.g[S6][0]);
        c.sd(frame.at(c, 0x60), c.g[S5][0]);
        c.sd(frame.at(c, 0x50), c.g[S4][0]);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[RA][0] = sx(0x0040_1750);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        callee::SYNC_S.call(c, 0x0040_1750)?;
        let sync_fail = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if sync_fail {
            c.cycles += 12;
            frame.restore_ra(c, 0xa0);
            c.g[FP][0] = c.ld(frame.at(c, 0x90));
            c.g[S7][0] = c.ld(frame.at(c, 0x80));
            c.g[S6][0] = c.ld(frame.at(c, 0x70));
            c.g[S5][0] = c.ld(frame.at(c, 0x60));
            c.g[S4][0] = c.ld(frame.at(c, 0x50));
            c.g[S3][0] = c.ld(frame.at(c, 0x40));
            c.g[S2][0] = c.ld(frame.at(c, 0x30));
            c.g[S1][0] = c.ld(frame.at(c, 0x20));
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 34;
        c.g[A0][0] = 0u64;
        c.g[RA][0] = sx(0x0040_1764);
        c.g[S5][0] = 0x450000;
        callee::INIT_RPC.call(c, 0x0040_1764)?;
        c.g[RA][0] = sx(0x0040_176c);
        c.g[S4][0] = 0x450000;
        callee::GET_THREAD_ID.call(c, 0x0040_176c)?;
        c.g[S0][0] = 0x520000;
        c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x1654));
        c.g[V1][0] = 0x1;
        c.g[A0][0] = 0x450000;
        c.g[A2][0] = 0x520000;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x1624), c.g[V1][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
        c.g[A0][0] = 0xffff_ffff_ffff_ffff;
        c.g[A3][0] = 0x450000;
        c.g[T0][0] = 0x450000;
        c.g[T1][0] = 0x450000;
        c.g[T3][0] = 0x450000;
        c.g[V1][0] = 0x450000;
        c.g[T2][0] = 0x450000;
        c.sw((c.g[A2][0] as u32).wrapping_add(0x64d8), c.g[V0][0]);
        c.g[V0][0] = 0x520000;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1640), c.g[A0][0]);
        c.sw((c.g[A3][0] as u32).wrapping_add(0x1644), c.g[A0][0]);
        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x66e8));
        c.sw((c.g[T0][0] as u32).wrapping_add(0x163c), c.g[A0][0]);
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6740));
        c.sw((c.g[T1][0] as u32).wrapping_add(0x164c), c.g[A0][0]);
        c.g[FP][0] = 0x450000;
        c.sw((c.g[T3][0] as u32).wrapping_add(0x1648), c.g[A0][0]);
        c.g[S7][0] = 0x450000;
        c.sw((c.g[T2][0] as u32).wrapping_add(0x1638), 0u64);
        c.g[S6][0] = 0x490000;
        c.sw((c.g[S5][0] as u32).wrapping_add(0x1654), c.g[A1][0]);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1650), c.g[A0][0]);
        // Bind the CDVD RPC client, retrying with a delay loop.
        'l_4017e0: loop {
            'b_401844: {
                c.cycles += 8;
                c.g[A1][0] = 0xffff_ffff_8000_0000;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] |= 0x592;
                c.g[RA][0] = sx(0x0040_17f4);
                c.g[A2][0] = 0u64;
                callee::BIND_RPC.call(c, 0x0040_17f4)?;
                c.g[A1][0] = c.g[V0][0];
                if (c.g[A1][0] as i64) >= 0 {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    break 'b_401844;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x1610));
                let debug = (c.g[V0][0] as i64) <= 0;
                c.g[A2][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x1654));
                if !debug {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_1814);
                    c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x7b18));
                    callee::PRINTF.call(c, 0x0040_1814)?;
                }
                c.cycles += 3;
                c.g[V0][0] = 0x100000;
                c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                'l_401820: loop {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let more = c.g[V0][0] != c.g[V1][0];
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0040_1820);
                        }
                        continue 'l_401820;
                    } else {
                        c.cycles += 2;
                        if event_due(c) {
                            deliver_event_here(c, 0x0040_17e0);
                        }
                        continue 'l_4017e0;
                    }
                }
            }
            'b_4018cc: {
                'b_40189c: {
                    c.cycles += 2;
                    let bound_done = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S2][0];
                    if bound_done {
                        break 'b_40189c;
                    }
                    c.cycles += 17;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x6740), c.g[S3][0]);
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x1650), 0u64);
                    c.g[RA][0] = sx(0x0040_185c);
                    c.g[A1][0] = 0x4;
                    callee::WRITEBACK.call(c, 0x0040_185c)?;
                    c.g[S0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2800));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A3][0] = c.g[S2][0];
                    c.sw(c.g[SP][0] as u32, 0u64);
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    c.g[T0][0] = 0x4;
                    c.g[T1][0] = c.g[S0][0];
                    c.g[T2][0] = 0x10;
                    c.g[RA][0] = sx(0x0040_1888);
                    c.g[T3][0] = 0u64;
                    callee::CALL_RPC.call(c, 0x0040_1888)?;
                    let rpc_ok = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = 0x450000;
                    if !rpc_ok {
                        c.cycles += 3;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1624), 0u64);
                        c.g[V0][0] = 0u64;
                        c.cycles += 12;
                        frame.restore_ra(c, 0xa0);
                        c.g[FP][0] = c.ld(frame.at(c, 0x90));
                        c.g[S7][0] = c.ld(frame.at(c, 0x80));
                        c.g[S6][0] = c.ld(frame.at(c, 0x70));
                        c.g[S5][0] = c.ld(frame.at(c, 0x60));
                        c.g[S4][0] = c.ld(frame.at(c, 0x50));
                        c.g[S3][0] = c.ld(frame.at(c, 0x40));
                        c.g[S2][0] = c.ld(frame.at(c, 0x30));
                        c.g[S1][0] = c.ld(frame.at(c, 0x20));
                        c.g[S0][0] = c.ld(frame.at(c, 0x10));
                        frame.close(c);
                        return Ok(());
                    }
                    break 'b_4018cc;
                }
                c.cycles += 3;
                c.g[V0][0] = 0x100000;
                c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                'l_4018a8: loop {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let more = c.g[V0][0] != c.g[V1][0];
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0040_18a8);
                        }
                        continue 'l_4018a8;
                    } else {
                        c.cycles += 2;
                        if event_due(c) {
                            deliver_event_here(c, 0x0040_17e0);
                        }
                        continue 'l_4017e0;
                    }
                }
            }
            // Out of the probe block: past the retry loop to the decode.
            break 'l_4017e0;
        }
        // Drive class from the probe reply.
        'b_401958: {
            'b_401954: {
                c.cycles += 13;
                c.g[A1][0] = 0x2000_0000;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[V0][0] |= c.g[A1][0];
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                c.g[A0][0] |= c.g[A1][0];
                c.g[V1][0] |= c.g[A1][0];
                c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                c.g[S0][0] = 0x1;
                c.g[V0][0] = 0xff;
                let is_ff = c.g[A2][0] == c.g[V0][0];
                c.g[A0][0] = c.lw(c.g[A0][0] as u32);
                if is_ff {
                    break 'b_401954;
                }
                'b_401918: {
                    c.cycles += 3;
                    c.g[V0][0] = 0xfe;
                    let not_fe = c.g[A2][0] != c.g[V0][0];
                    c.g[A2][0] = 0xffff_ffff_ffff_ffff;
                    if not_fe {
                        break 'b_401918;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x450000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x1610), c.g[S0][0]);
                    break 'b_401954;
                }
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xff));
                c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[A1][0] as i64)) as u64;
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = c.g[A1][0];
                }
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 8) as u32);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = 0x2;
                    break 'b_401954;
                }
                c.cycles += 7;
                c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[A0][0] as i64)) as u64;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xff));
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = c.g[A0][0];
                }
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 8) as u32);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                let new_drive = c.g[V0][0] == 0;
                c.g[A0][0] = 0x450000;
                if new_drive {
                    break 'b_401958;
                }
                c.cycles += 1;
                c.g[S0][0] = 0x2;
                break 'b_401954;
            }
            c.cycles += 1;
            c.g[A0][0] = 0x450000;
            break 'b_401958;
        }
        // Mode 5 tears the library down; other modes bring it up.
        c.cycles += 3;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x1624), 0u64);
        let mode_neg = (c.g[S3][0] as i64) < 0;
        c.g[V0][0] = ((c.g[S3][0] as i64) < 2i64) as u64;
        if mode_neg {
            // Negative mode: straight to the bring-up below.
        } else {
            c.cycles += 2;
            let mode_small = c.g[V0][0] != 0;
            c.g[V0][0] = 0x5;
            if mode_small {
                // Mode 0..1: the bring-up below too.
            } else {
                c.cycles += 2;
                let is_five = c.g[S3][0] == c.g[V0][0];
                c.g[V0][0] = 0x450000;
                if is_five {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1610));
                    let debug = (c.g[V1][0] as i64) <= 0;
                    c.g[A0][0] = 0x490000;
                    if !debug {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x0040_1988);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7b38));
                        callee::PRINTF.call(c, 0x0040_1988)?;
                    }
                    c.cycles += 11;
                    c.g[RA][0] = sx(0x0040_1990);
                    callee::CD_EXIT.call(c, 0x0040_1990)?;
                    c.g[A0][0] = 0xffff_ffff_ffff_ffff;
                    c.g[V0][0] = 0x450000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x1628), c.g[A0][0]);
                    c.g[V1][0] = 0x450000;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x162c), c.g[A0][0]);
                    c.g[V0][0] = 0x450000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x1620), c.g[A0][0]);
                    c.g[V0][0] = c.g[S0][0];
                    c.cycles += 12;
                    frame.restore_ra(c, 0xa0);
                    c.g[FP][0] = c.ld(frame.at(c, 0x90));
                    c.g[S7][0] = c.ld(frame.at(c, 0x80));
                    c.g[S6][0] = c.ld(frame.at(c, 0x70));
                    c.g[S5][0] = c.ld(frame.at(c, 0x60));
                    c.g[S4][0] = c.ld(frame.at(c, 0x50));
                    c.g[S3][0] = c.ld(frame.at(c, 0x40));
                    c.g[S2][0] = c.ld(frame.at(c, 0x30));
                    c.g[S1][0] = c.ld(frame.at(c, 0x20));
                    c.g[S0][0] = c.ld(frame.at(c, 0x10));
                    frame.close(c);
                    return Ok(());
                }
            }
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x0040_19bc);
        callee::SEMA_INIT.call(c, 0x0040_19bc)?;
        c.g[RA][0] = sx(0x0040_19c4);
        callee::INSTALL_POFF.call(c, 0x0040_19c4)?;
        c.g[V0][0] = c.g[S0][0];
        c.cycles += 12;
        frame.restore_ra(c, 0xa0);
        c.g[FP][0] = c.ld(frame.at(c, 0x90));
        c.g[S7][0] = c.ld(frame.at(c, 0x80));
        c.g[S6][0] = c.ld(frame.at(c, 0x70));
        c.g[S5][0] = c.ld(frame.at(c, 0x60));
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceCdInitEeCB(int prio, void *stack, int size)` (0x400a78): start
/// the libcdvd callback thread, or reprioritise it when it already runs.
/// Returns 1 when the thread is (now) running, 0 after reprioritising it.
pub fn sce_cd_init_ee_cb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        c.sd(frame.at(c, 0x40), c.g[S4][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S4][0] = 0x450000;
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x30), c.g[S3][0]);
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.g[S3][0] = 0x1;
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S2][0] = c.g[A1][0];
        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1614));
        c.g[S0][0] = c.g[A2][0];
        let already_running = c.g[A0][0] != 0;
        c.sd(frame.at(c, 0x50), c.g[RA][0]);
        if already_running {
            c.cycles += 4;
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0040_0b20);
            c.g[S3][0] = 0u64;
            callee::CHANGE_THREAD_PRIORITY.call(c, 0x0040_0b20)?;
            c.g[V0][0] = c.g[S3][0];
            c.cycles += 8;
            frame.restore_ra(c, 0x50);
            c.g[S4][0] = c.ld(frame.at(c, 0x40));
            c.g[S3][0] = c.ld(frame.at(c, 0x30));
            c.g[S2][0] = c.ld(frame.at(c, 0x20));
            c.g[S1][0] = c.ld(frame.at(c, 0x10));
            c.g[S0][0] = c.ld(frame.at(c, 0));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 25;
        c.g[RA][0] = sx(0x0040_0ab8);
        callee::GET_THREAD_ID.call(c, 0x0040_0ab8)?;
        c.g[V1][0] = 0x520000;
        c.g[A1][0] = 0x520000;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x64d8), c.g[V0][0]);
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0040_0ad0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x64e0));
        callee::REFER_THREAD_STATUS.call(c, 0x0040_0ad0)?;
        c.g[V1][0] = 0x520000;
        c.g[V0][0] = 0x400000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6510));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x9a8));
        c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[S0][0]);
        c.g[A0][0] = c.g[V1][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[S1][0]);
        c.g[RA][0] = sx(0x0040_0afc);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
        callee::CREATE_THREAD.call(c, 0x0040_0afc)?;
        c.g[A1][0] = 0u64;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1614), c.g[V0][0]);
        c.g[RA][0] = sx(0x0040_0b0c);
        c.g[A0][0] = c.g[V0][0];
        callee::START_THREAD.call(c, 0x0040_0b0c)?;
        c.g[V0][0] = c.g[S3][0];
        c.cycles += 8;
        frame.restore_ra(c, 0x50);
        c.g[S4][0] = c.ld(frame.at(c, 0x40));
        c.g[S3][0] = c.ld(frame.at(c, 0x30));
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int sceCdLayerSearchFile(cd_file *file, const char *name, int layer)`
/// (0x400ea8): copy `name` into the search struct, issue the layer-search
/// RPC and copy the 36-byte result back to `file`. Returns the reply word,
/// 0 when a check or the RPC fails.
pub fn sce_cd_layer_search_file(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0040_0ea8 => {
                    c.cycles += 22;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S3][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S2][0]);
                    c.g[S3][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S6][0]);
                    c.g[S2][0] = c.g[A1][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[RA][0]);
                    c.g[S6][0] = 0x450000;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[FP][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S7][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S5][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S4][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S1][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
                    c.g[RA][0] = sx(0x0040_0ee8);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                    callee::SEMA_INIT.call(c, 0x0040_0ee8)?;
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x1628));
                    c.g[RA][0] = sx(0x0040_0ef4);
                    callee::POLL_SEMA.call(c, 0x0040_0ef4)?;
                    c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x1628));
                    let sema_busy = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0u64;
                    if sema_busy {
                        pc = 0x0040_118c;
                    } else {
                        pc = 0x0040_0f00;
                    }
                }
                0x0040_0f00 => {
                    c.cycles += 12;
                    c.g[V0][0] = 0x520000;
                    c.g[A2][0] = 0x450000;
                    c.g[V1][0] = 0x1;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64d8));
                    c.g[A1][0] = 0x520000;
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x161c), c.g[V1][0]);
                    c.g[RA][0] = sx(0x0040_0f20);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x64e0));
                    callee::REFER_THREAD_STATUS.call(c, 0x0040_0f20)?;
                    c.g[RA][0] = sx(0x0040_0f28);
                    c.g[A0][0] = 0x1;
                    callee::CD_SYNC.call(c, 0x0040_0f28)?;
                    let sync_fail = c.g[V0][0] != 0;
                    c.g[A0][0] = 0u64;
                    if sync_fail {
                        pc = 0x0040_10c0;
                    } else {
                        pc = 0x0040_0f30;
                    }
                }
                0x0040_0f30 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0040_0f38);
                    c.g[S1][0] = 0x450000;
                    callee::INIT_RPC.call(c, 0x0040_0f38)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1644));
                    let bound = (c.g[V0][0] as i64) >= 0;
                    c.g[S7][0] = 0x520000;
                    if bound {
                        pc = 0x0040_0ff4;
                    } else {
                        pc = 0x0040_0f44;
                    }
                }
                0x0040_0f44 => {
                    c.cycles += 4;
                    c.g[S4][0] = 0x450000;
                    c.g[S5][0] = 0x520000;
                    c.g[FP][0] = 0x520000;
                    pc = 0x0040_0f7c;
                }
                0x0040_0f54 => {
                    c.cycles += 1;
                    pc = 0x0040_0f58;
                }
                0x0040_0f58 => {
                    c.cycles += 2;
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    pc = 0x0040_0f60;
                }
                0x0040_0f60 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let more = c.g[V0][0] != c.g[V1][0];
                    if more {
                        if back_edge_unwinds(c, 0x0040_0f60) {
                            return Ok(());
                        }
                        pc = 0x0040_0f60;
                    } else {
                        pc = 0x0040_0f7c;
                    }
                }
                0x0040_0f7c => {
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x66c0));
                    pc = 0x0040_0f80;
                }
                0x0040_0f80 => {
                    c.cycles += 7;
                    c.g[A1][0] = 0xffff_ffff_8000_0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] |= 0x597;
                    c.g[RA][0] = sx(0x0040_0f94);
                    c.g[A2][0] = 0u64;
                    callee::BIND_RPC.call(c, 0x0040_0f94)?;
                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                        pc = 0x0040_0fe4;
                    } else {
                        pc = 0x0040_0f9c;
                    }
                }
                0x0040_0f9c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1610));
                    let debug = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = 0x100000;
                    if debug {
                        pc = 0x0040_0fb8;
                    } else {
                        pc = 0x0040_0fa8;
                    }
                }
                0x0040_0fa8 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x490000;
                    c.g[RA][0] = sx(0x0040_0fb4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7a08));
                    callee::PRINTF.call(c, 0x0040_0fb4)?;
                    c.g[V0][0] = 0x100000;
                    pc = 0x0040_0fb8;
                }
                0x0040_0fb8 => {
                    c.cycles += 2;
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    pc = 0x0040_0fc0;
                }
                0x0040_0fc0 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let more = c.g[V0][0] != c.g[V1][0];
                    if more {
                        if back_edge_unwinds(c, 0x0040_0fc0) {
                            return Ok(());
                        }
                        pc = 0x0040_0fc0;
                    } else {
                        pc = 0x0040_0fdc;
                    }
                }
                0x0040_0fdc => {
                    c.cycles += 2;
                    c.g[S0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x66c0));
                    if back_edge_unwinds(c, 0x0040_0f80) {
                        return Ok(());
                    }
                    pc = 0x0040_0f80;
                }
                0x0040_0fe4 => {
                    c.cycles += 2;
                    let bound_done = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x100000;
                    if bound_done {
                        if back_edge_unwinds(c, 0x0040_0f58) {
                            return Ok(());
                        }
                        pc = 0x0040_0f58;
                    } else {
                        pc = 0x0040_0fec;
                    }
                }
                0x0040_0fec => {
                    c.cycles += 2;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1644), 0u64);
                    pc = 0x0040_1000;
                }
                0x0040_0ff4 => {
                    c.cycles += 3;
                    c.g[S4][0] = 0x450000;
                    c.g[S5][0] = 0x520000;
                    c.g[FP][0] = 0x520000;
                    pc = 0x0040_1000;
                }
                0x0040_1000 => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lbu(c.g[S2][0] as u32);
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x6540));
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                    let empty = c.g[V0][0] == 0;
                    c.sb((c.g[A0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                    if empty {
                        pc = 0x0040_1044;
                    } else {
                        pc = 0x0040_1018;
                    }
                }
                0x0040_1018 => {
                    c.cycles += 2;
                    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    pc = 0x0040_1020;
                }
                0x0040_1020 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 256i64) as u64;
                    let done = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    if done {
                        pc = 0x0040_1044;
                    } else {
                        pc = 0x0040_102c;
                    }
                }
                0x0040_102c => {
                    c.cycles += 6;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                    c.sb(c.g[A0][0] as u32, c.g[V1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 24);
                    if c.g[V1][0] != 0u64 {
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        if back_edge_unwinds(c, 0x0040_1020) {
                            return Ok(());
                        }
                        pc = 0x0040_1020;
                    } else {
                        pc = 0x0040_1044;
                    }
                }
                0x0040_1044 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x100;
                    let too_long = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1610));
                    if too_long {
                        pc = 0x0040_105c;
                    } else {
                        pc = 0x0040_1050;
                    }
                }
                0x0040_1050 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x6540));
                    c.sb((c.g[V0][0] as u32).wrapping_add(0x123), 0u64);
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1610));
                    pc = 0x0040_105c;
                }
                0x0040_105c => {
                    c.cycles += 2;
                    let debug = (c.g[V0][0] as i64) <= 0;
                    c.g[A0][0] = 0x490000;
                    if debug {
                        pc = 0x0040_1074;
                    } else {
                        pc = 0x0040_1064;
                    }
                }
                0x0040_1064 => {
                    c.cycles += 4;
                    c.g[A1][0] = 0x520000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7a28));
                    c.g[RA][0] = sx(0x0040_1074);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6564));
                    callee::PRINTF.call(c, 0x0040_1074)?;
                    pc = 0x0040_1074;
                }
                0x0040_1074 => {
                    c.cycles += 19;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x6540));
                    c.g[A1][0] = 0x12c;
                    c.g[A0][0] = c.g[S0][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x128), c.g[V0][0]);
                    c.g[RA][0] = sx(0x0040_1090);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x124), c.g[S0][0]);
                    callee::WRITEBACK.call(c, 0x0040_1090)?;
                    c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x66c0));
                    c.sw(c.g[SP][0] as u32, 0u64);
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = 0x12c;
                    c.g[T1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6680));
                    c.g[T2][0] = 0x4;
                    c.g[RA][0] = sx(0x0040_10b8);
                    c.g[T3][0] = 0u64;
                    callee::CALL_RPC.call(c, 0x0040_10b8)?;
                    let rpc_ok = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = 0x2000_0000;
                    if rpc_ok {
                        pc = 0x0040_10d4;
                    } else {
                        pc = 0x0040_10c0;
                    }
                }
                0x0040_10c0 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x1628));
                    c.g[RA][0] = sx(0x0040_10cc);
                    callee::SIGNAL_SEMA.call(c, 0x0040_10cc)?;
                    c.g[V0][0] = 0u64;
                    pc = 0x0040_118c;
                }
                0x0040_10d4 => {
                    c.cycles += 24;
                    c.g[V0][0] = c.g[S0][0] | c.g[V0][0];
                    c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                    c.ldr(3, c.g[V0][0] as u32);
                    c.ldl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
                    c.ldr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x17));
                    c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x10));
                    c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0x1f));
                    c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x18));
                    c.sdl(3, (c.g[S3][0] as u32).wrapping_add(0x7));
                    c.sdr(3, c.g[S3][0] as u32);
                    c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0xf));
                    c.sdr(4, (c.g[S3][0] as u32).wrapping_add(0x8));
                    c.sdl(5, (c.g[S3][0] as u32).wrapping_add(0x17));
                    c.sdr(5, (c.g[S3][0] as u32).wrapping_add(0x10));
                    c.sdl(6, (c.g[S3][0] as u32).wrapping_add(0x1f));
                    c.sdr(6, (c.g[S3][0] as u32).wrapping_add(0x18));
                    c.lwl(3, (c.g[V0][0] as u32).wrapping_add(0x23));
                    c.lwr(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                    c.swl(3, (c.g[S3][0] as u32).wrapping_add(0x23));
                    c.swr(3, (c.g[S3][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1610));
                    let debug = (c.g[V1][0] as i64) <= 0;
                    c.g[A0][0] = 0x490000;
                    if debug {
                        pc = 0x0040_1170;
                    } else {
                        pc = 0x0040_1134;
                    }
                }
                0x0040_1134 => {
                    c.cycles += 6;
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[RA][0] = sx(0x0040_1140);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7a40));
                    callee::PRINTF.call(c, 0x0040_1140)?;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1610));
                    let debug = (c.g[V0][0] as i64) <= 0;
                    c.g[A0][0] = 0x490000;
                    if debug {
                        pc = 0x0040_1170;
                    } else {
                        pc = 0x0040_114c;
                    }
                }
                0x0040_114c => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                    c.g[RA][0] = sx(0x0040_1158);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7a50));
                    callee::PRINTF.call(c, 0x0040_1158)?;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1610));
                    let debug = (c.g[V0][0] as i64) <= 0;
                    c.g[A0][0] = 0x490000;
                    if debug {
                        pc = 0x0040_1170;
                    } else {
                        pc = 0x0040_1164;
                    }
                }
                0x0040_1164 => {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw(c.g[S3][0] as u32);
                    c.g[RA][0] = sx(0x0040_1170);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7a60));
                    callee::PRINTF.call(c, 0x0040_1170)?;
                    pc = 0x0040_1170;
                }
                0x0040_1170 => {
                    c.cycles += 7;
                    c.g[V1][0] = 0x2000_0000;
                    c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6680));
                    c.g[V0][0] |= c.g[V1][0];
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x1628));
                    c.g[RA][0] = sx(0x0040_1188);
                    c.g[S0][0] = c.lw(c.g[V0][0] as u32);
                    callee::SIGNAL_SEMA.call(c, 0x0040_1188)?;
                    c.g[V0][0] = c.g[S0][0];
                    pc = 0x0040_118c;
                }
                0x0040_118c => {
                    c.cycles += 12;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0040_0ea8, 0x0040_11bc);
                    return Ok(());
                }
            }
        }
    })
}

/// `int sceCdMmode(int mode)` (0x402450): latch `mode`, issue the mode
/// RPC and return its reply word, 0 when the precheck/RPC fails.
pub fn sce_cd_mmode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S1][0] = 0x450000;
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[S0][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2c40));
        c.g[RA][0] = sx(0x0040_2478);
        c.g[A0][0] = 0x22;
        callee::SCMD_PRE_CHECK.call(c, 0x0040_2478)?;
        if c.g[V0][0] != 0 {
            Addr(c.g[S1][0] as u32).off(0x2c40).sw(c, c.g[S0][0]);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 6;
            frame.restore_ra(c, 0x40);
            c.g[S2][0] = c.ld(frame.at(c, 0x30));
            c.g[S1][0] = c.ld(frame.at(c, 0x20));
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 21;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0040_2494);
        c.g[A1][0] = 0x4;
        callee::WRITEBACK.call(c, 0x0040_2494)?;
        c.g[A0][0] = 0x450000;
        c.g[A2][0] = 0x450000;
        c.g[V1][0] = 0x450000;
        c.g[V0][0] = 0x450000;
        c.g[T0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x31ac)).lw(c);
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2800));
        c.g[A0][0] = 0x450000;
        c.g[T2][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x31b0)).lw(c);
        c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x31a8)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3180));
        c.g[A3][0] = c.g[S2][0];
        c.sw(frame.at(c, 0), 0);
        c.g[A2][0] = 0;
        c.g[T1][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0040_24d4);
        c.g[T3][0] = 0;
        callee::CALL_RPC.call(c, 0x0040_24d4)?;
        let rpc_ok = (c.g[V0][0] as i64) >= 0;
        c.g[V1][0] = 0x450000;
        if !rpc_ok {
            c.cycles += 5;
            c.g[V0][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_24e8);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x162c)).lw(c);
            callee::SIGNAL_SEMA.call(c, 0x0040_24e8)?;
            c.g[V0][0] = 0;
            c.cycles += 6;
            frame.restore_ra(c, 0x40);
            c.g[S2][0] = c.ld(frame.at(c, 0x30));
            c.g[S1][0] = c.ld(frame.at(c, 0x20));
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 6;
        c.g[V0][0] = 0x2000_0000;
        c.g[V0][0] |= c.g[S0][0];
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x162c)).lw(c);
        c.g[RA][0] = sx(0x0040_2504);
        c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
        callee::SIGNAL_SEMA.call(c, 0x0040_2504)?;
        c.g[V0][0] = c.g[S0][0];
        c.cycles += 6;
        frame.restore_ra(c, 0x40);
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceCdPOffCallback(int a0, int a1)` (0x400d58): install the
/// power-off callback pair, returning the previously installed handler.
/// Installs the SIF handler first when the `0x451640` word says it is not.
pub fn sce_cd_poff_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x450000;
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S2][0] = c.g[A0][0];
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1640));
        c.g[S0][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x30), c.g[RA][0]);
        let installed = (c.g[V1][0] as i64) >= 0;
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        if !installed {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0040_0d88);
            callee::INSTALL_POFF.call(c, 0x0040_0d88)?;
        }
        c.cycles += 11;
        c.g[RA][0] = sx(0x0040_0d90);
        callee::DINTR.call(c, 0x0040_0d90)?;
        c.g[A1][0] = 0x520000;
        c.g[A0][0] = 0x520000;
        c.g[S1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x64c8));
        c.g[V1][0] = 0x520000;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x64d0), c.g[S0][0]);
        c.sw((c.g[A1][0] as u32).wrapping_add(0x64c8), c.g[S2][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x64cc), c.g[GP][0]);
        let first_install = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !first_install {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0040_0dbc);
            callee::EINTR.call(c, 0x0040_0dbc)?;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 6;
        frame.restore_ra(c, 0x30);
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int sceCdPowerOff(int *status)` (0x402398): issue the power-off RPC
/// (function 0x21). On success stores reply word 1 through `status` and
/// returns reply word 0 (both read uncached from `0x452800`), then signals
/// the S-command semaphore. Returns 0 when the precheck or the RPC fails.
pub fn sce_cd_power_off(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x30), c.g[RA][0]);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[RA][0] = sx(0x0040_23b4);
        c.g[A0][0] = 0x21;
        callee::SCMD_PRE_CHECK.call(c, 0x0040_23b4)?;
        let pre_ok = c.g[V0][0] != 0;
        c.g[V0][0] = 0x450000;
        if !pre_ok {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 5;
            frame.restore_ra(c, 0x30);
            c.g[S1][0] = c.ld(frame.at(c, 0x20));
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 14;
        c.g[A0][0] = 0x450000;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2800));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3180));
        c.sw(frame.at(c, 0), 0);
        c.g[A1][0] = 0x21;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = 0x8;
        c.g[RA][0] = sx(0x0040_23f4);
        c.g[T3][0] = 0;
        callee::CALL_RPC.call(c, 0x0040_23f4)?;
        let rpc_ok = (c.g[V0][0] as i64) >= 0;
        c.g[V1][0] = 0x2000_0000;
        if !rpc_ok {
            c.cycles += 5;
            c.g[V0][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_2408);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x162c)).lw(c);
            callee::SIGNAL_SEMA.call(c, 0x0040_2408)?;
            c.g[V0][0] = 0;
            c.cycles += 5;
            frame.restore_ra(c, 0x30);
            c.g[S1][0] = c.ld(frame.at(c, 0x20));
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 10;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] |= c.g[V1][0];
        c.g[A1][0] = 0x450000;
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V1][0] |= c.g[S0][0];
        Addr(c.g[S1][0] as u32).sw(c, c.g[A0][0]);
        c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x162c)).lw(c);
        c.g[RA][0] = sx(0x0040_2434);
        c.g[S0][0] = Addr(c.g[V1][0] as u32).lw(c);
        callee::SIGNAL_SEMA.call(c, 0x0040_2434)?;
        c.g[V0][0] = c.g[S0][0];
        c.cycles += 5;
        frame.restore_ra(c, 0x30);
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceCdRead(u32 lsn, u32 nsec, void *dst, cd_read_mode *mode)`
/// (0x401df8): unless streaming is active, check the drive is ready, run
/// the N-command precheck, latch the request into the `0x451700`/`0x4527c0`
/// structs (the transfer size from the mode byte), and issue the read RPC.
/// Returns 1 once issued (a debug print while the libcdvd debug word is
/// set), 0 when a check or the RPC fails.
pub fn sce_cd_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = 0x450000;
        let frame = Frame::open(c, 0x80);
        c.sd(frame.at(c, 0x60), c.g[S5][0]);
        c.g[S5][0] = 0x450000;
        c.sd(frame.at(c, 0x50), c.g[S4][0]);
        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x1638));
        c.g[S4][0] = c.g[A2][0];
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[S3][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1700));
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S1][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x70), c.g[RA][0]);
        c.g[V0][0] &= 0x1;
        let streaming = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A3][0];
        'exit: {
            if !streaming {
                c.cycles += 5;
                c.g[RA][0] = sx(0x0040_1e48);
                callee::NCMD_DISK_READY.call(c, 0x0040_1e48)?;
                c.g[V1][0] = 0x6;
                let drive_not_ready = c.g[V0][0] == c.g[V1][0];
                c.g[V0][0] = 0;
                if drive_not_ready {
                    break 'exit;
                }
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x0040_1e5c);
            c.g[A0][0] = 0x4;
            callee::NCMD_PRE_CHECK.call(c, 0x0040_1e5c)?;
            let pre_ok = c.g[V0][0] != 0;
            c.g[T0][0] = 0x450000;
            if !pre_ok {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'exit;
            }
            // Latch the request: LSN/count/mode into `0x451700`, the transfer
            // size from the mode byte (CD/XA/STR shift or mult).
            c.cycles += 18;
            c.sw(c.g[S2][0] as u32, c.g[S3][0]);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[S1][0]);
            c.g[S3][0] = 0x450000;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[S4][0]);
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2700));
            c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x27c0));
            c.g[A2][0] = 0x1;
            c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
            c.sb((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1));
            c.sb((c.g[S2][0] as u32).wrapping_add(0xd), c.g[V1][0]);
            c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x2));
            c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[A0][0]);
            c.sb((c.g[S2][0] as u32).wrapping_add(0xe), c.g[V0][0]);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x14), c.g[A1][0]);
            c.g[A3][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x2));
            let is_str = c.g[A3][0] == c.g[A2][0];
            c.g[V0][0] = ((c.g[A3][0] as i64) < 2) as u64;
            if is_str {
                c.cycles += 3;
                c.g[V0][0] = 0x918;
                c.mult(0, 5, 17, 2);
            } else {
                c.cycles += 2;
                let small = c.g[V0][0] != 0;
                c.g[A1][0] = sx((c.g[S1][0] as u32) << 11);
                if small {
                    // Plain 2048-byte sectors: the size is already in a1.
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    let is_xa = c.g[A3][0] == c.g[V0][0];
                    c.g[V0][0] = 0x924;
                    if is_xa {
                        c.cycles += 1;
                        c.mult(0, 5, 17, 2);
                    } else {
                        c.cycles += 2;
                    }
                }
            }
            // Flush the destination unless streaming, then the request structs.
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x1638));
            c.g[S0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x27c0));
            c.g[V0][0] &= 0x2;
            let no_flush = c.g[V0][0] != 0;
            c.sw((c.g[T0][0] as u32).wrapping_add(0x27c0), 0u64);
            if !no_flush {
                c.cycles += 2;
                c.g[RA][0] = sx(0x0040_1ef4);
                c.g[A0][0] = c.g[S4][0];
                callee::WRITEBACK.call(c, 0x0040_1ef4)?;
            }
            c.cycles += 14;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2700));
            c.g[A1][0] = 0x90;
            c.g[A0][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x0040_1f08);
            c.g[S4][0] = 0x450000;
            callee::WRITEBACK.call(c, 0x0040_1f08)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x0040_1f14);
            c.g[A1][0] = 0x18;
            callee::WRITEBACK.call(c, 0x0040_1f14)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0040_1f20);
            c.g[A1][0] = 0x4;
            callee::WRITEBACK.call(c, 0x0040_1f20)?;
            c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1610));
            let debug_quiet = (c.g[V0][0] as i64) <= 0;
            c.g[A0][0] = 0x490000;
            if !debug_quiet {
                c.cycles += 2;
                c.g[RA][0] = sx(0x0040_1f34);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7bb0));
                callee::PRINTF.call(c, 0x0040_1f34)?;
            }
            c.cycles += 19;
            c.g[V0][0] = 0x1;
            c.g[S0][0] = 0x450000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1658), c.g[V0][0]);
            c.g[S1][0] = 0x450000;
            c.g[A0][0] = 0x450000;
            c.g[T3][0] = 0x400000;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x1634), c.g[V0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x27d0));
            c.sw(frame.at(c, 0), c.g[S3][0]);
            c.g[A3][0] = c.g[S2][0];
            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0xb48));
            c.g[A1][0] = 0x1;
            c.g[A2][0] = 0x1;
            c.g[T0][0] = 0x18;
            c.g[T1][0] = 0;
            c.g[RA][0] = sx(0x0040_1f78);
            c.g[T2][0] = 0;
            callee::CALL_RPC.call(c, 0x0040_1f78)?;
            if (c.g[V0][0] as i64) >= 0 {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1610));
                let debug_quiet2 = (c.g[V0][0] as i64) <= 0;
                c.g[V0][0] = 0x1;
                if !debug_quiet2 {
                    c.cycles += 4;
                    c.g[A0][0] = 0x490000;
                    c.g[RA][0] = sx(0x0040_1fb0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7bc8));
                    callee::PRINTF.call(c, 0x0040_1fb0)?;
                    c.g[V0][0] = 0x1;
                }
            } else {
                c.cycles += 5;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1658), 0u64);
                c.g[V0][0] = 0x450000;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x1634), 0u64);
                c.g[RA][0] = sx(0x0040_1f94);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1628));
                callee::SIGNAL_SEMA.call(c, 0x0040_1f94)?;
                // The precheck failure's exit: 0 in v0.
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 9;
        frame.restore_ra(c, 0x70);
        c.g[S5][0] = c.ld(frame.at(c, 0x60));
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceCdReadClock(cdclock *clock)` (0x402520): read the realtime
/// clock through the CDRPC into `*clock` (8-byte unaligned copy). Debug
/// prints fire while the libcdvd debug word is set. Returns the reply
/// word, 0 when the precheck/RPC fails.
pub fn sce_cd_read_clock(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x50), c.g[RA][0]);
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[A0][0] = 0xf;
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[RA][0] = sx(0x0040_2544);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        callee::SCMD_PRE_CHECK.call(c, 0x0040_2544)?;
        let pre_ok = c.g[V0][0] != 0;
        c.g[S3][0] = 0x450000;
        if !pre_ok {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 7;
            frame.restore_ra(c, 0x50);
            c.g[S3][0] = c.ld(frame.at(c, 0x40));
            c.g[S2][0] = c.ld(frame.at(c, 0x30));
            c.g[S1][0] = c.ld(frame.at(c, 0x20));
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x1610)).lw(c);
        let debug_quiet = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = 0x490000;
        if !debug_quiet {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0040_2568);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7bd8));
            callee::PRINTF.call(c, 0x0040_2568)?;
        }
        c.cycles += 15;
        c.g[V0][0] = 0x450000;
        c.g[A0][0] = 0x450000;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2800));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3180));
        c.sw(frame.at(c, 0), 0);
        c.g[A1][0] = 0x1;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = 0x10;
        c.g[RA][0] = sx(0x0040_259c);
        c.g[T3][0] = 0;
        callee::CALL_RPC.call(c, 0x0040_259c)?;
        if (c.g[V0][0] as i64) >= 0 {
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 5;
            c.g[V0][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_25b0);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x162c)).lw(c);
            callee::SIGNAL_SEMA.call(c, 0x0040_25b0)?;
            c.g[V0][0] = 0;
            c.cycles += 7;
            frame.restore_ra(c, 0x50);
            c.g[S3][0] = c.ld(frame.at(c, 0x40));
            c.g[S2][0] = c.ld(frame.at(c, 0x30));
            c.g[S1][0] = c.ld(frame.at(c, 0x20));
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 9;
        c.g[S1][0] = 0x2000_0000;
        c.g[V0][0] |= c.g[S1][0];
        c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
        c.ldr(3, c.g[V0][0] as u32);
        c.sdl(3, (c.g[S2][0] as u32).wrapping_add(0x7));
        c.sdr(3, c.g[S2][0] as u32);
        c.g[V1][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x1610)).lw(c);
        let debug_quiet2 = (c.g[V1][0] as i64) <= 0;
        c.g[A0][0] = 0x490000;
        if !debug_quiet2 {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0040_25e4);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7bf8));
            callee::PRINTF.call(c, 0x0040_25e4)?;
        }
        c.cycles += 6;
        c.g[V0][0] = 0x450000;
        c.g[V1][0] = c.g[S0][0] | c.g[S1][0];
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x162c)).lw(c);
        c.g[RA][0] = sx(0x0040_25f8);
        c.g[S0][0] = Addr(c.g[V1][0] as u32).lw(c);
        callee::SIGNAL_SEMA.call(c, 0x0040_25f8)?;
        c.g[V0][0] = c.g[S0][0];
        c.cycles += 7;
        frame.restore_ra(c, 0x50);
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceCdSearchFile(cd_file *file, const char *name)` (0x4011c0): layer-0
/// file search: calls `sceCdLayerSearchFile(file, name, 0)` and returns its
/// result.
pub fn sce_cd_search_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.sd(frame.at(c, 0), c.g[RA][0]);
        c.g[RA][0] = sx(0x0040_11d0);
        c.g[A2][0] = 0u64;
        callee::LAYER_SEARCH_FILE.call(c, 0x0040_11d0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int sceCdSync(int mode)` (0x4013e8): wait for the N-command RPC to
/// finish. Nonzero `mode` just checks the RPC status once (1 when a request
/// is pending); mode 0 spins on a delay until it clears. Returns 1 while
/// busy, 0 when idle.
pub fn sce_cd_sync(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0040_13e8 => {
                    c.cycles += 5;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                    let check_once = c.g[A0][0] != 0;
                    c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                    if check_once {
                        pc = 0x0040_144c;
                    } else {
                        pc = 0x0040_13fc;
                    }
                }
                0x0040_13fc => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x450000;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1610));
                    let debug = (c.g[V1][0] as i64) <= 0;
                    c.g[A0][0] = 0x490000;
                    if debug {
                        pc = 0x0040_1414;
                    } else {
                        pc = 0x0040_140c;
                    }
                }
                0x0040_140c => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_1414);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7ab8));
                    callee::PRINTF.call(c, 0x0040_1414)?;
                    pc = 0x0040_1414;
                }
                0x0040_1414 => {
                    c.cycles += 3;
                    c.g[S1][0] = 0x450000;
                    c.g[S0][0] = 0x450000;
                    pc = 0x0040_1428;
                }
                0x0040_1420 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_1428);
                    c.g[A0][0] = 0x3c;
                    callee::CD_DELAY_THREAD.call(c, 0x0040_1428)?;
                    pc = 0x0040_1428;
                }
                0x0040_1428 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1634));
                    let busy_flag = c.g[V0][0] != 0;
                    if busy_flag {
                        if back_edge_unwinds(c, 0x0040_1420) {
                            return Ok(());
                        }
                        pc = 0x0040_1420;
                    } else {
                        pc = 0x0040_1434;
                    }
                }
                0x0040_1434 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0040_143c);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x27d0));
                    callee::CHECK_STAT.call(c, 0x0040_143c)?;
                    let busy = c.g[V0][0] != 0;
                    c.g[V0][0] = 0u64;
                    if busy {
                        if back_edge_unwinds(c, 0x0040_1420) {
                            return Ok(());
                        }
                        pc = 0x0040_1420;
                    } else {
                        pc = 0x0040_1444;
                    }
                }
                0x0040_1444 => {
                    c.cycles += 2;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    pc = 0x0040_1478;
                }
                0x0040_144c => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x450000;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1634));
                    let busy_flag = c.g[V1][0] != 0;
                    c.g[V0][0] = 0x1;
                    if busy_flag {
                        pc = 0x0040_1474;
                    } else {
                        pc = 0x0040_145c;
                    }
                }
                0x0040_145c => {
                    c.cycles += 5;
                    c.g[A0][0] = 0x450000;
                    c.g[RA][0] = sx(0x0040_1468);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x27d0));
                    callee::CHECK_STAT.call(c, 0x0040_1468)?;
                    let busy = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x1;
                    if busy {
                        pc = 0x0040_1474;
                    } else {
                        pc = 0x0040_1470;
                    }
                }
                0x0040_1470 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x0040_1474;
                }
                0x0040_1474 => {
                    c.cycles += 1;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    pc = 0x0040_1478;
                }
                0x0040_1478 => {
                    c.cycles += 4;
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0040_13e8, 0x0040_1488);
                    return Ok(());
                }
            }
        }
    })
}

/// `int sceCdSyncS(int mode)` (0x401488): wait for the S-command RPC to
/// finish. Nonzero `mode` just checks the RPC status once; mode 0 spins on
/// a delay until it clears. Returns the status (nonzero while busy).
pub fn sce_cd_sync_s(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0040_1488 => {
                    c.cycles += 4;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    let check_once = c.g[A0][0] != 0;
                    c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                    if check_once {
                        pc = 0x0040_14d8;
                    } else {
                        pc = 0x0040_1498;
                    }
                }
                0x0040_1498 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x450000;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1610));
                    let debug = (c.g[V1][0] as i64) <= 0;
                    c.g[A0][0] = 0x490000;
                    if debug {
                        pc = 0x0040_14b0;
                    } else {
                        pc = 0x0040_14a8;
                    }
                }
                0x0040_14a8 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_14b0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7ac8));
                    callee::PRINTF.call(c, 0x0040_14b0)?;
                    pc = 0x0040_14b0;
                }
                0x0040_14b0 => {
                    c.cycles += 2;
                    c.g[S0][0] = 0x450000;
                    pc = 0x0040_14c0;
                }
                0x0040_14b8 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_14c0);
                    c.g[A0][0] = 0x3c;
                    callee::CD_DELAY_THREAD.call(c, 0x0040_14c0)?;
                    pc = 0x0040_14c0;
                }
                0x0040_14c0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0040_14c8);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3180));
                    callee::CHECK_STAT.call(c, 0x0040_14c8)?;
                    let busy = c.g[V0][0] != 0;
                    c.g[V0][0] = 0u64;
                    if busy {
                        if back_edge_unwinds(c, 0x0040_14b8) {
                            return Ok(());
                        }
                        pc = 0x0040_14b8;
                    } else {
                        pc = 0x0040_14d0;
                    }
                }
                0x0040_14d0 => {
                    c.cycles += 2;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    pc = 0x0040_14e8;
                }
                0x0040_14d8 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x450000;
                    c.g[RA][0] = sx(0x0040_14e4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3180));
                    callee::CHECK_STAT.call(c, 0x0040_14e4)?;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    pc = 0x0040_14e8;
                }
                0x0040_14e8 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0040_1488, 0x0040_14f4);
                    return Ok(());
                }
            }
        }
    })
}

/// `int sceCdTrayReq(int cmd, int *out)` (0x402618): store `cmd` in the send
/// word (`0x452c40`), write it back from the data cache and issue the tray
/// RPC (function 5). Copies reply word 1 to `*out` when `out` is non-null and
/// returns reply word 0, signalling the S-command semaphore on both RPC
/// paths. Returns -1 when the precheck or the RPC fails.
pub fn sce_cd_tray_req(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S1][0] = 0x450000;
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[S0][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2c40));
        c.sd(frame.at(c, 0x50), c.g[RA][0]);
        c.g[S2][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x0040_2648);
        c.g[A0][0] = 0x1a;
        callee::SCMD_PRE_CHECK.call(c, 0x0040_2648)?;
        if c.g[V0][0] != 0 {
            Addr(c.g[S1][0] as u32).off(0x2c40).sw(c, c.g[S0][0]);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            c.cycles += 7;
            frame.restore_ra(c, 0x50);
            c.g[S3][0] = c.ld(frame.at(c, 0x40));
            c.g[S2][0] = c.ld(frame.at(c, 0x30));
            c.g[S1][0] = c.ld(frame.at(c, 0x20));
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 18;
        c.g[A0][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x0040_2664);
        c.g[A1][0] = 0x4;
        callee::WRITEBACK.call(c, 0x0040_2664)?;
        c.g[V0][0] = 0x450000;
        c.g[A0][0] = 0x450000;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2800));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3180));
        c.g[A3][0] = c.g[S3][0];
        c.sw(frame.at(c, 0), 0);
        c.g[A1][0] = 0x5;
        c.g[A2][0] = 0;
        c.g[T0][0] = 0x4;
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = 0x8;
        c.g[RA][0] = sx(0x0040_2698);
        c.g[T3][0] = 0;
        callee::CALL_RPC.call(c, 0x0040_2698)?;
        let rpc_ok = (c.g[V0][0] as i64) >= 0;
        c.g[V0][0] = 0x450000;
        if !rpc_ok {
            c.cycles += 5;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x162c)).lw(c);
            c.g[RA][0] = sx(0x0040_26ac);
            callee::SIGNAL_SEMA.call(c, 0x0040_26ac)?;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            c.cycles += 7;
            frame.restore_ra(c, 0x50);
            c.g[S3][0] = c.ld(frame.at(c, 0x40));
            c.g[S2][0] = c.ld(frame.at(c, 0x30));
            c.g[S1][0] = c.ld(frame.at(c, 0x20));
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        let out_null = c.g[S2][0] == 0;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
        if !out_null {
            c.cycles += 4;
            c.g[V1][0] = 0x2000_0000;
            c.g[V0][0] |= c.g[V1][0];
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            Addr(c.g[S2][0] as u32).sw(c, c.g[A0][0]);
        }
        c.cycles += 7;
        c.g[V1][0] = 0x450000;
        c.g[V0][0] = 0x2000_0000;
        c.g[V0][0] |= c.g[S0][0];
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x162c)).lw(c);
        c.g[RA][0] = sx(0x0040_26e4);
        c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
        callee::SIGNAL_SEMA.call(c, 0x0040_26e4)?;
        c.g[V0][0] = c.g[S0][0];
        c.cycles += 7;
        frame.restore_ra(c, 0x50);
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::ee;
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::machine;
    use crate::difftest::require_paths;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use ssx3_ee_rt::cpu::sx;
    use ssx3_ee_rt::Cpu;

    /// `libcdvd_AlarmHandler`: arbitrary sema ids in `a2`.
    #[test]
    fn libcdvd_alarm_handler_matches() {
        let cdvd_alarm_handler_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x0040_0810, 0);
            elf_image(c);
            c.pc = 0x0040_0810;
            c.g[A2][0] = if seed % 3 == 0 {
                0
            } else {
                r.next_u32() as u64
            };
        };
        check_lohi(
            ee::s15::f_00400810__generated,
            libcdvd_alarm_handler,
            200,
            cdvd_alarm_handler_call,
        );
        require_paths(
            ee::s15::f_00400810__generated,
            200,
            &[16, 17, 18, 19, 20, 21, 22, 23],
            cdvd_alarm_handler_call,
        );
    }

    /// `libcdvd_CallbackCmdHandler`: the `0xb` command and other commands,
    /// with and without a pending callback (`[0x451614]`) and a registered
    /// callback (`[0x5264c0]`).
    #[test]
    fn libcdvd_callback_cmd_handler_matches() {
        let cdvd_callback_cmd_handler_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x0040_0908, 0);
            elf_image(c);
            c.pc = 0x0040_0908;
            let cmd = DATA + 0x2000;
            c.bus.write32(
                cmd,
                if seed % 3 == 0 {
                    0xb
                } else {
                    r.next_u32() % 13
                },
            );
            c.g[A0][0] = sx(cmd);
            c.bus.write32(0x0045_1628, r.next_u32() & 0xffff);
            c.bus.write32(0x0045_1620, r.next_u32() & 0xffff);
            c.bus
                .write32(0x0045_1614, if seed % 4 == 1 { 0 } else { 0x0052_0000 });
            c.bus
                .write32(0x0052_64c0, if seed % 5 == 2 { 0 } else { seed });
        };
        check_lohi(
            ee::s15::f_00400908__generated,
            libcdvd_callback_cmd_handler,
            300,
            cdvd_callback_cmd_handler_call,
        );
        require_paths(
            ee::s15::f_00400908__generated,
            300,
            &[
                21, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 47, 48, 49, 50, 51, 52, 53, 54, 55,
                56, 57, 58, 59, 60, 61,
            ],
            cdvd_callback_cmd_handler_call,
        );
    }

    /// `libcdvd_DelayThread`: zero, small, masked and large delays.
    #[test]
    fn libcdvd_delay_thread_matches() {
        let cdvd_delay_thread_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x0040_0838, 0);
            elf_image(c);
            c.pc = 0x0040_0838;
            c.g[A0][0] = match seed % 5 {
                0 => 0,
                1 => 1,
                2 => 0xffff,
                3 => 0x1_0000,
                _ => r.next_u32() as u64,
            };
        };
        check_lohi(
            ee::s15::f_00400838__generated,
            libcdvd_delay_thread,
            200,
            cdvd_delay_thread_call,
        );
        require_paths(
            ee::s15::f_00400838__generated,
            200,
            &[
                56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76,
                77, 78, 79, 80,
            ],
            cdvd_delay_thread_call,
        );
    }

    /// DiskReadyOld with no guest thread, mode 0 or 8: the sema-busy (6) and
    /// sync-busy (6/-1 by mode) exits, and the ready call, which stops the
    /// run inside the RPC (the kernel switches away); debug on/off. The bind
    /// and reply legs are in the `_events` tests.
    #[test]
    fn libcdvd_disk_ready_old_matches() {
        let cdvd_disk_ready_old_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_setup(c, seed, 0x0040_19f8);
            match seed % 4 {
                0 => c.bus.write32(0x0045_162c, 0),
                1 => rpc_in_flight(c, 0x0045_3180),
                _ => {}
            }
            if seed / 4 % 2 == 0 {
                c.bus.write32(0x0045_1610, 1);
            }
            c.g[A0][0] = if seed % 3 == 1 { 8 } else { 0 };
        };
        check(
            ee::s15::f_004019f8__generated,
            libcdvd_disk_ready_old,
            200,
            cdvd_disk_ready_old_call,
        );
        require_paths(
            ee::s15::f_004019f8__generated,
            200,
            &[58, 98, 120, 160, 1328, 1390],
            cdvd_disk_ready_old_call,
        );
    }
    driven!(gen_libcdvd_disk_ready_old, ee::s15::f_004019f8__generated);
    driven!(hand_libcdvd_disk_ready_old, libcdvd_disk_ready_old);

    /// DiskReadyOld on a running thread, mode 0 or 8: sema-busy, the
    /// S-command RPC still out (sync busy), and the client not yet bound
    /// (the server answers the bind at once or after the not-ready delay,
    /// then the ready call); debug on/off.
    #[test]
    fn libcdvd_disk_ready_old_events() {
        check_events(
            gen_libcdvd_disk_ready_old,
            hand_libcdvd_disk_ready_old,
            200,
            &|c, seed| {
                let mut r = Rng(0x004019f8 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                cd_setup(c, seed, 0x0040_19f8);
                match seed % 4 {
                    0 => c.bus.write32(0x0045_162c, 0),
                    1 => rpc_in_flight(c, 0x0045_3180),
                    3 => {
                        c.bus.write32(0x0045_1648, 0xffff_ffff);
                        rpc_answered(c, 0x0052_6710, seed / 8 % 2);
                        crate::difftest::fill(c, &mut r, 0x0045_2800, 1);
                    }
                    _ => {}
                }
                if seed / 4 % 2 == 0 {
                    c.bus.write32(0x0045_1610, 1);
                }
                c.g[A0][0] = if seed / 16 % 2 == 1 { 8 } else { 0 };
                schedule(c, &mut r, seed, 0x004019f8)
            },
        );
    }

    /// Exit with no guest thread: up/down crossed with interrupts on/off,
    /// up to the first DeleteSema (the kernel switches away);
    /// `libcdvd_exit_running_matches` runs the rest.
    #[test]
    fn libcdvd_exit_matches() {
        let cdvd_exit_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_setup(c, seed, 0x0040_0cb8);
            if seed % 2 == 0 {
                c.bus.write32(0x0045_1614, 1);
            }
            if seed % 4 < 2 {
                c.cop0[12] &= !0x1_0000;
            } else {
                c.cop0[12] |= 0x1_0000;
            }
        };
        check(
            ee::s15::f_00400cb8__generated,
            libcdvd_exit,
            200,
            cdvd_exit_call,
        );
        require_paths(
            ee::s15::f_00400cb8__generated,
            200,
            &[30, 40],
            cdvd_exit_call,
        );
    }

    /// InstallPOffHandler: interrupts on/off on entry (EIntr tail or
    /// not). Always returns 1.
    #[test]
    fn libcdvd_install_poff_handler_matches() {
        let cdvd_install_poff_handler_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_setup(c, seed, 0x0040_0e30);
            if seed % 2 == 0 {
                c.cop0[12] &= !0x1_0000;
            } else {
                c.cop0[12] |= 0x1_0000;
            }
        };
        check(
            ee::s15::f_00400e30__generated,
            libcdvd_install_poff_handler,
            200,
            cdvd_install_poff_handler_call,
        );
        require_paths(
            ee::s15::f_00400e30__generated,
            200,
            &[44, 60],
            cdvd_install_poff_handler_call,
        );
    }

    /// NCmdDiskReady with no guest thread: precheck fail (0), and the RPC
    /// leg, which stops the run inside the call (the kernel switches away;
    /// `libcdvd_ncmd_disk_ready_rpc_events` runs the rest).
    #[test]
    fn libcdvd_ncmd_disk_ready_matches() {
        let cdvd_ncmd_disk_ready_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_setup(c, seed, 0x0040_1350);
            if seed % 2 == 0 {
                c.bus.write32(0x0045_1628, 0);
            }
            c.g[A0][0] = 2;
        };
        check(
            ee::s15::f_00401350__generated,
            libcdvd_ncmd_disk_ready,
            200,
            cdvd_ncmd_disk_ready_call,
        );
        require_paths(
            ee::s15::f_00401350__generated,
            200,
            &[63, 1288],
            cdvd_ncmd_disk_ready_call,
        );
    }

    /// NCmdPreCheck: sema-busy (0, debug on/off), a pending callback (sync
    /// busy: 0) and ready (1) legs. The bind legs are in the `_events` tests.
    #[test]
    fn libcdvd_ncmd_pre_check_matches() {
        let cdvd_ncmd_pre_check_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_setup(c, seed, 0x0040_11e0);
            match seed % 4 {
                0 => c.bus.write32(0x0045_1628, 0),
                1 => c.bus.write32(0x0045_1634, 1),
                _ => {}
            }
            if seed / 4 % 2 == 0 {
                c.bus.write32(0x0045_1610, 1);
            }
            c.g[A0][0] = seed as u64;
        };
        check(
            ee::s15::f_004011e0__generated,
            libcdvd_ncmd_pre_check,
            200,
            cdvd_ncmd_pre_check_call,
        );
        require_paths(
            ee::s15::f_004011e0__generated,
            200,
            &[50, 83, 114, 1009],
            cdvd_ncmd_pre_check_call,
        );
    }
    driven!(gen_libcdvd_ncmd_pre_check, ee::s15::f_004011e0__generated);
    driven!(hand_libcdvd_ncmd_pre_check, libcdvd_ncmd_pre_check);

    /// NCmdPreCheck on a running thread: sema-busy, a pending callback
    /// (sync busy), ready, and the client not yet bound (the server answers
    /// the bind at once or after the not-ready delay); debug on/off.
    #[test]
    fn libcdvd_ncmd_pre_check_events() {
        check_events(
            gen_libcdvd_ncmd_pre_check,
            hand_libcdvd_ncmd_pre_check,
            200,
            &|c, seed| {
                let mut r = Rng(0x004011e0 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                cd_setup(c, seed, 0x0040_11e0);
                match seed % 4 {
                    0 => c.bus.write32(0x0045_1628, 0),
                    1 => c.bus.write32(0x0045_1634, 1),
                    3 => {
                        c.bus.write32(0x0045_163c, 0xffff_ffff);
                        rpc_answered(c, 0x0045_27d0, seed / 8 % 2);
                    }
                    _ => {}
                }
                if seed / 4 % 2 == 0 {
                    c.bus.write32(0x0045_1610, 1);
                }
                c.g[A0][0] = seed as u64;
                schedule(c, &mut r, seed, 0x004011e0)
            },
        );
    }

    /// POffCmdHandler: disarmed (0x520000), suppressed (flag word) and
    /// callback (0, via the 0x1406c0 leaf) legs.
    #[test]
    fn libcdvd_poff_cmd_handler_matches() {
        let cdvd_poff_cmd_handler_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_setup(c, seed, 0x0040_0dd8);
            match seed % 3 {
                0 => c.bus.write32(0x0052_64c8, 0),
                1 => {
                    c.bus.write32(0x0052_64c8, 0x0014_06c0);
                    c.bus.write32(0x0052_64d0, DATA + 0x300);
                    c.bus.write32(0x0045_1624, 0);
                }
                _ => {
                    c.bus.write32(0x0052_64c8, 0x0014_06c0);
                    c.bus.write32(0x0045_1624, 1);
                }
            }
        };
        check(
            ee::s15::f_00400dd8__generated,
            libcdvd_poff_cmd_handler,
            200,
            cdvd_poff_cmd_handler_call,
        );
        require_paths(
            ee::s15::f_00400dd8__generated,
            200,
            &[10, 13, 23],
            cdvd_poff_cmd_handler_call,
        );
    }

    /// SCmdPreCheck: sema-busy (0, debug on/off), the S-command RPC still
    /// out (sync busy: 0) and ready (1) legs. The bind legs are in the
    /// `_events` tests.
    #[test]
    fn libcdvd_scmd_pre_check_matches() {
        let cdvd_scmd_pre_check_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_setup(c, seed, 0x0040_14f8);
            match seed % 4 {
                0 => c.bus.write32(0x0045_162c, 0),
                1 => rpc_in_flight(c, 0x0045_3180),
                _ => {}
            }
            if seed / 4 % 2 == 0 {
                c.bus.write32(0x0045_1610, 1);
            }
            c.g[A0][0] = seed as u64;
        };
        check(
            ee::s15::f_004014f8__generated,
            libcdvd_scmd_pre_check,
            200,
            cdvd_scmd_pre_check_call,
        );
        require_paths(
            ee::s15::f_004014f8__generated,
            200,
            &[50, 93, 114, 1000],
            cdvd_scmd_pre_check_call,
        );
    }
    driven!(gen_libcdvd_scmd_pre_check, ee::s15::f_004014f8__generated);
    driven!(hand_libcdvd_scmd_pre_check, libcdvd_scmd_pre_check);

    /// SCmdPreCheck on a running thread: sema-busy, the S-command RPC
    /// still out (sync busy), ready, and the client not yet bound (the
    /// server answers the bind at once or after the not-ready delay); debug
    /// on/off.
    #[test]
    fn libcdvd_scmd_pre_check_events() {
        check_events(
            gen_libcdvd_scmd_pre_check,
            hand_libcdvd_scmd_pre_check,
            200,
            &|c, seed| {
                let mut r = Rng(0x004014f8 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                cd_setup(c, seed, 0x0040_14f8);
                match seed % 4 {
                    0 => c.bus.write32(0x0045_162c, 0),
                    1 => rpc_in_flight(c, 0x0045_3180),
                    3 => {
                        c.bus.write32(0x0045_164c, 0xffff_ffff);
                        rpc_answered(c, 0x0045_3180, seed / 8 % 2);
                    }
                    _ => {}
                }
                if seed / 4 % 2 == 0 {
                    c.bus.write32(0x0045_1610, 1);
                }
                c.g[A0][0] = seed as u64;
                schedule(c, &mut r, seed, 0x004014f8)
            },
        );
    }

    /// SemaInit: all cached (returns the third id) and each word invalid
    /// (creates the four semaphores, returns 0x450000).
    #[test]
    fn libcdvd_sema_init_matches() {
        let cdvd_sema_init_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_setup(c, seed, 0x0040_0c00);
            match seed % 4 {
                1 => c.bus.write32(0x0045_1628, 0xffff_ffff),
                2 => c.bus.write32(0x0045_162c, 0xffff_ffff),
                3 => c.bus.write32(0x0045_1630, 0xffff_ffff),
                _ => {}
            }
        };
        check(
            ee::s15::f_00400c00__generated,
            libcdvd_sema_init,
            200,
            cdvd_sema_init_call,
        );
        require_paths(
            ee::s15::f_00400c00__generated,
            200,
            &[21, 53, 56, 59],
            cdvd_sema_init_call,
        );
    }

    /// `sceCdCallback`: busy (`[0x451634]`) vs idle sync, interrupts on or
    /// off on the DIntr path.
    #[test]
    fn sce_cd_callback_matches() {
        let cd_callback_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x0040_08a0, 0);
            elf_image(c);
            c.pc = 0x0040_08a0;
            c.cop0[12] = (r.next_u32() & !0x1_0000) | if seed % 2 == 0 { 0 } else { 0x1_0000 };
            c.g[A0][0] = sx(DATA + 0x2000 + 0x10 * (seed % 3));
            c.bus
                .write32(0x0045_1634, if seed % 3 == 0 { 1 } else { 0 });
            c.bus.write32(0x0045_1610, 0);
            c.bus.write32(0x0045_27d0, 0);
            c.bus.write32(0x0052_64c0, r.next_u32());
        };
        check_lohi(
            ee::s15::f_004008a0__generated,
            sce_cd_callback,
            300,
            cd_callback_call,
        );
        require_paths(
            ee::s15::f_004008a0__generated,
            300,
            &[28, 56, 72],
            cd_callback_call,
        );
    }

    /// DiskReady with no guest thread: sema-busy (6/-1 by mode), the ready
    /// call failing for want of a free SIF packet (6/-1 by mode), and the
    /// ready call stopping the run inside the RPC (the kernel switches
    /// away); debug on/off. The bind, reply and old-fallback legs are in the
    /// `_events` tests.
    #[test]
    fn sce_cd_disk_ready_matches() {
        let cd_disk_ready_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_setup(c, seed, 0x0040_1bf0);
            if seed % 3 == 0 {
                c.bus.write32(0x0045_1630, 0);
            }
            if seed % 3 == 1 {
                no_rpc_packets(c);
            }
            if seed % 2 == 0 {
                c.bus.write32(0x0045_1610, 1);
            }
            c.g[A0][0] = if seed % 4 == 1 { 8 } else { 0 };
        };
        check(
            ee::s15::f_00401bf0__generated,
            sce_cd_disk_ready,
            200,
            cd_disk_ready_call,
        );
        require_paths(
            ee::s15::f_00401bf0__generated,
            200,
            &[60, 122, 225, 287, 1309, 1371],
            cd_disk_ready_call,
        );
    }
    driven!(gen_sce_cd_disk_ready, ee::s15::f_00401bf0__generated);
    driven!(hand_sce_cd_disk_ready, sce_cd_disk_ready);

    /// DiskReady on a running thread, mode 0 or 8: sema-busy, the ready
    /// call failing (no free SIF packet), and the client not yet bound (the
    /// server answers the bind, then the ready call); debug on/off.
    #[test]
    fn sce_cd_disk_ready_events() {
        check_events(
            gen_sce_cd_disk_ready,
            hand_sce_cd_disk_ready,
            200,
            &|c, seed| {
                let mut r = Rng(0x00401bf0 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                cd_setup(c, seed, 0x0040_1bf0);
                if seed % 3 == 0 {
                    c.bus.write32(0x0045_1630, 0);
                }
                match seed / 3 % 3 {
                    1 => no_rpc_packets(c),
                    2 => {
                        c.bus.write32(0x0045_1648, 0xffff_ffff);
                        rpc_answered(c, 0x0052_6710, 0);
                        crate::difftest::fill(c, &mut r, 0x0045_3080, 1); // the drive status
                    }
                    _ => {}
                }
                if seed % 2 == 0 {
                    c.bus.write32(0x0045_1610, 1);
                }
                c.g[A0][0] = if seed % 4 == 1 { 8 } else { 0 };
                schedule(c, &mut r, seed, 0x00401bf0)
            },
        );
    }

    /// GetDiskType with no guest thread: precheck fail (0), and the RPC
    /// leg, which stops the run inside the call (the kernel switches away;
    /// `sce_cd_get_disk_type_rpc_events` runs the rest).
    #[test]
    fn sce_cd_get_disk_type_matches() {
        let cd_get_disk_type_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_setup(c, seed, 0x0040_2268);
            if seed % 2 == 0 {
                c.bus.write32(0x0045_162c, 0);
            }
            c.g[A0][0] = seed as u64;
        };
        check(
            ee::s15::f_00402268__generated,
            sce_cd_get_disk_type,
            200,
            cd_get_disk_type_call,
        );
        require_paths(
            ee::s15::f_00402268__generated,
            200,
            &[63, 1279],
            cd_get_disk_type_call,
        );
    }

    /// GetError with no guest thread: precheck fail (-1), and the RPC leg,
    /// which stops the run inside the call (the kernel switches away;
    /// `sce_cd_get_error_rpc_events` runs the rest).
    #[test]
    fn sce_cd_get_error_matches() {
        let cd_get_error_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_poll_setup(c, seed, 0x0040_2300);
            c.g[A0][0] = seed as u64;
        };
        check(
            ee::s15::f_00402300__generated,
            sce_cd_get_error,
            200,
            cd_get_error_call,
        );
        require_paths(
            ee::s15::f_00402300__generated,
            200,
            &[63, 1279],
            cd_get_error_call,
        );
    }

    /// GetToc with no guest thread: precheck fail (0), and the TOC RPC,
    /// which stops the run inside the call (the kernel switches away).
    /// `sce_cd_get_toc_events` runs the copy loops.
    #[test]
    fn sce_cd_get_toc_matches() {
        let cd_get_toc_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_setup(c, seed, 0x0040_1fd8);
            if seed % 2 == 0 {
                c.bus.write32(0x0045_1628, 0);
            }
            c.g[A0][0] = (DATA + 0x100 + (seed % 3) * 8) as u64;
        };
        check(
            ee::s15::f_00401fd8__generated,
            sce_cd_get_toc,
            200,
            cd_get_toc_call,
        );
        require_paths(
            ee::s15::f_00401fd8__generated,
            200,
            &[76, 1470],
            cd_get_toc_call,
        );
    }

    /// InitEeCB: reprioritise (0) and create+start (1) legs.
    #[test]
    fn sce_cd_init_ee_cb_matches() {
        let cd_init_ee_cb_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_setup(c, seed, 0x0040_0a78);
            if seed % 2 == 0 {
                c.bus.write32(0x0045_1614, 1);
            }
            c.g[A0][0] = (seed % 8) as u64;
            c.g[A1][0] = (DATA + 0x900) as u64;
            c.g[A2][0] = (0x100 + seed % 0x100) as u64;
        };
        check(
            ee::s15::f_00400a78__generated,
            sce_cd_init_ee_cb,
            200,
            cd_init_ee_cb_call,
        );
        require_paths(
            ee::s15::f_00400a78__generated,
            200,
            &[30, 55],
            cd_init_ee_cb_call,
        );
    }

    /// Init with no guest thread: the sync-busy early exit (0), and past an
    /// idle sync the first bind, which stops the run inside the RPC (the
    /// kernel switches away). The bind, probe and mode legs are in
    /// `sce_cd_init_events` and `sce_cd_init_drive_class_events`.
    #[test]
    fn sce_cd_init_matches() {
        let cd_init_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_setup(c, seed, 0x0040_1718);
            if seed / 4 % 2 == 0 {
                stale_rpc_packet(c, 0x0045_3180);
            } else {
                rpc_in_flight(c, 0x0045_3180);
            }
            if seed % 2 == 0 {
                c.bus.write32(0x0045_1610, 1);
            }
            c.g[A0][0] = [-1i64 as u64, 0, 1, 5][seed as usize % 4];
        };
        check(
            ee::s15::f_00401718__generated,
            sce_cd_init,
            200,
            cd_init_call,
        );
        require_paths(
            ee::s15::f_00401718__generated,
            200,
            &[52, 1209],
            cd_init_call,
        );
    }

    /// LayerSearchFile with no guest thread across filename shapes:
    /// sema-busy (0), a pending callback (sync busy: 0), and the search
    /// call, which stops the run inside the RPC (the kernel switches away);
    /// debug on/off. The bind and reply legs are in the `_events` tests.
    #[test]
    fn sce_cd_layer_search_file_matches() {
        let cd_layer_search_file_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_setup(c, seed, 0x0040_0ea8);
            cd_filename(c, seed);
            match seed % 5 {
                0 => c.bus.write32(0x0045_1628, 0),
                1 => c.bus.write32(0x0045_1634, 1),
                _ => {}
            }
            if seed % 2 == 0 {
                c.bus.write32(0x0045_1610, 1);
            }
            c.g[A0][0] = (DATA + 0x800) as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            c.g[A2][0] = (seed & 0xff) as u64;
        };
        check(
            ee::s15::f_00400ea8__generated,
            sce_cd_layer_search_file,
            200,
            cd_layer_search_file_call,
        );
        require_paths(
            ee::s15::f_00400ea8__generated,
            200,
            &[59, 98, 1418, 1429, 1497, 1508, 3721, 3800],
            cd_layer_search_file_call,
        );
    }
    driven!(gen_sce_cd_layer_search_file, ee::s15::f_00400ea8__generated);
    driven!(hand_sce_cd_layer_search_file, sce_cd_layer_search_file);

    /// LayerSearchFile on a running thread across filename shapes:
    /// sema-busy, a pending callback (sync busy), and the client not yet
    /// bound (the server answers the bind at once or after the not-ready
    /// delay, then the search call); debug on/off.
    #[test]
    fn sce_cd_layer_search_file_events() {
        check_events(
            gen_sce_cd_layer_search_file,
            hand_sce_cd_layer_search_file,
            200,
            &|c, seed| {
                let mut r = Rng(0x00400ea8 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                cd_setup(c, seed, 0x0040_0ea8);
                cd_filename(c, seed);
                match seed % 5 {
                    0 => c.bus.write32(0x0045_1628, 0),
                    1 => c.bus.write32(0x0045_1634, 1),
                    4 => {
                        c.bus.write32(0x0045_1644, 0xffff_ffff);
                        rpc_answered(c, 0x0052_66c0, seed / 10 % 2);
                        crate::difftest::fill(c, &mut r, 0x0052_6680, 1); // found flag
                        crate::difftest::fill(c, &mut r, DATA + 0x800, 0x28 / 4);
                    }
                    _ => {}
                }
                if seed % 2 == 0 {
                    c.bus.write32(0x0045_1610, 1);
                }
                c.g[A0][0] = (DATA + 0x800) as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = (seed & 0xff) as u64;
                schedule(c, &mut r, seed, 0x00400ea8)
            },
        );
    }

    /// Mmode with no guest thread: precheck fail (0), and the RPC leg, which
    /// stops the run inside the call (the kernel switches away;
    /// `sce_cd_mmode_rpc_events` runs the rest); the latched mode varies.
    #[test]
    fn sce_cd_mmode_matches() {
        let cd_mmode_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_poll_setup(c, seed, 0x0040_2450);
            c.g[A0][0] = (1 + seed % 3) as u64;
        };
        check(
            ee::s15::f_00402450__generated,
            sce_cd_mmode,
            200,
            cd_mmode_call,
        );
        require_paths(
            ee::s15::f_00402450__generated,
            200,
            &[70, 1289],
            cd_mmode_call,
        );
    }

    /// POffCallback: install-needed/skipped crossed with old-handler
    /// empty/set (EIntr tail or not). Returns the old handler.
    #[test]
    fn sce_cd_poff_callback_matches() {
        let cd_poff_callback_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_setup(c, seed, 0x0040_0d58);
            if seed % 3 == 0 {
                c.bus.write32(0x0045_1640, 0xffff_ffff);
            }
            if seed % 2 == 0 {
                c.bus.write32(0x0052_64c8, 0x0014_06c0);
            }
            if seed % 4 == 1 {
                c.cop0[12] &= !0x1_0000;
            } else {
                c.cop0[12] |= 0x1_0000;
            }
            c.g[A0][0] = (DATA + 0x300) as u64;
            c.g[A1][0] = (DATA + 0x310) as u64;
        };
        check(
            ee::s15::f_00400d58__generated,
            sce_cd_poff_callback,
            200,
            cd_poff_callback_call,
        );
        require_paths(
            ee::s15::f_00400d58__generated,
            200,
            &[35, 51, 81, 113],
            cd_poff_callback_call,
        );
    }

    /// PowerOff with no guest thread: precheck fail (0), and the RPC leg,
    /// which stops the run inside the call (the kernel switches away;
    /// `sce_cd_power_off_rpc_events` runs the rest).
    #[test]
    fn sce_cd_power_off_matches() {
        let cd_power_off_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_poll_setup(c, seed, 0x0040_2398);
            c.g[A0][0] = (DATA + 0x100) as u64;
        };
        check(
            ee::s15::f_00402398__generated,
            sce_cd_power_off,
            200,
            cd_power_off_call,
        );
        require_paths(
            ee::s15::f_00402398__generated,
            200,
            &[66, 1281],
            cd_power_off_call,
        );
    }

    /// ReadClock with no guest thread: precheck fail (0), and the RPC leg
    /// with the first debug print on/off, which stops the run inside the
    /// call (the kernel switches away; `sce_cd_read_clock_rpc_events` runs
    /// the rest).
    #[test]
    fn sce_cd_read_clock_matches() {
        let cd_read_clock_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_poll_setup(c, seed, 0x0040_2520);
            c.g[A0][0] = (DATA + 0x100) as u64;
        };
        check(
            ee::s15::f_00402520__generated,
            sce_cd_read_clock,
            200,
            cd_read_clock_call,
        );
        require_paths(
            ee::s15::f_00402520__generated,
            200,
            &[70, 1287, 1364],
            cd_read_clock_call,
        );
    }

    /// Gate word for [`fake_ncmd_disk_ready`] (in the machine's RAM, so
    /// only the fixtures that set it see the fake).
    const DISK_READY_GATE: u32 = DATA + 0x6000;
    const DISK_READY_MAGIC: u32 = 0x4453_4b52;
    /// The drive status the fake returns.
    const DISK_READY_STATUS: u32 = DATA + 0x6004;

    /// `libcdvd_NCmdDiskReady` with its gate set: returns the status word
    /// (the real one blocks in its synchronous RPC with no IOP).
    fn fake_ncmd_disk_ready(c: &mut Cpu) {
        if c.bus.read32(DISK_READY_GATE) != DISK_READY_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(fake_ncmd_disk_ready),
                ee::s15::f_00401350__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(DISK_READY_STATUS) as u64;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// Hooks [`fake_ncmd_disk_ready`] into the override slot (once per process).
    fn hook_ncmd_disk_ready() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00401350),
            fake_ncmd_disk_ready,
            crate::difftest::fakes::prev_cell(fake_ncmd_disk_ready),
        );
    }

    /// Read: streaming on/off, flush on/off, mode byte CD/STR/XA/other, the
    /// drive check real (it blocks) or faked to 6 (not ready), 2 or 0, an
    /// empty SIF packet pool (the RPC fails: SignalSema, 0) or not (issued,
    /// 1), and debug prints on/off.
    #[test]
    fn sce_cd_read_matches() {
        let read_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            hook_ncmd_disk_ready();
            cd_setup(c, seed, 0x0040_1df8);
            // Bit 0: streaming (no drive check); bit 1: no destination flush.
            c.bus.write32(0x0045_1638, seed % 4);
            let mode = (seed / 4 % 4) as u8;
            match seed / 16 % 4 {
                0 => {}
                k => {
                    c.bus.write32(DISK_READY_GATE, DISK_READY_MAGIC);
                    c.bus.write32(DISK_READY_STATUS, [6, 2, 0][k as usize - 1]);
                }
            }
            if seed / 64 % 2 == 1 {
                // SIF RPC already set up, with no free packets: the read RPC
                // fails (the setup would otherwise give it a fresh pool).
                c.bus.write32(0x0045_5180, 1);
                c.bus
                    .write32(crate::game::lib::sif::tests::SIF_RPC_POOL_COUNT, 0);
            }
            if seed / 128 % 2 == 1 {
                c.bus.write32(0x0045_1610, 1);
            }
            if seed % 7 == 0 {
                c.bus.write32(0x0045_1628, 0);
            }
            c.bus.write8(DATA + 0x200, mode);
            c.bus.write8(DATA + 0x201, 0x5a);
            c.bus.write8(DATA + 0x202, mode);
            c.g[A0][0] = (0x31 + seed % 16) as u64;
            c.g[A1][0] = (1 + seed % 3) as u64;
            c.g[A2][0] = (DATA + 0x404) as u64;
            c.g[A3][0] = (DATA + 0x200) as u64;
        };
        check_lohi(ee::s15::f_00401df8__generated, sce_cd_read, 256, read_call);
        require_paths(
            ee::s15::f_00401df8__generated,
            256,
            &[
                36, 83, 92, 147, 151, 156, 279, 382, 383, 386, 387, 391, 392, 395, 396, 459, 460,
                463, 464, 468, 469, 472, 473, 487, 492, 496, 501, 516, 528, 564, 567, 569, 572,
                573, 581, 588, 593, 596, 597, 598, 602, 624, 628, 629, 644, 647, 649, 652, 665,
                674, 684, 701, 702, 703, 705, 706, 712, 715, 724, 729, 738, 761, 780, 783, 792,
                806, 809, 811, 814, 945, 1025, 1311, 1450, 1451, 1454, 1455, 1459, 1460, 1463,
                1464, 1555, 1560, 1564, 1569, 1587, 1593, 1606, 1607, 1610, 1611, 1615, 1616, 1619,
                1620, 1635, 1640, 1656, 1665, 1666, 1675, 1711, 1715, 1716, 1720, 1724, 1729, 1740,
                1743, 1749, 1771, 1774, 1791, 1796, 1800, 1805, 1812, 1822, 1831, 1876, 1880, 1885,
                1927, 1930, 1936,
            ],
            read_call,
        );
    }

    /// SearchFile: layer-0 search; filename shapes as in LayerSearchFile.
    #[test]
    fn sce_cd_search_file_matches() {
        let cd_search_file_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_setup(c, seed, 0x0040_11c0);
            cd_filename(c, seed);
            c.g[A0][0] = (DATA + 0x800) as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
        };
        check(
            ee::s15::f_004011c0__generated,
            sce_cd_search_file,
            200,
            cd_search_file_call,
        );
        require_paths(
            ee::s15::f_004011c0__generated,
            200,
            &[1425, 1436, 3728],
            cd_search_file_call,
        );
    }

    /// Sync: one-shot check (1/0) and spin-then-idle (0) legs, debug
    /// on/off. The delay/retry legs need a live timer + IOP.
    #[test]
    fn sce_cd_sync_matches() {
        let cd_sync_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_setup(c, seed, 0x0040_13e8);
            match seed % 5 {
                0 => c.g[A0][0] = 0,
                1 => {
                    c.g[A0][0] = 0;
                    c.bus.write32(0x0045_1610, 1);
                }
                2 => {
                    c.g[A0][0] = 1;
                }
                3 => {
                    c.g[A0][0] = 1;
                    stale_rpc_packet(c, 0x0045_27d0);
                }
                _ => {
                    c.g[A0][0] = 1;
                    c.bus.write32(0x0045_1610, 1);
                }
            }
        };
        check(
            ee::s15::f_004013e8__generated,
            sce_cd_sync,
            200,
            cd_sync_call,
        );
        require_paths(
            ee::s15::f_004013e8__generated,
            200,
            &[25, 29, 30, 91],
            cd_sync_call,
        );
    }

    /// SyncS: one-shot check (status in/out) and spin-then-idle legs, debug
    /// on/off. The delay/retry legs need a live timer + IOP.
    #[test]
    fn sce_cd_sync_s_matches() {
        let cd_sync_s_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_setup(c, seed, 0x0040_1488);
            match seed % 5 {
                0 => c.g[A0][0] = 0,
                1 => {
                    c.g[A0][0] = 0;
                    c.bus.write32(0x0045_1610, 1);
                }
                2 => {
                    c.g[A0][0] = 1;
                }
                3 => {
                    c.g[A0][0] = 1;
                    stale_rpc_packet(c, 0x0045_3180);
                }
                _ => {
                    c.g[A0][0] = 1;
                    c.bus.write32(0x0045_1610, 1);
                }
            }
        };
        check(
            ee::s15::f_00401488__generated,
            sce_cd_sync_s,
            200,
            cd_sync_s_call,
        );
        require_paths(
            ee::s15::f_00401488__generated,
            200,
            &[16, 20, 24, 85],
            cd_sync_s_call,
        );
    }

    /// TrayReq with no guest thread: precheck fail (-1), and the RPC leg,
    /// which stops the run inside the call (the kernel switches away;
    /// `sce_cd_tray_req_rpc_events` runs the rest).
    #[test]
    fn sce_cd_tray_req_matches() {
        let cd_tray_req_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_poll_setup(c, seed, 0x0040_2618);
            c.g[A0][0] = seed as u64;
            c.g[A1][0] = if seed % 3 == 0 {
                0
            } else {
                (DATA + 0x104) as u64
            };
        };
        check(
            ee::s15::f_00402618__generated,
            sce_cd_tray_req,
            200,
            cd_tray_req_call,
        );
        require_paths(
            ee::s15::f_00402618__generated,
            200,
            &[73, 1349],
            cd_tray_req_call,
        );
    }

    /// libcdvd RPC fixture: a real kernel sema (created before the
    /// scramble) planted at the libcdvd sema words so `SemaInit` returns at
    /// once. Even seeds point `0x162c` at 0 (PollSema fails: the precheck
    /// early exit); odd seeds use the real sema (the RPC leg: with no guest
    /// thread the run stops inside the call). The libcdvd debug word and
    /// the printf formats are NULled except where a test varies them.
    pub(crate) fn cd_poll_setup(c: &mut Cpu, seed: u32, entry: u32) {
        c.bus.write32(DATA + 0x5000, 0);
        c.bus.write32(DATA + 0x5004, 1);
        c.bus.write32(DATA + 0x5008, 1);
        c.bus.write32(DATA + 0x500c, 0);
        c.bus.write32(DATA + 0x5010, 0);
        c.bus.write32(DATA + 0x5014, 0);
        c.g[A0][0] = (DATA + 0x5000) as u64;
        ee::s15::CreateSema(c);
        let id = c.g[V0][0] as u32;
        let mut r = Rng(0x0040_2300 ^ (seed as u64).wrapping_mul(0x9e37_79b9) ^ entry as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.pc = entry;
        c.bus.write32(0x0045_1628, id);
        c.bus.write32(0x0045_1630, id);
        c.bus.write32(0x0045_164c, 0);
        c.bus.write32(0x0052_d688, 0);
        c.bus.write32(0x0049_7ad8, 0);
        c.bus.write32(0x0049_7bd8, 0);
        c.bus.write32(0x0049_7bf8, 0);
        if seed % 2 == 0 {
            c.bus.write32(0x0045_162c, 0);
            c.bus.write32(0x0045_1610, 0);
        } else {
            c.bus.write32(0x0045_162c, id);
            c.bus.write32(0x0045_1610, ((seed >> 2) & 1) as u32);
        }
    }

    /// Filename shapes for the search tests: empty, short, and longer than
    /// the 0x100-byte copy window (covers the truncation store).
    pub(crate) fn cd_filename(c: &mut Cpu, seed: u32) {
        match seed % 3 {
            0 => c.bus.write8(DATA + 0x200, 0),
            1 => {
                c.bus.write8(DATA + 0x200, b'A');
                c.bus.write8(DATA + 0x201, 0);
            }
            _ => {
                for i in 0..0x100 {
                    c.bus.write8(DATA + 0x200 + i, b'B');
                }
            }
        }
    }

    /// libcdvd fixture: a real kernel sema (created before the scramble)
    /// planted at the libcdvd sema words, bind-skip flags cleared, debug
    /// off, printf formats NULled. Individual tests override per seed.
    pub(crate) fn cd_setup(c: &mut Cpu, seed: u32, entry: u32) {
        c.bus.write32(DATA + 0x5000, 0);
        c.bus.write32(DATA + 0x5004, 1);
        c.bus.write32(DATA + 0x5008, 1);
        c.bus.write32(DATA + 0x500c, 0);
        c.bus.write32(DATA + 0x5010, 0);
        c.bus.write32(DATA + 0x5014, 0);
        c.g[A0][0] = (DATA + 0x5000) as u64;
        ee::s15::CreateSema(c);
        let id = c.g[V0][0] as u32;
        let mut r = Rng(0x0040_0000 ^ (seed as u64).wrapping_mul(0x9e37_79b9) ^ entry as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.pc = entry;
        c.bus.write32(0x0045_1628, id);
        c.bus.write32(0x0045_162c, id);
        c.bus.write32(0x0045_1630, id);
        c.bus.write32(0x0045_1620, id);
        c.bus.write32(0x0045_1610, 0);
        c.bus.write32(0x0045_1614, 0);
        c.bus.write32(0x0045_1624, 0);
        c.bus.write32(0x0045_1634, 0);
        c.bus.write32(0x0045_1638, 0);
        c.bus.write32(0x0045_1640, 0);
        c.bus.write32(0x0045_163c, 0);
        c.bus.write32(0x0045_1644, 0);
        c.bus.write32(0x0045_1648, 0);
        c.bus.write32(0x0045_164c, 0);
        c.bus.write32(0x0045_1650, 0);
        c.bus.write32(0x0045_1654, 0);
        c.bus.write32(0x0045_1658, 0);
        c.bus.write32(0x0045_1680, 0);
        c.bus.write32(0x0045_1684, 0);
        c.bus.write32(0x0045_27d0, 0);
        c.bus.write32(0x0045_3180, 0);
        for &off in PRINTF_OFFS {
            c.bus.write32(0x0049_0000 + off, 0);
        }
    }

    /// A client whose packet's id differs from the client's own, with the
    /// packet's busy bit set: `sceSifCheckStatRpc(addr)` reads it as idle (0).
    pub(crate) fn stale_rpc_packet(c: &mut Cpu, addr: u32) {
        c.bus.write32(addr, DATA + 0x600);
        c.bus.write32(DATA + 0x600 + 0x18, 0x1111);
        c.bus.write32(addr + 4, 0x2222);
        c.bus.write32(DATA + 0x600 + 0x10, 1);
    }

    /// Printf formats the libcdvd debug prints use (base `0x490000`); NULled
    /// so the real `scePrintf` returns at once on the debug legs.
    pub(crate) const PRINTF_OFFS: &[u32] = &[
        0x7a08, 0x7a28, 0x7a40, 0x7a50, 0x7a60, 0x7a78, 0x7aa0, 0x7ab8, 0x7ac8, 0x7ad8, 0x7b00,
        0x7b18, 0x7b38, 0x7b48, 0x7b60, 0x7b80, 0x7b98, 0x7bb0, 0x7bc8, 0x79e0,
    ];

    /// Gate block of [`fake_delay_thread`] (in the machine's RAM, so only the
    /// fixtures that set it see the fake).
    const DELAY_GATE: u32 = DATA + 0x6120;
    const DELAY_MAGIC: u32 = 0x444c_4159;
    /// Delays left before the drive goes idle.
    const DELAY_LEFT: u32 = DATA + 0x6124;

    /// `libcdvd_DelayThread` with its gate set: a fixed 20-cycle delay instead
    /// of the alarm and semaphore (the real one blocks the thread), and once
    /// the countdown runs out the drive is idle: no pending callback and no
    /// N- or S-command RPC in flight.
    fn fake_delay_thread(c: &mut Cpu) {
        if c.bus.read32(DELAY_GATE) != DELAY_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(fake_delay_thread),
                ee::s15::f_00400838__generated,
            );
        }
        let left = c.bus.read32(DELAY_LEFT).saturating_sub(1);
        c.bus.write32(DELAY_LEFT, left);
        if left == 0 {
            c.bus.write32(0x0045_1634, 0);
            c.bus.write32(0x0045_27d0, 0);
            c.bus.write32(0x0045_3180, 0);
        }
        c.g[V0][0] = 0;
        c.cycles += 20;
        crate::abi::ret(c);
    }

    /// Gate block of [`fake_wait_sema`].
    const WAIT_GATE: u32 = DATA + 0x6100;
    const WAIT_MAGIC: u32 = 0x5250_4c59;
    /// Waits that come back before the server has answered.
    const WAIT_SILENT: u32 = DATA + 0x6104;
    /// Where the answer goes (a client's server word) and its value.
    const WAIT_REPLY_AT: u32 = DATA + 0x6108;
    const WAIT_REPLY: u32 = DATA + 0x610c;
    /// One shot: the next event this many cycles after the wait returns.
    const WAIT_ARM: u32 = DATA + 0x6110;

    /// `WaitSema` with its gate set: the RPC's semaphore is already signalled
    /// (the server answered at once), so the wait returns its id without
    /// blocking. After the silent waits it also writes the server's answer
    /// (the bound server word), and it can arm the next event. Hooked into
    /// `WaitSema`'s override slot through `fakes::hook`, gated on `WAIT_GATE`.
    fn fake_wait_sema(c: &mut Cpu) {
        if c.bus.read32(WAIT_GATE) != WAIT_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(fake_wait_sema),
                ee::s15::f_00423de0__generated,
            );
        }
        c.cycles += 4;
        let silent = c.bus.read32(WAIT_SILENT);
        if silent > 0 {
            c.bus.write32(WAIT_SILENT, silent - 1);
        } else {
            let at = c.bus.read32(WAIT_REPLY_AT);
            if at != 0 {
                let reply = c.bus.read32(WAIT_REPLY);
                c.bus.write32(at, reply);
            }
        }
        let arm = c.bus.read32(WAIT_ARM);
        if arm != 0 {
            c.next_event = c.cycles + arm as u64;
            c.bus.write32(WAIT_ARM, 0);
        }
        c.g[V0][0] = sx(c.g[A0][0] as u32);
        crate::abi::ret(c);
    }

    /// Hooks the two fakes into their override slots (once per process).
    fn hook_drive_fakes() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00400838),
            fake_delay_thread,
            crate::difftest::fakes::prev_cell(fake_delay_thread),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00423DE0),
            fake_wait_sema,
            crate::difftest::fakes::prev_cell(fake_wait_sema),
        );
    }

    /// The drive stays busy for `delays` delay calls, then goes idle.
    fn drive_idle_after(c: &mut Cpu, delays: u32) {
        hook_drive_fakes();
        c.bus.write32(DELAY_GATE, DELAY_MAGIC);
        c.bus.write32(DELAY_LEFT, delays);
    }

    /// Make `sceSifCheckStatRpc(client)` report busy: the client's packet is
    /// still allocated (flag bit 0) under the client's own request id.
    fn rpc_in_flight(c: &mut Cpu, client: u32) {
        let packet = DATA + 0x6184;
        c.bus.write32(client, packet);
        c.bus.write32(client + 4, 0x0000_3a5c);
        c.bus.write32(packet + 0x18, 0x0000_3a5c);
        c.bus.write32(packet + 0x10, 0x0003_0005);
    }

    /// The SIF RPC library already set up with no free packet: the next
    /// RPC call fails at once (`sceSifInitRpc` would refill the pool).
    fn no_rpc_packets(c: &mut Cpu) {
        c.bus.write32(0x0045_5180, 1);
        c.bus
            .write32(crate::game::lib::sif::tests::SIF_RPC_POOL_COUNT, 0);
    }

    /// The SIF RPC packet pool the answered-RPC fixtures use.
    const RPC_PACKETS: u32 = DATA + 0x7040;

    /// Binds and calls that the server answers: the RPC library already set
    /// up, four free packets, commands sent to plain IOP memory (no reply
    /// packet ever comes back), and the fake `WaitSema` returning at once and,
    /// after `silent` waits, writing `client`'s server word.
    pub(crate) fn rpc_answered(c: &mut Cpu, client: u32, silent: u32) {
        hook_drive_fakes();
        c.bus.write32(0x0045_5180, 1); // sceSifInitRpc already ran
        c.bus.write32(0x0052_d680, 0x11); // request id counter
        c.bus.write32(0x0052_d684, RPC_PACKETS);
        c.bus.write32(0x0052_d688, 4);
        // Flag word (+0x10) bit 0 clear: every packet free.
        for i in 0..0x40 {
            c.bus.write32(RPC_PACKETS + i * 4, 0x5a5a_5a5a);
        }
        c.bus.write32(0x0052_bce0, 0x0001_f000); // IOP command buffer: plain IOP RAM
        c.bus.write32(client + 0x14, 0x0001_f400); // server's receive buffer
        c.bus.write32(WAIT_GATE, WAIT_MAGIC);
        c.bus.write32(WAIT_SILENT, silent);
        c.bus.write32(WAIT_REPLY_AT, client + 0x24);
        c.bus.write32(WAIT_REPLY, 0x0001_d2c4);
        c.bus.write32(WAIT_ARM, 0);
    }

    /// Enter in the middle of a function whose `frame`-byte frame is already
    /// open: the saved `ra` returns to the harness, the other saved registers
    /// hold caller values.
    fn inside_frame(c: &mut Cpu, r: &mut Rng, frame: u32, ra_slot: u32, saved: &[u32]) -> u32 {
        let sp = crate::difftest::SP - frame;
        c.g[SP][0] = sp as u64;
        c.bus.write64(sp + ra_slot, crate::difftest::RA as u64);
        for &off in saved {
            let v = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            c.bus.write64(sp + off, v);
        }
        sp
    }

    driven!(sce_cd_sync_gen_driven, ee::s15::f_004013e8__generated);
    driven!(sce_cd_sync_hand_driven, sce_cd_sync);

    /// Sync waiting for the drive: spinning on the delay while a command
    /// callback is pending or while the N-command RPC is still out, and
    /// re-entered at the delay after an event.
    #[test]
    fn sce_cd_sync_events() {
        check_events(
            sce_cd_sync_gen_driven,
            sce_cd_sync_hand_driven,
            48,
            &|c, seed| {
                let mut r = Rng(0x004013e8 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                let on_rpc = seed / 3 % 2 == 1;
                let delays = 3 + seed / 6 % 4;
                cd_setup(c, seed, 0x0040_13e8);
                drive_idle_after(c, delays);
                if on_rpc {
                    rpc_in_flight(c, 0x0045_27d0);
                } else {
                    c.bus.write32(0x0045_1634, 1);
                }
                c.g[A0][0] = 0;
                schedule(c, &mut r, seed, 0x0040_13e8)
            },
        );
    }

    driven!(sce_cd_sync_s_gen_driven, ee::s15::f_00401488__generated);
    driven!(sce_cd_sync_s_hand_driven, sce_cd_sync_s);

    /// SyncS waiting for the S-command RPC: spinning on the delay until it
    /// completes, re-entered at the delay after an event.
    #[test]
    fn sce_cd_sync_s_events() {
        check_events(
            sce_cd_sync_s_gen_driven,
            sce_cd_sync_s_hand_driven,
            48,
            &|c, seed| {
                let mut r = Rng(0x00401488 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                let delays = 3 + seed / 3 % 4;
                cd_setup(c, seed, 0x0040_1488);
                drive_idle_after(c, delays);
                rpc_in_flight(c, 0x0045_3180);
                c.g[A0][0] = 0;
                schedule(c, &mut r, seed, 0x0040_1488)
            },
        );
    }

    driven!(sce_cd_init_gen_driven, ee::s15::f_00401718__generated);
    driven!(sce_cd_init_hand_driven, sce_cd_init);

    /// Init when the drive server answers only the second bind: an event in
    /// the not-ready delay after the first bind, or at the rebind once that
    /// delay ends; then the init call and the mode switch (exit on 5).
    #[test]
    fn sce_cd_init_events() {
        check_events(
            sce_cd_init_gen_driven,
            sce_cd_init_hand_driven,
            96,
            &|c, seed| {
                let mut r = Rng(0x00401718 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                let at_rebind = seed / 3 % 2 == 1;
                let nudge = seed / 6 % 4;
                let mode = [-1i64 as u64, 0, 2, 5][(seed / 24 % 4) as usize];
                cd_setup(c, seed, 0x0040_1718);
                rpc_answered(c, 0x0052_66e8, 1);
                // The init reply the server leaves in the receive buffer.
                crate::difftest::fill(c, &mut r, 0x0045_2800, 4);
                c.g[A0][0] = mode;
                let events = schedule(c, &mut r, seed, 0x0040_1718);
                if events != crate::difftest::events::Events::Off {
                    // Armed when the first (unanswered) bind's wait returns:
                    // inside the 0x100001-pass delay, or in the 9-cycle
                    // window between its last back edge and the rebind.
                    c.next_event = crate::difftest::CYCLE_BUDGET;
                    let arm = if at_rebind {
                        7_340_050 + 2 * nudge
                    } else {
                        8 + 4096 * nudge
                    };
                    c.bus.write32(WAIT_ARM, arm);
                }
                events
            },
        );
    }

    driven!(sce_cd_get_toc_gen_driven, ee::s15::f_00401fd8__generated);
    driven!(sce_cd_get_toc_hand_driven, sce_cd_get_toc);

    /// Where GetToc copies the TOC.
    const TOC_OUT: u32 = DATA + 0x1000;

    /// GetToc copying the TOC the drive returned: the 1 KiB (CD) and 2 KiB
    /// (DVD) layouts into aligned and unaligned buffers, with an event inside
    /// each copy loop.
    #[test]
    fn sce_cd_get_toc_events() {
        check_events(
            sce_cd_get_toc_gen_driven,
            sce_cd_get_toc_hand_driven,
            96,
            &|c, seed| {
                let mut r = Rng(0x00401fd8 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                // 0: CD unaligned (0x4020c8), 1: CD aligned (0x402128),
                // 2: DVD unaligned (0x402184), 3: DVD aligned (0x4021e4).
                let layout = seed / 3 % 4;
                let arm = 8 + 48 * (seed / 12 % 4);
                let skew = [1, 5][(seed / 48 % 2) as usize];
                cd_setup(c, seed, 0x0040_1fd8);
                rpc_answered(c, 0x0045_27d0, 0);
                crate::difftest::fill(c, &mut r, 0x0052_6780, 0x810 / 4);
                crate::difftest::fill(c, &mut r, TOC_OUT, 0x820 / 4);
                c.bus.write32(0x0045_1680, r.next_u32() | 1);
                c.bus
                    .write32(0x0045_1684, if layout >= 2 { 0x0000_0201 } else { 0 });
                let skew = if layout % 2 == 0 { skew } else { 0 };
                c.g[A0][0] = (TOC_OUT + skew) as u64;
                let events = schedule(c, &mut r, seed, 0x0040_1fd8);
                if events != crate::difftest::events::Events::Off {
                    // The precheck's syscalls would take an early event;
                    // arm it when the TOC RPC returns instead.
                    c.next_event = crate::difftest::CYCLE_BUDGET;
                    c.bus.write32(WAIT_ARM, arm);
                }
                events
            },
        );
    }

    /// NCmdPreCheck binding the N-command server, re-entered after an event
    /// in the bind-failed delay, the server-not-ready delay, at the rebind,
    /// at the wait after a bind with no server, and right after a failed
    /// bind (print with debug on, then the whole delay).
    #[test]
    fn libcdvd_ncmd_pre_check_retry_events() {
        check_events(
            gen_libcdvd_ncmd_pre_check,
            hand_libcdvd_ncmd_pre_check,
            120,
            &|c, seed| {
                let mut r = Rng(0x004012a0 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                let head = seed / 3 % 5;
                let spins = 8 + seed / 15 % 4;
                // Not-ready delay (0x4012a8), bind-failed delay (0x401308),
                // the edge to the rebind (0x4012c8), the edge to the wait
                // (0x4012a0), the bind-failed block (0x4012e4).
                let entry = [
                    0x0040_12a8,
                    0x0040_1308,
                    0x0040_1324,
                    0x0040_132c,
                    0x0040_12e4,
                ][head as usize];
                cd_setup(c, seed, entry);
                rpc_answered(c, 0x0045_27d0, 0);
                if seed / 60 % 2 == 1 {
                    c.bus.write32(0x0045_1610, 1);
                }
                inside_frame(c, &mut r, 0x40, 0x30, &[0x00, 0x10, 0x20]);
                c.g[S0][0] = 0x0045_27d0;
                c.g[S1][0] = 0x45_0000;
                c.g[S2][0] = 0x45_0000;
                c.g[V0][0] = if head < 2 { spins as u64 } else { 0 };
                c.g[V1][0] = u64::MAX;
                let events = schedule(c, &mut r, seed, entry);
                if head >= 2 && events != crate::difftest::events::Events::Off {
                    c.next_event = c.cycles;
                }
                events
            },
        );
    }

    /// SCmdPreCheck binding the S-command server, re-entered after an event
    /// in the bind-failed delay, the server-not-ready delay, at the rebind,
    /// at the wait after a bind with no server, and right after a failed
    /// bind (print with debug on, then the whole delay).
    #[test]
    fn libcdvd_scmd_pre_check_retry_events() {
        check_events(
            gen_libcdvd_scmd_pre_check,
            hand_libcdvd_scmd_pre_check,
            120,
            &|c, seed| {
                let mut r = Rng(0x004015b8 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                let head = seed / 3 % 5;
                let spins = 8 + seed / 15 % 4;
                // 0x4015c0, 0x401620, edge to 0x4015e0, edge to 0x4015b8,
                // the bind-failed block 0x4015fc.
                let entry = [
                    0x0040_15c0,
                    0x0040_1620,
                    0x0040_163c,
                    0x0040_1644,
                    0x0040_15fc,
                ][head as usize];
                cd_setup(c, seed, entry);
                rpc_answered(c, 0x0045_3180, 0);
                if seed / 60 % 2 == 1 {
                    c.bus.write32(0x0045_1610, 1);
                }
                inside_frame(c, &mut r, 0x40, 0x30, &[0x00, 0x10, 0x20]);
                c.g[S0][0] = 0x0045_3180;
                c.g[S1][0] = 0x45_0000;
                c.g[S2][0] = 0x45_0000;
                c.g[V0][0] = if head < 2 { spins as u64 } else { 0 };
                c.g[V1][0] = u64::MAX;
                let events = schedule(c, &mut r, seed, entry);
                if head >= 2 && events != crate::difftest::events::Events::Off {
                    c.next_event = c.cycles;
                }
                events
            },
        );
    }

    /// DiskReadyOld binding the old ready server, re-entered after an event
    /// in the not-ready and bind-failed delays, at the rebind, at the wait
    /// after a bind with no server and right after a failed bind (print,
    /// then the whole delay); then the ready call (mode 0 or 8, debug print
    /// on or off).
    #[test]
    fn libcdvd_disk_ready_old_retry_events() {
        check_events(
            gen_libcdvd_disk_ready_old,
            hand_libcdvd_disk_ready_old,
            240,
            &|c, seed| {
                let mut r = Rng(0x00401a90 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                let head = seed / 3 % 5;
                let spins = 8 + seed / 15 % 4;
                let mode8 = seed / 60 % 2 == 1;
                // 0x401a98, 0x401af8, edge to 0x401ab8, edge to 0x401a90,
                // the bind-failed block 0x401ad4.
                let entry = [
                    0x0040_1a98,
                    0x0040_1af8,
                    0x0040_1b14,
                    0x0040_1b1c,
                    0x0040_1ad4,
                ][head as usize];
                cd_setup(c, seed, entry);
                rpc_answered(c, 0x0052_6710, 0);
                crate::difftest::fill(c, &mut r, 0x0045_2800, 1); // the drive status
                if seed / 120 % 2 == 1 {
                    c.bus.write32(0x0045_1610, 1);
                }
                inside_frame(
                    c,
                    &mut r,
                    0xa0,
                    0x90,
                    &[0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80],
                );
                c.g[S0][0] = 0x0052_6710;
                c.g[S1][0] = 0x45_0000;
                c.g[S2][0] = if mode8 { 8 } else { 0 };
                c.g[S3][0] = 0x45_0000;
                c.g[S4][0] = 0x52_0000;
                c.g[S5][0] = 0x52_0000;
                c.g[S6][0] = 0x45_0000;
                c.g[S7][0] = 0x45_0000;
                c.g[V0][0] = if head < 2 { spins as u64 } else { 0 };
                c.g[V1][0] = u64::MAX;
                let events = schedule(c, &mut r, seed, entry);
                if head >= 2 && events != crate::difftest::events::Events::Off {
                    c.next_event = c.cycles;
                }
                events
            },
        );
    }

    /// DiskReady binding the ready server, re-entered after an event in the
    /// not-ready and bind-failed delays, at the rebind, at the retry count
    /// check after a bind with no server (17 retries used: falls back to
    /// DiskReadyOld; 3 used: the not-ready delay and a rebind), and right
    /// after a failed bind (print, then the whole delay); mode 0 or 8, debug
    /// print on or off.
    #[test]
    fn sce_cd_disk_ready_retry_events() {
        check_events(
            gen_sce_cd_disk_ready,
            hand_sce_cd_disk_ready,
            240,
            &|c, seed| {
                let mut r = Rng(0x00401c70 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                let head = seed / 3 % 5;
                let spins = 8 + seed / 15 % 4;
                let mode8 = seed / 60 % 2 == 1;
                // 0x401c80, 0x401ce0, edge to 0x401ca4, edge to 0x401c70,
                // the bind-failed block 0x401cc0.
                let entry = [
                    0x0040_1c80,
                    0x0040_1ce0,
                    0x0040_1cfc,
                    0x0040_1d04,
                    0x0040_1cc0,
                ][head as usize];
                cd_setup(c, seed, entry);
                rpc_answered(c, 0x0052_6710, 0);
                crate::difftest::fill(c, &mut r, 0x0045_3080, 1); // the drive status
                crate::difftest::fill(c, &mut r, 0x0045_2800, 1); // old-path status
                if seed / 120 % 2 == 1 {
                    c.bus.write32(0x0045_1610, 1);
                }
                inside_frame(
                    c,
                    &mut r,
                    0x90,
                    0x80,
                    &[0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70],
                );
                let retries = match spins - 8 {
                    k if head == 3 && k < 2 => 17 + k,
                    _ => 3,
                };
                c.g[S0][0] = 0x0052_6710;
                c.g[S1][0] = retries as u64;
                c.g[S2][0] = if mode8 { 8 } else { 0 };
                c.g[S3][0] = 0x45_0000;
                c.g[S4][0] = 0x45_0000;
                c.g[S5][0] = 0x52_0000;
                c.g[S6][0] = 0x45_0000;
                match head {
                    0 => {
                        c.g[V1][0] = spins as u64;
                        c.g[A0][0] = u64::MAX;
                        c.g[V0][0] = (retries + 1) as u64;
                    }
                    1 => {
                        c.g[V1][0] = spins as u64;
                        c.g[V0][0] = u64::MAX;
                    }
                    _ => c.g[V0][0] = 0,
                }
                let events = schedule(c, &mut r, seed, entry);
                if head >= 2 && events != crate::difftest::events::Events::Off {
                    c.next_event = c.cycles;
                }
                events
            },
        );
    }

    /// LayerSearchFile binding the search server, re-entered after an event
    /// in the not-ready and bind-failed delays, at the rebind, at the wait
    /// after a bind with no server, in the filename copy, and right after a
    /// failed bind (print, then the whole delay); names of 16, 40 and 128
    /// bytes and one past the 0x100-byte window, debug on or off.
    #[test]
    fn sce_cd_layer_search_file_retry_events() {
        check_events(
            gen_sce_cd_layer_search_file,
            hand_sce_cd_layer_search_file,
            144,
            &|c, seed| {
                let mut r = Rng(0x00400f58 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                let head = seed / 3 % 6;
                let shape = seed / 18 % 4;
                let spins = 8 + shape;
                // 0x400f60, 0x400fc0, edge to 0x400f80, edge to 0x400f58,
                // the filename copy 0x401020, the bind-failed block 0x400f9c.
                let entry = [
                    0x0040_0f60,
                    0x0040_0fc0,
                    0x0040_0fdc,
                    0x0040_0fe4,
                    0x0040_1020,
                    0x0040_0f9c,
                ][head as usize];
                let (name, out) = (DATA + 0x213, DATA + 0x844);
                cd_setup(c, seed, entry);
                rpc_answered(c, 0x0052_66c0, 0);
                let len = [16, 40, 0x80, 0x120][shape as usize];
                for i in 0..len {
                    c.bus.write8(name + i, b'B');
                }
                c.bus.write8(name + len, 0);
                crate::difftest::fill(c, &mut r, 0x0052_6540, 0x130 / 4); // request
                crate::difftest::fill(c, &mut r, 0x0052_6680, 1); // found flag
                crate::difftest::fill(c, &mut r, out, 0x28 / 4);
                if seed / 72 % 2 == 1 {
                    c.bus.write32(0x0045_1610, 1);
                }
                let sp = inside_frame(
                    c,
                    &mut r,
                    0xc0,
                    0xb0,
                    &[0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xa0],
                );
                c.bus.write32(sp + 0x10, r.next_u32() & 0xff); // layer
                c.g[S0][0] = 0x0052_66c0;
                c.g[S1][0] = 0x45_0000;
                c.g[S2][0] = name as u64;
                c.g[S3][0] = out as u64;
                c.g[S4][0] = 0x45_0000;
                c.g[S5][0] = 0x52_0000;
                c.g[S6][0] = 0x45_0000;
                c.g[S7][0] = 0x52_0000;
                c.g[FP][0] = 0x52_0000;
                c.g[V1][0] = u64::MAX;
                match head {
                    0 | 1 => c.g[V0][0] = spins as u64,
                    4 => {
                        // Inside the copy: byte 0 already stored.
                        c.bus.write8(0x0052_6564, b'B');
                        c.g[A1][0] = 1;
                        c.g[A2][0] = 0x0052_6564;
                    }
                    _ => c.g[V0][0] = 0,
                }
                let events = schedule(c, &mut r, seed, entry);
                if (head == 2 || head == 3) && events != crate::difftest::events::Events::Off {
                    c.next_event = c.cycles;
                }
                events
            },
        );
    }

    /// A reply-word RPC function on a running thread, its precheck passing:
    /// the call fails for want of a free SIF packet (odd `seed / 3`), or the
    /// server answers it with the `words` reply words at `reply` non-zero
    /// (the low RAM words a wrong address would read are zero). Debug word
    /// on/off; an event is due once the call's wait returns. `args` go in
    /// `a0`/`a1`.
    fn rpc_reply_legs(
        c: &mut Cpu,
        seed: u32,
        (entry, client): (u32, u32),
        (reply, words): (u32, u32),
        args: [u64; 2],
    ) -> crate::difftest::events::Events {
        let mut r = Rng(entry as u64 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
        cd_setup(c, seed, entry);
        rpc_answered(c, client, 0);
        if seed / 3 % 2 == 1 {
            c.bus
                .write32(crate::game::lib::sif::tests::SIF_RPC_POOL_COUNT, 0);
        }
        for i in 0..words {
            c.bus.write32(reply + 4 * i, r.next_u32() | 1);
        }
        crate::difftest::fill(c, &mut r, DATA + 0x100, 4); // output words
        if seed / 6 % 2 == 1 {
            c.bus.write32(0x0045_1610, 1);
        }
        [c.g[A0][0], c.g[A1][0]] = args;
        let events = schedule(c, &mut r, seed, entry);
        if events != crate::difftest::events::Events::Off {
            c.next_event = crate::difftest::CYCLE_BUDGET;
            c.bus.write32(WAIT_ARM, 8 + 16 * (seed / 12 % 4));
        }
        events
    }

    driven!(gen_sce_cd_get_disk_type, ee::s15::f_00402268__generated);
    driven!(hand_sce_cd_get_disk_type, sce_cd_get_disk_type);

    /// GetDiskType's RPC: failing (0) or answered (the reply word).
    #[test]
    fn sce_cd_get_disk_type_rpc_events() {
        check_events(
            gen_sce_cd_get_disk_type,
            hand_sce_cd_get_disk_type,
            48,
            &|c, seed| {
                let at = (0x0040_2268, 0x0045_3180);
                rpc_reply_legs(c, seed, at, (0x0045_2800, 1), [seed as u64, 0])
            },
        );
    }

    driven!(gen_sce_cd_get_error, ee::s15::f_00402300__generated);
    driven!(hand_sce_cd_get_error, sce_cd_get_error);

    /// GetError's RPC: failing (-1) or answered (the reply word).
    #[test]
    fn sce_cd_get_error_rpc_events() {
        check_events(
            gen_sce_cd_get_error,
            hand_sce_cd_get_error,
            48,
            &|c, seed| {
                let at = (0x0040_2300, 0x0045_3180);
                rpc_reply_legs(c, seed, at, (0x0045_2800, 1), [seed as u64, 0])
            },
        );
    }

    driven!(gen_sce_cd_mmode, ee::s15::f_00402450__generated);
    driven!(hand_sce_cd_mmode, sce_cd_mmode);

    /// Mmode's RPC for modes 1-3: failing (0) or answered (the reply word).
    #[test]
    fn sce_cd_mmode_rpc_events() {
        check_events(gen_sce_cd_mmode, hand_sce_cd_mmode, 144, &|c, seed| {
            let at = (0x0040_2450, 0x0045_3180);
            let mode = (1 + seed / 48 % 3) as u64;
            rpc_reply_legs(c, seed, at, (0x0045_2800, 1), [mode, 0])
        });
    }

    driven!(gen_sce_cd_power_off, ee::s15::f_00402398__generated);
    driven!(hand_sce_cd_power_off, sce_cd_power_off);

    /// PowerOff's RPC: failing (0) or answered (reply word 1 stored through
    /// `status`, reply word 0 returned).
    #[test]
    fn sce_cd_power_off_rpc_events() {
        check_events(
            gen_sce_cd_power_off,
            hand_sce_cd_power_off,
            48,
            &|c, seed| {
                let at = (0x0040_2398, 0x0045_3180);
                let status = (DATA + 0x100) as u64;
                rpc_reply_legs(c, seed, at, (0x0045_2800, 2), [status, 0])
            },
        );
    }

    driven!(gen_sce_cd_tray_req, ee::s15::f_00402618__generated);
    driven!(hand_sce_cd_tray_req, sce_cd_tray_req);

    /// TrayReq's RPC: failing (-1) or answered (reply word 1 copied to
    /// `out` when it is not null, reply word 0 returned).
    #[test]
    fn sce_cd_tray_req_rpc_events() {
        check_events(gen_sce_cd_tray_req, hand_sce_cd_tray_req, 96, &|c, seed| {
            let at = (0x0040_2618, 0x0045_3180);
            let out = [0, (DATA + 0x104) as u64][(seed / 48 % 2) as usize];
            rpc_reply_legs(c, seed, at, (0x0045_2800, 2), [seed as u64 % 3, out])
        });
    }

    driven!(gen_libcdvd_ncmd_disk_ready, ee::s15::f_00401350__generated);
    driven!(hand_libcdvd_ncmd_disk_ready, libcdvd_ncmd_disk_ready);

    /// NCmdDiskReady's RPC: failing (0) or answered (the reply word).
    #[test]
    fn libcdvd_ncmd_disk_ready_rpc_events() {
        check_events(
            gen_libcdvd_ncmd_disk_ready,
            hand_libcdvd_ncmd_disk_ready,
            48,
            &|c, seed| {
                let at = (0x0040_1350, 0x0045_27d0);
                rpc_reply_legs(c, seed, at, (0x0045_1680, 1), [2, 0])
            },
        );
    }

    driven!(gen_sce_cd_read_clock, ee::s15::f_00402520__generated);
    driven!(hand_sce_cd_read_clock, sce_cd_read_clock);

    /// ReadClock's RPC: failing (0) or answered (the 8-byte clock copied to
    /// an aligned or unaligned `clock`, the reply word returned); both
    /// debug prints on/off.
    #[test]
    fn sce_cd_read_clock_rpc_events() {
        check_events(
            gen_sce_cd_read_clock,
            hand_sce_cd_read_clock,
            96,
            &|c, seed| {
                let at = (0x0040_2520, 0x0045_3180);
                let clock = (DATA + 0x100 + 3 * (seed / 48 % 2)) as u64;
                rpc_reply_legs(c, seed, at, (0x0045_2800, 3), [clock, 0])
            },
        );
    }

    /// Exit on a running thread (each DeleteSema reschedules): up/down
    /// crossed with interrupts on/off, through RemoveCmdHandler, with the
    /// EIntr tail or not.
    #[test]
    fn libcdvd_exit_running_matches() {
        let exit_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = Rng(0x0040_0cb8 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
            cd_setup(c, seed, 0x0040_0cb8);
            if seed % 2 == 0 {
                c.bus.write32(0x0045_1614, 1);
            }
            schedule(c, &mut r, 0, 0x0040_0cb8);
            if seed / 2 % 2 == 0 {
                c.cop0[12] &= !0x1_0000;
            } else {
                c.cop0[12] |= 0x1_0000;
            }
        };
        check(ee::s15::f_00400cb8__generated, libcdvd_exit, 200, exit_call);
        require_paths(
            ee::s15::f_00400cb8__generated,
            200,
            &[61, 71, 74, 84],
            exit_call,
        );
    }

    /// Gate block of [`create_sema_fails`].
    const SEMA_FAIL_GATE: u32 = DATA + 0x6200;
    const SEMA_FAIL_MAGIC: u32 = 0x5346_4149;
    /// `CreateSema` calls left to fail.
    const SEMA_FAIL_LEFT: u32 = DATA + 0x6204;

    /// The kernel's syscalls, except that with the gate set the next
    /// `SEMA_FAIL_LEFT` `CreateSema` calls (syscall 0x40) fail (-1).
    fn create_sema_fails(c: &mut Cpu, at: u32) {
        let left = c.bus.read32(SEMA_FAIL_LEFT);
        let gated = c.bus.read32(SEMA_FAIL_GATE) == SEMA_FAIL_MAGIC;
        if gated && c.g[V1][0] as u32 == 0x40 && left > 0 {
            c.bus.write32(SEMA_FAIL_LEFT, left - 1);
            c.g[V0][0] = u64::MAX;
            return;
        }
        (ssx3_ps2::platform::HOOKS.syscall)(c, at)
    }

    /// Init on a running thread, the server answering at once: the drive
    /// class from chosen probe words (0xff and 0xfe markers, old-drive
    /// values in either word, random), modes -1, 0, 2 and 5, debug on/off,
    /// and a first bind that fails (its CreateSema fails: the print, the
    /// whole delay, then the rebind). No events.
    #[test]
    fn sce_cd_init_drive_class_events() {
        check_events(
            sce_cd_init_gen_driven,
            sce_cd_init_hand_driven,
            112,
            &|c, seed| {
                let mut r = Rng(0x00401958 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                cd_setup(c, seed, 0x0040_1718);
                rpc_answered(c, 0x0052_66e8, 0);
                crate::difftest::fill(c, &mut r, 0x0045_2800, 4);
                let words: &[(u32, u32)] = match seed % 7 {
                    1 => &[(0xc, 0xff)],
                    2 => &[(0xc, 0xfe)],
                    3 => &[(4, 0x180), (8, 0x1_0000)],
                    4 => &[(4, 0x280), (8, 0x1_0000)],
                    5 => &[(8, 0x180), (4, 0x1_0000)],
                    6 => &[(8, 0x280), (4, 0x1_0000)],
                    _ => &[],
                };
                for &(off, w) in words {
                    c.bus.write32(0x0045_2800 + off, w);
                }
                c.hooks.syscall = create_sema_fails;
                if seed / 28 % 2 == 1 {
                    c.bus.write32(SEMA_FAIL_GATE, SEMA_FAIL_MAGIC);
                    c.bus.write32(SEMA_FAIL_LEFT, 1);
                }
                if seed / 56 % 2 == 1 {
                    c.bus.write32(0x0045_1610, 1);
                }
                c.g[A0][0] = [-1i64 as u64, 0, 2, 5][(seed / 7 % 4) as usize];
                schedule(c, &mut r, 0, 0x0040_1718);
                c.next_event = crate::difftest::CYCLE_BUDGET;
                crate::difftest::events::Events::Off
            },
        );
    }
}
