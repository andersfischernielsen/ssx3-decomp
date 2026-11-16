//! Staging module oc389 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc389`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x003d_c450, ustr_vsprintf, "UStr_vsprintf")];

// ---------------------------------------------------------------------------
// UStr_vsprintf (0x3dc450)
// ---------------------------------------------------------------------------

/// `UStr_vsprintf`'s stack frame size.
pub(crate) const USTR_VSPRINTF_FRAME: u32 = 0x110;
/// Frame slot holding the output pointer (`char16* dst`), advanced per char.
pub(crate) const USTR_VSPRINTF_DST: u32 = 0x60;
/// Frame offset of the upper-case hex digit string (`fp`); the lower-case
/// one sits at `sp + 0`.
pub(crate) const USTR_VSPRINTF_UPPER_DIGITS: u32 = 0x30;
/// ASCII "0123456789abcdef" / "0123456789ABCDEF" (.rodata).
pub(crate) const USTR_VSPRINTF_LOWER_ASCII: u32 = 0x0049_5bc0;
pub(crate) const USTR_VSPRINTF_UPPER_ASCII: u32 = 0x0049_5bd8;
/// Flag-character jump table (' '..'0', 17 entries, .rodata).
pub(crate) const USTR_VSPRINTF_FLAG_TABLE: u32 = 0x0049_5bf0;
/// Conversion jump table ('C'..'x', 0x36 entries, .rodata).
pub(crate) const USTR_VSPRINTF_CONV_TABLE: u32 = 0x0049_5c40;
/// `__floatsidf` (integer arguments are passed on as doubles).
pub(crate) const USTR_VSPRINTF_FLOATSIDF: u32 = 0x0041_2550;
const USTR_VSPRINTF_START: u32 = 0x003d_c450;
const USTR_VSPRINTF_END: u32 = 0x003d_c9f8;
/// Main loop head (`s1` = current char, `s2` = the char after it).
const USTR_VSPRINTF_LOOP: u32 = 0x003d_c4b8;

const USTR_VSPRINTF_FROM_ASCII: Callee = Callee {
    addr: 0x003d_be90,
    f: ee::s14::UStr_FromAscii,
};
const USTR_VSPRINTF_FORMAT_FIELD: Callee = Callee {
    addr: 0x003d_c148,
    f: ee::s14::UStr_FormatNumberField,
};

/// Where a conversion continues in the main loop's tail.
enum UStrVsprintfTail {
    /// 0x3dc9a4: `dst += 2` (v0 holds the old `dst`), then as `Next`.
    Advance,
    /// 0x3dc9ac: `s1 = *s2`, then as `Loop`.
    Next,
    /// 0x3dc9b0: `s2 = a1`, back to the loop head.
    Loop,
}

/// The output-pointer slot `sp + 0x60`.
#[inline]
fn ustr_vsprintf_dst_slot(c: &Cpu) -> u32 {
    sp(c).wrapping_add(USTR_VSPRINTF_DST)
}

/// `s1 = *s2++` (the next format character).
#[inline]
fn ustr_vsprintf_next_char(c: &mut Cpu) {
    c.g[S1][0] = c.lhu(r32(c, S2));
    c.g[S2][0] = sx(r32(c, S2).wrapping_add(2));
}

/// `s3 += 8` (the next va_list slot); the slot's address.
#[inline]
fn ustr_vsprintf_next_arg(c: &mut Cpu) -> u32 {
    c.g[S3][0] = sx(r32(c, S3).wrapping_add(8));
    r32(c, S3).wrapping_sub(8)
}

