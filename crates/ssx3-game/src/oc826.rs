//! Staging module oc826 (wave wave248): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc826`.

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
        0x0018_0840,
        keyboard_screen_type_key,
        "KeyboardScreen_TypeKey",
    ),
    (
        0x0018_0b60,
        keyboard_screen_init_keys,
        "KeyboardScreen_InitKeys",
    ),
    (
        0x0018_0ef0,
        keyboard_screen_handle_input,
        "KeyboardScreen_HandleInput",
    ),
    (
        0x0018_2220,
        fe_state_cheat_char_select_ctor,
        "cFEStateCheatCharSelect_ctor",
    ),
    (
        0x0018_2488,
        fe_state_cheat_char_select_on_widget_created,
        "cFEStateCheatCharSelect_OnWidgetCreated",
    ),
    (
        0x0018_2690,
        fe_state_cheat_char_select_on_widget_action,
        "cFEStateCheatCharSelect_OnWidgetAction",
    ),
];

/// `void KeyboardScreen_TypeKey(KeyboardScreen* self)` (0x180840): type the
/// highlighted key (`self+0x170`) into the text buffer (`self+0xec`) at the
/// cursor (`self+0x16c`, limit `self+0xd4`): digits, letters (upper case by
/// the caps flag `self+0xe4`), backspace (36), delete (37) and space (38).
/// Then tell the owner (`self+0xd0`) through its vtable: `(0, 13)` for key 39
/// (done), else `(0, 12)`.
pub fn keyboard_screen_type_key(c: &mut Cpu) {
    guest(c, |c| {
        'b_180954: {
            'b_180950: {
                'b_180938: {
                    'b_180874: {
                        c.cycles += 7;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                        c.g[A3][0] = c.g[A0][0];
                        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x170));
                        c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xa));
                        if t {
                            break 'b_180874;
                        } else {
                            c.cycles += 6;
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x16c));
                            c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x170));
                            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
                            let t = true;
                            c.sb((c.g[V1][0] as u32).wrapping_add(0xec), c.g[V0][0]);
                            if t {
                                break 'b_180938;
                            } else {
                                break 'b_180874;
                            }
                        }
                    }
                    'b_1808d0: {
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[V0][0] < 0x1au64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = 0x61;
                        if t {
                            break 'b_1808d0;
                        } else {
                            'b_1808c0: {
                                c.cycles += 16;
                                c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe4));
                                c.g[V1][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x170));
                                c.g[V0][0] = 0x41;
                                c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x16c));
                                if c.g[A2][0] == 0 {
                                    c.g[V0][0] = c.g[A1][0];
                                }
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xa));
                                c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xec));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.g[A0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x16c));
                                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xd4));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.sw((c.g[A3][0] as u32).wrapping_add(0x16c), c.g[V1][0]);
                                    break 'b_1808c0;
                                } else {
                                    break 'b_1808c0;
                                }
                            }
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x16c));
                            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            let t = true;
                            c.sb(c.g[V0][0] as u32, 0u64);
                            if t {
                                break 'b_180950;
                            } else {
                                break 'b_1808d0;
                            }
                        }
                    }
                    'b_180920: {
                        'b_180910: {
                            'b_1808f0: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x24;
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[V0][0] = 0x25;
                                if t {
                                    break 'b_1808f0;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x16c));
                                    let t = (c.g[V0][0] as i64) <= 0;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                    if t {
                                        break 'b_180910;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.sw((c.g[A3][0] as u32).wrapping_add(0x16c), c.g[V0][0]);
                                        if t {
                                            break 'b_180910;
                                        } else {
                                            break 'b_1808f0;
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = 0x26;
                            if t {
                                break 'b_180920;
                            } else {
                                c.cycles += 6;
                                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x16c));
                                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xd4));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.sw((c.g[A3][0] as u32).wrapping_add(0x16c), c.g[V1][0]);
                                    break 'b_180910;
                                } else {
                                    break 'b_180910;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x16c));
                        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        let t = true;
                        c.sb((c.g[V0][0] as u32).wrapping_add(0xec), 0u64);
                        if t {
                            break 'b_180950;
                        } else {
                            break 'b_180920;
                        }
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x170));
                        break 'b_180954;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x16c));
                        c.g[V1][0] = 0x20;
                        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sb((c.g[V0][0] as u32).wrapping_add(0xec), c.g[V1][0]);
                        break 'b_180938;
                    }
                }
                c.cycles += 6;
                c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x16c));
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xd4));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0u64 {
                    c.sw((c.g[A3][0] as u32).wrapping_add(0x16c), c.g[A0][0]);
                    break 'b_180950;
                } else {
                    break 'b_180950;
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x170));
            break 'b_180954;
        }
        'b_1809a4: {
            'b_180984: {
                c.cycles += 3;
                c.g[V0][0] = 0x27;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A3][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xd0));
                if t {
                    break 'b_180984;
                } else {
                    c.cycles += 9;
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0xd;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0018097c);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0018097c)?;
                    let t = true;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    if t {
                        break 'b_1809a4;
                    } else {
                        break 'b_180984;
                    }
                }
            }
            c.cycles += 8;
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0xc;
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001809a0);
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x001809a0)?;
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            break 'b_1809a4;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `void KeyboardScreen_InitKeys(KeyboardScreen* self, UIScreen* screen)`
/// (0x180b60): set up the on-screen keyboard objects: the arrow and END keys
/// get their texts, the letter keys `A..Z` and digit keys `0..9` get their
/// character, every cursor object is hidden, the `A` cursor is shown, and
/// the text field is cleared and bound to `self+0xec`. Each object is looked
/// up by `StrHash(name)` with `UIScreen_FindObject`.
pub fn keyboard_screen_init_keys(c: &mut Cpu) {
    guest(c, |c| {
        'b_180bcc: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S4][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[A0][0] = 0x460000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S2][0] = c.g[A1][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[RA][0] = sx(0x00180b90);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a90));
            call_ptr(c, 0x00317670, 0x00180b90)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00180b9c);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0039d860, 0x00180b9c)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_180bcc;
            } else {
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00180bbc);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180bbc)?;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00180bcc);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1370));
                call_ptr(c, 0x003a0c50, 0x00180bcc)?;
                break 'b_180bcc;
            }
        }
        'b_180c14: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x00180bd8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a78));
            call_ptr(c, 0x00317670, 0x00180bd8)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00180be4);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0039d860, 0x00180be4)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_180c14;
            } else {
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00180c04);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180c04)?;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00180c14);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1378));
                call_ptr(c, 0x003a0c50, 0x00180c14)?;
                break 'b_180c14;
            }
        }
        'b_180c5c: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x00180c20);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a60));
            call_ptr(c, 0x00317670, 0x00180c20)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00180c2c);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0039d860, 0x00180c2c)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_180c5c;
            } else {
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00180c4c);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180c4c)?;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00180c5c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1380));
                call_ptr(c, 0x003a0c50, 0x00180c5c)?;
                break 'b_180c5c;
            }
        }
        'b_180c94: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x00180c68);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a50));
            call_ptr(c, 0x00317670, 0x00180c68)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00180c74);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0039d860, 0x00180c74)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_180c94;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00180c94);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180c94)?;
                break 'b_180c94;
            }
        }
        'b_180ccc: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x00180ca0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a40));
            call_ptr(c, 0x00317670, 0x00180ca0)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00180cac);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0039d860, 0x00180cac)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_180ccc;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00180ccc);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180ccc)?;
                break 'b_180ccc;
            }
        }
        'b_180d04: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x00180cd8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a30));
            call_ptr(c, 0x00317670, 0x00180cd8)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00180ce4);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0039d860, 0x00180ce4)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_180d04;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00180d04);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180d04)?;
                break 'b_180d04;
            }
        }
        c.cycles += 5;
        c.g[V0][0] = 0x41;
        c.sb((c.g[SP][0] as u32).wrapping_add(0x21), 0u64);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
        c.g[S3][0] = 0u64;
        'l_180d18: loop {
            'b_180d54: {
                c.cycles += 13;
                c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0xa));
                c.g[A2][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00180d2c);
                c.g[A0][0] = c.g[S4][0];
                Callee {
                    addr: 0x001809b0,
                    f: ee::s02::KeyboardScreen_GetKeyObjectName,
                }
                .call(c, 0x00180d2c)?;
                c.g[RA][0] = sx(0x00180d34);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x00180d34)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00180d40);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0039d860, 0x00180d40)?;
                c.g[S0][0] = c.g[V0][0];
                let t = c.g[S0][0] == 0u64;
                c.g[A0][0] = c.g[S0][0];
                if t {
                    break 'b_180d54;
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00180d54);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    call_ptr(c, 0x003a0c50, 0x00180d54)?;
                    break 'b_180d54;
                }
            }
            'b_180d98: {
                c.cycles += 12;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x00180d64);
                c.g[A0][0] = c.g[S4][0];
                Callee {
                    addr: 0x00180a90,
                    f: ee::s02::KeyboardScreen_GetCursorObjectName,
                }
                .call(c, 0x00180d64)?;
                c.g[RA][0] = sx(0x00180d6c);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x00180d6c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00180d78);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0039d860, 0x00180d78)?;
                c.g[S0][0] = c.g[V0][0];
                let t = c.g[S0][0] == 0u64;
                c.g[A1][0] = 0u64;
                if t {
                    break 'b_180d98;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00180d98);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00180d98)?;
                    break 'b_180d98;
                }
            }
            c.cycles += 6;
            c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
            c.g[V1][0] = ((c.g[S3][0] as i64) < 26i64) as u64;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            let t = c.g[V1][0] != 0u64;
            c.sb((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00180d18);
                }
                continue 'l_180d18;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x30;
                c.g[S1][0] = 0u64;
                c.sb((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                'l_180dc0: loop {
                    'b_180df8: {
                        c.cycles += 12;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x00180dd0);
                        c.g[A0][0] = c.g[S4][0];
                        Callee {
                            addr: 0x001809b0,
                            f: ee::s02::KeyboardScreen_GetKeyObjectName,
                        }
                        .call(c, 0x00180dd0)?;
                        c.g[RA][0] = sx(0x00180dd8);
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x00317670, 0x00180dd8)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00180de4);
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0039d860, 0x00180de4)?;
                        c.g[S0][0] = c.g[V0][0];
                        let t = c.g[S0][0] == 0u64;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_180df8;
                        } else {
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x00180df8);
                            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                            call_ptr(c, 0x003a0c50, 0x00180df8)?;
                            break 'b_180df8;
                        }
                    }
                    'b_180e3c: {
                        c.cycles += 12;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x00180e08);
                        c.g[A0][0] = c.g[S4][0];
                        Callee {
                            addr: 0x00180a90,
                            f: ee::s02::KeyboardScreen_GetCursorObjectName,
                        }
                        .call(c, 0x00180e08)?;
                        c.g[RA][0] = sx(0x00180e10);
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x00317670, 0x00180e10)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00180e1c);
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0039d860, 0x00180e1c)?;
                        c.g[S0][0] = c.g[V0][0];
                        let t = c.g[S0][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_180e3c;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00180e3c);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00180e3c)?;
                            break 'b_180e3c;
                        }
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = ((c.g[S1][0] as i64) < 10i64) as u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    let t = c.g[V1][0] != 0u64;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00180dc0);
                        }
                        continue 'l_180dc0;
                    } else {
                        'b_180e94: {
                            c.cycles += 9;
                            c.g[A0][0] = 0x460000;
                            c.g[RA][0] = sx(0x00180e60);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a20));
                            call_ptr(c, 0x00317670, 0x00180e60)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00180e6c);
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x0039d860, 0x00180e6c)?;
                            c.g[S0][0] = c.g[V0][0];
                            if c.g[S0][0] == 0u64 {
                                c.g[A1][0] = 0x4a0000;
                                break 'b_180e94;
                            } else {
                                c.cycles += 7;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00180e90);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00180e90)?;
                                c.g[A1][0] = 0x4a0000;
                                break 'b_180e94;
                            }
                        }
                        'b_180ecc: {
                            c.cycles += 12;
                            c.g[A0][0] = c.g[S4][0];
                            c.g[RA][0] = sx(0x00180ea0);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1388));
                            Callee {
                                addr: 0x001805d8,
                                f: ee::s02::KeyboardScreen_SetText,
                            }
                            .call(c, 0x00180ea0)?;
                            c.g[A0][0] = 0x460000;
                            c.g[RA][0] = sx(0x00180eac);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a10));
                            call_ptr(c, 0x00317670, 0x00180eac)?;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x00180eb8);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00180eb8)?;
                            c.g[S0][0] = c.g[V0][0];
                            let t = c.g[S0][0] == 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_180ecc;
                            } else {
                                c.cycles += 2;
                                c.g[RA][0] = sx(0x00180ecc);
                                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0xec));
                                call_ptr(c, 0x003a0c50, 0x00180ecc)?;
                                break 'b_180ecc;
                            }
                        }
                        c.cycles += 8;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// `int KeyboardScreen_HandleInput(KeyboardScreen* self, FEUIInput* in)`
