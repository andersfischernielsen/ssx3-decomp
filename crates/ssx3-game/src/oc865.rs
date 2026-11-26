//! Staging module oc865 (wave wave250): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc865`.

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
/// Skipped (resumable block dispatchers, `loop { match pc`): 0x001a54d8
/// `cFEStateNetMessenger_AddNewBuddies` and 0x001a6070
/// `cFEStateNetMessenger_ShowBuddyInfo`.
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001a_63c8,
        cfe_state_net_messenger_set_row_icon,
        "cFEStateNetMessenger_SetRowIcon",
    ),
    (
        0x001a_66a8,
        cfe_state_net_messenger_set_row_colour,
        "cFEStateNetMessenger_SetRowColour",
    ),
    (
        0x001a_6978,
        cfe_state_net_messenger_refresh_player_list,
        "cFEStateNetMessenger_RefreshPlayerList",
    ),
    (
        0x001a_6ca0,
        cfe_state_net_messenger_update_next_icons,
        "cFEStateNetMessenger_UpdateNextIcons",
    ),
];

/// `void cFEStateNetMessenger_SetRowIcon(self, uint index, int state)` (0x1a63c8): show
/// one status icon of buddy row `index` (0 blocked, 1 message, 2 challenge,
/// else none) by calling the three row objects' `setVisible` (vtable +0x48).
pub fn cfe_state_net_messenger_set_row_icon(c: &mut Cpu) {
    guest(c, |c| {
        'b_1a65fc: {
            'b_1a65f8: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A1][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S3][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                let t = c.g[V0][0] == 0u64;
                c.g[S4][0] = c.g[A2][0];
                if t {
                    break 'b_1a65f8;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S2][0] < 0xfu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x460000;
                    if t {
                        break 'b_1a65f8;
                    } else {
                        'b_1a65a4: {
                            'b_1a658c: {
                                'b_1a6548: {
                                    'b_1a6500: {
                                        c.cycles += 35;
                                        c.g[A0][0] = c.g[SP][0];
                                        c.g[A2][0] = c.g[S2][0];
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x17f8));
                                        call_ptr(c, 0x0041_7828, 0x001a_6414)?;
                                        c.g[A0][0] = c.g[SP][0];
                                        call_ptr(c, 0x0031_7670, 0x001a_641c)?;
                                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039_d860, 0x001a_6428)?;
                                        c.g[A1][0] = 0x460000;
                                        c.g[A0][0] = c.g[SP][0];
                                        c.g[A2][0] = c.g[S2][0];
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1808));
                                        c.g[S0][0] = c.g[V0][0];
                                        call_ptr(c, 0x0041_7828, 0x001a_6440)?;
                                        c.g[A0][0] = c.g[SP][0];
                                        call_ptr(c, 0x0031_7670, 0x001a_6448)?;
                                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039_d860, 0x001a_6454)?;
                                        c.g[A1][0] = 0x460000;
                                        c.g[A2][0] = c.g[S2][0];
                                        c.g[A0][0] = c.g[SP][0];
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1818));
                                        c.g[S1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0041_7828, 0x001a_646c)?;
                                        c.g[A0][0] = c.g[SP][0];
                                        call_ptr(c, 0x0031_7670, 0x001a_6474)?;
                                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039_d860, 0x001a_6480)?;
                                        c.g[S2][0] = c.g[V0][0];
                                        c.g[V0][0] = 0x1;
                                        let t = c.g[S4][0] == c.g[V0][0];
                                        c.g[V0][0] = ((c.g[S4][0] as i64) < 2i64) as u64;
                                        if t {
                                            break 'b_1a6500;
                                        } else {
                                            'b_1a64b8: {
                                                'b_1a64a8: {
                                                    c.cycles += 2;
                                                    if c.g[V0][0] == 0u64 {
                                                        c.g[V0][0] = 0x2;
                                                        break 'b_1a64a8;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[S4][0] == 0u64;

                                                        if t {
                                                            break 'b_1a64b8;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;

                                                            if t {
                                                                break 'b_1a65a4;
                                                            } else {
                                                                break 'b_1a64a8;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[S4][0] == c.g[V0][0];

                                                if t {
                                                    break 'b_1a6548;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;

                                                    if t {
                                                        break 'b_1a65a4;
                                                    } else {
                                                        break 'b_1a64b8;
                                                    }
                                                }
                                            }
                                            'b_1a64d4: {
                                                c.cycles += 2;
                                                let t = c.g[S0][0] == 0u64;
                                                c.g[A1][0] = 0u64;
                                                if t {
                                                    break 'b_1a64d4;
                                                } else {
                                                    c.cycles += 5;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x001a_64d4)?;
                                                    break 'b_1a64d4;
                                                }
                                            }
                                            'b_1a64f0: {
                                                c.cycles += 2;
                                                let t = c.g[S1][0] == 0u64;
                                                c.g[A1][0] = 0x1;
                                                if t {
                                                    break 'b_1a64f0;
                                                } else {
                                                    c.cycles += 5;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x001a_64f0)?;
                                                    break 'b_1a64f0;
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = c.g[S2][0] == 0u64;
                                            c.g[A1][0] = 0u64;
                                            if t {
                                                break 'b_1a65f8;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                if t {
                                                    break 'b_1a658c;
                                                } else {
                                                    break 'b_1a6500;
                                                }
                                            }
                                        }
                                    }
                                    'b_1a651c: {
                                        c.cycles += 2;
                                        let t = c.g[S0][0] == 0u64;
                                        c.g[A1][0] = 0x1;
                                        if t {
                                            break 'b_1a651c;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x001a_651c)?;
                                            break 'b_1a651c;
                                        }
                                    }
                                    'b_1a6538: {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == 0u64;
                                        c.g[A1][0] = 0u64;
                                        if t {
                                            break 'b_1a6538;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x001a_6538)?;
                                            break 'b_1a6538;
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[S2][0] == 0u64;
                                    c.g[A1][0] = 0u64;
                                    if t {
                                        break 'b_1a65f8;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_1a658c;
                                        } else {
                                            break 'b_1a6548;
                                        }
                                    }
                                }
                                'b_1a6564: {
                                    c.cycles += 2;
                                    let t = c.g[S0][0] == 0u64;
                                    c.g[A1][0] = 0u64;
                                    if t {
                                        break 'b_1a6564;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001a_6564)?;
                                        break 'b_1a6564;
                                    }
                                }
                                'b_1a6580: {
                                    c.cycles += 2;
                                    let t = c.g[S1][0] == 0u64;
                                    c.g[A1][0] = 0u64;
                                    if t {
                                        break 'b_1a6580;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001a_6580)?;
                                        break 'b_1a6580;
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S2][0] == 0u64;
                                c.g[A1][0] = 0x1;
                                if t {
                                    break 'b_1a65f8;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                    break 'b_1a658c;
                                }
                            }
                            c.cycles += 6;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001a_659c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                            if t {
                                break 'b_1a65fc;
                            } else {
                                break 'b_1a65a4;
                            }
                        }
                        'b_1a65c0: {
                            c.cycles += 2;
                            let t = c.g[S0][0] == 0u64;
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_1a65c0;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001a_65c0)?;
                                break 'b_1a65c0;
                            }
                        }
                        'b_1a65dc: {
                            c.cycles += 2;
                            let t = c.g[S1][0] == 0u64;
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_1a65dc;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001a_65dc)?;
                                break 'b_1a65dc;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[S2][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_1a65f8;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001a_65f8)?;
                            break 'b_1a65f8;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            break 'b_1a65fc;
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    })
}

