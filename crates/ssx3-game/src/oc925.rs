//! Staging module oc925 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc925`.

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
        0x001b_60a8,
        cfe_state_net_config_on_widget_action,
        "cFEStateNetConfig_OnWidgetAction",
    ),
    (
        0x001b_63e8,
        net_config_auth_popup_init,
        "NetConfigAuthPopup_Init",
    ),
    (
        0x001b_65c0,
        net_config_auth_popup_on_widget_created,
        "NetConfigAuthPopup_OnWidgetCreated",
    ),
    (
        0x001b_6c98,
        cfe_state_auth_dnas_error_transition_in,
        "cFEStateAuthDNASError_TransitionIn",
    ),
    (
        0x001b_6ed0,
        cfe_state_net_account_mem_ctor,
        "cFEStateNetAccountMem_ctor",
    ),
    (
        0x001b_9b18,
        cfe_state_create_step1_validate_email,
        "cFEStateCreateStep1_ValidateEmail",
    ),
    (
        0x001b_9fa0,
        cfe_state_create_step1_validate_password,
        "cFEStateCreateStep1_ValidatePassword",
    ),
    (
        0x001b_d8c8,
        cfe_state_lobby_menu_on_join_room_result,
        "cFEStateLobbyMenu_OnJoinRoomResult",
    ),
];

