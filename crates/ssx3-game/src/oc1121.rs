//! Staging module oc1121 (wave ealib-1): `EAFile_*` request/queue (20).
//! Moved into the source-tree units by `tools/reorg_modules.py apply oc1121`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003d_cf10,
        ea_file_get_request_result,
        "EAFile_GetRequestResult",
    ),
    (
        0x003d_cf40,
        ea_file_get_request_bytes,
        "EAFile_GetRequestBytes",
    ),
    (
        0x003d_cfc0,
        ea_file_set_request_priority,
        "EAFile_SetRequestPriority",
    ),
    (0x003d_d148, ea_file_cancel_request, "EAFile_CancelRequest"),
    (0x003d_d1d8, ea_file_wait_request, "EAFile_WaitRequest"),
    (0x003d_d438, ea_file_queue_exists, "EAFile_QueueExists"),
    (0x003d_d720, ea_file_queue_write, "EAFile_QueueWrite"),
    (0x003d_d7e0, ea_file_queue_get_size, "EAFile_QueueGetSize"),
    (0x003d_d878, ea_file_queue_nop, "EAFile_QueueNop"),
    (
        0x003d_d8e8,
        ea_file_on_big_header_read,
        "EAFile_OnBigHeaderRead",
    ),
    (0x003d_da10, ea_file_on_big_opened, "EAFile_OnBigOpened"),
    (0x003d_dac0, ea_file_queue_mount_big, "EAFile_QueueMountBig"),
    (
        0x003d_dc00,
        ea_file_on_big_unmounted,
        "EAFile_OnBigUnmounted",
    ),
    (
        0x003d_dc30,
        ea_file_queue_unmount_big,
        "EAFile_QueueUnmountBig",
    ),
    (
        0x003d_ddf0,
        ea_file_call_with_priority,
        "EAFile_CallWithPriority",
    ),
    (0x003d_de50, ea_file_set_priority, "EAFile_SetPriority"),
    (0x003d_de78, ea_file_set_root_path, "EAFile_SetRootPath"),
    (0x003d_df80, ea_file_get_root_path, "EAFile_GetRootPath"),
    (0x003d_e4d0, ea_file_parse_big_path, "EAFile_ParseBigPath"),
    (
        0x003d_e668,
        ea_file_queue_signal_stub,
        "EAFile_QueueSignalStub",
    ),
];

mod callee {
    use super::ee;
    use crate::call::Callee;

    pub(crate) const EA_MUTEX_LOCK: Callee = Callee {
        addr: 0x003e_5700,
        f: ee::s14::EAMutex_Lock,
    };
    pub(crate) const EA_MUTEX_UNLOCK: Callee = Callee {
        addr: 0x003e_5760,
        f: ee::s14::EAMutex_Unlock,
    };
    pub(crate) const EA_THREAD_IS_CURRENT: Callee = Callee {
        addr: 0x003e_5440,
        f: ee::s14::EAThread_IsCurrent,
    };
    pub(crate) const EA_THREAD_YIELD: Callee = Callee {
        addr: 0x003e_5398,
        f: ee::s14::EAThread_Yield,
    };
    pub(crate) const EA_TIMER_SERVICE_CALLBACKS: Callee = Callee {
        addr: 0x003e_5928,
        f: ee::s14::EATimer_ServiceCallbacks,
    };
    pub(crate) const EA_FILE_RELEASE_REQUEST: Callee = Callee {
        addr: 0x003d_d310,
        f: ee::s14::EAFile_ReleaseRequest,
    };
    pub(crate) const EA_FILE_QUEUE_CLOSE: Callee = Callee {
        addr: 0x003d_d5a0,
        f: ee::s14::EAFile_QueueClose,
    };
    pub(crate) const EA_FILE_QUEUE_SIGNAL_STUB: Callee = Callee {
        addr: 0x003d_e668,
        f: ee::s14::EAFile_QueueSignalStub,
    };
    pub(crate) const EA_FILE_ALLOC_REQUEST: Callee = Callee {
        addr: 0x003d_e670,
        f: ee::s14::EAFile_AllocRequest,
    };
    pub(crate) const EA_FILE_ALLOC_FILE_HANDLE: Callee = Callee {
        addr: 0x003d_e800,
        f: ee::s14::EAFile_AllocFileHandle,
    };
    pub(crate) const EA_FILE_SERVICE_QUEUE: Callee = Callee {
        addr: 0x003d_dfa8,
        f: ee::s14::EAFile_ServiceQueue,
    };
    pub(crate) const EA_FILE_QUEUE_READ: Callee = Callee {
        addr: 0x003d_d648,
        f: ee::s14::EAFile_QueueRead,
    };
    pub(crate) const EA_FILE_QUEUE_OPEN: Callee = Callee {
        addr: 0x003d_d4e8,
        f: ee::s14::EAFile_QueueOpen,
    };
    pub(crate) const EA_FILE_SET_REQUEST_CALLBACK: Callee = Callee {
        addr: 0x003d_cf70,
        f: ee::s14::EAFile_SetRequestCallback,
    };
    pub(crate) const BIG_FILE_GET_TYPE: Callee = Callee {
        addr: 0x003e_2500,
        f: ee::s14::BigFile_GetType,
    };
    pub(crate) const BIG_FILE_GET_HEADER_SIZE: Callee = Callee {
        addr: 0x003e_2580,
        f: ee::s14::BigFile_GetHeaderSize,
    };
    pub(crate) const EA_FILE_PLATFORM_OPEN: Callee = Callee {
        addr: 0x003e_3208,
        f: ee::s14::EAFile_PlatformOpen,
    };
    pub(crate) const EA_FILE_BIG_FIND_ENTRY: Callee = Callee {
        addr: 0x003e_2768,
        f: ee::s14::EAFile_BigFindEntry,
    };

    /// `strlen` (newlib), called through the function table.
    pub(crate) const STRLEN: u32 = 0x0041_6810;
    pub(crate) const EA_FILE_PLATFORM_CLOSE: Callee = Callee {
        addr: 0x003e_32f8,
        f: ee::s14::EAFile_PlatformClose,
    };
    pub(crate) const EA_FILE_PLATFORM_GET_SIZE: Callee = Callee {
        addr: 0x003e_34e8,
        f: ee::s14::EAFile_PlatformGetSize,
    };

    /// `strncmp` (newlib), called through the function table.
    pub(crate) const STRNCMP: u32 = 0x0041_6b18;

    /// `strncpy` (newlib), called through the function table.
    pub(crate) const STRNCPY: u32 = 0x0041_6ce0;

    /// `strchr` (newlib), called through the function table.
    pub(crate) const STRCHR: u32 = 0x0041_6404;

    /// `strcpy` (newlib), called through the function table.
    pub(crate) const STRCPY: u32 = 0x0041_66f4;

    /// `strcmp` (newlib), called through the function table.
    pub(crate) const STRCMP: u32 = 0x0041_65a8;

    /// `strcat` (newlib), called through the function table.
    pub(crate) const STRCAT: u32 = 0x0041_62d0;
}

/// `u32 EAFile_GetRequestResult(request)` (0x3dcf10): the result field of
/// `request` (found through the 0x30-byte request table), like
/// [`ea_file_get_request_status`] but loading `+0xc`.
pub fn ea_file_get_request_result(c: &mut Cpu) {
    c.cycles += 11;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[A1][0] = 0x30;
    c.sw(c.g[SP][0] as u32, c.g[A0][0]);
    c.g[V0][0] = 0x520000;
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x6528));
    c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x3));
    c.mult(0, 3, 3, 5);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}

/// `u32 EAFile_GetRequestBytes(request)` (0x3dcf40): the transferred-bytes
/// field of `request` (`+0x14`).
pub fn ea_file_get_request_bytes(c: &mut Cpu) {
    c.cycles += 11;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[A1][0] = 0x30;
    c.sw(c.g[SP][0] as u32, c.g[A0][0]);
    c.g[V0][0] = 0x520000;
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x6528));
    c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x3));
    c.mult(0, 3, 3, 5);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}

