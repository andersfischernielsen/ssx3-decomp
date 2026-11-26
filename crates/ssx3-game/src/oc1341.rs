//! Staging oc1341 (wave libm): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1341`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call;
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::lib::gcc::frame_sd;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0041_ed60, floor, "floor"),
    (0x0040_f9f8, scalbnf, "scalbnf"),
    (0x0040_d8d0, fmodf, "fmodf"),
    (0x0040_f878, matherr, "matherr"),
    (0x003e_b090, ds_timetosecs, "ds_timetosecs"),
];

// ---------------------------------------------------------------------------
// Callees (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// The re-entrant errno getter (`_reent` in `v0`).
    pub(crate) use crate::game::misc::libm_wrap::GETREENT;

    /// Double comparison (`__cmpdf2`, 0x412500): -1/0/1 in `v0`.
    pub(crate) use crate::game::lib::gcc::CMPDF2;

    /// `double __adddf3(double, double)` (0x411f18).
    pub const ADDDF3: Callee = Callee {
        addr: 0x0041_1f18,
        f: ee::s15::__adddf3,
    };

    /// `float __ieee754_fmodf(float, float)` (0x40e300).
    pub const IEEE754_FMODF: Callee = Callee {
        addr: 0x0040_e300,
        f: ee::s15::__ieee754_fmodf,
    };

    /// `int isnanf(float)` (0x40f9d0).
    pub const ISNANF: Callee = Callee {
        addr: 0x0040_f9d0,
        f: ee::s15::isnanf,
    };

    /// `double __extendsfdf2(float)` (0x413af8).
    pub const EXTENDSFDF2: Callee = Callee {
        addr: 0x0041_3af8,
        f: ee::s15::__extendsfdf2,
    };

    /// `float __truncdfsf2(double)` (0x4127a8).
    pub const TRUNCDFSF2: Callee = Callee {
        addr: 0x0041_27a8,
        f: ee::s15::__truncdfsf2,
    };

    /// `float copysignf(float, float)` (0x40fb58): `|x|` with `y`'s sign.
    pub const COPYSIGNF: Callee = Callee {
        addr: 0x0040_fb58,
        f: ee::s15::copysignf,
    };

    /// `int matherr(struct exception *)` (0x40f878): always 0.
    pub const MATHERR: Callee = Callee {
        addr: 0x0040_f878,
        f: ee::s15::matherr,
    };

    /// `void ds_secstotime(void *, int)` (0x3eaeb8): seconds to YMDHMS.
    pub(crate) use crate::game::dirtysock::tags::callee::DS_SECSTOTIME;
}

// ---------------------------------------------------------------------------
// matherr

/// `int matherr(struct exception *e)` (0x40f878): compare `e->arg1` against
/// 0 (the SVID hook, which never handles anything) and return 0.
pub fn matherr(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = c.g[A0][0];
        callee::CMPDF2.call(c, 0x0040_f88c)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// fmodf

/// `float fmodf(float x, float y)` (0x40d8d0): the remainder of `x / y`
/// through `__ieee754_fmodf`, with the SVID domain handling (NaN operands
/// pass through; a zero `y` reports `EDOM` through `matherr`).
pub fn fmodf(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x80);
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x68));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.mov_s(21, 12);
        c.mov_s(20, 13);
        frame_sd(c, frame, 0x40, c.g[S1][0]);
        frame_sd(c, frame, 0x30, c.g[S0][0]);
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        frame.save_ra(c, 0x50);
        c.g[S1][0] = 0x4a0000;
        callee::IEEE754_FMODF.call(c, 0x0040_d8fc)?;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x6978));
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        let libm_sticky = c.g[S0][0] == c.g[V0][0];
        c.mov_s(22, 0);
        if libm_sticky {
            // Sticky libm state: return the saved `x`.
            c.cycles += 1;
            c.mov_s(0, 22);
            c.cycles += 1;
            frame.restore_ra(c, 0x50);
        } else {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0040_d914);
            c.mov_s(12, 20);
            callee::ISNANF.call(c, 0x0040_d914)?;
            let x_nan = c.g[V0][0] != 0;
            c.mov_s(0, 22);
            if x_nan {
                c.cycles += 1;
                frame.restore_ra(c, 0x50);
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0040_d924);
                c.mov_s(12, 21);
                callee::ISNANF.call(c, 0x0040_d924)?;
                let y_nan = c.g[V0][0] != 0;
                c.mov_s(0, 22);
                if y_nan {
                    c.cycles += 1;
                    frame.restore_ra(c, 0x50);
                } else {
                    c.cycles += 5;
                    c.mtc1(0, 0);
                    c.c_eq_s(20, 0);
                    // Nonzero `y` needs no domain handling; a zero `y`
                    // goes through `matherr`.
                    let y_nonzero = !c.fcc();
                    c.g[V0][0] = 0x4a0000;
                    if y_nonzero {
                        c.cycles += 1;
                        c.mov_s(0, 22);
                        c.cycles += 1;
                        frame.restore_ra(c, 0x50);
                    } else {
                        fmodf_domain(c)?;
                    }
                }
            }
        }
        c.cycles += 7;
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x68));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// fmodf_domain

