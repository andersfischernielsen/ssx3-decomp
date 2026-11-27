//! Staging module oc946 (wave wave256): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc946`.
//!
//! Not ported here (left generated): `MemCardUI_ApplySoundMode` (FPU
//! conversions and a long vtable/sound sequence; wants its own writer) and
//! `cFEStateNetHelpPopup_OnWidgetCreated` (a `match pc` dispatcher with a
//! `jump_out` default: the orchestrator converts it with `port_dispatcher.py`).

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001d_5138, c_fe_state_relogin_v31, "cFEStateRelogin_v31"),
    (
        0x001d_5c28,
        mem_card_ui_read_save_chunk,
        "MemCardUI_ReadSaveChunk",
    ),
    (
        0x001d_5d78,
        mem_card_ui_write_replay_save_block,
        "MemCardUI_WriteReplaySaveBlock",
    ),
    (
        0x001d_a110,
        c_fe_state_net_help_popup_build_body_text,
        "cFEStateNetHelpPopup_BuildBodyText",
    ),
    (
        0x001d_a528,
        c_fe_state_net_help_popup_set_num_buttons,
        "cFEStateNetHelpPopup_SetNumButtons",
    ),
    (
        0x001d_a648,
        lobby_menu_init_create_account_page,
        "LobbyMenu_InitCreateAccountPage",
    ),
];

/// `gp - back`: the address of a global the generated code reaches through gp.
pub(crate) fn oc946_gp(c: &Cpu, back: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(back)
}

/// `cFEStateRelogin` net-event codes the dispatcher tests (the reference's
/// `kNetEvent_LobbyConnect`, `kNetEvent_Authentication` and the two numbered
/// events it handles).
pub(crate) const RELOGIN_LOBBY_CONNECT: u64 = 0xc9;
pub(crate) const RELOGIN_AUTHENTICATION: u64 = 0xd4;
pub(crate) const RELOGIN_ONLINE_MENU: u64 = 0x101;
pub(crate) const RELOGIN_RANKED_OPTIONS: u64 = 0x111;

/// `cFEStateRelogin_v31(self, msg)` (0x1d5138): the net-event handler of the
/// relogin state. Lobby connect and authentication forward to LobbyMan, 0x101
/// builds the online main menu, 0x111 starts a ranked game with the network
/// options; any other event goes to the parent handler `func_001BF228`.
pub fn c_fe_state_relogin_v31(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = RELOGIN_AUTHENTICATION;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let msg = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        if relogin_handled(c, msg)? {
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            // Default: the parent state's handler with the same message.
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001b_f228, 0x001d_5228)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The cases of [`c_fe_state_relogin_v31`]: true when the message was handled
/// (the function returns 0), false when it goes to the parent handler. Leaves
/// `v0` as the generated dispatch leaves it.
fn relogin_handled(c: &mut Cpu, msg: u64) -> Result<bool, Unwound> {
    if msg == RELOGIN_AUTHENTICATION {
        c.cycles += 5;
        c.g[A0][0] = Addr(oc946_gp(c, 0xc8)).lw(c);
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x80)).lw(c);
        call_ptr(c, 0x0025_cd50, 0x001d_51a4)?;
        c.g[V0][0] = 0;
        return Ok(true);
    }
    c.cycles += 3;
    let below_213 = (msg as i64) < 213;
    c.g[V0][0] = RELOGIN_LOBBY_CONNECT;
    if !below_213 {
        c.cycles += 3;
        c.g[V0][0] = RELOGIN_ONLINE_MENU;
        let online_menu = msg == RELOGIN_ONLINE_MENU;
        c.g[V0][0] = RELOGIN_RANKED_OPTIONS;
        if online_menu {
            c.cycles += 14;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0x6d0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd8));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x001d_51c4)?;
            c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x10)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x001b_fbb8, 0x001d_51d0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x001a_8a40, 0x001d_51dc)?;
            c.g[V0][0] = 0;
            return Ok(true);
        }
        c.cycles += 2;
        let ranked_options = msg == RELOGIN_RANKED_OPTIONS;
        c.g[V0][0] = Addr(oc946_gp(c, 0x848)).lw(c);
        if ranked_options {
            c.cycles += 15;
            c.g[A1][0] = 7;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x78)).lw(c);
            call_ptr(c, 0x0014_dd58, 0x001d_51f0)?;
            c.g[T0][0] = 0x530000;
            c.g[A0][0] = Addr(oc946_gp(c, 0xc8)).lw(c);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x4b30));
            c.g[A2][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x8)).lw(c);
            c.g[T1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x3dc));
            c.g[A1][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x4)).lw(c);
            c.g[A3][0] = (0u64 < c.g[A2][0]) as u64;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x330));
            c.g[A2][0] = (c.g[A2][0] < 0x1u64) as u64;
            call_ptr(c, 0x0025_fd50, 0x001d_5218)?;
            c.g[V0][0] = 0;
            return Ok(true);
        }
        c.cycles += 2;
        return Ok(false);
    }
    // below 213: lobby connect, or the parent handler
    c.cycles += 2;
    if msg == RELOGIN_LOBBY_CONNECT {
        c.cycles += 4;
        c.g[A0][0] = Addr(oc946_gp(c, 0xc8)).lw(c);
        call_ptr(c, 0x0025_c610, 0x001d_5190)?;
        c.g[V0][0] = 0;
        return Ok(true);
    }
    c.cycles += 2;
    Ok(false)
}

