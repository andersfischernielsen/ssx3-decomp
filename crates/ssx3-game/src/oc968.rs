//! Staging module oc968 (wave wave257): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc968`.

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
        0x001e4b80,
        ov_screen_message_center_get_group_variant,
        "OVScreenMessageCenter_GetGroupVariant",
    ),
    (
        0x001e52b0,
        c_ov_state_view_popup_update,
        "cOVState_ViewPopup_Update",
    ),
    (
        0x001e52f8,
        c_ov_state_view_popup_on_widget_created,
        "cOVState_ViewPopup_OnWidgetCreated",
    ),
    (
        0x001e5428,
        c_ov_state_view_popup_show_message,
        "cOVState_ViewPopup_ShowMessage",
    ),
];

/// `OVScreenMessageCenter_Init` (0x001e3d58): builds the message-center screen: adds it to the UI engine by its hash name, plays it, and refreshes the list, count, button and arrow widgets.
/// The body keeps the generated block structure: every register, cycle and call is as generated.
// Not installed: its real callees run into the allocator on test data and
// no override slot exists in this module to fake them (see the wave report).
#[allow(dead_code)]
pub fn ov_screen_message_center_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e3da8: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = 0x470000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2078));
            c.g[RA][0] = sx(0x001e3d7c);
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001e3d7c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001e3d90);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001e3d90)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x17c), c.g[V0][0]);
            if t {
                break 'b_1e3da8;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001e3da8);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001e3da8)?;
                break 'b_1e3da8;
            }
        }
        'b_1e3dcc: {
            'b_1e3dc4: {
                c.cycles += 5;
                c.g[RA][0] = sx(0x001e3db0);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0020a380, 0x001e3db0)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_1e3dc4;
                } else {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];

                    if t {
                        break 'b_1e3dcc;
                    } else {
                        break 'b_1e3dc4;
                    }
                }
            }
            c.cycles += 2;
            call_ptr(c, 0x0020d110, 0x001e3dcc)?;
            break 'b_1e3dcc;
        }
        c.cycles += 25;
        c.g[RA][0] = sx(0x001e3dd4);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x174), 0u64);
        Callee {
            addr: 0x001e33e8,
            f: ee::s04::MessageCenter_GetMessageCount,
        }
        .call(c, 0x001e3dd4)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x4;
        c.g[RA][0] = sx(0x001e3de4);
        c.g[A1][0] = 0x1;
        Callee {
            addr: 0x001e2bb8,
            f: ee::s04::MessageCenter_CountGroupMessages,
        }
        .call(c, 0x001e3de4)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x144));
        c.g[RA][0] = sx(0x001e3df4);
        c.g[A1][0] = c.g[S0][0] & 0xff;
        call_ptr(c, 0x0039b760, 0x001e3df4)?;
        c.g[RA][0] = sx(0x001e3dfc);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x001e48b0,
            f: ee::s04::OVScreenMessageCenter_RefreshList,
        }
        .call(c, 0x001e3dfc)?;
        c.g[RA][0] = sx(0x001e3e04);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x001e4758,
            f: ee::s04::OVScreenMessageCenter_UpdateCountText,
        }
        .call(c, 0x001e3e04)?;
        c.g[RA][0] = sx(0x001e3e0c);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x001e4458,
            f: ee::s04::OVScreenMessageCenter_UpdateButtonText,
        }
        .call(c, 0x001e3e0c)?;
        c.g[RA][0] = sx(0x001e3e14);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x001e4658,
            f: ee::s04::OVScreenMessageCenter_UpdateScrollArrows,
        }
        .call(c, 0x001e3e14)?;
        c.g[V0][0] = 0x1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1030), c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenMessageCenter_UpdateButtonText` (0x001e4458): shows or hides the message-center buttons from the message count and the first row.
/// The body keeps the generated block structure: every register, cycle and call is as generated.
// Not installed: its real callees run into the allocator on test data and
// no override slot exists in this module to fake them (see the wave report).
#[allow(dead_code)]
pub fn ov_screen_message_center_update_button_text(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e4568: {
            'b_1e4564: {
                'b_1e4550: {
                    'b_1e4534: {
                        'b_1e44b0: {
                            c.cycles += 12;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[S1][0] = c.g[A0][0];
                            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x144));
                            c.g[RA][0] = sx(0x001e4478);
                            c.g[S0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
                            Callee {
                                addr: 0x001e33e8,
                                f: ee::s04::MessageCenter_GetMessageCount,
                            }
                            .call(c, 0x001e4478)?;
                            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[S0][0] as u32)));
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[V0][0] as i64) < 0;
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14c));
                            if t {
                                break 'b_1e44b0;
                            } else {
                                c.cycles += 8;
                                c.g[RA][0] = sx(0x001e4490);
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x001e4878,
                                    f: ee::s04::OVScreenMessageCenter_ShowWidget,
                                }
                                .call(c, 0x001e4490)?;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x150));
                                c.g[RA][0] = sx(0x001e449c);
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x001e4878,
                                    f: ee::s04::OVScreenMessageCenter_ShowWidget,
                                }
                                .call(c, 0x001e449c)?;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x158));
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = 0x470000;
                                if t {
                                    break 'b_1e4564;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fa0));
                                    if t {
                                        break 'b_1e4534;
                                    } else {
                                        break 'b_1e44b0;
                                    }
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[RA][0] = sx(0x001e44b8);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x001e4840,
                            f: ee::s04::OVScreenMessageCenter_HideWidget,
                        }
                        .call(c, 0x001e44b8)?;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x150));
                        c.g[RA][0] = sx(0x001e44c4);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x001e4840,
                            f: ee::s04::OVScreenMessageCenter_HideWidget,
                        }
                        .call(c, 0x001e44c4)?;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x158));
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = 0x4;
                        if t {
                            break 'b_1e4564;
                        } else {
                            c.cycles += 17;
                            c.g[A1][0] = sx(0u32.wrapping_sub((c.g[S0][0] as u32)));
                            c.g[RA][0] = sx(0x001e44dc);
                            c.g[A2][0] = 0x1;
                            Callee {
                                addr: 0x001e2c80,
                                f: ee::s04::MessageCenter_GetGroupMessageId,
                            }
                            .call(c, 0x001e44dc)?;
                            c.g[A2][0] = 0x18;
                            c.g[V1][0] = 0x4c0000;
                            c.mult(0, 6, 2, 6);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6c08));
                            c.g[A0][0] = 0x14;
                            c.g[A1][0] = 0x440000;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1630));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.g[A2][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                            c.mult(0, 4, 6, 4);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                            let t = c.g[V0][0] != c.g[V1][0];
                            c.g[A0][0] = 0x470000;
                            if t {
                                break 'b_1e4550;
                            } else {
                                'b_1e452c: {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001e451c);
                                    c.g[A0][0] = c.g[A2][0];
                                    Callee {
                                        addr: 0x001e3a30,
                                        f: ee::s04::MessageCenter_IsFlagSet,
                                    }
                                    .call(c, 0x001e451c)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] = 0x470000;
                                    if t {
                                        break 'b_1e452c;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1f90));
                                        if t {
                                            break 'b_1e4534;
                                        } else {
                                            break 'b_1e452c;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.g[A0][0] = 0x470000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1f80));
                                break 'b_1e4534;
                            }
                        }
                    }
                    c.cycles += 7;
                    call_ptr(c, 0x00317670, 0x001e453c)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x158));
                    c.g[RA][0] = sx(0x001e4548);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001e4548)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_1e4568;
                    } else {
                        break 'b_1e4550;
                    }
                }
                c.cycles += 5;
                c.g[RA][0] = sx(0x001e4558);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fa0));
                call_ptr(c, 0x00317670, 0x001e4558)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x158));
                c.g[RA][0] = sx(0x001e4564);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x001e4564)?;
                break 'b_1e4564;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1e4568;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenMessageCenter_UpdateScrollArrows` (0x001e4658): shows the scroll arrows from the list position, through the arrow widget's vtable.
