//! Staging module oc920 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc920`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001b_6798,
        net_config_auth_popup_update,
        "NetConfigAuthPopup_Update",
    ),
    (
        0x001b_9d68,
        fe_state_create_step1_validate_account_name,
        "cFEStateCreateStep1_ValidateAccountName",
    ),
    (
        0x001b_a618,
        fe_state_create_step2_on_widget_action,
        "cFEStateCreateStep2_OnWidgetAction",
    ),
    (
        0x001b_ce38,
        fe_state_lobby_menu_on_widget_created,
        "cFEStateLobbyMenu_OnWidgetCreated",
    ),
    (
        0x001b_e0a8,
        fe_state_lobby_menu_refresh_room_lists,
        "cFEStateLobbyMenu_RefreshRoomLists",
    ),
];

/// `NetConfigAuthPopup_Update(self)` (0x001b6798): per-frame update of the DNAS
/// authentication popup of the network setup screen. A small state machine on
/// `self+0x6e4`: 1 shows the cancel/X/submit widgets and the "authenticating"
/// text once the DNAS manager is ready, 2 waits for it to finish, 3 copies the
/// three error strings into the parent state (or reports success) and 4 posts
/// the popup event 20 when the manager is done. Always ends by updating the
/// manager and the base network state.
pub fn net_config_auth_popup_update(c: &mut Cpu) {
    if c.pc != 0x001b6798 {
        return ee::s03::f_001b6798__generated(c);
    }
    guest(c, |c| {
        'b_1b6a58: {
            'b_1b6a2c: {
                'b_1b6900: {
                    'b_1b68e4: {
                        c.cycles += 13;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.g[S0][0] = 0x2;
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[S1][0] = c.g[A0][0];
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6e4));
                        let t = c.g[V1][0] == c.g[S0][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                        if t {
                            break 'b_1b68e4;
                        } else {
                            'b_1b6800: {
                                'b_1b67e4: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_1b67e4;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[V1][0] == c.g[V0][0] {
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                            break 'b_1b6800;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;

                                            if t {
                                                break 'b_1b6a58;
                                            } else {
                                                break 'b_1b67e4;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0x3;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = 0x4;
                                if t {
                                    break 'b_1b6900;
                                } else {
                                    c.cycles += 2;
                                    if c.g[V1][0] == c.g[V0][0] {
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                        break 'b_1b6a2c;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;

                                        if t {
                                            break 'b_1b6a58;
                                        } else {
                                            break 'b_1b6800;
                                        }
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                            c.g[V0][0] = c.g[V0][0] ^ 0x1;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = 0x1;
                            if t {
                                break 'b_1b6a58;
                            } else {
                                'b_1b68d0: {
                                    c.cycles += 42;
                                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d8));
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001b682c);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001b682c)?;
                                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6dc));
                                    c.g[A1][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001b6848);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001b6848)?;
                                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6e0));
                                    c.g[A1][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001b6864);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001b6864)?;
                                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d4));
                                    c.g[A1][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001b6880);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001b6880)?;
                                    c.g[A0][0] = 0x460000;
                                    c.g[RA][0] = sx(0x001b688c);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4208));
                                    call_ptr(c, 0x00317670, 0x001b688c)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d4));
                                    c.g[RA][0] = sx(0x001b6898);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a1000, 0x001b6898)?;
                                    c.g[A0][0] = 0x460000;
                                    c.g[RA][0] = sx(0x001b68a4);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4250));
                                    call_ptr(c, 0x00317670, 0x001b68a4)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001b68b0);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001b68b0)?;
                                    c.g[A2][0] = c.g[V0][0];
                                    let t = c.g[A2][0] == 0u64;
                                    c.g[A1][0] = 0x1;
                                    if t {
                                        break 'b_1b68d0;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001b68d0);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001b68d0)?;
                                        break 'b_1b68d0;
                                    }
                                }
                                c.cycles += 5;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x6e4), c.g[S0][0]);
                                c.g[RA][0] = sx(0x001b68dc);
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                call_ptr(c, 0x002562d0, 0x001b68dc)?;
                                let t = true;

                                if t {
                                    break 'b_1b6a58;
                                } else {
                                    break 'b_1b68e4;
                                }
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = c.g[V0][0] ^ 0x5;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x3;
                    if t {
                        break 'b_1b6a58;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x6e4), c.g[V0][0]);
                        if t {
                            break 'b_1b6a58;
                        } else {
                            break 'b_1b6900;
                        }
                    }
                }
                'b_1b6a10: {
                    'b_1b6a04: {
                        c.cycles += 4;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                        let t = c.g[V0][0] == 0u64;
                        c.g[S5][0] = sx((c.g[A0][0] as u32).wrapping_add(0x850));
                        if t {
                            break 'b_1b6a04;
                        } else {
                            'b_1b6934: {
                                c.cycles += 6;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x450));
                                c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_add(0x650));
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x710));
                                let t = c.g[A0][0] == 0u64;

                                if t {
                                    break 'b_1b6934;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x001b6930);
                                    call_ptr(c, 0x00317e98, 0x001b6930)?;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x710), 0u64);
                                    break 'b_1b6934;
                                }
                            }
                            'b_1b6974: {
                                c.cycles += 13;
                                c.g[RA][0] = sx(0x001b693c);
                                c.g[A0][0] = c.g[S3][0];
                                call_ptr(c, 0x003dbca0, 0x001b693c)?;
                                c.g[V1][0] = 0x460000;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4220));
                                c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
                                c.g[A1][0] = c.g[S2][0];
                                c.g[A2][0] = 0x100;
                                c.g[RA][0] = sx(0x001b695c);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317e30, 0x001b695c)?;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x714));
                                let t = c.g[A0][0] == 0u64;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x710), c.g[V0][0]);
                                if t {
                                    break 'b_1b6974;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x001b6970);
                                    call_ptr(c, 0x00317e98, 0x001b6970)?;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x714), 0u64);
                                    break 'b_1b6974;
                                }
                            }
                            'b_1b69ac: {
                                c.cycles += 11;
                                c.g[RA][0] = sx(0x001b697c);
                                c.g[A0][0] = c.g[S4][0];
                                call_ptr(c, 0x003dbca0, 0x001b697c)?;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.g[A1][0] = c.g[S2][0];
                                c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
                                c.g[A2][0] = 0x100;
                                c.g[RA][0] = sx(0x001b6994);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317e30, 0x001b6994)?;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x718));
                                let t = c.g[A0][0] == 0u64;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x714), c.g[V0][0]);
                                if t {
                                    break 'b_1b69ac;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x001b69a8);
                                    call_ptr(c, 0x00317e98, 0x001b69a8)?;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x718), 0u64);
                                    break 'b_1b69ac;
                                }
                            }
                            c.cycles += 22;
                            c.g[RA][0] = sx(0x001b69b4);
                            c.g[A0][0] = c.g[S5][0];
                            call_ptr(c, 0x003dbca0, 0x001b69b4)?;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            c.g[A2][0] = 0x100;
                            c.g[A3][0] = 0u64;
                            c.g[A1][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001b69cc);
                            c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
                            call_ptr(c, 0x00317e30, 0x001b69cc)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x710));
                            c.g[A1][0] = c.g[S3][0];
                            c.g[RA][0] = sx(0x001b69dc);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x718), c.g[V0][0]);
                            call_ptr(c, 0x003dbe20, 0x001b69dc)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x714));
                            c.g[RA][0] = sx(0x001b69e8);
                            c.g[A1][0] = c.g[S4][0];
                            call_ptr(c, 0x003dbe20, 0x001b69e8)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x718));
                            c.g[RA][0] = sx(0x001b69f4);
                            c.g[A1][0] = c.g[S5][0];
                            call_ptr(c, 0x003dbe20, 0x001b69f4)?;
                            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                            c.g[A1][0] = c.g[S1][0];
                            let t = true;
                            c.g[A2][0] = 0x10;
                            if t {
                                break 'b_1b6a10;
                            } else {
                                break 'b_1b6a04;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = 0xf;
                    break 'b_1b6a10;
                }
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x88));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001b6a24);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001b6a24)?;
                let t = true;

                if t {
                    break 'b_1b6a58;
                } else {
                    break 'b_1b6a2c;
                }
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
            c.g[V0][0] = c.g[V0][0] ^ 0x5;
            let t = c.g[V0][0] != 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_1b6a58;
            } else {
                c.cycles += 7;
                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[A2][0] = 0x14;
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x88));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001b6a58);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001b6a58)?;
                break 'b_1b6a58;
            }
        }
        c.cycles += 13;
        c.g[RA][0] = sx(0x001b6a60);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
        call_ptr(c, 0x002563f8, 0x001b6a60)?;
        c.g[RA][0] = sx(0x001b6a68);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x001a8770,
            f: ee::s03::NetFEState_Update,
        }
        .call(c, 0x001b6a68)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateCreateStep1_ValidateAccountName(self)` (0x001b9d68): checks the new
/// account name (empty, length 4..16, first character not a digit 1..8, only
/// digits/letters/spaces) and shows the matching "account name" error popup.
/// Returns 1 when the name is fine.
pub fn fe_state_create_step1_validate_account_name(c: &mut Cpu) {
    if c.pc != 0x001b9d68 {
        return ee::s03::f_001b9d68__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x001b9d68;
        loop {
            match pc {
                0x001b9d68 => {
                    c.cycles += 18;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[A0][0] = c.g[SP][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[RA][0]);
                    c.g[RA][0] = sx(0x001b9d98);
                    c.g[S4][0] = 0x1;
                    Callee {
                        addr: 0x001c55f8,
                        f: ee::s03::cFEPopupConfirm_CfgCtor,
                    }
                    .call(c, 0x001b9d98)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
                    c.g[RA][0] = sx(0x001b9da4);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    call_ptr(c, 0x00416810, 0x001b9da4)?;
                    c.g[S3][0] = c.g[V0][0];
                    let t = c.g[S3][0] != 0u64;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x4));
                    if t {
                        pc = 0x001b9dd4;
                        continue;
                    }
                    pc = 0x001b9db0;
                    continue;
                }
                0x001b9db0 => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001b9db8);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001a9c50,
                        f: ee::s03::NetFEState_GetPopup,
                    }
                    .call(c, 0x001b9db8)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[S4][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2bd0));
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                    let t = true;
                    c.g[S4][0] = 0u64;
                    if t {
                        pc = 0x001b9e64;
                        continue;
                    }
                    pc = 0x001b9dd4;
                    continue;
                }
                0x001b9dd4 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0xdu64) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
                        pc = 0x001b9e04;
                        continue;
                    }
                    pc = 0x001b9de0;
                    continue;
                }
                0x001b9de0 => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001b9de8);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001a9c50,
                        f: ee::s03::NetFEState_GetPopup,
                    }
                    .call(c, 0x001b9de8)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[S4][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24b0));
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                    let t = true;
                    c.g[S4][0] = 0u64;
                    if t {
                        pc = 0x001b9e64;
                        continue;
                    }
                    pc = 0x001b9e04;
                    continue;
                }
                0x001b9e04 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V1][0] = c.lb((c.g[V1][0] as u32));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 49i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 57i64) as u64;
                    if t {
                        pc = 0x001b9eb0;
                        continue;
                    }
                    pc = 0x001b9e18;
                    continue;
                }
                0x001b9e18 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        pc = 0x001b9eb0;
                        continue;
                    }
                    pc = 0x001b9e20;
                    continue;
                }
                0x001b9e20 => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001b9e28);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001a9c50,
                        f: ee::s03::NetFEState_GetPopup,
                    }
                    .call(c, 0x001b9e28)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[S4][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4978));
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                    let t = true;
                    c.g[S4][0] = 0u64;
                    if t {
                        pc = 0x001b9e64;
                        continue;
                    }
                    pc = 0x001b9e44;
                    continue;
                }
                0x001b9e44 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x001b9e4c);
                    c.g[S4][0] = 0u64;
                    Callee {
                        addr: 0x001a9c50,
                        f: ee::s03::NetFEState_GetPopup,
                    }
                    .call(c, 0x001b9e4c)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    c.g[V0][0] = 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4978));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                    pc = 0x001b9e64;
                    continue;
                }
                0x001b9e64 => {
                    c.cycles += 19;
                    c.g[RA][0] = sx(0x001b9e6c);
                    call_ptr(c, 0x00317670, 0x001b9e6c)?;
                    c.g[V1][0] = 0x3;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                    c.g[RA][0] = sx(0x001b9e84);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                    call_ptr(c, 0x00317670, 0x001b9e84)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001b9e94);
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x001c57c0,
                        f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                    }
                    .call(c, 0x001b9e94)?;
                    c.g[V0][0] = 0x80d;
                    c.g[A0][0] = c.g[S2][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                    c.g[RA][0] = sx(0x001b9ea8);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001a9bc0,
                        f: ee::s03::NetFEState_ShowPopup,
                    }
                    .call(c, 0x001b9ea8)?;
                    let t = true;
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        pc = 0x001b9f38;
                        continue;
                    }
                    pc = 0x001b9eb0;
                    continue;
                }
                0x001b9eb0 => {
                    c.cycles += 2;
                    let t = (c.g[S3][0] as i64) <= 0;
                    c.g[S1][0] = 0u64;
                    if t {
                        pc = 0x001b9f34;
                        continue;
                    }
                    pc = 0x001b9eb8;
                    continue;
                }
                0x001b9eb8 => {
                    c.cycles += 2;
                    c.g[S5][0] = 0x20;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
                    pc = 0x001b9ec0;
                    continue;
                }
                0x001b9ec0 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                    c.g[S0][0] = c.lb((c.g[V0][0] as u32));
                    let t = c.g[S0][0] == c.g[S5][0];
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        pc = 0x001b9f24;
                        continue;
                    }
                    pc = 0x001b9ed4;
                    continue;
                }
                0x001b9ed4 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = 0x30;
                    c.g[RA][0] = sx(0x001b9ee4);
                    c.g[A3][0] = 0x39;
                    Callee {
                        addr: 0x001b9f68,
                        f: ee::s03::cFEStateCreateStep1_CharInRange,
                    }
                    .call(c, 0x001b9ee4)?;

                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        pc = 0x001b9f28;
                        continue;
                    }
                    pc = 0x001b9eec;
                    continue;
                }
                0x001b9eec => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = 0x61;
                    c.g[RA][0] = sx(0x001b9f00);
                    c.g[A3][0] = 0x7a;
                    Callee {
                        addr: 0x001b9f68,
                        f: ee::s03::cFEStateCreateStep1_CharInRange,
                    }
                    .call(c, 0x001b9f00)?;

                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        pc = 0x001b9f28;
                        continue;
                    }
                    pc = 0x001b9f08;
                    continue;
                }
                0x001b9f08 => {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = 0x41;
                    c.g[RA][0] = sx(0x001b9f1c);
                    c.g[A3][0] = 0x5a;
                    Callee {
                        addr: 0x001b9f68,
                        f: ee::s03::cFEStateCreateStep1_CharInRange,
                    }
                    .call(c, 0x001b9f1c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        if back_edge_unwinds(c, 0x001b9e44) {
                            return Ok(());
                        }
                        pc = 0x001b9e44;
                        continue;
                    }
                    pc = 0x001b9f24;
                    continue;
                }
                0x001b9f24 => {
                    c.cycles += 1;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    pc = 0x001b9f28;
                    continue;
                }
                0x001b9f28 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
                        if back_edge_unwinds(c, 0x001b9ec0) {
                            return Ok(());
                        }
                        pc = 0x001b9ec0;
                        continue;
                    }
                    pc = 0x001b9f34;
                    continue;
                }
                0x001b9f34 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[SP][0];
                    pc = 0x001b9f38;
                    continue;
                }
                0x001b9f38 => {
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x001b9f40);
                    c.g[A1][0] = 0x2;
                    Callee {
                        addr: 0x001c5750,
                        f: ee::s03::cFEPopupConfirm_CfgDtor,
                    }
                    .call(c, 0x001b9f40)?;
                    c.g[V0][0] = c.g[S4][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001b9d68, 0x001b9f68);
                    return Ok(());
                }
            }
        }
    });
}

/// `cFEStateCreateStep2_OnWidgetAction(self, widget, msg)` (0x001ba618): widget
/// events of the account-creation step 2. Msg 9 stores the birth month, year,
/// day, gender or legal answer from the list box that changed (refilling the
/// day list), 5 submits the form from the submit button, 6 goes back to step 1,
/// 8 opens the online agreement, anything else goes to the base network state.
pub fn fe_state_create_step2_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001ba618 {
        return ee::s03::f_001ba618__generated(c);
    }
    guest(c, |c| {
        'b_1ba8a0: {
            'b_1ba89c: {
                'b_1ba894: {
                    'b_1ba884: {
                        'b_1ba854: {
                            'b_1ba824: {
                                c.cycles += 10;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                                c.g[V0][0] = 0x6;
                                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.g[S1][0] = c.g[A0][0];
                                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.g[S2][0] = c.g[A1][0];
                                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                                let t = c.g[A2][0] == c.g[V0][0];
                                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                                if t {
                                    break 'b_1ba824;
                                } else {
                                    'b_1ba7f0: {
                                        'b_1ba65c: {
                                            c.cycles += 3;
                                            c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x5;
                                            if t {
                                                break 'b_1ba65c;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A2][0] == c.g[V0][0];
                                                c.g[A0][0] = 0x460000;
                                                if t {
                                                    break 'b_1ba7f0;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[A0][0] = c.g[S1][0];
                                                    if t {
                                                        break 'b_1ba894;
                                                    } else {
                                                        break 'b_1ba65c;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x8;
                                        let t = c.g[A2][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x9;
                                        if t {
                                            break 'b_1ba854;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[A2][0] != c.g[V0][0];
                                            c.g[A0][0] = c.g[S1][0];
                                            if t {
                                                break 'b_1ba894;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x6d4));
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = 0x4a0000;
                                                if t {
                                                    break 'b_1ba89c;
                                                } else {
                                                    'b_1ba6e0: {
                                                        c.cycles += 6;
                                                        c.g[S0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x38));
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x1bd8));
                                                        c.g[RA][0] = sx(0x001ba68c);
                                                        c.g[S3][0] = 0x4a0000;
                                                        call_ptr(c, 0x00317670, 0x001ba68c)?;
                                                        let t = c.g[S0][0] != c.g[V0][0];
                                                        c.g[A0][0] = 0x4a0000;
                                                        if t {
                                                            break 'b_1ba6e0;
                                                        } else {
                                                            c.cycles += 12;
                                                            c.g[RA][0] = sx(0x001ba69c);
                                                            c.g[A0][0] = c.g[S2][0];
                                                            call_ptr(c, 0x0039a738, 0x001ba69c)?;
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x6d0));
                                                            c.g[A0][0] = sx((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1be0));
                                                            c.g[RA][0] = sx(0x001ba6ac);
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x28),
                                                                c.g[V0][0],
                                                            );
                                                            call_ptr(c, 0x00317670, 0x001ba6ac)?;
                                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[RA][0] = sx(0x001ba6b8);
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x0039d860, 0x001ba6b8)?;
                                                            c.g[A1][0] = c.g[V0][0];
                                                            if c.g[A1][0] == 0u64 {
                                                                c.g[A0][0] = 0x4a0000;
                                                                break 'b_1ba6e0;
                                                            } else {
                                                                c.cycles += 7;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x6d0));
                                                                c.g[A0][0] = c.g[S1][0];
                                                                c.g[T0][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x2c));
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                c.g[RA][0] = sx(0x001ba6dc);
                                                                c.g[A3][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x30));
                                                                Callee { addr: 0x001bb840, f: ee::s03::cFEStateCreateStep2_FillDayList }.call(c, 0x001ba6dc)?;
                                                                c.g[A0][0] = 0x4a0000;
                                                                break 'b_1ba6e0;
                                                            }
                                                        }
                                                    }
                                                    'b_1ba774: {
                                                        c.cycles += 5;
                                                        c.g[S0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x38));
                                                        c.g[RA][0] = sx(0x001ba6ec);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x1be8));
                                                        call_ptr(c, 0x00317670, 0x001ba6ec)?;
                                                        if c.g[S0][0] != c.g[V0][0] {
                                                            c.g[S0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x38));
                                                            break 'b_1ba774;
                                                        } else {
                                                            'b_1ba770: {
                                                                c.cycles += 26;
                                                                c.g[V1][0] = c.lbu(
                                                                    (c.g[S2][0] as u32)
                                                                        .wrapping_add(0x319),
                                                                );
                                                                c.g[A1][0] = 0x4a0000;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x6d0));
                                                                c.g[A0][0] = c.g[SP][0];
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x76c));
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x12c0));
                                                                c.sw(
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                    c.g[V1][0],
                                                                );
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x6d0));
                                                                c.g[RA][0] = sx(0x001ba71c);
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x30));
                                                                call_ptr(
                                                                    c, 0x00417828, 0x001ba71c,
                                                                )?;
                                                                c.g[A1][0] = 0u64;
                                                                c.g[A2][0] = c.g[SP][0];
                                                                c.g[A3][0] = 0u64;
                                                                c.g[RA][0] = sx(0x001ba730);
                                                                c.g[A0][0] = c.g[S2][0];
                                                                call_ptr(
                                                                    c, 0x0039a7f0, 0x001ba730,
                                                                )?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x6d0));
                                                                c.g[RA][0] = sx(0x001ba73c);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                call_ptr(
                                                                    c, 0x003180b8, 0x001ba73c,
                                                                )?;
                                                                c.g[RA][0] = sx(0x001ba744);
                                                                c.g[A0][0] = sx((c.g[S3][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1be0));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001ba744,
                                                                )?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x40));
                                                                c.g[RA][0] = sx(0x001ba750);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x0039d860, 0x001ba750,
                                                                )?;
                                                                c.g[A1][0] = c.g[V0][0];
                                                                let t = c.g[A1][0] == 0u64;
                                                                c.g[A0][0] = c.g[S1][0];
                                                                if t {
                                                                    break 'b_1ba770;
                                                                } else {
                                                                    c.cycles += 5;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x6d0));
                                                                    c.g[T0][0] = c
                                                                        .lw((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x2c));
                                                                    c.g[A2][0] = c
                                                                        .lw((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x28));
                                                                    c.g[RA][0] = sx(0x001ba770);
                                                                    c.g[A3][0] = c
                                                                        .lw((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x30));
                                                                    Callee { addr: 0x001bb840, f: ee::s03::cFEStateCreateStep2_FillDayList }.call(c, 0x001ba770)?;
                                                                    break 'b_1ba770;
                                                                }
                                                            }
                                                            c.cycles += 1;
                                                            c.g[S0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x38));
                                                            break 'b_1ba774;
                                                        }
                                                    }
                                                    'b_1ba798: {
                                                        c.cycles += 4;
                                                        c.g[RA][0] = sx(0x001ba77c);
                                                        c.g[A0][0] = sx((c.g[S3][0] as u32)
                                                            .wrapping_add(0x1be0));
                                                        call_ptr(c, 0x00317670, 0x001ba77c)?;
                                                        if c.g[S0][0] != c.g[V0][0] {
                                                            c.g[A0][0] = 0x460000;
                                                            break 'b_1ba798;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.g[RA][0] = sx(0x001ba78c);
                                                            c.g[A0][0] = c.g[S2][0];
                                                            call_ptr(c, 0x0039a738, 0x001ba78c)?;
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x6d0));
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x2c),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[A0][0] = 0x460000;
                                                            break 'b_1ba798;
                                                        }
                                                    }
                                                    'b_1ba7c0: {
                                                        c.cycles += 5;
                                                        c.g[S0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x38));
                                                        c.g[RA][0] = sx(0x001ba7a4);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x4a18));
                                                        call_ptr(c, 0x00317670, 0x001ba7a4)?;
                                                        if c.g[S0][0] != c.g[V0][0] {
                                                            c.g[A0][0] = 0x460000;
                                                            break 'b_1ba7c0;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.g[RA][0] = sx(0x001ba7b4);
                                                            c.g[A0][0] = c.g[S2][0];
                                                            call_ptr(c, 0x0039a738, 0x001ba7b4)?;
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x6d0));
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x34),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[A0][0] = 0x460000;
                                                            break 'b_1ba7c0;
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[S0][0] = c
                                                        .lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                                    c.g[RA][0] = sx(0x001ba7cc);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x4a08));
                                                    call_ptr(c, 0x00317670, 0x001ba7cc)?;
                                                    let t = c.g[S0][0] != c.g[V0][0];
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x50),
                                                    );
                                                    if t {
                                                        break 'b_1ba8a0;
                                                    } else {
                                                        c.cycles += 7;
                                                        c.g[RA][0] = sx(0x001ba7dc);
                                                        c.g[A0][0] = c.g[S2][0];
                                                        call_ptr(c, 0x0039a738, 0x001ba7dc)?;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x6d0));
                                                        c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                        c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x40),
                                                            c.g[V0][0],
                                                        );
                                                        if t {
                                                            break 'b_1ba89c;
                                                        } else {
                                                            break 'b_1ba7f0;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                    c.g[RA][0] = sx(0x001ba7fc);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4a28));
                                    call_ptr(c, 0x00317670, 0x001ba7fc)?;
                                    let t = c.g[S0][0] != c.g[V0][0];
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    if t {
                                        break 'b_1ba8a0;
                                    } else {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001ba80c);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001bbaa8,
                                            f: ee::s03::cFEStateCreateStep2_ValidateFields,
                                        }
                                        .call(c, 0x001ba80c)?;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A0][0] = c.g[S1][0];
                                        if t {
                                            break 'b_1ba89c;
                                        } else {
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x001ba81c);
                                            c.g[A1][0] = 0x1;
                                            Callee {
                                                addr: 0x001ba9f8,
                                                f: ee::s03::cFEStateCreateStep2_SubmitAccount,
                                            }
                                            .call(c, 0x001ba81c)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_1ba8a0;
                                            } else {
                                                break 'b_1ba824;
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 12;
                            c.g[A1][0] = 0x460000;
                            c.g[A0][0] = 0x6f4;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2ed0));
                            c.g[A2][0] = 0u64;
                            c.g[RA][0] = sx(0x001ba83c);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x001ba83c)?;
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001ba84c);
                            c.g[A2][0] = 0x1;
                            Callee {
                                addr: 0x001b88f0,
                                f: ee::s03::cFEStateCreateStep1_ctor,
                            }
                            .call(c, 0x001ba84c)?;
                            let t = true;
                            c.g[A0][0] = c.g[S1][0];
                            if t {
                                break 'b_1ba884;
                            } else {
                                break 'b_1ba854;
                            }
                        }
                        c.cycles += 12;
                        c.g[A1][0] = 0x460000;
                        c.g[A0][0] = 0x6dc;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4860));
                        c.g[A2][0] = 0x100;
                        c.g[RA][0] = sx(0x001ba86c);
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x001ba86c)?;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = sx(0x001ba878);
                        c.g[A0][0] = c.g[V0][0];
                        Callee {
                            addr: 0x001c0418,
                            f: ee::s03::cFEStateOnlineAgreement_ctor,
                        }
                        .call(c, 0x001ba878)?;
                        c.g[V1][0] = 0x16;
                        c.g[A0][0] = c.g[S1][0];
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                        break 'b_1ba884;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001ba88c);
                    c.g[A1][0] = c.g[V0][0];
                    Callee {
                        addr: 0x001a8a40,
                        f: ee::s03::NetFEState_GotoNextState,
                    }
                    .call(c, 0x001ba88c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    if t {
                        break 'b_1ba8a0;
                    } else {
                        break 'b_1ba894;
                    }
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x001ba89c);
                c.g[A1][0] = c.g[S2][0];
                Callee {
                    addr: 0x001a8918,
                    f: ee::s03::NetFEState_OnWidgetAction,
                }
                .call(c, 0x001ba89c)?;
                break 'b_1ba89c;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_1ba8a0;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateLobbyMenu_OnWidgetCreated(self, obj)` (0x001bce38): the lobby menu
