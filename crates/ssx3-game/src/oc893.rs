//! Staging module oc893 (wave wave253): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc893`.

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
        0x001b_cb58,
        fe_state_lobby_menu_ctor,
        "cFEStateLobbyMenu_ctor",
    ),
    (
        0x001b_ce18,
        fe_state_lobby_menu_update,
        "cFEStateLobbyMenu_Update",
    ),
    (
        0x001b_d540,
        fe_state_lobby_menu_request_join_room,
        "cFEStateLobbyMenu_RequestJoinRoom",
    ),
    (
        0x001b_d610,
        fe_state_lobby_menu_create_room,
        "cFEStateLobbyMenu_CreateRoom",
    ),
];

/// Lobby menu fields (offsets from `this`, a `cFEStateLobbyMenu`).
const LOBBY_VTABLE: u32 = 0x008;
const LOBBY_FC: u32 = 0x00c;
const LOBBY_PLAYER_TEXT_LAST: u32 = 0x6fc;
const LOBBY_EMPTY_STRING_B80: u32 = 0xb80;
const LOBBY_ROOM_NAME: u32 = 0xb80;
const LOBBY_PASSWORD_SET: u32 = 0x770;

/// Popup info fields (offsets from the popup object).
const POPUP_TYPE: u32 = 0x0bc;
const POPUP_MSG_TYPE: u32 = 0x0c0;
const POPUP_MSG_UI: u32 = 0x0c4;
const POPUP_MISC: u32 = 0x150;
const POPUP_RECEIVE_PREV: u32 = 0x164;

/// Message strings (rodata, `0x460000` + offset).
const STR_ENTERING_ROOM: u32 = 0x0046_4fa8;
const STR_UNIQUE_ROOM: u32 = 0x0046_4fc0;
const STR_SUBMITTING_CREATE_ROOM: u32 = 0x0046_4fd8;
const STR_BTN_OK: u32 = 0x0046_1618;

