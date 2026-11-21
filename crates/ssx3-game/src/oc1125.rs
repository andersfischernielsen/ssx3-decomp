//! Hand-written replacements for `ealib/seg_2DCAE8` EAFile/BigFile calls
//! (wave `wave-ealib-1.txt` rows 101-120), each with its differential test
//! against the generated body. Moved into the source-tree units by
//! `tools/reorg_modules.py apply oc1125`.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003e_1c00,
        ea_file_load_files_worker,
        "EAFile_LoadFilesWorker",
    ),
    (0x003e_1c98, ea_file_load_files_ex, "EAFile_LoadFilesEx"),
    (0x003e_1d00, ea_file_load_files, "EAFile_LoadFiles"),
    (
        0x003e_1d68,
        ea_file_load_big_header_worker,
        "EAFile_LoadBigHeaderWorker",
    ),
    (
        0x003e_1ec8,
        ea_file_load_big_header_ex,
        "EAFile_LoadBigHeaderEx",
    ),
    (0x003e_1f08, ea_file_load_big_header, "EAFile_LoadBigHeader"),
    (
        0x003e_2130,
        ea_file_get_unpacked_size,
        "EAFile_GetUnpackedSize",
    ),
    (
        0x003e_2168,
        ea_file_read_unpacked_size,
        "EAFile_ReadUnpackedSize",
    ),
    (
        0x003e_2190,
        ea_file_load_decompressed,
        "EAFile_LoadDecompressed",
    ),
    (
        0x003e_22f0,
        ea_file_load_decompressed_thunk,
        "EAFile_LoadDecompressedThunk",
    ),
    (
        0x003e_2310,
        ea_file_load_decompressed_into,
        "EAFile_LoadDecompressedInto",
    ),
    (
        0x003e_2470,
        ea_file_load_decompressed_into_thunk,
        "EAFile_LoadDecompressedIntoThunk",
    ),
    (
        0x003e_2740,
        big_file_get_footer_version,
        "BigFile_GetFooterVersion",
    ),
    (0x003e_2ce0, big_file_find_entry, "BigFile_FindEntry"),
    (
        0x003e_2d00,
        big_file_get_entry_offset,
        "BigFile_GetEntryOffset",
    ),
    (
        0x003e_2d30,
        big_file_get_entry_offset_2,
        "BigFile_GetEntryOffset_2",
    ),
    (
        0x003e_2d60,
        big_file_get_entry_data_by_index,
        "BigFile_GetEntryDataByIndex",
    ),
    (
        0x003e_2da8,
        big_file_get_entry_data_by_index_2,
        "BigFile_GetEntryDataByIndex_2",
    ),
];

mod callee {
    use super::ee;
    use crate::call::Callee;