/// `void EAFile_SetRequestPriority(request, prio)` (0x3dcfc0): set the
/// priority of `request` and, when the queue is running and the request is
/// idle, unlink it and reinsert it in priority order.
pub fn ea_file_set_request_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x60);
        c.g[A3][0] = 0x30;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[V1][0] = 0x520000;
        frame.save(c, S0, 0x50);
        c.g[A2][0] = c.g[A0][0];
        c.g[V0][0] = c.lbu(c.g[SP][0] as u32);
        c.g[A0][0] = 0;
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.mult(0, 2, 2, 7);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6550));
        c.g[S2][0] = c.g[A1][0];
        c.g[S3][0] = c.g[V1][0];
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A2][0]);
        let null = c.g[A2][0] == 0;
        c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if !null {
            c.cycles += 10;
            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = 0xf0000;
            c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x3));
            c.g[A0][0] = c.g[A0][0] | 0xffff;
            c.mult(0, 2, 3, 7);
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[V0][0] = c.g[V0][0] ^ c.g[A2][0];
            c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
            c.g[A0][0] = (c.g[V0][0] < 0x1u64) as u64;
        }
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            frame.restore(c, S0, 0x50);
        } else {
            c.cycles += 8;
            c.g[RA][0] = sx(0x003d_d048);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
            callee::EA_MUTEX_LOCK.call(c, 0x003d_d048)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S2][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
            let queued = c.g[V0][0] != 0;
            c.g[A0][0] = 0x520000;
            if queued {
                const HEAD: u32 = 0x003d_d098;
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                let empty = c.g[V1][0] == 0;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
                if !empty {
                    c.cycles += 2;
                    let at_head = c.g[V1][0] == c.g[S0][0];
                    if !at_head {
                        c.cycles += 1;
                        c.g[A1][0] = c.g[V1][0];
                        loop {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                            let end = c.g[V1][0] == 0;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x2c), c.g[V1][0]);
                            if end {
                                break;
                            }
                            c.cycles += 2;
                            if c.g[V1][0] == c.g[S0][0] {
                                break;
                            }
                            c.g[A1][0] = c.g[V1][0];
                            if event_due(c) {
                                deliver_event_here(c, HEAD);
                            }
                        }
                    }
                }
                c.cycles += 2;
                let no_prev = c.g[A1][0] == 0;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6550));
                if !no_prev {
                    c.cycles += 2;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x2c), c.g[A2][0]);
                } else {
                    c.cycles += 2;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x30), c.g[A2][0]);
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6550));
                }
                c.cycles += 4;
                c.g[A1][0] = 0;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x30));
                let empty = c.g[V0][0] == 0;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                if !empty {
                    const HEAD2: u32 = 0x003d_d0f0;
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let after = c.g[V0][0] != 0;
                    if !after {
                        c.cycles += 2;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
                        loop {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                            let end = c.g[V0][0] == 0;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                            if end {
                                break;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] != 0 {
                                break;
                            }
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
                            if event_due(c) {
                                deliver_event_here(c, HEAD2);
                            }
                        }
                    }
                }
                c.cycles += 2;
                let no_prev2 = c.g[A1][0] == 0;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6550));
                if !no_prev2 {
                    c.cycles += 2;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x2c), c.g[S0][0]);
                } else {
                    c.cycles += 1;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x30), c.g[S0][0]);
                }
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003d_d128);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x653c));
            callee::EA_MUTEX_UNLOCK.call(c, 0x003d_d128)?;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_CancelRequest(request)` (0x3dd148): when the status bit is
/// set, mark the request slot `+4` as cancelled (1) for a matching kind
/// (3 or 0xa return 0xa); otherwise return the request word shifted down.
pub fn ea_file_cancel_request(c: &mut Cpu) {
    c.cycles += 6;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[A3][0] = c.g[A0][0];
    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A0][0]);
    c.g[V0][0] = 0;
    let null = c.g[A3][0] == 0;
    c.sw(c.g[SP][0] as u32, c.g[A3][0]);
    if !null {
        c.cycles += 12;
        c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x3));
        c.g[V0][0] = 0x30;
        c.g[A2][0] = 0x520000;
        c.g[A0][0] = 0xf0000;
        c.mult(0, 3, 3, 2);
        c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_sub(0x6528));
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.g[V0][0] = c.g[V0][0] ^ c.g[A3][0];
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    }
    c.cycles += 2;
    let clear = c.g[V0][0] == 0;
    c.g[V0][0] = sx((c.g[A3][0] as u32) >> 20);
    if !clear {
        c.cycles += 4;
        c.g[V1][0] = 0x3;
        c.g[A0][0] = c.g[V0][0] & 0xf;
        let is_kind3 = c.g[A0][0] == c.g[V1][0];
        c.g[V0][0] = 0xa;
        if !is_kind3 {
            c.cycles += 2;
            let is_kind_a = c.g[A0][0] == c.g[V0][0];
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
            if !is_kind_a {
                c.cycles += 8;
                c.g[A2][0] = 0x30;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x3));
                c.g[V1][0] = 0x520000;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6528));
                c.g[A1][0] = 0x1;
                c.mult(0, 2, 2, 6);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}

/// `u32 EAFile_WaitRequest(request)` (0x3dd1d8): -3 when `request` is null
/// or its status bit is clear; the ready word when it is already set;
/// otherwise yield (or run the timer callbacks on the file thread) until
/// the word arrives, re-checking the bit each round.
pub fn ea_file_wait_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x80);
        c.g[A3][0] = 0x30;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[V1][0] = 0x520000;
        frame.save(c, S1, 0x60);
        c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6550));
        c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x3));
        c.g[A1][0] = c.g[A0][0];
        frame.save(c, S0, 0x70);
        c.g[A0][0] = 0;
        frame.save(c, S2, 0x50);
        c.mult(0, 2, 2, 7);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x28));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        let null = c.g[A1][0] == 0;
        c.g[S1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if !null {
            c.cycles += 10;
            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = 0xf0000;
            c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x3));
            c.g[A0][0] = c.g[A0][0] | 0xffff;
            c.mult(0, 2, 3, 7);
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[V0][0] = c.g[V0][0] ^ c.g[A1][0];
            c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
            c.g[A0][0] = (c.g[V0][0] < 0x1u64) as u64;
        }
        c.cycles += 2;
        let clear = c.g[A0][0] == 0;
        c.g[V0][0] = 0xfffffffffffffffd;
        if !clear {
            const HEAD: u32 = 0x003d_d278;
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            let ready = c.g[V0][0] != 0;
            c.g[S0][0] = 0xf0000;
            if !ready {
                c.cycles += 5;
                c.g[S5][0] = c.g[T0][0];
                c.g[S0][0] = c.g[S0][0] | 0xffff;
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8));
                c.g[S4][0] = 0x30;
                c.g[S3][0] = 0x1;
                let mut aborted = false;
                loop {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003d_d280);
                    c.g[A0][0] = 0;
                    callee::EA_THREAD_IS_CURRENT.call(c, 0x003d_d280)?;
                    let on_file_thread = c.g[V0][0] == 0;
                    if !on_file_thread {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x003d_d290);
                        c.g[A0][0] = 0;
                        callee::EA_TIMER_SERVICE_CALLBACKS.call(c, 0x003d_d290)?;
                        c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    } else {
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x003d_d2a0);
                        c.g[A0][0] = 0x1;
                        callee::EA_THREAD_YIELD.call(c, 0x003d_d2a0)?;
                        c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    }
                    c.cycles += 3;
                    c.g[A2][0] = 0;
                    let gone = c.g[A1][0] == 0;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    if !gone {
                        c.cycles += 9;
                        c.g[V1][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x3));
                        c.g[A2][0] = c.g[S3][0];
                        c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x28));
                        c.mult(0, 3, 3, 20);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = c.g[V0][0] ^ c.g[A1][0];
                        c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
                        if c.g[V0][0] != 0 {
                            c.g[A2][0] = 0;
                        }
                    }
                    c.cycles += 2;
                    let done = c.g[A2][0] == 0;
                    c.g[V0][0] = 0xfffffffffffffffd;
                    if done {
                        aborted = true;
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    let ready = c.g[V0][0] == 0;
                    if !ready {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, HEAD);
                    }
                }
                if !aborted {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_QueueExists(handle, path, mode)` (0x3dd438): allocate a
