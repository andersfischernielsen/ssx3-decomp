//! `src/dirtysock/tags.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::festaterewards::r;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::movie::movieplayer::at;
pub(crate) use crate::game::movie::movieplayer::word;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003e_b198, tag_field_copy_tag, "TagFieldCopyTag"),
    (0x003e_b1e0, tag_field_set_raw, "TagFieldSetRaw"),
    (0x003e_b6d8, tag_field_find, "TagFieldFind"),
    (0x003e_b840, tag_field_find_2, "TagFieldFind2"),
    (0x003e_b8b8, tag_field_find_skip_ws, "TagFieldFindSkipWs"),
    (0x003e_b918, tag_field_delete, "TagFieldDelete"),
    (0x003e_b9e8, tag_field_rename, "TagFieldRename"),
    (0x003e_bb60, tag_field_strnzcpy, "TagFieldStrnzcpy"),
    (0x003e_be40, tag_field_set_number, "TagFieldSetNumber"),
    (0x003e_bf28, tag_field_set_flags, "TagFieldSetFlags"),
    (0x003e_bfc8, tag_field_set_address, "TagFieldSetAddress"),
    (0x003e_c0f8, tag_field_set_token, "TagFieldSetToken"),
    (0x003e_c180, tag_field_set_string, "TagFieldSetString"),
    (0x003e_c300, tag_field_set_binary, "TagFieldSetBinary"),
    (0x003e_c3c0, tag_field_set_structure, "TagFieldSetStructure"),
    (0x003e_c6d0, tag_field_set_encoded, "TagFieldSetEncoded"),
    (0x003e_c7e0, tag_field_set_date, "TagFieldSetDate"),
    (
        0x003e_c8b8,
        tag_field_set_date_ymdhms,
        "TagFieldSetDateYMDHMS",
    ),
    (
        0x003e_c9a0,
        tag_field_set_date_packed,
        "TagFieldSetDatePacked",
    ),
    (0x003e_ca88, tag_field_copy_field, "TagFieldCopyField"),
    (0x003e_cb00, tag_field_merge, "TagFieldMerge"),
    (0x003e_cbd8, tag_field_printf, "TagFieldPrintf"),
    (0x003e_cf30, tag_field_get_number, "TagFieldGetNumber"),
    (0x003e_cfb0, tag_field_get_flags, "TagFieldGetFlags"),
    (0x003e_cff8, tag_field_get_address, "TagFieldGetAddress"),
    (0x003e_d070, tag_field_get_token, "TagFieldGetToken"),
    (0x003e_d0f8, tag_field_get_delim, "TagFieldGetDelim"),
    (0x003e_d378, tag_field_get_string, "TagFieldGetString"),
    (0x003e_d578, tag_field_decode_hex, "TagFieldDecodeHex"),
    (0x003e_d678, tag_field_get_binary, "TagFieldGetBinary"),
    (0x003e_d970, tag_field_get_encoded, "TagFieldGetEncoded"),
    (0x003e_db08, tag_field_get_date, "TagFieldGetDate"),
    (
        0x003e_dd70,
        tag_field_get_date_ymdhms,
        "TagFieldGetDateYMDHMS",
    ),
    (
        0x003e_de80,
        tag_field_get_date_packed,
        "TagFieldGetDatePacked",
    ),
];

// ---------------------------------------------------------------------------
// Direct-call targets (the generated body's `c.pc = addr; Name(c)` calls).

pub(crate) mod callee {

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;

    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_COPY_TAG;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_SET_RAW as TAG_FIELD_SET_RAW_2;
}

// ---------------------------------------------------------------------------
// Dirtysock tag fields.

/// `int TagFieldStrnzcpy(char* dst, int len, const char* src)` (0x3ebb60):
/// copy at most `len-1` characters, NUL-terminate, return the number copied.
///
/// The generated body is a `pc`-dispatch loop (it re-enters at the store
/// stage 0x3ebb70 after a thread switch), so this port accepts the same
/// resume addresses.
pub fn tag_field_strnzcpy(c: &mut Cpu) {
    loop {
        match c.pc {
            0x003e_bb60 => {
                c.cycles += 3;
                c.g[A3][0] = c.g[A1][0]; // left = len
                c.g[V0][0] = ((c.g[A1][0] as i64) < 2) as u64;
                c.pc = 0x003e_bb84;
            }
            0x003e_bb6c => {
                c.cycles += 1;
                c.pc = 0x003e_bb70;
            }
            0x003e_bb84 => {
                c.cycles += 2;
                c.pc = if c.g[V0][0] != 0 {
                    0x003e_bb98
                } else {
                    0x003e_bb8c
                };
            }
            0x003e_bb8c => {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lb(c);
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lbu(c);
                if !more {
                    c.pc = 0x003e_bb98;
                    continue;
                }
                if back_edge_unwinds(c, 0x003e_bb70) {
                    return;
                }
                c.pc = 0x003e_bb70;
            }
            0x003e_bb70 => {
                c.cycles += 5;
                Addr(c.g[A0][0] as u32).sb(c, c.g[V1][0]);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < 2) as u64;
                c.pc = 0x003e_bb84;
            }
            0x003e_bb98 => {
                c.cycles += 2;
                if (c.g[A3][0] as i64) > 0 {
                    Addr(c.g[A0][0] as u32).sb(c, 0);
                }
                c.pc = 0x003e_bba0;
            }
            0x003e_bba0 => {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A3][0] as u32));
                crate::abi::ret(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x003e_bb60, 0x003e_bba8);
                return;
            }
        }
    }
}

/// `int TagFieldGetFlags(const char* type, int defval)` (0x3ecfb0): OR together
/// the flag words `D_004965E0[c]` for every character of the tag type; the
/// default is returned for a null type.
pub fn tag_field_get_flags(c: &mut Cpu) {
    loop {
        match c.pc {
            0x003e_cfb0 => {
                c.cycles += 2;
                c.g[V0][0] = c.g[A1][0];
                c.pc = if c.g[A0][0] == 0 {
                    0x003e_cff0
                } else {
                    0x003e_cfb8
                };
            }
            0x003e_cfb8 => {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lb(c);
                c.g[V1][0] = 0x0049_0000;
                c.g[V1][0] = sx(0x0049_0000u32.wrapping_add(0x65e0)); // D_004965E0
                c.g[A1][0] = 0;
                c.pc = 0x003e_cfd8;
            }
            0x003e_cfcc => {
                c.cycles += 1;
                c.pc = 0x003e_cfd0;
            }
            0x003e_cfd0 => {
                c.cycles += 2;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lb(c);
                c.pc = 0x003e_cfd8;
            }
            0x003e_cfd8 => {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[A1][0] |= c.g[V0][0];
                    if back_edge_unwinds(c, 0x003e_cfd0) {
                        return;
                    }
                    c.pc = 0x003e_cfd0;
                } else {
                    c.pc = 0x003e_cfec;
                }
            }
            0x003e_cfec => {
                c.cycles += 1;
                c.g[V0][0] = c.g[A1][0];
                c.pc = 0x003e_cff0;
            }
            0x003e_cff0 => {
                c.cycles += 2;
                crate::abi::ret(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x003e_cfb0, 0x003e_cff8);
                return;
            }
        }
    }
}

/// `lb off(reg)` (zero-extended).
#[inline]
pub(crate) fn lbu_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lbu(c)
}

/// `int TagFieldGetAddress(const char* tag, int defval)` (0x3ecff8): parse a
/// dotted IPv4 text address; digits shift in decimal, a '.' shifts the
/// accumulator left by eight, any other character ends the scan. Returns
/// `defval` for a null tag.
pub fn tag_field_get_address(c: &mut Cpu) {
    loop {
        match c.pc {
            0x003e_cff8 => {
                c.cycles += 4;
                c.g[A2][0] = c.g[A0][0];
                c.g[V0][0] = c.g[A1][0];
                let null = c.g[A2][0] == 0;
                c.g[V1][0] = 0;
                c.pc = if null { 0x003e_d068 } else { 0x003e_d008 };
            }
            0x003e_d008 => {
                c.cycles += 5;
                c.g[A1][0] = 0xffff_ffff_ffff_0000;
                c.g[T0][0] = 0xa;
                c.g[A1][0] = c.g[A1][0] | 0xff00;
                c.g[A3][0] = 0x2e;
                c.pc = 0x003e_d024;
            }
            0x003e_d020 => {
                c.cycles += 1;
                c.g[A2][0] = sx(low(c, A2).wrapping_add(1));
                c.pc = 0x003e_d024;
            }
            0x003e_d024 => {
                c.cycles += 5;
                c.g[A0][0] = lbu_r(c, A2, 0);
                c.g[V0][0] = sx(low(c, A0).wrapping_sub(0x30));
                c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
                let digit = c.g[V0][0] != 0;
                c.g[V0][0] = c.g[V1][0] & 0xff;
                c.pc = if digit { 0x003e_d038 } else { 0x003e_d050 };
            }
            0x003e_d038 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[A0][0] & 0xf;
                c.mult(0, V0, V0, T0); // lo = (accum & 0xff) * 10
                c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                c.g[V0][0] = sx(low(c, V0).wrapping_add(low(c, A0)));
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                if back_edge_unwinds(c, 0x003e_d020) {
                    return;
                }
                c.pc = 0x003e_d020;
            }
            0x003e_d050 => {
                c.cycles += 4;
                c.g[V0][0] = sx(low(c, A0) << 24);
                c.g[V0][0] = sx(((low(c, V0) as i32) >> 24) as u32);
                let dot = c.g[V0][0] != c.g[A3][0];
                c.g[V0][0] = c.g[V1][0];
                c.pc = if dot { 0x003e_d068 } else { 0x003e_d060 };
            }
            0x003e_d060 => {
                c.cycles += 2;
                c.g[V1][0] = sx(low(c, V1) << 8);
                if back_edge_unwinds(c, 0x003e_d020) {
                    return;
                }
                c.pc = 0x003e_d020;
            }
            0x003e_d068 => {
                c.cycles += 2;
                abi::ret(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x003e_cff8, 0x003e_d070);
                return;
            }
        }
    }
}

/// `int TagFieldGetNumber(const char* tag, int defval)` (0x003ecf30): parse a
/// signed decimal number from a tag field.
pub fn tag_field_get_number(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A2][0] = c.g[A0][0];
    let empty = c.g[A2][0] == 0;
    c.g[V0][0] = c.g[A1][0];
    if !empty {
        c.cycles += 4;
        c.g[V1][0] = Addr(at(c, A2, 0)).lb(c);
        c.g[V0][0] = 0x2b;
        let not_plus = c.g[V1][0] != c.g[V0][0];
        c.g[T0][0] = 0x1;
        if not_plus {
            c.cycles += 3;
            c.g[V0][0] = 0x2d;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A1][0] = Addr(at(c, A2, 0)).lbu(c);
            } else {
                c.cycles += 2;
                c.g[A2][0] = sx(word(c, A2).wrapping_add(1));
                c.g[T0][0] = 0xffffffffffffffff;
                c.cycles += 1;
                c.g[A1][0] = Addr(at(c, A2, 0)).lbu(c);
            }
        } else {
            c.cycles += 2;
            c.g[A2][0] = sx(word(c, A2).wrapping_add(1));
            c.cycles += 1;
            c.g[A1][0] = Addr(at(c, A2, 0)).lbu(c);
        }
        c.cycles += 4;
        c.g[V0][0] = sx(word(c, A1).wrapping_sub(0x30));
        c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
        let not_digit = c.g[V0][0] == 0;
        c.g[V1][0] = 0;
        if !not_digit {
            c.cycles += 1;
            c.g[A3][0] = 0xa;
            loop {
                c.cycles += 8;
                c.g[A0][0] = sx(word(c, A1) & 0xf);
                c.mult(0, V1, V1, A3);
                c.g[A2][0] = sx(word(c, A2).wrapping_add(1));
                c.g[A1][0] = Addr(at(c, A2, 0)).lbu(c);
                c.g[V0][0] = sx(word(c, A1).wrapping_sub(0x30));
                c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                let digit = c.g[V0][0] != 0;
                c.g[V1][0] = sx(word(c, V1).wrapping_add(word(c, A0)));
                if !digit {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003e_cf80);
                }
            }
        }
        c.cycles += 1;
        c.mult(0, V0, T0, V1);
    }
    c.cycles += 2;
    let ra = word(c, RA);
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Tag fields.

/// `int TagFieldGetToken(const char* data, int defval)` (0x3ed070): read up to
/// four characters into a FourCC (starting from `0x20202020`), or `defval`
/// when the text is empty or the token overflows; a token ends at a control
/// character or a character >= 127.
pub fn tag_field_get_token(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A2][0] = c.g[A0][0];
    let null = c.g[A2][0] == 0;
    c.g[V0][0] = c.g[A1][0];
    if null {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    let mut data = r32(c, A2);
    c.g[V1][0] = Addr(data).lb(c);
    let ch = c.g[V1][0];
    c.g[T0][0] = ((ch as i64) < 33) as u64;
    let too_small = c.g[T0][0] != 0;
    c.g[A3][0] = Addr(data).lbu(c);
    if too_small {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V1][0] = ((ch as i64) < 127) as u64;
    let too_big = c.g[V1][0] == 0;
    c.g[A0][0] = 0x2020_0000;
    if too_big {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[A0][0] | 0x2020;
    c.cycles += 2;
    c.g[A1][0] = 0x20ff_0000;
    c.cycles += 2;
    c.g[A1][0] = c.g[A1][0] | 0xfffe;
    c.g[V0][0] = sx(r32(c, A3) << 24);
    loop {
        c.cycles += 6;
        c.g[A0][0] = sx(r32(c, A0) << 8);
        c.g[V0][0] = sx(((r32(c, V0) as i32) >> 24) as u32);
        c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
        c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        let stop = c.g[V1][0] != 0;
        c.g[A2][0] = sx(data.wrapping_add(1));
        if stop {
            break;
        }
        c.cycles += 4;
        data = r32(c, A2);
        c.g[V1][0] = Addr(data).lb(c);
        let ch = c.g[V1][0];
        c.g[V0][0] = ((ch as i64) < 33) as u64;
        let too_small = c.g[V0][0] != 0;
        c.g[A3][0] = Addr(data).lbu(c);
        if too_small {
            break;
        }
        c.cycles += 3;
        c.g[V0][0] = ((ch as i64) < 127) as u64;
        let too_big = c.g[V0][0] != 0;
        c.g[V0][0] = sx(r32(c, A3) << 24);
        if too_big {
            if event_due(c) {
                deliver_event_here(c, 0x003e_d0b0);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `void TagFieldSetToken(TagField* self, u32 id, char* str, u32 token)`
/// (0x3ec0f8): copy the current tag into a stack buffer, append the token as
/// up to three characters (big-endian order) and set the field.
pub fn tag_field_set_token(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        open(c, 0x160);
        let sp0 = spv(c);
        c.sq(17, at_2(sp0, 0x140));
        c.sq(18, at_2(sp0, 0x130));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, at_2(sp0, 0x150));
        c.g[S2][0] = c.g[A1][0];
        c.sd(at_2(sp0, 0x120), c.g[RA][0]);
        c.g[S0][0] = c.g[A3][0];
        c.g[A1][0] = c.g[SP][0];
        callee::TAG_FIELD_COPY.call(c, 0x003e_c120)?;
        c.g[V1][0] = c.g[V0][0];
        let token = c.g[S0][0];
        if token != 0 {
            c.cycles += 4;
            c.g[A0][0] = 0x20ff_ffff;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S0][0] as i64)) as u64;
            loop {
                c.cycles += 2;
                let skip = c.g[V0][0] == 0;
                c.g[V0][0] = sx((((c.g[S0][0] as u32) as i32) >> 24) as u32);
                if !skip {
                    c.cycles += 2;
                    c.sb(c.g[V1][0] as u32, c.g[V0][0]);
                    c.g[V1][0] = sx(at_2(c.g[V1][0] as u32, 1));
                }
                c.cycles += 3;
                c.g[S0][0] = sx((c.g[S0][0] as u32) << 8);
                let more = c.g[S0][0] != 0;
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003e_c138);
                }
            }
        }
        c.cycles += 11;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        c.sb(c.g[V1][0] as u32, 0);
        c.g[A2][0] = c.g[SP][0];
        callee::TAG_FIELD_SET_RAW.call(c, 0x003e_c168)?;
        c.lq(16, at_2(sp0, 0x150));
        c.lq(17, at_2(sp0, 0x140));
        c.lq(18, at_2(sp0, 0x130));
        c.g[RA][0] = c.ld(at_2(sp0, 0x120));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_2(sp0, 0x160));
        c.pc = ra;
        Ok(())
    })
}

/// `void TagFieldSetFlags(Tag tag, int flags, void* format, int value)`
/// (0x3ebf28): copy `tag` to a stack buffer, append the value's base-32 digits
/// from the table at `0x4965c0` and set the flags field with the result.
pub fn tag_field_set_flags(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x160);
        frame.save(c, S1, 0x140);
        frame.save(c, S2, 0x130);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x150);
        c.g[S2][0] = c.g[A1][0];
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x120);
        c.g[S0][0] = c.g[A3][0];
        call_ptr(c, 0x003e_b198, 0x003e_bf50)?; // TagFieldCopyTag
        c.g[A1][0] = 0x0049_0000;
        c.g[V1][0] = c.g[V0][0];
        let no_value = c.g[S0][0] == 0;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x65c0));
        // `zero` means the common tail (A0 = S1) runs; `add_one` is its +1.
        let mut zero = no_value;
        let mut add_one = no_value;
        if !no_value {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lb(c);
            if c.g[V0][0] == 0 {
                zero = true;
            } else {
                c.cycles += 1;
                c.g[V0][0] = c.g[S0][0] & 1;
                loop {
                    c.cycles += 2;
                    let bit_clear = c.g[V0][0] == 0;
                    c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 1) as u32);
                    if !bit_clear {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(c.g[A0][0] as u32).lbu(c);
                        c.sb(c.g[V1][0] as u32, c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    }
                    c.cycles += 2;
                    let end = c.g[S0][0] == 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                    if end {
                        zero = true;
                        add_one = true;
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).lb(c);
                    let cont = c.g[V0][0] != 0;
                    c.g[V0][0] = c.g[S0][0] & 1;
                    if !cont {
                        zero = true;
                        add_one = true;
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_bf70);
                    }
                }
            }
        }
        if zero {
            if add_one {
                c.cycles += 1;
            }
            c.g[A0][0] = c.g[S1][0];
        }
        c.cycles += 10;
        c.g[A1][0] = c.g[S2][0];
        c.sb(c.g[V1][0] as u32, 0);
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x003e_b1e0, 0x003e_bfac)?; // TagFieldSetRaw
        frame.restore(c, S0, 0x150);
        frame.restore(c, S1, 0x140);
        frame.restore(c, S2, 0x130);
        frame.restore_ra(c, 0x120);
        frame.close(c);
        Ok(())
    });
}

/// `lbu` of `a + off`.
#[inline(always)]
pub(crate) fn lbu(c: &mut Cpu, a: u32, off: u32) -> u64 {
    Addr(p(a, off)).lbu(c)
}
// D_004960C0: low nibble -> character

pub fn tag_field_set_binary(c: &mut Cpu) {
    guest(c, |c| {
        'b_3ec38c: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1150));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1120));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1110));
            c.g[S2][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1140));
            c.g[S3][0] = c.g[A1][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x1130));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x1100), c.g[RA][0]);
            c.g[S1][0] = c.g[T0][0];
            c.g[S0][0] = c.g[A3][0];
            c.g[RA][0] = sx(0x003ec330);
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003eb198, 0x003ec330)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] = 0x24;
            c.sb(c.g[A0][0] as u32, c.g[V0][0]);
            let t = (c.g[S1][0] as i64) <= 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            if t {
                break 'b_3ec38c;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x490000;
                c.g[V1][0] = 0x490000;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5fc0));
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60c0));
                'l_3ec358: loop {
                    c.cycles += 13;
                    c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                    c.sb(c.g[A0][0] as u32, c.g[V1][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                    c.sb(c.g[A0][0] as u32, c.g[V1][0]);
                    let t = (c.g[S1][0] as i64) > 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if t {
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x003ec358);
                        }
                        continue 'l_3ec358;
                    } else {
                        break 'b_3ec38c;
                    }
                }
            }
        }
        c.cycles += 12;
        c.sb(c.g[A0][0] as u32, 0u64);
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x003ec3a0);
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x003eb1e0, 0x003ec3a0)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1140));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x1130));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1120));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1110));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x1100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1150));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tag fields.

/// `int TagFieldMerge(void* record, void* len, char* data)` (0x3ecb00): merge
/// whitespace-separated `name=value` fields from `data` into the tag record,
/// returning the number of fields accepted.
pub fn tag_field_merge(c: &mut Cpu) {
    let mut pc = c.pc;
    loop {
        match pc {
            0x003e_cb00 => {
                c.cycles += 15;
                c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x70));
                c.sq(S0, p(r32(c, SP), 0x60));
                c.sq(S1, p(r32(c, SP), 0x50));
                c.g[S0][0] = c.g[A2][0];
                c.sq(S2, p(r32(c, SP), 0x40));
                c.g[S1][0] = 0;
                c.sq(S3, p(r32(c, SP), 0x30));
                c.g[S2][0] = c.g[A1][0];
                c.sq(S4, p(r32(c, SP), 0x20));
                c.g[S3][0] = c.g[A0][0];
                c.sq(S5, p(r32(c, SP), 0x10));
                c.g[S4][0] = 0x3a;
                c.sd(r32(c, SP), c.g[RA][0]);
                c.g[S5][0] = 0x3d;
                pc = 0x003e_cb84;
            }
            0x003e_cb3c => {
                c.cycles += 1;
                pc = 0x003e_cb40;
            }
            0x003e_cb40 => {
                c.cycles += 8;
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S0][0];
                if call_ptr(c, 0x003e_b1e0, 0x003e_cb4c).is_err() {
                    return;
                }
                c.g[V1][0] = c.lbu(r32(c, S0));
                c.g[V0][0] = ((0u64 as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[V1][0] = (c.g[V1][0] < 0x20u64) as u64;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                if c.g[V1][0] != 0 {
                    pc = 0x003e_cb84;
                } else {
                    pc = 0x003e_cb60;
                }
            }
            0x003e_cb60 => {
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                pc = 0x003e_cb68;
            }
            0x003e_cb68 => {
                c.cycles += 7;
                c.g[V0][0] = c.lbu(r32(c, S0));
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    if back_edge_unwinds(c, 0x003e_cb68) {
                        return;
                    }
                    pc = 0x003e_cb68;
                } else {
                    pc = 0x003e_cb84;
                }
            }
            0x003e_cb84 => {
                c.cycles += 3;
                c.g[V1][0] = c.lbu(r32(c, S0));
                c.g[V0][0] = (c.g[V1][0] < 0x21u64) as u64;
                if c.g[V1][0] == 0 {
                    pc = 0x003e_cbb0;
                } else {
                    pc = 0x003e_cb90;
                }
            }
            0x003e_cb90 => {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    pc = 0x003e_cba0;
                } else {
                    pc = 0x003e_cb98;
                }
            }
            0x003e_cb98 => {
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                if back_edge_unwinds(c, 0x003e_cb84) {
                    return;
                }
                pc = 0x003e_cb84;
            }
            0x003e_cba0 => {
                c.cycles += 2;
                c.g[V0][0] = c.g[S1][0];
                if c.g[V1][0] == c.g[S5][0] {
                    pc = 0x003e_cbb4;
                } else {
                    pc = 0x003e_cba8;
                }
            }
            0x003e_cba8 => {
                c.cycles += 2;
                c.g[A0][0] = c.g[S3][0];
                if c.g[V1][0] != c.g[S4][0] {
                    if back_edge_unwinds(c, 0x003e_cb40) {
                        return;
                    }
                    pc = 0x003e_cb40;
                } else {
                    pc = 0x003e_cbb0;
                }
            }
            0x003e_cbb0 => {
                c.cycles += 1;
                c.g[V0][0] = c.g[S1][0];
                pc = 0x003e_cbb4;
            }
            0x003e_cbb4 => {
                c.cycles += 9;
                c.lq(S0, p(r32(c, SP), 0x60));
                c.lq(S1, p(r32(c, SP), 0x50));
                c.lq(S2, p(r32(c, SP), 0x40));
                c.lq(S3, p(r32(c, SP), 0x30));
                c.lq(S4, p(r32(c, SP), 0x20));
                c.lq(S5, p(r32(c, SP), 0x10));
                c.g[RA][0] = c.ld(r32(c, SP));
                let ra = r32(c, RA);
                c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x70));
                c.pc = ra;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003e_cb00, 0x003e_cbd8);
                return;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tag fields.

