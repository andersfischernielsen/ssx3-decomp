//! `src/bx/bigfile.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
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
    (0x0031_6898, big_file_ctor, "BigFile_ctor"),
    (0x0031_6a00, block_man_ctor, "BlockMan_ctor"),
    (0x0031_6a20, block_man_align_size, "BlockMan_AlignSize"),
    (
        0x003e_2580,
        big_file_get_header_size,
        "BigFile_GetHeaderSize",
    ),
    (0x003e_2610, big_file_get_footer, "BigFile_GetFooter"),
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
    (0x003e_2df0, big_file_get_entry_data, "BigFile_GetEntryData"),
    (
        0x003e_2e20,
        big_file_get_entry_data_2,
        "BigFile_GetEntryData_2",
    ),
    (
        0x003e_2e50,
        big_file_get_entry_data_list,
        "BigFile_GetEntryDataList",
    ),
    (
        0x003e_2ee0,
        big_file_get_entry_count,
        "BigFile_GetEntryCount",
    ),
    (0x003e_2f70, big_file_get_entry_name, "BigFile_GetEntryName"),
    (
        0x003e_2fc8,
        big_file_get_entry_name_2,
        "BigFile_GetEntryName_2",
    ),
];

/// `BigFile *BigFile_ctor(BigFile *this)` (0x316898): mark empty with size -1.
pub fn big_file_ctor(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    Addr(this).off(0x4).sw(c, 0);
    Addr(this).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `BlockMan *BlockMan_ctor(BlockMan *this)` (0x316a00): clear the free lists
/// and set the block size to 16.
pub fn block_man_ctor(c: &mut Cpu) {
    c.cycles += 8;
    let this = abi::arg(c, 0);
    c.g[V0][0] = 0x10;
    Addr(this).sw(c, 0);
    Addr(this).off(0x10).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    Addr(this).off(0x4).sw(c, 0);
    Addr(this).off(0x8).sw(c, 0);
    Addr(this).off(0xc).sw(c, 0);
    abi::ret(c);
}

/// `int BlockMan_AlignSize(BlockMan *this, int size)` (0x316a20): round `size`
/// up to the allocator's block granularity at +0x10.
pub fn block_man_align_size(c: &mut Cpu) {
    c.cycles += 4;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x10).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.cycles += 5;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
    c.div(0, A1, V0);
    c.g[A1][0] = c.lo[0];
    c.mult(0, V0, V0, A1);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// BigFile_GetHeaderSize (0x3e2580)

/// `BigFile_GetType(base)` (0x3e2500).
pub(crate) const OC320_BIG_TYPE: Callee = Callee {
    addr: 0x003e_2500,
    f: ee::s14::BigFile_GetType,
};

/// `int BigFile_GetHeaderSize(BigFile* base)` (0x3e2580): the header size of
/// a BIG file: `0xC0FB` type-1 headers encode it in bytes 2-3 (`+4`); types
/// 2 and 3 ("BIGF"/"BIG0") carry a big-endian size at `+0xc`; anything else
/// is 0.
pub fn big_file_get_header_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        OC320_BIG_TYPE.call(c, 0x003e_2598)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 0x1;
        let not_type1 = c.g[V1][0] != c.g[V0][0];
        c.g[S1][0] = 0;
        if !not_type1 {
            c.cycles += 6;
            c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x2));
            c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x3));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 2;
            let no_header = (c.g[V1][0] as i64) <= 0;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 4i64) as u64;
            if !no_header {
                c.cycles += 2;
                let not_short = c.g[V0][0] == 0;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                if !not_short {
                    c.cycles += 10;
                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
                    c.g[A2][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x3));
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 8);
                    c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                    c.g[S1][0] = c.g[V0][0] | c.g[A2][0];
                }
            }
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BigFile_GetFooter (0x3e2610)

/// `BigFile_GetHeaderSize(base)` (0x3e2580).
pub(crate) const OC320_BIG_HEADER_SIZE: Callee = Callee {
    addr: 0x003e_2580,
    f: ee::s14::BigFile_GetHeaderSize,
};