/// The body keeps the generated block structure: every register, cycle and call is as generated.
// Not installed: its real callees run into the allocator on test data and
// no override slot exists in this module to fake them (see the wave report).
#[allow(dead_code)]
pub fn ov_screen_message_center_update_scroll_arrows(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e46b4: {
            'b_1e4698: {
                c.cycles += 8;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x174));
                let t = c.g[V0][0] != 0u64;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x15c));
                if t {
                    break 'b_1e4698;
                } else {
                    c.cycles += 8;
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001e4690);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001e4690)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x102c));
                    if t {
                        break 'b_1e46b4;
                    } else {
                        break 'b_1e4698;
                    }
                }
            }
            c.cycles += 7;
            c.g[A1][0] = 0x1;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001e46b0);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x001e46b0)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x102c));
            break 'b_1e46b4;
        }
        'b_1e46e8: {
            'b_1e46d4: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;

                if t {
                    break 'b_1e46d4;
                } else {
                    c.cycles += 6;
                    Callee {
                        addr: 0x001e33e8,
                        f: ee::s04::MessageCenter_GetMessageCount,
                    }
                    .call(c, 0x001e46c4)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = 0x4;
                    let t = true;
                    c.g[A1][0] = 0x1;
                    if t {
                        break 'b_1e46e8;
                    } else {
                        break 'b_1e46d4;
                    }
                }
            }
            c.cycles += 5;
            Callee {
                addr: 0x001e33e8,
                f: ee::s04::MessageCenter_GetMessageCount,
            }
            .call(c, 0x001e46dc)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 0x4;
            c.g[A1][0] = 0u64;
            break 'b_1e46e8;
        }
        'b_1e4744: {
            'b_1e4728: {
                c.cycles += 8;
                Callee {
                    addr: 0x001e2bb8,
                    f: ee::s04::MessageCenter_CountGroupMessages,
                }
                .call(c, 0x001e46f0)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x174));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x160));
                if t {
                    break 'b_1e4728;
                } else {
                    c.cycles += 8;
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001e4720);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001e4720)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_1e4744;
                    } else {
                        break 'b_1e4728;
                    }
                }
            }
            c.cycles += 7;
            c.g[A1][0] = 0x1;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001e4740);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x001e4740)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1e4744;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenMessageCenter_GetGroupVariant` (0x001e4b80): the variant of a message group, from the game table, the global flag and the message index.
/// The body keeps the generated block structure: every register, cycle and call is as generated.
pub fn ov_screen_message_center_get_group_variant(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e4c68: {
            'b_1e4ba4: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x102c));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A2][0] = c.g[A1][0];
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                let t = c.g[V0][0] != 0u64;
                c.g[A3][0] = 0xff;
                if t {
                    break 'b_1e4ba4;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0xff;
                    if t {
                        break 'b_1e4c68;
                    } else {
                        break 'b_1e4ba4;
                    }
                }
            }
            c.cycles += 9;
            c.g[A0][0] = 0x18;
            c.g[V0][0] = 0x4c0000;
            c.mult(0, 4, 6, 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6c08));
            c.g[A1][0] = 0x1;
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            let t = c.g[V1][0] != c.g[A1][0];
            c.g[V0][0] = c.g[A3][0];
            if t {
                break 'b_1e4c68;
            } else {
                'b_1e4c54: {
                    'b_1e4c50: {
                        'b_1e4c4c: {
                            c.cycles += 13;
                            c.g[RA][0] = sx(0x001e4bd0);
                            c.g[A0][0] = c.g[A2][0];
                            Callee {
                                addr: 0x001e2dc0,
                                f: ee::s04::MessageCenter_GetIndexInType,
                            }
                            .call(c, 0x001e4bd0)?;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = 0x14;
                            c.g[A0][0] = 0x440000;
                            c.g[A2][0] = c.g[V0][0];
                            c.mult(0, 3, 3, 5);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1630));
                            c.g[V0][0] = 0x4;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                            let t = c.g[A0][0] == c.g[V0][0];
                            c.g[A1][0] = 0x1;
                            if t {
                                break 'b_1e4c4c;
                            } else {
                                'b_1e4c44: {
                                    'b_1e4c34: {
                                        'b_1e4c18: {
                                            c.cycles += 3;
                                            c.g[V0][0] = ((c.g[A0][0] as i64) < 5i64) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x3;
                                            if t {
                                                break 'b_1e4c18;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A0][0] == c.g[V0][0];
                                                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 31);
                                                if t {
                                                    break 'b_1e4c34;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.div(0, 6, 5);
                                                    if t {
                                                        break 'b_1e4c54;
                                                    } else {
                                                        break 'b_1e4c18;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x5;
                                        let t = c.g[A0][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x8;
                                        if t {
                                            break 'b_1e4c44;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[A0][0] == c.g[V0][0] {
                                                c.g[A1][0] = 0x4;
                                                break 'b_1e4c50;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.div(0, 6, 5);
                                                if t {
                                                    break 'b_1e4c54;
                                                } else {
                                                    break 'b_1e4c34;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A1][0] = 0xa;
                                    c.g[V0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    let t = true;
                                    c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                    if t {
                                        break 'b_1e4c50;
                                    } else {
                                        break 'b_1e4c44;
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.g[A1][0] = 0xb;
                                if t {
                                    break 'b_1e4c50;
                                } else {
                                    break 'b_1e4c4c;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A1][0] = 0xa;
                        break 'b_1e4c50;
                    }
                    c.cycles += 1;
                    c.div(0, 6, 5);
                    break 'b_1e4c54;
                }
                'b_1e4c5c: {
                    c.cycles += 2;
                    if c.g[A1][0] == 0u64 {
                        break 'b_1e4c5c;
                    } else {
                        break 'b_1e4c5c;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = c.hi[0];
                c.g[A3][0] = c.g[V0][0] & 0xff;
                c.g[V0][0] = c.g[A3][0];
                break 'b_1e4c68;
            }
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `cOVState_ViewPopup_Update` (0x001e52b0): one call to the popup update.
/// The body keeps the generated block structure: every register, cycle and call is as generated.
pub fn c_ov_state_view_popup_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        call_ptr(c, 0x0039e510, 0x001e52c0)?;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `cOVState_ViewPopup_OnWidgetCreated` (0x001e52f8): binds the created widgets to the popup fields, each through a vtable call.
/// The body keeps the generated block structure: every register, cycle and call is as generated.
pub fn c_ov_state_view_popup_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e5414: {
            'b_1e5410: {
                'b_1e534c: {
                    c.cycles += 19;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A1][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S2][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001e5330);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001e5330)?;
                    c.g[A0][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x001e533c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2120));
                    call_ptr(c, 0x00317670, 0x001e533c)?;
                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        break 'b_1e534c;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x50), c.g[S0][0]);
                        if t {
                            break 'b_1e5410;
                        } else {
                            break 'b_1e534c;
                        }
                    }
                }
                'b_1e5364: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e5354);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2128));
                    call_ptr(c, 0x00317670, 0x001e5354)?;
                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x470000;
                        break 'b_1e5364;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x54), c.g[S0][0]);
                        if t {
                            break 'b_1e5410;
                        } else {
                            break 'b_1e5364;
                        }
                    }
                }
                'b_1e537c: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e536c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1d98));
                    call_ptr(c, 0x00317670, 0x001e536c)?;
                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        break 'b_1e537c;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x48), c.g[S0][0]);
                        if t {
                            break 'b_1e5410;
                        } else {
                            break 'b_1e537c;
                        }
                    }
                }
                'b_1e5394: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e5384);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2130));
                    call_ptr(c, 0x00317670, 0x001e5384)?;
                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x470000;
                        break 'b_1e5394;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x4c), c.g[S0][0]);
                        if t {
                            break 'b_1e5410;
                        } else {
                            break 'b_1e5394;
                        }
                    }
                }
                'b_1e53ac: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e539c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1d88));
                    call_ptr(c, 0x00317670, 0x001e539c)?;
                    let t = c.g[S1][0] != c.g[V0][0];

                    if t {
                        break 'b_1e53ac;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x64), c.g[S0][0]);
                        if t {
                            break 'b_1e5410;
                        } else {
                            break 'b_1e53ac;
                        }
                    }
                }
                'b_1e53c4: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e53b4);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1048));
                    call_ptr(c, 0x00317670, 0x001e53b4)?;
                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x470000;
                        break 'b_1e53c4;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x5c), c.g[S0][0]);
                        if t {
                            break 'b_1e5410;
                        } else {
                            break 'b_1e53c4;
                        }
                    }
                }
                'b_1e53dc: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e53cc);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1d78));
                    call_ptr(c, 0x00317670, 0x001e53cc)?;
                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x470000;
                        break 'b_1e53dc;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x60), c.g[S0][0]);
                        if t {
                            break 'b_1e5410;
                        } else {
                            break 'b_1e53dc;
                        }
                    }
                }
                'b_1e53f4: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e53e4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1d60));
                    call_ptr(c, 0x00317670, 0x001e53e4)?;
                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x470000;
                        break 'b_1e53f4;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x58), c.g[S0][0]);
                        if t {
                            break 'b_1e5410;
                        } else {
                            break 'b_1e53f4;
                        }
                    }
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x001e53fc);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1d50));
                call_ptr(c, 0x00317670, 0x001e53fc)?;
                if c.g[S1][0] != c.g[V0][0] {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    break 'b_1e5414;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x90));
                    c.g[V0][0] = c.g[V0][0] | 0x8;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x90), c.g[V0][0]);
                    break 'b_1e5410;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1e5414;
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

/// `cOVState_ViewPopup_ShowMessage` (0x001e5428): fills the popup with a message's subject, body and sender.
/// The body keeps the generated block structure: every register, cycle and call is as generated.
pub fn c_ov_state_view_popup_show_message(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e54e8: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
            c.g[V0][0] = 0x18;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[V1][0] = 0x4c0000;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6c08));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[A3][0] = 0x14;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[A0][0] = 0x440000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1630));
            c.g[A2][0] = 0x4;
            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
            c.mult(0, 5, 5, 2);
            c.g[S3][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
            c.mult(0, 2, 2, 7);
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[V1][0] = c.lw((c.g[S0][0] as u32));
            if c.g[V1][0] != c.g[A2][0] {
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                break 'b_1e54e8;
            } else {
                'b_1e54a4: {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_1e54a4;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001e54a4);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001e54a4)?;
                        break 'b_1e54a4;
                    }
                }
                'b_1e54c4: {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x60));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_1e54c4;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001e54c4);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001e54c4)?;
                        break 'b_1e54c4;
                    }
                }
                'b_1e54e4: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x64));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1e54e4;
                    } else {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x001e54d8);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1d40));
                        call_ptr(c, 0x00317670, 0x001e54d8)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x64));
                        c.g[RA][0] = sx(0x001e54e4);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x001e54e4)?;
                        break 'b_1e54e4;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                break 'b_1e54e8;
            }
        }
        'b_1e557c: {
            'b_1e5578: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x4;
                if t {
                    break 'b_1e5578;
                } else {
                    'b_1e555c: {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[A1][0] = 0x4a0000;
                            break 'b_1e555c;
                        } else {
                            c.cycles += 24;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A0][0] = 0x470000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1d30));
                            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                            c.g[RA][0] = sx(0x001e551c);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            call_ptr(c, 0x00317670, 0x001e551c)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                            c.g[A1][0] = c.g[V0][0];
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001e552c);
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, t, 0x001e552c)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                            c.g[RA][0] = sx(0x001e5538);
                            c.g[S0][0] = c.g[V0][0];
                            Callee {
                                addr: 0x001e2dc0,
                                f: ee::s04::MessageCenter_GetIndexInType,
                            }
                            .call(c, 0x001e5538)?;
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A0][0] = c.g[SP][0];
                            c.g[RA][0] = sx(0x001e5548);
                            c.g[A2][0] = c.g[V0][0];
                            call_ptr(c, 0x002c26d0, 0x001e5548)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                            c.g[RA][0] = sx(0x001e5554);
                            c.g[A1][0] = c.g[SP][0];
                            call_ptr(c, 0x003a0e90, 0x001e5554)?;
                            let t = true;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                            if t {
                                break 'b_1e557c;
                            } else {
                                break 'b_1e555c;
                            }
                        }
                    }
                    c.cycles += 7;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6c));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2138));
                    c.g[RA][0] = sx(0x001e556c);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00417828, 0x001e556c)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                    c.g[RA][0] = sx(0x001e5578);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x003a0c50, 0x001e5578)?;
                    break 'b_1e5578;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
            break 'b_1e557c;
        }
        'b_1e559c: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                break 'b_1e559c;
            } else {
                c.cycles += 6;
                c.g[RA][0] = sx(0x001e558c);
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                Callee {
                    addr: 0x001e2ea0,
                    f: ee::s04::MessageCenter_GetSenderNameHash,
                }
                .call(c, 0x001e558c)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                c.g[RA][0] = sx(0x001e5598);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x001e5598)?;
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                break 'b_1e559c;
            }
        }
        c.cycles += 12;
        c.g[RA][0] = sx(0x001e55a4);
        c.g[A0][0] = c.g[S2][0];
        Callee {
            addr: 0x001e55d0,
            f: ee::s04::cOVState_ViewPopup_SetSubject,
        }
        .call(c, 0x001e55a4)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001e55b0);
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x68));
        Callee {
            addr: 0x001e5780,
            f: ee::s04::cOVState_ViewPopup_SetBody,
        }
        .call(c, 0x001e55b0)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenFreestyleStandings_Update` (0x001e64a8): one call to the standings update.
