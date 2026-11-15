//! Staging module oc378 (wave tagsunk): hand-written replacements for the 30
//! smallest `src/dirtysock/tagsunk.cpp` functions (reply stubs, request-code
//! setters, tiny BuddyMan/LobbyMan wrappers and manager create/destroy
//! pairs), each with its differential test against the generated body.
//! Moved into the source-tree units by `tools/reorg_modules.py apply oc378`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `FENetHook` vtable published at `this+8` by the constructor.
pub(crate) const FENET_HOOK_VTABLE: u32 = 0x0048_10c8;
/// `FENetHook2` vtable published at `this+0x14` by the constructor.
pub(crate) const FENET_HOOK2_VTABLE: u32 = 0x0048_10b0;
/// `gp - 0xc8`: the lobby manager singleton slot.
pub(crate) const LOBBY_MANAGER_SLOT: u32 = 0x004a_3028;
/// `gp + 0x2e8`: the shared login manager singleton slot.
pub(crate) const SHARED_LOGIN_MANAGER_SLOT: u32 = 0x004a_33d8;
/// `gp + 0x2e0`: the shared login manager slot written by create.
pub(crate) const SHARED_LOGIN_MANAGER_CREATED_SLOT: u32 = 0x004a_33d0;
/// `gp + 0x300`: the USB keyboard singleton slot.
pub(crate) const USB_KEYBOARD_SLOT: u32 = 0x004a_33f0;
/// `gp + 0x2e4`: the card state word written by the seek helper.
pub(crate) const LOGIN_CARD_STATE_SLOT: u32 = 0x004a_33d4;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0025_9398, lobby_man_compare_user_names, "LobbyMan_CompareUserNames"),
    (0x0025_93b8, lobby_man_compare_room_names, "LobbyMan_CompareRoomNames"),
    (0x0025_98a8, lobby_man_on_recv_debug, "LobbyMan_OnRecvDebug"),
    (0x0025_b650, lobby_man_create, "LobbyMan_Create"),
    (0x0025_c878, lobby_man_cancel_login, "LobbyMan_CancelLogin"),
    (0x0025_cd28, lobby_man_select_persona_index, "LobbyMan_SelectPersonaIndex"),
    (0x0025_fc20, lobby_man_on_mesg_reply, "LobbyMan_OnMesgReply"),
    (0x0025_fc28, lobby_man_send_decline, "LobbyMan_SendDecline"),
    (0x0025_fc48, lobby_man_send_abort_chal, "LobbyMan_SendAbortChal"),
    (0x0026_1a20, buddy_man_on_send_message_reply, "BuddyMan_OnSendMessageReply"),
    (0x0026_2390, buddy_man_is_buddy_at, "BuddyMan_IsBuddyAt"),
    (0x0026_23e0, buddy_man_is_blocked_at, "BuddyMan_IsBlockedAt"),
    (0x0026_2620, buddy_man_get_buddy_at, "BuddyMan_GetBuddyAt"),
    (0x0026_2658, buddy_man_get_blocked_at, "BuddyMan_GetBlockedAt"),
    (0x0026_3660, fe_net_hook_ctor, "FENetHook_ctor"),
    (0x0026_3998, fe_net_hook2_ctor, "FENetHook2_ctor"),
    (0x0026_5880, c_shared_login_man_create, "cSharedLoginMan_Create"),
    (0x0026_58e0, c_shared_login_man_ctor, "cSharedLoginMan_ctor"),
    (0x0026_5df0, c_shared_login_man_request_load, "cSharedLoginMan_RequestLoad"),
    (0x0026_5e00, c_shared_login_man_request_update, "cSharedLoginMan_RequestUpdate"),
    (0x0026_5e10, c_shared_login_man_request_create, "cSharedLoginMan_RequestCreate"),
    (0x0026_5e20, c_shared_login_man_request_delete, "cSharedLoginMan_RequestDelete"),
    (0x0026_5e30, c_shared_login_man_request_finish, "cSharedLoginMan_RequestFinish"),
    (0x0026_62e8, c_shared_login_man_mc_seek, "cSharedLoginMan_McSeek"),
    (0x0026_67c0, c_usb_keyboard_destroy, "cUsbKeyboard_Destroy"),
    (0x0026_6788, c_usb_keyboard_create, "cUsbKeyboard_Create"),
    (0x0026_6ed0, c_voip_man_toggle_active, "cVoipMan_ToggleActive"),
    (0x0026_6f08, c_voip_man_query_state, "cVoipMan_QueryState"),
    (0x0026_6f38, c_voip_man_get_shared_status, "cVoipMan_GetSharedStatus"),
    (0x0026_7da8, c_dsock_channel_base_disconnect, "cDSockChannel_BaseDisconnect"),
];

