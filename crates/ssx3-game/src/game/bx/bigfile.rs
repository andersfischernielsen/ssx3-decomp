//! `src/bx/bigfile.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Target;
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
    (0x0031_68b0, big_file_ctor_2, "BigFile_ctor_2"),
    (0x0031_68e8, big_file_dtor, "BigFile_dtor"),
    (0x0031_6930, big_file_unmount, "BigFile_Unmount"),
    (0x0031_6978, big_file_mount, "BigFile_Mount"),
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
    use super::ee;
    use crate::call::Callee;
    pub(crate) use crate::game::lib::eafile::callee::BIG_FILE_FIND_ENTRY;
    pub(crate) use crate::game::lib::eafile::callee::BIG_FILE_GET_ENTRY_DATA;
    pub(crate) use crate::game::lib::eafile::callee::BIG_FILE_GET_ENTRY_OFFSET;
    pub(crate) use crate::game::lib::eafile::callee::BIG_FILE_GET_ENTRY_OFFSET_2;
    pub(crate) use crate::game::lib::eafile::callee::BIG_FILE_GET_FOOTER;
    pub(crate) use crate::game::lib::eafile::callee::BIG_FILE_GET_TYPE;
    pub(crate) use crate::game::lib::eafile::callee::EA_FILE_BIG_FIND_ENTRY;
    pub(crate) use crate::game::lib::eafile::callee::STRCHR;
    pub(crate) use crate::game::lib::eafile::callee::STRCPY;
    pub(crate) use crate::game::misc::callee::BIG_FILE_MOUNT;
    pub(crate) use crate::game::misc::callee::BIG_FILE_UNMOUNT;
    pub(crate) use crate::game::misc::DPD_DELETE;

    pub(crate) const EA_FILE_MOUNT_BIG: Callee = Callee {
        addr: 0x003d_ed50,
        f: ee::s14::EAFile_MountBig,
    };

    pub(crate) const EA_FILE_UNMOUNT_BIG: Callee = Callee {
        addr: 0x003d_edc0,
        f: ee::s14::EAFile_UnmountBig,
    };

    pub(crate) const EA_FILE_EXISTS: Callee = Callee {
        addr: 0x003d_ee18,
        f: ee::s14::EAFile_Exists,
    };
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

/// `char *BigFile_FindEntry(big, name, index, out_offset, out_size)`
/// (0x3e2ce0): forwards to `EAFile_BigFindEntry`.
pub fn big_file_find_entry(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(callee::EA_FILE_BIG_FIND_ENTRY),
        0x003e_2cf0,
    );
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
                call_ptr(c, callee::STRCHR, 0x003e_2e8c)?;
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
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
            if c.g[V0][0] == 0 {
                break 'b_3e2f54;
            }
            c.cycles += 10;
            c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x8));
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
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `char *BigFile_GetEntryName(header, index, dst)` (0x3e2f70): copies the
/// name of entry `index` (through BigFile_FindEntry) to `dst` and returns
/// `dst`; a missing entry clears `dst[0]` and returns 0.
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
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::STRCPY, 0x003e_2fb0)?;
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

/// `char *BigFile_GetEntryName_2(header, index, dst)` (0x3e2fc8): same
/// through EAFile_BigFindEntry.
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
        'b_3e300c: {
            if !found {
                c.cycles += 3;
                c.sb(c.g[S0][0] as u32, 0);
                c.g[V0][0] = 0;
                break 'b_3e300c;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::STRCPY, 0x003e_3008)?;
            c.g[V0][0] = c.g[S0][0];
            break 'b_3e300c;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BigFile_ctor_2 (0x3168b0)

/// `BigFile *BigFile_ctor_2(BigFile *this)` (0x3168b0): empty file object
/// (`size` -1, data 0) mounted at once.
pub fn big_file_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        c.g[S0][0] = c.g[A0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
        c.g[RA][0] = sx(0x0031_68d0);
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        callee::BIG_FILE_MOUNT.call(c, 0x0031_68d0)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BigFile_dtor (0x3168e8)

/// `void BigFile_dtor(BigFile *this, int free)` (0x3168e8): unmount, and
/// delete the object when the low bit of `free` is set.
pub fn big_file_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x0031_6904);
        c.g[S0][0] = c.g[A1][0];
        callee::BIG_FILE_UNMOUNT.call(c, 0x0031_6904)?;
        c.g[S0][0] &= 0x1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0031_6918);
            c.g[A0][0] = c.g[S1][0];
            callee::DPD_DELETE.call(c, 0x0031_6918)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BigFile_Unmount (0x316930)

