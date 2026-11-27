//! Staging module oc987 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc987`.

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
        0x001f_e9a8,
        ov_screen_save_records_init,
        "OVScreenSaveRecords_Init",
    ),
    (
        0x001f_eca0,
        ov_screen_save_records_v26,
        "OVScreenSaveRecords_v26",
    ),
    (
        0x001f_fd08,
        ov_screen_reward_list_show_page,
        "OVScreenRewardList_ShowPage",
    ),
    (
        0x001f_ff78,
        ov_screen_reward_list_transition_in,
        "OVScreenRewardList_TransitionIn",
    ),
    (
        0x0020_87f0,
        ov_screen_session_transition_in,
        "OVScreenSession_TransitionIn",
    ),
];

/// `OVScreenSaveRecords_Init(self)` (0x001fe9a8): builds the Save Replay / Save Records screen (`self+0x40`): creates it by hash name, hides the replay title, shows the records title and help text, clears the text fields and hides memory-card slots 2..4; then a dark-blue backdrop screen and `cOVState_onCreateScreen`.
pub fn ov_screen_save_records_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1febd4: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S2][0] = c.g[A0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[A0][0] = 0x470000;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xe0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S4][0] = 0x470000;
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001fe9e8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001fe9fc)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1febd4;
            } else {
                c.cycles += 69;
                c.g[A2][0] = 0u64;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[S3][0] = 0x1;
                call_ptr(c, 0x0039c870, 0x001fea18)?;
                c.g[S7][0] = 0x470000;
                c.g[A0][0] = 0x470000;
                c.g[S6][0] = 0x4a0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf0));
                c.g[S5][0] = 0x1;
                call_ptr(c, 0x00317670, 0x001fea30)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fea3c)?;
                c.g[S0][0] = 0x4a0000;
                c.g[A0][0] = c.g[V0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2560));
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0c50, 0x001fea50)?;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x100));
                call_ptr(c, 0x00317670, 0x001fea5c)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fea68)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001fea84)?;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x118));
                call_ptr(c, 0x00317670, 0x001fea90)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fea9c)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001feab8)?;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1608));
                call_ptr(c, 0x00317670, 0x001feac4)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fead0)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001feaec)?;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x130));
                call_ptr(c, 0x00317670, 0x001feaf8)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001feb04)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0c50, 0x001feb10)?;
                c.g[A0][0] = c.g[SP][0];
                'l_1feb18: loop {
                    'b_1feb78: {
                        'b_1feb48: {
                            c.cycles += 10;
                            c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x148));
                            c.g[A2][0] = c.g[S3][0];
                            call_ptr(c, 0x00417828, 0x001feb24)?;
                            c.g[A0][0] = c.g[SP][0];
                            call_ptr(c, 0x00317670, 0x001feb2c)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001feb38)?;
                            let t = c.g[S3][0] != c.g[S5][0];
                            c.g[S1][0] = c.g[V0][0];
                            if t {
                                break 'b_1feb48;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
                                if t {
                                    break 'b_1feb78;
                                } else {
                                    break 'b_1feb48;
                                }
                            }
                        }
                        c.cycles += 12;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001feb60)?;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001feb78)?;
                        break 'b_1feb78;
                    }
                    'b_1febc8: {
                        c.cycles += 13;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x2138));
                        c.g[A2][0] = c.g[S3][0];
                        call_ptr(c, 0x00417828, 0x001feb88)?;
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x00317670, 0x001feb90)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001feb9c)?;
                        c.g[S1][0] = c.g[V0][0];
                        c.g[V0][0] = ((c.g[S3][0] as i64) < 2i64) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            break 'b_1febc8;
                        } else {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001febc4)?;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            break 'b_1febc8;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < 5i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001feb18);
                        }
                        continue 'l_1feb18;
                    } else {
                        break 'b_1febd4;
                    }
                }
            }
        }
        'b_1fec08: {
            c.cycles += 10;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1c88));
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001febe0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001febf4)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[V0][0];
            if t {
                break 'b_1fec08;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001fec08)?;
                break 'b_1fec08;
            }
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0020a380, 0x001fec10)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    })
}