/// `bool cFEMemCard_callbackReadUpdate(size)` (`MemCardUI_ReadSaveChunk`,
/// 0x1d5c28): reads one replay save chunk from the memory card. Only for the
/// replay card type: checks the BE data's CRC (a mismatch releases the replay
/// buffer and fails), then copies `size` bytes (at most 0x4000) into the
/// overlay manager's read. Returns true unless the CRC failed.
pub fn mem_card_ui_read_save_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = Addr(oc946_gp(c, 0x10c8)).lw(c);
        frame.save(c, S2, 0x30);
        c.g[S2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x1bc)).lw(c);
        c.g[S1][0] = c.g[A0][0];
        if c.g[S2][0] == c.g[V0][0] {
            if read_chunk_bulk(c)? {
                read_chunk_size(c)?;
                c.cycles += 1;
                c.g[V0][0] = 1;
            } else {
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The `mLoadBEData` part of [`mem_card_ui_read_save_chunk`]. False when the
/// CRC of the BE data did not match (the function then returns false).
fn read_chunk_bulk(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x200)).lw(c);
    if c.g[V0][0] == 0 {
        return Ok(true);
    }
    c.cycles += 29;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x200), 0u64);
    c.g[S3][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x1d0)).lw(c);
    call_ptr(c, 0x0014_dc80, 0x001d_5c6c)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_e048, 0x001d_5c74)?;
    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x1d0)).lw(c);
    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
    c.g[A2][0] = 0xfbea;
    call_ptr(c, 0x003e_62d0, 0x001d_5c84)?;
    c.g[S0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_dc80, 0x001d_5c8c)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_e048, 0x001d_5c94)?;
    c.g[A0][0] = Addr(oc946_gp(c, 0x10c8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1d0)).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x1d0), c.g[V1][0]);
    // The stored CRC: an unaligned 4-byte copy of the buffer tail to the stack.
    c.sw(sp(c), 0u64);
    c.lwl(2, (c.g[V1][0] as u32).wrapping_add(0x3));
    c.lwr(2, c.g[V1][0] as u32);
    c.swl(2, sp(c).wrapping_add(0x3));
    c.swr(2, sp(c));
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1d0)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x1d0), c.g[V0][0]);
    c.g[V1][0] = Addr(sp(c)).lw(c);
    if c.g[S0][0] != c.g[V1][0] {
        c.cycles += 13;
        call_ptr(c, 0x0014_dc80, 0x001d_5cdc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 5;
        call_ptr(c, 0x0014_dd58, 0x001d_5ce8)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0015_2728, 0x001d_5cf0)?;
        c.g[V1][0] = Addr(oc946_gp(c, 0x10c8)).lw(c);
        c.g[V0][0] = 0;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x228), 0u64);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1d4), 0u64);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1d0), 0u64);
        return Ok(false);
    }
    c.cycles += 5;
    c.sw(oc946_gp(c, 0x1738), c.g[S2][0]);
    call_ptr(c, 0x0014_dc80, 0x001d_5d10)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S3][0];
    call_ptr(c, 0x0014_e0c0, 0x001d_5d1c)?;
    Ok(true)
}

