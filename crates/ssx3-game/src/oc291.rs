//! Staging wave 211 (oc291): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003e_2768, ea_file_big_find_entry, "EAFile_BigFindEntry"),
];


// ---------------------------------------------------------------------------
// EAFile_BigFindEntry (0x3e2768)
// ---------------------------------------------------------------------------

/// The callees of [`ea_file_big_find_entry`].
mod big_find_entry_callee {
    use super::{ee, Callee};
    pub(super) const GET_FOOTER: Callee = Callee {
        addr: 0x003e_2610,
        f: ee::s14::BigFile_GetFooter,
    };
    pub(super) const GET_HEADER_SIZE: Callee = Callee {
        addr: 0x003e_2580,
        f: ee::s14::BigFile_GetHeaderSize,
    };
    pub(super) const GET_TYPE: Callee = Callee {
        addr: 0x003e_2500,
        f: ee::s14::BigFile_GetType,
    };
    pub(super) const STR_ICMP: Callee = Callee {
        addr: 0x003e_2490,
        f: ee::s14::EAFile_StrICmp,
    };
    /// `strlen` (newlib), called through the function table.
    pub(super) const STRLEN: u32 = 0x0041_6810;
}

/// Frame slots of [`ea_file_big_find_entry`]: the wanted entry index (a2)
/// and the `out_size` pointer (t0).
const BFE_INDEX: u32 = 0x00;
const BFE_OUT_SIZE: u32 = 0x04;
/// Where the index walk spills its constants 2/3/4 (t0..t2) around `strlen`.
const BFE_T0: u32 = 0x10;
const BFE_T1: u32 = 0x20;
const BFE_T2: u32 = 0x30;
/// BIG type 1 (`C0FB` magic): directory starts at +6 with 3-byte fields.
const BFE_TYPE_C0FB: u64 = 1;
/// BIG type 2 (`BIGF`): directory at +0x10, 4-byte fields (the defaults).
const BFE_TYPE_BIGF: u64 = 2;
/// BIG type 3 (`BIG<digit>`): the digit is the offset field's width.
const BFE_TYPE_BIGN: u64 = 3;

