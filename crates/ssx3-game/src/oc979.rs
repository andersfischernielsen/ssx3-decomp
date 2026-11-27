//! Staging module oc979 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc979`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001f_f7b8,
    ov_screen_reward_list_get_entry_text,
    "OVScreenRewardList_GetEntryText",
)];

// Stack slots of `OVScreenRewardList_GetEntryText` (frame 0x370; the first
// 0x28 bytes are the per-reward-type counters).
/// Saved `this` (a0): the reward screen.
const RWL_THIS: u32 = 0x2a0;
/// Saved a1: the output text buffer.
const RWL_OUT: u32 = 0x2a4;
/// Saved a2: the entry index asked for (-1: only count the lines).
const RWL_INDEX: u32 = 0x2a8;
/// The rewards file (`BERewardInt_GetRewardsFile`).
const RWL_FILE: u32 = 0x2ac;
/// The running line counter.
const RWL_LINES: u32 = 0x2b0;
/// The reward category (0..32) being listed.
const RWL_CAT: u32 = 0x2b4;
/// `sp + 0x30`: the format string scratch buffer.
const RWL_FMT: u32 = 0x2b8;
/// `sp + 0x100`: the second scratch buffer.
const RWL_TMP1: u32 = 0x2bc;
/// `sp + 0x1d0`: the third scratch buffer.
const RWL_TMP2: u32 = 0x2c0;
/// `reward type * 4`: the offset into the trophy name table.
const RWL_TYPE4: u32 = 0x2c4;

/// `(base + off)` sign-extended as a 32-bit sum.
#[inline(always)]
fn rwl_add(base: u64, off: u32) -> u64 {
    sx((base as u32).wrapping_add(off))
}

/// `lw off(reg)`.
#[inline(always)]
fn rwl_lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    c.lw(a)
}

/// `lw off(sp)`.
#[inline(always)]
fn rwl_slw(c: &mut Cpu, off: u32) -> u64 {
    let a = sp(c).wrapping_add(off);
    c.lw(a)
}

/// `sw v, off(sp)`.
#[inline(always)]
fn rwl_ssw(c: &mut Cpu, off: u32, v: u64) {
    let a = sp(c).wrapping_add(off);
    c.sw(a, v);
}

/// The world pointer at `gp - 0x848`.
#[inline(always)]
fn rwl_world(c: &mut Cpu) -> u64 {
    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848))
}

/// Second half of a string-table lookup: call the table's lookup method
/// (`vtable + 0x24`) with `this = s0`, `a1 = v0` (the string hash).
fn rwl_vcall(c: &mut Cpu, ret: u32) -> Flow {
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
    c.g[A0][0] = c.g[S0][0];
    let target = c.g[V1][0] as u32;
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, target, ret)
}

/// Look up a localised string: `a0` holds the name, `s0` the string table
/// object and `s1` its vtable. `StrHash(a0)`, then the table's lookup
/// method; the text pointer is left in `v0`.
fn rwl_string(c: &mut Cpu, ret_hash: u32, ret_lookup: u32) -> Flow {
    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = rwl_add(c.g[S0][0], c.g[V0][0] as u32);
    call_ptr(c, 0x0031_7670, ret_hash)?;
    rwl_vcall(c, ret_lookup)
}

/// Epilogue part shared by the exits that restore `s0` first.
fn rwl_restore_s0(c: &mut Cpu) {
    let a = sp(c).wrapping_add(0x360);
    c.lq(16, a);
}

/// `UStr_Sprintf(out, format = a1, text = v0)` and leave.
fn rwl_sprintf_out(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A2][0] = c.g[V0][0];
    call_ptr(c, 0x002c_26d0, 0x001f_fc80)?;
    rwl_restore_s0(c);
    Ok(())
}

/// `UStr_SprintfAsciiFmt_2(out, format = a1, text = v0)` and leave.
fn rwl_sprintf2_out(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A2][0] = c.g[V0][0];
    call_ptr(c, 0x002c_27c0, 0x001f_fcc4)?;
    rwl_restore_s0(c);
    Ok(())
}

/// The end of the function when the list has been walked: with index -1
/// the line count goes into `this + 0x9c`, otherwise the "no entry" text
/// is printed.
fn rwl_final(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    let not_count_only = c.g[A0][0] != c.g[V0][0];
    c.g[A1][0] = 0x4a0000;
    if not_count_only {
        c.cycles += 3;
        c.g[A0][0] = rwl_slw(c, RWL_OUT);
        c.g[A1][0] = rwl_add(c.g[A1][0], 0x2560);
        call_ptr(c, 0x002c_27c0, 0x001f_fcd8)?;
    } else {
        c.cycles += 4;
        c.g[V1][0] = rwl_slw(c, RWL_LINES);
        c.g[V0][0] = rwl_slw(c, RWL_THIS);
        let a = (c.g[V0][0] as u32).wrapping_add(0x9c);
        c.sw(a, c.g[V1][0]);
    }
    c.cycles += 1;
    rwl_restore_s0(c);
    Ok(())
}