/// `void cFEStateNetMessenger_SetRowColour(self, uint index, int flag)` (0x1a66a8):
/// colour a buddy row's "persona" and "overlay" objects (`setColour`, vtable +0x5c):
/// highlighted when `flag`, else by the buddy's status (online/away/offline).
pub fn cfe_state_net_messenger_set_row_colour(c: &mut Cpu) {
    guest(c, |c| {
        'b_1a6954: {
            'b_1a6950: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S0][0] = c.g[A0][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S1][0] = c.g[A1][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                let t = c.g[V0][0] == 0u64;
                c.g[S4][0] = c.g[A2][0];
                if t {
                    break 'b_1a6950;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < 0xfu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x460000;
                    if t {
                        break 'b_1a6950;
                    } else {
                        'b_1a691c: {
                            'b_1a6918: {
                                'b_1a6914: {
                                    'b_1a6900: {
                                        'b_1a67a0: {
                                            c.cycles += 22;
                                            c.g[A0][0] = c.g[SP][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x15a0));
                                            call_ptr(c, 0x0041_7828, 0x001a_66f8)?;
                                            c.g[A0][0] = c.g[SP][0];
                                            call_ptr(c, 0x0031_7670, 0x001a_6700)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, 0x0039_d860, 0x001a_670c)?;
                                            c.g[A1][0] = 0x460000;
                                            c.g[A0][0] = c.g[SP][0];
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x1830));
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[S3][0] = c.g[V0][0];
                                            call_ptr(c, 0x0041_7828, 0x001a_6724)?;
                                            c.g[A0][0] = c.g[SP][0];
                                            call_ptr(c, 0x0031_7670, 0x001a_672c)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, 0x0039_d860, 0x001a_6738)?;
                                            let t = c.g[S4][0] == 0u64;
                                            c.g[S2][0] = c.g[V0][0];
                                            if t {
                                                break 'b_1a67a0;
                                            } else {
                                                c.cycles += 24;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                c.g[S0][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 20);
                                                c.g[A1][0] = c.g[S0][0];
                                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x596c));
                                                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5968));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
                                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x28));
                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                                let t = c.g[V0][0] as u32;
                                                c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x001a_677c)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                                                c.g[A1][0] = c.g[S0][0];
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x2c));
                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                c.g[A0][0] = sx((c.g[S3][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x24));
                                                let t = true;
                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x28));
                                                if t {
                                                    break 'b_1a6900;
                                                } else {
                                                    break 'b_1a67a0;
                                                }
                                            }
                                        }
                                        'b_1a67f0: {
                                            c.cycles += 11;
                                            c.g[A1][0] = c.g[S1][0];
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x001a_4740, 0x001a_67ac)?;
                                            c.g[S4][0] = c.g[V0][0];
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x6e8));
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x6ec));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_sub(c.g[V1][0] as u32));
                                            c.g[V0][0] =
                                                sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                                            c.g[V0][0] = (c.g[S4][0] < c.g[V0][0]) as u64;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[V1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                                                break 'b_1a67f0;
                                            } else {
                                                c.cycles += 9;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                c.g[A1][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                                                c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                                                let t = true;
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                                                if t {
                                                    break 'b_1a6900;
                                                } else {
                                                    break 'b_1a67f0;
                                                }
                                            }
                                        }
                                        'b_1a6834: {
                                            c.cycles += 15;
                                            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                            c.g[AT][0] = 0x3f800000;
                                            c.mtc1(1, 20);
                                            c.g[S1][0] = c.g[A1][0];
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                            let t = c.g[V0][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[S3][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x001a_6820)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                            let t = c.g[V1][0] != 0u64;

                                            if t {
                                                break 'b_1a6834;
                                            } else {
                                                c.cycles += 2;
                                                call_ptr(c, 0x0026_0f80, 0x001a_6834)?;
                                                break 'b_1a6834;
                                            }
                                        }
                                        'b_1a6850: {
                                            c.cycles += 4;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                            call_ptr(c, 0x0026_1460, 0x001a_683c)?;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                            if t {
                                                break 'b_1a6850;
                                            } else {
                                                c.cycles += 3;
                                                c.g[A0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                                call_ptr(c, 0x0026_13c8, 0x001a_684c)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                                break 'b_1a6850;
                                            }
                                        }
                                        'b_1a6864: {
                                            c.cycles += 2;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x6e8));
                                                break 'b_1a6864;
                                            } else {
                                                c.cycles += 3;
                                                c.g[A0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                                call_ptr(c, 0x0025_b800, 0x001a_6860)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x6e8));
                                                break 'b_1a6864;
                                            }
                                        }
                                        c.cycles += 12;
                                        c.g[V1][0] = sx((c.g[S4][0] as u32) << 2);
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                                        call_ptr(c, 0x0026_24f8, 0x001a_6878)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x001a_4670, 0x001a_6884)?;
                                        c.g[V1][0] = c.g[V0][0];
                                        c.g[V0][0] = 0x1;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                                        if t {
                                            break 'b_1a6914;
                                        } else {
                                            'b_1a68e0: {
                                                'b_1a68ac: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x2;
                                                    if t {
                                                        break 'b_1a68ac;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] == 0u64;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x8));
                                                        if t {
                                                            break 'b_1a68e0;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A1][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_1a691c;
                                                            } else {
                                                                break 'b_1a68ac;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[V1][0] != c.g[V0][0];
                                                c.g[V1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                if t {
                                                    break 'b_1a6918;
                                                } else {
                                                    c.cycles += 11;
                                                    c.g[A1][0] = c.g[S1][0];
                                                    c.lwc1(
                                                        0,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x5964),
                                                    );
                                                    c.lwc1(
                                                        1,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x5960),
                                                    );
                                                    c.lwc1(
                                                        2,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x595c),
                                                    );
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                                    c.swc1(
                                                        20,
                                                        (c.g[SP][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.swc1(
                                                        0,
                                                        (c.g[SP][0] as u32).wrapping_add(0x24),
                                                    );
                                                    c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    c.swc1(
                                                        1,
                                                        (c.g[SP][0] as u32).wrapping_add(0x28),
                                                    );
                                                    let t = true;
                                                    c.swc1(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x2c),
                                                    );
                                                    if t {
                                                        break 'b_1a6900;
                                                    } else {
                                                        break 'b_1a68e0;
                                                    }
                                                }
                                            }
                                            c.cycles += 8;
                                            c.g[A1][0] = c.g[S1][0];
                                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5958));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x2c));
                                            c.g[A0][0] =
                                                sx((c.g[S2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
                                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x28));
                                            break 'b_1a6900;
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V0][0] as u32;
                                    call_ptr(c, t, 0x001a_690c)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                    if t {
                                        break 'b_1a6954;
                                    } else {
                                        break 'b_1a6914;
                                    }
                                }
                                c.cycles += 1;
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                break 'b_1a6918;
                            }
                            c.cycles += 1;
                            c.g[A1][0] = c.g[S1][0];
                            break 'b_1a691c;
                        }
                        c.cycles += 13;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5954));
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5950));
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x594c));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x2c));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001a_6950)?;
                        break 'b_1a6950;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            break 'b_1a6954;
        }
        c.cycles += 8;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x90));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    })
}

