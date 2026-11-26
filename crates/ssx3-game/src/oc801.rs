//! Staging module oc801 (wave wave247): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc801`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0019_de18,
        bolt_file_compare_texture_slots,
        "BoltFile_CompareTextureSlots",
    ),
    (
        0x0019_df20,
        bolt_file_prioritize_models,
        "BoltFile_PrioritizeModels",
    ),
    (
        0x0019_e070,
        bolt_file_prioritize_textures,
        "BoltFile_PrioritizeTextures",
    ),
];

/// `int BoltFile_CompareTextureSlots(const int* first, const int* second)` (0x19de18): the
/// qsort comparator that puts the loaded (state 3), type-11 slots with no load stamp
/// first and orders them by name (`strcasecmp`); `-1`/`1` when only the first/second is
/// such a slot, `0` when neither is. Keys are `(bank << 16) | slot` into the slot table
/// at `gp - 0x1784` (banks of 0x12000 bytes, slots of 0x120).
pub fn bolt_file_compare_texture_slots(c: &mut Cpu) {
    if c.pc != 0x0019de18 {
        return ee::s03::f_0019de18__generated(c);
    }
    guest(c, |c| {
        'b_19df14: {
            'b_19df10: {
                'b_19defc: {
                    'b_19deb0: {
                        c.cycles += 20;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                        c.g[V0][0] = 0x10000;
                        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                        c.g[V0][0] = c.g[V0][0] | 0x2000;
                        c.g[T0][0] = 0x120;
                        c.g[A3][0] = 0x3;
                        c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                        c.g[A2][0] = c.lhu(c.g[A1][0] as u32);
                        c.g[A0][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
                        c.mult(0, 4, 4, 2);
                        c.g[V1][0] = c.g[V1][0] & 0xffff;
                        c.mult(0, 3, 3, 8);
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1784));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.mult(0, 3, 6, 8);
                        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                        if c.g[V1][0] != c.g[A3][0] {
                            c.g[A2][0] = c.lw(c.g[A1][0] as u32);
                            break 'b_19deb0;
                        } else {
                            c.cycles += 4;
                            c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x110));
                            c.g[V0][0] = 0xb;
                            if c.g[A2][0] != c.g[V0][0] {
                                c.g[A2][0] = c.lw(c.g[A1][0] as u32);
                                break 'b_19deb0;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x11c));
                                if c.g[V0][0] != 0u64 {
                                    c.g[A2][0] = c.lw(c.g[A1][0] as u32);
                                    break 'b_19deb0;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                                    let t = c.g[V0][0] != c.g[V1][0];
                                    c.g[V0][0] = 0xffffffffffffffff;
                                    if t {
                                        break 'b_19df10;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x110));
                                        let t = c.g[V0][0] != c.g[A2][0];
                                        c.g[V0][0] = 0xffffffffffffffff;
                                        if t {
                                            break 'b_19df10;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[A1][0] as u32).wrapping_add(0x11c));
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0xffffffffffffffff;
                                            if t {
                                                break 'b_19defc;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                                if t {
                                                    break 'b_19df14;
                                                } else {
                                                    break 'b_19deb0;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_19df10;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x110));
                        c.g[V0][0] = 0xb;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_19df10;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x11c));
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_19df10;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                let t = c.g[V0][0] != c.g[A2][0];
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_19df10;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x110));
                                    let t = c.g[V0][0] != c.g[V1][0];
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_19df10;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x11c));
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_19df10;
                                        } else {
                                            break 'b_19defc;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 5;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[RA][0] = sx(0x0019df08);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                call_ptr(c, 0x0041aa88, 0x0019df08)?;
                let t = true;
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                if t {
                    break 'b_19df14;
                } else {
                    break 'b_19df10;
                }
            }
            c.cycles += 1;
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            break 'b_19df14;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `void BoltFile_PrioritizeModels(BoltTable* self, int bank)` (0x19df20): sorts the bank's 256
/// slot keys with the model comparator, then walks them in order, stamping each unstamped
/// type-8 (model) slot with the bank's running load counter and starting its load when
/// nothing is pending and the slot is in state 3. Stops at the first slot that is stamped or
/// of another type.
pub fn bolt_file_prioritize_models(c: &mut Cpu) {
    if c.pc != 0x0019df20 {
        return ee::s03::f_0019df20__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0019df20;
        loop {
            match pc {
                0x0019df20 => {
                    c.cycles += 16;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x4a0));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x440));
                    c.g[V1][0] = c.g[SP][0];
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x430));
                    c.g[S5][0] = c.g[A1][0];
                    c.g[S6][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x490));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x480));
                    c.g[A1][0] = sx((c.g[S5][0] as u32) << 16);
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x470));
                    c.g[A0][0] = 0u64;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x460));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x450));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x420));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x410));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x400), c.g[RA][0]);
                    pc = 0x0019df60;
                    continue;
                }
                0x0019df60 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.g[A1][0] | c.g[A0][0];
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 256i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0019df60) {
                            return Ok(());
                        }
                        pc = 0x0019df60;
                        continue;
                    }
                    pc = 0x0019df7c;
                    continue;
                }
                0x0019df7c => {
                    c.cycles += 23;
                    c.g[A3][0] = 0x1a0000;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x100;
                    c.g[A2][0] = 0x4;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x22f0));
                    c.g[RA][0] = sx(0x0019df98);
                    c.g[FP][0] = 0x120;
                    call_ptr(c, 0x00418ef8, 0x0019df98)?;
                    c.g[S3][0] = 0u64;
                    c.g[V0][0] = 0x10000;
                    c.g[A2][0] = c.lhu(c.g[SP][0] as u32);
                    c.g[V0][0] = c.g[V0][0] | 0x2000;
                    c.g[V1][0] = 0xb0000;
                    c.mult(0, 4, 21, 2);
                    c.mult(0, 5, 6, 30);
                    c.g[S2][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[S7][0] = 0x8;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4008));
                    c.g[S0][0] = c.g[SP][0];
                    c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                    c.g[A0][0] = sx((c.g[S5][0] as u32) << 2);
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    let t = true;
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    if t {
                        pc = 0x0019e028;
                        continue;
                    }
                    pc = 0x0019dfd8;
                    continue;
                }
                0x0019dfd8 => {
                    c.cycles += 6;
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x11c), c.g[V0][0]);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4004));

                    if (c.g[V1][0] as i64) > 0 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x0019e010;
                        continue;
                    }
                    pc = 0x0019dff0;
                    continue;
                }
                0x0019dff0 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V1][0] = 0x3;

                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x0019e010;
                        continue;
                    }
                    pc = 0x0019e000;
                    continue;
                }
                0x0019e000 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x0019e00c);
                    c.g[A1][0] = c.g[S5][0];
                    Callee {
                        addr: 0x0019d8b8,
                        f: ee::s03::BoltFile_LoadSlot,
                    }
                    .call(c, 0x0019e00c)?;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    pc = 0x0019e010;
                    continue;
                }
                0x0019e010 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < 256i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x0019e040;
                        continue;
                    }
                    pc = 0x0019e01c;
                    continue;
                }
                0x0019e01c => {
                    c.cycles += 3;
                    c.g[A2][0] = c.lhu(c.g[S0][0] as u32);
                    c.mult(0, 2, 6, 30);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    pc = 0x0019e028;
                    continue;
                }
                0x0019e028 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x11c));

                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x490));
                        pc = 0x0019e044;
                        continue;
                    }
                    pc = 0x0019e034;
                    continue;
                }
                0x0019e034 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x110));

                    if c.g[V0][0] == c.g[S7][0] {
                        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                        if back_edge_unwinds(c, 0x0019dfd8) {
                            return Ok(());
                        }
                        pc = 0x0019dfd8;
                        continue;
                    }
                    pc = 0x0019e040;
                    continue;
                }
                0x0019e040 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x490));
                    pc = 0x0019e044;
                    continue;
                }
                0x0019e044 => {
                    c.cycles += 11;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x480));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x470));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x460));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x450));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x440));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x430));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x420));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x410));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x400));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4a0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0019df20, 0x0019e070);
                    return Ok(());
                }
            }
        }
    });
}