/// `char* EAFile_BigFindEntry(const u8* big /*a0*/, const char* name /*a1*/,
/// int index /*a2*/, u32* out_offset /*a3*/, u32* out_size /*t0*/)`
/// (0x3e2768, asm-only `BIG_locateentryz` in the reference): look an entry
/// up in a BIG archive's directory, by name (case-insensitive) or, when
/// `name` is null, by position. Each directory entry is a big-endian
/// offset field, a big-endian size field and a NUL-terminated name; the
/// field widths depend on the archive type (`C0FB` 3/3, `BIGF` 4/4,
/// `BIG<n>` n/min(n,4) with widths over 4 keeping the low 32 bits). The
/// directory runs from the start of entries to the header end minus the
/// optional 8-byte footer. Writes the entry's offset and size through the
/// non-null out pointers (0 when not found) and returns the entry's name
/// pointer, or null.
pub fn ea_file_big_find_entry(c: &mut Cpu) {
    use big_find_entry_callee::*;
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S3, 0xa0);
        frame.save(c, S7, 0x60);
        frame.save(c, FP, 0x50);
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        frame.save(c, S2, 0xb0);
        frame.save(c, S4, 0x90);
        frame.save(c, S5, 0x80);
        frame.save(c, S6, 0x70);
        frame.save_ra(c, 0x40);
        c.g[S3][0] = c.g[A0][0]; // big
        c.g[S7][0] = c.g[A1][0]; // name
        let index = c.g[A2][0];
        let at = frame.at(c, BFE_INDEX);
        c.sw(at, index);
        c.g[FP][0] = c.g[A3][0]; // out_offset
        let out_size = c.g[T0][0];
        let at = frame.at(c, BFE_OUT_SIZE);
        c.sw(at, out_size);
        // Defaults (BIGF): entries at +0x10, 4-byte offset and size.
        c.g[S1][0] = sx(r32(c, S3).wrapping_add(0x10));
        c.g[S2][0] = 4;
        c.g[S5][0] = 4;
        c.g[S6][0] = 0;
        // BigFile_GetFooter(big, 0, 0): the footer's size (8 or 0).
        c.g[A2][0] = 0;
        c.g[A1][0] = 0;
        GET_FOOTER.call(c, 0x003e_27c4)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        GET_HEADER_SIZE.call(c, 0x003e_27d0)?;
        // The directory ends where the footer starts.
        c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
        c.g[A0][0] = c.g[S3][0];
        c.g[S4][0] = sx(r32(c, V0).wrapping_sub(r32(c, S0)));
        GET_TYPE.call(c, 0x003e_27e0)?;
        big_find_entry_field_widths(c);

        c.cycles += 2;
        let in_directory = c.g[S1][0] < c.g[S4][0];
        c.g[V0][0] = in_directory as u64;
        let found = if c.g[S7][0] != 0 {
            big_find_entry_by_name(c, in_directory)?
        } else {
            big_find_entry_by_index(c, in_directory)?
        };
        if !found {
            big_find_entry_not_found(c);
        }

        c.cycles += 12;
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S6, 0x70);
        frame.restore(c, S7, 0x60);
        frame.restore(c, FP, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// [`ea_file_big_find_entry`]: from the archive type (v0), the directory
/// start (s1) and the offset/size field widths (s2/s5). Leaves v0/v1 as the
/// type dispatch does.
fn big_find_entry_field_widths(c: &mut Cpu) {
    let kind = c.g[V0][0];
    c.g[V1][0] = kind;
    c.g[V0][0] = ((kind as i64) < 3) as u64;
    if kind == BFE_TYPE_BIGF {
        return;
    }
    c.cycles += 2;
    let below_3 = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if below_3 {
        c.cycles += 2;
        if kind == BFE_TYPE_C0FB {
            c.g[S1][0] = sx(r32(c, S3).wrapping_add(6));
            c.cycles += 2;
            c.g[S2][0] = 3;
            c.g[S5][0] = 3;
        } else {
            c.cycles += 2;
        }
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = 3;
    c.g[V1][0] = 4;
    if kind == BFE_TYPE_BIGN {
        c.cycles += 6;
        // `BIG<digit>`: offset width = digit, size width = min(digit, 4).
        c.g[V0][0] = c.lb(r32(c, S3).wrapping_add(3));
        c.g[S5][0] = sx(r32(c, V0).wrapping_sub(0x30));
        c.g[S2][0] = c.g[S5][0];
        c.g[V0][0] = ((c.g[S5][0] as i64) < 5) as u64;
        if c.g[V0][0] == 0 {
            c.g[S5][0] = c.g[V1][0];
        }
    }
}

/// [`ea_file_big_find_entry`] by name: compare each entry's name (s3) with
/// `name` (s7), stepping over names with `strlen`. True when found (results
/// stored), false at the directory end.
fn big_find_entry_by_name(c: &mut Cpu, in_directory: bool) -> Result<bool, crate::call::Unwound> {
    use big_find_entry_callee::*;
    c.cycles += 2;
    c.g[S0][0] = sx(r32(c, S1).wrapping_add(r32(c, S2)));
    if !in_directory {
        return Ok(false);
    }
    c.cycles += 1;
    c.g[S6][0] = 1;
    loop {
        c.cycles += 6;
        c.g[A1][0] = c.g[S7][0];
        c.g[S3][0] = sx(r32(c, S0).wrapping_add(r32(c, S5)));
        c.g[A0][0] = c.g[S3][0];
        STR_ICMP.call(c, 0x003e_2858)?;
        if c.g[V0][0] == 0 {
            break;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, STRLEN, 0x003e_2a0c)?;
        // Next entry: past the name's NUL.
        c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
        c.g[S1][0] = sx(r32(c, V0).wrapping_add(1));
        c.g[V0][0] = (c.g[S1][0] < c.g[S4][0]) as u64;
        c.g[S0][0] = sx(r32(c, S1).wrapping_add(r32(c, S2)));
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            return Ok(false);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003e_2848);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = ((c.g[S2][0] as i64) < 5) as u64;
    if c.g[FP][0] != 0 {
        big_find_entry_store_offset(c, true);
    }
    c.cycles += 3;
    let at = sp(c).wrapping_add(BFE_OUT_SIZE);
    c.g[V0][0] = c.lw(at);
    if c.g[V0][0] == 0 {
        c.g[V0][0] = c.g[S3][0];
        return Ok(true);
    }
    big_find_entry_store_size(c, true);
    Ok(true)
}

/// [`ea_file_big_find_entry`] by position: walk the entries counting in s6
/// until it equals the wanted index. True when found (results stored).
fn big_find_entry_by_index(c: &mut Cpu, in_directory: bool) -> Result<bool, crate::call::Unwound> {
    use big_find_entry_callee::*;
    c.cycles += 2;
    if !in_directory {
        return Ok(false);
    }
    c.cycles += 4;
    c.g[S7][0] = 1;
    c.g[T0][0] = 2;
    c.g[T1][0] = 3;
    c.g[T2][0] = 4;
    let frame = Frame { size: 0xe0 };
    loop {
        c.cycles += 6;
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, S2)));
        c.g[S3][0] = sx(r32(c, V0).wrapping_add(r32(c, S5)));
        c.g[S0][0] = c.g[V0][0];
        let at = frame.at(c, BFE_INDEX);
        c.g[V0][0] = c.lw(at);
        if c.g[S6][0] == c.g[V0][0] {
            break;
        }
        frame.save(c, T0, BFE_T0);
        c.cycles += 12;
        c.g[A0][0] = c.g[S3][0];
        frame.save(c, T1, BFE_T1);
        c.g[S6][0] = sx(r32(c, S6).wrapping_add(1));
        frame.save(c, T2, BFE_T2);
        call_ptr(c, STRLEN, 0x003e_2c78)?;
        c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
        frame.restore(c, T0, BFE_T0);
        c.g[S1][0] = sx(r32(c, V0).wrapping_add(1));
        frame.restore(c, T1, BFE_T1);
        c.g[V0][0] = (c.g[S1][0] < c.g[S4][0]) as u64;
        frame.restore(c, T2, BFE_T2);
        if c.g[V0][0] == 0 {
            return Ok(false);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003e_2a40);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = ((c.g[S2][0] as i64) < 5) as u64;
    if c.g[FP][0] != 0 {
        big_find_entry_store_offset(c, false);
    }
    c.cycles += 3;
    let at = frame.at(c, BFE_OUT_SIZE);
    c.g[V1][0] = c.lw(at);
    c.g[V0][0] = c.g[S3][0];
    if c.g[V1][0] == 0 {
        return Ok(true);
    }
    big_find_entry_store_size(c, false);
    Ok(true)
}

