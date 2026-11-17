//! Staging module oc1025 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1025`.

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
        0x003f_a6c8,
        slogin_mc_get_checksum_iface,
        "SLoginMc_GetChecksumIface",
    ),
    (
        0x003f_a6e8,
        slogin_mc_get_data_offset,
        "SLoginMc_GetDataOffset",
    ),
    (
        0x003f_a708,
        slogin_mc_split_device_index,
        "SLoginMc_SplitDeviceIndex",
    ),
    (
        0x003f_a7a0,
        slogin_mc_build_dir_path,
        "SLoginMc_BuildDirPath",
    ),
    (
        0x003f_a818,
        slogin_mc_build_file_paths,
        "SLoginMc_BuildFilePaths",
    ),
    (0x003f_b278, slogin_mc_is_available, "SLoginMc_IsAvailable"),
    (
        0x003f_b280,
        slogin_mc_calc_save_size_impl,
        "SLoginMc_CalcSaveSizeImpl",
    ),
    (
        0x003f_b2c8,
        slogin_mc_get_checksum_iface_impl,
        "SLoginMc_GetChecksumIfaceImpl",
    ),
    (0x003f_b2f8, slogin_mc_set_busy, "SLoginMc_SetBusy"),
    (
        0x003f_b300,
        slogin_mc_get_data_offset_impl,
        "SLoginMc_GetDataOffsetImpl",
    ),
    (
        0x003f_b308,
        slogin_mc_validate_icon_sys,
        "SLoginMc_ValidateIconSys",
    ),
    (0x003f_b318, crc32_table_entry, "Crc32_TableEntry"),
    (0x003f_b358, crc32_build_table, "Crc32_BuildTable"),
    (0x003f_b3a0, crc32_init, "Crc32_Init"),
    (0x003f_b430, crc32_shutdown, "Crc32_Shutdown"),
    (0x003f_b5e8, crc32_get_interface, "Crc32_GetInterface"),
    (
        0x003f_b800,
        slogin_store_get_cipher_iface,
        "SLoginStore_GetCipherIface",
    ),
    (
        0x003f_b810,
        slogin_store_map_mc_error,
        "SLoginStore_MapMcError",
    ),
    (
        0x003f_b828,
        slogin_store_map_cipher_error,
        "SLoginStore_MapCipherError",
    ),
];

mod callee {
    use super::*;

    pub(crate) const SLOGIN_MC_GET_CHECKSUM_IFACE_IMPL: Callee = Callee {
        addr: 0x003f_b2c8,
        f: ee::s15::SLoginMc_GetChecksumIfaceImpl,
    };
    pub(crate) const SLOGIN_MC_GET_DATA_OFFSET_IMPL: Callee = Callee {
        addr: 0x003f_b300,
        f: ee::s15::SLoginMc_GetDataOffsetImpl,
    };
    pub(crate) const MEMSET: Callee = Callee {
        addr: 0x0041_6210,
        f: ee::s15::memset,
    };
    pub(crate) const MEMCPY: Callee = Callee {
        addr: 0x0041_605c,
        f: ee::s15::memcpy,
    };
    pub(crate) const STRLEN: Callee = Callee {
        addr: 0x0041_6810,
        f: ee::s15::strlen,
    };
    pub(crate) const TIB_EXT_MEM_ALLOC: Callee = Callee {
        addr: 0x0026_6728,
        f: ee::s07::TibExtMemAlloc,
    };
    pub(crate) const TIB_EXT_MEM_FREE: Callee = Callee {
        addr: 0x0026_6760,
        f: ee::s07::TibExtMemFree,
    };

    pub(crate) const CRC32_GET_INTERFACE: Callee = Callee {
        addr: 0x003f_b5e8,
        f: ee::s15::Crc32_GetInterface,
    };
    pub(crate) const CRC32_TABLE_ENTRY: Callee = Callee {
        addr: 0x003f_b318,
        f: ee::s15::Crc32_TableEntry,
    };
    pub(crate) const CRC32_BUILD_TABLE: Callee = Callee {
        addr: 0x003f_b358,
        f: ee::s15::Crc32_BuildTable,
    };
}

