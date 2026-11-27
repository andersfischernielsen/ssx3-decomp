//! Staging module oc962 (wave wave257): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc962`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// `void OVScreenFinalStandings_OnActivate(cOVState* self)` (0x1e75d0): the final-standings overlay gains focus. Plays the focus sound chain, then for each human rider that shows a rank (player 1, and player 2 in two-player modes) formats the "rider%d" / "timePoints%d" ranking labels and fades them in; when a network game is connected it shows the chat widgets and sets up the chat text scroll; finishes with the menu sound 0xe.
///
/// Every register, cycle and call of the generated body is kept (labelled
/// blocks mirror its branches).
pub fn ov_screen_final_standings_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e766c: {
            c.cycles += 28;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S3][0] = c.g[A0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[RA][0] = sx(0x001e75fc);
            c.g[S5][0] = 0u64;
            call_ptr(c, 0x0039e4c0, 0x001e75fc)?;
            c.g[RA][0] = sx(0x001e7604);
            call_ptr(c, 0x0014dc80, 0x001e7604)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001e7610);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x001e7610)?;
            c.g[RA][0] = sx(0x001e7618);
            c.g[S4][0] = c.g[V0][0];
            call_ptr(c, 0x0014dc80, 0x001e7618)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001e7624);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x001e7624)?;
            c.g[V1][0] = 0x530000;
            c.g[A2][0] = 0x1;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
            c.g[A0][0] = 0x2;
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
            let t = c.g[V0][0] != c.g[A0][0];
            c.g[A1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x48));
            if t {
                break 'b_1e766c;
            } else {
                'b_1e7664: {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S5][0] = 0x1;
                    if t {
                        break 'b_1e7664;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x5;
                        let t = c.g[A1][0] == c.g[V0][0];
                        c.g[V0][0] = 0x6;
                        if t {
                            break 'b_1e7664;
                        } else {
                            c.cycles += 2;
                            let t = c.g[A1][0] != c.g[V0][0];

                            if t {
                                break 'b_1e766c;
                            } else {
                                break 'b_1e7664;
                            }
                        }
                    }
                }
                c.cycles += 2;
                c.g[A2][0] = 0u64;
                c.g[S5][0] = 0u64;
                break 'b_1e766c;
            }
        }
        'b_1e7700: {
            c.cycles += 2;
            let t = c.g[A2][0] == 0u64;
            c.g[S0][0] = 0x4a0000;
            if t {
                break 'b_1e7700;
            } else {
                c.cycles += 35;
                c.g[A0][0] = c.g[S4][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2170));
                c.g[RA][0] = sx(0x001e7684);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00146e98, 0x001e7684)?;
                c.g[S2][0] = 0x530000;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6730));
                c.g[A0][0] = c.g[SP][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                c.g[RA][0] = sx(0x001e76a8);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x00417828, 0x001e76a8)?;
                c.g[S1][0] = 0x470000;
                c.g[A1][0] = c.g[SP][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1b18));
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x001e76c0);
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x0020a6f0, 0x001e76c0)?;
                c.g[S0][0] = 0x470000;
                c.g[A0][0] = c.g[S4][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x19c8));
                c.g[RA][0] = sx(0x001e76d4);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00146e98, 0x001e76d4)?;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A1][0] = c.g[S0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                c.g[RA][0] = sx(0x001e76f0);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x00417828, 0x001e76f0)?;
                c.g[A2][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x001e7700);
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x0020a6f0, 0x001e7700)?;
                break 'b_1e7700;
            }
        }
        'b_1e7794: {
            c.cycles += 2;
            let t = c.g[S5][0] == 0u64;
            c.g[S0][0] = 0x4a0000;
            if t {
                break 'b_1e7794;
            } else {
                c.cycles += 35;
                c.g[A0][0] = c.g[S4][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2170));
                c.g[RA][0] = sx(0x001e7718);
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x00146e98, 0x001e7718)?;
                c.g[S2][0] = 0x530000;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6730));
                c.g[A0][0] = c.g[SP][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                c.g[RA][0] = sx(0x001e773c);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x00417828, 0x001e773c)?;
                c.g[S1][0] = 0x470000;
                c.g[A1][0] = c.g[SP][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1b18));
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x001e7754);
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x0020a6f0, 0x001e7754)?;
                c.g[S0][0] = 0x470000;
                c.g[A0][0] = c.g[S4][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x19c8));
                c.g[RA][0] = sx(0x001e7768);
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x00146e98, 0x001e7768)?;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A1][0] = c.g[S0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                c.g[RA][0] = sx(0x001e7784);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x00417828, 0x001e7784)?;
                c.g[A2][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x001e7794);
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x0020a6f0, 0x001e7794)?;
                break 'b_1e7794;
            }
        }
        'b_1e78e8: {
            c.cycles += 8;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = 0x7;
            c.g[RA][0] = sx(0x001e77a4);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
            call_ptr(c, 0x0014dd58, 0x001e77a4)?;
            c.g[V1][0] = 0x530000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
            if t {
                break 'b_1e78e8;
            } else {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;

                if t {
                    break 'b_1e78e8;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1e78e8;
                    } else {
                        'b_1e77fc: {
                            c.cycles += 8;
                            c.g[RA][0] = sx(0x001e77d0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1838));
                            call_ptr(c, 0x00317670, 0x001e77d0)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001e77dc);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001e77dc)?;
                            c.g[A2][0] = c.g[V0][0];
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = 0x1;
                            if t {
                                break 'b_1e77fc;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001e77fc);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001e77fc)?;
                                break 'b_1e77fc;
                            }
                        }
                        'b_1e783c: {
                            c.cycles += 9;
                            c.g[A0][0] = 0x470000;
                            c.g[RA][0] = sx(0x001e7808);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1828));
                            call_ptr(c, 0x00317670, 0x001e7808)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001e7814);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001e7814)?;
                            c.g[S0][0] = c.g[V0][0];
                            if c.g[S0][0] == 0u64 {
                                c.g[A0][0] = 0x470000;
                                break 'b_1e783c;
                            } else {
                                c.cycles += 7;
                                c.g[A0][0] = 0x470000;
                                c.g[RA][0] = sx(0x001e782c);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1818));
                                call_ptr(c, 0x00317670, 0x001e782c)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x001e7838);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x003a1000, 0x001e7838)?;
                                c.g[A0][0] = 0x470000;
                                break 'b_1e783c;
                            }
                        }
                        'b_1e7878: {
                            c.cycles += 8;
                            c.g[RA][0] = sx(0x001e7844);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1808));
                            call_ptr(c, 0x00317670, 0x001e7844)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001e7850);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001e7850)?;
                            c.g[A2][0] = c.g[V0][0];
                            if c.g[A2][0] == 0u64 {
                                c.g[A0][0] = 0x470000;
                                break 'b_1e7878;
                            } else {
                                c.cycles += 7;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001e7874);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001e7874)?;
                                c.g[A0][0] = 0x470000;
                                break 'b_1e7878;
                            }
                        }
                        c.cycles += 28;
                        c.g[RA][0] = sx(0x001e7880);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x17f8));
                        call_ptr(c, 0x00317670, 0x001e7880)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001e788c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001e788c)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[V1][0] = 0x4d0000;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[V0][0] = c.g[V0][0] | 0x1;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x74), c.g[V0][0]);
                        c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7bf8));
                        c.ldl(2, (c.g[T0][0] as u32).wrapping_add(0x7));
                        c.ldr(2, (c.g[T0][0] as u32));
                        c.ldl(6, (c.g[T0][0] as u32).wrapping_add(0xf));
                        c.ldr(6, (c.g[T0][0] as u32).wrapping_add(0x8));
                        c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0xaf));
                        c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0xa8));
                        c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0xb7));
                        c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0xb0));
                        c.g[RA][0] = sx(0x001e78d0);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x003a1310, 0x001e78d0)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001e78e8);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001e78e8)?;
                        break 'b_1e78e8;
                    }
                }
            }
        }
        c.cycles += 14;
        c.g[RA][0] = sx(0x001e78f0);
        call_ptr(c, 0x0028b180, 0x001e78f0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001e78fc);
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x00294f78, 0x001e78fc)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `void OVScreenFinishOv_Init(cOVState* self)` (0x1e7bb0): builds the race-finish overlay: adds the
/// screen and plays its first frame, hides the per-player widgets, shows the
/// finish time (via `FE_FormatMinSec`) and only the rank marker of the human
/// rider (markers 1..6), then shows or hides the mode, trick and net widgets
/// by the interface settings and plays the menu sound 0xe.
///
/// Every register, cycle and call of the generated body is kept (labelled
/// blocks mirror its branches).
pub fn ov_screen_finish_ov_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e7c1c: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[A0][0] = 0x470000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1cc0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[RA][0] = sx(0x001e7bf0);
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001e7bf0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001e7c04);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001e7c04)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1e7c1c;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001e7c1c);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001e7c1c)?;
                break 'b_1e7c1c;
            }
        }
        'b_1e7c5c: {
            c.cycles += 12;
            c.g[RA][0] = sx(0x001e7c24);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0020a380, 0x001e7c24)?;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x9c), 0u64);
            c.g[A0][0] = 0x470000;
            c.g[RA][0] = sx(0x001e7c34);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x17e8));
            call_ptr(c, 0x00317670, 0x001e7c34)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001e7c40);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001e7c40)?;
            c.g[V1][0] = c.g[V0][0];
            let t = c.g[V1][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_1e7c5c;
            } else {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x90));
                c.g[V0][0] = c.g[V0][0] | 0x8;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x90), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                break 'b_1e7c5c;
            }
        }
        'b_1e7d70: {
            'b_1e7d5c: {
                c.cycles += 60;
                c.g[RA][0] = sx(0x001e7c64);
                c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                call_ptr(c, 0x0014dc80, 0x001e7c64)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001e7c70);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x001e7c70)?;
                c.g[RA][0] = sx(0x001e7c78);
                call_ptr(c, 0x0014dc80, 0x001e7c78)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001e7c84);
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x001e7c84)?;
                c.g[RA][0] = sx(0x001e7c8c);
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dc80, 0x001e7c8c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001e7c98);
                c.g[A1][0] = 0x8;
                call_ptr(c, 0x0014dd58, 0x001e7c98)?;
                c.g[RA][0] = sx(0x001e7ca0);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                call_ptr(c, 0x0014dc80, 0x001e7ca0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001e7cac);
                c.g[A1][0] = 0xd;
                call_ptr(c, 0x0014dd58, 0x001e7cac)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001e7cbc);
                c.g[S5][0] = c.g[V0][0];
                call_ptr(c, 0x00146e98, 0x001e7cbc)?;
                c.g[S4][0] = c.g[V0][0];
                c.g[A1][0] = 0x470000;
                c.g[V0][0] = 0x530000;
                c.g[S0][0] = sx((c.g[S4][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17d8));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[S3][0] = c.lw((c.g[S1][0] as u32));
                c.g[S6][0] = c.lw((c.g[V0][0] as u32));
                c.g[RA][0] = sx(0x001e7ce8);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0039e8b8, 0x001e7ce8)?;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001e7cf8);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17c8));
                call_ptr(c, 0x0039e8b8, 0x001e7cf8)?;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001e7d08);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17b8));
                call_ptr(c, 0x0039e8b8, 0x001e7d08)?;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001e7d18);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17a8));
                call_ptr(c, 0x0039e8b8, 0x001e7d18)?;
                c.g[A0][0] = 0x470000;
                c.g[RA][0] = sx(0x001e7d24);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1798));
                call_ptr(c, 0x00317670, 0x001e7d24)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001e7d30);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001e7d30)?;
                c.g[V1][0] = 0x530000;
                c.g[S1][0] = c.g[V0][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                let t = (c.g[V0][0] as i64) < 0;
                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                if t {
                    break 'b_1e7d5c;
                } else {
                    c.cycles += 4;
                    c.mtc1(2, 0);
                    c.cvt_s_w(0, 0);
                    let t = true;

                    if t {
                        break 'b_1e7d70;
                    } else {
                        break 'b_1e7d5c;
                    }
                }
            }
            c.cycles += 5;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.add_s(0, 0, 0);
            break 'b_1e7d70;
        }
        c.cycles += 16;
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5790));
        c.g[S4][0] = sx((c.g[S4][0] as u32) << 2);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[S4][0]);
        c.g[S0][0] = 0x1;
        c.mul_s(12, 0, 12);
        c.g[S4][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
        c.g[S7][0] = 0x4a0000;
        c.g[S6][0] = 0x530000;
        c.g[RA][0] = sx(0x001e7d98);
        c.g[FP][0] = 0x470000;
        call_ptr(c, 0x004139f8, 0x001e7d98)?;
        c.g[RA][0] = sx(0x001e7da0);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0020a1a8, 0x001e7da0)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001e7dac);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a0c50, 0x001e7dac)?;
        c.g[A0][0] = c.g[SP][0];
        'l_1e7db0: loop {
            'b_1e7e10: {
                c.cycles += 17;
                c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x2198));
                c.g[RA][0] = sx(0x001e7dbc);
                c.g[A2][0] = c.g[S0][0];
                call_ptr(c, 0x00417828, 0x001e7dbc)?;
                c.g[RA][0] = sx(0x001e7dc4);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x001e7dc4)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001e7dd0);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001e7dd0)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001e7dec);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001e7dec)?;
                if c.g[S0][0] != c.g[S4][0] {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    break 'b_1e7e10;
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001e7e0c);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001e7e0c)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    break 'b_1e7e10;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 7i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[A0][0] = c.g[SP][0];
                if event_due(c) {
                    deliver_event_here(c, 0x001e7db0);
                }
                continue 'l_1e7db0;
            } else {
                'b_1e7e70: {
                    'b_1e7e64: {
                        'b_1e7e60: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                            let t = c.g[V0][0] != 0u64;
                            c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x1860));
                            if t {
                                break 'b_1e7e60;
                            } else {
                                'b_1e7e44: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x3;
                                    let t = c.g[S3][0] == c.g[V0][0];
                                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5bc8));
                                    if t {
                                        break 'b_1e7e44;
                                    } else {
                                        c.cycles += 4;
                                        c.g[A0][0] = 0x4;
                                        c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                                        if c.g[V1][0] != c.g[A0][0] {
                                            c.sw((c.g[GP][0] as u32).wrapping_sub(0xfb4), 0u64);
                                            break 'b_1e7e64;
                                        } else {
                                            break 'b_1e7e44;
                                        }
                                    }
                                }
                                c.cycles += 7;
                                c.g[V0][0] = 0x1;
                                c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x1860));
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0xfb4), c.g[V0][0]);
                                c.g[RA][0] = sx(0x001e7e58);
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x0039e948, 0x001e7e58)?;
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                                if t {
                                    break 'b_1e7e70;
                                } else {
                                    break 'b_1e7e60;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0xfb4), 0u64);
                        break 'b_1e7e64;
                    }
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001e7e6c);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0039e8b8, 0x001e7e6c)?;
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                    break 'b_1e7e70;
                }
                'b_1e7ec0: {
                    'b_1e7eb0: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V1][0] = 0x4a0000;
                        if t {
                            break 'b_1e7eb0;
                        } else {
                            'b_1e7e98: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x3;
                                let t = c.g[S3][0] == c.g[V0][0];
                                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5bc8));
                                if t {
                                    break 'b_1e7e98;
                                } else {
                                    c.cycles += 4;
                                    c.g[A0][0] = 0x4;
                                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                                    let t = c.g[V1][0] != c.g[A0][0];
                                    c.g[V1][0] = 0x4a0000;
                                    if t {
                                        break 'b_1e7eb0;
                                    } else {
                                        break 'b_1e7e98;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[V0][0] = 0x4a0000;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001e7ea8);
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2180));
                            call_ptr(c, 0x0039e948, 0x001e7ea8)?;
                            let t = true;
                            c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                            if t {
                                break 'b_1e7ec0;
                            } else {
                                break 'b_1e7eb0;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e7ebc);
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2180));
                    call_ptr(c, 0x0039e8b8, 0x001e7ebc)?;
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                    break 'b_1e7ec0;
                }
                'b_1e7f10: {
                    'b_1e7f00: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x4a0000;
                        if t {
                            break 'b_1e7f00;
                        } else {
                            'b_1e7ee8: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x3;
                                let t = c.g[S3][0] == c.g[V0][0];
                                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5bc8));
                                if t {
                                    break 'b_1e7ee8;
                                } else {
                                    c.cycles += 4;
                                    c.g[A0][0] = 0x4;
                                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                                    let t = c.g[V1][0] != c.g[A0][0];
                                    c.g[V0][0] = 0x4a0000;
                                    if t {
                                        break 'b_1e7f00;
                                    } else {
                                        break 'b_1e7ee8;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[A0][0] = 0x4a0000;
                            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2188));
                            c.g[RA][0] = sx(0x001e7ef8);
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x0039e948, 0x001e7ef8)?;
                            let t = true;
                            c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                            if t {
                                break 'b_1e7f10;
                            } else {
                                break 'b_1e7f00;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e7f0c);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2188));
                    call_ptr(c, 0x0039e8b8, 0x001e7f0c)?;
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                    break 'b_1e7f10;
                }
                'b_1e7f5c: {
                    'b_1e7f4c: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x3;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A0][0] = 0x4a0000;
                        if t {
                            break 'b_1e7f4c;
                        } else {
                            'b_1e7f34: {
                                c.cycles += 2;
                                let t = c.g[S3][0] == c.g[V1][0];
                                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5bc8));
                                if t {
                                    break 'b_1e7f34;
                                } else {
                                    c.cycles += 4;
                                    c.g[A0][0] = 0x4;
                                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                                    if c.g[V1][0] != c.g[A0][0] {
                                        c.g[A0][0] = 0x4a0000;
                                        break 'b_1e7f4c;
                                    } else {
                                        break 'b_1e7f34;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[V1][0] = 0x4a0000;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001e7f44);
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2190));
                            call_ptr(c, 0x0039e948, 0x001e7f44)?;
                            let t = true;
                            c.g[A0][0] = 0x470000;
                            if t {
                                break 'b_1e7f5c;
                            } else {
                                break 'b_1e7f4c;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2190));
                    c.g[RA][0] = sx(0x001e7f58);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0039e8b8, 0x001e7f58)?;
                    c.g[A0][0] = 0x470000;
                    break 'b_1e7f5c;
                }
                'b_1e7f98: {
                    'b_1e7f94: {
                        c.cycles += 10;
                        c.g[RA][0] = sx(0x001e7f64);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1788));
                        call_ptr(c, 0x00317670, 0x001e7f64)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001e7f70);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001e7f70)?;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[S1][0] = c.g[V0][0];
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                        let t = (c.g[V1][0] as i64) >= 0;
                        c.g[A0][0] = 0u64;
                        if t {
                            break 'b_1e7f94;
                        } else {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                            let t = (c.g[V0][0] as i64) < 0;

                            if t {
                                break 'b_1e7f98;
                            } else {
                                break 'b_1e7f94;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = 0x1;
                    break 'b_1e7f98;
                }
                'b_1e7fd8: {
                    'b_1e7fbc: {
                        c.cycles += 2;
                        let t = c.g[A0][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_1e7fbc;
                        } else {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001e7fb4);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001e7fb4)?;
                            let t = true;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            if t {
                                break 'b_1e7fd8;
                            } else {
                                break 'b_1e7fbc;
                            }
                        }
                    }
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001e7fd4);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001e7fd4)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    break 'b_1e7fd8;
                }
                'b_1e8068: {
                    'b_1e8050: {
                        'b_1e802c: {
                            'b_1e8028: {
                                c.cycles += 9;
                                c.g[A3][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5bc8));
                                c.g[A2][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x49));
                                c.g[A1][0] = 0x2;
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                let t = c.g[A2][0] == c.g[A1][0];
                                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                if t {
                                    break 'b_1e8028;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x480));
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A1][0] = 0x470000;
                                        break 'b_1e802c;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x4a));
                                        c.g[V0][0] = 0x6;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x7;
                                        if t {
                                            break 'b_1e8028;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x8;
                                            if t {
                                                break 'b_1e8028;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] != c.g[V0][0];

                                                if t {
                                                    break 'b_1e8050;
                                                } else {
                                                    break 'b_1e8028;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[A1][0] = 0x470000;
                            break 'b_1e802c;
                        }
                        c.cycles += 9;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x001e8038);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1778));
                        call_ptr(c, 0x0039e8b8, 0x001e8038)?;
                        c.g[A1][0] = 0x4a0000;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x001e8048);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x21a0));
                        call_ptr(c, 0x0039e8b8, 0x001e8048)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        if t {
                            break 'b_1e8068;
                        } else {
                            break 'b_1e8050;
                        }
                    }
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x001e8058);
                    call_ptr(c, 0x0028b180, 0x001e8058)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001e8064);
                    c.g[A1][0] = 0xe;
                    call_ptr(c, 0x00294f78, 0x001e8064)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    break 'b_1e8068;
                }
                c.cycles += 11;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `HUD_DrawTextLerpW` (0x1f25e0): draws a text between two HUD elements of the 0x24-byte