/// Not found: zero the non-null out pointers, return null.
fn big_find_entry_not_found(c: &mut Cpu) {
    c.cycles += 2;
    if c.g[FP][0] != 0 {
        c.sw(r32(c, FP), 0);
    }
    c.cycles += 3;
    let at = sp(c).wrapping_add(BFE_OUT_SIZE);
    c.g[V0][0] = c.lw(at);
    if c.g[V0][0] != 0 {
        c.sw(r32(c, V0), 0);
    }
    c.cycles += 1;
    c.g[V0][0] = 0;
}

/// A big-endian field of `width` (1..=4) bytes at `at`, leaving v1/a0/a1
/// as the compiled byte assembly does; other widths read nothing and give 0.
fn big_find_entry_load_be(c: &mut Cpu, at: u32, width: u64) -> u32 {
    match width {
        1 => c.lbu(at) as u32,
        2 => {
            let b0 = c.lbu(at) as u32;
            c.g[V1][0] = c.lbu(at.wrapping_add(1));
            (b0 << 8) | c.g[V1][0] as u32
        }
        3 => {
            let b0 = c.lbu(at) as u32;
            let b1 = c.lbu(at.wrapping_add(1)) as u32;
            c.g[A0][0] = c.lbu(at.wrapping_add(2));
            c.g[V1][0] = sx(b1 << 8);
            (b0 << 16) | (b1 << 8) | c.g[A0][0] as u32
        }
        4 => {
            let b0 = c.lbu(at) as u32;
            let b1 = c.lbu(at.wrapping_add(1)) as u32;
            let b2 = c.lbu(at.wrapping_add(2)) as u32;
            c.g[A1][0] = c.lbu(at.wrapping_add(3));
            c.g[V1][0] = sx(b1 << 16);
            c.g[A0][0] = sx(b2 << 8);
            (b0 << 24) | (b1 << 16) | (b2 << 8) | c.g[A1][0] as u32
        }
        _ => 0,
    }
}

