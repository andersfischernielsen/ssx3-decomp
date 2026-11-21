//! Hand-written replacements for `ealib/seg_2DCAE8` EAFile/AsyncFile calls
//! (wave `wave-ealib-1.txt` rows 41-60), each with its differential test
//! against the generated body. Moved into the source-tree units by
//! `tools/reorg_modules.py apply oc1122`.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003d_e928, ea_file_get_settings, "EAFile_GetSettings"),
    (0x003d_e9b8, ea_file_set_settings, "EAFile_SetSettings"),
    (
        0x003d_ea48,
        ea_file_on_chunk_transferred,
        "EAFile_OnChunkTransferred",
    ),
    (
        0x003d_eb50,
        ea_file_transfer_chunked,
        "EAFile_TransferChunked",
    ),
    (0x003d_ebf0, ea_file_open, "EAFile_Open"),
    (0x003d_ec60, ea_file_read, "EAFile_Read"),
    // NOTE: no EAFile_Write row: its hardcoded chunk callback (the real
    // queue writer, no hook slot) always returns nonzero on fixture, so
    // TransferChunked's wait loop spins identically on both sides and no
    // bounded fixture exists; the port below is kept and covered by the
    // replay gates (same treatment as EAFile_Shutdown in oc1120).
    (0x003d_eca0, ea_file_close, "EAFile_Close"),
    (0x003d_ecf8, ea_file_get_size, "EAFile_GetSize"),
    (0x003d_ed50, ea_file_mount_big, "EAFile_MountBig"),
    (0x003d_edc0, ea_file_unmount_big, "EAFile_UnmountBig"),
    (0x003d_ee18, ea_file_exists, "EAFile_Exists"),
    (0x003d_ee70, async_file_pool_push, "AsyncFile_PoolPush"),
    (0x003d_eed8, async_file_pool_pop, "AsyncFile_PoolPop"),
    (0x003d_efc0, async_file_free, "AsyncFile_Free"),
    (0x003d_f028, async_file_finish, "AsyncFile_Finish"),
    (0x003d_f0b8, async_file_on_close, "AsyncFile_OnClose"),
    (
        0x003d_f0f0,
        async_file_on_read_chunk,
        "AsyncFile_OnReadChunk",
    ),
    (0x003d_f1e0, async_file_on_size, "AsyncFile_OnSize"),
    (0x003d_f2c0, async_file_on_open, "AsyncFile_OnOpen"),
];

mod callee {
    use super::ee;
    use crate::call::Callee;

    pub(crate) const EA_MEM_ZERO: Callee = Callee {
        addr: 0x003e_6220,
        f: ee::s14::EA_MemZero,
    };
    pub(crate) const EA_MEM_COPY: Callee = Callee {
        addr: 0x003e_6574,
        f: ee::s14::EA_MemCopy,
    };
    pub(crate) const EA_MUTEX_LOCK: Callee = Callee {
        addr: 0x003e_5700,
        f: ee::s14::EAMutex_Lock,
    };
    pub(crate) const EA_MUTEX_UNLOCK: Callee = Callee {
        addr: 0x003e_5760,
        f: ee::s14::EAMutex_Unlock,
    };
    pub(crate) const EA_FILE_RELEASE_REQUEST: Callee = Callee {
        addr: 0x003d_d310,
        f: ee::s14::EAFile_ReleaseRequest,
    };
    pub(crate) const EA_FILE_WAIT_REQUEST: Callee = Callee {
        addr: 0x003d_d1d8,
        f: ee::s14::EAFile_WaitRequest,
    };
    pub(crate) const EA_FILE_GET_REQUEST_STATUS: Callee = Callee {
        addr: 0x003d_ce90,
        f: ee::s14::EAFile_GetRequestStatus,
    };
    pub(crate) const EA_FILE_QUEUE_OPEN: Callee = Callee {
        addr: 0x003d_d4e8,
        f: ee::s14::EAFile_QueueOpen,
    };
    pub(crate) const EA_FILE_QUEUE_CLOSE: Callee = Callee {
        addr: 0x003d_d5a0,
        f: ee::s14::EAFile_QueueClose,
    };
    pub(crate) const EA_FILE_QUEUE_GET_SIZE: Callee = Callee {
        addr: 0x003d_d7e0,
        f: ee::s14::EAFile_QueueGetSize,
    };
    pub(crate) const EA_FILE_QUEUE_MOUNT_BIG: Callee = Callee {
        addr: 0x003d_dac0,
        f: ee::s14::EAFile_QueueMountBig,
    };
    pub(crate) const EA_FILE_QUEUE_UNMOUNT_BIG: Callee = Callee {
        addr: 0x003d_dc30,
        f: ee::s14::EAFile_QueueUnmountBig,
    };
    pub(crate) const EA_FILE_QUEUE_EXISTS: Callee = Callee {
        addr: 0x003d_d438,
        f: ee::s14::EAFile_QueueExists,
    };
    pub(crate) const EA_FILE_QUEUE_READ: Callee = Callee {
        addr: 0x003d_d648,
        f: ee::s14::EAFile_QueueRead,
    };
    pub(crate) const EA_FILE_SET_REQUEST_CALLBACK: Callee = Callee {
        addr: 0x003d_cf70,
        f: ee::s14::EAFile_SetRequestCallback,
    };
    pub(crate) const EA_FILE_TRANSFER_CHUNKED: Callee = Callee {
        addr: 0x003d_eb50,
        f: ee::s14::EAFile_TransferChunked,
    };
    pub(crate) const ASYNC_FILE_POOL_PUSH: Callee = Callee {
        addr: 0x003d_ee70,
        f: ee::s14::AsyncFile_PoolPush,
    };
    pub(crate) const ASYNC_FILE_FREE: Callee = Callee {
        addr: 0x003d_efc0,
        f: ee::s14::AsyncFile_Free,
    };
    pub(crate) const ASYNC_FILE_FINISH: Callee = Callee {
        addr: 0x003d_f028,
        f: ee::s14::AsyncFile_Finish,
    };
}