/// `void BoltFile_PrioritizeTextures(BoltTable* self, int bank)` (0x19e070): the texture twin
/// of `BoltFile_PrioritizeModels` (type 11, sorted with `BoltFile_CompareTextureSlots`).
pub fn bolt_file_prioritize_textures(c: &mut Cpu) {
    if c.pc != 0x0019e070 {
        return ee::s03::f_0019e070__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0019e070;
        loop {
            match pc {
                0x0019e070 => {
                    c.cycles += 16;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x4a0));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x440));
                    c.g[V1][0] = c.g[SP][0];
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x430));
                    c.g[S5][0] = c.g[A1][0];
                    c.g[S6][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x490));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x480));
                    c.g[A1][0] = sx((c.g[S5][0] as u32) << 16);
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x470));
                    c.g[A0][0] = 0u64;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x460));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x450));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x420));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x410));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x400), c.g[RA][0]);
                    pc = 0x0019e0b0;
                    continue;
                }
                0x0019e0b0 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.g[A1][0] | c.g[A0][0];
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 256i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0019e0b0) {
                            return Ok(());
                        }
                        pc = 0x0019e0b0;
                        continue;
                    }
                    pc = 0x0019e0cc;
                    continue;
                }
                0x0019e0cc => {
                    c.cycles += 23;
                    c.g[A3][0] = 0x1a0000;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x100;
                    c.g[A2][0] = 0x4;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x21e8));
                    c.g[RA][0] = sx(0x0019e0e8);
                    c.g[FP][0] = 0x120;
                    call_ptr(c, 0x00418ef8, 0x0019e0e8)?;
                    c.g[S3][0] = 0u64;
                    c.g[V0][0] = 0x10000;
                    c.g[A2][0] = c.lhu(c.g[SP][0] as u32);
                    c.g[V0][0] = c.g[V0][0] | 0x2000;
                    c.g[V1][0] = 0xb0000;
                    c.mult(0, 4, 21, 2);
                    c.mult(0, 5, 6, 30);
                    c.g[S2][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[S7][0] = 0xb;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4008));
                    c.g[S0][0] = c.g[SP][0];
                    c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                    c.g[A0][0] = sx((c.g[S5][0] as u32) << 2);
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    let t = true;
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    if t {
                        pc = 0x0019e178;
                        continue;
                    }
                    pc = 0x0019e128;
                    continue;
                }
                0x0019e128 => {
                    c.cycles += 6;
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x11c), c.g[V0][0]);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4004));

                    if (c.g[V1][0] as i64) > 0 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x0019e160;
                        continue;
                    }
                    pc = 0x0019e140;
                    continue;
                }
                0x0019e140 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V1][0] = 0x3;

                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        pc = 0x0019e160;
                        continue;
                    }
                    pc = 0x0019e150;
                    continue;
                }
                0x0019e150 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x0019e15c);
                    c.g[A1][0] = c.g[S5][0];
                    Callee {
                        addr: 0x0019d8b8,
                        f: ee::s03::BoltFile_LoadSlot,
                    }
                    .call(c, 0x0019e15c)?;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    pc = 0x0019e160;
                    continue;
                }
                0x0019e160 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < 256i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x0019e190;
                        continue;
                    }
                    pc = 0x0019e16c;
                    continue;
                }
                0x0019e16c => {
                    c.cycles += 3;
                    c.g[A2][0] = c.lhu(c.g[S0][0] as u32);
                    c.mult(0, 2, 6, 30);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    pc = 0x0019e178;
                    continue;
                }
                0x0019e178 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x11c));

                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x490));
                        pc = 0x0019e194;
                        continue;
                    }
                    pc = 0x0019e184;
                    continue;
                }
                0x0019e184 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x110));

                    if c.g[V0][0] == c.g[S7][0] {
                        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                        if back_edge_unwinds(c, 0x0019e128) {
                            return Ok(());
                        }
                        pc = 0x0019e128;
                        continue;
                    }
                    pc = 0x0019e190;
                    continue;
                }
                0x0019e190 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x490));
                    pc = 0x0019e194;
                    continue;
                }
                0x0019e194 => {
                    c.cycles += 11;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x480));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x470));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x460));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x450));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x440));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x430));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x420));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x410));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x400));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4a0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0019e070, 0x0019e1c0);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