/// `int BigFile_GetFooter(void* base, int* num, int* tag)` (0x3e2610):
/// recognise the 8-byte footer an EA "big file" build tool appends before
/// the header: a letter and three digits at `header-8`, and the tag bytes at
/// `header-4`. Returns 8 and fills `num`/`tag` (each optional) when the
/// footer is there, 0 otherwise.
pub fn big_file_get_footer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        OC320_BIG_HEADER_SIZE.call(c, 0x003e_2638)?;
        c.g[S3][0] = 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_sub(0x8));
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x41));
        c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
        let lower = c.g[V0][0] != 0;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x8));
        let letter_ok;
        if lower {
            letter_ok = true;
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x61));
            c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
            letter_ok = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S3][0];
        }
        let mut digits_ok = false;
        if letter_ok {
            c.cycles += 5;
            c.g[A1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30));
            c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
            let d1 = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S3][0];
            if d1 {
                c.cycles += 5;
                c.g[V1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
                c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
                let d2 = c.g[V0][0] != 0;
                c.g[V0][0] = c.g[S3][0];
                if d2 {
                    c.cycles += 5;
                    c.g[A2][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x3));
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
                    digits_ok = c.g[V0][0] != 0;
                    c.g[V0][0] = c.g[S3][0];
                }
            }
        }
        if digits_ok {
            c.cycles += 2;
            let no_num = c.g[S1][0] == 0;
            c.g[S3][0] = 0x8;
            if !no_num {
                c.cycles += 16;
                c.g[V0][0] = sx((c.g[A1][0] as u32) << 24);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 24);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
                c.g[A0][0] = 0x64;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
                c.g[A1][0] = 0xa;
                c.mult(0, 2, 2, 4);
                c.mult(0, 3, 3, 5);
                c.g[A0][0] = sx((c.g[A2][0] as u32) << 24);
                c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            }
            c.cycles += 2;
            let no_tag = c.g[S2][0] == 0;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4));
            if !no_tag {
                c.cycles += 11;
                c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_sub(0x4));
                c.g[V1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
                c.g[A2][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x3));
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32) << 8);
                c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                c.sw(c.g[S2][0] as u32, c.g[V0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
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

pub(crate) mod callee {
    pub(crate) use crate::game::lib::eafile::callee::BIG_FILE_FIND_ENTRY;
    pub(crate) use crate::game::lib::eafile::callee::BIG_FILE_GET_ENTRY_DATA;
    pub(crate) use crate::game::lib::eafile::callee::BIG_FILE_GET_ENTRY_OFFSET;
    pub(crate) use crate::game::lib::eafile::callee::BIG_FILE_GET_ENTRY_OFFSET_2;
    pub(crate) use crate::game::lib::eafile::callee::BIG_FILE_GET_FOOTER;
    pub(crate) use crate::game::lib::eafile::callee::BIG_FILE_GET_TYPE;
    pub(crate) use crate::game::lib::eafile::callee::EA_FILE_BIG_FIND_ENTRY;
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

/// `u32 BigFile_GetEntryData(header, name)` (0x3e2df0): entry address
/// through the offset helper, or 0 for a missing name.
pub fn big_file_get_entry_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_2e04);
        c.g[S0][0] = c.g[A0][0];
        callee::BIG_FILE_GET_ENTRY_OFFSET.call(c, 0x003e_2e04)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if c.g[V0][0] == 0 {
            c.g[S0][0] = 0;
        }
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 BigFile_GetEntryData_2(header, name)` (0x3e2e20): same through
/// the second offset helper.
pub fn big_file_get_entry_data_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_2e34);
        c.g[S0][0] = c.g[A0][0];
        callee::BIG_FILE_GET_ENTRY_OFFSET_2.call(c, 0x003e_2e34)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if c.g[V0][0] == 0 {
            c.g[S0][0] = 0;
        }
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BigFile_GetEntryDataList(header, names, out)` (0x3e2e50):
/// comma-split list lookup, one address per name.
pub fn big_file_get_entry_data_list(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_80: u32 = 0x003e_2e80;
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S4][0] = 0x2c;
        frame.save_ra(c, 0);
        'l_3e2e80: loop {
            'b_3e2e98: {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x003e_2e8c);
                c.g[A1][0] = 0x2c;
                let hook = 0x0041_6404;
                c.pc = hook;
                call_ptr(c, hook, 0x003e_2e8c)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] != 0 {
                    c.sb(c.g[S0][0] as u32, 0);
                }
                break 'b_3e2e98;
            }
            'b_3e2ec0: {
                c.cycles += 6;
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x003e_2ea4);
                c.g[A0][0] = c.g[S3][0];
                callee::BIG_FILE_GET_ENTRY_DATA.call(c, 0x003e_2ea4)?;
                c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                let done = c.g[S0][0] == 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                if done {
                    break 'b_3e2ec0;
                }
                c.cycles += 4;
                c.sb(c.g[S0][0] as u32, c.g[S4][0]);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                let more = c.g[S0][0] != 0;
                c.g[S1][0] = c.g[S0][0];
                if more {
                    if event_due(c) {
                        deliver_event_here(c, HEAD_80);
                    }
                    continue 'l_3e2e80;
                }
                break 'b_3e2ec0;
            }
            // Both break arms converge here, inside the loop: the epilogue
            // returns, so the loop only repeats via `continue`.
            c.cycles += 8;
            frame.restore(c, S0, 0x50);
            frame.restore(c, S1, 0x40);
            frame.restore(c, S2, 0x30);
            frame.restore(c, S3, 0x20);
            frame.restore(c, S4, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `u32 BigFile_GetEntryCount(header)` (0x3e2ee0): entry count from the
/// header words, by header type.
pub fn big_file_get_entry_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_2ef8);
        c.g[S0][0] = c.g[A0][0];
        callee::BIG_FILE_GET_TYPE.call(c, 0x003e_2ef8)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 0x1;
        let not_one = c.g[V1][0] != c.g[V0][0];
        c.g[S1][0] = 0;
        'b_3e2f54: {
            'b_3e2f1c: {
                if not_one {
                    break 'b_3e2f1c;
                }
                c.cycles += 5;
                c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x5));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[S1][0] = c.g[V0][0] | c.g[V1][0];
                break 'b_3e2f54;
            }
            // Tail: gen re-tests V1/V0 here (not S1): the t==1 arm above
            // leaves V1 = type and V0 = 1, so the fall-through reuses them.
            c.cycles += 2;
            let small = (c.g[V1][0] as i64) <= 0;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 4) as u64;
            if small {
                break 'b_3e2f54;
            }
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                break 'b_3e2f54;
            }
            c.cycles += 10;
            c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x9));
            c.g[A1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xa));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
            c.g[A2][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xb));
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 8);
            c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
            c.g[S1][0] = c.g[V0][0] | c.g[A2][0];
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `char *BigFile_GetEntryName(header, name, dst)` (0x3e2f70): entry
/// path through the finder, copied to the caller buffer.
pub fn big_file_get_entry_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[T0][0] = 0;
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x003e_2f94);
        c.g[A1][0] = 0;
        callee::BIG_FILE_FIND_ENTRY.call(c, 0x003e_2f94)?;
        let found = c.g[V0][0] != 0;
        c.g[A1][0] = c.g[V0][0];
        'b_3e2fb4: {
            if !found {
                c.cycles += 3;
                c.sb(c.g[S0][0] as u32, 0);
                c.g[V0][0] = 0;
                break 'b_3e2fb4;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_2fb0);
            c.g[A0][0] = c.g[S0][0];
            let hook = 0x0041_66f4;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_2fb0)?;
            c.g[V0][0] = c.g[S0][0];
            break 'b_3e2fb4;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `char *BigFile_GetEntryName_2(header, name, dst)` (0x3e2fc8): same