/// `OVScreenSaveRecords_v26(self)` (0x001feca0, displayStatus): refreshes the memory-card status text of the Save Records screen from the memory-card manager (device string, slot name, static/enabled state of the slot widget, visibility of the two save-type widgets).
pub fn ov_screen_save_records_v26(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fed40: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x6e0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x6a0));
            c.g[S3][0] = c.g[A0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x690));
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x6d0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x6c0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x6b0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x680), c.g[RA][0]);
            call_ptr(c, 0x00227f80, 0x001feccc)?;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
            c.g[S4][0] = c.g[V0][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x001fece4)?;
            if c.g[V0][0] == 0u64 {
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                break 'b_1fed40;
            } else {
                'b_1fed1c: {
                    c.cycles += 8;
                    c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x434));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1ac));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001fed04)?;
                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x214));
                        break 'b_1fed1c;
                    } else {
                        c.cycles += 4;
                        call_ptr(c, 0x001d8240, 0x001fed14)?;
                        let t = true;
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        if t {
                            break 'b_1fed40;
                        } else {
                            break 'b_1fed1c;
                        }
                    }
                }
                'b_1fed3c: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A1][0] = 0x3;
                    if t {
                        break 'b_1fed3c;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x118));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001fed3c)?;
                        break 'b_1fed3c;
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                break 'b_1fed40;
            }
        }
        'b_1ff0e8: {
            'b_1ff0e4: {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001fed50)?;
                let t = c.g[V0][0] == 0u64;
                c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x230));
                if t {
                    break 'b_1ff0e4;
                } else {
                    'b_1fefe8: {
                        'b_1fefb4: {
                            'b_1fefa4: {
                                'b_1fef1c: {
                                    c.cycles += 86;
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x158));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x15c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001fed6c)?;
                                    c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x428));
                                    c.g[A2][0] = c.g[SP][0];
                                    c.g[A3][0] = 0x320;
                                    c.g[A0][0] = c.g[S4][0];
                                    call_ptr(c, 0x00241710, 0x001fed80)?;
                                    c.g[A0][0] = 0x470000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x130));
                                    call_ptr(c, 0x00317670, 0x001fed8c)?;
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fed98)?;
                                    c.g[A1][0] = c.g[SP][0];
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a0d00, 0x001feda4)?;
                                    c.g[A0][0] = 0x470000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf0));
                                    call_ptr(c, 0x00317670, 0x001fedb0)?;
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fedbc)?;
                                    c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x434));
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = c.g[SP][0];
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x200));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x204));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001feddc)?;
                                    c.g[A1][0] = c.g[SP][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x003a0e90, 0x001fede8)?;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001fee00)?;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[A0][0] = 0x470000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x158));
                                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    call_ptr(c, 0x00317670, 0x001fee20)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                    c.g[A0][0] = c.g[S0][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, t, 0x001fee30)?;
                                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x640));
                                    c.g[A1][0] = 0x470000;
                                    c.g[A2][0] = 0x1;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x148));
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x00417828, 0x001fee48)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x00317670, 0x001fee50)?;
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fee5c)?;
                                    c.g[A3][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x434));
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A2][0] = c.g[S2][0];
                                    c.g[A1][0] = 0u64;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x70));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x74));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001fee80)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = c.g[S2][0];
                                    call_ptr(c, 0x003a0e90, 0x001fee8c)?;
                                    c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x434));
                                    c.g[A1][0] = 0u64;
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x218));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x21c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001feea8)?;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x434));
                                    if t {
                                        break 'b_1fef1c;
                                    } else {
                                        c.cycles += 8;
                                        c.g[A1][0] = 0u64;
                                        c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x210));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001feec8)?;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[A2][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x434));
                                            break 'b_1fef1c;
                                        } else {
                                            'b_1fef18: {
                                                c.cycles += 8;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S4][0] as u32).wrapping_add(0x434));
                                                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x1b8));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x1bc));
                                                let t = c.g[V0][0] as u32;
                                                c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001feee8)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] = 0u64;
                                                if t {
                                                    break 'b_1fef18;
                                                } else {
                                                    c.cycles += 10;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x001fef04)?;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A1][0] = 0u64;
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                                    let t = true;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                                    if t {
                                                        break 'b_1fefa4;
                                                    } else {
                                                        break 'b_1fef18;
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[A2][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x434));
                                            break 'b_1fef1c;
                                        }
                                    }
                                }
                                c.cycles += 8;
                                c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x428));
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001fef34)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_1fefb4;
                                } else {
                                    'b_1fef9c: {
                                        'b_1fef7c: {
                                            c.cycles += 8;
                                            c.g[A2][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x434));
                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x210));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                            let t = c.g[V0][0] as u32;
                                            c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001fef54)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0x1;
                                            if t {
                                                break 'b_1fef7c;
                                            } else {
                                                c.cycles += 8;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                                let t = c.g[V1][0] as u32;
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001fef70)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                let t = true;
                                                c.g[A1][0] = 0x1;
                                                if t {
                                                    break 'b_1fef9c;
                                                } else {
                                                    break 'b_1fef7c;
                                                }
                                            }
                                        }
                                        c.cycles += 8;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001fef94)?;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0u64;
                                        break 'b_1fef9c;
                                    }
                                    c.cycles += 2;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                    break 'b_1fefa4;
                                }
                            }
                            c.cycles += 4;
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001fefac)?;
                            let t = true;
                            c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x428));
                            if t {
                                break 'b_1fefe8;
                            } else {
                                break 'b_1fefb4;
                            }
                        }
                        c.cycles += 13;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001fefcc)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001fefe4)?;
                        c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x428));
                        break 'b_1fefe8;
                    }
                    'b_1ff068: {
                        'b_1ff038: {
                            c.cycles += 7;
                            c.g[A0][0] = c.g[S3][0];
                            c.g[A1][0] = c.g[S4][0];
                            c.g[A3][0] = 0x1;
                            call_ptr(c, 0x001d97e0, 0x001feff8)?;
                            c.g[V1][0] = 0xffffffffffffffff;
                            let t = c.g[V0][0] != c.g[V1][0];
                            c.g[A0][0] = 0x470000;
                            if t {
                                break 'b_1ff038;
                            } else {
                                c.cycles += 13;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x168));
                                call_ptr(c, 0x00317670, 0x001ff00c)?;
                                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001ff018)?;
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0u64;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                let t = c.g[A2][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001ff030)?;
                                let t = true;
                                c.g[A1][0] = c.g[S4][0];
                                if t {
                                    break 'b_1ff068;
                                } else {
                                    break 'b_1ff038;
                                }
                            }
                        }
                        c.cycles += 12;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x168));
                        call_ptr(c, 0x00317670, 0x001ff040)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001ff04c)?;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                        let t = c.g[A2][0] as u32;
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001ff064)?;
                        c.g[A1][0] = c.g[S4][0];
                        break 'b_1ff068;
                    }
                    'b_1ff0b8: {
                        c.cycles += 7;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x428));
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x001d97e0, 0x001ff078)?;
                        c.g[V1][0] = 0xffffffffffffffff;
                        let t = c.g[V0][0] != c.g[V1][0];
                        c.g[A0][0] = 0x470000;
                        if t {
                            break 'b_1ff0b8;
                        } else {
                            c.cycles += 13;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x178));
                            call_ptr(c, 0x00317670, 0x001ff08c)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001ff098)?;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                            let t = c.g[A2][0] as u32;
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001ff0b0)?;
                            let t = true;
                            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x214));
                            if t {
                                break 'b_1ff0e8;
                            } else {
                                break 'b_1ff0b8;
                            }
                        }
                    }
                    c.cycles += 11;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x178));
                    call_ptr(c, 0x00317670, 0x001ff0c0)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001ff0cc)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                    let t = c.g[A2][0] as u32;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001ff0e4)?;
                    break 'b_1ff0e4;
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x214));
            break 'b_1ff0e8;
        }
        'b_1ff150: {
            c.cycles += 3;
            c.g[V0][0] = 0x3;
            let t = c.g[V1][0] == c.g[V0][0];
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x640));
            if t {
                break 'b_1ff150;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0x1;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x148));
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00417828, 0x001ff108)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00317670, 0x001ff110)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001ff11c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001ff138)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001ff150)?;
                break 'b_1ff150;
            }
        }
        c.cycles += 8;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x6d0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x6c0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x6b0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x6a0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x690));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x680));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x6e0));
        c.pc = t;
        return Ok(());
    })
}