/// `FEUICustom_CreateState` (0x1a0830): the jump-table switch over the front-end flow state
/// number (`*(a1 + 0xc)`, 60 states) that allocates and constructs the state object.
/// Kept as a faithful port of the generated body, not installed: its 60 arms reach about
/// 50 real constructors (half of them unported, no override slot to fake), which cannot be
/// bounded on test data.
#[allow(dead_code)]
pub fn feui_custom_create_state(c: &mut Cpu) {
    if c.pc != 0x001a0830 {
        return ee::s03::f_001a0830__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x001a0830;
        loop {
            match pc {
                0x001a0830 => {
                    c.cycles += 14;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S0][0] = c.g[A1][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = c.g[A2][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = (c.g[V1][0] < 0x3cu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S2][0] = c.g[A0][0];
                    if t {
                        pc = 0x001a1694;
                        continue;
                    }
                    pc = 0x001a0868;
                    continue;
                }
                0x001a0868 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x460000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf30));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001a0884 => {
                    c.cycles += 2;
                    let t = c.g[S1][0] == 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x001a089c;
                        continue;
                    }
                    pc = 0x001a088c;
                    continue;
                }
                0x001a088c => {
                    c.cycles += 2;
                    let t = c.g[S1][0] == c.g[V0][0];
                    c.g[A1][0] = 0x2;
                    if t {
                        pc = 0x001a08c8;
                        continue;
                    }
                    pc = 0x001a0894;
                    continue;
                }
                0x001a0894 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x001a08fc;
                        continue;
                    }
                    pc = 0x001a089c;
                    continue;
                }
                0x001a089c => {
                    c.cycles += 3;
                    c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    let t = c.g[A3][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x001a08f8;
                        continue;
                    }
                    pc = 0x001a08a8;
                    continue;
                }
                0x001a08a8 => {
                    c.cycles += 8;
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001a08c0);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001a08c0)?;
                    let t = true;
                    c.g[A1][0] = 0x2;
                    if t {
                        pc = 0x001a08fc;
                        continue;
                    }
                    pc = 0x001a08c8;
                    continue;
                }
                0x001a08c8 => {
                    c.cycles += 12;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x31b8));
                    c.g[A2][0] = 0x100;
                    c.g[A3][0] = 0u64;
                    c.g[A0][0] = 0x8;
                    c.g[RA][0] = sx(0x001a08e8);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x60), 0u64);
                    call_ptr(c, 0x00317d70, 0x001a08e8)?;
                    c.g[RA][0] = sx(0x001a08f0);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00232738, 0x001a08f0)?;
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        pc = 0x001a1270;
                        continue;
                    }
                    pc = 0x001a08f8;
                    continue;
                }
                0x001a08f8 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0x2;
                    pc = 0x001a08fc;
                    continue;
                }
                0x001a08fc => {
                    c.cycles += 14;
                    c.g[A0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1d90));
                    c.g[RA][0] = sx(0x001a0908);
                    c.g[A2][0] = 0u64;
                    Callee {
                        addr: 0x001a0708,
                        f: ee::s03::FEFlowState_SetMenuSelection,
                    }
                    .call(c, 0x001a0908)?;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x58;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xba8));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0920);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0920)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x001a092c);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00194a60, 0x001a092c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0934;
                    continue;
                }
                0x001a0934 => {
                    c.cycles += 14;
                    c.g[RA][0] = sx(0x001a093c);
                    c.g[A0][0] = 0x1;
                    Callee {
                        addr: 0x001a86e8,
                        f: ee::s03::FE_SetOnlineMode,
                    }
                    .call(c, 0x001a093c)?;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x90;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xbc0));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0954);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0954)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a0964);
                    c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    call_ptr(c, 0x00181090, 0x001a0964)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a096c;
                    continue;
                }
                0x001a096c => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x5));
                    c.g[V0][0] = (c.g[V1][0] < 0x2du64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x001a0a24;
                        continue;
                    }
                    pc = 0x001a097c;
                    continue;
                }
                0x001a097c => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1020));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001a0994 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001a099c);
                    c.g[A0][0] = 0u64;
                    Callee {
                        addr: 0x001a86e8,
                        f: ee::s03::FE_SetOnlineMode,
                    }
                    .call(c, 0x001a099c)?;
                    let t = true;
                    c.g[A1][0] = 0x460000;
                    if t {
                        pc = 0x001a1170;
                        continue;
                    }
                    pc = 0x001a09a4;
                    continue;
                }
                0x001a09a4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001a09ac);
                    c.g[A0][0] = 0u64;
                    Callee {
                        addr: 0x001a86e8,
                        f: ee::s03::FE_SetOnlineMode,
                    }
                    .call(c, 0x001a09ac)?;
                    let t = true;
                    c.g[A1][0] = 0x460000;
                    if t {
                        pc = 0x001a1170;
                        continue;
                    }
                    pc = 0x001a09b4;
                    continue;
                }
                0x001a09b4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001a09bc);
                    c.g[A0][0] = 0u64;
                    Callee {
                        addr: 0x001a86e8,
                        f: ee::s03::FE_SetOnlineMode,
                    }
                    .call(c, 0x001a09bc)?;
                    let t = true;
                    c.g[A1][0] = 0x460000;
                    if t {
                        pc = 0x001a1170;
                        continue;
                    }
                    pc = 0x001a09c4;
                    continue;
                }
                0x001a09c4 => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x48;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xbd8));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a09dc);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a09dc)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x001a09e8);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00195498, 0x001a09e8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a09f0;
                    continue;
                }
                0x001a09f0 => {
                    c.cycles += 13;
                    c.g[RA][0] = sx(0x001a09f8);
                    c.g[A0][0] = 0x1;
                    Callee {
                        addr: 0x001a86e8,
                        f: ee::s03::FE_SetOnlineMode,
                    }
                    .call(c, 0x001a09f8)?;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x91c;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xbf0));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0a10);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0a10)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x001a0a1c);
                    c.g[A0][0] = c.g[V0][0];
                    Callee {
                        addr: 0x001b3f78,
                        f: ee::s03::cFEStateNetConfig_ctor,
                    }
                    .call(c, 0x001a0a1c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0a24;
                    continue;
                }
                0x001a0a24 => {
                    c.cycles += 7;
                    c.g[V0][0] = (0u64 < c.g[S1][0]) as u64;
                    c.g[A1][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xbc0));
                    c.g[A0][0] = 0x90;
                    c.g[A2][0] = 0u64;
                    let t = true;
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x171c), c.g[V0][0]);
                    if t {
                        pc = 0x001a117c;
                        continue;
                    }
                    pc = 0x001a0a40;
                    continue;
                }
                0x001a0a40 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = 0x100;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x31a0));
                    c.g[A3][0] = 0u64;
                    let t = true;
                    c.g[A0][0] = 0x8;
                    if t {
                        pc = 0x001a125c;
                        continue;
                    }
                    pc = 0x001a0a58;
                    continue;
                }
                0x001a0a58 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x460000;
                    if t {
                        pc = 0x001a0ad4;
                        continue;
                    }
                    pc = 0x001a0a68;
                    continue;
                }
                0x001a0a68 => {
                    c.cycles += 16;
                    c.g[RA][0] = sx(0x001a0a70);
                    c.g[S5][0] = 0x1;
                    call_ptr(c, 0x0014dc80, 0x001a0a70)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a0a7c);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x001a0a7c)?;
                    c.g[RA][0] = sx(0x001a0a84);
                    c.g[S1][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x001a0a84)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a0a90);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0014dd58, 0x001a0a90)?;
                    c.g[S4][0] = c.g[V0][0];
                    c.g[V0][0] = 0x530000;
                    c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[V1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x49));
                    let t = c.g[V1][0] == c.g[S5][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                    if t {
                        pc = 0x001a0ad0;
                        continue;
                    }
                    pc = 0x001a0aa8;
                    continue;
                }
                0x001a0aa8 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x001a0ac0;
                        continue;
                    }
                    pc = 0x001a0ab0;
                    continue;
                }
                0x001a0ab0 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x001a0b00;
                        continue;
                    }
                    pc = 0x001a0ab8;
                    continue;
                }
                0x001a0ab8 => {
                    c.cycles += 2;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0ac0;
                    continue;
                }
                0x001a0ac0 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x001a0b80;
                        continue;
                    }
                    pc = 0x001a0ac8;
                    continue;
                }
                0x001a0ac8 => {
                    c.cycles += 2;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0ad0;
                    continue;
                }
                0x001a0ad0 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0x460000;
                    pc = 0x001a0ad4;
                    continue;
                }
                0x001a0ad4 => {
                    c.cycles += 11;
                    c.g[A0][0] = 0x50;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc08));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0ae8);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0ae8)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a0af8);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x001828c0, 0x001a0af8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0b00;
                    continue;
                }
                0x001a0b00 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001a0b0c);
                    c.g[A1][0] = 0x4;
                    call_ptr(c, 0x00144df0, 0x001a0b0c)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001a0b18);
                    c.g[A1][0] = 0xc;
                    call_ptr(c, 0x001451e8, 0x001a0b18)?;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x001a0b24);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00145c38, 0x001a0b24)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0xe;
                    if t {
                        pc = 0x001a0b38;
                        continue;
                    }
                    pc = 0x001a0b2c;
                    continue;
                }
                0x001a0b2c => {
                    c.cycles += 3;
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), c.g[S5][0]);
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                    if t {
                        pc = 0x001a0b4c;
                        continue;
                    }
                    pc = 0x001a0b38;
                    continue;
                }
                0x001a0b38 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x001a0b44);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00146d98, 0x001a0b44)?;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), c.g[S5][0]);
                    pc = 0x001a0b4c;
                    continue;
                }
                0x001a0b4c => {
                    c.cycles += 13;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001a0b60);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001a0b60)?;
                    c.g[V0][0] = 0x1;
                    c.g[A1][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x31a0));
                    c.g[A0][0] = 0x8;
                    c.g[A2][0] = 0x100;
                    c.g[A3][0] = 0u64;
                    let t = true;
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x1714), c.g[V0][0]);
                    if t {
                        pc = 0x001a125c;
                        continue;
                    }
                    pc = 0x001a0b80;
                    continue;
                }
                0x001a0b80 => {
                    c.cycles += 12;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x50;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc08));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0b98);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0b98)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a0ba8);
                    c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    call_ptr(c, 0x001828c0, 0x001a0ba8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0bb0;
                    continue;
                }
                0x001a0bb0 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < 0x5u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x001a0dec;
                        continue;
                    }
                    pc = 0x001a0bbc;
                    continue;
                }
                0x001a0bbc => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10e0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001a0bd4 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x171c));
                    if t {
                        pc = 0x001a0c10;
                        continue;
                    }
                    pc = 0x001a0be4;
                    continue;
                }
                0x001a0be4 => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x6d0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd8));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0bfc);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0bfc)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x001a0c08);
                    c.g[A0][0] = c.g[V0][0];
                    Callee {
                        addr: 0x001bfbb8,
                        f: ee::s03::cFEStateOnlineMainMenu_ctor,
                    }
                    .call(c, 0x001a0c08)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0c10;
                    continue;
                }
                0x001a0c10 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x001a0c24;
                        continue;
                    }
                    pc = 0x001a0c18;
                    continue;
                }
                0x001a0c18 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A1][0] = 0x460000;
                    if t {
                        pc = 0x001a0ce8;
                        continue;
                    }
                    pc = 0x001a0c24;
                    continue;
                }
                0x001a0c24 => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001a0c2c);
                    call_ptr(c, 0x0014dc80, 0x001a0c2c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a0c38);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x001a0c38)?;
                    c.g[V1][0] = 0x530000;
                    c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
                    let t = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                    if t {
                        pc = 0x001a0dec;
                        continue;
                    }
                    pc = 0x001a0c48;
                    continue;
                }
                0x001a0c48 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x001a0c64;
                        continue;
                    }
                    pc = 0x001a0c50;
                    continue;
                }
                0x001a0c50 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x7;
                    if t {
                        pc = 0x001a0ca4;
                        continue;
                    }
                    pc = 0x001a0c5c;
                    continue;
                }
                0x001a0c5c => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x001a0df0;
                        continue;
                    }
                    pc = 0x001a0c64;
                    continue;
                }
                0x001a0c64 => {
                    c.cycles += 16;
                    c.g[A1][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc38));
                    c.g[A0][0] = 0x33c;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0xb4c), c.g[V0][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0xb5c), 0u64);
                    c.g[RA][0] = sx(0x001a0c88);
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0xb58), 0u64);
                    call_ptr(c, 0x00317d70, 0x001a0c88)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a0c98);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x00186610, 0x001a0c98)?;
                    c.g[V1][0] = c.g[V0][0];
                    let t = true;
                    c.sb((c.g[V1][0] as u32).wrapping_add(0x44), 0u64);
                    if t {
                        pc = 0x001a1698;
                        continue;
                    }
                    pc = 0x001a0ca4;
                    continue;
                }
                0x001a0ca4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x171c));
                    if t {
                        pc = 0x001a0ce0;
                        continue;
                    }
                    pc = 0x001a0cb0;
                    continue;
                }
                0x001a0cb0 => {
                    c.cycles += 12;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x90;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xbc0));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0cc8);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0cc8)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a0cd8);
                    c.g[A2][0] = 0x1;
                    call_ptr(c, 0x00181090, 0x001a0cd8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0ce0;
                    continue;
                }
                0x001a0ce0 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x460000;
                    if t {
                        pc = 0x001a0d10;
                        continue;
                    }
                    pc = 0x001a0ce8;
                    continue;
                }
                0x001a0ce8 => {
                    c.cycles += 10;
                    c.g[A0][0] = 0x48;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc20));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0cfc);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0cfc)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x001a0d08);
                    c.g[A0][0] = c.g[V0][0];
                    Callee {
                        addr: 0x001a3218,
                        f: ee::s03::cFEStateMPCircuit_ctor,
                    }
                    .call(c, 0x001a0d08)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0d10;
                    continue;
                }
                0x001a0d10 => {
                    c.cycles += 16;
                    c.g[V0][0] = 0x1;
                    c.g[A1][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc38));
                    c.g[A0][0] = 0x33c;
                    c.g[A2][0] = 0u64;
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0xb4c), c.g[V0][0]);
                    c.g[A3][0] = 0u64;
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0xb5c), 0u64);
                    c.g[RA][0] = sx(0x001a0d38);
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0xb58), 0u64);
                    call_ptr(c, 0x00317d70, 0x001a0d38)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a0d48);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x00186610, 0x001a0d48)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0d50;
                    continue;
                }
                0x001a0d50 => {
                    c.cycles += 14;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0xaf0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc50));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0d68);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0d68)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0d80);
                    c.g[T0][0] = 0u64;
                    Callee {
                        addr: 0x00198db8,
                        f: ee::s03::cFEStateCharEquip_ctor,
                    }
                    .call(c, 0x001a0d80)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0d88;
                    continue;
                }
                0x001a0d88 => {
                    c.cycles += 12;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x54;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc68));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0da0);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0da0)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a0db0);
                    c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    call_ptr(c, 0x00183238, 0x001a0db0)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0db8;
                    continue;
                }
                0x001a0db8 => {
                    c.cycles += 13;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x80;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x650));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0dd0);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0dd0)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    c.g[RA][0] = sx(0x001a0de4);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00195ff0, 0x001a0de4)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0dec;
                    continue;
                }
                0x001a0dec => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x7;
                    pc = 0x001a0df0;
                    continue;
                }
                0x001a0df0 => {
                    c.cycles += 2;
                    let t = c.g[S1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 8i64) as u64;
                    if t {
                        pc = 0x001a143c;
                        continue;
                    }
                    pc = 0x001a0df8;
                    continue;
                }
                0x001a0df8 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x001a0e10;
                        continue;
                    }
                    pc = 0x001a0e00;
                    continue;
                }
                0x001a0e00 => {
                    c.cycles += 2;
                    let t = c.g[S1][0] == c.g[V0][0];
                    c.g[A1][0] = 0x460000;
                    if t {
                        pc = 0x001a13d8;
                        continue;
                    }
                    pc = 0x001a0e08;
                    continue;
                }
                0x001a0e08 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x210;
                    if t {
                        pc = 0x001a0e24;
                        continue;
                    }
                    pc = 0x001a0e10;
                    continue;
                }
                0x001a0e10 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x9;
                    let t = c.g[S1][0] == c.g[V0][0];
                    c.g[A1][0] = 0x460000;
                    if t {
                        pc = 0x001a149c;
                        continue;
                    }
                    pc = 0x001a0e1c;
                    continue;
                }
                0x001a0e1c => {
                    c.cycles += 2;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x210;
                    pc = 0x001a0e24;
                    continue;
                }
                0x001a0e24 => {
                    c.cycles += 11;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x470));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0e34);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0e34)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    c.g[RA][0] = sx(0x001a0e48);
                    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1718));
                    call_ptr(c, 0x00196b90, 0x001a0e48)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0e50;
                    continue;
                }
                0x001a0e50 => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001a0e58);
                    call_ptr(c, 0x0014dc80, 0x001a0e58)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a0e64);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x001a0e64)?;
                    c.g[V1][0] = 0x530000;
                    c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
                    let t = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                    if t {
                        pc = 0x001a0eb4;
                        continue;
                    }
                    pc = 0x001a0e74;
                    continue;
                }
                0x001a0e74 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x460000;
                    if t {
                        pc = 0x001a0eb8;
                        continue;
                    }
                    pc = 0x001a0e7c;
                    continue;
                }
                0x001a0e7c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = 0x9c;
                    if t {
                        pc = 0x001a0ebc;
                        continue;
                    }
                    pc = 0x001a0e88;
                    continue;
                }
                0x001a0e88 => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x73c;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xcc8));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0ea0);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0ea0)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x001a0eac);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00192d00, 0x001a0eac)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0eb4;
                    continue;
                }
                0x001a0eb4 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0x460000;
                    pc = 0x001a0eb8;
                    continue;
                }
                0x001a0eb8 => {
                    c.cycles += 1;
                    c.g[A0][0] = 0x9c;
                    pc = 0x001a0ebc;
                    continue;
                }
                0x001a0ebc => {
                    c.cycles += 9;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xce0));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0ecc);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0ecc)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x001a0ed8);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00191360, 0x001a0ed8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0ee0;
                    continue;
                }
                0x001a0ee0 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1c));
                    c.g[V0][0] = (c.g[V1][0] < 0x9u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x001a1694;
                        continue;
                    }
                    pc = 0x001a0ef0;
                    continue;
                }
                0x001a0ef0 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1100));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001a0f08 => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x4c;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xcf8));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0f20);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0f20)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x001a0f2c);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00188ee8, 0x001a0f2c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0f34;
                    continue;
                }
                0x001a0f34 => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x48;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xd10));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0f4c);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0f4c)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x001a0f58);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0018a258, 0x001a0f58)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0f60;
                    continue;
                }
                0x001a0f60 => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x48;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xd28));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0f78);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0f78)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x001a0f84);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0018c0e8, 0x001a0f84)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0f8c;
                    continue;
                }
                0x001a0f8c => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x54;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xd40));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0fa4);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0fa4)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x001a0fb0);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0018c658, 0x001a0fb0)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0fb8;
                    continue;
                }
                0x001a0fb8 => {
                    c.cycles += 13;
                    c.g[V0][0] = 0x18;
                    c.g[A1][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xd60));
                    c.g[A0][0] = 0x4c;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a0fd8);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a0fd8)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x001a0fe4);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0018d240, 0x001a0fe4)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a0fec;
                    continue;
                }
                0x001a0fec => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x68;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xd78));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a1004);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a1004)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x001a1010);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00185a18, 0x001a1010)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a1018;
                    continue;
                }
                0x001a1018 => {
                    c.cycles += 13;
                    c.g[V0][0] = 0x22;
                    c.g[A1][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xd90));
                    c.g[A0][0] = 0x23c;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x001a1038);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    call_ptr(c, 0x00317d70, 0x001a1038)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = 0x2;
                    let t = true;
                    c.g[A3][0] = 0x1;
                    if t {
                        pc = 0x001a10fc;
                        continue;
                    }
                    pc = 0x001a104c;
                    continue;
                }
                0x001a104c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = c.g[S1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                    if t {
                        pc = 0x001a10ac;
                        continue;
                    }
                    pc = 0x001a1058;
                    continue;
                }
                0x001a1058 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x001a1070;
                        continue;
                    }
                    pc = 0x001a1060;
                    continue;
                }
                0x001a1060 => {
                    c.cycles += 2;
                    let t = c.g[S1][0] == 0u64;
                    c.g[A1][0] = 0x460000;
                    if t {
                        pc = 0x001a1080;
                        continue;
                    }
                    pc = 0x001a1068;
                    continue;
                }
                0x001a1068 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x280;
                    if t {
                        pc = 0x001a110c;
                        continue;
                    }
                    pc = 0x001a1070;
                    continue;
                }
                0x001a1070 => {
                    c.cycles += 2;
                    let t = c.g[S1][0] == c.g[V0][0];
                    c.g[A1][0] = 0x460000;
                    if t {
                        pc = 0x001a10d4;
                        continue;
                    }
                    pc = 0x001a1078;
                    continue;
                }
                0x001a1078 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x280;
                    if t {
                        pc = 0x001a110c;
                        continue;
                    }
                    pc = 0x001a1080;
                    continue;
                }
                0x001a1080 => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x23c;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xd90));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a1098);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a1098)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = 0x2;
                    let t = true;
                    c.g[A3][0] = 0x1;
                    if t {
                        pc = 0x001a10fc;
                        continue;
                    }
                    pc = 0x001a10ac;
                    continue;
                }
                0x001a10ac => {
                    c.cycles += 10;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x23c;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xd90));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a10c4);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a10c4)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    let t = true;
                    c.g[A2][0] = 0x1;
                    if t {
                        pc = 0x001a10f8;
                        continue;
                    }
                    pc = 0x001a10d4;
                    continue;
                }
                0x001a10d4 => {
                    c.cycles += 9;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x23c;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xd90));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a10ec);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a10ec)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = 0x2;
                    pc = 0x001a10f8;
                    continue;
                }
                0x001a10f8 => {
                    c.cycles += 1;
                    c.g[A3][0] = 0u64;
                    pc = 0x001a10fc;
                    continue;
                }
                0x001a10fc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001a1104);
                    c.g[T0][0] = 0u64;
                    call_ptr(c, 0x0018d7f0, 0x001a1104)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a110c;
                    continue;
                }
                0x001a110c => {
                    c.cycles += 12;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2958));
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x001a1120);
                    c.g[S0][0] = c.g[S1][0] & 0xff;
                    call_ptr(c, 0x00317d70, 0x001a1120)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A2][0] = sx(((c.g[S1][0] as u32) as i32 >> 8) as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a1134);
                    c.g[A3][0] = c.g[S0][0];
                    call_ptr(c, 0x0018eca0, 0x001a1134)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a113c;
                    continue;
                }
                0x001a113c => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001a1144);
                    call_ptr(c, 0x0014dc80, 0x001a1144)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a1150);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x001a1150)?;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));

                    if c.g[V0][0] == 0u64 {
                        c.g[A1][0] = 0x460000;
                        pc = 0x001a1170;
                        continue;
                    }
                    pc = 0x001a1160;
                    continue;
                }
                0x001a1160 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                    if t {
                        pc = 0x001a1694;
                        continue;
                    }
                    pc = 0x001a1168;
                    continue;
                }
                0x001a1168 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x460000;
                    if t {
                        pc = 0x001a1694;
                        continue;
                    }
                    pc = 0x001a1170;
                    continue;
                }
                0x001a1170 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x90;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xbc0));
                    c.g[A2][0] = 0u64;
                    pc = 0x001a117c;
                    continue;
                }
                0x001a117c => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x001a1184);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a1184)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a1194);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x00181090, 0x001a1194)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a119c;
                    continue;
                }
                0x001a119c => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < 0x8u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x001a1694;
                        continue;
                    }
                    pc = 0x001a11a8;
                    continue;
                }
                0x001a11a8 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1130));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001a11c0 => {
                    c.cycles += 34;
                    c.g[RA][0] = sx(0x001a11c8);
                    call_ptr(c, 0x0014dc80, 0x001a11c8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a11d4);
                    c.g[A1][0] = 0x4;
                    call_ptr(c, 0x0014dd58, 0x001a11d4)?;
                    c.g[RA][0] = sx(0x001a11dc);
                    call_ptr(c, 0x0014dc80, 0x001a11dc)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a11e8);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x001a11e8)?;
                    c.g[RA][0] = sx(0x001a11f0);
                    c.g[S0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x001a11f0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a11fc);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0014dd58, 0x001a11fc)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001a120c);
                    c.g[A1][0] = 0x4;
                    call_ptr(c, 0x00144df0, 0x001a120c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001a1218);
                    c.g[A1][0] = 0xc;
                    call_ptr(c, 0x001451e8, 0x001a1218)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x001a1224);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00146d98, 0x001a1224)?;
                    c.g[V1][0] = 0x530000;
                    c.g[A0][0] = 0x1;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x5c08), c.g[V0][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001a1248);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001a1248)?;
                    pc = 0x001a1248;
                    continue;
                }
                0x001a1248 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x8;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x31a0));
                    c.g[A2][0] = 0x100;
                    c.g[A3][0] = 0u64;
                    pc = 0x001a125c;
                    continue;
                }
                0x001a125c => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x001a1264);
                    call_ptr(c, 0x00317d70, 0x001a1264)?;
                    c.g[RA][0] = sx(0x001a126c);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00232720, 0x001a126c)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    pc = 0x001a1270;
                    continue;
                }
                0x001a1270 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001a1278);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00316ef8, 0x001a1278)?;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x001a1698;
                        continue;
                    }
                    pc = 0x001a1280;
                    continue;
                }
                0x001a1280 => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0xaf0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc50));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a1298);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a1298)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    let t = true;
                    c.g[A3][0] = 0u64;
                    if t {
                        pc = 0x001a12d4;
                        continue;
                    }
                    pc = 0x001a12ac;
                    continue;
                }
                0x001a12ac => {
                    c.cycles += 10;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0xaf0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc50));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a12c4);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a12c4)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0x1;
                    pc = 0x001a12d4;
                    continue;
                }
                0x001a12d4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001a12dc);
                    c.g[T0][0] = 0x1;
                    Callee {
                        addr: 0x00198db8,
                        f: ee::s03::cFEStateCharEquip_ctor,
                    }
                    .call(c, 0x001a12dc)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a12e4;
                    continue;
                }
                0x001a12e4 => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0xb0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xdc0));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a12fc);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a12fc)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x001a1308);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x001f4728, 0x001a1308)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a1310;
                    continue;
                }
                0x001a1310 => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x4c;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xdd8));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a1328);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a1328)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x001a1334);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x001f3ff8, 0x001a1334)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a133c;
                    continue;
                }
                0x001a133c => {
                    c.cycles += 13;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x80;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x650));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a1354);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a1354)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a1368);
                    c.g[A3][0] = 0x1;
                    call_ptr(c, 0x00195ff0, 0x001a1368)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a1370;
                    continue;
                }
                0x001a1370 => {
                    c.cycles += 16;
                    c.g[RA][0] = sx(0x001a1378);
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x1718), 0u64);
                    call_ptr(c, 0x0014dc80, 0x001a1378)?;
                    c.g[A1][0] = 0x4;
                    c.g[RA][0] = sx(0x001a1384);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dd58, 0x001a1384)?;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x4c;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1d70));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a139c);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a139c)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x001a13a8);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00194738, 0x001a13a8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a13b0;
                    continue;
                }
                0x001a13b0 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x9));
                    c.g[V0][0] = (c.g[V1][0] < 0x7u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x001a1694;
                        continue;
                    }
                    pc = 0x001a13c0;
                    continue;
                }
                0x001a13c0 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1150));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001a13d8 => {
                    c.cycles += 13;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x50;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc80));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a13f0);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a13f0)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    c.g[RA][0] = sx(0x001a1404);
                    c.g[A3][0] = 0x1;
                    call_ptr(c, 0x001d4bc8, 0x001a1404)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a140c;
                    continue;
                }
                0x001a140c => {
                    c.cycles += 12;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0xc8;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xdf8));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a1424);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a1424)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a1434);
                    c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    call_ptr(c, 0x001d2678, 0x001a1434)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a143c;
                    continue;
                }
                0x001a143c => {
                    c.cycles += 12;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x60;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc98));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a1454);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a1454)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a1464);
                    c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    call_ptr(c, 0x00183a98, 0x001a1464)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a146c;
                    continue;
                }
                0x001a146c => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x78;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe10));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a1484);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a1484)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x001a1490);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x001f54b0, 0x001a1490)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a1498;
                    continue;
                }
                0x001a1498 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0x460000;
                    pc = 0x001a149c;
                    continue;
                }
                0x001a149c => {
                    c.cycles += 11;
                    c.g[A0][0] = 0x50;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xcb0));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a14b0);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a14b0)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a14c0);
                    c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    call_ptr(c, 0x00190858, 0x001a14c0)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a14c8;
                    continue;
                }
                0x001a14c8 => {
                    c.cycles += 13;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0xe4;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe28));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a14e0);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a14e0)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    c.g[RA][0] = sx(0x001a14f4);
                    c.g[A3][0] = c.g[S1][0];
                    call_ptr(c, 0x001d3780, 0x001a14f4)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a14fc;
                    continue;
                }
                0x001a14fc => {
                    c.cycles += 14;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0xa0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe40));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a1514);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a1514)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    c.g[T0][0] = c.g[S1][0] & 0xffff;
                    c.g[RA][0] = sx(0x001a152c);
                    c.g[A3][0] = sx(((c.g[S1][0] as u32) as i32 >> 16) as u32);
                    call_ptr(c, 0x001d4268, 0x001a152c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a1534;
                    continue;
                }
                0x001a1534 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x3));
                    c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x001a1694;
                        continue;
                    }
                    pc = 0x001a1544;
                    continue;
                }
                0x001a1544 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1170));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001a155c => {
                    c.cycles += 13;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0xad4;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe58));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a1574);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a1574)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    c.g[RA][0] = sx(0x001a1588);
                    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1718));
                    call_ptr(c, 0x001d1128, 0x001a1588)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a1590;
                    continue;
                }
                0x001a1590 => {
                    c.cycles += 13;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0xbf8;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe78));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a15a8);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a15a8)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    c.g[RA][0] = sx(0x001a15bc);
                    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1718));
                    call_ptr(c, 0x001d12c8, 0x001a15bc)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a15c4;
                    continue;
                }
                0x001a15c4 => {
                    c.cycles += 13;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0xbb8;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xea0));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a15dc);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a15dc)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    c.g[RA][0] = sx(0x001a15f0);
                    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1718));
                    call_ptr(c, 0x001d1460, 0x001a15f0)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a15f8;
                    continue;
                }
                0x001a15f8 => {
                    c.cycles += 13;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0xa98;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xec8));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a1610);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a1610)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    c.g[RA][0] = sx(0x001a1624);
                    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1718));
                    call_ptr(c, 0x001d1790, 0x001a1624)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a162c;
                    continue;
                }
                0x001a162c => {
                    c.cycles += 13;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0xa30;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xee8));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a1644);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a1644)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    c.g[RA][0] = sx(0x001a1658);
                    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1718));
                    call_ptr(c, 0x001d15f8, 0x001a1658)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a1660;
                    continue;
                }
                0x001a1660 => {
                    c.cycles += 13;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0xa98;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf08));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001a1678);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001a1678)?;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    c.g[RA][0] = sx(0x001a168c);
                    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1718));
                    call_ptr(c, 0x001d1928, 0x001a168c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001a169c;
                        continue;
                    }
                    pc = 0x001a1694;
                    continue;
                }
                0x001a1694 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x001a1698;
                    continue;
                }
                0x001a1698 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    pc = 0x001a169c;
                    continue;
                }
                0x001a169c => {
                    c.cycles += 8;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001a0830, 0x001a16bc);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use crate::game::fe::festatecharequipdetail::tests::{
        oc748_fixture, oc748_install_fakes, oc748_put_slot, oc748_slot, FX_TABLE,
    };

    /// `BoltFile_CompareTextureSlots`: every combination of a good / state-bad /
    /// type-bad / stamp-bad first and second slot, with equal, smaller and larger names.
    #[test]
    fn bolt_file_compare_texture_slots_matches() {
        check(
            ee::s03::f_0019de18__generated,
            bolt_file_compare_texture_slots,
            96,
            &|c, seed| {
                let mut r = Rng(0x0019_de18 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0019_de18;
                c.g[GP][0] = sx(GAME_GP);
                let table = DATA + 0x0020_0000;
                c.bus.write32(GAME_GP - 0x1784, table);
                let k = seed % 3;
                let j = 3 + seed % 3;
                let row = (seed / 7) % 2;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x40);
                c.bus.write32(DATA, row << 16 | k);
                c.bus.write32(DATA + 0x40, r.next_u32() & 0xffff_0000 | j);
                let a = table + row * 0x12000 + k * 0x120;
                let b = table + row * 0x12000 + j * 0x120;
                for s in [a, b] {
                    for off in (0..0x120).step_by(4) {
                        c.bus.write32(s + off, 0);
                    }
                }
                // 0: good, 1: bad state, 2: bad type, 3: stamp set.
                let put = |c: &mut Cpu, s: u32, variant: u32| {
                    c.bus.write32(s, if variant == 1 { 2 } else { 3 });
                    c.bus.write32(s + 0x110, if variant == 2 { 8 } else { 11 });
                    c.bus.write32(s + 0x11c, (variant == 3) as u32);
                };
                put(c, a, seed % 4);
                put(c, b, (seed / 4) % 4);
                let (na, nb) = match (seed / 16) % 3 {
                    0 => (*b"ABC\0", *b"abc\0"),
                    1 => (*b"ABC\0", *b"ABD\0"),
                    _ => (*b"ABE\0", *b"abd\0"),
                };
                c.bus.write32(a + 4, u32::from_le_bytes(na));
                c.bus.write32(b + 4, u32::from_le_bytes(nb));
            },
        );
    }

    /// Fixture of the two prioritize functions: the bank table at `FX_TABLE` (also
    /// the comparator's table at `gp - 0x1784`), a few slots of `ty` with no stamp
    /// (state 3 or not) among zero slots, the pending counter and the bank counter.
    fn oc801_prioritize_fixture(c: &mut Cpu, seed: u32, salt: u64, ty: u32, entry: u32) {
        let mut r = oc748_fixture(c, seed, salt, 8);
        let gp = c.g[GP][0] as u32;
        c.bus.write32(gp - 0x1784, FX_TABLE);
        let bank = seed % 10;
        let n = 1 + seed % 5;
        for _i in 0..n {
            let idx = r.next_u32() % 256;
            let state = if r.next_u32() % 3 == 0 { 2 } else { 3 };
            oc748_put_slot(c, &mut r, bank, idx, state, 0);
            let s = oc748_slot(bank, idx);
            // Mostly the wanted type; the odd one stops the walk.
            c.bus
                .write32(s + 0x110, if r.next_u32() % 5 == 0 { ty ^ 1 } else { ty });
        }
        // The loop-bound counters: async loads pending (> 0 skips the loads).
        c.bus
            .write32(FX_TABLE + 0xb4004, if seed % 3 == 0 { 1 } else { 0 });
        c.bus
            .write32(FX_TABLE + 0xb4008 + bank * 4, r.next_u32() & 0xff);
        c.g[A0][0] = sx(FX_TABLE);
        c.g[A1][0] = bank as u64;
        c.pc = entry;
    }

    /// `BoltFile_PrioritizeModels`: the real `qsort` and model comparator, slots of
    /// type 8 with and without a pending load, the real `BoltFile_LoadSlot` (its callees
    /// faked). The sort order decides how many slots the walk reaches.
    #[test]
    fn bolt_file_prioritize_models_matches() {
        oc748_install_fakes();
        check(
            ee::s03::f_0019df20__generated,
            bolt_file_prioritize_models,
            120,
            &|c, seed| oc801_prioritize_fixture(c, seed, 0x0019_df20, 8, 0x0019_df20),
        );
    }

    /// `BoltFile_PrioritizeTextures`: as the models, with type 11 and the texture
    /// comparator.
    #[test]
    fn bolt_file_prioritize_textures_matches() {
        oc748_install_fakes();
        check(
            ee::s03::f_0019e070__generated,
            bolt_file_prioritize_textures,
            120,
            &|c, seed| oc801_prioritize_fixture(c, seed, 0x0019_e070, 11, 0x0019_e070),
        );
    }
}
