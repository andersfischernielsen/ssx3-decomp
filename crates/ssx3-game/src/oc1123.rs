//! Hand-written replacements for `ealib/seg_2DCAE8` AsyncFile/StreamFile calls
//! (wave `wave-ealib-1.txt` rows 61-80), each with its differential test
//! against the generated body. Moved into the source-tree units by
//! `tools/reorg_modules.py apply oc1123`.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003d_f3c0,
        async_file_on_read_range_chunk,
        "AsyncFile_OnReadRangeChunk",
    ),
    (0x003d_f488, async_file_init, "AsyncFile_Init"),
    (0x003d_f570, async_file_shutdown, "AsyncFile_Shutdown"),
    (0x003d_f690, async_file_load, "AsyncFile_Load"),
    (0x003d_f748, async_file_load_into, "AsyncFile_LoadInto"),
    (0x003d_f808, async_file_read_range, "AsyncFile_ReadRange"),
    (
        0x003d_f8e8,
        async_file_set_callback,
        "AsyncFile_SetCallback",
    ),
    (0x003d_f9d8, async_file_get_result, "AsyncFile_GetResult"),
    (0x003d_faf0, async_file_cancel, "AsyncFile_Cancel"),
    (
        0x003d_fdd0,
        stream_file_lookup_request,
        "StreamFile_LookupRequest",
    ),
    (
        0x003e_06b0,
        stream_file_calc_mem_size,
        "StreamFile_CalcMemSize",
    ),
    (0x003e_06d8, stream_file_create, "StreamFile_Create"),
    (
        0x003e_0948,
        stream_file_set_channel_filter,
        "StreamFile_SetChannelFilter",
    ),
    (0x003e_0a28, stream_file_destroy, "StreamFile_Destroy"),
    (
        0x003e_0ad8,
        stream_file_set_priority,
        "StreamFile_SetPriority",
    ),
    (
        0x003e_0b28,
        stream_file_set_low_water,
        "StreamFile_SetLowWater",
    ),
    (
        0x003e_0b98,
        stream_file_set_starved,
        "StreamFile_SetStarved",
    ),
    (
        0x003e_0bf8,
        stream_file_get_channel,
        "StreamFile_GetChannel",
    ),
    (
        0x003e_0c58,
        stream_file_get_channel_id,
        "StreamFile_GetChannelId",
    ),
    (0x003e_0d80, stream_file_queue_file, "StreamFile_QueueFile"),
];

mod callee {
    use super::ee;
    use crate::call::Callee;

