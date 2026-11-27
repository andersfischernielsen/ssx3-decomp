//! Staging module oc986 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc986`.

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
    (0x001f_6f30, count_medals, "cFEStateCareerStats_CountMedals"),
    (0x001f_ab18, ingame_error_init, "OVScreenInGameError_Init"),
    (
        0x001f_cec0,
        mfpre_transition_in,
        "OVScreenMultiFreestylePre_TransitionIn",
    ),
    (0x001f_dc30, savereplay_init, "OVScreenSaveReplay_Init"),
    (
        0x001f_e6e8,
        savereplay_widget_action,
        "OVScreenSaveReplay_OnWidgetAction",
    ),
];

/// `void cFEStateCareerStats::CountMedals(this)` (0x001f6f30): tallies the medals the
/// current rider still has to earn. For the three reward checks, the 8 track
/// medals (`D_004780D0`, mode 4 on every third track) and the 12 trophy
/// medals (`D_00478110`, mode 5 on every fourth) it asks the reward interface
/// whether the medal is missing and, if so, bumps the counters at
/// `this+0x58/0x5c/0x60/0x68/0x6c/0x70` (and the bit masks).
pub fn count_medals(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f6ff8: {
            c.cycles += 44;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            call_ptr(c, 0x0014dc80, 0x001f6f64)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0xd;
            call_ptr(c, 0x0014dd58, 0x001f6f70)?;
            c.g[S4][0] = c.g[V0][0];
            call_ptr(c, 0x0014dc80, 0x001f6f78)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x001f6f84)?;
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, 0x0014dc80, 0x001f6f8c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x001f6f98)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0u64;
            c.g[FP][0] = c.g[V0][0];
            call_ptr(c, 0x00146e98, 0x001f6fa8)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x00147398, 0x001f6fb4)?;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0u64;
            c.g[S5][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            call_ptr(c, 0x00146e98, 0x001f6fc8)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[A2][0] = 0u64;
            call_ptr(c, 0x00157b08, 0x001f6fd8)?;
            let t = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.g[S1][0];
            if t {
                break 'b_1f6ff8;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x68), c.g[V1][0]);
                break 'b_1f6ff8;
            }
        }
        'b_1f7030: {
            c.cycles += 8;
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x00146e98, 0x001f7000)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[A2][0] = 0x1;
            call_ptr(c, 0x00157b08, 0x001f7010)?;
            let t = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.g[S1][0];
            if t {
                break 'b_1f7030;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x68), c.g[V1][0]);
                break 'b_1f7030;
            }
        }
        'b_1f706c: {
            c.cycles += 8;
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x00146e98, 0x001f7038)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[A2][0] = 0x2;
            call_ptr(c, 0x00157b08, 0x001f7048)?;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0x480000;
            if t {
                break 'b_1f706c;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x68), c.g[V1][0]);
                c.g[V0][0] = 0x480000;
                break 'b_1f706c;
            }
        }
        c.cycles += 5;
        c.g[S1][0] = 0u64;
        c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7f30));
        c.g[S2][0] = 0x3;
        c.g[S7][0] = 0x4;
        c.g[S6][0] = 0x1;
        'l_1f7080: loop {
            'b_1f708c: {
                c.cycles += 3;
                c.div(0, 17, 18);
                if c.g[S2][0] == 0u64 {
                    break 'b_1f708c;
                } else {
                    break 'b_1f708c;
                }
            }
            'b_1f70d8: {
                c.cycles += 11;
                c.g[T0][0] = c.lw((c.g[S3][0] as u32));
                c.g[A3][0] = 0u64;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = c.g[S5][0];
                c.g[V0][0] = c.hi[0];
                if c.g[V0][0] == 0 {
                    c.g[A3][0] = c.g[S7][0];
                }
                call_ptr(c, 0x00158910, 0x001f70ac)?;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                if c.g[V0][0] != 0u64 {
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    break 'b_1f70d8;
                } else {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                    c.g[A0][0] = sx((c.g[S6][0] as u32) << ((c.g[S1][0] as u32) & 31));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x60), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), c.g[V1][0]);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    break 'b_1f70d8;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 8i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001f7080);
                }
                continue 'l_1f7080;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x480000;
                c.g[S1][0] = 0u64;
                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ef0));
                c.g[S3][0] = 0x1;
                'l_1f70f8: loop {
                    'b_1f7114: {
                        'b_1f710c: {
                            c.cycles += 3;
                            c.g[V0][0] = c.g[S1][0] & 0x3;
                            if c.g[V0][0] != 0u64 {
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32));
                                break 'b_1f710c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = 0x5;
                                if t {
                                    break 'b_1f7114;
                                } else {
                                    break 'b_1f710c;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.g[FP][0];
                        call_ptr(c, 0x00145870, 0x001f7114)?;
                        break 'b_1f7114;
                    }
                    'b_1f7158: {
                        c.cycles += 9;
                        c.g[T0][0] = c.lw((c.g[S2][0] as u32));
                        c.g[A3][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S4][0];
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = c.g[S5][0];
                        call_ptr(c, 0x00158910, 0x001f712c)?;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        if c.g[V0][0] != 0u64 {
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            break 'b_1f7158;
                        } else {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                            c.g[A0][0] = sx((c.g[S3][0] as u32) << ((c.g[S1][0] as u32) & 31));
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                            c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x70), c.g[V0][0]);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x6c), c.g[V1][0]);
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            break 'b_1f7158;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 12i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001f70f8);
                        }
                        continue 'l_1f70f8;
                    } else {
                        c.cycles += 12;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// `void OVScreenInGameError_Init(OVScreen* this)` (0x001fab18): builds the
/// in-game error pop-up: adds the screen, hides the cancel/continue widgets,
/// picks the error text by the cheat state of `cGameComm` (`gp-0x204`),
/// and, with a game comm, pings the server and sets the cancel button.
pub fn ingame_error_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fab68: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = 0x470000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x630));
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001fab3c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001fab50)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1fab68;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001fab68)?;
                break 'b_1fab68;
            }
        }
        'b_1fabb0: {
            c.cycles += 11;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0020a380, 0x001fab70)?;
            c.g[A0][0] = 0x470000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x620));
            call_ptr(c, 0x00317670, 0x001fab7c)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001fab88)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0u64 {
                c.g[A0][0] = 0x470000;
                break 'b_1fabb0;
            } else {
                c.cycles += 7;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x610));
                call_ptr(c, 0x00317670, 0x001faba0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x001fabac)?;
                c.g[A0][0] = 0x470000;
                break 'b_1fabb0;
            }
        }
        'b_1fae78: {
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5f8));
            call_ptr(c, 0x00317670, 0x001fabb8)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001fabc4)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                break 'b_1fae78;
            } else {
                'b_1fad0c: {
                    'b_1fad08: {
                        'b_1facf0: {
                            'b_1facd4: {
                                'b_1fac48: {
                                    c.cycles += 13;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0x1;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001fabe8)?;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                                    c.g[V0][0] = c.g[V0][0] | 0x80;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24));
                                    let t = c.g[V1][0] == 0u64;
                                    c.g[A1][0] = 0x1;
                                    if t {
                                        break 'b_1fac48;
                                    } else {
                                        'b_1fac18: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[A1][0];
                                            c.g[V0][0] = 0x2;
                                            if t {
                                                break 'b_1fac18;
                                            } else {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x470000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x5e8));
                                                if t {
                                                    break 'b_1facd4;
                                                } else {
                                                    break 'b_1fac18;
                                                }
                                            }
                                        }
                                        'b_1fac2c: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x3;
                                            if t {
                                                break 'b_1fac2c;
                                            } else {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x470000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x5e8));
                                                if t {
                                                    break 'b_1facd4;
                                                } else {
                                                    break 'b_1fac2c;
                                                }
                                            }
                                        }
                                        'b_1fac40: {
                                            c.cycles += 2;
                                            if c.g[V1][0] != c.g[V0][0] {
                                                c.g[A0][0] = 0x470000;
                                                break 'b_1fac40;
                                            } else {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x470000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x5e8));
                                                if t {
                                                    break 'b_1facd4;
                                                } else {
                                                    break 'b_1fac40;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5c8));
                                        if t {
                                            break 'b_1facd4;
                                        } else {
                                            break 'b_1fac48;
                                        }
                                    }
                                }
                                'b_1fac9c: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8c));
                                        break 'b_1fac9c;
                                    } else {
                                        'b_1fac70: {
                                            c.cycles += 4;
                                            c.sw((c.g[S1][0] as u32).wrapping_add(0xa0), 0u64);
                                            c.g[V1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
                                            let t = c.g[V1][0] != c.g[A1][0];
                                            c.g[V0][0] = 0x2;
                                            if t {
                                                break 'b_1fac70;
                                            } else {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x470000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x5a8));
                                                if t {
                                                    break 'b_1facd4;
                                                } else {
                                                    break 'b_1fac70;
                                                }
                                            }
                                        }
                                        'b_1fac84: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x3;
                                            if t {
                                                break 'b_1fac84;
                                            } else {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x470000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x5a8));
                                                if t {
                                                    break 'b_1facd4;
                                                } else {
                                                    break 'b_1fac84;
                                                }
                                            }
                                        }
                                        'b_1fac94: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[A0][0] = 0x470000;
                                            if t {
                                                break 'b_1fac94;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x5a8));
                                                if t {
                                                    break 'b_1facd4;
                                                } else {
                                                    break 'b_1fac94;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5a8));
                                        if t {
                                            break 'b_1facd4;
                                        } else {
                                            break 'b_1fac9c;
                                        }
                                    }
                                }
                                'b_1facc4: {
                                    c.cycles += 2;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                                        break 'b_1facc4;
                                    } else {
                                        c.cycles += 8;
                                        c.g[A0][0] = 0x470000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x588));
                                        call_ptr(c, 0x00317670, 0x001facb0)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x003a1000, 0x001facbc)?;
                                        let t = true;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0xa0), 0u64);
                                        if t {
                                            break 'b_1fad08;
                                        } else {
                                            break 'b_1facc4;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0xa0), 0u64);
                                if t {
                                    break 'b_1facf0;
                                } else {
                                    c.cycles += 2;
                                    c.g[A0][0] = 0x470000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x588));
                                    break 'b_1facd4;
                                }
                            }
                            c.cycles += 7;
                            call_ptr(c, 0x00317670, 0x001facdc)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x001face8)?;
                            let t = true;
                            c.g[A0][0] = 0x470000;
                            if t {
                                break 'b_1fad0c;
                            } else {
                                break 'b_1facf0;
                            }
                        }
                        c.cycles += 6;
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5a8));
                        call_ptr(c, 0x00317670, 0x001facfc)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x001fad08)?;
                        break 'b_1fad08;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = 0x470000;
                    break 'b_1fad0c;
                }
                'b_1fad88: {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x570));
                    call_ptr(c, 0x00317670, 0x001fad14)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fad20)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                    if t {
                        break 'b_1fad88;
                    } else {
                        'b_1fad70: {
                            'b_1fad58: {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));
                                if c.g[V1][0] == 0u64 {
                                    c.g[A0][0] = 0x470000;
                                    break 'b_1fad58;
                                } else {
                                    c.cycles += 8;
                                    c.g[A0][0] = 0x470000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x560));
                                    call_ptr(c, 0x00317670, 0x001fad44)?;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x003a1000, 0x001fad50)?;
                                    let t = true;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    if t {
                                        break 'b_1fad70;
                                    } else {
                                        break 'b_1fad58;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x550));
                            call_ptr(c, 0x00317670, 0x001fad60)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x003a1000, 0x001fad6c)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            break 'b_1fad70;
                        }
                        c.cycles += 6;
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001fad84)?;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
                        break 'b_1fad88;
                    }
                }
                'b_1fadc0: {
                    c.cycles += 9;
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x538));
                    call_ptr(c, 0x00317670, 0x001fad94)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fada0)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_1fadc0;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001fadc0)?;
                        break 'b_1fadc0;
                    }
                }
                'b_1fadf8: {
                    c.cycles += 9;
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x528));
                    call_ptr(c, 0x00317670, 0x001fadcc)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fadd8)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_1fadf8;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001fadf8)?;
                        break 'b_1fadf8;
                    }
                }
                'b_1fae30: {
                    c.cycles += 9;
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x518));
                    call_ptr(c, 0x00317670, 0x001fae04)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fae10)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_1fae30;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001fae30)?;
                        break 'b_1fae30;
                    }
                }
                'b_1fae68: {
                    c.cycles += 9;
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x500));
                    call_ptr(c, 0x00317670, 0x001fae3c)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fae48)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_1fae68;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001fae68)?;
                        break 'b_1fae68;
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0u64;
                Callee {
                    addr: 0x001fb378,
                    f: ee::s04::OVScreenInGameError_ShowContinueButton,
                }
                .call(c, 0x001fae74)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                break 'b_1fae78;
            }
        }
        'b_1faef4: {
            'b_1faef0: {
                'b_1faee4: {
                    c.cycles += 2;
                    let t = c.g[A0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    if t {
                        break 'b_1faee4;
                    } else {
                        c.cycles += 18;
                        c.g[A2][0] = 0x3;
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                        call_ptr(c, 0x002583a8, 0x001fae8c)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x9c), c.g[V0][0]);
                        Callee {
                            addr: 0x001faff8,
                            f: ee::s04::OVScreenInGameError_ShowCancelGame,
                        }
                        .call(c, 0x001faea4)?;
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4e8));
                        call_ptr(c, 0x00317670, 0x001faeb0)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001faebc)?;
                        c.g[S0][0] = c.g[V0][0];
                        let t = c.g[S0][0] == 0u64;
                        c.g[A0][0] = 0x470000;
                        if t {
                            break 'b_1faef0;
                        } else {
                            c.cycles += 7;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4d8));
                            call_ptr(c, 0x00317670, 0x001faed0)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x001faedc)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_1faef4;
                            } else {
                                break 'b_1faee4;
                            }
                        }
                    }
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0x1;
                Callee {
                    addr: 0x001faff8,
                    f: ee::s04::OVScreenInGameError_ShowCancelGame,
                }
                .call(c, 0x001faef0)?;
                break 'b_1faef0;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1faef4;
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