/// `void cFEStateNetConfig_OnWidgetAction(self, pWidget, msg)` (0x1b60a8, the
/// C++ `cFEStateNetConfig_onWidgetEvent`): the net-config screen's widget
/// event. Widget misc data 0 and 1 (and 2) drive the card and account
/// popups; a message this state does not handle goes to the base handler
/// `NetFEState_OnWidgetAction`. `self` fields: `+0x10` engine, `+0x40`
/// screen, `+0x6d0` byte flag, `+0x6e4` and `+0x6ec` child objects.
#[allow(unused_labels, unreachable_code)]
pub fn cfe_state_net_config_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b6270: {
            'b_1b626c: {
                'b_1b6268: {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[S1][0] == 0u64;
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    if t {
                        break 'b_1b6268;
                    } else {
                        'b_1b6260: {
                            'b_1b6220: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x6;
                                let t = c.g[A2][0] == c.g[V0][0];
                                c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                                if t {
                                    break 'b_1b6220;
                                } else {
                                    'b_1b620c: {
                                        'b_1b61dc: {
                                            'b_1b610c: {
                                                'b_1b60f0: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x5;
                                                    if t {
                                                        break 'b_1b60f0;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[A2][0] == c.g[V0][0];
                                                        c.g[A0][0] = c.g[S2][0];
                                                        if t {
                                                            break 'b_1b610c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;

                                                            if t {
                                                                break 'b_1b6260;
                                                            } else {
                                                                break 'b_1b60f0;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x8;
                                                let t = c.g[A2][0] == c.g[V0][0];
                                                c.g[V0][0] = 0x9;
                                                if t {
                                                    break 'b_1b620c;
                                                } else {
                                                    c.cycles += 2;
                                                    if c.g[A2][0] == c.g[V0][0] {
                                                        c.g[A0][0] = 0x0046_0000;
                                                        break 'b_1b61dc;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[A0][0] = c.g[S2][0];
                                                        if t {
                                                            break 'b_1b6260;
                                                        } else {
                                                            break 'b_1b610c;
                                                        }
                                                    }
                                                }
                                            }
                                            'b_1b6154: {
                                                'b_1b6128: {
                                                    c.cycles += 3;
                                                    c.g[S3][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                                    let t = c.g[S3][0] == 0u64;
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_1b6128;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[S3][0] == c.g[V0][0];
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x40),
                                                        );
                                                        if t {
                                                            break 'b_1b6154;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.lq(
                                                                17,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x30),
                                                            );
                                                            if t {
                                                                break 'b_1b6270;
                                                            } else {
                                                                break 'b_1b6128;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 11;
                                                c.g[A2][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x6ec));
                                                c.g[A1][0] = 0x1;
                                                c.g[V0][0] =
                                                    c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                                let t = c.g[V1][0] as u32;
                                                c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001b_6144)?;
                                                c.g[A0][0] = c.g[S2][0];
                                                call_ptr(c, 0x001b_4198, 0x001b_614c)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                if t {
                                                    break 'b_1b626c;
                                                } else {
                                                    break 'b_1b6154;
                                                }
                                            }
                                            c.cycles += 34;
                                            c.g[A0][0] = c.g[S2][0];
                                            call_ptr(c, 0x001a_9c50, 0x001b_615c)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[A0][0] = 0x0046_0000;
                                            c.g[V0][0] = 0x2;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0xbc),
                                                c.g[V0][0],
                                            );
                                            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                            call_ptr(c, 0x0031_7670, 0x001b_6178)?;
                                            c.g[A2][0] = c.g[V0][0];
                                            c.g[A1][0] = 0u64;
                                            c.g[A0][0] = c.g[S1][0];
                                            call_ptr(c, 0x001c_57c0, 0x001b_6188)?;
                                            c.g[A0][0] = 0x0046_0000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                            call_ptr(c, 0x0031_7670, 0x001b_6194)?;
                                            c.g[A1][0] = 0x1;
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[A2][0] = c.g[V0][0];
                                            call_ptr(c, 0x001c_57c0, 0x001b_61a4)?;
                                            c.g[A0][0] = 0x0046_0000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x41f0));
                                            call_ptr(c, 0x0031_7670, 0x001b_61b0)?;
                                            c.g[A1][0] = 0x844;
                                            c.g[V1][0] = 0x3;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x150),
                                                c.g[A1][0],
                                            );
                                            c.g[A0][0] = c.g[S2][0];
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x168),
                                                c.g[S3][0],
                                            );
                                            c.g[A1][0] = c.g[S0][0];
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0xc4),
                                                c.g[V0][0],
                                            );
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0xc0),
                                                c.g[V1][0],
                                            );
                                            call_ptr(c, 0x001a_9bc0, 0x001b_61d4)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                            if t {
                                                break 'b_1b626c;
                                            } else {
                                                break 'b_1b61dc;
                                            }
                                        }
                                        c.cycles += 5;
                                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3f88));
                                        call_ptr(c, 0x0031_7670, 0x001b_61e8)?;
                                        let t = c.g[S0][0] != c.g[V0][0];
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        if t {
                                            break 'b_1b626c;
                                        } else {
                                            c.cycles += 7;
                                            c.g[V0][0] =
                                                c.lbu((c.g[S1][0] as u32).wrapping_add(0x319));
                                            c.g[A0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x6e4));
                                            c.sw(
                                                (c.g[S2][0] as u32).wrapping_add(0x6d0),
                                                c.g[V0][0],
                                            );
                                            c.g[A1][0] =
                                                c.lbu((c.g[S2][0] as u32).wrapping_add(0x6d0));
                                            call_ptr(c, 0x0039_a7a8, 0x001b_6204)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                            if t {
                                                break 'b_1b626c;
                                            } else {
                                                break 'b_1b620c;
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A1][0] = 0u64;
                                    call_ptr(c, 0x001b_5268, 0x001b_6218)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                    if t {
                                        break 'b_1b626c;
                                    } else {
                                        break 'b_1b6220;
                                    }
                                }
                            }
                            c.cycles += 16;
                            c.g[A0][0] = 0u64;
                            call_ptr(c, 0x001a_86e8, 0x001b_6228)?;
                            c.g[A1][0] = 0x0046_0000;
                            c.g[A3][0] = 0u64;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xba8));
                            c.g[A0][0] = 0x58;
                            c.g[A2][0] = 0u64;
                            call_ptr(c, 0x0031_7d70, 0x001b_6240)?;
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = c.g[V0][0];
                            call_ptr(c, 0x0019_4a60, 0x001b_624c)?;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x001a_8a40, 0x001b_6258)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            if t {
                                break 'b_1b626c;
                            } else {
                                break 'b_1b6260;
                            }
                        }
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x001a_8918, 0x001b_6268)?;
                        break 'b_1b6268;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_1b626c;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1b6270;
        }
        c.cycles += 5;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void NetConfigAuthPopup_Init(self)` (0x1b63e8, the C++
/// `cFEStateAuthDNAS_onCreateScreen`): creates the "authenticating" screen,
/// plays its frame and hides the DNAS group, allocates the 2640-byte DNAS
/// manager and starts DNAS authentication. The tail copies the 0x280-byte
/// state block of the net config into the new screen's buffer.
#[allow(unused_labels, unreachable_code)]
pub fn net_config_auth_popup_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b6478: {
            'b_1b6474: {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2c0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2a0));
                c.g[S1][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x290), c.g[RA][0]);
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2b0));
                c.g[A0][0] = 0x0046_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4238));
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                call_ptr(c, 0x0031_7670, 0x001b_640c)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x0039_7cc0, 0x001b_6420)?;
                let t = c.g[V0][0] == 0u64;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                if t {
                    break 'b_1b6474;
                } else {
                    c.cycles += 13;
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0039_c870, 0x001b_6438)?;
                    c.g[A0][0] = 0x0046_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4250));
                    call_ptr(c, 0x0031_7670, 0x001b_6444)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x001b_6450)?;
                    c.g[A2][0] = c.g[V0][0];
                    if c.g[A2][0] == 0u64 {
                        c.g[A1][0] = 0x0046_0000;
                        break 'b_1b6478;
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001b_6474)?;
                        break 'b_1b6474;
                    }
                }
            }
            c.cycles += 1;
            c.g[A1][0] = 0x0046_0000;
            break 'b_1b6478;
        }
        'b_1b6550: {
            'b_1b6520: {
                c.cycles += 20;
                c.g[A2][0] = 0x100;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4260));
                c.g[A3][0] = 0u64;
                c.g[A0][0] = 0xa50;
                call_ptr(c, 0x0031_7d70, 0x001b_648c)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0025_61b8, 0x001b_6494)?;
                c.g[V1][0] = 0x1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6d0), c.g[V0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6e4), c.g[V1][0]);
                call_ptr(c, 0x0014_dc80, 0x001b_64a4)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x4;
                call_ptr(c, 0x0014_dd58, 0x001b_64b0)?;
                c.g[V1][0] = 0x0053_0000;
                c.g[A1][0] = 0u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                c.g[V0][0] = c.g[V1][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[SP][0];
                if t {
                    break 'b_1b6520;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                    'l_1b64cc: loop {
                        c.cycles += 19;
                        c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(6, (c.g[V1][0] as u32));
                        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                        c.sdr(6, (c.g[A0][0] as u32));
                        c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0xf));
                        c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x8));
                        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x17));
                        c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x10));
                        c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x1f));
                        c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001b_64cc);
                            }
                            continue 'l_1b64cc;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_1b6550;
                            } else {
                                break 'b_1b6520;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
            'l_1b6524: loop {
                c.cycles += 11;
                c.g[A2][0] = c.ld((c.g[V1][0] as u32));
                c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd((c.g[A0][0] as u32), c.g[A2][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x001b_6524);
                    }
                    continue 'l_1b6524;
                } else {
                    break 'b_1b6550;
                }
            }
        }
        'b_1b65a4: {
            c.cycles += 10;
            c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(2, (c.g[V1][0] as u32));
            c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
            c.sdr(2, (c.g[A0][0] as u32));
            c.g[V0][0] = c.lw((c.g[SP][0] as u32));
            c.g[V1][0] = 0x1;
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 22);
            c.g[A0][0] = c.g[V0][0] & 0x7;
            if c.g[A0][0] == c.g[V1][0] {
                c.g[A1][0] = 0x1;
                break 'b_1b65a4;
            } else {
                'b_1b65a0: {
                    'b_1b6598: {
                        c.cycles += 2;
                        let t = c.g[A0][0] == 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_1b6598;
                        } else {
                            c.cycles += 2;
                            let t = c.g[A0][0] == c.g[V0][0];
                            c.g[V0][0] = 0x3;
                            if t {
                                break 'b_1b65a0;
                            } else {
                                c.cycles += 2;
                                if c.g[A0][0] == c.g[V0][0] {
                                    c.g[A1][0] = 0x3;
                                    break 'b_1b65a4;
                                } else {
                                    c.cycles += 2;
                                    let t = true;

                                    if t {
                                        break 'b_1b65a4;
                                    } else {
                                        break 'b_1b6598;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_1b65a4;
                    } else {
                        break 'b_1b65a0;
                    }
                }
                c.cycles += 1;
                c.g[A1][0] = 0x2;
                break 'b_1b65a4;
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
        call_ptr(c, 0x0025_6258, 0x001b_65ac)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2b0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2a0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x290));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2c0));
        c.pc = t;
        return Ok(());
    });
}

/// `void NetConfigAuthPopup_OnWidgetCreated(self, text)` (0x1b65c0): the
/// auth popup's widget callback. Records the text widget by its hash name
/// (cancel, auth, X button, submit rect) and sets the loading caption on the
/// auth text; the last two names set up `+0x6d4`/`+0x6dc`/`+0x6e0`.
#[allow(unused_labels, unreachable_code)]
pub fn net_config_auth_popup_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b66a0: {
            'b_1b666c: {
                'b_1b664c: {
                    'b_1b65fc: {
                        c.cycles += 13;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[S2][0] = c.g[A0][0];
                        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                        c.g[S1][0] = c.g[A1][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[A0][0] = 0x0046_0000;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x31e8));
                        call_ptr(c, 0x0031_7670, 0x001b_65ec)?;
                        if c.g[S0][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x0046_0000;
                            break 'b_1b65fc;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x6d8), c.g[S1][0]);
                            if t {
                                break 'b_1b664c;
                            } else {
                                break 'b_1b65fc;
                            }
                        }
                    }
                    'b_1b6634: {
                        c.cycles += 5;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4278));
                        call_ptr(c, 0x0031_7670, 0x001b_6608)?;
                        if c.g[S0][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x004a_0000;
                            break 'b_1b6634;
                        } else {
                            c.cycles += 9;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x6d4), c.g[S1][0]);
                            c.g[A0][0] = 0x0046_0000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4288));
                            call_ptr(c, 0x0031_7670, 0x001b_6620)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d4));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a_1000, 0x001b_662c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_1b66a0;
                            } else {
                                break 'b_1b6634;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1ba0));
                    call_ptr(c, 0x0031_7670, 0x001b_6640)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x0046_0000;
                        break 'b_1b666c;
                    } else {
                        c.cycles += 1;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x6dc), c.g[S1][0]);
                        break 'b_1b664c;
                    }
                }
                c.cycles += 8;
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001b_6664)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_1b66a0;
                } else {
                    break 'b_1b666c;
                }
            }
            c.cycles += 5;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x42a0));
            call_ptr(c, 0x0031_7670, 0x001b_6678)?;
            let t = c.g[S0][0] != c.g[V0][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            if t {
                break 'b_1b66a0;
            } else {
                c.cycles += 8;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x6e0), c.g[S1][0]);
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001b_669c)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_1b66a0;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateAuthDNASError_TransitionIn(self, on)` (0x1b6c98): shows or
/// hides the DNAS error popup's footer, body and title texts depending on
/// whether each string is empty, then refreshes the error button label.
#[allow(unused_labels, unreachable_code)]
pub fn cfe_state_auth_dnas_error_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b6dfc: {
            c.cycles += 5;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            let t = c.g[A1][0] == 0u64;
            c.g[S0][0] = c.g[A0][0];
            if t {
                break 'b_1b6dfc;
            } else {
                'b_1b6d50: {
                    'b_1b6d14: {
                        c.cycles += 4;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d4));
                        call_ptr(c, 0x002c_24d0, 0x001b_6cb4)?;
                        if c.g[V0][0] != 0u64 {
                            c.g[A0][0] = 0x004a_0000;
                            break 'b_1b6d14;
                        } else {
                            'b_1b6cf0: {
                                c.cycles += 10;
                                c.g[A0][0] = 0x004a_0000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1bc0));
                                call_ptr(c, 0x0031_7670, 0x001b_6cc8)?;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039_c778, 0x001b_6cd4)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V0][0] = 0xffff;
                                let t = c.g[A1][0] == c.g[V0][0];
                                c.g[A1][0] = c.g[A1][0] & 0xffff;
                                if t {
                                    break 'b_1b6cf0;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                    c.g[A2][0] = 0x1;
                                    call_ptr(c, 0x0039_c870, 0x001b_6cf0)?;
                                    break 'b_1b6cf0;
                                }
                            }
                            c.cycles += 9;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6dc));
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001b_6d0c)?;
                            let t = true;

                            if t {
                                break 'b_1b6d50;
                            } else {
                                break 'b_1b6d14;
                            }
                        }
                    }
                    'b_1b6d44: {
                        c.cycles += 9;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1bc8));
                        call_ptr(c, 0x0031_7670, 0x001b_6d1c)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_c778, 0x001b_6d28)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        let t = c.g[A1][0] == c.g[V0][0];
                        c.g[A1][0] = c.g[A1][0] & 0xffff;
                        if t {
                            break 'b_1b6d44;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x0039_c870, 0x001b_6d44)?;
                            break 'b_1b6d44;
                        }
                    }
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6dc));
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d4));
                    call_ptr(c, 0x003a_0d00, 0x001b_6d50)?;
                    break 'b_1b6d50;
                }
                'b_1b6d8c: {
                    'b_1b6d84: {
                        c.cycles += 4;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d0));
                        call_ptr(c, 0x002c_24d0, 0x001b_6d58)?;
                        if c.g[V0][0] != 0u64 {
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6e0));
                            break 'b_1b6d84;
                        } else {
                            c.cycles += 9;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6e0));
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001b_6d7c)?;
                            let t = true;

                            if t {
                                break 'b_1b6d8c;
                            } else {
                                break 'b_1b6d84;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d0));
                    call_ptr(c, 0x003a_0d00, 0x001b_6d8c)?;
                    break 'b_1b6d8c;
                }
                'b_1b6dcc: {
                    'b_1b6dc0: {
                        c.cycles += 4;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d8));
                        call_ptr(c, 0x002c_24d0, 0x001b_6d94)?;
                        if c.g[V0][0] != 0u64 {
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6e4));
                            break 'b_1b6dc0;
                        } else {
                            c.cycles += 9;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6e4));
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001b_6db8)?;
                            let t = true;
                            c.g[A0][0] = 0x0046_0000;
                            if t {
                                break 'b_1b6dcc;
                            } else {
                                break 'b_1b6dc0;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d8));
                    call_ptr(c, 0x003a_0d00, 0x001b_6dc8)?;
                    c.g[A0][0] = 0x0046_0000;
                    break 'b_1b6dcc;
                }
                c.cycles += 9;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x42d0));
                call_ptr(c, 0x0031_7670, 0x001b_6dd4)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_c778, 0x001b_6de0)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = 0xffff;
                let t = c.g[A1][0] == c.g[V0][0];
                c.g[A1][0] = c.g[A1][0] & 0xffff;
                if t {
                    break 'b_1b6dfc;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[A2][0] = 0x1;
                    call_ptr(c, 0x0039_c870, 0x001b_6dfc)?;
                    break 'b_1b6dfc;
                }
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = 0x1;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateNetAccountMem_ctor(self, mode, name, name2)` (0x1b6ed0): the
/// account-memory net state constructor. Runs the blank popup base
/// constructor, clears the user name, password and error fields, copies the
/// two names into the account buffers and picks the start state from `mode`.
/// `self` fields: `+0x6d8`, `+0x6e0`, `+0xb04`, `+0xb08`, `+0xb0c`.
#[allow(unused_labels, unreachable_code)]
pub fn cfe_state_net_account_mem_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b6f40: {
            c.cycles += 25;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A3][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = c.g[T0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S3][0] = c.g[T1][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6e0));
            call_ptr(c, 0x001a_bf70, 0x001b_6f08)?;
            c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6f1));
            c.g[V1][0] = 0x0047_0000;
            c.sb((c.g[S0][0] as u32).wrapping_add(0x6e0), 0u64);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7830));
            c.sb((c.g[S0][0] as u32).wrapping_add(0x6f1), 0u64);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x6d4), 0u64);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x6d8), 0u64);
            c.sw((c.g[S0][0] as u32).wrapping_add(0xb04), 0u64);
            let t = c.g[S2][0] == 0u64;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xb08), 0u64);
            if t {
                break 'b_1b6f40;
            } else {
                c.cycles += 3;
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S4][0];
                call_ptr(c, 0x0041_66f4, 0x001b_6f40)?;
                break 'b_1b6f40;
            }
        }
        'b_1b6f50: {
            c.cycles += 2;
            let t = c.g[S3][0] == 0u64;
            c.g[A1][0] = c.g[S3][0];
            if t {
                break 'b_1b6f50;
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x0041_66f4, 0x001b_6f50)?;
                break 'b_1b6f50;
            }
        }
        'b_1b6ffc: {
            'b_1b6ff8: {
                'b_1b6fc4: {
                    'b_1b6fb0: {
                        'b_1b6f9c: {
                            c.cycles += 3;
                            c.g[V0][0] = 0x1;
                            let t = c.g[S1][0] == c.g[V0][0];
                            c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_1b6f9c;
                            } else {
                                'b_1b6f8c: {
                                    'b_1b6f74: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x2;
                                        if t {
                                            break 'b_1b6f74;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[S1][0] == 0u64;
                                            c.g[A0][0] = c.g[S0][0];
                                            if t {
                                                break 'b_1b6f8c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;

                                                if t {
                                                    break 'b_1b6ffc;
                                                } else {
                                                    break 'b_1b6f74;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[S1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_1b6fb0;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x0053_0000;
                                        if t {
                                            break 'b_1b6fc4;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;

                                            if t {
                                                break 'b_1b6ffc;
                                            } else {
                                                break 'b_1b6f8c;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x001b_70d0, 0x001b_6f94)?;
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xb0c), 0u64);
                                if t {
                                    break 'b_1b6ffc;
                                } else {
                                    break 'b_1b6f9c;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x5;
                        call_ptr(c, 0x001b_70d0, 0x001b_6fa8)?;
                        let t = true;
                        c.g[V0][0] = 0x5;
                        if t {
                            break 'b_1b6ff8;
                        } else {
                            break 'b_1b6fb0;
                        }
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0xb;
                    call_ptr(c, 0x001b_70d0, 0x001b_6fbc)?;
                    let t = true;
                    c.g[V0][0] = 0xb;
                    if t {
                        break 'b_1b6ff8;
                    } else {
                        break 'b_1b6fc4;
                    }
                }
                'b_1b6fd8: {
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6760));
                    let t = c.g[A1][0] == 0u64;
                    c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x11));
                    if t {
                        break 'b_1b6fd8;
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S4][0];
                        call_ptr(c, 0x0041_66f4, 0x001b_6fd8)?;
                        break 'b_1b6fd8;
                    }
                }
                'b_1b6fe8: {
                    c.cycles += 2;
                    let t = c.g[S1][0] == 0u64;
                    c.g[A0][0] = c.g[S5][0];
                    if t {
                        break 'b_1b6fe8;
                    } else {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x0041_66f4, 0x001b_6fe8)?;
                        break 'b_1b6fe8;
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x13;
                call_ptr(c, 0x001b_70d0, 0x001b_6ff4)?;
                c.g[V0][0] = 0x13;
                break 'b_1b6ff8;
            }
            c.cycles += 1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xb0c), c.g[V0][0]);
            break 'b_1b6ffc;
        }
        c.cycles += 12;
        call_ptr(c, 0x0026_5880, 0x001b_7004)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
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

/// `bool cFEStateCreateStep1_ValidateEmail(self)` (0x1b9b18): the account
/// creation step 1 email check. An empty address opens the "email missing"
/// popup; otherwise the format check decides whether the popup says
/// "email invalid". Returns 1 when the address passed.
#[allow(unused_labels, unreachable_code)]
pub fn cfe_state_create_step1_validate_email(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b9bc0: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
            c.g[S2][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.g[A0][0] = c.g[SP][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[RA][0]);
            c.g[S4][0] = 0x1;
            call_ptr(c, 0x001c_55f8, 0x001b_9b44)?;
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x0041_6810, 0x001b_9b50)?;
            c.g[S3][0] = c.g[V0][0];
            let t = c.g[S3][0] != 0u64;
            c.g[V0][0] = ((c.g[S3][0] as i64) < 7i64) as u64;
            if t {
                break 'b_1b9bc0;
            } else {
                c.cycles += 25;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x001a_9c50, 0x001b_9b64)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = 0x0046_0000;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[S4][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2bf0));
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                c.g[S4][0] = 0u64;
                call_ptr(c, 0x0031_7670, 0x001b_9b80)?;
                c.g[V1][0] = 0x3;
                c.g[A0][0] = 0x0046_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                call_ptr(c, 0x0031_7670, 0x001b_9b98)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x001c_57c0, 0x001b_9ba8)?;
                c.g[V0][0] = 0x80d;
                c.g[A1][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x001a_9bc0, 0x001b_9bbc)?;
                c.g[V0][0] = ((c.g[S3][0] as i64) < 7i64) as u64;
                break 'b_1b9bc0;
            }
        }
        'b_1b9cac: {
            'b_1b9c30: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[S2][0];
                if t {
                    break 'b_1b9c30;
                } else {
                    c.cycles += 26;
                    c.g[S4][0] = 0u64;
                    call_ptr(c, 0x001a_9c50, 0x001b_9bd0)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = 0x0046_0000;
                    c.g[V0][0] = 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24d0));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                    call_ptr(c, 0x0031_7670, 0x001b_9bec)?;
                    c.g[V1][0] = 0x3;
                    c.g[A0][0] = 0x0046_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                    call_ptr(c, 0x0031_7670, 0x001b_9c04)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x001c_57c0, 0x001b_9c14)?;
                    c.g[V0][0] = 0x80d;
                    c.g[A0][0] = c.g[S2][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x001a_9bc0, 0x001b_9c28)?;
                    let t = true;
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        break 'b_1b9cac;
                    } else {
                        break 'b_1b9c30;
                    }
                }
            }
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x001b_9cd8, 0x001b_9c3c)?;
            let t = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.g[SP][0];
            if t {
                break 'b_1b9cac;
            } else {
                c.cycles += 26;
                c.g[A0][0] = c.g[S2][0];
                c.g[S4][0] = 0u64;
                call_ptr(c, 0x001a_9c50, 0x001b_9c50)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = 0x0046_0000;
                c.g[V0][0] = 0x1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4960));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                call_ptr(c, 0x0031_7670, 0x001b_9c6c)?;
                c.g[V1][0] = 0x3;
                c.g[A0][0] = 0x0046_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                call_ptr(c, 0x0031_7670, 0x001b_9c84)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x001c_57c0, 0x001b_9c94)?;
                c.g[V0][0] = 0x80d;
                c.g[A0][0] = c.g[S2][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x001a_9bc0, 0x001b_9ca8)?;
                c.g[A0][0] = c.g[SP][0];
                break 'b_1b9cac;
            }
        }
        c.cycles += 11;
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x001c_5750, 0x001b_9cb4)?;
        c.g[V0][0] = c.g[S4][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
        c.pc = t;
        return Ok(());
    });
}

/// `bool cFEStateCreateStep1_ValidatePassword(self)` (0x1b9fa0): the account
/// creation step 1 password check. An empty password opens the "password
/// missing" popup; a password that differs from its confirmation (`strcmp`)
/// opens the "passwords differ" popup; returns 1 when both checks pass.
#[allow(unused_labels, unreachable_code)]
pub fn cfe_state_create_step1_validate_password(c: &mut Cpu) {
    guest(c, |c| {
        'b_1ba0e4: {
            'b_1ba06c: {
                'b_1ba01c: {
                    'b_1b9ff8: {
                        c.cycles += 15;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x120));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.g[S2][0] = c.g[A0][0];
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.g[A0][0] = c.g[SP][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[RA][0]);
                        c.g[S3][0] = 0x1;
                        call_ptr(c, 0x001c_55f8, 0x001b_9fc8)?;
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                        call_ptr(c, 0x0041_6810, 0x001b_9fd4)?;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                        if t {
                            break 'b_1b9ff8;
                        } else {
                            c.cycles += 7;
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x001a_9c50, 0x001b_9fe4)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = 0x0046_0000;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[S3][0]);
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2bb0));
                            if t {
                                break 'b_1ba01c;
                            } else {
                                break 'b_1b9ff8;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0xdu64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
                        break 'b_1ba06c;
                    } else {
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x001a_9c50, 0x001b_a00c)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A0][0] = 0x0046_0000;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[S3][0]);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4998));
                        break 'b_1ba01c;
                    }
                }
                c.cycles += 20;
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                c.g[S3][0] = 0u64;
                call_ptr(c, 0x0031_7670, 0x001b_a028)?;
                c.g[V1][0] = 0x3;
                c.g[A0][0] = 0x0046_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                call_ptr(c, 0x0031_7670, 0x001b_a040)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x001c_57c0, 0x001b_a050)?;
                c.g[V0][0] = 0x80d;
                c.g[A0][0] = c.g[S2][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x001a_9bc0, 0x001b_a064)?;
                let t = true;
                c.g[A0][0] = c.g[SP][0];
                if t {
                    break 'b_1ba0e4;
                } else {
                    break 'b_1ba06c;
                }
            }
            c.cycles += 5;
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            call_ptr(c, 0x0041_65a8, 0x001b_a078)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[SP][0];
            if t {
                break 'b_1ba0e4;
            } else {
                c.cycles += 25;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x001a_9c50, 0x001b_a088)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = 0x0046_0000;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[S3][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x49b8));
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                c.g[S3][0] = 0u64;
                call_ptr(c, 0x0031_7670, 0x001b_a0a4)?;
                c.g[V1][0] = 0x3;
                c.g[A0][0] = 0x0046_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                call_ptr(c, 0x0031_7670, 0x001b_a0bc)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x001c_57c0, 0x001b_a0cc)?;
                c.g[V0][0] = 0x80d;
                c.g[A0][0] = c.g[S2][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x001a_9bc0, 0x001b_a0e0)?;
                c.g[A0][0] = c.g[SP][0];
                break 'b_1ba0e4;
            }
        }
        c.cycles += 10;
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x001c_5750, 0x001b_a0ec)?;
        c.g[V0][0] = c.g[S3][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateLobbyMenu_OnJoinRoomResult(self, msgPtr)` (0x1bd8c8): the
