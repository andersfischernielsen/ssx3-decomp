//! Staging oc1340 (wave unwinder): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1340`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call;
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::game::lib::gcc::{frame_ld, frame_sd};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0041_4338, execute_cfa_insn, "execute_cfa_insn"),
    (0x0041_49f0, fde_split, "fde_split"),
    (0x0041_4230, extract_cie_info, "extract_cie_info"),
    (0x0041_4148, find_fde, "find_fde"),
    (0x0041_3f80, add_fdes, "add_fdes"),
];

// ---------------------------------------------------------------------------
// Callees (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `int strcmp(const char *, const char *)` (0x4165a8).
    pub const STRCMP: Callee = Callee {
        addr: 0x0041_65a8,
        f: ee::s15::strcmp,
    };

    /// `size_t strlen(const char *)` (0x416810).
    pub const STRLEN: Callee = Callee {
        addr: 0x0041_6810,
        f: ee::s15::strlen,
    };

    /// ULEB128 decoder (`__decode_uleb128`, 0x413b38).
    pub(crate) use crate::game::lib::gcc::callee::DECODE_ULEB128;

    /// `malloc` (0x4188c0).
    pub(crate) use crate::game::lib::gcc::MALLOC;

    /// `free` (0x418910).
    pub(crate) use crate::game::lib::gcc::FREE;

    /// `abort` (0x418790).
    pub(crate) use crate::game::lib::gcc::ABORT;

    /// SLEB128 decoder (`__decode_sleb128`, 0x413b80).
    pub const DECODE_SLEB128: Callee = Callee {
        addr: 0x0041_3b80,
        f: ee::s15::decode_sleb128,
    };

    /// Frame-table initializer (`frame_init`, 0x414008).
    pub const FRAME_INIT: Callee = Callee {
        addr: 0x0041_4008,
        f: ee::s15::frame_init,
    };
}

// ---------------------------------------------------------------------------
// add_fdes

/// `void add_fdes(void *head, uint32_t *min, uint32_t count, uint32_t *max)`
/// (0x413f80): link the frame's FDE table into the sorted registry,
/// tracking the lowest start and the highest end seen.
pub fn add_fdes(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 6;
        c.g[T0][0] = c.g[A0][0];
        c.g[T3][0] = c.g[A1][0];
        c.g[V0][0] = c.lw(c.g[T0][0] as u32);
        c.g[T1][0] = c.lw(c.g[A2][0] as u32);
        let empty = c.g[V0][0] == 0;
        c.g[T2][0] = c.lw(c.g[A3][0] as u32);
        if !empty {
            loop {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                    } else {
                        c.cycles += 15;
                        c.g[A0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = c.lw(c.g[T3][0] as u32);
                        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.sw(c.g[V0][0] as u32, c.g[T0][0]);
                        c.sw((c.g[T3][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xc));
                        c.g[A1][0] = (c.g[V1][0] < c.g[T1][0]) as u64;
                        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        if c.g[A1][0] != 0 {
                            c.g[T1][0] = c.g[V1][0];
                        }
                        c.g[V0][0] = (c.g[T2][0] < c.g[A0][0]) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[T2][0] = c.g[A0][0];
                        }
                        c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                    }
                }
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                if c.g[V1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0041_3f98);
                }
            }
        }
        c.cycles += 3;
        c.sw(c.g[A2][0] as u32, c.g[T1][0]);
        let t = c.g[RA][0] as u32;
        c.sw(c.g[A3][0] as u32, c.g[T2][0]);
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// extract_cie_info