/// `void TagFieldSetNumber(TagField* record, char* out, ...)` (0x3ebe40):
/// format `value` in decimal into a temporary buffer and store it through the
/// tag field's append/terminate helpers. This one is a resume point list: the
/// two division/character loops can be re-entered after a host-thread unwind,
/// so every `pc` the generated dispatcher can continue at is accepted.
pub fn tag_field_set_number(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x003e_be40 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x160));
                    c.sq(17, p(r(c, SP), 0x140));
                    c.sq(18, p(r(c, SP), 0x130));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(16, p(r(c, SP), 0x150));
                    c.g[S2][0] = c.g[A1][0];
                    c.sd(p(r(c, SP), 0x120), c.g[RA][0]);
                    c.g[S0][0] = c.g[A3][0];
                    c.g[A1][0] = c.g[SP][0];
                    callee::TAG_FIELD_COPY_TAG.call(c, 0x003e_be68)?;
                    let non_negative = (c.g[S0][0] as i64) >= 0;
                    c.g[A2][0] = c.g[V0][0];
                    pc = if non_negative {
                        0x003e_be80
                    } else {
                        0x003e_be70
                    };
                }
                0x003e_be70 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x2d;
                    c.g[S0][0] = sx(0u32.wrapping_sub(r(c, S0)));
                    c.sb(r(c, A2), c.g[V0][0]);
                    c.g[A2][0] = sx(p(r(c, A2), 0x1));
                    pc = 0x003e_be80;
                }
                0x003e_be80 => {
                    c.cycles += 3;
                    c.g[A0][0] = sx(p(r(c, A2), 0x1f));
                    let no_digits = (c.g[S0][0] as i64) <= 0;
                    c.sb(p(r(c, A2), 0x1f), 0);
                    pc = if no_digits { 0x003e_bec8 } else { 0x003e_be8c };
                }
                0x003e_be8c => {
                    c.cycles += 3;
                    c.g[A1][0] = 0xa;
                    c.g[A3][0] = 0;
                    c.div(0, 16, 5);
                    pc = 0x003e_be98;
                }
                0x003e_be98 => {
                    c.cycles += 2;
                    if c.g[A1][0] == c.g[A3][0] {
                        pc = 0x003e_bea0;
                    } else {
                        pc = 0x003e_bea0;
                    }
                }
                0x003e_bea0 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx(r(c, A0).wrapping_sub(0x1));
                    c.g[V0][0] = c.hi[0];
                    c.g[V1][0] = c.lo[0];
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(0x30));
                    c.g[S0][0] = c.g[V1][0];
                    c.sb(r(c, A0), c.g[V0][0]);
                    if c.g[A1][0] == c.g[A3][0] {
                        pc = 0x003e_bec0;
                    } else {
                        pc = 0x003e_bec0;
                    }
                }
                0x003e_bec0 => {
                    c.cycles += 2;
                    if (c.g[S0][0] as i64) > 0 {
                        c.div(0, 16, 5);
                        if event_due(c) && deliver_event(c, 0x003e_be98) {
                            return Ok(());
                        }
                        pc = 0x003e_be98;
                    } else {
                        pc = 0x003e_bec8;
                    }
                }
                0x003e_bec8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu(r(c, A0));
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = c.lbu(r(c, A0));
                        pc = 0x003e_bee4;
                    } else {
                        pc = 0x003e_bed4;
                    }
                }
                0x003e_bed4 => {
                    c.cycles += 3;
                    c.g[A0][0] = sx(r(c, A0).wrapping_sub(0x1));
                    c.g[V0][0] = 0x30;
                    c.sb(r(c, A0), c.g[V0][0]);
                    pc = 0x003e_bee0;
                }
                0x003e_bee0 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lbu(r(c, A0));
                    pc = 0x003e_bee4;
                }
                0x003e_bee4 => {
                    c.cycles += 6;
                    c.g[A0][0] = sx(p(r(c, A0), 0x1));
                    c.sb(r(c, A2), c.g[V0][0]);
                    c.g[V0][0] = c.lbu(r(c, A0));
                    let more = c.g[V0][0] != 0;
                    c.g[A2][0] = sx(p(r(c, A2), 0x1));
                    if more {
                        if event_due(c) && deliver_event(c, 0x003e_bee0) {
                            return Ok(());
                        }
                        pc = 0x003e_bee0;
                    } else {
                        pc = 0x003e_befc;
                    }
                }
                0x003e_befc => {
                    c.cycles += 11;
                    c.sb(r(c, A2), 0);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[SP][0];
                    callee::TAG_FIELD_SET_RAW_2.call(c, 0x003e_bf10)?;
                    frame_lq_3(c, 0x150, 0x140, 0x130);
                    c.g[RA][0] = c.ld(p(r(c, SP), 0x120));
                    let ra = r(c, RA);
                    c.g[SP][0] = sx(r(c, SP).wrapping_add(0x160));
                    c.pc = ra;
                    return Ok(());
                }
                other => {
                    c.jump_out(other, 0x003e_be40, 0x003e_bf28);
                    return Ok(());
                }
            }
        }
    })
}

/// `lq s0/s1/s2, off(sp)` in that order.
#[inline(always)]
pub(crate) fn frame_lq_3(c: &mut Cpu, a: u32, b: u32, d: u32) {
    c.lq(16, p(r(c, SP), a));
    c.lq(17, p(r(c, SP), b));
    c.lq(18, p(r(c, SP), d));
}

/// `TagFieldDecodeHex(data, buffer, len)` (0x3ed578): decode `$`-prefixed
/// hex pairs. With a null buffer, count the pairs; otherwise decode up to
/// `len` pairs, returning -1 when the data is not `$`-prefixed or the length
/// is not positive.
///
/// Dispatcher-shaped: an event at a loop back edge may unwind; the machine
/// re-enters at the saved `pc`, so every stage is accepted here.
pub fn tag_field_decode_hex(c: &mut Cpu) {
    let mut pc = c.pc;
    loop {
        match pc {
            0x003e_d578 => {
                c.cycles += 3;
                c.g[T2][0] = c.g[A2][0];
                let t = c.g[A0][0] == 0;
                c.g[T0][0] = c.g[A1][0];
                pc = if t { 0x003e_d594 } else { 0x003e_d584 };
            }
            0x003e_d584 => {
                c.cycles += 4;
                c.g[V1][0] = c.lb(c.g[A0][0] as u32);
                c.g[V0][0] = 0x24;
                let t = c.g[V1][0] == c.g[V0][0];
                pc = if t { 0x003e_d59c } else { 0x003e_d594 };
            }
            0x003e_d594 => {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                abi::ret(c);
                return;
            }
            0x003e_d59c => {
                c.cycles += 2;
                let t = c.g[T0][0] != 0;
                c.g[T1][0] = 0;
                pc = if t { 0x003e_d5dc } else { 0x003e_d5a4 };
            }
            0x003e_d5a4 => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(1));
                c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                pc = 0x003e_d5b8;
            }
            0x003e_d5b0 => {
                c.cycles += 2;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
                c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                pc = 0x003e_d5b8;
            }
            0x003e_d5b8 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 0x30) as u64;
                let t = c.g[V0][0] != 0;
                pc = if t { 0x003e_d5d4 } else { 0x003e_d5c4 };
            }
            0x003e_d5c4 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[V0][0] < 0x30) as u64;
                let t = c.g[V0][0] == 0;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(2));
                if t {
                    if c.cycles >= c.next_event {
                        c.pc = 0x003e_d5b0;
                        if c.event() {
                            return;
                        }
                    }
                    pc = 0x003e_d5b0;
                } else {
                    pc = 0x003e_d5d4;
                }
            }
            0x003e_d5d4 => {
                c.cycles += 2;
                c.g[V0][0] = c.g[T1][0];
                abi::ret(c);
                return;
            }
            0x003e_d5dc => {
                c.cycles += 2;
                let t = (c.g[T2][0] as i64) <= 0;
                c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                if t {
                    if c.cycles >= c.next_event {
                        c.pc = 0x003e_d594;
                        if c.event() {
                            return;
                        }
                    }
                    pc = 0x003e_d594;
                } else {
                    pc = 0x003e_d5e4;
                }
            }
            0x003e_d5e4 => {
                c.cycles += 2;
                let t = (c.g[T2][0] as i64) <= 0;
                c.g[T1][0] = 0;
                pc = if t { 0x003e_d670 } else { 0x003e_d5ec };
            }
            0x003e_d5ec => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[V0][0] < 0x30) as u64;
                let t = c.g[V0][0] != 0;
                pc = if t { 0x003e_d670 } else { 0x003e_d5fc };
            }
            0x003e_d5fc => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[V0][0] < 0x30) as u64;
                let t = c.g[V0][0] != 0;
                c.g[V0][0] = 0x0049_0000;
                pc = if t { 0x003e_d670 } else { 0x003e_d60c };
            }
            0x003e_d60c => {
                c.cycles += 5;
                c.g[V1][0] = 0x0049_0000;
                c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x61c0));
                c.g[T3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x62c0));
                c.g[A1][0] = c.lbu(c.g[A3][0] as u32);
                pc = 0x003e_d620;
            }
            0x003e_d620 => {
                c.cycles += 12;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
                c.g[A0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(1));
                c.g[A2][0] = ((c.g[T1][0] as i64) < (c.g[T2][0] as i64)) as u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T4][0] as u32));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(2));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                c.g[V0][0] |= c.g[V1][0];
                c.sb(c.g[T0][0] as u32, c.g[V0][0]);
                let t = c.g[A2][0] == 0;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                pc = if t { 0x003e_d670 } else { 0x003e_d650 };
            }
            0x003e_d650 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                c.g[V0][0] = (c.g[V0][0] < 0x30) as u64;
                let t = c.g[V0][0] != 0;
                pc = if t { 0x003e_d670 } else { 0x003e_d660 };
            }
            0x003e_d660 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[V0][0] < 0x30) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = c.lbu(c.g[A3][0] as u32);
                    if c.cycles >= c.next_event {
                        c.pc = 0x003e_d620;
                        if c.event() {
                            return;
                        }
                    }
                    pc = 0x003e_d620;
                } else {
                    pc = 0x003e_d670;
                }
            }
            0x003e_d670 => {
                c.cycles += 2;
                c.g[V0][0] = c.g[T1][0];
                abi::ret(c);
                return;
            }
            _ => {
                c.jump_out(pc, 0x003e_d578, 0x003e_d678);
                return;
            }
        }
    }
}

/// `char* TagFieldCopyTag(char* buf, char* dst, const char* name)` (0x3eb198):
/// the append half of the tag writer - copy `name` to `dst`, then a `=`, and
/// return the new end; a null `name` just NUL-terminates `buf`.
pub fn tag_field_copy_tag(c: &mut Cpu) {
    'b_3eb1d8: {
        'b_3eb1a8: {
            c.cycles += 2;
            if c.g[A2][0] != 0u64 {
                c.g[V0][0] = c.lb(c.g[A2][0] as u32);
                break 'b_3eb1a8;
            } else {
                c.cycles += 2;
                let t = true;
                c.sb(c.g[A0][0] as u32, 0u64);
                if t {
                    break 'b_3eb1d8;
                } else {
                    break 'b_3eb1a8;
                }
            }
        }
        'b_3eb1cc: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = c.lbu(c.g[A2][0] as u32);
            if t {
                break 'b_3eb1cc;
            } else {
                'l_3eb1b0: loop {
                    c.cycles += 7;
                    c.sb(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lb(c.g[A2][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lbu(c.g[A2][0] as u32);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003eb1b0);
                        }
                        continue 'l_3eb1b0;
                    } else {
                        break 'b_3eb1cc;
                    }
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = 0x3d;
        c.sb(c.g[A1][0] as u32, c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
        break 'b_3eb1d8;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[A1][0];
    c.pc = t;
    return;
}

/// `int TagFieldSetRaw(char* record, int len, const char* temp)` (0x3eb1e0):
/// add or replace the `name=value` term `temp` in the record, returning the
/// term's name length or -1. Names compare through the class table at
/// `0x4964c0`; fields are separated by the byte at `gp+0x1980`, and `~…`
/// terms are appended at the end.
pub fn tag_field_set_raw(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003eb1e0 => {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.g[T2][0] = c.g[A0][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S5][0] = c.g[A2][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[T2][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    pc = 0x003eb4c0;
                    continue;
                }
                pc = 0x003eb20c;
                continue;
            }
            0x003eb20c => {
                c.cycles += 2;
                let t = c.g[S5][0] == 0u64;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    pc = 0x003eb550;
                    continue;
                }
                pc = 0x003eb214;
                continue;
            }
            0x003eb214 => {
                c.cycles += 5;
                c.g[V0][0] = c.lbu(c.g[S5][0] as u32);
                c.g[T1][0] = c.g[V0][0];
                c.g[V0][0] = (c.g[V0][0] < 0x21u64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    pc = 0x003eb550;
                    continue;
                }
                pc = 0x003eb228;
                continue;
            }
            0x003eb228 => {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[T1][0] as u32) << 24);
                c.g[V1][0] = 0x7e;
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                let t = c.g[V0][0] != c.g[V1][0];
                c.g[S0][0] = c.g[T2][0];
                if t {
                    pc = 0x003eb2c4;
                    continue;
                }
                pc = 0x003eb23c;
                continue;
            }
            0x003eb23c => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                let t = c.g[V0][0] == 0u64;
                if t {
                    pc = 0x003eb26c;
                    continue;
                }
                pc = 0x003eb248;
                continue;
            }
            0x003eb248 => {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                pc = 0x003eb250;
                continue;
            }
            0x003eb250 => {
                c.cycles += 7;
                c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                if c.g[V0][0] != 0u64 {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003eb250) {
                        return;
                    }
                    pc = 0x003eb250;
                    continue;
                }
                pc = 0x003eb26c;
                continue;
            }
            0x003eb26c => {
                c.cycles += 2;
                let t = c.g[S0][0] == c.g[T2][0];
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x8000));
                if t {
                    pc = 0x003eb2b4;
                    continue;
                }
                pc = 0x003eb274;
                continue;
            }
            0x003eb274 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x7fff));
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S1][0] = c.g[S0][0];
                if t {
                    pc = 0x003eb2b8;
                    continue;
                }
                pc = 0x003eb284;
                continue;
            }
            0x003eb284 => {
                c.cycles += 4;
                c.g[V1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x7fff));
                c.g[V0][0] = c.lb((c.g[GP][0] as u32).wrapping_add(0x1980));
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[A0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1980));
                if t {
                    pc = 0x003eb2b8;
                    continue;
                }
                pc = 0x003eb294;
                continue;
            }
            0x003eb294 => {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[S4][0] = c.g[S0][0];
                if t {
                    pc = 0x003eb2bc;
                    continue;
                }
                pc = 0x003eb2a8;
                continue;
            }
            0x003eb2a8 => {
                c.cycles += 3;
                c.sb(c.g[S0][0] as u32, c.g[A0][0]);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.sb(c.g[S0][0] as u32, 0u64);
                pc = 0x003eb2b4;
                continue;
            }
            0x003eb2b4 => {
                c.cycles += 1;
                c.g[S1][0] = c.g[S0][0];
                pc = 0x003eb2b8;
                continue;
            }
            0x003eb2b8 => {
                c.cycles += 1;
                c.g[S4][0] = c.g[S0][0];
                pc = 0x003eb2bc;
                continue;
            }
            0x003eb2bc => {
                c.cycles += 2;
                let t = true;
                c.g[T1][0] = c.lbu(c.g[S5][0] as u32);
                if t {
                    pc = 0x003eb434;
                    continue;
                }
                pc = 0x003eb2c4;
                continue;
            }
            0x003eb2c4 => {
                c.cycles += 6;
                c.g[T6][0] = 0x490000;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[T5][0] = 0x1;
                c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                let t = true;
                c.g[T4][0] = 0x2;
                if t {
                    pc = 0x003eb3dc;
                    continue;
                }
                pc = 0x003eb2dc;
                continue;
            }
            0x003eb2dc => {
                c.cycles += 1;
                pc = 0x003eb2e0;
                continue;
            }
            0x003eb2e0 => {
                c.cycles += 2;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lbu(c.g[S0][0] as u32);
                    pc = 0x003eb2f0;
                    continue;
                }
                pc = 0x003eb2e8;
                continue;
            }
            0x003eb2e8 => {
                c.cycles += 2;
                let t = true;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if t {
                    pc = 0x003eb3dc;
                    continue;
                }
                pc = 0x003eb2f0;
                continue;
            }
            0x003eb2f0 => {
                c.cycles += 5;
                c.g[A0][0] = sx((c.g[T6][0] as u32).wrapping_add(0x64c0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                let t = c.g[V0][0] == c.g[T5][0];
                c.g[V0][0] = sx((c.g[T1][0] as u32) << 24);
                if t {
                    pc = 0x003eb42c;
                    continue;
                }
                pc = 0x003eb304;
                continue;
            }
            0x003eb304 => {
                c.cycles += 6;
                c.g[V1][0] = c.lbu(c.g[V1][0] as u32);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[A2][0] = c.lbu(c.g[V0][0] as u32);
                let t = c.g[V1][0] != c.g[A2][0];
                c.g[S3][0] = 0u64;
                if t {
                    pc = 0x003eb368;
                    continue;
                }
                pc = 0x003eb31c;
                continue;
            }
            0x003eb31c => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V1][0] < 0x2u64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                if t {
                    pc = 0x003eb36c;
                    continue;
                }
                pc = 0x003eb328;
                continue;
            }
            0x003eb328 => {
                c.cycles += 4;
                c.g[T0][0] = c.g[A0][0];
                c.g[A3][0] = c.g[S5][0];
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                pc = 0x003eb338;
                continue;
            }
            0x003eb338 => {
                c.cycles += 10;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                c.g[V1][0] = c.lb(c.g[A3][0] as u32);
                c.g[A0][0] = c.lbu(c.g[V0][0] as u32);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[A2][0] = c.lbu(c.g[V1][0] as u32);
                c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                let t = c.g[V1][0] != c.g[A2][0];
                c.g[V0][0] = (c.g[V1][0] < 0x2u64) as u64;
                if t {
                    pc = 0x003eb368;
                    continue;
                }
                pc = 0x003eb360;
                continue;
            }
            0x003eb360 => {
                c.cycles += 2;
                if c.g[V0][0] == 0u64 {
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003eb338) {
                        return;
                    }
                    pc = 0x003eb338;
                    continue;
                }
                pc = 0x003eb368;
                continue;
            }
            0x003eb368 => {
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                pc = 0x003eb36c;
                continue;
            }
            0x003eb36c => {
                c.cycles += 2;
                if c.g[V0][0] != c.g[T4][0] {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    pc = 0x003eb3c0;
                    continue;
                }
                pc = 0x003eb374;
                continue;
            }
            0x003eb374 => {
                c.cycles += 5;
                c.g[S4][0] = c.g[S0][0];
                c.g[V0][0] = c.lbu(c.g[S4][0] as u32);
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S1][0] = c.g[S0][0];
                if t {
                    pc = 0x003eb3ac;
                    continue;
                }
                pc = 0x003eb388;
                continue;
            }
            0x003eb388 => {
                c.cycles += 2;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                pc = 0x003eb390;
                continue;
            }
            0x003eb390 => {
                c.cycles += 7;
                c.g[V0][0] = c.lbu(c.g[S1][0] as u32);
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if c.g[V0][0] == 0u64 {
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003eb390) {
                        return;
                    }
                    pc = 0x003eb390;
                    continue;
                }
                pc = 0x003eb3ac;
                continue;
            }
            0x003eb3ac => {
                c.cycles += 4;
                c.g[V1][0] = c.lbu(c.g[S1][0] as u32);
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                let t = true;
                if c.g[V1][0] != 0 {
                    c.g[S1][0] = c.g[V0][0];
                }
                if t {
                    pc = 0x003eb434;
                    continue;
                }
                pc = 0x003eb3bc;
                continue;
            }
            0x003eb3bc => {
                c.cycles += 1;
                pc = 0x003eb3c0;
                continue;
            }
            0x003eb3c0 => {
                c.cycles += 7;
                c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if c.g[V0][0] == 0u64 {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003eb3c0) {
                        return;
                    }
                    pc = 0x003eb3c0;
                    continue;
                }
                pc = 0x003eb3dc;
                continue;
            }
            0x003eb3dc => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = (c.g[V0][0] < 0x21u64) as u64;
                if t {
                    if back_edge_unwinds(c, 0x003eb2e0) {
                        return;
                    }
                    pc = 0x003eb2e0;
                    continue;
                }
                pc = 0x003eb3e8;
                continue;
            }
            0x003eb3e8 => {
                c.cycles += 2;
                let t = c.g[S0][0] == c.g[T2][0];
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x8000));
                if t {
                    pc = 0x003eb428;
                    continue;
                }
                pc = 0x003eb3f0;
                continue;
            }
            0x003eb3f0 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x7fff));
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.g[T1][0] = c.lbu(c.g[S5][0] as u32);
                    pc = 0x003eb42c;
                    continue;
                }
                pc = 0x003eb400;
                continue;
            }
            0x003eb400 => {
                c.cycles += 4;
                c.g[V1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x7fff));
                c.g[V0][0] = c.lb((c.g[GP][0] as u32).wrapping_add(0x1980));
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[A0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1980));
                if t {
                    pc = 0x003eb428;
                    continue;
                }
                pc = 0x003eb410;
                continue;
            }
            0x003eb410 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[S0][0] < c.g[T3][0]) as u64;
                if c.g[V0][0] == 0u64 {
                    c.g[T1][0] = c.lbu(c.g[S5][0] as u32);
                    pc = 0x003eb42c;
                    continue;
                }
                pc = 0x003eb41c;
                continue;
            }
            0x003eb41c => {
                c.cycles += 3;
                c.sb(c.g[S0][0] as u32, c.g[A0][0]);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.sb(c.g[S0][0] as u32, 0u64);
                pc = 0x003eb428;
                continue;
            }
            0x003eb428 => {
                c.cycles += 1;
                c.g[T1][0] = c.lbu(c.g[S5][0] as u32);
                pc = 0x003eb42c;
                continue;
            }
            0x003eb42c => {
                c.cycles += 2;
                c.g[S1][0] = c.g[S0][0];
                c.g[S4][0] = c.g[S0][0];
                pc = 0x003eb434;
                continue;
            }
            0x003eb434 => {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[T1][0] as u32) << 24);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 32i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S3][0] = 0u64;
                if t {
                    pc = 0x003eb46c;
                    continue;
                }
                pc = 0x003eb44c;
                continue;
            }
            0x003eb44c => {
                c.cycles += 1;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                pc = 0x003eb450;
                continue;
            }
            0x003eb450 => {
                c.cycles += 7;
                c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S3][0] as u32));
                c.g[V0][0] = c.lb(c.g[V1][0] as u32);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 32i64) as u64;
                if c.g[V0][0] == 0u64 {
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003eb450) {
                        return;
                    }
                    pc = 0x003eb450;
                    continue;
                }
                pc = 0x003eb46c;
                continue;
            }
            0x003eb46c => {
                c.cycles += 5;
                c.g[S0][0] = c.g[S1][0];
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                let t = c.g[V0][0] == 0u64;
                c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                if t {
                    pc = 0x003eb4a4;
                    continue;
                }
                pc = 0x003eb480;
                continue;
            }
            0x003eb480 => {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                pc = 0x003eb488;
                continue;
            }
            0x003eb488 => {
                c.cycles += 7;
                c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                if c.g[V0][0] != 0u64 {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003eb488) {
                        return;
                    }
                    pc = 0x003eb488;
                    continue;
                }
                pc = 0x003eb4a4;
                continue;
            }
            0x003eb4a4 => {
                c.cycles += 2;
                let t = (c.g[S2][0] as i64) <= 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if t {
                    pc = 0x003eb4c8;
                    continue;
                }
                pc = 0x003eb4ac;
                continue;
            }
            0x003eb4ac => {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                if t {
                    pc = 0x003eb4c8;
                    continue;
                }
                pc = 0x003eb4c0;
                continue;
            }
            0x003eb4c0 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    pc = 0x003eb550;
                    continue;
                }
                pc = 0x003eb4c8;
                continue;
            }
            0x003eb4c8 => {
                c.cycles += 2;
                let t = c.g[S2][0] != 0u64;
                c.g[A0][0] = c.g[S4][0];
                if t {
                    pc = 0x003eb4e4;
                    continue;
                }
                pc = 0x003eb4d0;
                continue;
            }
            0x003eb4d0 => {
                c.cycles += 5;
                c.g[A1][0] = c.g[S5][0];
                c.g[A2][0] = c.g[S3][0];
                if call_ptr(c, 0x00415fc8, 0x003eb4dc).is_err() {
                    return;
                }
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0u64;
                if t {
                    pc = 0x003eb550;
                    continue;
                }
                pc = 0x003eb4e4;
                continue;
            }
            0x003eb4e4 => {
                c.cycles += 2;
                let t = (c.g[S2][0] as i64) <= 0;
                c.g[A1][0] = c.g[S1][0];
                if t {
                    pc = 0x003eb4f8;
                    continue;
                }
                pc = 0x003eb4ec;
                continue;
            }
            0x003eb4ec => {
                c.cycles += 3;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                if call_ptr(c, 0x0041610c, 0x003eb4f8).is_err() {
                    return;
                }
                pc = 0x003eb4f8;
                continue;
            }
            0x003eb4f8 => {
                c.cycles += 2;
                let t = (c.g[S2][0] as i64) >= 0;
                c.g[A0][0] = c.g[S4][0];
                if t {
                    pc = 0x003eb510;
                    continue;
                }
                pc = 0x003eb500;
                continue;
            }
            0x003eb500 => {
                c.cycles += 4;
                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S2][0] as u32));
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                if call_ptr(c, 0x0041610c, 0x003eb50c).is_err() {
                    return;
                }
                c.g[A0][0] = c.g[S4][0];
                pc = 0x003eb510;
                continue;
            }
            0x003eb510 => {
                c.cycles += 7;
                c.g[A1][0] = c.g[S5][0];
                c.g[A2][0] = c.g[S3][0];
                if call_ptr(c, 0x0041605c, 0x003eb51c).is_err() {
                    return;
                }
                c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S3][0] as u32));
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = c.g[V1][0];
                if t {
                    pc = 0x003eb544;
                    continue;
                }
                pc = 0x003eb52c;
                continue;
            }
            0x003eb52c => {
                c.cycles += 4;
                c.g[V1][0] = c.lb((c.g[GP][0] as u32).wrapping_add(0x1980));
                c.g[V0][0] = 0xa;
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1980));
                if t {
                    pc = 0x003eb548;
                    continue;
                }
                pc = 0x003eb53c;
                continue;
            }
            0x003eb53c => {
                c.cycles += 2;
                let t = true;
                c.sb((c.g[A0][0] as u32).wrapping_sub(0x1), 0u64);
                if t {
                    pc = 0x003eb54c;
                    continue;
                }
                pc = 0x003eb544;
                continue;
            }
            0x003eb544 => {
                c.cycles += 1;
                c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1980));
                pc = 0x003eb548;
                continue;
            }
            0x003eb548 => {
                c.cycles += 1;
                c.sb((c.g[A0][0] as u32).wrapping_sub(0x1), c.g[V0][0]);
                pc = 0x003eb54c;
                continue;
            }
            0x003eb54c => {
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                pc = 0x003eb550;
                continue;
            }
            0x003eb550 => {
                c.cycles += 9;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003eb1e0, 0x003eb574);
                return;
            }
        }
    }
}