/// `cOVState_REWARDS::checkRewardList(this, text, index)`: the rewards
/// screen's list of earned rewards. It walks the 32 reward categories and
/// numbers the lines (a category header line, then one line per new reward;
/// money first when there is any) and builds the text of line `index`
/// into `text`; with `index == -1` it only stores the line count at
/// `this + 0x9c`. Frame 0x370, saves s0-s7 and fp.
pub fn ov_screen_reward_list_get_entry_text(c: &mut Cpu) {
    guest(c, |c| {
        rwl_body(c)?;
        // Epilogue ('b_1ffcdc): s0 was restored by the exit paths.
        let frame = Frame { size: 0x370 };
        c.cycles += 11;
        frame.restore(c, 17, 0x350);
        frame.restore(c, 18, 0x340);
        frame.restore(c, 19, 0x330);
        frame.restore(c, 20, 0x320);
        frame.restore(c, 21, 0x310);
        frame.restore(c, 22, 0x300);
        frame.restore(c, 23, 0x2f0);
        frame.restore(c, 30, 0x2e0);
        frame.restore_ra(c, 0x2d0);
        frame.close(c);
        Ok(())
    })
}

fn rwl_body(c: &mut Cpu) -> Flow {
    c.cycles += 31;
    let frame = Frame::open(c, 0x370);
    let a2 = c.g[A2][0];
    rwl_ssw(c, RWL_INDEX, a2);
    frame.save(c, 23, 0x2f0);
    frame.save(c, 16, 0x360);
    c.g[S7][0] = rwl_add(a2, 1);
    frame.save(c, 19, 0x330);
    frame.save(c, 22, 0x300);
    let a0 = c.g[A0][0];
    rwl_ssw(c, RWL_THIS, a0);
    let a1 = c.g[A1][0];
    rwl_ssw(c, RWL_OUT, a1);
    frame.save(c, 17, 0x350);
    frame.save(c, 18, 0x340);
    frame.save(c, 20, 0x320);
    frame.save(c, 21, 0x310);
    frame.save(c, 30, 0x2e0);
    frame.save_ra(c, 0x2d0);
    c.g[S0][0] = 0;
    // The reward interface: BELibrary -> interface 13.
    call_ptr(c, 0x0014_dc80, 0x001f_f7fc)?;
    rwl_ssw(c, RWL_LINES, 0);
    c.g[A1][0] = 0xd;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_dd58, 0x001f_f80c)?;
    c.g[S3][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x0015_8f20, 0x001f_f818)?; // GetNewRewards
    c.g[S6][0] = c.g[V0][0];
    call_ptr(c, 0x0015_a478, 0x001f_f820)?; // GetRewardsFile
    let file = c.g[V0][0];
    rwl_ssw(c, RWL_FILE, file);
    c.g[A0][0] = c.g[S6][0];
    call_ptr(c, 0x0015_a6f0, 0x001f_f82c)?; // CountNewRewards
    let any_new = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[SP][0];
    if !any_new {
        c.cycles += 3;
        c.g[V0][0] = rwl_lw(c, S3, 0x10);
        if c.g[V0][0] == 0 {
            // Nothing at all to list.
            c.g[A0][0] = rwl_slw(c, RWL_INDEX);
            return rwl_final(c);
        }
    }

    // Zero the per-type counters.
    c.cycles += 6;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x28;
    call_ptr(c, 0x0041_6210, 0x001f_f84c)?;
    c.g[V1][0] = rwl_lw(c, S3, 0x10);
    c.g[V0][0] = rwl_add(c.g[SP][0], 0x30);
    if c.g[V1][0] != 0 {
        // The money line comes first.
        c.cycles += 3;
        c.g[S0][0] = 1;
        rwl_ssw(c, RWL_LINES, 1);
        if c.g[S7][0] == c.g[S0][0] {
            c.cycles += 21;
            c.g[V0][0] = rwl_world(c);
            c.g[A0][0] = 0x470000;
            c.g[S2][0] = rwl_add(c.g[SP][0], 0x30);
            c.g[A0][0] = rwl_add(c.g[A0][0], 0x400);
            c.g[S0][0] = rwl_lw(c, V0, 0x8c);
            c.g[S1][0] = rwl_lw(c, S0, 4);
            rwl_string(c, 0x001f_f888, 0x001f_f898)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x002c_2508, 0x001f_f8a4)?; // UStr_Copy
            c.g[A0][0] = rwl_lw(c, S3, 0x10);
            call_ptr(c, 0x0019_8af0, 0x001f_f8ac)?; // FE_FormatMoney
            c.g[A0][0] = rwl_slw(c, RWL_OUT);
            c.g[A1][0] = c.g[S2][0];
            return rwl_sprintf_out(c);
        }
    }

    c.cycles += 6;
    c.g[V1][0] = rwl_add(c.g[SP][0], 0x100);
    c.g[A0][0] = rwl_add(c.g[SP][0], 0x1d0);
    rwl_ssw(c, RWL_CAT, 0);
    let fmt = c.g[V0][0];
    rwl_ssw(c, RWL_FMT, fmt);
    let tmp1 = c.g[V1][0];
    rwl_ssw(c, RWL_TMP1, tmp1);
    let tmp2 = c.g[A0][0];
    rwl_ssw(c, RWL_TMP2, tmp2);

    // One pass per category (header line plus its reward lines).
    'outer: loop {
        c.cycles += 6;
        c.g[V0][0] = rwl_slw(c, RWL_LINES);
        c.g[S0][0] = rwl_add(c.g[S0][0], 1);
        c.g[S4][0] = 0;
        c.g[S5][0] = c.g[SP][0];
        c.g[V0][0] = rwl_add(c.g[V0][0], 1);
        let lines = c.g[V0][0];
        rwl_ssw(c, RWL_LINES, lines);
        // One pass per reward type s4 (0..10) of the category.
        'inner: loop {
            c.cycles += 8;
            c.g[A1][0] = rwl_slw(c, RWL_CAT);
            c.g[A0][0] = c.g[S6][0];
            c.g[A2][0] = c.g[S4][0];
            call_ptr(c, 0x0015_a750, 0x001f_f8f8)?; // GetNewRewardCount
            c.g[S2][0] = c.g[V0][0];
            c.g[V0][0] = 8;
            let not_type8 = c.g[S4][0] != c.g[V0][0];
            c.g[V0][0] = 9;
            // The loop condition is ready in v0 only on the cheat-char skip.
            let mut cond_ready = false;
            if not_type8 {
                c.cycles += 2;
                if c.g[S4][0] == c.g[V0][0] {
                    // Type 9: the accessory / board graphic.
                    c.cycles += 2;
                    if (c.g[S2][0] as i64) <= 0 {
                        c.g[S4][0] = rwl_add(c.g[S4][0], 1);
                    } else {
                        c.cycles += 3;
                        c.g[S0][0] = rwl_add(c.g[S0][0], 1);
                        if c.g[S7][0] != c.g[S0][0] {
                            c.g[S4][0] = rwl_add(c.g[S4][0], 1);
                        } else {
                            return rwl_accessory_line(c);
                        }
                    }
                } else {
                    // Types 0..7: bit-tested rewards.
                    c.cycles += 2;
                    let none = (c.g[S2][0] as i64) <= 0;
                    c.g[A0][0] = c.g[S6][0];
                    if !none {
                        c.cycles += 7;
                        c.g[A1][0] = c.g[S4][0];
                        call_ptr(c, 0x0015_a768, 0x001f_fa9c)?; // GetNewRewardBits
                        c.g[S3][0] = c.g[S5][0];
                        c.g[A0][0] = sx((c.g[S4][0] as u32) << 2);
                        c.g[S1][0] = c.g[V0][0];
                        let type4 = c.g[A0][0];
                        rwl_ssw(c, RWL_TYPE4, type4);
                        c.g[FP][0] = 2;
                        loop {
                            c.cycles += 6;
                            c.g[A1][0] = rwl_lw(c, S3, 0);
                            c.g[A0][0] = c.g[S6][0];
                            c.g[A2][0] = c.g[S1][0];
                            call_ptr(c, 0x0015_a5f0, 0x001f_fac0)?; // NewRewardsTestBit
                            if c.g[V0][0] == 0 {
                                c.g[V0][0] = rwl_lw(c, S3, 0);
                            } else {
                                c.cycles += 3;
                                c.g[S0][0] = rwl_add(c.g[S0][0], 1);
                                let not_target = c.g[S7][0] != c.g[S0][0];
                                c.g[S2][0] = rwl_add(c.g[S2][0], 0xffff_ffff);
                                if not_target {
                                    c.cycles += 1;
                                    c.g[V0][0] = rwl_lw(c, S3, 0);
                                } else {
                                    c.cycles += 2;
                                    let not_type2 = c.g[S4][0] != c.g[FP][0];
                                    c.g[V0][0] = rwl_world(c);
                                    if not_type2 {
                                        return rwl_trophy_line(c);
                                    }
                                    return rwl_type2_line(c);
                                }
                            }
                            // Count this bit as seen.
                            c.cycles += 3;
                            c.g[V0][0] = rwl_add(c.g[V0][0], 1);
                            let again = (c.g[S2][0] as i64) > 0;
                            let a = c.g[S3][0] as u32;
                            c.sw(a, c.g[V0][0]);
                            if again {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001f_fab0);
                                }
                                continue;
                            }
                            break;
                        }
                    }
                    c.cycles += 1;
                    c.g[S4][0] = rwl_add(c.g[S4][0], 1);
                }
            } else {
                // Type 8: the cheat character.
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 10i64) as u64;
                let few = c.g[V0][0] != 0;
                c.g[S4][0] = rwl_add(c.g[S4][0], 1);
                if !few {
                    c.cycles += 3;
                    c.g[S0][0] = rwl_add(c.g[S0][0], 1);
                    let not_target = c.g[S7][0] != c.g[S0][0];
                    c.g[V0][0] = ((c.g[S4][0] as i64) < 10i64) as u64;
                    if not_target {
                        cond_ready = true;
                    } else {
                        return rwl_cheat_line(c);
                    }
                }
            }
            if !cond_ready {
                c.cycles += 1;
                c.g[V0][0] = ((c.g[S4][0] as i64) < 10i64) as u64;
            }
            c.cycles += 2;
            let more = c.g[V0][0] != 0;
            c.g[S5][0] = rwl_add(c.g[S5][0], 4);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001f_f8e8);
                }
                continue 'inner;
            }
            break 'inner;
        }

        // End of the category.
        c.cycles += 3;
        c.g[A0][0] = rwl_slw(c, RWL_LINES);
        let header_empty = c.g[A0][0] == c.g[S0][0];
        c.g[V0][0] = rwl_slw(c, RWL_LINES);
        if header_empty {
            // No reward lines: drop the header line again.
            c.cycles += 3;
            c.g[V0][0] = rwl_add(c.g[V0][0], 0xffff_ffff);
            let lines = c.g[V0][0];
            rwl_ssw(c, RWL_LINES, lines);
            c.g[S0][0] = c.g[V0][0];
        } else {
            c.cycles += 2;
            if c.g[A0][0] != c.g[S7][0] {
                let s0 = c.g[S0][0];
                rwl_ssw(c, RWL_LINES, s0);
            } else {
                return rwl_header_line(c);
            }
        }
        c.cycles += 5;
        c.g[V1][0] = rwl_slw(c, RWL_CAT);
        c.g[V1][0] = rwl_add(c.g[V1][0], 1);
        c.g[V0][0] = ((c.g[V1][0] as i64) < 32i64) as u64;
        let more = c.g[V0][0] != 0;
        let cat = c.g[V1][0];
        rwl_ssw(c, RWL_CAT, cat);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x001f_f8d0);
            }
            continue 'outer;
        }
        c.cycles += 1;
        c.g[A0][0] = rwl_slw(c, RWL_INDEX);
        return rwl_final(c);
    }
}

