//! Staging module oc863 (wave wave250): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc863`.

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
        0x001a_4e28,
        cfe_state_net_messenger_show_find_user_error,
        "cFEStateNetMessenger_ShowFindUserError",
    ),
    (
        0x001a_5010,
        cfe_state_net_messenger_v31,
        "cFEStateNetMessenger_v31",
    ),
    (
        0x001a_5128,
        cfe_state_net_messenger_update_scroll_bar,
        "cFEStateNetMessenger_UpdateScrollBar",
    ),
    (
        0x001a_52b0,
        cfe_state_net_messenger_find_next_pending,
        "cFEStateNetMessenger_FindNextPending",
    ),
];

/// `void cFEStateNetMessenger_v34(self, popup, int kind)` (0x1a49a0): the
/// messenger's popup-event handler (variant 1). Kind 15 opens the challenge
/// popup for the selected user, 16 forwards the event and reads or sends a
/// message, 17 toggles the selected user as a buddy (max-buddies popup on
/// error) and rebuilds the buddy and player lists, 18 does the same for
/// blocking a user (buddy list only); anything else is forwarded to
/// `NetFEState_OnChildEvent`.
#[cfg(test)] // not installed: its test runs away in unbounded real callees
#[allow(dead_code)]
pub fn cfe_state_net_messenger_v34(c: &mut Cpu) {
    guest(c, |c| {
        'b_1a4b10: {
            'b_1a4b04: {
                'b_1a4ab0: {
                    'b_1a4a54: {
                        'b_1a4a20: {
                            c.cycles += 8;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                            c.g[V0][0] = 0x10;
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[S0][0] = c.g[A0][0];
                            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                            let t = c.g[A2][0] == c.g[V0][0];
                            c.g[S1][0] = c.g[A1][0];
                            if t {
                                break 'b_1a4a20;
                            } else {
                                'b_1a49f8: {
                                    'b_1a49dc: {
                                        c.cycles += 3;
                                        c.g[V0][0] = (c.g[A2][0] < 0x11u64) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0xf;
                                        if t {
                                            break 'b_1a49dc;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[A2][0] == c.g[V0][0];
                                            c.g[A0][0] = c.g[S0][0];
                                            if t {
                                                break 'b_1a49f8;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                if t {
                                                    break 'b_1a4b04;
                                                } else {
                                                    break 'b_1a49dc;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x11;
                                    let t = c.g[A2][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x12;
                                    if t {
                                        break 'b_1a4a54;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[A2][0] == c.g[V0][0];
                                        c.g[A0][0] = c.g[S0][0];
                                        if t {
                                            break 'b_1a4ab0;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            if t {
                                                break 'b_1a4b04;
                                            } else {
                                                break 'b_1a49f8;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 10;
                                c.g[RA][0] = sx(0x001a4a00);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x001a98b0,
                                    f: ee::s03::NetFEState_GetSelectedUser,
                                }
                                .call(c, 0x001a4a00)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A2][0] = c.g[S1][0];
                                c.g[A3][0] = 0xf;
                                c.g[RA][0] = sx(0x001a4a18);
                                c.g[T0][0] = 0x1;
                                Callee {
                                    addr: 0x001abcd0,
                                    f: ee::s03::NetFEState_OpenChallengePopup,
                                }
                                .call(c, 0x001a4a18)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_1a4b10;
                                } else {
                                    break 'b_1a4a20;
                                }
                            }
                        }
                        c.cycles += 13;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = 0x10;
                        c.g[RA][0] = sx(0x001a4a30);
                        c.g[A0][0] = c.g[S0][0];
                        Callee {
                            addr: 0x001a97b8,
                            f: ee::s03::NetFEState_OnChildEvent,
                        }
                        .call(c, 0x001a4a30)?;
                        c.g[RA][0] = sx(0x001a4a38);
                        c.g[A0][0] = c.g[S0][0];
                        Callee {
                            addr: 0x001a98b0,
                            f: ee::s03::NetFEState_GetSelectedUser,
                        }
                        .call(c, 0x001a4a38)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A2][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x001a4a4c);
                        c.g[A3][0] = 0x10;
                        Callee {
                            addr: 0x001abb30,
                            f: ee::s03::NetFEState_ReadOrSendMessage,
                        }
                        .call(c, 0x001a4a4c)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_1a4b10;
                        } else {
                            break 'b_1a4a54;
                        }
                    }
                    'b_1a4a98: {
                        'b_1a4a8c: {
                            c.cycles += 10;
                            c.g[RA][0] = sx(0x001a4a5c);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001a98b0,
                                f: ee::s03::NetFEState_GetSelectedUser,
                            }
                            .call(c, 0x001a4a5c)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001a4a6c);
                            c.g[A2][0] = c.g[SP][0];
                            Callee {
                                addr: 0x001abe58,
                                f: ee::s03::NetFEState_ToggleMessengerBuddy,
                            }
                            .call(c, 0x001a4a6c)?;
                            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                            c.g[V0][0] = 0x1;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A1][0] = c.g[S1][0];
                            if t {
                                break 'b_1a4a8c;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001a4a84);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x001abed8,
                                    f: ee::s03::NetFEState_ShowMaxBuddiesPopup,
                                }
                                .call(c, 0x001a4a84)?;
                                let t = true;
                                if t {
                                    break 'b_1a4a98;
                                } else {
                                    break 'b_1a4a8c;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001a4a98);
                        c.g[A2][0] = 0x11;
                        Callee {
                            addr: 0x001a97b8,
                            f: ee::s03::NetFEState_OnChildEvent,
                        }
                        .call(c, 0x001a4a98)?;
                        break 'b_1a4a98;
                    }
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x001a4aa0);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001a5898,
                        f: ee::s03::cFEStateNetMessenger_BuildBuddyList,
                    }
                    .call(c, 0x001a4aa0)?;
                    c.g[RA][0] = sx(0x001a4aa8);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001a6978,
                        f: ee::s03::cFEStateNetMessenger_RefreshPlayerList,
                    }
                    .call(c, 0x001a4aa8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_1a4b10;
                    } else {
                        break 'b_1a4ab0;
                    }
                }
                'b_1a4af4: {
                    'b_1a4ae8: {
                        c.cycles += 10;
                        c.g[RA][0] = sx(0x001a4ab8);
                        c.g[A0][0] = c.g[S0][0];
                        Callee {
                            addr: 0x001a98b0,
                            f: ee::s03::NetFEState_GetSelectedUser,
                        }
                        .call(c, 0x001a4ab8)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001a4ac8);
                        c.g[A2][0] = c.g[SP][0] | 0x4;
                        Callee {
                            addr: 0x001abc50,
                            f: ee::s03::NetFEState_ToggleBlockUser,
                        }
                        .call(c, 0x001a4ac8)?;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = 0x1;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A1][0] = c.g[S1][0];
                        if t {
                            break 'b_1a4ae8;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001a4ae0);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001abed8,
                                f: ee::s03::NetFEState_ShowMaxBuddiesPopup,
                            }
                            .call(c, 0x001a4ae0)?;
                            let t = true;
                            if t {
                                break 'b_1a4af4;
                            } else {
                                break 'b_1a4ae8;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001a4af4);
                    c.g[A2][0] = 0x12;
                    Callee {
                        addr: 0x001a97b8,
                        f: ee::s03::NetFEState_OnChildEvent,
                    }
                    .call(c, 0x001a4af4)?;
                    break 'b_1a4af4;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a4afc);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001a5898,
                    f: ee::s03::cFEStateNetMessenger_BuildBuddyList,
                }
                .call(c, 0x001a4afc)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_1a4b10;
                } else {
                    break 'b_1a4b04;
                }
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x001a4b0c);
            c.g[A1][0] = c.g[S1][0];
            Callee {
                addr: 0x001a97b8,
                f: ee::s03::NetFEState_OnChildEvent,
            }
            .call(c, 0x001a4b0c)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1a4b10;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateNetMessenger_v35(self, popup, int kind)` (0x1a4b20): the
/// messenger's popup-event handler (variant 2). Kind 15 forwards the event and
/// reads or sends a message, 16 toggles the selected user as a buddy, 17
/// toggles the block state (both with the max-buddies popup on error, then
/// rebuild the buddy and player lists); anything else is forwarded to
/// `NetFEState_OnChildEvent`.
#[cfg(test)] // not installed: its test runs away in unbounded real callees
#[allow(dead_code)]
pub fn cfe_state_net_messenger_v35(c: &mut Cpu) {
    guest(c, |c| {
        'b_1a4c44: {
            'b_1a4c38: {
                'b_1a4c20: {
                    'b_1a4c18: {
                        'b_1a4c10: {
                            'b_1a4c00: {
                                'b_1a4bd8: {
                                    'b_1a4ba4: {
                                        c.cycles += 8;
                                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                                        c.g[V0][0] = 0x10;
                                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.g[S0][0] = c.g[A0][0];
                                        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                                        let t = c.g[A2][0] == c.g[V0][0];
                                        c.g[S1][0] = c.g[A1][0];
                                        if t {
                                            break 'b_1a4ba4;
                                        } else {
                                            'b_1a4b70: {
                                                'b_1a4b5c: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = (c.g[A2][0] < 0x11u64) as u64;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0xf;
                                                    if t {
                                                        break 'b_1a4b5c;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[A2][0] == c.g[V0][0];
                                                        c.g[A0][0] = c.g[S0][0];
                                                        if t {
                                                            break 'b_1a4b70;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            if t {
                                                                break 'b_1a4c38;
                                                            } else {
                                                                break 'b_1a4b5c;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x11;
                                                let t = c.g[A2][0] == c.g[V0][0];
                                                c.g[A0][0] = c.g[S0][0];
                                                if t {
                                                    break 'b_1a4bd8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    if t {
                                                        break 'b_1a4c38;
                                                    } else {
                                                        break 'b_1a4b70;
                                                    }
                                                }
                                            }
                                            c.cycles += 13;
                                            c.g[A1][0] = c.g[S1][0];
                                            c.g[A2][0] = 0xf;
                                            c.g[RA][0] = sx(0x001a4b80);
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x001a97b8,
                                                f: ee::s03::NetFEState_OnChildEvent,
                                            }
                                            .call(c, 0x001a4b80)?;
                                            c.g[RA][0] = sx(0x001a4b88);
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x001a98b0,
                                                f: ee::s03::NetFEState_GetSelectedUser,
                                            }
                                            .call(c, 0x001a4b88)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[RA][0] = sx(0x001a4b9c);
                                            c.g[A3][0] = 0xf;
                                            Callee {
                                                addr: 0x001abb30,
                                                f: ee::s03::NetFEState_ReadOrSendMessage,
                                            }
                                            .call(c, 0x001a4b9c)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            if t {
                                                break 'b_1a4c44;
                                            } else {
                                                break 'b_1a4ba4;
                                            }
                                        }
                                    }
                                    c.cycles += 10;
                                    c.g[RA][0] = sx(0x001a4bac);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x001a98b0,
                                        f: ee::s03::NetFEState_GetSelectedUser,
                                    }
                                    .call(c, 0x001a4bac)?;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x001a4bbc);
                                    c.g[A2][0] = c.g[SP][0];
                                    Callee {
                                        addr: 0x001abe58,
                                        f: ee::s03::NetFEState_ToggleMessengerBuddy,
                                    }
                                    .call(c, 0x001a4bbc)?;
                                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                                    c.g[V0][0] = 0x1;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[A1][0] = c.g[S1][0];
                                    if t {
                                        break 'b_1a4c00;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.g[S0][0];
                                        let t = true;
                                        c.g[A2][0] = 0x10;
                                        if t {
                                            break 'b_1a4c18;
                                        } else {
                                            break 'b_1a4bd8;
                                        }
                                    }
                                }
                                c.cycles += 10;
                                c.g[RA][0] = sx(0x001a4be0);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x001a98b0,
                                    f: ee::s03::NetFEState_GetSelectedUser,
                                }
                                .call(c, 0x001a4be0)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x001a4bf0);
                                c.g[A2][0] = c.g[SP][0] | 0x4;
                                Callee {
                                    addr: 0x001abc50,
                                    f: ee::s03::NetFEState_ToggleBlockUser,
                                }
                                .call(c, 0x001a4bf0)?;
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = 0x1;
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A1][0] = c.g[S1][0];
                                if t {
                                    break 'b_1a4c10;
                                } else {
                                    break 'b_1a4c00;
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001a4c08);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001abed8,
                                f: ee::s03::NetFEState_ShowMaxBuddiesPopup,
                            }
                            .call(c, 0x001a4c08)?;
                            let t = true;
                            if t {
                                break 'b_1a4c20;
                            } else {
                                break 'b_1a4c10;
                            }
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A2][0] = 0x11;
                        break 'b_1a4c18;
                    }
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x001a4c20);
                    Callee {
                        addr: 0x001a97b8,
                        f: ee::s03::NetFEState_OnChildEvent,
                    }
                    .call(c, 0x001a4c20)?;
                    break 'b_1a4c20;
                }
                c.cycles += 6;
                c.g[RA][0] = sx(0x001a4c28);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001a5898,
                    f: ee::s03::cFEStateNetMessenger_BuildBuddyList,
                }
                .call(c, 0x001a4c28)?;
                c.g[RA][0] = sx(0x001a4c30);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001a6978,
                    f: ee::s03::cFEStateNetMessenger_RefreshPlayerList,
                }
                .call(c, 0x001a4c30)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_1a4c44;
                } else {
                    break 'b_1a4c38;
                }
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x001a4c40);
            c.g[A1][0] = c.g[S1][0];
            Callee {
                addr: 0x001a97b8,
                f: ee::s03::NetFEState_OnChildEvent,
            }
            .call(c, 0x001a4c40)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1a4c44;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateNetMessenger_ShowFindUserError(self, online_ErrorEvent* msg)`
