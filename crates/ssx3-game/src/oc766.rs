//! Staging module oc766 (wave wave244): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc766`.

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
    &[(0x001f_b6b8, ov_screen_race_pre_init, "OVScreenRacePre_Init")];

/// `void OVScreenRacePre_Init(OVStateRacePre* state)` (0x1fb6b8, HEAT pre-event
/// screen `onCreateScreen`; `a0` is kept in `s5`): creates and starts the two
/// UI screens (the first one is stored at `state+0x40`), runs
/// `OVScreenOverlayMenu_Init`, fetches the BE interfaces (new-player, char
/// display, race, score), writes the track name / event-name line, shows the
/// rider-count objects (0x46fbd8/e8/f8 by `*(player_cfg)`, hiding 0x4a2148 or
/// setting its text for the game mode at `0x535c11`), sets the caption by
/// event kind, fills the six rider rows (names, with the player name for the
/// human riders in mode 2), the best-time text, plays FE sound 0xe and the
/// race-intro announcer line.
///
/// The body keeps the generated block structure, registers and cycle
/// charges (register leftovers are visible to the fake callees' call log);
/// every call goes through the function table with its return address.
pub fn ov_screen_race_pre_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fb724: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2f0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x290));
            c.g[S5][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x2c0));
            c.g[A0][0] = 0x470000;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x2b0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x438));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x2a0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x280));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x270));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x260));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x250), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
            c.g[S0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001fb6f8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S5][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001fb70c)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S5][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1fb724;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001fb724)?;
                break 'b_1fb724;
            }
        }
        // Second screen (0x46e378 = the overlay-less status screen): create and start its thread.
        'b_1fb760: {
            c.cycles += 11;
            c.g[A0][0] = 0x470000;
            c.g[S0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c88));
            call_ptr(c, 0x00317670, 0x001fb734)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S5][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001fb748)?;
            let t = c.g[V0][0] == 0u64;

            if t {
                break 'b_1fb760;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001fb760)?;
                break 'b_1fb760;
            }
        }
        // OVScreenOverlayMenu_Init, the BE interfaces, the track-name text, the
        // objective-mode object visibility and caption.
        'b_1fb990: {
            'b_1fb930: {
                c.cycles += 89;
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x0020d1d8, 0x001fb768)?;
                call_ptr(c, 0x0014dc80, 0x001fb770)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x001fb77c)?;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x230), c.g[V0][0]);
                call_ptr(c, 0x0014dc80, 0x001fb784)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x0014dd58, 0x001fb790)?;
                c.g[FP][0] = c.g[V0][0];
                call_ptr(c, 0x0014dc80, 0x001fb798)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x001fb7a4)?;
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dc80, 0x001fb7ac)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x8;
                call_ptr(c, 0x0014dd58, 0x001fb7b8)?;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x234), c.g[V0][0]);
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
                c.g[S6][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                c.g[S4][0] = c.lw(c.g[S6][0] as u32);
                call_ptr(c, 0x00146e98, 0x001fb7d4)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
                c.g[A1][0] = 0x1;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x238), c.g[V0][0]);
                call_ptr(c, 0x00146e98, 0x001fb7e4)?;
                c.g[A0][0] = 0x470000;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x23c), c.g[V0][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c78));
                call_ptr(c, 0x00317670, 0x001fb7f4)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fb800)?;
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S3][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00144bf0, 0x001fb810)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x002c2540, 0x001fb81c)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1ab8));
                c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x00317670, 0x001fb83c)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, t, 0x001fb84c)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A1][0] = 0x4a0000;
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2140));
                call_ptr(c, 0x002c27c0, 0x001fb868)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0e90, 0x001fb874)?;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.g[S5][0];
                c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x428));
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x0039e8b8, 0x001fb888)?;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.g[S5][0];
                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x418));
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x0039e8b8, 0x001fb89c)?;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.g[S5][0];
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x408));
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0039e8b8, 0x001fb8b0)?;
                c.g[V1][0] = 0x530000;
                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x49));
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = 0x4a0000;
                if t {
                    break 'b_1fb930;
                } else {
                    'b_1fb920: {
                        'b_1fb91c: {
                            'b_1fb918: {
                                'b_1fb90c: {
                                    'b_1fb8fc: {
                                        'b_1fb8f8: {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x2;
                                            let t = c.g[S4][0] == c.g[V0][0];
                                            c.g[V0][0] = ((c.g[S4][0] as i64) < 3i64) as u64;
                                            if t {
                                                break 'b_1fb8f8;
                                            } else {
                                                'b_1fb8e8: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[V0][0] = 0x3;
                                                    if t {
                                                        break 'b_1fb8e8;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[S4][0] == c.g[V0][0];
                                                        c.g[A1][0] = 0x4a0000;
                                                        if t {
                                                            break 'b_1fb90c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A0][0] = c.g[S5][0];
                                                            if t {
                                                                break 'b_1fb920;
                                                            } else {
                                                                break 'b_1fb8e8;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = (c.g[S4][0] as i64) < 0;
                                                c.g[A1][0] = c.g[S3][0];
                                                if t {
                                                    break 'b_1fb918;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;

                                                    if t {
                                                        break 'b_1fb8fc;
                                                    } else {
                                                        break 'b_1fb8f8;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[A1][0] = c.g[S2][0];
                                        break 'b_1fb8fc;
                                    }
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S5][0];
                                    call_ptr(c, 0x0039e948, 0x001fb904)?;
                                    let t = true;
                                    c.g[A1][0] = 0x4a0000;
                                    if t {
                                        break 'b_1fb91c;
                                    } else {
                                        break 'b_1fb90c;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A0][0] = c.g[S5][0];
                                call_ptr(c, 0x0039e948, 0x001fb918)?;
                                break 'b_1fb918;
                            }
                            c.cycles += 1;
                            c.g[A1][0] = 0x4a0000;
                            break 'b_1fb91c;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S5][0];
                        break 'b_1fb920;
                    }
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2148));
                    call_ptr(c, 0x0039e8b8, 0x001fb928)?;
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1fb990;
                    } else {
                        break 'b_1fb930;
                    }
                }
            }
            'b_1fb978: {
                c.cycles += 10;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2148));
                call_ptr(c, 0x00317670, 0x001fb938)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fb944)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x49));
                c.g[V0][0] = 0x2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A0][0] = 0x470000;
                    break 'b_1fb978;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3f8));
                    call_ptr(c, 0x00317670, 0x001fb964)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001fb970)?;
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1fb990;
                    } else {
                        break 'b_1fb978;
                    }
                }
            }
            c.cycles += 6;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3e0));
            call_ptr(c, 0x00317670, 0x001fb980)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a1000, 0x001fb98c)?;
            c.g[A0][0] = 0x470000;
            break 'b_1fb990;
        }
        // Caption text id by event kind (s4 = 1 or 2: special strings).
        'b_1fba04: {
            'b_1fb9ec: {
                'b_1fb9d0: {
                    'b_1fb9c0: {
                        c.cycles += 9;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3c8));
                        call_ptr(c, 0x00317670, 0x001fb998)?;
                        c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fb9a4)?;
                        c.g[S3][0] = c.g[V0][0];
                        c.g[V0][0] = 0x1;
                        let t = c.g[S4][0] != c.g[V0][0];
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_1fb9c0;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = 0x470000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3b8));
                            if t {
                                break 'b_1fb9d0;
                            } else {
                                break 'b_1fb9c0;
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x470000;
                        break 'b_1fb9ec;
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3a0));
                        break 'b_1fb9d0;
                    }
                }
                c.cycles += 7;
                call_ptr(c, 0x00317670, 0x001fb9d8)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x001fb9e4)?;
                let t = true;
                c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x40));
                if t {
                    break 'b_1fba04;
                } else {
                    break 'b_1fb9ec;
                }
            }
            c.cycles += 6;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x388));
            call_ptr(c, 0x00317670, 0x001fb9f4)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a1000, 0x001fba00)?;
            c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x40));
            break 'b_1fba04;
        }
        c.cycles += 13;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x18));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x248), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x244), c.g[S6][0]);
        c.g[S2][0] = 0u64;
        c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
        c.g[S7][0] = 0x4a0000;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x240), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x248));
        // Six rider rows (s2 = row): character display names, with the player name
        // for the human riders in multi-player mode (+0x49 of 0x535bc8 == 2).
        'l_1fba38: loop {
            'b_1fbb3c: {
                'b_1fbb24: {
                    c.cycles += 22;
                    c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S6][0] = c.g[A2][0];
                    c.g[S1][0] = c.lw(c.g[V1][0] as u32);
                    c.g[A0][0] = c.g[SP][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x244));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2170));
                    call_ptr(c, 0x00417828, 0x001fba68)?;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001fba70)?;
                    c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fba7c)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x240));
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        break 'b_1fbb24;
                    } else {
                        'b_1fbb04: {
                            c.cycles += 6;
                            c.g[A0][0] = 0x530000;
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                            c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                            c.g[A0][0] = 0x2;
                            if c.g[V1][0] != c.g[A0][0] {
                                c.g[A1][0] = c.g[S0][0];
                                break 'b_1fbb04;
                            } else {
                                'b_1fbabc: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x238));
                                    let t = c.g[S2][0] == c.g[V0][0];
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x23c));
                                    if t {
                                        break 'b_1fbabc;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S2][0] != c.g[V1][0];
                                        c.g[A1][0] = c.g[S0][0];
                                        if t {
                                            break 'b_1fbb04;
                                        } else {
                                            break 'b_1fbabc;
                                        }
                                    }
                                }
                                c.cycles += 18;
                                c.g[A2][0] = c.g[S1][0];
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A0][0] = c.g[FP][0];
                                call_ptr(c, 0x0014eec8, 0x001fbacc)?;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A1][0] = c.g[S2][0];
                                call_ptr(c, 0x00146f88, 0x001fbadc)?;
                                c.g[A0][0] = c.g[S4][0];
                                c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x24f0));
                                c.g[A2][0] = c.g[S0][0];
                                c.g[A3][0] = c.g[V0][0];
                                call_ptr(c, 0x00417828, 0x001fbaf0)?;
                                c.g[A0][0] = c.g[S3][0];
                                c.g[A1][0] = c.g[S4][0];
                                call_ptr(c, 0x003a0c50, 0x001fbafc)?;
                                let t = true;
                                c.g[S2][0] = c.g[S6][0];
                                if t {
                                    break 'b_1fbb3c;
                                } else {
                                    break 'b_1fbb04;
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[A0][0] = c.g[FP][0];
                        c.g[A2][0] = c.g[S1][0];
                        call_ptr(c, 0x0014eec8, 0x001fbb10)?;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a0c50, 0x001fbb1c)?;
                        let t = true;
                        c.g[S2][0] = c.g[S6][0];
                        if t {
                            break 'b_1fbb3c;
                        } else {
                            break 'b_1fbb24;
                        }
                    }
                }
                c.cycles += 6;
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001fbb38)?;
                c.g[S2][0] = c.g[S6][0];
                break 'b_1fbb3c;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 6i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x248));
                if event_due(c) {
                    deliver_event_here(c, 0x001fba38);
                }
                continue 'l_1fba38;
            } else {
                // High score text, FE sound 0xe and the race-intro announcer line.
                c.cycles += 36;
                c.g[A0][0] = 0x4a0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24f8));
                call_ptr(c, 0x00317670, 0x001fbb54)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fbb60)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x234));
                c.g[S3][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00155130, 0x001fbb70)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0020a1a8, 0x001fbb78)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a0c50, 0x001fbb84)?;
                call_ptr(c, 0x0028b180, 0x001fbb8c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0xe;
                call_ptr(c, 0x00294f78, 0x001fbb98)?;
                call_ptr(c, 0x0028b180, 0x001fbba0)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x002a31c0, 0x001fbba8)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x2b0));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x2a0));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x290));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x280));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x270));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x260));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x250));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2f0));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{fill, DATA};
    use std::sync::atomic::AtomicUsize;

    // Fixture objects, all in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    const OC766_BASE: u32 = DATA + 0x20_0000;
    const OC766_CTRL: u32 = OC766_BASE;
    const OC766_MAGIC: u32 = 0x4f43_3736;
    /// Results of the successive `UIEngine_CreateScreen` calls (0 = null).
    const OC766_CREATE: u32 = OC766_BASE + 0x100;
    /// `BENewPlayerInt_GetHumanRider(0)` and `(1)`.
    const OC766_RIDERS: u32 = OC766_BASE + 0x140;
    const OC766_STATE: u32 = OC766_BASE + 0x2000;
    const OC766_WORLD: u32 = OC766_BASE + 0x3000;
    const OC766_CFG: u32 = OC766_BASE + 0x3400;
    const OC766_W84: u32 = OC766_BASE + 0x3800;
    const OC766_W84P: u32 = OC766_BASE + 0x3900;
    const OC766_OVOBJ: u32 = OC766_BASE + 0x3a00;
    const OC766_IFACE: u32 = OC766_BASE + 0x3c00;
    const OC766_IFVT: u32 = OC766_BASE + 0x3d00;
    /// The game-mode byte (`0x535c11`).
    const OC766_MODE: u32 = 0x0053_5c11;

    /// The mailbox word of the fake for `addr`.
    fn oc766_mail(addr: u32) -> u32 {
        OC766_BASE + 0x10_0000 + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3).
    fn oc766_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc766_mail(addr), v.wrapping_add(3));
    }

    /// Make the fake for `addr` return 0.
    fn oc766_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc766_mail(addr), 1);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns
    /// the forced result of its mailbox (0 = the fake's pointer), the next
    /// word of the `CreateScreen` list or the human rider asked for.
    fn oc766_fake_call(c: &mut Cpu, addr: u32) {
        let a1 = c.g[A1][0] as u32;
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc766_mail(addr));
        let create = if addr == 0x0039_7cc0 {
            let k = c.bus.read32(OC766_CREATE + 0x20);
            c.bus.write32(OC766_CREATE + 0x20, k + 1);
            Some(c.bus.read32(OC766_CREATE + 4 * (k & 3)))
        } else {
            None
        };
        let rider = if addr == 0x0014_6e98 {
            Some(c.bus.read32(OC766_RIDERS + 4 * (a1 & 1)))
        } else {
            None
        };
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        if let Some(v) = rider {
            c.g[V0][0] = sx(v);
        } else if let Some(v) = create {
            if v == 0 {
                c.g[V0][0] = 0;
            }
        } else if mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc766_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC766_CTRL) != OC766_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc766_fake_call(c, $addr);
                }
            )*
            fn oc766_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc766_fakes!(
        (
            oc766_fake_317670,
            OC766_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc766_fake_397cc0,
            OC766_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc766_fake_39c870,
            OC766_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc766_fake_14dc80,
            OC766_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc766_fake_14dd58,
            OC766_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc766_fake_146e98,
            OC766_PREV_146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x0014_6e98
        ),
        (
            oc766_fake_39d860,
            OC766_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc766_fake_144bf0,
            OC766_PREV_144BF0,
            ee::s01::OVERRIDE_00144BF0,
            ee::s01::f_00144bf0__generated,
            0x0014_4bf0
        ),
        (
            oc766_fake_2c2540,
            OC766_PREV_2C2540,
            ee::s08::OVERRIDE_002C2540,
            ee::s08::f_002c2540__generated,
            0x002c_2540
        ),
        (
            oc766_fake_2c27c0,
            OC766_PREV_2C27C0,
            ee::s08::OVERRIDE_002C27C0,
            ee::s08::f_002c27c0__generated,
            0x002c_27c0
        ),
        (
            oc766_fake_3a0e90,
            OC766_PREV_3A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a_0e90
        ),
        (
            oc766_fake_39e8b8,
            OC766_PREV_39E8B8,
            ee::s13::OVERRIDE_0039E8B8,
            ee::s13::f_0039e8b8__generated,
            0x0039_e8b8
        ),
        (
            oc766_fake_39e948,
            OC766_PREV_39E948,
            ee::s13::OVERRIDE_0039E948,
            ee::s13::f_0039e948__generated,
            0x0039_e948
        ),
        (
            oc766_fake_3a1000,
            OC766_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc766_fake_417828,
            OC766_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc766_fake_14eec8,
            OC766_PREV_14EEC8,
            ee::s01::OVERRIDE_0014EEC8,
            ee::s01::f_0014eec8__generated,
            0x0014_eec8
        ),
        (
            oc766_fake_146f88,
            OC766_PREV_146F88,
            ee::s01::OVERRIDE_00146F88,
            ee::s01::f_00146f88__generated,
            0x0014_6f88
        ),
        (
            oc766_fake_155130,
            OC766_PREV_155130,
            ee::s01::OVERRIDE_00155130,
            ee::s01::f_00155130__generated,
            0x0015_5130
        ),
        (
            oc766_fake_28b180,
            OC766_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc766_fake_3a0c70,
            OC766_PREV_3A0C70,
            ee::s13::OVERRIDE_003A0C70,
            ee::s13::f_003a0c70__generated,
            0x003a_0c70
        ),
        (
            oc766_fake_2ea820,
            OC766_PREV_2EA820,
            ee::s09::OVERRIDE_002EA820,
            ee::s09::f_002ea820__generated,
            0x002e_a820
        ),
        (
            oc766_fake_2ea860,
            OC766_PREV_2EA860,
            ee::s09::OVERRIDE_002EA860,
            ee::s09::f_002ea860__generated,
            0x002e_a860
        ),
        (
            oc766_fake_39f9d8,
            OC766_PREV_39F9D8,
            ee::s13::OVERRIDE_0039F9D8,
            ee::s13::f_0039f9d8__generated,
            0x0039_f9d8
        ),
        (
            oc766_fake_413af8,
            OC766_PREV_413AF8,
            ee::s15::OVERRIDE_00413AF8,
            ee::s15::f_00413af8__generated,
            0x0041_3af8
        ),
        (
            oc766_fake_287920,
            OC766_PREV_287920,
            ee::s07::OVERRIDE_00287920,
            ee::s07::f_00287920__generated,
            0x0028_7920
        ),
        (
            oc766_fake_29f128,
            OC766_PREV_29F128,
            ee::s08::OVERRIDE_0029F128,
            ee::s08::f_0029f128__generated,
            0x0029_f128
        ),
        (
            oc766_fake_2a4040,
            OC766_PREV_2A4040,
            ee::s08::OVERRIDE_002A4040,
            ee::s08::f_002a4040__generated,
            0x002a_4040
        ),
        (
            oc766_fake_295028,
            OC766_PREV_295028,
            ee::s08::OVERRIDE_00295028,
            ee::s08::f_00295028__generated,
            0x0029_5028
        ),
        (
            oc766_fake_2b1458,
            OC766_PREV_2B1458,
            ee::s08::OVERRIDE_002B1458,
            ee::s08::f_002b1458__generated,
            0x002b_1458
        ),
        (
            oc766_fake_3d8008,
            OC766_PREV_3D8008,
            ee::s14::OVERRIDE_003D8008,
            ee::s14::f_003d8008__generated,
            0x003d_8008
        ),
    );

    /// `OVScreenRacePre_Init`: the two screens created (or null), the heat /
    /// event kind (-1 to 5) picking the shown rider-count object and the caption,
    /// the game mode byte (0, 1, 2: the +0x4a2148 text), 0 to 7 riders in the six
    /// rows (human riders get "name (player)" in mode 2), and the announcer
    /// line's gates; the interface vtable call, the real `OVScreenOverlayMenu_Init`
    /// (`cOVStateOverlay_Init`, 5 rows), `UIText_SetString`, `FE_FormatMinSec`,
    /// `PlayFESound` and `PASayRaceIntro` run on faked callees.
    #[test]
    fn ov_screen_race_pre_init_matches() {
        oc766_install_fakes();
        covered(
            ee::s04::f_001fb6b8__generated,
            ov_screen_race_pre_init,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1fb6b8, 40);
                c.pc = 0x001f_b6b8;
                c.bus.write32(OC766_CTRL, OC766_MAGIC);
                c.g[A0][0] = sx(OC766_STATE);
                c.bus.write32(OC766_STATE + 0x10, r.next_u32());
                // The world.
                c.bus.write32(crate::difftest::GAME_GP - 0x848, OC766_WORLD);
                c.bus.write32(OC766_WORLD + 0xc0, OC766_CFG);
                c.bus.write32(OC766_WORLD + 0x84, OC766_W84);
                c.bus.write32(OC766_WORLD + 0x8c, OC766_IFACE);
                // Heat / event kind and the rider rows.
                let kind = [-1i32, 0, 1, 2, 3, 4][(seed % 6) as usize];
                c.bus.write32(OC766_CFG, kind as u32);
                fill(c, &mut r, OC766_CFG + 4, 0x20);
                // Game mode byte.
                c.bus.write8(OC766_MODE, ((seed / 6) % 3) as u8);
                // Rider count.
                c.bus.write32(OC766_W84 + 0xc, OC766_W84P);
                c.bus.write32(OC766_W84P + 0x78, (seed / 18) % 8);
                // cOVStateOverlay_Init: the cut-scene transition state.
                c.bus.write32(OC766_W84 + 0x64, OC766_OVOBJ);
                c.bus
                    .write32(OC766_OVOBJ + 0x10, [0, 1, 3, 2][((seed / 5) % 4) as usize]);
                // Interface object: vtable slot (+0x20 delta, +0x24 target).
                c.bus.write32(OC766_IFACE + 4, OC766_IFVT);
                c.bus
                    .write16(OC766_IFVT + 0x20, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(OC766_IFVT + 0x24, fakes::FAKE + 0x40);
                // The two screens: null or not.
                for k in 0..4 {
                    let v = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        0x8000_0000 + 0x40 * k
                    };
                    c.bus.write32(OC766_CREATE + 4 * k, v);
                }
                // Human riders.
                for k in 0..2 {
                    let v = if r.next_u32() % 5 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 7
                    };
                    c.bus.write32(OC766_RIDERS + 4 * k, v);
                }
                // Announcer gates: speech on, not event mode 4.
                if (seed / 3) % 3 == 0 {
                    oc766_force_zero(c, 0x0029_f128);
                } else {
                    oc766_force(c, 0x0029_f128, 1);
                }
                if (seed / 9) % 3 == 0 {
                    oc766_force(c, 0x002a_4040, 1);
                } else {
                    oc766_force_zero(c, 0x002a_4040);
                }
                if (seed / 27) % 2 == 0 {
                    oc766_force_zero(c, 0x0029_5028);
                }
                if (seed / 54) % 2 == 0 {
                    oc766_force_zero(c, 0x002b_1458);
                }
            },
        );
    }
}
