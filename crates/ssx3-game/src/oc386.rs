//! Staging module oc386 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc386`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0041_7ab8, svfscanf, "__svfscanf")];

// ---------------------------------------------------------------------------
// __svfscanf (newlib, with the multibyte format scan)

/// Callees of `__svfscanf` (all in shard s15).
pub(crate) mod svfscanf_callee {
    use super::{ee, Callee};

    pub const MBTOWC_R: Callee = Callee {
        addr: 0x0041_8958,
        f: ee::s15::_mbtowc_r,
    };
    pub const SREFILL: Callee = Callee {
        addr: 0x0041_7648,
        f: ee::s15::__srefill,
    };
    pub const SCCL: Callee = Callee {
        addr: 0x0041_85d0,
        f: ee::s15::__sccl,
    };
    pub const FREAD: Callee = Callee {
        addr: 0x0042_0260,
        f: ee::s15::fread,
    };
    pub const UNGETC: Callee = Callee {
        addr: 0x0042_0378,
        f: ee::s15::ungetc,
    };
    pub const ATOF: Callee = Callee {
        addr: 0x0041_87a0,
        f: ee::s15::atof,
    };
    pub const TRUNCDFSF2: Callee = Callee {
        addr: 0x0041_27a8,
        f: ee::s15::__truncdfsf2,
    };
}
use svfscanf_callee as svc;

/// The function's address range (for computed jumps that leave the tables).
const SVFSCANF_START: u32 = 0x0041_7ab8;
const SVFSCANF_END: u32 = 0x0041_85cc;

/// `_impure_ptr` and `__mb_cur_max`.
const SVFSCANF_IMPURE_PTR: u32 = 0x0045_4944;
const SVFSCANF_MB_CUR_MAX: u32 = 0x0045_5140;
/// `_ctype_ + 1` (0x4a0000 - 0x637f): bit 0 upper, bit 3 space.
const SVFSCANF_CTYPE: u32 = 0x0049_9c81;
/// `basefix[17]` (shorts): the base an octal/decimal digit switches to.
const SVFSCANF_BASEFIX: u32 = 0x0049_9d90;
/// Jump tables: conversion characters (0..0x78), conversion kinds (0..4),
/// integer characters ('+'..'x'), float characters ('+'..'e').
const SVFSCANF_SPEC_TABLE: u32 = 0x0049_9dc0;
const SVFSCANF_KIND_TABLE: u32 = 0x0049_9fb0;
const SVFSCANF_INT_TABLE: u32 = 0x0049_9fd0;
const SVFSCANF_FLOAT_TABLE: u32 = 0x0049_a110;
/// `strtol` / `strtoul`, the integer converters kept at `CCFN`.
const SVFSCANF_STRTOL: u32 = 0x0041_e9e0;
const SVFSCANF_STRTOUL: u32 = 0x0041_aa50;

/// Frame slots (offsets from `sp`): the `[` class table (256 bytes) at 0,
/// the number buffer at 0x100, then the scalar locals.
pub(crate) const SVFSCANF_FRAME: u32 = 0x320;
pub(crate) const SVFSCANF_BUF: u32 = 0x100;
pub(crate) const SVFSCANF_WC: u32 = 0x260;
pub(crate) const SVFSCANF_MBSTATE: u32 = 0x264;
pub(crate) const SVFSCANF_FMT: u32 = 0x268;
pub(crate) const SVFSCANF_NASSIGNED: u32 = 0x26c;
pub(crate) const SVFSCANF_CCFN: u32 = 0x270;
pub(crate) const SVFSCANF_PWC: u32 = 0x274;
pub(crate) const SVFSCANF_PMBSTATE: u32 = 0x278;

/// `FILE` fields: read pointer (at 0, read straight from `fp`) and bytes left.
#[cfg(test)]
pub(crate) const SVFSCANF_FILE_P: u32 = 0x0;
pub(crate) const SVFSCANF_FILE_R: u32 = 0x4;

// Flags (kept in s3).
const SVF_LONG: u64 = 0x1;
const SVF_LONGDBL: u64 = 0x2;
const SVF_SHORT: u64 = 0x4;
const SVF_SUPPRESS: u64 = 0x8;
const SVF_POINTER: u64 = 0x10;
const SVF_NOSKIP: u64 = 0x20;
const SVF_SIGNOK: u64 = 0x40;
const SVF_NDIGITS: u64 = 0x80;
const SVF_PFXOK: u64 = 0x100;
const SVF_NZDIGITS: u64 = 0x200;

// Registers the original keeps its state in.
const SVF_FILE: usize = S2; // FILE *fp
const SVF_AP: usize = S5; // va_list (8-byte slots)
const SVF_NREAD: usize = S7; // characters consumed
const SVF_BASE: usize = FP; // integer base
const SVF_FLAGS: usize = S3;
const SVF_WIDTH: usize = S4;

/// Where the scan continues after a directive.
#[derive(Clone, Copy, PartialEq)]
enum SvfNext {
    /// The loop top (0x417b18): `v0 = 0x450000`, then the head.
    Top,
    /// The loop head (0x417b1c): the next format character.
    Head,
    /// The epilogue with `v0` set (0x41859c).
    Exit,
    /// A computed jump left the tables (already reported).
    Out,
}

/// Register helpers as methods (so an argument may read `c` too).
trait SvfRegs {
    /// `r = sx(v)`.
    fn svf_set(&mut self, r: usize, v: u32);
    /// `lbu(addr) & 8`: the ctype space bit of the entry at `addr`.
    fn svf_space_bit(&mut self, addr: u32) -> u64;
}

impl SvfRegs for Cpu {
    #[inline(always)]
    fn svf_set(&mut self, r: usize, v: u32) {
        self.g[r][0] = sx(v);
    }
    #[inline(always)]
    fn svf_space_bit(&mut self, addr: u32) -> u64 {
        self.lbu(addr) & 0x8
    }
}

#[inline(always)]
fn svf_back_edge(c: &mut Cpu, head: u32) {
    if event_due(c) {
        deliver_event_here(c, head);
    }
}

/// Jump to the loop top with its event check.
fn svf_top(c: &mut Cpu) -> SvfNext {
    svf_back_edge(c, 0x0041_7b18);
    SvfNext::Top
}

/// Jump to the loop head with its event check.
fn svf_head(c: &mut Cpu) -> SvfNext {
    svf_back_edge(c, 0x0041_7b1c);
    SvfNext::Head
}

fn svf_out(c: &mut Cpu, target: u32) -> SvfNext {
    c.jump_out(target, SVFSCANF_START, SVFSCANF_END);
    SvfNext::Out
}

/// 0x41858c `input_failure`: `v1` holds `nassigned`; return it, or EOF when 0.
fn svf_input_failure(c: &mut Cpu) -> SvfNext {
    c.cycles += 3;
    c.g[V0][0] = if c.g[V1][0] != 0 {
        c.g[V1][0]
    } else {
        u64::MAX
    };
    SvfNext::Exit
}

/// 0x418588: load `nassigned` into `v1`, then `input_failure`.
fn svf_fail_load(c: &mut Cpu) -> SvfNext {
    c.cycles += 1;
    let sp = sp(c);
    c.g[V1][0] = c.lw(sp + SVFSCANF_NASSIGNED);
    svf_input_failure(c)
}

/// 0x418598 `match_failure`/done: return `nassigned`.
fn svf_ret_count(c: &mut Cpu) -> SvfNext {
    c.cycles += 1;
    let sp = sp(c);
    c.g[V0][0] = c.lw(sp + SVFSCANF_NASSIGNED);
    SvfNext::Exit
}

/// `__srefill(fp)` at the call returning to `ret`: true when it failed (EOF).
fn svf_refill(c: &mut Cpu, ret: u32) -> Result<bool, Unwound> {
    c.g[A0][0] = c.g[SVF_FILE][0];
    svc::SREFILL.call(c, ret)?;
    Ok(c.g[V0][0] != 0)
}

/// `int __svfscanf(FILE *fp, const char *fmt, va_list ap)` (0x417ab8): the
/// scanf engine behind `sscanf`. Walks the format (multibyte characters via
/// `_mbtowc_r`): white space skips input white space, literals must match,
/// `%` conversions (`*`, width, `h`/`l`/`L`, `c s [ d i o u x X p D O e f g
/// E G n`) read from `fp` and store through the `ap` slots. Returns the
/// number of assignments, or EOF when the input ran out before the first.
pub fn svfscanf(c: &mut Cpu) {
    guest(c, |c| {
        svf_prologue(c);
        let mut next = SvfNext::Top;
        loop {
            match next {
                SvfNext::Top => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x450000;
                }
                SvfNext::Head => {}
                SvfNext::Exit => break,
                SvfNext::Out => return Ok(()),
            }
            next = svf_directive(c)?;
        }
        svf_epilogue(c);
        Ok(())
    })
}