/// (0x1a4e28): shows the find-user error popup. Error `'iper'` (0x69706572)
/// formats the "invalid name" text with the searched name into the message
/// buffer (`self + 0x9c`) and shows a "user not found" OK popup (misc data
/// 0x80a); error `'inam'` (0x696e616d) shows the "persona name length" OK popup
/// (misc data 0x806); any other error goes to `NetFEState_HandleNetEvent`.
pub fn cfe_state_net_messenger_show_find_user_error(c: &mut Cpu) {
    guest(c, |c| {
        'b_1a4f8c: {
            'b_1a4f80: {
                'b_1a4f68: {
                    'b_1a4f14: {
                        c.cycles += 14;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                        c.g[A2][0] = c.g[A1][0];
                        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = 0x696e0000;
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.g[V0][0] = c.g[V0][0] | 0x616d;
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[S5][0] = c.g[A0][0];
                        if t {
                            break 'b_1a4f14;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = 0x69700000;
                            c.g[V0][0] = c.g[V0][0] | 0x6572;
                            if c.g[V1][0] != c.g[V0][0] {
                                c.g[A1][0] = c.lw(c.g[A2][0] as u32);
                                break 'b_1a4f80;
                            } else {
                                c.cycles += 41;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A0][0] = 0x460000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1678));
                                c.g[S2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x9c));
                                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                c.g[S3][0] = 0x1;
                                c.g[S4][0] = 0x3;
                                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                c.g[RA][0] = sx(0x001a4e9c);
                                c.g[S0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                call_ptr(c, 0x00317670, 0x001a4e9c)?;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                c.g[A0][0] = c.g[S0][0];
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001a4eac);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, t, 0x001a4eac)?;
                                c.g[A2][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x6d4));
                                c.g[A1][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001a4ebc);
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x002c26d0, 0x001a4ebc)?;
                                c.g[RA][0] = sx(0x001a4ec4);
                                c.g[A0][0] = c.g[S5][0];
                                Callee {
                                    addr: 0x001a9c50,
                                    f: ee::s03::NetFEState_GetPopup,
                                }
                                .call(c, 0x001a4ec4)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A0][0] = 0x460000;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[S3][0]);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1690));
                                c.g[RA][0] = sx(0x001a4edc);
                                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                call_ptr(c, 0x00317670, 0x001a4edc)?;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[S2][0]);
                                c.g[A0][0] = 0x460000;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[S3][0]);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc8), c.g[S4][0]);
                                c.g[RA][0] = sx(0x001a4ef8);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xcc), c.g[V0][0]);
                                call_ptr(c, 0x00317670, 0x001a4ef8)?;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A2][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001a4f08);
                                c.g[A1][0] = 0u64;
                                Callee {
                                    addr: 0x001c57c0,
                                    f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                }
                                .call(c, 0x001a4f08)?;
                                c.g[V0][0] = 0x80a;
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x14c), c.g[S4][0]);
                                if t {
                                    break 'b_1a4f68;
                                } else {
                                    break 'b_1a4f14;
                                }
                            }
                        }
                    }
                    c.cycles += 21;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x001a4f20);
                    c.g[S2][0] = 0x3;
                    Callee {
                        addr: 0x001a9c50,
                        f: ee::s03::NetFEState_GetPopup,
                    }
                    .call(c, 0x001a4f20)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    c.g[V0][0] = 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x15f8));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                    c.g[RA][0] = sx(0x001a4f3c);
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                    call_ptr(c, 0x00317670, 0x001a4f3c)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[S2][0]);
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
                    c.g[RA][0] = sx(0x001a4f50);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                    call_ptr(c, 0x00317670, 0x001a4f50)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a4f60);
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x001c57c0,
                        f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                    }
                    .call(c, 0x001a4f60)?;
                    c.g[V0][0] = 0x806;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14c), c.g[S2][0]);
                    break 'b_1a4f68;
                }
                c.cycles += 6;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                c.g[A0][0] = c.g[S5][0];
                c.g[RA][0] = sx(0x001a4f78);
                c.g[A1][0] = c.g[S0][0];
                Callee {
                    addr: 0x001a9bc0,
                    f: ee::s03::NetFEState_ShowPopup,
                }
                .call(c, 0x001a4f78)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                if t {
                    break 'b_1a4f8c;
                } else {
                    break 'b_1a4f80;
                }
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x001a4f88);
            c.g[A0][0] = c.g[S5][0];
            Callee {
                addr: 0x001a8e40,
                f: ee::s03::NetFEState_HandleNetEvent,
            }
            .call(c, 0x001a4f88)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            break 'b_1a4f8c;
        }
        c.cycles += 8;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `void* cFEStateNetMessenger_v31(self, int eventType, int eventData)`
