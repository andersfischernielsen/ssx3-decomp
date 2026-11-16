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

    pub(crate) use crate::game::dirtysock::tagsunk::callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::fe::festatetrophyroom::callee::MEMSET;
    pub(crate) use crate::game::fe::festatetrophyroom::callee::SPRINTF;
    pub(crate) use crate::game::fe::ovstatehud::callee::STRCPY;
    pub(crate) use crate::game::lib::libc::callee::MEMCPY;
    pub(crate) use crate::game::lib::libc::callee::STRLEN;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;

    use crate::call::Callee;
    use ssx3_ee as ee;

    pub const TAG_FIELD_DELETE: Callee = Callee {
        addr: 0x003e_b918,
        f: ee::s14::TagFieldDelete,
    };
    pub const TAG_FIELD_SET_NUMBER: Callee = Callee {
        addr: 0x003e_be40,
        f: ee::s14::TagFieldSetNumber,
    };
    pub const TAG_FIELD_SET_FLAGS: Callee = Callee {
        addr: 0x003e_bf28,
        f: ee::s14::TagFieldSetFlags,
    };
    pub const TAG_FIELD_SET_ADDRESS: Callee = Callee {
        addr: 0x003e_bfc8,
        f: ee::s14::TagFieldSetAddress,
    };
    pub const TAG_FIELD_SET_STRING: Callee = Callee {
        addr: 0x003e_c180,
        f: ee::s14::TagFieldSetString,
    };
    pub const TAG_FIELD_SET_DATE: Callee = Callee {
        addr: 0x003e_c7e0,
        f: ee::s14::TagFieldSetDate,
    };
    pub const TAG_FIELD_GET_DATE: Callee = Callee {
        addr: 0x003e_db08,
        f: ee::s14::TagFieldGetDate,
    };
    pub const TAG_FIELD_PARSE_DECIMAL: Callee = Callee {
        addr: 0x003e_ae68,
        f: ee::s14::TagField_ParseDecimal,
    };
    pub const DS_SECSTOTIME: Callee = Callee {
        addr: 0x003e_aeb8,
        f: ee::s14::ds_secstotime,
    };
    pub const DS_TIMETOSECS: Callee = Callee {
        addr: 0x003e_b090,
        f: ee::s14::ds_timetosecs,
    };
    pub const DS_TIMEINSECS: Callee = Callee {
        addr: 0x003e_f088,
        f: ee::s14::ds_timeinsecs,
    };
    pub const MEMCMP: Callee = Callee {
        addr: 0x0041_5fc8,
        f: ee::s15::memcmp,
    };
    pub const MEMMOVE: Callee = Callee {
        addr: 0x0041_610c,
        f: ee::s15::memmove,
    };
    pub const STRCAT: Callee = Callee {
        addr: 0x0041_62d0,
        f: ee::s15::strcat,
    };
}

// ---------------------------------------------------------------------------
// Tag-record tables and globals (addresses from the reference's `D_` names).

/// Character-class table `D_004964C0`: class 1 = delimiter, >= 2 = name
/// chars (meaning inferred: values >= 2 act as per-character keys, since
/// the compare is `a == b`).
const CLASS_TABLE: u32 = 0x0049_64c0;
/// Byte-equivalence/fold table folded from `0x490000 + 0x63c0` in
/// `TagFieldFind` (name inferred: indexed by the raw byte, compared as
/// `tbl[name[i]] == tbl[record[i]]`).
const TAG_CHAR_CLASS: u32 = 0x0049_63c0;
/// Field-separator byte (`D_004A4A70`), at gp+0x1980.
const FIELD_SEP_GP: u32 = 0x1980;
/// `sprintf` format for `TagFieldSetDate` (`D_00496A40`).
const DATE_FMT: u32 = 0x0049_6a40;
/// Field-name string `D_004A4A78` looked up by `TagFieldFindSkipWs`.
const TAG_NAME_ADDR: u32 = 0x004a_4a78;
/// Empty field name `D_004A4A90` (also the null-data fallback pointer).
const EMPTY_NAME: u32 = 0x004a_4a90;
/// Default decode key `D_004A4A88` for a null/empty key.
const TAG_FALLBACK_KEY: u32 = 0x004a_4a88;
/// Byte -> high hex char table (`D_00495FC0`; encode).
const HEX_ENCODE_HI: u32 = 0x0049_5fc0;
/// Byte -> low hex char table (`D_004960C0`; encode).
const HEX_ENCODE_LO: u32 = 0x0049_60c0;
/// `%xx` decode table for the first hex digit (`D_004961C0`).
const HEX_DECODE_HI: u32 = 0x0049_61c0;
/// `%xx` decode table for the second hex digit (`D_004962C0`); also the
/// table `TagFieldGetBinary` uses for every hex digit of its `b`/`w`/`l`
/// values.
const HEX_DECODE_LO: u32 = 0x0049_62c0;

// `sDsTmK2` broken-down time fields (reference field order).
const TM_SEC: u32 = 0x00;
const TM_MIN: u32 = 0x04;
const TM_HOUR: u32 = 0x08;
const TM_MDAY: u32 = 0x0c;
const TM_MON: u32 = 0x10;
const TM_YEAR: u32 = 0x14;
const TM_ISDST: u32 = 0x20;
const TM_SIZE: u32 = 0x24;
/// Calendar year of `tm_year` 0.
const YEAR_BASE: u32 = 1900;
/// Unix epoch year (the zero point of the seconds counts).
const EPOCH_YEAR: u32 = 1970;
/// Valid years after the epoch: 1970..2099.
const EPOCH_YEARS: u64 = 130;

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
    c.cycles += 2;
    if c.g[A2][0] == 0u64 {
        c.cycles += 2;
        c.sb(c.g[A0][0] as u32, 0u64);
        c.cycles += 2;
        c.g[V0][0] = c.g[A1][0];
        abi::ret(c);
        return;
    }
    c.g[V0][0] = c.lb(c.g[A2][0] as u32);
    c.cycles += 2;
    let name_empty = c.g[V0][0] == 0u64;
    c.g[V1][0] = c.lbu(c.g[A2][0] as u32);
    if !name_empty {
        loop {
            c.cycles += 7;
            c.sb(c.g[A1][0] as u32, c.g[V1][0]);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = c.lb(c.g[A2][0] as u32);
            let more = c.g[V0][0] != 0u64;
            c.g[V1][0] = c.lbu(c.g[A2][0] as u32);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003e_b1b0);
            }
        }
    }
    c.cycles += 3;
    c.g[V0][0] = 0x3d;
    c.sb(c.g[A1][0] as u32, c.g[V0][0]);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    abi::ret(c);
}

/// `int TagFieldSetRaw(char* record, int len, const char* temp)` (0x3eb1e0):
/// add or replace the `name=value` term `temp` in the record, returning
/// the whole term's length, 0 for an identical term, or -1.
/// Names compare through the class table at
/// `0x4964c0`; fields are separated by the byte at `gp+0x1980`, and `~…`
/// terms are appended at the end.
pub fn tag_field_set_raw(c: &mut Cpu) {
    match c.pc {
        0x003e_b1e0 | 0x003e_b250 | 0x003e_b2e0 | 0x003e_b338 | 0x003e_b390 | 0x003e_b3c0
        | 0x003e_b450 | 0x003e_b488 => {}
        pc => {
            c.jump_out(pc, 0x003e_b1e0, 0x003e_b574);
            return;
        }
    }
    guest(c, |c| {
        let resume = c.pc;
        let mut pending = resume;
        let frame: Frame;
        let record_null: bool;
        if resume == 0x003e_b1e0 {
            c.cycles += 11;
            frame = Frame::open(c, 0x70);
            c.g[T2][0] = c.g[A0][0];
            frame.save(c, S5, 0x10);
            frame.save(c, S0, 0x60);
            c.g[S5][0] = c.g[A2][0];
            frame.save(c, S1, 0x50);
            frame.save(c, S2, 0x40);
            frame.save(c, S3, 0x30);
            frame.save(c, S4, 0x20);
            record_null = c.g[T2][0] == 0u64;
            frame.save_ra(c, 0);
        } else {
            // Resume at a loop head: frame already open on the stack.
            // The frame size must equal the size `Frame::open` uses on the entry arm.
            frame = Frame { size: 0x70 };
            record_null = false;
        }
        'out: {
            let mut mode_tail = false;
            let mut mode_tilde = false;
            if resume == 0x003e_b1e0 {
                if record_null {
                    // 0x3eb4c0: return -1.
                    c.cycles += 2;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    break 'out;
                }
                // 0x3eb20c
                c.cycles += 2;
                let temp_null = c.g[S5][0] == 0u64;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                if temp_null {
                    break 'out;
                }
                // 0x3eb214
                c.cycles += 5;
                c.g[V0][0] = c.lbu(c.g[S5][0] as u32);
                c.g[T1][0] = c.g[V0][0];
                c.g[V0][0] = (c.g[V0][0] < 0x21u64) as u64;
                let temp_blank = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                if temp_blank {
                    break 'out;
                }
                // 0x3eb228: '~' terms append at the end of the record.
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[T1][0] as u32) << 24);
                c.g[V1][0] = 0x7e;
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                let is_tilde = c.g[V0][0] == c.g[V1][0];
                c.g[S0][0] = c.g[T2][0];
                mode_tilde = is_tilde;
            } else if resume == 0x003e_b250 {
                mode_tilde = true;
            } else if resume == 0x003e_b450 || resume == 0x003e_b488 {
                mode_tail = true;
            }
            if !mode_tail && !mode_tilde {
                // Scan path: 0x3eb2c4 sets up the field scan (entry only;
                // resume heads already hold T3/T4/T5/T6 in registers).
                if resume == 0x003e_b1e0 {
                    c.cycles += 6;
                    // Kept for exactness: T6 scratch (consumer folded to CLASS_TABLE).
                    c.g[T6][0] = 0x0049_0000;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[T5][0] = 0x1;
                    c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[T4][0] = 0x2;
                }
                'scan: loop {
                    if pending == 0x003e_b3c0 {
                        // Resume at 0x3eb3c0 skip-loop head (+7 at loop top).
                        pending = 0;
                        loop {
                            c.cycles += 7;
                            c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                            c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                            if c.g[V0][0] != 0u64 {
                                break;
                            }
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            if back_edge_unwinds(c, 0x003e_b3c0) {
                                return Ok(());
                            }
                        }
                        continue 'scan;
                    }
                    if pending == 0x003e_b390 {
                        // Resume at 0x3eb390 end-loop head (+7 at loop top).
                        loop {
                            c.cycles += 7;
                            c.g[V0][0] = c.lbu(c.g[S1][0] as u32);
                            c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                            if c.g[V0][0] != 0u64 {
                                break;
                            }
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            if back_edge_unwinds(c, 0x003e_b390) {
                                return Ok(());
                            }
                        }
                        // 0x3eb3ac: step past a trailing separator, if any.
                        c.cycles += 4;
                        c.g[V1][0] = c.lbu(c.g[S1][0] as u32);
                        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        if c.g[V1][0] != 0 {
                            c.g[S1][0] = c.g[V0][0];
                        }
                        break 'scan;
                    }
                    if pending == 0x003e_b338 {
                        // Resume at 0x3eb338 compare-loop head (+10 at loop
                        // top); duplicated loop: sharing it would rerun the
                        // 0x3eb328 setup and change the entry charge/path.
                        pending = 0;
                        loop {
                            // 0x3eb338
                            c.cycles += 10;
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                            c.g[V1][0] = c.lb(c.g[A3][0] as u32);
                            c.g[A0][0] = c.lbu(c.g[V0][0] as u32);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.g[A2][0] = c.lbu(c.g[V1][0] as u32);
                            c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                            let differs = c.g[V1][0] != c.g[A2][0];
                            c.g[V0][0] = (c.g[V1][0] < 0x2u64) as u64;
                            if differs {
                                // 0x3eb368
                                c.cycles += 1;
                                c.g[V0][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                                break;
                            }
                            // 0x3eb360
                            c.cycles += 2;
                            if c.g[V0][0] != 0u64 {
                                // 0x3eb368
                                c.cycles += 1;
                                c.g[V0][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                                break;
                            }
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            if back_edge_unwinds(c, 0x003e_b338) {
                                return Ok(());
                            }
                        }
                        // Fall through to the common 0x3eb36c tail below:
                        // this arm is inside the scan loop by construction.
                    } else {
                        if pending == 0x003e_b2e0 {
                            // Resume at 0x3eb2e0: V0 already holds the
                            // 0x3eb3dc (*p < 0x21) flag; pay only the +2
                            // 0x3eb2e0 block and join the dispatch below.
                            c.cycles += 2;
                        } else {
                            // 0x3eb3dc: loop head; *p == 0 ends the record.
                            c.cycles += 3;
                            c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                            let nonzero = c.g[V0][0] != 0u64;
                            c.g[V0][0] = (c.g[V0][0] < 0x21u64) as u64;
                            if !nonzero {
                                // 0x3eb3e8: append the term at the end.
                                c.cycles += 2;
                                let at_start = c.g[S0][0] == c.g[T2][0];
                                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x8000)); // p[-1] idiom, kept for exactness
                                if at_start {
                                    // 0x3eb428, then 0x3eb42c.
                                    c.cycles += 1;
                                    c.g[T1][0] = c.lbu(c.g[S5][0] as u32);
                                    c.cycles += 2;
                                    c.g[S1][0] = c.g[S0][0];
                                    c.g[S4][0] = c.g[S0][0];
                                    break 'scan;
                                }
                                // 0x3eb3f0
                                c.cycles += 4;
                                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x7fff));
                                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[T1][0] = c.lbu(c.g[S5][0] as u32);
                                    // 0x3eb42c
                                    c.cycles += 2;
                                    c.g[S1][0] = c.g[S0][0];
                                    c.g[S4][0] = c.g[S0][0];
                                    break 'scan;
                                }
                                // 0x3eb400
                                c.cycles += 4;
                                c.g[V1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x7fff));
                                c.g[V0][0] = c.lb((c.g[GP][0] as u32).wrapping_add(FIELD_SEP_GP));
                                let prev_is_sep = c.g[V1][0] == c.g[V0][0];
                                c.g[A0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(FIELD_SEP_GP));
                                if prev_is_sep {
                                    // 0x3eb428, then 0x3eb42c.
                                    c.cycles += 1;
                                    c.g[T1][0] = c.lbu(c.g[S5][0] as u32);
                                    c.cycles += 2;
                                    c.g[S1][0] = c.g[S0][0];
                                    c.g[S4][0] = c.g[S0][0];
                                    break 'scan;
                                }
                                // 0x3eb410
                                c.cycles += 3;
                                c.g[V0][0] = (c.g[S0][0] < c.g[T3][0]) as u64;
                                if c.g[V0][0] == 0u64 {
                                    c.g[T1][0] = c.lbu(c.g[S5][0] as u32);
                                    // 0x3eb42c
                                    c.cycles += 2;
                                    c.g[S1][0] = c.g[S0][0];
                                    c.g[S4][0] = c.g[S0][0];
                                    break 'scan;
                                }
                                // 0x3eb41c: write the separator, then 0x3eb428/0x3eb42c.
                                c.cycles += 3;
                                c.sb(c.g[S0][0] as u32, c.g[A0][0]);
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.sb(c.g[S0][0] as u32, 0u64);
                                c.cycles += 1;
                                c.g[T1][0] = c.lbu(c.g[S5][0] as u32);
                                c.cycles += 2;
                                c.g[S1][0] = c.g[S0][0];
                                c.g[S4][0] = c.g[S0][0];
                                break 'scan;
                            }
                        }
                        if pending == 0x003e_b2e0 {
                            pending = 0;
                        } else {
                            if back_edge_unwinds(c, 0x003e_b2e0) {
                                return Ok(());
                            }
                            // 0x3eb2e0: whitespace skips one byte, else compare.
                            c.cycles += 2;
                        }
                        if c.g[V0][0] != 0u64 {
                            // 0x3eb2e8
                            c.cycles += 2;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            continue 'scan;
                        }
                        c.g[V1][0] = c.lbu(c.g[S0][0] as u32);
                        // 0x3eb2f0: a class-1 byte is a bare delimiter field.
                        c.cycles += 5;
                        c.g[A0][0] = sx(CLASS_TABLE);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                        let is_delim = c.g[V0][0] == c.g[T5][0];
                        c.g[V0][0] = sx((c.g[T1][0] as u32) << 24);
                        if is_delim {
                            // 0x3eb42c
                            c.cycles += 2;
                            c.g[S1][0] = c.g[S0][0];
                            c.g[S4][0] = c.g[S0][0];
                            break 'scan;
                        }
                        // 0x3eb304: compare the field name with `temp`.
                        c.cycles += 6;
                        c.g[V1][0] = c.lbu(c.g[V1][0] as u32);
                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[A2][0] = c.lbu(c.g[V0][0] as u32);
                        let first_diff = c.g[V1][0] != c.g[A2][0];
                        c.g[S3][0] = 0u64;
                        if !first_diff {
                            // 0x3eb31c
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[V1][0] < 0x2u64) as u64;
                            let below_two = c.g[V0][0] != 0u64;
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            if !below_two {
                                // 0x3eb328, then the 0x3eb338 compare loop.
                                c.cycles += 4;
                                c.g[T0][0] = c.g[A0][0];
                                c.g[A3][0] = c.g[S5][0];
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                loop {
                                    // 0x3eb338
                                    c.cycles += 10;
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                    c.g[V1][0] = c.lb(c.g[A3][0] as u32);
                                    c.g[A0][0] = c.lbu(c.g[V0][0] as u32);
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                    c.g[A2][0] = c.lbu(c.g[V1][0] as u32);
                                    c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                                    let differs = c.g[V1][0] != c.g[A2][0];
                                    c.g[V0][0] = (c.g[V1][0] < 0x2u64) as u64;
                                    if differs {
                                        // 0x3eb368
                                        c.cycles += 1;
                                        c.g[V0][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                                        break;
                                    }
                                    // 0x3eb360
                                    c.cycles += 2;
                                    if c.g[V0][0] != 0u64 {
                                        // 0x3eb368
                                        c.cycles += 1;
                                        c.g[V0][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                                        break;
                                    }
                                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                    if back_edge_unwinds(c, 0x003e_b338) {
                                        return Ok(());
                                    }
                                }
                            }
                        } else {
                            // 0x3eb368
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                        }
                    }
                    // 0x3eb36c: class sum 2 means the field matches.
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[T4][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        // 0x3eb3c0: skip the rest of this field.
                        loop {
                            c.cycles += 7;
                            c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                            c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                            if c.g[V0][0] != 0u64 {
                                break;
                            }
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            if back_edge_unwinds(c, 0x003e_b3c0) {
                                return Ok(());
                            }
                        }
                        continue 'scan;
                    }
                    // 0x3eb374: found; find the end of the old value.
                    c.cycles += 5;
                    c.g[S4][0] = c.g[S0][0];
                    c.g[V0][0] = c.lbu(c.g[S4][0] as u32);
                    c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                    let at_end = c.g[V0][0] != 0u64;
                    c.g[S1][0] = c.g[S0][0];
                    if !at_end {
                        // 0x3eb388, then the 0x3eb390 end scan.
                        c.cycles += 2;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        loop {
                            c.cycles += 7;
                            c.g[V0][0] = c.lbu(c.g[S1][0] as u32);
                            c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                            if c.g[V0][0] != 0u64 {
                                break;
                            }
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            if back_edge_unwinds(c, 0x003e_b390) {
                                return Ok(());
                            }
                        }
                    }
                    // 0x3eb3ac: step past a trailing separator, if any.
                    c.cycles += 4;
                    c.g[V1][0] = c.lbu(c.g[S1][0] as u32);
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if c.g[V1][0] != 0 {
                        c.g[S1][0] = c.g[V0][0];
                    }
                    break 'scan;
                }
            } else if mode_tilde {
                // '~' path: 0x3eb23c/0x3eb248/0x3eb250 walk to the end.
                if pending == 0x003e_b250 {
                    // Resume at 0x3eb250 walk-loop head (+7 at loop top).
                    loop {
                        // 0x3eb250
                        c.cycles += 7;
                        c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                        if c.g[V0][0] == 0u64 {
                            break;
                        }
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        if back_edge_unwinds(c, 0x003e_b250) {
                            return Ok(());
                        }
                    }
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                    if c.g[V0][0] != 0u64 {
                        // 0x3eb248
                        c.cycles += 2;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        loop {
                            // 0x3eb250
                            c.cycles += 7;
                            c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                            if c.g[V0][0] == 0u64 {
                                break;
                            }
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            if back_edge_unwinds(c, 0x003e_b250) {
                                return Ok(());
                            }
                        }
                    }
                }
                // 0x3eb26c: append a separator if there is room and need.
                c.cycles += 2;
                let at_start = c.g[S0][0] == c.g[T2][0];
                // p[-1] idiom (sub 0x8000 / add 0x7fff below), kept for exactness.
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x8000));
                if at_start {
                    // 0x3eb2b4, then 0x3eb2b8.
                    c.cycles += 1;
                    c.g[S1][0] = c.g[S0][0];
                    c.cycles += 1;
                    c.g[S4][0] = c.g[S0][0];
                } else {
                    // 0x3eb274
                    c.cycles += 4;
                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x7fff));
                    c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                    let prev_blank = c.g[V0][0] != 0u64;
                    c.g[S1][0] = c.g[S0][0];
                    if prev_blank {
                        // 0x3eb2b8
                        c.cycles += 1;
                        c.g[S4][0] = c.g[S0][0];
                    } else {
                        // 0x3eb284
                        c.cycles += 4;
                        c.g[V1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x7fff));
                        c.g[V0][0] = c.lb((c.g[GP][0] as u32).wrapping_add(FIELD_SEP_GP));
                        let prev_is_sep = c.g[V1][0] == c.g[V0][0];
                        c.g[A0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(FIELD_SEP_GP));
                        if prev_is_sep {
                            // 0x3eb2b8
                            c.cycles += 1;
                            c.g[S4][0] = c.g[S0][0];
                        } else {
                            // 0x3eb294
                            c.cycles += 5;
                            c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                            let no_room = c.g[V0][0] == 0u64;
                            c.g[S4][0] = c.g[S0][0];
                            if !no_room {
                                // 0x3eb2a8, then 0x3eb2b4/0x3eb2b8.
                                c.cycles += 3;
                                c.sb(c.g[S0][0] as u32, c.g[A0][0]);
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.sb(c.g[S0][0] as u32, 0u64);
                                c.cycles += 1;
                                c.g[S1][0] = c.g[S0][0];
                                c.cycles += 1;
                                c.g[S4][0] = c.g[S0][0];
                            }
                        }
                    }
                }
                // 0x3eb2bc
                c.cycles += 2;
                c.g[T1][0] = c.lbu(c.g[S5][0] as u32);
            }
            // Shared tail: 0x3eb434 measures the new term against the old.
            // Resume at 0x003e_b450/0x003e_b488 enters its loop directly
            // (loop-top charge is the head charge).
            let mut run_450 = false;
            let mut run_488 = false;
            if resume == 0x003e_b450 {
                run_450 = true;
            } else if resume == 0x003e_b488 {
                run_488 = true;
            } else {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[T1][0] as u32) << 24);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 32i64) as u64;
                let first_below_space = c.g[V0][0] != 0u64;
                c.g[S3][0] = 0u64;
                if !first_below_space {
                    // 0x3eb44c, then the 0x3eb450 term-length loop.
                    c.cycles += 1;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    run_450 = true;
                }
            }
            if run_450 {
                loop {
                    // 0x3eb450: counts the whole `name=value` term.
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S3][0] as u32));
                    c.g[V0][0] = c.lb(c.g[V1][0] as u32);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 32i64) as u64;
                    if c.g[V0][0] != 0u64 {
                        break;
                    }
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003e_b450) {
                        return Ok(());
                    }
                }
            }
            if resume != 0x003e_b488 {
                // 0x3eb46c: d = (whole name=value term length + 1) - old
                // length; find the end.
                c.cycles += 5;
                c.g[S0][0] = c.g[S1][0];
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                let at_nul = c.g[V0][0] == 0u64;
                c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                if !at_nul {
                    // 0x3eb480, then the 0x3eb488 record-end loop.
                    c.cycles += 2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    run_488 = true;
                }
            }
            if run_488 {
                loop {
                    // 0x3eb488
                    c.cycles += 7;
                    c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                    if c.g[V0][0] == 0u64 {
                        break;
                    }
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if back_edge_unwinds(c, 0x003e_b488) {
                        return Ok(());
                    }
                }
            }
            // 0x3eb4a4
            c.cycles += 2;
            let d_nonpos = (c.g[S2][0] as i64) <= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            if !d_nonpos {
                // 0x3eb4ac: growing must still fit in `len`.
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                let fits = c.g[V0][0] == 0u64;
                if !fits {
                    // 0x3eb4c0: return -1.
                    c.cycles += 2;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    break 'out;
                }
            }
            // 0x3eb4c8
            c.cycles += 2;
            let d_nonzero = c.g[S2][0] != 0u64;
            c.g[A0][0] = c.g[S4][0];
            if !d_nonzero {
                // 0x3eb4d0: same length; an identical term is a no-op.
                c.cycles += 5;
                c.g[A1][0] = c.g[S5][0];
                c.g[A2][0] = c.g[S3][0];
                callee::MEMCMP.call(c, 0x003e_b4dc)?;
                let identical = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0u64;
                if identical {
                    break 'out;
                }
            }
            // 0x3eb4e4
            c.cycles += 2;
            let d_nonpos2 = (c.g[S2][0] as i64) <= 0;
            c.g[A1][0] = c.g[S1][0];
            if !d_nonpos2 {
                // 0x3eb4ec: memmove(end + d, end, p - end)
                c.cycles += 3;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                callee::MEMMOVE.call(c, 0x003e_b4f8)?;
            }
            // 0x3eb4f8
            c.cycles += 2;
            let d_nonneg = (c.g[S2][0] as i64) >= 0;
            c.g[A0][0] = c.g[S4][0];
            if !d_nonneg {
                // 0x3eb500: memmove(start, start - d, p - (start - d))
                c.cycles += 4;
                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S2][0] as u32));
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                callee::MEMMOVE.call(c, 0x003e_b50c)?;
                c.g[A0][0] = c.g[S4][0];
            }
            // 0x3eb510: memcpy(start, temp, n), then terminate the term.
            c.cycles += 7;
            c.g[A1][0] = c.g[S5][0];
            c.g[A2][0] = c.g[S3][0];
            callee::MEMCPY.call(c, 0x003e_b51c)?;
            c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S3][0] as u32));
            c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
            let past_end = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.g[V1][0];
            if past_end {
                // 0x3eb544, then 0x3eb548.
                c.cycles += 1;
                c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(FIELD_SEP_GP));
                c.cycles += 1;
                c.sb((c.g[A0][0] as u32).wrapping_sub(0x1), c.g[V0][0]);
            } else {
                // 0x3eb52c: a newline separator keeps the byte, else NUL.
                c.cycles += 4;
                c.g[V1][0] = c.lb((c.g[GP][0] as u32).wrapping_add(FIELD_SEP_GP));
                c.g[V0][0] = 0xa;
                let sep_is_newline = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(FIELD_SEP_GP));
                if sep_is_newline {
                    // 0x3eb548
                    c.cycles += 1;
                    c.sb((c.g[A0][0] as u32).wrapping_sub(0x1), c.g[V0][0]);
                } else {
                    // 0x3eb53c
                    c.cycles += 2;
                    c.sb((c.g[A0][0] as u32).wrapping_sub(0x1), 0u64);
                }
            }
            // 0x3eb54c: return n - 1.
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
        }
        // 0x3eb550
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

