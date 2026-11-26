//! Staging module oc847 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc847`.

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
        0x0019_0f48,
        cfe_state_rider_bio_fill_faves,
        "cFEStateRiderBio_FillFaves",
    ),
    (
        0x0019_10c0,
        cfe_state_rider_bio_fill_qna,
        "cFEStateRiderBio_FillQNA",
    ),
    (
        0x0019_1e48,
        cfe_state_rule_select_on_widget_notify,
        "cFEStateRuleSelect_OnWidgetNotify",
    ),
    (
        0x0019_2088,
        cfe_state_rule_select_set_rule_value,
        "cFEStateRuleSelect_SetRuleValue",
    ),
    (
        0x0019_2380,
        cfe_state_rule_select_apply_rule_dependencies,
        "cFEStateRuleSelect_ApplyRuleDependencies",
    ),
    (
        0x0019_27b0,
        cfe_state_bragging_rights_update_row_colors,
        "cFEStateBraggingRights_UpdateRowColors",
    ),
];

/// `void cFEStateRiderBio_FillFaves(self)` (0x00190f48): copies the 12-entry (10 rows of 12 pointers) name table at 0x45f2d0 to the stack, then for each of the 12 widgets `<n>` (sprintf of the format at 0x4a1680) found on the screen at `+0x40` sets its text from the row `+0x48` of the table.
pub fn cfe_state_rider_bio_fill_faves(c: &mut Cpu) {
    guest(c, |c| {
        'b_191020: {
            'b_190fe4: {
                c.cycles += 15;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x270));
                c.g[V0][0] = 0x460000;
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x220));
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xd30));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x260));
                c.g[S4][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x250));
                c.g[A1][0] = c.g[SP][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x240));
                c.g[V0][0] = c.g[V1][0] & 0x7;
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x230));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x210));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x200));
                let t = c.g[V0][0] == 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x1f0), c.g[RA][0]);
                if t {
                    break 'b_190fe4;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1e0));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1e0));
                    c.g[S6][0] = 0x4a0000;
                    'l_190f90: loop {
                        c.cycles += 19;
                        c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(6, c.g[V1][0] as u32);
                        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                        c.sdr(6, c.g[A1][0] as u32);
                        c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
                        c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                        c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
                        c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
                        c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
                        c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00190f90);
                            }
                            continue 'l_190f90;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[S3][0] = c.g[A0][0];
                            if t {
                                break 'b_191020;
                            } else {
                                break 'b_190fe4;
                            }
                        }
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1e0));
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1e0));
            c.g[S6][0] = 0x4a0000;
            'l_190ff0: loop {
                c.cycles += 11;
                c.g[A2][0] = c.ld(c.g[V1][0] as u32);
                c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A1][0] as u32, c.g[A2][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00190ff0);
                    }
                    continue 'l_190ff0;
                } else {
                    c.cycles += 1;
                    c.g[S3][0] = c.g[A0][0];
                    break 'b_191020;
                }
            }
        }
        c.cycles += 4;
        c.g[S2][0] = 0u64;
        c.g[S5][0] = 0x30;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
        'l_191030: loop {
            'b_191088: {
                c.cycles += 13;
                c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1680));
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00191040);
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x00417828, 0x00191040)?;
                c.g[RA][0] = sx(0x00191048);
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x00317670, 0x00191048)?;
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x00191054);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x00191054)?;
                c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[S1][0] = c.g[V0][0];
                let t = c.g[S1][0] == 0u64;
                c.g[S2][0] = c.g[S0][0];
                if t {
                    break 'b_191088;
                } else {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x48));
                    c.mult(0, 2, 2, 21);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[RA][0] = sx(0x0019107c);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    call_ptr(c, 0x00317670, 0x0019107c)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00191088);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00191088)?;
                    break 'b_191088;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 12i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00191030);
                }
                continue 'l_191030;
            } else {
                c.cycles += 10;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x260));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x250));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x240));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x230));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x220));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x210));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x200));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x1f0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x270));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEStateRiderBio_FillQNA(self)` (0x001910c0): like FillFaves for the 4 question widgets (format at 0x4a1688, 4-entry rows of the table at 0x45f730).
pub fn cfe_state_rider_bio_fill_qna(c: &mut Cpu) {
    guest(c, |c| {
        'b_191194: {
            'b_191158: {
                c.cycles += 14;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x120));
                c.g[V0][0] = 0x460000;
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8d0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.g[S4][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[A1][0] = c.g[SP][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.g[V0][0] = c.g[V1][0] & 0x7;
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
                let t = c.g[V0][0] == 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[RA][0]);
                if t {
                    break 'b_191158;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xa0));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[S5][0] = 0x4a0000;
                    'l_191104: loop {
                        c.cycles += 19;
                        c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(6, c.g[V1][0] as u32);
                        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                        c.sdr(6, c.g[A1][0] as u32);
                        c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
                        c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                        c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
                        c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
                        c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
                        c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00191104);
                            }
                            continue 'l_191104;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[S3][0] = c.g[A0][0];
                            if t {
                                break 'b_191194;
                            } else {
                                break 'b_191158;
                            }
                        }
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xa0));
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S5][0] = 0x4a0000;
            'l_191164: loop {
                c.cycles += 11;
                c.g[A2][0] = c.ld(c.g[V1][0] as u32);
                c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A1][0] as u32, c.g[A2][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00191164);
                    }
                    continue 'l_191164;
                } else {
                    c.cycles += 1;
                    c.g[S3][0] = c.g[A0][0];
                    break 'b_191194;
                }
            }
        }
        c.cycles += 3;
        c.g[S2][0] = 0u64;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
        'l_1911a0: loop {
            'b_1911f8: {
                c.cycles += 13;
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1688));
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001911b0);
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x00417828, 0x001911b0)?;
                c.g[RA][0] = sx(0x001911b8);
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x00317670, 0x001911b8)?;
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001911c4);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001911c4)?;
                c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[S1][0] = c.g[V0][0];
                let t = c.g[S1][0] == 0u64;
                c.g[S2][0] = c.g[S0][0];
                if t {
                    break 'b_1911f8;
                } else {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[RA][0] = sx(0x001911ec);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    call_ptr(c, 0x00317670, 0x001911ec)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001911f8);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001911f8)?;
                    break 'b_1911f8;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 4i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001911a0);
                }
                continue 'l_1911a0;
            } else {
                c.cycles += 9;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `int cFEStateRuleSelect_OnWidgetNotify(self, w, kind, id)` (0x00191e48): kind 4 refills the
/// rule list (`FillRuleList`, returns 0x101); kind 1 (a rule changed) greys or
/// re-enables the `b`/`t` widgets of the row `id` by its rule flag and may start
/// the frame thread (`UIScreen_LookupValue`/`StartThread`); the other kinds
/// return 0x101 without work.
pub fn cfe_state_rule_select_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        'b_192060: {
            'b_19205c: {
                'b_192048: {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.g[V0][0] = 0x4;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[A2][0] == c.g[V0][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    if t {
                        break 'b_192048;
                    } else {
                        'b_191ea8: {
                            'b_191e94: {
                                c.cycles += 3;
                                c.g[V0][0] = (c.g[A2][0] < 0x5u64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_191e94;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[A2][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x101;
                                    if t {
                                        break 'b_191ea8;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                        if t {
                                            break 'b_192060;
                                        } else {
                                            break 'b_191e94;
                                        }
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = 0x9;
                            let t = c.g[A2][0] != c.g[V0][0];
                            c.g[V0][0] = 0x101;
                            if t {
                                break 'b_19205c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = 0x100;
                                if t {
                                    break 'b_19205c;
                                } else {
                                    break 'b_191ea8;
                                }
                            }
                        }
                        'b_191f44: {
                            c.cycles += 29;
                            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x48));
                            c.g[A0][0] = 0x460000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1d90));
                            c.g[S6][0] = 0u64;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                            c.g[RA][0] = sx(0x00191ec4);
                            c.g[S4][0] = c.lb(c.g[V0][0] as u32);
                            call_ptr(c, 0x00317670, 0x00191ec4)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x00191ed0);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00191ed0)?;
                            c.g[A0][0] = 0x460000;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00191ee0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1d80));
                            call_ptr(c, 0x00317670, 0x00191ee0)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x00191eec);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00191eec)?;
                            c.g[S2][0] = c.g[V0][0];
                            c.g[A0][0] = 0x460000;
                            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S4][0] as u32));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4e0));
                            c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x51));
                            c.g[RA][0] = sx(0x00191f08);
                            c.g[S5][0] = ((c.g[V1][0] as i64) < 1i64) as u64;
                            call_ptr(c, 0x00317670, 0x00191f08)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x00191f14);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00191f14)?;
                            let t = c.g[S5][0] == 0u64;
                            c.g[S1][0] = c.g[V0][0];
                            if t {
                                break 'b_191f44;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S1][0] == 0u64;
                                c.g[V0][0] = 0x440000;
                                if t {
                                    break 'b_191f44;
                                } else {
                                    c.cycles += 8;
                                    c.g[V1][0] = sx((c.g[S4][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1058));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[RA][0] = sx(0x00191f38);
                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                    call_ptr(c, 0x00317670, 0x00191f38)?;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x00191f44);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a1000, 0x00191f44)?;
                                    break 'b_191f44;
                                }
                            }
                        }
                        'b_192008: {
                            'b_191fac: {
                                c.cycles += 2;
                                let t = c.g[S4][0] == 0u64;

                                if t {
                                    break 'b_191fac;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[S5][0] == 0u64;

                                    if t {
                                        break 'b_192008;
                                    } else {
                                        'b_191f70: {
                                            c.cycles += 2;
                                            let t = c.g[S0][0] == 0u64;
                                            c.g[A1][0] = 0x1;
                                            if t {
                                                break 'b_191f70;
                                            } else {
                                                c.cycles += 5;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00191f70);
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00191f70)?;
                                                break 'b_191f70;
                                            }
                                        }
                                        'b_191f8c: {
                                            c.cycles += 2;
                                            let t = c.g[S2][0] == 0u64;
                                            c.g[A1][0] = 0x1;
                                            if t {
                                                break 'b_191f8c;
                                            } else {
                                                c.cycles += 5;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00191f8c);
                                                c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00191f8c)?;
                                                break 'b_191f8c;
                                            }
                                        }
                                        c.cycles += 8;
                                        c.g[V0][0] = 0x460000;
                                        c.g[S6][0] = 0x1;
                                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x528));
                                        c.g[A0][0] = c.ld((c.g[V0][0] as u32).wrapping_sub(0x528));
                                        c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x8));
                                        c.sd(c.g[SP][0] as u32, c.g[A0][0]);
                                        let t = true;
                                        c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                        if t {
                                            break 'b_192008;
                                        } else {
                                            break 'b_191fac;
                                        }
                                    }
                                }
                            }
                            'b_191fc8: {
                                c.cycles += 2;
                                let t = c.g[S0][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_191fc8;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00191fc8);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00191fc8)?;
                                    break 'b_191fc8;
                                }
                            }
                            'b_191fe4: {
                                c.cycles += 2;
                                let t = c.g[S2][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_191fe4;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00191fe4);
                                    c.g[A0][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00191fe4)?;
                                    break 'b_191fe4;
                                }
                            }
                            c.cycles += 9;
                            c.g[V0][0] = 0x4a0000;
                            c.g[S6][0] = 0x1;
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1708));
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1708));
                            c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x6));
                            c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x4));
                            c.sw(c.g[SP][0] as u32, c.g[A0][0]);
                            c.sh((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            c.sb((c.g[SP][0] as u32).wrapping_add(0x6), c.g[A1][0]);
                            break 'b_192008;
                        }
                        c.cycles += 2;
                        let t = c.g[S6][0] == 0u64;
                        c.g[V0][0] = c.g[S5][0];
                        if t {
                            break 'b_19205c;
                        } else {
                            'b_192040: {
                                c.cycles += 9;
                                c.g[RA][0] = sx(0x00192018);
                                c.g[A0][0] = c.g[SP][0];
                                call_ptr(c, 0x00317670, 0x00192018)?;
                                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x00192024);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039c778, 0x00192024)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V0][0] = 0xffff;
                                let t = c.g[A1][0] == c.g[V0][0];
                                c.g[A1][0] = c.g[A1][0] & 0xffff;
                                if t {
                                    break 'b_192040;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x00192040);
                                    c.g[A2][0] = 0x1;
                                    call_ptr(c, 0x0039c870, 0x00192040)?;
                                    break 'b_192040;
                                }
                            }
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = c.g[S5][0];
                            if t {
                                break 'b_19205c;
                            } else {
                                break 'b_192048;
                            }
                        }
                    }
                }
                c.cycles += 5;
                c.g[A1][0] = sx((c.g[A3][0] as u32) << 24);
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x00192058);
                c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
                Callee {
                    addr: 0x001913b8,
                    f: ee::s02::cFEStateRuleSelect_FillRuleList,
                }
                .call(c, 0x00192058)?;
                c.g[V0][0] = 0x101;
                break 'b_19205c;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            break 'b_192060;
        }
        c.cycles += 9;
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

/// `void cFEStateRuleSelect_ApplyRuleDependencies(self, widget)` (0x00192380): after a rule list box changed, resets (or decrements the counter byte at `+0x52+i` of) every dependent rule `i < 15` selected by the dependency mask of the list box value, then refreshes the row visibility.
pub fn cfe_state_rule_select_apply_rule_dependencies(c: &mut Cpu) {
    guest(c, |c| {
        'b_19245c: {
            'b_192418: {
                c.cycles += 19;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.g[A2][0] = 0x440000;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10c8));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S2][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A0][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[RA][0] = sx(0x001923c4);
                c.g[S3][0] = sx((c.g[V1][0] as u32) << 1);
                call_ptr(c, 0x0039a738, 0x001923c4)?;
                let t = c.g[V0][0] != 0u64;
                c.g[S1][0] = 0x1;
                if t {
                    break 'b_192418;
                } else {
                    c.cycles += 3;
                    c.g[A1][0] = 0x1;
                    c.g[A2][0] = 0x1;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x52));
                    'l_1923d8: loop {
                        'b_192404: {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[A2][0] as u32) << ((c.g[A1][0] as u32) & 31));
                            c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
                            if c.g[V0][0] != 0u64 {
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                break 'b_192404;
                            } else {
                                'b_192400: {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 24);
                                    let t = (c.g[V1][0] as i64) > 0;
                                    c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                                    if t {
                                        break 'b_192400;
                                    } else {
                                        c.cycles += 1;
                                        c.sb(c.g[A0][0] as u32, 0u64);
                                        break 'b_192400;
                                    }
                                }
                                c.cycles += 1;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                break 'b_192404;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A1][0] as i64) < 15i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001923d8);
                            }
                            continue 'l_1923d8;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_19245c;
                            } else {
                                break 'b_192418;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[S4][0] = 0x1;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x52));
            'l_192420: loop {
                'b_192450: {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S4][0] as u32) << ((c.g[S1][0] as u32) & 31));
                    c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        break 'b_192450;
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = 0u64;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.g[RA][0] = sx(0x0019244c);
                        c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                        Callee {
                            addr: 0x00192088,
                            f: ee::s02::cFEStateRuleSelect_SetRuleValue,
                        }
                        .call(c, 0x0019244c)?;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        break 'b_192450;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 15i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00192420);
                    }
                    continue 'l_192420;
                } else {
                    break 'b_19245c;
                }
            }
        }
        c.cycles += 10;
        c.g[RA][0] = sx(0x00192464);
        c.g[A0][0] = c.g[S2][0];
        Callee {
            addr: 0x00192488,
            f: ee::s02::cFEStateRuleSelect_UpdateRowVisibility,
        }
        .call(c, 0x00192464)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateBraggingRights_UpdateRowColors(self)` (0x001927b0): colours the 8 player-row widgets (`+8..`): the selected row (index at `obj+0x95`) white, enabled rows by the list box (`UIListBox_GetSelectedValue`) one of two colour pairs, via the widget vtable colour method (+0x5c).