/// `char* TagFieldFind(char* record, const char* name)` (0x3eb6d8): find the
/// `name=` field in the record and return the pointer to its value (after the
/// `=` or `:`), or 0 when the record or the name is missing or not found.
pub fn tag_field_find(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003eb6d8 => {
                c.cycles += 5;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[T5][0] = c.g[A0][0];
                c.sq(16, c.g[SP][0] as u32);
                let t = c.g[T5][0] == 0u64;
                c.g[T6][0] = c.g[A1][0];
                if t {
                    pc = 0x003eb82c;
                    continue;
                }
                pc = 0x003eb6ec;
                continue;
            }
            0x003eb6ec => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu(c.g[T5][0] as u32);
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0u64;
                if t {
                    pc = 0x003eb830;
                    continue;
                }
                pc = 0x003eb6f8;
                continue;
            }
            0x003eb6f8 => {
                c.cycles += 2;
                let t = c.g[T6][0] == 0u64;
                c.lq(16, c.g[SP][0] as u32);
                if t {
                    pc = 0x003eb834;
                    continue;
                }
                pc = 0x003eb700;
                continue;
            }
            0x003eb700 => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu(c.g[T6][0] as u32);
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0u64;
                if t {
                    pc = 0x003eb834;
                    continue;
                }
                pc = 0x003eb70c;
                continue;
            }
            0x003eb70c => {
                c.cycles += 3;
                c.g[T4][0] = c.g[T6][0];
                c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                pc = 0x003eb718;
                continue;
            }
            0x003eb718 => {
                c.cycles += 7;
                c.g[V0][0] = c.lbu(c.g[T4][0] as u32);
                if c.g[V0][0] != 0u64 {
                    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003eb718) {
                        return;
                    }
                    pc = 0x003eb718;
                    continue;
                }
                pc = 0x003eb734;
                continue;
            }
            0x003eb734 => {
                c.cycles += 8;
                c.g[V0][0] = 0x490000;
                c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x1));
                c.g[S0][0] = c.g[V0][0];
                c.g[T7][0] = sx((c.g[V0][0] as u32).wrapping_add(0x63c0));
                c.g[T0][0] = c.g[T5][0];
                c.g[T9][0] = 0x3d;
                let t = true;
                c.g[T8][0] = 0x3a;
                if t {
                    pc = 0x003eb7ec;
                    continue;
                }
                pc = 0x003eb754;
                continue;
            }
            0x003eb754 => {
                c.cycles += 1;
                pc = 0x003eb758;
                continue;
            }
            0x003eb758 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[A2][0] < c.g[T5][0]) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A3][0] = c.g[T4][0];
                if t {
                    pc = 0x003eb7e8;
                    continue;
                }
                pc = 0x003eb764;
                continue;
            }
            0x003eb764 => {
                c.cycles += 8;
                c.g[A0][0] = c.lbu((c.g[T0][0] as u32).wrapping_sub(0x1));
                c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T7][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T7][0] as u32));
                c.g[A1][0] = c.lbu(c.g[A0][0] as u32);
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[T3][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                if t {
                    pc = 0x003eb7e8;
                    continue;
                }
                pc = 0x003eb784;
                continue;
            }
            0x003eb784 => {
                c.cycles += 5;
                c.g[V0][0] = c.g[T1][0] & 0xff;
                c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x63c0));
                c.g[T1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = c.g[V0][0] ^ 0x20;
                if c.g[V0][0] != 0 {
                    c.g[T1][0] = c.g[T3][0];
                }
                pc = 0x003eb798;
                continue;
            }
            0x003eb798 => {
                c.cycles += 2;
                if c.g[A3][0] != c.g[T6][0] {
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    pc = 0x003eb7bc;
                    continue;
                }
                pc = 0x003eb7a0;
                continue;
            }
            0x003eb7a0 => {
                c.cycles += 2;
                let t = c.g[A2][0] == c.g[T5][0];
                c.g[V0][0] = c.g[T1][0];
                if t {
                    pc = 0x003eb830;
                    continue;
                }
                pc = 0x003eb7a8;
                continue;
            }
            0x003eb7a8 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A2][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = (c.g[V0][0] < 0x21u64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.g[T1][0];
                if t {
                    pc = 0x003eb830;
                    continue;
                }
                pc = 0x003eb7b8;
                continue;
            }
            0x003eb7b8 => {
                c.cycles += 1;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                pc = 0x003eb7bc;
                continue;
            }
            0x003eb7bc => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[A2][0] < c.g[T5][0]) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                if t {
                    pc = 0x003eb7e8;
                    continue;
                }
                pc = 0x003eb7c8;
                continue;
            }
            0x003eb7c8 => {
                c.cycles += 8;
                c.g[V1][0] = c.lbu(c.g[A2][0] as u32);
                c.g[A0][0] = c.lbu(c.g[A3][0] as u32);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                c.g[A1][0] = c.lbu(c.g[V1][0] as u32);
                c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                let t = c.g[A1][0] == c.g[V0][0];
                if t {
                    if back_edge_unwinds(c, 0x003eb798) {
                        return;
                    }
                    pc = 0x003eb798;
                    continue;
                }
                pc = 0x003eb7e8;
                continue;
            }
            0x003eb7e8 => {
                c.cycles += 1;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                pc = 0x003eb7ec;
                continue;
            }
            0x003eb7ec => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu(c.g[T0][0] as u32);
                if c.g[V0][0] == 0u64 {
                    c.g[V0][0] = 0u64;
                    pc = 0x003eb830;
                    continue;
                }
                pc = 0x003eb7f8;
                continue;
            }
            0x003eb7f8 => {
                c.cycles += 2;
                if c.g[V0][0] == c.g[T9][0] {
                    c.g[V0][0] = c.lbu((c.g[T0][0] as u32).wrapping_add(0x1));
                    pc = 0x003eb80c;
                    continue;
                }
                pc = 0x003eb800;
                continue;
            }
            0x003eb800 => {
                c.cycles += 2;
                if c.g[V0][0] != c.g[T8][0] {
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003eb7ec) {
                        return;
                    }
                    pc = 0x003eb7ec;
                    continue;
                }
                pc = 0x003eb808;
                continue;
            }
            0x003eb808 => {
                c.cycles += 1;
                c.g[V0][0] = c.lbu((c.g[T0][0] as u32).wrapping_add(0x1));
                pc = 0x003eb80c;
                continue;
            }
            0x003eb80c => {
                c.cycles += 4;
                c.g[T1][0] = c.g[V0][0];
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if c.g[V0][0] == 0u64 {
                    c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                    if back_edge_unwinds(c, 0x003eb758) {
                        return;
                    }
                    pc = 0x003eb758;
                    continue;
                }
                pc = 0x003eb81c;
                continue;
            }
            0x003eb81c => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[T0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = (c.g[V0][0] < 0x21u64) as u64;
                if c.g[V0][0] == 0u64 {
                    c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                    if back_edge_unwinds(c, 0x003eb758) {
                        return;
                    }
                    pc = 0x003eb758;
                    continue;
                }
                pc = 0x003eb82c;
                continue;
            }
            0x003eb82c => {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                pc = 0x003eb830;
                continue;
            }
            0x003eb830 => {
                c.cycles += 1;
                c.lq(16, c.g[SP][0] as u32);
                pc = 0x003eb834;
                continue;
            }
            0x003eb834 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003eb6d8, 0x003eb83c);
                return;
            }
        }
    }
}

/// `char* TagFieldFind2(char* record, const char* prefix, const char* name)`
/// (0x3eb840): find `name` (or `prefix`, or `prefix+name` concatenated) in the
/// record.
pub fn tag_field_find_2(c: &mut Cpu) {
    guest(c, |c| {
        'b_3eb8a8: {
            'b_3eb86c: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.g[S0][0] = c.g[A2][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
                let t = c.g[A1][0] != 0u64;
                c.g[S1][0] = c.g[A0][0];
                if t {
                    break 'b_3eb86c;
                } else {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x003eb6d8, 0x003eb864)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                    if t {
                        break 'b_3eb8a8;
                    } else {
                        break 'b_3eb86c;
                    }
                }
            }
            'b_3eb884: {
                c.cycles += 2;
                let t = c.g[S0][0] != 0u64;
                if t {
                    break 'b_3eb884;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x003eb6d8, 0x003eb87c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                    if t {
                        break 'b_3eb8a8;
                    } else {
                        break 'b_3eb884;
                    }
                }
            }
            c.cycles += 9;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x004166f4, 0x003eb88c)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x004162d0, 0x003eb898)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003eb6d8, 0x003eb8a4)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
            break 'b_3eb8a8;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
        c.pc = t;
        Ok(())
    })
}

/// `char* TagFieldFindSkipWs(char* record)` (0x3eb8b8): find the `tag` field
/// and skip the leading whitespace of its value; 0 when the value is empty.
pub fn tag_field_find_skip_ws(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003eb8b8 => {
                c.cycles += 8;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[A1][0] = 0x4a0000;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a78));
                if call_ptr(c, 0x003eb6d8, 0x003eb8cc).is_err() {
                    return;
                }
                c.g[V1][0] = c.g[V0][0];
                let t = c.g[V1][0] == 0u64;
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                if t {
                    pc = 0x003eb908;
                    continue;
                }
                pc = 0x003eb8d8;
                continue;
            }
            0x003eb8d8 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                if t {
                    pc = 0x003eb8e4;
                    continue;
                }
                pc = 0x003eb8e0;
                continue;
            }
            0x003eb8e0 => {
                c.cycles += 1;
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                pc = 0x003eb8e4;
                continue;
            }
            0x003eb8e4 => {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003eb8e0) {
                        return;
                    }
                    pc = 0x003eb8e0;
                    continue;
                }
                pc = 0x003eb8fc;
                continue;
            }
            0x003eb8fc => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = 0u64;
                }
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                pc = 0x003eb908;
                continue;
            }
            0x003eb908 => {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0];
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003eb8b8, 0x003eb914);
                return;
            }
        }
    }
}

/// `int TagFieldDelete(char* record, const char* name)` (0x3eb918): remove the
/// `name=` field from the record, returning 0 on success or -1 when absent.
pub fn tag_field_delete(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003eb918 => {
                c.cycles += 8;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S0][0] = c.g[A0][0];
                if call_ptr(c, 0x003eb6d8, 0x003eb92c).is_err() {
                    return;
                }
                c.g[V1][0] = c.g[V0][0];
                let t = c.g[V1][0] != 0u64;
                c.g[A0][0] = c.g[V1][0];
                if t {
                    pc = 0x003eb940;
                    continue;
                }
                pc = 0x003eb938;
                continue;
            }
            0x003eb938 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    pc = 0x003eb9d4;
                    continue;
                }
                pc = 0x003eb940;
                continue;
            }
            0x003eb940 => {
                c.cycles += 2;
                if c.g[A0][0] == c.g[S0][0] {
                    c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                    pc = 0x003eb964;
                    continue;
                }
                pc = 0x003eb948;
                continue;
            }
            0x003eb948 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = (c.g[V0][0] < 0x21u64) as u64;
                if c.g[V0][0] == 0u64 {
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    if back_edge_unwinds(c, 0x003eb940) {
                        return;
                    }
                    pc = 0x003eb940;
                    continue;
                }
                pc = 0x003eb958;
                continue;
            }
            0x003eb958 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                if t {
                    pc = 0x003eb964;
                    continue;
                }
                pc = 0x003eb960;
                continue;
            }
            0x003eb960 => {
                c.cycles += 1;
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                pc = 0x003eb964;
                continue;
            }
            0x003eb964 => {
                c.cycles += 6;
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003eb960) {
                        return;
                    }
                    pc = 0x003eb960;
                    continue;
                }
                pc = 0x003eb97c;
                continue;
            }
            0x003eb97c => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                if t {
                    pc = 0x003eb98c;
                    continue;
                }
                pc = 0x003eb984;
                continue;
            }
            0x003eb984 => {
                c.cycles += 1;
                pc = 0x003eb988;
                continue;
            }
            0x003eb988 => {
                c.cycles += 1;
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                pc = 0x003eb98c;
                continue;
            }
            0x003eb98c => {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = (c.g[V0][0] < 0x1fu64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003eb988) {
                        return;
                    }
                    pc = 0x003eb988;
                    continue;
                }
                pc = 0x003eb9a4;
                continue;
            }
            0x003eb9a4 => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                if c.g[V0][0] == 0u64 {
                    c.sb(c.g[A0][0] as u32, 0u64);
                    pc = 0x003eb9d0;
                    continue;
                }
                pc = 0x003eb9b0;
                continue;
            }
            0x003eb9b0 => {
                c.cycles += 7;
                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                if t {
                    if back_edge_unwinds(c, 0x003eb9b0) {
                        return;
                    }
                    pc = 0x003eb9b0;
                    continue;
                }
                pc = 0x003eb9cc;
                continue;
            }
            0x003eb9cc => {
                c.cycles += 1;
                c.sb(c.g[A0][0] as u32, 0u64);
                pc = 0x003eb9d0;
                continue;
            }
            0x003eb9d0 => {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                pc = 0x003eb9d4;
                continue;
            }
            0x003eb9d4 => {
                c.cycles += 4;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003eb918, 0x003eb9e4);
                return;
            }
        }
    }
}

/// `int TagFieldRename(char* record, int len, const char* name, const char* value)`
/// (0x3eb9e8): replace the `name` field's value with `value`, shifting the
/// record in place; 0 on success, -1 when the field or room is missing.
pub fn tag_field_rename(c: &mut Cpu) {
    guest(c, |c| {
        'b_3ebb40: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S3][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S4][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S2][0] = c.g[A3][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A1][0] = c.g[A2][0];
            call_ptr(c, 0x003eb6d8, 0x003eba18)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffffff;
            if t {
                break 'b_3ebb40;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_sub(0x1));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = c.g[V0][0] ^ 0x20;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = c.g[V1][0];
                }
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                c.g[S0][0] = c.g[S1][0];
                'l_3eba40: loop {
                    'b_3eba58: {
                        c.cycles += 2;
                        let t = c.g[S1][0] == c.g[S3][0];
                        if t {
                            break 'b_3eba58;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_sub(0x1));
                            c.g[V0][0] = (c.g[V0][0] < 0x21u64) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                if event_due(c) {
                                    deliver_event_here(c, 0x003eba40);
                                }
                                continue 'l_3eba40;
                            } else {
                                break 'b_3eba58;
                            }
                        }
                    }
                    'b_3ebaa8: {
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x00416810, 0x003eba60)?;
                        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                        let t = (c.g[A3][0] as i64) >= 0;
                        if t {
                            break 'b_3ebaa8;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                            if c.g[V0][0] == 0u64 {
                                c.sb(c.g[S0][0] as u32, 0u64);
                                break 'b_3ebaa8;
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                                c.sb(c.g[V1][0] as u32, c.g[V0][0]);
                                'l_3eba88: loop {
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                                    if c.g[V0][0] != 0u64 {
                                        c.sb(c.g[V1][0] as u32, c.g[V0][0]);
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003eba88);
                                        }
                                        continue 'l_3eba88;
                                    } else {
                                        c.cycles += 1;
                                        c.sb(c.g[S0][0] as u32, 0u64);
                                        break 'b_3ebaa8;
                                    }
                                }
                            }
                        }
                    }
                    'b_3ebb18: {
                        'b_3ebb14: {
                            c.cycles += 2;
                            let t = (c.g[A3][0] as i64) <= 0;
                            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[A3][0] as u32));
                            if t {
                                break 'b_3ebb14;
                            } else {
                                'b_3ebadc: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A2][0] = c.g[S0][0];
                                    if t {
                                        break 'b_3ebadc;
                                    } else {
                                        c.cycles += 1;
                                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                        'l_3ebac0: loop {
                                            c.cycles += 7;
                                            c.g[V0][0] = c.lbu(c.g[A2][0] as u32);
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A2][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x003ebac0);
                                                }
                                                continue 'l_3ebac0;
                                            } else {
                                                break 'b_3ebadc;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_sub(c.g[S3][0] as u32));
                                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0xffffffffffffffff;
                                if t {
                                    break 'b_3ebb40;
                                } else {
                                    c.cycles += 2;
                                    if c.g[A2][0] == c.g[S0][0] {
                                        c.g[V0][0] = c.lb(c.g[S2][0] as u32);
                                        break 'b_3ebb18;
                                    } else {
                                        c.cycles += 1;
                                        'l_3ebaf8: loop {
                                            c.cycles += 7;
                                            c.g[V1][0] = c.lbu(c.g[A2][0] as u32);
                                            c.g[V0][0] =
                                                sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A3][0] as u32));
                                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                            let t = c.g[A2][0] != c.g[S0][0];
                                            c.sb(c.g[V0][0] as u32, c.g[V1][0]);
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x003ebaf8);
                                                }
                                                continue 'l_3ebaf8;
                                            } else {
                                                break 'b_3ebb14;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lb(c.g[S2][0] as u32);
                        break 'b_3ebb18;
                    }
                    'b_3ebb3c: {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = c.lbu(c.g[S2][0] as u32);
                        if t {
                            break 'b_3ebb3c;
                        } else {
                            'l_3ebb20: loop {
                                c.cycles += 7;
                                c.sb(c.g[S1][0] as u32, c.g[V1][0]);
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = c.lb(c.g[S2][0] as u32);
                                let t = c.g[V0][0] != 0u64;
                                c.g[V1][0] = c.lbu(c.g[S2][0] as u32);
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003ebb20);
                                    }
                                    continue 'l_3ebb20;
                                } else {
                                    break 'b_3ebb3c;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    break 'b_3ebb40;
                }
            }
        }
        c.cycles += 8;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        Ok(())
    })
}

/// `int TagFieldSetAddress(char* record, int len, const char* name, unsigned int addr)`
/// (0x3ebfc8): store an IPv4 address field in dotted-decimal form.
pub fn tag_field_set_address(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x180));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.g[S2][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.g[S3][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A3][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x130), c.g[RA][0]);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x003eb198, 0x003ebff8)?;
        c.sb((c.g[SP][0] as u32).wrapping_add(0x3), c.g[S0][0]);
        c.g[A2][0] = c.g[S1][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32) >> 8);
        c.g[T2][0] = 0x2e;
        c.sb((c.g[SP][0] as u32).wrapping_add(0x2), c.g[S0][0]);
        c.g[T1][0] = 0x64;
        c.g[S0][0] = sx((c.g[S0][0] as u32) >> 8);
        c.g[T0][0] = 0u64;
        c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[S0][0]);
        c.g[A3][0] = 0xa;
        c.g[S0][0] = sx((c.g[S0][0] as u32) >> 8);
        c.g[A0][0] = c.g[V0][0];
        c.sb(c.g[SP][0] as u32, c.g[S0][0]);
        c.g[A1][0] = 0u64;
        'l_3ec030: loop {
            'b_3ec044: {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[A1][0] as u32));
                let t = (c.g[A1][0] as i64) <= 0;
                c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                if t {
                    break 'b_3ec044;
                } else {
                    c.cycles += 2;
                    c.sb(c.g[A0][0] as u32, c.g[T2][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    break 'b_3ec044;
                }
            }
            'b_3ec0b8: {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_3ec0b8;
                } else {
                    'b_3ec08c: {
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[V1][0] < 0x64u64) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.divu(0, 3, 7);
                            break 'b_3ec08c;
                        } else {
                            'b_3ec068: {
                                c.cycles += 3;
                                c.divu(0, 3, 9);
                                if c.g[T1][0] == c.g[T0][0] {
                                    break 'b_3ec068;
                                } else {
                                    break 'b_3ec068;
                                }
                            }
                            'b_3ec084: {
                                c.cycles += 7;
                                c.g[V0][0] = c.lo[0];
                                c.g[V1][0] = c.hi[0];
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
                                c.g[V1][0] = c.g[V1][0] & 0xff;
                                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                                if c.g[T1][0] == c.g[T0][0] {
                                    break 'b_3ec084;
                                } else {
                                    break 'b_3ec084;
                                }
                            }
                            c.cycles += 2;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            c.divu(0, 3, 7);
                            break 'b_3ec08c;
                        }
                    }
                    'b_3ec094: {
                        c.cycles += 2;
                        if c.g[A3][0] == c.g[T0][0] {
                            break 'b_3ec094;
                        } else {
                            break 'b_3ec094;
                        }
                    }
                    'b_3ec0b0: {
                        c.cycles += 7;
                        c.g[V0][0] = c.lo[0];
                        c.g[V1][0] = c.hi[0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
                        c.g[V1][0] = c.g[V1][0] & 0xff;
                        c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                        if c.g[A3][0] == c.g[T0][0] {
                            break 'b_3ec0b0;
                        } else {
                            break 'b_3ec0b0;
                        }
                    }
                    c.cycles += 2;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                    break 'b_3ec0b8;
                }
            }
            c.cycles += 5;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.sb(c.g[A0][0] as u32, c.g[V0][0]);
            c.g[V1][0] = ((c.g[A1][0] as i64) < 4i64) as u64;
            let t = c.g[V1][0] != 0u64;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x003ec030);
                }
                continue 'l_3ec030;
            } else {
                c.cycles += 11;
                c.sb(c.g[A0][0] as u32, 0u64);
                c.g[A1][0] = c.g[S3][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x003eb1e0, 0x003ec0dc)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x130));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x180));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `int TagFieldSetString(char* record, int len, const char* name, const char* value)`
