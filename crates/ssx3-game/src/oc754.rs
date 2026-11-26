//! Staging module oc754 (wave wave244): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc754`.

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
        0x0029_b818,
        ssx_audio_system_load_trigger_bank,
        "SSXAudioSystem_LoadTriggerBank",
    ),
    (0x002d_cb88, cboard_wake_fx_ctor, "cBoardWakeFX_ctor"),
    (
        0x0019_2240,
        cfe_state_rule_select_get_rule_value,
        "cFEStateRuleSelect_GetRuleValue",
    ),
    (
        0x0039_ad38,
        ui_menu_set_current_item,
        "UIMenu_SetCurrentItem",
    ),
    (0x002e_81e8, cboard_trail_fx_ctor, "cBoardTrailFX_ctor"),
    (0x001e_c1f0, cov_state_hud1p_v24, "cOVStateHUD1P_v24"),
];

/// `int cFEStateRuleSelect::getBEData(cFEStateRuleSelect* this, int rule)` (0x192240).
///
/// Reads one rule setting of the back-end (`cBERules`) as 0/1: `rule` 1..=14
/// indexes a jump table (0x45fb90) whose cases test a bit of the rule words at
/// 0x5308b8 / 0x5308d0 (case 2 returns the whole word at 0x5308d4). Any other
/// `rule` returns -1.
pub fn cfe_state_rule_select_get_rule_value(c: &mut Cpu) {
    if c.pc != 0x0019_2240 {
        return ee::s02::f_00192240__entries(c);
    }
    c.cycles += 4;
    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
    c.g[V0][0] = (c.g[V1][0] < 0xe) as u64;
    let out_of_range = c.g[V0][0] == 0;
    c.g[V0][0] = 0x460000;
    if out_of_range {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x470));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
    let target = c.g[A0][0] as u32;
    // (cycles, rule word, shift) of each case of the jump table.
    let (cycles, word, shift) = match target {
        0x0019_2268 => (4, RULE_WORD_D0, Shift::None),
        0x0019_2278 => (5, RULE_WORD_B8, Shift::Srl(3)),
        0x0019_228c => (5, RULE_WORD_D0, Shift::Sra(3)),
        0x0019_22a0 => (5, RULE_WORD_B8, Shift::Srl(5)),
        0x0019_22b4 => (5, RULE_WORD_B8, Shift::Srl(4)),
        0x0019_22c8 => (5, RULE_WORD_D0, Shift::Sra(1)),
        0x0019_22dc => (5, RULE_WORD_D0, Shift::Sra(2)),
        0x0019_22f0 => (3, RULE_WORD_D4, Shift::Whole),
        0x0019_22fc => (5, RULE_WORD_D0, Shift::Sra(4)),
        0x0019_2310 => (5, RULE_WORD_D0, Shift::Sra(5)),
        0x0019_2324 => (5, RULE_WORD_D0, Shift::Sra(6)),
        0x0019_2338 => (5, RULE_WORD_D0, Shift::Sra(7)),
        0x0019_234c => (5, RULE_WORD_D0, Shift::Sra(8)),
        0x0019_2360 => (5, RULE_WORD_D0, Shift::Sra(9)),
        _ => {
            c.jump_out(target, 0x0019_2240, 0x0019_237c);
            return;
        }
    };
    c.cycles += cycles;
    c.g[V1][0] = 0x530000;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(word));
    match shift {
        Shift::None => {}
        Shift::Srl(n) => c.g[V0][0] = sx((c.g[V0][0] as u32) >> n),
        Shift::Sra(n) => c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> n) as u32),
        Shift::Whole => {}
    }
    ret(c);
    if !matches!(shift, Shift::Whole) {
        c.g[V0][0] &= 1;
    }
}

/// The rule words below 0x530000 that `cfe_state_rule_select_get_rule_value` reads.
const RULE_WORD_B8: u32 = 0x8b8;
const RULE_WORD_D0: u32 = 0x8d0;
const RULE_WORD_D4: u32 = 0x8d4;

/// How a case of `cfe_state_rule_select_get_rule_value` picks its bit.
#[derive(Clone, Copy)]
enum Shift {
    /// Bit 0 of the word.
    None,
    /// `(word >> n) & 1`, logical shift.
    Srl(u32),
    /// `(word >> n) & 1`, arithmetic shift.
    Sra(u32),
    /// The whole word (no mask).
    Whole,
}