/// `u32 EAFile_GetSettings(count_ptr)` (0x3de928): copies the active file
/// settings out through the settings globals; always 1.
pub fn ea_file_get_settings(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = ((c.g[S1][0] as i64) < 37) as u64;
        let fits = c.g[V0][0] != 0;
        c.g[V0][0] = 0x450000;
        if !fits {
            c.cycles += 3;
            c.g[RA][0] = sx(0x003d_e954);
            c.g[A1][0] = c.g[S1][0];
            callee::EA_MEM_ZERO.call(c, 0x003d_e954)?;
            c.g[V0][0] = 0x450000;
        }
        c.cycles += 23;
        c.g[A1][0] = 0x450000;
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc58));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc58));
        c.g[A3][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc50));
        c.g[V0][0] = 0x450000;
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc10));
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[A2][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
        c.g[A2][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x003d_e99c);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A3][0]);
        callee::EA_MEM_COPY.call(c, 0x003d_e99c)?;
        c.g[V0][0] = 0x1;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_SetSettings(settings)` (0x3de9b8): installs `settings` as
/// the active file settings.
pub fn ea_file_set_settings(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 36) as u64;
        let skip = c.g[V0][0] == 0;
        c.g[S1][0] = 0x450000;
        if !skip {
            c.cycles += 3;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc10));
            c.g[RA][0] = sx(0x003d_e9e8);
            c.g[A1][0] = 0x24;
            callee::EA_MEM_ZERO.call(c, 0x003d_e9e8)?;
        }
        c.cycles += 24;
        c.g[A1][0] = c.g[S0][0];
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc10));
        c.g[A2][0] = c.lw(c.g[A1][0] as u32);
        c.g[RA][0] = sx(0x003d_e9fc);
        c.g[A0][0] = c.g[S0][0];
        callee::EA_MEM_COPY.call(c, 0x003d_e9fc)?;
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = 0x450000;
        c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc58));
        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = 0x450000;
        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = 0x1;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.sw((c.g[A1][0] as u32).wrapping_add(0xc58), c.g[A3][0]);
        c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.sw((c.g[A2][0] as u32).wrapping_add(0xc50), c.g[T2][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[T1][0]);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_OnChunkTransferred(kind, request, result)` (0x3dea48):
/// accumulates a transferred chunk into the request and runs its callback.
pub fn ea_file_on_chunk_transferred(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_ea64);
        c.g[S1][0] = c.g[A2][0];
        callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_ea64)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 0x1;
        let not_chunk = c.g[S0][0] != c.g[V0][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x20), 0);
        if not_chunk {
            c.cycles += 2;
            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0);
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 13;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            c.g[V1][0] = ((c.g[A0][0] as i64)
                < (c.lw((c.g[S1][0] as u32).wrapping_add(0x14)) as i64))
                as u64;
            if c.g[V1][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            } else {
                c.cycles += 3;
                c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0);
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
            let finished = (c.g[V0][0] as i64) <= 0;
            frame.restore(c, S0, 0x20);
            if !finished {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = 0x8000;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let fits = c.g[V0][0] == 0;
                if !fits {
                    c.cycles += 3;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                }
                c.cycles += 11;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[T1][0] = c.g[S1][0];
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.g[T0][0] = c.lw(c.g[S1][0] as u32);
                let chunk_cb = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003d_eb18);
                c.pc = chunk_cb;
                call_ptr(c, chunk_cb, 0x003d_eb18)?;
                let missing = c.g[V0][0] == 0;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                if missing {
                    c.cycles += 2;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0);
                    frame.restore(c, S0, 0x20);
                } else {
                    c.cycles += 6;
                    c.g[A1][0] = 0x3e0000;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003d_eb30);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x15b8));
                    callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_eb30)?;
                    frame.restore(c, S0, 0x20);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_TransferChunked(...)` (0x3deb50): runs one chunk of a
/// split transfer through its callback, then waits for the request.
pub fn ea_file_transfer_chunked(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003d_ebc0;
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = c.g[T1][0];
        c.sw(c.g[SP][0] as u32, c.g[T0][0]);
        c.g[T1][0] = 0x8000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[A3][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[A3][0]);
        frame.save_ra(c, 0x30);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A2][0]);
        c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let fits = c.g[V0][0] == 0;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
        if !fits {
            c.cycles += 1;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[T1][0]);
        }
        'b_3debdc: {
            c.cycles += 7;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            c.g[T1][0] = c.g[SP][0];
            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
            let chunk_cb = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x003d_eba4);
            c.pc = chunk_cb;
            call_ptr(c, chunk_cb, 0x003d_eba4)?;
            let null_cb = c.g[V0][0] == 0;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
            if !null_cb {
                c.cycles += 5;
                c.g[A1][0] = 0x3e0000;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x003d_ebbc);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x15b8));
                callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_ebbc)?;
                loop {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003d_ebc8);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    callee::EA_FILE_WAIT_REQUEST.call(c, 0x003d_ebc8)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                    let more_cb = c.g[V1][0] != 0;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    if !more_cb {
                        c.cycles += 2;
                        let more_req = c.g[V0][0] != 0;
                        if !more_req {
                            break 'b_3debdc;
                        }
                    }
                    if event_due(c) {
                        deliver_event_here(c, HEAD);
                    }
                }
            }
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_Open(path, mode, out_result)` (0x3debf0): queues an open,
/// waits, and reports status through `out_result`.
pub fn ea_file_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_ec14);
        c.g[S2][0] = 0;
        callee::EA_FILE_QUEUE_OPEN.call(c, 0x003d_ec14)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.sw(c.g[S1][0] as u32, 0);
        } else {
            c.cycles += 9;
            c.g[RA][0] = sx(0x003d_ec28);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_WAIT_REQUEST.call(c, 0x003d_ec28)?;
            c.g[RA][0] = sx(0x003d_ec30);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_GET_REQUEST_STATUS.call(c, 0x003d_ec30)?;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x003d_ec40);
            c.g[S2][0] = (c.g[V0][0] < 0x1u64) as u64;
            callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_ec40)?;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_Read(...)` (0x3dec60): a chunked transfer with the read
/// callback selected.
pub fn ea_file_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[T1][0] = 0x3e0000;
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_ec74);
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x29b8));
        callee::EA_FILE_TRANSFER_CHUNKED.call(c, 0x003d_ec74)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_Close(handle)` (0x3deca0): queues a close, waits, and