/// (0x1a5010): the messenger's net-event handler. Event types 0xe3..0x113
/// dispatch through the jump table at 0x4616b0 (buddy-list refreshes, the
/// looking-for-self and user-found popups, the find-user result); everything
/// else refreshes the buddy-action popup and goes to `NetFEState_HandleNetEvent`.
/// Returns 0 on the table paths. The resumable entries stay generated
/// (`__entries`): any `pc` other than the entry runs them.
pub fn cfe_state_net_messenger_v31(c: &mut Cpu) {
    if c.pc != 0x001a5010 {
        return ee::s03::f_001a5010__entries(c);
    }
    guest(c, |c| 'l_1a5010: loop {
        'b_1a5114: {
            'b_1a5110: {
                'b_1a50e0: {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xe3));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V0][0] = (c.g[V1][0] < 0x31u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S2][0] = c.g[A2][0];
                    if t {
                        break 'b_1a50e0;
                    } else {
                        'b_1a50c8: {
                            'b_1a50b4: {
                                'b_1a50a4: {
                                    'b_1a5060: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x460000;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x16b0));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x001a5010 => {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x001a5010);
                                                }
                                                continue 'l_1a5010;
                                            }
                                            0x001a5058 => {
                                                c.cycles += 2;
                                                c.g[RA][0] = sx(0x001a5060);
                                                c.g[A0][0] = c.g[S0][0];
                                                Callee {
                                                    addr: 0x001a5898,
                                                    f: ee::s03::cFEStateNetMessenger_BuildBuddyList,
                                                }
                                                .call(c, 0x001a5060)?;
                                                break 'b_1a5060;
                                            }
                                            0x001a5060 => {
                                                break 'b_1a5060;
                                            }
                                            0x001a50a4 => {
                                                break 'b_1a50a4;
                                            }
                                            0x001a50b4 => {
                                                break 'b_1a50b4;
                                            }
                                            0x001a50c8 => {
                                                break 'b_1a50c8;
                                            }
                                            0x001a50e0 => {
                                                break 'b_1a50e0;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x001a5010, 0x001a5128);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6ac));
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        break 'b_1a5110;
                                    } else {
                                        c.cycles += 14;
                                        c.g[RA][0] = sx(0x001a5074);
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x001a54d8,
                                            f: ee::s03::cFEStateNetMessenger_AddNewBuddies,
                                        }
                                        .call(c, 0x001a5074)?;
                                        c.g[RA][0] = sx(0x001a507c);
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x001a52b0,
                                            f: ee::s03::cFEStateNetMessenger_FindNextPending,
                                        }
                                        .call(c, 0x001a507c)?;
                                        c.g[RA][0] = sx(0x001a5084);
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x001a6978,
                                            f: ee::s03::cFEStateNetMessenger_RefreshPlayerList,
                                        }
                                        .call(c, 0x001a5084)?;
                                        c.g[RA][0] = sx(0x001a508c);
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x001a98b0,
                                            f: ee::s03::NetFEState_GetSelectedUser,
                                        }
                                        .call(c, 0x001a508c)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001a509c);
                                        c.g[A2][0] = 0u64;
                                        Callee {
                                            addr: 0x001ab3d0,
                                            f: ee::s03::NetFEState_RefreshBuddyActionPopup,
                                        }
                                        .call(c, 0x001a509c)?;
                                        let t = true;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_1a5110;
                                        } else {
                                            break 'b_1a50a4;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001a50ac);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x001a4d90,
                                    f: ee::s03::cFEStateNetMessenger_ShowLookingForSelf,
                                }
                                .call(c, 0x001a50ac)?;
                                let t = true;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_1a5110;
                                } else {
                                    break 'b_1a50b4;
                                }
                            }
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001a50c0);
                            c.g[A1][0] = c.g[S2][0];
                            Callee {
                                addr: 0x001a4ce8,
                                f: ee::s03::cFEStateNetMessenger_ShowUserFound,
                            }
                            .call(c, 0x001a50c0)?;
                            let t = true;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_1a5110;
                            } else {
                                break 'b_1a50c8;
                            }
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x001a50d8);
                        c.g[A2][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001a4fb0,
                            f: ee::s03::cFEStateNetMessenger_OnFindUserResult,
                        }
                        .call(c, 0x001a50d8)?;
                        let t = true;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_1a5110;
                        } else {
                            break 'b_1a50e0;
                        }
                    }
                }
                c.cycles += 12;
                c.g[RA][0] = sx(0x001a50e8);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001a98b0,
                    f: ee::s03::NetFEState_GetSelectedUser,
                }
                .call(c, 0x001a50e8)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001a50f8);
                c.g[A2][0] = 0u64;
                Callee {
                    addr: 0x001ab3d0,
                    f: ee::s03::NetFEState_RefreshBuddyActionPopup,
                }
                .call(c, 0x001a50f8)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x001a5108);
                c.g[A2][0] = c.g[S2][0];
                Callee {
                    addr: 0x001a8e40,
                    f: ee::s03::NetFEState_HandleNetEvent,
                }
                .call(c, 0x001a5108)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_1a5114;
                } else {
                    break 'b_1a5110;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1a5114;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateNetMessenger_UpdateScrollBar(self, int pos, int count)`
/// (0x1a5128): moves the scroll thumb (`ScrollButton`) along its track
/// (`ScrollVector`) of the player list screen (`self + 0x40`): thumb.y =
/// track.y + pos * (track height - 15) / (count - 1 if count >= 2), thumb.x =
/// track.x - 2, and shows both. Returns early without a screen, with `pos < 0`,
/// `count < pos` or `count <= 0`, or when either widget is missing.
pub fn cfe_state_net_messenger_update_scroll_bar(c: &mut Cpu) {
    guest(c, |c| {
        'b_1a5298: {
            'b_1a5294: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S0][0] = c.g[A2][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S1][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                let t = c.g[V0][0] == 0u64;
                c.g[S3][0] = c.g[A1][0];
                if t {
                    break 'b_1a5294;
                } else {
                    c.cycles += 2;
                    let t = (c.g[S3][0] as i64) < 0;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    if t {
                        break 'b_1a5294;
                    } else {
                        c.cycles += 2;
                        if c.g[V0][0] != 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                            break 'b_1a5298;
                        } else {
                            c.cycles += 2;
                            if (c.g[S0][0] as i64) <= 0 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                                break 'b_1a5298;
                            } else {
                                c.cycles += 16;
                                c.g[A0][0] = 0x460000;
                                c.g[RA][0] = sx(0x001a5178);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1778));
                                call_ptr(c, 0x00317670, 0x001a5178)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001a5184);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001a5184)?;
                                c.g[A0][0] = 0x460000;
                                c.g[S2][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001a5194);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1788));
                                call_ptr(c, 0x00317670, 0x001a5194)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001a51a0);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001a51a0)?;
                                c.g[S1][0] = c.g[V0][0];
                                if c.g[S1][0] == 0u64 {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                                    break 'b_1a5298;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[S2][0] == 0u64;
                                    c.g[A1][0] = c.g[SP][0];
                                    if t {
                                        break 'b_1a5294;
                                    } else {
                                        c.cycles += 56;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001a51c8);
                                        c.g[A0][0] =
                                            sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001a51c8)?;
                                        c.ldl(2, (c.g[S2][0] as u32).wrapping_add(0x4b));
                                        c.ldr(2, (c.g[S2][0] as u32).wrapping_add(0x44));
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                                        c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
                                        c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                                        c.mtc1(19, 1);
                                        c.cvt_s_w(1, 1);
                                        c.g[V1][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
                                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
                                        if c.g[V1][0] == 0 {
                                            c.g[S0][0] = c.g[V0][0];
                                        }
                                        c.g[AT][0] = 0xffffffffc1700000;
                                        c.mtc1(1, 0);
                                        c.mtc1(16, 5);
                                        c.cvt_s_w(5, 5);
                                        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x17));
                                        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                        c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x27));
                                        c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                                        c.g[A1][0] = 0x1;
                                        c.add_s(3, 3, 0);
                                        c.g[AT][0] = 0xffffffffc0000000;
                                        c.mtc1(1, 4);
                                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x24));
                                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.div_s(3, 3, 5);
                                        c.add_s(0, 0, 4);
                                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.mul_s(1, 1, 3);
                                        c.add_s(2, 2, 1);
                                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x24));
                                        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x27));
                                        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                        c.sdl(6, (c.g[S1][0] as u32).wrapping_add(0x4b));
                                        c.sdr(6, (c.g[S1][0] as u32).wrapping_add(0x44));
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001a527c);
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001a527c)?;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0x1;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001a5294);
                                        c.g[A0][0] =
                                            sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001a5294)?;
                                        break 'b_1a5294;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            break 'b_1a5298;
        }
        c.cycles += 6;
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

