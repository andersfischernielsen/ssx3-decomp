//! Staging module oc393 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc393`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, Unwound};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0041_98d8, strtod_r, "_strtod_r")];

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
const STRTOD_ERANGE: u64 = 0x22;
/// Mantissa bits of a double (`Frac_mask` over both words).
const STRTOD_FRAC: u64 = u64::MAX >> 12;
/// Low word mask.
const STRTOD_LOW: u64 = u64::MAX >> 32;
/// Double constants as bit patterns.
const STRTOD_TWO32: u64 = 0x41f0_0000_0000_0000;
const STRTOD_TWO: u64 = 0x4000_0000_0000_0000;
const STRTOD_ONE: u64 = 0x3ff0_0000_0000_0000;
const STRTOD_HALF: u64 = 0x3fe0_0000_0000_0000;
const STRTOD_MINUS_ONE: u64 = 0xbff0_0000_0000_0000;
/// The largest double (`Big0`/`Big1`), and its high word over the low mask.
const STRTOD_BIG: u64 = 0x7fef_ffff_ffff_ffff;
const STRTOD_BIG_HIGH: u64 = 0x7fef_ffff_0000_0000;
/// Rounding thresholds 0.5 - eps, 0.5 + eps, 0.25 - eps.
const STRTOD_HALF_LO: u64 = 0x3fdf_ffff_94a0_3595;
const STRTOD_HALF_HI: u64 = 0x3fe0_0000_35af_e535;
const STRTOD_QUARTER_LO: u64 = 0x3fcf_ffff_94a0_3595;

const STRTOD_FLOATSIDF: Callee = Callee {
    addr: 0x0041_2550,
    f: ee::s15::__floatsidf,
};
const STRTOD_FLOATDIDF: Callee = Callee {
    addr: 0x0041_1438,
    f: ee::s15::__floatdidf,
};
const STRTOD_ADDDF3: Callee = Callee {
    addr: 0x0041_1f18,
    f: ee::s15::__adddf3,
};
const STRTOD_SUBDF3: Callee = Callee {
    addr: 0x0041_1f70,
    f: ee::s15::__subdf3,
};
const STRTOD_MULDF3: Callee = Callee {
    addr: 0x0041_1fd8,
    f: ee::s15::__muldf3,
};
const STRTOD_DIVDF3: Callee = Callee {
    addr: 0x0041_2280,
    f: ee::s15::__divdf3,
};
const STRTOD_CMPDF2: Callee = Callee {
    addr: 0x0041_2500,
    f: ee::s15::__cmpdf2,
};
const STRTOD_FIXDFSI: Callee = Callee {
    addr: 0x0041_2608,
    f: ee::s15::__fixdfsi,
};
const STRTOD_FIXDFDI: Callee = Callee {
    addr: 0x0042_d320,
    f: ee::s15::__fixdfdi,
};
const STRTOD_ULP: Callee = Callee {
    addr: 0x0042_1020,
    f: ee::s15::__ulp,
};
const STRTOD_S2B: Callee = Callee {
    addr: 0x0042_06f8,
    f: ee::s15::__s2b,
};
const STRTOD_BALLOC: Callee = Callee {
    addr: 0x0042_0510,
    f: ee::s15::_Balloc,
};
const STRTOD_BFREE: Callee = Callee {
    addr: 0x0042_05b8,
    f: ee::s15::_Bfree,
};
const STRTOD_MEMCPY: Callee = Callee {
    addr: 0x0041_605c,
    f: ee::s15::memcpy,
};
const STRTOD_D2B: Callee = Callee {
    addr: 0x0042_1280,
    f: ee::s15::__d2b,
};
const STRTOD_I2B: Callee = Callee {
    addr: 0x0042_0978,
    f: ee::s15::__i2b,
};
const STRTOD_POW5MULT: Callee = Callee {
    addr: 0x0042_0bc8,
    f: ee::s15::__pow5mult,
};
const STRTOD_MULTIPLY: Callee = Callee {
    addr: 0x0042_09b0,
    f: ee::s15::__multiply,
};
const STRTOD_LSHIFT: Callee = Callee {
    addr: 0x0042_0cc8,
    f: ee::s15::__lshift,
};
const STRTOD_MDIFF: Callee = Callee {
    addr: 0x0042_0e90,
    f: ee::s15::__mdiff,
};
const STRTOD_MCMP: Callee = Callee {
    addr: 0x0042_0e28,
    f: ee::s15::__mcmp,
};
const STRTOD_RATIO: Callee = Callee {
    addr: 0x0042_1400,
    f: ee::s15::__ratio,
};

