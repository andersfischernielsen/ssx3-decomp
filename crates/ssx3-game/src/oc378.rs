//! Staging module oc378 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc378`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0032_1af0,
    input_parser_next_token,
    "InputParser_NextToken",
)];

// ---------------------------------------------------------------------------
// InputParser_NextToken (`cInputMapParser_readToken`)

/// The tokenizer's fields (`sTok` in the reference): the source text
/// (`base`, `len`), the read position, the `#line` file name, the line
/// counter, "at the start of a line", the error flag, the token kind, its
/// text and its numeric value.
pub(crate) const NEXT_TOKEN_BASE: u32 = 0x0;
pub(crate) const NEXT_TOKEN_LEN: u32 = 0x4;
pub(crate) const NEXT_TOKEN_POS: u32 = 0x8;
pub(crate) const NEXT_TOKEN_FNAME: u32 = 0xc;
pub(crate) const NEXT_TOKEN_LINE: u32 = 0x8c;
pub(crate) const NEXT_TOKEN_LINE_START: u32 = 0x90;
#[cfg(test)]
pub(crate) const NEXT_TOKEN_ERROR: u32 = 0x94;
pub(crate) const NEXT_TOKEN_KIND: u32 = 0x98;
pub(crate) const NEXT_TOKEN_TEXT: u32 = 0x9c;
pub(crate) const NEXT_TOKEN_VALUE: u32 = 0xdc;

/// `InputParser_Error(parser, fmt, ...)`.
const NEXT_TOKEN_ERROR_FN: Callee = Callee {
    addr: 0x0032_1500,
    f: ee::s10::InputParser_Error,
};
/// `strcasecmp` and `sscanf` (called through the function table).
const NEXT_TOKEN_STRCASECMP: u32 = 0x0041_aa88;
const NEXT_TOKEN_SSCANF: u32 = 0x0041_78b0;