/// `void cFEStateNetMessenger_RefreshPlayerList(self)` (0x1a6978): rebuild the 15
/// visible roster rows from the name list (scroll offset, empty rows), resize
/// the "playerMenu" scroll menu and highlight the selected row.
pub fn cfe_state_net_messenger_refresh_player_list(c: &mut Cpu) {
    guest(c, |c| {
        'b_1a6c24: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            let t = c.g[V0][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            if t {
                break 'b_1a6c24;
            } else {
                'b_1a69e4: {
                    c.cycles += 11;
                    call_ptr(c, 0x001a_6e70, 0x001a_69ac)?;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x001a_6ca0, 0x001a_69b4)?;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6e8));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6ec));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                    c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                    c.g[V1][0] = (c.g[A1][0] < 0x10u64) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.g[S3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6f4));
                    if t {
                        break 'b_1a69e4;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[S3][0] as u32));
                        c.g[V1][0] = 0xe;
                        c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf));
                        if c.g[V1][0] == 0 {
                            c.g[S3][0] = c.g[A0][0];
                        }
                        break 'b_1a69e4;
                    }
                }
                'b_1a6b0c: {
                    c.cycles += 5;
                    c.g[V0][0] = (c.g[S3][0] < c.g[A1][0]) as u64;
                    c.g[S4][0] = 0u64;
                    c.g[A1][0] = c.g[A2][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[S2][0] = 0u64;
                    if t {
                        break 'b_1a6b0c;
                    } else {
                        'l_1a69f8: loop {
                            'b_1a6a14: {
                                c.cycles += 5;
                                c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                c.g[V0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                let t = c.g[V1][0] != 0u64;
                                c.g[S0][0] = c.lw(c.g[V0][0] as u32);
                                if t {
                                    break 'b_1a6a14;
                                } else {
                                    c.cycles += 2;
                                    call_ptr(c, 0x0026_0f80, 0x001a_6a14)?;
                                    break 'b_1a6a14;
                                }
                            }
                            'b_1a6a30: {
                                c.cycles += 4;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                call_ptr(c, 0x0026_1460, 0x001a_6a1c)?;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                if t {
                                    break 'b_1a6a30;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    call_ptr(c, 0x0026_13c8, 0x001a_6a2c)?;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    break 'b_1a6a30;
                                }
                            }
                            'b_1a6a44: {
                                c.cycles += 2;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                if t {
                                    break 'b_1a6a44;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                    call_ptr(c, 0x0025_b800, 0x001a_6a40)?;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    break 'b_1a6a44;
                                }
                            }
                            'b_1a6ae4: {
                                'b_1a6a74: {
                                    c.cycles += 5;
                                    c.g[A1][0] = c.g[S0][0];
                                    call_ptr(c, 0x0026_24f8, 0x001a_6a4c)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    let t = (c.g[S0][0] as i64) >= 0;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    if t {
                                        break 'b_1a6a74;
                                    } else {
                                        c.cycles += 7;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A1][0] = c.g[S2][0];
                                        c.g[A2][0] = 0u64;
                                        c.g[A3][0] = 0xffffffffffffffff;
                                        call_ptr(c, 0x001a_71b0, 0x001a_6a6c)?;
                                        let t = true;
                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_1a6ae4;
                                        } else {
                                            break 'b_1a6a74;
                                        }
                                    }
                                }
                                'b_1a6a84: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                                    if t {
                                        break 'b_1a6a84;
                                    } else {
                                        c.cycles += 2;
                                        call_ptr(c, 0x0026_0f80, 0x001a_6a84)?;
                                        break 'b_1a6a84;
                                    }
                                }
                                'b_1a6aa0: {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    call_ptr(c, 0x0026_1460, 0x001a_6a8c)?;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    if t {
                                        break 'b_1a6aa0;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        call_ptr(c, 0x0026_13c8, 0x001a_6a9c)?;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        break 'b_1a6aa0;
                                    }
                                }
                                'b_1a6ab4: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    if t {
                                        break 'b_1a6ab4;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                        call_ptr(c, 0x0025_b800, 0x001a_6ab0)?;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        break 'b_1a6ab4;
                                    }
                                }
                                c.cycles += 12;
                                c.g[A1][0] = c.g[S0][0];
                                call_ptr(c, 0x0026_25c8, 0x001a_6abc)?;
                                c.g[A2][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = c.g[S2][0];
                                c.g[A3][0] = c.g[S0][0];
                                call_ptr(c, 0x001a_71b0, 0x001a_6ad0)?;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = c.g[S2][0];
                                c.g[A2][0] = 0u64;
                                call_ptr(c, 0x001a_66a8, 0x001a_6ae0)?;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                break 'b_1a6ae4;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S2][0] as i64) < 15i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            if t {
                                break 'b_1a6b0c;
                            } else {
                                c.cycles += 7;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6e8));
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6ec));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                                c.g[V0][0] = (c.g[S3][0] < c.g[V0][0]) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A1][0] = c.g[V1][0];
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x001a_69f8);
                                    }
                                    continue 'l_1a69f8;
                                } else {
                                    break 'b_1a6b0c;
                                }
                            }
                        }
                    }
                }
                'b_1a6be0: {
                    c.cycles += 9;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1590));
                    call_ptr(c, 0x0031_7670, 0x001a_6b18)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x001a_6b24)?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] == 0u64 {
                        c.g[S2][0] = c.g[S4][0];
                        break 'b_1a6be0;
                    } else {
                        'b_1a6b7c: {
                            'b_1a6b78: {
                                c.cycles += 7;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6ec));
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6e8));
                                c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x96));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                                let t = c.g[A0][0] == c.g[V0][0];
                                c.g[A1][0] = c.g[V0][0] & 0xff;
                                if t {
                                    break 'b_1a6b78;
                                } else {
                                    c.cycles += 8;
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x0039_b760, 0x001a_6b54)?;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6ec));
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6e8));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x95));
                                        break 'b_1a6b7c;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x1;
                                        call_ptr(c, 0x0039_b760, 0x001a_6b78)?;
                                        break 'b_1a6b78;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[A1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x95));
                            break 'b_1a6b7c;
                        }
                        'b_1a6bd4: {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x001a_6070, 0x001a_6b84)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[S1][0];
                            if t {
                                break 'b_1a6bd4;
                            } else {
                                c.cycles += 6;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6ec));
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6e8));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                                let t = c.g[V0][0] == 0u64;

                                if t {
                                    break 'b_1a6bd4;
                                } else {
                                    c.cycles += 12;
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x95));
                                    c.g[A1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x98));
                                    c.g[A1][0] =
                                        sx((c.g[V0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                                    call_ptr(c, 0x001a_5fb8, 0x001a_6bb4)?;
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x98));
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x95));
                                    c.g[A2][0] = 0x1;
                                    c.g[A1][0] =
                                        sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                                    call_ptr(c, 0x001a_66a8, 0x001a_6bcc)?;
                                    let t = true;
                                    c.g[S2][0] = c.g[S4][0];
                                    if t {
                                        break 'b_1a6be0;
                                    } else {
                                        break 'b_1a6bd4;
                                    }
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[A1][0] = 0xffffffffffffffff;
                        call_ptr(c, 0x001a_5fb8, 0x001a_6bdc)?;
                        c.g[S2][0] = c.g[S4][0];
                        break 'b_1a6be0;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 15i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                if t {
                    break 'b_1a6c24;
                } else {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S2][0];
                    'l_1a6bf0: loop {
                        c.cycles += 12;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0xffffffffffffffff;
                        call_ptr(c, 0x001a_71b0, 0x001a_6c00)?;
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A2][0] = 0u64;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        call_ptr(c, 0x001a_66a8, 0x001a_6c14)?;
                        c.g[V0][0] = ((c.g[S2][0] as i64) < 15i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = c.g[S2][0];
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001a_6bf0);
                            }
                            continue 'l_1a6bf0;
                        } else {
                            c.cycles += 1;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                            break 'b_1a6c24;
                        }
                    }
                }
            }
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    })
}