/// `void LobbyMan_OnMesgReply(void)` (0x25fc20): a lobby message reply stub
/// that returns at once, leaving every register as it found it.
pub fn lobby_man_on_mesg_reply(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void BuddyMan_OnSendMessageReply(void)` (0x261a20): a buddy message
/// reply stub that returns at once, leaving every register as it found it.
pub fn buddy_man_on_send_message_reply(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `int cSharedLoginMan_RequestLoad(cSharedLoginMan *self)` (0x265df0):
/// record the pending login-manager request code 2 (load) in the manager
/// word and return it in v0.
pub fn c_shared_login_man_request_load(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x2;
    Addr(r32(c, A0)).sw(c, c.g[V0][0]);
    ret(c);
}

/// `int cSharedLoginMan_RequestUpdate(cSharedLoginMan *self)` (0x265e00):
/// record the pending login-manager request code 6 (update) in the manager
/// word and return it in v0.
pub fn c_shared_login_man_request_update(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x6;
    Addr(r32(c, A0)).sw(c, c.g[V0][0]);
    ret(c);
}

/// `int cSharedLoginMan_RequestCreate(cSharedLoginMan *self)` (0x265e10):
/// record the pending login-manager request code 0xa (create) in the
/// manager word and return it in v0.
pub fn c_shared_login_man_request_create(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0xa;
    Addr(r32(c, A0)).sw(c, c.g[V0][0]);
    ret(c);
}

/// `int cSharedLoginMan_RequestDelete(cSharedLoginMan *self)` (0x265e20):
/// record the pending login-manager request code 0xe (delete) in the
/// manager word and return it in v0.
pub fn c_shared_login_man_request_delete(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0xe;
    Addr(r32(c, A0)).sw(c, c.g[V0][0]);
    ret(c);
}

/// `int cSharedLoginMan_RequestFinish(cSharedLoginMan *self)` (0x265e30):
/// record the pending login-manager request code 0x12 (finish) in the
/// manager word and return it in v0.
pub fn c_shared_login_man_request_finish(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x12;
    Addr(r32(c, A0)).sw(c, c.g[V0][0]);
    ret(c);
}

/// `FENetHook *FENetHook_ctor(FENetHook *self)` (0x263660): publish the
/// `FENetHook` vtable at `this+8`, clear `this`, and return it.
pub fn fe_net_hook_ctor(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = FENET_HOOK_VTABLE as u64;
    Addr(r32(c, A0)).sw(c, 0);
    Addr(r32(c, A0).wrapping_add(8)).sw(c, c.g[V1][0]);
    ret(c);
}

/// `FENetHook2 *FENetHook2_ctor(FENetHook2 *self)` (0x263998): publish the
/// `FENetHook2` vtable at `this+0x14`, clear `this`, and return it.
pub fn fe_net_hook2_ctor(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = FENET_HOOK2_VTABLE as u64;
    Addr(r32(c, A0)).sw(c, 0);
    Addr(r32(c, A0).wrapping_add(0x14)).sw(c, c.g[V1][0]);
    ret(c);
}

/// `void LobbyMan_OnRecvDebug(LobbyMan *self)` (0x2598a8): hand the debug
/// packet at `self+0x54` to `LobbyApi_DebugPacket`.
pub fn lobby_man_on_recv_debug(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(r32(c, A0)).off(0x54).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, 0x003e_8fd0, 0x0025_98b8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
/// `void *cVoipMan_GetSharedStatus(cVoipMan *self)` (0x266f38): return the
/// shared voice status word behind the voice global (`gp+0x2a28`).
pub fn c_voip_man_get_shared_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x003f_6c40, 0x0026_6f48)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cDSockChannel_BaseDisconnect(cDSockChannel *self)` (0x267da8):
/// reset the channel state, dropping any half-open connection.
pub fn c_dsock_channel_base_disconnect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0025_5840, 0x0026_7db8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int LobbyMan_CompareUserNames(void *a, void *b)` (0x259398): compare the
/// user names at `a2+0x1c` and `a3+0x1c` case-insensitively.
pub fn lobby_man_compare_user_names(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(0x1c));
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A3).wrapping_add(0x1c));
        call_ptr(c, 0x0041_aa88, 0x0025_93ac)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int LobbyMan_CompareRoomNames(void *a, void *b)` (0x2593b8): compare the
/// room names at `a2+0x8` and `a3+0x8` case-insensitively.
pub fn lobby_man_compare_room_names(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(0x8));
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A3).wrapping_add(0x8));
        call_ptr(c, 0x0041_aa88, 0x0025_93cc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_SendDecline(LobbyMan *self, char *to)` (0x25fc28): send
/// the decline line for `to` through `LobbyMan_SendPrivateMessage` with the
/// canned text at `0x4a3270`.
pub fn lobby_man_send_decline(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 0x4a0000;
        frame.save_ra(c, 0);
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x3270));
        call_ptr(c, 0x0025_fb80, 0x0025_fc3c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool BuddyMan_IsBuddyAt(BuddyMan *self, int idx)` (0x262390): whether
/// the roster entry at `idx` is a buddy (flag 4 at `entry+0xa8`).
pub fn buddy_man_is_buddy_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0026_2620, 0x0026_23a0)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `bool BuddyMan_IsBlockedAt(BuddyMan *self, int idx)` (0x2623e0): whether
/// the roster entry at `idx` is blocked (flag 0x200 at `entry+0xa8`).
pub fn buddy_man_is_blocked_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0026_2658, 0x0026_23f0)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_SelectPersonaIndex(LobbyMan *self, int idx)` (0x25cd28):
/// select the persona named by the `idx`-th entry of the table at
/// `self+0x70` through `LobbyMan_SelectPersona`.
pub fn lobby_man_select_persona_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx(r32(c, A1) << 2);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        c.g[A1][0] = Addr(r32(c, A1).wrapping_add(0x70)).lw(c);
        call_ptr(c, 0x0025_cd50, 0x0025_cd40)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_SendAbortChal(LobbyMan *self)` (0x25fc48): send the abort