/// The decimal field loops (width at 0x3dc5e8 into `s6`, precision at
/// 0x3dc670 into `s7`): `n = n * 10 + (ch - '0')` while digits follow.
fn ustr_vsprintf_digits(c: &mut Cpu, n: usize, head: u32) {
    loop {
        c.cycles += 8;
        c.mult(0, V0, n, V1);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x30));
        c.g[n][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
        c.g[S1][0] = c.lhu(r32(c, S2));
        c.g[V0][0] = sx(r32(c, S1).wrapping_sub(0x30));
        c.g[V0][0] = (c.g[V0][0] < 10) as u64;
        c.g[S2][0] = sx(r32(c, S2).wrapping_add(2));
        if c.g[V0][0] == 0 {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The integer conversions: `__floatsidf(va_arg(int))` (with `s4 |= set`
/// first), then the field arguments for `UStr_FormatNumberField`. `xor` is
/// the conversion letter whose digits are lower case (`t2 = sp`); any other
/// letter picks the upper-case digits (`t2 = fp`). `width` overrides `t0`.
fn ustr_vsprintf_int(
    c: &mut Cpu,
    cycles: u64,
    set: u64,
    ret: u32,
    base: u64,
    xor: Option<u64>,
    width: Option<u64>,
) -> crate::call::Flow {
    c.cycles += cycles;
    let slot = ustr_vsprintf_next_arg(c);
    c.g[S0][0] = sx(ustr_vsprintf_dst_slot(c));
    c.g[A0][0] = c.lw(slot);
    c.g[S4][0] |= set;
    call_ptr(c, USTR_VSPRINTF_FLOATSIDF, ret)?;
    if let Some(x) = xor {
        c.g[V1][0] = c.g[S1][0] ^ x;
    }
    c.g[T2][0] = c.g[SP][0];
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A3][0] = c.g[S4][0];
    c.g[T0][0] = match width {
        Some(w) => w,
        None => c.g[S6][0],
    };
    c.g[T1][0] = c.g[S7][0];
    if xor.is_some() && c.g[V1][0] != 0 {
        c.g[T2][0] = c.g[FP][0];
    }
    c.g[A2][0] = base;
    Ok(())
}

/// The float conversions (`%e`, `%g`, `%f`): the double straight from the
/// va_list, `flag` added to the flags, base 10.
fn ustr_vsprintf_float(c: &mut Cpu, flag: u64) {
    c.cycles += 9;
    let slot = ustr_vsprintf_next_arg(c);
    c.g[A3][0] = c.g[S4][0] | flag;
    c.g[A1][0] = c.ld(slot);
    c.g[T0][0] = c.g[S6][0];
    c.g[T1][0] = c.g[S7][0];
    c.g[A0][0] = sx(ustr_vsprintf_dst_slot(c));
    c.g[A2][0] = 10;
    c.g[T2][0] = c.g[SP][0];
}

/// `%s` (`wide`: UTF-16, `lhu`) and `%S` (bytes, sign-extended `lb`):
/// copy the string to `dst`; a null pointer prints "(null)".
fn ustr_vsprintf_string(c: &mut Cpu, wide: bool) -> UStrVsprintfTail {
    let (step, head) = if wide {
        (2, 0x003d_c8b8)
    } else {
        (1, 0x003d_c900)
    };
    c.cycles += 4;
    let slot = ustr_vsprintf_next_arg(c);
    c.g[V1][0] = c.lw(slot);
    c.g[A1][0] = sx(r32(c, S2).wrapping_add(2));
    if c.g[V1][0] == 0 {
        ustr_vsprintf_null(c);
        return UStrVsprintfTail::Next;
    }
    let load = |c: &mut Cpu| {
        let a = r32(c, V1);
        c.g[A0][0] = if wide { c.lhu(a) } else { c.lb(a) };
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(step));
    };
    c.cycles += 3;
    load(c);
    if c.g[A0][0] == 0 {
        return UStrVsprintfTail::Next;
    }
    c.cycles += 1;
    loop {
        c.cycles += 8;
        let dst_slot = ustr_vsprintf_dst_slot(c);
        c.g[V0][0] = c.lw(dst_slot);
        c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
        let ch = c.g[A0][0];
        c.sh(r32(c, V0), ch);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(2));
        let v0 = c.g[V0][0];
        c.sw(dst_slot, v0);
        load(c);
        if c.g[A0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
    c.cycles += 2;
    c.g[S1][0] = c.lhu(r32(c, S2));
    UStrVsprintfTail::Loop
}

/// "(null)" for a null `%s`/`%S` pointer (0x3dc928).
fn ustr_vsprintf_null(c: &mut Cpu) {
    c.cycles += 27;
    let slot = ustr_vsprintf_dst_slot(c);
    c.g[V0][0] = c.lw(slot);
    let d = r32(c, V0);
    c.sh(d, 0x28);
    let p = d.wrapping_add(2);
    c.g[V0][0] = sx(p);
    c.sw(slot, sx(p));
    c.sh(p, 0x6e);
    c.sw(slot, sx(p.wrapping_add(2)));
    c.sh(p.wrapping_add(2), 0x75);
    c.sw(slot, sx(p.wrapping_add(4)));
    c.sh(p.wrapping_add(4), 0x6c);
    c.g[S5][0] = sx(r32(c, S5).wrapping_add(6));
    c.sw(slot, sx(p.wrapping_add(6)));
    c.sh(p.wrapping_add(6), 0x6c);
    c.sw(slot, sx(p.wrapping_add(8)));
    c.sh(p.wrapping_add(8), 0x29);
    c.sw(slot, sx(p.wrapping_add(10)));
    c.g[V1][0] = sx(p.wrapping_add(8));
    c.g[A0][0] = sx(p.wrapping_add(4));
    c.g[A2][0] = sx(p.wrapping_add(10));
    c.g[A3][0] = 0x6c;
    c.g[T0][0] = 0x29;
    c.g[A1][0] = sx(r32(c, S2).wrapping_add(2));
}

/// `void UStr_vsprintf(char16* dst, const char16* fmt, va_list args)`
/// (0x3dc450): printf into a UTF-16 string. Flags `-+ #0`, width and
/// precision (digits or `*`), conversions `c C d i o u x X p P e E f g G n
/// s S`; anything else is copied as is (`%%` prints `%`). Integer arguments
/// go through `__floatsidf` into `UStr_FormatNumberField`. Every va_list
/// slot is 8 bytes. Leaves the terminating 0 written and `dst` advanced
/// past it in the frame (`v0`).
pub fn ustr_vsprintf(c: &mut Cpu) {
    guest(c, ustr_vsprintf_body);
}

fn ustr_vsprintf_body(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 26;
    let frame = Frame::open(c, USTR_VSPRINTF_FRAME);
    frame.save(c, S2, 0xe0);
    c.g[S2][0] = c.g[A1][0];
    frame.save(c, S3, 0xd0);
    let dst = c.g[A0][0];
    let slot = ustr_vsprintf_dst_slot(c);
    c.sw(slot, dst);
    frame.save(c, S1, 0xf0);
    frame.save(c, S5, 0xb0);
    frame.save(c, FP, 0x80);
    c.g[S3][0] = c.g[A2][0];
    frame.save(c, S0, 0x100);
    frame.save(c, S4, 0xc0);
    frame.save(c, S6, 0xa0);
    frame.save(c, S7, 0x90);
    frame.save_ra(c, 0x70);
    // The hex digit strings: lower case at sp, upper case at fp = sp + 0x30.
    c.g[A1][0] = sx(USTR_VSPRINTF_LOWER_ASCII);
    c.g[A0][0] = c.g[SP][0];
    c.g[S5][0] = 0;
    USTR_VSPRINTF_FROM_ASCII.call(c, 0x003d_c49c)?;
    c.g[A0][0] = sx(sp(c).wrapping_add(USTR_VSPRINTF_UPPER_DIGITS));
    c.g[FP][0] = c.g[A0][0];
    c.g[A1][0] = sx(USTR_VSPRINTF_UPPER_ASCII);
    USTR_VSPRINTF_FROM_ASCII.call(c, 0x003d_c4b0)?;
    ustr_vsprintf_next_char(c);

    'main: loop {
        // 0x3dc4b8
        c.cycles += 2;
        c.g[V0][0] = 0x25;
        if c.g[S1][0] == 0 {
            break 'main;
        }
        c.cycles += 2;
        c.g[V1][0] = c.lw(ustr_vsprintf_dst_slot(c));
        if c.g[S1][0] != 0x25 {
            // Copy literal characters up to the next '%'.
            c.cycles += 2;
            c.g[A0][0] = 0x25;
            loop {
                c.cycles += 8;
                c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
                let ch = c.g[S1][0];
                c.sh(r32(c, V1), ch);
                let dst_slot = ustr_vsprintf_dst_slot(c);
                c.g[V0][0] = c.lw(dst_slot);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(2));
                let v0 = c.g[V0][0];
                c.sw(dst_slot, v0);
                ustr_vsprintf_next_char(c);
                if c.g[S1][0] == 0 {
                    break 'main;
                }
                c.cycles += 2;
                c.g[V1][0] = c.lw(ustr_vsprintf_dst_slot(c));
                if c.g[S1][0] == c.g[A0][0] {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_c4d0);
                }
            }
        }
        // 0x3dc4f8: at a '%'.
        c.cycles += 2;
        c.g[A1][0] = 1;
        if c.g[S1][0] == 0 {
            break 'main;
        }
        c.cycles += 6;
        ustr_vsprintf_next_char(c);
        c.g[S4][0] = 0;
        c.g[S6][0] = 0;
        c.g[S7][0] = 6;
        c.g[V1][0] = sx(r32(c, S1).wrapping_sub(0x20));
        // Flags (0x3dc518), through the flag jump table.
        loop {
            c.cycles += 3;
            let in_table = c.g[V1][0] < 0x11;
            c.g[V0][0] = 0x0049_0000;
            if !in_table {
                c.cycles += 1;
                c.g[A1][0] = 0;
            } else {
                c.cycles += 6;
                c.g[V1][0] = sx(r32(c, V1) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(USTR_VSPRINTF_FLAG_TABLE - 0x0049_0000));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = c.lw(r32(c, V1));
                let flag = match r32(c, A0) {
                    0x003d_c53c => Some(0x10), // '-'
                    0x003d_c54c => Some(0x1),  // '+'
                    0x003d_c55c => Some(0x8),  // '0'
                    0x003d_c56c => Some(0x4),  // ' '
                    0x003d_c57c => Some(0x2),  // '#'
                    0x003d_c58c => None,
                    pc => {
                        c.jump_out(pc, USTR_VSPRINTF_START, USTR_VSPRINTF_END);
                        return Err(crate::call::Unwound);
                    }
                };
                match flag {
                    Some(f) => {
                        c.cycles += 4;
                        ustr_vsprintf_next_char(c);
                        c.g[S4][0] |= f;
                    }
                    None => {
                        c.cycles += 1;
                        c.g[A1][0] = 0;
                    }
                }
            }
            c.cycles += 2;
            c.g[V1][0] = sx(r32(c, S1).wrapping_sub(0x20));
            if c.g[A1][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_c518);
            }
        }
        // Width (0x3dc598) into s6.
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S1][0] as i64) < 42) as u64;
        'width: {
            if c.g[S1][0] == 0x2a {
                c.cycles += 5;
                let slot = ustr_vsprintf_next_arg(c);
                c.g[S1][0] = c.lhu(r32(c, S2));
                c.g[S6][0] = c.lw(slot);
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(2));
            } else {
                c.cycles += 2;
                let below = c.g[V0][0] != 0;
                c.g[V0][0] = 0x2e;
                if below {
                    break 'width;
                }
                c.cycles += 3;
                let above = c.g[S1][0] as i64 >= 58;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 49) as u64;
                if !above {
                    c.cycles += 2;
                    let below = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x2e;
                    if below {
                        break 'width;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx(r32(c, S1).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 10) as u64;
                    c.g[S6][0] = 0;
                    if c.g[V0][0] != 0 {
                        c.cycles += 1;
                        c.g[V1][0] = 10;
                        ustr_vsprintf_digits(c, S6, 0x003d_c5e8);
                    }
                }
            }
            // 0x3dc608
            c.cycles += 1;
            c.g[V0][0] = 0x2e;
        }
        // Precision (0x3dc60c) into s7, then the conversion index in v1.
        c.cycles += 2;
        'to698: {
            'to694: {
                if c.g[S1][0] != c.g[V0][0] {
                    c.g[V1][0] = sx(r32(c, S1).wrapping_sub(0x43));
                    break 'to694;
                }
                c.cycles += 4;
                ustr_vsprintf_next_char(c);
                c.g[V0][0] = 0x2a;
                if c.g[S1][0] == 0x2a {
                    c.cycles += 5;
                    let slot = ustr_vsprintf_next_arg(c);
                    c.g[S1][0] = c.lhu(r32(c, S2));
                    c.g[S7][0] = c.lw(slot);
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(2));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 42) as u64;
                    c.g[V1][0] = sx(r32(c, S1).wrapping_sub(0x43));
                    if c.g[V0][0] != 0 {
                        break 'to694;
                    }
                    c.cycles += 3;
                    let above = c.g[S1][0] as i64 >= 58;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 49) as u64;
                    if above {
                        break 'to694;
                    }
                    c.cycles += 2;
                    let below = c.g[V0][0] != 0;
                    c.g[V0][0] = (c.g[V1][0] < 0x36) as u64;
                    if below {
                        break 'to698;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx(r32(c, S1).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 10) as u64;
                    c.g[S7][0] = 0;
                    if c.g[V0][0] == 0 {
                        break 'to694;
                    }
                    c.cycles += 2;
                    c.g[V1][0] = 10;
                    ustr_vsprintf_digits(c, S7, 0x003d_c670);
                }
                // 0x3dc690
                c.cycles += 1;
                c.g[V1][0] = sx(r32(c, S1).wrapping_sub(0x43));
            }
            // 0x3dc694
            c.cycles += 1;
            c.g[V0][0] = (c.g[V1][0] < 0x36) as u64;
        }
        // 0x3dc698: the conversion.
        c.cycles += 2;
        let known = c.g[V0][0] != 0;
        c.g[V0][0] = 0x0049_0000;
        let tail = if !known {
            ustr_vsprintf_literal(c)
        } else {
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(USTR_VSPRINTF_CONV_TABLE - 0x0049_0000));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = c.lw(r32(c, V1));
            match r32(c, A0) {
                // %c (UTF-16) / %C (signed byte)
                pc @ (0x003d_c6b8 | 0x003d_c6d8) => {
                    c.cycles += 8;
                    let slot = ustr_vsprintf_next_arg(c);
                    let dst_slot = ustr_vsprintf_dst_slot(c);
                    c.g[A0][0] = c.lw(dst_slot);
                    c.g[V1][0] = if pc == 0x003d_c6b8 {
                        c.lhu(slot)
                    } else {
                        c.lb(slot)
                    };
                    c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
                    c.g[A1][0] = sx(r32(c, S2).wrapping_add(2));
                    let ch = c.g[V1][0];
                    c.sh(r32(c, A0), ch);
                    c.g[V0][0] = c.lw(dst_slot);
                    UStrVsprintfTail::Advance
                }
                // %d %i
                0x003d_c6f8 => {
                    ustr_vsprintf_int(c, 12, 0, 0x003d_c708, 10, None, None)?;
                    ustr_vsprintf_field(c)?
                }
                // %o
                0x003d_c728 => {
                    ustr_vsprintf_int(c, 13, 0x20, 0x003d_c73c, 8, None, None)?;
                    ustr_vsprintf_field(c)?
                }
                // %u
                0x003d_c75c => {
                    ustr_vsprintf_int(c, 13, 0x20, 0x003d_c770, 10, None, None)?;
                    ustr_vsprintf_field(c)?
                }
                // %x %X
                0x003d_c790 => {
                    ustr_vsprintf_int(c, 15, 0x20, 0x003d_c7a4, 16, Some(0x78), None)?;
                    ustr_vsprintf_field(c)?
                }
                // %p %P: 8 hex digits
                0x003d_c84c => {
                    ustr_vsprintf_int(c, 14, 0x28, 0x003d_c860, 16, Some(0x70), Some(8))?;
                    ustr_vsprintf_field(c)?
                }
                // %e %E
                0x003d_c7cc => {
                    ustr_vsprintf_float(c, 0x80);
                    ustr_vsprintf_field(c)?
                }
                // %g %G
                0x003d_c7f0 => {
                    ustr_vsprintf_float(c, 0x100);
                    ustr_vsprintf_field(c)?
                }
                // %f
                0x003d_c814 => {
                    ustr_vsprintf_float(c, 0x40);
                    ustr_vsprintf_field(c)?
                }
                // %n: store the count so far
                0x003d_c838 => {
                    c.cycles += 5;
                    let slot = ustr_vsprintf_next_arg(c);
                    c.g[A1][0] = sx(r32(c, S2).wrapping_add(2));
                    c.g[V0][0] = c.lw(slot);
                    let count = c.g[S5][0];
                    c.sw(r32(c, V0), count);
                    UStrVsprintfTail::Next
                }
                0x003d_c898 => ustr_vsprintf_string(c, true),
                0x003d_c8e0 => ustr_vsprintf_string(c, false),
                0x003d_c994 => ustr_vsprintf_literal(c),
                pc => {
                    c.jump_out(pc, USTR_VSPRINTF_START, USTR_VSPRINTF_END);
                    return Err(crate::call::Unwound);
                }
            }
        };
        if let UStrVsprintfTail::Advance = tail {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(2));
            let v0 = c.g[V0][0];
            c.sw(ustr_vsprintf_dst_slot(c), v0);
        }
        if !matches!(tail, UStrVsprintfTail::Loop) {
            c.cycles += 1;
            c.g[S1][0] = c.lhu(r32(c, S2));
        }
        // 0x3dc9b0
        c.cycles += 2;
        c.g[S2][0] = c.g[A1][0];
        if event_due(c) {
            deliver_event_here(c, USTR_VSPRINTF_LOOP);
        }
    }

    // 0x3dc9b8: terminate the string.
    c.cycles += 16;
    let dst_slot = ustr_vsprintf_dst_slot(c);
    c.g[V0][0] = c.lw(dst_slot);
    frame.restore(c, S0, 0x100);
    c.sh(r32(c, V0), 0);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(2));
    frame.restore(c, S1, 0xf0);
    frame.restore(c, S2, 0xe0);
    frame.restore(c, S3, 0xd0);
    frame.restore(c, S4, 0xc0);
    frame.restore(c, S5, 0xb0);
    frame.restore(c, S6, 0xa0);
    frame.restore(c, S7, 0x90);
    frame.restore(c, FP, 0x80);
    frame.restore_ra(c, 0x70);
    let v0 = c.g[V0][0];
    c.sw(dst_slot, v0);
    frame.close(c);
    Ok(())
}