/// lobby's join-room result. The error code (`msg+8`, a four-character tag)
/// selects the popup ("room full", "invalid room", ...), or, on success, the
/// room's name and player list are copied into the lobby state.
#[allow(unused_labels, unreachable_code)]
pub fn cfe_state_lobby_menu_on_join_room_result(c: &mut Cpu) {
    guest(c, |c| {
        'b_1bda68: {
            'b_1bda5c: {
                'b_1bda4c: {
                    'b_1bda34: {
                        'b_1bd980: {
                            c.cycles += 13;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                            c.g[A2][0] = c.g[A1][0];
                            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = 0x7061_0000;
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.g[V0][0] = c.g[V0][0] | 0x7373;
                            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[S4][0] = c.g[A0][0];
                            if t {
                                break 'b_1bd980;
                            } else {
                                'b_1bd92c: {
                                    'b_1bd920: {
                                        c.cycles += 3;
                                        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = 0x7572_0000;
                                        if t {
                                            break 'b_1bd920;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = 0x6675_0000;
                                            c.g[V0][0] = c.g[V0][0] | 0x6c6c;
                                            let t = c.g[V1][0] == c.g[V0][0];

                                            if t {
                                                break 'b_1bd92c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A1][0] = c.lw((c.g[A2][0] as u32));
                                                if t {
                                                    break 'b_1bda5c;
                                                } else {
                                                    break 'b_1bd920;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = c.g[V0][0] | 0x6f6d;
                                    if c.g[V1][0] != c.g[V0][0] {
                                        c.g[A1][0] = c.lw((c.g[A2][0] as u32));
                                        break 'b_1bda5c;
                                    } else {
                                        break 'b_1bd92c;
                                    }
                                }
                                c.cycles += 21;
                                c.g[A0][0] = c.g[S4][0];
                                call_ptr(c, 0x001a_9c50, 0x001b_d934)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A0][0] = 0x0046_0000;
                                c.g[V0][0] = 0x1;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50a0));
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                call_ptr(c, 0x0031_7670, 0x001b_d950)?;
                                c.g[V1][0] = 0x3;
                                c.g[A0][0] = 0x0046_0000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                                call_ptr(c, 0x0031_7670, 0x001b_d968)?;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A2][0] = c.g[V0][0];
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x001c_57c0, 0x001b_d978)?;
                                let t = true;
                                c.g[V0][0] = 0x82b;
                                if t {
                                    break 'b_1bda34;
                                } else {
                                    break 'b_1bd980;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x770));
                        if c.g[V0][0] != 0u64 {
                            c.g[A1][0] = c.lw((c.g[A2][0] as u32));
                            break 'b_1bda4c;
                        } else {
                            c.cycles += 42;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A0][0] = 0x0046_0000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50b8));
                            c.g[S3][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xb80));
                            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                            c.g[S2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x780));
                            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            call_ptr(c, 0x0031_7670, 0x001b_d9b4)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                            c.g[A0][0] = c.g[S0][0];
                            let t = c.g[V1][0] as u32;
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, t, 0x001b_d9c4)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A2][0] = c.g[S3][0];
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x002c_26d0, 0x001b_d9d4)?;
                            c.g[A0][0] = c.g[S4][0];
                            call_ptr(c, 0x001a_9c50, 0x001b_d9dc)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[V1][0] = 0x1;
                            c.g[V0][0] = 0x2;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[S2][0]);
                            c.g[A0][0] = 0x0046_0000;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50d0));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                            call_ptr(c, 0x0031_7670, 0x001b_da00)?;
                            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                            c.g[A2][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x001c_57c0, 0x001b_da14)?;
                            c.g[A0][0] = 0x0046_0000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2c88));
                            call_ptr(c, 0x0031_7670, 0x001b_da20)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A2][0] = c.g[V0][0];
                            c.g[A1][0] = 0x1;
                            call_ptr(c, 0x001c_57c0, 0x001b_da30)?;
                            c.g[V0][0] = 0x82a;
                            break 'b_1bda34;
                        }
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S4][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x001a_9bc0, 0x001b_da44)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    if t {
                        break 'b_1bda68;
                    } else {
                        break 'b_1bda4c;
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S4][0];
                call_ptr(c, 0x001a_8e40, 0x001b_da54)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                if t {
                    break 'b_1bda68;
                } else {
                    break 'b_1bda5c;
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x001a_8e40, 0x001b_da64)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_1bda68;
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use ssx3_ee_rt::cpu::{reg::*, sx};
    use std::ptr::addr_of_mut;

    /// The state object the tests run the functions on.
    pub(crate) const OC925_THIS: u32 = DATA + 0x1000;
    /// Child objects: `oc925_child(k)`, 0x100 bytes apart.
    pub(crate) const OC925_CHILD: u32 = DATA + 0x2000;
    /// A widget / text object for the callbacks.
    pub(crate) const OC925_WIDGET: u32 = DATA + 0x3000;
    /// The vtable every child points at (`+8` and `+4`).
    pub(crate) const OC925_VT: u32 = DATA + 0x3c00;
    /// The world at `gp - 0x848`, its localisation table and that table's vtable.
    pub(crate) const OC925_WORLD: u32 = DATA + 0x3e00;
    pub(crate) const OC925_LOC: u32 = DATA + 0x3f00;
    /// Forced-result mailbox: entry `(addr & 0xffff) * 8` holds (mode, value).
    pub(crate) const OC925_MAIL: u32 = DATA + 0x20_0000;
    /// The gated fakes answer through the mailbox only while this word holds `OC925_MAGIC`.
    pub(crate) const OC925_GATE: u32 = DATA + 0x2f_0000;
    pub(crate) const OC925_MAGIC: u32 = 0x0925_0925;

    pub(crate) fn oc925_child(k: u32) -> u32 {
        OC925_CHILD + 0x100 * k
    }

    /// The gated fake's body: log the call (`fakes::fake_call`), then return
    /// the forced value of the mailbox entry for `addr` when one is set.
    pub(crate) fn oc925_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = OC925_MAIL + (addr & 0xffff) * 8;
        let mode = c.bus.read32(mail);
        let val = c.bus.read32(mail + 4);
        fakes::fake_call(c, addr);
        if c.pc == ra && mode == 1 {
            c.g[V0][0] = sx(val);
        }
    }

    /// Forces the result of every call to `addr` in this machine to `v`.
    pub(crate) fn oc925_force(c: &mut Cpu, addr: u32, v: u32) {
        let mail = OC925_MAIL + (addr & 0xffff) * 8;
        c.bus.write32(mail, 1);
        c.bus.write32(mail + 4, v);
    }

    /// Gated fakes for the callees of the eight functions. Each one is
    /// installed once per process; with the gate off it runs the body it
    /// replaced (the chain of earlier fakes, or the generated body).
    macro_rules! oc925_fakes {
        ($(($fake:ident, $slot:expr, $gen:path, $addr:expr)),* $(,)?) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC925_GATE) != OC925_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $gen);
                    }
                    oc925_fake_call(c, $addr);
                }
            )*
            fn oc925_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake)); )*
            }
        };
    }

    oc925_fakes!(
        (
            oc925_fake_00194a60,
            ee::s02::OVERRIDE_00194A60,
            ee::s02::f_00194a60__generated,
            0x0019_4a60
        ),
        (
            oc925_fake_001a86e8,
            ee::s03::OVERRIDE_001A86E8,
            ee::s03::f_001a86e8__generated,
            0x001a_86e8
        ),
        (
            oc925_fake_001a8918,
            ee::s03::OVERRIDE_001A8918,
            ee::s03::f_001a8918__generated,
            0x001a_8918
        ),
        (
            oc925_fake_001a8a40,
            ee::s03::OVERRIDE_001A8A40,
            ee::s03::f_001a8a40__generated,
            0x001a_8a40
        ),
        (
            oc925_fake_001a9bc0,
            ee::s03::OVERRIDE_001A9BC0,
            ee::s03::f_001a9bc0__generated,
            0x001a_9bc0
        ),
        (
            oc925_fake_001a9c50,
            ee::s03::OVERRIDE_001A9C50,
            ee::s03::f_001a9c50__generated,
            0x001a_9c50
        ),
        (
            oc925_fake_001b4198,
            ee::s03::OVERRIDE_001B4198,
            ee::s03::f_001b4198__generated,
            0x001b_4198
        ),
        (
            oc925_fake_001b5268,
            ee::s03::OVERRIDE_001B5268,
            ee::s03::f_001b5268__generated,
            0x001b_5268
        ),
        (
            oc925_fake_001c57c0,
            ee::s03::OVERRIDE_001C57C0,
            ee::s03::f_001c57c0__generated,
            0x001c_57c0
        ),
        (
            oc925_fake_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc925_fake_00317d70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc925_fake_0039a7a8,
            ee::s13::OVERRIDE_0039A7A8,
            ee::s13::f_0039a7a8__generated,
            0x0039_a7a8
        ),
        (
            oc925_fake_0014dc80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc925_fake_0014dd58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc925_fake_00397cc0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc925_fake_0039c870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc925_fake_0039d860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc925_fake_003a1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc925_fake_002c24d0,
            ee::s08::OVERRIDE_002C24D0,
            ee::s08::f_002c24d0__generated,
            0x002c_24d0
        ),
        (
            oc925_fake_0039c778,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039_c778
        ),
        (
            oc925_fake_003a0d00,
            ee::s13::OVERRIDE_003A0D00,
            ee::s13::f_003a0d00__generated,
            0x003a_0d00
        ),
        (
            oc925_fake_001abf70,
            ee::s03::OVERRIDE_001ABF70,
            ee::s03::f_001abf70__generated,
            0x001a_bf70
        ),
        (
            oc925_fake_001b70d0,
            ee::s03::OVERRIDE_001B70D0,
            ee::s03::f_001b70d0__generated,
            0x001b_70d0
        ),
        (
            oc925_fake_00265880,
            ee::s07::OVERRIDE_00265880,
            ee::s07::f_00265880__generated,
            0x0026_5880
        ),
        (
            oc925_fake_004166f4,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x0041_66f4
        ),
        (
            oc925_fake_001b9cd8,
            ee::s03::OVERRIDE_001B9CD8,
            ee::s03::f_001b9cd8__generated,
            0x001b_9cd8
        ),
        (
            oc925_fake_001c55f8,
            ee::s03::OVERRIDE_001C55F8,
            ee::s03::f_001c55f8__generated,
            0x001c_55f8
        ),
        (
            oc925_fake_001c5750,
            ee::s03::OVERRIDE_001C5750,
            ee::s03::f_001c5750__generated,
            0x001c_5750
        ),
        (
            oc925_fake_00416810,
            ee::s15::OVERRIDE_00416810,
            ee::s15::f_00416810__generated,
            0x0041_6810
        ),
        (
            oc925_fake_004165a8,
            ee::s15::OVERRIDE_004165A8,
            ee::s15::f_004165a8__generated,
            0x0041_65a8
        ),
        (
            oc925_fake_001a8e40,
            ee::s03::OVERRIDE_001A8E40,
            ee::s03::f_001a8e40__generated,
            0x001a_8e40
        ),
        (
            oc925_fake_002c26d0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c_26d0
        ),
    );

    /// `fakes::machine_rt` with the fakes installed and switched on, and the
    /// fixture world written.
    pub(crate) fn oc925_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc925_install_fakes();
        let r = fakes::machine_rt(c, seed, salt, calls);
        c.bus.write32(OC925_GATE, OC925_MAGIC);
        oc925_fixture(c);
        r
    }

    /// The vtable (methods at `+0x20`, `+0x40`, `+0x48`: `{delta, fake}`),
    /// the world, and the child objects that point at the vtable.
    pub(crate) fn oc925_fixture(c: &mut Cpu) {
        let vt = OC925_VT;
        for off in [0x20u32, 0x40, 0x48] {
            c.bus.write16(vt + off, 0);
            c.bus.write32(vt + off + 4, fakes::FAKE + off);
        }
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC925_WORLD);
        c.bus.write32(OC925_WORLD + 0x8c, OC925_LOC);
        c.bus.write32(OC925_LOC + 4, vt);
        for k in 0..8 {
            let at = oc925_child(k);
            c.bus.write32(at + 4, vt);
            c.bus.write32(at + 8, vt);
        }
    }

    /// Stores child pointers into the object at `at`: each `(off, k)` stores child `k`.
    pub(crate) fn oc925_ptrs(c: &mut Cpu, at: u32, ptrs: &[(u32, u32)]) {
        for &(off, k) in ptrs {
            c.bus.write32(at + off, oc925_child(k));
        }
    }

    #[test]
    fn cfe_state_net_config_on_widget_action_matches() {
        check(
            ee::s03::f_001b60a8__generated,
            cfe_state_net_config_on_widget_action,
            300,
            &|c, seed| {
                let _r = oc925_machine(c, seed, 0x001b_60a8, 24);
                let s = seed as usize;
                let this = OC925_THIS;
                let widget = if s % 9 == 0 { 0 } else { OC925_WIDGET };
                oc925_ptrs(c, this, &[(0x10, 0), (0x40, 1), (0x6e4, 2), (0x6ec, 3)]);
                c.bus.write32(this + 0x6d0, [0u32, 1, 2][s % 3]);
                if widget != 0 {
                    c.bus.write32(widget + 0x18, [0u32, 1, 2, 3][(s / 3) % 4]);
                    c.bus
                        .write32(widget + 0x38, [0u32, 0x1234, 0x5678][(s / 5) % 3]);
                    c.bus.write8(widget + 0x319, [0u8, 1, 2][(s / 7) % 3]);
                }
                if s % 4 == 0 {
                    oc925_force(c, 0x0031_7670, 0);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(widget);
                c.g[A2][0] = [5u64, 6, 7, 8, 9, 3, 0x0c, 0][s % 8];
            },
        );
    }

    #[test]
    fn net_config_auth_popup_init_matches() {
        check(
            ee::s03::f_001b63e8__generated,
            net_config_auth_popup_init,
            200,
            &|c, seed| {
                let _r = oc925_machine(c, seed, 0x001b_63e8, 20);
                let s = seed as usize;
                let this = OC925_THIS;
                oc925_ptrs(c, this, &[(0x10, 0), (0x6d0, 1)]);
                if s % 5 == 0 {
                    oc925_force(c, 0x0039_7cc0, 0);
                }
                if s % 3 == 0 {
                    oc925_force(c, 0x0039_d860, 0);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(oc925_child(4));
            },
        );
    }

    #[test]
    fn net_config_auth_popup_on_widget_created_matches() {
        check(
            ee::s03::f_001b65c0__generated,
            net_config_auth_popup_on_widget_created,
            200,
            &|c, seed| {
                let _r = oc925_machine(c, seed, 0x001b_65c0, 10);
                let s = seed as usize;
                let this = OC925_THIS;
                let text = OC925_WIDGET;
                c.bus.write32(text + 8, OC925_VT);
                let hash = [0x1111u32, 0x2222, 0x3333][s % 3];
                c.bus.write32(text + 0x38, hash);
                c.bus
                    .write32(this + 0x6d4, [0u32, oc925_child(0)][(s / 3) % 2]);
                let forced = [hash, 0x4444][(s / 2) % 2];
                oc925_force(c, 0x0031_7670, forced);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(text);
            },
        );
    }

    #[test]
    fn cfe_state_auth_dnas_error_transition_in_matches() {
        check(
            ee::s03::f_001b6c98__generated,
            cfe_state_auth_dnas_error_transition_in,
            300,
            &|c, seed| {
                let _r = oc925_machine(c, seed, 0x001b_6c98, 16);
                let s = seed as usize;
                let this = OC925_THIS;
                oc925_ptrs(
                    c,
                    this,
                    &[
                        (0x40, 0),
                        (0x6d0, 1),
                        (0x6d4, 2),
                        (0x6d8, 3),
                        (0x6dc, 4),
                        (0x6e0, 5),
                        (0x6e4, 6),
                    ],
                );
                oc925_force(c, 0x002c_24d0, [0u32, 4][s % 2]);
                oc925_force(c, 0x0039_c778, [0xffffu32, 3][(s / 2) % 2]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = [0u64, 1, 1][s % 3];
            },
        );
    }

    #[test]
    fn cfe_state_net_account_mem_ctor_matches() {
        check(
            ee::s03::f_001b6ed0__generated,
            cfe_state_net_account_mem_ctor,
            300,
            &|c, seed| {
                let _r = oc925_machine(c, seed, 0x001b_6ed0, 12);
                let s = seed as usize;
                let this = OC925_THIS;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(oc925_child(0));
                c.g[A2][0] = sx(oc925_child(1));
                c.g[A3][0] = [0u64, 1, 2, 3, 5, 0x0b, 0x13, 0x0053_0000][s % 8];
                c.g[T0][0] = sx([0u32, oc925_child(2)][(s / 8) % 2]);
                c.g[T1][0] = sx([0u32, oc925_child(3)][(s / 16) % 2]);
            },
        );
    }

    #[test]
    fn cfe_state_create_step1_validate_email_matches() {
        check(
            ee::s03::f_001b9b18__generated,
            cfe_state_create_step1_validate_email,
            300,
            &|c, seed| {
                let _r = oc925_machine(c, seed, 0x001b_9b18, 16);
                let s = seed as usize;
                let this = OC925_THIS;
                oc925_ptrs(c, this, &[(0x6d0, 0)]);
                c.bus.write32(oc925_child(0) + 0xc, oc925_child(1));
                oc925_force(c, 0x0041_6810, [0u32, 5][s % 2]);
                oc925_force(c, 0x001b_9cd8, [0u32, 1][(s / 2) % 2]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn cfe_state_create_step1_validate_password_matches() {
        check(
            ee::s03::f_001b9fa0__generated,
            cfe_state_create_step1_validate_password,
            300,
            &|c, seed| {
                let _r = oc925_machine(c, seed, 0x001b_9fa0, 16);
                let s = seed as usize;
                let this = OC925_THIS;
                oc925_ptrs(c, this, &[(0x6d0, 0)]);
                c.bus.write32(oc925_child(0) + 4, oc925_child(1));
                c.bus.write32(oc925_child(0) + 8, oc925_child(2));
                oc925_force(c, 0x0041_6810, [0u32, 5][s % 2]);
                oc925_force(c, 0x0041_65a8, [0u32, 1][(s / 2) % 2]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn cfe_state_lobby_menu_on_join_room_result_matches() {
        check(
            ee::s03::f_001bd8c8__generated,
            cfe_state_lobby_menu_on_join_room_result,
            300,
            &|c, seed| {
                let _r = oc925_machine(c, seed, 0x001b_d8c8, 24);
                let s = seed as usize;
                let this = OC925_THIS;
                let msg = oc925_child(5);
                let errors = [
                    0x7061_7373u32,
                    0x6675_6c6c,
                    0x7572_6f6d,
                    1,
                    0x7572_0000,
                    0x6675_0000,
                    0x006f_6d00,
                ];
                c.bus.write32(msg + 8, errors[s % errors.len()]);
                c.bus.write32(this + 0x770, [0u32, 1][(s / 7) % 2]);
                c.bus.write32(this + 0xb80, [0u32, 7][(s / 3) % 2]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(msg);
            },
        );
    }
}
