//! `src/bx/bigfile.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

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

    use crate::difftest::fakes::machine;

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
}
