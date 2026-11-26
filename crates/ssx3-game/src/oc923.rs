//! Staging module oc923 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc923`.

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
        0x001a_ab68,
        net_fe_state_update_message_count,
        "NetFEState_UpdateMessageCount",
    ),
    (
        0x001a_af90,
        c_fe_state_net_config_get_error_text,
        "cFEStateNetConfig_GetErrorText",
    ),
    (
        0x001a_bb30,
        net_fe_state_read_or_send_message,
        "NetFEState_ReadOrSendMessage",
    ),
    (
        0x001a_bcd0,
        net_fe_state_open_challenge_popup,
        "NetFEState_OpenChallengePopup",
    ),
    (
        0x001a_e680,
        c_fe_state_challenge_popup_apply_settings,
        "cFEStateChallengePopup_ApplySettings",
    ),
    (
        0x001a_f098,
        c_fe_state_challenge_popup_init_setting_lists,
        "cFEStateChallengePopup_InitSettingLists",
    ),
    (0x001b_25e0, net_fe_format_room_name, "NetFE_FormatRoomName"),
    (
        0x001b_4018,
        c_fe_state_net_config_dtor,
        "cFEStateNetConfig_dtor",
    ),
];

#[inline(always)]
fn oc923_lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(r32(c, reg).wrapping_add(off)).lw(c)
}

#[inline(always)]
fn oc923_lh(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(r32(c, reg).wrapping_add(off)).lh(c)
}

#[inline(always)]
fn oc923_lb(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(r32(c, reg).wrapping_add(off)).lb(c)
}

#[inline(always)]
fn oc923_lbu(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(r32(c, reg).wrapping_add(off)).lbu(c)
}

#[inline(always)]
fn oc923_sw(c: &mut Cpu, reg: usize, off: u32, v: u64) {
    Addr(r32(c, reg).wrapping_add(off)).sw(c, v)
}

#[inline(always)]
fn oc923_sb(c: &mut Cpu, reg: usize, off: u32, v: u64) {
    Addr(r32(c, reg).wrapping_add(off)).sb(c, v)
}

/// `gp + 0x238`: the BuddyMan (messenger) session object, or 0.
const GP_BUDDY_MAN: u32 = 0x238;
/// `gp - 0xc8`: the LobbyMan object.
const GP_LOBBY_MAN: u32 = 0u32.wrapping_sub(0xc8);
/// `gp - 0x848`: the world / front-end object.
const GP_WORLD: u32 = 0u32.wrapping_sub(0x848);

/// `if (!BuddyMan) f_00261460's guard`: the second stage the message and
/// challenge helpers share: `if (func_00261460(BuddyMan) == 0)
/// func_002613c8(BuddyMan);` then v0 = the BuddyMan again.
fn oc923_buddy_man_ensure(c: &mut Cpu, ret_a: u32, ret_b: u32) -> crate::call::Flow {
    c.cycles += 4;
    c.g[A0][0] = oc923_lw(c, GP, GP_BUDDY_MAN);
    call_ptr(c, 0x0026_1460, ret_a)?;
    let found = c.g[V0][0] != 0;
    c.g[V0][0] = oc923_lw(c, GP, GP_BUDDY_MAN);
    if !found {
        c.cycles += 3;
        c.g[A0][0] = oc923_lw(c, GP, GP_BUDDY_MAN);
        call_ptr(c, 0x0026_13c8, ret_b)?;
        c.g[V0][0] = oc923_lw(c, GP, GP_BUDDY_MAN);
    }
    Ok(())
}

/// `void NetFEState_UpdateMessageCount(NetFEState* self)` (0x1aab68): once
/// a second while the state is shown, polls the messenger: remembers when
/// unread messages / challenges first appeared (+0x50 / +0x54) and shows
/// the unread count ("%d") in the `TextMessageCount` text object, hiding it
/// when there are none.
pub fn net_fe_state_update_message_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x10);
        let leave = |c: &mut Cpu| {
            frame.restore(c, S0, 0x40);
            c.cycles += 5;
            frame.restore(c, S1, 0x30);
            frame.restore(c, S2, 0x20);
            frame.restore_ra(c, 0x10);
            frame.close(c);
        };
        c.g[V0][0] = oc923_lw(c, S0, 0x40);
        if c.g[V0][0] == 0 {
            leave(c);
            return Ok(());
        }
        // `self->v29()` (the vtable method at +0xe8/+0xec): visible?
        c.cycles += 7;
        c.g[V1][0] = oc923_lw(c, S0, 8);
        c.g[A0][0] = oc923_lh(c, V1, 0xe8);
        c.g[V0][0] = oc923_lw(c, V1, 0xec);
        let method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x001a_aba0)?;
        let hidden = c.g[V0][0] == 0;
        c.g[V1][0] = oc923_lw(c, GP, 0x2a74);
        if hidden {
            c.cycles += 1;
            leave(c);
            return Ok(());
        }
        // Poll at most once a second: now = the app's tick (+0x18).
        c.cycles += 7;
        c.g[V0][0] = oc923_lw(c, S0, 0x4c);
        c.g[S2][0] = oc923_lw(c, V1, 0x18);
        c.g[V0][0] = sx(r32(c, S2).wrapping_sub(r32(c, V0)));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        c.g[V0][0] = (c.g[V0][0] < 0x3b) as u64;
        if c.g[V0][0] != 0 {
            leave(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = oc923_lw(c, GP, GP_BUDDY_MAN);
        let have_buddy = c.g[V0][0] != 0;
        let now = c.g[S2][0];
        oc923_sw(c, S0, 0x4c, now);
        if !have_buddy {
            c.cycles += 2;
            call_ptr(c, 0x0026_0f80, 0x001a_abd8)?;
        }
        oc923_buddy_man_ensure(c, 0x001a_abe0, 0x001a_abf0)?;
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[A0][0] = oc923_lw(c, GP, GP_LOBBY_MAN);
            call_ptr(c, 0x0025_b800, 0x001a_ac04)?;
        }
        // n = the unread message count.
        c.cycles += 6;
        c.g[A0][0] = oc923_lw(c, GP, GP_BUDDY_MAN);
        call_ptr(c, 0x0026_2938, 0x001a_ac0c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = oc923_lw(c, S0, 0x50);
        let n = c.g[S1][0] as i64;
        if c.g[V0][0] == 0 {
            // No hint yet: start it when there is a message.
            c.cycles += 2;
            if n > 0 {
                let now = c.g[S2][0];
                oc923_sw(c, S0, 0x50, now);
            }
            c.cycles += 1;
            c.g[V0][0] = oc923_lw(c, GP, GP_BUDDY_MAN);
        } else {
            c.cycles += 2;
            if n <= 0 {
                oc923_sw(c, S0, 0x50, 0);
                c.cycles += 1;
                c.g[V0][0] = oc923_lw(c, GP, GP_BUDDY_MAN);
            } else {
                c.cycles += 2;
                c.g[V0][0] = oc923_lw(c, GP, GP_BUDDY_MAN);
            }
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            call_ptr(c, 0x0026_0f80, 0x001a_ac48)?;
        }
        oc923_buddy_man_ensure(c, 0x001a_ac50, 0x001a_ac60)?;
        c.cycles += 2;
        let have_buddy = c.g[V0][0] != 0;
        c.g[V1][0] = oc923_lw(c, GP, GP_BUDDY_MAN);
        if !have_buddy {
            c.cycles += 3;
            c.g[A0][0] = oc923_lw(c, GP, GP_LOBBY_MAN);
            call_ptr(c, 0x0025_b800, 0x001a_ac74)?;
            c.g[V1][0] = oc923_lw(c, GP, GP_BUDDY_MAN);
        }
        // Any pending challenge: the list at BuddyMan+0xbc..0xc0 is not empty.
        c.cycles += 7;
        c.g[A1][0] = oc923_lw(c, S0, 0x54);
        c.g[A0][0] = oc923_lw(c, V1, 0xbc);
        c.g[V0][0] = oc923_lw(c, V1, 0xc0);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A0)));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        let no_hint = c.g[A1][0] == 0;
        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        if no_hint {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                let now = c.g[S2][0];
                oc923_sw(c, S0, 0x54, now);
            }
            c.cycles += 1;
        } else {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                oc923_sw(c, S0, 0x54, 0);
                c.cycles += 1;
            } else {
                c.cycles += 2;
            }
        }
        // The `TextMessageCount` text object.
        c.g[A0][0] = 0x46_0000;
        c.cycles += 8;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x2650));
        call_ptr(c, 0x0031_7670, 0x001a_acb8)?;
        c.g[A0][0] = oc923_lw(c, S0, 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001a_acc4)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            leave(c);
            return Ok(());
        }
        c.cycles += 2;
        let none = (c.g[S1][0] as i64) <= 0;
        c.g[V0][0] = oc923_lw(c, S0, 8);
        if !none {
            // setVisible(true), sprintf(sp, "%d", n), setAsciiString.
            c.cycles += 15;
            c.g[A1][0] = 1;
            c.g[A0][0] = oc923_lh(c, V0, 0x48);
            c.g[V1][0] = oc923_lw(c, V0, 0x4c);
            let method = r32(c, V1);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x001a_acec)?;
            c.g[A1][0] = 0x4a_0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x12c0));
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x0041_7828, 0x001a_ad00)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003a_0c50, 0x001a_ad0c)?;
            leave(c);
            return Ok(());
        }
        // setVisible(false), setAsciiString(" ").
        c.cycles += 9;
        c.g[A1][0] = 0;
        c.g[A0][0] = oc923_lh(c, V0, 0x48);
        c.g[V1][0] = oc923_lw(c, V0, 0x4c);
        let method = r32(c, V1);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x001a_ad28)?;
        c.g[A1][0] = 0x4a_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1390));
        call_ptr(c, 0x003a_0c50, 0x001a_ad38)?;
        c.cycles += 1;
        leave(c);
        Ok(())
    })
}