/// The width dispatch ahead of a short field read (1, 2, 3, 4 or none),
/// with its cycles; the by-name copy also sets v1 = 3 past width 1.
fn big_find_entry_short_field(c: &mut Cpu, at: u32, width: u64, by_name: bool) -> u32 {
    c.cycles += 2;
    if width == 1 {
        c.cycles += 2;
        return big_find_entry_load_be(c, at, 1);
    }
    c.cycles += 2;
    if by_name {
        c.g[V1][0] = 3;
    }
    if width == 2 {
        c.cycles += 5;
        return big_find_entry_load_be(c, at, 2);
    }
    c.cycles += 2;
    if width == 3 {
        c.cycles += 8;
        return big_find_entry_load_be(c, at, 3);
    }
    c.cycles += 2;
    if width == 4 {
        c.cycles += 10;
        return big_find_entry_load_be(c, at, 4);
    }
    0
}

/// Store the found entry's offset field (s2 bytes at s1) to `*out_offset`
/// (fp): widths over 4 read the high bytes, then the low 4 bytes, and keep
/// the low 32 bits.
fn big_find_entry_store_offset(c: &mut Cpu, by_name: bool) {
    let field = r32(c, S1);
    let width = c.g[S2][0];
    let value: u64;
    if (width as i64) < 5 {
        c.cycles += 2;
        let v = big_find_entry_short_field(c, field, width, by_name);
        c.cycles += 3;
        value = v as u64;
    } else {
        c.g[V0][0] = sx((width as u32).wrapping_sub(4));
        c.cycles += 2;
        let high_width = c.g[V0][0];
        let high = big_find_entry_short_field(c, field, high_width, false);
        c.cycles += 15;
        let low_at = r32(c, S0).wrapping_sub(4);
        c.g[A1][0] = sx(low_at);
        let b0 = c.lbu(low_at) as u32;
        let b1 = c.lbu(low_at.wrapping_add(1)) as u32;
        let b2 = c.lbu(low_at.wrapping_add(2)) as u32;
        c.g[A2][0] = c.lbu(low_at.wrapping_add(3));
        c.g[V1][0] = sx(b1 << 16);
        c.g[A0][0] = sx(b2 << 8);
        let low = (b0 << 24) | (b1 << 16) | (b2 << 8) | c.g[A2][0] as u32;
        c.g[A3][0] = (high as u64) << 32;
        value = c.g[A3][0] | low as u64;
    }
    c.cycles += 3;
    c.g[V0][0] = sx(value as u32);
    c.sw(r32(c, FP), c.g[V0][0]);
}

