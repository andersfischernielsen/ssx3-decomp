//! `src/seg/seg_316fb8.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use svfscanf_callee as svc;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0041_6b18, strncmp, "strncmp"),
    (0x0041_7ab8, svfscanf, "__svfscanf"),
    (0x0041_98d8, strtod_r, "_strtod_r"),
    (0x0041_aa88, strcasecmp, "strcasecmp"),
    (0x0041_ab78, strncasecmp, "strncasecmp"),
    (0x0041_ea18, tolower, "tolower"),
];

// ---------------------------------------------------------------------------
// __svfscanf (newlib, with the multibyte format scan)

/// Callees of `__svfscanf` (all in shard s15).
pub(crate) mod svfscanf_callee {
    use super::ee;
    use super::Callee;

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

// ---------------------------------------------------------------------------
// tolower (0x41ea18), strcasecmp (0x41aa88)

/// `_ctype_ + 1`: the newlib character class table (bit 0: upper case).
pub(crate) const CTYPE_TABLE: u32 = 0x0049_9c81;

/// `int tolower(int ch)` (0x41ea18): `ch + 0x20` if `_ctype_[ch]` says upper
/// case, else `ch` (a1 = the table entry's address, v1 = its upper bit).
pub fn tolower(c: &mut Cpu) {
    c.cycles += 8;
    let ch = c.g[A0][0];
    let at = CTYPE_TABLE.wrapping_add(ch as u32);
    c.g[A1][0] = sx(at);
    let upper = Addr(at).lbu(c) & 1;
    c.g[V1][0] = upper;
    c.g[V0][0] = if upper != 0 {
        sx((ch as u32).wrapping_add(0x20))
    } else {
        ch
    };
    abi::ret(c);
}

pub(crate) const STRCASECMP_ENTRY: u32 = 0x0041_aa88;

/// The loop head the trampoline resumes at after an unwind.
pub(crate) const STRCASECMP_HEAD: u32 = 0x0041_aaa8;

pub(crate) const STRCASECMP_END: u32 = 0x0041_ab04;

pub(crate) const TOLOWER: Callee = Callee {
    addr: 0x0041_ea18,
    f: ee::s15::tolower,
};

/// `int strcasecmp(const char* a, const char* b)` (0x41aa88): compare
/// case-insensitively (signed chars while scanning, unsigned for the
/// result); returns `tolower(*a) - tolower(*b)` at the first difference or
/// the end of `a`. s1/s2 walk the strings; resumable at its loop head.
pub fn strcasecmp(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x40 };
        let mut advance_b = match c.pc {
            STRCASECMP_ENTRY => {
                c.cycles += 8;
                Frame::open(c, 0x40);
                frame.save(c, S1, 0x20);
                frame.save(c, S2, 0x10);
                c.g[S1][0] = c.g[A0][0];
                frame.save(c, S0, 0x30);
                c.g[S2][0] = c.g[A1][0];
                frame.save_ra(c, 0);
                false
            }
            STRCASECMP_HEAD => true,
            pc => {
                c.jump_out(pc, STRCASECMP_ENTRY, STRCASECMP_END);
                return Ok(());
            }
        };
        loop {
            if advance_b {
                c.cycles += 1;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            }
            advance_b = true;
            c.cycles += 3;
            c.g[A0][0] = Addr(c.g[S1][0] as u32).lb(c);
            if c.g[A0][0] == 0 {
                break;
            }
            c.cycles += 7;
            TOLOWER.call(c, 0x0041_aac0)?;
            c.g[A0][0] = Addr(c.g[S2][0] as u32).lb(c);
            c.g[S0][0] = c.g[V0][0];
            TOLOWER.call(c, 0x0041_aacc)?;
            if c.g[S0][0] != c.g[V0][0] {
                break;
            }
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            if back_edge_unwinds(c, STRCASECMP_HEAD) {
                return Ok(());
            }
        }
        c.cycles += 12;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).lbu(c);
        TOLOWER.call(c, 0x0041_aadc)?;
        c.g[A0][0] = Addr(c.g[S2][0] as u32).lbu(c);
        c.g[S0][0] = c.g[V0][0];
        TOLOWER.call(c, 0x0041_aae8)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The function's address range (for computed jumps that leave the tables).
pub(crate) const SVFSCANF_START: u32 = 0x0041_7ab8;

pub(crate) const SVFSCANF_END: u32 = 0x0041_85cc;

/// `_impure_ptr` and `__mb_cur_max`.
pub(crate) const SVFSCANF_IMPURE_PTR: u32 = 0x0045_4944;

pub(crate) const SVFSCANF_MB_CUR_MAX: u32 = 0x0045_5140;

/// `_ctype_ + 1` (0x4a0000 - 0x637f): bit 0 upper, bit 3 space.
pub(crate) const SVFSCANF_CTYPE: u32 = 0x0049_9c81;

/// `basefix[17]` (shorts): the base an octal/decimal digit switches to.
pub(crate) const SVFSCANF_BASEFIX: u32 = 0x0049_9d90;

/// Jump tables: conversion characters (0..0x78), conversion kinds (0..4),
/// integer characters ('+'..'x'), float characters ('+'..'e').
pub(crate) const SVFSCANF_SPEC_TABLE: u32 = 0x0049_9dc0;

pub(crate) const SVFSCANF_KIND_TABLE: u32 = 0x0049_9fb0;

pub(crate) const SVFSCANF_INT_TABLE: u32 = 0x0049_9fd0;

pub(crate) const SVFSCANF_FLOAT_TABLE: u32 = 0x0049_a110;

/// `strtol` / `strtoul`, the integer converters kept at `CCFN`.
pub(crate) const SVFSCANF_STRTOL: u32 = 0x0041_e9e0;

pub(crate) const SVFSCANF_STRTOUL: u32 = 0x0041_aa50;

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
pub(crate) const SVF_LONG: u64 = 0x1;

pub(crate) const SVF_LONGDBL: u64 = 0x2;

pub(crate) const SVF_SHORT: u64 = 0x4;

pub(crate) const SVF_SUPPRESS: u64 = 0x8;

pub(crate) const SVF_POINTER: u64 = 0x10;

pub(crate) const SVF_NOSKIP: u64 = 0x20;

pub(crate) const SVF_SIGNOK: u64 = 0x40;

pub(crate) const SVF_NDIGITS: u64 = 0x80;

pub(crate) const SVF_PFXOK: u64 = 0x100;

pub(crate) const SVF_NZDIGITS: u64 = 0x200;

// Registers the original keeps its state in.
pub(crate) const SVF_FILE: usize = S2;
// FILE *fp
pub(crate) const SVF_AP: usize = S5;
// va_list (8-byte slots)
pub(crate) const SVF_NREAD: usize = S7;
// characters consumed
pub(crate) const SVF_BASE: usize = FP;
// integer base
pub(crate) const SVF_FLAGS: usize = S3;

pub(crate) const SVF_WIDTH: usize = S4;