/// `int cFEStateNetConfig_GetErrorText(self, NetMsg* msg)` (0x1aaf90): maps
/// the message's four-letter error code (`msg+8`: "-dif", "-dev", "-cfg",
/// "-ini", "-ton", "-bsy", "-con", "-???", "-ath", "-dns", "time") to its
/// popup text and returns the localised string (0 for an unknown code). The
/// generated body is a binary search over the codes; the port keeps its
/// compare order, because the cycles and the scratch `v0` depend on it.
pub fn c_fe_state_net_config_get_error_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x2d64_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] |= 0x6576;
        c.g[V1][0] = oc923_lw(c, A1, 8);
        let eq = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        // (extra cycles, offset of the text name literal) or None.
        let pick: Option<(u64, u32)> = 'pick: {
            if eq {
                break 'pick Some((3, 0x26a0)); // "-dev": Locate
            }
            c.cycles += 2;
            let above = c.g[V0][0] != 0;
            c.g[V0][0] = 0x2d69_0000;
            if !above {
                c.cycles += 4;
                c.g[V0][0] = 0x2d62_0000 | 0x7379;
                let eq = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                if eq {
                    break 'pick Some((3, 0x2700)); // "-bsy": Busy
                }
                c.cycles += 2;
                let above = c.g[V0][0] != 0;
                c.g[V0][0] = 0x2d63_0000;
                if !above {
                    c.cycles += 4;
                    c.g[V0][0] = 0x2d3f_0000 | 0x3f3f;
                    let eq = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x2d61_0000;
                    if eq {
                        break 'pick Some((2, 0x2728)); // "-???": ISPFail
                    }
                    c.cycles += 3;
                    c.g[V0][0] |= 0x7468;
                    let eq = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0;
                    if eq {
                        break 'pick Some((3, 0x2740)); // "-ath": PPP
                    }
                    c.cycles += 2;
                    break 'pick None;
                }
                c.cycles += 3;
                c.g[V0][0] |= 0x6667;
                let eq = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x2d63_0000;
                if eq {
                    break 'pick Some((3, 0x26b8)); // "-cfg": NetConfig
                }
                c.cycles += 3;
                c.g[V0][0] |= 0x6f6e;
                let eq = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0;
                if eq {
                    break 'pick Some((3, 0x2710)); // "-con": Connect
                }
                c.cycles += 2;
                break 'pick None;
            }
            c.cycles += 3;
            c.g[V0][0] |= 0x6e69;
            let eq = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
            if eq {
                break 'pick Some((3, 0x26d0)); // "-ini": HardwareInit
            }
            c.cycles += 2;
            let above = c.g[V0][0] != 0;
            c.g[V0][0] = 0x2d74_0000;
            if !above {
                c.cycles += 4;
                c.g[V0][0] = 0x2d64_0000 | 0x6966;
                let eq = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x2d64_0000;
                if eq {
                    break 'pick Some((3, 0x2688)); // "-dif": DifferentPS2
                }
                c.cycles += 3;
                c.g[V0][0] |= 0x6e73;
                let eq = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0;
                if eq {
                    break 'pick Some((2, 0x2728)); // "-dns": ISPFail
                }
                c.cycles += 2;
                break 'pick None;
            }
            c.cycles += 3;
            c.g[V0][0] |= 0x6f6e;
            let eq = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x7469_0000;
            if eq {
                break 'pick Some((3, 0x26e8)); // "-ton": Dialtone
            }
            c.cycles += 3;
            c.g[V0][0] |= 0x6d65;
            let eq = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0;
            if eq {
                break 'pick Some((2, 0x2728)); // "time": ISPFail
            }
            c.cycles += 2;
            None
        };
        match pick {
            Some((extra, offset)) => {
                c.cycles += extra;
                c.g[A0][0] = 0x46_0000;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(offset));
                // FELocale->getStringByHash(GetHashValue32(name)).
                c.cycles += 11;
                call_ptr(c, 0x0031_7670, 0x001a_b0d8)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = oc923_lw(c, GP, GP_WORLD);
                c.g[A2][0] = oc923_lw(c, V0, 0x8c);
                c.g[V1][0] = oc923_lw(c, A2, 4);
                c.g[A0][0] = oc923_lh(c, V1, 0x20);
                c.g[V0][0] = oc923_lw(c, V1, 0x24);
                let method = r32(c, V0);
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                call_ptr(c, method, 0x001a_b0f8)?;
                frame.restore_ra(c, 0);
            }
            None => frame.restore_ra(c, 0),
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void NetFEState_ReadOrSendMessage(self, name)` (0x1abb30): clears
/// `mUserPopupOpen`, makes sure the messenger session exists, then either
/// pushes the "read message" popup (when the buddy manager has a message
/// from `name`) or opens the keyboard to write one to `name`.
pub fn net_fe_state_read_or_send_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = oc923_lw(c, GP, GP_BUDDY_MAN);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let have_buddy = c.g[V0][0] != 0;
        oc923_sw(c, S0, 0x6c4, 0);
        if !have_buddy {
            c.cycles += 2;
            call_ptr(c, 0x0026_0f80, 0x001a_bb5c)?;
        }
        oc923_buddy_man_ensure(c, 0x001a_bb64, 0x001a_bb74)?;
        c.cycles += 2;
        let have_buddy = c.g[V0][0] != 0;
        c.g[A0][0] = oc923_lw(c, GP, GP_BUDDY_MAN);
        if !have_buddy {
            c.cycles += 3;
            c.g[A0][0] = oc923_lw(c, GP, GP_LOBBY_MAN);
            call_ptr(c, 0x0025_b800, 0x001a_bb88)?;
            c.g[A0][0] = oc923_lw(c, GP, GP_BUDDY_MAN);
        }
        c.cycles += 4;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0026_2a38, 0x001a_bb94)?;
        let no_message = c.g[V0][0] == 0;
        c.g[A2][0] = 0x100;
        if !no_message {
            // new cFEStateReadMessagePopup(engine, self, name), pushed.
            c.cycles += 18;
            c.g[A1][0] = 0x46_0000;
            c.g[A0][0] = 0x6fc;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x2aa8));
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x001a_bbb0)?;
            c.g[A1][0] = oc923_lw(c, S0, 0x10);
            c.g[A0][0] = c.g[V0][0];
            c.g[A3][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x001a_77a0, 0x001a_bbc4)?;
            c.g[V1][0] = oc923_lw(c, S0, 0x40);
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = oc923_lw(c, V1, 0xd0);
            c.g[A0][0] = oc923_lw(c, V0, 0x10);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
            call_ptr(c, 0x0039_f290, 0x001a_bbdc)?;
            frame.restore(c, S0, 0x20);
        } else {
            // Open the keyboard for the player name.
            c.cycles += 23;
            c.g[A3][0] = 0x4a_0000;
            c.g[T0][0] = 0;
            c.g[T1][0] = 2;
            c.g[T2][0] = 0x3f;
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(0x1408));
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x001a_9090, 0x001a_bc08)?;
            c.g[V0][0] = oc923_lw(c, S0, 0x69c);
            c.g[A1][0] = 0x4b;
            c.g[A2][0] = 1;
            c.g[A3][0] = 2;
            oc923_sw(c, V0, 0x43c, 0);
            c.g[A0][0] = oc923_lw(c, S0, 0x69c);
            call_ptr(c, 0x001c_e3c8, 0x001a_bc24)?;
            c.g[A0][0] = oc923_lw(c, S0, 0x69c);
            c.g[A1][0] = 1;
            call_ptr(c, 0x001c_e468, 0x001a_bc30)?;
            c.g[V1][0] = oc923_lw(c, S0, 0x69c);
            c.g[V0][0] = 1;
            let one = c.g[V0][0];
            oc923_sw(c, V1, 0x438, one);
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void NetFEState_OpenChallengePopup(self, name, popup, event, fromPopup)`
/// (0x1abcd0): clears `mUserPopupOpen`, forwards the child event, makes sure
/// the messenger session exists, then pushes the "challenge received" popup
/// (when `name` has challenged us) or starts a new challenge popup against
/// `name` (resetting the rules and the opponent name first).
pub fn net_fe_state_open_challenge_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[A2][0];
        oc923_sw(c, S1, 0x6c4, 0);
        c.g[S2][0] = c.g[T0][0];
        c.g[A2][0] = c.g[A3][0];
        call_ptr(c, 0x001a_97b8, 0x001a_bd00)?;
        c.g[V0][0] = oc923_lw(c, GP, GP_BUDDY_MAN);
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            call_ptr(c, 0x0026_0f80, 0x001a_bd14)?;
        }
        oc923_buddy_man_ensure(c, 0x001a_bd1c, 0x001a_bd2c)?;
        c.cycles += 2;
        let have_buddy = c.g[V0][0] != 0;
        c.g[A0][0] = oc923_lw(c, GP, GP_BUDDY_MAN);
        if !have_buddy {
            c.cycles += 3;
            c.g[A0][0] = oc923_lw(c, GP, GP_LOBBY_MAN);
            call_ptr(c, 0x0025_b800, 0x001a_bd40)?;
            c.g[A0][0] = oc923_lw(c, GP, GP_BUDDY_MAN);
        }
        c.cycles += 4;
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0026_3128, 0x001a_bd4c)?;
        let no_challenge = c.g[V0][0] == 0;
        c.g[A0][0] = 0x53_0000;
        if !no_challenge {
            // new cFEStateChallengeReceivedPopup(engine, self, name).
            c.cycles += 14;
            c.g[A1][0] = 0x46_0000;
            c.g[A0][0] = 0x6d8;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x2ac8));
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x001a_bd6c)?;
            c.g[A1][0] = oc923_lw(c, S1, 0x10);
            c.g[A0][0] = c.g[V0][0];
            c.g[A3][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x001a_fe08, 0x001a_bd80)?;
            c.g[A1][0] = c.g[V0][0];
            let from_popup = c.g[S2][0] != 0;
            c.g[V0][0] = 0x801;
            if from_popup {
                c.cycles += 1;
                c.g[V0][0] = 0x834;
            }
            c.cycles += 8;
            let misc = c.g[V0][0];
            oc923_sw(c, A1, 0x18, misc);
            c.g[V0][0] = oc923_lw(c, S1, 0x40);
            c.g[V1][0] = oc923_lw(c, V0, 0xd0);
            c.g[A0][0] = oc923_lw(c, V1, 0x10);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
            call_ptr(c, 0x0039_f290, 0x001a_bda8)?;
            frame.restore(c, S0, 0x40);
        } else {
            // Reset the rules, remember the opponent, start a new
            // cFEStateChallengePopup(engine, self).
            c.cycles += 29;
            c.g[A2][0] = 0x20;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x8b8));
            c.g[A1][0] = 0;
            call_ptr(c, 0x003e_6448, 0x001a_bdc0)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[V0][0] = 4;
            c.g[S0][0] = oc923_lw(c, GP, GP_LOBBY_MAN);
            let four = c.g[V0][0];
            oc923_sw(c, GP, 0x1d9c, four);
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_82d8, 0x001a_bdd8)?;
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x98));
            call_ptr(c, 0x0031_8448, 0x001a_bde4)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 2;
            call_ptr(c, 0x0031_8260, 0x001a_bdf0)?;
            c.g[A1][0] = 0x46_0000;
            c.g[A0][0] = 0x6f0;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x2ae8));
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x001a_be08)?;
            c.g[A1][0] = oc923_lw(c, S1, 0x10);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x001a_ddd0, 0x001a_be18)?;
            c.g[A1][0] = c.g[V0][0];
            let from_popup = c.g[S2][0] != 0;
            c.g[V0][0] = 0x800;
            if from_popup {
                c.cycles += 1;
                c.g[V0][0] = 0x834;
            }
            c.cycles += 7;
            let misc = c.g[V0][0];
            oc923_sw(c, A1, 0x18, misc);
            c.g[V0][0] = oc923_lw(c, S1, 0x40);
            c.g[V1][0] = oc923_lw(c, V0, 0xd0);
            c.g[A0][0] = oc923_lw(c, V1, 0x10);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
            call_ptr(c, 0x0039_f290, 0x001a_be40)?;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateChallengePopup_ApplySettings(self)` (0x1ae680): pushes the
/// popup's setting choices to the engine: the event-option flags are written
/// into a global, the four list boxes are updated, and the hook at the
/// world's vtable is called with the result. Does nothing when `self+0x40`
/// (the setting-list object) is empty.
pub fn c_fe_state_challenge_popup_apply_settings(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = oc923_lw(c, S1, 0x40);
        let empty = c.g[V0][0] == 0;
        c.g[V0][0] = oc923_lw(c, GP, GP_WORLD);
        if !empty {
            c.cycles += 58;
            c.g[A1][0] = 7;
            c.g[A0][0] = oc923_lw(c, V0, 0x78);
            call_ptr(c, 0x0014_dd58, 0x001a_e6ac)?;
            c.g[A0][0] = oc923_lw(c, S1, 0x6dc);
            c.g[S0][0] = c.g[V0][0];
            call_ptr(c, 0x0039_a738, 0x001a_e6b8)?;
            c.g[V0][0] = (0 < c.g[V0][0]) as u64;
            c.g[V1][0] = 0x53_0000;
            let v = c.g[V0][0];
            oc923_sw(c, V1, 0x4b34, v);
            c.g[V0][0] = oc923_lw(c, S0, 0xc);
            c.g[A0][0] = oc923_lh(c, V0, 8);
            c.g[V1][0] = oc923_lw(c, V0, 0xc);
            let hook = r32(c, V1);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            call_ptr(c, hook, 0x001a_e6d8)?;
            c.g[A0][0] = oc923_lw(c, S1, 0x6e8);
            call_ptr(c, 0x0039_a738, 0x001a_e6e0)?;
            // Event-option flags (global at 0x5308b8 + 0x18): bit 6, 8, 7.
            c.g[S0][0] = 0x53_0000;
            c.g[V0][0] &= 1;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x8b8));
            c.g[A0][0] = 0xffff_ffff_ffff_ffbf;
            c.g[V1][0] = oc923_lw(c, S0, 0x18);
            c.g[V0][0] = sx(r32(c, V0) << 6);
            c.g[V1][0] &= c.g[A0][0];
            c.g[V1][0] |= c.g[V0][0];
            let v = c.g[V1][0];
            oc923_sw(c, S0, 0x18, v);
            c.g[A0][0] = oc923_lw(c, S1, 0x6e4);
            call_ptr(c, 0x0039_a738, 0x001a_e70c)?;
            c.g[V1][0] = oc923_lw(c, S0, 0x18);
            c.g[V0][0] &= 1;
            c.g[A0][0] = 0xffff_ffff_ffff_feff;
            c.g[V0][0] = sx(r32(c, V0) << 8);
            c.g[V1][0] &= c.g[A0][0];
            c.g[V1][0] |= c.g[V0][0];
            let v = c.g[V1][0];
            oc923_sw(c, S0, 0x18, v);
            c.g[A0][0] = oc923_lw(c, S1, 0x6ec);
            call_ptr(c, 0x0039_a738, 0x001a_e730)?;
            c.g[V1][0] = oc923_lw(c, S0, 0x18);
            c.g[A0][0] = 0xffff_ffff_ffff_ff7f;
            c.g[V0][0] &= 1;
            c.g[V1][0] &= c.g[A0][0];
            c.g[V0][0] = sx(r32(c, V0) << 7);
            c.g[V1][0] |= c.g[V0][0];
            let v = c.g[V1][0];
            oc923_sw(c, S0, 0x18, v);
            call_ptr(c, 0x0014_dc80, 0x001a_e750)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dd58, 0x001a_e75c)?;
            c.g[A0][0] = oc923_lw(c, S1, 0x6e0);
            c.g[S0][0] = c.g[V0][0];
            call_ptr(c, 0x0039_a738, 0x001a_e768)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0014_4d98, 0x001a_e774)?;
            c.g[V1][0] = oc923_lw(c, S0, 0xc);
            c.g[A0][0] = oc923_lh(c, V1, 8);
            c.g[V0][0] = oc923_lw(c, V1, 0xc);
            let hook = r32(c, V0);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            call_ptr(c, hook, 0x001a_e788)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The power-ups list box: entry from bit 8 of the first option word when
/// `mpEventOptions[1]` is set, else entry 1.
fn oc923_set_power_ups_list(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 4;
    c.g[V0][0] = oc923_lw(c, S1, 0x6d8);
    c.g[V1][0] = oc923_lw(c, V0, 4);
    let unset = c.g[V1][0] == 0;
    c.g[V0][0] = 0x53_0000;
    if unset {
        c.cycles += 4;
        c.g[A0][0] = oc923_lw(c, S1, 0x6e4);
        c.g[A1][0] = 1;
        call_ptr(c, 0x0039_a7a8, 0x001a_f130)?;
    } else {
        c.cycles += 7;
        c.g[A0][0] = oc923_lw(c, S1, 0x6e4);
        c.g[A1][0] = oc923_lw(c, V0, 0x8d0);
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 8) as u32);
        c.g[A1][0] &= 1;
        call_ptr(c, 0x0039_a7a8, 0x001a_f11c)?;
    }
    c.g[V0][0] = oc923_lw(c, S1, 0x6d8);
    Ok(())
}

/// The multipliers list box: as the power-ups, bit 6 and option +0xc.
fn oc923_set_multipliers_list(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 3;
    c.g[V1][0] = oc923_lw(c, V0, 0xc);
    let unset = c.g[V1][0] == 0;
    c.g[V0][0] = 0x53_0000;
    if unset {
        c.cycles += 4;
        c.g[A0][0] = oc923_lw(c, S1, 0x6e8);
        c.g[A1][0] = 1;
        call_ptr(c, 0x0039_a7a8, 0x001a_f168)?;
    } else {
        c.cycles += 7;
        c.g[A0][0] = oc923_lw(c, S1, 0x6e8);
        c.g[A1][0] = oc923_lw(c, V0, 0x8d0);
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 6) as u32);
        c.g[A1][0] &= 1;
        call_ptr(c, 0x0039_a7a8, 0x001a_f154)?;
    }
    c.g[V0][0] = oc923_lw(c, S1, 0x6d8);
    Ok(())
}

/// The point-icons list box: as the power-ups, bit 7 and option +8.
fn oc923_set_point_icons_list(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 3;
    c.g[V1][0] = oc923_lw(c, V0, 8);
    let unset = c.g[V1][0] == 0;
    c.g[V0][0] = 0x53_0000;
    if unset {
        c.cycles += 4;
        c.g[A0][0] = oc923_lw(c, S1, 0x6ec);
        c.g[A1][0] = 1;
        call_ptr(c, 0x0039_a7a8, 0x001a_f1a0)?;
        c.g[V0][0] = 0x53_0000;
    } else {
        c.cycles += 7;
        c.g[A0][0] = oc923_lw(c, S1, 0x6ec);
        c.g[A1][0] = oc923_lw(c, V0, 0x8d0);
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 7) as u32);
        c.g[A1][0] &= 1;
        call_ptr(c, 0x0039_a7a8, 0x001a_f18c)?;
        c.g[V0][0] = 0x53_0000;
    }
    Ok(())
}

/// `void cFEStateChallengePopup_InitSettingLists(self, all)` (0x1af098): sets
/// the four list boxes (power-ups, multipliers, point icons, CPU riders) to
/// the current event options. With `all` set they all go to their defaults.
/// The CPU riders follow the event's AI count, or a global when there is no
/// game event.
pub fn c_fe_state_challenge_popup_init_setting_lists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x001a_f0b8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x001a_f0c4)?;
        let all = c.g[S0][0] != 0;
        c.g[S2][0] = c.g[V0][0];
        if all {
            // `all`: the three option lists go to entry 1.
            c.cycles += 11;
            c.g[A0][0] = oc923_lw(c, S1, 0x6e4);
            c.g[A1][0] = 1;
            call_ptr(c, 0x0039_a7a8, 0x001a_f0d8)?;
            c.g[A0][0] = oc923_lw(c, S1, 0x6e8);
            c.g[A1][0] = 1;
            call_ptr(c, 0x0039_a7a8, 0x001a_f0e4)?;
            c.g[A0][0] = oc923_lw(c, S1, 0x6ec);
            c.g[A1][0] = 1;
            call_ptr(c, 0x0039_a7a8, 0x001a_f0f0)?;
            c.g[A0][0] = oc923_lw(c, S1, 0x6e0);
        } else {
            oc923_set_power_ups_list(c)?;
            oc923_set_multipliers_list(c)?;
            oc923_set_point_icons_list(c)?;
            c.cycles += 3;
            c.g[V1][0] = oc923_lb(c, V0, 0x5c12);
            let no_event = c.g[V1][0] == 0;
            c.g[A1][0] = 0;
            if no_event {
                c.cycles += 4;
                c.g[A1][0] = oc923_lbu(c, GP, 0x1d9c);
                c.g[A0][0] = oc923_lw(c, S1, 0x6e0);
                call_ptr(c, 0x0039_a7a8, 0x001a_f1ec)?;
                frame.restore(c, S0, 0x30);
                c.cycles += 5;
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 8;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0014_4d98, 0x001a_f1b8)?;
            c.g[V1][0] = oc923_lw(c, S2, 0xc);
            c.g[A0][0] = oc923_lh(c, V1, 8);
            c.g[V0][0] = oc923_lw(c, V1, 0xc);
            let hook = r32(c, V0);
            c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
            call_ptr(c, hook, 0x001a_f1cc)?;
            c.g[A0][0] = oc923_lw(c, S1, 0x6e0);
        }
        // CPU riders: entry 0.
        c.cycles += 4;
        c.g[A1][0] = 0;
        call_ptr(c, 0x0039_a7a8, 0x001a_f1d8)?;
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void NetFE_FormatRoomName(const char* src, void* out)` (0x1b25e0): turns
/// a room file name `name.ext` into the display name: the known names
/// (four built-in room names) become their localised text, others are
/// copied; then the extension (with a leading space) is appended as
/// UTF-16. Does nothing when `src` is null.
pub fn net_fe_format_room_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x260);
        c.g[A3][0] = c.g[A0][0];
        frame.save(c, S2, 0x230);
        frame.save(c, S0, 0x250);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x240);
        frame.save(c, S3, 0x220);
        frame.save(c, S4, 0x210);
        let no_src = c.g[A3][0] == 0;
        frame.save_ra(c, 0x200);
        let leave = |c: &mut Cpu| {
            c.cycles += 8;
            frame.restore(c, S0, 0x250);
            frame.restore(c, S1, 0x240);
            frame.restore(c, S2, 0x230);
            frame.restore(c, S3, 0x220);
            frame.restore(c, S4, 0x210);
            frame.restore_ra(c, 0x200);
            frame.close(c);
        };
        if no_src {
            leave(c);
            return Ok(());
        }
        // name = src up to the first '.' (at most 127 bytes), at sp+0.
        c.cycles += 6;
        c.g[V1][0] = oc923_lb(c, A3, 0);
        c.g[A2][0] = 0;
        c.g[V0][0] = 0x2e;
        c.g[S3][0] = sx(sp(c).wrapping_add(0x80));
        let dot_first = c.g[V1][0] == c.g[V0][0];
        c.g[S4][0] = sx(sp(c).wrapping_add(0x100));
        if !dot_first {
            c.cycles += 4;
            c.g[T0][0] = 0x2e;
            c.g[A1][0] = c.g[A3][0];
            c.g[A0][0] = oc923_lbu(c, A1, 0);
            loop {
                c.cycles += 6;
                c.g[V1][0] = sx(sp(c).wrapping_add(r32(c, A2)));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                c.g[V0][0] = oc923_lb(c, A1, 0);
                let dot = c.g[V0][0] == c.g[T0][0];
                let ch = c.g[A0][0];
                oc923_sb(c, V1, 0, ch);
                if dot {
                    break;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A2][0] as i64) < 127) as u64;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = oc923_lbu(c, A1, 0);
                    if event_due(c) {
                        deliver_event_here(c, 0x001b_2630);
                    }
                    continue;
                }
                break;
            }
        }
        // NUL-terminate the name; ext = " " + the rest of src (sp+0x80).
        c.cycles += 9;
        c.g[V0][0] = sx(sp(c).wrapping_add(r32(c, A2)));
        c.g[V1][0] = 0x20;
        oc923_sb(c, V0, 0, 0);
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
        let space = c.g[V1][0];
        Addr(sp(c).wrapping_add(0x80)).sb(c, space);
        c.g[V1][0] = sx(r32(c, A3).wrapping_add(r32(c, A2)));
        c.g[V0][0] = oc923_lb(c, V1, 0);
        let no_ext = c.g[V0][0] == 0;
        c.g[A0][0] = 1;
        if !no_ext {
            c.cycles += 4;
            c.g[A2][0] = c.g[S3][0];
            c.g[A1][0] = c.g[V1][0];
            c.g[V1][0] = oc923_lbu(c, V1, 0);
            loop {
                c.cycles += 7;
                c.g[V0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                let ch = c.g[V1][0];
                oc923_sb(c, V0, 0, ch);
                c.g[V0][0] = oc923_lb(c, A1, 0);
                let end = c.g[V0][0] == 0;
                c.g[V1][0] = c.g[A1][0];
                if end {
                    break;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < 128) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = oc923_lbu(c, V1, 0);
                    if event_due(c) {
                        deliver_event_here(c, 0x001b_2688);
                    }
                    continue;
                }
                break;
            }
        }
        // NUL-terminate the extension, then compare the name with the four
        // built-in room names (strcmp(sp, gp[-0x1690/-0x168c/-0x1688/-0x1684])).
        c.cycles += 7;
        c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, A0)));
        c.g[A1][0] = oc923_lw(c, GP, 0u32.wrapping_sub(0x1690));
        oc923_sb(c, V0, 0, 0);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x0041_65a8, 0x001b_26c4)?;
        // Text literal offset of the matching built-in name (Elite, Advanced,
        // Voice, Beginner); `None`: no built-in name matched.
        let mut text: Option<u32> = None;
        if c.g[V0][0] != 0 {
            c.g[A1][0] = oc923_lw(c, GP, 0u32.wrapping_sub(0x168c));
            c.cycles += 4;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0041_65a8, 0x001b_26e4)?;
            if c.g[V0][0] != 0 {
                c.g[A1][0] = oc923_lw(c, GP, 0u32.wrapping_sub(0x1688));
                c.cycles += 4;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0041_65a8, 0x001b_2704)?;
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = oc923_lw(c, GP, 0u32.wrapping_sub(0x1684));
                    c.cycles += 4;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x0041_65a8, 0x001b_2724)?;
                    let none = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S2][0];
                    if !none {
                        c.cycles += 3;
                        c.g[V0][0] = oc923_lw(c, GP, GP_WORLD);
                        text = Some(0x3ac0);
                    }
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = oc923_lw(c, GP, GP_WORLD);
                    text = Some(0x3aa8);
                }
            } else {
                c.cycles += 4;
                c.g[V0][0] = oc923_lw(c, GP, GP_WORLD);
                text = Some(0x3a90);
            }
        } else {
            c.cycles += 4;
            c.g[V0][0] = oc923_lw(c, GP, GP_WORLD);
            text = Some(0x3a78);
        }
        if let Some(offset) = text {
            // out = localised(name literal) (FELocale->getStringByHash).
            c.g[A0][0] = 0x46_0000;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(offset));
            c.cycles += 14;
            c.g[S0][0] = oc923_lw(c, V0, 0x8c);
            c.g[S1][0] = oc923_lw(c, S0, 4);
            c.g[V0][0] = oc923_lh(c, S1, 0x20);
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
            call_ptr(c, 0x0031_7670, 0x001b_274c)?;
            c.g[V1][0] = oc923_lw(c, S1, 0x24);
            c.g[A0][0] = c.g[S0][0];
            let method = r32(c, V1);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, method, 0x001b_275c)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x002c_2508, 0x001b_2768)?;
            c.g[A1][0] = c.g[S3][0];
        } else {
            // Unknown name: copy it (func_002c2540(out, name)).
            c.cycles += 3;
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x002c_2540, 0x001b_2778)?;
            c.g[A1][0] = c.g[S3][0];
        }
        // out += UIAsciiToUnicode(sp+0x100, ext) ; func_002c2688(out, wext).
        c.cycles += 5;
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x0039_7ad8, 0x001b_2784)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S4][0];
        call_ptr(c, 0x002c_2688, 0x001b_2790)?;
        leave(c);
        Ok(())
    })
}