    pub(crate) const ASYNC_FILE_ASSIGN_SERIAL: Callee = Callee {
        addr: 0x003d_ef40,
        f: ee::s14::AsyncFile_AssignSerial,
    };
    pub(crate) const ASYNC_FILE_FROM_HANDLE: Callee = Callee {
        addr: 0x003d_ef70,
        f: ee::s14::AsyncFile_FromHandle,
    };
    pub(crate) const ASYNC_FILE_FREE: Callee = Callee {
        addr: 0x003d_efc0,
        f: ee::s14::AsyncFile_Free,
    };
    pub(crate) const ASYNC_FILE_FINISH: Callee = Callee {
        addr: 0x003d_f028,
        f: ee::s14::AsyncFile_Finish,
    };
    pub(crate) const ASYNC_FILE_POOL_POP: Callee = Callee {
        addr: 0x003d_eed8,
        f: ee::s14::AsyncFile_PoolPop,
    };
    pub(crate) const ASYNC_FILE_CANCEL: Callee = Callee {
        addr: 0x003d_faf0,
        f: ee::s14::AsyncFile_Cancel,
    };
    pub(crate) const STREAM_FILE_VALIDATE: Callee = Callee {
        addr: 0x003d_fbd0,
        f: ee::s14::StreamFile_Validate,
    };
    pub(crate) const STREAM_FILE_CANCEL_ALL: Callee = Callee {
        addr: 0x003e_1110,
        f: ee::s14::StreamFile_CancelAll,
    };
    pub(crate) const STREAM_FILE_ALLOC_REQUEST: Callee = Callee {
        addr: 0x003d_fcd8,
        f: ee::s14::StreamFile_AllocRequest,
    };
    pub(crate) const STREAM_FILE_ENQUEUE_REQUEST: Callee = Callee {
        addr: 0x003d_fd58,
        f: ee::s14::StreamFile_EnqueueRequest,
    };
    pub(crate) const STREAM_FILE_START_NEXT_REQUEST: Callee = Callee {
        addr: 0x003e_0270,
        f: ee::s14::StreamFile_StartNextRequest,
    };
    /// Intra-module call: the generated body calls the slot target, so both
    /// sides run the installed replacement.
    pub(crate) const STREAM_FILE_CALC_MEM_SIZE: Callee = Callee {
        addr: 0x003e_06b0,
        f: ee::s14::StreamFile_CalcMemSize,
    };
    /// Intra-module call: the generated body calls the raw body.
    pub(crate) const STREAM_FILE_SET_STARVED: Callee = Callee {
        addr: 0x003e_0b98,
        f: ee::s14::StreamFile_SetStarved,
    };
    pub(crate) const EA_FILE_QUEUE_OPEN: Callee = Callee {
        addr: 0x003d_d4e8,
        f: ee::s14::EAFile_QueueOpen,
    };
    pub(crate) const EA_FILE_QUEUE_READ: Callee = Callee {
        addr: 0x003d_d648,
        f: ee::s14::EAFile_QueueRead,
    };
    pub(crate) const EA_FILE_SET_REQUEST_CALLBACK: Callee = Callee {
        addr: 0x003d_cf70,
        f: ee::s14::EAFile_SetRequestCallback,
    };
    pub(crate) const EA_FILE_WAIT_REQUEST: Callee = Callee {
        addr: 0x003d_d1d8,
        f: ee::s14::EAFile_WaitRequest,
    };
    pub(crate) const EA_FILE_CANCEL_REQUEST: Callee = Callee {
        addr: 0x003d_dd48,
        f: ee::s14::EAFile_CancelRequest,
    };
    pub(crate) const EA_FILE_RELEASE_REQUEST: Callee = Callee {
        addr: 0x003d_d310,
        f: ee::s14::EAFile_ReleaseRequest,
    };
    pub(crate) const EA_FILE_CLOSE: Callee = Callee {
        addr: 0x003d_eca0,
        f: ee::s14::EAFile_Close,
    };
    pub(crate) const EA_FILE_SET_REQUEST_PRIORITY: Callee = Callee {
        addr: 0x003d_cfc0,
        f: ee::s14::EAFile_SetRequestPriority,
    };
    pub(crate) const EA_MUTEX_LOCK: Callee = Callee {
        addr: 0x003e_5700,
        f: ee::s14::EAMutex_Lock,
    };
    pub(crate) const EA_MUTEX_INIT: Callee = Callee {
        addr: 0x003e_5698,
        f: ee::s14::EAMutex_Init,
    };
    pub(crate) const EA_MUTEX_UNLOCK: Callee = Callee {
        addr: 0x003e_5760,
        f: ee::s14::EAMutex_Unlock,
    };
    pub(crate) const EA_MUTEX_DESTROY: Callee = Callee {
        addr: 0x003e_56e0,
        f: ee::s14::EAMutex_Destroy,
    };
    pub(crate) const EA_MEM_ZERO: Callee = Callee {
        addr: 0x003e_6220,
        f: ee::s14::EA_MemZero,
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
}

/// `void AsyncFile_OnReadRangeChunk(block)` (0x3df3c0): releases the read
/// request and queues the next range chunk, or finishes short chunks.
pub fn async_file_on_read_range_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[RA][0] = sx(0x003d_f3dc);
        callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_f3dc)?;
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
        // Short chunks and live-flagged long chunks both finish the block;
        // only cleared long chunks queue the next range read.
        let mut finished = short;
        if !short {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            if c.g[V0][0] != 0 {
                finished = true;
            } else {
                c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
            }
        }
        if finished {
            c.cycles += 4;
            c.g[RA][0] = sx(0x003d_f418);
            c.g[A0][0] = c.g[S0][0];
            callee::ASYNC_FILE_FINISH.call(c, 0x003d_f418)?;
            frame.restore(c, S0, 0x10);
        } else {
            // Long chunk, cleared flag: queue the next range read.
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
            c.g[RA][0] = sx(0x003d_f454);
            c.g[T1][0] = c.g[S0][0];
            callee::EA_FILE_QUEUE_READ.call(c, 0x003d_f454)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 5;
                c.g[A1][0] = 0x3e0000;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[RA][0] = sx(0x003d_f474);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xc40));
                callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_f474)?;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 AsyncFile_Init(pool_words, handle_count)` (0x3df488): zeroes the
/// async pool and mutex when the settings allow handles at all.
pub fn async_file_init(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003d_f520;
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x450000;
        frame.save(c, S1, 0x20);
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc34));
        c.g[S2][0] = c.g[V0][0];
        frame.save(c, S0, 0x30);
        let initialized = c.g[V1][0] != 0;
        frame.save_ra(c, 0);
        if !initialized {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 257) as u64;
            let skip = c.g[V0][0] == 0;
            c.g[S0][0] = 0x30;
            if !skip {
                c.cycles += 21;
                c.g[V1][0] = 0x450000;
                c.mult(0, 16, 17, 16);
                c.g[A1][0] = 0x520000;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc14));
                c.g[A0][0] = 0x490000;
                c.sw((c.g[A1][0] as u32).wrapping_sub(0x6410), c.g[S1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5db8));
                let alloc = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003d_f4e4);
                c.g[A1][0] = c.g[S0][0];
                c.pc = alloc;
                call_ptr(c, alloc, 0x003d_f4e4)?;
                c.g[A1][0] = 0x520000;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x30));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6408));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                c.g[A0][0] = 0x520000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x63f8));
                c.sw((c.g[A1][0] as u32).wrapping_sub(0x6408), c.g[V0][0]);
                c.g[RA][0] = sx(0x003d_f50c);
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc34), c.g[V0][0]);
                callee::EA_MUTEX_INIT.call(c, 0x003d_f50c)?;
                let no_handles = (c.g[S1][0] as i64) <= 0;
                c.g[A0][0] = 0;
                if !no_handles {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc34));
                    c.g[V1][0] = c.g[V0][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
                    'l_3df520: loop {
                        c.cycles += 8;
                        c.sw(c.g[V1][0] as u32, c.g[A0][0]);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), 0);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x30));
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, HEAD);
                            }
                            continue 'l_3df520;
                        }
                        break;
                    }
                }
                c.cycles += 5;
                c.g[V0][0] = 0x30;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc34));
                c.mult(0, 2, 17, 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_sub(0x2c), 0);
            }
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

/// `void AsyncFile_Shutdown()` (0x3df570): cancels every async block,
/// waits for the pool to drain and tears the pool down. Only the
/// already-idle early exit is unit-testable (the drain loop needs another
/// thread to make progress); the rest is covered by the replay gates.
pub fn async_file_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_CANCEL: u32 = 0x003d_f5b8;
        const HEAD_DRAIN: u32 = 0x003d_f5e8;
        const HEAD_INNER: u32 = 0x003d_f600;
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        c.g[V1][0] = 0x450000;
        frame.save(c, S3, 0x20);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc34));
        c.g[S3][0] = c.g[V1][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        let idle = c.g[V0][0] == 0;
        if !idle {
            c.cycles += 5;
            c.g[V1][0] = 0x520000;
            c.g[S0][0] = 0;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6410));
            let empty = (c.g[V0][0] as i64) <= 0;
            c.g[S2][0] = c.g[V1][0];
            if !empty {
                c.cycles += 2;
                c.g[S1][0] = 0;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc34));
                'l_3df5b8: loop {
                    c.cycles += 9;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[RA][0] = sx(0x003d_f5cc);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x30));
                    callee::ASYNC_FILE_CANCEL.call(c, 0x003d_f5cc)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x6410));
                    c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let more = c.g[V1][0] != 0;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc34));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, HEAD_CANCEL);
                        }
                        continue 'l_3df5b8;
                    }
                    break;
                }
            }
            c.cycles += 3;
            c.g[S4][0] = 0x520000;
            c.g[S1][0] = 0x450000;
            c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x6410));
            loop {
                c.cycles += 2;
                let drained = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = 0;
                if drained {
                    break;
                }
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc34));
                c.g[A1][0] = 0x1;
                c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x6410));
                'l_3df600: loop {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                    let more = c.g[S0][0] != 0;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = c.g[V1][0];
                    }
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, HEAD_INNER);
                        }
                        continue 'l_3df600;
                    }
                    break;
                }
                c.cycles += 2;
                let waiting = c.g[A0][0] == 0;
                c.g[V1][0] = 0x450000;
                if !waiting {
                    break;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003d_f62c);
                c.g[A0][0] = 0;
                callee::EA_THREAD_IS_CURRENT.call(c, 0x003d_f62c)?;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x003d_f63c);
                c.g[A0][0] = 0;
                callee::EA_TIMER_SERVICE_CALLBACKS.call(c, 0x003d_f63c)?;
                c.cycles += 4;
                c.g[RA][0] = sx(0x003d_f644);
                c.g[A0][0] = 0;
                callee::EA_THREAD_YIELD.call(c, 0x003d_f644)?;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x6410));
                if event_due(c) {
                    deliver_event_here(c, HEAD_DRAIN);
                }
            }
            c.cycles += 8;
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc10));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            let free_cb = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003d_f65c);
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc34));
            c.pc = free_cb;
            call_ptr(c, free_cb, 0x003d_f65c)?;
            c.g[A0][0] = 0x520000;
            c.g[RA][0] = sx(0x003d_f668);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x63f8));
            callee::EA_MUTEX_DESTROY.call(c, 0x003d_f668)?;
            c.sw((c.g[S3][0] as u32).wrapping_add(0xc34), 0);
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 AsyncFile_Load(path, size)` (0x3df690): pops a pool block,
/// assigns it and queues an open; 0 when the pool is empty.
pub fn async_file_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[A0][0] = 0x520000;
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x003d_f6b8);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6408));
        callee::ASYNC_FILE_POOL_POP.call(c, 0x003d_f6b8)?;
        c.g[S0][0] = c.g[V0][0];
        let empty = c.g[S0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 20;
            c.g[RA][0] = sx(0x003d_f6cc);
            c.g[A0][0] = c.g[S0][0];
            callee::ASYNC_FILE_ASSIGN_SERIAL.call(c, 0x003d_f6cc)?;
            c.g[V0][0] = 0x1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S1][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0);
            c.g[A0][0] = c.g[S2][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0);
            c.g[A1][0] = 0x1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0);
            c.g[A2][0] = 0x64;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0);
            c.g[A3][0] = c.g[S0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0);
            c.g[RA][0] = sx(0x003d_f704);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            callee::EA_FILE_QUEUE_OPEN.call(c, 0x003d_f704)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
            let queued = c.g[V0][0] != 0;
            c.g[A1][0] = 0x3e0000;
            if queued {
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[RA][0] = sx(0x003d_f720);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd40));
                callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_f720)?;
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
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

