//! Staging module oc866 (wave wave250): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc866`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001a_6e70,
        c_fe_state_net_messenger_remove_stale_names,
        "cFEStateNetMessenger_RemoveStaleNames",
    ),
    (
        0x001a_71b0,
        c_fe_state_net_messenger_set_row,
        "cFEStateNetMessenger_SetRow",
    ),
    (
        0x001a_7390,
        c_fe_state_net_messenger_show_buddy_presence,
        "cFEStateNetMessenger_ShowBuddyPresence",
    ),
    (
        0x001a_77a0,
        c_fe_state_read_message_popup_ctor,
        "cFEStateReadMessagePopup_ctor",
    ),
    (
        0x001a_7848,
        c_fe_state_read_message_popup_dtor,
        "cFEStateReadMessagePopup_dtor",
    ),
];

/// `lw off(base reg)`, sign-extended.
#[inline(always)]
fn oc866_lw(c: &mut Cpu, base: usize, off: u32) -> u64 {
    let a = (c.g[base][0] as u32).wrapping_add(off);
    c.lw(a)
}

/// The BuddyMan singleton pointer (`lw gp+0x238`).
const OC866_BUDDY_GP: u32 = 0x238;

/// The shared "make sure the BuddyMan is up" run of blocks: ask
/// `BuddyMan_IsConnected`, reconnect when it says no, and call
/// `LobbyMan_Disconnect` (on the UI manager at `gp-0xc8`) when there is still no
/// BuddyMan. `v0` is left as the generated code leaves it; `a0` ends as the
/// BuddyMan pointer. `r_*` are the return addresses of the three calls.
fn oc866_buddy_ready(c: &mut Cpu, r_connected: u32, r_reconnect: u32, r_disconnect: u32) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = oc866_lw(c, GP, OC866_BUDDY_GP);
    call_ptr(c, 0x0026_1460, r_connected)?; // BuddyMan_IsConnected
    let connected = c.g[V0][0] != 0;
    c.g[V0][0] = oc866_lw(c, GP, OC866_BUDDY_GP);
    if !connected {
        c.cycles += 3;
        c.g[A0][0] = oc866_lw(c, GP, OC866_BUDDY_GP);
        call_ptr(c, 0x0026_13c8, r_reconnect)?; // BuddyMan_Reconnect
        c.g[V0][0] = oc866_lw(c, GP, OC866_BUDDY_GP);
    }
    c.cycles += 2;
    let have = c.g[V0][0] != 0;
    c.g[A0][0] = oc866_lw(c, GP, OC866_BUDDY_GP);
    if !have {
        c.cycles += 3;
        c.g[A0][0] = oc866_lw(c, GP, 0u32.wrapping_sub(0xc8));
        call_ptr(c, 0x0025_b800, r_disconnect)?; // LobbyMan_Disconnect
        c.g[A0][0] = oc866_lw(c, GP, OC866_BUDDY_GP);
    }
    Ok(())
}

/// `void cFEStateNetMessenger_RemoveStaleNames(cFEStateNetMessenger* self)`
/// (0x1a6e70): walk the queued name list (`+0x6e8 .. +0x6ec`, name pointers)
/// from the back; every name the BuddyMan roster no longer knows
/// (`BuddyMan_FindRosterEntryNoCase` < 0) is removed by moving the tail down
/// one slot and shrinking the list. Loop head (event check) is 0x1a6eb0.
pub fn c_fe_state_net_messenger_remove_stale_names(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = oc866_lw(c, S0, 0x6ec);
        c.g[V1][0] = oc866_lw(c, S0, 0x6e8);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        let empty = (c.g[S1][0] as i64) < 0;
        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6e4));
        if !empty {
            c.cycles += 1;
            c.g[V0][0] = oc866_lw(c, S0, 0x6e8);
            loop {
                c.cycles += 5;
                c.g[S2][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[V1][0] = oc866_lw(c, GP, OC866_BUDDY_GP);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                let have = c.g[V1][0] != 0;
                c.g[S3][0] = c.lw(c.g[V0][0] as u32);
                if !have {
                    c.cycles += 2;
                    call_ptr(c, 0x0026_0f80, 0x001a_6ecc)?; // BuddyMan_Create
                }
                oc866_buddy_ready(c, 0x001a_6ed4, 0x001a_6ee4, 0x001a_6ef8)?;
                c.cycles += 4;
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x0026_24f8, 0x001a_6f04)?; // BuddyMan_FindRosterEntryNoCase
                if (c.g[V0][0] as i64) >= 0 {
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = oc866_lw(c, S0, 0x6e8);
                    c.g[V1][0] = oc866_lw(c, S0, 0x6ec);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                    let at_end = c.g[V0][0] == c.g[V1][0];
                    c.g[A1][0] = c.g[V0][0];
                    if !at_end {
                        c.cycles += 5;
                        c.g[A2][0] = oc866_lw(c, S4, 8);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 2) as u32);
                        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
                        call_ptr(c, 0x0041_610c, 0x001a_6f38)?; // memmove
                    }
                    c.cycles += 4;
                    c.g[V0][0] = oc866_lw(c, S0, 0x6ec);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x6ec), c.g[V0][0]);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                }
                c.cycles += 2;
                if (c.g[S1][0] as i64) >= 0 {
                    c.g[V0][0] = oc866_lw(c, S0, 0x6e8);
                    if event_due(c) {
                        deliver_event_here(c, 0x001a_6eb0);
                    }
                    continue;
                }
                break;
            }
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

