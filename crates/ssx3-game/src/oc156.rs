//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! - **`_vfiprintf_r`** (0x0041ca70): newlib's integer-only `printf` core —
//!   `int _vfiprintf_r(struct _reent *r, FILE *fp, const char *fmt, va_list ap)`.
//!   It is the formatter the game reaches through `vfprintf`/`sprintf` with no
//!   float conversion in the format, and 0.8 % of the race replay's work is
//!   spent here, so it is the largest single function in this wave.
//!
//! The original is one long state machine, so the generated body of this
//! function has an `__entries` twin with 244 entry points (the loop heads
//! where an event may unwind, every call's return address, and every arm of
//! the specifier jump table). This replacement keeps that shape: a `match pc`
//! over the same 244 addresses, so it can be entered at the function start or
//! resumed at any of them, exactly like the generated body.
//!
//! The stages of the machine (addresses are the original's):
//!
//! * `0x41cac0`-**literal run**: the format is advanced one multibyte
//!   character at a time through `_mbtowc_r` (`0x418958`) until the converted
//!   character is `'%'`; the run `cp..fmt` is then written through the
//!   inlined buffered-write fast path (`_p`/`_w` of the `FILE`) or handed to
//!   `vfiprintf_BufferedWrite` (`0x41c938`), and `written` is accumulated.
//! * `0x41cbd0`-**specifier**: flags (`' '`, `'#'`, `'\''`, `'*'`, `'-'`,
//!   `'+'`, `'.'`, `'0'`, digits), then the length modifiers (`'h'`, `'l'`,
//!   `'ll'`, `'q'`), then the conversion character.
//! * `0x41cbec`-**jump table** at `0x49a620` indexed by `c - ' '` (89
//!   entries, `' '`..`'x'`). Every other character (including `'%'` and
//!   `'\0'`) lands on `0x41d134`, which prints it literally or ends the scan.
//! * the per-conversion handlers produce the digits backwards from `sp+0x28`
//!   (`%d`/`%i`/`%u` with `__udivdi3`/`__umoddi3`, `%o`/`%x`/`%X` by shifting,
//!   `%s` with `strlen`/`memchr`, `%c`, `%p`, `%n`), compute the field width,
//!   and write sign, `0x`/`0` prefix, padding (in 16-byte chunks through
//!   `vfiprintf_BufferedWrite`) and digits.
//! * `0x41d960`-**epilogue**: restore `f16`-`f25`, `f30`, `ra`, return
//!   `written`.
//!
//! Frame layout (`sp` after the 0x110-byte frame is pushed):
//!
//! ```text
//! 0x00  staging buffer for one character, and for the digits with the
//!       thousands separators inserted
//! 0x28  digits are built backwards from here
//! 0x30  "0x"/"0X"/"0" prefix staging (2 bytes)
//! 0x40  the character converted from the format (read back as a `u16`),
//!       0x42 the sign byte, 0x44 the mbstate_t handed to _mbtowc_r
//! 0x48  length of the digits/string just prepared
//! 0x4c  characters written so far (the return value)
//! 0x50  field width, 0x54 precision, 0x58 width of the whole field
//! 0x5c  digit table of the conversion (hex), 0x60 `&wc`, 0x64 `&mbstate`
//! 0x68  `flags & LEFT`, saved for the padding writer
//! 0x70  saved ra, 0x80..0x100 saved f30, f16..f25
//! ```
//!
//! The VU0 is not touched. `%e`/`%f`/`%g` never reach this function (the
//! dispatcher at 0x41b658 sends formats containing them to `_vfprintf_r`), so
//! the handlers for them fall through to the "print the character literally"
//! arm, as in the original.

use crate::call::{guest, Callee};
use crate::events::back_edge_unwinds;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0041_ca70, vfiprintf_r, "_vfiprintf_r")];

// ---------------------------------------------------------------------------
// Callees (address, readable alias in the generated shard).

