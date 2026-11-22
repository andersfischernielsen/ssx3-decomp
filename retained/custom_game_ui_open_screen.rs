//! Native game-state to overlay-screen selection0x20a8f8, retained uninstalled.
//!
//! This is distinct from GameUI_OpenScreen20ab50, which accepts screen IDs.
//! States2 and7 select prerace/results screens;0xc/0xd are screen IDs and
//! do not select a screen here. No tests, build or runtime installation.

use crate::call::{call_ptr, guest, Callee, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0020_a8f8;
const END: u32 = 0x0020_ab4c;
const RESTORE_S0: u32 = 0x0020_ab2c;
const EPILOGUE: u32 = 0x0020_ab30;
const BE_GET: u32 = 0x0014_dc80;
const BE_INTERFACE: u32 = 0x0014_dd58;
const RESULT_SCREEN: Callee = Callee {
    addr: 0x0020_a8b0,
    f: ssx3_ee::s05::GameUI_GetResultsScreenId,
};
const OPEN_SCREEN: Callee = Callee {
    addr: 0x0020_ab50,
    f: ssx3_ee::s05::GameUI_OpenScreen,
};

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 { c.g[reg][0] as u32 }
#[inline]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 { r(c, reg).wrapping_add(off) }
#[inline]
fn gp(c: &Cpu, off: i32) -> u32 { r(c, GP).wrapping_add(off as u32) }

/// `void GameUI_OpenScreenForState(int state)`.
/// Suppress requests while the module state is nonzero and below10; map
/// state2's event/game kind or state7's score/reward status to a screen,
/// and invoke the ordinary screen constructor only when a nonzero ID results.
/// The fallback writes pending state2 at gp-9f4, rather than opening a dialog.
pub fn game_ui_open_screen_for_state(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x60 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 14;
                    Frame::open(c, 0x60);
                    c.g[V0][0] = c.lw(gp(c, -0x848));
                    frame.save(c, S2, 0x30);
                    frame.save(c, S0, 0x50);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, S1, 0x40);
                    frame.save(c, S3, 0x20);
                    frame.save(c, S4, 0x10);
                    frame.save_ra(c, 0);
                    c.g[V1][0] = c.lw(at(c, V0, 0x84));
                    c.g[A0][0] = c.lw(at(c, V1, 0x28));
                    c.g[A0][0] = c.lw(r(c, A0));
                    c.g[A1][0] = 0;
                    if c.g[A0][0] != 0 {
                        c.cycles += 1;
                        c.g[A1][0] = ((c.g[A0][0] as i64) < 10) as u64;
                    }
                    pc = 0x0020_a934;
                }
                0x0020_a930 => {
                    c.cycles += 1;
                    c.g[A1][0] = ((c.g[A0][0] as i64) < 10) as u64;
                    pc = 0x0020_a934;
                }
                0x0020_a934 => {
                    c.cycles += 2;
                    let gated = c.g[A1][0] != 0;
                    frame.restore(c, S0, 0x50);
                    pc = if gated { EPILOGUE } else { 0x0020_a93c };
                }
                0x0020_a93c => {
                    c.cycles += 15;
                    c.g[S3][0] = 0;
                    call_ptr(c, BE_GET, 0x0020_a944)?;
                    pc = 0x0020_a944;
                }
                0x0020_a944 => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, BE_INTERFACE, 0x0020_a950)?;
                    pc = 0x0020_a950;
                }
                0x0020_a950 => {
                    c.g[A1][0] = c.lw(gp(c, -0x848));
                    c.g[V0][0] = 0x0053_5bc8;
                    c.g[A0][0] = 2;
                    c.g[V1][0] = c.lw(at(c, A1, 0xc0));
                    c.g[S1][0] = c.lb(at(c, V0, 0x4a));
                    c.g[S4][0] = c.lw(r(c, V1));
                    c.g[S0][0] = c.lb(at(c, V0, 0x48));
                    let prerace = c.g[S2][0] == c.g[A0][0];
                    c.g[V1][0] = c.lb(at(c, V0, 0x49));
                    pc = if prerace { 0x0020_a98c } else { 0x0020_a978 };
                }
                0x0020_a978 => {
                    c.cycles += 3;
                    c.g[V0][0] = 7;
                    if c.g[S2][0] == 7 {
                        c.g[A0][0] = c.lw(at(c, A1, 0x78));
                        pc = 0x0020_a9ec;
                    } else {
                        c.cycles += 2;
                        pc = 0x0020_ab14;
                    }
                }
                0x0020_a984 => { c.cycles += 2; pc = 0x0020_ab14; }
                0x0020_a98c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(gp(c, -0x1ec8));
                    let special = c.g[V0][0] != 0;
                    c.sw(gp(c, -0x9ec), 0);
                    if special {
                        c.cycles += 2;
                        c.g[S3][0] = 0x11;
                        pc = 0x0020_ab14;
                    } else { pc = 0x0020_a9a0; }
                }
                0x0020_a998 => { c.cycles += 2; c.g[S3][0] = 0x11; pc = 0x0020_ab14; }
                0x0020_a9a0 => {
                    c.cycles += 3;
                    c.g[V0][0] = 4;
                    if c.g[S0][0] == 4 {
                        c.cycles += 2;
                        c.sw(gp(c, -0x9f4), c.g[S2][0]);
                        pc = 0x0020_ab14;
                    } else { pc = 0x0020_a9b4; }
                }
                0x0020_a9ac => { c.cycles += 2; c.sw(gp(c, -0x9f4), c.g[S2][0]); pc = 0x0020_ab14; }
                0x0020_a9b4 => {
                    c.cycles += 2;
                    let event = c.g[S1][0] != 0;
                    c.sw(gp(c, -0x9f4), 0);
                    if event { pc = 0x0020_a9c4; }
                    else { c.cycles += 2; c.g[S3][0] = 8; pc = 0x0020_ab14; }
                }
                0x0020_a9bc => { c.cycles += 2; c.g[S3][0] = 8; pc = 0x0020_ab14; }
                0x0020_a9c4 => {
                    c.cycles += 2;
                    let two_player = c.g[V1][0] == c.g[S2][0];
                    c.g[V0][0] = sx(r(c, S1).wrapping_sub(4));
                    if two_player { c.cycles += 2; c.g[S3][0] = 0x15; pc = 0x0020_ab14; }
                    else { pc = 0x0020_a9d4; }
                }
                0x0020_a9cc => { c.cycles += 2; c.g[S3][0] = 0x15; pc = 0x0020_ab14; }
                0x0020_a9d4 => {
                    c.cycles += 6;
                    c.g[V1][0] = (7u64 < c.g[V0][0]) as u64;
                    c.g[S3][0] = 9;
                    c.g[V0][0] = 0x14;
                    if c.g[V1][0] == 0 { c.g[S3][0] = c.g[V0][0]; }
                    pc = 0x0020_ab14;
                }
                0x0020_a9ec => {
                    c.cycles += 5;
                    c.g[A1][0] = 7;
                    call_ptr(c, BE_INTERFACE, 0x0020_a9f4)?;
                    pc = 0x0020_a9f4;
                }
                0x0020_a9f4 => {
                    c.g[V1][0] = 4;
                    if c.g[S0][0] == 4 {
                        c.cycles += 2;
                        c.sw(gp(c, -0x9f4), c.g[S0][0]);
                        pc = RESTORE_S0;
                    } else { pc = 0x0020_aa08; }
                }
                0x0020_aa00 => { c.cycles += 2; c.sw(gp(c, -0x9f4), c.g[S0][0]); pc = RESTORE_S0; }
                0x0020_aa08 => {
                    c.cycles += 14;
                    c.sw(gp(c, -0x9f4), 0);
                    call_ptr(c, BE_GET, 0x0020_aa10)?;
                    pc = 0x0020_aa10;
                }
                0x0020_aa10 => {
                    c.g[A0][0] = c.g[V0][0]; c.g[A1][0] = 8;
                    call_ptr(c, BE_INTERFACE, 0x0020_aa1c)?;
                    pc = 0x0020_aa1c;
                }
                0x0020_aa1c => {
                    c.g[S0][0] = c.g[V0][0];
                    call_ptr(c, BE_GET, 0x0020_aa24)?;
                    pc = 0x0020_aa24;
                }
                0x0020_aa24 => {
                    c.g[A0][0] = c.g[V0][0]; c.g[A1][0] = 0xd;
                    call_ptr(c, BE_INTERFACE, 0x0020_aa30)?;
                    pc = 0x0020_aa30;
                }
                0x0020_aa30 => {
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw(at(c, S0, 0x18));
                    c.g[V1][0] = 0;
                    pc = if (c.g[V0][0] as i64) >= 0 { 0x0020_aa4c } else { 0x0020_aa40 };
                }
                0x0020_aa40 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, S0, 0x1c));
                    pc = if (c.g[V0][0] as i64) < 0 { 0x0020_aa50 } else { 0x0020_aa4c };
                }
                0x0020_aa4c => { c.cycles += 1; c.g[V1][0] = 1; pc = 0x0020_aa50; }
                0x0020_aa50 => {
                    c.cycles += 2;
                    let has_rank = c.g[V1][0] != 0;
                    c.g[V0][0] = 0x0053_0000;
                    pc = if has_rank { 0x0020_aa58 } else { 0x0020_aa84 };
                }
                0x0020_aa58 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(at(c, V0, 0x4b30));
                    let flag = c.g[V1][0] != 0;
                    c.g[S0][0] = c.g[V0][0];
                    if flag {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S4][0]; c.g[A0][0] = c.g[S1][0];
                        RESULT_SCREEN.call(c, 0x0020_aa70)?;
                        pc = 0x0020_aa70;
                    } else { pc = 0x0020_aa78; }
                }
                0x0020_aa64 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S4][0]; c.g[A0][0] = c.g[S1][0];
                    RESULT_SCREEN.call(c, 0x0020_aa70)?;
                    pc = 0x0020_aa70;
                }
                0x0020_aa70 => { c.g[S3][0] = c.g[V0][0]; pc = 0x0020_aab8; }
                0x0020_aa78 => {
                    c.cycles += 3;
                    c.sw(gp(c, -0x9f0), 0); c.g[S3][0] = 0xf;
                    pc = 0x0020_aab8;
                }
                0x0020_aa84 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0015_8f30, 0x0020_aa8c)?;
                    pc = 0x0020_aa8c;
                }
                0x0020_aa8c => {
                    let rewards = (c.g[V0][0] as i64) > 0;
                    c.g[S3][0] = 0x10;
                    pc = if rewards { 0x0020_aaa0 } else { 0x0020_aa94 };
                }
                0x0020_aa94 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, S2, 0x10));
                    let none = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S4][0];
                    pc = if none { 0x0020_aaa8 } else { 0x0020_aaa0 };
                }
                0x0020_aaa0 => { c.cycles += 2; c.g[S0][0] = 0x0053_0000; pc = 0x0020_aab8; }
                0x0020_aaa8 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    RESULT_SCREEN.call(c, 0x0020_aab0)?;
                    pc = 0x0020_aab0;
                }
                0x0020_aab0 => { c.g[S0][0] = 0x0053_0000; c.g[S3][0] = c.g[V0][0]; pc = 0x0020_aab8; }
                0x0020_aab8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S1][0] ^ 6;
                    c.g[A0][0] = 0;
                    pc = if c.g[V0][0] == 0 { 0x0020_aaec } else { 0x0020_aac4 };
                }
                0x0020_aac4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S1][0] ^ 7;
                    let seven = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[S1][0] ^ 8;
                    pc = if seven { 0x0020_aaec } else { 0x0020_aad0 };
                }
                0x0020_aad0 | 0x0020_aad8 | 0x0020_aae0 => {
                    c.cycles += 2;
                    let match_previous = c.g[V0][0] == 0;
                    let (kind, next) = match pc {
                        0x0020_aad0 => (9, 0x0020_aad8),
                        0x0020_aad8 => (0xa, 0x0020_aae0),
                        _ => (0xb, 0x0020_aae8),
                    };
                    c.g[V0][0] = c.g[S1][0] ^ kind;
                    pc = if match_previous { 0x0020_aaec } else { next };
                }
                0x0020_aae8 => { c.cycles += 1; c.g[A0][0] = (0 < c.g[V0][0]) as u64; pc = 0x0020_aaec; }
                0x0020_aaec => {
                    c.cycles += 2;
                    let request_module = c.g[A0][0] != 0;
                    c.g[V0][0] = c.lw(gp(c, -0x848));
                    if request_module {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw(at(c, V0, 0x84));
                        c.g[A0][0] = c.lw(at(c, V1, 0x28));
                        call_ptr(c, 0x0027_06b8, 0x0020_ab00)?;
                    }
                    pc = 0x0020_ab00;
                }
                0x0020_aaf4 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(at(c, V0, 0x84));
                    c.g[A0][0] = c.lw(at(c, V1, 0x28));
                    call_ptr(c, 0x0027_06b8, 0x0020_ab00)?;
                    pc = 0x0020_ab00;
                }
                0x0020_ab00 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, S0, 0x4b30));
                    let flagged = c.g[V0][0] != 0;
                    c.g[V1][0] = c.lw(gp(c, -0x204));
                    if flagged {
                        c.cycles += 2; c.g[V0][0] = 0x8ca0;
                        c.sw(at(c, V1, 0x64), c.g[V0][0]);
                    }
                    pc = 0x0020_ab14;
                }
                0x0020_ab0c => {
                    c.cycles += 2; c.g[V0][0] = 0x8ca0;
                    c.sw(at(c, V1, 0x64), c.g[V0][0]);
                    pc = 0x0020_ab14;
                }
                0x0020_ab14 => {
                    c.cycles += 2;
                    let has_screen = c.g[S3][0] != 0;
                    c.g[V0][0] = 2;
                    if has_screen { pc = 0x0020_ab24; }
                    else { c.cycles += 2; c.sw(gp(c, -0x9f4), c.g[V0][0]); pc = RESTORE_S0; }
                }
                0x0020_ab1c => { c.cycles += 2; c.sw(gp(c, -0x9f4), c.g[V0][0]); pc = RESTORE_S0; }
                0x0020_ab24 => {
                    c.cycles += 2; c.g[A0][0] = c.g[S3][0];
                    OPEN_SCREEN.call(c, RESTORE_S0)?;
                    pc = RESTORE_S0;
                }
                RESTORE_S0 => { c.cycles += 1; frame.restore(c, S0, 0x50); pc = EPILOGUE; }
                EPILOGUE => {
                    c.cycles += 7;
                    frame.restore(c, S1, 0x40); frame.restore(c, S2, 0x30);
                    frame.restore(c, S3, 0x20); frame.restore(c, S4, 0x10);
                    frame.restore_ra(c, 0); frame.close(c);
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, END); return Ok(()); }
            }
        }
    });
}
