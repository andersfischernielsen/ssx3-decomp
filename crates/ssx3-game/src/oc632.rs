//! Staging module oc632 (wave234): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc632`.
//!
//! Each body keeps the generated control flow block for block (the labelled
//! blocks are the generated `'b_X` blocks), so every register, cycle and call
//! matches the generated code. Calls go through `call_ptr` (the function
//! table), which is what the generated code does for the cross-shard callees.

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
    (0x0015_6858, be_copy_profile_characters, "BE_CopyProfileCharacters"),
    (0x0015_6ee0, be_reward_int_pick_random_bolt_flag800, "BERewardInt_PickRandomBoltFlag800"),
    (0x0015_7920, be_reward_int_is_peak_gold_complete, "BERewardInt_IsPeakGoldComplete"),
    (0x0015_8470, be_reward_int_unlock_video, "BERewardInt_UnlockVideo"),
    (0x0015_8618, be_reward_int_unlock_rider_item, "BERewardInt_UnlockRiderItem"),
    (0x0015_97b0, be_reward_int_add_points_for_rider, "BERewardInt_AddPointsForRider"),
    (0x0015_9818, be_reward_int_add_points, "BERewardInt_AddPoints"),
];

/// `void cBEProfileLib_copy(sBEPlayerData* dst, sBEPlayerData* src)`
/// (0x156858): copy the ten 0xf88-byte rider records from `src` to `dst`
/// and rewrite each record's 16-bit tables (the entry count is gp-0x1f08).
pub fn be_copy_profile_characters(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = 0xf88;
        frame.save(c, S0, 0x60);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0);
        c.mult(0, 5, 18, 20);
        'profile: loop {
            'record: {
                c.cycles += 11;
                c.g[A2][0] = 0xf88;
                c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x288));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A1][0] as u32));
                call_ptr(c, 0x003e_6574, 0x0015_68a8)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f08));
                c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x288), c.g[S1][0]);
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = 0;
                if t {
                    break 'record;
                }
                c.cycles += 3;
                c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                c.g[V1][0] = c.g[S1][0];
                loop {
                    c.cycles += 7;
                    c.sh(c.g[V1][0] as u32, c.g[A1][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f08));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                    if !t {
                        break 'record;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_68c8);
                    }
                }
            }
            c.cycles += 5;
            c.mult(0, 3, 18, 20);
            c.g[A0][0] = 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
            c.g[V0][0] = c.lh(c.g[A1][0] as u32);
            'scan: loop {
                'store: {
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 1);
                    if t {
                        break 'store;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.sh(c.g[V0][0] as u32, c.g[A0][0]);
                }
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[A0][0] as i64) < 525) as u64;
                if c.g[V0][0] == 0 {
                    break 'scan;
                }
                c.g[V0][0] = c.lh(c.g[A1][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0015_68f8);
                }
            }
            c.cycles += 4;
            c.g[S2][0] = c.g[A2][0];
            c.g[V0][0] = ((c.g[S2][0] as i64) < 10) as u64;
            if c.g[V0][0] == 0 {
                break 'profile;
            }
            c.mult(0, 5, 18, 20);
            if event_due(c) {
                deliver_event_here(c, 0x0015_6890);
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BERewardInt_PickRandomBoltFlag800(self, profile, character)`
/// (0x156ee0): pick one of the character's bolts that carries the 0x800
/// flag, at random (`Rand_NextMain` modulo the flagged count), and return
/// its bolt id; -1 when there is none. Bolt records are 0x38 bytes apart.
pub fn be_reward_int_pick_random_bolt_flag800(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x60);
        'pick: {
            c.g[A0][0] = 0xf88;
            frame.save(c, S3, 0x20);
            c.g[V1][0] = 0x9b50;
            c.g[S3][0] = c.g[A2][0];
            c.mult(0, 5, 5, 3);
            c.mult(0, 4, 19, 4);
            c.g[V0][0] = 0x4a0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
            frame.save(c, S1, 0x40);
            frame.save(c, S0, 0x50);
            frame.save(c, S2, 0x30);
            c.g[S0][0] = 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            frame.save(c, S4, 0x10);
            frame.save_ra(c, 0);
            c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, 0x0014_bdb8, 0x0015_6f28)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0014_d988, 0x0015_6f38)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[S4][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x0014_d998, 0x0015_6f48)?;
            let t = (c.g[S4][0] as i64) <= 0;
            c.g[A0][0] = c.g[V0][0];
            if t {
                break 'pick;
            }
            c.cycles += 2;
            c.g[A1][0] = c.g[S4][0];
            loop {
                'entry: {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                    c.g[V0][0] &= 0x800;
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        break 'entry;
                    }
                    'flagged: {
                        'slot: {
                            c.cycles += 7;
                            c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x288));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                            c.g[V0][0] =
                                sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                            let t = (c.g[V0][0] as i64) < 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            if t {
                                break 'slot;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                            c.g[V0][0] =
                                sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            break 'flagged;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] &= 0x2;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[V1][0];
                    }
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                }
                c.cycles += 2;
                let t = c.g[A1][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
                if !t {
                    break 'pick;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0015_6f58);
                }
            }
        }
        'exit: {
            c.cycles += 2;
            let t = (c.g[S0][0] as i64) <= 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            if t {
                break 'exit;
            }
            c.cycles += 5;
            call_ptr(c, 0x0031_77f0, 0x0015_6fc0)?;
            c.divu(0, 2, 16);
            'bolt_pick: {
                c.cycles += 8;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S3][0];
                c.g[V1][0] = c.hi[0];
                c.g[S0][0] = c.g[V1][0];
                call_ptr(c, 0x0014_d998, 0x0015_6fe0)?;
                c.g[A0][0] = c.g[V0][0];
                let t = (c.g[S4][0] as i64) <= 0;
                c.g[A2][0] = 0;
                if t {
                    break 'bolt_pick;
                }
                c.cycles += 1;
                loop {
                    'bolt: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                        c.g[V0][0] &= 0x800;
                        if c.g[V0][0] == 0 {
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            break 'bolt;
                        }
                        'bolt_slot: {
                            'bolt_flag: {
                                c.cycles += 7;
                                c.g[A1][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x288));
                                c.g[V0][0] = sx((c.g[A1][0] as u32) << 1);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                                let t = (c.g[V0][0] as i64) < 0;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                if t {
                                    break 'bolt_flag;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                c.g[V0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                break 'bolt_slot;
                            }
                            c.cycles += 1;
                            c.g[V0][0] = 0;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                        c.g[V0][0] &= 0x2;
                        if c.g[V0][0] != 0 {
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            break 'bolt;
                        }
                        c.cycles += 3;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                        if (c.g[S0][0] as i64) >= 0 {
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            break 'bolt;
                        }
                        // The chosen bolt: its id is the result.
                        c.cycles += 2;
                        c.g[V0][0] = c.g[A1][0];
                        break 'exit;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
                    if !t {
                        break 'bolt_pick;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_6ff0);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool BERewardInt_IsPeakGoldComplete(self, profile, character, tier)`
/// (0x157920): walk the peak table of the character (signed-byte entries,
/// a negative entry ends a list); for each entry check the medal or
/// mission tier and the event's best place, and return 1 when the gold
/// goals of every peak are met, 0 as soon as one is not.
pub fn be_reward_int_is_peak_gold_complete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x28;
        frame.save(c, S5, 0x40);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, S2, 0x70);
        c.mult(0, 0, 21, 2);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S6, 0x30);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, FP, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S0, 0x90);
        c.g[FP][0] = c.lo[0];
        frame.save(c, S1, 0x80);
        c.g[S6][0] = 0;
        frame.save(c, S7, 0x20);
        frame.save_ra(c, 0);
        'outer: loop {
            c.cycles += 4;
            c.g[V0][0] = 0x460000;
            c.g[S7][0] = 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5528));
            c.g[S1][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[V0][0] as u32));
            'inner: loop {
                'done: {
                    'entry: {
                        c.cycles += 3;
                        c.g[S0][0] = c.lb(c.g[S1][0] as u32);
                        let t = (c.g[S0][0] as i64) < 0;
                        c.g[T0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'entry;
                        }
                        'slot: {
                            'case_a24: {
                                c.cycles += 3;
                                c.g[V0][0] = 3;
                                let t = c.g[S6][0] == c.g[V0][0];
                                c.g[V0][0] = 2;
                                if t {
                                    break 'case_a24;
                                }
                                'case_1579dc: {
                                    c.cycles += 2;
                                    let t = c.g[S6][0] != c.g[V0][0];
                                    c.g[A2][0] = c.g[S3][0];
                                    if t {
                                        break 'case_1579dc;
                                    }
                                    c.cycles += 6;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A1][0] = c.g[S4][0];
                                    c.g[A3][0] = c.g[S5][0];
                                    call_ptr(c, 0x0015_89b0, 0x0015_79b0)?;
                                    let t = c.g[V0][0] != 0;
                                    c.g[V0][0] = 0;
                                    if t {
                                        break 'done;
                                    }
                                    c.cycles += 7;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A1][0] = c.g[S4][0];
                                    c.g[A2][0] = c.g[S3][0];
                                    c.g[A3][0] = c.g[S5][0];
                                    call_ptr(c, 0x0015_8a50, 0x0015_79cc)?;
                                    let t = c.g[V0][0] == 0;
                                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                                    if t {
                                        break 'slot;
                                    }
                                    c.cycles += 2;
                                    c.g[V0][0] = 0;
                                    break 'done;
                                }
                                'case_157a18: {
                                    c.cycles += 9;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A1][0] = c.g[S4][0];
                                    c.g[A3][0] = c.g[S0][0];
                                    call_ptr(c, 0x0015_8910, 0x0015_79ec)?;
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4));
                                    c.g[V0][0] = (c.g[V0][0] < 0x8) as u64;
                                    let t = c.g[V0][0] == 0;
                                    c.g[A0][0] = c.g[S2][0];
                                    if t {
                                        break 'case_157a18;
                                    }
                                    c.cycles += 4;
                                    c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                                    call_ptr(c, 0x0015_6a60, 0x0015_7a08)?;
                                    let t = c.g[V0][0] != 0;
                                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                                    if t {
                                        break 'slot;
                                    }
                                    c.cycles += 2;
                                    c.g[V0][0] = 0;
                                    break 'done;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = (c.g[A2][0] < 0x2) as u64;
                                let t = c.g[V0][0] == 0;
                                c.g[V0][0] = 0;
                                if t {
                                    break 'done;
                                }
                                break 'case_a24;
                            }
                            c.cycles += 1;
                            c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                            break 'slot;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S7][0] as i64) < 5) as u64;
                        let t = c.g[V0][0] != 0;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0015_7980);
                            }
                            continue 'inner;
                        }
                        break 'entry;
                    }
                    c.cycles += 4;
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S6][0] as i64) < 4) as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0xa));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0015_7970);
                        }
                        continue 'outer;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                }
                break 'outer;
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BERewardInt_UnlockVideo(self, profile, character, index, give)`
/// (0x158470): mark the video reward owned, and when `give` is set charge
/// its price (the reward record's halfword at +0xc, times 10) from the money.
pub fn be_reward_int_unlock_video(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[T0][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A3][0];
        call_ptr(c, 0x0014_dc80, 0x0015_84a8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xb;
        call_ptr(c, 0x0014_dd58, 0x0015_84b4)?;
        c.g[V1][0] = 0xf88;
        c.g[A2][0] = 0x9b50;
        c.mult(0, 3, 19, 3);
        c.g[S5][0] = c.g[V0][0];
        c.mult(0, 6, 18, 6);
        c.g[V0][0] = 0x4a0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xf52));
        call_ptr(c, 0x0015_73f0, 0x0015_84e8)?;
        let t = c.g[S1][0] == 0;
        c.g[A0][0] = c.g[S5][0];
        'unlock: {
            if t {
                break 'unlock;
            }
            'price: {
                c.cycles += 8;
                call_ptr(c, 0x0015_a478, 0x0015_84f8)?;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                c.g[V0][0] = sx((c.g[S4][0] as u32) << 4);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A3][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc));
                let t = (c.g[A3][0] as i64) <= 0;
                c.g[V0][0] = 0xa;
                if t {
                    break 'price;
                }
                c.cycles += 1;
                c.mult(0, 7, 7, 2);
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            call_ptr(c, 0x0015_0b48, 0x0015_8524)?;
            c.g[A0][0] = c.g[S5][0];
        }
        c.cycles += 12;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S3][0];
        call_ptr(c, 0x0015_0928, 0x0015_8534)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BERewardInt_UnlockRiderItem(self, profile, character, index, give)`
