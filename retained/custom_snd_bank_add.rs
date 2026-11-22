//! Native SND_BankAdd (0x3b6098), retained uninstalled.
//!
//! Generated EE controls registers, memory, cycles and continuation order.
//! No test, build, replay or game execution accompanies this translation.

use crate::call::{call_ptr, guest, Frame};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x003b_6098;
const END: u32 = 0x003b_62fc;
const FRAME_BYTES: u32 = 0x70;
const PATCH_COUNT: u32 = 6;
const HEADER_BYTES: u32 = 8;
const SPU_BYTES: u32 = 0xc;
const IOP_BYTES: u32 = 0x10;
const PATCH_TABLE: u32 = 0x14;
const FIND_SLOT: u32 = 0x003c_6500;
const ALLOCATE: u32 = 0x003c_4b70;
const UPLOAD: u32 = 0x003c_42d8;
const UPLOAD_WAIT: u32 = 0x003c_42f8;
const RELOCATE_PATCH: u32 = 0x003b_6788;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `int SND_BankAdd(int* bank_id, BnkHeader* header)`.
/// Reserve/lock a bank slot; upload optional IOP and SPU regions in 0x1000-byte
/// chunks, then relocate their patches through the counted self-relative
/// table at +0x14. Each pass shares one first-address scratch word at SP+0.
/// Returns -6 on allocation failure, 7 with either uploaded region, else 8.
/// All native call returns and upload/patch loop heads accept re-entry.
pub fn snd_bank_add(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: FRAME_BYTES };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 26;
                    Frame::open(c, FRAME_BYTES);
                    c.g[V0][0] = 0x003b_0000;
                    frame.save(c, S0, 0x60);
                    c.g[V0][0] = sx(at(c, V0, 0x6300));
                    c.g[S0][0] = 0x0051_0000;
                    frame.save(c, S1, 0x50);
                    frame.save(c, S2, 0x40);
                    c.g[S0][0] = sx(r(c, S0).wrapping_sub(0x5718));
                    frame.save(c, S4, 0x20);
                    c.g[S2][0] = c.g[A1][0];
                    frame.save(c, S3, 0x30);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save_ra(c, 0x10);
                    c.sw(at(c, S0, 0x1e4), c.g[V0][0]);
                    call_ptr(c, FIND_SLOT, 0x003b_60d4)?;
                    pc = 0x003b_60d4;
                }
                0x003b_60d4 => {
                    c.sw(r(c, S1), c.g[V0][0]);
                    c.g[A0][0] = 1;
                    c.g[V0][0] = sx(r(c, V0) << 4);
                    c.g[V1][0] = c.lw(at(c, S0, 0x1f4));
                    c.g[S4][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                    c.sb(at(c, S4, 0xc), c.g[A0][0]);
                    c.sw(r(c, S4), c.g[S2][0]);
                    c.sw(at(c, S4, 4), 0);
                    c.g[A1][0] = c.lw(at(c, S2, IOP_BYTES));
                    let no_iop_region = c.g[A1][0] == 0;
                    c.g[A1][0] = sx(at(c, A1, 0x40));
                    if no_iop_region {
                        pc = 0x003b_61cc;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = 0x100;
                        call_ptr(c, ALLOCATE, 0x003b_6108)?;
                        pc = 0x003b_6108;
                    }
                }
                0x003b_6108 => {
                    let allocation_failed = c.g[V0][0] == 0;
                    c.sw(at(c, S4, 4), c.g[V0][0]);
                    if allocation_failed {
                        pc = 0x003b_61ec;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(at(c, S2, IOP_BYTES));
                    let empty = (c.g[V1][0] as i64) <= 0;
                    c.g[S0][0] = 0;
                    if empty {
                        pc = 0x003b_617c;
                    } else {
                        c.cycles += 3;
                        c.g[S3][0] = 0x1000;
                        c.g[A0][0] = sx(r(c, V1).wrapping_sub(r(c, S0)));
                        pc = 0x003b_6128;
                    }
                }
                0x003b_6128 => {
                    c.cycles += 12;
                    c.g[A1][0] = c.lw(at(c, S2, HEADER_BYTES));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 4097) as u64;
                    c.g[A2][0] = c.lw(at(c, S4, 4));
                    c.g[A3][0] = c.g[S3][0];
                    c.g[A1][0] = sx(r(c, S2).wrapping_add(r(c, A1)));
                    if c.g[V0][0] != 0 { c.g[A3][0] = c.g[A0][0]; }
                    c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, S0)));
                    c.g[A2][0] = sx(r(c, A2).wrapping_add(r(c, S0)));
                    c.g[A0][0] = 0x100;
                    c.g[S1][0] = sx(at(c, S0, 0x1000));
                    call_ptr(c, UPLOAD, 0x003b_6154)?;
                    pc = 0x003b_6154;
                }
                0x003b_6154 => {
                    c.g[S0][0] = c.g[V0][0];
                    pc = 0x003b_6158;
                }
                0x003b_6158 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, UPLOAD_WAIT, 0x003b_6160)?;
                    pc = 0x003b_6160;
                }
                0x003b_6160 => {
                    if c.g[V0][0] == 0 {
                        if back_edge_unwinds(c, 0x003b_6158) { return Ok(()) }
                        pc = 0x003b_6158;
                    } else {
                        c.cycles += 5;
                        c.g[V1][0] = c.lw(at(c, S2, IOP_BYTES));
                        c.g[S0][0] = c.g[S1][0];
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[A0][0] = sx(r(c, V1).wrapping_sub(r(c, S0)));
                            if back_edge_unwinds(c, 0x003b_6128) { return Ok(()) }
                            pc = 0x003b_6128;
                        } else { pc = 0x003b_617c; }
                    }
                }
                0x003b_617c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu(at(c, S2, PATCH_COUNT));
                    c.g[S0][0] = 0;
                    c.sw(r(c, SP), 0);
                    if c.g[V0][0] == 0 {
                        pc = 0x003b_61cc;
                    } else {
                        c.cycles += 3;
                        c.g[S1][0] = sx(at(c, S2, PATCH_TABLE));
                        c.g[S3][0] = c.g[S1][0];
                        pc = 0x003b_6198;
                    }
                }
                0x003b_6198 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r(c, S1));
                    let empty_patch = c.g[V0][0] == 0;
                    c.g[A1][0] = sx(r(c, S3).wrapping_add(r(c, V0)));
                    if !empty_patch {
                        c.cycles += 4;
                        c.g[A2][0] = c.lw(at(c, S4, 4));
                        c.g[A0][0] = 0x100;
                        c.g[A3][0] = c.g[SP][0];
                        call_ptr(c, RELOCATE_PATCH, 0x003b_61b4)?;
                    }
                    pc = 0x003b_61b4;
                }
                0x003b_61b4 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lhu(at(c, S2, PATCH_COUNT));
                    c.g[S0][0] = sx(at(c, S0, 1));
                    c.g[S3][0] = sx(at(c, S3, 4));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx(at(c, S1, 4));
                    if more {
                        if back_edge_unwinds(c, 0x003b_6198) { return Ok(()) }
                        pc = 0x003b_6198;
                    } else { pc = 0x003b_61cc; }
                }
                0x003b_61cc => {
                    c.cycles += 4;
                    c.sw(at(c, S4, 8), 0);
                    c.g[A1][0] = c.lw(at(c, S2, SPU_BYTES));
                    let no_spu_region = c.g[A1][0] == 0;
                    c.g[A1][0] = sx(at(c, A1, 0x30));
                    if no_spu_region {
                        pc = 0x003b_62bc;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = 8;
                        call_ptr(c, ALLOCATE, 0x003b_61e4)?;
                        pc = 0x003b_61e4;
                    }
                }
                0x003b_61e4 => {
                    let allocated = c.g[V0][0] != 0;
                    c.sw(at(c, S4, 8), c.g[V0][0]);
                    pc = if allocated { 0x003b_61fc } else { 0x003b_61ec };
                }
                0x003b_61ec => {
                    c.cycles += 4;
                    c.sb(at(c, S4, 0xc), 0);
                    c.g[V0][0] = 0xffff_ffff_ffff_fffa;
                    c.sw(r(c, S4), 0);
                    pc = 0x003b_62dc;
                }
                0x003b_61fc => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(at(c, S2, SPU_BYTES));
                    let empty = (c.g[V1][0] as i64) <= 0;
                    c.g[S0][0] = 0;
                    if empty {
                        pc = 0x003b_626c;
                    } else {
                        c.cycles += 2;
                        c.g[S3][0] = 0x1000;
                        c.g[A1][0] = c.lw(at(c, S2, HEADER_BYTES));
                        pc = 0x003b_6210;
                    }
                }
                0x003b_6210 => {
                    c.cycles += 14;
                    c.g[A0][0] = sx(r(c, V1).wrapping_sub(r(c, S0)));
                    c.g[V0][0] = c.lw(at(c, S2, IOP_BYTES));
                    c.g[V1][0] = ((c.g[A0][0] as i64) < 4097) as u64;
                    c.g[A1][0] = sx(r(c, S2).wrapping_add(r(c, A1)));
                    c.g[A2][0] = c.lw(at(c, S4, 8));
                    c.g[A3][0] = c.g[S3][0];
                    c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, V0)));
                    if c.g[V1][0] != 0 { c.g[A3][0] = c.g[A0][0]; }
                    c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, S0)));
                    c.g[A2][0] = sx(r(c, A2).wrapping_add(r(c, S0)));
                    c.g[A0][0] = 8;
                    c.g[S1][0] = sx(at(c, S0, 0x1000));
                    call_ptr(c, UPLOAD, 0x003b_6244)?;
                    pc = 0x003b_6244;
                }
                0x003b_6244 => {
                    c.g[S0][0] = c.g[V0][0];
                    pc = 0x003b_6248;
                }
                0x003b_6248 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, UPLOAD_WAIT, 0x003b_6250)?;
                    pc = 0x003b_6250;
                }
                0x003b_6250 => {
                    if c.g[V0][0] == 0 {
                        if back_edge_unwinds(c, 0x003b_6248) { return Ok(()) }
                        pc = 0x003b_6248;
                    } else {
                        c.cycles += 5;
                        c.g[V1][0] = c.lw(at(c, S2, SPU_BYTES));
                        c.g[S0][0] = c.g[S1][0];
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[A1][0] = c.lw(at(c, S2, HEADER_BYTES));
                            if back_edge_unwinds(c, 0x003b_6210) { return Ok(()) }
                            pc = 0x003b_6210;
                        } else { pc = 0x003b_626c; }
                    }
                }
                0x003b_626c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu(at(c, S2, PATCH_COUNT));
                    c.g[S0][0] = 0;
                    c.sw(r(c, SP), 0);
                    if c.g[V0][0] == 0 {
                        pc = 0x003b_62bc;
                    } else {
                        c.cycles += 3;
                        c.g[S1][0] = sx(at(c, S2, PATCH_TABLE));
                        c.g[S3][0] = c.g[S1][0];
                        pc = 0x003b_6288;
                    }
                }
                0x003b_6288 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r(c, S1));
                    let empty_patch = c.g[V0][0] == 0;
                    c.g[A1][0] = sx(r(c, S3).wrapping_add(r(c, V0)));
                    if !empty_patch {
                        c.cycles += 4;
                        c.g[A2][0] = c.lw(at(c, S4, 8));
                        c.g[A0][0] = 8;
                        c.g[A3][0] = c.g[SP][0];
                        call_ptr(c, RELOCATE_PATCH, 0x003b_62a4)?;
                    }
                    pc = 0x003b_62a4;
                }
                0x003b_62a4 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lhu(at(c, S2, PATCH_COUNT));
                    c.g[S0][0] = sx(at(c, S0, 1));
                    c.g[S3][0] = sx(at(c, S3, 4));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx(at(c, S1, 4));
                    if more {
                        if back_edge_unwinds(c, 0x003b_6288) { return Ok(()) }
                        pc = 0x003b_6288;
                    } else { pc = 0x003b_62bc; }
                }
                0x003b_62bc => {
                    c.cycles += 4;
                    c.sb(at(c, S4, 0xc), 0);
                    c.g[V0][0] = c.lw(at(c, S2, SPU_BYTES));
                    let had_spu = c.g[V0][0] != 0;
                    c.g[V0][0] = 7;
                    if !had_spu {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S2, IOP_BYTES));
                        let had_iop = c.g[V0][0] != 0;
                        c.g[V0][0] = 7;
                        if !had_iop {
                            c.cycles += 1;
                            c.g[V0][0] = 8;
                        }
                    }
                    pc = 0x003b_62dc;
                }
                0x003b_62dc => {
                    c.cycles += 8;
                    frame.restore(c, S0, 0x60);
                    frame.restore(c, S1, 0x50);
                    frame.restore(c, S2, 0x40);
                    frame.restore(c, S3, 0x30);
                    frame.restore(c, S4, 0x20);
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}
