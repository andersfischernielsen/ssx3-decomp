//! Staging module oc819 (wave wave248): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc819`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0017_e5c8, lobby_menu_render, "LobbyMenu_Render")];

/// `LobbyMenu_Render` (0x0017e5c8, cDebugLobbyScreen_draw): the debug lobby screen.
/// Draws the status text of the lobby state (jump table on `self.mState`, 0..24:
/// centred 'Connecting...' captions, error texts through `LobbyMan_GetErrorString`,
/// the 'Play Game' table of game name, opponent, host flag and addresses), then
/// the menu (`MenuSys_Draw`) and, with the comm system in net state 5, the local
/// address. The normal entry, converted from the generated labelled-block body
/// (every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (`__entries` now; at install switch to its `__generated` alias).
pub fn lobby_menu_render(c: &mut Cpu) {
    if c.pc != 0x0017e5c8 {
        return ee::s02::f_0017e5c8__entries(c);
    }
    guest(c, |c| 'l_17e5c8: loop {
        'b_17f1a8: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x78));
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x68));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x80));
            c.g[A0][0] = (c.g[A1][0] < 0x19u64) as u64;
            let t = c.g[A0][0] == 0u64;
            c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
            if t {
                break 'b_17f1a8;
            } else {
                'b_17ef20: {
                    'b_17eee8: {
                        'b_17ee24: {
                            'b_17ee14: {
                                'b_17ed3c: {
                                    'b_17ec64: {
                                        'b_17eb8c: {
                                            'b_17eb78: {
                                                'b_17eaa0: {
                                                    'b_17e9c8: {
                                                        'b_17e998: {
                                                            'b_17e94c: {
                                                                'b_17e93c: {
                                                                    'b_17e928: {
                                                                        'b_17e8c8: {
                                                                            'b_17e868: {
                                                                                'b_17e808: {
                                                                                    'b_17e7a8: {
                                                                                        'b_17e748: {
                                                                                            'b_17e6e8: {
                                                                                                'b_17e6b0: {
                                                                                                    'b_17e688: {
                                                                                                        c.cycles += 7;
                                                                                                        c.g[V0][0] = 0x460000;
                                                                                                        c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                                                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2bb0));
                                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                                                                        let t = c.g[A0][0] as u32;
                                                                                                        match t {
0x0017e5c8 => {
if event_due(c) { deliver_event_here(c, 0x0017e5c8); } continue 'l_17e5c8;
}
0x0017e628 => {
c.cycles += 24;
c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
c.g[A1][0] = 0x460000;
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2da8));
c.g[AT][0] = 0x3fc00000;
c.mtc1(1, 2);
c.g[A0][0] = c.g[S1][0];
c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5b10));
c.mul_s(0, 0, 2);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 4);
c.mul_s(1, 1, 2);
c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5b0c));
c.g[AT][0] = 0x43a00000;
c.mtc1(1, 12);
c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
c.swc1(4, c.g[SP][0] as u32);
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc));
c.g[AT][0] = 0x43480000;
c.mtc1(1, 13);
let t = true;
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
if t {
break 'b_17e998;
} else {
break 'b_17e688;
}
}
0x0017e688 => {
break 'b_17e688;
}
0x0017e6b0 => {
break 'b_17e6b0;
}
0x0017e6e8 => {
break 'b_17e6e8;
}
0x0017e748 => {
break 'b_17e748;
}
0x0017e7a8 => {
break 'b_17e7a8;
}
0x0017e808 => {
break 'b_17e808;
}
0x0017e868 => {
break 'b_17e868;
}
0x0017e8c8 => {
break 'b_17e8c8;
}
0x0017e928 => {
break 'b_17e928;
}
0x0017e93c => {
break 'b_17e93c;
}
0x0017e9c8 => {
break 'b_17e9c8;
}
0x0017eaa0 => {
break 'b_17eaa0;
}
0x0017eb78 => {
break 'b_17eb78;
}
0x0017eb8c => {
break 'b_17eb8c;
}
0x0017ec64 => {
break 'b_17ec64;
}
0x0017ed3c => {
break 'b_17ed3c;
}
0x0017ee14 => {
break 'b_17ee14;
}
0x0017ef20 => {
break 'b_17ef20;
}
0x0017f1a8 => {
break 'b_17f1a8;
}
_ => { c.jump_out(t, 0x0017e5c8, 0x0017f2cc); return Ok(()); }
}
                                                                                                    }
                                                                                                    c.cycles += 10;
                                                                                                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                                                                                    c.g[A1][0] = 0x460000;
                                                                                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2d90));
                                                                                                    c.g[AT][0] = 0x3fc00000;
                                                                                                    c.mtc1(1, 2);
                                                                                                    c.g[A0][0] = c.g[S1][0];
                                                                                                    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5b08));
                                                                                                    c.mul_s(0, 0, 2);
                                                                                                    c.g[AT][0] = 0x3f800000;
                                                                                                    break 'b_17e6b0;
                                                                                                }
                                                                                                c.cycles += 14;
                                                                                                c.mtc1(1, 4);
                                                                                                c.mul_s(1, 1, 2);
                                                                                                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5b04));
                                                                                                c.g[AT][0] = 0x43a00000;
                                                                                                c.mtc1(1, 12);
                                                                                                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                                                                                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                                                                                                c.swc1(4, c.g[SP][0] as u32);
                                                                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc));
                                                                                                c.g[AT][0] = 0x43480000;
                                                                                                c.mtc1(1, 13);
                                                                                                let t = true;
                                                                                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_17e998;
                                                                                                } else {
                                                                                                    break 'b_17e6e8;
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 24;
                                                                                            c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                                                                            c.g[A1][0] = 0x460000;
                                                                                            c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                                                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2d78));
                                                                                            c.g[AT][0] = 0x3fc00000;
                                                                                            c.mtc1(
                                                                                                1,
                                                                                                2,
                                                                                            );
                                                                                            c.g[A0][0] = c.g[S1][0];
                                                                                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5b00));
                                                                                            c.mul_s(0, 0, 2);
                                                                                            c.g[AT][0] = 0x3f800000;
                                                                                            c.mtc1(
                                                                                                1,
                                                                                                4,
                                                                                            );
                                                                                            c.mul_s(1, 1, 2);
                                                                                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5afc));
                                                                                            c.g[AT][0] = 0x43a00000;
                                                                                            c.mtc1(
                                                                                                1,
                                                                                                12,
                                                                                            );
                                                                                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                                                                            c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                                                                                            c.swc1(4, c.g[SP][0] as u32);
                                                                                            c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                            c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc));
                                                                                            c.g[AT][0] = 0x43480000;
                                                                                            c.mtc1(
                                                                                                1,
                                                                                                13,
                                                                                            );
                                                                                            let t =
                                                                                                true;
                                                                                            c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                                                            if t {
                                                                                                break 'b_17e998;
                                                                                            } else {
                                                                                                break 'b_17e748;
                                                                                            }
                                                                                        }
                                                                                        c.cycles +=
                                                                                            24;
                                                                                        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                                                                        c.g[A1][0] = 0x460000;
                                                                                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                                                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2d60));
                                                                                        c.g[AT][0] = 0x3fc00000;
                                                                                        c.mtc1(
                                                                                            1, 2,
                                                                                        );
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S1]
                                                                                            [0];
                                                                                        c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5af8));
                                                                                        c.mul_s(
                                                                                            0, 0, 2,
                                                                                        );
                                                                                        c.g[AT][0] = 0x3f800000;
                                                                                        c.mtc1(
                                                                                            1, 4,
                                                                                        );
                                                                                        c.mul_s(
                                                                                            1, 1, 2,
                                                                                        );
                                                                                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5af4));
                                                                                        c.g[AT][0] = 0x43a00000;
                                                                                        c.mtc1(
                                                                                            1, 12,
                                                                                        );
                                                                                        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                                                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                                                                                        c.swc1(4, c.g[SP][0] as u32);
                                                                                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc));
                                                                                        c.g[AT][0] = 0x43480000;
                                                                                        c.mtc1(
                                                                                            1, 13,
                                                                                        );
                                                                                        let t =
                                                                                            true;
                                                                                        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                                                        if t {
                                                                                            break 'b_17e998;
                                                                                        } else {
                                                                                            break 'b_17e7a8;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 24;
                                                                                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                                                                    c.g[A1][0] =
                                                                                        0x460000;
                                                                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2d48));
                                                                                    c.g[AT][0] =
                                                                                        0x3fc00000;
                                                                                    c.mtc1(1, 2);
                                                                                    c.g[A0][0] =
                                                                                        c.g[S1][0];
                                                                                    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5af0));
                                                                                    c.mul_s(
                                                                                        0, 0, 2,
                                                                                    );
                                                                                    c.g[AT][0] =
                                                                                        0x3f800000;
                                                                                    c.mtc1(1, 4);
                                                                                    c.mul_s(
                                                                                        1, 1, 2,
                                                                                    );
                                                                                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5aec));
                                                                                    c.g[AT][0] =
                                                                                        0x43a00000;
                                                                                    c.mtc1(1, 12);
                                                                                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                                                                    c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                                                                                    c.swc1(
                                                                                        4,
                                                                                        c.g[SP][0]
                                                                                            as u32,
                                                                                    );
                                                                                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                    c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc));
                                                                                    c.g[AT][0] =
                                                                                        0x43480000;
                                                                                    c.mtc1(1, 13);
                                                                                    let t = true;
                                                                                    c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                                                    if t {
                                                                                        break 'b_17e998;
                                                                                    } else {
                                                                                        break 'b_17e808;
                                                                                    }
                                                                                }
                                                                                c.cycles += 24;
                                                                                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                                                                c.g[A1][0] =
                                                                                    0x460000;
                                                                                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                                                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2d30));
                                                                                c.g[AT][0] =
                                                                                    0x3fc00000;
                                                                                c.mtc1(1, 2);
                                                                                c.g[A0][0] =
                                                                                    c.g[S1][0];
                                                                                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5ae8));
                                                                                c.mul_s(0, 0, 2);
                                                                                c.g[AT][0] =
                                                                                    0x3f800000;
                                                                                c.mtc1(1, 4);
                                                                                c.mul_s(1, 1, 2);
                                                                                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5ae4));
                                                                                c.g[AT][0] =
                                                                                    0x43a00000;
                                                                                c.mtc1(1, 12);
                                                                                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                                                                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                                                                                c.swc1(
                                                                                    4,
                                                                                    c.g[SP][0]
                                                                                        as u32,
                                                                                );
                                                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc));
                                                                                c.g[AT][0] =
                                                                                    0x43480000;
                                                                                c.mtc1(1, 13);
                                                                                let t = true;
                                                                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                                                if t {
                                                                                    break 'b_17e998;
                                                                                } else {
                                                                                    break 'b_17e868;
                                                                                }
                                                                            }
                                                                            c.cycles += 24;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[S1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x30,
                                                                                    ),
                                                                            );
                                                                            c.g[A1][0] = 0x460000;
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[S1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x34,
                                                                                    ),
                                                                            );
                                                                            c.g[A1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x2d18,
                                                                                    ));
                                                                            c.g[AT][0] = 0x3fc00000;
                                                                            c.mtc1(1, 2);
                                                                            c.g[A0][0] = c.g[S1][0];
                                                                            c.lwc1(
                                                                                5,
                                                                                (c.g[GP][0] as u32)
                                                                                    .wrapping_sub(
                                                                                        0x5ae0,
                                                                                    ),
                                                                            );
                                                                            c.mul_s(0, 0, 2);
                                                                            c.g[AT][0] = 0x3f800000;
                                                                            c.mtc1(1, 4);
                                                                            c.mul_s(1, 1, 2);
                                                                            c.lwc1(
                                                                                3,
                                                                                (c.g[GP][0] as u32)
                                                                                    .wrapping_sub(
                                                                                        0x5adc,
                                                                                    ),
                                                                            );
                                                                            c.g[AT][0] = 0x43a00000;
                                                                            c.mtc1(1, 12);
                                                                            c.swc1(
                                                                                0,
                                                                                (c.g[S1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x3c,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                1,
                                                                                (c.g[S1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x38,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                4,
                                                                                c.g[SP][0] as u32,
                                                                            );
                                                                            c.swc1(
                                                                                3,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x4,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                5,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xc,
                                                                                    ),
                                                                            );
                                                                            c.g[AT][0] = 0x43480000;
                                                                            c.mtc1(1, 13);
                                                                            let t = true;
                                                                            c.swc1(
                                                                                5,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x8,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_17e998;
                                                                            } else {
                                                                                break 'b_17e8c8;
                                                                            }
                                                                        }
                                                                        c.cycles += 24;
                                                                        c.lwc1(
                                                                            1,
                                                                            (c.g[S1][0] as u32)
                                                                                .wrapping_add(0x30),
                                                                        );
                                                                        c.g[A1][0] = 0x460000;
                                                                        c.lwc1(
                                                                            0,
                                                                            (c.g[S1][0] as u32)
                                                                                .wrapping_add(0x34),
                                                                        );
                                                                        c.g[A1][0] =
                                                                            sx((c.g[A1][0] as u32)
                                                                                .wrapping_sub(
                                                                                    0x2d00,
                                                                                ));
                                                                        c.g[AT][0] = 0x3fc00000;
                                                                        c.mtc1(1, 2);
                                                                        c.g[A0][0] = c.g[S1][0];
                                                                        c.lwc1(
                                                                            5,
                                                                            (c.g[GP][0] as u32)
                                                                                .wrapping_sub(
                                                                                    0x5ad8,
                                                                                ),
                                                                        );
                                                                        c.mul_s(0, 0, 2);
                                                                        c.g[AT][0] = 0x3f800000;
                                                                        c.mtc1(1, 4);
                                                                        c.mul_s(1, 1, 2);
                                                                        c.lwc1(
                                                                            3,
                                                                            (c.g[GP][0] as u32)
                                                                                .wrapping_sub(
                                                                                    0x5ad4,
                                                                                ),
                                                                        );
                                                                        c.g[AT][0] = 0x43a00000;
                                                                        c.mtc1(1, 12);
                                                                        c.swc1(
                                                                            0,
                                                                            (c.g[S1][0] as u32)
                                                                                .wrapping_add(0x3c),
                                                                        );
                                                                        c.swc1(
                                                                            1,
                                                                            (c.g[S1][0] as u32)
                                                                                .wrapping_add(0x38),
                                                                        );
                                                                        c.swc1(
                                                                            4,
                                                                            c.g[SP][0] as u32,
                                                                        );
                                                                        c.swc1(
                                                                            3,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x4),
                                                                        );
                                                                        c.swc1(
                                                                            5,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xc),
                                                                        );
                                                                        c.g[AT][0] = 0x43480000;
                                                                        c.mtc1(1, 13);
                                                                        let t = true;
                                                                        c.swc1(
                                                                            5,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x8),
                                                                        );
                                                                        if t {
                                                                            break 'b_17e998;
                                                                        } else {
                                                                            break 'b_17e928;
                                                                        }
                                                                    }
                                                                    c.cycles += 5;
                                                                    c.lwc1(
                                                                        1,
                                                                        (c.g[S1][0] as u32)
                                                                            .wrapping_add(0x30),
                                                                    );
                                                                    c.g[A1][0] = 0x460000;
                                                                    c.lwc1(
                                                                        0,
                                                                        (c.g[S1][0] as u32)
                                                                            .wrapping_add(0x34),
                                                                    );
                                                                    let t = true;
                                                                    c.g[A1][0] = sx((c.g[A1][0]
                                                                        as u32)
                                                                        .wrapping_sub(0x2cf0));
                                                                    if t {
                                                                        break 'b_17e94c;
                                                                    } else {
                                                                        break 'b_17e93c;
                                                                    }
                                                                }
                                                                c.cycles += 4;
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.g[A1][0] = 0x460000;
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0x34),
                                                                );
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x2cd8));
                                                                break 'b_17e94c;
                                                            }
                                                            c.cycles += 19;
                                                            c.g[AT][0] = 0x3fc00000;
                                                            c.mtc1(1, 2);
                                                            c.g[A0][0] = c.g[S1][0];
                                                            c.lwc1(
                                                                5,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x5ad0),
                                                            );
                                                            c.mul_s(0, 0, 2);
                                                            c.g[AT][0] = 0x3f800000;
                                                            c.mtc1(1, 4);
                                                            c.mul_s(1, 1, 2);
                                                            c.lwc1(
                                                                3,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x5acc),
                                                            );
                                                            c.g[AT][0] = 0x43a00000;
                                                            c.mtc1(1, 12);
                                                            c.swc1(
                                                                0,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x3c),
                                                            );
                                                            c.swc1(
                                                                1,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x38),
                                                            );
                                                            c.swc1(4, c.g[SP][0] as u32);
                                                            c.swc1(
                                                                3,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.swc1(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc),
                                                            );
                                                            c.swc1(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[AT][0] = 0x42c80000;
                                                            c.mtc1(1, 13);
                                                            break 'b_17e998;
                                                        }
                                                        c.cycles += 12;
                                                        c.ldl(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.ldr(2, c.g[SP][0] as u32);
                                                        c.ldl(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.ldr(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.sdl(
                                                            2,
                                                            (c.g[S1][0] as u32).wrapping_add(0x47),
                                                        );
                                                        c.sdr(
                                                            2,
                                                            (c.g[S1][0] as u32).wrapping_add(0x40),
                                                        );
                                                        c.sdl(
                                                            3,
                                                            (c.g[S1][0] as u32).wrapping_add(0x4f),
                                                        );
                                                        c.sdr(
                                                            3,
                                                            (c.g[S1][0] as u32).wrapping_add(0x48),
                                                        );
                                                        c.g[RA][0] = sx(0x0017e9c0);
                                                        call_ptr(c, 0x00392908, 0x0017e9c0)?;
                                                        let t = true;

                                                        if t {
                                                            break 'b_17f1a8;
                                                        } else {
                                                            break 'b_17e9c8;
                                                        }
                                                    }
                                                    c.cycles += 54;
                                                    c.lwc1(
                                                        1,
                                                        (c.g[S1][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.g[A1][0] = 0x460000;
                                                    c.lwc1(
                                                        0,
                                                        (c.g[S1][0] as u32).wrapping_add(0x34),
                                                    );
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_sub(0x2cc0));
                                                    c.g[AT][0] = 0x3fc00000;
                                                    c.mtc1(1, 2);
                                                    c.g[A0][0] = c.g[S1][0];
                                                    c.lwc1(
                                                        3,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x5ac8),
                                                    );
                                                    c.mul_s(0, 0, 2);
                                                    c.lwc1(
                                                        20,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x5ac4),
                                                    );
                                                    c.mul_s(1, 1, 2);
                                                    c.g[AT][0] = 0x3f800000;
                                                    c.mtc1(1, 21);
                                                    c.g[AT][0] = 0x43a00000;
                                                    c.mtc1(1, 22);
                                                    c.swc1(
                                                        0,
                                                        (c.g[S1][0] as u32).wrapping_add(0x3c),
                                                    );
                                                    c.swc1(
                                                        1,
                                                        (c.g[S1][0] as u32).wrapping_add(0x38),
                                                    );
                                                    c.swc1(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0xc),
                                                    );
                                                    c.swc1(21, c.g[SP][0] as u32);
                                                    c.swc1(
                                                        20,
                                                        (c.g[SP][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.swc1(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x8),
                                                    );
                                                    c.g[AT][0] = 0x42c80000;
                                                    c.mtc1(1, 13);
                                                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                                                    c.ldr(2, c.g[SP][0] as u32);
                                                    c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                                                    c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                    c.sdl(
                                                        2,
                                                        (c.g[S1][0] as u32).wrapping_add(0x47),
                                                    );
                                                    c.sdr(
                                                        2,
                                                        (c.g[S1][0] as u32).wrapping_add(0x40),
                                                    );
                                                    c.sdl(
                                                        3,
                                                        (c.g[S1][0] as u32).wrapping_add(0x4f),
                                                    );
                                                    c.sdr(
                                                        3,
                                                        (c.g[S1][0] as u32).wrapping_add(0x48),
                                                    );
                                                    c.g[RA][0] = sx(0x0017ea4c);
                                                    c.mov_s(12, 22);
                                                    call_ptr(c, 0x00392908, 0x0017ea4c)?;
                                                    c.lwc1(
                                                        2,
                                                        (c.g[S1][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.lwc1(
                                                        0,
                                                        (c.g[S1][0] as u32).wrapping_add(0x34),
                                                    );
                                                    c.lwc1(
                                                        1,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x5ac0),
                                                    );
                                                    c.mul_s(0, 0, 1);
                                                    c.mul_s(2, 2, 1);
                                                    c.swc1(
                                                        0,
                                                        (c.g[S1][0] as u32).wrapping_add(0x3c),
                                                    );
                                                    c.swc1(
                                                        2,
                                                        (c.g[S1][0] as u32).wrapping_add(0x38),
                                                    );
                                                    c.swc1(21, c.g[SP][0] as u32);
                                                    c.swc1(
                                                        20,
                                                        (c.g[SP][0] as u32).wrapping_add(0xc),
                                                    );
                                                    c.swc1(
                                                        20,
                                                        (c.g[SP][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.swc1(
                                                        20,
                                                        (c.g[SP][0] as u32).wrapping_add(0x8),
                                                    );
                                                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                                                    c.ldr(2, c.g[SP][0] as u32);
                                                    c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                                                    c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                    c.sdl(
                                                        2,
                                                        (c.g[S1][0] as u32).wrapping_add(0x47),
                                                    );
                                                    c.sdr(
                                                        2,
                                                        (c.g[S1][0] as u32).wrapping_add(0x40),
                                                    );
                                                    c.sdl(
                                                        3,
                                                        (c.g[S1][0] as u32).wrapping_add(0x4f),
                                                    );
                                                    c.sdr(
                                                        3,
                                                        (c.g[S1][0] as u32).wrapping_add(0x48),
                                                    );
                                                    let t = true;
                                                    c.g[A0][0] = c.g[SP][0];
                                                    if t {
                                                        break 'b_17eee8;
                                                    } else {
                                                        break 'b_17eaa0;
                                                    }
                                                }
                                                c.cycles += 54;
                                                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                                c.g[A1][0] = 0x460000;
                                                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0x2ca8));
                                                c.g[AT][0] = 0x3fc00000;
                                                c.mtc1(1, 2);
                                                c.g[A0][0] = c.g[S1][0];
                                                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5abc));
                                                c.mul_s(0, 0, 2);
                                                c.lwc1(
                                                    20,
                                                    (c.g[GP][0] as u32).wrapping_sub(0x5ab8),
                                                );
                                                c.mul_s(1, 1, 2);
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 21);
                                                c.g[AT][0] = 0x43a00000;
                                                c.mtc1(1, 22);
                                                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
                                                c.swc1(21, c.g[SP][0] as u32);
                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                c.g[AT][0] = 0x42c80000;
                                                c.mtc1(1, 13);
                                                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                                                c.ldr(7, c.g[SP][0] as u32);
                                                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0xf));
                                                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                c.sdl(7, (c.g[S1][0] as u32).wrapping_add(0x47));
                                                c.sdr(7, (c.g[S1][0] as u32).wrapping_add(0x40));
                                                c.sdl(2, (c.g[S1][0] as u32).wrapping_add(0x4f));
                                                c.sdr(2, (c.g[S1][0] as u32).wrapping_add(0x48));
                                                c.g[RA][0] = sx(0x0017eb24);
                                                c.mov_s(12, 22);
                                                call_ptr(c, 0x00392908, 0x0017eb24)?;
                                                c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x30));
                                                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5ab4));
                                                c.mul_s(0, 0, 1);
                                                c.mul_s(2, 2, 1);
                                                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                                c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x38));
                                                c.swc1(21, c.g[SP][0] as u32);
                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                                                c.ldr(7, c.g[SP][0] as u32);
                                                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0xf));
                                                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                c.sdl(7, (c.g[S1][0] as u32).wrapping_add(0x47));
                                                c.sdr(7, (c.g[S1][0] as u32).wrapping_add(0x40));
                                                c.sdl(2, (c.g[S1][0] as u32).wrapping_add(0x4f));
                                                c.sdr(2, (c.g[S1][0] as u32).wrapping_add(0x48));
                                                let t = true;
                                                c.g[A0][0] = c.g[SP][0];
                                                if t {
                                                    break 'b_17eee8;
                                                } else {
                                                    break 'b_17eb78;
                                                }
                                            }
                                            c.cycles += 5;
                                            c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                            c.g[A1][0] = 0x460000;
                                            c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                            let t = true;
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_sub(0x2c88));
                                            if t {
                                                break 'b_17ee24;
                                            } else {
                                                break 'b_17eb8c;
                                            }
                                        }
                                        c.cycles += 54;
                                        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                        c.g[A1][0] = 0x460000;
                                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2c70));
                                        c.g[AT][0] = 0x3fc00000;
                                        c.mtc1(1, 2);
                                        c.g[A0][0] = c.g[S1][0];
                                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5ab0));
                                        c.mul_s(0, 0, 2);
                                        c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x5aac));
                                        c.mul_s(1, 1, 2);
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 21);
                                        c.g[AT][0] = 0x43a00000;
                                        c.mtc1(1, 22);
                                        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
                                        c.swc1(21, c.g[SP][0] as u32);
                                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                        c.g[AT][0] = 0x42c80000;
                                        c.mtc1(1, 13);
                                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                                        c.ldr(3, c.g[SP][0] as u32);
                                        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xf));
                                        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x8));
                                        c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x47));
                                        c.sdr(3, (c.g[S1][0] as u32).wrapping_add(0x40));
                                        c.sdl(6, (c.g[S1][0] as u32).wrapping_add(0x4f));
                                        c.sdr(6, (c.g[S1][0] as u32).wrapping_add(0x48));
                                        c.g[RA][0] = sx(0x0017ec10);
                                        c.mov_s(12, 22);
                                        call_ptr(c, 0x00392908, 0x0017ec10)?;
                                        c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x30));
                                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5aa8));
                                        c.mul_s(0, 0, 1);
                                        c.mul_s(2, 2, 1);
                                        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                        c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x38));
                                        c.swc1(21, c.g[SP][0] as u32);
                                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
                                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
                                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                                        c.ldr(3, c.g[SP][0] as u32);
                                        c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0xf));
                                        c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                                        c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x47));
                                        c.sdr(3, (c.g[S1][0] as u32).wrapping_add(0x40));
                                        c.sdl(5, (c.g[S1][0] as u32).wrapping_add(0x4f));
                                        c.sdr(5, (c.g[S1][0] as u32).wrapping_add(0x48));
                                        let t = true;
                                        c.g[A0][0] = c.g[SP][0];
                                        if t {
                                            break 'b_17eee8;
                                        } else {
                                            break 'b_17ec64;
                                        }
                                    }
                                    c.cycles += 54;
                                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                    c.g[A1][0] = 0x460000;
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2c60));
                                    c.g[AT][0] = 0x3fc00000;
                                    c.mtc1(1, 2);
                                    c.g[A0][0] = c.g[S1][0];
                                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5aa4));
                                    c.mul_s(0, 0, 2);
                                    c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x5aa0));
                                    c.mul_s(1, 1, 2);
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 21);
                                    c.g[AT][0] = 0x43a00000;
                                    c.mtc1(1, 22);
                                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                    c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
                                    c.swc1(21, c.g[SP][0] as u32);
                                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                    c.g[AT][0] = 0x42c80000;
                                    c.mtc1(1, 13);
                                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                                    c.ldr(2, c.g[SP][0] as u32);
                                    c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                                    c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                    c.sdl(2, (c.g[S1][0] as u32).wrapping_add(0x47));
                                    c.sdr(2, (c.g[S1][0] as u32).wrapping_add(0x40));
                                    c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x4f));
                                    c.sdr(3, (c.g[S1][0] as u32).wrapping_add(0x48));
                                    c.g[RA][0] = sx(0x0017ece8);
                                    c.mov_s(12, 22);
                                    call_ptr(c, 0x00392908, 0x0017ece8)?;
                                    c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x30));
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5a9c));
                                    c.mul_s(0, 0, 1);
                                    c.mul_s(2, 2, 1);
                                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                    c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x38));
                                    c.swc1(21, c.g[SP][0] as u32);
                                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
                                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
                                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                                    c.ldr(2, c.g[SP][0] as u32);
                                    c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                                    c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                    c.sdl(2, (c.g[S1][0] as u32).wrapping_add(0x47));
                                    c.sdr(2, (c.g[S1][0] as u32).wrapping_add(0x40));
                                    c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x4f));
                                    c.sdr(3, (c.g[S1][0] as u32).wrapping_add(0x48));
                                    let t = true;
                                    c.g[A0][0] = c.g[SP][0];
                                    if t {
                                        break 'b_17eee8;
                                    } else {
                                        break 'b_17ed3c;
                                    }
                                }
                                c.cycles += 54;
                                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                                c.g[A1][0] = 0x460000;
                                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2c40));
                                c.g[AT][0] = 0x3fc00000;
                                c.mtc1(1, 2);
                                c.g[A0][0] = c.g[S1][0];
                                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5a98));
                                c.mul_s(0, 0, 2);
                                c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x5a94));
                                c.mul_s(1, 1, 2);
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 21);
                                c.g[AT][0] = 0x43a00000;
                                c.mtc1(1, 22);
                                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
                                c.swc1(21, c.g[SP][0] as u32);
                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.g[AT][0] = 0x42c80000;
                                c.mtc1(1, 13);
                                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                                c.ldr(7, c.g[SP][0] as u32);
                                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0xf));
                                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.sdl(7, (c.g[S1][0] as u32).wrapping_add(0x47));
                                c.sdr(7, (c.g[S1][0] as u32).wrapping_add(0x40));
                                c.sdl(2, (c.g[S1][0] as u32).wrapping_add(0x4f));
                                c.sdr(2, (c.g[S1][0] as u32).wrapping_add(0x48));
                                c.g[RA][0] = sx(0x0017edc0);
                                c.mov_s(12, 22);
                                call_ptr(c, 0x00392908, 0x0017edc0)?;
                                c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x30));
                                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5a90));
                                c.mul_s(0, 0, 1);
                                c.mul_s(2, 2, 1);
                                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                                c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x38));
                                c.swc1(21, c.g[SP][0] as u32);
                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                                c.ldr(7, c.g[SP][0] as u32);
                                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0xf));
                                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.sdl(7, (c.g[S1][0] as u32).wrapping_add(0x47));
                                c.sdr(7, (c.g[S1][0] as u32).wrapping_add(0x40));
                                c.sdl(2, (c.g[S1][0] as u32).wrapping_add(0x4f));
                                c.sdr(2, (c.g[S1][0] as u32).wrapping_add(0x48));
                                let t = true;
                                c.g[A0][0] = c.g[SP][0];
                                if t {
                                    break 'b_17eee8;
                                } else {
                                    break 'b_17ee14;
                                }
                            }
                            c.cycles += 4;
                            c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                            c.g[A1][0] = 0x460000;
                            c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2c28));
                            break 'b_17ee24;
                        }
                        c.cycles += 49;
                        c.g[AT][0] = 0x3fc00000;
                        c.mtc1(1, 2);
                        c.g[A0][0] = c.g[S1][0];
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5a8c));
                        c.mul_s(0, 0, 2);
                        c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x5a88));
                        c.mul_s(1, 1, 2);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 21);
                        c.g[AT][0] = 0x43a00000;
                        c.mtc1(1, 22);
                        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
                        c.swc1(21, c.g[SP][0] as u32);
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.g[AT][0] = 0x42c80000;
                        c.mtc1(1, 13);
                        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.ldr(6, c.g[SP][0] as u32);
                        c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0xf));
                        c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.sdl(6, (c.g[S1][0] as u32).wrapping_add(0x47));
                        c.sdr(6, (c.g[S1][0] as u32).wrapping_add(0x40));
                        c.sdl(7, (c.g[S1][0] as u32).wrapping_add(0x4f));
                        c.sdr(7, (c.g[S1][0] as u32).wrapping_add(0x48));
                        c.g[RA][0] = sx(0x0017ee98);
                        c.mov_s(12, 22);
                        call_ptr(c, 0x00392908, 0x0017ee98)?;
                        c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x30));
                        c.g[A0][0] = c.g[SP][0];
                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5a84));
                        c.mul_s(0, 0, 1);
                        c.mul_s(2, 2, 1);
                        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                        c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x38));
                        c.swc1(21, c.g[SP][0] as u32);
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.ldr(5, c.g[SP][0] as u32);
                        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xf));
                        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.sdl(5, (c.g[S1][0] as u32).wrapping_add(0x47));
                        c.sdr(5, (c.g[S1][0] as u32).wrapping_add(0x40));
                        c.sdl(6, (c.g[S1][0] as u32).wrapping_add(0x4f));
                        c.sdr(6, (c.g[S1][0] as u32).wrapping_add(0x48));
                        break 'b_17eee8;
                    }
                    c.cycles += 14;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    c.g[RA][0] = sx(0x0017eef4);
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c14));
                    call_ptr(c, 0x00260e50, 0x0017eef4)?;
                    c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[AT][0] = 0x43160000;
                    c.mtc1(1, 13);
                    c.g[RA][0] = sx(0x0017ef0c);
                    c.mov_s(12, 22);
                    call_ptr(c, 0x00392908, 0x0017ef0c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0017ef18);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x0017ef18)?;
                    let t = true;

                    if t {
                        break 'b_17f1a8;
                    } else {
                        break 'b_17ef20;
                    }
                }
                'b_17f0ac: {
                    'b_17f0a4: {
                        c.cycles += 95;
                        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                        c.g[A1][0] = 0x460000;
                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2c08));
                        c.g[AT][0] = 0x3fc00000;
                        c.mtc1(1, 2);
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5a80));
                        c.mul_s(0, 0, 2);
                        c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x5a7c));
                        c.mul_s(1, 1, 2);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 23);
                        c.g[AT][0] = 0x43a00000;
                        c.mtc1(1, 12);
                        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.swc1(23, c.g[SP][0] as u32);
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.g[AT][0] = 0x42c80000;
                        c.mtc1(1, 13);
                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.ldr(3, c.g[SP][0] as u32);
                        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xf));
                        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x47));
                        c.sdr(3, (c.g[S1][0] as u32).wrapping_add(0x40));
                        c.sdl(6, (c.g[S1][0] as u32).wrapping_add(0x4f));
                        c.sdr(6, (c.g[S1][0] as u32).wrapping_add(0x48));
                        c.g[RA][0] = sx(0x0017efa0);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00392908, 0x0017efa0)?;
                        c.g[AT][0] = 0x43480000;
                        c.mtc1(1, 24);
                        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                        c.g[A1][0] = 0x4a0000;
                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                        c.mov_s(13, 24);
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5a78));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1328));
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5a74));
                        c.g[A0][0] = c.g[S1][0];
                        c.mul_s(0, 0, 2);
                        c.g[AT][0] = 0x42dc0000;
                        c.mtc1(1, 20);
                        c.mul_s(1, 1, 2);
                        c.lwc1(22, (c.g[GP][0] as u32).wrapping_sub(0x5a70));
                        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                        c.swc1(23, c.g[SP][0] as u32);
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.ldr(3, c.g[SP][0] as u32);
                        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xf));
                        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x47));
                        c.sdr(3, (c.g[S1][0] as u32).wrapping_add(0x40));
                        c.sdl(6, (c.g[S1][0] as u32).wrapping_add(0x4f));
                        c.sdr(6, (c.g[S1][0] as u32).wrapping_add(0x48));
                        c.g[RA][0] = sx(0x0017f01c);
                        c.mov_s(12, 20);
                        call_ptr(c, 0x00391cb0, 0x0017f01c)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        c.mov_s(13, 24);
                        c.g[A0][0] = c.g[S1][0];
                        c.mov_s(12, 22);
                        c.g[AT][0] = 0x435c0000;
                        c.mtc1(1, 24);
                        c.g[RA][0] = sx(0x0017f03c);
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x88));
                        call_ptr(c, 0x00392a60, 0x0017f03c)?;
                        c.g[A1][0] = 0x460000;
                        c.mov_s(13, 24);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2bf8));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x0017f054);
                        c.mov_s(12, 20);
                        call_ptr(c, 0x00391cb0, 0x0017f054)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        c.mov_s(13, 24);
                        c.mov_s(12, 22);
                        c.g[A0][0] = c.g[S1][0];
                        c.g[AT][0] = 0x43700000;
                        c.mtc1(1, 24);
                        c.g[RA][0] = sx(0x0017f074);
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x98));
                        call_ptr(c, 0x00392a60, 0x0017f074)?;
                        c.g[A1][0] = 0x460000;
                        c.mov_s(12, 20);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2be8));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x0017f08c);
                        c.mov_s(13, 24);
                        call_ptr(c, 0x00391cb0, 0x0017f08c)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xb4));
                        let t = c.g[V1][0] == 0u64;
                        c.g[V0][0] = 0x4a0000;
                        if t {
                            break 'b_17f0a4;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12f8));
                            if t {
                                break 'b_17f0ac;
                            } else {
                                break 'b_17f0a4;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1330));
                    break 'b_17f0ac;
                }
                c.cycles += 63;
                c.g[AT][0] = 0x41a00000;
                c.mtc1(1, 20);
                c.g[A0][0] = c.g[S1][0];
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5a6c));
                c.g[AT][0] = 0x43700000;
                c.mtc1(1, 13);
                c.g[AT][0] = 0x42dc0000;
                c.mtc1(1, 21);
                c.g[RA][0] = sx(0x0017f0d4);
                c.add_s(24, 24, 20);
                call_ptr(c, 0x00392a60, 0x0017f0d4)?;
                c.lwc1(22, (c.g[GP][0] as u32).wrapping_sub(0x5a68));
                c.g[A1][0] = 0x460000;
                c.mov_s(13, 24);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2bd8));
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0017f0f0);
                c.mov_s(12, 21);
                call_ptr(c, 0x00391cb0, 0x0017f0f0)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                c.g[S0][0] = 0x460000;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3068));
                c.mov_s(13, 24);
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa8));
                c.add_s(24, 24, 20);
                c.g[T0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xa9));
                c.g[A1][0] = c.g[S0][0];
                c.g[A3][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xaa));
                c.g[T1][0] = c.g[A2][0] & 0xff;
                c.g[A2][0] = sx((c.g[A2][0] as u32) >> 24);
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0017f128);
                c.mov_s(12, 22);
                call_ptr(c, 0x001dca80, 0x0017f128)?;
                c.g[A1][0] = 0x460000;
                c.mov_s(13, 24);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2bc8));
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0017f140);
                c.mov_s(12, 21);
                call_ptr(c, 0x00391cb0, 0x0017f140)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                c.mov_s(13, 24);
                c.g[A1][0] = c.g[S0][0];
                c.add_s(24, 24, 20);
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xac));
                c.g[A0][0] = c.g[S1][0];
                c.g[T0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xad));
                c.mov_s(12, 22);
                c.g[A3][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xae));
                c.g[T1][0] = c.g[A2][0] & 0xff;
                c.g[RA][0] = sx(0x0017f170);
                c.g[A2][0] = sx((c.g[A2][0] as u32) >> 24);
                call_ptr(c, 0x001dca80, 0x0017f170)?;
                c.g[A1][0] = 0x4a0000;
                c.mov_s(12, 21);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1338));
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0017f188);
                c.mov_s(13, 24);
                call_ptr(c, 0x00391cb0, 0x0017f188)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                c.g[A1][0] = 0x4a0000;
                c.mov_s(12, 22);
                c.mov_s(13, 24);
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xb0));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12c0));
                c.g[RA][0] = sx(0x0017f1a8);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x001dca80, 0x0017f1a8)?;
                break 'b_17f1a8;
            }
        }
        'b_17f1e4: {
            'b_17f1e0: {
                c.cycles += 10;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5a64));
                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x34));
                c.mul_s(1, 1, 0);
                c.mul_s(2, 2, 0);
                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x3c));
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                if t {
                    break 'b_17f1e0;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    if t {
                        break 'b_17f1e4;
                    } else {
                        break 'b_17f1e0;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_17f1e4;
        }
        'b_17f1f8: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
            if t {
                break 'b_17f1f8;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0017f1f4);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                call_ptr(c, 0x002cc350, 0x0017f1f4)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                break 'b_17f1f8;
            }
        }
        'b_17f2a4: {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = c.g[V0][0] ^ 0x5;
            let t = c.g[V0][0] != 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            if t {
                break 'b_17f2a4;
            } else {
                c.cycles += 39;
                c.g[RA][0] = sx(0x0017f210);
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                call_ptr(c, 0x00256038, 0x0017f210)?;
                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x30));
                c.g[A1][0] = 0x460000;
                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
                c.g[A0][0] = c.g[S1][0];
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5a60));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3068));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5a5c));
                c.mul_s(0, 0, 2);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 3);
                c.mul_s(1, 1, 2);
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5a58));
                c.g[AT][0] = 0x43a00000;
                c.mtc1(1, 12);
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c));
                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x38));
                c.swc1(3, c.g[SP][0] as u32);
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xc));
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(3, c.g[SP][0] as u32);
                c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x47));
                c.sdr(3, (c.g[S1][0] as u32).wrapping_add(0x40));
                c.sdl(6, (c.g[S1][0] as u32).wrapping_add(0x4f));
                c.sdr(6, (c.g[S1][0] as u32).wrapping_add(0x48));
                c.g[AT][0] = 0x43d20000;
                c.mtc1(1, 13);
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A3][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x12));
                c.g[T1][0] = c.g[A2][0] & 0xff;
                c.g[T0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x11));
                c.g[RA][0] = sx(0x0017f2a0);
                c.g[A2][0] = sx((c.g[A2][0] as u32) >> 24);
                call_ptr(c, 0x001dca40, 0x0017f2a0)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_17f2a4;
            }
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x78));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x68));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, elf_image, fill, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    // Fixture objects, in DATA+0x200000 and up (clear of the fakes' call log at
    // DATA+0x100000 and of the pointers the fakes return).
    pub(crate) const OC819_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC819_MAGIC: u32 = 0x4f43_3831;
    pub(crate) const OC819_WORLD: u32 = DATA + 0x21_0000;
    pub(crate) const OC819_VIEW: u32 = DATA + 0x22_0000;
    pub(crate) const OC819_FONT: u32 = DATA + 0x23_0000;
    pub(crate) const OC819_SELF: u32 = DATA + 0x24_0000;
    pub(crate) const OC819_LOBBY: u32 = DATA + 0x25_0000;
    pub(crate) const OC819_COMM: u32 = DATA + 0x26_0000;

    /// Gated fake callees: logged like `fakes::fake_call` while the machine's
    /// gate word is set, the body they replaced otherwise.
    macro_rules! oc819_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC819_CTRL) != OC819_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc819_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc819_fakes!(
        (
            oc819_fake_392908,
            OC819_PREV_392908,
            ee::s12::OVERRIDE_00392908,
            ee::s12::f_00392908__generated,
            0x0039_2908
        ),
        (
            oc819_fake_391cb0,
            OC819_PREV_391CB0,
            ee::s12::OVERRIDE_00391CB0,
            ee::s12::f_00391cb0__generated,
            0x0039_1cb0
        ),
        (
            oc819_fake_392a60,
            OC819_PREV_392A60,
            ee::s13::OVERRIDE_00392A60,
            ee::s13::f_00392a60__generated,
            0x0039_2a60
        ),
        (
            oc819_fake_1dca80,
            OC819_PREV_1DCA80,
            ee::s04::OVERRIDE_001DCA80,
            ee::s04::f_001dca80__generated,
            0x001d_ca80
        ),
        (
            oc819_fake_1dca40,
            OC819_PREV_1DCA40,
            ee::s04::OVERRIDE_001DCA40,
            ee::s04::f_001dca40__generated,
            0x001d_ca40
        ),
        (
            oc819_fake_2cc350,
            OC819_PREV_2CC350,
            ee::s09::OVERRIDE_002CC350,
            ee::s09::f_002cc350__generated,
            0x002c_c350
        ),
        (
            oc819_fake_260e50,
            OC819_PREV_260E50,
            ee::s06::OVERRIDE_00260E50,
            ee::s06::f_00260e50__generated,
            0x0026_0e50
        ),
        (
            oc819_fake_318260,
            OC819_PREV_318260,
            ee::s10::OVERRIDE_00318260,
            ee::s10::f_00318260__generated,
            0x0031_8260
        ),
    );

    /// `LobbyMenu_Render`: every lobby state (0..24 with its jump-table target,
    /// 25..27 past the table), the font at `world->+0x80->+0x18`, the menu stack
    /// (none, or its top entry), the 'Play Game' table's host flag, and the
    /// comm system in a net state other than 5. The state-5 branch (the local
    /// address, through the unported `NetConnStatus` RPC chain at 0x3f3cc0) is
    /// not reached: that callee has no slot to fake.
    #[test]
    fn lobby_menu_render_matches() {
        oc819_install_fakes();
        check(
            ee::s02::f_0017e5c8__generated,
            lobby_menu_render,
            420,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x819, 24);
                c.bus.write32(OC819_CTRL, OC819_MAGIC);
                // World -> view -> font (scale at +0x30/+0x34, outputs +0x38..).
                c.bus.write32(GAME_GP.wrapping_sub(0x848), OC819_WORLD);
                c.bus.write32(OC819_WORLD + 0x80, OC819_VIEW);
                c.bus.write32(OC819_VIEW + 0x18, OC819_FONT);
                fill(c, &mut r, OC819_FONT, 0x20);
                let (scale_x, scale_y) = (r.edge_f32(), r.edge_f32());
                c.bus.write32(OC819_FONT + 0x30, scale_x);
                c.bus.write32(OC819_FONT + 0x34, scale_y);
                // The screen: state, menu stack count, entries, error code.
                fill(c, &mut r, OC819_SELF, 0x20);
                c.bus.write32(OC819_SELF + 4, seed % 28);
                let count = [0u32, 1, 2, 3, 0xffff_ffff][((seed / 28) % 5) as usize];
                c.bus.write32(OC819_SELF + 8, count);
                for i in 0..4u32 {
                    let top = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x1000 * (i + 1)
                    };
                    c.bus.write32(OC819_SELF + 0xc + 4 * i, top);
                }
                let err = r.next_u32();
                c.bus.write32(OC819_SELF + 0x1c14, err);
                // The lobby manager (gp-0xc8): names, host flag, addresses, seed.
                c.bus.write32(GAME_GP.wrapping_sub(0xc8), OC819_LOBBY);
                fill(c, &mut r, OC819_LOBBY + 0x80, 0x20);
                c.bus.write32(OC819_LOBBY + 0xb4, seed % 2);
                // The comm system (gp-0x238): net state 0..4, never 5.
                c.bus.write32(GAME_GP.wrapping_sub(0x238), OC819_COMM);
                c.bus.write32(OC819_COMM + 0x20, seed % 5);
                c.g[A0][0] = sx(OC819_SELF);
                c.pc = 0x0017_e5c8;
            },
        );
    }
}