/// `const ChecksumIface* SLoginMc_GetChecksumIface(const ChecksumIface** out)`
/// (0x3fa6c8): forward to `SLoginMc_GetChecksumIfaceImpl`.
pub fn slogin_mc_get_checksum_iface(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(
            c,
            callee::SLOGIN_MC_GET_CHECKSUM_IFACE_IMPL.addr,
            0x003f_a6d8,
        )?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginMc_GetDataOffset(void)` (0x3fa6e8): forward to
/// `SLoginMc_GetDataOffsetImpl` (the offset of the login data in the save).
pub fn slogin_mc_get_data_offset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, callee::SLOGIN_MC_GET_DATA_OFFSET_IMPL.addr, 0x003f_a6f8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The device-index jump table (8 words in the game's data).
#[cfg(test)]
pub(crate) const SPLIT_DEVICE_INDEX_TABLE: u32 = 0x0049_73e0;

/// The `port` byte at `out[0]` and `slot` byte at `out[1]`, written in the
/// order the arm writes them (`slot_first`).
fn split_device_index_store(c: &mut Cpu, slot_first: bool, port: u64, slot: u64) {
    let out = c.g[A1][0] as u32;
    if slot_first {
        c.sb(out.wrapping_add(1), slot);
        c.sb(out, port);
    } else {
        c.sb(out, port);
        c.sb(out.wrapping_add(1), slot);
    }
}

/// `void SLoginMc_SplitDeviceIndex(uint device, u8 out[2])` (0x3fa708): split
/// a memory-card device index 0..7 into port (`device / 4`, at `out[0]`) and
/// slot (`device % 4`, at `out[1]`) through the game's 8-entry jump table;
/// larger indices leave `out` alone.
pub fn slogin_mc_split_device_index(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = (c.g[A0][0] < 8) as u64;
    let in_range = c.g[V0][0] != 0;
    c.g[V0][0] = 0x49_0000;
    if !in_range {
        c.cycles += 2;
        ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x73e0));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
    let target = c.g[A0][0] as u32;
    match target {
        // Port 0, slot 0.
        0x003f_a72c => {
            c.cycles += 3;
            split_device_index_store(c, true, 0, 0);
        }
        // Port 0, slots 1..3 (shared tail at 0x3fa73c).
        0x003f_a738 | 0x003f_a748 | 0x003f_a750 => {
            let (cycles, slot) = match target {
                0x003f_a738 => (1, 1),
                0x003f_a748 => (2, 2),
                _ => (2, 3),
            };
            c.cycles += cycles;
            c.g[V0][0] = slot;
            c.cycles += 3;
            split_device_index_store(c, false, 0, slot);
        }
        // Port 1, slot 0.
        0x003f_a758 => {
            c.cycles += 4;
            c.g[V0][0] = 1;
            split_device_index_store(c, true, 1, 0);
        }
        // Port 1, slot 1.
        0x003f_a768 => {
            c.cycles += 4;
            c.g[V0][0] = 1;
            split_device_index_store(c, true, 1, 1);
        }
        // Port 1, slots 2..3 (shared tail at 0x3fa78c).
        0x003f_a778 | 0x003f_a784 => {
            let (cycles, slot) = if target == 0x003f_a778 {
                (3, 2)
            } else {
                (2, 3)
            };
            c.cycles += cycles;
            c.g[V1][0] = 1;
            c.g[V0][0] = slot;
            c.cycles += 2;
            split_device_index_store(c, true, 1, slot);
            c.cycles += 2;
        }
        _ => {
            c.jump_out(target, 0x003f_a708, 0x003f_a79c);
            return;
        }
    }
    ret(c);
}

/// The path lead byte (a global byte at 0x4a4be0).
const BUILD_DIR_PATH_LEAD: u32 = 0x004a_4be0;
/// Bytes each path buffer is cleared to before it is built.
const SLOGIN_MC_PATH_BYTES: u64 = 0x20;

/// `char* SLoginMc_BuildDirPath(char* out, int, const char* name)` (0x3fa7a0):
/// clear 32 bytes of `out`, write the path lead byte (global at 0x4a4be0)
/// followed by `name`, and return the end of the copied name.
pub fn slogin_mc_build_dir_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = SLOGIN_MC_PATH_BYTES;
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
        call_ptr(c, callee::MEMSET.addr, 0x003f_a7cc)?;
        c.g[V1][0] = 0x4a_0000;
        c.g[A0][0] = c.g[S1][0];
        c.g[V0][0] = c.lbu(BUILD_DIR_PATH_LEAD);
        c.sb(c.g[S0][0] as u32, c.g[V0][0]);
        call_ptr(c, callee::STRLEN.addr, 0x003f_a7e0)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, callee::MEMCPY.addr, 0x003f_a7f0)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::STRLEN.addr, 0x003f_a7f8)?;
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The 9-byte file-path prefix (the game's product code) in the game's data.
const BUILD_FILE_PATHS_PREFIX: u32 = 0x0049_7400;
const BUILD_FILE_PATHS_PREFIX_BYTES: u32 = 9;

/// Write the 9-byte file-path prefix at `dst` (8 unaligned bytes, then the
/// 9th byte through `v1`), addressed from the `lui` half kept in `s3`.
fn build_file_paths_prefix(c: &mut Cpu, dst: u32) {
    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(BUILD_FILE_PATHS_PREFIX & 0xffff));
    let src = c.g[A1][0] as u32;
    c.ldl(V0, src.wrapping_add(7));
    c.ldr(V0, src);
    c.g[V1][0] = c.lb(src.wrapping_add(8));
    c.sdl(V0, dst.wrapping_add(7));
    c.sdr(V0, dst);
    c.sb(dst.wrapping_add(8), c.g[V1][0]);
}

/// `int SLoginMc_BuildFilePaths(char* path_a, char* path_b, int,
/// const char* name)` (0x3fa818): clear 32 bytes of each buffer and build
/// the product-code prefix + `name` in both; return `strlen(name)`.
pub fn slogin_mc_build_file_paths(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 56;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = SLOGIN_MC_PATH_BYTES;
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[S3][0] = sx(BUILD_FILE_PATHS_PREFIX & 0xffff_0000);
        call_ptr(c, callee::MEMSET.addr, 0x003f_a84c)?;
        let path_a = c.g[S0][0] as u32;
        build_file_paths_prefix(c, path_a);
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(BUILD_FILE_PATHS_PREFIX_BYTES));
        call_ptr(c, callee::STRLEN.addr, 0x003f_a874)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(BUILD_FILE_PATHS_PREFIX_BYTES));
        call_ptr(c, callee::MEMCPY.addr, 0x003f_a888)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::STRLEN.addr, 0x003f_a890)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = SLOGIN_MC_PATH_BYTES;
        call_ptr(c, callee::MEMSET.addr, 0x003f_a8a0)?;
        let path_b = c.g[S2][0] as u32;
        build_file_paths_prefix(c, path_b);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::STRLEN.addr, 0x003f_a8c4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, callee::MEMCPY.addr, 0x003f_a8d4)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::STRLEN.addr, 0x003f_a8dc)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `bool SLoginMc_IsAvailable(void)` (0x3fb278): the card back end is always