/// request for the path and run the queue; the request word or, when the
/// handle is null, a stub signal.
pub fn ea_file_queue_exists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_d460);
        c.g[S3][0] = c.g[A0][0];
        callee::EA_FILE_ALLOC_REQUEST.call(c, 0x003d_d460)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = 0x800000;
        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
        c.g[V0][0] = 0xffffffffff0f0000;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x14), c.g[S0][0]);
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[S1][0]);
        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
        c.g[V0][0] = 0x1;
        c.sw(c.g[S2][0] as u32, c.g[V1][0]);
        c.g[RA][0] = sx(0x003d_d494);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        callee::EA_FILE_ALLOC_FILE_HANDLE.call(c, 0x003d_d494)?;
        let no_handle = c.g[V0][0] != 0;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x24), c.g[V0][0]);
        if !no_handle {
            c.cycles += 4;
            c.g[V0][0] = 0x2;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x003d_d4ac);
            c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            callee::EA_FILE_QUEUE_SIGNAL_STUB.call(c, 0x003d_d4ac)?;
        }
        c.cycles += 15;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = 0x100;
        c.g[RA][0] = sx(0x003d_d4c0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        c.pc = 0x0041_6ce0;
        call_ptr(c, callee::STRNCPY, 0x003d_d4c0)?;
        c.g[RA][0] = sx(0x003d_d4c8);
        c.g[A0][0] = c.g[S2][0];
        callee::EA_FILE_SERVICE_QUEUE.call(c, 0x003d_d4c8)?;
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_QueueWrite(handle, buf, size, off, mode)` (0x3dd720):
/// allocate a write request for the buffer and run the queue; a stub
/// signal when the handle is null.
pub fn ea_file_queue_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S2, 0x50);
        c.g[S0][0] = c.g[T1][0];
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[T0][0];
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A3][0];
        frame.save(c, S6, 0x10);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_d760);
        c.g[S6][0] = c.g[A2][0];
        callee::EA_FILE_ALLOC_REQUEST.call(c, 0x003d_d760)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = 0xffffffffff0f0000;
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.g[A0][0] = 0x500000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[S0][0]);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
        let no_handle = c.g[S2][0] != 0;
        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        if !no_handle {
            c.cycles += 4;
            c.g[V0][0] = 0x6;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x003d_d798);
            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            callee::EA_FILE_QUEUE_SIGNAL_STUB.call(c, 0x003d_d798)?;
        }
        c.cycles += 18;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x24), c.g[S2][0]);
        c.g[A0][0] = c.g[S1][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[S3][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[S4][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[S5][0]);
        c.g[RA][0] = sx(0x003d_d7b4);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x20), c.g[S6][0]);
        callee::EA_FILE_SERVICE_QUEUE.call(c, 0x003d_d7b4)?;
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_QueueGetSize(handle, path)` (0x3dd7e0): allocate a size
/// request for the path and run the queue; a stub signal when the queue
/// entry is missing.
pub fn ea_file_queue_get_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_d808);
        c.g[S3][0] = c.g[A0][0];
        callee::EA_FILE_ALLOC_REQUEST.call(c, 0x003d_d808)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V1][0] = 0xffffffffff0f0000;
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.g[A0][0] = 0x600000;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x14), c.g[S0][0]);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[S1][0]);
        c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
        let no_entry = c.g[S3][0] == 0;
        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
        if !no_entry {
            c.cycles += 5;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = 0x1;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[V1][0]);
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x6;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x003d_d858);
            c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            callee::EA_FILE_QUEUE_SIGNAL_STUB.call(c, 0x003d_d858)?;
        }
        c.cycles += 8;
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_QueueNop(a0, a1, a2)` (0x3dd878): allocate a no-op request
/// and run the queue.
pub fn ea_file_queue_nop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_d898);
        c.g[S2][0] = c.g[A1][0];
        callee::EA_FILE_ALLOC_REQUEST.call(c, 0x003d_d898)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x700000;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = 0xffffffffff0f0000;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S2][0]);
        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x003d_d8c8);
        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
        callee::EA_FILE_SERVICE_QUEUE.call(c, 0x003d_d8c8)?;
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_OnBigHeaderRead(request, handle)` (0x3dd8e8): release the
/// header request; when the archive type is still unknown, free the buffer
/// through the allocator callback. Then, when the buffered bytes cover the
/// header, record the request and run the queue, else read the full header
/// into place first.
pub fn ea_file_on_big_header_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x60);
        c.g[A3][0] = 0x30;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[A1][0] = 0x520000;
        frame.save(c, S0, 0x50);
        c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x3));
        c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6550));
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.mult(0, 3, 3, 7);
        frame.save(c, S3, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
        c.g[S3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[RA][0] = sx(0x003d_d93c);
        c.sw((c.g[A1][0] as u32).wrapping_add(0x24), c.g[V1][0]);
        callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_d93c)?;
        c.g[RA][0] = sx(0x003d_d944);
        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
        callee::BIG_FILE_GET_TYPE.call(c, 0x003d_d944)?;
        let known = c.g[V0][0] != 0;
        c.g[V1][0] = 0x450000;
        if !known {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc18));
            let free = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003d_d958);
            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
            c.pc = free;
            call_ptr(c, free, 0x003d_d958)?;
        }
        c.cycles += 7;
        c.g[RA][0] = sx(0x003d_d960);
        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
        callee::BIG_FILE_GET_HEADER_SIZE.call(c, 0x003d_d960)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
        let need_header = c.g[V0][0] != 0;
        if !need_header {
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
            c.cycles += 5;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x34), c.g[S1][0]);
            c.g[RA][0] = sx(0x003d_d9f0);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            callee::EA_FILE_SERVICE_QUEUE.call(c, 0x003d_d9f0)?;
        } else {
            c.cycles += 27;
            c.g[S0][0] = 0x450000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc10));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            let free = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003d_d988);
            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
            c.pc = free;
            call_ptr(c, free, 0x003d_d988)?;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[S2][0]);
            c.g[A0][0] = 0x490000;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d68));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            let alloc = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003d_d9a4);
            c.g[A1][0] = c.g[S2][0];
            c.pc = alloc;
            call_ptr(c, alloc, 0x003d_d9a4)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = 0;
            c.g[A3][0] = c.g[S2][0];
            c.g[T0][0] = c.g[S3][0];
            c.g[T1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003d_d9c4);
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            callee::EA_FILE_QUEUE_READ.call(c, 0x003d_d9c4)?;
            c.g[A1][0] = 0x3e0000;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2718));
            c.g[RA][0] = sx(0x003d_d9d8);
            c.sw(c.g[SP][0] as u32, c.g[V0][0]);
            callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_d9d8)?;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_OnBigOpened(request, handle)` (0x3dda10): release the open
/// request; when it was the header fetch, queue the header read, else mark
/// the archive ready and run the queue.
pub fn ea_file_on_big_opened(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x50);
        c.g[T0][0] = 0x30;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[A3][0] = 0x520000;
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_sub(0x6528));
        c.g[V0][0] = c.lbu(c.g[SP][0] as u32);
        frame.save(c, S0, 0x40);
        c.mult(0, 2, 2, 8);
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[RA][0] = sx(0x003d_da50);
        c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
        callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_da50)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 0x1;
        let is_header = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = 0x4;
        if !is_header {
            c.cycles += 6;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            c.g[RA][0] = sx(0x003d_da70);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            callee::EA_FILE_SERVICE_QUEUE.call(c, 0x003d_da70)?;
        } else {
            c.cycles += 12;
            c.g[A2][0] = c.lw(c.g[S1][0] as u32);
            c.g[A1][0] = 0;
            c.g[T0][0] = c.g[S2][0];
            c.g[T1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x003d_da90);
            c.g[A3][0] = 0x800;
            callee::EA_FILE_QUEUE_READ.call(c, 0x003d_da90)?;
            c.g[A1][0] = 0x3e0000;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2718));
            c.g[RA][0] = sx(0x003d_daa4);
            c.sw(c.g[SP][0] as u32, c.g[V0][0]);
            callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_daa4)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_QueueMountBig(handle, path, mode)` (0x3ddac0): unless the
/// archive is already mounted (name match), allocate a buffer, open the
/// path through the queue and set the mount callback.
pub fn ea_file_queue_mount_big(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003d_db30;
        c.cycles += 26;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0x520000;
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S2, 0x40);
        c.g[S5][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_dafc);
        c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x651c));
        callee::EA_FILE_ALLOC_REQUEST.call(c, 0x003d_dafc)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V1][0] = 0xffffffffff0f0000;
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.g[A0][0] = 0x900000;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x14), c.g[S0][0]);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[S5][0]);
        c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
        let none_mounted = c.g[S1][0] == 0;
        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
        let mut matched = false;
        if !none_mounted {
            c.cycles += 2;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
            loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S3][0];
                c.g[A2][0] = 0x100;
                c.g[RA][0] = sx(0x003d_db40);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                c.pc = 0x0041_6b18;
                call_ptr(c, callee::STRNCMP, 0x003d_db40)?;
                if c.g[V0][0] != 0 {
                    c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x003d_db58);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                    callee::EA_FILE_SERVICE_QUEUE.call(c, 0x003d_db58)?;
                    c.g[V0][0] = c.g[S2][0];
                    matched = true;
                    break;
                }
                c.cycles += 2;
                if c.g[S1][0] == 0 {
                    break;
                }
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        if !matched {
            c.cycles += 29;
            c.g[S0][0] = 0x450000;
            c.g[A0][0] = c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc10));
            c.g[A1][0] = 0x18;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            let alloc = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003d_db84);
            c.g[A2][0] = c.g[S4][0];
            c.pc = alloc;
            call_ptr(c, alloc, 0x003d_db84)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = 0x490000;
            c.g[V0][0] = 0x800;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[S2][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d68));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[S4][0]);
            c.g[A2][0] = c.g[S4][0];
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            let alloc2 = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003d_dbb0);
            c.g[A1][0] = 0x800;
            c.pc = alloc2;
            call_ptr(c, alloc2, 0x003d_dbb0)?;
            c.g[A1][0] = 0x1;
            c.g[A3][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S5][0];
            c.g[RA][0] = sx(0x003d_dbc8);
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            callee::EA_FILE_QUEUE_OPEN.call(c, 0x003d_dbc8)?;
            c.g[A1][0] = 0x3e0000;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003d_dbd8);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x25f0));
            callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_dbd8)?;
            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_OnBigUnmounted(request, handle)` (0x3ddc00): release the
/// unmount request, then run the queue on its handle.
pub fn ea_file_on_big_unmounted(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_dc14);
        c.g[S0][0] = c.g[A2][0];
        callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_dc14)?;
        c.g[RA][0] = sx(0x003d_dc1c);
        c.g[A0][0] = c.g[S0][0];
        callee::EA_FILE_SERVICE_QUEUE.call(c, 0x003d_dc1c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_QueueUnmountBig(handle, path)` (0x3ddc30): allocate an
