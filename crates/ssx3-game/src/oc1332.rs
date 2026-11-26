//! Staging module oc1332: EAFile CD/host/shutdown ports and their tests.
//! Hand-written replacements, each with its differential test against the
//! generated body. Moved into the source-tree units by
//! `tools/reorg_modules.py apply oc1332`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::game::lib::eafile::{EA_BSS_HI, EA_DATA_HI};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003e_4040, ea_file_cd_init, "EAFile_CdInit"),
    (0x003e_3d78, ea_file_cd_mount_disc, "EAFile_CdMountDisc"),
    (0x003e_3b00, ea_file_cd_issue_read, "EAFile_CdIssueRead"),
    (0x003e_44b0, ea_file_cd_shutdown, "EAFile_CdShutdown"),
    (
        0x003e_2030,
        ea_file_host_write_file_checked,
        "EAFile_HostWriteFileChecked",
    ),
    (
        0x003e_1f48,
        ea_file_host_write_file,
        "EAFile_HostWriteFile",
    ),
    (0x003d_cde0, ea_file_shutdown, "EAFile_Shutdown"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::ee;
    use crate::call::Callee;

    pub(crate) use crate::game::lib::eafile::callee::EA_FILE_CLOSE;
    pub(crate) use crate::game::lib::eafile::callee::EA_MEM_COPY;
    pub(crate) use crate::game::lib::eafile::callee::EA_MEM_SET;
    pub(crate) use crate::game::lib::eafile::callee::EA_MEM_ZERO;
    pub(crate) use crate::game::lib::eafile::callee::EA_TIMER_ADD_CALLBACK;

    /// `EAFile_DelayTicks` (0x3e35b0).
    pub(crate) const EA_FILE_DELAY_TICKS: Callee = Callee {
        addr: 0x003e_35b0,
        f: ee::s14::EAFile_DelayTicks,
    };

    /// `EAFile_CdReset` (0x3e3618).
    pub(crate) const EA_FILE_CD_RESET: Callee = Callee {
        addr: 0x003e_3618,
        f: ee::s14::EAFile_CdReset,
    };

    /// `EAFile_CdScanDir` (0x3e3758).
    pub(crate) const EA_FILE_CD_SCAN_DIR: Callee = Callee {
        addr: 0x003e_3758,
        f: ee::s14::EAFile_CdScanDir,
    };

    /// `EAFile_CdBuildHashTable` (0x3e39a8).
    pub(crate) const EA_FILE_CD_BUILD_HASH_TABLE: Callee = Callee {
        addr: 0x003e_39a8,
        f: ee::s14::EAFile_CdBuildHashTable,
    };

    /// Intra-module call: the generated body calls the raw body, so both
    /// sides run the generated code here (until the reorg gives it a slot).
    pub(crate) const EA_FILE_CD_SHUTDOWN: Callee = Callee {
        addr: 0x003e_44b0,
        f: ee::s14::EAFile_CdShutdown,
    };

    /// `EAMutex_Destroy` (0x3e56e0).
    pub(crate) const EA_MUTEX_DESTROY: Callee = Callee {
        addr: 0x003e_56e0,
        f: ee::s14::EAMutex_Destroy,
    };

    /// `EATimer_RemoveCallback` (0x3e58c8).
    pub(crate) const EA_TIMER_REMOVE_CALLBACK: Callee = Callee {
        addr: 0x003e_58c8,
        f: ee::s14::EATimer_RemoveCallback,
    };
}