/// `OVScreenRewardList_ShowPage(self, which)` (0x001ffd08, updateMenu): fills the 9 text rows of the reward list from entry `which`, greys rows past the count (`self+0x9c`) and shows or hides the up/down arrows.
pub fn ov_screen_reward_list_show_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x250));
        c.g[V0][0] = 0x4a0000;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x210));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1e0));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x200));
        c.g[S4][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1f0));
        c.g[S2][0] = 0u64;
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.g[S5][0] = 0x4a0000;
        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x248));
        c.g[S6][0] = 0x4a0000;
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x240));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x238));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x230));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x220));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x1b0), c.g[RA][0]);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 23);
        c.lwc1(22, (c.g[GP][0] as u32).wrapping_sub(0x553c));
        c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x5538));
        c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x5534));
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2580));
        c.g[V1][0] = c.lb((c.g[A2][0] as u32));
        c.g[A0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x1));
        c.sb((c.g[SP][0] as u32), c.g[V1][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[A0][0]);
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x30));
        'l_1ffd88: loop {
            'b_1ffdc4: {
                c.cycles += 13;
                c.g[A1][0] = c.g[S3][0];
                c.sb((c.g[SP][0] as u32), c.g[V0][0]);
                c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x001ff7b8, 0x001ffd9c)?;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x001ffda4)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001ffdb0)?;
                c.g[S0][0] = c.g[V0][0];
                let t = c.g[S0][0] == 0u64;
                c.g[A0][0] = c.g[S0][0];
                if t {
                    break 'b_1ffdc4;
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, 0x003a0e90, 0x001ffdc4)?;
                    break 'b_1ffdc4;
                }
            }
            'b_1ffe30: {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 10i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x1;
                if t {
                    break 'b_1ffe30;
                } else {
                    c.cycles += 23;
                    c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x1a4));
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x1a8));
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1ac));
                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x1a7));
                    c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                    c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x1af));
                    c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x1a8));
                    c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x8f));
                    c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0x88));
                    c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x97));
                    c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x90));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001ffe18)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001ffe30)?;
                    break 'b_1ffe30;
                }
            }
            c.cycles += 4;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[S2][0] as i64) < 9i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x30));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001ffd88);
                }
                continue 'l_1ffd88;
            } else {
                'b_1ffebc: {
                    'b_1ffeb8: {
                        'b_1ffe84: {
                            c.cycles += 2;
                            let t = c.g[S4][0] != 0u64;

                            if t {
                                break 'b_1ffe84;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x2570));
                                call_ptr(c, 0x00317670, 0x001ffe50)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001ffe5c)?;
                                c.g[A2][0] = c.g[V0][0];
                                let t = c.g[A2][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_1ffeb8;
                                } else {
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001ffe7c)?;
                                    let t = true;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                                    if t {
                                        break 'b_1ffebc;
                                    } else {
                                        break 'b_1ffe84;
                                    }
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x2570));
                        call_ptr(c, 0x00317670, 0x001ffe8c)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001ffe98)?;
                        c.g[A2][0] = c.g[V0][0];
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_1ffeb8;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001ffeb8)?;
                            break 'b_1ffeb8;
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                    break 'b_1ffebc;
                }
                'b_1fff40: {
                    'b_1fff3c: {
                        'b_1fff08: {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x9));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;

                            if t {
                                break 'b_1fff08;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x2578));
                                call_ptr(c, 0x00317670, 0x001ffed4)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001ffee0)?;
                                c.g[A2][0] = c.g[V0][0];
                                let t = c.g[A2][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_1fff3c;
                                } else {
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001fff00)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x220));
                                    if t {
                                        break 'b_1fff40;
                                    } else {
                                        break 'b_1fff08;
                                    }
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x2578));
                        call_ptr(c, 0x00317670, 0x001fff10)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fff1c)?;
                        c.g[A2][0] = c.g[V0][0];
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_1fff3c;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001fff3c)?;
                            break 'b_1fff3c;
                        }
                    }
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x220));
                    break 'b_1fff40;
                }
                c.cycles += 13;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x210));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x200));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x248));
                c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x240));
                c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x238));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x230));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x250));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `OVScreenRewardList_TransitionIn(self)` (0x001fff78, onGainTransition): once per screen, writes the reward