fn svf_prologue(c: &mut Cpu) {
    c.cycles += 24;
    let sp = sp(c).wrapping_sub(SVFSCANF_FRAME);
    c.g[SP][0] = sx(sp);
    c.g[V0][0] = sx(sp + SVFSCANF_WC);
    c.g[V1][0] = sx(sp + SVFSCANF_MBSTATE);
    c.sq(18, sp + 0x2f0);
    c.sq(21, sp + 0x2c0);
    c.g[S2][0] = c.g[A0][0];
    c.sq(23, sp + 0x2a0);
    c.g[S5][0] = c.g[A2][0];
    c.sq(30, sp + 0x290);
    c.g[S7][0] = 0;
    c.sq(16, sp + 0x310);
    c.g[FP][0] = 0;
    c.sq(17, sp + 0x300);
    c.sq(19, sp + 0x2e0);
    c.sq(20, sp + 0x2d0);
    c.sq(22, sp + 0x2b0);
    c.sd(sp + 0x280, c.g[RA][0]);
    c.sw(sp + SVFSCANF_MBSTATE, 0);
    c.sw(sp + SVFSCANF_FMT, c.g[A1][0]);
    c.sw(sp + SVFSCANF_CCFN, 0);
    c.sw(sp + SVFSCANF_NASSIGNED, 0);
    c.sw(sp + SVFSCANF_PWC, c.g[V0][0]);
    c.sw(sp + SVFSCANF_PMBSTATE, c.g[V1][0]);
}

fn svf_epilogue(c: &mut Cpu) {
    c.cycles += 12;
    let sp = sp(c);
    c.lq(16, sp + 0x310);
    c.lq(17, sp + 0x300);
    c.lq(18, sp + 0x2f0);
    c.lq(19, sp + 0x2e0);
    c.lq(20, sp + 0x2d0);
    c.lq(21, sp + 0x2c0);
    c.lq(22, sp + 0x2b0);
    c.lq(23, sp + 0x2a0);
    c.lq(30, sp + 0x290);
    c.g[RA][0] = c.ld(sp + 0x280);
    c.g[SP][0] = sx(sp.wrapping_add(SVFSCANF_FRAME));
    ret(c);
}