/// `int EAFile_CdInit(int file_count, const char *path)` (0x3e4040):
/// initialise the CD layer for `file_count` files named by `path`: clear the
/// state, start the drive, check the disc type (12/14 continue, anything else
/// returns 0), walk the request list fixing up names, reset and scan the disc
/// (unless no path byte is set), then build the hash table and start the read
/// thread and its timer. Returns 0 on the early exits, the file count word
/// on the full path.
pub fn ea_file_cd_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x1210);
        'b_3e4480: {
        c.g[V0][0] = EA_DATA_HI;
        frame.save(c, FP, 0x1180);
        c.g[A2][0] = 0x40;
        c.g[FP][0] = c.g[A0][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x1160), c.g[A1][0]);
        frame.save(c, S0, 0x1200);
        c.g[A0][0] = EA_BSS_HI;
        frame.save(c, S4, 0x11c0);
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc58));
        frame.save(c, S1, 0x11f0);
        c.g[A1][0] = 0u64;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x63c0));
        c.g[S4][0] = c.g[V0][0];
        frame.save(c, S2, 0x11e0);
        frame.save(c, S3, 0x11d0);
        frame.save(c, S5, 0x11b0);
        frame.save(c, S6, 0x11a0);
        frame.save(c, S7, 0x1190);
        frame.save_ra(c, 0x1170);
        c.g[RA][0] = sx(0x003e_4098);
        c.g[S1][0] = 0u64;
        callee::EA_MEM_SET.call(c, 0x003e_4098)?;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[A1][0] = 0u64;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc));
        c.mult(0, 0, 6, 30);
        c.g[A2][0] = c.lo[0];
        c.g[RA][0] = sx(0x003e_40b4);
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1160));
        callee::EA_MEM_SET.call(c, 0x003e_40b4)?;
        c.g[RA][0] = sx(0x003e_40bc);
        c.g[A0][0] = 0u64;
        call_ptr(c, 0x0040_1718, 0x003e_40bc)?;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0u64;
        if t {
            break 'b_3e4480;
        }
        'b_3e40fc: {
            c.cycles += 9;
            c.g[RA][0] = sx(0x003e_40cc);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            call_ptr(c, 0x0040_2450, 0x003e_40cc)?;
            c.g[RA][0] = sx(0x003e_40d4);
            c.g[A0][0] = 0u64;
            call_ptr(c, 0x0040_1bf0, 0x003e_40d4)?;
            c.g[RA][0] = sx(0x003e_40dc);
            call_ptr(c, 0x0040_2268, 0x003e_40dc)?;
            c.g[V1][0] = 0x12;
            if c.g[V0][0] == c.g[V1][0] {
                c.g[S1][0] = 0x1;
                // break 'b_3e40fc;
            } else {
                c.cycles += 5;
                c.g[RA][0] = sx(0x003e_40f0);
                call_ptr(c, 0x0040_2268, 0x003e_40f0)?;
                c.g[V1][0] = 0x14;
                if c.g[V0][0] == c.g[V1][0] {
                    c.g[S1][0] = 0x1;
                    // break 'b_3e40fc;
                } else {
                    // break 'b_3e40fc;
                }
            }
        }
        c.cycles += 2;
        let t = c.g[S1][0] == 0u64;
        c.g[V0][0] = 0u64;
        if t {
            break 'b_3e4480;
        }
        'b_3e4354: {
            'b_3e4350: {
                'b_3e4260: {
                    c.cycles += 33;
                    c.g[V0][0] = EA_BSS_HI;
                    c.g[A0][0] = 0u64;
                    c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x63c0));
                    c.g[RA][0] = sx(0x003e_4118);
                    c.g[S1][0] = EA_DATA_HI;
                    call_ptr(c, 0x0040_1bf0, 0x003e_4118)?;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc10));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[RA][0] = sx(0x003e_4128);
                    c.g[S2][0] = EA_DATA_HI;
                    call_ptr(c, 0x0040_1fd8, 0x003e_4128)?;
                    c.g[S7][0] = c.g[S2][0];
                    c.g[V1][0] = 0x10;
                    c.g[V0][0] = 0x1;
                    c.g[A0][0] = 0u64;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x2c), c.g[V1][0]);
                    c.sb((c.g[S3][0] as u32).wrapping_add(0x1d), c.g[V0][0]);
                    c.sb((c.g[S3][0] as u32).wrapping_add(0x1c), 0u64);
                    c.g[RA][0] = sx(0x003e_414c);
                    c.sb((c.g[S3][0] as u32).wrapping_add(0x1e), 0u64);
                    call_ptr(c, 0x0042_4020, 0x003e_414c)?;
                    c.g[S0][0] = 0x108;
                    c.g[A0][0] = 0x490000;
                    c.mult(0, 16, 30, 16);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5de0));
                    c.g[A2][0] = 0u64;
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x003e_416c);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, t, 0x003e_416c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xc68), c.g[V0][0]);
                    c.g[RA][0] = sx(0x003e_417c);
                    c.g[A1][0] = c.g[S0][0];
                    callee::EA_MEM_ZERO.call(c, 0x003e_417c)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = c.g[V0][0];
                    if t {
                        break 'b_3e4260;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V0][0] = c.lb(c.g[V1][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xc68));
                    if t {
                        break 'b_3e4350;
                    }
                    c.cycles += 4;
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1050));
                    c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1150));
                    c.g[S6][0] = EA_DATA_HI;
                    'l_3e41a8: loop {
                        'b_3e41f4: {
                            c.cycles += 9;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = c.g[S2][0];
                            c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x108));
                            c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                            c.g[RA][0] = sx(0x003e_41c0);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7));
                            call_ptr(c, 0x0041_66f4, 0x003e_41c0)?;
                            c.g[V1][0] = c.lb((c.g[SP][0] as u32).wrapping_add(0x1050));
                            let t = c.g[V1][0] == 0u64;
                            c.g[A0][0] = c.g[S2][0];
                            if t {
                                break 'b_3e41f4;
                            }
                            c.cycles += 3;
                            c.g[A1][0] = 0x5c;
                            c.g[V1][0] = 0x2f;
                            'l_3e41d8: loop {
                                'b_3e41e4: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                                    if c.g[V0][0] == c.g[A1][0] {
                                        c.sb(c.g[A0][0] as u32, c.g[V1][0]);
                                        break 'b_3e41e4;
                                    } else {
                                        break 'b_3e41e4;
                                    }
                                }
                                c.cycles += 4;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                                let t = c.g[V0][0] != 0u64;
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003e_41d8);
                                    }
                                    continue 'l_3e41d8;
                                } else {
                                    break 'b_3e41f4;
                                }
                            }
                        }
                        c.cycles += 25;
                        c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                        c.g[RA][0] = sx(0x003e_4200);
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x0041_66f4, 0x003e_4200)?;
                        c.g[RA][0] = sx(0x003e_4208);
                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                        call_ptr(c, 0x0041_6810, 0x003e_4208)?;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S3][0];
                        c.g[RA][0] = sx(0x003e_421c);
                        c.sb((c.g[V0][0] as u32).wrapping_sub(0x2), 0u64);
                        call_ptr(c, 0x0041_62d0, 0x003e_421c)?;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x003e_4228);
                        c.g[A1][0] = c.g[S2][0];
                        call_ptr(c, 0x0041_66f4, 0x003e_4228)?;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc6c));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x100), c.g[V1][0]);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[S0][0] = c.g[S4][0];
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x104), c.g[V1][0]);
                        c.sw((c.g[S6][0] as u32).wrapping_add(0xc6c), c.g[V0][0]);
                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                        c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                        let t = c.g[V1][0] != 0u64;
                        c.g[S1][0] = c.g[S5][0];
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x003e_41a8);
                            }
                            continue 'l_3e41a8;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = EA_DATA_HI;
                        break 'b_3e4354;
                    }
                }
                'b_3e4334: {
                    c.cycles += 13;
                    c.g[RA][0] = sx(0x003e_4268);
                    callee::EA_FILE_CD_RESET.call(c, 0x003e_4268)?;
                    c.g[V1][0] = EA_BSS_HI;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1380));
                    c.g[A0][0] = 0x2f;
                    c.sb((c.g[V0][0] as u32).wrapping_add(0x1), 0u64);
                    c.g[S0][0] = c.g[V1][0];
                    c.sb((c.g[V1][0] as u32).wrapping_sub(0x1380), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc58));
                    c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                    if c.g[V1][0] == 0u64 {
                        break 'b_3e4334;
                    }
                    c.cycles += 10;
                    c.lwl(2, (c.g[A2][0] as u32).wrapping_add(0xa1));
                    c.lwr(2, (c.g[A2][0] as u32).wrapping_add(0x9e));
                    c.g[RA][0] = sx(0x003e_42a4);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                    callee::EA_FILE_CD_RESET.call(c, 0x003e_42a4)?;
                    c.g[V1][0] = EA_BSS_HI;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5b80));
                    c.g[V0][0] = (c.g[V1][0] < c.g[S1][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = EA_DATA_HI;
                    if t {
                        break 'b_3e4354;
                    }
                    c.cycles += 3;
                    c.g[S2][0] = c.g[V1][0];
                    c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc58));
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x21));
                    'l_3e42c8: loop {
                        'b_3e42f4: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x003e_42d0);
                            c.g[A2][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x20));
                            call_ptr(c, 0x0041_6b18, 0x003e_42d0)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc58));
                            if t {
                                break 'b_3e42f4;
                            }
                            c.cycles += 5;
                            c.g[V0][0] = c.lb(c.g[S1][0] as u32);
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V1][0] = (c.g[S2][0] < c.g[S1][0]) as u64;
                            let t = c.g[V1][0] == 0u64;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x21));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x003e_42c8);
                                }
                                continue 'l_3e42c8;
                            }
                            c.cycles += 2;
                            c.g[A0][0] = EA_DATA_HI;
                            break 'b_3e4354;
                        }
                        c.cycles += 16;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1380));
                        c.g[RA][0] = sx(0x003e_4300);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0041_62d0, 0x003e_4300)?;
                        c.g[A1][0] = 0x490000;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x003e_4310);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5df0));
                        call_ptr(c, 0x0041_62d0, 0x003e_4310)?;
                        c.lwl(5, (c.g[S1][0] as u32).wrapping_add(0xd));
                        c.lwr(5, (c.g[S1][0] as u32).wrapping_add(0xa));
                        c.lwl(4, (c.g[S1][0] as u32).wrapping_add(0x5));
                        c.lwr(4, (c.g[S1][0] as u32).wrapping_add(0x2));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7ff));
                        c.g[RA][0] = sx(0x003e_432c);
                        c.g[A1][0] = sx((c.g[A1][0] as u32) >> 11);
                        callee::EA_FILE_CD_SCAN_DIR.call(c, 0x003e_432c)?;
                        c.g[A0][0] = EA_DATA_HI;
                        break 'b_3e4354;
                    }
                }
            c.cycles += 7;
            c.lwl(5, (c.g[A2][0] as u32).wrapping_add(0xa9));
            c.lwr(5, (c.g[A2][0] as u32).wrapping_add(0xa6));
            c.lwl(4, (c.g[A2][0] as u32).wrapping_add(0xa1));
            c.lwr(4, (c.g[A2][0] as u32).wrapping_add(0x9e));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7ff));
            c.g[RA][0] = sx(0x003e_4350);
            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 11);
            callee::EA_FILE_CD_SCAN_DIR.call(c, 0x003e_4350)?;
            break 'b_3e4350;
        }
        c.cycles += 1;
        c.g[A0][0] = EA_DATA_HI;
        break 'b_3e4354;
        }
        'b_3e4460: {
            c.cycles += 4;
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc6c));
            c.g[V0][0] = (c.g[FP][0] < c.g[V1][0]) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[A0][0] = EA_DATA_HI;
                break 'b_3e4460;
            } else {
                c.cycles += 63;
                c.g[A0][0] = EA_BSS_HI;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1160));
                c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x63c0));
                c.g[V0][0] = EA_DATA_HI;
                c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc70));
                c.g[A2][0] = c.g[V1][0];
                c.g[A0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xc68));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x38), c.g[A1][0]);
                c.g[RA][0] = sx(0x003e_438c);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                callee::EA_FILE_CD_BUILD_HASH_TABLE.call(c, 0x003e_438c)?;
                c.g[A0][0] = EA_DATA_HI;
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc10));
                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xc68));
                let t = c.g[A1][0] as u32;
                c.g[RA][0] = sx(0x003e_43a4);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                call_ptr(c, t, 0x003e_43a4)?;
                c.g[V0][0] = EA_DATA_HI;
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xc6c), 0u64);
                c.g[V0][0] = 0x20;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x38), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                c.g[RA][0] = sx(0x003e_43c4);
                c.sw((c.g[S7][0] as u32).wrapping_add(0xc68), 0u64);
                call_ptr(c, 0x0042_3da0, 0x003e_43c4)?;
                c.g[V1][0] = 0x3e0000;
                c.g[A1][0] = EA_BSS_HI;
                c.g[A2][0] = 0x4a0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3be0));
                c.g[A3][0] = 0x4000;
                c.g[T0][0] = 0xc;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x30f0));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5380));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                c.g[A0][0] = c.g[SP][0];
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[A3][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[T0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.g[RA][0] = sx(0x003e_4408);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                call_ptr(c, 0x0042_3ba0, 0x003e_4408)?;
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x003e_4418);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                call_ptr(c, 0x0042_3bc0, 0x003e_4418)?;
                c.g[A1][0] = EA_BSS_HI;
                c.g[A2][0] = 0x800;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5b80));
                c.g[RA][0] = sx(0x003e_442c);
                c.g[A0][0] = 0xb;
                call_ptr(c, 0x0040_0a78, 0x003e_442c)?;
                c.g[A0][0] = 0x3e0000;
                c.g[RA][0] = sx(0x003e_4438);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3ad8));
                call_ptr(c, 0x0040_08a0, 0x003e_4438)?;
                c.g[V1][0] = EA_BSS_HI;
                c.g[A0][0] = 0x3e0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6380));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4000));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), c.g[V1][0]);
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x003e_4458);
                c.g[A2][0] = 0u64;
                callee::EA_TIMER_ADD_CALLBACK.call(c, 0x003e_4458)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                break 'b_3e4480;
            }
        }
        c.cycles += 8;
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x003e_4470);
        c.g[A0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xc68));
        call_ptr(c, t, 0x003e_4470)?;
        c.g[V1][0] = EA_DATA_HI;
        c.sw((c.g[S7][0] as u32).wrapping_add(0xc68), 0u64);
        c.sw((c.g[V1][0] as u32).wrapping_add(0xc6c), 0u64);
        c.g[V0][0] = 0u64;
        break 'b_3e4480;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x1200);
        frame.restore(c, S1, 0x11f0);
        frame.restore(c, S2, 0x11e0);
        frame.restore(c, S3, 0x11d0);
        frame.restore(c, S4, 0x11c0);
        frame.restore(c, S5, 0x11b0);
        frame.restore(c, S6, 0x11a0);
        frame.restore(c, S7, 0x1190);
        frame.restore(c, FP, 0x1180);
        frame.restore_ra(c, 0x1170);
        frame.close(c);
        Ok(())
    })
}

