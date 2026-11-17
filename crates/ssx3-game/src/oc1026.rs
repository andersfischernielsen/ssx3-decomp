//! Staging module oc1026 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1026`.

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
    (
        0x003f_b840,
        slogin_store_map_hash_error,
        "SLoginStore_MapHashError",
    ),
    (
        0x003f_b858,
        slogin_store_get_alignment,
        "SLoginStore_GetAlignment",
    ),
    (
        0x003f_b898,
        slogin_store_calc_alloc_size,
        "SLoginStore_CalcAllocSize",
    ),
    (
        0x003f_b910,
        slogin_store_get_block_header,
        "SLoginStore_GetBlockHeader",
    ),
    (
        0x003f_b958,
        slogin_store_set_record_header,
        "SLoginStore_SetRecordHeader",
    ),
    (
        0x003f_b9a0,
        slogin_store_write_sentinel,
        "SLoginStore_WriteSentinel",
    ),
    (
        0x003f_b9d0,
        slogin_store_get_record_header,
        "SLoginStore_GetRecordHeader",
    ),
    (
        0x003f_ba20,
        slogin_store_find_record,
        "SLoginStore_FindRecord",
    ),
    (
        0x003f_ba78,
        slogin_store_set_record_crc,
        "SLoginStore_SetRecordCrc",
    ),
    (
        0x003f_bad0,
        slogin_store_add_record,
        "SLoginStore_AddRecord",
    ),
    (0x003f_bb50, slogin_store_hash, "SLoginStore_Hash"),
    (
        0x003f_bbe0,
        slogin_store_hash_directory,
        "SLoginStore_HashDirectory",
    ),
    (0x003f_bc40, slogin_store_encrypt, "SLoginStore_Encrypt"),
    (0x003f_bcd8, slogin_store_decrypt, "SLoginStore_Decrypt"),
    (
        0x003f_bd70,
        slogin_store_begin_open,
        "SLoginStore_BeginOpen",
    ),
    (
        0x003f_bdb8,
        slogin_store_seek_to_start,
        "SLoginStore_SeekToStart",
    ),
    (
        0x003f_bdf0,
        slogin_store_read_dir_header,
        "SLoginStore_ReadDirHeader",
    ),
    (
        0x003f_be40,
        slogin_store_load_dir_entry,
        "SLoginStore_LoadDirEntry",
    ),
    (
        0x003f_bed0,
        slogin_store_write_record,
        "SLoginStore_WriteRecord",
    ),
    (
        0x003f_bf80,
        slogin_store_read_record,
        "SLoginStore_ReadRecord",
    ),
];

/// `int SLoginStore_MapHashError(int hash_error)` (0x3fb840): translate a
/// checksum service error into the login store's error range.
pub fn slogin_store_map_hash_error(c: &mut Cpu) {
    c.cycles += 2;
    let hash_error = c.g[A0][0];
    c.g[V0][0] = 0;
    if hash_error != 0 {
        c.cycles += 1;
        c.g[V0][0] = sx((hash_error as u32).wrapping_add(0x12c));
    }
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
pub(crate) const ALIGNMENT_DISPATCH: u32 = 0x0049_7580;

/// `void SLoginStore_GetAlignment(int format, int* alignment, int* sectors)`
/// (0x3fb858): choose the record alignment and sector count for a store
/// format, writing both output words.
pub fn slogin_store_get_alignment(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A3][0] = 4;
    c.g[V0][0] = (c.g[A0][0] < 5) as u64;
    let valid_format = c.g[V0][0] != 0;
    c.g[V1][0] = 1;
    if valid_format {
        c.cycles += 7;
        c.g[V0][0] = 0x49_0000;
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7580));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        let target = c.g[A0][0] as u32;
        if target == 0x003f_b884 {
            c.cycles += 2;
            c.g[A3][0] = 4;
            c.g[V1][0] = 1;
        } else {
            c.jump_out(target, 0x003f_b858, 0x003f_b898);
            return;
        }
    }
    c.cycles += 3;
    c.sw(c.g[A1][0] as u32, c.g[A3][0]);
    c.sw(c.g[A2][0] as u32, c.g[V1][0]);
    ret(c);
}

/// `uint SLoginStore_CalcAllocSize(uint payload_bytes, int format)`
/// (0x3fb898): include the 12-byte block header and round up to the
/// format's required record alignment.
pub fn slogin_store_calc_alloc_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 4;
        frame.save(c, S0, 0x20);
        c.g[V1][0] = 1;
        c.g[S0][0] = c.g[A0][0];
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[A0][0] = c.g[A1][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(4), c.g[V1][0]);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.g[SP][0] | 4;
        Callee {
            addr: 0x003f_b858,
            f: ee::s15::f_003fb858,
        }
        .call(c, 0x003f_b8c8)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(4));
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(12));
        c.g[V0][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.g[A0][0];
        } else {
            c.cycles += 3;
            c.divu(0, 3, 4);
            c.cycles += 6;
            c.g[V1][0] = c.hi[0];
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            c.divu(0, 3, 4);
            c.g[V0][0] = c.hi[0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(12));
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

const BLOCK_HEADER_BYTES: u32 = 12;
#[cfg(test)]
pub(crate) const BLOCK_HEADER_MAGIC: u32 = 0x0049_7550;

/// `BlockHeader* SLoginStore_GetBlockHeader(void* payload, bool validate)`
/// (0x3fb910): return the 12-byte header before a record payload, rejecting
/// it when requested and its magic bytes differ from the store template.
pub fn slogin_store_get_block_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] &= 0xff;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let validate = c.g[A1][0] != 0;
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_sub(BLOCK_HEADER_BYTES));
        if validate {
            c.cycles += 6;
            c.g[A1][0] = 0x49_0000;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7550));
            c.g[A2][0] = BLOCK_HEADER_BYTES as u64;
            call_ptr(c, 0x0041_5fc8, 0x003f_b93c)?;
            if c.g[V0][0] != 0 {
                c.g[S0][0] = 0;
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void SLoginStore_SetRecordHeader(RecordHeader* header, uint size,
/// uint flags, const uint* crc)` (0x3fb958): write size, flags, and the CRC
/// word into the 12-byte record header with the guest's unaligned copies.
pub fn slogin_store_set_record_header(c: &mut Cpu) {
    c.cycles += 17;
    let frame = Frame::open(c, 0x10);
    c.sw(c.g[SP][0] as u32, c.g[A1][0]);
    c.sw((c.g[SP][0] as u32).wrapping_add(4), c.g[A2][0]);
    c.lwl(V0, (c.g[SP][0] as u32).wrapping_add(3));
    c.lwr(V0, c.g[SP][0] as u32);
    c.swl(V0, (c.g[A0][0] as u32).wrapping_add(3));
    c.swr(V0, c.g[A0][0] as u32);
    c.lwl(V0, (c.g[SP][0] as u32).wrapping_add(7));
    c.lwr(V0, (c.g[SP][0] as u32).wrapping_add(4));
    c.swl(V0, (c.g[A0][0] as u32).wrapping_add(7));
    c.swr(V0, (c.g[A0][0] as u32).wrapping_add(4));
    c.lwl(V0, (c.g[A3][0] as u32).wrapping_add(3));
    c.lwr(V0, c.g[A3][0] as u32);
    c.swl(V0, (c.g[A0][0] as u32).wrapping_add(11));
    c.swr(V0, (c.g[A0][0] as u32).wrapping_add(8));
    frame.close(c);
    ret(c);
}

/// `void SLoginStore_WriteSentinel(void* block)` (0x3fb9a0): write the
/// store's 12-byte sentinel pattern at the start of an allocated block.
pub fn slogin_store_write_sentinel(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0x49_0000;
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7550));
    c.ldl(V1, (c.g[A3][0] as u32).wrapping_add(7));
    c.ldr(V1, c.g[A3][0] as u32);
    c.lwl(A1, (c.g[A3][0] as u32).wrapping_add(11));
    c.lwr(A1, (c.g[A3][0] as u32).wrapping_add(8));
    c.sdl(V1, (c.g[A0][0] as u32).wrapping_add(7));
    c.sdr(V1, c.g[A0][0] as u32);
    c.swl(A1, (c.g[A0][0] as u32).wrapping_add(11));
    c.swr(A1, (c.g[A0][0] as u32).wrapping_add(8));
    ret(c);
}