/// unmount request, unlink the matching mounted archive, free its slots
/// and run the queue; returns the request word.
///
/// Dispatcher-shaped in the original (loop heads `0x3ddcf0`, `0x3ddcf4`
/// and `0x3ddd40` are resume entries): the port keeps the dispatch and
/// its state in the registers.
pub fn ea_file_queue_unmount_big(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003d_dc30 => {
                    c.cycles += 32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S0][0] = 0x520000;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S1][0] = c.g[A2][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S4][0] = c.g[A0][0];
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6550));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S6][0] = 0;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S7][0] = c.g[A1][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[RA][0] = sx(0x003d_dc78);
                    c.g[S2][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x34));
                    callee::EA_FILE_ALLOC_REQUEST.call(c, 0x003d_dc78)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[A0][0] = 0xa00000;
                    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = 0xffffffffff0f0000;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x2c));
                    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x14), c.g[S1][0]);
                    c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x10), c.g[S7][0]);
                    c.sw(c.g[S3][0] as u32, c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                    let no_big = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x7;
                    pc = if no_big { 0x003d_dcb4 } else { 0x003d_dcb0 };
                }
                0x003d_dcb0 => {
                    c.cycles += 1;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    pc = 0x003d_dcb4;
                }
                0x003d_dcb4 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = 0;
                    c.g[A2][0] = 0x450000;
                    let empty = (c.g[V0][0] as i64) <= 0;
                    c.g[S1][0] = 0x3e0000;
                    pc = if empty { 0x003d_dd44 } else { 0x003d_dcc8 };
                }
                0x003d_dcc8 => {
                    c.cycles += 2;
                    let no_handle = c.g[A1][0] == 0;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6550));
                    pc = if no_handle { 0x003d_dcf0 } else { 0x003d_dcd0 };
                }
                0x003d_dcd0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        pc = 0x003d_dcf4;
                    } else {
                        pc = 0x003d_dcdc;
                    }
                }
                0x003d_dcdc => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    let found = c.g[V0][0] == c.g[S4][0];
                    c.g[V0][0] = 0xfffffffffffffffe;
                    pc = if found { 0x003d_dd2c } else { 0x003d_dcec };
                }
                0x003d_dcec => {
                    c.cycles += 1;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6550));
                    pc = 0x003d_dcf0;
                }
                0x003d_dcf0 => {
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    pc = 0x003d_dcf4;
                }
                0x003d_dcf4 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x110));
                    pc = if done { 0x003d_dd44 } else { 0x003d_dd04 };
                }
                0x003d_dd04 => {
                    c.cycles += 2;
                    let empty = c.g[A1][0] == 0;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6550));
                    if empty {
                        if event_due(c) && back_edge_unwinds(c, 0x003d_dcf0) {
                            return Ok(());
                        }
                        pc = 0x003d_dcf0;
                    } else {
                        pc = 0x003d_dd0c;
                    }
                }
                0x003d_dd0c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        if event_due(c) && back_edge_unwinds(c, 0x003d_dcf4) {
                            return Ok(());
                        }
                        pc = 0x003d_dcf4;
                    } else {
                        pc = 0x003d_dd18;
                    }
                }
                0x003d_dd18 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let found = c.g[V1][0] != c.g[S4][0];
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6550));
                    if found {
                        if event_due(c) && back_edge_unwinds(c, 0x003d_dcf0) {
                            return Ok(());
                        }
                        pc = 0x003d_dcf0;
                    } else {
                        pc = 0x003d_dd28;
                    }
                }
                0x003d_dd28 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0xfffffffffffffffe;
                    pc = 0x003d_dd2c;
                }
                0x003d_dd2c => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x8;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    pc = 0x003d_dd44;
                }
                0x003d_dd3c => {
                    c.cycles += 1;
                    pc = 0x003d_dd40;
                }
                0x003d_dd40 => {
                    c.cycles += 1;
                    c.g[S2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                    pc = 0x003d_dd44;
                }
                0x003d_dd44 => {
                    c.cycles += 2;
                    let done = c.g[S2][0] == 0;
                    c.g[V0][0] = 0x5;
                    pc = if done { 0x003d_dd60 } else { 0x003d_dd4c };
                }
                0x003d_dd4c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    if c.g[V0][0] != c.g[S4][0] {
                        c.g[S6][0] = c.g[S2][0];
                        if event_due(c) && back_edge_unwinds(c, 0x003d_dd40) {
                            return Ok(());
                        }
                        pc = 0x003d_dd40;
                    } else {
                        pc = 0x003d_dd58;
                    }
                }
                0x003d_dd58 => {
                    c.cycles += 2;
                    let found = c.g[S2][0] != 0;
                    c.g[V0][0] = 0x5;
                    pc = if found { 0x003d_dd64 } else { 0x003d_dd60 };
                }
                0x003d_dd60 => {
                    c.cycles += 1;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    pc = 0x003d_dd64;
                }
                0x003d_dd64 => {
                    c.cycles += 2;
                    let linked = c.g[S6][0] == 0;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6550));
                    pc = if linked { 0x003d_dd78 } else { 0x003d_dd6c };
                }
                0x003d_dd6c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[S6][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    pc = 0x003d_dd84;
                }
                0x003d_dd78 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x34), c.g[A0][0]);
                    pc = 0x003d_dd84;
                }
                0x003d_dd84 => {
                    c.cycles += 26;
                    c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc10));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    let free = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x003d_dd94);
                    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                    c.pc = free;
                    call_ptr(c, free, 0x003d_dd94)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    let free2 = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x003d_dda0);
                    c.g[A0][0] = c.g[S2][0];
                    c.pc = free2;
                    call_ptr(c, free2, 0x003d_dda0)?;
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x003d_ddb0);
                    c.g[A1][0] = c.g[S7][0];
                    callee::EA_FILE_QUEUE_CLOSE.call(c, 0x003d_ddb0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003d_ddbc);
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2400));
                    callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_ddbc)?;
                    c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003d_dc30, 0x003d_ddec);
                    return Ok(());
                }
            }
        }
    })
}