/// (0x180ef0): hide the cursor object of the current key, act on the first
/// pressed button (left/right/up/down move the cursor, next types the key,
/// shift toggles caps), show the cursor object of the new key. Returns 1.
pub fn keyboard_screen_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_180f48: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[A2][0] = c.g[SP][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            c.g[RA][0] = sx(0x00180f14);
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x170));
            Callee {
                addr: 0x00180a90,
                f: ee::s02::KeyboardScreen_GetCursorObjectName,
            }
            .call(c, 0x00180f14)?;
            c.g[RA][0] = sx(0x00180f1c);
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x00317670, 0x00180f1c)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00180f28);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0039d860, 0x00180f28)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_180f48;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00180f48);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180f48)?;
                break 'b_180f48;
            }
        }
        'b_181038: {
            'b_180f74: {
                c.cycles += 7;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x98));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00180f5c);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180f5c)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    break 'b_180f74;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00180f6c);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00180618,
                        f: ee::s02::KeyboardScreen_CursorLeft,
                    }
                    .call(c, 0x00180f6c)?;
                    let t = true;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x170));
                    if t {
                        break 'b_181038;
                    } else {
                        break 'b_180f74;
                    }
                }
            }
            'b_180f9c: {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00180f84);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180f84)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    break 'b_180f9c;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00180f94);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00180678,
                        f: ee::s02::KeyboardScreen_CursorRight,
                    }
                    .call(c, 0x00180f94)?;
                    let t = true;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x170));
                    if t {
                        break 'b_181038;
                    } else {
                        break 'b_180f9c;
                    }
                }
            }
            'b_180fc4: {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00180fac);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180fac)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    break 'b_180fc4;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00180fbc);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x001806d8,
                        f: ee::s02::KeyboardScreen_CursorUp,
                    }
                    .call(c, 0x00180fbc)?;
                    let t = true;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x170));
                    if t {
                        break 'b_181038;
                    } else {
                        break 'b_180fc4;
                    }
                }
            }
            'b_180fec: {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00180fd4);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180fd4)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    break 'b_180fec;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00180fe4);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00180768,
                        f: ee::s02::KeyboardScreen_CursorDown,
                    }
                    .call(c, 0x00180fe4)?;
                    let t = true;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x170));
                    if t {
                        break 'b_181038;
                    } else {
                        break 'b_180fec;
                    }
                }
            }
            'b_181014: {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00180ffc);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00180ffc)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    break 'b_181014;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0018100c);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00180840,
                        f: ee::s02::KeyboardScreen_TypeKey,
                    }
                    .call(c, 0x0018100c)?;
                    let t = true;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x170));
                    if t {
                        break 'b_181038;
                    } else {
                        break 'b_181014;
                    }
                }
            }
            c.cycles += 6;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xd0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd4));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x00181024);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x00181024)?;
            if c.g[V0][0] == 0u64 {
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x170));
                break 'b_181038;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00181034);
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x001807c8,
                    f: ee::s02::KeyboardScreen_ToggleCaps,
                }
                .call(c, 0x00181034)?;
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x170));
                break 'b_181038;
            }
        }
        'b_181078: {
            c.cycles += 11;
            c.g[A2][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x00181044);
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x00180a90,
                f: ee::s02::KeyboardScreen_GetCursorObjectName,
            }
            .call(c, 0x00181044)?;
            c.g[RA][0] = sx(0x0018104c);
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x00317670, 0x0018104c)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x00181058);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00181058)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_181078;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00181078);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00181078)?;
                break 'b_181078;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = 0x1;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateCheatCharSelect* cFEStateCheatCharSelect_ctor(self, engine, owner,
