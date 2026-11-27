//! Staging module oc960 (wave wave257): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc960`.

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
    (
        0x001e_6718,
        ov_screen_final_standings_transition_in,
        "OVScreenFinalStandings_TransitionIn",
    ),
    (
        0x001e_8ff0,
        ov_screen_hiscore_list_init,
        "OVScreenHiscoreList_Init",
    ),
];

/// `OVScreenFinalStandings_TransitionIn` (0x001e6718, `cOVState_FINAL_STANDINGS_onGainTransition`):
/// the final-standings overlay's gain-transition handler. Fills the standings screen's text and
/// icon widgets (location, rider names and points, trophy icons) and shows or hides them by the
/// event type in the game state (jump table at 0x46e7b0 + 4 * kind).
/// Converted from the generated body (labelled blocks: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_final_standings_transition_in(c: &mut Cpu) {
    if c.pc != 0x001e6718 {
        return ee::s04::f_001e6718__generated(c);
    }
    guest(c, |c| 'l_1e6718: loop {
        'b_1e7528: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x460));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x3f0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x450));
            c.g[S6][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x440));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x430));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x420));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x410));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x400));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x3e0));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x3d0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x3c0), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x9c));
            let t = c.g[V0][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x450));
            if t {
                break 'b_1e7528;
            } else {
                'b_1e6868: {
                    'b_1e67e4: {
                        'b_1e67d0: {
                            c.cycles += 28;
                            c.g[RA][0] = sx(0x001e675c);
                            c.sw((c.g[S6][0] as u32).wrapping_add(0x9c), 0u64);
                            call_ptr(c, 0x0014dc80, 0x001e675c)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001e6768);
                            c.g[A1][0] = 0x1;
                            call_ptr(c, 0x0014dd58, 0x001e6768)?;
                            c.g[RA][0] = sx(0x001e6770);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x394), c.g[V0][0]);
                            call_ptr(c, 0x0014dc80, 0x001e6770)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001e677c);
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x0014dd58, 0x001e677c)?;
                            c.g[RA][0] = sx(0x001e6784);
                            c.g[S1][0] = c.g[V0][0];
                            call_ptr(c, 0x0014dc80, 0x001e6784)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001e6790);
                            c.g[A1][0] = 0x2;
                            call_ptr(c, 0x0014dd58, 0x001e6790)?;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x398), c.g[V0][0]);
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[V0][0] = 0x530000;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                            c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc0));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x39c), c.g[V1][0]);
                            c.g[V1][0] = c.lw((c.g[V1][0] as u32));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x3a0), c.g[V1][0]);
                            c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x3a4), c.g[A0][0]);
                            c.g[S7][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                            let t = c.g[S7][0] == 0u64;
                            c.g[V0][0] = 0x2;
                            if t {
                                break 'b_1e67d0;
                            } else {
                                c.cycles += 3;
                                c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x5));
                                let t = c.g[A0][0] == c.g[V0][0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x3b4), c.g[A1][0]);
                                if t {
                                    break 'b_1e67e4;
                                } else {
                                    break 'b_1e67d0;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x5));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x3b4), c.g[V0][0]);
                        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                        if c.g[V0][0] == 0u64 {
                            c.g[A0][0] = 0x470000;
                            break 'b_1e6868;
                        } else {
                            break 'b_1e67e4;
                        }
                    }
                    c.cycles += 33;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x001e67f0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1ac8));
                    call_ptr(c, 0x00317670, 0x001e67f0)?;
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001e67fc);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001e67fc)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001e6818);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001e6818)?;
                    c.ldl(6, (c.g[S0][0] as u32).wrapping_add(0x4b));
                    c.ldr(6, (c.g[S0][0] as u32).wrapping_add(0x44));
                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                    c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                    c.sdr(6, (c.g[SP][0] as u32));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[AT][0] = 0x43480000;
                    c.mtc1(1, 2);
                    c.lwc1(0, (c.g[SP][0] as u32));
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
                    c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x17));
                    c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                    c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0x4b));
                    c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0x44));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[A1][0]);
                    c.g[A0][0] = 0x470000;
                    break 'b_1e6868;
                }
                'b_1e6934: {
                    c.cycles += 19;
                    c.g[RA][0] = sx(0x001e6870);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c78));
                    call_ptr(c, 0x00317670, 0x001e6870)?;
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001e687c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001e687c)?;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001e6890);
                    c.g[S3][0] = c.g[S0][0];
                    call_ptr(c, 0x00144bf0, 0x001e6890)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001e689c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002c2540, 0x001e689c)?;
                    c.g[V0][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A3][0] = 0u64;
                    if t {
                        break 'b_1e6934;
                    } else {
                        'b_1e690c: {
                            'b_1e6900: {
                                'b_1e68f0: {
                                    'b_1e68e0: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x470000;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1850));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x001e6718 => {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x001e6718);
                                                }
                                                continue 'l_1e6718;
                                            }
                                            0x001e68d0 => {
                                                c.cycles += 4;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                c.g[A0][0] = 0x470000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x1ab8));
                                                if t {
                                                    break 'b_1e690c;
                                                } else {
                                                    break 'b_1e68e0;
                                                }
                                            }
                                            0x001e68e0 => {
                                                break 'b_1e68e0;
                                            }
                                            0x001e68f0 => {
                                                break 'b_1e68f0;
                                            }
                                            0x001e6900 => {
                                                break 'b_1e6900;
                                            }
                                            0x001e6934 => {
                                                break 'b_1e6934;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x001e6718, 0x001e7558);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[A0][0] = 0x470000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c68));
                                    if t {
                                        break 'b_1e690c;
                                    } else {
                                        break 'b_1e68f0;
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A0][0] = 0x470000;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c58));
                                if t {
                                    break 'b_1e690c;
                                } else {
                                    break 'b_1e6900;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A0][0] = 0x470000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c40));
                            break 'b_1e690c;
                        }
                        c.cycles += 10;
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = sx(0x001e6920);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        call_ptr(c, 0x00317670, 0x001e6920)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.g[A0][0] = c.g[S0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001e6930);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, t, 0x001e6930)?;
                        c.g[A3][0] = c.g[V0][0];
                        break 'b_1e6934;
                    }
                }
                'b_1e6978: {
                    'b_1e6964: {
                        c.cycles += 4;
                        c.g[V0][0] = 0x4a0000;
                        c.g[V1][0] = 0x6;
                        let t = c.g[S7][0] != c.g[V1][0];
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2140));
                        if t {
                            break 'b_1e6964;
                        } else {
                            c.cycles += 8;
                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                            c.g[A1][0] = 0x470000;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1aa8));
                            c.g[A2][0] = c.g[S3][0];
                            c.g[RA][0] = sx(0x001e695c);
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x002c27c0, 0x001e695c)?;
                            let t = true;
                            c.g[A1][0] = c.g[S0][0];
                            if t {
                                break 'b_1e6978;
                            } else {
                                break 'b_1e6964;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[A2][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001e6974);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002c27c0, 0x001e6974)?;
                    c.g[A1][0] = c.g[S0][0];
                    break 'b_1e6978;
                }
                'b_1e6a74: {
                    'b_1e6a5c: {
                        'b_1e6a40: {
                            'b_1e69b4: {
                                c.cycles += 12;
                                c.g[RA][0] = sx(0x001e6980);
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x003a0e90, 0x001e6980)?;
                                c.g[A0][0] = 0x4a0000;
                                c.g[RA][0] = sx(0x001e698c);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2148));
                                call_ptr(c, 0x00317670, 0x001e698c)?;
                                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001e6998);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001e6998)?;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3a4));
                                c.g[V1][0] = 0x2;
                                let t = c.g[A0][0] != c.g[V1][0];
                                c.g[S2][0] = c.g[V0][0];
                                if t {
                                    break 'b_1e69b4;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x470000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1a98));
                                    if t {
                                        break 'b_1e6a40;
                                    } else {
                                        break 'b_1e69b4;
                                    }
                                }
                            }
                            'b_1e69d0: {
                                c.cycles += 4;
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3a4));
                                c.g[A0][0] = 0x1;
                                let t = c.g[A1][0] != c.g[A0][0];

                                if t {
                                    break 'b_1e69d0;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x470000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1a78));
                                    if t {
                                        break 'b_1e6a40;
                                    } else {
                                        break 'b_1e69d0;
                                    }
                                }
                            }
                            'b_1e6a2c: {
                                c.cycles += 2;
                                if c.g[S7][0] != 0u64 {
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3b4));
                                    break 'b_1e6a2c;
                                } else {
                                    'b_1e6a24: {
                                        'b_1e6a18: {
                                            c.cycles += 3;
                                            c.g[A2][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x3a0));
                                            let t = c.g[A2][0] == c.g[V1][0];
                                            c.g[V0][0] = ((c.g[A2][0] as i64) < 3i64) as u64;
                                            if t {
                                                break 'b_1e6a18;
                                            } else {
                                                'b_1e6a10: {
                                                    'b_1e69fc: {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A3][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x3a0));
                                                        if t {
                                                            break 'b_1e69fc;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[A2][0] == c.g[A0][0];
                                                            c.g[A0][0] = 0x470000;
                                                            if t {
                                                                break 'b_1e6a10;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.g[A0][0] = 0x470000;
                                                                if t {
                                                                    break 'b_1e6a74;
                                                                } else {
                                                                    break 'b_1e69fc;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 3;
                                                    c.g[V0][0] = 0x3;
                                                    let t = c.g[A3][0] == c.g[V0][0];
                                                    c.g[A0][0] = 0x470000;
                                                    if t {
                                                        break 'b_1e6a24;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[A0][0] = 0x470000;
                                                        if t {
                                                            break 'b_1e6a74;
                                                        } else {
                                                            break 'b_1e6a10;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x1a58));
                                                if t {
                                                    break 'b_1e6a40;
                                                } else {
                                                    break 'b_1e6a18;
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x470000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1a40));
                                        if t {
                                            break 'b_1e6a40;
                                        } else {
                                            break 'b_1e6a24;
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1a28));
                                    if t {
                                        break 'b_1e6a40;
                                    } else {
                                        break 'b_1e6a2c;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[V1][0] < 0x2u64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = 0x470000;
                            if t {
                                break 'b_1e6a5c;
                            } else {
                                c.cycles += 2;
                                c.g[A0][0] = 0x470000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1a10));
                                break 'b_1e6a40;
                            }
                        }
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x001e6a48);
                        call_ptr(c, 0x00317670, 0x001e6a48)?;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x001e6a54);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x001e6a54)?;
                        let t = true;
                        c.g[A0][0] = 0x470000;
                        if t {
                            break 'b_1e6a74;
                        } else {
                            break 'b_1e6a5c;
                        }
                    }
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x001e6a64);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1a28));
                    call_ptr(c, 0x00317670, 0x001e6a64)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e6a70);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001e6a70)?;
                    c.g[A0][0] = 0x470000;
                    break 'b_1e6a74;
                }
                'b_1e6ad4: {
                    'b_1e6abc: {
                        'b_1e6a9c: {
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x001e6a7c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x19f8));
                            call_ptr(c, 0x00317670, 0x001e6a7c)?;
                            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001e6a88);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001e6a88)?;
                            let t = c.g[S7][0] == 0u64;
                            c.g[S2][0] = c.g[V0][0];
                            if t {
                                break 'b_1e6a9c;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = 0x5;
                                let t = c.g[S7][0] != c.g[V0][0];
                                c.g[A0][0] = 0x470000;
                                if t {
                                    break 'b_1e6abc;
                                } else {
                                    break 'b_1e6a9c;
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[A0][0] = 0x470000;
                        c.g[RA][0] = sx(0x001e6aa8);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x19e8));
                        call_ptr(c, 0x00317670, 0x001e6aa8)?;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x001e6ab4);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x001e6ab4)?;
                        let t = true;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3b4));
                        if t {
                            break 'b_1e6ad4;
                        } else {
                            break 'b_1e6abc;
                        }
                    }
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x001e6ac4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x19d8));
                    call_ptr(c, 0x00317670, 0x001e6ac4)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e6ad0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001e6ad0)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3b4));
                    break 'b_1e6ad4;
                }
                'b_1e6b20: {
                    'b_1e6af0: {
                        c.cycles += 4;
                        c.g[A0][0] = 0x6;
                        c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x3a8), c.g[A0][0]);
                        if t {
                            break 'b_1e6af0;
                        } else {
                            c.cycles += 3;
                            c.g[A2][0] = 0x2;
                            let t = true;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x3a8), c.g[A2][0]);
                            if t {
                                break 'b_1e6b20;
                            } else {
                                break 'b_1e6af0;
                            }
                        }
                    }
                    'b_1e6b1c: {
                        'b_1e6b0c: {
                            c.cycles += 2;
                            let t = c.g[S7][0] != 0u64;
                            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3a4));
                            if t {
                                break 'b_1e6b0c;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                let t = true;
                                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x78));
                                if t {
                                    break 'b_1e6b1c;
                                } else {
                                    break 'b_1e6b0c;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[V1][0] = 0x2;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3a8));
                        c.g[V0][0] = c.g[A3][0] ^ 0x2;
                        if c.g[V0][0] == 0 {
                            c.g[A0][0] = c.g[V1][0];
                        }
                        break 'b_1e6b1c;
                    }
                    c.cycles += 1;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x3a8), c.g[A0][0]);
                    break 'b_1e6b20;
                }
                c.cycles += 14;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x394));
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001e6b30);
                c.g[S1][0] = 0u64;
                call_ptr(c, 0x00146e98, 0x001e6b30)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x394));
                c.g[A1][0] = 0x1;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x3b0), 0u64);
                c.g[RA][0] = sx(0x001e6b44);
                c.g[FP][0] = c.g[V0][0];
                call_ptr(c, 0x00146e98, 0x001e6b44)?;
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x3ac), c.g[V0][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x3b8), c.g[A1][0]);
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
                c.g[A2][0] = 0x530000;
                'l_1e6b58: loop {
                    'b_1e6e20: {
                        'b_1e6e1c: {
                            'b_1e6db4: {
                                c.cycles += 7;
                                c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x6708));
                                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3a8));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[A0][0] = ((c.g[S1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                                let t = c.g[A0][0] == 0u64;
                                c.g[S4][0] = c.lw((c.g[V1][0] as u32));
                                if t {
                                    break 'b_1e6db4;
                                } else {
                                    'b_1e6cc0: {
                                        'b_1e6c94: {
                                            'b_1e6c2c: {
                                                'b_1e6c20: {
                                                    'b_1e6be0: {
                                                        c.cycles += 22;
                                                        c.g[S0][0] =
                                                            sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V0][0] = 0x4a0000;
                                                        c.g[A2][0] = c.g[S0][0];
                                                        c.g[A1][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x2170));
                                                        c.g[A0][0] = c.g[S3][0];
                                                        c.g[RA][0] = sx(0x001e6b90);
                                                        c.g[S5][0] = c.g[S0][0];
                                                        call_ptr(c, 0x00417828, 0x001e6b90)?;
                                                        c.g[S0][0] = 0u64;
                                                        c.g[RA][0] = sx(0x001e6b9c);
                                                        c.g[A0][0] = c.g[S3][0];
                                                        call_ptr(c, 0x00317670, 0x001e6b9c)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S6][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[RA][0] = sx(0x001e6ba8);
                                                        c.g[A1][0] = c.g[V0][0];
                                                        call_ptr(c, 0x0039d860, 0x001e6ba8)?;
                                                        c.g[A0][0] = 0x530000;
                                                        c.g[V1][0] = sx((c.g[FP][0] as u32) << 2);
                                                        c.g[A1][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x6730));
                                                        c.g[A2][0] = c.g[V1][0];
                                                        c.g[A3][0] = c.g[A0][0];
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add((c.g[A1][0] as u32)));
                                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                        let t = c.g[S1][0] != c.g[A0][0];
                                                        c.g[S2][0] = c.g[V0][0];
                                                        if t {
                                                            break 'b_1e6be0;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.g[A0][0] = 0x530000;
                                                            c.g[S0][0] = 0x1;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x66a8));
                                                            let t = true;
                                                            c.g[V1][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_add((c.g[A0][0] as u32)));
                                                            if t {
                                                                break 'b_1e6c20;
                                                            } else {
                                                                break 'b_1e6be0;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 6;
                                                    c.g[V1][0] = 0x530000;
                                                    c.g[A0][0] = 0x2;
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32)
                                                            .wrapping_add(0x5bc8));
                                                    c.g[V0][0] = c
                                                        .lb((c.g[V1][0] as u32).wrapping_add(0x49));
                                                    if c.g[V0][0] != c.g[A0][0] {
                                                        c.g[A2][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x39c));
                                                        break 'b_1e6c94;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x3ac));
                                                        c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add((c.g[A1][0] as u32)));
                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                        let t = c.g[S1][0] != c.g[V1][0];
                                                        c.g[V1][0] = 0x530000;
                                                        if t {
                                                            break 'b_1e6c2c;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[A1][0] = 0x530000;
                                                            c.g[S0][0] = 0x1;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x66a8));
                                                            c.g[V1][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add((c.g[A1][0] as u32)));
                                                            break 'b_1e6c20;
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                if c.g[V0][0] == 0 {
                                                    c.g[S0][0] = 0u64;
                                                }
                                                c.g[V1][0] = 0x530000;
                                                break 'b_1e6c2c;
                                            }
                                            c.cycles += 5;
                                            c.g[A0][0] = 0x2;
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                                            c.g[V0][0] =
                                                c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                                            if c.g[V0][0] != c.g[A0][0] {
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x39c));
                                                break 'b_1e6c94;
                                            } else {
                                                'b_1e6c78: {
                                                    'b_1e6c60: {
                                                        c.cycles += 5;
                                                        c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                            .wrapping_add(0x6730));
                                                        c.g[V0][0] = sx((c.g[A2][0] as u32)
                                                            .wrapping_add((c.g[A0][0] as u32)));
                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                        if c.g[S1][0] != c.g[V1][0] {
                                                            c.g[A1][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x3ac));
                                                            break 'b_1e6c60;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x394));
                                                            let t = true;
                                                            c.g[A1][0] = c.g[FP][0];
                                                            if t {
                                                                break 'b_1e6c78;
                                                            } else {
                                                                break 'b_1e6c60;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                    let t = c.g[S1][0] != c.g[V1][0];
                                                    c.g[A2][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x39c)
                                                        );
                                                    if t {
                                                        break 'b_1e6c94;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x394));
                                                        break 'b_1e6c78;
                                                    }
                                                }
                                                c.cycles += 7;
                                                c.g[RA][0] = sx(0x001e6c80);
                                                call_ptr(c, 0x00146f88, 0x001e6c80)?;
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[RA][0] = sx(0x001e6c8c);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, 0x003a0c50, 0x001e6c8c)?;
                                                let t = true;
                                                c.g[A3][0] = 0x470000;
                                                if t {
                                                    break 'b_1e6cc0;
                                                } else {
                                                    break 'b_1e6c94;
                                                }
                                            }
                                        }
                                        c.cycles += 11;
                                        c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x398));
                                        c.g[V1][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[V0][0] = c.g[V1][0];
                                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
                                        c.g[RA][0] = sx(0x001e6cb0);
                                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                                        call_ptr(c, 0x0014eec8, 0x001e6cb0)?;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[RA][0] = sx(0x001e6cbc);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x003a0c50, 0x001e6cbc)?;
                                        c.g[A3][0] = 0x470000;
                                        break 'b_1e6cc0;
                                    }
                                    'b_1e6d08: {
                                        c.cycles += 11;
                                        c.g[A2][0] = c.g[S5][0];
                                        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x19c8));
                                        c.g[RA][0] = sx(0x001e6cd0);
                                        c.g[A0][0] = c.g[S3][0];
                                        call_ptr(c, 0x00417828, 0x001e6cd0)?;
                                        c.g[RA][0] = sx(0x001e6cd8);
                                        c.g[A0][0] = c.g[S3][0];
                                        call_ptr(c, 0x00317670, 0x001e6cd8)?;
                                        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                        c.g[RA][0] = sx(0x001e6ce4);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x001e6ce4)?;
                                        let t = c.g[S0][0] == 0u64;
                                        c.g[S2][0] = c.g[V0][0];
                                        if t {
                                            break 'b_1e6d08;
                                        } else {
                                            c.cycles += 7;
                                            c.g[V0][0] = 0x4a0000;
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x001e6cfc);
                                            c.g[A1][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x2160));
                                            call_ptr(c, 0x003a0c50, 0x001e6cfc)?;
                                            c.g[V1][0] = 0x1;
                                            let t = true;
                                            c.sw(
                                                (c.g[SP][0] as u32).wrapping_add(0x3b0),
                                                c.g[V1][0],
                                            );
                                            if t {
                                                break 'b_1e6e1c;
                                            } else {
                                                break 'b_1e6d08;
                                            }
                                        }
                                    }
                                    'b_1e6d80: {
                                        'b_1e6d18: {
                                            c.cycles += 2;
                                            let t = c.g[S7][0] == 0u64;
                                            c.g[V0][0] = 0x5;
                                            if t {
                                                break 'b_1e6d18;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[S7][0] != c.g[V0][0];
                                                c.g[A2][0] = 0x530000;
                                                if t {
                                                    break 'b_1e6d80;
                                                } else {
                                                    break 'b_1e6d18;
                                                }
                                            }
                                        }
                                        'b_1e6d58: {
                                            'b_1e6d44: {
                                                c.cycles += 7;
                                                c.g[A0][0] = 0x530000;
                                                c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x6640));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                                                let t = (c.g[V0][0] as i64) < 0;
                                                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                                                if t {
                                                    break 'b_1e6d44;
                                                } else {
                                                    c.cycles += 4;
                                                    c.mtc1(2, 0);
                                                    c.cvt_s_w(0, 0);
                                                    let t = true;

                                                    if t {
                                                        break 'b_1e6d58;
                                                    } else {
                                                        break 'b_1e6d44;
                                                    }
                                                }
                                            }
                                            c.cycles += 5;
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                            c.mtc1(2, 0);
                                            c.cvt_s_w(0, 0);
                                            c.add_s(0, 0, 0);
                                            break 'b_1e6d58;
                                        }
                                        c.cycles += 10;
                                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5794));
                                        c.g[RA][0] = sx(0x001e6d64);
                                        c.mul_s(12, 0, 12);
                                        call_ptr(c, 0x004139f8, 0x001e6d64)?;
                                        c.g[RA][0] = sx(0x001e6d6c);
                                        c.g[A0][0] = c.g[V0][0];
                                        call_ptr(c, 0x0020a1a8, 0x001e6d6c)?;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[RA][0] = sx(0x001e6d78);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x003a0c50, 0x001e6d78)?;
                                        let t = true;
                                        c.g[S1][0] = c.g[S5][0];
                                        if t {
                                            break 'b_1e6e20;
                                        } else {
                                            break 'b_1e6d80;
                                        }
                                    }
                                    c.cycles += 13;
                                    c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x6640));
                                    c.g[A1][0] = 0x4a0000;
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2138));
                                    c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                                    c.g[RA][0] = sx(0x001e6da0);
                                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
                                    call_ptr(c, 0x00417828, 0x001e6da0)?;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x001e6dac);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
                                    call_ptr(c, 0x003a0c50, 0x001e6dac)?;
                                    let t = true;
                                    c.g[S1][0] = c.g[S5][0];
                                    if t {
                                        break 'b_1e6e20;
                                    } else {
                                        break 'b_1e6db4;
                                    }
                                }
                            }
                            c.cycles += 26;
                            c.g[A3][0] = 0x4a0000;
                            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2170));
                            c.g[A2][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001e6dcc);
                            c.g[A0][0] = c.g[S3][0];
                            call_ptr(c, 0x00417828, 0x001e6dcc)?;
                            c.g[S5][0] = c.g[S0][0];
                            c.g[A0][0] = c.g[S6][0];
                            c.g[RA][0] = sx(0x001e6ddc);
                            c.g[A1][0] = c.g[S3][0];
                            call_ptr(c, 0x0039e8b8, 0x001e6ddc)?;
                            c.g[V0][0] = 0x470000;
                            c.g[A2][0] = c.g[S0][0];
                            c.g[A0][0] = c.g[S3][0];
                            c.g[RA][0] = sx(0x001e6df0);
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x19c8));
                            call_ptr(c, 0x00417828, 0x001e6df0)?;
                            c.g[A1][0] = c.g[S3][0];
                            c.g[RA][0] = sx(0x001e6dfc);
                            c.g[A0][0] = c.g[S6][0];
                            call_ptr(c, 0x0039e8b8, 0x001e6dfc)?;
                            c.g[A1][0] = 0x4a0000;
                            c.g[A0][0] = c.g[S3][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2178));
                            c.g[RA][0] = sx(0x001e6e10);
                            c.g[A2][0] = c.g[S0][0];
                            call_ptr(c, 0x00417828, 0x001e6e10)?;
                            c.g[A0][0] = c.g[S6][0];
                            c.g[RA][0] = sx(0x001e6e1c);
                            c.g[A1][0] = c.g[S3][0];
                            call_ptr(c, 0x0039e8b8, 0x001e6e1c)?;
                            break 'b_1e6e1c;
                        }
                        c.cycles += 1;
                        c.g[S1][0] = c.g[S5][0];
                        break 'b_1e6e20;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 6i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = 0x530000;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001e6b58);
                        }
                        continue 'l_1e6b58;
                    } else {
                        'b_1e6ea4: {
                            c.cycles += 16;
                            c.g[V0][0] = 0x470000;
                            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x19b8));
                            c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
                            c.ldr(4, (c.g[A3][0] as u32));
                            c.ldl(5, (c.g[A3][0] as u32).wrapping_add(0xf));
                            c.ldr(5, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
                            c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
                            c.sdr(4, (c.g[SP][0] as u32));
                            c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0xf));
                            c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                            c.g[V1][0] = 0x2;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3a4));
                            let t = c.g[V0][0] != c.g[V1][0];
                            c.g[S3][0] = 0u64;
                            if t {
                                break 'b_1e6ea4;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S7][0] == 0u64;
                                c.g[V0][0] = 0x5;
                                if t {
                                    break 'b_1e6ea4;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[S7][0] == c.g[V0][0];
                                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3ac));
                                    if t {
                                        break 'b_1e6ea4;
                                    } else {
                                        c.cycles += 10;
                                        c.g[V1][0] = 0x530000;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
                                        c.g[A1][0] = sx((c.g[FP][0] as u32) << 2);
                                        c.g[A0][0] = sx((c.g[A2][0] as u32) << 2);
                                        c.g[A1][0] =
                                            sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[A0][0] =
                                            sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                                        c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                        c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                                        c.g[S3][0] = (c.g[V0][0] < 0x1u64) as u64;
                                        break 'b_1e6ea4;
                                    }
                                }
                            }
                        }
                        'b_1e7128: {
                            'b_1e7124: {
                                'b_1e702c: {
                                    c.cycles += 11;
                                    c.g[A0][0] = 0x470000;
                                    c.g[RA][0] = sx(0x001e6eb0);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x19a0));
                                    call_ptr(c, 0x00317670, 0x001e6eb0)?;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001e6ebc);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001e6ebc)?;
                                    c.g[S2][0] = c.g[V0][0];
                                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3a4));
                                    c.g[V0][0] = 0x2;
                                    let t = c.g[A3][0] == c.g[V0][0];
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3b4));
                                    if t {
                                        break 'b_1e702c;
                                    } else {
                                        'b_1e7014: {
                                            'b_1e6ffc: {
                                                'b_1e6fe0: {
                                                    'b_1e6f08: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = (c.g[V1][0] < 0x2u64) as u64;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = 0x530000;
                                                        if t {
                                                            break 'b_1e6f08;
                                                        } else {
                                                            'b_1e6f00: {
                                                                c.cycles += 6;
                                                                c.g[V1][0] =
                                                                    sx((c.g[FP][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x6730));
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V0][0] as u32),
                                                                    ));
                                                                c.g[A0][0] =
                                                                    c.lw((c.g[V1][0] as u32));
                                                                if c.g[A0][0] != 0u64 {
                                                                    c.g[A0][0] = 0x470000;
                                                                    break 'b_1e6f00;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A0][0] = 0x470000;
                                                                    let t = true;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_sub(0x1990));
                                                                    if t {
                                                                        break 'b_1e6fe0;
                                                                    } else {
                                                                        break 'b_1e6f00;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0x1978));
                                                            if t {
                                                                break 'b_1e6fe0;
                                                            } else {
                                                                break 'b_1e6f08;
                                                            }
                                                        }
                                                    }
                                                    'b_1e6f84: {
                                                        c.cycles += 2;
                                                        let t = c.g[S7][0] != 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0xfb4));
                                                        if t {
                                                            break 'b_1e6f84;
                                                        } else {
                                                            'b_1e6f80: {
                                                                c.cycles += 4;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x3a0));
                                                                c.g[V0][0] = ((c.g[A0][0] as i64)
                                                                    < 3i64)
                                                                    as u64;
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[V0][0] = 0x530000;
                                                                if t {
                                                                    break 'b_1e6f80;
                                                                } else {
                                                                    'b_1e6f60: {
                                                                        c.cycles += 7;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[FP][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x6730,
                                                                                ));
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            c.lw(
                                                                                (c.g[A0][0] as u32)
                                                                            );
                                                                        c.g[V1][0] = ((c.g[V1][0]
                                                                            as i64)
                                                                            < 3i64)
                                                                            as u64;
                                                                        let t = c.g[V1][0] == 0u64;
                                                                        c.g[V0][0] = 0x1;
                                                                        if t {
                                                                            break 'b_1e6f60;
                                                                        } else {
                                                                            'b_1e6f54: {
                                                                                c.cycles += 3;
                                                                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3a0));
                                                                                let t = c.g[A1][0]
                                                                                    != c.g[V0][0];
                                                                                c.sw(
                                                                                    (c.g[SP][0]
                                                                                        as u32),
                                                                                    c.g[V0][0],
                                                                                );
                                                                                if t {
                                                                                    break 'b_1e6f54;
                                                                                } else {
                                                                                    c.cycles += 3;
                                                                                    c.g[A0][0] =
                                                                                        0x470000;
                                                                                    let t = true;
                                                                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1960));
                                                                                    if t {
                                                                                        break 'b_1e6fe0;
                                                                                    } else {
                                                                                        break 'b_1e6f54;
                                                                                    }
                                                                                }
                                                                            }
                                                                            c.cycles += 3;
                                                                            c.g[A0][0] = 0x470000;
                                                                            let t = true;
                                                                            c.g[A0][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x1948,
                                                                                    ));
                                                                            if t {
                                                                                break 'b_1e6fe0;
                                                                            } else {
                                                                                break 'b_1e6f60;
                                                                            }
                                                                        }
                                                                    }
                                                                    'b_1e6f78: {
                                                                        c.cycles += 3;
                                                                        c.g[A2][0] = c.lw((c.g[SP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x3a0));
                                                                        if c.g[A2][0] != c.g[V0][0]
                                                                        {
                                                                            c.g[A0][0] = 0x470000;
                                                                            break 'b_1e6f78;
                                                                        } else {
                                                                            c.cycles += 3;
                                                                            c.g[A0][0] = 0x470000;
                                                                            let t = true;
                                                                            c.g[A0][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x1930,
                                                                                    ));
                                                                            if t {
                                                                                break 'b_1e6fe0;
                                                                            } else {
                                                                                break 'b_1e6f78;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_sub(0x1910));
                                                                    if t {
                                                                        break 'b_1e6fe0;
                                                                    } else {
                                                                        break 'b_1e6f80;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 1;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0xfb4));
                                                            break 'b_1e6f84;
                                                        }
                                                    }
                                                    'b_1e6f94: {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = 0x470000;
                                                        if t {
                                                            break 'b_1e6f94;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0x18f8));
                                                            if t {
                                                                break 'b_1e6fe0;
                                                            } else {
                                                                break 'b_1e6f94;
                                                            }
                                                        }
                                                    }
                                                    'b_1e6fbc: {
                                                        c.cycles += 7;
                                                        c.g[V0][0] = 0x530000;
                                                        c.g[V1][0] = sx((c.g[FP][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x6730));
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[V1][0] = c.lw((c.g[V1][0] as u32));
                                                        let t = c.g[V1][0] != 0u64;
                                                        c.g[V0][0] = 0x1;
                                                        if t {
                                                            break 'b_1e6fbc;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A0][0] = 0x470000;
                                                            let t = true;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0x18d8));
                                                            if t {
                                                                break 'b_1e6fe0;
                                                            } else {
                                                                break 'b_1e6fbc;
                                                            }
                                                        }
                                                    }
                                                    'b_1e6fd0: {
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] != c.g[V0][0];
                                                        c.g[V0][0] = 0x2;
                                                        if t {
                                                            break 'b_1e6fd0;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A0][0] = 0x470000;
                                                            let t = true;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0x18b8));
                                                            if t {
                                                                break 'b_1e6fe0;
                                                            } else {
                                                                break 'b_1e6fd0;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] != c.g[V0][0];
                                                    c.g[A0][0] = 0x470000;
                                                    if t {
                                                        break 'b_1e6ffc;
                                                    } else {
                                                        c.cycles += 2;
                                                        c.g[A0][0] = 0x470000;
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_sub(0x1898));
                                                        break 'b_1e6fe0;
                                                    }
                                                }
                                                c.cycles += 7;
                                                c.g[RA][0] = sx(0x001e6fe8);
                                                call_ptr(c, 0x00317670, 0x001e6fe8)?;
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[RA][0] = sx(0x001e6ff4);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, 0x003a1000, 0x001e6ff4)?;
                                                let t = true;
                                                c.g[A3][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x3a4));
                                                if t {
                                                    break 'b_1e7014;
                                                } else {
                                                    break 'b_1e6ffc;
                                                }
                                            }
                                            c.cycles += 6;
                                            c.g[RA][0] = sx(0x001e7004);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x1978));
                                            call_ptr(c, 0x00317670, 0x001e7004)?;
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x001e7010);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, 0x003a1000, 0x001e7010)?;
                                            c.g[A3][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x3a4));
                                            break 'b_1e7014;
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x1;
                                        let t = c.g[A3][0] != c.g[V0][0];
                                        c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3b0));
                                        if t {
                                            break 'b_1e7128;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x3;
                                            let t = true;
                                            c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                                            if t {
                                                break 'b_1e7124;
                                            } else {
                                                break 'b_1e702c;
                                            }
                                        }
                                    }
                                }
                                'b_1e7078: {
                                    'b_1e705c: {
                                        c.cycles += 9;
                                        c.g[V1][0] = 0x3;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3a4));
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
                                        if c.g[A0][0] != c.g[V0][0] {
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                            break 'b_1e705c;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[S3][0] == 0u64;
                                            c.g[A2][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x3ac));
                                            if t {
                                                break 'b_1e7078;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                break 'b_1e705c;
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001e7070);
                                    c.g[A0][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001e7070)?;
                                    let t = true;
                                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3b0));
                                    if t {
                                        break 'b_1e7128;
                                    } else {
                                        break 'b_1e7078;
                                    }
                                }
                                'b_1e70d8: {
                                    'b_1e70c0: {
                                        c.cycles += 11;
                                        c.g[V1][0] = 0x530000;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6730));
                                        c.g[A1][0] = sx((c.g[FP][0] as u32) << 2);
                                        c.g[A0][0] = sx((c.g[A2][0] as u32) << 2);
                                        c.g[A1][0] =
                                            sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[A0][0] =
                                            sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                                        c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                        c.g[V0][0] =
                                            ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x394));
                                        if t {
                                            break 'b_1e70c0;
                                        } else {
                                            c.cycles += 7;
                                            c.g[RA][0] = sx(0x001e70ac);
                                            c.g[A1][0] = c.g[FP][0];
                                            call_ptr(c, 0x00146f88, 0x001e70ac)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x3b8));
                                            c.g[RA][0] = sx(0x001e70b8);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, 0x002c2540, 0x001e70b8)?;
                                            let t = true;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            if t {
                                                break 'b_1e70d8;
                                            } else {
                                                break 'b_1e70c0;
                                            }
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[RA][0] = sx(0x001e70c8);
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3ac));
                                    call_ptr(c, 0x00146f88, 0x001e70c8)?;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3b8));
                                    c.g[RA][0] = sx(0x001e70d4);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x002c2540, 0x001e70d4)?;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    break 'b_1e70d8;
                                }
                                c.cycles += 19;
                                c.g[A0][0] = 0x470000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1878));
                                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                c.g[RA][0] = sx(0x001e70f4);
                                c.g[S0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                call_ptr(c, 0x00317670, 0x001e70f4)?;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                c.g[A0][0] = c.g[S0][0];
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001e7104);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, t, 0x001e7104)?;
                                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
                                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3b8));
                                c.g[A1][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001e7118);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x002c26d0, 0x001e7118)?;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[RA][0] = sx(0x001e7124);
                                c.g[A1][0] = c.g[S0][0];
                                call_ptr(c, 0x003a0d00, 0x001e7124)?;
                                break 'b_1e7124;
                            }
                            c.cycles += 1;
                            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3b0));
                            break 'b_1e7128;
                        }
                        'b_1e7148: {
                            c.cycles += 2;
                            let t = c.g[A3][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3a4));
                            if t {
                                break 'b_1e7148;
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = 0x2;
                                let t = c.g[V0][0] == c.g[V1][0];
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3a4));
                                if t {
                                    break 'b_1e7148;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x1;
                                    if c.g[A0][0] != c.g[V0][0] {
                                        c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                                        break 'b_1e7148;
                                    } else {
                                        break 'b_1e7148;
                                    }
                                }
                            }
                        }
                        'b_1e71b8: {
                            c.cycles += 19;
                            c.g[A2][0] = 0x4a0000;
                            c.g[A0][0] = c.g[S6][0];
                            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2180));
                            c.g[RA][0] = sx(0x001e715c);
                            c.g[S1][0] = 0u64;
                            call_ptr(c, 0x0039e8b8, 0x001e715c)?;
                            c.g[A3][0] = 0x4a0000;
                            c.g[A0][0] = c.g[S6][0];
                            c.g[RA][0] = sx(0x001e716c);
                            c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2188));
                            call_ptr(c, 0x0039e8b8, 0x001e716c)?;
                            c.g[V0][0] = 0x4a0000;
                            c.g[A0][0] = c.g[S6][0];
                            c.g[RA][0] = sx(0x001e717c);
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2190));
                            call_ptr(c, 0x0039e8b8, 0x001e717c)?;
                            c.g[V1][0] = 0x530000;
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                            c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                            c.g[V1][0] = 0x2;
                            let t = c.g[A0][0] == c.g[V1][0];
                            c.g[A2][0] = 0x1;
                            if t {
                                break 'b_1e71b8;
                            } else {
                                c.cycles += 4;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3b4));
                                c.g[V0][0] = (c.g[A0][0] < 0x2u64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x530000;
                                if t {
                                    break 'b_1e71b8;
                                } else {
                                    c.cycles += 5;
                                    c.g[V1][0] = sx((c.g[FP][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                    if c.g[A0][0] != 0 {
                                        c.g[A2][0] = 0u64;
                                    }
                                    break 'b_1e71b8;
                                }
                            }
                        }
                        'b_1e7230: {
                            c.cycles += 4;
                            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3ac));
                            c.g[V0][0] = 0xffffffffffffffff;
                            let t = c.g[A1][0] == c.g[V0][0];
                            c.g[A3][0] = 0x530000;
                            if t {
                                break 'b_1e7230;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = 0x2;
                                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5bc8));
                                c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                                let t = c.g[V1][0] != c.g[A0][0];
                                c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x1));
                                if t {
                                    break 'b_1e7230;
                                } else {
                                    'b_1e71fc: {
                                        c.cycles += 3;
                                        c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[S1][0] = 0x1;
                                        if t {
                                            break 'b_1e71fc;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x5;
                                            let t = c.g[S7][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x6;
                                            if t {
                                                break 'b_1e71fc;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[S7][0] != c.g[V0][0];

                                                if t {
                                                    break 'b_1e7230;
                                                } else {
                                                    break 'b_1e71fc;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 12;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3ac));
                                    c.g[V1][0] = 0x530000;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6730));
                                    c.g[A1][0] = sx((c.g[FP][0] as u32) << 2);
                                    c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[A1][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[A2][0] = 0u64;
                                        break 'b_1e7230;
                                    } else {
                                        c.cycles += 1;
                                        c.g[S1][0] = 0u64;
                                        break 'b_1e7230;
                                    }
                                }
                            }
                        }
                        'b_1e7240: {
                            c.cycles += 2;
                            let t = c.g[S3][0] == 0u64;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3a0));
                            if t {
                                break 'b_1e7240;
                            } else {
                                c.cycles += 2;
                                c.g[A2][0] = 0u64;
                                c.g[S1][0] = 0u64;
                                break 'b_1e7240;
                            }
                        }
                        'b_1e7394: {
                            'b_1e725c: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x3;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = 0x5;
                                if t {
                                    break 'b_1e725c;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[S7][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x6;
                                    if t {
                                        break 'b_1e725c;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S7][0] != c.g[V0][0];
                                        c.g[V0][0] = 0x8;
                                        if t {
                                            break 'b_1e7394;
                                        } else {
                                            break 'b_1e725c;
                                        }
                                    }
                                }
                            }
                            'b_1e731c: {
                                c.cycles += 2;
                                let t = c.g[A2][0] == 0u64;
                                c.g[V0][0] = 0x530000;
                                if t {
                                    break 'b_1e731c;
                                } else {
                                    'b_1e72d0: {
                                        'b_1e7294: {
                                            c.cycles += 6;
                                            c.g[V1][0] = sx((c.g[FP][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x6730));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[V1][0] = c.lw((c.g[V1][0] as u32));
                                            let t = c.g[V1][0] != 0u64;
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_1e7294;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A2][0] = 0x4a0000;
                                                c.g[A0][0] = c.g[S6][0];
                                                c.g[RA][0] = sx(0x001e728c);
                                                c.g[A1][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_add(0x2180));
                                                call_ptr(c, 0x0039e948, 0x001e728c)?;
                                                let t = true;
                                                c.g[V1][0] = 0x530000;
                                                if t {
                                                    break 'b_1e72d0;
                                                } else {
                                                    break 'b_1e7294;
                                                }
                                            }
                                        }
                                        'b_1e72b4: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x2;
                                            if t {
                                                break 'b_1e72b4;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A3][0] = 0x4a0000;
                                                c.g[A0][0] = c.g[S6][0];
                                                c.g[RA][0] = sx(0x001e72ac);
                                                c.g[A1][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x2188));
                                                call_ptr(c, 0x0039e948, 0x001e72ac)?;
                                                let t = true;
                                                c.g[V1][0] = 0x530000;
                                                if t {
                                                    break 'b_1e72d0;
                                                } else {
                                                    break 'b_1e72b4;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[V1][0] = 0x530000;
                                        if t {
                                            break 'b_1e72d0;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V0][0] = 0x4a0000;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[RA][0] = sx(0x001e72cc);
                                            c.g[A1][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x2190));
                                            call_ptr(c, 0x0039e948, 0x001e72cc)?;
                                            c.g[V1][0] = 0x530000;
                                            break 'b_1e72d0;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                                    let t = c.g[V1][0] != 0u64;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xfb4));
                                    if t {
                                        break 'b_1e731c;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_1e731c;
                                        } else {
                                            c.cycles += 13;
                                            c.g[A1][0] = 0x4a0000;
                                            c.g[RA][0] = sx(0x001e72f4);
                                            c.g[A0][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x2180));
                                            call_ptr(c, 0x00317670, 0x001e72f4)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                            c.g[RA][0] = sx(0x001e7300);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, 0x0039d860, 0x001e7300)?;
                                            c.g[A0][0] = 0x470000;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x001e7310);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x1860));
                                            call_ptr(c, 0x00317670, 0x001e7310)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x001e731c);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, 0x00399730, 0x001e731c)?;
                                            break 'b_1e731c;
                                        }
                                    }
                                }
                            }
                            'b_1e7390: {
                                c.cycles += 2;
                                let t = c.g[S1][0] == 0u64;
                                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3ac));
                                if t {
                                    break 'b_1e7390;
                                } else {
                                    'b_1e7358: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x530000;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
                                        c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[V1][0] = c.lw((c.g[V1][0] as u32));
                                        let t = c.g[V1][0] != 0u64;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_1e7358;
                                        } else {
                                            c.cycles += 6;
                                            c.g[A3][0] = 0x4a0000;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[RA][0] = sx(0x001e7350);
                                            c.g[A1][0] =
                                                sx((c.g[A3][0] as u32).wrapping_add(0x2180));
                                            call_ptr(c, 0x0039e948, 0x001e7350)?;
                                            let t = true;
                                            c.g[V0][0] = 0x8;
                                            if t {
                                                break 'b_1e7394;
                                            } else {
                                                break 'b_1e7358;
                                            }
                                        }
                                    }
                                    'b_1e7378: {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[V0][0] = 0x2;
                                        if t {
                                            break 'b_1e7378;
                                        } else {
                                            c.cycles += 6;
                                            c.g[V0][0] = 0x4a0000;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[RA][0] = sx(0x001e7370);
                                            c.g[A1][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x2188));
                                            call_ptr(c, 0x0039e948, 0x001e7370)?;
                                            let t = true;
                                            c.g[V0][0] = 0x8;
                                            if t {
                                                break 'b_1e7394;
                                            } else {
                                                break 'b_1e7378;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[V0][0] = 0x8;
                                    if t {
                                        break 'b_1e7394;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V1][0] = 0x4a0000;
                                        c.g[A0][0] = c.g[S6][0];
                                        c.g[RA][0] = sx(0x001e7390);
                                        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2190));
                                        call_ptr(c, 0x0039e948, 0x001e7390)?;
                                        break 'b_1e7390;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = 0x8;
                            break 'b_1e7394;
                        }
                        c.cycles += 7;
                        c.g[V1][0] = 0x440000;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x390), c.g[V0][0]);
                        c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1d10));
                        c.g[S4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3b8));
                        c.g[S2][0] = 0u64;
                        c.g[S3][0] = c.g[SP][0];
                        c.g[S7][0] = 0x470000;
                        'l_1e73b0: loop {
                            'b_1e7518: {
                                'b_1e7514: {
                                    c.cycles += 13;
                                    c.g[A0][0] = 0x470000;
                                    c.g[A2][0] = c.g[S2][0];
                                    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1b40));
                                    c.g[RA][0] = sx(0x001e73c4);
                                    c.g[A0][0] = c.g[S4][0];
                                    call_ptr(c, 0x00417828, 0x001e73c4)?;
                                    c.g[RA][0] = sx(0x001e73cc);
                                    c.g[A0][0] = c.g[S4][0];
                                    call_ptr(c, 0x00317670, 0x001e73cc)?;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001e73d8);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001e73d8)?;
                                    c.g[S1][0] = c.g[V0][0];
                                    let t = c.g[S1][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    if t {
                                        break 'b_1e7514;
                                    } else {
                                        'b_1e7474: {
                                            c.cycles += 7;
                                            c.g[A1][0] = 0x7;
                                            c.g[RA][0] = sx(0x001e73f0);
                                            c.g[A0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                                            call_ptr(c, 0x0014dd58, 0x001e73f0)?;
                                            c.g[V1][0] = 0x530000;
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = ((c.g[S2][0] as i64) < 5i64) as u64;
                                            if t {
                                                break 'b_1e7474;
                                            } else {
                                                'b_1e7440: {
                                                    c.cycles += 2;
                                                    if (c.g[S2][0] as i64) > 0 {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x8));
                                                        break 'b_1e7440;
                                                    } else {
                                                        c.cycles += 14;
                                                        c.g[V1][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x390));
                                                        c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
                                                        c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                            .wrapping_add((c.g[V1][0] as u32)));
                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add((c.g[S5][0] as u32)));
                                                        c.g[RA][0] = sx(0x001e7428);
                                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                                        call_ptr(c, 0x00317670, 0x001e7428)?;
                                                        c.g[A1][0] = c.g[V0][0];
                                                        c.g[RA][0] = sx(0x001e7434);
                                                        c.g[A0][0] = c.g[S1][0];
                                                        call_ptr(c, 0x003a1000, 0x001e7434)?;
                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x18),
                                                            c.g[V0][0],
                                                        );
                                                        if t {
                                                            break 'b_1e7514;
                                                        } else {
                                                            break 'b_1e7440;
                                                        }
                                                    }
                                                }
                                                c.cycles += 13;
                                                c.g[A1][0] = 0u64;
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x001e7454);
                                                c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001e7454)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                c.g[A1][0] = 0x1;
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x001e746c);
                                                c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001e746c)?;
                                                let t = true;
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    break 'b_1e7518;
                                                } else {
                                                    break 'b_1e7474;
                                                }
                                            }
                                        }
                                        'b_1e74e8: {
                                            c.cycles += 2;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                break 'b_1e74e8;
                                            } else {
                                                'b_1e74e0: {
                                                    c.cycles += 10;
                                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[S5][0] as u32)));
                                                    c.g[RA][0] = sx(0x001e7490);
                                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                                    call_ptr(c, 0x00317670, 0x001e7490)?;
                                                    c.g[A1][0] = c.g[V0][0];
                                                    c.g[RA][0] = sx(0x001e749c);
                                                    c.g[A0][0] = c.g[S1][0];
                                                    call_ptr(c, 0x003a1000, 0x001e749c)?;
                                                    if c.g[S2][0] != 0u64 {
                                                        c.g[V0][0] = c.lw((c.g[S3][0] as u32));
                                                        break 'b_1e74e0;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32));
                                                        c.g[V0][0] = 0x1;
                                                        if c.g[V1][0] != c.g[V0][0] {
                                                            c.g[V0][0] = c.lw((c.g[S3][0] as u32));
                                                            break 'b_1e74e0;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.g[A1][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x39c));
                                                            c.g[V0][0] = 0x3;
                                                            c.g[V1][0] = c.lw((c.g[A1][0] as u32)
                                                                .wrapping_add(0x70));
                                                            if c.g[V1][0] != c.g[V0][0] {
                                                                c.g[V0][0] =
                                                                    c.lw((c.g[S3][0] as u32));
                                                                break 'b_1e74e0;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.g[RA][0] = sx(0x001e74d0);
                                                                c.g[A0][0] = sx((c.g[S7][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x1b30));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001e74d0,
                                                                )?;
                                                                c.g[A1][0] = c.g[V0][0];
                                                                c.g[RA][0] = sx(0x001e74dc);
                                                                c.g[A0][0] = c.g[S1][0];
                                                                call_ptr(
                                                                    c, 0x003a1000, 0x001e74dc,
                                                                )?;
                                                                c.g[V0][0] =
                                                                    c.lw((c.g[S3][0] as u32));
                                                                break 'b_1e74e0;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = true;
                                                c.sw(
                                                    (c.g[S1][0] as u32).wrapping_add(0x18),
                                                    c.g[V0][0],
                                                );
                                                if t {
                                                    break 'b_1e7514;
                                                } else {
                                                    break 'b_1e74e8;
                                                }
                                            }
                                        }
                                        c.cycles += 11;
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001e74fc);
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001e74fc)?;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0x1;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001e7514);
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001e7514)?;
                                        break 'b_1e7514;
                                    }
                                }
                                c.cycles += 1;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                break 'b_1e7518;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S2][0] as i64) < 5i64) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001e73b0);
                                }
                                continue 'l_1e73b0;
                            } else {
                                c.cycles += 1;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x450));
                                break 'b_1e7528;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 12;
        c.g[V0][0] = 0u64;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x440));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x430));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x420));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x410));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x400));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x3f0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x3e0));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x3d0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3c0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x460));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenHiscoreList_Init` (0x001e8ff0, `cOVStateHiScoreList_onCreateScreen`): adds the hi-score
/// screen to the UI engine and plays it, then shows or hides its objects by the score-table kind
/// (byte at 0x535c10, jump table at 0x46ea30) and updates the HUD when no score rows exist.
/// Converted from the generated body (every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_hiscore_list_init(c: &mut Cpu) {
    if c.pc != 0x001e8ff0 {
        return ee::s04::f_001e8ff0__generated(c);
    }
    guest(c, |c| 'l_1e8ff0: loop {
        'b_1e9040: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = 0x470000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1628));
            c.g[RA][0] = sx(0x001e9014);
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001e9014)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001e9028);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001e9028)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1e9040;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001e9040);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001e9040)?;
                break 'b_1e9040;
            }
        }
        'b_1e90b0: {
            'b_1e90a0: {
                c.cycles += 12;
                c.g[RA][0] = sx(0x001e9048);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0020a380, 0x001e9048)?;
                c.g[RA][0] = sx(0x001e9050);
                call_ptr(c, 0x0014dc80, 0x001e9050)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001e905c);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x001e905c)?;
                c.g[V1][0] = 0x530000;
                c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c10));
                c.g[V0][0] = (c.g[V1][0] < 0x7u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x470000;
                if t {
                    break 'b_1e90a0;
                } else {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x15d0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;
                    match t {
                        0x001e8ff0 => {
                            if event_due(c) {
                                deliver_event_here(c, 0x001e8ff0);
                            }
                            continue 'l_1e8ff0;
                        }
                        0x001e9088 => {
                            c.cycles += 6;
                            c.g[A1][0] = 0x470000;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001e9098);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1618));
                            call_ptr(c, 0x0039e948, 0x001e9098)?;
                            let t = true;

                            if t {
                                break 'b_1e90b0;
                            } else {
                                break 'b_1e90a0;
                            }
                        }
                        0x001e90a0 => {
                            break 'b_1e90a0;
                        }
                        _ => {
                            c.jump_out(t, 0x001e8ff0, 0x001e9130);
                            return Ok(());
                        }
                    }
                }
            }
            c.cycles += 4;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001e90b0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1608));
            call_ptr(c, 0x0039e8b8, 0x001e90b0)?;
            break 'b_1e90b0;
        }
        'b_1e90e4: {
            'b_1e90e0: {
                c.cycles += 9;
                c.g[RA][0] = sx(0x001e90b8);
                call_ptr(c, 0x0014dc80, 0x001e90b8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001e90c4);
                c.g[A1][0] = 0x8;
                call_ptr(c, 0x0014dd58, 0x001e90c4)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                let t = (c.g[V0][0] as i64) >= 0;
                c.g[A0][0] = 0u64;
                if t {
                    break 'b_1e90e0;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    let t = (c.g[V0][0] as i64) < 0;

                    if t {
                        break 'b_1e90e4;
                    } else {
                        break 'b_1e90e0;
                    }
                }
            }
            c.cycles += 1;
            c.g[A0][0] = 0x1;
            break 'b_1e90e4;
        }
        'b_1e9108: {
            c.cycles += 2;
            let t = c.g[A0][0] != 0u64;
            c.g[A1][0] = 0x470000;
            if t {
                break 'b_1e9108;
            } else {
                c.cycles += 7;
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x001e90f8);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x15f8));
                call_ptr(c, 0x0039e8b8, 0x001e90f8)?;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x001e9108);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x15e8));
                call_ptr(c, 0x0039e8b8, 0x001e9108)?;
                break 'b_1e9108;
            }
        }
        c.cycles += 10;
        c.g[RA][0] = sx(0x001e9110);
        call_ptr(c, 0x0028b180, 0x001e9110)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001e911c);
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x00294f78, 0x001e911c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{elf_image, fill, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The fakes' gate word and its magic (each machine of a diff gets its own).
    const OC960_CTRL: u32 = DATA + 0x2f_0000;
    const OC960_MAGIC: u32 = 0x4f43_3936;
    /// Forced results: entry `i` is `{flag, value}` (i = index in the `oc960_fakes!` table).
    const OC960_MBOX: u32 = DATA + 0x2f_0100;
    /// "Pointer soup": words that are either small numbers or pointers back into the soup, so
    /// chains of loads from fixture data stay inside readable memory.
    const OC960_SOUP: u32 = DATA + 0x4000;
    const OC960_SOUP_BYTES: u32 = 0x1000;

    macro_rules! oc960_fakes {
        ($install:ident; $(($idx:expr, $fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC960_CTRL) != OC960_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc960_fake_call(c, $idx, $addr);
                }
            )*
            fn $install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    /// A logged fake call; the result is the forced one of mailbox `idx`, else a pointer into
    /// the soup picked by the call's hash.
    fn oc960_fake_call(c: &mut Cpu, idx: u32, addr: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        let mb = OC960_MBOX + 8 * idx;
        if c.bus.read32(mb) != 0 {
            c.g[V0][0] = sx(c.bus.read32(mb + 4));
        } else {
            c.g[V0][0] = (OC960_SOUP + (c.g[V0][0] as u32 & (OC960_SOUP_BYTES - 4))) as u64;
        }
    }

    /// Force the result of the fake at table index `idx`.
    fn oc960_force(c: &mut Cpu, idx: u32, v: u32) {
        c.bus.write32(OC960_MBOX + 8 * idx, 1);
        c.bus.write32(OC960_MBOX + 8 * idx + 4, v);
    }

    oc960_fakes!(
        oc960_install_fakes;
        (0, oc960_fake_0014dc80, OC960_PREV_0014DC80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated, 0x0014_dc80),
        (1, oc960_fake_0014dd58, OC960_PREV_0014DD58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated, 0x0014_dd58),
        (2, oc960_fake_00317670, OC960_PREV_00317670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, 0x0031_7670),
        (3, oc960_fake_00397cc0, OC960_PREV_00397CC0, ee::s13::OVERRIDE_00397CC0, ee::s13::f_00397cc0__generated, 0x0039_7cc0),
        (4, oc960_fake_0039c870, OC960_PREV_0039C870, ee::s13::OVERRIDE_0039C870, ee::s13::f_0039c870__generated, 0x0039_c870),
        (5, oc960_fake_0020a380, OC960_PREV_0020A380, ee::s05::OVERRIDE_0020A380, ee::s05::f_0020a380__generated, 0x0020_a380),
        (6, oc960_fake_0039e948, OC960_PREV_0039E948, ee::s13::OVERRIDE_0039E948, ee::s13::f_0039e948__generated, 0x0039_e948),
        (7, oc960_fake_0039e8b8, OC960_PREV_0039E8B8, ee::s13::OVERRIDE_0039E8B8, ee::s13::f_0039e8b8__generated, 0x0039_e8b8),
        (8, oc960_fake_0028b180, OC960_PREV_0028B180, ee::s07::OVERRIDE_0028B180, ee::s07::f_0028b180__generated, 0x0028_b180),
        (9, oc960_fake_00294f78, OC960_PREV_00294F78, ee::s08::OVERRIDE_00294F78, ee::s08::f_00294f78__generated, 0x0029_4f78),
        (10, oc960_fake_0039d860, OC960_PREV_0039D860, ee::s13::OVERRIDE_0039D860, ee::s13::f_0039d860__generated, 0x0039_d860),
        (11, oc960_fake_002c2540, OC960_PREV_002C2540, ee::s08::OVERRIDE_002C2540, ee::s08::f_002c2540__generated, 0x002c_2540),
        (12, oc960_fake_00144bf0, OC960_PREV_00144BF0, ee::s01::OVERRIDE_00144BF0, ee::s01::f_00144bf0__generated, 0x0014_4bf0),
        (13, oc960_fake_002c27c0, OC960_PREV_002C27C0, ee::s08::OVERRIDE_002C27C0, ee::s08::f_002c27c0__generated, 0x002c_27c0),
        (14, oc960_fake_003a0e90, OC960_PREV_003A0E90, ee::s13::OVERRIDE_003A0E90, ee::s13::f_003a0e90__generated, 0x003a_0e90),
        (15, oc960_fake_00146e98, OC960_PREV_00146E98, ee::s01::OVERRIDE_00146E98, ee::s01::f_00146e98__generated, 0x0014_6e98),
        (16, oc960_fake_00417828, OC960_PREV_00417828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated, 0x0041_7828),
        (17, oc960_fake_00146f88, OC960_PREV_00146F88, ee::s01::OVERRIDE_00146F88, ee::s01::f_00146f88__generated, 0x0014_6f88),
        (18, oc960_fake_0014eec8, OC960_PREV_0014EEC8, ee::s01::OVERRIDE_0014EEC8, ee::s01::f_0014eec8__generated, 0x0014_eec8),
        (19, oc960_fake_0020a1a8, OC960_PREV_0020A1A8, ee::s05::OVERRIDE_0020A1A8, ee::s05::f_0020a1a8__generated, 0x0020_a1a8),
        (20, oc960_fake_004139f8, OC960_PREV_004139F8, ee::s15::OVERRIDE_004139F8, ee::s15::f_004139f8__generated, 0x0041_39f8),
        (21, oc960_fake_002c26d0, OC960_PREV_002C26D0, ee::s08::OVERRIDE_002C26D0, ee::s08::f_002c26d0__generated, 0x002c_26d0),
        (22, oc960_fake_003a0d00, OC960_PREV_003A0D00, ee::s13::OVERRIDE_003A0D00, ee::s13::f_003a0d00__generated, 0x003a_0d00),
        (23, oc960_fake_00399730, OC960_PREV_00399730, ee::s13::OVERRIDE_00399730, ee::s13::f_00399730__generated, 0x0039_9730),
        (24, oc960_fake_003a0c50, OC960_PREV_003A0C50, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated, 0x003a_0c50),
        (25, oc960_fake_003a1000, OC960_PREV_003A1000, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated, 0x003a_1000),
    );

    /// Gate open, the soup filled (half small numbers 0..8, half soup pointers).
    fn oc960_open(c: &mut Cpu, r: &mut crate::difftest::Rng) {
        c.bus.write32(OC960_CTRL, OC960_MAGIC);
        for i in 0..OC960_SOUP_BYTES / 4 {
            let w = r.next_u32();
            let v = if w & 1 == 0 {
                (w >> 8) % 8
            } else {
                OC960_SOUP + ((w >> 8) & (OC960_SOUP_BYTES - 4))
            };
            c.bus.write32(OC960_SOUP + 4 * i, v);
        }
    }

    /// `OVScreenHiscoreList_Init(self)`: adds the screen (result 0 for some seeds: skips the
    /// play), picks the show / hide arm from the byte at 0x535c10 (0..9: the table entries show
    /// (0, 5), hide (1-4, 6) and fall out (7+) through the generated jump-out), and branches on
    /// the sign of the two score-count words behind the `getHandle(8)` result.
    #[test]
    fn ov_screen_hiscore_list_init_matches() {
        oc960_install_fakes();
        fakes::covered(
            ee::s04::f_001e8ff0__generated,
            ov_screen_hiscore_list_init,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e8ff0, 12);
                elf_image(c);
                oc960_open(c, &mut r);
                let this = DATA + 0x1000;
                c.bus.write32(this + 0x10, OC960_SOUP + 0x40);
                c.bus.write8(0x0053_5c10, (seed % 10) as u8);
                if seed % 4 == 0 {
                    oc960_force(c, 3, 0);
                }
                // getHandle(8): an entry whose two words at +0x18 / +0x1c have either sign.
                let entry = OC960_SOUP + 0x200;
                c.bus
                    .write32(entry + 0x18, if seed & 1 == 0 { 3 } else { 0x8000_0003 });
                c.bus
                    .write32(entry + 0x1c, if seed & 2 == 0 { 5 } else { 0x8000_0005 });
                oc960_force(c, 1, entry);
                c.g[A0][0] = sx(this);
                c.pc = 0x001e_8ff0;
            },
        );
    }

    /// `OVScreenFinalStandings_TransitionIn(self)`: the pending flag at `+0x9c` (clear: nothing
    /// to do), the event kind bytes at 0x535bc8 + 0x48 / 0x49 (selecting the jump-table arm and
    /// the rider / location branches), the soup for the world / rider-interface chains.
    #[test]
    fn ov_screen_final_standings_transition_in_matches() {
        oc960_install_fakes();
        fakes::covered(
            ee::s04::f_001e6718__generated,
            ov_screen_final_standings_transition_in,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e6718, 40);
                elf_image(c);
                oc960_open(c, &mut r);
                let this = DATA + 0x1000;
                c.bus
                    .write32(this + 0x9c, if seed % 7 == 0 { 0 } else { 1 });
                c.bus.write32(this + 0x40, OC960_SOUP + 0x80);
                let world = OC960_SOUP + 0x400;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(
                    GAME_GP - 0xfb4,
                    if seed % 3 == 0 { 0 } else { OC960_SOUP + 0x300 },
                );
                c.bus.write8(0x0053_5bc8 + 0x48, (seed % 8) as u8);
                c.bus.write8(0x0053_5bc8 + 0x49, ((seed / 8) % 4) as u8);
                if seed % 5 == 0 {
                    oc960_force(c, 10, 0);
                }
                c.g[A0][0] = sx(this);
                c.pc = 0x001e_6718;
            },
        );
    }
}