/// `int BigFile_Unmount(BigFile *this)` (0x316930): release the mount and
/// clear the data word; nonzero when the unmount released something.
pub fn big_file_unmount(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = 0x1;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.g[RA][0] = sx(0x0031_6958);
            c.g[A1][0] = 0x64;
            callee::EA_FILE_UNMOUNT_BIG.call(c, 0x0031_6958)?;
            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 6;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
        c.g[V0][0] = c.g[V1][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BigFile_Mount (0x316978)

/// `int BigFile_Mount(BigFile *this, path, memclass)` (0x316978): mount the
/// big file at `path`, recording `memclass` while the mount runs; the data
/// word afterwards (1 when the mount landed).
pub fn big_file_mount(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0031_69a8);
        c.g[A1][0] = 0x64;
        callee::EA_FILE_EXISTS.call(c, 0x0031_69a8)?;
        c.g[S3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x278));
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 8;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x278), c.g[S2][0]);
            c.g[A2][0] = 0x64;
            c.g[RA][0] = sx(0x0031_69d0);
            c.g[A3][0] = c.g[S0][0];
            callee::EA_FILE_MOUNT_BIG.call(c, 0x0031_69d0)?;
            let landed = c.g[V0][0] != 0;
            c.g[V0][0] = 0x1;
            if landed {
                c.cycles += 1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            }
            c.cycles += 2;
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x278), c.g[S3][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::check_paths;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::machine;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL;
    pub(crate) use crate::game::lib::eafile::tests::ea_fixed_costs;
    pub(crate) use crate::game::lib::eafile::tests::ea_install_fakes;
    pub(crate) use crate::game::lib::eafile::tests::ea_install_worker_fakes;
    pub(crate) use crate::game::lib::eafile::tests::ea_worker_fixture;
    pub(crate) use crate::game::lib::eafile::tests::put_ascii;
    pub(crate) use crate::game::lib::eafile::tests::req_fixture;
    pub(crate) use crate::game::lib::eafile::tests::stream_block;
    pub(crate) use crate::game::lib::eafile::tests::zero_block;
    pub(crate) use crate::game::lib::eafile::tests::EAF_CTRL;
    pub(crate) use crate::game::lib::eafile::tests::EAF_MAGIC;
    pub(crate) use crate::game::lib::eafile::tests::EAF_REQ;
    pub(crate) use crate::game::lib::eafile::tests::EAF_RET_BIGTYPE;
    pub(crate) use crate::game::lib::eafile::tests::EAF_RET_QUEUE;
    pub(crate) use crate::game::lib::eafile::tests::EAF_RET_RELEASE;
    pub(crate) use crate::game::misc::tests::huffmem_install_fakes;
    pub(crate) use crate::game::misc::tests::HUFFMEM_CTRL;
    pub(crate) use crate::game::misc::tests::HUFFMEM_MAGIC;

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

    /// `EAFile_StrICmp`'s ctype table (upper-case letters flagged).
    const CTYPE: u32 = 0x0049_9c81;

    /// The entry names of the test archive (`put_big`).
    const NAMES: [&str; 4] = ["alpha", "Beta", "gamma", "d"];

    /// Where the lookup tests put the archive and the looked-up name.
    const BIG: u32 = DATA + 0x700;

    const NAME_AT: u32 = DATA + 0x400;

    /// Write `v` big-endian at `at` (any alignment).
    fn put_be(c: &mut Cpu, at: u32, v: u32, width: u32) {
        for i in 0..width {
            c.bus.write8(at + i, (v >> (8 * (width - 1 - i))) as u8);
        }
    }

    /// A BIG archive at `big` holding `NAMES` (entry `i` at offset
    /// `0x100 * (i + 1)`, size `0x10 + i`): kind 0 `BIGF` (4-byte fields,
    /// count at +8, header size at +0xc), kind 1 `C0FB` (3-byte fields, count
    /// at +4, header size - 4 at +2), anything else no archive (zeroed).
    fn put_big(c: &mut Cpu, big: u32, kind: u32) {
        for b in b'A'..=b'Z' {
            c.bus.write8(CTYPE + b as u32, 1);
        }
        zero_block(c, big, 0x40);
        let (start, width) = match kind {
            0 => (big + 0x10, 4),
            1 => (big + 6, 3),
            _ => return,
        };
        let mut at = start;
        for (i, name) in NAMES.iter().enumerate() {
            put_be(c, at, 0x100 * (i as u32 + 1), width);
            put_be(c, at + width, 0x10 + i as u32, width);
            put_ascii(c, at + 2 * width, name);
            at += 2 * width + name.len() as u32 + 1;
        }
        let header = at - big;
        if kind == 0 {
            put_ascii(c, big, "BIGF");
            put_be(c, big + 8, NAMES.len() as u32, 4);
            put_be(c, big + 0xc, header, 4);
        } else {
            c.bus.write8(big, 0xc0);
            c.bus.write8(big + 1, 0xfb);
            put_be(c, big + 2, header - 4, 2);
            put_be(c, big + 4, NAMES.len() as u32, 2);
        }
    }

    /// The name looked up for `digit`: an entry's (exact), another's in
    /// swapped case, a missing one, or null.
    fn lookup_name(c: &mut Cpu, digit: u32) -> u64 {
        let name = match digit % 4 {
            0 => NAMES[2].to_string(),
            1 => NAMES[1].to_ascii_uppercase(),
            2 => "zeta".to_string(),
            _ => return 0,
        };
        put_ascii(c, NAME_AT, &name);
        sx(NAME_AT)
    }

    /// [`check`] that also requires equal scratch registers (`STRICT=args`
    /// alone does not compare them).
    fn check_exact(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        check(generated, hand, seeds, setup);
        for seed in 0..crate::difftest::seed_count(seeds) {
            let r = crate::difftest::compare(generated, hand, &|c| setup(c, seed));
            let soft: Vec<&String> = r
                .soft
                .iter()
                .filter(|m| !m.starts_with("vu0 ctrl 1"))
                .collect();
            assert!(soft.is_empty(), "seed {seed} (soft): {soft:#?}");
        }
    }

    driven!(gen_big_file_find_entry, ee::s14::f_003e2ce0__generated);

    driven!(hand_big_file_find_entry, big_file_find_entry);

    /// Forwards the lookup: a `BIGF`, `C0FB` or no archive; a found, other
    /// case, missing or null name with an index; real out pointers.
    #[test]
    fn big_file_find_entry_matches() {
        check(
            gen_big_file_find_entry,
            hand_big_file_find_entry,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_2ce0, 0);
                put_big(c, BIG, seed % 3);
                c.g[A0][0] = sx(BIG);
                c.g[A1][0] = lookup_name(c, seed / 3);
                c.g[A2][0] = (r.next_u32() % 6) as u64;
                c.g[A3][0] = sx(DATA + 0x500);
                c.g[T0][0] = sx(DATA + 0x504);
                c.pc = 0x003e_2ce0;
            },
        );
    }

    driven!(gen_big_file_get_entry_count, ee::s14::f_003e2ee0__generated);

    driven!(hand_big_file_get_entry_count, big_file_get_entry_count);

    /// The header type selects the count: `C0FB` (bytes 4-5), `BIGF`,
    /// `BIG0` and other `BIG<c>` (bytes 8-11 for types 2-3), unknown
    /// magic (0). Scratch registers are compared (the BIGF arm leaves
    /// `A0 = header + 8`).
    fn big_file_get_entry_count_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3e_2ee0, 0);
        let block = DATA + 0x700;
        zero_block(c, block, 16);
        for i in 4..12u32 {
            c.bus.write8(block + i, r.next_u32() as u8);
        }
        match seed % 7 {
            0 => {
                c.bus.write8(block, 0xc0);
                c.bus.write8(block + 1, 0xfb);
            }
            1 => put_ascii(c, block, "BIGF"),
            2 => put_ascii(c, block, "BIG0"),
            3 => put_ascii(c, block, "BIG4"),
            4 => put_ascii(c, block, "BIG:"),
            5 => c.bus.write8(block, (r.next_u32() % 251) as u8 + 1),
            // A header type of 4 or more, forced through the type fake.
            _ => {
                ea_install_fakes();
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                c.bus.write32(EAF_RET_BIGTYPE, 4 + r.next_u32() % 2);
            }
        }
        c.g[A0][0] = sx(block);
        c.pc = 0x003e_2ee0;
    }

    #[test]
    fn big_file_get_entry_count_matches() {
        check_exact(
            gen_big_file_get_entry_count,
            hand_big_file_get_entry_count,
            48,
            &big_file_get_entry_count_setup,
        );
        crate::difftest::require_paths(
            gen_big_file_get_entry_count,
            48,
            &PATHS_ENTRY_COUNT,
            &big_file_get_entry_count_setup,
        );
    }

    driven!(hand_big_file_get_entry_data_2, big_file_get_entry_data_2);

    /// Same via the second offset helper.
    fn big_file_get_entry_data_2_setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x3e_2e20, 0);
        put_big(c, BIG, seed % 3);
        c.g[A0][0] = sx(BIG);
        c.g[A1][0] = lookup_name(c, seed / 3);
        c.pc = 0x003e_2e20;
    }

    #[test]
    fn big_file_get_entry_data_2_matches() {
        check_paths(
            gen_big_file_get_entry_data_2,
            hand_big_file_get_entry_data_2,
            48,
            &PATHS_ENTRY_DATA_2,
            &big_file_get_entry_data_2_setup,
        );
    }

    /// Indexed lookups (in range or not) in a `BIGF`, `C0FB` or no archive.
    fn by_index_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        let mut r = machine(c, seed, salt, 0);
        put_big(c, BIG, seed % 3);
        c.g[A0][0] = sx(BIG);
        c.g[A1][0] = (seed / 3 % 6) as u64;
        c.g[A2][0] = r.next_u32() as u64;
        c.g[A3][0] = sx(DATA + 0x500);
        c.pc = entry;
    }

    fn big_file_get_entry_data_by_index_2_setup(c: &mut Cpu, seed: u32) {
        by_index_setup(c, seed, 0x3e_2da8, 0x003e_2da8);
    }

    /// Same via the EAFile entry point.
    #[test]
    fn big_file_get_entry_data_by_index_2_matches() {
        check_paths(
            gen_big_file_get_entry_data_by_index_2,
            hand_big_file_get_entry_data_by_index_2,
            36,
            &PATHS_BY_INDEX_2,
            &big_file_get_entry_data_by_index_2_setup,
        );
    }

    fn big_file_get_entry_data_by_index_setup(c: &mut Cpu, seed: u32) {
        by_index_setup(c, seed, 0x3e_2d60, 0x003e_2d60);
    }

    /// Indexed lookups add the base to the stored offset; misses return 0.
    #[test]
    fn big_file_get_entry_data_by_index_matches() {
        check_paths(
            gen_big_file_get_entry_data_by_index,
            hand_big_file_get_entry_data_by_index,
            36,
            &PATHS_BY_INDEX,
            &big_file_get_entry_data_by_index_setup,
        );
    }

    /// Comma lists of one to four names (found or missing) in a `BIGF` or
    /// `C0FB` archive; one address per name.
    fn data_list_fixture(c: &mut Cpu, seed: u32) {
        put_big(c, BIG, seed % 2);
        let names = [
            "alpha",
            "alpha,Beta",
            "gamma,zz,d",
            "x",
            "d,GAMMA,alpha,Beta",
        ][seed as usize / 2 % 5];
        // Poison RAM[0]: the no-comma path must not write (a
        // `==`-for-`!=` slip would `sb(0, 0)` here and be caught).
        c.bus.write8(0, 0xa5);
        put_ascii(c, DATA + 0x500, names);
        zero_block(c, DATA + 0x600, 8);
        c.g[A0][0] = sx(BIG);
        c.g[A1][0] = sx(DATA + 0x500);
        c.g[A2][0] = sx(DATA + 0x600);
        c.pc = 0x003e_2e50;
    }

    #[test]
    fn big_file_get_entry_data_list_matches() {
        check_paths(
            gen_big_file_get_entry_data_list,
            hand_big_file_get_entry_data_list,
            20,
            &PATHS_DATA_LIST,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2e50, 0);
                data_list_fixture(c, seed);
            },
        );
    }

    /// The comma walk with events, delivered without unwinding (schedule
    /// modes off and deliver): an unwind would land first in the strchr or
    /// lookup callees, which cannot resume mid-call in the test machine.
    #[test]
    fn big_file_get_entry_data_list_events() {
        check_events(
            gen_big_file_get_entry_data_list,
            hand_big_file_get_entry_data_list,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_2e50, 0);
                data_list_fixture(c, seed / 2);
                schedule(c, &mut r, seed % 2 * 2, 0x003e_2e50)
            },
        );
    }

    driven!(gen_big_file_get_entry_data, ee::s14::f_003e2df0__generated);

    driven!(hand_big_file_get_entry_data, big_file_get_entry_data);

    /// Found names report header + offset; missing and null ones 0.
    fn big_file_get_entry_data_setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x3e_2df0, 0);
        put_big(c, BIG, seed % 3);
        c.g[A0][0] = sx(BIG);
        c.g[A1][0] = lookup_name(c, seed / 3);
        c.pc = 0x003e_2df0;
    }

    #[test]
    fn big_file_get_entry_data_matches() {
        check_paths(
            gen_big_file_get_entry_data,
            hand_big_file_get_entry_data,
            48,
            &PATHS_ENTRY_DATA,
            &big_file_get_entry_data_setup,
        );
    }

    /// Indexed name lookups: hits copy the name to `dst`, misses clear its
    /// first byte.
    fn entry_name_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        let _r = machine(c, seed, salt, 0);
        put_big(c, BIG, seed % 3);
        for i in 0..4u32 {
            c.bus.write32(DATA + 0x500 + 4 * i, 0xa5a5_a5a5);
        }
        c.g[A0][0] = sx(BIG);
        c.g[A1][0] = (seed / 3 % 6) as u64;
        c.g[A2][0] = sx(DATA + 0x500);
        c.pc = entry;
    }

    driven!(hand_big_file_get_entry_name_2, big_file_get_entry_name_2);

    fn big_file_get_entry_name_2_setup(c: &mut Cpu, seed: u32) {
        entry_name_setup(c, seed, 0x3e_2fc8, 0x003e_2fc8);
    }

    /// Same through the EAFile finder.
    #[test]
    fn big_file_get_entry_name_2_matches() {
        check_paths(
            gen_big_file_get_entry_name_2,
            hand_big_file_get_entry_name_2,
            36,
            &PATHS_ENTRY_NAME_2,
            &big_file_get_entry_name_2_setup,
        );
    }

    driven!(gen_big_file_get_entry_name, ee::s14::f_003e2f70__generated);

    driven!(hand_big_file_get_entry_name, big_file_get_entry_name);

    fn big_file_get_entry_name_setup(c: &mut Cpu, seed: u32) {
        entry_name_setup(c, seed, 0x3e_2f70, 0x003e_2f70);
    }

    #[test]
    fn big_file_get_entry_name_matches() {
        check_paths(
            gen_big_file_get_entry_name,
            hand_big_file_get_entry_name,
            36,
            &PATHS_ENTRY_NAME,
            &big_file_get_entry_name_setup,
        );
    }

    /// Found, other-case, missing and null names; the offset word.
    fn offset_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        let _r = machine(c, seed, salt, 0);
        put_big(c, BIG, seed % 3);
        c.g[A0][0] = sx(BIG);
        c.g[A1][0] = lookup_name(c, seed / 3);
        c.pc = entry;
    }

    fn big_file_get_entry_offset_2_setup(c: &mut Cpu, seed: u32) {
        offset_setup(c, seed, 0x3e_2d30, 0x003e_2d30);
    }

    /// Same via the EAFile entry point.
    #[test]
    fn big_file_get_entry_offset_2_matches() {
        check_paths(
            gen_big_file_get_entry_offset_2,
            hand_big_file_get_entry_offset_2,
            48,
            &PATHS_OFFSET_2,
            &big_file_get_entry_offset_2_setup,
        );
    }

    driven!(hand_big_file_get_entry_offset, big_file_get_entry_offset);

    fn big_file_get_entry_offset_setup(c: &mut Cpu, seed: u32) {
        offset_setup(c, seed, 0x3e_2d00, 0x003e_2d00);
    }

    #[test]
    fn big_file_get_entry_offset_matches() {
        check_paths(
            gen_big_file_get_entry_offset,
            hand_big_file_get_entry_offset,
            48,
            &PATHS_OFFSET,
            &big_file_get_entry_offset_setup,
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

    // Arm totals (cycles of the generated body) each fixture must reach.
    const PATHS_ENTRY_COUNT: [u64; 8] = [33, 53, 58, 46, 23, 25, 29, 30];

    const PATHS_ENTRY_DATA_2: [u64; 6] = [564, 517, 471, 557, 20, 234];

    const PATHS_BY_INDEX_2: [u64; 4] = [264, 217, 483, 227];

    const PATHS_BY_INDEX: [u64; 4] = [271, 224, 490, 234];

    const PATHS_DATA_LIST: [u64; 6] = [502, 455, 1037, 1907, 648, 2349];

    const PATHS_ENTRY_DATA: [u64; 6] = [571, 524, 478, 564, 20, 241];

    const PATHS_ENTRY_NAME_2: [u64; 4] = [313, 246, 484, 228];

    const PATHS_ENTRY_NAME: [u64; 4] = [320, 253, 491, 235];

    const PATHS_OFFSET_2: [u64; 6] = [552, 505, 459, 545, 8, 222];

    const PATHS_OFFSET: [u64; 6] = [559, 512, 466, 552, 8, 229];

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

    driven!(gen_big_file_ctor_2, ee::s10::f_003168b0__generated);

    driven!(hand_big_file_ctor_2, big_file_ctor_2);

    driven!(gen_big_file_dtor, ee::s10::f_003168e8__generated);

    driven!(hand_big_file_dtor, big_file_dtor);

    driven!(gen_big_file_unmount, ee::s10::f_00316930__generated);

    driven!(hand_big_file_unmount, big_file_unmount);

    driven!(gen_big_file_mount, ee::s10::f_00316978__generated);

    driven!(hand_big_file_mount, big_file_mount);

    /// File object and path of the mount tests (nonzero low bytes).
    pub(crate) const BIG_OBJ: u32 = DATA + 0x614;

    pub(crate) const BIG_PATH: u32 = DATA + 0x404;

    /// The EA-backed mount fixture: request table, worker fakes and fixed
    /// fake costs, steered by the status slot (`seed % 2`), the size word
    /// (`seed / 2 % 2`), the queue cell (`seed / 4 % 2`) and the release
    /// cell (`seed / 8 % 2`).
    pub(crate) fn ea_mount_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        ea_install_fakes();
        ea_install_worker_fakes();
        req_fixture(c);
        ea_worker_fixture(c, &[]);
        ea_fixed_costs(c);
        c.g[GP][0] = GAME_GP as u64;
        put_ascii(c, BIG_PATH, "arch.big");
        c.bus
            .write32(DATA + 0x1000, if seed % 2 == 0 { 0x2000 } else { 0 });
        c.bus.write32(DATA + 0x1008, (seed / 2 % 2) as u32);
        c.bus
            .write32(EAF_RET_QUEUE, if seed / 4 % 2 == 0 { 0 } else { EAF_REQ });
        c.bus
            .write32(EAF_RET_RELEASE, if seed / 8 % 2 == 0 { 0 } else { 5 });
        r
    }

    /// Missing file (41), and a mounted file with the data word left 0 or
    /// set to 1 (121, 129); `seed / 16 % 2` picks the prior data word.
    #[test]
    fn big_file_mount_matches() {
        check_paths(
            gen_big_file_mount,
            hand_big_file_mount,
            48,
            &[41, 121, 129],
            &|c, seed| {
                let mut r = ea_mount_setup(c, seed, 0x316978);
                c.bus.write32(BIG_OBJ + 4, seed / 16 % 2);
                c.g[A0][0] = BIG_OBJ as u64;
                c.g[A1][0] = BIG_PATH as u64;
                c.g[A2][0] = (r.next_u32() | 0x8000_0001) as u64;
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0031_6978;
            },
        );
    }

    /// Empty handle (13) and live handles whose release reports nothing or
    /// something (35, 47); `seed / 16 % 3` picks the data word.
    #[test]
    fn big_file_unmount_matches() {
        check_paths(
            gen_big_file_unmount,
            hand_big_file_unmount,
            48,
            &[13, 35, 47],
            &|c, seed| {
                let mut r = ea_mount_setup(c, seed, 0x316930);
                c.bus.write32(BIG_OBJ, r.next_u32());
                c.bus.write32(
                    BIG_OBJ + 4,
                    [0, 0x11, 0x8000_0017][(seed / 16 % 3) as usize],
                );
                c.g[A0][0] = BIG_OBJ as u64;
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0031_6930;
            },
        );
    }

    /// Mount at construction: missing file (54) and the two mounted shapes
    /// (134, 142); `seed / 16 % 2` picks the prior data word.
    #[test]
    fn big_file_ctor_2_matches() {
        check_paths(
            gen_big_file_ctor_2,
            hand_big_file_ctor_2,
            48,
            &[54, 134, 142],
            &|c, seed| {
                let mut r = ea_mount_setup(c, seed, 0x3168b0);
                c.bus.write32(BIG_OBJ, r.next_u32());
                c.bus.write32(BIG_OBJ + 4, seed / 16 % 2);
                c.g[A0][0] = BIG_OBJ as u64;
                c.g[A1][0] = BIG_PATH as u64;
                c.g[A2][0] = (r.next_u32() | 0x8000_0001) as u64;
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0031_68b0;
            },
        );
    }

    /// Unmount (the three unmount shapes, `seed / 16 % 3`) with and without
    /// the delete bit (`seed / 48 % 4` picks `free`, high bits set): 27, 33,
    /// 49 keep the object, 55, 61, 67 delete it.
    #[test]
    fn big_file_dtor_matches() {
        check_paths(
            gen_big_file_dtor,
            hand_big_file_dtor,
            192,
            &[27, 33, 49, 55, 61, 67],
            &|c, seed| {
                let mut r = ea_mount_setup(c, seed, 0x3168e8);
                huffmem_install_fakes();
                ea_fixed_costs(c);
                c.bus.write32(HUFFMEM_CTRL, HUFFMEM_MAGIC);
                c.bus.write32(BIG_OBJ, r.next_u32());
                c.bus.write32(
                    BIG_OBJ + 4,
                    [0, 0x11, 0x8000_0017][(seed / 16 % 3) as usize],
                );
                c.g[A0][0] = BIG_OBJ as u64;
                c.g[A1][0] =
                    [0u64, 1, 0xffff_fffe, 0xffff_ffff_8000_0001][(seed / 48 % 4) as usize];
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0031_68e8;
            },
        );
    }
}
