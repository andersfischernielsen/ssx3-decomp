//! Staging module oc982 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc982`.

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
        0x001f_7958,
        ov_screen_top_times_init,
        "OVScreenTopTimes_Init",
    ),
    (
        0x001f_8168,
        ov_screen_pause_freeride_init,
        "OVScreenPauseFreeride_Init",
    ),
    (
        0x001f_8948,
        ov_screen_pause_freeride_on_widget_action,
        "OVScreenPauseFreeride_OnWidgetAction",
    ),
    (
        0x001f_d320,
        ov_screen_rival_pre_transition_in,
        "OVScreenRivalPre_TransitionIn",
    ),
];

/// `void OVScreenTopTimes_Init(cOVScreenTopTimes* self)` (0x001f7958): builds the
/// "top times" pause overlay. Adds the two UI screens (title and list), runs the
/// overlay base init, then fills the header text from the current event type
/// (`0x535bc8+0x4a`: peak events use their name text, event 5 a formatted track
/// name, others the track name plus the mode name from `0x535bc8+0x48`), shows or
/// hides the full/short layout objects by mode, and writes five rows of
/// score-holder name, owner and score (min:sec format in the full layout). Ends
/// with the front-end confirmation sound. Every register, cycle and call is kept
/// as in the generated body (its straight conversion: calls through `call_ptr`,
/// the mode and event jump tables as `match`).
pub fn ov_screen_top_times_init(c: &mut Cpu) {
    if c.pc != 0x001f7958 {
        return ee::s04::f_001f7958__generated(c);
    }
    guest(c, |c| 'l_1f7958: loop {
        'b_1f79c8: {
            c.cycles += 24;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x220));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1f0));
            c.g[S2][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x210));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x200));
            c.g[A0][0] = 0x470000;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1e0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xaf0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1d0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x1c0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x1b0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x1a0));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x190));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x180), c.g[RA][0]);
            c.sw((c.g[S2][0] as u32).wrapping_add(0xa0), 0u64);
            c.g[RA][0] = sx(0x001f799c);
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001f799c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f79b0);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001f79b0)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1f79c8;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001f79c8);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001f79c8)?;
                break 'b_1f79c8;
            }
        }
        'b_1f7a00: {
            c.cycles += 11;
            c.g[A0][0] = 0x470000;
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = sx(0x001f79d8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c88));
            call_ptr(c, 0x00317670, 0x001f79d8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f79ec);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001f79ec)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[V0][0];
            if t {
                break 'b_1f7a00;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001f7a00);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001f7a00)?;
                break 'b_1f7a00;
            }
        }
        'b_1f7b90: {
            'b_1f7acc: {
                'b_1f7a90: {
                    c.cycles += 22;
                    c.g[V0][0] = 0x1;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f7a10);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x170), c.g[V0][0]);
                    call_ptr(c, 0x0020a380, 0x001f7a10)?;
                    c.g[RA][0] = sx(0x001f7a18);
                    call_ptr(c, 0x0014dc80, 0x001f7a18)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001f7a24);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x001f7a24)?;
                    c.g[RA][0] = sx(0x001f7a2c);
                    c.g[S4][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x001f7a2c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001f7a38);
                    c.g[A1][0] = 0x8;
                    call_ptr(c, 0x0014dd58, 0x001f7a38)?;
                    c.g[S5][0] = c.g[V0][0];
                    c.g[V1][0] = 0x5;
                    c.g[V0][0] = 0x530000;
                    c.g[S6][0] = c.g[V0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                    let t = c.g[A0][0] == c.g[V1][0];
                    c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    if t {
                        break 'b_1f7a90;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A0][0] as i64) < 5i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = (c.g[A1][0] < 0x6u64) as u64;
                        if t {
                            break 'b_1f7acc;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[A0][0] as i64) < 12i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = (c.g[A1][0] < 0x6u64) as u64;
                            if t {
                                break 'b_1f7acc;
                            } else {
                                c.cycles += 8;
                                c.g[RA][0] = sx(0x001f7a78);
                                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                                Callee {
                                    addr: 0x001fd190,
                                    f: ee::s04::FE_GetEventNameText,
                                }
                                .call(c, 0x001f7a78)?;
                                c.g[S4][0] = c.g[S0][0];
                                c.g[A1][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001f7a88);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x002c2508, 0x001f7a88)?;
                                let t = true;
                                c.g[A0][0] = 0x470000;
                                if t {
                                    break 'b_1f7b90;
                                } else {
                                    break 'b_1f7a90;
                                }
                            }
                        }
                    }
                }
                c.cycles += 15;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
                c.g[A1][0] = 0x470000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xae0));
                c.g[RA][0] = sx(0x001f7aa4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c2540, 0x001f7aa4)?;
                c.g[A0][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x001f7ab0);
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                call_ptr(c, 0x00144bf0, 0x001f7ab0)?;
                c.g[S4][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f7ac4);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x002c26d0, 0x001f7ac4)?;
                let t = true;
                c.g[A0][0] = 0x470000;
                if t {
                    break 'b_1f7b90;
                } else {
                    break 'b_1f7acc;
                }
            }
            'b_1f7b54: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[S3][0] = 0u64;
                if t {
                    break 'b_1f7b54;
                } else {
                    'b_1f7b2c: {
                        'b_1f7b20: {
                            'b_1f7b10: {
                                'b_1f7b00: {
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x470000;
                                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xaa0));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x001f7958 => {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x001f7958);
                                            }
                                            continue 'l_1f7958;
                                        }
                                        0x001f7af0 => {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[A0][0] = 0x470000;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x1ab8));
                                            if t {
                                                break 'b_1f7b2c;
                                            } else {
                                                break 'b_1f7b00;
                                            }
                                        }
                                        0x001f7b00 => {
                                            break 'b_1f7b00;
                                        }
                                        0x001f7b10 => {
                                            break 'b_1f7b10;
                                        }
                                        0x001f7b20 => {
                                            break 'b_1f7b20;
                                        }
                                        0x001f7b54 => {
                                            break 'b_1f7b54;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x001f7958, 0x001f7e68);
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
                                    break 'b_1f7b2c;
                                } else {
                                    break 'b_1f7b10;
                                }
                            }
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A0][0] = 0x470000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c58));
                            if t {
                                break 'b_1f7b2c;
                            } else {
                                break 'b_1f7b20;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c40));
                        break 'b_1f7b2c;
                    }
                    c.cycles += 10;
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x001f7b40);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    call_ptr(c, 0x00317670, 0x001f7b40)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001f7b50);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x001f7b50)?;
                    c.g[S3][0] = c.g[V0][0];
                    break 'b_1f7b54;
                }
            }
            c.cycles += 15;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
            c.g[A1][0] = 0x4a0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2400));
            c.g[RA][0] = sx(0x001f7b68);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002c2540, 0x001f7b68)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x001f7b74);
            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
            call_ptr(c, 0x00144bf0, 0x001f7b74)?;
            c.g[S4][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A3][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x001f7b8c);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x002c26d0, 0x001f7b8c)?;
            c.g[A0][0] = 0x470000;
            break 'b_1f7b90;
        }
        'b_1f7ca4: {
            'b_1f7c38: {
                'b_1f7bc8: {
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x001f7b98);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xad0));
                    call_ptr(c, 0x00317670, 0x001f7b98)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f7ba4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f7ba4)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001f7bb0);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x003a0e90, 0x001f7bb0)?;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5bc8));
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x5;
                    if t {
                        break 'b_1f7bc8;
                    } else {
                        c.cycles += 2;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[A1][0] = 0x4a0000;
                            break 'b_1f7c38;
                        } else {
                            break 'b_1f7bc8;
                        }
                    }
                }
                c.cycles += 28;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2408));
                c.g[RA][0] = sx(0x001f7bdc);
                c.g[S0][0] = 0x470000;
                call_ptr(c, 0x0039e948, 0x001f7bdc)?;
                c.g[S1][0] = 0x470000;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2410));
                c.g[RA][0] = sx(0x001f7bf4);
                c.g[S4][0] = c.g[S0][0];
                call_ptr(c, 0x0039e8b8, 0x001f7bf4)?;
                c.g[S3][0] = c.g[S1][0];
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f7c08);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xac0));
                call_ptr(c, 0x0039e948, 0x001f7c08)?;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f7c18);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xab0));
                call_ptr(c, 0x0039e8b8, 0x001f7c18)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f7c24);
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1608));
                call_ptr(c, 0x0039e948, 0x001f7c24)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f7c30);
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1618));
                call_ptr(c, 0x0039e8b8, 0x001f7c30)?;
                let t = true;
                c.g[A1][0] = 0x4a0000;
                if t {
                    break 'b_1f7ca4;
                } else {
                    break 'b_1f7c38;
                }
            }
            c.cycles += 27;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2408));
            c.g[RA][0] = sx(0x001f7c48);
            c.g[S0][0] = 0x470000;
            call_ptr(c, 0x0039e8b8, 0x001f7c48)?;
            c.g[S1][0] = 0x470000;
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2410));
            c.g[RA][0] = sx(0x001f7c60);
            c.g[S4][0] = c.g[S0][0];
            call_ptr(c, 0x0039e948, 0x001f7c60)?;
            c.g[S3][0] = c.g[S1][0];
            c.g[A1][0] = 0x470000;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x170), 0u64);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xac0));
            c.g[RA][0] = sx(0x001f7c78);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0039e8b8, 0x001f7c78)?;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f7c88);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xab0));
            call_ptr(c, 0x0039e948, 0x001f7c88)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f7c94);
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1608));
            call_ptr(c, 0x0039e8b8, 0x001f7c94)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f7ca0);
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1618));
            call_ptr(c, 0x0039e948, 0x001f7ca0)?;
            c.g[A1][0] = 0x4a0000;
            break 'b_1f7ca4;
        }
        'b_1f7cf8: {
            c.cycles += 14;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f7cb0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2418));
            call_ptr(c, 0x0039e948, 0x001f7cb0)?;
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f7cc0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2420));
            call_ptr(c, 0x0039e948, 0x001f7cc0)?;
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f7cd0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2090));
            call_ptr(c, 0x0039e948, 0x001f7cd0)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x9f0));
            if c.g[V0][0] == 0u64 {
                c.g[S3][0] = 0x1;
                break 'b_1f7cf8;
            } else {
                c.cycles += 7;
                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1608));
                c.g[RA][0] = sx(0x001f7ce8);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0039e8b8, 0x001f7ce8)?;
                c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1618));
                c.g[RA][0] = sx(0x001f7cf4);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0039e8b8, 0x001f7cf4)?;
                c.g[S3][0] = 0x1;
                break 'b_1f7cf8;
            }
        }
        c.cycles += 6;
        c.g[FP][0] = 0x4a0000;
        c.g[S7][0] = 0x4a0000;
        c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S6][0] = 0x4a0000;
        c.g[V1][0] = 0x4a0000;
        'l_1f7d10: loop {
            'b_1f7e0c: {
                'b_1f7df0: {
                    c.cycles += 47;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2428));
                    c.g[RA][0] = sx(0x001f7d20);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x001f7d20)?;
                    c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                    c.g[RA][0] = sx(0x001f7d2c);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001f7d2c)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f7d38);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f7d38)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x001f7d48);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00155208, 0x001f7d48)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001f7d54);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a0c50, 0x001f7d54)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001f7d64);
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2430));
                    call_ptr(c, 0x00417828, 0x001f7d64)?;
                    c.g[RA][0] = sx(0x001f7d6c);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001f7d6c)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f7d78);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f7d78)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x001f7d88);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00155190, 0x001f7d88)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001f7d94);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a0c50, 0x001f7d94)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x2438));
                    c.g[RA][0] = sx(0x001f7da4);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x001f7da4)?;
                    c.g[RA][0] = sx(0x001f7dac);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001f7dac)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f7db8);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f7db8)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x170));
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        break 'b_1f7df0;
                    } else {
                        c.cycles += 9;
                        c.g[RA][0] = sx(0x001f7dd4);
                        c.g[A0][0] = c.g[S5][0];
                        call_ptr(c, 0x00155130, 0x001f7dd4)?;
                        c.g[RA][0] = sx(0x001f7ddc);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0020a1a8, 0x001f7ddc)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001f7de8);
                        c.g[A0][0] = c.g[S4][0];
                        call_ptr(c, 0x00417828, 0x001f7de8)?;
                        let t = true;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_1f7e0c;
                        } else {
                            break 'b_1f7df0;
                        }
                    }
                }
                c.cycles += 7;
                c.g[RA][0] = sx(0x001f7df8);
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x00155130, 0x001f7df8)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x001f7e08);
                c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x2138));
                call_ptr(c, 0x00417828, 0x001f7e08)?;
                c.g[A0][0] = c.g[S0][0];
                break 'b_1f7e0c;
            }
            c.cycles += 6;
            c.g[RA][0] = sx(0x001f7e14);
            c.g[A1][0] = c.g[S4][0];
            call_ptr(c, 0x003a0c50, 0x001f7e14)?;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[S3][0] as i64) < 6i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[V1][0] = 0x4a0000;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001f7d10);
                }
                continue 'l_1f7d10;
            } else {
                c.cycles += 17;
                c.g[RA][0] = sx(0x001f7e2c);
                call_ptr(c, 0x0028b180, 0x001f7e2c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f7e38);
                c.g[A1][0] = 0xe;
                call_ptr(c, 0x00294f78, 0x001f7e38)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x210));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x200));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x180));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x220));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void OVScreenPauseFreeride_Init(cOVScreenPauseFreeride* self)` (0x001f8168):
/// builds the freeride pause menu. Creates the screen from its hash name (and
/// plays its first frame), registers the PDA template, runs the overlay base
/// init, clears the option flag array, then for each of the eight menu slots
/// looks up the menu text object, asks the engine's sound/UI interface whether
/// the entry is enabled (and sets its text id or string), and hides or shows the
/// highlight icons. Finally sets the defaults (selected slot, flags), clears the
/// per-slot unlock table (depends on the BE state 0xb), plays the transition
/// and pauses the audio. Every register, cycle and call is kept as in the
/// generated body.
pub fn ov_screen_pause_freeride_init(c: &mut Cpu) {
    if c.pc != 0x001f8168 {
        return ee::s04::f_001f8168__generated(c);
    }
    guest(c, |c| {
        'b_1f81e4: {
            c.cycles += 27;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S2][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[A0][0] = 0x470000;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[RA][0] = sx(0x001f81a4);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7e8));
            call_ptr(c, 0x00317670, 0x001f81a4)?;
            c.g[A0][0] = 0x470000;
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7d0));
            c.g[RA][0] = sx(0x001f81b8);
            c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            call_ptr(c, 0x00317670, 0x001f81b8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f81cc);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001f81cc)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
            if t {
                break 'b_1f81e4;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001f81e4);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001f81e4)?;
                break 'b_1f81e4;
            }
        }
        c.cycles += 11;
        c.g[RA][0] = sx(0x001f81ec);
        c.g[S6][0] = 0u64;
        call_ptr(c, 0x0020d110, 0x001f81ec)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001f81f8);
        c.g[FP][0] = sx((c.g[S2][0] as u32).wrapping_add(0xd8));
        call_ptr(c, 0x0020a380, 0x001f81f8)?;
        c.g[S7][0] = 0u64;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x001f820c);
        c.g[A2][0] = 0x2;
        call_ptr(c, 0x00416210, 0x001f820c)?;
        'l_1f8210: loop {
            'b_1f8308: {
                c.cycles += 10;
                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x001f8220);
                c.sb((c.g[SP][0] as u32), c.g[V0][0]);
                call_ptr(c, 0x00317670, 0x001f8220)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc0));
                c.g[RA][0] = sx(0x001f822c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001f822c)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] == 0u64 {
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                    break 'b_1f8308;
                } else {
                    'b_1f8304: {
                        'b_1f82d4: {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xbc));
                            c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = sx((c.g[S6][0] as u32) << 2);
                            if t {
                                break 'b_1f82d4;
                            } else {
                                'b_1f82c8: {
                                    'b_1f82ac: {
                                        c.cycles += 21;
                                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x9c));
                                        c.g[S3][0] =
                                            sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[S5][0] = c.g[A0][0];
                                        c.g[V1][0] = c.lw((c.g[S3][0] as u32));
                                        c.g[V0][0] = 0x440000;
                                        c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1bc0));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[S4][0] as u32)));
                                        c.g[RA][0] = sx(0x001f8270);
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                        call_ptr(c, 0x00317670, 0x001f8270)?;
                                        c.g[S1][0] = c.g[V0][0];
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[A1][0] = c.g[S1][0];
                                        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001f8294);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001f8294)?;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A1][0] = c.g[S1][0];
                                        if t {
                                            break 'b_1f82ac;
                                        } else {
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x001f82a4);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x003a1000, 0x001f82a4)?;
                                            let t = true;
                                            c.g[V0][0] = sx((c.g[S5][0] as u32)
                                                .wrapping_add((c.g[S7][0] as u32)));
                                            if t {
                                                break 'b_1f82c8;
                                            } else {
                                                break 'b_1f82ac;
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32));
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                                    c.g[RA][0] = sx(0x001f82c4);
                                    c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                                    call_ptr(c, 0x003a0c50, 0x001f82c4)?;
                                    c.g[V0][0] =
                                        sx((c.g[S5][0] as u32).wrapping_add((c.g[S7][0] as u32)));
                                    break 'b_1f82c8;
                                }
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                                if t {
                                    break 'b_1f8304;
                                } else {
                                    break 'b_1f82d4;
                                }
                            }
                        }
                        c.cycles += 12;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001f82ec);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001f82ec)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001f8304);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001f8304)?;
                        break 'b_1f8304;
                    }
                    c.cycles += 1;
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                    break 'b_1f8308;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S6][0] as i64) < 8i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001f8210);
                }
                continue 'l_1f8210;
            } else {
                c.cycles += 9;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5544));
                c.g[V0][0] = 0x1;
                c.sw((c.g[S2][0] as u32).wrapping_add(0xd4), c.g[V0][0]);
                c.g[A0][0] = 0x1;
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0xcc));
                c.g[V1][0] = 0xd;
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc8), 0u64);
                c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x34));
                c.sw((c.g[S2][0] as u32).wrapping_add(0xd0), 0u64);
                'l_1f8338: loop {
                    c.cycles += 7;
                    c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[V1][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001f8338);
                        }
                        continue 'l_1f8338;
                    } else {
                        'b_1f836c: {
                            c.cycles += 6;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A1][0] = 0xb;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x214));
                            if c.g[V1][0] == c.g[A1][0] {
                                c.sw((c.g[S2][0] as u32).wrapping_add(0xf0), 0u64);
                                break 'b_1f836c;
                            } else {
                                break 'b_1f836c;
                            }
                        }
                        'b_1f83e0: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A1][0] = 0x7;
                            c.g[RA][0] = sx(0x001f837c);
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                            call_ptr(c, 0x0014dd58, 0x001f837c)?;
                            c.g[V1][0] = 0x530000;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
                            let t = c.g[V0][0] != 0u64;
                            c.g[A0][0] = 0x4a0000;
                            if t {
                                break 'b_1f83e0;
                            } else {
                                'b_1f83b0: {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[RA][0] = sx(0x001f8398);
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                    call_ptr(c, 0x00231ab8, 0x001f8398)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    if t {
                                        break 'b_1f83b0;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x224));
                                        let t = c.g[A0][0] == 0u64;
                                        c.g[A0][0] = 0x4a0000;
                                        if t {
                                            break 'b_1f83e0;
                                        } else {
                                            break 'b_1f83b0;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[A0][0] = c.g[FP][0];
                                c.g[V1][0] = 0u64;
                                c.g[A1][0] = 0x1;
                                'l_1f83c0: loop {
                                    'b_1f83cc: {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != c.g[A1][0];
                                        c.sw((c.g[A0][0] as u32), 0u64);
                                        if t {
                                            break 'b_1f83cc;
                                        } else {
                                            c.cycles += 1;
                                            c.sw(
                                                (c.g[S2][0] as u32).wrapping_add(0xdc),
                                                c.g[V1][0],
                                            );
                                            break 'b_1f83cc;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 14i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x001f83c0);
                                        }
                                        continue 'l_1f83c0;
                                    } else {
                                        c.cycles += 1;
                                        c.g[A0][0] = 0x4a0000;
                                        break 'b_1f83e0;
                                    }
                                }
                            }
                        }
                        'b_1f8404: {
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x001f83e8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2470));
                            call_ptr(c, 0x00317670, 0x001f83e8)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc0));
                            c.g[RA][0] = sx(0x001f83f4);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001f83f4)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.lbu((c.g[GP][0] as u32).wrapping_sub(0xc88));
                            if t {
                                break 'b_1f8404;
                            } else {
                                c.cycles += 2;
                                c.g[RA][0] = sx(0x001f8404);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x0039b960, 0x001f8404)?;
                                break 'b_1f8404;
                            }
                        }
                        c.cycles += 16;
                        c.g[RA][0] = sx(0x001f840c);
                        call_ptr(c, 0x0028b180, 0x001f840c)?;
                        c.g[RA][0] = sx(0x001f8414);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x00289b70, 0x001f8414)?;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// `void OVScreenPauseFreeride_OnWidgetAction(self, widget, event)` (0x001f8948):
/// the freeride pause menu's widget handler (`a2` = UI event 1..8, `a1` the
/// widget, whose `+0x18` is the menu slot). Select: plays the denied or accept
/// sound (by the unlock table at `self+0xd8`), remembers the slot and starts the
/// screen's transition-out, then by slot closes the pause (resume audio, pop
/// the state) or triggers overlay 0x17/7/0x18/0x1c/0x19/0x1a/0x1b/0x20/0x21/0x22
/// or opens the audio options popup. Other events: back, focus gain/loss
/// (icon animation) and the pause-state messages. Every register, cycle and call
/// is kept as in the generated body (the two jump tables as `match`).
pub fn ov_screen_pause_freeride_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001f8948 {
        return ee::s04::f_001f8948__generated(c);
    }
    guest(c, |c| 'l_1f8948: loop {
        'b_1f8dd4: {
            'b_1f8dd0: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A1][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[S0][0] == 0u64;
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                if t {
                    break 'b_1f8dd0;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V1][0] < 0x8u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x470000;
                    if t {
                        break 'b_1f8dd0;
                    } else {
                        'b_1f8d54: {
                            'b_1f8c88: {
                                'b_1f8c70: {
                                    'b_1f8bfc: {
                                        'b_1f8bec: {
                                            'b_1f8bd4: {
                                                'b_1f8bc4: {
                                                    'b_1f8bb4: {
                                                        'b_1f8ba4: {
                                                            'b_1f8b94: {
                                                                'b_1f8b84: {
                                                                    'b_1f8b74: {
                                                                        'b_1f8b0c: {
                                                                            'b_1f8afc: {
                                                                                'b_1f8ae4: {
                                                                                    'b_1f8a6c: {
                                                                                        c.cycles +=
                                                                                            6;
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x780));
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                                                        let t = c.g
                                                                                            [A0][0]
                                                                                            as u32;
                                                                                        match t {
0x001f8948 => {
if event_due(c) { deliver_event_here(c, 0x001f8948); } continue 'l_1f8948;
}
0x001f8994 => {
'l_1f8994: loop {
'b_1f89f4: {
c.cycles += 17;
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
c.g[S2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
c.g[A0][0] = 0x4a0000;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2470));
c.g[RA][0] = sx(0x001f89b0);
if c.g[V0][0] != 0 { c.g[S2][0] = c.g[V0][0]; }
call_ptr(c, 0x00317670, 0x001f89b0)?;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc0));
c.g[RA][0] = sx(0x001f89bc);
c.g[A1][0] = c.g[V0][0];
call_ptr(c, 0x0039d860, 0x001f89bc)?;
c.g[S3][0] = c.g[V0][0];
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd8));
let t = c.g[V1][0] != 0u64;
c.g[V0][0] = c.lw((c.g[S2][0] as u32));
if t {
break 'b_1f89f4;
} else {
c.cycles += 7;
c.g[A1][0] = 0x4;
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x001f89ec);
c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x001f89ec)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
if t {
break 'b_1f8dd4;
} else {
break 'b_1f89f4;
}
}
}
c.cycles += 24;
c.g[A1][0] = 0x6;
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x001f8a08);
c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x001f8a08)?;
c.g[V0][0] = c.lbu((c.g[S3][0] as u32).wrapping_add(0x95));
c.g[A0][0] = 0x470000;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7b0));
c.g[RA][0] = sx(0x001f8a1c);
c.sb((c.g[GP][0] as u32).wrapping_sub(0xc88), c.g[V0][0]);
call_ptr(c, 0x00317670, 0x001f8a1c)?;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc0));
c.g[RA][0] = sx(0x001f8a28);
c.g[A1][0] = c.g[V0][0];
call_ptr(c, 0x0039c778, 0x001f8a28)?;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc0));
c.g[A1][0] = c.g[V0][0] & 0xffff;
c.g[RA][0] = sx(0x001f8a38);
c.g[A2][0] = 0x1;
call_ptr(c, 0x0039c870, 0x001f8a38)?;
c.g[V1][0] = 0x1;
c.sw((c.g[S1][0] as u32).wrapping_add(0xd0), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[V1][0] = (c.g[A0][0] < 0xdu64) as u64;
let t = c.g[V1][0] == 0u64;
c.g[V0][0] = 0x470000;
if t {
break 'b_1f8dd0;
} else {
c.cycles += 6;
c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x760));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = c.lw((c.g[V1][0] as u32));
let t = c.g[A0][0] as u32;
match t {
0x001f8948 => {
if event_due(c) { deliver_event_here(c, 0x001f8948); } continue 'l_1f8948;
}
0x001f8994 => {
if event_due(c) { deliver_event_here(c, 0x001f8994); } continue 'l_1f8994;
}
0x001f8a6c => {
break 'b_1f8a6c;
}
0x001f8ae4 => {
break 'b_1f8ae4;
}
0x001f8afc => {
break 'b_1f8afc;
}
0x001f8b0c => {
break 'b_1f8b0c;
}
0x001f8b74 => {
break 'b_1f8b74;
}
0x001f8b84 => {
break 'b_1f8b84;
}
0x001f8b94 => {
break 'b_1f8b94;
}
0x001f8ba4 => {
break 'b_1f8ba4;
}
0x001f8bb4 => {
break 'b_1f8bb4;
}
0x001f8bc4 => {
break 'b_1f8bc4;
}
0x001f8bd4 => {
break 'b_1f8bd4;
}
0x001f8bec => {
break 'b_1f8bec;
}
0x001f8bfc => {
break 'b_1f8bfc;
}
0x001f8c88 => {
break 'b_1f8c88;
}
0x001f8d54 => {
break 'b_1f8d54;
}
0x001f8dd0 => {
break 'b_1f8dd0;
}
_ => { c.jump_out(t, 0x001f8948, 0x001f8dec); return Ok(()); }
}
}
}
}
0x001f8a6c => {
break 'b_1f8a6c;
}
0x001f8ae4 => {
break 'b_1f8ae4;
}
0x001f8afc => {
break 'b_1f8afc;
}
0x001f8b0c => {
break 'b_1f8b0c;
}
0x001f8b74 => {
break 'b_1f8b74;
}
0x001f8b84 => {
break 'b_1f8b84;
}
0x001f8b94 => {
break 'b_1f8b94;
}
0x001f8ba4 => {
break 'b_1f8ba4;
}
0x001f8bb4 => {
break 'b_1f8bb4;
}
0x001f8bc4 => {
break 'b_1f8bc4;
}
0x001f8bd4 => {
break 'b_1f8bd4;
}
0x001f8bec => {
break 'b_1f8bec;
}
0x001f8bfc => {
break 'b_1f8bfc;
}
0x001f8c88 => {
break 'b_1f8c88;
}
0x001f8d54 => {
break 'b_1f8d54;
}
0x001f8dd0 => {
break 'b_1f8dd0;
}
_ => { c.jump_out(t, 0x001f8948, 0x001f8dec); return Ok(()); }
}
                                                                                    }
                                                                                    'b_1f8a90: {
                                                                                        c.cycles +=
                                                                                            3;
                                                                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                                                                                        let t = c.g
                                                                                            [A0][0]
                                                                                            == 0u64;

                                                                                        if t {
                                                                                            break 'b_1f8a90;
                                                                                        } else {
                                                                                            c.cycles += 6;
                                                                                            c.g[RA][0] = sx(0x001f8a80);
                                                                                            c.g[A1][0] = 0u64;
                                                                                            call_ptr(c, 0x00258790, 0x001f8a80)?;
                                                                                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                                                                                            c.g[V0][0] = 0xffffffffffffffff;
                                                                                            c.sw((c.g[V1][0] as u32).wrapping_add(0x64), c.g[V0][0]);
                                                                                            c.sw((c.g[V1][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                                                                                            break 'b_1f8a90;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 21;
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001f8a98,
                                                                                    );
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x0020d190,
                                                                                        0x001f8a98,
                                                                                    )?;
                                                                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                                                                    c.g[A1][0] =
                                                                                        0x1;
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001f8aa8,
                                                                                    );
                                                                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x0039f190,
                                                                                        0x001f8aa8,
                                                                                    )?;
                                                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                                                                                    c.g[A1][0] =
                                                                                        0x530000;
                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x66e8));
                                                                                    c.g[A2][0] =
                                                                                        0x440000;
                                                                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                                                                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x28f0));
                                                                                    c.g[V1][0] = sx(
                                                                                        (c.g[A0][0]
                                                                                            as u32)
                                                                                            << 2,
                                                                                    );
                                                                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A0][0]);
                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                                                                    c.g[V0][0] = c
                                                                                        .lw(
                                                                                        (c.g[V1][0]
                                                                                            as u32),
                                                                                    );
                                                                                    c.g[V0][0] = sx(
                                                                                        (c.g[V0][0]
                                                                                            as u32)
                                                                                            << 2,
                                                                                    );
                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                                                                    c.g[V1][0] = c
                                                                                        .lw(
                                                                                        (c.g[V0][0]
                                                                                            as u32),
                                                                                    );
                                                                                    let t = true;
                                                                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V1][0]);
                                                                                    if t {
                                                                                        break 'b_1f8c70;
                                                                                    } else {
                                                                                        break 'b_1f8ae4;
                                                                                    }
                                                                                }
                                                                                c.cycles += 6;
                                                                                c.g[V0][0] = 0x1;
                                                                                c.g[A0][0] = 0x17;
                                                                                c.g[RA][0] =
                                                                                    sx(0x001f8af4);
                                                                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x9e8), c.g[V0][0]);
                                                                                call_ptr(
                                                                                    c, 0x0020ab50,
                                                                                    0x001f8af4,
                                                                                )?;
                                                                                let t = true;
                                                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                                                if t {
                                                                                    break 'b_1f8dd4;
                                                                                } else {
                                                                                    break 'b_1f8afc;
                                                                                }
                                                                            }
                                                                            c.cycles += 4;
                                                                            c.g[RA][0] =
                                                                                sx(0x001f8b04);
                                                                            c.g[A0][0] = 0x7;
                                                                            call_ptr(
                                                                                c, 0x0020ab50,
                                                                                0x001f8b04,
                                                                            )?;
                                                                            let t = true;
                                                                            c.lq(
                                                                                16,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x40,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_1f8dd4;
                                                                            } else {
                                                                                break 'b_1f8b0c;
                                                                            }
                                                                        }
                                                                        c.cycles += 26;
                                                                        c.g[V1][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x848));
                                                                        c.g[A1][0] = 0x470000;
                                                                        c.g[A1][0] =
                                                                            sx((c.g[A1][0] as u32)
                                                                                .wrapping_sub(
                                                                                    0x7a0,
                                                                                ));
                                                                        c.g[A0][0] = 0x80;
                                                                        c.g[V0][0] = c.lw((c.g[V1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x84));
                                                                        c.g[A2][0] = 0u64;
                                                                        c.g[A3][0] = 0u64;
                                                                        c.g[RA][0] = sx(0x001f8b30);
                                                                        c.g[S0][0] = c.lw((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x48));
                                                                        call_ptr(
                                                                            c, 0x00317d70,
                                                                            0x001f8b30,
                                                                        )?;
                                                                        c.g[A3][0] = 0u64;
                                                                        c.g[A0][0] = c.g[V0][0];
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        c.g[RA][0] = sx(0x001f8b44);
                                                                        c.g[A2][0] = 0u64;
                                                                        call_ptr(
                                                                            c, 0x00195ff0,
                                                                            0x001f8b44,
                                                                        )?;
                                                                        c.g[V1][0] = c.lw((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        c.g[A3][0] =
                                                                            0xffffffffffffc0ff;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x18,
                                                                                ));
                                                                        c.g[A1][0] = c.g[V0][0];
                                                                        c.g[V1][0] =
                                                                            c.g[V1][0] & c.g[A3][0];
                                                                        c.g[A2][0] = 0u64;
                                                                        c.g[V1][0] =
                                                                            c.g[V1][0] | 0x100;
                                                                        c.g[A3][0] = 0u64;
                                                                        c.g[RA][0] = sx(0x001f8b6c);
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32)
                                                                                .wrapping_add(0x1c),
                                                                            c.g[V1][0],
                                                                        );
                                                                        call_ptr(
                                                                            c, 0x0039f320,
                                                                            0x001f8b6c,
                                                                        )?;
                                                                        let t = true;
                                                                        c.lq(
                                                                            16,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x40),
                                                                        );
                                                                        if t {
                                                                            break 'b_1f8dd4;
                                                                        } else {
                                                                            break 'b_1f8b74;
                                                                        }
                                                                    }
                                                                    c.cycles += 4;
                                                                    c.g[RA][0] = sx(0x001f8b7c);
                                                                    c.g[A0][0] = 0x18;
                                                                    call_ptr(
                                                                        c, 0x0020ab50, 0x001f8b7c,
                                                                    )?;
                                                                    let t = true;
                                                                    c.lq(
                                                                        16,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x40),
                                                                    );
                                                                    if t {
                                                                        break 'b_1f8dd4;
                                                                    } else {
                                                                        break 'b_1f8b84;
                                                                    }
                                                                }
                                                                c.cycles += 4;
                                                                c.g[RA][0] = sx(0x001f8b8c);
                                                                c.g[A0][0] = 0x1c;
                                                                call_ptr(
                                                                    c, 0x0020ab50, 0x001f8b8c,
                                                                )?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x40),
                                                                );
                                                                if t {
                                                                    break 'b_1f8dd4;
                                                                } else {
                                                                    break 'b_1f8b94;
                                                                }
                                                            }
                                                            c.cycles += 4;
                                                            c.g[RA][0] = sx(0x001f8b9c);
                                                            c.g[A0][0] = 0x19;
                                                            call_ptr(c, 0x0020ab50, 0x001f8b9c)?;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x40),
                                                            );
                                                            if t {
                                                                break 'b_1f8dd4;
                                                            } else {
                                                                break 'b_1f8ba4;
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[RA][0] = sx(0x001f8bac);
                                                        c.g[A0][0] = 0x1a;
                                                        call_ptr(c, 0x0020ab50, 0x001f8bac)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x40),
                                                        );
                                                        if t {
                                                            break 'b_1f8dd4;
                                                        } else {
                                                            break 'b_1f8bb4;
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[RA][0] = sx(0x001f8bbc);
                                                    c.g[A0][0] = 0x1b;
                                                    call_ptr(c, 0x0020ab50, 0x001f8bbc)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x40),
                                                    );
                                                    if t {
                                                        break 'b_1f8dd4;
                                                    } else {
                                                        break 'b_1f8bc4;
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x001f8bcc);
                                                c.g[A0][0] = 0x20;
                                                call_ptr(c, 0x0020ab50, 0x001f8bcc)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                if t {
                                                    break 'b_1f8dd4;
                                                } else {
                                                    break 'b_1f8bd4;
                                                }
                                            }
                                            c.cycles += 6;
                                            c.g[V0][0] = 0x2;
                                            c.g[A0][0] = 0x21;
                                            c.g[RA][0] = sx(0x001f8be4);
                                            c.sw(
                                                (c.g[GP][0] as u32).wrapping_sub(0xb5c),
                                                c.g[V0][0],
                                            );
                                            call_ptr(c, 0x0020ab50, 0x001f8be4)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                            if t {
                                                break 'b_1f8dd4;
                                            } else {
                                                break 'b_1f8bec;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001f8bf4);
                                        c.g[A0][0] = 0x22;
                                        call_ptr(c, 0x0020ab50, 0x001f8bf4)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        if t {
                                            break 'b_1f8dd4;
                                        } else {
                                            break 'b_1f8bfc;
                                        }
                                    }
                                    'b_1f8c20: {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                                        let t = c.g[A0][0] == 0u64;

                                        if t {
                                            break 'b_1f8c20;
                                        } else {
                                            c.cycles += 6;
                                            c.g[RA][0] = sx(0x001f8c10);
                                            c.g[A1][0] = 0u64;
                                            call_ptr(c, 0x00258790, 0x001f8c10)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                                            c.g[V0][0] = 0xffffffffffffffff;
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x64),
                                                c.g[V0][0],
                                            );
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x68),
                                                c.g[V0][0],
                                            );
                                            break 'b_1f8c20;
                                        }
                                    }
                                    c.cycles += 20;
                                    c.g[RA][0] = sx(0x001f8c28);
                                    call_ptr(c, 0x0020d190, 0x001f8c28)?;
                                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                                    c.g[V0][0] = 0x530000;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x66e8));
                                    c.g[A3][0] = 0x440000;
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x28f0));
                                    c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[A1][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A2][0]);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                    c.g[RA][0] = sx(0x001f8c70);
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V1][0]);
                                    call_ptr(c, 0x0039f190, 0x001f8c70)?;
                                    break 'b_1f8c70;
                                }
                                c.cycles += 6;
                                c.g[RA][0] = sx(0x001f8c78);
                                call_ptr(c, 0x0028b180, 0x001f8c78)?;
                                c.g[RA][0] = sx(0x001f8c80);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x00289bb8, 0x001f8c80)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                if t {
                                    break 'b_1f8dd4;
                                } else {
                                    break 'b_1f8c88;
                                }
                            }
                            'b_1f8d08: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd0));
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = 0x4a0000;
                                if t {
                                    break 'b_1f8d08;
                                } else {
                                    'b_1f8cec: {
                                        'b_1f8ccc: {
                                            c.cycles += 6;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] = sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xd8));
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[A1][0] = 0x4d0000;
                                            if t {
                                                break 'b_1f8ccc;
                                            } else {
                                                c.cycles += 8;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0x7508));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x001f8cc4);
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001f8cc4)?;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0xd0));
                                                if t {
                                                    break 'b_1f8cec;
                                                } else {
                                                    break 'b_1f8ccc;
                                                }
                                            }
                                        }
                                        c.cycles += 8;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0x4d0000;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7518));
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001f8ce8);
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001f8ce8)?;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd0));
                                        break 'b_1f8cec;
                                    }
                                    c.cycles += 2;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A0][0] = 0x4a0000;
                                        break 'b_1f8d08;
                                    } else {
                                        c.cycles += 5;
                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[RA][0] = sx(0x001f8d04);
                                        c.g[A2][0] = 0x1;
                                        Callee {
                                            addr: 0x001f8df8,
                                            f: ee::s04::OVScreenPauseFreeride_PlayIconAnim,
                                        }
                                        .call(c, 0x001f8d04)?;
                                        c.g[A0][0] = 0x4a0000;
                                        break 'b_1f8d08;
                                    }
                                }
                            }
                            c.cycles += 8;
                            c.g[RA][0] = sx(0x001f8d10);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2480));
                            call_ptr(c, 0x00317670, 0x001f8d10)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc0));
                            c.g[RA][0] = sx(0x001f8d1c);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001f8d1c)?;
                            c.g[S1][0] = c.g[V0][0];
                            let t = c.g[S1][0] == 0u64;
                            c.g[V0][0] = 0x440000;
                            if t {
                                break 'b_1f8dd0;
                            } else {
                                c.cycles += 11;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1bf8));
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[RA][0] = sx(0x001f8d40);
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                call_ptr(c, 0x00317670, 0x001f8d40)?;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x001f8d4c);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x003a1000, 0x001f8d4c)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                if t {
                                    break 'b_1f8dd4;
                                } else {
                                    break 'b_1f8d54;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd0));
                        if c.g[V0][0] != 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            break 'b_1f8dd4;
                        } else {
                            'b_1f8db8: {
                                'b_1f8d98: {
                                    c.cycles += 6;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd8));
                                    let t = c.g[V1][0] == 0u64;
                                    c.g[A1][0] = 0x4d0000;
                                    if t {
                                        break 'b_1f8d98;
                                    } else {
                                        c.cycles += 8;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74e8));
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001f8d90);
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001f8d90)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd0));
                                        if t {
                                            break 'b_1f8db8;
                                        } else {
                                            break 'b_1f8d98;
                                        }
                                    }
                                }
                                c.cycles += 8;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x4d0000;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74f8));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001f8db4);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001f8db4)?;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd0));
                                break 'b_1f8db8;
                            }
                            c.cycles += 2;
                            if c.g[V0][0] != 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                break 'b_1f8dd4;
                            } else {
                                c.cycles += 4;
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                c.g[A0][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x001f8dd0);
                                c.g[A2][0] = 0u64;
                                Callee {
                                    addr: 0x001f8df8,
                                    f: ee::s04::OVScreenPauseFreeride_PlayIconAnim,
                                }
                                .call(c, 0x001f8dd0)?;
                                break 'b_1f8dd0;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_1f8dd4;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void OVScreenRivalPre_TransitionIn(cOVScreenRivalPre* self, int msg)`