/// SVID domain tail of `fmodf` (0x40d8d0): `y` is zero here. Extend both
/// operands to double, report `EDOM` through `matherr` unless libm runs
/// `_POSIX_`, and truncate the saved result back to float. The frame is
/// `fmodf`'s (restored by its epilogue); the return address for the
/// `__truncdfsf2` call is reloaded before leaving.
fn fmodf_domain(c: &mut Cpu) -> Flow {
    c.cycles += 12;
    c.g[V1][0] = 0x1;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6f68));
    c.sw(c.g[SP][0] as u32, c.g[V1][0]);
    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.mov_s(12, 21);
    c.g[RA][0] = sx(0x0040_d95c);
    c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
    callee::EXTENDSFDF2.call(c, 0x0040_d95c)?;
    c.mov_s(12, 20);
    c.g[RA][0] = sx(0x0040_d968);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    callee::EXTENDSFDF2.call(c, 0x0040_d968)?;
    let s0_nonzero = c.g[S0][0] != 0;
    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
    if !s0_nonzero {
        c.cycles += 4;
        c.g[RA][0] = sx(0x0040_d978);
        c.mov_s(12, 21);
        callee::EXTENDSFDF2.call(c, 0x0040_d978)?;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
    } else {
        c.cycles += 3;
        c.g[V0][0] = 0x4a0000;
        c.g[V1][0] = c.ld((c.g[V0][0] as u32).wrapping_sub(0x6f60));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
    }
    c.cycles += 4;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x6978));
    c.g[V0][0] = 0x2;
    // Report EDOM unless libm runs `_POSIX_` (2) or `matherr` handled it.
    let mut store_errno = true;
    if c.g[V1][0] != c.g[V0][0] {
        c.cycles += 4;
        c.g[RA][0] = sx(0x0040_d9a4);
        c.g[A0][0] = c.g[SP][0];
        callee::MATHERR.call(c, 0x0040_d9a4)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        if c.g[V0][0] != 0 {
            store_errno = false;
        }
    }
    if store_errno {
        c.cycles += 5;
        c.g[RA][0] = sx(0x0040_d9b4);
        callee::GETREENT.call(c, 0x0040_d9b4)?;
        c.g[V1][0] = 0x21;
        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[RA][0] = sx(0x0040_d9d0);
        callee::GETREENT.call(c, 0x0040_d9d0)?;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
    }
    c.cycles += 4;
    c.g[RA][0] = sx(0x0040_d9e0);
    c.g[A0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x18));
    callee::TRUNCDFSF2.call(c, 0x0040_d9e0)?;
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
    Ok(())
}

/// Floor's `+11` result-combine tail (break 'b_41eefc): fold the rounded
/// halves back into `a1`/`s3`.
fn floor_combine(c: &mut Cpu) {
    c.cycles += 11;
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    c.g[V1][0] >>= 32;
    c.g[A0][0] = c.g[S0][0] << 32;
    c.g[S3][0] &= c.g[V1][0];
    c.g[V0][0] = c.g[S2][0] << 32;
    c.g[S3][0] |= c.g[A0][0];
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    c.g[V1][0] <<= 32;
    c.g[V0][0] >>= 32;
    c.g[S3][0] &= c.g[V1][0];
    c.g[A1][0] = c.g[S3][0] | c.g[V0][0];
}

