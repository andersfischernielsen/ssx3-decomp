//! Native Event Select entry0x236250, retained uninstalled.
//!
//! Generated EE owns registers, charges and call order. No tests, builds,
//! replays or game runs: the user's explicit instruction disables them.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(ENTRY, event_select_enter, "GameStateEventSelect_Enter")];

const ENTRY: u32 = 0x0023_6250;
const END: u32 = 0x0023_6418;
const MODE: u32 = 0x14;
const EVENT: u32 = 0x18;
const SCRIPT_ENGINE_GP: u32 = 0x84c;
const MODULE_GP: u32 = 0x488;
const APP_GP: u32 = 0x848;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 { r(c, register).wrapping_add(offset) }

#[inline]
fn gp(c: &Cpu, negative: u32) -> u32 { r(c, GP).wrapping_sub(negative) }

/// `void GameStateEventSelect_Enter(GameState* self, int mode)`.
/// Deactivate scenery scripts, put AI in state3, clear the rider's selection
/// latch and initialize the entry mode/event fields. Mode1 substitutes the
/// saved global event value; matching the current BE track resets the race.
/// Mode0 queues script choices0x16/0x17. Modes1..3 prepare the existing list,
/// optionally queue choice0xb, invoke rider helper0x11d390 and start with
/// mode==2. That helper's body is outside this investigation.
/// This routine does not itself construct EventSelect UI peak entries.
pub fn event_select_enter(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x40 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 32;
                    Frame::open(c, 0x40);
                    frame.save(c, S0, 0x30);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save(c, S1, 0x20);
                    c.g[A0][0] = c.lw(r(c, GP).wrapping_add(0xce8));
                    frame.save(c, S2, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S1][0] = c.g[A1][0];
                    call_ptr(c, 0x0030_b7f8, 0x0023_6274)?;
                    pc = 0x0023_6274;
                }
                0x0023_6274 => {
                    call_ptr(c, 0x0014_dc80, 0x0023_627c)?;
                    pc = 0x0023_627c;
                }
                0x0023_627c => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0014_dd58, 0x0023_6288)?;
                    pc = 0x0023_6288;
                }
                0x0023_6288 => {
                    c.g[S2][0] = c.g[V0][0];
                    c.g[A1][0] = 3;
                    c.g[V0][0] = c.lw(gp(c, MODULE_GP));
                    c.g[A0][0] = c.lw(at(c, V0, 12));
                    call_ptr(c, 0x0011_3b10, 0x0023_629c)?;
                    pc = 0x0023_629c;
                }
                0x0023_629c => {
                    c.g[A0][0] = c.lw(gp(c, MODULE_GP));
                    c.g[A1][0] = 0x17;
                    c.g[A2][0] = 1;
                    c.g[V0][0] = c.lw(at(c, A0, 12));
                    c.g[V1][0] = c.lw(at(c, V0, 0x40));
                    c.g[A0][0] = c.lw(at(c, V1, 0x18));
                    c.sw(at(c, A0, 0x2f8), 0);
                    c.sw(at(c, S0, EVENT), c.g[A1][0]);
                    c.sw(at(c, S0, 0x10), 0);
                    c.sw(at(c, S0, MODE), c.g[S1][0]);
                    c.sw(at(c, S0, 8), 0);
                    let ordinary_mode = c.g[S1][0] != c.g[A2][0];
                    c.sw(at(c, S0, 0x1c), 0);
                    if ordinary_mode {
                        pc = 0x0023_62ec;
                    } else {
                        c.cycles += 7;
                        c.sw(at(c, S0, 8), c.g[S1][0]);
                        call_ptr(c, 0x0014_dc80, 0x0023_62d8)?;
                        pc = 0x0023_62d8;
                    }
                }
                0x0023_62d8 => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0014_dd58, 0x0023_62e4)?;
                    pc = 0x0023_62e4;
                }
                0x0023_62e4 => {
                    c.g[V1][0] = c.lw(gp(c, 0x1f34));
                    c.sw(at(c, S0, EVENT), c.g[V1][0]);
                    pc = 0x0023_62ec;
                }
                0x0023_62ec => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0014_4bc0, 0x0023_62f4)?;
                    pc = 0x0023_62f4;
                }
                0x0023_62f4 => {
                    c.g[V1][0] = c.lw(r(c, V0));
                    c.g[V0][0] = c.lw(at(c, S0, EVENT));
                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[V1][0] = c.lw(at(c, S0, MODE));
                        pc = 0x0023_6314;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw(gp(c, APP_GP));
                        c.g[A0][0] = c.lw(at(c, V0, 0x84));
                        call_ptr(c, 0x0023_0180, 0x0023_6310)?;
                        pc = 0x0023_6310;
                    }
                }
                0x0023_6310 => {
                    c.g[V1][0] = c.lw(at(c, S0, MODE));
                    pc = 0x0023_6314;
                }
                0x0023_6314 => {
                    c.cycles += 2;
                    let nonzero_mode = c.g[V1][0] != 0;
                    c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
                    if nonzero_mode {
                        pc = 0x0023_6388;
                    } else {
                        c.cycles += 27;
                        c.g[A1][0] = 1;
                        c.g[A2][0] = 0x16;
                        c.g[A3][0] = 3;
                        c.g[T0][0] = 0;
                        c.g[T1][0] = u64::MAX;
                        c.g[T2][0] = u64::MAX;
                        c.g[T3][0] = u64::MAX;
                        call_ptr(c, 0x0027_8e50, 0x0023_633c)?;
                        pc = 0x0023_633c;
                    }
                }
                0x0023_633c => {
                    c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
                    c.g[A2][0] = 0x17;
                    c.g[A3][0] = 1;
                    c.g[A1][0] = 1;
                    c.g[T0][0] = 0;
                    c.g[T1][0] = u64::MAX;
                    c.g[T2][0] = u64::MAX;
                    c.g[T3][0] = u64::MAX;
                    call_ptr(c, 0x0027_8e50, 0x0023_6360)?;
                    pc = 0x0023_6360;
                }
                0x0023_6360 => {
                    c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0027_8f38, 0x0023_636c)?;
                    pc = 0x0023_636c;
                }
                0x0023_636c => {
                    c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
                    c.g[A1][0] = 1;
                    c.g[A2][0] = 1;
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0027_8f68, 0x0023_6380)?;
                    pc = 0x0023_6380;
                }
                0x0023_6380 => {
                    frame.restore(c, S0, 0x30);
                    pc = 0x0023_6404;
                }
                0x0023_6388 => {
                    c.cycles += 2;
                    let negative_mode = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 4) as u64;
                    if negative_mode {
                        pc = 0x0023_6400;
                    } else {
                        c.cycles += 2;
                        let outside_range = c.g[V0][0] == 0;
                        c.g[V0][0] = c.g[V1][0] ^ 1;
                        if outside_range {
                            pc = 0x0023_6400;
                        } else {
                            c.cycles += 2;
                            let mode_one = c.g[V0][0] == 0;
                            c.g[A1][0] = 1;
                            if mode_one {
                                pc = 0x0023_63c0;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
                                c.g[A2][0] = 0xb;
                                c.g[A3][0] = 3;
                                c.g[T0][0] = 0;
                                c.g[T1][0] = u64::MAX;
                                c.g[T2][0] = u64::MAX;
                                c.g[T3][0] = u64::MAX;
                                call_ptr(c, 0x0027_8e50, 0x0023_63c0)?;
                                pc = 0x0023_63c0;
                            }
                        }
                    }
                }
                0x0023_63c0 => {
                    c.cycles += 16;
                    c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0027_8f38, 0x0023_63cc)?;
                    pc = 0x0023_63cc;
                }
                0x0023_63cc => {
                    c.g[V1][0] = c.lw(gp(c, APP_GP));
                    c.g[V0][0] = c.lw(at(c, V1, 0x84));
                    c.g[A0][0] = c.lw(at(c, V0, 12));
                    c.g[V1][0] = c.lw(at(c, A0, 0x40));
                    c.g[A0][0] = c.lw(at(c, V1, 0x18));
                    call_ptr(c, 0x0011_d390, 0x0023_63e4)?;
                    pc = 0x0023_63e4;
                }
                0x0023_63e4 => {
                    c.g[A2][0] = c.lw(at(c, S0, MODE));
                    c.g[A1][0] = 1;
                    c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
                    c.g[A3][0] = 0;
                    c.g[A2][0] ^= 2;
                    c.g[A2][0] = (c.g[A2][0] < 1) as u64;
                    call_ptr(c, 0x0027_8f68, 0x0023_6400)?;
                    pc = 0x0023_6400;
                }
                0x0023_6400 => {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x30);
                    pc = 0x0023_6404;
                }
                0x0023_6404 => {
                    c.cycles += 5;
                    frame.restore(c, S1, 0x20);
                    frame.restore(c, S2, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, END); return Ok(()); }
            }
        }
    });
}
