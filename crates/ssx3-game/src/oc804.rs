//! Staging module oc804 (wave wave247): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc804`.

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
    (0x0020ab50, game_ui_open_screen, "GameUI_OpenScreen"),
    (
        0x0022d6c8,
        section_man_on_event_trigger,
        "SectionMan_OnEventTrigger",
    ),
    (
        0x00246cf8,
        game_load_state_update_player_ranking,
        "cGameLoadState_UpdatePlayerRanking",
    ),
    (0x0026ada8, path_data_load_file, "PathData_LoadFile"),
];

/// `PathData_LoadFile` (0x0026ada8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn path_data_load_file(c: &mut Cpu) {
    if c.pc != 0x0026ada8 {
        return ee::s07::f_0026ada8__generated(c);
    }
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A0][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x0026adc4);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x003e22f0, 0x0026adc4)?;
        c.g[V1][0] = 0x1;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
        c.g[RA][0] = sx(0x0026add8);
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x0026adf0,
            f: ee::s07::PathData_Parse,
        }
        .call(c, 0x0026add8)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `SectionMan_OnEventTrigger` (0x0022d6c8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn section_man_on_event_trigger(c: &mut Cpu) {
    if c.pc != 0x0022d6c8 {
        return ee::s05::f_0022d6c8__generated(c);
    }
    guest(c, |c| {
        'b_22d748: {
            'b_22d744: {
                c.cycles += 19;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[RA][0] = sx(0x0022d6ec);
                c.g[S2][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x0022d6ec)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0022d6f8);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x0022d6f8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = 0x530000;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                let t = c.g[V1][0] != 0u64;
                c.g[A0][0] = 0u64;
                if t {
                    break 'b_22d744;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = c.g[V0][0] ^ 0x4;
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = 0x1;
                        break 'b_22d748;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1bc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
                        c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[A0][0] = 0x1;
                            break 'b_22d748;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x1;
                            let t = c.g[S1][0] != c.g[V0][0];

                            if t {
                                break 'b_22d748;
                            } else {
                                break 'b_22d744;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[A0][0] = 0x1;
            break 'b_22d748;
        }
        'b_22d8bc: {
            c.cycles += 2;
            if c.g[A0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_22d8bc;
            } else {
                c.cycles += 8;
                c.g[RA][0] = sx(0x0022d758);
                c.g[A0][0] = 0u64;
                call_ptr(c, 0x0014de68, 0x0022d758)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A0][0] = 0x4;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x214));
                if c.g[V1][0] != c.g[A0][0] {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    break 'b_22d8bc;
                } else {
                    'b_22d880: {
                        'b_22d7c0: {
                            c.cycles += 13;
                            c.g[RA][0] = sx(0x0022d778);
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                            call_ptr(c, 0x0012b180, 0x0022d778)?;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x78));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1bc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
                            c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_22d7c0;
                            } else {
                                c.cycles += 7;
                                c.g[RA][0] = sx(0x0022d7ac);
                                c.g[A1][0] = 0x4;
                                call_ptr(c, 0x00144df0, 0x0022d7ac)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x0022d7b8);
                                c.g[A1][0] = 0xc;
                                call_ptr(c, 0x001451e8, 0x0022d7b8)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                if t {
                                    break 'b_22d880;
                                } else {
                                    break 'b_22d7c0;
                                }
                            }
                        }
                        'b_22d87c: {
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[S1][0] < 0x7u64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x480000;
                            if t {
                                break 'b_22d87c;
                            } else {
                                'b_22d864: {
                                    'b_22d844: {
                                        'b_22d824: {
                                            'b_22d804: {
                                                c.cycles += 6;
                                                c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x4bb0));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                let t = c.g[A0][0] as u32;
                                                match t {
                                                    0x0022d7e4 => {
                                                        c.cycles += 8;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x0022d7f0);
                                                        c.g[A1][0] = 0u64;
                                                        call_ptr(c, 0x00144df0, 0x0022d7f0)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x0022d7fc);
                                                        c.g[A1][0] = 0u64;
                                                        call_ptr(c, 0x001451e8, 0x0022d7fc)?;
                                                        let t = true;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        if t {
                                                            break 'b_22d880;
                                                        } else {
                                                            break 'b_22d804;
                                                        }
                                                    }
                                                    0x0022d804 => {
                                                        break 'b_22d804;
                                                    }
                                                    0x0022d824 => {
                                                        break 'b_22d824;
                                                    }
                                                    0x0022d844 => {
                                                        break 'b_22d844;
                                                    }
                                                    0x0022d864 => {
                                                        break 'b_22d864;
                                                    }
                                                    0x0022d87c => {
                                                        break 'b_22d87c;
                                                    }
                                                    _ => {
                                                        c.jump_out(t, 0x0022d6c8, 0x0022d8d4);
                                                        return Ok(());
                                                    }
                                                }
                                            }
                                            c.cycles += 8;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x0022d810);
                                            c.g[A1][0] = 0x1;
                                            call_ptr(c, 0x00144df0, 0x0022d810)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x0022d81c);
                                            c.g[A1][0] = 0x1;
                                            call_ptr(c, 0x001451e8, 0x0022d81c)?;
                                            let t = true;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                            if t {
                                                break 'b_22d880;
                                            } else {
                                                break 'b_22d824;
                                            }
                                        }
                                        c.cycles += 8;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x0022d830);
                                        c.g[A1][0] = 0x2;
                                        call_ptr(c, 0x00144df0, 0x0022d830)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x0022d83c);
                                        c.g[A1][0] = 0x3;
                                        call_ptr(c, 0x001451e8, 0x0022d83c)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                        if t {
                                            break 'b_22d880;
                                        } else {
                                            break 'b_22d844;
                                        }
                                    }
                                    c.cycles += 8;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x0022d850);
                                    c.g[A1][0] = 0x3;
                                    call_ptr(c, 0x00144df0, 0x0022d850)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x0022d85c);
                                    c.g[A1][0] = 0x2;
                                    call_ptr(c, 0x001451e8, 0x0022d85c)?;
                                    let t = true;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                    if t {
                                        break 'b_22d880;
                                    } else {
                                        break 'b_22d864;
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x0022d870);
                                c.g[A1][0] = 0x5;
                                call_ptr(c, 0x00144df0, 0x0022d870)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x0022d87c);
                                c.g[A1][0] = 0x4;
                                call_ptr(c, 0x001451e8, 0x0022d87c)?;
                                break 'b_22d87c;
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        break 'b_22d880;
                    }
                    c.cycles += 15;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0022d890);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0022d890)?;
                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5bc8));
                    c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                    c.g[RA][0] = sx(0x0022d8a4);
                    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc0));
                    call_ptr(c, 0x00238160, 0x0022d8a4)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b0));
                    c.g[A1][0] = 0x1;
                    c.g[A2][0] = 0x1;
                    c.g[RA][0] = sx(0x0022d8b8);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00231250, 0x0022d8b8)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    break 'b_22d8bc;
                }
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `cGameLoadState_UpdatePlayerRanking` (0x00246cf8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn game_load_state_update_player_ranking(c: &mut Cpu) {
    if c.pc != 0x00246cf8 {
        return ee::s06::f_00246cf8__generated(c);
    }
    guest(c, |c| {
        'b_247294: {
            c.cycles += 25;
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x210));
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x150), c.g[A0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x154), c.g[A1][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x200));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x1f0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1e0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1d0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1c0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x1b0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x1a0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x190));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x180));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x170), c.g[RA][0]);
            c.g[RA][0] = sx(0x00246d3c);
            call_ptr(c, 0x0014dc80, 0x00246d3c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00246d48);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x00246d48)?;
            c.g[V1][0] = 0x530000;
            c.g[A0][0] = 0x2;
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
            let t = c.g[V0][0] != c.g[A0][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x200));
            if t {
                break 'b_247294;
            } else {
                'b_2470d0: {
                    'b_246fe0: {
                        'b_246fdc: {
                            c.cycles += 69;
                            c.g[V0][0] = 0x4a0000;
                            c.g[V1][0] = 0x4a0000;
                            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2c10));
                            c.lwl(4, (c.g[A3][0] as u32).wrapping_add(0x3));
                            c.lwr(4, c.g[A3][0] as u32);
                            c.swl(4, (c.g[SP][0] as u32).wrapping_add(0x3));
                            c.swr(4, c.g[SP][0] as u32);
                            c.g[FP][0] = 0xc;
                            c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2c18));
                            c.g[T0][0] = c.lb(c.g[A2][0] as u32);
                            c.g[V0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x1));
                            c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                            c.sb((c.g[SP][0] as u32).wrapping_add(0x11), c.g[V0][0]);
                            c.g[A1][0] = 0x4a0000;
                            c.g[S2][0] = 0x530000;
                            c.g[S6][0] = 0x1;
                            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                            c.g[S7][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8b8));
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x30));
                            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x31));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x158), c.g[A0][0]);
                            c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c20));
                            c.lwl(8, (c.g[A3][0] as u32).wrapping_add(0x3));
                            c.lwr(8, c.g[A3][0] as u32);
                            c.g[V1][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x4));
                            c.swl(8, (c.g[SP][0] as u32).wrapping_add(0x23));
                            c.swr(8, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.sb((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                            c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                            c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                            c.g[S1][0] = (0u64 < c.g[T0][0]) as u64;
                            c.g[RA][0] = sx(0x00246de0);
                            c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                            call_ptr(c, 0x0014dc80, 0x00246de0)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00246dec);
                            c.g[A1][0] = 0x1;
                            call_ptr(c, 0x0014dd58, 0x00246dec)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00246dfc);
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x00146e98, 0x00246dfc)?;
                            c.g[RA][0] = sx(0x00246e04);
                            c.g[S4][0] = c.g[V0][0];
                            call_ptr(c, 0x0014dc80, 0x00246e04)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00246e10);
                            c.g[A1][0] = 0x3;
                            call_ptr(c, 0x0014dd58, 0x00246e10)?;
                            c.g[S3][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00246e20);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x00146e98, 0x00246e20)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00246e2c);
                            c.g[A0][0] = c.g[S3][0];
                            call_ptr(c, 0x00148950, 0x00246e2c)?;
                            c.g[S5][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00246e3c);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x00146e98, 0x00246e3c)?;
                            c.mult(0, 2, 2, 30);
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A1][0] = 0x7;
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S7][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
                            c.g[V1][0] = c.g[V1][0] & 0x1;
                            c.g[RA][0] = sx(0x00246e64);
                            if c.g[V1][0] != 0 {
                                c.g[S5][0] = c.g[S6][0];
                            }
                            call_ptr(c, 0x0014dd58, 0x00246e64)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                            if t {
                                break 'b_246fdc;
                            } else {
                                'b_246e88: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == c.g[S6][0];
                                    c.g[A1][0] = 0x530000;
                                    if t {
                                        break 'b_246e88;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4b30));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                        let t = c.g[V1][0] == 0u64;
                                        c.g[V1][0] = 0xc;
                                        if t {
                                            break 'b_246fe0;
                                        } else {
                                            break 'b_246e88;
                                        }
                                    }
                                }
                                'b_246ee4: {
                                    c.cycles += 4;
                                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4b30));
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_246ee4;
                                    } else {
                                        c.cycles += 10;
                                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x160), c.g[V1][0]);
                                        c.g[S5][0] = 0x8;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x15c), c.g[A0][0]);
                                        c.g[A1][0] = 0x4a0000;
                                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                                        c.g[A0][0] = 0x28;
                                        c.g[V1][0] = 0x6;
                                        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x48));
                                        'l_246ec0: loop {
                                            c.cycles += 7;
                                            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                            let t = (c.g[V1][0] as i64) >= 0;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00246ec0);
                                                }
                                                continue 'l_246ec0;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A3][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                                                if t {
                                                    break 'b_2470d0;
                                                } else {
                                                    break 'b_246ee4;
                                                }
                                            }
                                        }
                                    }
                                }
                                'b_246f44: {
                                    c.cycles += 7;
                                    c.mult(0, 2, 20, 30);
                                    c.g[V1][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[S7][0] as u32));
                                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_246f44;
                                    } else {
                                        c.cycles += 8;
                                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x160), c.g[V1][0]);
                                        c.g[A1][0] = 0x4a0000;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x15c), c.g[A0][0]);
                                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                                        c.g[A0][0] = 0x5;
                                        c.g[V1][0] = 0x6;
                                        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x48));
                                        'l_246f20: loop {
                                            c.cycles += 7;
                                            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                            let t = (c.g[V1][0] as i64) >= 0;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00246f20);
                                                }
                                                continue 'l_246f20;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A3][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                                                if t {
                                                    break 'b_2470d0;
                                                } else {
                                                    break 'b_246f44;
                                                }
                                            }
                                        }
                                    }
                                }
                                c.cycles += 11;
                                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4a8));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x160), c.g[V0][0]);
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x15c), c.g[V1][0]);
                                c.g[A0][0] = 0u64;
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                c.g[A1][0] = 0x4a0000;
                                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                                'l_246f70: loop {
                                    c.cycles += 7;
                                    c.g[V0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                    c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = ((c.g[A2][0] as i64) < 7i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00246f70);
                                        }
                                        continue 'l_246f70;
                                    } else {
                                        'b_246fa8: {
                                            c.cycles += 7;
                                            c.g[V0][0] = 0x23;
                                            c.g[V1][0] = 0x530000;
                                            c.div(0, 4, 2);
                                            c.g[T0][0] = c.g[A3][0];
                                            c.g[A2][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0x4fd8));
                                            if c.g[V0][0] == 0u64 {
                                                break 'b_246fa8;
                                            } else {
                                                break 'b_246fa8;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[A3][0] = 0u64;
                                        c.g[A0][0] = c.lo[0];
                                        c.g[S5][0] = c.g[A0][0];
                                        'l_246fb8: loop {
                                            c.cycles += 7;
                                            c.g[V0][0] =
                                                sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A3][0] as u32));
                                            c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                            c.g[V0][0] = ((c.g[A3][0] as i64) < 7i64) as u64;
                                            c.sw(c.g[T0][0] as u32, c.g[V1][0]);
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00246fb8);
                                                }
                                                continue 'l_246fb8;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A3][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                                                if t {
                                                    break 'b_2470d0;
                                                } else {
                                                    break 'b_246fdc;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = 0xc;
                        break 'b_246fe0;
                    }
                    'b_247044: {
                        c.cycles += 8;
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8b8));
                        c.mult(0, 3, 20, 3);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                        c.g[V0][0] = c.g[V0][0] & 0x1;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_247044;
                        } else {
                            c.cycles += 8;
                            c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x160), c.g[V0][0]);
                            c.g[A1][0] = 0x4a0000;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x15c), c.g[V1][0]);
                            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                            c.g[A0][0] = 0x5;
                            c.g[V1][0] = 0x6;
                            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x48));
                            'l_247020: loop {
                                c.cycles += 7;
                                c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                let t = (c.g[V1][0] as i64) >= 0;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00247020);
                                    }
                                    continue 'l_247020;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                                    if t {
                                        break 'b_2470d0;
                                    } else {
                                        break 'b_247044;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 35;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00247050);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x001484e0, 0x00247050)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00247060);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x00148680, 0x00247060)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00247070);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x00148410, 0x00247070)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00247080);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x00148750, 0x00247080)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00247090);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x001488f0, 0x00247090)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x002470a0);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x00148820, 0x002470a0)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x002470b0);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V0][0]);
                    call_ptr(c, 0x001485b0, 0x002470b0)?;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                    c.g[A1][0] = 0x4a0000;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x160), c.g[A0][0]);
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x15c), c.g[A2][0]);
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                    break 'b_2470d0;
                }
                'b_24710c: {
                    'b_2470f4: {
                        c.cycles += 2;
                        let t = c.g[A3][0] != 0u64;
                        c.g[S2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c28));
                        if t {
                            break 'b_2470f4;
                        } else {
                            c.cycles += 7;
                            c.g[A0][0] = 0x480000;
                            c.g[RA][0] = sx(0x002470e4);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4368));
                            call_ptr(c, 0x00317670, 0x002470e4)?;
                            c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
                            c.g[A1][0] = c.g[V0][0];
                            let t = true;
                            c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x40));
                            if t {
                                break 'b_24710c;
                            } else {
                                break 'b_2470f4;
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[A0][0] = 0x480000;
                    c.g[RA][0] = sx(0x00247100);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4358));
                    call_ptr(c, 0x00317670, 0x00247100)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
                    break 'b_24710c;
                }
                'b_247140: {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x00247114);
                    call_ptr(c, 0x0039d860, 0x00247114)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00247128);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00417828, 0x00247128)?;
                    let t = c.g[S0][0] == 0u64;
                    c.g[S6][0] = 0u64;
                    if t {
                        break 'b_247140;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x0024713c);
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x003a0c50, 0x0024713c)?;
                        c.g[S6][0] = 0u64;
                        break 'b_247140;
                    }
                }
                c.cycles += 4;
                c.g[FP][0] = 0x5;
                c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x30));
                'l_247150: loop {
                    'b_247234: {
                        c.cycles += 19;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x0024715c);
                        c.sb((c.g[SP][0] as u32).wrapping_add(0x2), c.g[S0][0]);
                        call_ptr(c, 0x00317670, 0x0024715c)?;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
                        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x0024716c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x0024716c)?;
                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                        c.g[S4][0] = c.g[V0][0];
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                        c.sb((c.g[SP][0] as u32).wrapping_add(0x21), c.g[S0][0]);
                        c.g[RA][0] = sx(0x00247184);
                        c.sb((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A2][0]);
                        call_ptr(c, 0x00317670, 0x00247184)?;
                        c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00247194);
                        c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x40));
                        call_ptr(c, 0x0039d860, 0x00247194)?;
                        let t = c.g[S4][0] == 0u64;
                        c.g[S5][0] = c.g[V0][0];
                        if t {
                            break 'b_247234;
                        } else {
                            c.cycles += 2;
                            let t = c.g[S5][0] == 0u64;
                            c.g[V0][0] = sx((c.g[S6][0] as u32) << 2);
                            if t {
                                break 'b_247234;
                            } else {
                                'b_2471ac: {
                                    c.cycles += 2;
                                    if c.g[FP][0] == 0u64 {
                                        break 'b_2471ac;
                                    } else {
                                        break 'b_2471ac;
                                    }
                                }
                                c.cycles += 34;
                                c.g[V1][0] =
                                    sx((c.g[SP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[S2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x30));
                                c.g[T0][0] = 0x480000;
                                c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8c));
                                c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4348));
                                c.div(0, 18, 30);
                                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                c.g[S3][0] = c.lo[0];
                                c.g[RA][0] = sx(0x002471dc);
                                c.g[S0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                call_ptr(c, 0x00317670, 0x002471dc)?;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                c.g[A0][0] = c.g[S0][0];
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x002471ec);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, t, 0x002471ec)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S7][0];
                                c.g[RA][0] = sx(0x002471fc);
                                c.g[A2][0] = c.g[S3][0];
                                call_ptr(c, 0x002c26d0, 0x002471fc)?;
                                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00247214);
                                c.g[A0][0] =
                                    sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00247214)?;
                                c.g[A0][0] = c.g[S4][0];
                                c.g[RA][0] = sx(0x00247220);
                                c.g[A1][0] = c.g[S7][0];
                                call_ptr(c, 0x003a0e90, 0x00247220)?;
                                c.g[V0][0] = 0x37;
                                c.g[A0][0] = c.g[S5][0];
                                c.sw((c.g[S5][0] as u32).wrapping_add(0x80), c.g[V0][0]);
                                c.g[RA][0] = sx(0x00247234);
                                c.g[A1][0] = c.g[S2][0];
                                call_ptr(c, 0x0039c428, 0x00247234)?;
                                break 'b_247234;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S6][0] as i64) < 7i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x30));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00247150);
                        }
                        continue 'l_247150;
                    } else {
                        'b_247290: {
                            c.cycles += 10;
                            c.g[RA][0] = sx(0x0024724c);
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x15c));
                            call_ptr(c, 0x00317670, 0x0024724c)?;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x0024725c);
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
                            call_ptr(c, 0x0039d860, 0x0024725c)?;
                            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
                            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
                            let t = c.g[A0][0] == 0u64;
                            c.g[V1][0] = c.g[V0][0];
                            if t {
                                break 'b_247290;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V1][0] == 0u64;
                                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                                if t {
                                    break 'b_247290;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                                    let t = c.g[V0][0] == 0u64;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x200));
                                    if t {
                                        break 'b_247294;
                                    } else {
                                        c.cycles += 2;
                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x7c), 0u64);
                                        break 'b_247290;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x200));
                        break 'b_247294;
                    }
                }
            }
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x1f0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1e0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x170));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x210));
        c.pc = t;
        return Ok(());
    });
}