/// `char* TagFieldFind(char* record, const char* name)` (0x3eb6d8): find the
/// `name=` field in the record and return the pointer to its value (after the
/// `=` or `:`, with one space after the delimiter also skipped), or 0 when
/// the record or the name is missing or not found.
pub fn tag_field_find(c: &mut Cpu) {
    const ENTRY: u32 = 0x003e_b6d8;
    const HEAD_NAME: u32 = 0x003e_b718;
    const HEAD_MATCH: u32 = 0x003e_b758;
    const HEAD_BACK: u32 = 0x003e_b798;
    const HEAD_SCAN: u32 = 0x003e_b7ec;
    const END: u32 = 0x003e_b83c;
    // The frame size must equal the size `Frame::open` uses on the entry arm.
    let frame = Frame { size: 0x10 };
    let mut resume = match c.pc {
        ENTRY => ENTRY,
        HEAD_NAME => HEAD_NAME,
        HEAD_MATCH => HEAD_MATCH,
        HEAD_BACK => HEAD_BACK,
        HEAD_SCAN => HEAD_SCAN,
        pc => {
            c.jump_out(pc, ENTRY, END);
            return;
        }
    };
    if resume == ENTRY {
        c.cycles += 5; // 0x3eb6d8
        Frame::open(c, 0x10);
        c.g[T5][0] = c.g[A0][0];
        frame.save(c, S0, 0);
        let record_missing = c.g[T5][0] == 0;
        c.g[T6][0] = c.g[A1][0];
        if record_missing {
            c.cycles += 1; // 0x3eb82c
            c.g[V0][0] = 0;
            c.cycles += 1; // 0x3eb830
            frame.restore(c, S0, 0);
            c.cycles += 2; // 0x3eb834
            frame.close(c);
            return;
        }
        c.cycles += 3; // 0x3eb6ec
        c.g[V0][0] = c.lbu(c.g[T5][0] as u32);
        let record_empty = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if record_empty {
            c.cycles += 1; // 0x3eb830
            frame.restore(c, S0, 0);
            c.cycles += 2; // 0x3eb834
            frame.close(c);
            return;
        }
        c.cycles += 2; // 0x3eb6f8
        let name_missing = c.g[T6][0] == 0;
        frame.restore(c, S0, 0);
        if name_missing {
            c.cycles += 2; // 0x3eb834
            frame.close(c);
            return;
        }
        c.cycles += 3; // 0x3eb700
        c.g[V0][0] = c.lbu(c.g[T6][0] as u32);
        let name_empty = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if name_empty {
            c.cycles += 2; // 0x3eb834
            frame.close(c);
            return;
        }
        c.cycles += 3; // 0x3eb70c: T4 walks to the end of `name`.
        c.g[T4][0] = c.g[T6][0];
        c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
        resume = HEAD_NAME;
    }
    if resume == HEAD_NAME {
        loop {
            // 0x3eb718
            c.cycles += 7;
            c.g[V0][0] = c.lbu(c.g[T4][0] as u32);
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
            if back_edge_unwinds(c, 0x003e_b718) {
                return;
            }
        }
        c.cycles += 8; // 0x3eb734
        c.g[V0][0] = 0x0049_0000;
        c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(1));
        c.g[S0][0] = c.g[V0][0];
        c.g[T7][0] = sx(TAG_CHAR_CLASS);
        c.g[T0][0] = c.g[T5][0];
        c.g[T9][0] = 0x3d; // '='
        c.g[T8][0] = 0x3a; // ':'
        resume = HEAD_SCAN;
    }
    'scan: loop {
        // A HEAD_BACK entry goes straight to the back-match loop; a
        // HEAD_MATCH entry skips the scan head to the match attempt.
        let mut enter_back = resume == HEAD_BACK;
        if !enter_back {
            if resume != HEAD_MATCH {
                c.cycles += 3; // 0x3eb7ec: next record position.
                c.g[V0][0] = c.lbu(c.g[T0][0] as u32);
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = 0;
                    c.cycles += 1; // 0x3eb830
                    frame.restore(c, S0, 0);
                    c.cycles += 2; // 0x3eb834
                    frame.close(c);
                    return;
                }
                c.cycles += 2; // 0x3eb7f8
                if c.g[V0][0] == c.g[T9][0] {
                    c.g[V0][0] = c.lbu((c.g[T0][0] as u32).wrapping_add(1));
                    // -> 0x3eb80c
                } else {
                    c.cycles += 2; // 0x3eb800
                    if c.g[V0][0] != c.g[T8][0] {
                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                        if back_edge_unwinds(c, 0x003e_b7ec) {
                            return;
                        }
                        resume = HEAD_SCAN;
                        continue 'scan;
                    }
                    c.cycles += 1; // 0x3eb808
                    c.g[V0][0] = c.lbu((c.g[T0][0] as u32).wrapping_add(1));
                    // -> 0x3eb80c
                }
                c.cycles += 4; // 0x3eb80c
                c.g[T1][0] = c.g[V0][0];
                c.g[V0][0] = (c.g[V0][0] < 0x20) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_sub(1));
                    if back_edge_unwinds(c, 0x003e_b758) {
                        return;
                    }
                    // -> match attempt at 0x3eb758
                } else {
                    c.cycles += 4; // 0x3eb81c
                    c.g[V0][0] = c.lbu((c.g[T0][0] as u32).wrapping_sub(1));
                    c.g[V0][0] = (c.g[V0][0] < 0x21) as u64;
                    if c.g[V0][0] != 0 {
                        c.cycles += 1; // 0x3eb82c
                        c.g[V0][0] = 0;
                        c.cycles += 1; // 0x3eb830
                        frame.restore(c, S0, 0);
                        c.cycles += 2; // 0x3eb834
                        frame.close(c);
                        return;
                    }
                    c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_sub(1));
                    if back_edge_unwinds(c, 0x003e_b758) {
                        return;
                    }
                    // -> match attempt at 0x3eb758
                }
            }
            // Match attempt (0x3eb758..): every non-return exit advances T0
            // and rescans from 0x3eb7ec via the tail at the bottom.
            c.cycles += 3; // 0x3eb758
            c.g[V0][0] = (c.g[A2][0] < c.g[T5][0]) as u64;
            let before_record = c.g[V0][0] != 0;
            c.g[A3][0] = c.g[T4][0];
            if !before_record {
                c.cycles += 8; // 0x3eb764
                c.g[A0][0] = c.lbu((c.g[T0][0] as u32).wrapping_sub(1));
                c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T7][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T7][0] as u32));
                c.g[A1][0] = c.lbu(c.g[A0][0] as u32);
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                let mismatch = c.g[A1][0] != c.g[V0][0];
                c.g[T3][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                if !mismatch {
                    c.cycles += 5; // 0x3eb784
                    c.g[V0][0] = c.g[T1][0] & 0xff;
                    c.g[T2][0] = sx(TAG_CHAR_CLASS);
                    c.g[T1][0] = sx((c.g[T0][0] as u32).wrapping_add(2));
                    c.g[V0][0] ^= 0x20;
                    if c.g[V0][0] != 0 {
                        c.g[T1][0] = c.g[T3][0];
                    }
                    enter_back = true;
                }
            }
        }
        if enter_back {
            'back: loop {
                c.cycles += 2; // 0x3eb798
                if c.g[A3][0] != c.g[T6][0] {
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                    // -> 0x3eb7bc
                } else {
                    c.cycles += 2; // 0x3eb7a0
                    let at_record = c.g[A2][0] == c.g[T5][0];
                    c.g[V0][0] = c.g[T1][0];
                    if at_record {
                        c.cycles += 1; // 0x3eb830
                        frame.restore(c, S0, 0);
                        c.cycles += 2; // 0x3eb834
                        frame.close(c);
                        return;
                    }
                    c.cycles += 4; // 0x3eb7a8
                    c.g[V0][0] = c.lbu((c.g[A2][0] as u32).wrapping_sub(1));
                    c.g[V0][0] = (c.g[V0][0] < 0x21) as u64;
                    let ws_before = c.g[V0][0] != 0;
                    c.g[V0][0] = c.g[T1][0];
                    if ws_before {
                        c.cycles += 1; // 0x3eb830
                        frame.restore(c, S0, 0);
                        c.cycles += 2; // 0x3eb834
                        frame.close(c);
                        return;
                    }
                    c.cycles += 1; // 0x3eb7b8
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                    // -> 0x3eb7bc
                }
                c.cycles += 3; // 0x3eb7bc
                c.g[V0][0] = (c.g[A2][0] < c.g[T5][0]) as u64;
                let before_record = c.g[V0][0] != 0;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
                if before_record {
                    break 'back; // -> 0x3eb7e8
                }
                c.cycles += 8; // 0x3eb7c8
                c.g[V1][0] = c.lbu(c.g[A2][0] as u32);
                c.g[A0][0] = c.lbu(c.g[A3][0] as u32);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                c.g[A1][0] = c.lbu(c.g[V1][0] as u32);
                c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                if c.g[A1][0] != c.g[V0][0] {
                    break 'back; // -> 0x3eb7e8
                }
                if back_edge_unwinds(c, 0x003e_b798) {
                    return;
                }
                // -> 0x3eb798
            }
        }
        c.cycles += 1; // 0x3eb7e8
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
        resume = HEAD_SCAN;
        // -> 0x3eb7ec
    }
}

