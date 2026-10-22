//! `src/lib/libc.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::call::guest;
use ssx3_ee as ee;
use crate::events::back_edge_unwinds;
pub(crate) use crate::game::lib::gcc::frame_sd;
pub(crate) use crate::game::lib::gcc::frame_ld;
pub(crate) use crate::game::lib::gcc::MULDF3;

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
];

// ---------------------------------------------------------------------------
// Callees (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::computer::callee::RUBBER_BAND;
    pub(crate) use crate::game::ai::computer::callee::SET_ANIM_TIME_SCALE;
    pub(crate) use crate::game::ai::computer::callee::GET_SUB_STATE_2 as GET_SUB_STATE;
    pub(crate) use crate::game::ai::computer::callee::THINK_CRUISE;
    pub(crate) use crate::game::ai::computer::callee::THINK_BOARD_PRESS;
    pub(crate) use crate::game::ai::computer::callee::THINK_PREWIND;
    pub(crate) use crate::game::ai::computer::callee::THINK_NATURAL_AIR;
    pub(crate) use crate::game::ai::computer::callee::THINK_SPIN;
    pub(crate) use crate::game::ai::computer::callee::THINK_GATE;
    pub(crate) use crate::game::ai::computer::callee::THINK_RAILSLIDE;
    pub(crate) use crate::game::ai::computer::callee::THINK_WIPEOUT;
    pub(crate) use crate::game::ai::computer::callee::VFPRINTF_R;
    pub(crate) use crate::game::ai::computer::callee::VFIPRINTF_R;
    pub(crate) use crate::game::ai::computer::callee::EA_MEM_SET;


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

pub(crate) const FWALK: Callee = Callee { addr: 0x0041_7418, f: ssx3_ee::s15::_fwalk };

pub(crate) const CLEANUP_R: Callee = Callee { addr: 0x0041_6fc0, f: ssx3_ee::s15::_cleanup_r };


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
pub(crate) const CLOSE: Callee = Callee { addr: 0x0042_4488, f: ssx3_ee::s15::f_00424488 };

pub(crate) const OPEN: Callee = Callee { addr: 0x0042_4460, f: ssx3_ee::s15::f_00424460 };

pub(crate) const READ: Callee = Callee { addr: 0x0042_43e8, f: ssx3_ee::s15::f_004243e8 };

pub(crate) const WRITE: Callee = Callee { addr: 0x0042_4368, f: ssx3_ee::s15::f_00424368 };

pub(crate) const LSEEK: Callee = Callee { addr: 0x0042_4498, f: ssx3_ee::s15::f_00424498 };

pub(crate) const SBRK: Callee = Callee { addr: 0x0042_44a0, f: ssx3_ee::s15::f_004244a0 };

pub(crate) const FSTAT_C: Callee = Callee { addr: 0x0042_4558, f: ssx3_ee::s15::f_00424558 };