/// element table: position, scale and colour are blended with the factor in
/// `f12`, optionally scaled and offset, aligned through `HUD_AlignElemPos`, then
/// measured and drawn with `Font_MeasureTextW` / `Font_DrawTextW`.
///
/// Every register, cycle and call of the generated body is kept (labelled
/// blocks mirror its branches).
pub fn hud_draw_text_lerp_w(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f26c0: {
            c.cycles += 49;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x210));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 0);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1f0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x1e0));
            c.g[S0][0] = 0x24;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1d0));
            c.g[S1][0] = c.g[T3][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1c0));
            c.g[S2][0] = c.g[A1][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1b0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x1a0));
            c.g[S4][0] = c.g[A2][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x190));
            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x180));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x170));
            c.g[S7][0] = c.g[T0][0];
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x200));
            c.g[FP][0] = c.g[A3][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x160), c.g[RA][0]);
            c.mov_s(20, 12);
            c.g[S5][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x228));
            c.g[A3][0] = 0u64;
            c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x30));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x154), c.g[T1][0]);
            c.g[S6][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
            c.mov_s(12, 2);
            c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x34));
            c.g[A1][0] = c.g[T1][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x150), c.g[A0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x158), c.g[T2][0]);
            c.mov_s(13, 1);
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = c.g[S2][0];
            c.swc1(0, (c.g[SP][0] as u32));
            c.swc1(2, (c.g[S2][0] as u32).wrapping_add(0x38));
            c.g[RA][0] = sx(0x001f267c);
            c.swc1(1, (c.g[S2][0] as u32).wrapping_add(0x3c));
            call_ptr(c, 0x003921f0, 0x001f267c)?;
            c.mult(0, 2, 30, 16);
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S3][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[RA][0] = sx(0x001f2690);
            c.g[A0][0] = c.g[S3][0];
            Callee {
                addr: 0x001e91f8,
                f: ee::s04::HUD_GetElemPos,
            }
            .call(c, 0x001f2690)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x001f269c);
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
            Callee {
                addr: 0x001e9290,
                f: ee::s04::HUD_GetElemScale,
            }
            .call(c, 0x001f269c)?;
            let t = c.g[S1][0] == 0u64;
            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
            if t {
                break 'b_1f26c0;
            } else {
                c.cycles += 7;
                c.lwc1(2, (c.g[S1][0] as u32));
                c.lwc1(3, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                c.mul_s(0, 0, 2);
                c.mul_s(1, 1, 3);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                break 'b_1f26c0;
            }
        }
        'b_1f2704: {
            c.cycles += 10;
            c.mult(0, 2, 23, 16);
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
            c.g[RA][0] = sx(0x001f26d4);
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x001e91f8,
                f: ee::s04::HUD_GetElemPos,
            }
            .call(c, 0x001f26d4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x001f26e0);
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
            Callee {
                addr: 0x001e9290,
                f: ee::s04::HUD_GetElemScale,
            }
            .call(c, 0x001f26e0)?;
            let t = c.g[S1][0] == 0u64;
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x50));
            if t {
                break 'b_1f2704;
            } else {
                c.cycles += 7;
                c.lwc1(3, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.lwc1(2, (c.g[S1][0] as u32));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.mul_s(1, 1, 2);
                c.mul_s(0, 0, 3);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
                break 'b_1f2704;
            }
        }
        'b_1f2758: {
            'b_1f274c: {
                c.cycles += 16;
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = 0x7f;
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x1c));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.mul_s(0, 3, 0);
                c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.mul_s(1, 2, 1);
                c.mul_s(3, 3, 5);
                c.mul_s(2, 2, 4);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x90));
                let t = c.g[S5][0] != c.g[V0][0];
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x94));
                if t {
                    break 'b_1f274c;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[T1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_1f2758;
                    } else {
                        break 'b_1f274c;
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[S5][0] as u32) << 24);
            c.g[T1][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[V0][0] = 0x7f;
            break 'b_1f2758;
        }
        'b_1f2778: {
            'b_1f2774: {
                c.cycles += 2;
                let t = c.g[S6][0] != c.g[V0][0];
                c.g[V0][0] = sx((c.g[S6][0] as u32) << 24);
                if t {
                    break 'b_1f2774;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = 0x24;
                    c.mult(0, 3, 30, 2);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                    let t = true;
                    c.g[T2][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x21));
                    if t {
                        break 'b_1f2778;
                    } else {
                        break 'b_1f2774;
                    }
                }
            }
            c.cycles += 1;
            c.g[T2][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            break 'b_1f2778;
        }
        'b_1f279c: {
            'b_1f2798: {
                c.cycles += 3;
                c.g[V0][0] = 0x7f;
                let t = c.g[S5][0] != c.g[V0][0];
                c.g[V0][0] = sx((c.g[S5][0] as u32) << 24);
                if t {
                    break 'b_1f2798;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = 0x24;
                    c.mult(0, 3, 23, 2);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                    let t = true;
                    c.g[S1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_1f279c;
                    } else {
                        break 'b_1f2798;
                    }
                }
            }
            c.cycles += 1;
            c.g[S1][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            break 'b_1f279c;
        }
        'b_1f27c0: {
            'b_1f27bc: {
                c.cycles += 3;
                c.g[V0][0] = 0x7f;
                let t = c.g[S6][0] != c.g[V0][0];
                c.g[V0][0] = sx((c.g[S6][0] as u32) << 24);
                if t {
                    break 'b_1f27bc;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = 0x24;
                    c.mult(0, 3, 23, 2);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                    let t = true;
                    c.g[S0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x21));
                    if t {
                        break 'b_1f27c0;
                    } else {
                        break 'b_1f27bc;
                    }
                }
            }
            c.cycles += 1;
            c.g[S0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            break 'b_1f27c0;
        }
        'b_1f2898: {
            c.cycles += 48;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[T3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[RA][0] = sx(0x001f27dc);
            c.g[T0][0] = c.g[SP][0];
            Callee {
                addr: 0x001f10f8,
                f: ee::s04::HUD_AlignElemPos,
            }
            .call(c, 0x001f27dc)?;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[T3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
            c.g[T1][0] = c.g[S1][0];
            c.g[T2][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[RA][0] = sx(0x001f2800);
            c.g[T0][0] = c.g[SP][0];
            Callee {
                addr: 0x001f10f8,
                f: ee::s04::HUD_AlignElemPos,
            }
            .call(c, 0x001f2800)?;
            c.lwc1(8, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.lwc1(9, (c.g[SP][0] as u32).wrapping_add(0x34));
            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
            c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sub_s(0, 0, 8);
            c.lwc1(7, (c.g[SP][0] as u32).wrapping_add(0x64));
            c.sub_s(1, 1, 9);
            c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x74));
            c.sub_s(2, 2, 6);
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sub_s(3, 3, 7);
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc4));
            c.mul_s(0, 0, 20);
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x210));
            c.mul_s(4, 2, 20);
            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.mul_s(5, 3, 20);
            c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xe4));
            c.mul_s(1, 1, 20);
            c.add_s(3, 4, 6);
            c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.add_s(2, 5, 7);
            c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xd4));
            c.add_s(0, 0, 8);
            c.add_s(1, 1, 9);
            c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xb4));
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xa0));
            let t = c.g[A0][0] == 0u64;
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xa4));
            if t {
                break 'b_1f2898;
            } else {
                c.cycles += 6;
                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.lwc1(1, (c.g[A0][0] as u32));
                c.add_s(0, 2, 0);
                c.add_s(1, 3, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xb4));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xb0));
                break 'b_1f2898;
            }
        }
        'b_1f28d0: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x220));
            let t = c.g[A1][0] == 0u64;
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xa0));
            if t {
                break 'b_1f28d0;
            } else {
                c.cycles += 11;
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x1c));
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0xa4));
                c.mul_s(2, 2, 1);
                c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.mul_s(0, 0, 3);
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xb4));
                c.swc1(4, (c.g[A1][0] as u32));
                c.swc1(1, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.swc1(2, (c.g[A1][0] as u32).wrapping_add(0x8));
                break 'b_1f28d0;
            }
        }
        'b_1f29d4: {
            'b_1f2998: {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x218));
                let t = c.g[A2][0] != 0u64;
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x218));
                if t {
                    break 'b_1f2998;
                } else {
                    c.cycles += 47;
                    c.g[S0][0] = 0x24;
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.mult(0, 2, 30, 16);
                    c.g[RA][0] = sx(0x001f28f0);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                    Callee {
                        addr: 0x001e91d0,
                        f: ee::s04::HUD_GetElemColorRotated,
                    }
                    .call(c, 0x001f28f0)?;
                    c.mult(0, 2, 23, 16);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.g[RA][0] = sx(0x001f2900);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                    Callee {
                        addr: 0x001e91d0,
                        f: ee::s04::HUD_GetElemColorRotated,
                    }
                    .call(c, 0x001f2900)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), 0u64);
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.mfc1(3, 20);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x11c));
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x114));
                    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x118));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x124));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x128));
                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x12c));
                    c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x127));
                    c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x12f));
                    c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x128));
                    c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xd7));
                    c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0xdf));
                    c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0xd8));
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                    if t {
                        break 'b_1f29d4;
                    } else {
                        break 'b_1f2998;
                    }
                }
            }
            c.cycles += 15;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 1);
            c.ldl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
            c.ldr(2, (c.g[A3][0] as u32));
            c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0xf));
            c.ldr(3, (c.g[A3][0] as u32).wrapping_add(0x8));
            c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0xd7));
            c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xdf));
            c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0xd8));
            c.sub_s(1, 1, 20);
            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.mul_s(0, 0, 1);
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
            break 'b_1f29d4;
        }
        c.cycles += 51;
        c.g[A3][0] = 0xfffffffffffffc1f;
        c.g[V1][0] = c.lb((c.g[S4][0] as u32).wrapping_add(0x3ca));
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.g[V1][0] & 0x3e0;
        c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xd7));
        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0xdf));
        c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0xd8));
        c.sdl(6, (c.g[S2][0] as u32).wrapping_add(0x47));
        c.sdr(6, (c.g[S2][0] as u32).wrapping_add(0x40));
        c.sdl(7, (c.g[S2][0] as u32).wrapping_add(0x4f));
        c.sdr(7, (c.g[S2][0] as u32).wrapping_add(0x48));
        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0xa4));
        c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x30));
        c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x34));
        c.mul_s(1, 1, 2);
        c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.mul_s(0, 0, 3);
        c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0xb4));
        c.swc1(1, (c.g[S2][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x001f2a4c);
        c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x3c));
        call_ptr(c, 0x00391e30, 0x001f2a4c)?;
        c.lwc1(3, (c.g[SP][0] as u32));
        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x30));
        c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x34));
        c.mul_s(1, 1, 3);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1f0));
        c.mul_s(0, 0, 2);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x1e0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.swc1(1, (c.g[S2][0] as u32).wrapping_add(0x38));
        c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x3c));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x160));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x200));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x210));
        c.pc = t;
        return Ok(());
    });
}

