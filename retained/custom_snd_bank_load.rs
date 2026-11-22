//! Native SndBank_Load (0x28b320), retained uninstalled.
//!
//! Generated EE defines register, memory, cycle and continuation behavior.
//! No automated test, replay, build or game execution accompanies this port.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0028_b320;
const END: u32 = 0x0028_b524;
const FRAME_BYTES: u32 = 0x70;
const EPILOGUE: u32 = 0x0028_b508;
const STORAGE: u32 = 0x08;
const STREAM_ID: u32 = 0x0c;
const DATA: u32 = 0x10;
const CAPACITY: u32 = 0x14;
const NAME: u32 = 0x20;
const UNLOAD: u32 = 0x0028_b528;
const UNLOAD_LINKED: u32 = 0x0028_b2d0;
const LOAD_ALLOCATED: u32 = 0x003e_1a70;
const LOAD_INTO_BUFFER: u32 = 0x003e_1bb8;
const BANK_ADD: u32 = 0x003b_6098;
const BANK_GET_SIZE: u32 = 0x003b_6528;
const BANK_MOVE: u32 = 0x003b_6548;
const ALLOC: u32 = 0x0025_2fa0;
const FREE: u32 = 0x0025_23a8;
const STRCPY: u32 = 0x0041_66f4;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void SndBank_Load(SndBank* bank, const char* file, int align)`.
/// Unload old ownership, read the complete named file into an allocated or
/// preallocated buffer, add its SND bank, then retain/move/free that buffer
/// according to SND_BankAdd's result. The name is copied to bank+20 only
/// after loading and the stream synchronization call have completed.
/// This synchronous path does not call InitLoadRequest or SndLoadQueue.
/// All native call returns and branch heads remain direct resume entries.
pub fn snd_bank_load(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: FRAME_BYTES };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 18;
                    Frame::open(c, FRAME_BYTES);
                    frame.save(c, S0, 0x60);
                    frame.save(c, S2, 0x40);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save(c, S3, 0x30);
                    c.g[S2][0] = c.g[A2][0];
                    frame.save(c, S1, 0x50);
                    c.g[S3][0] = c.g[A1][0];
                    frame.save(c, S4, 0x20);
                    frame.save_ra(c, 0x10);
                    c.g[S1][0] = 0;
                    call_ptr(c, UNLOAD, 0x0028_b350)?;
                    pc = 0x0028_b350;
                }
                0x0028_b350 => {
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, UNLOAD_LINKED, 0x0028_b358)?;
                    pc = 0x0028_b358;
                }
                0x0028_b358 => {
                    c.g[V1][0] = c.lw(at(c, S0, STORAGE));
                    c.g[V0][0] = 1;
                    let existing_buffer = c.g[V1][0] == c.g[V0][0];
                    c.sw(r(c, SP), 0);
                    pc = if existing_buffer { 0x0028_b3c4 } else { 0x0028_b368 };
                }
                0x0028_b368 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
                    let stream_or_other = c.g[V0][0] == 0;
                    c.g[V0][0] = 2;
                    pc = if stream_or_other { 0x0028_b384 } else { 0x0028_b374 };
                }
                0x0028_b374 => {
                    c.cycles += 2;
                    let allocate = c.g[V1][0] == 0;
                    c.g[A2][0] = c.g[S2][0];
                    pc = if allocate { 0x0028_b394 } else { 0x0028_b37c };
                }
                0x0028_b37c | 0x0028_b38c => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(at(c, S0, STORAGE));
                    pc = 0x0028_b3ec;
                }
                0x0028_b384 => {
                    c.cycles += 2;
                    let stream = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[S3][0];
                    pc = if stream { 0x0028_b3a0 } else { 0x0028_b38c };
                }
                0x0028_b394 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[SP][0];
                    pc = 0x0028_b3a8;
                }
                0x0028_b3a0 => {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = 0x100;
                    pc = 0x0028_b3a8;
                }
                0x0028_b3a8 => {
                    c.cycles += 5;
                    call_ptr(c, LOAD_ALLOCATED, 0x0028_b3b0)?;
                    pc = 0x0028_b3b0;
                }
                0x0028_b3b0 => {
                    c.g[S1][0] = c.g[V0][0];
                    if c.g[S1][0] == 0 {
                        frame.restore(c, S0, 0x60);
                        pc = EPILOGUE;
                    } else {
                        pc = 0x0028_b3bc;
                    }
                }
                0x0028_b3bc => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(at(c, S0, STORAGE));
                    pc = 0x0028_b3ec;
                }
                0x0028_b3c4 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw(at(c, S0, DATA));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A2][0] = c.lw(at(c, S0, CAPACITY));
                    call_ptr(c, LOAD_INTO_BUFFER, 0x0028_b3d4)?;
                    pc = 0x0028_b3d4;
                }
                0x0028_b3d4 => {
                    if c.g[V0][0] == 0 {
                        frame.restore(c, S0, 0x60);
                        pc = EPILOGUE;
                    } else {
                        pc = 0x0028_b3dc;
                    }
                }
                0x0028_b3dc => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(at(c, S0, CAPACITY));
                    c.g[S1][0] = c.lw(at(c, S0, DATA));
                    c.sw(r(c, SP), c.g[V0][0]);
                    c.g[V0][0] = c.lw(at(c, S0, STORAGE));
                    pc = 0x0028_b3ec;
                }
                0x0028_b3ec => {
                    c.cycles += 3;
                    c.g[S4][0] = 2;
                    if c.g[V0][0] != c.g[S4][0] {
                        c.g[A0][0] = sx(at(c, S0, 4));
                        pc = 0x0028_b420;
                    } else {
                        pc = 0x0028_b3f8;
                    }
                }
                0x0028_b3f8 => {
                    c.cycles += 10;
                    c.g[V1][0] = c.lw(at(c, GP, 0x524));
                    c.g[A1][0] = c.lw(at(c, S0, STREAM_ID));
                    c.g[V0][0] = c.lw(at(c, V1, 0x1d8));
                    c.g[A2][0] = c.lw(r(c, V0));
                    c.g[V1][0] = c.lw(r(c, A2));
                    c.g[A0][0] = c.lh(at(c, V1, 0x30));
                    c.g[V0][0] = c.lw(at(c, V1, 0x34));
                    let target = r(c, V0);
                    c.g[A0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
                    call_ptr(c, target, 0x0028_b41c)?;
                    pc = 0x0028_b41c;
                }
                0x0028_b41c => {
                    c.g[A0][0] = sx(at(c, S0, 4));
                    pc = 0x0028_b420;
                }
                0x0028_b420 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, BANK_ADD, 0x0028_b428)?;
                    pc = 0x0028_b428;
                }
                0x0028_b428 => {
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = 7;
                    let move_bank = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 8;
                    pc = if move_bank { 0x0028_b448 } else { 0x0028_b438 };
                }
                0x0028_b438 => {
                    c.cycles += 2;
                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw(at(c, S0, STORAGE));
                        pc = 0x0028_b4a0;
                    } else {
                        pc = 0x0028_b440;
                    }
                }
                0x0028_b440 => {
                    c.cycles += 2;
                    c.g[V1][0] = c.lw(at(c, S0, STORAGE));
                    pc = 0x0028_b4c0;
                }
                0x0028_b448 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw(at(c, S0, 4));
                    call_ptr(c, BANK_GET_SIZE, 0x0028_b450)?;
                    pc = 0x0028_b450;
                }
                0x0028_b450 => {
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw(at(c, S0, STORAGE));
                    let allocate = c.g[V0][0] != c.g[S4][0];
                    c.g[A2][0] = 0;
                    pc = if allocate { 0x0028_b474 } else { 0x0028_b460 };
                }
                0x0028_b460 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(at(c, S0, DATA));
                    c.g[A1][0] = c.lw(at(c, S0, 4));
                    call_ptr(c, BANK_MOVE, 0x0028_b46c)?;
                    pc = 0x0028_b46c;
                }
                0x0028_b46c => {
                    c.g[V0][0] = 1;
                    pc = 0x0028_b4b8;
                }
                0x0028_b474 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, ALLOC, 0x0028_b480)?;
                    pc = 0x0028_b480;
                }
                0x0028_b480 => {
                    c.g[A1][0] = c.lw(at(c, S0, 4));
                    c.g[A0][0] = c.g[V0][0];
                    c.sw(at(c, S0, DATA), c.g[V0][0]);
                    call_ptr(c, BANK_MOVE, 0x0028_b490)?;
                    pc = 0x0028_b490;
                }
                0x0028_b490 => {
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, FREE, 0x0028_b498)?;
                    pc = 0x0028_b498;
                }
                0x0028_b498 => {
                    c.sw(at(c, S0, CAPACITY), c.g[S2][0]);
                    pc = 0x0028_b4b4;
                }
                0x0028_b4a0 => {
                    c.cycles += 2;
                    let retained = c.g[V0][0] != 0;
                    c.g[V0][0] = 1;
                    pc = if retained { 0x0028_b4b8 } else { 0x0028_b4a8 };
                }
                0x0028_b4a8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r(c, SP));
                    c.sw(at(c, S0, DATA), c.g[S1][0]);
                    c.sw(at(c, S0, CAPACITY), c.g[V0][0]);
                    pc = 0x0028_b4b4;
                }
                0x0028_b4b4 => {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                    pc = 0x0028_b4b8;
                }
                0x0028_b4b8 => {
                    c.cycles += 2;
                    c.sw(r(c, S0), c.g[V0][0]);
                    pc = 0x0028_b4d8;
                }
                0x0028_b4c0 => {
                    c.cycles += 3;
                    c.g[V0][0] = 1;
                    if c.g[V1][0] == c.g[V0][0] {
                        c.sw(r(c, S0), 0);
                        pc = 0x0028_b4d8;
                    } else {
                        pc = 0x0028_b4cc;
                    }
                }
                0x0028_b4cc => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, FREE, 0x0028_b4d4)?;
                    pc = 0x0028_b4d4;
                }
                0x0028_b4d4 => {
                    c.sw(r(c, S0), 0);
                    pc = 0x0028_b4d8;
                }
                0x0028_b4d8 => {
                    c.cycles += 12;
                    c.g[V0][0] = c.lw(at(c, GP, 0x524));
                    c.g[V1][0] = c.lw(at(c, V0, 0x1d8));
                    c.g[A1][0] = c.lw(r(c, V1));
                    c.g[V0][0] = c.lw(r(c, A1));
                    c.g[A0][0] = c.lh(at(c, V0, 0x38));
                    c.g[V1][0] = c.lw(at(c, V0, 0x3c));
                    let target = r(c, V1);
                    c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
                    call_ptr(c, target, 0x0028_b4f8)?;
                    pc = 0x0028_b4f8;
                }
                0x0028_b4f8 => {
                    c.g[A0][0] = sx(at(c, S0, NAME));
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, STRCPY, 0x0028_b504)?;
                    pc = 0x0028_b504;
                }
                0x0028_b504 => {
                    frame.restore(c, S0, 0x60);
                    pc = EPILOGUE;
                }
                EPILOGUE => {
                    c.cycles += 7;
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