/// `int EAFile_CdMountDisc(void)` (0x3e3d78): mount the disc in the drive:
/// copy the CD state, spin on `sceCdInit`/`sceCdDiskReady` until the drive
/// answers, read sectors and advance the LBA until `sceCdGetError` is quiet,
/// then register the callbacks and signal the semaphore. A `match pc` body
/// (back edges deliver events): re-entered at its block heads after an
/// unwind. Returns 0.
pub fn ea_file_cd_mount_disc(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003e_3d78 => {
                    c.cycles += 24;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
                    c.g[A0][0] = 0u64;
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S7][0] = 0u64;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.g[RA][0] = sx(0x003e_3db4);
                    c.g[FP][0] = 0u64;
                    call_ptr(c, 0x0040_08a0, 0x003e_3db4)?;
                    c.g[A1][0] = EA_BSS_HI;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x63a0));
                    c.g[RA][0] = sx(0x003e_3dc8);
                    c.g[A2][0] = 0x18;
                    callee::EA_MEM_COPY.call(c, 0x003e_3dc8)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    pc = 0x003e_3dd8;
                    continue;
                }
                0x003e_3dd8 => {
                    c.cycles += 11;
                    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1e0));
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1e0));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1e0));
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1e0));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                    pc = 0x003e_3e30;
                    continue;
                }
                0x003e_3e04 => {
                    c.cycles += 1;
                    pc = 0x003e_3e08;
                    continue;
                }
                0x003e_3e08 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e_3e10);
                    c.g[A0][0] = 0x1c20;
                    callee::EA_FILE_DELAY_TICKS.call(c, 0x003e_3e10)?;
                    pc = 0x003e_3e20;
                    continue;
                }
                0x003e_3e18 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x003e_3e20);
                    c.g[A0][0] = 0x1c20;
                    callee::EA_FILE_DELAY_TICKS.call(c, 0x003e_3e20)?;
                    pc = 0x003e_3e20;
                    continue;
                }
                0x003e_3e20 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e_3e28);
                    c.g[A0][0] = 0x5;
                    call_ptr(c, 0x0040_1718, 0x003e_3e28)?;
                    let t = c.g[V0][0] == 0u64;
                    if t {
                        if back_edge_unwinds(c, 0x003e_3e18) {
                            return Ok(());
                        }
                        pc = 0x003e_3e18;
                        continue;
                    }
                    pc = 0x003e_3e30;
                    continue;
                }
                0x003e_3e30 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e_3e38);
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x0040_1718, 0x003e_3e38)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = EA_DATA_HI;
                    if t {
                        if back_edge_unwinds(c, 0x003e_3e08) {
                            return Ok(());
                        }
                        pc = 0x003e_3e08;
                        continue;
                    }
                    pc = 0x003e_3e40;
                    continue;
                }
                0x003e_3e40 => {
                    c.cycles += 9;
                    c.g[S1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc58));
                    c.g[RA][0] = sx(0x003e_3e50);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    call_ptr(c, 0x0040_2450, 0x003e_3e50)?;
                    c.g[RA][0] = sx(0x003e_3e58);
                    c.g[A0][0] = c.g[FP][0] & 0xffff;
                    callee::EA_FILE_DELAY_TICKS.call(c, 0x003e_3e58)?;
                    c.g[V0][0] = EA_BSS_HI;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x63c0));
                    pc = 0x003e_3e70;
                    continue;
                }
                0x003e_3e64 => {
                    c.cycles += 1;
                    pc = 0x003e_3e68;
                    continue;
                }
                0x003e_3e68 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x003e_3e70);
                    c.g[A0][0] = c.g[S1][0] & 0xffff;
                    callee::EA_FILE_DELAY_TICKS.call(c, 0x003e_3e70)?;
                    pc = 0x003e_3e70;
                    continue;
                }
                0x003e_3e70 => {
                    c.cycles += 7;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                    c.g[A0][0] = 0x10;
                    c.g[A1][0] = 0x1;
                    c.g[RA][0] = sx(0x003e_3e84);
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
                    call_ptr(c, 0x0040_1df8, 0x003e_3e84)?;
                    let t = c.g[V0][0] == 0u64;
                    if t {
                        if back_edge_unwinds(c, 0x003e_3e68) {
                            return Ok(());
                        }
                        pc = 0x003e_3e68;
                        continue;
                    }
                    pc = 0x003e_3e8c;
                    continue;
                }
                0x003e_3e8c => {
                    c.cycles += 1;
                    c.g[S1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                    pc = 0x003e_3e90;
                    continue;
                }
                0x003e_3e90 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x003e_3e98);
                    c.g[A0][0] = 0x1;
                    call_ptr(c, 0x0040_13e8, 0x003e_3e98)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e_3ea4);
                    c.g[A0][0] = c.g[S1][0] & 0xffff;
                    callee::EA_FILE_DELAY_TICKS.call(c, 0x003e_3ea4)?;
                    let t = c.g[S0][0] != 0u64;
                    if t {
                        if back_edge_unwinds(c, 0x003e_3e90) {
                            return Ok(());
                        }
                        pc = 0x003e_3e90;
                        continue;
                    }
                    pc = 0x003e_3eac;
                    continue;
                }
                0x003e_3eac => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e_3eb4);
                    call_ptr(c, 0x0040_2300, 0x003e_3eb4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        pc = 0x003e_3f8c;
                        continue;
                    }
                    pc = 0x003e_3ebc;
                    continue;
                }
                0x003e_3ebc => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) <= 0;
                    if t {
                        pc = 0x003e_3f8c;
                        continue;
                    }
                    pc = 0x003e_3ec4;
                    continue;
                }
                0x003e_3ec4 => {
                    c.cycles += 5;
                    c.g[S3][0] = EA_BSS_HI;
                    c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S5][0] = EA_BSS_HI;
                    c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x63c0));
                    c.g[S6][0] = 0x1e0;
                    pc = 0x003e_3ed8;
                    continue;
                }
                0x003e_3ed8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e_3ee0);
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x0040_1bf0, 0x003e_3ee0)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_sub(0x63c0));
                    pc = 0x003e_3ef4;
                    continue;
                }
                0x003e_3ee8 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003e_3ef0);
                    c.g[A0][0] = c.g[S2][0] & 0xffff;
                    callee::EA_FILE_DELAY_TICKS.call(c, 0x003e_3ef0)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_sub(0x63c0));
                    pc = 0x003e_3ef4;
                    continue;
                }
                0x003e_3ef4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                    if t {
                        pc = 0x003e_3f04;
                        continue;
                    }
                    pc = 0x003e_3f00;
                    continue;
                }
                0x003e_3f00 => {
                    c.cycles += 1;
                    c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x3c));
                    pc = 0x003e_3f04;
                    continue;
                }
                0x003e_3f04 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                    c.g[A1][0] = 0x1;
                    c.g[RA][0] = sx(0x003e_3f14);
                    c.g[A3][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x63a4));
                    call_ptr(c, 0x0040_1df8, 0x003e_3f14)?;
                    let t = c.g[V0][0] == 0u64;
                    c.mult(0, 17, 23, 22);
                    if t {
                        if back_edge_unwinds(c, 0x003e_3ee8) {
                            return Ok(());
                        }
                        pc = 0x003e_3ee8;
                        continue;
                    }
                    pc = 0x003e_3f1c;
                    continue;
                }
                0x003e_3f1c => {
                    c.cycles += 1;
                    pc = 0x003e_3f20;
                    continue;
                }
                0x003e_3f20 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x003e_3f28);
                    c.g[A0][0] = 0x1;
                    call_ptr(c, 0x0040_13e8, 0x003e_3f28)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e_3f34);
                    c.g[A0][0] = c.g[S1][0] & 0xffff;
                    callee::EA_FILE_DELAY_TICKS.call(c, 0x003e_3f34)?;
                    let t = c.g[S0][0] != 0u64;
                    if t {
                        if back_edge_unwinds(c, 0x003e_3f20) {
                            return Ok(());
                        }
                        pc = 0x003e_3f20;
                        continue;
                    }
                    pc = 0x003e_3f3c;
                    continue;
                }
                0x003e_3f3c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e_3f44);
                    call_ptr(c, 0x0040_2300, 0x003e_3f44)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                    if t {
                        pc = 0x003e_3f5c;
                        continue;
                    }
                    pc = 0x003e_3f4c;
                    continue;
                }
                0x003e_3f4c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e_3f54);
                    c.g[A0][0] = 0x1c20;
                    callee::EA_FILE_DELAY_TICKS.call(c, 0x003e_3f54)?;
                    pc = 0x003e_3f8c;
                    continue;
                }
                0x003e_3f5c => {
                    c.cycles += 12;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x800));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                    c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                    let t = (c.g[A1][0] as i64) > 0;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                    if t {
                        if back_edge_unwinds(c, 0x003e_3ed8) {
                            return Ok(());
                        }
                        pc = 0x003e_3ed8;
                        continue;
                    }
                    pc = 0x003e_3f8c;
                    continue;
                }
                0x003e_3f8c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e_3f94);
                    call_ptr(c, 0x0040_2300, 0x003e_3f94)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x003e_3dd8) {
                            return Ok(());
                        }
                        pc = 0x003e_3dd8;
                        continue;
                    }
                    pc = 0x003e_3f9c;
                    continue;
                }
                0x003e_3f9c => {
                    c.cycles += 25;
                    c.g[A1][0] = EA_BSS_HI;
                    c.g[A2][0] = 0x800;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5b80));
                    c.g[RA][0] = sx(0x003e_3fb0);
                    c.g[A0][0] = 0xb;
                    call_ptr(c, 0x0040_0a78, 0x003e_3fb0)?;
                    c.g[A0][0] = 0x3e0000;
                    c.g[RA][0] = sx(0x003e_3fbc);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3ad8));
                    call_ptr(c, 0x0040_08a0, 0x003e_3fbc)?;
                    c.g[V1][0] = EA_BSS_HI;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x63c0));
                    c.g[RA][0] = sx(0x003e_3fcc);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0042_3dc0, 0x003e_3fcc)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.g[V0][0] = 0u64;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003e_3d78, 0x003e_4000);
                    return Ok(());
                }
            }
        }
    })
}

