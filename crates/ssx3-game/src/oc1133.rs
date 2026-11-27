//! Staging module oc1133 (wave wave271): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1133`.

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
    (0x001a_49a0, oc1133_net_messenger_popup_v34, "cFEStateNetMessenger_v34"),
    (0x001a_4b20, oc1133_net_messenger_popup_v35, "cFEStateNetMessenger_v35"),
    (0x003a_1d30, oc1133_text_scroll_set_wide_string, "cUITextScroll_SetWideString"),
];

/// `void cFEStateNetMessenger_v34(self, popup, int kind)` (0x1a49a0): the
/// messenger's popup-event handler (variant 1). Kind 15 opens the challenge
/// popup for the selected user, 16 forwards the event and reads or sends a
/// message, 17 toggles the selected user as a buddy (max-buddies popup on
/// error) and rebuilds the buddy and player lists, 18 does the same for
/// blocking a user (buddy list only); anything else is forwarded to
/// `NetFEState_OnChildEvent`.
pub fn oc1133_net_messenger_popup_v34(c: &mut Cpu) {
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
pub fn oc1133_net_messenger_popup_v35(c: &mut Cpu) {
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

/// `void cUITextScroll_SetWideString(cUITextScroll* self, const u16* text)`
/// (0x3a1d30): word-wraps a UCS-2 string into the scroll box. Calls the
/// widget's virtual `+0x2c` method with `a1 = 1`, gets the font, copies the
/// text into a fresh buffer and walks it: a doubled backslash ends a line,
/// otherwise the char widths (`font width * self+0x50`) are summed and, once
/// the sum passes `self+0x60`, the line is cut at the last space (or at the
/// previous char) and added with `cUITextScroll_AddLine`; the rest is added
/// at the end and the buffer freed.
pub fn oc1133_text_scroll_set_wide_string(c: &mut Cpu) {
    guest(c, |c| {
        'epilogue: {
            c.cycles += 25;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S0][0] = c.g[A1][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S4][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[A1][0] = 0x1;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x003a_1d80)?;
            c.g[A0][0] = c.g[S4][0];
            Callee {
                addr: 0x003a_12d0,
                f: ee::s13::cUITextScroll_GetFont,
            }
            .call(c, 0x003a_1d88)?;
            c.g[S7][0] = c.g[V0][0];
            if c.g[S7][0] == 0 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                break 'epilogue;
            }
            'text: {
                c.cycles += 20;
                c.g[A0][0] = c.g[S0][0];
                c.g[S5][0] = 0xffff_ffff_ffff_ffff;
                call_ptr(c, 0x003d_bca0, 0x003a_1da0)?;
                c.g[S2][0] = 0;
                c.g[S6][0] = c.g[V0][0];
                c.g[A1][0] = 0x490000;
                c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4078));
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                c.g[A2][0] = 0x100;
                c.mtc1(0, 20);
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7e30, 0x003a_1dc8)?;
                c.g[S1][0] = 0;
                c.g[S3][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x003d_be20, 0x003a_1ddc)?;
                if (c.g[S6][0] as i64) <= 0 {
                    break 'text;
                }
                c.cycles += 3;
                c.g[FP][0] = 0xffff_ffff_ffff_ffff;
                c.g[V0][0] = sx((c.g[S2][0] as u32) << 1);
                'scan: loop {
                    'step: {
                        {
                            'plain: {
                                'newline: {
                                    c.cycles += 6;
                                    c.g[V1][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A0][0] = c.lhu(c.g[V1][0] as u32);
                                    c.g[V0][0] = 0x5c;
                                    let not_slash = c.g[A0][0] != c.g[V0][0];
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                    if not_slash {
                                        break 'newline;
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2));
                                    let not_double = c.g[V0][0] != c.g[A0][0];
                                    c.g[A0][0] = c.g[S7][0];
                                    if not_double {
                                        break 'plain;
                                    }
                                    // `\\`: cut the line here and add it.
                                    c.cycles += 10;
                                    c.g[A1][0] = sx((c.g[S1][0] as u32) << 1);
                                    c.sh(c.g[V1][0] as u32, 0);
                                    c.g[A1][0] =
                                        sx((c.g[S3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2));
                                    c.g[A0][0] = c.g[S4][0];
                                    c.g[S5][0] = 0xffff_ffff_ffff_ffff;
                                    Callee {
                                        addr: 0x003a_1bf8,
                                        f: ee::s13::cUITextScroll_AddLine,
                                    }
                                    .call(c, 0x003a_1e30)?;
                                    c.mtc1(0, 20);
                                    c.g[S1][0] = c.g[S2][0];
                                    break 'step;
                                }
                                c.cycles += 1;
                                c.g[A0][0] = c.g[S7][0];
                            }
                            'advance: {
                                c.cycles += 12;
                                c.g[A1][0] = c.lhu(c.g[S0][0] as u32);
                                Callee {
                                    addr: 0x0039_2c60,
                                    f: ee::s13::Font_GetCharWidthW,
                                }
                                .call(c, 0x003a_1e48)?;
                                c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x50));
                                c.lwc1(2, (c.g[S4][0] as u32).wrapping_add(0x60));
                                c.mul_s(0, 0, 1);
                                c.g[V0][0] = c.lhu(c.g[S0][0] as u32);
                                c.g[V0][0] ^= 0x20;
                                c.add_s(20, 20, 0);
                                c.c_lt_s(2, 20);
                                let within = !c.fcc();
                                if c.g[V0][0] == 0 {
                                    c.g[S5][0] = c.g[S2][0];
                                }
                                if within {
                                    break 'advance;
                                }
                                c.cycles += 5;
                                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                c.g[V0][0] = ((c.g[FP][0] as i64) < (c.g[S5][0] as i64)) as u64;
                                if c.g[V0][0] == 0 {
                                    c.g[S5][0] = c.g[V1][0];
                                }
                                let no_cut = (c.g[S5][0] as i64) <= 0;
                                c.g[V0][0] = sx((c.g[S5][0] as u32) << 1);
                                if no_cut {
                                    break 'advance;
                                }
                                // Too wide: cut at the last space and add the line.
                                c.cycles += 10;
                                c.g[A1][0] = sx((c.g[S1][0] as u32) << 1);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                c.g[S2][0] = c.g[S5][0];
                                c.sh(c.g[V0][0] as u32, 0);
                                c.g[A1][0] =
                                    sx((c.g[S3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                c.mtc1(0, 20);
                                c.g[A0][0] = c.g[S4][0];
                                Callee {
                                    addr: 0x003a_1bf8,
                                    f: ee::s13::cUITextScroll_AddLine,
                                }
                                .call(c, 0x003a_1ea4)?;
                                c.g[S5][0] = 0xffff_ffff_ffff_ffff;
                                c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                            }
                            c.cycles += 1;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[S2][0] as u32) << 1);
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_1df0);
                        }
                        continue 'scan;
                    }
                    break 'text;
                }
            }
            c.cycles += 2;
            let rest_empty = c.g[S1][0] == c.g[S2][0];
            c.g[A0][0] = c.g[S4][0];
            if !rest_empty {
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[S1][0] as u32) << 1);
                c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                Callee {
                    addr: 0x003a_1bf8,
                    f: ee::s13::cUITextScroll_AddLine,
                }
                .call(c, 0x003a_1ed0)?;
            }
            c.cycles += 2;
            let no_buffer = c.g[S3][0] == 0;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            if !no_buffer {
                c.cycles += 3;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x0031_7e98, 0x003a_1ee0)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            }
        }
        c.cycles += 12;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, machine_rt};
    use crate::difftest::DATA;
    use std::ptr::addr_of_mut;

    /// Gate word of this module's fakes (their own word, in the machine's RAM).
    pub(crate) const OC1133_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC1133_MAGIC: u32 = 0x1133_c0de;

    /// The gated fake callee: logs the call (it may unwind), then answers
    /// what the fixture wrote: the font object (`CTRL+4`), the out-word of the
    /// buddy toggle (`CTRL+8`) and the block toggle (`CTRL+12`), a char width.
    pub(crate) fn oc1133_fake_body(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let a1 = c.g[A1][0] as u32;
        let a2 = c.g[A2][0] as u32;
        fakes::fake_call(c, target);
        if c.pc != ra {
            return;
        }
        match target {
            0x001a_be58 => {
                let w = c.bus.read32(OC1133_CTRL + 8);
                c.bus.write32(a2, w);
            }
            0x001a_bc50 => {
                let w = c.bus.read32(OC1133_CTRL + 12);
                c.bus.write32(a2, w);
            }
            0x003a_12d0 => c.g[V0][0] = c.bus.read32(OC1133_CTRL + 4) as u64,
            0x0039_2c60 => c.f[0] = (4.0f32 + (a1 & 7) as f32).to_bits(),
            _ => {}
        }
    }

    macro_rules! oc1133_fake {
        ($name:ident, $target:expr, $generated:path) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1133_CTRL) != OC1133_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $generated);
                }
                oc1133_fake_body(c, $target);
            }
        };
    }

    oc1133_fake!(oc1133_fake_1abcd0, 0x001a_bcd0, ee::s03::f_001abcd0__generated);
    oc1133_fake!(oc1133_fake_1a98b0, 0x001a_98b0, ee::s03::f_001a98b0__generated);
    oc1133_fake!(oc1133_fake_1a97b8, 0x001a_97b8, ee::s03::f_001a97b8__generated);
    oc1133_fake!(oc1133_fake_1abb30, 0x001a_bb30, ee::s03::f_001abb30__generated);
    oc1133_fake!(oc1133_fake_1abed8, 0x001a_bed8, ee::s03::f_001abed8__generated);
    oc1133_fake!(oc1133_fake_1a5898, 0x001a_5898, ee::s03::f_001a5898__generated);
    oc1133_fake!(oc1133_fake_1a6978, 0x001a_6978, ee::s03::f_001a6978__generated);
    oc1133_fake!(oc1133_fake_1abe58, 0x001a_be58, ee::s03::f_001abe58__generated);
    oc1133_fake!(oc1133_fake_1abc50, 0x001a_bc50, ee::s03::f_001abc50__generated);
    oc1133_fake!(oc1133_fake_3a12d0, 0x003a_12d0, ee::s13::f_003a12d0__generated);
    oc1133_fake!(oc1133_fake_3a1bf8, 0x003a_1bf8, ee::s13::f_003a1bf8__generated);
    oc1133_fake!(oc1133_fake_392c60, 0x0039_2c60, ee::s13::f_00392c60__generated);

    pub(crate) fn oc1133_install_fakes() {
        macro_rules! hook {
            ($slot:expr, $fake:ident) => {
                fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake));
            };
        }
        hook!(ee::s03::OVERRIDE_001ABCD0, oc1133_fake_1abcd0);
        hook!(ee::s03::OVERRIDE_001A98B0, oc1133_fake_1a98b0);
        hook!(ee::s03::OVERRIDE_001A97B8, oc1133_fake_1a97b8);
        hook!(ee::s03::OVERRIDE_001ABB30, oc1133_fake_1abb30);
        hook!(ee::s03::OVERRIDE_001ABED8, oc1133_fake_1abed8);
        hook!(ee::s03::OVERRIDE_001A5898, oc1133_fake_1a5898);
        hook!(ee::s03::OVERRIDE_001A6978, oc1133_fake_1a6978);
        hook!(ee::s03::OVERRIDE_001ABE58, oc1133_fake_1abe58);
        hook!(ee::s03::OVERRIDE_001ABC50, oc1133_fake_1abc50);
        hook!(ee::s13::OVERRIDE_003A12D0, oc1133_fake_3a12d0);
        hook!(ee::s13::OVERRIDE_003A1BF8, oc1133_fake_3a1bf8);
        hook!(ee::s13::OVERRIDE_00392C60, oc1133_fake_392c60);
    }

    /// Fixture of the messenger popup handlers: the event kind cycles through
    /// `kinds`; the toggles' out-words (1 = max buddies reached) vary by seed.
    pub(crate) fn oc1133_messenger_setup(c: &mut Cpu, seed: u32, salt: u64, kinds: &[u32]) {
        machine(c, seed, salt, 7);
        c.bus.write32(OC1133_CTRL, OC1133_MAGIC);
        c.bus
            .write32(OC1133_CTRL + 8, [1, 0, 5][(seed as usize / 6) % 3]);
        c.bus
            .write32(OC1133_CTRL + 12, [1, 2, 0][(seed as usize / 18) % 3]);
        c.g[A0][0] = sx(DATA + 0x100);
        c.g[A1][0] = sx(DATA + 0x200);
        c.g[A2][0] = kinds[seed as usize % kinds.len()] as u64;
    }

    /// The popup-event handler, variant 1 (kinds 15-18 and "other").
    #[test]
    fn oc1133_net_messenger_popup_v34_matches() {
        oc1133_install_fakes();
        covered(
            ee::s03::f_001a49a0__generated,
            oc1133_net_messenger_popup_v34,
            162,
            &|c, seed| oc1133_messenger_setup(c, seed, 0x1a49a0, &[15, 16, 17, 18, 3, 0x100]),
        );
    }

    /// The popup-event handler, variant 2 (kinds 15-17 and "other").
    #[test]
    fn oc1133_net_messenger_popup_v35_matches() {
        oc1133_install_fakes();
        covered(
            ee::s03::f_001a4b20__generated,
            oc1133_net_messenger_popup_v35,
            162,
            &|c, seed| oc1133_messenger_setup(c, seed, 0x1a4b20, &[15, 16, 17, 3, 0x100, 0]),
        );
    }

    /// Word-wrapping a UCS-2 string into the scroll box: random strings of
    /// letters, spaces and backslash pairs against several box widths, with
    /// the font missing for some seeds and the empty string for others.
    #[test]
    fn oc1133_text_scroll_set_wide_string_matches() {
        oc1133_install_fakes();
        covered(
            ee::s13::f_003a1d30__generated,
            oc1133_text_scroll_set_wide_string,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x3a1d30, 8);
                c.bus.write32(OC1133_CTRL, OC1133_MAGIC);
                c.bus
                    .write32(OC1133_CTRL + 4, if seed % 6 == 0 { 0 } else { DATA + 0x300 });
                let this = DATA + 0x100;
                let widget = DATA + 0x200;
                c.bus.write32(this + 8, widget);
                c.bus.write16(widget + 0x28, 0);
                c.bus.write32(widget + 0x2c, 0x0014_06c0);
                let scale = [1.0f32, 0.5][(seed as usize / 3) % 2];
                let width = [6.0f32, 20.0, 30.0, 1000.0][(seed as usize / 2) % 4];
                c.bus.write32(this + 0x50, scale.to_bits());
                c.bus.write32(this + 0x60, width.to_bits());
                let text = DATA + 0x400;
                let len = if seed % 7 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 26
                };
                let alphabet = [0x61u32, 0x62, 0x20, 0x5c, 0x5c, 0x41];
                for i in 0..len {
                    let ch = alphabet[(r.next_u32() % 6) as usize];
                    c.bus.write16(text + 2 * i, ch as u16);
                }
                c.bus.write16(text + 2 * len, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(text);
            },
        );
    }
}