/// available (returns 1).
pub fn slogin_mc_is_available(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 1;
    ret(c);
}

/// Bytes the save reserves on top of the two rounded payloads.
const CALC_SAVE_SIZE_OVERHEAD: u32 = 0x1400;
/// The two payload sizes it rounds (words at +0x3cc and +0x4).
pub(crate) const CALC_SAVE_SIZE_FIRST: u32 = 0x3cc;
pub(crate) const CALC_SAVE_SIZE_SECOND: u32 = 0x4;

/// `void SLoginMc_CalcSaveSizeImpl(const Login* login, uint* bytes,
/// uint* files)` (0x3fb280): round the words at +0x3cc and +0x4 up to whole
/// KiB, add 5 KiB, store that size in `*bytes` and 3 in `*files`.
pub fn slogin_mc_calc_save_size_impl(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(CALC_SAVE_SIZE_FIRST));
    c.g[V0][0] = c.g[V1][0] & 0x3ff;
    let first_whole = c.g[V0][0] == 0;
    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 10);
    if !first_whole {
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    }
    c.cycles += 5;
    c.g[V1][0] = sx((c.g[V0][0] as u32) << 10);
    c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(CALC_SAVE_SIZE_SECOND));
    c.g[V0][0] = c.g[A0][0] & 0x3ff;
    let second_whole = c.g[V0][0] == 0;
    c.g[V0][0] = sx((c.g[A0][0] as u32) >> 10);
    if !second_whole {
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    }
    c.cycles += 7;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 10);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(CALC_SAVE_SIZE_OVERHEAD));
    c.g[V1][0] = 3;
    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
    c.sw(c.g[A2][0] as u32, c.g[V1][0]);
    ret(c);
}