/// reports whether anything was released.
pub fn ea_file_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_ecbc);
        c.g[S1][0] = 0;
        callee::EA_FILE_QUEUE_CLOSE.call(c, 0x003d_ecbc)?;
        c.g[S0][0] = c.g[V0][0];
        let null = c.g[S0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !null {
            c.cycles += 6;
            c.g[RA][0] = sx(0x003d_ecd0);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_WAIT_REQUEST.call(c, 0x003d_ecd0)?;
            c.g[RA][0] = sx(0x003d_ecd8);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_ecd8)?;
            c.g[S1][0] = (0u64 < c.g[V0][0]) as u64;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_GetSize(handle)` (0x3decf8): queues a size query, waits,
/// and returns the released byte count.
pub fn ea_file_get_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_ed14);
        c.g[S1][0] = 0;
        callee::EA_FILE_QUEUE_GET_SIZE.call(c, 0x003d_ed14)?;
        c.g[S0][0] = c.g[V0][0];
        let null = c.g[S0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !null {
            c.cycles += 6;
            c.g[RA][0] = sx(0x003d_ed28);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_WAIT_REQUEST.call(c, 0x003d_ed28)?;
            c.g[RA][0] = sx(0x003d_ed30);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_ed30)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_MountBig(path, mode, out_result)` (0x3ded50): queues a big
/// mount, waits, and reports status through `out_result`.
pub fn ea_file_mount_big(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_ed74);
        c.g[S2][0] = 0;
        callee::EA_FILE_QUEUE_MOUNT_BIG.call(c, 0x003d_ed74)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.sw(c.g[S1][0] as u32, 0);
        } else {
            c.cycles += 9;
            c.g[RA][0] = sx(0x003d_ed88);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_WAIT_REQUEST.call(c, 0x003d_ed88)?;
            c.g[RA][0] = sx(0x003d_ed90);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_GET_REQUEST_STATUS.call(c, 0x003d_ed90)?;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x003d_eda0);
            c.g[S2][0] = (c.g[V0][0] < 0x1u64) as u64;
            callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_eda0)?;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_UnmountBig(handle)` (0x3dedc0): queues an unmount, waits,
/// and reports whether anything was released.
pub fn ea_file_unmount_big(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_eddc);
        c.g[S1][0] = 0;
        callee::EA_FILE_QUEUE_UNMOUNT_BIG.call(c, 0x003d_eddc)?;
        c.g[S0][0] = c.g[V0][0];
        let null = c.g[S0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !null {
            c.cycles += 6;
            c.g[RA][0] = sx(0x003d_edf0);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_WAIT_REQUEST.call(c, 0x003d_edf0)?;
            c.g[RA][0] = sx(0x003d_edf8);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_edf8)?;
            c.g[S1][0] = (0u64 < c.g[V0][0]) as u64;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_Exists(path)` (0x3dee18): queues an existence check, waits,
/// and reports whether anything was released.
pub fn ea_file_exists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_ee34);
        c.g[S1][0] = 0;
        callee::EA_FILE_QUEUE_EXISTS.call(c, 0x003d_ee34)?;
        c.g[S0][0] = c.g[V0][0];
        let null = c.g[S0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !null {
            c.cycles += 6;
            c.g[RA][0] = sx(0x003d_ee48);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_WAIT_REQUEST.call(c, 0x003d_ee48)?;
            c.g[RA][0] = sx(0x003d_ee50);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_ee50)?;
            c.g[S1][0] = (0u64 < c.g[V0][0]) as u64;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AsyncFile_PoolPush(pool, node)` (0x3dee70): links `node` at the
/// head of the async pool under its mutex.
pub fn async_file_pool_push(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0x520000;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x003d_ee98);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x63f8));
        callee::EA_MUTEX_LOCK.call(c, 0x003d_ee98)?;
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        let chained = c.g[V0][0] != 0;
        if chained {
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            c.cycles += 1;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S2][0]);
        } else {
            c.cycles += 2;
            c.sw(c.g[S1][0] as u32, c.g[S2][0]);
        }
        // Shared tail: link the node second and unlock.
        c.cycles += 10;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[S2][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x63f8));
        c.g[RA][0] = sx(0x003d_eec0);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0);
        callee::EA_MUTEX_UNLOCK.call(c, 0x003d_eec0)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 AsyncFile_PoolPop(pool)` (0x3deed8): unlinks and returns the head
/// of the async pool under its mutex, or 0 when empty.
pub fn async_file_pool_pop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x520000;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x63f8));
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_ef00);
        c.g[A0][0] = c.g[S2][0];
        callee::EA_MUTEX_LOCK.call(c, 0x003d_ef00)?;
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = 0;
        if !empty {
            c.cycles += 3;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        }
        c.cycles += 9;
        c.g[RA][0] = sx(0x003d_ef20);
        c.g[A0][0] = c.g[S2][0];
        callee::EA_MUTEX_UNLOCK.call(c, 0x003d_ef20)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AsyncFile_Free(block)` (0x3defc0): runs the block's free
/// callback when its generation says so, then returns it to the pool.
pub fn async_file_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        if c.g[V0][0] == 0 {
            c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            c.g[V0][0] = (c.g[A0][0] < 0x2u64) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
            } else {
                c.cycles += 5;
                c.g[V1][0] = 0x450000;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc18));
                let free_cb = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003d_effc);
                c.pc = free_cb;
                call_ptr(c, free_cb, 0x003d_effc)?;
                c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
            }
        }
        c.cycles += 10;
        c.g[A0][0] = 0x520000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0);
        c.g[A1][0] = c.g[S0][0];
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.g[RA][0] = sx(0x003d_f018);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6408));
        callee::ASYNC_FILE_POOL_PUSH.call(c, 0x003d_f018)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AsyncFile_Finish(block)` (0x3df028): detaches a finished block