/// An unknown conversion character is printed as is (0x3dc994).
fn ustr_vsprintf_literal(c: &mut Cpu) -> UStrVsprintfTail {
    c.cycles += 4;
    c.g[V0][0] = c.lw(ustr_vsprintf_dst_slot(c));
    c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
    c.g[A1][0] = sx(r32(c, S2).wrapping_add(2));
    let ch = c.g[S1][0];
    c.sh(r32(c, V0), ch);
    UStrVsprintfTail::Advance
}

/// `count += UStr_FormatNumberField(...)` (0x3dc884).
fn ustr_vsprintf_field(c: &mut Cpu) -> Result<UStrVsprintfTail, crate::call::Unwound> {
    c.cycles += 5;
    USTR_VSPRINTF_FORMAT_FIELD.call(c, 0x003d_c88c)?;
    c.g[S5][0] = sx(r32(c, S5).wrapping_add(r32(c, V0)));
    c.g[A1][0] = sx(r32(c, S2).wrapping_add(2));
    Ok(UStrVsprintfTail::Next)
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // UStr_vsprintf
    // -----------------------------------------------------------------------

    use crate::difftest::{check, elf_image, scramble, Rng, DATA, GAME_GP};

    /// UTF-16 format string, va_list (8-byte slots), argument strings,
    /// `%n` targets and the output buffer.
    const USTR_VSPRINTF_T_FMT: u32 = DATA;
    const USTR_VSPRINTF_T_ARGS: u32 = DATA + 0x2000;
    const USTR_VSPRINTF_T_STRS: u32 = DATA + 0x4000;
    const USTR_VSPRINTF_T_NOUT: u32 = DATA + 0x6000;
    const USTR_VSPRINTF_T_OUT: u32 = DATA + 0x8000;

    /// Builds a random format string with matching arguments: literals,
    /// flags, digit and `*` widths and precisions (including `.` alone and
    /// the `.0` quirk that prints the `0`), every conversion of both jump
    /// tables, unknown conversions, `%%`, null `%s`/`%S` and sometimes a
    /// dangling `%` at the end.
    fn ustr_vsprintf_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x3dc4_5000_0000 ^ (seed as u64) << 8);
        scramble(c, &mut r);
        elf_image(c);
        c.g[GP][0] = GAME_GP as u64;
        let mut fmt: Vec<u16> = Vec::new();
        let mut args = USTR_VSPRINTF_T_ARGS;
        let mut strs = USTR_VSPRINTF_T_STRS;
        let mut nout = USTR_VSPRINTF_T_NOUT;
        // One 8-byte va_list slot: the value and junk in the upper word.
        let push = |c: &mut Cpu, args: &mut u32, lo: u32, hi: u32| {
            c.bus.write32(*args, lo);
            c.bus.write32(*args + 4, hi);
            *args += 8;
        };
        let lits: &[u16] = &[
            b'a' as u16,
            b'Z' as u16,
            b' ' as u16,
            b'.' as u16,
            b'*' as u16,
            b'7' as u16,
            b'0' as u16,
            b'x' as u16,
            0x263a,
            0xff21,
        ];
        let convs = b"cCdiouxXpPeEfgGnsS%qzhDAt";
        let tokens = 1 + r.next_u32() % 6;
        for _ in 0..tokens {
            if r.next_u32() % 4 == 0 {
                for _ in 0..1 + r.next_u32() % 4 {
                    fmt.push(lits[(r.next_u32() % lits.len() as u32) as usize]);
                }
                continue;
            }
            fmt.push(b'%' as u16);
            for _ in 0..r.next_u32() % 4 {
                fmt.push(b" #+-0"[(r.next_u32() % 5) as usize] as u16);
            }
            match r.next_u32() % 4 {
                1 => {
                    fmt.push(b'1' as u16 + (r.next_u32() % 9) as u16);
                    if r.next_u32() % 2 == 0 {
                        fmt.push(b'0' as u16 + (r.next_u32() % 3) as u16);
                    }
                }
                2 => {
                    fmt.push(b'*' as u16);
                    let w = r.next_u32() % 25;
                    {
                        let hi = r.next_u32();
                        push(c, &mut args, w, hi);
                    }
                }
                _ => {}
            }
            let mut consumes = true;
            match r.next_u32() % 6 {
                1 => {
                    fmt.push(b'.' as u16);
                    fmt.push(b'1' as u16 + (r.next_u32() % 8) as u16);
                }
                2 => {
                    fmt.push(b'.' as u16);
                    fmt.push(b'*' as u16);
                    let p = r.next_u32() % 9;
                    {
                        let hi = r.next_u32();
                        push(c, &mut args, p, hi);
                    }
                }
                3 => fmt.push(b'.' as u16),
                4 => {
                    // ".0": the '0' is printed and the letter is literal text.
                    fmt.push(b'.' as u16);
                    fmt.push(b'0' as u16);
                    consumes = false;
                }
                _ => {}
            }
            let conv = convs[(r.next_u32() % convs.len() as u32) as usize];
            fmt.push(conv as u16);
            if !consumes {
                continue;
            }
            match conv {
                b'c' | b'C' => {
                    let ch = r.next_u32();
                    {
                        let hi = r.next_u32();
                        push(c, &mut args, ch, hi);
                    }
                }
                b'd' | b'i' | b'o' | b'u' | b'x' | b'X' | b'p' | b'P' => {
                    let v = match r.next_u32() % 3 {
                        0 => r.next_u32() % 100,
                        1 => r.next_u32(),
                        _ => (r.next_u32() % 100_000).wrapping_neg(),
                    };
                    {
                        let hi = r.next_u32();
                        push(c, &mut args, v, hi);
                    }
                }
                b'e' | b'E' | b'f' | b'g' | b'G' => {
                    let v = if r.next_u32() % 6 == 0 {
                        0.0
                    } else {
                        (r.next_u32() % 2_000_000) as f64 / 64.0 - 15_625.0
                    };
                    let bits = v.to_bits();
                    push(c, &mut args, bits as u32, (bits >> 32) as u32);
                }
                b'n' => {
                    let at = nout;
                    nout += 4;
                    {
                        let hi = r.next_u32();
                        push(c, &mut args, at, hi);
                    }
                }
                b's' | b'S' => {
                    if r.next_u32() % 5 == 0 {
                        {
                            let hi = r.next_u32();
                            push(c, &mut args, 0, hi);
                        }
                    } else {
                        let at = strs;
                        let n = r.next_u32() % 7;
                        for i in 0..n {
                            if conv == b's' {
                                let ch = 0x20 + (r.next_u32() % 0x2000) as u16;
                                c.bus.write16(at + 2 * i, ch);
                            } else {
                                let ch = 1 + (r.next_u32() % 0xff) as u8;
                                c.bus.write8(at + i, ch);
                            }
                        }
                        if conv == b's' {
                            c.bus.write16(at + 2 * n, 0);
                        } else {
                            c.bus.write8(at + n, 0);
                        }
                        strs += 0x40;
                        {
                            let hi = r.next_u32();
                            push(c, &mut args, at, hi);
                        }
                    }
                }
                _ => {}
            }
        }
        if seed % 11 == 0 {
            fmt.push(b'%' as u16);
        }
        fmt.push(0);
        for (i, ch) in fmt.iter().enumerate() {
            c.bus.write16(USTR_VSPRINTF_T_FMT + 2 * i as u32, *ch);
        }
        c.g[A0][0] = sx(USTR_VSPRINTF_T_OUT);
        c.g[A1][0] = sx(USTR_VSPRINTF_T_FMT);
        c.g[A2][0] = sx(USTR_VSPRINTF_T_ARGS);
        c.pc = USTR_VSPRINTF_START;
    }

    /// `UStr_vsprintf` (printf into a UTF-16 string) against the generated
    /// body, with the real `UStr_FromAscii`, `__floatsidf` and
    /// `UStr_FormatNumberField` on the game's .rodata (jump tables, digits).
    #[test]
    fn ustr_vsprintf_matches() {
        check(
            ssx3_ee::s14::f_003dc450__generated,
            ustr_vsprintf,
            300,
            &ustr_vsprintf_setup,
        );
    }
}