/// The size part of [`mem_card_ui_read_save_chunk`]: clamps `size` to 0x4000,
/// reads that many bytes through the overlay manager and advances the buffer.
fn read_chunk_size(c: &mut Cpu) -> Result<(), Unwound> {
    c.cycles += 2;
    let positive = (c.g[S1][0] as i64) > 0;
    c.g[V1][0] = ((c.g[S1][0] as i64) < 16385) as u64;
    if !positive {
        return Ok(());
    }
    c.cycles += 13;
    c.g[A0][0] = Addr(oc946_gp(c, 0x848)).lw(c);
    c.g[V0][0] = 0x4000;
    if c.g[V1][0] == 0 {
        c.g[S1][0] = c.g[V0][0];
    }
    call_ptr(c, 0x0022_7f80, 0x001d_5d34)?;
    c.g[V1][0] = Addr(oc946_gp(c, 0x10c8)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1d0)).lw(c);
    call_ptr(c, 0x0024_1200, 0x001d_5d48)?;
    c.g[V1][0] = Addr(oc946_gp(c, 0x10c8)).lw(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1d0)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x1d0), c.g[V0][0]);
    Ok(())
}

/// `bool cFEMemCard_callbackWriteUpdate(key)` (`MemCardUI_WriteReplaySaveBlock`,
/// 0x1d5d78): for the replay card type, asks the replay for its save data and
/// writes it to the overlay manager. Returns true when the data was written.
pub fn mem_card_ui_write_replay_save_block(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr(oc946_gp(c, 0x10c8)).lw(c);
        frame.save(c, S0, 0x20);
        c.g[A1][0] = 0x1;
        frame.save_ra(c, 0x10);
        c.sw(frame.at(c, 0x4), c.g[A0][0]);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1bc)).lw(c);
        let replay = c.g[V1][0] == c.g[A1][0];
        c.g[V0][0] = 0;
        if replay {
            c.cycles += 11;
            c.g[A0][0] = Addr(oc946_gp(c, 0x848)).lw(c);
            call_ptr(c, 0x0022_7f80, 0x001d_5da4)?;
            c.g[V1][0] = Addr(oc946_gp(c, 0x848)).lw(c);
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = c.g[SP][0] | 0x4;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c);
            call_ptr(c, 0x0027_10a8, 0x001d_5dc0)?;
            c.g[A2][0] = Addr(frame.at(c, 0x4)).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = Addr(sp(c)).lw(c);
                call_ptr(c, 0x0024_1138, 0x001d_5dd4)?;
                c.g[V0][0] = 1;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateNetHelpPopup::SetNumButtons(int n)` (0x1da528): stores the
/// button count at +0x48.
pub fn c_fe_state_net_help_popup_set_num_buttons(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x48), c.g[A1][0]);
        ret(c);
        Ok(())
    })
}

/// `cFEStateNetHelpPopup::BuildBodyText(self)` (0x1da110): collects the three
/// localised lines named by the ids at `self+0x58` (up to three, stopping at
/// the first zero id), allocates one buffer for them, concatenates the strings
/// into it and hands it to the popup body at `self+0xac`.
pub fn c_fe_state_net_help_popup_build_body_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S6, 0x20);
        frame.save(c, S0, 0x80);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x58));
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[SP][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x40);
        c.g[S3][0] = 0;
        frame.save(c, S5, 0x30);
        c.g[S4][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[S5][0] = 0;
        'l_1da150: loop {
            'b_1da1a0: {
                c.cycles += 3;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                let no_id = c.g[A1][0] == 0;
                c.g[V0][0] = Addr(oc946_gp(c, 0x848)).lw(c);
                if no_id {
                    break 'b_1da1a0;
                }
                c.cycles += 17;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                // sStrings: the vtable's slot 4 looks one id up
                c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8c)).lw(c);
                c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x4)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
                let lookup = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, lookup, 0x001d_a180)?;
                c.g[A0][0] = c.g[V0][0];
                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                call_ptr(c, 0x003d_bca0, 0x001d_a18c)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = ((c.g[S2][0] as i64) < 3) as u64;
                let more = c.g[A0][0] != 0;
                c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x001d_a150);
                    }
                    continue 'l_1da150;
                }
                break 'b_1da1a0;
            }
            'b_1da1f0: {
                c.cycles += 2;
                let empty = (c.g[S4][0] as i64) <= 0;
                c.g[A1][0] = 0x460000;
                if empty {
                    break 'b_1da1f0;
                }
                c.cycles += 8;
                c.g[A0][0] = sx((c.g[S4][0] as u32) << 1);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a70));
                c.g[A2][0] = 0x100;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7e30, 0x001d_a1bc)?;
                c.g[S3][0] = c.g[V0][0];
                let none = (c.g[S5][0] as i64) <= 0;
                c.sh(c.g[S3][0] as u32, 0u64);
                if none {
                    break 'b_1da1f0;
                }
                c.cycles += 4;
                c.g[S1][0] = c.g[S5][0];
                c.g[S0][0] = c.g[SP][0];
                c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                'l_1da1d8: loop {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    call_ptr(c, 0x002c_2688, 0x001d_a1e8)?;
                    if c.g[S1][0] != 0 {
                        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x001d_a1d8);
                        }
                        continue 'l_1da1d8;
                    }
                    break 'b_1da1f0;
                }
            }
            'b_1da210: {
                c.cycles += 5;
                c.g[A0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0xac)).lw(c);
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x003a_0d00, 0x001d_a1fc)?;
                let no_buf = c.g[S3][0] == 0;
                frame.restore(c, S0, 0x80);
                if no_buf {
                    break 'b_1da210;
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x0031_7e98, 0x001d_a20c)?;
                frame.restore(c, S0, 0x80);
                break 'b_1da210;
            }
            c.cycles += 9;
            frame.restore(c, S1, 0x70);
            frame.restore(c, S2, 0x60);
            frame.restore(c, S3, 0x50);
            frame.restore(c, S4, 0x40);
            frame.restore(c, S5, 0x30);
            frame.restore(c, S6, 0x20);
            frame.restore_ra(c, 0x10);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `LobbyMenu_InitCreateAccountPage(self, vinfo, arg)` (0x1da648): builds the
/// create-account menu in place: the menu base, the title and spacer, the
/// login-name and password text items, the action and cancel items, then the
/// two name fields copied from `SSXApp`. Returns `self`.
pub fn lobby_menu_init_create_account_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 103;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S1, 0x80);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        frame.save(c, FP, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x70);
        c.g[FP][0] = c.g[A2][0];
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x134));
        frame.save(c, S5, 0x40);
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x150));
        frame.save(c, S6, 0x30);
        c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x168));
        frame.save(c, S7, 0x20);
        c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x194));
        frame.save_ra(c, 0);
        c.g[S6][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1c0));
        call_ptr(c, 0x002c_aa58, 0x001d_a698)?;
        c.g[S7][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1e4));
        c.g[V0][0] = 0x470000;
        c.g[A1][0] = 0x460000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2970));
        c.g[A0][0] = c.g[S3][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x12c), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7a20));
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x130), c.g[V0][0]);
        call_ptr(c, 0x002c_d2a0, 0x001d_a6c0)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0xa;
        call_ptr(c, 0x002c_d350, 0x001d_a6cc)?;
        c.g[S0][0] = 0x4a0000;
        c.g[A1][0] = 0x460000;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1408));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7a30));
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x002c_e418, 0x001d_a6e8)?;
        c.g[A1][0] = 0x460000;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7a40));
        call_ptr(c, 0x002c_e418, 0x001d_a6fc)?;
        c.g[A3][0] = 0;
        c.g[A2][0] = c.g[FP][0];
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x002c_cf98, 0x001d_a710)?;
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = c.g[S7][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7a50));
        call_ptr(c, 0x002c_cdf0, 0x001d_a720)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_a730)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_a740)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_a750)?;
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_a760)?;
        c.g[A1][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_a770)?;
        c.g[A1][0] = c.g[S7][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_a780)?;
        c.g[V0][0] = Addr(oc946_gp(c, 0x848)).lw(c);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x180));
        c.g[A2][0] = 0xf;
        c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc4)).lw(c);
        c.sb((c.g[S1][0] as u32).wrapping_add(0x180), 0u64);
        call_ptr(c, 0x0041_6958, 0x001d_a798)?;
        c.g[V1][0] = Addr(oc946_gp(c, 0x848)).lw(c);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1ac));
        c.g[A2][0] = 0xf;
        c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc8)).lw(c);
        c.sb((c.g[S1][0] as u32).wrapping_add(0x1ac), 0u64);
        call_ptr(c, 0x0041_6958, 0x001d_a7b0)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Gate of the callee fakes: they answer only while this word holds
    /// `OC946_MAGIC` in the machine's RAM (otherwise the generated body runs).
    pub(crate) const OC946_CTRL: u32 = DATA + 0x0060_0000;
    pub(crate) const OC946_MAGIC: u32 = 0x0946_c0de;
    /// Forced results of the fakes that must return a chosen value (word `k`
    /// at `OC946_MBX + 4k`: 0 getDataSize, 1 getSaveData, 2 USTR_length).
    pub(crate) const OC946_MBX: u32 = DATA + 0x0060_1000;
    /// Fixture objects (self, LobbyMan, SSXApp, the card, the buffers), each
    /// within 0x4000 of this base.
    pub(crate) const OC946_OBJ: u32 = DATA + 0x0060_2000;

    /// Scrambled machine with the callee gate open.
    pub(crate) fn oc946_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = fakes::machine(c, seed, salt, 3);
        c.bus.write32(OC946_CTRL, OC946_MAGIC);
        r
    }

    /// A callee fake: logs the call (`fake_call`) and returns the generated
    /// body when the gate is closed. `$mbx` names a forced-result word.
    macro_rules! oc946_fake {
        ($name:ident, $target:expr, $gen:path) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(OC946_CTRL) != OC946_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                fakes::fake_call(c, $target);
            }
        };
        ($name:ident, $target:expr, $gen:path, mbx $slot:expr) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(OC946_CTRL) != OC946_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                fakes::fake_call(c, $target);
                c.g[V0][0] = c.bus.read32(OC946_MBX + 4 * $slot) as u64;
            }
        };
    }

    oc946_fake!(
        oc946_fake_25c610,
        0x0025_c610,
        ee::s06::f_0025c610__generated
    );
    oc946_fake!(
        oc946_fake_25cd50,
        0x0025_cd50,
        ee::s06::f_0025cd50__generated
    );
    oc946_fake!(
        oc946_fake_25fd50,
        0x0025_fd50,
        ee::s06::f_0025fd50__generated
    );
    oc946_fake!(
        oc946_fake_1a8a40,
        0x001a_8a40,
        ee::s03::f_001a8a40__generated
    );
    oc946_fake!(
        oc946_fake_1bfbb8,
        0x001b_fbb8,
        ee::s03::f_001bfbb8__generated
    );
    oc946_fake!(
        oc946_fake_1bf228,
        0x001b_f228,
        ee::s03::f_001bf228__generated
    );
    oc946_fake!(
        oc946_fake_317d70,
        0x0031_7d70,
        ee::s10::f_00317d70__generated
    );
    oc946_fake!(
        oc946_fake_14dd58,
        0x0014_dd58,
        ee::s01::f_0014dd58__generated
    );
    oc946_fake!(
        oc946_fake_227f80,
        0x0022_7f80,
        ee::s05::f_00227f80__generated
    );
    oc946_fake!(
        oc946_fake_14dc80,
        0x0014_dc80,
        ee::s01::f_0014dc80__generated
    );
    oc946_fake!(
        oc946_fake_14e048,
        0x0014_e048,
        ee::s01::f_0014e048__generated,
        mbx 0
    );
    oc946_fake!(
        oc946_fake_152728,
        0x0015_2728,
        ee::s01::f_00152728__generated
    );
    oc946_fake!(
        oc946_fake_14e0c0,
        0x0014_e0c0,
        ee::s01::f_0014e0c0__generated
    );
    oc946_fake!(
        oc946_fake_241200,
        0x0024_1200,
        ee::s06::f_00241200__generated
    );
    oc946_fake!(
        oc946_fake_2710a8,
        0x0027_10a8,
        ee::s07::f_002710a8__generated,
        mbx 1
    );
    oc946_fake!(
        oc946_fake_241138,
        0x0024_1138,
        ee::s06::f_00241138__generated
    );
    oc946_fake!(oc946_fake_3dbca0, 0x003d_bca0, ee::s14::f_003dbca0__generated, mbx 2);
    oc946_fake!(
        oc946_fake_317e30,
        0x0031_7e30,
        ee::s10::f_00317e30__generated
    );
    oc946_fake!(
        oc946_fake_2c2688,
        0x002c_2688,
        ee::s08::f_002c2688__generated
    );
    oc946_fake!(
        oc946_fake_3a0d00,
        0x003a_0d00,
        ee::s13::f_003a0d00__generated
    );
    oc946_fake!(
        oc946_fake_317e98,
        0x0031_7e98,
        ee::s10::f_00317e98__generated
    );
    oc946_fake!(
        oc946_fake_2caa58,
        0x002c_aa58,
        ee::s09::f_002caa58__generated
    );
    oc946_fake!(
        oc946_fake_2ce418,
        0x002c_e418,
        ee::s09::f_002ce418__generated
    );
    oc946_fake!(
        oc946_fake_2ccf98,
        0x002c_cf98,
        ee::s09::f_002ccf98__generated
    );
    oc946_fake!(
        oc946_fake_2ccdf0,
        0x002c_cdf0,
        ee::s09::f_002ccdf0__generated
    );
    oc946_fake!(
        oc946_fake_2cbcc8,
        0x002c_bcc8,
        ee::s09::f_002cbcc8__generated
    );
    oc946_fake!(
        oc946_fake_416958,
        0x0041_6958,
        ee::s15::f_00416958__generated
    );

    /// Hooks every callee fake once per process.
    pub(crate) fn oc946_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025C610),
            oc946_fake_25c610,
            fakes::prev_cell(oc946_fake_25c610),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025CD50),
            oc946_fake_25cd50,
            fakes::prev_cell(oc946_fake_25cd50),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025FD50),
            oc946_fake_25fd50,
            fakes::prev_cell(oc946_fake_25fd50),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A8A40),
            oc946_fake_1a8a40,
            fakes::prev_cell(oc946_fake_1a8a40),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001BFBB8),
            oc946_fake_1bfbb8,
            fakes::prev_cell(oc946_fake_1bfbb8),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001BF228),
            oc946_fake_1bf228,
            fakes::prev_cell(oc946_fake_1bf228),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc946_fake_317d70,
            fakes::prev_cell(oc946_fake_317d70),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc946_fake_14dd58,
            fakes::prev_cell(oc946_fake_14dd58),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00227F80),
            oc946_fake_227f80,
            fakes::prev_cell(oc946_fake_227f80),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc946_fake_14dc80,
            fakes::prev_cell(oc946_fake_14dc80),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014E048),
            oc946_fake_14e048,
            fakes::prev_cell(oc946_fake_14e048),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00152728),
            oc946_fake_152728,
            fakes::prev_cell(oc946_fake_152728),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014E0C0),
            oc946_fake_14e0c0,
            fakes::prev_cell(oc946_fake_14e0c0),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00241200),
            oc946_fake_241200,
            fakes::prev_cell(oc946_fake_241200),
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002710A8),
            oc946_fake_2710a8,
            fakes::prev_cell(oc946_fake_2710a8),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00241138),
            oc946_fake_241138,
            fakes::prev_cell(oc946_fake_241138),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DBCA0),
            oc946_fake_3dbca0,
            fakes::prev_cell(oc946_fake_3dbca0),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc946_fake_317e30,
            fakes::prev_cell(oc946_fake_317e30),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2688),
            oc946_fake_2c2688,
            fakes::prev_cell(oc946_fake_2c2688),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0D00),
            oc946_fake_3a0d00,
            fakes::prev_cell(oc946_fake_3a0d00),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc946_fake_317e98,
            fakes::prev_cell(oc946_fake_317e98),
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CAA58),
            oc946_fake_2caa58,
            fakes::prev_cell(oc946_fake_2caa58),
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CE418),
            oc946_fake_2ce418,
            fakes::prev_cell(oc946_fake_2ce418),
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CCF98),
            oc946_fake_2ccf98,
            fakes::prev_cell(oc946_fake_2ccf98),
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CCDF0),
            oc946_fake_2ccdf0,
            fakes::prev_cell(oc946_fake_2ccdf0),
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CBCC8),
            oc946_fake_2cbcc8,
            fakes::prev_cell(oc946_fake_2cbcc8),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00416958),
            oc946_fake_416958,
            fakes::prev_cell(oc946_fake_416958),
        );
    }

    #[test]
    fn c_fe_state_relogin_v31_matches() {
        oc946_install_fakes();
        check(
            ee::s04::f_001d5138__generated,
            c_fe_state_relogin_v31,
            200,
            &|c, seed| {
                let mut r = oc946_setup(c, seed, 0x5138);
                let msgs: [u64; 8] = [0xc9, 0xd4, 0x101, 0x111, 0x7, 0xd5, 0x200, 0xcb];
                c.g[A0][0] = sx(OC946_OBJ);
                c.g[A1][0] = msgs[(seed % 8) as usize];
                let lobby_slot = oc946_gp(c, 0xc8);
                let ssx_slot = oc946_gp(c, 0x848);
                c.bus.write32(lobby_slot, OC946_OBJ + 0x400);
                c.bus.write32(ssx_slot, OC946_OBJ + 0x800);
                c.bus.write32(OC946_OBJ + 0x800 + 0x78, OC946_OBJ + 0xc00);
                c.bus.write32(OC946_OBJ + 0x400 + 0x80, r.next_u32());
                c.bus.write32(OC946_OBJ + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_card_ui_read_save_chunk_matches() {
        oc946_install_fakes();
        check(
            ee::s04::f_001d5c28__generated,
            mem_card_ui_read_save_chunk,
            200,
            &|c, seed| {
                let mut r = oc946_setup(c, seed, 0x5c28);
                let sizes: [u64; 6] = [0, u64::MAX, 5, 0x4000, 0x5000, 0x10];
                c.g[A0][0] = sizes[(seed % 6) as usize];
                let card_slot = oc946_gp(c, 0x10c8);
                let ssx_slot = oc946_gp(c, 0x848);
                let card = OC946_OBJ + 0x3000;
                c.bus.write32(card_slot, card);
                c.bus.write32(ssx_slot, OC946_OBJ + 0x800);
                c.bus
                    .write32(card + 0x1bc, if seed % 4 == 0 { 2 } else { 1 });
                c.bus
                    .write32(card + 0x200, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(card + 0x1d0, OC946_OBJ + 0x3400);
                c.bus.write32(OC946_MBX, 8 + seed % 56);
                for i in 0..16 {
                    c.bus.write32(OC946_OBJ + 0x3400 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn mem_card_ui_write_replay_save_block_matches() {
        oc946_install_fakes();
        check(
            ee::s04::f_001d5d78__generated,
            mem_card_ui_write_replay_save_block,
            200,
            &|c, seed| {
                let mut r = oc946_setup(c, seed, 0x5d78);
                c.g[A0][0] = r.next_u32() as u64;
                let card_slot = oc946_gp(c, 0x10c8);
                let ssx_slot = oc946_gp(c, 0x848);
                let card = OC946_OBJ + 0x3000;
                c.bus.write32(card_slot, card);
                c.bus.write32(ssx_slot, OC946_OBJ + 0x800);
                c.bus
                    .write32(card + 0x1bc, if seed % 4 == 0 { 2 } else { 1 });
                c.bus.write32(OC946_OBJ + 0x800 + 0x84, OC946_OBJ + 0x3800);
                c.bus.write32(OC946_OBJ + 0x3800 + 0x28, OC946_OBJ + 0x3c00);
                c.bus
                    .write32(OC946_MBX + 4, if seed % 3 == 0 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn c_fe_state_net_help_popup_build_body_text_matches() {
        oc946_install_fakes();
        check(
            ee::s04::f_001da110__generated,
            c_fe_state_net_help_popup_build_body_text,
            200,
            &|c, seed| {
                let mut r = oc946_setup(c, seed, 0xa110);
                let ssx_slot = oc946_gp(c, 0x848);
                c.g[A0][0] = sx(OC946_OBJ + 0x2000);
                let this = OC946_OBJ + 0x2000;
                // the ids: 0 to 3 of them, the list stops at the first zero
                let n = (seed % 4) as u32;
                for i in 0..3u32 {
                    let id = if i < n { 0x100 + i } else { 0 };
                    c.bus.write32(this + 0x58 + 4 * i, id);
                }
                c.bus.write32(ssx_slot, OC946_OBJ + 0x800);
                c.bus.write32(OC946_OBJ + 0x800 + 0x8c, OC946_OBJ + 0x1000);
                c.bus.write32(OC946_OBJ + 0x1000 + 4, OC946_OBJ + 0x1400);
                c.bus.write16(OC946_OBJ + 0x1400 + 0x20, 0x10);
                c.bus.write32(OC946_OBJ + 0x1400 + 0x24, 0x0014_dc80);
                c.bus.write32(this + 0xac, OC946_OBJ + 0x1800);
                c.bus
                    .write32(OC946_MBX + 8, if seed % 5 == 0 { 0 } else { 5 + seed % 7 });
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn c_fe_state_net_help_popup_set_num_buttons_matches() {
        check(
            ee::s04::f_001da528__generated,
            c_fe_state_net_help_popup_set_num_buttons,
            100,
            &|c, seed| {
                let mut r = Rng(0xda528 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OC946_OBJ);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_init_create_account_page_matches() {
        oc946_install_fakes();
        check(
            ee::s04::f_001da648__generated,
            lobby_menu_init_create_account_page,
            200,
            &|c, seed| {
                let mut r = oc946_setup(c, seed, 0xa648);
                let ssx_slot = oc946_gp(c, 0x848);
                c.g[A0][0] = sx(OC946_OBJ + 0x2000);
                c.g[A1][0] = sx(OC946_OBJ + 0x2400);
                c.g[A2][0] = sx(OC946_OBJ + 0x2c00);
                c.bus.write32(OC946_OBJ + 0x2400, OC946_OBJ + 0x2800);
                c.bus.write32(ssx_slot, OC946_OBJ + 0x800);
                c.bus.write32(OC946_OBJ + 0x800 + 0xc4, r.next_u32());
                c.bus.write32(OC946_OBJ + 0x800 + 0xc8, r.next_u32());
            },
        );
    }
}