/// `int SSXAudioSystem_LoadTriggerBank(SSXAudioSystem* sys, const char* name, int* rec)` (0x29b818).
///
/// Makes sure the sound bank `name` is the one in the trigger-bank slot of
/// `sys` (`+0x5838`). Sets `*rec = 3` (bank record index); when the name
/// differs from the current one and the old bank's status is 0 or 1, unloads
/// it (`SndBankMan_UnloadBank`). Then, by the status word of record `*rec`
/// (0x60 bytes each) in the bank table: 1 returns 1 (already loaded); 0 copies
/// the name, formats the bank path with `sprintf` and starts
/// `SndBankMan_LoadBankAsync` (the function then returns -1 like every other
/// outcome).
pub fn ssx_audio_system_load_trigger_bank(c: &mut Cpu) {
    guest(c, |c| {
        'b_29b948: {
            'b_29b944: {
                'b_29b8b0: {
                    'b_29b8a8: {
                        c.cycles += 15;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
                        c.g[V0][0] = 0x3;
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.g[S0][0] = c.g[A0][0];
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.g[S1][0] = c.g[A2][0];
                        c.g[S2][0] = c.g[A1][0];
                        c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[RA][0]);
                        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5838));
                        call_ptr(c, 0x004165a8, 0x0029b84c)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = 0x60;
                        if t {
                            break 'b_29b8a8;
                        } else {
                            c.cycles += 9;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                            c.g[A0][0] = c.lw(c.g[A2][0] as u32);
                            c.mult(0, 2, 2, 5);
                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xacc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            let t = (c.g[V1][0] as i64) < 0;
                            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_29b8a8;
                            } else {
                                'b_29b894: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = 0x4a0000;
                                    if t {
                                        break 'b_29b894;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x2;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.g[V0][0] = 0xffffffffffffffff;
                                        if t {
                                            break 'b_29b944;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                                            if t {
                                                break 'b_29b8b0;
                                            } else {
                                                break 'b_29b894;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 5;
                                c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x35a8));
                                c.sb((c.g[S0][0] as u32).wrapping_add(0x5838), c.g[V1][0]);
                                c.g[A0][0] = c.lw(c.g[A2][0] as u32);
                                c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                call_ptr(c, 0x0028bce8, 0x0029b8a8)?;
                                break 'b_29b8a8;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                    c.g[A1][0] = 0x60;
                    break 'b_29b8b0;
                }
                'b_29b940: {
                    'b_29b8f4: {
                        'b_29b8ec: {
                            c.cycles += 9;
                            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                            c.g[A2][0] = 0x1;
                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                            c.mult(0, 2, 2, 5);
                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xacc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            let t = c.g[V1][0] == c.g[A2][0];
                            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_29b8ec;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x2;
                                if t {
                                    break 'b_29b940;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] == 0u64;
                                    c.g[V0][0] = 0xffffffffffffffff;
                                    if t {
                                        break 'b_29b8f4;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        if t {
                                            break 'b_29b948;
                                        } else {
                                            break 'b_29b8ec;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = true;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_29b944;
                        } else {
                            break 'b_29b8f4;
                        }
                    }
                    c.cycles += 19;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5838));
                    call_ptr(c, 0x004166f4, 0x0029b900)?;
                    c.g[A1][0] = 0x480000;
                    c.g[A2][0] = 0x4a0000;
                    c.g[T1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[T0][0] = c.g[S2][0];
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x36c8));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2dc8));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x587c));
                    call_ptr(c, 0x00417828, 0x0029b928)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[A3][0] = 0x100;
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    call_ptr(c, 0x0028bd10, 0x0029b940)?;
                    break 'b_29b940;
                }
                c.cycles += 1;
                c.g[V0][0] = 0xffffffffffffffff;
                break 'b_29b944;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
            break 'b_29b948;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = t;
        return Ok(());
    })
}

/// `cBoardWakeFX* cBoardWakeFX::cBoardWakeFX(cBoardWakeFX* this, int fromDerived)` (0x2dcb88).
///
/// Builds the base `cRider` part at `this + 0xc0` when `fromDerived` is
/// non-zero, installs the wake effect's vtables and copies the two vtable
/// images when it is zero, allocates the vertex buffers (`operator_new_named`,
/// 0xe00 and 0x1e00 bytes, 0x600-stride slices) and calls
/// `cBoardWakeFX::reset`. Returns `this`.
pub fn cboard_wake_fx_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2dcbc0: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x160));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
            c.g[S1][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x110));
            let t = c.g[S1][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
            if t {
                break 'b_2dcbc0;
            } else {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc0));
                c.g[A0][0] = c.g[V0][0];
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                call_ptr(c, 0x0011b698, 0x002dcbc0)?;
                break 'b_2dcbc0;
            }
        }
        'b_2dccd0: {
            c.cycles += 19;
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            c.g[V0][0] = 0x490000;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7258));
            c.g[A3][0] = c.g[V0][0];
            c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A2][0]);
            c.g[V0][0] = 0x460000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6470));
            c.g[V1][0] = 0x490000;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.g[T3][0] = c.g[V1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7210));
            c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            c.sw((c.g[A0][0] as u32).wrapping_add(0x6d0), c.g[V0][0]);
            c.g[A1][0] = 0x480000;
            c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[S3][0] = 0x480000;
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            let t = c.g[S1][0] != 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[V1][0]);
            if t {
                break 'b_2dccd0;
            } else {
                c.cycles += 3;
                c.g[V1][0] = c.g[SP][0];
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                'l_2dcc18: loop {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T2][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[T4][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[V1][0] as u32, c.g[A0][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002dcc18);
                        }
                        continue 'l_2dcc18;
                    } else {
                        c.cycles += 14;
                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7258));
                        c.g[A3][0] = c.ld(c.g[A2][0] as u32);
                        c.sd(c.g[V1][0] as u32, c.g[A3][0]);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                        c.g[A3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7210));
                        c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                        c.g[T2][0] = sx((c.g[A3][0] as u32).wrapping_add(0xa0));
                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                        c.g[A2][0] = c.g[T1][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc0));
                        c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        'l_2dcc7c: loop {
                            c.cycles += 11;
                            c.g[V0][0] = c.ld(c.g[A3][0] as u32);
                            c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                            c.g[T4][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                            c.sd(c.g[A2][0] as u32, c.g[V0][0]);
                            c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                            c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                            c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                            let t = c.g[A3][0] != c.g[T2][0];
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002dcc7c);
                                }
                                continue 'l_2dcc7c;
                            } else {
                                c.cycles += 10;
                                c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7210));
                                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = c.ld(c.g[A3][0] as u32);
                                c.g[T2][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                                c.sd(c.g[A2][0] as u32, c.g[V0][0]);
                                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x6c0), c.g[T1][0]);
                                c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                                break 'b_2dccd0;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 8;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7968));
        c.g[A0][0] = 0xe00;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002dcce4)?;
        c.g[A1][0] = 0xffffffffffffffff;
        c.g[V1][0] = 0x1f;
        'l_2dccf0: loop {
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            c.g[V1][0] = 0x4;
            c.g[V1][0] = c.g[A0][0];
            let t = c.g[V1][0] != c.g[A1][0];
            c.g[A0][0] = 0x1e00;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002dccf0);
                }
                continue 'l_2dccf0;
            } else {
                c.cycles += 7;
                c.sw(c.g[S4][0] as u32, c.g[V0][0]);
                c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x7980));
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317e30, 0x002dcd20)?;
                c.g[A0][0] = 0xffffffffffffffff;
                c.g[V1][0] = 0x9f;
                'l_2dcd28: loop {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[A1][0] = 0x600;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002dcd28);
                        }
                        continue 'l_2dcd28;
                    } else {
                        c.cycles += 3;
                        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = 0x3;
                        'l_2dcd50: loop {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x600));
                            let t = (c.g[A0][0] as i64) >= 0;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002dcd50);
                                }
                                continue 'l_2dcd50;
                            } else {
                                c.cycles += 13;
                                c.g[V0][0] = 0xffffffffffffffff;
                                c.g[A0][0] = c.g[S0][0];
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xb4), c.g[V0][0]);
                                Callee {
                                    addr: 0x002dcf28,
                                    f: ee::s09::cBoardWakeFX_Reset,
                                }
                                .call(c, 0x002dcd7c)?;
                                c.g[V0][0] = c.g[S0][0];
                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
                                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
                                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x110));
                                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
                                let t = c.g[RA][0] as u32;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
                                c.pc = t;
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
    })
}