    pub(crate) const EA_FILE_LOAD_FILE: Callee = Callee {
        addr: 0x003e_1908,
        f: ee::s14::EAFile_LoadFile,
    };
    pub(crate) const EA_FILE_LOAD_FILE_INTO: Callee = Callee {
        addr: 0x003e_1bb8,
        f: ee::s14::EAFile_LoadFileInto,
    };
    pub(crate) const EA_FILE_LOAD_FILE_GET_SIZE: Callee = Callee {
        addr: 0x003e_1a70,
        f: ee::s14::EAFile_LoadFileGetSize,
    };
    pub(crate) const EA_FILE_CALL_WITH_PRIORITY: Callee = Callee {
        addr: 0x003d_ddf0,
        f: ee::s14::EAFile_CallWithPriority,
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
    pub(crate) const BIG_FILE_GET_TYPE: Callee = Callee {
        addr: 0x003e_2500,
        f: ee::s14::BigFile_GetType,
    };
    pub(crate) const BIG_FILE_GET_HEADER_SIZE: Callee = Callee {
        addr: 0x003e_2580,
        f: ee::s14::BigFile_GetHeaderSize,
    };
    pub(crate) const BIG_FILE_GET_FOOTER: Callee = Callee {
        addr: 0x003e_2610,
        f: ee::s14::BigFile_GetFooter,
    };
    pub(crate) const EA_FILE_BIG_FIND_ENTRY: Callee = Callee {
        addr: 0x003e_2768,
        f: ee::s14::EAFile_BigFindEntry,
    };
    pub(crate) const BIG_FILE_FIND_ENTRY: Callee = Callee {
        addr: 0x003e_2ce0,
        f: ee::s14::BigFile_FindEntry,
    };
    /// Intra-module calls: the generated bodies call the raw bodies, so
    /// both sides run the generated code here.
    pub(crate) const EA_FILE_LOAD_DECOMPRESSED: Callee = Callee {
        addr: 0x003e_2190,
        f: ee::s14::EAFile_LoadDecompressed,
    };
    pub(crate) const EA_FILE_LOAD_DECOMPRESSED_INTO: Callee = Callee {
        addr: 0x003e_2310,
        f: ee::s14::EAFile_LoadDecompressedInto,
    };
    pub(crate) const EA_MEM_COPY: Callee = Callee {
        addr: 0x003e_6574,
        f: ee::s14::EA_MemCopy,
    };
}

/// `u32 EAFile_LoadFilesWorker(list, ...)` (0x3e1c00): loads every file
/// in a count-prefixed list, recording each buffer; stops early on a
/// null buffer and reports whether all loads succeeded.
pub fn ea_file_load_files_worker(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003e_1c30;
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = 0;
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = 0x1;
        if !empty {
            c.cycles += 1;
            loop {
                c.cycles += 17;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[RA][0] = sx(0x003e_1c4c);
                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                callee::EA_FILE_LOAD_FILE.call(c, 0x003e_1c4c)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                if c.g[V1][0] == 0 {
                    c.g[S3][0] = 0;
                }
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFilesEx(count, ...)` (0x3e1c98): file-list load
/// through the priority-call gate.
pub fn ea_file_load_files_ex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        c.g[T0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[T1][0] = c.g[A2][0];
        let empty = (c.g[T0][0] as i64) <= 0;
        c.g[T2][0] = c.g[A3][0];
        if empty {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 16;
            c.g[V0][0] = 0x450000;
            c.g[V1][0] = 0x1;
            c.sw(c.g[SP][0] as u32, c.g[T0][0]);
            c.g[A0][0] = 0x3e0000;
            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c00));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A1][0]);
            c.g[A1][0] = 0;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[T1][0]);
            c.g[A3][0] = c.g[SP][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[T2][0]);
            c.g[RA][0] = sx(0x003e_1ce4);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
            callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1ce4)?;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFiles(count, ...)` (0x3e1d00): same without the
/// priority flag.
pub fn ea_file_load_files(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[T0][0] = c.g[A1][0];
        c.g[T1][0] = c.g[A2][0];
        let empty = (c.g[V1][0] as i64) <= 0;
        c.g[T2][0] = c.g[A3][0];
        if empty {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 15;
            c.g[V0][0] = 0x450000;
            c.g[A0][0] = 0x3e0000;
            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c00));
            c.sw(c.g[SP][0] as u32, c.g[V1][0]);
            c.g[A1][0] = 0;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[T0][0]);
            c.g[A3][0] = c.g[SP][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[T1][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[T2][0]);
            c.g[RA][0] = sx(0x003e_1d4c);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0);
            callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1d4c)?;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadBigHeaderWorker(path, ...)` (0x3e1d68): opens the
/// file, allocates a header buffer and reads the big-file header into
/// it; null on any failure.
pub fn ea_file_load_big_header_worker(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_F8: u32 = 0x003e_1df8;
        c.cycles += 17;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        c.g[A3][0] = c.g[SP][0];
        frame.save(c, S3, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S2, 0x50);
        c.g[A1][0] = 0x1;
        frame.save(c, S4, 0x30);
        c.g[A2][0] = c.g[S0][0];
        frame.save(c, S5, 0x20);
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003e_1da4);
        c.g[A0][0] = c.lw(c.g[S3][0] as u32);
        callee::EA_FILE_OPEN.call(c, 0x003e_1da4)?;
        let failed = c.g[V0][0] == 0;
        c.g[V0][0] = 0x450000;
        let mut done = false;
        if failed {
            // Open failed: report null.
        } else {
            c.cycles += 9;
            c.g[A0][0] = c.lw(c.g[S3][0] as u32);
            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc10));
            c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4));
            let alloc = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003e_1dc4);
            c.g[A1][0] = 0xa90;
            c.pc = alloc;
            call_ptr(c, alloc, 0x003e_1dc4)?;
            c.g[S1][0] = c.g[V0][0];
            let no_mem = c.g[S1][0] == 0;
            c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            if no_mem {
                c.cycles += 2;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                c.g[RA][0] = sx(0x003e_1e98);
                callee::EA_FILE_CLOSE.call(c, 0x003e_1e98)?;
            } else {
                c.cycles += 10;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[S4][0] = c.g[T0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x003e_1de8);
                c.g[A3][0] = 0xa90;
                callee::EA_FILE_READ.call(c, 0x003e_1de8)?;
                c.g[RA][0] = sx(0x003e_1df0);
                c.g[A0][0] = c.g[S1][0];
                callee::BIG_FILE_GET_TYPE.call(c, 0x003e_1df0)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x003e_1e18);
                    c.g[A0][0] = c.g[S1][0];
                    callee::BIG_FILE_GET_HEADER_SIZE.call(c, 0x003e_1e18)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 2705) as u64;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    if c.g[V0][0] != 0 {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x003e_1e88);
                        c.g[A1][0] = c.g[S4][0];
                        callee::EA_FILE_CLOSE.call(c, 0x003e_1e88)?;
                        c.g[V0][0] = c.g[S1][0];
                        done = true;
                    } else {
                        c.cycles += 8;
                        c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = c.g[S2][0];
                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4));
                        let alloc2 = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x003e_1e3c);
                        c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                        c.pc = alloc2;
                        call_ptr(c, alloc2, 0x003e_1e3c)?;
                        c.g[S0][0] = c.g[V0][0];
                        let no_mem2 = c.g[S0][0] == 0;
                        c.g[A1][0] = c.g[S1][0];
                        if no_mem2 {
                            if event_due(c) {
                                deliver_event_here(c, HEAD_F8);
                            }
                            // Retry from the free-callback block below.
                        } else {
                            c.cycles += 14;
                            c.g[A2][0] = 0xa90;
                            c.g[RA][0] = sx(0x003e_1e54);
                            c.g[A0][0] = c.g[S0][0];
                            callee::EA_MEM_COPY.call(c, 0x003e_1e54)?;
                            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.g[S1][0];
                            let free = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x003e_1e64);
                            c.g[S1][0] = c.g[S0][0];
                            c.pc = free;
                            call_ptr(c, free, 0x003e_1e64)?;
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                            c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_sub(0xa90));
                            c.g[A1][0] = 0xa90;
                            c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0xa90));
                            c.g[RA][0] = sx(0x003e_1e7c);
                            c.g[T0][0] = c.g[S4][0];
                            callee::EA_FILE_READ.call(c, 0x003e_1e7c)?;
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x003e_1e88);
                            c.g[A1][0] = c.g[S4][0];
                            callee::EA_FILE_CLOSE.call(c, 0x003e_1e88)?;
                            c.g[V0][0] = c.g[S1][0];
                            done = true;
                        }
                        if !done {
                            // Free the first buffer and close out.
                            c.cycles += 6;
                            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                            let free = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x003e_1e04);
                            c.g[A0][0] = c.g[S1][0];
                            c.pc = free;
                            call_ptr(c, free, 0x003e_1e04)?;
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                            c.g[A1][0] = c.g[S4][0];
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x003e_1e98);
                            callee::EA_FILE_CLOSE.call(c, 0x003e_1e98)?;
                        }
                    }
                } else {
                    // Not a big file: free the buffer and close out.
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                    let free = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x003e_1e04);
                    c.g[A0][0] = c.g[S1][0];
                    c.pc = free;
                    call_ptr(c, free, 0x003e_1e04)?;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A1][0] = c.g[S4][0];
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x003e_1e98);
                    callee::EA_FILE_CLOSE.call(c, 0x003e_1e98)?;
                }
            }
        }
        if !done {
            c.cycles += 1;
            c.g[V0][0] = 0;
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

/// `u32 EAFile_LoadBigHeaderEx(path, ...)` (0x3e1ec8): header load
/// through the priority-call gate.
pub fn ea_file_load_big_header_ex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x450000;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[V1][0] = 0x1;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.g[A0][0] = 0x3e0000;
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d68));
        frame.save_ra(c, 0x20);
        c.g[A1][0] = 0;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[RA][0] = sx(0x003e_1efc);
        c.g[A3][0] = c.g[SP][0];
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1efc)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadBigHeader(path, ...)` (0x3e1f08): same without the
/// priority flag.
pub fn ea_file_load_big_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x450000;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[A3][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.g[A0][0] = 0x3e0000;
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d68));
        frame.save_ra(c, 0x20);
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x003e_1f38);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0);
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1f38)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadDecompressed(path, ...)` (0x3e2190): loads a
/// file, checks its unpacked size and inflates it into a fresh buffer.
pub fn ea_file_load_decompressed(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_98: u32 = 0x003e_2298;
        c.cycles += 18;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S5, 0x30);
        frame.save(c, S1, 0x70);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[S5][0] ^ 0x100;
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x80);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, S6, 0x20);
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003e_21cc);
        c.g[A2][0] = c.g[S1][0];
        callee::EA_FILE_LOAD_FILE_GET_SIZE.call(c, 0x003e_21cc)?;
        c.g[S0][0] = c.g[V0][0];
        let no_mem = c.g[S0][0] == 0;
        c.g[V0][0] = 0;
        'b_3e22c4: {
            if !no_mem {
                'b_3e22c0: {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003e_21e0);
                    c.g[A0][0] = c.g[S0][0];
                    let hook = 0x003b_4818;
                    c.pc = hook;
                    call_ptr(c, hook, 0x003e_21e0)?;
                    c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    let no_size = c.g[A1][0] == 0;
                    c.g[S4][0] = c.g[V0][0];
                    if !no_size {
                        c.cycles += 2;
                        let empty = c.g[S4][0] == 0;
                        c.g[V0][0] = 0x450000;
                        if !empty {
                            'b_3e222c: {
                                c.cycles += 9;
                                c.g[A2][0] = c.g[S1][0];
                                c.g[S6][0] = c.g[V0][0];
                                c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc10));
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                                let alloc = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x003e_220c);
                                c.g[A0][0] = c.g[S2][0];
                                c.pc = alloc;
                                call_ptr(c, alloc, 0x003e_220c)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[A2][0] = c.lw(c.g[SP][0] as u32);
                                if c.g[S1][0] != 0 {
                                    break 'b_3e222c;
                                }
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                                let free = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x003e_2224);
                                c.g[A0][0] = c.g[S0][0];
                                c.pc = free;
                                call_ptr(c, free, 0x003e_2224)?;
                                c.g[V0][0] = 0;
                                break 'b_3e22c4;
                            }
                            'b_3e2288: {
                                'b_3e2284: {
                                    c.cycles += 14;
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x003e_2238);
                                    c.g[A0][0] = c.g[S1][0];
                                    callee::EA_MEM_COPY.call(c, 0x003e_2238)?;
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                                    let free = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x003e_2244);
                                    c.g[A0][0] = c.g[S0][0];
                                    c.pc = free;
                                    call_ptr(c, free, 0x003e_2244)?;
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A1][0] = c.g[S4][0];
                                    let free2 = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x003e_2258);
                                    c.g[A2][0] = c.g[S5][0];
                                    c.pc = free2;
                                    call_ptr(c, free2, 0x003e_2258)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    let no_buf = c.g[S0][0] == 0;
                                    c.g[A0][0] = c.g[S1][0];
                                    if no_buf {
                                        break 'b_3e2284;
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x003e_226c);
                                    c.g[A1][0] = c.g[S0][0];
                                    let hook = 0x003b_4740;
                                    c.pc = hook;
                                    call_ptr(c, hook, 0x003e_226c)?;
                                    let ok = c.g[V0][0] != 0;
                                    c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0xc10));
                                    if ok {
                                        break 'b_3e2288;
                                    }
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                                    let free3 = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x003e_2284);
                                    c.g[S0][0] = 0;
                                    c.pc = free3;
                                    call_ptr(c, free3, 0x003e_2284)?;
                                    break 'b_3e2284;
                                }
                                c.cycles += 1;
                                c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0xc10));
                                break 'b_3e2288;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                            let free = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x003e_2294);
                            c.g[A0][0] = c.g[S1][0];
                            c.pc = free;
                            call_ptr(c, free, 0x003e_2294)?;
                            c.g[T0][0] = 0x0;
                            'l_3e2298: loop {
                                c.cycles += 9;
                                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x40));
                                c.g[T1][0] = ((c.g[T0][0] as i64) < 4096) as u64;
                                if c.g[T1][0] != 0 {
                                    if event_due(c) {
                                        deliver_event_here(c, HEAD_98);
                                    }
                                    continue 'l_3e2298;
                                }
                                c.cycles += 1;
                                break 'b_3e22c0;
                            }
                        }
                    }
                }
            }
            if !no_mem {
                // A1==0, S4==0 and cleanup paths converge here.
                c.cycles += 1;
                c.g[V0][0] = c.g[S0][0];
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadDecompressedInto(path, buf, ...)` (0x3e2310):
/// loads a file and inflates it into the caller's buffer.
pub fn ea_file_load_decompressed_into(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_418: u32 = 0x003e_2418;
        c.cycles += 18;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S3, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S6, 0x20);
        c.g[S3][0] = c.g[A2][0];
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S4, 0x40);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, S5, 0x30);
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003e_234c);
        c.g[A2][0] = 0x100;
        callee::EA_FILE_LOAD_FILE_GET_SIZE.call(c, 0x003e_234c)?;
        c.g[S1][0] = c.g[V0][0];
        let no_mem = c.g[S1][0] == 0;
        c.g[V0][0] = 0;
        'b_3e2444: {
            if !no_mem {
                'b_3e23e0: {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x003e_2364);
                    c.g[S5][0] = 0x450000;
                    let hook = 0x003b_4818;
                    c.pc = hook;
                    call_ptr(c, hook, 0x003e_2364)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    let empty = (c.g[V0][0] as i64) <= 0;
                    if empty {
                        break 'b_3e23e0;
                    }
                    c.cycles += 2;
                    let bad_size = (c.g[S2][0] as i64) <= 0;
                    c.g[S4][0] = sx((c.g[S5][0] as u32).wrapping_add(0xc10));
                    if bad_size {
                        break 'b_3e23e0;
                    }
                    'b_3e23c0: {
                        'b_3e23bc: {
                            c.cycles += 8;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = c.g[S2][0];
                            let alloc = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x003e_2390);
                            c.g[A2][0] = 0x100;
                            c.pc = alloc;
                            call_ptr(c, alloc, 0x003e_2390)?;
                            c.g[S0][0] = c.g[V0][0];
                            let no_buf = c.g[S0][0] == 0;
                            c.g[A0][0] = c.g[S1][0];
                            if no_buf {
                                break 'b_3e23bc;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x003e_23a4);
                            c.g[A1][0] = c.g[S0][0];
                            let hook = 0x003b_4740;
                            c.pc = hook;
                            call_ptr(c, hook, 0x003e_23a4)?;
                            let ok = c.g[V0][0] != 0;
                            c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0xc10));
                            if ok {
                                break 'b_3e23c0;
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                            let free = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x003e_23bc);
                            c.g[S0][0] = 0;
                            c.pc = free;
                            call_ptr(c, free, 0x003e_23bc)?;
                            break 'b_3e23bc;
                        }
                        c.cycles += 1;
                        c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0xc10));
                        break 'b_3e23c0;
                    }
                    'b_3e23dc: {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                        let free = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x003e_23cc);
                        c.g[A0][0] = c.g[S1][0];
                        c.pc = free;
                        call_ptr(c, free, 0x003e_23cc)?;
                        if c.g[S0][0] != 0 {
                            c.sw(c.g[SP][0] as u32, c.g[S2][0]);
                            break 'b_3e23dc;
                        }
                        c.cycles += 2;
                        c.g[V0][0] = 0;
                        break 'b_3e2444;
                    }
                    c.cycles += 1;
                    c.g[S1][0] = c.g[S0][0];
                    break 'b_3e23e0;
                }
                {
                    c.cycles += 2;
                    let empty = (c.g[S3][0] as i64) <= 0;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    if !empty {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.sw(c.g[SP][0] as u32, c.g[S3][0]);
                        }
                    }
                }
                c.cycles += 9;
                c.g[A2][0] = c.lw(c.g[SP][0] as u32);
                c.g[A0][0] = c.g[S6][0];
                c.g[RA][0] = sx(0x003e_2404);
                c.g[A1][0] = c.g[S1][0];
                callee::EA_MEM_COPY.call(c, 0x003e_2404)?;
                c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0xc10));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                let free = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003e_2414);
                c.g[A0][0] = c.g[S1][0];
                c.pc = free;
                call_ptr(c, free, 0x003e_2414)?;
                c.g[T0][0] = 0x0;
                'l_3e2418: loop {
                    c.cycles += 9;
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x40));
                    c.g[T1][0] = ((c.g[T0][0] as i64) < 4096) as u64;
                    if c.g[T1][0] != 0 {
                        if event_due(c) {
                            deliver_event_here(c, HEAD_418);
                        }
                        continue 'l_3e2418;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    break 'b_3e2444;
                }
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_GetUnpackedSize(path, ...)` (0x3e2130): loads a file
/// into a scratch buffer and reads its unpacked size; 0 on failure.
pub fn ea_file_get_unpacked_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x90);
        c.g[A2][0] = 0x80;
        frame.save_ra(c, 0x80);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_2144);
        callee::EA_FILE_LOAD_FILE_INTO.call(c, 0x003e_2144)?;
        let failed = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if !failed {
            c.cycles += 4;
            c.g[RA][0] = sx(0x003e_2154);
            c.g[A0][0] = c.g[SP][0];
            let hook = 0x003b_4818;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_2154)?;
            frame.restore_ra(c, 0x80);
        } else {
            c.cycles += 1;
            frame.restore_ra(c, 0x80);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_ReadUnpackedSize(path, ...)` (0x3e2168): same reader
