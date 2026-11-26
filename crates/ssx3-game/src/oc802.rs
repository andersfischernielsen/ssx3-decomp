//! Staging module oc802 (wave wave247): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc802`.

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
    (0x001a2e58, fe_ui_sound_v02, "FEUISound_v02"),
    (0x001e92a8, hud_layout_icon_text, "HUD_LayoutIconText"),
    (0x001fd190, fe_get_event_name_text, "FE_GetEventNameText"),
];

// `FEModule_Load` (0x001a1ec0) is skipped: already ported (not installed) in
// game/sound/streamsys.rs, whose test runs away in unslotted callees.

/// `FEUISound_v02` (0x001a2e58), `cFESound_playSound(self, sel)`: plays the front-end menu
/// sound `sel` (1..=6, a jump table) through the audio system. The normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn fe_ui_sound_v02(c: &mut Cpu) {
    if c.pc != 0x001a2e58 {
        return ee::s03::f_001a2e58__generated(c);
    }
    guest(c, |c| 'l_1a2e58: loop {
        'b_1a2f2c: {
            'b_1a2f28: {
                c.cycles += 5;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_1a2f28;
                } else {
                    'b_1a2f14: {
                        'b_1a2ef8: {
                            'b_1a2edc: {
                                'b_1a2ec0: {
                                    'b_1a2ea4: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x460000;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1380));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x001a2e58 => {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x001a2e58);
                                                }
                                                continue 'l_1a2e58;
                                            }
                                            0x001a2e88 => {
                                                c.cycles += 7;
                                                c.g[RA][0] = sx(0x001a2e90);
                                                call_ptr(c, 0x0028b180, 0x001a2e90)?;
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[RA][0] = sx(0x001a2e9c);
                                                c.g[A1][0] = 0x2;
                                                call_ptr(c, 0x00294f78, 0x001a2e9c)?;
                                                let t = true;
                                                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                                if t {
                                                    break 'b_1a2f2c;
                                                } else {
                                                    break 'b_1a2ea4;
                                                }
                                            }
                                            0x001a2ea4 => {
                                                break 'b_1a2ea4;
                                            }
                                            0x001a2ec0 => {
                                                break 'b_1a2ec0;
                                            }
                                            0x001a2edc => {
                                                break 'b_1a2edc;
                                            }
                                            0x001a2ef8 => {
                                                break 'b_1a2ef8;
                                            }
                                            0x001a2f14 => {
                                                break 'b_1a2f14;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x001a2e58, 0x001a2f34);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[RA][0] = sx(0x001a2eac);
                                    call_ptr(c, 0x0028b180, 0x001a2eac)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001a2eb8);
                                    c.g[A1][0] = 0x3;
                                    call_ptr(c, 0x00294f78, 0x001a2eb8)?;
                                    let t = true;
                                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                    if t {
                                        break 'b_1a2f2c;
                                    } else {
                                        break 'b_1a2ec0;
                                    }
                                }
                                c.cycles += 7;
                                c.g[RA][0] = sx(0x001a2ec8);
                                call_ptr(c, 0x0028b180, 0x001a2ec8)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001a2ed4);
                                c.g[A1][0] = 0x1;
                                call_ptr(c, 0x00294f78, 0x001a2ed4)?;
                                let t = true;
                                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                if t {
                                    break 'b_1a2f2c;
                                } else {
                                    break 'b_1a2edc;
                                }
                            }
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x001a2ee4);
                            call_ptr(c, 0x0028b180, 0x001a2ee4)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001a2ef0);
                            c.g[A1][0] = 0x4;
                            call_ptr(c, 0x00294f78, 0x001a2ef0)?;
                            let t = true;
                            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                            if t {
                                break 'b_1a2f2c;
                            } else {
                                break 'b_1a2ef8;
                            }
                        }
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x001a2f00);
                        call_ptr(c, 0x0028b180, 0x001a2f00)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001a2f0c);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x00294f78, 0x001a2f0c)?;
                        let t = true;
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        if t {
                            break 'b_1a2f2c;
                        } else {
                            break 'b_1a2f14;
                        }
                    }
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x001a2f1c);
                    call_ptr(c, 0x0028b180, 0x001a2f1c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a2f28);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00294f78, 0x001a2f28)?;
                    break 'b_1a2f28;
                }
            }
            c.cycles += 1;
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            break 'b_1a2f2c;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `HUD_LayoutIconText(self)` (0x001e92a8): lays out a HUD string with `@icon` tokens: measures
/// the text runs and looks icons up by name hash. The normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn hud_layout_icon_text(c: &mut Cpu) {
    if c.pc != 0x001e92a8 {
        return ee::s04::f_001e92a8__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x001e92a8;
        loop {
            match pc {
                0x001e92a8 => {
                    c.cycles += 32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4c));
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[S4][0] = 0u64;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1c));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S0][0] = c.g[V1][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xb8));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x10));
                    c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x30));
                    c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x34));
                    c.mul_s(0, 0, 1);
                    c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x38));
                    c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x14));
                    c.mul_s(2, 2, 1);
                    c.swc1(2, (c.g[S2][0] as u32).wrapping_add(0x3c));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1c), 0u64);
                    c.g[V0][0] = c.lhu((c.g[S3][0] as u32).wrapping_add(0x4c));
                    c.lwc1(21, (c.g[S3][0] as u32).wrapping_add(0x1c));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x18), 0u64);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                    let t = c.g[V0][0] == 0u64;
                    c.mov_s(20, 21);
                    if t {
                        pc = 0x001e943c;
                        continue;
                    }
                    pc = 0x001e9328;
                    continue;
                }
                0x001e9328 => {
                    c.cycles += 2;
                    c.g[S6][0] = 0x4d0000;
                    c.g[V1][0] = c.lhu(c.g[S0][0] as u32);
                    pc = 0x001e9330;
                    continue;
                }
                0x001e9330 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x40;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
                        pc = 0x001e9430;
                        continue;
                    }
                    pc = 0x001e933c;
                    continue;
                }
                0x001e933c => {
                    c.cycles += 7;
                    c.sh(c.g[S0][0] as u32, 0u64);
                    c.g[V0][0] = sx((c.g[S4][0] as u32) << 3);
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lhu(c.g[A1][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
                    if t {
                        pc = 0x001e9378;
                        continue;
                    }
                    pc = 0x001e9358;
                    continue;
                }
                0x001e9358 => {
                    c.cycles += 8;
                    c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0x38));
                    c.g[A0][0] = c.g[S2][0];
                    c.lwc1(13, (c.g[S2][0] as u32).wrapping_add(0x3c));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001e9370);
                    c.g[A3][0] = 0x1;
                    call_ptr(c, 0x003921f0, 0x001e9370)?;
                    c.add_s(20, 20, 0);
                    c.swc1(0, c.g[S1][0] as u32);
                    pc = 0x001e9378;
                    continue;
                }
                0x001e9378 => {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = c.g[A0][0];
                    c.g[A1][0] = 0x20;
                    pc = 0x001e9388;
                    continue;
                }
                0x001e9388 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lhu(c.g[S0][0] as u32);
                    let t = c.g[V0][0] == c.g[A1][0];

                    if t {
                        pc = 0x001e93b0;
                        continue;
                    }
                    pc = 0x001e9394;
                    continue;
                }
                0x001e9394 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        pc = 0x001e93b0;
                        continue;
                    }
                    pc = 0x001e939c;
                    continue;
                }
                0x001e939c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
                    c.sb(c.g[V1][0] as u32, c.g[V0][0]);
                    let t = true;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x001e9388) {
                            return Ok(());
                        }
                        pc = 0x001e9388;
                        continue;
                    }
                    pc = 0x001e93b0;
                    continue;
                }
                0x001e93b0 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x001e93b8);
                    c.sb(c.g[V1][0] as u32, 0u64);
                    call_ptr(c, 0x00317670, 0x001e93b8)?;
                    c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_sub(0x7680));
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
                    c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = 0u64;
                    let t = true;
                    c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x7680));
                    if t {
                        pc = 0x001e93d4;
                        continue;
                    }
                    pc = 0x001e93d0;
                    continue;
                }
                0x001e93d0 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                    pc = 0x001e93d4;
                    continue;
                }
                0x001e93d4 => {
                    c.cycles += 2;

                    if c.g[V1][0] == 0u64 {
                        c.g[V0][0] = sx((c.g[S4][0] as u32) << 3);
                        pc = 0x001e93ec;
                        continue;
                    }
                    pc = 0x001e93dc;
                    continue;
                }
                0x001e93dc => {
                    c.cycles += 2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24));
                        if back_edge_unwinds(c, 0x001e93d0) {
                            return Ok(());
                        }
                        pc = 0x001e93d0;
                        continue;
                    }
                    pc = 0x001e93e4;
                    continue;
                }
                0x001e93e4 => {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[A0][0];
                    c.g[V0][0] = sx((c.g[S4][0] as u32) << 3);
                    pc = 0x001e93ec;
                    continue;
                }
                0x001e93ec => {
                    c.cycles += 10;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                    c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0xc));
                    c.swc1(0, c.g[V0][0] as u32);
                    c.add_s(20, 20, 0);
                    c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x10));
                    c.c_lt_s(21, 0);
                    let t = !c.fcc();
                    c.g[S4][0] = c.g[A2][0];
                    if t {
                        pc = 0x001e9418;
                        continue;
                    }
                    pc = 0x001e9414;
                    continue;
                }
                0x001e9414 => {
                    c.cycles += 1;
                    c.mov_s(21, 0);
                    pc = 0x001e9418;
                    continue;
                }
                0x001e9418 => {
                    c.cycles += 6;
                    c.g[V0][0] = sx((c.g[S4][0] as u32) << 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[V1][0] = c.lhu(c.g[S0][0] as u32);
                    if c.g[V1][0] != 0 {
                        c.g[S0][0] = c.g[A3][0];
                    }
                    pc = 0x001e9430;
                    continue;
                }
                0x001e9430 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lhu(c.g[S0][0] as u32);

                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lhu(c.g[S0][0] as u32);
                        if back_edge_unwinds(c, 0x001e9330) {
                            return Ok(());
                        }
                        pc = 0x001e9330;
                        continue;
                    }
                    pc = 0x001e943c;
                    continue;
                }
                0x001e943c => {
                    c.cycles += 6;
                    c.g[V0][0] = sx((c.g[S4][0] as u32) << 3);
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lhu(c.g[A1][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        pc = 0x001e9474;
                        continue;
                    }
                    pc = 0x001e9454;
                    continue;
                }
                0x001e9454 => {
                    c.cycles += 8;
                    c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0x38));
                    c.lwc1(13, (c.g[S2][0] as u32).wrapping_add(0x3c));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x001e946c);
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x003921f0, 0x001e946c)?;
                    c.add_s(20, 20, 0);
                    c.swc1(0, c.g[S0][0] as u32);
                    pc = 0x001e9474;
                    continue;
                }
                0x001e9474 => {
                    c.cycles += 27;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[S4][0]);
                    c.lwc1(2, (c.g[S3][0] as u32).wrapping_add(0x14));
                    c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
                    c.cvt_s_w(0, 0);
                    c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x34));
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 3);
                    c.swc1(20, c.g[S3][0] as u32);
                    c.mul_s(0, 0, 1);
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.mul_s(0, 0, 2);
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.max_s(1, 0, 21);
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sub_s(0, 1, 0);
                    c.swc1(1, (c.g[S3][0] as u32).wrapping_add(0x4));
                    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xb8));
                    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.mul_s(0, 0, 3);
                    c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0xc));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001e92a8, 0x001e94e0);
                    return Ok(());
                }
            }
        }
    });
}