/// (0x3ec180): store a string value, quoting it when it contains whitespace or
/// `%`-escapes; a null value deletes the field.
pub fn tag_field_set_string(c: &mut Cpu) {
    guest(c, |c| {
        'b_3ec2e4: {
            'b_3ec1b8: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa40));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa10));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa30));
                c.g[S2][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa20));
                c.g[A1][0] = c.g[A2][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0xa00), c.g[RA][0]);
                c.g[S0][0] = c.g[A3][0];
                let t = c.g[S0][0] != 0u64;
                c.g[S1][0] = c.g[A0][0];
                if t {
                    break 'b_3ec1b8;
                } else {
                    c.cycles += 4;
                    call_ptr(c, 0x003eb918, 0x003ec1b0)?;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_3ec2e4;
                    } else {
                        break 'b_3ec1b8;
                    }
                }
            }
            'b_3ec21c: {
                'b_3ec20c: {
                    'b_3ec200: {
                        c.cycles += 8;
                        c.g[A2][0] = c.g[A1][0];
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, 0x003eb198, 0x003ec1c8)?;
                        c.g[A2][0] = c.g[S0][0];
                        c.g[V1][0] = c.lbu(c.g[A2][0] as u32);
                        let t = c.g[V1][0] == 0u64;
                        c.g[A0][0] = c.g[V0][0];
                        if t {
                            break 'b_3ec200;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x20;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V1][0] = 0x20;
                            if t {
                                break 'b_3ec20c;
                            } else {
                                c.cycles += 1;
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                'l_3ec1e8: loop {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lbu(c.g[A2][0] as u32);
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V1][0] = c.lbu(c.g[A2][0] as u32);
                                        break 'b_3ec200;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[V0][0] != c.g[V1][0] {
                                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x003ec1e8);
                                            }
                                            continue 'l_3ec1e8;
                                        } else {
                                            c.cycles += 1;
                                            c.g[V1][0] = c.lbu(c.g[A2][0] as u32);
                                            break 'b_3ec200;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x20;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A1][0] = c.lbu(c.g[S0][0] as u32);
                        break 'b_3ec21c;
                    } else {
                        break 'b_3ec20c;
                    }
                }
                c.cycles += 4;
                c.g[V0][0] = 0x22;
                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = c.lbu(c.g[S0][0] as u32);
                break 'b_3ec21c;
            }
            'b_3ec2b4: {
                c.cycles += 2;
                let t = c.g[A1][0] == 0u64;
                c.g[V0][0] = 0x490000;
                if t {
                    break 'b_3ec2b4;
                } else {
                    c.cycles += 9;
                    c.g[V1][0] = 0x490000;
                    c.g[T5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5fc0));
                    c.g[T4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60c0));
                    c.g[T3][0] = 0x3d;
                    c.g[T2][0] = 0x22;
                    c.g[T1][0] = 0x3a;
                    c.g[T0][0] = 0x25;
                    c.g[A3][0] = 0x25;
                    'l_3ec248: loop {
                        'b_3ec2a4: {
                            'b_3ec27c: {
                                'b_3ec278: {
                                    c.cycles += 4;
                                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x20));
                                    c.g[V0][0] = (c.g[V0][0] < 0x5fu64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.g[A1][0] & 0xff;
                                    if t {
                                        break 'b_3ec278;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[V0][0] == c.g[T3][0] {
                                            c.sb(c.g[A0][0] as u32, c.g[A3][0]);
                                            break 'b_3ec27c;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[V0][0] == c.g[T2][0] {
                                                c.sb(c.g[A0][0] as u32, c.g[A3][0]);
                                                break 'b_3ec27c;
                                            } else {
                                                c.cycles += 2;
                                                if c.g[V0][0] == c.g[T1][0] {
                                                    c.sb(c.g[A0][0] as u32, c.g[A3][0]);
                                                    break 'b_3ec27c;
                                                } else {
                                                    c.cycles += 2;
                                                    if c.g[V0][0] != c.g[T0][0] {
                                                        c.sb(c.g[A0][0] as u32, c.g[A1][0]);
                                                        break 'b_3ec2a4;
                                                    } else {
                                                        break 'b_3ec278;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.sb(c.g[A0][0] as u32, c.g[A3][0]);
                                break 'b_3ec27c;
                            }
                            c.cycles += 10;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T5][0] as u32));
                            c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                            c.sb(c.g[A0][0] as u32, c.g[V1][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
                            c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                            c.sb(c.g[A0][0] as u32, c.g[V1][0]);
                            break 'b_3ec2a4;
                        }
                        c.cycles += 4;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        c.g[A1][0] = c.lbu(c.g[S0][0] as u32);
                        let t = c.g[A1][0] != 0u64;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x003ec248);
                            }
                            continue 'l_3ec248;
                        } else {
                            break 'b_3ec2b4;
                        }
                    }
                }
            }
            'b_3ec2d4: {
                c.cycles += 4;
                c.g[V1][0] = c.lbu(c.g[A2][0] as u32);
                c.g[V0][0] = 0x20;
                if c.g[V1][0] != c.g[V0][0] {
                    c.sb(c.g[A0][0] as u32, 0u64);
                    break 'b_3ec2d4;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = 0x22;
                    c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.sb(c.g[A0][0] as u32, 0u64);
                    break 'b_3ec2d4;
                }
            }
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.g[SP][0];
            call_ptr(c, 0x003eb1e0, 0x003ec2e4)?;
            break 'b_3ec2e4;
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa00));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa40));
        c.pc = t;
        Ok(())
    })
}

/// `int TagFieldSetStructure(char* record, int len, const char* name, void* data, int size)`
/// (0x3ec3c0): store a structure field by walking a template string (`#…`, a
/// count, then `b`/`w`/`l`/`s`) and hex-encoding the matching data.
pub fn tag_field_set_structure(c: &mut Cpu) {
    guest(c, |c| {
        'b_3ec408: {
            'b_3ec404: {
                c.cycles += 14;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa70));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa60));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa50));
                c.g[S0][0] = c.g[T1][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa40));
                c.g[S1][0] = c.g[A3][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xa20));
                c.g[S2][0] = 0u64;
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xa10));
                c.g[S4][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa30));
                c.g[S5][0] = c.g[A1][0];
                let t = (c.g[T0][0] as i64) >= 0;
                c.sd((c.g[SP][0] as u32).wrapping_add(0xa00), c.g[RA][0]);
                if t {
                    break 'b_3ec404;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x10000;
                    let t = true;
                    c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if t {
                        break 'b_3ec408;
                    } else {
                        break 'b_3ec404;
                    }
                }
            }
            c.cycles += 1;
            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[T0][0] as u32));
            break 'b_3ec408;
        }
        'b_3ec660: {
            'b_3ec65c: {
                c.cycles += 7;
                c.g[A1][0] = c.g[SP][0];
                c.g[A0][0] = c.g[S4][0];
                call_ptr(c, 0x003eb198, 0x003ec414)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V1][0] = c.lb(c.g[S0][0] as u32);
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                if t {
                    break 'b_3ec65c;
                } else {
                    c.cycles += 3;
                    c.g[T6][0] = 0x61;
                    c.g[T5][0] = 0x2c;
                    c.g[T4][0] = 0x490000;
                    'l_3ec430: loop {
                        'b_3ec468: {
                            c.cycles += 5;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                            c.g[V1][0] = 0x23;
                            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                            if c.g[V0][0] != c.g[V1][0] {
                                c.g[A2][0] = c.lbu(c.g[S0][0] as u32);
                                break 'b_3ec468;
                            } else {
                                c.cycles += 3;
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[V1][0] = 0x3d;
                                'l_3ec450: loop {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                                    if c.g[V0][0] == 0u64 {
                                        c.g[A2][0] = c.lbu(c.g[S0][0] as u32);
                                        break 'b_3ec468;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] != c.g[V1][0];
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x003ec450);
                                            }
                                            continue 'l_3ec450;
                                        } else {
                                            c.cycles += 1;
                                            c.g[A2][0] = c.lbu(c.g[S0][0] as u32);
                                            break 'b_3ec468;
                                        }
                                    }
                                }
                            }
                        }
                        'b_3ec4a0: {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x30));
                            c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[T0][0] = 0u64;
                            if t {
                                break 'b_3ec4a0;
                            } else {
                                c.cycles += 2;
                                c.g[A3][0] = 0xa;
                                'l_3ec480: loop {
                                    c.cycles += 8;
                                    c.g[A0][0] = c.g[A2][0] & 0xf;
                                    c.mult(0, 3, 8, 7);
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    c.g[A2][0] = c.lbu(c.g[S0][0] as u32);
                                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x30));
                                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[T0][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003ec480);
                                        }
                                        continue 'l_3ec480;
                                    } else {
                                        break 'b_3ec4a0;
                                    }
                                }
                            }
                        }
                        'b_3ec4c0: {
                            c.cycles += 5;
                            c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                            c.g[A2][0] = 0u64;
                            c.g[T2][0] = 0u64;
                            let t = c.g[V0][0] != c.g[T6][0];
                            c.g[V1][0] = c.lbu(c.g[S0][0] as u32);
                            if t {
                                break 'b_3ec4c0;
                            } else {
                                c.cycles += 2;
                                if c.g[T0][0] == 0u64 {
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                    break 'b_3ec4c0;
                                } else {
                                    c.cycles += 1;
                                    c.g[S1][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                    break 'b_3ec4c0;
                                }
                            }
                        }
                        'b_3ec4e0: {
                            c.cycles += 5;
                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                            c.g[V1][0] = 0x62;
                            c.g[A0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                            let t = c.g[A0][0] != c.g[V1][0];
                            c.g[V0][0] = 0x77;
                            if t {
                                break 'b_3ec4e0;
                            } else {
                                c.cycles += 3;
                                c.g[S2][0] = c.lbu(c.g[S1][0] as u32);
                                c.g[A2][0] = 0x2;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                break 'b_3ec4e0;
                            }
                        }
                        'b_3ec4f4: {
                            c.cycles += 2;
                            let t = c.g[A0][0] != c.g[V0][0];
                            c.g[V0][0] = 0x6c;
                            if t {
                                break 'b_3ec4f4;
                            } else {
                                c.cycles += 3;
                                c.g[S2][0] = c.lhu(c.g[S1][0] as u32);
                                c.g[A2][0] = 0x4;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
                                break 'b_3ec4f4;
                            }
                        }
                        'b_3ec508: {
                            c.cycles += 2;
                            let t = c.g[A0][0] != c.g[V0][0];
                            c.g[V0][0] = 0x73;
                            if t {
                                break 'b_3ec508;
                            } else {
                                c.cycles += 3;
                                c.g[S2][0] = c.lw(c.g[S1][0] as u32);
                                c.g[A2][0] = 0x8;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                break 'b_3ec508;
                            }
                        }
                        'b_3ec51c: {
                            c.cycles += 2;
                            let t = c.g[A0][0] != c.g[V0][0];
                            c.g[V0][0] = (c.g[S3][0] < c.g[S1][0]) as u64;
                            if t {
                                break 'b_3ec51c;
                            } else {
                                c.cycles += 3;
                                c.g[T2][0] = c.g[S1][0];
                                c.g[S1][0] =
                                    sx((c.g[T2][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                c.g[V0][0] = (c.g[S3][0] < c.g[S1][0]) as u64;
                                break 'b_3ec51c;
                            }
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != 0u64 {
                            c.g[V1][0] = c.lb((c.g[A1][0] as u32).wrapping_sub(0x1));
                            break 'b_3ec660;
                        } else {
                            'b_3ec5bc: {
                                c.cycles += 2;
                                let t = (c.g[A2][0] as i64) <= 0;
                                c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_3ec5bc;
                                } else {
                                    'b_3ec548: {
                                        c.cycles += 4;
                                        c.g[V0][0] = 0xffffffffffff0000;
                                        c.g[V0][0] = (c.g[V0][0] < c.g[S2][0]) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x2d;
                                        if t {
                                            break 'b_3ec548;
                                        } else {
                                            c.cycles += 3;
                                            c.g[S2][0] = sx(0u32.wrapping_sub(c.g[S2][0] as u32));
                                            c.sb(c.g[A1][0] as u32, c.g[V0][0]);
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            break 'b_3ec548;
                                        }
                                    }
                                    'b_3ec57c: {
                                        c.cycles += 4;
                                        c.g[A0][0] =
                                            sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                                        c.g[A3][0] = c.g[A0][0];
                                        let t = c.g[S2][0] == 0u64;
                                        c.sb(c.g[A0][0] as u32, 0u64);
                                        if t {
                                            break 'b_3ec57c;
                                        } else {
                                            c.cycles += 2;
                                            c.g[A2][0] =
                                                sx((c.g[T4][0] as u32).wrapping_add(0x60c0));
                                            'l_3ec560: loop {
                                                c.cycles += 7;
                                                c.g[V1][0] = c.g[S2][0] & 0xf;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[A2][0] as u32));
                                                c.g[S2][0] = sx((c.g[S2][0] as u32) >> 4);
                                                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                                                let t = c.g[S2][0] != 0u64;
                                                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x003ec560);
                                                    }
                                                    continue 'l_3ec560;
                                                } else {
                                                    break 'b_3ec57c;
                                                }
                                            }
                                        }
                                    }
                                    'b_3ec58c: {
                                        c.cycles += 2;
                                        if c.g[A1][0] != c.g[A0][0] {
                                            c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                                            break 'b_3ec58c;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A1][0] = c.g[A3][0];
                                            if t {
                                                break 'b_3ec5bc;
                                            } else {
                                                break 'b_3ec58c;
                                            }
                                        }
                                    }
                                    'b_3ec5b4: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                                        if t {
                                            break 'b_3ec5b4;
                                        } else {
                                            c.cycles += 1;
                                            'l_3ec598: loop {
                                                c.cycles += 7;
                                                c.sb(c.g[A1][0] as u32, c.g[V1][0]);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x003ec598);
                                                    }
                                                    continue 'l_3ec598;
                                                } else {
                                                    break 'b_3ec5b4;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    c.sb(c.g[A1][0] as u32, c.g[T5][0]);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    break 'b_3ec5bc;
                                }
                            }
                        }
                        'b_3ec640: {
                            c.cycles += 2;
                            let t = c.g[T2][0] == 0u64;
                            c.g[S0][0] = c.g[T1][0];
                            if t {
                                break 'b_3ec640;
                            } else {
                                'b_3ec634: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lbu(c.g[T2][0] as u32);
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V1][0] = 0x490000;
                                    if t {
                                        break 'b_3ec634;
                                    } else {
                                        c.cycles += 6;
                                        c.g[V0][0] = 0x490000;
                                        c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x64c0));
                                        c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5fc0));
                                        c.g[A3][0] = 0x25;
                                        c.g[A2][0] = sx((c.g[T4][0] as u32).wrapping_add(0x60c0));
                                        'l_3ec5e8: loop {
                                            'b_3ec628: {
                                                'b_3ec604: {
                                                    c.cycles += 5;
                                                    c.g[S2][0] = c.lbu(c.g[T2][0] as u32);
                                                    c.g[V0][0] = sx((c.g[S2][0] as u32)
                                                        .wrapping_add(c.g[T3][0] as u32));
                                                    c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.g[T2][0] =
                                                        sx((c.g[T2][0] as u32).wrapping_add(0x1));
                                                    if t {
                                                        break 'b_3ec604;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.sb(c.g[A1][0] as u32, c.g[S2][0]);
                                                        if t {
                                                            break 'b_3ec628;
                                                        } else {
                                                            break 'b_3ec604;
                                                        }
                                                    }
                                                }
                                                c.cycles += 9;
                                                c.g[V0][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add(c.g[T0][0] as u32));
                                                c.sb(c.g[A1][0] as u32, c.g[A3][0]);
                                                c.g[A0][0] = c.lbu(c.g[V0][0] as u32);
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                c.g[V0][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add(c.g[A2][0] as u32));
                                                c.sb(c.g[A1][0] as u32, c.g[A0][0]);
                                                c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                c.sb(c.g[A1][0] as u32, c.g[V1][0]);
                                                break 'b_3ec628;
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = c.lbu(c.g[T2][0] as u32);
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x003ec5e8);
                                                }
                                                continue 'l_3ec5e8;
                                            } else {
                                                break 'b_3ec634;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.sb(c.g[A1][0] as u32, c.g[T5][0]);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                c.g[S0][0] = c.g[T1][0];
                                break 'b_3ec640;
                            }
                        }
                        c.cycles += 7;
                        c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = c.g[V0][0] ^ 0x2a;
                        if c.g[V0][0] == 0 {
                            c.g[S0][0] = c.g[V1][0];
                        }
                        c.g[A0][0] = c.lb(c.g[S0][0] as u32);
                        let t = c.g[A0][0] != 0u64;
                        c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x003ec430);
                            }
                            continue 'l_3ec430;
                        } else {
                            break 'b_3ec65c;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lb((c.g[A1][0] as u32).wrapping_sub(0x1));
            break 'b_3ec660;
        }
        'b_3ec698: {
            c.cycles += 3;
            c.g[V0][0] = 0x2c;
            if c.g[V1][0] != c.g[V0][0] {
                c.sb(c.g[A1][0] as u32, 0u64);
                break 'b_3ec698;
            } else {
                c.cycles += 3;
                c.g[V1][0] = 0x2c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                'l_3ec678: loop {
                    c.cycles += 7;
                    c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_sub(0x1));
                    if c.g[V0][0] == c.g[V1][0] {
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        if event_due(c) {
                            deliver_event_here(c, 0x003ec678);
                        }
                        continue 'l_3ec678;
                    } else {
                        c.cycles += 1;
                        c.sb(c.g[A1][0] as u32, 0u64);
                        break 'b_3ec698;
                    }
                }
            }
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S5][0];
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x003eb1e0, 0x003ec6a8)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xa30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xa20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xa10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa00));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa70));
        c.pc = t;
        Ok(())
    })
}

/// `int TagFieldSetEncoded(char* record, int len, const char* name, void* data, int size)`
/// (0x3ec6d0): store `size` bytes from `data` with each byte split into two
/// encode-table characters; a null `data` deletes the field.
pub fn tag_field_set_encoded(c: &mut Cpu) {
    guest(c, |c| {
        'b_3ec71c: {
            'b_3ec710: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1160));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1130));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1150));
                c.g[S2][0] = c.g[T0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x1140));
                c.g[S0][0] = c.g[A3][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1120));
                c.g[S1][0] = c.g[S2][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1110));
                c.g[S3][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x1100), c.g[RA][0]);
                let t = c.g[S0][0] == 0u64;
                c.g[S4][0] = c.g[A1][0];
                if t {
                    break 'b_3ec710;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S3][0];
                    if t {
                        break 'b_3ec71c;
                    } else {
                        break 'b_3ec710;
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = 0x4a0000;
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4a88));
            c.g[A0][0] = c.g[S3][0];
            break 'b_3ec71c;
        }
        'b_3ec7ac: {
            c.cycles += 10;
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003eb198, 0x003ec724)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[T1][0] = 0u64;
            c.g[V0][0] = 0x24;
            c.g[A3][0] = c.g[S0][0];
            c.sb(c.g[A2][0] as u32, c.g[V0][0]);
            c.g[T0][0] = c.lbu(c.g[S2][0] as u32);
            let t = c.g[T0][0] == 0u64;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            if t {
                break 'b_3ec7ac;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x490000;
                c.g[V1][0] = 0x490000;
                c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5fc0));
                c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60c0));
                'l_3ec758: loop {
                    c.cycles += 21;
                    c.g[V0][0] = c.g[T0][0] ^ c.g[T1][0];
                    c.g[A0][0] = c.lbu(c.g[A3][0] as u32);
                    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 5);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                    c.g[A1][0] = c.lbu(c.g[A3][0] as u32);
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.g[A0][0] ^ c.g[V0][0];
                    if c.g[A1][0] == 0 {
                        c.g[A3][0] = c.g[S0][0];
                    }
                    c.g[T1][0] = c.g[A0][0] & 0xff;
                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T3][0] as u32));
                    c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.g[A0][0] = c.lbu(c.g[V0][0] as u32);
                    c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                    c.sb(c.g[A2][0] as u32, c.g[A0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.sb(c.g[A2][0] as u32, c.g[V0][0]);
                    c.g[T0][0] = c.lbu(c.g[S1][0] as u32);
                    let t = c.g[T0][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003ec758);
                        }
                        continue 'l_3ec758;
                    } else {
                        break 'b_3ec7ac;
                    }
                }
            }
        }
        c.cycles += 13;
        c.sb(c.g[A2][0] as u32, 0u64);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x003eb1e0, 0x003ec7c0)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1150));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x1140));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1130));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1120));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1110));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x1100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1160));
        c.pc = t;
        Ok(())
    })
}

/// `int TagFieldSetDate(char* record, int len, const char* name, u32 epoch)`
/// (0x3ec7e0): store a calendar date/time by formatting the epoch seconds.
pub fn tag_field_set_date(c: &mut Cpu) {
    guest(c, |c| {
        'b_3ec824: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1b0));
            c.g[A3][0] = c.g[A3][0] << 32;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x160));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x180));
            c.g[S4][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x170));
            c.g[S2][0] = c.g[A2][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1a0));
            c.g[S3][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x190));
            c.g[A1][0] = c.g[A3][0] >> 32;
            let t = c.g[A1][0] != 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x150), c.g[RA][0]);
            if t {
                break 'b_3ec824;
            } else {
                c.cycles += 4;
                call_ptr(c, 0x003ef088, 0x003ec81c)?;
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[A1][0] = c.g[V0][0] >> 32;
                break 'b_3ec824;
            }
        }
        'b_3ec898: {
            'b_3ec890: {
                c.cycles += 5;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x003eaeb8, 0x003ec82c)?;
                c.g[S1][0] = c.g[V0][0];
                let t = c.g[S1][0] == 0u64;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_3ec890;
                } else {
                    c.cycles += 22;
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x003eb198, 0x003ec848)?;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = 0x490000;
                    c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[T3][0] = c.lw(c.g[S1][0] as u32);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6a40));
                    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x76c));
                    c.g[T1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    c.g[T2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x00417828, 0x003ec878)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, 0x003eb1e0, 0x003ec888)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                    if t {
                        break 'b_3ec898;
                    } else {
                        break 'b_3ec890;
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0xffffffffffffffff;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1a0));
            break 'b_3ec898;
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x150));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1b0));
        c.pc = t;
        Ok(())
    })
}

/// `int TagFieldSetDateYMDHMS(char* record, int len, const char* name, int y, int mo, int d, int h, int mi, int s, int adj)`
/// (0x3ec8b8): build a calendar time and store it as an epoch date.
pub fn tag_field_set_date_ymdhms(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 58;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S6][0] = c.g[A2][0];
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S7][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[FP][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[S0][0] = c.g[A3][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S1][0] = c.g[T0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[A0][0] = c.g[SP][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S3][0] = c.g[T1][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S4][0] = c.g[T2][0];
        c.g[S5][0] = c.g[T3][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
        c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd8));
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x24;
        call_ptr(c, 0x00416210, 0x003ec918)?;
        c.g[V0][0] = 0xffffffffffffffff;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x76c));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[A0][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[S0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[S3][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[S4][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[S5][0]);
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        call_ptr(c, 0x003eb090, 0x003ec94c)?;
        c.g[V0][0] = c.g[V0][0] << 32;
        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
        c.g[A3][0] = 0u64;
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S2][0] as u32));
        c.g[A0][0] = c.g[S7][0];
        c.g[A1][0] = c.g[FP][0];
        if c.g[V0][0] != 0 {
            c.g[A3][0] = c.g[S2][0];
        }
        c.g[A2][0] = c.g[S6][0];
        call_ptr(c, 0x003ec7e0, 0x003ec970)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.pc = t;
        Ok(())
    })
}

/// `int TagFieldSetDatePacked(char* record, int len, const char* name, int date, int time, int adj)`
/// (0x3ec9a0): unpack `date`/`time` bit fields into a calendar time and store it.
pub fn tag_field_set_date_packed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 57;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S3][0] = c.g[A0][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S4][0] = c.g[A1][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S5][0] = c.g[A2][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S0][0] = c.g[A3][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S1][0] = c.g[T0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
        c.g[S2][0] = c.g[T1][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x24;
        call_ptr(c, 0x00416210, 0x003ec9e8)?;
        c.g[V1][0] = sx(((c.g[S0][0] as u32) as i32 >> 8) as u32);
        c.g[A1][0] = sx((c.g[S0][0] as u32) >> 16);
        c.g[V1][0] = c.g[V1][0] & 0xff;
        c.g[A2][0] = sx(((c.g[S1][0] as u32) as i32 >> 16) as u32);
        c.g[A3][0] = sx(((c.g[S1][0] as u32) as i32 >> 8) as u32);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x76c));
        c.g[A2][0] = c.g[A2][0] & 0xff;
        c.g[A3][0] = c.g[A3][0] & 0xff;
        c.g[S0][0] = c.g[S0][0] & 0xff;
        c.g[S1][0] = c.g[S1][0] & 0xff;
        c.g[V0][0] = 0xffffffffffffffff;
        c.g[A0][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[S0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A2][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A3][0]);
        c.sw(c.g[SP][0] as u32, c.g[S1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        call_ptr(c, 0x003eb090, 0x003eca3c)?;
        c.g[V0][0] = c.g[V0][0] << 32;
        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
        c.g[A3][0] = 0u64;
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S2][0] as u32));
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S4][0];
        if c.g[V0][0] != 0 {
            c.g[A3][0] = c.g[S2][0];
        }
        c.g[A2][0] = c.g[S5][0];
        call_ptr(c, 0x003ec7e0, 0x003eca60)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        Ok(())
    })
}