/// without the null check.
pub fn ea_file_read_unpacked_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x90);
        c.g[A2][0] = 0x80;
        frame.save_ra(c, 0x80);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_217c);
        callee::EA_FILE_LOAD_FILE_INTO.call(c, 0x003e_217c)?;
        c.g[RA][0] = sx(0x003e_2184);
        c.g[A0][0] = c.g[SP][0];
        let hook = 0x003b_4818;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_2184)?;
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadDecompressedThunk(path, ...)` (0x3e22f0): tail call
/// to the worker with a fresh frame.
pub fn ea_file_load_decompressed_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_2300);
        callee::EA_FILE_LOAD_DECOMPRESSED.call(c, 0x003e_2300)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadDecompressedIntoThunk(path, ...)` (0x3e2470): tail
/// call to the into-buffer worker.
pub fn ea_file_load_decompressed_into_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_2480);
        callee::EA_FILE_LOAD_DECOMPRESSED_INTO.call(c, 0x003e_2480)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 BigFile_GetFooterVersion(...)` (0x3e2740): reads the footer
/// version word through BigFile_GetFooter.
pub fn big_file_get_footer_version(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_2758);
        c.sw(c.g[SP][0] as u32, 0);
        callee::BIG_FILE_GET_FOOTER.call(c, 0x003e_2758)?;
        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 BigFile_FindEntry(...)` (0x3e2ce0): forwards to