/// `void SLoginStore_GetRecordHeader(const RecordHeader* header, uint* size,
/// uint* flags, uint* crc)` (0x3fb9d0): copy each requested record-header
/// word to its optional output pointer.
pub fn slogin_store_get_record_header(c: &mut Cpu) {
    c.cycles += 2;
    if c.g[A1][0] != 0 {
        c.cycles += 4;
        c.lwl(V0, (c.g[A0][0] as u32).wrapping_add(3));
        c.lwr(V0, c.g[A0][0] as u32);
        c.swl(V0, (c.g[A1][0] as u32).wrapping_add(3));
        c.swr(V0, c.g[A1][0] as u32);
    }
    c.cycles += 2;
    if c.g[A2][0] != 0 {
        c.cycles += 4;
        c.lwl(V0, (c.g[A0][0] as u32).wrapping_add(7));
        c.lwr(V0, (c.g[A0][0] as u32).wrapping_add(4));
        c.swl(V0, (c.g[A2][0] as u32).wrapping_add(3));
        c.swr(V0, c.g[A2][0] as u32);
    }
    c.cycles += 2;
    if c.g[A3][0] != 0 {
        c.cycles += 4;
        c.lwl(T1, (c.g[A0][0] as u32).wrapping_add(11));
        c.lwr(T1, (c.g[A0][0] as u32).wrapping_add(8));
        c.swl(T1, (c.g[A3][0] as u32).wrapping_add(3));
        c.swr(T1, c.g[A3][0] as u32);
    }
    c.cycles += 2;
    ret(c);
}

const STORE_STATE_GP: u32 = 0x2a30;
const STORE_RECORD_COUNT: u32 = 0x94;
const STORE_RECORDS: u32 = 0x9c;
const STORE_RECORD_STRIDE: u32 = 0x10;