/// char player, int slot)` (0x182220): construct the cheat character select
/// state: the slot's own face (entry 0, texture from the owner's bank) and
/// one entry (id, icon texture, name) for every cheat character the player
/// owns; entry 0's name is the slot's short name. Clears the six selections
/// and reads the player control mask.
pub fn fe_state_cheat_char_select_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_182388: {
            c.cycles += 48;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S0][0] = sx((c.g[A3][0] as u32) << 24);
            c.g[S1][0] = c.g[A0][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S6][0] = c.g[T0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[RA][0] = sx(0x00182264);
            c.g[S3][0] = 0u64;
            call_ptr(c, 0x0039e318, 0x00182264)?;
            c.g[V0][0] = 0xa;
            c.g[V1][0] = 0x470000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30d0));
            c.g[A1][0] = 0x1;
            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            c.g[A0][0] = 0x440000;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf78));
            c.sb((c.g[S1][0] as u32).wrapping_add(0x44), c.g[S0][0]);
            c.g[V0][0] = sx((c.g[S6][0] as u32) << 2);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x78), c.g[A1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x70), 0u64);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x7c), 0u64);
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = sx(0x001822a8);
            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
            call_ptr(c, 0x00317670, 0x001822a8)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
            c.g[RA][0] = sx(0x001822b4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x00398380, 0x001822b4)?;
            c.g[RA][0] = sx(0x001822bc);
            c.sw((c.g[S1][0] as u32).wrapping_add(0xf4), c.g[V0][0]);
            call_ptr(c, 0x0014dc80, 0x001822bc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001822c8);
            c.g[A1][0] = 0xd;
            call_ptr(c, 0x0014dd58, 0x001822c8)?;
            c.g[RA][0] = sx(0x001822d0);
            c.sw(c.g[SP][0] as u32, c.g[V0][0]);
            call_ptr(c, 0x0015a478, 0x001822d0)?;
            c.g[S5][0] = c.g[V0][0];
            c.g[S4][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x50));
            let t = (c.g[S4][0] as i64) <= 0;

            if t {
                break 'b_182388;
            } else {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x7c));
                c.g[FP][0] = sx((c.g[S1][0] as u32).wrapping_add(0xf4));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[S7][0] = sx((c.g[S1][0] as u32).wrapping_add(0x16c));
                c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x28));
                'l_1822f8: loop {
                    'b_182378: {
                        c.cycles += 9;
                        c.g[V0][0] = sx((c.g[S3][0] as u32) << 4);
                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                        c.g[A2][0] = c.g[S6][0];
                        c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                        c.g[RA][0] = sx(0x00182314);
                        c.g[A3][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0xf));
                        call_ptr(c, 0x00157ee0, 0x00182314)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        if t {
                            break 'b_182378;
                        } else {
                            c.cycles += 23;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                            c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0xf));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                            c.g[RA][0] = sx(0x0018233c);
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                            call_ptr(c, 0x00317670, 0x0018233c)?;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                            c.g[RA][0] = sx(0x00182348);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x00398380, 0x00182348)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                            c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                            c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                            break 'b_182378;
                        }
                    }
                    c.cycles += 4;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x28));
                        if event_due(c) {
                            deliver_event_here(c, 0x001822f8);
                        }
                        continue 'l_1822f8;
                    } else {
                        break 'b_182388;
                    }
                }
            }
        }
        c.cycles += 12;
        c.g[RA][0] = sx(0x00182390);
        call_ptr(c, 0x0014dc80, 0x00182390)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0018239c);
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0014dd58, 0x0018239c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S6][0];
        c.g[RA][0] = sx(0x001823ac);
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x0014eec8, 0x001823ac)?;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x16c), c.g[V0][0]);
        c.g[V1][0] = 0x5;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x68));
        'l_1823b8: loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let t = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001823b8);
                }
                continue 'l_1823b8;
            } else {
                c.cycles += 19;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x74), 0u64);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                c.g[RA][0] = sx(0x001823e8);
                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                call_ptr(c, 0x001a1cd0, 0x001823e8)?;
                c.sb((c.g[S1][0] as u32).wrapping_add(0x15), c.g[V0][0]);
                c.g[V0][0] = c.g[S1][0];
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
    });
}