/// Line of type 9 (accessory / board graphic reward): "<kind>: <name>".
fn rwl_accessory_line(c: &mut Cpu) -> Flow {
    c.cycles += 15;
    c.g[A0][0] = 0;
    call_ptr(c, 0x0014_a130, 0x001f_f9c0)?; // BE_FindRiderByProfile
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_a080, 0x001f_f9c8)?; // BE_GetRiderCharacter
    c.g[S0][0] = c.g[V0][0];
    c.g[S1][0] = c.lh((c.g[S6][0] as u32).wrapping_add(4));
    call_ptr(c, 0x0014_bdb8, 0x001f_f9d4)?; // Bolts_GetDB
    c.g[S2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x0014_d448, 0x001f_f9e8)?; // Bolts_FindBoltIndex
    c.g[A1][0] = c.g[V0][0];
    let found = (c.g[A1][0] as i64) >= 0;
    c.g[V0][0] = 0x38;
    if found {
        c.cycles += 3;
        c.g[V1][0] = rwl_lw(c, S2, 4);
        c.mult(0, 2, 5, 2);
        c.g[S2][0] = rwl_add(c.g[V1][0], c.g[V0][0] as u32);
    } else {
        c.cycles += 2;
        c.g[S2][0] = 0;
    }
    c.cycles += 4;
    c.g[V0][0] = rwl_lw(c, S2, 0x34);
    c.g[V0][0] &= 0x800;
    let accessory = c.g[V0][0] == 0;
    c.g[V0][0] = rwl_world(c);
    if accessory {
        c.cycles += 2;
        c.g[A0][0] = 0x470000;
        c.g[A0][0] = rwl_add(c.g[A0][0], 0x440);
    } else {
        c.cycles += 3;
        c.g[A0][0] = 0x470000;
        c.g[A0][0] = rwl_add(c.g[A0][0], 0x428);
    }
    c.cycles += 24;
    c.g[S0][0] = rwl_lw(c, V0, 0x8c);
    c.g[S1][0] = rwl_lw(c, S0, 4);
    rwl_string(c, 0x001f_fa40, 0x001f_fa50)?;
    c.g[A0][0] = rwl_slw(c, RWL_TMP2);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x002c_2508, 0x001f_fa5c)?; // UStr_Copy
    c.g[A0][0] = rwl_slw(c, RWL_TMP2);
    c.g[A2][0] = c.g[A0][0];
    c.g[V1][0] = 0x470000;
    c.g[A0][0] = rwl_slw(c, RWL_FMT);
    c.g[A1][0] = rwl_add(c.g[V1][0], 0x418);
    call_ptr(c, 0x002c_2718, 0x001f_fa74)?; // UStr_SprintfAsciiFmt
    c.g[A2][0] = rwl_lw(c, S2, 0x14);
    c.g[A0][0] = rwl_slw(c, RWL_OUT);
    c.g[A1][0] = rwl_slw(c, RWL_FMT);
    call_ptr(c, 0x002c_26d0, 0x001f_fa84)?; // UStr_Sprintf
    rwl_restore_s0(c);
    Ok(())
}