/// `void cFEStateNetConfig_dtor(self, flags)` (0x1b4018): resets the vtable,
/// frees the three auth-error strings and the popup text, deletes the shared
/// memory-card object (global at `gp - 0x1578`) and runs the base-class
/// destructor.
pub fn c_fe_state_net_config_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x47_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x7500));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let vtable = c.g[V0][0];
        oc923_sw(c, S0, 8, vtable);
        c.g[A0][0] = oc923_lw(c, S0, 0x710);
        let none = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !none {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x001b_404c)?;
            oc923_sw(c, S0, 0x710, 0);
        }
        c.cycles += 3;
        c.g[A0][0] = oc923_lw(c, S0, 0x714);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = oc923_lw(c, S0, 0x718);
        } else {
            c.cycles += 4;
            call_ptr(c, 0x0031_7e98, 0x001b_4064)?;
            oc923_sw(c, S0, 0x714, 0);
            c.g[A0][0] = oc923_lw(c, S0, 0x718);
        }
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.g[A0][0] = oc923_lw(c, S0, 0x704);
        } else {
            c.cycles += 4;
            call_ptr(c, 0x0031_7e98, 0x001b_407c)?;
            oc923_sw(c, S0, 0x718, 0);
            c.g[A0][0] = oc923_lw(c, S0, 0x704);
        }
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.g[A0][0] = oc923_lw(c, GP, 0u32.wrapping_sub(0x1578));
        } else {
            c.cycles += 4;
            call_ptr(c, 0x0031_7e98, 0x001b_4094)?;
            oc923_sw(c, S0, 0x704, 0);
            c.g[A0][0] = oc923_lw(c, GP, 0u32.wrapping_sub(0x1578));
        }
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.g[A0][0] = c.g[S0][0];
        } else {
            c.cycles += 4;
            call_ptr(c, 0x002c_2300, 0x001b_40ac)?;
            oc923_sw(c, GP, 0u32.wrapping_sub(0x1578), 0);
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 7;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001a_85d0, 0x001b_40bc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes (they act only on machines
    /// whose RAM holds [`MAGIC`] there) and the forced-result table.
    const CTRL: u32 = DATA + 0x20_2000;
    const MAGIC: u32 = 0x0923_0923;
    /// Per fake: `{count, values[6], calls made}` (0x20 bytes).
    const FORCE: u32 = CTRL + 0x20;

    const F_260F80: u32 = 0;
    const F_261460: u32 = 1;
    const F_2613C8: u32 = 2;
    const F_25B800: u32 = 3;
    const F_262938: u32 = 4;
    const F_262A38: u32 = 5;
    const F_263128: u32 = 6;
    const F_317670: u32 = 7;
    const F_39D860: u32 = 8;
    const F_417828: u32 = 9;
    const F_3A0C50: u32 = 10;
    const F_317D70: u32 = 11;
    const F_1A77A0: u32 = 12;
    const F_1A9090: u32 = 14;
    const F_1CE3C8: u32 = 15;
    const F_1CE468: u32 = 16;
    const F_1A97B8: u32 = 17;
    const F_1AFE08: u32 = 18;
    const F_3E6448: u32 = 19;
    const F_3182D8: u32 = 20;
    const F_318448: u32 = 21;
    const F_318260: u32 = 22;
    const F_1ADDD0: u32 = 23;
    const F_14DD58: u32 = 24;
    const F_39A738: u32 = 25;
    const F_14DC80: u32 = 26;
    const F_144D98: u32 = 27;
    const F_39A7A8: u32 = 28;
    const F_4165A8: u32 = 29;
    const F_2C2508: u32 = 30;
    const F_2C2540: u32 = 31;
    const F_397AD8: u32 = 32;
    const F_2C2688: u32 = 33;
    const F_317E98: u32 = 34;
    const F_2C2300: u32 = 35;
    const F_1A85D0: u32 = 36;

    /// Gated stand-ins for the real callees in the function table (they
    /// would run away on test data): log the call like the machine's miss
    /// fake, and return the forced value of the k-th call if the test set
    /// one. Gate off, the slot runs the body it held.
    macro_rules! oc923_fakes {
        ($(($fake:ident, $shard:ident, $slot:ident, $gen:ident, $addr:expr, $idx:expr)),* $(,)?) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(CTRL) != MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), ee::$shard::$gen);
                    }
                    let ret = c.g[RA][0] as u32;
                    fakes::fake_call(c, $addr);
                    if c.pc == ret {
                        let slot = FORCE + 0x20 * $idx;
                        let n = c.bus.read32(slot);
                        if n != 0 {
                            let k = c.bus.read32(slot + 0x1c);
                            c.bus.write32(slot + 0x1c, k + 1);
                            let v = c.bus.read32(slot + 4 + 4 * (k % n));
                            c.g[V0][0] = sx(v);
                        }
                    }
                }
            )*

            /// Hook the callee slots once per process.
            fn oc923_install_fakes() {
                $( fakes::hook(addr_of_mut!(ee::$shard::$slot), $fake, fakes::prev_cell($fake)); )*
            }
        };
    }

    oc923_fakes! {
        (oc923_fake_260f80, s06, OVERRIDE_00260F80, f_00260f80__generated, 0x0026_0f80, F_260F80),
        (oc923_fake_261460, s06, OVERRIDE_00261460, f_00261460__generated, 0x0026_1460, F_261460),
        (oc923_fake_2613c8, s06, OVERRIDE_002613C8, f_002613c8__generated, 0x0026_13c8, F_2613C8),
        (oc923_fake_25b800, s06, OVERRIDE_0025B800, f_0025b800__generated, 0x0025_b800, F_25B800),
        (oc923_fake_262938, s07, OVERRIDE_00262938, f_00262938__generated, 0x0026_2938, F_262938),
        (oc923_fake_262a38, s07, OVERRIDE_00262A38, f_00262a38__generated, 0x0026_2a38, F_262A38),
        (oc923_fake_263128, s07, OVERRIDE_00263128, f_00263128__generated, 0x0026_3128, F_263128),
        (oc923_fake_317670, s10, OVERRIDE_00317670, f_00317670__generated, 0x0031_7670, F_317670),
        (oc923_fake_39d860, s13, OVERRIDE_0039D860, f_0039d860__generated, 0x0039_d860, F_39D860),
        (oc923_fake_417828, s15, OVERRIDE_00417828, f_00417828__generated, 0x0041_7828, F_417828),
        (oc923_fake_3a0c50, s13, OVERRIDE_003A0C50, f_003a0c50__generated, 0x003a_0c50, F_3A0C50),
        (oc923_fake_317d70, s10, OVERRIDE_00317D70, f_00317d70__generated, 0x0031_7d70, F_317D70),
        (oc923_fake_1a77a0, s03, OVERRIDE_001A77A0, f_001a77a0__generated, 0x001a_77a0, F_1A77A0),
        (oc923_fake_1a9090, s03, OVERRIDE_001A9090, f_001a9090__generated, 0x001a_9090, F_1A9090),
        (oc923_fake_1ce3c8, s04, OVERRIDE_001CE3C8, f_001ce3c8__generated, 0x001c_e3c8, F_1CE3C8),
        (oc923_fake_1ce468, s04, OVERRIDE_001CE468, f_001ce468__generated, 0x001c_e468, F_1CE468),
        (oc923_fake_1a97b8, s03, OVERRIDE_001A97B8, f_001a97b8__generated, 0x001a_97b8, F_1A97B8),
        (oc923_fake_1afe08, s03, OVERRIDE_001AFE08, f_001afe08__generated, 0x001a_fe08, F_1AFE08),
        (oc923_fake_3e6448, s14, OVERRIDE_003E6448, f_003e6448__generated, 0x003e_6448, F_3E6448),
        (oc923_fake_3182d8, s10, OVERRIDE_003182D8, f_003182d8__generated, 0x0031_82d8, F_3182D8),
        (oc923_fake_318448, s10, OVERRIDE_00318448, f_00318448__generated, 0x0031_8448, F_318448),
        (oc923_fake_318260, s10, OVERRIDE_00318260, f_00318260__generated, 0x0031_8260, F_318260),
        (oc923_fake_1addd0, s03, OVERRIDE_001ADDD0, f_001addd0__generated, 0x001a_ddd0, F_1ADDD0),
        (oc923_fake_14dd58, s01, OVERRIDE_0014DD58, f_0014dd58__generated, 0x0014_dd58, F_14DD58),
        (oc923_fake_39a738, s13, OVERRIDE_0039A738, f_0039a738__generated, 0x0039_a738, F_39A738),
        (oc923_fake_14dc80, s01, OVERRIDE_0014DC80, f_0014dc80__generated, 0x0014_dc80, F_14DC80),
        (oc923_fake_144d98, s01, OVERRIDE_00144D98, f_00144d98__generated, 0x0014_4d98, F_144D98),
        (oc923_fake_39a7a8, s13, OVERRIDE_0039A7A8, f_0039a7a8__generated, 0x0039_a7a8, F_39A7A8),
        (oc923_fake_4165a8, s15, OVERRIDE_004165A8, f_004165a8__generated, 0x0041_65a8, F_4165A8),
        (oc923_fake_2c2508, s08, OVERRIDE_002C2508, f_002c2508__generated, 0x002c_2508, F_2C2508),
        (oc923_fake_2c2540, s08, OVERRIDE_002C2540, f_002c2540__generated, 0x002c_2540, F_2C2540),
        (oc923_fake_397ad8, s13, OVERRIDE_00397AD8, f_00397ad8__generated, 0x0039_7ad8, F_397AD8),
        (oc923_fake_2c2688, s08, OVERRIDE_002C2688, f_002c2688__generated, 0x002c_2688, F_2C2688),
        (oc923_fake_317e98, s10, OVERRIDE_00317E98, f_00317e98__generated, 0x0031_7e98, F_317E98),
        (oc923_fake_2c2300, s08, OVERRIDE_002C2300, f_002c2300__generated, 0x002c_2300, F_2C2300),
        (oc923_fake_1a85d0, s03, OVERRIDE_001A85D0, f_001a85d0__generated, 0x001a_85d0, F_1A85D0),
    }

    /// Force the results of fake `idx`: the k-th call returns
    /// `values[k % len]` (at most six values).
    fn force(c: &mut Cpu, idx: u32, values: &[u32]) {
        let slot = FORCE + 0x20 * idx;
        c.bus.write32(slot, values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            c.bus.write32(slot + 4 + 4 * i as u32, *v);
        }
        c.bus.write32(slot + 0x1c, 0);
    }

    /// A random choice below `n`.
    fn pick(r: &mut Rng, n: u32) -> u32 {
        r.next_u32() % n
    }

    /// A machine with the gate open: the in-table callees are fakes.
    fn gated(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc923_install_fakes();
        let r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(CTRL, MAGIC);
        r
    }

    /// Fixture map (all inside DATA..DATA+0xfffff).
    const SELF: u32 = DATA + 0x1000;
    const VT: u32 = DATA + 0x2000;
    const APP: u32 = DATA + 0x3000;
    const BUDDY: u32 = DATA + 0x3100;
    const LOBBY: u32 = DATA + 0x3300;
    const TEXT: u32 = DATA + 0x3400;
    const TEXT_VT: u32 = DATA + 0x3500;
    const SCREEN: u32 = DATA + 0x3600;
    const NAME: u32 = DATA + 0x3700;
    const WORLD: u32 = DATA + 0x3800;
    const OWNER: u32 = DATA + 0x3900;
    const TABLE: u32 = DATA + 0x3a00;
    const MSG: u32 = DATA + 0x3b00;
    const KEYBOARD: u32 = DATA + 0x3c00;
    const STATE: u32 = DATA + 0x4400;
    const CHILD: u32 = DATA + 0x4500;
    const POPUP_OBJ: u32 = DATA + 0x4600;
    const OBJ_A: u32 = DATA + 0x4800;
    const OBJ_B: u32 = DATA + 0x4900;
    const VT_A: u32 = DATA + 0x4a00;
    const VT_B: u32 = DATA + 0x4b00;
    const OPTIONS: u32 = DATA + 0x4c00;
    const SRC: u32 = DATA + 0x6000;
    const OUT: u32 = DATA + 0x8000;

    /// A vtable slot `{delta: i16 at +slot, target: u32 at +slot+4}`.
    fn vt_slot(c: &mut Cpu, vt: u32, off: u32, delta: u16, target: u32) {
        c.bus.write16(vt + off, delta);
        c.bus.write32(vt + off + 4, target);
    }

    /// `NetFEState_UpdateMessageCount`: the early exits (no screen, hidden,
    /// polled less than a second ago), the message and challenge hint
    /// frames, and the text object shown (count > 0) or hidden.
    #[test]
    fn net_fe_state_update_message_count_matches() {
        check(
            ee::s03::f_001aab68__generated,
            net_fe_state_update_message_count,
            300,
            &|c, seed| {
                let mut r = gated(c, seed, 0x0923_0001, 12);
                c.g[A0][0] = SELF as u64;
                c.bus
                    .write32(SELF + 0x40, if pick(&mut r, 8) == 0 { 0 } else { SCREEN });
                c.bus.write32(SELF + 8, VT);
                // v29(): a fake method (non-zero) or a real leaf returning 0.
                let target = if pick(&mut r, 6) == 0 {
                    0x0014_06c0
                } else {
                    fakes::FAKE
                };
                vt_slot(c, VT, 0xe8, 0x10 * pick(&mut r, 3) as u16, target);
                let now = r.next_u32();
                let elapsed = match pick(&mut r, 6) {
                    0 => 0,
                    1 => 1,
                    2 => 59,
                    3 => 60,
                    4 => 1000,
                    _ => r.next_u32(),
                };
                c.bus.write32(GAME_GP + 0x2a74, APP);
                c.bus.write32(APP + 0x18, now);
                c.bus.write32(SELF + 0x4c, now.wrapping_sub(elapsed));
                let hint = |r: &mut Rng| if pick(r, 2) == 0 { 0 } else { r.next_u32() | 1 };
                let h50 = hint(&mut r);
                let h54 = hint(&mut r);
                c.bus.write32(SELF + 0x50, h50);
                c.bus.write32(SELF + 0x54, h54);
                c.bus.write32(
                    GAME_GP + 0x238,
                    if pick(&mut r, 5) == 0 { 0 } else { BUDDY },
                );
                let begin = r.next_u32();
                let span = [0u32, 4, 0x40, 0xffff_fffc, 0xffff_ffc0][pick(&mut r, 5) as usize];
                c.bus.write32(BUDDY + 0xbc, begin);
                c.bus.write32(BUDDY + 0xc0, begin.wrapping_add(span));
                c.bus.write32(GAME_GP.wrapping_sub(0xc8), LOBBY);
                force(c, F_261460, &[pick(&mut r, 2)]);
                let n = [0xffff_ffffu32, 0, 1, 3, 100][pick(&mut r, 5) as usize];
                force(c, F_262938, &[n]);
                let text = if pick(&mut r, 5) == 0 { 0 } else { TEXT };
                force(c, F_39D860, &[text]);
                c.bus.write32(TEXT + 8, TEXT_VT);
                vt_slot(
                    c,
                    TEXT_VT,
                    0x48,
                    0x10 * pick(&mut r, 3) as u16,
                    fakes::FAKE + 0x10,
                );
            },
        );
    }

    /// `cFEStateNetConfig_GetErrorText`: every error code of the search
    /// tree, its neighbours (to take every compare both ways) and random
    /// ones.
    #[test]
    fn c_fe_state_net_config_get_error_text_matches() {
        const CODES: [u32; 11] = [
            0x2d64_6966,
            0x2d64_6576,
            0x2d63_6667,
            0x2d69_6e69,
            0x2d74_6f6e,
            0x2d62_7379,
            0x2d63_6f6e,
            0x2d3f_3f3f,
            0x2d61_7468,
            0x2d64_6e73,
            0x7469_6d65,
        ];
        check(
            ee::s03::f_001aaf90__generated,
            c_fe_state_net_config_get_error_text,
            300,
            &|c, seed| {
                let mut r = gated(c, seed, 0x0923_0002, 3);
                c.g[A1][0] = MSG as u64;
                let code = match seed % 4 {
                    0 | 1 => CODES[(seed / 4) as usize % 11],
                    2 => CODES[(seed / 4) as usize % 11].wrapping_add(1),
                    _ if seed % 8 == 3 => CODES[(seed / 8) as usize % 11].wrapping_sub(1),
                    _ => [0, 0xffff_ffff, 0x8000_0000, r.next_u32()][(seed / 8) as usize % 4],
                };
                c.bus.write32(MSG + 8, code);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
                c.bus.write32(WORLD + 0x8c, OWNER);
                c.bus.write32(OWNER + 4, TABLE);
                vt_slot(c, TABLE, 0x20, 0x10 * pick(&mut r, 3) as u16, fakes::FAKE);
            },
        );
    }

    /// Shared fixture of the two messenger popups: the BuddyMan session
    /// (or none), the helpers' booleans, the engine and its state stack.
    fn messenger(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(SELF + 0x10, DATA + 0x5000);
        c.bus.write32(SELF + 0x40, SCREEN);
        c.bus.write32(SCREEN + 0xd0, STATE);
        c.bus.write32(STATE + 0x10, 0x1234 + pick(r, 0x100) * 4);
        c.bus.write32(SELF + 0x69c, KEYBOARD);
        c.bus
            .write32(GAME_GP + 0x238, if pick(r, 4) == 0 { 0 } else { BUDDY });
        c.bus.write32(GAME_GP.wrapping_sub(0xc8), LOBBY);
        force(c, F_261460, &[pick(r, 2)]);
    }

    /// `NetFEState_ReadOrSendMessage`: the session set-up paths, then the
    /// read-message popup (a message from `name`) or the keyboard.
    #[test]
    fn net_fe_state_read_or_send_message_matches() {
        check(
            ee::s03::f_001abb30__generated,
            net_fe_state_read_or_send_message,
            300,
            &|c, seed| {
                let mut r = gated(c, seed, 0x0923_0003, 8);
                messenger(c, &mut r);
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = NAME as u64;
                force(c, F_262A38, &[pick(&mut r, 2)]);
            },
        );
    }

    /// `NetFEState_OpenChallengePopup`: the session set-up paths, then the
    /// challenge-received popup or the new-challenge popup, each with and
    /// without `fromPopup`.
    #[test]
    fn net_fe_state_open_challenge_popup_matches() {
        check(
            ee::s03::f_001abcd0__generated,
            net_fe_state_open_challenge_popup,
            300,
            &|c, seed| {
                let mut r = gated(c, seed, 0x0923_0004, 12);
                messenger(c, &mut r);
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = NAME as u64;
                c.g[A2][0] = CHILD as u64;
                c.g[A3][0] = pick(&mut r, 0x20) as u64;
                c.g[T0][0] = pick(&mut r, 2) as u64;
                force(c, F_263128, &[pick(&mut r, 2)]);
                force(c, F_1AFE08, &[POPUP_OBJ]);
                force(c, F_1ADDD0, &[POPUP_OBJ]);
            },
        );
    }

    /// The setting-list object, the world's interface objects and their
    /// hook vtables of the two challenge-popup setting functions.
    fn popup_settings(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
        c.bus.write32(WORLD + 0x78, DATA + 0x5100);
        c.bus.write32(OBJ_A + 0xc, VT_A);
        c.bus.write32(OBJ_B + 0xc, VT_B);
        vt_slot(c, VT_A, 8, 0x10 * pick(r, 3) as u16, fakes::FAKE);
        vt_slot(c, VT_B, 8, 0x10 * pick(r, 3) as u16, fakes::FAKE + 0x10);
    }

    /// `cFEStateChallengePopup_ApplySettings`: with and without the setting
    /// list object, every list-box value pattern (the option bits).
    #[test]
    fn c_fe_state_challenge_popup_apply_settings_matches() {
        check(
            ee::s03::f_001ae680__generated,
            c_fe_state_challenge_popup_apply_settings,
            300,
            &|c, seed| {
                let mut r = gated(c, seed, 0x0923_0005, 12);
                popup_settings(c, &mut r);
                c.g[A0][0] = SELF as u64;
                c.bus
                    .write32(SELF + 0x40, if pick(&mut r, 4) == 0 { 0 } else { SCREEN });
                c.bus.write32(SELF + 0x6dc, DATA + 0x5200);
                force(c, F_14DD58, &[OBJ_A, OBJ_B]);
                let pool = [0u32, 1, 2, 3, 0xffff_ffff, 0x80, 0x100];
                let mut values = [0u32; 5];
                for v in values.iter_mut() {
                    *v = pool[pick(&mut r, 7) as usize];
                }
                force(c, F_39A738, &values);
                c.bus.write32(0x0053_08b8 + 0x18, r.next_u32());
            },
        );
    }

    /// `cFEStateChallengePopup_InitSettingLists`: `all` set, and the
    /// option-driven lists with each option object field set or clear, with
    /// and without the game event.
    #[test]
    fn c_fe_state_challenge_popup_init_setting_lists_matches() {
        check(
            ee::s03::f_001af098__generated,
            c_fe_state_challenge_popup_init_setting_lists,
            300,
            &|c, seed| {
                let mut r = gated(c, seed, 0x0923_0006, 8);
                popup_settings(c, &mut r);
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = (pick(&mut r, 3) == 0) as u64;
                c.bus.write32(SELF + 0x6d8, OPTIONS);
                for off in [4u32, 8, 0xc] {
                    let v = pick(&mut r, 2);
                    c.bus.write32(OPTIONS + off, v);
                }
                c.bus.write32(0x0053_08d0, r.next_u32());
                c.bus.write8(0x0053_5c12, pick(&mut r, 2) as u8);
                c.bus.write8(GAME_GP + 0x1d9c, pick(&mut r, 5) as u8);
                force(c, F_14DD58, &[OBJ_A]);
            },
        );
    }

    /// `NetFE_FormatRoomName`: no source, names with and without a dot, an
    /// empty extension, over-long names and extensions, and each of the four
    /// built-in name comparisons matching (or none).
    #[test]
    fn net_fe_format_room_name_matches() {
        check(
            ee::s03::f_001b25e0__generated,
            net_fe_format_room_name,
            300,
            &|c, seed| {
                let mut r = gated(c, seed, 0x0923_0007, 8);
                let text: &[u8] = match seed % 7 {
                    0 => b"elite.txt\0",
                    1 => b".foo\0",
                    2 => b"abc\0",
                    3 => b"name.\0",
                    4 => b"voice.ext\0",
                    5 => b"x.\xe4\xf6\0",
                    _ => b"beginner.room\0",
                };
                for (i, b) in text.iter().enumerate() {
                    c.bus.write8(SRC + i as u32, *b);
                }
                if seed % 7 == 5 || seed % 11 == 0 {
                    // an extension longer than the buffer
                    let start = SRC + 2;
                    for i in 0..160 {
                        c.bus.write8(start + i, b'y');
                    }
                    c.bus.write8(SRC + 1, b'.');
                }
                if seed % 13 == 0 {
                    // a name without a dot, longer than the buffer
                    for i in 0..160 {
                        c.bus.write8(SRC + i, b'n');
                    }
                }
                c.g[A0][0] = if seed % 17 == 0 { 0 } else { SRC as u64 };
                c.g[A1][0] = OUT as u64;
                // Which of the four strcmp calls reports "equal" (none: 4).
                let hit = pick(&mut r, 5) as usize;
                let mut results = [1u32; 4];
                if hit < 4 {
                    results[hit] = 0;
                }
                force(c, F_4165A8, &results);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
                c.bus.write32(WORLD + 0x8c, OWNER);
                c.bus.write32(OWNER + 4, TABLE);
                vt_slot(c, TABLE, 0x20, 0x10 * pick(&mut r, 3) as u16, 0);
                c.bus.write32(TABLE + 0x24, fakes::FAKE);
            },
        );
    }

    /// `cFEStateNetConfig_dtor`: each of the four strings and the shared
    /// memory-card object present or absent.
    #[test]
    fn c_fe_state_net_config_dtor_matches() {
        check(
            ee::s03::f_001b4018__generated,
            c_fe_state_net_config_dtor,
            200,
            &|c, seed| {
                let mut r = gated(c, seed, 0x0923_0008, 7);
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = r.next_u32() as u64 & 7;
                for off in [0x710u32, 0x714, 0x718, 0x704] {
                    let v = if pick(&mut r, 2) == 0 {
                        0
                    } else {
                        0x0090_1000 + off
                    };
                    c.bus.write32(SELF + off, v);
                }
                let card = if pick(&mut r, 2) == 0 { 0 } else { 0x0090_2000 };
                c.bus.write32(GAME_GP.wrapping_sub(0x1578), card);
            },
        );
    }
}
