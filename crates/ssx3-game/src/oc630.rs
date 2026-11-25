//! Staging module oc630 (wave g10.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc630`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_f960, be_bagt_int_v01, "BEBAGTInt_v01"),
];

/// The low 32 bits of a GPR (the address the MIPS code computes).
#[inline(always)]
fn bagt_lo32(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// `BEBAGTInt_v01` (0x14f960; the C++ `cBEBAGTInterface_submit`): copies the
/// profile records of the 3 x 10 table at `D_00530EC0` into `D_004A788E`.
/// Each record is 0x1fe bytes: 15 strides of 0x20 bytes (unaligned
/// `ldl/ldr` pairs when the source is misaligned, else plain `ld/sd`), then a
/// 0x1e-byte tail. Takes no arguments and leaves no registers of interest
/// besides the scratch ones the copy loop writes.
pub fn be_bagt_int_v01(c: &mut Cpu) {
    c.cycles += 14;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
    c.g[V0][0] = 0x4a_0000;
    c.g[V1][0] = 0x53_0000;
    c.sq(S0, sp(c).wrapping_add(0x10));
    c.sq(S1, sp(c));
    c.g[T3][0] = sx(bagt_lo32(c, V0).wrapping_add(0x788e));
    c.g[T2][0] = sx(bagt_lo32(c, V1).wrapping_add(0xec0));
    c.g[A0][0] = 0;
    c.g[T5][0] = 0x9b50;
    c.g[T1][0] = 0xf88;
    c.g[T4][0] = 0x13ec;
    c.g[T0][0] = 0x1fe;
    c.mult(0, 6, 4, 12);
    'outer: loop {
        c.cycles += 4;
        c.mult(0, 24, 4, 13);
        c.g[A1][0] = 0;
        c.g[A3][0] = sx(bagt_lo32(c, A0).wrapping_add(1));
        c.mult(0, 3, 5, 9);
        'record: loop {
            'b_fa58: {
                'b_fa24: {
                    c.cycles += 9;
                    c.mult(0, 4, 5, 8);
                    c.g[V0][0] = sx(bagt_lo32(c, V1).wrapping_add(bagt_lo32(c, T8)));
                    c.g[V1][0] = sx(bagt_lo32(c, A0).wrapping_add(bagt_lo32(c, A2)));
                    c.g[A0][0] = sx(bagt_lo32(c, V0).wrapping_add(bagt_lo32(c, T3)));
                    c.g[V1][0] = sx(bagt_lo32(c, V1).wrapping_add(bagt_lo32(c, T2)));
                    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
                    c.g[V0][0] = c.g[V0][0] & 0x7;
                    let aligned = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(bagt_lo32(c, A1).wrapping_add(0x1));
                    if aligned {
                        break 'b_fa24;
                    }
                    c.cycles += 1;
                    c.g[A1][0] = sx(bagt_lo32(c, V1).wrapping_add(0x1e0));
                    // Misaligned source: 0x20-byte strides with unaligned pairs.
                    'copy_u: loop {
                        c.cycles += 19;
                        c.ldl(14, bagt_lo32(c, V1).wrapping_add(0x7));
                        c.ldr(14, bagt_lo32(c, V1));
                        c.ldl(15, bagt_lo32(c, V1).wrapping_add(0xf));
                        c.ldr(15, bagt_lo32(c, V1).wrapping_add(0x8));
                        c.ldl(16, bagt_lo32(c, V1).wrapping_add(0x17));
                        c.ldr(16, bagt_lo32(c, V1).wrapping_add(0x10));
                        c.ldl(17, bagt_lo32(c, V1).wrapping_add(0x1f));
                        c.ldr(17, bagt_lo32(c, V1).wrapping_add(0x18));
                        c.sdl(14, bagt_lo32(c, A0).wrapping_add(0x7));
                        c.sdr(14, bagt_lo32(c, A0));
                        c.sdl(15, bagt_lo32(c, A0).wrapping_add(0xf));
                        c.sdr(15, bagt_lo32(c, A0).wrapping_add(0x8));
                        c.sdl(16, bagt_lo32(c, A0).wrapping_add(0x17));
                        c.sdr(16, bagt_lo32(c, A0).wrapping_add(0x10));
                        c.sdl(17, bagt_lo32(c, A0).wrapping_add(0x1f));
                        c.sdr(17, bagt_lo32(c, A0).wrapping_add(0x18));
                        c.g[V1][0] = sx(bagt_lo32(c, V1).wrapping_add(0x20));
                        let more = c.g[V1][0] != c.g[A1][0];
                        c.g[A0][0] = sx(bagt_lo32(c, A0).wrapping_add(0x20));
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x0014_f9d0);
                            }
                            continue 'copy_u;
                        }
                        c.cycles += 2;
                        c.g[A1][0] = c.g[V0][0];
                        break 'b_fa58;
                    }
                }
                // Aligned source: 0x20-byte strides with doubleword loads.
                c.cycles += 1;
                c.g[A1][0] = sx(bagt_lo32(c, V1).wrapping_add(0x1e0));
                'copy_a: loop {
                    c.cycles += 11;
                    c.g[T6][0] = c.ld(bagt_lo32(c, V1));
                    c.g[T7][0] = c.ld(bagt_lo32(c, V1).wrapping_add(0x8));
                    c.g[S0][0] = c.ld(bagt_lo32(c, V1).wrapping_add(0x10));
                    c.g[S1][0] = c.ld(bagt_lo32(c, V1).wrapping_add(0x18));
                    c.sd(bagt_lo32(c, A0), c.g[T6][0]);
                    c.sd(bagt_lo32(c, A0).wrapping_add(0x8), c.g[T7][0]);
                    c.sd(bagt_lo32(c, A0).wrapping_add(0x10), c.g[S0][0]);
                    c.sd(bagt_lo32(c, A0).wrapping_add(0x18), c.g[S1][0]);
                    c.g[V1][0] = sx(bagt_lo32(c, V1).wrapping_add(0x20));
                    let more = c.g[V1][0] != c.g[A1][0];
                    c.g[A0][0] = sx(bagt_lo32(c, A0).wrapping_add(0x20));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0014_fa28);
                        }
                        continue 'copy_a;
                    }
                    c.cycles += 1;
                    c.g[A1][0] = c.g[V0][0];
                    break 'b_fa58;
                }
            }
            // The 0x1e-byte tail of the record.
            c.cycles += 21;
            c.ldl(2, bagt_lo32(c, V1).wrapping_add(0x7));
            c.ldr(2, bagt_lo32(c, V1));
            c.ldl(14, bagt_lo32(c, V1).wrapping_add(0xf));
            c.ldr(14, bagt_lo32(c, V1).wrapping_add(0x8));
            c.ldl(15, bagt_lo32(c, V1).wrapping_add(0x17));
            c.ldr(15, bagt_lo32(c, V1).wrapping_add(0x10));
            c.lwl(16, bagt_lo32(c, V1).wrapping_add(0x1b));
            c.lwr(16, bagt_lo32(c, V1).wrapping_add(0x18));
            c.sdl(2, bagt_lo32(c, A0).wrapping_add(0x7));
            c.sdr(2, bagt_lo32(c, A0));
            c.sdl(14, bagt_lo32(c, A0).wrapping_add(0xf));
            c.sdr(14, bagt_lo32(c, A0).wrapping_add(0x8));
            c.sdl(15, bagt_lo32(c, A0).wrapping_add(0x17));
            c.sdr(15, bagt_lo32(c, A0).wrapping_add(0x10));
            c.swl(16, bagt_lo32(c, A0).wrapping_add(0x1b));
            c.swr(16, bagt_lo32(c, A0).wrapping_add(0x18));
            c.g[V0][0] = c.lh(bagt_lo32(c, V1).wrapping_add(0x1c));
            c.sh(bagt_lo32(c, A0).wrapping_add(0x1c), c.g[V0][0]);
            c.g[V0][0] = ((c.g[A1][0] as i64) < 10i64) as u64;
            if c.g[V0][0] != 0 {
                c.mult(0, 3, 5, 9);
                if event_due(c) {
                    deliver_event_here(c, 0x0014_f9a8);
                }
                continue 'record;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[A3][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
            if c.g[V0][0] != 0 {
                c.mult(0, 6, 4, 12);
                if event_due(c) {
                    deliver_event_here(c, 0x0014_f998);
                }
                continue 'outer;
            }
            c.cycles += 4;
            c.lq(S0, sp(c).wrapping_add(0x10));
            c.lq(S1, sp(c));
            let ra = c.g[RA][0] as u32;
            c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
            c.pc = ra;
            return;
        }
    }
}