/// list title (event name or `<location> <mode>`) and the congratulation line with the human rider's name
/// into the screen's text objects, then marks the transition done (`self+0x48`); returns 0.
/// The body is a `match pc` dispatcher (the mode jump table is in the game image).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_reward_list_transition_in(c: &mut Cpu) {
    if c.pc != 0x001fff78 {
        return ee::s05::f_001fff78__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x001fff78;
        loop {
            match pc {
                0x001fff78 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x270));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x220));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x260));
                    c.g[S4][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x250));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x240));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x230));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x210));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x200), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x48));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = 0x1;
                    if t {
                        pc = 0x002001ec;
                        continue;
                    }
                    pc = 0x001fffa8;
                    continue;
                }
                0x001fffa8 => {
                    c.cycles += 22;
                    c.g[RA][0] = sx(0x001fffb0);
                    call_ptr(c, 0x0014dc80, 0x001fffb0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001fffbc);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x0014dd58, 0x001fffbc)?;
                    c.g[RA][0] = sx(0x001fffc4);
                    c.g[S5][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x001fffc4)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001fffd0);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0014dd58, 0x001fffd0)?;
                    c.g[RA][0] = sx(0x001fffd8);
                    c.g[S3][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x001fffd8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001fffe4);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x001fffe4)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V1][0] = 0x5;
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                    let t = c.g[A0][0] == c.g[V1][0];
                    c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    if t {
                        pc = 0x00200034;
                        continue;
                    }
                    pc = 0x00200000;
                    continue;
                }
                0x00200000 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 5i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = (c.g[A1][0] < 0x6u64) as u64;
                    if t {
                        pc = 0x00200068;
                        continue;
                    }
                    pc = 0x0020000c;
                    continue;
                }
                0x0020000c => {
                    c.cycles += 1;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 12i64) as u64;
                    pc = 0x00200010;
                    continue;
                }
                0x00200010 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = (c.g[A1][0] < 0x6u64) as u64;
                    if t {
                        pc = 0x00200068;
                        continue;
                    }
                    pc = 0x00200018;
                    continue;
                }
                0x00200018 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00200020);
                    call_ptr(c, 0x001fd190, 0x00200020)?;
                    pc = 0x00200020;
                    continue;
                }
                0x00200020 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0020002c);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x002c2508, 0x0020002c)?;
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        pc = 0x00200124;
                        continue;
                    }
                    pc = 0x00200034;
                    continue;
                }
                0x00200034 => {
                    c.cycles += 13;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[A1][0] = 0x470000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xae0));
                    c.g[RA][0] = sx(0x00200048);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002c2540, 0x00200048)?;
                    c.g[RA][0] = sx(0x00200050);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x00144bf0, 0x00200050)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00200060);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x002c26d0, 0x00200060)?;
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        pc = 0x00200124;
                        continue;
                    }
                    pc = 0x00200068;
                    continue;
                }
                0x00200068 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = 0u64;
                    if t {
                        pc = 0x002000f0;
                        continue;
                    }
                    pc = 0x00200070;
                    continue;
                }
                0x00200070 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x470000;
                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4a0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x0020008c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1ab8));
                    if t {
                        pc = 0x002000c8;
                        continue;
                    }
                    pc = 0x0020009c;
                    continue;
                }
                0x0020009c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c68));
                    if t {
                        pc = 0x002000c8;
                        continue;
                    }
                    pc = 0x002000ac;
                    continue;
                }
                0x002000ac => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c58));
                    if t {
                        pc = 0x002000c8;
                        continue;
                    }
                    pc = 0x002000bc;
                    continue;
                }
                0x002000bc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c40));
                    pc = 0x002000c8;
                    continue;
                }
                0x002000c8 => {
                    c.cycles += 10;
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x002000dc);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    call_ptr(c, 0x00317670, 0x002000dc)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x002000ec);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x002000ec)?;
                    c.g[S1][0] = c.g[V0][0];
                    pc = 0x002000f0;
                    continue;
                }
                0x002000f0 => {
                    c.cycles += 13;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2400));
                    c.g[RA][0] = sx(0x00200104);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002c2540, 0x00200104)?;
                    c.g[RA][0] = sx(0x0020010c);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x00144bf0, 0x0020010c)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A3][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00200120);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x002c26d0, 0x00200120)?;
                    c.g[A0][0] = 0x470000;
                    pc = 0x00200124;
                    continue;
                }
                0x00200124 => {
                    c.cycles += 50;
                    c.g[RA][0] = sx(0x0020012c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x468));
                    call_ptr(c, 0x00317670, 0x0020012c)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00200138);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00200138)?;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00200144);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x003a0e90, 0x00200144)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00200150);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00146e98, 0x00200150)?;
                    c.g[A0][0] = 0x4a0000;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00200160);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2588));
                    call_ptr(c, 0x00317670, 0x00200160)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x0020016c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x0020016c)?;
                    c.g[S0][0] = 0x4a0000;
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1c0));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2590));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[S3][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00200188);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x0014ee58, 0x00200188)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00200198);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x002c27c0, 0x00200198)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x480));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x002001b8);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    call_ptr(c, 0x00317670, 0x002001b8)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x002001c8);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x002001c8)?;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002001dc);
                    c.g[A2][0] = c.g[S2][0];
                    call_ptr(c, 0x002c26d0, 0x002001dc)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x002001e8);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x003a0e90, 0x002001e8)?;
                    c.g[V1][0] = 0x1;
                    pc = 0x002001ec;
                    continue;
                }
                0x002001ec => {
                    c.cycles += 11;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x260));
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x48), c.g[V1][0]);
                    c.g[V0][0] = 0u64;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x250));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x240));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x230));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x220));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x210));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x200));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x270));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001fff78, 0x00200218);
                    return Ok(());
                }
            }
        }
    });
}

