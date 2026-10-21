//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold-path small functions, each with a differential test
//! against the generated body.
//!
//! The replacements keep their state in the guest registers the originals
//! use (callers may read `v0`/`v1`), follow the call protocol of
//! [`crate::call`] and the event rules of [`crate::events`].

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001b_cd18, cfestate_lobby_menu_transition_in, "cFEStateLobbyMenu_TransitionIn"),
    (0x0027_d688, script_obj_cam_set_binding, "ScriptObjCam_SetBinding"),
    (0x0012_c130, rider_gate_sub_update_push, "RiderGateSub_UpdatePush"),
    (0x0024_dc68, depth_fog_menu_dtor, "DepthFogMenu_dtor"),(0x002b_3ee8, music_player_enable_song_by_name, "MusicPlayer_EnableSongByName"),
    (0x0024_0860, mcmanager_state1a_delete, "MCManager_State1A_Delete"),(0x0014_c6a0, bolts_load_db, "Bolts_LoadDB"),
    (0x0019_79d8, cfestate_request_line_on_child_event, "cFEStateRequestLine_OnChildEvent"),
    (0x0019_8988, cfestate_request_line_open_buy_popup, "cFEStateRequestLine_OpenBuyPopup"),(0x002a_6f38, crowd_audio_unload, "CrowdAudio_Unload"),
    (0x003e_d578, tag_field_decode_hex, "TagFieldDecodeHex"),
    (0x0018_3b08, cfestate_uber_trick_select_left_panel, "cFEStateUberTrick_SelectLeftPanel"),
    (0x0018_3c08, cfestate_uber_trick_select_right_panel, "cFEStateUberTrick_SelectRightPanel"),];

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `lw` (sign-extended).
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}

/// `lh` (sign-extended).
#[inline(always)]
fn lh(c: &mut Cpu, a: u32) -> u64 {
    c.lh(a)
}

/// `lbu` (zero-extended).
#[inline(always)]
fn lbu(c: &mut Cpu, a: u32) -> u64 {
    c.lbu(a)
}

/// `sw`.
#[inline(always)]
fn sw(c: &mut Cpu, a: u32, v: u64) {
    c.sw(a, v)
}

