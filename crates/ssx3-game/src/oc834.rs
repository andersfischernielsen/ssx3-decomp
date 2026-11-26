//! Staging module oc834 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc834`.

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
        0x0018_8c68,
        cfe_state_options_on_widget_notify,
        "cFEStateOptions_OnWidgetNotify",
    ),
    (
        0x0018_8d08,
        cfe_state_options_on_child_event,
        "cFEStateOptions_OnChildEvent",
    ),
    (
        0x0018_95f0,
        cfe_state_options_game_on_activate,
        "cFEStateOptionsGame_OnActivate",
    ),
    (
        0x0018_9610,
        cfe_state_options_game_on_child_event,
        "cFEStateOptionsGame_OnChildEvent",
    ),
    (
        0x0018_9980,
        cfe_state_options_game_on_widget_action,
        "cFEStateOptionsGame_OnWidgetAction",
    ),
    (
        0x0018_9d60,
        cfe_state_options_game_refresh_list_boxes,
        "cFEStateOptionsGame_RefreshListBoxes",
    ),
];

/// `int cFEStateOptions_OnWidgetNotify(self, menu, key, index)` (0x188c68): the
/// options screen's answer to a widget notification. Keys 8 and 9 answer 0x100;
/// key 6 (when the list item at `index` carries the hash of the screen's name
/// string `0x4a1530`) and key 7 answer 1, or 0x101 when the child flag at
/// `+0x4c` is set; every other case answers 0x101.
pub fn cfe_state_options_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        'b_188cec: {
            'b_188ca4: {
                'b_188ca0: {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = c.g[A2][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = c.g[A0][0];
                    c.g[V0][0] = (c.g[S1][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        break 'b_188ca0;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[S1][0] < 0x8u64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0x6;
                        if t {
                            break 'b_188ca4;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0x100;
                            if t {
                                break 'b_188cec;
                            } else {
                                break 'b_188ca0;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0x6;
                break 'b_188ca4;
            }
            'b_188ce0: {
                'b_188cd4: {
                    c.cycles += 2;
                    let t = c.g[S1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x7;
                    if t {
                        break 'b_188cd4;
                    } else {
                        c.cycles += 9;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x74));
                        c.g[A1][0] = c.g[A3][0];
                        call_ptr(c, 0x00397870, 0x00188cb8)?;
                        c.g[A0][0] = 0x4a0000;
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x38));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1530));
                        call_ptr(c, 0x00317670, 0x00188cc8)?;
                        if c.g[S0][0] == c.g[V0][0] {
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                            break 'b_188ce0;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = 0x7;
                            break 'b_188cd4;
                        }
                    }
                }
                c.cycles += 2;
                if c.g[S1][0] != c.g[V0][0] {
                    c.g[V0][0] = 0x101;
                    break 'b_188cec;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                    break 'b_188ce0;
                }
            }
            c.cycles += 3;
            c.g[V1][0] = 0x101;
            c.g[V0][0] = 0x1;
            if c.g[A0][0] != 0 {
                c.g[V0][0] = c.g[V1][0];
            }
            break 'b_188cec;
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    })
}

/// `cFEStateOptions_OnChildEvent(self, popup, event)` (0x188d08): the reset
/// popup's events. 0x15 sets the popup text; 0x16 (when the popup's name is the
/// reset popup) forwards the answer to the options widget (`onWidgetEvent`
/// 5 or 6) and sets the save-options flags at `+0x48`/`+0x4c`.
pub fn cfe_state_options_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_188e44: {
            'b_188e40: {
                'b_188d88: {
                    'b_188d40: {
                        c.cycles += 9;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                        c.g[V0][0] = 0x15;
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[S1][0] = c.g[A0][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[S2][0] = c.g[A1][0];
                        let t = c.g[A2][0] == c.g[V0][0];
                        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                        if t {
                            break 'b_188d40;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x16;
                            let t = c.g[A2][0] == c.g[V0][0];
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_188d88;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_188e44;
                                } else {
                                    break 'b_188d40;
                                }
                            }
                        }
                    }
                    c.cycles += 10;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x22c0));
                    call_ptr(c, 0x00317670, 0x00188d50)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00188d5c)?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        break 'b_188e40;
                    } else {
                        c.cycles += 8;
                        c.g[A0][0] = 0x460000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x22b0));
                        call_ptr(c, 0x00317670, 0x00188d74)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x00188d80)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_188e40;
                        } else {
                            break 'b_188d88;
                        }
                    }
                }
                c.cycles += 6;
                c.g[A0][0] = 0x460000;
                c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x22e0));
                call_ptr(c, 0x00317670, 0x00188d98)?;
                let t = c.g[S0][0] != c.g[V0][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_188e40;
                } else {
                    'b_188e3c: {
                        'b_188e10: {
                            c.cycles += 15;
                            c.g[A0][0] = 0x4a0000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1530));
                            call_ptr(c, 0x00317670, 0x00188dac)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00188db8)?;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xb8));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xbc));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00188dd4)?;
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_188e10;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S0][0] == 0u64;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_188e3c;
                                } else {
                                    c.cycles += 11;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                                    c.g[A1][0] = c.g[S0][0];
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                                    c.g[A2][0] = 0x5;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x98));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00188e08)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_188e40;
                                    } else {
                                        break 'b_188e10;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[S0][0] == 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_188e3c;
                        } else {
                            c.cycles += 9;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x48), 0u64);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                            c.g[A1][0] = c.g[S0][0];
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A2][0] = 0x6;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00188e3c)?;
                            break 'b_188e3c;
                        }
                    }
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    break 'b_188e40;
                }
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_188e44;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    })
}

/// `cFEStateOptionsGame_OnActivate(self)` (0x1895f0): the base screen's
/// activation (0x39e4c0).
pub fn cfe_state_options_game_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        call_ptr(c, 0x0039e4c0, 0x00189600)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    })
}