/// The shared body: open a 0x30 frame (s0 = reent, s1 = 0x4a0000, ra),
/// shift `shift` arguments off the front, clear the errno, call `target`.
/// `-1` results read the errno back into `v0` and store it through `reent`.
pub(crate) fn reentrant_io(c: &mut Cpu, target: Callee, shift: usize, ret: u32, first: u64) -> call::Flow {
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

pub(crate) const LIB_EXIT: Callee = Callee { addr: 0x0042_c6e8, f: ssx3_ee::s15::f_0042c6e8 };


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
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[S0][0] as u32)));
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
                    c.g[S2][0] = sx((c.g[S7][0] as u32).wrapping_sub((c.g[S4][0] as u32)));
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
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[RA][0] = sx(0x0041cb38);
                    c.g[A2][0] = c.g[S2][0];
                    callee::MEMCPY.call(c, 0x0041_cb38)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[S2][0] as u32)));
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t { pc = 0x0041_cb9c; continue; }
                    pc = 0x0041_cb54; continue;
                }
                0x0041_cb54 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[RA][0] = sx(0x0041cb60);
                    c.g[A1][0] = c.g[S4][0];
                    callee::MEMCPY.call(c, 0x0041_cb60)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
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
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
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
                    c.g[S5][0] = c.lb((c.g[S7][0] as u32));
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
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;
                    pc = t; continue;
                }
                // ' '  a positive value gets a space sign.
                0x0041_cc14 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(sp + SIGN);
                    if c.g[V0][0] != 0u64 { c.g[S5][0] = c.lb((c.g[S7][0] as u32)); if back_edge_unwinds(c, 0x0041_cbe8) { return Ok(()); } pc = 0x0041_cbe8; continue; }
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
                    c.g[V1][0] = sx(0u32.wrapping_sub((c.g[V1][0] as u32)));
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
                    c.g[S5][0] = c.lb((c.g[S7][0] as u32));
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
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                    c.g[S5][0] = c.lb((c.g[S7][0] as u32));
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
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                    c.g[S5][0] = c.lb((c.g[S7][0] as u32));
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
                    c.g[V1][0] = c.lb((c.g[S7][0] as u32));
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
                    c.sd((c.g[V0][0] as u32), c.g[A1][0]);
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
                    c.sd((c.g[V0][0] as u32), c.g[A2][0]);
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
                    c.sh((c.g[V0][0] as u32), c.g[V1][0]);
                    if t { if back_edge_unwinds(c, 0x0041_cac0) { return Ok(()); } pc = 0x0041_cac0; continue; }
                    pc = 0x0041_ce10; continue;
                }
                0x0041_ce10 => {
                    c.cycles += 5;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lw(sp + WRITTEN);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_sub(0x8));
                    let t = true;
                    c.sw((c.g[V0][0] as u32), c.g[A0][0]);
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
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[S4][0] as u32)));
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
                    c.sb((c.g[S4][0] as u32), c.g[V0][0]);
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
                    c.sb((c.g[S4][0] as u32), c.g[V1][0]);
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
                    c.sb((c.g[S4][0] as u32), c.g[V0][0]);
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
                    c.sb((c.g[S4][0] as u32), c.g[V0][0]);
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
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[S0][0] = c.g[S0][0] >> 4;
                    c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
                    let t = c.g[S0][0] != 0u64;
                    c.sb((c.g[S4][0] as u32), c.g[V1][0]);
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
                    c.g[V1][0] = sx(sp.wrapping_sub((c.g[V0][0] as u32)));
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
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
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
                    c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
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
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lw((c.g[S1][0] as u32)); pc = 0x0041_d218; continue; }
                    pc = 0x0041_d1d4; continue;
                }
                0x0041_d1d4 => {
                    c.cycles += 17;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5a60));
                    c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                    c.ldr(3, (c.g[A2][0] as u32));
                    c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                    c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                    c.sdr(3, (c.g[V0][0] as u32));
                    c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
                    c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
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
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
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
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[RA][0] = sx(0x0041d298);
                    c.g[A2][0] = c.g[S0][0];
                    callee::MEMCPY.call(c, 0x0041_d298)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[S0][0] as u32)));
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t { pc = 0x0041_d2fc; continue; }
                    pc = 0x0041_d2b4; continue;
                }
                0x0041_d2b4 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[RA][0] = sx(0x0041d2c0);
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5a60));
                    callee::MEMCPY.call(c, 0x0041_d2c0)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
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
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lw((c.g[S1][0] as u32)); pc = 0x0041_d344; continue; }
                    pc = 0x0041_d328; continue;
                }
                0x0041_d328 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                    c.sb((c.g[V0][0] as u32), c.g[V1][0]);
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
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
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lw((c.g[S1][0] as u32)); pc = 0x0041_d3d8; continue; }
                    pc = 0x0041_d3a8; continue;
                }
                0x0041_d3a8 => {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V1][0] = c.lb(sp + PREFIXBUF);
                    c.g[A0][0] = c.lb(sp + PREFIXBUF + 1);
                    c.sb((c.g[V0][0] as u32), c.g[V1][0]);
                    c.sb((c.g[V0][0] as u32).wrapping_add(0x1), c.g[A0][0]);
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
                    pc = 0x0041_d3cc; continue;
                }
                0x0041_d3cc => {
                    c.cycles += 3;
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
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
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
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
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
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
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lw((c.g[S1][0] as u32)); pc = 0x0041_d4b8; continue; }
                    pc = 0x0041_d474; continue;
                }
                0x0041_d474 => {
                    c.cycles += 17;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5a50));
                    c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(4, (c.g[V1][0] as u32));
                    c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x7));
                    c.sdr(4, (c.g[V0][0] as u32));
                    c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
                    c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
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
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
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
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[RA][0] = sx(0x0041d530);
                    c.g[A2][0] = c.g[S0][0];
                    callee::MEMCPY.call(c, 0x0041_d530)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[S0][0] as u32)));
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t { pc = 0x0041_d594; continue; }
                    pc = 0x0041_d54c; continue;
                }
                0x0041_d54c => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[RA][0] = sx(0x0041d558);
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5a50));
                    callee::MEMCPY.call(c, 0x0041_d558)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
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
                    c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
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
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lw((c.g[S1][0] as u32)); pc = 0x0041_d618; continue; }
                    pc = 0x0041_d5d4; continue;
                }
                0x0041_d5d4 => {
                    c.cycles += 17;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5a50));
                    c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.ldr(5, (c.g[A0][0] as u32));
                    c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x7));
                    c.sdr(5, (c.g[V0][0] as u32));
                    c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0xf));
                    c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
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
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
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
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[RA][0] = sx(0x0041d690);
                    c.g[A2][0] = c.g[S0][0];
                    callee::MEMCPY.call(c, 0x0041_d690)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[S0][0] as u32)));
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t { pc = 0x0041_d6f4; continue; }
                    pc = 0x0041_d6ac; continue;
                }
                0x0041_d6ac => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[RA][0] = sx(0x0041d6b8);
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5a50));
                    callee::MEMCPY.call(c, 0x0041_d6b8)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
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
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                    if t { pc = 0x0041_d73c; continue; }
                    pc = 0x0041_d714; continue;
                }
                0x0041_d714 => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x0041d71c);
                    c.g[A1][0] = c.g[S4][0];
                    callee::MEMCPY.call(c, 0x0041_d71c)?;
                    c.g[A0][0] = c.lw(sp + LEN);
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
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
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
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
                    c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
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
                    if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lw((c.g[S1][0] as u32)); pc = 0x0041_d818; continue; }
                    pc = 0x0041_d7d4; continue;
                }
                0x0041_d7d4 => {
                    c.cycles += 17;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5a60));
                    c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                    c.ldr(3, (c.g[A2][0] as u32));
                    c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                    c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                    c.sdr(3, (c.g[V0][0] as u32));
                    c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
                    c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
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
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
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
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[RA][0] = sx(0x0041d890);
                    c.g[A2][0] = c.g[S0][0];
                    callee::MEMCPY.call(c, 0x0041_d890)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[S0][0] as u32)));
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t { pc = 0x0041_d8f4; continue; }
                    pc = 0x0041_d8ac; continue;
                }
                0x0041_d8ac => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[RA][0] = sx(0x0041d8b8);
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5a60));
                    callee::MEMCPY.call(c, 0x0041_d8b8)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
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
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fill;
    use crate::difftest::elf_image;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::compare;
    use crate::difftest::seed_count;
    pub(crate) use crate::game::lib::gcc::tests::SEEDS;
    pub(crate) use crate::game::lib::gcc::tests::plain;
    pub(crate) use crate::game::ai::computer::tests::SEEDS_3 as SEEDS_2;


    #[test]
    fn date_is_leap_year_matches() {
        check_lohi(ssx3_ee::s08::f_002c71c0, date_is_leap_year, 300, &|c, seed| {
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
        });
    }


    #[test]
    fn time_add_seconds_matches() {
        check_lohi(ssx3_ee::s08::f_002c7328, time_add_seconds, 200, &|c, seed| {
            let mut r = Rng(0x7328 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = (DATA + 0x200) as u64;
            c.g[A3][0] = (DATA + 0x300) as u64;
            c.g[T0][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(DATA + 0x100, r.next_u32());
            c.bus.write32(DATA + 0x200, r.next_u32());
            c.bus.write32(DATA + 0x300, r.next_u32());
        });
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
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { (3 + r.next_u32() % 8) as u64 };
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
            let p = if seed % 5 == 0 { 0 } else { (DATA + 0x500) as u64 };
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
    pub(crate) fn check_strict(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
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
        check_strict(ssx3_ee::s15::f_00418958__generated, mbtowc_r, SEEDS_3, &|c, seed| {
            setup(c, seed)
        });
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
}
