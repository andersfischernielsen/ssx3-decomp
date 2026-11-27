//! Staging module oc963 (wave wave257): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc963`.

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
        0x001f_3a38,
        cfe_state_lodge_on_widget_action,
        "cFEStateLodge_OnWidgetAction",
    ),
    (
        0x001f_3cf0,
        cfe_state_lodge_on_child_event,
        "cFEStateLodge_OnChildEvent",
    ),
    (
        0x001f_4400,
        cfe_state_lodge_rider_detail_on_widget_action,
        "cFEStateLodgeRiderDetail_OnWidgetAction",
    ),
    (
        0x001f_4c30,
        cfe_state_buy_attrib_post_input,
        "cFEStateBuyAttrib_PostInput",
    ),
];

/// `void cFEStateLodge_OnWidgetAction(self, widget, event)`: a lodge menu button was
/// activated (event 5, ONNEXT). A locked entry does nothing. Entry 0 (save
/// progress) and entry 6 (quit) open a confirm popup unless a transition is
/// already running; entry 8 opens the profile-load state; any other entry plays
/// the screen's "TransitionOut" frame and pushes the state the custom
/// interface returns.
/// (0x001f3a38; every register, cycle and call of the generated body is kept.)
pub fn cfe_state_lodge_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f3cac: {
            'b_1f3ca8: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[S0][0] == 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                if t {
                    break 'b_1f3ca8;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x5;
                    if c.g[A2][0] != c.g[V0][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        break 'b_1f3cac;
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = 0x440000;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1af8));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                        if c.g[A0][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                            break 'b_1f3cac;
                        } else {
                            'b_1f3be4: {
                                'b_1f3bd4: {
                                    'b_1f3b3c: {
                                        c.cycles += 8;
                                        c.g[RA][0] = sx(0x001f3a90);
                                        call_ptr(c, 0x0014dc80, 0x001f3a90)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001f3a9c);
                                        c.g[A1][0] = 0x4;
                                        call_ptr(c, 0x0014dd58, 0x001f3a9c)?;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                        if c.g[V1][0] != 0u64 {
                                            c.g[S3][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                            break 'b_1f3b3c;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                                            if c.g[V0][0] != 0u64 {
                                                c.g[S3][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                                break 'b_1f3b3c;
                                            } else {
                                                c.cycles += 34;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                c.g[A0][0] = 0x470000;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x1158));
                                                c.g[S0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                                c.g[S1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] =
                                                    c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                                c.g[RA][0] = sx(0x001f3ad4);
                                                c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                call_ptr(c, 0x00317670, 0x001f3ad4)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                                c.g[A0][0] = c.g[S0][0];
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x001f3ae4);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, t, 0x001f3ae4)?;
                                                c.g[A1][0] = 0x470000;
                                                c.g[S0][0] = c.g[V0][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0x1140));
                                                c.g[A0][0] = 0xbfc;
                                                c.g[A2][0] = 0x100;
                                                c.g[RA][0] = sx(0x001f3b00);
                                                c.g[A3][0] = 0u64;
                                                call_ptr(c, 0x00317d70, 0x001f3b00)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[A3][0] = c.g[S0][0];
                                                c.sw((c.g[SP][0] as u32), 0u64);
                                                c.g[A2][0] = c.g[S2][0];
                                                c.g[T0][0] = 0u64;
                                                c.g[T1][0] = 0u64;
                                                c.g[T2][0] = 0u64;
                                                c.g[RA][0] = sx(0x001f3b28);
                                                c.g[T3][0] = 0u64;
                                                Callee {
                                                    addr: 0x001d99c8,
                                                    f: ee::s04::cFEPopupConfirm_ctor,
                                                }
                                                .call(c, 0x001f3b28)?;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                                c.g[V1][0] = 0x10;
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x50),
                                                    c.g[V1][0],
                                                );
                                                let t = true;
                                                c.g[A1][0] = c.g[V0][0];
                                                if t {
                                                    break 'b_1f3bd4;
                                                } else {
                                                    break 'b_1f3b3c;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x6;
                                    let t = c.g[S3][0] != c.g[V0][0];
                                    c.g[V0][0] = 0x8;
                                    if t {
                                        break 'b_1f3be4;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = 0x8;
                                        if t {
                                            break 'b_1f3be4;
                                        } else {
                                            c.cycles += 32;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[A0][0] = 0x470000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x1130));
                                            c.g[S0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                            c.g[S1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                            c.g[RA][0] = sx(0x001f3b74);
                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            call_ptr(c, 0x00317670, 0x001f3b74)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                            c.g[A0][0] = c.g[S0][0];
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001f3b84);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, t, 0x001f3b84)?;
                                            c.g[A1][0] = 0x470000;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_sub(0x1140));
                                            c.g[A0][0] = 0xbfc;
                                            c.g[A2][0] = 0x100;
                                            c.g[RA][0] = sx(0x001f3ba0);
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00317d70, 0x001f3ba0)?;
                                            c.g[A1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[A3][0] = c.g[S0][0];
                                            c.sw((c.g[SP][0] as u32), 0u64);
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[T0][0] = 0u64;
                                            c.g[T1][0] = 0u64;
                                            c.g[T2][0] = 0u64;
                                            c.g[RA][0] = sx(0x001f3bc8);
                                            c.g[T3][0] = 0u64;
                                            Callee {
                                                addr: 0x001d99c8,
                                                f: ee::s04::cFEPopupConfirm_ctor,
                                            }
                                            .call(c, 0x001f3bc8)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                            c.g[A1][0] = c.g[V0][0];
                                            c.sw(
                                                (c.g[S2][0] as u32).wrapping_add(0x50),
                                                c.g[S3][0],
                                            );
                                            break 'b_1f3bd4;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001f3bdc);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f290, 0x001f3bdc)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                if t {
                                    break 'b_1f3cac;
                                } else {
                                    break 'b_1f3be4;
                                }
                            }
                            'b_1f3c4c: {
                                c.cycles += 2;
                                if c.g[S3][0] != c.g[V0][0] {
                                    c.g[A0][0] = 0x470000;
                                    break 'b_1f3c4c;
                                } else {
                                    c.cycles += 24;
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[T0][0] = 0x27;
                                    c.g[A1][0] = 0x470000;
                                    c.g[A3][0] = 0u64;
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                    c.g[A0][0] = 0x280;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1110));
                                    c.g[A2][0] = 0u64;
                                    c.g[RA][0] = sx(0x001f3c14);
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T0][0]);
                                    call_ptr(c, 0x00317d70, 0x001f3c14)?;
                                    c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x14));
                                    c.g[A3][0] = 0x1;
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A3][0] =
                                        sx((c.g[A3][0] as u32) << ((c.g[V1][0] as u32) & 31));
                                    c.g[A2][0] = 0x3;
                                    c.g[RA][0] = sx(0x001f3c34);
                                    c.g[A3][0] = c.g[A3][0] & 0xff;
                                    call_ptr(c, 0x0018eca0, 0x001f3c34)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001f3c44);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f400, 0x001f3c44)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    if t {
                                        break 'b_1f3cac;
                                    } else {
                                        break 'b_1f3c4c;
                                    }
                                }
                            }
                            c.cycles += 20;
                            c.g[RA][0] = sx(0x001f3c54);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fb0));
                            call_ptr(c, 0x00317670, 0x001f3c54)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001f3c60);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039c778, 0x001f3c60)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0] & 0xffff;
                            c.g[RA][0] = sx(0x001f3c70);
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x0039c870, 0x001f3c70)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                            c.g[A3][0] = c.lw((c.g[A0][0] as u32));
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001f3c94);
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001f3c94)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[V0][0];
                            if t {
                                break 'b_1f3ca8;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = sx(0x001f3ca8);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f400, 0x001f3ca8)?;
                                break 'b_1f3ca8;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_1f3cac;
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

/// `void cFEStateLodge_OnChildEvent(self, msg, id)`: child-screen events of the lodge.
/// 0xf and 0x10 track whether a child popup is open (+0x54) and close the top
/// states; 0x10 then re-enters the screen (state 0x10 or 0x11); 0x16 opens the
/// profile-load or save-confirm popup chosen by the pending path (+0x50).
/// (0x001f3cf0; every register, cycle and call of the generated body is kept.)
pub fn cfe_state_lodge_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f3fe8: {
            'b_1f3fe4: {
                'b_1f3fe0: {
                    'b_1f3e68: {
                        'b_1f3d80: {
                            c.cycles += 8;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[S0][0] = 0x10;
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.g[S2][0] = c.g[A0][0];
                            let t = c.g[A2][0] == c.g[S0][0];
                            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                            if t {
                                break 'b_1f3d80;
                            } else {
                                'b_1f3d48: {
                                    'b_1f3d2c: {
                                        c.cycles += 3;
                                        c.g[V0][0] = (c.g[A2][0] < 0x11u64) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0xf;
                                        if t {
                                            break 'b_1f3d2c;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[A2][0] == c.g[V0][0];
                                            c.g[V1][0] = 0x1;
                                            if t {
                                                break 'b_1f3d48;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                if t {
                                                    break 'b_1f3fe4;
                                                } else {
                                                    break 'b_1f3d2c;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x15;
                                    let t = c.g[A2][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x16;
                                    if t {
                                        break 'b_1f3fe0;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[A2][0] == c.g[V0][0] {
                                            c.g[V0][0] =
                                                c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                                            break 'b_1f3e68;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                            if t {
                                                break 'b_1f3fe4;
                                            } else {
                                                break 'b_1f3d48;
                                            }
                                        }
                                    }
                                }
                                'b_1f3d68: {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                                    c.g[V0][0] = 0x6;
                                    let t = c.g[A0][0] == c.g[V0][0];
                                    c.sw((c.g[S2][0] as u32).wrapping_add(0x54), c.g[V1][0]);
                                    if t {
                                        break 'b_1f3d68;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[A0][0] == c.g[S0][0];
                                        c.g[V0][0] = 0x11;
                                        if t {
                                            break 'b_1f3d68;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[A0][0] != c.g[V0][0];
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                            if t {
                                                break 'b_1f3fe4;
                                            } else {
                                                break 'b_1f3d68;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = 0x1;
                                c.g[RA][0] = sx(0x001f3d78);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f190, 0x001f3d78)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                if t {
                                    break 'b_1f3fe4;
                                } else {
                                    break 'b_1f3d80;
                                }
                            }
                        }
                        'b_1f3df8: {
                            'b_1f3db0: {
                                c.cycles += 8;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = 0x1;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x54), 0u64);
                                c.g[RA][0] = sx(0x001f3d94);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f190, 0x001f3d94)?;
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                                let t = c.g[V1][0] == c.g[S0][0];
                                c.g[V0][0] = 0x11;
                                if t {
                                    break 'b_1f3db0;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                    if t {
                                        break 'b_1f3df8;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_1f3fe8;
                                        } else {
                                            break 'b_1f3db0;
                                        }
                                    }
                                }
                            }
                            c.cycles += 18;
                            c.g[V0][0] = 0x1;
                            c.g[S1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                            c.g[A0][0] = 0x470000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1240));
                            c.g[S0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x98));
                            c.g[RA][0] = sx(0x001f3dd0);
                            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                            call_ptr(c, 0x00317670, 0x001f3dd0)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001f3ddc);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001f3ddc)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[V0][0];
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001f3df0);
                            c.g[A2][0] = 0x5;
                            call_ptr(c, t, 0x001f3df0)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            if t {
                                break 'b_1f3fe4;
                            } else {
                                break 'b_1f3df8;
                            }
                        }
                        c.cycles += 23;
                        c.g[V0][0] = 0x1;
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fb0));
                        c.g[RA][0] = sx(0x001f3e0c);
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                        call_ptr(c, 0x00317670, 0x001f3e0c)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001f3e18);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039c778, 0x001f3e18)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0] & 0xffff;
                        c.g[RA][0] = sx(0x001f3e28);
                        c.g[A2][0] = 0x1;
                        call_ptr(c, 0x0039c870, 0x001f3e28)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = 0x6;
                        c.g[A3][0] = c.lw((c.g[A0][0] as u32));
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001f3e4c);
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001f3e4c)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = c.g[V0][0];
                        if t {
                            break 'b_1f3fe0;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[RA][0] = sx(0x001f3e60);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            call_ptr(c, 0x0039f4c0, 0x001f3e60)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            if t {
                                break 'b_1f3fe4;
                            } else {
                                break 'b_1f3e68;
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        break 'b_1f3fe4;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                        if c.g[V0][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            break 'b_1f3fe4;
                        } else {
                            'b_1f3f58: {
                                'b_1f3f28: {
                                    'b_1f3ef0: {
                                        'b_1f3eb4: {
                                            c.cycles += 3;
                                            c.g[V1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                                            let t = c.g[V1][0] == c.g[S0][0];
                                            c.g[V0][0] = ((c.g[V1][0] as i64) < 17i64) as u64;
                                            if t {
                                                break 'b_1f3eb4;
                                            } else {
                                                'b_1f3ea0: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x6;
                                                    if t {
                                                        break 'b_1f3ea0;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] == c.g[V0][0];
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        if t {
                                                            break 'b_1f3f58;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x40),
                                                            );
                                                            if t {
                                                                break 'b_1f3fe4;
                                                            } else {
                                                                break 'b_1f3ea0;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x11;
                                                let t = c.g[V1][0] == c.g[V0][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                if t {
                                                    break 'b_1f3ef0;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        17,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_1f3fe8;
                                                    } else {
                                                        break 'b_1f3eb4;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 15;
                                        c.g[A1][0] = 0x470000;
                                        c.g[A0][0] = 0x280;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1110));
                                        c.g[A2][0] = 0x100;
                                        c.g[RA][0] = sx(0x001f3ecc);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317d70, 0x001f3ecc)?;
                                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A3][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x15));
                                        c.g[RA][0] = sx(0x001f3ee0);
                                        c.g[A2][0] = 0x3;
                                        call_ptr(c, 0x0018eca0, 0x001f3ee0)?;
                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                        c.g[A2][0] = 0xffffffffffffc0ff;
                                        let t = true;
                                        c.g[V1][0] = 0x1;
                                        if t {
                                            break 'b_1f3f28;
                                        } else {
                                            break 'b_1f3ef0;
                                        }
                                    }
                                    c.cycles += 14;
                                    c.g[A1][0] = 0x470000;
                                    c.g[A0][0] = 0x280;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1110));
                                    c.g[A2][0] = 0x100;
                                    c.g[RA][0] = sx(0x001f3f08);
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x001f3f08)?;
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A3][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x15));
                                    c.g[RA][0] = sx(0x001f3f1c);
                                    c.g[A2][0] = 0x3;
                                    call_ptr(c, 0x0018eca0, 0x001f3f1c)?;
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                    c.g[A2][0] = 0xffffffffffffc0ff;
                                    c.g[V1][0] = 0x2;
                                    break 'b_1f3f28;
                                }
                                c.cycles += 12;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[A0][0] & c.g[A2][0];
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x27c), c.g[V1][0]);
                                c.g[A0][0] = c.g[A0][0] | 0x100;
                                c.g[A2][0] = 0u64;
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                                c.g[A3][0] = 0x1;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = sx(0x001f3f50);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f320, 0x001f3f50)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                if t {
                                    break 'b_1f3fe4;
                                } else {
                                    break 'b_1f3f58;
                                }
                            }
                            c.cycles += 34;
                            c.g[A0][0] = 0x470000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10f8));
                            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                            c.g[RA][0] = sx(0x001f3f74);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            call_ptr(c, 0x00317670, 0x001f3f74)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                            c.g[A0][0] = c.g[S0][0];
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001f3f84);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, t, 0x001f3f84)?;
                            c.g[A1][0] = 0x470000;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1140));
                            c.g[A0][0] = 0xbfc;
                            c.g[A2][0] = 0x100;
                            c.g[RA][0] = sx(0x001f3fa0);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x001f3fa0)?;
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = c.g[V0][0];
                            c.g[T0][0] = 0u64;
                            c.g[T1][0] = 0u64;
                            c.g[T2][0] = 0u64;
                            c.g[T3][0] = 0u64;
                            c.g[A3][0] = c.g[S0][0];
                            c.sw((c.g[SP][0] as u32), 0u64);
                            c.g[RA][0] = sx(0x001f3fc8);
                            c.g[A2][0] = c.g[S2][0];
                            Callee {
                                addr: 0x001d99c8,
                                f: ee::s04::cFEPopupConfirm_ctor,
                            }
                            .call(c, 0x001f3fc8)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[V1][0] = 0x11;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x50), c.g[V1][0]);
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001f3fe0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            call_ptr(c, 0x0039f290, 0x001f3fe0)?;
                            break 'b_1f3fe0;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_1f3fe4;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1f3fe8;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateLodgeRiderDetail_OnWidgetAction(self, widget, event)`: ONNEXT (5) on an
