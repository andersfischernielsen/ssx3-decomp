//! Staging module oc936 (wave wave255): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc936`.

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
    (
        0x001d_2ad0,
        c_fe_state_mountain_room_on_widget_created,
        "cFEStateMountainRoom_OnWidgetCreated",
    ),
    (
        0x001d_3340,
        c_fe_state_mountain_room_update_event_icons,
        "cFEStateMountainRoom_UpdateEventIcons",
    ),
    (
        0x001d_38f8,
        c_fe_state_peak_room_on_widget_created,
        "cFEStatePeakRoom_OnWidgetCreated",
    ),
    (
        0x001c_2f00,
        c_fe_state_stats_on_widget_created,
        "cFEStateStats_OnWidgetCreated",
    ),
];

/// `cFEStateMountainRoom_OnWidgetCreated(this, widget)` (0x001d2ad0): a screen widget was just created; its name hash (`widget+0x38`) is matched against the screen's known names and the widget is stored in the matching slot of `this` (title text, pass icon, level-number texture, race / slopestyle / superpipe / backcountry / big-air marker slots, peak pages 0-3, help text).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias). Every register,
/// cycle and call of the generated body is kept.
pub fn c_fe_state_mountain_room_on_widget_created(c: &mut Cpu) {
    if c.pc != 0x001d2ad0 {
        return ee::s04::f_001d2ad0__generated(c);
    }
    guest(c, |c| {
        'b_1d2e88: {
            'b_1d2b24: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                c.g[S1][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A0][0] = 0x460000;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x001d2afc);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e60));
                call_ptr(c, 0x00317670, 0x001d2afc)?;
                if c.g[S0][0] != c.g[V0][0] {
                    c.g[A0][0] = 0x460000;
                    break 'b_1d2b24;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x001d2b10);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7238));
                    call_ptr(c, 0x00317670, 0x001d2b10)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001d2b1c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001d2b1c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    if t {
                        break 'b_1d2e88;
                    } else {
                        break 'b_1d2b24;
                    }
                }
            }
            'b_1d2e84: {
                'b_1d2dec: {
                    'b_1d2dcc: {
                        'b_1d2b3c: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2b2c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7250));
                            call_ptr(c, 0x00317670, 0x001d2b2c)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x4a0000;
                                break 'b_1d2b3c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x48), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2b3c;
                                }
                            }
                        }
                        'b_1d2b54: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2b44);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f20));
                            call_ptr(c, 0x00317670, 0x001d2b44)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2b54;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x4c), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2b54;
                                }
                            }
                        }
                        'b_1d2b6c: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2b5c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7260));
                            call_ptr(c, 0x00317670, 0x001d2b5c)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2b6c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x50), c.g[S1][0]);
                                if t {
                                    break 'b_1d2e84;
                                } else {
                                    break 'b_1d2b6c;
                                }
                            }
                        }
                        'b_1d2bf0: {
                            'b_1d2b8c: {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001d2b74);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7270));
                                call_ptr(c, 0x00317670, 0x001d2b74)?;
                                let t = c.g[S0][0] == c.g[V0][0];
                                c.g[A0][0] = 0x460000;
                                if t {
                                    break 'b_1d2b8c;
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001d2b84);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7280));
                                    call_ptr(c, 0x00317670, 0x001d2b84)?;
                                    let t = c.g[S0][0] != c.g[V0][0];
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_1d2bf0;
                                    } else {
                                        break 'b_1d2b8c;
                                    }
                                }
                            }
                            c.cycles += 25;
                            c.g[V0][0] = 0x460000;
                            c.g[A0][0] = c.g[SP][0] | 0x5;
                            c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7298));
                            c.lwl(3, (c.g[T1][0] as u32).wrapping_add(0x3));
                            c.lwr(3, (c.g[T1][0] as u32));
                            c.g[A3][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x4));
                            c.swl(3, (c.g[SP][0] as u32).wrapping_add(0x3));
                            c.swr(3, (c.g[SP][0] as u32));
                            c.sb((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A3][0]);
                            c.g[A1][0] = 0u64;
                            c.g[RA][0] = sx(0x001d2bbc);
                            c.g[A2][0] = 0xf;
                            call_ptr(c, 0x00416210, 0x001d2bbc)?;
                            c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0xb8));
                            c.g[A0][0] = c.g[SP][0];
                            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
                            c.g[RA][0] = sx(0x001d2bd4);
                            c.sb((c.g[SP][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                            call_ptr(c, 0x00317670, 0x001d2bd4)?;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                            c.g[RA][0] = sx(0x001d2be0);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x00398380, 0x001d2be0)?;
                            c.g[V1][0] = 0xffffffffffffffff;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x7c), c.g[V0][0]);
                            let t = true;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x78), c.g[V1][0]);
                            if t {
                                break 'b_1d2e84;
                            } else {
                                break 'b_1d2bf0;
                            }
                        }
                        'b_1d2c08: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2bf8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x72a0));
                            call_ptr(c, 0x00317670, 0x001d2bf8)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2c08;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x58), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2c08;
                                }
                            }
                        }
                        'b_1d2c20: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2c10);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x72b0));
                            call_ptr(c, 0x00317670, 0x001d2c10)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2c20;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x5c), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2c20;
                                }
                            }
                        }
                        'b_1d2c38: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2c28);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x72c0));
                            call_ptr(c, 0x00317670, 0x001d2c28)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2c38;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x60), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2c38;
                                }
                            }
                        }
                        'b_1d2c50: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2c40);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x72d0));
                            call_ptr(c, 0x00317670, 0x001d2c40)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2c50;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x64), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2c50;
                                }
                            }
                        }
                        'b_1d2c68: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2c58);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x72e0));
                            call_ptr(c, 0x00317670, 0x001d2c58)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2c68;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x68), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2c68;
                                }
                            }
                        }
                        'b_1d2c80: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2c70);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x72f0));
                            call_ptr(c, 0x00317670, 0x001d2c70)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2c80;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x6c), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2c80;
                                }
                            }
                        }
                        'b_1d2c98: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2c88);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7308));
                            call_ptr(c, 0x00317670, 0x001d2c88)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2c98;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x70), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2c98;
                                }
                            }
                        }
                        'b_1d2cb0: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2ca0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7320));
                            call_ptr(c, 0x00317670, 0x001d2ca0)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2cb0;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x74), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2cb0;
                                }
                            }
                        }
                        'b_1d2cc8: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2cb8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7338));
                            call_ptr(c, 0x00317670, 0x001d2cb8)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2cc8;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x90), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2cc8;
                                }
                            }
                        }
                        'b_1d2ce0: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2cd0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7348));
                            call_ptr(c, 0x00317670, 0x001d2cd0)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2ce0;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x94), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2ce0;
                                }
                            }
                        }
                        'b_1d2cf8: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2ce8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7358));
                            call_ptr(c, 0x00317670, 0x001d2ce8)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2cf8;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2cf8;
                                }
                            }
                        }
                        'b_1d2d10: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2d00);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7368));
                            call_ptr(c, 0x00317670, 0x001d2d00)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2d10;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x78), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2d10;
                                }
                            }
                        }
                        'b_1d2d28: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2d18);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7380));
                            call_ptr(c, 0x00317670, 0x001d2d18)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2d28;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x7c), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2d28;
                                }
                            }
                        }
                        'b_1d2d40: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2d30);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7398));
                            call_ptr(c, 0x00317670, 0x001d2d30)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2d40;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x80), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2d40;
                                }
                            }
                        }
                        'b_1d2d58: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2d48);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x73b0));
                            call_ptr(c, 0x00317670, 0x001d2d48)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2d58;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x84), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2d58;
                                }
                            }
                        }
                        'b_1d2d70: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2d60);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x73d0));
                            call_ptr(c, 0x00317670, 0x001d2d60)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2d70;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x88), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2d70;
                                }
                            }
                        }
                        'b_1d2d88: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2d78);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x73f0));
                            call_ptr(c, 0x00317670, 0x001d2d78)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2d88;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x8c), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2d88;
                                }
                            }
                        }
                        'b_1d2da0: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2d90);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7410));
                            call_ptr(c, 0x00317670, 0x001d2d90)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2da0;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x9c), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2da0;
                                }
                            }
                        }
                        'b_1d2db8: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d2da8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7420));
                            call_ptr(c, 0x00317670, 0x001d2da8)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1d2db8;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0xa0), c.g[S1][0]);
                                if t {
                                    break 'b_1d2dcc;
                                } else {
                                    break 'b_1d2db8;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001d2dc0);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7430));
                        call_ptr(c, 0x00317670, 0x001d2dc0)?;
                        if c.g[S0][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x460000;
                            break 'b_1d2dec;
                        } else {
                            c.cycles += 1;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0xa4), c.g[S1][0]);
                            break 'b_1d2dcc;
                        }
                    }
                    c.cycles += 8;
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d2de4);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001d2de4)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    if t {
                        break 'b_1d2e88;
                    } else {
                        break 'b_1d2dec;
                    }
                }
                'b_1d2e04: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d2df4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7440));
                    call_ptr(c, 0x00317670, 0x001d2df4)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        break 'b_1d2e04;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
                        if t {
                            break 'b_1d2e84;
                        } else {
                            break 'b_1d2e04;
                        }
                    }
                }
                'b_1d2e20: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d2e0c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f28));
                    call_ptr(c, 0x00317670, 0x001d2e0c)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        break 'b_1d2e20;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1d2e84;
                        } else {
                            break 'b_1d2e20;
                        }
                    }
                }
                'b_1d2e3c: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d2e28);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f30));
                    call_ptr(c, 0x00317670, 0x001d2e28)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        break 'b_1d2e3c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x2;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1d2e84;
                        } else {
                            break 'b_1d2e3c;
                        }
                    }
                }
                'b_1d2e58: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d2e44);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f38));
                    call_ptr(c, 0x00317670, 0x001d2e44)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        break 'b_1d2e58;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x3;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1d2e84;
                        } else {
                            break 'b_1d2e58;
                        }
                    }
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x001d2e60);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x488));
                call_ptr(c, 0x00317670, 0x001d2e60)?;
                let t = c.g[S0][0] != c.g[V0][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                if t {
                    break 'b_1d2e88;
                } else {
                    c.cycles += 7;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x54), c.g[S1][0]);
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d2e84);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001d2e84)?;
                    break 'b_1d2e84;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_1d2e88;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateMountainRoom_UpdateEventIcons(this)` (0x001d3340): walks mountains 0-2, peaks 0-1 and the up-to-five events of each (medal table at 0x45aad8), asks the trophy room for the event's reward and sets the marker widget of its kind (race, slopestyle, superpipe, big air, backcountry race / freestyle) to the icon of the result (`+0x78` icon, `+0x7c` texture).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias). Every register,