/// `void UIMenu::setCurrentItem(UIMenu* menu, UIObject* item, int replaceThread)` (0x39ad38).
///
/// Makes `item` the menu's current item (`+0xa0`; `+0xa4 = 0xff`). When the
/// old item is flagged (`+0x14` bit 4) its event thread is started and two
/// virtual calls (leave) are made; then the new item gets the virtual calls
/// (enter) and its event thread is started, or replaces the screen's thread.
pub fn ui_menu_set_current_item(c: &mut Cpu) {
    guest(c, |c| {
        'b_39adf0: {
            'b_39adec: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.g[V0][0] = 0xff;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[A1][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.sb((c.g[S0][0] as u32).wrapping_add(0xa4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                let t = c.g[A0][0] == 0u64;
                c.g[S2][0] = c.g[A2][0];
                if t {
                    break 'b_39adec;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    if c.g[V0][0] == 0u64 {
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[S1][0]);
                        break 'b_39adf0;
                    } else {
                        'b_39adb0: {
                            c.cycles += 6;
                            c.g[A1][0] = 0x1;
                            Callee {
                                addr: 0x0039ff78,
                                f: ee::s13::UIObject_GetEventThread,
                            }
                            .call(c, 0x0039ad84)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[V0][0] = 0xffff;
                            if c.g[A1][0] == c.g[V0][0] {
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                break 'b_39adb0;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                if c.g[A0][0] == 0u64 {
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                    break 'b_39adb0;
                                } else {
                                    c.cycles += 3;
                                    c.g[A2][0] = 0x1;
                                    Callee {
                                        addr: 0x0039c870,
                                        f: ee::s13::UIScreen_StartThread,
                                    }
                                    .call(c, 0x0039adac)?;
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                    break 'b_39adb0;
                                }
                            }
                        }
                        c.cycles += 15;
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0039adc8)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        c.g[A2][0] = 0x2;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                        c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xd0));
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0039adec)?;
                        break 'b_39adec;
                    }
                }
            }
            c.cycles += 1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[S1][0]);
            break 'b_39adf0;
        }
        'b_39ae80: {
            c.cycles += 23;
            c.g[A2][0] = 0x1;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0039ae14)?;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
            c.g[A1][0] = 0x1;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0039ae30)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
            c.g[A1][0] = 0u64;
            Callee {
                addr: 0x0039ff78,
                f: ee::s13::UIObject_GetEventThread,
            }
            .call(c, 0x0039ae3c)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = 0xffff;
            if c.g[A1][0] == c.g[V0][0] {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_39ae80;
            } else {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[A0][0] == 0u64;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_39ae80;
                } else {
                    'b_39ae74: {
                        c.cycles += 2;
                        let t = c.g[S2][0] == 0u64;

                        if t {
                            break 'b_39ae74;
                        } else {
                            c.cycles += 4;
                            c.g[A2][0] = 0x1;
                            Callee {
                                addr: 0x0039c870,
                                f: ee::s13::UIScreen_StartThread,
                            }
                            .call(c, 0x0039ae6c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_39ae80;
                            } else {
                                break 'b_39ae74;
                            }
                        }
                    }
                    c.cycles += 3;
                    Callee {
                        addr: 0x0039c820,
                        f: ee::s13::UIScreen_ReplaceThread,
                    }
                    .call(c, 0x0039ae7c)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    break 'b_39ae80;
                }
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
    })
}