/// screen reports each widget as it is created; the widget's hash name is
/// matched against the known names (room menu, player menu, channel pairs,
/// arrows, lists, locks, `TextPlayer0..9`) and stored in the matching slot of
/// the state, with the widget's initial attributes.
pub fn fe_state_lobby_menu_on_widget_created(c: &mut Cpu) {
    if c.pc != 0x001bce38 {
        return ee::s03::f_001bce38__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x001bce38;
        loop {
            match pc {
                0x001bce38 => {
                    c.cycles += 15;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S0][0] = c.g[A1][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = 0x460000;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4db0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[RA][0] = sx(0x001bce6c);
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                    call_ptr(c, 0x00317670, 0x001bce6c)?;
                    let t = c.g[S1][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001bce7c;
                        continue;
                    }
                    pc = 0x001bce74;
                    continue;
                }
                0x001bce74 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x6d4), c.g[S0][0]);
                    if t {
                        pc = 0x001bd194;
                        continue;
                    }
                    pc = 0x001bce7c;
                    continue;
                }
                0x001bce7c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bce84);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4dc0));
                    call_ptr(c, 0x00317670, 0x001bce84)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bceec;
                        continue;
                    }
                    pc = 0x001bce8c;
                    continue;
                }
                0x001bce8c => {
                    c.cycles += 24;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x6d0), c.g[S0][0]);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001bce9c);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0039ab00, 0x001bce9c)?;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001bceb8);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001bceb8)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
                    c.g[RA][0] = sx(0x001bcec4);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0039b760, 0x001bcec4)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
                    c.g[V1][0] = 0xffffffffffffff7f;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = c.g[V0][0] | 0x1;
                    let t = true;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    if t {
                        pc = 0x001bd194;
                        continue;
                    }
                    pc = 0x001bceec;
                    continue;
                }
                0x001bceec => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bcef4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4dd0));
                    call_ptr(c, 0x00317670, 0x001bcef4)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        pc = 0x001bcf1c;
                        continue;
                    }
                    pc = 0x001bcefc;
                    continue;
                }
                0x001bcefc => {
                    c.cycles += 8;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x001bcf08);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4de8));
                    call_ptr(c, 0x00317670, 0x001bcf08)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001bcf14);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001bcf14)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001bd198;
                        continue;
                    }
                    pc = 0x001bcf1c;
                    continue;
                }
                0x001bcf1c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bcf24);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1bf0));
                    call_ptr(c, 0x00317670, 0x001bcf24)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bcf34;
                        continue;
                    }
                    pc = 0x001bcf2c;
                    continue;
                }
                0x001bcf2c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x704), c.g[S0][0]);
                    if t {
                        pc = 0x001bd194;
                        continue;
                    }
                    pc = 0x001bcf34;
                    continue;
                }
                0x001bcf34 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bcf3c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4e00));
                    call_ptr(c, 0x00317670, 0x001bcf3c)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bcf54;
                        continue;
                    }
                    pc = 0x001bcf44;
                    continue;
                }
                0x001bcf44 => {
                    c.cycles += 4;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x700), c.g[S0][0]);
                    c.g[V0][0] = 0x4;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    if t {
                        pc = 0x001bd194;
                        continue;
                    }
                    pc = 0x001bcf54;
                    continue;
                }
                0x001bcf54 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bcf5c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4e10));
                    call_ptr(c, 0x00317670, 0x001bcf5c)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bcf70;
                        continue;
                    }
                    pc = 0x001bcf64;
                    continue;
                }
                0x001bcf64 => {
                    c.cycles += 3;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x718), c.g[S0][0]);
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
                    if t {
                        pc = 0x001bd194;
                        continue;
                    }
                    pc = 0x001bcf70;
                    continue;
                }
                0x001bcf70 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bcf78);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4e20));
                    call_ptr(c, 0x00317670, 0x001bcf78)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bcf90;
                        continue;
                    }
                    pc = 0x001bcf80;
                    continue;
                }
                0x001bcf80 => {
                    c.cycles += 4;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x71c), c.g[S0][0]);
                    c.g[V0][0] = 0x1;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    if t {
                        pc = 0x001bd194;
                        continue;
                    }
                    pc = 0x001bcf90;
                    continue;
                }
                0x001bcf90 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bcf98);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4e30));
                    call_ptr(c, 0x00317670, 0x001bcf98)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bcfb0;
                        continue;
                    }
                    pc = 0x001bcfa0;
                    continue;
                }
                0x001bcfa0 => {
                    c.cycles += 4;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x720), c.g[S0][0]);
                    c.g[V0][0] = 0x2;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    if t {
                        pc = 0x001bd194;
                        continue;
                    }
                    pc = 0x001bcfb0;
                    continue;
                }
                0x001bcfb0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bcfb8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4e40));
                    call_ptr(c, 0x00317670, 0x001bcfb8)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bcfd0;
                        continue;
                    }
                    pc = 0x001bcfc0;
                    continue;
                }
                0x001bcfc0 => {
                    c.cycles += 4;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x724), c.g[S0][0]);
                    c.g[V0][0] = 0x3;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    if t {
                        pc = 0x001bd194;
                        continue;
                    }
                    pc = 0x001bcfd0;
                    continue;
                }
                0x001bcfd0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bcfd8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4e50));
                    call_ptr(c, 0x00317670, 0x001bcfd8)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bcfec;
                        continue;
                    }
                    pc = 0x001bcfe0;
                    continue;
                }
                0x001bcfe0 => {
                    c.cycles += 3;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x730), c.g[S0][0]);
                    let t = true;
                    c.g[A1][0] = 0x1;
                    if t {
                        pc = 0x001bd134;
                        continue;
                    }
                    pc = 0x001bcfec;
                    continue;
                }
                0x001bcfec => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bcff4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4e60));
                    call_ptr(c, 0x00317670, 0x001bcff4)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bd004;
                        continue;
                    }
                    pc = 0x001bcffc;
                    continue;
                }
                0x001bcffc => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x734), c.g[S0][0]);
                    if t {
                        pc = 0x001bd130;
                        continue;
                    }
                    pc = 0x001bd004;
                    continue;
                }
                0x001bd004 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bd00c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4e70));
                    call_ptr(c, 0x00317670, 0x001bd00c)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bd01c;
                        continue;
                    }
                    pc = 0x001bd014;
                    continue;
                }
                0x001bd014 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x738), c.g[S0][0]);
                    if t {
                        pc = 0x001bd130;
                        continue;
                    }
                    pc = 0x001bd01c;
                    continue;
                }
                0x001bd01c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bd024);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4e80));
                    call_ptr(c, 0x00317670, 0x001bd024)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bd034;
                        continue;
                    }
                    pc = 0x001bd02c;
                    continue;
                }
                0x001bd02c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x73c), c.g[S0][0]);
                    if t {
                        pc = 0x001bd130;
                        continue;
                    }
                    pc = 0x001bd034;
                    continue;
                }
                0x001bd034 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bd03c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4e90));
                    call_ptr(c, 0x00317670, 0x001bd03c)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bd04c;
                        continue;
                    }
                    pc = 0x001bd044;
                    continue;
                }
                0x001bd044 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x728), c.g[S0][0]);
                    if t {
                        pc = 0x001bd194;
                        continue;
                    }
                    pc = 0x001bd04c;
                    continue;
                }
                0x001bd04c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bd054);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4ea0));
                    call_ptr(c, 0x00317670, 0x001bd054)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bd064;
                        continue;
                    }
                    pc = 0x001bd05c;
                    continue;
                }
                0x001bd05c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x72c), c.g[S0][0]);
                    if t {
                        pc = 0x001bd194;
                        continue;
                    }
                    pc = 0x001bd064;
                    continue;
                }
                0x001bd064 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bd06c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4eb0));
                    call_ptr(c, 0x00317670, 0x001bd06c)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bd07c;
                        continue;
                    }
                    pc = 0x001bd074;
                    continue;
                }
                0x001bd074 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x708), c.g[S0][0]);
                    if t {
                        pc = 0x001bd0c0;
                        continue;
                    }
                    pc = 0x001bd07c;
                    continue;
                }
                0x001bd07c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bd084);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4ec8));
                    call_ptr(c, 0x00317670, 0x001bd084)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bd094;
                        continue;
                    }
                    pc = 0x001bd08c;
                    continue;
                }
                0x001bd08c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x70c), c.g[S0][0]);
                    if t {
                        pc = 0x001bd0c0;
                        continue;
                    }
                    pc = 0x001bd094;
                    continue;
                }
                0x001bd094 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bd09c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4ee0));
                    call_ptr(c, 0x00317670, 0x001bd09c)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bd0ac;
                        continue;
                    }
                    pc = 0x001bd0a4;
                    continue;
                }
                0x001bd0a4 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x710), c.g[S0][0]);
                    if t {
                        pc = 0x001bd0c0;
                        continue;
                    }
                    pc = 0x001bd0ac;
                    continue;
                }
                0x001bd0ac => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bd0b4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4ef8));
                    call_ptr(c, 0x00317670, 0x001bd0b4)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bd0d4;
                        continue;
                    }
                    pc = 0x001bd0bc;
                    continue;
                }
                0x001bd0bc => {
                    c.cycles += 1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x714), c.g[S0][0]);
                    pc = 0x001bd0c0;
                    continue;
                }
                0x001bd0c0 => {
                    c.cycles += 5;
                    c.g[V1][0] = 0xfffffffffffffffe;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    if t {
                        pc = 0x001bd194;
                        continue;
                    }
                    pc = 0x001bd0d4;
                    continue;
                }
                0x001bd0d4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bd0dc);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4f10));
                    call_ptr(c, 0x00317670, 0x001bd0dc)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bd0ec;
                        continue;
                    }
                    pc = 0x001bd0e4;
                    continue;
                }
                0x001bd0e4 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x740), c.g[S0][0]);
                    if t {
                        pc = 0x001bd130;
                        continue;
                    }
                    pc = 0x001bd0ec;
                    continue;
                }
                0x001bd0ec => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bd0f4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4f20));
                    call_ptr(c, 0x00317670, 0x001bd0f4)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bd104;
                        continue;
                    }
                    pc = 0x001bd0fc;
                    continue;
                }
                0x001bd0fc => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x744), c.g[S0][0]);
                    if t {
                        pc = 0x001bd130;
                        continue;
                    }
                    pc = 0x001bd104;
                    continue;
                }
                0x001bd104 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bd10c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4f30));
                    call_ptr(c, 0x00317670, 0x001bd10c)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001bd11c;
                        continue;
                    }
                    pc = 0x001bd114;
                    continue;
                }
                0x001bd114 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x748), c.g[S0][0]);
                    if t {
                        pc = 0x001bd130;
                        continue;
                    }
                    pc = 0x001bd11c;
                    continue;
                }
                0x001bd11c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bd124);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4f40));
                    call_ptr(c, 0x00317670, 0x001bd124)?;
                    let t = c.g[S1][0] != c.g[V0][0];
                    c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6d8));
                    if t {
                        pc = 0x001bd150;
                        continue;
                    }
                    pc = 0x001bd12c;
                    continue;
                }
                0x001bd12c => {
                    c.cycles += 1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x74c), c.g[S0][0]);
                    pc = 0x001bd130;
                    continue;
                }
                0x001bd130 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0u64;
                    pc = 0x001bd134;
                    continue;
                }
                0x001bd134 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001bd148);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001bd148)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        pc = 0x001bd198;
                        continue;
                    }
                    pc = 0x001bd150;
                    continue;
                }
                0x001bd150 => {
                    c.cycles += 3;
                    c.g[S4][0] = 0x460000;
                    let t = true;
                    c.g[S2][0] = 0u64;
                    if t {
                        pc = 0x001bd164;
                        continue;
                    }
                    pc = 0x001bd15c;
                    continue;
                }
                0x001bd15c => {
                    c.cycles += 1;
                    pc = 0x001bd160;
                    continue;
                }
                0x001bd160 => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x001bd164;
                    continue;
                }
                0x001bd164 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 10i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4f50));
                    if t {
                        pc = 0x001bd194;
                        continue;
                    }
                    pc = 0x001bd170;
                    continue;
                }
                0x001bd170 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001bd17c);
                    c.g[A2][0] = c.g[S2][0];
                    call_ptr(c, 0x00417828, 0x001bd17c)?;
                    c.g[RA][0] = sx(0x001bd184);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001bd184)?;

                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                        if back_edge_unwinds(c, 0x001bd160) {
                            return Ok(());
                        }
                        pc = 0x001bd160;
                        continue;
                    }
                    pc = 0x001bd18c;
                    continue;
                }
                0x001bd18c => {
                    c.cycles += 2;
                    c.sw((c.g[S3][0] as u32), c.g[S0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S2][0]);
                    pc = 0x001bd194;
                    continue;
                }
                0x001bd194 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    pc = 0x001bd198;
                    continue;
                }
                0x001bd198 => {
                    c.cycles += 7;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001bce38, 0x001bd1b4);
                    return Ok(());
                }
            }
        }
    });
}

