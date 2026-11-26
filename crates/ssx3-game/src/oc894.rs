//! Staging module oc894 (wave wave253): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc894`.

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
        0x001b_e890,
        c_fe_state_net_create_room_popup_update,
        "cFEStateNetCreateRoomPopup_Update",
    ),
    (
        0x001b_da88,
        c_fe_state_lobby_menu_on_create_room_result,
        "cFEStateLobbyMenu_OnCreateRoomResult",
    ),
    (
        0x001b_de48,
        c_fe_state_lobby_menu_v25,
        "cFEStateLobbyMenu_v25",
    ),
    (
        0x001b_e3a8,
        c_fe_state_lobby_menu_update_players_in_room,
        "cFEStateLobbyMenu_UpdatePlayersInRoom",
    ),
    (
        0x001b_e540,
        c_fe_state_lobby_menu_fill_player_list,
        "cFEStateLobbyMenu_FillPlayerList",
    ),
];

/// `GetHashValue32` (`0x317670`): string address in A0, hash in V0.
const HASH_FN: u32 = 0x0031_7670;

const OC894_NET_FE_STATE_UPDATE: Callee = Callee {
    addr: 0x001a_8770,
    f: ee::s03::f_001a8770,
};
const OC894_NET_FE_STATE_GET_POPUP: Callee = Callee {
    addr: 0x001a_9c50,
    f: ee::s03::f_001a9c50,
};
const OC894_NET_FE_STATE_SHOW_POPUP: Callee = Callee {
    addr: 0x001a_9bc0,
    f: ee::s03::f_001a9bc0,
};
const OC894_NET_FE_STATE_HANDLE_NET_EVENT: Callee = Callee {
    addr: 0x001a_8e40,
    f: ee::s03::f_001a8e40,
};
const OC894_POPUP_CFG_SET_OPTION_TEXT_ID: Callee = Callee {
    addr: 0x001c_57c0,
    f: ee::s03::f_001c57c0,
};
const OC894_REFRESH_PLAYER_LIST: Callee = Callee {
    addr: 0x001b_e510,
    f: ee::s03::f_001be510,
};

/// `void cFEStateNetCreateRoomPopup_Update(self)` (0x1be890): forward the
/// update to `NetFEState_Update`.
pub fn c_fe_state_net_create_room_popup_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        OC894_NET_FE_STATE_UPDATE.call(c, 0x001b_e8a0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Which popup a create-room error code selects.
#[derive(Clone, Copy)]
enum RoomResult {
    /// `"full"` (0x66756c6c): max rooms reached (message 0x825).
    Full,
    /// `"dupl"` (0x6475706c): room already exists (message 0x826).
    Dupl,
    /// `"filt"` (0x66696c74): profanity in the room name (message 0x827).
    Filt,
    /// Any other message: `NetFEState_HandleNetEvent`.
    Other,
}

/// `GetHashValue32(0x46_0000 + off)` through the function table; returns to
/// `ret`.
fn hash_string(c: &mut Cpu, addr: u32, ret: u32) -> Flow {
    c.g[A0][0] = sx(addr);
    call_ptr(c, HASH_FN, ret)
}

/// One create-room popup: `NetFEState_GetPopup`, its message text and
/// buttons, the misc id at +0x150, then `NetFEState_ShowPopup`. `rets` are the
/// return addresses of the calls in the generated order (popup, title hash,
/// button hash, option text).
fn room_result_popup(c: &mut Cpu, title: u32, misc: u64, cycles: u64, rets: [u32; 4]) -> Flow {
    c.cycles += cycles;
    c.g[A0][0] = c.g[S2][0];
    OC894_NET_FE_STATE_GET_POPUP.call(c, rets[0])?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V0][0] = 1;
    c.g[A0][0] = sx(title);
    let v = c.g[V0][0];
    Addr(r32(c, S0).wrapping_add(0xbc)).sw(c, v);
    c.g[S1][0] = sx(r32(c, S0).wrapping_add(0xbc));
    call_ptr(c, HASH_FN, rets[1])?;
    c.g[V1][0] = 3;
    c.g[A0][0] = sx(0x0046_1618);
    let v1 = c.g[V1][0];
    Addr(r32(c, S0).wrapping_add(0xc0)).sw(c, v1);
    let v0 = c.g[V0][0];
    Addr(r32(c, S0).wrapping_add(0xc4)).sw(c, v0);
    call_ptr(c, HASH_FN, rets[2])?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    OC894_POPUP_CFG_SET_OPTION_TEXT_ID.call(c, rets[3])?;
    c.g[V0][0] = misc;
    c.cycles += 6;
    c.g[A0][0] = c.g[S2][0];
    let v0 = c.g[V0][0];
    Addr(r32(c, S0).wrapping_add(0x150)).sw(c, v0);
    c.g[A1][0] = c.g[S0][0];
    OC894_NET_FE_STATE_SHOW_POPUP.call(c, 0x001b_dbec)?;
    Ok(())
}