/// `cBoardTrailFX* cBoardTrailFX::cBoardTrailFX(cBoardTrailFX* this, int fromDerived)` (0x2e81e8).
///
/// Like `cBoardWakeFX::cBoardWakeFX`: base `cRider` at `this + 0xf0`, vtables,
/// the 48 trail entries below `this + 0x2c` cleared to -1, vertex buffers
/// (`operator_new_named` 0x3cc0 and 0xa20-stride slices, `EA_MemSet`) and
/// `cBoardTrailFX::reset`. Returns `this`.
pub fn cboard_trail_fx_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2e8218: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.g[S1][0] = c.g[A1][0];
            let t = c.g[S1][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
            if t {
                break 'b_2e8218;
            } else {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xf0));
                c.g[A0][0] = c.g[V0][0];
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                call_ptr(c, 0x0011b698, 0x002e8218)?;
                break 'b_2e8218;
            }
        }
        'b_2e8324: {
            c.cycles += 18;
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            c.g[V0][0] = 0x490000;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7448));
            c.g[A3][0] = c.g[V0][0];
            c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A2][0]);
            c.g[V0][0] = 0x460000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6470));
            c.g[V1][0] = 0x490000;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.g[T4][0] = c.g[V1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7400));
            c.g[T3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c));
            c.sw((c.g[A0][0] as u32).wrapping_add(0x6d0), c.g[V0][0]);
            c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = 0x480000;
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            let t = c.g[S1][0] != 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[V1][0]);
            if t {
                break 'b_2e8324;
            } else {
                c.cycles += 3;
                c.g[V1][0] = c.g[SP][0];
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                'l_2e826c: loop {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T2][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[T5][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[V1][0] as u32, c.g[A0][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T5][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002e826c);
                        }
                        continue 'l_2e826c;
                    } else {
                        c.cycles += 14;
                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7448));
                        c.g[A3][0] = c.ld(c.g[A2][0] as u32);
                        c.sd(c.g[V1][0] as u32, c.g[A3][0]);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                        c.g[A3][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x7400));
                        c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                        c.g[T2][0] = sx((c.g[A3][0] as u32).wrapping_add(0xa0));
                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                        c.g[A2][0] = c.g[T1][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf0));
                        c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        'l_2e82d0: loop {
                            c.cycles += 11;
                            c.g[V0][0] = c.ld(c.g[A3][0] as u32);
                            c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                            c.g[T5][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                            c.sd(c.g[A2][0] as u32, c.g[V0][0]);
                            c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                            c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                            c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[T5][0]);
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                            let t = c.g[A3][0] != c.g[T2][0];
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002e82d0);
                                }
                                continue 'l_2e82d0;
                            } else {
                                c.cycles += 10;
                                c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x7400));
                                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = c.ld(c.g[A3][0] as u32);
                                c.g[T2][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                                c.sd(c.g[A2][0] as u32, c.g[V0][0]);
                                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x6c0), c.g[T1][0]);
                                c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                                break 'b_2e8324;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 5;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0u64);
        c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_add(0x5e));
        c.g[A0][0] = 0xffffffffffffffff;
        c.g[V1][0] = 0x2f;
        'l_2e8338: loop {
            c.cycles += 7;
            c.sh(c.g[V0][0] as u32, c.g[A0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let t = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002e8338);
                }
                continue 'l_2e8338;
            } else {
                c.cycles += 7;
                c.g[A0][0] = 0x3cc0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7b20));
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317e30, 0x002e8368)?;
                c.g[A0][0] = 0xffffffffffffffff;
                c.g[V1][0] = 0x143;
                'l_2e8370: loop {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[A1][0] = 0xa20;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002e8370);
                        }
                        continue 'l_2e8370;
                    } else {
                        c.cycles += 3;
                        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = 0x4;
                        'l_2e8398: loop {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa20));
                            let t = (c.g[V1][0] as i64) >= 0;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002e8398);
                                }
                                continue 'l_2e8398;
                            } else {
                                c.cycles += 14;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = 0u64;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                                c.g[A2][0] = 0x3cc0;
                                call_ptr(c, 0x003e6448, 0x002e83c8)?;
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x002e8560,
                                    f: ee::s09::cBoardTrailFX_Reset,
                                }
                                .call(c, 0x002e83d0)?;
                                c.g[V0][0] = c.g[S0][0];
                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
                                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
                                let t = c.g[RA][0] as u32;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                                c.pc = t;
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
    })
}