/// EAFile_BigFindEntry and reports its result.
pub fn big_file_find_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_2cf0);
        callee::EA_FILE_BIG_FIND_ENTRY.call(c, 0x003e_2cf0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 BigFile_GetEntryOffset(path, name)` (0x3e2d00): finds an entry
/// and reports the offset word it wrote to the out slot.
pub fn big_file_get_entry_offset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        let missing = c.g[A1][0] == 0;
        c.sw(c.g[SP][0] as u32, 0);
        if !missing {
            c.cycles += 4;
            c.g[A2][0] = 0;
            c.g[A3][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x003e_2d20);
            c.g[T0][0] = 0;
            callee::BIG_FILE_FIND_ENTRY.call(c, 0x003e_2d20)?;
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 BigFile_GetEntryOffset_2(path, name)` (0x3e2d30): same via the
/// EAFile entry point.
pub fn big_file_get_entry_offset_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        let missing = c.g[A1][0] == 0;
        c.sw(c.g[SP][0] as u32, 0);
        if !missing {
            c.cycles += 4;
            c.g[A2][0] = 0;
            c.g[A3][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x003e_2d50);
            c.g[T0][0] = 0;
            callee::EA_FILE_BIG_FIND_ENTRY.call(c, 0x003e_2d50)?;
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 BigFile_GetEntryDataByIndex(...)` (0x3e2d60): finds an entry by
/// index and returns base plus the stored offset.
pub fn big_file_get_entry_data_by_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[A1][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A3][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_2d84);
        c.g[T0][0] = 0;
        callee::BIG_FILE_FIND_ENTRY.call(c, 0x003e_2d84)?;
        let found = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
        if found {
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 BigFile_GetEntryDataByIndex_2(...)` (0x3e2da8): same via the
/// EAFile entry point.
pub fn big_file_get_entry_data_by_index_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[A1][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A3][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_2dcc);
        c.g[T0][0] = 0;
        callee::EA_FILE_BIG_FIND_ENTRY.call(c, 0x003e_2dcc)?;
        let found = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
        if found {
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
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
    fn put_ascii(c: &mut Cpu, s: &str, addr: u32) {
        for (i, &b) in s.as_bytes().iter().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    /// Magic-gated fake control block and request/result cells.
    pub(crate) const EAF_CTRL: u32 = DATA + 0x2_0000;
    pub(crate) const EAF_MAGIC: u32 = 0xeaf0_0001;
    pub(crate) const EAF_RET_TYPE: u32 = DATA + 0x2_0010;
    pub(crate) const EAF_RET_ALLOC: u32 = DATA + 0x2_0030;
    pub(crate) const EAF_RET_SVCQ: u32 = DATA + 0x2_003c;

    /// A zeroed block with selectable header fields.
    fn stream_block(c: &mut Cpu, base: u32) {
        for i in 0..64u32 {
            c.bus.write32(base + 4 * i, 0);
        }
        c.bus.write32(EAF_CTRL, EAF_MAGIC);
    }

    macro_rules! ea_fake_ret {
        ($name:ident, $gen:expr, $cell:expr, $id:expr) => {
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
        ea_fake_queue_open,
        ee::s14::f_003dd4e8__generated,
        EAF_RET_TYPE,
        0xeaf0_2005
    );
    ea_fake_ret!(
        ea_fake_queue_close,
        ee::s14::f_003dd5a0__generated,
        EAF_RET_TYPE,
        0xeaf0_2006
    );
    ea_fake_ret!(
        ea_fake_queue_read,
        ee::s14::f_003dd648__generated,
        EAF_RET_TYPE,
        0xeaf0_2007
    );
    ea_fake_ret!(
        ea_fake_alloc_request,
        ee::s14::f_003de670__generated,
        EAF_RET_ALLOC,
        0xeaf0_2000
    );
    ea_fake_ret!(
        ea_fake_alloc_file_handle,
        ee::s14::f_003de800__generated,
        EAF_RET_ALLOC,
        0xeaf0_2001
    );
    ea_fake_ret!(
        ea_fake_service_queue,
        ee::s14::f_003ddfa8__generated,
        EAF_RET_SVCQ,
        0xeaf0_2017
    );

    pub(crate) fn ea_fake_mutex_lock(c: &mut Cpu) {
        // Unconditional: the real lock spins on fixture data.
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0xeaf0_2010, ra);
        c.g[V0][0] = 0;
    }

    pub(crate) fn ea_fake_mutex_unlock(c: &mut Cpu) {
        // Unconditional: see `ea_fake_mutex_lock`.
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0xeaf0_2011, ra);
        c.g[V0][0] = 0;
    }

    pub(crate) fn ea_install_oc1125_fakes() {
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

    /// Null queue/alloc/service results so the real Open/Close/Read
    /// workers take their early exits.
    fn ea_null_results(c: &mut Cpu) {
        c.bus.write32(EAF_RET_TYPE, 0);
        c.bus.write32(EAF_RET_ALLOC, 0);
        c.bus.write32(EAF_RET_SVCQ, 0);
        c.bus.write32(EAF_CTRL, EAF_MAGIC);
    }

    driven!(
        gen_ea_file_load_files_worker,
        ee::s14::f_003e1c00__generated
    );
    driven!(hand_ea_file_load_files_worker, ea_file_load_files_worker);

    /// Empty, single and multi-entry lists; null buffers clear the flag.
    /// Layout: count, path-pointer array, out stride, LoadFile A1.
    #[test]
    fn ea_file_load_files_worker_matches() {
        check(
            gen_ea_file_load_files_worker,
            hand_ea_file_load_files_worker,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1c00, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                let list = DATA + 0x400;
                let count = [0u32, 1, 2][seed as usize % 3];
                c.bus.write32(list, count);
                c.bus.write32(list + 4, DATA + 0x700);
                for i in 0..2u32 {
                    put_ascii(c, "f.dat", DATA + 0x500 + i * 0x40);
                    c.bus.write32(DATA + 0x700 + i * 4, DATA + 0x500 + i * 0x40);
                }
                c.bus
                    .write32(list + 8, [0u32, 0x100][seed as usize / 3 % 2]);
                c.bus.write32(list + 12, seed as u32 / 6 % 2);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(list);
                c.pc = 0x003e_1c00;
            },
        );
    }

    driven!(gen_ea_file_load_files_ex, ee::s14::f_003e1c98__generated);
    driven!(hand_ea_file_load_files_ex, ea_file_load_files_ex);

    /// The Ex form runs the worker through the priority gate.
    #[test]
    fn ea_file_load_files_ex_matches() {
        check(
            gen_ea_file_load_files_ex,
            hand_ea_file_load_files_ex,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1c98, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                put_ascii(c, "f.dat", DATA + 0x400);
                c.g[A0][0] = (seed % 5) as u64;
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x500);
                c.g[A3][0] = sx(DATA + 0x600);
                c.pc = 0x003e_1c98;
            },
        );
    }

    driven!(gen_ea_file_load_files, ee::s14::f_003e1d00__generated);
    driven!(hand_ea_file_load_files, ea_file_load_files);

    /// Same without the priority flag.
    #[test]
    fn ea_file_load_files_matches() {
        check(
            gen_ea_file_load_files,
            hand_ea_file_load_files,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1d00, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                put_ascii(c, "f.dat", DATA + 0x400);
                c.g[A0][0] = (seed % 5) as u64;
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x500);
                c.g[A3][0] = sx(DATA + 0x600);
                c.pc = 0x003e_1d00;
            },
        );
    }

    driven!(
        gen_ea_file_load_big_header_worker,
        ee::s14::f_003e1d68__generated
    );
    driven!(
        hand_ea_file_load_big_header_worker,
        ea_file_load_big_header_worker
    );

    /// Null opens report null; the settings/allocator words select the
    /// header-size and free-callback slots.
    #[test]
    fn ea_file_load_big_header_worker_matches() {
        check(
            gen_ea_file_load_big_header_worker,
            hand_ea_file_load_big_header_worker,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1d68, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c10, 0x450000 + seed % 3);
                let hdr = DATA + 0x400;
                stream_block(c, hdr);
                c.bus.write32(hdr + 8, 0xa90 + (seed / 3 % 3) * 4);
                put_ascii(c, "big.dat", DATA + 0x500);
                c.bus.write32(hdr, sx(DATA + 0x500) as u32);
                c.g[A0][0] = sx(hdr);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_1d68;
            },
        );
    }

    driven!(
        gen_ea_file_load_big_header_ex,
        ee::s14::f_003e1ec8__generated
    );
    driven!(hand_ea_file_load_big_header_ex, ea_file_load_big_header_ex);

    /// The Ex form runs the worker through the priority gate.
    #[test]
    fn ea_file_load_big_header_ex_matches() {
        check(
            gen_ea_file_load_big_header_ex,
            hand_ea_file_load_big_header_ex,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1ec8, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                put_ascii(c, "big.dat", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_1ec8;
            },
        );
    }

    driven!(gen_ea_file_load_big_header, ee::s14::f_003e1f08__generated);
    driven!(hand_ea_file_load_big_header, ea_file_load_big_header);

    /// Same without the priority flag.
    #[test]
    fn ea_file_load_big_header_matches() {
        check(
            gen_ea_file_load_big_header,
            hand_ea_file_load_big_header,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1f08, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                put_ascii(c, "big.dat", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_1f08;
            },
        );
    }

    driven!(
        gen_ea_file_get_unpacked_size,
        ee::s14::f_003e2130__generated
    );
    driven!(hand_ea_file_get_unpacked_size, ea_file_get_unpacked_size);

    /// Null loads report zero; live loads read the helper result.
    #[test]
    fn ea_file_get_unpacked_size_matches() {
        check(
            gen_ea_file_get_unpacked_size,
            hand_ea_file_get_unpacked_size,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2130, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                put_ascii(c, "pack.dat", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_2130;
            },
        );
    }

    driven!(
        gen_ea_file_read_unpacked_size,
        ee::s14::f_003e2168__generated
    );
    driven!(hand_ea_file_read_unpacked_size, ea_file_read_unpacked_size);

    /// Same reader without the null check.
    #[test]
    fn ea_file_read_unpacked_size_matches() {
        check(
            gen_ea_file_read_unpacked_size,
            hand_ea_file_read_unpacked_size,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2168, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                put_ascii(c, "pack.dat", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_2168;
            },
        );
    }

    driven!(
        gen_ea_file_load_decompressed,
        ee::s14::f_003e2190__generated
    );
    driven!(hand_ea_file_load_decompressed, ea_file_load_decompressed);

    /// Null size queries report null; live ones run the unpack helper
    /// and allocator/free callbacks through the jump-table fakes.
    #[test]
    fn ea_file_load_decompressed_matches() {
        check(
            gen_ea_file_load_decompressed,
            hand_ea_file_load_decompressed,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2190, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c10, 0x450000 + seed % 3);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                put_ascii(c, "pack.dat", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = [0u32, 0x100, 0x1234][seed as usize / 3 % 3] as u64;
                c.pc = 0x003e_2190;
            },
        );
    }

    driven!(
        gen_ea_file_load_decompressed_thunk,
        ee::s14::f_003e22f0__generated
    );
    driven!(
        hand_ea_file_load_decompressed_thunk,
        ea_file_load_decompressed_thunk
    );

    /// Thunk forwards to the worker.
    #[test]
    fn ea_file_load_decompressed_thunk_matches() {
        check(
            gen_ea_file_load_decompressed_thunk,
            hand_ea_file_load_decompressed_thunk,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_22f0, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c10, 0x450000 + seed % 3);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                put_ascii(c, "pack.dat", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_22f0;
            },
        );
    }

    driven!(
        gen_ea_file_load_decompressed_into,
        ee::s14::f_003e2310__generated
    );
    driven!(
        hand_ea_file_load_decompressed_into,
        ea_file_load_decompressed_into
    );

    /// Into-buffer variant with a caller size word.
    #[test]
    fn ea_file_load_decompressed_into_matches() {
        check(
            gen_ea_file_load_decompressed_into,
            hand_ea_file_load_decompressed_into,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2310, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c10, 0x450000 + seed % 3);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                put_ascii(c, "pack.dat", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = [0u32, 0x100, 0x1234][seed as usize / 3 % 3] as u64;
                c.pc = 0x003e_2310;
            },
        );
    }

    driven!(
        gen_ea_file_load_decompressed_into_thunk,
        ee::s14::f_003e2470__generated
    );
    driven!(
        hand_ea_file_load_decompressed_into_thunk,
        ea_file_load_decompressed_into_thunk
    );

    /// Thunk forwards to the into-buffer worker.
    #[test]
    fn ea_file_load_decompressed_into_thunk_matches() {
        check(
            gen_ea_file_load_decompressed_into_thunk,
            hand_ea_file_load_decompressed_into_thunk,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2470, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c10, 0x450000 + seed % 3);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                put_ascii(c, "pack.dat", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = sx(DATA + 0x600);
                c.pc = 0x003e_2470;
            },
        );
    }

    driven!(
        gen_big_file_get_footer_version,
        ee::s14::f_003e2740__generated
    );
    driven!(
        hand_big_file_get_footer_version,
        big_file_get_footer_version
    );

    /// Reads the footer version word through the footer helper.
    #[test]
    fn big_file_get_footer_version_matches() {
        check(
            gen_big_file_get_footer_version,
            hand_big_file_get_footer_version,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2740, 0);
                let block = DATA + 0x400;
                stream_block(c, block);
                c.bus
                    .write32(block + 8, [0u32, 0x38, 0x100][seed as usize % 3]);
                c.g[A0][0] = block as u64;
                c.pc = 0x003e_2740;
            },
        );
    }

    driven!(gen_big_file_find_entry, ee::s14::f_003e2ce0__generated);
    driven!(hand_big_file_find_entry, big_file_find_entry);

    /// Forwards the lookup; null and present names.
    #[test]
    fn big_file_find_entry_matches() {
        check(
            gen_big_file_find_entry,
            hand_big_file_find_entry,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2ce0, 0);
                put_ascii(c, "entry", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x400) };
                c.g[A2][0] = (seed / 3 % 7) as u64;
                c.g[A3][0] = sx(DATA + 0x500);
                c.pc = 0x003e_2ce0;
            },
        );
    }

    driven!(
        gen_big_file_get_entry_offset,
        ee::s14::f_003e2d00__generated
    );
    driven!(hand_big_file_get_entry_offset, big_file_get_entry_offset);

    /// Missing names report zero; present names report the slot.
    #[test]
    fn big_file_get_entry_offset_matches() {
        check(
            gen_big_file_get_entry_offset,
            hand_big_file_get_entry_offset,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2d00, 0);
                put_ascii(c, "entry", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x400) };
                c.pc = 0x003e_2d00;
            },
        );
    }

    driven!(
        gen_big_file_get_entry_offset_2,
        ee::s14::f_003e2d30__generated
    );
    driven!(
        hand_big_file_get_entry_offset_2,
        big_file_get_entry_offset_2
    );

    /// Same via the EAFile entry point.
    #[test]
    fn big_file_get_entry_offset_2_matches() {
        check(
            gen_big_file_get_entry_offset_2,
            hand_big_file_get_entry_offset_2,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2d30, 0);
                put_ascii(c, "entry", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x400) };
                c.pc = 0x003e_2d30;
            },
        );
    }

    driven!(
        gen_big_file_get_entry_data_by_index,
        ee::s14::f_003e2d60__generated
    );
    driven!(
        hand_big_file_get_entry_data_by_index,
        big_file_get_entry_data_by_index
    );

    /// Indexed lookups add the base to the stored offset.
    #[test]
    fn big_file_get_entry_data_by_index_matches() {
        check(
            gen_big_file_get_entry_data_by_index,
            hand_big_file_get_entry_data_by_index,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2d60, 0);
                put_ascii(c, "entry", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x700 + (seed % 3) * 0x10);
                c.g[A1][0] = (seed / 3 % 251) as u64;
                c.g[A2][0] = (seed / 9 % 7) as u64;
                c.g[A3][0] = sx(DATA + 0x500);
                c.pc = 0x003e_2d60;
            },
        );
    }

    driven!(
        gen_big_file_get_entry_data_by_index_2,
        ee::s14::f_003e2da8__generated
    );
    driven!(
        hand_big_file_get_entry_data_by_index_2,
        big_file_get_entry_data_by_index_2
    );

    /// Same via the EAFile entry point.
    #[test]
    fn big_file_get_entry_data_by_index_2_matches() {
        check(
            gen_big_file_get_entry_data_by_index_2,
            hand_big_file_get_entry_data_by_index_2,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2da8, 0);
                put_ascii(c, "entry", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x700 + (seed % 3) * 0x10);
                c.g[A1][0] = (seed / 3 % 251) as u64;
                c.g[A2][0] = (seed / 9 % 7) as u64;
                c.g[A3][0] = sx(DATA + 0x500);
                c.pc = 0x003e_2da8;
            },
        );
    }
}