// Stages, named after their guest address. The ones marked "head" are loop
// heads with an event check on their back edge: the trampoline re-enters the
// function there after an unwind.
/// Skip blanks, read the sign (head).
const STRTOD_SKIP: u32 = 0x0041_9920;
/// First character after the sign.
const STRTOD_FIRST: u32 = 0x0041_9974;
/// Leading zeros (head).
const STRTOD_LEAD0: u32 = 0x0041_9988;
/// Start of the mantissa digits.
const STRTOD_MANT: u32 = 0x0041_99ac;
/// Integer digits (head).
const STRTOD_INTDIG: u32 = 0x0041_99e8;
/// The decimal point test (v0 = '.').
const STRTOD_POINT: u32 = 0x0041_9a44;
/// Zeros right after the point when there is no integer digit (head).
const STRTOD_FRACZ: u32 = 0x0041_9a68;
/// First fraction character after those zeros (v0 = c - '1').
const STRTOD_FRACNZ: u32 = 0x0041_9a88;
/// Fraction digit (head).
const STRTOD_FRACDIG: u32 = 0x0041_9aa8;
/// Scale the digits by ten once per skipped zero (head).
const STRTOD_FRACMUL: u32 = 0x0041_9ad0;
/// Append the fraction digit.
const STRTOD_FRACLAST: u32 = 0x0041_9b0c;
/// Next fraction character.
const STRTOD_FRACNEXT: u32 = 0x0041_9b54;
/// Fraction loop test.
const STRTOD_FRACTEST: u32 = 0x0041_9b5c;
/// The exponent marker test (v0 = 'e').
const STRTOD_EXP: u32 = 0x0041_9b70;
/// The exponent's sign.
const STRTOD_EXPSIGN: u32 = 0x0041_9ba4;
/// Exponent leading zeros (head).
const STRTOD_EXPZ: u32 = 0x0041_9bf0;
/// First exponent digit (v0 = c - '1').
const STRTOD_EXPDIGS: u32 = 0x0041_9c10;
/// Exponent digits (head).
const STRTOD_EXPDIG: u32 = 0x0041_9c38;
/// Clamp a long exponent, apply its sign.
const STRTOD_EXPCLAMP: u32 = 0x0041_9c74;
/// End of the text: nothing parsed, or convert.
const STRTOD_DIGEND: u32 = 0x0041_9ca4;
/// The digits as a double, the exact fast cases.
const STRTOD_CONVERT: u32 = 0x0041_9cb8;
/// Scale by `e1` (v0 = nd - k).
const STRTOD_SCALE: u32 = 0x0041_9e60;
/// Overflow: HUGE_VAL, ERANGE (head; and its second entry, v0 = ERANGE).
const STRTOD_OVFL: u32 = 0x0041_9ea8;
const STRTOD_OVFL2: u32 = 0x0041_9eac;
/// Positive powers of 1e16.
const STRTOD_BIGPOW: u32 = 0x0041_9ec8;
/// `bigtens[]` loop (head).
const STRTOD_BIGLOOP: u32 = 0x0041_9ee0;
/// Last `bigtens[]` factor with the exponent biased down.
const STRTOD_BIGLAST: u32 = 0x0041_9f10;
/// Negative scale.
const STRTOD_NEG: u32 = 0x0041_9fa8;
/// `tinytens[]` loop (head).
const STRTOD_TINYLOOP: u32 = 0x0041_a008;
/// Last `tinytens[]` factor and the underflow checks.
const STRTOD_TINYLAST: u32 = 0x0041_a038;
/// Underflow: zero, ERANGE (head; and its second entry, v0 = ERANGE).
const STRTOD_UNDFL: u32 = 0x0041_a0a4;
const STRTOD_UNDFL2: u32 = 0x0041_a0a8;
/// `rv += ulp(rv)` and finish (head).
const STRTOD_ULPADD: u32 = 0x0041_a0c0;
/// Bigint correction setup (`a1` = s0 loaded at 0x41a0dc or by the caller).
const STRTOD_BIGCOMP0: u32 = 0x0041_a0dc;
const STRTOD_BIGCOMP1: u32 = 0x0041_a0e0;
/// `delta <<= 1; cmp(delta, bs)` (head).
const STRTOD_LSHIFT1: u32 = 0x0041_a118;
/// `delta >= bs`: the correction step (head).
const STRTOD_CMP: u32 = 0x0041_a144;
/// `rv` rounded down to the largest value below a power of two.
const STRTOD_ROUNDDOWN: u32 = 0x0041_a1b0;
/// Halfway case: round to even.
const STRTOD_ODD: u32 = 0x0041_a1f4;
/// `ratio(delta, bs)` and the adjustment `aadj`.
const STRTOD_RATIOSTEP: u32 = 0x0041_a244;
/// `aadj1 = -aadj` (0x41a314).
const STRTOD_NEGADJ: u32 = 0x0041_a314;
/// Apply `aadj1 * ulp(rv)`.
const STRTOD_ADJ: u32 = 0x0041_a320;
/// Free the loop's bigints and go round again.
const STRTOD_NEXT: u32 = 0x0041_a558;
/// Top of the correction loop.
const STRTOD_LOOPTOP: u32 = 0x0041_a584;
/// Free the bigints (`a1` = bb loaded at 0x41a758 or by the caller).
const STRTOD_FREE: u32 = 0x0041_a758;
const STRTOD_FREE1: u32 = 0x0041_a75c;
/// Store `*se`, apply the sign, return (and its second entry, a0 = se).
const STRTOD_OUT: u32 = 0x0041_a794;
const STRTOD_OUT2: u32 = 0x0041_a798;
/// Returned.
const STRTOD_DONE: u32 = 0;