/// and either frees it or runs its done callback.
pub fn async_file_finish(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0x520000;
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x63f8));
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_f054);
        c.g[A0][0] = c.g[S0][0];
        callee::EA_MUTEX_LOCK.call(c, 0x003d_f054)?;
        c.g[A0][0] = c.g[S0][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), 0);
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[S3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[RA][0] = sx(0x003d_f06c);
        c.g[S2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        callee::EA_MUTEX_UNLOCK.call(c, 0x003d_f06c)?;
        let pending = c.g[S0][0] != 0;
        if pending {
            c.cycles += 2;
            let queued = c.g[S2][0] != 0;
            frame.restore(c, S0, 0x40);
            if !queued {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003d_f084);
                c.g[A0][0] = c.g[S1][0];
                callee::ASYNC_FILE_FREE.call(c, 0x003d_f084)?;
                frame.restore(c, S0, 0x40);
            }
        } else {
            c.cycles += 2;
            let done_cb = c.g[S3][0] != 0;
            frame.restore(c, S0, 0x40);
            if done_cb {
                c.cycles += 3;
                let t = c.g[S3][0] as u32;
                c.g[RA][0] = sx(0x003d_f09c);
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                c.pc = t;
                call_ptr(c, t, 0x003d_f09c)?;
                frame.restore(c, S0, 0x40);
            }
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AsyncFile_OnClose(block)` (0x3df0b8): releases the close request
/// and finishes the block.
pub fn async_file_on_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[RA][0] = sx(0x003d_f0d4);
        callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_f0d4)?;
        c.g[RA][0] = sx(0x003d_f0dc);
        c.g[A0][0] = c.g[S0][0];
        callee::ASYNC_FILE_FINISH.call(c, 0x003d_f0dc)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AsyncFile_OnReadChunk(block)` (0x3df0f0): releases the read
/// request and either closes a short chunk or queues the next read.
pub fn async_file_on_read_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[RA][0] = sx(0x003d_f10c);
        callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_f10c)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = 0x7fff;
        c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[A2][0]);
        let short = c.g[V1][0] == 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        // A long chunk with a clear flag queues the next read; every other
        // shape closes the chunk (generated `break 'b_3df140` vs `break
        // 'b_3df178` out of the flag block).
        let mut close_chunk = short;
        if !short {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            if c.g[V0][0] == 0 {
                c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
            } else {
                close_chunk = true;
            }
        }
        if close_chunk {
            c.cycles += 8;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = 0x63;
            c.g[RA][0] = sx(0x003d_f150);
            c.g[A2][0] = c.g[S0][0];
            callee::EA_FILE_QUEUE_CLOSE.call(c, 0x003d_f150)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 6;
                c.g[A1][0] = 0x3e0000;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[RA][0] = sx(0x003d_f170);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf48));
                callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_f170)?;
                frame.restore(c, S0, 0x10);
            }
        } else {
            c.cycles += 17;
            c.g[A3][0] = 0x8000;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
            c.g[T0][0] = 0x63;
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[A1][0] as u32));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T1][0] as i64)) as u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
            if c.g[V0][0] == 0 {
                c.g[A3][0] = c.g[T1][0];
            }
            c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[T1][0]);
            c.g[A1][0] = c.g[V1][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
            c.g[RA][0] = sx(0x003d_f1ac);
            c.g[T1][0] = c.g[S0][0];
            callee::EA_FILE_QUEUE_READ.call(c, 0x003d_f1ac)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 5;
                c.g[A1][0] = 0x3e0000;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[RA][0] = sx(0x003d_f1cc);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf10));
                callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_f1cc)?;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AsyncFile_OnSize(block)` (0x3df1e0): releases the size request
/// and either closes a short block or queues the next read.
pub fn async_file_on_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[RA][0] = sx(0x003d_f1fc);
        callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_f1fc)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        let empty = c.g[V0][0] == 0;
        if !empty {
            c.cycles += 8;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = 0x63;
            c.g[RA][0] = sx(0x003d_f21c);
            c.g[A2][0] = c.g[S0][0];
            callee::EA_FILE_QUEUE_CLOSE.call(c, 0x003d_f21c)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 6;
                c.g[A1][0] = 0x3e0000;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[RA][0] = sx(0x003d_f23c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf48));
                callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_f23c)?;
                frame.restore(c, S0, 0x10);
            }
        } else {
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
            c.cycles += 22;
            c.g[V1][0] = 0x450000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[A1][0]);
            c.g[A0][0] = 0x490000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc14));
            let alloc_cb = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003d_f25c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5da8));
            c.pc = alloc_cb;
            call_ptr(c, alloc_cb, 0x003d_f25c)?;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
            c.g[A3][0] = 0x8000;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
            c.g[T0][0] = 0x63;
            c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
            if c.g[V1][0] == 0 {
                c.g[A3][0] = c.g[A2][0];
            }
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.g[A2][0] = c.g[V0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
            c.g[RA][0] = sx(0x003d_f28c);
            c.g[T1][0] = c.g[S0][0];
            callee::EA_FILE_QUEUE_READ.call(c, 0x003d_f28c)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 5;
                c.g[A1][0] = 0x3e0000;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[RA][0] = sx(0x003d_f2ac);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf10));
                callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_f2ac)?;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AsyncFile_OnOpen(block)` (0x3df2c0): releases the open request