/// Floor's 'b_41ef28 exit: the masked fraction is zero, return `a1`.
fn floor_join_28(c: &mut Cpu) {
    c.cycles += 1;
    c.g[V0][0] = c.g[A1][0];
}

/// Floor's 'b_41ef2c exit: reload the saved high half.
fn floor_join_2c(c: &mut Cpu, frame: Frame) {
    c.cycles += 1;
    frame.restore(c, S0, 0x50);
}

/// Floor's small-exponent mask (`0 <= s1 < 20` and `20 <= s1 < 52`
/// share this shape with different constants): when the fraction bits
/// are all clear the value is integral; otherwise nudge across the
/// boundary and re-check.
fn floor_mask_small(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    c.g[V0][0] |= 0xffff;
    c.g[S4][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[S1][0] as u32) & 31)) as u32);
    c.g[V1][0] = c.g[S0][0] & c.g[S4][0];
    c.g[V1][0] |= c.g[S2][0];
    c.g[A0][0] = c.g[A1][0];
    if c.g[V1][0] == 0 {
        floor_join_28(c);
        floor_join_2c(c, frame);
        return Ok(());
    }
    c.cycles += 8;
    c.g[V0][0] = 0x4a0000;
    c.g[A1][0] = c.ld((c.g[V0][0] as u32).wrapping_sub(0x5808));
    callee::ADDDF3.call(c, 0x0041_ee24)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    callee::CMPDF2.call(c, 0x0041_ee30)?;
    if (c.g[V0][0] as i64) <= 0 {
        floor_combine(c);
        floor_join_28(c);
        floor_join_2c(c, frame);
        return Ok(());
    }
    c.cycles += 2;
    let s0_nonneg = (c.g[S0][0] as i64) >= 0;
    c.g[V0][0] = !(0u64 | c.g[S4][0]);
    if !s0_nonneg {
        c.cycles += 4;
        c.g[V0][0] = 0x100000;
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[S1][0] as u32) & 31)) as u32);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = !(0u64 | c.g[S4][0]);
    }
    c.cycles += 3;
    c.g[S2][0] = 0;
    c.g[S0][0] &= c.g[V0][0];
    floor_combine(c);
    floor_join_28(c);
    floor_join_2c(c, frame);
    Ok(())
}

/// Floor's large-exponent mask (`52 > s1 >= 20`): same shape as
/// [`floor_mask_small`] with the wider mask.
fn floor_mask_large(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[V0][0] |= 0xffff;
    c.g[S4][0] = sx((c.g[V0][0] as u32) >> ((c.g[A0][0] as u32) & 31));
    c.g[V1][0] = c.g[S2][0] & c.g[S4][0];
    c.g[A0][0] = c.g[A1][0];
    if c.g[V1][0] == 0 {
        floor_join_28(c);
        floor_join_2c(c, frame);
        return Ok(());
    }
    c.cycles += 8;
    c.g[V0][0] = 0x4a0000;
    c.g[RA][0] = sx(0x0041_eea8);
    c.g[A1][0] = c.ld((c.g[V0][0] as u32).wrapping_sub(0x5808));
    callee::ADDDF3.call(c, 0x0041_eea8)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    callee::CMPDF2.call(c, 0x0041_eeb4)?;
    if (c.g[V0][0] as i64) <= 0 {
        floor_combine(c);
        floor_join_28(c);
        floor_join_2c(c, frame);
        return Ok(());
    }
    c.cycles += 2;
    let s0_nonneg = (c.g[S0][0] as i64) >= 0;
    c.g[V0][0] = !(0u64 | c.g[S4][0]);
    if !s0_nonneg {
        c.cycles += 3;
        c.g[V0][0] = 0x14;
        let s1_not_20 = c.g[S1][0] != c.g[V0][0];
        c.g[V0][0] = 0x34;
        if s1_not_20 {
            c.cycles += 7;
            c.g[V1][0] = 1;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S1][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[V0][0] as u32) & 31));
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = (c.g[V1][0] < c.g[S2][0]) as u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S2][0] = c.g[V1][0];
        } else {
            c.cycles += 2;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        }
        c.cycles += 1;
        c.g[V0][0] = !(0u64 | c.g[S4][0]);
    }
    c.cycles += 1;
    c.g[S2][0] &= c.g[V0][0];
    floor_combine(c);
    floor_join_28(c);
    floor_join_2c(c, frame);
    Ok(())
}