/// Line of type 8 (the cheat character): "<text> <name>".
fn rwl_cheat_line(c: &mut Cpu) -> Flow {
    c.cycles += 31;
    c.g[V0][0] = rwl_world(c);
    c.g[V1][0] = 0x440000;
    c.g[V1][0] = rwl_add(c.g[V1][0], 0x1ce8);
    c.g[S0][0] = rwl_lw(c, V0, 0x8c);
    c.g[A0][0] = rwl_lw(c, V1, 0x20);
    c.g[S1][0] = rwl_lw(c, S0, 4);
    rwl_string(c, 0x001f_f944, 0x001f_f954)?;
    c.g[A0][0] = rwl_slw(c, RWL_FMT);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x002c_2508, 0x001f_f960)?; // UStr_Copy
    c.g[A0][0] = rwl_slw(c, RWL_FILE);
    c.g[A2][0] = rwl_add(c.g[S2][0], 0xffff_fff6);
    c.g[A1][0] = 8;
    call_ptr(c, 0x0015_af40, 0x001f_f970)?; // GetRewardTypeText
    c.g[A1][0] = rwl_slw(c, RWL_FMT);
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = rwl_slw(c, RWL_TMP1);
    call_ptr(c, 0x002c_26d0, 0x001f_f980)?; // UStr_Sprintf
    c.g[V0][0] = 0x470000;
    c.g[A0][0] = rwl_slw(c, RWL_OUT);
    c.g[A2][0] = rwl_slw(c, RWL_TMP1);
    c.g[A1][0] = rwl_add(c.g[V0][0], 0x418);
    call_ptr(c, 0x002c_27c0, 0x001f_f994)?; // UStr_SprintfAsciiFmt_2
    rwl_restore_s0(c);
    Ok(())
}