/// `u32 AsyncFile_LoadInto(path, size, result)` (0x3df748): like Load
/// but records the caller's result word; 0 when the pool is empty.
pub fn async_file_load_into(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[A0][0] = 0x520000;
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        c.g[RA][0] = sx(0x003d_f778);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6408));
        callee::ASYNC_FILE_POOL_POP.call(c, 0x003d_f778)?;
        c.g[S0][0] = c.g[V0][0];
        let empty = c.g[S0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 19;
            c.g[RA][0] = sx(0x003d_f78c);
            c.g[A0][0] = c.g[S0][0];
            callee::ASYNC_FILE_ASSIGN_SERIAL.call(c, 0x003d_f78c)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S1][0]);
            c.g[A0][0] = c.g[S3][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[S2][0]);
            c.g[A1][0] = 0x1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0);
            c.g[A2][0] = 0x64;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0);
            c.g[A3][0] = c.g[S0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0);
            c.g[RA][0] = sx(0x003d_f7c0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0);
            callee::EA_FILE_QUEUE_OPEN.call(c, 0x003d_f7c0)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
            let queued = c.g[V0][0] != 0;
            c.g[A1][0] = 0x3e0000;
            if queued {
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[RA][0] = sx(0x003d_f7dc);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd40));
                callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_f7dc)?;
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 AsyncFile_ReadRange(handle, size, offset, length)` (0x3df808):
/// pops a block, assigns it and queues a range read; 0 when empty.
pub fn async_file_read_range(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        c.g[A0][0] = 0x520000;
        frame.save(c, S0, 0x50);
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A3][0];
        c.g[RA][0] = sx(0x003d_f840);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6408));
        callee::ASYNC_FILE_POOL_POP.call(c, 0x003d_f840)?;
        c.g[S0][0] = c.g[V0][0];
        let empty = c.g[S0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 24;
            c.g[RA][0] = sx(0x003d_f854);
            c.g[A0][0] = c.g[S0][0];
            callee::ASYNC_FILE_ASSIGN_SERIAL.call(c, 0x003d_f854)?;
            c.g[A3][0] = 0x8000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0);
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[S1][0] as i64)) as u64;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0);
            if c.g[V0][0] == 0 {
                c.g[A3][0] = c.g[S1][0];
            }
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
            c.g[A0][0] = c.g[S2][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S2][0]);
            c.g[A1][0] = c.g[S3][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0);
            c.g[A2][0] = c.g[S4][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[S3][0]);
            c.g[T0][0] = 0x64;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S1][0]);
            c.g[T1][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x003d_f89c);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[S4][0]);
            callee::EA_FILE_QUEUE_READ.call(c, 0x003d_f89c)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
            let queued = c.g[V0][0] != 0;
            c.g[A1][0] = 0x3e0000;
            if queued {
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[RA][0] = sx(0x003d_f8b8);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xc40));
                callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_f8b8)?;
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AsyncFile_SetCallback(handle, kind, fn)` (0x3df8e8): stores a
/// completion callback on a live handle under the pool mutex.
pub fn async_file_set_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_f910);
        c.g[S4][0] = c.g[A1][0];
        callee::ASYNC_FILE_FROM_HANDLE.call(c, 0x003d_f910)?;
        c.g[S2][0] = c.g[V0][0];
        let missing = c.g[S2][0] == 0;
        frame.restore(c, S0, 0x50);
        // The live-callback path jumps straight to the epilogue; the
        // others share the +1/S1-restore tail.
        let mut direct = false;
        if !missing {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
            if c.g[V0][0] != 0 {
                frame.restore(c, S1, 0x40);
                direct = true;
            } else {
                c.cycles += 11;
                c.g[S0][0] = 0x520000;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x63f8));
                c.g[RA][0] = sx(0x003d_f938);
                c.g[A0][0] = c.g[S0][0];
                callee::EA_MUTEX_LOCK.call(c, 0x003d_f938)?;
                c.g[S1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                c.g[A0][0] = c.g[S0][0];
                c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[S4][0]);
                c.g[RA][0] = sx(0x003d_f94c);
                c.g[S1][0] = (c.g[S1][0] < 0x1) as u64;
                callee::EA_MUTEX_UNLOCK.call(c, 0x003d_f94c)?;
                let armed = c.g[S1][0] != 0;
                frame.restore(c, S0, 0x50);
                if armed {
                    c.cycles += 3;
                    let user_cb = c.g[S4][0] as u32;
                    c.g[RA][0] = sx(0x003d_f95c);
                    c.g[A0][0] = c.g[S3][0];
                    c.pc = user_cb;
                    call_ptr(c, user_cb, 0x003d_f95c)?;
                    frame.restore(c, S0, 0x50);
                }
            }
        }
        if !direct {
            c.cycles += 1;
            frame.restore(c, S1, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AsyncFile_GetResult(handle, ...)` (0x3df9d8): waits for a
/// queued request and reports its outcome.
pub fn async_file_get_result(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = 0x520000;
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_fa0c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x63f8));
        callee::EA_MUTEX_LOCK.call(c, 0x003d_fa0c)?;
        c.g[S4][0] = 0;
        c.g[RA][0] = sx(0x003d_fa18);
        c.g[A0][0] = c.g[S1][0];
        callee::ASYNC_FILE_FROM_HANDLE.call(c, 0x003d_fa18)?;
        c.g[S1][0] = c.g[V0][0];
        // Fast classification: missing handle or live block returns now.
        if c.g[S1][0] != 0 {
            c.cycles += 3;
            c.g[S4][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = 0x1;
            if c.g[S4][0] == 0 {
                c.cycles += 1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            }
        }
        'b_3dfad4: {
            'b_3dfad0: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003d_fa3c);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x63f8));
                callee::EA_MUTEX_UNLOCK.call(c, 0x003d_fa3c)?;
                c.g[V0][0] = 0xffffffffffffffff;
                if c.g[S1][0] == 0 {
                    break 'b_3dfad0;
                }
                c.cycles += 2;
                if c.g[S4][0] != 0 {
                    frame.restore(c, S0, 0x50);
                    break 'b_3dfad4;
                }
                // The request is live: wait for its word, then report.
                const HEAD: u32 = 0x003d_fa58;
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                } else {
                    loop {
                        c.cycles += 6;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        c.g[RA][0] = sx(0x003d_fa64);
                        callee::EA_FILE_WAIT_REQUEST.call(c, 0x003d_fa64)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        if c.g[V1][0] == 0 {
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, HEAD);
                        }
                    }
                }
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    if c.g[S2][0] != 0 {
                        c.sw(c.g[S2][0] as u32, 0);
                    }
                    c.cycles += 2;
                    if c.g[S3][0] != 0 {
                        c.sw(c.g[S3][0] as u32, 0);
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003d_fa94);
                    c.g[A0][0] = c.g[S1][0];
                    callee::ASYNC_FILE_FREE.call(c, 0x003d_fa94)?;
                    c.g[V0][0] = 0xffffffffffffffff;
                    break 'b_3dfad0;
                }
                c.cycles += 2;
                let has_out = c.g[S2][0] != 0;
                if has_out {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                }
                c.cycles += 2;
                let has_size = c.g[S3][0] != 0;
                if has_size {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw(c.g[S3][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[RA][0] = sx(0x003d_facc);
                c.g[A0][0] = c.g[S1][0];
                callee::ASYNC_FILE_FREE.call(c, 0x003d_facc)?;
                c.g[V0][0] = 0x1;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 AsyncFile_Cancel(handle)` (0x3dfaf0): cancels a queued request;
/// 1 if the handle was live, the block itself if it was stale with a
/// result pending, 0xffffffff when unknown.
pub fn async_file_cancel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = 0x520000;
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x63f8));
        frame.save(c, S4, 0x10);
        c.g[S3][0] = 0;
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_fb24);
        c.g[S4][0] = 0;
        callee::EA_MUTEX_LOCK.call(c, 0x003d_fb24)?;
        c.g[S2][0] = 0;
        c.g[RA][0] = sx(0x003d_fb30);
        c.g[A0][0] = c.g[S0][0];
        callee::ASYNC_FILE_FROM_HANDLE.call(c, 0x003d_fb30)?;
        c.g[S0][0] = c.g[V0][0];
        // Fast path out: unknown handle keeps V0 zero; a live block is
        // marked done; a stale block with no result pending is freed.
        let mut done = c.g[S0][0] == 0;
        if !done {
            c.cycles += 4;
            c.g[S3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
            c.g[S4][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            let live = c.g[S3][0] != 0;
            c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            if !live {
                c.cycles += 2;
                done = c.g[S2][0] != 0;
            }
        }
        if !done {
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        }
        c.cycles += 4;
        c.g[RA][0] = sx(0x003d_fb64);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x63f8));
        callee::EA_MUTEX_UNLOCK.call(c, 0x003d_fb64)?;
        // The live-done path jumps straight to the epilogue; every other
        // path runs the +1/S0-restore tail first.
        let mut skip_tail = false;
        if c.g[S0][0] == 0 {
            c.g[V0][0] = 0xffffffffffffffff;
        } else {
            c.cycles += 2;
            if c.g[S4][0] != 0 {
                frame.restore(c, S0, 0x50);
                c.g[V0][0] = 0xffffffffffffffff;
                skip_tail = true;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                let finished = c.g[V0][0] == 0;
                c.g[V0][0] = 0xffffffffffffffff;
                if !finished {
                    c.cycles += 2;
                    let queued = c.g[S3][0] == 0;
                    if !queued {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x003d_fb90);
                        c.g[A0][0] = c.g[S3][0];
                        callee::EA_FILE_CANCEL_REQUEST.call(c, 0x003d_fb90)?;
                        c.g[V0][0] = 0x1;
                    } else {
                        c.cycles += 2;
                        let done = c.g[S2][0] != 0;
                        c.g[V0][0] = 0x1;
                        if !done {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x003d_fba8);
                            c.g[A0][0] = c.g[S0][0];
                            callee::ASYNC_FILE_FREE.call(c, 0x003d_fba8)?;
                            c.g[V0][0] = 0x1;
                        }
                    }
                }
            }
        }
        if !skip_tail {
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_LookupRequest(stream, handle)` (0x3dfdd0): finds the
/// queued request for `handle`, or 0 when absent or out of range.
pub fn stream_file_lookup_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
        c.g[V1][0] = c.g[A1][0] & 0xff;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[V0][0] = 0x124;
        if in_range {
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
            c.mult(0, 2, 3, 2);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            if c.g[A1][0] != c.g[V1][0] {
                c.cycles += 2;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = 0;
                }
            }
            c.pc = c.g[RA][0] as u32;
            return Ok(());
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
        Ok(())
    })
}

/// `u32 StreamFile_CalcMemSize(channels, buffer, sample_count)`
/// (0x3e06b0): sizes a stream buffer in bytes.
pub fn stream_file_calc_mem_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[V0][0] = 0xc;
        c.g[V1][0] = 0x124;
        c.mult(0, 5, 5, 2);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 4);
        c.mult(0, 2, 4, 3);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x180));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x40));
        // Tail call: returns directly to the caller's return address.
        c.pc = c.g[RA][0] as u32;
        Ok(())
    })
}

/// `u32 StreamFile_SetChannelFilter(stream, channel, filter)`
/// (0x3e0948): installs a per-channel filter; nonzero when the channel
/// index is out of range or already has one.
pub fn stream_file_set_channel_filter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[T0][0];
        frame.save_ra(c, 0x10);
        c.g[S3][0] = c.g[A3][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e097c);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e097c)?;
        // Every early exit below restores S0 and jumps to the epilogue;
        // only the fallthrough runs the shared +1 tail.
        let mut done = false;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x50);
            done = true;
        } else {
            c.cycles += 2;
            let empty = (c.g[S1][0] as i64) <= 0;
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            // An empty table skips straight to the shared tail; the
            // match/mismatch arms below always run the scan.
            let mut scanned = empty;
            if !empty {
                c.cycles += 5;
                c.g[A0][0] = c.g[V1][0];
                c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    // Channel table full: report the end of the table.
                    frame.restore(c, S0, 0x50);
                    done = true;
                    scanned = true;
                } else {
                    c.cycles += 2;
                    let mismatch = c.g[S1][0] != c.g[V1][0];
                    c.g[V0][0] = c.g[S2][0] | c.g[S3][0];
                    if !mismatch {
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            // A filter is already installed.
                            frame.restore(c, S0, 0x50);
                            done = true;
                        }
                        // A clean match falls through to the scan below.
                    }
                }
            }
            if !done && !scanned {
                // Scan the channel list for a matching entry.
                c.cycles += 2;
                if (c.g[S0][0] as i64) > 0 {
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0xffffffffffffffff;
                    let neg_one = c.g[S0][0] == c.g[V0][0];
                    c.g[V0][0] = 0xfffffffffffffffe;
                    if !neg_one {
                        c.cycles += 2;
                        if c.g[S0][0] != c.g[V0][0] {
                            frame.restore(c, S0, 0x50);
                            done = true;
                        }
                    }
                }
                if !done {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
                }
            }
            if !done && !scanned {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x50);
                    done = true;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x38));
                    if c.g[V0][0] != 0 {
                        frame.restore(c, S0, 0x50);
                        done = true;
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = 0xc;
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                        c.mult(0, 2, 17, 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                        c.sw(c.g[V1][0] as u32, c.g[S2][0]);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[S3][0]);
                    }
                }
            }
        }
        if !done {
            c.cycles += 1;
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

/// `u32 StreamFile_Destroy(handle)` (0x3e0a28): cancels all requests on
/// a stream and frees it; 0 unless the handle was missing.
pub fn stream_file_destroy(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003e_0a70;
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003e_0a44);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_0a44)?;
        // The invalid-stream path restores S0 and returns V0 directly.
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 7;
            c.g[RA][0] = sx(0x003e_0a54);
            c.g[A0][0] = c.g[S0][0];
            callee::STREAM_FILE_CANCEL_ALL.call(c, 0x003e_0a54)?;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            c.g[A0][0] = 0x1;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x38));
            if c.g[V1][0] != c.g[A0][0] {
                c.sw(c.g[V0][0] as u32, 0);
            } else {
                c.cycles += 2;
                c.g[S0][0] = 0x1;
                loop {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e_0a78);
                    c.g[A0][0] = 0;
                    callee::EA_THREAD_IS_CURRENT.call(c, 0x003e_0a78)?;
                    if c.g[V0][0] != 0 {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x003e_0a88);
                        c.g[A0][0] = 0;
                        callee::EA_TIMER_SERVICE_CALLBACKS.call(c, 0x003e_0a88)?;
                    }
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x003e_0a90);
                    c.g[A0][0] = 0;
                    callee::EA_THREAD_YIELD.call(c, 0x003e_0a90)?;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x38));
                    if c.g[V1][0] != c.g[S0][0] {
                        c.cycles += 1;
                        c.sw(c.g[V0][0] as u32, 0);
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, HEAD);
                    }
                }
            }
            c.cycles += 7;
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            c.g[RA][0] = sx(0x003e_0ab0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            callee::EA_MUTEX_DESTROY.call(c, 0x003e_0ab0)?;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x16c));
            if c.g[A0][0] == 0 {
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003e_0ac8);
                c.g[A1][0] = 0x64;
                callee::EA_FILE_CLOSE.call(c, 0x003e_0ac8)?;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_SetPriority(stream, pri, sub)` (0x3e0ad8): stores
/// the stream priority pair; 0 unless validation failed.
pub fn stream_file_set_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e0afc);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e0afc)?;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x3c), c.g[S0][0]);
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[S1][0]);
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_SetLowWater(stream, level)` (0x3e0b28): lowers the
/// starvation threshold and re-arms starvation when it rises.
pub fn stream_file_set_low_water(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e0b4c);
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e0b4c)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 9;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x44), c.g[S0][0]);
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
            c.g[A1][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
            let same = c.g[V0][0] == c.g[A1][0];
            frame.restore(c, S0, 0x30);
            if !same {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003e0b80);
                c.g[A0][0] = c.g[S1][0];
                callee::STREAM_FILE_SET_STARVED.call(c, 0x003e0b80)?;
                frame.restore(c, S1, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_SetStarved(stream, starved)` (0x3e0b98): latches