/// `cFEStateLobbyMenu` constructor (0x001bcb58): runs the net-state base
/// constructor, then sets the lobby vtable, the 0x3c field, the empty-string
/// slots, clears the ten player-text slots (`+0x6d8..=0x6fc`), the four
/// channel pair and list rows (`+0x708`, `+0x718`, `+0x730`, `+0x740`,
/// `+0x750`) and the room counters, and returns `this`.
pub fn fe_state_lobby_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.g[A2][0] = 0;
        let s = sp(c);
        c.sq(16, s.wrapping_add(0x10));
        c.sd(s, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001a_8500, 0x001b_cb70)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
        c.g[V0][0] = sx(0x0047_0000u32.wrapping_sub(0x5808));
        c.g[A0][0] = 0x3c;
        c.sw(this.wrapping_add(LOBBY_VTABLE), c.g[V0][0]);
        c.g[A1][0] = 0x9;
        c.sw(this.wrapping_add(LOBBY_EMPTY_STRING_B80), c.g[V1][0]);
        c.g[V0][0] = sx(this.wrapping_add(LOBBY_PLAYER_TEXT_LAST));
        c.sw(this.wrapping_add(LOBBY_FC), c.g[A0][0]);
        c.sw(this.wrapping_add(0x76c), c.g[V1][0]);
        c.sw(this.wrapping_add(0x778), 0);
        c.sw(this.wrapping_add(0x6d0), 0);
        c.sw(this.wrapping_add(0x6d4), 0);
        // Ten player-text slots, i = 9 down to 0 (+0x6fc down to +0x6d8).
        loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, 0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            let more = (c.g[A1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001b_cba8);
            }
        }
        c.cycles += 7;
        c.sw(this.wrapping_add(0x704), 0);
        c.g[A2][0] = sx(this.wrapping_add(0x750));
        c.g[A1][0] = sx(this.wrapping_add(0x740));
        c.g[A0][0] = sx(this.wrapping_add(0x730));
        c.g[V1][0] = sx(this.wrapping_add(0x718));
        c.g[V0][0] = sx(this.wrapping_add(0x708));
        c.g[A3][0] = 0x3;
        // Four rows of five words each, cleared together.
        loop {
            c.cycles += 12;
            c.sw(c.g[V0][0] as u32, 0);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
            c.sw(c.g[V1][0] as u32, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            c.sw(c.g[A0][0] as u32, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            c.sw(c.g[A1][0] as u32, 0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            c.sw(c.g[A2][0] as u32, 0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
            let more = (c.g[A3][0] as i64) >= 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001b_cbe0);
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S0][0];
        c.sw(this.wrapping_add(0x728), 0);
        c.sw(this.wrapping_add(0x72c), 0);
        let s = sp(c);
        c.lq(16, s.wrapping_add(0x10));
        c.g[RA][0] = c.ld(s);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(s.wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `void cFEStateLobbyMenu_Update(void)` (0x001bce18): the net-state update.
pub fn fe_state_lobby_menu_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x001a_8770, 0x001b_ce28)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RequestJoinRoom(this, roomName, password)` (0x001bd540): shows the
/// "entering room" popup, then records whether the room has a password
/// (`+0x770`, the password's first byte non-zero) and asks the lobby to join
/// it, passing the password or null. The room name is copied into
/// `mszRoomName` (`+0xb80`).
pub fn fe_state_lobby_menu_request_join_room(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
        let s = sp(c);
        c.sq(17, s.wrapping_add(0x30));
        c.sq(18, s.wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sq(19, s.wrapping_add(0x10));
        c.g[S2][0] = c.g[A2][0];
        c.sq(16, s.wrapping_add(0x40));
        c.sd(s, c.g[RA][0]);
        c.g[S3][0] = c.g[A1][0];
        call_ptr(c, 0x001a_9c50, 0x001b_d568)?;
        c.g[S0][0] = c.g[V0][0];
        let popup = c.g[S0][0] as u32;
        c.sw(popup.wrapping_add(POPUP_TYPE), 0);
        c.g[A0][0] = sx(STR_ENTERING_ROOM);
        call_ptr(c, 0x0031_7670, 0x001b_d57c)?;
        c.g[V1][0] = 3;
        c.g[A2][0] = 0x829;
        let popup = c.g[S0][0] as u32;
        c.sw(popup.wrapping_add(POPUP_RECEIVE_PREV), 0);
        c.g[A1][0] = c.g[S0][0];
        c.sw(popup.wrapping_add(POPUP_MSG_UI), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.sw(popup.wrapping_add(POPUP_MSG_TYPE), c.g[V1][0]);
        c.sw(popup.wrapping_add(POPUP_MISC), c.g[A2][0]);
        call_ptr(c, 0x001a_9bc0, 0x001b_d5a0)?;
        // Password: none (null or the empty string) clears the flag and
        // joins without one; otherwise the flag is set and it joins with it.
        let mut with_password = false;
        if c.g[S2][0] == 0 {
            let this = c.g[S1][0] as u32;
            c.sw(this.wrapping_add(LOBBY_PASSWORD_SET), 0);
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0041_6810, 0x001b_d5b0)?;
            let nonempty = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if nonempty {
                with_password = true;
            } else {
                c.cycles += 1;
                let this = c.g[S1][0] as u32;
                c.sw(this.wrapping_add(LOBBY_PASSWORD_SET), 0);
            }
        }
        if with_password {
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
            let this = c.g[S1][0] as u32;
            c.sw(this.wrapping_add(LOBBY_PASSWORD_SET), c.g[V0][0]);
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x0025_d860, 0x001b_d5e8)?;
        } else {
            c.cycles += 6;
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
            c.g[A2][0] = 0;
            call_ptr(c, 0x0025_d860, 0x001b_d5cc)?;
        }
        let this = c.g[S1][0] as u32;
        c.g[A0][0] = sx(this.wrapping_add(LOBBY_ROOM_NAME));
        c.cycles += 9;
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0031_84e8, 0x001b_d5f4)?;
        let s = sp(c);
        c.lq(16, s.wrapping_add(0x40));
        c.lq(17, s.wrapping_add(0x30));
        c.lq(18, s.wrapping_add(0x20));
        c.lq(19, s.wrapping_add(0x10));
        c.g[RA][0] = c.ld(s);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(s.wrapping_add(0x50));
        c.pc = ra;
        Ok(())
    })
}

/// `void CreateRoom(this, name, channel, extra)` (0x001bd610): an empty room
/// name shows the "unique room" error; otherwise the "submitting" popup
/// shows and the lobby creates the room on `channel` with `extra` (null when
/// `extra` is the empty string).
pub fn fe_state_lobby_menu_create_room(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x60));
        let s = sp(c);
        c.sq(18, s.wrapping_add(0x30));
        c.sq(19, s.wrapping_add(0x20));
        c.g[S2][0] = c.g[A0][0];
        c.sq(17, s.wrapping_add(0x40));
        c.g[S3][0] = c.g[A1][0];
        c.sq(20, s.wrapping_add(0x10));
        c.g[S1][0] = c.g[A3][0];
        c.sq(16, s.wrapping_add(0x50));
        c.g[S4][0] = c.g[A2][0];
        c.sd(s, c.g[RA][0]);
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0041_6810, 0x001b_d644)?;
        if c.g[V0][0] != 0 {
            create_room_submit(c)?;
        } else {
            create_room_unique_name(c)?;
        }
        let s = sp(c);
        c.lq(16, s.wrapping_add(0x50));
        c.cycles += 7;
        c.lq(17, s.wrapping_add(0x40));
        c.lq(18, s.wrapping_add(0x30));
        c.lq(19, s.wrapping_add(0x20));
        c.lq(20, s.wrapping_add(0x10));
        c.g[RA][0] = c.ld(s);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(s.wrapping_add(0x60));
        c.pc = ra;
        Ok(())
    })
}

/// CreateRoom's submit path (non-empty name): the "submitting" popup, then
/// the lobby create call with the channel and the extra string (or null).
fn create_room_submit(c: &mut Cpu) -> Flow {
    c.cycles += 26;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x001a_9c50, 0x001b_d6bc)?;
    c.g[S0][0] = c.g[V0][0];
    let popup = c.g[S0][0] as u32;
    c.sw(popup.wrapping_add(POPUP_TYPE), 0);
    c.g[A0][0] = sx(STR_SUBMITTING_CREATE_ROOM);
    call_ptr(c, 0x0031_7670, 0x001b_d6d0)?;
    c.g[A2][0] = 0x828;
    c.g[V1][0] = 3;
    let popup = c.g[S0][0] as u32;
    c.sw(popup.wrapping_add(POPUP_RECEIVE_PREV), 0);
    c.g[A1][0] = c.g[S0][0];
    c.sw(popup.wrapping_add(POPUP_MISC), c.g[A2][0]);
    c.g[A0][0] = c.g[S2][0];
    c.sw(popup.wrapping_add(POPUP_MSG_UI), c.g[V0][0]);
    c.sw(popup.wrapping_add(POPUP_MSG_TYPE), c.g[V1][0]);
    call_ptr(c, 0x001a_9bc0, 0x001b_d6f4)?;
    c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0041_6810, 0x001b_d700)?;
    c.g[A3][0] = 0;
    c.g[A0][0] = c.g[S0][0];
    if c.g[V0][0] != 0 {
        c.g[A3][0] = c.g[S1][0];
    }
    c.g[A1][0] = c.g[S4][0];
    c.g[A2][0] = c.g[S3][0];
    call_ptr(c, 0x0025_dac0, 0x001b_d718)?;
    Ok(())
}

/// CreateRoom's empty-name path: the "unique room" popup with an OK button.
fn create_room_unique_name(c: &mut Cpu) -> Flow {
    c.cycles += 26;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x001a_9c50, 0x001b_d654)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V0][0] = 1;
    c.g[A0][0] = sx(STR_UNIQUE_ROOM);
    let popup = c.g[S0][0] as u32;
    c.sw(popup.wrapping_add(POPUP_TYPE), c.g[V0][0]);
    c.g[S1][0] = sx(popup.wrapping_add(POPUP_TYPE));
    call_ptr(c, 0x0031_7670, 0x001b_d670)?;
    c.g[V1][0] = 3;
    c.g[A0][0] = sx(STR_BTN_OK);
    let popup = c.g[S0][0] as u32;
    c.sw(popup.wrapping_add(POPUP_MSG_TYPE), c.g[V1][0]);
    c.sw(popup.wrapping_add(POPUP_MSG_UI), c.g[V0][0]);
    call_ptr(c, 0x0031_7670, 0x001b_d688)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x001c_57c0, 0x001b_d698)?;
    c.g[V0][0] = 0x824;
    c.g[A0][0] = c.g[S2][0];
    let popup = c.g[S0][0] as u32;
    c.sw(popup.wrapping_add(POPUP_MISC), c.g[V0][0]);
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, 0x001a_9bc0, 0x001b_d6ac)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, scramble, Rng, DATA, GAME_GP};

    /// Lobby fixture: the self object at DATA+0x10000 (zeroed), the world
    /// pointer at gp-0x848, the lobby-man pointer at gp-0xc8 and the empty
    /// string at gp+0xda0 point into DATA.
    fn lobby_setup(c: &mut Cpu, r: &mut Rng) -> u32 {
        scramble(c, r);
        elf_image(c);
        c.g[SP][0] = sx(crate::difftest::SP);
        c.g[RA][0] = sx(crate::difftest::RA);
        c.g[GP][0] = sx(GAME_GP);
        c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x4_0000);
        c.bus.write32(GAME_GP + 0xda0, DATA + 0x6000);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x2000);
        DATA + 0x1_0000 // self
    }

    #[test]
    fn fe_state_lobby_menu_ctor_matches() {
        check(
            ee::s03::f_001bcb58__generated,
            fe_state_lobby_menu_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x1bcb_5800 ^ seed as u64);
                let this = lobby_setup(c, &mut r);
                c.pc = 0x001b_cb58;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x6fc, r.next_u32());
            },
        );
    }

    #[test]
    fn fe_state_lobby_menu_update_matches() {
        check(
            ee::s03::f_001bce18__generated,
            fe_state_lobby_menu_update,
            200,
            &|c, seed| {
                let mut r = Rng(0x1bce_1800 ^ seed as u64);
                let this = lobby_setup(c, &mut r);
                c.pc = 0x001b_ce18;
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn fe_state_lobby_menu_request_join_room_matches() {
        check(
            ee::s03::f_001bd540__generated,
            fe_state_lobby_menu_request_join_room,
            200,
            &|c, seed| {
                let mut r = Rng(0x1bd5_4000 ^ seed as u64);
                let this = lobby_setup(c, &mut r);
                let name = DATA + 0x2_0000;
                let pw = DATA + 0x3_0000;
                c.pc = 0x001b_d540;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(name);
                c.g[A2][0] = if seed % 3 == 0 { 0 } else { sx(pw) };
                c.bus.write32(name, 0x4142_4300);
                c.bus
                    .write32(pw, if seed % 5 == 0 { 0 } else { 0x4344_4500 });
            },
        );
    }

    #[test]
    fn fe_state_lobby_menu_create_room_matches() {
        check(
            ee::s03::f_001bd610__generated,
            fe_state_lobby_menu_create_room,
            200,
            &|c, seed| {
                let mut r = Rng(0x1bd6_1000 ^ seed as u64);
                let this = lobby_setup(c, &mut r);
                let name = DATA + 0x2_0000;
                let channel = DATA + 0x5000;
                let extra = DATA + 0x3_0000;
                c.pc = 0x001b_d610;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(name);
                c.g[A2][0] = sx(channel);
                c.g[A3][0] = sx(extra);
                c.bus
                    .write32(name, if seed % 4 == 0 { 0 } else { 0x4142_4300 });
                c.bus
                    .write32(extra, if seed % 3 == 0 { 0 } else { 0x4344_4500 });
                c.bus.write32(channel, 0x4548_4700);
            },
        );
    }
}