/// Line of a bit-tested reward type other than 2: "<type trophy text>
/// <reward name>".
fn rwl_trophy_line(c: &mut Cpu) -> Flow {
    c.cycles += 31;
    c.g[V1][0] = rwl_world(c);
    c.g[V0][0] = 0x440000;
    c.g[A0][0] = rwl_slw(c, RWL_TYPE4);
    c.g[V0][0] = rwl_add(c.g[V0][0], 0x1ce8);
    c.g[S0][0] = rwl_lw(c, V1, 0x8c);
    c.g[S2][0] = rwl_add(c.g[SP][0], 0x100);
    c.g[V0][0] = rwl_add(c.g[A0][0], c.g[V0][0] as u32);
    c.g[S1][0] = rwl_lw(c, S0, 4);
    c.g[A0][0] = rwl_lw(c, V0, 0);
    rwl_string(c, 0x001f_fc2c, 0x001f_fc3c)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x002c_2508, 0x001f_fc48)?; // UStr_Copy
    c.g[S0][0] = rwl_add(c.g[SP][0], 0x30);
    c.g[A1][0] = 0x470000;
    c.g[A1][0] = rwl_add(c.g[A1][0], 0x418);
    c.g[A2][0] = c.g[S2][0];
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002c_2718, 0x001f_fc60)?; // UStr_SprintfAsciiFmt
    c.g[A2][0] = rwl_lw(c, S3, 0);
    c.g[A1][0] = c.g[S4][0];
    c.g[A0][0] = rwl_slw(c, RWL_FILE);
    call_ptr(c, 0x0015_af40, 0x001f_fc70)?; // GetRewardTypeText
    c.g[A0][0] = rwl_slw(c, RWL_OUT);
    c.g[A1][0] = c.g[S0][0];
    rwl_sprintf_out(c)
}

/// Line of reward type 2: its text is the reward type text itself.
fn rwl_type2_line(c: &mut Cpu) -> Flow {
    c.cycles += 18;
    c.g[A1][0] = 2;
    c.g[A0][0] = rwl_slw(c, RWL_FILE);
    c.g[S0][0] = rwl_lw(c, V0, 0x8c);
    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(8));
    c.g[S1][0] = rwl_lw(c, S0, 4);
    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = rwl_add(c.g[S0][0], c.g[V0][0] as u32);
    call_ptr(c, 0x0015_af40, 0x001f_fafc)?; // GetRewardTypeText
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0031_7670, 0x001f_fb04)?; // StrHash
    rwl_vcall(c, 0x001f_fb14)?;
    c.g[V1][0] = 0x470000;
    c.g[A0][0] = rwl_slw(c, RWL_OUT);
    c.g[A1][0] = rwl_add(c.g[V1][0], 0x418);
    rwl_sprintf2_out(c)
}