/// `OVScreenSession_TransitionIn(self, on)` (0x002087f0, cOVState_MAP_onGainTransition): when `on` is set, labels the 8 map entries (enabled flags at `self+0x9c`), selects the current location in the menu and sets the two arrow widgets.
pub fn ov_screen_session_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        'b_208ac8: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
            c.g[S6][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
            let t = c.g[A1][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
            if t {
                break 'b_208ac8;
            } else {
                c.cycles += 21;
                c.g[S3][0] = 0u64;
                call_ptr(c, 0x0014dc80, 0x0020882c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x00208838)?;
                c.g[S7][0] = sx((c.g[S6][0] as u32).wrapping_add(0x9c));
                c.g[S1][0] = c.g[V0][0];
                c.g[FP][0] = 0x470000;
                call_ptr(c, 0x0014dc80, 0x00208848)?;
                c.g[A1][0] = 0xa;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dd58, 0x00208854)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x00144bc0, 0x00208860)?;
                c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x001545f8, 0x0020886c)?;
                c.g[S5][0] = c.g[V0][0];
                c.g[V0][0] = 0x470000;
                'l_208878: loop {
                    'b_20899c: {
                        'b_208960: {
                            c.cycles += 13;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1808));
                            c.g[A2][0] = c.g[S3][0];
                            call_ptr(c, 0x00417828, 0x00208888)?;
                            c.g[A0][0] = c.g[SP][0];
                            call_ptr(c, 0x00317670, 0x00208890)?;
                            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xd4));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x0020889c)?;
                            c.g[S2][0] = c.g[V0][0];
                            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S5][0] as i64)) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                            if t {
                                break 'b_208960;
                            } else {
                                c.cycles += 18;
                                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[V1][0] = 0x1;
                                c.g[V0][0] =
                                    sx((c.g[S7][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                                c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = 0x470000;
                                c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8c));
                                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1818));
                                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                c.g[S0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                call_ptr(c, 0x00317670, 0x002088dc)?;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                c.g[A0][0] = c.g[S0][0];
                                let t = c.g[V1][0] as u32;
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, t, 0x002088ec)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A2][0] = c.g[V0][0];
                                if t {
                                    break 'b_20899c;
                                } else {
                                    c.cycles += 7;
                                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                    c.g[V0][0] = 0x4a0000;
                                    c.g[A3][0] = c.g[S4][0];
                                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2558));
                                    call_ptr(c, 0x002c27c0, 0x00208908)?;
                                    let t = c.g[S2][0] == 0u64;
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_20899c;
                                    } else {
                                        'b_208950: {
                                            'b_208934: {
                                                'b_208924: {
                                                    c.cycles += 2;
                                                    let t = c.g[S3][0] != 0u64;
                                                    c.g[V0][0] =
                                                        sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                                                    if t {
                                                        break 'b_208924;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = 0x470000;
                                                        let t = true;
                                                        c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x1828));
                                                        if t {
                                                            break 'b_208934;
                                                        } else {
                                                            break 'b_208924;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[S3][0] != c.g[V0][0];
                                                c.g[V0][0] = 0x2;
                                                if t {
                                                    break 'b_208950;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[S5][0] == c.g[V0][0];
                                                    c.g[A0][0] =
                                                        sx((c.g[FP][0] as u32)
                                                            .wrapping_add(0x1838));
                                                    if t {
                                                        break 'b_208950;
                                                    } else {
                                                        break 'b_208934;
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            call_ptr(c, 0x00317670, 0x0020893c)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[A0][0] = c.g[S2][0];
                                            call_ptr(c, 0x003a1000, 0x00208948)?;
                                            let t = true;
                                            c.sw(
                                                (c.g[S2][0] as u32).wrapping_add(0x18),
                                                c.g[S4][0],
                                            );
                                            if t {
                                                break 'b_20899c;
                                            } else {
                                                break 'b_208950;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x003a0e90, 0x00208958)?;
                                        let t = true;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[S4][0]);
                                        if t {
                                            break 'b_20899c;
                                        } else {
                                            break 'b_208960;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 15;
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.sw((c.g[V0][0] as u32), 0u64);
                        c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x00208984)?;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0020899c)?;
                        break 'b_20899c;
                    }
                    c.cycles += 4;
                    c.g[S3][0] = c.g[S4][0];
                    c.g[V0][0] = ((c.g[S3][0] as i64) < 8i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x470000;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00208878);
                        }
                        continue 'l_208878;
                    } else {
                        'b_208a3c: {
                            c.cycles += 11;
                            c.g[A0][0] = 0x470000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1850));
                            call_ptr(c, 0x00317670, 0x002089b8)?;
                            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xd4));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x002089c4)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_208a3c;
                            } else {
                                c.cycles += 25;
                                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                                c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                call_ptr(c, t, 0x002089f8)?;
                                c.lq(3, (c.g[V0][0] as u32));
                                c.g[A0][0] = 0x4d0000;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33a0));
                                c.sq(3, (c.g[SP][0] as u32));
                                c.lwc1(0, (c.g[SP][0] as u32));
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
                                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0026b680, 0x00208a28)?;
                                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                c.g[A0][0] = c.g[S0][0];
                                if c.g[V0][0] == 0 {
                                    c.g[A1][0] = 0u64;
                                }
                                c.g[A1][0] = c.g[A1][0] & 0xff;
                                call_ptr(c, 0x0039b960, 0x00208a3c)?;
                                break 'b_208a3c;
                            }
                        }
                        'b_208a84: {
                            c.cycles += 11;
                            c.g[A0][0] = c.g[S6][0];
                            call_ptr(c, 0x00209e78, 0x00208a44)?;
                            c.g[A0][0] = 0x470000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6b8));
                            call_ptr(c, 0x00317670, 0x00208a50)?;
                            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xd4));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00208a5c)?;
                            c.g[S2][0] = c.g[V0][0];
                            let t = c.g[S2][0] == 0u64;
                            c.g[A1][0] = 0x4d0000;
                            if t {
                                break 'b_208a84;
                            } else {
                                c.cycles += 7;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x18), 0u64);
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74a8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x00208a84)?;
                                break 'b_208a84;
                            }
                        }
                        c.cycles += 9;
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c8));
                        call_ptr(c, 0x00317670, 0x00208a90)?;
                        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xd4));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00208a9c)?;
                        c.g[S2][0] = c.g[V0][0];
                        let t = c.g[S2][0] == 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_208ac8;
                        } else {
                            c.cycles += 8;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                            c.g[A1][0] = 0x4d0000;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74a8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x00208ac8)?;
                            break 'b_208ac8;
                        }
                    }
                }
            }
        }
        c.cycles += 13;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[V0][0] = 0u64;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = t;
        return Ok(());
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered};
    use crate::difftest::{elf_image, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word, magic and forced-result mailbox of this module's callee fakes
    /// (machine RAM only: both sides of a diff get their own).
    const OC987_GATE: u32 = DATA + 0x20_0000;
    const OC987_MAGIC: u32 = 0x0987_0987;
    const OC987_MBOX: u32 = DATA + 0x20_0100;
    /// The screen object every test passes in a0.
    const OC987_OBJ: u32 = DATA + 0x1000;

    /// `m = (value << 1) | 1`: the value call `n` returns instead of the fake's pointer.
    fn oc987_enc(v: i32) -> u32 {
        ((v as u32) << 1) | 1
    }

    /// A logged fake call (the shared `fakes::fake`), then the mailbox override.
    fn oc987_fake_call(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, target, ret);
        if c.pc == ret && n < 128 {
            let m = c.bus.read32(OC987_MBOX + 4 * n);
            if m & 1 != 0 {
                c.g[V0][0] = sx(((m as i32) >> 1) as u32);
            }
        }
    }

    macro_rules! oc987_fake {
        ($name:ident, $prev:ident, $addr:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC987_GATE) != OC987_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                oc987_fake_call(c, $addr);
            }
        };
    }

    oc987_fake!(
        oc987_fake_00144bc0,
        OC987_PREV_00144BC0,
        0x00144bc0,
        ee::s01::f_00144bc0__generated
    );
    oc987_fake!(
        oc987_fake_00144bf0,
        OC987_PREV_00144BF0,
        0x00144bf0,
        ee::s01::f_00144bf0__generated
    );
    oc987_fake!(
        oc987_fake_00146e98,
        OC987_PREV_00146E98,
        0x00146e98,
        ee::s01::f_00146e98__generated
    );
    oc987_fake!(
        oc987_fake_0014dc80,
        OC987_PREV_0014DC80,
        0x0014dc80,
        ee::s01::f_0014dc80__generated
    );
    oc987_fake!(
        oc987_fake_0014dd58,
        OC987_PREV_0014DD58,
        0x0014dd58,
        ee::s01::f_0014dd58__generated
    );
    oc987_fake!(
        oc987_fake_0014ee58,
        OC987_PREV_0014EE58,
        0x0014ee58,
        ee::s01::f_0014ee58__generated
    );
    oc987_fake!(
        oc987_fake_001545f8,
        OC987_PREV_001545F8,
        0x001545f8,
        ee::s01::f_001545f8__generated
    );
    oc987_fake!(
        oc987_fake_001d8240,
        OC987_PREV_001D8240,
        0x001d8240,
        ee::s04::f_001d8240__generated
    );
    oc987_fake!(
        oc987_fake_001d97e0,
        OC987_PREV_001D97E0,
        0x001d97e0,
        ee::s04::f_001d97e0__generated
    );
    oc987_fake!(
        oc987_fake_001fd190,
        OC987_PREV_001FD190,
        0x001fd190,
        ee::s04::f_001fd190__generated
    );
    oc987_fake!(
        oc987_fake_001ff7b8,
        OC987_PREV_001FF7B8,
        0x001ff7b8,
        ee::s05::f_001ff7b8__generated
    );
    oc987_fake!(
        oc987_fake_00209e78,
        OC987_PREV_00209E78,
        0x00209e78,
        ee::s05::f_00209e78__generated
    );
    oc987_fake!(
        oc987_fake_0020a380,
        OC987_PREV_0020A380,
        0x0020a380,
        ee::s05::f_0020a380__generated
    );
    oc987_fake!(
        oc987_fake_00227f80,
        OC987_PREV_00227F80,
        0x00227f80,
        ee::s05::f_00227f80__generated
    );
    oc987_fake!(
        oc987_fake_00241710,
        OC987_PREV_00241710,
        0x00241710,
        ee::s06::f_00241710__generated
    );
    oc987_fake!(
        oc987_fake_0026b680,
        OC987_PREV_0026B680,
        0x0026b680,
        ee::s07::f_0026b680__generated
    );
    oc987_fake!(
        oc987_fake_002c2508,
        OC987_PREV_002C2508,
        0x002c2508,
        ee::s08::f_002c2508__generated
    );
    oc987_fake!(
        oc987_fake_002c2540,
        OC987_PREV_002C2540,
        0x002c2540,
        ee::s08::f_002c2540__generated
    );
    oc987_fake!(
        oc987_fake_002c26d0,
        OC987_PREV_002C26D0,
        0x002c26d0,
        ee::s08::f_002c26d0__generated
    );
    oc987_fake!(
        oc987_fake_002c27c0,
        OC987_PREV_002C27C0,
        0x002c27c0,
        ee::s08::f_002c27c0__generated
    );
    oc987_fake!(
        oc987_fake_00317670,
        OC987_PREV_00317670,
        0x00317670,
        ee::s10::f_00317670__generated
    );
    oc987_fake!(
        oc987_fake_00397cc0,
        OC987_PREV_00397CC0,
        0x00397cc0,
        ee::s13::f_00397cc0__generated
    );
    oc987_fake!(
        oc987_fake_0039b960,
        OC987_PREV_0039B960,
        0x0039b960,
        ee::s13::f_0039b960__generated
    );
    oc987_fake!(
        oc987_fake_0039c870,
        OC987_PREV_0039C870,
        0x0039c870,
        ee::s13::f_0039c870__generated
    );
    oc987_fake!(
        oc987_fake_0039d860,
        OC987_PREV_0039D860,
        0x0039d860,
        ee::s13::f_0039d860__generated
    );
    oc987_fake!(
        oc987_fake_003a0c50,
        OC987_PREV_003A0C50,
        0x003a0c50,
        ee::s13::f_003a0c50__generated
    );
    oc987_fake!(
        oc987_fake_003a0d00,
        OC987_PREV_003A0D00,
        0x003a0d00,
        ee::s13::f_003a0d00__generated
    );
    oc987_fake!(
        oc987_fake_003a0e90,
        OC987_PREV_003A0E90,
        0x003a0e90,
        ee::s13::f_003a0e90__generated
    );
    oc987_fake!(
        oc987_fake_003a1000,
        OC987_PREV_003A1000,
        0x003a1000,
        ee::s13::f_003a1000__generated
    );
    oc987_fake!(
        oc987_fake_00417828,
        OC987_PREV_00417828,
        0x00417828,
        ee::s15::f_00417828__generated
    );

    fn oc987_install() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144BC0),
            oc987_fake_00144bc0,
            &OC987_PREV_00144BC0,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144BF0),
            oc987_fake_00144bf0,
            &OC987_PREV_00144BF0,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00146E98),
            oc987_fake_00146e98,
            &OC987_PREV_00146E98,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc987_fake_0014dc80,
            &OC987_PREV_0014DC80,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc987_fake_0014dd58,
            &OC987_PREV_0014DD58,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014EE58),
            oc987_fake_0014ee58,
            &OC987_PREV_0014EE58,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_001545F8),
            oc987_fake_001545f8,
            &OC987_PREV_001545F8,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001D8240),
            oc987_fake_001d8240,
            &OC987_PREV_001D8240,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001D97E0),
            oc987_fake_001d97e0,
            &OC987_PREV_001D97E0,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001FD190),
            oc987_fake_001fd190,
            &OC987_PREV_001FD190,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_001FF7B8),
            oc987_fake_001ff7b8,
            &OC987_PREV_001FF7B8,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00209E78),
            oc987_fake_00209e78,
            &OC987_PREV_00209E78,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0020A380),
            oc987_fake_0020a380,
            &OC987_PREV_0020A380,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00227F80),
            oc987_fake_00227f80,
            &OC987_PREV_00227F80,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00241710),
            oc987_fake_00241710,
            &OC987_PREV_00241710,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026B680),
            oc987_fake_0026b680,
            &OC987_PREV_0026B680,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2508),
            oc987_fake_002c2508,
            &OC987_PREV_002C2508,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2540),
            oc987_fake_002c2540,
            &OC987_PREV_002C2540,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C26D0),
            oc987_fake_002c26d0,
            &OC987_PREV_002C26D0,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C27C0),
            oc987_fake_002c27c0,
            &OC987_PREV_002C27C0,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc987_fake_00317670,
            &OC987_PREV_00317670,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00397CC0),
            oc987_fake_00397cc0,
            &OC987_PREV_00397CC0,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039B960),
            oc987_fake_0039b960,
            &OC987_PREV_0039B960,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc987_fake_0039c870,
            &OC987_PREV_0039C870,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc987_fake_0039d860,
            &OC987_PREV_0039D860,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0C50),
            oc987_fake_003a0c50,
            &OC987_PREV_003A0C50,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0D00),
            oc987_fake_003a0d00,
            &OC987_PREV_003A0D00,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0E90),
            oc987_fake_003a0e90,
            &OC987_PREV_003A0E90,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A1000),
            oc987_fake_003a1000,
            &OC987_PREV_003A1000,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc987_fake_00417828,
            &OC987_PREV_00417828,
        );
    }

    /// Machine with the gate set and a random mailbox: about one call in six
    /// returns one of `forced` instead of a pointer.
    fn oc987_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, forced: &[i32]) -> Rng {
        oc987_install();
        let mut r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC987_GATE, OC987_MAGIC);
        for n in 0..128u32 {
            if r.next_u32() % 6 == 3 {
                let v = forced[r.next_u32() as usize % forced.len()];
                c.bus.write32(OC987_MBOX + 4 * n, oc987_enc(v));
            }
        }
        c.g[A0][0] = sx(OC987_OBJ);
        r
    }

    /// Save Records screen build: null and non-null screens from the engine.
    #[test]
    fn ov_screen_save_records_init_matches() {
        covered(
            ee::s05::f_001fe9a8__generated,
            ov_screen_save_records_init,
            64,
            &|c, seed| {
                oc987_machine(c, seed, 0x1fe9a8, 60, &[0]);
            },
        );
    }

    /// Memory-card status refresh: `self+0x214 == 3` or not, card-manager
    /// answers forced to 0 / -1 (the `== -1` tests of `func_001D97E0`).
    #[test]
    fn ov_screen_save_records_v26_matches() {
        covered(
            ee::s05::f_001feca0__generated,
            ov_screen_save_records_v26,
            64,
            &|c, seed| {
                oc987_machine(c, seed, 0x1feca0, 40, &[0, -1, 1]);
                c.bus
                    .write32(OC987_OBJ + 0x214, if seed % 2 == 0 { 3 } else { 5 });
            },
        );
    }

    /// Reward list page: `which` 0 / not 0, row count below / above the page.
    #[test]
    fn ov_screen_reward_list_show_page_matches() {
        covered(
            ee::s05::f_001ffd08__generated,
            ov_screen_reward_list_show_page,
            64,
            &|c, seed| {
                oc987_machine(c, seed, 0x1ffd08, 40, &[0]);
                c.g[A1][0] = [0u64, 1, 2, 7][seed as usize % 4];
                c.bus.write32(
                    OC987_OBJ + 0x9c,
                    [3u32, 12, 20, 10][(seed as usize / 2) % 4],
                );
            },
        );
    }

    /// Reward list title: every event kind and mode of the jump table (the
    /// game image holds the table), done flag set or clear.
    #[test]
    fn ov_screen_reward_list_transition_in_matches() {
        covered(
            ee::s05::f_001fff78__generated,
            ov_screen_reward_list_transition_in,
            64,
            &|c, seed| {
                oc987_machine(c, seed, 0x1fff78, 30, &[0]);
                elf_image(c);
                c.pc = 0x001f_ff78;
                c.bus.write8(0x0053_5bc8 + 0x4a, (seed % 15) as u8);
                c.bus.write8(0x0053_5bc8 + 0x48, ((seed / 3) % 8) as u8);
                c.bus.write32(OC987_OBJ + 0x48, (seed % 5 == 4) as u32);
            },
        );
    }

    /// Map screen labels: `on` clear / set, the entry count forced to 0..9 at
    /// the call that returns it (`n`), a game world present or not.
    #[test]
    fn ov_screen_session_transition_in_matches() {
        covered(
            ee::s05::f_002087f0__generated,
            ov_screen_session_transition_in,
            64,
            &|c, seed| {
                oc987_machine(c, seed, 0x2087f0, 60, &[0, 1, 2]);
                c.g[A1][0] = (seed % 5 != 0) as u64;
                c.bus
                    .write32(OC987_MBOX + 4 * 5, oc987_enc((seed % 10) as i32));
                if seed % 2 == 1 {
                    let world = OC987_OBJ + 0x2000;
                    c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                    c.bus.write32(world + 0x84, world + 0x400);
                    c.bus.write32(world + 0x400 + 0xc, world + 0x800);
                    c.bus.write32(world + 0x800 + 0x28, world + 0xc00);
                }
            },
        );
    }
}
