//! Hand-written replacements for `ealib/seg_2DCAE8` StreamFile/EAFile calls
//! (wave `wave-ealib-1.txt` rows 81-100), each with its differential test
//! against the generated body. Moved into the source-tree units by
//! `tools/reorg_modules.py apply oc1124`.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003e_0e90,
        stream_file_cancel_request,
        "StreamFile_CancelRequest",
    ),
    (0x003e_1110, stream_file_cancel_all, "StreamFile_CancelAll"),
    (
        0x003e_14f8,
        stream_file_get_read_state,
        "StreamFile_GetReadState",
    ),
    (
        0x003e_1580,
        stream_file_get_buffer_size,
        "StreamFile_GetBufferSize",
    ),
    (
        0x003e_15b8,
        stream_file_get_buffered_bytes,
        "StreamFile_GetBufferedBytes",
    ),
    (
        0x003e_15e8,
        stream_file_get_chunk_request,
        "StreamFile_GetChunkRequest",
    ),
    (
        0x003e_16b0,
        stream_file_get_request_status,
        "StreamFile_GetRequestStatus",
    ),
    (0x003e_1700, ea_file_exists_default, "EAFile_ExistsDefault"),
    (
        0x003e_1728,
        ea_file_get_file_size_worker,
        "EAFile_GetFileSizeWorker",
    ),
    (
        0x003e_1798,
        ea_file_get_file_size_ex,
        "EAFile_GetFileSizeEx",
    ),
    (0x003e_17d8, ea_file_get_file_size, "EAFile_GetFileSize"),
    (
        0x003e_1810,
        ea_file_load_file_worker,
        "EAFile_LoadFileWorker",
    ),
    (0x003e_18c8, ea_file_load_file_ex, "EAFile_LoadFileEx"),
    (0x003e_1908, ea_file_load_file, "EAFile_LoadFile"),
    (
        0x003e_1948,
        ea_file_load_file_get_size_worker,
        "EAFile_LoadFileGetSizeWorker",
    ),
    (
        0x003e_1a10,
        ea_file_load_file_get_size_ex,
        "EAFile_LoadFileGetSizeEx",
    ),
    (
        0x003e_1a70,
        ea_file_load_file_get_size,
        "EAFile_LoadFileGetSize",
    ),
    (
        0x003e_1ad0,
        ea_file_load_file_into_worker,
        "EAFile_LoadFileIntoWorker",
    ),
    (
        0x003e_1b68,
        ea_file_load_file_into_ex,
        "EAFile_LoadFileIntoEx",
    ),
    (0x003e_1bb8, ea_file_load_file_into, "EAFile_LoadFileInto"),
];

mod callee {
    use super::ee;
    use crate::call::Callee;

    pub(crate) const STREAM_FILE_VALIDATE: Callee = Callee {
        addr: 0x003d_fbd0,
        f: ee::s14::StreamFile_Validate,
    };
    pub(crate) const STREAM_FILE_LOOKUP_REQUEST: Callee = Callee {
        addr: 0x003d_fdd0,
        f: ee::s14::StreamFile_LookupRequest,
    };
    pub(crate) const STREAM_FILE_FREE_REQUEST: Callee = Callee {
        addr: 0x003d_fe18,
        f: ee::s14::StreamFile_FreeRequest,
    };
    pub(crate) const STREAM_FILE_RING_IN_RANGE: Callee = Callee {
        addr: 0x003d_fc08,
        f: ee::s14::StreamFile_RingInRange,
    };
    pub(crate) const STREAM_FILE_CONSUME_BYTES: Callee = Callee {
        addr: 0x003d_fc48,
        f: ee::s14::StreamFile_ConsumeBytes,
    };
    pub(crate) const STREAM_FILE_RELEASE_CHUNK: Callee = Callee {
        addr: 0x003e_13e8,
        f: ee::s14::StreamFile_ReleaseChunk,
    };
    pub(crate) const STREAM_FILE_GET_CHUNK: Callee = Callee {
        addr: 0x003e_12e0,
        f: ee::s14::StreamFile_GetChunk,
    };
    /// Intra-module call: the generated body calls the raw body, so both
    /// sides run the generated code here.
    pub(crate) const STREAM_FILE_CANCEL_REQUEST: Callee = Callee {
        addr: 0x003e_0e90,
        f: ee::s14::StreamFile_CancelRequest,
    };
    pub(crate) const EA_FILE_OPEN: Callee = Callee {
        addr: 0x003d_ebf0,
        f: ee::s14::EAFile_Open,
    };
    pub(crate) const EA_FILE_CLOSE: Callee = Callee {
        addr: 0x003d_eca0,
        f: ee::s14::EAFile_Close,
    };
    pub(crate) const EA_FILE_READ: Callee = Callee {
        addr: 0x003d_ec60,
        f: ee::s14::EAFile_Read,
    };
    pub(crate) const EA_FILE_GET_SIZE: Callee = Callee {
        addr: 0x003d_ecf8,
        f: ee::s14::EAFile_GetSize,
    };
    pub(crate) const EA_FILE_EXISTS: Callee = Callee {
        addr: 0x003d_ee18,
        f: ee::s14::EAFile_Exists,
    };
    pub(crate) const EA_FILE_CALL_WITH_PRIORITY: Callee = Callee {
        addr: 0x003d_ddf0,
        f: ee::s14::EAFile_CallWithPriority,
    };
    pub(crate) const EA_MUTEX_LOCK: Callee = Callee {
        addr: 0x003e_5700,
        f: ee::s14::EAMutex_Lock,
    };
    pub(crate) const EA_MUTEX_UNLOCK: Callee = Callee {
        addr: 0x003e_5760,
        f: ee::s14::EAMutex_Unlock,
    };
}

/// `u32 StreamFile_GetReadState(stream)` (0x3e14f8): returns the
/// subscriber epoch word, or 0 when the stream is invalid.
pub fn stream_file_get_read_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_150c);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_150c)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            c.cycles += 4;
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x38));
            frame.restore_ra(c, 0x10);
        } else {
            c.cycles += 1;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_GetBufferSize(stream)` (0x3e1580): usable bytes
/// between the write and read cursors, or 0 when invalid.
pub fn stream_file_get_buffer_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_1594);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_1594)?;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 4;
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_GetBufferedBytes(stream)` (0x3e15b8): bytes currently
/// buffered, or 0 when invalid.
pub fn stream_file_get_buffered_bytes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_15cc);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_15cc)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            c.cycles += 2;
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_GetRequestStatus(stream, handle)` (0x3e16b0):
/// status word of a queued request, or 0 when absent/invalid.
pub fn stream_file_get_request_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003e_16cc);
        c.g[A1][0] = c.g[SP][0];
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_16cc)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            c.cycles += 5;
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            c.g[RA][0] = sx(0x003e_16e0);
            c.g[A1][0] = c.g[S0][0];
            callee::STREAM_FILE_LOOKUP_REQUEST.call(c, 0x003e_16e0)?;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_GetChunkRequest(stream)` (0x3e15e8): finds the next