/// Header line of a category: the category's title, or "platinum earned"
/// for the last three categories when the platinum flag is clear.
fn rwl_header_line(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    c.g[V0][0] = 0x4a0000;
    c.g[S0][0] = 0xffff_ffff_ffff_ffff;
    c.g[S2][0] = rwl_add(c.g[V0][0], 0x22a0);
    call_ptr(c, 0x0014_dc80, 0x001f_fb68)?; // BELibrary_GetInstance
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xd;
    call_ptr(c, 0x0014_dd58, 0x001f_fb74)?; // BELibrary_GetInterface
    if c.g[V0][0] != 0 {
        c.g[S0][0] = rwl_lw(c, V0, 0x14);
    }
    c.cycles += 5;
    c.g[V1][0] = rwl_slw(c, RWL_CAT);
    c.g[V0][0] = rwl_add(c.g[V1][0], 0xffff_ffef);
    c.g[V0][0] = (c.g[V0][0] < 3u64) as u64;
    let plain = c.g[V0][0] == 0;
    c.g[A0][0] = rwl_slw(c, RWL_CAT);
    let mut platinum_earned = false;
    if !plain {
        c.cycles += 2;
        platinum_earned = c.g[S0][0] == 0;
        c.g[V0][0] = rwl_world(c);
    }
    if platinum_earned {
        c.cycles += 4;
        c.g[A0][0] = 0x470000;
        c.g[A0][0] = rwl_add(c.g[A0][0], 0x450);
        c.g[S0][0] = rwl_lw(c, V0, 0x8c);
        c.g[S1][0] = rwl_lw(c, S0, 4);
    } else {
        c.cycles += 9;
        c.g[V1][0] = rwl_world(c);
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[S0][0] = rwl_lw(c, V1, 0x8c);
        c.g[A0][0] = 0x440000;
        c.g[A0][0] = rwl_add(c.g[A0][0], 0x1c68);
        c.g[S1][0] = rwl_lw(c, S0, 4);
        c.g[V0][0] = rwl_add(c.g[V0][0], c.g[A0][0] as u32);
        c.g[A0][0] = rwl_lw(c, V0, 0);
    }
    c.cycles += 9;
    rwl_string(c, 0x001f_fca4, 0x001f_fcb4)?;
    c.g[A0][0] = rwl_slw(c, RWL_OUT);
    c.g[A1][0] = c.g[S2][0];
    rwl_sprintf2_out(c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::{check_reaching, covered, machine};
    use crate::difftest::{DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    const OC979_BASE: u32 = DATA + 0x20_0000;
    /// Gate word: the fakes only answer while this holds the magic.
    const OC979_CTRL: u32 = OC979_BASE;
    const OC979_MAGIC: u32 = 0x4f43_3739;
    /// Per forced fake: its call counter (4 bytes each).
    const OC979_CNT: u32 = OC979_BASE + 0x100;
    /// Per forced fake: its answers, 0x800 bytes each (call n answers entry
    /// `n & 0x1ff`).
    const OC979_RES: u32 = OC979_BASE + 0x1000;
    const OC979_THIS: u32 = OC979_BASE + 0x9000;
    const OC979_OUT: u32 = OC979_BASE + 0x9400;
    const OC979_WORLD: u32 = OC979_BASE + 0xa000;
    const OC979_STRTAB: u32 = OC979_BASE + 0xa400;
    const OC979_STRVT: u32 = OC979_BASE + 0xa800;
    /// The reward interface (+0x10: money) and the second, platinum one
    /// (+0x14: platinum flag).
    const OC979_IFACE0: u32 = OC979_BASE + 0xb000;
    const OC979_IFACE1: u32 = OC979_BASE + 0xb100;
    /// The bolts database: header (+4: entries) and four 0x38 byte entries.
    const OC979_DB: u32 = OC979_BASE + 0xc000;
    const OC979_DBROWS: u32 = OC979_BASE + 0xc100;

    // Forced results (index into the answer tables).
    const F_IFACE: u32 = 0; // BELibrary_GetInterface
    const F_COUNT: u32 = 1; // CountNewRewards
    const F_RCOUNT: u32 = 2; // GetNewRewardCount
    const F_TESTBIT: u32 = 3; // NewRewardsTestBit
    const F_BOLTDB: u32 = 4; // Bolts_GetDB
    const F_BOLTIDX: u32 = 5; // Bolts_FindBoltIndex
    const F_NONE: u32 = 99;

    /// The fake callee: logs like `fakes::fake_call`; the forced ones answer
    /// from their table in RAM.
    fn oc979_fake_call(c: &mut Cpu, addr: u32, which: u32) {
        let ra = c.g[RA][0] as u32;
        let call = if which < 8 {
            let n = c.bus.read32(OC979_CNT + 4 * which);
            c.bus.write32(OC979_CNT + 4 * which, n + 1);
            Some(n)
        } else {
            None
        };
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        if let Some(n) = call {
            let v = c.bus.read32(OC979_RES + which * 0x800 + 4 * (n & 0x1ff));
            c.g[V0][0] = sx(v);
        }
    }

    macro_rules! oc979_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $which:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC979_CTRL) != OC979_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc979_fake_call(c, $addr, $which);
                }
            )*
            fn oc979_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc979_fakes!(
        (
            oc979_fake_14dc80,
            OC979_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80,
            F_NONE
        ),
        (
            oc979_fake_14dd58,
            OC979_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58,
            F_IFACE
        ),
        (
            oc979_fake_158f20,
            OC979_PREV_158F20,
            ee::s01::OVERRIDE_00158F20,
            ee::s01::f_00158f20__generated,
            0x0015_8f20,
            F_NONE
        ),
        (
            oc979_fake_15a478,
            OC979_PREV_15A478,
            ee::s01::OVERRIDE_0015A478,
            ee::s01::f_0015a478__generated,
            0x0015_a478,
            F_NONE
        ),
        (
            oc979_fake_15a6f0,
            OC979_PREV_15A6F0,
            ee::s01::OVERRIDE_0015A6F0,
            ee::s01::f_0015a6f0__generated,
            0x0015_a6f0,
            F_COUNT
        ),
        (
            oc979_fake_15a750,
            OC979_PREV_15A750,
            ee::s01::OVERRIDE_0015A750,
            ee::s01::f_0015a750__generated,
            0x0015_a750,
            F_RCOUNT
        ),
        (
            oc979_fake_15a768,
            OC979_PREV_15A768,
            ee::s01::OVERRIDE_0015A768,
            ee::s01::f_0015a768__generated,
            0x0015_a768,
            F_NONE
        ),
        (
            oc979_fake_15a5f0,
            OC979_PREV_15A5F0,
            ee::s01::OVERRIDE_0015A5F0,
            ee::s01::f_0015a5f0__generated,
            0x0015_a5f0,
            F_TESTBIT
        ),
        (
            oc979_fake_15af40,
            OC979_PREV_15AF40,
            ee::s01::OVERRIDE_0015AF40,
            ee::s01::f_0015af40__generated,
            0x0015_af40,
            F_NONE
        ),
        (
            oc979_fake_317670,
            OC979_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            F_NONE
        ),
        (
            oc979_fake_2c2508,
            OC979_PREV_2C2508,
            ee::s08::OVERRIDE_002C2508,
            ee::s08::f_002c2508__generated,
            0x002c_2508,
            F_NONE
        ),
        (
            oc979_fake_198af0,
            OC979_PREV_198AF0,
            ee::s03::OVERRIDE_00198AF0,
            ee::s03::f_00198af0__generated,
            0x0019_8af0,
            F_NONE
        ),
        (
            oc979_fake_2c26d0,
            OC979_PREV_2C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c_26d0,
            F_NONE
        ),
        (
            oc979_fake_2c2718,
            OC979_PREV_2C2718,
            ee::s08::OVERRIDE_002C2718,
            ee::s08::f_002c2718__generated,
            0x002c_2718,
            F_NONE
        ),
        (
            oc979_fake_2c27c0,
            OC979_PREV_2C27C0,
            ee::s08::OVERRIDE_002C27C0,
            ee::s08::f_002c27c0__generated,
            0x002c_27c0,
            F_NONE
        ),
        (
            oc979_fake_14a130,
            OC979_PREV_14A130,
            ee::s01::OVERRIDE_0014A130,
            ee::s01::f_0014a130__generated,
            0x0014_a130,
            F_NONE
        ),
        (
            oc979_fake_14a080,
            OC979_PREV_14A080,
            ee::s01::OVERRIDE_0014A080,
            ee::s01::f_0014a080__generated,
            0x0014_a080,
            F_NONE
        ),
        (
            oc979_fake_14bdb8,
            OC979_PREV_14BDB8,
            ee::s01::OVERRIDE_0014BDB8,
            ee::s01::f_0014bdb8__generated,
            0x0014_bdb8,
            F_BOLTDB
        ),
        (
            oc979_fake_14d448,
            OC979_PREV_14D448,
            ee::s01::OVERRIDE_0014D448,
            ee::s01::f_0014d448__generated,
            0x0014_d448,
            F_BOLTIDX
        ),
    );

    /// Answer `n` of forced fake `which`.
    fn oc979_put(c: &mut Cpu, which: u32, n: u32, v: u32) {
        c.bus.write32(OC979_RES + which * 0x800 + 4 * n, v);
    }

    /// A reward list fixture. The seed picks a scenario (`seed % 10`) that
    /// forces one line kind: 0 money, 1 cheat character, 2 accessory / board
    /// graphic, 3 reward type 2, 4 other bit-tested type, 5 category header,
    /// 6 header of a platinum category, 7 "count only" (-1) or an index past
    /// the end, 8 and 9 random lists. The reward counts of category k, type j
    /// are answer `k * 10 + j` of `GetNewRewardCount`.
    fn oc979_setup(c: &mut Cpu, seed: u32, calls: u32) {
        let mut r = machine(c, seed, 0x1f_f7b8, calls);
        c.pc = 0x001f_f7b8;
        c.bus.write32(OC979_CTRL, OC979_MAGIC);
        let scen = seed % 10;
        let mut money = 0u32;
        let mut count = 1 + r.next_u32() % 3;
        let mut index = 1u32;
        let mut cnt = [0u32; 512];
        let mut bit = [0u32; 512];
        let random_lists = matches!(scen, 0 | 7 | 8 | 9);
        for k in 0..512usize {
            let j = k % 10;
            let x = r.next_u32();
            let small = 1 + x / 16 % 3;
            if random_lists {
                cnt[k] = if j == 8 {
                    if x % 10 < 3 {
                        10 + x / 16 % 3
                    } else {
                        x / 16 % 3
                    }
                } else if j == 9 {
                    if x % 10 < 5 {
                        small
                    } else {
                        0
                    }
                } else if x % 10 < 3 {
                    small
                } else {
                    0
                };
            }
            bit[k] = (r.next_u32() % 10 < 7) as u32;
        }
        match scen {
            0 => {
                money = 1 + r.next_u32() % 0xff;
                index = 0;
            }
            1 => {
                cnt[8] = 10 + r.next_u32() % 3;
            }
            2 => {
                cnt[9] = 1 + r.next_u32() % 3;
            }
            3 => {
                cnt[2] = 1;
                bit[0] = 1;
            }
            4 => {
                cnt[1] = 1 + r.next_u32() % 2;
                bit[0] = 1;
            }
            5 => {
                cnt[3] = 1;
                bit[0] = 1;
                index = 0;
            }
            6 => {
                let cat = 17 + r.next_u32() % 3;
                cnt[(cat * 10 + 3) as usize] = 1;
                bit[0] = 1;
                index = 0;
            }
            7 => {
                money = if r.next_u32() % 3 == 0 {
                    1 + r.next_u32() % 0xff
                } else {
                    0
                };
                index = if (seed / 10) % 2 == 0 { u32::MAX } else { 1000 };
            }
            _ => {
                if r.next_u32() % 2 == 0 {
                    money = 1 + r.next_u32() % 0xff;
                }
                index = if r.next_u32() % 8 == 0 {
                    u32::MAX
                } else {
                    r.next_u32() % 14
                };
                if r.next_u32() % 5 == 0 {
                    count = 0;
                }
            }
        }
        if scen == 0 || scen == 7 {
            count = 1 + r.next_u32() % 3;
        }
        for k in 0..512u32 {
            oc979_put(c, F_RCOUNT, k, cnt[k as usize]);
            oc979_put(c, F_TESTBIT, k, bit[k as usize]);
        }
        oc979_put(c, F_COUNT, 0, count);
        // Interfaces: the first has the money, the second the platinum flag.
        oc979_put(c, F_IFACE, 0, OC979_IFACE0);
        let second = if r.next_u32() % 3 == 0 {
            0
        } else {
            OC979_IFACE1
        };
        oc979_put(c, F_IFACE, 1, second);
        c.bus.write32(OC979_IFACE0 + 0x10, money);
        c.bus.write32(
            OC979_IFACE1 + 0x14,
            if r.next_u32() % 2 == 0 { 0 } else { 5 },
        );
        // Bolts database.
        oc979_put(c, F_BOLTDB, 0, OC979_DB);
        c.bus.write32(OC979_DB + 4, OC979_DBROWS);
        for row in 0..4u32 {
            let flags = r.next_u32();
            let flags = if r.next_u32() % 2 == 0 {
                flags | 0x800
            } else {
                flags & !0x800
            };
            c.bus.write32(OC979_DBROWS + 0x38 * row + 0x34, flags);
            c.bus
                .write32(OC979_DBROWS + 0x38 * row + 0x14, r.next_u32());
        }
        let found = if r.next_u32() % 3 == 0 {
            u32::MAX
        } else {
            r.next_u32() % 4
        };
        oc979_put(c, F_BOLTIDX, 0, found);
        // The string table the text lookups go through.
        c.bus.write32(GAME_GP - 0x848, OC979_WORLD);
        c.bus.write32(OC979_WORLD + 0x8c, OC979_STRTAB);
        c.bus.write32(OC979_STRTAB + 4, OC979_STRVT);
        c.bus
            .write16(OC979_STRVT + 0x20, (r.next_u32() & 0xfc) as u16);
        c.bus.write32(OC979_STRVT + 0x24, fakes::FAKE + 0x40);
        c.g[A0][0] = sx(OC979_THIS);
        c.g[A1][0] = sx(OC979_OUT);
        c.g[A2][0] = sx(index);
    }

    /// `OVScreenRewardList_GetEntryText`: every kind of line (money, the
    /// cheat character, accessory / board graphic, reward types 2 and the
    /// others, category and platinum headers, the count-only and the
    /// no-entry endings) against the generated body.
    #[test]
    fn ov_screen_reward_list_get_entry_text_matches() {
        oc979_install_fakes();
        check_reaching(
            ee::s05::f_001ff7b8__generated,
            ov_screen_reward_list_get_entry_text,
            400,
            &[
                0x001f_f888, // money line
                0x001f_f970, // cheat character
                0x001f_f9c0, // accessory
                0x001f_fa74, // accessory sprintf
                0x001f_fafc, // reward type 2
                0x001f_fc70, // other reward types
                0x001f_fb68, // category header
                0x001f_fca4, // header text lookup
                0x001f_fcd8, // no entry text
            ],
            &|c, seed| oc979_setup(c, seed, 0),
        );
    }

    /// The same lists with a callee unwinding at a random call.
    #[test]
    fn ov_screen_reward_list_get_entry_text_unwinds() {
        oc979_install_fakes();
        covered(
            ee::s05::f_001ff7b8__generated,
            ov_screen_reward_list_get_entry_text,
            300,
            &|c, seed| oc979_setup(c, seed, 40),
        );
    }
}