/// `int OVScreenMultiFreestylePre_TransitionIn(OVScreen* this, void* msg)`
/// (0x001fcec0): with a message, shifts the layout groups by game mode
/// (modes 2/3: `screendata`-style group by +130 x; modes 5/6: three groups
/// moved in y). Returns 0.
pub fn mfpre_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fd0c4: {
            'b_1fd0c0: {
                c.cycles += 5;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                let t = c.g[A1][0] == 0u64;
                c.g[S0][0] = c.g[A0][0];
                if t {
                    break 'b_1fd0c0;
                } else {
                    'b_1fcf70: {
                        c.cycles += 11;
                        call_ptr(c, 0x0014dc80, 0x001fcedc)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0014dd58, 0x001fcee8)?;
                        c.g[V1][0] = 0x530000;
                        c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c10));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2));
                        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = 0x470000;
                        if t {
                            break 'b_1fcf70;
                        } else {
                            c.cycles += 28;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1778));
                            call_ptr(c, 0x00317670, 0x001fcf08)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001fcf14)?;
                            c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
                            c.ldr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                            c.sdr(3, (c.g[SP][0] as u32));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                            c.g[AT][0] = 0x43020000;
                            c.mtc1(1, 3);
                            c.lwc1(0, (c.g[SP][0] as u32));
                            c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x4));
                            c.add_s(0, 0, 3);
                            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
                            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x17));
                            c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                            c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x4b));
                            c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x44));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            if t {
                                break 'b_1fd0c4;
                            } else {
                                break 'b_1fcf70;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        break 'b_1fd0c0;
                    } else {
                        c.cycles += 80;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2548));
                        call_ptr(c, 0x00317670, 0x001fcf88)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fcf94)?;
                        c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                        c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                        c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.sdr(6, (c.g[SP][0] as u32));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                        c.g[AT][0] = 0x42200000;
                        c.mtc1(1, 3);
                        c.g[A0][0] = 0x470000;
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.lwc1(2, (c.g[SP][0] as u32));
                        c.add_s(0, 0, 3);
                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x28));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
                        c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x27));
                        c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                        c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                        c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A2][0]);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x150));
                        call_ptr(c, 0x00317670, 0x001fcff4)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fd000)?;
                        c.ldl(4, (c.g[V0][0] as u32).wrapping_add(0x4b));
                        c.ldr(4, (c.g[V0][0] as u32).wrapping_add(0x44));
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.sdr(4, (c.g[SP][0] as u32));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                        c.g[AT][0] = 0x42860000;
                        c.mtc1(1, 3);
                        c.g[A0][0] = 0x470000;
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.lwc1(2, (c.g[SP][0] as u32));
                        c.add_s(0, 0, 3);
                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x28));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x27));
                        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                        c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
                        c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A1][0]);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x140));
                        call_ptr(c, 0x00317670, 0x001fd060)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fd06c)?;
                        c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
                        c.ldr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.sdr(3, (c.g[SP][0] as u32));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                        c.g[AT][0] = 0x42400000;
                        c.mtc1(1, 3);
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.lwc1(2, (c.g[SP][0] as u32));
                        c.sub_s(0, 0, 3);
                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x28));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
                        c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x27));
                        c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                        c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x4b));
                        c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x44));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                        break 'b_1fd0c0;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_1fd0c4;
        }
        c.cycles += 4;
        c.g[V0][0] = 0u64;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void OVScreenSaveReplay_Init(OVScreen* this)` (0x001fdc30): builds the
/// save-replay screen: sets its texts, labels the four memory-card slots
/// (`sprintf`, slot `i-1` stored at widget+0x18), adds the dark-blue overlay
/// screen, pushes pause state 2 and runs `cOVStateOverlay_Init`.
pub fn savereplay_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fddd0: {
            c.cycles += 24;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S1][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = 0x470000;
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xe0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S3][0] = 0x470000;
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S6][0] = 0x530000;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[S4][0] = 0x440000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001fdc74)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001fdc88)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1fddd0;
            } else {
                c.cycles += 66;
                c.g[A2][0] = 0u64;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[S2][0] = 0x1;
                call_ptr(c, 0x0039c870, 0x001fdca4)?;
                c.g[S5][0] = 0x470000;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf0));
                call_ptr(c, 0x00317670, 0x001fdcb4)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fdcc0)?;
                c.g[S0][0] = 0x4a0000;
                c.g[A0][0] = c.g[V0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2560));
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0c50, 0x001fdcd4)?;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x100));
                call_ptr(c, 0x00317670, 0x001fdce0)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fdcec)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001fdd08)?;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x118));
                call_ptr(c, 0x00317670, 0x001fdd14)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fdd20)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001fdd3c)?;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1608));
                call_ptr(c, 0x00317670, 0x001fdd48)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fdd54)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001fdd70)?;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x130));
                call_ptr(c, 0x00317670, 0x001fdd7c)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fdd88)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0c50, 0x001fdd94)?;
                c.g[A0][0] = c.g[SP][0];
                'l_1fdd98: loop {
                    c.cycles += 14;
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x148));
                    call_ptr(c, 0x00417828, 0x001fdda4)?;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001fddac)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fddb8)?;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 5i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001fdd98);
                        }
                        continue 'l_1fdd98;
                    } else {
                        break 'b_1fddd0;
                    }
                }
            }
        }
        'b_1fde04: {
            c.cycles += 10;
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1c88));
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001fdddc)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001fddf0)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[V0][0];
            if t {
                break 'b_1fde04;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001fde04)?;
                break 'b_1fde04;
            }
        }
        c.cycles += 23;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x28f0));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_add(0x66e8));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.g[A1][0] = 0x2;
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[A0][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.g[A0][0] = c.g[S1][0];
        c.sw((c.g[V0][0] as u32), c.g[A1][0]);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[V1][0]);
        Callee {
            addr: 0x0020a380,
            f: ee::s05::cOVStateOverlay_Init,
        }
        .call(c, 0x001fde38)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// `void OVScreenSaveReplay_OnWidgetAction(OVScreen* this, void* msg, int type)`
/// (0x001fe6e8): widget events of the save-replay screen (types 5, 6, 7:
/// confirm card slot, stop/skip, save), driving the memory-card manager
/// (`MCManager_Get`) through its virtual methods and the state at `this+0x1c0`.
pub fn savereplay_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fe8d4: {
            'b_1fe8d0: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A1][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[S2][0] == 0u64;
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                if t {
                    break 'b_1fe8d0;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        break 'b_1fe8d4;
                    } else {
                        'b_1fe830: {
                            'b_1fe7c8: {
                                c.cycles += 3;
                                c.g[S1][0] = 0x6;
                                let t = c.g[A2][0] == c.g[S1][0];
                                c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                                if t {
                                    break 'b_1fe7c8;
                                } else {
                                    'b_1fe754: {
                                        'b_1fe740: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x5;
                                            if t {
                                                break 'b_1fe740;
                                            } else {
                                                c.cycles += 2;
                                                if c.g[A2][0] == c.g[V0][0] {
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x1c0)
                                                        );
                                                    break 'b_1fe754;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x50),
                                                    );
                                                    if t {
                                                        break 'b_1fe8d4;
                                                    } else {
                                                        break 'b_1fe740;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x7;
                                        if c.g[A2][0] == c.g[V0][0] {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                                            break 'b_1fe830;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_1fe8d4;
                                            } else {
                                                break 'b_1fe754;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == c.g[S1][0];
                                    c.g[S3][0] = 0x1;
                                    if t {
                                        break 'b_1fe8d0;
                                    } else {
                                        c.cycles += 7;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        Callee {
                                            addr: 0x00227f80,
                                            f: ee::s05::MCManager_Get,
                                        }
                                        .call(c, 0x001fe764)?;
                                        c.g[S1][0] = c.g[V0][0];
                                        c.g[S4][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x214));
                                        c.g[V0][0] = 0x3;
                                        if c.g[S4][0] != c.g[V0][0] {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            break 'b_1fe8d4;
                                        } else {
                                            'b_1fe7ac: {
                                                c.cycles += 7;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x1c4),
                                                    c.g[V0][0],
                                                );
                                                c.sw(
                                                    (c.g[S1][0] as u32).wrapping_add(0xf8),
                                                    c.g[V0][0],
                                                );
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x1a8),
                                                    c.g[S3][0],
                                                );
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x1c8));
                                                let t = c.g[V0][0] != c.g[S3][0];
                                                c.g[A0][0] = c.g[S1][0];
                                                if t {
                                                    break 'b_1fe7ac;
                                                } else {
                                                    c.cycles += 6;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                                    c.g[A0][0] = c
                                                        .lw((c.g[V1][0] as u32).wrapping_add(0x28));
                                                    call_ptr(c, 0x00270ed8, 0x001fe7a4)?;
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x1c8),
                                                        0u64,
                                                    );
                                                    c.g[A0][0] = c.g[S1][0];
                                                    break 'b_1fe7ac;
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[A1][0] = 0u64;
                                            c.g[A2][0] = 0xffffffffffffffff;
                                            call_ptr(c, 0x0023cae8, 0x001fe7b8)?;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x1c0),
                                                c.g[S4][0],
                                            );
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x1a8),
                                                c.g[S3][0],
                                            );
                                            let t = true;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x1dc),
                                                c.g[S3][0],
                                            );
                                            if t {
                                                break 'b_1fe8d0;
                                            } else {
                                                break 'b_1fe7c8;
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 7;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            Callee {
                                addr: 0x00227f80,
                                f: ee::s05::MCManager_Get,
                            }
                            .call(c, 0x001fe7d0)?;
                            c.g[A2][0] = c.g[V0][0];
                            c.g[A0][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                            if c.g[V0][0] == c.g[A0][0] {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                break 'b_1fe8d4;
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                                if c.g[V1][0] == c.g[S1][0] {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    break 'b_1fe8d4;
                                } else {
                                    'b_1fe828: {
                                        'b_1fe810: {
                                            'b_1fe80c: {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] == 0u64;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x1e4),
                                                    c.g[A0][0],
                                                );
                                                if t {
                                                    break 'b_1fe80c;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = 0x2;
                                                    if c.g[V1][0] == c.g[V0][0] {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x748));
                                                        break 'b_1fe810;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] != c.g[A0][0];
                                                        c.g[V0][0] = 0x6;
                                                        if t {
                                                            break 'b_1fe828;
                                                        } else {
                                                            break 'b_1fe80c;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x748));
                                            break 'b_1fe810;
                                        }
                                        c.cycles += 6;
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001fe824)?;
                                        c.g[V0][0] = 0x6;
                                        break 'b_1fe828;
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                                    if t {
                                        break 'b_1fe8d0;
                                    } else {
                                        break 'b_1fe830;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V0][0] == c.g[S1][0];
                        c.g[S3][0] = 0x1;
                        if t {
                            break 'b_1fe8d0;
                        } else {
                            c.cycles += 12;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            Callee {
                                addr: 0x00227f80,
                                f: ee::s05::MCManager_Get,
                            }
                            .call(c, 0x001fe840)?;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001fe860)?;
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                break 'b_1fe8d4;
                            } else {
                                c.cycles += 6;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c4), c.g[V0][0]);
                                c.sw((c.g[S1][0] as u32).wrapping_add(0xf8), c.g[V0][0]);
                                c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                                if c.g[S2][0] != c.g[S3][0] {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    break 'b_1fe8d4;
                                } else {
                                    c.cycles += 9;
                                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c4));
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x218));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x21c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001fe89c)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x4;
                                    if t {
                                        break 'b_1fe8d0;
                                    } else {
                                        c.cycles += 11;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1a8), c.g[S2][0]);
                                        c.g[A0][0] = c.g[S1][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                                        c.g[A1][0] = 0u64;
                                        call_ptr(c, 0x0023d570, 0x001fe8b8)?;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x748));
                                        c.g[A1][0] = 0x18;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001fe8d0)?;
                                        break 'b_1fe8d0;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_1fe8d4;
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, check_lohi, elf_image, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // The callees are replaced by gated fakes (live only on a machine whose
    // RAM holds `OC986_MAGIC` at `OC986_CTRL`). A fake logs its arguments
    // (RAM compare), returns the next scripted value for its target and may
    // unwind at a chosen call index.
    const OC986_CTRL: u32 = DATA + 0xa_0000;
    const OC986_MAGIC: u32 = 0x0c98_6a11;
    const OC986_COUNT: u32 = OC986_CTRL + 4;
    const OC986_UNWIND: u32 = OC986_CTRL + 8;
    const OC986_PER: u32 = OC986_CTRL + 0x100;
    const OC986_RES: u32 = OC986_CTRL + 0x200;
    const OC986_LOG: u32 = OC986_CTRL + 0x2000;
    /// Shared vtable-ish record of every fixture widget.
    const OC986_VT: u32 = DATA + 0x3000;

    fn oc986_fake_body(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(OC986_COUNT);
        c.bus.write32(OC986_COUNT, n.wrapping_add(1));
        let rec = OC986_LOG + 0x40 * (n & 63);
        let regs = [A0, A1, A2, A3, T0, V1, S0, S1, S2, RA];
        c.bus.write32(rec, target);
        for (i, r) in regs.into_iter().enumerate() {
            let v = c.g[r][0] as u32;
            c.bus.write32(rec + 4 + 4 * i as u32, v);
        }
        let k = c.bus.read32(OC986_PER + 4 * target);
        c.bus.write32(OC986_PER + 4 * target, k.wrapping_add(1));
        let res = c.bus.read32(OC986_RES + 0x80 * target + 4 * (k & 31));
        c.g[V0][0] = sx(res);
        c.cycles += 5;
        let ra = c.g[RA][0] as u32;
        c.pc = if n == c.bus.read32(OC986_UNWIND) {
            0xdead_0000 | n << 4
        } else {
            ra
        };
    }

    macro_rules! oc986_fakes {
        ($(($fake:ident, $prev:ident, $idx:expr, $slot:expr, $generated:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC986_CTRL) != OC986_MAGIC {
                        return fakes::fall_back(c, &$prev, $generated);
                    }
                    oc986_fake_body(c, $idx);
                }
            )*
            pub(crate) fn oc986_install() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    pub(crate) const OC986_T_145870: u32 = 0;
    pub(crate) const OC986_T_146E98: u32 = 1;
    pub(crate) const OC986_T_147398: u32 = 2;
    pub(crate) const OC986_T_14DC80: u32 = 3;
    pub(crate) const OC986_T_14DD58: u32 = 4;
    pub(crate) const OC986_T_157B08: u32 = 5;
    pub(crate) const OC986_T_158910: u32 = 6;
    pub(crate) const OC986_T_317670: u32 = 7;
    pub(crate) const OC986_T_397CC0: u32 = 8;
    pub(crate) const OC986_T_39C870: u32 = 9;
    pub(crate) const OC986_T_39D860: u32 = 10;
    pub(crate) const OC986_T_3A0C50: u32 = 11;
    pub(crate) const OC986_T_3A1000: u32 = 12;
    pub(crate) const OC986_T_20A380: u32 = 13;
    pub(crate) const OC986_T_2583A8: u32 = 14;
    pub(crate) const OC986_T_23CAE8: u32 = 15;
    pub(crate) const OC986_T_23D570: u32 = 16;
    pub(crate) const OC986_T_270ED8: u32 = 17;
    pub(crate) const OC986_T_417828: u32 = 18;
    pub(crate) const OC986_T_227F80: u32 = 19;
    pub(crate) const OC986_T_1FB378: u32 = 20;
    pub(crate) const OC986_T_1FAFF8: u32 = 21;

    oc986_fakes! {
        (oc986_fake_145870, OC986_PREV_145870, OC986_T_145870, ee::s01::OVERRIDE_00145870, ee::s01::f_00145870__generated),
        (oc986_fake_146e98, OC986_PREV_146E98, OC986_T_146E98, ee::s01::OVERRIDE_00146E98, ee::s01::f_00146e98__generated),
        (oc986_fake_147398, OC986_PREV_147398, OC986_T_147398, ee::s01::OVERRIDE_00147398, ee::s01::f_00147398__generated),
        (oc986_fake_14dc80, OC986_PREV_14DC80, OC986_T_14DC80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated),
        (oc986_fake_14dd58, OC986_PREV_14DD58, OC986_T_14DD58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated),
        (oc986_fake_157b08, OC986_PREV_157B08, OC986_T_157B08, ee::s01::OVERRIDE_00157B08, ee::s01::f_00157b08__generated),
        (oc986_fake_158910, OC986_PREV_158910, OC986_T_158910, ee::s01::OVERRIDE_00158910, ee::s01::f_00158910__generated),
        (oc986_fake_317670, OC986_PREV_317670, OC986_T_317670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated),
        (oc986_fake_397cc0, OC986_PREV_397CC0, OC986_T_397CC0, ee::s13::OVERRIDE_00397CC0, ee::s13::f_00397cc0__generated),
        (oc986_fake_39c870, OC986_PREV_39C870, OC986_T_39C870, ee::s13::OVERRIDE_0039C870, ee::s13::f_0039c870__generated),
        (oc986_fake_39d860, OC986_PREV_39D860, OC986_T_39D860, ee::s13::OVERRIDE_0039D860, ee::s13::f_0039d860__generated),
        (oc986_fake_3a0c50, OC986_PREV_3A0C50, OC986_T_3A0C50, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated),
        (oc986_fake_3a1000, OC986_PREV_3A1000, OC986_T_3A1000, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated),
        (oc986_fake_20a380, OC986_PREV_20A380, OC986_T_20A380, ee::s05::OVERRIDE_0020A380, ee::s05::f_0020a380__generated),
        (oc986_fake_2583a8, OC986_PREV_2583A8, OC986_T_2583A8, ee::s06::OVERRIDE_002583A8, ee::s06::f_002583a8__generated),
        (oc986_fake_23cae8, OC986_PREV_23CAE8, OC986_T_23CAE8, ee::s06::OVERRIDE_0023CAE8, ee::s06::f_0023cae8__generated),
        (oc986_fake_23d570, OC986_PREV_23D570, OC986_T_23D570, ee::s06::OVERRIDE_0023D570, ee::s06::f_0023d570__generated),
        (oc986_fake_270ed8, OC986_PREV_270ED8, OC986_T_270ED8, ee::s07::OVERRIDE_00270ED8, ee::s07::f_00270ed8__generated),
        (oc986_fake_417828, OC986_PREV_417828, OC986_T_417828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated),
        (oc986_fake_227f80, OC986_PREV_227F80, OC986_T_227F80, ee::s05::OVERRIDE_00227F80, ee::s05::f_00227f80__generated),
        (oc986_fake_1fb378, OC986_PREV_1FB378, OC986_T_1FB378, ee::s04::OVERRIDE_001FB378, ee::s04::f_001fb378__generated),
        (oc986_fake_1faff8, OC986_PREV_1FAFF8, OC986_T_1FAFF8, ee::s04::OVERRIDE_001FAFF8, ee::s04::f_001faff8__generated),
    }

    /// Fill the 32 scripted results of one callee (repeating).
    fn oc986_results(c: &mut Cpu, target: u32, f: &mut dyn FnMut(u32) -> u32) {
        for k in 0..32 {
            let v = f(k);
            c.bus.write32(OC986_RES + 0x80 * target + 4 * k, v);
        }
    }

    /// Scrambled machine with the fakes armed; one call in four unwinds at
    /// a call index below `fake_calls` (the machine's own miss hook, for
    /// vtable calls, unwinds among the first `miss_calls`).
    fn oc986_machine(c: &mut Cpu, seed: u32, salt: u64, miss_calls: u32, fake_calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, miss_calls);
        elf_image(c);
        c.bus.write32(OC986_CTRL, OC986_MAGIC);
        let at = if seed % 4 == 1 {
            r.next_u32() % fake_calls
        } else {
            u32::MAX
        };
        c.bus.write32(OC986_UNWIND, at);
        c.bus.write16(OC986_VT + 0x48, ((seed % 3) * 0x10) as u16);
        c.bus.write32(OC986_VT + 0x4c, FAKE);
        r
    }

    /// A fixture widget: `+8` points at the shared record, flags random.
    fn oc986_widget(c: &mut Cpu, w: u32, r: &mut Rng) {
        c.bus.write32(w + 8, OC986_VT);
        c.bus.write32(w + 0x14, r.next_u32());
        c.bus.write32(w + 0x18, r.next_u32());
    }

    /// 0 or a widget, per call of `UIScreen_FindObject`: `zero_mod` is the
    /// period at which a result is null (0: never).
    fn oc986_find_results(c: &mut Cpu, r: &mut Rng, zero_mod: u32) {
        let mut ws = Vec::new();
        for k in 0..32u32 {
            let w = DATA + 0x1_0000 + 0x100 * k;
            oc986_widget(c, w, r);
            let null = zero_mod != 0 && r.next_u32() % zero_mod == 0;
            ws.push(if null { 0 } else { w });
        }
        oc986_results(c, OC986_T_39D860, &mut |k| ws[k as usize]);
    }

    /// `CountMedals`: reward medals missing for the current rider.
    #[test]
    fn oc986_count_medals_matches() {
        oc986_install();
        check_lohi(
            ee::s04::f_001f6f30__generated,
            count_medals,
            160,
            &|c, seed| {
                let mut r = oc986_machine(c, seed, 0x1f6f30, 0, 70);
                let this = DATA + 0x1000;
                for off in [0x58u32, 0x5c, 0x60, 0x68, 0x6c, 0x70] {
                    c.bus.write32(this + off, r.next_u32() % 64);
                }
                oc986_results(c, OC986_T_14DC80, &mut |k| DATA + 0x2000 + 0x10 * k);
                oc986_results(c, OC986_T_14DD58, &mut |k| DATA + 0x2400 + 0x10 * k);
                oc986_results(c, OC986_T_146E98, &mut |_| r.next_u32() % 5);
                oc986_results(c, OC986_T_147398, &mut |_| r.next_u32());
                oc986_results(c, OC986_T_145870, &mut |_| r.next_u32() % 4);
                // The three reward checks (bit k of the seed: medal missing).
                oc986_results(c, OC986_T_157B08, &mut |k| {
                    if (seed >> (k % 3)) & 1 == 0 {
                        0
                    } else {
                        1 + (k % 5)
                    }
                });
                // The medal lookup: 0 (missing), nonzero with the low byte
                // zero (still missing: the result is a char) or a real medal.
                oc986_results(c, OC986_T_158910, &mut |_| match r.next_u32() % 3 {
                    0 => 0,
                    1 => 0x100,
                    _ => 1 + r.next_u32() % 255,
                });
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `InGameError_Init`: the in-game error pop-up (cheat/lost-connection text).
    #[test]
    fn oc986_ingame_error_init_matches() {
        oc986_install();
        check(
            ee::s04::f_001fab18__generated,
            ingame_error_init,
            200,
            &|c, seed| {
                let mut r = oc986_machine(c, seed, 0x1fab18, 12, 70);
                let this = DATA + 0x1000;
                c.bus.write32(this + 0x10, DATA + 0x1800);
                let comm = DATA + 0x5000;
                let cheat0 = [0u32, 1, 2, 3, 7][(seed % 5) as usize];
                let cheat1 = [0u32, 1, 2, 3, 9][((seed / 5) % 5) as usize];
                c.bus.write32(comm + 0x24, cheat0);
                c.bus.write32(comm + 0x28, cheat1);
                c.bus.write32(comm + 0x8c, (seed / 25) & 1);
                c.bus.write32(comm + 0x14, (seed / 50) & 1);
                c.bus.write32(comm + 0xa0, (seed / 100) & 1);
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x204),
                    if seed % 17 == 0 { 0 } else { comm },
                );
                c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x5400);
                c.bus.write32(DATA + 0x5404, r.next_u32());
                c.bus.write32(GAME_GP.wrapping_add(0x2a74), DATA + 0x5600);
                c.bus.write32(DATA + 0x5618, r.next_u32());
                oc986_results(c, OC986_T_317670, &mut |_| r.next_u32() | 1);
                let screen = if seed % 13 == 0 { 0 } else { DATA + 0x2000 };
                oc986_results(c, OC986_T_397CC0, &mut |_| screen);
                oc986_find_results(c, &mut r, 6);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `MultiFreestylePre_TransitionIn`: layout shift by game mode.
    #[test]
    fn oc986_mfpre_transition_in_matches() {
        oc986_install();
        check(
            ee::s04::f_001fcec0__generated,
            mfpre_transition_in,
            80,
            &|c, seed| {
                let mut r = oc986_machine(c, seed, 0x1fcec0, 0, 10);
                let this = DATA + 0x1000;
                c.bus.write32(this + 0x40, DATA + 0x2000);
                c.bus.write8(0x0053_5c10, (seed % 8) as u8);
                oc986_results(c, OC986_T_14DC80, &mut |k| DATA + 0x2400 + 0x10 * k);
                oc986_results(c, OC986_T_14DD58, &mut |k| DATA + 0x2800 + 0x10 * k);
                oc986_results(c, OC986_T_317670, &mut |_| r.next_u32() | 1);
                let mut ws = Vec::new();
                for k in 0..32u32 {
                    let w = DATA + 0x1_0000 + 0x100 * k;
                    for off in [0x44u32, 0x48, 0x4c] {
                        let v = r.edge_f32();
                        c.bus.write32(w + off, v);
                    }
                    ws.push(w);
                }
                oc986_results(c, OC986_T_39D860, &mut |k| ws[k as usize]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 9 == 0 { 0 } else { sx(DATA + 0x1c00) };
            },
        );
    }

    /// `SaveReplay_Init`: the save-replay screen and its card-slot labels.
    #[test]
    fn oc986_savereplay_init_matches() {
        oc986_install();
        check(
            ee::s05::f_001fdc30__generated,
            savereplay_init,
            120,
            &|c, seed| {
                let mut r = oc986_machine(c, seed, 0x1fdc30, 12, 60);
                let this = DATA + 0x1000;
                c.bus.write32(this + 0x10, DATA + 0x1800);
                c.bus.write32(GAME_GP.wrapping_sub(0x69c), (seed * 7) % 20);
                oc986_results(c, OC986_T_317670, &mut |_| r.next_u32() | 1);
                // First screen null every fifth seed, the overlay every third.
                oc986_results(c, OC986_T_397CC0, &mut |k| {
                    let null = if k % 2 == 0 {
                        seed % 5 == 0
                    } else {
                        seed % 3 == 0
                    };
                    if null {
                        0
                    } else {
                        DATA + 0x2000 + 0x100 * k
                    }
                });
                oc986_find_results(c, &mut r, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `SaveReplay_OnWidgetAction`: confirm / stop / save events (5, 6, 7).
    #[test]
    fn oc986_savereplay_widget_action_matches() {
        oc986_install();
        check(
            ee::s05::f_001fe6e8__generated,
            savereplay_widget_action,
            220,
            &|c, seed| {
                let mut r = oc986_machine(c, seed, 0x1fe6e8, 6, 6);
                let this = DATA + 0x1000;
                let msg = DATA + 0x1c00;
                let world = DATA + 0x5000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, DATA + 0x5100);
                c.bus.write32(DATA + 0x5128, r.next_u32());
                let ty = seed % 9;
                c.bus.write32(this + 0x1a8, (r.next_u32() % 8 == 0) as u32);
                let state = match ty {
                    7 if r.next_u32() % 4 != 0 => 1,
                    _ => r.next_u32() % 8,
                };
                c.bus.write32(this + 0x1c0, state);
                c.bus.write32(this + 0x1c8, r.next_u32() % 3);
                c.bus
                    .write32(this + 0x214, if r.next_u32() % 4 != 0 { 3 } else { 5 });
                c.bus.write32(msg + 0x18, r.next_u32());
                // The memory-card manager and its virtual methods: the call
                // targets are `FAKE` (answers non-zero) or a real leaf that
                // returns 0 (0x1406c0).
                let mp = DATA + 0x6000;
                let card = DATA + 0x6400;
                let card_vt = DATA + 0x6800;
                let obj = DATA + 0x6c00;
                c.bus.write32(mp + 0xf8, r.next_u32());
                c.bus.write32(mp + 0x428, r.next_u32());
                c.bus.write32(mp + 0x434, card);
                c.bus.write32(mp + 0x748, obj);
                c.bus.write32(card, card_vt);
                c.bus.write16(card_vt + 0x1b0, 0x10);
                c.bus.write32(
                    card_vt + 0x1b4,
                    if r.next_u32() % 4 == 0 {
                        0x0014_06c0
                    } else {
                        FAKE
                    },
                );
                c.bus.write16(card_vt + 0x218, 0x20);
                c.bus.write32(
                    card_vt + 0x21c,
                    if r.next_u32() % 4 == 0 {
                        0x0014_06c0
                    } else {
                        FAKE
                    },
                );
                c.bus.write16(obj + 8, 0x30);
                c.bus.write32(obj + 0xc, FAKE);
                oc986_results(c, OC986_T_227F80, &mut |_| mp);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 11 == 0 { 0 } else { sx(msg) };
                c.g[A2][0] = ty as u64;
            },
        );
    }
}