/// `HUD_DrawWrappedTextW` (0x1f2db0, `cOVStateHUD1P_renderMultiText1` in the fork): draws a
/// string wrapped at the element's width: the string is cut into lines with
/// `Font_FindLineBreakW`, the block is aligned by the element's flags (optionally
/// mirrored), each line is measured and drawn, and the covered rectangle is
/// returned through the optional rect pointer.
///
/// Every register, cycle and call of the generated body is kept (labelled
/// blocks mirror its branches).
pub fn hud_draw_wrapped_text_w(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f2ea8: {
            c.cycles += 50;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x330));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x2c0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x2b0));
            c.g[S3][0] = c.g[A1][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x2a0));
            c.g[S4][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2e0));
            c.g[S5][0] = c.g[A2][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x2d0));
            c.g[S1][0] = c.g[T0][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x280));
            c.g[A1][0] = c.g[SP][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2f0));
            c.g[S7][0] = c.g[A3][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x290));
            c.g[S2][0] = c.g[T2][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x270));
            c.g[A2][0] = 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x260), c.g[RA][0]);
            c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x414));
            c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x328));
            c.g[S6][0] = 0u64;
            c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x320));
            c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x318));
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x310));
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x308));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x300));
            c.g[RA][0] = sx(0x001f2e24);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x250), c.g[T1][0]);
            Callee {
                addr: 0x001e9220,
                f: ee::s04::HUD_GetElemSize,
            }
            .call(c, 0x001f2e24)?;
            c.g[S0][0] = 0x24;
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.mult(0, 2, 23, 16);
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
            c.g[RA][0] = sx(0x001f2e3c);
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x001e91f8,
                f: ee::s04::HUD_GetElemPos,
            }
            .call(c, 0x001f2e3c)?;
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[RA][0] = sx(0x001f2e48);
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x001e9290,
                f: ee::s04::HUD_GetElemScale,
            }
            .call(c, 0x001f2e48)?;
            c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x428));
            c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x230));
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = c.g[S1][0];
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x30));
            c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x34));
            c.mul_s(0, 0, 1);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x38));
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
            c.mul_s(2, 2, 1);
            let t = c.g[A1][0] == 0u64;
            c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x3c));
            if t {
                break 'b_1f2ea8;
            } else {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x428));
                'l_1f2e80: loop {
                    c.cycles += 10;
                    c.g[A2][0] = c.g[S0][0];
                    c.lwc1(13, (c.g[SP][0] as u32));
                    c.g[A3][0] = 0x32;
                    c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x64));
                    c.g[RA][0] = sx(0x001f2e9c);
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x00392680, 0x001f2e9c)?;
                    c.g[A1][0] = c.g[V0][0];
                    if c.g[A1][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x428));
                        if event_due(c) {
                            deliver_event_here(c, 0x001f2e80);
                        }
                        continue 'l_1f2e80;
                    } else {
                        break 'b_1f2ea8;
                    }
                }
            }
        }
        'b_1f2f54: {
            'b_1f2f4c: {
                c.cycles += 36;
                c.g[S0][0] = 0x24;
                c.g[A1][0] = c.g[FP][0];
                c.mult(0, 2, 23, 16);
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                c.g[RA][0] = sx(0x001f2ec0);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001e91a8,
                    f: ee::s04::HUD_GetElemColor,
                }
                .call(c, 0x001f2ec0)?;
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x237));
                c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x230));
                c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x23f));
                c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x238));
                c.sdl(3, (c.g[S3][0] as u32).wrapping_add(0x47));
                c.sdr(3, (c.g[S3][0] as u32).wrapping_add(0x40));
                c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0x4f));
                c.sdr(4, (c.g[S3][0] as u32).wrapping_add(0x48));
                c.mtc1(22, 3);
                c.cvt_s_w(3, 3);
                c.g[A1][0] = 0xfffffffffffffc1f;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x22));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x14));
                c.cvt_s_w(0, 0);
                c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x34));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x21));
                c.mul_s(0, 0, 1);
                c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.mul_s(23, 0, 2);
                let t = c.g[V0][0] != 0u64;
                c.mul_s(1, 23, 3);
                if t {
                    break 'b_1f2f4c;
                } else {
                    c.cycles += 5;
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 0);
                    c.mul_s(0, 1, 0);
                    let t = true;
                    c.sub_s(13, 13, 0);
                    if t {
                        break 'b_1f2f54;
                    } else {
                        break 'b_1f2f4c;
                    }
                }
            }
            c.cycles += 2;
            if (c.g[V0][0] as i64) > 0 {
                c.sub_s(13, 13, 1);
                break 'b_1f2f54;
            } else {
                break 'b_1f2f54;
            }
        }
        'b_1f2f74: {
            c.cycles += 2;
            let t = c.g[S2][0] == 0u64;
            c.mov_s(22, 13);
            if t {
                break 'b_1f2f74;
            } else {
                c.cycles += 6;
                c.g[AT][0] = 0x47800000;
                c.mtc1(1, 0);
                c.swc1(1, (c.g[S2][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[S2][0] as u32));
                c.swc1(22, (c.g[S2][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                break 'b_1f2f74;
            }
        }
        'b_1f3054: {
            c.cycles += 2;
            let t = (c.g[S6][0] as i64) <= 0;
            c.g[S1][0] = 0u64;
            if t {
                break 'b_1f3054;
            } else {
                c.cycles += 11;
                c.g[V0][0] = 0x24;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x250));
                c.mult(0, 3, 23, 2);
                c.g[AT][0] = 0x44200000;
                c.mtc1(1, 25);
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 24);
                c.g[S4][0] = c.g[A0][0] & 0x2;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S7][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                c.g[S5][0] = c.g[A0][0] & 0x1;
                'l_1f2fa8: loop {
                    'b_1f2fe0: {
                        c.cycles += 11;
                        c.lwc1(12, (c.g[S3][0] as u32).wrapping_add(0x38));
                        c.g[A0][0] = c.g[S3][0];
                        c.lwc1(13, (c.g[S3][0] as u32).wrapping_add(0x3c));
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = 0u64;
                        c.g[RA][0] = sx(0x001f2fc4);
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x003921f0, 0x001f2fc4)?;
                        c.mov_s(21, 0);
                        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                        let t = c.g[S5][0] == 0u64;
                        c.g[V1][0] = c.lb((c.g[S7][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_1f2fe0;
                        } else {
                            c.cycles += 3;
                            c.sub_s(20, 25, 20);
                            c.g[V0][0] = sx(0u32.wrapping_sub((c.g[V1][0] as u32)));
                            if c.g[S4][0] == 0 {
                                c.g[V1][0] = c.g[V0][0];
                            }
                            break 'b_1f2fe0;
                        }
                    }
                    'b_1f2ffc: {
                        'b_1f2ff4: {
                            c.cycles += 2;
                            let t = c.g[V1][0] != 0u64;

                            if t {
                                break 'b_1f2ff4;
                            } else {
                                c.cycles += 3;
                                c.mul_s(0, 21, 24);
                                let t = true;
                                c.sub_s(20, 20, 0);
                                if t {
                                    break 'b_1f2ffc;
                                } else {
                                    break 'b_1f2ff4;
                                }
                            }
                        }
                        c.cycles += 2;
                        if (c.g[V1][0] as i64) > 0 {
                            c.sub_s(20, 20, 21);
                            break 'b_1f2ffc;
                        } else {
                            break 'b_1f2ffc;
                        }
                    }
                    'b_1f3044: {
                        c.cycles += 7;
                        c.g[A0][0] = c.g[S3][0];
                        c.mov_s(12, 20);
                        c.mov_s(13, 22);
                        c.g[RA][0] = sx(0x001f3010);
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x00391e30, 0x001f3010)?;
                        if c.g[S2][0] == 0u64 {
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            break 'b_1f3044;
                        } else {
                            'b_1f302c: {
                                c.cycles += 5;
                                c.lwc1(0, (c.g[S2][0] as u32));
                                c.c_lt_s(20, 0);
                                if c.fcc() {
                                    c.swc1(20, (c.g[S2][0] as u32));
                                    break 'b_1f302c;
                                } else {
                                    break 'b_1f302c;
                                }
                            }
                            'b_1f3040: {
                                c.cycles += 5;
                                c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x8));
                                c.c_lt_s(0, 21);
                                if c.fcc() {
                                    c.swc1(21, (c.g[S2][0] as u32).wrapping_add(0x8));
                                    break 'b_1f3040;
                                } else {
                                    break 'b_1f3040;
                                }
                            }
                            c.cycles += 1;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            break 'b_1f3044;
                        }
                    }
                    c.cycles += 4;
                    c.add_s(22, 22, 23);
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x64));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001f2fa8);
                        }
                        continue 'l_1f2fa8;
                    } else {
                        break 'b_1f3054;
                    }
                }
            }
        }
        c.cycles += 26;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.lwc1(2, (c.g[S3][0] as u32).wrapping_add(0x30));
        c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x34));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x244));
        c.swc1(1, (c.g[S3][0] as u32).wrapping_add(0x3c));
        c.swc1(2, (c.g[S3][0] as u32).wrapping_add(0x38));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2f0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2e0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x2d0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x2c0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x2b0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x2a0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x290));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x280));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x270));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x260));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x328));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x320));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x318));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x310));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x308));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x300));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x240));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x330));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateLodge_OnWidgetCreated(self, widget)` (0x1f3840): maps the created widget's name
/// hash (`widget+0x38`) to its slot index (`widget+0x18`: 0..5, 8, 6, 7 for the
/// ten known names); the eighth also shows/hides through its vtable, the
/// last one sets its text by the character level (`BENewPlayerInt_GetCharRecordField27C`).
///
/// Every register, cycle and call of the generated body is kept (labelled
/// blocks mirror its branches).
pub fn c_fe_state_lodge_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f3a24: {
            'b_1f3a20: {
                'b_1f3874: {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.g[A0][0] = 0x470000;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1240));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x001f3864);
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    call_ptr(c, 0x00317670, 0x001f3864)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f3874;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
                        if t {
                            break 'b_1f3a20;
                        } else {
                            break 'b_1f3874;
                        }
                    }
                }
                'b_1f3890: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f387c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1230));
                    call_ptr(c, 0x00317670, 0x001f387c)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f3890;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1f3a20;
                        } else {
                            break 'b_1f3890;
                        }
                    }
                }
                'b_1f38ac: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f3898);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1220));
                    call_ptr(c, 0x00317670, 0x001f3898)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f38ac;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x2;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1f3a20;
                        } else {
                            break 'b_1f38ac;
                        }
                    }
                }
                'b_1f38c8: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f38b4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1210));
                    call_ptr(c, 0x00317670, 0x001f38b4)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f38c8;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x3;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1f3a20;
                        } else {
                            break 'b_1f38c8;
                        }
                    }
                }
                'b_1f38e4: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f38d0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1200));
                    call_ptr(c, 0x00317670, 0x001f38d0)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f38e4;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x4;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1f3a20;
                        } else {
                            break 'b_1f38e4;
                        }
                    }
                }
                'b_1f3900: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f38ec);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x11f0));
                    call_ptr(c, 0x00317670, 0x001f38ec)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f3900;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x5;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1f3a20;
                        } else {
                            break 'b_1f3900;
                        }
                    }
                }
                'b_1f391c: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f3908);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x11e0));
                    call_ptr(c, 0x00317670, 0x001f3908)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f391c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x8;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1f3a20;
                        } else {
                            break 'b_1f391c;
                        }
                    }
                }
                'b_1f3938: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f3924);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x11d0));
                    call_ptr(c, 0x00317670, 0x001f3924)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f3938;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x6;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1f3a20;
                        } else {
                            break 'b_1f3938;
                        }
                    }
                }
                'b_1f3988: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f3940);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x11c0));
                    call_ptr(c, 0x00317670, 0x001f3940)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f3988;
                    } else {
                        c.cycles += 16;
                        c.g[V0][0] = 0x7;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001f3968);
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001f3968)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001f3980);
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001f3980)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_1f3a24;
                        } else {
                            break 'b_1f3988;
                        }
                    }
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x001f3990);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x11b0));
                call_ptr(c, 0x00317670, 0x001f3990)?;
                if c.g[S0][0] != c.g[V0][0] {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    break 'b_1f3a24;
                } else {
                    'b_1f3a0c: {
                        c.cycles += 12;
                        c.g[RA][0] = sx(0x001f39a0);
                        call_ptr(c, 0x0014dc80, 0x001f39a0)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001f39ac);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x0014dd58, 0x001f39ac)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001f39b8);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x00146d98, 0x001f39b8)?;
                        c.g[V1][0] = c.g[V0][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 17i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = 0x470000;
                        if t {
                            break 'b_1f3a0c;
                        } else {
                            'b_1f39f0: {
                                'b_1f39dc: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 19i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] = 0x470000;
                                    if t {
                                        break 'b_1f39dc;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x11a0));
                                        if t {
                                            break 'b_1f39f0;
                                        } else {
                                            break 'b_1f39dc;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[V1][0] as i64) < 21i64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = 0x470000;
                                if t {
                                    break 'b_1f3a0c;
                                } else {
                                    c.cycles += 2;
                                    c.g[A0][0] = 0x470000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1188));
                                    break 'b_1f39f0;
                                }
                            }
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x001f39f8);
                            call_ptr(c, 0x00317670, 0x001f39f8)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001f3a04);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x001f3a04)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_1f3a24;
                            } else {
                                break 'b_1f3a0c;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x001f3a14);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1170));
                    call_ptr(c, 0x00317670, 0x001f3a14)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001f3a20);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001f3a20)?;
                    break 'b_1f3a20;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1f3a24;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001e_75d0,
        ov_screen_final_standings_on_activate,
        "OVScreenFinalStandings_OnActivate",
    ),
    (
        0x001e_7bb0,
        ov_screen_finish_ov_init,
        "OVScreenFinishOv_Init",
    ),
    (0x001f_25e0, hud_draw_text_lerp_w, "HUD_DrawTextLerpW"),
    (0x001f_2db0, hud_draw_wrapped_text_w, "HUD_DrawWrappedTextW"),
    (
        0x001f_3840,
        c_fe_state_lodge_on_widget_created,
        "cFEStateLodge_OnWidgetCreated",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{
        check, elf_image, fresh, run_generated, Rng, DATA, GAME_GP, SP as TEST_SP,
    };
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Control block of this module's callee fakes (machine RAM, so parallel
    /// tests do not see each other): the gate word, per-fake call counters,
    /// the per-call result mailboxes and the output words of the fakes that
    /// fill an out parameter.
    const OC962_CTRL: u32 = DATA + 0x20_0000;
    const OC962_MAGIC: u32 = 0x0962_c0de;

    // Fixture objects (above the region the fakes' default result pointers
    // fall in, DATA..DATA+0x3fff0).
    const OC962_SELF: u32 = DATA + 0x4_0000;
    const OC962_WORLD: u32 = DATA + 0x4_0400;
    const OC962_WIDGET: u32 = DATA + 0x4_0800;
    const OC962_WIDGET_VT: u32 = DATA + 0x4_0900;
    const OC962_IFACE8: u32 = DATA + 0x4_0a00;
    const OC962_IFACED: u32 = DATA + 0x4_0a40;
    const OC962_MODE: u32 = DATA + 0x4_0a80;
    const OC962_WCHAIN1: u32 = DATA + 0x4_0b00;
    const OC962_WCHAIN2: u32 = DATA + 0x4_0b80;
    const OC962_RIDER: u32 = DATA + 0x4_0c00;
    const OC962_NETP: u32 = DATA + 0x4_0d00;
    const OC962_NETQ: u32 = DATA + 0x4_0d40;
    const OC962_TEXT: u32 = DATA + 0x4_6000;
    const OC962_SCALE: u32 = DATA + 0x4_6100;
    const OC962_PTR: u32 = DATA + 0x4_6140;
    const OC962_COL: u32 = DATA + 0x4_6180;
    const OC962_OUTP: u32 = DATA + 0x4_61c0;
    const OC962_TABLE: u32 = DATA + 0x4_6200;
    const OC962_G: u32 = DATA + 0x4_6800;
    const OC962_RS: u32 = DATA + 0x4_6900;
    const OC962_FONT: u32 = DATA + 0x4_7000;
    const OC962_FONTOBJ: u32 = DATA + 0x4_7200;
    const OC962_TABLE2: u32 = DATA + 0x4_7400;
    const OC962_STR: u32 = DATA + 0x4_7800;
    const OC962_RECT: u32 = DATA + 0x4_7900;

    // Fake indices.
    const FK_ON_ACTIVATE: u32 = 0;
    const FK_GET_BE: u32 = 1;
    const FK_GET_INTERFACE: u32 = 2;
    const FK_HUMAN_RIDER: u32 = 3;
    const FK_SPRINTF: u32 = 4;
    const FK_PLAY_ANIM: u32 = 5;
    const FK_STR_HASH: u32 = 6;
    const FK_FIND_OBJECT: u32 = 7;
    const FK_SET_TEXT_ID: u32 = 8;
    const FK_NAV_BUTTONS: u32 = 9;
    const FK_AUDIO: u32 = 10;
    const FK_FE_SOUND: u32 = 11;
    const FK_MIN_SEC: u32 = 12;
    const FK_OV_INIT: u32 = 13;
    const FK_CREATE_SCREEN: u32 = 14;
    const FK_START_THREAD: u32 = 15;
    const FK_HIDE: u32 = 16;
    const FK_SHOW: u32 = 17;
    const FK_SET_STRING: u32 = 18;
    const FK_FIXUNS: u32 = 19;
    const FK_DRAW_TEXT: u32 = 20;
    const FK_MEASURE: u32 = 21;
    const FK_COLOR_ROT: u32 = 22;
    const FK_ELEM_POS: u32 = 23;
    const FK_ELEM_SCALE: u32 = 24;
    const FK_ALIGN: u32 = 25;
    const FK_ELEM_COLOR: u32 = 26;
    const FK_ELEM_SIZE: u32 = 27;
    const FK_FIELD27C: u32 = 28;

    /// A fake callee: logs the call like `fakes::fake`, then (unless it
    /// unwound) returns `mail - 1` when the fixture put `mail` (non-zero) in
    /// this fake's mailbox for this call number (16 calls, then wraps), and
    /// copies `out` fixture words to the pointer in `a1` (callees that fill
    /// an out parameter).
    fn oc962_fake_call(c: &mut Cpu, addr: u32, idx: u32, out: u32) {
        let ra = c.g[RA][0] as u32;
        let a1 = c.g[A1][0] as u32;
        let n = c.bus.read32(OC962_CTRL + 0x20 + 4 * idx);
        c.bus.write32(OC962_CTRL + 0x20 + 4 * idx, n + 1);
        let mail = c.bus.read32(OC962_CTRL + 0x100 + 0x40 * idx + 4 * (n & 15));
        fakes::fake(c, addr, ra);
        if c.pc == ra {
            if mail != 0 {
                c.g[V0][0] = (mail - 1) as u64;
            }
            for i in 0..out {
                let w = c.bus.read32(OC962_CTRL + 0x1000 + 0x20 * idx + 4 * i);
                c.bus.write32(a1.wrapping_add(4 * i), w);
            }
        }
    }

    macro_rules! oc962_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $idx:expr, $out:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC962_CTRL) != OC962_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc962_fake_call(c, $addr, $idx, $out);
                }
            )*
            fn oc962_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc962_fakes!(
        (
            oc962_fake_on_activate,
            OC962_PREV_ON_ACTIVATE,
            ee::s13::OVERRIDE_0039E4C0,
            ee::s13::f_0039e4c0__generated,
            0x0039_e4c0,
            FK_ON_ACTIVATE,
            0
        ),
        (
            oc962_fake_get_be,
            OC962_PREV_GET_BE,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80,
            FK_GET_BE,
            0
        ),
        (
            oc962_fake_get_interface,
            OC962_PREV_GET_INTERFACE,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58,
            FK_GET_INTERFACE,
            0
        ),
        (
            oc962_fake_human_rider,
            OC962_PREV_HUMAN_RIDER,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x0014_6e98,
            FK_HUMAN_RIDER,
            0
        ),
        (
            oc962_fake_sprintf,
            OC962_PREV_SPRINTF,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828,
            FK_SPRINTF,
            0
        ),
        (
            oc962_fake_play_anim,
            OC962_PREV_PLAY_ANIM,
            ee::s05::OVERRIDE_0020A6F0,
            ee::s05::f_0020a6f0__generated,
            0x0020_a6f0,
            FK_PLAY_ANIM,
            0
        ),
        (
            oc962_fake_str_hash,
            OC962_PREV_STR_HASH,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            FK_STR_HASH,
            0
        ),
        (
            oc962_fake_find_object,
            OC962_PREV_FIND_OBJECT,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860,
            FK_FIND_OBJECT,
            0
        ),
        (
            oc962_fake_set_text_id,
            OC962_PREV_SET_TEXT_ID,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000,
            FK_SET_TEXT_ID,
            0
        ),
        (
            oc962_fake_nav_buttons,
            OC962_PREV_NAV_BUTTONS,
            ee::s13::OVERRIDE_003A1310,
            ee::s13::f_003a1310__generated,
            0x003a_1310,
            FK_NAV_BUTTONS,
            0
        ),
        (
            oc962_fake_audio,
            OC962_PREV_AUDIO,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180,
            FK_AUDIO,
            0
        ),
        (
            oc962_fake_fe_sound,
            OC962_PREV_FE_SOUND,
            ee::s08::OVERRIDE_00294F78,
            ee::s08::f_00294f78__generated,
            0x0029_4f78,
            FK_FE_SOUND,
            0
        ),
        (
            oc962_fake_min_sec,
            OC962_PREV_MIN_SEC,
            ee::s05::OVERRIDE_0020A1A8,
            ee::s05::f_0020a1a8__generated,
            0x0020_a1a8,
            FK_MIN_SEC,
            0
        ),
        (
            oc962_fake_ov_init,
            OC962_PREV_OV_INIT,
            ee::s05::OVERRIDE_0020A380,
            ee::s05::f_0020a380__generated,
            0x0020_a380,
            FK_OV_INIT,
            0
        ),
        (
            oc962_fake_create_screen,
            OC962_PREV_CREATE_SCREEN,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0,
            FK_CREATE_SCREEN,
            0
        ),
        (
            oc962_fake_start_thread,
            OC962_PREV_START_THREAD,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870,
            FK_START_THREAD,
            0
        ),
        (
            oc962_fake_hide,
            OC962_PREV_HIDE,
            ee::s13::OVERRIDE_0039E8B8,
            ee::s13::f_0039e8b8__generated,
            0x0039_e8b8,
            FK_HIDE,
            0
        ),
        (
            oc962_fake_show,
            OC962_PREV_SHOW,
            ee::s13::OVERRIDE_0039E948,
            ee::s13::f_0039e948__generated,
            0x0039_e948,
            FK_SHOW,
            0
        ),
        (
            oc962_fake_set_string,
            OC962_PREV_SET_STRING,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50,
            FK_SET_STRING,
            0
        ),
        (
            oc962_fake_fixuns,
            OC962_PREV_FIXUNS,
            ee::s15::OVERRIDE_004139F8,
            ee::s15::f_004139f8__generated,
            0x0041_39f8,
            FK_FIXUNS,
            0
        ),
        (
            oc962_fake_draw_text,
            OC962_PREV_DRAW_TEXT,
            ee::s12::OVERRIDE_00391E30,
            ee::s12::f_00391e30__generated,
            0x0039_1e30,
            FK_DRAW_TEXT,
            0
        ),
        (
            oc962_fake_measure,
            OC962_PREV_MEASURE,
            ee::s12::OVERRIDE_003921F0,
            ee::s12::f_003921f0__generated,
            0x0039_21f0,
            FK_MEASURE,
            0
        ),
        (
            oc962_fake_color_rot,
            OC962_PREV_COLOR_ROT,
            ee::s04::OVERRIDE_001E91D0,
            ee::s04::f_001e91d0__generated,
            0x001e_91d0,
            FK_COLOR_ROT,
            4
        ),
        (
            oc962_fake_elem_pos,
            OC962_PREV_ELEM_POS,
            ee::s04::OVERRIDE_001E91F8,
            ee::s04::f_001e91f8__generated,
            0x001e_91f8,
            FK_ELEM_POS,
            2
        ),
        (
            oc962_fake_elem_scale,
            OC962_PREV_ELEM_SCALE,
            ee::s04::OVERRIDE_001E9290,
            ee::s04::f_001e9290__generated,
            0x001e_9290,
            FK_ELEM_SCALE,
            2
        ),
        (
            oc962_fake_align,
            OC962_PREV_ALIGN,
            ee::s04::OVERRIDE_001F10F8,
            ee::s04::f_001f10f8__generated,
            0x001f_10f8,
            FK_ALIGN,
            2
        ),
        (
            oc962_fake_elem_color,
            OC962_PREV_ELEM_COLOR,
            ee::s04::OVERRIDE_001E91A8,
            ee::s04::f_001e91a8__generated,
            0x001e_91a8,
            FK_ELEM_COLOR,
            4
        ),
        (
            oc962_fake_elem_size,
            OC962_PREV_ELEM_SIZE,
            ee::s04::OVERRIDE_001E9220,
            ee::s04::f_001e9220__generated,
            0x001e_9220,
            FK_ELEM_SIZE,
            2
        ),
        (
            oc962_fake_field27c,
            OC962_PREV_FIELD27C,
            ee::s01::OVERRIDE_00146D98,
            ee::s01::f_00146d98__generated,
            0x0014_6d98,
            FK_FIELD27C,
            0
        ),
    );

    /// Mailbox word of fake `idx` for call number `n`: the fake returns `value`.
    fn oc962_mail(c: &mut Cpu, idx: u32, n: u32, value: u32) {
        c.bus.write32(
            OC962_CTRL + 0x100 + 0x40 * idx + 4 * n,
            value.wrapping_add(1),
        );
    }

    /// The words an out-parameter fake copies to its `a1` buffer.
    fn oc962_out(c: &mut Cpu, idx: u32, words: &[u32]) {
        for (i, w) in words.iter().enumerate() {
            c.bus
                .write32(OC962_CTRL + 0x1000 + 0x20 * idx + 4 * i as u32, *w);
        }
    }

    /// A machine with the game image (constants, tables), this module's
    /// fakes switched on and the out-parameter words of the HUD element
    /// getters filled with plausible floats.
    fn oc962_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc962_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        elf_image(c);
        c.bus.write32(OC962_CTRL, OC962_MAGIC);
        let pos = [r.f32(600.0), r.f32(400.0)];
        oc962_out(c, FK_ELEM_POS, &pos);
        let scale = [
            1.0f32.to_bits() ^ (seed & 1) * 0x40_0000,
            (0.5f32 + (seed % 4) as f32 * 0.25).to_bits(),
        ];
        oc962_out(c, FK_ELEM_SCALE, &scale);
        // A positive wrap width: with zero font tables every glyph is 0 wide, so
        // the string always fits and `Font_FindLineBreakW` returns 0 at its end
        // (a width <= 0 makes it return its own start and the wrap loop spin).
        let size = [(200.0f32 + (seed % 5) as f32 * 50.0).to_bits(), r.f32(40.0)];
        oc962_out(c, FK_ELEM_SIZE, &size);
        let align = [r.f32(600.0), r.f32(400.0)];
        oc962_out(c, FK_ALIGN, &align);
        let col = [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)];
        oc962_out(c, FK_ELEM_COLOR, &col);
        let rot = [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)];
        oc962_out(c, FK_COLOR_ROT, &rot);
        r
    }

    /// The widget the fakes' find-object results point at: a vtable whose
    /// slots 0x40/0x44 (set visible flag pair) and 0x48/0x4c are fake methods.
    fn oc962_widget(c: &mut Cpu, widget: u32, vt: u32) {
        c.bus.write32(widget + 8, vt);
        c.bus.write16(vt + 0x40, 0x10);
        c.bus.write32(vt + 0x44, FAKE + 0x10);
        c.bus.write16(vt + 0x48, 0x20);
        c.bus.write32(vt + 0x4c, FAKE + 0x20);
    }

    /// Distinct cycle totals of the generated body over `seeds` fixtures (no
    /// unwinds): at least `min` paths must be taken.
    fn oc962_paths(generated: fn(&mut Cpu), seeds: u32, min: usize, setup: &dyn Fn(&mut Cpu, u32)) {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= min, "only {} paths: {seen:?}", seen.len());
    }

    // ---------------------------------------------------------------------
    // OVScreenFinalStandings_OnActivate
    // ---------------------------------------------------------------------

    /// Mode bytes (player count / game mode: every comparison of the
    /// first/second ranking flags), the ranking table, network on/off with the
    /// comm object present or missing, and find-object results that are null
    /// or a widget.
    fn oc962_standings_fixture(c: &mut Cpu, seed: u32, calls: u32) {
        let mut r = oc962_machine(c, seed, 0x1e75d0, calls);
        c.g[A0][0] = sx(OC962_SELF);
        c.bus.write32(OC962_SELF + 0x40, DATA + 0x4_0200);
        c.bus
            .write8(0x0053_5bc8 + 0x49, if seed % 4 == 3 { 0 } else { 2 });
        c.bus.write8(0x0053_5bc8 + 0x48, ((seed / 4) % 8) as u8);
        for i in 0..4 {
            c.bus.write32(0x0053_6730 + 4 * i, r.next_u32() % 6);
            oc962_mail(c, FK_HUMAN_RIDER, i, (seed / 8 + i) % 4);
        }
        c.bus
            .write32(0x0053_4b30, if (seed / 32) % 2 == 0 { 0 } else { 1 });
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC962_WORLD);
        c.bus.write32(OC962_WORLD + 0x78, DATA + 0x4_0100);
        c.bus.write32(
            GAME_GP.wrapping_sub(0x204),
            if seed % 5 == 0 { 0 } else { OC962_NETP },
        );
        c.bus
            .write32(OC962_NETP, if (seed / 5) % 4 == 0 { 0 } else { OC962_NETQ });
        oc962_widget(c, OC962_WIDGET, OC962_WIDGET_VT);
        let null_at = seed % 7;
        for n in 0..4 {
            oc962_mail(
                c,
                FK_FIND_OBJECT,
                n,
                if n == null_at { 0 } else { OC962_WIDGET },
            );
        }
    }

    /// `OVScreenFinalStandings_OnActivate`: same registers, cycles, memory and
    /// call log as the generated body over every mode combination.
    #[test]
    fn ov_screen_final_standings_on_activate_matches() {
        check(
            ee::s04::f_001e75d0__generated,
            ov_screen_final_standings_on_activate,
            160,
            &|c, seed| oc962_standings_fixture(c, seed, 30),
        );
    }

    /// The fixtures reach many different paths of the generated body.
    #[test]
    fn ov_screen_final_standings_on_activate_paths() {
        oc962_paths(ee::s04::f_001e75d0__generated, 160, 8, &|c, seed| {
            oc962_standings_fixture(c, seed, 0)
        });
    }

    // ---------------------------------------------------------------------
    // OVScreenFinishOv_Init
    // ---------------------------------------------------------------------

    /// Interfaces 8 and 0xd as fixture objects (rank flags, mode selector),
    /// the world chain to the rider record, the mode bytes and ranking table.
    fn oc962_finish_fixture(c: &mut Cpu, seed: u32, calls: u32) {
        let mut r = oc962_machine(c, seed, 0x1e7bb0, calls);
        c.g[A0][0] = sx(OC962_SELF);
        c.bus.write32(OC962_SELF + 0x10, DATA + 0x4_0100);
        let pl = seed % 4;
        oc962_mail(c, FK_HUMAN_RIDER, 0, pl);
        oc962_mail(c, FK_GET_INTERFACE, 2, OC962_IFACE8);
        oc962_mail(c, FK_GET_INTERFACE, 3, OC962_IFACED);
        c.bus.write32(
            OC962_IFACE8 + 0x18,
            if seed % 3 == 0 { 0xffff_ffff } else { 2 },
        );
        c.bus.write32(
            OC962_IFACE8 + 0x1c,
            if seed % 5 < 2 { 0xffff_ffff } else { 1 },
        );
        c.bus.write32(OC962_IFACED + 0x14, (seed / 3) % 5);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC962_WORLD);
        c.bus.write32(OC962_WORLD + 0xc0, OC962_MODE);
        c.bus.write32(
            OC962_MODE,
            if (seed / 15) % 3 == 0 {
                3
            } else {
                (seed / 15) % 5
            },
        );
        c.bus.write32(OC962_WORLD + 0x84, OC962_WCHAIN1);
        c.bus.write32(OC962_WCHAIN1 + 0xc, OC962_WCHAIN2);
        for off in 0..4 {
            c.bus.write32(OC962_WCHAIN2 + 0x28 + 4 * off, OC962_RIDER);
        }
        c.bus
            .write32(OC962_RIDER + 0x480, if (seed / 7) % 2 == 0 { 0 } else { 1 });
        c.bus
            .write8(0x0053_5bc8 + 0x49, if (seed / 4) % 3 == 0 { 2 } else { 0 });
        c.bus.write8(
            0x0053_5bc8 + 0x4a,
            [4u8, 6, 7, 8, 1, 3][((seed / 9) % 6) as usize],
        );
        for i in 0..4 {
            c.bus.write32(0x0053_6730 + 4 * i, r.next_u32() % 8);
            c.bus.write32(
                0x0053_6640 + 4 * i,
                if (seed + i) % 3 == 0 {
                    0x8000_0000 | r.next_u32()
                } else {
                    r.next_u32() % 100_000
                },
            );
        }
        c.bus.write32(GAME_GP.wrapping_sub(0xfb4), r.next_u32());
        oc962_widget(c, OC962_WIDGET, OC962_WIDGET_VT);
        let null_at = seed % 6;
        if seed % 4 == 1 {
            oc962_mail(c, FK_CREATE_SCREEN, 0, 0);
        }
        for n in 0..16 {
            oc962_mail(
                c,
                FK_FIND_OBJECT,
                n,
                if n == null_at { 0 } else { OC962_WIDGET },
            );
        }
    }

    /// `OVScreenFinishOv_Init`: every rank marker, mode selector, interface
    /// flag and result-text combination.
    #[test]
    fn ov_screen_finish_ov_init_matches() {
        check(
            ee::s04::f_001e7bb0__generated,
            ov_screen_finish_ov_init,
            200,
            &|c, seed| oc962_finish_fixture(c, seed, 60),
        );
    }

    #[test]
    fn ov_screen_finish_ov_init_paths() {
        oc962_paths(ee::s04::f_001e7bb0__generated, 200, 8, &|c, seed| {
            oc962_finish_fixture(c, seed, 0)
        });
    }

    // ---------------------------------------------------------------------
    // HUD_DrawTextLerpW
    // ---------------------------------------------------------------------

    /// Arguments: the text struct, the element table (random flag bytes), two
    /// element indices, optional scale / offset / colour pointers, the
    /// stack arguments (0x7f = "use the element's value" selectors) and the
    /// blend factor in f12.
    fn oc962_lerp_fixture(c: &mut Cpu, seed: u32, calls: u32) {
        let mut r = oc962_machine(c, seed, 0x1f25e0, calls);
        c.bus.write32(OC962_TEXT + 0x30, r.f32(2.0));
        c.bus.write32(OC962_TEXT + 0x34, r.f32(2.0));
        for i in 0..0x100 {
            c.bus.write32(OC962_TABLE + 4 * i, r.next_u32());
        }
        c.bus.write32(OC962_SCALE, r.f32(2.0));
        c.bus.write32(OC962_SCALE + 4, r.f32(2.0));
        c.bus.write32(OC962_PTR, r.f32(50.0));
        c.bus.write32(OC962_PTR + 4, r.f32(50.0));
        for i in 0..4 {
            c.bus.write32(OC962_COL + 4 * i, r.f32(1.0));
        }
        c.bus.write32(GAME_GP.wrapping_sub(0x854), OC962_G);
        c.bus.write32(OC962_G + 0xe84, OC962_RS);
        c.bus.write32(OC962_RS + 8, r.next_u32());
        c.g[A0][0] = sx(OC962_SELF);
        c.g[A1][0] = sx(OC962_TEXT);
        c.g[A2][0] = sx(OC962_TABLE);
        c.g[A3][0] = (seed % 4) as u64;
        c.g[T0][0] = ((seed / 4) % 4) as u64;
        c.g[T1][0] = sx(r.next_u32());
        c.g[T2][0] = sx(r.next_u32());
        c.g[T3][0] = if seed % 3 == 0 { 0 } else { sx(OC962_SCALE) };
        let sel = |x: u32| if x % 2 == 0 { 0x7f } else { x % 4 };
        c.bus.write32(TEST_SP + 0x18, sel(seed / 3));
        c.bus.write32(TEST_SP + 0x20, sel(seed / 5 + 1));
        c.bus
            .write32(TEST_SP, if (seed / 7) % 2 == 0 { 0 } else { OC962_PTR });
        c.bus.write32(
            TEST_SP + 8,
            if (seed / 11) % 2 == 0 { 0 } else { OC962_COL },
        );
        c.bus.write32(
            TEST_SP + 0x10,
            if (seed / 13) % 2 == 0 { 0 } else { OC962_OUTP },
        );
        c.f[12] = if seed % 7 == 0 {
            0
        } else {
            r.f32(1.0) & 0x7fff_ffff
        };
    }

    /// `HUD_DrawTextLerpW`: blend of two element layouts (position, scale,
    /// colour) with all the optional pointer and selector variants.
    #[test]
    fn hud_draw_text_lerp_w_matches() {
        check(
            ee::s04::f_001f25e0__generated,
            hud_draw_text_lerp_w,
            200,
            &|c, seed| oc962_lerp_fixture(c, seed, 10),
        );
    }

    #[test]
    fn hud_draw_text_lerp_w_paths() {
        oc962_paths(ee::s04::f_001f25e0__generated, 200, 6, &|c, seed| {
            oc962_lerp_fixture(c, seed, 0)
        });
    }

    // ---------------------------------------------------------------------
    // HUD_DrawWrappedTextW
    // ---------------------------------------------------------------------

    /// A short wide string (spaces to break at, empty or null variants), the
    /// font object, the element table and the flag / rect arguments.
    fn oc962_wrapped_fixture(c: &mut Cpu, seed: u32, calls: u32) {
        let mut r = oc962_machine(c, seed, 0x1f2db0, calls);
        c.bus.write32(OC962_SELF + 0x428, OC962_FONT);
        c.bus.write32(OC962_FONT + 0x30, r.f32(2.0));
        c.bus.write32(OC962_FONT + 0x34, r.f32(2.0));
        c.bus.write32(OC962_FONTOBJ + 0x14, r.next_u32() % 40);
        c.bus.write32(OC962_FONTOBJ + 0x34, r.f32(30.0));
        c.bus.write32(OC962_FONTOBJ + 0x38, r.f32(30.0));
        c.bus.write32(OC962_FONTOBJ + 0x3c, r.f32(30.0));
        for i in 0..0x100 {
            c.bus.write32(OC962_TABLE2 + 4 * i, r.next_u32());
        }
        let text: &[u16] = match seed % 4 {
            0 => &[0x61, 0x62, 0x20, 0x63, 0x64, 0],
            1 => &[0x41, 0x20, 0x42, 0x20, 0x43, 0x20, 0x44, 0x20, 0x45, 0],
            2 => &[0x78, 0],
            _ => &[0x61, 0x62, 0x63, 0],
        };
        for (i, ch) in text.iter().enumerate() {
            c.bus.write16(OC962_STR + 2 * i as u32, *ch);
        }
        c.bus.write32(GAME_GP.wrapping_sub(0x854), OC962_G);
        c.bus.write32(OC962_G + 0xe84, OC962_RS);
        c.bus.write32(OC962_RS + 8, r.next_u32());
        c.g[A0][0] = sx(OC962_SELF);
        c.g[A1][0] = sx(OC962_FONTOBJ);
        c.g[A2][0] = sx(OC962_TABLE2);
        c.g[A3][0] = ((seed / 4) % 4) as u64;
        c.g[T0][0] = if seed % 11 == 4 { 0 } else { sx(OC962_STR) };
        c.g[T1][0] = ((seed / 3) % 4) as u64;
        c.g[T2][0] = if (seed / 7) % 2 == 0 {
            0
        } else {
            sx(OC962_RECT)
        };
    }

    /// `HUD_DrawWrappedTextW`: wraps a string into lines, positions them by
    /// the element's alignment (mirrored flags) and draws each line.
    #[test]
    fn hud_draw_wrapped_text_w_matches() {
        check(
            ee::s04::f_001f2db0__generated,
            hud_draw_wrapped_text_w,
            160,
            &|c, seed| oc962_wrapped_fixture(c, seed, 10),
        );
    }

    #[test]
    fn hud_draw_wrapped_text_w_paths() {
        oc962_paths(ee::s04::f_001f2db0__generated, 160, 5, &|c, seed| {
            oc962_wrapped_fixture(c, seed, 0)
        });
    }

    // ---------------------------------------------------------------------
    // cFEStateLodge_OnWidgetCreated
    // ---------------------------------------------------------------------

    /// Hash of the created widget's name; the hash fake returns it for call
    /// number `k` (the k-th name comparison matches), or never (`k == 10`).
    fn oc962_lodge_fixture(c: &mut Cpu, seed: u32, calls: u32) {
        let _r = oc962_machine(c, seed, 0x1f3840, calls);
        const NAME_HASH: u32 = 0x1234_5678;
        let k = seed % 11;
        c.bus.write32(OC962_WIDGET + 0x38, NAME_HASH);
        oc962_widget(c, OC962_WIDGET, OC962_WIDGET_VT);
        c.g[A1][0] = sx(OC962_WIDGET);
        for n in 0..11 {
            if n == k {
                oc962_mail(c, FK_STR_HASH, n, NAME_HASH);
            }
        }
        let lvl = [0u32, 16, 17, 18, 19, 20, 21, 25][((seed / 11) % 8) as usize];
        oc962_mail(c, FK_FIELD27C, 0, lvl);
    }

    /// `cFEStateLodge_OnWidgetCreated`: each widget name sets its slot index
    /// (the 8th also hides/shows through the vtable; the 10th picks a text by
    /// the character level), unknown names do nothing.
    #[test]
    fn c_fe_state_lodge_on_widget_created_matches() {
        check(
            ee::s04::f_001f3840__generated,
            c_fe_state_lodge_on_widget_created,
            176,
            &|c, seed| oc962_lodge_fixture(c, seed, 14),
        );
    }

    #[test]
    fn c_fe_state_lodge_on_widget_created_paths() {
        oc962_paths(ee::s04::f_001f3840__generated, 176, 11, &|c, seed| {
            oc962_lodge_fixture(c, seed, 0)
        });
    }
}