/// The low 32 bits of a register as an address.
#[inline(always)]
fn a(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

// ---------------------------------------------------------------------------
// Functions.

/// `cFEStateLobbyMenu_TransitionIn(self, reset)` (0x1bcd18): entering the
/// lobby, reset the room tab and refresh the room lists; when called with
/// `reset == 0` and a room already chosen, clamp the selected player to the
/// room's player count and re-show the tab.
pub fn cfestate_lobby_menu_transition_in(c: &mut Cpu) {
    const REFRESH_ROOM_LISTS: Callee =
        Callee { addr: 0x001b_e0a8, f: ee::s03::cFEStateLobbyMenu_RefreshRoomLists };
    const UPDATE_PLAYER_ARROWS: Callee = Callee { addr: 0x001b_e720, f: ee::s03::cFEStateLobbyMenu_UpdatePlayerScrollArrows };
    const UPDATE_PLAYERS_IN_ROOM: Callee = Callee { addr: 0x001b_e3a8, f: ee::s03::cFEStateLobbyMenu_UpdatePlayersInRoom };
    const SHOW_ROOM_TAB: Callee = Callee { addr: 0x001b_df60, f: ee::s03::cFEStateLobbyMenu_ShowRoomTab };
    const UI_LISTBOX_SET_SELECTION: u32 = 0x0039_a7a8;
    const UI_MENU_SET_SELECTION: u32 = 0x0039_b960;

    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        let this = abi::arg(c, 0);
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        c.g[S1][0] = c.g[A0][0];
        let reset = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if reset {
            c.cycles += 15;
            let vt = Addr(this).off(8).lw(c);
            c.g[V0][0] = vt;
            c.g[A0][0] = Addr(vt as u32).off(0x100).lh(c);
            c.g[V1][0] = Addr(vt as u32).off(0x104).lw(c);
            c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001b_cd44)?;
            sw(c, p(this, 0x760), 0);
            c.g[A0][0] = c.g[S1][0];
            REFRESH_ROOM_LISTS.call(c, 0x001b_cd50)?;
            c.g[A0][0] = c.g[S1][0];
            UPDATE_PLAYER_ARROWS.call(c, 0x001b_cd58)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = lw(c, p(this, 0x718));
            UPDATE_PLAYERS_IN_ROOM.call(c, 0x001b_cd64)?;
            c.lq(16, frame.at(c, 0x20));
        } else {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p(this, 0x760));
            let no_room = c.g[V0][0] == 0;
            c.lq(16, frame.at(c, 0x20));
            if !no_room {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                REFRESH_ROOM_LISTS.call(c, 0x001b_cd80)?;
                c.g[V1][0] = lw(c, p(this, 0x764));
                c.g[V0][0] = ((c.g[V1][0] as i64) < 4) as u64;
                let out_of_range = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                if !out_of_range {
                    c.cycles += 7;
                    c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = lw(c, p(this, 0x768));
                    c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 0x708));
                    c.g[V0][0] = lbu(c, p(c.g[A0][0] as u32, 0x318));
                    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let clamp = c.g[V1][0] != 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    if !clamp {
                        c.cycles += 1;
                        sw(c, p(this, 0x768), c.g[V0][0]);
                    }
                    c.cycles += 2;
                    c.g[A1][0] = lbu(c, p(this, 0x768));
                    call_ptr(c, UI_LISTBOX_SET_SELECTION, 0x001b_cdb8)?;
                }
                c.cycles += 19;
                c.g[A0][0] = lw(c, p(this, 0x6d4));
                c.g[S0][0] = sx(this.wrapping_add(0x718));
                c.g[A1][0] = lbu(c, p(this, 0x764));
                call_ptr(c, UI_MENU_SET_SELECTION, 0x001b_cdc8)?;
                c.g[V0][0] = lw(c, p(this, 0x764));
                c.g[A0][0] = c.g[S1][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = lw(c, c.g[V0][0] as u32);
                UPDATE_PLAYERS_IN_ROOM.call(c, 0x001b_cde0)?;
                c.g[A0][0] = c.g[S1][0];
                UPDATE_PLAYER_ARROWS.call(c, 0x001b_cde8)?;
                c.g[V0][0] = lw(c, p(this, 0x764));
                c.g[A0][0] = c.g[S1][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = lw(c, c.g[S0][0] as u32);
                SHOW_ROOM_TAB.call(c, 0x001b_ce00)?;
                c.lq(16, frame.at(c, 0x20));
            }
        }
        c.cycles += 5;
        c.g[V0][0] = 0;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptObjCam_SetBinding(self, anim)` (0x27d688): bind a script-object
/// camera to an animation: tell the old binding process that its animation
/// is going away (`-1` for none), record the animation and its id from
/// `ScriptEngine_GetBindingProcess`, then tell the new process.
pub fn script_obj_cam_set_binding(c: &mut Cpu) {
    const GET_INSTANCE: Callee =
        Callee { addr: 0x0027_d2e8, f: ee::s07::ScriptEngine_GetInstance };
    const GET_BINDING_PROCESS: Callee =
        Callee { addr: 0x0027_d2f0, f: ee::s07::ScriptEngine_GetBindingProcess };

    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A1][0];
        let no_anim = c.g[S1][0] == 0;
        frame.save_ra(c, 0);
        if !no_anim {
            c.cycles += 9;
            GET_INSTANCE.call(c, 0x0027_d6b0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            GET_BINDING_PROCESS.call(c, 0x0027_d6bc)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x14));
            if c.g[V0][0] == 0 {
                sw(c, p(c.g[S0][0] as u32, 0x14), c.g[S1][0]);
            } else {
                c.cycles += 7;
                c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0xc));
                c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0x20));
                c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 0x28));
                c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x2c));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x0027_d6e4)?;
                sw(c, p(c.g[S0][0] as u32, 0x14), c.g[S1][0]);
            }
            c.cycles += 9;
            c.g[A1][0] = c.g[S2][0];
            sw(c, p(c.g[S0][0] as u32, 0x20), c.g[S2][0]);
            c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0xc));
            c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 0x20));
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x24));
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x0027_d704)?;
            c.lq(16, frame.at(c, 0x30));
        } else {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x14));
            let unbound = c.g[V0][0] == 0;
            c.g[V0][0] = 0x0050_0000;
            if !unbound {
                c.cycles += 22;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
                c.lqc2(1, c.g[V0][0] as u32);
                c.lqc2(2, p(c.g[V0][0] as u32, 0x10));
                c.lqc2(3, p(c.g[V0][0] as u32, 0x20));
                c.lqc2(4, p(c.g[V0][0] as u32, 0x30));
                c.sqc2(1, c.g[V1][0] as u32);
                c.sqc2(2, p(c.g[V1][0] as u32, 0x10));
                c.sqc2(3, p(c.g[V1][0] as u32, 0x20));
                c.sqc2(4, p(c.g[V1][0] as u32, 0x30));
                c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0xc));
                sw(c, p(c.g[S0][0] as u32, 0x70), 0);
                sw(c, p(c.g[S0][0] as u32, 0x74), 0);
                sw(c, p(c.g[S0][0] as u32, 0x78), 0);
                c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0x20));
                c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 0x28));
                c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x2c));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x0027_d764)?;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                sw(c, p(c.g[S0][0] as u32, 0x14), 0);
                sw(c, p(c.g[S0][0] as u32, 0x20), c.g[V0][0]);
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x30));
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderGateSub_UpdatePush(self)` (0x12c130): the rider-gate push
/// sub-state. Snap the slew targets to 1/0, then either the gate animation
/// finished (go to the sub-state after the push) or, while the rider is in
/// main state 3, wait for the push animation to pass halfway before jumping
/// to main state 1 and sub-state 0.
pub fn rider_gate_sub_update_push(c: &mut Cpu) {
    const SET_SLEW_TARGETS: Callee =
        Callee { addr: 0x0011_3f88, f: ee::s00::Rider_SetSlew3And4Targets };
    const GET_MAIN_STATE: Callee = Callee { addr: 0x0011_fe98, f: ee::s00::Rider_GetMainState };
    const SET_MAIN_STATE: Callee = Callee { addr: 0x0011_fe78, f: ee::s00::Rider_SetMainState };
    const SET_SUB_STATE: Callee = Callee { addr: 0x0011_fec8, f: ee::s00::Rider_SetSubState };
    const ANIM_GET_LAYER_ANIM_ID: u32 = 0x0031_2aa0;
    const ANIM_GET_LAYER_PROGRESS: u32 = 0x0031_2ab0;

    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 12); // f12 = 1.0
        frame.save(c, 16, 0x40);
        frame.save_ra(c, 0x30);
        c.g[S0][0] = c.g[A0][0];
        c.mtc1(0, 13); // f13 = 0.0
        c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0x14));
        SET_SLEW_TARGETS.call(c, 0x0012_c154)?;
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x14));
        c.g[A1][0] = 2;
        c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 0x784));
        call_ptr(c, ANIM_GET_LAYER_ANIM_ID, 0x0012_c164)?;
        c.g[V1][0] = 2;
        let anim_done = c.g[V0][0] != c.g[V1][0];
        if anim_done {
            // leave the push: main state 1 -> sub-state 4; else sub-state 0
            c.cycles += 5;
            c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0x14));
            GET_MAIN_STATE.call(c, 0x0012_c1f8)?;
            c.g[V1][0] = 1;
            let not_state_one = c.g[V0][0] != c.g[V1][0];
            c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0x14));
            if !not_state_one {
                c.cycles += 4;
                c.g[A1][0] = 4;
                SET_SUB_STATE.call(c, 0x0012_c20c)?;
                sw(c, c.g[S0][0] as u32, 0);
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0;
                SET_SUB_STATE.call(c, 0x0012_c21c)?;
                sw(c, c.g[S0][0] as u32, 0);
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x40));
        } else {
            c.cycles += 5;
            c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0x14));
            GET_MAIN_STATE.call(c, 0x0012_c178)?;
            c.g[V1][0] = 3;
            if c.g[V0][0] != c.g[V1][0] {
                c.lq(16, frame.at(c, 0x40));
            } else {
                c.cycles += 10;
                c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x14));
                c.g[A1][0] = 2;
                c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 0x784));
                call_ptr(c, ANIM_GET_LAYER_PROGRESS, 0x0012_c194)?;
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(1, 1); // f1 = 0.5
                c.c_lt_s(1, 0);
                if !c.fcc() {
                    // progress < 0.5: keep pushing
                    c.lq(16, frame.at(c, 0x40));
                } else {
                    c.cycles += 17;
                    c.g[V1][0] = lw(c, p(c.g[S0][0] as u32, 0x14));
                    c.g[A1][0] = 0;
                    c.lwc1(0, gp(c, -0x7838));
                    c.lq(2, p(c.g[V1][0] as u32, 0x1b0));
                    c.sq(2, frame.at(c, 0x10));
                    c.mfc1(4, 0);
                    c.lqc2(4, frame.at(c, 0x10));
                    c.qmtc2(4, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x20));
                    c.lq(2, frame.at(c, 0x20));
                    c.sq(2, frame.at(c, 0));
                    c.sq(2, p(c.g[V1][0] as u32, 0x1e0));
                    c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0x14));
                    SET_MAIN_STATE.call(c, 0x0012_c1e8)?;
                    c.lq(16, frame.at(c, 0x40));
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `DepthFogMenu_dtor(self, flags)` (0x24dc68): destroy the depth-fog menu's
/// items and pages, then the menu base.
pub fn depth_fog_menu_dtor(c: &mut Cpu) {
    const ITEM_DTOR: u32 = 0x002c_a280;
    const PAGE_DTOR: u32 = 0x002c_aa80;
    const MENU_DTOR: u32 = 0x002c_aa80;

    guest(c, |c| {
        c.cycles += 65;
        let frame = Frame::open(c, 0x40);
        let this = abi::arg(c, 0);
        frame.save(c, 16, 0x30);
        frame.save(c, 18, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(this.wrapping_add(0x2c4));
        const ITEMS: [u32; 16] = [
            0x904, 0x8e0, 0x8bc, 0x898, 0x874, 0x2dc, 0x2c4, 0x290, 0x25c, 0x22c, 0x1f8, 0x1c4,
            0x194, 0x164, 0x14c, 0x130,
        ];
        for (i, off) in ITEMS.iter().enumerate() {
            c.g[A0][0] = sx(this.wrapping_add(*off));
            c.g[A1][0] = 2;
            call_ptr(c, if *off == 0x2dc { PAGE_DTOR } else { ITEM_DTOR }, 0x0024_dc94 + 0xc * i as u32)?;
        }
        // The first entry is a page (MenuSysPage_dtor); the base dtor follows.
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, MENU_DTOR, 0x0024_dd54)?;
        c.lq(16, frame.at(c, 0x30));
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `PowerUpFXTogglesMenu_dtor(self, flags)` (0x24e0f8): destroy the
/// power-up FX toggles menu's items, then the menu base.
pub fn power_up_fx_toggles_menu_dtor(c: &mut Cpu) {
    const ITEM_DTOR: u32 = 0x002c_a280;
    const MENU_DTOR: u32 = 0x002c_aa80;

    guest(c, |c| {
        c.cycles += 65;
        let frame = Frame::open(c, 0x30);
        let this = abi::arg(c, 0);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        const ITEMS: [u32; 16] = [
            0x418, 0x3e8, 0x3b8, 0x388, 0x358, 0x328, 0x2f8, 0x2c8, 0x294, 0x260, 0x22c, 0x1f8,
            0x1c4, 0x194, 0x164, 0x14c,
        ];
        for (i, off) in ITEMS.iter().enumerate() {
            c.g[A0][0] = sx(this.wrapping_add(*off));
            c.g[A1][0] = 2;
            call_ptr(c, ITEM_DTOR, 0x0024_e11c + 0xc * i as u32)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, MENU_DTOR, 0x0024_e1e8)?;
        c.lq(16, frame.at(c, 0x20));
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MusicPlayer_EnableSongByName(self, name)` (0x2b3ee8): find a song by
/// name (`strstr` of the name in each entry) and mark it enabled in the
/// 64-bit enable mask, bumping the per-part counters; gives up when the
/// entry table is full (64 songs).
pub fn music_player_enable_song_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        let this = abi::arg(c, 0);
        frame.save(c, 16, 0x70);
        frame.save(c, 21, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x60);
        frame.save(c, 18, 0x50);
        frame.save(c, 19, 0x40);
        frame.save(c, 20, 0x30);
        frame.save(c, 22, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, p(this, 0x3e8));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 64) as u64;
        let full = c.g[V0][0] == 0;
        c.g[S5][0] = c.g[A1][0];
        let mut matched = false;
        if !full {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p(this, 0x3e4));
            c.g[S1][0] = 0;
            if (c.g[V0][0] as i64) > 0 {
                c.cycles += 5;
                c.g[S6][0] = 1;
                c.g[S2][0] = sx(this.wrapping_add(0x124));
                c.g[S4][0] = sx(this.wrapping_add(0x420));
                c.g[S3][0] = 0;
                'songs: loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, 0x0041_ad80, 0x002b_3f4c)?; // strstr
                    let found = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S6][0] << (c.g[S1][0] & 63);
                    if found {
                        matched = true;
                        c.cycles += 9;
                        c.g[V0][0] = c.ld(p(this, 0x3f8));
                        c.g[V1][0] = lw(c, p(this, 0x3e8));
                        c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S4][0] as u32));
                        c.g[V0][0] |= c.g[A0][0];
                        c.g[A1][0] = sx(this.wrapping_add(0x920));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                        c.sd(p(this, 0x3f8), c.g[V0][0]);
                        sw(c, p(this, 0x3e8), c.g[V1][0]);
                        c.g[V1][0] = 4;
                        loop {
                            c.cycles += 3;
                            c.g[V0][0] = lw(c, c.g[A2][0] as u32);
                            if c.g[V0][0] == 0 {
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = lw(c, c.g[A1][0] as u32);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                                sw(c, c.g[A1][0] as u32, c.g[V0][0]);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
                            }
                            c.cycles += 3;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                            let more = (c.g[V1][0] as i64) >= 0;
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
                            if !more {
                                c.cycles += 2;
                                break 'songs;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x002b_3f78);
                            }
                        }
                        matched = true;
                        break 'songs;
                    }
                    c.cycles += 6;
                    c.g[V0][0] = lw(c, p(this, 0x3e4));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xb));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x14));
                    if !more {
                        break 'songs;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_3f40);
                    }
                }
            }
        }
        if matched {
            c.lq(16, frame.at(c, 0x70));
        } else {
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x70));
        }
        c.cycles += 9;
        frame.restore(c, 17, 0x60);
        frame.restore(c, 18, 0x50);
        frame.restore(c, 19, 0x40);
        frame.restore(c, 20, 0x30);
        frame.restore(c, 21, 0x20);
        frame.restore(c, 22, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MCManager_State1A_Delete(self)` (0x240860): delete the selected item of
/// state 1A: ask the screen (or, when the delete countdown is done, the
/// focused list) whether its confirm handler wants to run, otherwise step
/// the countdown and dispatch the item's delete method.
pub fn mcmanager_state1a_delete(c: &mut Cpu) {
    const SCREEN_SLOT: u32 = 0x58;
    const LIST_SLOT: u32 = 0x248;

    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, p(this, 0x108));
        let expired = (c.g[V0][0] as i64) <= 0;
        c.g[A1][0] = lw(c, p(this, 0x434));
        let mut stage = 0u32; // 0 epilogue, 1 state 1c, 2 state 1b, 3 countdown
        if !expired {
            c.cycles += 7;
            c.g[V1][0] = lw(c, c.g[A1][0] as u32);
            c.g[A0][0] = lh(c, p(c.g[V1][0] as u32, SCREEN_SLOT));
            c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, SCREEN_SLOT + 4));
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x0024_0890)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = lw(c, p(this, 0x108));
                stage = 3;
            } else {
                c.cycles += 8;
                c.g[A1][0] = lw(c, p(this, 0x434));
                c.g[V1][0] = lw(c, c.g[A1][0] as u32);
                c.g[A0][0] = lh(c, p(c.g[V1][0] as u32, LIST_SLOT));
                c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, LIST_SLOT + 4));
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V0][0] as u32, 0x0024_08b0)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = lw(c, p(this, 0x748));
                    stage = 1;
                } else {
                    c.cycles += 8;
                    c.g[A2][0] = lw(c, p(this, 0x434));
                    c.g[A1][0] = lw(c, p(this, 0x428));
                    c.g[V0][0] = lw(c, c.g[A2][0] as u32);
                    c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 0x28));
                    c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x2c));
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, c.g[V1][0] as u32, 0x0024_08d4)?;
                    c.g[V0][0] = lw(c, p(this, 0x108));
                    stage = 3;
                }
            }
        } else {
            c.cycles += 7;
            c.g[V1][0] = lw(c, c.g[A1][0] as u32);
            c.g[A0][0] = lh(c, p(c.g[V1][0] as u32, SCREEN_SLOT));
            c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, SCREEN_SLOT + 4));
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x0024_08f8)?;
            if c.g[V0][0] != 0 {
                c.lq(16, frame.at(c, 0x10));
            } else {
                c.cycles += 8;
                c.g[A1][0] = lw(c, p(this, 0x434));
                c.g[V1][0] = lw(c, c.g[A1][0] as u32);
                c.g[A0][0] = lh(c, p(c.g[V1][0] as u32, LIST_SLOT));
                c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, LIST_SLOT + 4));
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V0][0] as u32, 0x0024_0918)?;
                let zero = c.g[V0][0] == 0;
                c.g[V0][0] = lw(c, p(this, 0x748));
                stage = if zero { 2 } else { 1 };
            }
        }
        match stage {
            3 => {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                sw(c, p(this, 0x108), c.g[V0][0]);
                c.cycles += 1;
                c.lq(16, frame.at(c, 0x10));
            }
            1 => {
                c.cycles += 7;
                c.g[A1][0] = 0x1c;
                c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 8));
                c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0xc));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x0024_0934)?;
                c.lq(16, frame.at(c, 0x10));
            }
            2 => {
                c.cycles += 5;
                c.g[A1][0] = 0x1b;
                c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 8));
                c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0xc));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x0024_0950)?;
                c.cycles += 1;
                c.lq(16, frame.at(c, 0x10));
            }
            _ => {
                c.lq(16, frame.at(c, 0x10));
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SHP_Create(a, b, c, d, e, f, g, h)` (0x3b2a68): format a texture header
/// into a stack buffer, allocate the calculated size, and initialise the
/// header; returns the new texture or null.
pub fn shp_create(c: &mut Cpu) {
    const CALC_ALLOC_SIZE: Callee =
        Callee { addr: 0x003b_26b8, f: ee::s13::SHP_CalcAllocSize };
    const INIT_HEADER: Callee = Callee { addr: 0x003b_27c8, f: ee::s13::SHP_InitHeader };
    const SPRINTF: u32 = 0x0041_7828;

    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, 18, 0x90);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 19, 0x80);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, 20, 0x70);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, 21, 0x60);
        c.g[S5][0] = c.g[T1][0];
        frame.save(c, 22, 0x50);
        c.g[S6][0] = c.g[A3][0];
        frame.save(c, 23, 0x40);
        c.g[S7][0] = c.g[T0][0];
        sw(c, frame.at(c, 0x10), c.g[T3][0]);
        c.g[FP][0] = c.g[T2][0];
        frame.save(c, 16, 0xb0);
        frame.save(c, 17, 0xa0);
        frame.save_ra(c, 0x20);
        c.g[T1][0] = c.g[FP][0];
        c.g[T2][0] = c.g[T3][0];
        CALC_ALLOC_SIZE.call(c, 0x003b_2ac0)?;
        c.g[S1][0] = c.g[V0][0];
        let none = c.g[S1][0] == 0;
        c.g[S0][0] = 0;
        if !none {
            c.cycles += 18;
            c.g[A1][0] = 0x0049_56b0;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S3][0];
            c.g[T0][0] = c.g[S4][0];
            call_ptr(c, SPRINTF, 0x003b_2ae8)?;
            c.g[V1][0] = 0x0045_0000;
            c.g[A1][0] = c.g[S1][0];
            c.g[V0][0] = lw(c, (c.g[V1][0] as u32).wrapping_sub(0x3bac));
            c.g[T0][0] = c.g[S5][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = 0;
            c.g[A3][0] = 0x10;
            call_ptr(c, c.g[V0][0] as u32, 0x003b_2b08)?;
            c.g[S0][0] = c.g[V0][0];
            let failed = c.g[S0][0] == 0;
            c.g[T3][0] = lw(c, frame.at(c, 0x10));
            if !failed {
                c.cycles += 8;
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = c.g[S4][0];
                c.g[T0][0] = c.g[S6][0];
                c.g[T1][0] = c.g[S7][0];
                c.g[T2][0] = c.g[FP][0];
                c.g[A0][0] = c.g[S0][0];
                INIT_HEADER.call(c, 0x003b_2b34)?;
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 17, 0xa0);
        frame.restore(c, 16, 0xb0);
        frame.restore(c, 18, 0x90);
        frame.restore(c, 19, 0x80);
        frame.restore(c, 20, 0x70);
        frame.restore(c, 21, 0x60);
        frame.restore(c, 22, 0x50);
        frame.restore(c, 23, 0x40);
        frame.restore(c, 30, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `Bolts_LoadDB(self)` (0x14c6a0): load the bolts database (read the file,
/// release the old one, index the four tables by character), then set bit 4
/// of every bolt entry's flags.
pub fn bolts_load_db(c: &mut Cpu) {
    const FREE_DB: Callee = Callee { addr: 0x0014_d240, f: ee::s01::Bolts_FreeDB };
    const READ_FILE: Callee = Callee { addr: 0x0014_be70, f: ee::s01::Bolts_ReadFile };
    const INDEX1: Callee = Callee { addr: 0x0014_c2b0, f: ee::s01::Bolts_IndexBoltsByChar };
    const INDEX2: Callee = Callee { addr: 0x0014_c320, f: ee::s01::Bolts_IndexTable2ByChar };
    const INDEX3: Callee = Callee { addr: 0x0014_c3c8, f: ee::s01::Bolts_IndexTable3ByChar };
    const INDEX4: Callee = Callee { addr: 0x0014_c488, f: ee::s01::Bolts_IndexTable4ByChar };

    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        let this = abi::arg(c, 0);
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, p(this, 0x10));
        if c.g[V0][0] == 0 {
            c.cycles += 15;
            c.g[S0][0] = lw(c, p(this, 4));
            c.g[S0][0] = (c.g[S0][0] < 1) as u64;
            c.g[A0][0] = c.g[S1][0];
            FREE_DB.call(c, 0x0014_c6cc)?;
            c.g[A0][0] = c.g[S1][0];
            READ_FILE.call(c, 0x0014_c6d4)?;
            c.g[A0][0] = c.g[S1][0];
            INDEX1.call(c, 0x0014_c6dc)?;
            c.g[A0][0] = c.g[S1][0];
            INDEX2.call(c, 0x0014_c6e4)?;
            c.g[A0][0] = c.g[S1][0];
            INDEX3.call(c, 0x0014_c6ec)?;
            c.g[A0][0] = c.g[S1][0];
            INDEX4.call(c, 0x0014_c6f4)?;
            let skip = c.g[S0][0] != 0;
            if !skip {
                c.cycles += 7;
                c.g[V0][0] = 0x004a_0000;
                c.g[A0][0] = 0;
                c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
                c.g[T4][0] = 0x9b50;
                c.g[T2][0] = 0xf88;
                c.mult(0, T0, A0, T4);
                'outer: loop {
                    c.cycles += 4;
                    c.g[V1][0] = 0;
                    c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                    c.mult(0, A0, V1, T2);
                    loop {
                        c.cycles += 6;
                        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x28c));
                        let empty = (c.g[V1][0] as i64) <= 0;
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                        if !empty {
                            c.cycles += 2;
                            c.g[A1][0] = c.g[V1][0];
                            c.g[A3][0] = 0xffff_ffff_ffff_ffef;
                            loop {
                                c.cycles += 4;
                                c.g[V1][0] = c.lhu(p(c.g[A0][0] as u32, 2));
                                c.g[V0][0] = c.g[V1][0] & 4;
                                let has4 = c.g[V0][0] != 0;
                                c.g[V0][0] = c.g[V1][0] | 0x10;
                                if !has4 {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.g[V1][0] & c.g[A3][0];
                                }
                                c.cycles += 4;
                                c.sh(p(c.g[A0][0] as u32, 2), c.g[V0][0]);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                                let more = c.g[A1][0] != 0;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                                if !more {
                                    break;
                                }
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014_c748);
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[V1][0] = c.g[A2][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 10) as u64;
                        let more = c.g[V0][0] != 0;
                        c.mult(0, A0, V1, T2);
                        if !more {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[T1][0];
                            c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
                            if c.g[V0][0] != 0 {
                                c.mult(0, T0, A0, T4);
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014_c718);
                                }
                                continue 'outer;
                            }
                            c.cycles += 1;
                            break 'outer;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0014_c728);
                        }
                    }
                }
            }
            c.lq(16, frame.at(c, 0x20));
        } else {
            c.lq(16, frame.at(c, 0x20));
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateRequestLine_OnChildEvent(self, arg, event)` (0x1979d8): the
/// request-line screen's child events: event 0x15 clears the "selected"
/// flag, event 0x16 sets it and refreshes the list when the child is the
/// current song.
pub fn cfestate_request_line_on_child_event(c: &mut Cpu) {
    const REFRESH_LIST: Callee =
        Callee { addr: 0x0019_7b88, f: ee::s02::cFEStateRequestLine_RefreshList };
    const UPDATE_HELP: Callee =
        Callee { addr: 0x0019_81f8, f: ee::s02::cFEStateRequestLine_UpdateHelpText };
    const GET_ROW_VALUE: u32 = 0x0019_88d8;
    const SET_ROW_SELECTED: u32 = 0x0019_86d0;
    const UPDATE_ARROWS: u32 = 0x0019_85b0;
    const UPDATE_LIST: u32 = 0x0019_85f0;
    const SET_ROW_STATE: u32 = 0x0019_8340;

    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        let this = abi::arg(c, 0);
        c.g[V0][0] = 0x15;
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        c.g[S1][0] = c.g[A0][0];
        let is_15 = c.g[A2][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        let mut stage = 0u32; // 0 epilogue, 1 event 0x15, 2 event 0x16
        if is_15 {
            stage = 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x16;
            let is_16 = c.g[A2][0] == c.g[V0][0];
            c.lq(16, frame.at(c, 0x20));
            if is_16 {
                stage = 2;
            } else {
                c.cycles += 2;
                c.lq(17, frame.at(c, 0x10));
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        if stage == 1 {
            c.cycles += 4;
            c.g[V1][0] = lw(c, p(c.g[A1][0] as u32, 0xc));
            c.g[V0][0] = 7;
            let current = c.g[V1][0] == c.g[V0][0];
            c.g[V1][0] = 0xffff_ffff_ffff_fff7;
            if !current {
                c.cycles += 4;
                c.g[V0][0] = lw(c, p(this, 0x1c));
                c.g[V0][0] &= c.g[V1][0];
                sw(c, p(this, 0x1c), c.g[V0][0]);
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x20));
        } else {
            // event 0x16: set bit 3 of the "selected" mask, then refresh.
            c.cycles += 5;
            c.g[V1][0] = lw(c, p(this, 0x1c));
            c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
            c.g[V0][0] &= 1;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] | 8;
                sw(c, p(this, 0x1c), c.g[V0][0]);
            }
            c.g[V1][0] = lw(c, p(c.g[A1][0] as u32, 0xc));
            c.cycles += 3;
            c.g[V0][0] = 7;
            let current = c.g[V1][0] != c.g[V0][0];
            c.lq(16, frame.at(c, 0x20));
            if current {
                c.cycles += 1;
                c.lq(17, frame.at(c, 0x10));
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 3;
            c.g[V0][0] = lw(c, p(c.g[A1][0] as u32, 0x6c));
            let no_child = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            if no_child {
                c.cycles += 1;
                c.lq(17, frame.at(c, 0x10));
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 25;
            c.g[A2][0] = lw(c, p(this, 0x170));
            c.g[V0][0] = lw(c, p(this, 0x150));
            c.g[V1][0] = lw(c, p(c.g[A2][0] as u32, 0xa0));
            c.g[A1][0] = lw(c, p(c.g[V1][0] as u32, 0x18));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S0][0] = lw(c, p(c.g[V0][0] as u32, 0x4c));
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, GET_ROW_VALUE, 0x0019_7a88)?;
            c.g[A0][0] = c.g[S1][0];
            REFRESH_LIST.call(c, 0x0019_7a90)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, SET_ROW_SELECTED, 0x0019_7a9c)?;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, UPDATE_ARROWS, 0x0019_7aa4)?;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, UPDATE_LIST, 0x0019_7aac)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, SET_ROW_STATE, 0x0019_7ab8)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            UPDATE_HELP.call(c, 0x0019_7ac4)?;
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x20));
        }
        c.cycles += 1;
        c.lq(17, frame.at(c, 0x10));
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateRequestLine_OpenBuyPopup(self, song)` (0x198988): allocate and
/// construct the buy popup for `song`, count the song's credits, fill the
/// title/price line, and push the popup on the UI state stack.
pub fn cfestate_request_line_open_buy_popup(c: &mut Cpu) {
    const MEM_ALLOC: u32 = 0x0031_7d70;
    const POPUP_CTOR: u32 = 0x001c_aba8;
    const GET_SONG_TITLE: Callee =
        Callee { addr: 0x0019_8a88, f: ee::s03::cFEStateRequestLine_GetSongTitle };
    const GET_SONG_PRICE: Callee =
        Callee { addr: 0x0019_8ae8, f: ee::s03::cFEStateRequestLine_GetSongPrice };
    const SET_TRACK: u32 = 0x001c_a980;
    const SET_CREDIT: u32 = 0x001c_a900;
    const PUSH_POPUP: u32 = 0x0039_f290;

    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x60);
        let this = abi::arg(c, 0);
        let song = abi::arg(c, 1);
        c.g[A2][0] = 0;
        frame.save(c, 20, 0x10);
        c.g[A3][0] = 0;
        frame.save(c, 16, 0x50);
        c.g[S4][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0x0046_0000;
        frame.save(c, 18, 0x30);
        c.g[A0][0] = 0x70;
        frame.save(c, 19, 0x20);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x790));
        frame.save(c, 17, 0x40);
        frame.save_ra(c, 0);
        c.g[S2][0] = 0;
        call_ptr(c, MEM_ALLOC, 0x0019_89c8)?;
        c.g[A1][0] = lw(c, p(this, 0x10));
        c.g[A0][0] = c.g[V0][0];
        c.g[A3][0] = c.lb(p(this, 0x44));
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, POPUP_CTOR, 0x0019_89dc)?;
        c.g[A1][0] = c.ld(p(this, 0x160));
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = 0;
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.g[A1][0] >> (c.g[A0][0] & 63);
            c.g[V0][0] &= 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            c.g[V1][0] = ((c.g[A0][0] as i64) < 64) as u64;
            let more = c.g[V1][0] != 0;
            c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_89e8);
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S4][0];
        GET_SONG_TITLE.call(c, 0x0019_8a10)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = ((c.g[S2][0] as i64) < 6) as u64;
        let few = c.g[V0][0] != 0;
        c.g[A2][0] = 6;
        if few {
            c.cycles += 5;
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[S2][0] as u32));
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x48));
            call_ptr(c, SET_CREDIT, 0x0019_8a58)?;
            c.g[A0][0] = lw(c, p(this, 0x10));
        } else {
            c.cycles += 10;
            c.g[A1][0] = c.g[S4][0];
            c.g[A0][0] = c.g[S0][0];
            GET_SONG_PRICE.call(c, 0x0019_8a2c)?;
            c.g[A3][0] = lw(c, p(this, 0x168));
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x48));
            call_ptr(c, SET_TRACK, 0x0019_8a40)?;
            c.g[A0][0] = lw(c, p(this, 0x10));
        }
        c.cycles += 11;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, PUSH_POPUP, 0x0019_8a68)?;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateSelectPersona_FillPersonaList(self, list)` (0x1ad3e8): fill the