// ---------------------------------------------------------------------------
// floor

/// `double floor(double x)` (0x41ed60): round toward negative infinity by
/// masking the fraction bits the unbiased exponent leaves, nudging across
/// the integer boundary with `__adddf3` when the masked fraction is
/// nonzero.
pub fn floor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x60);
        c.g[A1][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[V0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[V0][0] << 32;
        c.g[S2][0] = ((c.g[S2][0] as i64) >> 32) as u64;
        c.g[S0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
        c.g[V1][0] = sx(((c.g[S0][0] as u32) as i32 >> 20) as u32);
        c.g[V1][0] &= 0x7ff;
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3ff));
        c.g[V0][0] = ((c.g[S1][0] as i64) < 20) as u64;
        let large = c.g[V0][0] == 0;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        if large {
            // Large exponent: integral unless the exponent is huge.
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 52) as u64;
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x413));
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = 0x400;
                let keep = c.g[S1][0] != c.g[V0][0];
                c.g[V0][0] = c.g[A1][0];
                if keep {
                    floor_join_2c(c, frame);
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0041_ee7c);
                    c.g[A0][0] = c.g[A1][0];
                    callee::ADDDF3.call(c, 0x0041_ee7c)?;
                    frame.restore(c, S0, 0x50);
                }
            } else {
                floor_mask_large(c, frame)?;
            }
        } else {
            c.cycles += 2;
            let s1_nonneg = (c.g[S1][0] as i64) >= 0;
            c.g[V0][0] = 0xf0000;
            if s1_nonneg {
                floor_mask_small(c, frame)?;
            } else {
                c.cycles += 8;
                c.g[V0][0] = 0x4a0000;
                c.g[A1][0] = c.ld((c.g[V0][0] as u32).wrapping_sub(0x5808));
                callee::ADDDF3.call(c, 0x0041_edb8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                callee::CMPDF2.call(c, 0x0041_edc4)?;
                if (c.g[V0][0] as i64) <= 0 {
                    floor_combine(c);
                    floor_join_28(c);
                    floor_join_2c(c, frame);
                } else if (c.g[S0][0] as i64) < 0 {
                    c.cycles += 2;
                    c.g[V0][0] = 0x7fff0000;
                    c.cycles += 5;
                    c.g[V0][0] |= 0xffff;
                    c.g[V0][0] = c.g[S0][0] & c.g[V0][0];
                    c.g[V0][0] |= c.g[S2][0];
                    if c.g[V0][0] == 0 {
                        floor_combine(c);
                        floor_join_28(c);
                        floor_join_2c(c, frame);
                    } else {
                        c.cycles += 3;
                        c.g[S0][0] = 0xffff_ffff_bff0_0000;
                        c.g[S2][0] = 0;
                        floor_combine(c);
                        floor_join_28(c);
                        floor_join_2c(c, frame);
                    }
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x7fff0000;
                    c.cycles += 3;
                    c.g[S2][0] = 0;
                    c.g[S0][0] = 0;
                    floor_combine(c);
                    floor_join_28(c);
                    floor_join_2c(c, frame);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// scalbnf

// ---------------------------------------------------------------------------
// scalbnf

/// `float scalbnf(float x, int n)` (0x40f9f8): `x * 2^n` by direct
/// exponent surgery (subnormal inputs normalized first; overflow and
/// underflow saturate through the wide-mul paths).
pub fn scalbnf(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.mfc1(5, 12);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A1][0];
        c.g[A3][0] = 0x7f80_0000;
        c.g[V0][0] = c.g[A2][0] & c.g[A3][0];
        c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 23) as u32);
        let exp_nonzero = c.g[V1][0] != 0;
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        if !exp_nonzero {
            // Zero/subnormal exponent: scale a zero straight; otherwise
            // extract the fraction and normalize it.
            c.cycles += 5;
            c.g[V0][0] = 0x7fff_0000;
            c.g[V0][0] |= 0xffff;
            c.g[V0][0] = c.g[A2][0] & c.g[V0][0];
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.mtc1(5, 0);
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            } else {
                c.cycles += 13;
                c.g[AT][0] = 0x4c00_0000;
                c.mtc1(1, 0);
                c.mtc1(5, 1);
                c.mul_s(1, 1, 0);
                c.mfc1(5, 1);
                c.g[A2][0] = c.g[A1][0];
                c.g[V1][0] = c.g[A1][0] & c.g[A3][0];
                c.g[V0][0] = 0xffff_ffff_ffff_0000;
                c.g[V0][0] |= 0x3cb0;
                let underflow = (c.g[A0][0] as i64) < (c.g[V0][0] as i64);
                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 23) as u32);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x19));
                if underflow {
                    // Very negative shift: multiply out directly.
                    c.cycles += 5;
                    c.g[AT][0] = 0x0da2_0000;
                    c.g[AT][0] |= 0x4260;
                    c.mtc1(1, 0);
                    c.mul_s(0, 1, 0);
                    scalbnf_join(c)?;
                } else {
                    // Shifted exponent needs the range check.
                    scalbnf_shifted(c)?;
                }
            }
        } else {
            // Normal exponent: shift it, testing inf/nan first.
            scalbnf_shifted(c)?;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ds_timetosecs

/// `time_t ds_timetosecs(struct tm *t)` (0x3eb090): the seconds for a
/// date/time record, found by bisecting `ds_secstotime` (about 32 rounds;
/// the head checks events).
pub fn ds_timetosecs(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S2, 0x50);
        frame.save(c, S0, 0x70);
        c.g[S2][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, S3, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        c.g[S3][0] = 0;
        frame.save_ra(c, 0x30);
        c.g[S2][0] |= 0xffff;
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
        // Every exit below leaves `v0` ready except the converged
        // mismatch, which returns zero as-is.
        let mut shift = true;
        loop {
            c.cycles += 10;
            c.g[A0][0] = c.g[SP][0];
            c.g[S1][0] = sx(((c.g[V0][0] as u32) >> 1) as u32);
            c.g[A1][0] = c.g[S1][0] << 32;
            c.g[A1][0] >>= 32;
            callee::DS_SECSTOTIME.call(c, 0x003e_b0d4)?;
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            if c.g[V1][0] == 0 {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                if c.g[V1][0] == 0 {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    if c.g[V1][0] == 0 {
                        c.cycles += 5;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        if c.g[V1][0] == 0 {
                            c.cycles += 5;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                            if c.g[V1][0] == 0 {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                                c.g[V0][0] = c.g[S1][0] << 32;
                                if c.g[V1][0] == 0 {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            // Mismatch: halve the range toward the target.
            c.cycles += 2;
            if (c.g[V1][0] as i64) <= 0 {
                c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            } else {
                c.cycles += 1;
                c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[S2][0] < c.g[S3][0]) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                if event_due(c) {
                    deliver_event_here(c, 0x003e_b0c0);
                }
                continue;
            }
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = 0;
                shift = false;
                break;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S1][0] << 32;
            break;
        }
        if shift {
            c.cycles += 1;
            c.g[V0][0] >>= 32;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore_ra(c, 0x30);
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_paths;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::machine;
    use crate::difftest::{seed_count, Rng, DATA, GAME_GP, SP};
    use crate::game::misc::libm_wrap::LIB_VERSION;

    /// Seeds for the libm fixtures (independent digits).
    pub(crate) const SEEDS: u32 = 64;

    /// Fixture base: scrambled machine, non-zero low bytes on pointers.
    fn libm_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.g[A0][0] = (DATA + 0x400 + (r.next_u32() % 0xf0) * 16) as u64;
        c.g[A1][0] = (r.next_u32() % 7) as u64;
        c.g[A2][0] = (r.next_u32() % 9) as u64;
        r
    }

    #[test]
    fn matherr_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            libm_machine(c, seed, 0x40f8);
            // Exception record: type, name, two double args.
            c.bus.write32(DATA + 0x400, seed);
            c.bus.write32(DATA + 0x404, 0x4a0000 - 0x9088 + seed % 8);
            c.bus
                .write64(DATA + 0x408, 0x3ff8000000000000 + seed as u64);
            c.bus
                .write64(DATA + 0x410, 0x4000000000000000 + seed as u64);
            c.g[A0][0] = (DATA + 0x400) as u64;
        };
        check_paths(ee::s15::f_0040f878, matherr, SEEDS, &[106], &setup);
    }

    #[test]
    fn fmodf_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = libm_machine(c, seed, 0x40d8);
            // (x, y) float classes: zeros, normals, inf, NaN, denormals.
            let cls = seed % 8;
            let bits = |r: &mut Rng| r.next_u32();
            let pair = match cls {
                0 => (0x00000000, 0x00000000),
                1 => (0x3f800000, 0x40000000),
                2 => (0xc0000000, 0x00000000), // y = 0: domain path
                3 => (0x7f800000, 0x3f800000), // x = inf
                4 => (0x7fc00000, 0x3f800000), // x = NaN
                5 => (0x3f800000, 0x7fc00000), // y = NaN
                6 => (0x00000001, 0x3f800000), // x denormal
                _ => (bits(&mut r), bits(&mut r)),
            };
            c.f[12] = pair.0;
            c.f[13] = pair.1;
            // Libm version word steers the sticky/domain arms.
            c.bus
                .write32(LIB_VERSION, [0, 0, 2, 0xffff_ffff][seed as usize % 4]);
        };
        check(ee::s15::f_0040d8d0, fmodf, SEEDS, &setup);
    }

    #[test]
    fn floor_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = libm_machine(c, seed, 0x41ed);
            // Double classes steering every exponent band and sign.
            let bits = match seed % 10 {
                0 => 0x0000000000000000, // +0
                1 => 0x3fb999999999999a, // 0.1
                2 => 0x4059c00000000000, // 100.5
                3 => 0xc059c00000000000, // -100.5
                4 => 0x4330000000000000, // 2^52
                5 => 0x7ff0000000000000, // inf (S1 == 0x400)
                6 => 0x7ff8000000000000, // NaN (S1 == 0x400)
                7 => 0xbff0000000000000, // -1
                8 => 0x0000000000000001, // denormal
                _ => ((r.next_u32() as u64) << 32) | r.next_u32() as u64,
            };
            c.g[A0][0] = bits;
        };
        check_paths(
            ee::s15::f_0041ed60,
            floor,
            SEEDS,
            &[31, 34, 140, 143, 222, 247, 252],
            &setup,
        );
    }

    #[test]
    fn scalbnf_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = libm_machine(c, seed, 0x40f9);
            // Float classes and shifts: zero, normal, subnormal, inf,
            // NaN; n = 0, small, large positive/negative.
            let fbits = match seed % 6 {
                0 => 0x00000000,
                1 => 0x3f800000,
                2 => 0x00800000, // subnormal-ish
                3 => 0x7f800000, // inf
                4 => 0x7fc00000, // NaN
                _ => r.next_u32(),
            };
            c.f[12] = fbits;
            c.g[A0][0] = match seed % 7 {
                0 => 0,
                1 => 1,
                2 => 0xffff_ffff, // -1
                3 => 10,
                4 => 200,
                5 => 0xffff_ff9c, // -100
                _ => (r.next_u32() % 513) as u64 - 256,
            };
        };
        check_paths(
            ee::s15::f_0040f9f8,
            scalbnf,
            SEEDS,
            &[19, 27, 35, 39, 47, 50],
            &setup,
        );
    }

    driven!(ds_timetosecs_gen_driven, ee::s14::f_003eb090);
    driven!(ds_timetosecs_hand_driven, ds_timetosecs);

    #[test]
    fn ds_timetosecs_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = libm_machine(c, seed, 0x3eb0);
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            crate::difftest::elf_image(c);
            // Round-trip date: the seconds the search must find back.
            // (Exact fixed point, so the bisection always converges.)
            let secs = match seed % 8 {
                0 => 0,
                1 => 59,
                2 => 3600,
                3 => 86400,
                4 => 30 * 86400,
                5 => 365 * 86400,
                6 => 366 * 86400,
                _ => r.next_u32() % 2000000000,
            };
            c.g[A0][0] = (DATA + 0x400) as u64;
            c.g[A1][0] = secs as u64;
            c.pc = 0x003e_aeb8;
            ee::s14::ds_secstotime(c);
            c.g[A0][0] = (DATA + 0x400) as u64;
            let mode = schedule(c, &mut r, seed, 0x003e_b090);
            // Deliver-only: suppress VBlank unwinds. The loop calls the
            // match-pc dispatcher `ds_secstotime`, which would unwind
            // mid-call; re-entry restarts this function from the top, so
            // a VBlank inside the callee restarts the whole search
            // instead of resuming it (harness limitation). Events are
            // still delivered at the loop head every ~20k cycles.
            ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                .sched
                .next_vblank = 1 << 40;
            mode
        };
        check_events(
            ds_timetosecs_gen_driven,
            ds_timetosecs_hand_driven,
            SEEDS,
            &setup,
        );
    }
}

