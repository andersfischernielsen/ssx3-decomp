//! Staging module oc858 (wave wave250): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc858`.

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
        0x0019b618,
        cfe_state_char_equip_refresh_item_list,
        "cFEStateCharEquip_RefreshItemList",
    ),
    (
        0x0019b7e0,
        cfe_state_char_equip_set_equip_row,
        "cFEStateCharEquip_SetEquipRow",
    ),
    (
        0x0019ba60,
        cfe_state_char_equip_set_buy_row,
        "cFEStateCharEquip_SetBuyRow",
    ),
    (
        0x0019bd48,
        cfe_state_char_equip_update_stat_preview_bar,
        "cFEStateCharEquip_UpdateStatPreviewBar",
    ),
    (
        0x0019be80,
        cfe_state_char_equip_buy_selected_item,
        "cFEStateCharEquip_BuySelectedItem",
    ),
    (
        0x0019bee8,
        cfe_state_char_equip_equip_selected_item,
        "cFEStateCharEquip_EquipSelectedItem",
    ),
];

/// `void cFEStateCharEquip_RefreshItemList(cFEStateCharEquip *self, int reset, int select)` (0x19b618):
/// hides the six name/check widgets, optionally resets the scroll menu to the
/// item count, refills the rows (buy list or equip list by mode), shows the
/// "empty" row when there is nothing, and clamps and re-selects the visible row.
pub fn cfe_state_char_equip_refresh_item_list(c: &mut Cpu) {
    guest(c, |c| {
        'b_19b7bc: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A0][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S4][0] = c.g[A1][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            let t = c.g[V0][0] == 0u64;
            c.g[S5][0] = c.g[A2][0];
            if t {
                break 'b_19b7bc;
            } else {
                c.cycles += 5;
                c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa4));
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8c));
                c.g[S3][0] = 0x5;
                c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                'l_19b660: loop {
                    c.cycles += 18;
                    c.g[A1][0] = 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0019b680);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0019b680)?;
                    c.g[A2][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A1][0] = 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0019b6a0);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0019b6a0)?;
                    if (c.g[S3][0] as i64) >= 0 {
                        c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, 0x0019b660);
                        }
                        continue 'l_19b660;
                    } else {
                        'b_19b6d8: {
                            c.cycles += 2;
                            if c.g[S4][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
                                break 'b_19b6d8;
                            } else {
                                'b_19b6cc: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x958));
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                    if t {
                                        break 'b_19b6cc;
                                    } else {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x0019b6c4);
                                        c.g[A1][0] = 0x1;
                                        call_ptr(c, 0x0039b760, 0x0019b6c4)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
                                        if t {
                                            break 'b_19b6d8;
                                        } else {
                                            break 'b_19b6cc;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x0019b6d4);
                                c.g[A1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x958));
                                call_ptr(c, 0x0039b760, 0x0019b6d4)?;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
                                break 'b_19b6d8;
                            }
                        }
                        'b_19b700: {
                            'b_19b6f4: {
                                c.cycles += 3;
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                let t = c.g[V0][0] != 0u64;

                                if t {
                                    break 'b_19b6f4;
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x0019b6ec);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x0019b598,
                                        f: ee::s03::cFEStateCharEquip_FillBuyRows,
                                    }
                                    .call(c, 0x0019b6ec)?;
                                    let t = true;
                                    c.g[A1][0] = c.g[V0][0];
                                    if t {
                                        break 'b_19b700;
                                    } else {
                                        break 'b_19b6f4;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x0019b6fc);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x0019b518,
                                f: ee::s03::cFEStateCharEquip_FillEquipRows,
                            }
                            .call(c, 0x0019b6fc)?;
                            c.g[A1][0] = c.g[V0][0];
                            break 'b_19b700;
                        }
                        'b_19b788: {
                            'b_19b77c: {
                                c.cycles += 2;
                                if c.g[A1][0] != 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                    break 'b_19b77c;
                                } else {
                                    c.cycles += 29;
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8c));
                                    c.g[A1][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0019b724);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0019b724)?;
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8c));
                                    c.g[A1][0] = 0x4c0000;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6708));
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0019b744);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0019b744)?;
                                    c.g[A0][0] = 0x460000;
                                    c.g[RA][0] = sx(0x0019b750);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x9a0));
                                    call_ptr(c, 0x00317670, 0x0019b750)?;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8c));
                                    c.g[RA][0] = sx(0x0019b75c);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a1000, 0x0019b75c)?;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                    c.g[V1][0] = 0x1;
                                    c.g[A2][0] = 0u64;
                                    c.g[A1][0] = 0x1;
                                    c.sb((c.g[A0][0] as u32).wrapping_add(0x96), c.g[V1][0]);
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                    let t = true;
                                    c.sb((c.g[V0][0] as u32).wrapping_add(0x98), 0u64);
                                    if t {
                                        break 'b_19b788;
                                    } else {
                                        break 'b_19b77c;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[A0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x98));
                            c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
                            c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                            break 'b_19b788;
                        }
                        c.cycles += 8;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[A1][0] as i64)) as u64;
                        if c.g[V1][0] == 0 {
                            c.g[A2][0] = c.g[A0][0];
                        }
                        c.g[V0][0] = 0xffffffffffffffff;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            c.g[A2][0] = 0u64;
                        }
                        let t = c.g[S5][0] == 0u64;
                        if c.g[S4][0] != 0 {
                            c.g[A2][0] = 0u64;
                        }
                        if t {
                            break 'b_19b7bc;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                            c.g[A1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x98));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            c.g[RA][0] = sx(0x0019b7bc);
                            c.g[A1][0] = c.g[A1][0] & 0xff;
                            call_ptr(c, 0x0039b960, 0x0019b7bc)?;
                            break 'b_19b7bc;
                        }
                    }
                }
            }
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateCharEquip_SetEquipRow(cFEStateCharEquip *self, int row)` (0x19b7e0):
/// fills one row of the equip list: the item's name (or text id), and the
/// price-delta/colour state found by trial-toggling the bolt in a copy of the
/// character profile (which is restored afterwards).
pub fn cfe_state_char_equip_set_equip_row(c: &mut Cpu) {
    guest(c, |c| {
        'b_19b820: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x8f0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x8e0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x8d0));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x8c0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x8b0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x8a0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x890));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x880));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x870));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x860));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x850), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            let t = c.g[V1][0] == 0u64;
            c.g[V0][0] = 0u64;
            if t {
                break 'b_19b820;
            } else {
                c.cycles += 1;
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x98));
                break 'b_19b820;
            }
        }
        'b_19b990: {
            'b_19b878: {
                c.cycles += 20;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa78));
                c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x840), c.g[A1][0]);
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[FP][0] = c.g[A0][0];
                c.g[V1][0] = 0xfffffffffffe0000;
                c.g[S4][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V1][0] = c.g[V1][0] | 0x7961;
                c.g[A2][0] = c.g[V0][0];
                c.g[S6][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xa4));
                c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x34));
                c.g[V0][0] = c.g[V0][0] & 0x20;
                let t = c.g[V0][0] == 0u64;
                c.g[S7][0] = 0u64;
                if t {
                    break 'b_19b878;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[S7][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xaa0));
                    if t {
                        break 'b_19b990;
                    } else {
                        break 'b_19b878;
                    }
                }
            }
            'b_19b8d0: {
                'b_19b8cc: {
                    c.cycles += 18;
                    c.g[RA][0] = sx(0x0019b880);
                    call_ptr(c, 0x0014dc80, 0x0019b880)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0019b88c);
                    c.g[A1][0] = 0x9;
                    call_ptr(c, 0x0014dd58, 0x0019b88c)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                    c.g[RA][0] = sx(0x0019b8a0);
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x0014ad28, 0x0019b8a0)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x288));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        break 'b_19b8cc;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                        let t = true;
                        c.g[S2][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        if t {
                            break 'b_19b8d0;
                        } else {
                            break 'b_19b8cc;
                        }
                    }
                }
                c.cycles += 1;
                c.g[S2][0] = 0u64;
                break 'b_19b8d0;
            }
            c.cycles += 6;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x290));
            c.g[V1][0] = c.g[SP][0];
            c.g[A0][0] = 0x20c;
            c.g[A3][0] = 0xffffffffffffffff;
            c.g[A2][0] = 0xffffffffffffffff;
            'l_19b8e8: loop {
                c.cycles += 7;
                c.sh(c.g[V1][0] as u32, c.g[A3][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                c.sh((c.g[V1][0] as u32).wrapping_add(0x2), 0u64);
                let t = c.g[A0][0] != c.g[A2][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019b8e8);
                    }
                    continue 'l_19b8e8;
                } else {
                    'b_19b97c: {
                        c.cycles += 18;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x0019b910);
                        c.g[A2][0] = 0x834;
                        call_ptr(c, 0x003e6574, 0x0019b910)?;
                        c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                        c.g[A0][0] = c.g[S3][0];
                        c.g[RA][0] = sx(0x0019b920);
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                        call_ptr(c, 0x0014b478, 0x0019b920)?;
                        c.g[T0][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0x2));
                        c.g[S5][0] = c.g[V0][0];
                        c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                        c.g[A0][0] = c.g[S3][0];
                        c.g[T0][0] = c.g[T0][0] & 0x4;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                        c.g[A3][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x4));
                        c.g[RA][0] = sx(0x0019b944);
                        c.g[T0][0] = (c.g[T0][0] < 0x1u64) as u64;
                        call_ptr(c, 0x0014afb0, 0x0019b944)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S3][0];
                        if t {
                            break 'b_19b97c;
                        } else {
                            c.cycles += 11;
                            c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                            c.g[RA][0] = sx(0x0019b958);
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                            call_ptr(c, 0x0014b478, 0x0019b958)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x840));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S5][0] as u32));
                            c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                            c.g[V0][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0x2));
                            c.g[V0][0] = c.g[V0][0] & 0x4;
                            if c.g[V0][0] == 0u64 {
                                c.g[S7][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa9c));
                                break 'b_19b97c;
                            } else {
                                c.cycles += 1;
                                c.g[S7][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa98));
                                break 'b_19b97c;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    c.g[RA][0] = sx(0x0019b990);
                    c.g[A3][0] = c.g[SP][0];
                    call_ptr(c, 0x0014acb0, 0x0019b990)?;
                    break 'b_19b990;
                }
            }
        }
        'b_19b9e4: {
            c.cycles += 2;
            let t = c.g[S6][0] == 0u64;

            if t {
                break 'b_19b9e4;
            } else {
                'b_19b9d0: {
                    'b_19b9c4: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x34));
                        c.g[V0][0] = c.g[V0][0] & 0x20;
                        if c.g[V0][0] == 0u64 {
                            c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x14));
                            break 'b_19b9c4;
                        } else {
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x0019b9b0);
                            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x14));
                            call_ptr(c, 0x00317670, 0x0019b9b0)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x0019b9bc);
                            c.g[A0][0] = c.g[S6][0];
                            call_ptr(c, 0x003a1000, 0x0019b9bc)?;
                            let t = true;
                            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                            if t {
                                break 'b_19b9d0;
                            } else {
                                break 'b_19b9c4;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0019b9cc);
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x003a0c50, 0x0019b9cc)?;
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                    break 'b_19b9d0;
                }
                c.cycles += 5;
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0019b9e4);
                c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0019b9e4)?;
                break 'b_19b9e4;
            }
        }
        'b_19ba34: {
            c.cycles += 2;
            let t = c.g[S1][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x8e0));
            if t {
                break 'b_19ba34;
            } else {
                'b_19ba30: {
                    'b_19ba18: {
                        c.cycles += 2;
                        let t = c.g[S7][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_19ba18;
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x0019ba08);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0019ba08)?;
                            c.g[V0][0] = 0xffffffffffffffff;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x7c), c.g[S7][0]);
                            let t = true;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                            if t {
                                break 'b_19ba30;
                            } else {
                                break 'b_19ba18;
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0019ba30);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0019ba30)?;
                    break 'b_19ba30;
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x8e0));
                break 'b_19ba34;
            }
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x8d0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x8c0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x8b0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x8a0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x890));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x880));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x870));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x860));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x850));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8f0));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateCharEquip_SetBuyRow(cFEStateCharEquip *self, int row)` (0x19ba60):
/// fills one row of the buy list: the item's name and a coloured dollar icon
/// (green when affordable, red otherwise, plus when already owned).
pub fn cfe_state_char_equip_set_buy_row(c: &mut Cpu) {
    guest(c, |c| {
        'b_19ba88: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[A2][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x5c));
            let t = c.g[A0][0] == 0u64;
            c.g[V0][0] = 0u64;
            if t {
                break 'b_19ba88;
            } else {
                c.cycles += 1;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x98));
                break 'b_19ba88;
            }
        }
        'b_19bae4: {
            'b_19bac0: {
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[S0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa4));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                c.g[V0][0] = c.g[V0][0] & 0x20;
                let t = c.g[V0][0] == 0u64;
                c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                if t {
                    break 'b_19bac0;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[S2][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xaa0));
                    if t {
                        break 'b_19bae4;
                    } else {
                        break 'b_19bac0;
                    }
                }
            }
            'b_19bad0: {
                c.cycles += 3;
                c.g[V1][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0xe));
                let t = (c.g[V1][0] as i64) <= 0;
                c.g[V0][0] = 0xa;
                if t {
                    break 'b_19bad0;
                } else {
                    c.cycles += 1;
                    c.mult(0, 3, 3, 2);
                    break 'b_19bad0;
                }
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc0));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V0][0] == 0u64 {
                c.g[S2][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xa90));
                break 'b_19bae4;
            } else {
                c.cycles += 1;
                c.g[S2][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xa94));
                break 'b_19bae4;
            }
        }
        'b_19bb38: {
            c.cycles += 2;
            let t = c.g[S1][0] == 0u64;

            if t {
                break 'b_19bb38;
            } else {
                'b_19bb24: {
                    'b_19bb18: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                        c.g[V0][0] = c.g[V0][0] & 0x20;
                        if c.g[V0][0] == 0u64 {
                            c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                            break 'b_19bb18;
                        } else {
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x0019bb04);
                            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                            call_ptr(c, 0x00317670, 0x0019bb04)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x0019bb10);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x003a1000, 0x0019bb10)?;
                            let t = true;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            if t {
                                break 'b_19bb24;
                            } else {
                                break 'b_19bb18;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0019bb20);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x003a0c50, 0x0019bb20)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    break 'b_19bb24;
                }
                c.cycles += 5;
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0019bb38);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0019bb38)?;
                break 'b_19bb38;
            }
        }
        'b_19bb88: {
            c.cycles += 2;
            if c.g[S0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_19bb88;
            } else {
                'b_19bb84: {
                    'b_19bb6c: {
                        c.cycles += 2;
                        let t = c.g[S2][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_19bb6c;
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x0019bb5c);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0019bb5c)?;
                            c.g[V0][0] = 0xffffffffffffffff;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), c.g[S2][0]);
                            let t = true;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                            if t {
                                break 'b_19bb84;
                            } else {
                                break 'b_19bb6c;
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0019bb84);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0019bb84)?;
                    break 'b_19bb84;
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_19bb88;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateCharEquip_UpdateStatPreviewBar(cFEStateCharEquip *self, int delta)` (0x19bd48):
/// sets the points bar's colour (invalid when over the limit), visibility
/// and vertical scale from `(polys + delta - 1500) / (max - 1500)`, clamped to 0..1.
pub fn cfe_state_char_equip_update_stat_preview_bar(c: &mut Cpu) {
    guest(c, |c| {
        'b_19be68: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1818));
            if t {
                break 'b_19be68;
            } else {
                'b_19bdc4: {
                    'b_19bdb0: {
                        c.cycles += 16;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc4));
                        c.g[A2][0] = c.g[V0][0];
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        c.g[A0][0] = c.g[V1][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5dc));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5dc));
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.mtc1(3, 0);
                        c.cvt_s_w(0, 0);
                        c.div_s(20, 1, 0);
                        c.c_lt_s(2, 20);
                        let t = !c.fcc();

                        if t {
                            break 'b_19bdb0;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.mov_s(20, 2);
                            if t {
                                break 'b_19bdc4;
                            } else {
                                break 'b_19bdb0;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.mtc1(0, 0);
                    c.c_lt_s(20, 0);
                    if c.fcc() {
                        c.mov_s(20, 0);
                        break 'b_19bdc4;
                    } else {
                        break 'b_19bdc4;
                    }
                }
                'b_19be1c: {
                    'b_19bdf8: {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = 0x4c0000;
                        if t {
                            break 'b_19bdf8;
                        } else {
                            c.cycles += 9;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x66e8));
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x0019bdf0);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0019bdf0)?;
                            let t = true;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                            if t {
                                break 'b_19be1c;
                            } else {
                                break 'b_19bdf8;
                            }
                        }
                    }
                    c.cycles += 9;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                    c.g[A1][0] = 0x4c0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x66f8));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0019be18);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0019be18)?;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                    break 'b_19be1c;
                }
                c.cycles += 19;
                c.g[A1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0019be34);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0019be34)?;
                c.neg_s(1, 20);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.swc1(0, c.g[SP][0] as u32);
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(3, c.g[SP][0] as u32);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                break 'b_19be68;
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateCharEquip_BuySelectedItem(cFEStateCharEquip *self)` (0x19be80):
/// asks the profile interface to buy the selected item and stores the new money.
pub fn cfe_state_char_equip_buy_selected_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
        c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x95));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[RA][0] = sx(0x0019beac);
        c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
        call_ptr(c, 0x0014dc80, 0x0019beac)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0019beb8);
        c.g[A1][0] = 0x9;
        call_ptr(c, 0x0014dd58, 0x0019beb8)?;
        c.g[A3][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
        c.g[T0][0] = 0x1;
        c.g[RA][0] = sx(0x0019bed0);
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
        call_ptr(c, 0x0014b560, 0x0019bed0)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `int cFEStateCharEquip_EquipSelectedItem(cFEStateCharEquip *self)` (0x19bee8):
/// toggles the selected item's equipped flag in the profile; 1 if it changed.
pub fn cfe_state_char_equip_equip_selected_item(c: &mut Cpu) {
    guest(c, |c| {
        'b_19bfc8: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x958));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[V0][0] = 0u64;
            if t {
                break 'b_19bfc8;
            } else {
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                c.g[V0][0] = c.g[V0][0] & 0x4;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_19bfc8;
                } else {
                    'b_19bf88: {
                        'b_19bf84: {
                            c.cycles += 18;
                            c.g[RA][0] = sx(0x0019bf38);
                            call_ptr(c, 0x0014dc80, 0x0019bf38)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x0019bf44);
                            c.g[A1][0] = 0x9;
                            call_ptr(c, 0x0014dd58, 0x0019bf44)?;
                            c.g[S2][0] = c.g[V0][0];
                            c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                            c.g[RA][0] = sx(0x0019bf58);
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x0014ad28, 0x0019bf58)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x288));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                            let t = (c.g[V0][0] as i64) < 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            if t {
                                break 'b_19bf84;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                let t = true;
                                c.g[V0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                if t {
                                    break 'b_19bf88;
                                } else {
                                    break 'b_19bf84;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = 0u64;
                        break 'b_19bf88;
                    }
                    'b_19bfb8: {
                        c.cycles += 10;
                        c.g[T0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A3][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[T0][0] = c.g[T0][0] & 0x4;
                        c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                        c.g[RA][0] = sx(0x0019bfa8);
                        c.g[T0][0] = (c.g[T0][0] < 0x1u64) as u64;
                        call_ptr(c, 0x0014afb0, 0x0019bfa8)?;
                        if c.g[V0][0] != 0u64 {
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                            break 'b_19bfb8;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_19bfc8;
                            } else {
                                break 'b_19bfb8;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0019bfc4);
                    c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    call_ptr(c, 0x0014aea8, 0x0019bfc4)?;
                    c.g[V0][0] = 0x1;
                    break 'b_19bfc8;
                }
            }
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
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Gate of this module's callee fakes (the machine's own RAM).
    const OC858_CTRL: u32 = DATA + 0x21_0000;
    const OC858_MAGIC: u32 = 0x0c85_8858;

    // Fixture layout (all inside DATA..DATA+0xfffff).
    const THIS: u32 = DATA + 0x1000;
    const ITEMS: u32 = DATA + 0x2000;
    const MENU: u32 = DATA + 0x3000;
    const WIDGETS: u32 = DATA + 0x5000;
    const VT: u32 = DATA + 0x8000;
    const PROF: u32 = DATA + 0x9000;
    const TBL: u32 = DATA + 0xa000;
    const NAMES: u32 = DATA + 0xb000;

    // cFEStateCharEquip fields the functions read.
    const F_SLOT: u32 = 0x44;
    const F_MENU: u32 = 0x5c;
    const F_BAR: u32 = 0x74;
    const F_NAMES: u32 = 0x8c;
    const F_ICONS: u32 = 0xa4;
    const F_TEAM: u32 = 0xbc;
    const F_MONEY: u32 = 0xc0;
    const F_POLYS: u32 = 0xc4;
    const F_MODE: u32 = 0xc8;
    const F_ITEMS: u32 = 0x124;
    const F_NUM_ITEMS: u32 = 0x958;

    /// Forced results of fake callees (flag word, value word), by index.
    const FORCE_PROFILE: u32 = 0;
    const FORCE_TOGGLE: u32 = 1;
    const FORCE_FILL_BUY: u32 = 2;
    const FORCE_FILL_EQUIP: u32 = 3;

    macro_rules! oc858_fake {
        ($fake:ident, $prev:ident, $gen:path, $addr:expr, $idx:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC858_CTRL) != OC858_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
                if $idx != 99 && c.bus.read32(OC858_CTRL + 0x100 + 8 * $idx) != 0 {
                    c.g[V0][0] = sx(c.bus.read32(OC858_CTRL + 0x104 + 8 * $idx));
                }
            }
        };
    }

    oc858_fake!(
        oc858_fake_acb0,
        OC858_PREV_ACB0,
        ee::s01::f_0014acb0__generated,
        0x0014_acb0,
        99
    );
    oc858_fake!(
        oc858_fake_ad28,
        OC858_PREV_AD28,
        ee::s01::f_0014ad28__generated,
        0x0014_ad28,
        FORCE_PROFILE
    );
    oc858_fake!(
        oc858_fake_afb0,
        OC858_PREV_AFB0,
        ee::s01::f_0014afb0__generated,
        0x0014_afb0,
        FORCE_TOGGLE
    );
    oc858_fake!(
        oc858_fake_b478,
        OC858_PREV_B478,
        ee::s01::f_0014b478__generated,
        0x0014_b478,
        99
    );
    oc858_fake!(
        oc858_fake_b560,
        OC858_PREV_B560,
        ee::s01::f_0014b560__generated,
        0x0014_b560,
        99
    );
    oc858_fake!(
        oc858_fake_dc80,
        OC858_PREV_DC80,
        ee::s01::f_0014dc80__generated,
        0x0014_dc80,
        99
    );
    oc858_fake!(
        oc858_fake_dd58,
        OC858_PREV_DD58,
        ee::s01::f_0014dd58__generated,
        0x0014_dd58,
        99
    );
    oc858_fake!(
        oc858_fake_aea8,
        OC858_PREV_AEA8,
        ee::s01::f_0014aea8__generated,
        0x0014_aea8,
        99
    );
    oc858_fake!(
        oc858_fake_hash,
        OC858_PREV_HASH,
        ee::s10::f_00317670__generated,
        0x0031_7670,
        99
    );
    oc858_fake!(
        oc858_fake_scrollable,
        OC858_PREV_SCROLLABLE,
        ee::s13::f_0039b760__generated,
        0x0039_b760,
        99
    );
    oc858_fake!(
        oc858_fake_select,
        OC858_PREV_SELECT,
        ee::s13::f_0039b960__generated,
        0x0039_b960,
        99
    );
    oc858_fake!(
        oc858_fake_set_id,
        OC858_PREV_SET_ID,
        ee::s13::f_003a1000__generated,
        0x003a_1000,
        99
    );
    oc858_fake!(
        oc858_fake_set_ascii,
        OC858_PREV_SET_ASCII,
        ee::s13::f_003a0c50__generated,
        0x003a_0c50,
        99
    );
    oc858_fake!(
        oc858_fake_memcopy,
        OC858_PREV_MEMCOPY,
        ee::s14::f_003e6574__generated,
        0x003e_6574,
        99
    );
    oc858_fake!(
        oc858_fake_fill_equip,
        OC858_PREV_FILL_EQUIP,
        ee::s03::f_0019b518__generated,
        0x0019_b518,
        FORCE_FILL_EQUIP
    );
    oc858_fake!(
        oc858_fake_fill_buy,
        OC858_PREV_FILL_BUY,
        ee::s03::f_0019b598__generated,
        0x0019_b598,
        FORCE_FILL_BUY
    );

    fn oc858_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014ACB0),
            oc858_fake_acb0,
            &OC858_PREV_ACB0,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014AD28),
            oc858_fake_ad28,
            &OC858_PREV_AD28,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014AFB0),
            oc858_fake_afb0,
            &OC858_PREV_AFB0,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014B478),
            oc858_fake_b478,
            &OC858_PREV_B478,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014B560),
            oc858_fake_b560,
            &OC858_PREV_B560,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc858_fake_dc80,
            &OC858_PREV_DC80,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc858_fake_dd58,
            &OC858_PREV_DD58,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014AEA8),
            oc858_fake_aea8,
            &OC858_PREV_AEA8,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc858_fake_hash,
            &OC858_PREV_HASH,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039B760),
            oc858_fake_scrollable,
            &OC858_PREV_SCROLLABLE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039B960),
            oc858_fake_select,
            &OC858_PREV_SELECT,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A1000),
            oc858_fake_set_id,
            &OC858_PREV_SET_ID,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0C50),
            oc858_fake_set_ascii,
            &OC858_PREV_SET_ASCII,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6574),
            oc858_fake_memcopy,
            &OC858_PREV_MEMCOPY,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_0019B518),
            oc858_fake_fill_equip,
            &OC858_PREV_FILL_EQUIP,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_0019B598),
            oc858_fake_fill_buy,
            &OC858_PREV_FILL_BUY,
        );
    }

    /// A machine with the callee fakes installed and gated on, the widget
    /// vtable set up, and `calls` as the range of fake-call indices at which a
    /// call may unwind.
    fn oc858_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc858_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC858_CTRL, OC858_MAGIC);
        // Widget methods: +0x48 setVisible(delta, fn), +0x58 setColour.
        c.bus.write16(VT + 0x48, 0x10);
        c.bus.write32(VT + 0x4c, FAKE);
        c.bus.write16(VT + 0x58, 0x20);
        c.bus.write32(VT + 0x5c, FAKE + 0x10);
        c.g[GP][0] = sx(crate::difftest::GAME_GP);
        r
    }

    fn oc858_force(c: &mut Cpu, idx: u32, v: u32) {
        c.bus.write32(OC858_CTRL + 0x100 + 8 * idx, 1);
        c.bus.write32(OC858_CTRL + 0x104 + 8 * idx, v);
    }

    fn oc858_widget(c: &mut Cpu, k: u32) -> u32 {
        let w = WIDGETS + 0x100 * k;
        c.bus.write32(w + 8, VT);
        w
    }

    /// The state object with a menu (current row, scroll offset), items,
    /// and all twelve row widgets.
    fn oc858_state(c: &mut Cpu, r: &mut Rng, with_menu: bool) {
        c.bus
            .write32(THIS + F_MENU, if with_menu { MENU } else { 0 });
        c.bus.write8(MENU + 0x95, (r.next_u32() % 10) as u8);
        c.bus.write8(MENU + 0x96, 0);
        c.bus.write8(MENU + 0x98, (r.next_u32() % 4) as u8);
        c.bus.write8(THIS + F_SLOT, (r.next_u32() % 4) as u8);
        c.bus.write32(THIS + F_TEAM, r.next_u32() % 3);
        for k in 0..6 {
            let n = oc858_widget(c, k);
            c.bus.write32(THIS + F_NAMES + 4 * k, n);
            let i = oc858_widget(c, 6 + k);
            c.bus.write32(THIS + F_ICONS + 4 * k, i);
        }
        for k in 0..12u32 {
            c.bus.write32(THIS + F_ITEMS + 4 * k, ITEMS + 0x40 * k);
        }
        // The shared widget (the bar) of UpdateStatPreviewBar.
        let bar = oc858_widget(c, 12);
        c.bus.write32(THIS + F_BAR, bar);
    }

    fn oc858_item(c: &mut Cpu, r: &mut Rng, k: u32, flags: u32) {
        let it = ITEMS + 0x40 * k;
        c.bus.write32(it + 0x34, flags);
        c.bus.write16(it + 4, (r.next_u32() % 6) as u16);
        c.bus.write16(
            it + 0xe,
            [0u16, 0xfff6, 5, 40, 300][(r.next_u32() % 5) as usize],
        );
        c.bus.write32(it + 0x14, NAMES + 16 * k);
    }

    /// The profile of `FUNC_PROFILE`: a table of slot indices by item id
    /// (some negative: not in the profile) and the flag entries.
    fn oc858_profile(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(PROF + 0x288, TBL);
        for id in 0..6u32 {
            let v: i16 = if (seed / 11 + id) % 4 == 0 {
                -1
            } else {
                (r.next_u32() % 8) as i16
            };
            c.bus.write16(TBL + 2 * id, v as u16);
        }
        for k in 0..8u32 {
            c.bus.write32(PROF + 0x290 + 4 * k, r.next_u32());
        }
        oc858_force(c, FORCE_PROFILE, PROF);
    }

    /// `RefreshItemList`: with and without a menu, reset and select flags,
    /// both modes, an empty list or rows, and a clamped visible row.
    #[test]
    fn cfe_state_char_equip_refresh_item_list_matches() {
        check(
            ee::s03::f_0019b618__generated,
            cfe_state_char_equip_refresh_item_list,
            300,
            &|c, seed| {
                let mut r = oc858_machine(c, seed, 0x0019_b618, 20);
                oc858_state(c, &mut r, seed % 13 != 0);
                c.bus
                    .write32(THIS + F_MODE, [0u32, 1, 0x77][((seed / 4) % 3) as usize]);
                c.bus.write32(
                    THIS + F_NUM_ITEMS,
                    if (seed / 8) % 3 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 9
                    },
                );
                let n = [0u32, 1, 3, 9][((seed / 24) % 4) as usize];
                oc858_force(c, FORCE_FILL_BUY, n);
                oc858_force(c, FORCE_FILL_EQUIP, n);
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = [0u64, 1, 7][(seed % 3) as usize];
                c.g[A2][0] = [0u64, 1, 5][((seed / 2) % 3) as usize];
                c.pc = 0x0019_b618;
            },
        );
    }

    /// `SetEquipRow`: owned items, items found or missing in the profile,
    /// toggles that succeed or fail, and rows with or without widgets.
    #[test]
    fn cfe_state_char_equip_set_equip_row_matches() {
        check(
            ee::s03::f_0019b7e0__generated,
            cfe_state_char_equip_set_equip_row,
            300,
            &|c, seed| {
                let mut r = oc858_machine(c, seed, 0x0019_b7e0, 14);
                oc858_state(c, &mut r, seed % 17 != 0);
                let row = seed % 6;
                let top = c.bus.read8(MENU + 0x98) as u32;
                for k in 0..12 {
                    let owned = seed % 5 == 0 && k == top + row;
                    let bits = r.next_u32() & !0x20;
                    oc858_item(c, &mut r, k, bits | if owned { 0x20 } else { 0 });
                }
                oc858_profile(c, &mut r, seed);
                oc858_force(c, FORCE_TOGGLE, (seed / 3) % 2);
                if (seed / 7) % 5 == 0 {
                    c.bus.write32(THIS + F_NAMES + 4 * row, 0);
                }
                if (seed / 3) % 7 == 0 {
                    c.bus.write32(THIS + F_ICONS + 4 * row, 0);
                }
                for off in [0xa90u32, 0xa94, 0xa98, 0xa9c, 0xaa0] {
                    let v = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(THIS + off, v);
                }
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = row as u64;
                c.pc = 0x0019_b7e0;
            },
        );
    }

    /// `SetBuyRow`: owned items (plus icon), affordable and too-expensive
    /// ones, free ones, and rows without text or icon widget.
    #[test]
    fn cfe_state_char_equip_set_buy_row_matches() {
        check(
            ee::s03::f_0019ba60__generated,
            cfe_state_char_equip_set_buy_row,
            300,
            &|c, seed| {
                let mut r = oc858_machine(c, seed, 0x0019_ba60, 8);
                oc858_state(c, &mut r, seed % 17 != 0);
                let row = seed % 6;
                let top = c.bus.read8(MENU + 0x98) as u32;
                for k in 0..12 {
                    let owned = seed % 5 == 0 && k == top + row;
                    let bits = r.next_u32() & !0x20;
                    oc858_item(c, &mut r, k, bits | if owned { 0x20 } else { 0 });
                }
                let money = [0u32, 5, 10, 50, 400, 0xffff_fff0][((seed / 5) % 6) as usize];
                c.bus.write32(THIS + F_MONEY, money);
                if (seed / 7) % 5 == 0 {
                    c.bus.write32(THIS + F_NAMES + 4 * row, 0);
                }
                if (seed / 3) % 7 == 0 {
                    c.bus.write32(THIS + F_ICONS + 4 * row, 0);
                }
                for off in [0xa90u32, 0xa94, 0xaa0] {
                    let v = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(THIS + off, v);
                }
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = row as u64;
                c.pc = 0x0019_ba60;
            },
        );
    }

    /// `UpdateStatPreviewBar`: no bar, bars under, inside and over the
    /// polygon limit (clamped ratio at both ends).
    #[test]
    fn cfe_state_char_equip_update_stat_preview_bar_matches() {
        check(
            ee::s03::f_0019bd48__generated,
            cfe_state_char_equip_update_stat_preview_bar,
            300,
            &|c, seed| {
                let mut r = oc858_machine(c, seed, 0x0019_bd48, 4);
                oc858_state(c, &mut r, true);
                if seed % 9 == 0 {
                    c.bus.write32(THIS + F_BAR, 0);
                }
                let limit = [3000u32, 4000, 1500, 2000][(seed % 4) as usize];
                c.bus.write32(crate::difftest::GAME_GP - 0x1818, limit);
                c.bus.write32(
                    THIS + F_POLYS,
                    [0u32, 1000, 2500, 3900, 5000][((seed / 4) % 5) as usize],
                );
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] =
                    [0u64, 200, 0xffff_ffff_ffff_fe00, 1500, 3000][((seed / 20) % 5) as usize];
                c.pc = 0x0019_bd48;
            },
        );
    }

    /// `BuySelectedItem`: the money word after the buy call.
    #[test]
    fn cfe_state_char_equip_buy_selected_item_matches() {
        check(
            ee::s03::f_0019be80__generated,
            cfe_state_char_equip_buy_selected_item,
            200,
            &|c, seed| {
                let mut r = oc858_machine(c, seed, 0x0019_be80, 4);
                oc858_state(c, &mut r, true);
                for k in 0..12 {
                    oc858_item(c, &mut r, k, 0);
                }
                c.bus.write32(THIS + F_MONEY, r.next_u32());
                c.g[A0][0] = sx(THIS);
                c.pc = 0x0019_be80;
            },
        );
    }

    /// `EquipSelectedItem`: no items, an item that cannot be toggled, a
    /// missing profile entry, and a toggle that fails or succeeds.
    #[test]
    fn cfe_state_char_equip_equip_selected_item_matches() {
        check(
            ee::s03::f_0019bee8__generated,
            cfe_state_char_equip_equip_selected_item,
            300,
            &|c, seed| {
                let mut r = oc858_machine(c, seed, 0x0019_bee8, 6);
                oc858_state(c, &mut r, true);
                for k in 0..12 {
                    let bits = r.next_u32() & !4;
                    let flags = if seed % 4 == 1 { 0 } else { 4 | bits };
                    oc858_item(c, &mut r, k, flags);
                }
                let n = match seed % 6 {
                    0 => 0,
                    1 => 0xffff_fffe,
                    _ => 1 + r.next_u32() % 5,
                };
                c.bus.write32(THIS + F_NUM_ITEMS, n);
                oc858_profile(c, &mut r, seed);
                oc858_force(c, FORCE_TOGGLE, (seed / 6) % 2);
                c.g[A0][0] = sx(THIS);
                c.pc = 0x0019_bee8;
            },
        );
    }
}