/// `void SLoginMc_GetChecksumIfaceImpl(const ChecksumIface** out)`
/// (0x3fb2c8): store the CRC-32 interface in `*out` when `out` is non-null.
pub fn slogin_mc_get_checksum_iface_impl(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CRC32_GET_INTERFACE.call(c, 0x003f_b2dc)?;
        if c.g[S0][0] != 0 {
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SLoginMc_SetBusy(...)` (0x3fb2f8): does nothing on this back end.
pub fn slogin_mc_set_busy(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_GetDataOffsetImpl(void)` (0x3fb300): the login data starts
/// at offset 0.
pub fn slogin_mc_get_data_offset_impl(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

/// `int SLoginMc_ValidateIconSys(const void* icon_sys)` (0x3fb308): 0 when an
/// icon is given, else error 12.
pub fn slogin_mc_validate_icon_sys(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 12;
    if c.g[A0][0] != 0 {
        c.g[V0][0] = 0;
    }
    ret(c);
}

/// The reflected CRC-32 polynomial (sign-extended as `lui`/`ori` leave it).
const CRC32_POLYNOMIAL: u32 = 0xedb8_8320;

/// `uint Crc32_TableEntry(uint n)` (0x3fb318): the CRC-32 table entry for
/// byte `n`: eight shift-right steps, xoring the reflected polynomial
/// 0xedb88320 whenever the bit shifted out is set.
pub fn crc32_table_entry(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = sx(CRC32_POLYNOMIAL);
    c.g[A1][0] = 8;
    c.g[V0][0] = c.g[A0][0] & 1;
    loop {
        c.cycles += 2;
        let low_clear = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[A0][0] as u32) >> 1);
        if low_clear {
            c.cycles += 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32) >> 1);
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[V0][0] ^ c.g[V1][0];
        }
        c.cycles += 4;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        c.g[A1][0] = c.g[V0][0] & 0xff;
        let more = c.g[A1][0] != 0;
        c.g[V0][0] = c.g[A0][0] & 1;
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003f_b328);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// Entries in the CRC-32 table.
const CRC32_TABLE_ENTRIES: u64 = 0x100;

/// `void Crc32_BuildTable(uint table[256])` (0x3fb358): fill the 256-word
/// CRC-32 lookup table with `Crc32_TableEntry(0..255)`.
pub fn crc32_build_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        loop {
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            callee::CRC32_TABLE_ENTRY.call(c, 0x003f_b378)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            c.g[V0][0] = (c.g[S0][0] < CRC32_TABLE_ENTRIES) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_b370);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The CRC-32 service state: `{ uint* table; int +0x4; int +0x8;
/// uint +0xc; void* heap +0x10 }`.
pub(crate) const CRC32_STATE: u32 = 0x0045_1518;
/// `lui` half of [`CRC32_STATE`] (kept in `s0`) and its low half.
const CRC32_STATE_HI: u32 = CRC32_STATE & 0xffff_0000;
const CRC32_STATE_LO: u32 = CRC32_STATE & 0xffff;
#[cfg(test)]
pub(crate) const CRC32_STATE_TABLE: u32 = 0x0;
const CRC32_STATE_WORD_4: u32 = 0x4;
const CRC32_STATE_WORD_8: u32 = 0x8;
const CRC32_STATE_WORD_C: u32 = 0xc;
const CRC32_STATE_HEAP: u32 = 0x10;
/// Size and alignment of the table allocation.
const CRC32_TABLE_BYTES: u64 = 0x400;
const CRC32_TABLE_ALIGN: u64 = 4;
/// `Crc32_Init` results: already running, out of memory.
const CRC32_ERR_ALREADY_INIT: u64 = 5;
const CRC32_ERR_NO_MEMORY: u64 = 1;
/// `Crc32_Shutdown` result when not running.
const CRC32_ERR_NOT_INIT: u64 = 6;