/// `Record* SLoginStore_FindRecord(uint kind, uint occurrence)` (0x3fba20):
/// scan the store directory for the requested occurrence of a record kind.
pub fn slogin_store_find_record(c: &mut Cpu) {
    c.cycles += 8;
    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(STORE_RECORD_COUNT));
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(STORE_RECORDS));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = (c.g[V1][0] < c.g[A2][0]) as u64;
    let has_records = c.g[V0][0] != 0;
    c.g[A3][0] = 0;
    if has_records {
        c.cycles += 2;
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        loop {
            c.cycles += 2;
            if c.g[A0][0] != c.g[V0][0] {
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(STORE_RECORD_STRIDE));
            } else {
                c.cycles += 2;
                let other_occurrence = c.g[A1][0] != c.g[A3][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                if other_occurrence {
                    c.cycles += 1;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(STORE_RECORD_STRIDE));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V1][0];
                    ret(c);
                    return;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[V1][0] < c.g[A2][0]) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            if event_due(c) {
                deliver_event_here(c, 0x003f_ba48);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

// Store-state words (the struct at `gp+0x2a30`), named after how the record
// and I/O functions below use them.
const STORE_CIPHER_KEY: u32 = 0x64;
const STORE_CIPHER_KEY_LEN: u32 = 0x68;
const STORE_CIPHER_IFACE: u32 = 0x6c;
const STORE_HASH_IFACE: u32 = 0x70;
const STORE_OPEN_ARG: u32 = 0x74;
const STORE_PENDING_OP: u32 = 0x78;
const STORE_RECORD_BUFFER: u32 = 0x7c;
const STORE_DIR_BUFFER: u32 = 0x80;
const STORE_RECORD_SIZE: u32 = 0x84;
const STORE_RECORD_CRC: u32 = 0x88;
const STORE_RECORD_KIND: u32 = 0x8c;
const STORE_RECORD_CAPACITY: u32 = 0x90;
const STORE_DATA_END: u32 = 0x98;

// Directory entry words (`STORE_RECORD_STRIDE` bytes each).
const RECORD_OFFSET: u32 = 4;
const RECORD_SIZE: u32 = 8;
const RECORD_CRC: u32 = 0xc;

// Checksum interface slots (`Crc32_GetInterface`'s table).
const HASH_BEGIN: u32 = 0xc;
const HASH_UPDATE: u32 = 0x10;
const HASH_FINAL: u32 = 0x14;

// Cipher interface slots (`SLoginStore_GetCipherIface`'s table).
const CIPHER_SET_KEY: u32 = 0xc;
const CIPHER_ENCRYPT: u32 = 0x10;
const CIPHER_DECRYPT: u32 = 0x14;
const CIPHER_RESET: u32 = 0x18;

// Memory-card layer entry points (reached through the function table).

mod callee {
    use super::*;

    pub(crate) const SLOGIN_MC_START_OPEN: Callee = Callee {
        addr: 0x003f_99b8,
        f: ee::s14::SLoginMc_StartOpen,
    };

    pub(crate) const SLOGIN_MC_START_SEEK: Callee = Callee {
        addr: 0x003f_9c48,
        f: ee::s14::SLoginMc_StartSeek,
    };

    pub(crate) const SLOGIN_MC_START_READ: Callee = Callee {
        addr: 0x003f_9dc0,
        f: ee::s14::SLoginMc_StartRead,
    };

    pub(crate) const SLOGIN_MC_START_WRITE: Callee = Callee {
        addr: 0x003f_9f00,
        f: ee::s14::SLoginMc_StartWrite,
    };

    pub(crate) const SLOGIN_STORE_MAP_MC_ERROR: Callee = Callee {
        addr: 0x003f_b810,
        f: ee::s15::SLoginStore_MapMcError,
    };
    pub(crate) const SLOGIN_STORE_MAP_CIPHER_ERROR: Callee = Callee {
        addr: 0x003f_b828,
        f: ee::s15::SLoginStore_MapCipherError,
    };
    pub(crate) const SLOGIN_STORE_MAP_HASH_ERROR: Callee = Callee {
        addr: 0x003f_b840,
        f: ee::s15::SLoginStore_MapHashError,
    };
    pub(crate) const SLOGIN_STORE_CALC_ALLOC_SIZE: Callee = Callee {
        addr: 0x003f_b898,
        f: ee::s15::SLoginStore_CalcAllocSize,
    };
    pub(crate) const SLOGIN_STORE_GET_BLOCK_HEADER: Callee = Callee {
        addr: 0x003f_b910,
        f: ee::s15::SLoginStore_GetBlockHeader,
    };
    pub(crate) const SLOGIN_STORE_SET_RECORD_HEADER: Callee = Callee {
        addr: 0x003f_b958,
        f: ee::s15::SLoginStore_SetRecordHeader,
    };
    pub(crate) const SLOGIN_STORE_GET_RECORD_HEADER: Callee = Callee {
        addr: 0x003f_b9d0,
        f: ee::s15::SLoginStore_GetRecordHeader,
    };
    pub(crate) const SLOGIN_STORE_ADD_RECORD: Callee = Callee {
        addr: 0x003f_bad0,
        f: ee::s15::SLoginStore_AddRecord,
    };
    pub(crate) const SLOGIN_STORE_HASH: Callee = Callee {
        addr: 0x003f_bb50,
        f: ee::s15::SLoginStore_Hash,
    };
    pub(crate) const SLOGIN_STORE_ENCRYPT: Callee = Callee {
        addr: 0x003f_bc40,
        f: ee::s15::SLoginStore_Encrypt,
    };
    pub(crate) const SLOGIN_STORE_DECRYPT: Callee = Callee {
        addr: 0x003f_bcd8,
        f: ee::s15::SLoginStore_Decrypt,
    };
}

/// `void SLoginStore_SetRecordCrc(uint kind, uint occurrence, const uint* crc)`
/// (0x3fba78): store `*crc` as the CRC word of the requested occurrence of a
/// record kind; the whole directory is scanned either way.
pub fn slogin_store_set_record_crc(c: &mut Cpu) {
    c.cycles += 8;
    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(STORE_RECORD_COUNT));
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(STORE_RECORDS));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = (c.g[V1][0] < c.g[A3][0]) as u64;
    let has_records = c.g[V0][0] != 0;
    c.g[T0][0] = 0;
    if has_records {
        c.cycles += 2;
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        loop {
            c.cycles += 2;
            if c.g[A0][0] != c.g[V0][0] {
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(STORE_RECORD_STRIDE));
            } else {
                c.cycles += 2;
                let other_occurrence = c.g[A1][0] != c.g[T0][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                if !other_occurrence {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    c.sw((c.g[V1][0] as u32).wrapping_add(RECORD_CRC), c.g[V0][0]);
                }
                c.cycles += 1;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(STORE_RECORD_STRIDE));
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[V1][0] < c.g[A3][0]) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            if event_due(c) {
                deliver_event_here(c, 0x003f_baa0);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `uint SLoginStore_AddRecord(uint kind, uint size, const uint* crc)`
/// (0x3fbad0): append a directory entry (kind, data offset, size, CRC) after
/// the last one, advance the store's data end by the record's 12-byte header
/// plus its size, and return the new entry's data offset. A full directory
/// returns the current data end instead.
pub fn slogin_store_add_record(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(STORE_RECORD_COUNT));
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(STORE_RECORD_CAPACITY));
    let has_room = c.g[V1][0] != c.g[V0][0];
    c.g[T0][0] = c.g[A1][0];
    if !has_room {
        c.cycles += 2;
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(STORE_DATA_END));
        ret(c);
        return;
    }
    c.cycles += 4;
    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(STORE_RECORDS));
    c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
    let first = c.g[V1][0] == 0;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    if first {
        c.cycles += 1;
        c.sw((c.g[A1][0] as u32).wrapping_add(RECORD_OFFSET), 0);
    }
    c.cycles += 17;
    c.g[A3][0] = c.g[V0][0];
    c.g[V1][0] = 1;
    c.sw(c.g[A3][0] as u32, c.g[A0][0]);
    c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(BLOCK_HEADER_BYTES));
    c.sw((c.g[A3][0] as u32).wrapping_add(RECORD_SIZE), c.g[T0][0]);
    if c.g[A1][0] == 0 {
        c.g[A1][0] = c.g[V1][0];
    }
    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
    c.sw((c.g[A3][0] as u32).wrapping_add(RECORD_CRC), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(STORE_RECORD_COUNT));
    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(STORE_RECORD_CAPACITY));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
    c.sw(
        (c.g[V1][0] as u32).wrapping_add(STORE_RECORD_COUNT),
        c.g[A0][0],
    );
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(RECORD_OFFSET));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let now_full = c.g[A0][0] == c.g[A2][0];
    c.sw((c.g[V1][0] as u32).wrapping_add(STORE_DATA_END), c.g[V0][0]);
    if !now_full {
        c.cycles += 1;
        c.sw(
            (c.g[A3][0] as u32).wrapping_add(STORE_RECORD_STRIDE + RECORD_OFFSET),
            c.g[V0][0],
        );
    }
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(RECORD_OFFSET));
    ret(c);
}