/// and either finishes a failed open or chains close/size/read steps.
pub fn async_file_on_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[RA][0] = sx(0x003d_f2dc);
        callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_f2dc)?;
        c.g[A0][0] = c.g[V0][0];
        let failed = c.g[A0][0] != 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[A0][0]);
        if !failed {
            c.cycles += 4;
            c.g[RA][0] = sx(0x003d_f2f0);
            c.g[A0][0] = c.g[S0][0];
            callee::ASYNC_FILE_FINISH.call(c, 0x003d_f2f0)?;
            frame.restore(c, S0, 0x10);
        } else {
            // Only the double-skip (empty and unchained) reaches the size
            // query; the close/read arms break straight to the epilogue.
            let mut done = false;
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            let empty = c.g[V0][0] == 0;
            c.g[A1][0] = 0x63;
            if !empty {
                c.cycles += 6;
                c.g[RA][0] = sx(0x003d_f30c);
                c.g[A2][0] = c.g[S0][0];
                callee::EA_FILE_QUEUE_CLOSE.call(c, 0x003d_f30c)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                if c.g[V0][0] == 0 {
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 6;
                    c.g[A1][0] = 0x3e0000;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[RA][0] = sx(0x003d_f32c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf48));
                    callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_f32c)?;
                    frame.restore(c, S0, 0x10);
                    done = true;
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                let chained = c.g[V0][0] != 0;
                c.g[A3][0] = 0x8000;
                if !chained {
                    c.cycles += 12;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                    c.g[T0][0] = 0x63;
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
                    if c.g[V0][0] == 0 {
                        c.g[A3][0] = c.g[V1][0];
                    }
                    c.g[RA][0] = sx(0x003d_f360);
                    c.g[T1][0] = c.g[S0][0];
                    callee::EA_FILE_QUEUE_READ.call(c, 0x003d_f360)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    if c.g[V0][0] == 0 {
                        frame.restore(c, S0, 0x10);
                    } else {
                        c.cycles += 6;
                        c.g[A1][0] = 0x3e0000;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                        c.g[RA][0] = sx(0x003d_f380);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf10));
                        callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_f380)?;
                        frame.restore(c, S0, 0x10);
                        done = true;
                    }
                }
            }
            // Chain the size query after the open steps.
            if !done {
                c.cycles += 6;
                c.g[RA][0] = sx(0x003d_f390);
                c.g[A2][0] = c.g[S0][0];
                callee::EA_FILE_QUEUE_GET_SIZE.call(c, 0x003d_f390)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                if c.g[V0][0] == 0 {
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = 0x3e0000;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[RA][0] = sx(0x003d_f3b0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xe20));
                    callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_f3b0)?;
                    frame.restore(c, S0, 0x10);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::driven;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
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

    /// Magic-gated fake control block and request/result cells.
    pub(crate) const EAF_CTRL: u32 = DATA + 0x2_0000;
    pub(crate) const EAF_MAGIC: u32 = 0xeaf0_0001;
    pub(crate) const EAF_REQ: u32 = DATA + 0x2000;
    pub(crate) const EAF_HANDLE: u32 = DATA + 0x3000;

    /// Fixture-word returns for the small query callees.
    pub(crate) const EAF_RET_TYPE: u32 = DATA + 0x2_0010;
    pub(crate) const EAF_RET_SETCB: u32 = DATA + 0x2_0024;

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

    pub(crate) fn ea_fake_mutex_lock(c: &mut Cpu) {
        // Unconditional: the real lock would spin on fixture data.
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0xeaf0_1010, ra);
        c.g[V0][0] = 0;
    }

    pub(crate) fn ea_fake_mutex_unlock(c: &mut Cpu) {
        // Unconditional: see `ea_fake_mutex_lock`.
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0xeaf0_1011, ra);
        c.g[V0][0] = 0;
    }

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
        ea_fake_set_request_callback,
        0,
        ee::s14::f_003dcf70__generated,
        EAF_RET_SETCB,
        0xeaf0_100e
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
            addr_of_mut!(ee::s14::OVERRIDE_003DCF70),
            ea_fake_set_request_callback,
            fakes::prev_cell(ea_fake_set_request_callback),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5700),
            ea_fake_mutex_lock,
            fakes::prev_cell(ea_fake_mutex_lock),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5760),
            ea_fake_mutex_unlock,
            fakes::prev_cell(ea_fake_mutex_unlock),
        );
    }

    /// A zeroed request block plus the table entry selecting it, with the
    /// fake magic on.
    fn req_fixture(c: &mut Cpu) {
        for i in 0..12u32 {
            c.bus.write32(EAF_REQ + 4 * i, 0);
        }
        c.bus.write32(EAF_CTRL, EAF_MAGIC);
        table_setup(c);
    }

    /// A zeroed async pool block of two nodes at `base`.
    fn pool_fixture(c: &mut Cpu, base: u32, head: u32) {
        for i in 0..16u32 {
            c.bus.write32(base + 4 * i, 0);
            c.bus.write32(base + 0x40 + 4 * i, 0);
        }
        c.bus.write32(base, head);
        c.bus.write32(0x0051_9c08, 0);
        c.bus.write32(EAF_CTRL, EAF_MAGIC);
    }

    driven!(gen_ea_file_get_settings, ee::s14::f_003de928__generated);
    driven!(hand_ea_file_get_settings, ea_file_get_settings);

    /// Counts below and above the zero-fill threshold.
    #[test]
    fn ea_file_get_settings_matches() {
        check(
            gen_ea_file_get_settings,
            hand_ea_file_get_settings,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_e928, 0);
                let count = [0u32, 1, 36, 37, 64, 0xffff][seed as usize % 6];
                c.bus.write32(DATA + 0x100, count);
                c.bus.write32(0x0045_0c58, 0x11 + seed);
                c.bus.write32(0x0045_0c50, 0x22 + seed);
                c.bus.write32(0x0045_0c64, seed % 2);
                c.bus.write32(0x0045_0c5c, 0x44 + seed);
                for i in 0..8u32 {
                    c.bus.write32(0x0045_0c10 + 4 * i, 0xbeef_0000 + seed);
                }
                c.g[A0][0] = sx(DATA + 0x100);
                c.pc = 0x003d_e928;
            },
        );
    }

    driven!(gen_ea_file_set_settings, ee::s14::f_003de9b8__generated);
    driven!(hand_ea_file_set_settings, ea_file_set_settings);

    /// Counts below and above the zero-fill threshold, varied words.
    #[test]
    fn ea_file_set_settings_matches() {
        check(
            gen_ea_file_set_settings,
            hand_ea_file_set_settings,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_e9b8, 0);
                let count = [0u32, 1, 35, 36, 72][seed as usize % 5];
                c.bus.write32(DATA + 0x100, count);
                for i in 1..10u32 {
                    c.bus.write32(DATA + 0x100 + 4 * i, 0x1000 * seed + i);
                }
                for i in 0..8u32 {
                    c.bus.write32(0x0045_0c10 + 4 * i, 0xbeef_0000 + seed);
                }
                c.g[A0][0] = sx(DATA + 0x100);
                c.pc = 0x003d_e9b8;
            },
        );
    }

    driven!(
        gen_ea_file_on_chunk_transferred,
        ee::s14::f_003dea48__generated
    );
    driven!(
        hand_ea_file_on_chunk_transferred,
        ea_file_on_chunk_transferred
    );

    /// Chunk and non-chunk kinds; byte counts below, at and above the
    /// remaining length; null and queue callbacks.
    #[test]
    fn ea_file_on_chunk_transferred_matches() {
        check(
            gen_ea_file_on_chunk_transferred,
            hand_ea_file_on_chunk_transferred,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_ea48, 0);
                ea_install_queue_fakes();
                req_fixture(c);
                // Request block fields the chunk math walks.
                c.bus.write32(EAF_REQ + 0x4, 0x100 + seed);
                c.bus.write32(EAF_REQ + 0x8, 0x200 + seed);
                c.bus.write32(EAF_REQ + 0x10, 0x300 + seed);
                c.bus.write32(EAF_REQ + 0x18, 0x400 + seed);
                c.bus
                    .write32(EAF_REQ + 0x14, [0u32, 0x10, 0x1000][seed as usize / 2 % 3]);
                c.bus
                    .write32(EAF_REQ + 0xc, [0u32, 0x20, 0x9000][seed as usize / 6 % 3]);
                c.bus.write32(
                    EAF_REQ + 0x1c,
                    if seed / 18 % 2 == 0 { 0 } else { 0x003d_d878 },
                );
                c.bus.write32(EAF_RET_TYPE, (seed / 36 % 2) as u32);
                c.bus.write32(EAF_RET_SETCB, (seed / 72 % 2) as u32);
                c.g[A0][0] = (seed / 144 % 3) as u64;
                c.g[A1][0] = if seed % 2 == 0 { 1 } else { 0 };
                c.g[A2][0] = EAF_REQ as u64;
                c.pc = 0x003d_ea48;
            },
        );
    }

    driven!(gen_ea_file_transfer_chunked, ee::s14::f_003deb50__generated);
    driven!(hand_ea_file_transfer_chunked, ea_file_transfer_chunked);

    /// Clamp on and off; null chunk callback (a live callback would spin
    /// the wait loop on fixture data, so the replay gates cover it).
    #[test]
    fn ea_file_transfer_chunked_matches() {
        check(
            gen_ea_file_transfer_chunked,
            hand_ea_file_transfer_chunked,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_eb50, 0);
                ea_install_queue_fakes();
                req_fixture(c);
                c.bus.write32(EAF_RET_TYPE, (seed % 2) as u32);
                c.bus.write32(EAF_RET_SETCB, (seed / 2 % 2) as u32);
                c.g[A0][0] = 0x1000 + seed as u64;
                c.g[A1][0] = 0x2000 + seed as u64;
                c.g[A2][0] = 0x3000 + seed as u64;
                c.g[A3][0] = [0u32, 0x9000][seed as usize / 4 % 2] as u64;
                c.g[T0][0] = 0x4000 + seed as u64;
                // Chunk callback: an empty-pool pop returns 0, taking the
                // null-callback exit (a live callback would spin the wait
                // loop on fixture data; the replay gates cover it).
                c.bus.write32(DATA + 0x700, 0);
                c.g[A0][0] = sx(DATA + 0x700);
                c.g[T1][0] = 0x003d_eed8;
                c.pc = 0x003d_eb50;
            },
        );
    }

    driven!(gen_ea_file_open, ee::s14::f_003debf0__generated);
    driven!(hand_ea_file_open, ea_file_open);

    /// Null and live queue results; varied status words.
    #[test]
    fn ea_file_open_matches() {
        check(gen_ea_file_open, hand_ea_file_open, 64, &|c, seed| {
            let _r = machine(c, seed, 0x3d_ebf0, 0);
            ea_install_queue_fakes();
            req_fixture(c);
            put_ascii(c, DATA + 0x400, "file.dat");
            c.bus.write32(EAF_REQ + 0x8, (seed % 3) as u32);
            c.bus
                .write32(EAF_RET_TYPE, if seed / 3 % 2 == 0 { 0 } else { EAF_REQ });
            c.g[A0][0] = sx(DATA + 0x400);
            c.g[A1][0] = (seed / 6 % 4) as u64;
            c.g[A2][0] = 0;
            c.g[A3][0] = sx(DATA + 0x500);
            c.pc = 0x003d_ebf0;
        });
    }

    driven!(gen_ea_file_read, ee::s14::f_003dec60__generated);
    driven!(hand_ea_file_read, ea_file_read);

    /// Read is a chunked transfer with the read callback; the callback is
    /// faked to null so the transfer exits (a live callback would spin
    /// the wait loop on fixture data; the replay gates cover it).
    #[test]
    fn ea_file_read_matches() {
        check(gen_ea_file_read, hand_ea_file_read, 48, &|c, seed| {
            let _r = machine(c, seed, 0x3d_ec60, 0);
            ea_install_queue_fakes();
            req_fixture(c);
            c.bus.write32(EAF_RET_TYPE, 0);
            c.bus.write32(EAF_RET_SETCB, (seed / 2 % 2) as u32);
            c.g[A0][0] = 0x1000 + seed as u64;
            c.g[A1][0] = sx(DATA + 0x600);
            c.g[A2][0] = 0x3000 + seed as u64;
            c.g[A3][0] = [0u32, 0x9000][seed as usize / 4 % 2] as u64;
            c.g[T0][0] = 0x4000 + seed as u64;
            // Chunk callback: an empty-pool pop returns 0, taking the
            // null-callback exit (see transfer_chunked).
            c.bus.write32(DATA + 0x700, 0);
            c.g[A0][0] = sx(DATA + 0x700);
            c.g[T1][0] = 0x003d_eed8;
            c.pc = 0x003d_ec60;
        });
    }

    // NOTE: no `EAFile_Write` test or OVERRIDES row. Write hard-codes its
    // chunk callback to the real queue writer (no hook slot), which always
    // returns nonzero on fixture (it stores then reloads its own masked
    // result word): TransferChunked's wait loop then spins forever on both
    // sides (identical behavior, verified bit-equal in a debug run), so no
    // bounded fixture exists. The port below is a straight-line wrapper
    // identical in shape to the passing Read port; the replay gates cover it.

    driven!(gen_ea_file_close, ee::s14::f_003deca0__generated);
    driven!(hand_ea_file_close, ea_file_close);

    /// Null and live queue results; zero and nonzero releases.
    #[test]
    fn ea_file_close_matches() {
        check(gen_ea_file_close, hand_ea_file_close, 64, &|c, seed| {
            let _r = machine(c, seed, 0x3d_eca0, 0);
            ea_install_queue_fakes();
            req_fixture(c);
            c.bus
                .write32(EAF_RET_TYPE, if seed % 3 == 0 { 0 } else { EAF_REQ });
            c.g[A0][0] = EAF_HANDLE as u64;
            c.pc = 0x003d_eca0;
        });
    }

    driven!(gen_ea_file_get_size, ee::s14::f_003decf8__generated);
    driven!(hand_ea_file_get_size, ea_file_get_size);

    /// Null and live queue results; the release value passes through.
    #[test]
    fn ea_file_get_size_matches() {
        check(
            gen_ea_file_get_size,
            hand_ea_file_get_size,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_ecf8, 0);
                ea_install_queue_fakes();
                req_fixture(c);
                c.bus
                    .write32(EAF_RET_TYPE, if seed % 3 == 0 { 0 } else { 0x1234 + seed });
                c.g[A0][0] = EAF_HANDLE as u64;
                c.pc = 0x003d_ecf8;
            },
        );
    }

    driven!(gen_ea_file_mount_big, ee::s14::f_003ded50__generated);
    driven!(hand_ea_file_mount_big, ea_file_mount_big);

    /// Null and live queue results; varied status words.
    #[test]
    fn ea_file_mount_big_matches() {
        check(
            gen_ea_file_mount_big,
            hand_ea_file_mount_big,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_ed50, 0);
                ea_install_queue_fakes();
                req_fixture(c);
                put_ascii(c, DATA + 0x400, "arch.big");
                c.bus.write32(EAF_REQ + 0x8, (seed % 3) as u32);
                c.bus
                    .write32(EAF_RET_TYPE, if seed / 3 % 2 == 0 { 0 } else { EAF_REQ });
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = (seed / 6 % 4) as u64;
                c.g[A2][0] = 0;
                c.g[A3][0] = sx(DATA + 0x500);
                c.pc = 0x003d_ed50;
            },
        );
    }

    driven!(gen_ea_file_unmount_big, ee::s14::f_003dedc0__generated);
    driven!(hand_ea_file_unmount_big, ea_file_unmount_big);

    /// Null and live queue results; zero and nonzero releases.
    #[test]
    fn ea_file_unmount_big_matches() {
        check(
            gen_ea_file_unmount_big,
            hand_ea_file_unmount_big,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_edc0, 0);
                ea_install_queue_fakes();
                req_fixture(c);
                c.bus
                    .write32(EAF_RET_TYPE, if seed % 3 == 0 { 0 } else { 0x10 + seed });
                c.g[A0][0] = EAF_HANDLE as u64;
                c.pc = 0x003d_edc0;
            },
        );
    }

    driven!(gen_ea_file_exists, ee::s14::f_003dee18__generated);
    driven!(hand_ea_file_exists, ea_file_exists);

    /// Null and live queue results; zero and nonzero releases.
    #[test]
    fn ea_file_exists_matches() {
        check(gen_ea_file_exists, hand_ea_file_exists, 64, &|c, seed| {
            let _r = machine(c, seed, 0x3d_ee18, 0);
            ea_install_queue_fakes();
            req_fixture(c);
            put_ascii(c, DATA + 0x400, "file.dat");
            c.bus
                .write32(EAF_RET_TYPE, if seed % 3 == 0 { 0 } else { 0x20 + seed });
            c.g[A0][0] = sx(DATA + 0x400);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            c.pc = 0x003d_ee18;
        });
    }

    driven!(gen_async_file_pool_push, ee::s14::f_003dee70__generated);
    driven!(hand_async_file_pool_push, async_file_pool_push);

    /// Empty and chained pools; nodes second or fresh.
    #[test]
    fn async_file_pool_push_matches() {
        check(
            gen_async_file_pool_push,
            hand_async_file_pool_push,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_ee70, 0);
                ea_install_queue_fakes();
                let base = DATA + 0x600;
                let node = base + 0x80;
                let node2 = base + 0xc0;
                pool_fixture(c, base, if seed % 2 == 0 { 0 } else { node2 });
                c.bus
                    .write32(node2 + 0x4, if seed / 2 % 2 == 0 { 0 } else { node });
                c.bus.write32(node + 0x4, 0xbeef_0000 + seed);
                c.g[A0][0] = base as u64;
                c.g[A1][0] = node as u64;
                c.pc = 0x003d_ee70;
            },
        );
    }

    driven!(gen_async_file_pool_pop, ee::s14::f_003deed8__generated);
    driven!(hand_async_file_pool_pop, async_file_pool_pop);

    /// Empty, single-node and two-node pools.
    #[test]
    fn async_file_pool_pop_matches() {
        check(
            gen_async_file_pool_pop,
            hand_async_file_pool_pop,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_eed8, 0);
                ea_install_queue_fakes();
                let base = DATA + 0x600;
                let node = base + 0x80;
                let node2 = base + 0xc0;
                let mode = seed % 3;
                pool_fixture(c, base, if mode == 0 { 0 } else { node });
                c.bus.write32(node + 0x4, if mode == 2 { node2 } else { 0 });
                c.bus.write32(node2 + 0x4, 0xbeef_0000 + seed);
                c.g[A0][0] = base as u64;
                c.pc = 0x003d_eed8;
            },
        );
    }

    driven!(gen_async_file_free, ee::s14::f_003defc0__generated);
    driven!(hand_async_file_free, async_file_free);

    /// Live and dead blocks; young and old generations; null and live
    /// free callbacks (unmapped targets miss identically both sides).
    #[test]
    fn async_file_free_matches() {
        check(gen_async_file_free, hand_async_file_free, 96, &|c, seed| {
            let _r = machine(c, seed, 0x3d_efc0, 0);
            ea_install_queue_fakes();
            let base = DATA + 0x600;
            pool_fixture(c, base, 0);
            let blk = base + 0x80;
            c.bus.write32(blk, 0x40 + seed % 4);
            c.bus
                .write32(blk + 0x10, [0u32, 1, 2][seed as usize / 4 % 3]);
            c.bus
                .write32(blk + 0x14, [0u32, 1, 2, 3][seed as usize / 12 % 4]);
            c.bus.write32(
                0x0045_0c18,
                if seed / 48 % 2 == 0 { 0 } else { 0x00c0_0000 },
            );
            c.g[A0][0] = blk as u64;
            c.pc = 0x003d_efc0;
        });
    }

    driven!(gen_async_file_finish, ee::s14::f_003df028__generated);
    driven!(hand_async_file_finish, async_file_finish);

    /// Pending and detached blocks; queued and unqueued; null and live
    /// done callbacks (unmapped targets miss identically both sides).
    #[test]
    fn async_file_finish_matches() {
        check(
            gen_async_file_finish,
            hand_async_file_finish,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_f028, 0);
                ea_install_queue_fakes();
                let base = DATA + 0x600;
                pool_fixture(c, base, 0);
                let blk = base + 0x80;
                c.bus.write32(blk, 0x50 + seed % 2);
                c.bus.write32(blk + 0x10, [0u32, 1][seed as usize / 2 % 2]);
                c.bus.write32(blk + 0xc, [0u32, 1][seed as usize / 4 % 2]);
                c.bus
                    .write32(blk + 0x18, if seed / 8 % 3 == 0 { 0 } else { 0x00c0_0000 });
                c.bus.write32(0x0045_0c18, 0);
                c.g[A0][0] = blk as u64;
                c.pc = 0x003d_f028;
            },
        );
    }

    driven!(gen_async_file_on_close, ee::s14::f_003df0b8__generated);
    driven!(hand_async_file_on_close, async_file_on_close);

    /// Null and live release results; finish runs on the block.
    #[test]
    fn async_file_on_close_matches() {
        check(
            gen_async_file_on_close,
            hand_async_file_on_close,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_f0b8, 0);
                ea_install_queue_fakes();
                let base = DATA + 0x600;
                pool_fixture(c, base, 0);
                let blk = base + 0x80;
                c.bus.write32(blk, 0x60 + seed % 2);
                c.bus.write32(blk + 0x10, [0u32, 1][seed as usize / 2 % 2]);
                c.bus.write32(blk + 0xc, [0u32, 1][seed as usize / 4 % 2]);
                c.bus.write32(blk + 0x18, 0);
                c.bus.write32(blk + 0x1c, 0x70 + seed % 2);
                c.bus.write32(
                    EAF_RET_TYPE,
                    if seed / 8 % 2 == 0 { 0 } else { 0x80 + seed },
                );
                c.g[A0][0] = 0;
                c.g[A1][0] = 0;
                c.g[A2][0] = blk as u64;
                c.pc = 0x003d_f0b8;
            },
        );
    }

    driven!(gen_async_file_on_read_chunk, ee::s14::f_003df0f0__generated);
    driven!(hand_async_file_on_read_chunk, async_file_on_read_chunk);

    /// Short and long chunks; clear and set flags; null and live queue
    /// results and callbacks.
    #[test]
    fn async_file_on_read_chunk_matches() {
        check(
            gen_async_file_on_read_chunk,
            hand_async_file_on_read_chunk,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_f0f0, 0);
                ea_install_queue_fakes();
                let base = DATA + 0x600;
                pool_fixture(c, base, 0);
                let blk = base + 0x80;
                c.bus.write32(blk, 0x70 + seed % 2);
                c.bus.write32(blk + 0x1c, 0x80 + seed % 2);
                c.bus.write32(blk + 0x2c, 0x100 * (seed % 3) as u32);
                c.bus.write32(blk + 0x8, 0x200 * (seed / 3 % 3) as u32);
                c.bus.write32(blk + 0x10, [0u32, 1][seed as usize / 9 % 2]);
                c.bus.write32(blk + 0x20, 0x300 + seed % 5);
                c.bus.write32(blk + 0x24, 0x400 + seed % 7);
                c.bus.write32(blk + 0x28, 0x500 + seed % 3);
                c.bus.write32(blk + 0x14, 0x600 + seed % 3);
                c.bus.write32(
                    EAF_RET_TYPE,
                    if seed / 27 % 3 == 0 { 0 } else { 0x90 + seed },
                );
                c.bus.write32(EAF_RET_SETCB, (seed / 81 % 2) as u32);
                c.g[A0][0] = 0;
                c.g[A1][0] = [0u32, 0x100, 0x8000, 0x1_0000][seed as usize / 162 % 4] as u64;
                c.g[A2][0] = blk as u64;
                c.pc = 0x003d_f0f0;
            },
        );
    }

    driven!(gen_async_file_on_size, ee::s14::f_003df1e0__generated);
    driven!(hand_async_file_on_size, async_file_on_size);

    /// Empty and live blocks; close and read continuations; null and
    /// live results and callbacks.
    #[test]
    fn async_file_on_size_matches() {
        check(
            gen_async_file_on_size,
            hand_async_file_on_size,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_f1e0, 0);
                ea_install_queue_fakes();
                let base = DATA + 0x600;
                pool_fixture(c, base, 0);
                let blk = base + 0x80;
                c.bus.write32(blk, 0x80 + seed % 2);
                c.bus.write32(blk + 0x1c, 0x90 + seed % 2);
                c.bus.write32(blk + 0x10, [0u32, 1][seed as usize / 2 % 2]);
                c.bus.write32(blk + 0x20, 0x300 + seed % 5);
                c.bus.write32(blk + 0x24, 0x400 + seed % 7);
                c.bus.write32(blk + 0x28, 0x100 * (seed / 4 % 3) as u32);
                c.bus.write32(blk + 0x14, 0x600 + seed % 3);
                c.bus.write32(0x0045_0c14, 0);
                c.bus.write32(
                    EAF_RET_TYPE,
                    if seed / 12 % 3 == 0 { 0 } else { 0xa0 + seed },
                );
                c.bus.write32(EAF_RET_SETCB, (seed / 36 % 2) as u32);
                c.g[A0][0] = 0;
                c.g[A1][0] = (seed / 72 % 3) as u64;
                c.g[A2][0] = blk as u64;
                c.pc = 0x003d_f1e0;
            },
        );
    }

    driven!(gen_async_file_on_open, ee::s14::f_003df2c0__generated);
    driven!(hand_async_file_on_open, async_file_on_open);

    /// Failed and live opens; empty and live blocks; chained and
    /// unchained; null and live results and callbacks.
    #[test]
    fn async_file_on_open_matches() {
        check(
            gen_async_file_on_open,
            hand_async_file_on_open,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_f2c0, 0);
                ea_install_queue_fakes();
                let base = DATA + 0x600;
                pool_fixture(c, base, 0);
                let blk = base + 0x80;
                c.bus.write32(blk, 0x90 + seed % 2);
                c.bus.write32(blk + 0x1c, 0xa0 + seed % 2);
                c.bus.write32(blk + 0x10, [0u32, 1][seed as usize / 2 % 2]);
                c.bus.write32(blk + 0x14, [0u32, 1][seed as usize / 4 % 2]);
                c.bus.write32(blk + 0x20, 0x300 + seed % 5);
                c.bus.write32(blk + 0x24, 0x400 + seed % 7);
                c.bus.write32(blk + 0x28, 0x500 + seed % 3);
                c.bus.write32(blk + 0x2c, 0x600 + seed % 3);
                c.bus.write32(
                    EAF_RET_TYPE,
                    if seed / 8 % 2 == 0 { 0 } else { 0xb0 + seed },
                );
                c.bus.write32(EAF_RET_SETCB, (seed / 16 % 2) as u32);
                c.g[A0][0] = 0;
                c.g[A1][0] = 0;
                c.g[A2][0] = blk as u64;
                c.pc = 0x003d_f2c0;
            },
        );
    }
}