pub fn cfe_state_bragging_rights_update_row_colors(c: &mut Cpu) {
    guest(c, |c| {
        'b_1928e0: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x88));
            c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x78));
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x68));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
            let t = c.g[V0][0] == 0u64;
            c.g[S1][0] = 0u64;
            if t {
                break 'b_1928e0;
            } else {
                c.cycles += 10;
                c.g[S3][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x95));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                c.lwc1(25, (c.g[GP][0] as u32).wrapping_sub(0x59cc));
                c.lwc1(24, (c.g[GP][0] as u32).wrapping_sub(0x59c8));
                c.lwc1(23, (c.g[GP][0] as u32).wrapping_sub(0x59c4));
                c.lwc1(22, (c.g[GP][0] as u32).wrapping_sub(0x59c0));
                c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x59bc));
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                'l_192818: loop {
                    'b_1928d4: {
                        'b_1928d0: {
                            'b_1928cc: {
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = c.g[S2][0];
                                if t {
                                    break 'b_1928cc;
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x00192828);
                                    c.g[A1][0] = c.g[S1][0];
                                    Callee {
                                        addr: 0x00192948,
                                        f: ee::s02::cFEStateBraggingRights_IsRowEnabled,
                                    }
                                    .call(c, 0x00192828)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                        break 'b_1928d0;
                                    } else {
                                        'b_1928a0: {
                                            'b_19288c: {
                                                'b_19285c: {
                                                    c.cycles += 2;
                                                    let t = c.g[S1][0] != c.g[S3][0];
                                                    c.g[A1][0] = c.g[SP][0];
                                                    if t {
                                                        break 'b_19285c;
                                                    } else {
                                                        c.cycles += 9;
                                                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x58));
                                                        c.swc1(20, c.g[SP][0] as u32);
                                                        c.swc1(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0x4),
                                                        );
                                                        c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(c.g[A0][0] as u32));
                                                        c.swc1(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0x8),
                                                        );
                                                        let t = true;
                                                        c.swc1(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0xc),
                                                        );
                                                        if t {
                                                            break 'b_19288c;
                                                        } else {
                                                            break 'b_19285c;
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x00192864);
                                                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                                call_ptr(c, 0x0039a738, 0x00192864)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] = c.g[SP][0];
                                                if t {
                                                    break 'b_1928a0;
                                                } else {
                                                    c.cycles += 8;
                                                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                                    c.g[V1][0] =
                                                        c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                                    c.swc1(20, c.g[SP][0] as u32);
                                                    c.swc1(
                                                        25,
                                                        (c.g[SP][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    c.swc1(
                                                        24,
                                                        (c.g[SP][0] as u32).wrapping_add(0x8),
                                                    );
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0xc),
                                                        0u64,
                                                    );
                                                    break 'b_19288c;
                                                }
                                            }
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x00192898);
                                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                            call_ptr(c, t, 0x00192898)?;
                                            let t = true;
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                                            if t {
                                                break 'b_1928d4;
                                            } else {
                                                break 'b_1928a0;
                                            }
                                        }
                                        c.cycles += 11;
                                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                        c.swc1(20, c.g[SP][0] as u32);
                                        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x4));
                                        c.g[A0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x8));
                                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001928cc);
                                        c.g[A1][0] = c.g[SP][0];
                                        call_ptr(c, t, 0x001928cc)?;
                                        break 'b_1928cc;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            break 'b_1928d0;
                        }
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                        break 'b_1928d4;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 8i64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, 0x00192818);
                        }
                        continue 'l_192818;
                    } else {
                        break 'b_1928e0;
                    }
                }
            }
        }
        c.cycles += 13;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x88));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x78));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x68));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRuleSelect_SetRuleValue(self, int rule, int value)` (0x00192088): stores
/// `value & 1` as the bit of rule `rule` (jump table at 0x45fb50, 14 cases, in
/// the rule words at 0x5308b8/0x5308d0; one case stores the whole word at
/// 0x5308d4). Out-of-range rules do nothing. No calls.
pub fn cfe_state_rule_select_set_rule_value(c: &mut Cpu) {
    if c.pc != 0x00192088 {
        return ee::s02::f_00192088__entries(c);
    }
    'b_192238: {
        c.cycles += 4;
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = (c.g[V1][0] < 0xeu64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x460000;
        if t {
            break 'b_192238;
        } else {
            'b_1920e8: {
                'b_19222c: {
                    'b_19217c: {
                        'b_192160: {
                            'b_192144: {
                                'b_19212c: {
                                    'b_192114: {
                                        'b_192214: {
                                            'b_1921f8: {
                                                'b_1921dc: {
                                                    'b_1921c0: {
                                                        'b_1921a4: {
                                                            c.cycles += 6;
                                                            c.g[V1][0] =
                                                                sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x4b0));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                            let t = c.g[A0][0] as u32;
                                                            match t {
                                                                0x001920b0 => {
                                                                    c.cycles += 9;
                                                                    c.g[V1][0] = 0x530000;
                                                                    c.g[A1][0] = 0xfffffffffffffffe;
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x8b8));
                                                                    c.g[A0][0] = c.g[A2][0] & 0x1;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[V1][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    c.g[V0][0] =
                                                                        c.g[V0][0] & c.g[A1][0];
                                                                    c.g[V0][0] =
                                                                        c.g[V0][0] | c.g[A0][0];
                                                                    let t = c.g[RA][0] as u32;
                                                                    c.sw(
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x18),
                                                                        c.g[V0][0],
                                                                    );
                                                                    c.pc = t;
                                                                    return;
                                                                }
                                                                0x001920d4 => {
                                                                    c.cycles += 5;
                                                                    c.g[A1][0] = 0x530000;
                                                                    c.g[V1][0] = c.g[A2][0] & 0x1;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[A1][0] as u32)
                                                                            .wrapping_add(0x8b8));
                                                                    c.g[A0][0] = 0xfffffffffffffff7;
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        << 3);
                                                                    break 'b_1920e8;
                                                                }
                                                                0x001920f8 => {
                                                                    c.cycles += 7;
                                                                    c.g[A0][0] = 0x530000;
                                                                    c.g[V1][0] = c.g[A2][0] & 0x1;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x8b8));
                                                                    c.g[A1][0] = 0xfffffffffffffff7;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[A0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    let t = true;
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        << 3);
                                                                    if t {
                                                                        break 'b_19222c;
                                                                    } else {
                                                                        break 'b_192114;
                                                                    }
                                                                }
                                                                0x00192114 => {
                                                                    break 'b_192114;
                                                                }
                                                                0x0019212c => {
                                                                    break 'b_19212c;
                                                                }
                                                                0x00192144 => {
                                                                    break 'b_192144;
                                                                }
                                                                0x00192160 => {
                                                                    break 'b_192160;
                                                                }
                                                                0x0019217c => {
                                                                    break 'b_19217c;
                                                                }
                                                                0x00192188 => {
                                                                    c.cycles += 7;
                                                                    c.g[A0][0] = 0x530000;
                                                                    c.g[V1][0] = c.g[A2][0] & 0x1;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x8b8));
                                                                    c.g[A1][0] = 0xffffffffffffffef;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[A0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    let t = true;
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        << 4);
                                                                    if t {
                                                                        break 'b_19222c;
                                                                    } else {
                                                                        break 'b_1921a4;
                                                                    }
                                                                }
                                                                0x001921a4 => {
                                                                    break 'b_1921a4;
                                                                }
                                                                0x001921c0 => {
                                                                    break 'b_1921c0;
                                                                }
                                                                0x001921dc => {
                                                                    break 'b_1921dc;
                                                                }
                                                                0x001921f8 => {
                                                                    break 'b_1921f8;
                                                                }
                                                                0x00192214 => {
                                                                    break 'b_192214;
                                                                }
                                                                _ => {
                                                                    c.jump_out(
                                                                        t, 0x00192088, 0x00192240,
                                                                    );
                                                                    return;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 7;
                                                        c.g[A0][0] = 0x530000;
                                                        c.g[V1][0] = c.g[A2][0] & 0x1;
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x8b8));
                                                        c.g[A1][0] = 0xffffffffffffffdf;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x18));
                                                        let t = true;
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                                                        if t {
                                                            break 'b_19222c;
                                                        } else {
                                                            break 'b_1921c0;
                                                        }
                                                    }
                                                    c.cycles += 7;
                                                    c.g[A0][0] = 0x530000;
                                                    c.g[V1][0] = c.g[A2][0] & 0x1;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                                                    c.g[A1][0] = 0xffffffffffffffbf;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[A0][0] as u32).wrapping_add(0x18));
                                                    let t = true;
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                                                    if t {
                                                        break 'b_19222c;
                                                    } else {
                                                        break 'b_1921dc;
                                                    }
                                                }
                                                c.cycles += 7;
                                                c.g[A0][0] = 0x530000;
                                                c.g[V1][0] = c.g[A2][0] & 0x1;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                                                c.g[A1][0] = 0xffffffffffffff7f;
                                                c.g[V0][0] =
                                                    c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                                                let t = true;
                                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 7);
                                                if t {
                                                    break 'b_19222c;
                                                } else {
                                                    break 'b_1921f8;
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[A0][0] = 0x530000;
                                            c.g[V1][0] = c.g[A2][0] & 0x1;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                                            c.g[A1][0] = 0xfffffffffffffeff;
                                            c.g[V0][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                                            let t = true;
                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
                                            if t {
                                                break 'b_19222c;
                                            } else {
                                                break 'b_192214;
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[A0][0] = 0x530000;
                                        c.g[V1][0] = c.g[A2][0] & 0x1;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                                        c.g[A1][0] = 0xfffffffffffffdff;
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 9);
                                        break 'b_19222c;
                                    }
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x530000;
                                    c.g[V1][0] = c.g[A2][0] & 0x1;
                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8b8));
                                    c.g[A0][0] = 0xffffffffffffffdf;
                                    let t = true;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                                    if t {
                                        break 'b_1920e8;
                                    } else {
                                        break 'b_19212c;
                                    }
                                }
                                c.cycles += 6;
                                c.g[A1][0] = 0x530000;
                                c.g[V1][0] = c.g[A2][0] & 0x1;
                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8b8));
                                c.g[A0][0] = 0xffffffffffffffef;
                                let t = true;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                if t {
                                    break 'b_1920e8;
                                } else {
                                    break 'b_192144;
                                }
                            }
                            c.cycles += 7;
                            c.g[A0][0] = 0x530000;
                            c.g[V1][0] = c.g[A2][0] & 0x1;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                            c.g[A1][0] = 0xfffffffffffffffd;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                            let t = true;
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                            if t {
                                break 'b_19222c;
                            } else {
                                break 'b_192160;
                            }
                        }
                        c.cycles += 7;
                        c.g[A0][0] = 0x530000;
                        c.g[V1][0] = c.g[A2][0] & 0x1;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                        c.g[A1][0] = 0xfffffffffffffffb;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                        let t = true;
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        if t {
                            break 'b_19222c;
                        } else {
                            break 'b_19217c;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x530000;
                    let t = c.g[RA][0] as u32;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8d4), c.g[A2][0]);
                    c.pc = t;
                    return;
                }
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_192238;
            }
            c.cycles += 4;
            c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            let t = c.g[RA][0] as u32;
            c.sw((c.g[A1][0] as u32).wrapping_add(0x8b8), c.g[V0][0]);
            c.pc = t;
            return;
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
    return;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes (in the machine's own RAM).
    pub(crate) const OC847_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC847_MAGIC: u32 = 0x8470_f4ce;
    /// FindObject results, 2 bits per call (0: null, 1..3: widget object).
    const OC847_FIND_MASK: u32 = OC847_CTRL + 4;
    /// Number of FindObject calls so far.
    const OC847_FIND_N: u32 = OC847_CTRL + 8;
    /// Non-zero: UIScreen_LookupValue returns 0xffff.
    const OC847_LOOKUP_NONE: u32 = OC847_CTRL + 0xc;
    /// UIListBox_GetSelectedValue results, one bit per call.
    const OC847_SEL_MASK: u32 = OC847_CTRL + 0x10;
    const OC847_SEL_N: u32 = OC847_CTRL + 0x14;
    /// IsRowEnabled result bits by row.
    const OC847_ROW_MASK: u32 = OC847_CTRL + 0x18;
    /// Widget objects returned by the FindObject fake, and their vtable.
    const OC847_OBJS: u32 = DATA + 0x20_1000;
    const OC847_VT: u32 = DATA + 0x20_8000;

    fn oc847_post_none(_c: &mut Cpu, _a0: u32, _a1: u32) {}

    /// FindObject: null or one of three widget objects, per call by mask.
    fn oc847_post_find(c: &mut Cpu, _a0: u32, _a1: u32) {
        let n = c.bus.read32(OC847_FIND_N);
        c.bus.write32(OC847_FIND_N, n + 1);
        let sel = (c.bus.read32(OC847_FIND_MASK) >> (2 * (n % 16))) & 3;
        c.g[V0][0] = if sel == 0 {
            0
        } else {
            (OC847_OBJS + sel * 0x200) as u64
        };
    }

    fn oc847_post_lookup(c: &mut Cpu, _a0: u32, _a1: u32) {
        if c.bus.read32(OC847_LOOKUP_NONE) != 0 {
            c.g[V0][0] = 0xffff;
        }
    }

    fn oc847_post_selected(c: &mut Cpu, _a0: u32, _a1: u32) {
        let n = c.bus.read32(OC847_SEL_N);
        c.bus.write32(OC847_SEL_N, n + 1);
        c.g[V0][0] = ((c.bus.read32(OC847_SEL_MASK) >> (n % 32)) & 1) as u64;
    }

    fn oc847_post_row_enabled(c: &mut Cpu, _a0: u32, a1: u32) {
        c.g[V0][0] = ((c.bus.read32(OC847_ROW_MASK) >> (a1 % 32)) & 1) as u64;
    }

    macro_rules! oc847_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $post:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC847_CTRL) != OC847_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    let (a0, a1) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
                    fakes::fake_call(c, $addr);
                    if c.pc == ra {
                        $post(c, a0, a1);
                    }
                }
            )*
            pub(crate) fn oc847_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc847_fakes!(
        (
            oc847_fake_sprintf,
            OC847_PREV_SPRINTF,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828,
            oc847_post_none
        ),
        (
            oc847_fake_hash,
            OC847_PREV_HASH,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            oc847_post_none
        ),
        (
            oc847_fake_find,
            OC847_PREV_FIND,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860,
            oc847_post_find
        ),
        (
            oc847_fake_set_text,
            OC847_PREV_SET_TEXT,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000,
            oc847_post_none
        ),
        (
            oc847_fake_selected,
            OC847_PREV_SELECTED,
            ee::s13::OVERRIDE_0039A738,
            ee::s13::f_0039a738__generated,
            0x0039_a738,
            oc847_post_selected
        ),
        (
            oc847_fake_lookup,
            OC847_PREV_LOOKUP,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039_c778,
            oc847_post_lookup
        ),
        (
            oc847_fake_start,
            OC847_PREV_START,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870,
            oc847_post_none
        ),
        (
            oc847_fake_row_enabled,
            OC847_PREV_ROW_ENABLED,
            ee::s02::OVERRIDE_00192948,
            ee::s02::f_00192948__generated,
            0x0019_2948,
            oc847_post_row_enabled
        ),
        (
            oc847_fake_visibility,
            OC847_PREV_VISIBILITY,
            ee::s02::OVERRIDE_00192488,
            ee::s02::f_00192488__generated,
            0x0019_2488,
            oc847_post_none
        ),
        (
            oc847_fake_fill_list,
            OC847_PREV_FILL_LIST,
            ee::s02::OVERRIDE_001913B8,
            ee::s02::f_001913b8__generated,
            0x0019_13b8,
            oc847_post_none
        ),
    );

    /// Arms the gated fakes and sets this fixture's control words.
    fn oc847_arm(c: &mut Cpu, r: &mut crate::difftest::Rng, seed: u32) {
        oc847_install_fakes();
        c.bus.write32(OC847_CTRL, OC847_MAGIC);
        // Mostly "found", some nulls; every 5th seed finds everything.
        let find = if seed % 5 == 0 {
            0x5555_5555 | r.next_u32() & 0xaaaa_aaaa
        } else {
            r.next_u32()
        };
        c.bus.write32(OC847_FIND_MASK, find);
        c.bus.write32(OC847_LOOKUP_NONE, seed & 1);
        c.bus.write32(OC847_SEL_MASK, r.next_u32());
        c.bus.write32(OC847_ROW_MASK, r.next_u32());
        // Widget objects: vtable with the colour/text method (+0x48 delta,
        // +0x4c function) and the row colour method (+0x58, +0x5c).
        for k in 1..4u32 {
            let obj = OC847_OBJS + k * 0x200;
            c.bus.write32(obj + 8, OC847_VT);
        }
        for off in [0x48u32, 0x58] {
            let delta = r.next_u32() as i16 as i32 & !3;
            c.bus.write16(OC847_VT + off, delta as u16);
            c.bus.write32(OC847_VT + off + 4, fakes::FAKE + 0x10 + off);
        }
    }

    /// The rider-bio name tables: 10 rows copied to the stack, the screen at
    /// `+0x40`, the row index at `+0x48`.
    fn oc847_bio_fixture(c: &mut Cpu, seed: u32, salt: u64, table: u32, words: u32, calls: u32) {
        let mut r = machine(c, seed, salt, calls);
        oc847_arm(c, &mut r, seed);
        fill(c, &mut r, table, words);
        let this = DATA + 0x1000;
        c.g[A0][0] = this as u64;
        c.bus.write32(this + 0x40, 0x1234_5678);
        c.bus.write32(this + 0x48, r.next_u32() % 10);
    }

    /// `FillFaves`: all 12 favourites widgets (found or missing by mask) of
    /// every table row, and the unwinds of its sprintf/hash/find/text calls.
    /// The unaligned-source copy loop (`ldl`/`ldr`) is unreachable: the table
    /// address is a constant with `addr & 7 == 0`.
    #[test]
    fn cfe_state_rider_bio_fill_faves_matches() {
        check(
            ee::s02::f_00190f48__generated,
            cfe_state_rider_bio_fill_faves,
            200,
            &|c, seed| oc847_bio_fixture(c, seed, 0x8471, 0x0045_f2d0, 0x78, 60),
        );
    }

    /// `FillQNA`: the 4 question widgets of every table row.
    #[test]
    fn cfe_state_rider_bio_fill_qna_matches() {
        check(
            ee::s02::f_001910c0__generated,
            cfe_state_rider_bio_fill_qna,
            200,
            &|c, seed| oc847_bio_fixture(c, seed, 0x8472, 0x0045_f730, 0x28, 20),
        );
    }

    /// `OnWidgetNotify`: the kinds 0..=10 (4 refills the list, 1 recolours the
    /// row widgets, 9 and the rest answer directly), rule rows with and
    /// without a map entry, null or present widgets, the frame lookup failing
    /// or not, and the unwinds of the calls.
    #[test]
    fn cfe_state_rule_select_on_widget_notify_matches() {
        check(
            ee::s02::f_00191e48__generated,
            cfe_state_rule_select_on_widget_notify,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8473, 14);
                oc847_arm(c, &mut r, seed);
                let this = DATA + 0x1000;
                let map = DATA + 0x2000;
                c.bus.write32(this + 0x40, 0x1234_5678);
                c.bus.write32(this + 0x48, map);
                for i in 0..8 {
                    // Rule byte (0: no widget work) and the row flag bytes.
                    let b = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write8(map + i, b as u8);
                    c.bus
                        .write8(this + 0x51 + i, (r.next_u32() % 4).wrapping_sub(1) as u8);
                }
                fill(c, &mut r, 0x0044_1058, 8);
                const KINDS: [u32; 8] = [1, 1, 1, 4, 0, 2, 9, 10];
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = KINDS[(seed % 8) as usize] as u64;
                c.g[A3][0] = (r.next_u32() % 8) as u64;
            },
        );
    }

    /// The rule jump table of `SetRuleValue` (0x45fb50): the 14 case targets in
    /// a per-seed order, and now and then a corrupt entry.
    fn oc847_rule_table(c: &mut Cpu, r: &mut crate::difftest::Rng, seed: u32) {
        let mut t: [u32; 14] = [
            0x0019_20b0,
            0x0019_20d4,
            0x0019_20f8,
            0x0019_2114,
            0x0019_212c,
            0x0019_2144,
            0x0019_2160,
            0x0019_217c,
            0x0019_2188,
            0x0019_21a4,
            0x0019_21c0,
            0x0019_21dc,
            0x0019_21f8,
            0x0019_2214,
        ];
        for i in (1..14).rev() {
            t.swap(i, (r.next_u32() % (i as u32 + 1)) as usize);
        }
        if seed % 31 == 7 {
            t[(seed / 31) as usize % 14] = 0x0019_2100;
        }
        crate::difftest::put_words(c, 0x0045_fb50, &t);
        for off in [0x8b8u32, 0x8d0, 0x8d4] {
            c.bus.write32(0x0053_0000 + off, r.next_u32());
        }
    }

    /// `SetRuleValue`: every rule (0..=16: out of range too), every jump
    /// table target, both values.
    #[test]
    fn cfe_state_rule_select_set_rule_value_matches() {
        check(
            ee::s02::f_00192088__generated,
            cfe_state_rule_select_set_rule_value,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8474, 0);
                c.pc = 0x0019_2088;
                oc847_rule_table(c, &mut r, seed);
                c.g[A1][0] = (seed % 17) as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    /// `ApplyRuleDependencies`: every dependency mask (15 rule bits), the
    /// list box value set (resets the dependent rules through the real
    /// `SetRuleValue`) or not (decrements their counters).
    #[test]
    fn cfe_state_rule_select_apply_rule_dependencies_matches() {
        check(
            ee::s02::f_00192380__generated,
            cfe_state_rule_select_apply_rule_dependencies,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8475, 2);
                oc847_arm(c, &mut r, seed);
                oc847_rule_table(c, &mut r, 1);
                let this = DATA + 0x1000;
                let widget = DATA + 0x3000;
                let rule = r.next_u32() % 8;
                c.bus.write32(widget + 0x18, rule);
                for i in 0..8 {
                    let w = if i == rule && seed % 3 == 0 {
                        0
                    } else {
                        r.next_u32() & 0x7fff
                    };
                    c.bus.write32(0x0044_10c8 + 4 * i, w);
                }
                for i in 0..16 {
                    c.bus.write8(this + 0x52 + i, (r.next_u32() % 3) as u8);
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = widget as u64;
            },
        );
    }

    /// `UpdateRowColors`: eight rows with missing or present widgets, the
    /// selected row, enabled or disabled rows and the list box answers.
    #[test]
    fn cfe_state_bragging_rights_update_row_colors_matches() {
        check(
            ee::s02::f_001927b0__generated,
            cfe_state_bragging_rights_update_row_colors,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8476, 24);
                oc847_arm(c, &mut r, seed);
                let this = DATA + 0x1000;
                let owner = DATA + 0x2000;
                c.bus.write32(this, if seed % 17 == 3 { 0 } else { owner });
                c.bus.write8(owner + 0x95, (r.next_u32() % 9) as u8);
                for i in 0..8u32 {
                    let w = if r.next_u32() % 5 == 0 {
                        0
                    } else {
                        OC847_OBJS + (1 + i % 3) * 0x200
                    };
                    c.bus.write32(this + 8 + 4 * i, w);
                }
                for off in [0x59ccu32, 0x59c8, 0x59c4, 0x59c0, 0x59bc] {
                    c.bus.write32(GAME_GP - off, r.edge_f32());
                }
                c.g[A0][0] = this as u64;
            },
        );
    }
}