/// `BEBAGTInt_ResetProfile` (0x14fc40; the C++ `func_0014FC40`): the reverse of
/// [`be_bagt_int_v01`], copying the 3 x 10 records of `D_004A788E` back into
/// `D_00530EC0`. Same 0x1fe-byte records, strides and tail; no stack frame.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn be_bagt_int_reset_profile(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0x13ec;
    c.g[V1][0] = 0x9b50;
    c.mult(0, 7, 4, 2);
    c.mult(0, 6, 4, 3);
    c.g[V0][0] = 0x53_0000;
    c.g[V1][0] = 0x4a_0000;
    c.g[T3][0] = sx(bagt_lo32(c, V0).wrapping_add(0xec0));
    c.g[T2][0] = sx(bagt_lo32(c, V1).wrapping_add(0x788e));
    c.g[A1][0] = 0;
    c.g[T1][0] = 0x1fe;
    c.g[T0][0] = 0xf88;
    c.mult(0, 3, 5, 9);
    'outer: loop {
        'b_fd20: {
            'b_fcec: {
                c.cycles += 9;
                c.mult(0, 4, 5, 8);
                c.g[V0][0] = sx(bagt_lo32(c, V1).wrapping_add(bagt_lo32(c, A3)));
                c.g[V1][0] = sx(bagt_lo32(c, A0).wrapping_add(bagt_lo32(c, A2)));
                c.g[A0][0] = sx(bagt_lo32(c, V0).wrapping_add(bagt_lo32(c, T3)));
                c.g[V1][0] = sx(bagt_lo32(c, V1).wrapping_add(bagt_lo32(c, T2)));
                c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
                c.g[V0][0] = c.g[V0][0] & 0x7;
                let aligned = c.g[V0][0] == 0;
                c.g[V0][0] = sx(bagt_lo32(c, A1).wrapping_add(0x1));
                if aligned {
                    break 'b_fcec;
                }
                c.cycles += 1;
                c.g[A1][0] = sx(bagt_lo32(c, V1).wrapping_add(0x1e0));
                // Misaligned source: 0x20-byte strides with unaligned pairs.
                'copy_u: loop {
                    c.cycles += 19;
                    c.ldl(12, bagt_lo32(c, V1).wrapping_add(0x7));
                    c.ldr(12, bagt_lo32(c, V1));
                    c.ldl(13, bagt_lo32(c, V1).wrapping_add(0xf));
                    c.ldr(13, bagt_lo32(c, V1).wrapping_add(0x8));
                    c.ldl(14, bagt_lo32(c, V1).wrapping_add(0x17));
                    c.ldr(14, bagt_lo32(c, V1).wrapping_add(0x10));
                    c.ldl(15, bagt_lo32(c, V1).wrapping_add(0x1f));
                    c.ldr(15, bagt_lo32(c, V1).wrapping_add(0x18));
                    c.sdl(12, bagt_lo32(c, A0).wrapping_add(0x7));
                    c.sdr(12, bagt_lo32(c, A0));
                    c.sdl(13, bagt_lo32(c, A0).wrapping_add(0xf));
                    c.sdr(13, bagt_lo32(c, A0).wrapping_add(0x8));
                    c.sdl(14, bagt_lo32(c, A0).wrapping_add(0x17));
                    c.sdr(14, bagt_lo32(c, A0).wrapping_add(0x10));
                    c.sdl(15, bagt_lo32(c, A0).wrapping_add(0x1f));
                    c.sdr(15, bagt_lo32(c, A0).wrapping_add(0x18));
                    c.g[V1][0] = sx(bagt_lo32(c, V1).wrapping_add(0x20));
                    let more = c.g[V1][0] != c.g[A1][0];
                    c.g[A0][0] = sx(bagt_lo32(c, A0).wrapping_add(0x20));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0014_fc98);
                        }
                        continue 'copy_u;
                    }
                    c.cycles += 2;
                    c.g[A1][0] = c.g[V0][0];
                    break 'b_fd20;
                }
            }
            // Aligned source: 0x20-byte strides with doubleword loads.
            c.cycles += 1;
            c.g[A1][0] = sx(bagt_lo32(c, V1).wrapping_add(0x1e0));
            'copy_a: loop {
                c.cycles += 11;
                c.g[T4][0] = c.ld(bagt_lo32(c, V1));
                c.g[T5][0] = c.ld(bagt_lo32(c, V1).wrapping_add(0x8));
                c.g[T6][0] = c.ld(bagt_lo32(c, V1).wrapping_add(0x10));
                c.g[T7][0] = c.ld(bagt_lo32(c, V1).wrapping_add(0x18));
                c.sd(bagt_lo32(c, A0), c.g[T4][0]);
                c.sd(bagt_lo32(c, A0).wrapping_add(0x8), c.g[T5][0]);
                c.sd(bagt_lo32(c, A0).wrapping_add(0x10), c.g[T6][0]);
                c.sd(bagt_lo32(c, A0).wrapping_add(0x18), c.g[T7][0]);
                c.g[V1][0] = sx(bagt_lo32(c, V1).wrapping_add(0x20));
                let more = c.g[V1][0] != c.g[A1][0];
                c.g[A0][0] = sx(bagt_lo32(c, A0).wrapping_add(0x20));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0014_fcf0);
                    }
                    continue 'copy_a;
                }
                c.cycles += 1;
                c.g[A1][0] = c.g[V0][0];
                break 'b_fd20;
            }
        }
        // The 0x1e-byte tail of the record.
        c.cycles += 21;
        c.ldl(2, bagt_lo32(c, V1).wrapping_add(0x7));
        c.ldr(2, bagt_lo32(c, V1));
        c.ldl(12, bagt_lo32(c, V1).wrapping_add(0xf));
        c.ldr(12, bagt_lo32(c, V1).wrapping_add(0x8));
        c.ldl(13, bagt_lo32(c, V1).wrapping_add(0x17));
        c.ldr(13, bagt_lo32(c, V1).wrapping_add(0x10));
        c.lwl(14, bagt_lo32(c, V1).wrapping_add(0x1b));
        c.lwr(14, bagt_lo32(c, V1).wrapping_add(0x18));
        c.sdl(2, bagt_lo32(c, A0).wrapping_add(0x7));
        c.sdr(2, bagt_lo32(c, A0));
        c.sdl(12, bagt_lo32(c, A0).wrapping_add(0xf));
        c.sdr(12, bagt_lo32(c, A0).wrapping_add(0x8));
        c.sdl(13, bagt_lo32(c, A0).wrapping_add(0x17));
        c.sdr(13, bagt_lo32(c, A0).wrapping_add(0x10));
        c.swl(14, bagt_lo32(c, A0).wrapping_add(0x1b));
        c.swr(14, bagt_lo32(c, A0).wrapping_add(0x18));
        c.g[V0][0] = c.lh(bagt_lo32(c, V1).wrapping_add(0x1c));
        c.sh(bagt_lo32(c, A0).wrapping_add(0x1c), c.g[V0][0]);
        c.g[V0][0] = ((c.g[A1][0] as i64) < 10i64) as u64;
        if c.g[V0][0] != 0 {
            c.mult(0, 3, 5, 9);
            if event_due(c) {
                deliver_event_here(c, 0x0014_fc70);
            }
            continue 'outer;
        }
        c.cycles += 2;
        ret(c);
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};

    /// Fill `len` bytes at `base` with random words (the record tables).
    fn oc630_fill(c: &mut Cpu, r: &mut Rng, base: u32, len: u32) {
        for off in (0..len).step_by(4) {
            c.bus.write32(base + off, r.next_u32());
        }
    }

    /// Both copy directions read `D_00530EC0` and `D_004A788E` (3 x 10 records
    /// of 0x1fe bytes): random contents in both tables. Record starts fall on
    /// both aligned and misaligned phases, so both copy loops run.
    fn oc630_copy_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x0630_0001 ^ seed as u64);
        scramble(c, &mut r);
        oc630_fill(c, &mut r, 0x0053_0000 + 0xec0, 0x1_4000);
        oc630_fill(c, &mut r, 0x004a_0000 + 0x788e, 0x1_4000);
    }

    #[test]
    fn be_bagt_int_v01_matches() {
        check(ee::s01::f_0014f960__generated, be_bagt_int_v01, 40, &|c, seed| {
            oc630_copy_setup(c, seed);
            c.pc = 0x0014_f960;
        });
    }


}