/// `int extract_cie_info(void *cie)` (0x414230): identify the CIE's
/// augmentation (`"z"`-family or plain) and decode its code/data
/// alignment and return-address column; 0 when the augmentation is
/// unknown.
pub fn extract_cie_info(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = c.g[A0][0];
        frame_sd(c, frame, 0x20, c.g[S1][0]);
        frame.save_ra(c, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame_sd(c, frame, 0x30, c.g[S2][0]);
        c.g[A1][0] = 0x4a0000;
        frame_sd(c, frame, 0x10, c.g[S0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6710));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xd));
        c.g[A0][0] = c.g[V0][0];
        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        callee::STRCMP.call(c, 0x0041_426c)?;
        c.g[S0][0] = 0x4a0000;
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6708));
            callee::STRCMP.call(c, 0x0041_4280)?;
            c.g[A0][0] = 0x7a;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                c.g[V0][0] = 0;
                if c.g[V1][0] != c.g[A0][0] {
                    c.cycles += 6;
                    frame.restore_ra(c, 0x40);
                    c.g[S2][0] = frame_ld(c, frame, 0x30);
                    c.g[S1][0] = frame_ld(c, frame, 0x20);
                    c.g[S0][0] = frame_ld(c, frame, 0x10);
                    let t = c.g[RA][0] as u32;
                    frame.close(c);
                    c.pc = t;
                    return Ok(());
                }
            }
        }
        c.cycles += 9;
        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
        callee::STRLEN.call(c, 0x0041_42a0)?;
        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6708));
        c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
        callee::STRCMP.call(c, 0x0041_42b4)?;
        if c.g[V0][0] != 0 {
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0);
        } else {
            c.cycles += 4;
            c.lwl(2, (c.g[S0][0] as u32).wrapping_add(0x3));
            c.lwr(2, c.g[S0][0] as u32);
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        }
        c.cycles += 14;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
        callee::DECODE_ULEB128.call(c, 0x0041_42d8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        callee::DECODE_SLEB128.call(c, 0x0041_42e4)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
        c.g[V1][0] = c.lbu(c.g[S0][0] as u32);
        c.g[A0][0] = 0x7a;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[V0][0] = c.lb(c.g[A1][0] as u32);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        if c.g[V0][0] == c.g[A0][0] {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            callee::DECODE_ULEB128.call(c, 0x0041_4310)?;
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S0][0];
        c.cycles += 6;
        frame.restore_ra(c, 0x40);
        c.g[S2][0] = frame_ld(c, frame, 0x30);
        c.g[S1][0] = frame_ld(c, frame, 0x20);
        c.g[S0][0] = frame_ld(c, frame, 0x10);
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// fde_split

/// `void fde_split(void *fdes, void *out)` (0x4149f0): split the frame's
/// FDE table at each overlapping row, compacting the pieces into `out`
/// (the inner loops walk the sorted rows; the outer one steps the table).
pub fn fde_split(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[T7][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[FP][0]);
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
        c.g[FP][0] = c.g[SP][0];
        c.g[T4][0] = c.g[A1][0];
        c.g[A0][0] |= 0xffff;
        c.g[T8][0] = c.lw((c.g[T7][0] as u32).wrapping_add(0x4));
        c.g[T1][0] = 0;
        c.g[V0][0] = sx((c.g[T8][0] as u32) << 5);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[S0][0] = c.g[SP][0];
        if c.g[T8][0] == 0 {
            c.g[T0][0] = 0;
        } else {
            c.cycles += 2;
            c.g[T5][0] = 0xffff_ffff_ffff_0000;
            c.g[T5][0] |= 0xffff;
            'outer: loop {
                c.cycles += 3;
                c.g[T0][0] = c.g[A0][0];
                c.g[T3][0] = sx((c.g[T1][0] as u32) << 2);
                if c.g[T0][0] != c.g[T5][0] {
                    c.cycles += 11;
                    c.g[A3][0] = c.lw(c.g[T7][0] as u32);
                    c.g[A2][0] = sx((c.g[T0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    c.g[T9][0] = c.g[T3][0];
                    if (c.g[V0][0] as i64) < 0 {
                        c.cycles += 2;
                        c.g[T6][0] = c.lw(c.g[T4][0] as u32);
                        c.g[T2][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                        'inner: loop {
                            c.cycles += 14;
                            c.g[A0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A3][0] as u32));
                            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                            c.g[T0][0] = c.lw(c.g[V1][0] as u32);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T6][0] as u32));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                            c.sw((c.g[T4][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                            c.g[A3][0] = c.lw(c.g[T7][0] as u32);
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A3][0] as u32));
                            c.sw(c.g[V0][0] as u32, 0);
                            if c.g[T0][0] == c.g[T5][0] {
                                // Table end: rejoin past the inner latch.
                                c.cycles += 2;
                                c.g[A0][0] = c.g[T1][0];
                                c.g[V1][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                                break;
                            }
                            c.cycles += 11;
                            c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
                            c.g[V1][0] = sx((c.g[T9][0] as u32).wrapping_add(c.g[A3][0] as u32));
                            c.g[A2][0] = c.g[V0][0];
                            c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A3][0] as u32));
                            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                            c.g[A0][0] = c.g[T1][0];
                            if (c.g[V1][0] as i64) < 0 {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0041_4a80);
                                }
                                continue 'inner;
                            }
                            c.cycles += 2;
                            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                            break;
                        }
                    } else {
                        // Overlap check passed: rejoin past the inner latch.
                        c.cycles += 1;
                        c.g[T2][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                        c.cycles += 2;
                        c.g[A0][0] = c.g[T1][0];
                        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                    }
                } else {
                    // Head sentinel: rejoin past the inner latch.
                    c.cycles += 1;
                    c.g[T2][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                    c.cycles += 2;
                    c.g[A0][0] = c.g[T1][0];
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                }
                c.cycles += 4;
                c.g[T1][0] = c.g[T2][0];
                c.g[V0][0] = (c.g[T1][0] < c.g[T8][0]) as u64;
                c.sw(c.g[V1][0] as u32, c.g[T0][0]);
                if c.g[V0][0] != 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_4a40);
                    }
                    continue 'outer;
                }
                c.cycles += 1;
                c.g[T1][0] = 0;
                break;
            }
        }
        if c.g[T8][0] != 0 {
            // (generated `'b_414b4c` zeroes T0 under a +2 on both arms).
            c.cycles += 2;
            c.g[T0][0] = 0;
            c.cycles += 3;
            c.g[A3][0] = c.lw(c.g[T7][0] as u32);
            c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
            'pack: loop {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
                if c.g[V1][0] == 0 {
                    // Slot empty: latch and keep packing.
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                }
                c.cycles += 4;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[T1][0] < c.g[T8][0]) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_4b20);
                    }
                    continue 'pack;
                }
                break;
            }
        } else {
            // T8 == 0: the +2 and the T0 zero still happen.
            c.cycles += 2;
            c.g[T0][0] = 0;
        }
        c.cycles += 6;
        c.sw((c.g[T7][0] as u32).wrapping_add(0x4), c.g[T0][0]);
        c.g[SP][0] = c.g[FP][0];
        c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// find_fde