/// One format directive, from the loop head (0x417b1c).
fn svf_directive(c: &mut Cpu) -> Result<SvfNext, Unwound> {
    let sp = sp(c);
    // Next format character: nbytes = _mbtowc_r(_impure_ptr, &wc, fmt,
    // __mb_cur_max, &state); fmt += nbytes.
    c.cycles += 13;
    c.g[V1][0] = 0x450000;
    c.g[A0][0] = c.lw(SVFSCANF_IMPURE_PTR);
    c.g[A2][0] = c.lw(sp + SVFSCANF_FMT);
    c.g[A3][0] = c.lw(SVFSCANF_MB_CUR_MAX);
    c.g[A1][0] = c.lw(sp + SVFSCANF_PWC);
    c.g[T0][0] = c.lw(sp + SVFSCANF_PMBSTATE);
    svc::MBTOWC_R.call(c, 0x0041_7b38)?;
    c.g[A0][0] = c.lw(sp + SVFSCANF_FMT);
    c.g[S6][0] = c.g[V0][0];
    c.g[V0][0] = c.lhu(sp + SVFSCANF_WC);
    c.svf_set(A0, r32(c, A0).wrapping_add(r32(c, S6)));
    c.sw(sp + SVFSCANF_FMT, c.g[A0][0]);
    if c.g[V0][0] == 0 {
        // End of the format.
        return Ok(svf_ret_count(c));
    }

    c.cycles += 3;
    let single = c.g[S6][0] == 1;
    c.g[V0][0] = 0x1;
    c.g[V1][0] = c.lhu(sp + SVFSCANF_WC);
    if single {
        c.cycles += 7;
        c.g[V0][0] = 0x4a0000;
        c.g[A0][0] = sx(SVFSCANF_CTYPE);
        c.svf_set(V1, r32(c, V1).wrapping_add(SVFSCANF_CTYPE));
        c.g[V0][0] = c.svf_space_bit(r32(c, V1));
        c.g[S0][0] = c.g[A0][0];
        if c.g[V0][0] != 0 {
            // White space in the format: skip white space in the input.
            c.cycles += 2;
            c.g[V0][0] = c.lw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R));
            loop {
                c.cycles += 2;
                if (c.g[V0][0] as i64) > 0 {
                    c.g[A0][0] = c.lw(r32(c, SVF_FILE));
                } else {
                    c.cycles += 4;
                    let failed = svf_refill(c, 0x0041_7ba8)?;
                    c.g[V0][0] = c.lw(sp + SVFSCANF_NASSIGNED);
                    if failed {
                        return Ok(SvfNext::Exit);
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(r32(c, SVF_FILE));
                }
                c.cycles += 6;
                c.g[V0][0] = c.lbu(r32(c, A0));
                c.svf_set(V0, r32(c, V0).wrapping_add(r32(c, S0)));
                c.g[V1][0] = c.svf_space_bit(r32(c, V0));
                if c.g[V1][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = 0x450000;
                    return Ok(svf_head(c));
                }
                c.g[V1][0] = c.lw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R));
                svf_back_edge(c, 0x0041_7b80);
                // Consume the space: p++, nread++, r--.
                c.cycles += 6;
                c.svf_set(V0, r32(c, A0).wrapping_add(1));
                c.sw(r32(c, SVF_FILE), c.g[V0][0]);
                c.svf_set(SVF_NREAD, r32(c, SVF_NREAD).wrapping_add(1));
                c.svf_set(V1, r32(c, V1).wrapping_sub(1));
                c.sw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R), c.g[V1][0]);
                c.g[V0][0] = c.lw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R));
            }
        }
        c.cycles += 1;
        c.g[V1][0] = c.lhu(sp + SVFSCANF_WC);
    }

    c.cycles += 3;
    c.g[V0][0] = 0x25;
    let literal = c.g[V1][0] != 0x25;
    c.g[V1][0] = c.lw(sp + SVFSCANF_FMT);
    if literal {
        return svf_literal(c);
    }

    // A conversion: flags and width.
    c.cycles += 2;
    c.g[SVF_WIDTH][0] = 0;
    c.g[SVF_FLAGS][0] = 0;
    let kind_target = loop {
        c.cycles += 7;
        c.g[V0][0] = c.lw(sp + SVFSCANF_FMT);
        c.g[S0][0] = c.lb(r32(c, V0));
        c.svf_set(V0, r32(c, V0).wrapping_add(1));
        c.sw(sp + SVFSCANF_FMT, c.g[V0][0]);
        let in_table = c.g[S0][0] < 0x79;
        c.g[V0][0] = 0x4a0000;
        let target = if in_table {
            c.cycles += 6;
            c.svf_set(V1, r32(c, S0) << 2);
            c.g[V0][0] = sx(SVFSCANF_SPEC_TABLE);
            c.svf_set(V1, r32(c, V1).wrapping_add(SVFSCANF_SPEC_TABLE));
            c.g[A0][0] = c.lw(r32(c, V1));
            r32(c, A0)
        } else {
            0x0041_7dd0
        };
        match target {
            // '%%': a literal '%'.
            0x0041_7c20 => {
                c.cycles += 1;
                c.g[V1][0] = c.lw(sp + SVFSCANF_FMT);
                return svf_literal(c);
            }
            // '*', 'l', 'L', 'h': flags.
            0x0041_7c8c | 0x0041_7c94 | 0x0041_7c9c | 0x0041_7ca4 => {
                c.cycles += 2;
                let bit = match target {
                    0x0041_7c8c => SVF_SUPPRESS,
                    0x0041_7c94 => SVF_LONG,
                    0x0041_7c9c => SVF_LONGDBL,
                    _ => SVF_SHORT,
                };
                c.g[SVF_FLAGS][0] |= bit;
                svf_back_edge(c, 0x0041_7bec);
            }
            // '0'..'9': width = width * 10 + c - '0'.
            0x0041_7cac => {
                c.cycles += 5;
                c.g[V0][0] = 0xa;
                c.mult(0, V1, SVF_WIDTH, V0);
                c.svf_set(V0, r32(c, V1).wrapping_add(r32(c, S0)));
                c.svf_set(SVF_WIDTH, r32(c, V0).wrapping_sub(0x30));
                svf_back_edge(c, 0x0041_7bec);
            }
            t => break t,
        }
    };

    // The conversion character: choose the kind (s0), base and converter.
    match kind_target {
        // 'D', 'd': long / int, decimal, strtol.
        0x0041_7cc0 | 0x0041_7cc4 => {
            if kind_target == 0x0041_7cc0 {
                c.cycles += 1;
                c.g[SVF_FLAGS][0] |= SVF_LONG;
            }
            c.cycles += 4;
            c.g[V0][0] = 0x420000;
            c.g[S0][0] = 0x3;
            c.g[V0][0] = sx(SVFSCANF_STRTOL);
            svf_base10(c, sp);
        }
        // 'i': strtol, base from the prefix.
        0x0041_7cd4 => {
            c.cycles += 6;
            c.g[V0][0] = 0x420000;
            c.g[S0][0] = 0x3;
            c.g[V0][0] = sx(SVFSCANF_STRTOL);
            c.g[SVF_BASE][0] = 0;
            c.sw(sp + SVFSCANF_CCFN, c.g[V0][0]);
        }
        // 'O', 'o': octal, strtoul.
        0x0041_7cec | 0x0041_7cf0 => {
            if kind_target == 0x0041_7cec {
                c.cycles += 1;
                c.g[SVF_FLAGS][0] |= SVF_LONG;
            }
            c.cycles += 6;
            c.g[V0][0] = 0x420000;
            c.g[S0][0] = 0x3;
            c.g[V0][0] = sx(SVFSCANF_STRTOUL);
            c.g[SVF_BASE][0] = 0x8;
            c.sw(sp + SVFSCANF_CCFN, c.g[V0][0]);
        }
        // 'u': decimal, strtoul.
        0x0041_7d08 => {
            c.cycles += 3;
            c.g[V0][0] = 0x420000;
            c.g[S0][0] = 0x3;
            c.g[V0][0] = sx(SVFSCANF_STRTOUL);
            svf_base10(c, sp);
        }
        // 'X', 'x' (PFXOK) and 'p' (POINTER | PFXOK): hex, strtoul.
        0x0041_7d20 | 0x0041_7d64 => {
            if kind_target == 0x0041_7d20 {
                c.cycles += 3;
                c.g[V0][0] = 0x420000;
                c.g[SVF_FLAGS][0] |= SVF_PFXOK;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x420000;
                c.g[SVF_FLAGS][0] |= SVF_PFXOK | SVF_POINTER;
            }
            c.cycles += 5;
            c.g[V0][0] = sx(SVFSCANF_STRTOUL);
            c.g[S0][0] = 0x3;
            c.sw(sp + SVFSCANF_CCFN, c.g[V0][0]);
            c.g[SVF_BASE][0] = 0x10;
        }
        // 'e', 'f', 'g', 'E', 'G': float.
        0x0041_7d2c => {
            c.cycles += 2;
            c.g[S0][0] = 0x4;
        }
        // 's': string.
        0x0041_7d34 => {
            c.cycles += 2;
            c.g[S0][0] = 0x2;
        }
        // '[': build the class table at sp+0, fmt = past the ']'.
        0x0041_7d3c => {
            c.cycles += 7;
            c.g[A1][0] = c.lw(sp + SVFSCANF_FMT);
            c.g[A0][0] = c.g[SP][0];
            c.g[SVF_FLAGS][0] |= SVF_NOSKIP;
            c.g[S0][0] = 0x1;
            svc::SCCL.call(c, 0x0041_7d50)?;
            c.sw(sp + SVFSCANF_FMT, c.g[V0][0]);
        }
        // 'c': characters, no space skip.
        0x0041_7d58 => {
            c.cycles += 3;
            c.g[SVF_FLAGS][0] |= SVF_NOSKIP;
            c.g[S0][0] = 0;
        }
        // 'n': store nread (unless suppressed).
        0x0041_7d80 => return Ok(svf_store_nread(c)),
        // '\0' right after '%': EOF.
        0x0041_7dc8 => {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
            return Ok(SvfNext::Exit);
        }
        // Anything else: a decimal int (long when upper case).
        0x0041_7dd0 => {
            c.cycles += 11;
            c.g[V0][0] = 0x4a0000;
            c.g[V1][0] = sx(SVFSCANF_STRTOL);
            c.svf_set(V0, r32(c, S0).wrapping_add(SVFSCANF_CTYPE));
            c.sw(sp + SVFSCANF_CCFN, c.g[V1][0]);
            c.g[V1][0] = c.lbu(r32(c, V0));
            c.g[SVF_BASE][0] = 0xa;
            c.g[S0][0] = 0x3;
            c.g[V1][0] &= 0x1;
            c.g[SVF_FLAGS][0] |= c.g[V1][0];
        }
        t => return Ok(svf_out(c, t)),
    }

    // Make sure there is input.
    c.cycles += 3;
    let have = (c.lw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R)) as i64) > 0;
    c.g[V0][0] = c.g[SVF_FLAGS][0] & SVF_NOSKIP;
    if !have {
        c.cycles += 4;
        let failed = svf_refill(c, 0x0041_7e10)?;
        c.g[V1][0] = c.lw(sp + SVFSCANF_NASSIGNED);
        if failed {
            return Ok(svf_input_failure(c));
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[SVF_FLAGS][0] & SVF_NOSKIP;
    }

    // Skip leading input white space unless NOSKIP.
    c.cycles += 2;
    let noskip = c.g[V0][0] != 0;
    c.g[V0][0] = (c.g[S0][0] < 5) as u64;
    if !noskip {
        c.cycles += 9;
        c.g[A0][0] = c.lw(r32(c, SVF_FILE));
        c.g[V0][0] = 0x4a0000;
        c.g[A1][0] = sx(SVFSCANF_CTYPE);
        c.g[V1][0] = c.lbu(r32(c, A0));
        c.svf_set(V1, r32(c, V1).wrapping_add(r32(c, A1)));
        c.g[V0][0] = c.svf_space_bit(r32(c, V1));
        let not_space = c.g[V0][0] == 0;
        c.g[V0][0] = (c.g[S0][0] < 5) as u64;
        if !not_space {
            c.cycles += 2;
            c.g[S1][0] = c.g[A1][0];
            c.g[V0][0] = c.lw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R));
            loop {
                c.cycles += 4;
                c.svf_set(SVF_NREAD, r32(c, SVF_NREAD).wrapping_add(1));
                c.svf_set(V0, r32(c, V0).wrapping_sub(1));
                let empty = (c.g[V0][0] as i64) <= 0;
                c.sw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R), c.g[V0][0]);
                if !empty {
                    c.cycles += 3;
                    c.svf_set(V0, r32(c, A0).wrapping_add(1));
                    c.sw(r32(c, SVF_FILE), c.g[V0][0]);
                } else {
                    c.cycles += 4;
                    let failed = svf_refill(c, 0x0041_7e74)?;
                    c.g[V1][0] = c.lw(sp + SVFSCANF_NASSIGNED);
                    if failed {
                        return Ok(svf_input_failure(c));
                    }
                }
                c.cycles += 7;
                c.g[A0][0] = c.lw(r32(c, SVF_FILE));
                c.g[V0][0] = c.lbu(r32(c, A0));
                c.svf_set(V0, r32(c, V0).wrapping_add(r32(c, S1)));
                c.g[V1][0] = c.svf_space_bit(r32(c, V0));
                if c.g[V1][0] == 0 {
                    break;
                }
                c.g[V0][0] = c.lw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R));
                svf_back_edge(c, 0x0041_7e50);
            }
            c.cycles += 1;
            c.g[V0][0] = (c.g[S0][0] < 5) as u64;
        }
    }

    c.cycles += 2;
    let bad_kind = c.g[V0][0] == 0;
    c.g[V0][0] = 0x450000;
    if bad_kind {
        return Ok(svf_head(c));
    }
    c.cycles += 7;
    c.g[V0][0] = 0x4a0000;
    c.svf_set(V1, r32(c, S0) << 2);
    c.g[V0][0] = sx(SVFSCANF_KIND_TABLE);
    c.svf_set(V1, r32(c, V1).wrapping_add(SVFSCANF_KIND_TABLE));
    c.g[A0][0] = c.lw(r32(c, V1));
    match r32(c, A0) {
        0x0041_7ec0 => svf_scan_chars(c),
        0x0041_7f64 => svf_scan_class(c),
        0x0041_8070 => svf_scan_string(c),
        0x0041_817c => svf_scan_int(c),
        0x0041_83b4 => svf_scan_float(c),
        t => Ok(svf_out(c, t)),
    }
}

/// 0x417d14: base 10, converter in v0 stored to CCFN.
fn svf_base10(c: &mut Cpu, sp: u32) {
    c.cycles += 3;
    c.g[SVF_BASE][0] = 0xa;
    c.sw(sp + SVFSCANF_CCFN, c.g[V0][0]);
}

/// 0x417c24: match `nbytes` literal format bytes (ending at fmt, in v1)
/// against the input.
fn svf_literal(c: &mut Cpu) -> Result<SvfNext, Unwound> {
    let sp = sp(c);
    c.cycles += 3;
    c.g[S0][0] = 0;
    let none = (c.g[S6][0] as i64) <= 0;
    c.svf_set(S1, r32(c, V1).wrapping_sub(r32(c, S6)));
    if none {
        return Ok(svf_top(c));
    }
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.lw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R));
        if (c.g[V0][0] as i64) > 0 {
            c.g[A0][0] = c.lw(r32(c, SVF_FILE));
        } else {
            c.cycles += 4;
            let failed = svf_refill(c, 0x0041_7c44)?;
            c.g[V1][0] = c.lw(sp + SVFSCANF_NASSIGNED);
            if failed {
                return Ok(svf_input_failure(c));
            }
            c.cycles += 1;
            c.g[A0][0] = c.lw(r32(c, SVF_FILE));
        }
        c.cycles += 4;
        c.g[V1][0] = c.lb(r32(c, S1));
        c.g[V0][0] = c.lbu(r32(c, A0));
        let mismatch = c.g[V0][0] != c.g[V1][0];
        c.svf_set(V1, r32(c, A0).wrapping_add(1));
        if mismatch {
            return Ok(svf_ret_count(c));
        }
        c.cycles += 9;
        c.g[V0][0] = c.lw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R));
        c.svf_set(S0, r32(c, S0).wrapping_add(1));
        c.sw(r32(c, SVF_FILE), c.g[V1][0]);
        c.svf_set(V0, r32(c, V0).wrapping_sub(1));
        c.svf_set(SVF_NREAD, r32(c, SVF_NREAD).wrapping_add(1));
        c.sw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R), c.g[V0][0]);
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S6][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.svf_set(S1, r32(c, S1).wrapping_add(1));
        if !more {
            break;
        }
        svf_back_edge(c, 0x0041_7c30);
    }
    c.cycles += 2;
    c.g[V0][0] = 0x450000;
    Ok(svf_head(c))
}