/// through the EAFile finder.
pub fn big_file_get_entry_name_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[T0][0] = 0;
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x003e_2fec);
        c.g[A1][0] = 0;
        callee::EA_FILE_BIG_FIND_ENTRY.call(c, 0x003e_2fec)?;
        let found = c.g[V0][0] != 0;
        c.g[A1][0] = c.g[V0][0];
        'b_3e3004: {
            if !found {
                c.cycles += 3;
                c.sb(c.g[S0][0] as u32, 0);
                c.g[V0][0] = 0;
                break 'b_3e3004;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_3008);
            c.g[A0][0] = c.g[S0][0];
            let hook = 0x0041_66f4;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_3008)?;
            c.g[V0][0] = c.g[S0][0];
            break 'b_3e3004;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::events::driven;
    use crate::difftest::fakes::machine;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    pub(crate) use crate::game::lib::eafile::tests::put_ascii_2;
    pub(crate) use crate::game::lib::eafile::tests::put_ascii_3;
    pub(crate) use crate::game::lib::eafile::tests::stream_block;
    pub(crate) use crate::game::lib::eafile::tests::zero_block;

    #[test]
    fn big_file_ctor_matches() {
        check(ssx3_ee::s10::f_00316898, big_file_ctor, 100, &|c, seed| {
            let mut r = Rng(0x2011 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }

    #[test]
    fn block_man_ctor_matches() {
        check(ssx3_ee::s10::f_00316a00, block_man_ctor, 100, &|c, seed| {
            let mut r = Rng(0x2012 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x10, r.next_u32());
        });
    }

    #[test]
    fn block_man_align_size_matches() {
        check_lohi(
            ssx3_ee::s10::f_00316a20,
            block_man_align_size,
            300,
            &|c, seed| {
                let mut r = Rng(0x2013 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.bus.write32(
                    DATA + 0x10,
                    match seed % 6 {
                        0 => 0,
                        1 => 8,
                        2 => 16,
                        3 => 0x1000,
                        4 => 0x8000_0000,
                        _ => r.next_u32() & 0xffff,
                    },
                );
            },
        );
    }

    use crate::difftest::RA as SENTINEL;

    // -- 0x003e2580 BigFile_GetHeaderSize ------------------------------------

    /// A "big file" header of every type (`0xC0FB` with bytes 2-3, "BIGF"
    /// and "BIG0" with a big-endian size at +0xc, and an unknown one) plus
    /// the out-of-range type values the type code can return.
    #[test]
    fn big_file_get_header_size_matches() {
        check(
            ee::s14::f_003e2580__generated,
            big_file_get_header_size,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_2580, 0);
                let base = DATA + 0x1000;
                let size = (r.next_u32() & 0xff) | ((r.next_u32() & 0xff) << 8);
                match seed % 5 {
                    0 => {
                        // `0xC0FB` type 1: the size is only bytes 2-3 + 4.
                        c.bus.write8(base, 0xc0);
                        c.bus.write8(base + 1, 0xfb);
                        c.bus.write16(base + 2, size as u16);
                    }
                    1 => {
                        c.bus.write32(base, 0x4249_4746);
                        c.bus.write32(base + 0xc, r.next_u32());
                    }
                    2 => {
                        c.bus.write32(base, 0x4249_4700 | (r.next_u32() & 0xff));
                        c.bus.write32(base + 0xc, r.next_u32());
                    }
                    _ => {
                        c.bus.write32(base, r.next_u32());
                        c.bus.write32(base + 0xc, r.next_u32());
                    }
                }
                c.g[A0][0] = sx(base);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x003e_2580;
            },
        );
    }

    // -- 0x003e2610 BigFile_GetFooter ----------------------------------------

    pub(crate) const OC320_FOOTER_NUM: u32 = DATA + 0x800;

    pub(crate) const OC320_FOOTER_TAG: u32 = DATA + 0x804;

    /// Every header type (the footer sits right before the header), a footer
    /// letter in all three ranges and digits that are valid or not; `num`
    /// and `tag` present or null.
    #[test]
    fn big_file_get_footer_matches() {
        check(
            ee::s14::f_003e2610__generated,
            big_file_get_footer,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_2610, 0);
                let base = DATA + 0x2000;
                let hdr = match seed % 4 {
                    0 => {
                        c.bus.write32(base, r.next_u32() & 0x00ff_ffff);
                        0
                    }
                    1 => {
                        c.bus.write8(base, 0xc0);
                        c.bus.write8(base + 1, 0xfb);
                        c.bus.write16(base + 2, 0x1c);
                        0x20
                    }
                    2 => {
                        c.bus.write32(base, 0x4249_4746);
                        c.bus.write32(base + 0xc, 0x20);
                        0x20
                    }
                    _ => {
                        c.bus.write32(base, 0x4249_4700 | (r.next_u32() & 0xff));
                        c.bus.write32(base + 0xc, 0x20);
                        0x20
                    }
                };
                let q = base + hdr - 8;
                let letter = match seed % 6 {
                    0 => b'Z' - (r.next_u32() % 26) as u8,
                    1 => b'z' - (r.next_u32() % 26) as u8,
                    2 => r.next_u32() as u8,
                    3 => b'0' + (r.next_u32() % 10) as u8,
                    4 => b'A',
                    _ => 0x40,
                };
                c.bus.write8(q, letter);
                for d in 0..3u32 {
                    c.bus.write8(
                        q + 1 + d,
                        if seed % 5 == 0 && d == 2 {
                            b'/' - (r.next_u32() % 10) as u8
                        } else {
                            b'0' + (r.next_u32() % 10) as u8
                        },
                    );
                }
                c.bus.write32(q + 4, r.next_u32());
                c.g[A0][0] = sx(base);
                c.g[A1][0] = if seed % 7 == 0 {
                    0
                } else {
                    sx(OC320_FOOTER_NUM)
                };
                c.g[A2][0] = if seed % 7 == 3 {
                    0
                } else {
                    sx(OC320_FOOTER_TAG)
                };
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x003e_2610;
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
                put_ascii_2(c, "entry", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x400) };
                c.g[A2][0] = (seed / 3 % 7) as u64;
                c.g[A3][0] = sx(DATA + 0x500);
                c.pc = 0x003e_2ce0;
            },
        );
    }

    driven!(gen_big_file_get_entry_count, ee::s14::f_003e2ee0__generated);

    driven!(hand_big_file_get_entry_count, big_file_get_entry_count);

    /// Header magic selects the count word: C0 FB, BIGF, or neither.
    #[test]
    fn big_file_get_entry_count_matches() {
        check(
            gen_big_file_get_entry_count,
            hand_big_file_get_entry_count,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2ee0, 0);
                let block = DATA + 0x700;
                zero_block(c, block, 16);
                match seed % 3 {
                    0 => {
                        c.bus.write8(block, 0xc0);
                        c.bus.write8(block + 1, 0xfb);
                        c.bus.write8(block + 4, (seed / 3 % 5) as u8);
                        c.bus.write8(block + 5, 0x11);
                    }
                    1 => {
                        put_ascii_3(c, "BIGF", block);
                        c.bus.write8(block + 8, (seed / 3 % 7) as u8);
                        c.bus.write8(block + 9, 0x22);
                        c.bus.write8(block + 10, 0x33);
                        c.bus.write8(block + 11, 0x44);
                    }
                    _ => {
                        c.bus.write8(block, (seed / 3 % 251) as u8 + 1);
                    }
                }
                c.g[A0][0] = sx(block);
                c.pc = 0x003e_2ee0;
            },
        );
    }

    driven!(hand_big_file_get_entry_data_2, big_file_get_entry_data_2);

    /// Same via the second offset helper.
    #[test]
    fn big_file_get_entry_data_2_matches() {
        check(
            gen_big_file_get_entry_data_2,
            hand_big_file_get_entry_data_2,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2e20, 0);
                let block = DATA + 0x700 + (seed % 3) * 0x10;
                zero_block(c, block, 16);
                put_ascii_3(c, "entry", DATA + 0x400);
                c.g[A0][0] = sx(block);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x400) };
                c.pc = 0x003e_2e20;
            },
        );
    }

    /// Same via the EAFile entry point.
    #[test]
    fn big_file_get_entry_data_by_index_2_matches() {
        check(
            gen_big_file_get_entry_data_by_index_2,
            hand_big_file_get_entry_data_by_index_2,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2da8, 0);
                put_ascii_2(c, "entry", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x700 + (seed % 3) * 0x10);
                c.g[A1][0] = (seed / 3 % 251) as u64;
                c.g[A2][0] = (seed / 9 % 7) as u64;
                c.g[A3][0] = sx(DATA + 0x500);
                c.pc = 0x003e_2da8;
            },
        );
    }

    /// Indexed lookups add the base to the stored offset.
    #[test]
    fn big_file_get_entry_data_by_index_matches() {
        check(
            gen_big_file_get_entry_data_by_index,
            hand_big_file_get_entry_data_by_index,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2d60, 0);
                put_ascii_2(c, "entry", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x700 + (seed % 3) * 0x10);
                c.g[A1][0] = (seed / 3 % 251) as u64;
                c.g[A2][0] = (seed / 9 % 7) as u64;
                c.g[A3][0] = sx(DATA + 0x500);
                c.pc = 0x003e_2d60;
            },
        );
    }

    /// Comma counts drive 1/multi-iteration walks; no-comma ends at once.
    #[test]
    fn big_file_get_entry_data_list_matches() {
        check(
            gen_big_file_get_entry_data_list,
            hand_big_file_get_entry_data_list,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2e50, 0);
                let block = DATA + 0x700;
                zero_block(c, block, 16);
                let names = ["one", "one,two", "a,b,c", "x"][seed as usize % 4];
                put_ascii_3(c, names, DATA + 0x500);
                zero_block(c, DATA + 0x600, 8);
                c.g[A0][0] = sx(block);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = sx(DATA + 0x600);
                c.pc = 0x003e_2e50;
            },
        );
    }

    driven!(gen_big_file_get_entry_data, ee::s14::f_003e2df0__generated);

    driven!(hand_big_file_get_entry_data, big_file_get_entry_data);

    /// Missing names report zero; present ones report header+offset.
    #[test]
    fn big_file_get_entry_data_matches() {
        check(
            gen_big_file_get_entry_data,
            hand_big_file_get_entry_data,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2df0, 0);
                let block = DATA + 0x700 + (seed % 3) * 0x10;
                zero_block(c, block, 16);
                put_ascii_3(c, "entry", DATA + 0x400);
                c.g[A0][0] = sx(block);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x400) };
                c.pc = 0x003e_2df0;
            },
        );
    }

    driven!(hand_big_file_get_entry_name_2, big_file_get_entry_name_2);

    /// Same through the EAFile finder.
    #[test]
    fn big_file_get_entry_name_2_matches() {
        check(
            gen_big_file_get_entry_name_2,
            hand_big_file_get_entry_name_2,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2fc8, 0);
                let block = DATA + 0x700;
                zero_block(c, block, 16);
                put_ascii_3(c, "entry", DATA + 0x400);
                c.g[A0][0] = sx(block);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x400) };
                c.g[A2][0] = sx(DATA + 0x500);
                c.pc = 0x003e_2fc8;
            },
        );
    }

    driven!(gen_big_file_get_entry_name, ee::s14::f_003e2f70__generated);

    driven!(hand_big_file_get_entry_name, big_file_get_entry_name);

    /// Null lookups clear the dst word; hits copy through strcpy.
    #[test]
    fn big_file_get_entry_name_matches() {
        check(
            gen_big_file_get_entry_name,
            hand_big_file_get_entry_name,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2f70, 0);
                let block = DATA + 0x700;
                zero_block(c, block, 16);
                put_ascii_3(c, "entry", DATA + 0x400);
                c.g[A0][0] = sx(block);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x400) };
                c.g[A2][0] = sx(DATA + 0x500);
                c.pc = 0x003e_2f70;
            },
        );
    }

    /// Same via the EAFile entry point.
    #[test]
    fn big_file_get_entry_offset_2_matches() {
        check(
            gen_big_file_get_entry_offset_2,
            hand_big_file_get_entry_offset_2,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2d30, 0);
                put_ascii_2(c, "entry", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x400) };
                c.pc = 0x003e_2d30;
            },
        );
    }

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
                put_ascii_2(c, "entry", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x400) };
                c.pc = 0x003e_2d00;
            },
        );
    }

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
    driven!(
        gen_big_file_get_entry_data_2,
        ee::s14::f_003e2e20__generated
    );

    driven!(
        gen_big_file_get_entry_data_by_index,
        ee::s14::f_003e2d60__generated
    );

    driven!(
        gen_big_file_get_entry_data_by_index_2,
        ee::s14::f_003e2da8__generated
    );

    driven!(
        gen_big_file_get_entry_data_list,
        ee::s14::f_003e2e50__generated
    );

    driven!(
        gen_big_file_get_entry_name_2,
        ee::s14::f_003e2fc8__generated
    );

    driven!(
        gen_big_file_get_entry_offset,
        ee::s14::f_003e2d00__generated
    );

    driven!(
        gen_big_file_get_entry_offset_2,
        ee::s14::f_003e2d30__generated
    );

    driven!(
        gen_big_file_get_footer_version,
        ee::s14::f_003e2740__generated
    );

    driven!(
        hand_big_file_get_entry_data_by_index,
        big_file_get_entry_data_by_index
    );

    driven!(
        hand_big_file_get_entry_data_by_index_2,
        big_file_get_entry_data_by_index_2
    );

    driven!(
        hand_big_file_get_entry_data_list,
        big_file_get_entry_data_list
    );

    driven!(
        hand_big_file_get_entry_offset_2,
        big_file_get_entry_offset_2
    );

    driven!(
        hand_big_file_get_footer_version,
        big_file_get_footer_version
    );
}