/// cycle and call of the generated body is kept.
pub fn c_fe_state_mountain_room_update_event_icons(c: &mut Cpu) {
    if c.pc != 0x001d3340 {
        return ee::s04::f_001d3340__generated(c);
    }
    guest(c, |c| {
        c.cycles += 20;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
        c.sw((c.g[SP][0] as u32), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
        'l_1d3390: loop {
            c.cycles += 22;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = 0x28;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            c.mult(0, 2, 2, 3);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A0][0]);
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
            c.g[S7][0] = sx((c.g[V1][0] as u32) << 2);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
            c.g[S6][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[FP][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A0][0] = c.lw((c.g[SP][0] as u32));
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.g[S4][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[S3][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[S5][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
            'l_1d33e8: loop {
                'b_1d372c: {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[A0][0]);
                    if t {
                        break 'b_1d372c;
                    } else {
                        c.cycles += 12;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                        c.g[V0][0] = 0xa;
                        c.g[A0][0] = 0x460000;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                        c.lo[0] = c.g[V1][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5528));
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                        c.madd(0, 2, 3, 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.g[S0][0] = c.lb((c.g[V0][0] as u32));
                        let t = (c.g[S0][0] as i64) < 0;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        if t {
                            break 'b_1d372c;
                        } else {
                            c.cycles += 19;
                            c.g[S2][0] = 0xffffffffffffffff;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V1][0]);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[A0][0]);
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V1][0]);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[A0][0]);
                            c.g[A0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                            'l_1d3478: loop {
                                'b_1d36e4: {
                                    'b_1d3674: {
                                        'b_1d3658: {
                                            'b_1d34e8: {
                                                c.cycles += 7;
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0xb0));
                                                c.g[A3][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                                c.g[RA][0] = sx(0x001d348c);
                                                c.g[T0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                                Callee {
                                                    addr: 0x001ced90,
                                                    f: ee::s04::cFEStateTrophyRoom_QueryEventReward,
                                                }
                                                .call(c, 0x001d348c)?;
                                                let t = c.g[S0][0] != 0u64;
                                                c.g[V1][0] = c.g[V0][0];
                                                if t {
                                                    break 'b_1d34e8;
                                                } else {
                                                    'b_1d34ac: {
                                                        'b_1d34a8: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] =
                                                                (c.g[V1][0] < 0x4u64) as u64;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V1][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x58));
                                                            if t {
                                                                break 'b_1d34a8;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0xa8));
                                                                if t {
                                                                    break 'b_1d34ac;
                                                                } else {
                                                                    break 'b_1d34a8;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0xac));
                                                        break 'b_1d34ac;
                                                    }
                                                    c.cycles += 15;
                                                    c.g[A0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[S3][0] as u32)));
                                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x7c),
                                                        c.g[A1][0],
                                                    );
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x78),
                                                        c.g[S2][0],
                                                    );
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                                    c.g[S3][0] =
                                                        sx((c.g[S3][0] as u32).wrapping_add(0x4));
                                                    c.g[A1][0] = 0x1;
                                                    c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x44),
                                                        c.g[A0][0],
                                                    );
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                    let t = true;
                                                    c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                                                    if t {
                                                        break 'b_1d3658;
                                                    } else {
                                                        break 'b_1d34e8;
                                                    }
                                                }
                                            }
                                            'b_1d3548: {
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x1;
                                                let t = c.g[S0][0] != c.g[V0][0];
                                                c.g[V0][0] = 0x2;
                                                if t {
                                                    break 'b_1d3548;
                                                } else {
                                                    'b_1d350c: {
                                                        'b_1d3508: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] =
                                                                (c.g[V1][0] < 0x4u64) as u64;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V1][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x6c));
                                                            if t {
                                                                break 'b_1d3508;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0xa8));
                                                                if t {
                                                                    break 'b_1d350c;
                                                                } else {
                                                                    break 'b_1d3508;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0xac));
                                                        break 'b_1d350c;
                                                    }
                                                    c.cycles += 15;
                                                    c.g[A0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[S4][0] as u32)));
                                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x7c),
                                                        c.g[A1][0],
                                                    );
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x78),
                                                        c.g[S2][0],
                                                    );
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[S4][0] =
                                                        sx((c.g[S4][0] as u32).wrapping_add(0x4));
                                                    c.g[A1][0] = 0x1;
                                                    c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                                    c.g[V0][0] =
                                                        c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x40),
                                                        c.g[A0][0],
                                                    );
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x4),
                                                        c.g[V0][0],
                                                    );
                                                    if t {
                                                        break 'b_1d3658;
                                                    } else {
                                                        break 'b_1d3548;
                                                    }
                                                }
                                            }
                                            'b_1d35a4: {
                                                c.cycles += 2;
                                                let t = c.g[S0][0] != c.g[V0][0];
                                                c.g[V0][0] = 0x3;
                                                if t {
                                                    break 'b_1d35a4;
                                                } else {
                                                    'b_1d3568: {
                                                        'b_1d3564: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] =
                                                                (c.g[V1][0] < 0x4u64) as u64;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V1][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x90));
                                                            if t {
                                                                break 'b_1d3564;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0xa8));
                                                                if t {
                                                                    break 'b_1d3568;
                                                                } else {
                                                                    break 'b_1d3564;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0xac));
                                                        break 'b_1d3568;
                                                    }
                                                    c.cycles += 15;
                                                    c.g[A0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[S5][0] as u32)));
                                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x7c),
                                                        c.g[A1][0],
                                                    );
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x78),
                                                        c.g[S2][0],
                                                    );
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                    c.g[S5][0] =
                                                        sx((c.g[S5][0] as u32).wrapping_add(0x4));
                                                    c.g[A1][0] = 0x1;
                                                    c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                                    c.g[V0][0] =
                                                        c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x3c),
                                                        c.g[A0][0],
                                                    );
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x8),
                                                        c.g[V0][0],
                                                    );
                                                    if t {
                                                        break 'b_1d3658;
                                                    } else {
                                                        break 'b_1d35a4;
                                                    }
                                                }
                                            }
                                            'b_1d3600: {
                                                c.cycles += 2;
                                                let t = c.g[S0][0] != c.g[V0][0];
                                                c.g[V0][0] = 0x4;
                                                if t {
                                                    break 'b_1d3600;
                                                } else {
                                                    'b_1d35c4: {
                                                        'b_1d35c0: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] =
                                                                (c.g[V1][0] < 0x4u64) as u64;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V1][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x9c));
                                                            if t {
                                                                break 'b_1d35c0;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0xa8));
                                                                if t {
                                                                    break 'b_1d35c4;
                                                                } else {
                                                                    break 'b_1d35c0;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0xac));
                                                        break 'b_1d35c4;
                                                    }
                                                    c.cycles += 15;
                                                    c.g[A0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[S6][0] as u32)));
                                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x7c),
                                                        c.g[A1][0],
                                                    );
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x78),
                                                        c.g[S2][0],
                                                    );
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[S6][0] =
                                                        sx((c.g[S6][0] as u32).wrapping_add(0x4));
                                                    c.g[A1][0] = 0x1;
                                                    c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                                    c.g[V0][0] =
                                                        c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x38),
                                                        c.g[A0][0],
                                                    );
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0xc),
                                                        c.g[V0][0],
                                                    );
                                                    if t {
                                                        break 'b_1d3658;
                                                    } else {
                                                        break 'b_1d3600;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = c.g[S0][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x5;
                                            if t {
                                                break 'b_1d3674;
                                            } else {
                                                'b_1d3620: {
                                                    'b_1d361c: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = (c.g[V1][0] < 0x4u64) as u64;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V1][0] =
                                                            sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x78));
                                                        if t {
                                                            break 'b_1d361c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0xa8));
                                                            if t {
                                                                break 'b_1d3620;
                                                            } else {
                                                                break 'b_1d361c;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.g[A1][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0xac));
                                                    break 'b_1d3620;
                                                }
                                                c.cycles += 14;
                                                c.g[A0][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[S7][0] as u32)));
                                                c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                                c.sw(
                                                    (c.g[V0][0] as u32).wrapping_add(0x7c),
                                                    c.g[A1][0],
                                                );
                                                c.sw(
                                                    (c.g[V0][0] as u32).wrapping_add(0x78),
                                                    c.g[S2][0],
                                                );
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                c.g[S7][0] =
                                                    sx((c.g[S7][0] as u32).wrapping_add(0x4));
                                                c.g[A1][0] = 0x1;
                                                c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x34),
                                                    c.g[A0][0],
                                                );
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x10),
                                                    c.g[V0][0],
                                                );
                                                break 'b_1d3658;
                                            }
                                        }
                                        c.cycles += 7;
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001d366c);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001d366c)?;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_1d36e4;
                                        } else {
                                            break 'b_1d3674;
                                        }
                                    }
                                    c.cycles += 2;
                                    if c.g[S0][0] != c.g[V0][0] {
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                        break 'b_1d36e4;
                                    } else {
                                        'b_1d3694: {
                                            'b_1d3690: {
                                                c.cycles += 3;
                                                c.g[V0][0] = (c.g[V1][0] < 0x4u64) as u64;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x84));
                                                if t {
                                                    break 'b_1d3690;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[A1][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0xa8));
                                                    if t {
                                                        break 'b_1d3694;
                                                    } else {
                                                        break 'b_1d3690;
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[A1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0xac));
                                            break 'b_1d3694;
                                        }
                                        c.cycles += 20;
                                        c.g[A0][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[FP][0] as u32)));
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x78), c.g[S2][0]);
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                        c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x4));
                                        c.g[A1][0] = 0x1;
                                        c.g[V0][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A0][0]);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001d36e0);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001d36e0)?;
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                        break 'b_1d36e4;
                                    }
                                }
                                c.cycles += 4;
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < 5i64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                                if t {
                                    break 'b_1d372c;
                                } else {
                                    c.cycles += 14;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                    c.g[V1][0] = 0xa;
                                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 1);
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    c.mult(0, 3, 4, 3);
                                    c.g[A0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5528));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    c.g[S0][0] = c.lb((c.g[V0][0] as u32));
                                    if (c.g[S0][0] as i64) >= 0 {
                                        c.g[A0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                                        if event_due(c) {
                                            deliver_event_here(c, 0x001d3478);
                                        }
                                        continue 'l_1d3478;
                                    } else {
                                        break 'b_1d372c;
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 4i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x001d33e8);
                    }
                    continue 'l_1d33e8;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001d3390);
                        }
                        continue 'l_1d3390;
                    } else {
                        c.cycles += 12;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// `cFEStatePeakRoom_OnWidgetCreated(this, widget)` (0x001d38f8): a screen widget was just created; its name hash is matched against the known names (title / button texts by `this+0xe0`, peak page widgets, event markers from the name tables at 0x4415b8..) and the widget is stored in the matching slot of `this`.
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias). Every register,
/// cycle and call of the generated body is kept.
pub fn c_fe_state_peak_room_on_widget_created(c: &mut Cpu) {
    if c.pc != 0x001d38f8 {
        return ee::s04::f_001d38f8__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x001d38f8;
        loop {
            match pc {
                0x001d38f8 => {
                    c.cycles += 19;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S0][0] = c.g[A1][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[A0][0] = 0x460000;
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e60));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[RA][0] = sx(0x001d393c);
                    c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                    call_ptr(c, 0x00317670, 0x001d393c)?;
                    let t = c.g[S2][0] != c.g[V0][0];
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        pc = 0x001d39a4;
                        continue;
                    }
                    pc = 0x001d3944;
                    continue;
                }
                0x001d3944 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe0));
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x001d3960;
                        continue;
                    }
                    pc = 0x001d3954;
                    continue;
                }
                0x001d3954 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7658));
                    if t {
                        pc = 0x001d3970;
                        continue;
                    }
                    pc = 0x001d3960;
                    continue;
                }
                0x001d3960 => {
                    c.cycles += 2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001d398c;
                        continue;
                    }
                    pc = 0x001d3968;
                    continue;
                }
                0x001d3968 => {
                    c.cycles += 2;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7668));
                    pc = 0x001d3970;
                    continue;
                }
                0x001d3970 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x001d3978);
                    call_ptr(c, 0x00317670, 0x001d3978)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001d3984);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a1000, 0x001d3984)?;
                    let t = true;
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        pc = 0x001d39a4;
                        continue;
                    }
                    pc = 0x001d398c;
                    continue;
                }
                0x001d398c => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x001d3994);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7678));
                    call_ptr(c, 0x00317670, 0x001d3994)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001d39a0);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a1000, 0x001d39a0)?;
                    c.g[A0][0] = 0x4a0000;
                    pc = 0x001d39a4;
                    continue;
                }
                0x001d39a4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d39ac);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1fa8));
                    call_ptr(c, 0x00317670, 0x001d39ac)?;

                    if c.g[S2][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        pc = 0x001d39c0;
                        continue;
                    }
                    pc = 0x001d39b4;
                    continue;
                }
                0x001d39b4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xd4), c.g[V0][0]);
                    if t {
                        pc = 0x001d3c2c;
                        continue;
                    }
                    pc = 0x001d39c0;
                    continue;
                }
                0x001d39c0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d39c8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1fb0));
                    call_ptr(c, 0x00317670, 0x001d39c8)?;

                    if c.g[S2][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001d39dc;
                        continue;
                    }
                    pc = 0x001d39d0;
                    continue;
                }
                0x001d39d0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xd8), c.g[V0][0]);
                    if t {
                        pc = 0x001d3c2c;
                        continue;
                    }
                    pc = 0x001d39dc;
                    continue;
                }
                0x001d39dc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d39e4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x488));
                    call_ptr(c, 0x00317670, 0x001d39e4)?;
                    let t = c.g[S2][0] != c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x001d3abc;
                        continue;
                    }
                    pc = 0x001d39ec;
                    continue;
                }
                0x001d39ec => {
                    c.cycles += 8;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xd0), c.g[S0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d3a04);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001d3a04)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    if t {
                        pc = 0x001d3c30;
                        continue;
                    }
                    pc = 0x001d3a0c;
                    continue;
                }
                0x001d3a0c => {
                    c.cycles += 15;
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe0));
                    c.g[A2][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x001d3a1c);
                    c.g[A0][0] = c.g[S7][0];
                    call_ptr(c, 0x00156a90, 0x001d3a1c)?;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[RA][0] = sx(0x001d3a28);
                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x00317670, 0x001d3a28)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[RA][0] = sx(0x001d3a34);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00398380, 0x001d3a34)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[A0][0] = 0xffffffffffffffff;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x7c), c.g[V0][0]);
                    let t = true;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x78), c.g[A0][0]);
                    if t {
                        pc = 0x001d3c2c;
                        continue;
                    }
                    pc = 0x001d3a48;
                    continue;
                }
                0x001d3a48 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S5][0]);
                    if t {
                        pc = 0x001d3c2c;
                        continue;
                    }
                    pc = 0x001d3a50;
                    continue;
                }
                0x001d3a50 => {
                    c.cycles += 3;
                    c.g[A1][0] = 0u64;
                    let t = true;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c), c.g[S0][0]);
                    if t {
                        pc = 0x001d3aa0;
                        continue;
                    }
                    pc = 0x001d3a5c;
                    continue;
                }
                0x001d3a5c => {
                    c.cycles += 3;
                    c.g[A1][0] = 0u64;
                    let t = true;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x58), c.g[S0][0]);
                    if t {
                        pc = 0x001d3aa0;
                        continue;
                    }
                    pc = 0x001d3a68;
                    continue;
                }
                0x001d3a68 => {
                    c.cycles += 3;
                    c.g[A1][0] = 0u64;
                    let t = true;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x80), c.g[S0][0]);
                    if t {
                        pc = 0x001d3aa0;
                        continue;
                    }
                    pc = 0x001d3a74;
                    continue;
                }
                0x001d3a74 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                    c.g[A1][0] = 0u64;
                    let t = true;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x94), c.g[S0][0]);
                    if t {
                        pc = 0x001d3aa0;
                        continue;
                    }
                    pc = 0x001d3a84;
                    continue;
                }
                0x001d3a84 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                    c.g[A1][0] = 0u64;
                    let t = true;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xa8), c.g[S0][0]);
                    if t {
                        pc = 0x001d3aa0;
                        continue;
                    }
                    pc = 0x001d3a94;
                    continue;
                }
                0x001d3a94 => {
                    c.cycles += 3;
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                    c.g[A1][0] = 0u64;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xbc), c.g[S0][0]);
                    pc = 0x001d3aa0;
                    continue;
                }
                0x001d3aa0 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d3ab4);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001d3ab4)?;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S4][0]);
                    if t {
                        pc = 0x001d3c2c;
                        continue;
                    }
                    pc = 0x001d3abc;
                    continue;
                }
                0x001d3abc => {
                    c.cycles += 11;
                    c.g[RA][0] = sx(0x001d3ac4);
                    c.g[S5][0] = 0u64;
                    call_ptr(c, 0x0014dc80, 0x001d3ac4)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001d3ad0);
                    c.g[A1][0] = 0xd;
                    call_ptr(c, 0x0014dd58, 0x001d3ad0)?;
                    c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x48));
                    c.g[S7][0] = c.g[V0][0];
                    c.g[V1][0] = 0x440000;
                    c.g[V0][0] = 0x440000;
                    c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x15e0));
                    c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x15f0));
                    pc = 0x001d3ae8;
                    continue;
                }
                0x001d3ae8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d3af0);
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32));
                    call_ptr(c, 0x00317670, 0x001d3af0)?;
                    let t = c.g[S2][0] != c.g[V0][0];
                    c.g[A0][0] = c.g[S7][0];
                    if t {
                        pc = 0x001d3b38;
                        continue;
                    }
                    pc = 0x001d3af8;
                    continue;
                }
                0x001d3af8 => {
                    c.cycles += 8;
                    c.sw((c.g[S1][0] as u32), c.g[S0][0]);
                    c.g[T0][0] = c.g[S5][0];
                    c.g[A1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x44));
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xdc));
                    c.g[RA][0] = sx(0x001d3b10);
                    c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe0));
                    call_ptr(c, 0x00157bf0, 0x001d3b10)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        if back_edge_unwinds(c, 0x001d3a0c) {
                            return Ok(());
                        }
                        pc = 0x001d3a0c;
                        continue;
                    }
                    pc = 0x001d3b18;
                    continue;
                }
                0x001d3b18 => {
                    c.cycles += 8;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d3b30);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001d3b30)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    if t {
                        pc = 0x001d3c30;
                        continue;
                    }
                    pc = 0x001d3b38;
                    continue;
                }
                0x001d3b38 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d3b40);
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32));
                    call_ptr(c, 0x00317670, 0x001d3b40)?;
                    let t = c.g[S2][0] == c.g[V0][0];
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x001d3a48) {
                            return Ok(());
                        }
                        pc = 0x001d3a48;
                        continue;
                    }
                    pc = 0x001d3b48;
                    continue;
                }
                0x001d3b48 => {
                    c.cycles += 5;
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S5][0] as i64) < 4i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x001d3ae8) {
                            return Ok(());
                        }
                        pc = 0x001d3ae8;
                        continue;
                    }
                    pc = 0x001d3b5c;
                    continue;
                }
                0x001d3b5c => {
                    c.cycles += 15;
                    c.g[V1][0] = 0x440000;
                    c.g[V0][0] = 0x440000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x15b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x15a0));
                    c.g[A3][0] = 0x440000;
                    c.g[A2][0] = 0x440000;
                    c.g[A1][0] = 0x440000;
                    c.g[A0][0] = 0x440000;
                    c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                    c.g[FP][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1558));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    c.g[S7][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1540));
                    c.g[S6][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1570));
                    c.g[S5][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1588));
                    c.g[S4][0] = 0u64;
                    pc = 0x001d3b98;
                    continue;
                }
                0x001d3b98 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[S5][0] as u32));
                    c.g[RA][0] = sx(0x001d3ba4);
                    c.g[S1][0] = sx((c.g[S4][0] as u32) << 2);
                    call_ptr(c, 0x00317670, 0x001d3ba4)?;
                    let t = c.g[S2][0] == c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                    if t {
                        if back_edge_unwinds(c, 0x001d3a50) {
                            return Ok(());
                        }
                        pc = 0x001d3a50;
                        continue;
                    }
                    pc = 0x001d3bac;
                    continue;
                }
                0x001d3bac => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d3bb4);
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32));
                    call_ptr(c, 0x00317670, 0x001d3bb4)?;
                    let t = c.g[S2][0] == c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                    if t {
                        if back_edge_unwinds(c, 0x001d3a5c) {
                            return Ok(());
                        }
                        pc = 0x001d3a5c;
                        continue;
                    }
                    pc = 0x001d3bbc;
                    continue;
                }
                0x001d3bbc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d3bc4);
                    c.g[A0][0] = c.lw((c.g[S7][0] as u32));
                    call_ptr(c, 0x00317670, 0x001d3bc4)?;
                    let t = c.g[S2][0] == c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                    if t {
                        if back_edge_unwinds(c, 0x001d3a68) {
                            return Ok(());
                        }
                        pc = 0x001d3a68;
                        continue;
                    }
                    pc = 0x001d3bcc;
                    continue;
                }
                0x001d3bcc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d3bd4);
                    c.g[A0][0] = c.lw((c.g[FP][0] as u32));
                    call_ptr(c, 0x00317670, 0x001d3bd4)?;
                    let t = c.g[S2][0] == c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x001d3a74) {
                            return Ok(());
                        }
                        pc = 0x001d3a74;
                        continue;
                    }
                    pc = 0x001d3bdc;
                    continue;
                }
                0x001d3bdc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d3be4);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x001d3be4)?;
                    let t = c.g[S2][0] == c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x001d3a84) {
                            return Ok(());
                        }
                        pc = 0x001d3a84;
                        continue;
                    }
                    pc = 0x001d3bec;
                    continue;
                }
                0x001d3bec => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d3bf4);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x001d3bf4)?;
                    let t = c.g[S2][0] == c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x001d3a94) {
                            return Ok(());
                        }
                        pc = 0x001d3a94;
                        continue;
                    }
                    pc = 0x001d3bfc;
                    continue;
                }
                0x001d3bfc => {
                    c.cycles += 12;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x4));
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V0][0] = ((c.g[S4][0] as i64) < 5i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x001d3b98) {
                            return Ok(());
                        }
                        pc = 0x001d3b98;
                        continue;
                    }
                    pc = 0x001d3c2c;
                    continue;
                }
                0x001d3c2c => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    pc = 0x001d3c30;
                    continue;
                }
                0x001d3c30 => {
                    c.cycles += 11;
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
                _ => {
                    c.jump_out(pc, 0x001d38f8, 0x001d3c5c);
                    return Ok(());
                }
            }
        }
    });
}