/// 0x417d80 `%n`: store nread as short / long long / int (nothing when
/// suppressed).
fn svf_store_nread(c: &mut Cpu) -> SvfNext {
    c.cycles += 3;
    let suppressed = c.g[SVF_FLAGS][0] & SVF_SUPPRESS != 0;
    c.g[V0][0] = 0x450000;
    if suppressed {
        return svf_head(c);
    }
    c.cycles += 3;
    let short = c.g[SVF_FLAGS][0] & SVF_SHORT != 0;
    c.g[V0][0] = c.g[SVF_FLAGS][0] & SVF_LONG;
    if short {
        c.cycles += 4;
        c.svf_set(SVF_AP, r32(c, SVF_AP).wrapping_add(8));
        c.g[A0][0] = c.lw(r32(c, SVF_AP).wrapping_sub(8));
        c.sh(r32(c, A0), c.g[SVF_NREAD][0]);
        return svf_top(c);
    }
    c.cycles += 2;
    let long = c.g[V0][0] != 0;
    c.svf_set(SVF_AP, r32(c, SVF_AP).wrapping_add(8));
    if long {
        c.cycles += 3;
        c.g[V0][0] = c.lw(r32(c, SVF_AP).wrapping_sub(8));
        c.sd(r32(c, V0), c.g[SVF_NREAD][0]);
    } else {
        c.cycles += 3;
        c.g[A0][0] = c.lw(r32(c, SVF_AP).wrapping_sub(8));
        c.sw(r32(c, A0), c.g[SVF_NREAD][0]);
    }
    svf_top(c)
}

/// 0x417ec0 CT_CHAR: `width` characters (default 1), skipped or `fread`.
fn svf_scan_chars(c: &mut Cpu) -> Result<SvfNext, Unwound> {
    let sp = sp(c);
    c.cycles += 4;
    c.g[V0][0] = 0x1;
    c.g[V1][0] = c.g[SVF_FLAGS][0] & SVF_SUPPRESS;
    let store = c.g[V1][0] == 0;
    if c.g[SVF_WIDTH][0] == 0 {
        c.g[SVF_WIDTH][0] = c.g[V0][0];
    }
    if store {
        c.cycles += 8;
        c.svf_set(SVF_AP, r32(c, SVF_AP).wrapping_add(8));
        c.g[A2][0] = c.g[SVF_WIDTH][0];
        c.g[A0][0] = c.lw(r32(c, SVF_AP).wrapping_sub(8));
        c.g[A1][0] = 0x1;
        c.g[A3][0] = c.g[SVF_FILE][0];
        svc::FREAD.call(c, 0x0041_7f4c)?;
        let none = c.g[V0][0] == 0;
        c.g[A0][0] = c.lw(sp + SVFSCANF_NASSIGNED);
        if none {
            return Ok(svf_fail_load(c));
        }
        c.cycles += 4;
        c.svf_set(SVF_NREAD, r32(c, SVF_NREAD).wrapping_add(r32(c, V0)));
        c.svf_set(A0, r32(c, A0).wrapping_add(1));
        c.sw(sp + SVFSCANF_NASSIGNED, c.g[A0][0]);
        return Ok(svf_top(c));
    }
    // Suppressed: skip whole buffers until width is reached.
    c.cycles += 2;
    c.g[S1][0] = 0;
    c.g[S0][0] = c.lw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R));
    loop {
        c.cycles += 3;
        c.g[V0][0] = (c.g[S0][0] < c.g[SVF_WIDTH][0]) as u64;
        let enough = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[SVF_FILE][0];
        if enough {
            c.cycles += 8;
            c.g[V1][0] = c.lw(r32(c, SVF_FILE));
            c.svf_set(V0, r32(c, S0).wrapping_sub(r32(c, SVF_WIDTH)));
            c.sw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R), c.g[V0][0]);
            c.svf_set(S1, r32(c, S1).wrapping_add(r32(c, SVF_WIDTH)));
            c.svf_set(V1, r32(c, V1).wrapping_add(r32(c, SVF_WIDTH)));
            c.sw(r32(c, SVF_FILE), c.g[V1][0]);
            c.svf_set(SVF_NREAD, r32(c, SVF_NREAD).wrapping_add(r32(c, S1)));
            return Ok(svf_top(c));
        }
        c.cycles += 8;
        c.g[V0][0] = c.lw(r32(c, SVF_FILE));
        c.svf_set(S1, r32(c, S1).wrapping_add(r32(c, S0)));
        c.svf_set(SVF_WIDTH, r32(c, SVF_WIDTH).wrapping_sub(r32(c, S0)));
        c.svf_set(V0, r32(c, V0).wrapping_add(r32(c, S0)));
        c.sw(r32(c, SVF_FILE), c.g[V0][0]);
        svc::SREFILL.call(c, 0x0041_7efc)?;
        if c.g[V0][0] != 0 {
            break;
        }
        c.g[S0][0] = c.lw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R));
        svf_back_edge(c, 0x0041_7ed8);
    }
    c.cycles += 2;
    let none = c.g[S1][0] == 0;
    c.svf_set(SVF_NREAD, r32(c, SVF_NREAD).wrapping_add(r32(c, S1)));
    if none {
        return Ok(svf_fail_load(c));
    }
    c.cycles += 2;
    c.g[V0][0] = 0x450000;
    Ok(svf_head(c))
}

/// 0x417f64 CT_CCL: characters in the `[` class (table at sp+0).
fn svf_scan_class(c: &mut Cpu) -> Result<SvfNext, Unwound> {
    let sp = sp(c);
    c.cycles += 5;
    c.g[V0][0] = u64::MAX;
    c.g[V1][0] = c.g[SVF_FLAGS][0] & SVF_SUPPRESS;
    let store = c.g[V1][0] == 0;
    if c.g[SVF_WIDTH][0] == 0 {
        c.g[SVF_WIDTH][0] = c.g[V0][0];
    }
    if !store {
        // Suppressed: count matching characters (n in s0).
        c.cycles += 2;
        c.g[S0][0] = 0;
        c.g[A1][0] = c.lw(r32(c, SVF_FILE));
        loop {
            c.cycles += 5;
            c.g[V0][0] = c.lbu(r32(c, A1));
            c.svf_set(V1, sp.wrapping_add(r32(c, V0)));
            c.g[A0][0] = c.lb(r32(c, V1));
            let out = c.g[A0][0] == 0;
            c.svf_set(V0, r32(c, A1).wrapping_add(1));
            if out {
                break;
            }
            c.cycles += 7;
            c.g[V1][0] = c.lw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R));
            c.sw(r32(c, SVF_FILE), c.g[V0][0]);
            c.svf_set(S0, r32(c, S0).wrapping_add(1));
            c.svf_set(V1, r32(c, V1).wrapping_sub(1));
            c.svf_set(SVF_WIDTH, r32(c, SVF_WIDTH).wrapping_sub(1));
            let full = c.g[SVF_WIDTH][0] == 0;
            c.sw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R), c.g[V1][0]);
            if full {
                break;
            }
            c.cycles += 2;
            if (c.g[V1][0] as i64) > 0 {
                c.g[A1][0] = c.lw(r32(c, SVF_FILE));
                svf_back_edge(c, 0x0041_7f80);
                continue;
            }
            c.cycles += 4;
            if !svf_refill(c, 0x0041_7fc0)? {
                c.g[A1][0] = c.lw(r32(c, SVF_FILE));
                svf_back_edge(c, 0x0041_7f80);
                continue;
            }
            c.cycles += 2;
            let none = c.g[S0][0] == 0;
            c.g[V1][0] = c.lw(sp + SVFSCANF_NASSIGNED);
            if none {
                return Ok(svf_input_failure(c));
            }
            break;
        }
        c.cycles += 2;
        let none = c.g[S0][0] == 0;
        c.svf_set(SVF_NREAD, r32(c, SVF_NREAD).wrapping_add(r32(c, S0)));
        if none {
            return Ok(svf_ret_count(c));
        }
        c.cycles += 2;
        c.g[V0][0] = 0x450000;
        return Ok(svf_head(c));
    }

    // Store: p (s1) runs from p0 (s0).
    c.cycles += 4;
    c.svf_set(SVF_AP, r32(c, SVF_AP).wrapping_add(8));
    c.g[S1][0] = c.lw(r32(c, SVF_AP).wrapping_sub(8));
    c.g[S0][0] = c.g[S1][0];
    c.g[A1][0] = c.lw(r32(c, SVF_FILE));
    let mismatch = loop {
        c.cycles += 5;
        c.g[V0][0] = c.lbu(r32(c, A1));
        c.svf_set(V1, sp.wrapping_add(r32(c, V0)));
        c.g[A0][0] = c.lb(r32(c, V1));
        if c.g[A0][0] == 0 {
            break true;
        }
        c.cycles += 10;
        c.g[V0][0] = c.lw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R));
        c.svf_set(A0, r32(c, A1).wrapping_add(1));
        c.svf_set(SVF_WIDTH, r32(c, SVF_WIDTH).wrapping_sub(1));
        c.svf_set(V0, r32(c, V0).wrapping_sub(1));
        c.sw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R), c.g[V0][0]);
        c.g[V1][0] = c.lbu(r32(c, A1));
        c.sb(r32(c, S1), c.g[V1][0]);
        c.sw(r32(c, SVF_FILE), c.g[A0][0]);
        let full = c.g[SVF_WIDTH][0] == 0;
        c.svf_set(S1, r32(c, S1).wrapping_add(1));
        if full {
            break false;
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R));
        if (c.g[V0][0] as i64) > 0 {
            c.g[A1][0] = c.lw(r32(c, SVF_FILE));
            svf_back_edge(c, 0x0041_7ff0);
            continue;
        }
        c.cycles += 4;
        if !svf_refill(c, 0x0041_8040)? {
            c.g[A1][0] = c.lw(r32(c, SVF_FILE));
            svf_back_edge(c, 0x0041_7ff0);
            continue;
        }
        c.cycles += 2;
        let none = c.g[S1][0] == c.g[S0][0];
        c.g[V1][0] = c.lw(sp + SVFSCANF_NASSIGNED);
        if none {
            return Ok(svf_input_failure(c));
        }
        break false;
    };
    if !mismatch {
        c.cycles += 1;
    }
    c.svf_set(S0, r32(c, S1).wrapping_sub(r32(c, S0)));
    c.cycles += 2;
    let none = c.g[S0][0] == 0;
    c.g[V0][0] = c.lw(sp + SVFSCANF_NASSIGNED);
    if none {
        return Ok(SvfNext::Exit);
    }
    c.cycles += 5;
    c.sb(r32(c, S1), 0);
    c.g[V0][0] = c.lw(sp + SVFSCANF_NASSIGNED);
    c.svf_set(V0, r32(c, V0).wrapping_add(1));
    c.sw(sp + SVFSCANF_NASSIGNED, c.g[V0][0]);
    c.cycles += 2;
    c.svf_set(SVF_NREAD, r32(c, SVF_NREAD).wrapping_add(r32(c, S0)));
    Ok(svf_top(c))
}