/// `int TagFieldCopyField(char* record, int len, const char* name, char* data)`
/// (0x3eca88): find `name` in `data` and copy the whole field into the record.
pub fn tag_field_copy_field(c: &mut Cpu) {
    guest(c, |c| {
        'b_3ecae4: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A3][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.g[S2][0] = c.g[A1][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A1][0] = c.g[A2][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003eb6d8, 0x003ecab4)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[V0][0] = 0u64;
            if t {
                break 'b_3ecae4;
            } else {
                'l_3ecac0: loop {
                    'b_3ecadc: {
                        c.cycles += 2;
                        let t = c.g[A2][0] == c.g[S0][0];
                        c.g[A0][0] = c.g[S1][0];
                        if t {
                            break 'b_3ecadc;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lbu((c.g[A2][0] as u32).wrapping_sub(0x1));
                            c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                if event_due(c) {
                                    deliver_event_here(c, 0x003ecac0);
                                }
                                continue 'l_3ecac0;
                            } else {
                                c.cycles += 1;
                                c.g[A0][0] = c.g[S1][0];
                                break 'b_3ecadc;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x003eb1e0, 0x003ecae4)?;
                    break 'b_3ecae4;
                }
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `int TagFieldPrintf(char* buf, int len, const char* fmt, ...)` (0x3ecbd8):
/// a printf-style formatter that turns whitespace runs into the field
/// separator and `%s/%d/%a/%f/%e/%r` into tag fields from the arguments.
pub fn tag_field_printf(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003ecbd8 => {
                c.cycles += 25;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S1][0] = c.g[A1][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb8));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S3][0] = c.g[A2][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S5][0] = c.g[S1][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[A1][0] = c.lbu(c.g[S3][0] as u32);
                c.sd((c.g[SP][0] as u32).wrapping_add(0xb8), c.g[A3][0]);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[T0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0xc8), c.g[T1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[T2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0xd8), c.g[T3][0]);
                c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0xa8));
                c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0xac));
                c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                let t = c.g[A1][0] == 0u64;
                c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0xb4));
                if t {
                    pc = 0x003ece08;
                    continue;
                }
                pc = 0x003ecc3c;
                continue;
            }
            0x003ecc3c => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S4][0] = 0x4a0000;
                if t {
                    pc = 0x003ece08;
                    continue;
                }
                pc = 0x003ecc48;
                continue;
            }
            0x003ecc48 => {
                c.cycles += 2;
                c.g[V0][0] = (c.g[A1][0] < 0x21u64) as u64;
                pc = 0x003ecc50;
                continue;
            }
            0x003ecc50 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1980));
                if t {
                    pc = 0x003ecc9c;
                    continue;
                }
                pc = 0x003ecc58;
                continue;
            }
            0x003ecc58 => {
                c.cycles += 5;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[V0][0] = c.lb(c.g[S3][0] as u32);
                let t = c.g[V0][0] == 0u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if t {
                    pc = 0x003ecdf0;
                    continue;
                }
                pc = 0x003ecc6c;
                continue;
            }
            0x003ecc6c => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 33i64) as u64;
                if c.g[V0][0] == 0u64 {
                    c.g[A1][0] = c.lbu(c.g[S3][0] as u32);
                    pc = 0x003ecdf4;
                    continue;
                }
                pc = 0x003ecc78;
                continue;
            }
            0x003ecc78 => {
                c.cycles += 2;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                pc = 0x003ecc80;
                continue;
            }
            0x003ecc80 => {
                c.cycles += 3;
                c.g[V0][0] = c.lb(c.g[S3][0] as u32);
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 33i64) as u64;
                if t {
                    pc = 0x003ecdf0;
                    continue;
                }
                pc = 0x003ecc8c;
                continue;
            }
            0x003ecc8c => {
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003ecc80) {
                        return;
                    }
                    pc = 0x003ecc80;
                    continue;
                }
                pc = 0x003ecc94;
                continue;
            }
            0x003ecc94 => {
                c.cycles += 2;
                let t = true;
                c.g[A1][0] = c.lbu(c.g[S3][0] as u32);
                if t {
                    pc = 0x003ecdf4;
                    continue;
                }
                pc = 0x003ecc9c;
                continue;
            }
            0x003ecc9c => {
                c.cycles += 3;
                c.g[V0][0] = 0x25;
                if c.g[A1][0] == c.g[V0][0] {
                    c.g[V0][0] = c.lb(c.g[S3][0] as u32);
                    pc = 0x003eccb8;
                    continue;
                }
                pc = 0x003ecca8;
                continue;
            }
            0x003ecca8 => {
                c.cycles += 4;
                c.sb(c.g[S0][0] as u32, c.g[A1][0]);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                let t = true;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if t {
                    pc = 0x003ecdf0;
                    continue;
                }
                pc = 0x003eccb8;
                continue;
            }
            0x003eccb8 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[A2][0] = 0u64;
                if t {
                    pc = 0x003ecdf0;
                    continue;
                }
                pc = 0x003eccc0;
                continue;
            }
            0x003eccc0 => {
                c.cycles += 5;
                c.g[A1][0] = c.lbu(c.g[S3][0] as u32);
                c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = 0x3d;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                if t {
                    pc = 0x003eccdc;
                    continue;
                }
                pc = 0x003eccd4;
                continue;
            }
            0x003eccd4 => {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                pc = 0x003eccdc;
                continue;
            }
            0x003eccdc => {
                c.cycles += 3;
                c.g[V0][0] = 0x73;
                let t = c.g[A1][0] != c.g[V0][0];
                c.sb(c.g[S0][0] as u32, 0u64);
                if t {
                    pc = 0x003ecd08;
                    continue;
                }
                pc = 0x003ecce8;
                continue;
            }
            0x003ecce8 => {
                c.cycles += 8;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4a90));
                c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x8));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                if call_ptr(c, 0x003ec180, 0x003ecd00).is_err() {
                    return;
                }
                let t = true;
                c.g[A2][0] = c.g[V0][0];
                if t {
                    pc = 0x003ecde0;
                    continue;
                }
                pc = 0x003ecd08;
                continue;
            }
            0x003ecd08 => {
                c.cycles += 3;
                c.g[V0][0] = 0x64;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x61;
                if t {
                    pc = 0x003ecd34;
                    continue;
                }
                pc = 0x003ecd14;
                continue;
            }
            0x003ecd14 => {
                c.cycles += 8;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4a90));
                c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x8));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                if call_ptr(c, 0x003ebe40, 0x003ecd2c).is_err() {
                    return;
                }
                let t = true;
                c.g[A2][0] = c.g[V0][0];
                if t {
                    pc = 0x003ecde0;
                    continue;
                }
                pc = 0x003ecd34;
                continue;
            }
            0x003ecd34 => {
                c.cycles += 2;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x66;
                if t {
                    pc = 0x003ecd5c;
                    continue;
                }
                pc = 0x003ecd3c;
                continue;
            }
            0x003ecd3c => {
                c.cycles += 8;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4a90));
                c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x8));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                if call_ptr(c, 0x003ebfc8, 0x003ecd54).is_err() {
                    return;
                }
                let t = true;
                c.g[A2][0] = c.g[V0][0];
                if t {
                    pc = 0x003ecde0;
                    continue;
                }
                pc = 0x003ecd5c;
                continue;
            }
            0x003ecd5c => {
                c.cycles += 2;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x65;
                if t {
                    pc = 0x003ecd84;
                    continue;
                }
                pc = 0x003ecd64;
                continue;
            }
            0x003ecd64 => {
                c.cycles += 8;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4a90));
                c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x8));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                if call_ptr(c, 0x003ebf28, 0x003ecd7c).is_err() {
                    return;
                }
                let t = true;
                c.g[A2][0] = c.g[V0][0];
                if t {
                    pc = 0x003ecde0;
                    continue;
                }
                pc = 0x003ecd84;
                continue;
            }
            0x003ecd84 => {
                c.cycles += 2;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x72;
                if t {
                    pc = 0x003ecdac;
                    continue;
                }
                pc = 0x003ecd8c;
                continue;
            }
            0x003ecd8c => {
                c.cycles += 8;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4a90));
                c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x8));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                if call_ptr(c, 0x003ec7e0, 0x003ecda4).is_err() {
                    return;
                }
                let t = true;
                c.g[A2][0] = c.g[V0][0];
                if t {
                    pc = 0x003ecde0;
                    continue;
                }
                pc = 0x003ecdac;
                continue;
            }
            0x003ecdac => {
                c.cycles += 2;
                let t = c.g[A1][0] != c.g[V0][0];
                if t {
                    pc = 0x003ecde0;
                    continue;
                }
                pc = 0x003ecdb4;
                continue;
            }
            0x003ecdb4 => {
                c.cycles += 3;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                let t = true;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x8));
                if t {
                    pc = 0x003ecdcc;
                    continue;
                }
                pc = 0x003ecdc0;
                continue;
            }
            0x003ecdc0 => {
                c.cycles += 3;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                pc = 0x003ecdcc;
                continue;
            }
            0x003ecdcc => {
                c.cycles += 3;
                c.g[V0][0] = c.lb(c.g[V1][0] as u32);
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.lbu(c.g[V1][0] as u32);
                if t {
                    pc = 0x003ecde0;
                    continue;
                }
                pc = 0x003ecdd8;
                continue;
            }
            0x003ecdd8 => {
                c.cycles += 2;
                if (c.g[S1][0] as i64) > 0 {
                    c.sb(c.g[S0][0] as u32, c.g[A0][0]);
                    if back_edge_unwinds(c, 0x003ecdc0) {
                        return;
                    }
                    pc = 0x003ecdc0;
                    continue;
                }
                pc = 0x003ecde0;
                continue;
            }
            0x003ecde0 => {
                c.cycles += 2;
                if (c.g[A2][0] as i64) <= 0 {
                    c.g[A1][0] = c.lbu(c.g[S3][0] as u32);
                    pc = 0x003ecdf4;
                    continue;
                }
                pc = 0x003ecde8;
                continue;
            }
            0x003ecde8 => {
                c.cycles += 2;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                pc = 0x003ecdf0;
                continue;
            }
            0x003ecdf0 => {
                c.cycles += 1;
                c.g[A1][0] = c.lbu(c.g[S3][0] as u32);
                pc = 0x003ecdf4;
                continue;
            }
            0x003ecdf4 => {
                c.cycles += 2;
                let t = c.g[A1][0] == 0u64;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                if t {
                    pc = 0x003ece08;
                    continue;
                }
                pc = 0x003ecdfc;
                continue;
            }
            0x003ecdfc => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = (c.g[A1][0] < 0x21u64) as u64;
                if t {
                    if back_edge_unwinds(c, 0x003ecc50) {
                        return;
                    }
                    pc = 0x003ecc50;
                    continue;
                }
                pc = 0x003ece08;
                continue;
            }
            0x003ece08 => {
                c.cycles += 2;
                if (c.g[S1][0] as i64) > 0 {
                    c.sb(c.g[S0][0] as u32, 0u64);
                    pc = 0x003ece10;
                    continue;
                }
                pc = 0x003ece10;
                continue;
            }
            0x003ece10 => {
                c.cycles += 10;
                c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003ecbd8, 0x003ece38);
                return;
            }
        }
    }
}

/// `int TagFieldGetDelim(const char* data, char* buf, int len, const char* dflt, int index, int delim)`
/// (0x3ed0f8): GetString with an index/delim skip before the value.
pub fn tag_field_get_delim(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003ed0f8 => {
                c.cycles += 4;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[T2][0] = c.g[A1][0];
                let t = c.g[A0][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    pc = 0x003ed184;
                    continue;
                }
                pc = 0x003ed108;
                continue;
            }
            0x003ed108 => {
                c.cycles += 4;
                c.g[V1][0] = c.lb(c.g[A0][0] as u32);
                c.g[V0][0] = 0x22;
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x27;
                if t {
                    pc = 0x003ed120;
                    continue;
                }
                pc = 0x003ed118;
                continue;
            }
            0x003ed118 => {
                c.cycles += 2;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[T3][0] = 0u64;
                if t {
                    pc = 0x003ed134;
                    continue;
                }
                pc = 0x003ed120;
                continue;
            }
            0x003ed120 => {
                c.cycles += 3;
                c.g[T3][0] = c.lbu(c.g[A0][0] as u32);
                let t = true;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                if t {
                    pc = 0x003ed134;
                    continue;
                }
                pc = 0x003ed12c;
                continue;
            }
            0x003ed12c => {
                c.cycles += 2;
                let t = true;
                c.g[A0][0] = 0u64;
                if t {
                    pc = 0x003ed17c;
                    continue;
                }
                pc = 0x003ed134;
                continue;
            }
            0x003ed134 => {
                c.cycles += 2;
                let t = (c.g[T0][0] as i64) <= 0;
                if t {
                    pc = 0x003ed17c;
                    continue;
                }
                pc = 0x003ed13c;
                continue;
            }
            0x003ed13c => {
                c.cycles += 2;
                let t = true;
                c.g[A1][0] = c.lb(c.g[A0][0] as u32);
                if t {
                    pc = 0x003ed14c;
                    continue;
                }
                pc = 0x003ed144;
                continue;
            }
            0x003ed144 => {
                c.cycles += 1;
                pc = 0x003ed148;
                continue;
            }
            0x003ed148 => {
                c.cycles += 1;
                c.g[A1][0] = c.lb(c.g[A0][0] as u32);
                pc = 0x003ed14c;
                continue;
            }
            0x003ed14c => {
                c.cycles += 2;
                let t = c.g[A1][0] == c.g[T3][0];
                c.g[V0][0] = ((c.g[A1][0] as i64) < 32i64) as u64;
                if t {
                    pc = 0x003ed164;
                    continue;
                }
                pc = 0x003ed154;
                continue;
            }
            0x003ed154 => {
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                    pc = 0x003ed168;
                    continue;
                }
                pc = 0x003ed15c;
                continue;
            }
            0x003ed15c => {
                c.cycles += 2;
                if c.g[A1][0] != c.g[T1][0] {
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003ed148) {
                        return;
                    }
                    pc = 0x003ed148;
                    continue;
                }
                pc = 0x003ed164;
                continue;
            }
            0x003ed164 => {
                c.cycles += 1;
                c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                pc = 0x003ed168;
                continue;
            }
            0x003ed168 => {
                c.cycles += 2;
                let t = c.g[V0][0] != c.g[T1][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                if t {
                    if back_edge_unwinds(c, 0x003ed12c) {
                        return;
                    }
                    pc = 0x003ed12c;
                    continue;
                }
                pc = 0x003ed170;
                continue;
            }
            0x003ed170 => {
                c.cycles += 3;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                if (c.g[T0][0] as i64) > 0 {
                    c.g[A1][0] = c.lb(c.g[A0][0] as u32);
                    if back_edge_unwinds(c, 0x003ed14c) {
                        return;
                    }
                    pc = 0x003ed14c;
                    continue;
                }
                pc = 0x003ed17c;
                continue;
            }
            0x003ed17c => {
                c.cycles += 2;
                let t = c.g[A0][0] != 0u64;
                if t {
                    pc = 0x003ed1d8;
                    continue;
                }
                pc = 0x003ed184;
                continue;
            }
            0x003ed184 => {
                c.cycles += 2;
                let t = c.g[A3][0] == 0u64;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    pc = 0x003ed36c;
                    continue;
                }
                pc = 0x003ed18c;
                continue;
            }
            0x003ed18c => {
                c.cycles += 2;
                let t = c.g[T2][0] != 0u64;
                c.g[T0][0] = 0x1;
                if t {
                    pc = 0x003ed1b8;
                    continue;
                }
                pc = 0x003ed194;
                continue;
            }
            0x003ed194 => {
                c.cycles += 4;
                c.g[A0][0] = c.g[A3][0];
                if call_ptr(c, 0x00416810, 0x003ed19c).is_err() {
                    return;
                }
                let t = true;
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                if t {
                    pc = 0x003ed370;
                    continue;
                }
                pc = 0x003ed1a4;
                continue;
            }
            0x003ed1a4 => {
                c.cycles += 1;
                pc = 0x003ed1a8;
                continue;
            }
            0x003ed1a8 => {
                c.cycles += 4;
                c.sb(c.g[T2][0] as u32, c.g[V1][0]);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                pc = 0x003ed1b8;
                continue;
            }
            0x003ed1b8 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                if c.g[V0][0] == 0u64 {
                    c.sb(c.g[T2][0] as u32, 0u64);
                    pc = 0x003ed368;
                    continue;
                }
                pc = 0x003ed1c4;
                continue;
            }
            0x003ed1c4 => {
                c.cycles += 3;
                c.g[V0][0] = c.lb(c.g[A3][0] as u32);
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
                if t {
                    if back_edge_unwinds(c, 0x003ed1a8) {
                        return;
                    }
                    pc = 0x003ed1a8;
                    continue;
                }
                pc = 0x003ed1d0;
                continue;
            }
            0x003ed1d0 => {
                c.cycles += 2;
                let t = true;
                c.sb(c.g[T2][0] as u32, 0u64);
                if t {
                    pc = 0x003ed368;
                    continue;
                }
                pc = 0x003ed1d8;
                continue;
            }
            0x003ed1d8 => {
                c.cycles += 2;
                let t = c.g[T2][0] != 0u64;
                c.g[A3][0] = c.g[A0][0];
                if t {
                    pc = 0x003ed268;
                    continue;
                }
                pc = 0x003ed1e0;
                continue;
            }
            0x003ed1e0 => {
                c.cycles += 4;
                c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
                c.g[V0][0] = c.g[V1][0] & 0xff;
                let t = c.g[V0][0] == c.g[T3][0];
                c.g[T0][0] = 0u64;
                if t {
                    pc = 0x003ed260;
                    continue;
                }
                pc = 0x003ed1f0;
                continue;
            }
            0x003ed1f0 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.g[T0][0];
                if t {
                    pc = 0x003ed36c;
                    continue;
                }
                pc = 0x003ed1fc;
                continue;
            }
            0x003ed1fc => {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] & 0xff;
                let t = c.g[V0][0] == c.g[T1][0];
                c.g[V0][0] = c.g[T0][0];
                if t {
                    pc = 0x003ed36c;
                    continue;
                }
                pc = 0x003ed208;
                continue;
            }
            0x003ed208 => {
                c.cycles += 2;
                c.g[A0][0] = 0x25;
                pc = 0x003ed210;
                continue;
            }
            0x003ed210 => {
                c.cycles += 2;
                if c.g[V1][0] != c.g[A0][0] {
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    pc = 0x003ed23c;
                    continue;
                }
                pc = 0x003ed218;
                continue;
            }
            0x003ed218 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    pc = 0x003ed23c;
                    continue;
                }
                pc = 0x003ed228;
                continue;
            }
            0x003ed228 => {
                c.cycles += 5;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x2));
                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A3][0] = c.g[V1][0];
                }
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                pc = 0x003ed23c;
                continue;
            }
            0x003ed23c => {
                c.cycles += 4;
                c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
                c.g[V0][0] = c.g[V1][0] & 0xff;
                let t = c.g[V0][0] == c.g[T3][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                if t {
                    pc = 0x003ed260;
                    continue;
                }
                pc = 0x003ed24c;
                continue;
            }
            0x003ed24c => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.g[V1][0] & 0xff;
                if t {
                    pc = 0x003ed260;
                    continue;
                }
                pc = 0x003ed258;
                continue;
            }
            0x003ed258 => {
                c.cycles += 2;
                let t = c.g[V0][0] != c.g[T1][0];
                if t {
                    if back_edge_unwinds(c, 0x003ed210) {
                        return;
                    }
                    pc = 0x003ed210;
                    continue;
                }
                pc = 0x003ed260;
                continue;
            }
            0x003ed260 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = c.g[T0][0];
                if t {
                    pc = 0x003ed36c;
                    continue;
                }
                pc = 0x003ed268;
                continue;
            }
            0x003ed268 => {
                c.cycles += 2;
                let t = (c.g[A2][0] as i64) > 0;
                c.g[T0][0] = 0x1;
                if t {
                    pc = 0x003ed278;
                    continue;
                }
                pc = 0x003ed270;
                continue;
            }
            0x003ed270 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    pc = 0x003ed36c;
                    continue;
                }
                pc = 0x003ed278;
                continue;
            }
            0x003ed278 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A3][0] = c.g[A0][0];
                if t {
                    pc = 0x003ed364;
                    continue;
                }
                pc = 0x003ed284;
                continue;
            }
            0x003ed284 => {
                c.cycles += 4;
                c.g[A0][0] = c.lbu(c.g[A3][0] as u32);
                c.g[V0][0] = c.g[A0][0] & 0xff;
                let t = c.g[V0][0] == c.g[T3][0];
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if t {
                    pc = 0x003ed364;
                    continue;
                }
                pc = 0x003ed294;
                continue;
            }
            0x003ed294 => {
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    c.sb(c.g[T2][0] as u32, 0u64);
                    pc = 0x003ed368;
                    continue;
                }
                pc = 0x003ed29c;
                continue;
            }
            0x003ed29c => {
                c.cycles += 3;
                c.g[V0][0] = c.g[A0][0] & 0xff;
                let t = c.g[V0][0] == c.g[T1][0];
                c.g[V0][0] = 0x490000;
                if t {
                    pc = 0x003ed364;
                    continue;
                }
                pc = 0x003ed2a8;
                continue;
            }
            0x003ed2a8 => {
                c.cycles += 6;
                c.g[V1][0] = 0x490000;
                c.g[T6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x61c0));
                c.g[T5][0] = sx((c.g[V1][0] as u32).wrapping_add(0x62c0));
                c.g[T4][0] = 0x25;
                c.g[V1][0] = c.g[A0][0] & 0xff;
                pc = 0x003ed2c0;
                continue;
            }
            0x003ed2c0 => {
                c.cycles += 2;
                if c.g[V1][0] != c.g[T4][0] {
                    c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                    pc = 0x003ed328;
                    continue;
                }
                pc = 0x003ed2c8;
                continue;
            }
            0x003ed2c8 => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1));
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                    pc = 0x003ed2e8;
                    continue;
                }
                pc = 0x003ed2d4;
                continue;
            }
            0x003ed2d4 => {
                c.cycles += 5;
                c.sb(c.g[T2][0] as u32, c.g[V0][0]);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2));
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                let t = true;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                if t {
                    pc = 0x003ed338;
                    continue;
                }
                pc = 0x003ed2e8;
                continue;
            }
            0x003ed2e8 => {
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                    pc = 0x003ed328;
                    continue;
                }
                pc = 0x003ed2f0;
                continue;
            }
            0x003ed2f0 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                    pc = 0x003ed328;
                    continue;
                }
                pc = 0x003ed300;
                continue;
            }
            0x003ed300 => {
                c.cycles += 10;
                c.g[A1][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x2));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T6][0] as u32));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T5][0] as u32));
                c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                let t = true;
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                if t {
                    pc = 0x003ed330;
                    continue;
                }
                pc = 0x003ed328;
                continue;
            }
            0x003ed328 => {
                c.cycles += 2;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                pc = 0x003ed330;
                continue;
            }
            0x003ed330 => {
                c.cycles += 2;
                c.sb(c.g[T2][0] as u32, c.g[V0][0]);
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                pc = 0x003ed338;
                continue;
            }
            0x003ed338 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                if c.g[V0][0] == 0u64 {
                    c.sb(c.g[T2][0] as u32, 0u64);
                    pc = 0x003ed368;
                    continue;
                }
                pc = 0x003ed344;
                continue;
            }
            0x003ed344 => {
                c.cycles += 4;
                c.g[A0][0] = c.lbu(c.g[A3][0] as u32);
                c.g[V0][0] = c.g[A0][0] & 0xff;
                let t = c.g[V0][0] == c.g[T3][0];
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if t {
                    pc = 0x003ed364;
                    continue;
                }
                pc = 0x003ed354;
                continue;
            }
            0x003ed354 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.g[A0][0] & 0xff;
                if t {
                    pc = 0x003ed364;
                    continue;
                }
                pc = 0x003ed35c;
                continue;
            }
            0x003ed35c => {
                c.cycles += 2;
                let t = c.g[V0][0] != c.g[T1][0];
                c.g[V1][0] = c.g[A0][0] & 0xff;
                if t {
                    if back_edge_unwinds(c, 0x003ed2c0) {
                        return;
                    }
                    pc = 0x003ed2c0;
                    continue;
                }
                pc = 0x003ed364;
                continue;
            }
            0x003ed364 => {
                c.cycles += 1;
                c.sb(c.g[T2][0] as u32, 0u64);
                pc = 0x003ed368;
                continue;
            }
            0x003ed368 => {
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                pc = 0x003ed36c;
                continue;
            }
            0x003ed36c => {
                c.cycles += 1;
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                pc = 0x003ed370;
                continue;
            }
            0x003ed370 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003ed0f8, 0x003ed378);
                return;
            }
        }
    }
}