/// `int SLoginStore_Hash(const void* data, uint size, void* out)` (0x3fbb50):
/// run the store's checksum interface (begin, update over `data`, final into
/// `out`); the first failing step's error is mapped into the store's range,
/// success returns 0.
pub fn slogin_store_hash(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_HASH_IFACE));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(HASH_BEGIN));
        c.g[S3][0] = c.g[A2][0];
        let begin = c.g[V0][0] as u32;
        call_ptr(c, begin, 0x003f_bb84)?;
        let mut failed = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S1][0];
        if !failed {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(HASH_UPDATE));
            c.g[A1][0] = c.g[S2][0];
            let update = c.g[V0][0] as u32;
            call_ptr(c, update, 0x003f_bb98)?;
            failed = c.g[V0][0] != 0;
            if !failed {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(HASH_FINAL));
                c.g[A0][0] = c.g[S3][0];
                let finish = c.g[V0][0] as u32;
                call_ptr(c, finish, 0x003f_bbac)?;
                failed = c.g[V0][0] != 0;
                if !failed {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                }
            }
        }
        if failed {
            c.cycles += 2;
            c.g[A0][0] = c.g[V0][0];
            callee::SLOGIN_STORE_MAP_HASH_ERROR.call(c, 0x003f_bbc4)?;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SLoginStore_HashDirectory(uint* out)` (0x3fbbe0): checksum the
/// whole directory (16 bytes per entry) and copy the 4-byte result to `out`
/// (through the pointer at +4 of the object the final step leaves in the
/// out slot; the hash's error code is dropped); an empty directory stores 0.
pub fn slogin_store_hash_directory(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(STORE_RECORD_COUNT));
        c.g[A1][0] = sx((c.g[V0][0] as u32) << 4);
        let empty = c.g[A1][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if empty {
            c.cycles += 2;
            c.sw(c.g[S0][0] as u32, 0);
        } else {
            c.cycles += 9;
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(STORE_RECORDS));
            c.g[A2][0] = c.g[SP][0];
            callee::SLOGIN_STORE_HASH.call(c, 0x003f_bc14)?;
            c.g[V1][0] = c.lw(sp(c));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(4));
            c.lwl(V1, (c.g[V0][0] as u32).wrapping_add(3));
            c.lwr(V1, c.g[V0][0] as u32);
            c.swl(V1, (c.g[S0][0] as u32).wrapping_add(3));
            c.swr(V1, c.g[S0][0] as u32);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// One pass of the store cipher over `data` (`SLoginStore_Encrypt` and
/// `SLoginStore_Decrypt` differ only in the transform slot and their return
/// addresses: set-key, transform, reset, error map).
fn slogin_store_cipher_pass(c: &mut Cpu, transform_slot: u32, rets: [u32; 4]) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(STORE_CIPHER_IFACE));
        let no_cipher = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        let mut failed = false;
        if !no_cipher {
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(CIPHER_SET_KEY));
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(STORE_CIPHER_KEY_LEN));
            let set_key = c.g[V0][0] as u32;
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(STORE_CIPHER_KEY));
            call_ptr(c, set_key, rets[0])?;
            failed = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if !failed {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_CIPHER_IFACE));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(transform_slot));
                let transform = c.g[V0][0] as u32;
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, transform, rets[1])?;
                failed = c.g[V0][0] != 0;
                if !failed {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_CIPHER_IFACE));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(CIPHER_RESET));
                    let reset = c.g[V0][0] as u32;
                    call_ptr(c, reset, rets[2])?;
                    failed = c.g[V0][0] != 0;
                }
            }
        }
        if failed {
            c.cycles += 2;
            c.g[A0][0] = c.g[V0][0];
            callee::SLOGIN_STORE_MAP_CIPHER_ERROR.call(c, rets[3])?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_Encrypt(void* data, uint size)` (0x3fbc40): encrypt
/// `data` in place with the store cipher (set key, encrypt, reset); no
/// cipher means success. Errors are mapped into the store's range.
pub fn slogin_store_encrypt(c: &mut Cpu) {
    slogin_store_cipher_pass(
        c,
        CIPHER_ENCRYPT,
        [0x003f_bc74, 0x003f_bc90, 0x003f_bcac, 0x003f_bcc4],
    );
}

/// `int SLoginStore_Decrypt(void* data, uint size)` (0x3fbcd8): decrypt
/// `data` in place with the store cipher (set key, decrypt, reset); no
/// cipher means success. Errors are mapped into the store's range.
pub fn slogin_store_decrypt(c: &mut Cpu) {
    slogin_store_cipher_pass(
        c,
        CIPHER_DECRYPT,
        [0x003f_bd0c, 0x003f_bd28, 0x003f_bd44, 0x003f_bd5c],
    );
}

/// `int SLoginStore_BeginOpen(const char* name, int device, uint arg2,
/// uint arg3)` (0x3fbd70): start
/// opening the store file on the memory card (`SLoginMc_StartOpen` with the
/// caller's first three arguments), then record `a3` at +0x74, set pending
/// operation 4 and empty the directory. Returns the mapped card error.
pub fn slogin_store_begin_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A3][0];
        call_ptr(c, callee::SLOGIN_MC_START_OPEN.addr, 0x003f_bd84)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
        c.g[A1][0] = 4;
        c.g[A0][0] = c.g[V0][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(STORE_OPEN_ARG), c.g[S0][0]);
        c.sw(
            (c.g[V1][0] as u32).wrapping_add(STORE_PENDING_OP),
            c.g[A1][0],
        );
        c.sw((c.g[V1][0] as u32).wrapping_add(STORE_DATA_END), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(STORE_RECORD_COUNT), 0);
        callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_bda4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_SeekToStart(void)` (0x3fbdb8): set pending operation 9
/// and start a card seek of the store file to offset 0. Returns the mapped
/// card error.
pub fn slogin_store_seek_to_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
        frame.save_ra(c, 0);
        c.g[V1][0] = 9;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        c.sw(
            (c.g[V0][0] as u32).wrapping_add(STORE_PENDING_OP),
            c.g[V1][0],
        );
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::SLOGIN_MC_START_SEEK.addr, 0x003f_bddc)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_bde4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_ReadDirHeader(void)` (0x3fbdf0): set pending operation 7
/// and start a card read of one bare record header (12 bytes, rounded to
/// format 1) into the directory buffer. Returns the mapped card error.
pub fn slogin_store_read_dir_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 7;
        frame.save(c, S0, 0x10);
        c.g[A0][0] = 0;
        c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
        c.g[A1][0] = 1;
        frame.save_ra(c, 0);
        c.sw(
            (c.g[S0][0] as u32).wrapping_add(STORE_PENDING_OP),
            c.g[V0][0],
        );
        callee::SLOGIN_STORE_CALC_ALLOC_SIZE.call(c, 0x003f_be14)?;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(STORE_DIR_BUFFER));
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
        call_ptr(c, callee::SLOGIN_MC_START_READ.addr, 0x003f_be24)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_be2c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_LoadDirEntry(void)` (0x3fbe40): set pending operation 11,
/// decrypt the 12-byte record header in the directory buffer and add it to
/// the directory (its CRC through +0x88). Returns 5 when the new entry's
/// offset equals the data end (directory full), else starts a card seek to
/// the data end and returns the mapped card error; a decrypt error is
/// returned as is.
pub fn slogin_store_load_dir_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
        c.sw(sp(c).wrapping_add(4), 0);
        c.g[V1][0] = 0xb;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = BLOCK_HEADER_BYTES as u64;
        c.sw(
            (c.g[V0][0] as u32).wrapping_add(STORE_PENDING_OP),
            c.g[V1][0],
        );
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_DIR_BUFFER));
        callee::SLOGIN_STORE_DECRYPT.call(c, 0x003f_be64)?;
        let failed = c.g[V0][0] != 0;
        frame.restore_ra(c, 0x10);
        if !failed {
            c.cycles += 15;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
            c.g[A2][0] = c.g[SP][0] | 4;
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_DIR_BUFFER));
            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(STORE_RECORD_CRC));
            callee::SLOGIN_STORE_GET_RECORD_HEADER.call(c, 0x003f_be84)?;
            c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
            c.g[A1][0] = c.lw(sp(c).wrapping_add(4));
            c.g[A0][0] = c.lw(sp(c));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(STORE_RECORD_CRC));
            callee::SLOGIN_STORE_ADD_RECORD.call(c, 0x003f_be98)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
            c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(STORE_DATA_END));
            let at_end = c.g[A1][0] == c.g[V0][0];
            c.g[V0][0] = 5;
            if at_end {
                c.cycles += 1;
                frame.restore_ra(c, 0x10);
            } else {
                c.cycles += 6;
                c.g[A2][0] = 0;
                call_ptr(c, callee::SLOGIN_MC_START_SEEK.addr, 0x003f_beb0)?;
                c.g[A0][0] = c.g[V0][0];
                callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_beb8)?;
                frame.restore_ra(c, 0x10);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_WriteRecord(void)` (0x3fbed0): set pending operation 6,