/// `FE_GetEventNameText(kind)` (0x001fd190): the pause-screen text of an event kind 6..=11
/// (jump table): hashes a name through the world's string table. The normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn fe_get_event_name_text(c: &mut Cpu) {
    if c.pc != 0x001fd190 {
        return ee::s04::f_001fd190__generated(c);
    }
    guest(c, |c| {
        'b_1fd258: {
            'b_1fd250: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[V0][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_1fd250;
                } else {
                    'b_1fd224: {
                        'b_1fd218: {
                            'b_1fd208: {
                                'b_1fd1f8: {
                                    'b_1fd1e8: {
                                        'b_1fd1d8: {
                                            c.cycles += 7;
                                            c.g[V0][0] = 0x470000;
                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc0));
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                            let t = c.g[A0][0] as u32;
                                            match t {
                                                0x001fd1c8 => {
                                                    c.cycles += 4;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    c.g[A0][0] = 0x470000;
                                                    let t = true;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_sub(0x130));
                                                    if t {
                                                        break 'b_1fd224;
                                                    } else {
                                                        break 'b_1fd1d8;
                                                    }
                                                }
                                                0x001fd1d8 => {
                                                    break 'b_1fd1d8;
                                                }
                                                0x001fd1e8 => {
                                                    break 'b_1fd1e8;
                                                }
                                                0x001fd1f8 => {
                                                    break 'b_1fd1f8;
                                                }
                                                0x001fd208 => {
                                                    break 'b_1fd208;
                                                }
                                                0x001fd218 => {
                                                    break 'b_1fd218;
                                                }
                                                _ => {
                                                    c.jump_out(t, 0x001fd190, 0x001fd268);
                                                    return Ok(());
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.g[A0][0] = 0x470000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x120));
                                        if t {
                                            break 'b_1fd224;
                                        } else {
                                            break 'b_1fd1e8;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[A0][0] = 0x470000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x110));
                                    if t {
                                        break 'b_1fd224;
                                    } else {
                                        break 'b_1fd1f8;
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A0][0] = 0x470000;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xf8));
                                if t {
                                    break 'b_1fd224;
                                } else {
                                    break 'b_1fd208;
                                }
                            }
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A0][0] = 0x470000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xe8));
                            if t {
                                break 'b_1fd224;
                            } else {
                                break 'b_1fd218;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xd8));
                        break 'b_1fd224;
                    }
                    c.cycles += 11;
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x001fd238);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x001fd238)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001fd248);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x001fd248)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_1fd258;
                    } else {
                        break 'b_1fd250;
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1fd258;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{fakes, fill, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the fixtures (a fake runs only while it holds `OC802_MAGIC`).
    const OC802_CTRL: u32 = DATA + 0x20_0000;
    const OC802_MAGIC: u32 = 0x4f43_3830;
    /// Forced `StrHash` result (0 = the fake's pointer).
    const OC802_HASH: u32 = OC802_CTRL + 4;

    static OC802_PREV_28B180: AtomicUsize = AtomicUsize::new(0);
    static OC802_PREV_294F78: AtomicUsize = AtomicUsize::new(0);
    static OC802_PREV_317670: AtomicUsize = AtomicUsize::new(0);
    static OC802_PREV_3921F0: AtomicUsize = AtomicUsize::new(0);

    fn oc802_fake_28b180(c: &mut Cpu) {
        if c.bus.read32(OC802_CTRL) != OC802_MAGIC {
            return fakes::fall_back(c, &OC802_PREV_28B180, ee::s07::f_0028b180__generated);
        }
        fakes::fake_call(c, 0x0028_b180);
    }
    fn oc802_fake_294f78(c: &mut Cpu) {
        if c.bus.read32(OC802_CTRL) != OC802_MAGIC {
            return fakes::fall_back(c, &OC802_PREV_294F78, ee::s08::f_00294f78__generated);
        }
        fakes::fake_call(c, 0x0029_4f78);
    }
    fn oc802_fake_317670(c: &mut Cpu) {
        if c.bus.read32(OC802_CTRL) != OC802_MAGIC {
            return fakes::fall_back(c, &OC802_PREV_317670, ee::s10::f_00317670__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, 0x0031_7670);
        let h = c.bus.read32(OC802_HASH);
        if c.pc == ra && h != 0 {
            c.g[V0][0] = h as u64;
        }
    }
    fn oc802_fake_3921f0(c: &mut Cpu) {
        if c.bus.read32(OC802_CTRL) != OC802_MAGIC {
            return fakes::fall_back(c, &OC802_PREV_3921F0, ee::s12::f_003921f0__generated);
        }
        fakes::fake_call(c, 0x0039_21f0);
    }

    fn oc802_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028B180),
            oc802_fake_28b180,
            &OC802_PREV_28B180,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_00294F78),
            oc802_fake_294f78,
            &OC802_PREV_294F78,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc802_fake_317670,
            &OC802_PREV_317670,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_003921F0),
            oc802_fake_3921f0,
            &OC802_PREV_3921F0,
        );
    }

    /// `FEUISound_v02`: `sel` 0..=8 (0 and 7, 8 skip the table), a table entry
    /// that is off the function every 9th seed (the `jump_out` arm).
    #[test]
    fn fe_ui_sound_v02_matches() {
        oc802_install_fakes();
        fakes::covered(
            ee::s03::f_001a2e58__generated,
            fe_ui_sound_v02,
            90,
            &|c, seed| {
                let _ = fakes::machine(c, seed, 0x802a, 3);
                c.bus.write32(OC802_CTRL, OC802_MAGIC);
                let table = [
                    0x001a_2e88u32,
                    0x001a_2ea4,
                    0x001a_2ec0,
                    0x001a_2edc,
                    0x001a_2ef8,
                    0x001a_2f14,
                ];
                for (i, t) in table.into_iter().enumerate() {
                    c.bus.write32(0x0046_1380 + 4 * i as u32, t);
                }
                if seed % 9 == 8 {
                    c.bus.write32(0x0046_1380 + 4 * (seed / 9 % 6), 0x0012_3450);
                }
                c.g[A0][0] = sx(DATA + 0x21_0000);
                c.g[A1][0] = (seed % 9) as u64;
                c.pc = 0x001a_2e58;
            },
        );
    }

    /// `FE_GetEventNameText`: kinds 5..=12 (6..=11 through the table), the
    /// string-table lookup through the world's object.
    #[test]
    fn fe_get_event_name_text_matches() {
        oc802_install_fakes();
        fakes::covered(
            ee::s04::f_001fd190__generated,
            fe_get_event_name_text,
            80,
            &|c, seed| {
                let _ = fakes::machine(c, seed, 0x802b, 4);
                c.bus.write32(OC802_CTRL, OC802_MAGIC);
                let table = [
                    0x001f_d1c8u32,
                    0x001f_d1d8,
                    0x001f_d1e8,
                    0x001f_d1f8,
                    0x001f_d208,
                    0x001f_d218,
                ];
                for (i, t) in table.into_iter().enumerate() {
                    c.bus.write32(0x0046_ff40 + 4 * i as u32, t);
                }
                if seed % 11 == 10 {
                    c.bus
                        .write32(0x0046_ff40 + 4 * (seed / 11 % 6), 0x0012_3450);
                }
                let world = DATA + 0x25_0000;
                let obj = DATA + 0x26_0000;
                let inner = DATA + 0x27_0000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x8c, obj);
                c.bus.write32(obj + 4, inner);
                c.bus.write16(inner + 0x20, (seed * 8) as u16);
                c.bus.write32(inner + 0x24, fakes::FAKE);
                c.g[A0][0] = (5 + seed % 8) as u64;
                c.pc = 0x001f_d190;
            },
        );
    }

    /// `HUD_LayoutIconText`: random strings of `@`, space and letters (up to
    /// two `@` tokens), the hash table with and without a hit.
    #[test]
    fn hud_layout_icon_text_matches() {
        oc802_install_fakes();
        fakes::covered(
            ee::s04::f_001e92a8__generated,
            hud_layout_icon_text,
            400,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x802c, 8);
                c.bus.write32(OC802_CTRL, OC802_MAGIC);
                let me = DATA + 0x21_0000;
                let font = DATA + 0x22_0000;
                fill(c, &mut r, me, 0x20);
                fill(c, &mut r, font, 0x20);
                let floats = [0x3f80_0000u32, 0x4000_0000, 0x3f00_0000, 0x40a0_0000, 0];
                for off in [0x30u32, 0x34] {
                    c.bus
                        .write32(font + off, floats[(r.next_u32() % 5) as usize]);
                }
                c.bus.write32(font + 0x14, r.next_u32() % 40);
                c.bus.write32(me + 0x8, font);
                c.bus
                    .write32(me + 0x10, floats[(r.next_u32() % 5) as usize]);
                c.bus
                    .write32(me + 0x14, floats[(r.next_u32() % 5) as usize]);
                // The string at +0x4c: up to 9 units of '@', ' ', 'a', ended by 0
                // (at most two '@': the entries at +0x1c stay clear of the string).
                let mut ats = 0;
                let len = if seed % 7 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 9
                };
                for i in 0..10u32 {
                    let mut u = 0u32;
                    if i < len {
                        u = [0x40u32, 0x20, 0x61, 0x62][(r.next_u32() % 4) as usize];
                        if i == 0 && seed % 3 == 0 {
                            u = 0x40;
                        }
                        if u == 0x40 {
                            ats += 1;
                            if ats > 2 {
                                u = 0x61;
                            }
                        }
                    }
                    c.bus.write16(me + 0x4c + 2 * i, u as u16);
                }
                // The icon table: stride 0x24, hash, float at +0xc and +0x10.
                let tab = 0x004c_8980u32;
                for (i, h) in [0x2222u32, 0x1111, 0].into_iter().enumerate() {
                    let e = tab + 0x24 * i as u32;
                    c.bus.write32(e, h);
                    c.bus.write32(e + 0xc, floats[(r.next_u32() % 5) as usize]);
                    c.bus.write32(e + 0x10, floats[(r.next_u32() % 5) as usize]);
                }
                c.bus
                    .write32(OC802_HASH, [0u32, 0x1111, 0x2222][(seed % 3) as usize]);
                c.g[A0][0] = sx(me);
                c.pc = 0x001e_92a8;
            },
        );
    }
}