/// `int TagFieldGetString(const char* data, char* buf, int len, const char* dflt)`
/// (0x3ed378): copy a tag value (quoted or bare, with `%xx` decoding) into
/// `buf`, or the default; with a null buffer just return the size.
pub fn tag_field_get_string(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003ed378 => {
                c.cycles += 6;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[V1][0] = c.g[A0][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[T0][0] = c.g[A1][0];
                let t = c.g[V1][0] != 0u64;
                c.g[A0][0] = c.g[A3][0];
                if t {
                    pc = 0x003ed3e0;
                    continue;
                }
                pc = 0x003ed390;
                continue;
            }
            0x003ed390 => {
                c.cycles += 2;
                let t = c.g[A0][0] == 0u64;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    pc = 0x003ed56c;
                    continue;
                }
                pc = 0x003ed398;
                continue;
            }
            0x003ed398 => {
                c.cycles += 2;
                let t = c.g[T0][0] != 0u64;
                c.g[T1][0] = 0x1;
                if t {
                    pc = 0x003ed3c0;
                    continue;
                }
                pc = 0x003ed3a0;
                continue;
            }
            0x003ed3a0 => {
                c.cycles += 4;
                if call_ptr(c, 0x00416810, 0x003ed3a8).is_err() {
                    return;
                }
                let t = true;
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                if t {
                    pc = 0x003ed570;
                    continue;
                }
                pc = 0x003ed3b0;
                continue;
            }
            0x003ed3b0 => {
                c.cycles += 4;
                c.sb(c.g[T0][0] as u32, c.g[V1][0]);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                pc = 0x003ed3c0;
                continue;
            }
            0x003ed3c0 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                if c.g[V0][0] == 0u64 {
                    c.sb(c.g[T0][0] as u32, 0u64);
                    pc = 0x003ed568;
                    continue;
                }
                pc = 0x003ed3cc;
                continue;
            }
            0x003ed3cc => {
                c.cycles += 3;
                c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                if t {
                    if back_edge_unwinds(c, 0x003ed3b0) {
                        return;
                    }
                    pc = 0x003ed3b0;
                    continue;
                }
                pc = 0x003ed3d8;
                continue;
            }
            0x003ed3d8 => {
                c.cycles += 2;
                let t = true;
                c.sb(c.g[T0][0] as u32, 0u64);
                if t {
                    pc = 0x003ed568;
                    continue;
                }
                pc = 0x003ed3e0;
                continue;
            }
            0x003ed3e0 => {
                c.cycles += 4;
                c.g[A0][0] = c.lb(c.g[V1][0] as u32);
                c.g[V0][0] = 0x22;
                let t = c.g[A0][0] == c.g[V0][0];
                c.g[V0][0] = 0x27;
                if t {
                    pc = 0x003ed3f8;
                    continue;
                }
                pc = 0x003ed3f0;
                continue;
            }
            0x003ed3f0 => {
                c.cycles += 2;
                let t = c.g[A0][0] != c.g[V0][0];
                c.g[T2][0] = 0u64;
                if t {
                    pc = 0x003ed400;
                    continue;
                }
                pc = 0x003ed3f8;
                continue;
            }
            0x003ed3f8 => {
                c.cycles += 2;
                c.g[T2][0] = c.lbu(c.g[V1][0] as u32);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                pc = 0x003ed400;
                continue;
            }
            0x003ed400 => {
                c.cycles += 2;
                let t = c.g[T0][0] != 0u64;
                c.g[A3][0] = c.g[V1][0];
                if t {
                    pc = 0x003ed478;
                    continue;
                }
                pc = 0x003ed408;
                continue;
            }
            0x003ed408 => {
                c.cycles += 4;
                c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
                c.g[V0][0] = c.g[V1][0] & 0xff;
                let t = c.g[V0][0] == c.g[T2][0];
                c.g[T1][0] = 0u64;
                if t {
                    pc = 0x003ed470;
                    continue;
                }
                pc = 0x003ed418;
                continue;
            }
            0x003ed418 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.g[T1][0];
                if t {
                    pc = 0x003ed56c;
                    continue;
                }
                pc = 0x003ed424;
                continue;
            }
            0x003ed424 => {
                c.cycles += 1;
                c.g[A0][0] = 0x25;
                pc = 0x003ed428;
                continue;
            }
            0x003ed428 => {
                c.cycles += 2;
                if c.g[V1][0] != c.g[A0][0] {
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    pc = 0x003ed454;
                    continue;
                }
                pc = 0x003ed430;
                continue;
            }
            0x003ed430 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    pc = 0x003ed454;
                    continue;
                }
                pc = 0x003ed440;
                continue;
            }
            0x003ed440 => {
                c.cycles += 5;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x2));
                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A3][0] = c.g[V1][0];
                }
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                pc = 0x003ed454;
                continue;
            }
            0x003ed454 => {
                c.cycles += 4;
                c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
                c.g[V0][0] = c.g[V1][0] & 0xff;
                let t = c.g[V0][0] == c.g[T2][0];
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                if t {
                    pc = 0x003ed470;
                    continue;
                }
                pc = 0x003ed464;
                continue;
            }
            0x003ed464 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                let t = c.g[V0][0] == 0u64;
                if t {
                    if back_edge_unwinds(c, 0x003ed428) {
                        return;
                    }
                    pc = 0x003ed428;
                    continue;
                }
                pc = 0x003ed470;
                continue;
            }
            0x003ed470 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = c.g[T1][0];
                if t {
                    pc = 0x003ed56c;
                    continue;
                }
                pc = 0x003ed478;
                continue;
            }
            0x003ed478 => {
                c.cycles += 2;
                let t = (c.g[A2][0] as i64) > 0;
                c.g[T1][0] = 0x1;
                if t {
                    pc = 0x003ed488;
                    continue;
                }
                pc = 0x003ed480;
                continue;
            }
            0x003ed480 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    pc = 0x003ed56c;
                    continue;
                }
                pc = 0x003ed488;
                continue;
            }
            0x003ed488 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A3][0] = c.g[V1][0];
                if t {
                    pc = 0x003ed564;
                    continue;
                }
                pc = 0x003ed494;
                continue;
            }
            0x003ed494 => {
                c.cycles += 4;
                c.g[A0][0] = c.lbu(c.g[A3][0] as u32);
                c.g[V0][0] = c.g[A0][0] & 0xff;
                let t = c.g[V0][0] == c.g[T2][0];
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if t {
                    pc = 0x003ed564;
                    continue;
                }
                pc = 0x003ed4a4;
                continue;
            }
            0x003ed4a4 => {
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    c.sb(c.g[T0][0] as u32, 0u64);
                    pc = 0x003ed568;
                    continue;
                }
                pc = 0x003ed4ac;
                continue;
            }
            0x003ed4ac => {
                c.cycles += 7;
                c.g[V0][0] = 0x490000;
                c.g[V1][0] = 0x490000;
                c.g[T5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x61c0));
                c.g[T4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x62c0));
                c.g[T3][0] = 0x25;
                c.g[V1][0] = c.g[A0][0] & 0xff;
                pc = 0x003ed4c8;
                continue;
            }
            0x003ed4c8 => {
                c.cycles += 2;
                if c.g[V1][0] != c.g[T3][0] {
                    c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                    pc = 0x003ed530;
                    continue;
                }
                pc = 0x003ed4d0;
                continue;
            }
            0x003ed4d0 => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1));
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                    pc = 0x003ed4f0;
                    continue;
                }
                pc = 0x003ed4dc;
                continue;
            }
            0x003ed4dc => {
                c.cycles += 5;
                c.sb(c.g[T0][0] as u32, c.g[V0][0]);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2));
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                let t = true;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                if t {
                    pc = 0x003ed540;
                    continue;
                }
                pc = 0x003ed4f0;
                continue;
            }
            0x003ed4f0 => {
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                    pc = 0x003ed530;
                    continue;
                }
                pc = 0x003ed4f8;
                continue;
            }
            0x003ed4f8 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                    pc = 0x003ed530;
                    continue;
                }
                pc = 0x003ed508;
                continue;
            }
            0x003ed508 => {
                c.cycles += 10;
                c.g[A1][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x2));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T5][0] as u32));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T4][0] as u32));
                c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                let t = true;
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                if t {
                    pc = 0x003ed538;
                    continue;
                }
                pc = 0x003ed530;
                continue;
            }
            0x003ed530 => {
                c.cycles += 2;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                pc = 0x003ed538;
                continue;
            }
            0x003ed538 => {
                c.cycles += 2;
                c.sb(c.g[T0][0] as u32, c.g[V0][0]);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                pc = 0x003ed540;
                continue;
            }
            0x003ed540 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                if c.g[V0][0] == 0u64 {
                    c.sb(c.g[T0][0] as u32, 0u64);
                    pc = 0x003ed568;
                    continue;
                }
                pc = 0x003ed54c;
                continue;
            }
            0x003ed54c => {
                c.cycles += 4;
                c.g[A0][0] = c.lbu(c.g[A3][0] as u32);
                c.g[V0][0] = c.g[A0][0] & 0xff;
                let t = c.g[V0][0] == c.g[T2][0];
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if t {
                    pc = 0x003ed564;
                    continue;
                }
                pc = 0x003ed55c;
                continue;
            }
            0x003ed55c => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.g[A0][0] & 0xff;
                if t {
                    if back_edge_unwinds(c, 0x003ed4c8) {
                        return;
                    }
                    pc = 0x003ed4c8;
                    continue;
                }
                pc = 0x003ed564;
                continue;
            }
            0x003ed564 => {
                c.cycles += 1;
                c.sb(c.g[T0][0] as u32, 0u64);
                pc = 0x003ed568;
                continue;
            }
            0x003ed568 => {
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
                pc = 0x003ed56c;
                continue;
            }
            0x003ed56c => {
                c.cycles += 1;
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                pc = 0x003ed570;
                continue;
            }
            0x003ed570 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003ed378, 0x003ed578);
                return;
            }
        }
    }
}

/// `int TagFieldGetBinary(const char* data, void* buf, int len)` (0x3ed678):
/// decode a `$`-prefixed hex field into `buf`, or -1 without the `$`.
pub fn tag_field_get_binary(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003ed678 => {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x440));
                c.g[T7][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x430));
                c.g[T0][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x420));
                c.g[T1][0] = c.g[T7][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x410));
                let t = (c.g[A2][0] as i64) >= 0;
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x400));
                if t {
                    pc = 0x003ed6a8;
                    continue;
                }
                pc = 0x003ed69c;
                continue;
            }
            0x003ed69c => {
                c.cycles += 3;
                c.g[V0][0] = 0xffff;
                let t = true;
                c.g[T8][0] = sx((c.g[T7][0] as u32).wrapping_add(c.g[V0][0] as u32));
                if t {
                    pc = 0x003ed6ac;
                    continue;
                }
                pc = 0x003ed6a8;
                continue;
            }
            0x003ed6a8 => {
                c.cycles += 1;
                c.g[T8][0] = sx((c.g[T7][0] as u32).wrapping_add(c.g[A2][0] as u32));
                pc = 0x003ed6ac;
                continue;
            }
            0x003ed6ac => {
                c.cycles += 2;
                let t = c.g[T0][0] != 0u64;
                c.g[V0][0] = 0x4a0000;
                if t {
                    pc = 0x003ed6b8;
                    continue;
                }
                pc = 0x003ed6b4;
                continue;
            }
            0x003ed6b4 => {
                c.cycles += 1;
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4a90));
                pc = 0x003ed6b8;
                continue;
            }
            0x003ed6b8 => {
                c.cycles += 2;
                let t = c.g[T7][0] != 0u64;
                c.g[V0][0] = c.lb(c.g[A3][0] as u32);
                if t {
                    pc = 0x003ed6c8;
                    continue;
                }
                pc = 0x003ed6c0;
                continue;
            }
            0x003ed6c0 => {
                c.cycles += 2;
                c.g[T7][0] = c.g[SP][0];
                c.g[T1][0] = c.g[SP][0];
                pc = 0x003ed6c8;
                continue;
            }
            0x003ed6c8 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
                if t {
                    pc = 0x003ed950;
                    continue;
                }
                pc = 0x003ed6d0;
                continue;
            }
            0x003ed6d0 => {
                c.cycles += 6;
                c.g[S3][0] = 0x61;
                c.g[S2][0] = 0x2;
                c.g[S0][0] = 0x4;
                c.g[S1][0] = 0x8;
                c.g[T9][0] = 0x490000;
                pc = 0x003ed6e8;
                continue;
            }
            0x003ed6e8 => {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                c.g[V1][0] = 0x23;
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[A1][0] = c.lbu(c.g[A3][0] as u32);
                    pc = 0x003ed720;
                    continue;
                }
                pc = 0x003ed6fc;
                continue;
            }
            0x003ed6fc => {
                c.cycles += 3;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = 0x3d;
                pc = 0x003ed708;
                continue;
            }
            0x003ed708 => {
                c.cycles += 3;
                c.g[V0][0] = c.lb(c.g[A3][0] as u32);
                if c.g[V0][0] == 0u64 {
                    c.g[A1][0] = c.lbu(c.g[A3][0] as u32);
                    pc = 0x003ed720;
                    continue;
                }
                pc = 0x003ed714;
                continue;
            }
            0x003ed714 => {
                c.cycles += 2;
                let t = c.g[V0][0] != c.g[V1][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                if t {
                    if back_edge_unwinds(c, 0x003ed708) {
                        return;
                    }
                    pc = 0x003ed708;
                    continue;
                }
                pc = 0x003ed71c;
                continue;
            }
            0x003ed71c => {
                c.cycles += 1;
                c.g[A1][0] = c.lbu(c.g[A3][0] as u32);
                pc = 0x003ed720;
                continue;
            }
            0x003ed720 => {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30));
                c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[T4][0] = 0u64;
                if t {
                    pc = 0x003ed758;
                    continue;
                }
                pc = 0x003ed730;
                continue;
            }
            0x003ed730 => {
                c.cycles += 2;
                c.g[A2][0] = 0xa;
                pc = 0x003ed738;
                continue;
            }
            0x003ed738 => {
                c.cycles += 8;
                c.g[A0][0] = c.g[A1][0] & 0xf;
                c.mult(0, 3, 12, 6);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = c.lbu(c.g[A3][0] as u32);
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30));
                c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[T4][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                if t {
                    if back_edge_unwinds(c, 0x003ed738) {
                        return;
                    }
                    pc = 0x003ed738;
                    continue;
                }
                pc = 0x003ed758;
                continue;
            }
            0x003ed758 => {
                c.cycles += 3;
                c.g[V0][0] = c.lb(c.g[A3][0] as u32);
                let t = c.g[V0][0] != c.g[S3][0];
                c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
                if t {
                    pc = 0x003ed770;
                    continue;
                }
                pc = 0x003ed764;
                continue;
            }
            0x003ed764 => {
                c.cycles += 2;
                if c.g[T4][0] == 0u64 {
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                    pc = 0x003ed770;
                    continue;
                }
                pc = 0x003ed76c;
                continue;
            }
            0x003ed76c => {
                c.cycles += 1;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T4][0] as u32));
                pc = 0x003ed770;
                continue;
            }
            0x003ed770 => {
                c.cycles += 11;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                c.g[A2][0] = c.g[S2][0];
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[V1][0] = c.g[V0][0] ^ 0x62;
                c.g[A0][0] = c.g[V0][0] ^ 0x77;
                if c.g[V1][0] != 0 {
                    c.g[A2][0] = 0u64;
                }
                c.g[V0][0] = c.g[V0][0] ^ 0x6c;
                if c.g[A0][0] == 0 {
                    c.g[A2][0] = c.g[S0][0];
                }
                if c.g[V0][0] == 0 {
                    c.g[A2][0] = c.g[S1][0];
                }
                let t = (c.g[A2][0] as i64) <= 0;
                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                if t {
                    pc = 0x003ed854;
                    continue;
                }
                pc = 0x003ed79c;
                continue;
            }
            0x003ed79c => {
                c.cycles += 8;
                c.g[A1][0] = c.lb(c.g[T0][0] as u32);
                c.g[T2][0] = 0u64;
                c.g[V1][0] = c.g[A1][0] ^ 0x2d;
                if c.g[V1][0] == 0 {
                    c.g[T0][0] = c.g[V0][0];
                }
                c.g[A0][0] = c.lb(c.g[T0][0] as u32);
                c.g[A0][0] = ((c.g[A0][0] as i64) < 48i64) as u64;
                let t = c.g[A0][0] != 0u64;
                c.g[V1][0] = c.lbu(c.g[T0][0] as u32);
                if t {
                    pc = 0x003ed7f4;
                    continue;
                }
                pc = 0x003ed7bc;
                continue;
            }
            0x003ed7bc => {
                c.cycles += 1;
                c.g[T3][0] = sx((c.g[T9][0] as u32).wrapping_add(0x62c0));
                pc = 0x003ed7c0;
                continue;
            }
            0x003ed7c0 => {
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                c.g[A0][0] = sx((c.g[T2][0] as u32) << 4);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                let t = (c.g[A2][0] as i64) <= 0;
                c.g[T2][0] = c.g[A0][0] | c.g[V1][0];
                if t {
                    pc = 0x003ed808;
                    continue;
                }
                pc = 0x003ed7e4;
                continue;
            }
            0x003ed7e4 => {
                c.cycles += 4;
                c.g[V0][0] = c.lb(c.g[T0][0] as u32);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 48i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.lbu(c.g[T0][0] as u32);
                if t {
                    if back_edge_unwinds(c, 0x003ed7c0) {
                        return;
                    }
                    pc = 0x003ed7c0;
                    continue;
                }
                pc = 0x003ed7f4;
                continue;
            }
            0x003ed7f4 => {
                c.cycles += 2;
                let t = (c.g[A2][0] as i64) <= 0;
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                if t {
                    pc = 0x003ed808;
                    continue;
                }
                pc = 0x003ed7fc;
                continue;
            }
            0x003ed7fc => {
                c.cycles += 3;
                c.g[V0][0] = c.lb(c.g[T0][0] as u32);
                c.g[V0][0] = c.g[V0][0] ^ 0x2c;
                if c.g[V0][0] == 0 {
                    c.g[T0][0] = c.g[V1][0];
                }
                pc = 0x003ed808;
                continue;
            }
            0x003ed808 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[A1][0] ^ 0x2d;
                c.g[V1][0] = sx(0u32.wrapping_sub(c.g[T2][0] as u32));
                c.g[A1][0] = c.lb(c.g[A3][0] as u32);
                c.g[V0][0] = 0x62;
                let t = c.g[A1][0] != c.g[V0][0];
                if c.g[A0][0] == 0 {
                    c.g[T2][0] = c.g[V1][0];
                }
                if t {
                    pc = 0x003ed828;
                    continue;
                }
                pc = 0x003ed820;
                continue;
            }
            0x003ed820 => {
                c.cycles += 2;
                c.sb(c.g[T1][0] as u32, c.g[T2][0]);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                pc = 0x003ed828;
                continue;
            }
            0x003ed828 => {
                c.cycles += 4;
                c.g[V1][0] = c.lb(c.g[A3][0] as u32);
                c.g[V0][0] = 0x77;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0x6c;
                if t {
                    pc = 0x003ed844;
                    continue;
                }
                pc = 0x003ed838;
                continue;
            }
            0x003ed838 => {
                c.cycles += 3;
                c.sh(c.g[T1][0] as u32, c.g[T2][0]);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2));
                c.g[V1][0] = c.lb(c.g[A3][0] as u32);
                pc = 0x003ed844;
                continue;
            }
            0x003ed844 => {
                c.cycles += 2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V1][0] = c.lb(c.g[A3][0] as u32);
                    pc = 0x003ed858;
                    continue;
                }
                pc = 0x003ed84c;
                continue;
            }
            0x003ed84c => {
                c.cycles += 2;
                c.sw(c.g[T1][0] as u32, c.g[T2][0]);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x4));
                pc = 0x003ed854;
                continue;
            }
            0x003ed854 => {
                c.cycles += 1;
                c.g[V1][0] = c.lb(c.g[A3][0] as u32);
                pc = 0x003ed858;
                continue;
            }
            0x003ed858 => {
                c.cycles += 3;
                c.g[V0][0] = 0x73;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = (c.g[T1][0] < c.g[T8][0]) as u64;
                if t {
                    pc = 0x003ed928;
                    continue;
                }
                pc = 0x003ed864;
                continue;
            }
            0x003ed864 => {
                c.cycles += 2;
                let t = (c.g[T4][0] as i64) <= 0;
                c.g[V0][0] = 0x490000;
                if t {
                    pc = 0x003ed924;
                    continue;
                }
                pc = 0x003ed86c;
                continue;
            }
            0x003ed86c => {
                c.cycles += 5;
                c.g[A2][0] = 0u64;
                c.g[T6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x61c0));
                c.g[T3][0] = 0x25;
                let t = true;
                c.g[T5][0] = sx((c.g[T9][0] as u32).wrapping_add(0x62c0));
                if t {
                    pc = 0x003ed8c0;
                    continue;
                }
                pc = 0x003ed880;
                continue;
            }
            0x003ed880 => {
                c.cycles += 2;
                let t = c.g[V1][0] != c.g[T3][0];
                c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                if t {
                    pc = 0x003ed8b4;
                    continue;
                }
                pc = 0x003ed888;
                continue;
            }
            0x003ed888 => {
                c.cycles += 11;
                c.g[A1][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0x1));
                c.g[A2][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[A0][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0x2));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T6][0] as u32));
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x3));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T5][0] as u32));
                c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                let t = true;
                c.sb(c.g[A2][0] as u32, c.g[V0][0]);
                if t {
                    pc = 0x003ed8bc;
                    continue;
                }
                pc = 0x003ed8b4;
                continue;
            }
            0x003ed8b4 => {
                c.cycles += 2;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                c.sb(c.g[V0][0] as u32, c.g[A0][0]);
                pc = 0x003ed8bc;
                continue;
            }
            0x003ed8bc => {
                c.cycles += 1;
                c.g[A2][0] = c.g[T2][0];
                pc = 0x003ed8c0;
                continue;
            }
            0x003ed8c0 => {
                c.cycles += 4;
                c.g[V1][0] = c.lb(c.g[T0][0] as u32);
                c.g[V0][0] = ((c.g[V1][0] as i64) < 48i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.lbu(c.g[T0][0] as u32);
                if t {
                    pc = 0x003ed8d8;
                    continue;
                }
                pc = 0x003ed8d0;
                continue;
            }
            0x003ed8d0 => {
                c.cycles += 2;
                let t = c.g[V1][0] != c.g[T3][0];
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[T4][0] as i64)) as u64;
                if t {
                    pc = 0x003ed8ec;
                    continue;
                }
                pc = 0x003ed8d8;
                continue;
            }
            0x003ed8d8 => {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                c.g[T2][0] = c.g[V0][0];
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T4][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[T4][0] as i64)) as u64;
                if t {
                    if back_edge_unwinds(c, 0x003ed880) {
                        return;
                    }
                    pc = 0x003ed880;
                    continue;
                }
                pc = 0x003ed8ec;
                continue;
            }
            0x003ed8ec => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                if t {
                    pc = 0x003ed914;
                    continue;
                }
                pc = 0x003ed8f4;
                continue;
            }
            0x003ed8f4 => {
                c.cycles += 1;
                pc = 0x003ed8f8;
                continue;
            }
            0x003ed8f8 => {
                c.cycles += 7;
                c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[T4][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.sb(c.g[V1][0] as u32, 0u64);
                if t {
                    if back_edge_unwinds(c, 0x003ed8f8) {
                        return;
                    }
                    pc = 0x003ed8f8;
                    continue;
                }
                pc = 0x003ed914;
                continue;
            }
            0x003ed914 => {
                c.cycles += 4;
                c.g[V0][0] = c.lb(c.g[T0][0] as u32);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V0][0] = c.g[V0][0] ^ 0x2c;
                if c.g[V0][0] == 0 {
                    c.g[T0][0] = c.g[A0][0];
                }
                pc = 0x003ed924;
                continue;
            }
            0x003ed924 => {
                c.cycles += 1;
                c.g[V0][0] = (c.g[T1][0] < c.g[T8][0]) as u64;
                pc = 0x003ed928;
                continue;
            }
            0x003ed928 => {
                c.cycles += 2;
                if c.g[V0][0] == 0u64 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x430));
                    pc = 0x003ed954;
                    continue;
                }
                pc = 0x003ed930;
                continue;
            }
            0x003ed930 => {
                c.cycles += 8;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lb(c.g[A3][0] as u32);
                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = c.g[V0][0] ^ 0x2a;
                if c.g[V0][0] == 0 {
                    c.g[A3][0] = c.g[V1][0];
                }
                c.g[A0][0] = c.lb(c.g[A3][0] as u32);
                let t = c.g[A0][0] != 0u64;
                c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
                if t {
                    if back_edge_unwinds(c, 0x003ed6e8) {
                        return;
                    }
                    pc = 0x003ed6e8;
                    continue;
                }
                pc = 0x003ed950;
                continue;
            }
            0x003ed950 => {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x430));
                pc = 0x003ed954;
                continue;
            }
            0x003ed954 => {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[T7][0] as u32));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x420));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x410));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x400));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x440));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003ed678, 0x003ed96c);
                return;
            }
        }
    }
}