/// `char* TagFieldFind2(char* record, const char* prefix, const char* name)`
/// (0x3eb840): find `name` (or `prefix`, or `prefix+name` concatenated) in the
/// record.
pub fn tag_field_find_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x120);
        frame.save(c, S1, 0x110);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0x100);
        let prefix_present = c.g[A1][0] != 0u64;
        c.g[S1][0] = c.g[A0][0];
        if !prefix_present {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            callee::TAG_FIELD_FIND.call(c, 0x003e_b864)?;
            frame.restore(c, S0, 0x120);
        } else {
            c.cycles += 2;
            if c.g[S0][0] == 0u64 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                callee::TAG_FIELD_FIND.call(c, 0x003e_b87c)?;
                frame.restore(c, S0, 0x120);
            } else {
                c.cycles += 9;
                c.g[A0][0] = c.g[SP][0];
                callee::STRCPY.call(c, 0x003e_b88c)?;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[SP][0];
                callee::STRCAT.call(c, 0x003e_b898)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[SP][0];
                callee::TAG_FIELD_FIND.call(c, 0x003e_b8a4)?;
                frame.restore(c, S0, 0x120);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `char* TagFieldFindSkipWs(char* record)` (0x3eb8b8): find the `tag` field
/// and skip the leading whitespace of its value; 0 when the value is empty
/// or the field is not found.
pub fn tag_field_find_skip_ws(c: &mut Cpu) {
    const ENTRY: u32 = 0x003e_b8b8;
    const HEAD: u32 = 0x003e_b8e0;
    const END: u32 = 0x003e_b914;
    // The frame size must equal the size `Frame::open` uses on the entry arm.
    let frame = Frame { size: 0x10 };
    let resume = match c.pc {
        ENTRY => ENTRY,
        HEAD => HEAD,
        pc => {
            c.jump_out(pc, ENTRY, END);
            return;
        }
    };
    guest(c, |c| {
        // Initialize, then common code: ENTRY opens the frame and calls
        // Find, and loads V0 and sets enter_loop only when Find returns
        // non-null; HEAD already has the frame and the saved RA, and V1
        // points at the next value byte, and it reloads V0 (0x3eb8e0)
        // and always sets enter_loop.
        let mut enter_loop = false;
        if resume == ENTRY {
            c.cycles += 8;
            Frame::open(c, 0x10);
            // kept for exactness: A1 is overwritten with TAG_NAME_ADDR below.
            c.g[A1][0] = 0x004a_0000;
            frame.save_ra(c, 0);
            c.g[A1][0] = sx(TAG_NAME_ADDR);
            callee::TAG_FIELD_FIND.call(c, 0x003e_b8cc)?;
            c.g[V1][0] = c.g[V0][0];
            let found = c.g[V1][0] != 0u64;
            frame.restore_ra(c, 0);
            if found {
                c.cycles += 2;
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                enter_loop = true;
            }
        } else {
            c.cycles += 1; // 0x3eb8e0: reload the byte at V1.
            c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
            enter_loop = true;
        }
        if enter_loop {
            loop {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if c.g[V0][0] == 0u64 {
                    break;
                }
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                if back_edge_unwinds(c, 0x003e_b8e0) {
                    return Ok(());
                }
                c.cycles += 1;
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
            }
            c.cycles += 3;
            c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
            if c.g[V0][0] == 0 {
                c.g[V1][0] = 0u64;
            }
            frame.restore_ra(c, 0);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[V1][0];
        frame.close(c);
        Ok(())
    })
}

/// `int TagFieldDelete(char* record, const char* name)` (0x3eb918): remove the
/// `name=` field from the record, returning 0 on success or -1 when absent.
pub fn tag_field_delete(c: &mut Cpu) {
    const ENTRY: u32 = 0x003e_b918;
    const HEAD_WALK: u32 = 0x003e_b940;
    const HEAD_VALUE: u32 = 0x003e_b960;
    const HEAD_SEP: u32 = 0x003e_b988;
    const HEAD_COPY: u32 = 0x003e_b9b0;
    const END: u32 = 0x003e_b9e4;
    // The frame size must equal the size `Frame::open` uses on the entry arm.
    let frame = Frame { size: 0x20 };
    let resume = match c.pc {
        ENTRY => ENTRY,
        HEAD_WALK => HEAD_WALK,
        HEAD_VALUE => HEAD_VALUE,
        HEAD_SEP => HEAD_SEP,
        HEAD_COPY => HEAD_COPY,
        pc => {
            c.jump_out(pc, ENTRY, END);
            return;
        }
    };
    guest(c, |c| {
        // Initialize, then common code: ENTRY opens the frame and calls
        // Find; a HEAD entry already has the frame, the saved registers
        // and the loop pointers in A0/S0/V1. Sections before the entry
        // head are skipped; a reload head runs its reload once first.
        let mut at_record = false;
        if resume == ENTRY {
            c.cycles += 8; // 0x3eb918
            Frame::open(c, 0x20);
            frame.save(c, S0, 0x10);
            frame.save_ra(c, 0);
            c.g[S0][0] = c.g[A0][0];
            callee::TAG_FIELD_FIND.call(c, 0x003e_b92c)?;
            c.g[V1][0] = c.g[V0][0];
            let found = c.g[V1][0] != 0;
            c.g[A0][0] = c.g[V1][0];
            if !found {
                c.cycles += 2; // 0x3eb938
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                c.cycles += 4; // 0x3eb9d4
                frame.restore(c, S0, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        if resume == ENTRY || resume == HEAD_WALK {
            // Walk A0 back over the field name (bytes >= 0x21) to its start.
            loop {
                // 0x3eb940
                c.cycles += 2;
                if c.g[A0][0] == c.g[S0][0] {
                    c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                    at_record = true;
                    break; // -> 0x3eb964 (V0 already loaded)
                }
                c.cycles += 4; // 0x3eb948
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_sub(1));
                c.g[V0][0] = (c.g[V0][0] < 0x21) as u64;
                if c.g[V0][0] != 0 {
                    break; // -> 0x3eb958
                }
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                if back_edge_unwinds(c, 0x003e_b940) {
                    return Ok(());
                }
                // -> 0x3eb940
            }
            if !at_record {
                c.cycles += 2; // 0x3eb958
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                // -> 0x3eb964
            }
        }
        if resume == HEAD_VALUE {
            c.cycles += 1; // 0x3eb960: reload the byte at V1.
            c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
            // -> 0x3eb964
        }
        if resume != HEAD_SEP && resume != HEAD_COPY {
            loop {
                // 0x3eb964: advance V1 while the byte is >= 0x20.
                c.cycles += 6;
                c.g[V0][0] = (c.g[V0][0] < 0x20) as u64;
                if c.g[V0][0] != 0 {
                    break; // -> 0x3eb97c
                }
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                if back_edge_unwinds(c, 0x003e_b960) {
                    return Ok(());
                }
                c.cycles += 1; // 0x3eb960
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                // -> 0x3eb964
            }
            c.cycles += 2; // 0x3eb97c
            c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
        }
        if resume == HEAD_SEP {
            c.cycles += 1; // 0x3eb988: reload the byte at V1.
            c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
            // -> 0x3eb98c
        }
        if resume != HEAD_COPY {
            loop {
                // 0x3eb98c: advance V1 while (byte - 1) < 0x1f.
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.g[V0][0] = (c.g[V0][0] < 0x1f) as u64;
                if c.g[V0][0] == 0 {
                    break; // -> 0x3eb9a4
                }
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                if back_edge_unwinds(c, 0x003e_b988) {
                    return Ok(());
                }
                c.cycles += 1; // 0x3eb988
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                // -> 0x3eb98c
            }
            c.cycles += 3; // 0x3eb9a4
            c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
        }
        if c.g[V0][0] == 0 {
            c.sb(c.g[A0][0] as u32, 0u64);
            // -> 0x3eb9d0
        } else {
            loop {
                // 0x3eb9b0: copy the tail down over the deleted field.
                c.cycles += 7;
                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                if !more {
                    break; // -> 0x3eb9cc
                }
                if back_edge_unwinds(c, 0x003e_b9b0) {
                    return Ok(());
                }
                // -> 0x3eb9b0
            }
            c.cycles += 1; // 0x3eb9cc
            c.sb(c.g[A0][0] as u32, 0u64);
            // -> 0x3eb9d0
        }
        c.cycles += 1; // 0x3eb9d0
        c.g[V0][0] = 0;
        c.cycles += 4; // 0x3eb9d4
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int TagFieldRename(char* record, int len, const char* name, const char* value)`
/// (0x003e_b9e8): rename the `name` field to `value` (the 4th argument is the
/// new name), shifting the record in place; 0 on success, -1 when the field
/// or room is missing. On a shrink with a non-empty tail the original loops
/// forever; this port reproduces that.
pub fn tag_field_rename(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15; // 0x003e_b9e8
        let frame = Frame::open(c, 0x60);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S1, 0x40);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        callee::TAG_FIELD_FIND.call(c, 0x003e_ba18)?;
        c.g[S0][0] = c.g[V0][0];
        let missing = c.g[S0][0] == 0;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if !missing {
            c.cycles += 7; // 0x003e_ba24: back over a preceding space.
            c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_sub(1));
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
            c.g[V0][0] ^= 0x20;
            if c.g[V0][0] == 0 {
                c.g[S0][0] = c.g[V1][0];
            }
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
            c.g[S0][0] = c.g[S1][0];
            // Walk S1 back over the field name to its start (0x003e_ba40).
            loop {
                c.cycles += 2; // 0x003e_ba40
                if c.g[S1][0] == c.g[S3][0] {
                    break;
                }
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_sub(1));
                c.g[V0][0] = (c.g[V0][0] < 0x21) as u64;
                if c.g[V0][0] != 0 {
                    break;
                }
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                if event_due(c) {
                    deliver_event_here(c, 0x003e_ba40);
                }
                // -> 0x003e_ba40
            }
            c.cycles += 6; // 0x003e_ba58
            c.g[A0][0] = c.g[S2][0];
            callee::STRLEN.call(c, 0x003e_ba60)?;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[S1][0] as u32));
            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            let shrink = (c.g[A3][0] as i64) < 0;
            if shrink {
                c.cycles += 3; // 0x003e_ba70
                c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                if c.g[V0][0] == 0 {
                    c.sb(c.g[S0][0] as u32, 0u64);
                    // -> 0x003e_bb14
                } else {
                    c.cycles += 3;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.sb(c.g[V1][0] as u32, c.g[V0][0]);
                    loop {
                        // 0x003e_ba88: neither S0 nor V1 advances, so a non-empty tail loops forever (see doc).
                        c.cycles += 7;
                        c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                        if c.g[V0][0] == 0 {
                            c.cycles += 1;
                            c.sb(c.g[S0][0] as u32, 0u64);
                            break; // -> 0x003e_bb14
                        }
                        c.sb(c.g[V1][0] as u32, c.g[V0][0]);
                        if event_due(c) {
                            deliver_event_here(c, 0x003e_ba88);
                        }
                        // -> 0x003e_ba88
                    }
                }
            }
            // Grow path / room check (0x003e_baa8 head).
            c.cycles += 2;
            let no_grow = (c.g[A3][0] as i64) <= 0;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[A3][0] as u32));
            let mut room_fail = false;
            if no_grow {
                c.cycles += 1; // 0x003e_bb14 tail
                c.g[V0][0] = c.lb(c.g[S2][0] as u32);
                // -> copy at 0x003e_bb3c
            } else {
                // Find the end of the record (0x003e_bab0).
                c.cycles += 3;
                c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                let at_end = c.g[V0][0] == 0;
                c.g[A2][0] = c.g[S0][0];
                if !at_end {
                    c.cycles += 1;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    loop {
                        // 0x003e_bac0
                        c.cycles += 7;
                        c.g[V0][0] = c.lbu(c.g[A2][0] as u32);
                        if c.g[V0][0] == 0 {
                            break; // -> 0x003e_badc exit
                        }
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                        if event_due(c) {
                            deliver_event_here(c, 0x003e_bac0);
                        }
                        // -> 0x003e_bac0
                    }
                }
                c.cycles += 4; // 0x003e_badc: room check.
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[S3][0] as u32));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let no_room = c.g[V0][0] != 0;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                if no_room {
                    room_fail = true; // -> epilogue with V0 = -1
                } else if c.g[A2][0] == c.g[S0][0] {
                    c.cycles += 2; // 0x003e_baec
                    c.g[V0][0] = c.lb(c.g[S2][0] as u32);
                    // -> copy at 0x003e_bb3c
                } else {
                    c.cycles += 2; // 0x003e_baec
                    c.cycles += 1; // 0x003e_baf4 head
                    loop {
                        // 0x003e_baf8: shift the tail up by delta, backwards.
                        c.cycles += 7;
                        c.g[V1][0] = c.lbu(c.g[A2][0] as u32);
                        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A3][0] as u32));
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                        let more = c.g[A2][0] != c.g[S0][0];
                        c.sb(c.g[V0][0] as u32, c.g[V1][0]);
                        if !more {
                            break; // -> 0x003e_bb14 tail
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003e_baf8);
                        }
                        // -> 0x003e_baf8
                    }
                    c.cycles += 1; // 0x003e_bb14 tail
                    c.g[V0][0] = c.lb(c.g[S2][0] as u32);
                    // -> copy at 0x003e_bb3c
                }
            }
            if !room_fail {
                // Copy the new value in (0x003e_bb18).
                c.cycles += 2;
                let empty = c.g[V0][0] == 0;
                c.g[V1][0] = c.lbu(c.g[S2][0] as u32);
                if !empty {
                    loop {
                        // 0x003e_bb20
                        c.cycles += 7;
                        c.sb(c.g[S1][0] as u32, c.g[V1][0]);
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        c.g[V0][0] = c.lb(c.g[S2][0] as u32);
                        let more = c.g[V0][0] != 0;
                        c.g[V1][0] = c.lbu(c.g[S2][0] as u32);
                        if !more {
                            break; // -> 0x003e_bb3c exit
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003e_bb20);
                        }
                        // -> 0x003e_bb20
                    }
                }
                c.cycles += 1; // 0x003e_bb3c
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 8; // 0x003e_bb40 epilogue
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

/// `int TagFieldSetAddress(char* record, int len, const char* name, unsigned int addr)`
/// (0x003e_bfc8): store an IPv4 address field in dotted-decimal form.
pub fn tag_field_set_address(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x180);
        frame.save(c, S1, 0x160);
        frame.save(c, S2, 0x150);
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        frame.save(c, S3, 0x140);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x170);
        c.g[S3][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A3][0];
        frame.save_ra(c, 0x130);
        c.g[A1][0] = c.g[S1][0];
        callee::TAG_FIELD_COPY_TAG.call(c, 0x003e_bff8)?;
        c.sb((c.g[SP][0] as u32).wrapping_add(0x3), c.g[S0][0]);
        c.g[A2][0] = c.g[S1][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32) >> 8);
        c.g[T2][0] = b'.' as u64;
        c.sb((c.g[SP][0] as u32).wrapping_add(0x2), c.g[S0][0]);
        c.g[T1][0] = 0x64; // 100
        c.g[S0][0] = sx((c.g[S0][0] as u32) >> 8);
        c.g[T0][0] = 0u64;
        c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[S0][0]);
        c.g[A3][0] = 0xa; // 10
        c.g[S0][0] = sx((c.g[S0][0] as u32) >> 8);
        c.g[A0][0] = c.g[V0][0];
        c.sb(c.g[SP][0] as u32, c.g[S0][0]);
        c.g[A1][0] = 0u64;
        // Emit the four stack bytes (SP+0..SP+3) as dotted decimal.
        loop {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[A1][0] as u32));
            let first = (c.g[A1][0] as i64) <= 0;
            c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
            if !first {
                c.cycles += 2;
                c.sb(c.g[A0][0] as u32, c.g[T2][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
            let single_digit = c.g[V0][0] != 0u64;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(b'0' as u32));
            if !single_digit {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V1][0] < 0x64u64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.divu(0, 3, 7); // /10 (divisor in A3): quotient -> lo, remainder -> hi
                } else {
                    c.cycles += 3;
                    c.divu(0, 3, 9); // /100 (divisor in T1): quotient -> lo, remainder -> hi
                    c.cycles += 7;
                    c.g[V0][0] = c.lo[0];
                    c.g[V1][0] = c.hi[0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(b'0' as u32));
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                    c.cycles += 2;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.divu(0, 3, 7); // /10 (divisor in A3): quotient -> lo, remainder -> hi
                }
                // Kept split, not merged to += 9: +2 is the 0x003e_c094 fall-through and +7 is
                // 0x003e_c0b0; both follow the divu arms above, and keeping them split keeps each
                // generated block's charge visible before the lo/hi read that follows.
                c.cycles += 2;
                c.cycles += 7;
                c.g[V0][0] = c.lo[0];
                c.g[V1][0] = c.hi[0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(b'0' as u32));
                c.g[V1][0] = c.g[V1][0] & 0xff;
                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                c.cycles += 2;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(b'0' as u32));
            }
            c.cycles += 5;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.sb(c.g[A0][0] as u32, c.g[V0][0]);
            c.g[V1][0] = ((c.g[A1][0] as i64) < 4i64) as u64;
            let more = c.g[V1][0] != 0u64;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003e_c030);
                }
                continue;
            }
            c.cycles += 11;
            c.sb(c.g[A0][0] as u32, 0u64);
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.g[S2][0];
            callee::TAG_FIELD_SET_RAW.call(c, 0x003e_c0dc)?;
            frame.restore(c, S0, 0x170);
            frame.restore(c, S1, 0x160);
            frame.restore(c, S2, 0x150);
            frame.restore(c, S3, 0x140);
            frame.restore_ra(c, 0x130);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `int TagFieldSetString(char* record, int len, const char* name, const char* value)`
/// (0x003e_c180): store a string value, quoting it when it contains a space
/// byte (0x20); a null value deletes the field. `=`, `"`, `:` and `%` itself
/// (and any byte outside 0x20..=0x7e) are written as `%` followed by the two
/// hex encode-table chars of the byte, whether or not the value is quoted.
pub fn tag_field_set_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0xa40);
        frame.save(c, S2, 0xa10);
        frame.save(c, S0, 0xa30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0xa20);
        c.g[A1][0] = c.g[A2][0];
        frame.save_ra(c, 0xa00);
        c.g[S0][0] = c.g[A3][0];
        let has_value = c.g[S0][0] != 0u64;
        c.g[S1][0] = c.g[A0][0];
        if !has_value {
            c.cycles += 4;
            callee::TAG_FIELD_DELETE.call(c, 0x003e_c1b0)?;
            c.g[V0][0] = 0u64;
        } else {
            c.cycles += 8;
            c.g[A2][0] = c.g[A1][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[SP][0];
            callee::TAG_FIELD_COPY_TAG.call(c, 0x003e_c1c8)?;
            c.g[A2][0] = c.g[S0][0];
            c.g[V1][0] = c.lbu(c.g[A2][0] as u32);
            let empty = c.g[V1][0] == 0u64;
            c.g[A0][0] = c.g[V0][0];
            // Decide whether the value needs quoting (any space in it).
            let mut open_quote = false;
            if empty {
                c.cycles += 3;
                c.g[V0][0] = b' ' as u64;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A1][0] = c.lbu(c.g[S0][0] as u32);
                } else {
                    open_quote = true;
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = b' ' as u64;
                let first_is_space = c.g[V1][0] == c.g[V0][0];
                c.g[V1][0] = b' ' as u64;
                if first_is_space {
                    open_quote = true;
                } else {
                    c.cycles += 1;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    loop {
                        c.cycles += 3;
                        c.g[V0][0] = c.lbu(c.g[A2][0] as u32);
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lbu(c.g[A2][0] as u32);
                            break;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[V1][0] {
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            if event_due(c) {
                                deliver_event_here(c, 0x003e_c1e8);
                            }
                            continue;
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lbu(c.g[A2][0] as u32);
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = b' ' as u64;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A1][0] = c.lbu(c.g[S0][0] as u32);
                    } else {
                        open_quote = true;
                    }
                }
            }
            if open_quote {
                c.cycles += 4;
                c.g[V0][0] = b'"' as u64;
                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = c.lbu(c.g[S0][0] as u32);
            }
            // Copy the value, %-escaping the special chars.
            c.cycles += 2;
            let value_done = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0x0049_0000;
            if !value_done {
                c.cycles += 9;
                c.g[V1][0] = 0x0049_0000;
                c.g[T5][0] = sx(HEX_ENCODE_HI);
                c.g[T4][0] = sx(HEX_ENCODE_LO);
                c.g[T3][0] = b'=' as u64;
                c.g[T2][0] = b'"' as u64;
                c.g[T1][0] = b':' as u64;
                c.g[T0][0] = b'%' as u64;
                c.g[A3][0] = b'%' as u64;
                loop {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(b' ' as u32));
                    c.g[V0][0] = (c.g[V0][0] < 0x5fu64) as u64;
                    let out_of_range = c.g[V0][0] == 0u64; // (byte - b' ') >= 0x5f, i.e. outside 0x20..=0x7e
                    c.g[V0][0] = c.g[A1][0] & 0xff;
                    let mut hex_encode = false;
                    let mut shared_percent = false;
                    if out_of_range {
                        shared_percent = true;
                        hex_encode = true;
                    } else {
                        c.cycles += 2;
                        if c.g[V0][0] == c.g[T3][0] {
                            c.sb(c.g[A0][0] as u32, c.g[A3][0]);
                            hex_encode = true;
                        } else {
                            c.cycles += 2;
                            if c.g[V0][0] == c.g[T2][0] {
                                c.sb(c.g[A0][0] as u32, c.g[A3][0]);
                                hex_encode = true;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] == c.g[T1][0] {
                                    c.sb(c.g[A0][0] as u32, c.g[A3][0]);
                                    hex_encode = true;
                                } else {
                                    c.cycles += 2;
                                    if c.g[V0][0] != c.g[T0][0] {
                                        c.sb(c.g[A0][0] as u32, c.g[A1][0]);
                                    } else {
                                        shared_percent = true;
                                        hex_encode = true;
                                    }
                                }
                            }
                        }
                    }
                    if shared_percent {
                        c.cycles += 1;
                        c.sb(c.g[A0][0] as u32, c.g[A3][0]);
                    }
                    if hex_encode {
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
                    }
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = c.lbu(c.g[S0][0] as u32);
                    let more = c.g[A1][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x003e_c248);
                        }
                        continue;
                    }
                    break;
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.lbu(c.g[A2][0] as u32);
            c.g[V0][0] = b' ' as u64;
            if c.g[V1][0] != c.g[V0][0] {
                c.sb(c.g[A0][0] as u32, 0u64);
            } else {
                c.cycles += 4;
                c.g[V0][0] = b'"' as u64;
                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.sb(c.g[A0][0] as u32, 0u64);
            }
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.g[SP][0];
            callee::TAG_FIELD_SET_RAW.call(c, 0x003e_c2e4)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0xa30);
        frame.restore(c, S1, 0xa20);
        frame.restore(c, S2, 0xa10);
        frame.restore_ra(c, 0xa00);
        frame.close(c);
        Ok(())
    })
}

/// `int TagFieldSetStructure(char* record, int len, const char* name, void* data, int size, const char* fmt)`
/// (0x3ec3c0): store a structure field by walking template `fmt` (`#...`,
/// a count, then `b`/`w`/`l`/`s`) and hex-encoding the matching data.
pub fn tag_field_set_structure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xa70);
        frame.save(c, S0, 0xa60);
        frame.save(c, S1, 0xa50);
        c.g[S0][0] = c.g[T1][0];
        frame.save(c, S2, 0xa40);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S4, 0xa20);
        c.g[S2][0] = 0;
        frame.save(c, S5, 0xa10);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S3, 0xa30);
        c.g[S5][0] = c.g[A1][0];
        let count_nonneg = (c.g[T0][0] as i64) >= 0;
        frame.save_ra(c, 0xa00);
        if count_nonneg {
            c.cycles += 1;
            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[T0][0] as u32));
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x10000;
            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        }
        // CopyTag writes "name=" into the stack buffer; A1 is its end.
        let mut filled = false;
        {
            c.cycles += 7;
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = c.g[S4][0];
            callee::TAG_FIELD_COPY_TAG.call(c, 0x003e_c414)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V1][0] = c.lb(c.g[S0][0] as u32);
            let template_empty = c.g[V1][0] == 0;
            c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
            if !template_empty {
                c.cycles += 3;
                c.g[T6][0] = b'a' as u64;
                c.g[T5][0] = b',' as u64;
                c.g[T4][0] = 0x0049_0000; // kept for exactness
                'tmpl: loop {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                    c.g[V1][0] = b'#' as u64;
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    if c.g[V0][0] == c.g[V1][0] {
                        // `b'#'` prefix: skip through the `b'='`; the decimal count is parsed after it.
                        c.cycles += 3;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        c.g[V1][0] = b'=' as u64;
                        loop {
                            c.cycles += 3;
                            c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                            if c.g[V0][0] == 0 {
                                c.g[A2][0] = c.lbu(c.g[S0][0] as u32);
                                break;
                            }
                            c.cycles += 2;
                            let more = c.g[V0][0] != c.g[V1][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            if more {
                                if event_due(c) {
                                    deliver_event_here(c, 0x003e_c450);
                                }
                                continue;
                            }
                            c.cycles += 1;
                            c.g[A2][0] = c.lbu(c.g[S0][0] as u32);
                            break;
                        }
                    } else {
                        c.g[A2][0] = c.lbu(c.g[S0][0] as u32);
                    }
                    // Optional decimal repeat count into T0.
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
                    let has_count = c.g[V0][0] != 0;
                    c.g[T0][0] = 0;
                    if has_count {
                        c.cycles += 2;
                        c.g[A3][0] = 0xa;
                        loop {
                            c.cycles += 8;
                            c.g[A0][0] = c.g[A2][0] & 0xf;
                            c.mult(0, 3, 8, 7);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            c.g[A2][0] = c.lbu(c.g[S0][0] as u32);
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x30));
                            c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
                            let more = c.g[V0][0] != 0;
                            c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            if more {
                                if event_due(c) {
                                    deliver_event_here(c, 0x003e_c480);
                                }
                                continue;
                            }
                            break;
                        }
                    }
                    // `b'a'` advances the data cursor without emitting.
                    c.cycles += 5;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    c.g[A2][0] = 0;
                    c.g[T2][0] = 0;
                    let not_a = c.g[V0][0] != c.g[T6][0];
                    c.g[V1][0] = c.lbu(c.g[S0][0] as u32);
                    if !not_a {
                        c.cycles += 2;
                        if c.g[T0][0] == 0 {
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        } else {
                            c.cycles += 1;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        }
                    }
                    // Type letter in A0: `b'b'`/`b'w'`/`b'l'` read 1/2/4 bytes, `b's'` a string.
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                    c.g[V1][0] = b'b' as u64;
                    c.g[A0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    let not_b = c.g[A0][0] != c.g[V1][0];
                    c.g[V0][0] = b'w' as u64;
                    if !not_b {
                        c.cycles += 3;
                        c.g[S2][0] = c.lbu(c.g[S1][0] as u32);
                        c.g[A2][0] = 0x2;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    }
                    c.cycles += 2;
                    let not_w = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = b'l' as u64;
                    if !not_w {
                        c.cycles += 3;
                        c.g[S2][0] = c.lhu(c.g[S1][0] as u32);
                        c.g[A2][0] = 0x4;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
                    }
                    c.cycles += 2;
                    let not_l = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = b's' as u64;
                    if !not_l {
                        c.cycles += 3;
                        c.g[S2][0] = c.lw(c.g[S1][0] as u32);
                        c.g[A2][0] = 0x8;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    }
                    c.cycles += 2;
                    let not_s = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = (c.g[S3][0] < c.g[S1][0]) as u64;
                    if !not_s {
                        c.cycles += 3;
                        c.g[T2][0] = c.g[S1][0];
                        c.g[S1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V0][0] = (c.g[S3][0] < c.g[S1][0]) as u64;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        // Data cursor passed the end of the data buffer (S3 = data + size): stop, V1 already reloaded.
                        c.g[V1][0] = c.lb((c.g[A1][0] as u32).wrapping_sub(0x1));
                        filled = true;
                        break 'tmpl;
                    }
                    // Emit the numeric value as lowercase hex digits, then a comma; a value that exactly fills its width writes no comma.
                    c.cycles += 2;
                    let has_digits = (c.g[A2][0] as i64) > 0;
                    c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if has_digits {
                        c.cycles += 4;
                        c.g[V0][0] = 0xffff_ffff_ffff_0000;
                        c.g[V0][0] = (c.g[V0][0] < c.g[S2][0]) as u64;
                        // Unsigned S2 > 0xffff_ffff_ffff_0000: only -65535..-1 get '-'.
                        let is_small_negative = c.g[V0][0] != 0;
                        c.g[V0][0] = b'-' as u64;
                        if is_small_negative {
                            c.cycles += 3;
                            c.g[S2][0] = sx(0u32.wrapping_sub(c.g[S2][0] as u32));
                            c.sb(c.g[A1][0] as u32, c.g[V0][0]);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        }
                        c.cycles += 4;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                        c.g[A3][0] = c.g[A0][0];
                        let zero_value = c.g[S2][0] == 0;
                        c.sb(c.g[A0][0] as u32, 0);
                        if !zero_value {
                            c.cycles += 2;
                            c.g[A2][0] = sx(HEX_ENCODE_LO);
                            loop {
                                c.cycles += 7;
                                c.g[V1][0] = c.g[S2][0] & 0xf;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                                c.g[S2][0] = sx((c.g[S2][0] as u32) >> 4);
                                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                                let more = c.g[S2][0] != 0;
                                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                                if more {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003e_c560);
                                    }
                                    continue;
                                }
                                break;
                            }
                        }
                        c.cycles += 2;
                        if c.g[A1][0] != c.g[A0][0] {
                            c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                            c.cycles += 2;
                            let digit_end = c.g[V0][0] == 0;
                            c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                            if !digit_end {
                                c.cycles += 1;
                                loop {
                                    c.cycles += 7;
                                    c.sb(c.g[A1][0] as u32, c.g[V1][0]);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                                    let more = c.g[V0][0] != 0;
                                    c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                                    if more {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003e_c598);
                                        }
                                        continue;
                                    }
                                    break;
                                }
                            }
                            c.cycles += 2;
                            c.sb(c.g[A1][0] as u32, c.g[T5][0]);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        } else {
                            c.cycles += 2;
                            c.g[A1][0] = c.g[A3][0];
                        }
                    }
                    // Emit the string value, %-escaping bytes whose class
                    // table entry is zero, then a comma.
                    c.cycles += 2;
                    let no_string = c.g[T2][0] == 0;
                    c.g[S0][0] = c.g[T1][0];
                    if !no_string {
                        c.cycles += 3;
                        c.g[V0][0] = c.lbu(c.g[T2][0] as u32);
                        let empty_string = c.g[V0][0] == 0;
                        c.g[V1][0] = 0x0049_0000;
                        if !empty_string {
                            c.cycles += 6;
                            c.g[V0][0] = 0x0049_0000;
                            c.g[T3][0] = sx(CLASS_TABLE);
                            c.g[T0][0] = sx(HEX_ENCODE_HI);
                            c.g[A3][0] = b'%' as u64;
                            c.g[A2][0] = sx(HEX_ENCODE_LO);
                            loop {
                                c.cycles += 5;
                                c.g[S2][0] = c.lbu(c.g[T2][0] as u32);
                                c.g[V0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add(c.g[T3][0] as u32));
                                c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                                let escape = c.g[V1][0] == 0;
                                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                                if escape {
                                    c.cycles += 9;
                                    c.g[V0][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                    c.sb(c.g[A1][0] as u32, c.g[A3][0]);
                                    c.g[A0][0] = c.lbu(c.g[V0][0] as u32);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add(c.g[A2][0] as u32));
                                    c.sb(c.g[A1][0] as u32, c.g[A0][0]);
                                    c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.sb(c.g[A1][0] as u32, c.g[V1][0]);
                                } else {
                                    c.cycles += 2;
                                    c.sb(c.g[A1][0] as u32, c.g[S2][0]);
                                }
                                c.cycles += 3;
                                c.g[V0][0] = c.lbu(c.g[T2][0] as u32);
                                let more = c.g[V0][0] != 0;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                if more {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003e_c5e8);
                                    }
                                    continue;
                                }
                                break;
                            }
                        }
                        c.cycles += 3;
                        c.sb(c.g[A1][0] as u32, c.g[T5][0]);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        c.g[S0][0] = c.g[T1][0];
                    }
                    // Next template char; a trailing `b'*'` steps back one.
                    c.cycles += 7;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = c.g[V0][0] ^ b'*' as u64;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[V1][0];
                    }
                    c.g[A0][0] = c.lb(c.g[S0][0] as u32);
                    let more = c.g[A0][0] != 0;
                    c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x003e_c430);
                        }
                        continue 'tmpl;
                    }
                    break 'tmpl;
                }
            }
        }
        if !filled {
            c.cycles += 1;
            c.g[V1][0] = c.lb((c.g[A1][0] as u32).wrapping_sub(0x1));
        }
        // Drop a trailing comma, terminate, and store the field.
        c.cycles += 3;
        c.g[V0][0] = b',' as u64;
        if c.g[V1][0] != c.g[V0][0] {
            c.sb(c.g[A1][0] as u32, 0);
        } else {
            c.cycles += 3;
            c.g[V1][0] = b',' as u64;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            loop {
                c.cycles += 7;
                c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_sub(0x1));
                if c.g[V0][0] == c.g[V1][0] {
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_c678);
                    }
                    continue;
                }
                c.cycles += 1;
                c.sb(c.g[A1][0] as u32, 0);
                break;
            }
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S5][0];
        c.g[A2][0] = c.g[SP][0];
        callee::TAG_FIELD_SET_RAW.call(c, 0x003e_c6a8)?;
        frame.restore(c, S0, 0xa60);
        frame.restore(c, S1, 0xa50);
        frame.restore(c, S2, 0xa40);
        frame.restore(c, S3, 0xa30);
        frame.restore(c, S4, 0xa20);
        frame.restore(c, S5, 0xa10);
        frame.restore_ra(c, 0xa00);
        frame.close(c);
        Ok(())
    })
}