/// `void cFEStateNetMessenger_SetRow(self, u32 row, char* name, u32 arg)`
/// (0x1a71b0): fill one roster row (row < 15). With no name the row gets
/// icon 3, the name text of `0x4a1390` (empty) and colour 1; otherwise the
/// BuddyMan decides the icon: challenger 2, blocked 0, has-message-from 1,
/// else 3, and the row name is set from `name`.
pub fn c_fe_state_net_messenger_set_row(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = (c.g[S0][0] < 0xf) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[S3][0] = c.g[A3][0];
        // The exits through 0x1a7374 cost one more cycle than the ones that
        // return straight to the epilogue.
        let mut late_exit = true;
        'body: {
            if !in_range {
                break 'body;
            }
            c.cycles += 2;
            let no_name = c.g[S2][0] == 0;
            c.g[V0][0] = oc866_lw(c, GP, OC866_BUDDY_GP);
            if no_name {
                c.cycles += 14;
                c.g[A2][0] = 3;
                call_ptr(c, 0x001a_63c8, 0x001a_71f0)?; // SetRowIcon
                c.g[A2][0] = 0x4a_0000;
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1390));
                c.g[A1][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S3][0];
                call_ptr(c, 0x001a_6618, 0x001a_7208)?; // SetRowName
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 1;
                call_ptr(c, 0x001a_66a8, 0x001a_7218)?; // SetRowColour
                late_exit = false;
                break 'body;
            }
            // (ready-check ret addresses, BuddyMan query, its ret, icon on a hit,
            // ret of the icon call): challenger, blocked, message-from.
            'checks: {
                // Challenger.
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    call_ptr(c, 0x0026_0f80, 0x001a_7230)?; // BuddyMan_Create
                }
                oc866_buddy_ready(c, 0x001a_7238, 0x001a_7248, 0x001a_725c)?;
                c.cycles += 4;
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x0026_3128, 0x001a_7268)?; // BuddyMan_HasChallenger
                let hit = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S1][0];
                if hit {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = 2;
                    call_ptr(c, 0x001a_63c8, 0x001a_727c)?;
                    c.g[A0][0] = c.g[S1][0];
                    break 'checks;
                }
                // Blocked.
                c.cycles += 3;
                c.g[V0][0] = oc866_lw(c, GP, OC866_BUDDY_GP);
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    call_ptr(c, 0x0026_0f80, 0x001a_7298)?;
                }
                oc866_buddy_ready(c, 0x001a_72a0, 0x001a_72b0, 0x001a_72c4)?;
                c.cycles += 4;
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x0026_23b0, 0x001a_72d0)?; // BuddyMan_IsBlocked
                let hit = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S1][0];
                if hit {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = 0;
                    call_ptr(c, 0x001a_63c8, 0x001a_72e4)?;
                    c.g[A0][0] = c.g[S1][0];
                    break 'checks;
                }
                // Has a message from.
                c.cycles += 3;
                c.g[V0][0] = oc866_lw(c, GP, OC866_BUDDY_GP);
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    call_ptr(c, 0x0026_0f80, 0x001a_7300)?;
                }
                oc866_buddy_ready(c, 0x001a_7308, 0x001a_7318, 0x001a_732c)?;
                c.cycles += 4;
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x0026_2a38, 0x001a_7338)?; // BuddyMan_HasMessageFrom
                let hit = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S1][0];
                if hit {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = 1;
                    call_ptr(c, 0x001a_63c8, 0x001a_734c)?;
                    c.g[A0][0] = c.g[S1][0];
                    break 'checks;
                }
                c.cycles += 4;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 3;
                call_ptr(c, 0x001a_63c8, 0x001a_7360)?;
                c.g[A0][0] = c.g[S1][0];
            }
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S3][0];
            call_ptr(c, 0x001a_6618, 0x001a_7374)?; // SetRowName
        }
        if late_exit {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `0x1a7718` tail of `ShowBuddyPresence`: set the wide text from the
/// scratch buffer at `sp` (`a0` is already the text object).
fn oc866_presence_set_wide(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A1][0] = c.g[SP][0];
    call_ptr(c, 0x003a_0e90, 0x001a_7720) // UIText_SetWideString
}

/// The part of `ShowBuddyPresence` at `0x1a76e0`: ask the UI object of the
/// world (`world+0x8c`) for a string (hash of the key in `a0`), then format it
/// into the buffer at `sp` with `UStr_Sprintf` and set it as the wide text.
/// `v0` is the world pointer.
fn oc866_presence_world_text(c: &mut Cpu) -> Flow {
    c.cycles += 14;
    c.g[S0][0] = oc866_lw(c, V0, 0x8c);
    c.g[S1][0] = oc866_lw(c, S0, 4);
    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    call_ptr(c, 0x0031_7670, 0x001a_76f4)?; // StrHash
    c.g[V1][0] = oc866_lw(c, S1, 0x24);
    c.g[A0][0] = c.g[S0][0];
    let target = c.g[V1][0] as u32;
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, target, 0x001a_7704)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[SP][0];
    c.g[A2][0] = c.g[S6][0];
    call_ptr(c, 0x002c_26d0, 0x001a_7714)?; // UStr_Sprintf
    c.g[A0][0] = c.g[S3][0];
    oc866_presence_set_wide(c)
}