/// line for the pending challenge at `self+0x98` through
/// `LobbyMan_SendPrivateMessage` with the canned text at `0x4807f8`.
pub fn lobby_man_send_abort_chal(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 0x480000;
        frame.save_ra(c, 0);
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x7f8));
        c.g[A1][0] = Addr(r32(c, A0)).off(0x98).lw(c);
        call_ptr(c, 0x0025_fb80, 0x0025_fc60)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cSharedLoginMan *cSharedLoginMan_ctor(cSharedLoginMan *self)`
/// (0x2658e0): initialise the shared login manager in place and return it.
pub fn c_shared_login_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0026_5950, 0x0026_58f4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}
/// `void cUsbKeyboard_Destroy(void)` (0x2667c0): destroy the USB keyboard
/// singleton at `gp+0x300` and clear the slot when one exists.
pub fn c_usb_keyboard_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(USB_KEYBOARD_SLOT).lw(c);
        let absent = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !absent {
            c.cycles += 3;
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x0026_6850, 0x0026_67d8)?;
            Addr(USB_KEYBOARD_SLOT).sw(c, 0);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *cVoipMan_QueryState(cVoipMan *self)` (0x266f08): the voice
/// active flag from `VoipRpc_IsActive`, or 0 when the manager word is clear.
pub fn c_voip_man_query_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(r32(c, A0)).lw(c);
        let present = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !present {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 2;
            call_ptr(c, 0x003f_6b80, 0x0026_6f28)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_CancelLogin(LobbyMan *self)` (0x25c878): cancel the
/// pending login request through `LobbyApi_CancelRequest` when the lobby is
/// in state 8.
pub fn lobby_man_cancel_login(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x8;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0)).lw(c);
        let pending = c.g[V0][0] != c.g[V1][0];
        frame.restore_ra(c, 0);
        if !pending {
            c.cycles += 4;
            c.g[A1][0] = Addr(r32(c, A0)).off(0x40).lw(c);
            c.g[A0][0] = Addr(r32(c, A0)).off(0x54).lw(c);
            call_ptr(c, 0x003e_8d70, 0x0025_c89c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_Create(void)` (0x25b650): allocate the lobby manager and
/// construct it in place, publishing it at `gp-0xc8`.
pub fn lobby_man_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x480000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x838));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x300;
        call_ptr(c, 0x0031_7d70, 0x0025_b670)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0025_94d0, 0x0025_b678)?;
        Addr(LOBBY_MANAGER_SLOT).sw(c, c.g[V0][0]);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *BuddyMan_GetBuddyAt(BuddyMan *self, int idx)` (0x262620): the
/// roster entry at `idx` when it is a buddy (flag 4 at `entry+0xa8`), else
/// 0.
pub fn buddy_man_get_buddy_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0026_25c8, 0x0026_2630)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
            c.cycles += 1;
        } else {
            c.cycles += 4;
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xa8)).lw(c);
            c.g[V1][0] = c.g[V1][0] & 0x4;
            let buddy = c.g[V1][0] != 0;
            if !buddy {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *BuddyMan_GetBlockedAt(BuddyMan *self, int idx)` (0x262658): the