/// (0x158618): the rider-item twin of [`be_reward_int_unlock_video`]: the
/// index is `a3 - 10`, the set-bit offset is 0xf57 and the price record is
/// read at +0x28.
pub fn be_reward_int_unlock_rider_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[T0][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[S4][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xa));
        call_ptr(c, 0x0014_dc80, 0x0015_8650)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xb;
        call_ptr(c, 0x0014_dd58, 0x0015_865c)?;
        c.g[V1][0] = 0xf88;
        c.g[A2][0] = 0x9b50;
        c.mult(0, 3, 19, 3);
        c.g[S5][0] = c.g[V0][0];
        c.mult(0, 6, 18, 6);
        c.g[V0][0] = 0x4a0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xf57));
        call_ptr(c, 0x0015_73f0, 0x0015_8690)?;
        let t = c.g[S1][0] == 0;
        c.g[A0][0] = c.g[S5][0];
        'unlock: {
            if t {
                break 'unlock;
            }
            'price: {
                c.cycles += 8;
                call_ptr(c, 0x0015_a478, 0x0015_86a0)?;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = sx((c.g[S4][0] as u32) << 4);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A3][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc));
                let t = (c.g[A3][0] as i64) <= 0;
                c.g[V0][0] = 0xa;
                if t {
                    break 'price;
                }
                c.cycles += 1;
                c.mult(0, 7, 7, 2);
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            call_ptr(c, 0x0015_0b48, 0x0015_86cc)?;
            c.g[A0][0] = c.g[S5][0];
        }
        c.cycles += 12;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S3][0];
        call_ptr(c, 0x0015_0928, 0x0015_86dc)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BERewardInt_AddPointsForRider(self, rider, points)` (0x1597b0):
/// resolve the rider's profile and character and add the points to them.
pub fn be_reward_int_add_points_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a0e0, 0x0015_97dc)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a080, 0x0015_97e8)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, 0x0015_9818, 0x0015_97fc)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BERewardInt_AddPoints(self, profile, character, points, tier)`
/// (0x159818): add the points to the money, and grant each peak medal
/// reward that the new total completes (up to three peaks).
pub fn be_reward_int_add_points(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S6, 0x20);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S5, 0x30);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save_ra(c, 0x10);
        'b_159978: {
            'b_159964: {
                'b_159960: {
                    c.g[S6][0] = c.g[A3][0];
                    call_ptr(c, 0x0014_dc80, 0x0015_9850)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0xb;
                    call_ptr(c, 0x0014_dd58, 0x0015_985c)?;
                    c.g[S5][0] = c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1e90));
                    c.g[V0][0] = 0x4c0000;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3ef0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S6][0] as u32));
                    let t = c.g[A0][0] == 0;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x3ef0), c.g[V1][0]);
                    if t {
                        break 'b_159960;
                    }
                    c.cycles += 9;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x0015_09c0, 0x0015_9888)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1e90));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S5][0];
                    if t {
                        break 'b_159964;
                    }
                    c.cycles += 3;
                    c.g[S1][0] = 0;
                    c.g[S0][0] = c.g[SP][0];
                    'l_1598a8: loop {
                        c.cycles += 11;
                        c.g[A3][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S4][0];
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S3][0];
                        c.g[T0][0] = 0x3;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        call_ptr(c, 0x0015_7bf0, 0x0015_98c4)?;
                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                        c.g[V0][0] = ((c.g[S1][0] as i64) < 3) as u64;
                        let t = c.g[V0][0] != 0;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0015_98a8);
                            }
                            continue 'l_1598a8;
                        }
                        c.cycles += 13;
                        c.g[A0][0] = c.g[S5][0];
                        c.g[A3][0] = c.g[S6][0];
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S3][0];
                        call_ptr(c, 0x0015_0a90, 0x0015_98e8)?;
                        c.g[S1][0] = 0;
                        c.g[A0][0] = c.g[S4][0];
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S3][0];
                        call_ptr(c, 0x0015_8be0, 0x0015_98fc)?;
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1e90), c.g[V0][0]);
                        c.g[S0][0] = c.g[SP][0];
                        'l_159908: loop {
                            'b_15994c: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                if c.g[V0][0] != 0 {
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                    break 'b_15994c;
                                }
                                'b_159948: {
                                    c.cycles += 8;
                                    c.g[A0][0] = c.g[S4][0];
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A2][0] = c.g[S3][0];
                                    c.g[A3][0] = c.g[S1][0];
                                    c.g[T0][0] = 0x3;
                                    call_ptr(c, 0x0015_7bf0, 0x0015_992c)?;
                                    let t = c.g[V0][0] == 0;
                                    c.g[A0][0] = c.g[S4][0];
                                    if t {
                                        break 'b_159948;
                                    }
                                    c.cycles += 5;
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A2][0] = c.g[S3][0];
                                    c.g[A3][0] = c.g[S1][0];
                                    c.g[T0][0] = 0x3;
                                    call_ptr(c, 0x0015_9170, 0x0015_9948)?;
                                    break 'b_159948;
                                }
                                c.cycles += 1;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S1][0] as i64) < 3) as u64;
                            let t = c.g[V0][0] != 0;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0015_9908);
                                }
                                continue 'l_159908;
                            }
                            c.cycles += 2;
                            frame.restore(c, S0, 0x80);
                            break 'b_159978;
                        }
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = c.g[S5][0];
                break 'b_159964;
            }
            c.cycles += 5;
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S6][0];
            call_ptr(c, 0x0015_0a90, 0x0015_9974)?;
            frame.restore(c, S0, 0x80);
            break 'b_159978;
        }
        c.cycles += 9;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::fill;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    /// The BE library dummy the reward code reaches through `cBE_getBE`.
    fn be_library(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
    }

    #[test]
    fn be_copy_profile_characters_matches() {
        check(ssx3_ee::s01::f_00156858__generated, be_copy_profile_characters, 60, &|c, seed| {
            let mut r = Rng(0x6858_0000 ^ seed as u64);
            machine(c, seed, 0x156858, 0);
            let dst = DATA + 0x20_0000;
            let src = DATA + 0x30_0000;
            c.g[A0][0] = dst as u64;
            c.g[A1][0] = src as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x1f08), seed % 4);
            for i in 0..10u32 {
                let s = src + i * 0xf88;
                fill(c, &mut r, s, 0xf88 / 4);
                c.bus.write32(s + 0x288, DATA + 0x40_0000 + i * 0x100);
                c.bus.write32(dst + i * 0xf88 + 0x288, DATA + 0x40_0000 + i * 0x100);
                for k in 0..0x200u32 {
                    let v = if r.next_u32() & 1 == 0 {
                        0x8000 | (r.next_u32() as u16)
                    } else {
                        (r.next_u32() % 0x100) as u16
                    };
                    c.bus.write16(s + 0x290 + 4 * k, v);
                }
            }
            fill(c, &mut r, DATA + 0x40_0000, 0x1000);
        });
    }

    #[test]
    fn be_reward_int_pick_random_bolt_flag800_matches() {
        check(
            ssx3_ee::s01::f_00156ee0__generated,
            be_reward_int_pick_random_bolt_flag800,
            120,
            &|c, seed| {
                let mut r = Rng(0x6ee0_0000 ^ seed as u64);
                machine(c, seed, 0x156ee0, 0);
                let ch = seed % 8;
                let player = (seed / 8) % 4;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = player as u64;
                c.g[A2][0] = ch as u64;
                // Bolts_GetDB is the constant 0x4a6750: count, base and index.
                let db = 0x004a_6750u32;
                c.bus.write32(db + ch * 4 + 0x2c, seed % 6);
                c.bus.write32(db + ch * 4 + 0xa4, (seed / 3) % 4);
                c.bus.write32(db + 4, DATA + 0x20_0000);
                fill(c, &mut r, DATA + 0x20_0000, 0x400);
                let rec = 0x004a_6ca8 + player * 0x9b50 + ch * 0xf88;
                c.bus.write32(rec + 0x288, DATA + 0x40_0000);
                fill(c, &mut r, DATA + 0x40_0000, 0x400);
            },
        );
    }

    #[test]
    fn be_reward_int_is_peak_gold_complete_matches() {
        check(
            ssx3_ee::s01::f_00157920__generated,
            be_reward_int_is_peak_gold_complete,
            80,
            &|c, seed| {
                let mut r = Rng(0x7920_0000 ^ seed as u64);
                machine(c, seed, 0x157920, 0);
                be_library(c);
                c.g[A0][0] = (DATA + 0x20_0000) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = ((seed / 4) % 8) as u64;
                c.g[A3][0] = ((seed / 32) % 8) as u64;
                // The peak tables at fp + 0x45aad8 (signed-byte entries).
                fill(c, &mut r, 0x0045_a000, 0x200);
                fill(c, &mut r, DATA + 0x20_0000, 0x1000);
            },
        );
    }

    #[test]
    fn be_reward_int_unlock_video_matches() {
        check(
            ssx3_ee::s01::f_00158470__generated,
            be_reward_int_unlock_video,
            80,
            &|c, seed| {
                let mut r = Rng(0x8470_0000 ^ seed as u64);
                machine(c, seed, 0x158470, 0);
                be_library(c);
                c.g[A0][0] = (DATA + 0x20_0000) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = ((seed / 4) % 8) as u64;
                c.g[A3][0] = ((seed / 32) % 6) as u64;
                c.g[T0][0] = (seed % 2) as u64;
                fill(c, &mut r, DATA + 0x20_0000, 0x1000);
                fill(c, &mut r, 0x004a_6ca8, 0x2000);
            },
        );
    }

    #[test]
    fn be_reward_int_unlock_rider_item_matches() {
        check(
            ssx3_ee::s01::f_00158618__generated,
            be_reward_int_unlock_rider_item,
            80,
            &|c, seed| {
                let mut r = Rng(0x8618_0000 ^ seed as u64);
                machine(c, seed, 0x158618, 0);
                be_library(c);
                c.g[A0][0] = (DATA + 0x20_0000) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = ((seed / 4) % 8) as u64;
                c.g[A3][0] = (10 + (seed / 32) % 6) as u64;
                c.g[T0][0] = (seed % 2) as u64;
                fill(c, &mut r, DATA + 0x20_0000, 0x1000);
                fill(c, &mut r, 0x004a_6ca8, 0x2000);
            },
        );
    }

    #[test]
    fn be_reward_int_add_points_for_rider_matches() {
        check(
            ssx3_ee::s01::f_001597b0__generated,
            be_reward_int_add_points_for_rider,
            80,
            &|c, seed| {
                let mut r = Rng(0x97b0_0000 ^ seed as u64);
                machine(c, seed, 0x1597b0, 0);
                be_library(c);
                c.bus.write32(GAME_GP.wrapping_sub(0x1e90), seed % 40);
                c.g[A0][0] = (DATA + 0x20_0000) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 1000) as u64;
                c.g[A3][0] = (seed % 2) as u64;
                fill(c, &mut r, DATA + 0x20_0000, 0x1000);
            },
        );
    }

    #[test]
    fn be_reward_int_add_points_matches() {
        check(
            ssx3_ee::s01::f_00159818__generated,
            be_reward_int_add_points,
            80,
            &|c, seed| {
                let mut r = Rng(0x9818_0000 ^ seed as u64);
                machine(c, seed, 0x159818, 0);
                be_library(c);
                c.bus.write32(GAME_GP.wrapping_sub(0x1e90), seed % 40);
                c.g[A0][0] = (DATA + 0x20_0000) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = ((seed / 4) % 3) as u64;
                c.g[A3][0] = ((seed / 12) % 5) as u64;
                fill(c, &mut r, DATA + 0x20_0000, 0x1000);
            },
        );
    }
}