/// `void cFEStateNetMessenger_ShowBuddyPresence(self, tags, bool plain)`
/// (0x1a7390): show one buddy's presence line on the row's text object. The
/// buddy's tag string (`tags+0x54`) gives a status code (0..10, selecting
/// through a jump table at `0x461970` one of eleven presence texts) and three
/// strings; with `plain` set only the status text is applied, otherwise the
/// name/room line is composed and shown first. Resume entries (`pc != entry`)
/// stay with the generated `__entries` twin.
pub fn c_fe_state_net_messenger_show_buddy_presence(c: &mut Cpu) {
    if c.pc != 0x001a_7390 {
        return ee::s03::f_001a7390__entries(c);
    }
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x390);
        frame.save(c, S0, 0x380);
        frame.save(c, S4, 0x340);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S7, 0x310);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x370);
        frame.save(c, S2, 0x360);
        frame.save(c, S3, 0x350);
        frame.save(c, S5, 0x330);
        frame.save(c, S6, 0x320);
        frame.save_ra(c, 0x300);
        c.g[V0][0] = oc866_lw(c, S0, 0x40);
        let no_screen = c.g[V0][0] == 0;
        c.g[S7][0] = c.g[A2][0];
        // Exits through 0x1a7774 cost one cycle more than the direct ones.
        let mut late_exit = true;
        'body: {
            if no_screen {
                break 'body;
            }
            c.cycles += 2;
            let no_tags = c.g[S4][0] == 0;
            c.g[A0][0] = 0x46_0000;
            if no_tags {
                break 'body;
            }
            c.cycles += 15;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x17a8));
            call_ptr(c, 0x0031_7670, 0x001a_73dc)?; // StrHash
            c.g[A0][0] = oc866_lw(c, S0, 0x40);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001a_73e8)?; // UIScreen_FindObject
            c.g[A0][0] = 0x46_0000;
            c.g[S2][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1558));
            call_ptr(c, 0x0031_7670, 0x001a_73f8)?;
            c.g[A0][0] = oc866_lw(c, S0, 0x40);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001a_7404)?;
            c.g[S3][0] = c.g[V0][0];
            let no_text = c.g[S3][0] == 0;
            c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x54));
            if no_text {
                break 'body;
            }
            // Parse the tag string: status code, then three strings.
            c.cycles += 47;
            c.g[A1][0] = 0x4a_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a28));
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003e_b6d8, 0x001a_7420)?; // TagFieldFind
            c.g[A1][0] = 0xffff_ffff_ffff_ffff;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x003e_cf30, 0x001a_742c)?; // TagFieldGetNumber
            c.g[A1][0] = 0x4a_0000;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
            c.g[S5][0] = c.g[V0][0];
            call_ptr(c, 0x003e_b6d8, 0x001a_7440)?;
            c.g[A1][0] = 0xffff_ffff_ffff_ffff;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x003e_cf30, 0x001a_744c)?;
            c.g[A1][0] = 0x4a_0000;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a38));
            call_ptr(c, 0x003e_b6d8, 0x001a_745c)?;
            c.g[S0][0] = 0x4a_0000;
            c.g[A1][0] = sx(sp(c).wrapping_add(0xa0));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1408));
            c.g[A2][0] = 0x20;
            c.g[A3][0] = c.g[S0][0];
            c.g[S6][0] = c.g[A1][0];
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x003e_d378, 0x001a_747c)?; // TagFieldGetString
            c.g[A1][0] = 0x4a_0000;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a40));
            call_ptr(c, 0x003e_b6d8, 0x001a_748c)?;
            c.g[A1][0] = sx(sp(c).wrapping_add(0xc0));
            c.g[A2][0] = 0x20;
            c.g[A3][0] = c.g[S0][0];
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x003e_d378, 0x001a_74a0)?;
            c.g[A1][0] = 0x4a_0000;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a48));
            call_ptr(c, 0x003e_b6d8, 0x001a_74b0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A3][0] = c.g[S0][0];
            c.g[A1][0] = sx(sp(c).wrapping_add(0xe0));
            c.g[A2][0] = 0x20;
            call_ptr(c, 0x003e_d378, 0x001a_74c4)?;
            let plain = c.g[S7][0] != 0;
            c.g[V0][0] = (c.g[S5][0] < 0xb) as u64;
            if !plain {
                // Compose the name/room line and show it.
                c.cycles += 45;
                c.g[S0][0] = sx(sp(c).wrapping_add(0x100));
                c.g[A1][0] = c.g[S4][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0041_66f4, 0x001a_74dc)?; // strcpy
                c.g[V1][0] = 0x4a_0000;
                c.g[A0][0] = c.g[S0][0];
                c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1390));
                c.g[A1][0] = c.g[S4][0];
                call_ptr(c, 0x0041_62d0, 0x001a_74f0)?; // strcat
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0041_62d0, 0x001a_74fc)?;
                c.g[S1][0] = sx(sp(c).wrapping_add(0x160));
                c.g[A2][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0x60;
                call_ptr(c, 0x003e_fa48, 0x001a_7510)?; // Utf8DecodeToUCS2
                c.g[V1][0] = oc866_lw(c, S2, 8);
                c.g[A1][0] = sx(sp(c).wrapping_add(0x2e0));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                c.g[V0][0] = oc866_lw(c, V1, 0xa4);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x001a_7528)?; // text object virtual (size query)
                let a = (c.g[S2][0] as u32).wrapping_add(0x57);
                c.ldl(2, a);
                let a = (c.g[S2][0] as u32).wrapping_add(0x50);
                c.ldr(2, a);
                c.g[V1][0] = oc866_lw(c, S2, 0x58);
                c.sdl(2, sp(c).wrapping_add(0x2f7));
                c.sdr(2, sp(c).wrapping_add(0x2f0));
                c.sw(sp(c).wrapping_add(0x2f8), c.g[V1][0]);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x003a_04f0, 0x001a_7548)?; // UIText_GetFont
                c.g[S0][0] = sx(sp(c).wrapping_add(0x220));
                c.lwc1(12, sp(c).wrapping_add(0x2f0));
                c.lwc1(13, sp(c).wrapping_add(0x2e0));
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = 0x30;
                call_ptr(c, 0x0039_2680, 0x001a_7568)?; // cFont_textSmartLimit1
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a_0e90, 0x001a_7578)?; // UIText_SetWideString
                let none = c.g[S1][0] == 0;
                c.g[A0][0] = c.g[S3][0];
                c.cycles += 4;
                if none {
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x003a_0c50, 0x001a_7598)?; // UIText_SetString
                } else {
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x003a_0e90, 0x001a_7588)?;
                }
                late_exit = false;
                break 'body;
            }
            // The status text for the code in s5.
            c.cycles += 2;
            let bad_code = c.g[V0][0] == 0;
            c.g[V0][0] = 0x46_0000;
            if bad_code {
                break 'body;
            }
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[S5][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1970));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = oc866_lw(c, V1, 0);
            let target = c.g[A0][0] as u32;
            // Jump table of the original: every case sets a0 (the text key) and
            // joins at 0x1a7610 (the first seven), or does its own work.
            let key = match target {
                0x001a_75c0 => Some((3, 0x17e0)),
                0x001a_75cc => Some((3, 0x1868)),
                0x001a_75d8 => Some((3, 0x1880)),
                0x001a_75e4 => Some((3, 0x1898)),
                0x001a_75f0 => Some((3, 0x18b0)),
                0x001a_75fc => Some((3, 0x18c8)),
                0x001a_7608 => Some((2, 0x18e0)),
                _ => None,
            };
            if let Some((cost, off)) = key {
                c.cycles += cost;
                c.g[A0][0] = 0x46_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(off));
                c.cycles += 7;
                call_ptr(c, 0x0031_7670, 0x001a_7618)?; // StrHash
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1000, 0x001a_7624)?; // UIText_SetTextId
                late_exit = false;
                break 'body;
            }
            match target {
                0x001a_762c => {
                    c.cycles += 38;
                    c.g[V0][0] = oc866_lw(c, GP, 0u32.wrapping_sub(0x848));
                    c.g[A0][0] = 0x46_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18f8));
                    c.g[S0][0] = oc866_lw(c, V0, 0x8c);
                    c.g[S1][0] = oc866_lw(c, S0, 4);
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    call_ptr(c, 0x0031_7670, 0x001a_764c)?;
                    c.g[V1][0] = oc866_lw(c, S1, 0x24);
                    c.g[A0][0] = c.g[S0][0];
                    let vcall = c.g[V1][0] as u32;
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, vcall, 0x001a_765c)?;
                    c.g[S2][0] = sx(sp(c).wrapping_add(0x190));
                    c.g[V1][0] = 0x4a_0000;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A1][0] = sx(sp(c).wrapping_add(0x180));
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1390));
                    c.g[A3][0] = c.lb(c.g[A2][0] as u32);
                    c.g[T0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(1));
                    c.sb(sp(c).wrapping_add(0x180), c.g[A3][0]);
                    c.sb(sp(c).wrapping_add(0x181), c.g[T0][0]);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0039_7ad8, 0x001a_7688)?; // UI_AsciiToWide
                    c.g[S0][0] = sx(sp(c).wrapping_add(0x100));
                    c.g[A0][0] = c.g[S6][0];
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x001b_25e0, 0x001a_7698)?; // NetFE_FormatRoomName
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x002c_2508, 0x001a_76a4)?; // UStr_Copy
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x002c_2688, 0x001a_76b0)?; // UStr_Cat
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x002c_2688, 0x001a_76bc)?;
                    c.g[A0][0] = c.g[S3][0];
                    oc866_presence_set_wide(c)?;
                    late_exit = false;
                    break 'body;
                }
                0x001a_76c4 => {
                    c.cycles += 4;
                    c.g[V0][0] = oc866_lw(c, GP, 0u32.wrapping_sub(0x848));
                    c.g[A0][0] = 0x46_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1910));
                    oc866_presence_world_text(c)?;
                    late_exit = false;
                    break 'body;
                }
                0x001a_76d4 => {
                    c.cycles += 3;
                    c.g[V0][0] = oc866_lw(c, GP, 0u32.wrapping_sub(0x848));
                    c.g[A0][0] = 0x46_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1930));
                    oc866_presence_world_text(c)?;
                    late_exit = false;
                    break 'body;
                }
                0x001a_7728 => {
                    c.cycles += 19;
                    c.g[V0][0] = oc866_lw(c, GP, 0u32.wrapping_sub(0x848));
                    c.g[A0][0] = 0x46_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1950));
                    c.g[S0][0] = oc866_lw(c, V0, 0x8c);
                    c.g[S1][0] = oc866_lw(c, S0, 4);
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    call_ptr(c, 0x0031_7670, 0x001a_7748)?;
                    c.g[V1][0] = oc866_lw(c, S1, 0x24);
                    c.g[A0][0] = c.g[S0][0];
                    let vcall = c.g[V1][0] as u32;
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, vcall, 0x001a_7758)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S6][0];
                    call_ptr(c, 0x002c_26d0, 0x001a_7768)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x003a_0e90, 0x001a_7774)?;
                    break 'body;
                }
                other => {
                    c.jump_out(other, 0x001a_7390, 0x001a_77a0);
                    return Ok(());
                }
            }
        }
        if late_exit {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x380);
        c.cycles += 10;
        frame.restore(c, S1, 0x370);
        frame.restore(c, S2, 0x360);
        frame.restore(c, S3, 0x350);
        frame.restore(c, S4, 0x340);
        frame.restore(c, S5, 0x330);
        frame.restore(c, S6, 0x320);
        frame.restore(c, S7, 0x310);
        frame.restore_ra(c, 0x300);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateReadMessagePopup::cFEStateReadMessagePopup(this, a1, a2, char*
/// text)` (0x1a77a0): run `NetFEState_ctor`, install vtable `0x467e18`,
/// construct the `cBXString` at `+0x6d0` from `text`, set `+0xc = 0x49`,
/// clear the message vector (`+0x6dc..+0x6e4`), and store `+0x6e8 = 1` and
/// 16 bytes holding four 1.0f at `+0x6ec`.
pub fn c_fe_state_read_message_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A3][0];
        call_ptr(c, 0x001a_8500, 0x001a_77bc)?; // NetFEState_ctor
        c.g[V1][0] = 0x46_0000;
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7e18));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6d0));
        c.sw((c.g[S0][0] as u32).wrapping_add(8), c.g[V1][0]);
        call_ptr(c, 0x0031_82d8, 0x001a_77d4)?; // cBXString_ctor_cstr
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[V0][0] = 1;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6e8), c.g[V0][0]);
        c.g[V1][0] = 0x49;
        c.swc1(0, sp(c).wrapping_add(0x1c));
        c.g[V0][0] = c.g[S0][0];
        c.swc1(0, sp(c).wrapping_add(0x10));
        c.swc1(0, sp(c).wrapping_add(0x14));
        c.swc1(0, sp(c).wrapping_add(0x18));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6d4), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6dc), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6e0), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6e4), 0);
        c.ldl(3, sp(c).wrapping_add(0x17));
        c.ldr(3, sp(c).wrapping_add(0x10));
        c.ldl(4, sp(c).wrapping_add(0x1f));
        c.ldr(4, sp(c).wrapping_add(0x18));
        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x6f3));
        c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x6ec));
        c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0x6fb));
        c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0x6f4));
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateReadMessagePopup::~cFEStateReadMessagePopup(this, int flags)`
/// (0x1a7848): reset the vtable to `0x467e18`, destroy every message object in
/// the vector at `+0x6dc..+0x6e0` (virtual destructor with flag 3, last
/// first; the loop head with its event check is 0x1a7890), erase the vector's
/// storage, free the buffer (`__builtin_delete` for a one-element capacity,
/// `EA_MemFree` otherwise), destroy the `cBXString` at `+0x6d0`, and run
/// `NetFEState_dtor(this, flags)`.
pub fn c_fe_state_read_message_popup_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x46_0000;
        frame.save(c, S3, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7e18));
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0);
        c.sw((c.g[S3][0] as u32).wrapping_add(8), c.g[V0][0]);
        c.g[A0][0] = oc866_lw(c, S3, 0x6dc);
        c.g[V1][0] = oc866_lw(c, S3, 0x6e0);
        let empty = c.g[A0][0] == c.g[V1][0];
        c.g[S4][0] = c.g[A1][0];
        if !empty {
            c.cycles += 3;
            c.g[V0][0] = oc866_lw(c, S3, 0x6e0);
            c.g[A2][0] = oc866_lw(c, V0, 0u32.wrapping_sub(4));
            loop {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
                let skip = c.g[A2][0] == 0;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x6e0), c.g[V0][0]);
                if !skip {
                    c.cycles += 6;
                    c.g[V0][0] = oc866_lw(c, A2, 0xc);
                    c.g[A1][0] = 3;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(8));
                    c.g[V1][0] = oc866_lw(c, V0, 0xc);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001a_78b4)?;
                }
                c.cycles += 4;
                c.g[V1][0] = oc866_lw(c, S3, 0x6dc);
                c.g[V0][0] = oc866_lw(c, S3, 0x6e0);
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A2][0] = oc866_lw(c, V0, 0u32.wrapping_sub(4));
                    if event_due(c) {
                        deliver_event_here(c, 0x001a_7890);
                    }
                    continue;
                }
                break;
            }
        }
        // Erase the (now empty) range, then free the storage.
        c.cycles += 20;
        c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6d8));
        c.g[S0][0] = oc866_lw(c, S3, 0x6e0);
        c.g[A2][0] = oc866_lw(c, S2, 8);
        c.g[S1][0] = oc866_lw(c, S3, 0x6dc);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[S0][0] as u32));
        c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 2) as u32);
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        call_ptr(c, 0x0041_610c, 0x001a_78ec)?; // memmove
        c.g[V1][0] = oc866_lw(c, S3, 0x6e0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[S1][0] as u32));
        c.g[A0][0] = oc866_lw(c, S3, 0x6dc);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S0][0] as u32));
        c.g[V0][0] = oc866_lw(c, S3, 0x6e4);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x6e0), c.g[V1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        c.g[A0][0] = oc866_lw(c, S2, 4);
        let no_storage = c.g[A0][0] == 0;
        c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        if no_storage {
            c.cycles += 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 4;
                call_ptr(c, 0x0031_7e50, 0x001a_7928)?; // __builtin_delete
            } else {
                c.cycles += 2;
                call_ptr(c, 0x0031_7e98, 0x001a_7938)?; // EA_MemFree
                c.cycles += 1;
            }
        }
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6d0));
        c.cycles += 13;
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001a_7944)?; // cBXString_dtor
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S4][0];
        call_ptr(c, 0x001a_85d0, 0x001a_7950)?; // NetFEState_dtor
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::{DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the oc866 fakes (`DATA+0x200000` and up is free).
    pub(crate) const OC866_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC866_MAGIC: u32 = 0x4f43_3836;

    /// Slot `k` of the gated fakes: word 0 = mask of calls that run the real
    /// callee instead (bit n = n-th call), +4 = calls so far, +8 = mask of calls
    /// whose `v0` is forced, +0x10.. = the forced values.
    pub(crate) fn oc866_cfg(c: &mut Cpu, k: u32, real_mask: u32, force_mask: u32, forced: &[u32]) {
        let cfg = OC866_CTRL + 0x100 + 0x40 * k;
        c.bus.write32(cfg, real_mask);
        c.bus.write32(cfg + 4, 0);
        c.bus.write32(cfg + 8, force_mask);
        for (i, v) in forced.iter().enumerate() {
            c.bus.write32(cfg + 0x10 + 4 * i as u32, *v);
        }
    }

    /// Turn the gated fakes on for this machine.
    pub(crate) fn oc866_on(c: &mut Cpu) {
        oc866_install_fakes();
        c.bus.write32(OC866_CTRL, OC866_MAGIC);
    }

    macro_rules! oc866_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $k:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC866_CTRL) != OC866_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    let cfg = OC866_CTRL + 0x100 + 0x40 * $k;
                    let n = c.bus.read32(cfg + 4);
                    c.bus.write32(cfg + 4, n + 1);
                    if n < 32 && (c.bus.read32(cfg) >> n) & 1 != 0 {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    fakes::fake_call(c, $addr);
                    if c.pc == ra && n < 8 && (c.bus.read32(cfg + 8) >> n) & 1 != 0 {
                        c.g[V0][0] = c.bus.read32(cfg + 0x10 + 4 * n) as u64;
                    }
                }
            )*
            pub(crate) fn oc866_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc866_fakes!(
        (
            oc866_f_str_hash,
            OC866_PREV_STR_HASH,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            0
        ),
        (
            oc866_f_find_object,
            OC866_PREV_FIND_OBJECT,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860,
            1
        ),
        (
            oc866_f_tag_find,
            OC866_PREV_TAG_FIND,
            ee::s14::OVERRIDE_003EB6D8,
            ee::s14::f_003eb6d8__generated,
            0x003e_b6d8,
            2
        ),
        (
            oc866_f_tag_number,
            OC866_PREV_TAG_NUMBER,
            ee::s14::OVERRIDE_003ECF30,
            ee::s14::f_003ecf30__generated,
            0x003e_cf30,
            3
        ),
        (
            oc866_f_tag_string,
            OC866_PREV_TAG_STRING,
            ee::s14::OVERRIDE_003ED378,
            ee::s14::f_003ed378__generated,
            0x003e_d378,
            4
        ),
        (
            oc866_f_strcpy,
            OC866_PREV_STRCPY,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x0041_66f4,
            5
        ),
        (
            oc866_f_strcat,
            OC866_PREV_STRCAT,
            ee::s15::OVERRIDE_004162D0,
            ee::s15::f_004162d0__generated,
            0x0041_62d0,
            6
        ),
        (
            oc866_f_utf8,
            OC866_PREV_UTF8,
            ee::s14::OVERRIDE_003EFA48,
            ee::s14::f_003efa48__generated,
            0x003e_fa48,
            7
        ),
        (
            oc866_f_get_font,
            OC866_PREV_GET_FONT,
            ee::s13::OVERRIDE_003A04F0,
            ee::s13::f_003a04f0__generated,
            0x003a_04f0,
            8
        ),
        (
            oc866_f_set_wide,
            OC866_PREV_SET_WIDE,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a_0e90,
            9
        ),
        (
            oc866_f_set_string,
            OC866_PREV_SET_STRING,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50,
            10
        ),
        (
            oc866_f_set_text_id,
            OC866_PREV_SET_TEXT_ID,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000,
            11
        ),
        (
            oc866_f_ascii_to_wide,
            OC866_PREV_ASCII_TO_WIDE,
            ee::s13::OVERRIDE_00397AD8,
            ee::s13::f_00397ad8__generated,
            0x0039_7ad8,
            12
        ),
        (
            oc866_f_ustr_copy,
            OC866_PREV_USTR_COPY,
            ee::s08::OVERRIDE_002C2508,
            ee::s08::f_002c2508__generated,
            0x002c_2508,
            13
        ),
        (
            oc866_f_ustr_cat,
            OC866_PREV_USTR_CAT,
            ee::s08::OVERRIDE_002C2688,
            ee::s08::f_002c2688__generated,
            0x002c_2688,
            14
        ),
        (
            oc866_f_ustr_sprintf,
            OC866_PREV_USTR_SPRINTF,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c_26d0,
            15
        ),
        (
            oc866_f_find_roster,
            OC866_PREV_FIND_ROSTER,
            ee::s07::OVERRIDE_002624F8,
            ee::s07::f_002624f8__generated,
            0x0026_24f8,
            16
        ),
        (
            oc866_f_has_challenger,
            OC866_PREV_HAS_CHALLENGER,
            ee::s07::OVERRIDE_00263128,
            ee::s07::f_00263128__generated,
            0x0026_3128,
            17
        ),
        (
            oc866_f_is_blocked,
            OC866_PREV_IS_BLOCKED,
            ee::s07::OVERRIDE_002623B0,
            ee::s07::f_002623b0__generated,
            0x0026_23b0,
            18
        ),
        (
            oc866_f_has_message,
            OC866_PREV_HAS_MESSAGE,
            ee::s07::OVERRIDE_00262A38,
            ee::s07::f_00262a38__generated,
            0x0026_2a38,
            19
        ),
        (
            oc866_f_delete,
            OC866_PREV_DELETE,
            ee::s10::OVERRIDE_00317E50,
            ee::s10::f_00317e50__generated,
            0x0031_7e50,
            20
        ),
        (
            oc866_f_mem_free,
            OC866_PREV_MEM_FREE,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98,
            21
        ),
        (
            oc866_f_bxstring_dtor,
            OC866_PREV_BXSTRING_DTOR,
            ee::s10::OVERRIDE_00318260,
            ee::s10::f_00318260__generated,
            0x0031_8260,
            22
        ),
    );

    /// A BuddyMan fixture like the `GetBuddyStatus` test's: dummy BELibrary and
    /// world so the real BuddyMan callees terminate, the BuddyMan object at
    /// `gp+0x238` and the UI manager at `gp-0xc8`.
    pub(crate) fn oc866_buddy_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        let _ = machine(c, seed, salt, calls);
        let be = DATA + 0x5_2000;
        let world = DATA + 0x5_0000;
        let rm = DATA + 0x5_1000;
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), be);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x7c, rm);
        c.bus.write32(world + 0x84, rm);
        c.bus.write32(rm + 0x84, rm);
        c.bus.write32(rm + 0x10, 0);
        c.bus.write32(GAME_GP + 0x238, DATA + 0x1000);
        c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x1100);
    }

    /// 0..=4 queued names; per call, `BuddyMan_FindRosterEntryNoCase` is either
    /// faked (positive: the name stays) or real (no roster: -1, the name is
    /// removed); entry 0 of the list may be a null name.
    #[test]
    fn c_fe_state_net_messenger_remove_stale_names_matches() {
        check(
            ee::s03::f_001a6e70__generated,
            c_fe_state_net_messenger_remove_stale_names,
            80,
            &|c, seed| {
                oc866_buddy_base(c, seed, 0x1a6e70, 4);
                oc866_on(c);
                oc866_cfg(c, 16, seed / 5 & 0xf, 0, &[]);
                let this = DATA;
                let n = seed % 5;
                let names = DATA + 0x2000;
                for i in 0..n {
                    let name = if seed % 7 == 0 {
                        0
                    } else {
                        DATA + 0x2800 + 0x40 * i
                    };
                    c.bus.write32(names + 4 * i, name);
                }
                c.bus.write32(this + 0x6e8, names);
                c.bus.write32(this + 0x6ec, names + 4 * n);
                c.g[A0][0] = sx(this);
                c.pc = 0x001a_6e70;
            },
        );
    }

    /// `seed % 5`: no name; challenger; blocked; message-from; none of them
    /// (the three BuddyMan queries are forced). Every 7th seed has a row >= 15.
    #[test]
    fn c_fe_state_net_messenger_set_row_matches() {
        check(
            ee::s03::f_001a71b0__generated,
            c_fe_state_net_messenger_set_row,
            60,
            &|c, seed| {
                oc866_buddy_base(c, seed, 0x1a71b0, 3);
                oc866_on(c);
                let sel = seed % 5;
                oc866_cfg(c, 17, 0, 1, &[(sel == 1) as u32]);
                oc866_cfg(c, 18, 0, 1, &[(sel == 2) as u32]);
                oc866_cfg(c, 19, 0, 1, &[(sel == 3) as u32]);
                let this = DATA;
                // A null screen: the real SetRowIcon/Name/Colour return at once.
                c.bus.write32(this + 0x40, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 7 == 0 {
                    0x14
                } else {
                    (seed % 0xf) as u64
                };
                c.g[A2][0] = if sel == 0 { 0 } else { sx(DATA + 0x2400) };
                c.g[A3][0] = (seed % 0x100) as u64;
                c.pc = 0x001a_71b0;
            },
        );
    }

    /// `seed % 14` is the status code (11..13 are out of range), `seed / 14 % 2`
    /// the `plain` flag, with a missing screen, tag string or second text object
    /// now and then, and an invalid jump-table entry for code 10.
    #[test]
    fn c_fe_state_net_messenger_show_buddy_presence_matches() {
        check(
            ee::s03::f_001a7390__generated,
            c_fe_state_net_messenger_show_buddy_presence,
            84,
            &|c, seed| {
                let _ = machine(c, seed, 0x1a7390, 6);
                oc866_on(c);
                let code = seed % 14;
                let this = DATA + 0x1000;
                let screen = DATA + 0x1400;
                let text = DATA + 0x3100;
                let text2 = DATA + 0x3300;
                let font = DATA + 0x3400;
                let vt = DATA + 0x3200;
                oc866_cfg(c, 1, 0, 3, &[text, if seed % 9 == 0 { 0 } else { text2 }]);
                oc866_cfg(c, 3, 0, 1, &[code]);
                oc866_cfg(c, 8, 0, 1, &[font]);
                c.bus
                    .write32(this + 0x40, if seed % 17 == 5 { 0 } else { screen });
                // The text object's virtual size query.
                c.bus.write32(text + 8, vt);
                c.bus.write16(vt + 0xa0, 0);
                c.bus.write32(vt + 0xa4, FAKE);
                // The world's UI object (`world+0x8c`) with a string virtual.
                let world = DATA + 0x5000;
                let ui = DATA + 0x5100;
                let ui_vt = DATA + 0x5200;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x8c, ui);
                c.bus.write32(ui + 4, ui_vt);
                c.bus.write16(ui_vt + 0x20, 0);
                c.bus.write32(ui_vt + 0x24, FAKE);
                // The status jump table.
                const TABLE: [u32; 11] = [
                    0x001a_75c0,
                    0x001a_75cc,
                    0x001a_75d8,
                    0x001a_75e4,
                    0x001a_75f0,
                    0x001a_75fc,
                    0x001a_7608,
                    0x001a_762c,
                    0x001a_76c4,
                    0x001a_76d4,
                    0x001a_7728,
                ];
                for (i, t) in TABLE.iter().enumerate() {
                    c.bus.write32(0x0046_1970 + 4 * i as u32, *t);
                }
                if seed % 7 == 3 {
                    c.bus.write32(0x0046_1970 + 4 * 10, 0x001a_7000);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 19 == 7 { 0 } else { sx(DATA + 0x2000) };
                c.g[A2][0] = ((seed / 14) % 2) as u64;
                c.pc = 0x001a_7390;
            },
        );
    }

    #[test]
    fn c_fe_state_read_message_popup_ctor_matches() {
        use crate::game::fe::feasyncfile::tests::net_fe_state_fixture;
        check(
            ee::s03::f_001a77a0__generated,
            c_fe_state_read_message_popup_ctor,
            40,
            &|c, seed| {
                net_fe_state_fixture(c, seed, 0x1a77a0);
                for (i, b) in b"hello\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x2000 + i as u32, *b);
                }
                c.g[A3][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x2000) };
                c.pc = 0x001a_77a0;
            },
        );
    }

    /// `seed % 4` messages (0..3, the first two null in some seeds), capacity of
    /// one or eight entries, an empty vector without storage every 5th seed;
    /// `flags` alternates.
    #[test]
    fn c_fe_state_read_message_popup_dtor_matches() {
        use crate::game::fe::feasyncfile::tests::{alloc_ready, clear_3, common, heap_block};
        check(
            ee::s03::f_001a7848__generated,
            c_fe_state_read_message_popup_dtor,
            60,
            &|c, seed| {
                let _ = common(c, seed, 0x1a7848, 3);
                alloc_ready(c);
                oc866_on(c);
                let this = heap_block(c, 0x800);
                clear_3(c, this, 0x200);
                c.bus.write32(this + 0x6d0, 0x0048_dcd4); // static empty cBXString
                let m = seed % 4;
                let arr = DATA + 0x6000;
                let vt = DATA + 0x7000;
                c.bus.write16(vt + 8, 0);
                c.bus.write32(vt + 0xc, FAKE);
                for i in 0..m {
                    let obj = DATA + 0x6800 + 0x40 * i;
                    c.bus.write32(obj + 0xc, vt);
                    c.bus
                        .write32(arr + 4 * i, if seed % 5 == 1 && i == 0 { 0 } else { obj });
                }
                if seed % 5 == 4 && m == 0 {
                    c.bus.write32(this + 0x6dc, 0);
                    c.bus.write32(this + 0x6e0, 0);
                    c.bus.write32(this + 0x6e4, 0);
                } else {
                    let cap = if seed % 2 == 0 { 1 } else { 8 };
                    c.bus.write32(this + 0x6dc, arr);
                    c.bus.write32(this + 0x6e0, arr + 4 * m);
                    c.bus.write32(this + 0x6e4, arr + 4 * cap.max(m));
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = ((seed / 3) % 2) as u64;
                c.pc = 0x001a_7848;
            },
        );
    }
}