/// (0x001fd320): fills the rival pre-race overlay when it transitions in
/// (`a1` non-zero). Sets the rider name and the title/status texts from the
/// event type at `0x535bc8+0x4a` (rival time, rival points, the six peak
/// events: track name, challenger name, "beat the challenge" lines, goal time
/// or score formatted through `FE_FormatMinSec` / `sprintf`, locked-state
/// handling by the event path and location). Every register, cycle and call is
/// kept as in the generated body (the event jump table as `match`).
pub fn ov_screen_rival_pre_transition_in(c: &mut Cpu) {
    if c.pc != 0x001fd320 {
        return ee::s05::f_001fd320__generated(c);
    }
    guest(c, |c| 'l_1fd320: loop {
        'b_1fdb44: {
            'b_1fdb40: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x980));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x910));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x970));
                c.g[S6][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x960));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x950));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x940));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x930));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x920));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x900));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x8f0));
                let t = c.g[A1][0] == 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8e0), c.g[RA][0]);
                if t {
                    break 'b_1fdb40;
                } else {
                    'b_1fd52c: {
                        'b_1fd514: {
                            'b_1fd504: {
                                'b_1fd4c4: {
                                    c.cycles += 52;
                                    c.g[RA][0] = sx(0x001fd35c);
                                    c.g[S0][0] = 0x530000;
                                    call_ptr(c, 0x0014dc80, 0x001fd35c)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fd368);
                                    c.g[A1][0] = 0x1;
                                    call_ptr(c, 0x0014dd58, 0x001fd368)?;
                                    c.g[S1][0] = 0x4a0000;
                                    c.g[RA][0] = sx(0x001fd374);
                                    c.g[FP][0] = c.g[V0][0];
                                    call_ptr(c, 0x0014dc80, 0x001fd374)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fd380);
                                    c.g[A1][0] = 0u64;
                                    call_ptr(c, 0x0014dd58, 0x001fd380)?;
                                    c.g[RA][0] = sx(0x001fd388);
                                    c.g[S7][0] = c.g[V0][0];
                                    call_ptr(c, 0x0014dc80, 0x001fd388)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fd394);
                                    c.g[A1][0] = 0x2;
                                    call_ptr(c, 0x0014dd58, 0x001fd394)?;
                                    c.g[RA][0] = sx(0x001fd39c);
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8c0), c.g[V0][0]);
                                    call_ptr(c, 0x0014dc80, 0x001fd39c)?;
                                    c.g[A1][0] = 0xd;
                                    c.g[RA][0] = sx(0x001fd3a8);
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x0014dd58, 0x001fd3a8)?;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8c4), c.g[V0][0]);
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5bc8));
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2520));
                                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8c8), c.g[V0][0]);
                                    c.g[RA][0] = sx(0x001fd3c8);
                                    c.g[S5][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4a));
                                    call_ptr(c, 0x00317670, 0x001fd3c8)?;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001fd3d4);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fd3d4)?;
                                    c.g[S4][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[FP][0];
                                    c.g[RA][0] = sx(0x001fd3e4);
                                    c.g[A1][0] = 0u64;
                                    call_ptr(c, 0x00146e98, 0x001fd3e4)?;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8c0));
                                    c.g[RA][0] = sx(0x001fd3f0);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0014ee58, 0x001fd3f0)?;
                                    c.g[A0][0] = c.g[S4][0];
                                    c.g[RA][0] = sx(0x001fd3fc);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a0c50, 0x001fd3fc)?;
                                    c.g[A0][0] = 0x4a0000;
                                    c.g[RA][0] = sx(0x001fd408);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x21b8));
                                    call_ptr(c, 0x00317670, 0x001fd408)?;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001fd414);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fd414)?;
                                    c.g[S4][0] = c.g[V0][0];
                                    c.g[V0][0] = 0x5;
                                    let t = c.g[S5][0] == c.g[V0][0];
                                    c.g[V0][0] = ((c.g[S5][0] as i64) < 6i64) as u64;
                                    if t {
                                        break 'b_1fd4c4;
                                    } else {
                                        'b_1fd450: {
                                            'b_1fd43c: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = 0x4;
                                                if t {
                                                    break 'b_1fd43c;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[S5][0] == c.g[V0][0];
                                                    c.g[S3][0] =
                                                        sx((c.g[SP][0] as u32).wrapping_add(0x290));
                                                    if t {
                                                        break 'b_1fd450;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[A0][0] = 0x4a0000;
                                                        if t {
                                                            break 'b_1fd52c;
                                                        } else {
                                                            break 'b_1fd43c;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = ((c.g[S5][0] as i64) < 12i64) as u64;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[A0][0] = 0x4a0000;
                                                break 'b_1fd52c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;

                                                if t {
                                                    break 'b_1fd514;
                                                } else {
                                                    break 'b_1fd450;
                                                }
                                            }
                                        }
                                        c.cycles += 29;
                                        c.g[RA][0] = sx(0x001fd458);
                                        c.g[A0][0] = c.g[S7][0];
                                        call_ptr(c, 0x00144bf0, 0x001fd458)?;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001fd464);
                                        c.g[A0][0] = c.g[S3][0];
                                        call_ptr(c, 0x002c2540, 0x001fd464)?;
                                        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x260));
                                        c.g[A1][0] = 0x4a0000;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2140));
                                        c.g[RA][0] = sx(0x001fd478);
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x002c2540, 0x001fd478)?;
                                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.g[A0][0] = 0x470000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1ab8));
                                        c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                        c.g[RA][0] = sx(0x001fd498);
                                        c.g[S0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        call_ptr(c, 0x00317670, 0x001fd498)?;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                        c.g[A0][0] = c.g[S0][0];
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001fd4a8);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, t, 0x001fd4a8)?;
                                        c.g[A1][0] = c.g[S2][0];
                                        c.g[A0][0] = c.g[SP][0];
                                        c.g[A2][0] = c.g[S3][0];
                                        c.g[RA][0] = sx(0x001fd4bc);
                                        c.g[A3][0] = c.g[V0][0];
                                        call_ptr(c, 0x002c26d0, 0x001fd4bc)?;
                                        let t = true;
                                        c.g[A0][0] = c.g[S4][0];
                                        if t {
                                            break 'b_1fd504;
                                        } else {
                                            break 'b_1fd4c4;
                                        }
                                    }
                                }
                                c.cycles += 16;
                                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x290));
                                c.g[RA][0] = sx(0x001fd4d0);
                                c.g[A0][0] = c.g[S7][0];
                                call_ptr(c, 0x00144bf0, 0x001fd4d0)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001fd4dc);
                                c.g[A0][0] = c.g[S1][0];
                                call_ptr(c, 0x002c2540, 0x001fd4dc)?;
                                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x260));
                                c.g[A1][0] = 0x470000;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1aa8));
                                c.g[RA][0] = sx(0x001fd4f0);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x002c2540, 0x001fd4f0)?;
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A0][0] = c.g[SP][0];
                                c.g[RA][0] = sx(0x001fd500);
                                c.g[A2][0] = c.g[S1][0];
                                call_ptr(c, 0x002c26d0, 0x001fd500)?;
                                c.g[A0][0] = c.g[S4][0];
                                break 'b_1fd504;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001fd50c);
                            c.g[A1][0] = c.g[SP][0];
                            call_ptr(c, 0x003a0e90, 0x001fd50c)?;
                            let t = true;
                            c.g[A0][0] = 0x4a0000;
                            if t {
                                break 'b_1fd52c;
                            } else {
                                break 'b_1fd514;
                            }
                        }
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x001fd51c);
                        c.g[A0][0] = c.g[S5][0];
                        call_ptr(c, 0x001fd190, 0x001fd51c)?;
                        c.g[A0][0] = c.g[S4][0];
                        c.g[RA][0] = sx(0x001fd528);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a0e90, 0x001fd528)?;
                        c.g[A0][0] = 0x4a0000;
                        break 'b_1fd52c;
                    }
                    'b_1fd654: {
                        'b_1fd624: {
                            'b_1fd5f8: {
                                'b_1fd5e0: {
                                    c.cycles += 42;
                                    c.g[S0][0] = 0x4a0000;
                                    c.g[RA][0] = sx(0x001fd538);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2528));
                                    call_ptr(c, 0x00317670, 0x001fd538)?;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001fd544);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fd544)?;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8cc), c.g[V0][0]);
                                    c.g[RA][0] = sx(0x001fd550);
                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2530));
                                    call_ptr(c, 0x00317670, 0x001fd550)?;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001fd55c);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fd55c)?;
                                    c.g[A0][0] = 0x4a0000;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8d0), c.g[V0][0]);
                                    c.g[RA][0] = sx(0x001fd56c);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x21b0));
                                    call_ptr(c, 0x00317670, 0x001fd56c)?;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001fd578);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fd578)?;
                                    c.g[A0][0] = 0x470000;
                                    c.g[S4][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fd588);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1a10));
                                    call_ptr(c, 0x00317670, 0x001fd588)?;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fd594);
                                    c.g[A0][0] = c.g[S4][0];
                                    call_ptr(c, 0x003a1000, 0x001fd594)?;
                                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2c0));
                                    c.g[RA][0] = sx(0x001fd5a0);
                                    c.g[A0][0] = c.g[S7][0];
                                    call_ptr(c, 0x00145750, 0x001fd5a0)?;
                                    c.g[S3][0] = c.g[S0][0];
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8c0));
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fd5b4);
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, 0x0014eec8, 0x001fd5b4)?;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fd5c0);
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x002c2540, 0x001fd5c0)?;
                                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x4));
                                    c.g[S2][0] = c.g[V1][0];
                                    c.g[V1][0] = (c.g[V1][0] < 0x2u64) as u64;
                                    let t = c.g[V1][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    if t {
                                        break 'b_1fd5e0;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x470000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x98));
                                        if t {
                                            break 'b_1fd5f8;
                                        } else {
                                            break 'b_1fd5e0;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x6));
                                c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_1fd624;
                                } else {
                                    c.cycles += 2;
                                    c.g[A0][0] = 0x470000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x80));
                                    break 'b_1fd5f8;
                                }
                            }
                            c.cycles += 11;
                            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                            c.g[RA][0] = sx(0x001fd60c);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            call_ptr(c, 0x00317670, 0x001fd60c)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                            c.g[A0][0] = c.g[S0][0];
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001fd61c);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, t, 0x001fd61c)?;
                            let t = true;
                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x300));
                            if t {
                                break 'b_1fd654;
                            } else {
                                break 'b_1fd624;
                            }
                        }
                        c.cycles += 12;
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x68));
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = sx(0x001fd640);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        call_ptr(c, 0x00317670, 0x001fd640)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.g[A0][0] = c.g[S0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001fd650);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, t, 0x001fd650)?;
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x300));
                        break 'b_1fd654;
                    }
                    c.cycles += 16;
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001fd664);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002c26d0, 0x001fd664)?;
                    c.g[V0][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x001fd670);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2520));
                    call_ptr(c, 0x00317670, 0x001fd670)?;
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001fd67c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fd67c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001fd688);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x003a0d00, 0x001fd688)?;
                    c.g[V0][0] = (c.g[S2][0] < 0x8u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x470000;
                    if t {
                        break 'b_1fdb40;
                    } else {
                        'b_1fdb2c: {
                            'b_1fdb10: {
                                'b_1fda18: {
                                    'b_1fd8c0: {
                                        'b_1fd8ac: {
                                            'b_1fd88c: {
                                                'b_1fd878: {
                                                    'b_1fd794: {
                                                        c.cycles += 6;
                                                        c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                                                        c.g[V0][0] =
                                                            sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc0));
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                        let t = c.g[A0][0] as u32;
                                                        match t {
                                                            0x001fd320 => {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x001fd320,
                                                                    );
                                                                }
                                                                continue 'l_1fd320;
                                                            }
                                                            0x001fd6ac => {
                                                                c.cycles += 18;
                                                                c.g[A0][0] = 0x470000;
                                                                c.g[RA][0] = sx(0x001fd6b8);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x1c8));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001fd6b8,
                                                                )?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8cc));
                                                                c.g[RA][0] = sx(0x001fd6c4);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x003a1000, 0x001fd6c4,
                                                                )?;
                                                                c.g[A0][0] = 0x470000;
                                                                c.g[RA][0] = sx(0x001fd6d0);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x50));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001fd6d0,
                                                                )?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8d0));
                                                                c.g[RA][0] = sx(0x001fd6dc);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x003a1000, 0x001fd6dc,
                                                                )?;
                                                                c.g[V0][0] = 0x530000;
                                                                c.g[A0][0] = 0x1;
                                                                c.g[V1][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x5bc8));
                                                                c.g[V0][0] = c
                                                                    .lb((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x49));
                                                                let t = c.g[V0][0] == c.g[A0][0];
                                                                c.g[V1][0] = 0x4a0000;
                                                                if t {
                                                                    break 'b_1fd88c;
                                                                } else {
                                                                    'b_1fd72c: {
                                                                        c.cycles += 5;
                                                                        c.g[RA][0] = sx(0x001fd6fc);
                                                                        c.g[A0][0] = c.g[S7][0];
                                                                        call_ptr(
                                                                            c, 0x00144be0,
                                                                            0x001fd6fc,
                                                                        )?;
                                                                        c.g[V1][0] = 0xe;
                                                                        let t = c.g[V0][0]
                                                                            != c.g[V1][0];

                                                                        if t {
                                                                            break 'b_1fd72c;
                                                                        } else {
                                                                            c.cycles += 9;
                                                                            c.g[RA][0] =
                                                                                sx(0x001fd710);
                                                                            c.g[A0][0] = 0u64;
                                                                            call_ptr(
                                                                                c, 0x0014a080,
                                                                                0x001fd710,
                                                                            )?;
                                                                            c.g[A2][0] = c.g[V0][0];
                                                                            c.g[A0][0] = c.g[FP][0];
                                                                            c.g[A1][0] = 0u64;
                                                                            c.g[RA][0] =
                                                                                sx(0x001fd724);
                                                                            c.g[A3][0] = 0x6;
                                                                            call_ptr(
                                                                                c, 0x00146a70,
                                                                                0x001fd724,
                                                                            )?;
                                                                            let t =
                                                                                c.g[V0][0] == 0u64;
                                                                            c.g[V1][0] = 0x4a0000;
                                                                            if t {
                                                                                break 'b_1fd88c;
                                                                            } else {
                                                                                break 'b_1fd72c;
                                                                            }
                                                                        }
                                                                    }
                                                                    'b_1fd764: {
                                                                        c.cycles += 5;
                                                                        c.g[RA][0] = sx(0x001fd734);
                                                                        c.g[A0][0] = c.g[S7][0];
                                                                        call_ptr(
                                                                            c, 0x00144be0,
                                                                            0x001fd734,
                                                                        )?;
                                                                        c.g[V1][0] = 0xf;
                                                                        let t = c.g[V0][0]
                                                                            != c.g[V1][0];

                                                                        if t {
                                                                            break 'b_1fd764;
                                                                        } else {
                                                                            c.cycles += 9;
                                                                            c.g[RA][0] =
                                                                                sx(0x001fd748);
                                                                            c.g[A0][0] = 0u64;
                                                                            call_ptr(
                                                                                c, 0x0014a080,
                                                                                0x001fd748,
                                                                            )?;
                                                                            c.g[A2][0] = c.g[V0][0];
                                                                            c.g[A0][0] = c.g[FP][0];
                                                                            c.g[A1][0] = 0u64;
                                                                            c.g[RA][0] =
                                                                                sx(0x001fd75c);
                                                                            c.g[A3][0] = 0x7;
                                                                            call_ptr(
                                                                                c, 0x00146a70,
                                                                                0x001fd75c,
                                                                            )?;
                                                                            let t =
                                                                                c.g[V0][0] == 0u64;
                                                                            c.g[V1][0] = 0x4a0000;
                                                                            if t {
                                                                                break 'b_1fd88c;
                                                                            } else {
                                                                                break 'b_1fd764;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 5;
                                                                    c.g[RA][0] = sx(0x001fd76c);
                                                                    c.g[A0][0] = c.g[S7][0];
                                                                    call_ptr(
                                                                        c, 0x00144be0, 0x001fd76c,
                                                                    )?;
                                                                    c.g[V1][0] = 0x10;
                                                                    let t =
                                                                        c.g[V0][0] != c.g[V1][0];
                                                                    c.g[A1][0] = 0x470000;
                                                                    if t {
                                                                        break 'b_1fd8ac;
                                                                    } else {
                                                                        c.cycles += 7;
                                                                        c.g[RA][0] = sx(0x001fd780);
                                                                        c.g[A0][0] = 0u64;
                                                                        call_ptr(
                                                                            c, 0x0014a080,
                                                                            0x001fd780,
                                                                        )?;
                                                                        c.g[A0][0] = c.g[FP][0];
                                                                        c.g[A2][0] = c.g[V0][0];
                                                                        c.g[A1][0] = 0u64;
                                                                        let t = true;
                                                                        c.g[A3][0] = 0x8;
                                                                        if t {
                                                                            break 'b_1fd878;
                                                                        } else {
                                                                            break 'b_1fd794;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            0x001fd794 => {
                                                                break 'b_1fd794;
                                                            }
                                                            0x001fd8c0 => {
                                                                break 'b_1fd8c0;
                                                            }
                                                            0x001fda18 => {
                                                                break 'b_1fda18;
                                                            }
                                                            _ => {
                                                                c.jump_out(
                                                                    t, 0x001fd320, 0x001fdb74,
                                                                );
                                                                return Ok(());
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 18;
                                                    c.g[A0][0] = 0x470000;
                                                    c.g[RA][0] = sx(0x001fd7a0);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_sub(0x20));
                                                    call_ptr(c, 0x00317670, 0x001fd7a0)?;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x8cc)
                                                        );
                                                    c.g[RA][0] = sx(0x001fd7ac);
                                                    c.g[A1][0] = c.g[V0][0];
                                                    call_ptr(c, 0x003a1000, 0x001fd7ac)?;
                                                    c.g[A0][0] = 0x470000;
                                                    c.g[RA][0] = sx(0x001fd7b8);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x0));
                                                    call_ptr(c, 0x00317670, 0x001fd7b8)?;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x8d0)
                                                        );
                                                    c.g[RA][0] = sx(0x001fd7c4);
                                                    c.g[A1][0] = c.g[V0][0];
                                                    call_ptr(c, 0x003a1000, 0x001fd7c4)?;
                                                    c.g[V0][0] = 0x530000;
                                                    c.g[A0][0] = 0x1;
                                                    c.g[V1][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x5bc8));
                                                    c.g[V0][0] = c
                                                        .lb((c.g[V1][0] as u32).wrapping_add(0x49));
                                                    let t = c.g[V0][0] == c.g[A0][0];
                                                    c.g[V1][0] = 0x4a0000;
                                                    if t {
                                                        break 'b_1fd88c;
                                                    } else {
                                                        'b_1fd814: {
                                                            c.cycles += 5;
                                                            c.g[RA][0] = sx(0x001fd7e4);
                                                            c.g[A0][0] = c.g[S7][0];
                                                            call_ptr(c, 0x00144be0, 0x001fd7e4)?;
                                                            c.g[V1][0] = 0xe;
                                                            let t = c.g[V0][0] != c.g[V1][0];

                                                            if t {
                                                                break 'b_1fd814;
                                                            } else {
                                                                c.cycles += 9;
                                                                c.g[RA][0] = sx(0x001fd7f8);
                                                                c.g[A0][0] = 0u64;
                                                                call_ptr(
                                                                    c, 0x0014a080, 0x001fd7f8,
                                                                )?;
                                                                c.g[A2][0] = c.g[V0][0];
                                                                c.g[A0][0] = c.g[FP][0];
                                                                c.g[A1][0] = 0u64;
                                                                c.g[RA][0] = sx(0x001fd80c);
                                                                c.g[A3][0] = 0x9;
                                                                call_ptr(
                                                                    c, 0x00146a70, 0x001fd80c,
                                                                )?;
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[V1][0] = 0x4a0000;
                                                                if t {
                                                                    break 'b_1fd88c;
                                                                } else {
                                                                    break 'b_1fd814;
                                                                }
                                                            }
                                                        }
                                                        'b_1fd84c: {
                                                            c.cycles += 5;
                                                            c.g[RA][0] = sx(0x001fd81c);
                                                            c.g[A0][0] = c.g[S7][0];
                                                            call_ptr(c, 0x00144be0, 0x001fd81c)?;
                                                            c.g[V1][0] = 0xf;
                                                            let t = c.g[V0][0] != c.g[V1][0];

                                                            if t {
                                                                break 'b_1fd84c;
                                                            } else {
                                                                c.cycles += 9;
                                                                c.g[RA][0] = sx(0x001fd830);
                                                                c.g[A0][0] = 0u64;
                                                                call_ptr(
                                                                    c, 0x0014a080, 0x001fd830,
                                                                )?;
                                                                c.g[A2][0] = c.g[V0][0];
                                                                c.g[A0][0] = c.g[FP][0];
                                                                c.g[A1][0] = 0u64;
                                                                c.g[RA][0] = sx(0x001fd844);
                                                                c.g[A3][0] = 0xa;
                                                                call_ptr(
                                                                    c, 0x00146a70, 0x001fd844,
                                                                )?;
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[V1][0] = 0x4a0000;
                                                                if t {
                                                                    break 'b_1fd88c;
                                                                } else {
                                                                    break 'b_1fd84c;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 5;
                                                        c.g[RA][0] = sx(0x001fd854);
                                                        c.g[A0][0] = c.g[S7][0];
                                                        call_ptr(c, 0x00144be0, 0x001fd854)?;
                                                        c.g[V1][0] = 0x10;
                                                        let t = c.g[V0][0] != c.g[V1][0];
                                                        c.g[A1][0] = 0x470000;
                                                        if t {
                                                            break 'b_1fd8ac;
                                                        } else {
                                                            c.cycles += 6;
                                                            c.g[RA][0] = sx(0x001fd868);
                                                            c.g[A0][0] = 0u64;
                                                            call_ptr(c, 0x0014a080, 0x001fd868)?;
                                                            c.g[A0][0] = c.g[FP][0];
                                                            c.g[A2][0] = c.g[V0][0];
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A3][0] = 0xb;
                                                            break 'b_1fd878;
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x001fd880);
                                                call_ptr(c, 0x00146a70, 0x001fd880)?;
                                                if c.g[V0][0] != 0u64 {
                                                    c.g[A1][0] = 0x470000;
                                                    break 'b_1fd8ac;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V1][0] = 0x4a0000;
                                                    break 'b_1fd88c;
                                                }
                                            }
                                            c.cycles += 8;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[RA][0] = sx(0x001fd898);
                                            c.g[A1][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0x2530));
                                            call_ptr(c, 0x0039e8b8, 0x001fd898)?;
                                            c.g[A1][0] = 0x4a0000;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[RA][0] = sx(0x001fd8a8);
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x2538));
                                            call_ptr(c, 0x0039e8b8, 0x001fd8a8)?;
                                            c.g[A1][0] = 0x470000;
                                            break 'b_1fd8ac;
                                        }
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[S6][0];
                                        c.g[RA][0] = sx(0x001fd8b8);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30));
                                        call_ptr(c, 0x0039e8b8, 0x001fd8b8)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x970));
                                        if t {
                                            break 'b_1fdb44;
                                        } else {
                                            break 'b_1fd8c0;
                                        }
                                    }
                                    'b_1fd910: {
                                        'b_1fd8f8: {
                                            c.cycles += 10;
                                            c.g[A0][0] = 0x470000;
                                            c.g[RA][0] = sx(0x001fd8cc);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x30));
                                            call_ptr(c, 0x00317670, 0x001fd8cc)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                            c.g[RA][0] = sx(0x001fd8d8);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, 0x0039d860, 0x001fd8d8)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x8c8));
                                            c.g[A0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
                                            let t = (c.g[A0][0] as i64) < 0;
                                            c.g[S4][0] = c.g[V0][0];
                                            if t {
                                                break 'b_1fd8f8;
                                            } else {
                                                c.cycles += 4;
                                                c.mtc1(4, 0);
                                                c.cvt_s_w(0, 0);
                                                let t = true;

                                                if t {
                                                    break 'b_1fd910;
                                                } else {
                                                    break 'b_1fd8f8;
                                                }
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[V1][0] = sx((c.g[A0][0] as u32) >> 1);
                                        c.g[V0][0] = c.g[A0][0] & 0x1;
                                        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                        c.mtc1(2, 0);
                                        c.cvt_s_w(0, 0);
                                        c.add_s(0, 0, 0);
                                        break 'b_1fd910;
                                    }
                                    'b_1fda10: {
                                        'b_1fda04: {
                                            c.cycles += 45;
                                            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5540));
                                            c.g[RA][0] = sx(0x001fd91c);
                                            c.mul_s(12, 0, 12);
                                            call_ptr(c, 0x004139f8, 0x001fd91c)?;
                                            c.g[S3][0] =
                                                sx((c.g[SP][0] as u32).wrapping_add(0x620));
                                            c.g[RA][0] = sx(0x001fd928);
                                            c.g[A0][0] = c.g[V0][0];
                                            Callee {
                                                addr: 0x0020a1a8,
                                                f: ee::s05::FE_FormatMinSec,
                                            }
                                            .call(c, 0x001fd928)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x001fd934);
                                            c.g[A0][0] = c.g[S3][0];
                                            call_ptr(c, 0x002c2540, 0x001fd934)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[A0][0] = 0x470000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x16e8));
                                            c.g[S0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                            c.g[S1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                            c.g[RA][0] = sx(0x001fd954);
                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            call_ptr(c, 0x00317670, 0x001fd954)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                            c.g[A0][0] = c.g[S0][0];
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001fd964);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, t, 0x001fd964)?;
                                            c.g[S2][0] =
                                                sx((c.g[SP][0] as u32).wrapping_add(0x290));
                                            c.g[A1][0] = 0x4a0000;
                                            c.g[S1][0] = c.g[V0][0];
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x2550));
                                            c.g[RA][0] = sx(0x001fd97c);
                                            c.g[A0][0] = c.g[S2][0];
                                            call_ptr(c, 0x002c2540, 0x001fd97c)?;
                                            c.g[S0][0] =
                                                sx((c.g[SP][0] as u32).wrapping_add(0x660));
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S3][0];
                                            c.g[A1][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x001fd994);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x002c26d0, 0x001fd994)?;
                                            c.g[A1][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x001fd9a0);
                                            c.g[A0][0] = c.g[S4][0];
                                            call_ptr(c, 0x003a0d00, 0x001fd9a0)?;
                                            c.g[A0][0] = 0x470000;
                                            c.g[RA][0] = sx(0x001fd9ac);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                            call_ptr(c, 0x00317670, 0x001fd9ac)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x8cc));
                                            c.g[RA][0] = sx(0x001fd9b8);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, 0x003a1000, 0x001fd9b8)?;
                                            c.g[V0][0] = 0x8;
                                            let t = c.g[S5][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x7;
                                            if t {
                                                break 'b_1fda04;
                                            } else {
                                                'b_1fd9e4: {
                                                    c.cycles += 2;
                                                    let t = c.g[S5][0] != c.g[V0][0];
                                                    c.g[V0][0] = 0x6;
                                                    if t {
                                                        break 'b_1fd9e4;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x8c4));
                                                        c.g[A1][0] = 0u64;
                                                        c.g[RA][0] = sx(0x001fd9dc);
                                                        c.g[A2][0] = 0x2;
                                                        call_ptr(c, 0x00157b08, 0x001fd9dc)?;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = 0x6;
                                                        if t {
                                                            break 'b_1fda04;
                                                        } else {
                                                            break 'b_1fd9e4;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[S5][0] != c.g[V0][0];
                                                c.g[A0][0] = 0x470000;
                                                if t {
                                                    break 'b_1fda10;
                                                } else {
                                                    c.cycles += 6;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x8c4)
                                                        );
                                                    c.g[A1][0] = 0u64;
                                                    c.g[RA][0] = sx(0x001fd9fc);
                                                    c.g[A2][0] = 0x1;
                                                    call_ptr(c, 0x00157b08, 0x001fd9fc)?;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.g[A0][0] = 0x470000;
                                                        break 'b_1fda10;
                                                    } else {
                                                        break 'b_1fda04;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x470000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_1fdb10;
                                        } else {
                                            break 'b_1fda10;
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
                                    if t {
                                        break 'b_1fdb10;
                                    } else {
                                        break 'b_1fda18;
                                    }
                                }
                                'b_1fdb08: {
                                    c.cycles += 44;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[A0][0] = 0x470000;
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8c8));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x16b0));
                                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                    c.g[S3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[RA][0] = sx(0x001fda40);
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    call_ptr(c, 0x00317670, 0x001fda40)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                    c.g[A0][0] = c.g[S0][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001fda50);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, t, 0x001fda50)?;
                                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x290));
                                    c.g[A1][0] = 0x4a0000;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2558));
                                    c.g[RA][0] = sx(0x001fda68);
                                    c.g[A0][0] = c.g[S1][0];
                                    call_ptr(c, 0x002c2540, 0x001fda68)?;
                                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x620));
                                    c.g[A2][0] = c.g[S0][0];
                                    c.g[A3][0] = c.g[S3][0];
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x001fda80);
                                    c.g[A0][0] = c.g[S2][0];
                                    call_ptr(c, 0x002c26d0, 0x001fda80)?;
                                    c.g[A0][0] = 0x470000;
                                    c.g[RA][0] = sx(0x001fda8c);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x30));
                                    call_ptr(c, 0x00317670, 0x001fda8c)?;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001fda98);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fda98)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fdaa4);
                                    c.g[A1][0] = c.g[S2][0];
                                    call_ptr(c, 0x003a0d00, 0x001fdaa4)?;
                                    c.g[A0][0] = 0x470000;
                                    c.g[RA][0] = sx(0x001fdab0);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x68));
                                    call_ptr(c, 0x00317670, 0x001fdab0)?;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8cc));
                                    c.g[RA][0] = sx(0x001fdabc);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a1000, 0x001fdabc)?;
                                    c.g[V0][0] = 0xb;
                                    let t = c.g[S5][0] == c.g[V0][0];
                                    c.g[V0][0] = 0xa;
                                    if t {
                                        break 'b_1fdb08;
                                    } else {
                                        'b_1fdae8: {
                                            c.cycles += 2;
                                            let t = c.g[S5][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x9;
                                            if t {
                                                break 'b_1fdae8;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x8c4));
                                                c.g[A1][0] = 0u64;
                                                c.g[RA][0] = sx(0x001fdae0);
                                                c.g[A2][0] = 0x2;
                                                call_ptr(c, 0x00157b08, 0x001fdae0)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = 0x9;
                                                if t {
                                                    break 'b_1fdb08;
                                                } else {
                                                    break 'b_1fdae8;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[S5][0] != c.g[V0][0];
                                        c.g[A0][0] = 0x470000;
                                        if t {
                                            break 'b_1fdb2c;
                                        } else {
                                            c.cycles += 6;
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x8c4));
                                            c.g[A1][0] = 0u64;
                                            c.g[RA][0] = sx(0x001fdb00);
                                            c.g[A2][0] = 0x1;
                                            call_ptr(c, 0x00157b08, 0x001fdb00)?;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A0][0] = 0x470000;
                                            if t {
                                                break 'b_1fdb2c;
                                            } else {
                                                break 'b_1fdb08;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.g[A0][0] = 0x470000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x80));
                                break 'b_1fdb10;
                            }
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x001fdb18);
                            call_ptr(c, 0x00317670, 0x001fdb18)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8d0));
                            c.g[RA][0] = sx(0x001fdb24);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x001fdb24)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x970));
                            if t {
                                break 'b_1fdb44;
                            } else {
                                break 'b_1fdb2c;
                            }
                        }
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x001fdb34);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x98));
                        call_ptr(c, 0x00317670, 0x001fdb34)?;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8d0));
                        c.g[RA][0] = sx(0x001fdb40);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x001fdb40)?;
                        break 'b_1fdb40;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x970));
            break 'b_1fdb44;
        }
        c.cycles += 12;
        c.g[V0][0] = 0u64;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x960));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x950));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x940));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x930));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x920));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x910));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x900));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x8f0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x8e0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x980));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, elf_image, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes (in the machine's RAM): the
    /// fakes only act when it holds `OC982_MAGIC`, so other tests that reach
    /// the same override slots run the real callees.
    pub(crate) const OC982_CTRL: u32 = DATA + 0xe_0000;
    pub(crate) const OC982_MAGIC: u32 = 0x9820_c0de;
    /// The overlay object (`a0`).
    pub(crate) const OC982_SELF: u32 = DATA + 0x5_0000;
    /// The world (`gp - 0x848`) and what hangs off it.
    pub(crate) const OC982_WORLD: u32 = DATA + 0x6_0000;
    pub(crate) const OC982_GAME: u32 = DATA + 0x6_1000;
    pub(crate) const OC982_IFACE: u32 = DATA + 0x6_2000;
    pub(crate) const OC982_IFACE_VT: u32 = DATA + 0x6_3000;
    pub(crate) const OC982_MODEMAN: u32 = DATA + 0x6_4000;
    /// The game state bytes at `0x535bc8`: `+0x48` mode, `+0x49` flag, `+0x4a` event.
    pub(crate) const OC982_STATE: u32 = 0x0053_5bc8;

    /// The callee fakes: a call logs everything the caller set up (so the
    /// register dance is compared) and returns a pointer, or 0 where the
    /// seed's zero mask says so.
    fn oc982_fake_call(c: &mut Cpu, addr: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let mask = c.bus.read64(OC982_CTRL + 8);
        if n < 64 && (mask >> n) & 1 == 1 {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc982_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC982_CTRL) != OC982_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc982_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc982_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc982_fakes!(
        (
            oc982_fake_00144be0,
            OC982_PREV_00144be0,
            ee::s01::OVERRIDE_00144BE0,
            ee::s01::f_00144be0__generated,
            0x00144be0
        ),
        (
            oc982_fake_00144bf0,
            OC982_PREV_00144bf0,
            ee::s01::OVERRIDE_00144BF0,
            ee::s01::f_00144bf0__generated,
            0x00144bf0
        ),
        (
            oc982_fake_00145750,
            OC982_PREV_00145750,
            ee::s01::OVERRIDE_00145750,
            ee::s01::f_00145750__generated,
            0x00145750
        ),
        (
            oc982_fake_00146a70,
            OC982_PREV_00146a70,
            ee::s01::OVERRIDE_00146A70,
            ee::s01::f_00146a70__generated,
            0x00146a70
        ),
        (
            oc982_fake_00146e98,
            OC982_PREV_00146e98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x00146e98
        ),
        (
            oc982_fake_0014a080,
            OC982_PREV_0014a080,
            ee::s01::OVERRIDE_0014A080,
            ee::s01::f_0014a080__generated,
            0x0014a080
        ),
        (
            oc982_fake_0014dc80,
            OC982_PREV_0014dc80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc982_fake_0014dd58,
            OC982_PREV_0014dd58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc982_fake_0014ee58,
            OC982_PREV_0014ee58,
            ee::s01::OVERRIDE_0014EE58,
            ee::s01::f_0014ee58__generated,
            0x0014ee58
        ),
        (
            oc982_fake_0014eec8,
            OC982_PREV_0014eec8,
            ee::s01::OVERRIDE_0014EEC8,
            ee::s01::f_0014eec8__generated,
            0x0014eec8
        ),
        (
            oc982_fake_00155130,
            OC982_PREV_00155130,
            ee::s01::OVERRIDE_00155130,
            ee::s01::f_00155130__generated,
            0x00155130
        ),
        (
            oc982_fake_00155190,
            OC982_PREV_00155190,
            ee::s01::OVERRIDE_00155190,
            ee::s01::f_00155190__generated,
            0x00155190
        ),
        (
            oc982_fake_00155208,
            OC982_PREV_00155208,
            ee::s01::OVERRIDE_00155208,
            ee::s01::f_00155208__generated,
            0x00155208
        ),
        (
            oc982_fake_00157b08,
            OC982_PREV_00157b08,
            ee::s01::OVERRIDE_00157B08,
            ee::s01::f_00157b08__generated,
            0x00157b08
        ),
        (
            oc982_fake_00195ff0,
            OC982_PREV_00195ff0,
            ee::s02::OVERRIDE_00195FF0,
            ee::s02::f_00195ff0__generated,
            0x00195ff0
        ),
        (
            oc982_fake_001f8df8,
            OC982_PREV_001f8df8,
            ee::s04::OVERRIDE_001F8DF8,
            ee::s04::f_001f8df8__generated,
            0x001f8df8
        ),
        (
            oc982_fake_001fd190,
            OC982_PREV_001fd190,
            ee::s04::OVERRIDE_001FD190,
            ee::s04::f_001fd190__generated,
            0x001fd190
        ),
        (
            oc982_fake_0020a1a8,
            OC982_PREV_0020a1a8,
            ee::s05::OVERRIDE_0020A1A8,
            ee::s05::f_0020a1a8__generated,
            0x0020a1a8
        ),
        (
            oc982_fake_0020a380,
            OC982_PREV_0020a380,
            ee::s05::OVERRIDE_0020A380,
            ee::s05::f_0020a380__generated,
            0x0020a380
        ),
        (
            oc982_fake_0020ab50,
            OC982_PREV_0020ab50,
            ee::s05::OVERRIDE_0020AB50,
            ee::s05::f_0020ab50__generated,
            0x0020ab50
        ),
        (
            oc982_fake_0020d110,
            OC982_PREV_0020d110,
            ee::s05::OVERRIDE_0020D110,
            ee::s05::f_0020d110__generated,
            0x0020d110
        ),
        (
            oc982_fake_0020d190,
            OC982_PREV_0020d190,
            ee::s05::OVERRIDE_0020D190,
            ee::s05::f_0020d190__generated,
            0x0020d190
        ),
        (
            oc982_fake_00231ab8,
            OC982_PREV_00231ab8,
            ee::s06::OVERRIDE_00231AB8,
            ee::s06::f_00231ab8__generated,
            0x00231ab8
        ),
        (
            oc982_fake_00258790,
            OC982_PREV_00258790,
            ee::s06::OVERRIDE_00258790,
            ee::s06::f_00258790__generated,
            0x00258790
        ),
        (
            oc982_fake_00289b70,
            OC982_PREV_00289b70,
            ee::s07::OVERRIDE_00289B70,
            ee::s07::f_00289b70__generated,
            0x00289b70
        ),
        (
            oc982_fake_00289bb8,
            OC982_PREV_00289bb8,
            ee::s07::OVERRIDE_00289BB8,
            ee::s07::f_00289bb8__generated,
            0x00289bb8
        ),
        (
            oc982_fake_0028b180,
            OC982_PREV_0028b180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028b180
        ),
        (
            oc982_fake_00294f78,
            OC982_PREV_00294f78,
            ee::s08::OVERRIDE_00294F78,
            ee::s08::f_00294f78__generated,
            0x00294f78
        ),
        (
            oc982_fake_002c2508,
            OC982_PREV_002c2508,
            ee::s08::OVERRIDE_002C2508,
            ee::s08::f_002c2508__generated,
            0x002c2508
        ),
        (
            oc982_fake_002c2540,
            OC982_PREV_002c2540,
            ee::s08::OVERRIDE_002C2540,
            ee::s08::f_002c2540__generated,
            0x002c2540
        ),
        (
            oc982_fake_002c26d0,
            OC982_PREV_002c26d0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c26d0
        ),
        (
            oc982_fake_00317670,
            OC982_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc982_fake_00317d70,
            OC982_PREV_00317d70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x00317d70
        ),
        (
            oc982_fake_00397cc0,
            OC982_PREV_00397cc0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x00397cc0
        ),
        (
            oc982_fake_0039b960,
            OC982_PREV_0039b960,
            ee::s13::OVERRIDE_0039B960,
            ee::s13::f_0039b960__generated,
            0x0039b960
        ),
        (
            oc982_fake_0039c778,
            OC982_PREV_0039c778,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039c778
        ),
        (
            oc982_fake_0039c870,
            OC982_PREV_0039c870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039c870
        ),
        (
            oc982_fake_0039d860,
            OC982_PREV_0039d860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039d860
        ),
        (
            oc982_fake_0039e8b8,
            OC982_PREV_0039e8b8,
            ee::s13::OVERRIDE_0039E8B8,
            ee::s13::f_0039e8b8__generated,
            0x0039e8b8
        ),
        (
            oc982_fake_0039e948,
            OC982_PREV_0039e948,
            ee::s13::OVERRIDE_0039E948,
            ee::s13::f_0039e948__generated,
            0x0039e948
        ),
        (
            oc982_fake_0039f190,
            OC982_PREV_0039f190,
            ee::s13::OVERRIDE_0039F190,
            ee::s13::f_0039f190__generated,
            0x0039f190
        ),
        (
            oc982_fake_0039f320,
            OC982_PREV_0039f320,
            ee::s13::OVERRIDE_0039F320,
            ee::s13::f_0039f320__generated,
            0x0039f320
        ),
        (
            oc982_fake_003a0c50,
            OC982_PREV_003a0c50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a0c50
        ),
        (
            oc982_fake_003a0d00,
            OC982_PREV_003a0d00,
            ee::s13::OVERRIDE_003A0D00,
            ee::s13::f_003a0d00__generated,
            0x003a0d00
        ),
        (
            oc982_fake_003a0e90,
            OC982_PREV_003a0e90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a0e90
        ),
        (
            oc982_fake_003a1000,
            OC982_PREV_003a1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a1000
        ),
        (
            oc982_fake_004139f8,
            OC982_PREV_004139f8,
            ee::s15::OVERRIDE_004139F8,
            ee::s15::f_004139f8__generated,
            0x004139f8
        ),
        (
            oc982_fake_00416210,
            OC982_PREV_00416210,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x00416210
        ),
        (
            oc982_fake_00417828,
            OC982_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x00417828
        ),
    );

    /// A test machine with the game image (jump tables, `.lit4`), the callee
    /// fakes armed, and a per-seed zero mask for the fakes' results.
    pub(crate) fn oc982_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        elf_image(c);
        oc982_install_fakes();
        c.bus.write32(OC982_CTRL, OC982_MAGIC);
        let noise = |r: &mut Rng| (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        let mask = match seed % 5 {
            0 => 0,
            1 => u64::MAX,
            _ => noise(&mut r) & noise(&mut r),
        };
        c.bus.write64(OC982_CTRL + 8, mask);
        r
    }

    /// The world with the interface object whose vtable the front end calls
    /// through (`world+0x8c -> +4 -> +0x20 / +0x24`), the game (`+0x84`) and
    /// the game-mode manager (`+0xc0`).
    pub(crate) fn oc982_world(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC982_WORLD);
        c.bus.write32(OC982_WORLD + 0x84, OC982_GAME);
        c.bus.write32(OC982_WORLD + 0x8c, OC982_IFACE);
        c.bus.write32(OC982_IFACE + 4, OC982_IFACE_VT);
        c.bus
            .write16(OC982_IFACE_VT + 0x20, (r.next_u32() % 64 * 4) as u16);
        c.bus.write32(OC982_IFACE_VT + 0x24, fakes::FAKE);
        c.bus.write32(OC982_WORLD + 0xc0, OC982_MODEMAN);
        c.bus.write32(OC982_WORLD + 0x78, DATA + 0x6_5000);
    }

    /// The distinct cycle totals the generated body takes over the seeds
    /// (a path count: the fixtures must reach different arms).
    fn oc982_path_count(generated: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) -> usize {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, generated);
            seen.insert(c.cycles - before);
        }
        seen.len()
    }

    // -- 0x001f7958 OVScreenTopTimes_Init ------------------------------------

    fn top_times_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc982_machine(c, seed, 0x1f7958, 70);
        oc982_world(c, &mut r);
        // Event type (5, the peak events 6..11, others) and mode (0..7).
        let ev = [5u8, 6, 9, 11, 0, 3, 12, 7][seed as usize % 8];
        let mode = ((seed / 3) % 8) as u8;
        c.bus.write8(OC982_STATE + 0x4a, ev);
        c.bus.write8(OC982_STATE + 0x48, mode);
        c.bus.write32(GAME_GP.wrapping_sub(0x9f0), (seed / 5) % 2);
        c.g[A0][0] = OC982_SELF as u64;
        c.pc = 0x001f_7958;
    }

    /// OVScreenTopTimes_Init with every callee faked (some seeds unwind inside
    /// a call, some results are zero), over the event types and modes.
    #[test]
    fn ov_screen_top_times_init_matches() {
        check(
            ee::s04::f_001f7958__generated,
            ov_screen_top_times_init,
            96,
            &top_times_setup,
        );
        assert!(
            oc982_path_count(ee::s04::f_001f7958__generated, 96, &top_times_setup) >= 4,
            "fixture reaches too few paths"
        );
    }

    // -- 0x001f8168 OVScreenPauseFreeride_Init -------------------------------

    fn pause_freeride_init_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc982_machine(c, seed, 0x1f8168, 80);
        oc982_world(c, &mut r);
        let (engine, x, y) = (DATA + 0x5_1000, DATA + 0x5_2000, DATA + 0x5_3000);
        c.bus.write32(OC982_SELF + 0x10, engine);
        c.bus.write32(engine + 0x10, x);
        c.bus.write32(x + 4, y);
        c.bus.write16(y + 0x20, (r.next_u32() % 64 * 4) as u16);
        c.bus.write32(y + 0x24, fakes::FAKE);
        // Menu slot count and the slots' string indices.
        c.bus.write32(OC982_SELF + 0xbc, seed % 10);
        for i in 0..8 {
            c.bus.write32(OC982_SELF + 0x9c + 4 * i, r.next_u32() % 16);
        }
        let state = if seed % 3 == 0 { 0xb } else { 0 };
        c.bus.write32(OC982_GAME + 0x214, state);
        c.bus.write32(OC982_GAME + 0x224, (seed / 2) % 2);
        c.bus.write32(0x0053_4b30, (seed / 4) % 2);
        c.g[A0][0] = OC982_SELF as u64;
        c.pc = 0x001f_8168;
    }

    /// OVScreenPauseFreeride_Init: zero and non-zero screen/object results,
    /// the BE state 0xb or not, one to eight menu slots, unwinds inside calls.
    #[test]
    fn ov_screen_pause_freeride_init_matches() {
        check(
            ee::s04::f_001f8168__generated,
            ov_screen_pause_freeride_init,
            96,
            &pause_freeride_init_setup,
        );
        assert!(
            oc982_path_count(
                ee::s04::f_001f8168__generated,
                96,
                &pause_freeride_init_setup
            ) >= 4,
            "fixture reaches too few paths"
        );
    }

    // -- 0x001f8948 OVScreenPauseFreeride_OnWidgetAction ---------------------

    fn pause_freeride_action_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc982_machine(c, seed, 0x1f8948, 24);
        oc982_world(c, &mut r);
        let (engine, snd, snd_vt) = (DATA + 0x5_1000, DATA + 0x5_4000, DATA + 0x5_5000);
        let (wid, wid_obj, pause) = (DATA + 0x5_8000, DATA + 0x5_9000, DATA + 0x5_6000);
        c.bus.write32(OC982_SELF + 0x10, engine);
        // The engine's sound object, or the null sound in the image (gp + 0x2968).
        let sound = if seed % 2 == 0 { snd } else { 0 };
        c.bus.write32(engine + 0x14, sound);
        c.bus.write32(snd, snd_vt);
        c.bus.write32(GAME_GP + 0x2968, snd_vt);
        c.bus.write16(snd_vt + 0x10, (r.next_u32() % 64 * 4) as u16);
        c.bus.write32(snd_vt + 0x14, fakes::FAKE);
        c.bus.write32(OC982_SELF + 0xc0, DATA + 0x5_a000);
        c.bus.write32(OC982_SELF + 0xd0, seed % 2);
        // The unlock flag of every slot.
        for i in 0..16 {
            c.bus.write32(OC982_SELF + 0xd8 + 4 * i, r.next_u32() % 2);
        }
        // The widget: slot index and its object's vtable call.
        let idx = (seed / 3 + seed / 10) % 15;
        c.bus.write32(wid + 0x18, idx);
        c.bus.write32(wid + 0x8, wid_obj);
        c.bus
            .write16(wid_obj + 0x58, (r.next_u32() % 64 * 4) as u16);
        c.bus.write32(wid_obj + 0x5c, fakes::FAKE);
        c.bus
            .write16(wid_obj + 0x48, (r.next_u32() % 64 * 4) as u16);
        c.bus.write32(wid_obj + 0x4c, fakes::FAKE);
        c.bus
            .write16(wid_obj + 0x40, (r.next_u32() % 64 * 4) as u16);
        c.bus.write32(wid_obj + 0x44, fakes::FAKE);
        // The pause-state record and the pause stack counter.
        let record = if seed % 3 == 0 { 0 } else { pause };
        c.bus.write32(GAME_GP.wrapping_sub(0x204), record);
        c.bus
            .write32(GAME_GP.wrapping_sub(0x69c), 1 + r.next_u32() % 4);
        c.g[A0][0] = OC982_SELF as u64;
        c.g[A1][0] = if seed % 11 == 0 { 0 } else { wid as u64 };
        c.g[A2][0] = (seed % 10) as u64;
        c.pc = 0x001f_8948;
    }

    /// OVScreenPauseFreeride_OnWidgetAction over every UI event (0..9), every
    /// menu slot (0..14), locked and unlocked entries, the engine's sound or
    /// the null sound, with and without the pause record; callees faked.
    #[test]
    fn ov_screen_pause_freeride_on_widget_action_matches() {
        check(
            ee::s04::f_001f8948__generated,
            ov_screen_pause_freeride_on_widget_action,
            220,
            &pause_freeride_action_setup,
        );
        assert!(
            oc982_path_count(
                ee::s04::f_001f8948__generated,
                220,
                &pause_freeride_action_setup
            ) >= 6,
            "fixture reaches too few paths"
        );
    }

    // -- 0x001fd320 OVScreenRivalPre_TransitionIn ----------------------------

    fn rival_pre_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc982_machine(c, seed, 0x1fd320, 90);
        oc982_world(c, &mut r);
        // Event type 4..11 (rival time/points, the peak events) and others.
        let ev = [4u8, 5, 6, 7, 8, 9, 10, 11, 0, 12][seed as usize % 10];
        c.bus.write8(OC982_STATE + 0x4a, ev);
        c.bus.write8(OC982_STATE + 0x49, ((seed / 10) % 3) as u8);
        c.bus.write8(OC982_STATE + 0x48, ((seed / 3) % 4) as u8);
        c.bus.write32(OC982_SELF + 0x40, DATA + 0x5_a000);
        // The game-mode manager: times and scores read from it.
        let value = match seed % 4 {
            0 => 0,
            1 => r.next_u32() % 100_000,
            2 => 0x8000_0000 | r.next_u32(),
            _ => r.next_u32() % 100,
        };
        c.bus.write32(OC982_MODEMAN + 0x78, value);
        c.bus.write32(OC982_MODEMAN + 0x7c, r.next_u32() % 100_000);
        c.g[A0][0] = OC982_SELF as u64;
        c.g[A1][0] = if seed % 7 == 0 { 0 } else { 1 };
        c.pc = 0x001f_d320;
    }

    /// OVScreenRivalPre_TransitionIn over the event types, the state flags and
    /// time or score values (including a negative one for the unsigned float
    /// conversion), with and without the message argument; callees faked.
    #[test]
    fn ov_screen_rival_pre_transition_in_matches() {
        check(
            ee::s05::f_001fd320__generated,
            ov_screen_rival_pre_transition_in,
            140,
            &rival_pre_setup,
        );
        assert!(
            oc982_path_count(ee::s05::f_001fd320__generated, 140, &rival_pre_setup) >= 6,
            "fixture reaches too few paths"
        );
    }
}