/// `void *find_fde(void *pc)` (0x414148): the FDE covering `pc`, or null.
/// Walks the frame registry (initializing each table through `frame_init`
/// once); the scan and the binary search both check events, so an unwind
/// resumes at their heads.
pub fn find_fde(c: &mut Cpu) {
    call::guest(c, |c| {
        if c.pc != 0x0041_4148 && c.pc != 0x0041_4168 && c.pc != 0x0041_41d0 && c.pc != 0x0041_41b4
        {
            c.jump_out(c.pc, 0x0041_4148, 0x0041_422c);
            return Ok(());
        }
        if c.pc == 0x0041_4148 {
            c.cycles += 8;
            let frame = Frame::open(c, 0x30);
            c.g[V0][0] = 0x530000;
            frame_sd(c, frame, 0, c.g[S0][0]);
            frame_sd(c, frame, 0x10, c.g[S1][0]);
            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5218));
            c.g[S1][0] = c.g[A0][0];
            frame.save_ra(c, 0x20);
            c.pc = 0x0041_4168;
        }
        loop {
            match c.pc {
                0x0041_4168 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    c.pc = if c.g[S0][0] == 0 {
                        0x0041_4218
                    } else {
                        0x0041_4170
                    };
                }
                0x0041_4170 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    c.pc = if t { 0x0041_418c } else { 0x0041_417c };
                }
                0x0041_417c => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    callee::FRAME_INIT.call(c, 0x0041_4184)?;
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    c.pc = 0x0041_418c;
                }
                0x0041_418c => {
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        if back_edge_unwinds(c, 0x0041_4168) {
                            return Ok(());
                        }
                        c.pc = 0x0041_4168;
                    } else {
                        c.pc = 0x0041_4194;
                    }
                }
                0x0041_4194 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        if back_edge_unwinds(c, 0x0041_4168) {
                            return Ok(());
                        }
                        c.pc = 0x0041_4168;
                    } else {
                        c.pc = 0x0041_41a4;
                    }
                }
                0x0041_41a4 => {
                    c.cycles += 2;
                    if c.g[S0][0] != 0 {
                        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.pc = 0x0041_41bc;
                    } else {
                        c.pc = 0x0041_41ac;
                    }
                }
                0x0041_41ac => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    c.pc = 0x0041_4218;
                }
                0x0041_41b4 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V1][0];
                    c.pc = 0x0041_4218;
                }
                0x0041_41bc => {
                    c.cycles += 2;
                    c.g[T0][0] = 0;
                    if c.g[A3][0] == 0 {
                        c.pc = 0x0041_4214;
                    } else {
                        c.pc = 0x0041_41c4;
                    }
                }
                0x0041_41c4 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.pc = 0x0041_41d0;
                }
                0x0041_41d0 => {
                    c.cycles += 8;
                    c.g[A2][0] = sx((c.g[V0][0] as u32) >> 1);
                    c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V1][0] as u32);
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = (c.g[S1][0] < c.g[A1][0]) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                        c.pc = 0x0041_41f8;
                    } else {
                        c.pc = 0x0041_41f0;
                    }
                }
                0x0041_41f0 => {
                    c.cycles += 2;
                    c.g[A3][0] = c.g[A2][0];
                    c.pc = 0x0041_4208;
                }
                0x0041_41f8 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    if c.g[V0][0] != 0 {
                        if back_edge_unwinds(c, 0x0041_41b4) {
                            return Ok(());
                        }
                        c.pc = 0x0041_41b4;
                    } else {
                        c.pc = 0x0041_4208;
                    }
                }
                0x0041_4208 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[T0][0] < c.g[A3][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x0041_41d0) {
                            return Ok(());
                        }
                        c.pc = 0x0041_41d0;
                    } else {
                        c.pc = 0x0041_4214;
                    }
                }
                0x0041_4214 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    c.pc = 0x0041_4218;
                }
                0x0041_4218 => {
                    c.cycles += 5;
                    let frame = Frame { size: 0x30 };
                    frame.restore_ra(c, 0x20);
                    c.g[S1][0] = frame_ld(c, frame, 0x10);
                    c.g[S0][0] = frame_ld(c, frame, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => break,
            }
        }
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// execute_cfa_insn