/// `int TagFieldGetEncoded(const char* data, char* buf, int len, const char* key, const char* dflt)`
/// (0x3ed970): decode a `$`-prefixed encrypted field, or fall back to GetString.
pub fn tag_field_get_encoded(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003ed970 => {
                c.cycles += 6;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[T2][0] = c.g[A1][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[T5][0] = c.g[A2][0];
                let t = c.g[A0][0] == 0u64;
                c.g[T4][0] = c.g[A3][0];
                if t {
                    pc = 0x003eda0c;
                    continue;
                }
                pc = 0x003ed988;
                continue;
            }
            0x003ed988 => {
                c.cycles += 4;
                c.g[V1][0] = c.lb(c.g[A0][0] as u32);
                c.g[V0][0] = 0x24;
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[A1][0] = c.g[T2][0];
                if t {
                    pc = 0x003ed9ac;
                    continue;
                }
                pc = 0x003ed998;
                continue;
            }
            0x003ed998 => {
                c.cycles += 5;
                c.g[A2][0] = c.g[T5][0];
                c.g[A3][0] = c.g[T0][0];
                if call_ptr(c, 0x003ed378, 0x003ed9a4).is_err() {
                    return;
                }
                let t = true;
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                if t {
                    pc = 0x003edafc;
                    continue;
                }
                pc = 0x003ed9ac;
                continue;
            }
            0x003ed9ac => {
                c.cycles += 2;
                let t = c.g[T4][0] == 0u64;
                c.g[V0][0] = 0x4a0000;
                if t {
                    pc = 0x003ed9c0;
                    continue;
                }
                pc = 0x003ed9b4;
                continue;
            }
            0x003ed9b4 => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu(c.g[T4][0] as u32);
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x4a0000;
                if t {
                    pc = 0x003ed9c4;
                    continue;
                }
                pc = 0x003ed9c0;
                continue;
            }
            0x003ed9c0 => {
                c.cycles += 1;
                c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4a88));
                pc = 0x003ed9c4;
                continue;
            }
            0x003ed9c4 => {
                c.cycles += 2;
                let t = c.g[T2][0] != 0u64;
                c.g[T3][0] = 0u64;
                if t {
                    pc = 0x003eda04;
                    continue;
                }
                pc = 0x003ed9cc;
                continue;
            }
            0x003ed9cc => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x1));
                let t = true;
                c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                if t {
                    pc = 0x003ed9e0;
                    continue;
                }
                pc = 0x003ed9d8;
                continue;
            }
            0x003ed9d8 => {
                c.cycles += 2;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lbu(c.g[T0][0] as u32);
                pc = 0x003ed9e0;
                continue;
            }
            0x003ed9e0 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 0x30u64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.g[T3][0];
                if t {
                    pc = 0x003edaf8;
                    continue;
                }
                pc = 0x003ed9ec;
                continue;
            }
            0x003ed9ec => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[T0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[V0][0] < 0x30u64) as u64;
                if c.g[V0][0] == 0u64 {
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x2));
                    if back_edge_unwinds(c, 0x003ed9d8) {
                        return;
                    }
                    pc = 0x003ed9d8;
                    continue;
                }
                pc = 0x003ed9fc;
                continue;
            }
            0x003ed9fc => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = c.g[T3][0];
                if t {
                    pc = 0x003edaf8;
                    continue;
                }
                pc = 0x003eda04;
                continue;
            }
            0x003eda04 => {
                c.cycles += 2;
                let t = (c.g[T5][0] as i64) > 0;
                c.g[T3][0] = 0x1;
                if t {
                    pc = 0x003eda14;
                    continue;
                }
                pc = 0x003eda0c;
                continue;
            }
            0x003eda0c => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    pc = 0x003edaf8;
                    continue;
                }
                pc = 0x003eda14;
                continue;
            }
            0x003eda14 => {
                c.cycles += 5;
                c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[T1][0] = c.g[T4][0];
                c.g[V0][0] = ((c.g[T3][0] as i64) < (c.g[T5][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[T8][0] = 0u64;
                if t {
                    pc = 0x003edaf0;
                    continue;
                }
                pc = 0x003eda28;
                continue;
            }
            0x003eda28 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[V0][0] < 0x30u64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.sb(c.g[T2][0] as u32, 0u64);
                    pc = 0x003edaf4;
                    continue;
                }
                pc = 0x003eda38;
                continue;
            }
            0x003eda38 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[T0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[V0][0] < 0x30u64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.sb(c.g[T2][0] as u32, 0u64);
                    pc = 0x003edaf4;
                    continue;
                }
                pc = 0x003eda48;
                continue;
            }
            0x003eda48 => {
                c.cycles += 6;
                c.g[V0][0] = 0x490000;
                c.g[V1][0] = 0x490000;
                c.g[T7][0] = sx((c.g[V0][0] as u32).wrapping_add(0x61c0));
                c.g[T6][0] = sx((c.g[V1][0] as u32).wrapping_add(0x62c0));
                c.g[A1][0] = c.lbu(c.g[T0][0] as u32);
                pc = 0x003eda60;
                continue;
            }
            0x003eda60 => {
                c.cycles += 28;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = c.lbu((c.g[T0][0] as u32).wrapping_add(0x1));
                c.g[A3][0] = ((c.g[T3][0] as i64) < (c.g[T5][0] as i64)) as u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T7][0] as u32));
                c.g[V0][0] = c.lbu(c.g[T1][0] as u32);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T6][0] as u32));
                c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
                c.g[A2][0] = c.lbu(c.g[A0][0] as u32);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = c.lbu(c.g[T1][0] as u32);
                c.g[V1][0] = c.g[V1][0] | c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                if c.g[A0][0] == 0 {
                    c.g[T1][0] = c.g[T4][0];
                }
                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 3);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] ^ c.g[T8][0];
                c.sb(c.g[T2][0] as u32, c.g[V0][0]);
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lbu(c.g[T0][0] as u32);
                c.g[A0][0] = c.lbu((c.g[T0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T7][0] as u32));
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x2));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T6][0] as u32));
                c.g[A1][0] = c.lbu(c.g[V1][0] as u32);
                c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                let t = c.g[A3][0] == 0u64;
                c.g[T8][0] = c.g[V0][0] | c.g[A1][0];
                if t {
                    pc = 0x003edaf0;
                    continue;
                }
                pc = 0x003edad0;
                continue;
            }
            0x003edad0 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu(c.g[T0][0] as u32);
                c.g[V0][0] = (c.g[V0][0] < 0x30u64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.sb(c.g[T2][0] as u32, 0u64);
                    pc = 0x003edaf4;
                    continue;
                }
                pc = 0x003edae0;
                continue;
            }
            0x003edae0 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[T0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[V0][0] < 0x30u64) as u64;
                if c.g[V0][0] == 0u64 {
                    c.g[A1][0] = c.lbu(c.g[T0][0] as u32);
                    if back_edge_unwinds(c, 0x003eda60) {
                        return;
                    }
                    pc = 0x003eda60;
                    continue;
                }
                pc = 0x003edaf0;
                continue;
            }
            0x003edaf0 => {
                c.cycles += 1;
                c.sb(c.g[T2][0] as u32, 0u64);
                pc = 0x003edaf4;
                continue;
            }
            0x003edaf4 => {
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x1));
                pc = 0x003edaf8;
                continue;
            }
            0x003edaf8 => {
                c.cycles += 1;
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                pc = 0x003edafc;
                continue;
            }
            0x003edafc => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003ed970, 0x003edb04);
                return;
            }
        }
    }
}

/// `u32 TagFieldGetDate(const char* data, u32 dflt)` (0x3edb08): parse a date
/// tag (`$`-hex epoch, plain seconds, or `YYYY-MM-DD HH:MM:SS`) into seconds.
pub fn tag_field_get_date(c: &mut Cpu) {
    guest(c, |c| {
        'b_3edd2c: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S1][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S2][0] = 0u64;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S3][0] = c.g[A1][0];
            let t = c.g[S1][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
            if t {
                break 'b_3edd2c;
            } else {
                'b_3edb94: {
                    c.cycles += 4;
                    c.g[V1][0] = c.lb(c.g[S1][0] as u32);
                    c.g[V0][0] = 0x24;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = c.lbu(c.g[S1][0] as u32);
                    if t {
                        break 'b_3edb94;
                    } else {
                        c.cycles += 5;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = c.lb(c.g[S1][0] as u32);
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 48i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A2][0] = c.lbu(c.g[S1][0] as u32);
                        if t {
                            break 'b_3edd2c;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x490000;
                            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x62c0));
                            'l_3edb60: loop {
                                c.cycles += 11;
                                c.g[V0][0] = sx((c.g[A2][0] as u32) << 24);
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                                c.g[V1][0] = c.lb(c.g[S1][0] as u32);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                                c.g[A1][0] = sx((c.g[S2][0] as u32) << 4);
                                c.g[A0][0] = c.lbu(c.g[V0][0] as u32);
                                c.g[V1][0] = ((c.g[V1][0] as i64) < 48i64) as u64;
                                c.g[A2][0] = c.lbu(c.g[S1][0] as u32);
                                let t = c.g[V1][0] == 0u64;
                                c.g[S2][0] = c.g[A1][0] | c.g[A0][0];
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003edb60);
                                    }
                                    continue 'l_3edb60;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    if t {
                                        break 'b_3edd2c;
                                    } else {
                                        break 'b_3edb94;
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x30));
                c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[S1][0];
                if t {
                    break 'b_3edd2c;
                } else {
                    'b_3edbc4: {
                        c.cycles += 6;
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                        call_ptr(c, 0x003eae68, 0x003edbac)?;
                        c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                        c.g[V1][0] = ((c.g[V1][0] as i64) < 33i64) as u64;
                        if c.g[V1][0] == 0u64 {
                            c.g[V0][0] = c.lbu(c.g[S1][0] as u32);
                            break 'b_3edbc4;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_3edd2c;
                            } else {
                                break 'b_3edbc4;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        break 'b_3edd2c;
                    } else {
                        'b_3edcd0: {
                            'b_3edcc8: {
                                c.cycles += 53;
                                c.g[A2][0] = 0x24;
                                c.g[A1][0] = 0u64;
                                c.g[S0][0] = 0x1;
                                call_ptr(c, 0x00416210, 0x003edbe4)?;
                                c.g[V1][0] = 0xffffffffffffffff;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                                c.g[A0][0] = c.g[S1][0];
                                call_ptr(c, 0x003eae68, 0x003edbf8)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[V0][0] = c.lbu(c.g[S1][0] as u32);
                                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2d));
                                c.g[V0][0] = c.g[V0][0] & 0xff;
                                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                                if c.g[V0][0] != 0 {
                                    c.g[A0][0] = c.g[S1][0];
                                }
                                call_ptr(c, 0x003eae68, 0x003edc1c)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[A1][0] = c.g[SP][0] | 0xc;
                                c.g[V0][0] = c.lbu(c.g[S1][0] as u32);
                                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2d));
                                c.g[V0][0] = c.g[V0][0] & 0xff;
                                c.g[S0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                                if c.g[S0][0] != 0 {
                                    c.g[A0][0] = c.g[S1][0];
                                }
                                call_ptr(c, 0x003eae68, 0x003edc40)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[A1][0] = c.g[SP][0] | 0x8;
                                c.g[V0][0] = c.lb(c.g[S1][0] as u32);
                                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = c.g[V0][0] ^ 0x20;
                                if c.g[V0][0] != 0 {
                                    c.g[A0][0] = c.g[S1][0];
                                }
                                call_ptr(c, 0x003eae68, 0x003edc5c)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[A1][0] = c.g[SP][0] | 0x4;
                                c.g[V0][0] = c.lb(c.g[S1][0] as u32);
                                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = c.g[V0][0] ^ 0x3a;
                                if c.g[V0][0] != 0 {
                                    c.g[A0][0] = c.g[S1][0];
                                }
                                call_ptr(c, 0x003eae68, 0x003edc78)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[A1][0] = c.g[SP][0];
                                c.g[V0][0] = c.lb(c.g[S1][0] as u32);
                                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = c.g[V0][0] ^ 0x3a;
                                if c.g[V0][0] != 0 {
                                    c.g[A0][0] = c.g[S1][0];
                                }
                                call_ptr(c, 0x003eae68, 0x003edc94)?;
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7b2));
                                c.g[V1][0] = (c.g[V1][0] < 0x82u64) as u64;
                                let t = c.g[V1][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_3edcc8;
                                } else {
                                    c.cycles += 2;
                                    let t = (c.g[V0][0] as i64) <= 0;
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < 13i64) as u64;
                                    if t {
                                        break 'b_3edcc8;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                        if t {
                                            break 'b_3edcc8;
                                        } else {
                                            c.cycles += 2;
                                            let t = (c.g[V0][0] as i64) <= 0;
                                            c.g[V0][0] = ((c.g[V0][0] as i64) < 32i64) as u64;
                                            if t {
                                                break 'b_3edcc8;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                                if t {
                                                    break 'b_3edcd0;
                                                } else {
                                                    break 'b_3edcc8;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                            break 'b_3edcd0;
                        }
                        'b_3edd04: {
                            'b_3edcfc: {
                                c.cycles += 3;
                                c.g[V0][0] = (c.g[V0][0] < 0x18u64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                if t {
                                    break 'b_3edcfc;
                                } else {
                                    c.cycles += 2;
                                    let t = (c.g[V0][0] as i64) < 0;
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < 60i64) as u64;
                                    if t {
                                        break 'b_3edcfc;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                                        if t {
                                            break 'b_3edcfc;
                                        } else {
                                            c.cycles += 2;
                                            let t = (c.g[V0][0] as i64) < 0;
                                            c.g[V0][0] = ((c.g[V0][0] as i64) < 62i64) as u64;
                                            if t {
                                                break 'b_3edcfc;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                                if t {
                                                    break 'b_3edd04;
                                                } else {
                                                    break 'b_3edcfc;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                            break 'b_3edd04;
                        }
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                        if t {
                            break 'b_3edd2c;
                        } else {
                            c.cycles += 8;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x76c));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                            c.g[A0][0] = c.g[SP][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                            call_ptr(c, 0x003eb090, 0x003edd24)?;
                            c.g[S2][0] = c.g[V0][0] << 32;
                            c.g[S2][0] = ((c.g[S2][0] as i64) >> 32) as u64;
                            break 'b_3edd2c;
                        }
                    }
                }
            }
        }
        'b_3edd50: {
            c.cycles += 2;
            let t = c.g[S2][0] != 0u64;
            c.g[V0][0] = c.g[S2][0];
            if t {
                break 'b_3edd50;
            } else {
                c.cycles += 3;
                c.g[S2][0] = c.g[S3][0];
                let t = c.g[S2][0] != 0u64;
                c.g[V0][0] = c.g[S2][0];
                if t {
                    break 'b_3edd50;
                } else {
                    c.cycles += 4;
                    call_ptr(c, 0x003ef088, 0x003edd48)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V0][0] = c.g[S2][0];
                    break 'b_3edd50;
                }
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        Ok(())
    })
}

/// `int TagFieldGetDateYMDHMS(char* data, int* y, int* mo, int* d, int* h, int* mi, int* s, int off)`
/// (0x3edd70): split a date tag into calendar fields.
pub fn tag_field_get_date_ymdhms(c: &mut Cpu) {
    guest(c, |c| {
        'b_3ede54: {
            c.cycles += 22;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S1][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[A1][0] = 0x1;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S2][0] = c.g[A2][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A3][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S4][0] = c.g[T0][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S5][0] = c.g[T1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[S6][0] = c.g[T2][0];
            c.g[S0][0] = c.g[T3][0];
            call_ptr(c, 0x003edb08, 0x003eddb8)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0xffffffffffffffff;
            if t {
                break 'b_3ede54;
            } else {
                c.cycles += 8;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[A1][0] << 32;
                c.g[A1][0] = c.g[A1][0] >> 32;
                call_ptr(c, 0x003eaeb8, 0x003edddc)?;
                c.g[V1][0] = c.g[V0][0];
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    break 'b_3ede54;
                } else {
                    'b_3eddfc: {
                        c.cycles += 2;
                        let t = c.g[S1][0] == 0u64;
                        if t {
                            break 'b_3eddfc;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x76c));
                            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                            break 'b_3eddfc;
                        }
                    }
                    'b_3ede10: {
                        c.cycles += 2;
                        let t = c.g[S2][0] == 0u64;
                        if t {
                            break 'b_3ede10;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                            break 'b_3ede10;
                        }
                    }
                    'b_3ede20: {
                        c.cycles += 2;
                        let t = c.g[S3][0] == 0u64;
                        if t {
                            break 'b_3ede20;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                            c.sw(c.g[S3][0] as u32, c.g[V0][0]);
                            break 'b_3ede20;
                        }
                    }
                    'b_3ede30: {
                        c.cycles += 2;
                        let t = c.g[S4][0] == 0u64;
                        if t {
                            break 'b_3ede30;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                            c.sw(c.g[S4][0] as u32, c.g[V0][0]);
                            break 'b_3ede30;
                        }
                    }
                    'b_3ede40: {
                        c.cycles += 2;
                        let t = c.g[S5][0] == 0u64;
                        if t {
                            break 'b_3ede40;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                            c.sw(c.g[S5][0] as u32, c.g[V0][0]);
                            break 'b_3ede40;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S6][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_3ede54;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.sw(c.g[S6][0] as u32, c.g[V0][0]);
                        c.g[V0][0] = 0u64;
                        break 'b_3ede54;
                    }
                }
            }
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        Ok(())
    })
}