/// `void cFEStateNetMessenger_FindNextPending(self)` (0x1a52b0): looks for the
/// next pending buddy message or challenge. Makes sure the buddy manager
/// singleton (`gp + 0x238`) exists and is connected (inlined `GetSession`, three
/// times), then, when it holds messages, asks for the challenger and the first
/// message; the name found is copied into a temporary string whose
/// `FindNameStr` index goes to `self + 0x6f8` (`-1` when nothing is pending).
pub fn cfe_state_net_messenger_find_next_pending(c: &mut Cpu) {
    guest(c, |c| {
        'b_1a52d0: {
            c.cycles += 6;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = c.g[A0][0];
            if t {
                break 'b_1a52d0;
            } else {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a52d0);
                call_ptr(c, 0x00260f80, 0x001a52d0)?;
                break 'b_1a52d0;
            }
        }
        'b_1a52ec: {
            c.cycles += 4;
            c.g[RA][0] = sx(0x001a52d8);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            call_ptr(c, 0x00261460, 0x001a52d8)?;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            if t {
                break 'b_1a52ec;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a52e8);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a52e8)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                break 'b_1a52ec;
            }
        }
        'b_1a5300: {
            c.cycles += 2;
            let t = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            if t {
                break 'b_1a5300;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a52fc);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a52fc)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                break 'b_1a5300;
            }
        }
        'b_1a5440: {
            'b_1a5434: {
                'b_1a5408: {
                    'b_1a536c: {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc0));
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xbc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                        if t {
                            break 'b_1a536c;
                        } else {
                            'b_1a5328: {
                                c.cycles += 2;
                                let t = c.g[A0][0] != 0u64;
                                if t {
                                    break 'b_1a5328;
                                } else {
                                    c.cycles += 2;
                                    c.g[RA][0] = sx(0x001a5328);
                                    call_ptr(c, 0x00260f80, 0x001a5328)?;
                                    break 'b_1a5328;
                                }
                            }
                            'b_1a5344: {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001a5330);
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                call_ptr(c, 0x00261460, 0x001a5330)?;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                if t {
                                    break 'b_1a5344;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x001a5340);
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    call_ptr(c, 0x002613c8, 0x001a5340)?;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    break 'b_1a5344;
                                }
                            }
                            'b_1a5358: {
                                c.cycles += 2;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                if t {
                                    break 'b_1a5358;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x001a5354);
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                    call_ptr(c, 0x0025b800, 0x001a5354)?;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    break 'b_1a5358;
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001a5360);
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x00263308, 0x001a5360)?;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A0][0] = c.g[SP][0];
                            if t {
                                break 'b_1a5408;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                break 'b_1a536c;
                            }
                        }
                    }
                    'b_1a537c: {
                        c.cycles += 2;
                        let t = c.g[V0][0] != 0u64;
                        if t {
                            break 'b_1a537c;
                        } else {
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x001a537c);
                            call_ptr(c, 0x00260f80, 0x001a537c)?;
                            break 'b_1a537c;
                        }
                    }
                    'b_1a5398: {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001a5384);
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                        call_ptr(c, 0x00261460, 0x001a5384)?;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                        if t {
                            break 'b_1a5398;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x001a5394);
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            call_ptr(c, 0x002613c8, 0x001a5394)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            break 'b_1a5398;
                        }
                    }
                    'b_1a53a8: {
                        c.cycles += 2;
                        let t = c.g[V0][0] != 0u64;
                        if t {
                            break 'b_1a53a8;
                        } else {
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x001a53a8);
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                            call_ptr(c, 0x0025b800, 0x001a53a8)?;
                            break 'b_1a53a8;
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001a53b0);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    call_ptr(c, 0x00262938, 0x001a53b0)?;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    if t {
                        break 'b_1a5434;
                    } else {
                        'b_1a53c8: {
                            c.cycles += 2;
                            let t = c.g[V0][0] != 0u64;
                            if t {
                                break 'b_1a53c8;
                            } else {
                                c.cycles += 2;
                                c.g[RA][0] = sx(0x001a53c8);
                                call_ptr(c, 0x00260f80, 0x001a53c8)?;
                                break 'b_1a53c8;
                            }
                        }
                        'b_1a53e4: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001a53d0);
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            call_ptr(c, 0x00261460, 0x001a53d0)?;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            if t {
                                break 'b_1a53e4;
                            } else {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x001a53e0);
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                call_ptr(c, 0x002613c8, 0x001a53e0)?;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                break 'b_1a53e4;
                            }
                        }
                        'b_1a53f8: {
                            c.cycles += 2;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            if t {
                                break 'b_1a53f8;
                            } else {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x001a53f4);
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                call_ptr(c, 0x0025b800, 0x001a53f4)?;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                break 'b_1a53f8;
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001a5400);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x00263630, 0x001a5400)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[SP][0];
                        if t {
                            break 'b_1a5434;
                        } else {
                            break 'b_1a5408;
                        }
                    }
                }
                c.cycles += 11;
                c.g[RA][0] = sx(0x001a5410);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x00317fe8, 0x001a5410)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001a541c);
                c.g[A1][0] = c.g[SP][0];
                Callee {
                    addr: 0x001a5800,
                    f: ee::s03::cFEStateNetMessenger_FindNameStr,
                }
                .call(c, 0x001a541c)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x6f8), c.g[V0][0]);
                c.g[A0][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x001a542c);
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x00318260, 0x001a542c)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                if t {
                    break 'b_1a5440;
                } else {
                    break 'b_1a5434;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = 0xffffffffffffffff;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x6f8), c.g[V0][0]);
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1a5440;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, elf_image, DATA, GAME_GP};

    pub(crate) const OC863_SELF: u32 = DATA + 0x1000;
    pub(crate) const OC863_POPUP: u32 = DATA + 0x2000;
    pub(crate) const OC863_MSG: u32 = DATA + 0x3000;
    pub(crate) const OC863_BUDDY: u32 = DATA + 0x4000;
    pub(crate) const OC863_BUDLIST: u32 = DATA + 0x4400;
    pub(crate) const OC863_WORLD: u32 = DATA + 0x5000;
    pub(crate) const OC863_TRACK: u32 = DATA + 0x7000;
    pub(crate) const OC863_THUMB: u32 = DATA + 0x7200;
    pub(crate) const OC863_VT: u32 = DATA + 0x7400;
    /// Fixture control word (the seed's random bits) and the `FindObject`
    /// call counter behind it. Above the fakes' call log, in zeroed RAM.
    pub(crate) const OC863_CTRL: u32 = DATA + 0x20_0000;

    /// The miss hook of this module's fixtures: the plain logging fake, then
    /// the results the paths need: the buddy manager queries
    /// (`IsConnected`, `GetChallenger`, `GetMessageCount`, `GetMessage`) give
    /// 0, 1, -1 or a pointer by the control word; `BuddyMan_ToggleUser` stores
    /// 0 or 1 into its `outError` argument (a3); `UIScreen_FindObject` finds
    /// the scroll track and thumb widgets, or nothing.
    fn oc863_hook(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let a3 = c.g[A3][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        let ctrl = c.bus.read32(OC863_CTRL) as u64;
        fakes::fake_call(c, target);
        if c.pc != ra {
            return;
        }
        match target {
            0x0026_1460 | 0x0026_3308 | 0x0026_2938 | 0x0026_3630 => {
                let pick = fakes::mix(ctrl ^ (target as u64) << 8 ^ (n as u64) << 32) & 3;
                match pick {
                    0 => c.g[V0][0] = 0,
                    1 => c.g[V0][0] = 1,
                    2 => c.g[V0][0] = u64::MAX,
                    _ => {}
                }
            }
            0x0026_1ef8 => {
                let flag = (fakes::mix(ctrl ^ (n as u64) << 16) >> 5) & 1;
                c.bus.write32(a3, flag as u32);
            }
            0x0039_d860 => {
                let k = c.bus.read32(OC863_CTRL + 8);
                c.bus.write32(OC863_CTRL + 8, k + 1);
                c.g[V0][0] = match k {
                    0 if ctrl & 1 == 0 => OC863_TRACK as u64,
                    1 if ctrl & 2 == 0 => OC863_THUMB as u64,
                    _ => 0,
                };
            }
            _ => {}
        }
    }

    /// The machine, the real game image (jump tables), this module's hook and
    /// a messenger at `OC863_SELF` with a popup, a buddy manager and the
    /// control word. Returns the generator for the caller's own fields.
    fn oc863_fixture(c: &mut Cpu, seed: u32, salt: u64, entry: u32) -> crate::difftest::Rng {
        let mut r = machine(c, seed, salt, 8);
        c.hooks.miss = oc863_hook;
        elf_image(c);
        c.pc = entry;
        c.bus.write32(OC863_CTRL, r.next_u32());
        c.bus.write32(OC863_CTRL + 8, 0);
        c.bus
            .write32(GAME_GP + 0x238, if seed % 3 == 0 { 0 } else { OC863_BUDDY });
        c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x6000);
        c.bus.write32(OC863_BUDDY + 0xbc, OC863_BUDLIST);
        c.bus
            .write32(OC863_BUDDY + 0xc0, OC863_BUDLIST + 4 * (seed % 4));
        c.bus.write32(OC863_SELF + 0x6a0, OC863_POPUP);
        c.bus.write32(OC863_SELF + 0x6c0, r.next_u32() & 0xff);
        c.g[A0][0] = sx(OC863_SELF);
        r
    }

    /// `ShowFindUserError`: both known errors ('iper' with and without an
    /// existing popup, 'inam') and an unknown one that goes to
    /// `NetFEState_HandleNetEvent` (with an event type outside its table).
    #[test]
    fn cfe_state_net_messenger_show_find_user_error_matches() {
        check(
            ee::s03::f_001a4e28__generated,
            cfe_state_net_messenger_show_find_user_error,
            90,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0863_4e28, 8);
                c.hooks.miss = oc863_hook;
                c.pc = 0x001a_4e28;
                c.bus.write32(OC863_CTRL, r.next_u32());
                // `SSXApp->FELocale` chain: world -> +0x8c -> +4 -> {delta, vtable}.
                c.bus.write32(GAME_GP.wrapping_sub(0x848), OC863_WORLD);
                c.bus.write32(OC863_WORLD + 0x8c, OC863_WORLD + 0x100);
                c.bus.write32(OC863_WORLD + 0x104, OC863_WORLD + 0x200);
                c.bus
                    .write16(OC863_WORLD + 0x220, (r.next_u32() & 0x3c) as u16);
                c.bus.write32(OC863_WORLD + 0x224, fakes::FAKE);
                c.bus.write32(OC863_SELF + 0x6d4, r.next_u32());
                c.bus.write32(
                    OC863_SELF + 0x6a0,
                    if seed % 4 == 3 { 0 } else { OC863_POPUP },
                );
                let error = match seed % 3 {
                    0 => 0x6970_6572,
                    1 => 0x696e_616d,
                    _ => 0x1234_5678,
                };
                c.bus.write32(OC863_MSG + 8, error);
                // An event type below the table of `HandleNetEvent`.
                c.bus.write32(OC863_MSG, 0x20 + r.next_u32() % 0x40);
                c.g[A0][0] = sx(OC863_SELF);
                c.g[A1][0] = sx(OC863_MSG);
            },
        );
    }

    /// `v31`: every event of the jump table (0xe3..0x113 except 0xef, whose
    /// default leads through `GotoNextState`) and events outside it that end
    /// in `HandleNetEvent`'s plain return. 0x102 (the find-user result) is set
    /// up to show the find-user error.
    #[test]
    fn cfe_state_net_messenger_v31_matches() {
        let mut events: Vec<u32> = (0xe3..=0x113).filter(|&e| e != 0xef).collect();
        events.extend([0x20, 0xdf, 0xe2, 0x500, 0]);
        check(
            ee::s03::f_001a5010__generated,
            cfe_state_net_messenger_v31,
            events.len() as u32 * 2,
            &|c, seed| {
                let mut r = oc863_fixture(c, seed, 0x0863_5010, 0x001a_5010);
                let event = events[seed as usize % events.len()];
                c.g[A1][0] = sx(event);
                c.g[A2][0] = sx(OC863_MSG);
                // `HandleNetEvent`'s own table sends every event of its range
                // that can reach it here to its plain return.
                c.bus.write32(OC863_MSG, 0x20);
                c.bus.write32(OC863_MSG + 8, 0x1111_1111);
                c.bus.write32(OC863_SELF + 0x6ac, r.next_u32() & 1);
                if event == 0x102 {
                    c.bus.write32(OC863_SELF + 0x1c, 6);
                    c.bus.write32(OC863_POPUP + 0x18, 0x807);
                }
            },
        );
    }

    /// `UpdateScrollBar`: the early exits (no screen, `pos < 0`,
    /// `count < pos`, `count <= 0`, a missing widget) and the move of the
    /// thumb for `count` 1 and 2+.
    #[test]
    fn cfe_state_net_messenger_update_scroll_bar_matches() {
        check(
            ee::s03::f_001a5128__generated,
            cfe_state_net_messenger_update_scroll_bar,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0863_5128, 8);
                c.hooks.miss = oc863_hook;
                c.pc = 0x001a_5128;
                c.bus.write32(OC863_CTRL, r.next_u32() & !3);
                if seed % 5 == 0 {
                    let ctrl = c.bus.read32(OC863_CTRL) | (1 + r.next_u32() % 3);
                    c.bus.write32(OC863_CTRL, ctrl);
                }
                c.bus.write32(OC863_CTRL + 8, 0);
                c.bus.write32(
                    OC863_SELF + 0x40,
                    if seed % 11 == 0 { 0 } else { OC863_POPUP },
                );
                for w in [OC863_TRACK, OC863_THUMB] {
                    c.bus.write32(w + 8, OC863_VT);
                    for i in 0..3 {
                        let f = (r.next_u32() % 4000) as f32 / 8.0;
                        c.bus.write32(w + 0x44 + 4 * i, f.to_bits());
                    }
                }
                for off in [0xa0u32, 0x48] {
                    c.bus.write16(OC863_VT + off, 0);
                    c.bus.write32(OC863_VT + off + 4, fakes::FAKE + off);
                }
                let (pos, count): (i32, i32) = match seed % 8 {
                    0 => (-1 - (r.next_u32() % 5) as i32, 4),
                    1 => (5, 2),
                    2 => (0, 0),
                    3 => (0, -3),
                    4 => (0, 1),
                    5 => (1, 2),
                    _ => ((r.next_u32() % 20) as i32, 20 + (r.next_u32() % 30) as i32),
                };
                c.g[A0][0] = sx(OC863_SELF);
                c.g[A1][0] = sx(pos as u32);
                c.g[A2][0] = sx(count as u32);
            },
        );
    }

    /// `FindNextPending`: the buddy manager missing or present (created,
    /// reconnected, lobby manager disconnected, by the fake results), no
    /// messages or some, a challenger or none, and a message found or not.
    #[test]
    fn cfe_state_net_messenger_find_next_pending_matches() {
        check(
            ee::s03::f_001a52b0__generated,
            cfe_state_net_messenger_find_next_pending,
            200,
            &|c, seed| {
                let _r = oc863_fixture(c, seed, 0x0863_52b0, 0x001a_52b0);
            },
        );
    }
}