/// Scale's join (break 'b_40fb48): reload the return address and the
/// saved input for the epilogue.
fn scalbnf_join(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    Ok(())
}

/// Scale's shifted-exponent dispatch (break 'b_40fa88): an inf/nan input
/// adds the shift directly; anything else goes through the range check.
fn scalbnf_shifted(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = 0xff;
    let finite = c.g[V1][0] != c.g[V0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    if finite {
        scalbnf_range(c)?;
    } else {
        c.cycles += 3;
        c.mtc1(5, 2);
        c.add_s(0, 2, 2);
        scalbnf_join(c)?;
    }
    Ok(())
}

/// Scale's exponent range check: a shifted exponent of `0xff` saturates
/// through the add path; below `-24` it multiplies out; a huge shift
/// saturates through `copysignf`; otherwise the wide multiply.
fn scalbnf_range(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V1][0] as i64) < 255) as u64;
    if c.g[V0][0] == 0 {
        // Shifted exponent stays huge: saturate through `copysignf`.
        scalbnf_at_block(c);
        scalbnf_copysignf_path(c)?;
        return Ok(());
    } else {
        // Small shift: check for a direct build first.
        c.cycles += 2;
        let v1_nonpos = (c.g[V1][0] as i64) <= 0;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        if v1_nonpos {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < -24) as u64;
            let in_range = c.g[V0][0] == 0;
            c.g[V0][0] = 0xc350;
            if in_range {
                // Near normal: widen and multiply.
                c.cycles += 11;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x19));
                c.g[V0][0] = 0xffff_ffff_807f_0000;
                c.g[V0][0] |= 0xffff;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 23);
                c.g[V0][0] = c.g[A2][0] & c.g[V0][0];
                c.g[A1][0] = c.g[V0][0] | c.g[V1][0];
                c.g[AT][0] = 0x3300_0000;
                c.mtc1(1, 0);
                c.mtc1(5, 1);
                c.mul_s(0, 1, 0);
                scalbnf_join(c)?;
                return Ok(());
            } else {
                c.cycles += 6;
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                c.g[AT][0] = 0x0da2_0000;
                c.g[AT][0] |= 0x4260;
                c.mtc1(1, 20);
                if c.g[V0][0] != 0 {
                    // Overflow: force inf through `copysignf`.
                    scalbnf_at_block(c);
                }
                scalbnf_copysignf_path(c)?;
                return Ok(());
            }
        } else {
            // Small positive shift: build the scaled value directly.
            c.cycles += 8;
            c.g[V0][0] = 0xffff_ffff_807f_0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 23);
            c.g[V0][0] |= 0xffff;
            c.g[V0][0] = c.g[A2][0] & c.g[V0][0];
            c.g[A1][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(5, 0);
            c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            return Ok(());
        }
    }
}

/// Scale's `0x7149f2ca` saturate block (break 'b_40fafc): load the inf
/// pattern for the `copysignf` multiply.
fn scalbnf_at_block(c: &mut Cpu) {
    c.cycles += 3;
    c.g[AT][0] = 0x7149_0000;
    c.g[AT][0] |= 0xf2ca;
    c.mtc1(1, 20);
}

/// Scale's `copysignf` tail: `copysignf(f20, f13) * f1`.
fn scalbnf_copysignf_path(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.mtc1(5, 13);
    c.g[RA][0] = sx(0x0040_fb14);
    c.mov_s(12, 20);
    callee::COPYSIGNF.call(c, 0x0040_fb14)?;
    c.mul_s(0, 0, 20);
    scalbnf_join(c)?;
    Ok(())
}