/// live chunk request, or 0 when the stream is invalid or drained.
pub fn stream_file_get_chunk_request(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_650: u32 = 0x003e_1650;
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003e_160c);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_160c)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            let live = c.g[V1][0] != 0;
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            if live {
                c.cycles += 9;
                c.g[S3][0] = 0x1;
                c.g[RA][0] = sx(0x003e_1638);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                callee::EA_MUTEX_LOCK.call(c, 0x003e_1638)?;
                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x50));
                c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                // Walk the subscriber list for a live entry.
                loop {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    if c.g[A1][0] == 0 {
                        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                    let found = c.g[V0][0] == c.g[S3][0];
                    c.g[A2][0] = c.g[S2][0];
                    if found {
                        c.cycles += 1;
                        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                        break;
                    }
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x120));
                    c.g[RA][0] = sx(0x003e_1678);
                    c.g[A1][0] = c.g[S0][0];
                    callee::STREAM_FILE_RING_IN_RANGE.call(c, 0x003e_1678)?;
                    if c.g[V0][0] != 0 {
                        c.cycles += 1;
                        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                        break;
                    }
                    c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S0][0];
                    if event_due(c) {
                        deliver_event_here(c, HEAD_650);
                    }
                }
                // Found (or list end): unlock and report.
                c.cycles += 3;
                c.g[RA][0] = sx(0x003e_168c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                callee::EA_MUTEX_UNLOCK.call(c, 0x003e_168c)?;
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
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

/// `u32 EAFile_ExistsDefault(path)` (0x3e1700): existence check with
/// the default settings word.
pub fn ea_file_exists_default(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V0][0] = 0x450000;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_1718);
        callee::EA_FILE_EXISTS.call(c, 0x003e_1718)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_GetFileSizeWorker(path, ...)` (0x3e1728): opens the file,
/// reads its size and closes it; 0 when the open fails.
pub fn ea_file_get_file_size_worker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[A3][0] = c.g[SP][0];
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.lw(c.g[A1][0] as u32);
        c.g[RA][0] = sx(0x003e_1750);
        c.g[A1][0] = 0x1;
        callee::EA_FILE_OPEN.call(c, 0x003e_1750)?;
        let opened = c.g[V0][0] != 0;
        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
        if opened {
            c.cycles += 8;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            c.g[RA][0] = sx(0x003e_176c);
            c.g[A1][0] = c.g[S1][0];
            callee::EA_FILE_GET_SIZE.call(c, 0x003e_176c)?;
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            c.g[S0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003e_177c);
            c.g[A1][0] = c.g[S1][0];
            callee::EA_FILE_CLOSE.call(c, 0x003e_177c)?;
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_GetFileSizeEx(path, size_out)` (0x3e1798): size query
/// through the priority-call gate.
pub fn ea_file_get_file_size_ex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x450000;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[V1][0] = 0x1;
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = 0x3e0000;
        frame.save_ra(c, 0x20);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1728));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x003e_17c8);
        c.g[A3][0] = c.g[SP][0];
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_17c8)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_GetFileSize(path, size_out)` (0x3e17d8): size query
/// without the priority flag.
pub fn ea_file_get_file_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x450000;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[A1][0] = 0;
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = 0x3e0000;
        frame.save_ra(c, 0x20);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1728));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0);
        c.g[RA][0] = sx(0x003e_1804);
        c.g[A3][0] = c.g[SP][0];
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1804)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFileWorker(path, ...)` (0x3e1810): opens the file,
/// allocates a buffer, reads it fully and closes; 0 on any failure.
pub fn ea_file_load_file_worker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[A3][0] = c.g[SP][0];
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = 0x1;
        c.g[A2][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x003e_1840);
        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
        callee::EA_FILE_OPEN.call(c, 0x003e_1840)?;
        let failed = c.g[V0][0] == 0;
        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
        // The success path returns the buffer directly; every other
        // path runs the shared +1/null tail.
        let mut done = false;
        if !failed {
            c.cycles += 13;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            c.g[RA][0] = sx(0x003e_1854);
            c.g[A1][0] = c.g[S1][0];
            callee::EA_FILE_GET_SIZE.call(c, 0x003e_1854)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V1][0] = 0x450000;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = c.g[S2][0];
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc14));
            let alloc = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003e_1870);
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.pc = alloc;
            call_ptr(c, alloc, 0x003e_1870)?;
            c.g[S0][0] = c.g[V0][0];
            let no_mem = c.g[S0][0] == 0;
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            if no_mem {
                c.cycles += 2;
                c.g[RA][0] = sx(0x003e_18ac);
                c.g[A1][0] = c.g[S1][0];
                callee::EA_FILE_CLOSE.call(c, 0x003e_18ac)?;
            } else {
                c.cycles += 10;
                c.g[A3][0] = c.g[S2][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003e_1890);
                c.g[T0][0] = c.g[S1][0];
                callee::EA_FILE_READ.call(c, 0x003e_1890)?;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[RA][0] = sx(0x003e_189c);
                c.g[A1][0] = c.g[S1][0];
                callee::EA_FILE_CLOSE.call(c, 0x003e_189c)?;
                c.g[V0][0] = c.g[S0][0];
                done = true;
            }
        }
        if !done {
            c.cycles += 1;
            c.g[V0][0] = 0;
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

/// `u32 EAFile_LoadFileEx(path, ...)` (0x3e18c8): load through the
/// priority-call gate with the allocate flag set.
pub fn ea_file_load_file_ex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = 0x450000;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[V0][0] = 0x1;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
        c.g[A0][0] = 0x3e0000;
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1810));
        frame.save_ra(c, 0x20);
        c.g[A1][0] = 0;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[RA][0] = sx(0x003e_18fc);
        c.g[A3][0] = c.g[SP][0];
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_18fc)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFile(path, ...)` (0x3e1908): load through the
/// priority-call gate without the allocate flag.
pub fn ea_file_load_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x450000;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[A3][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
        c.g[A0][0] = 0x3e0000;
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1810));
        frame.save_ra(c, 0x20);
        c.g[A1][0] = 0;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0);
        c.g[RA][0] = sx(0x003e_1938);
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1938)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFileGetSizeWorker(path, ...)` (0x3e1948): opens the
/// file, allocates a buffer, reads it, records the size and closes.
pub fn ea_file_load_file_get_size_worker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[A3][0] = c.g[SP][0];
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S2, 0x30);
        c.g[A1][0] = 0x1;
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x003e_197c);
        c.g[A0][0] = c.lw(c.g[S3][0] as u32);
        callee::EA_FILE_OPEN.call(c, 0x003e_197c)?;
        let failed = c.g[V0][0] == 0;
        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
        let mut done = false;
        if !failed {
            c.cycles += 13;
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            c.g[RA][0] = sx(0x003e_1990);
            c.g[A1][0] = c.g[S1][0];
            callee::EA_FILE_GET_SIZE.call(c, 0x003e_1990)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V1][0] = 0x450000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc14));
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.lw(c.g[S3][0] as u32);
            let alloc = c.g[V0][0] as u32;
            c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = sx(0x003e_19ac);
            c.pc = alloc;
            call_ptr(c, alloc, 0x003e_19ac)?;
            c.g[S0][0] = c.g[V0][0];
            let no_mem = c.g[S0][0] == 0;
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            if no_mem {
                c.cycles += 2;
                c.g[RA][0] = sx(0x003e_19ec);
                c.g[A1][0] = c.g[S1][0];
                callee::EA_FILE_CLOSE.call(c, 0x003e_19ec)?;
            } else {
                c.cycles += 11;
                c.g[A1][0] = 0;
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x003e_19cc);
                c.g[T0][0] = c.g[S1][0];
                callee::EA_FILE_READ.call(c, 0x003e_19cc)?;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[RA][0] = sx(0x003e_19d8);
                c.g[A1][0] = c.g[S1][0];
                callee::EA_FILE_CLOSE.call(c, 0x003e_19d8)?;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[S2][0]);
                c.g[V0][0] = c.g[S0][0];
                done = true;
            }
        }
        if !done {
            c.cycles += 1;
            c.g[V0][0] = 0;
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

/// `u32 EAFile_LoadFileGetSizeEx(path, size_out, ...)` (0x3e1a10):
/// get-size-and-load through the priority-call gate.
pub fn ea_file_load_file_get_size_ex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x450000;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[V1][0] = 0x1;
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x3e0000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
        c.g[S0][0] = c.g[A1][0];
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1948));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0x20);
        c.g[RA][0] = sx(0x003e_1a4c);
        c.g[A3][0] = c.g[SP][0];
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1a4c)?;
        let missing = c.g[S0][0] == 0;
        c.g[V1][0] = c.g[V0][0];
        if missing {
            // Fall through to the shared tail below.
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = c.g[V1][0];
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFileGetSize(path, size_out, ...)` (0x3e1a70): same
/// without the priority flag.
pub fn ea_file_load_file_get_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x450000;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[A3][0] = c.g[SP][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x3e0000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
        c.g[S0][0] = c.g[A1][0];
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1948));
        frame.save_ra(c, 0x20);
        c.g[A1][0] = 0;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0);
        c.g[RA][0] = sx(0x003e_1aa8);
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1aa8)?;
        let missing = c.g[S0][0] == 0;
        c.g[V1][0] = c.g[V0][0];
        if missing {
            // Fall through to the shared tail below.
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = c.g[V1][0];
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFileIntoWorker(path, ...)` (0x3e1ad0): opens the
/// file, checks the caller's size and reads into its buffer.
pub fn ea_file_load_file_into_worker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x7fff0000;
        frame.save(c, S0, 0x40);
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        c.g[A1][0] = 0x1;
        c.g[S1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.lw(c.g[S2][0] as u32);
        c.g[A3][0] = c.g[SP][0];
        c.g[V1][0] = ((0u64 as i64) < (c.g[S1][0] as i64)) as u64;
        c.g[RA][0] = sx(0x003e_1b14);
        if c.g[V1][0] == 0 {
            c.g[S1][0] = c.g[V0][0];
        }
        callee::EA_FILE_OPEN.call(c, 0x003e_1b14)?;
        let opened = c.g[V0][0] != 0;
        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
        let mut done = false;
        if opened {
            c.cycles += 11;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
            c.g[A3][0] = c.g[S1][0];
            c.g[T0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x003e_1b3c);
            c.g[A1][0] = 0;
            callee::EA_FILE_READ.call(c, 0x003e_1b3c)?;
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            c.g[S1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003e_1b4c);
            c.g[A1][0] = c.g[S0][0];
            callee::EA_FILE_CLOSE.call(c, 0x003e_1b4c)?;
            c.g[V0][0] = c.g[S1][0];
            done = true;
        }
        if !done {
            c.cycles += 2;
            c.g[V0][0] = 0;
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

/// `u32 EAFile_LoadFileIntoEx(path, ...)` (0x3e1b68): into-buffer load
/// through the priority-call gate.
pub fn ea_file_load_file_into_ex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[A3][0] = c.g[A2][0];
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[V0][0] = 0x450000;
        c.g[V1][0] = 0x1;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
        c.g[A0][0] = 0x3e0000;
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1ad0));
        frame.save_ra(c, 0x20);
        c.g[A1][0] = 0;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[A3][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_1ba8);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0);
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1ba8)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFileInto(path, ...)` (0x3e1bb8): into-buffer load
/// without the priority flag.
pub fn ea_file_load_file_into(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = c.g[A2][0];
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[V0][0] = 0x450000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        c.g[A0][0] = 0x3e0000;
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1ad0));
        frame.save_ra(c, 0x20);
        c.g[A1][0] = 0;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A3][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0);
        c.g[RA][0] = sx(0x003e_1bf4);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0);
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1bf4)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_CancelRequest(stream, handle)` (0x3e0e90): cancels
/// a queued chunk request, freeing it or marking it done.
pub fn stream_file_cancel_request(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_F98: u32 = 0x003e_0f98;
        const HEAD_1000: u32 = 0x003e_1000;
        const HEAD_1090: u32 = 0x003e_1090;
        c.cycles += 20;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        frame.save(c, S6, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S7, 0x30);
        c.g[S6][0] = 0;
        frame.save(c, S1, 0x90);
        c.g[S7][0] = 0;
        frame.save(c, S2, 0x80);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003e_0ed8);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0);
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_0ed8)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0xa0);
        } else {
            'b_3e0f74: {
                'b_3e0f70: {
                    'b_3e0f18: {
                        'b_3e0f10: {
                            c.cycles += 9;
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                            c.g[RA][0] = sx(0x003e_0eec);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                            callee::EA_MUTEX_LOCK.call(c, 0x003e_0eec)?;
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                            c.g[RA][0] = sx(0x003e_0ef8);
                            c.g[A1][0] = c.g[S0][0];
                            callee::STREAM_FILE_LOOKUP_REQUEST.call(c, 0x003e_0ef8)?;
                            c.g[A1][0] = c.g[V0][0];
                            let missing = c.g[A1][0] == 0;
                            c.g[V1][0] = 0x4;
                            if missing {
                                break 'b_3e0f10;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                            if c.g[V0][0] != c.g[V1][0] {
                                c.g[A0][0] = 0x1;
                                break 'b_3e0f18;
                            }
                            break 'b_3e0f10;
                        }
                        c.cycles += 2;
                        c.g[S0][0] = 0x1;
                        break 'b_3e0f70;
                    }
                    'b_3e0f34: {
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A0][0] {
                            c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                            break 'b_3e0f34;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                        c.g[RA][0] = sx(0x003e_0f2c);
                        c.g[S0][0] = 0x1;
                        callee::STREAM_FILE_FREE_REQUEST.call(c, 0x003e_0f2c)?;
                        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                        break 'b_3e0f74;
                    }
                    'b_3e0f50: {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x50));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                        let tied = c.g[A1][0] == c.g[V0][0];
                        c.g[S6][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        if tied {
                            break 'b_3e0f50;
                        }
                        c.cycles += 1;
                        c.g[S6][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x120));
                        break 'b_3e0f50;
                    }
                    'b_3e0f6c: {
                        c.cycles += 3;
                        c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                        if c.g[A1][0] == 0 {
                            c.g[S7][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                            break 'b_3e0f6c;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                        if c.g[V0][0] != c.g[A0][0] {
                            c.g[S7][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x120));
                            break 'b_3e0f6c;
                        }
                        c.cycles += 1;
                        c.g[S7][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                        break 'b_3e0f6c;
                    }
                    c.cycles += 1;
                    c.g[S0][0] = 0;
                    break 'b_3e0f70;
                }
                c.cycles += 1;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                break 'b_3e0f74;
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x003e_0f7c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            callee::EA_MUTEX_UNLOCK.call(c, 0x003e_0f7c)?;
            if c.g[S0][0] != 0 {
                frame.restore(c, S0, 0xa0);
            } else {
                'b_3e10e0: {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                    let empty = (c.g[V0][0] as i64) <= 0;
                    c.g[A0][0] = 0;
                    if empty {
                        break 'b_3e10e0;
                    }
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                    'l_3e0f98: loop {
                        'b_3e10cc: {
                            c.cycles += 8;
                            c.g[V0][0] = sx((c.g[A0][0] as u32) << 4);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[A0][0]);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                            if (c.g[V0][0] as i64) <= 0 {
                                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                                break 'b_3e10cc;
                            }
                            'b_3e10c8: {
                                'b_3e1070: {
                                    c.cycles += 6;
                                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                    c.g[A1][0] = c.g[S6][0];
                                    c.g[RA][0] = sx(0x003e_0fc8);
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                    callee::STREAM_FILE_RING_IN_RANGE.call(c, 0x003e_0fc8)?;
                                    let out = c.g[V0][0] == 0;
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                    if out {
                                        break 'b_3e1070;
                                    }
                                    c.cycles += 5;
                                    c.g[S0][0] = c.g[S6][0];
                                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                    let synced = c.g[S6][0] == c.g[S7][0];
                                    c.g[S3][0] = sx((c.g[V0][0] as u32) << 24);
                                    if synced {
                                        break 'b_3e10c8;
                                    }
                                    c.cycles += 7;
                                    c.g[S2][0] = 0xff0000;
                                    c.g[FP][0] = 0xffffffffffffffff;
                                    c.g[S2][0] = c.g[S2][0] | 0xffff;
                                    c.g[S5][0] = 0xffffffffff000000;
                                    c.g[S4][0] = 0xfffffffffffffffe;
                                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                    'l_3e1000: loop {
                                        'b_3e1060: {
                                            'b_3e1010: {
                                                c.cycles += 2;
                                                if c.g[V0][0] != c.g[FP][0] {
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                    break 'b_3e1010;
                                                }
                                                c.cycles += 2;
                                                c.g[S0][0] =
                                                    c.lw((c.g[A0][0] as u32).wrapping_add(0x30));
                                                break 'b_3e1060;
                                            }
                                            'b_3e105c: {
                                                c.cycles += 4;
                                                c.g[S1][0] = c.g[V0][0] & c.g[S2][0];
                                                c.g[V0][0] = c.g[V0][0] & c.g[S5][0];
                                                let other = c.g[V0][0] != c.g[S3][0];
                                                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                                                if other {
                                                    break 'b_3e105c;
                                                }
                                                c.cycles += 15;
                                                c.g[RA][0] = sx(0x003e_1028);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                                callee::EA_MUTEX_LOCK.call(c, 0x003e_1028)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_sub(c.g[S1][0] as u32));
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x8),
                                                    c.g[V0][0],
                                                );
                                                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                                                c.g[RA][0] = sx(0x003e_1044);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                                callee::EA_MUTEX_UNLOCK.call(c, 0x003e_1044)?;
                                                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                                                c.g[RA][0] = sx(0x003e_1050);
                                                c.g[A1][0] = c.g[S1][0];
                                                callee::STREAM_FILE_CONSUME_BYTES
                                                    .call(c, 0x003e_1050)?;
                                                c.sw(c.g[S0][0] as u32, c.g[S4][0]);
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x4),
                                                    c.g[S1][0],
                                                );
                                                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                                                break 'b_3e105c;
                                            }
                                            c.cycles += 1;
                                            c.g[S0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[S1][0] as u32));
                                            break 'b_3e1060;
                                        }
                                        c.cycles += 2;
                                        if c.g[S0][0] != c.g[S7][0] {
                                            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                            if event_due(c) {
                                                deliver_event_here(c, HEAD_1000);
                                            }
                                            continue 'l_3e1000;
                                        }
                                        c.cycles += 2;
                                        c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                                        break 'b_3e10cc;
                                    }
                                }
                                c.cycles += 7;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                c.g[A0][0] = c.g[S6][0];
                                c.g[A1][0] = c.g[S7][0];
                                c.g[RA][0] = sx(0x003e_1084);
                                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                callee::STREAM_FILE_RING_IN_RANGE.call(c, 0x003e_1084)?;
                                let out = c.g[V0][0] == 0;
                                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                                if out {
                                    break 'b_3e10cc;
                                }
                                c.cycles += 1;
                                'l_3e1090: loop {
                                    c.cycles += 9;
                                    c.g[RA][0] = sx(0x003e_1098);
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                    callee::STREAM_FILE_GET_CHUNK.call(c, 0x003e_1098)?;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                    c.g[RA][0] = sx(0x003e_10a4);
                                    c.g[A1][0] = c.g[V0][0];
                                    callee::STREAM_FILE_RELEASE_CHUNK.call(c, 0x003e_10a4)?;
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                    let starved = (c.g[V0][0] as i64) <= 0;
                                    c.g[A0][0] = c.g[S6][0];
                                    if starved {
                                        break 'b_3e10c8;
                                    }
                                    c.cycles += 5;
                                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                    c.g[RA][0] = sx(0x003e_10c0);
                                    c.g[A1][0] = c.g[S7][0];
                                    callee::STREAM_FILE_RING_IN_RANGE.call(c, 0x003e_10c0)?;
                                    if c.g[V0][0] == 0 {
                                        break 'b_3e10c8;
                                    }
                                    if event_due(c) {
                                        deliver_event_here(c, HEAD_1090);
                                    }
                                    continue 'l_3e1090;
                                }
                            }
                            c.cycles += 1;
                            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                            break 'b_3e10cc;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                            if event_due(c) {
                                deliver_event_here(c, HEAD_F98);
                            }
                            continue 'l_3e0f98;
                        }
                        break 'b_3e10e0;
                    }
                }
                c.cycles += 1;
                frame.restore(c, S0, 0xa0);
            }
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_CancelAll(stream)` (0x3e1110): cancels every chunk
/// request on a stream and reclaims its buffers.
pub fn stream_file_cancel_all(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_1158: u32 = 0x003e_1158;
        const HEAD_1188: u32 = 0x003e_1188;
        const HEAD_11C8: u32 = 0x003e_11c8;
        const HEAD_1220: u32 = 0x003e_1220;
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = 0;
        c.g[RA][0] = sx(0x003e_1134);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_1134)?;
        let invalid = c.g[V0][0] != 0;
        if invalid {
            frame.restore(c, S0, 0x30);
        } else {
            // Per-channel head: an empty head restores S0 and skips
            // straight to the shared epilogue (no drain/consume/fixup).
            c.cycles += 4;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
            let no_head = c.g[A1][0] == 0;
            if no_head {
                frame.restore(c, S0, 0x30);
            }
            if !no_head {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                loop {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V0][0] < 0x2) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[A1][0] = c.lw(c.g[A1][0] as u32);
                    if event_due(c) {
                        deliver_event_here(c, HEAD_1158);
                    }
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003e_1160);
                    c.g[A0][0] = c.g[S0][0];
                    callee::STREAM_FILE_CANCEL_REQUEST.call(c, 0x003e_1160)?;
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                }
                c.cycles += 2;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            }
            if !no_head {
                // Drain live channel requests.
                loop {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5c));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x60));
                    let drained = c.g[V1][0] == c.g[V0][0];
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    if drained {
                        break;
                    }
                    c.cycles += 4;
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5c));
                    c.g[RA][0] = sx(0x003e_1194);
                    callee::STREAM_FILE_FREE_REQUEST.call(c, 0x003e_1194)?;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    if event_due(c) {
                        deliver_event_here(c, HEAD_1188);
                    }
                }
                // Zero the buffer table.
                c.cycles += 8;
                c.g[A0][0] = 0x4;
                c.g[A1][0] = 0;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x60));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                let empty = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                if !empty {
                    loop {
                        c.cycles += 10;
                        c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                        c.g[V0][0] = sx((c.g[A1][0] as u32) << 4);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
                        c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, HEAD_11C8);
                        }
                    }
                }
                // Consume leftover bytes, then fix up the cursors.
                c.cycles += 8;
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4c));
                c.g[RA][0] = sx(0x003e_11fc);
                callee::STREAM_FILE_CONSUME_BYTES.call(c, 0x003e_11fc)?;
                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x50));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                let synced = c.g[A0][0] == c.g[V0][0];
                c.g[A1][0] = 0xff0000;
                if !synced {
                    c.cycles += 4;
                    c.g[A3][0] = 0xffffffffffffffff;
                    c.g[A1][0] = c.g[A1][0] | 0xffff;
                    c.g[A2][0] = 0xfffffffffffffffe;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    // Walk entries to the cursor.
                    loop {
                        c.cycles += 2;
                        if c.g[V0][0] == c.g[A3][0] {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x30));
                            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                        } else {
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[S1][0] = c.g[V0][0] & c.g[A1][0];
                            c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                            let other = c.g[V0][0] != c.g[S3][0];
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                            if other {
                                break;
                            }
                            c.cycles += 15;
                            c.g[RA][0] = sx(0x003e_1028);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                            callee::EA_MUTEX_LOCK.call(c, 0x003e_1028)?;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                            c.g[RA][0] = sx(0x003e_1044);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                            callee::EA_MUTEX_UNLOCK.call(c, 0x003e_1044)?;
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                            c.g[RA][0] = sx(0x003e_1050);
                            c.g[A1][0] = c.g[S1][0];
                            callee::STREAM_FILE_CONSUME_BYTES.call(c, 0x003e_1050)?;
                            c.sw(c.g[S0][0] as u32, c.g[S4][0]);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                        if c.g[A0][0] == c.g[V0][0] {
                            break;
                        }
                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, HEAD_1220);
                        }
                    }
                }
                c.cycles += 5;
                c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                c.g[V1][0] = 0x2;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x38));
                let starved = c.g[V0][0] != c.g[V1][0];
                frame.restore(c, S0, 0x30);
                if !starved {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x30));
                    if c.g[A0][0] != c.g[V0][0] {
                        c.cycles += 15;
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x54));
                        c.g[V1][0] = 0x40;
                        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x54));
                        c.g[V0][0] = c.g[V0][0] & 0x3f;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                        c.g[V0][0] = c.g[V1][0] ^ 0x40;
                        if c.g[V0][0] == 0 {
                            c.g[V1][0] = 0;
                        }
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x54));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                        c.sw((c.g[A1][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x54));
                        c.sw((c.g[A1][0] as u32).wrapping_add(0x30), c.g[V1][0]);
                        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    }
                }
                c.cycles += 2;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x38), 0);
                frame.restore(c, S0, 0x30);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
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
    pub(crate) const EAF_RET_VALID: u32 = DATA + 0x2_002c;
    pub(crate) const EAF_RET_ALLOC: u32 = DATA + 0x2_0030;
    pub(crate) const EAF_RET_SVCQ: u32 = DATA + 0x2_003c;

    /// A zeroed stream block with selectable header fields.
    fn stream_block(c: &mut Cpu, base: u32) {
        for i in 0..64u32 {
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

    /// Validate passes its stream through `[SP]/[SP+4]` like the real one.
    pub(crate) fn ea_fake_stream_validate(c: &mut Cpu) {
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_stream_validate),
                ee::s14::f_003dfbd0__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0xeaf0_1012, ra);
        let v = c.bus.read32(EAF_RET_VALID);
        c.g[V0][0] = v as u64;
        if v == 0 {
            c.bus.write32(c.g[SP][0] as u32 + 4, c.g[A0][0] as u32);
            let inner = c.bus.read32(c.g[A0][0] as u32);
            c.bus.write32(c.g[SP][0] as u32, inner);
        }
    }

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
        ea_fake_alloc_request,
        0,
        ee::s14::f_003de670__generated,
        EAF_RET_ALLOC,
        0xeaf0_1000
    );
    ea_fake_ret!(
        ea_fake_alloc_file_handle,
        0,
        ee::s14::f_003de800__generated,
        EAF_RET_ALLOC,
        0xeaf0_1001
    );
    ea_fake_ret!(
        ea_fake_service_queue,
        0,
        ee::s14::f_003ddfa8__generated,
        EAF_RET_SVCQ,
        0xeaf0_1017
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

    pub(crate) fn ea_install_oc1124_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DFBD0),
            ea_fake_stream_validate,
            fakes::prev_cell(ea_fake_stream_validate),
        );
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
            addr_of_mut!(ee::s14::OVERRIDE_003DDFA8),
            ea_fake_service_queue,
            fakes::prev_cell(ea_fake_service_queue),
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
        gen_stream_file_cancel_request,
        ee::s14::f_003e0e90__generated
    );
    driven!(hand_stream_file_cancel_request, stream_file_cancel_request);

    /// Invalid streams; lookup miss/hit; mark, free and match arms;
    /// empty and single-step scan lists.
    #[test]
    fn stream_file_cancel_request_matches() {
        check(
            gen_stream_file_cancel_request,
            hand_stream_file_cancel_request,
            128,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_0e90, 0);
                ea_install_oc1124_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 5 == 0) as u32);
                // Self-describing stream: [stream] points back at it, so
                // the validate fake resolves [SP] to the stream.
                c.bus.write32(stream, stream);
                let table = DATA + 0x700;
                let mode = seed % 6;
                // Lookup table: count + one entry slot.
                c.bus.write32(stream + 0x18, if mode == 1 { 0 } else { 1 });
                c.bus.write32(stream + 0x14, table);
                c.bus.write32(table, 0x40 + (seed / 6 % 4));
                c.bus
                    .write32(table + 4, [0u32, 1, 4, 9][seed as usize / 24 % 4]);
                c.bus.write32(table + 0xc, 0);
                c.bus.write32(stream + 0x50, 0x9000 + seed % 3);
                c.bus.write32(stream + 0x5c, 0x9100 + seed % 3);
                c.bus.write32(stream + 0x54, 0x9200 + seed % 5);
                c.bus
                    .write32(stream + 0x28, [0u32, 1][seed as usize / 120 % 2]);
                c.bus.write32(stream + 0x24, DATA + 0x900);
                c.bus.write32(DATA + 0x900 + 8, 0);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = if mode == 0 {
                    0x40 + (seed / 240 % 4) as u64
                } else {
                    (seed / 240 % 251) as u64
                };
                c.pc = 0x003e_0e90;
            },
        );
    }

    driven!(gen_stream_file_cancel_all, ee::s14::f_003e1110__generated);
    driven!(hand_stream_file_cancel_all, stream_file_cancel_all);

    /// Invalid streams; empty heads, short walks and immediate exits;
    /// always with terminating request lists.
    #[test]
    fn stream_file_cancel_all_matches() {
        check(
            gen_stream_file_cancel_all,
            hand_stream_file_cancel_all,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1110, 0);
                ea_install_oc1124_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 4 == 0) as u32);
                c.bus.write32(stream, stream);
                // Request list head: null skips the loop, otherwise a
                // two-node chain ending in a count >= 3.
                let n1 = DATA + 0x500;
                let n2 = DATA + 0x540;
                let head_mode = seed % 3;
                c.bus
                    .write32(stream + 0x64, if head_mode == 0 { 0 } else { n1 });
                c.bus.write32(n1, n2);
                c.bus.write32(n1 + 4, [0u32, 0, 5][head_mode as usize]);
                c.bus.write32(n2, 0);
                c.bus.write32(n2 + 4, 5);
                // Free-list: single link resolving in one step.
                c.bus.write32(stream + 0x5c, EAF_REQ);
                c.bus.write32(EAF_REQ + 0xc, stream + 0x60);
                c.bus.write32(stream + 0x60, stream + 0x60);
                // Buffer table: empty or one bounded entry.
                c.bus
                    .write32(stream + 0x28, [0u32, 1][seed as usize / 3 % 2]);
                c.bus.write32(stream + 0x24, DATA + 0x900);
                c.bus.write32(DATA + 0x900 + 8, 0);
                // Cursor words kept in sync so the fix-up walk is skipped
                // (it needs another thread's writes to terminate).
                c.bus.write32(stream + 0x50, 0x9200 + seed % 3);
                c.bus.write32(stream + 0x54, 0x9200 + seed % 3);
                c.bus
                    .write32(stream + 0x38, [2u32, 0x9300][seed as usize / 6 % 2]);
                c.bus
                    .write32(stream + 0x30, [stream, 0][seed as usize / 12 % 2]);
                c.bus.write32(stream + 0x2c, 0x9400 + seed % 3);
                c.g[A0][0] = stream as u64;
                c.pc = 0x003e_1110;
            },
        );
    }

    driven!(
        gen_stream_file_get_read_state,
        ee::s14::f_003e14f8__generated
    );
    driven!(hand_stream_file_get_read_state, stream_file_get_read_state);

    /// Invalid streams; present and absent epoch words.
    #[test]
    fn stream_file_get_read_state_matches() {
        check(
            gen_stream_file_get_read_state,
            hand_stream_file_get_read_state,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_14f8, 0);
                ea_install_oc1124_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 3 == 0) as u32);
                c.bus.write32(stream, stream);
                c.bus.write32(stream + 0x38, 0x1234 + seed % 5);
                c.g[A0][0] = stream as u64;
                c.pc = 0x003e_14f8;
            },
        );
    }

    driven!(
        gen_stream_file_get_buffer_size,
        ee::s14::f_003e1580__generated
    );
    driven!(
        hand_stream_file_get_buffer_size,
        stream_file_get_buffer_size
    );

    /// Invalid streams; cursor pairs across the wrap point.
    #[test]
    fn stream_file_get_buffer_size_matches() {
        check(
            gen_stream_file_get_buffer_size,
            hand_stream_file_get_buffer_size,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1580, 0);
                ea_install_oc1124_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 3 == 0) as u32);
                c.bus.write32(stream, stream);
                c.bus
                    .write32(stream + 0x2c, 0x1000 + (seed / 3 % 7) * 0x100);
                c.bus
                    .write32(stream + 0x34, 0x1000 + (seed / 21 % 7) * 0x100);
                c.g[A0][0] = stream as u64;
                c.pc = 0x003e_1580;
            },
        );
    }

    driven!(
        gen_stream_file_get_buffered_bytes,
        ee::s14::f_003e15b8__generated
    );
    driven!(
        hand_stream_file_get_buffered_bytes,
        stream_file_get_buffered_bytes
    );

    /// Invalid streams; empty and stocked level words.
    #[test]
    fn stream_file_get_buffered_bytes_matches() {
        check(
            gen_stream_file_get_buffered_bytes,
            hand_stream_file_get_buffered_bytes,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_15b8, 0);
                ea_install_oc1124_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 3 == 0) as u32);
                c.bus.write32(stream, stream);
                c.bus
                    .write32(stream + 0x4c, [0u32, 5, 0x100][seed as usize / 3 % 3]);
                c.g[A0][0] = stream as u64;
                c.pc = 0x003e_15b8;
            },
        );
    }

    driven!(
        gen_stream_file_get_chunk_request,
        ee::s14::f_003e15e8__generated
    );
    driven!(
        hand_stream_file_get_chunk_request,
        stream_file_get_chunk_request
    );

    /// Invalid streams; empty and populated subscriber lists.
    #[test]
    fn stream_file_get_chunk_request_matches() {
        check(
            gen_stream_file_get_chunk_request,
            hand_stream_file_get_chunk_request,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_15e8, 0);
                ea_install_oc1124_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 4 == 0) as u32);
                c.bus.write32(stream, stream);
                c.bus
                    .write32(stream + 0x8, [0u32, 1][seed as usize / 4 % 2]);
                c.bus.write32(stream + 0x50, 0x9500 + seed % 3);
                c.bus.write32(stream + 0x5c, 0x9600 + seed % 3);
                // Subscriber node: +0xc link, +0x4 serial.
                let node = DATA + 0x500;
                c.bus.write32(stream + 0x4, node);
                c.bus.write32(node, 0x1234);
                c.bus
                    .write32(node + 4, [0u32, 1, 2][seed as usize / 12 % 3]);
                c.bus.write32(node + 8, 0x2000 + seed % 5);
                c.bus
                    .write32(node + 0xc, if seed / 36 % 2 == 0 { 0 } else { node });
                c.bus.write32(node + 0x120, 0x9700 + seed % 3);
                c.bus.write32(stream + 0x54, 0x9800 + seed % 3);
                c.g[A0][0] = stream as u64;
                c.pc = 0x003e_15e8;
            },
        );
    }

    driven!(
        gen_stream_file_get_request_status,
        ee::s14::f_003e16b0__generated
    );
    driven!(
        hand_stream_file_get_request_status,
        stream_file_get_request_status
    );

    /// Invalid streams; missing and live requests.
    #[test]
    fn stream_file_get_request_status_matches() {
        check(
            gen_stream_file_get_request_status,
            hand_stream_file_get_request_status,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_16b0, 0);
                ea_install_oc1124_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 4 == 0) as u32);
                c.bus.write32(stream, stream);
                let table = DATA + 0x700;
                c.bus
                    .write32(stream + 0x18, [0u32, 1, 4][seed as usize / 4 % 3]);
                c.bus.write32(stream + 0x14, table);
                c.bus.write32(table, (seed / 12 % 200) as u32);
                c.bus.write32(table + 4, 0x4321 + seed % 5);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = (seed / 60 % 200) as u64;
                c.pc = 0x003e_16b0;
            },
        );
    }

    driven!(gen_ea_file_exists_default, ee::s14::f_003e1700__generated);
    driven!(hand_ea_file_exists_default, ea_file_exists_default);

    /// The default-settings word selects null vs live queue results.
    #[test]
    fn ea_file_exists_default_matches() {
        check(
            gen_ea_file_exists_default,
            hand_ea_file_exists_default,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1700, 0);
                ea_install_oc1124_fakes();
                c.bus.write32(0x0045_0c38, [0u32, 1][seed as usize % 2]);
                c.bus.write32(EAF_RET_TYPE, (seed / 2 % 2) as u32);
                c.bus.write32(EAF_RET_ALLOC, 0);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                // QueueExists resolves a null alloc to the flags word
                // 0x800000 at [0]; pre-complete that pseudo-request so the
                // real WaitRequest exits immediately instead of yielding
                // forever (no other thread can complete it at unit level).
                // WaitRequest's S1 is NOT the request pointer: its prologue
                // sets S1=[0x519AD8]+top_byte(A0), so the pool word is
                // pinned to 0x800000 to land S1 on the pre-completed block
                // (a zero pool word spins in EAThread_Yield forever).
                c.bus.write32(0, 0);
                c.bus.write32(0x0080_0008, 1);
                c.bus.write32(0x0051_9AD8, 0x0080_0000);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.pc = 0x003e_1700;
            },
        );
    }

    driven!(
        gen_ea_file_get_file_size_worker,
        ee::s14::f_003e1728__generated
    );
    driven!(
        hand_ea_file_get_file_size_worker,
        ea_file_get_file_size_worker
    );

    /// Null and live opens; the size callback is faked to null.
    #[test]
    fn ea_file_get_file_size_worker_matches() {
        check(
            gen_ea_file_get_file_size_worker,
            hand_ea_file_get_file_size_worker,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1728, 0);
                ea_install_oc1124_fakes();
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_SETCB, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_1728;
            },
        );
    }

    driven!(gen_ea_file_get_file_size_ex, ee::s14::f_003e1798__generated);
    driven!(hand_ea_file_get_file_size_ex, ea_file_get_file_size_ex);

    /// The Ex form runs the worker through the priority gate.
    #[test]
    fn ea_file_get_file_size_ex_matches() {
        check(
            gen_ea_file_get_file_size_ex,
            hand_ea_file_get_file_size_ex,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1798, 0);
                ea_install_oc1124_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_SETCB, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = (seed / 4 % 7) as u64;
                c.pc = 0x003e_1798;
            },
        );
    }

    driven!(gen_ea_file_get_file_size, ee::s14::f_003e17d8__generated);
    driven!(hand_ea_file_get_file_size, ea_file_get_file_size);

    /// Same without the priority flag.
    #[test]
    fn ea_file_get_file_size_matches() {
        check(
            gen_ea_file_get_file_size,
            hand_ea_file_get_file_size,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_17d8, 0);
                ea_install_oc1124_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_SETCB, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = (seed / 4 % 7) as u64;
                c.pc = 0x003e_17d8;
            },
        );
    }

    driven!(gen_ea_file_load_file_worker, ee::s14::f_003e1810__generated);
    driven!(hand_ea_file_load_file_worker, ea_file_load_file_worker);

    /// Null opens plus pool-allocator outcomes; reads never start.
    #[test]
    fn ea_file_load_file_worker_matches() {
        check(
            gen_ea_file_load_file_worker,
            hand_ea_file_load_file_worker,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1810, 0);
                ea_install_oc1124_fakes();
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_1810;
            },
        );
    }

    driven!(gen_ea_file_load_file_ex, ee::s14::f_003e18c8__generated);
    driven!(hand_ea_file_load_file_ex, ea_file_load_file_ex);

    /// The Ex form runs the worker through the priority gate.
    #[test]
    fn ea_file_load_file_ex_matches() {
        check(
            gen_ea_file_load_file_ex,
            hand_ea_file_load_file_ex,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_18c8, 0);
                ea_install_oc1124_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_18c8;
            },
        );
    }

    driven!(gen_ea_file_load_file, ee::s14::f_003e1908__generated);
    driven!(hand_ea_file_load_file, ea_file_load_file);

    /// Same without the priority flag.
    #[test]
    fn ea_file_load_file_matches() {
        check(
            gen_ea_file_load_file,
            hand_ea_file_load_file,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1908, 0);
                ea_install_oc1124_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_1908;
            },
        );
    }

    driven!(
        gen_ea_file_load_file_get_size_worker,
        ee::s14::f_003e1948__generated
    );
    driven!(
        hand_ea_file_load_file_get_size_worker,
        ea_file_load_file_get_size_worker
    );

    /// Null opens plus pool-allocator outcomes; reads never start.
    #[test]
    fn ea_file_load_file_get_size_worker_matches() {
        check(
            gen_ea_file_load_file_get_size_worker,
            hand_ea_file_load_file_get_size_worker,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1948, 0);
                ea_install_oc1124_fakes();
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_1948;
            },
        );
    }

    driven!(
        gen_ea_file_load_file_get_size_ex,
        ee::s14::f_003e1a10__generated
    );
    driven!(
        hand_ea_file_load_file_get_size_ex,
        ea_file_load_file_get_size_ex
    );

    /// The Ex form runs the worker through the priority gate.
    #[test]
    fn ea_file_load_file_get_size_ex_matches() {
        check(
            gen_ea_file_load_file_get_size_ex,
            hand_ea_file_load_file_get_size_ex,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1a10, 0);
                ea_install_oc1124_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = (seed / 4 % 7) as u64;
                c.pc = 0x003e_1a10;
            },
        );
    }

    driven!(
        gen_ea_file_load_file_get_size,
        ee::s14::f_003e1a70__generated
    );
    driven!(hand_ea_file_load_file_get_size, ea_file_load_file_get_size);

    /// Same without the priority flag.
    #[test]
    fn ea_file_load_file_get_size_matches() {
        check(
            gen_ea_file_load_file_get_size,
            hand_ea_file_load_file_get_size,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1a70, 0);
                ea_install_oc1124_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = (seed / 4 % 7) as u64;
                c.pc = 0x003e_1a70;
            },
        );
    }

    driven!(
        gen_ea_file_load_file_into_worker,
        ee::s14::f_003e1ad0__generated
    );
    driven!(
        hand_ea_file_load_file_into_worker,
        ea_file_load_file_into_worker
    );

    /// Null opens plus short sizes; reads never start.
    #[test]
    fn ea_file_load_file_into_worker_matches() {
        check(
            gen_ea_file_load_file_into_worker,
            hand_ea_file_load_file_into_worker,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1ad0, 0);
                ea_install_oc1124_fakes();
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_1ad0;
            },
        );
    }

    driven!(
        gen_ea_file_load_file_into_ex,
        ee::s14::f_003e1b68__generated
    );
    driven!(hand_ea_file_load_file_into_ex, ea_file_load_file_into_ex);

    /// The Ex form runs the worker through the priority gate.
    #[test]
    fn ea_file_load_file_into_ex_matches() {
        check(
            gen_ea_file_load_file_into_ex,
            hand_ea_file_load_file_into_ex,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1b68, 0);
                ea_install_oc1124_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = sx(DATA + 0x600);
                c.pc = 0x003e_1b68;
            },
        );
    }

    driven!(gen_ea_file_load_file_into, ee::s14::f_003e1bb8__generated);
    driven!(hand_ea_file_load_file_into, ea_file_load_file_into);

    /// Same without the priority flag.
    #[test]
    fn ea_file_load_file_into_matches() {
        check(
            gen_ea_file_load_file_into,
            hand_ea_file_load_file_into,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1bb8, 0);
                ea_install_oc1124_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = sx(DATA + 0x600);
                c.pc = 0x003e_1bb8;
            },
        );
    }
}