/// `int TagFieldSetEncoded(char* record, int len, const char* name, const char* key, const char* str)`
/// (0x3ec6d0): store plaintext `str` encrypted (the `TagFieldSetCrypt`
/// encryption): the stored form starts with `$`; each byte is xor-chained
/// with the previous cipher byte, rotated, xored with the cycling key bytes,
/// then written as two hex encode-table chars. A null or empty `key` selects
/// the default key `D_004A4A88`; nothing deletes.
pub fn tag_field_set_encoded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x1160);
        frame.save(c, S2, 0x1130);
        frame.save(c, S0, 0x1150);
        c.g[S2][0] = c.g[T0][0];
        frame.save(c, S1, 0x1140);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S3, 0x1120);
        c.g[S1][0] = c.g[S2][0];
        frame.save(c, S4, 0x1110);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x1100);
        let key_is_null = c.g[S0][0] == 0u64;
        c.g[S4][0] = c.g[A1][0];
        let mut fallback = false;
        if key_is_null {
            fallback = true;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
            let key_nonempty = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.g[S3][0];
            if !key_nonempty {
                fallback = true;
            }
        }
        if fallback {
            c.cycles += 3;
            c.g[V0][0] = 0x004a_0000;
            c.g[S0][0] = sx(TAG_FALLBACK_KEY);
            c.g[A0][0] = c.g[S3][0];
        }
        c.cycles += 10;
        c.g[A1][0] = c.g[SP][0];
        callee::TAG_FIELD_COPY_TAG.call(c, 0x003e_c724)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[T1][0] = 0u64;
        c.g[V0][0] = b'$' as u64;
        c.g[A3][0] = c.g[S0][0];
        c.sb(c.g[A2][0] as u32, c.g[V0][0]);
        c.g[T0][0] = c.lbu(c.g[S2][0] as u32);
        let has_bytes = c.g[T0][0] != 0u64;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
        if has_bytes {
            c.cycles += 5;
            c.g[V0][0] = 0x0049_0000;
            c.g[V1][0] = 0x0049_0000;
            c.g[T3][0] = sx(HEX_ENCODE_HI);
            c.g[T2][0] = sx(HEX_ENCODE_LO);
            loop {
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
                let more = c.g[T0][0] != 0u64;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_c758);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 13;
        c.sb(c.g[A2][0] as u32, 0u64);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = c.g[SP][0];
        callee::TAG_FIELD_SET_RAW.call(c, 0x003e_c7c0)?;
        frame.restore(c, S0, 0x1150);
        frame.restore(c, S1, 0x1140);
        frame.restore(c, S2, 0x1130);
        frame.restore(c, S3, 0x1120);
        frame.restore(c, S4, 0x1110);
        frame.restore_ra(c, 0x1100);
        frame.close(c);
        Ok(())
    })
}

/// `int TagFieldSetDate(char* record, int len, const char* name, u32 epoch)`
/// (0x3ec7e0): store a calendar date/time by formatting the epoch seconds.
pub fn tag_field_set_date(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x1b0);
        c.g[A3][0] = c.g[A3][0] << 32;
        frame.save(c, S4, 0x160);
        frame.save(c, S2, 0x180);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S3, 0x170);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x1a0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x190);
        c.g[A1][0] = c.g[A3][0] >> 32;
        let epoch_given = c.g[A1][0] != 0u64;
        frame.save_ra(c, 0x150);
        if !epoch_given {
            c.cycles += 4;
            callee::DS_TIMEINSECS.call(c, 0x003e_c81c)?;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[A1][0] = c.g[V0][0] >> 32;
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[SP][0];
        callee::DS_SECSTOTIME.call(c, 0x003e_c82c)?;
        c.g[S1][0] = c.g[V0][0];
        let tm_missing = c.g[S1][0] == 0u64;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        if tm_missing {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            frame.restore(c, S0, 0x1a0);
        } else {
            c.cycles += 22;
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S3][0];
            callee::TAG_FIELD_COPY_TAG.call(c, 0x003e_c848)?;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(TM_YEAR));
            c.g[A1][0] = 0x0049_0000;
            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(TM_MON));
            c.g[A0][0] = c.g[V0][0];
            c.g[T3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(TM_SEC));
            c.g[A1][0] = sx(DATE_FMT);
            c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(TM_MDAY));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(YEAR_BASE));
            c.g[T1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(TM_HOUR));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            c.g[T2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(TM_MIN));
            callee::SPRINTF.call(c, 0x003e_c878)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = c.g[S0][0];
            callee::TAG_FIELD_SET_RAW.call(c, 0x003e_c888)?;
            frame.restore(c, S0, 0x1a0);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x190);
        frame.restore(c, S2, 0x180);
        frame.restore(c, S3, 0x170);
        frame.restore(c, S4, 0x160);
        frame.restore_ra(c, 0x150);
        frame.close(c);
        Ok(())
    })
}

