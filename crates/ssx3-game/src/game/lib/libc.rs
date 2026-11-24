//! `src/lib/libc.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::ret;
use crate::call;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::splashmanager::at;
pub(crate) use crate::game::lib::gcc::frame_ld;
pub(crate) use crate::game::lib::gcc::frame_sd;
pub(crate) use crate::game::lib::gcc::MULDF3;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002c_71c0, date_is_leap_year, "Date_IsLeapYear"),
    (0x002c_7328, time_add_seconds, "Time_AddSeconds"),
    (0x0041_6fc0, cleanup_r, "_cleanup_r"),
    (0x0041_6fe0, cleanup, "_cleanup"),
    (0x0041_8958, mbtowc_r, "_mbtowc_r"),
    (0x0041_b658, vfprintf_dispatch, "vfprintf"),
    (0x0041_ca70, vfiprintf_r, "_vfiprintf_r"),
    (0x0041_ec08, localeconv_r, "_localeconv_r"),
    (0x0041_fb98, close_r, "_close_r"),
    (0x0041_fbf0, fstat_r, "_fstat_r"),
    (0x0041_fc50, lseek_r, "_lseek_r"),
    (0x0041_fcb0, open_r, "_open_r"),
    (0x0041_fd10, read_r, "_read_r"),
    (0x0041_fd70, sbrk_r, "_sbrk_r"),
    (0x0041_fdd0, write_r, "_write_r"),
    (0x0042_05b8, bfree, "_Bfree"),
    (0x0042_14c0, mprec_log10, "_mprec_log10"),
    (0x0042_4558, fstat, "_fstat"),
    (0x0042_4578, kill, "_kill"),
    (0x0041_1cd8, fpadd_parts_d, "_fpadd_parts_d"),
    (0x0041_6eb0, fopen_r, "_fopen_r"),
    (0x0041_7418, fwalk, "_fwalk"),
    (0x0041_7578, printf_r, "_printf_r"),
    (0x0041_77b0, sprintf_r, "_sprintf_r"),
    (0x0041_a840, strtoul_r, "_strtoul_r"),
    (0x0041_ae00, vfprintf_round_digits, "vfprintf_RoundDigits"),
    (0x0041_ae70, vfprintf_str_reverse, "vfprintf_StrReverse"),
    (0x0041_aee8, vfprintf_int_to_str, "vfprintf_IntToStr"),
    (0x0041_b528, vfprintf_flush_file, "vfprintf_FlushFile"),
    (
        0x0041_b550,
        vfprintf_buffered_write,
        "vfprintf_BufferedWrite",
    ),
    (
        0x0041_c788,
        vfprintf_insert_thousands_sep,
        "vfprintf_InsertThousandsSep",
    ),
    (0x0041_e340, free_r, "_free_r"),
    (0x0041_e7a8, strtol_r, "_strtol_r"),
    (0x0041_ea38, setlocale_r, "_setlocale_r"),
    (0x0042_1b98, _gcvt, "_gcvt"),
    (0x0042_24a0, calloc_r, "_calloc_r"),
    (0x0041_610c, memmove, "memmove"),
    (0x0041_62d0, strcat, "strcat"),
    (0x0041_6958, strncat, "strncat"),
    (0x0041_7008, fseek, "fseek"),
    (0x0041_74b0, smakebuf, "__smakebuf"),
    (0x0041_75c8, printf, "printf"),
    (0x0041_7628, lflush, "lflush"),
    (0x0041_7648, srefill, "__srefill"),
    (0x0041_78b0, sscanf, "sscanf"),
    (0x0041_85d0, sccl, "__sccl"),
    (0x0041_86c8, vsprintf, "vsprintf"),
    (0x0041_8720, vsnprintf, "vsnprintf"),
    (0x0041_8790, abort, "abort"),
    (0x0041_87a0, atof, "atof"),
    (0x0041_87c0, atoi, "atoi"),
    (0x0041_87e8, bsearch, "bsearch"),
    (0x0041_88c0, malloc, "malloc"),
    (0x0041_8910, free, "free"),
    (0x0041_8d48, fcvt, "fcvt"),
    (0x0041_8d68, fcvtf, "fcvtf"),
    (0x0041_8db8, gcvtf, "gcvtf"),
    (0x0041_8df8, ecvt, "ecvt"),
    (0x0041_8e18, ecvtf, "ecvtf"),
    (0x0041_8e68, gcvt, "gcvt"),
    (0x0041_a7e8, strtod, "strtod"),
    (0x0041_a818, strtof, "strtof"),
    (0x0041_aa50, strtoul, "strtoul"),
    (0x0041_ab08, strcspn, "strcspn"),
    (0x0041_ac40, strpbrk, "strpbrk"),
    (0x0041_acc0, strrchr, "strrchr"),
    (0x0041_ad10, strspn, "strspn"),
    (0x0041_ad80, strstr, "strstr"),
    (
        0x0041_c938,
        vfiprintf_buffered_write,
        "vfiprintf_BufferedWrite",
    ),
    (0x0041_ca40, vfiprintf, "vfiprintf"),
    (0x0041_d990, malloc_extend_top, "malloc_extend_top"),
    (0x0041_e638, malloc_trim, "malloc_trim"),
    (0x0041_e9e0, strtol, "strtol"),
    (0x0041_ec48, localeconv, "localeconv"),
    (0x0041_ec70, memchr, "memchr"),
    (0x0041_fe30, fflush, "fflush"),
    (0x0041_ff40, std, "std"),
    (0x0041_ff98, sfmoreglue, "__sfmoreglue"),
    (0x0042_0010, sfp, "__sfp"),
    (0x0042_00f0, sinit, "__sinit"),
    (0x0042_0228, fread_bcopy, "fread_bcopy"),
    (0x0042_0260, fread, "fread"),
    (0x0042_0378, ungetc, "ungetc"),
    (0x0042_0978, i2b, "__i2b"),
    (0x0042_1530, print_f, "print_f"),
    (0x0042_16e8, print_e, "print_e"),
    (0x0042_18d8, fcvtbuf, "fcvtbuf"),
    (0x0042_1a50, ecvtbuf, "ecvtbuf"),
    (0x0042_2560, quorem, "quorem"),
];

// ---------------------------------------------------------------------------
// Callees (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::ai::computer::callee::VFIPRINTF_R;
    pub(crate) use crate::game::ai::computer::callee::VFPRINTF_R;

    /// `size_t strlen(const char*)` (`a0` in, `v0` out).
    pub const STRLEN: Callee = Callee {
        addr: 0x0041_6810,
        f: ee::s15::strlen,
    };

    /// `int strcmp(const char*, const char*)` (`a0`, `a1` in, `v0` out).
    pub const STRCMP: Callee = Callee {
        addr: 0x0041_65a8,
        f: ee::s15::strcmp,
    };

    /// Converts one multibyte character of the format (`wc`, `mbstate`).
    pub const MBTOWC_R: Callee = Callee {
        addr: 0x0041_8958,
        f: ee::s15::_mbtowc_r,
    };

    /// Copies a literal run / a padding chunk into the output buffer.
    pub const MEMCPY: Callee = Callee {
        addr: 0x0041_605c,
        f: ee::s15::memcpy,
    };

    /// Bounded `strlen` for `%s` with a precision.
    pub const MEMCHR: Callee = Callee {
        addr: 0x0041_ec70,
        f: ee::s15::memchr,
    };

    /// Pushes bytes into the (unbuffered) file's log buffer.
    pub const BUFFERED_WRITE: Callee = Callee {
        addr: 0x0041_c938,
        f: ee::s15::vfiprintf_BufferedWrite,
    };

    /// Inserts the thousands separators for the `'\''` flag.
    pub const INSERT_THOUSANDS_SEP: Callee = Callee {
        addr: 0x0041_c788,
        f: ee::s15::vfprintf_InsertThousandsSep,
    };

    /// 64-bit unsigned division / modulo for `%d`, `%u` and friends.
    pub const UMODDI3: Callee = Callee {
        addr: 0x0042_dad8,
        f: ee::s15::__umoddi3,
    };

    pub const UDIVDI3: Callee = Callee {
        addr: 0x0041_1538,
        f: ee::s15::__udivdi3,
    };
    pub(crate) use crate::game::misc::callee::BALLOC;
    pub(crate) use crate::game::misc::callee::BUFFERED_WRITE as VFPRINTF_BUFFERED_WRITE;
    pub(crate) use crate::game::misc::callee::CMPDF2;
    pub(crate) use crate::game::misc::callee::DTOA_R;
    pub(crate) use crate::game::misc::callee::ECVT;
    pub(crate) use crate::game::misc::callee::ECVTBUF;
    pub(crate) use crate::game::misc::callee::EXTENDSFDF2;
    pub(crate) use crate::game::misc::callee::FCVT;
    pub(crate) use crate::game::misc::callee::FCVTBUF;
    pub(crate) use crate::game::misc::callee::FFLUSH;
    pub(crate) use crate::game::misc::callee::FREAD_BCOPY;
    pub(crate) use crate::game::misc::callee::FREE_R;
    pub(crate) use crate::game::misc::callee::FSEEK;
    pub(crate) use crate::game::misc::callee::FSTAT_R;
    pub(crate) use crate::game::misc::callee::FWALK;
    pub(crate) use crate::game::misc::callee::GCVT;
    pub(crate) use crate::game::misc::callee::GCVT_DISPATCH;
    pub(crate) use crate::game::misc::callee::LOCALECONV_R;
    pub(crate) use crate::game::misc::callee::MALLOC_LOCK;
    pub(crate) use crate::game::misc::callee::MALLOC_R;
    pub(crate) use crate::game::misc::callee::MALLOC_TRIM;
    pub(crate) use crate::game::misc::callee::MALLOC_UNLOCK;
    pub(crate) use crate::game::misc::callee::MCMP;
    pub(crate) use crate::game::misc::callee::MEMSET;
    pub(crate) use crate::game::misc::callee::MPREC_LOG10;
    pub(crate) use crate::game::misc::callee::MULDI3;
    pub(crate) use crate::game::misc::callee::OPEN_R;
    pub(crate) use crate::game::misc::callee::PRINT_E;
    pub(crate) use crate::game::misc::callee::REALLOC_R;
    pub(crate) use crate::game::misc::callee::SBRK_R;
    pub(crate) use crate::game::misc::callee::SFLAGS;
    pub(crate) use crate::game::misc::callee::SFMOREGLUE;
    pub(crate) use crate::game::misc::callee::SFP;
    pub(crate) use crate::game::misc::callee::SINIT;
    pub(crate) use crate::game::misc::callee::SMAKEBUF;
    pub(crate) use crate::game::misc::callee::SREFILL;
    pub(crate) use crate::game::misc::callee::STD;
    pub(crate) use crate::game::misc::callee::STRCPY;
    pub(crate) use crate::game::misc::callee::STRTOD;
    pub(crate) use crate::game::misc::callee::STRTOD_R;
    pub(crate) use crate::game::misc::callee::STRTOL;
    pub(crate) use crate::game::misc::callee::STRTOL_R;
    pub(crate) use crate::game::misc::callee::STRTOUL_R;
    pub(crate) use crate::game::misc::callee::STR_REVERSE;
    pub(crate) use crate::game::misc::callee::SUBDF3;
    pub(crate) use crate::game::misc::callee::SVFSCANF;
    pub(crate) use crate::game::misc::callee::TRUNCDFSF2;
    pub(crate) use crate::game::misc::callee::VFPRINTF_DISPATCH;
    pub(crate) use crate::game::misc::callee::WRITE;
}

// ---------------------------------------------------------------------------
// Block labels. The generated body dispatches on `pc`, so these are the
// names the arms answer to (the original jumps between them with a computed
// jump, and the event checks resume at `MINUS1` and `JIS_NEXT_BYTE`).

pub(crate) mod l {

    pub const ENTRY: u32 = 0x0041_8958;

    /// `n == 0`: -1.
    pub const N_ZERO: u32 = 0x0041_8990;

    /// Fetch the locale name; a null name is a single-byte locale.
    pub const LOCALE: u32 = 0x0041_8998;

    /// `strlen(name) < 2` is a single-byte locale.
    pub const LOCALE_LEN: u32 = 0x0041_89a4;

    /// `strcmp(name, "C-SJIS")`.
    pub const CMP_SJIS: u32 = 0x0041_89b8;

    /// `C-SJIS`: *s is not NUL; return 0 for a null string.
    pub const SJIS_HEAD: u32 = 0x0041_89cc;

    /// Lead byte 0x81..0x9f?
    pub const SJIS_LEAD_HI: u32 = 0x0041_89d8;

    /// Lead byte 0xe0..0xef?
    pub const SJIS_LEAD_LO: u32 = 0x0041_89e8;

    /// Load the trail byte, reject an incomplete sequence.
    pub const SJIS_TRAIL: u32 = 0x0041_89f8;

    /// Return -1 (invalid or incomplete).
    pub const MINUS1: u32 = 0x0041_8a00;

    /// Trail byte 0x40..0x7e?
    pub const SJIS_TRAIL1: u32 = 0x0041_8a08;

    /// Trail byte 0x80..0xfc?
    pub const SJIS_TRAIL2: u32 = 0x0041_8a18;

    /// Return -1 (invalid trail).
    pub const MINUS1_TRAIL: u32 = 0x0041_8a28;

    /// `strcmp(name, "C-EUCJP")`.
    pub const CMP_EUCJP: u32 = 0x0041_8a30;

    /// `C-EUCJP`: *s is not NUL; return 0 for a null string.
    pub const EUC_HEAD: u32 = 0x0041_8a44;

    /// Lead byte 0xa1..0xfe? (else single byte.)
    pub const EUC_LEAD: u32 = 0x0041_8a4c;

    /// Reject an incomplete sequence, else classify the trail byte.
    pub const EUC_TRAIL: u32 = 0x0041_8a5c;

    /// Trail byte 0xa1..0xfe?
    pub const EUC_TRAIL_CLASS: u32 = 0x0041_8a64;

    /// Load the lead byte for the two-byte pack.
    pub const EUC_LEAD_BYTE: u32 = 0x0041_8a74;

    /// `*pwc = lead << 8 | trail`, return 2.
    pub const TWO_BYTES: u32 = 0x0041_8a78;

    /// `strcmp(name, "C-JIS")`.
    pub const CMP_JIS: u32 = 0x0041_8a90;

    /// `C-JIS`: a null string resets the state and returns 1.
    pub const JIS_STATE: u32 = 0x0041_8aa4;

    /// Reset the mbstate, return 1.
    pub const JIS_RESET: u32 = 0x0041_8aac;

    /// Start scanning at offset 0; `t1` selects the state row.
    pub const JIS_SETUP: u32 = 0x0041_8ab8;

    /// `a3 = s + offset`.
    pub const JIS_NEXT_BYTE: u32 = 0x0041_8acc;

    /// Classify the current input byte.
    pub const JIS_CLASSIFY: u32 = 0x0041_8ad0;

    /// `c < ')'`?
    pub const JIS_LT_41: u32 = 0x0041_8ae0;

    /// ESC?
    pub const JIS_ESC: u32 = 0x0041_8aec;

    /// `c < 0x1c`?
    pub const JIS_LT_28: u32 = 0x0041_8af4;

    /// NUL?
    pub const JIS_NUL: u32 = 0x0041_8afc;

    /// Class 7 or 8 depending on `0x5d < c - 0x21`.
    pub const JIS_CLASS_5D_A: u32 = 0x0041_8b04;

    /// `$`?
    pub const JIS_DOLLAR: u32 = 0x0041_8b0c;

    /// Class 7 or 8 (second copy).
    pub const JIS_CLASS_5D_B: u32 = 0x0041_8b14;

    /// `c < 'C'`, `B`?
    pub const JIS_GE_41: u32 = 0x0041_8b1c;

    /// `@`?
    pub const JIS_GE_43: u32 = 0x0041_8b28;

    /// `@` class 3 or 7/8.
    pub const JIS_AT: u32 = 0x0041_8b30;

    /// Class 7 or 8 (third copy).
    pub const JIS_CLASS_5D_C: u32 = 0x0041_8b38;

    /// `J`?
    pub const JIS_GE_67: u32 = 0x0041_8b40;

    /// Class 7 or 8 (fourth copy).
    pub const JIS_CLASS_5D_D: u32 = 0x0041_8b4c;

    /// Class 0 (ESC).
    pub const CLASS_ESC: u32 = 0x0041_8b54;

    /// Class 1 (`$`).
    pub const CLASS_DOLLAR: u32 = 0x0041_8b5c;

    /// Class 3 (`@`).
    pub const CLASS_AT: u32 = 0x0041_8b64;

    /// Class 2 (`(`).
    pub const CLASS_LPAREN: u32 = 0x0041_8b6c;

    /// Class 4 (`B`).
    pub const CLASS_B: u32 = 0x0041_8b74;

    /// Class 5 (`J`).
    pub const CLASS_J: u32 = 0x0041_8b7c;

    /// Class 6 (anything else).
    pub const CLASS_OTHER: u32 = 0x0041_8b84;

    /// Class 7 or 8 (shared tail).
    pub const CLASS_5D: u32 = 0x0041_8b8c;

    /// Look the class up in the two `C-JIS` tables.
    pub const JIS_TABLE: u32 = 0x0041_8b9c;

    /// Jump through the handler table.
    pub const JIS_DISPATCH: u32 = 0x0041_8bd4;

    /// Reset the state, `*pwc = 0`, return the scan offset.
    pub const JIS_RESET_ZERO: u32 = 0x0041_8bf0;

    /// Reset the state, return one byte.
    pub const JIS_ONE_BYTE: u32 = 0x0041_8c00;

    /// Reset the state, store two bytes, return `offset + 1`.
    pub const JIS_TWO_BYTES: u32 = 0x0041_8c14;

    /// Reset the state to 1, store two bytes, return 2.
    pub const JIS_TWO_BYTES_STATE1: u32 = 0x0041_8c20;

    /// `*pwc = b0 << 8 | b1`, return `v0`.
    pub const STORE_TWO: u32 = 0x0041_8c30;

    /// Advance to the next input byte.
    pub const JIS_ADVANCE: u32 = 0x0041_8c48;

    /// Loop tail: more input bytes?
    pub const JIS_LOOP: u32 = 0x0041_8c4c;

    /// Return -1 (no handler accepted the sequence).
    pub const MINUS1_SCAN: u32 = 0x0041_8c5c;

    /// Single-byte locale: a null string returns 0.
    pub const SINGLE_BYTE: u32 = 0x0041_8c64;

    /// `*pwc = *s; return (0 < *s)`.
    pub const SINGLE_BYTE_COPY: u32 = 0x0041_8c6c;

    /// Return 0.
    pub const RETURN_ZERO: u32 = 0x0041_8c80;

    /// Restore the frame and return.
    pub const EPILOGUE: u32 = 0x0041_8c84;
}

// ---------------------------------------------------------------------------
// Date / time.

/// `bool Date_IsLeapYear(int year)` (0x2c71c0): the usual 4/100/400 rule.
pub fn date_is_leap_year(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A1][0] & 0x3;
    c.g[A0][0] = 0;
    let year = c.g[A1][0] as u32;
    if c.g[V0][0] == 0 {
        c.cycles += 5;
        c.g[V0][0] = 0x64;
        c.g[A0][0] = 0x1;
        let _ = abi::div(c, year, 0x64);
        c.cycles += 3;
        c.g[V1][0] = c.hi[0];
        c.g[V0][0] = 0x190;
        if c.g[V1][0] == 0 {
            c.cycles += 3;
            let _ = abi::div(c, year, 0x190);
            c.cycles += 2;
            c.g[V1][0] = c.hi[0];
            c.g[A0][0] = (c.g[V1][0] < 0x1) as u64;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `void Time_AddSeconds(int* hours, int* minutes, int* seconds, int extra,
/// int carry)` (0x2c7328): fold a second count back into wall-clock fields.
pub fn time_add_seconds(c: &mut Cpu) {
    c.cycles += 12;
    let a1 = c.g[A1][0] as u32;
    let a2 = c.g[A2][0] as u32;
    let a3 = c.g[A3][0] as u32;
    c.g[V0][0] = Addr(a3).lw(c);
    c.g[T3][0] = 0x3c;
    c.g[T1][0] = Addr(a2).lw(c);
    c.g[T2][0] = 0xe10;
    let p = abi::mult(c, c.g[V0][0] as u32, 0xe10);
    c.g[V0][0] = sx(p);
    c.g[T5][0] = Addr(a1).lw(c);
    let p = abi::mult(c, c.g[T1][0] as u32, 0x3c);
    c.g[T1][0] = sx(p);
    c.g[A0][0] = 0x10000;
    c.g[A0][0] |= 0x5180;
    c.g[V1][0] = 0x10000;
    c.cycles += 25;
    c.g[V1][0] |= 0x5180;
    c.g[T4][0] = 0x18;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
    c.g[T7][0] = 0x3c;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T5][0] as u32));
    c.g[T1][0] = 0xe10;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    let (q, r) = abi::div(c, c.g[V0][0] as u32, 0x15180);
    c.g[T1][0] = 0x3c;
    c.g[T5][0] = 0x3c;
    c.g[V1][0] = sx(r);
    c.g[V0][0] = sx(q);
    let (q, _r) = abi::div(c, c.g[V1][0] as u32, 0xe10);
    c.g[T2][0] = sx(q);
    let (q, r) = abi::div(c, c.g[V1][0] as u32, 0x3c);
    c.g[V1][0] = sx(q);
    c.g[T6][0] = sx(r);
    let (_q, r) = abi::div(c, c.g[T2][0] as u32, 0x18);
    c.g[A0][0] = sx(r);
    let (_q2, r2) = abi::div(c, c.g[V1][0] as u32, 0x3c);
    Addr(a3).sw(c, c.g[A0][0]);
    c.g[T0][0] = sx(r2);
    Addr(a2).sw(c, c.g[T0][0]);
    Addr(a1).sw(c, c.g[T6][0]);
    abi::ret(c);
}

pub(crate) const FWALK: Callee = Callee {
    addr: 0x0041_7418,
    f: ssx3_ee::s15::_fwalk,
};

pub(crate) const CLEANUP_R: Callee = Callee {
    addr: 0x0041_6fc0,
    f: ssx3_ee::s15::_cleanup_r,
};

/// `void _cleanup_r(struct _reent *ptr)` (0x416fc0): `_fwalk(ptr,
/// _cleanup_r)`-style flush of all open streams; here it walks with the
/// helper at `0x420000 - 0x1d0`.
pub fn cleanup_r(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x420000;
        frame_sd(c, frame, 0, c.g[RA][0]);
        c.g[A1][0] = sx(0x0042_0000u32.wrapping_sub(0x1d0));
        FWALK.call(c, 0x0041_6fd4)?;
        c.g[RA][0] = frame_ld(c, frame, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void _cleanup(void)` (0x416fe0): `_cleanup_r(_REENT)`.
pub fn cleanup(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        c.g[V0][0] = 0x450000;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(0x0045_4944).lw(c);
        frame_sd(c, frame, 0, c.g[RA][0]);
        CLEANUP_R.call(c, 0x0041_6ff8)?;
        c.g[RA][0] = frame_ld(c, frame, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Leaves.

/// `struct lconv *_localeconv_r(struct _reent *r)` (0x41ec08): the built-in
/// C locale (a static object at 0x49a790); ignores `r`.
pub fn localeconv_r(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx(0x0049_a790);
    abi::ret(c);
}

/// `int _fstat(int fd, struct stat *st)` (0x424558): the stub used when the
/// kernel's `fstat` isn't wired up: zero the size field and mark `st` as a
/// character device (0x2000), return 0.
pub fn fstat(c: &mut Cpu) {
    c.cycles += 5;
    let st = Addr(abi::arg(c, 1));
    st.off(0x48).sd(c, 0);
    st.off(0x4).sw(c, 0x2000);
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Reentrant (newlib `_r`) I/O wrappers. Each shifts the arguments (dropping
// the leading `struct _reent *`), clears the reent errno at 0x4a5bb8, calls
// the underlying syscall stub and, when that returns -1, copies the errno
// into `*reent`.

/// The reentrant struct's `_errno` word (at `0x4a0000 + 0x5bb8`).
pub(crate) const REENT_ERRNO: u32 = 0x004a_0000 + 0x5bb8;

/// The newlib syscall stubs the wrappers call.
pub(crate) const CLOSE: Callee = Callee {
    addr: 0x0042_4488,
    f: ssx3_ee::s15::f_00424488,
};

pub(crate) const OPEN: Callee = Callee {
    addr: 0x0042_4460,
    f: ssx3_ee::s15::f_00424460,
};

pub(crate) const READ: Callee = Callee {
    addr: 0x0042_43e8,
    f: ssx3_ee::s15::f_004243e8,
};

pub(crate) const WRITE: Callee = Callee {
    addr: 0x0042_4368,
    f: ssx3_ee::s15::f_00424368,
};

pub(crate) const LSEEK: Callee = Callee {
    addr: 0x0042_4498,
    f: ssx3_ee::s15::f_00424498,
};

pub(crate) const SBRK: Callee = Callee {
    addr: 0x0042_44a0,
    f: ssx3_ee::s15::f_004244a0,
};

pub(crate) const FSTAT_C: Callee = Callee {
    addr: 0x0042_4558,
    f: ssx3_ee::s15::f_00424558,
};

/// The shared body: open a 0x30 frame (s0 = reent, s1 = 0x4a0000, ra),
/// shift `shift` arguments off the front, clear the errno, call `target`.
/// `-1` results read the errno back into `v0` and store it through `reent`.
pub(crate) fn reentrant_io(
    c: &mut Cpu,
    target: Callee,
    shift: usize,
    ret: u32,
    first: u64,
) -> call::Flow {
    c.cycles += first;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = 0x004a_0000;
    frame.save_ra(c, 0);
    c.g[A0][0] = c.g[A1][0];
    if shift >= 2 {
        c.g[A1][0] = c.g[A2][0];
    }
    if shift >= 3 {
        c.g[A2][0] = c.g[A3][0];
    }
    Addr(REENT_ERRNO).sw(c, 0);
    target.call(c, ret)?;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = u64::MAX;
    if c.g[V1][0] == u64::MAX {
        c.cycles += 3;
        let errno = Addr(REENT_ERRNO).lw(c);
        c.g[V0][0] = errno;
        if errno != 0 {
            Addr(c.g[S0][0] as u32).sw(c, errno);
        }
        c.cycles += 1;
    }
    frame.restore(c, S0, 0x20);
    c.cycles += 5;
    c.g[V0][0] = c.g[V1][0];
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `int _close_r(struct _reent *r, int fd)` (0x41fb98).
pub fn close_r(c: &mut Cpu) {
    call::guest(c, |c| reentrant_io(c, CLOSE, 1, 0x0041_fbbc, 13))
}

/// `int _fstat_r(struct _reent *r, int fd, struct stat *st)` (0x41fbf0).
pub fn fstat_r(c: &mut Cpu) {
    call::guest(c, |c| reentrant_io(c, FSTAT_C, 2, 0x0041_fc18, 14))
}

/// `int _lseek_r(struct _reent *r, int fd, int off, int whence)` (0x41fc50).
pub fn lseek_r(c: &mut Cpu) {
    call::guest(c, |c| reentrant_io(c, LSEEK, 3, 0x0041_fc7c, 15))
}

/// `int _open_r(struct _reent *r, const char *path, int flags, int mode)`
/// (0x41fcb0).
pub fn open_r(c: &mut Cpu) {
    call::guest(c, |c| reentrant_io(c, OPEN, 3, 0x0041_fcdc, 15))
}

/// `int _read_r(struct _reent *r, int fd, void *buf, int n)` (0x41fd10).
pub fn read_r(c: &mut Cpu) {
    call::guest(c, |c| reentrant_io(c, READ, 3, 0x0041_fd3c, 15))
}

/// `void *_sbrk_r(struct _reent *r, int incr)` (0x41fd70).
pub fn sbrk_r(c: &mut Cpu) {
    call::guest(c, |c| reentrant_io(c, SBRK, 1, 0x0041_fd94, 14))
}

/// `int _write_r(struct _reent *r, int fd, const void *buf, int n)`
/// (0x41fdd0).
pub fn write_r(c: &mut Cpu) {
    call::guest(c, |c| reentrant_io(c, WRITE, 3, 0x0041_fdfc, 15))
}

pub(crate) const LIB_EXIT: Callee = Callee {
    addr: 0x0042_c6e8,
    f: ssx3_ee::s15::f_0042c6e8,
};

/// `int _kill(int pid, int sig)` (0x424578): pid 1 exits the process; any
/// other pid reports success (the stub can't signal).
pub fn kill(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        let not_self = c.g[A0][0] != 1;
        frame.save_ra(c, 0);
        if !not_self {
            c.cycles += 2;
            c.g[A0][0] = c.g[A1][0];
            LIB_EXIT.call(c, 0x0042_4590)?;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    })
}

/// `void _Bfree(struct _reent *r, struct _Bigint *p)` (0x4205b8): push `p`
/// onto the free-list bucket `r->freelist[p->k]` (at `r + 0x4c`, `k` at
/// `p + 4`, `p->next` at `p`).
pub fn bfree(c: &mut Cpu) {
    c.cycles += 2;
    let r = c.g[A0][0];
    let p = c.g[A1][0];
    c.g[A2][0] = r;
    if p != 0 {
        c.cycles += 9;
        c.g[A0][0] = Addr(p as u32).off(0x4).lw(c);
        c.g[V0][0] = Addr(r as u32).off(0x4c).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        Addr(p as u32).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(r as u32).off(0x4c).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        Addr(c.g[A0][0] as u32).sw(c, p);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `double _mprec_log10(int exp)` (0x4214c0): 10^exp via a table for
/// `0 < exp < 24`, else repeated multiplication (1.0 for `exp <= 0`).
pub fn mprec_log10(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let exp = c.g[A0][0];
        c.g[S0][0] = exp;
        c.g[A0][0] = 0xffc0u64 << 46; // 1.0
        c.g[V0][0] = ((exp as i64) < 24) as u64;
        let big = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !big {
            c.cycles += 6;
            c.g[V0][0] = 0x004a_0000;
            c.g[V1][0] = sx((c.g[S0][0] as u32) << 3);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5708));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).ld(c);
        } else {
            c.cycles += 2;
            let nonpos = (c.g[S0][0] as i64) <= 0;
            c.g[V0][0] = c.g[A0][0];
            if !nonpos {
                loop {
                    c.cycles += 7;
                    c.g[A1][0] = 0x8048u64 << 47; // 10.0
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                    MULDF3.call(c, 0x0042_1510)?;
                    c.g[A0][0] = c.g[V0][0];
                    if (c.g[S0][0] as i64) <= 0 {
                        c.cycles += 1;
                        c.g[V0][0] = c.g[A0][0];
                        break;
                    }
                    if event_due(c) {
                        crate::events::deliver_event_here(c, 0x0042_1500);
                    }
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// vfprintf_dispatch (0x0041b658)

/// The conversion table the original indexes with the specifier: word
/// `c - 'E'` is the address of the handler block (0x450000 + 0x6a30).
#[cfg(test)]
pub(crate) const SPEC_TABLE: u32 = 0x0045_6a30;

pub(crate) const SPEC_FIRST: u32 = 0x45;
// 'E'
pub(crate) const SPEC_COUNT: u64 = 0x23;

/// The two handler blocks of the table: the float-capable formatter and
/// "keep scanning".
pub(crate) const HANDLER_VFPRINTF: u32 = 0x0041_b6e8;

pub(crate) const HANDLER_CONTINUE: u32 = 0x0041_b6fc;

/// `int vfprintf(FILE* fp, const char* fmt, va_list ap)` (0x0041b658): choose
/// between `_vfprintf_r` (a float conversion is present) and
/// `_vfiprintf_r` (integer conversions only). The format is scanned once:
/// `%` starts a specifier, the flags/width/precision (anything below `'A'`)
/// are skipped, and the first byte at or above `'A'` is looked up in the
/// specifier table.
pub fn vfprintf_dispatch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[T0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[A2][0];
        c.g[A1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[T0][0] as u32).lb(c);
        let empty = c.g[V0][0] == 0;
        c.g[A2][0] = c.g[T0][0];
        if !empty {
            c.cycles += 2;
            c.g[V1][0] = Addr(c.g[A2][0] as u32).lb(c);
            'scan: loop {
                c.cycles += 3;
                c.g[V0][0] = 0x25; // '%'
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(1)).lb(c);
                    let trailing = c.g[V0][0] == 0;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    if !trailing {
                        // Skip flags/width/precision (bytes below 'A'); stop
                        // at the first specifier byte (or the terminator).
                        loop {
                            c.cycles += 4;
                            c.g[V0][0] = Addr(c.g[A2][0] as u32).lb(c);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 65i64) as u64;
                            if c.g[V0][0] != 0 {
                                c.cycles += 3;
                                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(1)).lb(c);
                                if c.g[V0][0] != 0 {
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0041_b698);
                                    }
                                    continue;
                                }
                                c.cycles += 1;
                            }
                            c.g[V0][0] = Addr(c.g[A2][0] as u32).lbu(c);
                            break;
                        }
                        c.cycles += 6;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(SPEC_FIRST));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        c.g[A0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                        c.g[V1][0] = (c.g[A0][0] < SPEC_COUNT) as u64;
                        let out_of_range = c.g[V1][0] == 0;
                        c.g[V0][0] = 0x4a0000;
                        if !out_of_range {
                            c.cycles += 6;
                            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5ce0));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                            match c.g[A0][0] as u32 {
                                HANDLER_VFPRINTF => {
                                    c.cycles += 5;
                                    c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x54)).lw(c);
                                    c.g[A2][0] = c.g[T0][0];
                                    callee::VFPRINTF_R.call(c, 0x0041_b6f4)?;
                                    c.g[RA][0] = Addr(call::sp(c)).ld(c);
                                    c.cycles += 2;
                                    frame.close(c);
                                    return Ok(());
                                }
                                HANDLER_CONTINUE => {}
                                other => {
                                    c.jump_out(other, 0x0041_b658, 0x0041_b724);
                                    return Ok(());
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    }
                }
                // Back edge: look for the next '%' (or the terminator).
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lb(c);
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = Addr(c.g[A2][0] as u32).lb(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_b680);
                    }
                    continue 'scan;
                }
                break;
            }
        }
        // Tail call into the chosen formatter.
        c.cycles += 4;
        c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x54)).lw(c);
        c.g[A2][0] = c.g[T0][0];
        callee::VFIPRINTF_R.call(c, 0x0041_b718)?;
        c.g[RA][0] = Addr(call::sp(c)).ld(c);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// The game image's locale names and C-JIS tables.

/// `"C-SJIS"`.
pub const LOCALE_SJIS: u32 = 0x0049_a200;

/// `"C-EUCJP"`.
pub const LOCALE_EUCJP: u32 = 0x0049_a208;

/// `"C-JIS"`.
pub const LOCALE_JIS: u32 = 0x0049_a210;

/// The `strcmp` second argument the original materialises for `C-SJIS`
/// (`0x4a0000 - 0x5e00`); the other two are derived from it.
pub(crate) const STR_BASE: u32 = 0x004a_0000;

/// `[state][class]` action index table (`0x450000 + 0x4af8`, indexed at
/// `+0x24` by the class and by `state * 9 * 4`).
pub const JIS_ACTIONS: u32 = 0x0045_4af8;

/// `[state][class]` next-state table (`0x450000 + 0x4948`, same indexing).
pub const JIS_STATES: u32 = 0x0045_4948;

/// The original's entry and end (for the out-of-range `pc` hook).
pub(crate) const START: u32 = l::ENTRY;

pub(crate) const END: u32 = 0x0041_8ca4;

#[inline(always)]
pub(crate) fn a(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// `int _mbtowc_r(_reent* r, wchar_t* pwc, const char* s, size_t n,
///                mbstate_t* state)` (0x418958): decode the next character of
/// `s` under the reent's Japanese locale, write it to `pwc` and return its
/// number of bytes (-1 invalid, 0 for a null string or a reset).
pub fn mbtowc_r(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                l::ENTRY => {
                    c.cycles += 14;
                    c.g[SP][0] = sx(a(c, SP).wrapping_sub(0x70));
                    c.sq(19, a(c, SP).wrapping_add(0x30)); // save s3
                    c.sq(16, a(c, SP).wrapping_add(0x60)); // save s0
                    c.g[S3][0] = c.g[SP][0]; // the wide-character sink (pwc)
                    c.sq(17, a(c, SP).wrapping_add(0x50)); // save s1
                    c.g[S0][0] = c.g[A2][0]; // s0 = s
                    c.sq(18, a(c, SP).wrapping_add(0x40)); // save s2
                    c.g[S1][0] = c.g[A0][0]; // s1 = reent
                    c.sq(20, a(c, SP).wrapping_add(0x20)); // save s4
                    c.g[S2][0] = c.g[T0][0]; // s2 = state
                    c.sd(a(c, SP).wrapping_add(0x10), c.g[RA][0]);
                    c.g[S4][0] = c.g[A3][0]; // s4 = n
                    let null_string = c.g[S0][0] == 0;
                    if c.g[A1][0] != 0 {
                        c.g[S3][0] = c.g[A1][0];
                    }
                    pc = if null_string { l::LOCALE } else { l::N_ZERO };
                }

                l::N_ZERO => {
                    c.cycles += 2;
                    let none = c.g[S4][0] == 0;
                    c.g[V0][0] = sx(0xffff_ffff);
                    pc = if none { l::EPILOGUE } else { l::LOCALE };
                }

                l::LOCALE => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(a(c, S1).wrapping_add(0x34));
                    let null_name = c.g[A0][0] == 0;
                    pc = if null_name {
                        l::SINGLE_BYTE
                    } else {
                        l::LOCALE_LEN
                    };
                }

                l::LOCALE_LEN => {
                    c.cycles += 5;
                    callee::STRLEN.call(c, 0x0041_89ac)?;
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let short = c.g[V0][0] != 0;
                    c.g[A1][0] = sx(STR_BASE);
                    pc = if short { l::SINGLE_BYTE } else { l::CMP_SJIS };
                }

                l::CMP_SJIS => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(a(c, S1).wrapping_add(0x34));
                    c.g[A1][0] = sx(a(c, A1).wrapping_sub(0x5e00));
                    callee::STRCMP.call(c, 0x0041_89c4)?;
                    if c.g[V0][0] != 0 {
                        c.g[A1][0] = sx(STR_BASE);
                        pc = l::CMP_EUCJP;
                    } else {
                        pc = l::SJIS_HEAD;
                    }
                }

                l::SJIS_HEAD => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lbu(a(c, S0));
                    let null_string = c.g[S0][0] == 0;
                    c.g[V0][0] = 0;
                    pc = if null_string {
                        l::EPILOGUE
                    } else {
                        l::SJIS_LEAD_HI
                    };
                }

                l::SJIS_LEAD_HI => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(a(c, V1).wrapping_sub(0x81));
                    c.g[V0][0] = (c.g[V0][0] < 0x1fu64) as u64;
                    let lead = c.g[V0][0] != 0;
                    c.g[V0][0] = (c.g[S4][0] < 0x2u64) as u64;
                    pc = if lead { l::SJIS_TRAIL } else { l::SJIS_LEAD_LO };
                }

                l::SJIS_LEAD_LO => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(a(c, V1).wrapping_sub(0xe0));
                    c.g[V0][0] = (c.g[V0][0] < 0x10u64) as u64;
                    let single = c.g[V0][0] == 0;
                    c.g[V0][0] = (c.g[S4][0] < 0x2u64) as u64;
                    pc = if single {
                        l::SINGLE_BYTE
                    } else {
                        l::SJIS_TRAIL
                    };
                }

                l::SJIS_TRAIL => {
                    c.cycles += 2;
                    // `v0` is `n < 2`: with a trail byte available the second
                    // byte is classified, otherwise the sequence is incomplete.
                    let have_trail = c.g[V0][0] == 0;
                    c.g[V1][0] = c.lbu(a(c, S0).wrapping_add(0x1));
                    pc = if have_trail {
                        l::SJIS_TRAIL1
                    } else {
                        l::MINUS1
                    };
                }

                l::MINUS1 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx(0xffff_ffff);
                    pc = l::EPILOGUE;
                }

                l::SJIS_TRAIL1 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(a(c, V1).wrapping_sub(0x40));
                    c.g[V0][0] = (c.g[V0][0] < 0x3fu64) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = c.lbu(a(c, S0));
                        pc = l::TWO_BYTES;
                    } else {
                        pc = l::SJIS_TRAIL2;
                    }
                }

                l::SJIS_TRAIL2 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(a(c, V1).wrapping_sub(0x80));
                    c.g[V0][0] = (c.g[V0][0] < 0x7du64) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = c.lbu(a(c, S0));
                        pc = l::TWO_BYTES;
                    } else {
                        pc = l::MINUS1_TRAIL;
                    }
                }

                l::MINUS1_TRAIL => {
                    c.cycles += 2;
                    c.g[V0][0] = sx(0xffff_ffff);
                    pc = l::EPILOGUE;
                }

                l::CMP_EUCJP => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(a(c, S1).wrapping_add(0x34));
                    c.g[A1][0] = sx(a(c, A1).wrapping_sub(0x5df8));
                    callee::STRCMP.call(c, 0x0041_8a3c)?;
                    if c.g[V0][0] != 0 {
                        c.g[A1][0] = sx(STR_BASE);
                        pc = l::CMP_JIS;
                    } else {
                        pc = l::EUC_HEAD;
                    }
                }

                l::EUC_HEAD => {
                    c.cycles += 2;
                    let null_string = c.g[S0][0] == 0;
                    c.g[V0][0] = c.lbu(a(c, S0));
                    pc = if null_string {
                        l::RETURN_ZERO
                    } else {
                        l::EUC_LEAD
                    };
                }

                l::EUC_LEAD => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(a(c, V0).wrapping_sub(0xa1));
                    c.g[V0][0] = (c.g[V0][0] < 0x5eu64) as u64;
                    let single = c.g[V0][0] == 0;
                    c.g[V0][0] = (c.g[S4][0] < 0x2u64) as u64;
                    pc = if single { l::SINGLE_BYTE } else { l::EUC_TRAIL };
                }

                l::EUC_TRAIL => {
                    c.cycles += 2;
                    let incomplete = c.g[V0][0] != 0;
                    c.g[V1][0] = c.lbu(a(c, S0).wrapping_add(0x1));
                    if incomplete {
                        // A single check before the forward jump to `MINUS1`.
                        if c.cycles >= c.next_event {
                            c.pc = l::MINUS1;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = l::MINUS1;
                    } else {
                        pc = l::EUC_TRAIL_CLASS;
                    }
                }

                l::EUC_TRAIL_CLASS => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(a(c, V1).wrapping_sub(0xa1));
                    c.g[V0][0] = (c.g[V0][0] < 0x5eu64) as u64;
                    let invalid = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(0xffff_ffff);
                    pc = if invalid {
                        l::EPILOGUE
                    } else {
                        l::EUC_LEAD_BYTE
                    };
                }

                l::EUC_LEAD_BYTE => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lbu(a(c, S0));
                    pc = l::TWO_BYTES;
                }

                l::TWO_BYTES => {
                    c.cycles += 6;
                    c.g[V0][0] = 2;
                    c.g[V1][0] = c.lbu(a(c, S0).wrapping_add(0x1));
                    c.g[A0][0] = sx(a(c, A0) << 8);
                    c.g[V1][0] = sx(a(c, V1).wrapping_add(a(c, A0)));
                    c.sh(a(c, S3), c.g[V1][0]);
                    pc = l::EPILOGUE;
                }

                l::CMP_JIS => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(a(c, S1).wrapping_add(0x34));
                    c.g[A1][0] = sx(a(c, A1).wrapping_sub(0x5df0));
                    callee::STRCMP.call(c, 0x0041_8a9c)?;
                    let other = c.g[V0][0] != 0;
                    pc = if other { l::SINGLE_BYTE } else { l::JIS_STATE };
                }

                l::JIS_STATE => {
                    c.cycles += 2;
                    if c.g[S0][0] != 0 {
                        c.g[V0][0] = c.lw(a(c, S2));
                        pc = l::JIS_SETUP;
                    } else {
                        pc = l::JIS_RESET;
                    }
                }

                l::JIS_RESET => {
                    c.cycles += 3;
                    c.sw(a(c, S2), 0);
                    c.g[V0][0] = 1;
                    pc = l::EPILOGUE;
                }

                l::JIS_SETUP => {
                    c.cycles += 5;
                    c.g[T1][0] = 3; // table row: 3 for a state in progress
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A2][0] = 0;
                    let none = c.g[S4][0] == 0;
                    if c.g[V0][0] == 0 {
                        c.g[T1][0] = 0; // row 0 for the initial state
                    }
                    if none {
                        // A single check before the forward jump to `MINUS1`.
                        if c.cycles >= c.next_event {
                            c.pc = l::MINUS1;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = l::MINUS1;
                    } else {
                        pc = l::JIS_NEXT_BYTE;
                    }
                }

                l::JIS_NEXT_BYTE => {
                    c.cycles += 1;
                    c.g[V0][0] = sx(a(c, S0).wrapping_add(a(c, A2)));
                    pc = l::JIS_CLASSIFY;
                }

                l::JIS_CLASSIFY => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x28; // '('
                    c.g[A0][0] = c.lbu(a(c, V0));
                    let lparen = c.g[A0][0] == c.g[V1][0];
                    c.g[T0][0] = c.g[V0][0];
                    pc = if lparen {
                        l::CLASS_LPAREN
                    } else {
                        l::JIS_LT_41
                    };
                }

                l::JIS_LT_41 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 41i64) as u64;
                    let ge = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x1b; // ESC
                    pc = if ge { l::JIS_GE_41 } else { l::JIS_ESC };
                }

                l::JIS_ESC => {
                    c.cycles += 2;
                    let esc = c.g[A0][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 28i64) as u64;
                    pc = if esc { l::CLASS_ESC } else { l::JIS_LT_28 };
                }

                l::JIS_LT_28 => {
                    c.cycles += 2;
                    let lt = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x24; // '$'
                    pc = if lt { l::JIS_DOLLAR } else { l::JIS_NUL };
                }

                l::JIS_NUL => {
                    c.cycles += 2;
                    let nul = c.g[A0][0] == 0;
                    c.g[V0][0] = sx(a(c, A0).wrapping_sub(0x21));
                    pc = if nul {
                        l::CLASS_OTHER
                    } else {
                        l::JIS_CLASS_5D_A
                    };
                }

                l::JIS_CLASS_5D_A => {
                    c.cycles += 2;
                    c.g[V1][0] = 0x5d; // ']'
                    pc = l::CLASS_5D;
                }

                l::JIS_DOLLAR => {
                    c.cycles += 2;
                    let dollar = c.g[A0][0] == c.g[V0][0];
                    c.g[V0][0] = sx(a(c, A0).wrapping_sub(0x21));
                    pc = if dollar {
                        l::CLASS_DOLLAR
                    } else {
                        l::JIS_CLASS_5D_B
                    };
                }

                l::JIS_CLASS_5D_B => {
                    c.cycles += 2;
                    c.g[V1][0] = 0x5d; // ']'
                    pc = l::CLASS_5D;
                }

                l::JIS_GE_41 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x42; // 'B'
                    let is_b = c.g[A0][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 67i64) as u64;
                    pc = if is_b { l::CLASS_B } else { l::JIS_GE_43 };
                }

                l::JIS_GE_43 => {
                    c.cycles += 2;
                    let lt = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x40; // '@'
                    pc = if lt { l::JIS_GE_67 } else { l::JIS_AT };
                }

                l::JIS_AT => {
                    c.cycles += 2;
                    let at = c.g[A0][0] == c.g[V0][0];
                    c.g[V0][0] = sx(a(c, A0).wrapping_sub(0x21));
                    pc = if at { l::CLASS_AT } else { l::JIS_CLASS_5D_C };
                }

                l::JIS_CLASS_5D_C => {
                    c.cycles += 2;
                    c.g[V1][0] = 0x5d; // ']'
                    pc = l::CLASS_5D;
                }

                l::JIS_GE_67 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4a; // 'J'
                    let is_j = c.g[A0][0] == c.g[V0][0];
                    c.g[V0][0] = sx(a(c, A0).wrapping_sub(0x21));
                    pc = if is_j { l::CLASS_J } else { l::JIS_CLASS_5D_D };
                }

                l::JIS_CLASS_5D_D => {
                    c.cycles += 2;
                    c.g[V1][0] = 0x5d; // ']'
                    pc = l::CLASS_5D;
                }

                l::CLASS_ESC => {
                    c.cycles += 2;
                    c.g[A0][0] = 0;
                    pc = l::JIS_TABLE;
                }

                l::CLASS_DOLLAR => {
                    c.cycles += 2;
                    c.g[A0][0] = 1;
                    pc = l::JIS_TABLE;
                }

                l::CLASS_AT => {
                    c.cycles += 2;
                    c.g[A0][0] = 3;
                    pc = l::JIS_TABLE;
                }

                l::CLASS_LPAREN => {
                    c.cycles += 2;
                    c.g[A0][0] = 2;
                    pc = l::JIS_TABLE;
                }

                l::CLASS_B => {
                    c.cycles += 2;
                    c.g[A0][0] = 4;
                    pc = l::JIS_TABLE;
                }

                l::CLASS_J => {
                    c.cycles += 2;
                    c.g[A0][0] = 5;
                    pc = l::JIS_TABLE;
                }

                l::CLASS_OTHER => {
                    c.cycles += 2;
                    c.g[A0][0] = 6;
                    pc = l::JIS_TABLE;
                }

                l::CLASS_5D => {
                    c.cycles += 4;
                    c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                    c.g[A0][0] = 8;
                    c.g[V0][0] = 7;
                    if c.g[V1][0] == 0 {
                        c.g[A0][0] = c.g[V0][0];
                    }
                    pc = l::JIS_TABLE;
                }

                l::JIS_TABLE => {
                    c.cycles += 14;
                    c.g[A1][0] = 0x24; // one table row: 9 classes * 4 bytes
                    c.g[A0][0] = sx(a(c, A0) << 2);
                    // `mult a1, t1, a1`: the row offset (0 or 3 rows).
                    c.mult(0, A1, T1, A1);
                    c.g[V1][0] = sx(JIS_ACTIONS);
                    c.g[V0][0] = sx(JIS_STATES);
                    c.g[A0][0] = sx(a(c, A0).wrapping_add(a(c, A1)));
                    c.g[V1][0] = sx(a(c, A0).wrapping_add(a(c, V1)));
                    c.g[V1][0] = c.lw(a(c, V1)); // the action index
                    c.g[A0][0] = sx(a(c, A0).wrapping_add(a(c, V0)));
                    c.g[V0][0] = (c.g[V1][0] < 0x8u64) as u64;
                    let none = c.g[V0][0] == 0;
                    c.g[T1][0] = c.lw(a(c, A0)); // the next state
                    if none {
                        // A single check before the forward jump to `MINUS1`.
                        if c.cycles >= c.next_event {
                            c.pc = l::MINUS1;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = l::MINUS1;
                    } else {
                        pc = l::JIS_DISPATCH;
                    }
                }

                l::JIS_DISPATCH => {
                    c.cycles += 7;
                    c.g[V0][0] = sx(STR_BASE);
                    c.g[V1][0] = sx(a(c, V1) << 2);
                    c.g[V0][0] = sx(a(c, V0).wrapping_sub(0x5de0));
                    c.g[V1][0] = sx(a(c, V1).wrapping_add(a(c, V0)));
                    c.g[A0][0] = c.lw(a(c, V1));
                    pc = a(c, A0);
                }

                l::JIS_RESET_ZERO => {
                    c.cycles += 4;
                    c.sw(a(c, S2), 0);
                    c.g[V0][0] = c.g[A2][0];
                    c.sh(a(c, S3), 0);
                    pc = l::EPILOGUE;
                }

                l::JIS_ONE_BYTE => {
                    c.cycles += 5;
                    c.sw(a(c, S2), 0);
                    c.g[V0][0] = sx(a(c, A2).wrapping_add(0x1));
                    c.g[V1][0] = c.lbu(a(c, A3));
                    c.sh(a(c, S3), c.g[V1][0]);
                    pc = l::EPILOGUE;
                }

                l::JIS_TWO_BYTES => {
                    c.cycles += 3;
                    c.sw(a(c, S2), 0);
                    c.g[V0][0] = sx(a(c, A2).wrapping_add(0x1));
                    pc = l::STORE_TWO;
                }

                l::JIS_TWO_BYTES_STATE1 => {
                    c.cycles += 4;
                    c.g[V1][0] = 1;
                    c.g[V0][0] = sx(a(c, A3).wrapping_sub(a(c, S0)));
                    c.sw(a(c, S2), c.g[V1][0]);
                    c.g[V0][0] = sx(a(c, V0).wrapping_add(0x2));
                    pc = l::STORE_TWO;
                }

                l::STORE_TWO => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lbu(a(c, A3));
                    c.g[A0][0] = c.lbu(a(c, A3).wrapping_add(0x1));
                    c.g[V1][0] = sx(a(c, V1) << 8);
                    c.g[A0][0] = sx(a(c, A0).wrapping_add(a(c, V1)));
                    c.sh(a(c, S3), c.g[A0][0]);
                    pc = l::EPILOGUE;
                }

                l::JIS_ADVANCE => {
                    c.cycles += 1;
                    c.g[A3][0] = sx(a(c, T0).wrapping_add(0x1));
                    pc = l::JIS_LOOP;
                }

                l::JIS_LOOP => {
                    c.cycles += 4;
                    c.g[A2][0] = sx(a(c, A2).wrapping_add(0x1));
                    c.g[V0][0] = (c.g[A2][0] < c.g[S4][0]) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(a(c, S0).wrapping_add(a(c, A2)));
                    if more {
                        // The loop's back edge: an unwind resumes the function
                        // at the classification head.
                        if back_edge_unwinds(c, l::JIS_CLASSIFY) {
                            return Ok(());
                        }
                        pc = l::JIS_CLASSIFY;
                    } else {
                        pc = l::MINUS1_SCAN;
                    }
                }

                l::MINUS1_SCAN => {
                    c.cycles += 2;
                    c.g[V0][0] = sx(0xffff_ffff);
                    pc = l::EPILOGUE;
                }

                l::SINGLE_BYTE => {
                    c.cycles += 2;
                    if c.g[S0][0] == 0 {
                        c.g[V0][0] = 0;
                        pc = l::EPILOGUE;
                    } else {
                        pc = l::SINGLE_BYTE_COPY;
                    }
                }

                l::SINGLE_BYTE_COPY => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lbu(a(c, S0));
                    c.sh(a(c, S3), c.g[V0][0]);
                    c.g[V0][0] = c.lbu(a(c, S0));
                    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                    pc = l::EPILOGUE;
                }

                l::RETURN_ZERO => {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    pc = l::EPILOGUE;
                }

                l::EPILOGUE => {
                    c.cycles += 8;
                    let sp = a(c, SP);
                    c.lq(16, sp.wrapping_add(0x60));
                    c.lq(17, sp.wrapping_add(0x50));
                    c.lq(18, sp.wrapping_add(0x40));
                    c.lq(19, sp.wrapping_add(0x30));
                    c.lq(20, sp.wrapping_add(0x20));
                    c.g[RA][0] = c.ld(sp.wrapping_add(0x10));
                    c.g[SP][0] = sx(sp.wrapping_add(0x70));
                    abi::ret(c);
                    return Ok(());
                }

                _ => {
                    c.jump_out(pc, START, END);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Format flags (the original keeps them in `s2`; the bit numbers are this
// build's, not upstream newlib's).

pub(crate) const ALT: u64 = 0x0001;
// '#'  alternate form (0x/0X/leading 0)
pub(crate) const HASHX: u64 = 0x0002;
// internal: the base prefix is to be emitted
pub(crate) const LEFT: u64 = 0x0004;
// '-'  left justify
pub(crate) const LONGINT: u64 = 0x0010;
// 'l'  long
pub(crate) const LONGLONG: u64 = 0x0020;
// 'll', 'q'  long long
pub(crate) const SHORTINT: u64 = 0x0040;
// 'h'  short
pub(crate) const ZEROPAD: u64 = 0x0080;
// '0'  pad with zeroes (dropped when a
//      precision is given)
pub(crate) const GROUP: u64 = 0x0200;
// '\'' insert thousands separators

/// The specifier jump table (`0x450000 - 0x59e0`, 89 entries, `' '`..`'x'`),
/// listing the arm each conversion character reaches:
///
/// ```text
/// ' '       0x41cc14   '+'       0x41cc5c   'p'       0x41ce68
/// '#'       0x41cc2c   '.'       0x41cc68   's'       0x41ce8c
/// '\''      0x41cc34   '0'       0x41ccdc   'U'       0x41ceec
/// '*'       0x41cc3c   '1'..'9'  0x41cce4   'u'       0x41cef0
/// '-'       0x41cc54   'h'       0x41cd18   'X'       0x41cf30
/// 'l'       0x41cd20   'q'       0x41cd34   'x'       0x41cf3c
/// 'c'       0x41cd3c   'D'       0x41cd68   anything  0x41d134
/// 'd', 'i'  0x41cd6c   'n'       0x41cdbc
/// 'O'       0x41ce24   'o'       0x41ce28
/// ```

// ---------------------------------------------------------------------------
// Frame slots of the original (offsets from `sp`, see the module docs).

pub(crate) const NUMBUF: u32 = 0x28;
// digits are built backwards from here
pub(crate) const PREFIXBUF: u32 = 0x30;
// "0x"/"0X"/"0" prefix (2 bytes)
pub(crate) const WC: u32 = 0x40;
// wchar_t converted from the format
pub(crate) const SIGN: u32 = 0x42;
// sign character: '-', '+', ' ' or 0
pub(crate) const MBSTATE: u32 = 0x44;
// mbstate_t for _mbtowc_r
pub(crate) const LEN: u32 = 0x48;
// length of the digits / string to write
pub(crate) const WRITTEN: u32 = 0x4c;
// characters written so far
pub(crate) const WIDTH: u32 = 0x50;
// field width (0 when absent)
pub(crate) const PREC: u32 = 0x54;
// precision (-1 when absent)
pub(crate) const PAD: u32 = 0x58;
// width of the whole field (digits + sign/prefix)
pub(crate) const BASE: u32 = 0x5c;
// digit table of a hex conversion
pub(crate) const PWC: u32 = 0x60;
// &WC
pub(crate) const PMBSTATE: u32 = 0x64;
// &MBSTATE
pub(crate) const GROUP_FLAG: u32 = 0x68;
// flags & LEFT, saved for the padding writer

/// `int _vfiprintf_r(struct _reent *r, FILE *fp, const char *fmt, va_list ap)`
/// (0x0041ca70): formats `fmt` with the varargs behind `ap` and writes the
/// result to `fp`, returning the number of characters produced.
///
/// Entered at `c.pc = 0x0041ca70`; the body resumes at any of the original's
/// 244 entry points (see the module docs).
#[rustfmt::skip]
pub fn vfiprintf_r(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            let sp = c.g[SP][0] as u32;
            match pc {
                // Prologue: push the 0x110-byte frame, save ra, f30 and f16-f25;
                    // fp = a1, fmt = a2, ap = a3 (the reent in a0 is unused).
                0x0041_ca70 => {
                    c.cycles += 20;
                    let sp = sp.wrapping_sub(0x110); // 0x110-byte frame
                    c.g[SP][0] = sx(sp);
                    c.g[V0][0] = sx(sp + WC);
                    c.g[V1][0] = sx(sp + MBSTATE);
                    c.sq(17, sp + 0xf0);
                    c.sq(19, sp + 0xd0);
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(23, sp + 0x90);
                    c.g[S3][0] = c.g[A3][0];
                    c.sq(16, sp + 0x100);
                    c.g[S7][0] = c.g[A2][0];
                    c.sq(18, sp + 0xe0);
                    c.sq(20, sp + 0xc0);
                    c.sq(21, sp + 0xb0);
                    c.sq(22, sp + 0xa0);
                    c.sq(30, sp + 0x80);
                    c.sd(sp + 0x70, c.g[RA][0]);
                    c.sw(sp + MBSTATE, 0u64);
                    c.sw(sp + WRITTEN, 0u64);
                    c.sw(sp + PWC, c.g[V0][0]);
                    c.sw(sp + PMBSTATE, c.g[V1][0]);
                    pc = 0x0041_cac0; continue;
                }
                // Literal-run head: cp = fmt; the character looked for is '%'.
                0x0041_cac0 => {
                    c.cycles += 2;
                    c.g[S4][0] = c.g[S7][0];
                    c.g[S2][0] = 0x25;
                    pc = 0x0041_cac8; continue;
                }
                // Convert one multibyte character of the format (_mbtowc_r).
                0x0041_cac8 => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x450000;
                    c.g[A2][0] = 0x450000;
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4944));
                    c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x5140));
                    c.g[A1][0] = c.lw(sp + PWC);
                    c.g[A2][0] = c.g[S7][0];
                    c.g[RA][0] = sx(0x0041cae8);
                    c.g[T0][0] = c.lw(sp + PMBSTATE);
                    callee::MBTOWC_R.call(c, 0x0041_cae8)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = (c.g[S0][0] as i64) <= 0;
                    c.g[V0][0] = c.lhu(sp + WC);
                    if t { pc = 0x0041_cb00; continue; }
                    pc = 0x0041_caf4; continue;
                }
                // Not a '%': step over the character (n bytes) and rescan.
                0x0041_caf4 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != c.g[S2][0];
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    if t { if back_edge_unwinds(c, 0x0041_cac8) { return Ok(()); } pc = 0x0041_cac8; continue; }
                    pc = 0x0041_cafc; continue;
                }
                // A '%': step back so fmt points at it.
                0x0041_cafc => {
                    c.cycles += 1;
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x1));
                    pc = 0x0041_cb00; continue;
                }
                // n = fmt - cp, the literal run before the conversion.
                0x0041_cb00 => {
                    c.cycles += 3;
                    c.g[S2][0] = sx((c.g[S7][0] as u32).wrapping_sub(c.g[S4][0] as u32));
                    let t = c.g[S2][0] == 0u64;
                    if t { pc = 0x0041_cba8; continue; }
                    pc = 0x0041_cb0c; continue;
                }
                // Write the run through the inlined __sfvwrite fast path.
                0x0041_cb0c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] & 0x200;
                    if c.g[V0][0] == 0u64 { c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc)); pc = 0x0041_cb78; continue; }
                    pc = 0x0041_cb1c; continue;
                }
                0x0041_cb1c => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S4][0];
                    if t { pc = 0x0041_cb54; continue; }
                    pc = 0x0041_cb2c; continue;
                }
                0x0041_cb2c => {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[RA][0] = sx(0x0041cb38);
                    c.g[A2][0] = c.g[S2][0];
                    callee::MEMCPY.call(c, 0x0041_cb38)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S2][0] as u32));
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t { pc = 0x0041_cb9c; continue; }
                    pc = 0x0041_cb54; continue;
                }
                0x0041_cb54 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[RA][0] = sx(0x0041cb60);
                    c.g[A1][0] = c.g[S4][0];
                    callee::MEMCPY.call(c, 0x0041_cb60)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    if t { pc = 0x0041_cb9c; continue; }
                    pc = 0x0041_cb78; continue;
                }
                // Not __SWR: hand the run to vfiprintf_BufferedWrite.
                0x0041_cb78 => {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe)); pc = 0x0041_cb8c; continue; }
                    pc = 0x0041_cb80; continue;
                }
                0x0041_cb80 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    c.sh((c.g[S1][0] as u32).wrapping_add(0xe), c.g[V0][0]);
                    c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe));
                    pc = 0x0041_cb8c; continue;
                }
                0x0041_cb8c => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0041cb9c);
                    c.g[A3][0] = 0u64;
                    callee::BUFFERED_WRITE.call(c, 0x0041_cb9c)?;
                    pc = 0x0041_cb9c; continue;
                }
                // The run was written: written += n.
                0x0041_cb9c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(sp + WRITTEN);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.sw(sp + WRITTEN, c.g[V0][0]);
                    pc = 0x0041_cba8; continue;
                }
                // Scan hit the end of the format?
                0x0041_cba8 => {
                    c.cycles += 2;
                    if (c.g[S0][0] as i64) <= 0 { c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc)); pc = 0x0041_d93c; continue; }
                    pc = 0x0041_cbb0; continue;
                }
                0x0041_cbb0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if (c.g[V0][0] as i64) > 0 { c.sb(sp + SIGN, 0u64); pc = 0x0041_cbd0; continue; }
                    pc = 0x0041_cbbc; continue;
                }
                0x0041_cbbc => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] & 0x200;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw(sp + WRITTEN);
                    if t { pc = 0x0041_d960; continue; }
                    pc = 0x0041_cbcc; continue;
                }
                0x0041_cbcc => {
                    c.cycles += 1;
                    c.sb(sp + SIGN, 0u64);
                    pc = 0x0041_cbd0; continue;
                }
                // Start a conversion: skip the '%', reset flags/width/prec.
                0x0041_cbd0 => {
                    c.cycles += 5;
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                    c.sw(sp + PREC, 0u64);
                    c.g[S2][0] = 0u64;
                    c.sw(sp + WIDTH, 0u64);
                    c.g[FP][0] = 0xffffffffffffffff;
                    pc = 0x0041_cbe4; continue;
                }
                // Read the conversion character.
                0x0041_cbe4 => {
                    c.cycles += 1;
                    c.g[S5][0] = c.lb(c.g[S7][0] as u32);
                    pc = 0x0041_cbe8; continue;
                }
                0x0041_cbe8 => {
                    c.cycles += 1;
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                    pc = 0x0041_cbec; continue;
                }
                // Look it up in the specifier table (' '..'x').
                0x0041_cbec => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x20));
                    c.g[V0][0] = (c.g[V1][0] < 0x59u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t { pc = 0x0041_d134; continue; }
                    pc = 0x0041_cbfc; continue;
                }
                // Jump through the table.
                0x0041_cbfc => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x59e0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;
                    pc = t; continue;
                }
                // ' '  a positive value gets a space sign.
                0x0041_cc14 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(sp + SIGN);
                    if c.g[V0][0] != 0u64 { c.g[S5][0] = c.lb(c.g[S7][0] as u32); if back_edge_unwinds(c, 0x0041_cbe8) { return Ok(()); } pc = 0x0041_cbe8; continue; }
                    pc = 0x0041_cc20; continue;
                }
                0x0041_cc20 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x20;
                    let t = true;
                    c.sb(sp + SIGN, c.g[V0][0]);
                    if t { if back_edge_unwinds(c, 0x0041_cbe4) { return Ok(()); } pc = 0x0041_cbe4; continue; }
                    pc = 0x0041_cc2c; continue;
                }
                // '#'  alternate form.
                0x0041_cc2c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S2][0] = c.g[S2][0] | ALT;
                    if t { if back_edge_unwinds(c, 0x0041_cbe4) { return Ok(()); } pc = 0x0041_cbe4; continue; }
                    pc = 0x0041_cc34; continue;
                }
                // '\''  group the digits (thousands separators).
                0x0041_cc34 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S2][0] = c.g[S2][0] | GROUP;
                    if t { if back_edge_unwinds(c, 0x0041_cbe4) { return Ok(()); } pc = 0x0041_cbe4; continue; }
                    pc = 0x0041_cc3c; continue;
                }
                // '*'  field width from the next argument.
                0x0041_cc3c => {
                    c.cycles += 4;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_sub(0x8));
                    let t = (c.g[V1][0] as i64) >= 0;
                    c.sw(sp + WIDTH, c.g[V1][0]);
                    if t { if back_edge_unwinds(c, 0x0041_cbe4) { return Ok(()); } pc = 0x0041_cbe4; continue; }
                    pc = 0x0041_cc4c; continue;
                }
                // A negative width means left justification.
                0x0041_cc4c => {
                    c.cycles += 2;
                    c.g[V1][0] = sx(0u32.wrapping_sub(c.g[V1][0] as u32));
                    c.sw(sp + WIDTH, c.g[V1][0]);
                    pc = 0x0041_cc54; continue;
                }
                // '-'  left justify.
                0x0041_cc54 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S2][0] = c.g[S2][0] | LEFT;
                    if t { if back_edge_unwinds(c, 0x0041_cbe4) { return Ok(()); } pc = 0x0041_cbe4; continue; }
                    pc = 0x0041_cc5c; continue;
                }
                // '+'  always show a sign.
                0x0041_cc5c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2b;
                    let t = true;
                    c.sb(sp + SIGN, c.g[V0][0]);
                    if t { if back_edge_unwinds(c, 0x0041_cbe4) { return Ok(()); } pc = 0x0041_cbe4; continue; }
                    pc = 0x0041_cc68; continue;
                }
                // '.'  precision: '*' from the argument, or decimal digits.
                0x0041_cc68 => {
                    c.cycles += 4;
                    c.g[S5][0] = c.lb(c.g[S7][0] as u32);
                    c.g[V0][0] = 0x2a;
                    let t = c.g[S5][0] != c.g[V0][0];
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                    if t { pc = 0x0041_cc94; continue; }
                    pc = 0x0041_cc78; continue;
                }
                0x0041_cc78 => {
                    c.cycles += 7;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = 0xfffffffffffffffe;
                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_sub(0x8));
                    c.g[FP][0] = 0xffffffffffffffff;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    let t = true;
                    if c.g[V0][0] != 0 { c.g[FP][0] = c.g[S0][0]; }
                    if t { if back_edge_unwinds(c, 0x0041_cbe4) { return Ok(()); } pc = 0x0041_cbe4; continue; }
                    pc = 0x0041_cc94; continue;
                }
                // Precision digits.
                0x0041_cc94 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = 0u64;
                    if t { pc = 0x0041_ccc8; continue; }
                    pc = 0x0041_cca4; continue;
                }
                0x0041_cca4 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0xa;
                    pc = 0x0041_cca8; continue;
                }
                0x0041_cca8 => {
                    c.cycles += 8;
                    c.mult(0, 2, 16, 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[S5][0] = c.lb(c.g[S7][0] as u32);
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                    if t { if back_edge_unwinds(c, 0x0041_cca8) { return Ok(()); } pc = 0x0041_cca8; continue; }
                    pc = 0x0041_ccc8; continue;
                }
                0x0041_ccc8 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0xfffffffffffffffe;
                    c.g[FP][0] = 0xffffffffffffffff;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    let t = true;
                    if c.g[V0][0] != 0 { c.g[FP][0] = c.g[S0][0]; }
                    if t { if back_edge_unwinds(c, 0x0041_cbec) { return Ok(()); } pc = 0x0041_cbec; continue; }
                    pc = 0x0041_ccdc; continue;
                }
                // '0'  zero pad.
                0x0041_ccdc => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S2][0] = c.g[S2][0] | ZEROPAD;
                    if t { if back_edge_unwinds(c, 0x0041_cbe4) { return Ok(()); } pc = 0x0041_cbe4; continue; }
                    pc = 0x0041_cce4; continue;
                }
                // '1'..'9'  field width digits.
                0x0041_cce4 => {
                    c.cycles += 3;
                    c.g[S0][0] = 0u64;
                    c.g[V1][0] = 0xa;
                    pc = 0x0041_ccf0; continue;
                }
                0x0041_ccf0 => {
                    c.cycles += 8;
                    c.mult(0, 2, 16, 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[S5][0] = c.lb(c.g[S7][0] as u32);
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                    if t { if back_edge_unwinds(c, 0x0041_ccf0) { return Ok(()); } pc = 0x0041_ccf0; continue; }
                    pc = 0x0041_cd10; continue;
                }
                0x0041_cd10 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw(sp + WIDTH, c.g[S0][0]);
                    if t { if back_edge_unwinds(c, 0x0041_cbec) { return Ok(()); } pc = 0x0041_cbec; continue; }
                    pc = 0x0041_cd18; continue;
                }
                // 'h'  short.
                0x0041_cd18 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S2][0] = c.g[S2][0] | SHORTINT;
                    if t { if back_edge_unwinds(c, 0x0041_cbe4) { return Ok(()); } pc = 0x0041_cbe4; continue; }
                    pc = 0x0041_cd20; continue;
                }
                // 'l'  long, or 'll' for long long.
                0x0041_cd20 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lb(c.g[S7][0] as u32);
                    c.g[V0][0] = 0x6c;
                    if c.g[V1][0] != c.g[V0][0] { c.g[S2][0] = c.g[S2][0] | LONGINT; if back_edge_unwinds(c, 0x0041_cbe4) { return Ok(()); } pc = 0x0041_cbe4; continue; }
                    pc = 0x0041_cd30; continue;
                }
                0x0041_cd30 => {
                    c.cycles += 1;
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                    pc = 0x0041_cd34; continue;
                }
                // 'q'  long long.
                0x0041_cd34 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S2][0] = c.g[S2][0] | LONGLONG;
                    if t { if back_edge_unwinds(c, 0x0041_cbe4) { return Ok(()); } pc = 0x0041_cbe4; continue; }
                    pc = 0x0041_cd3c; continue;
                }
                // 'c'  one character (staged in the frame's first byte).
                0x0041_cd3c => {
                    c.cycles += 11;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = 0x1;
                    c.g[V1][0] = c.lbu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    c.g[A0][0] = c.g[S2][0] & LEFT;
                    c.g[S4][0] = c.g[SP][0];
                    c.sw(sp + LEN, c.g[V0][0]);
                    c.g[FP][0] = c.g[S2][0] & (ZEROPAD | LEFT);
                    c.sb(sp, c.g[V1][0]);
                    c.sw(sp + GROUP_FLAG, c.g[A0][0]);
                    let t = true;
                    c.sb(sp + SIGN, 0u64);
                    if t { pc = 0x0041_d158; continue; }
                    pc = 0x0041_cd68; continue;
                }
                // 'D'  long signed decimal.
                0x0041_cd68 => {
                    c.cycles += 1;
                    c.g[S2][0] = c.g[S2][0] | LONGINT;
                    pc = 0x0041_cd6c; continue;
                }
                // 'd', 'i'  signed decimal.
                0x0041_cd6c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S2][0] & LONGLONG;
                    if c.g[V0][0] != 0u64 { c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8)); pc = 0x0041_cd88; continue; }
                    pc = 0x0041_cd78; continue;
                }
                0x0041_cd78 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S2][0] & LONGINT;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[S2][0] & SHORTINT;
                    if t { pc = 0x0041_cd90; continue; }
                    pc = 0x0041_cd84; continue;
                }
                0x0041_cd84 => {
                    c.cycles += 1;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    pc = 0x0041_cd88; continue;
                }
                // Fetch the argument: ld for long/long long, lh for short, lwu for int.
                0x0041_cd88 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.ld((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t { pc = 0x0041_cda4; continue; }
                    pc = 0x0041_cd90; continue;
                }
                0x0041_cd90 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t { pc = 0x0041_cda0; continue; }
                    pc = 0x0041_cd98; continue;
                }
                0x0041_cd98 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.lh((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t { pc = 0x0041_cda4; continue; }
                    pc = 0x0041_cda0; continue;
                }
                0x0041_cda0 => {
                    c.cycles += 1;
                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_sub(0x8));
                    pc = 0x0041_cda4; continue;
                }
                // A negative decimal value prints a '-' and is negated.
                0x0041_cda4 => {
                    c.cycles += 2;
                    let t = (c.g[S0][0] as i64) >= 0;
                    c.g[S6][0] = 0x1;
                    if t { pc = 0x0041_cf98; continue; }
                    pc = 0x0041_cdac; continue;
                }
                0x0041_cdac => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x2d;
                    c.g[S0][0] = 0u64.wrapping_sub(c.g[S0][0]);
                    let t = true;
                    c.sb(sp + SIGN, c.g[V0][0]);
                    if t { pc = 0x0041_cf98; continue; }
                    pc = 0x0041_cdbc; continue;
                }
                // 'n'  store written into the argument pointer.
                0x0041_cdbc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S2][0] & LONGLONG;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.lw(sp + WRITTEN);
                    if t { pc = 0x0041_cdd8; continue; }
                    pc = 0x0041_cdc8; continue;
                }
                0x0041_cdc8 => {
                    c.cycles += 4;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_sub(0x8));
                    let t = true;
                    c.sd(c.g[V0][0] as u32, c.g[A1][0]);
                    if t { if back_edge_unwinds(c, 0x0041_cac0) { return Ok(()); } pc = 0x0041_cac0; continue; }
                    pc = 0x0041_cdd8; continue;
                }
                0x0041_cdd8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S2][0] & LONGINT;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = c.lw(sp + WRITTEN);
                    if t { pc = 0x0041_cdf4; continue; }
                    pc = 0x0041_cde4; continue;
                }
                0x0041_cde4 => {
                    c.cycles += 4;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_sub(0x8));
                    let t = true;
                    c.sd(c.g[V0][0] as u32, c.g[A2][0]);
                    if t { if back_edge_unwinds(c, 0x0041_cac0) { return Ok(()); } pc = 0x0041_cac0; continue; }
                    pc = 0x0041_cdf4; continue;
                }
                0x0041_cdf4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S2][0] & SHORTINT;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lw(sp + WRITTEN);
                    if t { pc = 0x0041_ce10; continue; }
                    pc = 0x0041_ce00; continue;
                }
                0x0041_ce00 => {
                    c.cycles += 4;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_sub(0x8));
                    let t = true;
                    c.sh(c.g[V0][0] as u32, c.g[V1][0]);
                    if t { if back_edge_unwinds(c, 0x0041_cac0) { return Ok(()); } pc = 0x0041_cac0; continue; }
                    pc = 0x0041_ce10; continue;
                }
                0x0041_ce10 => {
                    c.cycles += 5;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lw(sp + WRITTEN);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_sub(0x8));
                    let t = true;
                    c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                    if t { if back_edge_unwinds(c, 0x0041_cac0) { return Ok(()); } pc = 0x0041_cac0; continue; }
                    pc = 0x0041_ce24; continue;
                }
                // 'O'  long octal.
                0x0041_ce24 => {
                    c.cycles += 1;
                    c.g[S2][0] = c.g[S2][0] | LONGINT;
                    pc = 0x0041_ce28; continue;
                }
                // 'o'  octal.
                0x0041_ce28 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S2][0] & LONGLONG;
                    if c.g[V0][0] != 0u64 { c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8)); pc = 0x0041_ce44; continue; }
                    pc = 0x0041_ce34; continue;
                }
                0x0041_ce34 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S2][0] & LONGINT;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[S2][0] & SHORTINT;
                    if t { pc = 0x0041_ce4c; continue; }
                    pc = 0x0041_ce40; continue;
                }
                0x0041_ce40 => {
                    c.cycles += 1;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    pc = 0x0041_ce44; continue;
                }
                0x0041_ce44 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.ld((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t { pc = 0x0041_ce60; continue; }
                    pc = 0x0041_ce4c; continue;
                }
                0x0041_ce4c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t { pc = 0x0041_ce5c; continue; }
                    pc = 0x0041_ce54; continue;
                }
                0x0041_ce54 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.lhu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t { pc = 0x0041_ce60; continue; }
                    pc = 0x0041_ce5c; continue;
                }
                0x0041_ce5c => {
                    c.cycles += 1;
                    c.g[S0][0] = c.lwu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    pc = 0x0041_ce60; continue;
                }
                0x0041_ce60 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S6][0] = 0u64;
                    if t { pc = 0x0041_cf94; continue; }
                    pc = 0x0041_ce68; continue;
                }
                // 'p'  pointer: '#' + lowercase hex.
                0x0041_ce68 => {
                    c.cycles += 9;
                    c.g[V0][0] = 0x4a0000;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5a40));
                    c.g[S6][0] = 0x2;
                    c.sw(sp + BASE, c.g[V0][0]);
                    c.g[S2][0] = c.g[S2][0] | HASHX;
                    c.g[S5][0] = 0x78;
                    let t = true;
                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t { pc = 0x0041_cf94; continue; }
                    pc = 0x0041_ce8c; continue;
                }
                // 's'  string.
                0x0041_ce8c => {
                    c.cycles += 4;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[S4][0] = c.lw((c.g[S3][0] as u32).wrapping_sub(0x8));
                    let t = c.g[S4][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t { pc = 0x0041_cea0; continue; }
                    pc = 0x0041_ce9c; continue;
                }
                0x0041_ce9c => {
                    c.cycles += 1;
                    c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5a28));
                    pc = 0x0041_cea0; continue;
                }
                0x0041_cea0 => {
                    c.cycles += 2;
                    let t = (c.g[FP][0] as i64) < 0;
                    c.g[A0][0] = c.g[S4][0];
                    if t { pc = 0x0041_ced0; continue; }
                    pc = 0x0041_cea8; continue;
                }
                0x0041_cea8 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x0041ceb4);
                    c.g[A2][0] = c.g[FP][0];
                    callee::MEMCHR.call(c, 0x0041_ceb4)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S4][0] as u32));
                    if t { pc = 0x0041_cec8; continue; }
                    pc = 0x0041_cebc; continue;
                }
                0x0041_cebc => {
                    c.cycles += 3;
                    c.g[V1][0] = ((c.g[FP][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.sw(sp + LEN, c.g[V0][0]);
                    if t { pc = 0x0041_cedc; continue; }
                    pc = 0x0041_cec8; continue;
                }
                0x0041_cec8 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw(sp + LEN, c.g[FP][0]);
                    if t { pc = 0x0041_cedc; continue; }
                    pc = 0x0041_ced0; continue;
                }
                0x0041_ced0 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0041ced8);
                    c.g[A0][0] = c.g[S4][0];
                    callee::STRLEN.call(c, 0x0041_ced8)?;
                    c.sw(sp + LEN, c.g[V0][0]);
                    pc = 0x0041_cedc; continue;
                }
                0x0041_cedc => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S2][0] & LEFT;
                    c.sb(sp + SIGN, 0u64);
                    let t = true;
                    c.g[FP][0] = c.g[S2][0] & (ZEROPAD | LEFT);
                    if t { pc = 0x0041_d154; continue; }
                    pc = 0x0041_ceec; continue;
                }
                // 'U'  long unsigned decimal.
                0x0041_ceec => {
                    c.cycles += 1;
                    c.g[S2][0] = c.g[S2][0] | LONGINT;
                    pc = 0x0041_cef0; continue;
                }
                // 'u'  unsigned decimal.
                0x0041_cef0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S2][0] & LONGLONG;
                    if c.g[V0][0] != 0u64 { c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8)); pc = 0x0041_cf0c; continue; }
                    pc = 0x0041_cefc; continue;
                }
                0x0041_cefc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S2][0] & LONGINT;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[S2][0] & SHORTINT;
                    if t { pc = 0x0041_cf14; continue; }
                    pc = 0x0041_cf08; continue;
                }
                0x0041_cf08 => {
                    c.cycles += 1;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    pc = 0x0041_cf0c; continue;
                }
                0x0041_cf0c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.ld((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t { pc = 0x0041_cf28; continue; }
                    pc = 0x0041_cf14; continue;
                }
                0x0041_cf14 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t { pc = 0x0041_cf24; continue; }
                    pc = 0x0041_cf1c; continue;
                }
                0x0041_cf1c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.lhu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t { pc = 0x0041_cf28; continue; }
                    pc = 0x0041_cf24; continue;
                }
                0x0041_cf24 => {
                    c.cycles += 1;
                    c.g[S0][0] = c.lwu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    pc = 0x0041_cf28; continue;
                }
                0x0041_cf28 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S6][0] = 0x1;
                    if t { pc = 0x0041_cf94; continue; }
                    pc = 0x0041_cf30; continue;
                }
                // 'X'  uppercase hex.
                0x0041_cf30 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4a0000;
                    let t = true;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5a20));
                    if t { pc = 0x0041_cf44; continue; }
                    pc = 0x0041_cf3c; continue;
                }
                // 'x'  lowercase hex.
                0x0041_cf3c => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x4a0000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5a40));
                    pc = 0x0041_cf44; continue;
                }
                0x0041_cf44 => {
                    c.cycles += 4;
                    c.sw(sp + BASE, c.g[V0][0]);
                    c.g[V0][0] = c.g[S2][0] & LONGLONG;
                    if c.g[V0][0] != 0u64 { c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8)); pc = 0x0041_cf64; continue; }
                    pc = 0x0041_cf54; continue;
                }
                0x0041_cf54 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S2][0] & LONGINT;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[S2][0] & SHORTINT;
                    if t { pc = 0x0041_cf6c; continue; }
                    pc = 0x0041_cf60; continue;
                }
                0x0041_cf60 => {
                    c.cycles += 1;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    pc = 0x0041_cf64; continue;
                }
                0x0041_cf64 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.ld((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t { pc = 0x0041_cf80; continue; }
                    pc = 0x0041_cf6c; continue;
                }
                0x0041_cf6c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t { pc = 0x0041_cf7c; continue; }
                    pc = 0x0041_cf74; continue;
                }
                0x0041_cf74 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.lhu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    if t { pc = 0x0041_cf80; continue; }
                    pc = 0x0041_cf7c; continue;
                }
                0x0041_cf7c => {
                    c.cycles += 1;
                    c.g[S0][0] = c.lwu((c.g[S3][0] as u32).wrapping_sub(0x8));
                    pc = 0x0041_cf80; continue;
                }
                // 'x'/'X': '#' adds the 0x/0X prefix unless the value is 0.
                0x0041_cf80 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.g[S2][0] & ALT;
                    let t = c.g[V1][0] == 0u64;
                    c.g[S6][0] = 0x2;
                    if t { pc = 0x0041_cf94; continue; }
                    pc = 0x0041_cf8c; continue;
                }
                0x0041_cf8c => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S2][0] | HASHX;
                    if c.g[S0][0] != 0 { c.g[S2][0] = c.g[V0][0]; }
                    pc = 0x0041_cf94; continue;
                }
                // Number or string ready: no sign, resolve the precision.
                0x0041_cf94 => {
                    c.cycles += 1;
                    c.sb(sp + SIGN, 0u64);
                    pc = 0x0041_cf98; continue;
                }
                // prec = -1 when absent; a precision drops the zero padding.
                0x0041_cf98 => {
                    c.cycles += 3;
                    c.g[A2][0] = c.g[FP][0];
                    let t = (c.g[A2][0] as i64) < 0;
                    c.sw(sp + PREC, c.g[FP][0]);
                    if t { pc = 0x0041_cfac; continue; }
                    pc = 0x0041_cfa4; continue;
                }
                0x0041_cfa4 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0xffffffffffffff7f;
                    c.g[S2][0] = c.g[S2][0] & c.g[V0][0];
                    pc = 0x0041_cfac; continue;
                }
                // value == 0 prints one '0' (nothing when prec == 0).
                0x0041_cfac => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[S4][0] = sx(sp + NUMBUF);
                    if t { pc = 0x0041_cfc8; continue; }
                    pc = 0x0041_cfb4; continue;
                }
                0x0041_cfb4 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.g[S2][0] & LEFT;
                    c.g[V1][0] = c.lw(sp + PREC);
                    c.g[FP][0] = c.g[S2][0] & (ZEROPAD | LEFT);
                    let t = c.g[V1][0] == 0u64;
                    c.sw(sp + GROUP_FLAG, c.g[V0][0]);
                    if t { pc = 0x0041_d0f4; continue; }
                    pc = 0x0041_cfc8; continue;
                }
                // Pick the digit loop from the base (s6): 0 octal, 1 decimal, 2 hex.
                0x0041_cfc8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = c.g[S6][0] == c.g[V0][0];
                    c.g[A1][0] = c.g[S2][0] & LEFT;
                    if t { pc = 0x0041_d038; continue; }
                    pc = 0x0041_cfd4; continue;
                }
                0x0041_cfd4 => {
                    c.cycles += 2;
                    let t = c.g[S6][0] == 0u64;
                    c.g[V0][0] = 0x2;
                    if t { pc = 0x0041_cfec; continue; }
                    pc = 0x0041_cfdc; continue;
                }
                0x0041_cfdc => {
                    c.cycles += 2;
                    let t = c.g[S6][0] == c.g[V0][0];
                    c.g[V0][0] = 0x4a0000;
                    if t { pc = 0x0041_d098; continue; }
                    pc = 0x0041_cfe4; continue;
                }
                0x0041_cfe4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = c.g[S2][0] & LEFT;
                    if t { pc = 0x0041_d0d8; continue; }
                    pc = 0x0041_cfec; continue;
                }
                0x0041_cfec => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0] & LEFT;
                    c.g[V1][0] = c.g[S2][0] & ALT;
                    c.sw(sp + GROUP_FLAG, c.g[A0][0]);
                    c.g[FP][0] = c.g[S2][0] & (ZEROPAD | LEFT);
                    pc = 0x0041_d000; continue;
                }
                // Octal digits.
                0x0041_d000 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.g[S0][0] & 0x7;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = c.g[V0][0].wrapping_add(0x30);
                    c.g[S0][0] = c.g[S0][0] >> 3;
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    let t = c.g[S0][0] != 0u64;
                    c.sb(c.g[S4][0] as u32, c.g[V0][0]);
                    if t { if back_edge_unwinds(c, 0x0041_d000) { return Ok(()); } pc = 0x0041_d000; continue; }
                    pc = 0x0041_d01c; continue;
                }
                0x0041_d01c => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V1][0] = 0x30;
                    if t { pc = 0x0041_d0f4; continue; }
                    pc = 0x0041_d024; continue;
                }
                0x0041_d024 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == c.g[V1][0];
                    c.g[V0][0] = 0x1;
                    if t { pc = 0x0041_d0f8; continue; }
                    pc = 0x0041_d02c; continue;
                }
                0x0041_d02c => {
                    c.cycles += 3;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
                    let t = true;
                    c.sb(c.g[S4][0] as u32, c.g[V1][0]);
                    if t { pc = 0x0041_d0f8; continue; }
                    pc = 0x0041_d038; continue;
                }
                // Decimal digits (__umoddi3/__udivdi3).
                0x0041_d038 => {
                    c.cycles += 4;
                    c.g[V0][0] = (c.g[S0][0] < 0xau64) as u64;
                    c.g[FP][0] = c.g[S2][0] & (ZEROPAD | LEFT);
                    let t = c.g[V0][0] != 0u64;
                    c.sw(sp + GROUP_FLAG, c.g[A1][0]);
                    if t { pc = 0x0041_d084; continue; }
                    pc = 0x0041_d048; continue;
                }
                0x0041_d048 => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    pc = 0x0041_d050; continue;
                }
                0x0041_d050 => {
                    c.cycles += 13;
                    c.g[RA][0] = sx(0x0041d058);
                    c.g[A1][0] = 0xa;
                    callee::UMODDI3.call(c, 0x0041_d058)?;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = c.g[V0][0].wrapping_add(0x30);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.g[A1][0] = 0xa;
                    c.g[RA][0] = sx(0x0041d074);
                    c.sb(c.g[S4][0] as u32, c.g[V0][0]);
                    callee::UDIVDI3.call(c, 0x0041_d074)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = (c.g[S0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t { if back_edge_unwinds(c, 0x0041_d050) { return Ok(()); } pc = 0x0041_d050; continue; }
                    pc = 0x0041_d084; continue;
                }
                // Short decimal (value < 10).
                0x0041_d084 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.g[S0][0].wrapping_add(0x30);
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    let t = true;
                    c.sb(c.g[S4][0] as u32, c.g[V0][0]);
                    if t { pc = 0x0041_d0f4; continue; }
                    pc = 0x0041_d098; continue;
                }
                0x0041_d098 => {
                    c.cycles += 4;
                    c.g[A2][0] = c.g[S2][0] & LEFT;
                    c.g[FP][0] = c.g[S2][0] & (ZEROPAD | LEFT);
                    c.sw(sp + GROUP_FLAG, c.g[A2][0]);
                    pc = 0x0041_d0a8; continue;
                }
                // Hex digits from the table at BASE.
                0x0041_d0a8 => {
                    c.cycles += 10;
                    c.g[V1][0] = c.lw(sp + BASE);
                    c.g[V0][0] = c.g[S0][0] & 0xf;
                    c.g[V0][0] = c.g[V0][0] << 32;
                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S0][0] = c.g[S0][0] >> 4;
                    c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                    let t = c.g[S0][0] != 0u64;
                    c.sb(c.g[S4][0] as u32, c.g[V1][0]);
                    if t { if back_edge_unwinds(c, 0x0041_d0a8) { return Ok(()); } pc = 0x0041_d0a8; continue; }
                    pc = 0x0041_d0d0; continue;
                }
                0x0041_d0d0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t { pc = 0x0041_d0f8; continue; }
                    pc = 0x0041_d0d8; continue;
                }
                // Decimal digits with a positive value.
                0x0041_d0d8 => {
                    c.cycles += 7;
                    c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5a08));
                    c.sw(sp + GROUP_FLAG, c.g[A0][0]);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0041d0ec);
                    c.g[FP][0] = c.g[S2][0] & (ZEROPAD | LEFT);
                    callee::STRLEN.call(c, 0x0041_d0ec)?;
                    let t = true;
                    c.sw(sp + LEN, c.g[V0][0]);
                    if t { pc = 0x0041_d158; continue; }
                    pc = 0x0041_d0f4; continue;
                }
                0x0041_d0f4 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x0041_d0f8; continue;
                }
                0x0041_d0f8 => {
                    c.cycles += 2;
                    let t = c.g[S6][0] != c.g[V0][0];
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x28));
                    if t { pc = 0x0041_d128; continue; }
                    pc = 0x0041_d100; continue;
                }
                0x0041_d100 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S2][0] & GROUP;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S4][0];
                    if t { pc = 0x0041_d124; continue; }
                    pc = 0x0041_d10c; continue;
                }
                0x0041_d10c => {
                    c.cycles += 6;
                    c.g[A2][0] = sx(sp + NUMBUF);
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0041d11c);
                    c.g[A3][0] = sx(sp + LEN);
                    callee::INSERT_THOUSANDS_SEP.call(c, 0x0041_d11c)?;
                    let t = true;
                    c.g[S4][0] = c.g[V0][0];
                    if t { pc = 0x0041_d158; continue; }
                    pc = 0x0041_d124; continue;
                }
                // '\'' flag: insert the thousands separators.
                0x0041_d124 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x28));
                    pc = 0x0041_d128; continue;
                }
                0x0041_d128 => {
                    c.cycles += 3;
                    c.g[V1][0] = sx(sp.wrapping_sub(c.g[V0][0] as u32));
                    let t = true;
                    c.sw(sp + LEN, c.g[V1][0]);
                    if t { pc = 0x0041_d158; continue; }
                    pc = 0x0041_d134; continue;
                }
                // Not in the table: print the character literally, or stop at '\0'.
                0x0041_d134 => {
                    c.cycles += 2;
                    let t = c.g[S5][0] == 0u64;
                    c.g[V0][0] = 0x1;
                    if t { pc = 0x0041_d938; continue; }
                    pc = 0x0041_d13c; continue;
                }
                0x0041_d13c => {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S2][0] & LEFT;
                    c.sb(sp, c.g[S5][0]);
                    c.g[S4][0] = c.g[SP][0];
                    c.sw(sp + LEN, c.g[V0][0]);
                    c.g[FP][0] = c.g[S2][0] & (ZEROPAD | LEFT);
                    c.sb(sp + SIGN, 0u64);
                    pc = 0x0041_d154; continue;
                }
                0x0041_d154 => {
                    c.cycles += 1;
                    c.sw(sp + GROUP_FLAG, c.g[A1][0]);
                    pc = 0x0041_d158; continue;
                }
                // Field width: max(len, prec) plus one for the sign.
                0x0041_d158 => {
                    c.cycles += 7;
                    c.g[A2][0] = c.lw(sp + LEN);
                    c.g[A0][0] = c.lw(sp + PREC);
                    c.g[V1][0] = c.lb(sp + SIGN);
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 { c.g[A0][0] = c.g[A2][0]; }
                    let t = c.g[V1][0] == 0u64;
                    c.sw(sp + PAD, c.g[A0][0]);
                    if t { pc = 0x0041_d180; continue; }
                    pc = 0x0041_d174; continue;
                }
                0x0041_d174 => {
                    c.cycles += 3;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    let t = true;
                    c.sw(sp + PAD, c.g[A0][0]);
                    if t { pc = 0x0041_d190; continue; }
                    pc = 0x0041_d180; continue;
                }
                0x0041_d180 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.lw(sp + PAD);
                    c.g[V0][0] = c.g[S2][0] & HASHX;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.sw(sp + PAD, c.g[A1][0]);
                    pc = 0x0041_d190; continue;
                }
                // Padding and output.
                0x0041_d190 => {
                    c.cycles += 2;
                    let t = c.g[FP][0] != 0u64;
                    c.g[V0][0] = c.lb(sp + SIGN);
                    if t { pc = 0x0041_d300; continue; }
                    pc = 0x0041_d198; continue;
                }
                0x0041_d198 => {
                    c.cycles += 5;
                    c.g[A2][0] = c.lw(sp + WIDTH);
                    c.g[V0][0] = c.lw(sp + PAD);
                    c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    let t = (c.g[S0][0] as i64) <= 0;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 17i64) as u64;
                    if t { pc = 0x0041_d2fc; continue; }
                    pc = 0x0041_d1ac; continue;
                }
                0x0041_d1ac => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S6][0] = 0x4a0000;
                    if t { pc = 0x0041_d26c; continue; }
                    pc = 0x0041_d1b4; continue;
                }
                0x0041_d1b4 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                    pc = 0x0041_d1b8; continue;
                }
                // Pad with spaces in 16-byte chunks (through vfiprintf_BufferedWrite).
                0x0041_d1b8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] & 0x200;
                    if c.g[V0][0] == 0u64 { c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc)); pc = 0x0041_d238; continue; }
                    pc = 0x0041_d1c4; continue;
                }
                0x0041_d1c4 => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 17i64) as u64;
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lw(c.g[S1][0] as u32); pc = 0x0041_d218; continue; }
                    pc = 0x0041_d1d4; continue;
                }
                0x0041_d1d4 => {
                    c.cycles += 17;
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5a60));
                    c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                    c.ldr(3, c.g[A2][0] as u32);
                    c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                    c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                    c.sdr(3, c.g[V0][0] as u32);
                    c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
                    c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t { pc = 0x0041_d25c; continue; }
                    pc = 0x0041_d218; continue;
                }
                // The buffer holds 16 padding bytes: flush them.
                0x0041_d218 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x0041d220);
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5a60));
                    callee::MEMCPY.call(c, 0x0041_d220)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    if t { pc = 0x0041_d25c; continue; }
                    pc = 0x0041_d238; continue;
                }
                0x0041_d238 => {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe)); pc = 0x0041_d24c; continue; }
                    pc = 0x0041_d240; continue;
                }
                0x0041_d240 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    c.sh((c.g[S1][0] as u32).wrapping_add(0xe), c.g[V0][0]);
                    c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe));
                    pc = 0x0041_d24c; continue;
                }
                0x0041_d24c => {
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5a60));
                    c.g[A2][0] = 0x10;
                    c.g[RA][0] = sx(0x0041d25c);
                    c.g[A3][0] = 0u64;
                    callee::BUFFERED_WRITE.call(c, 0x0041_d25c)?;
                    pc = 0x0041_d25c; continue;
                }
                0x0041_d25c => {
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x10));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 17i64) as u64;
                    if c.g[V0][0] == 0u64 { c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc)); if back_edge_unwinds(c, 0x0041_d1b8) { return Ok(()); } pc = 0x0041_d1b8; continue; }
                    pc = 0x0041_d26c; continue;
                }
                0x0041_d26c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] & 0x200;
                    if c.g[V0][0] == 0u64 { c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc)); pc = 0x0041_d2d8; continue; }
                    pc = 0x0041_d27c; continue;
                }
                0x0041_d27c => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5a60));
                    if t { pc = 0x0041_d2b4; continue; }
                    pc = 0x0041_d28c; continue;
                }
                0x0041_d28c => {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[RA][0] = sx(0x0041d298);
                    c.g[A2][0] = c.g[S0][0];
                    callee::MEMCPY.call(c, 0x0041_d298)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t { pc = 0x0041_d2fc; continue; }
                    pc = 0x0041_d2b4; continue;
                }
                0x0041_d2b4 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[RA][0] = sx(0x0041d2c0);
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5a60));
                    callee::MEMCPY.call(c, 0x0041_d2c0)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    if t { pc = 0x0041_d2fc; continue; }
                    pc = 0x0041_d2d8; continue;
                }
                0x0041_d2d8 => {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe)); pc = 0x0041_d2ec; continue; }
                    pc = 0x0041_d2e0; continue;
                }
                0x0041_d2e0 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    c.sh((c.g[S1][0] as u32).wrapping_add(0xe), c.g[V0][0]);
                    c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe));
                    pc = 0x0041_d2ec; continue;
                }
                0x0041_d2ec => {
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5a60));
                    c.g[A2][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0041d2fc);
                    c.g[A3][0] = 0u64;
                    callee::BUFFERED_WRITE.call(c, 0x0041_d2fc)?;
                    pc = 0x0041_d2fc; continue;
                }
                0x0041_d2fc => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lb(sp + SIGN);
                    pc = 0x0041_d300; continue;
                }
                // Zero padding?
                0x0041_d300 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lbu(sp + SIGN);
                    if t { pc = 0x0041_d378; continue; }
                    pc = 0x0041_d308; continue;
                }
                0x0041_d308 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] & 0x200;
                    if c.g[V0][0] == 0u64 { c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc)); pc = 0x0041_d34c; continue; }
                    pc = 0x0041_d318; continue;
                }
                0x0041_d318 => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 2i64) as u64;
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lw(c.g[S1][0] as u32); pc = 0x0041_d344; continue; }
                    pc = 0x0041_d328; continue;
                }
                0x0041_d328 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    c.sb(c.g[V0][0] as u32, c.g[V1][0]);
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    let t = true;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    if t { pc = 0x0041_d3cc; continue; }
                    pc = 0x0041_d344; continue;
                }
                0x0041_d344 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = sx(sp + SIGN);
                    if t { pc = 0x0041_d3dc; continue; }
                    pc = 0x0041_d34c; continue;
                }
                0x0041_d34c => {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe)); pc = 0x0041_d360; continue; }
                    pc = 0x0041_d354; continue;
                }
                0x0041_d354 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    c.sh((c.g[S1][0] as u32).wrapping_add(0xe), c.g[V0][0]);
                    c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe));
                    pc = 0x0041_d360; continue;
                }
                0x0041_d360 => {
                    c.cycles += 6;
                    c.g[A1][0] = sx(sp + SIGN);
                    c.g[A2][0] = 0x1;
                    c.g[RA][0] = sx(0x0041d370);
                    c.g[A3][0] = 0u64;
                    callee::BUFFERED_WRITE.call(c, 0x0041_d370)?;
                    let t = true;
                    c.g[V0][0] = 0x80;
                    if t { pc = 0x0041_d428; continue; }
                    pc = 0x0041_d378; continue;
                }
                0x0041_d378 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S2][0] & HASHX;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x30;
                    if t { pc = 0x0041_d424; continue; }
                    pc = 0x0041_d384; continue;
                }
                0x0041_d384 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.sb(sp + PREFIXBUF, c.g[V0][0]);
                    c.g[V1][0] = c.g[V1][0] & 0x200;
                    let t = c.g[V1][0] == 0u64;
                    c.sb(sp + PREFIXBUF + 1, c.g[S5][0]);
                    if t { pc = 0x0041_d3fc; continue; }
                    pc = 0x0041_d398; continue;
                }
                0x0041_d398 => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 3i64) as u64;
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lw(c.g[S1][0] as u32); pc = 0x0041_d3d8; continue; }
                    pc = 0x0041_d3a8; continue;
                }
                0x0041_d3a8 => {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V1][0] = c.lb(sp + PREFIXBUF);
                    c.g[A0][0] = c.lb(sp + PREFIXBUF + 1);
                    c.sb(c.g[V0][0] as u32, c.g[V1][0]);
                    c.sb((c.g[V0][0] as u32).wrapping_add(0x1), c.g[A0][0]);
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
                    pc = 0x0041_d3cc; continue;
                }
                0x0041_d3cc => {
                    c.cycles += 3;
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t { pc = 0x0041_d424; continue; }
                    pc = 0x0041_d3d8; continue;
                }
                0x0041_d3d8 => {
                    c.cycles += 1;
                    c.g[A1][0] = sx(sp + PREFIXBUF);
                    pc = 0x0041_d3dc; continue;
                }
                0x0041_d3dc => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x0041d3e4);
                    callee::MEMCPY.call(c, 0x0041_d3e4)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    if t { pc = 0x0041_d424; continue; }
                    pc = 0x0041_d3fc; continue;
                }
                0x0041_d3fc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe)); pc = 0x0041_d414; continue; }
                    pc = 0x0041_d408; continue;
                }
                0x0041_d408 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    c.sh((c.g[S1][0] as u32).wrapping_add(0xe), c.g[V0][0]);
                    c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe));
                    pc = 0x0041_d414; continue;
                }
                0x0041_d414 => {
                    c.cycles += 4;
                    c.g[A1][0] = sx(sp + PREFIXBUF);
                    c.g[A2][0] = 0x2;
                    c.g[RA][0] = sx(0x0041d424);
                    c.g[A3][0] = 0u64;
                    callee::BUFFERED_WRITE.call(c, 0x0041_d424)?;
                    pc = 0x0041_d424; continue;
                }
                0x0041_d424 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x80;
                    pc = 0x0041_d428; continue;
                }
                // The '0' prefix of an alternate octal.
                0x0041_d428 => {
                    c.cycles += 2;
                    let t = c.g[FP][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(sp + LEN);
                    if t { pc = 0x0041_d598; continue; }
                    pc = 0x0041_d430; continue;
                }
                0x0041_d430 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(sp + WIDTH);
                    c.g[V1][0] = c.lw(sp + PAD);
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    let t = (c.g[S0][0] as i64) <= 0;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 17i64) as u64;
                    if t { pc = 0x0041_d594; continue; }
                    pc = 0x0041_d444; continue;
                }
                0x0041_d444 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = 0x4a0000;
                    if t { pc = 0x0041_d504; continue; }
                    pc = 0x0041_d44c; continue;
                }
                0x0041_d44c => {
                    c.cycles += 3;
                    c.g[S5][0] = 0x1;
                    c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                    pc = 0x0041_d458; continue;
                }
                // Zero pad in 16-byte chunks.
                0x0041_d458 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] & 0x200;
                    if c.g[V0][0] == 0u64 { c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc)); pc = 0x0041_d4d8; continue; }
                    pc = 0x0041_d464; continue;
                }
                0x0041_d464 => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 17i64) as u64;
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lw(c.g[S1][0] as u32); pc = 0x0041_d4b8; continue; }
                    pc = 0x0041_d474; continue;
                }
                0x0041_d474 => {
                    c.cycles += 17;
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5a50));
                    c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(4, c.g[V1][0] as u32);
                    c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x7));
                    c.sdr(4, c.g[V0][0] as u32);
                    c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
                    c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t { pc = 0x0041_d4f4; continue; }
                    pc = 0x0041_d4b8; continue;
                }
                0x0041_d4b8 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x0041d4c0);
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5a50));
                    callee::MEMCPY.call(c, 0x0041_d4c0)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    if t { pc = 0x0041_d4f4; continue; }
                    pc = 0x0041_d4d8; continue;
                }
                0x0041_d4d8 => {
                    c.cycles += 2;
                    if c.g[V0][0] == 0u64 { c.sh((c.g[S1][0] as u32).wrapping_add(0xe), c.g[S5][0]); pc = 0x0041_d4e0; continue; }
                    pc = 0x0041_d4e0; continue;
                }
                0x0041_d4e0 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe));
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5a50));
                    c.g[A2][0] = 0x10;
                    c.g[RA][0] = sx(0x0041d4f4);
                    c.g[A3][0] = 0u64;
                    callee::BUFFERED_WRITE.call(c, 0x0041_d4f4)?;
                    pc = 0x0041_d4f4; continue;
                }
                0x0041_d4f4 => {
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x10));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 17i64) as u64;
                    if c.g[V0][0] == 0u64 { c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc)); if back_edge_unwinds(c, 0x0041_d458) { return Ok(()); } pc = 0x0041_d458; continue; }
                    pc = 0x0041_d504; continue;
                }
                // Chunked zero padding.
                0x0041_d504 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] & 0x200;
                    if c.g[V0][0] == 0u64 { c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc)); pc = 0x0041_d570; continue; }
                    pc = 0x0041_d514; continue;
                }
                0x0041_d514 => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5a50));
                    if t { pc = 0x0041_d54c; continue; }
                    pc = 0x0041_d524; continue;
                }
                0x0041_d524 => {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[RA][0] = sx(0x0041d530);
                    c.g[A2][0] = c.g[S0][0];
                    callee::MEMCPY.call(c, 0x0041_d530)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t { pc = 0x0041_d594; continue; }
                    pc = 0x0041_d54c; continue;
                }
                0x0041_d54c => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[RA][0] = sx(0x0041d558);
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5a50));
                    callee::MEMCPY.call(c, 0x0041_d558)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    if t { pc = 0x0041_d594; continue; }
                    pc = 0x0041_d570; continue;
                }
                0x0041_d570 => {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe)); pc = 0x0041_d584; continue; }
                    pc = 0x0041_d578; continue;
                }
                0x0041_d578 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    c.sh((c.g[S1][0] as u32).wrapping_add(0xe), c.g[V0][0]);
                    c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe));
                    pc = 0x0041_d584; continue;
                }
                0x0041_d584 => {
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5a50));
                    c.g[A2][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0041d594);
                    c.g[A3][0] = 0u64;
                    callee::BUFFERED_WRITE.call(c, 0x0041_d594)?;
                    pc = 0x0041_d594; continue;
                }
                0x0041_d594 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(sp + LEN);
                    pc = 0x0041_d598; continue;
                }
                // Sign, prefix, digits.
                0x0041_d598 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(sp + PREC);
                    c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    let t = (c.g[S0][0] as i64) <= 0;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 17i64) as u64;
                    if t { pc = 0x0041_d6f4; continue; }
                    pc = 0x0041_d5a8; continue;
                }
                0x0041_d5a8 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = 0x4a0000;
                    if t { pc = 0x0041_d664; continue; }
                    pc = 0x0041_d5b0; continue;
                }
                0x0041_d5b0 => {
                    c.cycles += 2;
                    c.g[S5][0] = 0x1;
                    c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                    pc = 0x0041_d5b8; continue;
                }
                // Padding after a left-justified number.
                0x0041_d5b8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] & 0x200;
                    if c.g[V0][0] == 0u64 { c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc)); pc = 0x0041_d638; continue; }
                    pc = 0x0041_d5c4; continue;
                }
                0x0041_d5c4 => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 17i64) as u64;
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lw(c.g[S1][0] as u32); pc = 0x0041_d618; continue; }
                    pc = 0x0041_d5d4; continue;
                }
                0x0041_d5d4 => {
                    c.cycles += 17;
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5a50));
                    c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.ldr(5, c.g[A0][0] as u32);
                    c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x7));
                    c.sdr(5, c.g[V0][0] as u32);
                    c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0xf));
                    c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t { pc = 0x0041_d654; continue; }
                    pc = 0x0041_d618; continue;
                }
                0x0041_d618 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x0041d620);
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5a50));
                    callee::MEMCPY.call(c, 0x0041_d620)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    if t { pc = 0x0041_d654; continue; }
                    pc = 0x0041_d638; continue;
                }
                0x0041_d638 => {
                    c.cycles += 2;
                    if c.g[V0][0] == 0u64 { c.sh((c.g[S1][0] as u32).wrapping_add(0xe), c.g[S5][0]); pc = 0x0041_d640; continue; }
                    pc = 0x0041_d640; continue;
                }
                0x0041_d640 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe));
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5a50));
                    c.g[A2][0] = 0x10;
                    c.g[RA][0] = sx(0x0041d654);
                    c.g[A3][0] = 0u64;
                    callee::BUFFERED_WRITE.call(c, 0x0041_d654)?;
                    pc = 0x0041_d654; continue;
                }
                0x0041_d654 => {
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x10));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 17i64) as u64;
                    if c.g[V0][0] == 0u64 { c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc)); if back_edge_unwinds(c, 0x0041_d5b8) { return Ok(()); } pc = 0x0041_d5b8; continue; }
                    pc = 0x0041_d664; continue;
                }
                0x0041_d664 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] & 0x200;
                    if c.g[V0][0] == 0u64 { c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc)); pc = 0x0041_d6d0; continue; }
                    pc = 0x0041_d674; continue;
                }
                0x0041_d674 => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5a50));
                    if t { pc = 0x0041_d6ac; continue; }
                    pc = 0x0041_d684; continue;
                }
                0x0041_d684 => {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[RA][0] = sx(0x0041d690);
                    c.g[A2][0] = c.g[S0][0];
                    callee::MEMCPY.call(c, 0x0041_d690)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t { pc = 0x0041_d6f4; continue; }
                    pc = 0x0041_d6ac; continue;
                }
                0x0041_d6ac => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[RA][0] = sx(0x0041d6b8);
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5a50));
                    callee::MEMCPY.call(c, 0x0041_d6b8)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    if t { pc = 0x0041_d6f4; continue; }
                    pc = 0x0041_d6d0; continue;
                }
                0x0041_d6d0 => {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe)); pc = 0x0041_d6e4; continue; }
                    pc = 0x0041_d6d8; continue;
                }
                0x0041_d6d8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    c.sh((c.g[S1][0] as u32).wrapping_add(0xe), c.g[V0][0]);
                    c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe));
                    pc = 0x0041_d6e4; continue;
                }
                0x0041_d6e4 => {
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5a50));
                    c.g[A2][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0041d6f4);
                    c.g[A3][0] = 0u64;
                    callee::BUFFERED_WRITE.call(c, 0x0041_d6f4)?;
                    pc = 0x0041_d6f4; continue;
                }
                0x0041_d6f4 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] & 0x200;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = c.lw(sp + LEN);
                    if t { pc = 0x0041_d760; continue; }
                    pc = 0x0041_d704; continue;
                }
                0x0041_d704 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    if t { pc = 0x0041_d73c; continue; }
                    pc = 0x0041_d714; continue;
                }
                0x0041_d714 => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x0041d71c);
                    c.g[A1][0] = c.g[S4][0];
                    callee::MEMCPY.call(c, 0x0041_d71c)?;
                    c.g[A0][0] = c.lw(sp + LEN);
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t { pc = 0x0041_d788; continue; }
                    pc = 0x0041_d73c; continue;
                }
                0x0041_d73c => {
                    c.cycles += 9;
                    c.g[A2][0] = c.g[V1][0];
                    c.g[RA][0] = sx(0x0041d748);
                    c.g[A1][0] = c.g[S4][0];
                    callee::MEMCPY.call(c, 0x0041_d748)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    if t { pc = 0x0041_d788; continue; }
                    pc = 0x0041_d760; continue;
                }
                0x0041_d760 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe)); pc = 0x0041_d778; continue; }
                    pc = 0x0041_d76c; continue;
                }
                0x0041_d76c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    c.sh((c.g[S1][0] as u32).wrapping_add(0xe), c.g[V0][0]);
                    c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe));
                    pc = 0x0041_d778; continue;
                }
                0x0041_d778 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A2][0] = c.lw(sp + LEN);
                    c.g[RA][0] = sx(0x0041d788);
                    c.g[A3][0] = 0u64;
                    callee::BUFFERED_WRITE.call(c, 0x0041_d788)?;
                    pc = 0x0041_d788; continue;
                }
                0x0041_d788 => {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw(sp + GROUP_FLAG);
                    let t = c.g[A1][0] == 0u64;
                    c.g[A2][0] = c.lw(sp + WIDTH);
                    if t { pc = 0x0041_d8f4; continue; }
                    pc = 0x0041_d794; continue;
                }
                0x0041_d794 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(sp + PAD);
                    c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    let t = (c.g[S0][0] as i64) <= 0;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 17i64) as u64;
                    if t { pc = 0x0041_d8f4; continue; }
                    pc = 0x0041_d7a4; continue;
                }
                0x0041_d7a4 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S6][0] = 0x4a0000;
                    if t { pc = 0x0041_d864; continue; }
                    pc = 0x0041_d7ac; continue;
                }
                0x0041_d7ac => {
                    c.cycles += 3;
                    c.g[S2][0] = 0x1;
                    c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                    pc = 0x0041_d7b8; continue;
                }
                // Padding after a left-adjusted string (spaces).
                0x0041_d7b8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] & 0x200;
                    if c.g[V0][0] == 0u64 { c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc)); pc = 0x0041_d838; continue; }
                    pc = 0x0041_d7c4; continue;
                }
                0x0041_d7c4 => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 17i64) as u64;
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lw(c.g[S1][0] as u32); pc = 0x0041_d818; continue; }
                    pc = 0x0041_d7d4; continue;
                }
                0x0041_d7d4 => {
                    c.cycles += 17;
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5a60));
                    c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                    c.ldr(3, c.g[A2][0] as u32);
                    c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                    c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                    c.sdr(3, c.g[V0][0] as u32);
                    c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
                    c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t { pc = 0x0041_d854; continue; }
                    pc = 0x0041_d818; continue;
                }
                0x0041_d818 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x0041d820);
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5a60));
                    callee::MEMCPY.call(c, 0x0041_d820)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    if t { pc = 0x0041_d854; continue; }
                    pc = 0x0041_d838; continue;
                }
                0x0041_d838 => {
                    c.cycles += 2;
                    if c.g[V0][0] == 0u64 { c.sh((c.g[S1][0] as u32).wrapping_add(0xe), c.g[S2][0]); pc = 0x0041_d840; continue; }
                    pc = 0x0041_d840; continue;
                }
                0x0041_d840 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe));
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5a60));
                    c.g[A2][0] = 0x10;
                    c.g[RA][0] = sx(0x0041d854);
                    c.g[A3][0] = 0u64;
                    callee::BUFFERED_WRITE.call(c, 0x0041_d854)?;
                    pc = 0x0041_d854; continue;
                }
                0x0041_d854 => {
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x10));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 17i64) as u64;
                    if c.g[V0][0] == 0u64 { c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc)); if back_edge_unwinds(c, 0x0041_d7b8) { return Ok(()); } pc = 0x0041_d7b8; continue; }
                    pc = 0x0041_d864; continue;
                }
                0x0041_d864 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] & 0x200;
                    if c.g[V0][0] == 0u64 { c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc)); pc = 0x0041_d8d0; continue; }
                    pc = 0x0041_d874; continue;
                }
                0x0041_d874 => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5a60));
                    if t { pc = 0x0041_d8ac; continue; }
                    pc = 0x0041_d884; continue;
                }
                0x0041_d884 => {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[RA][0] = sx(0x0041d890);
                    c.g[A2][0] = c.g[S0][0];
                    callee::MEMCPY.call(c, 0x0041_d890)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t { pc = 0x0041_d8f4; continue; }
                    pc = 0x0041_d8ac; continue;
                }
                0x0041_d8ac => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[RA][0] = sx(0x0041d8b8);
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5a60));
                    callee::MEMCPY.call(c, 0x0041_d8b8)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    if t { pc = 0x0041_d8f4; continue; }
                    pc = 0x0041_d8d0; continue;
                }
                0x0041_d8d0 => {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe)); pc = 0x0041_d8e4; continue; }
                    pc = 0x0041_d8d8; continue;
                }
                0x0041_d8d8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    c.sh((c.g[S1][0] as u32).wrapping_add(0xe), c.g[V0][0]);
                    c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe));
                    pc = 0x0041_d8e4; continue;
                }
                0x0041_d8e4 => {
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5a60));
                    c.g[A2][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0041d8f4);
                    c.g[A3][0] = 0u64;
                    callee::BUFFERED_WRITE.call(c, 0x0041_d8f4)?;
                    pc = 0x0041_d8f4; continue;
                }
                // Pad after a left-justified conversion, 16 bytes at a time.
                0x0041_d8f4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    let t = (c.g[V0][0] as i64) > 0;
                    c.g[V1][0] = c.lw(sp + PAD);
                    if t { pc = 0x0041_d91c; continue; }
                    pc = 0x0041_d900; continue;
                }
                0x0041_d900 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] & 0x200;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lw(sp + WIDTH);
                    if t { pc = 0x0041_d920; continue; }
                    pc = 0x0041_d910; continue;
                }
                0x0041_d910 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    let t = true;
                    c.sw(sp + WRITTEN, c.g[V0][0]);
                    if t { if back_edge_unwinds(c, 0x0041_cac0) { return Ok(()); } pc = 0x0041_cac0; continue; }
                    pc = 0x0041_d91c; continue;
                }
                0x0041_d91c => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(sp + WIDTH);
                    pc = 0x0041_d920; continue;
                }
                0x0041_d920 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw(sp + WRITTEN);
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 { c.g[A0][0] = c.g[V1][0]; }
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    let t = true;
                    c.sw(sp + WRITTEN, c.g[A1][0]);
                    if t { if back_edge_unwinds(c, 0x0041_cac0) { return Ok(()); } pc = 0x0041_cac0; continue; }
                    pc = 0x0041_d938; continue;
                }
                0x0041_d938 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                    pc = 0x0041_d93c; continue;
                }
                // A conversion follows.
                0x0041_d93c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] & 0x200;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw(sp + WRITTEN);
                    if t { pc = 0x0041_d960; continue; }
                    pc = 0x0041_d948; continue;
                }
                0x0041_d948 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe));
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x0041d95c);
                    c.g[A3][0] = 0x1;
                    callee::BUFFERED_WRITE.call(c, 0x0041_d95c)?;
                    c.g[V0][0] = c.lw(sp + WRITTEN);
                    pc = 0x0041_d960; continue;
                }
                // Epilogue: restore f16-f25, f30 and ra; return written.
                0x0041_d960 => {
                    c.cycles += 12;
                    c.lq(16, sp + 0x100);
                    c.lq(17, sp + 0xf0);
                    c.lq(18, sp + 0xe0);
                    c.lq(19, sp + 0xd0);
                    c.lq(20, sp + 0xc0);
                    c.lq(21, sp + 0xb0);
                    c.lq(22, sp + 0xa0);
                    c.lq(23, sp + 0x90);
                    c.lq(30, sp + 0x80);
                    c.g[RA][0] = c.ld(sp + 0x70);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx(sp + 0x110);
                    c.pc = t; return Ok(());
                }
                other => {
                    c.jump_out(other, 0x0041_ca70, 0x0041_d990);
                    return Ok(());
                }
            }
        }

    })
}

// ------------------------------------------------------------ _fpadd_parts_d

/// `fp_number_type* _fpadd_parts_d(fp_number_type* a, fp_number_type* b,
/// fp_number_type* tmp)` (0x411cd8, libgcc soft-float): the core of
/// `__adddf3`/`__subdf3` on unpacked operands (class, sign, exponent,
/// fraction; see `__unpack_d`).
///
/// - A NaN operand is returned as is (`a` first); infinity + infinity of
///   opposite signs returns the static NaN (0x52adc0); an infinity or a zero
///   operand returns the other one; zero + zero writes `tmp` as a copy of
///   `a` with the signs and-ed.
/// - Otherwise the smaller exponent's fraction is shifted right to align
///   (keeping a sticky low bit), or dropped when the exponents are 64 or
///   more apart; the fractions are added or subtracted by sign into `tmp`
///   (negative differences flip the sign), normalised left until the
///   implicit one reaches bit 60 and shifted right once (sticky) when the
///   sum overflowed past it. `tmp` becomes class "number"; returns `tmp`.
///
/// Registers as the original: t0 = a (then a's sign), a1 = b (then b's
/// sign, then the fraction), a3 = the exponent, t2/t3 = b's/a's fraction.
pub fn fpadd_parts_d(c: &mut Cpu) {
    const CLASS: u32 = 0x0;
    const SIGN: u32 = 0x4;
    /// The static NaN result (`0x530000 - 0x5240`).
    const NAN: u64 = 0x0052_adc0;

    'ret_a: {
        c.cycles += 5;
        c.g[T0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(at(c, T0, CLASS));
        c.g[V0][0] = (c.g[A0][0] < 2) as u64;
        if c.g[V0][0] != 0 {
            break 'ret_a; // a is a NaN
        }
        c.g[V1][0] = c.lw(at(c, A1, CLASS));
        'ret_b: {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V1][0] < 2) as u64;
            let b_nan = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[A0][0] ^ 0x4;
            if b_nan {
                break 'ret_b;
            }
            c.cycles += 2;
            let a_inf = c.g[V0][0] == 0;
            c.g[V0][0] = c.g[V1][0] ^ 0x4;
            if a_inf {
                // inf + x: x finite, or the same infinity, gives a.
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    break 'ret_a;
                }
                c.cycles += 4;
                c.g[V1][0] = c.lw(at(c, A1, SIGN));
                c.g[V0][0] = c.lw(at(c, T0, SIGN));
                let same = c.g[V0][0] == c.g[V1][0];
                c.g[V0][0] = 0x0053_0000;
                if same {
                    break 'ret_a;
                }
                c.cycles += 2;
                c.g[V0][0] = NAN;
                ret(c);
                return;
            }
            c.cycles += 2;
            let b_inf = c.g[V0][0] == 0;
            c.g[V0][0] = c.g[V1][0] ^ 0x2;
            if b_inf {
                break 'ret_b;
            }
            c.cycles += 2;
            let b_zero = c.g[V0][0] == 0;
            c.g[V0][0] = c.g[A0][0] ^ 0x2;
            if b_zero {
                c.cycles += 2;
                let a_nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = c.g[A2][0];
                if a_nonzero {
                    break 'ret_a;
                }
                // zero + zero: tmp = a with sign a.sign & b.sign.
                c.cycles += 11;
                c.g[A0][0] = c.ld(at(c, T0, 0));
                c.sd(at(c, A2, 0), c.g[A0][0]);
                c.g[V1][0] = c.ld(at(c, T0, 0x8));
                c.sd(at(c, A2, 0x8), c.g[V1][0]);
                c.g[A0][0] = c.ld(at(c, T0, 0x10));
                c.sd(at(c, A2, 0x10), c.g[A0][0]);
                c.g[V1][0] = c.lw(at(c, T0, SIGN));
                c.g[A0][0] = c.lw(at(c, A1, SIGN));
                c.g[V1][0] &= c.g[A0][0];
                c.sw(at(c, A2, SIGN), c.g[V1][0]);
                ret(c);
                return;
            }
            c.cycles += 2;
            let a_zero = c.g[V0][0] == 0;
            c.g[V0][0] = u64::MAX;
            if a_zero {
                break 'ret_b;
            }
            fpadd_numbers(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A1][0];
        ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T0][0];
    ret(c);
}

/// `_fpadd_parts_d` on two numbers (from 0x411d74; v0 = -1, t0 = a, a1 = b,
/// a2 = tmp): align, add, normalise, return `tmp`.
pub(crate) fn fpadd_numbers(c: &mut Cpu) {
    const SIGN: u32 = 0x4;
    const EXP: u32 = 0x8;
    const FRAC: u32 = 0x10;
    /// The implicit one's bit (60) times two, minus one: fractions at or
    /// below it are normalised further.
    const NORMAL_LIMIT: u64 = (u64::MAX << 5) >> 4;
    /// Above this the sum carried past bit 60.
    const OVERFLOW_LIMIT: u64 = u64::MAX >> 3;

    // Align the exponents.
    'aligned: {
        'far_apart: {
            c.cycles += 10;
            c.g[A3][0] = c.lw(at(c, T0, EXP)); // a.exp
            c.g[T1][0] = c.lw(at(c, A1, EXP)); // b.exp
            c.g[T3][0] = c.ld(at(c, T0, FRAC)); // a.frac
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[T1][0] as u32));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            c.g[A0][0] = sx(0u32.wrapping_sub(c.g[V1][0] as u32));
            if c.g[V0][0] == 0 {
                c.g[V1][0] = c.g[A0][0];
            }
            c.g[V1][0] = ((c.g[V1][0] as i64) < 64) as u64;
            c.g[T2][0] = c.ld(at(c, A1, FRAC)); // b.frac
            if c.g[V1][0] == 0 {
                break 'far_apart;
            }
            // Shift b's fraction right while b.exp < a.exp (sticky bit 0).
            'shift_a: {
                'b_not_smaller: {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    let b_smaller = c.g[V0][0] != 0;
                    c.g[T0][0] = c.lw(at(c, T0, SIGN));
                    if !b_smaller {
                        break 'b_not_smaller;
                    }
                    c.cycles += 2;
                    c.g[A1][0] = c.lw(at(c, A1, SIGN));
                    loop {
                        c.cycles += 7;
                        c.g[V0][0] = c.g[T2][0] >> 1;
                        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
                        c.g[V1][0] = c.g[T2][0] & 0x1;
                        c.g[A0][0] = ((c.g[T1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                        c.g[T2][0] = c.g[V1][0] | c.g[V0][0];
                        if c.g[A0][0] == 0 {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0041_1db8);
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    break 'shift_a;
                }
                c.cycles += 2;
                c.g[A1][0] = c.lw(at(c, A1, SIGN));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T1][0] as i64)) as u64;
            }
            // Shift a's fraction right by b.exp - a.exp (sticky bit 0).
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                break 'aligned;
            }
            c.cycles += 1;
            c.g[A3][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[A3][0] as u32));
            loop {
                c.cycles += 7;
                c.g[V1][0] = c.g[T3][0] >> 1;
                c.g[V0][0] = c.g[T3][0] & 0x1;
                c.g[T3][0] = c.g[V0][0] | c.g[V1][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
                if c.g[A3][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0041_1df0);
                }
            }
            c.cycles += 2;
            c.g[A3][0] = c.g[T1][0];
            break 'aligned;
        }
        // 64 or more apart: the smaller operand's fraction drops out.
        c.cycles += 3;
        c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A3][0] as i64)) as u64;
        let b_smaller = c.g[V0][0] != 0;
        c.g[T0][0] = c.lw(at(c, T0, SIGN));
        if b_smaller {
            c.cycles += 3;
            c.g[T2][0] = 0;
            c.g[A1][0] = c.lw(at(c, A1, SIGN));
            break 'aligned;
        }
        c.cycles += 3;
        c.g[A3][0] = c.g[T1][0];
        c.g[A1][0] = c.lw(at(c, A1, SIGN));
        c.g[T3][0] = 0;
    }

    // Add (same signs) or subtract (the negative one from the other).
    'normalised: {
        c.cycles += 2;
        let same_sign = c.g[T0][0] == c.g[A1][0];
        c.g[V0][0] = c.g[T3][0].wrapping_add(c.g[T2][0]);
        if same_sign {
            c.cycles += 4;
            c.sw(at(c, A2, SIGN), c.g[T0][0]);
            c.sw(at(c, A2, EXP), c.g[A3][0]);
            c.g[A1][0] = c.g[V0][0];
            c.sd(at(c, A2, FRAC), c.g[V0][0]);
            break 'normalised;
        }
        c.cycles += 2;
        let a_negative = c.g[T0][0] != 0;
        c.g[V0][0] = c.g[T2][0].wrapping_sub(c.g[T3][0]);
        if !a_negative {
            c.cycles += 1;
            c.g[V0][0] = c.g[T3][0].wrapping_sub(c.g[T2][0]);
        }
        c.cycles += 2;
        let negative = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = 0u64.wrapping_sub(c.g[V0][0]);
        c.cycles += 4;
        c.sw(at(c, A2, EXP), c.g[A3][0]);
        if negative {
            c.g[V0][0] = 0x1;
            c.sd(at(c, A2, FRAC), c.g[V1][0]);
            c.sw(at(c, A2, SIGN), c.g[V0][0]);
        } else {
            c.sd(at(c, A2, FRAC), c.g[V0][0]);
            c.sw(at(c, A2, SIGN), 0);
        }
        // Normalise: shift left until the implicit one reaches bit 60.
        c.cycles += 8;
        c.g[A1][0] = c.ld(at(c, A2, FRAC));
        c.g[V0][0] = NORMAL_LIMIT;
        c.g[V1][0] = c.g[A1][0].wrapping_sub(1);
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        c.g[A3][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            break 'normalised;
        }
        c.cycles += 3;
        c.g[A1][0] = NORMAL_LIMIT;
        loop {
            c.cycles += 9;
            c.g[V0][0] = c.lw(at(c, A2, EXP));
            c.g[A0][0] = c.g[A3][0] << 1;
            c.g[V1][0] = c.g[A0][0].wrapping_sub(1);
            c.sd(at(c, A2, FRAC), c.g[A0][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            c.g[V1][0] = (c.g[A1][0] < c.g[V1][0]) as u64;
            c.sw(at(c, A2, EXP), c.g[V0][0]);
            c.g[A3][0] = c.g[A0][0];
            if c.g[V1][0] != 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0041_1ea0);
            }
        }
        c.cycles += 2;
        c.g[A1][0] = c.g[A0][0];
    }

    // Class "number"; a carry past the implicit one shifts right once.
    c.cycles += 6;
    c.g[V1][0] = 0x3;
    c.g[V0][0] = ((OVERFLOW_LIMIT) < c.g[A1][0]) as u64;
    c.sw(at(c, A2, 0), c.g[V1][0]);
    if c.g[V0][0] != 0 {
        c.cycles += 7;
        c.g[V0][0] = c.lw(at(c, A2, EXP));
        c.g[A0][0] = c.g[A1][0] >> 1;
        c.g[V1][0] = c.g[A1][0] & 0x1;
        c.g[V1][0] |= c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        c.sd(at(c, A2, FRAC), c.g[V1][0]);
        c.sw(at(c, A2, EXP), c.g[V0][0]);
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    ret(c);
}

/// `FILE *_fopen_r(struct _reent *reent, const char *file, const char *mode)`
/// (0x416eb0): `fopen` backend. Resolves the mode with `__sflags`, takes a
/// free `FILE` from `__sfp`, opens it with `_open_r` (NULL back on failure,
/// after flagging the `FILE` closed), and seeks to the end for append modes.
pub fn fopen_r(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S2][0] = c.g[A1][0];
        c.g[A1][0] = c.g[A2][0];
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[RA][0] = sx(0x0041_6ed8);
        c.g[A2][0] = c.g[SP][0];
        callee::SFLAGS.call(c, 0x0041_6ed8)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = 0u64;
        if c.g[S1][0] == 0 {
            // Unknown mode: no `FILE`.
            c.g[V0][0] = 0u64;
        } else {
            c.cycles += 5;
            c.g[RA][0] = sx(0x0041_6eec);
            c.g[A0][0] = c.g[S0][0];
            callee::SFP.call(c, 0x0041_6eec)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            if c.g[S0][0] == 0 {
                // No free `FILE` slot.
                c.cycles += 2;
                c.g[V0][0] = 0u64;
            } else {
                c.cycles += 7;
                c.g[A2][0] = c.lw(c.g[SP][0] as u32);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                c.g[RA][0] = sx(0x0041_6f08);
                c.g[A3][0] = 0x1b6;
                callee::OPEN_R.call(c, 0x0041_6f08)?;
                c.g[A2][0] = c.g[V0][0];
                if (c.g[A2][0] as i64) >= 0 {
                    // Opened: fill in the `FILE` linkage words.
                    c.cycles += 16;
                    c.g[V0][0] = 0x410000;
                    c.g[V1][0] = 0x410000;
                    c.g[A0][0] = 0x410000;
                    c.g[A1][0] = 0x410000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7948));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x79b0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7a30));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7a98));
                    c.sh((c.g[S0][0] as u32).wrapping_add(0xe), c.g[A2][0]);
                    c.g[A2][0] = c.g[S1][0] & 0x100;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[A0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[A1][0]);
                    c.sh((c.g[S0][0] as u32).wrapping_add(0xc), c.g[S1][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[S0][0]);
                    if c.g[A2][0] == 0 {
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S0][0];
                    } else {
                        // Append mode: seek to the end.
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = 0x2;
                        callee::FSEEK.call(c, 0x0041_6f70)?;
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S0][0];
                    }
                } else {
                    // The open failed: mark the `FILE` closed.
                    c.cycles += 1;
                    c.sh((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                    c.cycles += 2;
                    c.g[V0][0] = 0u64;
                }
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = ra;
        Ok(())
    });
}

/// `int _fwalk(struct _reent *reent, int (*fn)(FILE *))` (0x417418):
/// call `fn` on every open `FILE` of the glue list, OR-ing the results.
/// Indirect call goes through the function table.
pub fn fwalk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S3][0] = 0u64;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S4][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        if c.g[S2][0] == 0 {
            // No glue list at all.
        } else {
            c.cycles += 1;
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
            'glue: loop {
                'files: {
                    c.cycles += 3;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    let done = (c.g[S0][0] as i64) < 0;
                    c.g[S1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    if done {
                        break 'files;
                    }
                    c.cycles += 1;
                    'one: loop {
                        c.cycles += 3;
                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xc));
                        if c.g[V0][0] == 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                        } else {
                            c.cycles += 4;
                            let target = c.g[S4][0] as u32;
                            c.g[RA][0] = sx(0x0041_746c);
                            c.g[A0][0] = c.g[S1][0];
                            call::call_ptr(c, target, 0x0041_746c)?;
                            c.g[S3][0] = c.g[S3][0] | c.g[V0][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                        }
                        c.cycles += 2;
                        let more = (c.g[S0][0] as i64) >= 0;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x58));
                        if !more {
                            break 'files;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0041_7458);
                        }
                        continue 'one;
                    }
                }
                c.cycles += 3;
                c.g[S2][0] = c.lw(c.g[S2][0] as u32);
                if c.g[S2][0] == 0u64 {
                    break;
                }
                c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                if event_due(c) {
                    deliver_event_here(c, 0x0041_7448);
                }
                continue 'glue;
            }
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S3][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = ra;
        Ok(())
    });
}

/// `int _printf_r(struct _reent *reent, const char *fmt, ...)`
/// (0x417578): reentrant `printf` core. Packs the register varargs into a
/// stack `va_list` and funnels them into `_vfprintf_r` through the reent's
/// `stdout`.
pub fn printf_r(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
        c.g[A2][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T0][0]);
        c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T3][0]);
        c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0x44));
        c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0x48));
        c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0x4c));
        c.g[RA][0] = sx(0x0041_75b8);
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        callee::VFPRINTF_R.call(c, 0x0041_75b8)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = ra;
        Ok(())
    });
}

/// `int _sprintf_r(struct _reent *reent, char *str, const char *fmt, ...)`
/// (0x4177b0): `sprintf` backend. A stack `__sFILE` with `__SWR|__SSTR`
/// (0x208) flags funnels into `vfprintf`; the terminator is stored by hand.
pub fn sprintf_r(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
        c.g[T4][0] = c.g[A1][0];
        c.g[V0][0] = 0x7fff0000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x54), c.g[A0][0]);
        c.g[A1][0] = c.g[A2][0];
        c.g[V0][0] |= 0xffff;
        c.g[V1][0] = 0x208;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
        c.sh((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[A0][0] = c.g[SP][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0xb8), c.g[A3][0]);
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb8));
        c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[T0][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0xc8), c.g[T1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[T2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0xd8), c.g[T3][0]);
        c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0xa8));
        c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0xac));
        c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0xb4));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[T4][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        c.sw(c.g[SP][0] as u32, c.g[T4][0]);
        c.g[RA][0] = sx(0x0041_7814);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        callee::VFPRINTF_DISPATCH.call(c, 0x0041_7814)?;
        c.g[V1][0] = c.lw(c.g[SP][0] as u32);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.sb(c.g[V1][0] as u32, 0u64);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
        c.pc = ra;
        Ok(())
    });
}

/// `unsigned long _strtoul_r(reent*, const char *s, char **end, int base)`
/// (0x41a840): the `strtoul` backend. Skips whitespace via the ctype
/// table, takes an optional sign and `0x` prefix, picks base 8 for a
/// leading `0` (10 otherwise) when `base` is 0, then accumulates digits
/// with overflow clamping through `__udivdi3`/`__umoddi3`/`__muldi3`.
/// Overflow keeps consuming digits so the end pointer lands past them.
pub fn strtoul_r(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S7][0] = 0x4a0000;
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x637f));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[FP][0] = c.g[A2][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S3][0] = c.g[A3][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
        // Skip whitespace (ctype class 8).
        loop {
            c.cycles += 7;
            c.g[S1][0] = c.lb(c.g[S2][0] as u32);
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
            c.g[V0][0] &= 0x8;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            if c.g[V0][0] == 0u64 {
                break;
            }
            if back_edge_unwinds(c, 0x0041_a890) {
                return Ok(());
            }
        }
        // Optional sign (0x41a8ac/0x41a8b8/0x41a8c8/0x41a8d0/0x41a8d4):
        // only a consumed sign advances `S2`.
        c.cycles += 3;
        c.g[V0][0] = 0x2d; // '-'
        if c.g[S1][0] == c.g[V0][0] {
            c.cycles += 4;
            c.g[S1][0] = c.lb(c.g[S2][0] as u32);
            c.g[V0][0] = 0x1;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.cycles += 1;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0x2b; // '+'
            if c.g[S1][0] == c.g[V0][0] {
                c.cycles += 1;
                c.g[S1][0] = c.lb(c.g[S2][0] as u32);
                c.cycles += 1;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            }
        }
        // Base selection (0x41a8d8/0x41a8e0).
        c.cycles += 2;
        c.g[V0][0] = 0x10;
        if c.g[S3][0] != 0 {
            c.cycles += 2;
        }
        if c.g[S3][0] == 0 || c.g[S3][0] == c.g[V0][0] {
            // `0x`/`0X` prefix forces base 16 (0x41a8e8). The `V0 = 0x78`
            // below is a delay slot: gen sets it even when `S1` is not `0`.
            c.cycles += 3;
            c.g[V0][0] = 0x30; // '0'
            c.g[V0][0] = 0x78; // 'x'
            if c.g[S1][0] == 0x30 {
                c.cycles += 3;
                c.g[V1][0] = c.lb(c.g[S2][0] as u32);
                c.g[V0][0] = 0x58; // 'X'
                if c.g[V1][0] == 0x78 {
                    // 'x' (0x41a8f4 -> 0x41a908).
                    c.cycles += 3;
                    c.g[S1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[S3][0] = 0x10;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2));
                } else {
                    // (0x41a900).
                    c.cycles += 2;
                    if c.g[V1][0] == c.g[V0][0] {
                        // 'X' (0x41a908).
                        c.cycles += 3;
                        c.g[S1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x1));
                        c.g[S3][0] = 0x10;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2));
                    }
                }
            }
        }
        // Default base: 8 for a leading `0`, else 10 (0x41a914/0x41a91c).
        c.cycles += 2;
        c.g[S0][0] = c.g[S3][0];
        if c.g[S3][0] == 0 {
            c.cycles += 5;
            c.g[S3][0] = 0xa;
            c.g[V1][0] = 0x8;
            c.g[V0][0] = c.g[S1][0] ^ 0x30;
            if c.g[V0][0] == 0 {
                c.g[S3][0] = c.g[V1][0];
            }
            c.g[S0][0] = c.g[S3][0];
        }
        // Cutoff/cutlim for the overflow check: `ULONG_MAX / base`,
        // `ULONG_MAX % base`.
        c.cycles += 12;
        c.g[A0][0] = 0xffffffffffffffff;
        c.g[A1][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0041_a940);
        c.g[S4][0] = 0u64;
        callee::UDIVDI3.call(c, 0x0041_a940)?;
        c.g[A0][0] = 0xffffffffffffffff;
        c.g[S5][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0041_a950);
        c.g[A1][0] = c.g[S0][0];
        callee::UMODDI3.call(c, 0x0041_a950)?;
        c.g[S6][0] = c.g[V0][0] << 32;
        c.g[S6][0] = ((c.g[S6][0] as i64) >> 32) as u64;
        c.g[A0][0] = 0u64;
        // Digit bytes: entry from cutoff falls into classify (0x41a9ac);
        // 0x41a960 is the loop-back head. Overflow flags but keeps
        // consuming digits so the end pointer lands past them.
        loop {
            // Classify the current byte (0x41a9ac).
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x637f));
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = c.lbu(c.g[V0][0] as u32);
            c.g[V1][0] = c.g[A1][0] & 0x4;
            c.g[V0][0] = c.g[A1][0] & 0x3;
            if c.g[V1][0] != 0 {
                // Decimal digit (0x41a9c4).
                c.cycles += 2;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x30));
            } else {
                // Letter (or terminator): cutoff check first (0x41a9cc).
                c.cycles += 2;
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x57));
                if c.g[V0][0] == 0 {
                    break;
                }
                // Hex-adjust (0x41a9d4).
                c.cycles += 4;
                c.g[V0][0] = c.g[A1][0] & 0x1;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x37));
                if c.g[V0][0] == 0 {
                    c.g[S1][0] = c.g[V1][0];
                }
            }
            if back_edge_unwinds(c, 0x0041_a960) {
                return Ok(());
            }
            // Range check (0x41a960).
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            // Overflow check against cutoff/cutlim (0x41a96c).
            c.cycles += 2;
            c.g[V0][0] = (c.g[S5][0] < c.g[A0][0]) as u64;
            if (c.g[S4][0] as i64) < 0 {
                // Already overflowed: keep the flag, skip the multiply.
                c.cycles += 2;
                c.g[S4][0] = 0xffffffffffffffff;
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    // Above cutoff: flag overflow, skip the multiply.
                    c.g[S4][0] = 0xffffffffffffffff;
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    if c.g[A0][0] == c.g[S5][0] {
                        // At cutoff: the next digit must fit under cutlim.
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[S1][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.cycles += 2;
                            c.g[S4][0] = 0xffffffffffffffff;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x0041_a9a0);
                            c.g[S4][0] = 0x1;
                            callee::MULDI3.call(c, 0x0041_a9a0)?;
                            c.g[A0][0] = c.g[S1][0].wrapping_add(c.g[V0][0]);
                        }
                    } else {
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x0041_a9a0);
                        c.g[S4][0] = 0x1;
                        callee::MULDI3.call(c, 0x0041_a9a0)?;
                        c.g[A0][0] = c.g[S1][0].wrapping_add(c.g[V0][0]);
                    }
                }
            }
            // Load the next byte (0x41a9a4), back to classify.
            c.cycles += 2;
            c.g[S1][0] = c.lb(c.g[S2][0] as u32);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
        }
        // Done: `errno` marker on overflow, negation, end pointer out.
        c.cycles += 2;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
        if (c.g[S4][0] as i64) >= 0 {
            c.cycles += 2;
            c.g[V0][0] = 0u64.wrapping_sub(c.g[A0][0]);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = c.g[V0][0];
            }
        } else {
            c.cycles += 5;
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            c.g[V0][0] = 0x22;
            c.g[A0][0] = 0xffffffffffffffff;
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        }
        c.cycles += 2;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        if c.g[FP][0] == 0 {
            // No end pointer wanted.
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
            if c.g[S4][0] == 0 {
                c.g[V0][0] = c.g[V1][0];
            }
            c.sw(c.g[FP][0] as u32, c.g[V0][0]);
        }
        c.cycles += 13;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[V0][0] = c.g[A0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = ra;
        Ok(())
    });
}

/// `int vfprintf_RoundDigits(char*, int)` (0x41ae00): round the digit
/// string for `%f`-style output; returns 1, or 0 when a carry ripples
/// past the leading digit.
pub fn vfprintf_round_digits(c: &mut Cpu) {
    c.cycles += 2;
    let neg = (c.g[A1][0] as i64) <= 0;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
    if neg {
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = c.lb(c.g[V1][0] as u32);
    c.g[A3][0] = 0x30;
    if (c.g[V0][0] as i64) < 53 {
        // Last kept digit is below '5': no rounding needed.
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[A2][0] = 0x39;
    c.sb(c.g[V1][0] as u32, c.g[A3][0]);
    // Propagate the carry leftward over any trailing '9's.
    loop {
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        if (c.g[A1][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = c.lb(c.g[V1][0] as u32);
            if c.g[V0][0] == c.g[A2][0] {
                c.sb(c.g[V1][0] as u32, c.g[A3][0]);
                if event_due(c) {
                    deliver_event_here(c, 0x0041_ae28);
                }
                continue;
            }
        }
        break;
    }
    c.cycles += 5;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = 0x39;
    c.g[V0][0] = c.lb(c.g[A0][0] as u32);
    c.g[A1][0] = c.lbu(c.g[A0][0] as u32);
    if c.g[V0][0] != c.g[V1][0] {
        c.cycles += 2;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
        c.sb(c.g[A0][0] as u32, c.g[V0][0]);
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    abi::ret(c);
}

/// `char *vfprintf_StrReverse(char*)` (0x41ae70): reverse a NUL-terminated
/// string in place; returns the original pointer.
pub fn vfprintf_str_reverse(c: &mut Cpu) {
    c.cycles += 4;
    c.g[T1][0] = c.g[A0][0];
    c.g[V0][0] = c.lb(c.g[T1][0] as u32);
    c.g[A3][0] = 0u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[V1][0] = c.lb(c.g[V0][0] as u32);
            if c.g[V1][0] == 0 {
                break;
            }
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            if event_due(c) {
                deliver_event_here(c, 0x0041_ae88);
            }
        }
    }
    // `A3` is now `strlen + 1` (1 when the string was empty).
    c.cycles += 3;
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
    c.g[T0][0] = 0u64;
    if (c.g[A3][0] as i64) > 0 {
        c.cycles += 2;
        c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T1][0] as u32));
        c.g[A1][0] = c.g[T1][0];
        loop {
            c.cycles += 10;
            c.g[V0][0] = c.lbu(c.g[A2][0] as u32);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
            c.g[V1][0] = c.lb(c.g[A1][0] as u32);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
            c.sb(c.g[A1][0] as u32, c.g[V0][0]);
            c.g[A0][0] = ((c.g[T0][0] as i64) < (c.g[A3][0] as i64)) as u64;
            c.sb(c.g[A2][0] as u32, c.g[V1][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
            if c.g[A0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0041_aeb8);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T1][0];
    abi::ret(c);
}

/// `char *vfprintf_IntToStr(char*, unsigned, int base)` (0x41aee8):
/// format an unsigned value into `buf`, reversing via StrReverse.
/// A negative input (sign bit set) gains a `-` prefix; its magnitude is
/// negated first when `base == 10`.
pub fn vfprintf_int_to_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[T1][0] = c.g[A1][0];
        c.g[T2][0] = sx((c.g[A0][0] as u32) >> 31);
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        if c.g[T2][0] != 0 {
            c.cycles += 3;
            c.g[V1][0] = sx(0u32.wrapping_sub(c.g[A0][0] as u32));
            c.g[V0][0] = c.g[A2][0] ^ 0xa;
            if c.g[V0][0] == 0 {
                c.g[A0][0] = c.g[V1][0];
            }
        }
        c.cycles += 6;
        c.g[V0][0] = 0x450000;
        c.g[A3][0] = 0u64;
        c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4cb0));
        c.g[T0][0] = 0u64;
        c.divu(0, 4, 6);
        loop {
            // (Generated: two vacuous branch blocks here; net effect
            // is just the cycle counts below.)
            c.cycles += 2;
            c.cycles += 9;
            c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            c.g[V1][0] = c.hi[0];
            c.g[A0][0] = c.lo[0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T3][0] as u32));
            c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
            c.sb(c.g[A1][0] as u32, c.g[V0][0]);
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                break;
            }
            c.divu(0, 4, 6);
            if event_due(c) {
                deliver_event_here(c, 0x0041_af20);
            }
        }
        c.cycles += 2;
        c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A3][0] as u32));
        if c.g[T2][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = 0x2d;
            c.sb(c.g[V1][0] as u32, c.g[V0][0]);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
        }
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[A0][0] = c.g[T1][0];
        c.g[RA][0] = sx(0x0041_af78);
        c.sb(c.g[V0][0] as u32, 0u64);
        callee::STR_REVERSE.call(c, 0x0041_af78)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `int vfprintf_FlushFile(FILE *fp)` (0x41b528): flush `fp` through
/// `BufferedWrite` with its `_file` descriptor.
pub fn vfprintf_flush_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[A1][0] = 0u64;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0x2;
        c.g[RA][0] = sx(0x0041_b544);
        c.g[A0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0xe));
        callee::VFPRINTF_BUFFERED_WRITE.call(c, 0x0041_b544)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `int vfprintf_BufferedWrite(int fd, const char *buf, size_t len, int mode)`
/// (0x41b550): append `len` bytes to the static buffer, flushing through
/// `_write` when it fills; returns `len`, or 0 if the flush wrote
/// nothing. Mode 1 writes through `_write` directly and returns its result.
pub fn vfprintf_buffered_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.g[V0][0] = 0x1;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S3][0] = c.g[A2][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S4][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S6][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        if c.g[A3][0] == c.g[V0][0] {
            // Direct `_write`, bypassing the buffer.
            c.cycles += 10;
            c.g[S0][0] = 0x450000;
            c.g[S1][0] = sx(0x530000u32.wrapping_sub(0x5168));
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4cd8));
            c.g[RA][0] = sx(0x0041_b5a0);
            c.g[A1][0] = c.g[S1][0];
            callee::WRITE.call(c, 0x0041_b5a0)?;
            c.g[V1][0] = 0x450000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4cd8), 0u64);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4cdc), c.g[S1][0]);
        } else {
            c.cycles += 2;
            c.g[S0][0] = 0u64;
            if c.g[S3][0] != 0 {
                c.cycles += 6;
                c.g[V0][0] = 0x530000;
                c.g[S1][0] = 0x450000;
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5168));
                c.g[S2][0] = 0x450000;
                c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S0][0] as u32));
                let mut flushed_zero = false;
                loop {
                    c.cycles += 11;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4cdc));
                    c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                    c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4cd8));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4cdc));
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4cd8), c.g[A2][0]);
                    // Gen keeps the `slti` result in `V0` (dead: the
                    // flush below overwrites it, the loop exit ignores it).
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 128) as u64;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4cdc), c.g[V1][0]);
                    if c.g[V0][0] == 0 {
                        // Buffer full: flush through `_write`.
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S4][0];
                        c.g[RA][0] = sx(0x0041_b610);
                        c.g[A1][0] = c.g[S5][0];
                        callee::WRITE.call(c, 0x0041_b610)?;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x4cdc), c.g[S5][0]);
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x4cd8), 0u64);
                        if c.g[V0][0] == 0 {
                            // Nothing written: return 0.
                            if back_edge_unwinds(c, 0x0041_b5b0) {
                                return Ok(());
                            }
                            c.cycles += 2;
                            c.g[V0][0] = 0u64;
                            flushed_zero = true;
                            break;
                        }
                    }
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = (c.g[S0][0] < c.g[S3][0]) as u64;
                    c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    if back_edge_unwinds(c, 0x0041_b5d8) {
                        return Ok(());
                    }
                }
                if !flushed_zero {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S0][0];
                }
            } else {
                // Empty write: return 0.
                c.cycles += 1;
                c.g[V0][0] = c.g[S0][0];
            }
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = ra;
        Ok(())
    });
}

/// `char *vfprintf_InsertThousandsSep(char *dst, const char *start,
/// const char *end, int *added)` (0x41c788): copy a decimal float string
/// inserting `,` separators; returns `dst`, or `start` for exponents.
pub fn vfprintf_insert_thousands_sep(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[A1][0] as u32));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x19));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = sx((c.g[V1][0] as u32) >> 4);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[FP][0] = c.g[SP][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xa));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.g[S2][0] = c.g[A3][0];
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[S1][0] = c.g[A0][0];
        c.g[T0][0] = 0u64;
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        if c.g[S1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0u64;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0u64;
            if c.g[A1][0] != 0 {
                c.cycles += 2;
                c.g[A0][0] = c.g[A1][0];
                if c.g[A2][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = 0u64;
                } else {
                    // Main path: scan the digits (single length check up
                    // front; the inner loop re-scans via `continue`).
                    c.cycles += 4;
                    c.sw(c.g[FP][0] as u32, 0u64);
                    c.g[V0][0] = (c.g[A2][0] < c.g[A0][0]) as u64;
                    c.sw((c.g[FP][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                    if c.g[V0][0] == 0 {
                        c.cycles += 7;
                        c.g[T5][0] = 0x45;
                        c.g[T4][0] = 0x2e;
                        c.g[T3][0] = 0x65;
                        c.g[T2][0] = 0x1;
                        c.g[T1][0] = 0x2;
                        c.g[V1][0] = c.lb(c.g[A0][0] as u32);
                        'scan: loop {
                            // Classify one byte. Outcome 0 = advanced past a
                            // digit/other straight to the end check; 1 = 'e'
                            // (recorded, needs the shared advance); 2 =
                            // 'E'/'.' (needs the split-record tail, then
                            // the shared advance).
                            let path = 'classify: {
                                c.cycles += 2;
                                c.g[V0][0] = ((c.g[V1][0] as i64) < 70i64) as u64;
                                if c.g[V1][0] == c.g[T5][0] {
                                    // 'E': exponent marker. Mark it, then
                                    // advance below (no split record).
                                    c.cycles += 1;
                                    c.sw(c.g[FP][0] as u32, c.g[T1][0]);
                                    break 'classify 1;
                                }
                                c.cycles += 2;
                                if c.g[V0][0] != 0 {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.lw(c.g[FP][0] as u32);
                                    if c.g[V1][0] != c.g[T4][0] {
                                        // Digit: advance, to the end check.
                                        c.cycles += 2;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                        break 'classify 0;
                                    }
                                    // '.': record at the tail below.
                                    break 'classify 2;
                                }
                                // Not a digit or dot: maybe 'e'.
                                c.cycles += 2;
                                if c.g[V1][0] == c.g[T3][0] {
                                    c.sw(c.g[FP][0] as u32, c.g[T1][0]);
                                    break 'classify 1;
                                }
                                c.cycles += 2;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                break 'classify 0;
                            };
                            if path == 2 {
                                // Shared split-record tail.
                                c.cycles += 2;
                                let split = c.g[V0][0] != 0u64;
                                c.sw((c.g[FP][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                                if !split {
                                    c.cycles += 2;
                                    c.sw(c.g[FP][0] as u32, c.g[T2][0]);
                                }
                            }
                            if path != 0 {
                                // Shared advance.
                                c.cycles += 1;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            }
                            // End check: past the end, or exponent seen.
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[A2][0] < c.g[A0][0]) as u64;
                            let past_end = c.g[V0][0] != 0u64;
                            c.g[V0][0] = c.lw(c.g[FP][0] as u32);
                            if past_end {
                                break;
                            }
                            c.cycles += 2;
                            if c.g[V0][0] != c.g[T1][0] {
                                c.g[V1][0] = c.lb(c.g[A0][0] as u32);
                                if event_due(c) {
                                    deliver_event_here(c, 0x0041_c818);
                                }
                                continue 'scan;
                            } else {
                                break;
                            }
                        }
                    }
                    // (b_88c) Length check.
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(c.g[FP][0] as u32);
                    c.g[V0][0] = 0x2;
                    c.g[A0][0] = c.g[A2][0];
                    if c.g[V1][0] == c.g[V0][0] {
                        // Exponent: return the start.
                        c.cycles += 2;
                        c.g[V0][0] = c.g[A1][0];
                    } else {
                        // (b_8f4) Insert separators right to left.
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[A1][0] < c.g[A0][0]) as u64;
                        c.g[T1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
                        if c.g[V0][0] != 0 {
                            c.cycles += 4;
                            c.g[A2][0] = 0x3;
                            c.g[T3][0] = 0u64;
                            c.g[T2][0] = 0x2c;
                            c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                            loop {
                                c.cycles += 4;
                                c.g[V1][0] = (c.g[T1][0] < c.g[A0][0]) as u64;
                                c.sb(c.g[A3][0] as u32, c.g[V0][0]);
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                                if c.g[V1][0] == 0 {
                                    // Group boundary accounting.
                                    c.cycles += 3;
                                    c.div(0, 8, 6);
                                    c.cycles += 3;
                                    c.g[V0][0] = c.hi[0];
                                    if c.g[V0][0] == 0 {
                                        c.cycles += 2;
                                        let t = c.g[T0][0] == 0u64;
                                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                        if !t {
                                            c.cycles += 3;
                                            c.sb(c.g[A3][0] as u32, c.g[T2][0]);
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                                        }
                                    } else {
                                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                    }
                                }
                                c.cycles += 4;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                                c.g[V0][0] = (c.g[A1][0] < c.g[A0][0]) as u64;
                                if c.g[V0][0] == 0 {
                                    break;
                                }
                                c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                                if event_due(c) {
                                    deliver_event_here(c, 0x0041_c8a8);
                                }
                            }
                        }
                        // Copy the reordered digits over the input.
                        c.cycles += 9;
                        c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xa));
                        c.g[A1][0] = c.g[A3][0];
                        c.g[A0][0] = c.g[S1][0];
                        c.sb(c.g[A3][0] as u32, c.g[V0][0]);
                        c.g[RA][0] = sx(0x0041_c910);
                        c.g[A2][0] = c.g[S0][0];
                        callee::MEMCPY.call(c, 0x0041_c910)?;
                        c.sw(c.g[S2][0] as u32, c.g[S0][0]);
                        c.g[V0][0] = c.g[S1][0];
                    }
                }
            }
        }
        // Epilogue (0x41c918).
        c.cycles += 8;
        c.g[SP][0] = c.g[FP][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = ra;
        Ok(())
    });
}

/// `void _free_r(reent *r, void *ptr)` (0x41e340): return a chunk to
/// the heap, consolidating with free neighbors and linking it into a
/// bin (or trimming the top). `free(NULL)` is a no-op.
pub fn free_r(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        // The fast path restores s16 itself and skips the shared `+1 /
        // lq16` below, like the generated body's direct break.
        let mut s16_done = false;
        if c.g[S0][0] != 0 {
            // Lock, then check the chunk against the arena top.
            c.cycles += 17;
            c.g[RA][0] = sx(0x0041_e364);
            callee::MALLOC_LOCK.call(c, 0x0041_e364)?;
            c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x8));
            c.g[V0][0] = 0xffffffffffff0000u64;
            c.g[A2][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x4));
            c.g[V0][0] |= 0xfffeu64;
            c.g[A1][0] = 0x450000;
            c.g[V1][0] = 0xffffffffffff0000u64;
            c.g[T1][0] = c.g[A2][0] & c.g[V0][0];
            c.g[T3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4ce8));
            c.g[A3][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[T1][0] as u32));
            c.g[V1][0] |= 0xfffcu64;
            c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
            c.g[T5][0] = c.g[A1][0];
            c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x8));
            c.g[A0][0] &= c.g[V1][0];
            if c.g[A3][0] == c.g[V0][0] {
                // Fast path: unlink a free predecessor, link in.
                c.cycles += 3;
                c.g[V0][0] = c.g[A2][0] & 0x1;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                if c.g[V0][0] == 0 {
                    c.cycles += 7;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x8));
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[T0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0xc));
                    c.g[A2][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[A2][0] as u32).wrapping_add(0xc), c.g[T0][0]);
                    c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                }
                c.cycles += 9;
                c.g[A0][0] = 0x450000;
                c.g[V1][0] = c.g[T1][0] << 32;
                c.g[A1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x50f0));
                c.g[V0][0] = c.g[T1][0] | 0x1;
                c.sw((c.g[T2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] >>= 32;
                c.g[V1][0] = (c.g[V1][0] < c.g[A1][0]) as u64;
                c.sw((c.g[T3][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                if c.g[V1][0] == 0 {
                    c.cycles += 4;
                    c.g[V0][0] = 0x450000;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0041_e3fc);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x50f8));
                    callee::MALLOC_TRIM.call(c, 0x0041_e3fc)?;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x0041_e404);
                c.g[A0][0] = c.g[S1][0];
                callee::MALLOC_UNLOCK.call(c, 0x0041_e404)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                s16_done = true;
            } else {
                // Slow path: consolidate with free neighbors first.
                c.cycles += 4;
                c.g[V0][0] = c.g[A2][0] & 0x1;
                c.sw((c.g[A3][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                c.g[T4][0] = 0u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x8));
                    c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_add(0x8));
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                    c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    if c.g[V1][0] == c.g[V0][0] {
                        c.cycles += 2;
                        c.g[T4][0] = 0x1;
                    } else {
                        c.cycles += 4;
                        c.g[T0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0xc));
                        c.g[A2][0] = c.g[V1][0];
                        c.sw((c.g[A2][0] as u32).wrapping_add(0xc), c.g[T0][0]);
                        c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    }
                }
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] &= 0x1;
                let next_in_use = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.g[T1][0] | 0x1;
                if !next_in_use {
                    // Next chunk free: unlink it, absorb it.
                    c.cycles += 3;
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                    let mut spliced = false;
                    if c.g[T4][0] != 0 {
                        c.cycles += 1;
                        c.g[T0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = 0x450000;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4cf0));
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[T0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                        } else {
                            c.cycles += 6;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T2][0]);
                            c.g[T4][0] = 0x1;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                            c.sw((c.g[T2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                            c.sw((c.g[T2][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                            spliced = true;
                        }
                    }
                    if !spliced {
                        c.cycles += 3;
                        c.g[A2][0] = c.g[V1][0];
                        c.sw((c.g[A2][0] as u32).wrapping_add(0xc), c.g[T0][0]);
                        c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[T1][0] | 0x1;
                }
                // Link the (possibly grown) chunk into its bin.
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[T1][0] as u32));
                c.sw((c.g[T2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw(c.g[V1][0] as u32, c.g[T1][0]);
                // The bin-index/tree/insert region: the small-bin and
                // empty-bin arms break straight out to the +4 stores.
                'bin: {
                    if c.g[T4][0] == 0 {
                        // Compute the bin index for the size.
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[T1][0] < 0x200u64) as u64;
                        c.g[A3][0] = sx((c.g[T1][0] as u32) >> 3);
                        if c.g[V0][0] == 0 {
                            // Large bins: tree walk by size class.
                            c.cycles += 3;
                            c.g[V1][0] = sx((c.g[T1][0] as u32) >> 9);
                            c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
                            if c.g[V1][0] == 0 {
                                c.cycles += 2;
                                c.g[A3][0] = sx((c.g[T1][0] as u32) >> 3);
                            } else {
                                c.cycles += 2;
                                let v1_ge_5 = c.g[V0][0] == 0;
                                c.g[V0][0] = sx((c.g[T1][0] as u32) >> 6);
                                if v1_ge_5 {
                                    c.cycles += 3;
                                    c.g[V0][0] = (c.g[V1][0] < 0x15u64) as u64;
                                    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5b));
                                    if c.g[V0][0] == 0 {
                                        c.cycles += 3;
                                        c.g[V0][0] = (c.g[V1][0] < 0x55u64) as u64;
                                        let v1_ge_55 = c.g[V0][0] == 0;
                                        c.g[V0][0] = sx((c.g[T1][0] as u32) >> 12);
                                        if !v1_ge_55 {
                                            c.cycles += 2;
                                            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6e));
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = (c.g[V1][0] < 0x155u64) as u64;
                                            let v1_ge_155 = c.g[V0][0] == 0;
                                            c.g[V0][0] = sx((c.g[T1][0] as u32) >> 15);
                                            if !v1_ge_155 {
                                                c.cycles += 2;
                                                c.g[A3][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x77));
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] = (c.g[V1][0] < 0x555u64) as u64;
                                                let v1_ge_555 = c.g[V0][0] == 0;
                                                c.g[V0][0] = sx((c.g[T1][0] as u32) >> 18);
                                                if !v1_ge_555 {
                                                    c.cycles += 2;
                                                    c.g[A3][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x7c));
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[A3][0] = 0x7e;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    c.cycles += 2;
                                    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x38));
                                }
                            }
                        } else {
                            // Small bins: set the bit, link at the head.
                            c.cycles += 17;
                            c.g[V0][0] = 0xffffffffffffffff;
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
                            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3));
                            if c.g[V0][0] != 0 {
                                c.g[V1][0] = c.g[A3][0];
                            }
                            c.g[A1][0] = sx((c.g[T5][0] as u32).wrapping_add(0x4ce8));
                            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 2) as u32);
                            c.g[V0][0] = 0x1;
                            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] <<= c.g[V1][0] & 63;
                            c.g[V0][0] <<= 32;
                            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                            c.g[V1][0] = sx((c.g[A3][0] as u32) << 3);
                            c.g[A0][0] |= c.g[V0][0];
                            c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                            c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                            break 'bin;
                        }
                        // Insert into the bin's circular list.
                        c.cycles += 6;
                        c.g[T3][0] = sx((c.g[T5][0] as u32).wrapping_add(0x4ce8));
                        c.g[V0][0] = sx((c.g[A3][0] as u32) << 3);
                        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                        c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                        if c.g[A2][0] == c.g[T0][0] {
                            c.cycles += 13;
                            c.g[V0][0] = 0xffffffffffffffff;
                            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
                            c.g[A0][0] = 0x1;
                            if c.g[V0][0] != 0 {
                                c.g[V1][0] = c.g[A3][0];
                            }
                            c.g[A1][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 2) as u32);
                            c.g[A0][0] <<= c.g[V1][0] & 63;
                            c.g[A0][0] <<= 32;
                            c.g[A0][0] = ((c.g[A0][0] as i64) >> 32) as u64;
                            c.g[A1][0] |= c.g[A0][0];
                            c.sw((c.g[T3][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                            break 'bin;
                        } else {
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                        }
                        c.cycles += 6;
                        c.g[V0][0] = 0xffffffffffff0000u64;
                        c.g[V0][0] |= 0xfffcu64;
                        c.g[V1][0] &= c.g[V0][0];
                        c.g[V1][0] = (c.g[T1][0] < c.g[V1][0]) as u64;
                        if c.g[V1][0] != 0 {
                            c.cycles += 3;
                            c.g[V1][0] = 0xffffffffffff0000u64;
                            c.g[V1][0] |= 0xfffcu64;
                            c.g[A2][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            loop {
                                c.cycles += 2;
                                if c.g[A2][0] == c.g[T0][0] {
                                    c.g[T0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                                    break;
                                }
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] &= c.g[V1][0];
                                c.g[V0][0] = (c.g[T1][0] < c.g[V0][0]) as u64;
                                if c.g[V0][0] != 0 {
                                    c.g[A2][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0041_e5e8);
                                    }
                                    continue;
                                }
                                c.cycles += 1;
                                c.g[T0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                                break;
                            }
                        } else {
                            c.g[T0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                        }
                    } // close if T4==0
                } // close 'bin: small/empty-bin break out; large falls through
                if c.g[T4][0] == 0 {
                    // A last-remainder merge skips the link stores with the
                    // rest of the bin region (gen `break 'b_41e618`).
                    c.cycles += 4;
                    c.sw((c.g[T2][0] as u32).wrapping_add(0xc), c.g[T0][0]);
                    c.sw((c.g[T2][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                    c.sw((c.g[A2][0] as u32).wrapping_add(0xc), c.g[T2][0]);
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x0041_e620);
                c.g[A0][0] = c.g[S1][0];
                callee::MALLOC_UNLOCK.call(c, 0x0041_e620)?;
            }
        }
        if !s16_done {
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `long _strtol_r(reent*, const char *s, char **end, int base)`
/// (0x41e7a8): the `strtol` backend. Like `_strtoul_r` but with a sign
/// flag (`S7`) and `LONG_MIN`/`LONG_MAX` clamping.
pub fn strtol_r(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[FP][0] = 0x4a0000;
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x637f));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S3][0] = c.g[A3][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S7][0] = 0u64;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A2][0]);
        c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        // Skip whitespace (ctype class 8).
        loop {
            c.cycles += 7;
            c.g[S1][0] = c.lb(c.g[S2][0] as u32);
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
            c.g[V0][0] &= 0x8;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            if c.g[V0][0] == 0u64 {
                break;
            }
            if back_edge_unwinds(c, 0x0041_e7f8) {
                return Ok(());
            }
        }
        // Optional sign; only a consumed sign advances `S2`.
        c.cycles += 3;
        c.g[V0][0] = 0x2d;
        if c.g[S1][0] == c.g[V0][0] {
            c.cycles += 3;
            c.g[S1][0] = c.lb(c.g[S2][0] as u32);
            c.g[S7][0] = 0x1;
            c.cycles += 1;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0x2b;
            if c.g[S1][0] == c.g[V0][0] {
                c.cycles += 1;
                c.g[S1][0] = c.lb(c.g[S2][0] as u32);
                c.cycles += 1;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            }
        }
        // Base selection.
        c.cycles += 2;
        c.g[V0][0] = 0x10;
        if c.g[S3][0] != 0 {
            c.cycles += 2;
        }
        if c.g[S3][0] == 0 || c.g[S3][0] == c.g[V0][0] {
            // `0x`/`0X` prefix forces base 16.
            c.cycles += 3;
            c.g[V0][0] = 0x30;
            // Delay slot of the check below: gen sets it even when the
            // first digit is not `0` (it feeds the modulo call).
            c.g[V0][0] = 0x78;
            if c.g[S1][0] == 0x30 {
                c.cycles += 3;
                c.g[V1][0] = c.lb(c.g[S2][0] as u32);
                c.g[V0][0] = 0x58;
                if c.g[V1][0] == 0x78 {
                    c.cycles += 3;
                    c.g[S1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[S3][0] = 0x10;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2));
                } else {
                    c.cycles += 2;
                    if c.g[V1][0] == c.g[V0][0] {
                        c.cycles += 3;
                        c.g[S1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x1));
                        c.g[S3][0] = 0x10;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2));
                    }
                }
            }
        }
        // Default base: 8 for a leading `0`, else 10.
        c.cycles += 2;
        c.g[V1][0] = 0x8;
        if c.g[S3][0] == 0 {
            c.cycles += 3;
            c.g[S3][0] = 0xa;
            c.g[V0][0] = c.g[S1][0] ^ 0x30;
            if c.g[V0][0] == 0 {
                c.g[S3][0] = c.g[V1][0];
            }
        }
        // Cutoff/cutlim: `LONG_MAX`/`LONG_MIN` over the base.
        c.cycles += 18;
        c.g[V0][0] = 0xffffffffffffffffu64;
        c.g[V0][0] <<= 63;
        c.g[S4][0] = 0xffffffffffffffffu64;
        c.g[S4][0] >>= 1;
        if c.g[S7][0] != 0 {
            c.g[S4][0] = c.g[V0][0];
        }
        c.g[S0][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S4][0];
        c.g[RA][0] = sx(0x0041_e8b0);
        c.g[A1][0] = c.g[S0][0];
        callee::UMODDI3.call(c, 0x0041_e8b0)?;
        c.g[S5][0] = 0u64;
        c.g[A0][0] = c.g[S4][0];
        c.g[S6][0] = c.g[V0][0] << 32;
        c.g[S6][0] = ((c.g[S6][0] as i64) >> 32) as u64;
        c.g[RA][0] = sx(0x0041_e8c8);
        c.g[A1][0] = c.g[S0][0];
        callee::UDIVDI3.call(c, 0x0041_e8c8)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[A0][0] = 0u64;
        // Digit bytes: entry at classify; the range check is the back edge.
        // (The generator's `0x41e8d4` arm is dead: always jumps to classify.)
        loop {
            // Classify the current byte.
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x637f));
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = c.lbu(c.g[V0][0] as u32);
            c.g[V1][0] = c.g[A1][0] & 0x4;
            c.g[V0][0] = c.g[A1][0] & 0x3;
            if c.g[V1][0] != 0 {
                // Decimal digit.
                c.cycles += 2;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x30));
            } else {
                // Letter (or terminator): cutoff check first.
                c.cycles += 2;
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x57));
                if c.g[V0][0] == 0 {
                    break;
                }
                // Hex-adjust.
                c.cycles += 4;
                c.g[V0][0] = c.g[A1][0] & 0x1;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x37));
                if c.g[V0][0] == 0 {
                    c.g[S1][0] = c.g[V1][0];
                }
            }
            if back_edge_unwinds(c, 0x0041_e8d8) {
                return Ok(());
            }
            // Range check.
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            // Overflow check against cutoff/cutlim.
            c.cycles += 2;
            c.g[V0][0] = (c.g[S4][0] < c.g[A0][0]) as u64;
            if (c.g[S5][0] as i64) < 0 {
                c.cycles += 2;
                c.g[S5][0] = 0xffffffffffffffff;
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[S5][0] = 0xffffffffffffffff;
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    if c.g[A0][0] == c.g[S4][0] {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[S1][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x0041_e918);
                            c.g[S5][0] = 0x1;
                            callee::MULDI3.call(c, 0x0041_e918)?;
                            c.g[A0][0] = c.g[S1][0].wrapping_add(c.g[V0][0]);
                        } else {
                            c.cycles += 2;
                            c.g[S5][0] = 0xffffffffffffffff;
                        }
                    } else {
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x0041_e918);
                        c.g[S5][0] = 0x1;
                        callee::MULDI3.call(c, 0x0041_e918)?;
                        c.g[A0][0] = c.g[S1][0].wrapping_add(c.g[V0][0]);
                    }
                }
            }
            // Load the next byte, back to classify.
            c.cycles += 2;
            c.g[S1][0] = c.lb(c.g[S2][0] as u32);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
        }
        // Done: clamp on overflow, negate, end pointer out.
        c.cycles += 2;
        c.g[V0][0] = 0u64.wrapping_sub(c.g[A0][0]);
        if (c.g[S5][0] as i64) >= 0 {
            c.cycles += 1;
            if c.g[S7][0] != 0 {
                c.g[A0][0] = c.g[V0][0];
            }
        } else {
            c.cycles += 9;
            c.g[V0][0] = 0xffffffffffffffffu64;
            c.g[V0][0] <<= 63;
            c.g[A0][0] = 0xffffffffffffffffu64;
            c.g[A0][0] >>= 1;
            if c.g[S7][0] != 0 {
                c.g[A0][0] = c.g[V0][0];
            }
            c.g[V1][0] = 0x22;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
        }
        c.cycles += 3;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
        let no_end = c.g[V1][0] == 0u64;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        if !no_end {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
            if c.g[S5][0] == 0 {
                c.g[V0][0] = c.g[V1][0];
            }
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        }
        c.cycles += 13;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[V0][0] = c.g[A0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = ra;
        Ok(())
    });
}

/// `char *_setlocale_r(reent *r, int category, const char *locale)`
/// (0x41ea38): select the `C`/default locale data, or return NULL for an
/// unknown locale; with `locale == NULL`, query the current setting.
pub fn setlocale_r(c: &mut Cpu) {
    guest(c, |c| {
        'b_41ebe8: {
            'b_41ebe4: {
                'b_41ebe0: {
                    'b_41ebc8: {
                        c.cycles += 11;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[S1][0] = c.g[A2][0];
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[S2][0] = c.g[A1][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.g[S4][0] = c.g[A0][0];
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                        let t = c.g[S1][0] == 0u64;
                        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                        if t {
                            break 'b_41ebc8;
                        } else {
                            'b_41ebb0: {
                                'b_41eba8: {
                                    'b_41eba4: {
                                        'b_41eacc: {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x2;
                                            let t = c.g[S2][0] == c.g[V0][0];
                                            c.g[A1][0] = 0x4a0000;
                                            if t {
                                                break 'b_41eacc;
                                            } else {
                                                'b_41ea98: {
                                                    c.cycles += 5;
                                                    c.g[A0][0] = c.g[S1][0];
                                                    c.g[RA][0] = sx(0x0041ea7c);
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_sub(0x5830));
                                                    callee::STRCMP.call(c, 0x0041_ea7c)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A0][0] = c.g[S1][0];
                                                    if t {
                                                        break 'b_41ea98;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.g[A1][0] = 0x4a0000;
                                                        c.g[RA][0] = sx(0x0041ea90);
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_sub(0x5840));
                                                        callee::STRCMP.call(c, 0x0041_ea90)?;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[V0][0] = 0u64;
                                                        if t {
                                                            break 'b_41ebe8;
                                                        } else {
                                                            break 'b_41ea98;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                if c.g[S2][0] != 0u64 {
                                                    c.sw(
                                                        (c.g[S4][0] as u32).wrapping_add(0x34),
                                                        c.g[S1][0],
                                                    );
                                                    break 'b_41ebb0;
                                                } else {
                                                    c.cycles += 11;
                                                    c.g[S0][0] = 0x450000;
                                                    c.g[A0][0] = 0x450000;
                                                    c.g[S0][0] =
                                                        sx((c.g[S0][0] as u32)
                                                            .wrapping_add(0x5148));
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x5150));
                                                    c.g[RA][0] = sx(0x0041eab8);
                                                    c.g[A1][0] = c.g[S0][0];
                                                    callee::STRCPY.call(c, 0x0041_eab8)?;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x0041eac4);
                                                    c.g[A1][0] = c.g[S1][0];
                                                    callee::STRCPY.call(c, 0x0041_eac4)?;
                                                    let t = true;
                                                    c.g[V1][0] = 0x450000;
                                                    if t {
                                                        break 'b_41eba4;
                                                    } else {
                                                        break 'b_41eacc;
                                                    }
                                                }
                                            }
                                        }
                                        'b_41eb48: {
                                            c.cycles += 8;
                                            c.g[V0][0] = 0x4a0000;
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[S0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_sub(0x5830));
                                            c.g[S3][0] = 0x4a0000;
                                            c.g[RA][0] = sx(0x0041eae4);
                                            c.g[A1][0] = c.g[S0][0];
                                            callee::STRCMP.call(c, 0x0041_eae4)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0x4a0000;
                                            if t {
                                                break 'b_41eb48;
                                            } else {
                                                c.cycles += 5;
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[RA][0] = sx(0x0041eaf8);
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0x5840));
                                                callee::STRCMP.call(c, 0x0041_eaf8)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] = c.g[S0][0];
                                                if t {
                                                    break 'b_41eb48;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[RA][0] = sx(0x0041eb08);
                                                    c.g[A0][0] = c.g[S1][0];
                                                    callee::STRCMP.call(c, 0x0041_eb08)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A1][0] =
                                                        sx((c.g[S3][0] as u32)
                                                            .wrapping_sub(0x5828));
                                                    if t {
                                                        break 'b_41eb48;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[RA][0] = sx(0x0041eb18);
                                                        c.g[A0][0] = c.g[S1][0];
                                                        callee::STRCMP.call(c, 0x0041_eb18)?;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A1][0] = 0x4a0000;
                                                        if t {
                                                            break 'b_41eb48;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.g[A0][0] = c.g[S1][0];
                                                            c.g[RA][0] = sx(0x0041eb2c);
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_sub(0x5820));
                                                            callee::STRCMP.call(c, 0x0041_eb2c)?;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A1][0] = 0x4a0000;
                                                            if t {
                                                                break 'b_41eb48;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[A0][0] = c.g[S1][0];
                                                                c.g[RA][0] = sx(0x0041eb40);
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x5818));
                                                                callee::STRCMP
                                                                    .call(c, 0x0041_eb40)?;
                                                                let t = c.g[V0][0] != 0u64;
                                                                c.g[V0][0] = 0u64;
                                                                if t {
                                                                    break 'b_41ebe8;
                                                                } else {
                                                                    break 'b_41eb48;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        'b_41eb88: {
                                            c.cycles += 14;
                                            c.g[S0][0] = 0x450000;
                                            c.g[A0][0] = 0x450000;
                                            c.g[S0][0] =
                                                sx((c.g[S0][0] as u32).wrapping_add(0x5148));
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x5150));
                                            c.g[RA][0] = sx(0x0041eb60);
                                            c.g[A1][0] = c.g[S0][0];
                                            callee::STRCPY.call(c, 0x0041_eb60)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x0041eb6c);
                                            c.g[A1][0] = c.g[S1][0];
                                            callee::STRCPY.call(c, 0x0041_eb6c)?;
                                            c.g[A1][0] =
                                                sx((c.g[S3][0] as u32).wrapping_sub(0x5828));
                                            c.g[RA][0] = sx(0x0041eb78);
                                            c.g[A0][0] = c.g[S1][0];
                                            callee::STRCMP.call(c, 0x0041_eb78)?;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[V1][0] = 0x450000;
                                            if t {
                                                break 'b_41eb88;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V0][0] = 0x8;
                                                if t {
                                                    break 'b_41eba8;
                                                } else {
                                                    break 'b_41eb88;
                                                }
                                            }
                                        }
                                        c.cycles += 5;
                                        c.g[RA][0] = sx(0x0041eb90);
                                        c.g[A0][0] = c.g[S1][0];
                                        callee::STRLEN.call(c, 0x0041_eb90)?;
                                        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V1][0] = 0x450000;
                                        if t {
                                            break 'b_41eba4;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[V0][0] = 0x2;
                                            if t {
                                                break 'b_41eba8;
                                            } else {
                                                break 'b_41eba4;
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = 0x1;
                                    break 'b_41eba8;
                                }
                                c.cycles += 2;
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x5140), c.g[V0][0]);
                                c.sw((c.g[S4][0] as u32).wrapping_add(0x34), c.g[S1][0]);
                                break 'b_41ebb0;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = 0x2;
                            let t = c.g[S2][0] != c.g[V0][0];
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x30), c.g[S2][0]);
                            if t {
                                break 'b_41ebe0;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = 0x450000;
                                let t = true;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5150));
                                if t {
                                    break 'b_41ebe8;
                                } else {
                                    break 'b_41ebc8;
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    if c.g[S2][0] != c.g[V0][0] {
                        c.g[V0][0] = 0x4a0000;
                        break 'b_41ebe4;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x450000;
                        let t = true;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5148));
                        if t {
                            break 'b_41ebe8;
                        } else {
                            break 'b_41ebe0;
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0x4a0000;
                break 'b_41ebe4;
            }
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5830));
            break 'b_41ebe8;
        }
        c.cycles += 8;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = ra; // setlocale_r epilogue
        Ok(())
    });
}

/// `void *_calloc_r(reent *r, size_t n, size_t size)`
/// (0x4224a0): `n * size` zeroed bytes through `_malloc_r` + `memset`.
/// Small results are zeroed word by word; larger ones via `memset`.
/// Returns the block, or NULL when the allocation fails.
pub fn calloc_r(c: &mut Cpu) {
    guest(c, |c| {
        'b_42254c: {
            'b_4224d0: {
                c.cycles += 10;
                c.mult(0, 0, 5, 6);
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[A1][0] = c.lo[0];
                c.g[RA][0] = sx(0x004224bc);
                callee::MALLOC_R.call(c, 0x0042_24bc)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] != 0u64 {
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x4));
                    break 'b_4224d0;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_42254c;
                    } else {
                        break 'b_4224d0;
                    }
                }
            }
            'b_422548: {
                'b_42253c: {
                    c.cycles += 7;
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[V0][0] = c.g[V0][0] | 0xfffc;
                    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4));
                    c.g[V0][0] = (c.g[A2][0] < 0x25u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = (c.g[A2][0] < 0x14u64) as u64;
                    if t {
                        break 'b_42253c;
                    } else {
                        'b_422528: {
                            c.cycles += 2;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_422528;
                            } else {
                                c.cycles += 5;
                                c.sw(c.g[S0][0] as u32, 0u64);
                                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = (c.g[A2][0] < 0x1cu64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
                                if t {
                                    break 'b_422528;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = (c.g[A2][0] < 0x24u64) as u64;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                                    if t {
                                        break 'b_422528;
                                    } else {
                                        c.cycles += 3;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
                                        break 'b_422528;
                                    }
                                }
                            }
                        }
                        c.cycles += 5;
                        c.sw(c.g[A0][0] as u32, 0u64);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), 0u64);
                        let t = true;
                        c.sw(c.g[A0][0] as u32, 0u64);
                        if t {
                            break 'b_422548;
                        } else {
                            break 'b_42253c;
                        }
                    }
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00422548);
                c.g[A1][0] = 0u64;
                callee::MEMSET.call(c, 0x0042_2548)?;
                break 'b_422548;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
            break 'b_42254c;
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    });
}

/// `char *_gcvt(reent *r, double v, int ndigits, char *buf)` (0x421b98):
/// format `v` with `ndigits` significant digits into `buf` (like `%g`),
/// returning `buf`. Handles sign, zero, huge (via `print_e`), and the
/// `_dtoa_r` digit path with trailing-zero and alternate-form fixups.
pub fn _gcvt(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.g[T0][0] = sx((c.g[T0][0] as u32) << 24);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S0][0] = c.g[A3][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S2][0] = c.g[A1][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S3][0] = 0u64;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S4][0] = c.g[A0][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S1][0] = c.g[A2][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S5][0] = c.g[S0][0];
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S6][0] = c.g[T1][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[S7][0] = sx(((c.g[T0][0] as u32) as i32 >> 24) as u32);
        // Sign: negate negatives, remember via the `S7` flag path below.
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0042_1bf0);
        c.g[A1][0] = c.g[S3][0];
        callee::CMPDF2.call(c, 0x0042_1bf0)?;
        c.g[A1][0] = c.g[S3][0];
        if (c.g[V0][0] as i64) < 0 {
            c.cycles += 5;
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x0042_1c04);
            c.g[A0][0] = c.g[S3][0];
            callee::SUBDF3.call(c, 0x0042_1c04)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
        }
        // Zero check.
        c.cycles += 4;
        c.g[RA][0] = sx(0x0042_1c14);
        c.g[A0][0] = c.g[S2][0];
        callee::CMPDF2.call(c, 0x0042_1c14)?;
        let nonzero = c.g[V0][0] != 0u64;
        c.g[V0][0] = 0x30;
        if !nonzero {
            // Zero: `"0"` + NUL.
            c.cycles += 3;
            c.sb((c.g[S5][0] as u32).wrapping_add(0x1), 0u64);
            c.sb(c.g[S5][0] as u32, c.g[V0][0]);
            c.cycles += 1;
            c.g[V0][0] = c.g[S5][0];
        } else {
            // Nonzero: huge magnitudes go through `print_e`.
            let mut via_e = false;
            c.cycles += 11;
            c.g[A1][0] = 0xfc68;
            c.g[A1][0] <<= 16;
            c.g[A1][0] |= 0xdb8b;
            c.g[A1][0] <<= 16;
            c.g[A1][0] |= 0xac71;
            c.g[A1][0] <<= 14;
            c.g[A1][0] |= 0x32d;
            c.g[RA][0] = sx(0x0042_1c4c);
            c.g[A0][0] = c.g[S2][0];
            callee::CMPDF2.call(c, 0x0042_1c4c)?;
            c.g[A0][0] = c.g[S4][0];
            if (c.g[V0][0] as i64) <= 0 {
                via_e = true;
            } else {
                c.cycles += 7;
                c.g[RA][0] = sx(0x0042_1c5c);
                c.g[A0][0] = c.g[S1][0];
                callee::MPREC_LOG10.call(c, 0x0042_1c5c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0042_1c68);
                c.g[A0][0] = c.g[S2][0];
                callee::CMPDF2.call(c, 0x0042_1c68)?;
                c.g[A0][0] = c.g[S4][0];
                if (c.g[V0][0] as i64) >= 0 {
                    via_e = true;
                }
            }
            if via_e {
                // `print_e` path: exponent form into the buffer.
                c.cycles += 8;
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                c.g[T0][0] = c.g[S7][0];
                c.g[T1][0] = c.g[S6][0];
                c.g[RA][0] = sx(0x0042_1c88);
                c.g[A1][0] = c.g[S5][0];
                callee::PRINT_E.call(c, 0x0042_1c88)?;
                c.g[V0][0] = c.g[S5][0];
            } else {
                // `dtoa` digits path.
                c.cycles += 6;
                c.g[A1][0] = 0xffc0;
                c.g[A1][0] <<= 46;
                c.g[RA][0] = sx(0x0042_1ca0);
                c.g[A0][0] = c.g[S2][0];
                callee::CMPDF2.call(c, 0x0042_1ca0)?;
                c.g[A1][0] = c.g[S2][0];
                if (c.g[V0][0] as i64) >= 0 {
                    // At least 1: `ndigits` significant digits, mode 2.
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A2][0] = 0x2;
                } else {
                    // Below 1: `ndigits` digits after the point, mode 3.
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A2][0] = 0x3;
                }
                c.cycles += 10;
                c.g[A3][0] = c.g[S1][0];
                c.g[T0][0] = c.g[SP][0];
                c.g[T1][0] = c.g[SP][0] | 0x4;
                c.g[RA][0] = sx(0x0042_1cd0);
                c.g[T2][0] = c.g[SP][0] | 0x8;
                callee::DTOA_R.call(c, 0x0042_1cd0)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                c.g[V0][0] = 0x270f;
                if c.g[V1][0] == c.g[V0][0] {
                    // `Infinity`: copy the whole word.
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0042_1cec);
                    c.g[A0][0] = c.g[S0][0];
                    callee::STRCPY.call(c, 0x0042_1cec)?;
                    c.g[V0][0] = c.g[S0][0];
                } else {
                    // Classify the first digit.
                    c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                    c.cycles += 2;
                    c.g[A0][0] = c.lbu(c.g[A1][0] as u32);
                    // Empty digits go straight to the length check; a
                    // nonpositive decpt skips the copy loop entirely.
                    let mut check_len = c.g[V0][0] == 0;
                    if c.g[V0][0] != 0 {
                        c.cycles += 2;
                        if (c.g[V1][0] as i64) > 0 {
                            // Leading digit before the point.
                            c.cycles += 1;
                            c.sb(c.g[S0][0] as u32, c.g[A0][0]);
                            // Copy digits, one per byte.
                            loop {
                                c.cycles += 9;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                                c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                                c.g[A0][0] = c.lbu(c.g[A1][0] as u32);
                                if c.g[V0][0] == 0 {
                                    break;
                                }
                                c.cycles += 2;
                                if (c.g[V1][0] as i64) <= 0 {
                                    break;
                                }
                                c.sb(c.g[S0][0] as u32, c.g[A0][0]);
                                if back_edge_unwinds(c, 0x0042_1d08) {
                                    return Ok(());
                                }
                            }
                            check_len = true;
                        }
                    }
                    // Zero-fill while the point is still ahead. Gen `0x421d40`
                    // skips the loop when `S1 <= 0` and `0x421d68` leaves
                    // it as soon as `S1 <= 0` again.
                    if check_len {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                        if (c.g[V0][0] as i64) > 0 {
                            c.cycles += 2;
                            if (c.g[S1][0] as i64) > 0 {
                                c.cycles += 2;
                                c.g[V1][0] = 0x30;
                                c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                                loop {
                                    c.cycles += 6;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                    c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                                    if (c.g[V0][0] as i64) <= 0 {
                                        break;
                                    }
                                    c.cycles += 2;
                                    if (c.g[S1][0] as i64) <= 0 {
                                        break;
                                    }
                                    c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                                    if back_edge_unwinds(c, 0x0042_1d50) {
                                        return Ok(());
                                    }
                                }
                            }
                        }
                    }
                    // Close the `check_len` length check: point and
                    // fraction, unless suppressed entirely.
                    c.cycles += 2;
                    let mut tail_done = false;
                    if c.g[S6][0] == 0 {
                        // No alternate form: end here if no digits follow.
                        c.cycles += 3;
                        c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                        if c.g[V0][0] == 0 {
                            c.sb(c.g[S0][0] as u32, 0u64);
                            c.cycles += 1;
                            c.g[V0][0] = c.g[S5][0];
                            tail_done = true;
                        }
                    }
                    if !tail_done {
                        // Emit `.`, then the fraction digits.
                        c.cycles += 2;
                        c.g[V0][0] = 0x2e;
                        if c.g[S0][0] == c.g[S5][0] {
                            c.cycles += 4;
                            c.g[V0][0] = 0x30;
                            c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                            c.sb(c.g[S5][0] as u32, c.g[V0][0]);
                            c.g[V0][0] = 0x2e;
                        }
                        c.cycles += 4;
                        c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        if (c.g[V0][0] as i64) >= 0 {
                            // Nonnegative decpt: copy significant digits.
                            loop {
                                c.cycles += 1;
                                c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                                c.cycles += 2;
                                c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
                                if c.g[V0][0] == 0 {
                                    break;
                                }
                                c.cycles += 2;
                                if (c.g[S1][0] as i64) <= 0 {
                                    break;
                                }
                                c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                                if back_edge_unwinds(c, 0x0042_1de0) {
                                    return Ok(());
                                }
                                c.cycles += 3;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                            }
                        } else {
                            // Negative decpt: zero-fill, then significant digits.
                            c.cycles += 2;
                            c.g[V1][0] = 0x30;
                            if (c.g[S1][0] as i64) > 0 {
                                c.cycles += 1;
                                c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                                let mut to_dec = false;
                                loop {
                                    c.cycles += 6;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                                    if (c.g[V0][0] as i64) >= 0 {
                                        to_dec = true;
                                        break;
                                    }
                                    c.cycles += 2;
                                    if (c.g[S1][0] as i64) <= 0 {
                                        break;
                                    }
                                    c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                                    if back_edge_unwinds(c, 0x0042_1db8) {
                                        return Ok(());
                                    }
                                }
                                if to_dec {
                                    loop {
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                                        c.cycles += 2;
                                        c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
                                        if c.g[V0][0] == 0 {
                                            break;
                                        }
                                        c.cycles += 2;
                                        if (c.g[S1][0] as i64) <= 0 {
                                            break;
                                        }
                                        c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                                        if back_edge_unwinds(c, 0x0042_1de0) {
                                            return Ok(());
                                        }
                                        c.cycles += 3;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                    }
                                } else {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                                    loop {
                                        c.cycles += 2;
                                        c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
                                        if c.g[V0][0] == 0 {
                                            break;
                                        }
                                        c.cycles += 2;
                                        if (c.g[S1][0] as i64) <= 0 {
                                            break;
                                        }
                                        c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                                        if back_edge_unwinds(c, 0x0042_1de0) {
                                            return Ok(());
                                        }
                                        c.cycles += 3;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                                    }
                                }
                            } else {
                                // No fraction digits wanted: straight on.
                                loop {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                                    c.cycles += 2;
                                    c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
                                    if c.g[V0][0] == 0 {
                                        break;
                                    }
                                    c.cycles += 2;
                                    if (c.g[S1][0] as i64) <= 0 {
                                        break;
                                    }
                                    c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                                    if back_edge_unwinds(c, 0x0042_1de0) {
                                        return Ok(());
                                    }
                                    c.cycles += 3;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                }
                            }
                        }
                        // Trailing `'0'` fill unless suppressed.
                        c.cycles += 2;
                        if c.g[S6][0] == 0 {
                            c.sb(c.g[S0][0] as u32, 0u64);
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = 0x30;
                            if (c.g[S1][0] as i64) > 0 {
                                loop {
                                    c.cycles += 7;
                                    c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    if (c.g[S1][0] as i64) <= 0 {
                                        break;
                                    }
                                    if back_edge_unwinds(c, 0x0042_1e10) {
                                        return Ok(());
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.sb(c.g[S0][0] as u32, 0u64);
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S5][0];
                    }
                }
            }
        }
        c.cycles += 11;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = ra;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// memmove / strcat / strncat
// ---------------------------------------------------------------------------

/// `void *memmove(void *dst, const void *src, size_t n)` (0x41610c): copy
/// `n` bytes, backwards from the end when `src < dst` so overlap is safe.
/// Returns `dst`.
pub fn memmove(c: &mut Cpu) {
    c.cycles += 4;
    c.g[T0][0] = c.g[A0][0];
    c.g[V0][0] = (c.g[A1][0] < c.g[T0][0]) as u64;
    c.g[V1][0] = c.g[T0][0];
    if c.g[V0][0] != 0 {
        // `src < dst`: copy backwards when overlapping, else forward.
        // (The overlap test the original computes into `v0` is discarded:
        // `v0` is forced to -1 either way.)
        c.cycles += 4;
        c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V0][0] = (c.g[T0][0] < c.g[A3][0]) as u64;
        let no_overlap = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0xffffffffffffffff;
        if !no_overlap {
            c.cycles += 4;
            c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
            c.g[A1][0] = c.g[A3][0];
            if c.g[A2][0] == c.g[V0][0] {
                // `n == 0`.
                c.cycles += 2;
                c.g[V0][0] = c.g[T0][0];
                abi::ret(c);
                return;
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[V0][0];
            loop {
                c.cycles += 7;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                let more = c.g[A2][0] != c.g[A0][0];
                c.sb(c.g[V1][0] as u32, c.g[V0][0]);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0041_6140);
                }
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[T0][0];
            abi::ret(c);
            return;
        }
    }
    crate::game::misc::memmove_forward(c);
}

/// `char *strcat(char *dst, const char *src)` (0x4162d0): append `src`
/// (with its NUL) at the end of `dst`. Returns `dst`. The end of `dst` is
/// found with the usual SIMD zero-byte scan, then the tail is `strcpy`.
pub fn strcat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, c.g[SP][0] as u32);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.g[S0][0] & 0x7;
        let unaligned = c.g[V0][0] != 0u64;
        c.sq(31, (c.g[SP][0] as u32).wrapping_add(0x10));
        if !unaligned {
            c.cycles += 17;
            c.g[V0][0] = c.g[S0][0] & 0xf;
            // Zero-byte detect masks.
            c.g[V1][0] = 0x8080;
            c.g[V1][0] <<= 16;
            c.g[V1][0] |= 0x8080;
            c.g[V1][0] <<= 16;
            c.g[V1][0] |= 0x8080;
            c.g[V1][0] <<= 9;
            c.g[V1][0] |= 0x101;
            c.g[A0][0] = 0x8080;
            c.g[A0][0] <<= 16;
            c.g[A0][0] |= 0x8080;
            c.g[A0][0] <<= 16;
            c.g[A0][0] |= 0x8080;
            c.g[A0][0] <<= 16;
            c.g[A0][0] |= 0x8080;
            let dw_misaligned = c.g[V0][0] != 0u64;
            c.g[A2][0] = c.ld(c.g[S0][0] as u32);
            if !dw_misaligned {
                c.cycles += 11;
                c.lq(2, c.g[S0][0] as u32);
                c.pcpyld(7, 3, 3);
                c.pcpyld(8, 4, 4);
                c.psubb(3, 2, 7);
                c.pnor(2, 0, 2);
                c.pand(3, 3, 2);
                c.pand(3, 3, 8);
                c.pcpyud(2, 3, 3);
                c.g[V1][0] = c.g[V0][0] | c.g[V1][0];
                let found = c.g[V1][0] != 0u64;
                c.g[A0][0] = c.g[S0][0];
                if !found {
                    c.cycles += 1;
                    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                    'scan16: loop {
                        c.cycles += 9;
                        c.lq(2, c.g[A2][0] as u32);
                        c.pnor(3, 0, 2);
                        c.psubb(2, 2, 7);
                        c.pand(2, 2, 3);
                        c.pand(2, 2, 8);
                        c.pcpyud(3, 2, 2);
                        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                        if c.g[V0][0] == 0u64 {
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                            if event_due(c) {
                                deliver_event_here(c, 0x0041_635c);
                            }
                            continue 'scan16;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.g[A2][0];
                        break;
                    }
                }
            } else {
                // `dst` is 8-aligned but not 16-aligned: test the first
                // doubleword, then scan 8 bytes at a time.
                c.cycles += 8;
                c.g[A3][0] = c.g[V1][0];
                c.g[T0][0] = c.g[A0][0];
                c.g[V1][0] = c.g[A2][0].wrapping_sub(c.g[V1][0]);
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                c.g[V1][0] &= c.g[V0][0];
                c.g[V1][0] &= c.g[A0][0];
                let found = c.g[V1][0] != 0u64;
                c.g[A0][0] = c.g[S0][0];
                if !found {
                    c.cycles += 1;
                    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                    'scan8: loop {
                        c.cycles += 7;
                        c.g[V0][0] = c.ld(c.g[A2][0] as u32);
                        c.g[V1][0] = !(0u64 | c.g[V0][0]);
                        c.g[V0][0] = c.g[V0][0].wrapping_sub(c.g[A3][0]);
                        c.g[V0][0] &= c.g[V1][0];
                        c.g[V0][0] &= c.g[T0][0];
                        if c.g[V0][0] == 0u64 {
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
                            if event_due(c) {
                                deliver_event_here(c, 0x0041_63ac);
                            }
                            continue 'scan8;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[A2][0];
                        break;
                    }
                }
            }
        }
        // Byte scan for the NUL from the candidate chunk.
        'scan1: loop {
            c.cycles += 7;
            c.g[V0][0] = c.lb(c.g[A0][0] as u32);
            if c.g[V0][0] != 0u64 {
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                if event_due(c) {
                    deliver_event_here(c, 0x0041_63cc);
                }
                continue 'scan1;
            }
            break;
        }
        c.cycles += 7;
        callee::STRCPY.call(c, 0x0041_63f0)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(31, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(16, c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `char *strncat(char *dst, const char *src, size_t n)` (0x416958):
/// append at most `n` bytes of `src` (plus a NUL, unless `n == 0`) at the
/// end of `dst`. Returns `dst`.
pub fn strncat(c: &mut Cpu) {
    c.cycles += 4;
    c.g[T0][0] = c.g[A0][0];
    c.g[V0][0] = c.g[T0][0] & 0x7;
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[V1][0] = c.g[T0][0] & 0xf;
        if c.g[V1][0] == 0 {
            c.cycles += 25;
            c.g[A3][0] = 0x8080;
            c.g[A3][0] <<= 16;
            c.g[A3][0] |= 0x8080;
            c.g[A3][0] <<= 16;
            c.g[A3][0] |= 0x8080;
            c.g[A3][0] <<= 9;
            c.g[A3][0] |= 0x101;
            c.lq(2, c.g[T0][0] as u32);
            c.pcpyld(9, 7, 7);
            c.g[A0][0] = 0x8080;
            c.g[A0][0] <<= 16;
            c.g[A0][0] |= 0x8080;
            c.g[A0][0] <<= 16;
            c.g[A0][0] |= 0x8080;
            c.g[A0][0] <<= 16;
            c.g[A0][0] |= 0x8080;
            c.psubb(3, 2, 9);
            c.pnor(2, 0, 2);
            c.pcpyld(10, 4, 4);
            c.pand(3, 3, 2);
            c.pand(3, 3, 10);
            c.pcpyud(2, 3, 4);
            c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
            c.g[A3][0] = c.g[T0][0];
            if c.g[V1][0] == 0 {
                c.cycles += 2;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10));
                'scan16: loop {
                    c.cycles += 9;
                    c.lq(2, c.g[A3][0] as u32);
                    c.pnor(3, 0, 2);
                    c.psubb(2, 2, 9);
                    c.pand(2, 2, 3);
                    c.pand(2, 2, 10);
                    c.pcpyud(3, 2, 4);
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    let clean = c.g[V0][0] == 0u64;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10));
                    if clean {
                        if event_due(c) {
                            deliver_event_here(c, 0x0041_69e0);
                        }
                        continue 'scan16;
                    }
                    c.cycles += 2;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x10));
                    break;
                }
            }
        } else {
            c.cycles += 20;
            c.g[V0][0] = c.ld(c.g[T0][0] as u32);
            c.g[T1][0] = 0x8080;
            c.g[T1][0] <<= 16;
            c.g[T1][0] |= 0x8080;
            c.g[T1][0] <<= 16;
            c.g[T1][0] |= 0x8080;
            c.g[T1][0] <<= 9;
            c.g[T1][0] |= 0x101;
            c.g[A0][0] = 0x8080;
            c.g[A0][0] <<= 16;
            c.g[A0][0] |= 0x8080;
            c.g[A0][0] <<= 16;
            c.g[A0][0] |= 0x8080;
            c.g[A0][0] <<= 16;
            c.g[A0][0] |= 0x8080;
            c.g[V1][0] = c.g[V0][0].wrapping_sub(c.g[T1][0]);
            c.g[V0][0] = !(0u64 | c.g[V0][0]);
            c.g[V1][0] &= c.g[V0][0];
            c.g[V1][0] &= c.g[A0][0];
            c.g[A3][0] = c.g[T0][0];
            if c.g[V1][0] == 0 {
                c.cycles += 1;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x8));
                'scan8: loop {
                    c.cycles += 7;
                    c.g[V0][0] = c.ld(c.g[A3][0] as u32);
                    c.g[V1][0] = !(0u64 | c.g[V0][0]);
                    c.g[V0][0] = c.g[V0][0].wrapping_sub(c.g[T1][0]);
                    c.g[V0][0] &= c.g[V1][0];
                    c.g[V0][0] &= c.g[A0][0];
                    if c.g[V0][0] == 0u64 {
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x8));
                        if event_due(c) {
                            deliver_event_here(c, 0x0041_6a60);
                        }
                        continue 'scan8;
                    }
                    break;
                }
            }
        }
        c.cycles += 2;
        c.g[A0][0] = c.g[A3][0];
        c.g[V0][0] = c.lb(c.g[A0][0] as u32);
    } else {
        c.g[V0][0] = c.lb(c.g[A0][0] as u32);
    }
    c.cycles += 2;
    let at_end = c.g[V0][0] == 0u64;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
    if !at_end {
        c.cycles += 1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        'scan1: loop {
            c.cycles += 7;
            c.g[V0][0] = c.lb(c.g[A0][0] as u32);
            if c.g[V0][0] != 0u64 {
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                if event_due(c) {
                    deliver_event_here(c, 0x0041_6a90);
                }
                continue 'scan1;
            }
            break;
        }
    }
    c.cycles += 4;
    c.g[V0][0] = 0xffffffffffff0000;
    c.g[V0][0] |= 0xffff;
    if c.g[A2][0] != c.g[V0][0] {
        c.cycles += 6;
        c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
        c.sb(c.g[A0][0] as u32, c.g[V0][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
        let src_ends = c.g[V0][0] == 0u64;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        if !src_ends {
            c.cycles += 3;
            c.g[V1][0] = 0xffffffffffff0000;
            c.g[V1][0] |= 0xffff;
            loop {
                c.cycles += 2;
                if c.g[A2][0] == 0u64 {
                    c.sb(c.g[A0][0] as u32, 0u64);
                }
                c.cycles += 3;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                if c.g[A2][0] == c.g[V1][0] {
                    break;
                }
                c.cycles += 6;
                c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                let src_more = c.g[V0][0] != 0u64;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                if !src_more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0041_6ae0);
                }
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T0][0];
    abi::ret(c);
}

/// `int fseek(FILE *fp, long offset, int whence)` (0x417008): reposition
/// a stream. `whence` 0/1/2 with a working seek cookie get past
/// validation; anything else fails with `EINVAL`. The position math
/// discounts the buffered but unread bytes, then the cookie seeks.
pub fn fseek(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[S0][0] = c.g[A0][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S4][0] = c.g[A2][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
        c.g[S6][0] = c.g[A1][0];
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = 0x450000;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4944));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V1][0]);
        }
        c.cycles += 4;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x38));
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0041_7068);
            callee::SINIT.call(c, 0x0041_7068)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        } else {
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        }
        c.cycles += 4;
        c.g[V1][0] = 0x108;
        c.g[V0][0] &= 0x108;
        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0041_7084);
            c.g[A0][0] = c.g[S0][0];
            callee::FFLUSH.call(c, 0x0041_7084)?;
        }
        c.cycles += 3;
        c.g[S5][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = 0x1;
        if c.g[S5][0] == 0 {
            // No seek cookie: `EINVAL` (0x1d). Generated 0x417090 falls
            // into 0x41716c, which sets `v0 = -1` before the shared tail.
            c.cycles += 2;
            c.g[V1][0] = 0x1d;
            c.cycles += 1;
            c.g[V0][0] = 0xffffffffffffffff;
            crate::game::misc::fseek_fail(c);
            return Ok(());
        }
        c.cycles += 2;
        let seek_cur = c.g[S4][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[S4][0] as i64) < 2i64) as u64;
        if !seek_cur {
            // 0x4170a0 validates the range first.
            c.cycles += 2;
            let above_one = c.g[V0][0] == 0u64;
            // Delay slot of the range check: `V0 = 2` on both paths.
            c.g[V0][0] = 0x2;
            if above_one {
                // `whence` above 1: 2 continues below, anything else fails.
                c.cycles += 2;
                c.g[V1][0] = 0x16;
                if c.g[S4][0] == c.g[V0][0] {
                    // `whence` 2: reset the known-position flag, then join
                    // the positioned tail.
                    c.cycles += 3;
                    c.g[S7][0] = 0u64;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    crate::game::misc::fseek_positioned(c)?;
                    return Ok(());
                }
                // Above `SEEK_END`: `EINVAL`.
                c.cycles += 2;
                c.g[V0][0] = 0xffffffffffffffff;
                crate::game::misc::fseek_fail(c);
                return Ok(());
            }
            // `whence` below 2 but not `SEEK_CUR`: 0 continues below,
            // anything else fails.
            c.cycles += 2;
            c.g[V1][0] = 0x16;
            if c.g[S4][0] == 0 {
                // `whence` 0: reset the known-position flag, then join the
                // positioned tail.
                c.cycles += 3;
                c.g[S7][0] = 0u64;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                crate::game::misc::fseek_positioned(c)?;
                return Ok(());
            }
            // Negative `whence`: `EINVAL`.
            c.cycles += 2;
            c.g[V0][0] = 0xffffffffffffffff;
            crate::game::misc::fseek_fail(c);
            return Ok(());
        }
        // `whence == SEEK_CUR` from here.
        c.cycles += 6;
        c.g[RA][0] = sx(0x0041_70d0);
        c.g[A0][0] = c.g[S0][0];
        callee::FFLUSH.call(c, 0x0041_70d0)?;
        c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[V1][0] &= 0x1000;
        if c.g[V1][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
        } else {
            c.cycles += 8;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
            c.g[A1][0] = 0u64;
            let seek = c.g[S5][0] as u32;
            c.g[RA][0] = sx(0x0041_70f0);
            c.g[A2][0] = 0x1;
            call::call_ptr(c, seek, 0x0041_70f0)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V0][0] = 0xffffffffffffffff;
            if c.g[S2][0] == c.g[V0][0] {
                // Cookie reports failure.
                c.cycles += 2;
                c.g[V0][0] = 0xffffffffffffffff;
                c.cycles += 11;
                crate::game::misc::finish_fseek(c);
                return Ok(());
            }
            c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[V1][0] & 0x4;
        c.g[V0][0] = c.g[V1][0] & 0x8;
        if c.g[V1][0] & 0x4 == 0 {
            c.cycles += 2;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                let present = c.g[V0][0] != 0u64;
                // Delay slot of the check below: computed on both paths.
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                if present {
                    c.cycles += 1;
                    c.g[S2][0] = c.g[S2][0].wrapping_add(c.g[V0][0]);
                }
            }
        } else {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = c.g[S2][0].wrapping_sub(c.g[V0][0]);
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            if c.g[V1][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                c.g[S2][0] = c.g[S2][0].wrapping_sub(c.g[V0][0]);
            }
        }
        c.cycles += 4;
        c.g[S6][0] = c.g[S6][0].wrapping_add(c.g[S2][0]);
        c.g[S4][0] = 0u64;
        c.g[S7][0] = 0x1;
        crate::game::misc::fseek_positioned(c)?;
        Ok(())
    });
}

/// `int __smakebuf(FILE *fp)` (0x4174b0): make `fp` buffered. Unbuffered
/// files get a one-byte buffer at `fp + 0x43`; otherwise `fstat` and the
/// seek cookie pick the flags.
pub fn smakebuf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[V0][0] &= 0x2;
        // Delay slot of the check below: set on both paths.
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x43));
        if c.g[V0][0] != 0 {
            // Unbuffered: a one-byte buffer at `fp + 0x43`. The shared
            // tail below is skipped on this path.
            c.cycles += 5;
            c.g[V0][0] = 0x1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
        } else {
            c.cycles += 3;
            c.g[A1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xe));
            if (c.g[A1][0] as i64) < 0 {
                c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                c.g[RA][0] = sx(0x0041_74fc);
                c.g[A2][0] = c.g[SP][0];
                callee::FSTAT_R.call(c, 0x0041_74fc)?;
                let stat_failed = (c.g[V0][0] as i64) < 0;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                if stat_failed {
                    // Only this path runs the block tail (`+1; V0 = flags`):
                    // every other arm breaks past it.
                    c.cycles += 1;
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = 0x8000;
                    c.g[V0][0] = c.g[V0][0] & 0xf000;
                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = 0x410000;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7a30));
                        let is_tty = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
                        if !is_tty {
                            // Non-tty char device: tail directly, without the
                            // shared `|= 0x800` below.
                            c.cycles += 4;
                            c.g[V1][0] = 0x400;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                            c.g[V0][0] |= 0x400;
                            c.cycles += 9;
                            c.sh((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x43));
                            c.g[V1][0] = 0x1;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                            c.g[V0][0] |= 0x2;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                            c.sh((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            c.sw(c.g[S0][0] as u32, c.g[A0][0]);
                            c.cycles += 4;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                            let ra = c.g[RA][0] as u32;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                            c.pc = ra;
                            return Ok(());
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] |= 0x800;
            c.cycles += 9;
            c.sh((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x43));
            c.g[V1][0] = 0x1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
            c.g[V0][0] |= 0x2;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
            c.sh((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            c.sw(c.g[S0][0] as u32, c.g[A0][0]);
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = ra;
        Ok(())
    });
}

/// `int printf(const char *fmt, ...)` (0x4175c8): the global-`reent`
/// `stdout` is wired to the reent, then `vfprintf` formats into it.
pub fn printf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.g[T4][0] = 0x450000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[A2][0]);
        c.g[A1][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[A3][0]);
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x58));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T0][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[T2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x88), c.g[T3][0]);
        c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x48));
        c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0x4c));
        c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0x54));
        c.g[V0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x4944));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x54), c.g[V0][0]);
        c.g[RA][0] = sx(0x0041_761c);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        callee::VFPRINTF_DISPATCH.call(c, 0x0041_761c)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = ra;
        Ok(())
    });
}

/// `int lflush(FILE *fp)` (0x417628): flush helper: always calls
/// `fflush` and returns its result.
pub fn lflush(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0041_7638);
        callee::FFLUSH.call(c, 0x0041_7638)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `int __srefill(FILE *fp)` (0x417648): refill a read buffer. Handles the
/// uninitialised-reent, unbuffered (`__SNBF`), short-read (`__SRD` with a
/// `__sread` cookie) and line-buffered cases, then calls the `FILE`'s read
/// cookie through the function table. Returns 0 on success, -1 on EOF.
pub fn srefill(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = 0x450000;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4944));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V1][0]);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
        } else {
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x38));
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0041_768c);
            callee::SINIT.call(c, 0x0041_768c)?;
            c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
        } else {
            c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[V1][0] & 0x20;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
        if c.g[V0][0] != 0 {
            // `__SNBF`: unbuffered, EOF.
            c.cycles += 2;
            c.g[V0][0] = 0xffffffffffffffff;
            c.cycles += 4;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            let ra = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.pc = ra;
            return Ok(());
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] & 0x4;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] & 0x10;
                let short = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.g[V1][0] & 0x8;
                if !short {
                    // Neither `__SRD` nor `__SWR`: EOF.
                    c.cycles += 2;
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.cycles += 4;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.pc = ra;
                    return Ok(());
                } else if c.g[V0][0] != 0 {
                    // `__SWR` set: flush first.
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0041_76cc);
                    c.g[A0][0] = c.g[S0][0];
                    c.cycles += 4;
                    callee::FFLUSH.call(c, 0x0041_76cc)?;
                    let flush_failed = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if flush_failed {
                        c.cycles += 4;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        let ra = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.pc = ra;
                        return Ok(());
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V0][0] &= 0xfff7;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
                    c.sh((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.cycles += 3;
                    c.g[V0][0] |= 0x4;
                    c.sh((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.cycles += 3;
                    c.g[V0][0] |= 0x4;
                    c.sh((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                }
            } else {
                // `__SRD` with a pending short read: account it first.
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                    let empty = c.g[V0][0] == 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    if empty {
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                        c.g[V0][0] = 0u64;
                        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                        // Gen `0x41770c` returns straight to the epilogue.
                        c.cycles += 4;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        let ra = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.pc = ra;
                        return Ok(());
                    }
                }
            }
            if c.g[V0][0] == 0 {
                // No buffer yet (or a short read): make one, then settle
                // the flags and call the read cookie. Gen `0x417720`
                // (+2) falls into `0x417728` (+3) here.
                c.cycles += 5;
                c.g[RA][0] = sx(0x0041_7730);
                c.g[A0][0] = c.g[S0][0];
                callee::SMAKEBUF.call(c, 0x0041_7730)?;
                c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
            }
            c.cycles += 3;
            c.g[V0][0] &= 0x3;
            let plain = c.g[V0][0] == 0u64;
            c.g[A1][0] = 0x410000;
            if !plain {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                c.g[RA][0] = sx(0x0041_774c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7628));
                callee::FWALK.call(c, 0x0041_774c)?;
            }
            c.cycles += 13;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
            c.g[A1][0] = c.g[V1][0];
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            let read = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0041_7768);
            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
            call::call_ptr(c, read, 0x0041_7768)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
            c.g[V0][0] &= 0xdfff;
            let got = (c.g[V1][0] as i64) > 0;
            c.sh((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            if !got {
                c.cycles += 2;
                if c.g[V1][0] != 0u64 {
                    c.g[V0][0] |= 0x40;
                    c.cycles += 1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] |= 0x20;
                }
                c.cycles += 2;
                c.sh((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                if back_edge_unwinds(c, 0x0041_76b4) {
                    return Ok(());
                }
                // Back at the `__SNBF` head: EOF (-1).
                c.cycles += 2;
                c.g[V0][0] = 0xffffffffffffffff;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
            }
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `int sscanf(const char *str, const char *fmt, ...)` (0x4178b0): `scanf`
/// from a string. Measures the string, then funnels into `__svfscanf`
/// through a stack `__sFILE` with `__SRD|__SSTR` (4) flags.
pub fn sscanf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
        c.g[V0][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = 0x4;
        c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[A2][0]);
        c.g[S0][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
        c.sh((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0xc8), c.g[A3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[T0][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0xd8), c.g[T1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[T2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0xe8), c.g[T3][0]);
        c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0xb4));
        c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0xb8));
        c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0xbc));
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[RA][0] = sx(0x0041_7900);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        callee::STRLEN.call(c, 0x0041_7900)?;
        c.g[A0][0] = 0x450000;
        c.g[V1][0] = 0x410000;
        c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4944));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x78a8));
        c.g[A1][0] = c.g[S0][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x54), c.g[A2][0]);
        c.g[A0][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V1][0]);
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
        c.g[RA][0] = sx(0x0041_7938);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x44), 0u64);
        callee::SVFSCANF.call(c, 0x0041_7938)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.pc = ra;
        Ok(())
    });
}

/// `unsigned char *__sccl(char *tab, const unsigned char *fmt)`
/// (0x4185d0): `scanf` `%[...]` helper. Fills the 256-byte table at `tab`
/// with the negate flag, then sets one byte per class member (`a-z` ranges
/// included); `]` ends the class, NUL ends the format. Returns the format
/// position past `]` (or at the NUL).
pub fn sccl(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = c.lb(c.g[A1][0] as u32);
    c.g[V0][0] = 0x5e; // '^'
    let no_caret = c.g[A2][0] != c.g[V0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
    c.g[T0][0] = 0u64;
    if !no_caret {
        c.cycles += 4;
        c.g[A2][0] = c.lb(c.g[A1][0] as u32);
        c.g[T0][0] = 0x1;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
    } else {
        c.cycles += 1;
    }
    c.cycles += 3;
    c.g[A3][0] = 0xff;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xff));
    loop {
        c.cycles += 7;
        c.sb(c.g[V0][0] as u32, c.g[T0][0]);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
        let more = (c.g[A3][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0041_8600);
        }
    }
    'done: {
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        if c.g[A2][0] == 0u64 {
            // Empty class: stop at the NUL.
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            break 'done;
        }
        c.cycles += 1;
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[T0][0] as u32));
        'outer: loop {
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            'inner: loop {
                c.cycles += 2;
                c.sb(c.g[V0][0] as u32, c.g[T0][0]);
                c.g[A3][0] = c.lb(c.g[A1][0] as u32);
                'char: loop {
                    {
                        c.cycles += 3;
                        c.g[V0][0] = 0x2d; // '-'
                        let dash = c.g[A3][0] == c.g[V0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        if !dash {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[A3][0] as i64) < 46i64) as u64;
                            let below_dash = c.g[V0][0] != 0u64;
                            c.g[V0][0] = 0x5d; // ']'
                            if below_dash {
                                // Below `-`: a lone member; NUL ends the
                                // format at `a1 - 1`, like an empty class.
                                c.cycles += 2;
                                c.g[A2][0] = c.g[A3][0];
                                if c.g[A3][0] == 0u64 {
                                    c.cycles += 2;
                                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                                    break 'done;
                                }
                                c.cycles += 2;
                                c.g[V0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                                if event_due(c) {
                                    deliver_event_here(c, 0x0041_8634);
                                }
                                continue 'inner;
                            }
                            c.cycles += 2;
                            c.g[A2][0] = c.g[A3][0];
                            if c.g[A3][0] == c.g[V0][0] {
                                // `]`: the class ends here.
                                c.cycles += 2;
                                c.g[V0][0] = c.g[A1][0];
                                break 'done;
                            }
                            c.cycles += 2;
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            if event_due(c) {
                                deliver_event_here(c, 0x0041_8634);
                            }
                            continue 'inner;
                        }
                    }
                    // A `-` opens a range: the bound follows.
                    'range: {
                        c.cycles += 4;
                        c.g[A3][0] = c.lb(c.g[A1][0] as u32);
                        c.g[V0][0] = 0x5d; // ']'
                        let closed = c.g[A3][0] == c.g[V0][0];
                        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
                        if !closed {
                            c.cycles += 2;
                            if c.g[V0][0] == 0u64 {
                                // Ordinary upper bound: consume it, fill.
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                break 'range;
                            }
                        }
                        // `]` (or an inverted range): the `-` was
                        // literal; restart the member loop from `-`.
                        c.cycles += 2;
                        c.g[A2][0] = 0x2d;
                        if event_due(c) {
                            deliver_event_here(c, 0x0041_8630);
                        }
                        continue 'outer;
                    }
                    loop {
                        c.cycles += 7;
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                        c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
                        let more = c.g[V1][0] != 0u64;
                        c.sb(c.g[V0][0] as u32, c.g[T0][0]);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0041_8698);
                        }
                    }
                    c.cycles += 2;
                    c.g[A3][0] = c.lb(c.g[A1][0] as u32);
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_863c);
                    }
                    continue 'char;
                }
            }
        }
    }
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
}

/// `int vsprintf(char *str, const char *fmt, va_list ap)` (0x4186c8):
/// the global-reent `sprintf` core: same stack `__sFILE` as `_sprintf_r`.
pub fn vsprintf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        c.g[V0][0] = 0x450000;
        c.g[V1][0] = 0x7fff0000;
        c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4944));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.g[A3][0] = c.g[A0][0];
        c.g[V1][0] |= 0xffff;
        c.g[V0][0] = 0x208;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V1][0]);
        c.g[A0][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.sh((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A3][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x54), c.g[T0][0]);
        c.g[RA][0] = sx(0x0041_8708);
        c.sw(c.g[SP][0] as u32, c.g[A3][0]);
        callee::VFPRINTF_DISPATCH.call(c, 0x0041_8708)?;
        c.g[V1][0] = c.lw(c.g[SP][0] as u32);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.sb(c.g[V1][0] as u32, 0u64);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = ra;
        Ok(())
    });
}

/// `int vsnprintf(char *str, size_t n, const char *fmt, va_list ap)`
/// (0x418720): bounded `vsprintf`. Returns -1 when the output is
/// truncated, and always NUL-terminates.
pub fn vsnprintf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.g[V0][0] = 0x450000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = c.g[A0][0];
        c.g[S0][0] = c.g[A1][0];
        c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4944));
        c.g[A1][0] = c.g[A2][0];
        c.g[V0][0] = 0x208;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
        c.g[A0][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[A2][0] = c.g[A3][0];
        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[S0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[S0][0]);
        c.sh((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[RA][0] = sx(0x0041_8768);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x54), c.g[T0][0]);
        callee::VFPRINTF_DISPATCH.call(c, 0x0041_8768)?;
        c.g[S0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
        if c.g[S0][0] != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.sb(c.g[A0][0] as u32, 0u64);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = ra;
        Ok(())
    });
}

/// `void abort(void)` (0x418790): `raise`-free abort: exit the thread with
/// status 1, else fall into `atof` (unreachable when `_exit` runs).
pub fn abort(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A0][0] = 0x1;
        call::call_ptr(c, 0x0010_01c8, 0x0041_87a0)?;
        c.pc = 0x0041_87a0;
        ee::s15::atof(c);
        Ok(())
    });
}

/// `double atof(const char *s)` (0x4187a0): `strtod` with a throwaway
/// end pointer; the double comes back in `f0`.
pub fn atof(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0041_87b0);
        c.g[A1][0] = 0u64;
        callee::STRTOD.call(c, 0x0041_87b0)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `int atoi(const char *s)` (0x4187c0): `strtol` base 10, truncated to 32
/// bits on return.
pub fn atoi(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[A1][0] = 0u64;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0041_87d4);
        c.g[A2][0] = 0xa;
        callee::STRTOL.call(c, 0x0041_87d4)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[V0][0] <<= 32;
        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `void *bsearch(const void *key, const void *base, size_t nmemb,
/// size_t size, int (*compar)(const void *, const void *))` (0x4187e8):
/// binary search of the sorted array at `base`. The comparator runs through
/// the function table. Returns the matching element, or NULL.
pub fn bsearch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = c.g[A2][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S3][0] = 0u64;
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S4][0] = c.g[A3][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S5][0] = c.g[T0][0];
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S6][0] = c.g[A1][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S7][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        let empty = c.g[S2][0] == 0u64;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        if !empty {
            c.cycles += 2;
            c.g[V0][0] = 0u64;
            if c.g[S4][0] == 0u64 {
                // Zero-sized elements: no match.
                c.cycles += 2;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            } else {
                c.cycles += 2;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                if c.g[S2][0] != 0u64 {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    loop {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[S7][0];
                        c.g[S0][0] = sx((c.g[V0][0] as u32) >> 1);
                        c.mult(0, 2, 16, 20);
                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                        let compar = c.g[S5][0] as u32;
                        c.g[RA][0] = sx(0x0041_8868);
                        c.g[A1][0] = c.g[S1][0];
                        call::call_ptr(c, compar, 0x0041_8868)?;
                        if (c.g[V0][0] as i64) < 0 {
                            c.cycles += 2;
                            c.g[S2][0] = c.g[S0][0];
                        } else {
                            c.cycles += 2;
                            let found = (c.g[V0][0] as i64) <= 0;
                            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            if found {
                                if back_edge_unwinds(c, 0x0041_883c) {
                                    return Ok(());
                                }
                                c.cycles += 2;
                                c.g[V0][0] = c.g[S1][0];
                                c.cycles += 1;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                break;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[S3][0] < c.g[S2][0]) as u64;
                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        if c.g[S3][0] >= c.g[S2][0] {
                            // Exhausted: no match.
                            c.cycles += 1;
                            c.g[V0][0] = 0u64;
                            c.cycles += 1;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                            break;
                        }
                        if back_edge_unwinds(c, 0x0041_8850) {
                            return Ok(());
                        }
                    }
                }
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = ra;
        Ok(())
    });
}

/// `void *malloc(size_t n)` (0x4188c0): lock the global heap, allocate
/// with `_malloc_r`, unlock. Returns the block (or NULL).
pub fn malloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = 0x450000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0041_88e0);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4944));
        callee::MALLOC_LOCK.call(c, 0x0041_88e0)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4944));
        c.g[RA][0] = sx(0x0041_88ec);
        c.g[A1][0] = c.g[S0][0];
        callee::MALLOC_R.call(c, 0x0041_88ec)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4944));
        c.g[RA][0] = sx(0x0041_88f8);
        c.g[S0][0] = c.g[V0][0];
        callee::MALLOC_UNLOCK.call(c, 0x0041_88f8)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `void free(void *p)` (0x418910): lock the global heap, release with
/// `_free_r`, unlock.
pub fn free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = 0x450000;
        c.g[S1][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0041_8930);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4944));
        callee::MALLOC_LOCK.call(c, 0x0041_8930)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4944));
        c.g[RA][0] = sx(0x0041_893c);
        c.g[A1][0] = c.g[S1][0];
        callee::FREE_R.call(c, 0x0041_893c)?;
        c.g[RA][0] = sx(0x0041_8944);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4944));
        callee::MALLOC_UNLOCK.call(c, 0x0041_8944)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `char *fcvt(double value, int ndigit, int *decpt, int *sign)`
/// (0x418d48): `fcvtbuf` with a NULL buffer (`t0`), so the digits land in
/// the reent buffer.
pub fn fcvt(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0041_8d58);
        c.g[T0][0] = 0u64;
        callee::FCVTBUF.call(c, 0x0041_8d58)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `char *fcvtf(float value, int ndigit, int *decpt, int *sign)`
/// (0x418d68): widen to double, then `fcvt`.
pub fn fcvtf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0041_8d8c);
        c.g[S2][0] = c.g[A2][0];
        callee::EXTENDSFDF2.call(c, 0x0041_8d8c)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0041_8da0);
        c.g[A0][0] = c.g[V0][0];
        callee::FCVT.call(c, 0x0041_8da0)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `char *gcvtf(float value, int ndigit, char *buf)` (0x418db8): widen
/// to double, then `gcvt`.
pub fn gcvtf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0041_8dd4);
        c.g[S1][0] = c.g[A1][0];
        callee::EXTENDSFDF2.call(c, 0x0041_8dd4)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0041_8de4);
        c.g[A0][0] = c.g[V0][0];
        callee::GCVT_DISPATCH.call(c, 0x0041_8de4)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `char *ecvt(double value, int ndigit, int *decpt, int *sign)`
/// (0x418df8): `ecvtbuf` with a NULL buffer (`t0`), so the digits land in
/// the reent buffer.
pub fn ecvt(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0041_8e08);
        c.g[T0][0] = 0u64;
        callee::ECVTBUF.call(c, 0x0041_8e08)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `char *ecvtf(float value, int ndigit, int *decpt, int *sign)`
/// (0x418e18): widen to double, then `ecvt`.
pub fn ecvtf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0041_8e3c);
        c.g[S2][0] = c.g[A2][0];
        callee::EXTENDSFDF2.call(c, 0x0041_8e3c)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0041_8e50);
        c.g[A0][0] = c.g[V0][0];
        callee::ECVT.call(c, 0x0041_8e50)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `char *gcvt(double value, int ndigit, char *buf)` (0x418e68): leading
/// `-` for negatives, then `_gcvt`; returns `buf`.
pub fn gcvt(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A2][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S3][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S2][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0041_8e98);
        c.g[A1][0] = 0u64;
        callee::CMPDF2.call(c, 0x0041_8e98)?;
        let nonneg = (c.g[V0][0] as i64) >= 0;
        c.g[V0][0] = 0x450000;
        if !nonneg {
            // Negative: stash `-`, shrink the precision.
            c.cycles += 5;
            c.g[V0][0] = 0x2d;
            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            c.sb(c.g[S1][0] as u32, c.g[V0][0]);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            c.g[V0][0] = 0x450000;
        }
        c.cycles += 16;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4944));
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[T0][0] = 0x67;
        c.g[RA][0] = sx(0x0041_8ed0);
        c.g[T1][0] = 0u64;
        callee::GCVT.call(c, 0x0041_8ed0)?;
        if c.g[V0][0] == 0 {
            c.g[S1][0] = 0u64;
        }
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = c.g[S1][0];
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = ra;
        Ok(())
    });
}

/// `double strtod(const char *s, char **end)` (0x41a7e8): reentrant
/// conversion through the global reent's `_strtod_r`.
pub fn strtod(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[V0][0] = c.g[A0][0];
        c.g[V1][0] = 0x450000;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[A2][0] = c.g[A1][0];
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4944));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0041_a808);
        c.g[A1][0] = c.g[V0][0];
        callee::STRTOD_R.call(c, 0x0041_a808)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `float strtof(const char *s, char **end)` (0x41a818): `strtod`
/// narrowed with `__truncdfsf2`.
pub fn strtof(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0041_a828);
        callee::STRTOD.call(c, 0x0041_a828)?;
        c.g[RA][0] = sx(0x0041_a830);
        c.g[A0][0] = c.g[V0][0];
        callee::TRUNCDFSF2.call(c, 0x0041_a830)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `unsigned long strtoul(const char *s, char **end, int base)`
/// (0x41aa50): reentrant conversion through the global reent.
pub fn strtoul(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[T0][0] = c.g[A0][0];
        c.g[V0][0] = 0x450000;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[V1][0] = c.g[A1][0];
        c.g[A3][0] = c.g[A2][0];
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4944));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A1][0] = c.g[T0][0];
        c.g[RA][0] = sx(0x0041_aa78);
        c.g[A2][0] = c.g[V1][0];
        callee::STRTOUL_R.call(c, 0x0041_aa78)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `size_t strcspn(const char *s, const char *reject)` (0x41ab08):
/// length of the initial segment of `s` with no bytes from `reject`.
pub fn strcspn(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.lb(c.g[A0][0] as u32);
    c.g[T0][0] = c.g[A0][0];
    c.g[A3][0] = c.lbu(c.g[A0][0] as u32);
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[T0][0] as u32));
        abi::ret(c);
        return;
    }
    loop {
        // Scan `reject` for the current `s` byte.
        c.cycles += 4;
        c.g[A2][0] = c.g[A1][0];
        c.g[V1][0] = c.lb(c.g[A2][0] as u32);
        c.g[V0][0] = sx((c.g[A3][0] as u32) << 24);
        let mut matched = false;
        if c.g[V1][0] == 0 {
            // Empty `reject`: no match possible; advance `s`.
            c.cycles += 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            if c.g[V0][0] == c.g[V1][0] {
                // Match on the first `reject` byte: the span ends here.
                c.g[V0][0] = c.lb(c.g[A2][0] as u32);
                matched = true;
            } else {
                c.cycles += 1;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                'scan: loop {
                    c.cycles += 3;
                    c.g[V1][0] = c.lb(c.g[A2][0] as u32);
                    if c.g[V1][0] == 0 {
                        // `reject` exhausted: advance `s`, re-scan.
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        break;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        if event_due(c) {
                            deliver_event_here(c, 0x0041_ab38);
                        }
                        continue 'scan;
                    }
                    // Match: the span ends here.
                    c.cycles += 1;
                    c.g[V0][0] = c.lb(c.g[A2][0] as u32);
                    matched = true;
                    break;
                }
            }
        }
        if matched {
            // (The matched `reject` byte is nonzero.)
            c.cycles += 2;
            break;
        }
        // No match at this position (`A0` already advanced on both paths
        // above): re-scan from the top with the next `s` byte.
        c.cycles += 3;
        c.g[V0][0] = c.lb(c.g[A0][0] as u32);
        c.g[A3][0] = c.lbu(c.g[A0][0] as u32);
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0041_ab18);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[T0][0] as u32));
    abi::ret(c);
}

/// `char *strpbrk(const char *s, const char *accept)` (0x41ac40):
/// first `s` byte that is in `accept`, or NULL.
pub fn strpbrk(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lb(c.g[A0][0] as u32);
    c.g[V1][0] = c.g[A1][0];
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0u64;
        abi::ret(c);
        return;
    }
    loop {
        // Try the current `s` byte against `accept`.
        let matched = 'tried: {
            c.cycles += 3;
            c.g[A2][0] = c.lb(c.g[V1][0] as u32);
            if c.g[A2][0] == 0 {
                // `accept` exhausted: advance `s`, re-try.
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                break 'tried false;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lb(c.g[A0][0] as u32);
            if c.g[V0][0] == c.g[A2][0] {
                // First-`accept`-byte match.
                c.g[V0][0] = c.lb(c.g[V1][0] as u32);
                break 'tried true;
            }
            c.cycles += 2;
            c.g[A2][0] = c.g[V0][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            'scan: loop {
                c.cycles += 3;
                c.g[V0][0] = c.lb(c.g[V1][0] as u32);
                if c.g[V0][0] == 0 {
                    // `accept` exhausted: advance `s`, re-try.
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    break 'tried false;
                }
                c.cycles += 4;
                if c.g[A2][0] != c.g[V0][0] {
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_ac78);
                    }
                    continue 'scan;
                }
                // Match.
                c.cycles += 1;
                c.g[V0][0] = c.lb(c.g[V1][0] as u32);
                break 'tried true;
            }
        };
        if matched {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                // A matched NUL is not a hit: advance `s`, re-try.
                c.cycles += 1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            } else {
                c.g[V1][0] = c.lb(c.g[V1][0] as u32);
                break;
            }
        }
        // Re-try with the advanced `s` (`A0` already moved on every path
        // above that falls through here).
        c.cycles += 3;
        c.g[V0][0] = c.lb(c.g[A0][0] as u32);
        if c.g[V0][0] == 0 {
            // `s` exhausted: no hit.
            c.cycles += 1;
            c.g[V1][0] = c.lb(c.g[V1][0] as u32);
            break;
        }
        c.g[V1][0] = c.g[A1][0];
        if event_due(c) {
            deliver_event_here(c, 0x0041_ac58);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = 0u64;
    if c.g[V1][0] != 0 {
        c.g[V0][0] = c.g[A0][0];
    }
    abi::ret(c);
}

/// `char *strrchr(const char *s, int c)` (0x41acc0): last occurrence
/// of `c` in `s` (the NUL terminator included in the search), or NULL.
pub fn strrchr(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    c.g[V0][0] = c.lb(c.g[A0][0] as u32);
    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
    c.g[A2][0] = 0u64;
    c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
    if c.g[V0][0] == 0 {
        // Empty string: the NUL itself matches only `c == 0`.
        c.cycles += 5;
        c.g[V1][0] = c.lb(c.g[A0][0] as u32);
        c.g[V0][0] = c.g[A0][0];
        c.g[V1][0] ^= c.g[A1][0];
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[A2][0];
        }
        abi::ret(c);
        return;
    }
    loop {
        c.cycles += 8;
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
        c.g[V0][0] ^= c.g[A1][0];
        if c.g[V0][0] == 0 {
            c.g[A2][0] = c.g[A0][0];
        }
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = c.lb(c.g[A0][0] as u32);
        c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0041_acd8);
        }
    }
    c.cycles += 5;
    c.g[V1][0] = c.lb(c.g[A0][0] as u32);
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] ^= c.g[A1][0];
    if c.g[V1][0] != 0 {
        c.g[V0][0] = c.g[A2][0];
    }
    abi::ret(c);
}

/// `size_t strspn(const char *s, const char *accept)` (0x41ad10):
/// length of the initial segment of `s` with only bytes from `accept`.
pub fn strspn(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.lb(c.g[A0][0] as u32);
    c.g[T0][0] = c.g[A0][0];
    // Delay slot of the empty check: set on both paths.
    c.g[A3][0] = c.lbu(c.g[A0][0] as u32);
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[T0][0] as u32));
        abi::ret(c);
        return;
    }
    'outer: loop {
        c.cycles += 4;
        c.g[A2][0] = c.g[A1][0];
        c.g[V1][0] = c.lb(c.g[A2][0] as u32);
        c.g[V0][0] = sx((c.g[A3][0] as u32) << 24);
        if c.g[V1][0] == 0 {
            // Empty `accept`: no byte can match.
            break;
        }
        c.cycles += 3;
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
        if c.g[V0][0] == c.g[V1][0] {
            // Match on the first `accept` byte: span continues.
            c.g[V0][0] = c.lb(c.g[A2][0] as u32);
        } else {
            c.cycles += 1;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            loop {
                c.cycles += 3;
                c.g[V1][0] = c.lb(c.g[A2][0] as u32);
                if c.g[V1][0] == 0 {
                    // End of `accept` with no match: the span ends here.
                    break 'outer;
                }
                c.cycles += 4;
                c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                if c.g[V0][0] == c.g[V1][0] {
                    // Match: the span continues past this byte.
                    c.cycles += 1;
                    c.g[V0][0] = c.lb(c.g[A2][0] as u32);
                    break;
                }
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                if event_due(c) {
                    deliver_event_here(c, 0x0041_ad40);
                }
            }
        }
        // Matched: advance `s`, re-scan from the top with the new byte.
        // (The matched `accept` byte is nonzero, so the `V0 == 0` exit
        // below is dead; kept for fidelity with the generated body.)
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            break;
        }
        c.cycles += 4;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = c.lb(c.g[A0][0] as u32);
        c.g[A3][0] = c.lbu(c.g[A0][0] as u32);
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0041_ad20);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[T0][0] as u32));
    abi::ret(c);
}

/// `char *strstr(const char *haystack, const char *needle)` (0x41ad80):
/// first occurrence of `needle` in `haystack`, or NULL.
/// An empty `needle` matches at `haystack`.
pub fn strstr(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lb(c.g[A0][0] as u32);
    c.g[V1][0] = c.lb(c.g[A1][0] as u32);
    if c.g[V0][0] == 0 {
        // Empty haystack: only an empty needle matches.
        c.cycles += 3;
        c.g[V0][0] = 0u64;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = c.g[A0][0];
        }
        abi::ret(c);
        return;
    }
    loop {
        // Try a match at the current `haystack` position.
        let found = 'tried: {
            c.cycles += 5;
            c.g[A3][0] = 0u64;
            if c.g[V1][0] == 0 {
                // Empty needle matches here.
                break 'tried true;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lb(c.g[A0][0] as u32);
            if c.g[V1][0] != c.g[V0][0] {
                // First-byte mismatch: advance `haystack`.
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                break 'tried false;
            }
            c.cycles += 2;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            'cmp: loop {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[A2][0] = c.lb(c.g[V0][0] as u32);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                if c.g[A2][0] == 0 {
                    // Needle exhausted: match at the current position.
                    break 'tried true;
                }
                c.cycles += 3;
                c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                if c.g[A2][0] == c.g[V1][0] {
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_adc8);
                    }
                    continue 'cmp;
                }
                c.cycles += 1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                break 'tried false;
            }
        };
        if found {
            break;
        }
        c.cycles += 3;
        c.g[V0][0] = c.lb(c.g[A0][0] as u32);
        if c.g[V0][0] == 0 {
            // Haystack exhausted with no match.
            c.cycles += 2;
            c.g[V0][0] = 0u64;
            abi::ret(c);
            return;
        }
        c.g[V1][0] = c.lb(c.g[A1][0] as u32);
        if event_due(c) {
            deliver_event_here(c, 0x0041_ada0);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `int vfiprintf_BufferedWrite(int fd, const char *buf, size_t len, int mode)`
/// (0x41c938): same algorithm as `vfprintf_BufferedWrite` on `vfiprintf`'s
/// static buffer.
pub fn vfiprintf_buffered_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.g[V0][0] = 0x1;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S3][0] = c.g[A2][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S4][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S6][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        if c.g[A3][0] == c.g[V0][0] {
            // Direct `_write`, bypassing the buffer.
            c.cycles += 10;
            c.g[S0][0] = 0x450000;
            c.g[S1][0] = sx(0x530000u32.wrapping_sub(0x50e8));
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4ce0));
            c.g[RA][0] = sx(0x0041_c988);
            c.g[A1][0] = c.g[S1][0];
            callee::WRITE.call(c, 0x0041_c988)?;
            c.g[V1][0] = 0x450000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4ce0), 0u64);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4ce4), c.g[S1][0]);
        } else {
            c.cycles += 2;
            c.g[S0][0] = 0u64;
            if c.g[S3][0] != 0 {
                c.cycles += 6;
                c.g[V0][0] = 0x530000;
                c.g[S1][0] = 0x450000;
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x50e8));
                c.g[S2][0] = 0x450000;
                c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S0][0] as u32));
                let mut flushed_zero = false;
                loop {
                    c.cycles += 11;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4ce4));
                    c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                    c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4ce0));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4ce4));
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4ce0), c.g[A2][0]);
                    // Gen keeps the `slti` result in `V0` (dead: the
                    // flush below overwrites it, the loop exit ignores it).
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 128) as u64;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4ce4), c.g[V1][0]);
                    if c.g[V0][0] == 0 {
                        // Buffer full: flush through `_write`.
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S4][0];
                        c.g[RA][0] = sx(0x0041_c9f8);
                        c.g[A1][0] = c.g[S5][0];
                        callee::WRITE.call(c, 0x0041_c9f8)?;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x4ce4), c.g[S5][0]);
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x4ce0), 0u64);
                        if c.g[V0][0] == 0 {
                            // Nothing written: return 0.
                            if back_edge_unwinds(c, 0x0041_c998) {
                                return Ok(());
                            }
                            c.cycles += 2;
                            c.g[V0][0] = 0u64;
                            flushed_zero = true;
                            break;
                        }
                    }
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = (c.g[S0][0] < c.g[S3][0]) as u64;
                    c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    if back_edge_unwinds(c, 0x0041_c9c0) {
                        return Ok(());
                    }
                }
                if !flushed_zero {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S0][0];
                }
            } else {
                // Empty write: return 0.
                c.cycles += 1;
                c.g[V0][0] = c.g[S0][0];
            }
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = ra;
        Ok(())
    });
}

/// `int vfiprintf(FILE *fp, const char *fmt, va_list ap)` (0x41ca40):
/// `vfprintf` on `fp` with the reent stored in the `FILE`.
pub fn vfiprintf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[V0][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[V1][0] = c.g[A1][0];
        c.g[A3][0] = c.g[A2][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x54));
        c.g[RA][0] = sx(0x0041_ca64);
        c.g[A2][0] = c.g[V1][0];
        callee::VFIPRINTF_R.call(c, 0x0041_ca64)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `void *malloc_extend_top(reent *r, size_t bytes)` (0x41d990): grow
/// the heap via `_sbrk_r` and link the new top chunk. Returns the old
/// break, or -1 when the `sbrk` fails.
pub fn malloc_extend_top(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.g[V0][0] = 0x450000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[A2][0] = 0x450000;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[A1][0] = c.g[A1][0] << 32;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[A1][0] = c.g[A1][0] >> 32;
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S0][0] = 0xffffffffffff0000u64;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] |= 0xffffu64;
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S6][0] = 0x450000;
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S7][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[FP][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4ce8));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[V0][0] = 0xffffffffffff0000u64;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[V0][0] |= 0xfffcu64;
        c.g[S4][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x50f8));
        c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
        c.g[A1][0] = c.g[A1][0].wrapping_add(c.g[V1][0]);
        c.g[S3][0] = c.g[A2][0] & c.g[V0][0];
        c.g[A1][0] = c.g[A1][0].wrapping_add(0x10);
        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x5100));
        c.g[S2][0] = c.g[A1][0] << 32;
        c.g[S2][0] = ((c.g[S2][0] as i64) >> 32) as u64;
        c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S3][0] as u32));
        if c.g[V0][0] != c.g[S0][0] {
            // Round the request up to a page.
            c.cycles += 7;
            c.g[V0][0] = c.g[S2][0] << 32;
            c.g[V1][0] = 0xfffffffffffff000u64;
            c.g[V0][0] >>= 32;
            c.g[V0][0] = c.g[V0][0].wrapping_add(0xfff);
            c.g[V0][0] &= c.g[V1][0];
            c.g[S2][0] = c.g[V0][0] << 32;
            c.g[S2][0] = ((c.g[S2][0] as i64) >> 32) as u64;
        }
        // Grow the break.
        c.cycles += 6;
        c.g[A0][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x0041_da3c);
        c.g[A1][0] = c.g[S2][0];
        callee::SBRK_R.call(c, 0x0041_da3c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = (c.g[S1][0] < c.g[S5][0]) as u64;
        // The shared tail owes a +1 unless a fail path already gave up
        // (those still restore r16 below).
        let mut plus1 = true;
        let mut done_early = false;
        if c.g[S1][0] == c.g[S0][0] {
            // `sbrk` failed: return -1.
        } else {
            c.cycles += 2;
            let past_goal = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x450000;
            let mut proceed = past_goal;
            if !past_goal {
                c.cycles += 2;
                if c.g[S4][0] == c.g[FP][0] {
                    c.cycles += 1;
                    c.g[V0][0] = 0x450000;
                    proceed = true;
                } else {
                    // Top moved under us: give up, return -1.
                    plus1 = false;
                }
            }
            if proceed {
                // Break already past the goal, or top intact: record it.
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5118));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                let moved = c.g[S1][0] != c.g[S5][0];
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5118), c.g[V1][0]);
                // `S1 == S5` skips straight to the footprint below.
                let mut direct_footprint = false;
                if moved {
                    // (b_aa0) Fix the max-footprint marker if needed.
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x5100));
                    let max_unset = c.g[V0][0] == c.g[S0][0];
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[S5][0] as u32));
                    if max_unset {
                        c.cycles += 2;
                        c.sw((c.g[S6][0] as u32).wrapping_add(0x5100), c.g[S1][0]);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = 0x450000;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x5118), c.g[V0][0]);
                    }
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S3][0] as u32));
                    c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[A1][0] | 0x1;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    direct_footprint = true;
                }
                if !direct_footprint {
                    // (b_ac0) Align the new top.
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.g[V0][0] & 0xf;
                    c.g[V0][0] = 0x10;
                    if c.g[A1][0] != 0 {
                        c.cycles += 3;
                        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    } else {
                        c.cycles += 1;
                        c.g[S0][0] = 0u64;
                    }
                    // Second `sbrk` for the alignment pad.
                    c.cycles += 15;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[V1][0] = 0x1000;
                    c.g[V0][0] &= 0xfff;
                    c.g[A0][0] = c.g[S7][0];
                    c.g[V1][0] = c.g[V1][0].wrapping_sub(c.g[V0][0]);
                    c.g[V1][0] = c.g[V1][0] << 32;
                    c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[RA][0] = sx(0x0041_dae8);
                    c.g[A1][0] = c.g[S0][0];
                    callee::SBRK_R.call(c, 0x0041_dae8)?;
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = 0xffffffffffff0000u64;
                    c.g[V0][0] |= 0xffffu64;
                    if c.g[V1][0] == c.g[V0][0] {
                        // Pad `sbrk` failed: return -1. Gen restores `s0`
                        // and goes straight to the epilogue here.
                        plus1 = false;
                        done_early = true;
                    } else {
                        c.cycles += 13;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                        c.g[A0][0] = 0x450000;
                        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5118));
                        c.g[V1][0] = 0x450000;
                        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4ce8));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        c.g[V1][0] = c.g[A1][0] | 0x1;
                        c.g[A1][0] = 0x450000;
                        c.sw((c.g[A1][0] as u32).wrapping_add(0x5118), c.g[V0][0]);
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[S1][0]);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                        // Small top already: set the bit and return it.
                        // Otherwise stitch the new top chunk in.
                        if !done_early && c.g[S4][0] != c.g[A0][0] {
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[S3][0] < 0x10u64) as u64;
                            let big = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x1;
                            if big {
                                // (fall through to the stitch below)
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                // Gen `break 'b_41dbc4` lands on the shared
                                // `+1` tail: keep `plus1` set.
                                done_early = true;
                            }
                        }
                        if !done_early && c.g[S4][0] != c.g[A0][0] {
                            // Stitch the new top chunk in.
                            c.cycles += 14;
                            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = 0xffffffffffff0000u64;
                            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0xc));
                            c.g[V0][0] |= 0xfff0u64;
                            c.g[S3][0] = c.g[A0][0] & c.g[V0][0];
                            c.g[V1][0] &= 0x1;
                            c.g[V1][0] |= c.g[S3][0];
                            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S3][0] as u32));
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                            c.g[V0][0] = 0x5;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            c.g[V1][0] = (c.g[S3][0] < 0x10u64) as u64;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            if c.g[V1][0] == 0 {
                                c.cycles += 3;
                                c.g[A0][0] = c.g[S7][0];
                                c.g[RA][0] = sx(0x0041_db8c);
                                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x8));
                                callee::FREE_R.call(c, 0x0041_db8c)?;
                            }
                        } // close the stitch guard
                    } // close the pad-ok `else`
                } // close `if !direct_footprint`
                if direct_footprint || !done_early {
                    // (b_a8) Record the new footprint highs.
                    c.cycles += 7;
                    c.g[V0][0] = 0x450000;
                    c.g[A0][0] = 0x450000;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5118));
                    c.g[V0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x5108));
                    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    if c.g[V0][0] != 0 {
                        c.sd((c.g[A0][0] as u32).wrapping_add(0x5108), c.g[V1][0]);
                    }
                    c.cycles += 7;
                    c.g[A0][0] = 0x450000;
                    c.g[A1][0] = 0x450000;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x5118));
                    c.g[V0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x5110));
                    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    if c.g[V0][0] != 0 {
                        c.sd((c.g[A0][0] as u32).wrapping_add(0x5110), c.g[V1][0]);
                    }
                } // close the footprint guard
            } // close `if proceed`
        } // close first-`sbrk` `else`
        if plus1 {
            c.cycles += 1;
        }
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = ra;
        Ok(())
    });
}

/// `int malloc_trim(reent *r, size_t pad)` (0x41e638): release free top
/// memory back via `_sbrk_r`. Returns 1 if anything was released.
pub fn malloc_trim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0041_e660);
        c.g[S0][0] = c.g[A1][0];
        callee::MALLOC_LOCK.call(c, 0x0041_e660)?;
        c.g[V0][0] = 0x450000;
        c.g[V1][0] = 0xffffffffffff0000u64;
        c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4ce8));
        c.g[V1][0] |= 0xfffcu64;
        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
        c.g[S0][0] = (c.g[S0][0] << 32) >> 32;
        c.g[A1][0] = 0x1000;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] &= c.g[V1][0];
        c.g[V0][0] <<= 32;
        c.g[S1][0] = c.g[V0][0] >> 32;
        c.g[S0][0] = c.g[S1][0].wrapping_sub(c.g[S0][0]);
        c.g[RA][0] = sx(0x0041_e69c);
        c.g[A0][0] = c.g[S0][0].wrapping_add(0xfef);
        callee::UDIVDI3.call(c, 0x0041_e69c)?;
        c.g[A0][0] = c.g[V0][0].wrapping_sub(0x1);
        c.g[RA][0] = sx(0x0041_e6a8);
        c.g[A1][0] = 0x1000;
        callee::MULDI3.call(c, 0x0041_e6a8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = ((c.g[S0][0] as i64) < 4096i64) as u64;
        c.g[A0][0] = c.g[S2][0];
        if c.g[V0][0] == 0 {
            c.cycles += 8;
            c.g[RA][0] = sx(0x0041_e6c0);
            c.g[A1][0] = 0u64;
            callee::SBRK_R.call(c, 0x0041_e6c0)?;
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.g[S1][0] << 32;
            c.g[A0][0] = ((c.g[A0][0] as i64) >> 32) as u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A0][0] = c.g[S2][0];
            if c.g[V0][0] != c.g[V1][0] {
                // Heap moved under us: unlock, return 0.
                c.cycles += 4;
                c.g[RA][0] = sx(0x0041_e748);
                c.g[A0][0] = c.g[S2][0];
                callee::MALLOC_UNLOCK.call(c, 0x0041_e748)?;
                c.g[V0][0] = 0u64;
            } else {
                c.cycles += 8;
                c.g[S3][0] = c.g[S0][0] << 32;
                c.g[S3][0] = ((c.g[S3][0] as i64) >> 32) as u64;
                c.g[RA][0] = sx(0x0041_e6e8);
                c.g[A1][0] = sx(0u32.wrapping_sub(c.g[S3][0] as u32));
                callee::SBRK_R.call(c, 0x0041_e6e8)?;
                c.g[V1][0] = 0xffffffffffffffffu64;
                let sbrk_ok = c.g[V0][0] != c.g[V1][0];
                c.g[V1][0] = 0x1;
                if sbrk_ok {
                    // Released: fix the top size, unlock, return 1.
                    c.cycles += 13;
                    c.g[V0][0] = c.g[S1][0].wrapping_sub(c.g[S0][0]);
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] |= c.g[V1][0];
                    c.g[V0][0] <<= 32;
                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                    c.g[V1][0] = 0x450000;
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5118));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S3][0] as u32));
                    c.g[RA][0] = sx(0x0041_e780);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x5118), c.g[V0][0]);
                    callee::MALLOC_UNLOCK.call(c, 0x0041_e780)?;
                    c.g[V0][0] = 0x1;
                } else {
                    c.cycles += 9;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x0041_e704);
                    c.g[A0][0] = c.g[S2][0];
                    callee::SBRK_R.call(c, 0x0041_e704)?;
                    c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 16i64) as u64;
                    c.g[A0][0] = 0x450000;
                    if c.g[V0][0] == 0 {
                        c.cycles += 9;
                        c.g[V0][0] = 0x1;
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5100));
                        c.g[V0][0] = c.g[S1][0] | c.g[V0][0];
                        c.g[A0][0] = 0x450000;
                        c.g[V0][0] <<= 32;
                        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x5118), c.g[V1][0]);
                        c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    }
                    // Fall through to the unlock-and-return-0 tail.
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0041_e748);
                    c.g[A0][0] = c.g[S2][0];
                    callee::MALLOC_UNLOCK.call(c, 0x0041_e748)?;
                    c.g[V0][0] = 0u64;
                }
            }
        } else {
            // Nothing worth releasing: unlock, return 0.
            c.cycles += 4;
            c.g[RA][0] = sx(0x0041_e748);
            c.g[A0][0] = c.g[S2][0];
            callee::MALLOC_UNLOCK.call(c, 0x0041_e748)?;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 8;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = ra;
        Ok(())
    });
}

/// `long strtol(const char *s, char **end, int base)` (0x41e9e0):
/// reentrant conversion through the global reent.
pub fn strtol(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[T0][0] = c.g[A0][0];
        c.g[V0][0] = 0x450000;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[V1][0] = c.g[A1][0];
        c.g[A3][0] = c.g[A2][0];
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4944));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A1][0] = c.g[T0][0];
        c.g[RA][0] = sx(0x0041_ea08);
        c.g[A2][0] = c.g[V1][0];
        callee::STRTOL_R.call(c, 0x0041_ea08)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `struct lconv *localeconv(void)` (0x41ec48): the global reent's
/// `lconv` through `_localeconv_r`.
pub fn localeconv(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V0][0] = 0x450000;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4944));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0041_ec60);
        callee::LOCALECONV_R.call(c, 0x0041_ec60)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `void *memchr(const void *s, int c, size_t n)` (0x41ec70): first
/// `c` (as a byte) in the first `n` bytes of `s`, or NULL. Scans 16
/// bytes at a time with MMI ops when `s` is aligned and `n >= 16`.
pub fn memchr(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = (c.g[A2][0] < 0x10u64) as u64;
    c.g[A1][0] &= 0xff;
    if c.g[V0][0] != 0 {
        // Short or unaligned: byte loop below.
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.g[A0][0] & 0xf;
        c.g[A3][0] = c.g[A0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 20;
            c.g[V1][0] = c.g[A1][0] << 8;
            c.g[V0][0] = 0x8080u64 << 16;
            c.g[V0][0] |= 0x8080;
            c.g[V0][0] <<= 16;
            c.g[V0][0] |= 0x8080;
            c.g[V0][0] <<= 9;
            c.g[V0][0] |= 0x101;
            c.g[T2][0] = c.g[V1][0].wrapping_add(c.g[A1][0]);
            c.g[V1][0] = 0x8080u64 << 16;
            c.g[V1][0] |= 0x8080;
            c.g[V1][0] <<= 16;
            c.g[V1][0] |= 0x8080;
            c.g[V1][0] <<= 16;
            c.g[V1][0] |= 0x8080;
            c.pcpyh(8, 10);
            c.pcpyld(9, 8, 8);
            c.g[A0][0] = c.g[V0][0];
            c.pcpyld(8, 3, 3);
            loop {
                c.cycles += 11;
                c.lq(2, c.g[A3][0] as u32);
                c.pxor(2, 2, 9);
                c.pcpyld(10, 4, 4);
                c.pnor(3, 0, 2);
                c.psubb(2, 2, 10);
                c.pand(2, 2, 3);
                c.pand(2, 2, 8);
                c.pcpyud(3, 2, 9);
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = c.g[A3][0];
                    break;
                }
                c.cycles += 4;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x10));
                c.g[V0][0] = (c.g[A2][0] < 0x10u64) as u64;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10));
                if c.g[V0][0] == 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_ecd8);
                    }
                    continue;
                }
                c.cycles += 1;
                c.g[A0][0] = c.g[A3][0];
                break;
            }
        }
    }
    // Byte loop over the head, tail, or whole range.
    c.cycles += 5;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
    if (c.g[A2][0] as i64) == -1 {
        c.cycles += 2;
        c.g[V0][0] = 0u64;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V1][0] = 0xffffffffffff0000u64 | 0xffffu64;
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
        if c.g[V0][0] == c.g[A1][0] {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            abi::ret(c);
            return;
        }
        c.cycles += 4;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        if c.g[A2][0] == c.g[V1][0] {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0041_ed34);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    abi::ret(c);
}

/// `int fflush(FILE *fp)` (0x41fe30): flush `fp`, or every file via
/// `_fwalk` when `fp` is NULL. Returns 0, or EOF when the write fails.
pub fn fflush(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        if c.g[S1][0] == 0 {
            // Flush every file.
            c.cycles += 7;
            c.g[V0][0] = 0x450000;
            c.g[A1][0] = sx(0x420000u32.wrapping_sub(0x1d0));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4944));
            c.g[RA][0] = sx(0x0041_fe60);
            callee::FWALK.call(c, 0x0041_fe60)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        } else {
            // Flush one file: wire its reent, then its write buffer.
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
            if c.g[V0][0] == 0 {
                c.cycles += 5;
                c.g[V0][0] = 0x450000;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4944));
                c.sw((c.g[S1][0] as u32).wrapping_add(0x54), c.g[V1][0]);
            }
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x38));
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0041_fe9c);
                callee::SINIT.call(c, 0x0041_fe9c)?;
            } else {
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
            }
            c.g[V1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xc));
            c.cycles += 3;
            c.g[V0][0] = 0u64;
            if c.g[V1][0] & 0x8 == 0 {
                // Nothing buffered for writing.
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            } else {
                c.cycles += 3;
                c.g[S2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                if c.g[S2][0] == 0 {
                    // Nothing to write: short epilogue (no `ff24`).
                    c.cycles += 2;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V1][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] &= 0x3;
                    c.sw(c.g[S1][0] as u32, c.g[S2][0]);
                    c.g[A0][0] = 0u64;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S2][0] as u32));
                    if c.g[V1][0] == 0 {
                        c.cycles += 1;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    }
                    c.cycles += 2;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    let mut failed = false;
                    if (c.g[S0][0] as i64) > 0 {
                        c.cycles += 1;
                        loop {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                            c.g[A2][0] = c.g[S0][0];
                            let target = c.g[V0][0] as u32;
                            call::call_ptr(c, target, 0x0041_ff0c)?;
                            c.g[V1][0] = c.g[V0][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                            if (c.g[V1][0] as i64) > 0 {
                                c.cycles += 2;
                                c.g[S2][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                if (c.g[S0][0] as i64) > 0 {
                                    if back_edge_unwinds(c, 0x0041_fef8) {
                                        return Ok(());
                                    }
                                    continue;
                                }
                                break;
                            }
                            if back_edge_unwinds(c, 0x0041_fec0) {
                                return Ok(());
                            }
                            // Write error: mark `__SERR`, return EOF.
                            c.cycles += 5;
                            c.g[V1][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = 0xffffffffffffffff;
                            c.g[V1][0] |= 0x40;
                            c.sh((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                            failed = true;
                            break;
                        }
                    }
                    if !failed {
                        c.cycles += 1;
                        c.g[V0][0] = 0u64;
                    }
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                }
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `FILE *std(FILE *fp, int flags, int fd, reent *r)` (0x41ff40): wire a
/// stock `FILE` (`stdin`/`stdout`/`stderr` shape) at `fp`.
pub fn std(c: &mut Cpu) {
    c.cycles += 22;
    c.g[V0][0] = sx(0x410000u32.wrapping_add(0x7948));
    c.g[V1][0] = sx(0x410000u32.wrapping_add(0x79b0));
    c.g[T0][0] = sx(0x410000u32.wrapping_add(0x7a30));
    c.g[T1][0] = sx(0x410000u32.wrapping_add(0x7a98));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x54), c.g[A3][0]);
    c.sh((c.g[A0][0] as u32).wrapping_add(0xc), c.g[A1][0]);
    c.sh((c.g[A0][0] as u32).wrapping_add(0xe), c.g[A2][0]);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x28), c.g[T0][0]);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x2c), c.g[T1][0]);
    c.sw(c.g[A0][0] as u32, 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x10), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x18), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
    abi::ret(c);
}

/// `struct __sFileExt *__sfmoreglue(reent *r, int n)` (0x41ff98):
/// allocate a glue block with `n` more `FILE`s. Returns the block, or
/// NULL when the allocation fails.
pub fn sfmoreglue(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[V0][0] = 0x58;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A1][0];
        c.mult(0, 18, 17, 2);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0041_ffc0);
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
        callee::MALLOC_R.call(c, 0x0041_ffc0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        if c.g[S0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = 0u64;
        } else {
            c.cycles += 9;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
            c.sw(c.g[S0][0] as u32, 0u64);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S2][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.g[RA][0] = sx(0x0041_ffe8);
            c.g[A1][0] = 0u64;
            callee::MEMSET.call(c, 0x0041_ffe8)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `FILE *__sfp(reent *r)` (0x420010): find a free `FILE` slot,
/// extending the glue via `__sfmoreglue` (and `__sinit` on first use).
/// Returns the slot, or NULL + `ENOMEM` when extension fails.
pub fn sfp(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1d8));
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0042_0038);
            callee::SINIT.call(c, 0x0042_0038)?;
        }
        loop {
            // Scan one glue block for a closed (`_flags == 0`) slot.
            c.cycles += 4;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            let mut found = false;
            if (c.g[V1][0] as i64) >= 0 {
                // Walk the slots of this block.
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    if c.g[V0][0] == 0 {
                        found = true;
                        break;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x58));
                    if (c.g[V1][0] as i64) < 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0042_0048);
                    }
                }
            }
            if found {
                // Claim the free slot.
                c.cycles += 16;
                c.g[V0][0] = 0x1;
                c.g[V1][0] = 0xffffffffffffffff;
                c.sh((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.sh((c.g[A0][0] as u32).wrapping_add(0xe), c.g[V1][0]);
                c.g[V0][0] = c.g[A0][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x54), c.g[S1][0]);
                c.sw(c.g[A0][0] as u32, 0u64);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x10), 0u64);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x14), 0u64);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x18), 0u64);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x30), 0u64);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x34), 0u64);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x44), 0u64);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x48), 0u64);
                break;
            }
            // No free slot in this block: next glue, or extend it.
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            if c.g[V0][0] != 0 {
                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0042_0038);
                }
                continue;
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0042_007c);
            c.g[A1][0] = 0x4;
            callee::SFMOREGLUE.call(c, 0x0042_007c)?;
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            if c.g[V0][0] == 0 {
                // Extension failed: `ENOMEM`, NULL out.
                c.cycles += 4;
                c.g[V1][0] = 0xc;
                c.g[V0][0] = 0u64;
                c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                break;
            }
            c.cycles += 2;
            c.g[S0][0] = c.lw(c.g[S0][0] as u32);
            if event_due(c) {
                deliver_event_here(c, 0x0042_0038);
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `void __sinit(reent *r)` (0x4200f0): wire up `stdin`/`stdout`/`stderr`
/// via `std`.
pub fn sinit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V0][0] = sx(0x410000u32.wrapping_add(0x6fc0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[V1][0] = 0x1;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1e4));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x38), c.g[V1][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = 0x4;
        c.g[RA][0] = sx(0x0042_0130);
        c.g[A2][0] = 0u64;
        callee::STD.call(c, 0x0042_0130)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x23c));
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = 0x9;
        c.g[RA][0] = sx(0x0042_0144);
        c.g[A2][0] = 0x1;
        callee::STD.call(c, 0x0042_0144)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x294));
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = 0xa;
        c.g[RA][0] = sx(0x0042_0158);
        c.g[A2][0] = 0x2;
        callee::STD.call(c, 0x0042_0158)?;
        c.g[V0][0] = 0x3;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1e0), c.g[S1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1dc), c.g[V0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1d8), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `size_t fread_bcopy(void *dst, const void *src, size_t n)` (0x420228):
/// copy `n` bytes from the buffer cursor, bumping both.
pub fn fread_bcopy(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
    if c.g[A2][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[A2][0] = 0xffffffffffff0000u64 | 0xffffu64;
    loop {
        c.cycles += 7;
        c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
        c.sb(c.g[A0][0] as u32, c.g[V0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        if c.g[V1][0] == c.g[A2][0] {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0042_0238);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `size_t fread(void *dst, size_t size, size_t nmemb, FILE *fp)`
/// (0x420260): copy buffered bytes, refilling via `__srefill` until the
/// request is met or the refill fails. Returns full items, or 0/partial.
pub fn fread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S4][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S6][0] = c.g[A2][0];
        c.mult(0, 19, 22, 20);
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S1][0] = c.g[A3][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        // NOTE: `mult` above wrote the total into `S3`.
        let mut direct_epilogue = false;
        if c.g[S3][0] == 0 {
            // Empty read: return 0.
            c.cycles += 2;
            c.g[V0][0] = 0u64;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            if (c.g[V0][0] as i64) < 0 {
                c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
            }
            c.cycles += 5;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            c.g[S2][0] = c.g[A0][0];
            c.g[V0][0] = (c.g[S0][0] < c.g[S3][0]) as u64;
            c.g[S5][0] = c.g[S3][0];
            if c.g[V0][0] == 0 {
                // Everything is buffered: single copy below.
            } else {
                // Copy the buffered part, then refill and repeat.
                c.cycles += 2;
                c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                loop {
                    c.cycles += 12;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0042_02f0);
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                    callee::FREAD_BCOPY.call(c, 0x0042_02f0)?;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[RA][0] = sx(0x0042_0308);
                    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                    callee::SREFILL.call(c, 0x0042_0308)?;
                    let failed = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_sub(c.g[S3][0] as u32));
                    if failed {
                        // Refill failed: return completed items.
                        if back_edge_unwinds(c, 0x0042_02a0) {
                            return Ok(());
                        }
                        c.cycles += 2;
                        c.cycles += 4;
                        c.divu(0, 2, 20);
                        c.g[V0][0] = c.lo[0];
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                        direct_epilogue = true;
                        break;
                    }
                    c.cycles += 4;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = (c.g[S0][0] < c.g[S3][0]) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                    if back_edge_unwinds(c, 0x0042_02e0) {
                        return Ok(());
                    }
                }
            }
            if !direct_epilogue {
                // Copy the (remaining) buffered bytes, update the cursor.
                c.cycles += 11;
                c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0042_0330);
                c.g[A2][0] = c.g[S3][0];
                callee::FREAD_BCOPY.call(c, 0x0042_0330)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[S6][0];
                c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[S3][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                c.sw(c.g[S1][0] as u32, c.g[V1][0]);
            }
        }
        if !direct_epilogue {
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        }
        c.cycles += 9;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = ra;
        Ok(())
    });
}

/// `int ungetc(int c, FILE *fp)` (0x420378): push `c` back onto
/// `fp` (or flush and buffer it), returning `c`, or `EOF` when the
/// stream cannot take it.
pub fn ungetc(c: &mut Cpu) {
    guest(c, |c| {
        'b_4204f8: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[V0][0] = 0xffffffffffffffff;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A1][0];
            c.g[S1][0] = c.g[A0][0];
            let t = c.g[S1][0] == c.g[V0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            if t {
                break 'b_4204f8;
            } else {
                'b_4203b8: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                        break 'b_4203b8;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = 0x450000;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4944));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V1][0]);
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                        break 'b_4203b8;
                    }
                }
                'b_4203d0: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x38));
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
                        break 'b_4203d0;
                    } else {
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x004203cc);
                        callee::SINIT.call(c, 0x0042_03cc)?;
                        c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
                        break 'b_4203d0;
                    }
                }
                'b_420480: {
                    'b_420460: {
                        'b_420458: {
                            'b_420424: {
                                c.cycles += 4;
                                c.g[V1][0] = c.g[V0][0] & 0xffdf;
                                c.g[V0][0] = c.g[V1][0] & 0x4;
                                let t = c.g[V0][0] != 0u64;
                                c.sh((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                                if t {
                                    break 'b_420424;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.g[V1][0] & 0x10;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.g[V1][0] & 0x8;
                                    if t {
                                        break 'b_420458;
                                    } else {
                                        'b_42041c: {
                                            c.cycles += 2;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[V0][0] =
                                                    c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
                                                break 'b_42041c;
                                            } else {
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x004203fc);
                                                c.g[A0][0] = c.g[S0][0];
                                                callee::FFLUSH.call(c, 0x0042_03fc)?;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] = 0xffffffffffffffff;
                                                if t {
                                                    break 'b_4204f8;
                                                } else {
                                                    c.cycles += 6;
                                                    c.g[V0][0] = c
                                                        .lhu((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x8),
                                                        0u64,
                                                    );
                                                    c.g[V0][0] = c.g[V0][0] & 0xfff7;
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x18),
                                                        0u64,
                                                    );
                                                    c.sh(
                                                        (c.g[S0][0] as u32).wrapping_add(0xc),
                                                        c.g[V0][0],
                                                    );
                                                    c.g[V0][0] = c
                                                        .lhu((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    break 'b_42041c;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        c.g[V0][0] = c.g[V0][0] | 0x4;
                                        c.sh((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                                        break 'b_420424;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                            let t = c.g[V0][0] == 0u64;
                            c.g[S1][0] = c.g[S1][0] & 0xff;
                            if t {
                                break 'b_420480;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                    break 'b_420460;
                                } else {
                                    c.cycles += 5;
                                    c.g[A1][0] = 0x4a0000;
                                    c.g[A0][0] = 0x1;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5760));
                                    c.g[RA][0] = sx(0x00420458);
                                    c.g[A2][0] = 0x44;
                                    callee::WRITE.call(c, 0x0042_0458)?;
                                    break 'b_420458;
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = true;
                        c.g[V0][0] = 0xffffffffffffffff;
                        if t {
                            break 'b_4204f8;
                        } else {
                            break 'b_420460;
                        }
                    }
                    c.cycles += 8;
                    c.g[V0][0] = c.g[S1][0];
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                    c.sb((c.g[A0][0] as u32).wrapping_sub(0x1), c.g[S1][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    if t {
                        break 'b_4204f8;
                    } else {
                        break 'b_420480;
                    }
                }
                'b_4204c4: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    if c.g[V0][0] == 0u64 {
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        break 'b_4204c4;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                        c.g[V0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
                        if c.g[V0][0] == 0u64 {
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            break 'b_4204c4;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_sub(0x1));
                            if c.g[V0][0] != c.g[S1][0] {
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                break 'b_4204c4;
                            } else {
                                c.cycles += 7;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = c.g[S1][0];
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                                if t {
                                    break 'b_4204f8;
                                } else {
                                    break 'b_4204c4;
                                }
                            }
                        }
                    }
                }
                c.cycles += 13;
                c.g[V0][0] = 0x1;
                c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = 0x3;
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x42));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), c.g[A0][0]);
                c.g[V0][0] = c.g[S1][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x38), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[A3][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x34), c.g[A1][0]);
                c.sw(c.g[S0][0] as u32, c.g[A2][0]);
                c.sb((c.g[S0][0] as u32).wrapping_add(0x42), c.g[S1][0]);
                break 'b_4204f8;
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    });
}

/// `Bigint *__i2b(reent *r, int v)` (0x420978): wrap `v` in a
/// freshly `_Balloc`'d one-digit Bignum.
pub fn i2b(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x00420990);
        c.g[A1][0] = 0x1;
        callee::BALLOC.call(c, 0x0042_0990)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[A0][0] = 0x1;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[S0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    });
}

/// `char *print_f(double v, ...)` (0x421530): format `v` in
/// fixed (`%f`-style) notation through `_dtoa_r`, copying the digits
/// into the caller buffer.
pub fn print_f(c: &mut Cpu) {
    guest(c, |c| {
        'b_4216d4: {
            'b_42158c: {
                c.cycles += 19;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[T0][0] = c.g[SP][0];
                c.g[S0][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A1][0] = c.g[A2][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A2][0] = c.g[T2][0];
                c.g[S1][0] = c.g[A3][0];
                c.g[S2][0] = c.g[T1][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[T1][0] = c.g[SP][0] | 0x4;
                c.g[RA][0] = sx(0x00421568);
                c.g[T2][0] = c.g[SP][0] | 0x8;
                callee::DTOA_R.call(c, 0x0042_1568)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                c.g[V0][0] = 0x270f;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A2][0] = c.g[A1][0];
                if t {
                    break 'b_42158c;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00421584);
                    c.g[A0][0] = c.g[S0][0];
                    callee::STRCPY.call(c, 0x0042_1584)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        break 'b_4216d4;
                    } else {
                        break 'b_42158c;
                    }
                }
            }
            'b_4215f8: {
                'b_4215c8: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lbu(c.g[A1][0] as u32);
                    if t {
                        break 'b_4215c8;
                    } else {
                        c.cycles += 2;
                        let t = (c.g[V1][0] as i64) <= 0;
                        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                        if t {
                            break 'b_4215f8;
                        } else {
                            'l_4215a0: loop {
                                c.cycles += 8;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                c.sb(c.g[S0][0] as u32, c.g[A0][0]);
                                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                                c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = c.lbu(c.g[A1][0] as u32);
                                if t {
                                    break 'b_4215c8;
                                } else {
                                    c.cycles += 2;
                                    let t = (c.g[V1][0] as i64) > 0;
                                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0042_15a0);
                                        }
                                        continue 'l_4215a0;
                                    } else {
                                        break 'b_4215c8;
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                let t = (c.g[V0][0] as i64) <= 0;

                if t {
                    break 'b_4215f8;
                } else {
                    c.cycles += 1;
                    c.g[V1][0] = 0x30;
                    'l_4215d8: loop {
                        c.cycles += 7;
                        c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        let t = (c.g[V0][0] as i64) > 0;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0042_15d8);
                            }
                            continue 'l_4215d8;
                        } else {
                            c.cycles += 1;
                            c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                            break 'b_4215f8;
                        }
                    }
                }
            }
            'b_4216d0: {
                'b_42160c: {
                    c.cycles += 2;
                    let t = c.g[S2][0] != 0u64;

                    if t {
                        break 'b_42160c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                        if c.g[V0][0] == 0u64 {
                            c.sb(c.g[S0][0] as u32, 0u64);
                            break 'b_4216d0;
                        } else {
                            break 'b_42160c;
                        }
                    }
                }
                'b_421624: {
                    c.cycles += 2;
                    let t = c.g[A1][0] != c.g[A2][0];
                    c.g[V0][0] = 0x2e;
                    if t {
                        break 'b_421624;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = 0x30;
                        c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = 0x2e;
                        break 'b_421624;
                    }
                }
                'b_421668: {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                    let t = (c.g[V1][0] as i64) >= 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        break 'b_421668;
                    } else {
                        c.cycles += 2;
                        let t = (c.g[S1][0] as i64) <= 0;
                        c.g[V0][0] = c.g[V1][0];
                        if t {
                            break 'b_421668;
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = 0x30;
                            c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                            'l_421648: loop {
                                'b_421664: {
                                    c.cycles += 4;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    let t = (c.g[V0][0] as i64) >= 0;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                    if t {
                                        break 'b_421664;
                                    } else {
                                        c.cycles += 3;
                                        if (c.g[S1][0] as i64) > 0 {
                                            c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0042_1648);
                                            }
                                            continue 'l_421648;
                                        } else {
                                            break 'b_421664;
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                                break 'b_421668;
                            }
                        }
                    }
                }
                'b_4216a0: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
                    if t {
                        break 'b_4216a0;
                    } else {
                        c.cycles += 2;
                        if (c.g[S1][0] as i64) <= 0 {
                            c.sb(c.g[S0][0] as u32, 0u64);
                            break 'b_4216d0;
                        } else {
                            c.cycles += 1;
                            c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                            'l_421680: loop {
                                c.cycles += 6;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                                let t = c.g[V0][0] == 0u64;
                                c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
                                if t {
                                    break 'b_4216a0;
                                } else {
                                    c.cycles += 2;
                                    if (c.g[S1][0] as i64) > 0 {
                                        c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0042_1680);
                                        }
                                        continue 'l_421680;
                                    } else {
                                        break 'b_4216a0;
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 2;
                if (c.g[S1][0] as i64) <= 0 {
                    c.sb(c.g[S0][0] as u32, 0u64);
                    break 'b_4216d0;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x30;
                    'l_4216b0: loop {
                        c.cycles += 7;
                        c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                        let t = (c.g[S1][0] as i64) > 0;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0042_16b0);
                            }
                            continue 'l_4216b0;
                        } else {
                            c.cycles += 1;
                            c.sb(c.g[S0][0] as u32, 0u64);
                            break 'b_4216d0;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_4216d4;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    });
}

/// `char *print_e(double v, ...)` (0x4216e8): format `v` in
/// scientific (`%e`-style) notation through `_dtoa_r`, copying the
/// digits into the caller buffer.
pub fn print_e(c: &mut Cpu) {
    guest(c, |c| {
        'b_4218b8: {
            'b_42175c: {
                c.cycles += 25;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.g[T0][0] = sx((c.g[T0][0] as u32) << 24);
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[T2][0] = c.g[SP][0] | 0x8;
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S4][0] = c.g[A3][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[A2][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = sx(((c.g[T0][0] as u32) as i32 >> 24) as u32);
                c.g[S3][0] = c.g[T1][0];
                c.g[S1][0] = c.g[S4][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[A2][0] = 0x2;
                c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[T0][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x00421738);
                c.g[T1][0] = c.g[SP][0] | 0x4;
                callee::DTOA_R.call(c, 0x0042_1738)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                c.g[V0][0] = 0x270f;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                    break 'b_42175c;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00421754);
                    c.g[A0][0] = c.g[S0][0];
                    callee::STRCPY.call(c, 0x0042_1754)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        break 'b_4218b8;
                    } else {
                        break 'b_42175c;
                    }
                }
            }
            'b_421784: {
                'b_421774: {
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                    let t = c.g[S3][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        break 'b_421774;
                    } else {
                        c.cycles += 2;
                        if c.g[S1][0] == 0u64 {
                            c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                            break 'b_421784;
                        } else {
                            break 'b_421774;
                        }
                    }
                }
                c.cycles += 4;
                c.g[V0][0] = 0x2e;
                c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                break 'b_421784;
            }
            'b_4217bc: {
                'b_4217b8: {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
                    if t {
                        break 'b_4217b8;
                    } else {
                        c.cycles += 2;
                        let t = (c.g[S4][0] as i64) <= 0;
                        c.g[V0][0] = 0x67;
                        if t {
                            break 'b_4217bc;
                        } else {
                            c.cycles += 1;
                            c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                            'l_421798: loop {
                                c.cycles += 6;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                                let t = c.g[V0][0] == 0u64;
                                c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
                                if t {
                                    break 'b_4217b8;
                                } else {
                                    c.cycles += 2;
                                    if (c.g[S1][0] as i64) > 0 {
                                        c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0042_1798);
                                        }
                                        continue 'l_421798;
                                    } else {
                                        break 'b_4217b8;
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0x67;
                break 'b_4217bc;
            }
            'b_421804: {
                'b_4217cc: {
                    c.cycles += 2;
                    let t = c.g[S2][0] != c.g[V0][0];
                    c.g[V0][0] = 0x47;
                    if t {
                        break 'b_4217cc;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.g[S2][0] = 0x65;
                        if t {
                            break 'b_421804;
                        } else {
                            break 'b_4217cc;
                        }
                    }
                }
                'b_4217dc: {
                    c.cycles += 2;
                    let t = c.g[S2][0] != c.g[V0][0];

                    if t {
                        break 'b_4217dc;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.g[S2][0] = 0x45;
                        if t {
                            break 'b_421804;
                        } else {
                            break 'b_4217dc;
                        }
                    }
                }
                c.cycles += 2;
                let t = (c.g[S1][0] as i64) <= 0;
                c.g[V0][0] = 0x30;
                if t {
                    break 'b_421804;
                } else {
                    c.cycles += 1;
                    'l_4217e8: loop {
                        c.cycles += 7;
                        c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                        let t = (c.g[S1][0] as i64) > 0;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0042_17e8);
                            }
                            continue 'l_4217e8;
                        } else {
                            break 'b_421804;
                        }
                    }
                }
            }
            'b_421838: {
                'b_421830: {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    c.sb(c.g[S0][0] as u32, c.g[S2][0]);
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    let t = (c.g[V1][0] as i64) >= 0;
                    c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                    if t {
                        break 'b_421830;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = 0x2d;
                        c.g[V1][0] = sx(0u32.wrapping_sub(c.g[V1][0] as u32));
                        c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                        let t = true;
                        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                        if t {
                            break 'b_421838;
                        } else {
                            break 'b_421830;
                        }
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = 0x2b;
                c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                break 'b_421838;
            }
            'b_421878: {
                c.cycles += 4;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[V0][0] = ((c.g[A0][0] as i64) < 100i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if t {
                    break 'b_421878;
                } else {
                    'b_421858: {
                        c.cycles += 4;
                        c.g[V0][0] = 0x64;
                        c.div(0, 4, 2);
                        if c.g[V0][0] == 0u64 {
                            break 'b_421858;
                        } else {
                            break 'b_421858;
                        }
                    }
                    c.cycles += 8;
                    c.g[V1][0] = c.lo[0];
                    c.mult(0, 2, 3, 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                    c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    break 'b_421878;
                }
            }
            'b_421884: {
                c.cycles += 3;
                c.g[V1][0] = 0xa;
                if c.g[V1][0] == 0u64 {
                    break 'b_421884;
                } else {
                    break 'b_421884;
                }
            }
            c.cycles += 13;
            c.div(0, 4, 3);
            c.g[V0][0] = c.lo[0];
            c.mult(0, 3, 2, 3);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
            c.sb(c.g[S0][0] as u32, c.g[V0][0]);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            c.sb((c.g[S0][0] as u32).wrapping_add(0x1), 0u64);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            c.sw(c.g[SP][0] as u32, c.g[A0][0]);
            c.g[V0][0] = c.lbu(c.g[SP][0] as u32);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
            c.sb(c.g[S0][0] as u32, c.g[V0][0]);
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            break 'b_4218b8;
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    });
}

/// `char *fcvtbuf(double v, int ndigits, ...)` (0x4218d8): `fcvt`-style
/// conversion backend. Keeps a per-reent buffer (growing it through
/// `_realloc_r`), formats `v` with `_dtoa_r`, copies the digits and
/// pads with `'0'` to `ndigits`. Returns NULL when out of memory.
pub fn fcvtbuf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S0][0] = c.g[T0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S2][0] = 0u64;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S3][0] = c.g[A0][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S4][0] = c.g[A1][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S5][0] = 0x450000;
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S6][0] = c.g[A2][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S7][0] = c.g[A3][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        // Returns NULL through the shared epilogue.
        let mut failed = false;
        if c.g[S0][0] == 0 {
            // No caller buffer: use (or grow) the reent buffer.
            c.cycles += 5;
            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4944));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x50));
            if (c.g[S4][0] as i64) < (c.g[V0][0] as i64) {
                // Fits: reuse the reent buffer.
                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4944));
            } else {
                // Grow the reent buffer through `_realloc_r`.
                c.cycles += 7;
                c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x54));
                c.g[RA][0] = sx(0x0042_1944);
                c.g[A2][0] = c.g[S1][0];
                callee::REALLOC_R.call(c, 0x0042_1944)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = 0u64;
                if c.g[S0][0] == 0 {
                    failed = true;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4944));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x54), c.g[S0][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x50), c.g[S1][0]);
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4944));
                }
            }
            if !failed {
                c.cycles += 1;
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x54));
            }
        }
        if failed {
            c.g[V0][0] = 0u64;
        } else {
            // Format through `_dtoa_r`: sign check picks fixed (3) vs
            // float (2) mode.
            c.cycles += 7;
            c.g[A1][0] = 0xffc0;
            c.g[A1][0] <<= 46;
            c.g[S1][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0042_1978);
            c.g[A0][0] = c.g[S3][0];
            callee::CMPDF2.call(c, 0x0042_1978)?;
            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4944));
            let huge = (c.g[V0][0] as i64) >= 0;
            let mut mode = 3u64;
            if huge {
                c.cycles += 4;
            } else {
                c.cycles += 6;
                c.g[A1][0] = 0xbff0;
                c.g[A1][0] <<= 48;
                c.g[RA][0] = sx(0x0042_1990);
                c.g[A0][0] = c.g[S3][0];
                callee::CMPDF2.call(c, 0x0042_1990)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4944));
                if (c.g[V0][0] as i64) <= 0 {
                    c.cycles += 4;
                } else {
                    c.cycles += 5;
                    mode = 2;
                }
            }
            c.g[A1][0] = c.g[S3][0];
            c.g[T0][0] = c.g[S6][0];
            c.g[T1][0] = c.g[S7][0];
            c.cycles += 5;
            c.g[A3][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x0042_19c8);
            c.g[T2][0] = c.g[SP][0];
            c.g[A2][0] = mode;
            callee::DTOA_R.call(c, 0x0042_19c8)?;
            c.g[V1][0] = c.g[V0][0];
            // Copy digits until the end pointer, then pad with `'0'`.
            loop {
                c.cycles += 4;
                c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                if !(c.g[V1][0] < c.g[V0][0]) {
                    break;
                }
                c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                if back_edge_unwinds(c, 0x0042_19d0) {
                    return Ok(());
                }
                c.cycles += 4;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            }
            // Pad with `'0'` up to `ndigits`.
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S4][0] as i64)) as u64;
            c.g[S2][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[S2][0] as u32));
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                c.sb(c.g[S0][0] as u32, 0u64);
                c.g[V0][0] = c.g[S1][0];
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0x30;
                loop {
                    c.cycles += 7;
                    c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if c.g[S2][0] == 0 {
                        break;
                    }
                    if back_edge_unwinds(c, 0x0042_1a00) {
                        return Ok(());
                    }
                }
                c.cycles += 2;
                c.sb(c.g[S0][0] as u32, 0u64);
                c.g[V0][0] = c.g[S1][0];
            }
        }
        c.cycles += 11;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = ra;
        Ok(())
    });
}

/// `char *ecvtbuf(double v, ...)` (0x421a50): `ecvt`-style
/// conversion backend: `_dtoa_r` digits into the caller buffer (or the
/// reent buffer when NULL) with the decimal point and sign out.
pub fn ecvtbuf(c: &mut Cpu) {
    guest(c, |c| {
        'b_421b6c: {
            'b_421adc: {
                c.cycles += 18;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S0][0] = c.g[T0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S1][0] = 0u64;
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S3][0] = c.g[A1][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S4][0] = 0x450000;
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S5][0] = c.g[A2][0];
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S6][0] = c.g[A3][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S7][0] = c.g[A0][0];
                let t = c.g[S0][0] != 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                if t {
                    break 'b_421adc;
                } else {
                    'b_421ad8: {
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4944));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x50));
                        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4944));
                            break 'b_421ad8;
                        } else {
                            c.cycles += 7;
                            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x54));
                            c.g[RA][0] = sx(0x00421abc);
                            c.g[A2][0] = c.g[S2][0];
                            callee::REALLOC_R.call(c, 0x0042_1abc)?;
                            c.g[S0][0] = c.g[V0][0];
                            let t = c.g[S0][0] == 0u64;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_421b6c;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4944));
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x54), c.g[S0][0]);
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x50), c.g[S2][0]);
                                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4944));
                                break 'b_421ad8;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x54));
                    break 'b_421adc;
                }
            }
            'b_421b3c: {
                c.cycles += 14;
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4944));
                c.g[A1][0] = c.g[S7][0];
                c.g[T0][0] = c.g[S5][0];
                c.g[T1][0] = c.g[S6][0];
                c.g[A2][0] = 0x2;
                c.g[A3][0] = c.g[S3][0];
                c.g[T2][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x00421b00);
                c.g[S2][0] = c.g[S0][0];
                callee::DTOA_R.call(c, 0x0042_1b00)?;
                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                c.g[A0][0] = c.g[V0][0];
                c.g[V1][0] = (c.g[A0][0] < c.g[V1][0]) as u64;
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                if t {
                    break 'b_421b3c;
                } else {
                    c.cycles += 1;
                    'l_421b18: loop {
                        c.cycles += 8;
                        c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                        c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0042_1b18);
                            }
                            continue 'l_421b18;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                            break 'b_421b3c;
                        }
                    }
                }
            }
            'b_421b64: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                if t {
                    break 'b_421b64;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 0x30;
                    'l_421b48: loop {
                        c.cycles += 7;
                        c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                        let t = c.g[S1][0] != 0u64;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0042_1b48);
                            }
                            continue 'l_421b48;
                        } else {
                            break 'b_421b64;
                        }
                    }
                }
            }
            c.cycles += 2;
            c.sb(c.g[S0][0] as u32, 0u64);
            c.g[V0][0] = c.g[S2][0];
            break 'b_421b6c;
        }
        c.cycles += 11;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        Ok(())
    });
}

/// `int quorem(Bigint *b, Bigint *S)` (0x422560): divide `b` by `S`
/// in place, returning the quotient digit. Long-division estimate via
/// `divu`, subtract/add fixup loops, and a final `__mcmp` correction.
pub fn quorem(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.g[T5][0] = c.g[A1][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S4][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
        let narrow = c.g[V0][0] != 0u64;
        c.g[V0][0] = 0u64;
        if narrow {
            // `b` narrower than `S`: quotient digit 0.
        } else {
            // Quotient estimate `S5 = n/d`.
            c.cycles += 14;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            c.g[T3][0] = sx((c.g[T5][0] as u32).wrapping_add(0x14));
            c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[T2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x14));
            c.g[S3][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[T0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = c.lw(c.g[S3][0] as u32);
            c.g[S6][0] = c.g[T3][0];
            c.g[V1][0] = c.lw(c.g[T0][0] as u32);
            c.g[S1][0] = c.g[T2][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.divu(0, 3, 2);
            // (The `V0 == 0` arm rejoins at `0x25d8` either way.)
            c.cycles += 4;
            c.g[S5][0] = c.lo[0];
            c.g[S2][0] = c.g[S5][0];
            c.g[T4][0] = 0u64;
            if c.g[S2][0] == 0 {
                // Zero estimate: skip the subtract loop.
                c.cycles += 1;
                c.g[A1][0] = c.g[T5][0];
            } else {
                // Subtract `S2 * S` from `b`, limb by limb.
                c.cycles += 2;
                c.g[V1][0] = 0u64;
                loop {
                    c.cycles += 26;
                    c.g[A0][0] = c.lw(c.g[T3][0] as u32);
                    c.g[A2][0] = c.lw(c.g[T2][0] as u32);
                    c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[A0][0] & 0xffff;
                    c.g[A3][0] = (c.g[S3][0] < c.g[T3][0]) as u64;
                    c.mult(0, 5, 2, 18);
                    c.g[A0][0] = sx((c.g[A0][0] as u32) >> 16);
                    c.mult(0, 4, 4, 18);
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A1][0] = c.g[V0][0] & 0xffff;
                    c.g[V1][0] = c.g[A2][0] & 0xffff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 16);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                    c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T4][0] as u32));
                    c.g[A2][0] = sx((c.g[A2][0] as u32) >> 16);
                    c.g[V0][0] = c.g[T1][0] & 0xffff;
                    c.g[T4][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T4][0] as u32));
                    c.sh(c.g[T2][0] as u32, c.g[V1][0]);
                    c.sh((c.g[T2][0] as u32).wrapping_add(0x2), c.g[A1][0]);
                    c.g[V1][0] = sx((c.g[T1][0] as u32) >> 16);
                    c.g[T4][0] = sx(((c.g[A1][0] as u32) as i32 >> 16) as u32);
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x4));
                    if c.g[A3][0] != 0 {
                        break;
                    }
                    if back_edge_unwinds(c, 0x0042_25f0) {
                        return Ok(());
                    }
                }
                // Trim leading zero limbs off `b`.
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                c.g[A1][0] = c.g[T5][0];
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4));
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < c.g[T0][0]) as u64;
                    if c.g[V0][0] != 0 {
                        // Zero-skip loop back through `0x2670`.
                        loop {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4));
                            if c.g[V0][0] != 0 {
                                // Nonzero limb: store the trimmed width,
                                // same as the tail trim and generated 0x268c.
                                c.cycles += 1;
                                c.sw((c.g[S4][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                                break;
                            }
                            if back_edge_unwinds(c, 0x0042_2670) {
                                return Ok(());
                            }
                            c.cycles += 1;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[S1][0] < c.g[T0][0]) as u64;
                            if c.g[V0][0] == 0 {
                                c.sw((c.g[S4][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                                break;
                            }
                        }
                    } else {
                        c.sw((c.g[S4][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                    }
                    c.cycles += 1;
                    c.g[A1][0] = c.g[T5][0];
                }
            }
            // Correction: compare, then add back while negative.
            c.cycles += 4;
            c.g[RA][0] = sx(0x0042_269c);
            c.g[A0][0] = c.g[S4][0];
            callee::MCMP.call(c, 0x0042_269c)?;
            c.g[T3][0] = c.g[S6][0];
            if (c.g[V0][0] as i64) < 0 {
                // Over-subtracted: quotient digit stands, return it.
                c.cycles += 1;
                c.g[V0][0] = c.g[S2][0];
            } else {
                // Add `S` back until `b` is nonnegative.
                c.cycles += 5;
                c.g[S2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                c.g[T4][0] = 0u64;
                c.g[V1][0] = 0u64;
                c.g[T2][0] = c.g[S1][0];
                c.g[A3][0] = sx((c.g[S0][0] as u32) << 2);
                loop {
                    c.cycles += 24;
                    c.g[A0][0] = c.lw(c.g[T3][0] as u32);
                    c.g[A1][0] = c.lw(c.g[T2][0] as u32);
                    c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[A0][0] & 0xffff;
                    c.g[A2][0] = sx((c.g[A0][0] as u32) >> 16);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = c.g[V0][0] & 0xffff;
                    c.g[V1][0] = c.g[A1][0] & 0xffff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 16);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T4][0] as u32));
                    c.g[V0][0] = c.g[T1][0] & 0xffff;
                    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 16);
                    c.g[T4][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T4][0] as u32));
                    c.sh(c.g[T2][0] as u32, c.g[V1][0]);
                    c.sh((c.g[T2][0] as u32).wrapping_add(0x2), c.g[A1][0]);
                    c.g[V1][0] = sx((c.g[T1][0] as u32) >> 16);
                    c.g[T4][0] = sx(((c.g[A1][0] as u32) as i32 >> 16) as u32);
                    c.g[V0][0] = (c.g[S3][0] < c.g[T3][0]) as u64;
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x4));
                    if c.g[V0][0] != 0 {
                        break;
                    }
                    if back_edge_unwinds(c, 0x0042_26b8) {
                        return Ok(());
                    }
                }
                // Trim a zero tail the same way as the head.
                c.cycles += 4;
                c.g[T0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                let tail_nonzero = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.g[S2][0];
                if !tail_nonzero {
                    c.cycles += 2;
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4));
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < c.g[T0][0]) as u64;
                    if c.g[V0][0] == 0 {
                        c.sw((c.g[S4][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                    } else {
                        loop {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                            if c.g[V0][0] != 0 {
                                c.cycles += 1;
                                c.sw((c.g[S4][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                                break;
                            }
                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4));
                            if back_edge_unwinds(c, 0x0042_2730) {
                                return Ok(());
                            }
                            c.cycles += 1;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[S1][0] < c.g[T0][0]) as u64;
                            if c.g[V0][0] == 0 {
                                c.sw((c.g[S4][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                                break;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S2][0];
                }
            }
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = ra;
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::check_paths;
    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::ai::computer::tests::SEEDS_3 as SEEDS_2;
    pub(crate) use crate::game::lib::gcc::tests::plain;
    pub(crate) use crate::game::lib::gcc::tests::SEEDS;

    #[test]
    fn date_is_leap_year_matches() {
        check_lohi(
            ssx3_ee::s08::f_002c71c0,
            date_is_leap_year,
            300,
            &|c, seed| {
                let mut r = Rng(0x71c0 ^ seed as u64);
                scramble(c, &mut r);
                let year = match seed % 5 {
                    0 => (r.next_u32() % 1000).wrapping_mul(4).wrapping_add(1),
                    1 => (r.next_u32() % 1000).wrapping_mul(4),
                    2 => (r.next_u32() % 100).wrapping_mul(100),
                    3 => (r.next_u32() % 100).wrapping_mul(400),
                    _ => r.next_u32(),
                };
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | year as u64;
            },
        );
    }

    #[test]
    fn time_add_seconds_matches() {
        check_lohi(
            ssx3_ee::s08::f_002c7328,
            time_add_seconds,
            200,
            &|c, seed| {
                let mut r = Rng(0x7328 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = (DATA + 0x200) as u64;
                c.g[A3][0] = (DATA + 0x300) as u64;
                c.g[T0][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.bus.write32(DATA + 0x100, r.next_u32());
                c.bus.write32(DATA + 0x200, r.next_u32());
                c.bus.write32(DATA + 0x300, r.next_u32());
            },
        );
    }

    #[test]
    fn cleanup_r_matches() {
        plain(ssx3_ee::s15::f_00416fc0, cleanup_r, 0x162);
    }

    #[test]
    fn cleanup_matches() {
        plain(ssx3_ee::s15::f_00416fe0, cleanup, 0x163);
    }

    // ------------------------------------------------------------ leaves

    #[test]
    fn localeconv_r_matches() {
        check(ssx3_ee::s15::f_0041ec08, localeconv_r, SEEDS, &|c, seed| {
            let mut r = Rng(0x1111_2222 ^ seed as u64);
            scramble(c, &mut r);
        });
    }

    // ------------------------------------------------- reentrant wrappers

    /// Scrambled state with `reent` and the arguments the wrapper forwards.
    pub(crate) fn reent_args(c: &mut Cpu, seed: u32, salt: u64, n: usize) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.g[A0][0] = (DATA + 0x400) as u64;
        for i in 0..n {
            c.g[[A1, A2, A3][i]][0] = (r.next_u32() % 0x2000) as u64;
        }
        r
    }

    #[test]
    fn close_r_matches() {
        check(ssx3_ee::s15::f_0041fb98, close_r, SEEDS, &|c, seed| {
            reent_args(c, seed, 20, 1);
        });
    }

    #[test]
    fn fstat_r_matches() {
        check(ssx3_ee::s15::f_0041fbf0, fstat_r, SEEDS, &|c, seed| {
            reent_args(c, seed, 21, 3);
            c.g[A2][0] = (DATA + 0x800) as u64; // struct stat
        });
    }

    #[test]
    fn lseek_r_matches() {
        check(ssx3_ee::s15::f_0041fc50, lseek_r, SEEDS, &|c, seed| {
            reent_args(c, seed, 22, 3);
        });
    }

    #[test]
    fn open_r_matches() {
        check(ssx3_ee::s15::f_0041fcb0, open_r, SEEDS, &|c, seed| {
            reent_args(c, seed, 23, 3);
            c.g[A1][0] = (DATA + 0x100) as u64; // path
        });
    }

    #[test]
    fn read_r_matches() {
        check(ssx3_ee::s15::f_0041fd10, read_r, SEEDS, &|c, seed| {
            let mut r = reent_args(c, seed, 24, 3);
            // A non-zero fd keeps `_read` on its quick `-1` path.
            c.g[A1][0] = (1 + r.next_u32() % 8) as u64;
            c.g[A2][0] = (DATA + 0x1000) as u64; // buffer
        });
    }

    #[test]
    fn write_r_matches() {
        check(ssx3_ee::s15::f_0041fdd0, write_r, SEEDS, &|c, seed| {
            let mut r = reent_args(c, seed, 25, 3);
            // fd 0 or >= 3 keeps `_write` off the tty path.
            c.g[A1][0] = if seed % 2 == 0 {
                0
            } else {
                (3 + r.next_u32() % 8) as u64
            };
            c.g[A2][0] = (DATA + 0x1000) as u64; // buffer
        });
    }

    #[test]
    fn sbrk_r_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 26, 1);
            c.g[A0][0] = (DATA + 0x400) as u64;
            c.g[A1][0] = (r.next_u32() % 0x1000) as u64;
        };
        covered(ssx3_ee::s15::f_0041fd70, sbrk_r, SEEDS, &setup);
    }

    // ---------------------------------------------------------------- kill

    #[test]
    fn kill_matches() {
        check(ssx3_ee::s15::f_00424578, kill, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 42, 0);
            // Any pid but 1: the exit path runs the kernel's shutdown code,
            // which cannot run on a test machine.
            c.g[A0][0] = (2 + r.next_u32() % 30) as u64;
            c.g[A1][0] = (r.next_u32() % 32) as u64;
        });
    }

    // --------------------------------------------------- bignum / bit helpers

    #[test]
    fn bfree_matches() {
        check(ssx3_ee::s15::f_004205b8, bfree, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 60, 0);
            let reent = DATA + 0x400;
            let bucket = DATA + 0x1000;
            c.g[A0][0] = reent as u64;
            c.bus.write32(reent + 0x4c, bucket);
            let p = if seed % 5 == 0 {
                0
            } else {
                (DATA + 0x500) as u64
            };
            c.g[A1][0] = p;
            if p != 0 {
                c.bus.write32(p as u32 + 0x4, seed % 4);
                c.bus.write32(p as u32, r.next_u32());
            }
        });
    }

    #[test]
    fn mprec_log10_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 65, 0);
            let exp = match seed % 5 {
                0 => (r.next_u32() % 4) as i32 - 2, // <= 0
                1 => (r.next_u32() % 24) as i32,
                2 => 24 + (r.next_u32() % 8) as i32,
                _ => (r.next_u32() % 40) as i32,
            };
            c.g[A0][0] = sx(exp as u32);
            schedule(c, &mut r, seed, 0x0042_14c0)
        };
        check_events(ssx3_ee::s15::f_004214c0, mprec_log10, SEEDS, &setup);
    }

    #[test]
    fn fstat_matches() {
        check(ssx3_ee::s15::f_00424558, fstat, SEEDS, &|c, seed| {
            let mut r = Rng(0x3333_4444 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (r.next_u32() % 8) as u64;
            c.g[A1][0] = (DATA + (r.next_u32() & 0xfff0)) as u64;
        });
    }

    // -------------------------------------------------------------- vfprintf

    /// Format strings that reach every scan path. `%K` is made to select a
    /// table word that is not a handler (the `jump_out` arm, which the test
    /// machine turns into a fake call); `%M` is a real "keep scanning" word;
    /// `%L` is a real `_vfprintf_r` word and, being only a length modifier,
    /// leaves the formatter at the end of the string (no conversion argument
    /// is fetched, so no float formatting runs).
    pub(crate) const FORMATS: [&[u8]; 9] = [
        b"", b"%", b"a%", b"a%K", b"%1K", b"%M%K", b"%K", b"%%K", b"%L",
    ];

    /// The specifier table word for `c` (the original indexes it by
    /// `c - 'E'`).
    pub(crate) fn spec_slot(ch: u8) -> u32 {
        SPEC_TABLE + (ch as u32 - SPEC_FIRST) * 4
    }

    pub(crate) fn printf_setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x106, 0);
        // The locale, the stdio globals and the specifier table are the
        // game's own data.
        elf_image(c);
        // FILE at DATA: zeroed, only the reent pointer at +0x54 is used.
        c.bus.write32(DATA + 0x54, DATA + 0x800);
        let fmt = DATA + 0x100;
        let bytes = FORMATS[seed as usize % FORMATS.len()];
        for (i, b) in bytes.iter().enumerate() {
            c.bus.write8(fmt + i as u32, *b);
        }
        c.bus.write8(fmt + bytes.len() as u32, 0);
        // Break the 'K' slot: the dispatcher must take its out-of-range arm.
        c.bus.write32(spec_slot(b'K'), 0xdead_beef);
        // 'M' keeps the real "keep scanning" word; make sure it is there.
        c.bus.write32(spec_slot(b'M'), 0x0041_b6fc);
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = fmt as u64;
        c.g[A2][0] = (DATA + 0x200) as u64;
        c.pc = 0x0041_b658;
    }

    #[test]
    fn vfprintf_dispatch_matches() {
        check(
            ssx3_ee::s15::f_0041b658__generated,
            vfprintf_dispatch,
            SEEDS_2,
            &printf_setup,
        );
    }

    pub(crate) const SEEDS_3: u32 = 300;

    pub(crate) const REENT: u32 = DATA + 0x1000;

    pub(crate) const STATE: u32 = DATA + 0x1100;

    pub(crate) const STR: u32 = DATA + 0x1200;

    pub(crate) const PWC: u32 = DATA + 0x1300;

    /// `""` and `"XY"`: the short/unknown locale names.
    pub(crate) const SHORT: u32 = DATA + 0x1400;

    pub(crate) const OTHER: u32 = DATA + 0x1410;

    /// Hard, soft and LO/HI must all match: the original keeps every
    /// intermediate in guest registers (and runs one `mult`), so a caller
    /// could observe them.
    pub(crate) fn check_strict(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..seed_count(seeds) {
            let r = compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} (scratch): {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} (LO/HI): {:#?}", r.lohi);
        }
    }

    /// Bytes that land in every class of the three decoders: NUL, a plain
    /// ASCII letter, ESC, `$`, `(`, `)`, `@`, `B`, `J`, `]`, an SJIS lead, a
    /// high SJIS lead, an EUC lead/trail, and the ends of both trail ranges.
    pub(crate) const BYTES: [u8; 16] = [
        0x00, 0x41, 0x1b, 0x24, 0x28, 0x29, 0x40, 0x42, 0x4a, 0x5d, 0x81, 0x9f, 0xa1, 0xe0, 0xef,
        0xfc,
    ];

    pub(crate) fn setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x4189_58, 0);
        // The locale names and the two C-JIS tables are the game's own data.
        elf_image(c);
        c.bus.write8(SHORT, 0);
        c.bus.write8(OTHER, b'X');
        c.bus.write8(OTHER + 1, b'Y');
        c.bus.write8(OTHER + 2, 0);

        let case = seed % 7;
        let locale = match case {
            0 => 0,     // no locale name: single byte
            1 => SHORT, // strlen < 2: single byte
            2 => LOCALE_SJIS,
            3 => LOCALE_EUCJP,
            4 => LOCALE_JIS,
            5 => OTHER, // none of the three: single byte
            _ => LOCALE_SJIS,
        };
        c.bus.write32(REENT + 0x34, locale);

        // Input bytes: the two selected ones then zeros (so a two-byte pack
        // reads well-defined data). The strides 5 and 7 walk all of `BYTES`
        // for every case within the seed range.
        let b0 = BYTES[(seed as usize * 5 + case as usize) % BYTES.len()];
        let b1 = BYTES[(seed as usize * 7 + case as usize) % BYTES.len()];
        for i in 0..8u32 {
            c.bus.write8(STR + i, 0);
        }
        c.bus.write8(STR, b0);
        c.bus.write8(STR + 1, b1);

        // The mbstate rows: 0 (initial) and a state in progress.
        c.bus
            .write32(STATE, [0u32, 1, 0xff, 0][(seed as usize / 7) % 4]);

        c.g[A0][0] = REENT as u64;
        c.g[A1][0] = if seed % 5 == 0 { 0 } else { PWC as u64 };
        c.g[A2][0] = if seed % 3 == 0 { 0 } else { STR as u64 };
        c.g[A3][0] = (seed % 4) as u64;
        c.g[T0][0] = STATE as u64;
        c.pc = START;
    }

    #[test]
    fn mbtowc_r_matches() {
        check_strict(
            ssx3_ee::s15::f_00418958__generated,
            mbtowc_r,
            SEEDS_3,
            &|c, seed| setup(c, seed),
        );
    }

    /// The `__sFILE` the formatter writes through (`_p` at +0, `_w` at +8,
    /// `_flags`/`_file` at +0xc/+0xe, `_bf` at +0x10, `_data` at +0x54).
    pub(crate) const FILE: u32 = DATA;

    /// `struct _reent` handed in as the first argument (and at FILE + 0x54).
    pub(crate) const REENT_2: u32 = DATA + 0x400;

    /// The format string.
    pub(crate) const FMT: u32 = DATA + 0x100;

    /// The `va_list`: one 8-byte slot per argument.
    pub(crate) const ARGS: u32 = DATA + 0x200;

    /// `%s` source strings.
    pub(crate) const STR_2: u32 = DATA + 0x2000;

    /// `%n` target.
    pub(crate) const NVAL: u32 = DATA + 0x3000;

    /// `fp->_p`: the write buffer the inlined copy path fills.
    pub(crate) const OUT: u32 = DATA + 0x4000;

    /// What kind of argument a conversion takes (the value is made from the
    /// seed in [`printf_setup_2`]).
    #[derive(Clone, Copy, PartialEq)]
    pub(crate) enum Kind {
        /// `int` (`lw`/`lh`, zero-extended slot).
        Int,
        /// `long`/`long long` (the original reads the whole 8-byte slot).
        Long,
        /// `const char *`.
        Str,
        /// A writable pointer (`%p`, `%n`).
        Ptr,
        /// `*` width: a small (sometimes negative) int.
        Width,
        /// `.*` precision: `-2..9`.
        Prec,
    }

    pub(crate) struct Case {
        fmt: &'static [u8],
        args: &'static [Kind],
    }

    /// Formats that reach every handler of the specifier table, every flag,
    /// both padding writers and both write paths. `%e`/`%f`/`%g` cannot occur
    /// here (the dispatcher sends them to `_vfprintf_r`).
    #[rustfmt::skip]
    pub(crate) const CASES: &[Case] = &[
        Case { fmt: b"",            args: &[] },
        Case { fmt: b"a",           args: &[] },
        Case { fmt: b"%%",          args: &[] },
        Case { fmt: b"a%%b",        args: &[] },
        Case { fmt: b"y",           args: &[] },
        Case { fmt: b"%y",          args: &[] },
        Case { fmt: b"%d",          args: &[Kind::Int] },
        Case { fmt: b"%i",          args: &[Kind::Int] },
        Case { fmt: b"%u",          args: &[Kind::Int] },
        Case { fmt: b"%c",          args: &[Kind::Int] },
        Case { fmt: b"%s",          args: &[Kind::Str] },
        Case { fmt: b"%x",          args: &[Kind::Int] },
        Case { fmt: b"%X",          args: &[Kind::Int] },
        Case { fmt: b"%#x",         args: &[Kind::Int] },
        Case { fmt: b"%#X",         args: &[Kind::Int] },
        Case { fmt: b"%#o",         args: &[Kind::Int] },
        Case { fmt: b"%o",          args: &[Kind::Int] },
        Case { fmt: b"%O",          args: &[Kind::Int] },
        Case { fmt: b"%p",          args: &[Kind::Ptr] },
        Case { fmt: b"%n",          args: &[Kind::Ptr] },
        Case { fmt: b"%hhn",        args: &[Kind::Ptr] },
        Case { fmt: b"%ln",         args: &[Kind::Ptr] },
        Case { fmt: b"%hd",         args: &[Kind::Int] },
        Case { fmt: b"%ld",         args: &[Kind::Long] },
        Case { fmt: b"%lld",        args: &[Kind::Long] },
        Case { fmt: b"%llu",        args: &[Kind::Long] },
        Case { fmt: b"%llx",        args: &[Kind::Long] },
        Case { fmt: b"%qd",         args: &[Kind::Long] },
        Case { fmt: b"%D",          args: &[Kind::Int] },
        Case { fmt: b"%U",          args: &[Kind::Int] },
        Case { fmt: b"%5d",         args: &[Kind::Int] },
        Case { fmt: b"%-5d",        args: &[Kind::Int] },
        Case { fmt: b"%05d",        args: &[Kind::Int] },
        Case { fmt: b"%+d",         args: &[Kind::Int] },
        Case { fmt: b"% d",         args: &[Kind::Int] },
        Case { fmt: b"%.3d",        args: &[Kind::Int] },
        Case { fmt: b"%.0d",        args: &[Kind::Int] },
        Case { fmt: b"%*d",         args: &[Kind::Width, Kind::Int] },
        Case { fmt: b"%-*d",        args: &[Kind::Width, Kind::Int] },
        Case { fmt: b"%.*d",        args: &[Kind::Prec, Kind::Int] },
        Case { fmt: b"%.*s",        args: &[Kind::Prec, Kind::Str] },
        Case { fmt: b"%40d",        args: &[Kind::Int] },
        Case { fmt: b"%-40d",       args: &[Kind::Int] },
        Case { fmt: b"%030d",       args: &[Kind::Int] },
        Case { fmt: b"%'d",         args: &[Kind::Int] },
        Case { fmt: b"%'20d",       args: &[Kind::Int] },
        Case { fmt: b"%-'20d",      args: &[Kind::Int] },
        Case { fmt: b"%10.5s",      args: &[Kind::Str] },
        Case { fmt: b"%.2s",        args: &[Kind::Str] },
        Case { fmt: b"%5s",         args: &[Kind::Str] },
        Case { fmt: b"%-5s",        args: &[Kind::Str] },
        Case { fmt: b"%40s",        args: &[Kind::Str] },
        Case { fmt: b"%5c",         args: &[Kind::Int] },
        Case { fmt: b"%d%d%d",      args: &[Kind::Int, Kind::Int, Kind::Int] },
        Case { fmt: b"a %s b %d c", args: &[Kind::Str, Kind::Int] },
        Case { fmt: b"%08.3d",      args: &[Kind::Int] },
    ];

    /// The formatter's inputs: the game's own image (locale, `_mbtowc_r`,
    /// digit tables, the `vfiprintf_BufferedWrite` log buffer), a writeable
    /// `FILE` (alternating between the buffered fast path and the
    /// `vfiprintf_BufferedWrite` path, and occasionally a full buffer so the
    /// formatter gives up after the first run), and one case's format plus
    /// its varargs.
    pub(crate) fn printf_setup_2(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) * 0x0100_0001);
        scramble(c, &mut r);
        elf_image(c);
        c.g[GP][0] = GAME_GP as u64;

        // __sFILE: buffered for most seeds, "buffer full" for some.
        let (flags, w) = match seed % 5 {
            0 => (0u16, 0x400u32), // not __SWR: vfiprintf_BufferedWrite
            1 => (0x200, 0),       // __SWR with no room: stop after a run
            _ => (0x200, 0x400),   // __SWR with room: the inline copy
        };
        c.bus.write32(FILE, OUT); // _p
        c.bus.write32(FILE + 0x4, 0); // _r
        c.bus.write32(FILE + 0x8, w); // _w
        c.bus.write16(FILE + 0xc, flags); // _flags
        c.bus.write16(FILE + 0xe, (seed % 4) as u16); // _file
        c.bus.write32(FILE + 0x10, OUT); // _bf._base
        c.bus.write32(FILE + 0x14, 0x100); // _bf._size
        c.bus.write32(FILE + 0x54, REENT_2); // _data

        let case = &CASES[seed as usize % CASES.len()];
        for (i, b) in case.fmt.iter().enumerate() {
            c.bus.write8(FMT + i as u32, *b);
        }
        c.bus.write8(FMT + case.fmt.len() as u32, 0);

        for (i, kind) in case.args.iter().enumerate() {
            let (lo, hi) = match *kind {
                Kind::Int => {
                    let v = match seed % 11 {
                        0 => 0,
                        1 => 1,
                        2 => 0xffff_ffff, // -1
                        3 => 0x8000_0000, // INT_MIN
                        4 => 9,
                        _ => r.next_u32(),
                    };
                    (v, 0)
                }
                Kind::Long => {
                    let v = match seed % 13 {
                        0 => 0,
                        1 => 1,
                        2 => u64::MAX, // -1
                        3 => 0x8000_0000_0000_0000,
                        4 => 0x0000_0001_0000_0000,
                        5 => 0x7fff_ffff_ffff_ffff,
                        _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                    };
                    ((v & 0xffff_ffff) as u32, (v >> 32) as u32)
                }
                Kind::Str => {
                    if seed % 13 == 0 {
                        (0, 0) // the "(null)" default string
                    } else {
                        let len = seed % 24;
                        for j in 0..len {
                            c.bus.write8(STR_2 + j, b'a' + ((seed + j) % 26) as u8);
                        }
                        c.bus.write8(STR_2 + len, 0);
                        (STR_2, 0)
                    }
                }
                Kind::Ptr => (NVAL, 0),
                Kind::Width => (((seed as i32 % 60) - 20) as u32, 0),
                Kind::Prec => (((seed as i32 % 12) - 2) as u32, 0),
            };
            let slot = ARGS + 8 * i as u32;
            c.bus.write32(slot, lo);
            c.bus.write32(slot + 4, hi);
        }

        c.g[A0][0] = REENT_2 as u64;
        c.g[A1][0] = FILE as u64;
        c.g[A2][0] = FMT as u64;
        c.g[A3][0] = ARGS as u64;
        c.pc = 0x0041_ca70;
    }

    /// Strict comparison: hard state, caller-saved state *and* LO/HI (the
    /// decimal/octal/hex digit loops use `mult`, and `__udivdi3` leaves its
    /// own LO/HI).
    ///
    /// NOTE: once 0x41ca70 gets an override slot the generated body is
    /// `f_0041ca70__generated`; this must be switched to that name, otherwise
    /// the test would compare the replacement with itself.
    #[test]
    fn vfiprintf_r_matches() {
        for seed in 0..SEEDS {
            let r = compare(ssx3_ee::s15::f_0041ca70__generated, vfiprintf_r, &|c| {
                printf_setup_2(c, seed)
            });
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(
                r.soft.is_empty(),
                "seed {seed} (caller-saved): {:#?}",
                r.soft
            );
            assert!(r.lohi.is_empty(), "seed {seed} (LO/HI): {:#?}", r.lohi);
        }
    }

    // -- 0x00411cd8 _fpadd_parts_d --------------------------------------------

    /// `_fpadd_parts_d` against the generated body: every class pair (NaNs,
    /// zeros, infinities of equal and opposite signs, numbers), exponents
    /// equal, close (alignment shifts both ways) and 64 or more apart,
    /// same and opposite signs (sums that carry past bit 60, differences
    /// that cancel partly or completely and normalise left).
    #[test]
    fn fpadd_parts_d_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        const A: u32 = DATA;
        const B: u32 = DATA + 0x40;
        const TMP: u32 = DATA + 0x80;
        check(
            ee::s15::f_00411cd8__generated,
            fpadd_parts_d,
            400,
            &|c, seed| {
                let mut r = Rng(0x0041_1cd8 ^ (seed as u64) << 24 ^ 0x5bd1_e995);
                scramble(c, &mut r);
                c.pc = 0x0041_1cd8;
                let class = |r: &mut Rng| match r.next_u32() % 10 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 4,
                    _ => 3,
                };
                let frac = |r: &mut Rng| {
                    let f = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                    match r.next_u32() % 4 {
                        0 => f, // anything (NaN payloads, unnormalised)
                        _ => (f & ((1u64 << 60) - 1)) | (1u64 << 60),
                    }
                };
                let ea = (r.next_u32() % 200) as i32 - 100;
                let eb = match r.next_u32() % 4 {
                    0 => ea,
                    1 => ea + (r.next_u32() % 70) as i32 - 35,
                    2 => ea + 64 + (r.next_u32() % 10) as i32,
                    _ => ea - 64 - (r.next_u32() % 10) as i32,
                };
                let fa = frac(&mut r);
                // Near-cancellation: b's fraction close to a's.
                let fb = if seed % 5 == 0 {
                    fa ^ (r.next_u32() as u64 & 0xffff)
                } else {
                    frac(&mut r)
                };
                for (p, cl, e, fr) in [(A, class(&mut r), ea, fa), (B, class(&mut r), eb, fb)] {
                    c.bus.write32(p, cl);
                    c.bus.write32(p + 4, r.next_u32() % 2);
                    c.bus.write32(p + 8, e as u32);
                    c.bus.write64(p + 0x10, fr);
                }
                if seed % 7 == 0 {
                    // Equal operands of opposite sign: exact cancellation.
                    let (cl, sign, e) = (c.bus.read32(A), c.bus.read32(A + 4), c.bus.read32(A + 8));
                    let fr = c.bus.read64(A + 0x10);
                    c.bus.write32(B, cl);
                    c.bus.write32(B + 4, sign ^ 1);
                    c.bus.write32(B + 8, e);
                    c.bus.write64(B + 0x10, fr);
                }
                for k in 0..6 {
                    c.bus.write32(TMP + 4 * k, r.next_u32());
                }
                c.g[A0][0] = sx(A);
                c.g[A1][0] = sx(B);
                c.g[A2][0] = sx(TMP);
            },
        );
    }
    pub(crate) use crate::game::misc::tests::FILE as FILE_2;
    pub(crate) use crate::game::misc::tests::OUT as OUT_2;

    /// Gate block for the override-slot fakes below: a magic word plus
    /// the values the fakes report. The real `_open_r` and `_fstat_r`
    /// always fail in the harness (no host files), so the success arms
    /// need gated replacements. Each fake is hooked once per process
    /// into its generated override slot; with the gate word clear (fresh
    /// RAM) the fake runs whatever the slot held, i.e. the generated
    /// body, so the other tests are unaffected.
    pub(crate) const FAKE_GATE: u32 = DATA + 0x5000;
    pub(crate) const FAKE_MAGIC: u32 = 0x4641_4b45;
    /// Gate offsets: the fd the `_open_r` fake reports; the mode halfword
    /// and size doubleword the `_fstat_r` fake stores at `A2 + 4` /
    /// `A2 + 0x10`.
    pub(crate) const FAKE_FD_OFF: u32 = 4;
    pub(crate) const FAKE_MODE_OFF: u32 = 8;
    pub(crate) const FAKE_SIZE_OFF: u32 = 16;
    /// Gate offset: the word the `_fstat_r` fake returns (0 unless a
    /// fixture asks for a failure; the game's own `_fstat` never fails).
    pub(crate) const FAKE_RESULT_OFF: u32 = 0x18;

    /// Gated `_open_r` (slot `0x41fcb0`): report the fixture's fd.
    fn libc_fake_open_r(c: &mut Cpu) {
        if c.bus.read32(FAKE_GATE) != FAKE_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(libc_fake_open_r),
                ee::s15::f_0041fcb0__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(FAKE_GATE + FAKE_FD_OFF) as u64;
        c.cycles += 3;
        crate::abi::ret(c);
    }

    /// Gated `_fstat_r` (slot `0x41fbf0`): store the fixture's mode and
    /// size at the stat buffer, return the fixture's result word.
    fn libc_fake_fstat_r(c: &mut Cpu) {
        if c.bus.read32(FAKE_GATE) != FAKE_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(libc_fake_fstat_r),
                ee::s15::f_0041fbf0__generated,
            );
        }
        let at = c.g[A2][0] as u32;
        let mode = c.bus.read32(FAKE_GATE + FAKE_MODE_OFF);
        let size = c.bus.read64(FAKE_GATE + FAKE_SIZE_OFF);
        c.bus.write32(at.wrapping_add(4), mode);
        c.bus.write64(at.wrapping_add(0x10), size);
        c.g[V0][0] = sx(c.bus.read32(FAKE_GATE + FAKE_RESULT_OFF));
        c.cycles += 3;
        crate::abi::ret(c);
    }

    /// Hook the two fakes into their override slots (once per process;
    /// the gate word stays per-Cpu, so unarmed tests still run the
    /// generated bodies).
    #[allow(unused_unsafe)]
    pub(crate) fn libc_install_fakes() {
        unsafe {
            crate::difftest::fakes::hook(
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_0041FCB0),
                libc_fake_open_r,
                crate::difftest::fakes::prev_cell(libc_fake_open_r),
            );
            crate::difftest::fakes::hook(
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_0041FBF0),
                libc_fake_fstat_r,
                crate::difftest::fakes::prev_cell(libc_fake_fstat_r),
            );
        }
    }

    /// `_sbrk_r` fake gate: the magic word, a call counter, then one
    /// scripted result per call (0 runs the real body for that call).
    pub(crate) const SBRK_GATE: u32 = DATA + 0x5040;

    /// Gated `_sbrk_r` (slot `0x41fd70`): call n returns script word n,
    /// sign-extended (`u32::MAX` is the failure). The real `sbrk` never
    /// fails a shrink or moves the break between calls.
    fn libc_fake_sbrk_r(c: &mut Cpu) {
        let armed = c.bus.read32(SBRK_GATE) == FAKE_MAGIC;
        let n = c.bus.read32(SBRK_GATE + 4);
        let word = if armed && n < 4 {
            c.bus.read32(SBRK_GATE + 8 + 4 * n)
        } else {
            0
        };
        if armed {
            c.bus.write32(SBRK_GATE + 4, n + 1);
        }
        if word == 0 {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(libc_fake_sbrk_r),
                ee::s15::f_0041fd70__generated,
            );
        }
        c.g[V0][0] = word as i32 as i64 as u64;
        c.cycles += 3;
        crate::abi::ret(c);
    }

    /// Hook the `_sbrk_r` fake (once per process) and arm its script.
    #[allow(unused_unsafe)]
    pub(crate) fn libc_arm_sbrk(c: &mut Cpu, script: &[u32]) {
        unsafe {
            crate::difftest::fakes::hook(
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_0041FD70),
                libc_fake_sbrk_r,
                crate::difftest::fakes::prev_cell(libc_fake_sbrk_r),
            );
        }
        c.bus.write32(SBRK_GATE, FAKE_MAGIC);
        c.bus.write32(SBRK_GATE + 4, 0);
        for i in 0..4 {
            let w = script.get(i).copied().unwrap_or(0);
            c.bus.write32(SBRK_GATE + 8 + 4 * i as u32, w);
        }
    }

    /// Arm the gate with the fd, mode and size the fakes report.
    pub(crate) fn libc_arm_fakes(c: &mut Cpu, fd: u32, mode: u32, size: u64) {
        libc_install_fakes();
        c.bus.write32(FAKE_GATE, FAKE_MAGIC);
        c.bus.write32(FAKE_GATE + FAKE_FD_OFF, fd);
        c.bus.write32(FAKE_GATE + FAKE_MODE_OFF, mode);
        c.bus.write64(FAKE_GATE + FAKE_SIZE_OFF, size);
        c.bus.write32(FAKE_GATE + FAKE_RESULT_OFF, 0);
    }

    #[test]
    fn fopen_r_matches() {
        check(ee::s15::f_00416eb0__generated, fopen_r, 200, &|c, seed| {
            let mut r = file_fixture(c, seed, 0x6eb0, 3);
            let mode = match seed % 5 {
                0 => &b"r"[..],
                1 => &b"w"[..],
                2 => &b"a"[..],
                3 => &b"q"[..], // invalid: NULL without touching slots
                _ => &b"r+"[..],
            };
            for (i, b) in mode.iter().enumerate() {
                c.bus.write8(FMT + i as u32, *b);
            }
            c.bus.write8(FMT + mode.len() as u32, 0);
            c.bus.write8(SRC, b'x');
            c.bus.write8(SRC + 1, 0);
            if seed % 7 == 0 {
                // No free slot: every `FILE` busy.
                c.bus.write16(FILE_2 + 0xc, 1);
            }
            c.g[A0][0] = REENT_3 as u64;
            c.g[A1][0] = SRC as u64;
            c.g[A2][0] = FMT as u64;
            let _ = &mut r;
        });
    }

    #[test]
    fn fopen_r_open_success_matches() {
        // `_open_r` reporting fd 3 reaches the `FILE` linkage setup
        // (cookie words, flags, the stored fd); mode "a" additionally
        // runs the append `fseek` on the fresh `FILE`.
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = file_fixture(c, seed, 0x6eb0, 3);
            libc_arm_fakes(c, 3, 0, 0);
            let mode = if seed % 2 == 0 { &b"r"[..] } else { &b"a"[..] };
            for (i, b) in mode.iter().enumerate() {
                c.bus.write8(FMT + i as u32, *b);
            }
            c.bus.write8(FMT + mode.len() as u32, 0);
            c.bus.write8(SRC, b'x');
            c.bus.write8(SRC + 1, 0);
            c.g[A0][0] = REENT_3 as u64;
            c.g[A1][0] = SRC as u64;
            c.g[A2][0] = FMT as u64;
            let _ = &mut r;
        };
        check_paths(
            ee::s15::f_00416eb0__generated,
            fopen_r,
            60,
            &[2735, 2956],
            &setup,
        );
    }

    #[test]
    fn fwalk_matches() {
        use crate::difftest::fakes::{
            fake_call, fake_jump_out, fake_syscall, FAKE, LOG_RESULTS, LOG_UNWIND_AT,
        };
        check(ee::s15::f_00417418__generated, fwalk, 200, &|c, seed| {
            let mut r = file_fixture(c, seed, 0x7418, 3);
            let take = seed % 5;
            if take == 0 {
                // No glue at all.
                c.bus.write32(REENT_3 + 0x1d8, 0);
            } else if take == 1 {
                // `FILE` closed: the callback is skipped.
                c.bus.write16(FILE_2 + 0xc, 0);
            } else if take == 4 {
                // Nonzero callback results: the walked callback is a fake
                // returning nonzero, over two open `FILE`s, so the result
                // accumulation observes both arms.
                c.hooks.miss = fake_call;
                c.hooks.jump_out = fake_jump_out;
                c.hooks.syscall = fake_syscall;
                c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                c.bus.write32(LOG_RESULTS, 3);
                c.bus.write32(LOG_RESULTS + 4, 3);
                c.bus.write16(FILE_2 + 0x58 + 0xc, 0x200);
                c.bus.write32(GLUE + 0x4, 2);
                c.g[A1][0] = FAKE as u64;
            }
            c.g[A0][0] = REENT_3 as u64;
            // A real leaf as the walked callback.
            if take != 4 {
                c.g[A1][0] = 0x0014_06c0;
            }
            let _ = &mut r;
        });
    }

    #[test]
    fn fwalk_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = file_fixture(c, seed, 0x7418, 3);
            // Three open `FILE`s so the inner loop runs past event checks.
            c.bus.write32(GLUE + 0x4, 3);
            c.g[A0][0] = REENT_3 as u64;
            c.g[A1][0] = 0x0014_06c0;
            schedule(c, &mut r, seed, 0x0041_7418)
        };
        check_events(ee::s15::f_00417418__generated, fwalk, 60, &setup);
    }

    #[test]
    fn printf_r_matches() {
        check(ee::s15::f_00417578__generated, printf_r, 120, &|c, seed| {
            let (s, _) = printf_case(c, seed, 0x7578);
            c.g[A0][0] = REENT_3 as u64;
            c.g[A1][0] = FMT as u64;
            c.g[A2][0] = s[0] as u64;
            c.g[A3][0] = s[1] as u64;
            c.g[T0][0] = s[2] as u64;
        });
    }

    #[test]
    fn sprintf_r_matches() {
        check(
            ee::s15::f_004177b0__generated,
            sprintf_r,
            120,
            &|c, seed| {
                let (s, _) = printf_case(c, seed, 0x77b0);
                c.g[A0][0] = REENT_3 as u64;
                c.g[A1][0] = OUT_2 as u64;
                c.g[A2][0] = FMT as u64;
                c.g[A3][0] = s[0] as u64;
                c.g[T0][0] = s[1] as u64;
                c.g[T1][0] = s[2] as u64;
            },
        );
    }

    #[test]
    fn strtoul_r_matches() {
        check(
            ee::s15::f_0041a840__generated,
            strtoul_r,
            200,
            &|c, seed| {
                let mut r = Rng(0xa840 ^ seed as u64);
                // Real ctype table from the image: digit classification
                // drives the conversion loop.
                wire_reent(c, &mut r);
                c.pc = 0x0041_a840;
                let s: &[u8] = match seed % 10 {
                    0 => b"123",
                    1 => b"  -45",
                    2 => b"0x1F",
                    3 => b"010",
                    4 => b"zz",
                    5 => b"",
                    6 => b"18446744073709551615", // ULONG_MAX.
                    7 => b"18446744073709551616", // overflow.
                    8 => b"101",
                    _ => b"  +0Xab",
                };
                place_str(c, SRC, s);
                c.g[A0][0] = REENT_3 as u64;
                c.g[A1][0] = SRC as u64;
                c.g[A2][0] = if seed % 2 == 0 { 0 } else { SCR as u64 };
                c.g[A3][0] = [0, 0, 8, 10, 16, 2, 36][seed as usize % 7] as u64;
            },
        );
    }

    #[test]
    fn strtoul_r_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0xa840 ^ seed as u64);
            // Real ctype table from the image (see above).
            wire_reent(c, &mut r);
            // A long digit run so the conversion loop passes event checks.
            for i in 0..128 {
                c.bus.write8(SRC + i as u32, b'9' - (i % 3) as u8);
            }
            c.bus.write8(SRC + 128, 0);
            c.g[A0][0] = REENT_3 as u64;
            c.g[A1][0] = SRC as u64;
            c.g[A2][0] = 0;
            c.g[A3][0] = 10;
            schedule(c, &mut r, seed, 0x0041_a840)
        };
        check_events(ee::s15::f_0041a840__generated, strtoul_r, 60, &setup);
    }

    #[test]
    fn round_digits_matches() {
        check(
            ee::s15::f_0041ae00__generated,
            vfprintf_round_digits,
            200,
            &|c, seed| {
                let mut r = Rng(0xae00 ^ seed as u64);
                scramble(c, &mut r);
                let digits: &[u8] = match seed % 8 {
                    0 => b"5",
                    1 => b"49",
                    2 => b"59",
                    3 => b"1999",
                    4 => b"999",
                    5 => b"12345",
                    6 => b"12349",
                    _ => b"9",
                };
                place_str(c, DST, digits);
                c.g[A0][0] = DST as u64;
                c.g[A1][0] = (match seed % 5 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    _ => digits.len(),
                }) as u64;
            },
        );
    }

    #[test]
    fn round_digits_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0xae00 ^ seed as u64);
            scramble(c, &mut r);
            // A long carry run: the ripple loop passes event checks.
            for i in 0..128 {
                c.bus.write8(DST + i as u32, b'9');
            }
            c.bus.write8(DST + 128, 0);
            c.g[A0][0] = DST as u64;
            c.g[A1][0] = 100;
            schedule(c, &mut r, seed, 0x0041_ae00)
        };
        check_events(
            ee::s15::f_0041ae00__generated,
            vfprintf_round_digits,
            60,
            &setup,
        );
    }

    #[test]
    fn str_reverse_matches() {
        check(
            ee::s15::f_0041ae70__generated,
            vfprintf_str_reverse,
            200,
            &|c, seed| {
                let mut r = Rng(0xae70 ^ seed as u64);
                scramble(c, &mut r);
                let s: &[u8] = match seed % 7 {
                    0 => b"",
                    1 => b"a",
                    2 => b"ab",
                    3 => b"abc",
                    4 => b"1234567",
                    5 => b"0123456789abcdef",
                    _ => &[b'w'; 64],
                };
                place_str(c, SRC, s);
                c.g[A0][0] = SRC as u64;
            },
        );
    }

    #[test]
    fn str_reverse_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0xae70 ^ seed as u64);
            scramble(c, &mut r);
            for i in 0..256 {
                c.bus.write8(SRC + i as u32, b'r' + (i % 10) as u8);
            }
            c.bus.write8(SRC + 256, 0);
            c.g[A0][0] = SRC as u64;
            schedule(c, &mut r, seed, 0x0041_ae70)
        };
        check_events(
            ee::s15::f_0041ae70__generated,
            vfprintf_str_reverse,
            60,
            &setup,
        );
    }

    #[test]
    fn int_to_str_matches() {
        check(
            ee::s15::f_0041aee8__generated,
            vfprintf_int_to_str,
            200,
            &|c, seed| {
                let mut r = Rng(0xaee8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = [
                    0u64,
                    1,
                    9,
                    10,
                    99,
                    255,
                    1000,
                    65535,
                    123456789,
                    0xffff_ffff,
                    0x8000_0001,
                    0x7fff_ffff,
                ][seed as usize % 12];
                c.g[A1][0] = DST as u64;
                c.g[A2][0] = [8, 10, 10, 16][seed as usize % 4] as u64;
                for i in 0..0x40 {
                    c.bus.write8(DST + i as u32, 0xcc);
                }
            },
        );
    }

    #[test]
    fn int_to_str_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0xaee8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = 0xffff_ffff;
            c.g[A1][0] = DST as u64;
            c.g[A2][0] = 8; // octal: eleven digits of loop.
            for i in 0..0x40 {
                c.bus.write8(DST + i as u32, 0xcc);
            }
            schedule(c, &mut r, seed, 0x0041_aee8)
        };
        check_events(
            ee::s15::f_0041aee8__generated,
            vfprintf_int_to_str,
            60,
            &setup,
        );
    }

    #[test]
    fn flush_file_matches() {
        check(
            ee::s15::f_0041b528__generated,
            vfprintf_flush_file,
            120,
            &|c, seed| {
                let mut r = Rng(0xb528 ^ seed as u64);
                wire_reent(c, &mut r);
                // `_file` descriptor; the flush writes through the scratch
                // `FILE`'s own write slot.
                c.bus.write16(SRC + 0xe, [3, 1, 2][seed as usize % 3]);
                c.g[A0][0] = SRC as u64;
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn buffered_write_matches() {
        check(
            ee::s15::f_0041b550__generated,
            vfprintf_buffered_write,
            200,
            &|c, seed| {
                let mut r = Rng(0xb550 ^ seed as u64);
                wire_reent(c, &mut r);
                c.pc = 0x0041_b550;
                // Static buffer state: count + write cursor.
                c.bus
                    .write32(0x0045_4cd8, [0, 5, 126, 127][seed as usize % 4]);
                c.bus.write32(0x0045_4cdc, DST);
                for i in 0..0x200 {
                    c.bus.write8(DST + i as u32, 0xcc);
                    c.bus.write8(SRC + i as u32, b'k' + (i % 7) as u8);
                }
                c.g[A0][0] = 3; // fd with no host side effects.
                c.g[A1][0] = SRC as u64;
                c.g[A2][0] = [0, 1, 10, 64, 130][seed as usize % 5] as u64;
                c.g[A3][0] = [0, 1, 2][seed as usize % 3] as u64;
            },
        );
    }

    #[test]
    fn buffered_write_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0xb550 ^ seed as u64);
            wire_reent(c, &mut r);
            c.bus.write32(0x0045_4cd8, 0);
            c.bus.write32(0x0045_4cdc, DST);
            for i in 0..0x400 {
                c.bus.write8(DST + i as u32, 0xcc);
                c.bus.write8(SRC + i as u32, b'q');
            }
            c.g[A0][0] = 3;
            c.g[A1][0] = SRC as u64;
            c.g[A2][0] = 200;
            c.g[A3][0] = 0;
            schedule(c, &mut r, seed, 0x0041_b550)
        };
        check_events(
            ee::s15::f_0041b550__generated,
            vfprintf_buffered_write,
            60,
            &setup,
        );
    }

    #[test]
    fn insert_thousands_sep_matches() {
        check(
            ee::s15::f_0041c788__generated,
            vfprintf_insert_thousands_sep,
            200,
            &|c, seed| {
                let mut r = Rng(0xc788 ^ seed as u64);
                scramble(c, &mut r);
                let s: &[u8] = match seed % 12 {
                    0 => b"12345",
                    1 => b"1234567",
                    2 => b"12.5",
                    3 => b"1e10",
                    4 => b"1E5",
                    5 => b"123",
                    6 => b"12",
                    7 => b"1",
                    8 => b"123456789012",
                    9 => b"0.5",
                    10 => b"",
                    _ => b"12F34",
                };
                place_str(c, SRC, s);
                for i in 0..0x80 {
                    c.bus.write8(DST + i as u32, 0xcc);
                }
                c.g[A0][0] = DST as u64;
                c.g[A1][0] = SRC as u64;
                c.g[A2][0] = (SRC + s.len() as u32) as u64;
                c.g[A3][0] = SCR as u64;
                c.bus.write32(SCR, 0);
            },
        );
    }

    #[test]
    fn insert_thousands_sep_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0xc788 ^ seed as u64);
            scramble(c, &mut r);
            for i in 0..200 {
                c.bus.write8(SRC + i as u32, b'0' + (i % 10) as u8);
            }
            c.bus.write8(SRC + 200, 0);
            for i in 0..0x200 {
                c.bus.write8(DST + i as u32, 0xcc);
            }
            c.g[A0][0] = DST as u64;
            c.g[A1][0] = SRC as u64;
            c.g[A2][0] = (SRC + 200) as u64;
            c.g[A3][0] = SCR as u64;
            c.bus.write32(SCR, 0);
            schedule(c, &mut r, seed, 0x0041_c788)
        };
        check_events(
            ee::s15::f_0041c788__generated,
            vfprintf_insert_thousands_sep,
            60,
            &setup,
        );
    }

    #[test]
    fn strtol_r_matches() {
        check(ee::s15::f_0041e7a8__generated, strtol_r, 200, &|c, seed| {
            let mut r = Rng(0xe7a8 ^ seed as u64);
            // Real ctype table from the image: without it the digit
            // loop classifies everything as NUL and breaks immediately.
            wire_reent(c, &mut r);
            c.pc = 0x0041_e7a8;
            let s: &[u8] = match seed % 14 {
                0 => b"123",
                1 => b"  -45",
                2 => b"0x1F",
                3 => b"010",
                4 => b"zz",
                5 => b"",
                6 => b"9223372036854775807",     // LONG_MAX.
                7 => b"-9223372036854775808",    // LONG_MIN.
                8 => b"9999999999999999999999",  // overflow.
                9 => b"-9999999999999999999999", // negative overflow.
                10 => b"101",
                11 => b"  +0Xab",
                12 => b"9223372036854775808", // LONG_MAX + 1: acc == cutoff,
                13 => b"-9223372036854775809", // LONG_MIN - 1: digit != cutlim.
                _ => b"  +0Xab",
            };
            place_str(c, SRC, s);
            c.g[A0][0] = REENT_3 as u64;
            c.g[A1][0] = SRC as u64;
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { SCR as u64 };
            // Letter strings need digit-valid bases: `zz` in base 36,
            // `0Xab` in base 16 (exercises the hex-adjust parity arm).
            // The base is its own seed digit, so every string meets
            // base 0 (`010` as octal) and base 10 (`LONG_MAX` exactly
            // at cutoff and cutlim).
            c.g[A3][0] = match seed % 14 {
                4 => 36,
                11 => 16,
                12 | 13 => 10,
                _ => [0, 0, 8, 10, 16, 2, 36][(seed / 14) as usize % 7] as u64,
            };
        });
    }

    #[test]
    fn strtol_r_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0xe7a8 ^ seed as u64);
            wire_reent(c, &mut r);
            for i in 0..128 {
                c.bus.write8(SRC + i as u32, b'7');
            }
            c.bus.write8(SRC + 128, 0);
            c.g[A0][0] = REENT_3 as u64;
            c.g[A1][0] = SRC as u64;
            c.g[A2][0] = 0;
            c.g[A3][0] = 10;
            schedule(c, &mut r, seed, 0x0041_e7a8)
        };
        check_events(ee::s15::f_0041e7a8__generated, strtol_r, 60, &setup);
    }

    #[test]
    fn free_r_matches() {
        check(ee::s15::f_0041e340__generated, free_r, 120, &|c, seed| {
            let mut r = Rng(0xe340 ^ seed as u64);
            match seed % 6 {
                0 => {
                    // `free(NULL)` is a no-op.
                    heap_init(c, &mut r);
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = 0;
                }
                1 => {
                    // One fresh block, no history.
                    heap_init(c, &mut r);
                    let size = [8, 16, 64, 256, 1024][seed as usize % 5] as u64
                        + (r.next_u32() % 32) as u64;
                    c.g[A0][0] = size;
                    ee::s15::f_004188c0__generated(c);
                    assert_ne!(c.g[V0][0], 0, "seed {seed}: malloc failed");
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = c.g[V0][0];
                }
                2 => {
                    // A non-top 0x200 chunk: slow path, T4 == 0.
                    heap_init(c, &mut r);
                    c.g[A0][0] = 0x1f8;
                    ee::s15::f_004188c0__generated(c);
                    let kept = c.g[V0][0];
                    assert_ne!(kept, 0, "seed {seed}: malloc failed");
                    c.g[A0][0] = 16;
                    ee::s15::f_004188c0__generated(c);
                    assert_ne!(c.g[V0][0], 0, "seed {seed}: malloc failed");
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = kept;
                }
                _ => {
                    // A kept block in a varied heap: consolidation with free
                    // neighbors, binned chunks, top and large chunks.
                    let kept = heap_history(c, seed, 0xe340);
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = kept as u64;
                }
            }
            let _ = &mut r;
        });
    }

    #[test]
    fn free_r_events() {
        // Same-size large bin: the insert walk passes the `0x41e5e8`
        // edge with a due VBlank.
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0xe341 ^ seed as u64);
            heap_init(c, &mut r);
            let mut ps = [0u64; 6];
            for p in ps.iter_mut() {
                c.g[A0][0] = 0xaf8;
                ee::s15::f_004188c0__generated(c);
                assert_ne!(c.g[V0][0], 0, "seed {seed}: malloc failed");
                *p = c.g[V0][0];
            }
            for &i in &[0usize, 2] {
                c.g[A0][0] = REENT_3 as u64;
                c.g[A1][0] = ps[i];
                ee::s15::f_0041e340__generated(c);
            }
            c.g[A0][0] = REENT_3 as u64;
            c.g[A1][0] = ps[4];
            let _ = &mut r;
            heap_schedule(c, &mut r, seed, 0x0041_e340)
        };
        check_events(ee::s15::f_0041e340__generated, free_r, 60, &setup);
    }

    #[test]
    fn free_r_bins() {
        // Bin-index ladder bands, a small bin holding 2+ chunks, and
        // non-empty large bins (insert walk taken and skipped).
        // Each arm frees an isolated (T4 == 0) chunk so the freed chunk
        // goes through the bin-index/insert region.
        check(ee::s15::f_0041e340__generated, free_r, 23, &|c, seed| {
            let mut r = Rng(0xe341 ^ seed as u64);
            heap_init(c, &mut r);
            match seed {
                0 => {
                    // Small bin with 2 occupants, then free a third.
                    // Size 32 (T1 = 0x28, A3 = 5): the third free takes
                    // the `(V0+3)>>2` index arm.
                    let mut ps = [0u64; 10];
                    for p in ps.iter_mut() {
                        c.g[A0][0] = 32;
                        ee::s15::f_004188c0__generated(c);
                        assert_ne!(c.g[V0][0], 0, "seed {seed}: malloc failed");
                        *p = c.g[V0][0];
                    }
                    for &i in &[2usize, 6] {
                        c.g[A0][0] = REENT_3 as u64;
                        c.g[A1][0] = ps[i];
                        ee::s15::f_0041e340__generated(c);
                    }
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = ps[8];
                }
                6 | 7 | 20 | 21 => {
                    // Same large bin (V1 == 5), two occupants; then free
                    // a smaller (walk taken, seed 6) or larger (walk
                    // skipped, seed 7) isolated chunk into it. Seeds
                    // 20/21: the bin holds [0xB80, 0xA00] / [0xB80, 0xB00]
                    // and freeing 0xB00 walks past 0xB80, then stops at a
                    // smaller (20) or equal-size (21) second occupant.
                    let sizes = match seed {
                        // Larger into smaller: bin holds 0xA00s instead.
                        7 => [0x9f8u64, 16, 0x9f8, 16, 0xaf8, 16],
                        20 => [0xb78u64, 16, 0x9f8, 16, 0xaf8, 16],
                        21 => [0xb78u64, 16, 0xaf8, 16, 0xaf8, 16],
                        _ => [0xaf8u64, 16, 0xaf8, 16, 0x9f8, 16],
                    };
                    let mut ps = [0u64; 6];
                    for (p, &s) in ps.iter_mut().zip(sizes.iter()) {
                        c.g[A0][0] = s;
                        ee::s15::f_004188c0__generated(c);
                        assert_ne!(c.g[V0][0], 0, "seed {seed}: malloc failed");
                        *p = c.g[V0][0];
                    }
                    for &i in &[0usize, 2] {
                        c.g[A0][0] = REENT_3 as u64;
                        c.g[A1][0] = ps[i];
                        ee::s15::f_0041e340__generated(c);
                    }
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = ps[4];
                }
                8 => {
                    // Empty small bin: first free of a 0x28 chunk sets
                    // a clear bit (occupied bins hold a set bit, so the
                    // two index paths diverge here).
                    c.g[A0][0] = 32;
                    ee::s15::f_004188c0__generated(c);
                    let kept = c.g[V0][0];
                    assert_ne!(kept, 0, "seed {seed}: malloc failed");
                    c.g[A0][0] = 16;
                    ee::s15::f_004188c0__generated(c);
                    assert_ne!(c.g[V0][0], 0, "seed {seed}: malloc failed");
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = kept;
                }
                9 => {
                    // Same-size large bin: freeing a 0xB00 chunk into a
                    // bin holding 0xB00 hits `T1 == masked` exactly
                    // (the equal-size walk arm).
                    let mut ps = [0u64; 6];
                    for p in ps.iter_mut() {
                        c.g[A0][0] = 0xaf8;
                        ee::s15::f_004188c0__generated(c);
                        assert_ne!(c.g[V0][0], 0, "seed {seed}: malloc failed");
                        *p = c.g[V0][0];
                    }
                    for &i in &[0usize, 2] {
                        c.g[A0][0] = REENT_3 as u64;
                        c.g[A1][0] = ps[i];
                        ee::s15::f_0041e340__generated(c);
                    }
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = ps[4];
                }
                10 | 19 => {
                    // Empty large bin with A3 % 4 != 0: the
                    // `(V1+3)>>2` vs `V1>>2` index paths diverge on
                    // unaligned bands. Targets the +0x38 band (V1 == 1);
                    // the +0x5b band is covered by the same mechanism.
                    // Seed 19: T1 = 0xAAA00 puts V1 == 0x555 on the last
                    // rung (the `< 0x555` arms cost 1 vs 2 cycles).
                    let t1 = if seed == 19 { 0xaaa00u64 } else { 0x240u64 };
                    c.g[A0][0] = t1 - 8;
                    ee::s15::f_004188c0__generated(c);
                    let kept = c.g[V0][0];
                    assert_ne!(kept, 0, "seed {seed}: malloc failed");
                    c.g[A0][0] = 16;
                    ee::s15::f_004188c0__generated(c);
                    assert_ne!(c.g[V0][0], 0, "seed {seed}: malloc failed");
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = kept;
                }
                11 => {
                    // Huge chunk (T1 > 0x20000 trim threshold): the
                    // fast-path `T1 < A1` check is false (exact
                    // T1 == A1 never happens: large T1 are
                    // 0xFF8 mod 0x1000, small T1 are below threshold).
                    c.g[A0][0] = 0x30000;
                    ee::s15::f_004188c0__generated(c);
                    let kept = c.g[V0][0];
                    assert_ne!(kept, 0, "seed {seed}: malloc failed");
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = kept;
                }
                12 => {
                    // Adjacent frees: free two neighbors in order so the
                    // second free finds a free predecessor (T4 == 1,
                    // prev-consolidation path).
                    let mut ps = [0u64; 4];
                    for p in ps.iter_mut() {
                        c.g[A0][0] = 64;
                        ee::s15::f_004188c0__generated(c);
                        assert_ne!(c.g[V0][0], 0, "seed {seed}: malloc failed");
                        *p = c.g[V0][0];
                    }
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = ps[0];
                    ee::s15::f_0041e340__generated(c);
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = ps[1];
                }
                13 | 14 | 22 => {
                    // Backward frees: free the later neighbor first so
                    // the tested free absorbs a free successor (next
                    // chunk free). Seed 13 has an allocated predecessor
                    // (T4 == 0); seed 14 also frees the predecessor
                    // first (prev-consolidation, still T4 == 0: only a
                    // last-remainder neighbor sets T4). Seed 22 is seed
                    // 14 with an odd prev_size (0x48 - 3): the absorbed
                    // T1 = 0xd5 is odd, so `T1 | 1` != `T1 ^ 1`.
                    let mut ps = [0u64; 4];
                    for p in ps.iter_mut() {
                        c.g[A0][0] = 64;
                        ee::s15::f_004188c0__generated(c);
                        assert_ne!(c.g[V0][0], 0, "seed {seed}: malloc failed");
                        *p = c.g[V0][0];
                    }
                    if seed != 13 {
                        c.g[A0][0] = REENT_3 as u64;
                        c.g[A1][0] = ps[0];
                        ee::s15::f_0041e340__generated(c);
                    }
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = ps[2];
                    ee::s15::f_0041e340__generated(c);
                    if seed == 22 {
                        let at = ps[1] as u32 - 8;
                        let prev = c.bus.read32(at);
                        c.bus.write32(at, prev - 3);
                    }
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = ps[1];
                }
                15 | 18 => {
                    // Last-remainder predecessor: a free chunk whose `fd`
                    // is the `0x454cf0` sentinel, next to an allocated
                    // successor. Freeing the middle chunk merges the
                    // sentinel predecessor (T4 == 1), skipping the four
                    // bin-link stores.
                    let mut ps = [0u64; 4];
                    for p in ps.iter_mut() {
                        c.g[A0][0] = 64;
                        ee::s15::f_004188c0__generated(c);
                        assert_ne!(c.g[V0][0], 0, "seed {seed}: malloc failed");
                        *p = c.g[V0][0];
                    }
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = ps[1];
                    ee::s15::f_0041e340__generated(c);
                    // Forge the sentinel: `fd` of a free chunk is at
                    // the user address itself (chunk + 8).
                    c.bus.write32(ps[1] as u32, 0x0045_4cf0);
                    if seed == 18 {
                        // Odd prev_size (0x48 - 3): T2 = chunk1 + 3 still
                        // reads the forged fd (loads align down), and the
                        // merged T1 = 0x8d is odd, so `T1 | 1` != `T1 ^ 1`
                        // in the stored size (next chunk in use).
                        let at = ps[2] as u32 - 8;
                        let prev = c.bus.read32(at);
                        c.bus.write32(at, prev - 3);
                    }
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = ps[2];
                }
                16 => {
                    // Top merge with a free predecessor whose prev_size
                    // is odd (real - 3): the merged top size T1 is odd,
                    // so `T1 | 1` != `T1 ^ 1`. T2 = chunk0 + 3 still finds
                    // chunk0's fd/bk (loads align down).
                    let mut ps = [0u64; 2];
                    for p in ps.iter_mut() {
                        c.g[A0][0] = 64;
                        ee::s15::f_004188c0__generated(c);
                        assert_ne!(c.g[V0][0], 0, "seed {seed}: malloc failed");
                        *p = c.g[V0][0];
                    }
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = ps[0];
                    ee::s15::f_0041e340__generated(c);
                    let at = ps[1] as u32 - 8;
                    let prev = c.bus.read32(at);
                    c.bus.write32(at, prev - 3);
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = ps[1];
                }
                17 => {
                    // Top merge whose size equals the trim threshold
                    // (forged at 0x4550f0): `T1 < A1` is false, so
                    // malloc_trim runs.
                    c.g[A0][0] = 64;
                    ee::s15::f_004188c0__generated(c);
                    let kept = c.g[V0][0];
                    assert_ne!(kept, 0, "seed {seed}: malloc failed");
                    let ch = kept as u32 - 8;
                    let size = c.bus.read32(ch + 4) & !1;
                    let top = c.bus.read32(0x0045_4cf0);
                    assert_eq!(top, ch + size, "seed {seed}: not next to top");
                    let t1 = size.wrapping_add(c.bus.read32(top + 4) & !3);
                    c.bus.write64(0x0045_50f0, t1 as u64);
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = kept;
                }
                _ => {
                    // Large bands: T1 = 0xA00, 0x2A00, 0xAA00, 0x2AA00,
                    // plus 0x200 as the empty-bin baseline.
                    let t1 = [0x200u64, 0xa00, 0x2a00, 0xaa00, 0x2aa00][seed as usize % 5];
                    c.g[A0][0] = t1 - 8;
                    ee::s15::f_004188c0__generated(c);
                    let kept = c.g[V0][0];
                    assert_ne!(kept, 0, "seed {seed}: malloc failed");
                    c.g[A0][0] = 16;
                    ee::s15::f_004188c0__generated(c);
                    assert_ne!(c.g[V0][0], 0, "seed {seed}: malloc failed");
                    c.g[A0][0] = REENT_3 as u64;
                    c.g[A1][0] = kept;
                }
            }
            let _ = &mut r;
        });
    }

    #[test]
    fn setlocale_r_matches() {
        check(
            ee::s15::f_0041ea38__generated,
            setlocale_r,
            200,
            &|c, seed| {
                let mut r = Rng(0xea38 ^ seed as u64);
                wire_reent(c, &mut r);
                c.g[A0][0] = REENT_3 as u64;
                c.g[A1][0] = [0, 1, 2, 3, 6][seed as usize % 5] as u64;
                let loc: &[u8] = match seed % 6 {
                    0 => b"C",
                    1 => b"",
                    2 => b"POSIX",
                    3 => b"xx", // unknown: NULL out.
                    4 => b"C.UTF-8",
                    _ => b"C",
                };
                if seed % 7 == 0 {
                    c.g[A2][0] = 0; // query the current setting.
                } else {
                    place_str(c, SRC, loc);
                    c.g[A2][0] = SRC as u64;
                }
            },
        );
    }

    #[test]
    fn _gcvt_matches() {
        check(ee::s15::f_00421b98__generated, _gcvt, 256, &|c, seed| {
            let mut r = Rng(0x1b98 ^ seed as u64);
            heap_init(c, &mut r);
            c.pc = 0x0042_1b98;
            for i in 0..0x200 {
                c.bus.write8(DST + i as u32, 0xcc);
            }
            // Stale words in the frame's `dtoa` out-slots (decpt, sign,
            // end) at `sp - 0xa0`: a store to the wrong slot leaves one.
            let sp = c.g[SP][0] as u32;
            for i in 0..0x10 {
                c.bus.write8(sp.wrapping_sub(0xa0) + i, 0x5a);
            }
            c.g[A0][0] = REENT_3 as u64;
            c.g[A1][0] = dbl_2(seed, &mut r);
            c.g[A2][0] = [1, 2, 3, 6, 10, 15, 17][seed as usize % 7] as u64;
            if seed % 8 == 6 {
                // Long digit string, tiny precision: the fraction
                // loop re-checks with `S1 == 0` and digits remaining.
                c.g[A1][0] = 123.456f64.to_bits();
                c.g[A2][0] = 3;
            } else if seed % 8 == 7 {
                c.g[A1][0] = 0.001f64.to_bits();
                c.g[A2][0] = 2;
            } else if seed % 16 == 8 {
                // The pre-batch pair this seed class displaced:
                // `FIXED[6]` (100.0) with ndigits 3 reaches the
                // zero-fill guard with `S1 > 0` (covers the guard
                // at seed 8).
                c.g[A1][0] = 100.0f64.to_bits();
                c.g[A2][0] = 3;
            } else if seed % 16 == 9 {
                // One digit with the point still ahead: the zero-fill
                // loop is skipped when `S1 <= 0`.
                c.g[A1][0] = 9.5f64.to_bits();
                c.g[A2][0] = 1;
            } else if seed % 16 == 10 {
                c.g[A1][0] = 99.5f64.to_bits();
                c.g[A2][0] = 2;
            } else if seed % 16 == 11 || seed % 16 == 12 {
                // Below 1 with a negative decpt and digits (mode 3): the
                // zero fill reaches the point with `S1 > 0` left, then
                // the digit loop copies (both alternate-form settings).
                let v = [0.00123f64, 0.0123, 0.000987, 0.0096][seed as usize / 16 % 4];
                c.g[A1][0] = v.to_bits();
                c.g[A2][0] = [5, 4, 6, 2][seed as usize / 64 % 4];
            } else if seed % 16 == 13 {
                // `ndigits` whose high word is not the sign extension:
                // dtoa and `_mprec_log10` use the low word, the raw
                // `S1 > 0` test at 0x421dac fails, so a negative decpt
                // skips the zero fill into the digit loop at 0x421dec.
                let v = [0.00123f64, 0.0123, 0.000987, 0.0096][seed as usize / 16 % 4];
                c.g[A1][0] = v.to_bits();
                c.g[A2][0] = 0xffff_ffff_0000_0000 | [5u64, 4, 6, 2][seed as usize / 64 % 4];
            }
            c.g[A3][0] = DST as u64;
            c.g[T0][0] = [0, 1][seed as usize % 2] as u64;
            c.g[T1][0] = [0, 1][seed as usize % 2] as u64;
        });
    }

    #[test]
    fn _gcvt_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0x1b98 ^ seed as u64);
            heap_init(c, &mut r);
            for i in 0..0x200 {
                c.bus.write8(DST + i as u32, 0xcc);
            }
            c.g[A0][0] = REENT_3 as u64;
            c.g[A1][0] = 0.3333333333333333f64.to_bits();
            c.g[A2][0] = 17;
            c.g[A3][0] = DST as u64;
            c.g[T0][0] = 1;
            c.g[T1][0] = 1;
            heap_schedule(c, &mut r, seed, 0x0042_1b98)
        };
        check_events(ee::s15::f_00421b98__generated, _gcvt, 60, &setup);
    }

    #[test]
    fn calloc_r_matches() {
        check(ee::s15::f_004224a0__generated, calloc_r, 120, &|c, seed| {
            let mut r = Rng(0x24a0 ^ seed as u64);
            heap_init(c, &mut r);
            c.g[A0][0] = REENT_3 as u64;
            c.g[A1][0] = [0, 1, 2, 3, 5, 8, 64, 1024, 65536][seed as usize % 9] as u64;
            c.g[A2][0] = [0, 1, 2, 3, 4, 5, 16, 36, 256][(seed as usize / 9) % 9] as u64;
        });
    }

    #[test]
    fn abort_matches() {
        // `_exit` (the installed `crt_exit`) never comes back on the test
        // machine, so both sides take the early return with the same state.
        check(ee::s15::f_00418790__generated, abort, 30, &|c, seed| {
            let mut r = Rng(0x8790 ^ seed as u64);
            scramble(c, &mut r);
            let _ = &mut r;
        });
    }

    #[test]
    fn atof_matches() {
        check(ee::s15::f_004187a0__generated, atof, 120, &|c, seed| {
            let mut r = Rng(0x87a0 ^ seed as u64);
            scramble(c, &mut r);
            // `atof` funnels into `strtod`, which loads the global reent:
            // wire it or both sides take the same early out (vacuous).
            wire_reent(c, &mut r);
            let s: &[u8] = match seed % 5 {
                0 => b"3.25",
                1 => b"-0.5e2",
                2 => b"abc",
                3 => b"1e-3",
                _ => b"  42",
            };
            for (i, b) in s.iter().enumerate() {
                c.bus.write8(SRC + i as u32, *b);
            }
            c.bus.write8(SRC + s.len() as u32, 0);
            c.g[A0][0] = SRC as u64;
            let _ = &mut r;
        });
    }

    #[test]
    fn atoi_matches() {
        check(ee::s15::f_004187c0__generated, atoi, 200, &|c, seed| {
            let mut r = Rng(0x87c0 ^ seed as u64);
            scramble(c, &mut r);
            // `atoi` funnels into `strtol`, which loads the global reent:
            // wire it or both sides take the same early out (vacuous).
            wire_reent(c, &mut r);
            let s: &[u8] = match seed % 6 {
                0 => b"123",
                1 => b"-45",
                2 => b"  78xy",
                3 => b"abc",
                4 => b"0x10",
                _ => b"2147483648",
            };
            for (i, b) in s.iter().enumerate() {
                c.bus.write8(SRC + i as u32, *b);
            }
            c.bus.write8(SRC + s.len() as u32, 0);
            c.g[A0][0] = SRC as u64;
            let _ = &mut r;
        });
    }

    #[test]
    fn bsearch_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0x87e8 ^ seed as u64);
            scramble(c, &mut r);
            for i in 0..64 {
                c.bus.write32(SRC + i * 4, (i * 3) as u32);
            }
            c.bus.write32(DST, 9);
            c.g[A0][0] = DST as u64;
            c.g[A1][0] = SRC as u64;
            c.g[A2][0] = 64;
            c.g[A3][0] = 4;
            c.bus.write8(DST + 4, b'9');
            c.bus.write8(DST + 5, 0);
            c.g[T0][0] = 0x0041_87c0;
            schedule(c, &mut r, seed, 0x0041_87e8)
        };
        check_events(ee::s15::f_004187e8__generated, bsearch, 60, &setup);
    }

    #[test]
    fn bsearch_matches() {
        check(ee::s15::f_004187e8__generated, bsearch, 200, &|c, seed| {
            let mut r = Rng(0x87e8 ^ seed as u64);
            scramble(c, &mut r);
            // The `atoi` comparator funnels into `strtol`: wire the reent.
            wire_reent(c, &mut r);
            c.pc = 0x0041_87e8;
            for i in 0..NITEMS {
                c.bus.write32(SRC + i * 4, (i * 7) as u32);
            }
            let key = match seed % 6 {
                0 => 0,   // first element
                1 => 77,  // last element
                2 => 21,  // middle element
                3 => 22,  // absent, inside the range
                4 => 200, // absent, past the end
                _ => (r.next_u32() % 90) as u32,
            };
            c.bus.write32(DST, key);
            c.g[A0][0] = DST as u64;
            c.g[A1][0] = SRC as u64;
            c.g[A2][0] = (match seed % 5 {
                0 => NITEMS,
                1 => 0, // empty: NULL
                2 => 1,
                _ => NITEMS,
            }) as u64;
            c.g[A3][0] = 4;
            // Comparator: 0 for even seeds (mid-element hit); `atoi` of a
            // digit-string key for odd seeds (drives less, equal, greater).
            if seed % 2 == 0 {
                c.g[T0][0] = 0x0014_06c0;
            } else {
                let s: &[u8] = match (seed / 2) % 3 {
                    0 => b"-5",
                    1 => b"42",
                    _ => b"abc",
                };
                place_str(c, DST, s);
                c.g[T0][0] = 0x0041_87c0;
            }
            let _ = &mut r;
        });
    }

    #[test]
    fn ecvt_matches() {
        check(ee::s15::f_00418df8__generated, ecvt, 200, &|c, seed| {
            let mut r = Rng(0x8df8 ^ seed as u64);
            heap_init(c, &mut r);
            c.g[A0][0] = dbl(seed, &mut r);
            c.g[A1][0] = [0, 1, 3, 6, 10, 17][seed as usize % 6] as u64;
            c.g[A2][0] = SCR as u64;
            c.g[A3][0] = (SCR + 4) as u64;
        });
    }

    #[test]
    fn ecvtbuf_events() {
        // Long digit string: the digit loops pass the `0x421b18` /
        // `0x421b48` edges with a due VBlank.
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0x1a50 ^ seed as u64);
            heap_init(c, &mut r);
            c.g[A0][0] = 0.3333333333333333f64.to_bits();
            c.g[A1][0] = 17;
            c.g[A2][0] = SCR as u64;
            c.g[A3][0] = (SCR + 4) as u64;
            c.g[T0][0] = DST as u64;
            c.bus.write32(SCR, 0);
            c.bus.write32(SCR + 4, 0);
            for i in 0..0x200 {
                c.bus.write8(DST + i as u32, 0xcc);
            }
            heap_schedule(c, &mut r, seed, 0x0042_1a50)
        };
        check_events(ee::s15::f_00421a50__generated, ecvtbuf, 60, &setup);
    }

    #[test]
    fn ecvtbuf_matches() {
        check(ee::s15::f_00421a50__generated, ecvtbuf, 200, &|c, seed| {
            let mut r = Rng(0x1a50 ^ seed as u64);
            heap_init(c, &mut r);
            c.g[A0][0] = dbl_2(seed, &mut r);
            c.g[A1][0] = [0, 1, 3, 6, 10, 17][seed as usize % 6] as u64;
            c.g[A2][0] = SCR as u64;
            c.g[A3][0] = (SCR + 4) as u64;
            c.g[T0][0] = if seed % 3 == 0 { 0 } else { DST as u64 };
            c.bus.write32(SCR, 0);
            c.bus.write32(SCR + 4, 0);
            for i in 0..0x200 {
                c.bus.write8(DST + i as u32, 0xcc);
            }
        });
    }

    #[test]
    fn ecvtf_matches() {
        check(ee::s15::f_00418e18__generated, ecvtf, 200, &|c, seed| {
            let mut r = Rng(0x8e18 ^ seed as u64);
            heap_init(c, &mut r);
            c.f[12] = flt(seed, &mut r);
            c.g[A0][0] = [0, 1, 3, 6, 10][seed as usize % 5] as u64;
            c.g[A1][0] = SCR as u64;
            c.g[A2][0] = (SCR + 4) as u64;
        });
    }

    #[test]
    fn fcvt_matches() {
        check(ee::s15::f_00418d48__generated, fcvt, 200, &|c, seed| {
            let mut r = Rng(0x8d48 ^ seed as u64);
            heap_init(c, &mut r);
            c.g[A0][0] = dbl(seed, &mut r);
            c.g[A1][0] = [0, 1, 3, 6, 10, 17][seed as usize % 6] as u64;
            c.g[A2][0] = SCR as u64;
            c.g[A3][0] = (SCR + 4) as u64;
        });
    }

    #[test]
    fn fcvtbuf_events() {
        // Long digit string: the digit loops pass the `0x4219d0` /
        // `0x421a00` edges with a due VBlank.
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0x18d8 ^ seed as u64);
            heap_init(c, &mut r);
            c.pc = 0x0042_18d8;
            for i in 0..0x200 {
                c.bus.write8(DST + i as u32, 0xcc);
            }
            c.g[A0][0] = 0.3333333333333333f64.to_bits();
            c.g[A1][0] = 17;
            c.g[A2][0] = SCR as u64;
            c.g[A3][0] = (SCR + 4) as u64;
            c.g[T0][0] = DST as u64;
            c.bus.write32(SCR, 0);
            c.bus.write32(SCR + 4, 0);
            heap_schedule(c, &mut r, seed, 0x0042_18d8)
        };
        check_events(ee::s15::f_004218d8__generated, fcvtbuf, 60, &setup);
    }

    #[test]
    fn fcvtbuf_matches() {
        check(ee::s15::f_004218d8__generated, fcvtbuf, 200, &|c, seed| {
            let mut r = Rng(0x18d8 ^ seed as u64);
            heap_init(c, &mut r);
            c.pc = 0x0042_18d8;
            c.g[A0][0] = dbl_2(seed, &mut r);
            c.g[A1][0] = [0, 1, 3, 6, 10, 17][seed as usize % 6] as u64;
            c.g[A2][0] = SCR as u64;
            c.g[A3][0] = (SCR + 4) as u64;
            c.g[T0][0] = if seed % 3 == 0 { 0 } else { DST as u64 };
            c.bus.write32(SCR, 0);
            c.bus.write32(SCR + 4, 0);
            for i in 0..0x200 {
                c.bus.write8(DST + i as u32, 0xcc);
            }
        });
    }

    #[test]
    fn fcvtf_matches() {
        check(ee::s15::f_00418d68__generated, fcvtf, 200, &|c, seed| {
            let mut r = Rng(0x8d68 ^ seed as u64);
            heap_init(c, &mut r);
            c.f[12] = flt(seed, &mut r);
            c.g[A0][0] = [0, 1, 3, 6, 10][seed as usize % 5] as u64;
            c.g[A1][0] = SCR as u64;
            c.g[A2][0] = (SCR + 4) as u64;
        });
    }

    #[test]
    fn fflush_matches() {
        use crate::difftest::fakes::{
            fake_call, fake_jump_out, fake_syscall, FAKE, LOG_RESULTS, LOG_UNWIND_AT,
        };
        check(ee::s15::f_0041fe30__generated, fflush, 200, &|c, seed| {
            let mut r = file_fixture_2(c, seed, 0xfe30, 3);
            c.pc = 0x0041_fe30;
            match seed % 7 {
                0 => {
                    // Flush every file.
                    c.g[A0][0] = 0;
                }
                1 => {
                    // Reading: nothing buffered for writing.
                    c.bus.write16(FILE_4 + 0xc, 0x10);
                    c.g[A0][0] = FILE_4 as u64;
                }
                2 => {
                    // Buffered but `_bf` base zero: short epilogue.
                    c.bus.write16(FILE_4 + 0xc, 0x208);
                    c.bus.write32(FILE_4 + 0x10, 0);
                    c.g[A0][0] = FILE_4 as u64;
                }
                3 => {
                    // Nothing pending (`_p == base`): no write call.
                    c.bus.write16(FILE_4 + 0xc, 0x208);
                    c.g[A0][0] = FILE_4 as u64;
                }
                4 => {
                    // Eight bytes pending: one write call.
                    c.bus.write16(FILE_4 + 0xc, 0x208);
                    c.bus.write32(FILE_4, OUT_3 + 8);
                    for i in 0..8 {
                        c.bus.write8(OUT_3 + i as u32, b'f' + i as u8);
                    }
                    c.g[A0][0] = FILE_4 as u64;
                }
                5 => {
                    // FAKE write returning a partial count: the loop
                    // writes again (`S0 > 0` after the first write).
                    c.hooks.miss = fake_call;
                    c.hooks.jump_out = fake_jump_out;
                    c.hooks.syscall = fake_syscall;
                    c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                    c.bus.write16(FILE_4 + 0xc, 0x208);
                    c.bus.write32(FILE_4, OUT_3 + 8);
                    for i in 0..8 {
                        c.bus.write8(OUT_3 + i as u32, b'f' + i as u8);
                    }
                    c.bus.write32(FILE_4 + 0x1c, 3);
                    c.bus.write32(FILE_4 + 0x24, FAKE);
                    c.bus.write32(LOG_RESULTS, 3);
                    c.g[A0][0] = FILE_4 as u64;
                }
                _ => {
                    // FAKE write returning the full count.
                    c.hooks.miss = fake_call;
                    c.hooks.jump_out = fake_jump_out;
                    c.hooks.syscall = fake_syscall;
                    c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                    c.bus.write16(FILE_4 + 0xc, 0x208);
                    c.bus.write32(FILE_4, OUT_3 + 8);
                    for i in 0..8 {
                        c.bus.write8(OUT_3 + i as u32, b'f' + i as u8);
                    }
                    c.bus.write32(FILE_4 + 0x1c, 3);
                    c.bus.write32(FILE_4 + 0x24, FAKE);
                    c.bus.write32(LOG_RESULTS, 8);
                    c.g[A0][0] = FILE_4 as u64;
                }
            }
            let _ = &mut r;
        });
    }

    #[test]
    fn fflush_events() {
        use crate::difftest::fakes::{
            fake_call, fake_jump_out, fake_syscall, FAKE, LOG_RESULTS, LOG_UNWIND_AT,
        };
        // A FAKE cookie returning a partial count keeps `S0 > 0`, so the
        // partial-write back edge (`0x41fef8`) sees a due VBlank.
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = file_fixture_2(c, seed, 0xfe30, 3);
            c.pc = 0x0041_fe30;
            c.hooks.miss = fake_call;
            c.hooks.jump_out = fake_jump_out;
            c.hooks.syscall = fake_syscall;
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            c.bus.write16(FILE_4 + 0xc, 0x208);
            c.bus.write32(FILE_4, OUT_3 + 8);
            for i in 0..8 {
                c.bus.write8(OUT_3 + i as u32, b'f' + i as u8);
            }
            c.bus.write32(FILE_4 + 0x1c, 3);
            c.bus.write32(FILE_4 + 0x24, FAKE);
            c.bus.write32(LOG_RESULTS, 3);
            c.g[A0][0] = FILE_4 as u64;
            let _ = &mut r;
            schedule(c, &mut r, seed, 0x0041_fe30)
        };
        check_events(ee::s15::f_0041fe30__generated, fflush, 60, &setup);
    }

    #[test]
    fn fread_bcopy_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0x0228 ^ seed as u64);
            scramble(c, &mut r);
            for i in 0..0x200 {
                c.bus.write8(SRC + i as u32, b'c');
                c.bus.write8(DST + i as u32, 0xcc);
            }
            c.g[A0][0] = DST as u64;
            c.g[A1][0] = SRC as u64;
            c.g[A2][0] = 200;
            schedule(c, &mut r, seed, 0x0042_0228)
        };
        check_events(ee::s15::f_00420228__generated, fread_bcopy, 60, &setup);
    }

    #[test]
    fn fread_bcopy_matches() {
        check(
            ee::s15::f_00420228__generated,
            fread_bcopy,
            200,
            &|c, seed| {
                let mut r = Rng(0x0228 ^ seed as u64);
                scramble(c, &mut r);
                for i in 0..0x100 {
                    c.bus.write8(SRC + i as u32, b'b' + (i % 11) as u8);
                    c.bus.write8(DST + i as u32, 0xcc);
                }
                c.g[A0][0] = DST as u64;
                c.g[A1][0] = (SRC + seed as u32 % 3) as u64;
                c.g[A2][0] = [0u64, 1, 7, 16, 33, 100][seed as usize % 6];
            },
        );
    }

    #[test]
    fn fread_events() {
        use crate::difftest::fakes::{
            fake_call, fake_jump_out, fake_syscall, FAKE, LOG_RESULTS, LOG_UNWIND_AT,
        };
        // Buffered partial with a FAKE cookie refilling the remainder:
        // the refill loop passes the `0x4202e0` edge with a due VBlank.
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = file_fixture_2(c, seed, 0x0260, 3);
            c.pc = 0x0042_0260;
            for i in 0..0x100 {
                c.bus.write8(SRC + i as u32, b'r' + (i % 7) as u8);
            }
            c.hooks.miss = fake_call;
            c.hooks.jump_out = fake_jump_out;
            c.hooks.syscall = fake_syscall;
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            c.bus.write32(FILE_4, SRC);
            c.bus.write32(FILE_4 + 0x4, 8);
            c.bus.write16(FILE_4 + 0xc, 0x10);
            c.bus.write32(FILE_4 + 0x1c, 3);
            c.bus.write32(FILE_4 + 0x20, FAKE);
            c.bus.write32(FILE_4 + 0x28, 0x0014_06c0);
            c.bus.write32(FILE_4 + 0x10, OUT_3);
            c.bus.write32(FILE_4 + 0x14, 0x100);
            c.bus.write32(LOG_RESULTS, 24);
            c.bus.write32(LOG_RESULTS + 4, 24);
            c.g[A0][0] = DST as u64;
            c.g[A1][0] = 4;
            c.g[A2][0] = 8;
            c.g[A3][0] = FILE_4 as u64;
            let _ = &mut r;
            schedule(c, &mut r, seed, 0x0042_0260)
        };
        check_events(ee::s15::f_00420260__generated, fread, 60, &setup);
    }

    #[test]
    fn fread_matches() {
        use crate::difftest::fakes::{
            fake_call, fake_jump_out, fake_syscall, FAKE, LOG_RESULTS, LOG_UNWIND_AT,
        };
        check(ee::s15::f_00420260__generated, fread, 200, &|c, seed| {
            let mut r = file_fixture_2(c, seed, 0x0260, 3);
            c.pc = 0x0042_0260;
            for i in 0..0x100 {
                c.bus.write8(SRC + i as u32, b'r' + (i % 7) as u8);
            }
            match seed % 8 {
                0 => {
                    // Empty read: zero items.
                    c.g[A0][0] = DST as u64;
                    c.g[A1][0] = 4;
                    c.g[A2][0] = 0;
                    c.g[A3][0] = FILE_4 as u64;
                }
                7 => {
                    // Buffered partial with a FAKE cookie refilling
                    // exactly the remainder: the inner check sees
                    // `remaining == wanted`.
                    c.hooks.miss = fake_call;
                    c.hooks.jump_out = fake_jump_out;
                    c.hooks.syscall = fake_syscall;
                    c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                    c.bus.write32(FILE_4, SRC);
                    c.bus.write32(FILE_4 + 0x4, 8);
                    c.bus.write16(FILE_4 + 0xc, 0x10);
                    c.bus.write32(FILE_4 + 0x1c, 3);
                    c.bus.write32(FILE_4 + 0x20, FAKE);
                    c.bus.write32(FILE_4 + 0x28, 0x0014_06c0);
                    c.bus.write32(FILE_4 + 0x10, OUT_3);
                    c.bus.write32(FILE_4 + 0x14, 0x100);
                    c.bus.write32(LOG_RESULTS, 24);
                    c.g[A0][0] = DST as u64;
                    c.g[A1][0] = 4;
                    c.g[A2][0] = 8;
                    c.g[A3][0] = FILE_4 as u64;
                }
                6 => {
                    // Exactly buffered: remaining == wanted, no refill.
                    c.bus.write32(FILE_4, SRC);
                    c.bus.write32(FILE_4 + 0x4, 32);
                    c.bus.write16(FILE_4 + 0xc, 0x10);
                    c.g[A0][0] = DST as u64;
                    c.g[A1][0] = 4;
                    c.g[A2][0] = 8;
                    c.g[A3][0] = FILE_4 as u64;
                }
                1 => {
                    // Fully buffered: no refill.
                    c.bus.write32(FILE_4, SRC);
                    c.bus.write32(FILE_4 + 0x4, 64);
                    c.bus.write16(FILE_4 + 0xc, 0x10);
                    c.g[A0][0] = DST as u64;
                    c.g[A1][0] = 4;
                    c.g[A2][0] = 8;
                    c.g[A3][0] = FILE_4 as u64;
                }
                2 => {
                    // Partially buffered, refill hits EOF.
                    c.bus.write32(FILE_4, SRC);
                    c.bus.write32(FILE_4 + 0x4, 8);
                    c.bus.write16(FILE_4 + 0xc, 0x10);
                    c.bus.write32(FILE_4 + 0x10, OUT_3);
                    c.bus.write32(FILE_4 + 0x14, 0x100);
                    c.g[A0][0] = DST as u64;
                    c.g[A1][0] = 4;
                    c.g[A2][0] = 8;
                    c.g[A3][0] = FILE_4 as u64;
                }
                3 => {
                    // Unbuffered read with an EOF cookie; on odd rounds
                    // a negative `_r` (left by a `getc` that went to
                    // `__srget`), which `fread` clears first.
                    c.bus.write32(FILE_4, SRC);
                    let left = if seed / 8 % 2 == 1 { 0xffff_fffe } else { 0 };
                    c.bus.write32(FILE_4 + 0x4, left);
                    c.bus.write16(FILE_4 + 0xc, 0x10);
                    c.bus.write32(FILE_4 + 0x28, 0x0014_06c0);
                    c.bus.write32(FILE_4 + 0x10, OUT_3);
                    c.g[A0][0] = DST as u64;
                    c.g[A1][0] = 1;
                    c.g[A2][0] = 16;
                    c.g[A3][0] = FILE_4 as u64;
                }
                4 => {
                    // Unbuffered read with a FAKE cookie reporting a full
                    // transfer: the success path.
                    c.hooks.miss = fake_call;
                    c.hooks.jump_out = fake_jump_out;
                    c.hooks.syscall = fake_syscall;
                    c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                    c.bus.write32(FILE_4, SRC);
                    c.bus.write32(FILE_4 + 0x4, 0);
                    c.bus.write16(FILE_4 + 0xc, 0x10);
                    c.bus.write32(FILE_4 + 0x1c, 3);
                    c.bus.write32(FILE_4 + 0x20, FAKE);
                    c.bus.write32(FILE_4 + 0x28, 0x0014_06c0);
                    c.bus.write32(FILE_4 + 0x10, OUT_3);
                    c.bus.write32(LOG_RESULTS, 0x40);
                    c.g[A0][0] = DST as u64;
                    c.g[A1][0] = 4;
                    c.g[A2][0] = 8;
                    c.g[A3][0] = FILE_4 as u64;
                }
                _ => {
                    // Unbuffered read with a FAKE cookie reporting a short
                    // transfer, then EOF.
                    c.hooks.miss = fake_call;
                    c.hooks.jump_out = fake_jump_out;
                    c.hooks.syscall = fake_syscall;
                    c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                    c.bus.write32(FILE_4, SRC);
                    c.bus.write32(FILE_4 + 0x4, 0);
                    c.bus.write16(FILE_4 + 0xc, 0x10);
                    c.bus.write32(FILE_4 + 0x1c, 3);
                    c.bus.write32(FILE_4 + 0x20, FAKE);
                    c.bus.write32(FILE_4 + 0x28, 0x0014_06c0);
                    c.bus.write32(FILE_4 + 0x10, OUT_3);
                    c.bus.write32(LOG_RESULTS, 2);
                    c.bus.write32(LOG_RESULTS + 4, 0);
                    c.g[A0][0] = DST as u64;
                    c.g[A1][0] = 4;
                    c.g[A2][0] = 8;
                    c.g[A3][0] = FILE_4 as u64;
                }
            }
            let _ = &mut r;
        });
    }

    #[test]
    fn free_matches() {
        check(ee::s15::f_00418910__generated, free, 120, &|c, seed| {
            let mut r = Rng(0x8910 ^ seed as u64);
            heap_init(c, &mut r);
            if seed % 4 == 3 {
                c.g[A0][0] = 0; // `free(NULL)` is a no-op.
            } else {
                let size = [8, 16, 64, 256, 1024][seed as usize % 5] + (r.next_u32() % 32);
                c.g[A0][0] = size as u64;
                ee::s15::f_004188c0__generated(c);
                assert_ne!(c.g[V0][0], 0, "seed {seed}: malloc failed");
                c.g[A0][0] = c.g[V0][0];
            }
        });
    }

    // NOTE (checklist 9): no `check_events` for `fseek` — its single
    // 0x417100 check uses `back_edge_unwinds` like the generated body, but
    // the check point never coincides with a deliverable event in this
    // harness (0 of 80 seeds reschedule), so the test would assert nothing
    // beyond `fseek_matches`. The unwind-early-return shape matches the
    // generated `if c.event() { return; }` by construction.
    #[test]
    fn fseek_events() {
        // `SEEK_SET` with the real (failing) cookie: the positioned tail
        // runs the failing cookie seek, whose retry-fail edge (`0x417100`)
        // sees a due VBlank.
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = file_fixture(c, seed, 0x7008, 3);
            c.pc = 0x0041_7008;
            c.bus.write32(FILE_2 + 0x1c, 3);
            c.bus.write32(FILE_2 + 0x28, 0x0041_7a30);
            c.g[A0][0] = FILE_2 as u64;
            c.g[A1][0] = 20;
            c.g[A2][0] = 0;
            let _ = &mut r;
            schedule(c, &mut r, seed, 0x0041_7008)
        };
        check_events(ee::s15::f_00417008__generated, fseek, 60, &setup);
    }
    #[test]
    fn fseek_matches() {
        use crate::difftest::fakes::{
            fake_call, fake_jump_out, fake_syscall, FAKE, LOG_RESULTS, LOG_UNWIND_AT,
        };
        check(ee::s15::f_00417008__generated, fseek, 200, &|c, seed| {
            let mut r = file_fixture(c, seed, 0x7008, 3);
            c.pc = 0x0041_7008;
            // Seek cookie: the real `__sseek` except the null-cookie fail
            // case. The invalid fd fails every real seek deterministically,
            // and the scratch cookie word is pinned (never a valid host fd).
            c.bus.write32(FILE_2 + 0x1c, 3);
            c.bus.write32(FILE_2 + 0x28, 0x0041_7a30);
            match seed % 14 {
                0 => {
                    // No seek cookie: `EINVAL`.
                    c.bus.write32(FILE_2 + 0x28, 0);
                    c.g[A0][0] = FILE_2 as u64;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                }
                1 => {
                    // `whence` above `SEEK_END`: `EINVAL`.
                    c.g[A0][0] = FILE_2 as u64;
                    c.g[A1][0] = 10;
                    c.g[A2][0] = 3;
                }
                2 => {
                    // Negative `whence`: `EINVAL`.
                    c.g[A0][0] = FILE_2 as u64;
                    c.g[A1][0] = 10;
                    c.g[A2][0] = 0xffff_ffff;
                }
                3 => {
                    // `SEEK_SET` with buffered data: entry flush is skipped
                    // (write-only), the positioned tail runs the failing
                    // cookie seek.
                    c.g[A0][0] = FILE_2 as u64;
                    c.g[A1][0] = 20;
                    c.g[A2][0] = 0;
                }
                4 => {
                    // `SEEK_END`: the positioned tail from the other entry.
                    c.g[A0][0] = FILE_2 as u64;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 2;
                }
                5 => {
                    // `SEEK_CUR`: flush, then the failing cookie seek.
                    c.g[A0][0] = FILE_2 as u64;
                    c.g[A1][0] = 5;
                    c.g[A2][0] = 1;
                }
                6 => {
                    // `SEEK_SET` with no buffer base: the positioned tail
                    // runs `__smakebuf` first.
                    c.bus.write32(FILE_2 + 0x10, 0);
                    c.g[A0][0] = FILE_2 as u64;
                    c.g[A1][0] = 8;
                    c.g[A2][0] = 0;
                }
                7 => {
                    // Entry flush: flags carry the flush bits.
                    c.bus.write16(FILE_2 + 0xc, 0x108);
                    c.g[A0][0] = FILE_2 as u64;
                    c.g[A1][0] = 20;
                    c.g[A2][0] = 0;
                }
                8 => {
                    // `SEEK_SET` with a FAKE cookie reporting success: the
                    // positioned tail completes.
                    c.hooks.miss = fake_call;
                    c.hooks.jump_out = fake_jump_out;
                    c.hooks.syscall = fake_syscall;
                    c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                    c.bus.write32(FILE_2 + 0x28, FAKE);
                    c.bus.write32(LOG_RESULTS, 100);
                    c.g[A0][0] = FILE_2 as u64;
                    c.g[A1][0] = 50;
                    c.g[A2][0] = 0;
                }
                9 => {
                    // `SEEK_CUR` with a FAKE cookie reporting success.
                    c.hooks.miss = fake_call;
                    c.hooks.jump_out = fake_jump_out;
                    c.hooks.syscall = fake_syscall;
                    c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                    c.bus.write32(FILE_2 + 0x28, FAKE);
                    c.bus.write32(LOG_RESULTS, 60);
                    c.bus.write16(
                        FILE_2 + 0xc,
                        [0x200, 0x204, 0x208, 0x210][(seed / 10) as usize % 4],
                    );
                    c.g[A0][0] = FILE_2 as u64;
                    c.g[A1][0] = 5;
                    c.g[A2][0] = 1;
                }
                10 => {
                    // `SEEK_CUR` with `__SOFF` set: the short-seek arm
                    // reloads the full flags.
                    c.bus.write16(FILE_2 + 0xc, 0x1004);
                    c.g[A0][0] = FILE_2 as u64;
                    c.g[A1][0] = 5;
                    c.g[A2][0] = 1;
                }
                11 => {
                    // `SEEK_SET` with `0x400` flags and a FAKE cookie:
                    // straight to the positioned join, no unget word.
                    c.hooks.miss = fake_call;
                    c.hooks.jump_out = fake_jump_out;
                    c.hooks.syscall = fake_syscall;
                    c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                    c.bus.write32(FILE_2 + 0x28, FAKE);
                    c.bus.write32(LOG_RESULTS, 100);
                    c.bus.write32(LOG_RESULTS + 4, 100);
                    c.bus.write16(FILE_2 + 0xc, 0x400);
                    c.g[A0][0] = FILE_2 as u64;
                    c.g[A1][0] = 20;
                    c.g[A2][0] = 0;
                }
                12 => {
                    // `SEEK_CUR` with `0x400` flags and a FAKE cookie: the
                    // positioned join with a known target (`S7 != 0`).
                    c.hooks.miss = fake_call;
                    c.hooks.jump_out = fake_jump_out;
                    c.hooks.syscall = fake_syscall;
                    c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                    c.bus.write32(FILE_2 + 0x28, FAKE);
                    c.bus.write32(LOG_RESULTS, 60);
                    c.bus.write32(LOG_RESULTS + 4, 100);
                    c.bus.write16(FILE_2 + 0xc, 0x400);
                    c.bus.write32(FILE_2 + 0x30, 0);
                    c.g[A0][0] = FILE_2 as u64;
                    c.g[A1][0] = 5;
                    c.g[A2][0] = 1;
                }
                _ => {
                    // `SEEK_END` with `0x400` flags: the positioned join
                    // takes the `_fstat_r` branch, where the real
                    // `_fstat_r` reports a character device, so the
                    // size comes from the stack slot.
                    c.bus.write16(FILE_2 + 0xc, 0x400);
                    c.g[A0][0] = FILE_2 as u64;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 2;
                }
            }
            let _ = &mut r;
        });
    }

    #[test]
    fn fseek_regular_file_matches() {
        // `SEEK_END` on a stream whose cookie is the real `__sseek` and
        // whose flags lack `0x400`: the gated `_fstat_r` reports a
        // regular file, so the positioned tail takes the regular-file
        // arm (sets `0x400`) and the second `_fstat_r` adds the file
        // size to the target.
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = file_fixture(c, seed, 0x7008, 3);
            c.pc = 0x0041_7008;
            libc_arm_fakes(c, 3, 0x8000, 0x1000 + (seed % 2) as u64 * 0x100);
            c.bus.write32(FILE_2 + 0x1c, 3);
            c.bus.write32(FILE_2 + 0x28, 0x0041_7a30);
            c.bus.write16(FILE_2 + 0xc, 0);
            c.g[A0][0] = FILE_2 as u64;
            c.g[A1][0] = 0;
            c.g[A2][0] = 2;
            let _ = &mut r;
        };
        check_paths(ee::s15::f_00417008__generated, fseek, 60, &[230], &setup);
    }

    #[test]
    fn gcvt_matches() {
        check(ee::s15::f_00418e68__generated, gcvt, 200, &|c, seed| {
            let mut r = Rng(0x8e68 ^ seed as u64);
            heap_init(c, &mut r);
            c.g[A0][0] = dbl(seed, &mut r);
            // Mod 7 (vs the mod-10 fixed table) so exact digit-count
            // combos occur: `S1 == 0` at the loop guards.
            c.g[A1][0] = [1, 2, 3, 6, 10, 15, 17][seed as usize % 7] as u64;
            c.g[A2][0] = DST as u64;
            for i in 0..0x40 {
                c.bus.write8(DST + i as u32, 0xcc);
            }
        });
    }

    #[test]
    fn gcvtf_matches() {
        check(ee::s15::f_00418db8__generated, gcvtf, 200, &|c, seed| {
            let mut r = Rng(0x8db8 ^ seed as u64);
            heap_init(c, &mut r);
            c.f[12] = flt(seed, &mut r);
            c.g[A0][0] = [1, 3, 6, 10, 15][seed as usize % 5] as u64;
            c.g[A1][0] = DST as u64;
            for i in 0..0x40 {
                c.bus.write8(DST + i as u32, 0xcc);
            }
        });
    }

    #[test]
    fn lflush_matches() {
        check(ee::s15::f_00417628__generated, lflush, 120, &|c, seed| {
            let mut r = file_fixture(c, seed, 0x7628, 3);
            c.g[A0][0] = FILE_2 as u64;
            let _ = &mut r;
        });
    }

    #[test]
    fn localeconv_matches() {
        check(
            ee::s15::f_0041ec48__generated,
            localeconv,
            60,
            &|c, seed| {
                let mut r = Rng(0xec48 ^ seed as u64);
                wire_reent(c, &mut r);
                let _ = seed;
            },
        );
    }

    #[test]
    fn malloc_extend_top_matches() {
        check(
            ee::s15::f_0041d990__generated,
            malloc_extend_top,
            120,
            &|c, seed| {
                // A lived-in heap so the top chunk sits at varied
                // addresses/sizes; fresh-heap-only fixtures never reached
                // the bin/top-split tails. Fresh-heap large allocs hit
                // the stitch guards (fragmented histories do not).
                match seed % 8 {
                    6 | 7 => {
                        let mut r = Rng(0xd990 ^ seed as u64);
                        heap_init(c, &mut r);
                    }
                    _ => {
                        let _ = heap_history(c, seed, 0xd990);
                    }
                }
                c.g[A0][0] = REENT_3 as u64;
                c.g[A1][0] = [0, 16, 0x100, 0x1000, 0x10000, 0x100000, 0x10000, 0x100000]
                    [seed as usize % 8] as u64;
            },
        );
    }

    #[test]
    fn malloc_extend_top_pad_fail_matches() {
        // A fresh heap whose ceiling admits the first `_sbrk_r` (the
        // break moves `0x900000` -> `0x900110` for the `0x100` request)
        // but not the alignment-pad call (`0x901000`): the body takes
        // the pad-failure arm and returns -1 off the shared tail.
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = Rng(0xd990 ^ seed as u64);
            heap_init(c, &mut r);
            c.bus.write32(0x0045_516c, 0x0090_0000);
            ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                .kernel
                .threads
                .get_mut(&1)
                .expect("main thread")
                .heap = 0x0090_0800;
            c.g[A0][0] = REENT_3 as u64;
            c.g[A1][0] = 0x100;
            let _ = seed;
        };
        check_paths(
            ee::s15::f_0041d990__generated,
            malloc_extend_top,
            20,
            &[208],
            &setup,
        );
    }

    #[test]
    fn malloc_extend_top_small_top_matches() {
        // A lived-in heap with the break raised past the old top end
        // (`seed % 5`): 0 forges the top size to `0x8` (the small-top
        // arm: sets the bit, returns, keeps the shared `+1`); 1 keeps the
        // real size (the stitch frees the old top); 2 forges `0x21` (the
        // stitch leaves exactly `0x10`); 3 forges `0x11` (exactly `0x10`
        // at the small-top test). 4 drops the break to the top itself, so
        // `sbrk` returns below the old end and the body gives up.
        let setup = |c: &mut Cpu, seed: u32| {
            let _ = heap_history(c, seed, 0xd990);
            let top = c.bus.read32(0x0045_4cf0);
            let class = seed % 5;
            if let Some(&size) = [0x8u32, 0, 0x21, 0x11].get(class as usize) {
                if size != 0 {
                    c.bus.write32(top.wrapping_add(4), size);
                }
            }
            let brk = c.bus.read32(0x0045_516c);
            let brk = if class == 4 {
                top
            } else {
                brk.wrapping_add(0x2000)
            };
            c.bus.write32(0x0045_516c, brk);
            c.g[A0][0] = REENT_3 as u64;
            c.g[A1][0] = 0x100;
        };
        check_paths(
            ee::s15::f_0041d990__generated,
            malloc_extend_top,
            80,
            &[223],
            &setup,
        );
    }

    #[test]
    fn malloc_extend_top_in_place_matches() {
        // Break pinned to the top end, no top pad: `sbrk` returns the old
        // end and the top grows in place. `seed % 3` 0/1: `sbrk_base`
        // forged unset, so the odd request is not page-rounded and the new
        // size is odd. 2: a 0xff0 request rounds to one page and
        // `max_total_mem` is forged to the resulting `sbrked_mem`.
        check(
            ee::s15::f_0041d990__generated,
            malloc_extend_top,
            30,
            &|c, seed| {
                let _ = heap_history(c, seed, 0xd990);
                let top = c.bus.read32(0x0045_4cf0);
                let size = c.bus.read32(top.wrapping_add(4)) & !3;
                c.bus.write32(0x0045_516c, top.wrapping_add(size));
                c.bus.write32(0x0045_50f8, 0);
                c.g[A0][0] = REENT_3 as u64;
                if seed % 3 == 2 {
                    c.bus.write32(0x0045_5100, 0x0090_0000);
                    let mem = c.bus.read32(0x0045_5118).wrapping_add(0x1000);
                    c.bus.write64(0x0045_5110, mem as i32 as i64 as u64);
                    c.g[A1][0] = 0xff0;
                } else {
                    c.bus.write32(0x0045_5100, u32::MAX);
                    c.g[A1][0] = [0x101, 0x1235][seed as usize % 3] as u64;
                }
            },
        );
    }

    #[test]
    fn malloc_extend_top_pad_odd_matches() {
        // Fresh heap; the alignment-pad `sbrk` (fake call 1) reports an
        // odd break. The real `sbrk` keeps the new top size even, so only
        // a forged result reaches an odd size in the new top's head.
        check(
            ee::s15::f_0041d990__generated,
            malloc_extend_top,
            16,
            &|c, seed| {
                let mut r = Rng(0xd990 ^ seed as u64);
                heap_init(c, &mut r);
                libc_arm_sbrk(c, &[0, 0x0090_1001 + 2 * (seed % 4)]);
                c.g[A0][0] = REENT_3 as u64;
                c.g[A1][0] = [0x100, 0x1000][seed as usize / 4 % 2] as u64;
            },
        );
    }

    #[test]
    fn malloc_matches() {
        check(ee::s15::f_004188c0__generated, malloc, 120, &|c, seed| {
            let mut r = Rng(0x88c0 ^ seed as u64);
            heap_init(c, &mut r);
            c.g[A0][0] = ([0, 1, 8, 16, 64, 256, 1024, 65536][seed as usize % 8]
                + (r.next_u32() % 16)) as u64;
        });
    }

    #[test]
    fn malloc_trim_matches() {
        check(
            ee::s15::f_0041e638__generated,
            malloc_trim,
            120,
            &|c, seed| {
                // Varied heap with free neighbors and top slack to trim;
                // direct `_malloc_r` always fails in tests, so history
                // goes through the wrappers.
                let _ = heap_history(c, seed, 0xe638);
                c.g[A0][0] = REENT_3 as u64;
                c.g[A1][0] = [0, 0x100, 0x1000, 0x10000][seed as usize % 4] as u64;
            },
        );
    }

    #[test]
    fn malloc_trim_sbrk_fail_matches() {
        // A forged 0x10000-byte top and a scripted `_sbrk_r`: call 0
        // reports the top's end (the trim goes ahead), call 1 fails the
        // shrink, call 2 re-reads the break at `top + slack`. The real
        // `sbrk` never fails a shrink. Slack (`seed % 4`): 8 (under 16),
        // 16 (the `S1 < 16` edge), 0x21 (odd), 0x2000.
        check(
            ee::s15::f_0041e638__generated,
            malloc_trim,
            32,
            &|c, seed| {
                const TOP: u32 = 0x0090_0100;
                let mut r = Rng(0xe638 ^ seed as u64);
                heap_init(c, &mut r);
                c.bus.write32(0x0045_4cf0, TOP);
                c.bus.write32(TOP + 4, 0x0001_0001);
                let slack = [8u32, 16, 0x21, 0x2000][seed as usize % 4];
                libc_arm_sbrk(c, &[TOP + 0x1_0000, u32::MAX, TOP + slack]);
                c.g[A0][0] = REENT_3 as u64;
                c.g[A1][0] = [0, 0x100][seed as usize / 4 % 2] as u64;
            },
        );
    }

    #[test]
    fn memchr_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0xec70 ^ seed as u64);
            scramble(c, &mut r);
            for i in 0..256 {
                c.bus.write8(SRC + i as u32, b'm');
            }
            c.g[A0][0] = SRC as u64;
            c.g[A1][0] = b'z' as u64; // absent: scans everything.
            c.g[A2][0] = 256;
            schedule(c, &mut r, seed, 0x0041_ec70)
        };
        check_events(ee::s15::f_0041ec70__generated, memchr, 60, &setup);
    }

    #[test]
    fn memchr_matches() {
        check(ee::s15::f_0041ec70__generated, memchr, 200, &|c, seed| {
            let mut r = Rng(0xec70 ^ seed as u64);
            scramble(c, &mut r);
            let (len, off) = match seed % 6 {
                0 => (0usize, 0usize),
                1 => (1, 1), // unaligned start
                2 => (15, 0),
                3 => (16, 0), // word loop, one chunk
                4 => (40, 3), // unaligned + multi-chunk
                _ => ((r.next_u32() % 48) as usize, (r.next_u32() % 4) as usize),
            };
            for i in 0..0x100 {
                c.bus.write8(SRC + i as u32, b'x');
            }
            // A late hit for some seeds.
            if seed % 3 == 0 && len > 2 {
                c.bus.write8(SRC + off as u32 + len as u32 - 2, b'q');
            }
            // Multi-match: every chunk hits, so the running mask and the
            // chunk mask share bits (`|` vs `^` diverges).
            if seed % 6 == 5 {
                for i in 0..0x100 {
                    c.bus.write8(SRC + i as u32, b'q');
                }
                c.g[A1][0] = b'q' as u64;
            }
            c.g[A0][0] = (SRC + off as u32) as u64;
            c.g[A1][0] = [0, b'q' as u64, 0x7a, 0x1ff][seed as usize % 4];
            c.g[A2][0] = len as u64;
        });
    }

    #[test]
    fn memmove_events() {
        // Length 144 leaves 16 bytes after the 16-byte loop, so the
        // 8-byte loop runs twice and its `0x4161bc` edge sees a due
        // event; the 16-byte loop's own edge fires as before.
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let (mut r, _, _) = str_pair(c, seed, 0x610c);
            c.g[A0][0] = DST as u64;
            c.g[A1][0] = SRC as u64;
            c.g[A2][0] = 144;
            schedule(c, &mut r, seed, 0x0041_610c)
        };
        check_events(ee::s15::f_0041610c__generated, memmove, 60, &setup);
    }

    #[test]
    fn memmove_matches() {
        check(ee::s15::f_0041610c__generated, memmove, 300, &|c, seed| {
            let (_, _, _) = str_pair(c, seed, 0x610c);
            let (d, s, n) = match seed % 9 {
                0 => (DST, SRC, 0),
                1 => (DST + 8, DST, 16),    // dst inside src, backwards
                2 => (DST, DST + 8, 16),    // src inside dst, forwards
                3 => (DST, SRC, 64),        // long, word loops
                4 => (DST + 1, SRC + 3, 7), // unaligned
                5 => (DST, DST, 24),        // same address
                6 => (DST + 16, DST, 16),   // dst starts where src ends
                7 => (DST, SRC + 1, 32 + (seed / 9 % 3) as u32 * 8),
                _ => (DST, SRC, 40 + (seed / 9 % 3) as u32 * 8),
            };
            c.g[A0][0] = d as u64;
            c.g[A1][0] = s as u64;
            c.g[A2][0] = n as u64;
        });
    }

    #[test]
    fn print_e_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0x16e8 ^ seed as u64);
            heap_init(c, &mut r);
            for i in 0..0x300 {
                c.bus.write8(DST + i as u32, 0xcc);
            }
            c.g[A0][0] = REENT_3 as u64;
            c.g[A1][0] = DST as u64;
            c.g[A2][0] = 0.3333333333333333f64.to_bits();
            c.g[A3][0] = 30;
            c.g[T0][0] = 1;
            c.g[T1][0] = 0;
            c.g[T2][0] = 2;
            heap_schedule(c, &mut r, seed, 0x0042_16e8)
        };
        check_events(ee::s15::f_004216e8__generated, print_e, 60, &setup);
    }

    #[test]
    fn print_e_matches() {
        check(ee::s15::f_004216e8__generated, print_e, 120, &|c, seed| {
            let mut r = Rng(0x16e8 ^ seed as u64);
            heap_init(c, &mut r);
            for i in 0..0x200 {
                c.bus.write8(DST + i as u32, 0xcc);
            }
            c.g[A0][0] = REENT_3 as u64;
            c.g[A1][0] = DST as u64;
            c.g[A2][0] = dbl(seed, &mut r);
            c.g[A3][0] = [0, 1, 2, 3, 6, 10, 17, 30][seed as usize % 8] as u64;
            if seed % 8 == 6 {
                // Three-digit exponents engage the exponent-digit path.
                c.g[A2][0] = 1e100f64.to_bits();
            } else if seed % 8 == 7 {
                c.g[A2][0] = 1e-100f64.to_bits();
            }
            if seed % 8 == 2 {
                // More digits than precision: the copy loop re-checks
                // with `S1 == 0` and digits remaining.
                c.g[A2][0] = 123.456f64.to_bits();
                c.g[A3][0] = 3;
            }
            c.g[T0][0] = [0, 1][seed as usize % 2] as u64;
            c.g[T1][0] = [0, 1][seed as usize % 2] as u64;
            c.g[T2][0] = [0, 2, 3][seed as usize % 3] as u64;
        });
    }

    #[test]
    fn print_f_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0x1530 ^ seed as u64);
            heap_init(c, &mut r);
            for i in 0..0x300 {
                c.bus.write8(DST + i as u32, 0xcc);
            }
            c.g[A0][0] = REENT_3 as u64;
            c.g[A1][0] = DST as u64;
            c.g[A2][0] = 0.3333333333333333f64.to_bits();
            c.g[A3][0] = 30;
            c.g[T0][0] = 0;
            c.g[T1][0] = 10;
            c.g[T2][0] = 2;
            heap_schedule(c, &mut r, seed, 0x0042_1530)
        };
        check_events(ee::s15::f_00421530__generated, print_f, 60, &setup);
    }

    #[test]
    fn print_f_matches() {
        check(ee::s15::f_00421530__generated, print_f, 120, &|c, seed| {
            let mut r = Rng(0x1530 ^ seed as u64);
            heap_init(c, &mut r);
            for i in 0..0x200 {
                c.bus.write8(DST + i as u32, 0xcc);
            }
            c.g[A0][0] = REENT_3 as u64;
            c.g[A1][0] = DST as u64;
            // `dbl` (not `dbl_2`): big values like 1e10 leave
            // `decpt - int_digits > 0` and engage the zero-fill loop.
            c.g[A2][0] = dbl(seed, &mut r);
            c.g[A3][0] = [0, 1, 2, 3, 6, 10, 17, 30][seed as usize % 8] as u64;
            c.g[T0][0] = 0;
            c.g[T1][0] = [0, 6, 10][seed as usize % 3] as u64;
            c.g[T2][0] = [0, 2, 3][seed as usize % 3] as u64;
        });
    }

    #[test]
    fn printf_matches() {
        check(ee::s15::f_004175c8__generated, printf, 120, &|c, seed| {
            use crate::difftest::elf_image;
            let (s, _) = printf_case(c, seed, 0x75c8);
            // Global reent + its stdout `FILE_2`.
            elf_image(c);
            c.bus.write32(0x0045_4944, REENT_3);
            c.bus.write32(REENT_3 + 0x8, FILE_2);
            c.bus.write32(FILE_2 + 0x54, REENT_3);
            c.g[A0][0] = FMT as u64;
            c.g[A1][0] = s[0] as u64;
            c.g[A2][0] = s[1] as u64;
            c.g[A3][0] = s[2] as u64;
        });
    }

    #[test]
    fn quorem_events() {
        // Denormalized limbs: a zero top limb engages the trim entries,
        // whose checks see a due VBlank.
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0x2560 ^ seed as u64);
            heap_init(c, &mut r);
            c.bus.write32(SRC, 0);
            c.bus.write32(SRC + 4, 0);
            c.bus.write32(SRC + 8, 8);
            c.bus.write32(SRC + 0xc, 0);
            c.bus.write32(SRC + 0x10, 2);
            c.bus.write32(SRC + 0x14, 0);
            c.bus.write32(SRC + 0x18, 0);
            c.bus.write32(DST, 0);
            c.bus.write32(DST + 4, 0);
            c.bus.write32(DST + 8, 8);
            c.bus.write32(DST + 0xc, 0);
            c.bus.write32(DST + 0x10, 2);
            c.bus.write32(DST + 0x14, 1 + (r.next_u32() % 0xffff));
            c.bus.write32(DST + 0x18, 0);
            c.g[A0][0] = DST as u64;
            c.g[A1][0] = SRC as u64;
            heap_schedule(c, &mut r, seed, 0x0042_2560)
        };
        check_events(ee::s15::f_00422560__generated, quorem, 60, &setup);
    }

    #[test]
    fn quorem_matches() {
        check(ee::s15::f_00422560__generated, quorem, 136, &|c, seed| {
            let mut r = Rng(0x2560 ^ seed as u64);
            heap_init(c, &mut r);
            // Divisor `S`: 1-2 limbs.
            let swds = 1 + (seed % 2) as u32;
            c.bus.write32(SRC, 0);
            c.bus.write32(SRC + 4, 0);
            c.bus.write32(SRC + 8, 8);
            c.bus.write32(SRC + 0xc, 0);
            c.bus.write32(SRC + 0x10, swds);
            for i in 0..swds {
                let mut w = r.next_u32() & 0xffff;
                if i == swds - 1 && w == 0 {
                    w = 1; // top limb nonzero.
                }
                c.bus.write32(SRC + 0x14 + i * 4, w);
            }
            // Dividend `b`: at least as wide as `S` (or narrower for the
            // early-0 path every fourth seed).
            let narrow = seed % 4 == 3;
            let bwds = if narrow { 1 } else { swds + (r.next_u32() % 3) };
            c.bus.write32(DST, 0);
            c.bus.write32(DST + 4, 0);
            c.bus.write32(DST + 8, 8);
            c.bus.write32(DST + 0xc, 0);
            c.bus.write32(DST + 0x10, bwds.max(swds));
            for i in 0..8 {
                c.bus.write32(DST + 0x14 + i * 4, r.next_u32() & 0xffff);
            }
            if narrow {
                c.bus.write32(DST + 0x10, 1);
                c.bus.write32(SRC + 0x10, 2);
                c.bus.write32(SRC + 0x14, 5);
                c.bus.write32(SRC + 0x18, 7);
                c.bus.write32(DST + 0x14, 3);
            }
            // Boundary arms: exact-equality hits the `mcmp == 0`
            // correction path; all-zero low limbs drive the trim loop
            // down to exactly `T0 == S1`.
            if seed % 8 == 6 {
                c.bus.write32(DST + 0x10, swds);
                for i in 0..swds {
                    let w = c.bus.read32(SRC + 0x14 + i * 4);
                    c.bus.write32(DST + 0x14 + i * 4, w);
                }
            } else if seed % 8 == 7 {
                // Denormalized limbs (widths unchanged): a zero top
                // limb engages the trim entries; an all-zero divisor
                // walks the entry check down to exactly `T0 == S1`.
                c.bus.write32(SRC + 0x10, 2);
                c.bus.write32(SRC + 0x14, 0);
                c.bus.write32(SRC + 0x18, 0);
                c.bus.write32(DST + 0x10, 2);
                c.bus.write32(DST + 0x18, 0);
            } else if seed % 16 == 13 {
                // Borrow-chain regression arm: S = [FFFF,FFFF,FFFF,5]
                // w4, b = S+1. The estimate is 1, the subtract borrows
                // through to the top limb, and the head trim must store
                // the trimmed width when it stops on nonzero.
                c.bus.write32(SRC + 0x10, 4);
                c.bus.write32(SRC + 0x14, 0xffff);
                c.bus.write32(SRC + 0x18, 0xffff);
                c.bus.write32(SRC + 0x1c, 0xffff);
                c.bus.write32(SRC + 0x20, 5);
                c.bus.write32(DST + 0x10, 4);
                c.bus.write32(DST + 0x14, 0);
                c.bus.write32(DST + 0x18, 0);
                c.bus.write32(DST + 0x1c, 0);
                c.bus.write32(DST + 0x20, 6);
            } else if seed % 16 == 9 {
                // Narrow borrow: S = [FFFF, 5] w2, b = S+1. The head
                // trim stops immediately at the base (`T0 == S1`),
                // taking the trim loop's zero-trip arm.
                c.bus.write32(SRC + 0x10, 2);
                c.bus.write32(SRC + 0x14, 0xffff);
                c.bus.write32(SRC + 0x18, 5);
                c.bus.write32(DST + 0x10, 2);
                c.bus.write32(DST + 0x14, 0);
                c.bus.write32(DST + 0x18, 6);
            }
            // Full-width trim arms (seeds 120..135). R = [r0, r1 or 0, 0..]
            // leaves k-1 or k-2 zero top limbs, so the trim loops re-enter
            // and stop on the base or on r1. Head (j even): S = [FFFFFFFF;
            // k-1] ++ [5], b = S + R: estimate 1, subtract leaves R. Tail
            // (j odd): S limbs with high halves set (nonzero add-back T1),
            // b = S + R without a top carry: estimate 0, mcmp > 0, the
            // add-back leaves R and the tail trim runs.
            if seed >= 120 {
                let j = seed - 120;
                let k = 3 + (j / 2) % 3;
                let head = j % 2 == 0;
                let r0 = (r.next_u32() & 0x7fff_ffff) | 1;
                let r1 = if (j / 6) % 2 == 1 {
                    (r.next_u32() & 0x7fff_ffff) | 1
                } else {
                    0
                };
                c.bus.write32(SRC + 0x10, k);
                c.bus.write32(DST + 0x10, k);
                for i in 0..k {
                    let (s, b) = if i == k - 1 {
                        (5, if head { 6 } else { 5 })
                    } else if head {
                        (0xffff_ffff, [r0 - 1, r1, 0][i.min(2) as usize])
                    } else {
                        let s = (r.next_u32() & 0x7fff_ffff) | 0x1_0000;
                        (s, s + [r0, r1, 0][i.min(2) as usize])
                    };
                    c.bus.write32(SRC + 0x14 + i * 4, s);
                    c.bus.write32(DST + 0x14 + i * 4, b);
                }
            }
            c.g[A0][0] = DST as u64;
            c.g[A1][0] = SRC as u64;
            c.pc = 0x0042_2560;
        });
    }

    #[test]
    fn sscanf_matches() {
        check(ee::s15::f_004178b0__generated, sscanf, 120, &|c, seed| {
            let mut r = file_fixture(c, seed, 0x78b0, 1);
            let (input, fmt) = match seed % 4 {
                0 => (&b"123 abc"[..], &b"%d %s"[..]),
                1 => (&b"  -45 67"[..], &b"%d %d"[..]),
                2 => (&b"0x1f!"[..], &b"%x %c"[..]),
                _ => (&b"9 8 7"[..], &b"%d %d %d"[..]),
            };
            for (i, b) in input.iter().enumerate() {
                c.bus.write8(SRC + i as u32, *b);
            }
            c.bus.write8(SRC + input.len() as u32, 0);
            for (i, b) in fmt.iter().enumerate() {
                c.bus.write8(FMT + i as u32, *b);
            }
            c.bus.write8(FMT + fmt.len() as u32, 0);
            c.g[A0][0] = SRC as u64;
            c.g[A1][0] = FMT as u64;
            c.g[A2][0] = (ARGS) as u64;
            c.g[A3][0] = (ARGS + 4) as u64;
            c.g[T0][0] = (ARGS + 8) as u64;
            let _ = &mut r;
        });
    }

    #[test]
    fn std_matches() {
        check(ee::s15::f_0041ff40__generated, std, 60, &|c, seed| {
            let mut r = Rng(0xff40 ^ seed as u64);
            scramble(c, &mut r);
            for i in 0..0x60 {
                c.bus.write8(DST + i as u32, 0xcc);
            }
            c.g[A0][0] = DST as u64;
            c.g[A1][0] = [0x200, 0x208, 0x200 | 0x40][seed as usize % 3] as u64;
            c.g[A2][0] = seed as u64 & 0xffff;
            c.g[A3][0] = c.g[A0][0];
        });
    }

    #[test]
    fn strcat_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let (mut r, _, _) = str_pair(c, seed, 0x62d0);
            // Long `dst` so the scan loops past several event checks.
            for i in 0..96 {
                c.bus.write8(DST + i as u32, b'a' + (i % 26) as u8);
            }
            c.bus.write8(DST + 96, 0);
            c.g[A0][0] = DST as u64;
            c.g[A1][0] = SRC as u64;
            schedule(c, &mut r, seed, 0x0041_62d0)
        };
        check_events(ee::s15::f_004162d0__generated, strcat, 60, &setup);
    }

    #[test]
    fn strcat_matches() {
        check(ee::s15::f_004162d0__generated, strcat, 300, &|c, seed| {
            let (r, dst_end, src_end) = str_pair(c, seed, 0x62d0);
            let _ = r;
            // Stay at or before each terminator: the scan must find one.
            // Offsets reach 8 so the 8-aligned word loop runs too.
            c.g[A0][0] = (DST + (seed % 9).min(dst_end)) as u64;
            c.g[A1][0] = (SRC + ((seed / 3) % 9).min(src_end)) as u64;
        });
    }

    #[test]
    fn strcspn_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0xab08 ^ seed as u64);
            scramble(c, &mut r);
            for i in 0..64 {
                c.bus.write8(SRC + i as u32, b'x');
            }
            c.bus.write8(SRC + 64, 0);
            for i in 0..96 {
                c.bus.write8(DST + i as u32, b'y' + (i % 26) as u8);
            }
            c.bus.write8(DST + 96, 0);
            c.g[A0][0] = SRC as u64;
            c.g[A1][0] = DST as u64;
            schedule(c, &mut r, seed, 0x0041_ab08)
        };
        check_events(ee::s15::f_0041ab08__generated, strcspn, 60, &setup);
    }

    #[test]
    fn strcspn_matches() {
        check(ee::s15::f_0041ab08__generated, strcspn, 200, &|c, seed| {
            span_pair(c, seed, 0xab08);
        });
    }

    #[test]
    fn strncat_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let (mut r, _, _) = str_pair(c, seed, 0x6958);
            for i in 0..96 {
                c.bus.write8(DST + i as u32, b'a' + (i % 26) as u8);
            }
            c.bus.write8(DST + 96, 0);
            c.g[A0][0] = DST as u64;
            c.g[A1][0] = SRC as u64;
            c.g[A2][0] = 64;
            schedule(c, &mut r, seed, 0x0041_6958)
        };
        check_events(ee::s15::f_00416958__generated, strncat, 60, &setup);
    }

    #[test]
    fn strncat_matches() {
        check(ee::s15::f_00416958__generated, strncat, 300, &|c, seed| {
            let (mut r, dst_end, src_end) = str_pair(c, seed, 0x6958);
            let n = match (seed / 27) % 6 {
                0 => 0,  // no copy, no NUL
                1 => 2,  // short copy
                2 => 40, // past a long `src`
                3 => (r.next_u32() % 32) as u32,
                4 => (r.next_u32() % 32) as u32,
                _ => 64, // whole long `src` + NUL
            };
            c.g[A0][0] = (DST + (seed % 9).min(dst_end)) as u64;
            c.g[A1][0] = (SRC + ((seed / 3) % 9).min(src_end)) as u64;
            c.g[A2][0] = n as u64;
        });
    }

    #[test]
    fn strpbrk_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0xac40 ^ seed as u64);
            scramble(c, &mut r);
            for i in 0..128 {
                c.bus.write8(SRC + i as u32, b'm');
            }
            c.bus.write8(SRC + 128, 0);
            for i in 0..64 {
                c.bus.write8(DST + i as u32, b'n' + (i % 8) as u8);
            }
            c.bus.write8(DST + 64, 0);
            c.g[A0][0] = SRC as u64;
            c.g[A1][0] = DST as u64;
            schedule(c, &mut r, seed, 0x0041_ac40)
        };
        check_events(ee::s15::f_0041ac40__generated, strpbrk, 60, &setup);
    }

    #[test]
    fn strpbrk_matches() {
        check(ee::s15::f_0041ac40__generated, strpbrk, 200, &|c, seed| {
            let mut r = Rng(0xac40 ^ seed as u64);
            scramble(c, &mut r);
            let s: &[u8] = match seed % 6 {
                0 => b"",
                1 => b"abc",
                2 => b"hello world",
                3 => b"aaaaab",
                4 => &[b'q'; 64],
                _ => b"xyz",
            };
            let ac: &[u8] = match (seed / 6) % 5 {
                0 => b"",
                1 => b"b",
                2 => b"zyx",
                3 => b"ole",
                _ => b"qq",
            };
            place_str(c, SRC, s);
            place_str(c, DST, ac);
            c.g[A0][0] = SRC as u64;
            c.g[A1][0] = DST as u64;
        });
    }

    #[test]
    fn strrchr_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0xacc0 ^ seed as u64);
            scramble(c, &mut r);
            for i in 0..256 {
                c.bus.write8(SRC + i as u32, b'k');
            }
            c.bus.write8(SRC + 256, 0);
            c.g[A0][0] = SRC as u64;
            c.g[A1][0] = b'z' as u64; // absent: scans the whole string.
            schedule(c, &mut r, seed, 0x0041_acc0)
        };
        check_events(ee::s15::f_0041acc0__generated, strrchr, 60, &setup);
    }

    #[test]
    fn strrchr_matches() {
        check(ee::s15::f_0041acc0__generated, strrchr, 200, &|c, seed| {
            let mut r = Rng(0xacc0 ^ seed as u64);
            scramble(c, &mut r);
            let s: &[u8] = match seed % 5 {
                0 => b"",
                1 => b"a",
                2 => b"hello world",
                3 => b"aaaaa",
                _ => &[b'z'; 64],
            };
            place_str(c, SRC, s);
            c.g[A0][0] = SRC as u64;
            c.g[A1][0] = [0, 0x61, 0x7a, 0x80, 0xff, 0x141][seed as usize % 6] as u64;
        });
    }

    #[test]
    fn strspn_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0xad10 ^ seed as u64);
            scramble(c, &mut r);
            for i in 0..128 {
                c.bus.write8(SRC + i as u32, b'a' + (i % 3) as u8);
            }
            c.bus.write8(SRC + 128, 0);
            place_str(c, DST, b"abc");
            c.g[A0][0] = SRC as u64;
            c.g[A1][0] = DST as u64;
            schedule(c, &mut r, seed, 0x0041_ad10)
        };
        check_events(ee::s15::f_0041ad10__generated, strspn, 60, &setup);
    }

    #[test]
    fn strspn_matches() {
        check(ee::s15::f_0041ad10__generated, strspn, 200, &|c, seed| {
            span_pair(c, seed, 0xad10);
        });
    }

    #[test]
    fn strstr_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0xad80 ^ seed as u64);
            scramble(c, &mut r);
            // Near-matches at every position: the inner compare loop runs.
            for i in 0..200 {
                c.bus.write8(SRC + i as u32, b'a');
            }
            c.bus.write8(SRC + 200, 0);
            for i in 0..64 {
                c.bus.write8(DST + i as u32, b'a');
            }
            c.bus.write8(DST + 64, b'b');
            c.bus.write8(DST + 65, 0);
            c.g[A0][0] = SRC as u64;
            c.g[A1][0] = DST as u64;
            schedule(c, &mut r, seed, 0x0041_ad80)
        };
        check_events(ee::s15::f_0041ad80__generated, strstr, 60, &setup);
    }

    #[test]
    fn strstr_matches() {
        check(ee::s15::f_0041ad80__generated, strstr, 200, &|c, seed| {
            let mut r = Rng(0xad80 ^ seed as u64);
            scramble(c, &mut r);
            let hay: &[u8] = match seed % 5 {
                0 => b"",
                1 => b"abc",
                2 => b"hello world hello",
                3 => &[b'a'; 64],
                _ => b"mississippi",
            };
            let ndl: &[u8] = match (seed / 5) % 6 {
                0 => b"",
                1 => b"a",
                2 => b"world",
                3 => b"zzz",
                4 => b"aa",
                _ => b"hello world hello!",
            };
            place_str(c, SRC, hay);
            place_str(c, DST, ndl);
            c.g[A0][0] = SRC as u64;
            c.g[A1][0] = DST as u64;
        });
    }

    #[test]
    fn strtod_matches() {
        check(ee::s15::f_0041a7e8__generated, strtod, 200, &|c, seed| {
            let mut r = Rng(0xa7e8 ^ seed as u64);
            wire_reent(c, &mut r);
            let s: &[u8] = match seed % 10 {
                0 => b"123.45",
                1 => b"-0.5",
                2 => b"  +3.14xyz",
                3 => b"1e10",
                4 => b"abc",
                5 => b"",
                6 => b"0x10",
                7 => b"12.5e-3",
                8 => b".5",
                _ => b"1e999", // overflow: HUGE_VAL + ERANGE.
            };
            place_str(c, SRC, s);
            c.g[A0][0] = SRC as u64;
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { SCR as u64 };
        });
    }

    #[test]
    fn strtof_matches() {
        check(ee::s15::f_0041a818__generated, strtof, 200, &|c, seed| {
            let mut r = Rng(0xa818 ^ seed as u64);
            wire_reent(c, &mut r);
            let s: &[u8] = match seed % 10 {
                0 => b"123.45",
                1 => b"-0.5",
                2 => b"  +3.14xyz",
                3 => b"1e10",
                4 => b"abc",
                5 => b"",
                6 => b"0x10",
                7 => b"12.5e-3",
                8 => b".5",
                // NOTE: huge exponents (e.g. `1e99`) drive the shared
                // `_dtoa_r` Bignum path, which needs boot-initialized reent
                // state and spins on a scratch reent; both sides loop
                // identically there, so it is out of differential scope.
                _ => b"99.99",
            };
            place_str(c, SRC, s);
            c.g[A0][0] = SRC as u64;
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { SCR as u64 };
        });
    }

    #[test]
    fn strtol_matches() {
        check(ee::s15::f_0041e9e0__generated, strtol, 200, &|c, seed| {
            let mut r = Rng(0xe9e0 ^ seed as u64);
            wire_reent(c, &mut r);
            let s: &[u8] = match seed % 10 {
                0 => b"123",
                1 => b"  -45",
                2 => b"0x1F",
                3 => b"010",
                4 => b"zz",
                5 => b"",
                6 => b"2147483647",
                7 => b"-2147483649",
                8 => b"101",
                _ => b"  +0Xab",
            };
            place_str(c, SRC, s);
            c.g[A0][0] = SRC as u64;
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { SCR as u64 };
            c.g[A2][0] = [0, 0, 8, 10, 16, 2, 36][seed as usize % 7] as u64;
        });
    }

    #[test]
    fn strtoul_matches() {
        check(ee::s15::f_0041aa50__generated, strtoul, 200, &|c, seed| {
            let mut r = Rng(0xaa50 ^ seed as u64);
            wire_reent(c, &mut r);
            let s: &[u8] = match seed % 10 {
                0 => b"123",
                1 => b"  -45",
                2 => b"0x1F",
                3 => b"010",
                4 => b"zz",
                5 => b"",
                6 => b"18446744073709551615",
                7 => b"18446744073709551616",
                8 => b"101",
                _ => b"  +0Xab",
            };
            place_str(c, SRC, s);
            c.g[A0][0] = SRC as u64;
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { SCR as u64 };
            c.g[A2][0] = [0, 0, 8, 10, 16, 2, 36][seed as usize % 7] as u64;
        });
    }

    #[test]
    fn ungetc_matches() {
        check(ee::s15::f_00420378__generated, ungetc, 200, &|c, seed| {
            let mut r = file_fixture_3(c, seed, 0x0378, 3);
            match seed % 11 {
                0 => {
                    // `EOF`: nothing pushed back.
                    c.g[A0][0] = 0xffffffffffffffff;
                    c.g[A1][0] = FILE_3 as u64;
                }
                1 => {
                    // Push back onto a buffered read stream.
                    c.bus.write32(FILE_3 + 0x4, 8);
                    c.g[A0][0] = b'A' as u64;
                    c.g[A1][0] = FILE_3 as u64;
                }
                2 => {
                    // Empty read buffer: the `_ubuf` path.
                    c.bus.write32(FILE_3 + 0x4, 0);
                    c.g[A0][0] = b'B' as u64;
                    c.g[A1][0] = FILE_3 as u64;
                }
                3 => {
                    // Write mode: flush first.
                    c.bus.write16(FILE_3 + 0xc, 0x208);
                    c.bus.write32(FILE_3 + 0x10, SRC);
                    c.bus.write32(FILE_3, SRC + 4);
                    c.g[A0][0] = b'C' as u64;
                    c.g[A1][0] = FILE_3 as u64;
                }
                4 => {
                    // Unfiltered character (>255 masked to a byte).
                    c.bus.write32(FILE_3 + 0x4, 3);
                    c.g[A0][0] = 0x141;
                    c.g[A1][0] = FILE_3 as u64;
                }
                5 => {
                    // Write mode with the 0x10 bit: the flush path runs.
                    c.bus.write16(FILE_3 + 0xc, 0x218);
                    c.bus.write32(FILE_3 + 0x10, SRC);
                    c.bus.write32(FILE_3, SRC + 4);
                    c.g[A0][0] = b'D' as u64;
                    c.g[A1][0] = FILE_3 as u64;
                }
                6 => {
                    // Read count at the `_ubuf` limit.
                    c.bus.write32(FILE_3 + 0x4, 5);
                    c.bus.write32(FILE_3 + 0x34, 5);
                    c.bus.write32(FILE_3 + 0x30, DST);
                    c.g[A0][0] = b'E' as u64;
                    c.g[A1][0] = FILE_3 as u64;
                }
                7 => {
                    // Read count past it.
                    c.bus.write32(FILE_3 + 0x4, 8);
                    c.bus.write32(FILE_3 + 0x34, 3);
                    c.bus.write32(FILE_3 + 0x30, DST);
                    c.g[A0][0] = b'F' as u64;
                    c.g[A1][0] = FILE_3 as u64;
                }
                8 => {
                    // No buffer base: the null-base path.
                    c.bus.write32(FILE_3 + 0x10, 0);
                    c.bus.write32(FILE_3 + 0x4, 8);
                    c.g[A0][0] = b'G' as u64;
                    c.g[A1][0] = FILE_3 as u64;
                }
                9 => {
                    // Write mode with a FAKE cookie the flush can call:
                    // the flush succeeds, reaching the `& ~0x8` clear
                    // (covers the flag-clear store).
                    use crate::difftest::fakes::{
                        fake_call, fake_jump_out, fake_syscall, FAKE, LOG_RESULTS, LOG_UNWIND_AT,
                    };
                    c.hooks.miss = fake_call;
                    c.hooks.jump_out = fake_jump_out;
                    c.hooks.syscall = fake_syscall;
                    c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                    c.bus.write16(FILE_3 + 0xc, 0x218);
                    c.bus.write32(FILE_3 + 0x10, SRC);
                    c.bus.write32(FILE_3, SRC + 4);
                    c.bus.write32(FILE_3 + 0x1c, 3);
                    c.bus.write32(FILE_3 + 0x24, FAKE);
                    c.bus.write32(LOG_RESULTS, 4);
                    c.g[A0][0] = b'G' as u64;
                    c.g[A1][0] = FILE_3 as u64;
                }
                _ => {
                    // Advanced position: `_p` past the buffer base.
                    c.bus.write32(FILE_3 + 0x10, SRC);
                    c.bus.write32(FILE_3, SRC + 10);
                    c.bus.write32(FILE_3 + 0x4, 8);
                    c.g[A0][0] = b'H' as u64;
                    c.g[A1][0] = FILE_3 as u64;
                }
            }
            let _ = &mut r;
        });
    }

    #[test]
    fn vfiprintf_buffered_write_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0xc938 ^ seed as u64);
            wire_reent(c, &mut r);
            c.bus.write32(0x0045_4ce0, 0);
            c.bus.write32(0x0045_4ce4, DST);
            for i in 0..0x400 {
                c.bus.write8(DST + i as u32, 0xcc);
                c.bus.write8(SRC + i as u32, b'e');
            }
            c.g[A0][0] = 3;
            c.g[A1][0] = SRC as u64;
            c.g[A2][0] = 200;
            c.g[A3][0] = 0;
            schedule(c, &mut r, seed, 0x0041_c938)
        };
        check_events(
            ee::s15::f_0041c938__generated,
            vfiprintf_buffered_write,
            60,
            &setup,
        );
    }

    #[test]
    fn vfiprintf_buffered_write_matches() {
        check(
            ee::s15::f_0041c938__generated,
            vfiprintf_buffered_write,
            200,
            &|c, seed| {
                let mut r = Rng(0xc938 ^ seed as u64);
                wire_reent(c, &mut r);
                c.pc = 0x0041_c938;
                c.bus
                    .write32(0x0045_4ce0, [0, 5, 126, 127][seed as usize % 4]);
                c.bus.write32(0x0045_4ce4, DST);
                for i in 0..0x200 {
                    c.bus.write8(DST + i as u32, 0xcc);
                    c.bus.write8(SRC + i as u32, b'w' + (i % 5) as u8);
                }
                c.g[A0][0] = 3;
                c.g[A1][0] = SRC as u64;
                c.g[A2][0] = [0, 1, 10, 64, 130][seed as usize % 5] as u64;
                c.g[A3][0] = [0, 1, 2][seed as usize % 3] as u64;
            },
        );
    }

    #[test]
    fn vfiprintf_matches() {
        check(
            ee::s15::f_0041ca40__generated,
            vfiprintf,
            200,
            &|c, seed| {
                let mut r = file_fixture_2(c, seed, 0xca40, 3);
                let fmt: &[u8] = match seed % 6 {
                    0 => b"hi",
                    1 => b"%d",
                    2 => b"%s|%x",
                    3 => b"%c%c%c",
                    4 => b"%u %o",
                    _ => b"a%db",
                };
                place_str(c, FMT_2, fmt);
                place_str(c, SRC, b"hi");
                c.bus.write32(ARGS_2, 0x1234);
                c.bus.write32(ARGS_2 + 4, SRC);
                c.bus.write32(ARGS_2 + 8, 0x41);
                c.g[A0][0] = FILE_4 as u64;
                c.g[A1][0] = FMT_2 as u64;
                c.g[A2][0] = ARGS_2 as u64;
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn vsnprintf_matches() {
        check(
            ee::s15::f_00418720__generated,
            vsnprintf,
            120,
            &|c, seed| {
                let _ = printf_case(c, seed, 0x8720);
                c.g[A0][0] = OUT_2 as u64;
                // Small bound for some seeds (bounded store), huge for others.
                c.g[A1][0] = (match seed % 3 {
                    0 => 0u32,
                    1 => 8u32,
                    _ => 0x7fff_ffffu32,
                }) as u64;
                c.g[A2][0] = FMT as u64;
                c.g[A3][0] = ARGS as u64;
            },
        );
    }

    #[test]
    fn vsprintf_matches() {
        check(ee::s15::f_004186c8__generated, vsprintf, 120, &|c, seed| {
            let _ = printf_case(c, seed, 0x86c8);
            c.g[A0][0] = OUT_2 as u64;
            c.g[A1][0] = FMT as u64;
            c.g[A2][0] = ARGS as u64;
        });
    }

    #[test]
    fn i2b_matches() {
        check(ee::s15::f_00420978__generated, i2b, 120, &|c, seed| {
            let mut r = Rng(0x0978 ^ seed as u64);
            heap_init(c, &mut r);
            c.g[A0][0] = REENT_3 as u64;
            c.g[A1][0] = [0, 1, 0x12345, 0x7fffffff, 2, 9][seed as usize % 6] as u64;
        });
    }

    #[test]
    fn sccl_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0x85d0 ^ seed as u64);
            scramble(c, &mut r);
            // A long class so the member loop passes event checks.
            for i in 0..64 {
                c.bus.write8(FMT + i as u32, b'a' + (i % 26) as u8);
            }
            c.bus.write8(FMT + 64, b']');
            c.g[A0][0] = OUT_2 as u64;
            c.g[A1][0] = FMT as u64;
            schedule(c, &mut r, seed, 0x0041_85d0)
        };
        check_events(ee::s15::f_004185d0__generated, sccl, 60, &setup);
    }

    #[test]
    fn sccl_matches() {
        check(ee::s15::f_004185d0__generated, sccl, 200, &|c, seed| {
            let mut r = Rng(0x85d0 ^ seed as u64);
            scramble(c, &mut r);
            let fmt: &[u8] = match seed % 11 {
                0 => b"abc]",
                1 => b"^a-z]",
                2 => b"a-z]",
                3 => b"a-]",
                4 => b"-az]",
                5 => b"a-c-e]",
                6 => b"]",
                7 => b"z-a]",
                8 => b"x.]",
                9 => b"a-a]",
                _ => b"a!]",
            };
            for (i, b) in fmt.iter().enumerate() {
                c.bus.write8(FMT + i as u32, *b);
            }
            c.g[A0][0] = OUT_2 as u64;
            c.g[A1][0] = FMT as u64;
            let _ = &mut r;
        });
    }

    #[test]
    fn sfmoreglue_matches() {
        check(
            ee::s15::f_0041ff98__generated,
            sfmoreglue,
            120,
            &|c, seed| {
                let mut r = Rng(0xff98 ^ seed as u64);
                heap_init(c, &mut r);
                c.g[A0][0] = REENT_3 as u64;
                c.g[A1][0] = [1, 2, 4, 8][seed as usize % 4] as u64;
            },
        );
    }

    #[test]
    fn sfp_events() {
        // Every slot busy: the glue walk extends past the `0x420038` /
        // `0x420048` edges with a due VBlank.
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0x0010 ^ seed as u64);
            heap_init(c, &mut r);
            c.bus.write32(REENT_3 + 0x38, 1);
            c.bus.write32(REENT_3 + 0x1d8, GLUE_2);
            c.bus.write32(GLUE_2, 0);
            c.bus.write32(GLUE_2 + 0x4, 1);
            c.bus.write32(GLUE_2 + 0x8, FILE_4);
            c.bus.write16(FILE_4 + 0xc, 1); // busy
            c.bus.write32(FILE_4 + 0x54, REENT_3);
            c.g[A0][0] = REENT_3 as u64;
            let _ = &mut r;
            schedule(c, &mut r, seed, 0x0042_0010)
        };
        check_events(ee::s15::f_00420010__generated, sfp, 60, &setup);
    }

    #[test]
    fn sfp_matches() {
        check(ee::s15::f_00420010__generated, sfp, 200, &|c, seed| {
            let mut r = Rng(0x0010 ^ seed as u64);
            heap_init(c, &mut r);
            match seed % 3 {
                0 => {
                    // Never initialised: `__sinit` runs first.
                    c.bus.write32(REENT_3 + 0x38, 0);
                }
                1 => {
                    // One free slot in this block.
                    c.bus.write32(REENT_3 + 0x38, 1);
                    c.bus.write32(REENT_3 + 0x1d8, GLUE_2);
                    c.bus.write32(GLUE_2, 0);
                    c.bus.write32(GLUE_2 + 0x4, 2);
                    c.bus.write32(GLUE_2 + 0x8, FILE_4);
                    c.bus.write16(FILE_4 + 0xc, 1); // busy
                    c.bus.write16(FILE_4 + 0x58 + 0xc, 0); // free
                    c.bus.write32(FILE_4 + 0x54, REENT_3);
                    c.bus.write32(FILE_4 + 0x58 + 0x54, 0);
                }
                _ => {
                    // Every slot busy: extend the glue.
                    c.bus.write32(REENT_3 + 0x38, 1);
                    c.bus.write32(REENT_3 + 0x1d8, GLUE_2);
                    c.bus.write32(GLUE_2, 0);
                    c.bus.write32(GLUE_2 + 0x4, 1);
                    c.bus.write32(GLUE_2 + 0x8, FILE_4);
                    c.bus.write16(FILE_4 + 0xc, 1); // busy
                    c.bus.write32(FILE_4 + 0x54, REENT_3);
                }
            }
            c.g[A0][0] = REENT_3 as u64;
            let _ = &mut r;
        });
    }

    #[test]
    fn sinit_matches() {
        check(ee::s15::f_004200f0__generated, sinit, 60, &|c, seed| {
            let mut r = Rng(0x00f0 ^ seed as u64);
            wire_reent(c, &mut r);
            c.bus.write32(REENT_3 + 0x38, 0);
            c.g[A0][0] = REENT_3 as u64;
            let _ = seed;
        });
    }

    #[test]
    fn smakebuf_matches() {
        check(ee::s15::f_004174b0__generated, smakebuf, 200, &|c, seed| {
            let mut r = file_fixture(c, seed, 0x74b0, 3);
            match seed % 5 {
                0 => c.bus.write16(FILE_2 + 0xc, 0x22), // unbuffered
                1 => {
                    // No `fstat` info: character device + tty buffer word.
                    c.bus.write16(FILE_2 + 0xe, 0xffff);
                }
                2 => {
                    // `fstat` path: regular file shape via the stack stat.
                    c.bus.write16(FILE_2 + 0xe, 3);
                }
                3 => {
                    // fd 0 is a real descriptor: `fstat` still runs.
                    c.bus.write16(FILE_2 + 0xe, 0);
                }
                _ => {
                    c.bus.write16(FILE_2 + 0xe, 3);
                    c.bus.write16(FILE_2 + 0xc, 0x208);
                }
            }
            c.g[A0][0] = FILE_2 as u64;
            let _ = &mut r;
        });
    }

    #[test]
    fn smakebuf_fstat_success_matches() {
        // The gated `_fstat_r` reports a regular file: the tty cookie
        // takes the non-tty device tail, any other cookie the shared
        // flush tail.
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = file_fixture(c, seed, 0x74b0, 3);
            libc_arm_fakes(c, 3, 0x8000, 0);
            if seed / 2 % 3 == 2 {
                // A failing `fstat` (-1) takes the only arm that runs
                // the block tail; the game's `_fstat` never fails.
                c.bus.write32(FAKE_GATE + FAKE_RESULT_OFF, 0xffff_ffff);
            }
            c.bus.write16(FILE_2 + 0xe, 3);
            c.bus.write32(
                FILE_2 + 0x28,
                if seed % 2 == 0 {
                    0x0041_7a30
                } else {
                    0x0014_06c0
                },
            );
            c.g[A0][0] = FILE_2 as u64;
            let _ = &mut r;
        };
        check_paths(
            ee::s15::f_004174b0__generated,
            smakebuf,
            60,
            &[42, 45, 34],
            &setup,
        );
    }

    #[test]
    fn srefill_matches() {
        check(ee::s15::f_00417648__generated, srefill, 120, &|c, seed| {
            let mut r = file_fixture(c, seed, 0x7648, 3);
            c.pc = 0x0041_7648;
            match seed % 8 {
                0 => c.bus.write16(FILE_2 + 0xc, 0x20), // unbuffered-ish: EOF
                1 => {
                    // `__SRD` with a short read pending.
                    c.bus.write16(FILE_2 + 0xc, 0x14);
                    c.bus.write32(FILE_2 + 0x30, SRC);
                }
                2 => {
                    // `__SWR` + `__SRD`: flush path.
                    c.bus.write16(FILE_2 + 0xc, 0x218);
                }
                3 => {
                    // Plain read with a cookie that reports EOF.
                    c.bus.write16(FILE_2 + 0xc, 0x10);
                    c.bus.write32(FILE_2 + 0x28, 0x0014_06c0);
                }
                4 => {
                    // `__SWR` without `__SRD`: EOF, no flush
                    // (covers the no-bit-4 case).
                    c.bus.write16(FILE_2 + 0xc, 0x8);
                }
                5 => {
                    c.bus.write16(FILE_2 + 0xc, 0x10);
                    c.bus.write32(FILE_2 + 0x28, 0x0014_06c0);
                    c.bus.write32(FILE_2 + 0x10, OUT_2);
                }
                6 => {
                    // Unget buffer restored and present: the refill
                    // returns 0 straight from `0x41770c`.
                    c.bus.write16(FILE_2 + 0xc, 0x14);
                    c.bus.write32(FILE_2 + 0x30, SRC);
                    c.bus.write32(FILE_2 + 0x3c, SRC + 4);
                }
                _ => {
                    // No buffer base: the refill runs `__smakebuf`.
                    c.bus.write16(FILE_2 + 0xc, 0x10);
                    c.bus.write32(FILE_2 + 0x28, 0x0014_06c0);
                    c.bus.write32(FILE_2 + 0x10, 0);
                }
            }
            c.g[A0][0] = FILE_2 as u64;
            let _ = &mut r;
        });
    }

    #[test]
    fn srefill_success_matches() {
        use crate::difftest::fakes::{
            fake_call, fake_jump_out, fake_syscall, FAKE, LOG_RESULTS, LOG_UNWIND_AT,
        };
        // The read-cookie half of `__srefill`: the fixture stream reports
        // `__SRD`, and the cookie at `FILE_2 + 0x20` decides the outcome.
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = file_fixture(c, seed, 0x7648, 3);
            c.pc = 0x0041_7648;
            c.bus.write16(FILE_2 + 0xc, 0x10);
            c.bus.write32(FILE_2 + 0x1c, 3);
            c.bus.write32(FILE_2 + 0x10, OUT_2);
            c.g[A0][0] = FILE_2 as u64;
            match seed % 4 {
                // A fake cookie reporting a full/short transfer: success (0).
                0 | 1 => {
                    c.hooks.miss = fake_call;
                    c.hooks.jump_out = fake_jump_out;
                    c.hooks.syscall = fake_syscall;
                    // Fresh RAM leaves `LOG_UNWIND_AT` at 0, which would
                    // unwind the first fake call; never unwind here.
                    c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                    c.bus.write32(FILE_2 + 0x20, FAKE);
                    c.bus
                        .write32(LOG_RESULTS, if seed % 4 == 0 { 0x100 } else { 2 });
                }
                // The real `_close_r` as the cookie: the close fails.
                2 => {
                    c.bus.write32(FILE_2 + 0x1c, REENT_3);
                    c.bus.write32(FILE_2 + 0x20, 0x0041_fb98);
                }
                // Flags needing the `_fwalk` settle, then a cookie at EOF.
                _ => {
                    c.bus.write16(FILE_2 + 0xc, 0x11);
                    c.bus.write32(FILE_2 + 0x20, 0x0014_06c0);
                }
            }
            let _ = &mut r;
        };
        check(ee::s15::f_00417648__generated, srefill, 120, &setup);
        // The success path returns 0.
        let mut a = crate::difftest::fresh();
        let mut b = crate::difftest::fresh();
        setup(&mut a, 0);
        setup(&mut b, 0);
        ee::s15::f_00417648__generated(&mut a);
        srefill(&mut b);
        assert_eq!(a.g[V0][0], 0);
        assert_eq!(b.g[V0][0], 0);
    }

    #[test]
    fn srefill_events() {
        // The EOF exit checks for an event at the `0x417794` edge, so a
        // cookie at EOF plus a due VBlank exercises it.
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = file_fixture(c, seed, 0x7648, 3);
            c.pc = 0x0041_7648;
            c.bus.write16(FILE_2 + 0xc, 0x10);
            c.bus.write32(FILE_2 + 0x1c, 3);
            c.bus.write32(FILE_2 + 0x10, OUT_2);
            c.bus.write32(FILE_2 + 0x20, 0x0014_06c0);
            c.g[A0][0] = FILE_2 as u64;
            let _ = &mut r;
            schedule(c, &mut r, seed, 0x0041_7648)
        };
        check_events(ee::s15::f_00417648__generated, srefill, 60, &setup);
    }

    /// A closed `FILE_2` (flags 0) with a 0x400-byte output buffer and the
    /// given `_file` descriptor; reent wired, glue with one slot.
    pub(crate) fn file_fixture(c: &mut Cpu, seed: u32, salt: u64, fd: u16) -> Rng {
        let mut r = Rng(salt ^ seed as u64);
        scramble(c, &mut r);
        for j in 0..0x400 {
            c.bus.write8(OUT_2 + j as u32, 0xcc);
        }
        c.bus.write32(REENT_3 + 0x38, 1); // skip `__sinit`
        c.bus.write32(GLUE + 0x4, 1);
        c.bus.write32(GLUE + 0x8, FILE_2);
        c.bus.write32(GLUE, 0);
        c.bus.write32(REENT_3 + 0x1d8, GLUE);
        c.bus.write32(FILE_2, OUT_2); // _p
        c.bus.write32(FILE_2 + 0x4, 0); // _r
        c.bus.write32(FILE_2 + 0x8, 0x400); // _w
        c.bus.write16(FILE_2 + 0xc, 0x200); // __SWR
        c.bus.write16(FILE_2 + 0xe, fd); // _file
        c.bus.write32(FILE_2 + 0x10, OUT_2); // _bf
        c.bus.write32(FILE_2 + 0x14, 0x100);
        c.bus.write32(FILE_2 + 0x54, REENT_3); // _data
        r
    }

    /// A closed `FILE_2` (write mode) with a 0x400-byte output buffer and
    /// the given `_file` descriptor; reent wired, glue with one slot.
    pub(crate) fn file_fixture_2(c: &mut Cpu, seed: u32, salt: u64, fd: u16) -> Rng {
        let mut r = Rng(salt ^ seed as u64);
        wire_reent(c, &mut r);
        for j in 0..0x400 {
            c.bus.write8(OUT_3 + j as u32, 0xcc);
        }
        c.bus.write32(REENT_3 + 0x38, 1); // skip `__sinit`
        c.bus.write32(GLUE_2 + 0x4, 1);
        c.bus.write32(GLUE_2 + 0x8, FILE_4);
        c.bus.write32(GLUE_2, 0);
        c.bus.write32(REENT_3 + 0x1d8, GLUE_2);
        c.bus.write32(FILE_4, OUT_3); // _p
        c.bus.write32(FILE_4 + 0x4, 0); // _r
        c.bus.write32(FILE_4 + 0x8, 0x400); // _w
        c.bus.write16(FILE_4 + 0xc, 0x200); // __SWR
        c.bus.write16(FILE_4 + 0xe, fd); // _file
        c.bus.write32(FILE_4 + 0x10, OUT_3); // _bf base
        c.bus.write32(FILE_4 + 0x14, 0x100);
        c.bus.write32(FILE_4 + 0x1c, 0); // cookie
        c.bus.write32(FILE_4 + 0x24, 0x0014_06c0); // write fn
        c.bus.write32(FILE_4 + 0x28, 0x0014_06c0); // read fn
        c.bus.write32(FILE_4 + 0x54, REENT_3); // _data
        r
    }

    /// A closed `FILE_2` (read mode) with scratch read/write areas.
    pub(crate) fn file_fixture_3(c: &mut Cpu, seed: u32, salt: u64, fd: u16) -> Rng {
        let mut r = Rng(salt ^ seed as u64);
        wire_reent(c, &mut r);
        for j in 0..0x400 {
            c.bus.write8(DST + j as u32, 0xcc);
            c.bus.write8(SRC + j as u32, b'r' + (j % 7) as u8);
        }
        c.bus.write32(REENT_3 + 0x38, 1); // skip `__sinit`
        c.bus.write32(FILE_3 + 0x0, SRC); // _p
        c.bus.write32(FILE_3 + 0x4, 0); // _r
        c.bus.write32(FILE_3 + 0x8, 0x400); // _w
        c.bus.write16(FILE_3 + 0xc, 0x10); // __SRD
        c.bus.write16(FILE_3 + 0xe, fd); // _file
        c.bus.write32(FILE_3 + 0x10, SRC); // _bf base
        c.bus.write32(FILE_3 + 0x14, 0x100);
        c.bus.write32(FILE_3 + 0x1c, 0); // cookie
        c.bus.write32(FILE_3 + 0x24, 0x0014_06c0); // write fn
        c.bus.write32(FILE_3 + 0x28, 0x0014_06c0); // read fn
        c.bus.write32(FILE_3 + 0x30, 0); // _ubuf base
        c.bus.write32(FILE_3 + 0x34, 0);
        c.bus.write32(FILE_3 + 0x38, 0);
        c.bus.write32(FILE_3 + 0x3c, 0);
        c.bus.write32(FILE_3 + 0x54, REENT_3); // _data
        r
    }

    /// Boot-equivalent heap for the malloc family: the `_sbrk` start
    /// pointer plus the kernel `EndOfHeap` ceiling on a main thread.
    /// Without it every allocation returns NULL (and the test would pass
    /// vacuously). Call before setting up `A0`/regs.
    pub(crate) fn heap_init(c: &mut Cpu, r: &mut Rng) {
        wire_reent(c, r);
        c.bus.write32(0x0045_516c, 0x0090_0000);
        ssx3_ps2::kernel::start_main_thread(c, 0x0041_88c0, 0, 0);
        ssx3_ps2::platform::machine_of(&mut c.bus.plat)
            .kernel
            .threads
            .get_mut(&1)
            .expect("main thread")
            .heap = 0x00a0_0000;
    }

    /// Heap history for malloc-family tests: allocate a few blocks of
    /// seed-driven sizes with generated `malloc`, free all but one with
    /// generated `free`, and return the kept pointer (0 if its allocation
    /// failed). Both sides build the identical heap, so the tested call
    /// then exercises consolidation with free neighbors, binned chunks,
    /// the top chunk and large chunks. Sizes stay small enough that no
    /// allocation can fail on the heap.
    pub(crate) fn heap_history(c: &mut Cpu, seed: u32, salt: u64) -> u32 {
        const SIZES: [u32; 8] = [8, 16, 24, 64, 256, 1024, 4096, 65536];
        let mut r = Rng(salt ^ seed as u64);
        heap_init(c, &mut r);
        let mut ptrs = [0u32; 6];
        let k = 2 + (seed % 4) as usize;
        for i in 0..k {
            let size = SIZES[((seed >> 2) as usize + i * 3) % SIZES.len()];
            c.g[A0][0] = size as u64;
            ee::s15::f_004188c0__generated(c);
            ptrs[i] = c.g[V0][0] as u32;
        }
        let keep = seed as usize % k;
        // Free order varies by seed so neighbors free in both orders.
        let mut order: [usize; 6] = [0, 1, 2, 3, 4, 5];
        if seed & 1 == 0 {
            order[..k].reverse();
        }
        for &i in &order[..k] {
            if i == keep || ptrs[i] == 0 {
                continue;
            }
            c.g[A0][0] = ptrs[i] as u64;
            ee::s15::f_00418910__generated(c);
        }
        ptrs[keep]
    }

    /// `schedule` replaces thread 1 (resetting its `EndOfHeap` ceiling to
    /// zero), so dtoa/malloc events setups must restore the ceiling after
    /// scheduling or every allocation fails and `_dtoa_r` spins.
    pub(crate) fn heap_schedule(c: &mut Cpu, r: &mut Rng, seed: u32, entry: u32) -> Events {
        let mode = schedule(c, r, seed, entry);
        ssx3_ps2::platform::machine_of(&mut c.bus.plat)
            .kernel
            .threads
            .get_mut(&1)
            .expect("main thread")
            .heap = 0x00a0_0000;
        mode
    }

    /// Loads the game image and wires the global reent to scratch.
    pub(crate) fn wire_reent(c: &mut Cpu, r: &mut Rng) {
        elf_image(c);
        scramble(c, r);
        for i in 0..16 {
            c.bus.write32(REENT_3 + i * 4, 0);
        }
        c.bus.write32(0x0045_4944, REENT_3);
    }

    /// Shared `printf`-family fixture: format at `FMT`, `%s` source at
    /// `SRC`, output `FILE_2`, the three conversion slots in `ARGS`.
    /// Returns the slots (for register varargs) and the rng.
    pub(crate) fn printf_case(c: &mut Cpu, seed: u32, salt: u64) -> ([u32; 3], Rng) {
        let r = file_fixture(c, seed, salt, 1);
        let (fmt, slots): (&[u8], [u32; 3]) = match seed % 6 {
            0 => (&b"a%db"[..], [1, 0, 0]),
            1 => (&b"%s|%x|%c"[..], [SRC, 0x1234, 0x41]),
            2 => (&b"%u %o %i"[..], [0xffff_ffff, 0o17, (-7i32) as u32]),
            3 => (&b"%%n%%"[..], [0, 0, 0]),
            4 => (&b"%ld %hd"[..], [0x1234_5678, 0x9abc_def0, 0]),
            _ => (&b"%p %d"[..], [0x40, 7, 0]),
        };
        for (i, b) in fmt.iter().enumerate() {
            c.bus.write8(FMT + i as u32, *b);
        }
        c.bus.write8(FMT + fmt.len() as u32, 0);
        // `%s` source.
        c.bus.write8(SRC, b'h');
        c.bus.write8(SRC + 1, b'i');
        c.bus.write8(SRC + 2, 0);
        c.g[A1][0] = slots[0] as u64;
        c.g[A2][0] = slots[1] as u64;
        c.g[A3][0] = slots[2] as u64;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        for (i, v) in slots.iter().enumerate() {
            c.bus.write32(ARGS + (i * 4) as u32, *v);
        }
        (slots, r)
    }

    /// Writes `bytes` + NUL at `at` (no fill).
    pub(crate) fn place_str(c: &mut Cpu, at: u32, bytes: &[u8]) {
        for (i, b) in bytes.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + bytes.len() as u32, 0);
    }

    /// Short / long / empty / unaligned string pairs by seed digit.
    /// Returns the machine RNG and the string extents (off+len) so callers
    /// can keep their start pointers at or before the terminator.
    pub(crate) fn str_pair(c: &mut Cpu, seed: u32, salt: u64) -> (Rng, u32, u32) {
        let mut r = Rng(salt ^ seed as u64);
        scramble(c, &mut r);
        for j in 0..0x400 {
            c.bus.write8(DST + j as u32, 0x44);
            c.bus.write8(SRC + j as u32, 0x61);
        }
        let (dst_off, src_off, dst_len, src_len) = match (seed / 9) % 7 {
            0 => (0, 0, 0, 5),   // empty dst
            1 => (1, 3, 3, 7),   // both unaligned
            2 => (0, 0, 40, 40), // long, hits the word loops
            3 => (8, 8, 20, 2),  // aligned, short src
            4 => (0, 5, 9, 33),  // src crosses the 32-byte line
            5 => (7, 0, 15, 15),
            _ => (0, 0, 16, 5), // NUL quad: zero-detect sees NULs everywhere
        };
        // Byte content varies by seed independently of shape: lowercase
        // letters (the old coverage), full-range bytes (high bytes stress
        // the zero-detect masks), and NUL-containing strings (early
        // terminators inside long buffers).
        let content = (seed / 54) % 3;
        let mut dst = vec![0u8; dst_len];
        // Class 6 stays zeros: a full NUL quad for the zero-detect merge.
        if (seed / 9) % 7 != 6 {
            for b in dst.iter_mut() {
                *b = match content {
                    0 => 0x44,
                    1 => 1 + (r.next_u32() % 255) as u8,
                    _ => (r.next_u32() % 256) as u8,
                };
            }
        }
        let mut src = vec![0u8; src_len];
        for b in src.iter_mut() {
            *b = match content {
                0 => 0x61 + (r.next_u32() % 26) as u8,
                1 => 1 + (r.next_u32() % 255) as u8,
                _ => (r.next_u32() % 256) as u8,
            };
        }
        place_str(c, DST + dst_off as u32, &dst[..dst_len.min(64)]);
        place_str(c, SRC + src_off as u32, &src);
        (
            r,
            dst_off + dst_len.min(64) as u32,
            src_off + src_len as u32,
        )
    }

    /// `s`/`reject` (`strcspn`) or `s`/`accept` (`strspn`) pairs by seed.
    pub(crate) fn span_pair(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = Rng(salt ^ seed as u64);
        scramble(c, &mut r);
        let s: &[u8] = match seed % 6 {
            0 => b"",
            1 => b"abc",
            2 => b"aaaaab",
            3 => b"hello world",
            4 => &[b'a'; 64],
            _ => b"ab",
        };
        let rj: &[u8] = match (seed / 6) % 6 {
            0 => b"",
            1 => b"b",
            2 => b"xyz",
            3 => b"aeiou",
            4 => b"ab",
            _ => &[b'y'; 96],
        };
        place_str(c, SRC, s);
        place_str(c, DST, rj);
        c.g[A0][0] = SRC as u64;
        c.g[A1][0] = DST as u64;
        if seed % 6 == 4 {
            // Long `s` with a late hit for one seed in six.
            c.bus.write8(SRC + 63, b'y');
        }
    }

    /// A double's bits, biased toward short decimals and edge magnitudes.
    /// Random cases are masked to finite values: NaN/Inf drive the shared
    /// `_dtoa_r` Bignum path, which needs boot-initialized reent state and
    /// spins on a scratch reent (both sides loop identically, so those are
    /// out of differential scope).
    pub(crate) fn dbl(seed: u32, r: &mut Rng) -> u64 {
        let fixed: [f64; 10] = [
            0.0,
            1.5,
            -1.5,
            123.456,
            -123.456,
            std::f64::consts::PI,
            10.0,
            0.1,
            1e10,
            -2.5e-3,
        ];
        if seed % 4 == 3 {
            // Random sign + mantissa, exponent forced normal ±(1.x).
            ((((r.next_u32() as u64) << 32) | r.next_u32() as u64) & 0x800f_ffff_ffff_ffff)
                | 0x3ff0_0000_0000_0000
        } else {
            fixed[seed as usize % fixed.len()].to_bits()
        }
    }

    /// A finite test double as bits (no NaN/Inf: those spin `_dtoa_r`).
    pub(crate) fn dbl_2(seed: u32, r: &mut Rng) -> u64 {
        const FIXED: [f64; 8] = [0.0, 1.5, -2.25, 123.456, -0.5, 3.14, 100.0, 0.001];
        if seed % 4 == 3 {
            // Random finite magnitude, `ndigits`-friendly.
            let neg = r.next_u32() & 1 == 0;
            let v = (r.next_u32() % 100000) as f64 / 1000.0 + 0.001;
            (if neg { -v } else { v }).to_bits()
        } else {
            FIXED[seed as usize % FIXED.len()].to_bits()
        }
    }

    /// A float's bits, biased toward short decimals. Random cases are
    /// masked to normal ±(1.x-2.x) values (see `dbl`).
    pub(crate) fn flt(seed: u32, r: &mut Rng) -> u32 {
        let fixed: [f32; 8] = [0.0, 1.5, -2.25, 123.456, -0.5, 3.14, 100.0, 0.001];
        if seed % 4 == 3 {
            (r.next_u32() & 0x807f_ffff) | 0x3f80_0000
        } else {
            fixed[seed as usize % fixed.len()].to_bits()
        }
    }

    /// `struct _reent` handed to the stdio functions.
    pub(crate) const REENT_3: u32 = DATA + 0x800;

    /// One glue link: count + one `FILE`.
    pub(crate) const GLUE: u32 = DATA + 0x900;

    /// Glue chain head for `FILE` tests.
    pub(crate) const GLUE_2: u32 = DATA + 0x1400;

    /// Scratch `dst` / `src` buffers for the string tests.
    pub(crate) const DST: u32 = DATA;

    pub(crate) const SRC: u32 = DATA + 0x400;

    /// Scratch for `decpt`/`sign`/`endptr` outputs.
    pub(crate) const SCR: u32 = DATA + 0x1000;

    /// Sorted `u32` array for `bsearch`, at `SRC`.
    pub(crate) const NITEMS: u32 = 12;

    /// The scratch `FILE`.
    pub(crate) const FILE_4: u32 = DATA + 0x1500;

    /// Output buffer for the `printf` family.
    pub(crate) const OUT_3: u32 = DATA + 0x1800;

    /// The scratch `FILE`.
    pub(crate) const FILE_3: u32 = DATA + 0x1800;

    /// Format strings for the `printf` family.
    pub(crate) const FMT_2: u32 = DATA + 0x1200;

    /// `va_list` slots for the `v*` wrappers.
    pub(crate) const ARGS_2: u32 = DATA + 0x1c00;
}