/// `cFEStateOptionsGame_OnChildEvent(self, popup, event)` (0x189610): the
/// popups of the game options screen. 0x16 applies the reset-defaults, screen
/// position (restore or store the offsets) and video calibration answers, sets
/// the options-changed flag at `gp-0x1c20`, refreshes the list boxes and calls
/// the option interface's slot 1.
pub fn cfe_state_options_game_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_18993c: {
            'b_189938: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x570));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x560));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x550));
                c.g[S0][0] = c.g[A2][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x530));
                c.g[S1][0] = c.g[A1][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x540));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x520), c.g[RA][0]);
                c.g[S3][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x00189638)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x4;
                call_ptr(c, 0x0014dd58, 0x00189644)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = 0x15;
                let t = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = 0x16;
                if t {
                    break 'b_189938;
                } else {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x560));
                    if t {
                        break 'b_18993c;
                    } else {
                        'b_189928: {
                            'b_1896c8: {
                                c.cycles += 6;
                                c.g[A0][0] = 0x460000;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x22e0));
                                call_ptr(c, 0x00317670, 0x0018966c)?;
                                if c.g[S0][0] != c.g[V0][0] {
                                    c.g[A0][0] = 0x460000;
                                    break 'b_1896c8;
                                } else {
                                    c.cycles += 8;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0x2;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xb8));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xbc));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0018968c)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                        break 'b_189928;
                                    } else {
                                        c.cycles += 13;
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x0014f4f8, 0x0018969c)?;
                                        c.g[V0][0] = 0x1;
                                        c.g[A0][0] = c.g[S3][0];
                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1c20), c.g[V0][0]);
                                        Callee {
                                            addr: 0x00189d60,
                                            f: ee::s02::f_00189d60,
                                        }
                                        .call(c, 0x001896ac)?;
                                        call_ptr(c, 0x0028b180, 0x001896b4)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x1;
                                        call_ptr(c, 0x0029ed90, 0x001896c0)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                        if t {
                                            break 'b_189928;
                                        } else {
                                            break 'b_1896c8;
                                        }
                                    }
                                }
                            }
                            'b_189924: {
                                'b_1898e0: {
                                    c.cycles += 5;
                                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2228));
                                    call_ptr(c, 0x00317670, 0x001896d4)?;
                                    if c.g[S0][0] != c.g[V0][0] {
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                        break 'b_1898e0;
                                    } else {
                                        'b_189748: {
                                            c.cycles += 8;
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = 0x2;
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0xb8));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xbc));
                                            let t = c.g[V0][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x001896f4)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0u64;
                                            if t {
                                                break 'b_189748;
                                            } else {
                                                c.cycles += 19;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0xb8));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xbc));
                                                let t = c.g[V0][0] as u32;
                                                c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00189710)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                c.g[S0][0] = c.g[V0][0];
                                                c.g[A1][0] = 0x1;
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0xb8));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xbc));
                                                let t = c.g[V0][0] as u32;
                                                c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x0018972c)?;
                                                c.g[V1][0] = 0x530000;
                                                c.g[A0][0] = 0x1;
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                                                c.sw(
                                                    (c.g[GP][0] as u32).wrapping_sub(0x1c20),
                                                    c.g[A0][0],
                                                );
                                                c.sb(
                                                    (c.g[V1][0] as u32).wrapping_add(0x5),
                                                    c.g[V0][0],
                                                );
                                                let t = true;
                                                c.sb(
                                                    (c.g[V1][0] as u32).wrapping_add(0x6),
                                                    c.g[S0][0],
                                                );
                                                if t {
                                                    break 'b_189924;
                                                } else {
                                                    break 'b_189748;
                                                }
                                            }
                                        }
                                        'b_189804: {
                                            'b_1897d0: {
                                                c.cycles += 11;
                                                c.g[A1][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                                c.g[V0][0] = 0x530000;
                                                c.g[A3][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x5610));
                                                c.g[T2][0] = c.g[V0][0];
                                                c.g[V1][0] =
                                                    c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                                                c.g[A2][0] = c.g[SP][0];
                                                c.g[A0][0] = c.g[A3][0] & 0x7;
                                                c.g[V0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x60));
                                                c.g[T1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x64));
                                                let t = c.g[A0][0] == 0u64;
                                                c.g[T0][0] = sx((c.g[A1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                if t {
                                                    break 'b_1897d0;
                                                } else {
                                                    c.cycles += 2;
                                                    c.g[V1][0] =
                                                        sx((c.g[A3][0] as u32).wrapping_add(0x280));
                                                    c.g[V0][0] =
                                                        sx((c.g[SP][0] as u32).wrapping_add(0x290));
                                                    'l_18977c: loop {
                                                        c.cycles += 19;
                                                        c.ldl(
                                                            4,
                                                            (c.g[A3][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.ldr(4, c.g[A3][0] as u32);
                                                        c.ldl(
                                                            5,
                                                            (c.g[A3][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.ldr(
                                                            5,
                                                            (c.g[A3][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.ldl(
                                                            11,
                                                            (c.g[A3][0] as u32).wrapping_add(0x17),
                                                        );
                                                        c.ldr(
                                                            11,
                                                            (c.g[A3][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.ldl(
                                                            12,
                                                            (c.g[A3][0] as u32).wrapping_add(0x1f),
                                                        );
                                                        c.ldr(
                                                            12,
                                                            (c.g[A3][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.sdl(
                                                            4,
                                                            (c.g[A2][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.sdr(4, c.g[A2][0] as u32);
                                                        c.sdl(
                                                            5,
                                                            (c.g[A2][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.sdr(
                                                            5,
                                                            (c.g[A2][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.sdl(
                                                            11,
                                                            (c.g[A2][0] as u32).wrapping_add(0x17),
                                                        );
                                                        c.sdr(
                                                            11,
                                                            (c.g[A2][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.sdl(
                                                            12,
                                                            (c.g[A2][0] as u32).wrapping_add(0x1f),
                                                        );
                                                        c.sdr(
                                                            12,
                                                            (c.g[A2][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.g[A3][0] =
                                                            sx((c.g[A3][0] as u32)
                                                                .wrapping_add(0x20));
                                                        let t = c.g[A3][0] != c.g[V1][0];
                                                        c.g[A2][0] =
                                                            sx((c.g[A2][0] as u32)
                                                                .wrapping_add(0x20));
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x0018977c);
                                                            }
                                                            continue 'l_18977c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;

                                                            if t {
                                                                break 'b_189804;
                                                            } else {
                                                                break 'b_1897d0;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.g[V1][0] =
                                                sx((c.g[A3][0] as u32).wrapping_add(0x280));
                                            c.g[V0][0] =
                                                sx((c.g[SP][0] as u32).wrapping_add(0x290));
                                            'l_1897d8: loop {
                                                c.cycles += 11;
                                                c.g[A0][0] = c.ld(c.g[A3][0] as u32);
                                                c.g[A1][0] =
                                                    c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                                                c.g[T3][0] =
                                                    c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                                                c.g[T4][0] =
                                                    c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                                                c.sd(c.g[A2][0] as u32, c.g[A0][0]);
                                                c.sd(
                                                    (c.g[A2][0] as u32).wrapping_add(0x8),
                                                    c.g[A1][0],
                                                );
                                                c.sd(
                                                    (c.g[A2][0] as u32).wrapping_add(0x10),
                                                    c.g[T3][0],
                                                );
                                                c.sd(
                                                    (c.g[A2][0] as u32).wrapping_add(0x18),
                                                    c.g[T4][0],
                                                );
                                                c.g[A3][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x20));
                                                let t = c.g[A3][0] != c.g[V1][0];
                                                c.g[A2][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x001897d8);
                                                    }
                                                    continue 'l_1897d8;
                                                } else {
                                                    break 'b_189804;
                                                }
                                            }
                                        }
                                        'b_1898b8: {
                                            'b_189888: {
                                                c.cycles += 11;
                                                c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                                                c.ldr(3, c.g[A3][0] as u32);
                                                c.sdl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                                                c.sdr(3, c.g[A2][0] as u32);
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[A2][0] =
                                                    sx((c.g[T2][0] as u32).wrapping_add(0x5610));
                                                c.g[V0][0] =
                                                    c.lbu((c.g[SP][0] as u32).wrapping_add(0x6));
                                                c.g[V1][0] = c.g[A2][0] & 0x7;
                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                                                let t = c.g[V1][0] == 0u64;
                                                c.g[A1][0] =
                                                    sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                                                if t {
                                                    break 'b_189888;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V1][0] =
                                                        sx((c.g[A2][0] as u32).wrapping_add(0x280));
                                                    'l_189834: loop {
                                                        c.cycles += 19;
                                                        c.ldl(
                                                            12,
                                                            (c.g[A2][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.ldr(12, c.g[A2][0] as u32);
                                                        c.ldl(
                                                            2,
                                                            (c.g[A2][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.ldr(
                                                            2,
                                                            (c.g[A2][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.ldl(
                                                            7,
                                                            (c.g[A2][0] as u32).wrapping_add(0x17),
                                                        );
                                                        c.ldr(
                                                            7,
                                                            (c.g[A2][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.ldl(
                                                            10,
                                                            (c.g[A2][0] as u32).wrapping_add(0x1f),
                                                        );
                                                        c.ldr(
                                                            10,
                                                            (c.g[A2][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.sdl(
                                                            12,
                                                            (c.g[A0][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.sdr(12, c.g[A0][0] as u32);
                                                        c.sdl(
                                                            2,
                                                            (c.g[A0][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.sdr(
                                                            2,
                                                            (c.g[A0][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.sdl(
                                                            7,
                                                            (c.g[A0][0] as u32).wrapping_add(0x17),
                                                        );
                                                        c.sdr(
                                                            7,
                                                            (c.g[A0][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.sdl(
                                                            10,
                                                            (c.g[A0][0] as u32).wrapping_add(0x1f),
                                                        );
                                                        c.sdr(
                                                            10,
                                                            (c.g[A0][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.g[A2][0] =
                                                            sx((c.g[A2][0] as u32)
                                                                .wrapping_add(0x20));
                                                        let t = c.g[A2][0] != c.g[V1][0];
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x20));
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x00189834);
                                                            }
                                                            continue 'l_189834;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;

                                                            if t {
                                                                break 'b_1898b8;
                                                            } else {
                                                                break 'b_189888;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[V1][0] =
                                                sx((c.g[A2][0] as u32).wrapping_add(0x280));
                                            'l_18988c: loop {
                                                c.cycles += 11;
                                                c.g[T3][0] = c.ld(c.g[A2][0] as u32);
                                                c.g[T4][0] =
                                                    c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                                                c.g[V0][0] =
                                                    c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                                                c.g[A3][0] =
                                                    c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                                                c.sd(c.g[A0][0] as u32, c.g[T3][0]);
                                                c.sd(
                                                    (c.g[A0][0] as u32).wrapping_add(0x8),
                                                    c.g[T4][0],
                                                );
                                                c.sd(
                                                    (c.g[A0][0] as u32).wrapping_add(0x10),
                                                    c.g[V0][0],
                                                );
                                                c.sd(
                                                    (c.g[A0][0] as u32).wrapping_add(0x18),
                                                    c.g[A3][0],
                                                );
                                                c.g[A2][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_add(0x20));
                                                let t = c.g[A2][0] != c.g[V1][0];
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x0018988c);
                                                    }
                                                    continue 'l_18988c;
                                                } else {
                                                    break 'b_1898b8;
                                                }
                                            }
                                        }
                                        c.cycles += 10;
                                        c.ldl(10, (c.g[A2][0] as u32).wrapping_add(0x7));
                                        c.ldr(10, c.g[A2][0] as u32);
                                        c.sdl(10, (c.g[A0][0] as u32).wrapping_add(0x7));
                                        c.sdr(10, c.g[A0][0] as u32);
                                        c.g[V0][0] = c.lw(c.g[T1][0] as u32);
                                        c.g[A0][0] = c.g[T0][0];
                                        let t = c.g[V0][0] as u32;
                                        c.g[A2][0] = c.lb((c.g[SP][0] as u32).wrapping_add(0x295));
                                        call_ptr(c, t, 0x001898d8)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                        if t {
                                            break 'b_189928;
                                        } else {
                                            break 'b_1898e0;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0x1b;
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                    break 'b_189928;
                                } else {
                                    c.cycles += 8;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0x2;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xb8));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xbc));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00189904)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                        break 'b_189928;
                                    } else {
                                        c.cycles += 6;
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x0014f4f8, 0x00189914)?;
                                        c.g[V0][0] = 0x1;
                                        c.g[A0][0] = c.g[S3][0];
                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1c20), c.g[V0][0]);
                                        Callee {
                                            addr: 0x00189d60,
                                            f: ee::s02::f_00189d60,
                                        }
                                        .call(c, 0x00189924)?;
                                        break 'b_189924;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                            break 'b_189928;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x00189938)?;
                        break 'b_189938;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x560));
            break 'b_18993c;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x550));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x540));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x530));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x520));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x570));
        c.pc = t;
        return Ok(());
    })
}

/// `cFEStateOptionsGame_OnWidgetAction(self, widget, event)` (0x189980):
/// widget events of the game options screen: 7 and 5 push the reset, screen
/// position and calibration popups; 6 activates the options, refreshes and
/// pushes the next state; 9 stores the language, metric units and widescreen
/// choices. A null widget returns at once.
pub fn cfe_state_options_game_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_189d4c: {
            'b_189d48: {
                'b_189d44: {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.g[A2][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S3][0] = c.g[A1][0];
                    let t = c.g[S3][0] == 0u64;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        break 'b_189d44;
                    } else {
                        'b_189bd8: {
                            'b_189b34: {
                                c.cycles += 9;
                                call_ptr(c, 0x0014dc80, 0x001899b0)?;
                                c.g[A1][0] = 0x4;
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x0014dd58, 0x001899bc)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V0][0] = 0x6;
                                let t = c.g[S0][0] == c.g[V0][0];
                                c.g[V0][0] = (c.g[S0][0] < 0x7u64) as u64;
                                if t {
                                    break 'b_189b34;
                                } else {
                                    'b_189a54: {
                                        'b_1899e4: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x5;
                                            if t {
                                                break 'b_1899e4;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[S0][0] == c.g[V0][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                if t {
                                                    break 'b_189a54;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        17,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_189d4c;
                                                    } else {
                                                        break 'b_1899e4;
                                                    }
                                                }
                                            }
                                        }
                                        'b_189a00: {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x7;
                                            let t = c.g[S0][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x9;
                                            if t {
                                                break 'b_189a00;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[S0][0] == c.g[V0][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                if t {
                                                    break 'b_189bd8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        17,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_189d4c;
                                                    } else {
                                                        break 'b_189a00;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 21;
                                        c.g[A1][0] = 0x460000;
                                        c.g[A0][0] = 0x4c;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2298));
                                        c.g[A2][0] = 0x100;
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317d70, 0x00189a18)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A2][0] = c.g[S2][0];
                                        call_ptr(c, 0x0039e318, 0x00189a2c)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[V0][0] = 0x470000;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4c30));
                                        c.g[A1][0] = c.g[S0][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        c.sb((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                                        call_ptr(c, 0x0039f290, 0x00189a4c)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        if t {
                                            break 'b_189d48;
                                        } else {
                                            break 'b_189a54;
                                        }
                                    }
                                    'b_189ac8: {
                                        c.cycles += 6;
                                        c.g[A0][0] = 0x4a0000;
                                        c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x38));
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1520));
                                        call_ptr(c, 0x00317670, 0x00189a64)?;
                                        let t = c.g[S0][0] != c.g[V0][0];
                                        c.g[A0][0] = 0x4a0000;
                                        if t {
                                            break 'b_189ac8;
                                        } else {
                                            c.cycles += 23;
                                            c.g[A1][0] = 0x460000;
                                            c.g[A0][0] = 0x54;
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_sub(0x2078));
                                            c.g[A2][0] = 0x100;
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00317d70, 0x00189a84)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[A1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            call_ptr(c, 0x0039e318, 0x00189a98)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                            c.g[V0][0] = 0x470000;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_sub(0x4dd0));
                                            c.g[A1][0] = c.g[S0][0];
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                                            c.sb((c.g[S0][0] as u32).wrapping_add(0x50), 0u64);
                                            call_ptr(c, 0x0039f290, 0x00189ac0)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                            if t {
                                                break 'b_189d48;
                                            } else {
                                                break 'b_189ac8;
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x38));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1528));
                                    call_ptr(c, 0x00317670, 0x00189ad4)?;
                                    let t = c.g[S0][0] != c.g[V0][0];
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                    if t {
                                        break 'b_189d48;
                                    } else {
                                        c.cycles += 22;
                                        c.g[A1][0] = 0x460000;
                                        c.g[A0][0] = 0x4c;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2060));
                                        c.g[A2][0] = 0x100;
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317d70, 0x00189af4)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A2][0] = c.g[S2][0];
                                        call_ptr(c, 0x0039e318, 0x00189b08)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[V0][0] = 0x470000;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4d00));
                                        c.g[V1][0] = 0x1b;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                                        c.g[A1][0] = c.g[S0][0];
                                        call_ptr(c, 0x0039f290, 0x00189b2c)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        if t {
                                            break 'b_189d48;
                                        } else {
                                            break 'b_189b34;
                                        }
                                    }
                                }
                            }
                            c.cycles += 36;
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00189b48)?;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            call_ptr(c, 0x00228c08, 0x00189b50)?;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[V0][0] = 0x530000;
                            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5610));
                            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 22);
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = c.g[A1][0] & 0x7;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00189b7c)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x48));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x80));
                            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                            let t = c.g[A1][0] as u32;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            call_ptr(c, t, 0x00189b98)?;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                            c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00189bbc)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[V0][0];
                            if t {
                                break 'b_189d44;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f400, 0x00189bd0)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                if t {
                                    break 'b_189d48;
                                } else {
                                    break 'b_189bd8;
                                }
                            }
                        }
                        'b_189c94: {
                            'b_189c90: {
                                c.cycles += 3;
                                c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x48));
                                let t = (c.g[V1][0] as i64) < 0;
                                c.g[V0][0] = 0x440000;
                                if t {
                                    break 'b_189c90;
                                } else {
                                    c.cycles += 8;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfd8));
                                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x38));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                    call_ptr(c, 0x00317670, 0x00189bfc)?;
                                    if c.g[S0][0] != c.g[V0][0] {
                                        c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x49));
                                        break 'b_189c94;
                                    } else {
                                        c.cycles += 35;
                                        c.g[A0][0] = c.g[S3][0];
                                        call_ptr(c, 0x0039a738, 0x00189c0c)?;
                                        c.g[A3][0] = 0x530000;
                                        c.g[V1][0] = 0xfffffffffe3f0000;
                                        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x5610));
                                        c.g[V1][0] = c.g[V1][0] | 0xffff;
                                        c.g[V0][0] = c.g[V0][0] & 0x7;
                                        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 22);
                                        c.g[A1][0] = c.g[A1][0] & c.g[V1][0];
                                        c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
                                        c.g[A0][0] = c.g[S3][0];
                                        c.sw((c.g[A3][0] as u32).wrapping_add(0x5610), c.g[A1][0]);
                                        c.g[S0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8c));
                                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x18));
                                        c.g[S0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        call_ptr(c, 0x0039a738, 0x00189c4c)?;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                        c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, t, 0x00189c60)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x48));
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                                        c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x80));
                                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                        let t = c.g[A1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                        call_ptr(c, t, 0x00189c7c)?;
                                        call_ptr(c, 0x0028b180, 0x00189c84)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x1;
                                        call_ptr(c, 0x0029ed90, 0x00189c90)?;
                                        break 'b_189c90;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x49));
                            break 'b_189c94;
                        }
                        'b_189ce8: {
                            'b_189ce4: {
                                c.cycles += 2;
                                let t = (c.g[V1][0] as i64) < 0;
                                c.g[V0][0] = 0x440000;
                                if t {
                                    break 'b_189ce4;
                                } else {
                                    c.cycles += 8;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfd8));
                                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x38));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                    call_ptr(c, 0x00317670, 0x00189cb4)?;
                                    if c.g[S0][0] != c.g[V0][0] {
                                        c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x4a));
                                        break 'b_189ce8;
                                    } else {
                                        c.cycles += 10;
                                        c.g[A0][0] = c.lbu((c.g[S3][0] as u32).wrapping_add(0x319));
                                        c.g[A1][0] = 0x530000;
                                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x5610));
                                        c.g[V0][0] = 0xfffffffffff70000;
                                        c.g[A0][0] = (0u64 < c.g[A0][0]) as u64;
                                        c.g[V0][0] = c.g[V0][0] | 0xffff;
                                        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 19);
                                        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                                        c.sw((c.g[A1][0] as u32).wrapping_add(0x5610), c.g[V1][0]);
                                        break 'b_189ce4;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x4a));
                            break 'b_189ce8;
                        }
                        'b_189d34: {
                            c.cycles += 2;
                            let t = (c.g[V1][0] as i64) < 0;
                            c.g[V0][0] = 0x440000;
                            if t {
                                break 'b_189d34;
                            } else {
                                c.cycles += 8;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfd8));
                                c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x38));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                call_ptr(c, 0x00317670, 0x00189d08)?;
                                let t = c.g[S0][0] != c.g[V0][0];
                                c.g[A1][0] = 0x530000;
                                if t {
                                    break 'b_189d34;
                                } else {
                                    c.cycles += 9;
                                    c.g[A0][0] = c.lbu((c.g[S3][0] as u32).wrapping_add(0x319));
                                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x5610));
                                    c.g[V0][0] = 0xffffffffffcf0000;
                                    c.g[A0][0] = c.g[A0][0] & 0x3;
                                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                                    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 20);
                                    c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x5610), c.g[V1][0]);
                                    break 'b_189d34;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        call_ptr(c, 0x00228c08, 0x00189d3c)?;
                        c.g[V0][0] = 0x1;
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1c20), c.g[V0][0]);
                        break 'b_189d44;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_189d48;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_189d4c;
        }
        c.cycles += 5;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    })
}