#[inline(always)]
fn nt_r(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// Address of a tokenizer field (`s0` holds the tokenizer).
#[inline(always)]
fn nt_f(c: &Cpu, off: u32) -> u32 {
    nt_r(c, S0).wrapping_add(off)
}

/// `(int8)(x << 24) >> 24` of a register value.
#[inline(always)]
fn nt_sext8(v: u64) -> u64 {
    sx((((v as u32) << 24) as i32 >> 24) as u32)
}

/// `v1 = len; a0 = pos; v0 = (pos < v0 + len)` with `v0` = base already
/// loaded: is there input left?
fn nt_more_v0(c: &mut Cpu) -> bool {
    c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
    c.g[A0][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
    c.g[V0][0] = sx(nt_r(c, V0).wrapping_add(nt_r(c, V1)));
    c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
    c.g[V0][0] != 0
}

/// `nt_more_v0` loading the base first.
fn nt_more(c: &mut Cpu) -> bool {
    c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
    nt_more_v0(c)
}

/// Step past the current character (the 7-cycle inline form).
fn nt_adv7(c: &mut Cpu) {
    c.cycles += 7;
    let more = nt_more(c);
    c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
    if more {
        c.cycles += 1;
        c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
    }
}

/// `s2` = the current character (unsigned byte) or -1 at the end.
fn nt_peek_s2(c: &mut Cpu) {
    c.cycles += 7;
    let more = nt_more(c);
    c.g[S2][0] = u64::MAX;
    if more {
        c.cycles += 1;
        c.g[S2][0] = c.lbu(nt_r(c, A0));
    }
}

/// Append register `src`'s byte to the token text (`s1`) and step past it.
fn nt_push_adv(c: &mut Cpu, src: usize) {
    c.cycles += 8;
    c.sb(nt_r(c, S1), c.g[src][0]);
    let more = nt_more(c);
    c.g[S1][0] = sx(nt_r(c, S1).wrapping_add(1));
    if more {
        c.cycles += 2;
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
        c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
    }
}

/// `(unsigned)(ch - '0') < 10` into `v0`.
#[inline(always)]
fn nt_digit(c: &mut Cpu, ch: u64) -> bool {
    c.g[V0][0] = sx((ch as u32).wrapping_sub(0x30));
    c.g[V0][0] = (c.g[V0][0] < 10) as u64;
    c.g[V0][0] != 0
}

/// Whitespace test, the shape that loads 9 and 13 into `v0` (the first
/// compare, against ' ', and `flag = 0` are the caller's).
fn nt_space_v0(c: &mut Cpu, ch: u64, flag: usize) {
    if ch == 0x20 {
        c.cycles += 1;
        c.g[flag][0] = 1;
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = 9;
    if ch == 9 {
        c.g[flag][0] = 1;
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0xd;
    if ch == 0xa {
        c.cycles += 1;
        c.g[flag][0] = 1;
        return;
    }
    c.cycles += 2;
    if ch == 0xd {
        c.cycles += 1;
        c.g[flag][0] = 1;
    }
}

/// Whitespace test with the constants held in registers.
fn nt_space_t(c: &mut Cpu, ch: u64, flag: usize) {
    if ch == 0x20 {
        c.cycles += 1;
        c.g[flag][0] = 1;
        return;
    }
    c.cycles += 2;
    if ch == 9 {
        c.g[flag][0] = 1;
        return;
    }
    c.cycles += 2;
    if ch == 0xa {
        c.g[flag][0] = 1;
        return;
    }
    c.cycles += 2;
    if ch == 0xd {
        c.cycles += 1;
        c.g[flag][0] = 1;
    }
}

/// `v0` on leaving `nt_blank_run`: base (`late`) or `a3 + a2` (the end).
fn nt_blank_exit(c: &mut Cpu, late: bool) {
    c.g[V0][0] = if late {
        c.lw(nt_f(c, NEXT_TOKEN_BASE))
    } else {
        sx(nt_r(c, A3).wrapping_add(nt_r(c, A2)))
    };
}

/// `while (isWh(pk) && pk != -1 && pk != '\n') adv` with `a1` = end of
/// input; `end_reg` holds the end too. On exit `v0` = end (`late` = false)
/// or base (`late`), `a0` = position.
fn nt_blank_run(c: &mut Cpu, ev_head: u32, end_reg: usize, late: bool) {
    let exit = nt_blank_exit;
    loop {
        c.cycles += 5;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
        c.g[V1][0] = u64::MAX;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 4;
        c.g[V0][0] = nt_sext8(c.g[V1][0]);
        c.g[V1][0] = 0;
        let ch = c.g[V0][0];
        nt_space_t(c, ch, V1);
        c.cycles += 2;
        c.g[V0][0] = (c.g[A0][0] < c.g[end_reg][0]) as u64;
        if c.g[V1][0] == 0 {
            c.cycles += 1;
            exit(c, late);
            return;
        }
        if late {
            c.cycles += 2;
        }
        if event_due(c) {
            deliver_event_here(c, ev_head);
        }
        c.cycles += 2;
        c.g[V1][0] = u64::MAX;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 2;
        if c.g[V1][0] == u64::MAX {
            exit(c, late);
            return;
        }
        c.cycles += 2;
        c.g[V1][0] = u64::MAX;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 2;
        let newline = c.g[V1][0] == 0xa;
        exit(c, late);
        if newline {
            return;
        }
        let more = if late {
            c.cycles += 6;
            nt_more_v0(c)
        } else {
            c.cycles += 7;
            nt_more(c)
        };
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
        if more {
            c.cycles += 1;
            c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
        }
    }
}

/// `InputParser_NextToken` (0x321af0, `cInputMapParser_readToken(parser)`):
/// read the next token of an input-map script. Skips whitespace, `//`
/// comments and `# line "file"` directives (which set the line counter and
/// file name for error messages), then reads an operator, an identifier
/// (keywords `max`..`sqrt` and `minradian` get their own kinds), a number
/// (parsed into `+0xdc` with `sscanf("%f")`) or reports an unexpected
/// character. The kind goes to `+0x98`, the text to `+0x9c`.
pub fn input_parser_next_token(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S3, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S5, 0x30);
        c.g[S3][0] = 0xa;
        frame.save(c, S1, 0x70);
        c.g[S5][0] = 0x0049_0000;
        frame.save(c, S2, 0x60);
        frame.save(c, S4, 0x40);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(nt_r(c, S0));

        nt_skip_blank(c)?;
        let store = nt_token(c)?;
        if store {
            c.cycles += 1;
            c.sw(nt_f(c, NEXT_TOKEN_KIND), c.g[V0][0]);
        }

        c.cycles += 11;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The whitespace loop: skip blanks, newlines (counting lines), comments and
/// line directives while the current character is whitespace. Leaves
/// `v0` = base, `s2` = the last character looked at, `s4` = token text.
fn nt_skip_blank(c: &mut Cpu) -> Result<(), Unwound> {
    loop {
        // 0x321b28: peek (v0 = base).
        c.cycles += 8;
        c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
        c.g[A0][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
        c.g[A3][0] = c.g[V0][0];
        c.g[V0][0] = sx(nt_r(c, V0).wrapping_add(nt_r(c, V1)));
        c.g[A2][0] = c.g[V1][0];
        c.g[V1][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
        c.g[S2][0] = u64::MAX;
        if c.g[V1][0] != 0 {
            c.cycles += 1;
            c.g[S2][0] = c.lbu(nt_r(c, A0));
        }
        let ready = nt_skip_one(c)?;
        if !ready {
            c.cycles += 1;
            c.g[V1][0] = sx(nt_r(c, S2) << 24);
        }
        // 0x3222fc: loop while the character is whitespace.
        c.cycles += 4;
        c.g[V1][0] = sx(((nt_r(c, V1)) as i32 >> 24) as u32);
        c.g[V0][0] = 0x20;
        c.g[A0][0] = 0;
        let ch = c.g[V1][0];
        nt_space_v0(c, ch, A0);
        c.cycles += 2;
        c.g[V0][0] = c.lw(nt_r(c, S0));
        if c.g[A0][0] == 0 {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_1b28);
        }
    }
}

/// One step of the whitespace loop for the character in `s2`. True when `v1`
/// already holds `s2 << 24`.
fn nt_skip_one(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 2;
    c.g[V0][0] = 0x23;
    if c.g[S2][0] == c.g[S3][0] {
        // Newline: step, line start, count the line.
        c.cycles += 2;
        let more = c.g[V1][0] != 0;
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
        if more {
            c.cycles += 1;
            c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
        }
        c.cycles += 7;
        c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_LINE));
        c.g[V0][0] = 1;
        c.sw(nt_f(c, NEXT_TOKEN_LINE_START), c.g[V0][0]);
        c.g[S4][0] = sx(nt_f(c, NEXT_TOKEN_TEXT));
        c.g[V1][0] = sx(nt_r(c, V1).wrapping_add(1));
        c.sw(nt_f(c, NEXT_TOKEN_LINE), c.g[V1][0]);
        return Ok(false);
    }
    c.cycles += 2;
    let hash = c.g[S2][0] == c.g[V0][0];
    c.g[V0][0] = 0x2f;
    if hash {
        c.cycles += 3;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_LINE_START));
        let line_start = c.g[V0][0] != 0;
        c.g[V0][0] = 0x2f;
        if line_start {
            return nt_directive(c);
        }
    }
    // 0x3221ec: `//` comment?
    c.cycles += 2;
    let slash = c.g[S2][0] == c.g[V0][0];
    c.g[V1][0] = sx(nt_r(c, S2) << 24);
    if slash {
        c.cycles += 5;
        c.g[V0][0] = sx(nt_r(c, A3).wrapping_add(nt_r(c, A2)));
        c.g[V1][0] = sx(nt_r(c, A0).wrapping_add(1));
        c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
        c.g[V0][0] = u64::MAX;
        if c.g[V1][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = c.lbu(nt_r(c, A0).wrapping_add(1));
        }
        c.cycles += 2;
        let second = c.g[V0][0] == c.g[S2][0];
        c.g[V1][0] = sx(nt_r(c, S2) << 24);
        if second {
            nt_line_comment(c);
            return Ok(false);
        }
    }
    // 0x3222ac: other whitespace is stepped over.
    c.cycles += 4;
    c.g[V1][0] = sx((nt_r(c, V1) as i32 >> 24) as u32);
    c.g[V0][0] = 0x20;
    c.g[A1][0] = 0;
    let ch = c.g[V1][0];
    nt_space_v0(c, ch, A1);
    c.cycles += 2;
    c.g[S4][0] = sx(nt_f(c, NEXT_TOKEN_TEXT));
    if c.g[A1][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = sx(nt_r(c, A3).wrapping_add(nt_r(c, A2)));
        c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
        if more {
            c.cycles += 1;
            c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
        }
    }
    Ok(false)
}

/// `//`: step over both slashes and the rest of the line (up to the
/// newline); the character becomes ' '.
fn nt_line_comment(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
    c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
    c.g[A1][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
    c.g[A0][0] = sx(nt_r(c, A0).wrapping_add(nt_r(c, V1)));
    c.g[V0][0] = (c.g[A1][0] < c.g[A0][0]) as u64;
    let more = c.g[V0][0] != 0;
    c.g[V0][0] = sx(nt_r(c, A1).wrapping_add(1));
    if more {
        c.cycles += 3;
        c.g[V1][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
        c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
        if c.g[V1][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = sx(nt_r(c, A1).wrapping_add(2));
            c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
        }
    }
    c.cycles += 2;
    c.g[S4][0] = sx(nt_f(c, NEXT_TOKEN_TEXT));
    loop {
        c.cycles += 8;
        c.g[A0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
        c.g[A2][0] = u64::MAX;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
        c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
        c.g[V0][0] = sx(nt_r(c, V0).wrapping_add(nt_r(c, A0)));
        c.g[A1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
        c.g[A0][0] = c.g[V1][0];
        if c.g[A1][0] != 0 {
            c.cycles += 1;
            c.g[A2][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 2;
        if c.g[A2][0] == c.g[S3][0] {
            c.cycles += 1;
            c.sw(nt_f(c, NEXT_TOKEN_LINE_START), 0);
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_2250);
        }
        c.cycles += 2;
        c.g[V1][0] = u64::MAX;
        if c.g[A1][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        if c.g[V1][0] == c.g[V0][0] {
            c.sw(nt_f(c, NEXT_TOKEN_LINE_START), 0);
            break;
        }
        c.cycles += 2;
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
        if c.g[A1][0] != 0 {
            c.cycles += 1;
            c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
        }
    }
    c.cycles += 2;
    c.g[S2][0] = 0x20;
}

/// `#` at the start of a line: `# <line> "<file>"`. Sets the line counter
/// and the file name, reporting "malformed pre-processor line" (and skipping
/// to the end of the line) when the shape is wrong. True when `v1` already
/// holds `s2 << 24`.
fn nt_directive(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 2;
    let more = c.g[V1][0] != 0;
    c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
    if more {
        c.cycles += 1;
        c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
    }
    c.cycles += 2;
    c.g[S4][0] = sx(nt_f(c, NEXT_TOKEN_TEXT));
    // Blanks before the line number.
    loop {
        c.cycles += 9;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
        c.g[A1][0] = u64::MAX;
        c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
        c.g[A0][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
        c.g[A3][0] = c.g[V0][0];
        c.g[V0][0] = sx(nt_r(c, V0).wrapping_add(nt_r(c, V1)));
        c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
        c.g[A2][0] = c.g[V1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[A1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 5;
        c.g[V1][0] = sx(nt_r(c, A1) << 24);
        c.g[V0][0] = 0x20;
        c.g[V1][0] = sx((nt_r(c, V1) as i32 >> 24) as u32);
        c.g[A1][0] = 0;
        let ch = c.g[V1][0];
        nt_space_v0(c, ch, A1);
        c.cycles += 2;
        c.g[V0][0] = sx(nt_r(c, A3).wrapping_add(nt_r(c, A2)));
        if c.g[A1][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_1ba8);
        }
        c.cycles += 3;
        c.g[V1][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
        c.g[A1][0] = u64::MAX;
        if c.g[V1][0] != 0 {
            c.cycles += 1;
            c.g[A1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        if c.g[A1][0] == c.g[V0][0] {
            c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
            break;
        }
        c.cycles += 2;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 2;
        let newline = c.g[V0][0] == c.g[S3][0];
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
        if newline {
            break;
        }
        c.cycles += 6;
        let more = nt_more_v0(c);
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
        if more {
            c.cycles += 1;
            c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
        }
    }
    // 0x321c58: a digit?
    c.cycles += 6;
    let more = nt_more_v0(c);
    c.g[A1][0] = u64::MAX;
    if more {
        c.cycles += 1;
        c.g[A1][0] = c.lbu(nt_r(c, A0));
    }
    c.cycles += 6;
    let ch = nt_sext8(c.g[A1][0]);
    if nt_digit(c, ch) {
        c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
        return nt_directive_line(c);
    }
    // Malformed: skip to the end of the line.
    c.cycles += 10;
    c.g[A1][0] = sx(nt_r(c, S5).wrapping_sub(0x2218));
    c.g[A0][0] = c.g[S0][0];
    NEXT_TOKEN_ERROR_FN.call(c, 0x0032_1c98)?;
    c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
    c.g[T0][0] = 0xa;
    c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
    c.g[A3][0] = u64::MAX;
    c.g[A1][0] = sx(nt_r(c, V1).wrapping_add(nt_r(c, V0)));
    c.g[A2][0] = c.g[A1][0];
    loop {
        c.cycles += 5;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
        c.g[V1][0] = u64::MAX;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 2;
        let newline = c.g[V1][0] == c.g[T0][0];
        c.g[V0][0] = (c.g[A0][0] < c.g[A2][0]) as u64;
        if newline {
            return Ok(false);
        }
        c.cycles += 2;
        if event_due(c) {
            deliver_event_here(c, 0x0032_1cb8);
        }
        c.cycles += 2;
        c.g[V1][0] = u64::MAX;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 2;
        let end = c.g[V1][0] == c.g[A3][0];
        c.g[V1][0] = sx(nt_r(c, S2) << 24);
        if end {
            return Ok(true);
        }
        c.cycles += 7;
        let more = nt_more(c);
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
        if more {
            c.cycles += 1;
            c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
        }
    }
}

/// The directive's line number and quoted file name (`v1` = base, `a0` =
/// position).
fn nt_directive_line(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 5;
    c.g[A3][0] = 0xa;
    c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
    c.sw(nt_f(c, NEXT_TOKEN_LINE), 0);
    c.g[A2][0] = sx(nt_r(c, V1).wrapping_add(nt_r(c, V0)));
    // line = line * 10 + digit
    loop {
        c.cycles += 4;
        c.g[A0][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
        c.g[V0][0] = (c.g[A0][0] < c.g[A2][0]) as u64;
        c.g[V1][0] = u64::MAX;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 6;
        let ch = nt_sext8(c.g[V1][0]);
        let digit = nt_digit(c, ch);
        c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
        if !digit {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_1d28);
        }
        c.cycles += 5;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
        c.g[V1][0] = sx(nt_r(c, V1).wrapping_add(nt_r(c, V0)));
        c.g[V1][0] = (c.g[A0][0] < c.g[V1][0]) as u64;
        c.g[A1][0] = u64::MAX;
        if c.g[V1][0] != 0 {
            c.cycles += 1;
            c.g[A1][0] = c.lbu(nt_r(c, A0));
        }
        c.cycles += 6;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_LINE));
        c.mult(0, V0, V0, A3);
        c.g[V0][0] = sx(nt_r(c, V0).wrapping_sub(0x30));
        c.g[V0][0] = sx(nt_r(c, V0).wrapping_add(nt_r(c, A1)));
        let more = c.g[V1][0] != 0;
        c.sw(nt_f(c, NEXT_TOKEN_LINE), c.g[V0][0]);
        if more {
            c.cycles += 2;
            c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
            c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
        }
    }
    // Blanks, then the opening quote.
    c.cycles += 11;
    c.g[T5][0] = 0x20;
    c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
    c.g[T4][0] = 9;
    c.g[A3][0] = c.g[V1][0];
    c.g[T0][0] = 0xa;
    c.g[A1][0] = sx(nt_r(c, V1).wrapping_add(nt_r(c, V0)));
    c.g[A2][0] = c.g[V0][0];
    c.g[T1][0] = c.g[A1][0];
    c.g[T3][0] = 0xd;
    c.g[T2][0] = u64::MAX;
    nt_blank_run(c, 0x0032_1db8, T1, false);
    c.cycles += 3;
    c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
    c.g[V1][0] = u64::MAX;
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.g[V1][0] = c.lbu(nt_r(c, A0));
    }
    c.cycles += 3;
    c.g[V0][0] = 0x22;
    let quote = c.g[V1][0] == c.g[V0][0];
    c.g[A1][0] = sx(nt_r(c, S5).wrapping_sub(0x2218));
    if !quote {
        // Malformed: skip to the end of the line; the character becomes ' '.
        c.cycles += 9;
        c.g[A0][0] = c.g[S0][0];
        NEXT_TOKEN_ERROR_FN.call(c, 0x0032_1e74)?;
        c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
        c.g[T0][0] = 0xa;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
        c.g[A3][0] = u64::MAX;
        c.g[A1][0] = sx(nt_r(c, V1).wrapping_add(nt_r(c, V0)));
        c.g[A2][0] = c.g[A1][0];
        loop {
            c.cycles += 5;
            c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
            c.g[V1][0] = u64::MAX;
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                c.g[V1][0] = c.lbu(nt_r(c, A0));
            }
            c.cycles += 2;
            let newline = c.g[V1][0] == c.g[T0][0];
            c.g[V0][0] = (c.g[A0][0] < c.g[A2][0]) as u64;
            if newline {
                c.cycles += 2;
                c.g[S2][0] = 0x20;
                return Ok(false);
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_1e90);
            }
            c.cycles += 2;
            c.g[V1][0] = u64::MAX;
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                c.g[V1][0] = c.lbu(nt_r(c, A0));
            }
            c.cycles += 2;
            let end = c.g[V1][0] == c.g[A3][0];
            c.g[S2][0] = 0x20;
            if end {
                return Ok(false);
            }
            c.cycles += 6;
            c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
            c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
            c.g[V0][0] = sx(nt_r(c, V0).wrapping_add(nt_r(c, V1)));
            c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
            if more {
                c.cycles += 1;
                c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
            }
        }
    }
    nt_adv7(c);
    nt_directive_file(c)
}

/// The quoted file name (at most 127 characters kept) and the end of the
/// directive line.
fn nt_directive_file(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 4;
    c.g[S1][0] = sx(nt_f(c, NEXT_TOKEN_FNAME));
    c.g[T0][0] = 0;
    c.g[A1][0] = 0x0049_0000;
    'name: loop {
        c.cycles += 1;
        c.g[A0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
        loop {
            c.cycles += 7;
            c.g[A3][0] = u64::MAX;
            c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
            c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
            c.g[V0][0] = sx(nt_r(c, V0).wrapping_add(nt_r(c, A0)));
            c.g[A2][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
            c.g[A0][0] = c.g[V1][0];
            if c.g[A2][0] != 0 {
                c.cycles += 1;
                c.g[A3][0] = c.lbu(nt_r(c, A0));
            }
            c.cycles += 2;
            let unterminated = if c.g[A3][0] == c.g[S3][0] {
                c.g[A0][0] = c.g[S0][0];
                true
            } else {
                c.cycles += 2;
                c.g[V1][0] = u64::MAX;
                if c.g[A2][0] != 0 {
                    c.cycles += 1;
                    c.g[V1][0] = c.lbu(nt_r(c, A0));
                }
                c.cycles += 3;
                c.g[V0][0] = u64::MAX;
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S0][0];
                    true
                } else {
                    false
                }
            };
            if unterminated {
                c.cycles += 9;
                c.g[A1][0] = sx(nt_r(c, A1).wrapping_sub(0x2218));
                NEXT_TOKEN_ERROR_FN.call(c, 0x0032_1f68)?;
                let more = nt_more(c);
                c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
                if more {
                    c.cycles += 2;
                    c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
                }
                break 'name;
            }
            c.cycles += 2;
            c.g[V1][0] = u64::MAX;
            if c.g[A2][0] != 0 {
                c.cycles += 1;
                c.g[V1][0] = c.lbu(nt_r(c, A0));
            }
            c.cycles += 3;
            c.g[V0][0] = 0x22;
            let close = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = ((c.g[T0][0] as i64) < 127) as u64;
            if close {
                c.cycles += 2;
                let more = c.g[A2][0] != 0;
                c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
                if more {
                    c.cycles += 2;
                    c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
                }
                break 'name;
            }
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
            } else {
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                if c.g[A2][0] != 0 {
                    c.cycles += 1;
                    c.g[V0][0] = c.lbu(nt_r(c, A0));
                }
                c.cycles += 4;
                c.sb(nt_r(c, S1), c.g[V0][0]);
                c.g[T0][0] = sx(nt_r(c, T0).wrapping_add(1));
                c.g[S1][0] = sx(nt_r(c, S1).wrapping_add(1));
                c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
            }
            c.cycles += 6;
            if !nt_more_v0(c) {
                c.g[A0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
                if event_due(c) {
                    deliver_event_here(c, 0x0032_1f1c);
                }
                continue;
            }
            c.cycles += 3;
            c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
            c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
            if event_due(c) {
                deliver_event_here(c, 0x0032_1f18);
            }
            continue 'name;
        }
    }
    // 0x321ffc: terminate the name; blanks; then the newline.
    c.cycles += 13;
    c.sb(nt_r(c, S1), 0);
    c.g[T5][0] = 0x20;
    c.g[T4][0] = 9;
    c.g[T1][0] = 0xa;
    c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
    c.g[T3][0] = 0xd;
    c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
    c.g[T2][0] = u64::MAX;
    c.g[A3][0] = c.g[V1][0];
    c.g[A1][0] = sx(nt_r(c, V1).wrapping_add(nt_r(c, V0)));
    c.g[A2][0] = c.g[V0][0];
    c.g[T0][0] = c.g[A1][0];
    nt_blank_run(c, 0x0032_2030, T0, false);
    c.cycles += 3;
    c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
    c.g[V1][0] = u64::MAX;
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.g[V1][0] = c.lbu(nt_r(c, A0));
    }
    c.cycles += 2;
    let newline = c.g[V1][0] == c.g[S3][0];
    c.g[A1][0] = sx(nt_r(c, S5).wrapping_sub(0x2218));
    if newline {
        nt_adv7(c);
        c.cycles += 1;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
    } else {
        // Junk after the name: report it and skip blanks.
        c.cycles += 12;
        c.g[A0][0] = c.g[S0][0];
        NEXT_TOKEN_ERROR_FN.call(c, 0x0032_20e8)?;
        c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
        c.g[T3][0] = 0x20;
        c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
        c.g[T2][0] = 9;
        c.g[A2][0] = 0xa;
        c.g[T1][0] = 0xd;
        c.g[A1][0] = sx(nt_r(c, V1).wrapping_add(nt_r(c, V0)));
        c.g[T0][0] = u64::MAX;
        c.g[A3][0] = c.g[A1][0];
        nt_blank_run(c, 0x0032_2110, A3, true);
    }
    // 0x3221cc: the current character.
    c.cycles += 6;
    let more = nt_more_v0(c);
    c.g[S2][0] = u64::MAX;
    if more {
        c.cycles += 2;
        c.g[S2][0] = c.lbu(nt_r(c, A0));
    }
    Ok(false)
}

/// `cmp`: the dispatch compare `s2 == ch` (2 cycles) with the delay slot's
/// register load.
#[inline(always)]
fn nt_is(c: &mut Cpu, ch: u64, delay: usize, value: u64) -> bool {
    c.cycles += 2;
    let hit = c.g[S2][0] == ch;
    c.g[delay][0] = value;
    hit
}

/// A one-character token: step, terminate the text, kind `kind`.
fn nt_single(c: &mut Cpu, kind: u64) -> Result<bool, Unwound> {
    nt_adv7(c);
    c.cycles += 3;
    c.sb(nt_r(c, S1), 0);
    c.g[V0][0] = kind;
    Ok(true)
}

/// `X` or `X=` (`!`, `=`, `<`, `>`; `a1` = '='), after stepping past `X`.
fn nt_pair_eq(c: &mut Cpu, first: u64, single: u64, pair: u64) -> Result<bool, Unwound> {
    nt_adv7(c);
    nt_peek_s2(c);
    c.cycles += first;
    if first == 3 {
        c.g[V0][0] = 0x3d;
    }
    let eq_reg = if first == 3 { V0 } else { A1 };
    if c.g[S2][0] != c.g[eq_reg][0] {
        c.sb(nt_r(c, S1), 0);
        c.cycles += 2;
        c.g[V0][0] = single;
        return Ok(true);
    }
    nt_push_adv(c, S2);
    c.cycles += 3;
    c.sb(nt_r(c, S1), 0);
    c.g[V0][0] = pair;
    Ok(true)
}

/// `++` / `-+`: "Operator '++' not defined.", kind 1.
fn nt_plus_plus(c: &mut Cpu) -> Result<bool, Unwound> {
    nt_push_adv(c, S2);
    c.cycles += 7;
    c.sb(nt_r(c, S1), 0);
    c.g[A1][0] = 0x0049_0000;
    c.g[A1][0] = sx(nt_r(c, A1).wrapping_sub(0x21f8));
    c.g[A0][0] = c.g[S0][0];
    NEXT_TOKEN_ERROR_FN.call(c, 0x0032_24b4)?;
    c.g[V0][0] = 1;
    Ok(true)
}

/// `&&` / `||`: `None` (with `v0` = `fail_v0`) when the next character is
/// not the same, and the dispatch goes on.
fn nt_double(c: &mut Cpu, fail_v0: u64, kind: u64) -> Option<()> {
    c.cycles += 8;
    c.g[A1][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
    c.g[V0][0] = c.lw(nt_f(c, NEXT_TOKEN_BASE));
    c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
    c.g[A2][0] = sx(nt_r(c, A1).wrapping_add(1));
    c.g[V0][0] = sx(nt_r(c, V0).wrapping_add(nt_r(c, V1)));
    c.g[A0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
    c.g[A3][0] = u64::MAX;
    if c.g[A0][0] != 0 {
        c.cycles += 1;
        c.g[A3][0] = c.lbu(nt_r(c, A1).wrapping_add(1));
    }
    c.cycles += 2;
    if c.g[A3][0] != c.g[S2][0] {
        c.g[V0][0] = fail_v0;
        return None;
    }
    c.cycles += 3;
    c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
    if c.g[V0][0] != 0 {
        c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[A2][0]);
    }
    c.cycles += 7;
    let more = nt_more(c);
    c.g[A1][0] = u64::MAX;
    if more {
        c.cycles += 1;
        c.g[A1][0] = c.lbu(nt_r(c, A0));
    }
    nt_push_adv(c, A1);
    c.cycles += 3;
    c.sb(nt_r(c, S1), 0);
    c.g[V0][0] = kind;
    Some(())
}

/// Store `s2` in the token text, or (63 characters reached) report
/// `msg = s6 - msg_off`, set the kind to 1 (`s5`) and store it anyway.
fn nt_push_checked(c: &mut Cpu, msg_off: u32, ret_addr: u32) -> Result<(), Unwound> {
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.sb(nt_r(c, S1), c.g[S2][0]);
        return Ok(());
    }
    c.cycles += 6;
    c.sb(nt_r(c, S1), 0);
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx(nt_r(c, S6).wrapping_sub(msg_off));
    NEXT_TOKEN_ERROR_FN.call(c, ret_addr)?;
    c.sw(nt_f(c, NEXT_TOKEN_KIND), c.g[S5][0]);
    c.sb(nt_r(c, S1), c.g[S2][0]);
    Ok(())
}

/// The exponent's checked store (the length computed from `s1`).
fn nt_push_checked_exp(c: &mut Cpu, ret_addr: u32) -> Result<(), Unwound> {
    c.cycles += 5;
    c.g[V0][0] = sx(nt_r(c, S1).wrapping_sub(0x9c));
    c.g[V0][0] = sx(nt_r(c, V0).wrapping_sub(nt_r(c, S0)));
    c.g[V0][0] = ((c.g[V0][0] as i64) < 63) as u64;
    if c.g[V0][0] != 0 {
        c.sb(nt_r(c, S1), c.g[S2][0]);
        return Ok(());
    }
    c.cycles += 8;
    c.sb(nt_r(c, S1), 0);
    c.g[A1][0] = 0x0049_0000;
    c.g[A1][0] = sx(nt_r(c, A1).wrapping_sub(0x21a0));
    c.g[A0][0] = c.g[S0][0];
    NEXT_TOKEN_ERROR_FN.call(c, ret_addr)?;
    c.g[V0][0] = 1;
    c.sw(nt_f(c, NEXT_TOKEN_KIND), c.g[V0][0]);
    c.sb(nt_r(c, S1), c.g[S2][0]);
    Ok(())
}

/// Step after a stored character, `s1` moving on (the 7-cycle form).
fn nt_step_s1(c: &mut Cpu) {
    c.cycles += 7;
    let more = nt_more(c);
    c.g[S1][0] = sx(nt_r(c, S1).wrapping_add(1));
    if more {
        c.cycles += 2;
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
        c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
    }
}

/// Count, step and move `s1` after a stored character (8 cycles); the
/// position store costs `store_cycles` (3 on a loop back edge, else 2).
fn nt_count_step(c: &mut Cpu, store_cycles: u64) {
    c.cycles += 8;
    c.g[S3][0] = sx(nt_r(c, S3).wrapping_add(1));
    let more = nt_more(c);
    c.g[S1][0] = sx(nt_r(c, S1).wrapping_add(1));
    if more {
        c.cycles += store_cycles;
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
        c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
    }
}

/// Peek into `s2`, then: is it a digit? (`v0` = `s3 < 63` after.)
fn nt_peek_digit(c: &mut Cpu) -> bool {
    nt_peek_s2(c);
    c.cycles += 6;
    let ch = nt_sext8(c.g[S2][0]);
    let digit = nt_digit(c, ch);
    c.g[V0][0] = ((c.g[S3][0] as i64) < 63) as u64;
    digit
}

/// `isalpha` of `a0` (sign-extended): `a1` = result. The first compare is
/// the caller's.
fn nt_alpha_tail(c: &mut Cpu) {
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.g[A1][0] = 1;
        return;
    }
    c.cycles += 4;
    c.g[V0][0] = sx(nt_r(c, A0).wrapping_sub(0x41));
    c.g[V0][0] = (c.g[V0][0] < 26) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.g[A1][0] = 1;
    }
}

/// The token itself, from the first non-blank character. True when the
/// kind in `v0` is still to be stored.
fn nt_token(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 8;
    c.g[S1][0] = c.g[S4][0];
    c.g[V1][0] = c.lw(nt_f(c, NEXT_TOKEN_LEN));
    c.g[S2][0] = u64::MAX;
    c.g[A0][0] = c.lw(nt_f(c, NEXT_TOKEN_POS));
    c.g[V0][0] = sx(nt_r(c, V0).wrapping_add(nt_r(c, V1)));
    c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
    let more = c.g[V0][0] != 0;
    c.sw(nt_f(c, NEXT_TOKEN_LINE_START), 0);
    if more {
        c.cycles += 1;
        c.g[S2][0] = c.lbu(nt_r(c, A0));
    }
    c.cycles += 4;
    c.sb(nt_r(c, S1), c.g[S2][0]);
    c.g[V0][0] = u64::MAX;
    let end = c.g[S2][0] == c.g[V0][0];
    c.g[S1][0] = sx(nt_r(c, S1).wrapping_add(1));
    if end {
        nt_adv7(c);
        c.cycles += 3;
        c.sb(nt_r(c, S1), 0);
        c.sw(nt_f(c, NEXT_TOKEN_KIND), 0);
        return Ok(false);
    }

    // Operators.
    c.cycles += 1;
    if nt_is(c, 0x21, V0, 0x21) {
        c.g[A1][0] = 0x2b;
        return nt_pair_eq(c, 3, 0x18, 0x13);
    }
    c.g[A1][0] = 0x2b;
    if nt_is(c, 0x2b, V0, 0x2d) {
        nt_adv7(c);
        nt_peek_s2(c);
        c.cycles += 2;
        if c.g[S2][0] != c.g[A1][0] {
            c.sb(nt_r(c, S1), 0);
            c.cycles += 2;
            c.g[V0][0] = 0x14;
            return Ok(true);
        }
        return nt_plus_plus(c);
    }
    if nt_is(c, 0x2d, V0, 0x2e) {
        nt_adv7(c);
        nt_peek_s2(c);
        c.cycles += 2;
        let plus = c.g[S2][0] == c.g[A1][0];
        c.g[V0][0] = 0x15;
        if plus {
            if event_due(c) {
                deliver_event_here(c, 0x0032_2478);
            }
            return nt_plus_plus(c);
        }
        c.cycles += 2;
        c.sb(nt_r(c, S1), 0);
        return Ok(true);
    }
    if nt_is(c, 0x2e, V0, 0x2a) {
        return nt_single(c, 4);
    }
    if nt_is(c, 0x2a, V0, 0x2f) {
        return nt_single(c, 0x16);
    }
    if nt_is(c, 0x2f, A1, 0x3d) {
        return nt_single(c, 0x17);
    }
    if nt_is(c, 0x3d, V0, 0x3c) {
        return nt_pair_eq(c, 2, 3, 0x12);
    }
    if nt_is(c, 0x3c, V0, 0x3e) {
        return nt_pair_eq(c, 2, 0x10, 0x11);
    }
    if nt_is(c, 0x3e, V0, 0x26) {
        return nt_pair_eq(c, 2, 0xe, 0xf);
    }
    if nt_is(c, 0x26, V0, 0x7c) && nt_double(c, 0x7c, 0xd).is_some() {
        return Ok(true);
    }
    if nt_is(c, 0x7c, V0, 0x28) && nt_double(c, 0x28, 0xc).is_some() {
        return Ok(true);
    }
    if nt_is(c, 0x28, V0, 0x29) {
        return nt_single(c, 9);
    }
    if nt_is(c, 0x29, V0, 0x2c) {
        return nt_single(c, 0xa);
    }
    if nt_is(c, 0x2c, V0, 0x3f) {
        return nt_single(c, 8);
    }
    if nt_is(c, 0x3f, V0, 0x3b) {
        return nt_single(c, 6);
    }
    if nt_is(c, 0x3b, V0, 0x3a) {
        return nt_single(c, 5);
    }
    let shifted = sx(nt_r(c, S2) << 24);
    if nt_is(c, 0x3a, V1, shifted) {
        return nt_single(c, 7);
    }

    // Identifier or keyword?
    c.cycles += 5;
    c.g[A0][0] = sx((nt_r(c, V1) as i32 >> 24) as u32);
    c.g[V0][0] = sx(nt_r(c, A0).wrapping_sub(0x61));
    c.g[V0][0] = (c.g[V0][0] < 26) as u64;
    c.g[A1][0] = 0;
    nt_alpha_tail(c);
    c.cycles += 2;
    c.g[V0][0] = 0x2b;
    if c.g[A1][0] != 0 {
        return nt_identifier(c);
    }

    // Number?
    c.cycles += 2;
    let number = if c.g[S2][0] == c.g[V0][0] {
        c.g[V0][0] = 0x2d;
        true
    } else {
        c.g[V0][0] = 0x2d;
        c.cycles += 2;
        let minus = c.g[S2][0] == c.g[V0][0];
        c.g[V0][0] = sx((nt_r(c, V1) as i32 >> 24) as u32);
        if minus {
            true
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx(nt_r(c, V0).wrapping_sub(0x30));
            c.g[V0][0] = (c.g[V0][0] < 10) as u64;
            c.g[A1][0] = 0x0049_0000;
            c.g[V0][0] != 0
        }
    };
    if number {
        return nt_number(c);
    }

    // "Unexpected character in input '%c'", kind 1.
    c.cycles += 12;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A1][0] = sx(nt_r(c, A1).wrapping_sub(0x2158));
    NEXT_TOKEN_ERROR_FN.call(c, 0x0032_3040)?;
    c.sb(nt_r(c, S1), 0);
    let more = nt_more(c);
    c.g[V0][0] = sx(nt_r(c, A0).wrapping_add(1));
    if more {
        c.cycles += 1;
        c.sw(nt_f(c, NEXT_TOKEN_POS), c.g[V0][0]);
    }
    c.cycles += 1;
    c.g[V0][0] = 1;
    Ok(true)
}

/// Letters, digits and `_` (63 kept, "Identifer exceeds 64 characters."
/// beyond), then the keyword lookup (`strcasecmp`).
fn nt_identifier(c: &mut Cpu) -> Result<bool, Unwound> {
    nt_adv7(c);
    c.cycles += 5;
    c.g[V0][0] = sx(nt_r(c, S1).wrapping_sub(0x9c));
    c.g[S7][0] = 0x5f;
    c.g[S3][0] = sx(nt_r(c, V0).wrapping_sub(nt_r(c, S0)));
    c.g[S6][0] = 0x0049_0000;
    c.g[S5][0] = 1;
    loop {
        nt_peek_s2(c);
        c.cycles += 6;
        c.g[V1][0] = sx(nt_r(c, S2) << 24);
        c.g[A0][0] = sx((nt_r(c, V1) as i32 >> 24) as u32);
        c.g[V0][0] = sx(nt_r(c, A0).wrapping_sub(0x61));
        c.g[V0][0] = (c.g[V0][0] < 26) as u64;
        c.g[A1][0] = 0;
        nt_alpha_tail(c);
        c.cycles += 2;
        c.g[V0][0] = ((c.g[S3][0] as i64) < 63) as u64;
        let take = if c.g[A1][0] != 0 {
            true
        } else {
            c.cycles += 5;
            let ch = sx((nt_r(c, V1) as i32 >> 24) as u32);
            let digit = nt_digit(c, ch);
            c.g[V0][0] = ((c.g[S3][0] as i64) < 63) as u64;
            if digit {
                true
            } else {
                c.cycles += 2;
                c.g[A1][0] = 0x004a_0000;
                c.g[S2][0] == c.g[S7][0]
            }
        };
        if !take {
            break;
        }
        nt_push_checked(c, 0x21d8, 0x0032_2aa8)?;
        nt_count_step(c, 3);
        if event_due(c) {
            deliver_event_here(c, 0x0032_2a20);
        }
    }
    // Keywords: max min pow atan2 atan sin cos exp log floor ceil abs sqrt
    // (0x4a3f30 + 8 * i, kinds 0x19..0x25), then "minradian" (0x26).
    c.cycles += 6;
    c.sb(nt_r(c, S1), 0);
    c.g[A1][0] = sx(nt_r(c, A1).wrapping_add(0x3f30));
    c.g[A0][0] = c.g[S4][0];
    call_ptr(c, NEXT_TOKEN_STRCASECMP, 0x0032_2aec)?;
    let differs = c.g[V0][0] != 0;
    c.g[A1][0] = 0x004a_0000;
    if !differs {
        c.cycles += 2;
        c.g[V0][0] = 0x19;
        return Ok(true);
    }
    for i in 1..13u32 {
        c.cycles += 5;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = sx(nt_r(c, A1).wrapping_add(0x3f30 + 8 * i));
        call_ptr(c, NEXT_TOKEN_STRCASECMP, 0x0032_2aec + 0x1c * i)?;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0x19 + i as u64;
            return Ok(true);
        }
        c.g[A1][0] = if i == 12 { 0x0049_0000 } else { 0x004a_0000 };
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = sx(nt_r(c, A1).wrapping_sub(0x21b0));
    call_ptr(c, NEXT_TOKEN_STRCASECMP, 0x0032_2c58)?;
    let differs = c.g[V0][0] != 0;
    c.g[V0][0] = 2;
    if !differs {
        c.cycles += 2;
        c.g[V0][0] = 0x26;
    }
    Ok(true)
}

/// `[sign] digits [. digits] [(E|e) [sign] digits] [f]` (63 characters kept,
/// "Constant exceeds 64 characters." beyond), parsed with `sscanf("%f")`
/// into the value; kind 0xb.
fn nt_number(c: &mut Cpu) -> Result<bool, Unwound> {
    nt_adv7(c);
    c.cycles += 6;
    c.g[V0][0] = sx(nt_r(c, S1).wrapping_sub(0x9c));
    c.g[S7][0] = sx(nt_f(c, NEXT_TOKEN_VALUE));
    c.g[S3][0] = sx(nt_r(c, V0).wrapping_sub(nt_r(c, S0)));
    c.g[S6][0] = 0x0049_0000;
    c.g[S5][0] = 1;
    while nt_peek_digit(c) {
        nt_push_checked(c, 0x21a0, 0x0032_2d10)?;
        nt_count_step(c, 3);
        if event_due(c) {
            deliver_event_here(c, 0x0032_2cc0);
        }
    }
    // '.', 'E' or 'e'?
    c.cycles += 3;
    c.g[V0][0] = 0x2e;
    let dot = c.g[S2][0] == c.g[V0][0];
    c.g[V0][0] = 0x45;
    let mut check_e = false;
    let mut exponent = false;
    if dot {
        // Fraction.
        c.cycles += 6;
        c.g[V0][0] = sx(nt_r(c, S1).wrapping_sub(0x9c));
        c.g[S6][0] = 0x0049_0000;
        c.g[S3][0] = sx(nt_r(c, V0).wrapping_sub(nt_r(c, S0)));
        c.g[S5][0] = 1;
        c.g[V0][0] = ((c.g[S3][0] as i64) < 63) as u64;
        loop {
            nt_push_checked(c, 0x21a0, 0x0032_2d98)?;
            nt_count_step(c, 2);
            if !nt_peek_digit(c) {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_2d80);
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 0x45;
        check_e = true;
    } else {
        c.cycles += 2;
        let upper = c.g[S2][0] == c.g[V0][0];
        c.g[V0][0] = 0x65;
        if upper {
            exponent = true;
        } else {
            c.cycles += 2;
            let lower = c.g[S2][0] == c.g[V0][0];
            c.g[V0][0] = 0x66;
            if lower {
                c.cycles += 2;
                c.g[V0][0] = 0x45;
                check_e = true;
            }
        }
    }
    if check_e {
        c.cycles += 2;
        let upper = c.g[S2][0] == c.g[V0][0];
        c.g[V0][0] = 0x65;
        if upper {
            exponent = true;
        } else {
            c.cycles += 2;
            exponent = c.g[S2][0] == c.g[V0][0];
            c.g[V0][0] = 0x66;
        }
    }
    if exponent {
        nt_push_checked_exp(c, 0x0032_2e3c)?;
        nt_step_s1(c);
        nt_peek_s2(c);
        c.cycles += 3;
        c.g[V0][0] = 0x2d;
        let minus = c.g[S2][0] == c.g[V0][0];
        c.g[V0][0] = 0x2b;
        let sign = if minus {
            true
        } else {
            c.cycles += 2;
            let plus = c.g[S2][0] == c.g[V0][0];
            c.g[V0][0] = sx(nt_r(c, S2) << 24);
            plus
        };
        if sign {
            nt_push_checked_exp(c, 0x0032_2ec8)?;
            nt_step_s1(c);
            nt_peek_s2(c);
            c.cycles += 1;
            c.g[V0][0] = sx(nt_r(c, S2) << 24);
        }
        c.cycles += 5;
        let ch = sx((nt_r(c, V0) as i32 >> 24) as u32);
        let digit = nt_digit(c, ch);
        c.g[V0][0] = sx(nt_r(c, S1).wrapping_sub(0x9c));
        if !digit {
            // "Malformed floating point constant.", kind 1.
            c.cycles += 8;
            c.sb(nt_r(c, S1), 0);
            c.g[A1][0] = 0x0049_0000;
            c.g[A1][0] = sx(nt_r(c, A1).wrapping_sub(0x2180));
            c.g[A0][0] = c.g[S0][0];
            NEXT_TOKEN_ERROR_FN.call(c, 0x0032_2f44)?;
            c.g[V0][0] = 1;
            c.sw(nt_f(c, NEXT_TOKEN_KIND), c.g[V0][0]);
            c.g[V0][0] = sx(nt_r(c, S1).wrapping_sub(0x9c));
        }
        c.cycles += 4;
        c.g[S6][0] = 0x0049_0000;
        c.g[S3][0] = sx(nt_r(c, V0).wrapping_sub(nt_r(c, S0)));
        c.g[S5][0] = 1;
        c.g[V0][0] = ((c.g[S3][0] as i64) < 63) as u64;
        loop {
            nt_push_checked(c, 0x21a0, 0x0032_2f78)?;
            nt_count_step(c, 2);
            if !nt_peek_digit(c) {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_2f60);
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 0x66;
    }
    // Optional 'f' suffix, then sscanf(text, "%f", &value).
    c.cycles += 2;
    if c.g[S2][0] != c.g[V0][0] {
        c.sb(nt_r(c, S1), 0);
    } else {
        nt_adv7(c);
        c.cycles += 1;
        c.sb(nt_r(c, S1), 0);
    }
    c.cycles += 8;
    c.g[A1][0] = 0x004a_0000;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = sx(nt_r(c, A1).wrapping_add(0x3f98));
    c.g[A2][0] = c.g[S7][0];
    call_ptr(c, NEXT_TOKEN_SSCANF, 0x0032_3024)?;
    c.g[V1][0] = 0xb;
    c.sw(nt_f(c, NEXT_TOKEN_KIND), c.g[V1][0]);
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, elf_image, DATA};
    use crate::game::fe::feasyncfile::tests::env;

    /// Script snippets reaching every branch of the tokenizer.
    fn next_token_cases() -> Vec<Vec<u8>> {
        let lits: &[&[u8]] = &[
            b"",
            b"   \t\r  ",
            b"\n\n  foo",
            b"\t\r\n  max",
            // Line directives.
            b"# 12 \"file.map\"\nmax",
            b"#12\"a\"  \nx",
            b"# x\nfoo",
            b"# 12 x\n=",
            b"# 7 \"unterminated\nabc",
            b"# 7 \"unterminated",
            b"# 3 \"f\" junk\n<",
            b"# 3 \"f\"  ",
            b"#",
            b"# 5",
            b"#\t9\"f\"\r\n:",
            b"# 1 \"f\"\t \n;",
            b"# 123456789 \"f\"\n(",
            b"#5 \"x\"\n",
            b"\n#1 \"g\"\n\n  foo",
            b" \n// c\n# 4 \"h\"\n 7.5",
            // Comments and operators.
            b"// comment here\n  sin(x)",
            b"//",
            b"// x",
            b"/ 3",
            b"/",
            b"/\n",
            b"!=",
            b"!x",
            b"!",
            b"++",
            b"+",
            b"+=",
            b"-+",
            b"-",
            b"-1",
            b".",
            b"*",
            b"==",
            b"= 1",
            b"=",
            b"<=",
            b"<",
            b">=",
            b">",
            b"&&",
            b"&x",
            b"&",
            b"||",
            b"|",
            b"|y",
            b"(",
            b")",
            b",",
            b"?",
            b";",
            b":",
            // Identifiers and keywords.
            b"max",
            b"MIN(",
            b"pow",
            b"atan2",
            b"atan",
            b"sin",
            b"cos",
            b"exp",
            b"log",
            b"floor",
            b"ceil",
            b"abs",
            b"sqrt",
            b"minradian",
            b"foo_bar9 ",
            b"x",
            b"Zebra_",
            // Numbers.
            b"123",
            b"42;",
            b"1.5",
            b"0.25f",
            b"3f",
            b"2E3",
            b"2e-3",
            b"1.5e+2f",
            b"7e",
            b"7e+",
            b"7E-x",
            b"1.e5",
            b"1.",
            b"5x",
            b"9",
            b"0.5e",
            // Unexpected characters.
            b"@",
            b"\x80",
            b"\xff",
            b"~a",
            b"\xe9t",
            b"_x",
            b"$",
        ];
        let mut v: Vec<Vec<u8>> = lits.iter().map(|s| s.to_vec()).collect();
        let mut long_name = b"# 3 \"".to_vec();
        long_name.extend(std::iter::repeat(b'n').take(140));
        long_name.extend_from_slice(b"\"\n1");
        v.push(long_name);
        let mut long_ident = vec![b'a'; 60];
        long_ident.extend_from_slice(b"b_12345678");
        v.push(long_ident);
        let mut long_int = vec![b'0'; 70];
        long_int.push(b'1');
        v.push(long_int);
        let mut long_frac = b"0.".to_vec();
        long_frac.extend(std::iter::repeat(b'0').take(70));
        long_frac.push(b'5');
        v.push(long_frac);
        let mut long_exp = b"1e".to_vec();
        long_exp.extend(std::iter::repeat(b'0').take(70));
        long_exp.push(b'2');
        v.push(long_exp);
        let mut long_exp_sign = b"1".to_vec();
        long_exp_sign.extend(std::iter::repeat(b'0').take(66));
        long_exp_sign.extend_from_slice(b"e-1f");
        v.push(long_exp_sign);
        v
    }

    #[test]
    fn input_parser_next_token_matches() {
        let cases = next_token_cases();
        let n = cases.len() as u32;
        check(
            ee::s10::f_00321af0__generated,
            input_parser_next_token,
            2 * n,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_1af0, 0);
                elf_image(c);
                env(c);
                c.pc = 0x0032_1af0;
                let text = &cases[(seed % n) as usize];
                let this = DATA + 0x400;
                let buf = DATA + 0x1000;
                // Junk after the end: the bounds checks must stop there.
                for i in 0..0x40 {
                    c.bus
                        .write8(buf + text.len() as u32 + i, (r.next_u32() | 0x20) as u8);
                }
                for (i, b) in text.iter().enumerate() {
                    c.bus.write8(buf + i as u32, *b);
                }
                for off in (0..0x100).step_by(4) {
                    c.bus.write32(this + off, r.next_u32());
                }
                c.bus.write32(this + NEXT_TOKEN_BASE, buf);
                c.bus.write32(this + NEXT_TOKEN_LEN, text.len() as u32);
                c.bus.write32(this + NEXT_TOKEN_POS, buf);
                for (i, b) in b"pad.map\0".iter().enumerate() {
                    c.bus.write8(this + NEXT_TOKEN_FNAME + i as u32, *b);
                }
                c.bus.write32(this + NEXT_TOKEN_LINE, r.next_u32() % 1000);
                // Second round: not at the start of a line.
                let line_start = if seed < n { r.next_u32() | 1 } else { 0 };
                c.bus.write32(this + NEXT_TOKEN_LINE_START, line_start);
                c.bus.write32(this + NEXT_TOKEN_ERROR, r.next_u32() & 1);
                c.g[A0][0] = sx(this);
            },
        );
    }
}
