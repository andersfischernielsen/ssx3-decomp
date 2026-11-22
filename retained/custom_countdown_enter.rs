//! Native countdown entry (0x00234ad0), retained uninstalled.
//!
//! No tests, builds, replays or game runs: disabled by the user.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(ENTRY, countdown_enter, "GameStateCountdown_Enter")];

const ENTRY: u32 = 0x0023_4ad0;
const END: u32 = 0x0023_4be8;
const MODULE_GP: u32 = 0x488;
const APP_GP: u32 = 0x848;
const COMM_GP: u32 = 0x204;
const RACE: u32 = 0xc;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn gp(c: &Cpu, negative: u32) -> u32 { r(c, GP).wrapping_sub(negative) }

/// `void GameStateCountdown_Enter(GameState* self)`.
/// Clear race+0x8c, notify communication/BE, optionally reset scripts/world,
/// and prepare the presentation. Modes4..6 immediately advance the state.
/// Other modes call AI_SetRaceState(race,4), which starts the180-frame timer,
/// then initialize countdown audio and run the named world-script event.
/// The race-state transition occurs only after the optional world reset.
pub fn countdown_enter(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x20 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 9;
                    Frame::open(c, 0x20);
                    c.g[V1][0] = c.lw(gp(c, MODULE_GP));
                    frame.save(c, S0, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[A0][0] = c.lw(gp(c, COMM_GP));
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(RACE));
                    let no_comm = c.g[A0][0] == 0;
                    c.sw(r(c, V0).wrapping_add(0x8c), 0);
                    if !no_comm {
                        c.cycles += 2;
                        call_ptr(c, 0x0025_8968, 0x0023_4afc)?;
                    }
                    pc = 0x0023_4afc;
                }
                0x0023_4afc => {
                    c.cycles += 10;
                    c.g[A0][0] = 0;
                    call_ptr(c, 0x0014_de68, 0x0023_4b04)?;
                    pc = 0x0023_4b04;
                }
                0x0023_4b04 => {
                    c.g[V0][0] = c.lw(gp(c, APP_GP));
                    c.g[A1][0] = 7;
                    c.g[A0][0] = c.lw(r(c, V0).wrapping_add(0x78));
                    call_ptr(c, 0x0014_dd58, 0x0023_4b14)?;
                    pc = 0x0023_4b14;
                }
                0x0023_4b14 => {
                    c.g[V1][0] = 0x0053_0000;
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x4b30));
                    let skip_reset = c.g[V0][0] == 0;
                    c.g[V0][0] = c.lw(gp(c, MODULE_GP));
                    if skip_reset {
                        pc = 0x0023_4b30;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw(gp(c, MODULE_GP));
                        call_ptr(c, 0x0023_00f0, 0x0023_4b2c)?;
                        pc = 0x0023_4b2c;
                    }
                }
                0x0023_4b2c => {
                    c.g[V0][0] = c.lw(gp(c, MODULE_GP));
                    pc = 0x0023_4b30;
                }
                0x0023_4b30 => {
                    c.cycles += 16;
                    c.g[A0][0] = c.lw(r(c, V0).wrapping_add(0x28));
                    call_ptr(c, 0x0026_f228, 0x0023_4b38)?;
                    pc = 0x0023_4b38;
                }
                0x0023_4b38 => {
                    c.g[V0][0] = c.lw(gp(c, MODULE_GP));
                    c.g[A0][0] = c.lw(r(c, V0).wrapping_add(0x28));
                    call_ptr(c, 0x0026_f7b8, 0x0023_4b44)?;
                    pc = 0x0023_4b44;
                }
                0x0023_4b44 => {
                    call_ptr(c, 0x0014_dc80, 0x0023_4b4c)?;
                    pc = 0x0023_4b4c;
                }
                0x0023_4b4c => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0014_dd58, 0x0023_4b58)?;
                    pc = 0x0023_4b58;
                }
                0x0023_4b58 => {
                    c.g[V1][0] = 0x0053_0000;
                    c.g[V0][0] = c.lb(r(c, V1).wrapping_add(0x5c10));
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(4));
                    c.g[V0][0] = (c.g[V0][0] < 3) as u64;
                    let run_countdown = c.g[V0][0] == 0;
                    c.g[V0][0] = c.lw(gp(c, MODULE_GP));
                    if run_countdown {
                        pc = 0x0023_4b80;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0023_3aa0, 0x0023_4b78)?;
                        pc = 0x0023_4b78;
                    }
                }
                0x0023_4b78 => {
                    frame.restore(c, S0, 0x10);
                    pc = 0x0023_4bdc;
                }
                0x0023_4b80 => {
                    c.cycles += 23;
                    c.g[A1][0] = 4;
                    c.g[A0][0] = c.lw(r(c, V0).wrapping_add(RACE));
                    call_ptr(c, 0x0011_3b10, 0x0023_4b8c)?;
                    pc = 0x0023_4b8c;
                }
                0x0023_4b8c => {
                    call_ptr(c, 0x0028_b180, 0x0023_4b94)?;
                    pc = 0x0023_4b94;
                }
                0x0023_4b94 => {
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0029_c418, 0x0023_4b9c)?;
                    pc = 0x0023_4b9c;
                }
                0x0023_4b9c => {
                    call_ptr(c, 0x0028_b180, 0x0023_4ba4)?;
                    pc = 0x0023_4ba4;
                }
                0x0023_4ba4 => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 3;
                    call_ptr(c, 0x0029_c420, 0x0023_4bb0)?;
                    pc = 0x0023_4bb0;
                }
                0x0023_4bb0 => {
                    c.g[A0][0] = 0x0048_0000;
                    c.g[A0][0] = sx(r(c, A0).wrapping_sub(0x3fc8));
                    call_ptr(c, 0x0031_7670, 0x0023_4bbc)?;
                    pc = 0x0023_4bbc;
                }
                0x0023_4bbc => {
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0023_4be8, 0x0023_4bc8)?;
                    pc = 0x0023_4bc8;
                }
                0x0023_4bc8 => {
                    c.g[V0][0] = c.lw(gp(c, MODULE_GP));
                    c.g[A0][0] = c.lw(r(c, V0).wrapping_add(0x48));
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f840, 0x0023_4bd8)?;
                    pc = 0x0023_4bd8;
                }
                0x0023_4bd8 => {
                    frame.restore(c, S0, 0x10);
                    pc = 0x0023_4bdc;
                }
                0x0023_4bdc => {
                    c.cycles += 3;
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, END); return Ok(()); }
            }
        }
    });
}