/// `void cFEStateLobbyMenu_OnCreateRoomResult(self, msg)` (0x1bda88): the
/// server's answer to a create-room request. `full`, `dupl` and `filt` raise
/// their popup (message ids 0x825..0x827); any other message goes to
/// `NetFEState_HandleNetEvent(self, msg->type, msg)`.
pub fn c_fe_state_lobby_menu_on_create_room_result(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[V0][0] = 0x6669_6c74;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
        let filt = c.g[V1][0] == c.g[V0][0];
        c.g[S2][0] = c.g[A0][0];
        let result = if filt {
            RoomResult::Filt
        } else {
            c.cycles += 3;
            let above = c.g[V0][0] < c.g[V1][0];
            c.g[V0][0] = 0x6675_0000;
            if above {
                c.cycles += 3;
                c.g[V0][0] |= 0x6c6c;
                if c.g[V1][0] == c.g[V0][0] {
                    RoomResult::Full
                } else {
                    RoomResult::Other
                }
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x6475_706c;
                if c.g[V1][0] == c.g[V0][0] {
                    RoomResult::Dupl
                } else {
                    c.cycles += 2;
                    RoomResult::Other
                }
            }
        };
        match result {
            RoomResult::Filt => {
                room_result_popup(
                    c,
                    0x0046_5128,
                    0x827,
                    20,
                    [0x001b_db94, 0x001b_dbb0, 0x001b_dbc8, 0x001b_dbd8],
                )?;
            }
            RoomResult::Dupl => {
                room_result_popup(
                    c,
                    0x0046_5108,
                    0x826,
                    21,
                    [0x001b_db40, 0x001b_db5c, 0x001b_db74, 0x001b_db84],
                )?;
            }
            RoomResult::Full => {
                room_result_popup(
                    c,
                    0x0046_50f0,
                    0x825,
                    21,
                    [0x001b_daec, 0x001b_db08, 0x001b_db20, 0x001b_db30],
                )?;
            }
            RoomResult::Other => {
                c.g[A1][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.cycles += 3;
                c.g[A0][0] = c.g[S2][0];
                OC894_NET_FE_STATE_HANDLE_NET_EVENT.call(c, 0x001b_dbfc)?;
            }
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One `0x1da510` call of `cFEStateLobbyMenu_v25`: `this` in A0, the hash in
/// A2 (from V0), `a1` and `a3` as given.
fn v25_slot(c: &mut Cpu, this: u64, a1: u64, a3: u64, ret: u32) -> Flow {
    c.g[A0][0] = this;
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = a1;
    c.g[A3][0] = a3;
    call_ptr(c, 0x001d_a510, ret)
}

/// `void cFEStateLobbyMenu_v25(self, a1)` (0x1bde48): fill the menu's
/// per-widget tables: the first-widget hash at +0x54, then the 0x1da510 and
/// 0x1da530 vtable slots for each of the six option widgets.
pub fn c_fe_state_lobby_menu_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 70;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[A1][0] = 6;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_a528, 0x001b_de64)?;
        hash_string(c, 0x0046_5148, 0x001b_de70)?;
        let v = c.g[V0][0];
        Addr(r32(c, S0).wrapping_add(0x54)).sw(c, v);
        hash_string(c, 0x0046_5160, 0x001b_de80)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x001d_a530, 0x001b_de90)?;
        hash_string(c, 0x0045_fe60, 0x001b_de9c)?;
        let this = c.g[S0][0];
        v25_slot(c, this, 0, 3, 0x001b_deb0)?;
        hash_string(c, 0x0045_fe40, 0x001b_debc)?;
        let this = c.g[S0][0];
        v25_slot(c, this, 1, 4, 0x001b_ded0)?;
        hash_string(c, 0x0046_5178, 0x001b_dedc)?;
        let this = c.g[S0][0];
        v25_slot(c, this, 2, 8, 0x001b_def0)?;
        hash_string(c, 0x0046_5198, 0x001b_defc)?;
        let this = c.g[S0][0];
        v25_slot(c, this, 3, 0, 0x001b_df10)?;
        hash_string(c, 0x0046_1458, 0x001b_df1c)?;
        let this = c.g[S0][0];
        v25_slot(c, this, 4, 2, 0x001b_df30)?;
        hash_string(c, 0x0046_14e8, 0x001b_df3c)?;
        let this = c.g[S0][0];
        v25_slot(c, this, 5, 1, 0x001b_df50)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `*(gp - back)` (`lw`), the globals the lobby code reads.
fn gp_lw(c: &mut Cpu, back: u32) -> u64 {
    Addr((c.g[GP][0] as u32).wrapping_sub(back)).lw(c)
}

/// `obj->vtable->method(obj + delta, a1)` as the generated code calls it: the
/// vtable at `obj+8`, the `this` delta at `vt+delta_off` (`lh`), the method at
/// `vt+fn_off`. The caller sets A2 (the object) itself, as the code does.
fn vcall(c: &mut Cpu, obj: u64, a1: u64, delta_off: u32, fn_off: u32, ret: u32) -> Flow {
    c.g[V0][0] = Addr(obj as u32).off(8).lw(c);
    let vt = c.g[V0][0] as u32;
    c.g[A0][0] = Addr(vt).off(delta_off).lh(c);
    c.g[V1][0] = Addr(vt).off(fn_off).lw(c);
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[A1][0] = a1;
    call_ptr(c, method, ret)
}

/// `void cFEStateLobbyMenu_UpdatePlayersInRoom(self, channel)` (0x1be3a8):
/// for each of the four channel tabs, show the lock icon of the tab whose
/// pair matches `channel` (and select its first room), or clear the lock
/// when the tab is not the one shown. A `channel` of -1 resets the list.
pub fn c_fe_state_lobby_menu_update_players_in_room(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S2, 0x80);
        frame.save(c, S1, 0x90);
        frame.save(c, S0, 0xa0);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save_ra(c, 0x50);
        c.g[S2][0] = c.g[A0][0];
        c.g[S0][0] = sx(r32(c, S2).wrapping_add(0x740));
        c.g[S1][0] = u64::MAX;
        c.g[S3][0] = 3;
        c.g[S4][0] = c.g[A1][0];
        c.g[V0][0] = Addr(r32(c, S0).wrapping_sub(0x28)).lw(c);
        loop {
            c.cycles += 2;
            if c.g[S4][0] != c.g[V0][0] {
                c.g[A2][0] = Addr(r32(c, S0)).lw(c);
                c.cycles += 7;
                let obj = c.g[A2][0];
                vcall(c, obj, 0, 0x48, 0x4c, 0x001b_e44c)?;
            } else {
                c.cycles += 4;
                c.g[A0][0] = Addr(r32(c, S0).wrapping_sub(0x38)).lw(c);
                c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x318).lbu(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 15;
                    call_ptr(c, 0x0039_a738, 0x001b_e400)?;
                    c.g[A0][0] = gp_lw(c, 0xc8);
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0025_f180, 0x001b_e410)?;
                    c.g[A2][0] = Addr(r32(c, S0)).lw(c);
                    let on = c.g[V0][0];
                    let obj = c.g[A2][0];
                    vcall(c, obj, on, 0x48, 0x4c, 0x001b_e42c)?;
                }
            }
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            c.cycles += 3;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            if (c.g[S3][0] as i64) < 0 {
                break;
            }
            c.g[V0][0] = Addr(r32(c, S0).wrapping_sub(0x28)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x001b_e3e0);
            }
        }
        // The tabs are done (S3 < 0): pick the selected room list, or reset it.
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        let reset = c.g[S1][0] == c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        if !reset {
            let selected = c.g[S1][0];
            c.cycles += 9;
            Addr(r32(c, S2).wrapping_add(0x774)).sw(c, selected);
            Addr(r32(c, S2).wrapping_add(0x778)).sw(c, 0);
            c.g[A1][0] = selected;
            OC894_REFRESH_PLAYER_LIST.call(c, 0x001b_e478)?;
            c.g[A0][0] = gp_lw(c, 0xc8);
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0025_d770, 0x001b_e484)?;
        } else {
            c.cycles += 26;
            c.g[V0][0] = gp_lw(c, 0x848);
            c.g[A0][0] = sx(0x0046_51c8);
            c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x8c).lw(c);
            c.g[S1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
            c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x20).lh(c);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            call_ptr(c, HASH_FN, 0x001b_e4ac)?;
            c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x24).lw(c);
            c.g[A0][0] = c.g[S0][0];
            let method = c.g[V1][0] as u32;
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, method, 0x001b_e4bc)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = 0;
            call_ptr(c, 0x002c_26d0, 0x001b_e4cc)?;
            c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0x704)).lw(c);
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003a_0e90, 0x001b_e4d8)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = u64::MAX;
            OC894_REFRESH_PLAYER_LIST.call(c, 0x001b_e4e4)?;
            c.g[A0][0] = gp_lw(c, 0xc8);
            c.g[A1][0] = u64::MAX;
            call_ptr(c, 0x0025_d770, 0x001b_e4f0)?;
        }
        frame.restore(c, S0, 0xa0);
        c.cycles += 7;
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateLobbyMenu_FillPlayerList(self, first)` (0x1be540): fill the
/// visible rows of the player list (ten rows at `self+0x6d8`) from the room's
/// users, starting at index `first`; `first == -1` empties the rows.
pub fn c_fe_state_lobby_menu_fill_player_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0xc0);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S3, 0x80);
        frame.save(c, S0, 0xb0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        let reset = c.g[A1][0] == c.g[V0][0];
        frame.save_ra(c, 0x50);
        if reset {
            c.cycles += 4;
            Addr(r32(c, S3).wrapping_add(0x77c)).sw(c, 0);
            c.g[S0][0] = sx(r32(c, S3).wrapping_add(0x6d8));
            c.g[S1][0] = 9;
            c.g[A2][0] = Addr(r32(c, S0)).lw(c);
            loop {
                c.cycles += 17;
                c.g[A1][0] = 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                let obj = c.g[A2][0];
                vcall(c, obj, 0, 0x48, 0x4c, 0x001b_e6d4)?;
                c.g[A2][0] = Addr(r32(c, S0)).lw(c);
                c.g[A1][0] = 1;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                let obj = c.g[A2][0];
                vcall(c, obj, 1, 0x40, 0x44, 0x001b_e6f4)?;
                if (c.g[S1][0] as i64) < 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0xb0);
                    break;
                }
                c.g[A2][0] = Addr(r32(c, S0)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x001b_e6b8);
                }
            }
        } else {
            c.cycles += 29;
            c.g[S5][0] = gp_lw(c, 0xc8);
            c.g[S4][0] = 0;
            c.g[S2][0] = sx(r32(c, S3).wrapping_add(0x6d8));
            c.g[A0][0] = c.g[S5][0];
            call_ptr(c, 0x0025_f1b0, 0x001b_e580)?;
            let count = c.g[V0][0];
            Addr(r32(c, S3).wrapping_add(0x77c)).sw(c, count);
            c.g[A0][0] = sx(0x0046_51c8);
            c.g[V0][0] = gp_lw(c, 0x848);
            c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x8c).lw(c);
            c.g[S1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
            c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x20).lh(c);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            call_ptr(c, HASH_FN, 0x001b_e5a4)?;
            c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x24).lw(c);
            c.g[A0][0] = c.g[S0][0];
            let method = c.g[V1][0] as u32;
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, method, 0x001b_e5b4)?;
            c.g[A2][0] = Addr(r32(c, S3).wrapping_add(0x77c)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x002c_26d0, 0x001b_e5c4)?;
            c.g[A0][0] = Addr(r32(c, S3).wrapping_add(0x704)).lw(c);
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003a_0e90, 0x001b_e5d0)?;
            c.g[V1][0] = Addr(r32(c, S3).wrapping_add(0x77c)).lbu(c);
            c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x6d0)).lw(c);
            let v = c.g[V1][0];
            Addr(c.g[V0][0] as u32).off(0x96).sb(c, v);
            loop {
                c.cycles += 6;
                c.g[V1][0] = Addr(r32(c, S3).wrapping_add(0x778)).lw(c);
                c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x77c)).lw(c);
                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S4][0] as u32));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[A2][0] = 0;
                if c.g[V0][0] != 0 {
                    c.cycles += 24;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0025_f1e0, 0x001b_e604)?;
                    c.g[A0][0] = Addr(r32(c, S2)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a_0c50, 0x001b_e610)?;
                    c.g[V0][0] = Addr(r32(c, S2)).lw(c);
                    c.g[A1][0] = 1;
                    let index = c.g[S0][0];
                    Addr(c.g[V0][0] as u32).off(0x18).sw(c, index);
                    c.g[A2][0] = Addr(r32(c, S2)).lw(c);
                    let obj = c.g[A2][0];
                    vcall(c, obj, 1, 0x48, 0x4c, 0x001b_e634)?;
                    c.g[A2][0] = Addr(r32(c, S2)).lw(c);
                    let obj = c.g[A2][0];
                    vcall(c, obj, 0, 0x40, 0x44, 0x001b_e650)?;
                } else {
                    c.cycles += 15;
                    c.g[A2][0] = Addr(r32(c, S2)).lw(c);
                    let obj = c.g[A2][0];
                    vcall(c, obj, 0, 0x48, 0x4c, 0x001b_e674)?;
                    c.g[A2][0] = Addr(r32(c, S2)).lw(c);
                    let obj = c.g[A2][0];
                    vcall(c, obj, 1, 0x40, 0x44, 0x001b_e690)?;
                }
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                c.cycles += 3;
                let more = ((c.g[S4][0] as i64) < 10) as u64;
                c.g[V0][0] = more;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(4));
                if more == 0 {
                    c.cycles += 2;
                    frame.restore(c, S0, 0xb0);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001b_e5e0);
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, DATA, GAME_GP};

    /// Scrambled registers and the fake hooks, with the entry in `pc`.
    fn base(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        let _ = machine(c, seed, salt, 4);
        c.pc = entry;
    }

    #[test]
    fn c_fe_state_net_create_room_popup_update_matches() {
        check(
            ee::s03::f_001be890__generated,
            c_fe_state_net_create_room_popup_update,
            200,
            &|c, seed| {
                base(c, seed, 0x1be8, 0x001b_e890);
                c.g[A0][0] = sx(DATA + 0x2000);
            },
        );
    }

    #[test]
    fn c_fe_state_lobby_menu_on_create_room_result_matches() {
        check(
            ee::s03::f_001bda88__generated,
            c_fe_state_lobby_menu_on_create_room_result,
            300,
            &|c, seed| {
                base(c, seed, 0x1bda, 0x001b_da88);
                let msg = DATA + 0x1000;
                c.g[A0][0] = sx(DATA + 0x2000);
                c.g[A1][0] = sx(msg);
                // msg->type at +0, msg->error at +8: every branch of the
                // error chain (filt, full, dupl, above and below the chain).
                c.bus.write32(msg, seed);
                let error = match seed % 5 {
                    0 => 0x6669_6c74,
                    1 => 0x6475_706c,
                    2 => 0x6675_6c6c,
                    3 => 0x7000_0000,
                    _ => 0x1234_5678,
                };
                c.bus.write32(msg + 8, error);
            },
        );
    }

    #[test]
    fn c_fe_state_lobby_menu_v25_matches() {
        check(
            ee::s03::f_001bde48__generated,
            c_fe_state_lobby_menu_v25,
            200,
            &|c, seed| {
                base(c, seed, 0x1bde, 0x001b_de48);
                c.g[A1][0] = sx(DATA + 0x2000);
            },
        );
    }

    /// The lobby's channel tables (four lock widgets and boxes, pairs
    /// 0..3), the lock vtable (both setVisible slots are fakes), the text
    /// object, and the globals the lobby code reads.
    pub(crate) fn oc894_lobby_fixture(c: &mut Cpu, seed: u32) {
        let this = DATA + 0x2000;
        let vt = DATA + 0x6800;
        c.bus.write16(vt + 0x40, 0);
        c.bus.write32(vt + 0x44, FAKE + 0x10);
        c.bus.write16(vt + 0x48, 0);
        c.bus.write32(vt + 0x4c, FAKE + 0x20);
        for i in 0..4u32 {
            let lock = DATA + 0x4000 + 0x100 * i;
            c.bus.write32(lock + 8, vt);
            c.bus.write32(this + 0x740 + 4 * i, lock);
            c.bus.write32(this + 0x718 + 4 * i, i);
            let boxed = DATA + 0x5000 + 0x100 * i;
            c.bus.write32(this + 0x708 + 4 * i, boxed);
            Addr(boxed + 0x318).write_u8(c, ((seed >> i) & 1) as u8);
        }
        c.bus.write32(this + 0x704, DATA + 0x7100);
        c.bus.write32(this + 0x6d0, DATA + 0x7000);
        // The room-list globals: LobbyMan, the world (+0x8c -> +4 -> +0x20,
        // method +0x24 a fake).
        c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x6900);
        let (world, w1, w2) = (DATA + 0x6a00, DATA + 0x6b00, DATA + 0x6c00);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x8c, w1);
        c.bus.write32(w1 + 4, w2);
        c.bus.write16(w2 + 0x20, 0);
        c.bus.write32(w2 + 0x24, FAKE + 0x40);
    }

    #[test]
    fn c_fe_state_lobby_menu_update_players_in_room_matches() {
        check(
            ee::s03::f_001be3a8__generated,
            c_fe_state_lobby_menu_update_players_in_room,
            200,
            &|c, seed| {
                base(c, seed, 0x1be3, 0x001b_e3a8);
                oc894_lobby_fixture(c, seed);
                c.g[A0][0] = sx(DATA + 0x2000);
                // channels 0..3 match a pair; 4 matches none (the reset path).
                c.g[A1][0] = (seed % 5) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_lobby_menu_fill_player_list_matches() {
        check(
            ee::s03::f_001be540__generated,
            c_fe_state_lobby_menu_fill_player_list,
            200,
            &|c, seed| {
                base(c, seed, 0x1be5, 0x001b_e540);
                oc894_lobby_fixture(c, seed);
                let this = DATA + 0x2000;
                // Ten rows at +0x6d8, each a widget with the lock vtable.
                let vt = DATA + 0x6800;
                for k in 0..10u32 {
                    let row = DATA + 0x8000 + 0x100 * k;
                    c.bus.write32(row + 8, vt);
                    c.bus.write32(this + 0x6d8 + 4 * k, row);
                }
                c.bus.write32(this + 0x778, seed % 3);
                c.bus.write32(this + 0x77c, seed % 12);
                c.g[A0][0] = sx(this);
                // -1 empties the rows; other values fill them.
                c.g[A1][0] = if seed % 4 == 0 {
                    u64::MAX
                } else {
                    (seed % 7) as u64
                };
            },
        );
    }
}
