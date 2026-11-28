//! Staging module oc1529 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1529`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `cFEStateTrophyRoom_SetEventStatText(widget, profile, char, peak, kind, idx)`
/// (0x1ce9e0): fill a trophy-room text widget with one event statistic.
/// The arguments arrive as a0 = text widget (s6), a1 = profile index (s3),
/// a2 = character id (s4), a3 = peak (s2), t0 = kind (s0), t1 = idx (s1).
/// Kind 3: "you earned $N"; kind 2: collectables found/total (idx 0) or
/// missions completed/total; kind 0: best time as `mm:ss`; kind 1: best
/// score. Each path formats a localised string through `sprintf` into a
/// `0x1d0(sp)` buffer and hands it to `cUIText_setUnicodeString` (0x3a0e90).
pub fn cfe_state_trophy_room_set_event_stat_text(c: &mut Cpu) {
    guest(c, |c| {
        'b_1ced68: {
            'b_1ced64: {
                'b_1cecd0: {
                    'b_1cecc0: {
                        'b_1cecb0: {
                            'b_1ceacc: {
                                c.cycles += 23;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x220));
                                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x210));
                                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x200));
                                c.g[S0][0] = c.g[T0][0];
                                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                                c.g[S1][0] = c.g[T1][0];
                                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                                c.g[S2][0] = c.g[A3][0];
                                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                                c.g[S3][0] = c.g[A1][0];
                                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                                c.g[S4][0] = c.g[A2][0];
                                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                                c.sd((c.g[SP][0] as u32).wrapping_add(0x1a0), c.g[RA][0]);
                                c.g[S6][0] = c.g[A0][0];
                                call_ptr(c, 0x0014dc80, 0x001cea20)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0xd;
                                call_ptr(c, 0x0014dd58, 0x001cea2c)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[V0][0] = 0x3;
                                let t = c.g[S0][0] != c.g[V0][0];
                                c.g[V0][0] = 0x2;
                                if t {
                                    break 'b_1ceacc;
                                } else {
                                    c.cycles += 34;
                                    call_ptr(c, 0x0014dc80, 0x001cea44)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A1][0] = 0xb;
                                    call_ptr(c, 0x0014dd58, 0x001cea50)?;
                                    c.g[A2][0] = c.g[S4][0];
                                    c.g[A1][0] = c.g[S3][0];
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x001509c0, 0x001cea60)?;
                                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[S2][0] = c.g[V0][0];
                                    c.g[A0][0] = 0x460000;
                                    c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ce0));
                                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    call_ptr(c, 0x00317670, 0x001cea84)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                    c.g[A1][0] = c.g[V0][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, t, 0x001cea94)?;
                                    c.g[S0][0] = 0x4a0000;
                                    c.g[S1][0] = c.g[V0][0];
                                    c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1ee0));
                                    call_ptr(c, 0x00198af0, 0x001ceaac)?;
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S3][0];
                                    call_ptr(c, 0x002c27c0, 0x001ceabc)?;
                                    let t = c.g[S1][0] == 0u64;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x210));
                                    if t {
                                        break 'b_1ced68;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[A1][0] = c.g[S1][0];
                                        if t {
                                            break 'b_1cecb0;
                                        } else {
                                            break 'b_1ceacc;
                                        }
                                    }
                                }
                            }
                            'b_1cebe0: {
                                c.cycles += 2;
                                let t = c.g[S0][0] != c.g[V0][0];

                                if t {
                                    break 'b_1cebe0;
                                } else {
                                    'b_1ceb68: {
                                        c.cycles += 7;
                                        call_ptr(c, 0x0014dc80, 0x001ceadc)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A1][0] = 0xa;
                                        call_ptr(c, 0x0014dd58, 0x001ceae8)?;
                                        let t = c.g[S1][0] != 0u64;
                                        c.g[S0][0] = c.g[V0][0];
                                        if t {
                                            break 'b_1ceb68;
                                        } else {
                                            c.cycles += 24;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = c.g[S2][0];
                                            call_ptr(c, 0x00153390, 0x001ceafc)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = c.g[S3][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[A2][0] = c.g[S4][0];
                                            c.g[S5][0] = c.g[V0][0];
                                            call_ptr(c, 0x00153688, 0x001ceb14)?;
                                            c.g[S2][0] = c.g[V0][0];
                                            c.g[A0][0] = 0x460000;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x6cf8));
                                            c.g[S0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                            c.g[S1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            call_ptr(c, 0x00317670, 0x001ceb38)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                            c.g[A0][0] = c.g[S0][0];
                                            let t = c.g[V1][0] as u32;
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, t, 0x001ceb48)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A0][0] = c.g[SP][0];
                                            if t {
                                                break 'b_1ced64;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[A2][0] = c.g[S2][0];
                                                c.g[A3][0] = c.g[S5][0];
                                                call_ptr(c, 0x002c26d0, 0x001ceb60)?;
                                                let t = true;
                                                c.g[A0][0] = c.g[S6][0];
                                                if t {
                                                    break 'b_1cecc0;
                                                } else {
                                                    break 'b_1ceb68;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 24;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = c.g[S2][0];
                                    call_ptr(c, 0x00154368, 0x001ceb74)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = c.g[S3][0];
                                    c.g[A3][0] = c.g[S2][0];
                                    c.g[A2][0] = c.g[S4][0];
                                    c.g[S5][0] = c.g[V0][0];
                                    call_ptr(c, 0x001544d0, 0x001ceb8c)?;
                                    c.g[S2][0] = c.g[V0][0];
                                    c.g[A0][0] = 0x460000;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6d10));
                                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    call_ptr(c, 0x00317670, 0x001cebb0)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                    c.g[A0][0] = c.g[S0][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, t, 0x001cebc0)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] = c.g[SP][0];
                                    if t {
                                        break 'b_1ced64;
                                    } else {
                                        c.cycles += 6;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[A2][0] = c.g[S2][0];
                                        c.g[A3][0] = c.g[S5][0];
                                        call_ptr(c, 0x002c26d0, 0x001cebd8)?;
                                        let t = true;
                                        c.g[A0][0] = c.g[S6][0];
                                        if t {
                                            break 'b_1cecc0;
                                        } else {
                                            break 'b_1cebe0;
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[S0][0] != 0u64;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_1cecd0;
                            } else {
                                'b_1cec30: {
                                    c.cycles += 18;
                                    c.g[A3][0] = 0x28;
                                    c.g[V1][0] = sx((c.g[S1][0] as u32) << 1);
                                    c.mult(0, 7, 18, 7);
                                    c.g[V0][0] = 0x460000;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5528));
                                    c.g[A1][0] = c.g[S3][0];
                                    c.g[A2][0] = c.g[S4][0];
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                                    c.g[T1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.g[A3][0] = c.lb((c.g[T1][0] as u32));
                                    c.g[T0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x1));
                                    call_ptr(c, 0x00158960, 0x001cec1c)?;
                                    c.g[V1][0] = 0x3c;
                                    c.g[A0][0] = 0x3c;
                                    c.divu(0, 2, 3);
                                    if c.g[V1][0] == 0u64 {
                                        break 'b_1cec30;
                                    } else {
                                        break 'b_1cec30;
                                    }
                                }
                                c.cycles += 31;
                                c.g[S0][0] = 0x460000;
                                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                                c.g[V0][0] = c.lo[0];
                                c.g[S2][0] = c.hi[0];
                                c.mtc1(2, 12);
                                c.cvt_s_w(12, 12);
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6d28));
                                call_ptr(c, 0x00413af8, 0x001cec50)?;
                                c.mtc1(18, 12);
                                c.cvt_s_w(12, 12);
                                c.g[S1][0] = c.g[V0][0];
                                call_ptr(c, 0x00413af8, 0x001cec60)?;
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A2][0] = c.g[S1][0];
                                c.g[A0][0] = c.g[S3][0];
                                c.g[A3][0] = c.g[V0][0];
                                call_ptr(c, 0x00417828, 0x001cec74)?;
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A0][0] = 0x460000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6d38));
                                c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                c.g[S0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                call_ptr(c, 0x00317670, 0x001cec94)?;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                c.g[A0][0] = c.g[S0][0];
                                let t = c.g[V1][0] as u32;
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, t, 0x001ceca4)?;
                                let t = c.g[V0][0] == 0u64;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x210));
                                if t {
                                    break 'b_1ced68;
                                } else {
                                    c.cycles += 1;
                                    c.g[A1][0] = c.g[V0][0];
                                    break 'b_1cecb0;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A2][0] = c.g[S3][0];
                        call_ptr(c, 0x002c26d0, 0x001cecbc)?;
                        c.g[A0][0] = c.g[S6][0];
                        break 'b_1cecc0;
                    }
                    c.cycles += 4;
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x003a0e90, 0x001cecc8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x210));
                    if t {
                        break 'b_1ced68;
                    } else {
                        break 'b_1cecd0;
                    }
                }
                c.cycles += 2;
                if c.g[S0][0] != c.g[V0][0] {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x210));
                    break 'b_1ced68;
                } else {
                    c.cycles += 29;
                    c.g[A3][0] = 0x28;
                    c.g[V1][0] = sx((c.g[S1][0] as u32) << 1);
                    c.mult(0, 7, 18, 7);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xa));
                    c.g[V0][0] = 0x460000;
                    c.g[A1][0] = c.g[S3][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5528));
                    c.g[A2][0] = c.g[S4][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                    c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[A3][0] = c.lb((c.g[T0][0] as u32));
                    c.g[T0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x00158960, 0x001ced10)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6d50));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    call_ptr(c, 0x00317670, 0x001ced34)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x001ced44)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        break 'b_1ced64;
                    } else {
                        c.cycles += 6;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A2][0] = c.g[S2][0];
                        call_ptr(c, 0x002c26d0, 0x001ced58)?;
                        c.g[A0][0] = c.g[S6][0];
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, 0x003a0e90, 0x001ced64)?;
                        break 'b_1ced64;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x210));
            break 'b_1ced68;
        }
        c.cycles += 9;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x200));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1f0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1e0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x1a0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x220));
        c.pc = t;
        Ok(())
    })
}