/// `int cOVStateHUD1P::handleEvent(cOVStateHUD1P* this, int msg, void* param)` (0x1ec1f0).
///
/// A jump table over 16 messages (table at 0x46eb00): counters at
/// `+0x3d4`/`+0x3d8`, flag word `+0x3cc`, the two text slots (`+0x944`,
/// `+0xa48`), the song title hash (`+0x3b4`) and the slide state. Messages
/// >= 16 do nothing. Always returns 0.
pub fn cov_state_hud1p_v24(c: &mut Cpu) {
    if c.pc != 0x001ec1f0 {
        return ee::s04::f_001ec1f0__entries(c);
    }
    guest(c, |c| 'l_1ec1f0: loop {
        'b_1ec3e4: {
            'b_1ec3e0: {
                c.cycles += 6;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.g[V0][0] = (c.g[A1][0] < 0x10u64) as u64;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                let t = c.g[V0][0] == 0u64;
                c.g[S0][0] = c.g[A0][0];
                if t {
                    break 'b_1ec3e0;
                } else {
                    'b_1ec3d4: {
                        'b_1ec3c4: {
                            'b_1ec38c: {
                                'b_1ec378: {
                                    'b_1ec2e0: {
                                        'b_1ec2c4: {
                                            'b_1ec2b4: {
                                                'b_1ec2a0: {
                                                    'b_1ec280: {
                                                        'b_1ec264: {
                                                            'b_1ec250: {
                                                                'b_1ec240: {
                                                                    'b_1ec238: {
                                                                        'b_1ec230: {
                                                                            c.cycles += 7;
                                                                            c.g[V0][0] = 0x470000;
                                                                            c.g[V1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    << 2);
                                                                            c.g[V0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x1500,
                                                                                    ));
                                                                            c.g[V1][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        c.g[V0][0]
                                                                                            as u32,
                                                                                    ));
                                                                            c.g[A0][0] = c
                                                                                .lw(c.g[V1][0]
                                                                                    as u32);
                                                                            let t =
                                                                                c.g[A0][0] as u32;
                                                                            match t {
                                                                                0x001ec1f0 => {
                                                                                    if event_due(c)
                                                                                    {
                                                                                        deliver_event_here(c, 0x001ec1f0);
                                                                                    }
                                                                                    continue 'l_1ec1f0;
                                                                                }
                                                                                0x001ec224 => {
                                                                                    c.cycles += 3;
                                                                                    c.g[V0][0] =
                                                                                        0x1;
                                                                                    let t = true;
                                                                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x3d4), c.g[V0][0]);
                                                                                    if t {
                                                                                        break 'b_1ec3e0;
                                                                                    } else {
                                                                                        break 'b_1ec230;
                                                                                    }
                                                                                }
                                                                                0x001ec230 => {
                                                                                    break 'b_1ec230;
                                                                                }
                                                                                0x001ec238 => {
                                                                                    break 'b_1ec238;
                                                                                }
                                                                                0x001ec240 => {
                                                                                    break 'b_1ec240;
                                                                                }
                                                                                0x001ec250 => {
                                                                                    break 'b_1ec250;
                                                                                }
                                                                                0x001ec264 => {
                                                                                    break 'b_1ec264;
                                                                                }
                                                                                0x001ec280 => {
                                                                                    break 'b_1ec280;
                                                                                }
                                                                                0x001ec2a0 => {
                                                                                    break 'b_1ec2a0;
                                                                                }
                                                                                0x001ec2c4 => {
                                                                                    break 'b_1ec2c4;
                                                                                }
                                                                                0x001ec2e0 => {
                                                                                    break 'b_1ec2e0;
                                                                                }
                                                                                0x001ec378 => {
                                                                                    break 'b_1ec378;
                                                                                }
                                                                                0x001ec38c => {
                                                                                    break 'b_1ec38c;
                                                                                }
                                                                                0x001ec3c4 => {
                                                                                    break 'b_1ec3c4;
                                                                                }
                                                                                0x001ec3d4 => {
                                                                                    break 'b_1ec3d4;
                                                                                }
                                                                                0x001ec3e0 => {
                                                                                    break 'b_1ec3e0;
                                                                                }
                                                                                _ => {
                                                                                    c.jump_out(
                                                                                        t,
                                                                                        0x001ec1f0,
                                                                                        0x001ec3f4,
                                                                                    );
                                                                                    return Ok(());
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        let t = true;
                                                                        c.sw(
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x3d4,
                                                                                ),
                                                                            0u64,
                                                                        );
                                                                        if t {
                                                                            break 'b_1ec3e0;
                                                                        } else {
                                                                            break 'b_1ec238;
                                                                        }
                                                                    }
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.sw(
                                                                        (c.g[S0][0] as u32)
                                                                            .wrapping_add(0x3d8),
                                                                        0u64,
                                                                    );
                                                                    if t {
                                                                        break 'b_1ec3e0;
                                                                    } else {
                                                                        break 'b_1ec240;
                                                                    }
                                                                }
                                                                c.cycles += 4;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x3d8));
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                let t = true;
                                                                c.sw(
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x3d8),
                                                                    c.g[V0][0],
                                                                );
                                                                if t {
                                                                    break 'b_1ec3e0;
                                                                } else {
                                                                    break 'b_1ec250;
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x3d8));
                                                            let t = (c.g[V0][0] as i64) <= 0;
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x1));
                                                            if t {
                                                                break 'b_1ec3e0;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.sw(
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x3d8),
                                                                    c.g[V0][0],
                                                                );
                                                                if t {
                                                                    break 'b_1ec3e0;
                                                                } else {
                                                                    break 'b_1ec264;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 7;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x3cc));
                                                        c.g[V1][0] = 0x20000;
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0xa44),
                                                            0u64,
                                                        );
                                                        c.g[A0][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x944));
                                                        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                                        let t = true;
                                                        c.g[A1][0] = c.g[A2][0];
                                                        if t {
                                                            break 'b_1ec2b4;
                                                        } else {
                                                            break 'b_1ec280;
                                                        }
                                                    }
                                                    c.cycles += 8;
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x3cc)
                                                        );
                                                    c.g[V0][0] = 0xfffffffffffd0000;
                                                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                                                    c.sh(
                                                        (c.g[S0][0] as u32).wrapping_add(0x944),
                                                        0u64,
                                                    );
                                                    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0xa44),
                                                        0u64,
                                                    );
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x3cc),
                                                        c.g[V1][0],
                                                    );
                                                    if t {
                                                        break 'b_1ec3e0;
                                                    } else {
                                                        break 'b_1ec2a0;
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x3cc));
                                                c.g[V1][0] = 0x40000;
                                                c.g[A0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0xa48));
                                                c.g[A1][0] = c.g[A2][0];
                                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                                break 'b_1ec2b4;
                                            }
                                            c.cycles += 4;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x3cc),
                                                c.g[V0][0],
                                            );
                                            call_ptr(c, 0x002c2508, 0x001ec2bc)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                            if t {
                                                break 'b_1ec3e4;
                                            } else {
                                                break 'b_1ec2c4;
                                            }
                                        }
                                        c.cycles += 7;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3cc));
                                        c.g[V0][0] = 0xfffffffffffb0000;
                                        c.g[V0][0] = c.g[V0][0] | 0xffff;
                                        c.sh((c.g[S0][0] as u32).wrapping_add(0xa48), 0u64);
                                        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                        let t = true;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x3cc), c.g[V1][0]);
                                        if t {
                                            break 'b_1ec3e0;
                                        } else {
                                            break 'b_1ec2e0;
                                        }
                                    }
                                    'b_1ec338: {
                                        c.cycles += 13;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3cc));
                                        c.g[V0][0] = 0xfffffffffffe0000;
                                        c.g[V0][0] = c.g[V0][0] | 0xffff;
                                        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x3cc), c.g[V1][0]);
                                        call_ptr(c, 0x0028b180, 0x001ec2f8)?;
                                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
                                        c.g[A1][0] = 0xffffffffffffffff;
                                        call_ptr(c, 0x002b40f0, 0x001ec304)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A1][0] = 0xfffffffff0000000;
                                        if c.g[A0][0] == 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x3cc));
                                            break 'b_1ec338;
                                        } else {
                                            'b_1ec334: {
                                                c.cycles += 3;
                                                c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V1][0] = 0x10000;
                                                if t {
                                                    break 'b_1ec334;
                                                } else {
                                                    c.cycles += 5;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x3cc)
                                                        );
                                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x3cc),
                                                        c.g[V0][0],
                                                    );
                                                    call_ptr(c, 0x00317670, 0x001ec330)?;
                                                    c.g[A1][0] = c.g[V0][0];
                                                    break 'b_1ec334;
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x3cc));
                                            break 'b_1ec338;
                                        }
                                    }
                                    'b_1ec354: {
                                        c.cycles += 4;
                                        c.g[V1][0] = 0x10000;
                                        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_1ec354;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x3b4));
                                            if c.g[A1][0] == c.g[V0][0] {
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                                break 'b_1ec3e4;
                                            } else {
                                                break 'b_1ec354;
                                            }
                                        }
                                    }
                                    c.cycles += 9;
                                    c.g[AT][0] = 0xffffffffbf800000;
                                    c.mtc1(1, 0);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x3b4), c.g[A1][0]);
                                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x190));
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x194), 0u64);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x198), 0u64);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x3a4), 0u64);
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x3a8), 0u64);
                                    if t {
                                        break 'b_1ec3e0;
                                    } else {
                                        break 'b_1ec378;
                                    }
                                }
                                c.cycles += 5;
                                c.g[V0][0] = 0x8;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x3c8), 0u64);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x3c4), c.g[V0][0]);
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x3c0), 0u64);
                                if t {
                                    break 'b_1ec3e0;
                                } else {
                                    break 'b_1ec38c;
                                }
                            }
                            c.cycles += 14;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3cc));
                            c.g[V0][0] = 0xffffffffffbd0000;
                            c.g[V0][0] = c.g[V0][0] | 0xc3ff;
                            c.g[V1][0] = 0xffffffffffff0000;
                            c.g[A0][0] = c.g[A0][0] & c.g[V0][0];
                            c.g[A1][0] = 0x50;
                            c.g[V1][0] = c.g[V1][0] | 0xffff;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x3cc), c.g[A0][0]);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x18c), c.g[V1][0]);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x3c4), c.g[A1][0]);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x3c0), 0u64);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xa44), 0u64);
                            let t = true;
                            c.sh((c.g[S0][0] as u32).wrapping_add(0x944), 0u64);
                            if t {
                                break 'b_1ec3e0;
                            } else {
                                break 'b_1ec3c4;
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = 0x1;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x168), 0u64);
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x160), c.g[V0][0]);
                        if t {
                            break 'b_1ec3e0;
                        } else {
                            break 'b_1ec3d4;
                        }
                    }
                    c.cycles += 3;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x164), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x160), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x168), 0u64);
                    break 'b_1ec3e0;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1ec3e4;
        }
        c.cycles += 4;
        c.g[V0][0] = 0u64;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, put_words, DATA};
    use std::sync::atomic::AtomicUsize;

    /// The word that arms oc754's callee fakes in a machine's RAM.
    pub(crate) const OC754_CTRL: u32 = DATA + 0x30_0000;
    pub(crate) const OC754_MAGIC: u32 = 0x0754_f4ce;

    /// The mailbox word of the fake for `addr` (0: the fake's own pointer).
    pub(crate) fn oc754_mail(addr: u32) -> u32 {
        DATA + 0x30_1000 + (addr & 0xffc)
    }

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc754_arm(c: &mut Cpu) {
        c.bus.write32(OC754_CTRL, OC754_MAGIC);
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3).
    pub(crate) fn oc754_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc754_mail(addr), v.wrapping_add(3));
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc754_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc754_mail(addr), 1);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox unless it unwound.
    pub(crate) fn oc754_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc754_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                2 => a0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc754_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC754_CTRL) != OC754_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc754_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc754_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc754_fakes!(
        (
            oc754_fk_4165a8,
            OC754_PREV_4165A8,
            ee::s15::OVERRIDE_004165A8,
            ee::s15::f_004165a8__generated,
            0x0041_65a8
        ),
        (
            oc754_fk_4166f4,
            OC754_PREV_4166F4,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x0041_66f4
        ),
        (
            oc754_fk_417828,
            OC754_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc754_fk_28bce8,
            OC754_PREV_28BCE8,
            ee::s07::OVERRIDE_0028BCE8,
            ee::s07::f_0028bce8__generated,
            0x0028_bce8
        ),
        (
            oc754_fk_28bd10,
            OC754_PREV_28BD10,
            ee::s07::OVERRIDE_0028BD10,
            ee::s07::f_0028bd10__generated,
            0x0028_bd10
        ),
        (
            oc754_fk_28b180,
            OC754_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc754_fk_39ff78,
            OC754_PREV_39FF78,
            ee::s13::OVERRIDE_0039FF78,
            ee::s13::f_0039ff78__generated,
            0x0039_ff78
        ),
        (
            oc754_fk_39c870,
            OC754_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc754_fk_39c7c0,
            OC754_PREV_39C7C0,
            ee::s13::OVERRIDE_0039C7C0,
            ee::s13::f_0039c7c0__generated,
            0x0039_c7c0
        ),
        (
            oc754_fk_2c2508,
            OC754_PREV_2C2508,
            ee::s08::OVERRIDE_002C2508,
            ee::s08::f_002c2508__generated,
            0x002c_2508
        ),
        (
            oc754_fk_2b40f0,
            OC754_PREV_2B40F0,
            ee::s08::OVERRIDE_002B40F0,
            ee::s08::f_002b40f0__generated,
            0x002b_40f0
        ),
        (
            oc754_fk_317670,
            OC754_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc754_fk_317e30,
            OC754_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc754_fk_11b698,
            OC754_PREV_11B698,
            ee::s00::OVERRIDE_0011B698,
            ee::s00::f_0011b698__generated,
            0x0011_b698
        ),
        (
            oc754_fk_2dcf28,
            OC754_PREV_2DCF28,
            ee::s09::OVERRIDE_002DCF28,
            ee::s09::f_002dcf28__generated,
            0x002d_cf28
        ),
        (
            oc754_fk_2e8560,
            OC754_PREV_2E8560,
            ee::s09::OVERRIDE_002E8560,
            ee::s09::f_002e8560__generated,
            0x002e_8560
        ),
        (
            oc754_fk_3e6448,
            OC754_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
    );

    pub(crate) const OC754_SYS: u32 = DATA + 0x1_0000;
    pub(crate) const OC754_MGR: u32 = DATA + 0x2_0000;
    pub(crate) const OC754_BANKS_OWNER: u32 = DATA + 0x2_1000;
    pub(crate) const OC754_BANK_TABLE: u32 = DATA + 0x2_2000;
    pub(crate) const OC754_REC: u32 = DATA + 0x3_0000;
    pub(crate) const OC754_NAME: u32 = DATA + 0x4_0000;

    /// `SSXAudioSystem_LoadTriggerBank`: the bank record's status (-1, 0, 1,
    /// 2, 7), whether the name equals the current one (strcmp), the old
    /// status paths and the unwinds of its five calls.
    #[test]
    fn ssx_audio_system_load_trigger_bank_matches() {
        oc754_install_fakes();
        fakes::covered(
            ee::s08::f_0029b818__generated,
            ssx_audio_system_load_trigger_bank,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x754a, 4);
                oc754_arm(c);
                c.g[A0][0] = sx(OC754_SYS);
                c.g[A1][0] = sx(OC754_NAME);
                c.g[A2][0] = sx(OC754_REC);
                c.bus.write32(OC754_SYS + 0x118, OC754_MGR);
                c.bus.write32(OC754_MGR, OC754_BANKS_OWNER);
                c.bus.write32(OC754_BANKS_OWNER + 0xacc, OC754_BANK_TABLE);
                let status = [-1i32, 0, 1, 2, 7][(seed % 5) as usize] as u32;
                // `*rec` is set to 3: the record is the fourth, 0x60 bytes each.
                c.bus.write32(OC754_BANK_TABLE + 3 * 0x60, status);
                if (seed / 5) % 2 == 0 {
                    oc754_force_zero(c, 0x0041_65a8);
                }
                c.bus.write8(0x004a_35a8, r.next_u32() as u8);
            },
        );
    }

    /// Fill the vtable images the board FX constructors copy (ROM data that the
    /// test machine's empty RAM lacks).
    fn oc754_fx_images(c: &mut Cpu, r: &mut crate::difftest::Rng) {
        fill(c, r, 0x0048_8b00, 0x100);
        fill(c, r, 0x0048_8d00, 0x100);
    }

    /// `cBoardWakeFX_ctor` with and without the base `cRider` constructor, and
    /// the unwinds of its four calls.
    #[test]
    fn cboard_wake_fx_ctor_matches() {
        oc754_install_fakes();
        fakes::covered(
            ee::s09::f_002dcb88__generated,
            cboard_wake_fx_ctor,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x754b, 4);
                oc754_arm(c);
                oc754_fx_images(c, &mut r);
                c.g[A0][0] = sx(OC754_SYS);
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    sx(r.next_u32() | 1)
                };
                c.bus.write32(OC754_SYS, OC754_MGR);
            },
        );
    }

    /// `cBoardTrailFX_ctor`, same variants.
    #[test]
    fn cboard_trail_fx_ctor_matches() {
        oc754_install_fakes();
        fakes::covered(
            ee::s09::f_002e81e8__generated,
            cboard_trail_fx_ctor,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x754c, 5);
                oc754_arm(c);
                oc754_fx_images(c, &mut r);
                c.g[A0][0] = sx(OC754_SYS);
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    sx(r.next_u32() | 1)
                };
                c.bus.write32(OC754_SYS, OC754_MGR);
            },
        );
    }

    /// `cFEStateRuleSelect_GetRuleValue`: every rule 0..=16 (out of range, the
    /// 14 cases of the jump table) over random rule words, and a corrupt table
    /// entry (jump out).
    #[test]
    fn cfe_state_rule_select_get_rule_value_matches() {
        let table: [u32; 14] = [
            0x1922dc, 0x1922f0, 0x192278, 0x192268, 0x19228c, 0x1922fc, 0x1922b4, 0x1922c8,
            0x192310, 0x192324, 0x192338, 0x19234c, 0x192360, 0x1922a0,
        ];
        check(
            ee::s02::f_00192240__generated,
            cfe_state_rule_select_get_rule_value,
            340,
            &|c, seed| {
                let mut r = machine(c, seed, 0x754d, 0);
                c.pc = 0x0019_2240;
                put_words(c, 0x0045_fb90, &table);
                for off in [0x8b8u32, 0x8d0, 0x8d4] {
                    c.bus.write32(0x0053_0000 + off, r.next_u32());
                }
                let rule = seed % 17;
                c.g[A1][0] = sx(rule);
                if seed % 23 == 0 && (1..=14).contains(&rule) {
                    c.bus.write32(0x0045_fb90 + 4 * (rule - 1), 0x0019_2250);
                }
            },
        );
    }

    pub(crate) const OC754_MENU: u32 = DATA + 0x1_0000;
    pub(crate) const OC754_OWNER: u32 = DATA + 0x2_0000;
    pub(crate) const OC754_HELPER: u32 = DATA + 0x3_0000;
    pub(crate) const OC754_CUR: u32 = DATA + 0x4_0000;
    pub(crate) const OC754_ITEM: u32 = DATA + 0x5_0000;
    pub(crate) const OC754_VT: u32 = DATA + 0x6_0000;

    /// `UIMenu_SetCurrentItem`: no/flagged/unflagged old item, event thread
    /// -1 or not, items with and without a thread object (`+0x5c`), start or
    /// replace the thread, and the unwinds of its calls.
    #[test]
    fn ui_menu_set_current_item_matches() {
        oc754_install_fakes();
        fakes::covered(
            ee::s13::f_0039ad38__generated,
            ui_menu_set_current_item,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x754e, 10);
                oc754_arm(c);
                c.g[A0][0] = sx(OC754_MENU);
                c.g[A1][0] = sx(OC754_ITEM);
                c.g[A2][0] = ((seed / 32) % 2) as u64;
                let cur = if seed % 4 == 0 { 0 } else { OC754_CUR };
                c.bus.write32(OC754_MENU + 0xa0, cur);
                c.bus.write32(OC754_MENU + 0x5c, OC754_OWNER);
                c.bus.write32(OC754_OWNER + 0xd0, OC754_HELPER);
                c.bus.write32(OC754_HELPER + 8, OC754_VT);
                c.bus.write32(OC754_CUR + 8, OC754_VT);
                c.bus.write32(OC754_ITEM + 8, OC754_VT);
                for (slot, i) in [(0x38u32, 0u32), (0x98, 1)] {
                    c.bus
                        .write16(OC754_VT + slot, (r.next_u32() as i16 as i32 & !3) as u16);
                    c.bus.write32(OC754_VT + slot + 4, fakes::FAKE + 0x10 * i);
                }
                let flag4 = if (seed / 4) % 2 == 0 { 0x10 } else { 0 };
                c.bus
                    .write32(OC754_CUR + 0x14, (r.next_u32() & !0x10) | flag4);
                c.bus.write32(
                    OC754_CUR + 0x5c,
                    if (seed / 8) % 2 == 0 { 0 } else { 0x1234 },
                );
                c.bus.write32(
                    OC754_ITEM + 0x5c,
                    if (seed / 16) % 2 == 0 { 0 } else { 0x5678 },
                );
                if seed % 3 == 0 {
                    oc754_force(c, 0x0039_ff78, 0xffff);
                }
                // ReplaceThread (real, unported) finds no primary thread to destroy.
                oc754_force_zero(c, 0x0039_c7c0);
            },
        );
    }

    pub(crate) const OC754_HUD: u32 = DATA + 0x1_0000;
    pub(crate) const OC754_TITLE: u32 = DATA + 0x5_0000;

    /// `cOVStateHUD1P_v24`: each of the 16 messages (and two out of range),
    /// random flag words, the song-title lookup (null, empty, text) and its
    /// hash against the stored one.
    #[test]
    fn cov_state_hud1p_v24_matches() {
        oc754_install_fakes();
        let table: [u32; 16] = [
            0x1ec224, 0x1ec230, 0x1ec238, 0x1ec240, 0x1ec250, 0x1ec2e0, 0x1ec378, 0x1ec38c,
            0x1ec3c4, 0x1ec3d4, 0x1ec3e0, 0x1ec3e0, 0x1ec264, 0x1ec280, 0x1ec2a0, 0x1ec2c4,
        ];
        check(
            ee::s04::f_001ec1f0__generated,
            cov_state_hud1p_v24,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x754f, 4);
                oc754_arm(c);
                c.pc = 0x001e_c1f0;
                put_words(c, 0x0046_eb00, &table);
                c.g[A0][0] = sx(OC754_HUD);
                c.g[A1][0] = sx(seed % 18);
                c.g[A2][0] = sx(DATA + 0x4_0000);
                c.bus.write32(OC754_HUD + 0x3cc, r.next_u32());
                c.bus
                    .write32(OC754_HUD + 0x3d8, (r.next_u32() % 5).wrapping_sub(2));
                let hash = 0x1234;
                let stored = [0xf000_0000u32, hash, r.next_u32()][((seed / 18) % 3) as usize];
                c.bus.write32(OC754_HUD + 0x3b4, stored);
                c.bus
                    .write8(OC754_TITLE, if (seed / 54) % 2 == 0 { 0 } else { b'A' });
                match (seed / 108) % 3 {
                    0 => oc754_force_zero(c, 0x002b_40f0),
                    _ => oc754_force(c, 0x002b_40f0, OC754_TITLE),
                }
                oc754_force(c, 0x0031_7670, hash);
            },
        );
    }
}