/// 0x418070 CT_STRING: non-space characters, NUL-terminated when stored.
fn svf_scan_string(c: &mut Cpu) -> Result<SvfNext, Unwound> {
    let sp = sp(c);
    c.cycles += 5;
    c.g[V0][0] = u64::MAX;
    c.g[V1][0] = c.g[SVF_FLAGS][0] & SVF_SUPPRESS;
    let store = c.g[V1][0] == 0;
    if c.g[SVF_WIDTH][0] == 0 {
        c.g[SVF_WIDTH][0] = c.g[V0][0];
    }
    if !store {
        c.cycles += 5;
        c.g[V0][0] = 0x4a0000;
        c.g[S0][0] = 0;
        c.g[S1][0] = sx(SVFSCANF_CTYPE);
        c.g[A0][0] = c.lw(r32(c, SVF_FILE));
        loop {
            c.cycles += 6;
            c.g[V0][0] = c.lbu(r32(c, A0));
            c.svf_set(V0, r32(c, V0).wrapping_add(r32(c, S1)));
            c.g[V1][0] = c.svf_space_bit(r32(c, V0));
            let space = c.g[V1][0] != 0;
            c.svf_set(V0, r32(c, A0).wrapping_add(1));
            if space {
                break;
            }
            c.cycles += 7;
            c.g[V1][0] = c.lw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R));
            c.sw(r32(c, SVF_FILE), c.g[V0][0]);
            c.svf_set(S0, r32(c, S0).wrapping_add(1));
            c.svf_set(V1, r32(c, V1).wrapping_sub(1));
            c.svf_set(SVF_WIDTH, r32(c, SVF_WIDTH).wrapping_sub(1));
            let full = c.g[SVF_WIDTH][0] == 0;
            c.sw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R), c.g[V1][0]);
            if full {
                break;
            }
            c.cycles += 2;
            if (c.g[V1][0] as i64) > 0 {
                c.g[A0][0] = c.lw(r32(c, SVF_FILE));
                svf_back_edge(c, 0x0041_8098);
                continue;
            }
            c.cycles += 4;
            if !svf_refill(c, 0x0041_80dc)? {
                c.g[A0][0] = c.lw(r32(c, SVF_FILE));
                svf_back_edge(c, 0x0041_8098);
                continue;
            }
            break;
        }
        c.cycles += 2;
        c.svf_set(SVF_NREAD, r32(c, SVF_NREAD).wrapping_add(r32(c, S0)));
        return Ok(svf_top(c));
    }

    // Store: s3 now holds nassigned + 1 and s6 the ctype table.
    c.cycles += 9;
    c.svf_set(SVF_AP, r32(c, SVF_AP).wrapping_add(8));
    c.g[V1][0] = c.lw(sp + SVFSCANF_NASSIGNED);
    c.g[S1][0] = c.lw(r32(c, SVF_AP).wrapping_sub(8));
    c.g[V0][0] = 0x4a0000;
    c.g[S6][0] = sx(SVFSCANF_CTYPE);
    c.svf_set(S3, r32(c, V1).wrapping_add(1));
    c.g[S0][0] = c.g[S1][0];
    c.g[A1][0] = c.lw(r32(c, SVF_FILE));
    loop {
        c.cycles += 6;
        c.g[V0][0] = c.lbu(r32(c, A1));
        c.svf_set(V0, r32(c, V0).wrapping_add(r32(c, S6)));
        c.g[V1][0] = c.svf_space_bit(r32(c, V0));
        let space = c.g[V1][0] != 0;
        c.svf_set(A0, r32(c, A1).wrapping_add(1));
        if space {
            break;
        }
        c.cycles += 9;
        c.g[V0][0] = c.lw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R));
        c.svf_set(SVF_WIDTH, r32(c, SVF_WIDTH).wrapping_sub(1));
        c.svf_set(V0, r32(c, V0).wrapping_sub(1));
        c.sw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R), c.g[V0][0]);
        c.g[V1][0] = c.lbu(r32(c, A1));
        c.sb(r32(c, S1), c.g[V1][0]);
        c.sw(r32(c, SVF_FILE), c.g[A0][0]);
        let full = c.g[SVF_WIDTH][0] == 0;
        c.svf_set(S1, r32(c, S1).wrapping_add(1));
        if full {
            break;
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R));
        if (c.g[V0][0] as i64) > 0 {
            c.g[A1][0] = c.lw(r32(c, SVF_FILE));
            svf_back_edge(c, 0x0041_8110);
            continue;
        }
        c.cycles += 4;
        if !svf_refill(c, 0x0041_8160)? {
            c.g[A1][0] = c.lw(r32(c, SVF_FILE));
            svf_back_edge(c, 0x0041_8110);
            continue;
        }
        break;
    }
    c.cycles += 5;
    c.svf_set(V0, r32(c, S1).wrapping_sub(r32(c, S0)));
    c.sw(sp + SVFSCANF_NASSIGNED, c.g[S3][0]);
    c.svf_set(SVF_NREAD, r32(c, SVF_NREAD).wrapping_add(r32(c, V0)));
    c.sb(r32(c, S1), 0);
    Ok(svf_top(c))
}

/// Number scans (0x41817c, 0x4183b4): width = min(width - 1, 0x15c) + 1
/// (unsigned), the buffer at sp+0x100 in s1/s6, `set` added to the flags.
/// False when the width is 0 (nothing to scan).
fn svf_number_start(c: &mut Cpu, set: u64) -> bool {
    let sp = sp(c);
    c.cycles += 9;
    c.svf_set(SVF_WIDTH, r32(c, SVF_WIDTH).wrapping_sub(1));
    c.g[V1][0] = 0x15c;
    c.g[V0][0] = (c.g[SVF_WIDTH][0] < 0x15d) as u64;
    c.svf_set(S1, sp.wrapping_add(SVFSCANF_BUF));
    if c.g[V0][0] == 0 {
        c.g[SVF_WIDTH][0] = c.g[V1][0];
    }
    c.g[SVF_FLAGS][0] |= set;
    c.svf_set(SVF_WIDTH, r32(c, SVF_WIDTH).wrapping_add(1));
    c.g[S6][0] = c.g[S1][0];
    c.g[SVF_WIDTH][0] != 0
}