/// The resume heads (the loop heads with an event check on a back edge).
const STRTOD_HEADS: [u32; 17] = [
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

type StrtodStep = Result<u32, Unwound>;

/// A back edge to `head`: deliver a due event there (unwinding out if it
/// asks to), then continue at `head`.
fn strtod_back(c: &mut Cpu, head: u32) -> StrtodStep {
    if back_edge_unwinds(c, head) {
        Err(Unwound)
    } else {
        Ok(head)
    }
}

/// `sp + off`.
fn strtod_at(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// `rd = rs + rt` (32-bit, sign-extended).
fn strtod_addu(c: &mut Cpu, rd: usize, rs: usize, rt: usize) {
    c.g[rd][0] = sx(r32(c, rs).wrapping_add(r32(c, rt)));
}

/// `rd = rs + imm` (32-bit, sign-extended).
fn strtod_addiu(c: &mut Cpu, rd: usize, rs: usize, imm: u32) {
    c.g[rd][0] = sx(r32(c, rs).wrapping_add(imm));
}

/// The high word of a double, sign-extended (`dsra32`).
fn strtod_hi(v: u64) -> u64 {
    ((v as i64) >> 32) as u64
}

/// The low word sign-extended (`dsll32; dsra32`).
fn strtod_sext32(v: u64) -> u64 {
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
fn strtod_prologue(c: &mut Cpu) {
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
fn strtod_skip(c: &mut Cpu) -> StrtodStep {
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
fn strtod_sign_char(c: &mut Cpu) -> u32 {
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
fn strtod_first(c: &mut Cpu) -> u32 {
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
fn strtod_lead0(c: &mut Cpu) -> StrtodStep {
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
fn strtod_mant(c: &mut Cpu) -> u32 {
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
fn strtod_intdig(c: &mut Cpu) -> StrtodStep {
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
fn strtod_point(c: &mut Cpu) -> u32 {
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
fn strtod_fracz(c: &mut Cpu) -> StrtodStep {
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
fn strtod_fracnz(c: &mut Cpu) -> u32 {
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
fn strtod_fracdig(c: &mut Cpu) -> u32 {
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
fn strtod_fracmul(c: &mut Cpu) -> StrtodStep {
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
fn strtod_fraclast(c: &mut Cpu) -> u32 {
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
fn strtod_fracnext(c: &mut Cpu) -> u32 {
    c.cycles += 2;
    c.g[S5][0] = c.g[T0][0];
    c.g[A0][0] = c.lb(r32(c, S5));
    STRTOD_FRACTEST
}

/// 0x419b5c: another fraction digit?
fn strtod_fractest(c: &mut Cpu) -> StrtodStep {
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
fn strtod_exp(c: &mut Cpu) -> u32 {
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
fn strtod_expsign(c: &mut Cpu) -> u32 {
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
fn strtod_expz(c: &mut Cpu) -> StrtodStep {
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
fn strtod_expdigs(c: &mut Cpu) -> u32 {
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
fn strtod_expdig(c: &mut Cpu) -> StrtodStep {
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
fn strtod_expclamp(c: &mut Cpu) -> u32 {
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
fn strtod_digend(c: &mut Cpu) -> u32 {
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
fn strtod_convert(c: &mut Cpu) -> StrtodStep {
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
fn strtod_scale(c: &mut Cpu) -> StrtodStep {
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
fn strtod_ovfl2(c: &mut Cpu) -> u32 {
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
fn strtod_bigpow(c: &mut Cpu) -> u32 {
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
fn strtod_powloop(c: &mut Cpu, head: u32, ret_addr: u32, last: u32) -> StrtodStep {
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
fn strtod_biglast(c: &mut Cpu) -> StrtodStep {
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
fn strtod_neg(c: &mut Cpu) -> StrtodStep {
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
fn strtod_tinylast(c: &mut Cpu) -> StrtodStep {
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
fn strtod_undfl2(c: &mut Cpu) -> u32 {
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
fn strtod_ulpadd(c: &mut Cpu) -> StrtodStep {
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
fn strtod_bigcomp(c: &mut Cpu) -> StrtodStep {
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
fn strtod_lshift1(c: &mut Cpu) -> StrtodStep {
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
fn strtod_cmp(c: &mut Cpu) -> u32 {
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
fn strtod_rounddown(c: &mut Cpu) -> u32 {
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
fn strtod_odd(c: &mut Cpu) -> StrtodStep {
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
fn strtod_ratio(c: &mut Cpu) -> StrtodStep {
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
fn strtod_adj(c: &mut Cpu) -> StrtodStep {
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
fn strtod_next(c: &mut Cpu) -> StrtodStep {
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
fn strtod_looptop(c: &mut Cpu) -> StrtodStep {
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
fn strtod_free(c: &mut Cpu) -> StrtodStep {
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
fn strtod_out(c: &mut Cpu) -> StrtodStep {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::{check_lohi, elf_image, fresh, scramble, Rng, DATA};

    /// The `_reent` (errno at 0, the bigint freelist pointer at 0x4c), its
    /// freelist array (16 heads), the pool of free bigints, the endptr cell
    /// and the text.
    const STRTOD_T_REENT: u32 = DATA;
    const STRTOD_T_FREELIST: u32 = DATA + 0x100;
    const STRTOD_T_BIGINTS: u32 = DATA + 0x2000;
    const STRTOD_T_ENDPTR: u32 = DATA + 0x40;
    const STRTOD_T_TEXT: u32 = DATA + 0x8_0000;

    /// Texts that reach the special paths: nothing to read, blanks and
    /// signs, zeros, fractions, exponents (clamped, without digits), the
    /// exact fast cases, the scale loops, overflow, underflow, denormals,
    /// halfway cases and the top of the range.
    const STRTOD_T_FIXED: &[&[u8]] = &[
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
    fn strtod_t_text(seed: u32, r: &mut Rng) -> Vec<u8> {
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
    fn strtod_t_bigints(c: &mut Cpu) {
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
    fn strtod_t_setup(c: &mut Cpu, seed: u32, prefix: &[u8]) -> Rng {
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
    fn strtod_t_args(c: &mut Cpu, seed: u32) {
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
}