/// `cOVStateHUD1P_DrawCollectedCount(self, obj, rider, x, animFrame)`
/// (0x21f9b0): when the current section is active, draw the "collected / total"
/// HUD element: the icon bitmap, pulsing (sin-table scale) while `animFrame`
/// (f12) is below 0.44, then the `%d/%d` text through `sprintf` and
/// `cFont_text` (0x391cb0) with the font scale restored afterwards.
pub fn c_ov_state_hud1p_draw_collected_count(c: &mut Cpu) {
    guest(c, |c| {
        'b_21fcf8: {
            c.cycles += 28;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x160));
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x100));
            c.g[S2][0] = c.g[A3][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.g[S3][0] = c.g[A0][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S5][0] = c.g[A1][0];
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x150));
            c.g[FP][0] = c.g[A2][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
            c.mov_s(22, 12);
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
            c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x158));
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x148));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x140));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
            c.g[S6][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1bc));
            c.g[A1][0] = c.g[S6][0];
            Callee {
                addr: 0x0022d390,
                f: ee::s05::f_0022d390,
            }
            .call(c, 0x0021fa18)?;
            let t = c.g[V0][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
            if t {
                break 'b_21fcf8;
            } else {
                'b_21faa8: {
                    c.cycles += 14;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 20);
                    c.lwc1(23, (c.g[GP][0] as u32).wrapping_sub(0x533c));
                    c.swc1(20, (c.g[SP][0] as u32));
                    c.mov_s(21, 20);
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x0014dc80, 0x0021fa3c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0xa;
                    call_ptr(c, 0x0014dd58, 0x0021fa48)?;
                    c.c_lt_s(22, 23);
                    let t = !c.fcc();
                    c.g[S7][0] = c.g[V0][0];
                    if t {
                        break 'b_21faa8;
                    } else {
                        c.cycles += 20;
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5338));
                        c.g[V1][0] = 0x500000;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5334));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fb8));
                        c.mul_s(1, 22, 1);
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5330));
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x532c));
                        c.mul_s(0, 1, 0);
                        c.mul_s(1, 1, 1);
                        c.mul_s(0, 0, 2);
                        c.sub_s(1, 20, 1);
                        c.cvt_w_s(2, 0);
                        c.mfc1(2, 2);
                        c.g[V0][0] = c.g[V0][0] & 0x1ff;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.lwc1(0, (c.g[V0][0] as u32));
                        c.mul_s(0, 0, 3);
                        c.mul_s(0, 0, 1);
                        c.add_s(21, 0, 20);
                        break 'b_21faa8;
                    }
                }
                'b_21fc20: {
                    'b_21fbf4: {
                        c.cycles += 78;
                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[S4][0] = sx((c.g[S5][0] as u32).wrapping_add(0x804));
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S4][0];
                        call_ptr(c, 0x001e91f8, 0x0021fabc)?;
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4a8));
                        c.g[A0][0] = c.g[S4][0];
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x001e9220, 0x0021fad0)?;
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[A0][0] = c.g[S4][0];
                        call_ptr(c, 0x001e9290, 0x0021fadc)?;
                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[T3][0] = c.g[S2][0];
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
                        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.mul_s(3, 3, 21);
                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x44));
                        c.mul_s(0, 0, 21);
                        c.mul_s(1, 1, 21);
                        c.g[T1][0] = c.lb((c.g[S5][0] as u32).wrapping_add(0x824));
                        c.mul_s(2, 2, 21);
                        c.g[T2][0] = c.lb((c.g[S5][0] as u32).wrapping_add(0x825));
                        c.g[A2][0] = c.g[S1][0];
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
                        c.g[T0][0] = c.g[SP][0];
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[S2][0];
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x44));
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A3][0] = c.g[S0][0];
                        call_ptr(c, 0x001f10f8, 0x0021fb34)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.g[T1][0] = 0xfffffffffffffc1f;
                        c.g[V1][0] = c.lb((c.g[S5][0] as u32).wrapping_add(0x826));
                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
                        c.g[A2][0] = c.g[S2][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                        c.g[A3][0] = c.g[S0][0];
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = c.g[V1][0] & 0x3e0;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[T0][0] = c.g[SP][0];
                        c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                        c.g[T1][0] = c.g[S1][0];
                        c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x5c));
                        c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4a8));
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x54));
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x58));
                        call_ptr(c, 0x001f1190, 0x0021fb8c)?;
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.c_lt_s(23, 22);
                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x42c));
                        c.add_s(0, 0, 1);
                        c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x44));
                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x34));
                        c.g[AT][0] = 0x3f000000;
                        c.mtc1(1, 5);
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x14));
                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                        c.cvt_s_w(0, 0);
                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x34));
                        c.mul_s(0, 0, 1);
                        c.mul_s(0, 0, 4);
                        c.sub_s(2, 2, 0);
                        c.mul_s(2, 2, 5);
                        c.add_s(3, 3, 2);
                        let t = !c.fcc();
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x14));
                        if t {
                            break 'b_21fbf4;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x001e91a8, 0x0021fbec)?;
                            let t = true;
                            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x42c));
                            if t {
                                break 'b_21fc20;
                            } else {
                                break 'b_21fbf4;
                            }
                        }
                    }
                    c.cycles += 11;
                    c.g[V0][0] = 0x4d0000;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x79b8));
                    c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                    c.ldr(3, (c.g[A2][0] as u32));
                    c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                    c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x57));
                    c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x5f));
                    c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x42c));
                    break 'b_21fc20;
                }
                c.cycles += 54;
                c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6c0));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S1][0] = 0x4a0000;
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x34));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2300));
                c.mul_s(1, 1, 0);
                c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x38));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                c.mul_s(2, 2, 0);
                c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x3c));
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x42c));
                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x47));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x40));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x4f));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x6c0));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x0021fc88)?;
                c.g[A2][0] = c.g[S6][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S7][0];
                call_ptr(c, 0x00153520, 0x0021fc98)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S7][0];
                c.g[A1][0] = c.g[S6][0];
                call_ptr(c, 0x00153350, 0x0021fca8)?;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00417828, 0x0021fcbc)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x42c));
                c.g[A1][0] = c.g[S2][0];
                c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x14));
                call_ptr(c, 0x00391cb0, 0x0021fcd0)?;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x42c));
                c.lwc1(1, (c.g[SP][0] as u32));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x34));
                c.mul_s(0, 0, 1);
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x38));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.mul_s(2, 2, 1);
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x3c));
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                break 'b_21fcf8;
            }
        }
        c.cycles += 15;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x158));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x148));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x140));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
        c.pc = t;
        Ok(())
    })
}