/// `void cFEStateNetMessenger_UpdateNextIcons(self)` (0x1a6ca0): show the
/// "next challenge / next message" hints from the first pending buddy.
pub fn cfe_state_net_messenger_update_next_icons(c: &mut Cpu) {
    guest(c, |c| {
        'b_1a6e5c: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_1a6e5c;
            } else {
                'b_1a6d10: {
                    c.cycles += 14;
                    call_ptr(c, 0x001a_52b0, 0x001a_6ccc)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6f8));
                    c.g[A0][0] = 0x4a0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a20));
                    c.g[V0][0] = !(0u64 | c.g[V0][0]);
                    c.g[S2][0] = sx((c.g[V0][0] as u32) >> 31);
                    call_ptr(c, 0x0031_7670, 0x001a_6ce4)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x001a_6cf0)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = c.g[S2][0];
                    if t {
                        break 'b_1a6d10;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001a_6d10)?;
                        break 'b_1a6d10;
                    }
                }
                'b_1a6d48: {
                    c.cycles += 9;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1570));
                    call_ptr(c, 0x0031_7670, 0x001a_6d1c)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x001a_6d28)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = c.g[S2][0];
                    if t {
                        break 'b_1a6d48;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001a_6d48)?;
                        break 'b_1a6d48;
                    }
                }
                'b_1a6e58: {
                    c.cycles += 15;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1840));
                    call_ptr(c, 0x0031_7670, 0x001a_6d54)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x001a_6d60)?;
                    c.g[A0][0] = 0x460000;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1858));
                    call_ptr(c, 0x0031_7670, 0x001a_6d70)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x001a_6d7c)?;
                    let t = c.g[S1][0] == 0u64;
                    c.g[S0][0] = c.g[V0][0];
                    if t {
                        break 'b_1a6e58;
                    } else {
                        c.cycles += 2;
                        if c.g[S0][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_1a6e5c;
                        } else {
                            'b_1a6e2c: {
                                'b_1a6dfc: {
                                    'b_1a6da0: {
                                        c.cycles += 2;
                                        let t = c.g[S2][0] != 0u64;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        if t {
                                            break 'b_1a6da0;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            let t = true;
                                            c.g[A1][0] = 0u64;
                                            if t {
                                                break 'b_1a6dfc;
                                            } else {
                                                break 'b_1a6da0;
                                            }
                                        }
                                    }
                                    'b_1a6db0: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] != 0u64;

                                        if t {
                                            break 'b_1a6db0;
                                        } else {
                                            c.cycles += 2;
                                            call_ptr(c, 0x0026_0f80, 0x001a_6db0)?;
                                            break 'b_1a6db0;
                                        }
                                    }
                                    'b_1a6dcc: {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        call_ptr(c, 0x0026_1460, 0x001a_6db8)?;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        if t {
                                            break 'b_1a6dcc;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                            call_ptr(c, 0x0026_13c8, 0x001a_6dc8)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                            break 'b_1a6dcc;
                                        }
                                    }
                                    'b_1a6de0: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        if t {
                                            break 'b_1a6de0;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                            call_ptr(c, 0x0025_b800, 0x001a_6ddc)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                            break 'b_1a6de0;
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xbc));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 2) as u32);
                                    let t = c.g[V1][0] == 0u64;
                                    c.g[A1][0] = c.g[S2][0];
                                    if t {
                                        break 'b_1a6e2c;
                                    } else {
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        break 'b_1a6dfc;
                                    }
                                }
                                c.cycles += 12;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001a_6e0c)?;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0u64;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001a_6e24)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_1a6e5c;
                                } else {
                                    break 'b_1a6e2c;
                                }
                            }
                            c.cycles += 11;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001a_6e40)?;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001a_6e58)?;
                            break 'b_1a6e58;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_1a6e5c;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, fresh, Rng, DATA, GAME_GP};
    use std::collections::HashMap;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;
    use std::sync::Mutex;

    // Fixture blocks, all clear of the fake log (DATA+0x100000), the fake
    // pointers (DATA..DATA+0x40000) and the oc736 mailboxes.
    const OC865_CTRL: u32 = DATA + 0x20_0000;
    const OC865_MAGIC: u32 = 0x4f43_3836;
    /// Two words per gated fake (mode, value) at `OC865_MAIL + (addr & 0xffff)`.
    const OC865_MAIL: u32 = DATA + 0x34_0000;
    const OC865_SELF: u32 = DATA + 0x22_0000;
    const OC865_SCREEN: u32 = DATA + 0x22_1000;
    const OC865_OBJS: u32 = DATA + 0x22_2000;
    const OC865_VT: u32 = DATA + 0x22_3000;
    const OC865_NAMES: u32 = DATA + 0x22_4000;
    const OC865_SESS: u32 = DATA + 0x22_5000;
    const OC865_CH_ARR: u32 = DATA + 0x22_5100;
    const OC865_STR_OBJ: u32 = DATA + 0x22_5200;
    const OC865_STR_DATA: u32 = DATA + 0x22_5310;
    const OC865_MENU: u32 = DATA + 0x22_6000;

    fn oc865_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    fn oc865_gp(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    /// `StrHash` of `s`, by the generated hasher (cached).
    fn oc865_hash(s: &str) -> u32 {
        static CACHE: Mutex<Option<HashMap<String, u32>>> = Mutex::new(None);
        let mut cache = CACHE.lock().unwrap_or_else(|p| p.into_inner());
        let map = cache.get_or_insert_with(HashMap::new);
        if let Some(h) = map.get(s) {
            return *h;
        }
        let mut c = fresh();
        for (i, b) in s.bytes().enumerate() {
            c.bus.write8(DATA + i as u32, b);
        }
        c.bus.write8(DATA + s.len() as u32, 0);
        c.g[A0][0] = DATA as u64;
        c.g[RA][0] = 0x0012_3450;
        c.g[GP][0] = GAME_GP as u64;
        c.pc = 0x0031_7670;
        ee::s10::f_00317670(&mut c);
        let h = c.g[V0][0] as u32;
        map.insert(s.to_string(), h);
        h
    }

    /// Write the C string `s` into guest RAM at `addr`.
    fn oc865_str(c: &mut Cpu, addr: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    fn oc865_mail(addr: u32) -> u32 {
        OC865_MAIL + (addr & 0xffff)
    }

    /// Make the gated fake of `addr` return `v` (sign-extended) in `v0`.
    fn oc865_force(c: &mut Cpu, addr: u32, v: u32) {
        oc865_w(c, oc865_mail(addr), 1);
        oc865_w(c, oc865_mail(addr) + 4, v);
    }

    /// Make the gated fake of `addr` return the roster index of the id in
    /// `a1` (-1 when its top bit is set).
    fn oc865_force_ids(c: &mut Cpu, addr: u32) {
        oc865_w(c, oc865_mail(addr), 2);
    }

    /// The gated fake callee: logs the call like `fakes::fake_call`, then
    /// returns the forced result of its mailbox (unless it unwound).
    fn oc865_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = oc865_mail(addr);
        let mode = c.bus.read32(mail);
        let val = c.bus.read32(mail + 4);
        let a1 = c.g[A1][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc == ra {
            match mode {
                1 => c.g[V0][0] = sx(val),
                2 => {
                    c.g[V0][0] = if a1 & 0x8000_0000 != 0 {
                        sx(0xffff_ffff)
                    } else {
                        sx(a1 & 0xf)
                    }
                }
                _ => {}
            }
        }
    }

    macro_rules! oc865_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC865_CTRL) != OC865_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc865_fake_call(c, $addr);
                }
            )*
            fn oc865_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc865_fakes!(
        (
            oc865_fake_260f80,
            OC865_PREV_260F80,
            ee::s06::OVERRIDE_00260F80,
            ee::s06::f_00260f80__generated,
            0x0026_0f80
        ),
        (
            oc865_fake_261460,
            OC865_PREV_261460,
            ee::s06::OVERRIDE_00261460,
            ee::s06::f_00261460__generated,
            0x0026_1460
        ),
        (
            oc865_fake_2613c8,
            OC865_PREV_2613C8,
            ee::s06::OVERRIDE_002613C8,
            ee::s06::f_002613c8__generated,
            0x0026_13c8
        ),
        (
            oc865_fake_25b800,
            OC865_PREV_25B800,
            ee::s06::OVERRIDE_0025B800,
            ee::s06::f_0025b800__generated,
            0x0025_b800
        ),
        (
            oc865_fake_2624f8,
            OC865_PREV_2624F8,
            ee::s07::OVERRIDE_002624F8,
            ee::s07::f_002624f8__generated,
            0x0026_24f8
        ),
        (
            oc865_fake_2625c8,
            OC865_PREV_2625C8,
            ee::s07::OVERRIDE_002625C8,
            ee::s07::f_002625c8__generated,
            0x0026_25c8
        ),
        (
            oc865_fake_1a4670,
            OC865_PREV_1A4670,
            ee::s03::OVERRIDE_001A4670,
            ee::s03::f_001a4670__generated,
            0x001a_4670
        ),
        (
            oc865_fake_1a5800,
            OC865_PREV_1A5800,
            ee::s03::OVERRIDE_001A5800,
            ee::s03::f_001a5800__generated,
            0x001a_5800
        ),
        (
            oc865_fake_262938,
            OC865_PREV_262938,
            ee::s07::OVERRIDE_00262938,
            ee::s07::f_00262938__generated,
            0x0026_2938
        ),
        (
            oc865_fake_263630,
            OC865_PREV_263630,
            ee::s07::OVERRIDE_00263630,
            ee::s07::f_00263630__generated,
            0x0026_3630
        ),
        (
            oc865_fake_39b760,
            OC865_PREV_39B760,
            ee::s13::OVERRIDE_0039B760,
            ee::s13::f_0039b760__generated,
            0x0039_b760
        ),
        (
            oc865_fake_1a5fb8,
            OC865_PREV_1A5FB8,
            ee::s03::OVERRIDE_001A5FB8,
            ee::s03::f_001a5fb8__generated,
            0x001a_5fb8
        ),
        (
            oc865_fake_41610c,
            OC865_PREV_41610C,
            ee::s15::OVERRIDE_0041610C,
            ee::s15::f_0041610c__generated,
            0x0041_610c
        ),
    );

    /// The vtable shared by every fixture object: `setVisible` (+0x48) and
    /// `setColour` (+0x5c) with random deltas, both at a fake.
    fn oc865_vtable(c: &mut Cpu, r: &mut Rng) {
        for off in [0x48u32, 0x58] {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(OC865_VT + off, delta as u16);
            oc865_w(c, OC865_VT + off + 4, FAKE);
        }
    }

    /// A UI screen at `OC865_SCREEN` holding `objs` (pointer, hash) and
    /// `extra` more scanned entries past the terminating null.
    fn oc865_screen(c: &mut Cpu, objs: &[(u32, u32)], extra: u32) {
        let list = OC865_SCREEN + 0x100;
        let count = OC865_SCREEN + 0x80;
        oc865_w(c, OC865_SCREEN + 0x3c, list);
        oc865_w(c, OC865_SCREEN + 0x38, count);
        oc865_w(c, count, objs.len() as u32 + extra);
        for (i, (obj, hash)) in objs.iter().enumerate() {
            oc865_w(c, list + 4 * i as u32, *obj);
            oc865_w(c, *obj + 8, OC865_VT);
            oc865_w(c, *obj + 0x38, *hash);
        }
        oc865_w(c, list + 4 * objs.len() as u32, 0);
    }

    /// Common setup: gated fakes on, the session pointer, the vtable, the
    /// screen pointer of `self`.
    fn oc865_base(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        calls: u32,
        session: bool,
        screen: bool,
    ) -> Rng {
        oc865_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        oc865_w(c, OC865_CTRL, OC865_MAGIC);
        oc865_w(c, oc865_gp(0x238), if session { OC865_SESS } else { 0 });
        oc865_w(c, oc865_gp(-0xc8), OC865_SESS + 0x800);
        oc865_vtable(c, &mut r);
        oc865_w(c, OC865_SELF + 0x40, if screen { OC865_SCREEN } else { 0 });
        c.g[A0][0] = OC865_SELF as u64;
        r
    }

    /// Row-object names: `{M,B,C}%02d` for `SetRowIcon`.
    fn oc865_icon_formats(c: &mut Cpu) {
        oc865_str(c, 0x0046_17f8, "M%02d");
        oc865_str(c, 0x0046_1808, "B%02d");
        oc865_str(c, 0x0046_1818, "C%02d");
    }

    /// `SetRowIcon` shows one of three row icons by `state` (0 blocked,
    /// 1 message, 2 challenge, else none); the fixture varies which icon
    /// objects exist, the state (also values outside 32 bits), the row
    /// (including 15 and above) and whether the screen exists.
    #[test]
    fn cfe_state_net_messenger_set_row_icon_matches() {
        check(
            ee::s03::f_001a63c8__generated,
            cfe_state_net_messenger_set_row_icon,
            300,
            &|c, seed| {
                let mut r = oc865_base(c, seed, 0x001a_63c8, 4, true, seed % 13 != 12);
                oc865_icon_formats(c);
                let state = [0u64, 1, 2, 3, 5, u64::MAX, 0x1_0000_0001][(seed % 7) as usize];
                let present = (seed / 7) % 8;
                let index = [0u32, 3, 14, 15, 200][((seed / 56) % 5) as usize];
                c.g[A1][0] = index as u64;
                c.g[A2][0] = state;
                let mut objs = vec![(OC865_OBJS, 0x1234_5678u32)];
                for (bit, (prefix, slot)) in
                    [("M", 1u32), ("B", 2), ("C", 3)].into_iter().enumerate()
                {
                    if present & (1 << bit) != 0 {
                        let name = format!("{prefix}{index:02}");
                        objs.push((OC865_OBJS + 0x100 * slot, oc865_hash(&name)));
                    }
                }
                oc865_screen(c, &objs, r.next_u32() & 1);
            },
        );
    }

    /// Float constants `SetRowColour` loads from `gp - 0x5970 ..`.
    fn oc865_colour_constants(c: &mut Cpu, r: &mut Rng) {
        for i in 0..12u32 {
            let v = if i % 3 == 0 {
                0x3f80_0000
            } else {
                r.next_u32()
            };
            oc865_w(c, oc865_gp(-0x5970) + 4 * i, v);
        }
    }

    /// `SetRowColour` colours a row's persona and overlay objects: highlighted
    /// when `flag`, otherwise by the buddy status of the row's name (offline,
    /// away or online) or untouched when the row is past the name list.
    #[test]
    fn cfe_state_net_messenger_set_row_colour_matches() {
        check(
            ee::s03::f_001a66a8__generated,
            cfe_state_net_messenger_set_row_colour,
            300,
            &|c, seed| {
                let mut r = oc865_base(c, seed, 0x001a_66a8, 4, seed % 7 != 6, seed % 13 != 12);
                oc865_str(c, 0x0046_15a0, "P%02d");
                oc865_str(c, 0x0046_1830, "O%02d");
                oc865_colour_constants(c, &mut r);
                let index = [0u32, 1, 3, 14, 15, 99][(seed % 6) as usize];
                let flag = [0u64, 0, 1, 0xffff_ffff_ffff_ffff][((seed / 6) % 4) as usize];
                c.g[A1][0] = index as u64;
                c.g[A2][0] = flag;
                let objs = [
                    (OC865_OBJS, oc865_hash(&format!("P{index:02}"))),
                    (OC865_OBJS + 0x100, oc865_hash(&format!("O{index:02}"))),
                ];
                oc865_screen(c, &objs, 0);
                // The name list: `n` ids, `list_index = offset + index`.
                let n = [0u32, 2, 5, 20][((seed / 24) % 4) as usize];
                let offset = [0u32, 1, 3][((seed / 96) % 3) as usize];
                oc865_w(c, OC865_SELF + 0x6e8, OC865_NAMES);
                oc865_w(c, OC865_SELF + 0x6ec, OC865_NAMES + 4 * n);
                oc865_w(c, OC865_SELF + 0x6f4, offset);
                for i in 0..n {
                    oc865_w(c, OC865_NAMES + 4 * i, r.next_u32());
                }
                oc865_force_ids(c, 0x0026_24f8);
                oc865_force(c, 0x001a_4670, [0u32, 1, 2, 7][((seed / 288) % 4) as usize]);
            },
        );
    }

    /// `RefreshPlayerList` rebuilds the 15 roster rows (scroll offset, stale
    /// names removed, empty rows), resizes the "playerMenu" menu when it
    /// exists and highlights the selected row. `ShowBuddyInfo` is the real
    /// generated body with no text objects on the screen, so the branch where
    /// it returns non-zero is not reached.
    #[test]
    fn cfe_state_net_messenger_refresh_player_list_matches() {
        check(
            ee::s03::f_001a6978__generated,
            cfe_state_net_messenger_refresh_player_list,
            240,
            &|c, seed| {
                let mut r = oc865_base(c, seed, 0x001a_6978, 6, true, seed % 9 != 0);
                oc865_str(c, 0x0046_15a0, "P%02d");
                oc865_str(c, 0x0046_1830, "O%02d");
                oc865_str(c, 0x0046_1590, "playerMenu");
                oc865_colour_constants(c, &mut r);
                let mut objs = Vec::new();
                for row in 0..15u32 {
                    objs.push((
                        OC865_OBJS + 0x100 * (2 * row),
                        oc865_hash(&format!("P{row:02}")),
                    ));
                    objs.push((
                        OC865_OBJS + 0x100 * (2 * row + 1),
                        oc865_hash(&format!("O{row:02}")),
                    ));
                }
                if seed % 2 == 1 {
                    objs.push((OC865_MENU, oc865_hash("playerMenu")));
                    c.bus.write8(
                        OC865_MENU + 0x96,
                        [0u8, 4, 20, 31][((seed / 2) % 4) as usize],
                    );
                    c.bus
                        .write8(OC865_MENU + 0x95, [0u8, 1, 7][((seed / 8) % 3) as usize]);
                    c.bus
                        .write8(OC865_MENU + 0x98, [0u8, 1, 3][((seed / 24) % 3) as usize]);
                }
                oc865_screen(c, &objs, 0);
                let n = [0u32, 3, 14, 15, 16, 17, 20, 31][((seed / 3) % 8) as usize];
                oc865_w(c, OC865_SELF + 0x6e8, OC865_NAMES);
                oc865_w(c, OC865_SELF + 0x6ec, OC865_NAMES + 4 * n);
                oc865_w(c, OC865_SELF + 0x6f4, (seed / 5) % 24);
                for i in 0..n {
                    let id = r.next_u32();
                    // About one name in four is stale (id with the top bit set).
                    oc865_w(
                        c,
                        OC865_NAMES + 4 * i,
                        if id % 4 == 0 {
                            id | 0x8000_0000
                        } else {
                            id & 0x7fff_ffff
                        },
                    );
                }
                oc865_force_ids(c, 0x0026_24f8);
            },
        );
    }

    /// `UpdateNextIcons` shows the "next challenge / message" hints: the
    /// first-pending lookup (`FindNextPending`, real, driven by the session's
    /// challenger list and the faked message and name lookups) sets the
    /// selected index, and the hint and icon objects follow it.
    #[test]
    fn cfe_state_net_messenger_update_next_icons_matches() {
        check(
            ee::s03::f_001a6ca0__generated,
            cfe_state_net_messenger_update_next_icons,
            384,
            &|c, seed| {
                let mut r = oc865_base(c, seed, 0x001a_6ca0, 6, seed % 11 != 10, seed % 17 != 16);
                oc865_str(c, 0x004a_1a20, "ps2L1");
                oc865_str(c, 0x0046_1570, "TextEqGoToNext");
                oc865_str(c, 0x0046_1840, "IconNextChallenge");
                oc865_str(c, 0x0046_1858, "IconNextMessage");
                let present = seed % 16;
                let mut objs = vec![(OC865_OBJS, 0x1357_9bdfu32)];
                for (bit, name) in [
                    "ps2L1",
                    "TextEqGoToNext",
                    "IconNextChallenge",
                    "IconNextMessage",
                ]
                .into_iter()
                .enumerate()
                {
                    if present & (1 << bit) != 0 {
                        objs.push((OC865_OBJS + 0x100 * (bit as u32 + 1), oc865_hash(name)));
                    }
                }
                oc865_screen(c, &objs, r.next_u32() & 1);
                // Session: one challenger (a shared string) or none.
                let challengers = (seed / 16) % 2;
                oc865_w(c, OC865_SESS + 0xbc, OC865_CH_ARR);
                oc865_w(c, OC865_SESS + 0xc0, OC865_CH_ARR + 4 * challengers);
                oc865_w(c, OC865_CH_ARR, OC865_STR_OBJ);
                oc865_w(c, OC865_STR_OBJ, OC865_STR_DATA);
                oc865_w(c, OC865_STR_DATA - 0xc, 5);
                oc865_force(c, 0x0026_2938, [0u32, 2][((seed / 32) % 2) as usize]);
                oc865_force(c, 0x0026_3630, OC865_STR_OBJ);
                oc865_force(
                    c,
                    0x001a_5800,
                    [0xffff_ffffu32, 0, 3][((seed / 64) % 3) as usize],
                );
                oc865_force(c, 0x0026_1460, ((seed / 192) % 2) as u32);
                oc865_w(c, OC865_SELF + 0x6f8, r.next_u32());
            },
        );
    }
}