/// `int Crc32_Init(void* heap)` (0x3fb3a0): allocate the 1 KiB CRC-32 table
/// from `heap` and build it, setting the state words (+0x4 = 4, +0x8 = 0,
/// +0xc = 0xffffffff, +0x10 = heap). Returns 0, 5 if already initialised, or
/// 1 if the allocation fails.
pub fn crc32_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = sx(CRC32_STATE_HI);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CRC32_STATE_LO));
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(CRC32_STATE_LO));
        let running = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if running {
            c.cycles += 2;
            c.g[V0][0] = CRC32_ERR_ALREADY_INIT;
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = CRC32_TABLE_BYTES;
            c.g[A2][0] = CRC32_TABLE_ALIGN;
            call_ptr(c, callee::TIB_EXT_MEM_ALLOC.addr, 0x003f_b3e0)?;
            let failed = c.g[V0][0] == 0;
            c.sw((c.g[S0][0] as u32).wrapping_add(CRC32_STATE_LO), c.g[V0][0]);
            if failed {
                c.cycles += 1;
                c.g[V0][0] = CRC32_ERR_NO_MEMORY;
            } else {
                c.cycles += 11;
                c.g[A0][0] = c.g[V0][0];
                c.sw(
                    (c.g[S2][0] as u32).wrapping_add(CRC32_STATE_HEAP),
                    c.g[S1][0],
                );
                callee::CRC32_BUILD_TABLE.call(c, 0x003f_b3f4)?;
                c.g[V0][0] = u64::MAX;
                c.g[V1][0] = 4;
                c.sw(
                    (c.g[S2][0] as u32).wrapping_add(CRC32_STATE_WORD_4),
                    c.g[V1][0],
                );
                c.sw(
                    (c.g[S2][0] as u32).wrapping_add(CRC32_STATE_WORD_C),
                    c.g[V0][0],
                );
                c.g[V0][0] = 0;
                c.sw((c.g[S2][0] as u32).wrapping_add(CRC32_STATE_WORD_8), 0);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int Crc32_Shutdown(void)` (0x3fb430): free the CRC-32 table back to the
/// heap it came from and clear the table pointer; 6 if not initialised.
pub fn crc32_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = sx(CRC32_STATE_HI);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CRC32_STATE_LO));
        let running = c.g[A1][0] != 0;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(CRC32_STATE_LO));
        if running {
            c.cycles += 7;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(CRC32_STATE_HEAP));
            c.g[A2][0] = CRC32_TABLE_BYTES;
            c.g[A3][0] = CRC32_TABLE_ALIGN;
            call_ptr(c, callee::TIB_EXT_MEM_FREE.addr, 0x003f_b45c)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(CRC32_STATE_LO), 0);
            c.g[V0][0] = 0;
        } else {
            c.cycles += 1;
            c.g[V0][0] = CRC32_ERR_NOT_INIT;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The CRC-32 checksum interface (a table of method pointers in the game's
/// data: Init, Shutdown, ...).
const CRC32_INTERFACE: u32 = 0x0049_7508;

/// `const ChecksumIface* Crc32_GetInterface(void)` (0x3fb5e8): the CRC-32
/// checksum interface at 0x497508.
pub fn crc32_get_interface(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x49_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(CRC32_INTERFACE & 0xffff));
    ret(c);
}

/// The login store's cipher interface (method pointers in the game's data).
const SLOGIN_STORE_CIPHER_INTERFACE: u32 = 0x0049_7528;

/// `const CipherIface* SLoginStore_GetCipherIface(void)` (0x3fb800): the
/// cipher interface at 0x497528.
pub fn slogin_store_get_cipher_iface(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x49_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(SLOGIN_STORE_CIPHER_INTERFACE & 0xffff));
    ret(c);
}

/// Shared shape of the store's error mappers: 0 stays 0, anything else
/// moves up by `base`.
fn slogin_store_map_error(c: &mut Cpu, base: u32) {
    c.cycles += 2;
    let error = c.g[A0][0];
    c.g[V0][0] = 0;
    if error != 0 {
        c.cycles += 1;
        c.g[V0][0] = sx((error as u32).wrapping_add(base));
    }
    c.cycles += 2;
    ret(c);
}

/// Store error range base of memory-card errors.
const SLOGIN_STORE_MC_ERROR_BASE: u32 = 100;
/// Store error range base of cipher errors.
const SLOGIN_STORE_CIPHER_ERROR_BASE: u32 = 200;

/// `int SLoginStore_MapMcError(int mc_error)` (0x3fb810): translate a
/// memory-card error into the login store's error range (`+ 100`, 0 stays 0).
pub fn slogin_store_map_mc_error(c: &mut Cpu) {
    slogin_store_map_error(c, SLOGIN_STORE_MC_ERROR_BASE);
}

/// `int SLoginStore_MapCipherError(int cipher_error)` (0x3fb828): translate
/// a cipher error into the login store's error range (`+ 200`, 0 stays 0).
pub fn slogin_store_map_cipher_error(c: &mut Cpu) {
    slogin_store_map_error(c, SLOGIN_STORE_CIPHER_ERROR_BASE);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, elf_image, scramble, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Scrambled registers and memory around the stack, entry at `entry`.
    fn slogin_mc_leaf_machine(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut r = Rng(entry as u64 ^ (seed as u64) << 24);
        scramble(c, &mut r);
        c.pc = entry;
        r
    }

    #[test]
    fn slogin_mc_get_checksum_iface_matches() {
        check(
            ee::s14::f_003fa6c8__generated,
            slogin_mc_get_checksum_iface,
            32,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_a6c8);
                c.g[A0][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x10) };
                c.bus.write32(DATA + 0x10, 0x5a5a_5a5a);
            },
        );
    }

    #[test]
    fn slogin_mc_get_data_offset_matches() {
        check(
            ee::s14::f_003fa6e8__generated,
            slogin_mc_get_data_offset,
            16,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_a6e8);
            },
        );
    }

    #[test]
    fn slogin_mc_split_device_index_matches() {
        check(
            ee::s14::f_003fa708__generated,
            slogin_mc_split_device_index,
            80,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_a708);
                elf_image(c);
                // The game's table must hold the eight arms this port knows.
                assert_eq!(c.bus.read32(SPLIT_DEVICE_INDEX_TABLE), 0x003f_a72c);
                let out = DATA + 0x200 + seed / 10 % 2;
                c.g[A0][0] = match seed % 10 {
                    d @ 0..=7 => d as u64,
                    8 => 8 + (seed / 20 % 4) as u64,
                    _ => 1u64 << 32 | (seed / 20 % 8) as u64,
                };
                c.g[A1][0] = sx(out);
                c.bus.write32(DATA + 0x200, 0xcccc_cccc);
            },
        );
    }

    /// A NUL-terminated name of `len` letters at `at`, with bytes after it.
    fn slogin_mc_path_name(c: &mut Cpu, at: u32, len: u32, salt: u32) {
        for i in 0..len {
            c.bus.write8(at + i, b'A' + ((i + salt) % 26) as u8);
        }
        c.bus.write8(at + len, 0);
        c.bus.write32(at + len + 1, 0x7777_7777);
    }

    #[test]
    fn slogin_mc_build_dir_path_matches() {
        check(
            ee::s14::f_003fa7a0__generated,
            slogin_mc_build_dir_path,
            48,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_a7a0);
                let out = DATA + 0x400 + seed / 8 % 4;
                for i in 0..0x30 {
                    c.bus.write8(DATA + 0x400 + i, 0xa5);
                }
                c.g[A0][0] = sx(out);
                c.g[A2][0] = sx(DATA + 0x500);
                slogin_mc_path_name(c, DATA + 0x500, seed % 8 * 3, seed);
                // Lead byte with and without its top bit (`lbu`).
                c.bus.write8(
                    BUILD_DIR_PATH_LEAD,
                    if seed / 32 % 2 == 0 { b'*' } else { 0xb0 },
                );
            },
        );
    }

    #[test]
    fn slogin_mc_build_file_paths_matches() {
        check(
            ee::s14::f_003fa818__generated,
            slogin_mc_build_file_paths,
            64,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_a818);
                elf_image(c);
                for i in 0..0x30 {
                    c.bus.write8(DATA + 0x600 + i, 0xa5);
                    c.bus.write8(DATA + 0x700 + i, 0x5a);
                }
                c.g[A0][0] = sx(DATA + 0x600 + seed / 8 % 8);
                c.g[A1][0] = sx(DATA + 0x700 + seed / 2 % 4);
                c.g[A3][0] = sx(DATA + 0x800);
                slogin_mc_path_name(c, DATA + 0x800, seed % 8 * 3, seed);
                // The 9th prefix byte with its top bit set (`lb` into v1).
                if seed / 32 % 2 == 1 {
                    c.bus.write8(BUILD_FILE_PATHS_PREFIX + 8, 0xb8);
                }
            },
        );
    }

    #[test]
    fn slogin_mc_is_available_matches() {
        check(
            ee::s15::f_003fb278__generated,
            slogin_mc_is_available,
            8,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_b278);
            },
        );
    }

    #[test]
    fn slogin_mc_calc_save_size_impl_matches() {
        check(
            ee::s15::f_003fb280__generated,
            slogin_mc_calc_save_size_impl,
            48,
            &|c, seed| {
                let mut r = slogin_mc_leaf_machine(c, seed, 0x003f_b280);
                let login = DATA + 0x100;
                c.g[A0][0] = sx(login);
                c.g[A1][0] = sx(DATA + 0x900);
                c.g[A2][0] = sx(DATA + 0x904);
                c.bus.write32(DATA + 0x900, 0x1111_1111);
                c.bus.write32(DATA + 0x904, 0x2222_2222);
                let kib = |r: &mut Rng, partial: bool| {
                    let whole = r.next_u32() & 0x7fff_fc00;
                    if partial {
                        whole | 1 + seed / 4 % 0x3ff
                    } else {
                        whole
                    }
                };
                let first = kib(&mut r, seed % 2 == 1);
                let second = kib(&mut r, seed / 2 % 2 == 1);
                c.bus.write32(
                    login + CALC_SAVE_SIZE_FIRST,
                    if seed % 12 == 0 { 0 } else { first },
                );
                c.bus.write32(login + CALC_SAVE_SIZE_SECOND, second);
            },
        );
    }

    #[test]
    fn slogin_mc_get_checksum_iface_impl_matches() {
        check(
            ee::s15::f_003fb2c8__generated,
            slogin_mc_get_checksum_iface_impl,
            32,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_b2c8);
                c.g[A0][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x10) };
                c.bus.write32(DATA + 0x10, 0x5a5a_5a5a);
            },
        );
    }

    #[test]
    fn slogin_mc_set_busy_matches() {
        check(
            ee::s15::f_003fb2f8__generated,
            slogin_mc_set_busy,
            8,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_b2f8);
            },
        );
    }

    #[test]
    fn slogin_mc_get_data_offset_impl_matches() {
        check(
            ee::s15::f_003fb300__generated,
            slogin_mc_get_data_offset_impl,
            8,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_b300);
            },
        );
    }

    #[test]
    fn slogin_mc_validate_icon_sys_matches() {
        check(
            ee::s15::f_003fb308__generated,
            slogin_mc_validate_icon_sys,
            32,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_b308);
                c.g[A0][0] = match seed % 4 {
                    0 => 0,
                    1 => sx(DATA),
                    2 => 1u64 << 40, // low word zero: the 64-bit test still sees it
                    _ => 1,
                };
            },
        );
    }

    fn crc32_table_entry_fixture(c: &mut Cpu, seed: u32) {
        let _r = slogin_mc_leaf_machine(c, seed, 0x003f_b318);
        let byte = (seed * 37 + seed / 7) % 256;
        c.g[A0][0] = match seed / 3 % 3 {
            0 => byte as u64,
            1 => 0xffff_ffff_0000_0000 | byte as u64 | 0x8000_0000,
            _ => (seed as u64) << 40 | byte as u64 | (seed as u64 % 16) << 20,
        };
    }

    #[test]
    fn crc32_table_entry_matches() {
        check(
            ee::s15::f_003fb318__generated,
            crc32_table_entry,
            96,
            &|c, seed| {
                crc32_table_entry_fixture(c, seed);
            },
        );
    }

    #[test]
    fn crc32_table_entry_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s15::f_003fb318__generated,
            crc32_table_entry,
            90,
            &|c, seed| {
                crc32_table_entry_fixture(c, seed * 5 + 1);
                let mut r = Rng(0x003f_b318 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_b318)
            },
        );
    }

    fn crc32_build_table_fixture(c: &mut Cpu, seed: u32) {
        let _r = slogin_mc_leaf_machine(c, seed, 0x003f_b358);
        let table = DATA + 0x1000 + 4 * (seed % 4);
        for i in 0..0x110 {
            c.bus.write32(DATA + 0x1000 + 4 * i, 0xc3c3_c3c3);
        }
        c.g[A0][0] = sx(table);
    }

    #[test]
    fn crc32_build_table_matches() {
        check(
            ee::s15::f_003fb358__generated,
            crc32_build_table,
            8,
            &|c, seed| {
                crc32_build_table_fixture(c, seed);
            },
        );
    }

    #[test]
    fn crc32_build_table_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s15::f_003fb358__generated,
            crc32_build_table,
            45,
            &|c, seed| {
                crc32_build_table_fixture(c, seed);
                let mut r = Rng(0x003f_b358 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_b358)
            },
        );
    }

    /// Gate of the table-allocator and table-free fakes; word +4 picks
    /// whether the allocation succeeds.
    const CRC32_HEAP_CTRL: u32 = DATA + 0x6f0;
    const CRC32_HEAP_MAGIC: u32 = 0x4352_4348;
    static CRC32_ALLOC_PREV: AtomicUsize = AtomicUsize::new(0);
    static CRC32_FREE_PREV: AtomicUsize = AtomicUsize::new(0);

    fn crc32_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(CRC32_HEAP_CTRL) != CRC32_HEAP_MAGIC {
            return fakes::fall_back(c, &CRC32_ALLOC_PREV, ee::s07::f_00266728__generated);
        }
        fakes::fake_call(c, callee::TIB_EXT_MEM_ALLOC.addr);
        if c.bus.read32(CRC32_HEAP_CTRL + 4) == 0 {
            c.g[V0][0] = 0;
        }
    }

    fn crc32_fake_free(c: &mut Cpu) {
        if c.bus.read32(CRC32_HEAP_CTRL) != CRC32_HEAP_MAGIC {
            return fakes::fall_back(c, &CRC32_FREE_PREV, ee::s07::f_00266760__generated);
        }
        fakes::fake_call(c, callee::TIB_EXT_MEM_FREE.addr);
    }

    fn crc32_heap_fakes() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266728),
            crc32_fake_alloc,
            &CRC32_ALLOC_PREV,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266760),
            crc32_fake_free,
            &CRC32_FREE_PREV,
        );
    }

    /// The service state words, non-zero, with the table pointer `table`.
    fn crc32_state(c: &mut Cpu, table: u32) {
        for i in 0..6 {
            c.bus.write32(CRC32_STATE + 4 * i, 0x9e00_0000 | i);
        }
        c.bus.write32(CRC32_STATE + CRC32_STATE_TABLE, table);
    }

    #[test]
    fn crc32_init_matches() {
        crc32_heap_fakes();
        fakes::covered(
            ee::s15::f_003fb3a0__generated,
            crc32_init,
            72,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_b3a0, 1);
                c.pc = 0x003f_b3a0;
                let mode = seed % 3; // 0 running, 1 allocation fails, 2 builds
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed / 3 % 4 == 0 { 0 } else { u32::MAX },
                );
                c.bus.write32(CRC32_HEAP_CTRL, CRC32_HEAP_MAGIC);
                c.bus.write32(CRC32_HEAP_CTRL + 4, (mode == 2) as u32);
                c.bus.write32(fakes::LOG_RESULTS, DATA + 0x2000);
                for i in 0..0x104 {
                    c.bus.write32(DATA + 0x2000 + 4 * i, 0xc3c3_c3c3);
                }
                crc32_state(c, if mode == 0 { DATA + 0x2000 } else { 0 });
                c.g[A0][0] = sx(DATA + 0x3000 + seed % 4 * 0x10);
            },
        );
    }

    #[test]
    fn crc32_shutdown_matches() {
        crc32_heap_fakes();
        fakes::covered(
            ee::s15::f_003fb430__generated,
            crc32_shutdown,
            48,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_b430, 1);
                c.pc = 0x003f_b430;
                let running = seed % 2 == 1;
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed / 2 % 3 == 0 { 0 } else { u32::MAX },
                );
                c.bus.write32(CRC32_HEAP_CTRL, CRC32_HEAP_MAGIC);
                crc32_state(c, if running { DATA + 0x2000 } else { 0 });
                c.bus.write32(
                    CRC32_STATE + CRC32_STATE_HEAP,
                    DATA + 0x3000 + seed % 4 * 0x10,
                );
            },
        );
    }

    #[test]
    fn crc32_get_interface_matches() {
        check(
            ee::s15::f_003fb5e8__generated,
            crc32_get_interface,
            8,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_b5e8);
            },
        );
    }

    #[test]
    fn slogin_store_get_cipher_iface_matches() {
        check(
            ee::s15::f_003fb800__generated,
            slogin_store_get_cipher_iface,
            8,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_b800);
            },
        );
    }

    fn slogin_store_map_error_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = slogin_mc_leaf_machine(c, seed, entry);
        c.g[A0][0] = match seed % 5 {
            0 => 0,
            1 => 1,
            2 => u32::MAX as u64,
            3 => 1u64 << 36, // low word zero: the 64-bit test still sees it
            _ => sx(r.next_u32() | 0x8000_0000),
        };
    }

    #[test]
    fn slogin_store_map_mc_error_matches() {
        check(
            ee::s15::f_003fb810__generated,
            slogin_store_map_mc_error,
            40,
            &|c, seed| {
                slogin_store_map_error_fixture(c, seed, 0x003f_b810);
            },
        );
    }

    #[test]
    fn slogin_store_map_cipher_error_matches() {
        check(
            ee::s15::f_003fb828__generated,
            slogin_store_map_cipher_error,
            40,
            &|c, seed| {
                slogin_store_map_error_fixture(c, seed, 0x003f_b828);
            },
        );
    }
}