/// `void execute_cfa_insn(void *cie, void *fde, uint64_t pc)` (0x414338):
/// run one DWARF CFA opcode from `s0` against the frame state at `s1`,
/// returning the next opcode address in `v0`. A flat `pc` dispatch over the
/// entry and every resume head (the generated `__entries` twin is flat the
/// same way). Opcode bytes dispatch through the rodata table.
pub fn execute_cfa_insn(c: &mut Cpu) {
    call::guest(c, |c| {
        let mut pc = c.pc;
        if pc != 0x00414338
            && pc != 0x00414368
            && pc != 0x0041437c
            && pc != 0x00414388
            && pc != 0x00414394
            && pc != 0x004143a0
            && pc != 0x004143b0
            && pc != 0x004143bc
            && pc != 0x004143d4
            && pc != 0x004143e8
            && pc != 0x004143f8
            && pc != 0x00414414
            && pc != 0x00414420
            && pc != 0x00414430
            && pc != 0x0041443c
            && pc != 0x0041444c
            && pc != 0x0041448c
            && pc != 0x004144ac
            && pc != 0x004144f4
            && pc != 0x00414524
            && pc != 0x00414540
            && pc != 0x0041455c
            && pc != 0x00414570
            && pc != 0x004145a4
            && pc != 0x004145c4
            && pc != 0x004145d4
            && pc != 0x00414608
            && pc != 0x0041462c
            && pc != 0x00414648
            && pc != 0x00414688
            && pc != 0x00414690
            && pc != 0x004146ac
            && pc != 0x004146b4
            && pc != 0x004146b8
        {
            c.jump_out(c.pc, 0x0041_4338, 0x0041_46d0);
            return Ok(());
        }
        loop {
            match pc {
                // Entry: open the frame, fetch the opcode byte.
                0x00414338 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S2][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S1][0]);
                    c.g[S2][0] = c.g[A2][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                    c.g[S1][0] = c.g[A1][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V1][0] = c.lbu(c.g[S0][0] as u32);
                    c.g[V0][0] = c.g[V1][0] & 0x40;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0041437c;
                        continue;
                    }
                    pc = 0x00414368;
                    continue;
                }
                // DW_CFA_advance_loc (0x40-0x7f): scale the delta by the code factor.
                0x00414368 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V1][0] & 0x3f;
                    c.g[V1][0] = c.lw(c.g[A3][0] as u32);
                    let t = true;
                    c.mult(0, 2, 2, 4);
                    if t {
                        pc = 0x00414414;
                        continue;
                    }
                    pc = 0x0041437c;
                    continue;
                }
                // Plain opcodes (bits 7 and 6 clear).
                0x0041437c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V1][0] & 0x80;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[V1][0] & 0x3f;
                    if t {
                        pc = 0x00414394;
                        continue;
                    }
                    pc = 0x00414388;
                    continue;
                }
                // Advance path.
                0x00414388 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    if t {
                        pc = 0x0041444c;
                        continue;
                    }
                    pc = 0x00414394;
                    continue;
                }
                // Offset path (bit 7 set).
                0x00414394 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V1][0] & 0xc0;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[V1][0] & 0x3f;
                    if t {
                        pc = 0x004143b0;
                        continue;
                    }
                    pc = 0x004143a0;
                    continue;
                }
                // Remember the advance.
                0x004143a0 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    let t = true;
                    c.sb((c.g[V1][0] as u32).wrapping_add(0x29c), 0u64);
                    if t {
                        pc = 0x004146b4;
                        continue;
                    }
                    pc = 0x004143b0;
                    continue;
                }
                // Opcodes >= 0x2f go to the GNU/abort tail.
                0x004143b0 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V1][0] < 0x2fu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x004146ac;
                        continue;
                    }
                    pc = 0x004143bc;
                    continue;
                }
                // Dispatch through the handler table (rodata at
                // `0x4a0000 - 0x6700`: one handler address per opcode byte).
                0x004143bc => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6700));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;
                    pc = t;
                    continue;
                }
                // DW_CFA_set_loc (0x01): set the location from an address operand.
                0x004143d4 => {
                    c.cycles += 5;
                    c.lwl(2, (c.g[S0][0] as u32).wrapping_add(0x3));
                    c.lwr(2, c.g[S0][0] as u32);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    let t = true;
                    c.sw(c.g[A3][0] as u32, c.g[V0][0]);
                    if t {
                        pc = 0x004146b4;
                        continue;
                    }
                    pc = 0x004143e8;
                    continue;
                }
                // DW_CFA_advance_loc1 (0x02): 1-byte delta.
                0x004143e8 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lbu(c.g[S0][0] as u32);
                    c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                    let t = true;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00414430;
                        continue;
                    }
                    pc = 0x004143f8;
                    continue;
                }
                // DW_CFA_advance_loc2 (0x03): 2-byte delta.
                0x004143f8 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    c.g[V0][0] = c.g[V0][0] << 8;
                    c.g[V1][0] = c.lw(c.g[A3][0] as u32);
                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = c.g[V0][0] & 0xffff;
                    pc = 0x00414414;
                    continue;
                }
                // Advance join.
                0x00414414 => {
                    c.cycles += 3;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.sw(c.g[A3][0] as u32, c.g[V1][0]);
                    if t {
                        pc = 0x004146b4;
                        continue;
                    }
                    pc = 0x00414420;
                    continue;
                }
                // DW_CFA_advance_loc4 (0x04): 4-byte delta.
                0x00414420 => {
                    c.cycles += 4;
                    c.lwl(3, (c.g[S0][0] as u32).wrapping_add(0x3));
                    c.lwr(3, c.g[S0][0] as u32);
                    c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    pc = 0x00414430;
                    continue;
                }
                // Location join.
                0x00414430 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let t = true;
                    c.sw(c.g[A3][0] as u32, c.g[V0][0]);
                    if t {
                        pc = 0x004146b4;
                        continue;
                    }
                    pc = 0x0041443c;
                    continue;
                }
                // DW_CFA_offset_extended (0x05).
                0x0041443c => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00414448);
                    c.g[A1][0] = c.g[SP][0] | 0x4;
                    callee::DECODE_ULEB128.call(c, 0x00414448)?;
                    c.g[A0][0] = c.g[V0][0];
                    pc = 0x0041444c;
                    continue;
                }
                // Offset body.
                0x0041444c => {
                    c.cycles += 16;
                    c.g[RA][0] = sx(0x00414454);
                    c.g[A1][0] = c.g[SP][0];
                    callee::DECODE_ULEB128.call(c, 0x00414454)?;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[A2][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.mult(0, 4, 4, 5);
                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 3);
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sb((c.g[V0][0] as u32).wrapping_add(0x29c), c.g[A2][0]);
                    c.sw(c.g[SP][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = c.g[A0][0];
                    let t = true;
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    if t {
                        pc = 0x004146b4;
                        continue;
                    }
                    pc = 0x0041448c;
                    continue;
                }
                // DW_CFA_restore_extended (0x06).
                0x0041448c => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00414498);
                    c.g[A1][0] = c.g[SP][0] | 0x4;
                    callee::DECODE_ULEB128.call(c, 0x00414498)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let t = true;
                    c.sb((c.g[V1][0] as u32).wrapping_add(0x29c), 0u64);
                    if t {
                        pc = 0x004146b4;
                        continue;
                    }
                    pc = 0x004144ac;
                    continue;
                }
                // DW_CFA_register (0x09).
                0x004144ac => {
                    c.cycles += 18;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x004144b8);
                    c.g[A1][0] = c.g[SP][0] | 0x4;
                    callee::DECODE_ULEB128.call(c, 0x004144b8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x004144c4);
                    c.g[A1][0] = c.g[SP][0] | 0x8;
                    callee::DECODE_ULEB128.call(c, 0x004144c4)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x2;
                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.g[V1][0] << 32;
                    c.sb((c.g[A0][0] as u32).wrapping_add(0x29c), c.g[A1][0]);
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.g[V1][0] >> 32;
                    let t = true;
                    c.sd((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    if t {
                        pc = 0x004146b4;
                        continue;
                    }
                    pc = 0x004144f4;
                    continue;
                }
                // DW_CFA_def_cfa (0x0c).
                0x004144f4 => {
                    c.cycles += 12;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00414500);
                    c.g[A1][0] = c.g[SP][0] | 0x4;
                    callee::DECODE_ULEB128.call(c, 0x00414500)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0041450c);
                    c.g[A1][0] = c.g[SP][0];
                    callee::DECODE_ULEB128.call(c, 0x0041450c)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    c.sh((c.g[S1][0] as u32).wrapping_add(0x298), c.g[V1][0]);
                    let t = true;
                    c.sd((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t {
                        pc = 0x004146b4;
                        continue;
                    }
                    pc = 0x00414524;
                    continue;
                }
                // DW_CFA_def_cfa_register (0x0d).
                0x00414524 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00414530);
                    c.g[A1][0] = c.g[SP][0] | 0x4;
                    callee::DECODE_ULEB128.call(c, 0x00414530)?;
                    c.g[V1][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[S0][0] = c.g[V0][0];
                    let t = true;
                    c.sh((c.g[S1][0] as u32).wrapping_add(0x298), c.g[V1][0]);
                    if t {
                        pc = 0x004146b4;
                        continue;
                    }
                    pc = 0x00414540;
                    continue;
                }
                // DW_CFA_def_cfa_offset (0x0e).
                0x00414540 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0041454c);
                    c.g[A1][0] = c.g[SP][0];
                    callee::DECODE_ULEB128.call(c, 0x0041454c)?;
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[S0][0] = c.g[V0][0];
                    let t = true;
                    c.sd((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    if t {
                        pc = 0x004146b4;
                        continue;
                    }
                    pc = 0x0041455c;
                    continue;
                }
                // DW_CFA_remember_state (0x0a): snapshot the register rules.
                0x0041455c => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x00414564);
                    c.g[A0][0] = 0x2f8;
                    callee::MALLOC.call(c, 0x00414564)?;
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2e0));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = c.g[S1][0];
                    pc = 0x00414570;
                    continue;
                }
                // Remember copy loop.
                0x00414570 => {
                    c.cycles += 13;
                    c.g[A2][0] = c.ld(c.g[V1][0] as u32);
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[A2][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[A1][0];
                    if t {
                        if back_edge_unwinds(c, 0x00414570) {
                            return Ok(());
                        }
                        pc = 0x00414570;
                        continue;
                    }
                    pc = 0x004145a4;
                    continue;
                }
                // Remember tail.
                0x004145a4 => {
                    c.cycles += 8;
                    c.g[A1][0] = c.ld(c.g[V1][0] as u32);
                    c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[A0][0] as u32, c.g[A1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x2f0), c.g[V0][0]);
                    if t {
                        pc = 0x004146b4;
                        continue;
                    }
                    pc = 0x004145c4;
                    continue;
                }
                // DW_CFA_restore_state (0x0b): restore the snapshot.
                0x004145c4 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2f0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2e0));
                    c.g[A2][0] = c.g[A0][0];
                    pc = 0x004145d4;
                    continue;
                }
                // Restore copy loop.
                0x004145d4 => {
                    c.cycles += 13;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[V0][0];
                    if t {
                        if back_edge_unwinds(c, 0x004145d4) {
                            return Ok(());
                        }
                        pc = 0x004145d4;
                        continue;
                    }
                    pc = 0x00414608;
                    continue;
                }
                // Restore tail.
                0x00414608 => {
                    c.cycles += 9;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[RA][0] = sx(0x00414624);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    callee::FREE.call(c, 0x00414624)?;
                    let t = true;
                    c.g[V0][0] = c.g[S0][0];
                    if t {
                        pc = 0x004146b8;
                        continue;
                    }
                    pc = 0x0041462c;
                    continue;
                }
                // GNU window_save (0x2d): spill the register window.
                0x0041462c => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x10;
                    c.g[A0][0] = 0x10;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[A3][0] = 0x10;
                    c.g[T1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x29c));
                    c.g[T0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
                    pc = 0x00414648;
                    continue;
                }
                // Window spill loop.
                0x00414648 => {
                    c.cycles += 16;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
                    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                    c.sb(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[V0][0] = c.g[V0][0] << 32;
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = c.g[V0][0] >> 32;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                    c.g[A3][0] = c.g[A2][0];
                    c.sd(c.g[A0][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = (c.g[A3][0] < 0x20u64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[A3][0];
                    if t {
                        if back_edge_unwinds(c, 0x00414648) {
                            return Ok(());
                        }
                        pc = 0x00414648;
                        continue;
                    }
                    pc = 0x00414688;
                    continue;
                }
                // Window done.
                0x00414688 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.g[S0][0];
                    if t {
                        pc = 0x004146b8;
                        continue;
                    }
                    pc = 0x00414690;
                    continue;
                }
                // GNU args_size (0x2e): record the argument size.
                0x00414690 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0041469c);
                    c.g[A1][0] = c.g[SP][0];
                    callee::DECODE_ULEB128.call(c, 0x0041469c)?;
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[S0][0] = c.g[V0][0];
                    let t = true;
                    c.sd((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    if t {
                        pc = 0x004146b4;
                        continue;
                    }
                    pc = 0x004146ac;
                    continue;
                }
                // Unknown opcode: abort.
                0x004146ac => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x004146b4);
                    callee::ABORT.call(c, 0x004146b4)?;
                    pc = 0x004146b4;
                    continue;
                }
                // Join: the next opcode is at `s0`.
                0x004146b4 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S0][0];
                    pc = 0x004146b8;
                    continue;
                }
                // Epilogue: close the frame, return `s0`.
                0x004146b8 => {
                    c.cycles += 6;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0041_4338, 0x0041_46d0);
                    return Ok(());
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_paths;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::machine;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    /// Seeds for the unwinder fixtures.
    pub(crate) const SEEDS: u32 = 64;

    /// Scratch tables for the FDE walk: node words, min/max holders.
    pub(crate) const TAB: u32 = DATA + 0x400;
    pub(crate) const NODES: u32 = DATA + 0x600;

    /// Fixture base: scrambled machine with gp set.
    fn unwind_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.g[GP][0] = crate::difftest::GAME_GP as u64;
        c.g[A0][0] = (r.next_u32() % 0x1000) as u64;
        c.g[A1][0] = (r.next_u32() % 0x1000) as u64;
        c.g[A2][0] = (r.next_u32() % 0x1000) as u64;
        c.g[A3][0] = (r.next_u32() % 0x1000) as u64;
        r
    }

    #[test]
    fn add_fdes_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = unwind_machine(c, seed, 0x3f80);
            // Table struct (base + count) and min/max holders.
            c.bus.write32(TAB, DATA + 0x500);
            c.bus.write32(TAB + 4, (seed % 3) as u32);
            for i in 0..8u32 {
                c.bus.write32(DATA + 0x500 + i * 4, 0);
            }
            c.bus.write32(DATA + 0x700, 0xffff_ffff);
            c.bus.write32(DATA + 0x704, 0);
            // FDE chain: two nodes; the count/valid words steer the
            // quick skips vs the merge arm.
            for (n, on) in [(0u32, seed % 2 == 0), (1u32, seed % 3 == 0)] {
                let at = NODES + n * 16;
                c.bus.write32(at, if n == 0 { NODES + 16 } else { 0 });
                c.bus.write32(at + 4, if on { 1 } else { 0 });
                c.bus
                    .write32(at + 8, if on { (r.next_u32() % 0x1000) + 1 } else { 0 });
                c.bus.write32(at + 0xc, r.next_u32() % 0x1000);
            }
            c.g[A0][0] = NODES as u64;
            c.g[A1][0] = TAB as u64;
            c.g[A2][0] = (DATA + 0x700) as u64;
            c.g[A3][0] = (DATA + 0x704) as u64;
        };
        check_paths(ee::s15::f_00413f80, add_fdes, SEEDS, &[16, 34], &setup);
    }

    driven!(fde_split_gen_driven, ee::s15::f_004149f0);
    driven!(fde_split_hand_driven, fde_split);

    #[test]
    fn fde_split_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = unwind_machine(c, seed, 0x4149);
            // FDE table: entry count steered small; rows point at
            // scratch FDEs with sorted ranges.
            let n = 1 + (seed % 3) as u32;
            c.bus.write32(DATA + 0x400, DATA + 0x500);
            c.bus.write32(DATA + 0x404, n);
            for i in 0..4u32 {
                let e = DATA + 0x600 + i * 16;
                c.bus.write32(e, if i < n { 0x10 } else { 0 });
                c.bus
                    .write32(e + 4, if i < n { 1 + (seed + i) % 3 } else { 0 });
                c.bus
                    .write32(e + 8, if i < n { 0x1000 * (i + 1) } else { 0 });
                c.bus.write32(e + 0xc, (seed * 37 + i * 11 + 1) % 0x100);
                c.bus.write32(DATA + 0x500 + i * 4, e);
            }
            c.bus.write32(DATA + 0x700, 0);
            c.g[A0][0] = (DATA + 0x400) as u64;
            c.g[A1][0] = (DATA + 0x700) as u64;
            let mode = schedule(c, &mut r, seed, 0x0041_49f0);
            // Deliver-only (same harness reason as ds_timetosecs).
            ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                .sched
                .next_vblank = 1 << 40;
            mode
        };
        check_paths(
            ee::s15::f_004149f0,
            fde_split,
            SEEDS,
            &[53, 85, 117],
            &|c, seed| {
                setup(c, seed);
            },
        );
        check_events(fde_split_gen_driven, fde_split_hand_driven, SEEDS, &setup);
    }

    driven!(find_fde_gen_driven, ee::s15::f_00414148);
    driven!(find_fde_hand_driven, find_fde);

    /// Frame-registry head the unwinder walks (real game address, in RAM).
    pub(crate) const REG_HEAD: u32 = 0x0052_ade8;
    /// Registry objects: low, high, fde array, table, count, next.
    pub(crate) const OBJ0: u32 = DATA + 0x600;

    #[test]
    fn find_fde_matches() {
        fn setup_paths(c: &mut Cpu, seed: u32) -> Rng {
            let r = unwind_machine(c, seed, 0x4148);
            // Up to two objects with sorted non-overlapping rows; row
            // starts stay inside [low, high).
            let n_objs = 1 + (seed % 2);
            for i in 0..2u32 {
                let obj = OBJ0 + i * 0x40;
                let low = 0x1_0000 * (i + 1);
                let count = 1 + ((seed + i) % 3);
                let tbl = DATA + 0x700 + i * 0x40;
                for j in 0..4u32 {
                    let row = DATA + 0x800 + (i * 4 + j) * 0x10;
                    let on = i < n_objs && j < count;
                    let start = low + 0x100 + j * 0x200;
                    c.bus.write32(row + 8, if on { start } else { 0 });
                    c.bus.write32(row + 0xc, if on { 0x80 } else { 0 });
                    c.bus.write32(tbl + j * 4, if on { row } else { 0 });
                }
                c.bus.write32(obj, low);
                c.bus.write32(obj + 4, low + 0x1000);
                c.bus.write32(obj + 8, 0);
                c.bus.write32(obj + 0xc, tbl);
                c.bus.write32(obj + 0x10, count);
                c.bus
                    .write32(obj + 0x14, if i + 1 < n_objs { obj + 0x40 } else { 0 });
            }
            // Seed 0 mod 7: empty registry (immediate null return).
            // Seed 6 mod 7: first object uninitialized (frame_init path,
            // with a zeroed FDE chain; odd quarters keep the table word
            // set so the counting loop runs too).
            if seed % 7 == 0 {
                c.bus.write32(REG_HEAD, 0);
            } else {
                c.bus.write32(REG_HEAD, OBJ0);
            }
            if seed % 7 == 6 {
                c.bus.write32(OBJ0, 0);
                c.bus.write32(DATA + 0x900, 0);
                c.bus.write32(DATA + 0x904, 0);
                c.bus.write32(DATA + 0x910, DATA + 0x900);
                c.bus.write32(DATA + 0x914, 0);
                c.bus.write32(OBJ0 + 8, DATA + 0x900);
                if (seed / 7) % 2 == 0 {
                    c.bus.write32(OBJ0 + 0xc, 0);
                } else {
                    c.bus.write32(OBJ0 + 0xc, DATA + 0x910);
                }
            }
            // Looked-up pc: below all, row starts, row interiors, the
            // high edge, the last row start, past the end. Two-object
            // seeds steer at the second object, so the scan walks further.
            let low0 = 0x1_0000u32;
            let low = if n_objs == 2 && matches!(seed % 6, 1 | 2 | 4) {
                0x2_0000u32
            } else {
                low0
            };
            let count = if low == low0 {
                1 + (seed % 3)
            } else {
                1 + ((seed + 1) % 3)
            };
            let pc = match seed % 6 {
                0 => low0 - 4,
                1 => low + 0x100,
                2 => low + 0x110,
                3 => low0 + 0x1000,
                4 if count > 1 => low + 0x100 + 0x200,
                4 => low + 0x180,
                _ => low0 + 0x2000,
            };
            c.g[A0][0] = pc as u64;
            c.pc = 0x0041_4148;
            r
        }
        check_paths(
            ee::s15::f_00414148,
            find_fde,
            SEEDS,
            &[15, 22, 33, 45, 58, 1048, 1075],
            &|c, seed| {
                setup_paths(c, seed);
            },
        );
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = setup_paths(c, seed);
            schedule(c, &mut r, seed, 0x0041_4148)
        };
        check_events(find_fde_gen_driven, find_fde_hand_driven, SEEDS, &setup);
    }

    driven!(cfa_gen_driven, ee::s15::f_00414338);
    driven!(cfa_hand_driven, execute_cfa_insn);

    /// CFA fixtures: frame-state scratch, opcode stream, CIE factors, loc.
    pub(crate) const CFA_STATE: u32 = DATA + 0x400;
    pub(crate) const CFA_STREAM: u32 = DATA + 0x900;
    pub(crate) const CFA_CIE: u32 = DATA + 0xa00;
    pub(crate) const CFA_LOC: u32 = DATA + 0xb00;
    pub(crate) const CFA_TABLE: u32 = 0x0043_9900;

    #[test]
    fn execute_cfa_insn_matches() {
        // Handler per opcode (the table is built at runtime; the fixture
        // writes it the same way). Unlisted entries join out safely.
        const HANDLERS: [(u32, u32); 15] = [
            (0x01, 0x0041_43d4),
            (0x02, 0x0041_43e8),
            (0x03, 0x0041_43f8),
            (0x04, 0x0041_4420),
            (0x05, 0x0041_443c),
            (0x06, 0x0041_448c),
            (0x09, 0x0041_44ac),
            (0x0a, 0x0041_455c),
            (0x0b, 0x0041_45c4),
            (0x0c, 0x0041_44f4),
            (0x0d, 0x0041_4524),
            (0x0e, 0x0041_4540),
            (0x2d, 0x0041_462c),
            (0x2e, 0x0041_4690),
            (0x00, 0x0041_46b4),
        ];
        // One opcode per seed: every table arm, the GNU pair, the abort
        // tail, advance/offset immediates, plus looping-op repeats.
        const OPS: [u32; 28] = [
            0x01, 0x0b, 0x03, 0x04, 0x05, 0x06, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x2d, 0x2e,
            0x2f, 0x40, 0x41, 0x7f, 0x80, 0x81, 0xbf, 0x00, 0x0a, 0x02, 0x2d, 0x0a, 0x0b, 0x2d,
        ];
        fn setup_paths(c: &mut Cpu, seed: u32, for_events: bool) -> Rng {
            let r = unwind_machine(c, seed, 0x4338);
            crate::difftest::elf_image(c);
            for op in 0..0x30u32 {
                let mut h = 0x0041_46b4;
                for (o, hh) in HANDLERS {
                    if o == op {
                        h = hh;
                    }
                }
                c.bus.write32(CFA_TABLE + op * 4, h);
            }
            for i in 0..0x100u32 {
                c.bus.write32(CFA_STATE + i * 4, 0xa5a5_a5a5);
            }
            // A NULL snapshot: restore_state copies zeros, then frees NULL.
            c.bus.write32(CFA_STATE + 0x2f0, 0);
            let mut op = OPS[(seed as usize) % OPS.len()];
            if for_events && op == 0x2f {
                // The abort tail returns without an unwind (real `abort`
                // on both sides); drive() cannot re-enter past it, so the
                // events run exercises the join arm instead. The abort arm
                // itself is still pinned by check_paths above.
                op = 0x00;
            }
            c.bus.write8(CFA_STREAM, op as u8);
            let b = (1 + (seed % 5)) as u8;
            for i in 1..16u32 {
                c.bus.write8(CFA_STREAM + i, b);
            }
            c.bus.write32(CFA_CIE + 8, 1 + (seed % 3));
            c.bus.write32(CFA_CIE + 0xc, 0xffff_fffc);
            c.bus.write32(CFA_LOC, 0x1234_5678);
            c.g[A0][0] = CFA_STREAM as u64;
            c.g[A1][0] = CFA_STATE as u64;
            c.g[A2][0] = CFA_CIE as u64;
            c.g[A3][0] = CFA_LOC as u64;
            c.pc = 0x0041_4338;
            r
        }
        check_paths(
            ee::s15::f_00414338,
            execute_cfa_insn,
            SEEDS,
            &[27, 34, 39, 41, 44, 50, 51, 64, 70, 72, 149, 298, 445, 828],
            &|c, seed| {
                setup_paths(c, seed, false);
            },
        );
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = setup_paths(c, seed, true);
            schedule(c, &mut r, seed, 0x0041_4338)
        };
        check_events(cfa_gen_driven, cfa_hand_driven, SEEDS, &setup);
    }

    /// CIE info record the extractor fills (augmentation at +0xd when the
    /// length word is zero).
    pub(crate) const CIE: u32 = DATA + 0x800;
    pub(crate) const CIE_INFO: u32 = DATA + 0x900;

    #[test]
    fn extract_cie_info_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = unwind_machine(c, seed, 0x4230);
            crate::difftest::elf_image(c);
            // Augmentation strings: empty, `z`-family, `eh`, other.
            let aug: &[u8] = match seed % 4 {
                0 => b"\0",
                1 => b"zR\0",
                2 => b"eh\0",
                _ => b"x\0",
            };
            c.bus.write32(CIE + 4, 0);
            for (i, b) in aug.iter().enumerate() {
                c.bus.write8(CIE + 0xd + i as u32, *b);
            }
            // Code/data alignment + return column scratch.
            for i in 0..8u32 {
                c.bus.write32(CIE_INFO + i * 4, r.next_u32());
            }
            c.g[A0][0] = CIE as u64;
            c.g[A1][0] = CIE_INFO as u64;
        };
        check_paths(
            ee::s15::f_00414230,
            extract_cie_info,
            SEEDS,
            &[60, 104, 165, 181],
            &setup,
        );
    }
}