/// `int EAFile_CdIssueRead(void)` (0x3e3b00): issue one CD read from the
/// current request state: delay, sync, then up to three `sceCdRead` tries
/// (delaying between them); on success set the done bits, otherwise set the
/// error bits. A `match pc` body (one back edge delivers events).
/// Returns 0.
pub fn ea_file_cd_issue_read(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003e_3b00 => {
                    c.cycles += 14;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                    c.g[V0][0] = EA_BSS_HI;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x63a0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x20));
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x20));
                    if t {
                        pc = 0x003e_3b40;
                        continue;
                    }
                    pc = 0x003e_3b38;
                    continue;
                }
                0x003e_3b38 => {
                    c.cycles += 2;
                    c.g[S3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x3c));
                    pc = 0x003e_3b44;
                    continue;
                }
                0x003e_3b40 => {
                    c.cycles += 1;
                    c.g[S3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    pc = 0x003e_3b44;
                    continue;
                }
                0x003e_3b44 => {
                    c.cycles += 4;
                    c.g[S4][0] = 0u64;
                    c.g[S2][0] = 0u64;
                    c.g[S5][0] = EA_BSS_HI;
                    pc = 0x003e_3b6c;
                    continue;
                }
                0x003e_3b54 => {
                    c.cycles += 1;
                    pc = 0x003e_3b58;
                    continue;
                }
                0x003e_3b58 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[V0][0] & 0xffff;
                    c.g[RA][0] = sx(0x003e_3b64);
                    c.g[S2][0] = c.g[V0][0];
                    callee::EA_FILE_DELAY_TICKS.call(c, 0x003e_3b64)?;
                    c.g[RA][0] = sx(0x003e_3b6c);
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x0040_13e8, 0x003e_3b6c)?;
                    pc = 0x003e_3b6c;
                    continue;
                }
                0x003e_3b6c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x63a4));
                    if t {
                        pc = 0x003e_3ba4;
                        continue;
                    }
                    pc = 0x003e_3b78;
                    continue;
                }
                0x003e_3b78 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[A2][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x003e_3b8c);
                    c.g[A3][0] = c.g[S0][0];
                    call_ptr(c, 0x0040_1df8, 0x003e_3b8c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x003e_3b58) {
                            return Ok(());
                        }
                        pc = 0x003e_3b58;
                        continue;
                    }
                    pc = 0x003e_3b94;
                    continue;
                }
                0x003e_3b94 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x1c));
                    c.g[S4][0] = 0x1;
                    c.g[V0][0] = c.g[V0][0] | 0x2;
                    c.sw((c.g[S0][0] as u32).wrapping_sub(0x1c), c.g[V0][0]);
                    pc = 0x003e_3ba4;
                    continue;
                }
                0x003e_3ba4 => {
                    c.cycles += 2;
                    let t = c.g[S4][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x003e_3bbc;
                        continue;
                    }
                    pc = 0x003e_3bac;
                    continue;
                }
                0x003e_3bac => {
                    c.cycles += 4;
                    c.g[V1][0] = EA_BSS_HI;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x63c0));
                    c.g[V0][0] = c.g[V0][0] | 0x6;
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
                    pc = 0x003e_3bbc;
                    continue;
                }
                0x003e_3bbc => {
                    c.cycles += 8;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003e_3b00, 0x003e_3bdc);
                    return Ok(());
                }
            }
        }
    })
}