/// Where the scan continues after a directive.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum SvfNext {
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
pub(crate) trait SvfRegs {
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
pub(crate) fn svf_back_edge(c: &mut Cpu, head: u32) {
    if event_due(c) {
        deliver_event_here(c, head);
    }
}

/// Jump to the loop top with its event check.
pub(crate) fn svf_top(c: &mut Cpu) -> SvfNext {
    svf_back_edge(c, 0x0041_7b18);
    SvfNext::Top
}

/// Jump to the loop head with its event check.
pub(crate) fn svf_head(c: &mut Cpu) -> SvfNext {
    svf_back_edge(c, 0x0041_7b1c);
    SvfNext::Head
}

pub(crate) fn svf_out(c: &mut Cpu, target: u32) -> SvfNext {
    c.jump_out(target, SVFSCANF_START, SVFSCANF_END);
    SvfNext::Out
}

/// 0x41858c `input_failure`: `v1` holds `nassigned`; return it, or EOF when 0.
pub(crate) fn svf_input_failure(c: &mut Cpu) -> SvfNext {
    c.cycles += 3;
    c.g[V0][0] = if c.g[V1][0] != 0 {
        c.g[V1][0]
    } else {
        u64::MAX
    };
    SvfNext::Exit
}

/// 0x418588: load `nassigned` into `v1`, then `input_failure`.
pub(crate) fn svf_fail_load(c: &mut Cpu) -> SvfNext {
    c.cycles += 1;
    let sp = sp(c);
    c.g[V1][0] = c.lw(sp + SVFSCANF_NASSIGNED);
    svf_input_failure(c)
}

/// 0x418598 `match_failure`/done: return `nassigned`.
pub(crate) fn svf_ret_count(c: &mut Cpu) -> SvfNext {
    c.cycles += 1;
    let sp = sp(c);
    c.g[V0][0] = c.lw(sp + SVFSCANF_NASSIGNED);
    SvfNext::Exit
}

/// `__srefill(fp)` at the call returning to `ret`: true when it failed (EOF).
pub(crate) fn svf_refill(c: &mut Cpu, ret: u32) -> Result<bool, Unwound> {
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

pub(crate) fn svf_prologue(c: &mut Cpu) {
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

pub(crate) fn svf_epilogue(c: &mut Cpu) {
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
pub(crate) fn svf_directive(c: &mut Cpu) -> Result<SvfNext, Unwound> {
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
pub(crate) fn svf_base10(c: &mut Cpu, sp: u32) {
    c.cycles += 3;
    c.g[SVF_BASE][0] = 0xa;
    c.sw(sp + SVFSCANF_CCFN, c.g[V0][0]);
}

/// 0x417c24: match `nbytes` literal format bytes (ending at fmt, in v1)
/// against the input.
pub(crate) fn svf_literal(c: &mut Cpu) -> Result<SvfNext, Unwound> {
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
pub(crate) fn svf_store_nread(c: &mut Cpu) -> SvfNext {
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
pub(crate) fn svf_scan_chars(c: &mut Cpu) -> Result<SvfNext, Unwound> {
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
pub(crate) fn svf_scan_class(c: &mut Cpu) -> Result<SvfNext, Unwound> {
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
pub(crate) fn svf_scan_string(c: &mut Cpu) -> Result<SvfNext, Unwound> {
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
pub(crate) fn svf_number_start(c: &mut Cpu, set: u64) -> bool {
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
pub(crate) fn svf_number_take(c: &mut Cpu, ret: u32) -> Result<bool, Unwound> {
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
pub(crate) enum SvfStop {
    /// A character that does not fit, or the width ran out (v0 not set).
    Char,
    /// `__srefill` failed (v0 = flags & NDIGITS already).
    Eof,
    Out,
}

/// 0x41817c CT_INT: collect an integer into the buffer, then convert it with
/// the `CCFN` converter and store it.
pub(crate) fn svf_scan_int(c: &mut Cpu) -> Result<SvfNext, Unwound> {
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
pub(crate) fn svf_buf_len_top(c: &mut Cpu) -> SvfNext {
    c.cycles += 1;
    c.svf_set(V0, r32(c, S1).wrapping_sub(r32(c, S6)));
    svf_add_nread_top(c)
}

/// 0x418580: nread += v0, next directive.
pub(crate) fn svf_add_nread_top(c: &mut Cpu) -> SvfNext {
    c.cycles += 2;
    c.svf_set(SVF_NREAD, r32(c, SVF_NREAD).wrapping_add(r32(c, V0)));
    svf_top(c)
}

/// 0x4183b4 CT_FLOAT: collect a float into the buffer, then `atof` it and
/// store a double (`l`/`L`) or a float.
pub(crate) fn svf_scan_float(c: &mut Cpu) -> Result<SvfNext, Unwound> {
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

// ---------------------------------------------------------------------------
// strncmp (0x416b18)

/// `0x0101010101010101`: the "subtract one from every byte" constant of the
/// NUL-byte test.
pub(crate) const STRNCMP_ONES: u64 = 0x0101_0101_0101_0101;

/// `0x8080808080808080`: the byte sign bits.
pub(crate) const STRNCMP_HIGHS: u64 = 0x8080_8080_8080_8080;

/// Resume points (loop heads with an event check) and the stages between.
pub(crate) const STRNCMP_ENTRY: u32 = 0x0041_6b18;

pub(crate) const STRNCMP_RET0: u32 = 0x0041_6b20;

pub(crate) const STRNCMP_LOOP16: u32 = 0x0041_6bac;

pub(crate) const STRNCMP_LOOP8: u32 = 0x0041_6c48;

pub(crate) const STRNCMP_BYTES: u32 = 0x0041_6ca8;

pub(crate) const STRNCMP_TAIL: u32 = 0x0041_6c94;

pub(crate) const STRNCMP_SETUP8: u32 = 0x0041_6c08;

pub(crate) const STRNCMP_TAIL_FROM_A3: u32 = 0x0041_6c88;

pub(crate) const STRNCMP_TAIL_SET: u32 = 0x0041_6c8c;

pub(crate) const STRNCMP_COMPARE: u32 = 0x0041_6cc4;

pub(crate) const STRNCMP_END: u32 = 0x0041_6cd0;

/// `int strncmp(const char* a, const char* b, size_t n)` (0x416b18): the
/// libc compare with the EE fast paths: both strings 16-byte aligned and
/// `n >= 16`: quadwords while equal and NUL-free (MMI `psubw`/`psubb`/`pnor`);
/// 8-byte aligned: doublewords the same way; the rest byte by byte. Returns
/// the difference of the first differing bytes (unsigned), 0 when equal up to
/// `n` or a NUL. Its loop heads are resume points (an event may unwind the
/// thread there), so it keeps the original's stages and dispatches on `pc`.
pub fn strncmp(c: &mut Cpu) {
    use crate::events::back_edge_unwinds;
    let mut pc = c.pc;
    loop {
        match pc {
            STRNCMP_ENTRY => {
                c.cycles += 2;
                c.g[V1][0] = c.g[A0][0] | c.g[A1][0];
                if c.g[A2][0] == 0 {
                    pc = STRNCMP_RET0;
                    continue;
                }
                c.cycles += 3;
                c.g[V0][0] = c.g[A2][0];
                if c.g[V1][0] & 7 != 0 {
                    pc = STRNCMP_TAIL; // unaligned: bytes
                    continue;
                }
                c.cycles += 12;
                c.g[A3][0] = (c.g[A2][0] < 0x10) as u64;
                c.g[T1][0] = STRNCMP_ONES;
                c.g[V0][0] = (c.g[V1][0] & 0xf) | c.g[A3][0];
                c.g[A3][0] = c.g[A0][0];
                if c.g[V0][0] != 0 {
                    pc = STRNCMP_SETUP8;
                    continue;
                }
                // Quadwords: the first one must be equal.
                c.cycles += 17;
                c.lq(3, r32(c, A0));
                c.pcpyld(T2, T1, T1);
                c.lq(2, r32(c, A1));
                c.g[T0][0] = STRNCMP_HIGHS;
                c.psubw(V1, V1, V0);
                c.pcpyld(T1, T0, T0);
                c.pcpyud(V0, V1, A0);
                c.g[T0][0] = c.g[A1][0];
                c.g[V1][0] |= c.g[V0][0];
                let differ = c.g[V1][0] != 0;
                c.g[V0][0] = c.g[A2][0];
                if differ {
                    pc = STRNCMP_TAIL;
                    continue;
                }
                c.cycles += 1;
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(0x10));
                pc = STRNCMP_LOOP16;
            }
            STRNCMP_RET0 => {
                c.cycles += 2;
                c.g[V0][0] = 0;
                ret(c);
                return;
            }
            STRNCMP_LOOP16 => {
                c.cycles += 2;
                if c.g[A2][0] == 0 {
                    if back_edge_unwinds(c, STRNCMP_RET0) {
                        return;
                    }
                    pc = STRNCMP_RET0;
                    continue;
                }
                // A NUL byte in the (equal) quadword ends the compare.
                c.cycles += 9;
                c.lq(2, r32(c, A3));
                c.pnor(V1, 0, V0);
                c.psubb(V0, V0, T2);
                c.pand(V0, V0, V1);
                c.pand(V1, V0, T1);
                c.pcpyud(V0, V1, A0);
                c.g[V0][0] |= c.g[V1][0];
                let nul = c.g[V0][0] != 0;
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(0x10));
                if nul {
                    if back_edge_unwinds(c, STRNCMP_RET0) {
                        return;
                    }
                    pc = STRNCMP_RET0;
                    continue;
                }
                c.cycles += 4;
                c.g[V0][0] = (c.g[A2][0] < 0x10) as u64;
                c.lq(3, r32(c, A3));
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x10));
                if c.g[V0][0] != 0 {
                    pc = STRNCMP_TAIL_FROM_A3;
                    continue;
                }
                c.cycles += 6;
                c.lq(2, r32(c, T0));
                c.psubw(V1, V1, V0);
                c.pcpyud(V0, V1, A0);
                c.g[V0][0] |= c.g[V1][0];
                if c.g[V0][0] == 0 {
                    c.g[A2][0] = sx(r32(c, A2).wrapping_sub(0x10));
                    if back_edge_unwinds(c, STRNCMP_LOOP16) {
                        return;
                    }
                    pc = STRNCMP_LOOP16;
                    continue;
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[A3][0];
                pc = STRNCMP_TAIL_SET;
            }
            STRNCMP_SETUP8 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[A2][0] < 8) as u64;
                c.g[T0][0] = c.g[A1][0];
                if c.g[V0][0] != 0 {
                    pc = STRNCMP_TAIL_FROM_A3;
                    continue;
                }
                // Doublewords: the first one must be equal.
                c.cycles += 4;
                c.g[V1][0] = Addr(r32(c, A0)).ld(c);
                c.g[V0][0] = Addr(r32(c, A1)).ld(c);
                let differ = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = c.g[A2][0];
                if differ {
                    pc = STRNCMP_TAIL;
                    continue;
                }
                c.cycles += 9;
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(8));
                c.g[T2][0] = STRNCMP_HIGHS;
                pc = STRNCMP_LOOP8;
            }
            STRNCMP_LOOP8 => {
                c.cycles += 2;
                if c.g[A2][0] == 0 {
                    if back_edge_unwinds(c, STRNCMP_RET0) {
                        return;
                    }
                    pc = STRNCMP_RET0;
                    continue;
                }
                // `(w - 0x01..) & ~w & 0x80..`: a NUL byte in w.
                c.cycles += 7;
                c.g[V0][0] = Addr(r32(c, A3)).ld(c);
                c.g[V1][0] = !c.g[V0][0];
                c.g[V0][0] = c.g[V0][0].wrapping_sub(c.g[T1][0]);
                c.g[V0][0] &= c.g[V1][0];
                c.g[V0][0] &= c.g[T2][0];
                let nul = c.g[V0][0] != 0;
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(8));
                if nul {
                    if back_edge_unwinds(c, STRNCMP_RET0) {
                        return;
                    }
                    pc = STRNCMP_RET0;
                    continue;
                }
                c.cycles += 3;
                c.g[V0][0] = (c.g[A2][0] < 8) as u64;
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(8));
                if c.g[V0][0] != 0 {
                    pc = STRNCMP_TAIL_FROM_A3;
                    continue;
                }
                c.cycles += 4;
                c.g[V1][0] = Addr(r32(c, A3)).ld(c);
                c.g[V0][0] = Addr(r32(c, T0)).ld(c);
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[A2][0] = sx(r32(c, A2).wrapping_sub(8));
                    if back_edge_unwinds(c, STRNCMP_LOOP8) {
                        return;
                    }
                    pc = STRNCMP_LOOP8;
                    continue;
                }
                pc = STRNCMP_TAIL_FROM_A3;
            }
            STRNCMP_TAIL_FROM_A3 => {
                c.cycles += 1;
                c.g[A0][0] = c.g[A3][0];
                pc = STRNCMP_TAIL_SET;
            }
            STRNCMP_TAIL_SET => {
                c.cycles += 2;
                c.g[A1][0] = c.g[T0][0];
                c.g[V0][0] = c.g[A2][0];
                pc = STRNCMP_TAIL;
            }
            STRNCMP_TAIL => {
                // Byte by byte (v0 = bytes left).
                c.cycles += 2;
                let none = c.g[V0][0] == 0;
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
                if none {
                    pc = STRNCMP_END;
                    continue;
                }
                c.cycles += 2;
                c.g[V1][0] = Addr(r32(c, A0)).lb(c);
                pc = STRNCMP_COMPARE;
            }
            STRNCMP_BYTES => {
                c.cycles += 2;
                if c.g[A2][0] == 0 {
                    if back_edge_unwinds(c, STRNCMP_RET0) {
                        return;
                    }
                    pc = STRNCMP_RET0;
                    continue;
                }
                c.cycles += 2;
                let nul = c.g[A3][0] == 0;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                if nul {
                    if back_edge_unwinds(c, STRNCMP_RET0) {
                        return;
                    }
                    pc = STRNCMP_RET0;
                    continue;
                }
                c.cycles += 3;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[V1][0] = Addr(r32(c, A0)).lb(c);
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
                pc = STRNCMP_COMPARE;
            }
            STRNCMP_COMPARE => {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, A1)).lb(c);
                let same = c.g[V1][0] == c.g[V0][0];
                c.g[A3][0] = Addr(r32(c, A0)).lbu(c);
                if same {
                    if back_edge_unwinds(c, STRNCMP_BYTES) {
                        return;
                    }
                    pc = STRNCMP_BYTES;
                    continue;
                }
                pc = STRNCMP_END;
            }
            STRNCMP_END => {
                c.cycles += 4;
                c.g[V0][0] = Addr(r32(c, A1)).lbu(c);
                c.g[V1][0] = Addr(r32(c, A0)).lbu(c);
                c.g[V0][0] = sx(r32(c, V1).wrapping_sub(r32(c, V0)));
                ret(c);
                return;
            }
            _ => {
                c.jump_out(pc, STRNCMP_ENTRY, 0x0041_6ce0);
                return;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// _strtod_r (newlib's David Gay strtod on soft-float doubles)
// ---------------------------------------------------------------------------

/// Entry and end (exclusive) of `_strtod_r`.
pub(crate) const STRTOD_ENTRY: u32 = 0x0041_98d8;

pub(crate) const STRTOD_END: u32 = 0x0041_a7e8;

/// Frame 0xf0: ra at 0x50, s0..s7/fp saved as quadwords at 0x60..0xe0.
pub(crate) const STRTOD_FRAME: u32 = 0xf0;

/// Stack slots: `bbe` (0x0) and `bbbits` (0x4) filled by `__d2b`, the
/// caller's `endptr` (0x8), the sign flag (0xc), the decimal exponent `e`
/// (doubleword, 0x10), the first significant digit `s0` (0x18), `y` (the
/// first nine digits, later the exponent bits of `rv`, 0x20), the bigints
/// `bb`, `bd`, `bd0`, `bs`, `delta` (0x30..0x40), `e` as int (0x44) and
/// `&bbbits` (0x48).
pub(crate) const STRTOD_SE: u32 = 0x8;

pub(crate) const STRTOD_SIGN: u32 = 0xc;

pub(crate) const STRTOD_E: u32 = 0x10;

pub(crate) const STRTOD_S0: u32 = 0x18;

pub(crate) const STRTOD_Y: u32 = 0x20;

pub(crate) const STRTOD_BB: u32 = 0x30;

pub(crate) const STRTOD_BD: u32 = 0x34;

pub(crate) const STRTOD_BD0: u32 = 0x38;

pub(crate) const STRTOD_BS: u32 = 0x3c;

pub(crate) const STRTOD_DELTA: u32 = 0x40;

pub(crate) const STRTOD_EINT: u32 = 0x44;

pub(crate) const STRTOD_BBBITS_PTR: u32 = 0x48;

/// Tables: the character switch (0x49a240), `tens[]` 1e0..1e22 (0x49a8f8)
/// and HUGE_VAL (0x49a898). `bigtens[]` (0x49a9c0 = 0x4a0000 - 0x5640) and
/// `tinytens[]` (0x49a9e8 = 0x4a0000 - 0x5618) are addressed from s6.
pub(crate) const STRTOD_SWITCH: u32 = 0x0049_a240;

pub(crate) const STRTOD_TENS: u32 = 0x0049_a8f8;

pub(crate) const STRTOD_HUGE: u32 = 0x0049_a898;

/// `ERANGE`, stored in `ptr->_errno` (offset 0).
pub(crate) const STRTOD_ERANGE: u64 = 0x22;

/// Mantissa bits of a double (`Frac_mask` over both words).
pub(crate) const STRTOD_FRAC: u64 = u64::MAX >> 12;

/// Low word mask.
pub(crate) const STRTOD_LOW: u64 = u64::MAX >> 32;

/// Double constants as bit patterns.
pub(crate) const STRTOD_TWO32: u64 = 0x41f0_0000_0000_0000;

pub(crate) const STRTOD_TWO: u64 = 0x4000_0000_0000_0000;

pub(crate) const STRTOD_ONE: u64 = 0x3ff0_0000_0000_0000;

pub(crate) const STRTOD_HALF: u64 = 0x3fe0_0000_0000_0000;

pub(crate) const STRTOD_MINUS_ONE: u64 = 0xbff0_0000_0000_0000;

/// The largest double (`Big0`/`Big1`), and its high word over the low mask.
pub(crate) const STRTOD_BIG: u64 = 0x7fef_ffff_ffff_ffff;

pub(crate) const STRTOD_BIG_HIGH: u64 = 0x7fef_ffff_0000_0000;

/// Rounding thresholds 0.5 - eps, 0.5 + eps, 0.25 - eps.
pub(crate) const STRTOD_HALF_LO: u64 = 0x3fdf_ffff_94a0_3595;

pub(crate) const STRTOD_HALF_HI: u64 = 0x3fe0_0000_35af_e535;

pub(crate) const STRTOD_QUARTER_LO: u64 = 0x3fcf_ffff_94a0_3595;

pub(crate) const STRTOD_FLOATSIDF: Callee = Callee {
    addr: 0x0041_2550,
    f: ee::s15::__floatsidf,
};

pub(crate) const STRTOD_FLOATDIDF: Callee = Callee {
    addr: 0x0041_1438,
    f: ee::s15::__floatdidf,
};

pub(crate) const STRTOD_ADDDF3: Callee = Callee {
    addr: 0x0041_1f18,
    f: ee::s15::__adddf3,
};

pub(crate) const STRTOD_SUBDF3: Callee = Callee {
    addr: 0x0041_1f70,
    f: ee::s15::__subdf3,
};

pub(crate) const STRTOD_MULDF3: Callee = Callee {
    addr: 0x0041_1fd8,
    f: ee::s15::__muldf3,
};

pub(crate) const STRTOD_DIVDF3: Callee = Callee {
    addr: 0x0041_2280,
    f: ee::s15::__divdf3,
};

pub(crate) const STRTOD_CMPDF2: Callee = Callee {
    addr: 0x0041_2500,
    f: ee::s15::__cmpdf2,
};

pub(crate) const STRTOD_FIXDFSI: Callee = Callee {
    addr: 0x0041_2608,
    f: ee::s15::__fixdfsi,
};

pub(crate) const STRTOD_FIXDFDI: Callee = Callee {
    addr: 0x0042_d320,
    f: ee::s15::__fixdfdi,
};

pub(crate) const STRTOD_ULP: Callee = Callee {
    addr: 0x0042_1020,
    f: ee::s15::__ulp,
};

pub(crate) const STRTOD_S2B: Callee = Callee {
    addr: 0x0042_06f8,
    f: ee::s15::__s2b,
};

pub(crate) const STRTOD_BALLOC: Callee = Callee {
    addr: 0x0042_0510,
    f: ee::s15::_Balloc,
};

pub(crate) const STRTOD_BFREE: Callee = Callee {
    addr: 0x0042_05b8,
    f: ee::s15::_Bfree,
};

pub(crate) const STRTOD_MEMCPY: Callee = Callee {
    addr: 0x0041_605c,
    f: ee::s15::memcpy,
};

pub(crate) const STRTOD_D2B: Callee = Callee {
    addr: 0x0042_1280,
    f: ee::s15::__d2b,
};

pub(crate) const STRTOD_I2B: Callee = Callee {
    addr: 0x0042_0978,
    f: ee::s15::__i2b,
};

pub(crate) const STRTOD_POW5MULT: Callee = Callee {
    addr: 0x0042_0bc8,
    f: ee::s15::__pow5mult,
};

pub(crate) const STRTOD_MULTIPLY: Callee = Callee {
    addr: 0x0042_09b0,
    f: ee::s15::__multiply,
};

pub(crate) const STRTOD_LSHIFT: Callee = Callee {
    addr: 0x0042_0cc8,
    f: ee::s15::__lshift,
};

pub(crate) const STRTOD_MDIFF: Callee = Callee {
    addr: 0x0042_0e90,
    f: ee::s15::__mdiff,
};

pub(crate) const STRTOD_MCMP: Callee = Callee {
    addr: 0x0042_0e28,
    f: ee::s15::__mcmp,
};

pub(crate) const STRTOD_RATIO: Callee = Callee {
    addr: 0x0042_1400,
    f: ee::s15::__ratio,
};

// Stages, named after their guest address. The ones marked "head" are loop
// heads with an event check on their back edge: the trampoline re-enters the
// function there after an unwind.
/// Skip blanks, read the sign (head).
pub(crate) const STRTOD_SKIP: u32 = 0x0041_9920;

/// First character after the sign.
pub(crate) const STRTOD_FIRST: u32 = 0x0041_9974;

/// Leading zeros (head).
pub(crate) const STRTOD_LEAD0: u32 = 0x0041_9988;

/// Start of the mantissa digits.
pub(crate) const STRTOD_MANT: u32 = 0x0041_99ac;

/// Integer digits (head).
pub(crate) const STRTOD_INTDIG: u32 = 0x0041_99e8;

/// The decimal point test (v0 = '.').
pub(crate) const STRTOD_POINT: u32 = 0x0041_9a44;

/// Zeros right after the point when there is no integer digit (head).
pub(crate) const STRTOD_FRACZ: u32 = 0x0041_9a68;

/// First fraction character after those zeros (v0 = c - '1').
pub(crate) const STRTOD_FRACNZ: u32 = 0x0041_9a88;

/// Fraction digit (head).
pub(crate) const STRTOD_FRACDIG: u32 = 0x0041_9aa8;

/// Scale the digits by ten once per skipped zero (head).
pub(crate) const STRTOD_FRACMUL: u32 = 0x0041_9ad0;

/// Append the fraction digit.
pub(crate) const STRTOD_FRACLAST: u32 = 0x0041_9b0c;

/// Next fraction character.
pub(crate) const STRTOD_FRACNEXT: u32 = 0x0041_9b54;

/// Fraction loop test.
pub(crate) const STRTOD_FRACTEST: u32 = 0x0041_9b5c;

/// The exponent marker test (v0 = 'e').
pub(crate) const STRTOD_EXP: u32 = 0x0041_9b70;

/// The exponent's sign.
pub(crate) const STRTOD_EXPSIGN: u32 = 0x0041_9ba4;

/// Exponent leading zeros (head).
pub(crate) const STRTOD_EXPZ: u32 = 0x0041_9bf0;

/// First exponent digit (v0 = c - '1').
pub(crate) const STRTOD_EXPDIGS: u32 = 0x0041_9c10;

/// Exponent digits (head).
pub(crate) const STRTOD_EXPDIG: u32 = 0x0041_9c38;

/// Clamp a long exponent, apply its sign.
pub(crate) const STRTOD_EXPCLAMP: u32 = 0x0041_9c74;

/// End of the text: nothing parsed, or convert.
pub(crate) const STRTOD_DIGEND: u32 = 0x0041_9ca4;

/// The digits as a double, the exact fast cases.
pub(crate) const STRTOD_CONVERT: u32 = 0x0041_9cb8;

/// Scale by `e1` (v0 = nd - k).
pub(crate) const STRTOD_SCALE: u32 = 0x0041_9e60;

/// Overflow: HUGE_VAL, ERANGE (head; and its second entry, v0 = ERANGE).
pub(crate) const STRTOD_OVFL: u32 = 0x0041_9ea8;

pub(crate) const STRTOD_OVFL2: u32 = 0x0041_9eac;

/// Positive powers of 1e16.
pub(crate) const STRTOD_BIGPOW: u32 = 0x0041_9ec8;

/// `bigtens[]` loop (head).
pub(crate) const STRTOD_BIGLOOP: u32 = 0x0041_9ee0;

/// Last `bigtens[]` factor with the exponent biased down.
pub(crate) const STRTOD_BIGLAST: u32 = 0x0041_9f10;

/// Negative scale.
pub(crate) const STRTOD_NEG: u32 = 0x0041_9fa8;

/// `tinytens[]` loop (head).
pub(crate) const STRTOD_TINYLOOP: u32 = 0x0041_a008;

/// Last `tinytens[]` factor and the underflow checks.
pub(crate) const STRTOD_TINYLAST: u32 = 0x0041_a038;

/// Underflow: zero, ERANGE (head; and its second entry, v0 = ERANGE).
pub(crate) const STRTOD_UNDFL: u32 = 0x0041_a0a4;

pub(crate) const STRTOD_UNDFL2: u32 = 0x0041_a0a8;

/// `rv += ulp(rv)` and finish (head).
pub(crate) const STRTOD_ULPADD: u32 = 0x0041_a0c0;

/// Bigint correction setup (`a1` = s0 loaded at 0x41a0dc or by the caller).
pub(crate) const STRTOD_BIGCOMP0: u32 = 0x0041_a0dc;

pub(crate) const STRTOD_BIGCOMP1: u32 = 0x0041_a0e0;

/// `delta <<= 1; cmp(delta, bs)` (head).
pub(crate) const STRTOD_LSHIFT1: u32 = 0x0041_a118;

/// `delta >= bs`: the correction step (head).
pub(crate) const STRTOD_CMP: u32 = 0x0041_a144;

/// `rv` rounded down to the largest value below a power of two.
pub(crate) const STRTOD_ROUNDDOWN: u32 = 0x0041_a1b0;

/// Halfway case: round to even.
pub(crate) const STRTOD_ODD: u32 = 0x0041_a1f4;

/// `ratio(delta, bs)` and the adjustment `aadj`.
pub(crate) const STRTOD_RATIOSTEP: u32 = 0x0041_a244;

/// `aadj1 = -aadj` (0x41a314).
pub(crate) const STRTOD_NEGADJ: u32 = 0x0041_a314;

/// Apply `aadj1 * ulp(rv)`.
pub(crate) const STRTOD_ADJ: u32 = 0x0041_a320;

/// Free the loop's bigints and go round again.
pub(crate) const STRTOD_NEXT: u32 = 0x0041_a558;

/// Top of the correction loop.
pub(crate) const STRTOD_LOOPTOP: u32 = 0x0041_a584;

/// Free the bigints (`a1` = bb loaded at 0x41a758 or by the caller).
pub(crate) const STRTOD_FREE: u32 = 0x0041_a758;

pub(crate) const STRTOD_FREE1: u32 = 0x0041_a75c;

/// Store `*se`, apply the sign, return (and its second entry, a0 = se).
pub(crate) const STRTOD_OUT: u32 = 0x0041_a794;

pub(crate) const STRTOD_OUT2: u32 = 0x0041_a798;

/// Returned.
pub(crate) const STRTOD_DONE: u32 = 0;

/// The resume heads (the loop heads with an event check on a back edge).
pub(crate) const STRTOD_HEADS: [u32; 17] = [
    STRTOD_SKIP,
    STRTOD_LEAD0,
    STRTOD_INTDIG,
    STRTOD_FRACZ,
    STRTOD_FRACDIG,
    STRTOD_FRACMUL,
    STRTOD_EXPZ,
    STRTOD_EXPDIG,
    STRTOD_OVFL,
    STRTOD_OVFL2,
    STRTOD_BIGLOOP,
    STRTOD_TINYLOOP,
    STRTOD_UNDFL,
    STRTOD_UNDFL2,
    STRTOD_ULPADD,
    STRTOD_LSHIFT1,
    STRTOD_CMP,
];

pub(crate) type StrtodStep = Result<u32, Unwound>;

/// A back edge to `head`: deliver a due event there (unwinding out if it
/// asks to), then continue at `head`.
pub(crate) fn strtod_back(c: &mut Cpu, head: u32) -> StrtodStep {
    if back_edge_unwinds(c, head) {
        Err(Unwound)
    } else {
        Ok(head)
    }
}

/// `sp + off`.
pub(crate) fn strtod_at(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// `rd = rs + rt` (32-bit, sign-extended).
pub(crate) fn strtod_addu(c: &mut Cpu, rd: usize, rs: usize, rt: usize) {
    c.g[rd][0] = sx(r32(c, rs).wrapping_add(r32(c, rt)));
}

/// `rd = rs + imm` (32-bit, sign-extended).
pub(crate) fn strtod_addiu(c: &mut Cpu, rd: usize, rs: usize, imm: u32) {
    c.g[rd][0] = sx(r32(c, rs).wrapping_add(imm));
}

/// The high word of a double, sign-extended (`dsra32`).
pub(crate) fn strtod_hi(v: u64) -> u64 {
    ((v as i64) >> 32) as u64
}

/// The low word sign-extended (`dsll32; dsra32`).
pub(crate) fn strtod_sext32(v: u64) -> u64 {
    ((v << 32) as i64 >> 32) as u64
}

/// `double _strtod_r(struct _reent* ptr, const char* s00, char** se)`
/// (0x4198d8): newlib's strtod. Skips blanks, reads a sign, the mantissa
/// digits (the first nine as an int in `y`, the next seven in `z`), a
/// fraction and an exponent; makes a first approximation with the
/// soft-float routines (exact when it can: up to 15 digits scaled by a
/// power of ten from `tens[]`), scales by `bigtens[]`/`tinytens[]`, and
/// then corrects it with bigint arithmetic (`__s2b`, `__d2b`, `__mdiff`,
/// `__ratio`) until it is the correctly rounded value. Sets `ERANGE` in
/// `ptr->_errno` on overflow (HUGE_VAL) and underflow (0). Stores the end
/// of the number in `*se` (the start of the text when nothing was read).
/// The value is returned in `v0` as double bits.
///
/// Dispatcher-shaped: re-entered at any of [`STRTOD_HEADS`] after an
/// unwind at that loop's event check; the state lives in the registers
/// (s5 = text, s4 = nd, fp = nd0, s2 = z, s1 = rv, s0 = e1, s7 = ptr) and
/// the frame.
pub fn strtod_r(c: &mut Cpu) {
    guest(c, |c| {
        let mut at = match c.pc {
            STRTOD_ENTRY => {
                strtod_prologue(c);
                STRTOD_SKIP
            }
            pc if STRTOD_HEADS.contains(&pc) => pc,
            pc => {
                c.jump_out(pc, STRTOD_ENTRY, STRTOD_END);
                return Ok(());
            }
        };
        loop {
            at = match at {
                STRTOD_SKIP => strtod_skip(c)?,
                STRTOD_FIRST => strtod_first(c),
                STRTOD_LEAD0 => strtod_lead0(c)?,
                STRTOD_MANT => strtod_mant(c),
                STRTOD_INTDIG => strtod_intdig(c)?,
                STRTOD_POINT => strtod_point(c),
                STRTOD_FRACZ => strtod_fracz(c)?,
                STRTOD_FRACNZ => strtod_fracnz(c),
                STRTOD_FRACDIG => strtod_fracdig(c),
                STRTOD_FRACMUL => strtod_fracmul(c)?,
                STRTOD_FRACLAST => strtod_fraclast(c),
                STRTOD_FRACNEXT => strtod_fracnext(c),
                STRTOD_FRACTEST => strtod_fractest(c)?,
                STRTOD_EXP => strtod_exp(c),
                STRTOD_EXPSIGN => strtod_expsign(c),
                STRTOD_EXPZ => strtod_expz(c)?,
                STRTOD_EXPDIGS => strtod_expdigs(c),
                STRTOD_EXPDIG => strtod_expdig(c)?,
                STRTOD_EXPCLAMP => strtod_expclamp(c),
                STRTOD_DIGEND => strtod_digend(c),
                STRTOD_CONVERT => strtod_convert(c)?,
                STRTOD_SCALE => strtod_scale(c)?,
                STRTOD_OVFL => {
                    c.cycles += 1;
                    c.g[V0][0] = STRTOD_ERANGE;
                    STRTOD_OVFL2
                }
                STRTOD_OVFL2 => strtod_ovfl2(c),
                STRTOD_BIGPOW => strtod_bigpow(c),
                STRTOD_BIGLOOP => strtod_powloop(c, STRTOD_BIGLOOP, 0x0041_9ef8, STRTOD_BIGLAST)?,
                STRTOD_BIGLAST => strtod_biglast(c)?,
                STRTOD_NEG => strtod_neg(c)?,
                STRTOD_TINYLOOP => {
                    strtod_powloop(c, STRTOD_TINYLOOP, 0x0041_a020, STRTOD_TINYLAST)?
                }
                STRTOD_TINYLAST => strtod_tinylast(c)?,
                STRTOD_UNDFL => {
                    c.cycles += 1;
                    c.g[V0][0] = STRTOD_ERANGE;
                    STRTOD_UNDFL2
                }
                STRTOD_UNDFL2 => strtod_undfl2(c),
                STRTOD_ULPADD => strtod_ulpadd(c)?,
                STRTOD_BIGCOMP0 => {
                    c.cycles += 1;
                    c.g[A1][0] = c.lw(strtod_at(c, STRTOD_S0));
                    STRTOD_BIGCOMP1
                }
                STRTOD_BIGCOMP1 => strtod_bigcomp(c)?,
                STRTOD_LSHIFT1 => strtod_lshift1(c)?,
                STRTOD_CMP => strtod_cmp(c),
                STRTOD_ROUNDDOWN => strtod_rounddown(c),
                STRTOD_ODD => strtod_odd(c)?,
                STRTOD_RATIOSTEP => strtod_ratio(c)?,
                STRTOD_NEGADJ => {
                    // aadj1 = 0 - aadj (a0 = 0).
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S0][0];
                    STRTOD_SUBDF3.call(c, 0x0041_a31c)?;
                    c.g[S2][0] = c.g[V0][0];
                    STRTOD_ADJ
                }
                STRTOD_ADJ => strtod_adj(c)?,
                STRTOD_NEXT => strtod_next(c)?,
                STRTOD_LOOPTOP => strtod_looptop(c)?,
                STRTOD_FREE => {
                    c.cycles += 1;
                    c.g[A1][0] = c.lw(strtod_at(c, STRTOD_BB));
                    STRTOD_FREE1
                }
                STRTOD_FREE1 => strtod_free(c)?,
                STRTOD_OUT => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(strtod_at(c, STRTOD_SE));
                    STRTOD_OUT2
                }
                STRTOD_OUT2 => strtod_out(c)?,
                _ => return Ok(()),
            };
            if at == STRTOD_DONE {
                return Ok(());
            }
        }
    })
}

/// 0x4198d8: open the frame, `s5 = s00`, `s7 = ptr`, `*(sp+8) = se`,
/// sign = 0, nz0 (t2) = nz (a2) = 0, rv (s1) = 0.
pub(crate) fn strtod_prologue(c: &mut Cpu) {
    c.cycles += 18;
    let frame = Frame::open(c, STRTOD_FRAME);
    c.g[T2][0] = 0;
    frame.save(c, S1, 0xd0);
    frame.save(c, S5, 0x90);
    c.g[S1][0] = 0;
    frame.save(c, S7, 0x70);
    c.g[S5][0] = c.g[A1][0];
    c.sw(frame.at(c, STRTOD_SE), c.g[A2][0]);
    c.g[S7][0] = c.g[A0][0];
    frame.save(c, S0, 0xe0);
    c.g[A2][0] = 0;
    frame.save(c, S2, 0xc0);
    frame.save(c, S3, 0xb0);
    frame.save(c, S4, 0xa0);
    frame.save(c, S6, 0x80);
    frame.save(c, FP, 0x60);
    frame.save_ra(c, 0x50);
    c.sw(frame.at(c, STRTOD_SIGN), 0);
}

/// 0x419920 (head): switch on the character below '.' through the table:
/// NUL gives up, blanks are skipped, '+'/'-' set the sign.
pub(crate) fn strtod_skip(c: &mut Cpu) -> StrtodStep {
    c.cycles += 4;
    c.g[V1][0] = c.lb(r32(c, S5));
    let small = c.g[V1][0] < 0x2e;
    c.g[V0][0] = 0x4a_0000;
    if !small {
        return Ok(STRTOD_FIRST);
    }
    c.cycles += 6;
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[V0][0] = sx(STRTOD_SWITCH);
    strtod_addu(c, V1, V1, V0);
    c.g[A0][0] = c.lw(r32(c, V1));
    match r32(c, A0) {
        // NUL: no number.
        0x0041_9b9c => {
            c.cycles += 2;
            c.g[S5][0] = c.g[A1][0];
            Ok(STRTOD_OUT)
        }
        // Any other character.
        0x0041_9970 => {
            c.cycles += 1;
            c.g[V1][0] = c.lb(r32(c, S5));
            Ok(STRTOD_FIRST)
        }
        // Blank: skip.
        0x0041_9968 => {
            c.cycles += 2;
            strtod_addiu(c, S5, S5, 1);
            strtod_back(c, STRTOD_SKIP)
        }
        // '-': sign = 1, then as '+'.
        0x0041_9948 => {
            c.cycles += 2;
            c.g[V0][0] = 1;
            c.sw(strtod_at(c, STRTOD_SIGN), 1);
            Ok(strtod_sign_char(c))
        }
        // '+'.
        0x0041_9950 => Ok(strtod_sign_char(c)),
        pc => {
            c.jump_out(pc, STRTOD_ENTRY, STRTOD_END);
            Err(Unwound)
        }
    }
}

/// 0x419950: past the sign; a sign alone is no number.
pub(crate) fn strtod_sign_char(c: &mut Cpu) -> u32 {
    c.cycles += 4;
    strtod_addiu(c, S5, S5, 1);
    c.g[V0][0] = c.lb(r32(c, S5));
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.lb(r32(c, S5));
        return STRTOD_FIRST;
    }
    c.cycles += 2;
    c.g[S5][0] = c.g[A1][0];
    STRTOD_OUT
}

/// 0x419974: a leading '0' sets nz0 (t2) and skips the zeros.
pub(crate) fn strtod_first(c: &mut Cpu) -> u32 {
    c.cycles += 3;
    c.g[V0][0] = 0x30;
    let zero = c.g[V1][0] == c.g[V0][0];
    c.g[A0][0] = c.lbu(r32(c, S5));
    if !zero {
        return STRTOD_MANT;
    }
    c.cycles += 2;
    c.g[T2][0] = 1;
    c.g[A0][0] = 0x30;
    STRTOD_LEAD0
}

/// 0x419988 (head): skip leading zeros; only zeros is the number 0.
pub(crate) fn strtod_lead0(c: &mut Cpu) -> StrtodStep {
    c.cycles += 7;
    strtod_addiu(c, S5, S5, 1);
    c.g[V1][0] = c.lb(r32(c, S5));
    let zero = c.g[V1][0] == c.g[A0][0];
    c.g[V0][0] = c.lbu(r32(c, S5));
    if zero {
        return strtod_back(c, STRTOD_LEAD0);
    }
    c.cycles += 2;
    let end = c.g[V1][0] == 0;
    c.g[A0][0] = c.g[V0][0];
    Ok(if end { STRTOD_OUT } else { STRTOD_MANT })
}

/// 0x4199ac: s0 = s; y = z = nd = nf = 0; digits?
pub(crate) fn strtod_mant(c: &mut Cpu) -> u32 {
    c.cycles += 9;
    c.g[V0][0] = sx(r32(c, A0) << 24);
    c.sw(strtod_at(c, STRTOD_S0), c.g[S5][0]);
    c.g[A0][0] = sx(((r32(c, V0) as i32) >> 24) as u32);
    c.g[S2][0] = 0;
    c.sw(strtod_at(c, STRTOD_Y), 0);
    c.g[T1][0] = 0;
    let below = (c.g[A0][0] as i64) < 48;
    c.g[V0][0] = below as u64;
    c.g[S4][0] = 0;
    if below {
        c.cycles += 1;
        c.g[V0][0] = 0x2e;
        return STRTOD_POINT;
    }
    c.cycles += 3;
    let digit = (c.g[A0][0] as i64) < 58;
    c.g[V0][0] = 0x2e;
    if !digit {
        return STRTOD_POINT;
    }
    c.cycles += 3;
    c.g[V1][0] = 0xa;
    c.g[V0][0] = ((c.g[S4][0] as i64) < 9) as u64;
    STRTOD_INTDIG
}

/// 0x4199e8 (head): an integer digit: into `y` for the first nine, into
/// `z` up to sixteen.
pub(crate) fn strtod_intdig(c: &mut Cpu) -> StrtodStep {
    c.cycles += 2;
    let in_y = c.g[V0][0] != 0;
    c.g[A3][0] = c.lw(strtod_at(c, STRTOD_Y));
    if in_y {
        // y = 10 * y + c - '0'
        c.cycles += 5;
        c.mult(0, A3, A3, V1);
        strtod_addu(c, V0, A3, A0);
        strtod_addiu(c, V0, V0, 0u32.wrapping_sub(0x30));
        c.sw(strtod_at(c, STRTOD_Y), c.g[V0][0]);
    } else {
        c.cycles += 3;
        let in_z = (c.g[S4][0] as i64) < 16;
        c.g[V0][0] = c.g[S2][0] << 2;
        if in_z {
            // z = 10 * z + c - '0' (64-bit)
            c.cycles += 4;
            c.g[V0][0] = c.g[V0][0].wrapping_add(c.g[S2][0]);
            c.g[V0][0] <<= 1;
            c.g[V0][0] = c.g[A0][0].wrapping_add(c.g[V0][0]);
            c.g[S2][0] = c.g[V0][0].wrapping_sub(0x30);
        }
    }
    // 0x419a20: next character, nd++.
    c.cycles += 5;
    strtod_addiu(c, S5, S5, 1);
    c.g[A0][0] = c.lb(r32(c, S5));
    let below = (c.g[A0][0] as i64) < 48;
    c.g[V0][0] = below as u64;
    strtod_addiu(c, S4, S4, 1);
    if !below {
        c.cycles += 3;
        let digit = (c.g[A0][0] as i64) < 58;
        c.g[V0][0] = ((c.g[S4][0] as i64) < 9) as u64;
        if digit {
            return strtod_back(c, STRTOD_INTDIG);
        }
    }
    c.cycles += 1;
    c.g[V0][0] = 0x2e;
    Ok(STRTOD_POINT)
}

/// 0x419a44: nd0 = nd; a '.' starts the fraction.
pub(crate) fn strtod_point(c: &mut Cpu) -> u32 {
    c.cycles += 2;
    let not_point = c.g[A0][0] != c.g[V0][0];
    c.g[FP][0] = c.g[S4][0];
    if not_point {
        c.cycles += 1;
        c.g[V0][0] = 0x65;
        return STRTOD_EXP;
    }
    c.cycles += 3;
    strtod_addiu(c, S5, S5, 1);
    let have_digits = c.g[S4][0] != 0;
    c.g[A0][0] = c.lb(r32(c, S5));
    if have_digits {
        return STRTOD_FRACTEST;
    }
    c.cycles += 3;
    c.g[V0][0] = 0x30;
    if c.g[A0][0] != c.g[V0][0] {
        c.g[V0][0] = sx(r32(c, A0).wrapping_sub(0x31));
        return STRTOD_FRACNZ;
    }
    c.cycles += 1;
    STRTOD_FRACZ
}

/// 0x419a68 (head): count the zeros after the point (nz, a2).
pub(crate) fn strtod_fracz(c: &mut Cpu) -> StrtodStep {
    c.cycles += 7;
    strtod_addiu(c, S5, S5, 1);
    c.g[A0][0] = c.lb(r32(c, S5));
    let zero = c.g[A0][0] == c.g[V0][0];
    strtod_addiu(c, A2, A2, 1);
    if zero {
        return strtod_back(c, STRTOD_FRACZ);
    }
    c.cycles += 1;
    c.g[V0][0] = sx(r32(c, A0).wrapping_sub(0x31));
    Ok(STRTOD_FRACNZ)
}

/// 0x419a88: a non-zero digit after the zeros: nf = nz, s0 = s, nz = 0.
pub(crate) fn strtod_fracnz(c: &mut Cpu) -> u32 {
    c.cycles += 3;
    let digit = c.g[V0][0] < 9;
    c.g[V0][0] = 0x65;
    if !digit {
        return STRTOD_EXP;
    }
    c.cycles += 5;
    c.g[T1][0] = c.g[A2][0];
    c.sw(strtod_at(c, STRTOD_S0), c.g[S5][0]);
    c.g[A2][0] = 0;
    strtod_addiu(c, T0, S5, 1);
    STRTOD_FRACDIG
}

/// 0x419aa8 (head): a fraction digit; zeros are only counted (nz).
pub(crate) fn strtod_fracdig(c: &mut Cpu) -> u32 {
    c.cycles += 3;
    strtod_addiu(c, A0, A0, 0u32.wrapping_sub(0x30));
    let zero = c.g[A0][0] == 0;
    strtod_addiu(c, A2, A2, 1);
    if zero {
        return STRTOD_FRACNEXT;
    }
    c.cycles += 4;
    c.g[A3][0] = 1;
    let pending = (c.g[A3][0] as i64) < (c.g[A2][0] as i64);
    c.g[V0][0] = pending as u64;
    strtod_addu(c, T1, T1, A2);
    if !pending {
        return STRTOD_FRACLAST;
    }
    c.cycles += 3;
    strtod_addiu(c, A3, A2, 0u32.wrapping_sub(1));
    c.g[V1][0] = 0xa;
    c.g[V0][0] = c.g[S4][0];
    STRTOD_FRACMUL
}

/// 0x419ad0 (head): one counted zero: y or z times ten, nd++.
pub(crate) fn strtod_fracmul(c: &mut Cpu) -> StrtodStep {
    c.cycles += 3;
    let in_y = (c.g[V0][0] as i64) < 9;
    c.g[V0][0] = in_y as u64;
    strtod_addiu(c, S4, S4, 1);
    if in_y {
        c.cycles += 4;
        c.g[V0][0] = c.lw(strtod_at(c, STRTOD_Y));
        c.mult(0, V0, V0, V1);
        c.sw(strtod_at(c, STRTOD_Y), c.g[V0][0]);
    } else {
        c.cycles += 3;
        let in_z = (c.g[S4][0] as i64) < 17;
        c.g[V0][0] = c.g[S2][0] << 2;
        if in_z {
            c.cycles += 2;
            c.g[V0][0] = c.g[V0][0].wrapping_add(c.g[S2][0]);
            c.g[S2][0] = c.g[V0][0] << 1;
        }
    }
    // 0x419b00
    c.cycles += 3;
    strtod_addiu(c, A3, A3, 0u32.wrapping_sub(1));
    let more = c.g[A3][0] != 0;
    c.g[V0][0] = c.g[S4][0];
    if more {
        return strtod_back(c, STRTOD_FRACMUL);
    }
    Ok(STRTOD_FRACLAST)
}

/// 0x419b0c: append the digit to y or z, nd++, nz = 0.
pub(crate) fn strtod_fraclast(c: &mut Cpu) -> u32 {
    c.cycles += 4;
    c.g[V0][0] = c.g[S4][0];
    let in_y = (c.g[V0][0] as i64) < 9;
    c.g[V0][0] = in_y as u64;
    strtod_addiu(c, S4, S4, 1);
    if in_y {
        // y = 10 * y + c (madd onto LO = c)
        c.cycles += 7;
        c.g[V1][0] = c.lw(strtod_at(c, STRTOD_Y));
        c.g[V0][0] = 0xa;
        c.lo[0] = c.g[A0][0];
        c.madd(0, 0, V1, V0);
        c.g[V1][0] = c.lo[0];
        c.sw(strtod_at(c, STRTOD_Y), c.g[V1][0]);
    } else {
        c.cycles += 3;
        let in_z = (c.g[S4][0] as i64) < 17;
        c.g[V0][0] = c.g[S2][0] << 2;
        if in_z {
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0].wrapping_add(c.g[S2][0]);
            c.g[V0][0] <<= 1;
            c.g[S2][0] = c.g[A0][0].wrapping_add(c.g[V0][0]);
        }
    }
    c.cycles += 1;
    c.g[A2][0] = 0;
    STRTOD_FRACNEXT
}

/// 0x419b54: the next fraction character.
pub(crate) fn strtod_fracnext(c: &mut Cpu) -> u32 {
    c.cycles += 2;
    c.g[S5][0] = c.g[T0][0];
    c.g[A0][0] = c.lb(r32(c, S5));
    STRTOD_FRACTEST
}

/// 0x419b5c: another fraction digit?
pub(crate) fn strtod_fractest(c: &mut Cpu) -> StrtodStep {
    c.cycles += 4;
    c.g[V0][0] = sx(r32(c, A0).wrapping_sub(0x30));
    let digit = c.g[V0][0] < 0xa;
    c.g[V0][0] = digit as u64;
    strtod_addiu(c, T0, S5, 1);
    if digit {
        return strtod_back(c, STRTOD_FRACDIG);
    }
    c.cycles += 1;
    c.g[V0][0] = 0x65;
    Ok(STRTOD_EXP)
}

/// 0x419b70: e = 0; an 'e'/'E' after some digits (or zeros) starts the
/// exponent; with nothing read the text is no number.
pub(crate) fn strtod_exp(c: &mut Cpu) -> u32 {
    c.cycles += 2;
    let lower = c.g[A0][0] == c.g[V0][0];
    c.sd(strtod_at(c, STRTOD_E), 0);
    if !lower {
        c.cycles += 3;
        c.g[V0][0] = 0x45;
        if c.g[A0][0] != c.g[V0][0] {
            return STRTOD_DIGEND;
        }
    }
    for reg in [S4, A2, T2] {
        c.cycles += 2;
        if c.g[reg][0] != 0 {
            c.g[A1][0] = c.g[S5][0];
            return STRTOD_EXPSIGN;
        }
    }
    c.cycles += 2;
    c.g[S5][0] = c.g[A1][0];
    STRTOD_OUT
}

/// 0x419ba4: s00 (a1) = s; the exponent's sign (esign, t0) and first digit.
pub(crate) fn strtod_expsign(c: &mut Cpu) -> u32 {
    c.cycles += 5;
    strtod_addiu(c, S5, S5, 1);
    c.g[V0][0] = 0x2b;
    c.g[A0][0] = c.lb(r32(c, S5));
    let plus = c.g[A0][0] == c.g[V0][0];
    c.g[T0][0] = 0;
    let mut signed = plus;
    if !plus {
        c.cycles += 3;
        c.g[V0][0] = 0x2d;
        let minus = c.g[A0][0] == c.g[V0][0];
        c.g[V0][0] = sx(r32(c, A0).wrapping_sub(0x30));
        if minus {
            c.cycles += 1;
            c.g[T0][0] = 1;
            signed = true;
        }
    }
    if signed {
        c.cycles += 3;
        strtod_addiu(c, S5, S5, 1);
        c.g[A0][0] = c.lb(r32(c, S5));
        c.g[V0][0] = sx(r32(c, A0).wrapping_sub(0x30));
    }
    // 0x419bd4
    c.cycles += 3;
    let digit = c.g[V0][0] < 0xa;
    c.g[V0][0] = 0x30;
    if !digit {
        // No exponent digits: back to the 'e'.
        c.cycles += 1;
        c.g[S5][0] = c.g[A1][0];
        return STRTOD_DIGEND;
    }
    c.cycles += 2;
    if c.g[A0][0] != c.g[V0][0] {
        c.g[V0][0] = sx(r32(c, A0).wrapping_sub(0x31));
        return STRTOD_EXPDIGS;
    }
    c.cycles += 2;
    strtod_addiu(c, S5, S5, 1);
    STRTOD_EXPZ
}

/// 0x419bf0 (head): skip the exponent's leading zeros.
pub(crate) fn strtod_expz(c: &mut Cpu) -> StrtodStep {
    c.cycles += 7;
    c.g[A0][0] = c.lb(r32(c, S5));
    if c.g[A0][0] == c.g[V0][0] {
        strtod_addiu(c, S5, S5, 1);
        return strtod_back(c, STRTOD_EXPZ);
    }
    c.cycles += 1;
    c.g[V0][0] = sx(r32(c, A0).wrapping_sub(0x31));
    Ok(STRTOD_EXPDIGS)
}

/// 0x419c10: the first non-zero exponent digit (or e = 0).
pub(crate) fn strtod_expdigs(c: &mut Cpu) -> u32 {
    c.cycles += 3;
    let digit = c.g[V0][0] < 9;
    c.g[V0][0] = digit as u64;
    strtod_addiu(c, A0, A0, 0u32.wrapping_sub(0x30));
    if !digit {
        c.cycles += 2;
        c.sd(strtod_at(c, STRTOD_E), 0);
        return STRTOD_DIGEND;
    }
    c.cycles += 7;
    c.g[A3][0] = c.g[S5][0];
    c.sd(strtod_at(c, STRTOD_E), c.g[A0][0]);
    strtod_addiu(c, S5, S5, 1);
    c.g[A0][0] = c.lb(r32(c, S5));
    let below = (c.g[A0][0] as i64) < 48;
    c.g[V0][0] = below as u64;
    c.g[V0][0] = sx(r32(c, S5).wrapping_sub(r32(c, A3)));
    if below {
        return STRTOD_EXPCLAMP;
    }
    STRTOD_EXPDIG
}

/// 0x419c38 (head): e = 10 * e + c - '0' (64-bit).
pub(crate) fn strtod_expdig(c: &mut Cpu) -> StrtodStep {
    c.cycles += 3;
    let digit = (c.g[A0][0] as i64) < 58;
    c.g[V0][0] = digit as u64;
    c.g[V1][0] = c.ld(strtod_at(c, STRTOD_E));
    if digit {
        c.cycles += 10;
        strtod_addiu(c, S5, S5, 1);
        c.g[V0][0] = c.g[V1][0] << 2;
        c.g[V0][0] = c.g[V0][0].wrapping_add(c.g[V1][0]);
        c.g[V0][0] <<= 1;
        c.g[V0][0] = c.g[A0][0].wrapping_add(c.g[V0][0]);
        c.g[A0][0] = c.lb(r32(c, S5));
        c.g[V0][0] = c.g[V0][0].wrapping_sub(0x30);
        let below = (c.g[A0][0] as i64) < 48;
        c.g[V1][0] = below as u64;
        c.sd(strtod_at(c, STRTOD_E), c.g[V0][0]);
        if !below {
            return strtod_back(c, STRTOD_EXPDIG);
        }
        c.cycles += 1;
    }
    c.cycles += 1;
    c.g[V0][0] = sx(r32(c, S5).wrapping_sub(r32(c, A3)));
    Ok(STRTOD_EXPCLAMP)
}

/// 0x419c74: more than eight exponent digits clamp e to 9999999; esign
/// negates it.
pub(crate) fn strtod_expclamp(c: &mut Cpu) -> u32 {
    c.cycles += 9;
    c.g[A0][0] = 0x98_967f;
    c.g[A3][0] = c.ld(strtod_at(c, STRTOD_E));
    c.g[V0][0] = ((c.g[V0][0] as i64) < 9) as u64;
    if c.g[V0][0] == 0 {
        c.g[A3][0] = c.g[A0][0];
    }
    c.g[V1][0] = 0u64.wrapping_sub(c.g[A3][0]);
    if c.g[T0][0] != 0 {
        c.g[A3][0] = c.g[V1][0];
    }
    c.sd(strtod_at(c, STRTOD_E), c.g[A3][0]);
    STRTOD_DIGEND
}

/// 0x419ca4: no digits: 0 (with the end after the zeros, or the start).
pub(crate) fn strtod_digend(c: &mut Cpu) -> u32 {
    c.cycles += 2;
    let have_digits = c.g[S4][0] != 0;
    c.g[V1][0] = c.ld(strtod_at(c, STRTOD_E));
    if have_digits {
        return STRTOD_CONVERT;
    }
    c.cycles += 3;
    if c.g[T2][0] != 0 {
        c.g[A1][0] = c.g[S5][0];
    }
    if c.g[A2][0] == 0 {
        c.g[S5][0] = c.g[A1][0];
    }
    STRTOD_OUT
}

/// 0x419cb8: e1 = e -= nf; k = min(nd, 16); rv = y (+2^32 if negative);
/// above nine digits rv = tens[k-9] * rv + z. Up to fifteen digits and a
/// small exponent the value is exact: one multiply or divide by tens[].
pub(crate) fn strtod_convert(c: &mut Cpu) -> StrtodStep {
    c.cycles += 13;
    let short = (c.g[S4][0] as i64) < 17;
    c.g[A0][0] = c.lw(strtod_at(c, STRTOD_Y));
    c.g[S3][0] = 0x10;
    c.g[V1][0] = c.g[V1][0].wrapping_sub(c.g[T1][0]);
    if short {
        c.g[S3][0] = c.g[S4][0];
    }
    c.g[S0][0] = strtod_sext32(c.g[V1][0]);
    c.sd(strtod_at(c, STRTOD_E), c.g[V1][0]);
    STRTOD_FLOATSIDF.call(c, 0x0041_9cdc)?;
    if c.g[FP][0] == 0 {
        c.g[FP][0] = c.g[S4][0];
    }
    c.g[A3][0] = c.lw(strtod_at(c, STRTOD_Y));
    let unsigned_fix = (c.g[A3][0] as i64) < 0;
    c.g[A0][0] = c.g[V0][0];
    if unsigned_fix {
        c.cycles += 5;
        c.g[A1][0] = STRTOD_TWO32;
        STRTOD_ADDDF3.call(c, 0x0041_9cfc)?;
        c.g[A0][0] = c.g[V0][0];
    }
    // 0x419d00
    c.cycles += 3;
    let le9 = (c.g[S3][0] as i64) < 10;
    c.g[S1][0] = c.g[A0][0];
    if !le9 {
        c.cycles += 10;
        c.g[V1][0] = sx(STRTOD_TENS);
        strtod_addiu(c, V0, S3, 0u32.wrapping_sub(9));
        c.g[V0][0] = sx(r32(c, V0) << 3);
        strtod_addu(c, V0, V0, V1);
        c.g[A1][0] = c.g[A0][0];
        c.g[A0][0] = c.ld(r32(c, V0));
        STRTOD_MULDF3.call(c, 0x0041_9d2c)?;
        let z_neg = (c.g[S2][0] as i64) < 0;
        c.g[S1][0] = c.g[V0][0];
        if !z_neg {
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            STRTOD_FLOATDIDF.call(c, 0x0041_9d3c)?;
            c.g[A0][0] = c.g[S1][0];
        } else {
            // Unsigned: (z >> 1 | z & 1) * 2.
            c.cycles += 8;
            c.g[V0][0] = c.g[S2][0] >> 1;
            c.g[A0][0] = c.g[S2][0] & 1;
            c.g[A0][0] |= c.g[V0][0];
            STRTOD_FLOATDIDF.call(c, 0x0041_9d54)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[A0][0];
            STRTOD_ADDDF3.call(c, 0x0041_9d60)?;
            c.g[A0][0] = c.g[S1][0];
        }
        c.cycles += 3;
        c.g[A1][0] = c.g[V0][0];
        STRTOD_ADDDF3.call(c, 0x0041_9d6c)?;
        c.g[S1][0] = c.g[V0][0];
    }
    // 0x419d70: bd0 = 0.
    c.cycles += 3;
    let fast = (c.g[S4][0] as i64) < 16;
    c.sw(strtod_at(c, STRTOD_BD0), 0);
    if !fast {
        c.cycles += 1;
        c.g[V0][0] = sx(r32(c, S4).wrapping_sub(r32(c, S3)));
        return Ok(STRTOD_SCALE);
    }
    c.cycles += 3;
    c.g[V0][0] = c.ld(strtod_at(c, STRTOD_E));
    let no_exp = c.g[V0][0] == 0;
    c.g[A0][0] = c.lw(strtod_at(c, STRTOD_SE));
    if no_exp {
        return Ok(STRTOD_OUT2);
    }
    c.cycles += 2;
    let negative = (c.g[V0][0] as i64) <= 0;
    c.g[V0][0] = ((c.g[V0][0] as i64) < 23) as u64;
    if negative {
        // 0x419e20: e >= -22: rv /= tens[-e].
        c.cycles += 4;
        c.g[A3][0] = c.ld(strtod_at(c, STRTOD_E));
        let tiny = (c.g[A3][0] as i64) < -22;
        c.g[V0][0] = tiny as u64;
        c.g[V0][0] = sx(r32(c, S4).wrapping_sub(r32(c, S3)));
        if tiny {
            return Ok(STRTOD_SCALE);
        }
        c.cycles += 11;
        c.g[V1][0] = sx((strtod_sext32(c.g[A3][0]) as u32) << 3);
        c.g[V0][0] = sx(STRTOD_TENS.wrapping_sub(r32(c, V1)));
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.ld(r32(c, V0));
        STRTOD_DIVDF3.call(c, 0x0041_9e54)?;
        c.g[S1][0] = c.g[V0][0];
        return Ok(STRTOD_OUT);
    }
    c.cycles += 2;
    let small = c.g[V0][0] != 0;
    c.g[A0][0] = c.ld(strtod_at(c, STRTOD_E));
    if small {
        // e <= 22: rv *= tens[e].
        c.cycles += 8;
        c.g[V0][0] = sx(STRTOD_TENS);
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = sx((strtod_sext32(c.g[A0][0]) as u32) << 3);
        strtod_addu(c, V1, V1, V0);
    } else {
        // e <= 15 - nd + 22: rv *= tens[15 - nd], then tens[e - (15 - nd)].
        c.cycles += 7;
        c.g[A3][0] = sx(0xfu32.wrapping_sub(r32(c, S4)));
        c.g[V1][0] = c.ld(strtod_at(c, STRTOD_E));
        strtod_addiu(c, V0, A3, 0x16);
        let too_big = (c.g[V0][0] as i64) < (c.g[V1][0] as i64);
        c.g[V0][0] = too_big as u64;
        c.g[V0][0] = sx(r32(c, S4).wrapping_sub(r32(c, S3)));
        if too_big {
            return Ok(STRTOD_SCALE);
        }
        c.cycles += 15;
        c.g[S0][0] = sx(STRTOD_TENS);
        c.g[V0][0] = sx(r32(c, A3) << 3);
        c.g[V1][0] = c.g[V1][0].wrapping_sub(c.g[A3][0]);
        strtod_addu(c, V0, V0, S0);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.ld(r32(c, V0));
        c.sd(strtod_at(c, STRTOD_E), c.g[V1][0]);
        STRTOD_MULDF3.call(c, 0x0041_9df8)?;
        c.g[A0][0] = c.ld(strtod_at(c, STRTOD_E));
        c.g[A1][0] = c.g[V0][0];
        c.g[V1][0] = sx((strtod_sext32(c.g[A0][0]) as u32) << 3);
        strtod_addu(c, V1, V1, S0);
    }
    // 0x419e10
    c.cycles += 4;
    c.g[A0][0] = c.ld(r32(c, V1));
    STRTOD_MULDF3.call(c, 0x0041_9e18)?;
    c.g[S1][0] = c.g[V0][0];
    Ok(STRTOD_OUT)
}

/// 0x419e60: e1 += nd - k; a positive e1 multiplies by tens[e1 & 15] and
/// goes on with the powers of 1e16 (overflow past 1e308).
pub(crate) fn strtod_scale(c: &mut Cpu) -> StrtodStep {
    c.cycles += 3;
    strtod_addu(c, S0, S0, V0);
    let not_positive = (c.g[S0][0] as i64) <= 0;
    c.g[A3][0] = c.g[S0][0] & 0xf;
    if not_positive {
        return Ok(STRTOD_NEG);
    }
    c.cycles += 2;
    let low = c.g[A3][0] != 0;
    c.g[V0][0] = 0x4a_0000;
    if low {
        c.cycles += 7;
        c.g[V1][0] = sx(r32(c, A3) << 3);
        c.g[V0][0] = sx(STRTOD_TENS);
        c.g[A1][0] = c.g[S1][0];
        strtod_addu(c, V1, V1, V0);
        c.g[A0][0] = c.ld(r32(c, V1));
        STRTOD_MULDF3.call(c, 0x0041_9e8c)?;
        c.g[S1][0] = c.g[V0][0];
    }
    // 0x419e90
    c.cycles += 4;
    c.g[V0][0] = 0xffff_ffff_ffff_fff0;
    c.g[S0][0] &= c.g[V0][0];
    let done = c.g[S0][0] == 0;
    let in_range = (c.g[S0][0] as i64) < 309;
    c.g[V0][0] = in_range as u64;
    if done {
        return Ok(STRTOD_BIGCOMP0);
    }
    c.cycles += 2;
    if in_range {
        c.g[S0][0] = sx(((r32(c, S0) as i32) >> 4) as u32);
        return Ok(STRTOD_BIGPOW);
    }
    Ok(STRTOD_OVFL)
}

/// 0x419eac: ERANGE, rv = HUGE_VAL; free the bigints if there are any.
pub(crate) fn strtod_ovfl2(c: &mut Cpu) -> u32 {
    c.cycles += 5;
    c.g[V1][0] = 0x4a_0000;
    c.sw(r32(c, S7), c.g[V0][0]);
    c.g[V0][0] = c.lw(strtod_at(c, STRTOD_BD0));
    let have_bigints = c.g[V0][0] != 0;
    c.g[S1][0] = c.ld(STRTOD_HUGE);
    if have_bigints {
        return STRTOD_FREE;
    }
    c.cycles += 2;
    c.g[A0][0] = c.lw(strtod_at(c, STRTOD_SE));
    STRTOD_OUT2
}

/// 0x419ec8: e1 >>= 4 (done above); j = 0 (s3).
pub(crate) fn strtod_bigpow(c: &mut Cpu) -> u32 {
    c.cycles += 2;
    let none = c.g[S0][0] == 0;
    c.g[S3][0] = 0;
    if none {
        return STRTOD_BIGCOMP0;
    }
    c.cycles += 3;
    let last = (c.g[S0][0] as i64) < 2;
    c.g[V0][0] = last as u64;
    c.g[S6][0] = 0x4a_0000;
    if last {
        return STRTOD_BIGLAST;
    }
    c.cycles += 1;
    strtod_addiu(c, S2, S6, 0u32.wrapping_sub(0x5640));
    STRTOD_BIGLOOP
}

/// 0x419ee0 / 0x41a008 (heads): for each set bit of e1 multiply by the
/// table entry at s2 (`bigtens[]` or `tinytens[]`), j++.
pub(crate) fn strtod_powloop(c: &mut Cpu, head: u32, ret_addr: u32, last: u32) -> StrtodStep {
    c.cycles += 3;
    c.g[V0][0] = c.g[S0][0] & 1;
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.g[A0][0] = c.ld(r32(c, S2));
        c.g[A1][0] = c.g[S1][0];
        STRTOD_MULDF3.call(c, ret_addr)?;
        c.g[S1][0] = c.g[V0][0];
    }
    c.g[S0][0] = sx(((r32(c, S0) as i32) >> 1) as u32);
    c.cycles += 4;
    strtod_addiu(c, S2, S2, 8);
    let more = (c.g[S0][0] as i64) >= 2;
    c.g[V0][0] = (!more) as u64;
    strtod_addiu(c, S3, S3, 1);
    if more {
        return strtod_back(c, head);
    }
    Ok(last)
}

/// 0x419f10: the last factor with the exponent biased down by 53*2^20
/// (so the product cannot overflow), then check and rebias.
pub(crate) fn strtod_biglast(c: &mut Cpu) -> StrtodStep {
    c.cycles += 21;
    strtod_addiu(c, A0, S6, 0u32.wrapping_sub(0x5640));
    c.g[A1][0] = strtod_hi(c.g[S1][0]);
    c.g[V0][0] = 0xffff_ffff_fcb0_0000;
    c.g[V1][0] = sx(r32(c, S3) << 3);
    strtod_addu(c, V1, V1, A0);
    strtod_addu(c, A1, A1, V0);
    c.g[S0][0] = STRTOD_LOW;
    c.g[A0][0] = c.ld(r32(c, V1));
    c.g[A1][0] <<= 32;
    c.g[S1][0] &= c.g[S0][0];
    c.g[A1][0] |= c.g[S1][0];
    STRTOD_MULDF3.call(c, 0x0041_9f44)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[V1][0] = 0x7ff0_0000;
    c.g[A0][0] = strtod_hi(c.g[S1][0]);
    c.g[V0][0] = 0x7ca0_0000;
    c.g[S2][0] = c.g[A0][0] & c.g[V1][0];
    let ovfl = c.g[V0][0] < c.g[S2][0];
    c.g[V0][0] = STRTOD_ERANGE;
    if ovfl {
        return strtod_back(c, STRTOD_OVFL2);
    }
    c.cycles += 4;
    c.g[V0][0] = 0x7c90_0000;
    let near_max = c.g[V0][0] < c.g[S2][0];
    c.g[V0][0] = 0x350_0000;
    if near_max {
        // rv = Big0:Big1
        c.cycles += 8;
        c.g[S1][0] &= c.g[S0][0];
        c.g[V0][0] = STRTOD_BIG_HIGH;
        c.g[S1][0] |= c.g[V0][0];
        c.g[S1][0] |= c.g[S0][0];
    } else {
        c.cycles += 5;
        c.g[S1][0] &= c.g[S0][0];
        strtod_addu(c, V0, A0, V0);
        c.g[V0][0] <<= 32;
        c.g[S1][0] |= c.g[V0][0];
    }
    Ok(STRTOD_BIGCOMP0)
}

/// 0x419fa8: a negative e1 divides by tens[-e1 & 15] and goes on with the
/// powers of 1e-16 (underflow past 1e-512).
pub(crate) fn strtod_neg(c: &mut Cpu) -> StrtodStep {
    c.cycles += 2;
    let zero = (c.g[S0][0] as i64) >= 0;
    c.g[A1][0] = c.lw(strtod_at(c, STRTOD_S0));
    if zero {
        return Ok(STRTOD_BIGCOMP1);
    }
    c.cycles += 4;
    c.g[S0][0] = sx(0u32.wrapping_sub(r32(c, S0)));
    c.g[A3][0] = c.g[S0][0] & 0xf;
    let low = c.g[A3][0] != 0;
    c.g[V0][0] = 0x4a_0000;
    if low {
        c.cycles += 7;
        c.g[V1][0] = sx(r32(c, A3) << 3);
        c.g[V0][0] = sx(STRTOD_TENS);
        c.g[A0][0] = c.g[S1][0];
        strtod_addu(c, V1, V1, V0);
        c.g[A1][0] = c.ld(r32(c, V1));
        STRTOD_DIVDF3.call(c, 0x0041_9fd8)?;
        c.g[S1][0] = c.g[V0][0];
    }
    // 0x419fdc
    c.cycles += 4;
    c.g[V0][0] = 0xffff_ffff_ffff_fff0;
    c.g[S0][0] &= c.g[V0][0];
    let done = c.g[S0][0] == 0;
    c.g[S0][0] = sx(((r32(c, S0) as i32) >> 4) as u32);
    if done {
        return Ok(STRTOD_BIGCOMP0);
    }
    c.cycles += 3;
    let in_range = (c.g[S0][0] as i64) < 32;
    c.g[V0][0] = in_range as u64;
    c.g[S3][0] = 0;
    if !in_range {
        return Ok(STRTOD_UNDFL);
    }
    c.cycles += 3;
    let last = (c.g[S0][0] as i64) < 2;
    c.g[V0][0] = last as u64;
    c.g[S6][0] = 0x4a_0000;
    if last {
        return Ok(STRTOD_TINYLAST);
    }
    c.cycles += 1;
    strtod_addiu(c, S2, S6, 0u32.wrapping_sub(0x5618));
    Ok(STRTOD_TINYLOOP)
}

/// 0x41a038: the last tiny factor; if the product is 0, retry with 2*rv
/// (s1 = 1 marks that), and if that is 0 too it underflowed.
pub(crate) fn strtod_tinylast(c: &mut Cpu) -> StrtodStep {
    c.cycles += 15;
    strtod_addiu(c, V0, S6, 0u32.wrapping_sub(0x5618));
    c.g[V1][0] = sx(r32(c, S3) << 3);
    strtod_addu(c, V1, V1, V0);
    c.g[A1][0] = c.g[S1][0];
    c.g[S0][0] = c.ld(r32(c, V1));
    c.g[S6][0] = c.g[S1][0];
    c.g[S2][0] = 0;
    c.g[A0][0] = c.g[S0][0];
    STRTOD_MULDF3.call(c, 0x0041_a05c)?;
    c.g[A1][0] = c.g[S2][0];
    c.g[A0][0] = c.g[V0][0];
    c.g[S1][0] = c.g[V0][0];
    STRTOD_CMPDF2.call(c, 0x0041_a06c)?;
    let nonzero = c.g[V0][0] != 0;
    c.g[A1][0] = c.lw(strtod_at(c, STRTOD_S0));
    if nonzero {
        return Ok(STRTOD_BIGCOMP1);
    }
    c.cycles += 12;
    c.g[A0][0] = c.g[S6][0];
    c.g[A1][0] = c.g[A0][0];
    STRTOD_ADDDF3.call(c, 0x0041_a080)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    STRTOD_MULDF3.call(c, 0x0041_a08c)?;
    c.g[A1][0] = c.g[S2][0];
    c.g[A0][0] = c.g[V0][0];
    c.g[S1][0] = c.g[V0][0];
    STRTOD_CMPDF2.call(c, 0x0041_a09c)?;
    let nonzero = c.g[V0][0] != 0;
    c.g[S1][0] = 1;
    if nonzero {
        return Ok(STRTOD_BIGCOMP0);
    }
    Ok(STRTOD_UNDFL)
}

/// 0x41a0a8: ERANGE, rv = 0; free the bigints if there are any.
pub(crate) fn strtod_undfl2(c: &mut Cpu) -> u32 {
    c.cycles += 4;
    c.sw(r32(c, S7), c.g[V0][0]);
    c.g[V1][0] = c.lw(strtod_at(c, STRTOD_BD0));
    let have_bigints = c.g[V1][0] != 0;
    c.g[S1][0] = 0;
    if have_bigints {
        return STRTOD_FREE;
    }
    c.cycles += 2;
    c.g[A0][0] = c.lw(strtod_at(c, STRTOD_SE));
    STRTOD_OUT2
}

/// 0x41a0c0 (head): rv += ulp(rv), done.
pub(crate) fn strtod_ulpadd(c: &mut Cpu) -> StrtodStep {
    c.cycles += 7;
    c.g[A0][0] = c.g[S1][0];
    STRTOD_ULP.call(c, 0x0041_a0c8)?;
    c.g[A1][0] = c.g[S1][0];
    c.g[A0][0] = c.g[V0][0];
    STRTOD_ADDDF3.call(c, 0x0041_a0d4)?;
    c.g[S1][0] = c.g[V0][0];
    Ok(STRTOD_FREE)
}

/// 0x41a0e0: bd0 = s2b(ptr, s0, nd0, nd, y); remember e and &bbbits.
pub(crate) fn strtod_bigcomp(c: &mut Cpu) -> StrtodStep {
    c.cycles += 13;
    c.g[A2][0] = c.g[FP][0];
    c.g[T0][0] = c.lw(strtod_at(c, STRTOD_Y));
    c.g[A3][0] = c.g[S4][0];
    c.g[A0][0] = c.g[S7][0];
    STRTOD_S2B.call(c, 0x0041_a0f4)?;
    c.sw(strtod_at(c, STRTOD_BD0), c.g[V0][0]);
    c.g[A0][0] = strtod_sext32(c.ld(strtod_at(c, STRTOD_E)));
    c.g[A3][0] = sx(sp(c).wrapping_add(4));
    c.sw(strtod_at(c, STRTOD_EINT), c.g[A0][0]);
    c.sw(strtod_at(c, STRTOD_BBBITS_PTR), c.g[A3][0]);
    Ok(STRTOD_LOOPTOP)
}

/// 0x41a118 (head): exactly a power of two below: delta <<= 1; if it is
/// above bs, round down.
pub(crate) fn strtod_lshift1(c: &mut Cpu) -> StrtodStep {
    c.cycles += 9;
    c.g[A0][0] = c.g[S7][0];
    c.g[A2][0] = 1;
    STRTOD_LSHIFT.call(c, 0x0041_a124)?;
    c.sw(strtod_at(c, STRTOD_DELTA), c.g[V0][0]);
    c.g[A1][0] = c.lw(strtod_at(c, STRTOD_BS));
    c.g[A0][0] = c.g[V0][0];
    STRTOD_MCMP.call(c, 0x0041_a134)?;
    let above = (c.g[V0][0] as i64) > 0;
    c.g[A1][0] = c.lw(strtod_at(c, STRTOD_BB));
    if above {
        return Ok(STRTOD_ROUNDDOWN);
    }
    c.cycles += 2;
    Ok(STRTOD_FREE1)
}

/// 0x41a144 (head): delta >= bs (a3 = cmp): above goes to the ratio; equal
/// is the halfway case.
pub(crate) fn strtod_cmp(c: &mut Cpu) -> u32 {
    c.cycles += 2;
    let above = c.g[A3][0] != 0;
    c.g[A0][0] = c.lw(strtod_at(c, STRTOD_DELTA));
    if above {
        return STRTOD_RATIOSTEP;
    }
    c.cycles += 2;
    if c.g[S4][0] == 0 {
        // 0x41a19c: !dsign: a zero mantissa rounds down.
        c.cycles += 5;
        c.g[V0][0] = STRTOD_FRAC;
        c.g[V0][0] &= c.g[S1][0];
        if c.g[V0][0] != 0 {
            return STRTOD_ODD;
        }
        return STRTOD_ROUNDDOWN;
    }
    // dsign: an all-ones mantissa rounds up to the next power of two.
    c.cycles += 5;
    c.g[V1][0] = STRTOD_FRAC;
    c.g[V0][0] = c.g[S1][0] & c.g[V1][0];
    let not_full = c.g[V0][0] != c.g[V1][0];
    c.g[V1][0] = 0x7ff0_0000;
    if not_full {
        return STRTOD_ODD;
    }
    c.cycles += 13;
    c.g[V0][0] = strtod_hi(c.g[S1][0]);
    c.g[V0][0] &= c.g[V1][0];
    c.g[A0][0] = 0x10_0000;
    c.g[V1][0] = STRTOD_LOW;
    c.g[V0][0] = c.g[V0][0].wrapping_add(c.g[A0][0]);
    c.g[S1][0] &= c.g[V1][0];
    c.g[V0][0] <<= 32;
    c.g[S1][0] |= c.g[V0][0];
    c.g[V1][0] = u64::MAX << 32;
    c.g[S1][0] &= c.g[V1][0];
    STRTOD_FREE
}

/// 0x41a1b0: rv = (exponent - 1) with an all-ones mantissa.
pub(crate) fn strtod_rounddown(c: &mut Cpu) -> u32 {
    c.cycles += 17;
    c.g[V0][0] = strtod_hi(c.g[S1][0]);
    c.g[V1][0] = 0x7ff0_0000;
    c.g[V0][0] &= c.g[V1][0];
    c.g[A0][0] = 0xffff_ffff_fff0_0000;
    strtod_addu(c, V0, V0, A0);
    c.g[A1][0] = STRTOD_LOW;
    c.g[V0][0] <<= 32;
    c.g[V1][0] = 0x000f_ffff_0000_0000;
    c.g[V1][0] |= c.g[V0][0];
    c.g[S1][0] &= c.g[A1][0];
    c.g[S1][0] |= c.g[V1][0];
    c.g[S1][0] |= c.g[A1][0];
    STRTOD_FREE
}

/// 0x41a1f4: halfway with an odd mantissa: round to even (up with dsign,
/// else down, underflowing at 0).
pub(crate) fn strtod_odd(c: &mut Cpu) -> StrtodStep {
    c.cycles += 5;
    c.g[V0][0] = strtod_sext32(c.g[S1][0]) & 1;
    let even = c.g[V0][0] == 0;
    c.g[A1][0] = c.lw(strtod_at(c, STRTOD_BB));
    if even {
        return Ok(STRTOD_FREE1);
    }
    c.cycles += 2;
    if c.g[S4][0] != 0 {
        return strtod_back(c, STRTOD_ULPADD);
    }
    c.cycles += 11;
    c.g[A0][0] = c.g[S1][0];
    STRTOD_ULP.call(c, 0x0041_a218)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[V0][0];
    STRTOD_SUBDF3.call(c, 0x0041_a224)?;
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[V0][0];
    c.g[S1][0] = c.g[V0][0];
    STRTOD_CMPDF2.call(c, 0x0041_a234)?;
    let zero = c.g[V0][0] == 0;
    c.g[A1][0] = c.lw(strtod_at(c, STRTOD_BB));
    if zero {
        return strtod_back(c, STRTOD_UNDFL);
    }
    c.cycles += 2;
    Ok(STRTOD_FREE1)
}

/// 0x41a244: aadj (s0) = ratio(delta, bs); aadj1 (s2) the signed step.
pub(crate) fn strtod_ratio(c: &mut Cpu) -> StrtodStep {
    c.cycles += 9;
    c.g[A1][0] = c.lw(strtod_at(c, STRTOD_BS));
    STRTOD_RATIO.call(c, 0x0041_a24c)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = STRTOD_TWO;
    c.g[A0][0] = c.g[S0][0];
    STRTOD_CMPDF2.call(c, 0x0041_a260)?;
    if (c.g[V0][0] as i64) > 0 {
        // 0x41a2f4: aadj *= 0.5; aadj1 = dsign ? aadj : -aadj.
        c.cycles += 7;
        c.g[A1][0] = STRTOD_HALF;
        c.g[A0][0] = c.g[S0][0];
        STRTOD_MULDF3.call(c, 0x0041_a304)?;
        c.g[S0][0] = c.g[V0][0];
        let dsign = c.g[S4][0] != 0;
        c.g[S2][0] = c.g[S0][0];
        if dsign {
            return Ok(STRTOD_ADJ);
        }
        c.cycles += 1;
        c.g[A0][0] = 0;
        return Ok(STRTOD_NEGADJ);
    }
    c.cycles += 2;
    if c.g[S4][0] != 0 {
        // aadj = aadj1 = 1.
        c.cycles += 4;
        c.g[S0][0] = STRTOD_ONE;
        c.g[S2][0] = c.g[S0][0];
        return Ok(STRTOD_ADJ);
    }
    c.cycles += 5;
    c.g[V0][0] = STRTOD_FRAC;
    c.g[V0][0] &= c.g[S1][0];
    let frac_zero = c.g[V0][0] == 0;
    c.g[V0][0] = 1;
    if !frac_zero {
        c.cycles += 2;
        let smallest = c.g[S1][0] == c.g[V0][0];
        c.g[V0][0] = STRTOD_ERANGE;
        if smallest {
            return strtod_back(c, STRTOD_UNDFL2);
        }
        // aadj = 1, aadj1 = -1.
        c.cycles += 6;
        c.g[S0][0] = STRTOD_ONE;
        c.g[S2][0] = STRTOD_MINUS_ONE;
        return Ok(STRTOD_ADJ);
    }
    // 0x41a2b4: aadj = aadj < 1 ? 0.5 : aadj * 0.5; aadj1 = -aadj.
    c.cycles += 6;
    c.g[A1][0] = STRTOD_ONE;
    c.g[A0][0] = c.g[S0][0];
    STRTOD_CMPDF2.call(c, 0x0041_a2c4)?;
    let at_least_one = (c.g[V0][0] as i64) >= 0;
    c.g[A0][0] = 0;
    if !at_least_one {
        c.cycles += 4;
        c.g[S0][0] = STRTOD_HALF;
        return Ok(STRTOD_NEGADJ);
    }
    c.cycles += 6;
    c.g[A1][0] = STRTOD_HALF;
    c.g[A0][0] = c.g[S0][0];
    STRTOD_MULDF3.call(c, 0x0041_a2ec)?;
    c.g[S0][0] = c.g[V0][0];
    c.cycles += 1;
    c.g[A0][0] = 0;
    Ok(STRTOD_NEGADJ)
}

/// 0x41a320: rv += aadj1 * ulp(rv), with the exponent biased down at the
/// top of the range (overflow check) and aadj rounded to an integer in the
/// low range; then decide whether rv is close enough.
pub(crate) fn strtod_adj(c: &mut Cpu) -> StrtodStep {
    c.cycles += 8;
    c.g[V1][0] = strtod_hi(c.g[S1][0]);
    c.g[FP][0] = 0x7ff0_0000;
    c.g[V0][0] = c.g[V1][0] & c.g[FP][0];
    c.sw(strtod_at(c, STRTOD_Y), c.g[V0][0]);
    c.g[V0][0] = 0x7fe0_0000;
    c.g[A0][0] = c.lw(strtod_at(c, STRTOD_Y));
    let at_top = c.g[A0][0] == c.g[V0][0];
    c.g[A3][0] = c.lw(strtod_at(c, STRTOD_Y));
    if at_top {
        // 0x41a340: rv0 = rv (s6); bias down by 53*2^20.
        c.cycles += 24;
        c.g[V0][0] = 0xffff_ffff_fcb0_0000;
        c.g[S6][0] = c.g[S1][0];
        strtod_addu(c, V0, V1, V0);
        c.g[S3][0] = STRTOD_LOW;
        c.g[V0][0] <<= 32;
        c.g[S1][0] &= c.g[S3][0];
        c.g[S1][0] |= c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        STRTOD_ULP.call(c, 0x0041_a368)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[V0][0];
        STRTOD_MULDF3.call(c, 0x0041_a374)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        STRTOD_ADDDF3.call(c, 0x0041_a380)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = 0x7c9f_ffff;
        c.g[A0][0] = strtod_hi(c.g[S1][0]);
        c.g[V0][0] = c.g[A0][0] & c.g[FP][0];
        let past = c.g[V1][0] < c.g[V0][0];
        c.g[V1][0] = past as u64;
        c.g[V0][0] = 0x350_0000;
        if !past {
            // Rebias.
            c.cycles += 5;
            c.g[S1][0] &= c.g[S3][0];
            strtod_addu(c, V0, A0, V0);
            c.g[V0][0] <<= 32;
            c.g[S1][0] |= c.g[V0][0];
        } else {
            c.cycles += 9;
            c.g[V0][0] = STRTOD_BIG;
            let was_max = c.g[S6][0] == c.g[V0][0];
            c.g[S1][0] &= c.g[S3][0];
            if was_max {
                return strtod_back(c, STRTOD_OVFL);
            }
            // rv = Big0:Big1, and go round again.
            c.cycles += 7;
            c.g[V0][0] = STRTOD_BIG_HIGH;
            c.g[S1][0] |= c.g[V0][0];
            c.g[S1][0] |= c.g[S3][0];
            c.cycles += 1;
            c.g[A1][0] = c.lw(strtod_at(c, STRTOD_BB));
            return Ok(STRTOD_NEXT);
        }
    } else {
        // 0x41a3f4: in the low range with aadj >= 1, round aadj to an
        // integer (aadj1 = dsign ? aadj : -aadj).
        c.cycles += 4;
        c.g[V0][0] = 0x340_0000;
        let high = c.g[V0][0] < c.g[A3][0];
        c.g[V0][0] = high as u64;
        if !high {
            c.cycles += 6;
            c.g[A1][0] = STRTOD_ONE;
            c.g[A0][0] = c.g[S0][0];
            STRTOD_CMPDF2.call(c, 0x0041_a414)?;
            if (c.g[V0][0] as i64) >= 0 {
                c.cycles += 10;
                c.g[A1][0] = STRTOD_HALF;
                c.g[A0][0] = c.g[S0][0];
                STRTOD_ADDDF3.call(c, 0x0041_a42c)?;
                c.g[A0][0] = c.g[V0][0];
                STRTOD_FIXDFSI.call(c, 0x0041_a434)?;
                c.g[A0][0] = c.g[V0][0];
                STRTOD_FLOATSIDF.call(c, 0x0041_a43c)?;
                let dsign = c.g[S4][0] != 0;
                c.g[S2][0] = c.g[V0][0];
                if !dsign {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = 0;
                    STRTOD_SUBDF3.call(c, 0x0041_a450)?;
                    c.g[S2][0] = c.g[V0][0];
                }
            }
        }
        // 0x41a454: rv += aadj1 * ulp(rv).
        c.cycles += 9;
        c.g[A0][0] = c.g[S1][0];
        STRTOD_ULP.call(c, 0x0041_a45c)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[V0][0];
        STRTOD_MULDF3.call(c, 0x0041_a468)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        STRTOD_ADDDF3.call(c, 0x0041_a474)?;
        c.g[S1][0] = c.g[V0][0];
    }
    // 0x41a478: same exponent as before (y)? Then is aadj far enough from
    // a half to stop?
    c.cycles += 8;
    c.g[V1][0] = c.lw(strtod_at(c, STRTOD_Y));
    c.g[A0][0] = strtod_hi(c.g[S1][0]);
    c.g[V0][0] = c.g[V1][0] << 32;
    c.g[V1][0] = 0x7ff0_0000;
    c.g[V0][0] >>= 32;
    c.g[S2][0] = c.g[A0][0] & c.g[V1][0];
    let moved = c.g[V0][0] != c.g[S2][0];
    c.g[A1][0] = c.lw(strtod_at(c, STRTOD_BB));
    if moved {
        return Ok(STRTOD_NEXT);
    }
    // aadj -= (long long)aadj
    c.cycles += 9;
    c.g[A0][0] = c.g[S0][0];
    STRTOD_FIXDFDI.call(c, 0x0041_a4a0)?;
    c.g[A0][0] = c.g[V0][0];
    STRTOD_FLOATDIDF.call(c, 0x0041_a4a8)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    STRTOD_SUBDF3.call(c, 0x0041_a4b4)?;
    let dsign = c.g[S4][0] != 0;
    c.g[S0][0] = c.g[V0][0];
    if !dsign {
        c.cycles += 5;
        c.g[V0][0] = STRTOD_FRAC;
        c.g[V0][0] &= c.g[S1][0];
        if c.g[V0][0] == 0 {
            // 0x41a528: aadj < 0.25 - eps: done.
            c.cycles += 11;
            c.g[A1][0] = STRTOD_QUARTER_LO;
            c.g[A0][0] = c.g[S0][0];
            STRTOD_CMPDF2.call(c, 0x0041_a54c)?;
            if (c.g[V0][0] as i64) < 0 {
                c.g[A1][0] = c.lw(strtod_at(c, STRTOD_BB));
                return Ok(STRTOD_FREE1);
            }
            c.cycles += 1;
            c.g[A1][0] = c.lw(strtod_at(c, STRTOD_BB));
            return Ok(STRTOD_NEXT);
        }
    }
    // 0x41a4d0: aadj < 0.5 - eps or aadj > 0.5 + eps: done.
    c.cycles += 11;
    c.g[A1][0] = STRTOD_HALF_LO;
    c.g[A0][0] = c.g[S0][0];
    STRTOD_CMPDF2.call(c, 0x0041_a4f4)?;
    let below = (c.g[V0][0] as i64) < 0;
    c.g[A1][0] = c.lw(strtod_at(c, STRTOD_BB));
    if below {
        return Ok(STRTOD_FREE1);
    }
    c.cycles += 9;
    c.g[A1][0] = STRTOD_HALF_HI;
    c.g[A0][0] = c.g[S0][0];
    STRTOD_CMPDF2.call(c, 0x0041_a518)?;
    let above = (c.g[V0][0] as i64) > 0;
    c.g[A1][0] = c.lw(strtod_at(c, STRTOD_BB));
    if above {
        return Ok(STRTOD_FREE);
    }
    c.cycles += 2;
    Ok(STRTOD_NEXT)
}

/// 0x41a558: Bfree bb (a1), bd, bs, delta.
pub(crate) fn strtod_next(c: &mut Cpu) -> StrtodStep {
    c.cycles += 11;
    c.g[A0][0] = c.g[S7][0];
    STRTOD_BFREE.call(c, 0x0041_a560)?;
    for (slot, ret_addr) in [
        (STRTOD_BD, 0x0041_a56c),
        (STRTOD_BS, 0x0041_a578),
        (STRTOD_DELTA, 0x0041_a584),
    ] {
        c.g[A1][0] = c.lw(strtod_at(c, slot));
        c.g[A0][0] = c.g[S7][0];
        STRTOD_BFREE.call(c, ret_addr)?;
    }
    Ok(STRTOD_LOOPTOP)
}

/// 0x41a584: one correction round: bd = copy of bd0, bb = d2b(rv), bs = 1,
/// both scaled by the powers of 2 and 5 of e and of rv's exponent;
/// delta = |bb - bd| (dsign, s4 = bb < bd); cmp(delta, bs).
pub(crate) fn strtod_looptop(c: &mut Cpu) -> StrtodStep {
    c.cycles += 25;
    c.g[A0][0] = c.lw(strtod_at(c, STRTOD_BD0));
    c.g[A1][0] = c.lw(r32(c, A0).wrapping_add(4));
    c.g[A0][0] = c.g[S7][0];
    STRTOD_BALLOC.call(c, 0x0041_a594)?;
    c.g[A3][0] = c.lw(strtod_at(c, STRTOD_BD0));
    c.g[A2][0] = c.lw(r32(c, A3).wrapping_add(0x10));
    strtod_addiu(c, A1, A3, 0xc);
    c.sw(strtod_at(c, STRTOD_BD), c.g[V0][0]);
    c.g[A2][0] = sx(r32(c, A2) << 2);
    strtod_addiu(c, A0, V0, 0xc);
    strtod_addiu(c, A2, A2, 8);
    STRTOD_MEMCPY.call(c, 0x0041_a5b4)?;
    c.g[A3][0] = c.lw(strtod_at(c, STRTOD_BBBITS_PTR));
    c.g[A0][0] = c.g[S7][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = c.g[SP][0];
    STRTOD_D2B.call(c, 0x0041_a5c8)?;
    c.sw(strtod_at(c, STRTOD_BB), c.g[V0][0]);
    c.g[A0][0] = c.g[S7][0];
    c.g[A1][0] = 1;
    STRTOD_I2B.call(c, 0x0041_a5d8)?;
    c.sw(strtod_at(c, STRTOD_BS), c.g[V0][0]);
    c.g[V0][0] = c.ld(strtod_at(c, STRTOD_E));
    let e_neg = (c.g[V0][0] as i64) < 0;
    c.g[S0][0] = c.lw(strtod_at(c, STRTOD_EINT));
    // bb2 (s2), bb5 (a2), bd2 (s0), bd5 (s6)
    if !e_neg {
        c.cycles += 4;
        c.g[A2][0] = 0;
        c.g[S2][0] = 0;
        c.g[S6][0] = c.g[S0][0];
    } else {
        c.cycles += 5;
        c.g[V1][0] = c.lw(strtod_at(c, STRTOD_EINT));
        c.g[S6][0] = 0;
        c.g[S0][0] = 0;
        c.g[S2][0] = sx(0u32.wrapping_sub(r32(c, V1)));
        c.g[A2][0] = c.g[S2][0];
    }
    // 0x41a60c: bbe
    c.cycles += 3;
    c.g[A0][0] = c.lw(sp(c));
    if (c.g[A0][0] as i64) < 0 {
        c.g[S0][0] = sx(r32(c, S0).wrapping_sub(r32(c, A0)));
    } else {
        c.cycles += 1;
        strtod_addu(c, S2, S2, A0);
    }
    // 0x41a61c: j = bbe + bbbits - 1; i = denormal ? bbe + 1075 : 54 - bbbits
    c.cycles += 6;
    c.g[A1][0] = c.lw(strtod_at(c, 4));
    strtod_addu(c, V0, A0, A1);
    strtod_addiu(c, A3, V0, 0u32.wrapping_sub(1));
    let denormal = (c.g[A3][0] as i64) < -1022;
    c.g[V1][0] = denormal as u64;
    c.g[S4][0] = c.g[S2][0];
    c.cycles += 2;
    if denormal {
        strtod_addiu(c, S3, A0, 0x433);
    } else {
        c.g[V0][0] = 0x36;
        c.g[S3][0] = sx(0x36u32.wrapping_sub(r32(c, A1)));
    }
    // 0x41a644: bb2 += i; bd2 += i; i = min(bb2, bd2, bs2); subtract it.
    c.cycles += 9;
    strtod_addu(c, A3, S0, S3);
    strtod_addu(c, S2, S2, S3);
    c.g[S0][0] = c.g[A3][0];
    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[A3][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.g[A3][0] = c.g[S2][0];
    }
    c.g[V1][0] = ((c.g[S4][0] as i64) < (c.g[A3][0] as i64)) as u64;
    if c.g[V1][0] != 0 {
        c.g[A3][0] = c.g[S4][0];
    }
    if (c.g[A3][0] as i64) > 0 {
        c.cycles += 3;
        c.g[S4][0] = sx(r32(c, S4).wrapping_sub(r32(c, A3)));
        c.g[S2][0] = sx(r32(c, S2).wrapping_sub(r32(c, A3)));
        c.g[S0][0] = sx(r32(c, S0).wrapping_sub(r32(c, A3)));
    }
    // 0x41a674: bb5 > 0: bs = pow5mult(bs, bb5); bb = bs * bb.
    c.cycles += 2;
    let bb5 = (c.g[A2][0] as i64) > 0;
    c.g[A1][0] = c.lw(strtod_at(c, STRTOD_BS));
    if bb5 {
        c.cycles += 11;
        c.g[A0][0] = c.g[S7][0];
        STRTOD_POW5MULT.call(c, 0x0041_a684)?;
        c.sw(strtod_at(c, STRTOD_BS), c.g[V0][0]);
        c.g[A0][0] = c.g[S7][0];
        c.g[A2][0] = c.lw(strtod_at(c, STRTOD_BB));
        c.g[A1][0] = c.g[V0][0];
        STRTOD_MULTIPLY.call(c, 0x0041_a698)?;
        c.g[A1][0] = c.lw(strtod_at(c, STRTOD_BB));
        c.g[A0][0] = c.g[S7][0];
        c.sw(strtod_at(c, STRTOD_BB), c.g[V0][0]);
        STRTOD_BFREE.call(c, 0x0041_a6a8)?;
    }
    // bb2 > 0: bb <<= bb2; bd5 > 0: bd = pow5mult(bd, bd5); bd2 > 0:
    // bd <<= bd2; bs2 > 0: bs <<= bs2.
    for (count, slot, callee, ret_addr) in [
        (S2, STRTOD_BB, STRTOD_LSHIFT, 0x0041_a6bc),
        (S6, STRTOD_BD, STRTOD_POW5MULT, 0x0041_a6d4),
        (S0, STRTOD_BD, STRTOD_LSHIFT, 0x0041_a6ec),
        (S4, STRTOD_BS, STRTOD_LSHIFT, 0x0041_a704),
    ] {
        c.cycles += 2;
        let positive = (c.g[count][0] as i64) > 0;
        c.g[A1][0] = c.lw(strtod_at(c, slot));
        if positive {
            c.cycles += 4;
            c.g[A2][0] = c.g[count][0];
            c.g[A0][0] = c.g[S7][0];
            callee.call(c, ret_addr)?;
            c.sw(strtod_at(c, slot), c.g[V0][0]);
        }
    }
    // 0x41a708: delta = mdiff(bb, bd); dsign = delta->sign; delta->sign = 0.
    c.cycles += 13;
    c.g[A1][0] = c.lw(strtod_at(c, STRTOD_BB));
    c.g[A0][0] = c.g[S7][0];
    c.g[A2][0] = c.lw(strtod_at(c, STRTOD_BD));
    STRTOD_MDIFF.call(c, 0x0041_a718)?;
    c.sw(strtod_at(c, STRTOD_DELTA), c.g[V0][0]);
    c.g[A1][0] = c.lw(strtod_at(c, STRTOD_BS));
    c.g[S4][0] = c.lw(r32(c, V0).wrapping_add(0xc));
    c.g[A0][0] = c.g[V0][0];
    c.sw(r32(c, A0).wrapping_add(0xc), 0);
    STRTOD_MCMP.call(c, 0x0041_a730)?;
    c.g[A3][0] = c.g[V0][0];
    if (c.g[A3][0] as i64) >= 0 {
        return strtod_back(c, STRTOD_CMP);
    }
    // delta < bs: rv is right, unless it is a power of two from below.
    c.cycles += 2;
    let dsign = c.g[S4][0] != 0;
    c.g[A1][0] = c.lw(strtod_at(c, STRTOD_BB));
    if dsign {
        return Ok(STRTOD_FREE1);
    }
    c.cycles += 5;
    c.g[V0][0] = STRTOD_FRAC;
    c.g[V0][0] &= c.g[S1][0];
    let power_of_two = c.g[V0][0] == 0;
    c.g[A1][0] = c.lw(strtod_at(c, STRTOD_DELTA));
    if power_of_two {
        return strtod_back(c, STRTOD_LSHIFT1);
    }
    Ok(STRTOD_FREE)
}

/// 0x41a75c: Bfree bb (a1), bd, bs, bd0, delta.
pub(crate) fn strtod_free(c: &mut Cpu) -> StrtodStep {
    c.cycles += 14;
    c.g[A0][0] = c.g[S7][0];
    STRTOD_BFREE.call(c, 0x0041_a764)?;
    for (slot, ret_addr) in [
        (STRTOD_BD, 0x0041_a770),
        (STRTOD_BS, 0x0041_a77c),
        (STRTOD_BD0, 0x0041_a788),
        (STRTOD_DELTA, 0x0041_a794),
    ] {
        c.g[A1][0] = c.lw(strtod_at(c, slot));
        c.g[A0][0] = c.g[S7][0];
        STRTOD_BFREE.call(c, ret_addr)?;
    }
    Ok(STRTOD_OUT)
}

/// 0x41a798: `*se = s` (a0 = se); return sign ? -rv : rv.
pub(crate) fn strtod_out(c: &mut Cpu) -> StrtodStep {
    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.sw(r32(c, A0), c.g[S5][0]);
    }
    c.cycles += 3;
    c.g[A3][0] = c.lw(strtod_at(c, STRTOD_SIGN));
    let negative = c.g[A3][0] != 0;
    c.g[V0][0] = c.g[S1][0];
    if negative {
        c.cycles += 3;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = 0;
        STRTOD_SUBDF3.call(c, 0x0041_a7b8)?;
    }
    c.cycles += 12;
    let frame = Frame { size: STRTOD_FRAME };
    frame.restore(c, S0, 0xe0);
    frame.restore(c, S1, 0xd0);
    frame.restore(c, S2, 0xc0);
    frame.restore(c, S3, 0xb0);
    frame.restore(c, S4, 0xa0);
    frame.restore(c, S5, 0x90);
    frame.restore(c, S6, 0x80);
    frame.restore(c, S7, 0x70);
    frame.restore(c, FP, 0x60);
    frame.restore_ra(c, 0x50);
    frame.close(c);
    Ok(STRTOD_DONE)
}

// ---------------------------------------------------------------------------
// strncasecmp (0x41ab78)

/// `tolower` (0x41ea18), the libc character-class lookup.
pub(crate) const STRNCASECMP_TOLOWER: Callee = Callee {
    addr: 0x0041_ea18,
    f: ee::s15::tolower,
};

/// `int strncasecmp(const char* s1, const char* s2, int n)` (0x41ab78):
/// compare at most `n` characters case-insensitively; returns the
/// difference of the lowered characters where they part (0 for `n == 0`).
pub fn strncasecmp(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        if c.g[S1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[S4][0] = 0xffff_ffff_ffff_ffff;
            'scan: loop {
                // n-- != 0
                c.cycles += 3;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                if c.g[S1][0] == c.g[S4][0] {
                    break 'scan;
                }
                c.cycles += 7;
                c.g[A0][0] = c.lb(c.g[S2][0] as u32);
                STRNCASECMP_TOLOWER.call(c, 0x0041_abd0)?;
                c.g[A0][0] = c.lb(c.g[S3][0] as u32);
                c.g[S0][0] = c.g[V0][0];
                STRNCASECMP_TOLOWER.call(c, 0x0041_abdc)?;
                if c.g[S0][0] != c.g[V0][0] {
                    break 'scan;
                }
                c.cycles += 2;
                if c.g[S1][0] == 0 {
                    break 'scan;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lb(c.g[S2][0] as u32);
                if c.g[V0][0] == 0 {
                    break 'scan;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lb(c.g[S3][0] as u32);
                if c.g[V0][0] == 0 {
                    break 'scan;
                }
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                if event_due(c) {
                    deliver_event_here(c, 0x0041_abb8);
                }
                c.cycles += 1;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            }
            // The difference of the lowered (unsigned) characters.
            c.cycles += 6;
            c.g[A0][0] = c.lbu(c.g[S2][0] as u32);
            STRNCASECMP_TOLOWER.call(c, 0x0041_ac0c)?;
            c.g[A0][0] = c.lbu(c.g[S3][0] as u32);
            c.g[S0][0] = c.g[V0][0];
            STRNCASECMP_TOLOWER.call(c, 0x0041_ac18)?;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        }
        c.cycles += 8;
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::machine;
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fresh;

    pub(crate) use crate::game::input::inputparse::tests::oc396_ctype_upper;
    pub(crate) use crate::game::seg::seg_2fe810::tests::oc383_machine;

    #[test]
    fn tolower_matches() {
        check(ee::s15::f_0041ea18__generated, tolower, 300, &|c, seed| {
            let mut r = oc383_machine(c, seed, 0xea18, 0x0041_ea18);
            let ch = if seed < 256 {
                seed
            } else {
                r.next_u32() & 0x1ff
            };
            c.g[A0][0] = if seed % 7 == 6 {
                sx(ch | 0xffff_ff00)
            } else {
                ch as u64
            };
        });
    }

    /// A random string of letters and punctuation (mixed case) at `at`.
    pub(crate) fn oc383_put_str(c: &mut Cpu, at: u32, text: &[u8]) {
        for (i, &b) in text.iter().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
        c.bus.write8(at + text.len() as u32, 0);
    }

    #[test]
    fn strcasecmp_matches() {
        check(
            ee::s15::f_0041aa88__generated,
            strcasecmp,
            300,
            &|c, seed| {
                let mut r = oc383_machine(c, seed, 0xaa88, STRCASECMP_ENTRY);
                let len = (r.next_u32() % 8) as usize;
                let a: Vec<u8> = (0..len)
                    .map(|_| {
                        let k = r.next_u32();
                        [b'a', b'A', b'0', b'_', 0xc1][(k % 5) as usize]
                            .wrapping_add(((k >> 8) % 26) as u8 * ((k % 5 < 2) as u8))
                    })
                    .collect();
                let mut b: Vec<u8> = a
                    .iter()
                    .map(|&x| {
                        if r.next_u32() & 1 == 0 {
                            x.to_ascii_uppercase()
                        } else {
                            x.to_ascii_lowercase()
                        }
                    })
                    .collect();
                match seed % 4 {
                    0 if !b.is_empty() => {
                        let i = (r.next_u32() as usize) % b.len();
                        b[i] = b[i].wrapping_add(1 + (r.next_u32() % 3) as u8);
                    }
                    1 => b.truncate(b.len() / 2),
                    2 => b.push(b'x'),
                    _ => {}
                }
                let (sa, sb) = (DATA + 0x100, DATA + 0x200);
                oc383_put_str(c, sa, &a);
                oc383_put_str(c, sb, &b);
                c.g[A0][0] = sx(sa);
                c.g[A1][0] = sx(sb);
                if seed % 5 == 4 {
                    // Resume at the loop head inside the frame (s1/s2 mid-string).
                    let sp = crate::difftest::SP - 0x40;
                    fill(c, &mut r, sp, 0x10);
                    c.bus.write64(sp, crate::difftest::RA as u64);
                    c.g[SP][0] = sx(sp);
                    c.g[S1][0] = sx(sa + 1u32.min(len as u32));
                    c.g[S2][0] = sx(sb);
                    c.pc = STRCASECMP_HEAD;
                }
            },
        );
    }

    /// A string `FILE` as `sscanf` builds it, the input text, the format,
    /// the `va_list` slots, the outputs and a reent with `__sdidinit` set.
    pub(crate) const SVFSCANF_T_FILE: u32 = DATA;

    pub(crate) const SVFSCANF_T_INPUT: u32 = DATA + 0x1000;

    pub(crate) const SVFSCANF_T_FMT: u32 = DATA + 0x2000;

    pub(crate) const SVFSCANF_T_AP: u32 = DATA + 0x3000;

    pub(crate) const SVFSCANF_T_OUT: u32 = DATA + 0x4000;

    pub(crate) const SVFSCANF_T_REENT: u32 = DATA + 0x6000;

    /// `eofread`, the string FILE's read function.
    pub(crate) const SVFSCANF_T_EOFREAD: u32 = 0x0041_78a8;

    /// (format, input) pairs reaching every directive, conversion and
    /// failure path.
    pub(crate) const SVFSCANF_CASES: &[(&[u8], &[u8])] = &[
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

    /// strncmp: strings at 16-, 8- and 1-byte alignments, equal prefixes of
    /// random length with a difference and/or a NUL somewhere, n from 0 to
    /// 80 (the quadword, doubleword and byte paths and their exits).
    #[test]
    fn strncmp_matches() {
        const A: u32 = DATA + 0x100;
        const B: u32 = DATA + 0x400;
        check(ee::s15::f_00416b18__generated, strncmp, 400, &|c, seed| {
            let mut r = machine(c, seed, 0x416b18, 0);
            c.pc = STRNCMP_ENTRY;
            let (oa, ob) = match seed % 5 {
                0 | 1 => (0, 0),
                2 => (8, 8),
                3 => (8 * (r.next_u32() % 2), 8 * (r.next_u32() % 2)),
                _ => (r.next_u32() % 16, r.next_u32() % 16),
            };
            let n = match seed % 7 {
                0 => r.next_u32() % 16,
                1 => 0,
                _ => r.next_u32() % 80,
            };
            // Non-NUL bytes, the same in both strings.
            for i in 0..0x120u32 {
                let v = (r.next_u32() % 255 + 1) as u8;
                c.bus.write8(A + oa + i, v);
                c.bus.write8(B + ob + i, v);
            }
            if seed % 3 != 0 {
                let at = r.next_u32() % (n + 8);
                let v = c.bus.read8(B + ob + at);
                c.bus
                    .write8(B + ob + at, v ^ (1 + r.next_u32() % 255) as u8);
            }
            if seed % 4 != 1 {
                let at = r.next_u32() % (n + 8);
                c.bus.write8(A + oa + at, 0);
                if seed % 2 == 0 {
                    c.bus.write8(B + ob + at, 0);
                }
            }
            c.g[A0][0] = sx(A + oa);
            c.g[A1][0] = sx(B + ob);
            c.g[A2][0] = n as u64;
        });
    }

    /// The `_reent` (errno at 0, the bigint freelist pointer at 0x4c), its
    /// freelist array (16 heads), the pool of free bigints, the endptr cell
    /// and the text.
    pub(crate) const STRTOD_T_REENT: u32 = DATA;

    pub(crate) const STRTOD_T_FREELIST: u32 = DATA + 0x100;

    pub(crate) const STRTOD_T_BIGINTS: u32 = DATA + 0x2000;

    pub(crate) const STRTOD_T_ENDPTR: u32 = DATA + 0x40;

    pub(crate) const STRTOD_T_TEXT: u32 = DATA + 0x8_0000;

    /// Texts that reach the special paths: nothing to read, blanks and
    /// signs, zeros, fractions, exponents (clamped, without digits), the
    /// exact fast cases, the scale loops, overflow, underflow, denormals,
    /// halfway cases and the top of the range.
    pub(crate) const STRTOD_T_FIXED: &[&[u8]] = &[
        b"",
        b"   ",
        b"+",
        b"-",
        b" \t\n\x0b\x0c\r-12.5e3x",
        b"0",
        b"000",
        b"0.000",
        b"-0.0e5",
        b".",
        b".e5",
        b"e5",
        b"1e",
        b"1e+",
        b"1e-x",
        b"0e5",
        b"00e-7",
        b"abc",
        b"\xc39",
        b"7\xc3",
        b"123456789012345678901234567890",
        b"0.000000000000000000001234567890123456789",
        b"1e22",
        b"1e23",
        b"12345e30",
        b"1e30",
        b"1e40",
        b"1e-22",
        b"1e-23",
        b"1e-30",
        b"1e-400",
        b"1e400",
        b"4.9e-324",
        b"2.4703282292062327e-324",
        b"2.4703282292062328e-324",
        b"2.2250738585072011e-308",
        b"2.2250738585072014e-308",
        b"1.7976931348623157e308",
        b"1.7976931348623158e308",
        b"1.797693134862316e308",
        b"8.98846567431158e307",
        b"9007199254740993",
        b"9007199254740992.5",
        b"0.1",
        b"3.14159265358979323846264338327950288",
        b"1e0000000000000000005",
        b"1e123456789",
        b"1e-123456789",
        b"123456789e-300",
        b"1.0000000000000002",
        b"0.5e-323",
        b"123.456e-5",
        b"000123",
        b"-.5",
        b"0.0000001",
        b"4294967295",
        b"4294967296.5",
        b"18446744073709551615",
        b"99999999999999999999e10",
        b"5e-324",
        b"1.5e-320",
        b"7.2057594037927933e16",
        b"6.631236846766476e-316",
        b"3.237883913302901e-308",
        b"1e308",
        b"1e309",
        b"1e-307",
        b"1e-308",
        b"1e-320",
        b"0x1p3",
    ];

    /// One text per seed: the fixed ones first, then random numbers.
    pub(crate) fn strtod_t_text(seed: u32, r: &mut Rng) -> Vec<u8> {
        if let Some(t) = STRTOD_T_FIXED.get(seed as usize) {
            return t.to_vec();
        }
        let mut t = Vec::new();
        let digit = |r: &mut Rng| b'0' + (r.next_u32() % 10) as u8;
        for _ in 0..r.next_u32() % 3 {
            t.push(b" \t\n\x0b\x0c\r"[(r.next_u32() % 6) as usize]);
        }
        match r.next_u32() % 4 {
            0 => t.push(b'-'),
            1 => t.push(b'+'),
            _ => {}
        }
        if r.next_u32() % 3 == 0 {
            for _ in 0..1 + r.next_u32() % 3 {
                t.push(b'0');
            }
        }
        for _ in 0..r.next_u32() % 24 {
            let d = digit(r);
            t.push(d);
        }
        if r.next_u32() % 2 == 0 {
            t.push(b'.');
            if r.next_u32() % 3 == 0 {
                for _ in 0..1 + r.next_u32() % 5 {
                    t.push(b'0');
                }
            }
            for _ in 0..r.next_u32() % 20 {
                // Runs of zeros inside the fraction.
                let d = if r.next_u32() % 4 == 0 {
                    b'0'
                } else {
                    digit(r)
                };
                t.push(d);
            }
        }
        if r.next_u32() % 4 != 0 {
            t.push(if r.next_u32() % 2 == 0 { b'e' } else { b'E' });
            match r.next_u32() % 3 {
                0 => t.push(b'-'),
                1 => t.push(b'+'),
                _ => {}
            }
            if r.next_u32() % 5 == 0 {
                t.push(b'0');
            }
            let e = match r.next_u32() % 4 {
                0 => r.next_u32() % 25,
                1 => 20 + r.next_u32() % 40,
                2 => 280 + r.next_u32() % 60,
                _ => r.next_u32() % 400,
            };
            if r.next_u32() % 16 != 0 {
                t.extend_from_slice(e.to_string().as_bytes());
            }
        }
        if r.next_u32() % 3 == 0 {
            t.push(b"x.e+-9 "[(r.next_u32() % 7) as usize]);
        }
        t
    }

    /// A freelist with 40 free bigints of each size k = 0..8, so `_Balloc`
    /// never needs `_calloc_r` (bigint: next, k, maxwds, sign, wds, x[]).
    pub(crate) fn strtod_t_bigints(c: &mut Cpu) {
        c.bus.write32(STRTOD_T_REENT + 0x4c, STRTOD_T_FREELIST);
        let mut at = STRTOD_T_BIGINTS;
        for k in 0..=8u32 {
            let size = (0x14 + (4u32 << k) + 0xf) & !0xf;
            let mut head = 0;
            for _ in 0..40 {
                c.bus.write32(at, head);
                c.bus.write32(at + 4, k);
                c.bus.write32(at + 8, 1 << k);
                head = at;
                at += size;
            }
            c.bus.write32(STRTOD_T_FREELIST + 4 * k, head);
        }
    }

    /// The game image (switch table, tens[], the soft-float routines'
    /// data), the reent with its bigint pool, the text, the arguments.
    pub(crate) fn strtod_t_setup(c: &mut Cpu, seed: u32, prefix: &[u8]) -> Rng {
        let mut r = Rng(0x0041_98d8 ^ (seed as u64) << 24);
        scramble(c, &mut r);
        elf_image(c);
        strtod_t_bigints(c);
        let mut text = prefix.to_vec();
        text.extend(strtod_t_text(seed, &mut r));
        for (i, b) in text.iter().enumerate() {
            c.bus.write8(STRTOD_T_TEXT + i as u32, *b);
        }
        c.bus.write8(STRTOD_T_TEXT + text.len() as u32, 0);
        c.bus.write32(STRTOD_T_REENT, 0);
        c.bus.write32(STRTOD_T_ENDPTR, 0xdead_beef);
        strtod_t_args(c, seed);
        c.pc = STRTOD_ENTRY;
        r
    }

    /// a0 = reent, a1 = text, a2 = &endptr (or NULL).
    pub(crate) fn strtod_t_args(c: &mut Cpu, seed: u32) {
        c.g[A0][0] = sx(STRTOD_T_REENT);
        c.g[A1][0] = sx(STRTOD_T_TEXT);
        c.g[A2][0] = if seed % 5 == 4 {
            0
        } else {
            sx(STRTOD_T_ENDPTR)
        };
    }

    /// The port's results for a few texts are the correctly rounded
    /// doubles (a check of the fixture: tables loaded, bigints usable).
    #[test]
    fn strtod_r_parses() {
        for (seed, text) in STRTOD_T_FIXED.iter().enumerate() {
            let Some(want) = std::str::from_utf8(text)
                .ok()
                .filter(|t| {
                    [
                        "0.1",
                        "1e23",
                        "123456789012345678901234567890",
                        "1e40",
                        "1e-30",
                        "123.456e-5",
                    ]
                    .contains(t)
                })
                .and_then(|t| t.parse::<f64>().ok())
            else {
                continue;
            };
            let mut c = fresh();
            strtod_t_setup(&mut c, seed as u32, b"");
            strtod_r(&mut c);
            assert_eq!(c.pc, crate::difftest::RA);
            assert_eq!(
                c.g[V0][0],
                want.to_bits(),
                "{:?}",
                std::str::from_utf8(text)
            );
        }
    }

    /// `_strtod_r` against the generated body over the fixed texts and
    /// random numbers (every parse path, the fast cases, both scale loops,
    /// overflow/underflow and the bigint correction loop).
    #[test]
    fn strtod_r_matches() {
        check_lohi(
            ssx3_ee::s15::f_004198d8__generated,
            strtod_r,
            250,
            &|c, seed| {
                strtod_t_setup(c, seed, b"");
            },
        );
    }

    driven!(strtod_t_gen_driven, ssx3_ee::s15::f_004198d8__generated);

    driven!(strtod_t_hand_driven, strtod_r);

    /// With events due in the parse and correction loops (delivered, and
    /// unwinding with re-entry at the loop heads).
    #[test]
    fn strtod_r_events_match() {
        check_events(strtod_t_gen_driven, strtod_t_hand_driven, 96, &|c, seed| {
            // Random texts after three blanks: the first due event lands
            // on a parse loop's back edge (the blanks', the digits'), not
            // inside a callee.
            let seed2 = seed + STRTOD_T_FIXED.len() as u32;
            let mut r = strtod_t_setup(c, seed2, b"   ");
            let mode = schedule(c, &mut r, seed, STRTOD_ENTRY);
            strtod_t_args(c, seed2);
            if mode != crate::difftest::events::Events::Off {
                c.next_event = c.cycles + 20 + (r.next_u32() % 40) as u64;
            }
            mode
        });
    }

    #[test]
    fn strncasecmp_matches() {
        check(
            ee::s15::f_0041ab78__generated,
            strncasecmp,
            300,
            &|c, seed| {
                let mut r = Rng(0x0396_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0041_ab78;
                oc396_ctype_upper(c);
                let alpha = b"aAbBzZ0_";
                let len = 1 + r.next_u32() % 6;
                let (s1, s2) = (DATA, DATA + 0x100);
                for i in 0..len {
                    let ch = alpha[(r.next_u32() % 8) as usize];
                    c.bus.write8(s1 + i, ch);
                    // Mostly the same letter in the other case, sometimes a different one.
                    let other = if r.next_u32() % 5 == 0 {
                        alpha[(r.next_u32() % 8) as usize]
                    } else if ch.is_ascii_uppercase() {
                        ch.to_ascii_lowercase()
                    } else {
                        ch.to_ascii_uppercase()
                    };
                    c.bus.write8(s2 + i, other);
                }
                // Different lengths: one string ends early.
                let end2 = if seed % 3 == 0 { len - 1 } else { len };
                c.bus.write8(s2 + end2, 0);
                c.bus.write8(s1 + len, 0);
                c.g[A0][0] = sx(s1);
                c.g[A1][0] = sx(s2);
                c.g[A2][0] = match seed % 5 {
                    0 => 0,
                    1 => 1 + (r.next_u32() % 3) as u64,
                    _ => 8,
                };
            },
        );
    }
}