/// the starved flag and re-arms the file's request priority.
pub fn stream_file_set_starved(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e0bb4);
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e0bb4)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            let clear = c.g[S0][0] == 0;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x48), c.g[S0][0]);
            if clear {
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[V1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x38));
                if c.g[V0][0] != c.g[V1][0] {
                    frame.restore(c, S0, 0x20);
                } else {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x003e0be4);
                    c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x174));
                    callee::EA_FILE_SET_REQUEST_PRIORITY.call(c, 0x003e0be4)?;
                    frame.restore(c, S0, 0x20);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_GetChannel(stream, index)` (0x3e0bf8): returns the
/// channel id at `index`, or 0 when out of range.
pub fn stream_file_get_channel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_0c14);
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_0c14)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            c.cycles += 2;
            let empty = (c.g[S0][0] as i64) <= 0;
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            if !empty {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                let in_range = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[S0][0] as u32) << 4);
                if in_range {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_GetChannelId(stream)` (0x3e0c58): returns the id of
/// the stream's current channel, or 0 when invalid.
pub fn stream_file_get_channel_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_0c6c);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_0c6c)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            c.cycles += 2;
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_QueueFile(stream, ...)` (0x3e0d80): allocates and
/// enqueues a stream request, then kicks the queue; nonzero on success.
pub fn stream_file_queue_file(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003e_0de8;
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S3, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S4, 0x20);
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003e_0db4);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_0db4)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            c.cycles += 5;
            c.g[RA][0] = sx(0x003e_0dc4);
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            callee::STREAM_FILE_ALLOC_REQUEST.call(c, 0x003e_0dc4)?;
            c.g[S1][0] = c.g[V0][0];
            let queued = c.g[S1][0] == 0;
            c.g[V0][0] = 0;
            if !queued {
                c.cycles += 2;
                let has_offset = c.g[S0][0] != 0;
                c.g[S4][0] = 0x1;
                if !has_offset {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                    let direct = c.g[V0][0] == c.g[S2][0];
                    c.g[A0][0] = c.g[S3][0];
                    if !direct {
                        c.cycles += 1;
                        loop {
                            c.cycles += 7;
                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                            let more = c.g[V0][0] != c.g[S2][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            if more {
                                if event_due(c) {
                                    deliver_event_here(c, HEAD);
                                }
                                continue;
                            }
                            break;
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[S4][0] = 0x1;
                c.cycles += 14;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x118), c.g[S0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x114), c.g[S3][0]);
                c.g[A1][0] = c.g[S1][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x11c), c.g[S2][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[S4][0]);
                c.g[RA][0] = sx(0x003e_0e2c);
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                callee::STREAM_FILE_ENQUEUE_REQUEST.call(c, 0x003e_0e2c)?;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[RA][0] = sx(0x003e_0e38);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                callee::EA_MUTEX_LOCK.call(c, 0x003e_0e38)?;
                c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x38));
                if c.g[S0][0] == 0 {
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x38), c.g[S4][0]);
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[RA][0] = sx(0x003e_0e54);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                callee::EA_MUTEX_UNLOCK.call(c, 0x003e_0e54)?;
                if c.g[S0][0] == 0 {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[RA][0] = sx(0x003e_0e68);
                    c.g[A1][0] = 0;
                    callee::STREAM_FILE_START_NEXT_REQUEST.call(c, 0x003e_0e68)?;
                }
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_Create(handle, channels, mode, arg3, arg4, arg5)`
/// (0x3e06d8): validates the configuration and lays out the stream
/// block pool; 0 when any check fails, otherwise the entries base.
pub fn stream_file_create(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_878: u32 = 0x003e_0878;
        const HEAD_8D0: u32 = 0x003e_08d0;
        const HEAD_8F8: u32 = 0x003e_08f8;
        c.cycles += 18;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x10);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S4, 0x20);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_0710);
        c.g[S5][0] = c.g[T0][0];
        callee::STREAM_FILE_CALC_MEM_SIZE.call(c, 0x003e_0710)?;
        c.g[S4][0] = sx((c.g[S5][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[V0][0] = ((c.g[S4][0] as i64) < 6144) as u64;
        let small = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        let mut ok = !small;
        if ok {
            // Channel-count and range validation chain.
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 2) as u64;
            let big_enough = c.g[V0][0] == 0;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 257) as u64;
            if !big_enough {
                c.cycles += 2;
                ok = false;
            } else {
                c.cycles += 2;
                let too_big = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                if too_big {
                    c.cycles += 2;
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_072c);
                    }
                    ok = false;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x10) as u64;
                    let fits = c.g[V0][0] != 0;
                    c.g[V0][0] = 0;
                    if !fits {
                        ok = false;
                    } else {
                        c.cycles += 2;
                        let nonneg = (c.g[S3][0] as i64) > 0;
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                        if !nonneg {
                            c.cycles += 2;
                            if event_due(c) {
                                deliver_event_here(c, 0x003e_072c);
                            }
                            ok = false;
                        } else {
                            c.cycles += 2;
                            let ordered = c.g[V0][0] != 0;
                            c.g[V0][0] = 0;
                            if ordered {
                                ok = false;
                            }
                        }
                    }
                }
            }
        }
        if !ok {
            // Every validation failure exits with V0 zero.
            c.g[V0][0] = 0;
        }
        if ok {
            c.cycles += 48;
            c.g[V0][0] = 0x4d520000;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.g[V0][0] | 0x5453;
            c.g[RA][0] = sx(0x003e_076c);
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            callee::EA_MUTEX_INIT.call(c, 0x003e_076c)?;
            c.g[V0][0] = 0x124;
            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x180));
            c.mult(0, 3, 17, 2);
            c.g[T0][0] = 0xffffffffffffffc0;
            c.g[V0][0] = sx((c.g[S3][0] as u32) << 4);
            c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S5][0] as u32));
            c.g[T1][0] = 0x96;
            c.g[A3][0] = 0x32;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S1][0]);
            c.g[A1][0] = 0x100;
            c.g[T3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S2][0]);
            c.g[V1][0] = 0xc;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S3][0]);
            c.mult(0, 4, 18, 3);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x38), 0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x44), 0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0);
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T3][0] as u32));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), 0);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x60), 0);
            c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x64), 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x40));
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[V0][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[V0][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x50), c.g[V0][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x34), c.g[T2][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), c.g[T1][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x40), c.g[A3][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x68), c.g[A2][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[A2][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[T3][0]);
            c.g[RA][0] = sx(0x003e_080c);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
            callee::EA_MEM_ZERO.call(c, 0x003e_080c)?;
            // Entry-size ladder from the leftover budget.
            c.sw((c.g[S0][0] as u32).wrapping_add(0x16c), 0);
            c.g[V0][0] = ((c.g[S4][0] as i64) < 16384) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = 0x800;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x7fff;
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                let big = c.g[V0][0] != 0;
                c.g[V0][0] = 0xffff;
                if big {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    let big2 = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x10000;
                    if big2 {
                        c.cycles += 4;
                        c.g[V0][0] = c.g[V0][0] | 0xffff;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                        let big3 = c.g[V0][0] != 0;
                        c.g[V0][0] = 0x8000;
                        if !big3 {
                            c.cycles += 1;
                            c.g[V0][0] = 0x4000;
                        }
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = 0x2000;
                    }
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x1000;
                }
            }
            c.cycles += 3;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x17c), c.g[V0][0]);
            let empty_ch = (c.g[S1][0] as i64) <= 0;
            c.g[A1][0] = 0;
            if !empty_ch {
                c.cycles += 3;
                c.g[A3][0] = 0x124;
                c.g[A2][0] = 0;
                loop {
                    c.cycles += 12;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x124));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    if c.g[A0][0] == 0 {
                        break;
                    }
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x124));
                    if event_due(c) {
                        deliver_event_here(c, HEAD_878);
                    }
                }
            }
            c.cycles += 6;
            c.g[V0][0] = 0x124;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            c.mult(0, 2, 17, 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            let empty_buf = (c.g[S2][0] as i64) <= 0;
            c.sw((c.g[V0][0] as u32).wrapping_sub(0x118), 0);
            if !empty_buf {
                c.cycles += 4;
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = 0x1;
                c.g[V1][0] = 0;
                loop {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.sw(c.g[V0][0] as u32, 0);
                    let more = c.g[A1][0] != 0;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, HEAD_8D0);
                    }
                }
            }
            c.cycles += 2;
            let empty_sub = (c.g[S3][0] as i64) <= 0;
            c.g[A1][0] = 0;
            if !empty_sub {
                loop {
                    c.cycles += 10;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 4);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = c.g[A0][0];
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0);
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    c.sw(c.g[V1][0] as u32, c.g[S0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, HEAD_8F8);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
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

    /// Magic-gated fake control block and request/result cells.
    pub(crate) const EAF_CTRL: u32 = DATA + 0x2_0000;
    pub(crate) const EAF_MAGIC: u32 = 0xeaf0_0001;
    pub(crate) const EAF_REQ: u32 = DATA + 0x2000;
    pub(crate) const EAF_RET_TYPE: u32 = DATA + 0x2_0010;
    pub(crate) const EAF_RET_SETCB: u32 = DATA + 0x2_0024;
    pub(crate) const EAF_RET_FH: u32 = DATA + 0x2_0028;
    pub(crate) const EAF_RET_VALID: u32 = DATA + 0x2_002c;
    pub(crate) const EAF_RET_ALLOC: u32 = DATA + 0x2_0030;
    pub(crate) const EAF_RET_ENQ: u32 = DATA + 0x2_0034;
    pub(crate) const EAF_RET_NEXT: u32 = DATA + 0x2_0038;

    /// A zeroed async pool block with a valid header word.
    fn pool_block(c: &mut Cpu, base: u32) {
        for i in 0..16u32 {
            c.bus.write32(base + 4 * i, 0);
        }
        c.bus.write32(EAF_CTRL, EAF_MAGIC);
    }

    /// A zeroed stream block with selectable header fields.
    fn stream_block(c: &mut Cpu, base: u32) {
        for i in 0..32u32 {
            c.bus.write32(base + 4 * i, 0);
        }
        c.bus.write32(EAF_CTRL, EAF_MAGIC);
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
        ea_fake_set_request_callback,
        0,
        ee::s14::f_003dcf70__generated,
        EAF_RET_SETCB,
        0xeaf0_100e
    );
    ea_fake_ret!(
        ea_fake_from_handle,
        0,
        ee::s14::f_003def70__generated,
        EAF_RET_FH,
        0xeaf0_100f
    );
    ea_fake_ret!(
        ea_fake_stream_validate,
        0,
        ee::s14::f_003dfbd0__generated,
        EAF_RET_VALID,
        0xeaf0_1012
    );
    ea_fake_ret!(
        ea_fake_stream_alloc,
        0,
        ee::s14::f_003dfcd8__generated,
        EAF_RET_ALLOC,
        0xeaf0_1013
    );
    ea_fake_ret!(
        ea_fake_stream_enqueue,
        0,
        ee::s14::f_003dfd58__generated,
        EAF_RET_ENQ,
        0xeaf0_1014
    );
    ea_fake_ret!(
        ea_fake_stream_start_next,
        0,
        ee::s14::f_003e0270__generated,
        EAF_RET_NEXT,
        0xeaf0_1015
    );
    ea_fake_ret!(
        ea_fake_alloc_request,
        0,
        ee::s14::f_003de670__generated,
        EAF_RET_ALLOC,
        0xeaf0_1000
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

    pub(crate) fn ea_fake_mutex_lock(c: &mut Cpu) {
        // Unconditional: the real lock spins on fixture data.
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

    pub(crate) fn ea_install_oc1123_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DCF70),
            ea_fake_set_request_callback,
            fakes::prev_cell(ea_fake_set_request_callback),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DEF70),
            ea_fake_from_handle,
            fakes::prev_cell(ea_fake_from_handle),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DFBD0),
            ea_fake_stream_validate,
            fakes::prev_cell(ea_fake_stream_validate),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DFCD8),
            ea_fake_stream_alloc,
            fakes::prev_cell(ea_fake_stream_alloc),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DFD58),
            ea_fake_stream_enqueue,
            fakes::prev_cell(ea_fake_stream_enqueue),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E0270),
            ea_fake_stream_start_next,
            fakes::prev_cell(ea_fake_stream_start_next),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DE670),
            ea_fake_alloc_request,
            fakes::prev_cell(ea_fake_alloc_request),
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

    driven!(
        gen_async_file_on_read_range_chunk,
        ee::s14::f_003df3c0__generated
    );
    driven!(
        hand_async_file_on_read_range_chunk,
        async_file_on_read_range_chunk
    );

    /// Short and long chunks; clear and live flags; null and live queue
    /// results and callbacks.
    #[test]
    fn async_file_on_read_range_chunk_matches() {
        check(
            gen_async_file_on_read_range_chunk,
            hand_async_file_on_read_range_chunk,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_f3c0, 0);
                ea_install_oc1123_fakes();
                let base = DATA + 0x600;
                pool_block(c, base);
                let blk = base + 0x80;
                c.bus.write32(blk, 0x70 + seed % 2);
                c.bus.write32(blk + 0x1c, 0x80 + seed % 2);
                c.bus.write32(blk + 0x2c, 0x100 * (seed / 2 % 3) as u32);
                c.bus.write32(blk + 0x8, 0x200 * (seed / 6 % 3) as u32);
                c.bus.write32(blk + 0x10, [0u32, 1][seed as usize / 18 % 2]);
                c.bus.write32(blk + 0x20, 0x300 + seed % 5);
                c.bus.write32(blk + 0x24, 0x400 + seed % 7);
                c.bus.write32(blk + 0x14, 0x600 + seed % 3);
                c.bus.write32(
                    EAF_RET_TYPE,
                    if seed / 54 % 3 == 0 { 0 } else { 0x90 + seed },
                );
                c.bus.write32(EAF_RET_SETCB, (seed / 162 % 2) as u32);
                c.g[A0][0] = 0;
                c.g[A1][0] = [0u32, 0x100, 0x8000, 0x1_0000][seed as usize / 324 % 4] as u64;
                c.g[A2][0] = blk as u64;
                c.pc = 0x003d_f3c0;
            },
        );
    }

    driven!(gen_async_file_init, ee::s14::f_003df488__generated);
    driven!(hand_async_file_init, async_file_init);

    /// Uninitialized and initialized tables; small and large counts.
    #[test]
    fn async_file_init_matches() {
        check(gen_async_file_init, hand_async_file_init, 48, &|c, seed| {
            let _r = machine(c, seed, 0x3d_f488, 0);
            ea_install_oc1123_fakes();
            c.bus.write32(0x0045_0c34, (seed % 2) as u32);
            c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
            c.bus.write32(0x0051_9bf0, 0);
            c.g[A0][0] = [0u32, 1, 0x40, 0x200][seed as usize / 2 % 4] as u64;
            c.g[A1][0] = [0u32, 1, 5, 300][seed as usize / 8 % 4] as u64;
            c.pc = 0x003d_f488;
        });
    }

    driven!(gen_async_file_shutdown, ee::s14::f_003df570__generated);
    driven!(hand_async_file_shutdown, async_file_shutdown);

    /// The idle early exit (the drain loop needs another thread).
    #[test]
    fn async_file_shutdown_matches() {
        check(
            gen_async_file_shutdown,
            hand_async_file_shutdown,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_f570, 0);
                ea_install_oc1123_fakes();
                c.bus.write32(0x0045_0c34, 0);
                c.pc = 0x003d_f570;
            },
        );
    }

    driven!(gen_async_file_load, ee::s14::f_003df690__generated);
    driven!(hand_async_file_load, async_file_load);

    /// Empty and stocked pools; null and live queue results.
    #[test]
    fn async_file_load_matches() {
        check(gen_async_file_load, hand_async_file_load, 64, &|c, seed| {
            let _r = machine(c, seed, 0x3d_f690, 0);
            ea_install_oc1123_fakes();
            let base = DATA + 0x600;
            let node = base + 0x80;
            pool_block(c, base);
            let mode = seed % 3;
            c.bus.write32(0x0051_9bf8, if mode == 0 { 0 } else { node });
            c.bus.write32(node, 0x40 + seed % 4);
            c.bus.write32(node + 0x4, 0);
            c.bus
                .write32(EAF_RET_TYPE, if mode == 1 { EAF_REQ } else { 0 });
            c.bus.write32(EAF_RET_SETCB, (seed / 3 % 2) as u32);
            put_ascii(c, DATA + 0x400, "file.dat");
            c.g[A0][0] = sx(DATA + 0x400);
            c.g[A1][0] = (seed / 6 % 4) as u64;
            c.pc = 0x003d_f690;
        });
    }

    driven!(gen_async_file_load_into, ee::s14::f_003df748__generated);
    driven!(hand_async_file_load_into, async_file_load_into);

    /// Empty and stocked pools; null and live queue results.
    #[test]
    fn async_file_load_into_matches() {
        check(
            gen_async_file_load_into,
            hand_async_file_load_into,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_f748, 0);
                ea_install_oc1123_fakes();
                let base = DATA + 0x600;
                let node = base + 0x80;
                pool_block(c, base);
                let mode = seed % 3;
                c.bus.write32(0x0051_9bf8, if mode == 0 { 0 } else { node });
                c.bus.write32(node, 0x40 + seed % 4);
                c.bus.write32(node + 0x4, 0);
                c.bus
                    .write32(EAF_RET_TYPE, if mode == 1 { EAF_REQ } else { 0 });
                c.bus.write32(EAF_RET_SETCB, (seed / 3 % 2) as u32);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = (seed / 6 % 4) as u64;
                c.g[A3][0] = 0x1234 + seed as u64;
                c.pc = 0x003d_f748;
            },
        );
    }

    driven!(gen_async_file_read_range, ee::s14::f_003df808__generated);
    driven!(hand_async_file_read_range, async_file_read_range);

    /// Empty and stocked pools; short and long ranges; null and live
    /// queue results and callbacks.
    #[test]
    fn async_file_read_range_matches() {
        check(
            gen_async_file_read_range,
            hand_async_file_read_range,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_f808, 0);
                ea_install_oc1123_fakes();
                let base = DATA + 0x600;
                let node = base + 0x80;
                pool_block(c, base);
                let mode = seed % 2;
                c.bus.write32(0x0051_9bf8, if mode == 0 { 0 } else { node });
                c.bus.write32(node, 0x40 + seed % 4);
                c.bus.write32(node + 0x4, 0);
                c.bus
                    .write32(EAF_RET_TYPE, if mode == 1 { EAF_REQ } else { 0 });
                c.bus.write32(EAF_RET_SETCB, (seed / 2 % 2) as u32);
                c.g[A0][0] = EAF_REQ as u64;
                c.g[A1][0] = 0x3000 + seed as u64;
                c.g[A2][0] = 0x40 + (seed / 4 % 3) as u64 * 0x10;
                c.g[A3][0] = [0u32, 0x8000, 0x1_0000][seed as usize / 12 % 3] as u64;
                c.pc = 0x003d_f808;
            },
        );
    }

    driven!(gen_async_file_set_callback, ee::s14::f_003df8e8__generated);
    driven!(hand_async_file_set_callback, async_file_set_callback);

    /// Missing and live handles; cleared and set flags; null and live
    /// user callbacks.
    #[test]
    fn async_file_set_callback_matches() {
        check(
            gen_async_file_set_callback,
            hand_async_file_set_callback,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_f8e8, 0);
                ea_install_oc1123_fakes();
                let base = DATA + 0x600;
                pool_block(c, base);
                let blk = base + 0x80;
                let mode = seed % 4;
                c.bus.write32(EAF_RET_FH, if mode == 0 { 0 } else { blk });
                c.bus.write32(blk, 0x90 + seed % 2);
                c.bus.write32(blk + 0x18, [0u32, 1][seed as usize / 4 % 2]);
                c.bus.write32(blk + 0x40, 0x300 + seed % 3);
                c.bus
                    .write32(blk + 0x174, if seed / 8 % 2 == 0 { 0 } else { 0x7700_1234 });
                c.bus.write32(blk + 0x38, (seed / 16 % 2) as u32);
                c.g[A0][0] = (seed / 32 % 256) as u64;
                c.g[A1][0] = [0u32, 1, 2][seed as usize / 96 % 3] as u64;
                c.pc = 0x003d_f8e8;
            },
        );
    }

    driven!(gen_async_file_get_result, ee::s14::f_003df9d8__generated);
    driven!(hand_async_file_get_result, async_file_get_result);

    /// Missing handles, live and finished blocks, with and without
    /// result/size out-words. The wait loop always sees a clear word.
    #[test]
    fn async_file_get_result_matches() {
        check(
            gen_async_file_get_result,
            hand_async_file_get_result,
            128,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_f9d8, 0);
                ea_install_oc1123_fakes();
                let base = DATA + 0x600;
                pool_block(c, base);
                let blk = base + 0x80;
                let mode = seed % 4;
                c.bus.write32(EAF_RET_FH, if mode == 0 { 0 } else { blk });
                c.bus.write32(blk, 0xa0 + seed % 2);
                c.bus
                    .write32(blk + 0x10, [0u32, 1, 2][seed as usize / 4 % 3]);
                c.bus.write32(blk + 0x8, 0x400 + seed % 5);
                c.bus.write32(blk + 0x1c, 0);
                c.bus.write32(blk + 0x2c, 0x500 + seed % 3);
                c.bus.write32(blk + 0xc, (seed / 12 % 2) as u32);
                c.g[A0][0] = (seed / 24 % 251) as u64;
                c.g[A1][0] = if seed / 48 % 3 == 0 {
                    0
                } else {
                    sx(DATA + 0x300)
                };
                c.g[A2][0] = if seed / 144 % 3 == 0 {
                    0
                } else {
                    sx(DATA + 0x310)
                };
                c.pc = 0x003d_f9d8;
            },
        );
    }

    driven!(gen_async_file_cancel, ee::s14::f_003dfaf0__generated);
    driven!(hand_async_file_cancel, async_file_cancel);

    /// Missing, live, and stale handles, with and without results and
    /// with and without a queued cancel.
    #[test]
    fn async_file_cancel_matches() {
        check(
            gen_async_file_cancel,
            hand_async_file_cancel,
            128,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_faf0, 0);
                ea_install_oc1123_fakes();
                let base = DATA + 0x600;
                pool_block(c, base);
                let blk = base + 0x80;
                let mode = seed % 4;
                c.bus.write32(EAF_RET_FH, if mode == 0 { 0 } else { blk });
                c.bus.write32(blk, 0xb0 + seed % 2);
                c.bus
                    .write32(blk + 0x10, [0u32, 1, 2][seed as usize / 4 % 3]);
                c.bus.write32(blk + 0xc, [0u32, 1][seed as usize / 12 % 2]);
                c.bus
                    .write32(blk + 0x1c, if seed / 24 % 3 == 0 { 0 } else { 0x7700_4321 });
                c.bus.write32(EAF_RET_TYPE, (seed / 72 % 2) as u32);
                c.g[A0][0] = (seed / 144 % 251) as u64;
                c.pc = 0x003d_faf0;
            },
        );
    }

    driven!(
        gen_stream_file_lookup_request,
        ee::s14::f_003dfdd0__generated
    );
    driven!(hand_stream_file_lookup_request, stream_file_lookup_request);

    /// In-range and out-of-range handles, matching and mismatched words.
    #[test]
    fn stream_file_lookup_request_matches() {
        check(
            gen_stream_file_lookup_request,
            hand_stream_file_lookup_request,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_fdd0, 0);
                let base = DATA + 0x600;
                for i in 0..16u32 {
                    c.bus.write32(base + 4 * i, 0);
                }
                let count = [0u32, 1, 4, 300][seed as usize % 4];
                c.bus.write32(base + 0x18, count);
                c.bus.write32(base + 0x14, DATA + 0x700);
                for i in 0..4u32 {
                    c.bus.write32(DATA + 0x700 + 4 * i, (seed / 4 + i) as u32);
                    c.bus.write32(DATA + 0x700 + 4 * i + 0x124, 0x1234 + i);
                }
                c.bus
                    .write32(DATA + 0x700 + 4 * (seed as u32 / 16 % 4), 0x1234 + seed % 4);
                c.g[A0][0] = base as u64;
                c.g[A1][0] = (seed / 64 % 300) as u64;
                c.pc = 0x003d_fdd0;
            },
        );
    }

    driven!(
        gen_stream_file_calc_mem_size,
        ee::s14::f_003e06b0__generated
    );
    driven!(hand_stream_file_calc_mem_size, stream_file_calc_mem_size);

    /// Channel counts and buffer sizes across the shift range.
    #[test]
    fn stream_file_calc_mem_size_matches() {
        check(
            gen_stream_file_calc_mem_size,
            hand_stream_file_calc_mem_size,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_06b0, 0);
                c.g[A0][0] = (seed % 17) as u64;
                c.g[A1][0] = (seed / 17 % 300) as u64;
                c.g[A2][0] = (seed / 5100 % 8) as u64;
                c.pc = 0x003e_06b0;
            },
        );
    }

    driven!(
        gen_stream_file_set_channel_filter,
        ee::s14::f_003e0948__generated
    );
    driven!(
        hand_stream_file_set_channel_filter,
        stream_file_set_channel_filter
    );

    /// Invalid streams, full and mismatched tables, set and unset slots.
    #[test]
    fn stream_file_set_channel_filter_matches() {
        check(
            gen_stream_file_set_channel_filter,
            hand_stream_file_set_channel_filter,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_0948, 0);
                ea_install_oc1123_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 5 == 0) as u32);
                c.bus
                    .write32(stream + 0x20, [0u32, 1, 4, 9][seed as usize / 5 % 4]);
                c.bus.write32(stream + 0x28, 0x10 + (seed / 20 % 4));
                c.bus.write32(stream + 0x1c, 0x5000 + seed % 7);
                c.bus.write32(stream + 0x4, 0x6000 + seed % 3);
                c.bus.write32(stream + 0x38, 0x7000 + seed % 3);
                c.g[A0][0] = (seed / 80 % 251) as u64;
                c.g[A1][0] = (seed / 320 % 9) as u64;
                c.g[A2][0] = (0x9000 + seed % 5) as u64;
                c.g[A3][0] = (0xa000 + seed % 5) as u64;
                c.g[T0][0] = [0i64 as u64, 1, 0xffff_ffff, 5][seed as usize / 1440 % 4];
                c.pc = 0x003e_0948;
            },
        );
    }

    driven!(gen_stream_file_destroy, ee::s14::f_003e0a28__generated);
    driven!(hand_stream_file_destroy, stream_file_destroy);

    /// Invalid streams (early exit); live streams with an empty V0 slot
    /// (the wait loop needs another thread, so it stays uncovered).
    #[test]
    fn stream_file_destroy_matches() {
        check(
            gen_stream_file_destroy,
            hand_stream_file_destroy,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_0a28, 0);
                ea_install_oc1123_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 3 == 0) as u32);
                c.g[A0][0] = (seed / 3 % 251) as u64;
                c.pc = 0x003e_0a28;
            },
        );
    }

    driven!(gen_stream_file_set_priority, ee::s14::f_003e0ad8__generated);
    driven!(hand_stream_file_set_priority, stream_file_set_priority);

    /// Invalid streams and stored priority pairs.
    #[test]
    fn stream_file_set_priority_matches() {
        check(
            gen_stream_file_set_priority,
            hand_stream_file_set_priority,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_0ad8, 0);
                ea_install_oc1123_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 3 == 0) as u32);
                c.bus.write32(stream + 0x3c, 0x100 + seed % 3);
                c.bus.write32(stream + 0x40, 0x200 + seed % 3);
                c.g[A0][0] = (seed / 3 % 251) as u64;
                c.g[A1][0] = (seed / 9 % 7) as u64;
                c.g[A2][0] = (seed / 63 % 7) as u64;
                c.pc = 0x003e_0ad8;
            },
        );
    }

    driven!(
        gen_stream_file_set_low_water,
        ee::s14::f_003e0b28__generated
    );
    driven!(hand_stream_file_set_low_water, stream_file_set_low_water);

    /// Invalid streams; low-water marks above and below the starved and
    /// current levels (the re-arm path calls back into SetStarved).
    #[test]
    fn stream_file_set_low_water_matches() {
        check(
            gen_stream_file_set_low_water,
            hand_stream_file_set_low_water,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_0b28, 0);
                ea_install_oc1123_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 4 == 0) as u32);
                // [SP] stream's level words: +0x44 current, +0x4c starved.
                c.bus
                    .write32(stream + 0x44, [0u32, 5, 10][seed as usize / 4 % 3]);
                c.bus
                    .write32(stream + 0x4c, [0u32, 5, 10][seed as usize / 12 % 3]);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = [0u32, 5, 10][seed as usize / 36 % 3] as u64;
                c.pc = 0x003e_0b28;
            },
        );
    }

    driven!(gen_stream_file_set_starved, ee::s14::f_003e0b98__generated);
    driven!(hand_stream_file_set_starved, stream_file_set_starved);

    /// Invalid streams; cleared and set flags; live and missing priority
    /// callbacks (null targets miss identically on both sides).
    #[test]
    fn stream_file_set_starved_matches() {
        check(
            gen_stream_file_set_starved,
            hand_stream_file_set_starved,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_0b98, 0);
                ea_install_oc1123_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 4 == 0) as u32);
                c.bus.write32(stream + 0x38, (seed / 4 % 2) as u32);
                c.bus.write32(stream + 0x40, 0x300 + seed % 3);
                c.bus.write32(
                    stream + 0x174,
                    if seed / 12 % 2 == 0 { 0 } else { 0x00c0_0000 },
                );
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = (seed / 24 % 3) as u64;
                c.pc = 0x003e_0b98;
            },
        );
    }

    driven!(gen_stream_file_get_channel, ee::s14::f_003e0bf8__generated);
    driven!(hand_stream_file_get_channel, stream_file_get_channel);

    /// Invalid streams; empty and populated channel tables.
    #[test]
    fn stream_file_get_channel_matches() {
        check(
            gen_stream_file_get_channel,
            hand_stream_file_get_channel,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_0bf8, 0);
                ea_install_oc1123_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 4 == 0) as u32);
                c.bus
                    .write32(stream + 0x28, [0u32, 1, 4][seed as usize / 4 % 3]);
                c.bus.write32(stream + 0x24, 0x8000 + seed % 5);
                c.bus.write32(stream + 0x1c, 0x8100 + seed % 5);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = [0u32, 1, 4, 9][seed as usize / 16 % 4] as u64;
                c.pc = 0x003e_0bf8;
            },
        );
    }

    driven!(
        gen_stream_file_get_channel_id,
        ee::s14::f_003e0c58__generated
    );
    driven!(hand_stream_file_get_channel_id, stream_file_get_channel_id);

    /// Invalid streams; present and absent channel ids.
    #[test]
    fn stream_file_get_channel_id_matches() {
        check(
            gen_stream_file_get_channel_id,
            hand_stream_file_get_channel_id,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_0c58, 0);
                ea_install_oc1123_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 3 == 0) as u32);
                c.bus
                    .write32(stream + 0x80 + 0x4, [0u32, 0x42][seed as usize / 3 % 2]);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = 0;
                c.pc = 0x003e_0c58;
            },
        );
    }

    driven!(gen_stream_file_queue_file, ee::s14::f_003e0d80__generated);
    driven!(hand_stream_file_queue_file, stream_file_queue_file);

    /// Invalid streams; null and live alloc results; matching and
    /// scanning channel lists; empty and live subscriber slots.
    #[test]
    fn stream_file_queue_file_matches() {
        check(
            gen_stream_file_queue_file,
            hand_stream_file_queue_file,
            128,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_0d80, 0);
                ea_install_oc1123_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 5 == 0) as u32);
                c.bus
                    .write32(EAF_RET_ALLOC, if seed % 5 == 1 { 0 } else { EAF_REQ });
                c.bus.write32(EAF_RET_ENQ, (seed / 5 % 2) as u32);
                c.bus.write32(EAF_RET_NEXT, (seed / 10 % 2) as u32);
                c.bus.write32(EAF_RET_TYPE, (seed / 20 % 2) as u32);
                c.bus.write32(EAF_RET_SETCB, (seed / 40 % 2) as u32);
                // Channel list walk starts at the stream word: [stream] is
                // the head id, [stream+4] links to a node holding the
                // target, so the walk terminates in at most one step
                // (a zero link with a mismatched head would spin the
                // fixed-word loop identically on both sides).
                let head = DATA + 0x500;
                c.bus.write32(stream, head);
                let target = if seed / 1920 % 2 == 0 { 0x1234 } else { 0x5678 };
                c.bus
                    .write32(head, if seed / 80 % 2 == 0 { target } else { 0x5678 });
                c.bus.write32(stream + 4, 0x20);
                c.bus.write32(stream + 0x20, target);
                c.bus.write32(EAF_REQ, 0);
                c.bus.write32(EAF_REQ + 4, 0);
                c.bus.write32(0x0051_9ae4, 0);
                c.g[A0][0] = 0;
                c.g[A1][0] = stream as u64;
                c.g[A2][0] = [0u32, 0x10, 0x1234][seed as usize / 640 % 3] as u64;
                c.g[A3][0] = target as u64;
                c.pc = 0x003e_0d80;
            },
        );
    }

    driven!(gen_stream_file_create, ee::s14::f_003e06d8__generated);
    driven!(hand_stream_file_create, stream_file_create);

    /// Valid configs lay out the pool and report the entries base;
    /// short channels, oversized counts, tiny budgets and inverted
    /// ranges take the zero exit.
    #[test]
    fn stream_file_create_matches() {
        check(
            gen_stream_file_create,
            hand_stream_file_create,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_06d8, 0);
                let stream = DATA + 0x400;
                for i in 0..256u32 {
                    c.bus.write32(stream + 4 * i, 0);
                }
                let (ch, s2, s3, s5) = match seed % 8 {
                    4 => (0, 4, 2, 0x20000),
                    5 => (300, 4, 2, 0x20000),
                    6 => (3, 4, 2, 0),
                    7 => (3, 1, 2, 0x20000),
                    _ => (2 + seed % 4, 4, 2, 0x20000),
                };
                c.g[A0][0] = ch as u64;
                c.g[A1][0] = s2 as u64;
                c.g[A2][0] = s3 as u64;
                c.g[A3][0] = stream as u64;
                c.g[T0][0] = s5 as u64;
                c.pc = 0x003e_06d8;
            },
        );
    }
}