/// `char *EAFile_GetRootPath()` (0x3ddf80): copy the current root into a
/// scratch buffer; returns the buffer.
pub fn ea_file_get_root_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x520000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6510));
        c.g[RA][0] = sx(0x003d_df98);
        c.g[A2][0] = 0x100;
        call_ptr(c, callee::STRNCPY, 0x003d_df98)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_ParseBigPath(path, a1, a2, out_size)` (0x3de4d0): split
/// `path` at `|`; with no pipe, open the plain file for its size, else
/// find the archive entry in a mounted big. Returns 1 with the size stored
/// (or 0).
pub fn ea_file_parse_big_path(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003d_e5e8;
        c.cycles += 18;
        let frame = Frame::open(c, 0x280);
        frame.save(c, S4, 0x230);
        frame.save(c, S1, 0x260);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S2, 0x250);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S5, 0x220);
        c.g[S2][0] = 0;
        frame.save(c, S0, 0x270);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S3, 0x240);
        frame.save_ra(c, 0x210);
        c.g[A1][0] = 0x7c;
        c.sw(c.g[S4][0] as u32, 0);
        c.g[RA][0] = sx(0x003d_e510);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x100), 0);
        call_ptr(c, callee::STRCHR, 0x003d_e510)?;
        let no_pipe = c.g[V0][0] == 0;
        c.g[V0][0] = 0x7c;
        'b_3de598: {
            if !no_pipe {
                'b_3de554: {
                    'b_3de52c: {
                        c.cycles += 3;
                        c.g[V1][0] = c.lb(c.g[S1][0] as u32);
                        let not_first = c.g[V1][0] != c.g[V0][0];
                        c.g[A0][0] = c.g[S1][0];
                        if not_first {
                            break 'b_3de52c;
                        }
                        c.cycles += 2;
                        c.g[S3][0] = 0x2;
                        break 'b_3de554;
                    }
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x003d_e534);
                    c.g[A1][0] = 0x7c;
                    call_ptr(c, callee::STRCHR, 0x003d_e534)?;
                    c.g[S3][0] = 0x4;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x003d_e54c);
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, callee::STRNCPY, 0x003d_e54c)?;
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.sb(c.g[V1][0] as u32, 0);
                    break 'b_3de554;
                }
                c.cycles += 9;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x003d_e564);
                c.g[A1][0] = 0x7c;
                call_ptr(c, callee::STRCHR, 0x003d_e564)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003d_e570);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                call_ptr(c, callee::STRCPY, 0x003d_e570)?;
                c.g[V0][0] = c.g[S3][0] & 0x1;
                // (Generated: unconditional break to 3de598; the else is dead.)
                break 'b_3de598;
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[S5][0] & 0x1;
            let direct = c.g[V0][0] == 0;
            c.g[S3][0] = 0x1;
            if !direct {
                c.cycles += 4;
                c.g[S3][0] = 0x3;
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[RA][0] = sx(0x003d_e594);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, callee::STRCPY, 0x003d_e594)?;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0] & 0x1;
            break 'b_3de598;
        }
        'b_3de5c8: {
            c.cycles += 2;
            let done = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            if done {
                break 'b_3de5c8;
            }
            c.cycles += 5;
            c.g[A1][0] = c.g[S5][0];
            c.g[RA][0] = sx(0x003d_e5ac);
            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
            callee::EA_FILE_PLATFORM_OPEN.call(c, 0x003d_e5ac)?;
            let failed = c.g[V0][0] == 0;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x200));
            if failed {
                break 'b_3de5c8;
            }
            c.cycles += 5;
            c.g[RA][0] = sx(0x003d_e5bc);
            c.g[S2][0] = 0x1;
            callee::EA_FILE_PLATFORM_GET_SIZE.call(c, 0x003d_e5bc)?;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x200));
            c.g[RA][0] = sx(0x003d_e5c8);
            c.sw(c.g[S4][0] as u32, c.g[V0][0]);
            callee::EA_FILE_PLATFORM_CLOSE.call(c, 0x003d_e5c8)?;
            break 'b_3de5c8;
        }
        'b_3de644: {
            c.cycles += 2;
            let have = c.g[S2][0] != 0;
            c.g[V0][0] = c.g[S2][0];
            if have {
                break 'b_3de644;
            }
            'b_3de640: {
                c.cycles += 3;
                c.g[V0][0] = c.g[S3][0] & 0x6;
                let plain = c.g[V0][0] == 0;
                c.g[V0][0] = 0x520000;
                if plain {
                    break 'b_3de640;
                }
                c.cycles += 3;
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x651c));
                let none = c.g[S0][0] == 0;
                c.g[S1][0] = c.g[S3][0] & 0x4;
                if none {
                    break 'b_3de640;
                }
                loop {
                    c.cycles += 2;
                    let skip_name = c.g[S1][0] == 0;
                    c.g[A1][0] = c.g[SP][0];
                    let mut do_find = skip_name;
                    // Mismatch advances `S0` and skips the shared advance
                    // below (generated `break 'b_3de630`).
                    let mut mismatched = false;
                    if !skip_name {
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[RA][0] = sx(0x003d_e5fc);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                        c.pc = 0x0041_65a8;
                        call_ptr(c, callee::STRCMP, 0x003d_e5fc)?;
                        if c.g[V0][0] != 0 {
                            c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            mismatched = true;
                        } else {
                            do_find = true;
                        }
                    }
                    if !mismatched {
                        if do_find {
                            c.cycles += 8;
                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                            c.g[A2][0] = 0;
                            c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x204));
                            c.g[RA][0] = sx(0x003d_e61c);
                            c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x208));
                            callee::EA_FILE_BIG_FIND_ENTRY.call(c, 0x003d_e61c)?;
                            let missing = c.g[V0][0] == 0;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x208));
                            if !missing {
                                c.cycles += 2;
                                c.g[S2][0] = 0x1;
                                c.sw(c.g[S4][0] as u32, c.g[V0][0]);
                            }
                        }
                        c.cycles += 1;
                        c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    }
                    c.cycles += 2;
                    let end = c.g[S0][0] == 0;
                    c.g[V0][0] = c.g[S2][0];
                    if end {
                        break 'b_3de644;
                    }
                    c.cycles += 2;
                    let found = c.g[S2][0] != 0;
                    if !found {
                        if event_due(c) {
                            deliver_event_here(c, HEAD);
                        }
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S2][0];
                        break;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0];
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x270);
        frame.restore(c, S1, 0x260);
        frame.restore(c, S2, 0x250);
        frame.restore(c, S3, 0x240);
        frame.restore(c, S4, 0x230);
        frame.restore(c, S5, 0x220);
        frame.restore_ra(c, 0x210);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_QueueSignalStub()` (0x3de668): the 2-cycle queue-signal
/// placeholder.
pub fn ea_file_queue_signal_stub(c: &mut Cpu) {
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    /// Write `s` as bytes at `addr`, NUL-terminated.
    fn put_ascii(c: &mut Cpu, addr: u32, s: &str) {
        for (i, &b) in s.as_bytes().iter().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    /// The request hash table: every top byte selects `DATA + 0x1000`.
    fn table_setup(c: &mut Cpu) {
        c.bus.write32(0x0051_9ad8, DATA + 0x1000);
    }

    /// Queue-layer block (`0x519ab0`): count, head and the mutex.
    fn queue_setup(c: &mut Cpu, r: &mut Rng, count: u32, head: u32) {
        c.bus.write32(0x0051_9ab0 + 0x28, count);
        c.bus.write32(0x0051_9ab0 + 0x30, head);
        c.bus.write32(0x0051_9ab0 + 0xc, r.next_u32() & 0xff);
        c.bus.write32(0x0051_9ab0 + 0x24, 0);
        c.bus.write32(0x0051_9ab0 + 0x1c, r.next_u32() & 3);
        c.bus
            .write32(0x0051_9ac4 + 0x4, 0x7700_0000 | r.next_u32() & 0xffff);
        c.bus.write32(0x0051_9ac4 + 0x8, r.next_u32() & 3);
        c.bus.write32(0x0051_9ac4 + 0xc, r.next_u32() & 0xff);
        c.bus.write32(0x0051_9ad8, DATA + 0x1000);
    }

    /// Gated fakes for the queue tests: the allocator and the service
    /// queue answer from fixtures; anywhere else they fall back to the
    /// installed bodies.
    pub(crate) const EAF_CTRL: u32 = DATA + 0x2_0000;
    pub(crate) const EAF_MAGIC: u32 = 0xeaf0_0001;
    pub(crate) const EAF_REQ: u32 = DATA + 0x2000;
    pub(crate) const EAF_HANDLE: u32 = DATA + 0x3000;

    pub(crate) fn ea_fake_alloc_request(c: &mut Cpu) {
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_alloc_request),
                ee::s14::f_003de670__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0xeaf0_1000, ra);
        c.g[V0][0] = EAF_REQ as u64;
    }

    pub(crate) fn ea_fake_alloc_file_handle(c: &mut Cpu) {
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_alloc_file_handle),
                ee::s14::f_003de800__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0xeaf0_1001, ra);
        c.g[V0][0] = EAF_HANDLE as u64;
    }

    pub(crate) fn ea_fake_service_queue(c: &mut Cpu) {
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_service_queue),
                ee::s14::f_003ddfa8__generated,
            );
        }
        let req = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0xeaf0_1002, ra);
        c.g[V0][0] = req;
    }

    pub(crate) fn ea_fake_yield(c: &mut Cpu) {
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_yield),
                ee::s14::f_003e5398__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0xeaf0_1003, ra);
        c.bus.write32(DATA + 0x1008, 1);
        c.g[V0][0] = 0;
    }

    /// Fixture-word returns for the small query callees.
    pub(crate) const EAF_RET_TYPE: u32 = DATA + 0x2_0010;
    pub(crate) const EAF_RET_HDRSIZE: u32 = DATA + 0x2_0014;
    pub(crate) const EAF_RET_FIND: u32 = DATA + 0x2_0018;
    pub(crate) const EAF_RET_POPEN: u32 = DATA + 0x2_001c;
    pub(crate) const EAF_RET_PSIZE: u32 = DATA + 0x2_0020;

    macro_rules! ea_fake_ret {
        ($name:ident, $slot:expr, $gen:expr, $cell:expr, $id:expr) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $id, ra);
                c.g[V0][0] = c.bus.read32($cell) as u64;
            }
        };
    }

    ea_fake_ret!(
        ea_fake_release_request,
        0,
        ee::s14::f_003dd310__generated,
        EAF_RET_TYPE,
        0xeaf0_1004
    );
    ea_fake_ret!(
        ea_fake_queue_open,
        0,
        ee::s14::f_003dd4e8__generated,
        EAF_RET_TYPE,
        0xeaf0_1005
    );
    ea_fake_ret!(
        ea_fake_queue_close,
        0,
        ee::s14::f_003dd5a0__generated,
        EAF_RET_TYPE,
        0xeaf0_1006
    );
    ea_fake_ret!(
        ea_fake_queue_read,
        0,
        ee::s14::f_003dd648__generated,
        EAF_RET_TYPE,
        0xeaf0_1007
    );
    ea_fake_ret!(
        ea_fake_big_get_type,
        0,
        ee::s14::f_003e2500__generated,
        EAF_RET_TYPE,
        0xeaf0_1008
    );
    ea_fake_ret!(
        ea_fake_big_get_header_size,
        0,
        ee::s14::f_003e2580__generated,
        EAF_RET_HDRSIZE,
        0xeaf0_1009
    );
    ea_fake_ret!(
        ea_fake_big_find_entry,
        0,
        ee::s14::f_003e2768__generated,
        EAF_RET_FIND,
        0xeaf0_100a
    );
    ea_fake_ret!(
        ea_fake_platform_open,
        0,
        ee::s14::f_003e3208__generated,
        EAF_RET_POPEN,
        0xeaf0_100b
    );
    ea_fake_ret!(
        ea_fake_platform_get_size,
        0,
        ee::s14::f_003e34e8__generated,
        EAF_RET_PSIZE,
        0xeaf0_100c
    );
    ea_fake_ret!(
        ea_fake_platform_close,
        0,
        ee::s14::f_003e32f8__generated,
        EAF_RET_TYPE,
        0xeaf0_100d
    );

    pub(crate) fn ea_install_queue_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DE670),
            ea_fake_alloc_request,
            fakes::prev_cell(ea_fake_alloc_request),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DE800),
            ea_fake_alloc_file_handle,
            fakes::prev_cell(ea_fake_alloc_file_handle),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DDFA8),
            ea_fake_service_queue,
            fakes::prev_cell(ea_fake_service_queue),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5398),
            ea_fake_yield,
            fakes::prev_cell(ea_fake_yield),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD310),
            ea_fake_release_request,
            fakes::prev_cell(ea_fake_release_request),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD4E8),
            ea_fake_queue_open,
            fakes::prev_cell(ea_fake_queue_open),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD5A0),
            ea_fake_queue_close,
            fakes::prev_cell(ea_fake_queue_close),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD648),
            ea_fake_queue_read,
            fakes::prev_cell(ea_fake_queue_read),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E2500),
            ea_fake_big_get_type,
            fakes::prev_cell(ea_fake_big_get_type),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E2580),
            ea_fake_big_get_header_size,
            fakes::prev_cell(ea_fake_big_get_header_size),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E2768),
            ea_fake_big_find_entry,
            fakes::prev_cell(ea_fake_big_find_entry),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E3208),
            ea_fake_platform_open,
            fakes::prev_cell(ea_fake_platform_open),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E34E8),
            ea_fake_platform_get_size,
            fakes::prev_cell(ea_fake_platform_get_size),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E32F8),
            ea_fake_platform_close,
            fakes::prev_cell(ea_fake_platform_close),
        );
    }

    /// A zeroed request block plus the table entry selecting it.
    fn req_fixture(c: &mut Cpu) {
        for i in 0..12u32 {
            c.bus.write32(EAF_REQ + 4 * i, 0);
        }
        c.bus.write32(EAF_CTRL, EAF_MAGIC);
        table_setup(c);
    }

    /// Result/bytes words with a matching status bit, or zeros.
    #[test]
    fn ea_file_get_request_result_matches() {
        check(
            ee::s14::f_003dcf10__generated,
            ea_file_get_request_result,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_cf10, 0);
                table_setup(c);
                let i = seed % 4;
                let entry = DATA + 0x1000 + 0x30 * i;
                c.bus.write32(entry + 8, 0x34_0000 + (i << 24));
                c.bus.write32(entry + 0xc, 0x1000 + seed);
                c.g[A0][0] = 0x34_0000 + ((i << 24) as u64);
                c.g[A1][0] = seed as u64;
                c.pc = 0x003d_cf10;
            },
        );
    }

    #[test]
    fn ea_file_get_request_bytes_matches() {
        check(
            ee::s14::f_003dcf40__generated,
            ea_file_get_request_bytes,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_cf40, 0);
                table_setup(c);
                let i = seed % 4;
                let entry = DATA + 0x1000 + 0x30 * i;
                c.bus.write32(entry + 8, 0x34_0000 + (i << 24));
                c.bus.write32(entry + 0x14, 0x2000 + seed);
                c.g[A0][0] = 0x34_0000 + ((i << 24) as u64);
                c.g[A1][0] = seed as u64;
                c.pc = 0x003d_cf40;
            },
        );
    }

    driven!(
        gen_ea_file_set_request_priority,
        ee::s14::f_003dcfc0__generated
    );
    driven!(
        hand_ea_file_set_request_priority,
        ea_file_set_request_priority
    );

    /// Null, status-clear, and set requests; the set ones with short and
    /// long queues, matching/missing heads, busy and idle entries.
    #[test]
    fn ea_file_set_request_priority_matches() {
        check(
            gen_ea_file_set_request_priority,
            hand_ea_file_set_request_priority,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_cfc0, 0);
                let mode = seed % 6;
                let entry = DATA + 0x1000;
                for i in 0..12u32 {
                    c.bus.write32(entry + 4 * i, 0);
                }
                if mode == 0 {
                    c.g[A0][0] = 0;
                    c.g[A1][0] = seed as u64;
                    queue_setup(c, &mut r, 0, 0);
                    c.pc = 0x003d_cfc0;
                    return;
                }
                // Status word decides clear vs set.
                let status = if mode == 1 { 0 } else { 0x34_0000 };
                c.bus.write32(entry + 8, status);
                // Queue: count 0..3, head present or not.
                let count = seed / 6 % 4;
                let node = DATA + 0x2000;
                let node2 = DATA + 0x2040;
                for i in 0..12u32 {
                    c.bus.write32(node + 4 * i, 0);
                    c.bus.write32(node2 + 4 * i, 0);
                }
                let head = if count >= 2 && seed / 24 % 2 == 0 {
                    node
                } else {
                    0
                };
                queue_setup(c, &mut r, count, head);
                // The request: priority seed%3, busy flag, next link.
                c.bus.write32(entry + 0x10, seed % 3);
                c.bus.write32(entry + 0x8 + 0, status);
                c.bus
                    .write32(entry + 0x2c, if seed / 48 % 2 == 0 { node } else { 0 });
                c.bus.write32(node + 0x10, (seed / 96 % 3) as u32);
                c.bus.write32(node + 0x2c, node2);
                c.bus.write32(node + 0x8, (seed / 288 % 2) as u32);
                c.bus.write32(node2 + 0x10, (seed / 576 % 3) as u32);
                c.bus.write32(node2 + 0x2c, 0);
                c.g[A0][0] = 0x34_0000;
                c.g[A1][0] = seed as u64;
                c.pc = 0x003d_cfc0;
            },
        );
    }

    /// Null (0), status-clear (the shifted handle), and set requests of
    /// kinds 3, 0xa and other (slot store).
    #[test]
    fn ea_file_cancel_request_matches() {
        check(
            ee::s14::f_003dd148__generated,
            ea_file_cancel_request,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_d148, 0);
                table_setup(c);
                let mode = seed % 5;
                if mode == 0 {
                    c.g[A0][0] = 0;
                    c.g[A1][0] = seed as u64;
                    c.pc = 0x003d_d148;
                    return;
                }
                let entry = DATA + 0x1000;
                for i in 0..12u32 {
                    c.bus.write32(entry + 4 * i, 0);
                }
                // Low nibble of byte 2 is the kind; byte 3 selects entry 0.
                // Mode 1 clears the bit with a matching word.
                let kind = [0u32, 3, 3, 0xa, 7][mode as usize];
                let handle = (kind << 16) as u64;
                // Clear iff (word ^ handle) & 0xfffff == 0.
                let word = if mode == 1 {
                    handle as u32 & 0xf_ffff
                } else {
                    0
                };
                c.bus.write32(entry + 8, word);
                c.g[A0][0] = handle;
                c.g[A1][0] = seed as u64;
                c.pc = 0x003d_d148;
            },
        );
    }

    driven!(gen_ea_file_wait_request, ee::s14::f_003dd1d8__generated);
    driven!(hand_ea_file_wait_request, ea_file_wait_request);

    /// Null, status-clear, already-ready, and spinning (the yield fake
    /// delivers the word) requests.
    #[test]
    fn ea_file_wait_request_events() {
        check_events(
            gen_ea_file_wait_request,
            hand_ea_file_wait_request,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_d1d8, 0);
                ea_install_queue_fakes();
                table_setup(c);
                let mode = seed % 4;
                let entry = DATA + 0x1000;
                for i in 0..12u32 {
                    c.bus.write32(entry + 4 * i, 0);
                }
                c.bus.write32(0x0051_9ab0 + 0x28, DATA + 0x1000);
                if mode == 0 {
                    c.g[A0][0] = 0;
                } else {
                    let handle = 0x33_0000u64;
                    // Status bit from entry+0; the wait word is entry+8.
                    // Bit set iff the word differs in the low 20 bits.
                    let word = if mode == 1 {
                        handle as u32 & 0xf_ffff
                    } else {
                        0
                    };
                    c.bus.write32(entry, word);
                    c.bus.write32(entry + 8, if mode == 2 { 0x42 } else { 0 });
                    c.bus.write32(EAF_CTRL, EAF_MAGIC);
                    c.g[A0][0] = handle;
                }
                c.g[A1][0] = 0;
                schedule(c, &mut r, seed, 0x003d_d1d8)
            },
        );
    }

    driven!(gen_ea_file_queue_exists, ee::s14::f_003dd438__generated);
    driven!(hand_ea_file_queue_exists, ea_file_queue_exists);

    /// Null and live handles; short and 100-char paths.
    #[test]
    fn ea_file_queue_exists_events() {
        check_events(
            gen_ea_file_queue_exists,
            hand_ea_file_queue_exists,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_d438, 0);
                ea_install_queue_fakes();
                req_fixture(c);
                for i in 0..12u32 {
                    c.bus.write32(EAF_HANDLE + 4 * i, 0);
                }
                let paths = [
                    "f.dat",
                    "",
                    "a/very/long/path/that/goes/on/for/a/while/file.dat",
                ];
                let path = paths[seed as usize % paths.len()];
                put_ascii(c, DATA + 0x400, path);
                c.g[A0][0] = if seed / 3 % 2 == 0 {
                    0
                } else {
                    EAF_HANDLE as u64
                };
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = seed as u64;
                schedule(c, &mut r, seed, 0x003d_d438)
            },
        );
    }

    driven!(gen_ea_file_queue_write, ee::s14::f_003dd720__generated);
    driven!(hand_ea_file_queue_write, ea_file_queue_write);

    /// Null and live handles; empty and 64-byte buffers.
    #[test]
    fn ea_file_queue_write_matches() {
        check(
            gen_ea_file_queue_write,
            hand_ea_file_queue_write,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_d720, 0);
                ea_install_queue_fakes();
                req_fixture(c);
                let len = [0, 1, 64][seed as usize % 3];
                for i in 0..len {
                    c.bus.write8(DATA + 0x400 + i as u32, b'a' + i as u8 % 26);
                }
                c.g[A0][0] = if seed / 3 % 2 == 0 {
                    0
                } else {
                    EAF_HANDLE as u64
                };
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = len as u64;
                c.g[A3][0] = seed as u64;
                c.g[T0][0] = (seed / 6) as u64;
                c.g[T1][0] = (seed / 48) as u64;
                c.pc = 0x003d_d720;
            },
        );
    }

    driven!(gen_ea_file_queue_get_size, ee::s14::f_003dd7e0__generated);
    driven!(hand_ea_file_queue_get_size, ea_file_queue_get_size);

    /// Missing and present entries.
    #[test]
    fn ea_file_queue_get_size_matches() {
        check(
            gen_ea_file_queue_get_size,
            hand_ea_file_queue_get_size,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_d7e0, 0);
                ea_install_queue_fakes();
                req_fixture(c);
                if seed % 2 == 0 {
                    c.g[A0][0] = 0;
                } else {
                    c.g[A0][0] = sx(DATA + 0x800);
                    c.bus.write32(DATA + 0x804, 0x400 + seed);
                }
                c.g[A1][0] = sx(DATA + 0x400);
                put_ascii(c, DATA + 0x400, "f.dat");
                c.g[A2][0] = seed as u64;
                c.pc = 0x003d_d7e0;
            },
        );
    }

    driven!(gen_ea_file_queue_nop, ee::s14::f_003dd878__generated);
    driven!(hand_ea_file_queue_nop, ea_file_queue_nop);

    /// Plain request allocation plus queue run.
    #[test]
    fn ea_file_queue_nop_matches() {
        check(
            gen_ea_file_queue_nop,
            hand_ea_file_queue_nop,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_d878, 0);
                ea_install_queue_fakes();
                req_fixture(c);
                c.g[A0][0] = seed as u64;
                c.g[A1][0] = (seed / 7) as u64;
                c.g[A2][0] = (seed / 13) as u64;
                c.pc = 0x003d_d878;
            },
        );
    }

    driven!(
        gen_ea_file_on_big_header_read,
        ee::s14::f_003dd8e8__generated
    );
    driven!(hand_ea_file_on_big_header_read, ea_file_on_big_header_read);

    /// Known and unknown archive types; buffered bytes below and above
    /// the header size.
    #[test]
    fn ea_file_on_big_header_read_matches() {
        check(
            gen_ea_file_on_big_header_read,
            hand_ea_file_on_big_header_read,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_d8e8, 0);
                ea_install_queue_fakes();
                req_fixture(c);
                // A1: block whose +0x10 selects the handle; entry words.
                c.bus.write32(DATA + 0x800, EAF_REQ as u32 + 0x20);
                c.bus.write32(DATA + 0x810, EAF_HANDLE as u32);
                c.bus.write32(DATA + 0x804, seed % 0x10);
                c.bus.write32(EAF_REQ + 0x4, (seed / 16 % 0x20) as u32);
                c.bus.write32(EAF_REQ + 0x8, EAF_HANDLE as u32);
                c.bus.write32(EAF_REQ + 0xc, 0);
                // Allocator free callback (unknown-type path).
                c.bus.write32(0x0045_0c18, 0x00c0_0000);
                c.bus.write32(EAF_RET_TYPE, (seed / 32 % 2) as u32);
                c.bus.write32(EAF_RET_HDRSIZE, (seed / 64 % 0x30) as u32);
                c.g[A0][0] = EAF_REQ as u64;
                c.g[A1][0] = sx(DATA + 0x800);
                c.g[A2][0] = sx(DATA + 0x900);
                c.pc = 0x003d_d8e8;
            },
        );
    }

    driven!(gen_ea_file_on_big_opened, ee::s14::f_003dda10__generated);
    driven!(hand_ea_file_on_big_opened, ea_file_on_big_opened);

    /// Header fetch (s0 == 1) and ready-mark paths.
    #[test]
    fn ea_file_on_big_opened_matches() {
        check(
            gen_ea_file_on_big_opened,
            hand_ea_file_on_big_opened,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_da10, 0);
                ea_install_queue_fakes();
                req_fixture(c);
                c.bus.write32(DATA + 0x800, EAF_REQ as u32);
                c.bus.write32(DATA + 0x810, EAF_HANDLE as u32);
                c.bus.write32(EAF_RET_TYPE, EAF_REQ as u32);
                c.g[A0][0] = if seed % 3 == 0 { 0 } else { EAF_REQ as u64 };
                c.g[A1][0] = if seed % 2 == 0 { 1 } else { 2 };
                c.g[A2][0] = sx(DATA + 0x800);
                c.pc = 0x003d_da10;
            },
        );
    }

    driven!(gen_ea_file_queue_mount_big, ee::s14::f_003ddac0__generated);
    driven!(hand_ea_file_queue_mount_big, ea_file_queue_mount_big);

    /// No mounted archives, a name match (queue run), and a full scan
    /// with no match (mount path).
    #[test]
    fn ea_file_queue_mount_big_events() {
        check_events(
            gen_ea_file_queue_mount_big,
            hand_ea_file_queue_mount_big,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_dac0, 0);
                ea_install_queue_fakes();
                req_fixture(c);
                for i in 0..12u32 {
                    c.bus.write32(EAF_HANDLE + 4 * i, 0);
                }
                let mode = seed % 3;
                // Mounted list at DATA+0x900: each node holds a name
                // struct at +0xc whose string starts at +0x10.
                let names = ["other.big", "arch.big", "nomatch.big"];
                let want = ["arch.big", "missing.big", "arch.big"][mode as usize];
                put_ascii(c, DATA + 0x400, want);
                let mut prev = 0u32;
                if mode != 0 {
                    for (k, name) in names.iter().enumerate() {
                        let node = DATA + 0x900 + 64 * k as u32;
                        let st = DATA + 0xb00 + 0x40 * k as u32;
                        put_ascii(c, st + 0x10, name);
                        c.bus.write32(node + 0xc, st);
                        c.bus.write32(node + 0x14, prev);
                        c.bus.write32(node, 0x7000 + k as u32);
                        prev = node;
                    }
                }
                c.bus.write32(0x0051_9ae4, prev);
                // Allocator + root words the mount path reads.
                c.bus.write32(0x0045_0c10 + 0x4, 0x00c0_0000);
                c.bus.write32(0x0045_0c10, 0x00c0_0000);
                c.bus.write32(0x0049_5d68, 0);
                c.bus.write32(0x0049_5d28, 0x3000);
                c.bus.write32(EAF_RET_TYPE, EAF_REQ as u32);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x800);
                c.bus.write32(DATA + 0x800, EAF_HANDLE as u32);
                c.g[A2][0] = 0;
                c.g[A3][0] = seed as u64;
                schedule(c, &mut r, seed, 0x003d_dac0)
            },
        );
    }

    driven!(gen_ea_file_on_big_unmounted, ee::s14::f_003ddc00__generated);
    driven!(hand_ea_file_on_big_unmounted, ea_file_on_big_unmounted);

    /// Release plus queue run.
    #[test]
    fn ea_file_on_big_unmounted_matches() {
        check(
            gen_ea_file_on_big_unmounted,
            hand_ea_file_on_big_unmounted,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_dc00, 0);
                ea_install_queue_fakes();
                req_fixture(c);
                c.bus.write32(EAF_RET_TYPE, 0);
                c.g[A0][0] = EAF_REQ as u64;
                c.g[A2][0] = sx(DATA + 0x800);
                c.bus.write32(DATA + 0x800, EAF_HANDLE as u32);
                c.pc = 0x003d_dc00;
            },
        );
    }

    driven!(
        gen_ea_file_queue_unmount_big,
        ee::s14::f_003ddc30__generated
    );
    driven!(hand_ea_file_queue_unmount_big, ea_file_queue_unmount_big);

    /// Empty and populated mount lists, matching and missing handles.
    #[test]
    fn ea_file_queue_unmount_big_events() {
        check_events(
            gen_ea_file_queue_unmount_big,
            hand_ea_file_queue_unmount_big,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_dc30, 0);
                ea_install_queue_fakes();
                req_fixture(c);
                let mode = seed % 4;
                // Mounted archives at DATA+0x900: +0xc owner word,
                // +0x14 next; the loop compares owner +0xc to the handle.
                let handles = [0x7000u32, 0x8000, 0x9000];
                let mut prev = 0u32;
                if mode != 0 {
                    for (k, &h) in handles.iter().enumerate() {
                        let node = DATA + 0x900 + 64 * k as u32;
                        c.bus.write32(node + 0xc, h);
                        c.bus.write32(node + 0x14, prev);
                        c.bus.write32(node, 0x6000 + k as u32);
                        prev = node;
                    }
                }
                c.bus.write32(0x0051_9ae4, prev);
                let handle = [0u32, 0x8000, 0x1234, 0x9000][mode as usize];
                for i in 0..12u32 {
                    c.bus.write32(EAF_HANDLE + 4 * i, 0);
                }
                c.bus.write32(EAF_RET_TYPE, EAF_REQ as u32);
                c.g[A0][0] = handle as u64;
                c.g[A1][0] = sx(DATA + 0x400);
                put_ascii(c, DATA + 0x400, "arch.big");
                c.g[A2][0] = seed as u64;
                schedule(c, &mut r, seed, 0x003d_dc30)
            },
        );
    }

    driven!(
        gen_ea_file_call_with_priority,
        ee::s14::f_003dddf0__generated
    );
    driven!(hand_ea_file_call_with_priority, ea_file_call_with_priority);

    /// The callback runs through the table; the pending count is saved
    /// and restored around it.
    #[test]
    fn ea_file_call_with_priority_matches() {
        check(
            gen_ea_file_call_with_priority,
            hand_ea_file_call_with_priority,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_ddf0, 0);
                ea_install_queue_fakes();
                req_fixture(c);
                c.bus.write32(0x0051_9ab0 + 0x8, 0x40 + seed % 8);
                c.g[A0][0] = 0x00c0_1000 + (seed % 2) as u64;
                c.g[A1][0] = seed as u64;
                c.g[A2][0] = seed as u64;
                c.g[A3][0] = sx(DATA + 0x400);
                c.pc = 0x003d_ddf0;
            },
        );
    }

    driven!(gen_ea_file_set_priority, ee::s14::f_003dde50__generated);
    driven!(hand_ea_file_set_priority, ea_file_set_priority);

    /// Stores the priority word and runs the queue.
    #[test]
    fn ea_file_set_priority_events() {
        check_events(
            gen_ea_file_set_priority,
            hand_ea_file_set_priority,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_de50, 0);
                ea_install_queue_fakes();
                c.bus.write32(0x0051_9ab0 + 0x8, 0);
                c.g[A0][0] = seed as u64;
                schedule(c, &mut r, seed, 0x003d_de50)
            },
        );
    }

    driven!(gen_ea_file_set_root_path, ee::s14::f_003dde78__generated);
    driven!(hand_ea_file_set_root_path, ea_file_set_root_path);

    /// Same, shorter, and different roots; cd and non-cd modes.
    #[test]
    fn ea_file_set_root_path_matches() {
        check(
            gen_ea_file_set_root_path,
            hand_ea_file_set_root_path,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_de78, 0);
                elf_image(c);
                let roots = ["", "host:", "host:dir", "cdrom:", "other:"];
                let root = roots[seed as usize % roots.len()];
                put_ascii(c, DATA + 0x400, root);
                // Current root buffer + mode flags.
                for i in 0..8u32 {
                    c.bus.write8(0x0051_9ef0 + i, 0);
                }
                put_ascii(
                    c,
                    0x0051_9ef0,
                    ["", "host:", "cdrom:"][seed as usize / 5 % 3],
                );
                c.bus.write32(0x0045_0c50, 0x3000);
                c.bus.write32(0x0045_0c08, 0x495d40);
                put_ascii(c, 0x0049_5d40, "host:");
                c.bus
                    .write32(0x0045_0c48, [0x2, 0x0, 0x3][seed as usize / 15 % 3]);
                c.bus.write32(0x0045_0c44, 0);
                c.g[A0][0] = sx(DATA + 0x400);
                c.pc = 0x003d_de78;
            },
        );
    }

    driven!(gen_ea_file_get_root_path, ee::s14::f_003ddf80__generated);
    driven!(hand_ea_file_get_root_path, ea_file_get_root_path);

    /// Copies the current root (empty and short).
    #[test]
    fn ea_file_get_root_path_events() {
        check_events(
            gen_ea_file_get_root_path,
            hand_ea_file_get_root_path,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_df80, 0);
                for i in 0..8u32 {
                    c.bus.write8(0x0051_9ef0 + i, 0);
                }
                if seed % 2 == 0 {
                    put_ascii(c, 0x0051_9ef0, "host:");
                }
                schedule(c, &mut r, seed, 0x003d_df80)
            },
        );
    }

    driven!(gen_ea_file_parse_big_path, ee::s14::f_003de4d0__generated);
    driven!(hand_ea_file_parse_big_path, ea_file_parse_big_path);

    /// Plain paths, `big|name` paths, direct and archive modes.
    #[test]
    fn ea_file_parse_big_path_matches() {
        check(
            gen_ea_file_parse_big_path,
            hand_ea_file_parse_big_path,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_de4d0, 0);
                elf_image(c);
                ea_install_queue_fakes();
                let paths = ["file.dat", "big|name", "|name", "big|", "a|b|c", ""];
                let path = paths[seed as usize % paths.len()];
                put_ascii(c, DATA + 0x400, path);
                c.bus.write32(
                    EAF_RET_POPEN,
                    if seed / 6 % 3 == 0 { 0 } else { DATA + 0x800 },
                );
                c.bus.write32(EAF_RET_PSIZE, 0x1234 + seed);
                c.bus.write32(EAF_RET_FIND, (seed / 18 % 2) as u32);
                // BigFindEntry's out word on the stack: pre-fill so a
                // missing store shows (sp is fixed per test machine).
                c.bus.write32(DATA + 0x5000, 0xbeef_beef);
                // Mounted list for the entry scan.
                let mut prev = 0u32;
                if seed / 36 % 2 == 0 {
                    for k in 0..2u32 {
                        let node = DATA + 0x900 + 64 * k;
                        put_ascii(c, DATA + 0xb00 + 0x40 * k, "arch.big");
                        c.bus.write32(node + 0xc, DATA + 0xb00 + 0x40 * k);
                        c.bus.write32(node + 0x14, prev);
                        prev = node;
                    }
                }
                c.bus.write32(0x0051_9ae4, prev);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = (seed / 72 % 2) as u64;
                c.g[A2][0] = (seed / 144 % 3) as u64;
                c.g[A3][0] = sx(DATA + 0x5000);
                c.pc = 0x003d_de4d0;
            },
        );
    }

    driven!(
        gen_ea_file_queue_signal_stub,
        ee::s14::f_003de668__generated
    );
    driven!(hand_ea_file_queue_signal_stub, ea_file_queue_signal_stub);

    /// The 2-cycle return.
    #[test]
    fn ea_file_queue_signal_stub_matches() {
        check(
            gen_ea_file_queue_signal_stub,
            hand_ea_file_queue_signal_stub,
            8,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_de68, 0);
                c.pc = 0x003d_de68;
            },
        );
    }
}

/// `void EAFile_CallWithPriority(prio, a1, fn, arg)` (0x3dddf0): stash the
/// queue's pending count, call `fn(arg)` through the table, restore the
/// count and run the queue with `prio`.
pub fn ea_file_call_with_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = c.g[A2][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0x520000;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6550));
        frame.save(c, S2, 0x10);
        c.g[A1][0] = c.g[A3][0];
        c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        let func = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x003d_de24);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
        c.pc = func;
        call_ptr(c, func, 0x003d_de24)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[S2][0]);
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x003d_de34);
        c.g[A0][0] = 0;
        callee::EA_FILE_SERVICE_QUEUE.call(c, 0x003d_de34)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_SetPriority(prio)` (0x3dde50): store the queue priority