/// `int TagFieldGetDatePacked(char* data, u32* date, u32* time, int off)` (0x3ede80):
/// split a date tag into packed date/time bit fields.
pub fn tag_field_get_date_packed(c: &mut Cpu) {
    guest(c, |c| {
        'b_3edf34: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S1][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = 0x1;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[S2][0] = c.g[A2][0];
            c.g[S0][0] = c.g[A3][0];
            call_ptr(c, 0x003edb08, 0x003edea8)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0xffffffffffffffff;
            if t {
                break 'b_3edf34;
            } else {
                c.cycles += 8;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[A1][0] << 32;
                c.g[A1][0] = c.g[A1][0] >> 32;
                call_ptr(c, 0x003eaeb8, 0x003edecc)?;
                c.g[A1][0] = c.g[V0][0];
                let t = c.g[A1][0] == 0u64;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    break 'b_3edf34;
                } else {
                    'b_3edf08: {
                        c.cycles += 2;
                        let t = c.g[S1][0] == 0u64;
                        if t {
                            break 'b_3edf08;
                        } else {
                            c.cycles += 10;
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x76c));
                            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
                            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                            c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                            break 'b_3edf08;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S2][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_3edf34;
                    } else {
                        c.cycles += 9;
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.lw(c.g[A1][0] as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
                        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                        c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                        c.g[V0][0] = 0u64;
                        break 'b_3edf34;
                    }
                }
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}
#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::ai::computer::tests::wb;
    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::bx::bxstring::tests::b;
    pub(crate) use crate::game::bx::bxstring::tests::heap;
    pub(crate) use crate::game::bx::bxstring::tests::put_str_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_18 as base_5;

    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict_2 as strict_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_17 as SEEDS_3;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_4;

    pub(crate) use crate::game::fe::festaterewards::tests::strict as strict_2;

    pub(crate) use crate::game::main::gamemode::tests::be_library;

    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base;

    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_3;

    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;

    use ssx3_ee as ee;

    // -----------------------------------------------------------------------
    // 0x3ebb60 TagFieldStrnzcpy.

    #[test]
    fn tag_field_strnzcpy_matches() {
        check(
            ee::s14::f_003ebb60__generated,
            tag_field_strnzcpy,
            32,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3ebb_60, 0);
                c.pc = 0x003e_bb60;
                let src = DATA + 0x200;
                let dst = DATA + 0x400;
                c.g[A0][0] = sx(dst);
                c.g[A1][0] = (seed % 6) as u64;
                c.g[A2][0] = sx(src);
                let n = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 3,
                    _ => 5,
                };
                for i in 0..n {
                    c.bus.write8(src + i, b'a' + (r.next_u32() % 26) as u8);
                }
                c.bus.write8(src + n, 0);
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x3ecfb0 TagFieldGetFlags.

    #[test]
    fn tag_field_get_flags_matches() {
        check(
            ee::s14::f_003ecfb0__generated,
            tag_field_get_flags,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3ecf_b0, 0);
                c.pc = 0x003e_cfb0;
                let text = DATA + 0x200;
                c.g[A1][0] = r.next_u32() as u64;
                if seed % 4 == 0 {
                    c.g[A0][0] = 0;
                } else {
                    c.g[A0][0] = sx(text);
                    let n = 1 + seed % 6;
                    for i in 0..n {
                        c.bus.write8(text + i, (r.next_u32() % 0x30) as u8);
                    }
                    c.bus.write8(text + n, 0);
                    // A table of nonzero flag words for the tag characters used.
                    for i in 0..0x30u32 {
                        let (ch, flags) = (i as u8, (i * 3) as u32);
                        c.bus.write32(0x0049_65e0 + 4 * ch as u32, flags);
                    }
                }
            },
        );
    }

    #[test]
    fn tag_field_get_address_matches() {
        check(
            ee::s14::f_003ecff8__generated,
            tag_field_get_address,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3ecff8, 0);
                be_library(c);
                c.pc = 0x003e_cff8;
                c.g[A1][0] = r.next_u32() as u64;
                let s = DATA + 0x200;
                if seed % 4 == 0 {
                    c.g[A0][0] = 0;
                } else {
                    c.g[A0][0] = s as u64;
                    let n = 1 + seed % 6;
                    let chars = *b"0129.x ";
                    for i in 0..n {
                        b(c, s + i, chars[((seed + i) % 7) as usize]);
                    }
                    b(c, s + n, 0);
                }
            },
        );
    }

    pub(crate) fn put_str(c: &mut Cpu, a: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            wb(c, a + i as u32, *b);
        }
        wb(c, a + s.len() as u32, 0);
    }

    #[test]
    fn tag_field_get_number_matches() {
        check(
            ee::s14::f_003ecf30__generated,
            tag_field_get_number,
            24,
            &|c, seed| {
                let _ = base(c, seed, 0x3ecf_30, 0);
                let tag = DATA + 0x1000;
                let text: &[u8] = match seed % 6 {
                    0 => b"123",
                    1 => b"-45",
                    2 => b"+7",
                    3 => b"abc",
                    4 => b"",
                    _ => b"0x10",
                };
                put_str(c, tag, text);
                c.g[A0][0] = tag as u64;
                c.g[A1][0] = 77;
            },
        );
    }

    // ------------------------------------------------------------------
    // Tag fields.

    #[test]
    fn tag_field_get_token_matches() {
        check(
            ssx3_ee::s14::f_003ed070__generated,
            tag_field_get_token,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3ed070, 0);
                let s = DATA + 0x200;
                if seed % 7 == 0 {
                    c.g[A0][0] = 0;
                } else {
                    c.g[A0][0] = s as u64;
                    // A run of printable characters, a control character and a
                    // high byte, at varying offsets.
                    let run = (r.next_u32() % 6) as usize;
                    for i in 0..0x10 {
                        let v = if i < run {
                            b'a' + (r.next_u32() % 26) as u8
                        } else {
                            [0u8, 0x20, 0x7f, 0x80, 0xff][(r.next_u32() % 5) as usize]
                        };
                        b(c, s + i as u32, v);
                    }
                }
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn tag_field_set_token_matches() {
        check(
            ee::s14::f_003ec0f8__generated,
            tag_field_set_token,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x3ec0f8, 2);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A3][0] = [0u64, 1, 0x414243, u32::MAX as u64][seed as usize % 4];
            },
        );
    }

    #[test]
    fn tag_field_set_flags_matches() {
        check(
            ssx3_ee::s14::f_003ebf28__generated,
            tag_field_set_flags,
            12,
            &|c, seed| {
                let _ = base_2(c, seed, 0x003e_bf28);
                heap(c);
                let tag = DATA + 0x100;
                clear(c, tag, 0x120);
                put_str_2(c, 0x0049_65c0, "0123456789abcdefghijklmnopqrstuv");
                c.g[A0][0] = sx(tag);
                c.g[A1][0] = (seed % 0x10000) as u64;
                c.g[A2][0] = 0;
                c.g[A3][0] = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 0xffff_ffff,
                    _ => (seed * 2654435761) as u64,
                };
            },
        );
    }

    #[test]
    fn tag_field_set_binary_matches() {
        strict(
            ee::s14::f_003ec300__generated,
            tag_field_set_binary,
            120,
            &|c, seed| {
                let mut r = Rng(0x003e_c300 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                let record = DATA;
                c.bus.write32(record, 0);
                let name = DATA + 0x100;
                let len = if seed % 3 == 0 {
                    0
                } else {
                    1 + (r.next_u32() % 6) as usize
                };
                for i in 0..len {
                    c.bus
                        .write8(name + i as u32, b'A' + (r.next_u32() % 26) as u8);
                }
                c.bus.write8(name + len as u32, 0);
                c.bus.write8(name + len as u32 + 1, b'Q');
                for i in 0..16u32 {
                    c.bus
                        .write8(DATA + 0x200 + i, (r.next_u32() % 40 + 32) as u8);
                }
                let size: i32 = [0, 1, 3, -1][(seed / 2 % 4) as usize];
                c.g[A0][0] = sx(record);
                c.g[A1][0] = 0;
                c.g[A2][0] = if seed % 3 == 0 { 0 } else { sx(name) };
                c.g[A3][0] = sx(DATA + 0x200);
                c.g[T0][0] = sx(size as u32);
            },
        );
    }

    #[test]
    fn tag_field_merge_matches() {
        check(
            ee::s14::f_003ecb00__generated,
            tag_field_merge,
            16,
            &|c, seed| {
                let _ = base_3(c, seed, 0x3ecb00, 2);
                let rec = DATA;
                let data = DATA + 0x200;
                clear_2(c, rec, 0x200);
                clear_2(c, data, 0x200);
                c.pc = 0x003e_cb00;
                c.g[A0][0] = rec as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = data as u64;
                let text: &[u8] = match seed % 5 {
                    0 => b"name=value other:2  \0",
                    1 => b"\0",
                    2 => b"   \0",
                    3 => b"a=b\0",
                    _ => b"key=1=2\0",
                };
                for (i, b) in text.iter().enumerate() {
                    c.bus.write8(data + i as u32, *b);
                }
            },
        );
    }

    // ----------------------------------------------------------- tag fields

    #[test]
    fn tag_field_set_number_matches() {
        strict_2(
            ee::s14::f_003ebe40__generated,
            tag_field_set_number,
            90,
            &|c, seed| {
                let _ = base_4(c, seed, 0x3ebe, 6);
                let value = match seed % 5 {
                    0 => 0,
                    1 => 7,
                    2 => 12345,
                    3 => 0x7fff_ffff,
                    _ => (seed % 1000) + 1,
                };
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 8;
                c.g[A2][0] = (DATA + 0x900) as u64;
                c.g[A3][0] = if seed % 2 == 0 {
                    value as u64
                } else {
                    (-(value as i64)) as u64
                };
                for (i, b) in b"name\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x900 + i as u32, *b);
                }
                c.pc = 0x003e_be40;
            },
        );
    }

    #[test]
    fn tag_field_decode_hex_matches() {
        strict_3(
            ssx3_ee::s14::f_003ed578__generated,
            tag_field_decode_hex,
            SEEDS_3,
            &|c, seed| {
                let _ = base_5(c, seed, 0xd578);
                let data = DATA + 0x1000;
                let buf = DATA + 0x2000;
                let len = DATA + 0x3000;
                let text: &[u8] = match seed % 6 {
                    0 => b"\0",
                    1 => b"XABCDEF\0",
                    2 => b"$ABCDEF\0",
                    3 => b"$ABCDEF\0",
                    4 => b"$ABCDEF\0",
                    _ => b"$AB\0",
                };
                for (i, b) in text.iter().enumerate() {
                    c.bus.write8(data + i as u32, *b);
                }
                c.bus.write16(len, (seed % 4) as u16);
                c.g[A0][0] = sx(if seed % 6 == 0 { 0 } else { data });
                c.g[A1][0] = sx(if seed % 6 == 2 { 0 } else { buf });
                c.g[A2][0] = sx(c.bus.read32(len));
                c.pc = 0x003e_d578;
            },
        );
    }

    /// The tag helpers read these tables at fixed addresses: the field
    /// separator (gp+0x1980), the name-class table (0x4964c0) and the two
    /// hex tables (0x4961c0/0x4962c0 decode, 0x495fc0/0x4960c0 encode).
    /// Stand-in values, not game strings.
    /// `ds_secstotime` walks a month-length table at 0x4969e0, subtracting
    /// entries from the day count (plain row first, leap row at word 12);
    /// without game rodata the scan reads zeros and never terminates, so
    /// stand in the real month lengths (calendar facts, not game data).
    fn tables(c: &mut Cpu) {
        c.bus.write8(0x004a_4a70, b'\n');
        for (i, days) in [
            31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31,
            30, 31,
        ]
        .iter()
        .enumerate()
        {
            c.bus.write32(0x0049_69e0 + i as u32 * 4, *days);
        }
        for i in 0..256u32 {
            let cls = if (b'a'..=b'z').contains(&(i as u8))
                || (b'A'..=b'Z').contains(&(i as u8))
                || (b'0'..=b'9').contains(&(i as u8))
                || i == b'_' as u32
                || i == b'-' as u32
                || i == b'.' as u32
            {
                2
            } else if i == b'=' as u32 || i == b':' as u32 {
                1
            } else {
                0
            };
            c.bus.write8(0x0049_64c0 + i, cls);
            let v = (i as u8 as char).to_digit(16).unwrap_or(0) as u8;
            c.bus.write8(0x0049_61c0 + i, v << 4);
            c.bus.write8(0x0049_62c0 + i, v);
            c.bus
                .write8(0x0049_5fc0 + i, b"0123456789abcdef"[(i >> 4) as usize]);
            c.bus
                .write8(0x0049_60c0 + i, b"0123456789abcdef"[(i & 15) as usize]);
        }
    }

    // 0x3eb198 TagFieldCopyTag.
    #[test]
    fn tag_field_copy_tag_matches() {
        check(
            ee::s14::f_003eb198__generated,
            tag_field_copy_tag,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3eb198, 0);
                c.pc = 0x003eb198;
                let name = DATA + 0x200;
                let dst = DATA + 0x400;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(dst);
                if seed % 3 == 0 {
                    c.g[A2][0] = 0;
                } else {
                    c.g[A2][0] = sx(name);
                    let n = (r.next_u32() % 6) as usize;
                    for i in 0..n {
                        c.bus
                            .write8(name + i as u32, b'a' + (r.next_u32() % 26) as u8);
                    }
                    c.bus.write8(name + n as u32, 0);
                }
            },
        );
    }

    // 0x3eb1e0 TagFieldSetRaw.
    #[test]
    fn tag_field_set_raw_matches() {
        check(
            ee::s14::f_003eb1e0__generated,
            tag_field_set_raw,
            96,
            &|c, seed| {
                let _ = machine(c, seed, 0x3eb1e0, 0);
                tables(c);
                c.pc = 0x003eb1e0;
                let rec = DATA;
                let len = if seed % 8 == 1 { 8 } else { 0x200 };
                c.g[A0][0] = if seed % 8 == 0 { 0 } else { sx(rec) };
                c.g[A1][0] = len as u64;
                let tmp = DATA + 0x400;
                if seed % 5 == 0 {
                    c.g[A2][0] = 0;
                } else {
                    c.g[A2][0] = sx(tmp);
                    let name = match seed % 4 {
                        0 => "abc",
                        1 => "x y",
                        2 => "",
                        _ => "k",
                    };
                    put_str(c, tmp, name.as_bytes());
                }
                // A record: a couple of "name=value" fields.
                put_str(c, rec, b"abc=1\nxy=2\n");
            },
        );
    }

    // 0x3eb6d8 TagFieldFind.
    #[test]
    fn tag_field_find_matches() {
        check(
            ee::s14::f_003eb6d8__generated,
            tag_field_find,
            96,
            &|c, seed| {
                let _ = machine(c, seed, 0x3eb6d8, 0);
                tables(c);
                c.pc = 0x003eb6d8;
                let rec = DATA;
                let name = DATA + 0x200;
                c.g[A0][0] = if seed % 7 == 0 { 0 } else { sx(rec) };
                c.g[A1][0] = if seed % 11 == 0 { 0 } else { sx(name) };
                put_str(c, rec, b"alpha=1\nbeta=2\n");
                put_str(
                    c,
                    name,
                    match seed % 4 {
                        0 => b"alpha",
                        1 => b"beta",
                        2 => b"gamma",
                        _ => b"",
                    },
                );
            },
        );
    }

    // 0x3eb840 TagFieldFind2.
    #[test]
    fn tag_field_find_2_matches() {
        check(
            ee::s14::f_003eb840__generated,
            tag_field_find_2,
            64,
            &|c, seed| {
                let _ = machine(c, seed, 0x3eb840, 0);
                tables(c);
                c.pc = 0x003eb840;
                let rec = DATA;
                let prefix = DATA + 0x200;
                let name = DATA + 0x400;
                c.g[A0][0] = sx(rec);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(prefix) };
                c.g[A2][0] = if seed % 5 == 0 { 0 } else { sx(name) };
                put_str(c, rec, b"abc=1\nab=2\n");
                put_str(c, prefix, b"a");
                put_str(c, name, b"bc");
            },
        );
    }

    // 0x3eb8b8 TagFieldFindSkipWs.
    #[test]
    fn tag_field_find_skip_ws_matches() {
        check(
            ee::s14::f_003eb8b8__generated,
            tag_field_find_skip_ws,
            48,
            &|c, seed| {
                let _ = machine(c, seed, 0x3eb8b8, 0);
                tables(c);
                c.pc = 0x003eb8b8;
                let rec = DATA;
                c.g[A0][0] = sx(rec);
                put_str(
                    c,
                    rec,
                    match seed % 3 {
                        0 => b"tag=  value\n",
                        1 => b"tag=value\n",
                        _ => b"nope\n",
                    },
                );
            },
        );
    }

    // 0x3eb918 TagFieldDelete.
    #[test]
    fn tag_field_delete_matches() {
        check(
            ee::s14::f_003eb918__generated,
            tag_field_delete,
            64,
            &|c, seed| {
                let _ = machine(c, seed, 0x3eb918, 0);
                tables(c);
                c.pc = 0x003eb918;
                let rec = DATA;
                let name = DATA + 0x200;
                c.g[A0][0] = sx(rec);
                c.g[A1][0] = sx(name);
                put_str(c, rec, b"aa=1\nbb=2\n");
                put_str(
                    c,
                    name,
                    match seed % 3 {
                        0 => b"aa",
                        1 => b"bb",
                        _ => b"cc",
                    },
                );
            },
        );
    }

    // 0x3eb9e8 TagFieldRename.
    #[test]
    fn tag_field_rename_matches() {
        check(
            ee::s14::f_003eb9e8__generated,
            tag_field_rename,
            64,
            &|c, seed| {
                let _ = machine(c, seed, 0x3eb9e8, 0);
                tables(c);
                c.pc = 0x003eb9e8;
                let rec = DATA;
                let name = DATA + 0x200;
                let val = DATA + 0x400;
                c.g[A0][0] = sx(rec);
                c.g[A1][0] = if seed % 3 == 1 { 4 } else { 0x100 };
                c.g[A2][0] = sx(name);
                c.g[A3][0] = sx(val);
                put_str(c, rec, b"aa=1\nbb=2\n");
                put_str(c, name, if seed % 3 == 0 { b"zz" } else { b"aa" });
                put_str(c, val, b"hello");
            },
        );
    }

    // 0x3ebfc8 TagFieldSetAddress.
    #[test]
    fn tag_field_set_address_matches() {
        check(
            ee::s14::f_003ebfc8__generated,
            tag_field_set_address,
            48,
            &|c, seed| {
                let _ = machine(c, seed, 0x3ebfc8, 0);
                tables(c);
                c.pc = 0x003ebfc8;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0x100;
                c.g[A2][0] = sx(DATA + 0x200);
                c.g[A3][0] = [0u64, 0x7f000001, 0xffffffff, 0x0a00000a][seed as usize % 4];
                put_str(c, DATA + 0x200, b"ip");
                put_str(c, DATA, b"");
            },
        );
    }

    // 0x3ec180 TagFieldSetString.
    #[test]
    fn tag_field_set_string_matches() {
        check(
            ee::s14::f_003ec180__generated,
            tag_field_set_string,
            48,
            &|c, seed| {
                let _ = machine(c, seed, 0x3ec180, 0);
                tables(c);
                c.pc = 0x003ec180;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 6 == 1 { 8 } else { 0x200 };
                c.g[A2][0] = sx(DATA + 0x200);
                c.g[A3][0] = if seed % 5 == 0 { 0 } else { sx(DATA + 0x400) };
                put_str(c, DATA + 0x200, b"key");
                put_str(
                    c,
                    DATA + 0x400,
                    match seed % 4 {
                        0 => b"plain",
                        1 => b"two words",
                        2 => b"%41%42",
                        _ => b"",
                    },
                );
                put_str(c, DATA, b"");
            },
        );
    }

    // 0x3ec3c0 TagFieldSetStructure.
    #[test]
    fn tag_field_set_structure_matches() {
        check(
            ee::s14::f_003ec3c0__generated,
            tag_field_set_structure,
            32,
            &|c, seed| {
                let _ = machine(c, seed, 0x3ec3c0, 0);
                tables(c);
                c.pc = 0x003ec3c0;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0x200;
                c.g[A2][0] = sx(DATA + 0x200);
                c.g[A3][0] = sx(DATA + 0x400);
                c.g[T0][0] = 0x40;
                c.g[T1][0] = sx(DATA + 0x600);
                put_str(c, DATA + 0x200, b"struct");
                put_str(
                    c,
                    DATA + 0x600,
                    match seed % 4 {
                        0 => b"#4b4s4",
                        1 => b"#2w2l4",
                        2 => b"#1s8",
                        _ => b"4b4s4",
                    },
                );
                put_str(c, DATA, b"");
            },
        );
    }

    // 0x3ec6d0 TagFieldSetEncoded.
    #[test]
    fn tag_field_set_encoded_matches() {
        check(
            ee::s14::f_003ec6d0__generated,
            tag_field_set_encoded,
            32,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3ec6d0, 0);
                tables(c);
                c.pc = 0x003ec6d0;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0x200;
                c.g[A2][0] = sx(DATA + 0x200);
                c.g[A3][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x400) };
                c.g[T0][0] = (1 + seed % 6) as u64;
                put_str(c, DATA + 0x200, b"crypt");
                for i in 0..6u32 {
                    c.bus
                        .write8(DATA + 0x400 + i, (b'a' + (r.next_u32() % 26) as u8) as u8);
                }
                put_str(c, DATA, b"");
            },
        );
    }

    // 0x3ec7e0 TagFieldSetDate.
    #[test]
    fn tag_field_set_date_matches() {
        check(
            ee::s14::f_003ec7e0__generated,
            tag_field_set_date,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x3ec7e0, 0);
                tables(c);
                c.pc = 0x003ec7e0;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0x200;
                c.g[A2][0] = sx(DATA + 0x200);
                c.g[A3][0] = [0u64, 1, 1000000000, 2000000000][seed as usize % 4];
                put_str(c, DATA + 0x200, b"date");
                put_str(c, DATA, b"");
            },
        );
    }

    // 0x3ec8b8 TagFieldSetDateYMDHMS.
    #[test]
    fn tag_field_set_date_ymdhms_matches() {
        check(
            ee::s14::f_003ec8b8__generated,
            tag_field_set_date_ymdhms,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x3ec8b8, 0);
                tables(c);
                c.pc = 0x003ec8b8;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0x200;
                c.g[A2][0] = sx(DATA + 0x200);
                c.g[A3][0] = (2020 + seed % 5) as u64;
                c.g[T0][0] = (1 + seed % 12) as u64;
                c.g[T1][0] = (1 + seed % 28) as u64;
                c.g[T2][0] = (seed % 24) as u64;
                c.g[T3][0] = (seed % 60) as u64;
                put_str(c, DATA + 0x200, b"ymd");
                put_str(c, DATA, b"");
            },
        );
    }

    // 0x3ec9a0 TagFieldSetDatePacked.
    #[test]
    fn tag_field_set_date_packed_matches() {
        check(
            ee::s14::f_003ec9a0__generated,
            tag_field_set_date_packed,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x3ec9a0, 0);
                tables(c);
                c.pc = 0x003ec9a0;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0x200;
                c.g[A2][0] = sx(DATA + 0x200);
                c.g[A3][0] =
                    ((2020 + seed % 5) << 16 | (1 + seed % 12) << 8 | (1 + seed % 28)) as u64;
                c.g[T0][0] = ((seed % 24) << 16 | (seed % 60) << 8 | (seed % 60)) as u64;
                c.g[T1][0] = 0;
                put_str(c, DATA + 0x200, b"packed");
                put_str(c, DATA, b"");
            },
        );
    }

    // 0x3eca88 TagFieldCopyField.
    #[test]
    fn tag_field_copy_field_matches() {
        check(
            ee::s14::f_003eca88__generated,
            tag_field_copy_field,
            48,
            &|c, seed| {
                let _ = machine(c, seed, 0x3eca88, 0);
                tables(c);
                c.pc = 0x003eca88;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0x200;
                c.g[A2][0] = sx(DATA + 0x200);
                c.g[A3][0] = sx(DATA + 0x400);
                put_str(c, DATA + 0x200, if seed % 2 == 0 { b"zz" } else { b"key" });
                put_str(c, DATA + 0x400, b"key=value\nother=x\n");
                put_str(c, DATA, b"");
            },
        );
    }

    // 0x3ecbd8 TagFieldPrintf.
    #[test]
    fn tag_field_printf_matches() {
        check(
            ee::s14::f_003ecbd8__generated,
            tag_field_printf,
            24,
            &|c, seed| {
                let _ = machine(c, seed, 0x3ecbd8, 0);
                tables(c);
                c.pc = 0x003ecbd8;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0x400;
                c.g[A2][0] = sx(DATA + 0x200);
                c.g[A3][0] = sx(DATA + 0x400);
                put_str(
                    c,
                    DATA + 0x200,
                    match seed % 3 {
                        0 => b"%s=%d",
                        1 => b"a b",
                        _ => b"x%sy",
                    },
                );
                put_str(c, DATA + 0x400, b"v");
                put_str(c, DATA, b"");
            },
        );
    }

    // 0x3ed0f8 TagFieldGetDelim.
    #[test]
    fn tag_field_get_delim_matches() {
        check(
            ee::s14::f_003ed0f8__generated,
            tag_field_get_delim,
            48,
            &|c, seed| {
                let _ = machine(c, seed, 0x3ed0f8, 0);
                tables(c);
                c.pc = 0x003ed0f8;
                c.g[A0][0] = if seed % 6 == 0 { 0 } else { sx(DATA) };
                c.g[A1][0] = if seed % 7 == 0 { 0 } else { sx(DATA + 0x200) };
                c.g[A2][0] = 0x40;
                c.g[A3][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x400) };
                c.g[T0][0] = (seed % 3) as u64;
                c.g[T1][0] = b',' as u64;
                put_str(c, DATA, b"a,b,c");
                put_str(c, DATA + 0x400, b"default");
            },
        );
    }

    // 0x3ed378 TagFieldGetString.
    #[test]
    fn tag_field_get_string_matches() {
        check(
            ee::s14::f_003ed378__generated,
            tag_field_get_string,
            48,
            &|c, seed| {
                let _ = machine(c, seed, 0x3ed378, 0);
                tables(c);
                c.pc = 0x003ed378;
                c.g[A0][0] = if seed % 6 == 0 { 0 } else { sx(DATA) };
                c.g[A1][0] = if seed % 5 == 0 { 0 } else { sx(DATA + 0x200) };
                c.g[A2][0] = [0, 1, 8, 0x40][seed as usize % 4] as u64;
                c.g[A3][0] = if seed % 7 == 0 { 0 } else { sx(DATA + 0x400) };
                put_str(
                    c,
                    DATA,
                    match seed % 4 {
                        0 => b"plain",
                        1 => b"\"quoted\"",
                        2 => b"ab%43d",
                        _ => b"",
                    },
                );
                put_str(c, DATA + 0x400, b"default");
            },
        );
    }

    // 0x3ed678 TagFieldGetBinary.
    #[test]
    fn tag_field_get_binary_matches() {
        check(
            ee::s14::f_003ed678__generated,
            tag_field_get_binary,
            48,
            &|c, seed| {
                let _ = machine(c, seed, 0x3ed678, 0);
                tables(c);
                c.pc = 0x003ed678;
                c.g[A0][0] = if seed % 5 == 0 { 0 } else { sx(DATA) };
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x400) };
                c.g[A2][0] = [0, 1, 4, -1][seed as usize % 4] as i64 as u64;
                c.g[A3][0] = sx(DATA + 0x200);
                put_str(
                    c,
                    DATA + 0x200,
                    match seed % 3 {
                        0 => b"b2w2l4",
                        1 => b"l1w1",
                        _ => b"",
                    },
                );
                put_str(c, DATA, b"$4142");
            },
        );
    }

    // 0x3ed970 TagFieldGetEncoded.
    #[test]
    fn tag_field_get_encoded_matches() {
        check(
            ee::s14::f_003ed970__generated,
            tag_field_get_encoded,
            48,
            &|c, seed| {
                let _ = machine(c, seed, 0x3ed970, 0);
                tables(c);
                c.pc = 0x003ed970;
                c.g[A0][0] = if seed % 5 == 0 { 0 } else { sx(DATA) };
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x200) };
                c.g[A2][0] = 0x40;
                c.g[A3][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x400) };
                c.g[T0][0] = if seed % 6 == 0 { 0 } else { sx(DATA + 0x600) };
                put_str(
                    c,
                    DATA,
                    match seed % 3 {
                        0 => b"$4142",
                        1 => b"plain",
                        _ => b"",
                    },
                );
                put_str(c, DATA + 0x400, b"key");
                put_str(c, DATA + 0x600, b"default");
            },
        );
    }

    // 0x3edb08 TagFieldGetDate.
    #[test]
    fn tag_field_get_date_matches() {
        check(
            ee::s14::f_003edb08__generated,
            tag_field_get_date,
            32,
            &|c, seed| {
                let _ = machine(c, seed, 0x3edb08, 0);
                tables(c);
                c.pc = 0x003edb08;
                c.g[A0][0] = if seed % 6 == 0 { 0 } else { sx(DATA) };
                c.g[A1][0] = [0u64, 1, 1000, 0xffffffff][seed as usize % 4];
                put_str(
                    c,
                    DATA,
                    match seed % 4 {
                        0 => b"$00bcd000",
                        1 => b"1000000",
                        2 => b"2020-01-02 03:04:05",
                        _ => b"",
                    },
                );
            },
        );
    }

    // 0x3edd70 TagFieldGetDateYMDHMS.
    #[test]
    fn tag_field_get_date_ymdhms_matches() {
        check(
            ee::s14::f_003edd70__generated,
            tag_field_get_date_ymdhms,
            24,
            &|c, seed| {
                let _ = machine(c, seed, 0x3edd70, 0);
                tables(c);
                c.pc = 0x003edd70;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x200) };
                c.g[A2][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x210) };
                c.g[A3][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x220) };
                c.g[T0][0] = 0;
                c.g[T1][0] = 0;
                c.g[T2][0] = 0;
                c.g[T3][0] = (seed % 3600) as u64;
                put_str(c, DATA, b"1000000");
            },
        );
    }

    // 0x3ede80 TagFieldGetDatePacked.
    #[test]
    fn tag_field_get_date_packed_matches() {
        check(
            ee::s14::f_003ede80__generated,
            tag_field_get_date_packed,
            24,
            &|c, seed| {
                let _ = machine(c, seed, 0x3ede80, 0);
                tables(c);
                c.pc = 0x003ede80;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x200) };
                c.g[A2][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x210) };
                c.g[A3][0] = (seed % 3600) as u64;
                put_str(c, DATA, b"1000000");
            },
        );
    }
}