/// `cOVStateHUD1P_DrawMissionStatus(self, obj)` (0x21fd38): draw the mission
/// status circle: first-use initialisation of the HUD layout globals
/// (`gp+0x1dac..0x1db4` flags, `gp+0x2a40..0x2a54` sizes), the background
/// bitmap, the completed/not-completed coloured circle and the mission number
/// printed in the middle through `PrintAt` (0x1dca40).
pub fn c_ov_state_hud1p_draw_mission_status(c: &mut Cpu) {
    guest(c, |c| {
        'b_21fd8c: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1dac));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S5][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.g[S7][0] = c.g[A1][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            let t = c.g[V0][0] != 0u64;
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
            if t {
                break 'b_21fd8c;
            } else {
                c.cycles += 6;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[V0][0] = 0x1;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1dac), c.g[V0][0]);
                c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x2a4c));
                c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x2a48));
                break 'b_21fd8c;
            }
        }
        'b_21fdb4: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1db0));
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1db4));
            if t {
                break 'b_21fdb4;
            } else {
                c.cycles += 7;
                c.g[AT][0] = 0x40c00000;
                c.mtc1(1, 0);
                c.g[V0][0] = 0x1;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1db0), c.g[V0][0]);
                c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x2a44));
                c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x2a40));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1db4));
                break 'b_21fdb4;
            }
        }
        'b_21fdd0: {
            c.cycles += 2;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_21fdd0;
            } else {
                c.cycles += 5;
                c.g[AT][0] = 0x42080000;
                c.mtc1(1, 0);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1db4), c.g[V0][0]);
                c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x2a54));
                c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x2a50));
                break 'b_21fdd0;
            }
        }
        'b_21ff3c: {
            'b_21ff10: {
                c.cycles += 68;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 4);
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x2a50));
                c.g[S2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x9b4));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_add(0x2a54));
                c.g[A0][0] = c.g[S2][0];
                c.mul_s(0, 0, 4);
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_add(0x2a40));
                c.mul_s(1, 1, 4);
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_add(0x2a44));
                c.g[S6][0] = c.lb((c.g[S7][0] as u32).wrapping_add(0x9d6));
                c.g[A1][0] = c.g[S1][0];
                c.add_s(3, 3, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.add_s(2, 2, 1);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.g[S4][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2a48));
                c.swc1(3, (c.g[SP][0] as u32));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x4));
                call_ptr(c, 0x001e91f8, 0x0021fe24)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[A2][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4cc));
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x001e9220, 0x0021fe38)?;
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[T2][0] = c.lb((c.g[S7][0] as u32).wrapping_add(0x9d5));
                c.g[T1][0] = c.lb((c.g[S7][0] as u32).wrapping_add(0x9d4));
                c.g[T3][0] = 0u64;
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = c.g[S0][0];
                c.g[T0][0] = c.g[S4][0];
                c.g[A0][0] = c.g[S5][0];
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x001f10f8, 0x0021fe60)?;
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x001e91a8, 0x0021fe70)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[T2][0] = 0xfffffffffffffc1f;
                c.g[V1][0] = sx((c.g[S6][0] as u32) << 5);
                c.g[T0][0] = c.g[S4][0];
                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[T1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S5][0];
                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4cc));
                call_ptr(c, 0x001f1190, 0x0021feb0)?;
                call_ptr(c, 0x0014dc80, 0x0021feb8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0xa;
                call_ptr(c, 0x0014dd58, 0x0021fec4)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x18c));
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00153d78, 0x0021fed4)?;
                c.g[T0][0] = c.g[S4][0];
                let t = c.g[V0][0] != 0u64;
                c.g[T1][0] = c.g[S1][0];
                if t {
                    break 'b_21ff10;
                } else {
                    c.cycles += 12;
                    c.g[V0][0] = 0x4d0000;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x77b8));
                    c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                    c.ldr(3, (c.g[A2][0] as u32));
                    c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                    c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x27));
                    c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x2f));
                    c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x28));
                    let t = true;
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_21ff3c;
                    } else {
                        break 'b_21ff10;
                    }
                }
            }
            c.cycles += 11;
            c.g[V0][0] = 0x4d0000;
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x77d8));
            c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.ldr(7, (c.g[A1][0] as u32));
            c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
            c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
            c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x27));
            c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x2f));
            c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x28));
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_21ff3c;
        }
        c.cycles += 96;
        c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x2a40));
        c.g[S0][0] = 0xfffffffffffffc1f;
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_add(0x2a44));
        c.g[A1][0] = sx((c.g[S6][0] as u32) << 5);
        c.add_s(0, 1, 0);
        c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.add_s(1, 1, 2);
        c.g[A1][0] = c.g[A1][0] & 0x3e0;
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A3][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2a50));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A0][0] = c.g[S5][0];
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
        c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x2));
        c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
        c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4d0));
        call_ptr(c, 0x001f1190, 0x0021ff94)?;
        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x42c));
        c.g[A0][0] = c.g[S2][0];
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5328));
        c.lwc1(20, (c.g[V0][0] as u32).wrapping_add(0x14));
        c.cvt_s_w(20, 20);
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x34));
        c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x18c));
        c.mul_s(20, 20, 0);
        c.mul_s(20, 20, 1);
        call_ptr(c, 0x00154240, 0x0021ffbc)?;
        c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x42c));
        c.g[V1][0] = 0x4d0000;
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7798));
        c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(6, (c.g[A1][0] as u32));
        c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
        c.ldr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x27));
        c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x2f));
        c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x28));
        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x27));
        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x2f));
        c.ldr(8, (c.g[SP][0] as u32).wrapping_add(0x28));
        c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x47));
        c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x40));
        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x4f));
        c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x48));
        c.g[A3][0] = sx((c.g[S6][0] as u32) << 5);
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5324));
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x42c));
        c.g[A3][0] = c.g[A3][0] & 0x3e0;
        c.g[A1][0] = 0x4a0000;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x30));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2138));
        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x34));
        c.mul_s(0, 0, 2);
        c.mul_s(1, 1, 2);
        c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x38));
        c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0x3c));
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
        c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.lwc1(1, (c.g[SP][0] as u32));
        c.add_s(13, 13, 0);
        c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x42c));
        c.add_s(12, 12, 1);
        c.sub_s(13, 13, 20);
        call_ptr(c, 0x001dca40, 0x00220078)?;
        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x42c));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x30));
        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x34));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x38));
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x3c));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
        c.pc = t;
        Ok(())
    })
}