/// `cFEStateOptionsGame_RefreshListBoxes(self)` (0x189d60): set the language,
/// units and widescreen list boxes from the option interface, tell the locale
/// and the graphics manager, and activate the options.
pub fn cfe_state_options_game_refresh_list_boxes(c: &mut Cpu) {
    guest(c, |c| {
        'b_189edc: {
            'b_189ed8: {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x580));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x550));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x540));
                c.g[S2][0] = c.g[A0][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x530));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x570));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x560));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x520), c.g[RA][0]);
                c.g[S4][0] = 0x530000;
                call_ptr(c, 0x0014dc80, 0x00189d88)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x4;
                call_ptr(c, 0x0014dd58, 0x00189d94)?;
                c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x48));
                let t = (c.g[V1][0] as i64) < 0;
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x290));
                if t {
                    break 'b_189ed8;
                } else {
                    c.cycles += 12;
                    c.g[V0][0] = 0x440000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfd8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    call_ptr(c, 0x00317670, 0x00189db8)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00189dc4)?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] == 0u64 {
                        c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x49));
                        break 'b_189edc;
                    } else {
                        'b_189e68: {
                            'b_189e38: {
                                c.cycles += 4;
                                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5610));
                                c.g[V0][0] = c.g[A0][0] & 0x7;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V1][0] = c.g[SP][0];
                                if t {
                                    break 'b_189e38;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x280));
                                    'l_189de4: loop {
                                        c.cycles += 19;
                                        c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                                        c.ldr(5, c.g[A0][0] as u32);
                                        c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                                        c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                                        c.ldl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
                                        c.ldr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
                                        c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                                        c.ldr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                                        c.sdl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                                        c.sdr(5, c.g[V1][0] as u32);
                                        c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                                        c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                                        c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                                        c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                                        c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                        c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x18));
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                        let t = c.g[A0][0] != c.g[V0][0];
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00189de4);
                                            }
                                            continue 'l_189de4;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;

                                            if t {
                                                break 'b_189e68;
                                            } else {
                                                break 'b_189e38;
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x280));
                            'l_189e3c: loop {
                                c.cycles += 11;
                                c.g[T1][0] = c.ld(c.g[A0][0] as u32);
                                c.g[T2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                                c.g[T3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                                c.sd(c.g[V1][0] as u32, c.g[T1][0]);
                                c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T3][0]);
                                c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                let t = c.g[A0][0] != c.g[V0][0];
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00189e3c);
                                    }
                                    continue 'l_189e3c;
                                } else {
                                    break 'b_189e68;
                                }
                            }
                        }
                        c.cycles += 28;
                        c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                        c.ldr(6, c.g[A0][0] as u32);
                        c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.sdr(6, c.g[V1][0] as u32);
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                        c.g[A1][0] = sx((c.g[A1][0] as u32) >> 22);
                        c.g[A1][0] = c.g[A1][0] & 0x7;
                        call_ptr(c, 0x0039a7a8, 0x00189e8c)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x18));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        call_ptr(c, 0x0039a738, 0x00189ea8)?;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        c.g[A0][0] = c.g[S0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                        call_ptr(c, t, 0x00189ebc)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x48));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                        c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x80));
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        let t = c.g[A1][0] as u32;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        call_ptr(c, t, 0x00189ed8)?;
                        break 'b_189ed8;
                    }
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x49));
            break 'b_189edc;
        }
        'b_189fc4: {
            c.cycles += 2;
            let t = (c.g[V1][0] as i64) < 0;
            c.g[V0][0] = 0x440000;
            if t {
                break 'b_189fc4;
            } else {
                c.cycles += 11;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfd8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                call_ptr(c, 0x00317670, 0x00189ef8)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x00189f04)?;
                c.g[A0][0] = c.g[V0][0];
                let t = c.g[A0][0] == 0u64;
                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5610));
                if t {
                    break 'b_189fc4;
                } else {
                    'b_189fa4: {
                        'b_189f74: {
                            c.cycles += 3;
                            c.g[V0][0] = c.g[A1][0] & 0x7;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = c.g[SP][0];
                            if t {
                                break 'b_189f74;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                                'l_189f20: loop {
                                    c.cycles += 19;
                                    c.ldl(10, (c.g[A1][0] as u32).wrapping_add(0x7));
                                    c.ldr(10, c.g[A1][0] as u32);
                                    c.ldl(11, (c.g[A1][0] as u32).wrapping_add(0xf));
                                    c.ldr(11, (c.g[A1][0] as u32).wrapping_add(0x8));
                                    c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0x17));
                                    c.ldr(6, (c.g[A1][0] as u32).wrapping_add(0x10));
                                    c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x1f));
                                    c.ldr(7, (c.g[A1][0] as u32).wrapping_add(0x18));
                                    c.sdl(10, (c.g[V1][0] as u32).wrapping_add(0x7));
                                    c.sdr(10, c.g[V1][0] as u32);
                                    c.sdl(11, (c.g[V1][0] as u32).wrapping_add(0xf));
                                    c.sdr(11, (c.g[V1][0] as u32).wrapping_add(0x8));
                                    c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x17));
                                    c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x10));
                                    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                    let t = c.g[A1][0] != c.g[V0][0];
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00189f20);
                                        }
                                        continue 'l_189f20;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;

                                        if t {
                                            break 'b_189fa4;
                                        } else {
                                            break 'b_189f74;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                        'l_189f78: loop {
                            c.cycles += 11;
                            c.g[T0][0] = c.ld(c.g[A1][0] as u32);
                            c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                            c.g[T2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                            c.g[T3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                            c.sd(c.g[V1][0] as u32, c.g[T0][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            let t = c.g[A1][0] != c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00189f78);
                                }
                                continue 'l_189f78;
                            } else {
                                break 'b_189fa4;
                            }
                        }
                    }
                    c.cycles += 8;
                    c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[A1][0] as u32);
                    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.sdr(2, c.g[V1][0] as u32);
                    c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 19);
                    c.g[A1][0] = c.g[A1][0] & 0x1;
                    call_ptr(c, 0x0039a7a8, 0x00189fc4)?;
                    break 'b_189fc4;
                }
            }
        }
        'b_18a0b0: {
            c.cycles += 3;
            c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x4a));
            let t = (c.g[V1][0] as i64) < 0;
            c.g[V0][0] = 0x440000;
            if t {
                break 'b_18a0b0;
            } else {
                c.cycles += 11;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfd8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                call_ptr(c, 0x00317670, 0x00189fe4)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x00189ff0)?;
                c.g[A0][0] = c.g[V0][0];
                let t = c.g[A0][0] == 0u64;
                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5610));
                if t {
                    break 'b_18a0b0;
                } else {
                    'b_18a090: {
                        'b_18a060: {
                            c.cycles += 3;
                            c.g[V0][0] = c.g[A1][0] & 0x7;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = c.g[SP][0];
                            if t {
                                break 'b_18a060;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                                'l_18a00c: loop {
                                    c.cycles += 19;
                                    c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
                                    c.ldr(9, c.g[A1][0] as u32);
                                    c.ldl(10, (c.g[A1][0] as u32).wrapping_add(0xf));
                                    c.ldr(10, (c.g[A1][0] as u32).wrapping_add(0x8));
                                    c.ldl(11, (c.g[A1][0] as u32).wrapping_add(0x17));
                                    c.ldr(11, (c.g[A1][0] as u32).wrapping_add(0x10));
                                    c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0x1f));
                                    c.ldr(6, (c.g[A1][0] as u32).wrapping_add(0x18));
                                    c.sdl(9, (c.g[V1][0] as u32).wrapping_add(0x7));
                                    c.sdr(9, c.g[V1][0] as u32);
                                    c.sdl(10, (c.g[V1][0] as u32).wrapping_add(0xf));
                                    c.sdr(10, (c.g[V1][0] as u32).wrapping_add(0x8));
                                    c.sdl(11, (c.g[V1][0] as u32).wrapping_add(0x17));
                                    c.sdr(11, (c.g[V1][0] as u32).wrapping_add(0x10));
                                    c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                    c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                    let t = c.g[A1][0] != c.g[V0][0];
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0018a00c);
                                        }
                                        continue 'l_18a00c;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;

                                        if t {
                                            break 'b_18a090;
                                        } else {
                                            break 'b_18a060;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                        'l_18a064: loop {
                            c.cycles += 11;
                            c.g[A3][0] = c.ld(c.g[A1][0] as u32);
                            c.g[T0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                            c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                            c.g[T2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                            c.sd(c.g[V1][0] as u32, c.g[A3][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T2][0]);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            let t = c.g[A1][0] != c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018a064);
                                }
                                continue 'l_18a064;
                            } else {
                                break 'b_18a090;
                            }
                        }
                    }
                    c.cycles += 8;
                    c.ldl(11, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(11, c.g[A1][0] as u32);
                    c.sdl(11, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.sdr(11, c.g[V1][0] as u32);
                    c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 20);
                    c.g[A1][0] = c.g[A1][0] & 0x3;
                    call_ptr(c, 0x0039a7a8, 0x0018a0b0)?;
                    break 'b_18a0b0;
                }
            }
        }
        'b_18a15c: {
            'b_18a12c: {
                c.cycles += 9;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[A3][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5610));
                c.g[A2][0] = c.g[SP][0];
                c.g[A1][0] = c.g[A3][0] & 0x7;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10d8));
                c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x60));
                c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x64));
                let t = c.g[A1][0] == 0u64;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                if t {
                    break 'b_18a12c;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x280));
                    'l_18a0d8: loop {
                        c.cycles += 19;
                        c.ldl(9, (c.g[A3][0] as u32).wrapping_add(0x7));
                        c.ldr(9, c.g[A3][0] as u32);
                        c.ldl(10, (c.g[A3][0] as u32).wrapping_add(0xf));
                        c.ldr(10, (c.g[A3][0] as u32).wrapping_add(0x8));
                        c.ldl(11, (c.g[A3][0] as u32).wrapping_add(0x17));
                        c.ldr(11, (c.g[A3][0] as u32).wrapping_add(0x10));
                        c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0x1f));
                        c.ldr(3, (c.g[A3][0] as u32).wrapping_add(0x18));
                        c.sdl(9, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.sdr(9, c.g[A2][0] as u32);
                        c.sdl(10, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.sdr(10, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.sdl(11, (c.g[A2][0] as u32).wrapping_add(0x17));
                        c.sdr(11, (c.g[A2][0] as u32).wrapping_add(0x10));
                        c.sdl(3, (c.g[A2][0] as u32).wrapping_add(0x1f));
                        c.sdr(3, (c.g[A2][0] as u32).wrapping_add(0x18));
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                        let t = c.g[A3][0] != c.g[V0][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0018a0d8);
                            }
                            continue 'l_18a0d8;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_18a15c;
                            } else {
                                break 'b_18a12c;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x280));
            'l_18a130: loop {
                c.cycles += 11;
                c.g[A1][0] = c.ld(c.g[A3][0] as u32);
                c.g[T1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[T2][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[T3][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A2][0] as u32, c.g[A1][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                let t = c.g[A3][0] != c.g[V0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0018a130);
                    }
                    continue 'l_18a130;
                } else {
                    break 'b_18a15c;
                }
            }
        }
        'b_18a214: {
            'b_18a1e4: {
                c.cycles += 12;
                c.ldl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
                c.ldr(2, c.g[A3][0] as u32);
                c.sdl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.sdr(2, c.g[A2][0] as u32);
                c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x6));
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5610));
                c.g[V1][0] = c.g[A2][0] | c.g[A3][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[V1][0] = c.g[V1][0] & 0x7;
                let t = c.g[V1][0] == 0u64;
                c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                if t {
                    break 'b_18a1e4;
                } else {
                    c.cycles += 1;
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x280));
                    'l_18a190: loop {
                        c.cycles += 19;
                        c.ldl(10, (c.g[A3][0] as u32).wrapping_add(0x7));
                        c.ldr(10, c.g[A3][0] as u32);
                        c.ldl(11, (c.g[A3][0] as u32).wrapping_add(0xf));
                        c.ldr(11, (c.g[A3][0] as u32).wrapping_add(0x8));
                        c.ldl(2, (c.g[A3][0] as u32).wrapping_add(0x17));
                        c.ldr(2, (c.g[A3][0] as u32).wrapping_add(0x10));
                        c.ldl(9, (c.g[A3][0] as u32).wrapping_add(0x1f));
                        c.ldr(9, (c.g[A3][0] as u32).wrapping_add(0x18));
                        c.sdl(10, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.sdr(10, c.g[A2][0] as u32);
                        c.sdl(11, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.sdr(11, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.sdl(2, (c.g[A2][0] as u32).wrapping_add(0x17));
                        c.sdr(2, (c.g[A2][0] as u32).wrapping_add(0x10));
                        c.sdl(9, (c.g[A2][0] as u32).wrapping_add(0x1f));
                        c.sdr(9, (c.g[A2][0] as u32).wrapping_add(0x18));
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                        let t = c.g[A3][0] != c.g[V1][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0018a190);
                            }
                            continue 'l_18a190;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_18a214;
                            } else {
                                break 'b_18a1e4;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x280));
            'l_18a1e8: loop {
                c.cycles += 11;
                c.g[T2][0] = c.ld(c.g[A3][0] as u32);
                c.g[T3][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A2][0] as u32, c.g[T2][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T3][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                let t = c.g[A3][0] != c.g[V1][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0018a1e8);
                    }
                    continue 'l_18a1e8;
                } else {
                    break 'b_18a214;
                }
            }
        }
        c.cycles += 17;
        c.ldl(10, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(10, c.g[A3][0] as u32);
        c.sdl(10, (c.g[A2][0] as u32).wrapping_add(0x7));
        c.sdr(10, c.g[A2][0] as u32);
        c.g[V0][0] = c.lw(c.g[T0][0] as u32);
        let t = c.g[V0][0] as u32;
        c.g[A2][0] = c.lb((c.g[SP][0] as u32).wrapping_add(0x295));
        call_ptr(c, t, 0x0018a230)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        call_ptr(c, 0x00228c08, 0x0018a238)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x570));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x560));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x550));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x540));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x530));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x520));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x580));
        c.pc = t;
        return Ok(());
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Fixture objects, all above DATA+0x200000: clear of the fake log
    /// (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    const OC834_CTRL: u32 = DATA + 0x20_0100;
    const OC834_MAGIC: u32 = 0x4f43_8340;
    /// The shared fake vtable: every slot `{delta 0, FAKE + offset}`.
    const OC834_VT: u32 = DATA + 0x22_0000;
    /// Four 0x1000-byte objects; the last one is the interface object.
    const OC834_OBJ: u32 = DATA + 0x23_0000;
    const OC834_IFACE: u32 = OC834_OBJ + 3 * 0x1000;
    /// Forced result meaning "v0 = 0" (a fake cannot return 0 by itself).
    const OC834_ZERO: u32 = 0xffff_ffff;
    /// Values that fields and forced results are drawn from, so that
    /// comparisons between the two come out equal now and then.
    const OC834_POOL: [u32; 8] = [
        0,
        0x1234,
        0x5678,
        OC834_OBJ,
        OC834_OBJ + 0x1000,
        OC834_IFACE,
        0x0001_0203,
        0xff01_0200,
    ];

    /// The fake callee: logs the call, returns a pointer or the forced result
    /// of this call index (`OC834_ZERO`: 0).
    fn oc834_call(c: &mut Cpu, addr: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let forced = if n < 8 {
            c.bus.read32(fakes::LOG_RESULTS + 4 * n)
        } else {
            0
        };
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc == ra && forced == OC834_ZERO {
            c.g[V0][0] = 0;
        }
    }

    fn oc834_miss(c: &mut Cpu, target: u32) {
        oc834_call(c, target);
    }

    macro_rules! oc834_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC834_CTRL) != OC834_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc834_call(c, $addr);
                }
            )*
            fn oc834_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc834_fakes!(
        (
            oc834_fake_14dc80,
            OC834_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc834_fake_14dd58,
            OC834_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc834_fake_14f4f8,
            OC834_PREV_14F4F8,
            ee::s01::OVERRIDE_0014F4F8,
            ee::s01::f_0014f4f8__generated,
            0x0014_f4f8
        ),
        (
            oc834_fake_228c08,
            OC834_PREV_228C08,
            ee::s05::OVERRIDE_00228C08,
            ee::s05::f_00228c08__generated,
            0x0022_8c08
        ),
        (
            oc834_fake_28b180,
            OC834_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc834_fake_29ed90,
            OC834_PREV_29ED90,
            ee::s08::OVERRIDE_0029ED90,
            ee::s08::f_0029ed90__generated,
            0x0029_ed90
        ),
        (
            oc834_fake_317670,
            OC834_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc834_fake_317d70,
            OC834_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc834_fake_397870,
            OC834_PREV_397870,
            ee::s13::OVERRIDE_00397870,
            ee::s13::f_00397870__generated,
            0x0039_7870
        ),
        (
            oc834_fake_397840,
            OC834_PREV_397840,
            ee::s13::OVERRIDE_00397840,
            ee::s13::f_00397840__generated,
            0x0039_7840
        ),
        (
            oc834_fake_3978f0,
            OC834_PREV_3978F0,
            ee::s13::OVERRIDE_003978F0,
            ee::s13::f_003978f0__generated,
            0x0039_78f0
        ),
        (
            oc834_fake_39a738,
            OC834_PREV_39A738,
            ee::s13::OVERRIDE_0039A738,
            ee::s13::f_0039a738__generated,
            0x0039_a738
        ),
        (
            oc834_fake_39a7a8,
            OC834_PREV_39A7A8,
            ee::s13::OVERRIDE_0039A7A8,
            ee::s13::f_0039a7a8__generated,
            0x0039_a7a8
        ),
        (
            oc834_fake_39d860,
            OC834_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc834_fake_39e318,
            OC834_PREV_39E318,
            ee::s13::OVERRIDE_0039E318,
            ee::s13::f_0039e318__generated,
            0x0039_e318
        ),
        (
            oc834_fake_39e4c0,
            OC834_PREV_39E4C0,
            ee::s13::OVERRIDE_0039E4C0,
            ee::s13::f_0039e4c0__generated,
            0x0039_e4c0
        ),
        (
            oc834_fake_3a1000,
            OC834_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
    );

    fn oc834_pick(r: &mut Rng) -> u32 {
        OC834_POOL[(r.next_u32() % 8) as usize]
    }

    /// A machine with the gated fakes on, the vtable and objects built and
    /// the forced results of the first calls drawn from the pool.
    fn oc834_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc834_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        c.hooks.miss = oc834_miss;
        c.bus.write32(OC834_CTRL, OC834_MAGIC);
        for off in (0..0x400u32).step_by(8) {
            c.bus.write16(OC834_VT + off, 0);
            c.bus.write32(OC834_VT + off + 4, fakes::FAKE + off);
        }
        for i in 0..4u32 {
            let base = OC834_OBJ + i * 0x1000;
            for off in (0..0x400u32).step_by(4) {
                let v = oc834_pick(&mut r);
                c.bus.write32(base + off, v);
            }
            c.bus.write32(base + 8, OC834_VT);
        }
        c.bus.write32(OC834_IFACE + 0xc, OC834_VT);
        for n in 0..8u32 {
            let v = oc834_pick(&mut r);
            c.bus.write32(
                fakes::LOG_RESULTS + 4 * n,
                if v == 0 { OC834_ZERO } else { v },
            );
        }
        r
    }

    /// Menu notifications: keys 6/7 compare the list item's name hash.
    #[test]
    fn cfe_state_options_on_widget_notify_matches() {
        check(
            ee::s02::f_00188c68__generated,
            cfe_state_options_on_widget_notify,
            400,
            &|c, seed| {
                let mut r = oc834_setup(c, seed, 0x188c68, 3);
                c.g[A0][0] = OC834_OBJ as u64;
                c.g[A1][0] = (OC834_OBJ + 0x1000) as u64;
                c.g[A2][0] = [5u64, 6, 7, 8, 9, 10, 0x20, 6, 7][(r.next_u32() % 9) as usize];
                c.g[A3][0] = (r.next_u32() % 4) as u64;
            },
        );
    }

    /// Popup child events 0x15 (set the texts) and 0x16 (answer).
    #[test]
    fn cfe_state_options_on_child_event_matches() {
        check(
            ee::s02::f_00188d08__generated,
            cfe_state_options_on_child_event,
            400,
            &|c, seed| {
                let mut r = oc834_setup(c, seed, 0x188d08, 8);
                c.g[A0][0] = OC834_OBJ as u64;
                c.g[A1][0] = (OC834_OBJ + 0x1000) as u64;
                c.g[A2][0] = [0x15u64, 0x16, 0x16, 0x15, 0x17][(r.next_u32() % 5) as usize];
            },
        );
    }

    /// OnActivate is a single call to the base activation.
    #[test]
    fn cfe_state_options_game_on_activate_matches() {
        check(
            ee::s02::f_001895f0__generated,
            cfe_state_options_game_on_activate,
            12,
            &|c, seed| {
                oc834_setup(c, seed, 0x1895f0, 1);
                c.g[A0][0] = OC834_OBJ as u64;
            },
        );
    }

    /// Game options popup events.
    #[test]
    fn cfe_state_options_game_on_child_event_matches() {
        check(
            ee::s02::f_00189610__generated,
            cfe_state_options_game_on_child_event,
            600,
            &|c, seed| {
                let mut r = oc834_setup(c, seed, 0x189610, 14);
                c.g[A0][0] = OC834_OBJ as u64;
                c.g[A1][0] = (OC834_OBJ + 0x1000) as u64;
                c.g[A2][0] = [0x15u64, 0x16, 0x16, 0x16, 0x17][(r.next_u32() % 5) as usize];
            },
        );
    }

    /// Widget actions: popups (events 5, 7), language and unit options (9),
    /// the next state (6); a null widget returns at once.
    #[test]
    fn cfe_state_options_game_on_widget_action_matches() {
        check(
            ee::s02::f_00189980__generated,
            cfe_state_options_game_on_widget_action,
            600,
            &|c, seed| {
                let mut r = oc834_setup(c, seed, 0x189980, 14);
                c.g[A0][0] = OC834_OBJ as u64;
                c.g[A1][0] = if r.next_u32() % 8 == 0 {
                    0
                } else {
                    (OC834_OBJ + 0x1000) as u64
                };
                c.g[A2][0] = [5u64, 6, 7, 9, 9, 5, 8][(r.next_u32() % 7) as usize];
            },
        );
    }

    /// Refresh the list boxes of the game options screen.
    #[test]
    fn cfe_state_options_game_refresh_list_boxes_matches() {
        check(
            ee::s02::f_00189d60__generated,
            cfe_state_options_game_refresh_list_boxes,
            600,
            &|c, seed| {
                oc834_setup(c, seed, 0x189d60, 14);
                c.g[A0][0] = OC834_OBJ as u64;
            },
        );
    }
}