/// unlocked entry: 14 opens the name keyboard popup (8 characters, restricted
/// key set, current player name), 11 opens the cheat-character select when the
/// rider owns one, others play "TransitionOut" and push the next state. ONPREV
/// (6) pushes the previous state the custom interface returns.
/// (0x001f4400; every register, cycle and call of the generated body is kept.)
pub fn cfe_state_lodge_rider_detail_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f471c: {
            'b_1f4718: {
                'b_1f4714: {
                    c.cycles += 7;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A1][0];
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    let t = c.g[S0][0] == 0u64;
                    c.g[S1][0] = c.g[A0][0];
                    if t {
                        break 'b_1f4714;
                    } else {
                        'b_1f46dc: {
                            'b_1f4438: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x5;
                                let t = c.g[A2][0] == c.g[V0][0];
                                c.g[V0][0] = 0x6;
                                if t {
                                    break 'b_1f4438;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[A2][0] == c.g[V0][0];
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_1f46dc;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        if t {
                                            break 'b_1f471c;
                                        } else {
                                            break 'b_1f4438;
                                        }
                                    }
                                }
                            }
                            c.cycles += 8;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                            c.g[V0][0] = 0x440000;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1af8));
                            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                            let t = c.g[A0][0] == 0u64;
                            c.g[V0][0] = 0xe;
                            if t {
                                break 'b_1f4714;
                            } else {
                                'b_1f45e0: {
                                    c.cycles += 2;
                                    let t = c.g[A1][0] != c.g[V0][0];
                                    c.g[V0][0] = 0xb;
                                    if t {
                                        break 'b_1f45e0;
                                    } else {
                                        c.cycles += 96;
                                        c.g[A1][0] = 0x470000;
                                        c.g[A0][0] = 0x444;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1030));
                                        c.g[A2][0] = 0x100;
                                        c.g[RA][0] = sx(0x001f4478);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317d70, 0x001f4478)?;
                                        c.g[T0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x15));
                                        c.g[A2][0] = c.g[S1][0];
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                        c.g[A3][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f4490);
                                        c.g[A0][0] = c.g[V0][0];
                                        Callee {
                                            addr: 0x001cb030,
                                            f: ee::s04::cKeyboardPopup_ctor,
                                        }
                                        .call(c, 0x001f4490)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x8;
                                        c.g[V0][0] = 0x37;
                                        c.sb((c.g[S0][0] as u32).wrapping_add(0x44), 0u64);
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x140), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x001f44ac);
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x001cd088,
                                            f: ee::s04::cKeyboardPopup_SetMaxLength,
                                        }
                                        .call(c, 0x001f44ac)?;
                                        c.g[A2][0] = 0x1;
                                        c.g[A3][0] = 0x2;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001f44c0);
                                        c.g[A1][0] = 0x4b;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f44c0)?;
                                        c.g[V0][0] = 0x1;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x001f44d4);
                                        c.g[A1][0] = 0x1;
                                        Callee {
                                            addr: 0x001ce408,
                                            f: ee::s04::cKeyboardPopup_SetDigitKeysDisabled,
                                        }
                                        .call(c, 0x001f44d4)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0xe;
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f44e8);
                                        c.g[A3][0] = 0x2;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f44e8)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x27;
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f44fc);
                                        c.g[A3][0] = 0x2;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f44fc)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x28;
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f4510);
                                        c.g[A3][0] = 0x2;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f4510)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x34;
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f4524);
                                        c.g[A3][0] = 0x2;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f4524)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x1a;
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f4538);
                                        c.g[A3][0] = 0x2;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f4538)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x19;
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f454c);
                                        c.g[A3][0] = 0x2;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f454c)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x35;
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f4560);
                                        c.g[A3][0] = 0x2;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f4560)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x40;
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f4574);
                                        c.g[A3][0] = 0x2;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f4574)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x41;
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f4588);
                                        c.g[A3][0] = 0x2;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f4588)?;
                                        c.g[A2][0] = 0u64;
                                        c.g[A3][0] = 0x2;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001f459c);
                                        c.g[A1][0] = 0x42;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f459c)?;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                        c.g[A1][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001f45ac);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        call_ptr(c, 0x0039f290, 0x001f45ac)?;
                                        c.g[RA][0] = sx(0x001f45b4);
                                        call_ptr(c, 0x0014dc80, 0x001f45b4)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001f45c0);
                                        c.g[A1][0] = 0x1;
                                        call_ptr(c, 0x0014dd58, 0x001f45c0)?;
                                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                                        c.g[RA][0] = sx(0x001f45cc);
                                        c.g[A0][0] = c.g[V0][0];
                                        call_ptr(c, 0x00147170, 0x001f45cc)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001f45d8);
                                        c.g[A1][0] = c.g[V0][0];
                                        Callee {
                                            addr: 0x001cb418,
                                            f: ee::s04::cKeyboardPopup_SetText,
                                        }
                                        .call(c, 0x001f45d8)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_1f4718;
                                        } else {
                                            break 'b_1f45e0;
                                        }
                                    }
                                }
                                'b_1f4678: {
                                    c.cycles += 2;
                                    if c.g[A1][0] != c.g[V0][0] {
                                        c.g[A0][0] = 0x470000;
                                        break 'b_1f4678;
                                    } else {
                                        c.cycles += 19;
                                        c.g[RA][0] = sx(0x001f45f0);
                                        call_ptr(c, 0x0014dc80, 0x001f45f0)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001f45fc);
                                        c.g[A1][0] = 0x1;
                                        call_ptr(c, 0x0014dd58, 0x001f45fc)?;
                                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                                        c.g[RA][0] = sx(0x001f4608);
                                        c.g[A0][0] = c.g[V0][0];
                                        call_ptr(c, 0x00147488, 0x001f4608)?;
                                        c.g[RA][0] = sx(0x001f4610);
                                        c.g[S0][0] = c.g[V0][0];
                                        call_ptr(c, 0x0014dc80, 0x001f4610)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001f461c);
                                        c.g[A1][0] = 0xd;
                                        call_ptr(c, 0x0014dd58, 0x001f461c)?;
                                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001f462c);
                                        c.g[A2][0] = c.g[S0][0];
                                        call_ptr(c, 0x001577a0, 0x001f462c)?;
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[A1][0] = 0x470000;
                                        if t {
                                            break 'b_1f4714;
                                        } else {
                                            c.cycles += 17;
                                            c.g[A0][0] = 0x1e4;
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_sub(0x1020));
                                            c.g[A2][0] = 0u64;
                                            c.g[RA][0] = sx(0x001f4648);
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00317d70, 0x001f4648)?;
                                            c.g[A1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[A3][0] =
                                                c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x001f4660);
                                            c.g[A2][0] = c.g[S1][0];
                                            call_ptr(c, 0x00182220, 0x001f4660)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x001f4670);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                            call_ptr(c, 0x0039f290, 0x001f4670)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            if t {
                                                break 'b_1f4718;
                                            } else {
                                                break 'b_1f4678;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 20;
                                c.g[RA][0] = sx(0x001f4680);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fb0));
                                call_ptr(c, 0x00317670, 0x001f4680)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001f468c);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039c778, 0x001f468c)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[A1][0] = c.g[V0][0] & 0xffff;
                                c.g[RA][0] = sx(0x001f469c);
                                c.g[A2][0] = 0x1;
                                call_ptr(c, 0x0039c870, 0x001f469c)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                c.g[A3][0] = c.lw((c.g[A0][0] as u32));
                                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001f46c0);
                                c.g[A0][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001f46c0)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A1][0] = c.g[V0][0];
                                if t {
                                    break 'b_1f4714;
                                } else {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    c.g[RA][0] = sx(0x001f46d4);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f400, 0x001f46d4)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_1f4718;
                                    } else {
                                        break 'b_1f46dc;
                                    }
                                }
                            }
                        }
                        c.cycles += 11;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = c.lw((c.g[V0][0] as u32));
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001f4700);
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001f4700)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = c.g[V0][0];
                        if t {
                            break 'b_1f4714;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                            c.g[RA][0] = sx(0x001f4714);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            call_ptr(c, 0x0039f400, 0x001f4714)?;
                            break 'b_1f4714;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_1f4718;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1f471c;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateBuyAttrib_PostInput(self, widget)`: after the input pass, if not busy,
/// the attribute widget's decrement (vtable 0x98) refunds one bought level and
/// its increment (0xa0) buys one when the cost fits the bank and the per-level
/// limit; otherwise a "cannot" message goes to the custom interface. Updates
/// the total cost and the bars when anything changed.
/// (0x001f4c30; every register, cycle and call of the generated body is kept.)
pub fn cfe_state_buy_attrib_post_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f4e74: {
            c.cycles += 27;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S0][0] = c.g[A0][0];
            c.g[S1][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x001f4c5c);
            c.g[S4][0] = 0u64;
            call_ptr(c, 0x0039e508, 0x001f4c5c)?;
            c.g[RA][0] = sx(0x001f4c64);
            call_ptr(c, 0x0014dc80, 0x001f4c64)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001f4c70);
            c.g[A1][0] = 0xb;
            call_ptr(c, 0x0014dd58, 0x001f4c70)?;
            c.g[A0][0] = 0x470000;
            c.g[S3][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001f4c80);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xfc0));
            call_ptr(c, 0x00317670, 0x001f4c80)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
            c.g[RA][0] = sx(0x001f4c8c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001f4c8c)?;
            c.g[S2][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa8));
            let t = c.g[V0][0] != 0u64;

            if t {
                break 'b_1f4e74;
            } else {
                'b_1f4e4c: {
                    'b_1f4e48: {
                        'b_1f4e30: {
                            'b_1f4de4: {
                                'b_1f4d28: {
                                    c.cycles += 7;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x98));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x001f4cb0);
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001f4cb0)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x84));
                                    if t {
                                        break 'b_1f4d28;
                                    } else {
                                        'b_1f4d08: {
                                            c.cycles += 5;
                                            c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                                            c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                            let t = (c.g[V0][0] as i64) <= 0;
                                            c.g[S4][0] = 0x1;
                                            if t {
                                                break 'b_1f4d08;
                                            } else {
                                                c.cycles += 15;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                                c.g[V1][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add((c.g[V1][0] as u32)));
                                                c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                                                c.g[A0][0] = c.g[S3][0];
                                                c.g[A1][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x68));
                                                c.g[RA][0] = sx(0x001f4ce8);
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                                                call_ptr(c, 0x00150e50, 0x001f4ce8)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_sub((c.g[V0][0] as u32)));
                                                c.g[RA][0] = sx(0x001f4cfc);
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xa0),
                                                    c.g[V1][0],
                                                );
                                                Callee {
                                                    addr: 0x001f5240,
                                                    f: ee::s04::cFEStateBuyAttrib_UpdateTotalCost,
                                                }
                                                .call(c, 0x001f4cfc)?;
                                                c.g[A0][0] = 0x4a0000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x2330));
                                                if t {
                                                    break 'b_1f4de4;
                                                } else {
                                                    break 'b_1f4d08;
                                                }
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                        c.g[V1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
                                        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                                        if c.g[A2][0] == 0 {
                                            c.g[A2][0] = c.g[V1][0];
                                        }
                                        let t = c.g[A2][0] == 0u64;
                                        c.g[A1][0] = 0x4;
                                        if t {
                                            break 'b_1f4e74;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                                            if t {
                                                break 'b_1f4e30;
                                            } else {
                                                break 'b_1f4d28;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 7;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001f4d3c);
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001f4d3c)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x68));
                                if t {
                                    break 'b_1f4e74;
                                } else {
                                    c.cycles += 12;
                                    c.g[S1][0] = sx((c.g[S2][0] as u32) << 2);
                                    c.g[S2][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                                    c.g[A0][0] = c.g[S3][0];
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32));
                                    c.g[S4][0] = 0x1;
                                    c.g[RA][0] = sx(0x001f4d60);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                                    call_ptr(c, 0x00150e50, 0x001f4d60)?;
                                    c.g[A2][0] = c.lw((c.g[S2][0] as u32));
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                    c.g[A0][0] = ((c.g[A2][0] as i64) < 11i64) as u64;
                                    let t = c.g[A0][0] == 0u64;
                                    c.g[V1][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    if t {
                                        break 'b_1f4e48;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                                        c.g[V0][0] =
                                            ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                            break 'b_1f4e4c;
                                        } else {
                                            c.cycles += 12;
                                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x84));
                                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                            c.g[A3][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[S1][0] as u32)));
                                            c.g[A1][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[S1][0] as u32)));
                                            c.g[A2][0] = c.lw((c.g[A3][0] as u32));
                                            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                                            c.g[V0][0] =
                                                c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[A2][0] as u32)));
                                            c.g[V0][0] =
                                                ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                            if t {
                                                break 'b_1f4e74;
                                            } else {
                                                c.cycles += 12;
                                                c.g[A0][0] = c.g[S3][0];
                                                c.sw((c.g[A3][0] as u32), c.g[V0][0]);
                                                c.g[A1][0] = c.lw((c.g[S2][0] as u32));
                                                c.g[RA][0] = sx(0x001f4dc8);
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                                                call_ptr(c, 0x00150e50, 0x001f4dc8)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[RA][0] = sx(0x001f4ddc);
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xa0),
                                                    c.g[V1][0],
                                                );
                                                Callee {
                                                    addr: 0x001f5240,
                                                    f: ee::s04::cFEStateBuyAttrib_UpdateTotalCost,
                                                }
                                                .call(c, 0x001f4ddc)?;
                                                c.g[A0][0] = 0x470000;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0xf90));
                                                break 'b_1f4de4;
                                            }
                                        }
                                    }
                                }
                            }
                            'b_1f4e18: {
                                c.cycles += 9;
                                c.g[RA][0] = sx(0x001f4dec);
                                call_ptr(c, 0x00317670, 0x001f4dec)?;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                                c.g[RA][0] = sx(0x001f4df8);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039c778, 0x001f4df8)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V0][0] = 0xffff;
                                if c.g[A1][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    break 'b_1f4e18;
                                } else {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                                    c.g[RA][0] = sx(0x001f4e14);
                                    c.g[A2][0] = 0x1;
                                    call_ptr(c, 0x0039c870, 0x001f4e14)?;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    break 'b_1f4e18;
                                }
                            }
                            c.cycles += 5;
                            c.g[V1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
                            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                            if c.g[A2][0] == 0 {
                                c.g[A2][0] = c.g[V1][0];
                            }
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = 0x1;
                            if t {
                                break 'b_1f4e74;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                                break 'b_1f4e30;
                            }
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001f4e40);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001f4e40)?;
                        let t = true;

                        if t {
                            break 'b_1f4e74;
                        } else {
                            break 'b_1f4e48;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    break 'b_1f4e4c;
                }
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                if c.g[A2][0] == 0 {
                    c.g[A2][0] = c.g[V1][0];
                }
                let t = c.g[A2][0] == 0u64;
                c.g[A1][0] = 0x4;
                if t {
                    break 'b_1f4e74;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001f4e74);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001f4e74)?;
                    break 'b_1f4e74;
                }
            }
        }
        'b_1f4e88: {
            c.cycles += 2;
            if c.g[S4][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_1f4e88;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001f4e84);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001f50b0,
                    f: ee::s04::cFEStateBuyAttrib_UpdateBars,
                }
                .call(c, 0x001f4e84)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_1f4e88;
            }
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
    use crate::difftest::fakes::{self, fake_call, machine, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes: only machines holding the magic here
    /// take them (the fakes sit in shared override slots).
    const OC963_CTRL: u32 = DATA + 0x20_0000;
    const OC963_MAGIC: u32 = 0x0963_1f3a;
    /// Forced results (own mailbox): per callee `[flag, value]`.
    const OC963_FORCE: u32 = OC963_CTRL + 0x10;
    const OC963_F_OBJECT: u32 = 0; // UIScreen_FindObject (0x39d860)
    const OC963_F_OWNED: u32 = 1; // getNumOwnedCheatChars (0x1577a0)
    const OC963_F_COST: u32 = 2; // attribute level cost (0x150e50)
    const OC963_F_LOOKUP: u32 = 3; // UIScreen_LookupValue (0x39c778)

    const OC963_SELF: u32 = DATA + 0x20_1000;
    const OC963_ENG: u32 = DATA + 0x20_2000;
    const OC963_CUS: u32 = DATA + 0x20_3000;
    const OC963_CUSVT: u32 = DATA + 0x20_4000;
    const OC963_SCREEN: u32 = DATA + 0x20_5000;
    const OC963_WIDGET: u32 = DATA + 0x20_6000;
    const OC963_WVT: u32 = DATA + 0x20_7000;
    const OC963_MSG: u32 = DATA + 0x20_8000;
    const OC963_WORLD: u32 = DATA + 0x20_9000;
    const OC963_LOC: u32 = DATA + 0x20_a000;
    const OC963_LOCVT: u32 = DATA + 0x20_b000;
    const OC963_SELFVT: u32 = DATA + 0x20_c000;
    const OC963_OBJ95: u32 = DATA + 0x20_d000;
    const OC963_MSGOBJ: u32 = DATA + 0x20_e000;
    const OC963_MSGVT: u32 = DATA + 0x20_f000;
    /// A real leaf that returns 0 (a vtable slot pointing here answers null).
    const OC963_RET_ZERO: u32 = 0x0028_b180;
    /// The lodge "unlocked" table.
    const OC963_UNLOCKED: u32 = 0x0044_1af8;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// A vtable slot `{delta (halfword), method}` at `vt + off`.
    fn oc963_slot(c: &mut Cpu, vt: u32, off: u32, target: u32, delta: u16) {
        c.bus.write16(vt + off, delta);
        c.bus.write32(vt + off + 4, target);
    }

    fn oc963_force(c: &mut Cpu, k: u32, v: u32) {
        c.bus.write32(OC963_FORCE + 8 * k, 1);
        c.bus.write32(OC963_FORCE + 8 * k + 4, v);
    }

    /// A method that answers null (a real leaf) or a nonzero pointer (a fake).
    fn oc963_method(r: &mut Rng) -> u32 {
        match r.next_u32() % 3 {
            0 => OC963_RET_ZERO,
            n => FAKE + 0x10 * n,
        }
    }

    /// A fake callee: logs the call like `fakes::fake_call`, then answers the
    /// forced result of the mailbox, if the test set one.
    fn oc963_hit(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        let k = match addr {
            0x0039_d860 => OC963_F_OBJECT,
            0x0015_77a0 => OC963_F_OWNED,
            0x0015_0e50 => OC963_F_COST,
            0x0039_c778 => OC963_F_LOOKUP,
            _ => return,
        };
        if c.bus.read32(OC963_FORCE + 8 * k) != 0 {
            let v = c.bus.read32(OC963_FORCE + 8 * k + 4);
            c.g[V0][0] = sx(v);
        }
    }

    macro_rules! oc963_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC963_CTRL) != OC963_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc963_hit(c, $addr);
                }
            )*
            fn oc963_install() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc963_fakes!(
        (
            oc963_fake_0014dc80,
            OC963_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc963_fake_0014dd58,
            OC963_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc963_fake_00147170,
            OC963_PREV_00147170,
            ee::s01::OVERRIDE_00147170,
            ee::s01::f_00147170__generated,
            0x0014_7170
        ),
        (
            oc963_fake_00147488,
            OC963_PREV_00147488,
            ee::s01::OVERRIDE_00147488,
            ee::s01::f_00147488__generated,
            0x0014_7488
        ),
        (
            oc963_fake_001577a0,
            OC963_PREV_001577A0,
            ee::s01::OVERRIDE_001577A0,
            ee::s01::f_001577a0__generated,
            0x0015_77a0
        ),
        (
            oc963_fake_00150e50,
            OC963_PREV_00150E50,
            ee::s01::OVERRIDE_00150E50,
            ee::s01::f_00150e50__generated,
            0x0015_0e50
        ),
        (
            oc963_fake_0018eca0,
            OC963_PREV_0018ECA0,
            ee::s02::OVERRIDE_0018ECA0,
            ee::s02::f_0018eca0__generated,
            0x0018_eca0
        ),
        (
            oc963_fake_00182220,
            OC963_PREV_00182220,
            ee::s02::OVERRIDE_00182220,
            ee::s02::f_00182220__generated,
            0x0018_2220
        ),
        (
            oc963_fake_00317670,
            OC963_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc963_fake_00317d70,
            OC963_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc963_fake_0039c778,
            OC963_PREV_0039C778,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039_c778
        ),
        (
            oc963_fake_0039c870,
            OC963_PREV_0039C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc963_fake_0039f400,
            OC963_PREV_0039F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc963_fake_0039d860,
            OC963_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc963_fake_0039f190,
            OC963_PREV_0039F190,
            ee::s13::OVERRIDE_0039F190,
            ee::s13::f_0039f190__generated,
            0x0039_f190
        ),
        (
            oc963_fake_0039f320,
            OC963_PREV_0039F320,
            ee::s13::OVERRIDE_0039F320,
            ee::s13::f_0039f320__generated,
            0x0039_f320
        ),
        (
            oc963_fake_0039f4c0,
            OC963_PREV_0039F4C0,
            ee::s13::OVERRIDE_0039F4C0,
            ee::s13::f_0039f4c0__generated,
            0x0039_f4c0
        ),
        (
            oc963_fake_0039e508,
            OC963_PREV_0039E508,
            ee::s13::OVERRIDE_0039E508,
            ee::s13::f_0039e508__generated,
            0x0039_e508
        ),
        (
            oc963_fake_001d99c8,
            OC963_PREV_001D99C8,
            ee::s04::OVERRIDE_001D99C8,
            ee::s04::f_001d99c8__generated,
            0x001d_99c8
        ),
        (
            oc963_fake_001cb030,
            OC963_PREV_001CB030,
            ee::s04::OVERRIDE_001CB030,
            ee::s04::f_001cb030__generated,
            0x001c_b030
        ),
        (
            oc963_fake_001cb418,
            OC963_PREV_001CB418,
            ee::s04::OVERRIDE_001CB418,
            ee::s04::f_001cb418__generated,
            0x001c_b418
        ),
        (
            oc963_fake_001cd088,
            OC963_PREV_001CD088,
            ee::s04::OVERRIDE_001CD088,
            ee::s04::f_001cd088__generated,
            0x001c_d088
        ),
        (
            oc963_fake_001ce3c8,
            OC963_PREV_001CE3C8,
            ee::s04::OVERRIDE_001CE3C8,
            ee::s04::f_001ce3c8__generated,
            0x001c_e3c8
        ),
        (
            oc963_fake_001ce408,
            OC963_PREV_001CE408,
            ee::s04::OVERRIDE_001CE408,
            ee::s04::f_001ce408__generated,
            0x001c_e408
        ),
        (
            oc963_fake_001f50b0,
            OC963_PREV_001F50B0,
            ee::s04::OVERRIDE_001F50B0,
            ee::s04::f_001f50b0__generated,
            0x001f_50b0
        ),
        (
            oc963_fake_001f5240,
            OC963_PREV_001F5240,
            ee::s04::OVERRIDE_001F5240,
            ee::s04::f_001f5240__generated,
            0x001f_5240
        ),
    );

    /// The lodge-state fixture: the state at `OC963_SELF` with its engine, the
    /// custom interface (vtable slots 0x20 next / 0x28 previous state), the
    /// screen, the locale manager and the unlocked table (all entries open).
    fn oc963_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        oc963_install();
        let mut r = machine(c, seed, salt, 12);
        w(c, OC963_CTRL, OC963_MAGIC);
        w(c, GAME_GP.wrapping_sub(0x848), OC963_WORLD);
        w(c, OC963_WORLD + 0x8c, OC963_LOC);
        w(c, OC963_LOC + 4, OC963_LOCVT);
        oc963_slot(c, OC963_LOCVT, 0x20, FAKE + 0x40, 4 * (seed % 3) as u16);
        w(c, OC963_SELF + 0x10, OC963_ENG);
        w(c, OC963_SELF + 0x40, OC963_SCREEN);
        w(c, OC963_SELF + 8, OC963_SELFVT);
        w(c, OC963_ENG, OC963_CUS);
        w(c, OC963_CUS + 4, OC963_CUSVT);
        let next = oc963_method(&mut r);
        oc963_slot(c, OC963_CUSVT, 0x20, next, 8 * (seed % 2) as u16);
        let prev = oc963_method(&mut r);
        oc963_slot(c, OC963_CUSVT, 0x28, prev, 0);
        oc963_slot(c, OC963_SELFVT, 0x98, FAKE + 0x80, 4 * (seed % 5) as u16);
        w(c, OC963_WIDGET + 8, OC963_WVT);
        for i in 0..16 {
            w(c, OC963_UNLOCKED + 4 * i, 1);
        }
        c.bus.write8(OC963_SELF + 0x14, (r.next_u32() % 4) as u8);
        c.bus.write8(OC963_SELF + 0x15, (r.next_u32() % 16) as u8);
        c.g[A0][0] = sx(OC963_SELF);
        r
    }

    /// A random choice of a widget id: sometimes locked in the table.
    fn oc963_widget(c: &mut Cpu, r: &mut Rng, ids: &[u32]) -> u32 {
        let idx = ids[(r.next_u32() as usize) % ids.len()];
        w(c, OC963_WIDGET + 0x18, idx);
        if r.next_u32() % 6 == 0 {
            w(c, OC963_UNLOCKED + 4 * idx, 0);
        }
        c.g[A1][0] = if r.next_u32() % 12 == 0 {
            0
        } else {
            sx(OC963_WIDGET)
        };
        idx
    }

    /// Lodge widget action: the save-progress popup (entry 0), the quit popup
    /// (6), the profile load (8) and the generic transition, with locked
    /// entries, a null widget, other events, and a running transition.
    #[test]
    fn cfe_state_lodge_on_widget_action_matches() {
        check(
            ee::s04::f_001f3a38__generated,
            cfe_state_lodge_on_widget_action,
            240,
            &|c, seed| {
                let mut r = oc963_base(c, seed, 0x1f3a38);
                oc963_widget(c, &mut r, &[0, 6, 8, 3]);
                w(c, OC963_SELF + 0x4c, r.next_u32() % 2);
                c.g[A2][0] = if r.next_u32() % 6 == 0 { 4 } else { 5 };
            },
        );
    }

    /// Lodge child events 0xf, 0x10, 0x15, 0x16 and others, for each pending
    /// path (6, 0x10, 0x11, other), with the popup-open flag set and clear.
    #[test]
    fn cfe_state_lodge_on_child_event_matches() {
        check(
            ee::s04::f_001f3cf0__generated,
            cfe_state_lodge_on_child_event,
            300,
            &|c, seed| {
                let mut r = oc963_base(c, seed, 0x1f3cf0);
                let id =
                    [0xf, 0x10, 0x15, 0x16, 0x7, 0x10, 0x16, 0x16][(r.next_u32() % 8) as usize];
                let state = [6, 0x10, 0x11, 3][(r.next_u32() % 4) as usize];
                w(c, OC963_SELF + 0x50, state);
                w(c, OC963_SELF + 0x54, r.next_u32() % 2);
                w(c, OC963_MSG + 0xc, (r.next_u32() % 5 == 0) as u32);
                c.g[A1][0] = sx(OC963_MSG);
                c.g[A2][0] = id as u64;
            },
        );
    }

    /// Rider-detail widget action: ONNEXT on entries 14 (name keyboard), 11
    /// (cheat character select, owned or not) and others; ONPREV; locked
    /// entries and a null widget.
    #[test]
    fn cfe_state_lodge_rider_detail_on_widget_action_matches() {
        check(
            ee::s04::f_001f4400__generated,
            cfe_state_lodge_rider_detail_on_widget_action,
            300,
            &|c, seed| {
                let mut r = oc963_base(c, seed, 0x1f4400);
                oc963_widget(c, &mut r, &[14, 11, 3, 14, 11, 5]);
                let owned = [0, 1, 3][(r.next_u32() % 3) as usize];
                oc963_force(c, OC963_F_OWNED, owned);
                c.g[A2][0] = [5u64, 5, 5, 6, 7][(r.next_u32() % 5) as usize];
            },
        );
    }

    /// Buy-attribute post-input: the widget's decrement (vtable 0x98) refunds
    /// a bought level, the increment (0xa0) buys one when the cost fits the
    /// bank and the level limit; refusals send the "cannot" message; a busy
    /// state does nothing; the bars update when something changed.
    #[test]
    fn cfe_state_buy_attrib_post_input_matches() {
        check(
            ee::s04::f_001f4c30__generated,
            cfe_state_buy_attrib_post_input,
            400,
            &|c, seed| {
                let mut r = oc963_base(c, seed, 0x1f4c30);
                let idx = r.next_u32() % 4;
                c.bus.write8(OC963_OBJ95 + 0x95, idx as u8);
                oc963_force(c, OC963_F_OBJECT, OC963_OBJ95);
                oc963_force(c, OC963_F_COST, 5 + r.next_u32() % 20);
                oc963_force(
                    c,
                    OC963_F_LOOKUP,
                    if r.next_u32() % 2 == 0 { 0xffff } else { 3 },
                );
                w(c, OC963_SELF + 0x48, OC963_SCREEN);
                w(c, OC963_SELF + 0xa8, (r.next_u32() % 6 == 0) as u32);
                w(c, OC963_SELF + 0xa0, r.next_u32() % 100);
                w(c, OC963_SELF + 0xa4, r.next_u32() % 120);
                for i in 0..4 {
                    w(c, OC963_SELF + 0x84 + 4 * i, r.next_u32() % 4);
                    w(c, OC963_SELF + 0x68 + 4 * i, 1 + r.next_u32() % 12);
                    w(c, OC963_SELF + 0x4c + 4 * i, r.next_u32() % 30);
                }
                let dec = oc963_method(&mut r);
                let inc = oc963_method(&mut r);
                oc963_slot(c, OC963_WVT, 0x98, dec, 0);
                oc963_slot(c, OC963_WVT, 0xa0, inc, 4);
                // The message object: the engine's own, or the gp fallback.
                w(c, OC963_MSGOBJ, OC963_MSGVT);
                w(c, GAME_GP + 0x2968, OC963_MSGVT);
                w(
                    c,
                    OC963_ENG + 0x14,
                    if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        OC963_MSGOBJ
                    },
                );
                oc963_slot(c, OC963_MSGVT, 0x10, FAKE + 0xc0, 0);
                c.g[A1][0] = sx(OC963_WIDGET);
            },
        );
    }
}