/// `int TagFieldSetDateYMDHMS(char* record, int len, const char* name, int y, int mo, int d, int h, int mi, int s, int adj)`
/// (0x3ec8b8): build a calendar time and store it as an epoch date.
/// The broken-down time is converted to epoch seconds and forwarded to
/// `TagFieldSetDate`, shifted back by `adj` when the result is non-zero.
pub fn tag_field_set_date_ymdhms(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 58;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S6, 0x60);
        frame.save(c, S7, 0x50);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, FP, 0x40);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, S0, 0xc0);
        c.g[FP][0] = c.g[A1][0];
        frame.save(c, S1, 0xb0);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0xa0);
        c.g[S1][0] = c.g[T0][0];
        frame.save(c, S3, 0x90);
        c.g[A0][0] = c.g[SP][0];
        frame.save(c, S4, 0x80);
        c.g[S3][0] = c.g[T1][0];
        frame.save(c, S5, 0x70);
        c.g[S4][0] = c.g[T2][0];
        c.g[S5][0] = c.g[T3][0];
        frame.save_ra(c, 0x30);
        c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd8));
        c.g[A1][0] = 0u64;
        c.g[A2][0] = TM_SIZE as u64;
        callee::MEMSET.call(c, 0x003e_c918)?;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(YEAR_BASE));
        c.sw((c.g[SP][0] as u32).wrapping_add(TM_ISDST), c.g[V0][0]);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[A0][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(TM_YEAR), c.g[S0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(TM_MON), c.g[S1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(TM_MDAY), c.g[S3][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(TM_HOUR), c.g[S4][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(TM_MIN), c.g[S5][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(TM_SEC), c.g[V0][0]);
        callee::DS_TIMETOSECS.call(c, 0x003e_c94c)?;
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
        callee::TAG_FIELD_SET_DATE.call(c, 0x003e_c970)?;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore(c, S7, 0x50);
        frame.restore(c, FP, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `int TagFieldSetDatePacked(char* record, int len, const char* name, int date, int time, int adj)`
/// (0x3ec9a0): unpack `date`/`time` bit fields into a calendar time and store it.
/// `date` packs year/month/day and `time` packs hour/minute/second;
/// the result is forwarded to `TagFieldSetDate` like the YMDHMS form.
pub fn tag_field_set_date_packed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 57;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x40);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x90);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S1, 0x80);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x70);
        c.g[S1][0] = c.g[T0][0];
        frame.save_ra(c, 0x30);
        c.g[S2][0] = c.g[T1][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0u64;
        c.g[A2][0] = TM_SIZE as u64;
        callee::MEMSET.call(c, 0x003e_c9e8)?;
        c.g[V1][0] = sx(((c.g[S0][0] as u32) as i32 >> 8) as u32);
        c.g[A1][0] = sx((c.g[S0][0] as u32) >> 16);
        c.g[V1][0] = c.g[V1][0] & 0xff;
        c.g[A2][0] = sx(((c.g[S1][0] as u32) as i32 >> 16) as u32);
        c.g[A3][0] = sx(((c.g[S1][0] as u32) as i32 >> 8) as u32);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(YEAR_BASE));
        c.g[A2][0] = c.g[A2][0] & 0xff;
        c.g[A3][0] = c.g[A3][0] & 0xff;
        c.g[S0][0] = c.g[S0][0] & 0xff;
        c.g[S1][0] = c.g[S1][0] & 0xff;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        c.g[A0][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(TM_YEAR), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(TM_MDAY), c.g[S0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(TM_HOUR), c.g[A2][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(TM_MIN), c.g[A3][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(TM_SEC), c.g[S1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(TM_ISDST), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(TM_MON), c.g[V1][0]);
        callee::DS_TIMETOSECS.call(c, 0x003e_ca3c)?;
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
        callee::TAG_FIELD_SET_DATE.call(c, 0x003e_ca60)?;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `int TagFieldCopyField(char* record, int len, const char* name, char* data)`
/// (0x003e_ca88): find `name` in `data` and copy the whole field into the record.
pub fn tag_field_copy_field(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = c.g[S0][0];
        callee::TAG_FIELD_FIND.call(c, 0x003e_cab4)?;
        c.g[A2][0] = c.g[V0][0];
        let found = c.g[A2][0] != 0u64;
        c.g[V0][0] = 0u64;
        if found {
            loop {
                c.cycles += 2;
                let at_start = c.g[A2][0] == c.g[S0][0];
                c.g[A0][0] = c.g[S1][0];
                if !at_start {
                    c.cycles += 4;
                    c.g[V0][0] = c.lbu((c.g[A2][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                        if event_due(c) {
                            deliver_event_here(c, 0x003e_cac0);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S1][0];
                }
                break;
            }
            c.cycles += 2;
            c.g[A1][0] = c.g[S2][0];
            callee::TAG_FIELD_SET_RAW.call(c, 0x003e_cae4)?;
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

/// `int TagFieldPrintf(char* buf, int len, const char* fmt, ...)` (0x3ecbd8):
/// a printf-style formatter that turns whitespace runs into the field
/// separator and `%s/%d/%a/%f/%e` into tag fields from the arguments;
/// `%r` copies its argument string raw.
pub fn tag_field_printf(c: &mut Cpu) {
    guest(c, |c| {
        const ENTRY: u32 = 0x003e_cbd8;
        const HEAD_FMT: u32 = 0x003e_cc50;
        const HEAD_WS: u32 = 0x003e_cc80;
        const HEAD_RAW: u32 = 0x003e_cdc0;
        const END: u32 = 0x003e_ce38;
        // The frame size must equal the size `Frame::open` uses on the entry arm.
        let frame = Frame { size: 0xe0 };
        let mut fmt_empty = false;
        let mut resume = match c.pc {
            ENTRY => {
                c.cycles += 25;
                Frame::open(c, 0xe0);
                frame.save(c, S0, 0x60);
                frame.save(c, S1, 0x50);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S2, 0x40);
                c.g[S1][0] = c.g[A1][0];
                frame.save(c, S3, 0x30);
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb8));
                frame.save(c, S5, 0x10);
                c.g[S3][0] = c.g[A2][0];
                frame.save(c, S4, 0x20);
                c.g[S5][0] = c.g[S1][0];
                frame.save_ra(c, 0);
                c.g[A1][0] = c.lbu(c.g[S3][0] as u32);
                c.sd(frame.at(c, 0xb8), c.g[A3][0]);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                c.sd(frame.at(c, 0xc0), c.g[T0][0]);
                c.sd(frame.at(c, 0xc8), c.g[T1][0]);
                c.sd(frame.at(c, 0xd0), c.g[T2][0]);
                c.sd(frame.at(c, 0xd8), c.g[T3][0]);
                c.swc1(12, frame.at(c, 0xa8));
                c.swc1(14, frame.at(c, 0xac));
                c.swc1(16, frame.at(c, 0xb0));
                fmt_empty = c.g[A1][0] == 0;
                c.swc1(18, frame.at(c, 0xb4));
                ENTRY
            }
            HEAD_FMT => HEAD_FMT,
            HEAD_WS => HEAD_WS,
            HEAD_RAW => HEAD_RAW,
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        };
        let mut run_fmt = resume != ENTRY;
        if resume == ENTRY && !fmt_empty {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 2) as u64;
            let len_short = c.g[V0][0] != 0;
            c.g[S4][0] = 0x4a0000;
            if !len_short {
                c.cycles += 2;
                c.g[V0][0] = (c.g[A1][0] < 0x21) as u64;
                run_fmt = true;
            }
        }
        if run_fmt {
            'fmt: loop {
                let mut tail_reloads_char = false;
                if resume == HEAD_WS {
                    resume = HEAD_FMT;
                    tail_reloads_char = loop {
                        c.cycles += 3;
                        c.g[V0][0] = c.lb(c.g[S3][0] as u32);
                        let at_nul = c.g[V0][0] == 0;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 33) as u64;
                        if at_nul {
                            break true;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            if back_edge_unwinds(c, HEAD_WS) {
                                return Ok(());
                            }
                            continue;
                        }
                        c.cycles += 2;
                        c.g[A1][0] = c.lbu(c.g[S3][0] as u32);
                        break false;
                    };
                } else if resume == HEAD_RAW {
                    resume = HEAD_FMT;
                    // 0x003e_cdc0 increment block, charged once on resume.
                    c.cycles += 3;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    loop {
                        c.cycles += 3;
                        c.g[V0][0] = c.lb(c.g[V1][0] as u32);
                        let at_nul = c.g[V0][0] == 0;
                        c.g[A0][0] = c.lbu(c.g[V1][0] as u32);
                        if at_nul {
                            break;
                        }
                        c.cycles += 2;
                        if (c.g[S1][0] as i64) <= 0 {
                            break;
                        }
                        c.sb(c.g[S0][0] as u32, c.g[A0][0]);
                        if back_edge_unwinds(c, HEAD_RAW) {
                            return Ok(());
                        }
                        c.cycles += 3;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    }
                    // Shared conversion tail (same as after the dispatch).
                    c.cycles += 2;
                    if (c.g[A2][0] as i64) <= 0 {
                        c.g[A1][0] = c.lbu(c.g[S3][0] as u32);
                    } else {
                        c.cycles += 2;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                        tail_reloads_char = true;
                    }
                } else {
                    c.cycles += 2;
                    let literal = c.g[V0][0] == 0;
                    c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(FIELD_SEP_GP));
                    if !literal {
                        // Whitespace: emit one separator, skip the run.
                        c.cycles += 5;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                        c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                        c.g[V0][0] = c.lb(c.g[S3][0] as u32);
                        let fmt_ended = c.g[V0][0] == 0;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        if fmt_ended {
                            tail_reloads_char = true;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 33) as u64;
                            if c.g[V0][0] == 0 {
                                c.g[A1][0] = c.lbu(c.g[S3][0] as u32);
                            } else {
                                c.cycles += 2;
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                tail_reloads_char = loop {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lb(c.g[S3][0] as u32);
                                    let at_nul = c.g[V0][0] == 0;
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < 33) as u64;
                                    if at_nul {
                                        break true;
                                    }
                                    c.cycles += 2;
                                    if c.g[V0][0] != 0 {
                                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                        if back_edge_unwinds(c, HEAD_WS) {
                                            return Ok(());
                                        }
                                        continue;
                                    }
                                    c.cycles += 2;
                                    c.g[A1][0] = c.lbu(c.g[S3][0] as u32);
                                    break false;
                                };
                            }
                        }
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x25;
                        if c.g[A1][0] != c.g[V0][0] {
                            // Plain character, copied through.
                            c.cycles += 4;
                            c.sb(c.g[S0][0] as u32, c.g[A1][0]);
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            tail_reloads_char = true;
                        } else {
                            c.g[V0][0] = c.lb(c.g[S3][0] as u32);
                            c.cycles += 2;
                            let spec_empty = c.g[V0][0] == 0;
                            c.g[A2][0] = 0;
                            if spec_empty {
                                tail_reloads_char = true;
                            } else {
                                // `%` conversion; see dispatch below.
                                c.cycles += 5;
                                c.g[A1][0] = c.lbu(c.g[S3][0] as u32);
                                c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_sub(0x1));
                                c.g[V0][0] = 0x3d;
                                let no_equals = c.g[V1][0] != c.g[V0][0];
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                if !no_equals {
                                    // Drop the `=` the spec replaces.
                                    c.cycles += 2;
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0x73;
                                let not_s = c.g[A1][0] != c.g[V0][0];
                                c.sb(c.g[S0][0] as u32, 0);
                                if !not_s {
                                    c.cycles += 8;
                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                                    c.g[A2][0] = sx(EMPTY_NAME);
                                    c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x8));
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = c.g[S1][0];
                                    callee::TAG_FIELD_SET_STRING.call(c, 0x003e_cd00)?;
                                    c.g[A2][0] = c.g[V0][0];
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x64;
                                    let not_d = c.g[A1][0] != c.g[V0][0];
                                    c.g[V0][0] = 0x61;
                                    if !not_d {
                                        c.cycles += 8;
                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                                        c.g[A2][0] = sx(EMPTY_NAME);
                                        c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x8));
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = c.g[S1][0];
                                        callee::TAG_FIELD_SET_NUMBER.call(c, 0x003e_cd2c)?;
                                        c.g[A2][0] = c.g[V0][0];
                                    } else {
                                        c.cycles += 2;
                                        let not_a = c.g[A1][0] != c.g[V0][0];
                                        c.g[V0][0] = 0x66;
                                        if !not_a {
                                            c.cycles += 8;
                                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                                            c.g[A2][0] = sx(EMPTY_NAME);
                                            c.g[A3][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_sub(0x8));
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = c.g[S1][0];
                                            callee::TAG_FIELD_SET_ADDRESS.call(c, 0x003e_cd54)?;
                                            c.g[A2][0] = c.g[V0][0];
                                        } else {
                                            c.cycles += 2;
                                            let not_f = c.g[A1][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x65;
                                            if !not_f {
                                                c.cycles += 8;
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x8));
                                                c.g[A2][0] = sx(EMPTY_NAME);
                                                c.g[A3][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_sub(0x8));
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] = c.g[S1][0];
                                                callee::TAG_FIELD_SET_FLAGS.call(c, 0x003e_cd7c)?;
                                                c.g[A2][0] = c.g[V0][0];
                                            } else {
                                                c.cycles += 2;
                                                let not_e = c.g[A1][0] != c.g[V0][0];
                                                c.g[V0][0] = 0x72;
                                                if !not_e {
                                                    c.cycles += 8;
                                                    c.g[S2][0] =
                                                        sx((c.g[S2][0] as u32).wrapping_add(0x8));
                                                    c.g[A2][0] = sx(EMPTY_NAME);
                                                    c.g[A3][0] =
                                                        c.lw((c.g[S2][0] as u32).wrapping_sub(0x8));
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[A1][0] = c.g[S1][0];
                                                    callee::TAG_FIELD_SET_DATE
                                                        .call(c, 0x003e_cda4)?;
                                                    c.g[A2][0] = c.g[V0][0];
                                                } else {
                                                    c.cycles += 2;
                                                    let not_r = c.g[A1][0] != c.g[V0][0];
                                                    if !not_r {
                                                        // `%r`: copy the raw string.
                                                        c.cycles += 3;
                                                        c.g[S2][0] =
                                                            sx((c.g[S2][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_sub(0x8));
                                                        loop {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lb(c.g[V1][0] as u32);
                                                            let at_nul = c.g[V0][0] == 0;
                                                            c.g[A0][0] = c.lbu(c.g[V1][0] as u32);
                                                            if at_nul {
                                                                break;
                                                            }
                                                            c.cycles += 2;
                                                            if (c.g[S1][0] as i64) <= 0 {
                                                                break;
                                                            }
                                                            c.sb(c.g[S0][0] as u32, c.g[A0][0]);
                                                            if back_edge_unwinds(c, HEAD_RAW) {
                                                                return Ok(());
                                                            }
                                                            c.cycles += 3;
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[S1][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_sub(0x1));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                // Shared conversion tail: account for n chars.
                                c.cycles += 2;
                                if (c.g[A2][0] as i64) <= 0 {
                                    c.g[A1][0] = c.lbu(c.g[S3][0] as u32);
                                } else {
                                    c.cycles += 2;
                                    c.g[S1][0] =
                                        sx((c.g[S1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                                    tail_reloads_char = true;
                                }
                            }
                        }
                    }
                }
                if tail_reloads_char {
                    c.cycles += 1;
                    c.g[A1][0] = c.lbu(c.g[S3][0] as u32);
                }
                c.cycles += 2;
                let fmt_done = c.g[A1][0] == 0;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                if fmt_done {
                    break 'fmt;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 2) as u64;
                let has_room = c.g[V0][0] == 0;
                c.g[V0][0] = (c.g[A1][0] < 0x21) as u64;
                if has_room {
                    if back_edge_unwinds(c, HEAD_FMT) {
                        return Ok(());
                    }
                    continue 'fmt;
                }
                break 'fmt;
            }
        }
        c.cycles += 2;
        if (c.g[S1][0] as i64) > 0 {
            c.sb(c.g[S0][0] as u32, 0);
        }
        c.cycles += 10;
        c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_sub(c.g[S1][0] as u32));
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

/// `int TagFieldGetDelim(const char* data, char* buf, int len, const char* dflt, int index, int delim)`
/// (0x3ed0f8): GetString with an index/delim skip before the value.
pub fn tag_field_get_delim(c: &mut Cpu) {
    guest(c, |c| {
        const ENTRY: u32 = 0x003e_d0f8;
        const HEAD_SKIP_FAIL: u32 = 0x003e_d12c;
        const HEAD_SCAN_NEXT: u32 = 0x003e_d148;
        const HEAD_SCAN: u32 = 0x003e_d14c;
        const HEAD_DEFAULT: u32 = 0x003e_d1a8;
        const HEAD_MEASURE: u32 = 0x003e_d210;
        const HEAD_COPY: u32 = 0x003e_d2c0;
        const END: u32 = 0x003e_d378;
        // The frame size must equal the size `Frame::open` uses on the entry arm.
        let frame = Frame { size: 0x10 };
        let mut data_is_null = false;
        let mut resume = match c.pc {
            ENTRY => {
                c.cycles += 4;
                Frame::open(c, 0x10);
                c.g[T2][0] = c.g[A1][0];
                data_is_null = c.g[A0][0] == 0u64;
                frame.save_ra(c, 0);
                ENTRY
            }
            HEAD_SKIP_FAIL => HEAD_SKIP_FAIL,
            HEAD_SCAN_NEXT => HEAD_SCAN_NEXT,
            HEAD_SCAN => HEAD_SCAN,
            HEAD_DEFAULT => HEAD_DEFAULT,
            HEAD_MEASURE => HEAD_MEASURE,
            HEAD_COPY => HEAD_COPY,
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        };
        let mut through_skip = false;
        'body: {
            if resume == HEAD_SKIP_FAIL {
                // 0x003e_d12c block, charged once on resume.
                c.cycles += 2;
                c.g[A0][0] = 0u64;
                resume = ENTRY;
                through_skip = true;
            } else if resume == ENTRY || resume == HEAD_SCAN || resume == HEAD_SCAN_NEXT {
                if resume == ENTRY && !data_is_null {
                    // Quote sniff (0x3ed108/118/120): T3 = cQuote.
                    c.cycles += 4;
                    c.g[V1][0] = c.lb(c.g[A0][0] as u32);
                    c.g[V0][0] = 0x22;
                    let is_double_quote = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x27;
                    if is_double_quote {
                        c.cycles += 3;
                        c.g[T3][0] = c.lbu(c.g[A0][0] as u32);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    } else {
                        c.cycles += 2;
                        let first_is_not_single_quote = c.g[V1][0] != c.g[V0][0];
                        c.g[T3][0] = 0u64;
                        if !first_is_not_single_quote {
                            c.cycles += 3;
                            c.g[T3][0] = c.lbu(c.g[A0][0] as u32);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        }
                    }
                }
                if resume != ENTRY || !data_is_null {
                    through_skip = true;
                    // Skip `index` (T0) fields (0x3ed134..0x3ed170).
                    'skip: loop {
                        if resume == HEAD_SCAN_NEXT {
                            // 0x003e_d148 reload, charged once on resume.
                            c.cycles += 1;
                            c.g[A1][0] = c.lb(c.g[A0][0] as u32);
                            resume = HEAD_SCAN;
                        }
                        if resume == HEAD_SCAN {
                            resume = ENTRY;
                        } else {
                            c.cycles += 2;
                            if (c.g[T0][0] as i64) <= 0 {
                                break 'skip;
                            }
                            c.cycles += 2;
                            c.g[A1][0] = c.lb(c.g[A0][0] as u32);
                        }
                        'scan: loop {
                            c.cycles += 2;
                            let at_quote = c.g[A1][0] == c.g[T3][0];
                            c.g[V0][0] = ((c.g[A1][0] as i64) < 32i64) as u64;
                            if at_quote {
                                c.cycles += 1;
                                c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                                } else {
                                    c.cycles += 2;
                                    if c.g[A1][0] != c.g[T1][0] {
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                        if back_edge_unwinds(c, HEAD_SCAN_NEXT) {
                                            return Ok(());
                                        }
                                        c.cycles += 1;
                                        c.g[A1][0] = c.lb(c.g[A0][0] as u32);
                                        continue 'scan;
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                                }
                            }
                            c.cycles += 2;
                            let not_delim = c.g[V0][0] != c.g[T1][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            if not_delim {
                                if back_edge_unwinds(c, HEAD_SKIP_FAIL) {
                                    return Ok(());
                                }
                                c.cycles += 2;
                                c.g[A0][0] = 0u64;
                                break 'skip;
                            }
                            c.cycles += 3;
                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                            if (c.g[T0][0] as i64) > 0 {
                                c.g[A1][0] = c.lb(c.g[A0][0] as u32);
                                if back_edge_unwinds(c, HEAD_SCAN) {
                                    return Ok(());
                                }
                                continue 'scan;
                            }
                            break 'skip;
                        }
                    }
                }
            }
            let mut enter_measure = false;
            let mut finish_measure = false;
            let mut enter_copy = false;
            let mut enter_default = false;
            if resume == HEAD_MEASURE {
                enter_measure = true;
            } else if resume == HEAD_COPY {
                enter_copy = true;
            } else if resume == HEAD_DEFAULT {
                // 0x003e_d1a8 block, charged once on resume.
                c.cycles += 4;
                c.sb(c.g[T2][0] as u32, c.g[V1][0]);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                enter_default = true;
            } else {
                // 0x3ed17c test charge: only on the through-skip path (a
                // null-data entry goes straight to 0x3ed184 below).
                if through_skip {
                    c.cycles += 2;
                }
                if c.g[A0][0] != 0u64 {
                    // Value path (0x3ed1d8).
                    c.cycles += 2;
                    let buf_is_null = c.g[T2][0] == 0u64;
                    c.g[A3][0] = c.g[A0][0];
                    if buf_is_null {
                        // Measure only (0x3ed1e0..0x3ed260).
                        c.cycles += 4;
                        c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
                        c.g[V0][0] = c.g[V1][0] & 0xff;
                        let first_at_quote = c.g[V0][0] == c.g[T3][0];
                        c.g[T0][0] = 0u64;
                        if !first_at_quote {
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                            let first_is_ctrl = c.g[V0][0] != 0u64;
                            c.g[V0][0] = c.g[T0][0];
                            if first_is_ctrl {
                                break 'body;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.g[V1][0] & 0xff;
                            let first_at_delim = c.g[V0][0] == c.g[T1][0];
                            c.g[V0][0] = c.g[T0][0];
                            if first_at_delim {
                                break 'body;
                            }
                            c.cycles += 2;
                            c.g[A0][0] = 0x25;
                            enter_measure = true;
                        } else {
                            finish_measure = true;
                        }
                    } else {
                        // Decode-copy setup (0x3ed268..0x3ed2a8).
                        c.cycles += 2;
                        let len_positive = (c.g[A2][0] as i64) > 0;
                        c.g[T0][0] = 0x1;
                        if !len_positive {
                            c.cycles += 2;
                            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                            break 'body;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                        let already_full = c.g[V0][0] == 0u64;
                        c.g[A3][0] = c.g[A0][0];
                        if already_full {
                            c.cycles += 1;
                            c.sb(c.g[T2][0] as u32, 0u64);
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                            break 'body;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.lbu(c.g[A3][0] as u32);
                        c.g[V0][0] = c.g[A0][0] & 0xff;
                        let head_at_quote = c.g[V0][0] == c.g[T3][0];
                        c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                        if head_at_quote {
                            c.cycles += 1;
                            c.sb(c.g[T2][0] as u32, 0u64);
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                            break 'body;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != 0u64 {
                            c.sb(c.g[T2][0] as u32, 0u64);
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                            break 'body;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.g[A0][0] & 0xff;
                        let head_at_delim = c.g[V0][0] == c.g[T1][0];
                        c.g[V0][0] = 0x490000;
                        if head_at_delim {
                            c.cycles += 1;
                            c.sb(c.g[T2][0] as u32, 0u64);
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                            break 'body;
                        }
                        c.cycles += 6;
                        c.g[V1][0] = 0x490000;
                        c.g[T6][0] = sx(HEX_DECODE_HI);
                        c.g[T5][0] = sx(HEX_DECODE_LO);
                        c.g[T4][0] = 0x25;
                        c.g[V1][0] = c.g[A0][0] & 0xff;
                        enter_copy = true;
                    }
                } else {
                    // Default path (0x3ed184).
                    c.cycles += 2;
                    let default_is_null = c.g[A3][0] == 0u64;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    if default_is_null {
                        break 'body;
                    }
                    c.cycles += 2;
                    let buf_is_null = c.g[T2][0] == 0u64;
                    c.g[T0][0] = 0x1;
                    if buf_is_null {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[A3][0];
                        callee::STRLEN.call(c, 0x003e_d19c)?;
                        frame.restore_ra(c, 0);
                        c.cycles += 2;
                        frame.close(c);
                        return Ok(());
                    }
                    enter_default = true;
                }
            }
            if enter_measure {
                'measure: loop {
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[A0][0] {
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x2));
                            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2));
                            c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                            if c.g[V0][0] == 0 {
                                c.g[A3][0] = c.g[V1][0];
                            }
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                        }
                    }
                    c.cycles += 4;
                    c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
                    c.g[V0][0] = c.g[V1][0] & 0xff;
                    let at_quote = c.g[V0][0] == c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    if at_quote {
                        break 'measure;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                    let is_ctrl = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.g[V1][0] & 0xff;
                    if is_ctrl {
                        break 'measure;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[T1][0] {
                        if back_edge_unwinds(c, HEAD_MEASURE) {
                            return Ok(());
                        }
                        continue 'measure;
                    }
                    break 'measure;
                }
                finish_measure = true;
            }
            if finish_measure {
                c.cycles += 2;
                c.g[V0][0] = c.g[T0][0];
                break 'body;
            }
            if enter_copy {
                'copy: loop {
                    let mut skip_store = false;
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[T4][0] {
                        c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                        c.cycles += 2;
                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1));
                        if c.g[V0][0] != c.g[V1][0] {
                            c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                            c.cycles += 2;
                            if c.g[V0][0] != 0u64 {
                                c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                                c.cycles += 2;
                                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x2));
                                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                                    c.cycles += 2;
                                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                } else {
                                    c.cycles += 10;
                                    c.g[A1][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1));
                                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                    c.g[A0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x2));
                                    c.g[A1][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add(c.g[T6][0] as u32));
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3));
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add(c.g[T5][0] as u32));
                                    c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                                    c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                }
                            }
                        } else {
                            c.cycles += 5;
                            c.sb(c.g[T2][0] as u32, c.g[V0][0]);
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2));
                            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                            skip_store = true;
                        }
                    }
                    if !skip_store {
                        c.cycles += 2;
                        c.sb(c.g[T2][0] as u32, c.g[V0][0]);
                        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.sb(c.g[T2][0] as u32, 0u64);
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                        break 'body;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.lbu(c.g[A3][0] as u32);
                    c.g[V0][0] = c.g[A0][0] & 0xff;
                    let at_quote = c.g[V0][0] == c.g[T3][0];
                    c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                    if at_quote {
                        c.cycles += 1;
                        c.sb(c.g[T2][0] as u32, 0u64);
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                        break 'body;
                    }
                    c.cycles += 2;
                    let is_ctrl = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.g[A0][0] & 0xff;
                    if is_ctrl {
                        c.cycles += 1;
                        c.sb(c.g[T2][0] as u32, 0u64);
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                        break 'body;
                    }
                    c.cycles += 2;
                    let not_delim = c.g[V0][0] != c.g[T1][0];
                    c.g[V1][0] = c.g[A0][0] & 0xff;
                    if not_delim {
                        if back_edge_unwinds(c, HEAD_COPY) {
                            return Ok(());
                        }
                        continue 'copy;
                    }
                    c.cycles += 1;
                    c.sb(c.g[T2][0] as u32, 0u64);
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                    break 'body;
                }
            }
            if enter_default {
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.sb(c.g[T2][0] as u32, 0u64);
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                        break 'body;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[A3][0] as u32);
                    let byte_nonzero = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
                    if !byte_nonzero {
                        c.cycles += 2;
                        c.sb(c.g[T2][0] as u32, 0u64);
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                        break 'body;
                    }
                    if back_edge_unwinds(c, HEAD_DEFAULT) {
                        return Ok(());
                    }
                    c.cycles += 4;
                    c.sb(c.g[T2][0] as u32, c.g[V1][0]);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                }
            }
        }
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `int TagFieldGetString(const char* data, char* buf, int len, const char* dflt)`
/// (0x3ed378): copy a tag value (quoted or bare, with `%xx` decoding) into
/// `buf`, or the default; with a null buffer just return the size.
pub fn tag_field_get_string(c: &mut Cpu) {
    guest(c, |c| {
        const ENTRY: u32 = 0x003e_d378;
        const HEAD_DEFAULT: u32 = 0x003e_d3b0;
        const HEAD_MEASURE: u32 = 0x003e_d428;
        const HEAD_COPY: u32 = 0x003e_d4c8;
        const END: u32 = 0x003e_d578;
        let resume = c.pc;
        let mut data_present = false;
        // The frame size must equal the size `Frame::open` uses on the entry arm.
        let frame = match resume {
            ENTRY => {
                c.cycles += 6;
                let frame = Frame::open(c, 0x10);
                c.g[V1][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[T0][0] = c.g[A1][0];
                data_present = c.g[V1][0] != 0u64;
                c.g[A0][0] = c.g[A3][0];
                frame
            }
            HEAD_DEFAULT | HEAD_MEASURE | HEAD_COPY => Frame { size: 0x10 },
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        };
        'body: {
            if resume == HEAD_DEFAULT || (resume == ENTRY && !data_present) {
                // Default path (0x3ed390..0x3ed3d8).
                if resume == HEAD_DEFAULT {
                    // Resume at 0x003e_d3b0: pending store from the back edge, charged once.
                    c.cycles += 4;
                    c.sb(c.g[T0][0] as u32, c.g[V1][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                } else {
                    c.cycles += 2;
                    let default_is_null = c.g[A0][0] == 0u64;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    if default_is_null {
                        break 'body;
                    }
                    c.cycles += 2;
                    let buf_is_null = c.g[T0][0] == 0u64;
                    c.g[T1][0] = 0x1;
                    if buf_is_null {
                        c.cycles += 4;
                        callee::STRLEN.call(c, 0x003e_d3a8)?;
                        frame.restore_ra(c, 0);
                        c.cycles += 2;
                        frame.close(c);
                        return Ok(());
                    }
                }
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.sb(c.g[T0][0] as u32, 0u64);
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
                        break 'body;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                    let byte_nonzero = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                    if !byte_nonzero {
                        c.cycles += 2;
                        c.sb(c.g[T0][0] as u32, 0u64);
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
                        break 'body;
                    }
                    if back_edge_unwinds(c, HEAD_DEFAULT) {
                        return Ok(());
                    }
                    c.cycles += 4;
                    c.sb(c.g[T0][0] as u32, c.g[V1][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                }
            }
            let mut buf_present = resume == HEAD_COPY;
            if resume == ENTRY {
                // Quote sniff (0x3ed3e0/3f0/3f8): T2 = cQuote, V1 past quote.
                c.cycles += 4;
                c.g[A0][0] = c.lb(c.g[V1][0] as u32);
                c.g[V0][0] = 0x22;
                let is_double_quote = c.g[A0][0] == c.g[V0][0];
                c.g[V0][0] = 0x27;
                if is_double_quote {
                    c.cycles += 2;
                    c.g[T2][0] = c.lbu(c.g[V1][0] as u32);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                } else {
                    c.cycles += 2;
                    let not_single_quote = c.g[A0][0] != c.g[V0][0];
                    c.g[T2][0] = 0u64;
                    if !not_single_quote {
                        c.cycles += 2;
                        c.g[T2][0] = c.lbu(c.g[V1][0] as u32);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    }
                }
                c.cycles += 2;
                buf_present = c.g[T0][0] != 0u64;
                c.g[A3][0] = c.g[V1][0];
            }
            if !buf_present {
                // Measure only (0x3ed408..0x3ed470). Resume at 0x003e_d428 enters
                // the loop directly: state is in A3/T1/T2/V1/A0 (A0 = b'%').
                if resume != HEAD_MEASURE {
                    c.cycles += 4;
                    c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
                    c.g[V0][0] = c.g[V1][0] & 0xff;
                    let first_at_quote = c.g[V0][0] == c.g[T2][0];
                    c.g[T1][0] = 0u64;
                    if first_at_quote {
                        c.cycles += 2;
                        c.g[V0][0] = c.g[T1][0];
                        break 'body;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                    let first_is_ctrl = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.g[T1][0];
                    if first_is_ctrl {
                        break 'body;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = b'%' as u64;
                }
                'measure: loop {
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[A0][0] {
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x2));
                            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2));
                            c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                            if c.g[V0][0] == 0 {
                                c.g[A3][0] = c.g[V1][0];
                            }
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                        }
                    }
                    c.cycles += 4;
                    c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
                    c.g[V0][0] = c.g[V1][0] & 0xff;
                    let at_quote = c.g[V0][0] == c.g[T2][0];
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                    if at_quote {
                        break 'measure;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                    if c.g[V0][0] == 0u64 {
                        if back_edge_unwinds(c, HEAD_MEASURE) {
                            return Ok(());
                        }
                        continue 'measure;
                    }
                    break 'measure;
                }
                c.cycles += 2;
                c.g[V0][0] = c.g[T1][0];
                break 'body;
            }
            // Decode-copy the value (0x3ed478..0x3ed564). Resume at 0x003e_d4c8
            // enters the loop directly: state is in A2/A3/T0/T1/T2/T3/T4/T5/V1.
            if resume != HEAD_COPY {
                c.cycles += 2;
                let len_positive = (c.g[A2][0] as i64) > 0;
                c.g[T1][0] = 0x1;
                if !len_positive {
                    c.cycles += 2;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    break 'body;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                let already_full = c.g[V0][0] == 0u64;
                c.g[A3][0] = c.g[V1][0];
                if already_full {
                    c.cycles += 1;
                    c.sb(c.g[T0][0] as u32, 0u64);
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
                    break 'body;
                }
                c.cycles += 4;
                c.g[A0][0] = c.lbu(c.g[A3][0] as u32);
                c.g[V0][0] = c.g[A0][0] & 0xff;
                let head_at_quote = c.g[V0][0] == c.g[T2][0];
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if head_at_quote {
                    c.cycles += 1;
                    c.sb(c.g[T0][0] as u32, 0u64);
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
                    break 'body;
                }
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    c.sb(c.g[T0][0] as u32, 0u64);
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
                    break 'body;
                }
                c.cycles += 7;
                c.g[V0][0] = 0x490000;
                c.g[V1][0] = 0x490000;
                c.g[T5][0] = sx(HEX_DECODE_HI);
                c.g[T4][0] = sx(HEX_DECODE_LO);
                c.g[T3][0] = b'%' as u64;
                c.g[V1][0] = c.g[A0][0] & 0xff;
            }
            'copy: loop {
                let mut skip_store = false;
                c.cycles += 2;
                if c.g[V1][0] != c.g[T3][0] {
                    c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                    c.cycles += 2;
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1));
                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                        c.cycles += 2;
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                            c.cycles += 2;
                            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x2));
                            c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                                c.cycles += 2;
                                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                            } else {
                                c.cycles += 10;
                                c.g[A1][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x1));
                                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                                c.g[A0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x2));
                                c.g[A1][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(c.g[T5][0] as u32));
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3));
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[T4][0] as u32));
                                c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                                c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                            }
                        }
                    } else {
                        c.cycles += 5;
                        c.sb(c.g[T0][0] as u32, c.g[V0][0]);
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2));
                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                        skip_store = true;
                    }
                }
                if !skip_store {
                    c.cycles += 2;
                    c.sb(c.g[T0][0] as u32, c.g[V0][0]);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                if c.g[V0][0] == 0u64 {
                    c.sb(c.g[T0][0] as u32, 0u64);
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
                    break 'body;
                }
                c.cycles += 4;
                c.g[A0][0] = c.lbu(c.g[A3][0] as u32);
                c.g[V0][0] = c.g[A0][0] & 0xff;
                let at_quote = c.g[V0][0] == c.g[T2][0];
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if at_quote {
                    c.cycles += 1;
                    c.sb(c.g[T0][0] as u32, 0u64);
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
                    break 'body;
                }
                c.cycles += 2;
                let not_ctrl = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.g[A0][0] & 0xff;
                if not_ctrl {
                    if back_edge_unwinds(c, HEAD_COPY) {
                        return Ok(());
                    }
                    continue 'copy;
                }
                c.cycles += 1;
                c.sb(c.g[T0][0] as u32, 0u64);
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
                break 'body;
            }
        }
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `int TagFieldGetBinary(const char* data, void* buf, int len, const char* fmt)`
/// (0x3ed678): 4-arg template decoder (reference `TagFieldGetStructure`): walk
/// the `fmt` letters `#` (skip to `=`), `a`, `b`, `w`, `l`, `s`, `*` decoding
/// the field value into `buf`.
pub fn tag_field_get_binary(c: &mut Cpu) {
    const ENTRY: u32 = 0x003e_d678;
    const HEAD_FMT: u32 = 0x003e_d6e8;
    const HEAD_SKIP: u32 = 0x003e_d708;
    const HEAD_DIGIT: u32 = 0x003e_d738;
    const HEAD_HEX: u32 = 0x003e_d7c0;
    const HEAD_S: u32 = 0x003e_d880;
    const HEAD_FILL: u32 = 0x003e_d8f8;
    const END: u32 = 0x003e_d96c;
    let resume = c.pc;
    let mut fmt_done = false;
    // The frame size must equal the size `Frame::open` uses on the entry arm.
    let frame = match resume {
        ENTRY => {
            c.cycles += 9;
            let frame = Frame::open(c, 0x440);
            c.g[T7][0] = c.g[A1][0];
            frame.save(c, S0, 0x430);
            c.g[T0][0] = c.g[A0][0];
            frame.save(c, S1, 0x420);
            c.g[T1][0] = c.g[T7][0];
            frame.save(c, S2, 0x410);
            let len_nonneg = (c.g[A2][0] as i64) >= 0;
            frame.save(c, S3, 0x400);
            if len_nonneg {
                c.cycles += 1;
                c.g[T8][0] = sx((c.g[T7][0] as u32).wrapping_add(c.g[A2][0] as u32));
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0xffff;
                c.g[T8][0] = sx((c.g[T7][0] as u32).wrapping_add(c.g[V0][0] as u32));
            }
            c.cycles += 2;
            let data_present = c.g[T0][0] != 0u64;
            c.g[V0][0] = 0x4a0000;
            if !data_present {
                c.cycles += 1;
                c.g[T0][0] = sx(EMPTY_NAME);
            }
            c.cycles += 2;
            let buf_present = c.g[T7][0] != 0u64;
            c.g[V0][0] = c.lb(c.g[A3][0] as u32);
            if !buf_present {
                c.cycles += 2;
                c.g[T7][0] = c.g[SP][0];
                c.g[T1][0] = c.g[SP][0];
            }
            c.cycles += 2;
            fmt_done = c.g[V0][0] == 0u64;
            c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
            frame
        }
        HEAD_FMT | HEAD_SKIP | HEAD_DIGIT | HEAD_HEX | HEAD_S | HEAD_FILL => Frame { size: 0x440 },
        pc => {
            c.jump_out(pc, ENTRY, END);
            return;
        }
    };
    if fmt_done {
        c.cycles += 1;
    } else {
        if resume == ENTRY {
            c.cycles += 6;
            c.g[S3][0] = b'a' as u64;
            c.g[S2][0] = 0x2;
            c.g[S0][0] = 0x4;
            c.g[S1][0] = 0x8;
            c.g[T9][0] = 0x490000;
        }
        let mut resume_at = resume;
        loop {
            // Resume stage for the first iteration (0 on later iterations):
            // 0 fmt/skip-pre, 1 `#`-skip loop, 2 digit loop, 3 hex loop,
            // 4 `s` store, 5 zero-fill loop. State is in registers/stack.
            let stage: u32 = match resume_at {
                HEAD_SKIP => 1,
                HEAD_DIGIT => 2,
                HEAD_HEX => 3,
                HEAD_S => 4,
                HEAD_FILL => 5,
                _ => 0,
            };
            resume_at = ENTRY;
            let mut not_digit = false;
            let mut enter_skip = stage == 1;
            if stage == 0 {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                c.g[V1][0] = b'#' as u64;
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                if c.g[V0][0] == c.g[V1][0] {
                    c.cycles += 3;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = b'=' as u64;
                    enter_skip = true;
                } else {
                    c.g[A1][0] = c.lbu(c.g[A3][0] as u32);
                }
            }
            if enter_skip {
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[A3][0] as u32);
                    if c.g[V0][0] == 0u64 {
                        c.g[A1][0] = c.lbu(c.g[A3][0] as u32);
                        break;
                    }
                    c.cycles += 2;
                    let not_eq = c.g[V0][0] != c.g[V1][0];
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    if !not_eq {
                        c.cycles += 1;
                        c.g[A1][0] = c.lbu(c.g[A3][0] as u32);
                        break;
                    }
                    if back_edge_unwinds(c, HEAD_SKIP) {
                        return;
                    }
                }
            }
            if stage <= 1 {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(b'0' as u32));
                c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                not_digit = c.g[V0][0] == 0u64;
                c.g[T4][0] = 0u64;
                if !not_digit {
                    c.cycles += 2;
                    c.g[A2][0] = 0xa;
                }
            }
            if stage == 2 || (stage <= 1 && !not_digit) {
                loop {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[A1][0] & 0xf;
                    c.mult(0, 3, 12, 6);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = c.lbu(c.g[A3][0] as u32);
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(b'0' as u32));
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let more = c.g[V0][0] != 0u64;
                    c.g[T4][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    if !more {
                        break;
                    }
                    if back_edge_unwinds(c, HEAD_DIGIT) {
                        return;
                    }
                }
            }
            if stage <= 2 {
                c.cycles += 3;
                c.g[V0][0] = c.lb(c.g[A3][0] as u32);
                let not_a = c.g[V0][0] != c.g[S3][0];
                c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
                if !not_a {
                    c.cycles += 2;
                    if c.g[T4][0] == 0u64 {
                        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                    } else {
                        c.cycles += 1;
                        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T4][0] as u32));
                    }
                }
            }
            let mut no_size = false;
            if stage <= 3 {
                if stage < 3 {
                    c.cycles += 11;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                    c.g[A2][0] = c.g[S2][0];
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    c.g[V1][0] = c.g[V0][0] ^ b'b' as u64;
                    c.g[A0][0] = c.g[V0][0] ^ b'w' as u64;
                    if c.g[V1][0] != 0 {
                        c.g[A2][0] = 0u64;
                    }
                    c.g[V0][0] = c.g[V0][0] ^ b'l' as u64;
                    if c.g[A0][0] == 0 {
                        c.g[A2][0] = c.g[S0][0];
                    }
                    if c.g[V0][0] == 0 {
                        c.g[A2][0] = c.g[S1][0];
                    }
                    no_size = (c.g[A2][0] as i64) <= 0;
                    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                }
                if no_size {
                    if stage < 3 {
                        c.cycles += 1;
                        c.g[V1][0] = c.lb(c.g[A3][0] as u32);
                    }
                } else {
                    let mut value_end = false;
                    if stage < 3 {
                        c.cycles += 8;
                        c.g[A1][0] = c.lb(c.g[T0][0] as u32);
                        c.g[T2][0] = 0u64;
                        c.g[V1][0] = c.g[A1][0] ^ b'-' as u64;
                        if c.g[V1][0] == 0 {
                            c.g[T0][0] = c.g[V0][0];
                        }
                        c.g[A0][0] = c.lb(c.g[T0][0] as u32);
                        c.g[A0][0] = ((c.g[A0][0] as i64) < b'0' as i64) as u64;
                        value_end = c.g[A0][0] != 0u64;
                        c.g[V1][0] = c.lbu(c.g[T0][0] as u32);
                    }
                    let mut hex_done = false;
                    if !value_end {
                        if stage < 3 {
                            c.cycles += 1;
                            c.g[T3][0] = sx(HEX_DECODE_LO);
                        }
                        loop {
                            c.cycles += 9;
                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                            c.g[A0][0] = sx((c.g[T2][0] as u32) << 4);
                            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                            c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                            let size_done = (c.g[A2][0] as i64) <= 0;
                            c.g[T2][0] = c.g[A0][0] | c.g[V1][0];
                            if size_done {
                                hex_done = true;
                                break;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = c.lb(c.g[T0][0] as u32);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < b'0' as i64) as u64;
                            let more_hex = c.g[V0][0] == 0u64;
                            c.g[V1][0] = c.lbu(c.g[T0][0] as u32);
                            if !more_hex {
                                break;
                            }
                            if back_edge_unwinds(c, HEAD_HEX) {
                                return;
                            }
                        }
                    }
                    if !hex_done {
                        c.cycles += 2;
                        // kept: original re-tests `a2 <= 0` at 0x003e_d7f4, which cannot be true here (a2 is 2, 4 or 8, or was checked at 0x003e_d7c0).
                        let no_more = (c.g[A2][0] as i64) <= 0;
                        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                        if !no_more {
                            c.cycles += 3;
                            c.g[V0][0] = c.lb(c.g[T0][0] as u32);
                            c.g[V0][0] = c.g[V0][0] ^ b',' as u64;
                            if c.g[V0][0] == 0 {
                                c.g[T0][0] = c.g[V1][0];
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.g[A1][0] ^ b'-' as u64;
                    c.g[V1][0] = sx(0u32.wrapping_sub(c.g[T2][0] as u32));
                    c.g[A1][0] = c.lb(c.g[A3][0] as u32);
                    c.g[V0][0] = b'b' as u64;
                    let not_b = c.g[A1][0] != c.g[V0][0];
                    if c.g[A0][0] == 0 {
                        c.g[T2][0] = c.g[V1][0];
                    }
                    if !not_b {
                        c.cycles += 2;
                        c.sb(c.g[T1][0] as u32, c.g[T2][0]);
                        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                    }
                    c.cycles += 4;
                    c.g[V1][0] = c.lb(c.g[A3][0] as u32);
                    c.g[V0][0] = b'w' as u64;
                    let not_w = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = b'l' as u64;
                    if !not_w {
                        c.cycles += 3;
                        c.sh(c.g[T1][0] as u32, c.g[T2][0]);
                        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2));
                        c.g[V1][0] = c.lb(c.g[A3][0] as u32);
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V1][0] = c.lb(c.g[A3][0] as u32);
                    } else {
                        c.cycles += 2;
                        c.sw(c.g[T1][0] as u32, c.g[T2][0]);
                        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x4));
                        c.cycles += 1;
                        c.g[V1][0] = c.lb(c.g[A3][0] as u32);
                    }
                }
            }
            let mut not_s = false;
            if stage < 4 {
                c.cycles += 3;
                c.g[V0][0] = b's' as u64;
                not_s = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = (c.g[T1][0] < c.g[T8][0]) as u64;
            }
            if !not_s {
                let mut count_empty = false;
                if stage < 4 {
                    c.cycles += 2;
                    count_empty = (c.g[T4][0] as i64) <= 0;
                    c.g[V0][0] = 0x490000;
                }
                if !count_empty {
                    if stage < 4 {
                        c.cycles += 5;
                        c.g[A2][0] = 0u64;
                        c.g[T6][0] = sx(HEX_DECODE_HI);
                        c.g[T3][0] = b'%' as u64;
                        c.g[T5][0] = sx(HEX_DECODE_LO);
                    }
                    if stage == 4 {
                        // Resume at 0x003e_d880: pending `s` store, charged once.
                        c.cycles += 2;
                        let plain = c.g[V1][0] != c.g[T3][0];
                        c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                        if plain {
                            c.cycles += 2;
                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                            c.sb(c.g[V0][0] as u32, c.g[A0][0]);
                        } else {
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
                            c.sb(c.g[A2][0] as u32, c.g[V0][0]);
                        }
                        c.cycles += 1;
                        c.g[A2][0] = c.g[T2][0];
                    }
                    if stage < 5 {
                        loop {
                            c.cycles += 4;
                            c.g[V1][0] = c.lb(c.g[T0][0] as u32);
                            c.g[V0][0] = ((c.g[V1][0] as i64) < b'0' as i64) as u64;
                            let below_digit = c.g[V0][0] != 0u64;
                            c.g[A0][0] = c.lbu(c.g[T0][0] as u32);
                            let is_data = if below_digit {
                                c.cycles += 2;
                                let not_pct = c.g[V1][0] != c.g[T3][0];
                                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[T4][0] as i64)) as u64;
                                !not_pct
                            } else {
                                true
                            };
                            if !is_data {
                                break;
                            }
                            c.cycles += 5;
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            c.g[T2][0] = c.g[V0][0];
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T4][0] as i64)) as u64;
                            let has_room = c.g[V0][0] != 0u64;
                            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[T4][0] as i64)) as u64;
                            if !has_room {
                                break;
                            }
                            if back_edge_unwinds(c, HEAD_S) {
                                return;
                            }
                            c.cycles += 2;
                            let plain = c.g[V1][0] != c.g[T3][0];
                            c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            if plain {
                                c.cycles += 2;
                                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                c.sb(c.g[V0][0] as u32, c.g[A0][0]);
                            } else {
                                c.cycles += 11;
                                c.g[A1][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0x1));
                                c.g[A2][0] =
                                    sx((c.g[T1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                                c.g[A0][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0x2));
                                c.g[A1][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(c.g[T6][0] as u32));
                                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x3));
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[T5][0] as u32));
                                c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                                c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.sb(c.g[A2][0] as u32, c.g[V0][0]);
                            }
                            c.cycles += 1;
                            c.g[A2][0] = c.g[T2][0];
                        }
                    }
                    let mut filled = false;
                    if stage < 5 {
                        c.cycles += 2;
                        filled = c.g[V0][0] == 0u64;
                        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    }
                    if stage == 5 || !filled {
                        if stage < 5 {
                            c.cycles += 1;
                        }
                        loop {
                            c.cycles += 7;
                            c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[T4][0] as i64)) as u64;
                            let more_fill = c.g[V0][0] != 0u64;
                            c.sb(c.g[V1][0] as u32, 0u64);
                            if !more_fill {
                                break;
                            }
                            if back_edge_unwinds(c, HEAD_FILL) {
                                return;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[V0][0] = c.lb(c.g[T0][0] as u32);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V0][0] = c.g[V0][0] ^ b',' as u64;
                    if c.g[V0][0] == 0 {
                        c.g[T0][0] = c.g[A0][0];
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = (c.g[T1][0] < c.g[T8][0]) as u64;
            }
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                break;
            }
            c.cycles += 8;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = c.lb(c.g[A3][0] as u32);
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
            c.g[V0][0] = c.g[V0][0] ^ b'*' as u64;
            if c.g[V0][0] == 0 {
                c.g[A3][0] = c.g[V1][0];
            }
            c.g[A0][0] = c.lb(c.g[A3][0] as u32);
            let more_fmt = c.g[A0][0] != 0u64;
            c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
            if !more_fmt {
                c.cycles += 1;
                break;
            }
            if back_edge_unwinds(c, HEAD_FMT) {
                return;
            }
        }
    }
    frame.restore(c, S0, 0x430);
    c.cycles += 6;
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[T7][0] as u32));
    frame.restore(c, S1, 0x420);
    frame.restore(c, S2, 0x410);
    frame.restore(c, S3, 0x400);
    frame.close(c);
}