/// Store the found entry's size field (s5 bytes at s0) to `*out_size` and
/// return the entry's name (v0 = s3). The by-name and by-index copies of
/// the width dispatch differ only in cycles.
fn big_find_entry_store_size(c: &mut Cpu, by_name: bool) {
    let field = r32(c, S0);
    let width = c.g[S5][0];
    let value = if by_name {
        let cycles = match width {
            1 => 4,
            2 => 8,
            3 => 15,
            4 => 18,
            _ => 11,
        };
        c.cycles += cycles;
        big_find_entry_load_be(c, field, width)
    } else {
        big_find_entry_short_field(c, field, width, false)
    };
    c.cycles += 4;
    let at = sp(c).wrapping_add(BFE_OUT_SIZE);
    c.g[V1][0] = c.lw(at);
    c.sw(r32(c, V1), value as u64);
    c.g[V0][0] = c.g[S3][0];
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A BIG archive at DATA: the header kind (unknown, `C0FB`, `BIGF`,
    /// `BIG0`..`BIG:`), 1-5 entries with random offset/size bytes and
    /// letter names, an optional `L123` footer; the lookup by an existing
    /// name (exact or other case), a missing name, or by index (null name),
    /// with null or real out pointers.
    #[test]
    fn ea_file_big_find_entry_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        check(ee::s14::f_003e2768__generated, ea_file_big_find_entry, 300, &|c, seed| {
            let mut r = Rng(0x0b16_f11e ^ (seed as u64).wrapping_mul(0x9e37_79b9));
            scramble(c, &mut r);
            c.pc = 0x003e_2768;
            // EAFile_StrICmp's ctype table: upper-case letters flagged.
            for b in b'A'..=b'Z' {
                c.bus.write8(0x0049_9c81 + b as u32, 1);
            }
            let big = DATA;
            let kind = seed % 15;
            let (start, off_w, size_w): (u32, i32, i32) = match kind {
                0 => {
                    c.bus.write32(big, 0x7856_3412);
                    (big + 0x10, 4, 4)
                }
                1 => {
                    c.bus.write8(big, 0xc0);
                    c.bus.write8(big + 1, 0xfb);
                    (big + 6, 3, 3)
                }
                2 | 14 => {
                    for (i, b) in b"BIGF".iter().enumerate() {
                        c.bus.write8(big + i as u32, *b);
                    }
                    (big + 0x10, 4, 4)
                }
                _ => {
                    let digit = (kind - 3) as i32;
                    for (i, b) in b"BIG".iter().enumerate() {
                        c.bus.write8(big + i as u32, *b);
                    }
                    c.bus.write8(big + 3, b'0' + digit as u8);
                    (big + 0x10, digit, digit.min(4))
                }
            };
            // The directory.
            let count = 1 + r.next_u32() % 5;
            let mut names: Vec<Vec<u8>> = Vec::new();
            let mut at = start;
            for _ in 0..count {
                for _ in 0..(off_w + size_w) {
                    c.bus.write8(at, r.next_u32() as u8);
                    at += 1;
                }
                let len = 1 + r.next_u32() % 8;
                let mut name = Vec::new();
                for _ in 0..len {
                    let ch = b'a' + (r.next_u32() % 26) as u8;
                    let ch = if r.next_u32() % 2 == 0 { ch.to_ascii_uppercase() } else { ch };
                    name.push(ch);
                    c.bus.write8(at, ch);
                    at += 1;
                }
                c.bus.write8(at, 0);
                at += 1;
                names.push(name);
            }
            if (seed / 15) % 2 == 0 {
                for (i, b) in b"L123".iter().enumerate() {
                    c.bus.write8(at + i as u32, *b);
                }
                c.bus.write32(at + 4, r.next_u32());
                at += 8;
            }
            let header = at - big;
            if kind == 1 {
                c.bus.write8(big + 2, ((header - 4) >> 8) as u8);
                c.bus.write8(big + 3, (header - 4) as u8);
            } else if kind != 0 {
                c.bus.write32(big + 0xc, header.swap_bytes());
            }
            // The lookup.
            let name_at = DATA + 0x800;
            c.g[A0][0] = sx(big);
            c.g[A2][0] = sx(r.next_u32() % (count + 2));
            match (seed / 30) % 4 {
                0 | 1 => {
                    let mut name = names[(r.next_u32() % count) as usize].clone();
                    if (seed / 30) % 4 == 1 {
                        for ch in name.iter_mut() {
                            *ch ^= 0x20;
                        }
                    }
                    for (i, b) in name.iter().enumerate() {
                        c.bus.write8(name_at + i as u32, *b);
                    }
                    c.bus.write8(name_at + name.len() as u32, 0);
                    c.g[A1][0] = sx(name_at);
                }
                2 => {
                    for (i, b) in b"q9\0".iter().enumerate() {
                        c.bus.write8(name_at + i as u32, *b);
                    }
                    c.g[A1][0] = sx(name_at);
                }
                _ => c.g[A1][0] = 0,
            }
            let out_offset = DATA + 0x900;
            let out_size = DATA + 0x904;
            c.bus.write32(out_offset, r.next_u32());
            c.bus.write32(out_size, r.next_u32());
            c.g[A3][0] = if r.next_u32() % 4 == 0 { 0 } else { sx(out_offset) };
            c.g[T0][0] = if r.next_u32() % 4 == 0 { 0 } else { sx(out_size) };
        });
    }
}
