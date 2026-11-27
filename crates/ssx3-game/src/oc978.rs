//! Staging module oc978 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc978`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001f_c878,
    ov_screen_multi_freestyle_pre_init,
    "OVScreenMultiFreestylePre_Init",
)];

/// `void OVScreenMultiFreestylePre_Init(OVStateMultiFreestylePre* self)`
/// (0x1fc878, ovstatepause.cpp): builds the multiplayer freestyle pre-race
/// screen. Adds the "50multifs_pre" and "OV_darkblue" UI screens (play frame 0
/// each), then fills the labels from the new-race settings (`0x535bc8`: byte
/// +0x48 game mode, +0x49 game path, +0x4a event kind): the location and mode
/// caption, the two rider names (multiplayer path: the player-slot names), the
/// objective texts per mode and the record time or score line, showing or hiding
/// the mode-specific objects. Every call goes through the function table in the
/// original order, so the body follows the generated block structure.
pub fn ov_screen_multi_freestyle_pre_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fc8e4: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x490));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x430));
            c.g[S5][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x470));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x460));
            c.g[A0][0] = 0x470000;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x450));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1f8));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x440));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x420));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x410));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x400));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x3f0), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x480));
            c.g[S0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x0031_7670, 0x001f_c8b8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S5][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x0039_7cc0, 0x001f_c8cc)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S5][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1fc8e4;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039_c870, 0x001f_c8e4)?;
                break 'b_1fc8e4;
            }
        }
        'b_1fc924: {
            c.cycles += 11;
            c.g[A0][0] = 0x470000;
            c.g[S0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c88));
            call_ptr(c, 0x0031_7670, 0x001f_c8f4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S5][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x0039_7cc0, 0x001f_c908)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = c.g[S5][0];
                break 'b_1fc924;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039_c870, 0x001f_c920)?;
                c.g[A0][0] = c.g[S5][0];
                break 'b_1fc924;
            }
        }
        'b_1fca98: {
            'b_1fca94: {
                'b_1fca68: {
                    'b_1fca5c: {
                        'b_1fca4c: {
                            c.cycles += 54;
                            c.g[S0][0] = 0x530000;
                            call_ptr(c, 0x0020_a380, 0x001f_c92c)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5bc8));
                            call_ptr(c, 0x0014_dc80, 0x001f_c934)?;
                            c.g[S1][0] = 0x4a0000;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0x1;
                            call_ptr(c, 0x0014_dd58, 0x001f_c944)?;
                            c.g[S2][0] = 0x470000;
                            c.g[S7][0] = c.g[V0][0];
                            call_ptr(c, 0x0014_dc80, 0x001f_c950)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x0014_dd58, 0x001f_c95c)?;
                            c.g[S3][0] = c.g[V0][0];
                            call_ptr(c, 0x0014_dc80, 0x001f_c964)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0x2;
                            call_ptr(c, 0x0014_dd58, 0x001f_c970)?;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x3e0), c.g[V0][0]);
                            call_ptr(c, 0x0014_dc80, 0x001f_c978)?;
                            c.g[A1][0] = 0x8;
                            c.g[A0][0] = c.g[V0][0];
                            call_ptr(c, 0x0014_dd58, 0x001f_c984)?;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x3e4), c.g[V0][0]);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x21b0));
                            c.g[FP][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4a));
                            c.g[S6][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x48));
                            call_ptr(c, 0x0031_7670, 0x001f_c998)?;
                            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039_d860, 0x001f_c9a4)?;
                            c.g[S4][0] = c.g[V0][0];
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x3f8));
                            call_ptr(c, 0x0031_7670, 0x001f_c9b0)?;
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a_1000, 0x001f_c9bc)?;
                            c.g[A0][0] = 0x4a0000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x21b8));
                            call_ptr(c, 0x0031_7670, 0x001f_c9c8)?;
                            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039_d860, 0x001f_c9d4)?;
                            c.g[A0][0] = c.g[S3][0];
                            c.g[S4][0] = c.g[V0][0];
                            call_ptr(c, 0x0014_4bf0, 0x001f_c9e0)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[SP][0];
                            call_ptr(c, 0x002c_2540, 0x001f_c9ec)?;
                            c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x48));
                            c.g[V0][0] = 0x2;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[A3][0] = 0u64;
                            if t {
                                break 'b_1fca4c;
                            } else {
                                'b_1fca3c: {
                                    'b_1fca18: {
                                        c.cycles += 3;
                                        c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_1fca18;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x4a0000;
                                            if t {
                                                break 'b_1fca3c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V1][0] = 0x6;
                                                if t {
                                                    break 'b_1fca98;
                                                } else {
                                                    break 'b_1fca18;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x3;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x5;
                                    if t {
                                        break 'b_1fca5c;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[V0][0] = 0x4a0000;
                                        if t {
                                            break 'b_1fca94;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[A0][0] = 0x470000;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x1ab8));
                                            if t {
                                                break 'b_1fca68;
                                            } else {
                                                break 'b_1fca3c;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A0][0] = 0x470000;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c68));
                                if t {
                                    break 'b_1fca68;
                                } else {
                                    break 'b_1fca4c;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = 0x470000;
                        let t = true;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c58));
                        if t {
                            break 'b_1fca68;
                        } else {
                            break 'b_1fca5c;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c40));
                    break 'b_1fca68;
                }
                c.cycles += 11;
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                call_ptr(c, 0x0031_7670, 0x001f_ca7c)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, t, 0x001f_ca8c)?;
                c.g[A3][0] = c.g[V0][0];
                c.g[V0][0] = 0x4a0000;
                break 'b_1fca94;
            }
            c.cycles += 1;
            c.g[V1][0] = 0x6;
            break 'b_1fca98;
        }
        'b_1fcad4: {
            'b_1fcac0: {
                c.cycles += 2;
                let t = c.g[S6][0] != c.g[V1][0];
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2140));
                if t {
                    break 'b_1fcac0;
                } else {
                    c.cycles += 8;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[A1][0] = 0x470000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1aa8));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[SP][0];
                    call_ptr(c, 0x002c_27c0, 0x001f_cab8)?;
                    let t = true;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        break 'b_1fcad4;
                    } else {
                        break 'b_1fcac0;
                    }
                }
            }
            c.cycles += 5;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A2][0] = c.g[SP][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002c_27c0, 0x001f_cad0)?;
            c.g[A1][0] = c.g[S0][0];
            break 'b_1fcad4;
        }
        'b_1fcb50: {
            'b_1fcb2c: {
                c.cycles += 15;
                c.g[A0][0] = c.g[S4][0];
                call_ptr(c, 0x003a_0e90, 0x001f_cadc)?;
                c.g[A0][0] = 0x4a0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2520));
                call_ptr(c, 0x0031_7670, 0x001f_cae8)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001f_caf4)?;
                c.g[A0][0] = 0x530000;
                c.g[S4][0] = c.g[V0][0];
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                c.g[V0][0] = 0x2;
                c.g[A0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                let t = c.g[A0][0] != c.g[V0][0];
                c.g[A0][0] = c.g[S7][0];
                if t {
                    break 'b_1fcb2c;
                } else {
                    c.cycles += 7;
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014_6f88, 0x001f_cb18)?;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a_0c50, 0x001f_cb24)?;
                    let t = true;
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        break 'b_1fcb50;
                    } else {
                        break 'b_1fcb2c;
                    }
                }
            }
            c.cycles += 9;
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014_6e98, 0x001f_cb34)?;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3e0));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0014_ee58, 0x001f_cb40)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x001f_cb4c)?;
            c.g[A0][0] = 0x4a0000;
            break 'b_1fcb50;
        }
        'b_1fcc0c: {
            'b_1fcbe8: {
                c.cycles += 10;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2528));
                call_ptr(c, 0x0031_7670, 0x001f_cb58)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001f_cb64)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[S0][0] = 0x4a0000;
                if t {
                    break 'b_1fcbe8;
                } else {
                    'b_1fcbcc: {
                        'b_1fcbc8: {
                            c.cycles += 17;
                            c.g[A0][0] = c.g[S5][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2530));
                            c.g[A1][0] = c.g[S0][0];
                            call_ptr(c, 0x0039_e948, 0x001f_cb88)?;
                            c.g[A1][0] = 0x4a0000;
                            c.g[A0][0] = c.g[S5][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2538));
                            call_ptr(c, 0x0039_e948, 0x001f_cb98)?;
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0031_7670, 0x001f_cba0)?;
                            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039_d860, 0x001f_cbac)?;
                            c.g[S4][0] = c.g[V0][0];
                            c.g[V0][0] = 0x1;
                            if c.g[S6][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x470000;
                                break 'b_1fcbc8;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = 0x470000;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2c8));
                                if t {
                                    break 'b_1fcbcc;
                                } else {
                                    break 'b_1fcbc8;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a8));
                        break 'b_1fcbcc;
                    }
                    c.cycles += 7;
                    call_ptr(c, 0x0031_7670, 0x001f_cbd4)?;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a_1000, 0x001f_cbe0)?;
                    let t = true;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        break 'b_1fcc0c;
                    } else {
                        break 'b_1fcbe8;
                    }
                }
            }
            c.cycles += 9;
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2530));
            call_ptr(c, 0x0039_e8b8, 0x001f_cbf8)?;
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2538));
            call_ptr(c, 0x0039_e8b8, 0x001f_cc08)?;
            c.g[V0][0] = 0x4a0000;
            break 'b_1fcc0c;
        }
        'b_1fcca4: {
            'b_1fcc78: {
                c.cycles += 18;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x21b0));
                call_ptr(c, 0x0031_7670, 0x001f_cc14)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001f_cc20)?;
                c.g[V1][0] = 0x470000;
                c.g[S4][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3f8));
                call_ptr(c, 0x0031_7670, 0x001f_cc30)?;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1000, 0x001f_cc3c)?;
                c.g[A0][0] = 0x530000;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                c.g[A0][0] = 0x2;
                let t = c.g[V1][0] != c.g[A0][0];
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                if t {
                    break 'b_1fcc78;
                } else {
                    c.cycles += 9;
                    c.g[A0][0] = c.g[S7][0];
                    c.g[A1][0] = 0x1;
                    c.g[S2][0] = c.g[S0][0];
                    call_ptr(c, 0x0014_6f88, 0x001f_cc64)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0041_66f4, 0x001f_cc70)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_1fcca4;
                    } else {
                        break 'b_1fcc78;
                    }
                }
            }
            c.cycles += 11;
            c.g[A0][0] = c.g[S7][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014_6e98, 0x001f_cc84)?;
            c.g[S2][0] = c.g[S0][0];
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3e0));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0014_ee58, 0x001f_cc94)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0041_66f4, 0x001f_cca0)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            break 'b_1fcca4;
        }
        'b_1fcd5c: {
            'b_1fcd2c: {
                c.cycles += 29;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1e8));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                call_ptr(c, 0x0031_7670, 0x001f_ccc0)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, t, 0x001f_ccd0)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
                c.g[A2][0] = c.g[S2][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_26d0, 0x001f_cce4)?;
                c.g[A0][0] = 0x4a0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2540));
                call_ptr(c, 0x0031_7670, 0x001f_ccf0)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001f_ccfc)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a_0e90, 0x001f_cd08)?;
                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x470000;
                if t {
                    break 'b_1fcd2c;
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1d8));
                    call_ptr(c, 0x0039_e8b8, 0x001f_cd24)?;
                    let t = true;
                    c.g[V0][0] = 0x4;
                    if t {
                        break 'b_1fcd5c;
                    } else {
                        break 'b_1fcd2c;
                    }
                }
            }
            'b_1fcd58: {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5));
                c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x470000;
                if t {
                    break 'b_1fcd58;
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1778));
                    call_ptr(c, 0x0039_e8b8, 0x001f_cd48)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1d8));
                    call_ptr(c, 0x0039_e8b8, 0x001f_cd58)?;
                    break 'b_1fcd58;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x4;
            break 'b_1fcd5c;
        }
        'b_1fcd9c: {
            'b_1fcd84: {
                c.cycles += 2;
                if c.g[FP][0] != c.g[V0][0] {
                    c.g[A0][0] = 0x470000;
                    break 'b_1fcd84;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c8));
                    call_ptr(c, 0x0031_7670, 0x001f_cd70)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a_1000, 0x001f_cd7c)?;
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1fcd9c;
                    } else {
                        break 'b_1fcd84;
                    }
                }
            }
            c.cycles += 6;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1b0));
            call_ptr(c, 0x0031_7670, 0x001f_cd8c)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1000, 0x001f_cd98)?;
            c.g[A0][0] = 0x470000;
            break 'b_1fcd9c;
        }
        'b_1fce90: {
            'b_1fce28: {
                'b_1fcdcc: {
                    c.cycles += 9;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x190));
                    call_ptr(c, 0x0031_7670, 0x001f_cda4)?;
                    c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x001f_cdb0)?;
                    c.g[S4][0] = c.g[V0][0];
                    c.g[V0][0] = 0x4;
                    let t = c.g[FP][0] == c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1fcdcc;
                    } else {
                        c.cycles += 2;
                        let t = c.g[FP][0] != 0u64;
                        c.g[A0][0] = 0x470000;
                        if t {
                            break 'b_1fce28;
                        } else {
                            c.cycles += 1;
                            c.g[A0][0] = 0x470000;
                            break 'b_1fcdcc;
                        }
                    }
                }
                c.cycles += 23;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x180));
                call_ptr(c, 0x0031_7670, 0x001f_cdd4)?;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1000, 0x001f_cde0)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3e4));
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0015_5130, 0x001f_cdec)?;
                c.g[A0][0] = 0x470000;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x218));
                call_ptr(c, 0x0031_7670, 0x001f_cdfc)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001f_ce08)?;
                c.g[S4][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0020_a1a8, 0x001f_ce14)?;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_0c50, 0x001f_ce20)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x480));
                if t {
                    break 'b_1fce90;
                } else {
                    break 'b_1fce28;
                }
            }
            c.cycles += 26;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x168));
            call_ptr(c, 0x0031_7670, 0x001f_ce30)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1000, 0x001f_ce3c)?;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3e4));
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0015_5130, 0x001f_ce48)?;
            c.g[A0][0] = 0x470000;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x218));
            call_ptr(c, 0x0031_7670, 0x001f_ce58)?;
            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_ce64)?;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3c0));
            c.g[A1][0] = 0x4a0000;
            c.g[S4][0] = c.g[V0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2138));
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x0041_7828, 0x001f_ce80)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x003a_0c50, 0x001f_ce8c)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x480));
            break 'b_1fce90;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x470));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x460));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x450));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x440));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x430));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x420));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x410));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x400));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3f0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x490));
        c.pc = t;
        return Ok(());
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::{DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    const OC978_BASE: u32 = DATA + 0x20_0000;
    /// Gate word: the fakes only answer while this holds the magic.
    const OC978_CTRL: u32 = OC978_BASE;
    const OC978_MAGIC: u32 = 0x4f43_3938;
    /// Results of the two `UIEngine_AddScreenByHashName` calls (0 = null),
    /// and the call counter at +0x20.
    const OC978_CREATE: u32 = OC978_BASE + 0x100;
    const OC978_STATE: u32 = OC978_BASE + 0x2000;
    const OC978_WORLD: u32 = OC978_BASE + 0x3000;
    const OC978_IFACE: u32 = OC978_BASE + 0x3400;
    const OC978_IFVT: u32 = OC978_BASE + 0x3800;
    /// The new-race settings (`cBENewRaceInterface_newRace`).
    const OC978_NEWRACE: u32 = 0x0053_5bc8;

    /// The fake callee: logs like `fakes::fake_call`; the screen creator
    /// answers from the `OC978_CREATE` list (0 or a pointer).
    fn oc978_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let create = if addr == 0x0039_7cc0 {
            let k = c.bus.read32(OC978_CREATE + 0x20);
            c.bus.write32(OC978_CREATE + 0x20, k + 1);
            Some(c.bus.read32(OC978_CREATE + 4 * (k & 3)))
        } else {
            None
        };
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        if create == Some(0) {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc978_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC978_CTRL) != OC978_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc978_fake_call(c, $addr);
                }
            )*
            fn oc978_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc978_fakes!(
        (
            oc978_fake_317670,
            OC978_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc978_fake_397cc0,
            OC978_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc978_fake_39c870,
            OC978_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc978_fake_20a380,
            OC978_PREV_20A380,
            ee::s05::OVERRIDE_0020A380,
            ee::s05::f_0020a380__generated,
            0x0020_a380
        ),
        (
            oc978_fake_14dc80,
            OC978_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc978_fake_14dd58,
            OC978_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc978_fake_39d860,
            OC978_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc978_fake_3a1000,
            OC978_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc978_fake_144bf0,
            OC978_PREV_144BF0,
            ee::s01::OVERRIDE_00144BF0,
            ee::s01::f_00144bf0__generated,
            0x0014_4bf0
        ),
        (
            oc978_fake_2c2540,
            OC978_PREV_2C2540,
            ee::s08::OVERRIDE_002C2540,
            ee::s08::f_002c2540__generated,
            0x002c_2540
        ),
        (
            oc978_fake_2c27c0,
            OC978_PREV_2C27C0,
            ee::s08::OVERRIDE_002C27C0,
            ee::s08::f_002c27c0__generated,
            0x002c_27c0
        ),
        (
            oc978_fake_3a0e90,
            OC978_PREV_3A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a_0e90
        ),
        (
            oc978_fake_146f88,
            OC978_PREV_146F88,
            ee::s01::OVERRIDE_00146F88,
            ee::s01::f_00146f88__generated,
            0x0014_6f88
        ),
        (
            oc978_fake_146e98,
            OC978_PREV_146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x0014_6e98
        ),
        (
            oc978_fake_14ee58,
            OC978_PREV_14EE58,
            ee::s01::OVERRIDE_0014EE58,
            ee::s01::f_0014ee58__generated,
            0x0014_ee58
        ),
        (
            oc978_fake_3a0c50,
            OC978_PREV_3A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50
        ),
        (
            oc978_fake_39e948,
            OC978_PREV_39E948,
            ee::s13::OVERRIDE_0039E948,
            ee::s13::f_0039e948__generated,
            0x0039_e948
        ),
        (
            oc978_fake_39e8b8,
            OC978_PREV_39E8B8,
            ee::s13::OVERRIDE_0039E8B8,
            ee::s13::f_0039e8b8__generated,
            0x0039_e8b8
        ),
        (
            oc978_fake_4166f4,
            OC978_PREV_4166F4,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x0041_66f4
        ),
        (
            oc978_fake_2c26d0,
            OC978_PREV_2C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c_26d0
        ),
        (
            oc978_fake_155130,
            OC978_PREV_155130,
            ee::s01::OVERRIDE_00155130,
            ee::s01::f_00155130__generated,
            0x0015_5130
        ),
        (
            oc978_fake_20a1a8,
            OC978_PREV_20A1A8,
            ee::s05::OVERRIDE_0020A1A8,
            ee::s05::f_0020a1a8__generated,
            0x0020_a1a8
        ),
        (
            oc978_fake_417828,
            OC978_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
    );

    /// `OVScreenMultiFreestylePre_Init`: both screens created or null, game
    /// mode 0..7 (the per-mode objective lines and hidden objects), game path
    /// (multiplayer or not), event kind 0..5, the interface vtable call of the
    /// location lookup, and unwinds at any call.
    #[test]
    fn ov_screen_multi_freestyle_pre_init_matches() {
        oc978_install_fakes();
        covered(
            ee::s04::f_001fc878__generated,
            ov_screen_multi_freestyle_pre_init,
            576,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1fc878, 45);
                c.pc = 0x001f_c878;
                c.bus.write32(OC978_CTRL, OC978_MAGIC);
                c.g[A0][0] = sx(OC978_STATE);
                c.bus.write32(OC978_STATE + 0x10, r.next_u32());
                c.bus.write32(GAME_GP - 0x848, OC978_WORLD);
                c.bus.write32(OC978_WORLD + 0x8c, OC978_IFACE);
                c.bus.write32(OC978_IFACE + 4, OC978_IFVT);
                c.bus
                    .write16(OC978_IFVT + 0x20, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(OC978_IFVT + 0x24, fakes::FAKE + 0x40);
                c.bus.write8(OC978_NEWRACE + 0x48, (seed % 8) as u8);
                c.bus
                    .write8(OC978_NEWRACE + 0x49, [2u8, 0, 1][((seed / 8) % 3) as usize]);
                c.bus.write8(OC978_NEWRACE + 0x4a, ((seed / 24) % 6) as u8);
                for k in 0..4u32 {
                    let v = if (seed / 144) & 1 != 0 && k % 2 == 0 {
                        0
                    } else if (seed / 288) & 1 != 0 && k % 2 == 1 {
                        0
                    } else {
                        0x8000_0000 + 0x40 * k
                    };
                    c.bus.write32(OC978_CREATE + 4 * k, v);
                }
                c.bus.write32(OC978_CREATE + 0x20, 0);
            },
        );
    }
}