/// `cFEStateLobbyMenu_RefreshRoomLists(self)` (0x001be0a8): clears the four
/// channel list boxes and refills them from the lobby manager's rooms (room
/// names `<channel>.<name>`; the channel prefix picks the list, the first room of
/// a channel sets its lock icon), then adds "no available rooms" to the empty
/// lists and clamps the selections.
pub fn fe_state_lobby_menu_refresh_room_lists(c: &mut Cpu) {
    if c.pc != 0x001be0a8 {
        return ee::s03::f_001be0a8__generated(c);
    }
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S4][0] = c.g[A0][0];
        c.g[S7][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x708));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[A0][0] = c.g[S7][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S0][0] = c.g[V0][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S1][0] = 0x3;
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
        c.g[RA][0] = sx(0x001be0f4);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x50), c.g[V0][0]);
        call_ptr(c, 0x0025f120, 0x001be0f4)?;
        c.g[S6][0] = c.g[V0][0];
        c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x54), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
        'l_1be120: loop {
            c.cycles += 5;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            c.g[RA][0] = sx(0x001be12c);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            call_ptr(c, 0x0039a670, 0x001be12c)?;
            if (c.g[S1][0] as i64) >= 0 {
                c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                if event_due(c) {
                    deliver_event_here(c, 0x001be120);
                }
                continue 'l_1be120;
            } else {
                'b_1be2e4: {
                    c.cycles += 2;
                    let t = (c.g[S6][0] as i64) <= 0;
                    c.g[S3][0] = 0u64;
                    if t {
                        break 'b_1be2e4;
                    } else {
                        c.cycles += 3;
                        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[A0][0] = c.g[S7][0];
                        'l_1be148: loop {
                            'b_1be2d8: {
                                'b_1be26c: {
                                    'b_1be1f8: {
                                        c.cycles += 38;
                                        c.g[RA][0] = sx(0x001be150);
                                        c.g[A1][0] = c.g[S3][0];
                                        call_ptr(c, 0x0025f150, 0x001be150)?;
                                        c.g[A0][0] = c.g[SP][0];
                                        c.g[RA][0] = sx(0x001be15c);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x003184e8, 0x001be15c)?;
                                        c.g[A0][0] = c.g[SP][0];
                                        c.g[A1][0] = 0x2e;
                                        c.g[RA][0] = sx(0x001be16c);
                                        c.g[A2][0] = 0u64;
                                        call_ptr(c, 0x00318c18, 0x001be16c)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.g[SP][0];
                                        c.g[A2][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001be180);
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x003190a8, 0x001be180)?;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x54));
                                        c.g[RA][0] = sx(0x001be18c);
                                        c.g[A1][0] = c.g[S2][0];
                                        call_ptr(c, 0x00318448, 0x001be18c)?;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[RA][0] = sx(0x001be198);
                                        c.g[A1][0] = 0x2;
                                        call_ptr(c, 0x00318260, 0x001be198)?;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                                        c.g[A1][0] = c.g[SP][0];
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x8));
                                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                        c.g[RA][0] = sx(0x001be1b4);
                                        c.g[A2][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_sub((c.g[S0][0] as u32)));
                                        call_ptr(c, 0x00319028, 0x001be1b4)?;
                                        c.g[A0][0] = c.g[FP][0];
                                        c.g[RA][0] = sx(0x001be1c0);
                                        c.g[A1][0] = c.g[S2][0];
                                        call_ptr(c, 0x00318448, 0x001be1c0)?;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[RA][0] = sx(0x001be1cc);
                                        c.g[A1][0] = 0x2;
                                        call_ptr(c, 0x00318260, 0x001be1cc)?;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.g[RA][0] = sx(0x001be1d8);
                                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1684));
                                        call_ptr(c, 0x004165a8, 0x001be1d8)?;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                        if t {
                                            break 'b_1be1f8;
                                        } else {
                                            c.cycles += 6;
                                            c.g[V0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                            c.g[V1][0] = 0u64;
                                            c.g[S1][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x708));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                            let t = true;
                                            c.sw(
                                                (c.g[SP][0] as u32).wrapping_add(0x30),
                                                c.g[V0][0],
                                            );
                                            if t {
                                                break 'b_1be26c;
                                            } else {
                                                break 'b_1be1f8;
                                            }
                                        }
                                    }
                                    'b_1be220: {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001be200);
                                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1688));
                                        call_ptr(c, 0x004165a8, 0x001be200)?;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                        if t {
                                            break 'b_1be220;
                                        } else {
                                            c.cycles += 6;
                                            c.g[V0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                            c.g[V1][0] = 0x1;
                                            c.g[S1][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x70c));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                            let t = true;
                                            c.sw(
                                                (c.g[SP][0] as u32).wrapping_add(0x34),
                                                c.g[V0][0],
                                            );
                                            if t {
                                                break 'b_1be26c;
                                            } else {
                                                break 'b_1be220;
                                            }
                                        }
                                    }
                                    'b_1be248: {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001be228);
                                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x168c));
                                        call_ptr(c, 0x004165a8, 0x001be228)?;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                        if t {
                                            break 'b_1be248;
                                        } else {
                                            c.cycles += 6;
                                            c.g[V0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                            c.g[V1][0] = 0x2;
                                            c.g[S1][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x710));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                            let t = true;
                                            c.sw(
                                                (c.g[SP][0] as u32).wrapping_add(0x38),
                                                c.g[V0][0],
                                            );
                                            if t {
                                                break 'b_1be26c;
                                            } else {
                                                break 'b_1be248;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001be250);
                                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1690));
                                    call_ptr(c, 0x004165a8, 0x001be250)?;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                        break 'b_1be2d8;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                        c.g[V1][0] = 0x3;
                                        c.g[S1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x714));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                                        break 'b_1be26c;
                                    }
                                }
                                'b_1be2b0: {
                                    c.cycles += 5;
                                    c.g[S0][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[S5][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                    let t = c.g[V1][0] != 0u64;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_1be2b0;
                                    } else {
                                        c.cycles += 12;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                        c.g[RA][0] = sx(0x001be28c);
                                        c.g[A1][0] = c.g[S3][0];
                                        call_ptr(c, 0x0025f180, 0x001be28c)?;
                                        c.g[V1][0] =
                                            sx((c.g[S4][0] as u32)
                                                .wrapping_add((c.g[S0][0] as u32)));
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x740));
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001be2ac);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001be2ac)?;
                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                        break 'b_1be2b0;
                                    }
                                }
                                c.cycles += 10;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x001be2bc);
                                c.g[A2][0] = c.g[S3][0];
                                call_ptr(c, 0x0039a3f8, 0x001be2bc)?;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0u64;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001be2d4);
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001be2d4)?;
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                break 'b_1be2d8;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S6][0] as i64)) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.g[A0][0] = c.g[S7][0];
                                if event_due(c) {
                                    deliver_event_here(c, 0x001be148);
                                }
                                continue 'l_1be148;
                            } else {
                                break 'b_1be2e4;
                            }
                        }
                    }
                }
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S1][0] = c.g[S5][0];
                c.g[S3][0] = 0x460000;
                c.g[S2][0] = 0x3;
                'l_1be2f8: loop {
                    'b_1be340: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                        if c.g[V0][0] != 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            break 'b_1be340;
                        } else {
                            c.cycles += 15;
                            c.g[RA][0] = sx(0x001be30c);
                            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x51b0));
                            call_ptr(c, 0x00317670, 0x001be30c)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                            c.g[A2][0] = 0xffffffffffff0000;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001be320);
                            c.g[A2][0] = c.g[A2][0] | 0xffff;
                            call_ptr(c, 0x0039a568, 0x001be320)?;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                            c.g[A1][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001be33c);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001be33c)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            break 'b_1be340;
                        }
                    }
                    c.cycles += 3;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[S2][0] as i64) >= 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001be2f8);
                        }
                        continue 'l_1be2f8;
                    } else {
                        c.cycles += 23;
                        c.g[RA][0] = sx(0x001be354);
                        c.g[A0][0] = c.g[S4][0];
                        Callee {
                            addr: 0x001be038,
                            f: ee::s03::cFEStateLobbyMenu_ClampRoomSelections,
                        }
                        .call(c, 0x001be354)?;
                        c.g[A0][0] = c.g[FP][0];
                        c.g[RA][0] = sx(0x001be360);
                        c.g[A1][0] = 0x2;
                        call_ptr(c, 0x00318260, 0x001be360)?;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x54));
                        c.g[RA][0] = sx(0x001be36c);
                        c.g[A1][0] = 0x2;
                        call_ptr(c, 0x00318260, 0x001be36c)?;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x001be378);
                        c.g[A1][0] = 0x2;
                        call_ptr(c, 0x00318260, 0x001be378)?;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{fakes, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Fixture areas, clear of the fake log (DATA+0x100000), the pointers the
    /// fakes return (DATA..DATA+0x40000), the allocator and the BELibrary dummy.
    pub(crate) const OC920_BASE: u32 = DATA + 0x28_0000;
    /// Control block: magic word, the `StrHash` match index, its call counter,
    /// the per-address forced results and the per-call forced results.
    pub(crate) const OC920_CTRL: u32 = OC920_BASE;
    pub(crate) const OC920_MAGIC: u32 = 0x4f43_3932;
    const OC920_KTH: u32 = OC920_CTRL + 4;
    const OC920_HASH_N: u32 = OC920_CTRL + 8;
    const OC920_BY_ADDR: u32 = OC920_CTRL + 0x100;
    const OC920_RES: u32 = OC920_CTRL + 0x1000;
    /// The hash `StrHash` returns for the call picked by [`OC920_KTH`].
    const OC920_HASH: u32 = 0x1234_5678;
    const OC920_SELF: u32 = OC920_BASE + 0x4000;
    /// Account / DNAS manager / widget the state works on.
    const OC920_OBJ: u32 = OC920_BASE + 0x6000;
    const OC920_PARENT: u32 = OC920_BASE + 0x8000;
    /// Vtables (0x400 apart) shared by the widgets.
    const OC920_VT: u32 = OC920_BASE + 0xa000;
    /// Widgets, 0x100 apart.
    const OC920_WIDGET: u32 = OC920_BASE + 0xb000;
    const OC920_STR: u32 = OC920_BASE + 0xd000;
    const OC920_ZERO: u32 = OC920_BASE + 0xe000;

    /// The fake callee: log the call like `fakes::fake_call`, then apply the
    /// forced result (by address, else by call index; `word - 1`, 0 = keep the
    /// fake's own pointer) unless the call unwound.
    pub(crate) fn oc920_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake_call(c, addr);
        if c.pc == ra {
            let mut word = c.bus.read32(OC920_RES + 4 * (n & 0x3ff));
            for i in 0..8u32 {
                if c.bus.read32(OC920_BY_ADDR + 8 * i) == addr {
                    let w = c.bus.read32(OC920_BY_ADDR + 8 * i + 4);
                    if w != 0 {
                        word = w;
                    }
                }
            }
            if word != 0 {
                c.g[V0][0] = word.wrapping_sub(1) as u64;
            }
        }
    }

    fn oc920_miss(c: &mut Cpu, target: u32) {
        oc920_call(c, target);
    }

    /// Gated fakes of the callees in their override slots.
    macro_rules! oc920_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC920_CTRL) != OC920_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc920_call(c, $addr);
                }
            )*
            fn oc920_install_plain() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc920_fakes!(
        (
            oc920_fake_1a8770,
            OC920_PREV_1A8770,
            ee::s03::OVERRIDE_001A8770,
            ee::s03::f_001a8770__generated,
            0x001a_8770
        ),
        (
            oc920_fake_317e30,
            OC920_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc920_fake_317e98,
            OC920_PREV_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc920_fake_39d860,
            OC920_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc920_fake_3a1000,
            OC920_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc920_fake_1a9bc0,
            OC920_PREV_1A9BC0,
            ee::s03::OVERRIDE_001A9BC0,
            ee::s03::f_001a9bc0__generated,
            0x001a_9bc0
        ),
        (
            oc920_fake_1a9c50,
            OC920_PREV_1A9C50,
            ee::s03::OVERRIDE_001A9C50,
            ee::s03::f_001a9c50__generated,
            0x001a_9c50
        ),
        (
            oc920_fake_1c55f8,
            OC920_PREV_1C55F8,
            ee::s03::OVERRIDE_001C55F8,
            ee::s03::f_001c55f8__generated,
            0x001c_55f8
        ),
        (
            oc920_fake_1c5750,
            OC920_PREV_1C5750,
            ee::s03::OVERRIDE_001C5750,
            ee::s03::f_001c5750__generated,
            0x001c_5750
        ),
        (
            oc920_fake_1c57c0,
            OC920_PREV_1C57C0,
            ee::s03::OVERRIDE_001C57C0,
            ee::s03::f_001c57c0__generated,
            0x001c_57c0
        ),
        (
            oc920_fake_1a8918,
            OC920_PREV_1A8918,
            ee::s03::OVERRIDE_001A8918,
            ee::s03::f_001a8918__generated,
            0x001a_8918
        ),
        (
            oc920_fake_1a8a40,
            OC920_PREV_1A8A40,
            ee::s03::OVERRIDE_001A8A40,
            ee::s03::f_001a8a40__generated,
            0x001a_8a40
        ),
        (
            oc920_fake_1b88f0,
            OC920_PREV_1B88F0,
            ee::s03::OVERRIDE_001B88F0,
            ee::s03::f_001b88f0__generated,
            0x001b_88f0
        ),
        (
            oc920_fake_1ba9f8,
            OC920_PREV_1BA9F8,
            ee::s03::OVERRIDE_001BA9F8,
            ee::s03::f_001ba9f8__generated,
            0x001b_a9f8
        ),
        (
            oc920_fake_1bb840,
            OC920_PREV_1BB840,
            ee::s03::OVERRIDE_001BB840,
            ee::s03::f_001bb840__generated,
            0x001b_b840
        ),
        (
            oc920_fake_1bbaa8,
            OC920_PREV_1BBAA8,
            ee::s03::OVERRIDE_001BBAA8,
            ee::s03::f_001bbaa8__generated,
            0x001b_baa8
        ),
        (
            oc920_fake_1c0418,
            OC920_PREV_1C0418,
            ee::s03::OVERRIDE_001C0418,
            ee::s03::f_001c0418__generated,
            0x001c_0418
        ),
        (
            oc920_fake_317d70,
            OC920_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc920_fake_3180b8,
            OC920_PREV_3180B8,
            ee::s10::OVERRIDE_003180B8,
            ee::s10::f_003180b8__generated,
            0x0031_80b8
        ),
        (
            oc920_fake_39a738,
            OC920_PREV_39A738,
            ee::s13::OVERRIDE_0039A738,
            ee::s13::f_0039a738__generated,
            0x0039_a738
        ),
        (
            oc920_fake_39a7f0,
            OC920_PREV_39A7F0,
            ee::s13::OVERRIDE_0039A7F0,
            ee::s13::f_0039a7f0__generated,
            0x0039_a7f0
        ),
        (
            oc920_fake_417828,
            OC920_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc920_fake_39ab00,
            OC920_PREV_39AB00,
            ee::s13::OVERRIDE_0039AB00,
            ee::s13::f_0039ab00__generated,
            0x0039_ab00
        ),
        (
            oc920_fake_39b760,
            OC920_PREV_39B760,
            ee::s13::OVERRIDE_0039B760,
            ee::s13::f_0039b760__generated,
            0x0039_b760
        ),
        (
            oc920_fake_1be038,
            OC920_PREV_1BE038,
            ee::s03::OVERRIDE_001BE038,
            ee::s03::f_001be038__generated,
            0x001b_e038
        ),
        (
            oc920_fake_25f120,
            OC920_PREV_25F120,
            ee::s06::OVERRIDE_0025F120,
            ee::s06::f_0025f120__generated,
            0x0025_f120
        ),
        (
            oc920_fake_25f150,
            OC920_PREV_25F150,
            ee::s06::OVERRIDE_0025F150,
            ee::s06::f_0025f150__generated,
            0x0025_f150
        ),
        (
            oc920_fake_25f180,
            OC920_PREV_25F180,
            ee::s06::OVERRIDE_0025F180,
            ee::s06::f_0025f180__generated,
            0x0025_f180
        ),
        (
            oc920_fake_318260,
            OC920_PREV_318260,
            ee::s10::OVERRIDE_00318260,
            ee::s10::f_00318260__generated,
            0x0031_8260
        ),
        (
            oc920_fake_318448,
            OC920_PREV_318448,
            ee::s10::OVERRIDE_00318448,
            ee::s10::f_00318448__generated,
            0x0031_8448
        ),
        (
            oc920_fake_3184e8,
            OC920_PREV_3184E8,
            ee::s10::OVERRIDE_003184E8,
            ee::s10::f_003184e8__generated,
            0x0031_84e8
        ),
        (
            oc920_fake_318c18,
            OC920_PREV_318C18,
            ee::s10::OVERRIDE_00318C18,
            ee::s10::f_00318c18__generated,
            0x0031_8c18
        ),
        (
            oc920_fake_319028,
            OC920_PREV_319028,
            ee::s10::OVERRIDE_00319028,
            ee::s10::f_00319028__generated,
            0x0031_9028
        ),
        (
            oc920_fake_3190a8,
            OC920_PREV_3190A8,
            ee::s10::OVERRIDE_003190A8,
            ee::s10::f_003190a8__generated,
            0x0031_90a8
        ),
        (
            oc920_fake_39a3f8,
            OC920_PREV_39A3F8,
            ee::s13::OVERRIDE_0039A3F8,
            ee::s13::f_0039a3f8__generated,
            0x0039_a3f8
        ),
        (
            oc920_fake_39a568,
            OC920_PREV_39A568,
            ee::s13::OVERRIDE_0039A568,
            ee::s13::f_0039a568__generated,
            0x0039_a568
        ),
        (
            oc920_fake_39a670,
            OC920_PREV_39A670,
            ee::s13::OVERRIDE_0039A670,
            ee::s13::f_0039a670__generated,
            0x0039_a670
        ),
        (
            oc920_fake_4165a8,
            OC920_PREV_4165A8,
            ee::s15::OVERRIDE_004165A8,
            ee::s15::f_004165a8__generated,
            0x0041_65a8
        ),
    );

    static OC920_PREV_317670: AtomicUsize = AtomicUsize::new(0);

    /// `StrHash`: the call whose index equals [`OC920_KTH`] returns
    /// [`OC920_HASH`], every other call a different value, so a widget whose
    /// hash name is `OC920_HASH` matches exactly the K-th compared name.
    fn oc920_fake_317670(c: &mut Cpu) {
        if c.bus.read32(OC920_CTRL) != OC920_MAGIC {
            return fakes::fall_back(c, &OC920_PREV_317670, ee::s10::f_00317670__generated);
        }
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(OC920_HASH_N);
        c.bus.write32(OC920_HASH_N, n + 1);
        let k = c.bus.read32(OC920_KTH);
        oc920_call(c, 0x0031_7670);
        if c.pc == ra {
            let v = if n == k {
                OC920_HASH
            } else {
                OC920_HASH.wrapping_add(1 + n)
            };
            c.g[V0][0] = v as u64;
        }
    }

    fn oc920_install() {
        use std::ptr::addr_of_mut;
        oc920_install_plain();
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc920_fake_317670,
            &OC920_PREV_317670,
        );
    }

    /// Scrambled machine with the runtime, the fakes armed, no `StrHash`
    /// match (`kth` = none) and `pc` at the function's entry.
    fn oc920_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc920_install();
        let r = fakes::machine_rt(c, seed, salt, calls);
        c.hooks.miss = oc920_miss;
        c.bus.write32(OC920_CTRL, OC920_MAGIC);
        c.bus.write32(OC920_KTH, u32::MAX);
        c.pc = entry;
        r
    }

    /// Force the result of every call to `addr` (`word - 1`; 1 gives 0).
    fn oc920_force(c: &mut Cpu, slot: u32, addr: u32, word: u32) {
        c.bus.write32(OC920_BY_ADDR + 8 * slot, addr);
        c.bus.write32(OC920_BY_ADDR + 8 * slot + 4, word);
    }

    /// `obj+8 = vt`, and for each slot offset a random `this` adjustment and a
    /// fake method address (`vt+off`: delta, `vt+off+4`: method).
    fn oc920_object(c: &mut Cpu, r: &mut Rng, obj: u32, vt: u32, slots: &[u32]) {
        c.bus.write32(obj + 8, vt);
        for (i, &off) in slots.iter().enumerate() {
            c.bus.write16(vt + off, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * i as u32);
        }
    }

    /// `NetConfigAuthPopup_Update`: every state 0..5 of the popup, with the
    /// DNAS manager in states 1/3/5/9, with and without an authentication
    /// error (and the parent's old error strings to free), a missing DNAS
    /// group widget, and fakes that unwind.
    #[test]
    fn net_config_auth_popup_update_matches() {
        fakes::covered(
            ee::s03::f_001b6798__generated,
            net_config_auth_popup_update,
            240,
            &|c, seed| {
                let mut r = oc920_machine(c, seed, 0x920_6798, 8, 0x001b_6798);
                let state = seed % 6;
                let variant = seed / 6;
                let auth = OC920_OBJ;
                c.bus.write32(OC920_SELF + 0x6e4, state);
                c.bus.write32(OC920_SELF + 0x6d0, auth);
                for (k, off) in [0x6d4u32, 0x6d8, 0x6dc, 0x6e0].into_iter().enumerate() {
                    let w = OC920_WIDGET + 0x100 * k as u32;
                    c.bus.write32(OC920_SELF + off, w);
                    oc920_object(c, &mut r, w, OC920_VT, &[0x48]);
                }
                c.bus.write32(OC920_SELF + 0x40, OC920_ZERO);
                c.bus.write32(OC920_SELF + 0x20, OC920_PARENT);
                oc920_object(c, &mut r, OC920_PARENT, OC920_VT + 0x400, &[0x88]);
                c.bus.write32(auth, [3u32, 5, 9, 1][(variant % 4) as usize]);
                c.bus.write32(auth + 0x1c, ((variant / 4) % 2) * 0x77);
                for (k, off) in [0x710u32, 0x714, 0x718].into_iter().enumerate() {
                    let old = if (variant / 8) % 2 == 0 {
                        0
                    } else {
                        OC920_ZERO + 0x100 * (k as u32 + 1)
                    };
                    c.bus.write32(OC920_PARENT + off, old);
                }
                if variant % 3 == 0 {
                    oc920_force(c, 0, 0x0039_d860, 1);
                }
                c.g[A0][0] = sx(OC920_SELF);
            },
        );
    }

    /// `cFEStateCreateStep1_ValidateAccountName`: empty, too short, too long,
    /// a first character 1..8, a first '9'/'0', a bad character, spaces and
    /// valid names (the real `strlen` and `CharInRange`).
    #[test]
    fn fe_state_create_step1_validate_account_name_matches() {
        const NAMES: [&[u8]; 10] = [
            b"",
            b"abc",
            b"abcdefghijklmnopqrstuvwxyz",
            b"5abcd",
            b"9abcd",
            b"0abcd",
            b"abc d123",
            b"ab!cd",
            b"abcd",
            b"Zz09 AZaz",
        ];
        fakes::covered(
            ee::s03::f_001b9d68__generated,
            fe_state_create_step1_validate_account_name,
            200,
            &|c, seed| {
                let _ = oc920_machine(c, seed, 0x920_9d68, 7, 0x001b_9d68);
                let name = NAMES[seed as usize % NAMES.len()];
                c.bus.write32(OC920_SELF + 0x6d0, OC920_OBJ);
                c.bus.write32(OC920_OBJ, OC920_STR);
                for (i, &b) in name.iter().enumerate() {
                    c.bus.write8(OC920_STR + i as u32, b);
                }
                c.g[A0][0] = sx(OC920_SELF);
            },
        );
    }

    /// `cFEStateCreateStep2_OnWidgetAction`: messages 9 (with the screen ready
    /// or not, the list box whose name matches picked through `StrHash`, the
    /// day list box missing or not), 5 (submit, validation passing or not), 6,
    /// 8 and the default.
    #[test]
    fn fe_state_create_step2_on_widget_action_matches() {
        fakes::covered(
            ee::s03::f_001ba618__generated,
            fe_state_create_step2_on_widget_action,
            300,
            &|c, seed| {
                let _ = oc920_machine(c, seed, 0x920_a618, 10, 0x001b_a618);
                let msg = [9u32, 5, 6, 8, 7, 12][(seed % 6) as usize];
                let variant = seed / 6;
                c.bus.write32(OC920_SELF + 0x6d0, OC920_OBJ);
                c.bus.write32(OC920_SELF + 0x6d4, variant & 1);
                c.bus.write32(OC920_SELF + 0x40, OC920_ZERO);
                c.bus.write32(OC920_SELF + 0x10, OC920_ZERO + 0x100);
                c.bus.write32(OC920_WIDGET + 0x38, OC920_HASH);
                let kth = match msg {
                    9 => (variant / 8) % 10,
                    5 => {
                        if (variant / 2) % 2 == 0 {
                            0
                        } else {
                            u32::MAX
                        }
                    }
                    _ => u32::MAX,
                };
                c.bus.write32(OC920_KTH, kth);
                if variant & 2 != 0 {
                    oc920_force(c, 0, 0x001b_baa8, 1);
                }
                if variant & 4 != 0 {
                    oc920_force(c, 1, 0x0039_d860, 1);
                }
                c.g[A0][0] = sx(OC920_SELF);
                c.g[A1][0] = sx(OC920_WIDGET);
                c.g[A2][0] = msg as u64;
            },
        );
    }

    /// `cFEStateLobbyMenu_OnWidgetCreated`: each of the 24 named widgets and
    /// each `TextPlayerN` (the widget's name matches the K-th hashed name),
    /// and a widget with no known name.
    #[test]
    fn fe_state_lobby_menu_on_widget_created_matches() {
        fakes::covered(
            ee::s03::f_001bce38__generated,
            fe_state_lobby_menu_on_widget_created,
            216,
            &|c, seed| {
                let mut r = oc920_machine(c, seed, 0x920_ce38, 9, 0x001b_ce38);
                let obj = OC920_WIDGET;
                c.bus.write32(obj + 0x38, OC920_HASH);
                c.bus.write32(obj + 0x14, r.next_u32());
                c.bus.write32(obj + 0x18, r.next_u32());
                oc920_object(c, &mut r, obj, OC920_VT, &[0x38, 0x48]);
                c.bus.write32(OC920_KTH, seed % 36);
                c.g[A0][0] = sx(OC920_SELF);
                c.g[A1][0] = sx(obj);
            },
        );
    }

    /// `cFEStateLobbyMenu_RefreshRoomLists`: 0..4 rooms whose names (the
    /// `strcmp` results are random) fall into any of the four channels or
    /// none, empty channels getting the "no rooms" entry.
    #[test]
    fn fe_state_lobby_menu_refresh_room_lists_matches() {
        fakes::covered(
            ee::s03::f_001be0a8__generated,
            fe_state_lobby_menu_refresh_room_lists,
            200,
            &|c, seed| {
                let mut r = oc920_machine(c, seed, 0x920_e0a8, 30, 0x001b_e0a8);
                c.bus
                    .write32(crate::difftest::GAME_GP.wrapping_sub(0xc8), OC920_ZERO);
                c.bus.write32(
                    crate::difftest::GAME_GP.wrapping_add(0xda0),
                    OC920_ZERO + 0x108,
                );
                for k in 0..4u32 {
                    let list = OC920_WIDGET + 0x100 * k;
                    let lock = OC920_WIDGET + 0x400 + 0x100 * k;
                    c.bus.write32(OC920_SELF + 0x708 + 4 * k, list);
                    c.bus.write32(OC920_SELF + 0x740 + 4 * k, lock);
                    oc920_object(c, &mut r, list, OC920_VT, &[0x40]);
                    oc920_object(c, &mut r, lock, OC920_VT + 0x400, &[0x48]);
                }
                for i in 0..0x40u32 {
                    let w = [0u32, 1, 2, 2, 1, 3][(r.next_u32() % 6) as usize];
                    c.bus.write32(OC920_RES + 4 * i, w);
                }
                oc920_force(c, 0, 0x0025_f120, 1 + (seed % 5));
                c.g[A0][0] = sx(OC920_SELF);
            },
        );
    }
}