/// fill the validated block header before the record buffer (kind +0x8c,
/// size +0x84, CRC +0x88), encrypt the header and then the payload, and start
/// a card write of the whole block (size rounded to format 2). Returns 4 for
/// a buffer without a valid block header, a cipher error, or the mapped card
/// error.
pub fn slogin_store_write_record(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
        frame.save(c, S0, 0x10);
        c.g[V1][0] = 6;
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        c.sw(
            (c.g[V0][0] as u32).wrapping_add(STORE_PENDING_OP),
            c.g[V1][0],
        );
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_RECORD_BUFFER));
        callee::SLOGIN_STORE_GET_BLOCK_HEADER.call(c, 0x003f_bef4)?;
        'done: {
            'started: {
                c.g[S0][0] = c.g[V0][0];
                let no_header = c.g[S0][0] == 0;
                c.g[V0][0] = 4;
                if no_header {
                    break 'started;
                }
                c.cycles += 11;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_RECORD_KIND));
                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(STORE_RECORD_CRC));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_RECORD_SIZE));
                callee::SLOGIN_STORE_SET_RECORD_HEADER.call(c, 0x003f_bf18)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = BLOCK_HEADER_BYTES as u64;
                callee::SLOGIN_STORE_ENCRYPT.call(c, 0x003f_bf24)?;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x10);
                    break 'done;
                }
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_RECORD_SIZE));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_RECORD_BUFFER));
                callee::SLOGIN_STORE_ENCRYPT.call(c, 0x003f_bf3c)?;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x10);
                    break 'done;
                }
                c.cycles += 10;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
                c.g[A1][0] = 2;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_RECORD_SIZE));
                callee::SLOGIN_STORE_CALC_ALLOC_SIZE.call(c, 0x003f_bf54)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[V0][0];
                call_ptr(c, callee::SLOGIN_MC_START_WRITE.addr, 0x003f_bf64)?;
                c.g[A0][0] = c.g[V0][0];
                callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_bf6c)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_ReadRecord(void)` (0x3fbf80): set pending operation 10
/// and start a card read of the record block (header before the record
/// buffer, size +0x84 rounded to format 1). Returns the mapped card error.
pub fn slogin_store_read_record(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
        frame.save(c, S0, 0x10);
        c.g[V1][0] = 0xa;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.sw(
            (c.g[V0][0] as u32).wrapping_add(STORE_PENDING_OP),
            c.g[V1][0],
        );
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_RECORD_BUFFER));
        callee::SLOGIN_STORE_GET_BLOCK_HEADER.call(c, 0x003f_bfa4)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(STORE_RECORD_SIZE));
        callee::SLOGIN_STORE_CALC_ALLOC_SIZE.call(c, 0x003f_bfb8)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, callee::SLOGIN_MC_START_READ.addr, 0x003f_bfc8)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_bfd0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};

    #[test]
    fn slogin_store_map_hash_error_matches() {
        check(
            ee::s15::f_003fb840__generated,
            slogin_store_map_hash_error,
            48,
            &|c, seed| {
                let mut r = Rng(0x3fb8_4000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_b840;
                c.g[A0][0] = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => u32::MAX as u64,
                    _ => r.next_u32() as u64,
                };
            },
        );
    }

    #[test]
    fn slogin_store_get_alignment_matches() {
        check(
            ee::s15::f_003fb858__generated,
            slogin_store_get_alignment,
            60,
            &|c, seed| {
                let mut r = Rng(0x3fb8_5800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_b858;
                c.g[A0][0] = (seed % 7) as u64;
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x104);
                for i in 0..5 {
                    c.bus.write32(ALIGNMENT_DISPATCH + 4 * i, 0x003f_b884);
                }
            },
        );
    }

    #[test]
    fn slogin_store_calc_alloc_size_matches() {
        check(
            ee::s15::f_003fb898__generated,
            slogin_store_calc_alloc_size,
            70,
            &|c, seed| {
                let mut r = Rng(0x3fb8_9800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_b898;
                c.g[A0][0] = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 4,
                    3 => 0xfff,
                    4 => (u32::MAX - 11) as u64,
                    _ => r.next_u32() as u64,
                };
                c.g[A1][0] = (seed % 7) as u64;
                for i in 0..5 {
                    c.bus.write32(ALIGNMENT_DISPATCH + 4 * i, 0x003f_b884);
                }
            },
        );
    }

    #[test]
    fn slogin_store_get_block_header_matches() {
        check(
            ee::s15::f_003fb910__generated,
            slogin_store_get_block_header,
            45,
            &|c, seed| {
                let _r = crate::difftest::fakes::machine(c, seed, 0x3fb9_1000, 0);
                let mode = seed % 3;
                c.pc = 0x003f_b910;
                c.g[A0][0] = sx(DATA + 0x20);
                c.g[A1][0] = if mode == 0 { 0 } else { 1 };
                for i in 0..BLOCK_HEADER_BYTES {
                    let b = (i * 17 + 7) as u8;
                    c.bus.write8(BLOCK_HEADER_MAGIC + i, b);
                    c.bus.write8(
                        DATA + 0x14 + i,
                        if mode == 2 && i == 4 { b ^ 0xff } else { b },
                    );
                }
            },
        );
    }

    #[test]
    fn slogin_store_set_record_header_matches() {
        check(
            ee::s15::f_003fb958__generated,
            slogin_store_set_record_header,
            48,
            &|c, seed| {
                let mut r = Rng(0x3fb9_5800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_b958;
                c.g[A0][0] = sx(DATA + 0x300 + seed % 4);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = sx(DATA + 0x500 + seed % 4);
                c.bus.write32(DATA + 0x500 + seed % 4, r.next_u32());
            },
        );
    }

    #[test]
    fn slogin_store_write_sentinel_matches() {
        check(
            ee::s15::f_003fb9a0__generated,
            slogin_store_write_sentinel,
            48,
            &|c, seed| {
                let mut r = Rng(0x3fb9_a000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_b9a0;
                c.g[A0][0] = sx(DATA + 0x700 + seed % 4);
                for i in 0..12 {
                    c.bus.write8(BLOCK_HEADER_MAGIC + i, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn slogin_store_get_record_header_matches() {
        check(
            ee::s15::f_003fb9d0__generated,
            slogin_store_get_record_header,
            64,
            &|c, seed| {
                let mut r = Rng(0x3fb9_d000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_b9d0;
                c.g[A0][0] = sx(DATA + 0x900 + seed % 4);
                c.g[A1][0] = if seed & 1 == 0 {
                    0
                } else {
                    sx(DATA + 0xa00 + seed % 4)
                };
                c.g[A2][0] = if seed & 2 == 0 {
                    0
                } else {
                    sx(DATA + 0xa10 + seed % 4)
                };
                c.g[A3][0] = if seed & 4 == 0 {
                    0
                } else {
                    sx(DATA + 0xa20 + seed % 4)
                };
                for i in 0..12 {
                    c.bus
                        .write8(DATA + 0x900 + seed % 4 + i, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn slogin_store_find_record_matches() {
        check(
            ee::s15::f_003fba20__generated,
            slogin_store_find_record,
            75,
            &|c, seed| {
                let mut r = Rng(0x3fba_2000 ^ seed as u64);
                scramble(c, &mut r);
                let mode = seed % 5;
                c.pc = 0x003f_ba20;
                c.bus
                    .write32(crate::difftest::GAME_GP + STORE_STATE_GP, DATA + 0xc00);
                c.bus.write32(
                    DATA + 0xc00 + STORE_RECORD_COUNT,
                    if mode == 0 { 0 } else { 3 },
                );
                c.bus.write32(DATA + 0xc00 + STORE_RECORDS, DATA + 0xd00);
                for (i, kind) in [5, 7, 5].iter().enumerate() {
                    c.bus
                        .write32(DATA + 0xd00 + i as u32 * STORE_RECORD_STRIDE, *kind);
                }
                c.g[A0][0] = match mode {
                    3 => 9,
                    4 => 7,
                    _ => 5,
                };
                c.g[A1][0] = if mode == 2 { 1 } else { 0 };
            },
        );
    }

    use crate::difftest::fakes::{self, FAKE, LOG_UNWIND_AT};
    use crate::difftest::{elf_image, fill, GAME_GP};

    const STORE_TEST_STATE: u32 = DATA + 0x2000;
    const STORE_TEST_HASH_IFACE: u32 = DATA + 0x2200;
    const STORE_TEST_CIPHER_IFACE: u32 = DATA + 0x2300;
    const STORE_TEST_MC_MASK: u32 = DATA + 0x2400;
    const STORE_TEST_MC_STATE: u32 = DATA + 0x2500;
    const STORE_TEST_RECORDS: u32 = DATA + 0x3000;
    /// Bit n set: fake interface call n fails (non-zero `v0`), else returns 0.
    const STORE_TEST_FAIL_MASK: u32 = DATA + 0x3f0;
    /// Memory-card globals the real `SLoginMc_Start*` short paths read.
    const STORE_TEST_MC_IFACE_GP: u32 = 0x1aec;
    const STORE_TEST_MC_STATE_GP: u32 = 0x1ae8;

    /// The `miss` hook for the checksum/cipher interface slots: the logging
    /// fake, returning 0 unless the fixture's fail mask names this call.
    fn slogin_store_iface_fake(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let fail = n < 32 && c.bus.read32(STORE_TEST_FAIL_MASK) >> n & 1 != 0;
        fakes::fake_call(c, target);
        if !fail {
            c.g[V0][0] = 0;
        }
    }

    /// Scrambled machine with a random store state at `gp+0x2a30` whose
    /// checksum and cipher interfaces point at fake slots.
    fn slogin_store_io_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = fakes::machine(c, seed, salt, 0);
        c.hooks.miss = slogin_store_iface_fake;
        c.bus.write32(GAME_GP + STORE_STATE_GP, STORE_TEST_STATE);
        fill(c, &mut r, STORE_TEST_STATE, 0x28);
        c.bus
            .write32(STORE_TEST_STATE + STORE_HASH_IFACE, STORE_TEST_HASH_IFACE);
        c.bus.write32(
            STORE_TEST_STATE + STORE_CIPHER_IFACE,
            STORE_TEST_CIPHER_IFACE,
        );
        for (i, slot) in [HASH_BEGIN, HASH_UPDATE, HASH_FINAL]
            .into_iter()
            .enumerate()
        {
            c.bus
                .write32(STORE_TEST_HASH_IFACE + slot, FAKE + 0x10 * i as u32);
        }
        for (i, slot) in [CIPHER_SET_KEY, CIPHER_ENCRYPT, CIPHER_DECRYPT, CIPHER_RESET]
            .into_iter()
            .enumerate()
        {
            c.bus.write32(
                STORE_TEST_CIPHER_IFACE + slot,
                FAKE + 0x40 + 0x10 * i as u32,
            );
        }
        c.bus.write32(STORE_TEST_FAIL_MASK, 0);
        r
    }

    /// Memory-card layer state for the real `SLoginMc_Start*` short paths:
    /// 0 uninitialised (error 2); 1 initialised and busy (error 13);
    /// 2 initialised, idle, device mask empty (open: error 11).
    fn slogin_store_mc_fixture(c: &mut Cpu, mode: u32) {
        c.bus.write32(
            GAME_GP + STORE_TEST_MC_IFACE_GP,
            if mode == 0 { 0 } else { STORE_TEST_MC_MASK },
        );
        c.bus
            .write32(GAME_GP + STORE_TEST_MC_STATE_GP, STORE_TEST_MC_STATE);
        c.bus
            .write16(STORE_TEST_MC_MASK, if mode == 2 { 0 } else { 0xffff });
        c.bus
            .write32(STORE_TEST_MC_STATE, if mode == 1 { 1 } else { 0 });
    }

    /// A directory of `count` entries (kinds 5, 7, 5, 9, 5, 5 repeating) with
    /// random offset, size and CRC words.
    fn slogin_store_set_record_crc_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        count: u32,
        kind: u64,
        occurrence: u64,
    ) {
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP + STORE_STATE_GP, STORE_TEST_STATE);
        c.bus.write32(STORE_TEST_STATE + STORE_RECORD_COUNT, count);
        c.bus
            .write32(STORE_TEST_STATE + STORE_RECORDS, STORE_TEST_RECORDS);
        fill(c, r, STORE_TEST_RECORDS, 4 * 16);
        for i in 0..count {
            c.bus.write32(
                STORE_TEST_RECORDS + i * STORE_RECORD_STRIDE,
                [5, 7, 5, 9, 5, 5][(i % 6) as usize],
            );
        }
        c.bus.write32(DATA + 0x3800, r.next_u32() | 1);
        c.g[A0][0] = kind;
        c.g[A1][0] = occurrence;
        c.g[A2][0] = sx(DATA + 0x3800);
    }

    #[test]
    fn slogin_store_set_record_crc_matches() {
        check(
            ee::s15::f_003fba78__generated,
            slogin_store_set_record_crc,
            96,
            &|c, seed| {
                let mut r = Rng(0x3fba_7800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_ba78;
                // Kind 0x1_0000_0005 never matches (the compare is 64-bit).
                let kind = [5u64, 7, 0x1_0000_0005][(seed / 4 % 3) as usize];
                slogin_store_set_record_crc_fixture(
                    c,
                    &mut r,
                    [0, 1, 3, 6][(seed % 4) as usize],
                    kind,
                    (seed / 12 % 4) as u64,
                );
            },
        );
    }

    #[test]
    fn slogin_store_set_record_crc_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s15::f_003fba78__generated,
            slogin_store_set_record_crc,
            72,
            &|c, seed| {
                let mut r = Rng(0x3fba_78ee ^ seed as u64);
                scramble(c, &mut r);
                let mode = schedule(c, &mut r, seed, 0x003f_ba78);
                slogin_store_set_record_crc_fixture(c, &mut r, 16, 5, (seed / 3 % 4) as u64);
                mode
            },
        );
    }

    #[test]
    fn slogin_store_add_record_matches() {
        check(
            ee::s15::f_003fbad0__generated,
            slogin_store_add_record,
            54,
            &|c, seed| {
                let mut r = Rng(0x3fba_d000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_bad0;
                c.g[GP][0] = GAME_GP as u64;
                let count = [0, 1, 3][(seed % 3) as usize];
                // Full now, full after this entry, or room to spare.
                let capacity = count + [0, 1, 4][(seed / 3 % 3) as usize];
                // 0xffff_fff4 makes size + 12 zero (the `movz` arm).
                let size = [r.next_u32() & 0xfff, 0xffff_fff4, 0][(seed / 9 % 3) as usize];
                c.bus.write32(GAME_GP + STORE_STATE_GP, STORE_TEST_STATE);
                fill(c, &mut r, STORE_TEST_STATE, 0x28);
                c.bus.write32(STORE_TEST_STATE + STORE_RECORD_COUNT, count);
                c.bus
                    .write32(STORE_TEST_STATE + STORE_RECORD_CAPACITY, capacity);
                c.bus
                    .write32(STORE_TEST_STATE + STORE_RECORDS, STORE_TEST_RECORDS);
                fill(c, &mut r, STORE_TEST_RECORDS, 4 * 8);
                c.bus.write32(DATA + 0x3800, r.next_u32());
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = sx(size);
                c.g[A2][0] = sx(DATA + 0x3800);
            },
        );
    }

    #[test]
    fn slogin_store_hash_matches() {
        fakes::covered(
            ee::s15::f_003fbb50__generated,
            slogin_store_hash,
            64,
            &|c, seed| {
                let mut r = slogin_store_io_machine(c, seed, 0x3fbb_5000);
                c.pc = 0x003f_bb50;
                // Begin fails, update fails, final fails, all succeed.
                c.bus
                    .write32(STORE_TEST_FAIL_MASK, [1, 2, 4, 0][(seed % 4) as usize]);
                c.bus
                    .write32(LOG_UNWIND_AT, [0, 1, 2, u32::MAX][(seed / 4 % 4) as usize]);
                c.g[A0][0] = sx(DATA + 0x4000);
                c.g[A1][0] = (r.next_u32() & 0xfff) as u64;
                c.g[A2][0] = sx(DATA + 0x4100);
            },
        );
    }

    #[test]
    fn slogin_store_hash_directory_matches() {
        fakes::covered(
            ee::s15::f_003fbbe0__generated,
            slogin_store_hash_directory,
            72,
            &|c, seed| {
                let mut r = slogin_store_io_machine(c, seed, 0x3fbb_e000);
                c.pc = 0x003f_bbe0;
                // 0x1000_0000 entries is 0 bytes after the shift: the empty arm.
                let count = [0, 3, 0x1000_0000][(seed % 3) as usize];
                c.bus.write32(STORE_TEST_STATE + STORE_RECORD_COUNT, count);
                c.bus
                    .write32(STORE_TEST_STATE + STORE_RECORDS, STORE_TEST_RECORDS);
                c.bus
                    .write32(STORE_TEST_FAIL_MASK, [1, 2, 4, 0][(seed / 3 % 4) as usize]);
                c.bus
                    .write32(LOG_UNWIND_AT, [0, 2, u32::MAX][(seed / 12 % 3) as usize]);
                // Unaligned out pointer or unaligned digest, per seed digit.
                let (out_off, digest_off) = [(0, 1), (3, 0)][(seed / 36 % 2) as usize];
                fill(c, &mut r, DATA + 0x4200, 4);
                fill(c, &mut r, DATA + 0x4400, 4);
                // The object the final step leaves in the caller's out slot.
                c.bus.write32(crate::difftest::SP - 0x30, DATA + 0x4300);
                c.bus.write32(DATA + 0x4304, DATA + 0x4400 + digest_off);
                c.g[A0][0] = sx(DATA + 0x4200 + out_off);
            },
        );
    }

    /// Encrypt/decrypt fixture: no cipher, set-key fails, transform fails,
    /// reset fails, or all succeed; unwinding at fake call 0, 1, 2 or never.
    fn slogin_store_cipher_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = slogin_store_io_machine(c, seed, entry as u64);
        c.pc = entry;
        let arm = seed % 5;
        if arm == 0 {
            c.bus.write32(STORE_TEST_STATE + STORE_CIPHER_IFACE, 0);
        }
        c.bus
            .write32(STORE_TEST_FAIL_MASK, [0, 1, 2, 4, 0][arm as usize]);
        c.bus
            .write32(LOG_UNWIND_AT, [0, 1, 2, u32::MAX][(seed / 5 % 4) as usize]);
        c.g[A0][0] = sx(DATA + 0x4000);
        c.g[A1][0] = (r.next_u32() & 0xfff) as u64;
    }

    #[test]
    fn slogin_store_encrypt_matches() {
        fakes::covered(
            ee::s15::f_003fbc40__generated,
            slogin_store_encrypt,
            60,
            &|c, seed| {
                slogin_store_cipher_fixture(c, seed, 0x003f_bc40);
            },
        );
    }

    #[test]
    fn slogin_store_decrypt_matches() {
        fakes::covered(
            ee::s15::f_003fbcd8__generated,
            slogin_store_decrypt,
            60,
            &|c, seed| {
                slogin_store_cipher_fixture(c, seed, 0x003f_bcd8);
            },
        );
    }

    #[test]
    fn slogin_store_begin_open_matches() {
        check(
            ee::s15::f_003fbd70__generated,
            slogin_store_begin_open,
            40,
            &|c, seed| {
                let mut r = slogin_store_io_machine(c, seed, 0x3fbd_7000);
                c.pc = 0x003f_bd70;
                // StartOpen: uninitialised (2), no name (12), bad device (9),
                // device not in the mask (11), busy (13).
                let mode = seed % 5;
                slogin_store_mc_fixture(
                    c,
                    match mode {
                        0 => 0,
                        3 => 2,
                        _ => 1,
                    },
                );
                c.g[A0][0] = if mode == 1 { 0 } else { sx(DATA + 0x4000) };
                // Device 8 is out of range; -1 is the automatic device.
                c.g[A1][0] = match mode {
                    2 => 8,
                    4 if seed / 5 % 2 == 1 => u64::MAX,
                    _ => 2,
                };
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn slogin_store_seek_to_start_matches() {
        check(
            ee::s15::f_003fbdb8__generated,
            slogin_store_seek_to_start,
            16,
            &|c, seed| {
                let _r = slogin_store_io_machine(c, seed, 0x3fbd_b800);
                c.pc = 0x003f_bdb8;
                slogin_store_mc_fixture(c, seed % 2);
            },
        );
    }

    #[test]
    fn slogin_store_read_dir_header_matches() {
        check(
            ee::s15::f_003fbdf0__generated,
            slogin_store_read_dir_header,
            16,
            &|c, seed| {
                let _r = slogin_store_io_machine(c, seed, 0x3fbd_f000);
                elf_image(c);
                c.pc = 0x003f_bdf0;
                slogin_store_mc_fixture(c, seed % 2);
            },
        );
    }

    #[test]
    fn slogin_store_load_dir_entry_matches() {
        fakes::covered(
            ee::s15::f_003fbe40__generated,
            slogin_store_load_dir_entry,
            72,
            &|c, seed| {
                let mut r = slogin_store_io_machine(c, seed, 0x3fbe_4000);
                c.pc = 0x003f_be40;
                // 0: decrypt fails; 1: directory full (returns 5); 2: entry added
                // after others (seek); 3: first entry (seek), no cipher.
                let arm = seed % 4;
                if arm == 1 || arm == 3 {
                    c.bus.write32(STORE_TEST_STATE + STORE_CIPHER_IFACE, 0);
                }
                let fail = if arm == 0 {
                    [1, 2, 4][(seed / 4 % 3) as usize]
                } else {
                    0
                };
                c.bus.write32(STORE_TEST_FAIL_MASK, fail);
                let (count, capacity) = [(2, 6), (3, 3), (2, 6), (0, 6)][arm as usize];
                c.bus.write32(STORE_TEST_STATE + STORE_RECORD_COUNT, count);
                c.bus
                    .write32(STORE_TEST_STATE + STORE_RECORD_CAPACITY, capacity);
                c.bus
                    .write32(STORE_TEST_STATE + STORE_RECORDS, STORE_TEST_RECORDS);
                c.bus
                    .write32(STORE_TEST_STATE + STORE_DIR_BUFFER, DATA + 0x4000);
                fill(c, &mut r, STORE_TEST_RECORDS, 4 * 8);
                fill(c, &mut r, DATA + 0x4000, 3);
                c.bus.write32(DATA + 0x4004, r.next_u32() & 0xfff);
                slogin_store_mc_fixture(c, seed / 12 % 2);
                c.bus
                    .write32(LOG_UNWIND_AT, [0, 1, u32::MAX][(seed / 24 % 3) as usize]);
            },
        );
    }

    #[test]
    fn slogin_store_write_record_matches() {
        fakes::covered(
            ee::s15::f_003fbed0__generated,
            slogin_store_write_record,
            90,
            &|c, seed| {
                let mut r = slogin_store_io_machine(c, seed, 0x3fbe_d000);
                elf_image(c);
                c.pc = 0x003f_bed0;
                // 0: block header invalid (4); 1: header encrypt fails; 2: payload
                // encrypt fails; 3: written through the cipher; 4: no cipher.
                let arm = seed % 5;
                let step = seed / 5 % 3;
                if arm == 4 {
                    c.bus.write32(STORE_TEST_STATE + STORE_CIPHER_IFACE, 0);
                }
                let fail = match arm {
                    1 => 1 << step,
                    2 => 8 << step,
                    _ => 0,
                };
                c.bus.write32(STORE_TEST_FAIL_MASK, fail);
                let buffer = DATA + 0x5010;
                c.bus
                    .write32(STORE_TEST_STATE + STORE_RECORD_BUFFER, buffer);
                for i in 0..BLOCK_HEADER_BYTES {
                    let b = c.bus.read8(BLOCK_HEADER_MAGIC + i);
                    let corrupt = arm == 0 && i == seed / 5 % BLOCK_HEADER_BYTES;
                    c.bus.write8(
                        buffer - BLOCK_HEADER_BYTES + i,
                        if corrupt { b ^ 0x20 } else { b },
                    );
                }
                fill(c, &mut r, buffer, 8);
                c.bus
                    .write32(STORE_TEST_STATE + STORE_RECORD_SIZE, r.next_u32() & 0xfff);
                slogin_store_mc_fixture(c, seed / 15 % 2);
                c.bus
                    .write32(LOG_UNWIND_AT, [0, 4, u32::MAX][(seed / 30 % 3) as usize]);
            },
        );
    }

    #[test]
    fn slogin_store_read_record_matches() {
        check(
            ee::s15::f_003fbf80__generated,
            slogin_store_read_record,
            32,
            &|c, seed| {
                let mut r = slogin_store_io_machine(c, seed, 0x3fbf_8000);
                elf_image(c);
                c.pc = 0x003f_bf80;
                c.bus
                    .write32(STORE_TEST_STATE + STORE_RECORD_BUFFER, DATA + 0x5010);
                let size = [0, 13, r.next_u32() & 0xfff, 0xffff_fff4][(seed / 2 % 4) as usize];
                c.bus.write32(STORE_TEST_STATE + STORE_RECORD_SIZE, size);
                slogin_store_mc_fixture(c, seed % 2);
            },
        );
    }
}