mod callee {
    use super::{ee, Callee};

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
    /// `strlen` for `%s` without a precision.
    pub const STRLEN: Callee = Callee {
        addr: 0x0041_6810,
        f: ee::s15::strlen,
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
// Format flags (the original keeps them in `s2`; the bit numbers are this
// build's, not upstream newlib's).

const ALT: u64 = 0x0001; // '#'  alternate form (0x/0X/leading 0)
const HASHX: u64 = 0x0002; // internal: the base prefix is to be emitted
const LEFT: u64 = 0x0004; // '-'  left justify
const LONGINT: u64 = 0x0010; // 'l'  long
const LONGLONG: u64 = 0x0020; // 'll', 'q'  long long
const SHORTINT: u64 = 0x0040; // 'h'  short
const ZEROPAD: u64 = 0x0080; // '0'  pad with zeroes (dropped when a
                             //      precision is given)
const GROUP: u64 = 0x0200; // '\'' insert thousands separators

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

const NUMBUF: u32 = 0x28; // digits are built backwards from here
const PREFIXBUF: u32 = 0x30; // "0x"/"0X"/"0" prefix (2 bytes)
const WC: u32 = 0x40; // wchar_t converted from the format
const SIGN: u32 = 0x42; // sign character: '-', '+', ' ' or 0
const MBSTATE: u32 = 0x44; // mbstate_t for _mbtowc_r
const LEN: u32 = 0x48; // length of the digits / string to write
const WRITTEN: u32 = 0x4c; // characters written so far
const WIDTH: u32 = 0x50; // field width (0 when absent)
const PREC: u32 = 0x54; // precision (-1 when absent)
const PAD: u32 = 0x58; // width of the whole field (digits + sign/prefix)
const BASE: u32 = 0x5c; // digit table of a hex conversion
const PWC: u32 = 0x60; // &WC
const PMBSTATE: u32 = 0x64; // &MBSTATE
const GROUP_FLAG: u32 = 0x68; // flags & LEFT, saved for the padding writer

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
mod tests {
    use super::*;
    use crate::difftest::{compare, elf_image, scramble, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 200;

    /// The `__sFILE` the formatter writes through (`_p` at +0, `_w` at +8,
    /// `_flags`/`_file` at +0xc/+0xe, `_bf` at +0x10, `_data` at +0x54).
    const FILE: u32 = DATA;
    /// `struct _reent` handed in as the first argument (and at FILE + 0x54).
    const REENT: u32 = DATA + 0x400;
    /// The format string.
    const FMT: u32 = DATA + 0x100;
    /// The `va_list`: one 8-byte slot per argument.
    const ARGS: u32 = DATA + 0x200;
    /// `%s` source strings.
    const STR: u32 = DATA + 0x2000;
    /// `%n` target.
    const NVAL: u32 = DATA + 0x3000;
    /// `fp->_p`: the write buffer the inlined copy path fills.
    const OUT: u32 = DATA + 0x4000;

    /// What kind of argument a conversion takes (the value is made from the
    /// seed in [`printf_setup`]).
    #[derive(Clone, Copy, PartialEq)]
    enum Kind {
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

    struct Case {
        fmt: &'static [u8],
        args: &'static [Kind],
    }

    /// Formats that reach every handler of the specifier table, every flag,
    /// both padding writers and both write paths. `%e`/`%f`/`%g` cannot occur
    /// here (the dispatcher sends them to `_vfprintf_r`).
    #[rustfmt::skip]
    const CASES: &[Case] = &[
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
    fn printf_setup(c: &mut Cpu, seed: u32) {
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
        c.bus.write32(FILE + 0x54, REENT); // _data

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
                            c.bus.write8(STR + j, b'a' + ((seed + j) % 26) as u8);
                        }
                        c.bus.write8(STR + len, 0);
                        (STR, 0)
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

        c.g[A0][0] = REENT as u64;
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
                printf_setup(c, seed)
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