/// `void cFEStateCheatCharSelect_OnWidgetCreated(self, UIWidget* w)`
/// (0x182488): bind a created widget by its name hash: the menu (`6pics`),
/// the two arrows (shown when 7 or more characters), the name text, or one of
/// the six character widgets (table at 0x440fa0).
pub fn fe_state_cheat_char_select_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_182660: {
            'b_182514: {
                c.cycles += 19;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S1][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S2][0] = c.g[A1][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A0][0] = 0x4a0000;
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1400));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001824cc);
                c.g[S5][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                call_ptr(c, 0x00317670, 0x001824cc)?;
                let t = c.g[S5][0] != c.g[V0][0];
                c.g[A0][0] = 0x460000;
                if t {
                    break 'b_182514;
                } else {
                    c.cycles += 11;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x6c), c.g[S2][0]);
                    c.g[V1][0] = 0xfffffffffffffffe;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x18), 0u64);
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 6i64) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        break 'b_182660;
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x78));
                        c.g[RA][0] = sx(0x0018250c);
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                        call_ptr(c, 0x0039b760, 0x0018250c)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        if t {
                            break 'b_182660;
                        } else {
                            break 'b_182514;
                        }
                    }
                }
            }
            'b_182568: {
                'b_182540: {
                    'b_18252c: {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0018251c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2870));
                        call_ptr(c, 0x00317670, 0x0018251c)?;
                        let t = c.g[S5][0] != c.g[V0][0];
                        c.g[A0][0] = 0x460000;
                        if t {
                            break 'b_18252c;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[S2][0]);
                            if t {
                                break 'b_182540;
                            } else {
                                break 'b_18252c;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00182534);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2860));
                    call_ptr(c, 0x00317670, 0x00182534)?;
                    let t = c.g[S5][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        break 'b_182568;
                    } else {
                        c.cycles += 1;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[S2][0]);
                        break 'b_182540;
                    }
                }
                c.cycles += 10;
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = ((c.g[A1][0] as i64) < 7i64) as u64;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                c.g[A1][0] = c.g[A1][0] ^ 0x1;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00182560);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00182560)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                if t {
                    break 'b_182660;
                } else {
                    break 'b_182568;
                }
            }
            'b_182594: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00182570);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2850));
                call_ptr(c, 0x00317670, 0x00182570)?;
                let t = c.g[S5][0] != c.g[V0][0];
                c.g[V0][0] = 0x440000;
                if t {
                    break 'b_182594;
                } else {
                    c.cycles += 7;
                    c.g[A1][0] = 0x4a0000;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x50), c.g[S2][0]);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0018258c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                    call_ptr(c, 0x003a0c50, 0x0018258c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        break 'b_182660;
                    } else {
                        break 'b_182594;
                    }
                }
            }
            c.cycles += 7;
            c.g[S3][0] = 0u64;
            c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfa0));
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x54));
            c.g[FP][0] = 0x4c0000;
            c.g[S7][0] = 0xffffffffffffffff;
            c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0xf4));
            'l_1825b0: loop {
                'b_182648: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001825b8);
                    c.g[A0][0] = c.lw(c.g[S6][0] as u32);
                    call_ptr(c, 0x00317670, 0x001825b8)?;
                    if c.g[S5][0] != c.g[V0][0] {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        break 'b_182648;
                    } else {
                        'b_1825e8: {
                            c.cycles += 3;
                            c.sw(c.g[S0][0] as u32, c.g[S2][0]);
                            let t = c.g[S3][0] != 0u64;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[S3][0]);
                            if t {
                                break 'b_1825e8;
                            } else {
                                c.cycles += 7;
                                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                                c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x66b8));
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa8));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xac));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001825e8);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001825e8)?;
                                break 'b_1825e8;
                            }
                        }
                        'b_182638: {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                break 'b_182638;
                            } else {
                                c.cycles += 16;
                                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                                c.g[A1][0] = 0u64;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00182614);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00182614)?;
                                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                                c.g[A1][0] = 0x1;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00182630);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00182630)?;
                                let t = true;
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_182648;
                                } else {
                                    break 'b_182638;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(c.g[S4][0] as u32);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x78), c.g[S7][0]);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), c.g[V1][0]);
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        break 'b_182648;
                    }
                }
                c.cycles += 5;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = ((c.g[S3][0] as i64) < 6i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x001825b0);
                    }
                    continue 'l_1825b0;
                } else {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    break 'b_182660;
                }
            }
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateCheatCharSelect_OnWidgetAction(self, UIWidget* menu, int msg)`
/// (0x182690): widget messages: 1 selects (grow the widget, set the name),
/// 2 deselects (shrink), 5 confirms (store the player's cheat character,
/// pop the state), 6 cancels (pop the state). A null widget does nothing.
pub fn fe_state_cheat_char_select_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_1827fc: {
            'b_1827f8: {
                'b_1827f4: {
                    c.cycles += 7;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.g[A3][0] = c.g[A1][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    let t = c.g[A3][0] == 0u64;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        break 'b_1827f4;
                    } else {
                        'b_182774: {
                            'b_182758: {
                                'b_182710: {
                                    'b_1826ec: {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x2;
                                        let t = c.g[A2][0] == c.g[V0][0];
                                        c.g[V0][0] = (c.g[A2][0] < 0x3u64) as u64;
                                        if t {
                                            break 'b_1826ec;
                                        } else {
                                            'b_1826d0: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = 0x1;
                                                if t {
                                                    break 'b_1826d0;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[A2][0] == c.g[V0][0];
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x20),
                                                    );
                                                    if t {
                                                        break 'b_182710;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.lq(
                                                            17,
                                                            (c.g[SP][0] as u32).wrapping_add(0x10),
                                                        );
                                                        if t {
                                                            break 'b_1827fc;
                                                        } else {
                                                            break 'b_1826d0;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x5;
                                            let t = c.g[A2][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x6;
                                            if t {
                                                break 'b_182774;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A2][0] == c.g[V0][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    break 'b_182758;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        17,
                                                        (c.g[SP][0] as u32).wrapping_add(0x10),
                                                    );
                                                    if t {
                                                        break 'b_1827fc;
                                                    } else {
                                                        break 'b_1826ec;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 9;
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0x4c0000;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x66a8));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa8));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xac));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x00182708);
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00182708)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_1827f8;
                                    } else {
                                        break 'b_182710;
                                    }
                                }
                                c.cycles += 18;
                                c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x4c0000;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x66b8));
                                c.g[S0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x18));
                                c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x98));
                                c.g[A0][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0xa8));
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xac));
                                c.g[S0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x0018273c);
                                c.g[A0][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0018273c)?;
                                c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x50));
                                c.g[S0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                c.g[RA][0] = sx(0x00182750);
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x16c));
                                call_ptr(c, 0x003a0c50, 0x00182750)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_1827f8;
                                } else {
                                    break 'b_182758;
                                }
                            }
                            c.cycles += 7;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                            c.g[A1][0] = 0x1;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x70), 0u64);
                            c.g[RA][0] = sx(0x0018276c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            call_ptr(c, 0x0039f190, 0x0018276c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_1827f8;
                            } else {
                                break 'b_182774;
                            }
                        }
                        'b_1827a0: {
                            'b_182794: {
                                c.cycles += 6;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                                c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x18));
                                c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x98));
                                c.g[V0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                if t {
                                    break 'b_182794;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x70), 0u64);
                                    if t {
                                        break 'b_1827a0;
                                    } else {
                                        break 'b_182794;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x70), c.g[V1][0]);
                            break 'b_1827a0;
                        }
                        c.cycles += 21;
                        c.g[RA][0] = sx(0x001827a8);
                        call_ptr(c, 0x0014dc80, 0x001827a8)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001827b4);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x0014dd58, 0x001827b4)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                        c.g[RA][0] = sx(0x001827c8);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x001474a8, 0x001827c8)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001827dc);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001827dc)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = 0x1;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x74), c.g[V0][0]);
                        c.g[A1][0] = 0x1;
                        c.g[RA][0] = sx(0x001827f4);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039f190, 0x001827f4)?;
                        break 'b_1827f4;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_1827f8;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1827fc;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{compare, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Fixture areas, clear of the fake log (DATA+0x100000), the pointers the
    /// fakes return (DATA..DATA+0x40000) and the other waves' fixtures.
    const OC826_CTRL: u32 = DATA + 0x3a_0000;
    const OC826_MAGIC: u32 = 0x4f43_3832;
    /// Forced result of fake call `n`: word `n & 0x3ff` (0: the fake's own
    /// pointer, else `v0 = word - 1`).
    const OC826_RES: u32 = OC826_CTRL + 0x1000;
    const OC826_SELF: u32 = DATA + 0x3a_2000;
    /// Two objects whose vtable (at +8 and +0xc) has a fake method in every slot.
    const OC826_OBJA: u32 = DATA + 0x3a_4000;
    const OC826_OBJB: u32 = DATA + 0x3a_5000;
    const OC826_MENU: u32 = DATA + 0x3a_6000;
    const OC826_DB: u32 = DATA + 0x3a_7000;
    const OC826_ENTRIES: u32 = DATA + 0x3a_7400;
    const OC826_STR: u32 = DATA + 0x3a_8000;

    /// The fake callee: log the call like `fakes::fake_call`, then apply the
    /// forced result of this call index (unless the call unwound).
    fn oc826_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake_call(c, addr);
        if c.pc == ra {
            let m = c.bus.read32(OC826_RES + 4 * (n & 0x3ff));
            if m != 0 {
                c.g[V0][0] = m.wrapping_sub(1) as u64;
            }
        }
    }

    fn oc826_miss(c: &mut Cpu, target: u32) {
        oc826_call(c, target);
    }

    /// Gated fakes of the callees in their override slots.
    macro_rules! oc826_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC826_CTRL) != OC826_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc826_call(c, $addr);
                }
            )*
            fn oc826_install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc826_fakes!(
        (
            oc826_fake_317670,
            OC826_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc826_fake_39d860,
            OC826_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc826_fake_3a0c50,
            OC826_PREV_3A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50
        ),
        (
            oc826_fake_39f190,
            OC826_PREV_39F190,
            ee::s13::OVERRIDE_0039F190,
            ee::s13::f_0039f190__generated,
            0x0039_f190
        ),
        (
            oc826_fake_39b760,
            OC826_PREV_39B760,
            ee::s13::OVERRIDE_0039B760,
            ee::s13::f_0039b760__generated,
            0x0039_b760
        ),
        (
            oc826_fake_39e318,
            OC826_PREV_39E318,
            ee::s13::OVERRIDE_0039E318,
            ee::s13::f_0039e318__generated,
            0x0039_e318
        ),
        (
            oc826_fake_398380,
            OC826_PREV_398380,
            ee::s13::OVERRIDE_00398380,
            ee::s13::f_00398380__generated,
            0x0039_8380
        ),
        (
            oc826_fake_14dc80,
            OC826_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc826_fake_14dd58,
            OC826_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc826_fake_15a478,
            OC826_PREV_15A478,
            ee::s01::OVERRIDE_0015A478,
            ee::s01::f_0015a478__generated,
            0x0015_a478
        ),
        (
            oc826_fake_157ee0,
            OC826_PREV_157EE0,
            ee::s01::OVERRIDE_00157EE0,
            ee::s01::f_00157ee0__generated,
            0x0015_7ee0
        ),
        (
            oc826_fake_14eec8,
            OC826_PREV_14EEC8,
            ee::s01::OVERRIDE_0014EEC8,
            ee::s01::f_0014eec8__generated,
            0x0014_eec8
        ),
        (
            oc826_fake_1474a8,
            OC826_PREV_1474A8,
            ee::s01::OVERRIDE_001474A8,
            ee::s01::f_001474a8__generated,
            0x0014_74a8
        ),
        (
            oc826_fake_1a1cd0,
            OC826_PREV_1A1CD0,
            ee::s03::OVERRIDE_001A1CD0,
            ee::s03::f_001a1cd0__generated,
            0x001a_1cd0
        ),
        (
            oc826_fake_1809b0,
            OC826_PREV_1809B0,
            ee::s02::OVERRIDE_001809B0,
            ee::s02::f_001809b0__generated,
            0x0018_09b0
        ),
        (
            oc826_fake_180a90,
            OC826_PREV_180A90,
            ee::s02::OVERRIDE_00180A90,
            ee::s02::f_00180a90__generated,
            0x0018_0a90
        ),
        (
            oc826_fake_1805d8,
            OC826_PREV_1805D8,
            ee::s02::OVERRIDE_001805D8,
            ee::s02::f_001805d8__generated,
            0x0018_05d8
        ),
        (
            oc826_fake_180618,
            OC826_PREV_180618,
            ee::s02::OVERRIDE_00180618,
            ee::s02::f_00180618__generated,
            0x0018_0618
        ),
        (
            oc826_fake_180678,
            OC826_PREV_180678,
            ee::s02::OVERRIDE_00180678,
            ee::s02::f_00180678__generated,
            0x0018_0678
        ),
        (
            oc826_fake_1806d8,
            OC826_PREV_1806D8,
            ee::s02::OVERRIDE_001806D8,
            ee::s02::f_001806d8__generated,
            0x0018_06d8
        ),
        (
            oc826_fake_180768,
            OC826_PREV_180768,
            ee::s02::OVERRIDE_00180768,
            ee::s02::f_00180768__generated,
            0x0018_0768
        ),
        (
            oc826_fake_1807c8,
            OC826_PREV_1807C8,
            ee::s02::OVERRIDE_001807C8,
            ee::s02::f_001807c8__generated,
            0x0018_07c8
        ),
    );

    /// Scrambled machine with the fakes armed (the miss hook and the gated
    /// slot fakes) and a random result for every fake call: 0, the fake's own
    /// pointer, or the object [`OC826_OBJA`].
    fn oc826_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc826_install();
        let mut r = machine(c, seed, salt, calls);
        c.hooks.miss = oc826_miss;
        c.bus.write32(OC826_CTRL, OC826_MAGIC);
        for i in 0..0x400u32 {
            let w = match r.next_u32() % 4 {
                0 => 0,
                1 => 1,
                _ => OC826_OBJA + 1,
            };
            c.bus.write32(OC826_RES + 4 * i, w);
        }
        for obj in [OC826_OBJA, OC826_OBJB] {
            oc826_object(c, obj);
        }
        r
    }

    /// Force the result of fake call `n` (`None`: 0).
    fn oc826_force(c: &mut Cpu, n: u32, v: Option<u32>) {
        c.bus
            .write32(OC826_RES + 4 * n, v.map_or(1, |v| v.wrapping_add(1)));
    }

    /// An object whose vtable pointer (+8 and +0xc) leads to a vtable with a
    /// fake method `{delta 0, FAKE + off}` in every 8-byte slot.
    fn oc826_object(c: &mut Cpu, obj: u32) {
        let vt = obj + 0x400;
        c.bus.write32(obj + 8, vt);
        c.bus.write32(obj + 0xc, vt);
        for off in (0..0x300u32).step_by(8) {
            c.bus.write16(vt + off, 0);
            c.bus.write32(vt + off + 4, FAKE + off);
        }
    }

    fn oc826_zero(c: &mut Cpu, addr: u32, bytes: u32) {
        for i in 0..bytes / 4 {
            c.bus.write32(addr + 4 * i, 0);
        }
    }

    /// `check` with the scratch registers, call arguments and LO/HI compared.
    fn oc826_strict(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seeds {
            let r = compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed} hard: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} soft: {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} lohi: {:#?}", r.lohi);
        }
    }

    /// A keyboard screen `self` at [`OC826_SELF`] with its owner object
    /// (`+0xd0`), text limit (`+0xd4`), cursor (`+0x16c`) and key (`+0x170`).
    fn oc826_keyboard(c: &mut Cpu, r: &mut Rng, key: u32) {
        oc826_zero(c, OC826_SELF, 0x200);
        let max = 1 + r.next_u32() % 8;
        c.bus.write32(OC826_SELF + 0xd0, OC826_OBJA);
        c.bus.write32(OC826_SELF + 0xd4, max);
        c.bus.write32(OC826_SELF + 0xe4, r.next_u32() % 2);
        c.bus.write32(OC826_SELF + 0x16c, r.next_u32() % (max + 1));
        c.bus.write32(OC826_SELF + 0x170, key);
    }

    /// `KeyboardScreen_TypeKey`: every key class (digit, letter, backspace,
    /// delete, space, done, others), caps on and off, cursor at either end.
    #[test]
    fn keyboard_screen_type_key_matches() {
        oc826_strict(
            ee::s02::f_00180840__generated,
            keyboard_screen_type_key,
            270,
            &|c, seed| {
                let mut r = oc826_machine(c, seed, 0x18_0840, 2);
                oc826_keyboard(c, &mut r, seed % 44);
                c.g[A0][0] = sx(OC826_SELF);
            },
        );
    }

    /// `KeyboardScreen_InitKeys`: the objects found or not (random results of
    /// `UIScreen_FindObject`), every unwind point.
    #[test]
    fn keyboard_screen_init_keys_matches() {
        oc826_strict(
            ee::s02::f_00180b60__generated,
            keyboard_screen_init_keys,
            40,
            &|c, seed| {
                let mut r = oc826_machine(c, seed, 0x18_0b60, 330);
                oc826_keyboard(c, &mut r, 0);
                c.g[A0][0] = sx(OC826_SELF);
                c.g[A1][0] = sx(OC826_STR);
            },
        );
    }

    /// `KeyboardScreen_HandleInput`: the cursor object found or not, and
    /// each of the six buttons being the first one pressed (or none).
    #[test]
    fn keyboard_screen_handle_input_matches() {
        oc826_strict(
            ee::s02::f_00180ef0__generated,
            keyboard_screen_handle_input,
            84,
            &|c, seed| {
                let mut r = oc826_machine(c, seed, 0x18_0ef0, 16);
                let key = r.next_u32() % 44;
                oc826_keyboard(c, &mut r, key);
                c.g[A0][0] = sx(OC826_SELF);
                c.g[A1][0] = sx(OC826_OBJB);
                // Calls: 0 cursor name, 1 hash, 2 find, [3 hide], then one
                // button test each: pressed is the first nonzero result.
                let found = seed % 5 != 0;
                oc826_force(c, 2, found.then_some(OC826_OBJA));
                let base = if found { 4 } else { 3 };
                let pressed = seed % 7;
                for j in 0..6 {
                    let v = (j == pressed).then_some(OC826_OBJA);
                    if j <= pressed {
                        oc826_force(c, base + j, v);
                    }
                }
            },
        );
    }

    /// `cFEStateCheatCharSelect_ctor`: no, some and many owned cheat
    /// characters, every slot of the face table.
    #[test]
    fn fe_state_cheat_char_select_ctor_matches() {
        oc826_strict(
            ee::s02::f_00182220__generated,
            fe_state_cheat_char_select_ctor,
            48,
            &|c, seed| {
                let mut r = oc826_machine(c, seed, 0x18_2220, 48);
                oc826_zero(c, OC826_SELF, 0x200);
                c.bus.write32(OC826_SELF + 0x10, OC826_STR);
                let slot = seed % 8;
                for i in 0..8u32 {
                    c.bus.write32(0x0044_0f78 + 4 * i, OC826_STR + 0x10 * i);
                }
                let n = [0u32, 1, 3, 6][(seed / 8) as usize % 4];
                c.bus.write32(OC826_DB + 0x50, n);
                c.bus.write32(OC826_DB + 0x28, OC826_ENTRIES);
                for i in 0..6 * 4u32 {
                    c.bus.write32(OC826_ENTRIES + 4 * i, r.next_u32());
                }
                // Calls: 0 base ctor, 1 hash, 2 face, 3 BE, 4 interface, 5 DB.
                oc826_force(c, 5, Some(OC826_DB));
                c.g[A0][0] = sx(OC826_SELF);
                c.g[A1][0] = sx(OC826_STR);
                c.g[A2][0] = sx(OC826_STR);
                c.g[A3][0] = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
                c.g[T0][0] = slot as u64;
            },
        );
    }

    /// `cFEStateCheatCharSelect_OnWidgetCreated`: the widget's hash matches
    /// each of the ten names in turn (or none).
    #[test]
    fn fe_state_cheat_char_select_on_widget_created_matches() {
        const HASH: u32 = 0x7777;
        oc826_strict(
            ee::s02::f_00182488__generated,
            fe_state_cheat_char_select_on_widget_created,
            66,
            &|c, seed| {
                let mut r = oc826_machine(c, seed, 0x18_2488, 14);
                oc826_zero(c, OC826_SELF, 0x200);
                for i in 0..6u32 {
                    c.bus.write32(0x0044_0fa0 + 4 * i, OC826_STR + 0x10 * i);
                }
                c.bus.write32(OC826_SELF + 0x6c, r.next_u32() & 0xffff_fff0);
                c.bus.write32(OC826_SELF + 0x78, r.next_u32() % 10);
                oc826_zero(c, OC826_OBJB + 0x10, 0x30);
                c.bus.write32(OC826_OBJB + 0x14, r.next_u32());
                c.bus.write32(OC826_OBJB + 0x38, HASH);
                let hit = seed % 11;
                for j in 0..=hit.min(9) {
                    let v = if j == hit { HASH } else { HASH + 1 };
                    oc826_force(c, j, Some(v));
                }
                c.g[A0][0] = sx(OC826_SELF);
                c.g[A1][0] = sx(OC826_OBJB);
            },
        );
    }

    /// `cFEStateCheatCharSelect_OnWidgetAction`: a null widget, and the
    /// messages 0 to 7 (1, 2, 5 and 6 act).
    #[test]
    fn fe_state_cheat_char_select_on_widget_action_matches() {
        oc826_strict(
            ee::s02::f_00182690__generated,
            fe_state_cheat_char_select_on_widget_action,
            64,
            &|c, seed| {
                let mut r = oc826_machine(c, seed, 0x18_2690, 12);
                oc826_zero(c, OC826_SELF, 0x200);
                oc826_zero(c, OC826_MENU, 0x100);
                c.bus.write32(OC826_SELF + 0x6c, OC826_MENU);
                c.bus.write32(OC826_SELF + 0x50, OC826_STR);
                c.bus.write32(OC826_SELF + 0x70, r.next_u32());
                for i in 0..8u32 {
                    c.bus.write32(OC826_SELF + 0x7c + 4 * i, r.next_u32());
                    c.bus.write32(OC826_SELF + 0x16c + 4 * i, r.next_u32());
                }
                c.bus.write8(OC826_MENU + 0x98, (r.next_u32() % 3) as u8);
                c.bus.write32(OC826_OBJB + 0x18, r.next_u32() % 3);
                // Message 5 makes the second call (the interface) return an object.
                oc826_force(c, 1, Some(OC826_OBJA));
                c.g[A0][0] = sx(OC826_SELF);
                c.g[A1][0] = if seed % 9 == 8 { 0 } else { sx(OC826_OBJB) };
                c.g[A2][0] = (seed % 8) as u64;
            },
        );
    }
}