/// Store the accepted character (s0) and advance the input (0x418284 /
/// 0x418458). True when `__srefill` failed (v0 = flags & NDIGITS then).
fn svf_number_take(c: &mut Cpu, ret: u32) -> Result<bool, Unwound> {
    c.cycles += 6;
    c.sb(r32(c, S1), c.g[S0][0]);
    c.svf_set(S1, r32(c, S1).wrapping_add(1));
    c.g[V0][0] = c.lw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R));
    c.svf_set(V0, r32(c, V0).wrapping_sub(1));
    let empty = (c.g[V0][0] as i64) <= 0;
    c.sw(r32(c, SVF_FILE).wrapping_add(SVFSCANF_FILE_R), c.g[V0][0]);
    if !empty {
        c.cycles += 4;
        c.g[V0][0] = c.lw(r32(c, SVF_FILE));
        c.svf_set(V0, r32(c, V0).wrapping_add(1));
        c.sw(r32(c, SVF_FILE), c.g[V0][0]);
        return Ok(false);
    }
    c.cycles += 4;
    let failed = svf_refill(c, ret)?;
    c.g[V0][0] = c.g[SVF_FLAGS][0] & SVF_NDIGITS;
    Ok(failed)
}

/// How a number scan's character loop ended.
enum SvfStop {
    /// A character that does not fit, or the width ran out (v0 not set).
    Char,
    /// `__srefill` failed (v0 = flags & NDIGITS already).
    Eof,
    Out,
}

/// 0x41817c CT_INT: collect an integer into the buffer, then convert it with
/// the `CCFN` converter and store it.
fn svf_scan_int(c: &mut Cpu) -> Result<SvfNext, Unwound> {
    let sp = sp(c);
    let stop = if !svf_number_start(c, SVF_SIGNOK | SVF_NDIGITS | SVF_NZDIGITS) {
        SvfStop::Char
    } else {
        c.cycles += 2;
        c.g[V0][0] = c.lw(r32(c, SVF_FILE));
        loop {
            c.cycles += 5;
            c.g[S0][0] = c.lbu(r32(c, V0));
            c.svf_set(V1, r32(c, S0).wrapping_sub(0x2b));
            c.g[V0][0] = (c.g[V1][0] < 0x4e) as u64;
            let outside = c.g[V0][0] == 0;
            c.g[V0][0] = 0x4a0000;
            if outside {
                break SvfStop::Char;
            }
            c.cycles += 6;
            c.svf_set(V1, r32(c, V1) << 2);
            c.g[V0][0] = sx(SVFSCANF_INT_TABLE);
            c.svf_set(V1, r32(c, V1).wrapping_add(SVFSCANF_INT_TABLE));
            c.g[A0][0] = c.lw(r32(c, V1));
            // Each arm either ends the scan or leaves the mask to clear in v0
            // (`Some(true)`: still to apply at 0x418280).
            let and_v0 = match r32(c, A0) {
                // '0': base 0 becomes octal with an 0x prefix allowed.
                0x0041_81d4 => {
                    c.cycles += 2;
                    let based = c.g[SVF_BASE][0] != 0;
                    c.g[V0][0] = c.g[SVF_FLAGS][0] & SVF_NZDIGITS;
                    if !based {
                        c.cycles += 3;
                        c.g[SVF_BASE][0] = 0x8;
                        c.g[SVF_FLAGS][0] |= SVF_PFXOK;
                        c.g[V0][0] = c.g[SVF_FLAGS][0] & SVF_NZDIGITS;
                    }
                    c.cycles += 2;
                    let nz = c.g[V0][0] != 0;
                    c.g[V0][0] = !(SVF_SIGNOK | SVF_NDIGITS | SVF_NZDIGITS);
                    if !nz {
                        c.cycles += 2;
                        c.g[V0][0] = !(SVF_SIGNOK | SVF_PFXOK | SVF_NDIGITS);
                    }
                    true
                }
                // '1'..'7': base = basefix[base].
                0x0041_81f8 => {
                    c.cycles += 8;
                    c.g[V0][0] = 0x4a0000;
                    c.svf_set(V1, r32(c, SVF_BASE) << 1);
                    c.g[V0][0] = sx(SVFSCANF_BASEFIX);
                    c.g[A0][0] = !(SVF_SIGNOK | SVF_PFXOK | SVF_NDIGITS);
                    c.svf_set(V1, r32(c, V1).wrapping_add(SVFSCANF_BASEFIX));
                    c.g[SVF_FLAGS][0] &= c.g[A0][0];
                    c.g[SVF_BASE][0] = c.lh(r32(c, V1));
                    false
                }
                // '8', '9': base = basefix[base], only if above octal.
                0x0041_8218 => {
                    c.cycles += 8;
                    c.g[V0][0] = 0x4a0000;
                    c.svf_set(V1, r32(c, SVF_BASE) << 1);
                    c.g[V0][0] = sx(SVFSCANF_BASEFIX);
                    c.svf_set(V1, r32(c, V1).wrapping_add(SVFSCANF_BASEFIX));
                    c.g[SVF_BASE][0] = c.lh(r32(c, V1));
                    c.g[V0][0] = ((c.g[SVF_BASE][0] as i64) < 9) as u64;
                    let bad = c.g[V0][0] != 0;
                    c.g[V0][0] = !(SVF_SIGNOK | SVF_PFXOK | SVF_NDIGITS);
                    if bad {
                        break SvfStop::Char;
                    }
                    c.cycles += 2;
                    c.g[SVF_FLAGS][0] &= c.g[V0][0];
                    false
                }
                // Hex letters: only in base 16.
                0x0041_8240 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[SVF_BASE][0] as i64) < 11) as u64;
                    let bad = c.g[V0][0] != 0;
                    c.g[V0][0] = !(SVF_SIGNOK | SVF_PFXOK | SVF_NDIGITS);
                    if bad {
                        break SvfStop::Char;
                    }
                    c.cycles += 2;
                    c.g[SVF_FLAGS][0] &= c.g[V0][0];
                    false
                }
                // '+', '-': only first.
                0x0041_8254 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[SVF_FLAGS][0] & SVF_SIGNOK;
                    let bad = c.g[V0][0] == 0;
                    c.g[V0][0] = !SVF_SIGNOK;
                    if bad {
                        break SvfStop::Char;
                    }
                    c.cycles += 2;
                    c.g[SVF_FLAGS][0] &= c.g[V0][0];
                    false
                }
                // 'x', 'X': only right after a leading "0".
                0x0041_8268 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[SVF_FLAGS][0] & SVF_PFXOK;
                    let bad = c.g[V0][0] == 0;
                    c.svf_set(V0, sp.wrapping_add(SVFSCANF_BUF + 1));
                    if bad {
                        break SvfStop::Char;
                    }
                    c.cycles += 2;
                    let bad = c.g[S1][0] != c.g[V0][0];
                    c.g[V0][0] = !SVF_PFXOK;
                    if bad {
                        break SvfStop::Char;
                    }
                    c.cycles += 1;
                    c.g[SVF_BASE][0] = 0x10;
                    true
                }
                0x0041_82c8 => break SvfStop::Char,
                t => {
                    svf_out(c, t);
                    break SvfStop::Out;
                }
            };
            if and_v0 {
                c.cycles += 1;
                c.g[SVF_FLAGS][0] &= c.g[V0][0];
            }
            if svf_number_take(c, 0x0041_82b4)? {
                break SvfStop::Eof;
            }
            c.cycles += 3;
            c.svf_set(SVF_WIDTH, r32(c, SVF_WIDTH).wrapping_sub(1));
            if c.g[SVF_WIDTH][0] == 0 {
                break SvfStop::Char;
            }
            c.g[V0][0] = c.lw(r32(c, SVF_FILE));
            svf_back_edge(c, 0x0041_81a8);
        }
    };
    match stop {
        SvfStop::Out => return Ok(SvfNext::Out),
        SvfStop::Char => {
            c.cycles += 1;
            c.g[V0][0] = c.g[SVF_FLAGS][0] & SVF_NDIGITS;
        }
        SvfStop::Eof => {}
    }

    c.cycles += 2;
    let no_digits = c.g[V0][0] != 0;
    c.g[V0][0] = (c.g[S6][0] < c.g[S1][0]) as u64;
    if no_digits {
        // No digits: push back the last character (a sign) and stop.
        c.cycles += 2;
        let any = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(sp + SVFSCANF_NASSIGNED);
        if !any {
            return Ok(SvfNext::Exit);
        }
        c.cycles += 5;
        c.g[A0][0] = c.lb(r32(c, S1).wrapping_sub(1));
        c.g[A1][0] = c.g[SVF_FILE][0];
        svc::UNGETC.call(c, 0x0041_82e8)?;
        c.g[V0][0] = c.lw(sp + SVFSCANF_NASSIGNED);
        return Ok(SvfNext::Exit);
    }

    // A trailing "x" of a lone "0x" goes back to the input.
    c.cycles += 4;
    c.g[S0][0] = c.lb(r32(c, S1).wrapping_sub(1));
    c.g[V0][0] = 0x78;
    let mut unget = c.g[S0][0] == 0x78;
    c.g[V0][0] = 0x58;
    if !unget {
        c.cycles += 2;
        unget = c.g[S0][0] == 0x58;
        c.g[V0][0] = c.g[SVF_FLAGS][0] & SVF_SUPPRESS;
    }
    if unget {
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        c.svf_set(S1, r32(c, S1).wrapping_sub(1));
        c.g[A1][0] = c.g[SVF_FILE][0];
        svc::UNGETC.call(c, 0x0041_8318)?;
        c.g[V0][0] = c.g[SVF_FLAGS][0] & SVF_SUPPRESS;
    }

    c.cycles += 2;
    let suppressed = c.g[V0][0] != 0;
    c.svf_set(V0, r32(c, S1).wrapping_sub(r32(c, S6)));
    if suppressed {
        return Ok(svf_add_nread_top(c));
    }

    // res = (*ccfn)(buf, NULL, base), stored as pointer / short / long long
    // / int.
    c.cycles += 10;
    c.sb(r32(c, S1), 0);
    c.g[A0][0] = c.g[S6][0];
    c.g[A1][0] = 0;
    c.g[V1][0] = c.lw(sp + SVFSCANF_CCFN);
    let ccfn = r32(c, V1);
    c.g[A2][0] = c.g[SVF_BASE][0];
    call_ptr(c, ccfn, 0x0041_833c)?;
    c.g[V1][0] = c.g[V0][0];
    let pointer = c.g[SVF_FLAGS][0] & SVF_POINTER != 0;
    c.g[V0][0] = c.g[SVF_FLAGS][0] & SVF_SHORT;
    if pointer {
        c.cycles += 6;
        c.svf_set(SVF_AP, r32(c, SVF_AP).wrapping_add(8));
        c.g[V1][0] = sx(r32(c, V1));
        c.g[V0][0] = c.lw(r32(c, SVF_AP).wrapping_sub(8));
        c.sw(r32(c, V0), c.g[V1][0]);
    } else {
        c.cycles += 2;
        let short = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0] & 0xffff;
        if short {
            c.cycles += 4;
            c.svf_set(SVF_AP, r32(c, SVF_AP).wrapping_add(8));
            c.g[A0][0] = c.lw(r32(c, SVF_AP).wrapping_sub(8));
            c.sh(r32(c, A0), c.g[V0][0]);
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.g[SVF_FLAGS][0] & SVF_LONG;
            let long = c.g[V0][0] != 0;
            c.svf_set(SVF_AP, r32(c, SVF_AP).wrapping_add(8));
            if long {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, SVF_AP).wrapping_sub(8));
                c.sd(r32(c, V0), c.g[V1][0]);
            } else {
                c.cycles += 4;
                c.g[V0][0] = sx(r32(c, V1));
                c.g[A0][0] = c.lw(r32(c, SVF_AP).wrapping_sub(8));
                c.sw(r32(c, A0), c.g[V0][0]);
            }
        }
    }
    c.cycles += 4;
    c.g[A0][0] = c.lw(sp + SVFSCANF_NASSIGNED);
    c.svf_set(A0, r32(c, A0).wrapping_add(1));
    c.sw(sp + SVFSCANF_NASSIGNED, c.g[A0][0]);
    Ok(svf_buf_len_top(c))
}