/// `cFEStateStats_OnWidgetCreated(this, widget)` (0x001c2f00): a screen widget was just created; its name hash is matched against the known names (stat page widgets, scroll widget, numbered `sprintf` names) and stored in the matching slot of `this`, with the widget's visibility method called.
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias). Every register,
/// cycle and call of the generated body is kept.
pub fn c_fe_state_stats_on_widget_created(c: &mut Cpu) {
    if c.pc != 0x001c2f00 {
        return ee::s03::f_001c2f00__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x001c2f00;
        loop {
            match pc {
                0x001c2f00 => {
                    c.cycles += 17;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[A0][0] = 0x460000;
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5c78));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[RA][0] = sx(0x001c2f3c);
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    call_ptr(c, 0x00317670, 0x001c2f3c)?;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001c2f58;
                        continue;
                    }
                    pc = 0x001c2f44;
                    continue;
                }
                0x001c2f44 => {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c2f50);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5c88));
                    call_ptr(c, 0x00317670, 0x001c2f50)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001c2fa0;
                        continue;
                    }
                    pc = 0x001c2f58;
                    continue;
                }
                0x001c2f58 => {
                    c.cycles += 18;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = 0xfffffffffffffffe;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.g[SP][0];
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    c.g[A0][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x58));
                    c.swc1(0, (c.g[SP][0] as u32));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001c2f98);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001c2f98)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    if t {
                        pc = 0x001c3294;
                        continue;
                    }
                    pc = 0x001c2fa0;
                    continue;
                }
                0x001c2fa0 => {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c2fac);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5c98));
                    call_ptr(c, 0x00317670, 0x001c2fac)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        pc = 0x001c2fc0;
                        continue;
                    }
                    pc = 0x001c2fb4;
                    continue;
                }
                0x001c2fb4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x001c31dc;
                        continue;
                    }
                    pc = 0x001c2fc0;
                    continue;
                }
                0x001c2fc0 => {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c2fcc);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c10));
                    call_ptr(c, 0x00317670, 0x001c2fcc)?;

                    if c.g[S0][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x001c31d8;
                        continue;
                    }
                    pc = 0x001c2fd4;
                    continue;
                }
                0x001c2fd4 => {
                    c.cycles += 6;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c2fe4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5ca8));
                    call_ptr(c, 0x00317670, 0x001c2fe4)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001c2ff8;
                        continue;
                    }
                    pc = 0x001c2fec;
                    continue;
                }
                0x001c2fec => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x001c31dc;
                        continue;
                    }
                    pc = 0x001c2ff8;
                    continue;
                }
                0x001c2ff8 => {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c3004);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5cb8));
                    call_ptr(c, 0x00317670, 0x001c3004)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001c3018;
                        continue;
                    }
                    pc = 0x001c300c;
                    continue;
                }
                0x001c300c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x001c31dc;
                        continue;
                    }
                    pc = 0x001c3018;
                    continue;
                }
                0x001c3018 => {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c3024);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5cc8));
                    call_ptr(c, 0x00317670, 0x001c3024)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001c3038;
                        continue;
                    }
                    pc = 0x001c302c;
                    continue;
                }
                0x001c302c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x001c31dc;
                        continue;
                    }
                    pc = 0x001c3038;
                    continue;
                }
                0x001c3038 => {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c3044);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5ce0));
                    call_ptr(c, 0x00317670, 0x001c3044)?;

                    if c.g[S0][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x001c31d8;
                        continue;
                    }
                    pc = 0x001c304c;
                    continue;
                }
                0x001c304c => {
                    c.cycles += 6;
                    c.g[A0][0] = 0x4a0000;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c305c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c18));
                    call_ptr(c, 0x00317670, 0x001c305c)?;

                    if c.g[S0][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x001c31d8;
                        continue;
                    }
                    pc = 0x001c3064;
                    continue;
                }
                0x001c3064 => {
                    c.cycles += 6;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c3074);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5cf0));
                    call_ptr(c, 0x00317670, 0x001c3074)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001c3090;
                        continue;
                    }
                    pc = 0x001c307c;
                    continue;
                }
                0x001c307c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                    if t {
                        pc = 0x001c31e4;
                        continue;
                    }
                    pc = 0x001c3090;
                    continue;
                }
                0x001c3090 => {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c309c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d00));
                    call_ptr(c, 0x00317670, 0x001c309c)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001c30b8;
                        continue;
                    }
                    pc = 0x001c30a4;
                    continue;
                }
                0x001c30a4 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                    if t {
                        pc = 0x001c31e4;
                        continue;
                    }
                    pc = 0x001c30b8;
                    continue;
                }
                0x001c30b8 => {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c30c4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d10));
                    call_ptr(c, 0x00317670, 0x001c30c4)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        pc = 0x001c30e0;
                        continue;
                    }
                    pc = 0x001c30cc;
                    continue;
                }
                0x001c30cc => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                    if t {
                        pc = 0x001c31e4;
                        continue;
                    }
                    pc = 0x001c30e0;
                    continue;
                }
                0x001c30e0 => {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c30ec);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1bb8));
                    call_ptr(c, 0x00317670, 0x001c30ec)?;

                    if c.g[S0][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x001c31d8;
                        continue;
                    }
                    pc = 0x001c30f4;
                    continue;
                }
                0x001c30f4 => {
                    c.cycles += 6;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c3104);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d20));
                    call_ptr(c, 0x00317670, 0x001c3104)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001c313c;
                        continue;
                    }
                    pc = 0x001c310c;
                    continue;
                }
                0x001c310c => {
                    c.cycles += 12;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = 0xfffffffffffffffe;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    c.g[A0][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001c3134);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001c3134)?;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x6d0), c.g[S1][0]);
                    if t {
                        pc = 0x001c3290;
                        continue;
                    }
                    pc = 0x001c313c;
                    continue;
                }
                0x001c313c => {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c3148);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d30));
                    call_ptr(c, 0x00317670, 0x001c3148)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001c3158;
                        continue;
                    }
                    pc = 0x001c3150;
                    continue;
                }
                0x001c3150 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x6d4), c.g[S1][0]);
                    if t {
                        pc = 0x001c3170;
                        continue;
                    }
                    pc = 0x001c3158;
                    continue;
                }
                0x001c3158 => {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c3164);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d40));
                    call_ptr(c, 0x00317670, 0x001c3164)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001c3180;
                        continue;
                    }
                    pc = 0x001c316c;
                    continue;
                }
                0x001c316c => {
                    c.cycles += 1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x6d8), c.g[S1][0]);
                    pc = 0x001c3170;
                    continue;
                }
                0x001c3170 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x90));
                    c.g[V0][0] = c.g[V0][0] | 0x8;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x90), c.g[V0][0]);
                    if t {
                        pc = 0x001c3290;
                        continue;
                    }
                    pc = 0x001c3180;
                    continue;
                }
                0x001c3180 => {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c318c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d50));
                    call_ptr(c, 0x00317670, 0x001c318c)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[S3][0] = 0u64;
                    if t {
                        pc = 0x001c31f4;
                        continue;
                    }
                    pc = 0x001c3194;
                    continue;
                }
                0x001c3194 => {
                    c.cycles += 16;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001c31a0);
                    c.g[A1][0] = 0xa;
                    call_ptr(c, 0x0039b760, 0x001c31a0)?;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x6dc), c.g[S1][0]);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001c31b0);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0039ab00, 0x001c31b0)?;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6dc));
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c31cc);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001c31cc)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    if t {
                        pc = 0x001c3294;
                        continue;
                    }
                    pc = 0x001c31d4;
                    continue;
                }
                0x001c31d4 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    pc = 0x001c31d8;
                    continue;
                }
                0x001c31d8 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0x1;
                    pc = 0x001c31dc;
                    continue;
                }
                0x001c31dc => {
                    c.cycles += 2;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    pc = 0x001c31e4;
                    continue;
                }
                0x001c31e4 => {
                    c.cycles += 4;
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c31ec);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001c31ec)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    if t {
                        pc = 0x001c3294;
                        continue;
                    }
                    pc = 0x001c31f4;
                    continue;
                }
                0x001c31f4 => {
                    c.cycles += 4;
                    c.g[S6][0] = 0x460000;
                    c.g[S5][0] = 0x460000;
                    let t = true;
                    c.g[S4][0] = 0x460000;
                    if t {
                        pc = 0x001c3248;
                        continue;
                    }
                    pc = 0x001c3204;
                    continue;
                }
                0x001c3204 => {
                    c.cycles += 1;
                    pc = 0x001c3208;
                    continue;
                }
                0x001c3208 => {
                    c.cycles += 8;
                    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5d70));
                    c.g[RA][0] = sx(0x001c3214);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x001c3214)?;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c3220);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x00317670, 0x001c3220)?;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5d80));
                    if t {
                        if back_edge_unwinds(c, 0x001c31d4) {
                            return Ok(());
                        }
                        pc = 0x001c31d4;
                        continue;
                    }
                    pc = 0x001c3228;
                    continue;
                }
                0x001c3228 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001c3234);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x001c3234)?;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c3240);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x00317670, 0x001c3240)?;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x001c31d4) {
                            return Ok(());
                        }
                        pc = 0x001c31d4;
                        continue;
                    }
                    pc = 0x001c3248;
                    continue;
                }
                0x001c3248 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < 10i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        pc = 0x001c3290;
                        continue;
                    }
                    pc = 0x001c3254;
                    continue;
                }
                0x001c3254 => {
                    c.cycles += 9;
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5d60));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001c3264);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x001c3264)?;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001c3270);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x00317670, 0x001c3270)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = c.g[S2][0];
                        if back_edge_unwinds(c, 0x001c3208) {
                            return Ok(());
                        }
                        pc = 0x001c3208;
                        continue;
                    }
                    pc = 0x001c3278;
                    continue;
                }
                0x001c3278 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c3290);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001c3290)?;
                    pc = 0x001c3290;
                    continue;
                }
                0x001c3290 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    pc = 0x001c3294;
                    continue;
                }
                0x001c3294 => {
                    c.cycles += 9;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001c2f00, 0x001c32b8);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check_lohi, elf_image, fresh, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The fixture's control words: the gate of the fakes (`OC936_MAGIC`), the
    /// per-callee call counters and the result table the fakes answer from.
    pub(crate) const OC936_CTRL: u32 = DATA + 0x28_0000;
    pub(crate) const OC936_GATE: u32 = OC936_CTRL;
    pub(crate) const OC936_MAGIC: u32 = 0x4f43_3936;
    pub(crate) const OC936_SEEN: u32 = OC936_CTRL + 0x100;
    pub(crate) const OC936_TAB: u32 = OC936_CTRL + 0x400;
    /// The screen state `this`, the widget under test, the widget vtable and
    /// eight widget objects the state's slots point at.
    pub(crate) const OC936_THIS: u32 = DATA + 0x1000;
    pub(crate) const OC936_WIDGET: u32 = DATA + 0x2000;
    pub(crate) const OC936_VT: u32 = DATA + 0x3000;
    pub(crate) const OC936_WIDGETS: u32 = DATA + 0x4000;
    pub(crate) const OC936_OBJ: u32 = DATA + 0x6000;

    /// A fake callee: logs like the plain fake; `StrHash` answers its
    /// argument (so a widget whose hash is a name's address matches that
    /// name); the BE reward queries and the goal check answer from the
    /// fixture's result table (zero included).
    pub(crate) fn oc936_hook(c: &mut Cpu, target: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let slot = OC936_SEEN + 4 * ((target >> 4) & 0xff);
        let k = c.bus.read32(slot);
        c.bus.write32(slot, k + 1);
        fakes::fake_call(c, target);
        if c.pc != ra {
            return;
        }
        match target {
            0x0031_7670 => c.g[V0][0] = a0,
            0x0015_8910 | 0x0015_89b0 | 0x0015_8a50 | 0x0015_7bf0 => {
                let v = c.bus.read32(OC936_TAB + 4 * (k & 15));
                c.g[V0][0] = v as u64;
            }
            _ => {}
        }
    }

    macro_rules! oc936_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC936_GATE) != OC936_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc936_hook(c, $addr);
                }
            )*
            pub(crate) fn oc936_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc936_fakes!(
        (
            oc936_fk_317670,
            OC936_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc936_fk_398380,
            OC936_PREV_398380,
            ee::s13::OVERRIDE_00398380,
            ee::s13::f_00398380__generated,
            0x0039_8380
        ),
        (
            oc936_fk_3a1000,
            OC936_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc936_fk_416210,
            OC936_PREV_416210,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x0041_6210
        ),
        (
            oc936_fk_14dc80,
            OC936_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc936_fk_14dd58,
            OC936_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc936_fk_156a90,
            OC936_PREV_156A90,
            ee::s01::OVERRIDE_00156A90,
            ee::s01::f_00156a90__generated,
            0x0015_6a90
        ),
        (
            oc936_fk_157bf0,
            OC936_PREV_157BF0,
            ee::s01::OVERRIDE_00157BF0,
            ee::s01::f_00157bf0__generated,
            0x0015_7bf0
        ),
        (
            oc936_fk_158910,
            OC936_PREV_158910,
            ee::s01::OVERRIDE_00158910,
            ee::s01::f_00158910__generated,
            0x0015_8910
        ),
        (
            oc936_fk_1589b0,
            OC936_PREV_1589B0,
            ee::s01::OVERRIDE_001589B0,
            ee::s01::f_001589b0__generated,
            0x0015_89b0
        ),
        (
            oc936_fk_158a50,
            OC936_PREV_158A50,
            ee::s01::OVERRIDE_00158A50,
            ee::s01::f_00158a50__generated,
            0x0015_8a50
        ),
        (
            oc936_fk_39ab00,
            OC936_PREV_39AB00,
            ee::s13::OVERRIDE_0039AB00,
            ee::s13::f_0039ab00__generated,
            0x0039_ab00
        ),
        (
            oc936_fk_39b760,
            OC936_PREV_39B760,
            ee::s13::OVERRIDE_0039B760,
            ee::s13::f_0039b760__generated,
            0x0039_b760
        ),
        (
            oc936_fk_417828,
            OC936_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
    );

    /// The four functions: entry, generated body, hand port.
    fn oc936_kind(kind: u32) -> (u32, GuestFn, GuestFn) {
        match kind {
            0 => (
                0x001d_2ad0,
                ee::s04::f_001d2ad0__generated,
                c_fe_state_mountain_room_on_widget_created,
            ),
            1 => (
                0x001d_3340,
                ee::s04::f_001d3340__generated,
                c_fe_state_mountain_room_update_event_icons,
            ),
            2 => (
                0x001d_38f8,
                ee::s04::f_001d38f8__generated,
                c_fe_state_peak_room_on_widget_created,
            ),
            _ => (
                0x001c_2f00,
                ee::s03::f_001c2f00__generated,
                c_fe_state_stats_on_widget_created,
            ),
        }
    }

    /// Fixture: the game image (name tables, medal table), the gated fakes,
    /// a screen state full of widget pointers, the widget under test with
    /// name hash `hash`, and a result table for the BE queries. The fields
    /// come from their own generator, so a dry run (`unwind` off) and the
    /// real run see the same screen.
    fn oc936_fixture(c: &mut Cpu, seed: u32, kind: u32, hash: u32, unwind: bool) {
        let _ = machine(
            c,
            seed,
            0x0936_0000 + kind as u64,
            if unwind { 40 } else { 0 },
        );
        elf_image(c);
        oc936_install_fakes();
        let mut r = Rng(0x9360_0001 ^ (seed as u64) << 8 ^ (kind as u64) << 40);
        c.bus.write32(OC936_GATE, OC936_MAGIC);
        for i in 0..16u32 {
            let v = match r.next_u32() % 6 {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => 5,
                _ => 0x0001_0000,
            };
            c.bus.write32(OC936_TAB + 4 * i, v);
        }
        // Widget vtable: halfword delta + method at every 8 bytes.
        for off in (0x20..0x80).step_by(8) {
            c.bus.write16(OC936_VT + off, 0);
            c.bus
                .write32(OC936_VT + off + 4, fakes::FAKE + 0x10 * (off / 8));
        }
        for i in 0..8u32 {
            c.bus.write32(OC936_WIDGETS + 0x100 * i + 8, OC936_VT);
        }
        c.bus.write32(OC936_WIDGET + 8, OC936_VT);
        c.bus.write32(OC936_WIDGET + 0x38, hash);
        c.bus.write32(OC936_WIDGET + 0x14, r.next_u32());
        c.bus.write32(OC936_WIDGET + 0x90, r.next_u32());
        c.bus.write32(OC936_WIDGET + 0x7c, r.next_u32());
        for w in (0..0x800).step_by(4) {
            let p = OC936_WIDGETS + 0x100 * (r.next_u32() % 8);
            c.bus.write32(OC936_THIS + w, p);
        }
        c.bus.write32(OC936_THIS + 0x10, OC936_OBJ);
        c.bus.write8(OC936_THIS + 0x44, (r.next_u32() % 2) as u8);
        c.bus.write8(OC936_THIS + 0xb8, (r.next_u32() % 4) as u8);
        c.bus.write32(OC936_THIS + 0xe0, r.next_u32() % 4);
        c.bus.write32(OC936_THIS + 0xdc, r.next_u32() % 4);
        c.bus.write32(OC936_THIS + 0xa8, r.next_u32());
        c.bus.write32(OC936_THIS + 0xac, r.next_u32());
        c.pc = oc936_kind(kind).0;
        c.g[A0][0] = sx(OC936_THIS);
        c.g[A1][0] = sx(OC936_WIDGET);
    }

    /// The name addresses the generated body hashes when nothing matches (the
    /// `a0` of each `StrHash` call), in call order.
    fn oc936_names(kind: u32, seed: u32) -> Vec<u32> {
        let mut c = fresh();
        oc936_fixture(&mut c, seed, kind, 0, false);
        (oc936_kind(kind).1)(&mut c);
        let n = c.bus.read32(fakes::LOG_COUNT);
        let mut out = Vec::new();
        for i in 0..n {
            let p = fakes::LOG_START + 0x220 * i;
            if c.bus.read32(p) == 0x0031_7670 {
                out.push(c.bus.read32(p + 0x50));
            }
        }
        out
    }

    /// Widget name = the `seed`-th name hashed (every name is reached), or
    /// none at all for some seeds.
    fn oc936_pick(kind: u32, seed: u32) -> u32 {
        let names = oc936_names(kind, seed);
        if names.is_empty() || seed % 11 == 10 {
            0
        } else {
            names[(seed as usize) % names.len()]
        }
    }

    fn oc936_check(kind: u32, seeds: u32) {
        let (_, generated, hand) = oc936_kind(kind);
        check_lohi(generated, hand, seeds, &|c, seed| {
            let h = oc936_pick(kind, seed);
            oc936_fixture(c, seed, kind, h, true)
        });
    }

    /// The fixture reaches at least `min` distinct paths (cycle totals of the
    /// generated body, no unwinds).
    fn oc936_paths(kind: u32, seeds: u32, min: usize) {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            let h = oc936_pick(kind, seed);
            oc936_fixture(&mut c, seed, kind, h, false);
            let before = c.cycles;
            (oc936_kind(kind).1)(&mut c);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= min, "paths {} {seen:?}", seen.len());
    }

    /// `cFEStateMountainRoom_OnWidgetCreated`: each known widget name, an
    /// unknown one, and the unwinds of its calls.
    #[test]
    fn c_fe_state_mountain_room_on_widget_created_matches() {
        oc936_check(0, 200);
    }

    #[test]
    fn c_fe_state_mountain_room_on_widget_created_paths() {
        oc936_paths(0, 200, 25);
    }

    /// `cFEStateMountainRoom_UpdateEventIcons`: the medal table walk with
    /// reward results below and above the icon threshold, and unwinds.
    #[test]
    fn c_fe_state_mountain_room_update_event_icons_matches() {
        oc936_check(1, 120);
    }

    #[test]
    fn c_fe_state_mountain_room_update_event_icons_paths() {
        oc936_paths(1, 120, 5);
    }

    /// `cFEStatePeakRoom_OnWidgetCreated`: each known widget name (both
    /// `this+0xe0` states), the marker name tables, the goal check results.
    #[test]
    fn c_fe_state_peak_room_on_widget_created_matches() {
        oc936_check(2, 240);
    }

    #[test]
    fn c_fe_state_peak_room_on_widget_created_paths() {
        oc936_paths(2, 240, 15);
    }

    /// `cFEStateStats_OnWidgetCreated`: each known widget name including the
    /// `sprintf` names, and unwinds.
    #[test]
    fn c_fe_state_stats_on_widget_created_matches() {
        oc936_check(3, 240);
    }

    #[test]
    fn c_fe_state_stats_on_widget_created_paths() {
        oc936_paths(3, 240, 19);
    }
}