/// list box with one entry per persona in the global persona manager, giving
/// each entry its index, then the four fixed persona slots.
pub fn cfestate_select_persona_fill_persona_list(c: &mut Cpu) {
    const LIST_GET_AT: u32 = 0x0039_7870;
    const LISTBOX_ADD_ENTRY: u32 = 0x003a_0c50;
    const STR_ID: u32 = 0x0031_7670;
    const LISTBOX_SET_STR: u32 = 0x003a_1000;

    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.g[S3][0] = lw(c, gp(c, -0xc8));
        frame.save(c, 19, 0x20);
        frame.save(c, 17, 0x40);
        frame.save(c, 20, 0x10);
        frame.save(c, 16, 0x50);
        frame.save(c, 18, 0x30);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, p(c.g[S3][0] as u32, 0x6c));
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S4][0] = c.g[A1][0];
        if !empty {
            c.cycles += 2;
            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x70));
            c.g[A1][0] = c.g[S1][0];
            loop {
                c.cycles += 19;
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x74));
                call_ptr(c, LIST_GET_AT, 0x001a_d428)?;
                c.g[S0][0] = c.g[V0][0];
                sw(c, p(c.g[S0][0] as u32, 0x18), c.g[S1][0]);
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[A1][0] = lw(c, c.g[S2][0] as u32);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
                call_ptr(c, LISTBOX_ADD_ENTRY, 0x001a_d444)?;
                c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 8));
                c.g[A1][0] = 1;
                c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 0x48));
                c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x4c));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x001a_d45c)?;
                c.g[V0][0] = lw(c, p(c.g[S3][0] as u32, 0x6c));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001a_d420);
                }
            }
        }
        // Four fixed persona slots.
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S1][0] as i64) < 4) as u64;
        let full = c.g[V0][0] == 0;
        c.g[S2][0] = 0x0046_0000;
        if !full {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            loop {
                c.cycles += 19;
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x74));
                call_ptr(c, LIST_GET_AT, 0x001a_d488)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2e58));
                sw(c, p(c.g[S0][0] as u32, 0x18), c.g[S1][0]);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                call_ptr(c, STR_ID, 0x001a_d49c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, LISTBOX_SET_STR, 0x001a_d4a8)?;
                c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 8));
                c.g[A1][0] = 1;
                c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 0x48));
                c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x4c));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x001a_d4c0)?;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 4) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001a_d480);
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CrowdAudio_Unload(self)` (0x2a6f38): stop all crowd audio, release the
/// instance array, then free the four sample lists of every bank.
pub fn crowd_audio_unload(c: &mut Cpu) {
    const STOP_ALL: Callee = Callee { addr: 0x002a_7718, f: ee::s08::CrowdAudio_StopAll };
    const FREE_LIST: Callee = Callee { addr: 0x002a_4cf8, f: ee::s08::CrowdAudio_FreeSampleList };
    const DESTROY: Callee =
        Callee { addr: 0x002a_4e38, f: ee::s08::CrowdAudio_DestroyInstance };
    const MEM_FREE: u32 = 0x0031_7e98;

    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        let this = abi::arg(c, 0);
        frame.save(c, 17, 0x60);
        frame.save(c, 16, 0x70);
        frame.save(c, 18, 0x50);
        frame.save(c, 19, 0x40);
        frame.save(c, 20, 0x30);
        frame.save(c, 21, 0x20);
        frame.save(c, 22, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        STOP_ALL.call(c, 0x002a_6f64)?;
        c.g[A1][0] = lw(c, p(this, 0x18));
        let no_instances = c.g[A1][0] == 0;
        c.g[V0][0] = 0x190;
        if !no_instances {
            c.cycles += 5;
            c.g[V1][0] = lw(c, (c.g[A1][0] as u32).wrapping_sub(0x10));
            c.mult(0, A0, V1, V0);
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            if c.g[A1][0] == c.g[S0][0] {
                c.g[A0][0] = lw(c, p(this, 0x18));
            } else {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x190));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    DESTROY.call(c, 0x002a_6f94)?;
                    c.g[V0][0] = lw(c, p(this, 0x18));
                    let more = c.g[V0][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x190));
                    if !more {
                        c.cycles += 1;
                        c.g[A0][0] = lw(c, p(this, 0x18));
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002a_6f88);
                    }
                }
            }
            c.cycles += 4;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, MEM_FREE, 0x002a_6fac)?;
            sw(c, p(c.g[S1][0] as u32, 0x18), 0);
            sw(c, p(c.g[S1][0] as u32, 0x14), 0);
        }
        c.cycles += 7;
        c.g[S6][0] = sx((c.g[S1][0] as u32).wrapping_add(0x168));
        c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x34));
        c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c));
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c));
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x68));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x64));
        loop {
            c.cycles += 17;
            sw(c, c.g[S1][0] as u32, 0);
            c.g[A0][0] = c.g[S4][0];
            sw(c, c.g[S2][0] as u32, 0);
            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x54));
            sw(c, c.g[S0][0] as u32, 0);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x54));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x54));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x54));
            FREE_LIST.call(c, 0x002a_6ff4)?;
            c.g[A0][0] = c.g[S5][0];
            FREE_LIST.call(c, 0x002a_6ffc)?;
            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x54));
            c.g[A0][0] = c.g[S3][0];
            FREE_LIST.call(c, 0x002a_7008)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S6][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x54));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_6fd0);
            }
        }
        c.cycles += 10;
        frame.restore(c, 16, 0x70);
        frame.restore(c, 17, 0x60);
        frame.restore(c, 18, 0x50);
        frame.restore(c, 19, 0x40);
        frame.restore(c, 20, 0x30);
        frame.restore(c, 21, 0x20);
        frame.restore(c, 22, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `TagFieldDecodeHex(data, buffer, len)` (0x3ed578): decode `$`-prefixed
/// hex pairs. With a null buffer, count the pairs; otherwise decode up to
/// `len` pairs, returning -1 when the data is not `$`-prefixed or the length
/// is not positive.
///
/// Dispatcher-shaped: an event at a loop back edge may unwind; the machine
/// re-enters at the saved `pc`, so every stage is accepted here.
pub fn tag_field_decode_hex(c: &mut Cpu) {
    let mut pc = c.pc;
    loop {
        match pc {
            0x003e_d578 => {
                c.cycles += 3;
                c.g[T2][0] = c.g[A2][0];
                let t = c.g[A0][0] == 0;
                c.g[T0][0] = c.g[A1][0];
                pc = if t { 0x003e_d594 } else { 0x003e_d584 };
            }
            0x003e_d584 => {
                c.cycles += 4;
                c.g[V1][0] = c.lb(c.g[A0][0] as u32);
                c.g[V0][0] = 0x24;
                let t = c.g[V1][0] == c.g[V0][0];
                pc = if t { 0x003e_d59c } else { 0x003e_d594 };
            }
            0x003e_d594 => {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                abi::ret(c);
                return;
            }
            0x003e_d59c => {
                c.cycles += 2;
                let t = c.g[T0][0] != 0;
                c.g[T1][0] = 0;
                pc = if t { 0x003e_d5dc } else { 0x003e_d5a4 };
            }
            0x003e_d5a4 => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(1));
                c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                pc = 0x003e_d5b8;
            }
            0x003e_d5b0 => {
                c.cycles += 2;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
                c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                pc = 0x003e_d5b8;
            }
            0x003e_d5b8 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 0x30) as u64;
                let t = c.g[V0][0] != 0;
                pc = if t { 0x003e_d5d4 } else { 0x003e_d5c4 };
            }
            0x003e_d5c4 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[V0][0] < 0x30) as u64;
                let t = c.g[V0][0] == 0;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(2));
                if t {
                    if c.cycles >= c.next_event {
                        c.pc = 0x003e_d5b0;
                        if c.event() {
                            return;
                        }
                    }
                    pc = 0x003e_d5b0;
                } else {
                    pc = 0x003e_d5d4;
                }
            }
            0x003e_d5d4 => {
                c.cycles += 2;
                c.g[V0][0] = c.g[T1][0];
                abi::ret(c);
                return;
            }
            0x003e_d5dc => {
                c.cycles += 2;
                let t = (c.g[T2][0] as i64) <= 0;
                c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                if t {
                    if c.cycles >= c.next_event {
                        c.pc = 0x003e_d594;
                        if c.event() {
                            return;
                        }
                    }
                    pc = 0x003e_d594;
                } else {
                    pc = 0x003e_d5e4;
                }
            }
            0x003e_d5e4 => {
                c.cycles += 2;
                let t = (c.g[T2][0] as i64) <= 0;
                c.g[T1][0] = 0;
                pc = if t { 0x003e_d670 } else { 0x003e_d5ec };
            }
            0x003e_d5ec => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[V0][0] < 0x30) as u64;
                let t = c.g[V0][0] != 0;
                pc = if t { 0x003e_d670 } else { 0x003e_d5fc };
            }
            0x003e_d5fc => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[V0][0] < 0x30) as u64;
                let t = c.g[V0][0] != 0;
                c.g[V0][0] = 0x0049_0000;
                pc = if t { 0x003e_d670 } else { 0x003e_d60c };
            }
            0x003e_d60c => {
                c.cycles += 5;
                c.g[V1][0] = 0x0049_0000;
                c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x61c0));
                c.g[T3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x62c0));
                c.g[A1][0] = c.lbu(c.g[A3][0] as u32);
                pc = 0x003e_d620;
            }
            0x003e_d620 => {
                c.cycles += 12;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
                c.g[A0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(1));
                c.g[A2][0] = ((c.g[T1][0] as i64) < (c.g[T2][0] as i64)) as u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T4][0] as u32));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(2));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                c.g[V0][0] |= c.g[V1][0];
                c.sb(c.g[T0][0] as u32, c.g[V0][0]);
                let t = c.g[A2][0] == 0;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                pc = if t { 0x003e_d670 } else { 0x003e_d650 };
            }
            0x003e_d650 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                c.g[V0][0] = (c.g[V0][0] < 0x30) as u64;
                let t = c.g[V0][0] != 0;
                pc = if t { 0x003e_d670 } else { 0x003e_d660 };
            }
            0x003e_d660 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[V0][0] < 0x30) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = c.lbu(c.g[A3][0] as u32);
                    if c.cycles >= c.next_event {
                        c.pc = 0x003e_d620;
                        if c.event() {
                            return;
                        }
                    }
                    pc = 0x003e_d620;
                } else {
                    pc = 0x003e_d670;
                }
            }
            0x003e_d670 => {
                c.cycles += 2;
                c.g[V0][0] = c.g[T1][0];
                abi::ret(c);
                return;
            }
            _ => {
                c.jump_out(pc, 0x003e_d578, 0x003e_d678);
                return;
            }
        }
    }
}

/// `BoltFile_CompareModelSlots(row, id)` (0x19dd10): compare two model
/// slots of a bolt row by name: an empty/disabled slot sorts before a live
/// one; two live slots are compared with `strcasecmp`.
pub fn bolt_file_compare_model_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x10);
        let row = abi::arg(c, 0);
        let id = abi::arg(c, 1);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x0001_2000;
        c.g[T0][0] = 0x120;
        c.g[A3][0] = 3;
        c.g[V1][0] = lw(c, row);
        c.g[A2][0] = c.lhu(id);
        c.g[A0][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
        c.mult(0, A0, A0, A2);
        c.g[V1][0] &= 0xffff;
        c.mult(0, V1, V1, T0);
        c.g[V0][0] = lw(c, gp(c, -0x1784));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.mult(0, V1, A2, T0);
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = lw(c, c.g[A0][0] as u32);
        let mut check_other = false;
        let mut memcmp = false;
        let mut extra = 0u32; // 1: the `break b_19de08` path charges one more
        if c.g[V1][0] != c.g[A3][0] {
            c.g[A2][0] = lw(c, c.g[A1][0] as u32);
            check_other = true;
        } else {
            c.cycles += 4;
            c.g[A2][0] = lw(c, p(c.g[A0][0] as u32, 0x110));
            c.g[V0][0] = 8;
            if c.g[A2][0] != c.g[V0][0] {
                c.g[A2][0] = lw(c, c.g[A1][0] as u32);
                check_other = true;
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw(c, p(c.g[A0][0] as u32, 0x11c));
                if c.g[V0][0] != 0 {
                    c.g[A2][0] = lw(c, c.g[A1][0] as u32);
                    check_other = true;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, c.g[A1][0] as u32);
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    if t {
                        extra = 1;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = lw(c, p(c.g[A1][0] as u32, 0x110));
                        let t = c.g[V0][0] != c.g[A2][0];
                        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                        if t {
                            extra = 1;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = lw(c, p(c.g[A1][0] as u32, 0x11c));
                            let t = c.g[V0][0] == 0;
                            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                            if t {
                                memcmp = true;
                            } else {
                                c.cycles += 2;
                            }
                        }
                    }
                }
            }
        }
        if check_other {
            // sb is not live: 0; otherwise the same tests as sa, then 1.
            c.cycles += 3;
            c.g[V0][0] = 3;
            let t = c.g[A2][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            if t {
                extra = 1;
            } else {
                c.cycles += 4;
                c.g[V1][0] = lw(c, p(c.g[A1][0] as u32, 0x110));
                c.g[V0][0] = 8;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0;
                if t {
                    extra = 1;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, p(c.g[A1][0] as u32, 0x11c));
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = 0;
                    if t {
                        extra = 1;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = lw(c, c.g[A0][0] as u32);
                        let t = c.g[V0][0] != c.g[A2][0];
                        c.g[V0][0] = 1;
                        if t {
                            extra = 1;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = lw(c, p(c.g[A0][0] as u32, 0x110));
                            let t = c.g[V0][0] != c.g[V1][0];
                            c.g[V0][0] = 1;
                            if t {
                                extra = 1;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = lw(c, p(c.g[A0][0] as u32, 0x11c));
                                let t = c.g[V0][0] != 0;
                                c.g[V0][0] = 1;
                                if t {
                                    extra = 1;
                                } else {
                                    memcmp = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        if memcmp {
            c.cycles += 5;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
            call_ptr(c, 0x0041_aa88, 0x003e_de00)?; // strcasecmp
        } else {
            c.cycles += extra as u64;
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One `select panel` step: look up the item on the screen by string id and
/// set its enabled flag (or clear it when it is missing).
fn panel_select(c: &mut Cpu, this: u32, string_off: u32, flag: u64, ret: u32, charge: u32) -> crate::call::Flow {
    c.cycles += charge as u64;
    c.g[A0][0] = 0x0046_0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(string_off));
    call_ptr(c, 0x0031_7670, ret.wrapping_sub(0x2c))?; // StrHash
    c.g[A0][0] = lw(c, p(this, 0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ret.wrapping_sub(0x20))?; // UIScreen_FindObject
    c.g[A2][0] = c.g[V0][0];
    let missing = c.g[A2][0] == 0;
    c.g[A1][0] = flag;
    if !missing {
        c.cycles += 5;
        c.g[V0][0] = lw(c, p(c.g[A2][0] as u32, 8));
        c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 0x48));
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x4c));
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, ret)?;
    }
    Ok(())
}

/// `cFEStateUberTrick_SelectLeftPanel(self)` (0x183b08): mark the left-hand
/// UberTrick panel items enabled/disabled and refresh their state.
pub fn cfestate_uber_trick_select_left_panel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        panel_select(c, this, 0x2700, 0, 0x0018_3b50, 0)?;
        panel_select(c, this, 0x26f0, 1, 0x0018_3b88, 9)?;
        panel_select(c, this, 0x26e0, 1, 0x0018_3bc0, 9)?;
        panel_select(c, this, 0x26d0, 0, 0x0018_3bf8, 9)?;
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateUberTrick_SelectRightPanel(self)` (0x183c08): the mirrored
/// version of [`cfestate_uber_trick_select_left_panel`].
pub fn cfestate_uber_trick_select_right_panel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        panel_select(c, this, 0x2700, 1, 0x0018_3c50, 0)?;
        panel_select(c, this, 0x26f0, 0, 0x0018_3c88, 9)?;
        panel_select(c, this, 0x26e0, 0, 0x0018_3cc0, 9)?;
        panel_select(c, this, 0x26d0, 1, 0x0018_3cf8, 9)?;
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// FUNCS_END

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE, LOG_UNWIND_AT};
    use crate::difftest::{compare, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 120;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// Every compared state must match: hard, caller-saved (`soft`) and LO/HI.
    fn strict(g: GuestFn, h: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seeds {
            let r = compare(g, h, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard={:#?} soft={:#?} lohi={:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }

    /// The `BELibrary` singleton (`gp - 0x1f04`) and the world (`gp - 0x848`).
    fn globals(c: &mut Cpu) {
        let be = DATA + 0x3_0000;
        let world = DATA + 0x8_0000;
        w(c, GAME_GP.wrapping_sub(0x1f04), be);
        w(c, GAME_GP.wrapping_sub(0x848), world);
        let game = world + 0x1000;
        w(c, world + 0x84, game);
        let race = world + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }

    /// The engine's size-class pool allocator behind `gp + 0xdd8`.
    fn pool(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x32000),
            (0x2a7c, 0x25800),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        let nodes = DATA + 0x28_000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x400 * i as u32;
            for n in 0..8u32 {
                let node = base + 0x40 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x40 });
            }
            w(c, pool + head, base);
            w(c, GAME_GP + slot, pool);
        }
    }

    /// Scrambled state, the fake call hooks, `gp`, the globals and the pools.
    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        globals(c);
        pool(c);
        r
    }

    #[test]
    fn music_player_enable_song_by_name_matches() {
        strict(
            ssx3_ee::s08::f_002b3ee8__generated,
            music_player_enable_song_by_name,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x3ee8);
                let player = DATA + 0x1000;
                let name = DATA + 0x3000;
                w(c, player + 0x3e8, if seed % 5 == 0 { 64 } else { r.next_u32() % 60 });
                w(c, player + 0x3e4, 1 + r.next_u32() % 2);
                w(c, player + 0x3f8, r.next_u32());
                w(c, player + 0x3fc, r.next_u32());
                for i in 0..4 {
                    w(c, player + 0x920 + 4 * i, r.next_u32() % 3);
                }
                c.bus.write8(name, if seed % 3 == 0 { b'A' } else { 0 });
                c.bus.write8(name + 1, 0);
                c.g[A0][0] = sx(player);
                c.g[A1][0] = sx(name);
            },
        );
    }

    #[test]
    fn mcmanager_state1a_delete_matches() {
        strict(
            ssx3_ee::s06::f_00240860__generated,
            mcmanager_state1a_delete,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x0860);
                let this = DATA + 0x1000;
                let screen = DATA + 0x2000;
                let list = DATA + 0x3000;
                let vt = DATA + 0x4000;
                let item = DATA + 0x5000;
                let vt2 = DATA + 0x6000;
                for off in [0x58, 0x248, 0x28] {
                    c.bus.write16(vt + off, 0);
                    w(c, vt + off + 4, FAKE);
                }
                w(c, screen, vt);
                w(c, list, 0);
                w(c, this + 0x434, screen);
                w(c, this + 0x428, list);
                w(c, this + 0x748, item);
                w(c, item, vt2);
                c.bus.write16(vt2 + 8, 0);
                w(c, vt2 + 0xc, FAKE);
                w(c, this + 0x108, if seed % 2 == 0 { 0 } else { 2 });
                c.g[A0][0] = sx(this);
            },
        );
    }

        #[test]
    fn bolts_load_db_matches() {
        strict(ssx3_ee::s01::f_0014c6a0__generated, bolts_load_db, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0xc6a0);
            let this = DATA + 0x1000;
            // A loaded database returns at once; the load path reads files
            // (a disc-load runaway in a test machine).
            w(c, this + 0x10, 1 + (r.next_u32() % 2));
            w(c, this + 4, r.next_u32());
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn cfestate_request_line_on_child_event_matches() {
        strict(
            ssx3_ee::s02::f_001979d8__generated,
            cfestate_request_line_on_child_event,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x79d8);
                let this = DATA + 0x1000;
                let child = DATA + 0x2000;
                w(c, this + 0x1c, r.next_u32());
                w(c, this + 0x170, DATA + 0x3000);
                w(c, this + 0x150, 0);
                w(c, child + 0xc, if seed % 2 == 0 { 7 } else { r.next_u32() % 16 });
                w(c, child + 0x6c, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(child);
                c.g[A2][0] = sx(match seed % 3 {
                    0 => 0x15,
                    1 => 0x16,
                    _ => r.next_u32() % 32,
                });
            },
        );
    }

    #[test]
    fn cfestate_request_line_open_buy_popup_matches() {
        strict(
            ssx3_ee::s03::f_00198988__generated,
            cfestate_request_line_open_buy_popup,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x8988);
                let this = DATA + 0x1000;
                let song = DATA + 0x2000;
                let state = DATA + 0x5000;
                w(c, this + 0x10, state);
                w(c, this + 0x44, r.next_u32() & 0x7f);
                w(c, this + 0x160, r.next_u32());
                w(c, this + 0x164, r.next_u32());
                w(c, this + 0x168, r.next_u32() % 5000);
                w(c, song, r.next_u32());
                w(c, GAME_GP.wrapping_sub(0xc8), DATA + 0x6000);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(song);
            },
        );
    }

        #[test]
    fn crowd_audio_unload_matches() {
        strict(ssx3_ee::s08::f_002a6f38__generated, crowd_audio_unload, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x6f38);
            let this = DATA + 0x1000;
            // No instance array and no samples: StopAll and the per-bank
            // free loops all return at once.
            w(c, this + 0x18, 0);
            w(c, this + 0x14, 0);
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn tag_field_decode_hex_matches() {
        strict(ssx3_ee::s14::f_003ed578__generated, tag_field_decode_hex, SEEDS, &|c, seed| {
            let _ = base(c, seed, 0xd578);
            let data = DATA + 0x1000;
            let buf = DATA + 0x2000;
            let len = DATA + 0x3000;
            let text: &[u8] = match seed % 6 {
                0 => b"\0",
                1 => b"XABCDEF\0",
                2 => b"$ABCDEF\0",
                3 => b"$ABCDEF\0",
                4 => b"$ABCDEF\0",
                _ => b"$AB\0",
            };
            for (i, b) in text.iter().enumerate() {
                c.bus.write8(data + i as u32, *b);
            }
            c.bus.write16(len, (seed % 4) as u16);
            c.g[A0][0] = sx(if seed % 6 == 0 { 0 } else { data });
            c.g[A1][0] = sx(if seed % 6 == 2 { 0 } else { buf });
            c.g[A2][0] = sx(c.bus.read32(len));
            c.pc = 0x003e_d578;
        });
    }

        #[test]
    fn cfestate_uber_trick_select_left_panel_matches() {
        strict(
            ssx3_ee::s02::f_00183b08__generated,
            cfestate_uber_trick_select_left_panel,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x3b08);
                let this = DATA + 0x1000;
                // No screen: every item lookup misses and only the flags of
                // the (absent) items are touched.
                w(c, this + 0x40, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn cfestate_uber_trick_select_right_panel_matches() {
        strict(
            ssx3_ee::s02::f_00183c08__generated,
            cfestate_uber_trick_select_right_panel,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x3c08);
                let this = DATA + 0x1000;
                w(c, this + 0x40, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn cfestate_lobby_menu_transition_in_matches() {
        strict(
            ssx3_ee::s03::f_001bcd18__generated,
            cfestate_lobby_menu_transition_in,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0xbcd18);
                let this = DATA + 0x1000;
                let vt = DATA + 0x2000;
                c.bus.write16(vt + 0x100, 0);
                w(c, vt + 0x104, FAKE);
                w(c, this + 8, vt);
                let reset = seed % 2 == 1;
                // Reset runs the vtable method (a fake); stop there. The
                // other half takes the "no room chosen" early return.
                w(c, this + 0x760, if reset { 1 } else { 0 });
                w(c, this + 0x764, r.next_u32() % 4);
                w(c, this + 0x768, r.next_u32() % 4);
                w(c, this + 0x6d4, DATA + 0x3000);
                for i in 0..4 {
                    w(c, this + 0x708 + 4 * i, DATA + 0x4000 + 0x100 * i);
                }
                w(c, this + 0x718, 0);
                w(c, DATA + 0x4000 + 0x318, 0);
                c.bus.write32(LOG_UNWIND_AT, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(if reset { 1 } else { 0 });
            },
        );
    }

    #[test]
    fn script_obj_cam_set_binding_matches() {
        strict(
            ssx3_ee::s07::f_0027d688__generated,
            script_obj_cam_set_binding,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0xd688);
                let obj = DATA + 0x1000;
                let vt = DATA + 0x2000;
                let anim = DATA + 0x3000;
                let instance = DATA + 0x5000;
                c.bus.write16(vt + 0x20, 0);
                w(c, vt + 0x24, FAKE);
                c.bus.write16(vt + 0x28, 0);
                w(c, vt + 0x2c, FAKE);
                w(c, obj + 0xc, vt);
                w(c, obj + 0x14, if seed % 3 == 0 { 0 } else { anim });
                w(c, obj + 0x20, r.next_u32());
                w(c, anim + 4, 0);
                w(c, GAME_GP.wrapping_add(0x3ec), instance);
                w(c, instance + 0x5c, 0);
                w(c, instance + 8, r.next_u32());
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(if seed % 3 == 2 { 0 } else { anim });
            },
        );
    }

    #[test]
    fn rider_gate_sub_update_push_matches() {
        strict(
            ssx3_ee::s00::f_0012c130__generated,
            rider_gate_sub_update_push,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0xc130);
                let gate = DATA + 0x1000;
                let rider = DATA + 0x2000;
                let base = DATA + 0x3000;
                let anim = DATA + 0x4000;
                w(c, gate + 0x14, rider);
                w(c, rider + 0x77c, base);
                w(c, base + 0xde0, [1u32, 3, 7, 8][(seed % 4) as usize]);
                w(c, rider + 0x784, anim);
                // Layer 2's animation id: 2 keeps pushing, otherwise done.
                w(c, anim + 8, if seed % 2 == 0 { 2 } else { r.next_u32() % 8 });
                // A two-entry sequence list whose last entry is at 1.0.
                let list = DATA + 0x5000;
                let n0 = DATA + 0x5100;
                let n1 = DATA + 0x5200;
                w(c, anim + 0x50, list);
                w(c, list + 4, n0);
                w(c, n0 + 0xc8, n1);
                w(c, n1 + 8, 0x3f80_0000);
                w(c, n1 + 0x10, 0x3f80_0000);
                c.g[A0][0] = sx(gate);
            },
        );
    }

    #[test]
    fn depth_fog_menu_dtor_matches() {
        strict(ssx3_ee::s06::f_0024dc68__generated, depth_fog_menu_dtor, SEEDS, &|c, seed| {
            let _ = base(c, seed, 0xdc68);
            c.g[A0][0] = sx(DATA + 0x1000);
            c.g[A1][0] = sx(if seed % 2 == 0 { 0 } else { 2 });
        });
    }

    }