/// 0x41857c: v0 = buffer length, then 0x418580.
fn svf_buf_len_top(c: &mut Cpu) -> SvfNext {
    c.cycles += 1;
    c.svf_set(V0, r32(c, S1).wrapping_sub(r32(c, S6)));
    svf_add_nread_top(c)
}

/// 0x418580: nread += v0, next directive.
fn svf_add_nread_top(c: &mut Cpu) -> SvfNext {
    c.cycles += 2;
    c.svf_set(SVF_NREAD, r32(c, SVF_NREAD).wrapping_add(r32(c, V0)));
    svf_top(c)
}

/// 0x4183b4 CT_FLOAT: collect a float into the buffer, then `atof` it and
/// store a double (`l`/`L`) or a float.
fn svf_scan_float(c: &mut Cpu) -> Result<SvfNext, Unwound> {
    let sp = sp(c);
    // Float flags reuse the int bits: 0x100 is DPTOK, 0x200 EXPOK.
    const DPTOK: u64 = SVF_PFXOK;
    const EXPOK: u64 = SVF_NZDIGITS;
    let stop = if !svf_number_start(c, SVF_SIGNOK | SVF_NDIGITS | DPTOK | EXPOK) {
        SvfStop::Char
    } else {
        c.cycles += 2;
        c.g[V0][0] = c.lw(r32(c, SVF_FILE));
        loop {
            c.cycles += 5;
            c.g[S0][0] = c.lbu(r32(c, V0));
            c.svf_set(V1, r32(c, S0).wrapping_sub(0x2b));
            c.g[V0][0] = (c.g[V1][0] < 0x3b) as u64;
            let outside = c.g[V0][0] == 0;
            c.g[V0][0] = 0x4a0000;
            if outside {
                break SvfStop::Char;
            }
            c.cycles += 6;
            c.svf_set(V1, r32(c, V1) << 2);
            c.g[V0][0] = sx(SVFSCANF_FLOAT_TABLE);
            c.svf_set(V1, r32(c, V1).wrapping_add(SVFSCANF_FLOAT_TABLE));
            c.g[A0][0] = c.lw(r32(c, V1));
            match r32(c, A0) {
                // Digits.
                0x0041_840c => {
                    c.cycles += 3;
                    c.g[V0][0] = !(SVF_SIGNOK | SVF_NDIGITS);
                    c.g[SVF_FLAGS][0] &= c.g[V0][0];
                }
                // '+', '-': at the start or right after the exponent.
                0x0041_8418 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[SVF_FLAGS][0] & SVF_SIGNOK;
                    let bad = c.g[V0][0] == 0;
                    c.g[V0][0] = !SVF_SIGNOK;
                    if bad {
                        break SvfStop::Char;
                    }
                    c.cycles += 2;
                    c.g[SVF_FLAGS][0] &= c.g[V0][0];
                }
                // '.': once, before the exponent.
                0x0041_842c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[SVF_FLAGS][0] & DPTOK;
                    let bad = c.g[V0][0] == 0;
                    c.g[V0][0] = !(SVF_SIGNOK | DPTOK);
                    if bad {
                        break SvfStop::Char;
                    }
                    c.cycles += 2;
                    c.g[SVF_FLAGS][0] &= c.g[V0][0];
                }
                // 'e', 'E': once, after a digit.
                0x0041_8440 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.g[SVF_FLAGS][0] & (EXPOK | SVF_NDIGITS);
                    c.g[V0][0] = EXPOK;
                    let bad = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = !(EXPOK | DPTOK);
                    if bad {
                        break SvfStop::Char;
                    }
                    c.cycles += 2;
                    c.g[V0][0] &= c.g[SVF_FLAGS][0];
                    c.g[SVF_FLAGS][0] = c.g[V0][0] | SVF_SIGNOK | SVF_NDIGITS;
                }
                0x0041_849c => break SvfStop::Char,
                t => {
                    svf_out(c, t);
                    break SvfStop::Out;
                }
            }
            if svf_number_take(c, 0x0041_8488)? {
                break SvfStop::Eof;
            }
            c.cycles += 3;
            c.svf_set(SVF_WIDTH, r32(c, SVF_WIDTH).wrapping_sub(1));
            if c.g[SVF_WIDTH][0] == 0 {
                break SvfStop::Char;
            }
            c.g[V0][0] = c.lw(r32(c, SVF_FILE));
            svf_back_edge(c, 0x0041_83e0);
        }
    };
    match stop {
        SvfStop::Out => return Ok(SvfNext::Out),
        SvfStop::Char => {
            c.cycles += 1;
            c.g[V0][0] = c.g[SVF_FLAGS][0] & SVF_NDIGITS;
        }
        SvfStop::Eof => {}
    }

    c.cycles += 2;
    let no_digits = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[SVF_FLAGS][0] & EXPOK;
    if no_digits {
        c.cycles += 2;
        let in_mantissa = c.g[V0][0] != 0;
        c.g[V0][0] = (c.g[S6][0] < c.g[S1][0]) as u64;
        if in_mantissa {
            // No digits at all: push everything back and stop.
            c.cycles += 2;
            let any = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw(sp + SVFSCANF_NASSIGNED);
            if !any {
                return Ok(SvfNext::Exit);
            }
            c.cycles += 2;
            c.svf_set(S1, r32(c, S1).wrapping_sub(1));
            loop {
                c.cycles += 7;
                c.g[A1][0] = c.g[SVF_FILE][0];
                c.g[A0][0] = c.lb(r32(c, S1));
                svc::UNGETC.call(c, 0x0041_84cc)?;
                c.g[V1][0] = (c.g[S6][0] < c.g[S1][0]) as u64;
                let more = c.g[V1][0] != 0;
                c.svf_set(S1, r32(c, S1).wrapping_sub(1));
                if !more {
                    break;
                }
                svf_back_edge(c, 0x0041_84c0);
            }
            c.cycles += 2;
            c.g[V0][0] = c.lw(sp + SVFSCANF_NASSIGNED);
            return Ok(SvfNext::Exit);
        }
        // An exponent without digits: push back its sign and the 'e'.
        c.cycles += 5;
        c.svf_set(S1, r32(c, S1).wrapping_sub(1));
        c.g[V0][0] = 0x65;
        c.g[S0][0] = c.lb(r32(c, S1));
        let is_e = c.g[S0][0] == 0x65;
        c.g[V0][0] = 0x45;
        if !is_e {
            c.cycles += 2;
            let is_e = c.g[S0][0] == 0x45;
            c.g[A0][0] = c.g[S0][0];
            if !is_e {
                c.cycles += 4;
                c.g[A1][0] = c.g[SVF_FILE][0];
                svc::UNGETC.call(c, 0x0041_8508)?;
                c.svf_set(S1, r32(c, S1).wrapping_sub(1));
                c.g[S0][0] = c.lb(r32(c, S1));
            }
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SVF_FILE][0];
        svc::UNGETC.call(c, 0x0041_851c)?;
    }

    c.cycles += 3;
    let suppressed = c.g[SVF_FLAGS][0] & SVF_SUPPRESS != 0;
    c.svf_set(V0, r32(c, S1).wrapping_sub(r32(c, S6)));
    if suppressed {
        return Ok(svf_add_nread_top(c));
    }

    c.cycles += 7;
    c.sb(r32(c, S1), 0);
    c.g[A0][0] = c.g[S6][0];
    svc::ATOF.call(c, 0x0041_8534)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = c.g[SVF_FLAGS][0] & SVF_LONG;
    let double = if c.g[V0][0] != 0 {
        c.svf_set(SVF_AP, r32(c, SVF_AP).wrapping_add(8));
        true
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.g[SVF_FLAGS][0] & SVF_LONGDBL;
        let long_double = c.g[V0][0] != 0;
        if long_double {
            c.cycles += 1;
            c.svf_set(SVF_AP, r32(c, SVF_AP).wrapping_add(8));
        }
        long_double
    };
    if double {
        c.cycles += 3;
        c.g[V0][0] = c.lw(r32(c, SVF_AP).wrapping_sub(8));
        c.sd(r32(c, V0), c.g[A0][0]);
    } else {
        c.cycles += 4;
        c.svf_set(SVF_AP, r32(c, SVF_AP).wrapping_add(8));
        svc::TRUNCDFSF2.call(c, 0x0041_8568)?;
        c.g[S0][0] = c.lw(r32(c, SVF_AP).wrapping_sub(8));
        c.swc1(0, r32(c, S0));
    }
    c.cycles += 3;
    c.g[V0][0] = c.lw(sp + SVFSCANF_NASSIGNED);
    c.svf_set(V0, r32(c, V0).wrapping_add(1));
    c.sw(sp + SVFSCANF_NASSIGNED, c.g[V0][0]);
    Ok(svf_buf_len_top(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::{check_lohi, elf_image, fill, scramble, Rng, DATA, GAME_GP};

    /// A string `FILE` as `sscanf` builds it, the input text, the format,
    /// the `va_list` slots, the outputs and a reent with `__sdidinit` set.
    const SVFSCANF_T_FILE: u32 = DATA;
    const SVFSCANF_T_INPUT: u32 = DATA + 0x1000;
    const SVFSCANF_T_FMT: u32 = DATA + 0x2000;
    const SVFSCANF_T_AP: u32 = DATA + 0x3000;
    const SVFSCANF_T_OUT: u32 = DATA + 0x4000;
    const SVFSCANF_T_REENT: u32 = DATA + 0x6000;
    /// `eofread`, the string FILE's read function.
    const SVFSCANF_T_EOFREAD: u32 = 0x0041_78a8;

    /// (format, input) pairs reaching every directive, conversion and
    /// failure path.
    const SVFSCANF_CASES: &[(&[u8], &[u8])] = &[
        (b"%d", b"  123 x"),
        (b"%i", b"0x1fz"),
        (b"%i", b"017 "),
        (b"%i", b"0xg"),
        (b"%o %O", b"777 12"),
        (b"%o", b"89"),
        (b"%u", b"+42"),
        (b"%x %X", b"dead BEEF"),
        (b"%p", b"0x1234"),
        (b"%hd %ld %D", b"-7 99 123"),
        (b"%*d %d", b"5 6"),
        (b"%3d%d", b"12345"),
        (b"%999d", b"8"),
        (b"%d", b"-z"),
        (b"%d", b"z"),
        (b"%d", b"+"),
        (b"%x", b"0x"),
        (b"%d", b"12-3"),
        (b"%d", b"9a"),
        (b"%i", b"-0x10"),
        (b"%i", b"0"),
        (b"%x", b"0X1A"),
        (b"%4i", b"0x7fff"),
        (b"%lu", b"4294967295"),
        (b"%f", b"3.25"),
        (b"%lf %Lf", b"-1.5e2 +.5"),
        (b"%e", b"1e"),
        (b"%g", b"1e+"),
        (b"%f", b"+."),
        (b"%*f %f", b"1.0 2.5"),
        (b"%f", b"."),
        (b"%E", b"2E5x"),
        (b"%f", b"1.2.3"),
        (b"%e", b"e5"),
        (b"%f", b"-"),
        (b"%G", b"1e5"),
        (b"%5f", b"123.456"),
        (b"%s", b"  hello world"),
        (b"%3s%s", b"abcdef"),
        (b"%*s %s", b"skip keep"),
        (b"%*s", b"tail"),
        (b"%[abc]", b"abcabd"),
        (b"%[^,],%s", b"foo,bar"),
        (b"%*[0-9]%d", b"123 45"),
        (b"%[a]", b"b"),
        (b"%*[a]", b"b"),
        (b"%2[a-z]", b"abc"),
        (b"%*2[a-z]%c", b"abc"),
        (b"%[a-z]", b"abc"),
        (b"%c", b"x"),
        (b"%3c", b"abcd"),
        (b"%*2c%c", b"abc"),
        (b"%*5c", b"ab"),
        (b"%c", b""),
        (b"%d%n%hn%ln%*n", b"12"),
        (b"abc%d", b"abc5"),
        (b"abc%d", b"abd5"),
        (b"ab", b"a"),
        (b"%%%d", b"%7"),
        (b"%", b"1"),
        (b"%q", b"33"),
        (b"%Q", b"44"),
        (b"%\xb5", b"5"),
        (b" %d", b"\t\n 9"),
        (b" ", b"   "),
        (b"%d %d", b"1"),
        (b"%d", b""),
        (b"%s", b"   "),
        (b"%d %s %c %f %x", b"12 ab c 1.5 ff"),
        (b"%hu,%lx;%5c", b"65535,ffff;12345"),
    ];

    /// `__svfscanf` on a string FILE (real callees: `_mbtowc_r`,
    /// `__srefill`/`eofread`, `__sccl`, `fread`, `ungetc`, `strtol`,
    /// `strtoul`, `atof`, `__truncdfsf2`). Later seeds cut the input short
    /// at a random point to reach the EOF paths.
    #[test]
    fn svfscanf_matches() {
        check_lohi(
            ssx3_ee::s15::f_00417ab8__generated,
            svfscanf,
            300,
            &|c, seed| {
                let mut r = Rng(0x5cf5_ca4f ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                elf_image(c);
                let n = SVFSCANF_CASES.len();
                let (fmt, input) = SVFSCANF_CASES[seed as usize % n];
                let len = if seed as usize >= n && seed % 2 == 1 {
                    r.next_u32() as usize % (input.len() + 1)
                } else {
                    input.len()
                };
                for (i, &b) in fmt.iter().enumerate() {
                    c.bus.write8(SVFSCANF_T_FMT + i as u32, b);
                }
                c.bus.write8(SVFSCANF_T_FMT + fmt.len() as u32, 0);
                for (i, &b) in input[..len].iter().enumerate() {
                    c.bus.write8(SVFSCANF_T_INPUT + i as u32, b);
                }
                c.bus.write8(SVFSCANF_T_INPUT + len as u32, 0);
                // The FILE, laid out as sscanf does.
                fill(c, &mut r, SVFSCANF_T_FILE, 0x60 / 4);
                let f = SVFSCANF_T_FILE;
                c.bus.write32(f + SVFSCANF_FILE_P, SVFSCANF_T_INPUT);
                c.bus.write32(f + SVFSCANF_FILE_R, len as u32);
                c.bus.write16(f + 0xc, 0x4); // __SRD
                c.bus.write32(f + 0x10, SVFSCANF_T_INPUT); // _bf._base
                c.bus.write32(f + 0x14, len as u32); // _bf._size
                c.bus.write32(f + 0x20, SVFSCANF_T_EOFREAD); // _read
                c.bus.write32(f + 0x30, 0); // _ub._base
                c.bus.write32(f + 0x44, 0); // _lb._base
                c.bus.write32(f + 0x54, SVFSCANF_T_REENT); // _data
                c.bus.write32(SVFSCANF_T_REENT + 0x38, 1); // __sdidinit
                                                           // va_list: 8-byte slots, each pointing at a 0x40-byte output.
                fill(c, &mut r, SVFSCANF_T_OUT, 0x200 / 4);
                for i in 0..8u32 {
                    c.bus
                        .write32(SVFSCANF_T_AP + 8 * i, SVFSCANF_T_OUT + 0x40 * i);
                    c.bus.write32(SVFSCANF_T_AP + 8 * i + 4, 0);
                }
                c.g[A0][0] = sx(SVFSCANF_T_FILE);
                c.g[A1][0] = sx(SVFSCANF_T_FMT);
                c.g[A2][0] = sx(SVFSCANF_T_AP);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0041_7ab8;
            },
        );
    }
}