/// The body keeps the generated block structure: every register, cycle and call is as generated.
// Not installed: its real callees run into the allocator on test data and
// no override slot exists in this module to fake them (see the wave report).
#[allow(dead_code)]
pub fn ov_screen_freestyle_standings_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        call_ptr(c, 0x0020e900, 0x001e64b8)?;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, machine_rt};
    use crate::difftest::{check, elf_image, Rng, DATA};

    const OC968_SELF: u32 = DATA + 0x1000;
    const OC968_SUB: u32 = DATA + 0x3000;
    const OC968_VT: u32 = DATA + 0x4000;
    const OC968_WIDGET: u32 = DATA + 0x5000;
    const OC968_OUT: u32 = DATA + 0x6000;
    const OC968_FAKE: u32 = 0x0060_0000;

    /// The message-center screen: an inbox sub-object at +0x144 (first-row
    /// byte at +0x95), eight widgets at +0xfc (each with a vtable whose slots
    /// 0x48/0x4c are fake call targets), and the flags the functions read.
    fn oc968_screen(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let s = OC968_SELF;
        c.bus.write32(s + 0x144, OC968_SUB);
        c.bus.write8(OC968_SUB + 0x95, (r.next_u32() % 4) as u8);
        c.bus.write32(s + 0x10, OC968_SUB + 0x100);
        c.bus.write32(s + 0x17c, OC968_SUB + 0x200);
        c.bus.write32(s + 0x170, seed % 2);
        c.bus.write32(s + 0x174, seed % 3);
        c.bus.write32(s + 0x18, seed % 3);
        c.bus.write32(OC968_VT + 0x28, OC968_FAKE);
        c.bus.write32(OC968_VT + 0x2c, OC968_FAKE + 0x10);
        c.bus.write32(OC968_VT + 0x48, OC968_FAKE + 0x20);
        c.bus.write32(OC968_VT + 0x4c, OC968_FAKE + 0x30);
        for i in 0..8u32 {
            let w = OC968_WIDGET + 0x100 * i;
            c.bus.write32(s + 0xfc + 4 * i, w);
            c.bus.write32(w + 8, OC968_VT);
        }
    }

    /// A word in the game's gp-relative globals.
    fn oc968_gp_word(c: &mut Cpu, off: u32, v: u32) {
        let at = (c.g[GP][0] as u32).wrapping_sub(off);
        c.bus.write32(at, v);
    }

    /// OVScreenMessageCenter_Init: the screen built from scratch, with the

    /// OVScreenMessageCenter_UpdateButtonText: the buttons from the message

    /// OVScreenMessageCenter_UpdateScrollArrows: the arrows from the list

    /// OVScreenMessageCenter_GetGroupVariant: the variant of a message group
    /// from the game table, the global flag and the message index.
    #[test]
    fn ov_screen_message_center_get_group_variant_matches() {
        check(
            ee::s04::f_001e4b80__generated,
            ov_screen_message_center_get_group_variant,
            192,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x1e4b80, 0);
                elf_image(c);
                oc968_gp_word(c, 0x102c, if seed % 2 == 0 { 0 } else { OC968_OUT });
                c.g[A0][0] = (r.next_u32() % 4) as u64;
                c.g[A1][0] = (seed % 12) as u64;
                c.pc = 0x001e_4b80;
            },
        );
    }

    /// cOVState_ViewPopup_Update: one call to the popup's update.
    #[test]
    fn c_ov_state_view_popup_update_matches() {
        check(
            ee::s04::f_001e52b0__generated,
            c_ov_state_view_popup_update,
            96,
            &|c, seed| {
                machine_rt(c, seed, 0x1e52b0, 0);
                elf_image(c);
                c.pc = 0x001e_52b0;
            },
        );
    }

    /// cOVState_ViewPopup_OnWidgetCreated: binds the created widgets to the
    /// popup's fields, each with a vtable call.
    #[test]
    fn c_ov_state_view_popup_on_widget_created_matches() {
        check(
            ee::s04::f_001e52f8__generated,
            c_ov_state_view_popup_on_widget_created,
            192,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x1e52f8, 0);
                elf_image(c);
                oc968_screen(c, &mut r, seed);
                let w = OC968_WIDGET + 0x100 * (seed % 8);
                c.bus
                    .write32(w + 0x38, if seed % 3 == 0 { 0 } else { OC968_SUB });
                c.bus.write32(OC968_SELF + 0x90, r.next_u32());
                c.pc = 0x001e_52f8;
                c.g[A0][0] = sx(OC968_SELF);
                c.g[A1][0] = sx(w);
            },
        );
    }

    /// cOVState_ViewPopup_ShowMessage: the message's subject, body and
    /// sender from the popup state and the game table.
    #[test]
    fn c_ov_state_view_popup_show_message_matches() {
        check(
            ee::s04::f_001e5428__generated,
            c_ov_state_view_popup_show_message,
            192,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x1e5428, 0);
                elf_image(c);
                oc968_screen(c, &mut r, seed);
                let s = OC968_SELF;
                let obj = |k: u32| OC968_WIDGET + 0x100 * k;
                c.bus
                    .write32(s + 0x4c, if seed % 4 == 0 { 0 } else { obj(1) });
                c.bus
                    .write32(s + 0x50, if seed % 5 == 0 { 0 } else { obj(2) });
                c.bus
                    .write32(s + 0x5c, if seed % 3 == 0 { 0 } else { obj(3) });
                c.bus
                    .write32(s + 0x60, if seed % 6 == 0 { 0 } else { obj(4) });
                c.bus.write32(s + 0x64, seed % 2);
                c.bus.write32(s + 0x68, seed % 7);
                c.bus.write32(s + 0x6c, seed % 5);
                c.bus.write32(s + 0x90, r.next_u32());
                c.pc = 0x001e_5428;
                c.g[A0][0] = sx(s);
            },
        );
    }
}
