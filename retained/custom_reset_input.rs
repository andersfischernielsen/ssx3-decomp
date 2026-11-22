//! Exact native RiderResetSub_HandleInput0x12f398, retained uninstalled.
//!
//! No tests, builds, replays or game runs were added or executed, following
//! the user's explicit development instruction.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0012_f398, rider_reset_sub_handle_input, "RiderResetSub_HandleInput",
)];

const RIDER: u32 = 8;
const PHASE: u32 = 4;
const AI_PATH: u32 = 0xab8;
const AI_DISTANCE: u32 = 0x4c4;
const RECOVERY_POINT: u32 = 0x490;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void RiderResetSub_HandleInput(RiderResetSub*)`.
/// Advance the fade phase towards one. Crossing one half snaps to the nearest
/// AI path, uses its local forward vector and places the rider at +0x490 with
/// 200cm clearance (1000cm in the special mode), then resets motion. At one,
/// restore the camera/trick animation and enter the ordinary air state.
/// No recovery marker or Rider_ResetAtMatrix call participates here.
pub fn rider_reset_sub_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x50 };
        let mut pc = c.pc;
        loop {
            match pc {
                0x0012_f398 => {
                    c.cycles += 18;
                    Frame::open(c, 0x50);
                    c.lwc1(2, r(c, GP).wrapping_sub(0x771c));
                    frame.save(c, S0, 0x30);
                    frame.save(c, S1, 0x20);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0x10);
                    frame.save_fpr(c, 20, 0x40);
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 4);
                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                    c.lwc1(3, at(c, S0, PHASE));
                    c.lwc1(0, at(c, V0, 0x300));
                    c.mul_s(2, 0, 2);
                    c.add_s(1, 2, 4);
                    c.c_lt_s(1, 3);
                    if c.fcc() {
                        c.cycles += 2;
                        c.sub_s(1, 3, 2);
                    } else {
                        c.sub_s(0, 4, 2);
                        c.cycles += 4;
                        c.c_lt_s(3, 0);
                        let below = c.fcc();
                        c.mov_s(1, 4);
                        if below {
                            c.cycles += 1;
                            c.add_s(1, 3, 2);
                        }
                    }
                    c.cycles += 6;
                    c.g[AT][0] = 0x3f00_0000;
                    c.mtc1(AT, 0);
                    c.c_le_s(3, 0);
                    let before_midpoint = c.fcc();
                    c.swc1(1, at(c, S0, PHASE));
                    if !before_midpoint {
                        pc = 0x0012_f4ec;
                        continue;
                    }
                    c.cycles += 4;
                    c.c_lt_s(0, 1);
                    if !c.fcc() {
                        c.lwc1(0, at(c, S0, PHASE));
                        pc = 0x0012_f4f0;
                        continue;
                    }
                    c.cycles += 16;
                    c.g[AT][0] = 0x4348_0000;
                    c.mtc1(AT, 20);
                    c.g[A0][0] = c.lw(at(c, S0, RIDER));
                    call_ptr(c, 0x0011_2d58, 0x0012_f434)?;
                    pc = 0x0012_f434;
                }
                0x0012_f434 => {
                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.lw(at(c, V0, AI_PATH));
                    c.lwc1(12, at(c, V0, AI_DISTANCE));
                    call_ptr(c, 0x0026_a8b8, 0x0012_f448)?;
                    pc = 0x0012_f448;
                }
                0x0012_f448 => {
                    c.g[S1][0] = c.g[SP][0];
                    call_ptr(c, 0x0014_dc80, 0x0012_f450)?;
                    pc = 0x0012_f450;
                }
                0x0012_f450 => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0014_dd58, 0x0012_f45c)?;
                    pc = 0x0012_f45c;
                }
                0x0012_f45c => {
                    let no_interface = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x0053_0000;
                    if no_interface {
                        c.cycles += 1;
                        c.g[A0][0] = c.lw(at(c, S0, RIDER));
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = 2;
                        c.g[V1][0] = c.lb(at(c, V0, 0x5c12));
                        if c.g[V1][0] != c.g[A0][0] {
                            c.g[A0][0] = c.lw(at(c, S0, RIDER));
                        } else {
                            c.cycles += 2;
                            c.g[AT][0] = 0x447a_0000;
                            c.mtc1(AT, 20);
                            c.cycles += 1;
                            c.g[A0][0] = c.lw(at(c, S0, RIDER));
                        }
                    }
                    c.cycles += 11;
                    c.g[A2][0] = c.g[S1][0];
                    c.mov_s(12, 20);
                    c.g[A3][0] = 0x11f;
                    c.g[A1][0] = sx(at(c, A0, RECOVERY_POINT));
                    call_ptr(c, 0x0011_d660, 0x0012_f494)?;
                    pc = 0x0012_f494;
                }
                0x0012_f494 => {
                    c.g[A0][0] = c.lw(at(c, S0, RIDER));
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0011_df18, 0x0012_f4a0)?;
                    pc = 0x0012_f4a0;
                }
                0x0012_f4a0 => {
                    c.g[V0][0] = c.lw(r(c, S0));
                    let default_reset = c.g[V0][0] == 0;
                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                    if default_reset {
                        c.cycles += 7;
                        c.g[A1][0] = 1;
                        c.g[A0][0] = c.lw(at(c, V0, 0x790));
                        call_ptr(c, 0x0011_9368, 0x0012_f4dc)?;
                        pc = 0x0012_f4dc;
                    } else {
                        c.cycles += 9;
                        c.g[A1][0] = 0;
                        c.g[A0][0] = c.lw(at(c, V0, 0x790));
                        call_ptr(c, 0x0011_9368, 0x0012_f4b8)?;
                        pc = 0x0012_f4b8;
                    }
                }
                0x0012_f4b8 | 0x0012_f4dc => {
                    let back = if pc == 0x0012_f4b8 { 0x0012_f4c8 } else { 0x0012_f4ec };
                    c.g[A0][0] = c.lw(at(c, S0, RIDER));
                    c.mov_s(12, 0);
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0010_e098, back)?;
                    pc = back;
                }
                0x0012_f4c8 => {
                    c.lwc1(0, at(c, S0, PHASE));
                    pc = 0x0012_f4f0;
                }
                0x0012_f4ec => {
                    c.cycles += 1;
                    c.lwc1(0, at(c, S0, PHASE));
                    pc = 0x0012_f4f0;
                }
                0x0012_f4f0 => {
                    c.cycles += 6;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 20);
                    c.c_eq_s(0, 20);
                    if !c.fcc() {
                        frame.restore(c, S0, 0x30);
                        pc = 0x0012_f574;
                        continue;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0012_f588, 0x0012_f510)?;
                    pc = 0x0012_f510;
                }
                0x0012_f510 => {
                    let nonlocal = c.g[V0][0] == 0;
                    c.g[V0][0] = c.lw(r(c, GP).wrapping_sub(0x848));
                    if nonlocal {
                        pc = 0x0012_f530;
                    } else {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw(at(c, S0, RIDER));
                        c.g[A0][0] = c.lw(at(c, V0, 0x84));
                        c.g[A1][0] = c.lw(at(c, V1, 0x870));
                        call_ptr(c, 0x0023_0698, 0x0012_f528)?;
                        pc = 0x0012_f528;
                    }
                }
                0x0012_f528 => {
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x002e_4578, 0x0012_f530)?;
                    pc = 0x0012_f530;
                }
                0x0012_f530 => {
                    c.cycles += 17;
                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                    c.g[A2][0] = 0;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.lw(at(c, V0, 0x790));
                    call_ptr(c, 0x0011_9e38, 0x0012_f544)?;
                    pc = 0x0012_f544;
                }
                0x0012_f544 => {
                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                    c.g[A1][0] = 2;
                    c.g[A0][0] = c.lw(at(c, V0, 0x784));
                    call_ptr(c, 0x0031_1b20, 0x0012_f554)?;
                    pc = 0x0012_f554;
                }
                0x0012_f554 => {
                    c.swc1(20, at(c, V0, 0x90));
                    c.g[A1][0] = 4;
                    c.g[A0][0] = c.lw(at(c, S0, RIDER));
                    call_ptr(c, 0x0011_fec8, 0x0012_f564)?;
                    pc = 0x0012_f564;
                }
                0x0012_f564 => {
                    c.g[A0][0] = c.lw(at(c, S0, RIDER));
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0011_fe78, 0x0012_f570)?;
                    pc = 0x0012_f570;
                }
                0x0012_f570 => {
                    frame.restore(c, S0, 0x30);
                    pc = 0x0012_f574;
                }
                0x0012_f574 => {
                    c.cycles += 5;
                    frame.restore(c, S1, 0x20);
                    frame.restore_ra(c, 0x10);
                    frame.restore_fpr(c, 20, 0x40);
                    frame.close(c);
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, 0x0012_f398, 0x0012_f588);
                    return Ok(());
                }
            }
        }
    });
}
