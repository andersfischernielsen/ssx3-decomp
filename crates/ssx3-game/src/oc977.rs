//! Staging module oc977 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc977`.

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
    0x001f_bd20,
    ov_screen_freestyle_pre_init,
    "OVScreenFreestylePre_Init",
)];

/// `void OVScreenFreestylePre_Init(OVScreenFreestylePre* self)` (0x1fbd20): the
/// freestyle pre-event overlay. Creates the "freestyle_pre" and "OV_darkblue"
/// screens (starting their threads), runs the overlay template init, then
/// fills the screen's text objects: the event caption (location name plus the
/// slopestyle / big air / half-pipe title), the heat objectives and tabs (path
/// 0: heat 1 / 2 / final; other paths: quick play or multiplayer headings),
/// the three rival rows (name, run 1, run 2 and total scores, skipping the
/// human rider's own rank), the player rows and the record score.
///
/// Kept as the generated block structure (every register, cycle and call in
/// order) with the calls through `call_ptr` and unwinds through `?`.
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn ov_screen_freestyle_pre_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fbd8c: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x840));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x800));
            c.g[S3][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x820));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x810));
            c.g[A0][0] = 0x470000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x7f0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x370));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x7e0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x7d0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x7c0));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x7b0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x7a0), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x830));
            c.g[RA][0] = sx(0x001fbd60);
            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001fbd60)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x001fbd74);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001fbd74)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1fbd8c;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001fbd8c);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001fbd8c)?;
                break 'b_1fbd8c;
            }
        }
        'b_1fbdc4: {
            c.cycles += 11;
            c.g[A0][0] = 0x470000;
            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = sx(0x001fbd9c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c88));
            call_ptr(c, 0x00317670, 0x001fbd9c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x001fbdb0);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001fbdb0)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1fbdc4;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001fbdc4);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001fbdc4)?;
                break 'b_1fbdc4;
            }
        }
        'b_1fbef8: {
            'b_1fbecc: {
                'b_1fbec4: {
                    'b_1fbeb4: {
                        c.cycles += 45;
                        c.g[RA][0] = sx(0x001fbdcc);
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x0020d1d8, 0x001fbdcc)?;
                        c.g[RA][0] = sx(0x001fbdd4);
                        call_ptr(c, 0x0014dc80, 0x001fbdd4)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001fbde0);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0014dd58, 0x001fbde0)?;
                        c.g[RA][0] = sx(0x001fbde8);
                        c.g[S0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014dc80, 0x001fbde8)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001fbdf4);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x0014dd58, 0x001fbdf4)?;
                        c.g[RA][0] = sx(0x001fbdfc);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x780), c.g[V0][0]);
                        call_ptr(c, 0x0014dc80, 0x001fbdfc)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001fbe08);
                        c.g[A1][0] = 0x2;
                        call_ptr(c, 0x0014dd58, 0x001fbe08)?;
                        c.g[RA][0] = sx(0x001fbe10);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x784), c.g[V0][0]);
                        call_ptr(c, 0x0014dc80, 0x001fbe10)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001fbe1c);
                        c.g[A1][0] = 0x8;
                        call_ptr(c, 0x0014dd58, 0x001fbe1c)?;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x788), c.g[V0][0]);
                        c.g[A0][0] = 0x470000;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c78));
                        c.g[FP][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                        c.g[V0][0] = c.lw((c.g[FP][0] as u32));
                        c.g[RA][0] = sx(0x001fbe3c);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x78c), c.g[V0][0]);
                        call_ptr(c, 0x00317670, 0x001fbe3c)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001fbe48);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fbe48)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001fbe54);
                        c.g[S2][0] = c.g[V0][0];
                        call_ptr(c, 0x00144bf0, 0x001fbe54)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001fbe60);
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x002c2540, 0x001fbe60)?;
                        c.g[V1][0] = 0x530000;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                        c.g[V0][0] = 0x2;
                        c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x48));
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[A3][0] = 0u64;
                        if t {
                            break 'b_1fbeb4;
                        } else {
                            'b_1fbea8: {
                                'b_1fbe94: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_1fbe94;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        if t {
                                            break 'b_1fbea8;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                                            if t {
                                                break 'b_1fbef8;
                                            } else {
                                                break 'b_1fbe94;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0x3;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_1fbec4;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                                    if t {
                                        break 'b_1fbef8;
                                    } else {
                                        break 'b_1fbea8;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[A0][0] = 0x470000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c68));
                            if t {
                                break 'b_1fbecc;
                            } else {
                                break 'b_1fbeb4;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c58));
                    if t {
                        break 'b_1fbecc;
                    } else {
                        break 'b_1fbec4;
                    }
                }
                c.cycles += 2;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c40));
                break 'b_1fbecc;
            }
            c.cycles += 11;
            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
            c.g[RA][0] = sx(0x001fbee0);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            call_ptr(c, 0x00317670, 0x001fbee0)?;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
            c.g[A0][0] = c.g[S0][0];
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001fbef0);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, t, 0x001fbef0)?;
            c.g[A3][0] = c.g[V0][0];
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
            break 'b_1fbef8;
        }
        'b_1fc13c: {
            'b_1fc138: {
                'b_1fc120: {
                    'b_1fc0c0: {
                        c.cycles += 27;
                        c.g[A1][0] = 0x4a0000;
                        c.g[A2][0] = c.g[SP][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2140));
                        c.g[RA][0] = sx(0x001fbf0c);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x002c27c0, 0x001fbf0c)?;
                        c.g[V1][0] = 0x530000;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                        c.g[RA][0] = sx(0x001fbf20);
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x003a0e90, 0x001fbf20)?;
                        c.g[A0][0] = 0x470000;
                        c.g[RA][0] = sx(0x001fbf2c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x360));
                        call_ptr(c, 0x00317670, 0x001fbf2c)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001fbf38);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fbf38)?;
                        c.g[A0][0] = 0x470000;
                        c.g[S6][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001fbf48);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x350));
                        call_ptr(c, 0x00317670, 0x001fbf48)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001fbf54);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fbf54)?;
                        c.g[S2][0] = c.g[V0][0];
                        c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x49));
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = 0x470000;
                        if t {
                            break 'b_1fc0c0;
                        } else {
                            'b_1fbf98: {
                                c.cycles += 4;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78c));
                                c.g[V0][0] = 0x1;
                                let t = c.g[A0][0] != c.g[V0][0];
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78c));
                                if t {
                                    break 'b_1fbf98;
                                } else {
                                    c.cycles += 9;
                                    c.g[A0][0] = 0x470000;
                                    c.g[RA][0] = sx(0x001fbf80);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x340));
                                    call_ptr(c, 0x00317670, 0x001fbf80)?;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fbf8c);
                                    c.g[A0][0] = c.g[S6][0];
                                    call_ptr(c, 0x003a1000, 0x001fbf8c)?;
                                    c.g[A0][0] = 0x470000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                    if t {
                                        break 'b_1fc120;
                                    } else {
                                        break 'b_1fbf98;
                                    }
                                }
                            }
                            'b_1fc0a0: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x2;
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.g[A0][0] = 0x470000;
                                    break 'b_1fc0a0;
                                } else {
                                    'b_1fc074: {
                                        'b_1fc054: {
                                            c.cycles += 22;
                                            c.g[S0][0] = 0x530000;
                                            c.g[A0][0] = c.g[FP][0];
                                            c.g[S0][0] =
                                                sx((c.g[S0][0] as u32).wrapping_add(0x6708));
                                            c.g[A2][0] = 0u64;
                                            c.g[RA][0] = sx(0x001fbfbc);
                                            c.g[A1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            call_ptr(c, 0x00238590, 0x001fbfbc)?;
                                            c.g[A1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A2][0] = 0x1;
                                            c.g[S1][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x001fbfd0);
                                            c.g[A0][0] = c.g[FP][0];
                                            call_ptr(c, 0x00238590, 0x001fbfd0)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x780));
                                            c.g[S1][0] = sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[RA][0] = sx(0x001fbfe0);
                                            c.g[A1][0] = 0u64;
                                            call_ptr(c, 0x00146e98, 0x001fbfe0)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[A0][0] = c.g[FP][0];
                                            c.g[RA][0] = sx(0x001fbff0);
                                            c.g[A2][0] = 0u64;
                                            call_ptr(c, 0x00238590, 0x001fbff0)?;
                                            c.g[S4][0] = sx((c.g[S1][0] as u32)
                                                .wrapping_sub((c.g[V0][0] as u32)));
                                            let t = (c.g[S4][0] as i64) <= 0;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            if t {
                                                break 'b_1fc054;
                                            } else {
                                                c.cycles += 22;
                                                c.g[A0][0] = 0x470000;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x310));
                                                c.g[S0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                                c.g[S1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] =
                                                    c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                                c.g[RA][0] = sx(0x001fc018);
                                                c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                call_ptr(c, 0x00317670, 0x001fc018)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                                c.g[A0][0] = c.g[S0][0];
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x001fc028);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, t, 0x001fc028)?;
                                                c.g[S0][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x140));
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[RA][0] = sx(0x001fc03c);
                                                c.g[A2][0] = c.g[S4][0];
                                                call_ptr(c, 0x002c26d0, 0x001fc03c)?;
                                                c.g[S5][0] = c.g[S0][0];
                                                c.g[A0][0] = c.g[S6][0];
                                                c.g[RA][0] = sx(0x001fc04c);
                                                c.g[A1][0] = c.g[S0][0];
                                                call_ptr(c, 0x003a0e90, 0x001fc04c)?;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                if t {
                                                    break 'b_1fc074;
                                                } else {
                                                    break 'b_1fc054;
                                                }
                                            }
                                        }
                                        c.cycles += 8;
                                        c.g[A0][0] = 0x470000;
                                        c.g[RA][0] = sx(0x001fc060);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1b98));
                                        call_ptr(c, 0x00317670, 0x001fc060)?;
                                        c.g[A0][0] = c.g[S6][0];
                                        c.g[RA][0] = sx(0x001fc06c);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x003a1000, 0x001fc06c)?;
                                        c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                        break 'b_1fc074;
                                    }
                                    c.cycles += 11;
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001fc088);
                                    c.g[A0][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001fc088)?;
                                    c.g[A1][0] = 0x4a0000;
                                    c.g[A0][0] = c.g[S3][0];
                                    c.g[RA][0] = sx(0x001fc098);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2500));
                                    call_ptr(c, 0x0039e8b8, 0x001fc098)?;
                                    let t = true;
                                    c.g[V0][0] = 0x470000;
                                    if t {
                                        break 'b_1fc13c;
                                    } else {
                                        break 'b_1fc0a0;
                                    }
                                }
                            }
                            c.cycles += 8;
                            c.g[RA][0] = sx(0x001fc0a8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2f8));
                            call_ptr(c, 0x00317670, 0x001fc0a8)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001fc0b4);
                            c.g[A0][0] = c.g[S6][0];
                            call_ptr(c, 0x003a1000, 0x001fc0b4)?;
                            c.g[A0][0] = 0x470000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2e0));
                            if t {
                                break 'b_1fc120;
                            } else {
                                break 'b_1fc0c0;
                            }
                        }
                    }
                    'b_1fc104: {
                        c.cycles += 9;
                        c.g[RA][0] = sx(0x001fc0c8);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2e0));
                        call_ptr(c, 0x00317670, 0x001fc0c8)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001fc0d4);
                        c.g[A0][0] = c.g[S6][0];
                        call_ptr(c, 0x003a1000, 0x001fc0d4)?;
                        c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x49));
                        c.g[V0][0] = 0x2;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x48));
                            break 'b_1fc104;
                        } else {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001fc0fc);
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001fc0fc)?;
                            let t = true;
                            c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                            if t {
                                break 'b_1fc138;
                            } else {
                                break 'b_1fc104;
                            }
                        }
                    }
                    'b_1fc11c: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x470000;
                            break 'b_1fc11c;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = 0x470000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2c8));
                            if t {
                                break 'b_1fc120;
                            } else {
                                break 'b_1fc11c;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a8));
                    break 'b_1fc120;
                }
                c.cycles += 6;
                c.g[RA][0] = sx(0x001fc128);
                call_ptr(c, 0x00317670, 0x001fc128)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001fc134);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x001fc134)?;
                c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                break 'b_1fc138;
            }
            c.cycles += 1;
            c.g[V0][0] = 0x470000;
            break 'b_1fc13c;
        }
        'b_1fc25c: {
            'b_1fc1fc: {
                c.cycles += 19;
                c.g[A0][0] = c.g[S3][0];
                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x290));
                c.g[V0][0] = 0x530000;
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001fc154);
                c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                call_ptr(c, 0x0039e8b8, 0x001fc154)?;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.g[S3][0];
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x280));
                c.g[RA][0] = sx(0x001fc168);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0039e8b8, 0x001fc168)?;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.g[S3][0];
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x270));
                c.g[RA][0] = sx(0x001fc17c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0039e8b8, 0x001fc17c)?;
                c.g[V0][0] = c.lb((c.g[S4][0] as u32).wrapping_add(0x49));
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = 0x4a0000;
                if t {
                    break 'b_1fc1fc;
                } else {
                    'b_1fc1ec: {
                        'b_1fc1e8: {
                            'b_1fc1e4: {
                                'b_1fc1d8: {
                                    'b_1fc1c8: {
                                        'b_1fc1c4: {
                                            c.cycles += 4;
                                            c.g[V1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x78c));
                                            c.g[V0][0] = 0x2;
                                            let t = c.g[V1][0] == c.g[V0][0];
                                            c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                                            if t {
                                                break 'b_1fc1c4;
                                            } else {
                                                'b_1fc1b4: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x78c)
                                                        );
                                                    if t {
                                                        break 'b_1fc1b4;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = 0x3;
                                                        let t = c.g[V1][0] == c.g[V0][0];
                                                        c.g[A1][0] = 0x4a0000;
                                                        if t {
                                                            break 'b_1fc1d8;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A0][0] = c.g[S3][0];
                                                            if t {
                                                                break 'b_1fc1ec;
                                                            } else {
                                                                break 'b_1fc1b4;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = (c.g[A0][0] as i64) < 0;
                                                c.g[A1][0] = c.g[S2][0];
                                                if t {
                                                    break 'b_1fc1e4;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;

                                                    if t {
                                                        break 'b_1fc1c8;
                                                    } else {
                                                        break 'b_1fc1c4;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[A1][0] = c.g[S1][0];
                                        break 'b_1fc1c8;
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001fc1d0);
                                    c.g[A0][0] = c.g[S3][0];
                                    call_ptr(c, 0x0039e948, 0x001fc1d0)?;
                                    let t = true;
                                    c.g[A1][0] = 0x4a0000;
                                    if t {
                                        break 'b_1fc1e8;
                                    } else {
                                        break 'b_1fc1d8;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A1][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x001fc1e4);
                                c.g[A0][0] = c.g[S3][0];
                                call_ptr(c, 0x0039e948, 0x001fc1e4)?;
                                break 'b_1fc1e4;
                            }
                            c.cycles += 1;
                            c.g[A1][0] = 0x4a0000;
                            break 'b_1fc1e8;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S3][0];
                        break 'b_1fc1ec;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001fc1f4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2148));
                    call_ptr(c, 0x0039e8b8, 0x001fc1f4)?;
                    let t = true;
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        break 'b_1fc25c;
                    } else {
                        break 'b_1fc1fc;
                    }
                }
            }
            'b_1fc244: {
                c.cycles += 10;
                c.g[RA][0] = sx(0x001fc204);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2148));
                call_ptr(c, 0x00317670, 0x001fc204)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001fc210);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fc210)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V1][0] = c.lb((c.g[S4][0] as u32).wrapping_add(0x49));
                c.g[V0][0] = 0x2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A0][0] = 0x470000;
                    break 'b_1fc244;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x001fc230);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3f8));
                    call_ptr(c, 0x00317670, 0x001fc230)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001fc23c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001fc23c)?;
                    let t = true;
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        break 'b_1fc25c;
                    } else {
                        break 'b_1fc244;
                    }
                }
            }
            c.cycles += 6;
            c.g[RA][0] = sx(0x001fc24c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3e0));
            call_ptr(c, 0x00317670, 0x001fc24c)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001fc258);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a1000, 0x001fc258)?;
            c.g[A0][0] = 0x4a0000;
            break 'b_1fc25c;
        }
        'b_1fc344: {
            'b_1fc30c: {
                c.cycles += 11;
                c.g[RA][0] = sx(0x001fc264);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2158));
                call_ptr(c, 0x00317670, 0x001fc264)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001fc270);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fc270)?;
                c.g[A0][0] = 0x530000;
                c.g[S2][0] = c.g[V0][0];
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                if c.g[V0][0] != 0u64 {
                    c.g[A0][0] = 0x470000;
                    break 'b_1fc30c;
                } else {
                    'b_1fc2d0: {
                        'b_1fc2b8: {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78c));
                            c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.g[A0][0] = 0x470000;
                                break 'b_1fc2b8;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = 0x470000;
                                c.g[RA][0] = sx(0x001fc2a4);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c00));
                                call_ptr(c, 0x00317670, 0x001fc2a4)?;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[RA][0] = sx(0x001fc2b0);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x003a1000, 0x001fc2b0)?;
                                let t = true;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78c));
                                if t {
                                    break 'b_1fc2d0;
                                } else {
                                    break 'b_1fc2b8;
                                }
                            }
                        }
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x001fc2c0);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x260));
                        call_ptr(c, 0x00317670, 0x001fc2c0)?;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x001fc2cc);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x001fc2cc)?;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78c));
                        break 'b_1fc2d0;
                    }
                    'b_1fc2e4: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[A0][0] == c.g[V0][0];
                        c.g[V0][0] = 0x3;
                        if t {
                            break 'b_1fc2e4;
                        } else {
                            c.cycles += 2;
                            let t = c.g[A0][0] != c.g[V0][0];
                            c.g[V0][0] = 0x530000;
                            if t {
                                break 'b_1fc344;
                            } else {
                                break 'b_1fc2e4;
                            }
                        }
                    }
                    c.cycles += 10;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001fc2f4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x250));
                    call_ptr(c, 0x0039e8b8, 0x001fc2f4)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001fc304);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x238));
                    call_ptr(c, 0x0039e8b8, 0x001fc304)?;
                    let t = true;
                    c.g[V0][0] = 0x530000;
                    if t {
                        break 'b_1fc344;
                    } else {
                        break 'b_1fc30c;
                    }
                }
            }
            c.cycles += 14;
            c.g[RA][0] = sx(0x001fc314);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c00));
            call_ptr(c, 0x00317670, 0x001fc314)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001fc320);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x003a1000, 0x001fc320)?;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x001fc330);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x250));
            call_ptr(c, 0x0039e8b8, 0x001fc330)?;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x001fc340);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x238));
            call_ptr(c, 0x0039e8b8, 0x001fc340)?;
            c.g[V0][0] = 0x530000;
            break 'b_1fc344;
        }
        c.cycles += 9;
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
        c.g[S7][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6708));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x790), c.g[S5][0]);
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x170));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x798), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x794), c.g[V0][0]);
        c.g[S6][0] = 0x1;
        c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
        'l_1fc368: loop {
            'b_1fc388: {
                c.cycles += 6;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x780));
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001fc378);
                c.g[S2][0] = c.lw((c.g[S7][0] as u32));
                call_ptr(c, 0x00146e98, 0x001fc378)?;
                let t = c.g[S2][0] != c.g[V0][0];
                c.g[A0][0] = 0x470000;
                if t {
                    break 'b_1fc388;
                } else {
                    c.cycles += 2;
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x4));
                    c.g[S2][0] = c.lw((c.g[S7][0] as u32));
                    break 'b_1fc388;
                }
            }
            'b_1fc3dc: {
                c.cycles += 12;
                c.g[A2][0] = c.g[S6][0];
                c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1be8));
                c.g[RA][0] = sx(0x001fc398);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x790));
                call_ptr(c, 0x00417828, 0x001fc398)?;
                c.g[RA][0] = sx(0x001fc3a0);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x790));
                call_ptr(c, 0x00317670, 0x001fc3a0)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001fc3ac);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fc3ac)?;
                c.g[S1][0] = c.g[V0][0];
                let t = c.g[S1][0] == 0u64;
                c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                if t {
                    break 'b_1fc3dc;
                } else {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x784));
                    c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V0][0] = c.g[V1][0];
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001fc3d0);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0014eec8, 0x001fc3d0)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001fc3dc);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a0c50, 0x001fc3dc)?;
                    break 'b_1fc3dc;
                }
            }
            'b_1fc448: {
                c.cycles += 13;
                c.g[V0][0] = 0x470000;
                c.g[A2][0] = c.g[S6][0];
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1bd8));
                c.g[RA][0] = sx(0x001fc3f0);
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x00417828, 0x001fc3f0)?;
                c.g[RA][0] = sx(0x001fc3f8);
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x00317670, 0x001fc3f8)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001fc404);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fc404)?;
                c.g[S1][0] = c.g[V0][0];
                let t = c.g[S1][0] == 0u64;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78c));
                if t {
                    break 'b_1fc448;
                } else {
                    c.cycles += 14;
                    c.g[A2][0] = 0x2;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[V0][0] = c.g[V1][0] ^ 0x3;
                    c.g[RA][0] = sx(0x001fc428);
                    if c.g[V0][0] != 0 {
                        c.g[A2][0] = 0u64;
                    }
                    call_ptr(c, 0x00238590, 0x001fc428)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[V0][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x001fc43c);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2138));
                    call_ptr(c, 0x00417828, 0x001fc43c)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001fc448);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x003a0c50, 0x001fc448)?;
                    break 'b_1fc448;
                }
            }
            'b_1fc548: {
                'b_1fc538: {
                    c.cycles += 16;
                    c.g[V1][0] = 0x470000;
                    c.g[A2][0] = c.g[S6][0];
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1bc8));
                    c.g[RA][0] = sx(0x001fc45c);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x00417828, 0x001fc45c)?;
                    c.g[RA][0] = sx(0x001fc464);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x00317670, 0x001fc464)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001fc470);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fc470)?;
                    c.g[A0][0] = 0x530000;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                    c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        break 'b_1fc538;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78c));
                        c.g[V1][0] = 0x2;
                        let t = c.g[V0][0] != c.g[V1][0];
                        c.g[V0][0] = 0x4a0000;
                        if t {
                            break 'b_1fc538;
                        } else {
                            c.cycles += 40;
                            c.g[A0][0] = c.g[FP][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001fc4a8);
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x00238590, 0x001fc4a8)?;
                            c.g[A2][0] = c.g[V0][0];
                            c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_sub(0xfb8));
                            c.g[RA][0] = sx(0x001fc4b8);
                            c.g[A0][0] = c.g[S4][0];
                            call_ptr(c, 0x00417828, 0x001fc4b8)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001fc4c4);
                            c.g[A1][0] = c.g[S4][0];
                            call_ptr(c, 0x003a0c50, 0x001fc4c4)?;
                            c.g[A0][0] = 0x470000;
                            c.g[A2][0] = c.g[S6][0];
                            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1bb8));
                            c.g[RA][0] = sx(0x001fc4d8);
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x794));
                            call_ptr(c, 0x00417828, 0x001fc4d8)?;
                            c.g[RA][0] = sx(0x001fc4e0);
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x794));
                            call_ptr(c, 0x00317670, 0x001fc4e0)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001fc4ec);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001fc4ec)?;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[FP][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001fc500);
                            c.g[A2][0] = 0u64;
                            call_ptr(c, 0x00238590, 0x001fc500)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A0][0] = c.g[FP][0];
                            c.g[RA][0] = sx(0x001fc514);
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x00238590, 0x001fc514)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x798));
                            c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_sub(0xfb8));
                            c.g[RA][0] = sx(0x001fc524);
                            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            call_ptr(c, 0x00417828, 0x001fc524)?;
                            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x798));
                            c.g[RA][0] = sx(0x001fc530);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x003a0c50, 0x001fc530)?;
                            let t = true;
                            c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                            if t {
                                break 'b_1fc548;
                            } else {
                                break 'b_1fc538;
                            }
                        }
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x001fc544);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2168));
                call_ptr(c, 0x003a0c50, 0x001fc544)?;
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                break 'b_1fc548;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S6][0] as i64) < 4i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001fc368);
                }
                continue 'l_1fc368;
            } else {
                'b_1fc64c: {
                    'b_1fc640: {
                        'b_1fc63c: {
                            c.cycles += 37;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x780));
                            c.g[RA][0] = sx(0x001fc560);
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x00146e98, 0x001fc560)?;
                            c.g[A0][0] = 0x4a0000;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001fc570);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2508));
                            call_ptr(c, 0x00317670, 0x001fc570)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001fc57c);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001fc57c)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x780));
                            c.g[S2][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001fc58c);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x00147398, 0x001fc58c)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x780));
                            c.g[S0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001fc59c);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x00147410, 0x001fc59c)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x784));
                            c.g[A2][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001fc5ac);
                            c.g[A1][0] = c.g[S0][0];
                            call_ptr(c, 0x0014eec8, 0x001fc5ac)?;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001fc5b8);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a0c50, 0x001fc5b8)?;
                            c.g[A0][0] = 0x4a0000;
                            c.g[RA][0] = sx(0x001fc5c4);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2510));
                            call_ptr(c, 0x00317670, 0x001fc5c4)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001fc5d0);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001fc5d0)?;
                            c.g[A0][0] = 0x530000;
                            c.g[S2][0] = c.g[V0][0];
                            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                            let t = c.g[V0][0] != 0u64;
                            c.g[A1][0] = 0x4a0000;
                            if t {
                                break 'b_1fc63c;
                            } else {
                                c.cycles += 4;
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78c));
                                c.g[V0][0] = 0x2;
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A0][0] = c.g[S2][0];
                                if t {
                                    break 'b_1fc640;
                                } else {
                                    c.cycles += 17;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x780));
                                    c.g[RA][0] = sx(0x001fc604);
                                    c.g[A1][0] = 0u64;
                                    call_ptr(c, 0x00146e98, 0x001fc604)?;
                                    c.g[A0][0] = c.g[FP][0];
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fc614);
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, 0x00238590, 0x001fc614)?;
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S5][0];
                                    c.g[V0][0] = 0x4a0000;
                                    c.g[RA][0] = sx(0x001fc628);
                                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2138));
                                    call_ptr(c, 0x00417828, 0x001fc628)?;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x001fc634);
                                    c.g[A1][0] = c.g[S5][0];
                                    call_ptr(c, 0x003a0c50, 0x001fc634)?;
                                    let t = true;
                                    c.g[A0][0] = 0x4a0000;
                                    if t {
                                        break 'b_1fc64c;
                                    } else {
                                        break 'b_1fc63c;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S2][0];
                        break 'b_1fc640;
                    }
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001fc648);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2168));
                    call_ptr(c, 0x003a0c50, 0x001fc648)?;
                    c.g[A0][0] = 0x4a0000;
                    break 'b_1fc64c;
                }
                'b_1fc6f4: {
                    'b_1fc6cc: {
                        c.cycles += 12;
                        c.g[RA][0] = sx(0x001fc654);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2518));
                        call_ptr(c, 0x00317670, 0x001fc654)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001fc660);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fc660)?;
                        c.g[A0][0] = 0x530000;
                        c.g[S2][0] = c.g[V0][0];
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                        c.g[V0][0] = 0x2;
                        c.g[A0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                        if c.g[A0][0] != c.g[V0][0] {
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            break 'b_1fc6cc;
                        } else {
                            c.cycles += 20;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x780));
                            c.g[RA][0] = sx(0x001fc688);
                            c.g[A1][0] = 0x1;
                            call_ptr(c, 0x00146e98, 0x001fc688)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x780));
                            c.g[S1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001fc698);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x00147398, 0x001fc698)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x780));
                            c.g[S0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001fc6a8);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x00147410, 0x001fc6a8)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x784));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001fc6b8);
                            c.g[A2][0] = c.g[V0][0];
                            call_ptr(c, 0x0014eec8, 0x001fc6b8)?;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001fc6c4);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a0c50, 0x001fc6c4)?;
                            let t = true;
                            c.g[A0][0] = 0x470000;
                            if t {
                                break 'b_1fc6f4;
                            } else {
                                break 'b_1fc6cc;
                            }
                        }
                    }
                    c.cycles += 10;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001fc6e0);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001fc6e0)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001fc6f0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x228));
                    call_ptr(c, 0x0039e8b8, 0x001fc6f0)?;
                    c.g[A0][0] = 0x470000;
                    break 'b_1fc6f4;
                }
                c.cycles += 29;
                c.g[RA][0] = sx(0x001fc6fc);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x218));
                call_ptr(c, 0x00317670, 0x001fc6fc)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001fc708);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fc708)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x788));
                c.g[S2][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001fc718);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00155130, 0x001fc718)?;
                c.g[V1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S5][0];
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2138));
                c.g[RA][0] = sx(0x001fc72c);
                c.g[A2][0] = c.g[V0][0];
                call_ptr(c, 0x00417828, 0x001fc72c)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001fc738);
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, 0x003a0c50, 0x001fc738)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x830));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x820));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x810));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x800));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x7f0));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x7e0));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x7d0));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x7c0));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x7b0));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x7a0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x840));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::{covered, machine};
    use crate::difftest::{fill, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    // Fixture objects, all in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    const OC977_BASE: u32 = DATA + 0x20_0000;
    /// The word that arms this module's callee fakes in a machine's RAM.
    const OC977_CTRL: u32 = OC977_BASE;
    const OC977_MAGIC: u32 = 0x0c97_7f5e;
    const OC977_STATE: u32 = OC977_BASE + 0x2000;
    const OC977_WORLD: u32 = OC977_BASE + 0x3000;
    const OC977_CFG: u32 = OC977_BASE + 0x3400;
    const OC977_IFACE: u32 = OC977_BASE + 0x3c00;
    const OC977_IFVT: u32 = OC977_BASE + 0x3d00;
    /// `GameModeMan` byte (the event kind, `0x535c10`) and game-path byte (`0x535c11`).
    const OC977_KIND: u32 = 0x0053_5c10;
    const OC977_PATH: u32 = 0x0053_5c11;
    /// `cGameModeMan_iCharRank` (`0x536708`).
    const OC977_RANK: u32 = 0x0053_6708;

    /// The mailbox word of the fake for `addr` (0: the fake's own pointer).
    fn oc977_mail(addr: u32) -> u32 {
        OC977_BASE + 0x10_0000 + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0`.
    fn oc977_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc977_mail(addr), v.wrapping_add(3));
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox unless it unwound.
    fn oc977_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc977_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                m => sx(m - 3),
            };
        }
    }

    macro_rules! oc977_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC977_CTRL) != OC977_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc977_fake_call(c, $addr);
                }
            )*
            fn oc977_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc977_fakes!(
        (
            oc977_fake_144bf0,
            OC977_PREV_00144BF0,
            ee::s01::OVERRIDE_00144BF0,
            ee::s01::f_00144bf0__generated,
            0x00144bf0
        ),
        (
            oc977_fake_146e98,
            OC977_PREV_00146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x00146e98
        ),
        (
            oc977_fake_147398,
            OC977_PREV_00147398,
            ee::s01::OVERRIDE_00147398,
            ee::s01::f_00147398__generated,
            0x00147398
        ),
        (
            oc977_fake_147410,
            OC977_PREV_00147410,
            ee::s01::OVERRIDE_00147410,
            ee::s01::f_00147410__generated,
            0x00147410
        ),
        (
            oc977_fake_14dc80,
            OC977_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc977_fake_14dd58,
            OC977_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc977_fake_14eec8,
            OC977_PREV_0014EEC8,
            ee::s01::OVERRIDE_0014EEC8,
            ee::s01::f_0014eec8__generated,
            0x0014eec8
        ),
        (
            oc977_fake_155130,
            OC977_PREV_00155130,
            ee::s01::OVERRIDE_00155130,
            ee::s01::f_00155130__generated,
            0x00155130
        ),
        (
            oc977_fake_20d1d8,
            OC977_PREV_0020D1D8,
            ee::s05::OVERRIDE_0020D1D8,
            ee::s05::f_0020d1d8__generated,
            0x0020d1d8
        ),
        (
            oc977_fake_238590,
            OC977_PREV_00238590,
            ee::s06::OVERRIDE_00238590,
            ee::s06::f_00238590__generated,
            0x00238590
        ),
        (
            oc977_fake_2c2540,
            OC977_PREV_002C2540,
            ee::s08::OVERRIDE_002C2540,
            ee::s08::f_002c2540__generated,
            0x002c2540
        ),
        (
            oc977_fake_2c26d0,
            OC977_PREV_002C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c26d0
        ),
        (
            oc977_fake_2c27c0,
            OC977_PREV_002C27C0,
            ee::s08::OVERRIDE_002C27C0,
            ee::s08::f_002c27c0__generated,
            0x002c27c0
        ),
        (
            oc977_fake_317670,
            OC977_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc977_fake_397cc0,
            OC977_PREV_00397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x00397cc0
        ),
        (
            oc977_fake_39c870,
            OC977_PREV_0039C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039c870
        ),
        (
            oc977_fake_39d860,
            OC977_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039d860
        ),
        (
            oc977_fake_39e8b8,
            OC977_PREV_0039E8B8,
            ee::s13::OVERRIDE_0039E8B8,
            ee::s13::f_0039e8b8__generated,
            0x0039e8b8
        ),
        (
            oc977_fake_39e948,
            OC977_PREV_0039E948,
            ee::s13::OVERRIDE_0039E948,
            ee::s13::f_0039e948__generated,
            0x0039e948
        ),
        (
            oc977_fake_3a0c50,
            OC977_PREV_003A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a0c50
        ),
        (
            oc977_fake_3a0e90,
            OC977_PREV_003A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a0e90
        ),
        (
            oc977_fake_3a1000,
            OC977_PREV_003A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a1000
        ),
        (
            oc977_fake_417828,
            OC977_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x00417828
        ),
    );

    /// `OVScreenFreestylePre_Init`: both screens created or null (the thread
    /// starts follow), the event kind byte (0..4: caption pick), the game path
    /// (0 career, 1 quick play, 2 multiplayer), the heat number (-1..5: objective
    /// texts, tabs, score columns), the human rider equal to the first ranked
    /// rider or not (the row skip), the heat-score result (0 or positive: the
    /// "earn x points" branch), and null text objects. Every callee runs on a
    /// logged fake; `covered` also unwinds at random call indices.
    #[test]
    fn ov_screen_freestyle_pre_init_matches() {
        oc977_install_fakes();
        covered(
            ee::s04::f_001fbd20__generated,
            ov_screen_freestyle_pre_init,
            420,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1fbd20, 100);
                c.pc = 0x001f_bd20;
                c.bus.write32(OC977_CTRL, OC977_MAGIC);
                c.g[A0][0] = sx(OC977_STATE);
                c.bus.write32(OC977_STATE + 0x10, r.next_u32());
                // The world, the heat config (heat number first) and the
                // interface object the event kind query goes through.
                c.bus.write32(GAME_GP - 0x848, OC977_WORLD);
                c.bus.write32(OC977_WORLD + 0xc0, OC977_CFG);
                c.bus.write32(OC977_WORLD + 0x8c, OC977_IFACE);
                fill(c, &mut r, OC977_CFG, 0x100);
                let heat = [-1i32, 0, 1, 2, 3, 4, 5][((seed / 15) % 7) as usize];
                c.bus.write32(OC977_CFG, heat as u32);
                c.bus.write32(OC977_IFACE + 4, OC977_IFVT);
                c.bus
                    .write16(OC977_IFVT + 0x20, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(OC977_IFVT + 0x24, fakes::FAKE + 0x40);
                // Event kind and game path.
                c.bus.write8(OC977_KIND, (seed % 5) as u8);
                c.bus.write8(OC977_PATH, ((seed / 5) % 3) as u8);
                // Rank table with small character ids.
                for k in 0..8 {
                    c.bus.write32(OC977_RANK + 4 * k, r.next_u32() % 8);
                }
                // The human rider: equal to the first ranked one, or a pointer.
                if r.next_u32() % 2 == 0 {
                    let first = c.bus.read32(OC977_RANK);
                    oc977_force(c, 0x0014_6e98, first);
                }
                // Heat scores: zero, small positive, or the fake's pointer.
                match r.next_u32() % 3 {
                    0 => oc977_force(c, 0x0023_8590, 0),
                    1 => oc977_force(c, 0x0023_8590, 5),
                    _ => {}
                }
                // Null screens / text objects.
                if r.next_u32() % 4 == 0 {
                    oc977_force(c, 0x0039_7cc0, 0);
                }
                if r.next_u32() % 5 == 0 {
                    oc977_force(c, 0x0039_d860, 0);
                }
            },
        );
    }
}