/// `int TagFieldGetEncoded(const char* data, char* buf, int len, const char* key, const char* dflt)`
/// (0x3ed970): decode a `$`-prefixed encrypted field using `key` (the default
/// key when `key` is null or empty), or fall back to GetString. Returns -1 for
/// null data or `len <= 0` with a buffer, the pair count with a null buffer,
/// the decoded byte count otherwise, and GetString's result for non-`$` data.
pub fn tag_field_get_encoded(c: &mut Cpu) {
    guest(c, |c| {
        const ENTRY: u32 = 0x003e_d970;
        const HEAD_COUNT: u32 = 0x003e_d9d8;
        const HEAD_DECODE: u32 = 0x003e_da60;
        const END: u32 = 0x003e_db04;
        let resume = c.pc;
        let mut data_null = false;
        let mut enter_count = resume == HEAD_COUNT;
        let mut enter_decode = resume == HEAD_DECODE;
        // The frame size must equal the size `Frame::open` uses on the entry arm.
        let frame = match resume {
            ENTRY => {
                c.cycles += 6;
                let frame = Frame::open(c, 0x10);
                c.g[T2][0] = c.g[A1][0];
                frame.save_ra(c, 0);
                c.g[T5][0] = c.g[A2][0];
                data_null = c.g[A0][0] == 0u64;
                c.g[T4][0] = c.g[A3][0];
                frame
            }
            HEAD_COUNT | HEAD_DECODE => Frame { size: 0x10 },
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        };
        if resume == ENTRY {
            if data_null {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            } else {
                c.cycles += 4;
                c.g[V1][0] = c.lb(c.g[A0][0] as u32);
                c.g[V0][0] = b'$' as u64;
                let is_dollar = c.g[V1][0] == c.g[V0][0];
                c.g[A1][0] = c.g[T2][0];
                if !is_dollar {
                    c.cycles += 5;
                    c.g[A2][0] = c.g[T5][0];
                    c.g[A3][0] = c.g[T0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e_d9a4)?;
                    frame.restore_ra(c, 0);
                    c.cycles += 2;
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 2;
                let key_null = c.g[T4][0] == 0u64;
                c.g[V0][0] = 0x4a0000;
                let mut use_fallback_key = key_null;
                if !key_null {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu(c.g[T4][0] as u32);
                    let key_nonempty = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    use_fallback_key = !key_nonempty;
                }
                if use_fallback_key {
                    c.cycles += 1;
                    c.g[T4][0] = sx(TAG_FALLBACK_KEY);
                }
                c.cycles += 2;
                let buf_present = c.g[T2][0] != 0u64;
                c.g[T3][0] = 0u64;
                enter_count = !buf_present;
                enter_decode = buf_present;
            }
        }
        if enter_count {
            // Null-buffer pair count. Resume at 0x003e_d9d8: pending count step,
            // charged once; state is in T0/T3.
            if resume == HEAD_COUNT {
                c.cycles += 2;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lbu(c.g[T0][0] as u32);
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            }
            loop {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < b'0' as u64) as u64;
                let stop = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.g[T3][0];
                if stop {
                    break;
                }
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[T0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[V0][0] < b'0' as u64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[T3][0];
                    break;
                }
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x2));
                if back_edge_unwinds(c, HEAD_COUNT) {
                    return Ok(());
                }
                c.cycles += 2;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lbu(c.g[T0][0] as u32);
            }
        }
        if enter_decode {
            let len_pos = resume == HEAD_DECODE || (c.g[T5][0] as i64) > 0;
            if resume == ENTRY {
                c.cycles += 2;
                c.g[T3][0] = 0x1;
            }
            if !len_pos {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            } else {
                // Resume at 0x003e_da60 enters the decode loop below directly:
                // state is in T0/T1/T2/T3/T4/T5/T6/T7/T8/A1.
                let at_limit = if resume == HEAD_DECODE {
                    false
                } else {
                    c.cycles += 5;
                    c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[T1][0] = c.g[T4][0];
                    c.g[V0][0] = ((c.g[T3][0] as i64) < (c.g[T5][0] as i64)) as u64;
                    c.g[T8][0] = 0u64;
                    c.g[V0][0] == 0u64
                };
                if at_limit {
                    c.cycles += 1;
                    c.sb(c.g[T2][0] as u32, 0u64);
                } else {
                    let mut bad_char = false;
                    let mut finish_with_nul = false;
                    if resume != HEAD_DECODE {
                        c.cycles += 4;
                        c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = (c.g[V0][0] < b'0' as u64) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.sb(c.g[T2][0] as u32, 0u64);
                            bad_char = true;
                        }
                        if !bad_char {
                            c.cycles += 4;
                            c.g[V0][0] = c.lbu((c.g[T0][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = (c.g[V0][0] < b'0' as u64) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.sb(c.g[T2][0] as u32, 0u64);
                                bad_char = true;
                            }
                        }
                    }
                    if !bad_char {
                        if resume != HEAD_DECODE {
                            c.cycles += 6;
                            c.g[V0][0] = 0x490000;
                            c.g[V1][0] = 0x490000;
                            c.g[T7][0] = sx(HEX_DECODE_HI);
                            c.g[T6][0] = sx(HEX_DECODE_LO);
                            c.g[A1][0] = c.lbu(c.g[T0][0] as u32);
                        }
                        loop {
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
                            let count_done = c.g[A3][0] == 0u64;
                            c.g[T8][0] = c.g[V0][0] | c.g[A1][0];
                            if count_done {
                                finish_with_nul = true;
                                break;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = c.lbu(c.g[T0][0] as u32);
                            c.g[V0][0] = (c.g[V0][0] < b'0' as u64) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.sb(c.g[T2][0] as u32, 0u64);
                                break;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = c.lbu((c.g[T0][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = (c.g[V0][0] < b'0' as u64) as u64;
                            if c.g[V0][0] != 0u64 {
                                finish_with_nul = true;
                                break;
                            }
                            c.g[A1][0] = c.lbu(c.g[T0][0] as u32);
                            if back_edge_unwinds(c, HEAD_DECODE) {
                                return Ok(());
                            }
                        }
                        if finish_with_nul {
                            c.cycles += 1;
                            c.sb(c.g[T2][0] as u32, 0u64);
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x1));
            }
        }
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `u32 TagFieldGetDate(const char* data, u32 dflt)` (0x3edb08): parse a date
/// tag (`$`-hex epoch, plain seconds, or `YYYY-MM-DD HH:MM:SS`) into seconds.
/// A tag that is unparseable, or that parses to 0, also falls back: it yields
/// `dflt`, or the current time when that is 0.
pub fn tag_field_get_date(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = 0u64;
        frame.save(c, S0, 0x80);
        c.g[S3][0] = c.g[A1][0];
        let data_null = c.g[S1][0] == 0u64;
        frame.save_ra(c, 0x40);
        if !data_null {
            c.cycles += 4;
            c.g[V1][0] = c.lb(c.g[S1][0] as u32);
            c.g[V0][0] = b'$' as u64;
            let not_dollar = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = c.lbu(c.g[S1][0] as u32);
            if !not_dollar {
                c.cycles += 5;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lb(c.g[S1][0] as u32);
                c.g[V0][0] = ((c.g[V0][0] as i64) < b'0' as i64) as u64;
                let has_hex = c.g[V0][0] == 0u64;
                c.g[A2][0] = c.lbu(c.g[S1][0] as u32);
                if has_hex {
                    c.cycles += 3;
                    c.g[V0][0] = 0x490000;
                    c.g[A3][0] = sx(HEX_DECODE_LO);
                    loop {
                        c.cycles += 11;
                        c.g[V0][0] = sx((c.g[A2][0] as u32) << 24);
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                        c.g[V1][0] = c.lb(c.g[S1][0] as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                        c.g[A1][0] = sx((c.g[S2][0] as u32) << 4);
                        c.g[A0][0] = c.lbu(c.g[V0][0] as u32);
                        c.g[V1][0] = ((c.g[V1][0] as i64) < b'0' as i64) as u64;
                        c.g[A2][0] = c.lbu(c.g[S1][0] as u32);
                        let more_hex = c.g[V1][0] == 0u64;
                        c.g[S2][0] = c.g[A1][0] | c.g[A0][0];
                        if !more_hex {
                            c.cycles += 2;
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003e_db60);
                        }
                    }
                }
            } else {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(b'0' as u32));
                c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                let not_digit = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[S1][0];
                if !not_digit {
                    let mut plain_done = false;
                    c.cycles += 6;
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    callee::TAG_FIELD_PARSE_DECIMAL.call(c, 0x003e_dbac)?;
                    c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 33i64) as u64;
                    if c.g[V1][0] == 0u64 {
                        c.g[V0][0] = c.lbu(c.g[S1][0] as u32);
                    } else {
                        c.cycles += 2;
                        c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                        plain_done = true;
                    }
                    if !plain_done {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(b'0' as u32));
                        c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                        // kept: original re-tests the byte here even though it is already known to be a digit (0x3edbcc).
                        let not_digit2 = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[SP][0];
                        if !not_digit2 {
                            c.cycles += 53;
                            c.g[A2][0] = TM_SIZE as u64;
                            c.g[A1][0] = 0u64;
                            c.g[S0][0] = 0x1;
                            callee::MEMSET.call(c, 0x003e_dbe4)?;
                            c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(TM_YEAR));
                            c.sw((c.g[SP][0] as u32).wrapping_add(TM_ISDST), c.g[V1][0]);
                            c.g[A0][0] = c.g[S1][0];
                            callee::TAG_FIELD_PARSE_DECIMAL.call(c, 0x003e_dbf8)?;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(TM_MON));
                            c.g[V0][0] = c.lbu(c.g[S1][0] as u32);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(b'-' as u32));
                            c.g[V0][0] = c.g[V0][0] & 0xff;
                            c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                            if c.g[V0][0] != 0 {
                                c.g[A0][0] = c.g[S1][0];
                            }
                            callee::TAG_FIELD_PARSE_DECIMAL.call(c, 0x003e_dc1c)?;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[SP][0] | TM_MDAY as u64;
                            c.g[V0][0] = c.lbu(c.g[S1][0] as u32);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(b'-' as u32));
                            c.g[V0][0] = c.g[V0][0] & 0xff;
                            c.g[S0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                            if c.g[S0][0] != 0 {
                                c.g[A0][0] = c.g[S1][0];
                            }
                            callee::TAG_FIELD_PARSE_DECIMAL.call(c, 0x003e_dc40)?;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[SP][0] | TM_HOUR as u64;
                            c.g[V0][0] = c.lb(c.g[S1][0] as u32);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = c.g[V0][0] ^ 0x20;
                            if c.g[V0][0] != 0 {
                                c.g[A0][0] = c.g[S1][0];
                            }
                            callee::TAG_FIELD_PARSE_DECIMAL.call(c, 0x003e_dc5c)?;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[SP][0] | TM_MIN as u64;
                            c.g[V0][0] = c.lb(c.g[S1][0] as u32);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = c.g[V0][0] ^ b':' as u64;
                            if c.g[V0][0] != 0 {
                                c.g[A0][0] = c.g[S1][0];
                            }
                            callee::TAG_FIELD_PARSE_DECIMAL.call(c, 0x003e_dc78)?;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[SP][0];
                            c.g[V0][0] = c.lb(c.g[S1][0] as u32);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = c.g[V0][0] ^ b':' as u64;
                            if c.g[V0][0] != 0 {
                                c.g[A0][0] = c.g[S1][0];
                            }
                            callee::TAG_FIELD_PARSE_DECIMAL.call(c, 0x003e_dc94)?;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(TM_YEAR));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(EPOCH_YEAR)); // EPOCH_YEAR = 1970
                            c.g[V1][0] = (c.g[V1][0] < EPOCH_YEARS) as u64; // valid years 1970..2099
                            let year_bad = c.g[V1][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(TM_MON));
                            let mut date_bad = year_bad;
                            if !year_bad {
                                c.cycles += 2;
                                let mon_le0 = (c.g[V0][0] as i64) <= 0;
                                c.g[V0][0] = ((c.g[V0][0] as i64) < 13i64) as u64;
                                if mon_le0 {
                                    date_bad = true;
                                } else {
                                    c.cycles += 2;
                                    let mon_bad = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(TM_MDAY));
                                    if mon_bad {
                                        date_bad = true;
                                    } else {
                                        c.cycles += 2;
                                        let mday_le0 = (c.g[V0][0] as i64) <= 0;
                                        c.g[V0][0] = ((c.g[V0][0] as i64) < 32i64) as u64;
                                        if mday_le0 {
                                            date_bad = true;
                                        } else {
                                            c.cycles += 2;
                                            let mday_bad = c.g[V0][0] == 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(TM_HOUR));
                                            if mday_bad {
                                                date_bad = true;
                                            }
                                        }
                                    }
                                }
                            }
                            if date_bad {
                                c.cycles += 2;
                                c.sw((c.g[SP][0] as u32).wrapping_add(TM_YEAR), 0u64);
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(TM_HOUR));
                            }
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[V0][0] < 0x18u64) as u64;
                            let hour_bad = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(TM_MIN));
                            let mut time_bad = hour_bad;
                            if !hour_bad {
                                c.cycles += 2;
                                let min_neg = (c.g[V0][0] as i64) < 0;
                                c.g[V0][0] = ((c.g[V0][0] as i64) < 60i64) as u64;
                                if min_neg {
                                    time_bad = true;
                                } else {
                                    c.cycles += 2;
                                    let min_bad = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(TM_SEC));
                                    if min_bad {
                                        time_bad = true;
                                    } else {
                                        c.cycles += 2;
                                        let sec_neg = (c.g[V0][0] as i64) < 0;
                                        c.g[V0][0] = ((c.g[V0][0] as i64) < 62i64) as u64;
                                        if sec_neg {
                                            time_bad = true;
                                        } else {
                                            c.cycles += 2;
                                            let sec_bad = c.g[V0][0] == 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(TM_YEAR));
                                            if sec_bad {
                                                time_bad = true;
                                            }
                                        }
                                    }
                                }
                            }
                            if time_bad {
                                c.cycles += 2;
                                c.sw((c.g[SP][0] as u32).wrapping_add(TM_YEAR), 0u64);
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(TM_YEAR));
                            }
                            c.cycles += 2;
                            let year_zero = c.g[V0][0] == 0u64;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(TM_MON));
                            if !year_zero {
                                c.cycles += 8;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(YEAR_BASE));
                                c.sw((c.g[SP][0] as u32).wrapping_add(TM_YEAR), c.g[V0][0]);
                                c.g[A0][0] = c.g[SP][0];
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                c.sw((c.g[SP][0] as u32).wrapping_add(TM_MON), c.g[V1][0]);
                                callee::DS_TIMETOSECS.call(c, 0x003e_dd24)?;
                                c.g[S2][0] = c.g[V0][0] << 32;
                                c.g[S2][0] = ((c.g[S2][0] as i64) >> 32) as u64;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[S2][0];
        if c.g[S2][0] == 0u64 {
            c.cycles += 3;
            c.g[S2][0] = c.g[S3][0];
            c.g[V0][0] = c.g[S2][0];
            if c.g[S2][0] == 0u64 {
                c.cycles += 4;
                callee::DS_TIMEINSECS.call(c, 0x003e_dd48)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = c.g[S2][0];
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `int TagFieldGetDateYMDHMS(char* data, int* y, int* mo, int* d, int* h, int* mi, int* s, int off)`
/// (0x3edd70): split a date tag into calendar fields. Each output is
/// stored only when its pointer is non-null; -1 when the tag yields no
/// usable time.
pub fn tag_field_get_date_ymdhms(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S1, 0x90);
        frame.save(c, S0, 0xa0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x80);
        c.g[A1][0] = 0x1;
        frame.save(c, S3, 0x70);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x60);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S5, 0x50);
        c.g[S4][0] = c.g[T0][0];
        frame.save(c, S6, 0x40);
        c.g[S5][0] = c.g[T1][0];
        frame.save_ra(c, 0x30);
        c.g[S6][0] = c.g[T2][0];
        c.g[S0][0] = c.g[T3][0];
        callee::TAG_FIELD_GET_DATE.call(c, 0x003e_ddb8)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
        let secs_too_small = c.g[V0][0] != 0u64;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if !secs_too_small {
            c.cycles += 8;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[A1][0] << 32;
            c.g[A1][0] = c.g[A1][0] >> 32;
            callee::DS_SECSTOTIME.call(c, 0x003e_dddc)?;
            c.g[V1][0] = c.g[V0][0];
            let tm_missing = c.g[V1][0] == 0u64;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            if !tm_missing {
                c.cycles += 2;
                if c.g[S1][0] != 0u64 {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(TM_YEAR));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(YEAR_BASE));
                    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                }
                c.cycles += 2;
                if c.g[S2][0] != 0u64 {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(TM_MON));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                }
                c.cycles += 2;
                if c.g[S3][0] != 0u64 {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(TM_MDAY));
                    c.sw(c.g[S3][0] as u32, c.g[V0][0]);
                }
                c.cycles += 2;
                if c.g[S4][0] != 0u64 {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(TM_HOUR));
                    c.sw(c.g[S4][0] as u32, c.g[V0][0]);
                }
                c.cycles += 2;
                if c.g[S5][0] != 0u64 {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(TM_MIN));
                    c.sw(c.g[S5][0] as u32, c.g[V0][0]);
                }
                c.cycles += 2;
                let no_sec_ptr = c.g[S6][0] == 0u64;
                c.g[V0][0] = 0u64;
                if !no_sec_ptr {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(TM_SEC));
                    c.sw(c.g[S6][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = 0u64;
                }
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `int TagFieldGetDatePacked(char* data, u32* date, u32* time, int off)` (0x3ede80):
/// split a date tag into packed date/time bit fields.
pub fn tag_field_get_date_packed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S0, 0x60);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x40);
        c.g[A1][0] = 0x1;
        frame.save_ra(c, 0x30);
        c.g[S2][0] = c.g[A2][0];
        c.g[S0][0] = c.g[A3][0];
        callee::TAG_FIELD_GET_DATE.call(c, 0x003e_dea8)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
        let secs_too_small = c.g[V0][0] != 0u64;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if !secs_too_small {
            c.cycles += 8;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[A1][0] << 32;
            c.g[A1][0] = c.g[A1][0] >> 32;
            callee::DS_SECSTOTIME.call(c, 0x003e_decc)?;
            c.g[A1][0] = c.g[V0][0];
            let tm_missing = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            if !tm_missing {
                c.cycles += 2;
                let have_date_ptr = c.g[S1][0] != 0u64;
                if have_date_ptr {
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(TM_YEAR));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(TM_MON));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(YEAR_BASE));
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(TM_MDAY));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                }
                c.cycles += 2;
                let no_time_ptr = c.g[S2][0] == 0u64;
                c.g[V0][0] = 0u64;
                if !no_time_ptr {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(TM_HOUR));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(TM_MIN));
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(TM_SEC));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = 0u64;
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::{check_events, driven, schedule, Events};

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
    /// separator (gp+0x1980), the name-class table (0x4964c0), Find's
    /// byte-fold table (0x4963c0) and the two hex tables (0x4961c0/0x4962c0
    /// decode, 0x495fc0/0x4960c0 encode). Stand-in values, not game strings.
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
            // Find compares folded bytes for equality only, so folding
            // a-z up to A-Z (the fold at 0x4963c0) makes its name compare
            // case-insensitive; without this the zeroed table would match
            // every name against the first field.
            c.bus
                .write8(TAG_CHAR_CLASS + i, (i as u8).to_ascii_uppercase());
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

    driven!(gen_set_raw, ssx3_ee::s14::f_003eb1e0__generated);
    driven!(hand_set_raw, tag_field_set_raw);

    // Re-enter `tag_field_set_raw` at every loop head: the seed class
    // (seed % 21) / 3 picks the input whose first back edge is that head,
    // with the delay skipping the earlier edges on paths that have them.
    // The common-prefix compare (3eb338), end scan (3eb390), skip (3eb3c0),
    // term-length (3eb450) and record-end (3eb488) classes run long inputs
    // so the target loop is still running when the event fires.
    #[test]
    fn tag_field_set_raw_events_matches() {
        check_events(gen_set_raw, hand_set_raw, 63, &|c, seed| {
            let mut r = machine(c, seed, 0x3eb1e0, 0);
            tables(c);
            let rec = DATA;
            let tmp = DATA + 0x400;
            let delay: u64 = match (seed % 21) / 3 {
                // Scan head: first back edge on the plain scan path.
                0 => {
                    put_str(c, rec, b"abcdef=1\nxy=2\n");
                    put_str(c, tmp, b"abcdef=9");
                    4
                }
                // Tilde walk head: `~` terms append at the end of the record.
                1 => {
                    put_str(c, rec, b"abcdef=1\nxy=2\n");
                    put_str(c, tmp, b"~new=1");
                    4
                }
                // Compare head: a 7-char common prefix keeps the compare
                // loop running past the scan head's check.
                2 => {
                    put_str(c, rec, b"aaaaaaa=1\n");
                    put_str(c, tmp, b"aaaaaaa=9");
                    40
                }
                // End-scan head: the old value is 100 chars, so the end
                // scan is still running when the event fires past the
                // six compare-loop edges (the `=` then matches and ends
                // the compare with no edge of its own).
                3 => {
                    put_str(c, rec, b"abcdef=");
                    for i in 0..100u32 {
                        wb(c, rec + 7 + i, b'1');
                    }
                    put_str(c, rec + 107, b"\n");
                    put_str(c, tmp, b"abcdef=9");
                    150
                }
                // Skip head: the first bytes mismatch in class only, so the
                // compare loop exits with no edge and the skip runs over a
                // 100-char non-matching field.
                4 => {
                    wb(c, rec, b'z');
                    for i in 0..100u32 {
                        wb(c, rec + 1 + i, b'1');
                    }
                    put_str(c, rec + 101, b"\n");
                    put_str(c, tmp, b"a=9");
                    60
                }
                // Term-length head: the tilde path skips the scan loops;
                // the walk over the short record ends well before the event
                // fires inside the 100-char term-length loop.
                5 => {
                    put_str(c, rec, b"a=1\n");
                    wb(c, tmp, b'~');
                    for i in 0..100u32 {
                        wb(c, tmp + 1 + i, b'9');
                    }
                    wb(c, tmp + 101, 0);
                    120
                }
                // Record-end head: the field is replaced in place and the
                // record tail is ~150 bytes, so the end loop is still
                // running when the event fires past the short prefix loops.
                _ => {
                    put_str(c, rec, b"a=1\n");
                    for k in 0..25u32 {
                        put_str(c, rec + 4 + 5 * k, b"zz=2\n");
                    }
                    put_str(c, tmp, b"a=9");
                    260
                }
            };
            c.g[A0][0] = sx(rec);
            c.g[A1][0] = 0x200;
            c.g[A2][0] = sx(tmp);
            let mode = schedule(c, &mut r, seed, 0x003e_b1e0);
            if mode != Events::Off {
                c.next_event = c.cycles + delay;
            }
            mode
        });
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

    driven!(gen_find, ssx3_ee::s14::f_003eb6d8__generated);
    driven!(hand_find, tag_field_find);

    // Re-enter `tag_field_find` at every loop head: the seed class
    // (seed % 12) / 3 picks the input whose first back edge is that head.
    // The back-match class (3eb798) walks back over a 50-char match, so the
    // back loop is still running when the event fires past the name-walk,
    // scan and match-attempt edges.
    #[test]
    fn tag_field_find_events_matches() {
        check_events(gen_find, hand_find, 36, &|c, seed| {
            let mut r = machine(c, seed, 0x3eb6d8, 0);
            tables(c);
            let rec = DATA;
            let name = DATA + 0x200;
            let delay: u64 = match (seed % 12) / 3 {
                // Name-walk head: first back edge on the plain lookup path.
                0 => {
                    put_str(c, rec, b"alpha=1\nbeta=2\n");
                    put_str(c, name, b"beta");
                    4
                }
                // Match-attempt head: a 1-char name walks no loop, and `=`
                // is the record's first byte, so the match attempt is the
                // first back edge (it misses: the `=` is before the record).
                1 => {
                    put_str(c, rec, b"=1\n");
                    put_str(c, name, b"b");
                    4
                }
                // Back-match head: the 50-char name matches the 50-char
                // field-name suffix, so the back-match loop is still
                // running when the event fires past the name-walk, scan
                // and match-attempt edges.
                2 => {
                    wb(c, rec, b'a');
                    for i in 0..50u32 {
                        wb(c, rec + 1 + i, b'b');
                    }
                    put_str(c, rec + 51, b"=1\n");
                    for i in 0..50u32 {
                        wb(c, name + i, b'b');
                    }
                    wb(c, name + 50, 0);
                    1200
                }
                // Scan head: a 1-char name walks no loop, so the record
                // scan is the first back edge.
                _ => {
                    for i in 0..50u32 {
                        wb(c, rec + i, b'a');
                    }
                    put_str(c, rec + 50, b"=1\n");
                    put_str(c, name, b"z");
                    4
                }
            };
            c.g[A0][0] = sx(rec);
            c.g[A1][0] = sx(name);
            let mode = schedule(c, &mut r, seed, 0x003e_b6d8);
            if mode != Events::Off {
                c.next_event = c.cycles + delay;
            }
            mode
        });
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
    //
    // The tag name lives at TAG_NAME_ADDR: without it TagFieldFind returns
    // 0 on every seed and the skip loop is never compared. The byte after
    // `=` is steered across the 0x20/0x21 boundary: `!` (0x21) stops the
    // loop under `<` but not under `<=` (mutation tags.rs:2344 `<` -> `<=`).
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
                put_str(c, TAG_NAME_ADDR, b"tag");
                put_str(
                    c,
                    rec,
                    match seed % 4 {
                        0 => b"tag=  value\n",
                        1 => b"tag=!value\n",
                        2 => b"tag=value\n",
                        _ => b"nope\n",
                    },
                );
            },
        );
    }

    driven!(gen_find_skip_ws, ssx3_ee::s14::f_003eb8b8__generated);
    driven!(hand_find_skip_ws, tag_field_find_skip_ws);

    #[test]
    fn tag_field_find_skip_ws_events_matches() {
        check_events(gen_find_skip_ws, hand_find_skip_ws, 36, &|c, seed| {
            let mut r = machine(c, seed, 0x3eb8b8, 0);
            tables(c);
            let rec = DATA;
            put_str(c, rec, b"tag=      value\n");
            put_str(c, 0x004a_4a78, b"tag");
            c.g[A0][0] = sx(rec);
            let mode = schedule(c, &mut r, seed, 0x003e_b8b8);
            // Due after the Find call returns, inside the whitespace loop
            // (window 126..168).
            if mode != Events::Off {
                c.next_event = c.cycles + 150;
            }
            mode
        });
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

    driven!(gen_delete, ssx3_ee::s14::f_003eb918__generated);
    driven!(hand_delete, tag_field_delete);

    // Re-enter `tag_field_delete` at every loop head: every delay is past
    // the Find call's return (an event due inside Find would unwind at one
    // of Find's own heads), and the seed class (seed % 12) / 3 picks the
    // input and delay landing the first unwind in each of Delete's loops.
    // The walk (3eb940), value (3eb960) and separator (3eb988) classes run
    // long loops so the target loop is still running when the event fires.
    #[test]
    fn tag_field_delete_events_matches() {
        check_events(gen_delete, hand_delete, 36, &|c, seed| {
            let mut r = machine(c, seed, 0x3eb918, 0);
            tables(c);
            let rec = DATA;
            let name = DATA + 0x200;
            let delay: u64 = match (seed % 12) / 3 {
                // Walk head: a 60-char field name walks back 60 positions.
                0 => {
                    for i in 0..60u32 {
                        wb(c, rec + i, b'n');
                    }
                    put_str(c, rec + 60, b"=1\neb=2\n");
                    for i in 0..60u32 {
                        wb(c, name + i, b'n');
                    }
                    wb(c, name + 60, 0);
                    1800
                }
                // Value-scan head: the 100-char value keeps the scan
                // running past the short walk.
                1 => {
                    put_str(c, rec, b"bb=");
                    for i in 0..100u32 {
                        wb(c, rec + 3 + i, b'2');
                    }
                    put_str(c, rec + 103, b"\ncc=3\n");
                    put_str(c, name, b"bb");
                    300
                }
                // Separator-scan head: 21 newlines keep the separator scan
                // running past the short walk and value scans.
                2 => {
                    put_str(c, rec, b"bb=2\n");
                    for i in 0..20u32 {
                        wb(c, rec + 5 + i, b'\n');
                    }
                    put_str(c, rec + 25, b"cc=3\n");
                    put_str(c, name, b"bb");
                    260
                }
                // Copy head: the previous shape (due after the Find call
                // returns, inside Delete's own loops: the 0x3eb9b0 window
                // is 206..222 with Find's fold table filled).
                _ => {
                    put_str(c, rec, b"aa=1\nbb=2\ncc=3\n");
                    put_str(c, name, b"bb");
                    214
                }
            };
            c.g[A0][0] = sx(rec);
            c.g[A1][0] = sx(name);
            let mode = schedule(c, &mut r, seed, 0x003e_b918);
            if mode != Events::Off {
                c.next_event = c.cycles + delay;
            }
            mode
        });
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

    driven!(gen_printf, ssx3_ee::s14::f_003ecbd8__generated);
    driven!(hand_printf, tag_field_printf);

    // Re-enter `tag_field_printf` at every loop head: the seed class
    // (seed % 9) / 3 picks the format whose first back edge is that head.
    // All formats stay within plain characters, whitespace runs and `%r`
    // (whose raw-copy loop is printf's own): no conversion in any class
    // calls another port before the first unwind lands.
    #[test]
    fn tag_field_printf_events_matches() {
        check_events(gen_printf, hand_printf, 36, &|c, seed| {
            let mut r = machine(c, seed, 0x3ecbd8, 0);
            tables(c);
            let fmt = DATA + 0x200;
            let delay: u64 = match (seed % 9) / 3 {
                // Format head: first back edge on the plain format path.
                0 => {
                    put_str(c, fmt, b"a   b   c   d");
                    put_str(c, DATA + 0x400, b"v");
                    c.g[A3][0] = sx(DATA + 0x400);
                    4
                }
                // Whitespace head: a plain `a` first (its format-loop edge
                // is skipped by the delay), then a 30-space run keeps the
                // whitespace skip running when the event fires.
                1 => {
                    wb(c, fmt, b'a');
                    for i in 0..30u32 {
                        wb(c, fmt + 1 + i, b' ');
                    }
                    put_str(c, fmt + 31, b"b");
                    put_str(c, DATA + 0x400, b"v");
                    c.g[A3][0] = sx(DATA + 0x400);
                    48
                }
                // Raw-copy head: `%r` copies a 200-char raw string in
                // printf's own loop, still running when the event fires
                // past the format-loop edge and the conversion dispatch.
                _ => {
                    put_str(c, fmt, b"a%r");
                    for i in 0..200u32 {
                        wb(c, DATA + 0x500 + i, b'R');
                    }
                    wb(c, DATA + 0x500 + 200, 0);
                    c.g[A3][0] = sx(DATA + 0x500);
                    100
                }
            };
            put_str(c, DATA, b"");
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = 0x400;
            c.g[A2][0] = sx(fmt);
            let mode = schedule(c, &mut r, seed, 0x003e_cbd8);
            if mode != Events::Off {
                c.next_event = c.cycles + delay;
            }
            mode
        });
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

    driven!(gen_get_delim, ssx3_ee::s14::f_003ed0f8__generated);
    driven!(hand_get_delim, tag_field_get_delim);

    // Re-enter `tag_field_get_delim` at every loop head: the seed class
    // (seed % 18) / 3 picks the input whose first back edge is that head,
    // each with the event due 4 cycles in. Buffers stay non-null on every
    // path that reaches the default copy (the null-buffer default path
    // would call STRLEN, whose own back edges would unwind mid-call).
    #[test]
    fn tag_field_get_delim_events_matches() {
        check_events(gen_get_delim, hand_get_delim, 54, &|c, seed| {
            let mut r = machine(c, seed, 0x3ed0f8, 0);
            tables(c);
            let buf = DATA + 0x200;
            put_str(c, DATA + 0x400, b"default");
            let data_null = (seed % 18) / 3 == 3;
            if !data_null {
                match (seed % 18) / 3 {
                    // Skip-fail head: a control byte first character ends
                    // the field scan with no delimiter in sight.
                    0 => put_str(c, DATA, b"\nabc"),
                    // Skip-next head: first back edge on the plain skip path.
                    1 => put_str(c, DATA, b"a,b,c,d,e,f"),
                    // Scan head: the empty first field reaches the
                    // field-boundary edge with no advance edge before it.
                    2 => put_str(c, DATA, b",b,c"),
                    // Measure head: index 0 skips no fields, and the null
                    // buffer takes the measure-only path (a pure loop).
                    4 => {
                        for i in 0..100u32 {
                            wb(c, DATA + i, b'a');
                        }
                        wb(c, DATA + 100, 0);
                    }
                    // Copy head: the value copies 100 bytes before the
                    // delimiter stops it.
                    _ => {
                        for i in 0..100u32 {
                            wb(c, DATA + i, b'a');
                        }
                        put_str(c, DATA + 100, b",tail");
                    }
                }
            }
            c.g[A0][0] = if data_null { 0 } else { sx(DATA) };
            c.g[A1][0] = if (seed % 18) / 3 == 4 { 0 } else { sx(buf) };
            c.g[A2][0] = 0x40;
            c.g[A3][0] = sx(DATA + 0x400);
            c.g[T0][0] = match (seed % 18) / 3 {
                0 => 1,
                1 => 2,
                2 => 2,
                4 => 0,
                _ => 0,
            };
            c.g[T1][0] = b',' as u64;
            let mode = schedule(c, &mut r, seed, 0x003e_d0f8);
            if mode != Events::Off {
                c.next_event = c.cycles + 4;
            }
            mode
        });
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

    driven!(gen_get_string, ssx3_ee::s14::f_003ed378__generated);
    driven!(hand_get_string, tag_field_get_string);

    // Re-enter `tag_field_get_string` at every loop head: the seed class
    // (seed % 9) / 3 picks the input whose first back edge is that head,
    // each with the event due 4 cycles in. The buffer stays non-null on
    // the default path (the null-buffer default path would call STRLEN,
    // whose own back edges would unwind mid-call).
    #[test]
    fn tag_field_get_string_events_matches() {
        check_events(gen_get_string, hand_get_string, 36, &|c, seed| {
            let mut r = machine(c, seed, 0x3ed378, 0);
            tables(c);
            put_str(c, DATA + 0x400, b"default");
            let class = (seed % 9) / 3;
            // Default head: null data copies the 40-char default.
            // Measure head: the null buffer takes the measure-only path.
            // Copy head: the 100-char value copies before the NUL stops it.
            if class == 0 {
                for i in 0..40u32 {
                    wb(c, DATA + 0x400 + i, b'd');
                }
                wb(c, DATA + 0x400 + 40, 0);
            } else {
                for i in 0..100u32 {
                    wb(c, DATA + i, b'a');
                }
                wb(c, DATA + 100, 0);
            }
            c.g[A0][0] = if class == 0 { 0 } else { sx(DATA) };
            c.g[A1][0] = if class == 1 { 0 } else { sx(DATA + 0x200) };
            c.g[A2][0] = 0x40;
            c.g[A3][0] = sx(DATA + 0x400);
            let mode = schedule(c, &mut r, seed, 0x003e_d378);
            if mode != Events::Off {
                c.next_event = c.cycles + 4;
            }
            mode
        });
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

    driven!(gen_get_binary, ssx3_ee::s14::f_003ed678__generated);
    driven!(hand_get_binary, tag_field_get_binary);

    // Re-enter `tag_field_get_binary` at every loop head: the seed class
    // (seed % 18) / 3 picks a format whose first back edge is that head
    // (skip, digit, hex, `s` copy, zero fill, next format letter), with an
    // `s` letter and a `%`-escaped value in the copy classes. The event is
    // due a few cycles in (later for the fill class, whose copy loop must
    // finish first), so an unwind seed re-enters at the target head.
    #[test]
    fn tag_field_get_binary_events_matches() {
        check_events(gen_get_binary, hand_get_binary, 72, &|c, seed| {
            let mut r = machine(c, seed, 0x3ed678, 0);
            tables(c);
            let data = DATA;
            let fmt = DATA + 0x200;
            let (fmt_text, data_text, delay): (&[u8], &[u8], u64) = match (seed % 18) / 3 {
                0 => (b"#fieldname=6s", b"%42ABCDE", 4),
                1 => (b"123b", b"41424344", 4),
                2 => (b"l", b"41424344", 4),
                3 => (b"6s", b"%42ABCDE", 4),
                4 => (b"8s", b"AB", 100),
                _ => (b"1a1a1a", b"1234", 4),
            };
            put_str(c, fmt, fmt_text);
            put_str(c, data, data_text);
            c.g[A0][0] = sx(data);
            c.g[A1][0] = sx(DATA + 0x400);
            c.g[A2][0] = 0x40;
            c.g[A3][0] = sx(fmt);
            let mode = schedule(c, &mut r, seed, 0x003e_d678);
            if mode != Events::Off {
                c.next_event = c.cycles + delay;
            }
            mode
        });
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

    driven!(gen_get_encoded, ssx3_ee::s14::f_003ed970__generated);
    driven!(hand_get_encoded, tag_field_get_encoded);

    // Re-enter `tag_field_get_encoded` at both loop heads: the seed class
    // (seed % 6) / 3 picks the null-buffer pair count (3ed9d8, a pure loop
    // over 50 hex pairs) or the decode loop (3eda60, the previous shape),
    // each with the event due 4 cycles in.
    #[test]
    fn tag_field_get_encoded_events_matches() {
        check_events(gen_get_encoded, hand_get_encoded, 36, &|c, seed| {
            let mut r = machine(c, seed, 0x3ed970, 0);
            tables(c);
            let class = (seed % 6) / 3;
            if class == 0 {
                wb(c, DATA, b'$');
                for i in 0..50u32 {
                    wb(c, DATA + 1 + 2 * i, b'4');
                    wb(c, DATA + 2 + 2 * i, b'1');
                }
                wb(c, DATA + 101, 0);
            } else {
                put_str(c, DATA, b"$4142434445464748");
            }
            put_str(c, DATA + 0x400, b"key");
            put_str(c, DATA + 0x600, b"default");
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = if class == 0 { 0 } else { sx(DATA + 0x200) };
            c.g[A2][0] = 0x40;
            c.g[A3][0] = sx(DATA + 0x400);
            c.g[T0][0] = sx(DATA + 0x600);
            let mode = schedule(c, &mut r, seed, 0x003e_d970);
            if mode != Events::Off {
                c.next_event = c.cycles + 4;
            }
            mode
        });
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