/// `GameUI_OpenScreen` (0x0020ab50): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn game_ui_open_screen(c: &mut Cpu) {
    if c.pc != 0x0020ab50 {
        return ee::s05::f_0020ab50__generated(c);
    }
    guest(c, |c| {
        'b_20abdc: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[T0][0] = 0u64;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = 0x1;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
            let t = c.g[S1][0] != c.g[V1][0];
            c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
            if t {
                break 'b_20abdc;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020ab9c);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020ab9c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020abac);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020abac)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3098));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                break 'b_20abdc;
            }
        }
        'b_20ac44: {
            c.cycles += 3;
            c.g[V0][0] = 0x2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x3;
            if t {
                break 'b_20ac44;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020ac00);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020ac00)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020ac10);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020ac10)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2fc0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x3;
                break 'b_20ac44;
            }
        }
        'b_20aca8: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x4;
            if t {
                break 'b_20aca8;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020ac64);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020ac64)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020ac74);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020ac74)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2ee8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x4;
                break 'b_20aca8;
            }
        }
        'b_20ad0c: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x5;
            if t {
                break 'b_20ad0c;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020acc8);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020acc8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020acd8);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020acd8)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2e10));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x5;
                break 'b_20ad0c;
            }
        }
        'b_20ad70: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x6;
            if t {
                break 'b_20ad70;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020ad2c);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020ad2c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020ad3c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020ad3c)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2d38));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x6;
                break 'b_20ad70;
            }
        }
        'b_20add4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x7;
            if t {
                break 'b_20add4;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020ad90);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020ad90)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020ada0);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020ada0)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2c60));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x7;
                break 'b_20add4;
            }
        }
        'b_20ae34: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x8;
            if t {
                break 'b_20ae34;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020adf4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020adf4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020ae04);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020ae04)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2b88));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x8;
                break 'b_20ae34;
            }
        }
        'b_20ae94: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x9;
            if t {
                break 'b_20ae94;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020ae54);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020ae54)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020ae64);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020ae64)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2850));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x9;
                break 'b_20ae94;
            }
        }
        'b_20aef4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xa;
            if t {
                break 'b_20aef4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020aeb4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020aeb4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020aec4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020aec4)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2778));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0xa;
                break 'b_20aef4;
            }
        }
        'b_20af54: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xb;
            if t {
                break 'b_20af54;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020af14);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020af14)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020af24);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020af24)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4088));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0xb;
                break 'b_20af54;
            }
        }
        'b_20afb4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xc;
            if t {
                break 'b_20afb4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020af74);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020af74)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020af84);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020af84)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3fb0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0xc;
                break 'b_20afb4;
            }
        }
        'b_20b02c: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xd;
            if t {
                break 'b_20b02c;
            } else {
                c.cycles += 28;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[RA][0] = sx(0x0020afd4);
                c.g[A0][0] = 0xa4;
                call_ptr(c, 0x00317d70, 0x0020afd4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020afe4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020afe4)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e00));
                c.g[V1][0] = 0x1;
                c.g[A0][0] = 0x470000;
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1cc0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020b020);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                call_ptr(c, 0x00317670, 0x0020b020)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.g[V0][0] = 0xd;
                break 'b_20b02c;
            }
        }
        'b_20b0a4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xe;
            if t {
                break 'b_20b0a4;
            } else {
                c.cycles += 28;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[RA][0] = sx(0x0020b04c);
                c.g[A0][0] = 0xa4;
                call_ptr(c, 0x00317d70, 0x0020b04c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b05c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b05c)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3d28));
                c.g[V1][0] = 0x1;
                c.g[A0][0] = 0x470000;
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1cc0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020b098);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                call_ptr(c, 0x00317670, 0x0020b098)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.g[V0][0] = 0xe;
                break 'b_20b0a4;
            }
        }
        'b_20b104: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xf;
            if t {
                break 'b_20b104;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b0c4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b0c4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b0d4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b0d4)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4b80));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0xf;
                break 'b_20b104;
            }
        }
        'b_20b164: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x10;
            if t {
                break 'b_20b164;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b124);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b124)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b134);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b134)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34d0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x10;
                break 'b_20b164;
            }
        }
        'b_20b1c4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x11;
            if t {
                break 'b_20b1c4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b184);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b184)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b194);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b194)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2108));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x11;
                break 'b_20b1c4;
            }
        }
        'b_20b224: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x12;
            if t {
                break 'b_20b224;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xb8;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b1e4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b1e4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b1f4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b1f4)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4aa8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x12;
                break 'b_20b224;
            }
        }
        'b_20b25c: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x13;
            if t {
                break 'b_20b25c;
            } else {
                c.cycles += 12;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x274;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b244);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b244)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b254);
                c.g[A2][0] = 0x12;
                Callee {
                    addr: 0x001fdbf0,
                    f: ee::s05::OVScreenSaveReplay_ctor,
                }
                .call(c, 0x0020b254)?;
                c.g[T0][0] = c.g[V0][0];
                c.g[V0][0] = 0x13;
                break 'b_20b25c;
            }
        }
        'b_20b294: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x14;
            if t {
                break 'b_20b294;
            } else {
                c.cycles += 12;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x274;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b27c);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b27c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b28c);
                c.g[A2][0] = 0x13;
                Callee {
                    addr: 0x001fe968,
                    f: ee::s05::OVScreenSaveRecords_ctor,
                }
                .call(c, 0x0020b28c)?;
                c.g[T0][0] = c.g[V0][0];
                c.g[V0][0] = 0x14;
                break 'b_20b294;
            }
        }
        'b_20b2f4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x15;
            if t {
                break 'b_20b2f4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b2b4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b2b4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b2c4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b2c4)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x25c8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x15;
                break 'b_20b2f4;
            }
        }
        'b_20b354: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x16;
            if t {
                break 'b_20b354;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b314);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b314)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b324);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b324)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x26a0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x16;
                break 'b_20b354;
            }
        }
        'b_20b3b4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x17;
            if t {
                break 'b_20b3b4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b374);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b374)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b384);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b384)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3ed8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x17;
                break 'b_20b3b4;
            }
        }
        'b_20b418: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x18;
            if t {
                break 'b_20b418;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b3d4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b3d4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b3e4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b3e4)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x49d0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), 0u64);
                c.g[V0][0] = 0x18;
                break 'b_20b418;
            }
        }
        'b_20b480: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x19;
            if t {
                break 'b_20b480;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b438);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b438)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b448);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b448)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x48f8));
                c.g[V1][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x19;
                break 'b_20b480;
            }
        }
        'b_20b4e8: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1a;
            if t {
                break 'b_20b4e8;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b4a0);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b4a0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b4b0);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b4b0)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4820));
                c.g[V1][0] = 0x2;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x1a;
                break 'b_20b4e8;
            }
        }
        'b_20b550: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1b;
            if t {
                break 'b_20b550;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b508);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b508)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b518);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b518)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4748));
                c.g[V1][0] = 0x3;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x1b;
                break 'b_20b550;
            }
        }
        'b_20b5b8: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1c;
            if t {
                break 'b_20b5b8;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b570);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b570)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b580);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b580)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4670));
                c.g[V1][0] = 0x4;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x1c;
                break 'b_20b5b8;
            }
        }
        'b_20b620: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1d;
            if t {
                break 'b_20b620;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b5d8);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b5d8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b5e8);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b5e8)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4598));
                c.g[V1][0] = 0x5;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x1d;
                break 'b_20b620;
            }
        }
        'b_20b688: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1e;
            if t {
                break 'b_20b688;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[RA][0] = sx(0x0020b640);
                c.g[A0][0] = 0xa0;
                call_ptr(c, 0x00317d70, 0x0020b640)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b650);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b650)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3688));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020b680);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                call_ptr(c, 0x001f7548, 0x0020b680)?;
                c.g[T0][0] = c.g[S0][0];
                c.g[V0][0] = 0x1e;
                break 'b_20b688;
            }
        }
        'b_20b6f0: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1f;
            if t {
                break 'b_20b6f0;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[RA][0] = sx(0x0020b6a8);
                c.g[A0][0] = 0xa4;
                call_ptr(c, 0x00317d70, 0x0020b6a8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b6b8);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b6b8)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35b0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020b6e8);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                call_ptr(c, 0x001f7738, 0x0020b6e8)?;
                c.g[T0][0] = c.g[S0][0];
                c.g[V0][0] = 0x1f;
                break 'b_20b6f0;
            }
        }
        'b_20b750: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x20;
            if t {
                break 'b_20b750;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b710);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b710)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b720);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b720)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3760));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x20;
                break 'b_20b750;
            }
        }
        'b_20b7cc: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
            if t {
                break 'b_20b7cc;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x1d4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b770);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b770)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b780);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b780)?;
                c.g[V0][0] = 0x4a0000;
                c.g[V1][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2088));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3320));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.g[A1][0] = 0xffffffffffffffff;
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[V0][0] = 0x7;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                'l_20b7b0: loop {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[V0][0] != c.g[A1][0];
                    c.g[T0][0] = c.g[S0][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0020b7b0);
                        }
                        continue 'l_20b7b0;
                    } else {
                        break 'b_20b7cc;
                    }
                }
            }
        }
        'b_20b838: {
            c.cycles += 3;
            c.g[V0][0] = 0x21;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x22;
            if t {
                break 'b_20b838;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[RA][0] = sx(0x0020b7f0);
                c.g[A0][0] = 0x390;
                call_ptr(c, 0x00317d70, 0x0020b7f0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b800);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b800)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x33f8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020b830);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                Callee {
                    addr: 0x002006b8,
                    f: ee::s05::EventSelect_ctor,
                }
                .call(c, 0x0020b830)?;
                c.g[T0][0] = c.g[S0][0];
                c.g[V0][0] = 0x22;
                break 'b_20b838;
            }
        }
        'b_20b870: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x23;
            if t {
                break 'b_20b870;
            } else {
                c.cycles += 12;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x180;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b858);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b858)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b868);
                c.g[A2][0] = 0x22;
                call_ptr(c, 0x001e3c00, 0x0020b868)?;
                c.g[T0][0] = c.g[V0][0];
                c.g[V0][0] = 0x23;
                break 'b_20b870;
            }
        }
        'b_20b8d0: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x24;
            if t {
                break 'b_20b8d0;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b890);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b890)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b8a0);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b8a0)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3c50));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x24;
                break 'b_20b8d0;
            }
        }
        'b_20b90c: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x25;
            if t {
                break 'b_20b90c;
            } else {
                c.cycles += 13;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x27c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b8f0);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b8f0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = 0x24;
                c.g[RA][0] = sx(0x0020b904);
                c.g[A3][0] = 0x2a;
                Callee {
                    addr: 0x00211088,
                    f: ee::s05::OVScreenProfileSavePDA_ctor,
                }
                .call(c, 0x0020b904)?;
                c.g[T0][0] = c.g[V0][0];
                c.g[V0][0] = 0x25;
                break 'b_20b90c;
            }
        }
        'b_20b944: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x26;
            if t {
                break 'b_20b944;
            } else {
                c.cycles += 12;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x234;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b92c);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b92c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b93c);
                c.g[A2][0] = 0x25;
                Callee {
                    addr: 0x00212138,
                    f: ee::s05::OVScreenAutosave_ctor,
                }
                .call(c, 0x0020b93c)?;
                c.g[T0][0] = c.g[V0][0];
                c.g[V0][0] = 0x26;
                break 'b_20b944;
            }
        }
        'b_20b9ac: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x27;
            if t {
                break 'b_20b9ac;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b964);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b964)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b974);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b974)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x44c0));
                c.g[V1][0] = 0x6;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x27;
                break 'b_20b9ac;
            }
        }
        'b_20ba18: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x28;
            if t {
                break 'b_20ba18;
            } else {
                c.cycles += 25;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020b9cc);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020b9cc)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020b9dc);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020b9dc)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2ab0));
                c.g[V1][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), 0u64);
                c.g[V0][0] = 0x28;
                break 'b_20ba18;
            }
        }
        'b_20ba78: {
            c.cycles += 2;
            if c.g[S1][0] != c.g[V0][0] {
                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x1c));
                break 'b_20ba78;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xe4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020ba38);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020ba38)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020ba48);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020ba48)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3248));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.g[T0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x1c));
                break 'b_20ba78;
            }
        }
        c.cycles += 16;
        c.g[V1][0] = 0xffffffffffffc0ff;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[T0][0];
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[A2][0] = 0u64;
        c.g[V0][0] = c.g[V0][0] | 0x100;
        c.g[A3][0] = 0u64;
        c.g[RA][0] = sx(0x0020ba9c);
        c.sw((c.g[T0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        call_ptr(c, 0x0039f320, 0x0020ba9c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{elf_image, fill, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    // Fixture objects, all in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    pub(crate) const OC804_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC804_MAGIC: u32 = 0x4f43_3834;
    pub(crate) const OC804_SELF: u32 = DATA + 0x21_0000;
    pub(crate) const OC804_NAME: u32 = DATA + 0x22_0000;
    pub(crate) const OC804_WORLD: u32 = DATA + 0x25_0000;
    pub(crate) const OC804_X: u32 = DATA + 0x26_0000;
    pub(crate) const OC804_Y: u32 = DATA + 0x27_0000;
    pub(crate) const OC804_Z: u32 = DATA + 0x28_0000;
    pub(crate) const OC804_IFACE: u32 = DATA + 0x29_0000;
    pub(crate) const OC804_IFVT: u32 = DATA + 0x2a_0000;
    pub(crate) const OC804_TBL: u32 = DATA + 0x2b_0000;
    pub(crate) const OC804_P: u32 = DATA + 0x2c_0000;
    pub(crate) const OC804_Q: u32 = DATA + 0x2d_0000;
    /// Per-callee forced results: one word per fake, at `OC804_FORCE + (addr & 0xffffc)`.
    pub(crate) const OC804_FORCE: u32 = DATA + 0x30_0000;

    /// The mailbox word of the fake for `addr`.
    pub(crate) fn oc804_mail(addr: u32) -> u32 {
        OC804_FORCE + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0`.
    pub(crate) fn oc804_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc804_mail(addr), v.wrapping_add(3));
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc804_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc804_mail(addr), 1);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox (0 = the fake's pointer) unless it unwound.
    pub(crate) fn oc804_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc804_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc804_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC804_CTRL) != OC804_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc804_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc804_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc804_fakes!(
        (
            oc804_fake_003e22f0,
            OC804_PREV_003E22F0,
            ee::s14::OVERRIDE_003E22F0,
            ee::s14::f_003e22f0__generated,
            0x003e22f0
        ),
        (
            oc804_fake_0026adf0,
            OC804_PREV_0026ADF0,
            ee::s07::OVERRIDE_0026ADF0,
            ee::s07::f_0026adf0__generated,
            0x0026adf0
        ),
        (
            oc804_fake_0012b180,
            OC804_PREV_0012B180,
            ee::s00::OVERRIDE_0012B180,
            ee::s00::f_0012b180__generated,
            0x0012b180
        ),
        (
            oc804_fake_00144df0,
            OC804_PREV_00144DF0,
            ee::s01::OVERRIDE_00144DF0,
            ee::s01::f_00144df0__generated,
            0x00144df0
        ),
        (
            oc804_fake_001451e8,
            OC804_PREV_001451E8,
            ee::s01::OVERRIDE_001451E8,
            ee::s01::f_001451e8__generated,
            0x001451e8
        ),
        (
            oc804_fake_0014dc80,
            OC804_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc804_fake_0014dd58,
            OC804_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc804_fake_0014de68,
            OC804_PREV_0014DE68,
            ee::s01::OVERRIDE_0014DE68,
            ee::s01::f_0014de68__generated,
            0x0014de68
        ),
        (
            oc804_fake_00231250,
            OC804_PREV_00231250,
            ee::s06::OVERRIDE_00231250,
            ee::s06::f_00231250__generated,
            0x00231250
        ),
        (
            oc804_fake_00238160,
            OC804_PREV_00238160,
            ee::s06::OVERRIDE_00238160,
            ee::s06::f_00238160__generated,
            0x00238160
        ),
        (
            oc804_fake_00146e98,
            OC804_PREV_00146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x00146e98
        ),
        (
            oc804_fake_00148410,
            OC804_PREV_00148410,
            ee::s01::OVERRIDE_00148410,
            ee::s01::f_00148410__generated,
            0x00148410
        ),
        (
            oc804_fake_001484e0,
            OC804_PREV_001484E0,
            ee::s01::OVERRIDE_001484E0,
            ee::s01::f_001484e0__generated,
            0x001484e0
        ),
        (
            oc804_fake_001485b0,
            OC804_PREV_001485B0,
            ee::s01::OVERRIDE_001485B0,
            ee::s01::f_001485b0__generated,
            0x001485b0
        ),
        (
            oc804_fake_00148680,
            OC804_PREV_00148680,
            ee::s01::OVERRIDE_00148680,
            ee::s01::f_00148680__generated,
            0x00148680
        ),
        (
            oc804_fake_00148750,
            OC804_PREV_00148750,
            ee::s01::OVERRIDE_00148750,
            ee::s01::f_00148750__generated,
            0x00148750
        ),
        (
            oc804_fake_00148820,
            OC804_PREV_00148820,
            ee::s01::OVERRIDE_00148820,
            ee::s01::f_00148820__generated,
            0x00148820
        ),
        (
            oc804_fake_001488f0,
            OC804_PREV_001488F0,
            ee::s01::OVERRIDE_001488F0,
            ee::s01::f_001488f0__generated,
            0x001488f0
        ),
        (
            oc804_fake_00148950,
            OC804_PREV_00148950,
            ee::s01::OVERRIDE_00148950,
            ee::s01::f_00148950__generated,
            0x00148950
        ),
        (
            oc804_fake_002c26d0,
            OC804_PREV_002C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c26d0
        ),
        (
            oc804_fake_00317670,
            OC804_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc804_fake_0039c428,
            OC804_PREV_0039C428,
            ee::s13::OVERRIDE_0039C428,
            ee::s13::f_0039c428__generated,
            0x0039c428
        ),
        (
            oc804_fake_0039d860,
            OC804_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039d860
        ),
        (
            oc804_fake_003a0c50,
            OC804_PREV_003A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a0c50
        ),
        (
            oc804_fake_003a0e90,
            OC804_PREV_003A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a0e90
        ),
        (
            oc804_fake_00417828,
            OC804_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x00417828
        ),
        (
            oc804_fake_001e3c00,
            OC804_PREV_001E3C00,
            ee::s04::OVERRIDE_001E3C00,
            ee::s04::f_001e3c00__generated,
            0x001e3c00
        ),
        (
            oc804_fake_001f7548,
            OC804_PREV_001F7548,
            ee::s04::OVERRIDE_001F7548,
            ee::s04::f_001f7548__generated,
            0x001f7548
        ),
        (
            oc804_fake_001f7738,
            OC804_PREV_001F7738,
            ee::s04::OVERRIDE_001F7738,
            ee::s04::f_001f7738__generated,
            0x001f7738
        ),
        (
            oc804_fake_00317d70,
            OC804_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x00317d70
        ),
        (
            oc804_fake_0039e2a0,
            OC804_PREV_0039E2A0,
            ee::s13::OVERRIDE_0039E2A0,
            ee::s13::f_0039e2a0__generated,
            0x0039e2a0
        ),
        (
            oc804_fake_0039f320,
            OC804_PREV_0039F320,
            ee::s13::OVERRIDE_0039F320,
            ee::s13::f_0039f320__generated,
            0x0039f320
        ),
        (
            oc804_fake_001fdbf0,
            OC804_PREV_001FDBF0,
            ee::s05::OVERRIDE_001FDBF0,
            ee::s05::f_001fdbf0__generated,
            0x001fdbf0
        ),
        (
            oc804_fake_001fe968,
            OC804_PREV_001FE968,
            ee::s05::OVERRIDE_001FE968,
            ee::s05::f_001fe968__generated,
            0x001fe968
        ),
        (
            oc804_fake_002006b8,
            OC804_PREV_002006B8,
            ee::s05::OVERRIDE_002006B8,
            ee::s05::f_002006b8__generated,
            0x002006b8
        ),
        (
            oc804_fake_00212138,
            OC804_PREV_00212138,
            ee::s05::OVERRIDE_00212138,
            ee::s05::f_00212138__generated,
            0x00212138
        ),
    );

    /// Scrambled registers, fake hooks, the fakes' gate, stack junk below `sp`
    /// (half of its words 0), the world at `gp-0x848`.
    pub(crate) fn oc804_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(OC804_CTRL, OC804_MAGIC);
        for i in 1..=0x300u32 {
            let w = if r.next_u32() % 2 == 0 {
                0
            } else {
                r.next_u32()
            };
            c.bus.write32(crate::difftest::SP - 4 * i, w);
        }
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC804_WORLD);
        r
    }

    /// `PathData_LoadFile(path, name)`: loads a file through the EA thunk,
    /// stores the buffer and a "loaded" flag, then parses it.
    #[test]
    fn path_data_load_file_matches() {
        oc804_install_fakes();
        fakes::covered(
            ee::s07::f_0026ada8__generated,
            path_data_load_file,
            60,
            &|c, seed| {
                let mut r = oc804_fixture(c, seed, 0x0026_ada8, 2);
                fill(c, &mut r, OC804_SELF, 0x30);
                c.g[A0][0] = sx(OC804_SELF);
                c.g[A1][0] = sx(OC804_NAME);
                c.pc = 0x0026_ada8;
            },
        );
    }

    /// `SectionMan_OnEventTrigger(this, mode)`: configures the race back end
    /// for a mode (the jump table of modes 0 to 6), then calls a virtual method
    /// of the interface and starts the game mode. The early exits come from the
    /// global flags (`0x535bc8`: bytes 0x48/0x49), `this+0x1bc` and mode 1.
    #[test]
    fn section_man_on_event_trigger_matches() {
        oc804_install_fakes();
        fakes::covered(
            ee::s05::f_0022d6c8__generated,
            section_man_on_event_trigger,
            220,
            &|c, seed| {
                elf_image(c);
                let mut r = oc804_fixture(c, seed, 0x0022_d6c8, 10);
                let counts = [0xdu32, 0xe, 0xf, 0x10, 0x11, 0, 0x1c];
                c.bus.write8(0x0053_5bc8 + 0x49, (seed % 7 == 0) as u8);
                c.bus
                    .write8(0x0053_5bc8 + 0x48, if seed % 5 == 0 { 3 } else { 4 });
                fill(c, &mut r, OC804_SELF, 0x80);
                c.bus
                    .write32(OC804_SELF + 0x1bc, counts[(r.next_u32() % 7) as usize]);
                c.bus.write32(OC804_SELF + 0x1b0, r.next_u32());
                // The world: world+0x84 = X; X+0xc = Y, X+0x78 = Z, X+0x214 = 4.
                fill(c, &mut r, OC804_X, 0x100);
                fill(c, &mut r, OC804_Y, 8);
                fill(c, &mut r, OC804_WORLD, 0x40);
                c.bus.write32(OC804_WORLD + 0x84, OC804_X);
                c.bus.write32(OC804_X + 0xc, OC804_Y);
                c.bus.write32(OC804_X + 0x78, OC804_Z);
                c.bus
                    .write32(OC804_X + 0x214, if seed % 4 == 3 { 5 } else { 4 });
                c.bus
                    .write32(OC804_Z + 0x1bc, counts[(r.next_u32() % 7) as usize]);
                // The interface the back-end calls go through: its method.
                c.bus.write32(OC804_IFACE + 0xc, OC804_IFVT);
                c.bus.write16(OC804_IFVT + 8, 0x10 * (seed % 3) as u16);
                c.bus.write32(OC804_IFVT + 0xc, fakes::FAKE + 0x20);
                oc804_force(c, 0x0014_dd58, OC804_IFACE);
                c.g[A0][0] = sx(OC804_SELF);
                c.g[A1][0] = (seed % 9) as u64;
                c.pc = 0x0022_d6c8;
            },
        );
    }

    /// `cGameLoadState_UpdatePlayerRanking(this, player)`: fills the ranking
    /// tables of the loading screen from the back-end interfaces and the stat
    /// tables of the player (`0x5308b8`), through the real loops, ending with
    /// the load-state object's texture objects.
    #[test]
    fn game_load_state_update_player_ranking_matches() {
        oc804_install_fakes();
        fakes::covered(
            ee::s06::f_00246cf8__generated,
            game_load_state_update_player_ranking,
            260,
            &|c, seed| {
                elf_image(c);
                let mut r = oc804_fixture(c, seed, 0x0024_6cf8, 40);
                c.bus.write8(0x0053_5c11, if seed % 9 == 0 { 1 } else { 2 });
                c.bus.write32(GAME_GP.wrapping_sub(0x238), seed % 2);
                for i in 0..0x48u32 {
                    let w = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(0x0053_08b8 + 4 * i, w);
                }
                c.bus
                    .write32(0x0053_4b30 + 4, if seed % 3 == 0 { 0 } else { 1 });
                for i in 0..8u32 {
                    c.bus.write32(0x0053_4b30 + 4 * i + 4, r.next_u32() % 2);
                }
                fill(c, &mut r, OC804_SELF, 0x20);
                c.bus
                    .write32(OC804_SELF + 0x48, if seed % 7 == 0 { 0 } else { OC804_TBL });
                for i in 0..0x20u32 {
                    c.bus.write32(OC804_TBL + 4 * i, r.next_u32() % 3);
                }
                c.bus.write32(OC804_WORLD + 0x78, OC804_Z);
                c.bus.write32(OC804_WORLD + 0x8c, OC804_P);
                c.bus.write32(OC804_P + 4, OC804_Q);
                c.bus.write16(OC804_Q + 0x20, 0);
                c.bus.write32(OC804_Q + 0x24, fakes::FAKE + 0x30);
                for i in 0..7u32 {
                    c.bus.write8(OC804_Z + 0x4a8 + i, (r.next_u32() % 4) as u8);
                }
                oc804_force(c, 0x0014_6e98, seed % 6);
                if seed % 4 == 0 {
                    oc804_force_zero(c, 0x0039_d860);
                }
                if seed % 5 == 1 {
                    oc804_force_zero(c, 0x0014_8950);
                }
                c.g[A0][0] = sx(OC804_SELF);
                c.g[A1][0] = [0u64, 1, 2, 3, 0xff][(seed % 5) as usize];
                c.pc = 0x0024_6cf8;
            },
        );
    }

    /// `GameUI_OpenScreen(id)`: builds the front-end screen object of `id`
    /// (an allocation, the base constructor, the vtable and the id stored, a
    /// few ids call their own constructor) and opens it. Id 0x23's
    /// constructor (`OVScreenProfileSavePDA_ctor`, no override slot) stays
    /// out: the real one would run on fixture data.
    #[test]
    fn game_ui_open_screen_matches() {
        oc804_install_fakes();
        fakes::covered(
            ee::s05::f_0020ab50__generated,
            game_ui_open_screen,
            172,
            &|c, seed| {
                let mut r = oc804_fixture(c, seed, 0x0020_ab50, 4);
                let id = match seed % 0x2b {
                    0x23 => 0x50,
                    k => k,
                };
                fill(c, &mut r, OC804_X, 0x40);
                c.bus.write32(OC804_WORLD + 0x84, OC804_X);
                c.bus.write32(OC804_X + 0x48, r.next_u32());
                c.g[A0][0] = id as u64;
                c.pc = 0x0020_ab50;
            },
        );
    }
}