/// `void EAFile_CdShutdown(void)` (0x3e44b0): tear the CD layer down:
/// remove the timer callback, then terminate and delete the read thread and
/// delete its semaphore.
pub fn ea_file_cd_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = 0x3e0000;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_44c8);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4000));
        callee::EA_TIMER_REMOVE_CALLBACK.call(c, 0x003e_44c8)?;
        c.g[RA][0] = sx(0x003e_44d0);
        c.g[A0][0] = 0u64;
        call_ptr(c, 0x0040_08a0, 0x003e_44d0)?;
        c.g[S0][0] = EA_BSS_HI;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x63c0));
        c.g[RA][0] = sx(0x003e_44e0);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        call_ptr(c, 0x0042_3bf0, 0x003e_44e0)?;
        c.g[RA][0] = sx(0x003e_44e8);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        call_ptr(c, 0x0042_3bb0, 0x003e_44e8)?;
        c.g[RA][0] = sx(0x003e_44f0);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        call_ptr(c, 0x0042_3db0, 0x003e_44f0)?;
        c.g[RA][0] = sx(0x003e_44f8);
        c.g[A0][0] = 0x5;
        call_ptr(c, 0x0040_1718, 0x003e_44f8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int EAFile_HostWriteFileChecked(const char *path, const void *buf,
/// unsigned len)` (0x3e2030): write `len` bytes of `buf` to the host file
/// `path` (prefixing the host root when missing), checking every `sceWrite`
/// for an error. The write loop and the close check both steer early exits.
/// Always returns 0.
pub fn ea_file_host_write_file_checked(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_B0: u32 = 0x003e_20b0;
        c.cycles += 19;
        let frame = Frame::open(c, 0x150);
        frame.save(c, S1, 0x130);
        frame.save(c, S3, 0x110);
        c.g[S1][0] = EA_DATA_HI;
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x140);
        frame.save(c, S2, 0x120);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc50));
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x100);
        c.g[RA][0] = sx(0x003e_2064);
        c.sb(c.g[SP][0] as u32, 0u64);
        call_ptr(c, 0x0041_6810, 0x003e_2064)?;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc50));
        c.g[A2][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x003e_2074);
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0041_6b18, 0x003e_2074)?;
        let t = c.g[V0][0] == 0u64;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc50));
        'b_3e2084: {
            if t {
                break 'b_3e2084;
            }
            c.cycles += 2;
            c.g[RA][0] = sx(0x003e_2084);
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0041_66f4, 0x003e_2084)?;
            break 'b_3e2084;
        }
        'b_3e2110: {
            'b_3e210c: {
                c.cycles += 9;
                c.g[A0][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x003e_2090);
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x0041_62d0, 0x003e_2090)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x003e_209c);
                c.g[A1][0] = 0x602;
                call_ptr(c, 0x0042_7d60, 0x003e_209c)?;
                c.g[S1][0] = c.g[V0][0];
                let t = (c.g[S1][0] as i64) < 0;
                c.g[V0][0] = !(0u64 | c.g[S1][0]);
                if t {
                    break 'b_3e210c;
                }
                c.cycles += 2;
                c.g[V0][0] = ((c.g[S0][0] as i64) < 4097i64) as u64;
                'l_3e20b0: loop {
                    'b_3e2104: {
                        'b_3e20ec: {
                            'b_3e20d8: {
                                c.cycles += 2;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A2][0] = c.g[S0][0];
                                if t {
                                    break 'b_3e20d8;
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = c.g[S2][0];
                                c.g[RA][0] = sx(0x003e_20c8);
                                c.g[A2][0] = 0x1000;
                                call_ptr(c, 0x0042_8600, 0x003e_20c8)?;
                                let t = (c.g[V0][0] as i64) < 0;
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1000));
                                if t {
                                    break 'b_3e2104;
                                }
                                c.cycles += 2;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1000));
                                break 'b_3e20ec;
                            }
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x003e_20e4);
                            c.g[A1][0] = c.g[S2][0];
                            call_ptr(c, 0x0042_8600, 0x003e_20e4)?;
                            let t = (c.g[V0][0] as i64) < 0;
                            c.g[S0][0] = 0u64;
                            if t {
                                break 'b_3e2104;
                            }
                            break 'b_3e20ec;
                        }
                        c.cycles += 2;
                        let t = (c.g[S0][0] as i64) > 0;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < 4097i64) as u64;
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, HEAD_B0);
                            }
                            continue 'l_3e20b0;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x003e_20fc);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x0042_7fe8, 0x003e_20fc)?;
                        let t = (c.g[V0][0] as i64) >= 0;
                        c.g[V0][0] = !(0u64 | c.g[S1][0]);
                        if t {
                            break 'b_3e210c;
                        }
                        break 'b_3e2104;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0u64;
                    break 'b_3e2110;
                }
            }
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 31);
        break 'b_3e2110;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x140);
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore(c, S3, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `int EAFile_HostWriteFile(const char *path, const void *buf, unsigned len)`
/// (0x3e1f48): write `len` bytes of `buf` to the host file `path`
/// (prefixing the host root when missing), without per-write error checks.
/// Always returns 0.
pub fn ea_file_host_write_file(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_C8: u32 = 0x003e_1fc8;
        c.cycles += 19;
        let frame = Frame::open(c, 0x150);
        frame.save(c, S1, 0x130);
        frame.save(c, S3, 0x110);
        c.g[S1][0] = EA_DATA_HI;
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x140);
        frame.save(c, S2, 0x120);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc50));
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x100);
        c.g[RA][0] = sx(0x003e_1f7c);
        c.sb(c.g[SP][0] as u32, 0u64);
        call_ptr(c, 0x0041_6810, 0x003e_1f7c)?;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc50));
        c.g[A2][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x003e_1f8c);
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0041_6b18, 0x003e_1f8c)?;
        let t = c.g[V0][0] == 0u64;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc50));
        'b_3e1f9c: {
            if t {
                break 'b_3e1f9c;
            }
            c.cycles += 2;
            c.g[RA][0] = sx(0x003e_1f9c);
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0041_66f4, 0x003e_1f9c)?;
            break 'b_3e1f9c;
        }
        'b_3e2010: {
            c.cycles += 9;
            c.g[A0][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x003e_1fa8);
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x0041_62d0, 0x003e_1fa8)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x003e_1fb4);
            c.g[A1][0] = 0x602;
            call_ptr(c, 0x0042_7d60, 0x003e_1fb4)?;
            c.g[S1][0] = c.g[V0][0];
            let t = (c.g[S1][0] as i64) < 0;
            c.g[V0][0] = !(0u64 | c.g[S1][0]);
            if t {
                break 'b_3e2010;
            }
            c.cycles += 2;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 4097i64) as u64;
            'l_3e1fc8: loop {
                'b_3e1ffc: {
                    'b_3e1fec: {
                        c.cycles += 2;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A2][0] = c.g[S0][0];
                        if t {
                            break 'b_3e1fec;
                        }
                        c.cycles += 7;
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A2][0] = 0x1000;
                        c.g[RA][0] = sx(0x003e_1fe4);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1000));
                        call_ptr(c, 0x0042_8600, 0x003e_1fe4)?;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1000));
                        break 'b_3e1ffc;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x003e_1ffc);
                    c.g[S0][0] = 0u64;
                    call_ptr(c, 0x0042_8600, 0x003e_1ffc)?;
                    break 'b_3e1ffc;
                }
                c.cycles += 2;
                let t = (c.g[S0][0] as i64) > 0;
                c.g[V0][0] = ((c.g[S0][0] as i64) < 4097i64) as u64;
                if t {
                    if event_due(c) {
                        deliver_event_here(c, HEAD_C8);
                    }
                    continue 'l_3e1fc8;
                }
                c.cycles += 3;
                c.g[RA][0] = sx(0x003e_200c);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0042_7fe8, 0x003e_200c)?;
                c.g[V0][0] = !(0u64 | c.g[S1][0]);
                break 'b_3e2010;
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x140);
        frame.restore(c, S1, 0x130);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 31);
        frame.restore(c, S2, 0x120);
        frame.restore(c, S3, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_Shutdown(void)` (0x3dcde0): shut the EA file system down:
/// shut the CD layer down, close every open request (calling each request's
/// cleanup first), destroy the request mutex and clear it, then run the
/// shutdown callback when one is registered.
pub fn ea_file_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_18: u32 = 0x003d_ce18;
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_cdfc);
        c.g[S2][0] = EA_BSS_HI;
        callee::EA_FILE_CD_SHUTDOWN.call(c, 0x003d_cdfc)?;
        c.g[V0][0] = EA_BSS_HI;
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x651c));
        let t = c.g[S0][0] == 0u64;
        c.g[V0][0] = EA_DATA_HI;
        'b_3dce3c: {
            if t {
                break 'b_3dce3c;
            }
            c.cycles += 3;
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc10));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            'l_3dce18: loop {
                c.cycles += 9;
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003d_ce20);
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                call_ptr(c, t, 0x003d_ce20)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x003d_ce30);
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                callee::EA_FILE_CLOSE.call(c, 0x003d_ce30)?;
                c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                if c.g[S0][0] != 0u64 {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if event_due(c) {
                        deliver_event_here(c, HEAD_18);
                    }
                    continue 'l_3dce18;
                }
                break 'b_3dce3c;
            }
        }
        'b_3dce74: {
            c.cycles += 10;
            c.g[S0][0] = EA_BSS_HI;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x653c));
            c.g[RA][0] = sx(0x003d_ce4c);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_MUTEX_DESTROY.call(c, 0x003d_ce4c)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x14));
            c.g[RA][0] = sx(0x003d_ce58);
            c.g[A1][0] = 0x38;
            callee::EA_MEM_ZERO.call(c, 0x003d_ce58)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x6518));
            let t = c.g[A0][0] == 0u64;
            c.g[V1][0] = EA_DATA_HI;
            if t {
                break 'b_3dce74;
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc18));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003d_ce70);
            call_ptr(c, t, 0x003d_ce70)?;
            c.sw((c.g[S2][0] as u32).wrapping_sub(0x6518), 0u64);
            break 'b_3dce74;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::{self, covered, hook, machine, LOG_UNWIND_AT};
    use crate::difftest::{Rng, DATA};
    use crate::game::ealib::seg_2dcae8::tests::oc697_hook_all;
    use crate::game::lib::eafile::tests::{
        ea_fixed_costs, oc697_open, oc697_script, put_ascii,
    };
    use std::ptr::addr_of_mut;

    /// Gate word for this module's slot fakes (per-machine RAM): with
    /// [`EA_FILE_TEST_MAGIC`] set, the fakes below answer from their result cell;
    /// otherwise they run the body the slot held before.
    pub(crate) const EA_FILE_TEST_CTRL: u32 = DATA + 0x8_0000;

    pub(crate) const EA_FILE_TEST_MAGIC: u32 = 0x1332_0c32;

    /// Fixed cost of a gated test-fake call on both sides.
    const EA_FILE_TEST_FAKE_COST: u64 = 8;

    pub(crate) const EA_FILE_TEST_RET_SCE_OPEN: u32 = DATA + 0x8_0010;
    pub(crate) const EA_FILE_TEST_RET_CD_CALLBACK: u32 = DATA + 0x8_0014;
    pub(crate) const EA_FILE_TEST_RET_CD_INIT_EE_CB: u32 = DATA + 0x8_0018;
    pub(crate) const EA_FILE_TEST_RET_CD_GET_TOC: u32 = DATA + 0x8_001c;
    pub(crate) const EA_FILE_TEST_RET_CD_GET_DISK_TYPE: u32 = DATA + 0x8_0020;
    pub(crate) const EA_FILE_TEST_RET_SIGNAL_SEMA: u32 = DATA + 0x8_0024;
    pub(crate) const EA_FILE_TEST_RET_FLUSH_CACHE: u32 = DATA + 0x8_0028;
    pub(crate) const EA_FILE_TEST_RET_START_THREAD: u32 = DATA + 0x8_002c;
    pub(crate) const EA_FILE_TEST_RET_TIMER_ADD: u32 = DATA + 0x8_0030;
    pub(crate) const EA_FILE_TEST_RET_TIMER_REMOVE: u32 = DATA + 0x8_0034;
    pub(crate) const EA_FILE_TEST_RET_MUTEX_DESTROY: u32 = DATA + 0x8_0038;
    pub(crate) const EA_FILE_TEST_RET_CD_RESET: u32 = DATA + 0x8_003c;
    pub(crate) const EA_FILE_TEST_RET_CD_SCAN: u32 = DATA + 0x8_0040;
    pub(crate) const EA_FILE_TEST_RET_BUILD_HASH: u32 = DATA + 0x8_0044;
    pub(crate) const EA_FILE_TEST_RET_CLOSE: u32 = DATA + 0x8_0048;

    macro_rules! ea_file_fake_ret {
        ($name:ident, $gen:expr, $cell:expr, $id:expr) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(EA_FILE_TEST_CTRL) != EA_FILE_TEST_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                let ra = c.g[RA][0] as u32;
                let before = c.cycles;
                fakes::fake(c, $id, ra);
                c.cycles = before + EA_FILE_TEST_FAKE_COST;
                c.g[V0][0] = sx(c.bus.read32($cell));
            }
        };
    }

    ea_file_fake_ret!(
        ea_file_fake_sce_open,
        ee::s15::f_00427d60__generated,
        EA_FILE_TEST_RET_SCE_OPEN,
        0x1332_0001
    );
    ea_file_fake_ret!(
        ea_file_fake_cd_callback,
        ee::s15::f_004008a0__generated,
        EA_FILE_TEST_RET_CD_CALLBACK,
        0x1332_0002
    );
    ea_file_fake_ret!(
        ea_file_fake_cd_init_ee_cb,
        ee::s15::f_00400a78__generated,
        EA_FILE_TEST_RET_CD_INIT_EE_CB,
        0x1332_0003
    );
    ea_file_fake_ret!(
        ea_file_fake_cd_get_toc,
        ee::s15::f_00401fd8__generated,
        EA_FILE_TEST_RET_CD_GET_TOC,
        0x1332_0004
    );
    ea_file_fake_ret!(
        ea_file_fake_cd_get_disk_type,
        ee::s15::f_00402268__generated,
        EA_FILE_TEST_RET_CD_GET_DISK_TYPE,
        0x1332_0005
    );
    ea_file_fake_ret!(
        ea_file_fake_signal_sema,
        ee::s15::f_00423dc0__generated,
        EA_FILE_TEST_RET_SIGNAL_SEMA,
        0x1332_0006
    );
    ea_file_fake_ret!(
        ea_file_fake_flush_cache,
        ee::s15::f_00424020__generated,
        EA_FILE_TEST_RET_FLUSH_CACHE,
        0x1332_0007
    );
    ea_file_fake_ret!(
        ea_file_fake_start_thread,
        ee::s15::f_00423bc0__generated,
        EA_FILE_TEST_RET_START_THREAD,
        0x1332_0008
    );
    ea_file_fake_ret!(
        ea_file_fake_timer_add,
        ee::s14::f_003e57f8__generated,
        EA_FILE_TEST_RET_TIMER_ADD,
        0x1332_0009
    );
    ea_file_fake_ret!(
        ea_file_fake_timer_remove,
        ee::s14::f_003e58c8__generated,
        EA_FILE_TEST_RET_TIMER_REMOVE,
        0x1332_000a
    );
    ea_file_fake_ret!(
        ea_file_fake_mutex_destroy,
        ee::s14::f_003e56e0__generated,
        EA_FILE_TEST_RET_MUTEX_DESTROY,
        0x1332_000b
    );
    ea_file_fake_ret!(
        ea_file_fake_cd_reset,
        ee::s14::f_003e3618__generated,
        EA_FILE_TEST_RET_CD_RESET,
        0x1332_000c
    );
    ea_file_fake_ret!(
        ea_file_fake_cd_scan_dir,
        ee::s14::f_003e3758__generated,
        EA_FILE_TEST_RET_CD_SCAN,
        0x1332_000d
    );
    ea_file_fake_ret!(
        ea_file_fake_cd_build_hash,
        ee::s14::f_003e39a8__generated,
        EA_FILE_TEST_RET_BUILD_HASH,
        0x1332_000e
    );
    ea_file_fake_ret!(
        ea_file_fake_close,
        ee::s14::f_003deca0__generated,
        EA_FILE_TEST_RET_CLOSE,
        0x1332_000f
    );

    /// Hook this module's slot fakes (once per process; all gated).
    pub(crate) fn ea_file_install_test_fakes() {
        let hooks: [(*mut fn(&mut Cpu), fn(&mut Cpu)); 15] = [
            (
                addr_of_mut!(ee::s15::OVERRIDE_00427D60),
                ea_file_fake_sce_open,
            ),
            (
                addr_of_mut!(ee::s15::OVERRIDE_004008A0),
                ea_file_fake_cd_callback,
            ),
            (
                addr_of_mut!(ee::s15::OVERRIDE_00400A78),
                ea_file_fake_cd_init_ee_cb,
            ),
            (
                addr_of_mut!(ee::s15::OVERRIDE_00401FD8),
                ea_file_fake_cd_get_toc,
            ),
            (
                addr_of_mut!(ee::s15::OVERRIDE_00402268),
                ea_file_fake_cd_get_disk_type,
            ),
            (
                addr_of_mut!(ee::s15::OVERRIDE_00423DC0),
                ea_file_fake_signal_sema,
            ),
            (
                addr_of_mut!(ee::s15::OVERRIDE_00424020),
                ea_file_fake_flush_cache,
            ),
            (
                addr_of_mut!(ee::s15::OVERRIDE_00423BC0),
                ea_file_fake_start_thread,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E57F8),
                ea_file_fake_timer_add,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E58C8),
                ea_file_fake_timer_remove,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E56E0),
                ea_file_fake_mutex_destroy,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E3618),
                ea_file_fake_cd_reset,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E3758),
                ea_file_fake_cd_scan_dir,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E39A8),
                ea_file_fake_cd_build_hash,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003DECA0),
                ea_file_fake_close,
            ),
        ];
        for (slot, fake) in hooks {
            hook(slot, fake, fakes::prev_cell(fake));
        }
    }

    /// Open this module's fake gate with fakes unwinding at call `unwind`
    /// (`u32::MAX`: never).
    pub(crate) fn ea_file_test_open(c: &mut Cpu, unwind: u32) {
        ea_file_install_test_fakes();
        oc697_hook_all();
        c.bus.write32(EA_FILE_TEST_CTRL, EA_FILE_TEST_MAGIC);
        c.bus.write32(LOG_UNWIND_AT, unwind);
    }

    // -------------------------------------------------------------------
    // EAFile_CdInit

    /// The `CdInit` fixture: `sceCdInit` failing (early exit) or not; the
    /// disc type (other/`0x12`/`0x14`); the request list (empty, one entry,
    /// or an empty first entry); the file count against the table count.
    fn cd_init_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3e_4040, 0);
        oc697_open(c, u32::MAX);
        ea_file_test_open(
            c,
            if seed % 4 == 0 {
                1 + r.next_u32() % 30
            } else {
                u32::MAX
            },
        );
        ea_fixed_costs(c);
        c.pc = 0x003e_4040;
        // S6 steers the second MemSet length `([0x450c5c] + 12) * S6`.
        c.g[S6][0] = (seed % 3) as u64;
        c.g[A0][0] = (seed % 6) as u64;
        c.g[A1][0] = (DATA + 0x2000) as u64;
        c.bus.write32(0x0045_0c5c, 0x10 + seed / 3 % 0x10);
        c.bus.write32(0x0045_0c58, 0);
        c.bus.write32(0x0045_0c68, DATA + 0x3000);
        c.bus.write32(0x0045_0c6c, if seed / 9 % 2 == 0 { 0x20 } else { 0 });
        c.bus.write32(0x0045_0c14, DATA + 0x5000);
        c.bus.write32(0x0045_0c10, 0);
        c.bus.write32(0x0045_0c18, DATA + 0x0911);
        for w in 0..0x40 {
            c.bus.write32(DATA + 0x3000 + 4 * w, 0xa5a5_0000 | w);
        }
        oc697_script(
            c,
            0x0040_1718,
            if seed % 3 == 0 { &[0] } else { &[1] },
        );
        c.bus.write32(
            EA_FILE_TEST_RET_CD_GET_DISK_TYPE,
            [0x10, 0x12, 0x14][(seed / 3 % 3) as usize],
        );
        // The reset answer is a scratch sector buffer the scan tail reads.
        c.bus.write32(EA_FILE_TEST_RET_CD_RESET, DATA + 0x6000);
        for w in 0..0x10 {
            c.bus.write32(DATA + 0x6000 + 4 * w, 0xb5b5_0000 | w);
        }
        c.bus.write32(EA_FILE_TEST_RET_CD_SCAN, 0);
        c.bus.write32(EA_FILE_TEST_RET_BUILD_HASH, 0);
        c.bus.write32(EA_FILE_TEST_RET_CD_CALLBACK, 0);
        c.bus.write32(EA_FILE_TEST_RET_CD_INIT_EE_CB, 0);
        c.bus.write32(EA_FILE_TEST_RET_TIMER_ADD, 0);
        c.bus.write32(EA_FILE_TEST_RET_FLUSH_CACHE, 0);
        c.bus.write32(EA_FILE_TEST_RET_CD_GET_TOC, 0);
        c.bus.write32(EA_FILE_TEST_RET_START_THREAD, 0);
        match seed / 18 % 3 {
            0 => c.bus.write32(0x0045_0c30, 0),
            1 => {
                c.bus.write32(0x0045_0c30, DATA + 0x1000);
                put_ascii(c, DATA + 0x1100, "AB.;1");
                put_ascii(c, DATA + 0x1200, "");
                put_ascii(c, DATA + 0x1300, "1234567ABC");
                c.bus.write32(DATA + 0x1000, DATA + 0x1100);
                c.bus.write32(DATA + 0x1004, 0xa5a5_0001);
                c.bus.write32(DATA + 0x1008, 0xa5a5_0002);
                c.bus.write32(DATA + 0x100c, DATA + 0x1300);
                c.bus.write32(DATA + 0x1010, DATA + 0x1200);
                c.bus.write32(DATA + 0x1014, 0xa5a5_0005);
                c.bus.write32(DATA + 0x1018, 0xa5a5_0006);
                c.bus.write32(DATA + 0x101c, 0xa5a5_0007);
            }
            _ => {
                c.bus.write32(0x0045_0c30, DATA + 0x1000);
                put_ascii(c, DATA + 0x1200, "");
                c.bus.write32(DATA + 0x1000, DATA + 0x1200);
            }
        }
        c.bus.write32(0x0051_9c40, seed % 2);
    }

    /// `EAFile_CdInit`: init failure, bad disc type, the empty/missing
    /// request list, the entry walk, and the free/build tail, with fakes
    /// unwinding on some seeds.
    #[test]
    fn ea_file_cd_init_matches() {
        covered(ee::s14::f_003e4040__generated, ea_file_cd_init, 200, &cd_init_setup);
    }

    driven!(gen_ea_file_cd_init, ee::s14::f_003e4040__generated);

    driven!(hand_ea_file_cd_init, ea_file_cd_init);

    /// `EAFile_CdInit` on a running thread with events due in the entry
    /// fixup loops.
    #[test]
    fn ea_file_cd_init_events() {
        check_events(gen_ea_file_cd_init, hand_ea_file_cd_init, 96, &|c, seed| {
            let mut r = Rng(0x003e_4040 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
            machine(c, seed, 0x3e_4040, 0);
            ea_file_test_open(c, u32::MAX);
            oc697_open(c, u32::MAX);
            ea_fixed_costs(c);
            c.g[S6][0] = 1;
            c.g[A0][0] = 0;
            c.g[A1][0] = (DATA + 0x2000) as u64;
            c.bus.write32(0x0045_0c5c, 0x10);
            c.bus.write32(0x0045_0c58, 0);
            c.bus.write32(0x0045_0c68, DATA + 0x3000);
            c.bus.write32(0x0045_0c6c, 0);
            c.bus.write32(0x0045_0c14, DATA + 0x5000);
            c.bus.write32(0x0045_0c10, 0);
            c.bus.write32(0x0045_0c18, DATA + 0x0911);
            for w in 0..0x40 {
                c.bus.write32(DATA + 0x3000 + 4 * w, 0xa5a5_0000 | w);
            }
            oc697_script(c, 0x0040_1718, &[1]);
            c.bus.write32(EA_FILE_TEST_RET_CD_GET_DISK_TYPE, 0x12);
            c.bus.write32(EA_FILE_TEST_RET_CD_RESET, DATA + 0x6000);
            for w in 0..0x10 {
                c.bus.write32(DATA + 0x6000 + 4 * w, 0xb5b5_0000 | w);
            }
            c.bus.write32(EA_FILE_TEST_RET_CD_SCAN, 0);
            c.bus.write32(EA_FILE_TEST_RET_BUILD_HASH, 0);
            c.bus.write32(EA_FILE_TEST_RET_CD_CALLBACK, 0);
            c.bus.write32(EA_FILE_TEST_RET_CD_INIT_EE_CB, 0);
            c.bus.write32(EA_FILE_TEST_RET_TIMER_ADD, 0);
            c.bus.write32(EA_FILE_TEST_RET_FLUSH_CACHE, 0);
            c.bus.write32(EA_FILE_TEST_RET_CD_GET_TOC, 0);
            c.bus.write32(EA_FILE_TEST_RET_START_THREAD, 0);
            c.bus.write32(0x0045_0c30, DATA + 0x1000);
            put_ascii(c, DATA + 0x1100, "AB.;1");
            put_ascii(c, DATA + 0x1200, "");
            put_ascii(c, DATA + 0x1300, "1234567ABC");
            put_ascii(c, DATA + 0x1400, "CD.;1");
            put_ascii(c, DATA + 0x1500, "1234567DEF");
            for i in 0..2u32 {
                let base = DATA + 0x1000 + 0x20 * i;
                c.bus.write32(base, DATA + 0x1100 + 0x300 * i);
                c.bus.write32(base + 4, 0xa5a5_0001);
                c.bus.write32(base + 8, 0xa5a5_0002);
                c.bus.write32(base + 12, DATA + 0x1300 + 0x200 * i);
            }
            c.bus.write32(DATA + 0x1040, DATA + 0x1200);
            c.bus.write32(0x0051_9c40, seed % 2);
            schedule(c, &mut r, seed, 0x003e_4040)
        });
    }

    // -------------------------------------------------------------------
    // EAFile_CdMountDisc

    /// The `MountDisc` fixture: the CD state words the entry copy spreads
    /// over the frame (`[0x519c40]` flags, `[0x519c50]` the sector count,
    /// `[0x519c54]` the sector target); the init/sync/read/error scripts
    /// steering the poll loops and the outer re-mount loop.
    fn cd_mount_disc_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3e_3d78, 0);
        oc697_open(c, u32::MAX);
        ea_file_test_open(
            c,
            if seed % 4 == 0 {
                1 + r.next_u32() % 40
            } else {
                u32::MAX
            },
        );
        ea_fixed_costs(c);
        c.pc = 0x003e_3d78;
        c.bus.write32(0x0051_9c40, seed % 2);
        c.bus.write32(0x0051_9c44, 0xa5a5_1111);
        c.bus.write32(0x0051_9c48, 0xa5a5_2222);
        c.bus.write32(0x0051_9c4c, 0x10);
        c.bus
            .write32(0x0051_9c50, if seed / 12 % 2 == 0 { 0 } else { 2 });
        c.bus.write32(0x0051_9c54, DATA + 0x6000);
        c.bus.write32(0x0051_9c7c, DATA + 0x6100);
        c.bus.write32(0x0051_9bfc, 0xa5a5_3333);
        let mut init = vec![0; (seed % 3) as usize];
        init.extend([1, 1]);
        oc697_script(c, 0x0040_1718, &init);
        oc697_script(c, 0x0040_13e8, if seed / 3 % 2 == 0 { &[0] } else { &[1, 0] });
        oc697_script(c, 0x0040_1df8, if seed % 2 == 0 { &[1] } else { &[0, 1] });
        oc697_script(
            c,
            0x0040_2300,
            if seed / 6 % 2 == 0 { &[0] } else { &[1, 1, 0] },
        );
        c.bus.write32(EA_FILE_TEST_RET_SIGNAL_SEMA, 0);
        c.bus.write32(EA_FILE_TEST_RET_CD_CALLBACK, 0);
        c.bus.write32(EA_FILE_TEST_RET_CD_INIT_EE_CB, 0);
    }

    /// `EAFile_CdMountDisc`: the init/delay/sync/read poll loops, the
    /// sector-advance loop and the outer re-mount loop, with fakes
    /// unwinding on some seeds.
    #[test]
    fn ea_file_cd_mount_disc_matches() {
        covered(
            ee::s14::f_003e3d78__generated,
            ea_file_cd_mount_disc,
            120,
            &cd_mount_disc_setup,
        );
    }

    driven!(gen_ea_file_cd_mount_disc, ee::s14::f_003e3d78__generated);

    driven!(hand_ea_file_cd_mount_disc, ea_file_cd_mount_disc);

    /// `EAFile_CdMountDisc` on a running thread with events due in the
    /// poll and sector loops.
    #[test]
    fn ea_file_cd_mount_disc_events() {
        check_events(
            gen_ea_file_cd_mount_disc,
            hand_ea_file_cd_mount_disc,
            96,
            &|c, seed| {
                let mut r =
                    Rng(0x003e_3d78 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                machine(c, seed, 0x3e_3d78, 0);
                ea_file_test_open(c, u32::MAX);
                oc697_open(c, u32::MAX);
                ea_fixed_costs(c);
                c.bus.write32(0x0051_9c40, seed % 2);
                c.bus.write32(0x0051_9c44, 0xa5a5_1111);
                c.bus.write32(0x0051_9c48, 0xa5a5_2222);
                c.bus.write32(0x0051_9c4c, 0x10);
                c.bus.write32(0x0051_9c50, 2);
                c.bus.write32(0x0051_9c54, DATA + 0x6000);
                c.bus.write32(0x0051_9c7c, DATA + 0x6100);
                c.bus.write32(0x0051_9bfc, 0xa5a5_3333);
                oc697_script(c, 0x0040_1718, &[0, 0, 1, 1]);
                oc697_script(c, 0x0040_13e8, &[1, 0]);
                oc697_script(c, 0x0040_1df8, &[0, 1]);
                oc697_script(c, 0x0040_2300, &[0]);
                c.bus.write32(EA_FILE_TEST_RET_SIGNAL_SEMA, 0);
                c.bus.write32(EA_FILE_TEST_RET_CD_CALLBACK, 0);
                c.bus.write32(EA_FILE_TEST_RET_CD_INIT_EE_CB, 0);
                schedule(c, &mut r, seed, 0x003e_3d78)
            },
        );
    }

    // -------------------------------------------------------------------
    // EAFile_CdIssueRead

    /// The `IssueRead` fixture: the flag bit choosing the request word, the
    /// request fields, and the read script (failures before the success, or
    /// three failures for the give-up arm).
    fn cd_issue_read_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3e_3b00, 0);
        oc697_open(c, u32::MAX);
        ea_file_test_open(
            c,
            if seed % 4 == 0 {
                1 + r.next_u32() % 10
            } else {
                u32::MAX
            },
        );
        ea_fixed_costs(c);
        c.pc = 0x003e_3b00;
        c.bus.write32(0x0051_9c40, seed % 2);
        c.bus.write32(0x0051_9c6c, DATA + 0x4000);
        c.bus.write32(0x0051_9c70, DATA + 0x4100);
        c.bus.write32(0x0051_9c74, DATA + 0x7000);
        c.bus.write32(0x0051_9c7c, DATA + 0x7000);
        c.bus.write32(0x0051_9b40, 0xa5a5_5a5a);
        for w in 0..0x10 {
            c.bus.write32(DATA + 0x4000 + 4 * w, 0xc5c5_0000 | w);
            c.bus.write32(DATA + 0x7000 + 4 * w, 0xd5d5_0000 | w);
        }
        let k = seed / 2 % 4;
        let mut reads = vec![0; k.min(3) as usize];
        if k < 3 {
            reads.push(1);
        }
        oc697_script(c, 0x0040_1df8, &reads);
    }

    /// `EAFile_CdIssueRead`: the flag arms, success on each try and the
    /// three-failure give-up, with fakes unwinding on some seeds.
    #[test]
    fn ea_file_cd_issue_read_matches() {
        covered(
            ee::s14::f_003e3b00__generated,
            ea_file_cd_issue_read,
            120,
            &cd_issue_read_setup,
        );
    }

    driven!(gen_ea_file_cd_issue_read, ee::s14::f_003e3b00__generated);

    driven!(hand_ea_file_cd_issue_read, ea_file_cd_issue_read);

    /// `EAFile_CdIssueRead` on a running thread with events due in the
    /// read-retry loop.
    #[test]
    fn ea_file_cd_issue_read_events() {
        check_events(
            gen_ea_file_cd_issue_read,
            hand_ea_file_cd_issue_read,
            96,
            &|c, seed| {
                let mut r =
                    Rng(0x003e_3b00 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                machine(c, seed, 0x3e_3b00, 0);
                ea_file_test_open(c, u32::MAX);
                oc697_open(c, u32::MAX);
                ea_fixed_costs(c);
                c.bus.write32(0x0051_9c40, seed % 2);
                c.bus.write32(0x0051_9c6c, DATA + 0x4000);
                c.bus.write32(0x0051_9c70, DATA + 0x4100);
                c.bus.write32(0x0051_9c74, DATA + 0x7000);
                c.bus.write32(0x0051_9c7c, DATA + 0x7000);
                c.bus.write32(0x0051_9b40, 0xa5a5_5a5a);
                for w in 0..0x10 {
                    c.bus.write32(DATA + 0x4000 + 4 * w, 0xc5c5_0000 | w);
                    c.bus.write32(DATA + 0x7000 + 4 * w, 0xd5d5_0000 | w);
                }
                oc697_script(c, 0x0040_1df8, &[0, 0, 1]);
                schedule(c, &mut r, seed, 0x003e_3b00)
            },
        );
    }

    // -------------------------------------------------------------------
    // EAFile_CdShutdown

    /// `EAFile_CdShutdown`: the timer callback removed, the thread and its
    /// semaphore deleted, with the first fake unwinding on some seeds.
    #[test]
    fn ea_file_cd_shutdown_matches() {
        covered(ee::s14::f_003e44b0__generated, ea_file_cd_shutdown, 64, &|c, seed| {
            let _r = machine(c, seed, 0x3e_44b0, 0);
            oc697_open(c, u32::MAX);
            ea_file_test_open(c, if seed % 3 == 0 { 0 } else { u32::MAX });
            ea_fixed_costs(c);
            c.pc = 0x003e_44b0;
            c.bus.write32(EA_FILE_TEST_RET_TIMER_REMOVE, 0);
            c.bus.write32(EA_FILE_TEST_RET_CD_CALLBACK, 0);
            c.bus.write32(0x0051_9c48, 0x10 + seed % 0xf0);
            c.bus.write32(0x0051_9c4c, 0x20 + seed % 0xf0);
        });
    }

    // -------------------------------------------------------------------
    // EAFile_HostWriteFileChecked / EAFile_HostWriteFile

    /// The host-write fixture: the host root, a path with or without it,
    /// a non-zero buffer, a short or multi-chunk length, and the open
    /// answer (failure or descriptor 3). The real `sceWrite`/`sceClose`
    /// fail fast with the file library down.
    fn host_write_setup(c: &mut Cpu, seed: u32, checked: bool) {
        let mut r = machine(
            c,
            seed,
            if checked { 0x3e_2030 } else { 0x3e_1f48 },
            0,
        );
        oc697_open(c, u32::MAX);
        ea_file_test_open(
            c,
            if seed % 4 == 0 {
                1 + r.next_u32() % 12
            } else {
                u32::MAX
            },
        );
        ea_fixed_costs(c);
        c.pc = if checked { 0x003e_2030 } else { 0x003e_1f48 };
        c.bus.write32(0x0045_0c50, DATA + 0x100);
        put_ascii(c, DATA + 0x100, "host0:");
        if seed % 2 == 0 {
            put_ascii(c, DATA + 0x200, "host0:save");
        } else {
            put_ascii(c, DATA + 0x200, "save0000");
        }
        for w in 0..0x400 {
            c.bus
                .write32(DATA + 0x1000 + 4 * w, 0xa5a5_0000 | (w & 0xffff));
        }
        c.bus.write32(0x0045_520c, 0);
        c.bus.write32(
            EA_FILE_TEST_RET_SCE_OPEN,
            if seed / 12 % 3 == 0 { 0xffff_ffed } else { 3 },
        );
        c.g[A0][0] = (DATA + 0x200) as u64;
        c.g[A1][0] = (DATA + 0x1000) as u64;
        c.g[A2][0] = [0, 100, 4096, 4097, 5000, 9000][(seed / 2 % 6) as usize] as u64;
    }

    /// `EAFile_HostWriteFileChecked`: prefixed and plain paths, open
    /// failure, short and multi-chunk writes with failing `sceWrite`s, and
    /// the close check, with fakes unwinding on some seeds.
    #[test]
    fn ea_file_host_write_file_checked_matches() {
        covered(
            ee::s14::f_003e2030__generated,
            ea_file_host_write_file_checked,
            120,
            &|c, seed| host_write_setup(c, seed, true),
        );
    }

    driven!(
        gen_ea_file_host_write_file_checked,
        ee::s14::f_003e2030__generated
    );

    driven!(
        hand_ea_file_host_write_file_checked,
        ea_file_host_write_file_checked
    );

    /// `EAFile_HostWriteFileChecked` on a running thread with events due
    /// in the multi-chunk write loop.
    #[test]
    fn ea_file_host_write_file_checked_events() {
        check_events(
            gen_ea_file_host_write_file_checked,
            hand_ea_file_host_write_file_checked,
            96,
            &|c, seed| {
                let mut r =
                    Rng(0x003e_2030 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                machine(c, seed, 0x3e_2030, 0);
                ea_file_test_open(c, u32::MAX);
                oc697_open(c, u32::MAX);
                ea_fixed_costs(c);
                c.bus.write32(0x0045_0c50, DATA + 0x100);
                put_ascii(c, DATA + 0x100, "host0:");
                if seed % 2 == 0 {
                    put_ascii(c, DATA + 0x200, "host0:save");
                } else {
                    put_ascii(c, DATA + 0x200, "save0000");
                }
                for w in 0..0x400 {
                    c.bus.write32(DATA + 0x1000 + 4 * w, 0xa5a5_0000 | w);
                }
                c.bus.write32(0x0045_520c, 0);
                c.bus.write32(EA_FILE_TEST_RET_SCE_OPEN, 3);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = 9000;
                schedule(c, &mut r, seed, 0x003e_2030)
            },
        );
    }

    /// `EAFile_HostWriteFile`: prefixed and plain paths, open failure and
    /// short and multi-chunk writes, with fakes unwinding on some seeds.
    #[test]
    fn ea_file_host_write_file_matches() {
        covered(
            ee::s14::f_003e1f48__generated,
            ea_file_host_write_file,
            120,
            &|c, seed| host_write_setup(c, seed, false),
        );
    }

    driven!(gen_ea_file_host_write_file, ee::s14::f_003e1f48__generated);

    driven!(hand_ea_file_host_write_file, ea_file_host_write_file);

    /// `EAFile_HostWriteFile` on a running thread with events due in the
    /// multi-chunk write loop.
    #[test]
    fn ea_file_host_write_file_events() {
        check_events(
            gen_ea_file_host_write_file,
            hand_ea_file_host_write_file,
            96,
            &|c, seed| {
                let mut r =
                    Rng(0x003e_1f48 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                machine(c, seed, 0x3e_1f48, 0);
                ea_file_test_open(c, u32::MAX);
                oc697_open(c, u32::MAX);
                ea_fixed_costs(c);
                c.bus.write32(0x0045_0c50, DATA + 0x100);
                put_ascii(c, DATA + 0x100, "host0:");
                if seed % 2 == 0 {
                    put_ascii(c, DATA + 0x200, "host0:save");
                } else {
                    put_ascii(c, DATA + 0x200, "save0000");
                }
                for w in 0..0x400 {
                    c.bus.write32(DATA + 0x1000 + 4 * w, 0xa5a5_0000 | w);
                }
                c.bus.write32(0x0045_520c, 0);
                c.bus.write32(EA_FILE_TEST_RET_SCE_OPEN, 3);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = 9000;
                schedule(c, &mut r, seed, 0x003e_1f48)
            },
        );
    }

    // -------------------------------------------------------------------
    // EAFile_Shutdown

    /// The `Shutdown` fixture: zero to two open requests on the list head
    /// and the shutdown callback word set or clear.
    fn ea_file_shutdown_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3d_cde0, 0);
        oc697_open(c, u32::MAX);
        ea_file_test_open(
            c,
            if seed % 4 == 0 {
                1 + r.next_u32() % 12
            } else {
                u32::MAX
            },
        );
        ea_fixed_costs(c);
        c.pc = 0x003d_cde0;
        c.bus.write32(0x0045_0c18, DATA + 0x0911);
        let nodes = seed % 3;
        c.bus.write32(
            0x0051_9ae4,
            if nodes == 0 {
                0
            } else {
                DATA + 0x1000
            },
        );
        for i in 0..nodes {
            let n = DATA + 0x1000 + 0x18 * i;
            let next = if i + 1 < nodes {
                DATA + 0x1000 + 0x18 * (i + 1)
            } else {
                0
            };
            c.bus.write32(n, DATA + 0x0911);
            c.bus.write32(n + 4, 0xa5a5_1000 + i);
            c.bus.write32(n + 8, 0xa5a5_2000 + i);
            c.bus.write32(n + 0xc, DATA + 0x1100 + 0x40 * i);
            c.bus.write32(n + 0x10, 0xa5a5_3000 + i);
            c.bus.write32(n + 0x14, next);
            c.bus.write32(DATA + 0x1100 + 0x40 * i, 0xc5c5_0001);
        }
        c.bus
            .write32(0x0051_9ae8, if seed / 3 % 2 == 0 { 0 } else { 1 });
        c.bus.write32(EA_FILE_TEST_RET_CLOSE, 0);
        c.bus.write32(EA_FILE_TEST_RET_MUTEX_DESTROY, 0);
        c.bus.write32(EA_FILE_TEST_RET_TIMER_REMOVE, 0);
        c.bus.write32(EA_FILE_TEST_RET_CD_CALLBACK, 0);
    }

    /// `EAFile_Shutdown`: the empty and non-empty request list and the
    /// shutdown callback set or clear, with fakes unwinding on some seeds.
    #[test]
    fn ea_file_shutdown_matches() {
        covered(
            ee::s14::f_003dcde0__generated,
            ea_file_shutdown,
            120,
            &ea_file_shutdown_setup,
        );
    }

    driven!(gen_ea_file_shutdown, ee::s14::f_003dcde0__generated);

    driven!(hand_ea_file_shutdown, ea_file_shutdown);

    /// `EAFile_Shutdown` on a running thread with events due in the
    /// request-close loop.
    #[test]
    fn ea_file_shutdown_events() {
        check_events(gen_ea_file_shutdown, hand_ea_file_shutdown, 96, &|c, seed| {
            let mut r =
                Rng(0x003d_cde0 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
            machine(c, seed, 0x3d_cde0, 0);
            ea_file_test_open(c, u32::MAX);
            oc697_open(c, u32::MAX);
            ea_fixed_costs(c);
            c.bus.write32(0x0045_0c18, DATA + 0x0911);
            c.bus.write32(0x0051_9ae4, DATA + 0x1000);
            for i in 0..3u32 {
                let n = DATA + 0x1000 + 0x18 * i;
                c.bus.write32(n, DATA + 0x0911);
                c.bus.write32(n + 4, 0xa5a5_1000 + i);
                c.bus.write32(n + 8, 0xa5a5_2000 + i);
                c.bus.write32(n + 0xc, DATA + 0x1100 + 0x40 * i);
                c.bus.write32(n + 0x10, 0xa5a5_3000 + i);
                c.bus.write32(
                    n + 0x14,
                    if i < 2 {
                        DATA + 0x1000 + 0x18 * (i + 1)
                    } else {
                        0
                    },
                );
                c.bus.write32(DATA + 0x1100 + 0x40 * i, 0xc5c5_0001);
            }
            c.bus.write32(0x0051_9ae8, 1);
            c.bus.write32(EA_FILE_TEST_RET_CLOSE, 0);
            c.bus.write32(EA_FILE_TEST_RET_MUTEX_DESTROY, 0);
            c.bus.write32(EA_FILE_TEST_RET_TIMER_REMOVE, 0);
            c.bus.write32(EA_FILE_TEST_RET_CD_CALLBACK, 0);
            schedule(c, &mut r, seed, 0x003d_cde0)
        });
    }
}