/// and run the queue once.
pub fn ea_file_set_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V0][0] = 0x520000;
        let frame = Frame::open(c, 0x10);
        c.sw((c.g[V0][0] as u32).wrapping_sub(0x6548), c.g[A0][0]);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_de68);
        c.g[A0][0] = 0;
        callee::EA_FILE_SERVICE_QUEUE.call(c, 0x003d_de68)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_SetRootPath(path)` (0x3dde78): switch the file root to
/// `path` (creating it), keeping the `0x45c44` mode flags in sync.
pub fn ea_file_set_root_path(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = 0x450000;
        frame.save(c, S1, 0x20);
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc50));
        frame.save_ra(c, 0);
        'b_3ddf6c: {
            'b_3ddf68: {
                'b_3ddf20: {
                    'b_3ddf1c: {
                        'b_3ddee8: {
                            c.cycles += 18;
                            c.g[RA][0] = sx(0x003d_dea0);
                            call_ptr(c, callee::STRLEN, 0x003d_dea0)?;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            c.g[V0][0] = 0x450000;
                            c.g[A2][0] = 0x3;
                            c.g[RA][0] = sx(0x003d_deb8);
                            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc08));
                            call_ptr(c, callee::STRNCMP, 0x003d_deb8)?;
                            if c.g[V0][0] != 0 {
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc50));
                                break 'b_3ddee8;
                            }
                            c.cycles += 6;
                            c.g[V0][0] = 0x450000;
                            c.g[A0][0] = 0x2;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc48));
                            c.g[V1][0] = c.g[V1][0] & 0x3;
                            let keep = c.g[V1][0] == c.g[A0][0];
                            c.g[V1][0] = 0x450000;
                            if keep {
                                break 'b_3ddf68;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = 0x1;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0xc44), c.g[V0][0]);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3));
                            break 'b_3ddf1c;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x003d_def4);
                        c.g[A2][0] = c.g[S1][0];
                        call_ptr(c, callee::STRNCMP, 0x003d_def4)?;
                        let differ = c.g[V0][0] != 0;
                        c.g[V0][0] = 0x520000;
                        if differ {
                            break 'b_3ddf20;
                        }
                        c.cycles += 5;
                        c.g[V1][0] = 0x450000;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc48));
                        c.g[V0][0] = c.g[V0][0] & 0x2;
                        let no_cd = c.g[V0][0] == 0;
                        c.g[V1][0] = 0x450000;
                        if no_cd {
                            break 'b_3ddf68;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0x2;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0xc44), c.g[V0][0]);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        break 'b_3ddf1c;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x520000;
                    break 'b_3ddf20;
                }
                c.cycles += 7;
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6510));
                c.g[RA][0] = sx(0x003d_df30);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::STRCPY, 0x003d_df30)?;
                c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0xff));
                if c.g[V1][0] != 0 {
                    frame.restore(c, S0, 0x30);
                    break 'b_3ddf6c;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003d_df44);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::STRLEN, 0x003d_df44)?;
                let empty = (c.g[V0][0] as i64) <= 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                if empty {
                    break 'b_3ddf68;
                }
                c.cycles += 4;
                c.g[A0][0] = 0x2f;
                c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_sub(0x1));
                let slash = c.g[V1][0] == c.g[A0][0];
                c.g[A0][0] = c.g[S0][0];
                if slash {
                    break 'b_3ddf68;
                }
                c.cycles += 3;
                c.g[A1][0] = 0x490000;
                c.g[RA][0] = sx(0x003d_df68);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5d78));
                call_ptr(c, callee::STRCAT, 0x003d_df68)?;
                break 'b_3ddf68;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
            break 'b_3ddf6c;
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