/// roster entry at `idx` when it is blocked (flag 0x200 at `entry+0xa8`),
/// else 0.
pub fn buddy_man_get_blocked_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0026_25c8, 0x0026_2668)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
            c.cycles += 1;
        } else {
            c.cycles += 4;
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xa8)).lw(c);
            c.g[V1][0] = c.g[V1][0] & 0x200;
            let blocked = c.g[V1][0] != 0;
            if !blocked {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cSharedLoginMan_Create(void)` (0x265880): allocate the shared
/// login manager and construct it in place, publishing it at `gp+0x2e0`.
pub fn c_shared_login_man_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x480000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0xf30));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x8;
        call_ptr(c, 0x0031_7d70, 0x0026_58a0)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0026_58e0, 0x0026_58a8)?;
        Addr(SHARED_LOGIN_MANAGER_CREATED_SLOT).sw(c, c.g[V0][0]);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cSharedLoginMan_McSeek(int a1, int a2)` (0x2662e8): pass the seek
/// request (`a1`, with `a2` reduced to 0/1) to the card manager at
/// `gp+0x2e8`, then post card state 5 at `gp+0x2e4`.
pub fn c_shared_login_man_mc_seek(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0;
        let skip = c.g[A2][0] == 0;
        frame.save_ra(c, 0);
        if !skip {
            c.cycles += 2;
            c.g[V0][0] = c.g[A2][0] ^ 0x1;
            c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
        }
        c.cycles += 8;
        c.g[A0][0] = Addr(SHARED_LOGIN_MANAGER_SLOT).lw(c);
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, 0x002c_6a60, 0x0026_630c)?;
        c.g[V0][0] = 0x5;
        frame.restore_ra(c, 0);
        Addr(LOGIN_CARD_STATE_SLOT).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void cUsbKeyboard_Create(void)` (0x266788): allocate the USB keyboard
/// object and construct it in place, publishing it at `gp+0x300`.
pub fn c_usb_keyboard_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x480000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1090));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x464;
        call_ptr(c, 0x0031_7d70, 0x0026_67a8)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0026_67e8, 0x0026_67b0)?;
        Addr(USB_KEYBOARD_SLOT).sw(c, c.g[V0][0]);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cVoipMan_ToggleActive(cVoipMan *self)` (0x266ed0): deactivate the
/// voice manager when its word at `+4` is set, else activate it.
pub fn c_voip_man_toggle_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0)).off(4).lw(c);
        let active = c.g[V0][0] != 0;
        if active {
            c.cycles += 4;
            call_ptr(c, 0x0026_6e88, 0x0026_6eec)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0026_6e30, 0x0026_6efc)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    #[test]
    fn lobby_man_on_mesg_reply_matches() {
        check(
            ee::s06::f_0025fc20__generated,
            lobby_man_on_mesg_reply,
            8,
            &|c, _seed| {
                let mut r = Rng(0x25fc_2000);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn buddy_man_on_send_message_reply_matches() {
        check(
            ee::s06::f_00261a20__generated,
            buddy_man_on_send_message_reply,
            8,
            &|c, _seed| {
                let mut r = Rng(0x261a_2000);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn c_shared_login_man_request_load_matches() {
        check(
            ee::s07::f_00265df0__generated,
            c_shared_login_man_request_load,
            24,
            &|c, seed| {
                let mut r = Rng(0x265d_f000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = match seed % 3 {
                    0 => DATA as u64,
                    1 => (DATA + 0x40) as u64,
                    _ => 0,
                };
            },
        );
    }

    #[test]
    fn c_shared_login_man_request_update_matches() {
        check(
            ee::s07::f_00265e00__generated,
            c_shared_login_man_request_update,
            24,
            &|c, seed| {
                let mut r = Rng(0x265e_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = match seed % 3 {
                    0 => DATA as u64,
                    1 => (DATA + 0x40) as u64,
                    _ => 0,
                };
            },
        );
    }

    #[test]
    fn c_shared_login_man_request_create_matches() {
        check(
            ee::s07::f_00265e10__generated,
            c_shared_login_man_request_create,
            24,
            &|c, seed| {
                let mut r = Rng(0x265e_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = match seed % 3 {
                    0 => DATA as u64,
                    1 => (DATA + 0x40) as u64,
                    _ => 0,
                };
            },
        );
    }

    #[test]
    fn c_shared_login_man_request_delete_matches() {
        check(
            ee::s07::f_00265e20__generated,
            c_shared_login_man_request_delete,
            24,
            &|c, seed| {
                let mut r = Rng(0x265e_2000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = match seed % 3 {
                    0 => DATA as u64,
                    1 => (DATA + 0x40) as u64,
                    _ => 0,
                };
            },
        );
    }

    #[test]
    fn c_shared_login_man_request_finish_matches() {
        check(
            ee::s07::f_00265e30__generated,
            c_shared_login_man_request_finish,
            24,
            &|c, seed| {
                let mut r = Rng(0x265e_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = match seed % 3 {
                    0 => DATA as u64,
                    1 => (DATA + 0x40) as u64,
                    _ => 0,
                };
            },
        );
    }

    #[test]
    fn fe_net_hook_ctor_matches() {
        check(
            ee::s07::f_00263660__generated,
            fe_net_hook_ctor,
            24,
            &|c, seed| {
                let mut r = Rng(0x2636_6000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = match seed % 3 {
                    0 => DATA as u64,
                    1 => (DATA + 0x100) as u64,
                    _ => 0,
                };
            },
        );
    }

    #[test]
    fn fe_net_hook2_ctor_matches() {
        check(
            ee::s07::f_00263998__generated,
            fe_net_hook2_ctor,
            24,
            &|c, seed| {
                let mut r = Rng(0x2639_9800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = match seed % 3 {
                    0 => DATA as u64,
                    1 => (DATA + 0x100) as u64,
                    _ => 0,
                };
            },
        );
    }

    #[test]
    fn lobby_man_on_recv_debug_matches() {
        check(
            ee::s06::f_002598a8__generated,
            lobby_man_on_recv_debug,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x2598_a800, 0);
                let lobby = DATA + 0x1000;
                let packet = DATA + 0x2000;
                let text = DATA + 0x3000;
                c.bus.write32(
                    lobby + 0x54,
                    if seed % 3 == 0 { 0 } else { packet },
                );
                c.bus.write32(packet + 4, if seed % 2 == 0 { 0 } else { 0xffff_ffff });
                c.bus.write32(packet + 8, 0);
                c.bus.write32(packet + 0xc, text);
                match seed % 3 {
                    0 => c.bus.write8(text, 0),
                    1 => {
                        c.bus.write8(text, b'A');
                        c.bus.write8(text + 1, 0);
                    }
                    _ => {
                        c.bus.write8(text, 200);
                        c.bus.write8(text + 1, 0);
                    }
                }
                c.g[A0][0] = lobby as u64;
                c.g[A1][0] = packet as u64;
            },
        );
    }

    #[test]
    fn c_voip_man_get_shared_status_matches() {
        check(
            ee::s07::f_00266f38__generated,
            c_voip_man_get_shared_status,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x266f_3800, 0);
                let status = DATA + 0x1000;
                c.bus.write32(
                    GAME_GP + 0x2a28,
                    if seed % 3 == 0 { 0 } else { status },
                );
                c.bus.write32(status, 0x4000 + seed);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn c_dsock_channel_base_disconnect_matches() {
        check(
            ee::s07::f_00267da8__generated,
            c_dsock_channel_base_disconnect,
            32,
            &|c, seed| {
                let mut r = Rng(0x267d_a800 ^ seed as u64);
                machine(c, seed, 0x267d_a800, 0);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(DATA + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn lobby_man_compare_user_names_matches() {
        check(
            ee::s06::f_00259398__generated,
            lobby_man_compare_user_names,
            32,
            &|c, seed| {
                let mut r = Rng(0x2593_9800 ^ seed as u64);
                scramble(c, &mut r);
                let put = crate::game::dirtysock::tags::tests::put_str;
                let (a, b) = match seed % 4 {
                    0 => (&b"alice"[..], &b"alice"[..]),
                    1 => (&b"alice"[..], &b"bob"[..]),
                    2 => (&b""[..], &b"x"[..]),
                    _ => (&b"bob"[..], &b"bobby"[..]),
                };
                put(c, DATA + 0x100, a);
                put(c, DATA + 0x200, b);
                c.g[A2][0] = (DATA + 0x100 - 0x1c) as u64;
                c.g[A3][0] = (DATA + 0x200 - 0x1c) as u64;
            },
        );
    }

    #[test]
    fn lobby_man_compare_room_names_matches() {
        check(
            ee::s06::f_002593b8__generated,
            lobby_man_compare_room_names,
            32,
            &|c, seed| {
                let mut r = Rng(0x2593_b800 ^ seed as u64);
                scramble(c, &mut r);
                let put = crate::game::dirtysock::tags::tests::put_str;
                let (a, b) = match seed % 4 {
                    0 => (&b"room1"[..], &b"room1"[..]),
                    1 => (&b"room1"[..], &b"room2"[..]),
                    2 => (&b""[..], &b"z"[..]),
                    _ => (&b"lobby"[..], &b"lobb"[..]),
                };
                put(c, DATA + 0x100, a);
                put(c, DATA + 0x200, b);
                c.g[A2][0] = (DATA + 0x100 - 0x8) as u64;
                c.g[A3][0] = (DATA + 0x200 - 0x8) as u64;
            },
        );
    }

    #[test]
    fn lobby_man_send_decline_matches() {
        check(
            ee::s06::f_0025fc28__generated,
            lobby_man_send_decline,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x25fc_2800, 0);
                let put = crate::game::dirtysock::tags::tests::put_str;
                let lobby = DATA + 0x4000;
                let packet = DATA + 0x6000;
                c.bus.write32(lobby, (seed % 2) as u32);
                c.bus.write32(lobby + 0x54, 0);
                c.bus.write32(lobby + 0xf0, packet);
                c.bus.write32(packet, 0);
                c.bus.write32(packet + 4, DATA + 0x7000);
                match seed % 3 {
                    0 => put(c, DATA + 0x5000, b"bob"),
                    1 => put(c, DATA + 0x5000, b""),
                    _ => put(c, DATA + 0x5000, b"x"),
                }
                c.g[A0][0] = lobby as u64;
                c.g[A1][0] = (DATA + 0x5000) as u64;
            },
        );
    }
    fn roster_setup(c: &mut Cpu, seed: u32, bit: u32) {
        let man = DATA + 0x1000;
        let list = DATA + 0x2000;
        let array = DATA + 0x2100;
        let entry = DATA + 0x2200;
        match seed % 4 {
            0 => {
                c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                c.bus.write32(man + 0xcc, 0);
            }
            1 => {
                c.g[A1][0] = 0;
                c.bus.write32(man + 0xcc, 0);
            }
            _ => {
                c.g[A1][0] = 0;
                c.bus.write32(man + 0xcc, list);
                c.bus.write32(list + 0x18, 2);
                c.bus.write32(list + 0x40, array);
                c.bus.write32(array, entry);
                c.bus.write32(
                    entry + 0xa8,
                    if seed % 4 == 2 { 0 } else { bit },
                );
            }
        }
        c.g[A0][0] = man as u64;
    }

    #[test]
    fn buddy_man_is_buddy_at_matches() {
        check(
            ee::s07::f_00262390__generated,
            buddy_man_is_buddy_at,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x2623_9000, 0);
                roster_setup(c, seed, 0x4);
            },
        );
    }

    #[test]
    fn buddy_man_is_blocked_at_matches() {
        check(
            ee::s07::f_002623e0__generated,
            buddy_man_is_blocked_at,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x2623_e000, 0);
                roster_setup(c, seed, 0x200);
            },
        );
    }

    #[test]
    fn buddy_man_get_buddy_at_matches() {
        check(
            ee::s07::f_00262620__generated,
            buddy_man_get_buddy_at,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x2626_2000, 0);
                roster_setup(c, seed, 0x4);
            },
        );
    }

    #[test]
    fn buddy_man_get_blocked_at_matches() {
        check(
            ee::s07::f_00262658__generated,
            buddy_man_get_blocked_at,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x2626_5800, 0);
                roster_setup(c, seed, 0x200);
            },
        );
    }

    #[test]
    fn lobby_man_select_persona_index_matches() {
        check(
            ee::s06::f_0025cd28__generated,
            lobby_man_select_persona_index,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x25cd_2800, 0);
                let put = crate::game::dirtysock::tags::tests::put_str;
                let this = DATA + 0x1000;
                c.bus.write32(this + 0x54, 0);
                c.bus.write32(this + 0x18, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(this + 0xf0, 0);
                put(c, DATA + 0x2000, b"a");
                put(c, DATA + 0x2100, b"bb");
                put(c, DATA + 0x2200, b"ccc");
                c.bus.write32(this + 0x70, DATA + 0x2000);
                c.bus.write32(this + 0x74, DATA + 0x2100);
                c.bus.write32(this + 0x78, DATA + 0x2200);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn lobby_man_send_abort_chal_matches() {
        check(
            ee::s06::f_0025fc48__generated,
            lobby_man_send_abort_chal,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x25fc_4800, 0);
                let put = crate::game::dirtysock::tags::tests::put_str;
                let lobby = DATA + 0x4000;
                let packet = DATA + 0x6000;
                c.bus.write32(lobby, (seed % 2) as u32);
                c.bus.write32(lobby + 0x54, 0);
                c.bus.write32(lobby + 0xf0, packet);
                c.bus.write32(packet, 0);
                c.bus.write32(packet + 4, DATA + 0x7000);
                match seed % 3 {
                    0 => {
                        put(c, DATA + 0x5000, b"challenger");
                        c.bus.write32(lobby + 0x98, DATA + 0x5000);
                    }
                    1 => {
                        put(c, DATA + 0x5000, b"");
                        c.bus.write32(lobby + 0x98, DATA + 0x5000);
                    }
                    _ => c.bus.write32(lobby + 0x98, 0),
                }
                c.g[A0][0] = lobby as u64;
            },
        );
    }

    #[test]
    fn c_shared_login_man_ctor_matches() {
        check(
            ee::s07::f_002658e0__generated,
            c_shared_login_man_ctor,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x2658_e000, 0);
                c.g[A0][0] = (DATA + 0x1000 + (seed % 2) * 0x100) as u64;
            },
        );
    }
    #[test]
    fn c_usb_keyboard_destroy_matches() {
        check(
            ee::s07::f_002667c0__generated,
            c_usb_keyboard_destroy,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x2667_c000, 0);
                if seed % 2 == 0 {
                    c.bus.write32(GAME_GP + 0x300, 0);
                } else {
                    let kb =
                        crate::game::dirtysock::tagsunk::tests::heap_block(c, 0x468);
                    c.bus.write32(kb + 0x460, 0);
                    c.bus.write32(GAME_GP + 0x300, kb);
                }
            },
        );
    }

    #[test]
    fn c_voip_man_query_state_matches() {
        check(
            ee::s07::f_00266f08__generated,
            c_voip_man_query_state,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x266f_0800, 0);
                let status = DATA + 0x1000;
                c.bus.write32(
                    GAME_GP + 0x2a28,
                    if seed % 4 == 3 { 0 } else { status },
                );
                c.bus.write32(status, 0x5000 + seed);
                let v = DATA + 0x2000;
                c.bus.write32(v, if seed % 2 == 0 { 0 } else { 1 });
                c.g[A0][0] = v as u64;
            },
        );
    }

    #[test]
    fn lobby_man_cancel_login_matches() {
        check(
            ee::s06::f_0025c878__generated,
            lobby_man_cancel_login,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x25c8_7800, 0);
                let lobby = DATA + 0x1000;
                c.bus.write32(lobby, if seed % 3 == 0 { 8 } else { 7 });
                c.bus.write32(lobby + 0x40, 0x1234);
                c.bus.write32(lobby + 0x54, 0);
                c.g[A0][0] = lobby as u64;
            },
        );
    }

    #[test]
    fn lobby_man_create_matches() {
        check(
            ee::s06::f_0025b650__generated,
            lobby_man_create,
            8,
            &|c, seed| {
                let _r = machine(c, seed, 0x25b6_5000, 0);
                crate::game::dirtysock::tagsunk::tests::heap_init(c);
                c.g[A0][0] = seed as u64;
            },
        );
    }

    #[test]
    fn c_shared_login_man_create_matches() {
        check(
            ee::s07::f_00265880__generated,
            c_shared_login_man_create,
            8,
            &|c, seed| {
                let _r = machine(c, seed, 0x2658_8000, 0);
                crate::game::dirtysock::tagsunk::tests::heap_init(c);
                c.g[A0][0] = seed as u64;
            },
        );
    }

    #[test]
    fn c_shared_login_man_mc_seek_matches() {
        check(
            ee::s07::f_002662e8__generated,
            c_shared_login_man_mc_seek,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x2662_e800, 0);
                let card = DATA + 0x4000;
                c.bus.write32(
                    GAME_GP + 0x2e8,
                    if seed % 2 == 0 { 0 } else { card },
                );
                c.g[A1][0] = seed as u64;
                c.g[A2][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn c_usb_keyboard_create_matches() {
        check(
            ee::s07::f_00266788__generated,
            c_usb_keyboard_create,
            8,
            &|c, seed| {
                let _r = machine(c, seed, 0x2667_8800, 0);
                crate::game::dirtysock::tagsunk::tests::heap_init(c);
                c.g[A0][0] = seed as u64;
            },
        );
    }

    #[test]
    fn c_voip_man_toggle_active_matches() {
        check(
            ee::s07::f_00266ed0__generated,
            c_voip_man_toggle_active,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x266e_d000, 0);
                let v = DATA + 0x1000;
                c.bus.write32(v, 0);
                c.bus.write32(v + 4, (seed % 2) as u32);
                c.g[A0][0] = v as u64;
            },
        );
    }
}