/// `SSXAudioSystem_BaseDtor(self, flags)` (0x2ab478): destroy the audio
/// system: reset the vtables of the music, speech and crowd subsystems (and
/// build local copies of the sound-system, bank-monitor and bank-manager
/// vtables adjusted by the this-pointer deltas when `flags == 0`), then run
/// the sub-destructors; bit 1 of `flags` also destroys the stream array,
/// bank manager and sound manager, bit 0 frees the object (0x317e50).
pub fn ssx_audio_system_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2ab61c: {
            c.cycles += 36;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
            c.g[V0][0] = 0x480000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x38f8));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[V1][0] = 0x480000;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[A2][0] = 0x480000;
            c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3870));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x5558), c.g[V0][0]);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x37f8));
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
            c.g[A0][0] = 0x480000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x571c), c.g[V1][0]);
            c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x37d8));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
            c.g[A3][0] = 0x480000;
            c.g[T7][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3760));
            c.g[T9][0] = 0x480000;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
            c.g[T8][0] = sx((c.g[T9][0] as u32).wrapping_add(0x37a0));
            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
            c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
            c.g[S1][0] = c.g[A1][0];
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xab0), c.g[T7][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
            let t = c.g[S1][0] != 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1d4), c.g[T8][0]);
            if t {
                break 'b_2ab61c;
            } else {
                c.cycles += 69;
                c.g[A1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x37d8));
                c.g[T5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[T6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.sd((c.g[SP][0] as u32), c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                c.g[T3][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x3760));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[T4][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[T2][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x10));
                c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                c.g[T1][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw((c.g[T0][0] as u32));
                c.g[A3][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[A2][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5728));
                c.g[A1][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x30));
                c.g[V1][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x38));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T3][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[T4][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[T2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[T1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[T0][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xab0), c.g[T5][0]);
                c.g[T4][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                c.g[A0][0] = c.lhu((c.g[T7][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = c.lw((c.g[T4][0] as u32));
                c.g[V0][0] = c.lhu((c.g[T7][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5730));
                c.g[T3][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x37a0));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                c.g[A1][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x8));
                c.g[T2][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[T1][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x18));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[A2][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x20));
                c.g[A3][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x28));
                c.g[T0][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[A1][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x88), c.g[A3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[T0][0]);
                c.g[V0][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lw((c.g[T4][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw((c.g[A0][0] as u32).wrapping_add(0x1d4), c.g[T6][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                break 'b_2ab61c;
            }
        }
        'b_2ab680: {
            c.cycles += 12;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0u64;
            Callee {
                addr: 0x002b0ae8,
                f: ee::s08::f_002b0ae8,
            }
            .call(c, 0x002ab628)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0u64;
            Callee {
                addr: 0x002b3398,
                f: ee::s08::f_002b3398,
            }
            .call(c, 0x002ab634)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0u64;
            Callee {
                addr: 0x002a6b50,
                f: ee::s08::f_002a6b50,
            }
            .call(c, 0x002ab640)?;
            c.g[V0][0] = c.g[S1][0] & 0x2;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = c.g[S1][0] & 0x1;
            if t {
                break 'b_2ab680;
            } else {
                c.cycles += 13;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                c.g[A1][0] = 0u64;
                Callee {
                    addr: 0x002aa648,
                    f: ee::s08::f_002aa648,
                }
                .call(c, 0x002ab658)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                call_ptr(c, 0x0028bb10, 0x002ab668)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
                Callee {
                    addr: 0x002adee8,
                    f: ee::s08::f_002adee8,
                }
                .call(c, 0x002ab67c)?;
                c.g[V0][0] = c.g[S1][0] & 0x1;
                break 'b_2ab680;
            }
        }
        'b_2ab694: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
                break 'b_2ab694;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00317e50, 0x002ab690)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
                break 'b_2ab694;
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.pc = t;
        Ok(())
    })
}

/// `BankMonitor_ResetCurrentParams(p)` (0x2b93b8): copy the 0x18-byte
/// current parameter record into slot `p->idx` (+0x1c) of the record array
/// (+0x20) and reset that slot's parallel arrays (volume 100, pan 90, ...,
/// mute flag 1).
pub fn bank_monitor_reset_current_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 126;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.g[T1][0] = 0x18;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = 0u64;
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = 0x8;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
        c.mult(0, 3, 3, 9);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0xb));
        c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x4));
        c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x13));
        c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0xc));
        c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1b));
        c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x14));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.sdr(2, (c.g[V1][0] as u32));
        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
        c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x24), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
        c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
        c.mult(0, 4, 4, 9);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
        c.g[V1][0] = c.lb((c.g[A0][0] as u32));
        c.sw((c.g[A3][0] as u32), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
        call_ptr(c, 0x00416210, 0x002b9488)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = 0x64;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
        c.g[A1][0] = 0x5a;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A2][0] = 0x32;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A3][0] = 0x7f;
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[T0][0] = 0x1;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), c.g[A0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), c.g[A3][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.sw((c.g[V1][0] as u32), c.g[T0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001c_e9e0,
        cfe_state_trophy_room_set_event_stat_text,
        "cFEStateTrophyRoom_SetEventStatText",
    ),
    (
        0x0021_f9b0,
        c_ov_state_hud1p_draw_collected_count,
        "cOVStateHUD1P_DrawCollectedCount",
    ),
    (
        0x0021_fd38,
        c_ov_state_hud1p_draw_mission_status,
        "cOVStateHUD1P_DrawMissionStatus",
    ),
    (
        0x002a_b478,
        ssx_audio_system_base_dtor,
        "SSXAudioSystem_BaseDtor",
    ),
    (
        0x002b_93b8,
        bank_monitor_reset_current_params,
        "BankMonitor_ResetCurrentParams",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check_lohi, elf_image, fakes, fill, run_generated, Rng, DATA, GAME_GP};
    use std::collections::BTreeSet;
    use std::ptr::addr_of_mut;

    /// Gate word of this module's fakes (its own address and magic in the
    /// machine's RAM, so other tests' machines run the real bodies).
    pub(crate) const OC1529_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC1529_MAGIC: u32 = 0x1529_f00d;
    /// Result mailbox of the "localised string getter" dummy target
    /// ([`OC1529_LOC`]): the value the vtable call returns in v0.
    pub(crate) const OC1529_LOCRES: u32 = DATA + 0x20_0100;
    /// Forced results by call ordinal (`+ 4 * n`): 0 = the fake's own
    /// pointer, 1 = zero, anything else = that value.
    pub(crate) const OC1529_Q: u32 = DATA + 0x20_0200;
    /// A function nobody here calls directly; fixtures put its address in a
    /// vtable slot so the call through the table reaches a gated fake.
    pub(crate) const OC1529_LOC: u32 = 0x003d_edc0;

    /// Every faked callee: a gated fake in its override slot that logs the
    /// call (all registers) and answers from its result queue.
    macro_rules! oc1529_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1529_CTRL) != OC1529_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    oc1529_fake(c, $addr);
                }
            )*
            /// Puts every fake in its override slot (once per process).
            fn oc1529_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc1529_fakes! {
        oc1529_fake_0014dc80, 0x0014_dc80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated;
        oc1529_fake_0014dd58, 0x0014_dd58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated;
        oc1529_fake_001509c0, 0x0015_09c0, ee::s01::OVERRIDE_001509C0, ee::s01::f_001509c0__generated;
        oc1529_fake_00317670, 0x0031_7670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated;
        oc1529_fake_00198af0, 0x0019_8af0, ee::s03::OVERRIDE_00198AF0, ee::s03::f_00198af0__generated;
        oc1529_fake_002c27c0, 0x002c_27c0, ee::s08::OVERRIDE_002C27C0, ee::s08::f_002c27c0__generated;
        oc1529_fake_00153390, 0x0015_3390, ee::s01::OVERRIDE_00153390, ee::s01::f_00153390__generated;
        oc1529_fake_00153688, 0x0015_3688, ee::s01::OVERRIDE_00153688, ee::s01::f_00153688__generated;
        oc1529_fake_00154368, 0x0015_4368, ee::s01::OVERRIDE_00154368, ee::s01::f_00154368__generated;
        oc1529_fake_001544d0, 0x0015_44d0, ee::s01::OVERRIDE_001544D0, ee::s01::f_001544d0__generated;
        oc1529_fake_002c26d0, 0x002c_26d0, ee::s08::OVERRIDE_002C26D0, ee::s08::f_002c26d0__generated;
        oc1529_fake_00158960, 0x0015_8960, ee::s01::OVERRIDE_00158960, ee::s01::f_00158960__generated;
        oc1529_fake_00413af8, 0x0041_3af8, ee::s15::OVERRIDE_00413AF8, ee::s15::f_00413af8__generated;
        oc1529_fake_00417828, 0x0041_7828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated;
        oc1529_fake_003a0e90, 0x003a_0e90, ee::s13::OVERRIDE_003A0E90, ee::s13::f_003a0e90__generated;
        oc1529_fake_0022d390, 0x0022_d390, ee::s05::OVERRIDE_0022D390, ee::s05::f_0022d390__generated;
        oc1529_fake_001e91f8, 0x001e_91f8, ee::s04::OVERRIDE_001E91F8, ee::s04::f_001e91f8__generated;
        oc1529_fake_001e9220, 0x001e_9220, ee::s04::OVERRIDE_001E9220, ee::s04::f_001e9220__generated;
        oc1529_fake_001e9290, 0x001e_9290, ee::s04::OVERRIDE_001E9290, ee::s04::f_001e9290__generated;
        oc1529_fake_001f10f8, 0x001f_10f8, ee::s04::OVERRIDE_001F10F8, ee::s04::f_001f10f8__generated;
        oc1529_fake_001f1190, 0x001f_1190, ee::s04::OVERRIDE_001F1190, ee::s04::f_001f1190__generated;
        oc1529_fake_001e91a8, 0x001e_91a8, ee::s04::OVERRIDE_001E91A8, ee::s04::f_001e91a8__generated;
        oc1529_fake_00391cb0, 0x0039_1cb0, ee::s12::OVERRIDE_00391CB0, ee::s12::f_00391cb0__generated;
        oc1529_fake_00153520, 0x0015_3520, ee::s01::OVERRIDE_00153520, ee::s01::f_00153520__generated;
        oc1529_fake_00153350, 0x0015_3350, ee::s01::OVERRIDE_00153350, ee::s01::f_00153350__generated;
        oc1529_fake_00153d78, 0x0015_3d78, ee::s01::OVERRIDE_00153D78, ee::s01::f_00153d78__generated;
        oc1529_fake_00154240, 0x0015_4240, ee::s01::OVERRIDE_00154240, ee::s01::f_00154240__generated;
        oc1529_fake_001dca40, 0x001d_ca40, ee::s04::OVERRIDE_001DCA40, ee::s04::f_001dca40__generated;
        oc1529_fake_00416210, 0x0041_6210, ee::s15::OVERRIDE_00416210, ee::s15::f_00416210__generated;
        oc1529_fake_0028bb10, 0x0028_bb10, ee::s07::OVERRIDE_0028BB10, ee::s07::f_0028bb10__generated;
        oc1529_fake_002aa648, 0x002a_a648, ee::s08::OVERRIDE_002AA648, ee::s08::f_002aa648__generated;
        oc1529_fake_002adee8, 0x002a_dee8, ee::s08::OVERRIDE_002ADEE8, ee::s08::f_002adee8__generated;
        oc1529_fake_00317e50, 0x0031_7e50, ee::s10::OVERRIDE_00317E50, ee::s10::f_00317e50__generated;
        oc1529_fake_002b0ae8, 0x002b_0ae8, ee::s08::OVERRIDE_002B0AE8, ee::s08::f_002b0ae8__generated;
        oc1529_fake_002b3398, 0x002b_3398, ee::s08::OVERRIDE_002B3398, ee::s08::f_002b3398__generated;
        oc1529_fake_002a6b50, 0x002a_6b50, ee::s08::OVERRIDE_002A6B50, ee::s08::f_002a6b50__generated;
        oc1529_fake_003dedc0, 0x003d_edc0, ee::s14::OVERRIDE_003DEDC0, ee::s14::f_003dedc0__generated;
    }

    /// The logged call; its result comes from the mailbox (the dummy vtable
    /// target) or from the queue slot of this call's ordinal.
    pub(crate) fn oc1529_fake(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake_call(c, target);
        if target == OC1529_LOC {
            let w = c.bus.read32(OC1529_LOCRES);
            c.g[V0][0] = sx(w);
        } else if n < 32 {
            let w = c.bus.read32(OC1529_Q + 4 * n);
            if w == 1 {
                c.g[V0][0] = 0;
            } else if w != 0 {
                c.g[V0][0] = sx(w);
            }
        }
    }

    /// Hooked fakes, scrambled registers, the game image, the gate on.
    pub(crate) fn oc1529_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1529_hook();
        let r = fakes::machine(c, seed, salt, calls);
        elf_image(c);
        c.bus.write32(OC1529_CTRL, OC1529_MAGIC);
        r
    }

    /// Distinct cycle totals of the generated body over the seeds: the
    /// fixture's path count.
    fn oc1529_paths(generated: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) -> usize {
        let mut seen = BTreeSet::new();
        for seed in 0..seeds {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, generated);
            seen.insert(c.cycles - before);
        }
        seen.len()
    }

    /// A random halfword this-pointer delta (multiple of 4).
    fn oc1529_delta(r: &mut Rng) -> u16 {
        (r.next_u32() as i16 as i32 & !3) as u16
    }

    /// Trophy-room stat text: kinds 0..5 (best time, best score, collect or
    /// mission counts by idx, money earned, unknown), a localised string that
    /// is empty or not, units from random peaks.
    fn trophy_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1529_machine(c, seed, 0x001c_e9e0, 8);
        let (w, m, v) = (DATA + 0x1000, DATA + 0x1100, DATA + 0x1200);
        fill(c, &mut r, w, 0x40);
        fill(c, &mut r, m, 0x40);
        fill(c, &mut r, v, 0x40);
        c.bus.write32(GAME_GP - 0x848, w);
        c.bus.write32(w + 0x8c, m);
        c.bus.write32(m + 4, v);
        let delta = oc1529_delta(&mut r);
        c.bus.write16(v + 0x20, delta);
        c.bus.write32(v + 0x24, OC1529_LOC);
        c.bus
            .write32(OC1529_LOCRES, if seed % 5 == 0 { 0 } else { DATA + 0x3000 });
        let kind = seed % 6;
        c.g[A0][0] = sx(DATA + 0x2000);
        c.g[A1][0] = sx(r.next_u32() % 4);
        c.g[A2][0] = sx(r.next_u32() % 16);
        c.g[A3][0] = sx(r.next_u32() % 3);
        c.g[T0][0] = sx(kind);
        c.g[T1][0] = sx((seed / 6) % 3 * (1 + (seed / 18) % 2));
    }

    /// `cFEStateTrophyRoom_SetEventStatText`: every kind arm, the empty and
    /// non-empty localised string, calls unwinding at random points.
    #[test]
    fn cfe_state_trophy_room_set_event_stat_text_matches() {
        fakes::covered(
            ee::s04::f_001ce9e0__generated,
            cfe_state_trophy_room_set_event_stat_text,
            200,
            &trophy_setup,
        );
        check_lohi(
            ee::s04::f_001ce9e0__generated,
            cfe_state_trophy_room_set_event_stat_text,
            100,
            &trophy_setup,
        );
        let n = oc1529_paths(ee::s04::f_001ce9e0__generated, 200, &trophy_setup);
        assert!(n >= 8, "fixture reaches too few paths: {n}");
    }

    /// Collected-count HUD element: the section active or not, the animation
    /// frame below, at and above the pulse threshold.
    fn collected_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1529_machine(c, seed, 0x0021_f9b0, 12);
        let sp = crate::difftest::SP;
        fill(c, &mut r, sp - 0x160, 0x58);
        let (w, x, y) = (DATA + 0x1000, DATA + 0x1100, DATA + 0x1200);
        let (g, d, f, q1) = (DATA + 0x1300, DATA + 0x1400, DATA + 0x1500, DATA + 0x1600);
        let (obj, this, rider) = (DATA + 0x4000, DATA + 0x5000, DATA + 0x6000);
        fill(c, &mut r, obj, 0x220);
        fill(c, &mut r, this, 0x140);
        fill(c, &mut r, f, 0x20);
        fill(c, &mut r, d, 4);
        fill(c, &mut r, y, 0x80);
        c.bus.write32(GAME_GP - 0x848, w);
        c.bus.write32(w + 0x84, x);
        c.bus.write32(x + 0x78, y);
        c.bus.write32(GAME_GP - 0x854, g);
        c.bus.write32(g + 0xe84, d);
        c.bus.write32(this + 0x42c, f);
        c.bus.write32(f + 0x14, r.next_u32() % 40);
        c.bus.write32(f + 0x30, r.f32(4.0));
        c.bus.write32(f + 0x34, r.f32(4.0));
        c.bus.write32(rider + 0x6c0, q1);
        let delta = oc1529_delta(&mut r);
        c.bus.write16(q1 + 0x38, delta);
        c.bus.write32(q1 + 0x3c, OC1529_LOC);
        c.bus.write32(OC1529_LOCRES, DATA + 0x3100);
        // The first call is the section-active test: zero for some seeds.
        c.bus.write32(OC1529_Q, if seed % 7 == 0 { 1 } else { 0 });
        let frames = [0.1f32, 0.3, 0.439_954_28, 0.9, -0.2, 0.43, 0.44, 2.0];
        c.f[12] = frames[(seed % 8) as usize].to_bits();
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(obj);
        c.g[A2][0] = sx(rider);
        c.g[A3][0] = sx(r.next_u32());
    }

    /// `cOVStateHUD1P_DrawCollectedCount`: inactive section, each animation
    /// frame band, calls unwinding.
    #[test]
    fn c_ov_state_hud1p_draw_collected_count_matches() {
        fakes::covered(
            ee::s05::f_0021f9b0__generated,
            c_ov_state_hud1p_draw_collected_count,
            200,
            &collected_setup,
        );
        let n = oc1529_paths(ee::s05::f_0021f9b0__generated, 200, &collected_setup);
        assert!(n >= 4, "fixture reaches too few paths: {n}");
    }

    /// Mission-status HUD element: the three first-use flags set or clear,
    /// the mission completed or not (result of the 8th call).
    fn mission_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1529_machine(c, seed, 0x0021_fd38, 11);
        let sp = crate::difftest::SP;
        fill(c, &mut r, sp - 0x100, 0x40);
        let (g, d, f) = (DATA + 0x7000, DATA + 0x1400, DATA + 0x1500);
        let (obj, this) = (DATA + 0x4000, DATA + 0x5000);
        fill(c, &mut r, obj, 0x240);
        fill(c, &mut r, this, 0x140);
        fill(c, &mut r, f, 0x20);
        fill(c, &mut r, d, 4);
        c.bus.write32(GAME_GP + 0x1dac, seed & 1);
        c.bus.write32(GAME_GP + 0x1db0, (seed >> 1) & 1);
        c.bus.write32(GAME_GP + 0x1db4, (seed >> 2) & 1);
        for off in [0x2a40u32, 0x2a44, 0x2a48, 0x2a4c, 0x2a50, 0x2a54] {
            c.bus.write32(GAME_GP + off, r.f32(50.0));
        }
        c.bus.write32(GAME_GP - 0x854, g);
        c.bus.write32(g + 0xe84, d);
        c.bus.write32(this + 0x42c, f);
        c.bus.write32(f + 0x14, r.next_u32() % 40);
        c.bus.write32(f + 0x30, r.f32(4.0));
        c.bus.write32(f + 0x34, r.f32(4.0));
        // The mission-completed query is always the 8th call (ordinal 7).
        c.bus
            .write32(OC1529_Q + 4 * 7, if (seed >> 3) & 1 == 1 { 1 } else { 0 });
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(obj);
    }

    /// `cOVStateHUD1P_DrawMissionStatus`: every first-use flag combination,
    /// completed and open missions, calls unwinding.
    #[test]
    fn c_ov_state_hud1p_draw_mission_status_matches() {
        fakes::covered(
            ee::s05::f_0021fd38__generated,
            c_ov_state_hud1p_draw_mission_status,
            200,
            &mission_setup,
        );
        let n = oc1529_paths(ee::s05::f_0021fd38__generated, 200, &mission_setup);
        assert!(n >= 8, "fixture reaches too few paths: {n}");
    }

    /// Audio-system destructor: flags 0..7 (vtable copies, stream/bank/sound
    /// sub-destructors, free), calls unwinding.
    fn audio_dtor_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1529_machine(c, seed, 0x002a_b478, 3);
        let (this, ss, bm, snd) = (DATA + 0x1000, DATA + 0x8000, DATA + 0x9000, DATA + 0xa000);
        fill(c, &mut r, this, 0x1600);
        fill(c, &mut r, bm, 0x2c0);
        fill(c, &mut r, snd, 0x40);
        c.bus.write32(this + 0x118, ss);
        c.bus.write32(ss, bm);
        c.bus.write32(bm + 0x1d8, snd);
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(seed % 8);
    }

    /// `SSXAudioSystem_BaseDtor`.
    #[test]
    fn ssx_audio_system_base_dtor_matches() {
        fakes::covered(
            ee::s08::f_002ab478__generated,
            ssx_audio_system_base_dtor,
            200,
            &audio_dtor_setup,
        );
        let n = oc1529_paths(ee::s08::f_002ab478__generated, 200, &audio_dtor_setup);
        assert!(n >= 4, "fixture reaches too few paths: {n}");
    }

    /// Bank monitor parameter reset: a monitor with 13 parallel arrays and a
    /// record slot idx 0..3, the record array at a misaligned address.
    fn bank_reset_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1529_machine(c, seed, 0x002b_93b8, 1);
        let p = DATA + 0x1000;
        fill(c, &mut r, p, 0x40);
        let recs = DATA + 0x2000 + seed % 5;
        fill(c, &mut r, DATA + 0x2000, 0x40);
        c.bus.write32(p + 0x1c, seed % 4);
        c.bus.write32(p + 0x20, recs);
        for (k, off) in [
            0x34u32, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x84,
        ]
        .into_iter()
        .enumerate()
        {
            let arr = DATA + 0x3000 + 0x100 * k as u32;
            fill(c, &mut r, arr, 0x10);
            c.bus.write32(p + off, arr);
        }
        c.g[A0][0] = sx(p);
    }

    /// `BankMonitor_ResetCurrentParams` (straight-line; one `memset` call).
    #[test]
    fn bank_monitor_reset_current_params_matches() {
        fakes::covered(
            ee::s08::f_002b93b8__generated,
            bank_monitor_reset_current_params,
            200,
            &bank_reset_setup,
        );
        check_lohi(
            ee::s08::f_002b93b8__generated,
            bank_monitor_reset_current_params,
            100,
            &bank_reset_setup,
        );
    }
}
