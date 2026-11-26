//! `src/fe/debugfe.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::rider::INPUT_BINDING_EVAL_BOOL;
pub(crate) use crate::game::be::beintnetwork::ldl;
pub(crate) use crate::game::be::beintnetwork::sdl;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_3;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_6;
pub(crate) use crate::game::fe::feasyncfile::gp_16 as gp_4;
pub(crate) use crate::game::fe::feasyncfile::gp_7 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_9 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::sw_7 as sw_2;
pub(crate) use crate::game::fe::fepopup::at as at_4;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_4;
pub(crate) use crate::game::fe::fepopup::oc508_at;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_3;
pub(crate) use crate::game::fe::festateoptions::gpa;
pub(crate) use crate::game::fe::festaterewards::gp_4 as gp_5;
pub(crate) use crate::game::fe::festaterewards::r;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::wscript::wscriptman::gp;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0017_d008,
        debug_menu_set_game_mode,
        "DebugMenu_SetGameMode",
    ),
    (
        0x0017_d200,
        lobby_menu_init_message_page,
        "LobbyMenu_InitMessagePage",
    ),
    (
        0x0017_d260,
        lobby_menu_lobby_event_hook,
        "LobbyMenu_LobbyEventHook",
    ),
    (0x0017_d688, lobby_menu_update, "LobbyMenu_Update"),
    (
        0x0017_e030,
        lobby_menu_create_account,
        "LobbyMenu_CreateAccount",
    ),
    (0x0017_e118, lobby_menu_login, "LobbyMenu_Login"),
    (
        0x0017_e1c0,
        lobby_menu_create_persona,
        "LobbyMenu_CreatePersona",
    ),
    (
        0x0017_e228,
        lobby_menu_delete_persona,
        "LobbyMenu_DeletePersona",
    ),
    (
        0x0017_e290,
        lobby_menu_select_persona,
        "LobbyMenu_SelectPersona",
    ),
    (0x0017_e2f8, lobby_menu_enter_room, "LobbyMenu_EnterRoom"),
    (
        0x0017_e368,
        lobby_menu_leave_occupants,
        "LobbyMenu_LeaveOccupants",
    ),
    (
        0x0017_e3b8,
        lobby_menu_show_occupant_detail,
        "LobbyMenu_ShowOccupantDetail",
    ),
    (0x0017_e418, lobby_menu_send_chat, "LobbyMenu_SendChat"),
    (
        0x0017_e490,
        lobby_menu_accept_challenge,
        "LobbyMenu_AcceptChallenge",
    ),
    (
        0x0017_e4f0,
        lobby_menu_decline_challenge,
        "LobbyMenu_DeclineChallenge",
    ),
    (
        0x0017_e558,
        lobby_menu_cancel_challenge,
        "LobbyMenu_CancelChallenge",
    ),
    (
        0x0017_f2d0,
        debug_menu_module_update_cursor_trail,
        "DebugMenuModule_UpdateCursorTrail",
    ),
    (
        0x0017_fa60,
        debug_menu_module_enter,
        "DebugMenuModule_Enter",
    ),
    (
        0x0017_fad0,
        debug_menu_module_shutdown,
        "DebugMenuModule_Shutdown",
    ),
    (0x0017_fb98, debug_menu_module_v06, "DebugMenuModule_v06"),
    (
        0x0017_fbf0,
        debug_menu_module_render,
        "DebugMenuModule_Render",
    ),
    (
        0x0018_0300,
        fe_ui_input_is_any_button,
        "FEUIInput_IsAnyButton",
    ),
    (0x0018_0478, feui_input_set_port, "FEUIInput_SetPort"),
    (0x0018_0498, feui_input_has_binding, "FEUIInput_HasBinding"),
    (
        0x0018_05d8,
        keyboard_screen_set_text,
        "KeyboardScreen_SetText",
    ),
    (
        0x0018_0618,
        keyboard_screen_cursor_left,
        "KeyboardScreen_CursorLeft",
    ),
    (
        0x0018_0678,
        keyboard_screen_cursor_right,
        "KeyboardScreen_CursorRight",
    ),
    (
        0x0018_06d8,
        keyboard_screen_cursor_up,
        "KeyboardScreen_CursorUp",
    ),
    (
        0x0018_0768,
        keyboard_screen_cursor_down,
        "KeyboardScreen_CursorDown",
    ),
    (
        0x0018_07c8,
        keyboard_screen_toggle_caps,
        "KeyboardScreen_ToggleCaps",
    ),
    (
        0x0018_09b0,
        keyboardscreen_get_key_object_name,
        "KeyboardScreen_GetKeyObjectName",
    ),
    (
        0x0018_0a90,
        keyboard_screen_get_cursor_object_name,
        "KeyboardScreen_GetCursorObjectName",
    ),
    (
        0x0018_04b8,
        feui_input_poll_usb_keyboard,
        "FEUIInput_PollUsbKeyboard",
    ),
    (0x001d_d530, fe_ui_input_dtor, "FEUIInput_dtor"),
    (
        0x001d_b148,
        lobby_menu_init_challenge_page,
        "LobbyMenu_InitChallengePage",
    ),
    (
        0x001d_b2b0,
        lobby_menu_init_abort_page,
        "LobbyMenu_InitAbortPage",
    ),
    (
        0x001d_b330,
        lobby_menu_fill_delete_persona_page,
        "LobbyMenu_FillDeletePersonaPage",
    ),
    (
        0x001d_b418,
        lobby_menu_fill_persona_pages,
        "LobbyMenu_FillPersonaPages",
    ),
    (
        0x001d_b528,
        lobby_menu_fill_room_list,
        "LobbyMenu_FillRoomList",
    ),
    (
        0x001d_b620,
        lobby_menu_fill_occupant_list,
        "LobbyMenu_FillOccupantList",
    ),
    (
        0x001d_bab8,
        lobby_menu_on_abort_page_action,
        "LobbyMenu_OnAbortPageAction",
    ),
    (
        0x001d_bad8,
        lobby_menu_on_challenge_page_action,
        "LobbyMenu_OnChallengePageAction",
    ),
    (
        0x001d_bb20,
        lobby_menu_on_chat_page_action,
        "LobbyMenu_OnChatPageAction",
    ),
    (
        0x001d_bb48,
        lobby_menu_on_occupants_page_action,
        "LobbyMenu_OnOccupantsPageAction",
    ),
    (
        0x001d_bb90,
        lobby_menu_on_select_room_page_action,
        "LobbyMenu_OnSelectRoomPageAction",
    ),
    (
        0x001d_bbc0,
        lobby_menu_on_select_persona_page_action,
        "LobbyMenu_OnSelectPersonaPageAction",
    ),
    (
        0x001d_bbe8,
        lobby_menu_on_create_persona_page_action,
        "LobbyMenu_OnCreatePersonaPageAction",
    ),
    (
        0x001d_bc10,
        lobby_menu_on_delete_persona_page_action,
        "LobbyMenu_OnDeletePersonaPageAction",
    ),
    (
        0x001d_bc38,
        lobby_menu_on_create_account_page_action,
        "LobbyMenu_OnCreateAccountPageAction",
    ),
    (
        0x001d_bc68,
        lobby_menu_on_login_page_action,
        "LobbyMenu_OnLoginPageAction",
    ),
    (
        0x001d_cda0,
        lobby_menu_destroy_login_page,
        "LobbyMenu_DestroyLoginPage",
    ),
    (
        0x001d_ce20,
        lobby_menu_destroy_create_account_page,
        "LobbyMenu_DestroyCreateAccountPage",
    ),
    (
        0x001d_cea0,
        lobby_menu_destroy_login_method_page,
        "LobbyMenu_DestroyLoginMethodPage",
    ),
    (
        0x001d_cfc8,
        lobby_menu_destroy_delete_persona_page,
        "LobbyMenu_DestroyDeletePersonaPage",
    ),
    (
        0x001d_d060,
        lobby_menu_destroy_create_persona_page,
        "LobbyMenu_DestroyCreatePersonaPage",
    ),
    (
        0x001d_d0c8,
        lobby_menu_destroy_select_persona_page,
        "LobbyMenu_DestroySelectPersonaPage",
    ),
    (
        0x001d_d258,
        lobby_menu_destroy_select_room_page,
        "LobbyMenu_DestroySelectRoomPage",
    ),
    (
        0x001d_d2f0,
        lobby_menu_destroy_room_occupants_page,
        "LobbyMenu_DestroyRoomOccupantsPage",
    ),
    (
        0x001d_d398,
        lobby_menu_destroy_chat_page,
        "LobbyMenu_DestroyChatPage",
    ),
    (
        0x001d_d440,
        lobby_menu_destroy_challenge_page,
        "LobbyMenu_DestroyChallengePage",
    ),
    (
        0x001d_d4c0,
        lobby_menu_destroy_abort_page,
        "LobbyMenu_DestroyAbortPage",
    ),
    (0x0017_fec0, feui_input_is_next, "FEUIInput_IsNext"),
    (0x0018_0040, feui_input_is_up, "FEUIInput_IsUp"),
    (0x0018_0060, feui_input_is_down, "FEUIInput_IsDown"),
    (0x0018_0240, feui_input_is_cam_zoom, "FEUIInput_IsCamZoom"),
    (0x0018_0260, feui_input_is_cam_pan, "FEUIInput_IsCamPan"),
    (
        0x0018_0280,
        feui_input_is_cam_rotate,
        "FEUIInput_IsCamRotate",
    ),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tagsunk::callee::LOBBY_MAN_FORWARD_EVENT_TO_UI;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::LOBBY_MENU_ON_LOBBY_EVENT;

    pub(crate) use crate::game::fe::fepopup::callee::C0017F500;

    pub(crate) use crate::game::fe::ovstatepause::callee::STRCPY;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRLEN;

    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_POP;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_SELECT;

    pub(crate) use crate::game::wscript::wscriptman::callee::APP_MODULE_UPDATE_FADE;

    pub(crate) use crate::game::wscript::wscriptman::callee::DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {

    pub(crate) use crate::game::scripter::ssxscriptengine::addr::FE_NET_SEND;

    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_POP_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_PUSH_PAGE;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {

    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_EVENT_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_GAME_MODE;
}

/// `int KeyboardScreen_ToggleCaps(KeyboardScreen* s)` (0x1807c8): flips the
/// caps-lock bit of the screen's flag word at +0xe4 and returns the new word.
pub fn keyboard_screen_toggle_caps(c: &mut Cpu) {
    c.cycles += 4;
    let flags = Addr(abi::arg(c, 0)).off(0xe4);
    let v = flags.lw(c) ^ 1;
    c.g[V0][0] = v;
    flags.sw(c, v);
    abi::ret(c);
}

/// `void LobbyMenu_LobbyEventHook(void* ev)` (0x17d260): forward the event to
/// the live lobby menu, then to the lobby manager's UI.
pub fn lobby_menu_lobby_event_hook(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x1da0)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let no_menu = c.g[A0][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if !no_menu {
            c.cycles += 2;
            callee::LOBBY_MENU_ON_LOBBY_EVENT.call(c, 0x0017_d280)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::LOBBY_MAN_FORWARD_EVENT_TO_UI.call(c, 0x0017_d288)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void KeyboardScreen_SetText(Screen* s, const char* text)` (0x1805d8):
/// copy `text` into the 0x80-byte buffer at `+0xec` and cache its length at
/// `+0x16c`.
pub fn keyboard_screen_set_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = sx(p(crate::abi::r32(c, S1), 0xec));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::STRCPY.call(c, 0x0018_05f8)?;
        c.g[A0][0] = c.g[S0][0];
        callee::STRLEN.call(c, 0x0018_0600)?;
        Addr(p(crate::abi::r32(c, S1), 0x16c)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DebugMenuModule_Enter()` (0x17fa60): release the current game
/// object, clear the debug menu's active flag, then ask the game object's
/// vtable slot 0x78/0x7c for a view-space point and pass it on.
pub fn debug_menu_module_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
        frame.save_ra(c, 0x10);
        call_ptr(c, 0x0036_9ff8, 0x0017_fa74)?;
        Addr((c.g[GP][0] as u32).wrapping_add(0x1234)).sw(c, 0);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x10d8).lw(c);
        let game = c.g[V0][0] as u32;
        let vt = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(vt).off(0x78).lh(c);
        Addr(c.g[SP][0] as u32).sw(c, 0);
        Addr((c.g[SP][0] as u32).wrapping_add(4)).sw(c, 0);
        c.g[A0][0] = sx(game.wrapping_add(c.g[A0][0] as u32));
        Addr((c.g[SP][0] as u32).wrapping_add(8)).sw(c, 0);
        c.g[V0][0] = Addr(vt).off(0x7c).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0017_faa0);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, target, 0x0017_faa0)?;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Lobby menu.

/// `void LobbyMenu_LeaveOccupants(LobbyMenu* menu)` (0x17e368): destroy every
/// occupant in the list at +0x8, then mark the menu left (state 0xd at +0x4).
/// 8 (+5 per occupant) + 7 cycles plus the callees.
pub fn lobby_menu_leave_occupants(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        let mut cur = p(self_, 8);
        let first = c.lw(p(self_, 8));
        c.g[V0][0] = first;
        c.g[S0][0] = sx(cur);
        if first != 0 {
            loop {
                c.cycles += 5;
                c.g[A0][0] = sx(cur);
                call_ptr(c, 0x002c_c460, 0x0017_e390)?;
                cur = c.g[S0][0] as u32;
                c.g[V0][0] = c.lw(cur);
                if c.g[V0][0] != 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x0017_e388);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 7;
        c.g[V0][0] = 0xd;
        frame.restore(c, 16, 0x20);
        c.sw(p(self_, 4), c.g[V0][0]);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DebugMenuModule_v06(DebugMenuModule* self)` (0x17fb98): poll the
/// debug menu's child module (when no world is up) and update the cursor
/// trail.
pub fn debug_menu_module_v06(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::APP_MODULE_UPDATE_FADE.call(c, 0x0017_fbac)?;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(4).lw(c);
        if c.g[V1][0] == 0 {
            c.cycles += 6;
            c.g[A1][0] = Addr(p(this, 0x14)).lw(c);
            c.g[V0][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x10).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x14).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x0017_fbd4)?;
        }
        c.cycles += 6;
        c.g[A0][0] = sx(this.wrapping_add(0x24));
        callee::DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL.call(c, 0x0017_fbdc)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMenu_ShowOccupantDetail(LobbyMenu* self)` (0x17e3b8): drain the
/// occupant list and open the detail panel for the selected occupant.
pub fn lobby_menu_show_occupant_detail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(this).off(0x8).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = sx(this.wrapping_add(0x8));
        if !empty {
            loop {
                c.cycles += 5;
                c.g[RA][0] = sx(0x0017_e3e0);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::LOBBY_OCCUPANT_POP, 0x0017_e3e0)?;
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e3d8);
                }
            }
        }
        c.cycles += 10;
        c.g[V0][0] = 0xe;
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x1534));
        Addr(r32(c, S1)).off(0x4).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x0017_e400);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x8));
        call_ptr(c, callee::LOBBY_OCCUPANT_SELECT, 0x0017_e400)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void KeyboardScreen_CursorDown(KeyboardScreen* self)` (0x180768): move
/// the cursor to the next row, wrapping through the hidden rows.
pub fn keyboard_screen_cursor_down(c: &mut Cpu) {
    c.cycles += 5;
    let a0 = r32(c, A0);
    c.g[V1][0] = Addr(a0).off(0x170).lw(c);
    c.g[A1][0] = sx(r32(c, V1).wrapping_sub(0x1e));
    c.g[V0][0] = (c.g[A1][0] < 0xa) as u64;
    let in_normal_range = c.g[V0][0] != 0;
    c.g[V0][0] = 0x1c;
    if !in_normal_range {
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = Addr(a0).off(0x170).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(a0).off(0xe0).lw(c);
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(a0).off(0x170).lw(c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x27;
                Addr(a0).off(0x170).sw(c, c.g[V0][0]);
                abi::ret(c);
                return;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0xa));
        Addr(a0).off(0x170).sw(c, c.g[V0][0]);
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(a0).off(0xd8).lw(c);
    let no_shift = c.g[V0][0] == 0;
    c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0x14));
    c.cycles += 2;
    if no_shift {
        Addr(a0).off(0x170).sw(c, c.g[V0][0]);
    } else {
        Addr(a0).off(0x170).sw(c, c.g[A1][0]);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front-end input (fe/feuiinput.cpp).

/// `void FEUIInput_SetPort(FEUIInput* self, int port)` (0x180478): record the
/// controller port at `self+0x4` and bind the per-port binding object
/// (`world->[port]` at `+0xb0`) at `self+0x18`.
pub fn feui_input_set_port(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = Addr(gpa(c, -0x848)).lw(c);
    c.g[A1][0] &= 0xff;
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xb0)).lw(c);
    let this = r32(c, A0);
    Addr(this.wrapping_add(0x4)).sb(c, c.g[A1][0]);
    Addr(this.wrapping_add(0x18)).sw(c, c.g[A2][0]);
    abi::ret(c);
}

/// `int FEUIInput_HasBinding(FEUIInput* self)` (0x180498): whether the bound
/// object at `+0x18` (two pointer hops) holds a non-zero action count.
pub fn feui_input_has_binding(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0x18)).lw(c);
    if c.g[V1][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Small helpers.

/// `reg + off` (32-bit address arithmetic).
#[inline(always)]
pub(crate) fn at(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// The low 32 bits of a register.
#[inline(always)]
pub(crate) fn r32_2(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

// ---------------------------------------------------------------------------
// Menus.

/// `void LobbyMenu_InitMessagePage(LobbyMenu* menu)` (0x17d200): initialise
/// the message-page child widget and its two labels.
pub fn lobby_menu_init_message_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let menu = c.g[A0][0];
        c.g[S0][0] = menu;
        call_ptr(c, 0x002c_aa58, 0x0017_d218)?;
        c.g[S1][0] = sx(at(c, S0, 0x130));
        c.g[V1][0] = 0x46_d740;
        c.g[A1][0] = 0x45_d060;
        c.g[A0][0] = c.g[S1][0];
        Addr(at(c, S0, 0x12c)).sw(c, c.g[V1][0]);
        call_ptr(c, 0x002c_cdf0, 0x0017_d238)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x0017_d248)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMenu_AcceptChallenge(LobbyMenu* menu)` (0x17e490): drain the
/// challenge queue, then show the accepted-challenge page.
pub fn lobby_menu_accept_challenge(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let menu = c.g[A0][0];
        c.g[S1][0] = menu;
        c.g[V0][0] = Addr(at(c, S1, 0x8)).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = sx(at(c, S1, 0x8));
        if !empty {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, 0x0017_e4b8)?;
                c.g[V0][0] = Addr(r32_2(c, S0)).lw(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e4b0);
                }
            }
        }
        c.cycles += 11;
        c.g[A1][0] = Addr(at(c, S1, 0x1c0c)).lw(c);
        c.g[V0][0] = 0xd;
        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
        c.g[A2][0] = 0;
        Addr(at(c, S1, 0x4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0025_fb58, 0x0017_e4dc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void KeyboardScreen_CursorLeft(KeyboardScreen* s)` (0x180618): move the
/// keyboard cursor one key to the left (cursor ids 0,10,20,30,39..).
pub fn keyboard_screen_cursor_left(c: &mut Cpu) {
    let obj = abi::arg(c, 0);
    'end: {
        c.cycles += 3;
        c.g[V1][0] = Addr(obj.wrapping_add(0x170)).lw(c);
        let at_zero = c.g[V1][0] == 0;
        c.g[V0][0] = 0xa;
        if at_zero {
            break 'end;
        }
        c.cycles += 2;
        let equal = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x14;
        if equal {
            break 'end;
        }
        c.cycles += 2;
        let equal = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x1e;
        if equal {
            break 'end;
        }
        c.cycles += 2;
        let different = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = 0x27;
        if different {
            'step: {
                c.cycles += 2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V0][0] = Addr(obj.wrapping_add(0x170)).lw(c);
                    break 'step;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(obj.wrapping_add(0xe0)).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(obj.wrapping_add(0x170)).lw(c);
                    break 'step;
                }
                c.cycles += 3;
                c.g[V0][0] = 0x25;
                Addr(obj.wrapping_add(0x170)).sw(c, c.g[V0][0]);
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            Addr(obj.wrapping_add(0x170)).sw(c, c.g[V0][0]);
            abi::ret(c);
            return;
        }
    }
    c.cycles += 4;
    c.g[V0][0] = Addr(obj.wrapping_add(0x170)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(9));
    Addr(obj.wrapping_add(0x170)).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void KeyboardScreen_CursorRight(KeyboardScreen* s)` (0x180678): move the
/// keyboard cursor one key to the right.
pub fn keyboard_screen_cursor_right(c: &mut Cpu) {
    let obj = abi::arg(c, 0);
    'end: {
        c.cycles += 4;
        c.g[V1][0] = Addr(obj.wrapping_add(0x170)).lw(c);
        c.g[V0][0] = 9;
        let mut equal = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x13;
        if equal {
            break 'end;
        }
        c.cycles += 2;
        equal = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x1d;
        if equal {
            break 'end;
        }
        c.cycles += 2;
        equal = c.g[V1][0] == c.g[V0][0];
        c.g[A1][0] = 0x27;
        if equal {
            break 'end;
        }
        c.cycles += 2;
        let different = c.g[V1][0] != c.g[A1][0];
        c.g[V0][0] = 0x25;
        if different {
            'step: {
                c.cycles += 2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V0][0] = Addr(obj.wrapping_add(0x170)).lw(c);
                    break 'step;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(obj.wrapping_add(0xe0)).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(obj.wrapping_add(0x170)).lw(c);
                    break 'step;
                }
                c.cycles += 2;
                Addr(obj.wrapping_add(0x170)).sw(c, c.g[A1][0]);
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            Addr(obj.wrapping_add(0x170)).sw(c, c.g[V0][0]);
            abi::ret(c);
            return;
        }
    }
    c.cycles += 4;
    c.g[V0][0] = Addr(obj.wrapping_add(0x170)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(9));
    Addr(obj.wrapping_add(0x170)).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `LobbyMenu_Update(menu)` 0x17d688: tick the two lobby sub-systems, update
/// the top of the menu stack when present, and run the menu's state machine.
pub fn lobby_menu_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(gp_2(c, -0x238)).lw(c);
        call_ptr(c, 0x0025_5a20, 0x0017_d6a0)?;
        c.g[A0][0] = Addr(gp_2(c, -0xc8)).lw(c);
        call_ptr(c, 0x0025_ac50, 0x0017_d6a8)?;
        c.g[V0][0] = lw_at(c, this, 8);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        if empty {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(this.wrapping_add(r32(c, V0)));
            c.g[V0][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = sx(p(this, 8));
            call_ptr(c, 0x002c_c0c0, 0x0017_d6d8)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0017_db18, 0x0017_d6e0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_CreatePersona(menu, arg)` 0x17e1c0: drain the persona list, then
/// enter state 8 and ask the lobby layer to create the persona.
pub fn lobby_menu_create_persona(c: &mut Cpu) {
    lobby_menu_drain(c, 8, 0x0017_e1f0, 0x0017_e1e8, 0x0017_e210, 0x0025_d1b8)
}

/// `LobbyMenu_DeletePersona(menu, arg)` 0x17e228: drain the list, enter state
/// 9 and ask the lobby layer to delete the persona.
pub fn lobby_menu_delete_persona(c: &mut Cpu) {
    lobby_menu_drain(c, 9, 0x0017_e258, 0x0017_e250, 0x0017_e278, 0x0025_d428)
}

/// `LobbyMenu_SelectPersona(menu, arg)` 0x17e290: drain the list, enter state
/// 7 and ask the lobby layer to select the persona.
pub fn lobby_menu_select_persona(c: &mut Cpu) {
    lobby_menu_drain(c, 7, 0x0017_e2c0, 0x0017_e2b8, 0x0017_e2e0, 0x0025_cd28)
}

/// The shared `list = &self->list; while (*list) pop(list); state = n; ...`
/// body of the three lobby persona hooks.
pub(crate) fn lobby_menu_drain(
    c: &mut Cpu,
    state: u32,
    loop_ret: u32,
    head: u32,
    ret: u32,
    call: u32,
) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        let this = abi::arg64(c, 0);
        let arg = abi::arg64(c, 1);
        c.g[S1][0] = this;
        frame.save_ra(c, 0);
        c.g[S2][0] = arg;
        c.g[V0][0] = lw_at(c, this as u32, 8);
        c.g[S0][0] = sx(p(this as u32, 8));
        if c.g[V0][0] != 0 {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, loop_ret)?;
                c.g[V0][0] = Addr(p(r32(c, S0), 0)).lw(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, head);
                }
            }
        }
        c.cycles += 11;
        c.g[V0][0] = sx(state);
        c.g[A0][0] = Addr(gp_2(c, -0xc8)).lw(c);
        Addr(p(this as u32, 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, call, ret)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_DeclineChallenge(menu)` 0x17e4f0: drain the list, clear the
/// pending challenge, enter state 12 and send the decline message.
pub fn lobby_menu_decline_challenge(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        c.g[V0][0] = lw_at(c, this, 8);
        c.g[S0][0] = sx(p(this, 8));
        if c.g[V0][0] != 0 {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, 0x0017_e518)?;
                c.g[V0][0] = Addr(p(r32(c, S0), 0)).lw(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e510);
                }
            }
        }
        c.cycles += 13;
        c.g[A0][0] = sx(p(this, 8));
        c.g[A1][0] = sx(p(this, 0x1044));
        call_ptr(c, 0x002c_c3b8, 0x0017_e530)?;
        c.g[A1][0] = lw_at(c, this, 0x1c0c);
        c.g[V0][0] = 0xc;
        c.g[A0][0] = Addr(gp_2(c, -0xc8)).lw(c);
        Addr(p(this, 4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0025_fc28, 0x0017_e544)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Lobby menu.

/// `LobbyMenu::EnterRoom(self, room)` (0x17e2f8): pop every page off the page
/// stack, move to room `room` and set state 0xb.
pub fn lobby_menu_enter_room(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 8)).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = sx(p(c.g[S1][0] as u32, 8));
        if !empty {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, 0x0017_e328)?;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                let more = c.g[V0][0] != 0;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e320);
                }
            }
        }
        c.cycles += 12;
        c.g[V0][0] = 0xb;
        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
        Addr(p(c.g[S1][0] as u32, 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = 0;
        call_ptr(c, 0x0025_d800, 0x0017_e34c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu::CancelChallenge(self)` (0x17e558): pop every page off the page
/// stack, push the challenge list, set state 0xc and cancel the challenge.
pub fn lobby_menu_cancel_challenge(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 8)).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = sx(p(c.g[S1][0] as u32, 8));
        if !empty {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, 0x0017_e580)?;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                let more = c.g[V0][0] != 0;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e578);
                }
            }
        }
        c.cycles += 14;
        c.g[A0][0] = sx(p(c.g[S1][0] as u32, 8));
        c.g[A1][0] = sx(p(c.g[S1][0] as u32, 0x1044));
        call_ptr(c, 0x002c_c3b8, 0x0017_e598)?;
        c.g[V0][0] = 0xc;
        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
        Addr(p(c.g[S1][0] as u32, 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 1;
        c.g[A2][0] = 0;
        call_ptr(c, 0x0025_f628, 0x0017_e5b0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end menus and screens.

/// `LobbyMenu_SendChat` (0x17E418): pop every queued page, push the lobby's
/// page back and hand the chat text to the network layer.
pub fn lobby_menu_send_chat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[V0][0] = lw_at_2(c, S1, 0x8);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = low(c, S1).wrapping_add(0x8);
        if !empty {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, addr::MENU_POP_PAGE, 0x0017_e448)?;
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e440);
                }
            }
        }
        c.cycles += 14;
        c.g[A0][0] = low(c, S1).wrapping_add(0x8);
        c.g[A1][0] = low(c, S1).wrapping_add(0x1044);
        call_ptr(c, addr::MENU_PUSH_PAGE, 0x0017_e460)?;
        c.g[V0][0] = 0xc;
        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
        sw_at(c, S1, 0x4, c.g[V0][0]);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, addr::FE_NET_SEND, 0x0017_e474)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void KeyboardScreen_CursorUp(KeyboardScreen* self)` (0x1806d8): move the
/// keyboard cursor up one row (skipping the page-jump cases), wrapping to the
/// last row.
pub fn keyboard_screen_cursor_up(c: &mut Cpu) {
    c.cycles += 4;
    let self_ = abi::arg(c, 0);
    c.g[V1][0] = Addr(p(self_, 0x170)).lw(c);
    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
    let in_first_row_band = c.g[V0][0] == 0;
    c.g[V0][0] = 8;
    if in_first_row_band {
        // Row 10..19.
        c.cycles += 4;
        c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0xa));
        c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(p(self_, 0x170)).lw(c);
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xa));
            Addr(p(self_, 0x170)).sw(c, c.g[V0][0]);
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(p(self_, 0xd8)).lw(c);
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr(p(self_, 0x170)).lw(c);
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xa));
            Addr(p(self_, 0x170)).sw(c, c.g[V0][0]);
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = 0x12;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = Addr(p(self_, 0x170)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(self_, 0xe0)).lw(c);
            let wrap = c.g[V0][0] == 0;
            c.g[V0][0] = 0x27;
            if wrap {
                // Straight to the common store: row 39.
                c.cycles += 2;
                Addr(p(self_, 0x170)).sw(c, c.g[V0][0]);
                abi::ret(c);
                return;
            }
            c.cycles += 1;
            c.g[V0][0] = Addr(p(self_, 0x170)).lw(c);
        }
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x14));
        Addr(p(self_, 0x170)).sw(c, c.g[V0][0]);
        abi::ret(c);
    } else {
        // Row 0..9.
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = Addr(p(self_, 0x170)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(self_, 0xe0)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = 0x27;
                c.cycles += 2;
                Addr(p(self_, 0x170)).sw(c, c.g[V0][0]);
                abi::ret(c);
                return;
            }
            c.g[V0][0] = Addr(p(self_, 0x170)).lw(c);
        }
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x1e));
        Addr(p(self_, 0x170)).sw(c, c.g[V0][0]);
        abi::ret(c);
    }
}

/// `void DebugMenu_SetGameMode(self, index)` (0x17d008): store the selected
/// mode into the debug slot table and apply it to the race interface.
pub fn debug_menu_set_game_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = Addr(gp_3(c, -0x848)).lw(c);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[A1][0] = 0;
        c.g[A0][0] = lw(c, V0, 0x78);
        call_ptr(c, table::BE_GET_INTERFACE, 0x0017_d034)?;
        c.g[A0][0] = 0x0053_0000;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = 0x0053_5bc8;
        c.g[A1][0] = 0x8c38;
        c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x49).lb(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x48).lb(c);
        c.g[A1][0] = c.g[S2][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A0][0] = c.g[S1][0];
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
        call_ptr(c, table::BE_NEW_RACE_SET_GAME_MODE, 0x0017_d068)?;
        c.g[S2][0] = sx((c.g[S2][0] as u32) << 2);
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        call_ptr(c, table::BE_NEW_RACE_SET_EVENT_TYPE, 0x0017_d07c)?;
        c.g[V1][0] = lw(c, S1, 0xc);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(8).lh(c);
        c.g[V0][0] = lw(c, V1, 0xc);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0017_d090)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMenu_Login(menu, a1, a2)` (0x17e118): pop the menu's pages,
/// copy the two account strings into the interface and log in.
pub fn lobby_menu_login(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        c.cycles += 12;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(at_3(c.g[S1][0], 8));
        c.g[V0][0] = lw_2(c, at_3(c.g[S1][0], 8));
        let t = c.g[V0][0] == 0;
        c.g[S3][0] = c.g[A2][0];
        if !t {
            loop {
                c.cycles += 5;
                c.g[RA][0] = sx(0x0017_e150);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, 0x0017_e150)?;
                c.g[V0][0] = lw_2(c, c.g[S0][0] as u32);
                let t = c.g[V0][0] != 0;
                if !t {
                    break;
                }
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0017_e148);
                }
            }
        }
        c.cycles += 25;
        c.g[A1][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0017_e168);
        c.g[A0][0] = sx(at_3(c.g[S1][0], 0x1c04));
        call_ptr(c, 0x0031_84e8, 0x0017_e168)?;
        c.g[A0][0] = sx(at_3(c.g[S1][0], 0x1c08));
        c.g[RA][0] = sx(0x0017_e174);
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0031_84e8, 0x0017_e174)?;
        c.g[A0][0] = lw_2(c, gp_4(c, -0xc8));
        c.g[V0][0] = 5;
        c.g[A1][0] = lw_2(c, at_3(c.g[S1][0], 0x1c04));
        c.g[A0][0] = sx(at_3(c.g[A0][0], 0x48));
        c.g[RA][0] = sx(0x0017_e18c);
        Addr(at_3(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_84e8, 0x0017_e18c)?;
        c.g[A0][0] = lw_2(c, gp_4(c, -0xc8));
        c.g[A1][0] = lw_2(c, at_3(c.g[S1][0], 0x1c08));
        c.g[RA][0] = sx(0x0017_e19c);
        c.g[A0][0] = sx(at_3(c.g[A0][0], 0x4c));
        call_ptr(c, 0x0031_84e8, 0x0017_e19c)?;
        c.g[RA][0] = sx(0x0017_e1a4);
        c.g[A0][0] = lw_2(c, gp_4(c, -0xc8));
        call_ptr(c, 0x0025_c610, 0x0017_e1a4)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `DebugMenuModule_Shutdown(this)` (0x17fad0): release the two debug
/// objects, clear the world's two debug counters and the font handle, and
/// stop the two debug sounds.
pub fn debug_menu_module_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x40);
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        c.g[S2][0] = abi::arg64(c, 0);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = sx(this.wrapping_add(0xc));
        frame.save_ra(c, 0);
        c.g[S1][0] = 1;

        loop {
            c.cycles += 3;
            c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
            let absent = c.g[A2][0] == 0;
            c.g[A1][0] = 3;
            if !absent {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0017_fb10)?;
            }
            c.cycles += 3;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0017_faf0);
            }
        }

        c.cycles += 6;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        Addr(c.g[V0][0] as u32).off(0x80).sw(c, 0);
        Addr(c.g[V0][0] as u32).off(0x7c).sw(c, 0);

        c.g[A1][0] = Addr(this).off(0x20).lw(c);
        let no_sound20 = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = Addr(gp(c, -0x854)).lw(c);
        if !no_sound20 {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x10d8).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x190).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x194).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0017_fb48)?;
        }

        c.cycles += 3;
        c.g[A1][0] = Addr(this).off(0x1c).lw(c);
        let no_sound1c = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = Addr(gp(c, -0x854)).lw(c);
        if !no_sound1c {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x10d8).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x190).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x194).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0017_fb68)?;
        }

        c.cycles += 3;
        c.g[A0][0] = Addr(this).off(0x18).lw(c);
        let no_font = c.g[A0][0] == 0;
        c.lq(16, frame.at(c, 0x30));
        if !no_font {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0039_16c0, 0x0017_fb7c)?;
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

// ---------------------------------------------------------------------------
// Front-end screens.

/// `void KeyboardScreen_GetCursorObjectName(int key, char *out, char *dst)`
/// (0x180a90): copy an eight-byte prefix and append the character (or the
/// named function key) for `key` into `out`.
pub fn keyboard_screen_get_cursor_object_name(c: &mut Cpu) {
    guest(c, |c| {
        'b_180b54: {
            'b_180ae4: {
                'b_180acc: {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.g[A0][0] = c.g[A2][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[V0][0] = 0x4a0000;
                    c.g[V1][0] = ((c.g[A1][0] as i64) < 10i64) as u64;
                    c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1368));
                    c.ldl(6, (c.g[T1][0] as u32).wrapping_add(0x7));
                    c.ldr(6, c.g[T1][0] as u32);
                    c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(6, c.g[A0][0] as u32);
                    let t = c.g[V1][0] != 0u64;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_180acc;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A1][0] as i64) < 36i64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x37));
                        if t {
                            break 'b_180ae4;
                        } else {
                            break 'b_180acc;
                        }
                    }
                }
                c.cycles += 6;
                c.sb((c.g[SP][0] as u32).wrapping_add(0x1), 0u64);
                c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                c.g[RA][0] = sx(0x00180adc);
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x004162d0, 0x00180adc)?;
                let t = true;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                if t {
                    break 'b_180b54;
                } else {
                    break 'b_180ae4;
                }
            }
            'b_180b04: {
                c.cycles += 3;
                c.g[V0][0] = 0x24;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x25;
                if t {
                    break 'b_180b04;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x00180afc);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1350));
                    call_ptr(c, 0x004162d0, 0x00180afc)?;
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_180b54;
                    } else {
                        break 'b_180b04;
                    }
                }
            }
            'b_180b20: {
                c.cycles += 2;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x26;
                if t {
                    break 'b_180b20;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = 0x460000;
                    c.g[RA][0] = sx(0x00180b18);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2aa0));
                    call_ptr(c, 0x004162d0, 0x00180b18)?;
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_180b54;
                    } else {
                        break 'b_180b20;
                    }
                }
            }
            'b_180b3c: {
                c.cycles += 2;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x27;
                if t {
                    break 'b_180b3c;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x00180b34);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1358));
                    call_ptr(c, 0x004162d0, 0x00180b34)?;
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_180b54;
                    } else {
                        break 'b_180b3c;
                    }
                }
            }
            c.cycles += 2;
            let t = c.g[A1][0] != c.g[V0][0];
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
            if t {
                break 'b_180b54;
            } else {
                c.cycles += 4;
                c.g[A1][0] = 0x4a0000;
                c.g[RA][0] = sx(0x00180b50);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1360));
                call_ptr(c, 0x004162d0, 0x00180b50)?;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_180b54;
            }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `void KeyboardScreen_GetKeyObjectName(char* dst, int key, char* out)`
/// (0x1809b0): copy the key's object name (or its printable character) into
/// `out`.
pub fn keyboardscreen_get_key_object_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x20));
        c.g[A0][0] = c.g[A2][0];
        c.sd(p(r32(c, SP), 0x10), c.g[RA][0]);
        c.g[V0][0] = 0x460000;
        c.g[V1][0] = ((c.g[A1][0] as i64) < 10i64) as u64;
        c.g[T1][0] = sx(r32(c, V0).wrapping_sub(0x2ab0));
        c.ldl(6, p(r32(c, T1), 7));
        c.ldr(6, r32(c, T1));
        c.g[A3][0] = Addr(p(r32(c, T1), 8)).lb(c);
        c.g[T0][0] = Addr(p(r32(c, T1), 9)).lb(c);
        c.sdl(6, p(r32(c, A0), 7));
        c.sdr(6, r32(c, A0));
        Addr(p(r32(c, A0), 8)).sb(c, c.g[A3][0]);
        Addr(p(r32(c, A0), 9)).sb(c, c.g[T0][0]);
        let digit = c.g[V1][0] != 0;
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(0x30));
        if !digit {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 36i64) as u64;
            let alphabet = c.g[V0][0] != 0;
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(0x37));
            if !alphabet {
                self::ks_special_chain(c)?;
                return Ok(());
            }
        }
        c.cycles += 6;
        Addr(p(r32(c, SP), 1)).sb(c, 0);
        Addr(r32(c, SP)).sb(c, c.g[V0][0]);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0041_62d0, 0x0018_0a0c)?;
        ks_finish(c);
        Ok(())
    })
}

/// The `@ : * #` branch of [`keyboardscreen_get_key_object_name`].
pub(crate) fn ks_special_chain(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = 0x24;
    let not_at = c.g[A1][0] != c.g[V0][0];
    c.g[V0][0] = 0x25;
    if !not_at {
        c.cycles += 5;
        c.g[A1][0] = 0x4a0000;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1350));
        call_ptr(c, 0x0041_62d0, 0x0018_0a2c)?;
        ks_finish(c);
        return Ok(());
    }
    c.cycles += 2;
    let not_colon = c.g[A1][0] != c.g[V0][0];
    c.g[V0][0] = 0x26;
    if !not_colon {
        c.cycles += 5;
        c.g[A1][0] = 0x460000;
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x2aa0));
        call_ptr(c, 0x0041_62d0, 0x0018_0a48)?;
        ks_finish(c);
        return Ok(());
    }
    c.cycles += 2;
    let not_star = c.g[A1][0] != c.g[V0][0];
    c.g[V0][0] = 0x27;
    if !not_star {
        c.cycles += 5;
        c.g[A1][0] = 0x4a0000;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1358));
        call_ptr(c, 0x0041_62d0, 0x0018_0a64)?;
        ks_finish(c);
        return Ok(());
    }
    c.cycles += 2;
    let not_hash = c.g[A1][0] != c.g[V0][0];
    c.g[RA][0] = c.ld(p(r32(c, SP), 0x10));
    if not_hash {
        ks_close(c);
        return Ok(());
    }
    c.cycles += 4;
    c.g[A1][0] = 0x4a0000;
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1360));
    call_ptr(c, 0x0041_62d0, 0x0018_0a80)?;
    ks_finish(c);
    Ok(())
}

/// The shared return of [`keyboardscreen_get_key_object_name`]: `ra` is
/// already restored, add the trailing cycles and leave.
pub(crate) fn ks_close(c: &mut Cpu) {
    let ra = r32(c, RA);
    c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x20));
    c.pc = ra;
}

/// `ra = ld(sp+0x10); cycles += 2; sp += 0x20; return`.
pub(crate) fn ks_finish(c: &mut Cpu) {
    c.cycles += 2;
    c.g[RA][0] = c.ld(p(r32(c, SP), 0x10));
    ks_close(c);
}

// ---------------------------------------------------------------------------
// Lobby.

/// `void LobbyMenu_CreateAccount(void* self, char* user, char* pass)`
/// (0x17e030): drain the menu's page list, store the new account name and
/// password into the lobby's strings and fill the pending account request
/// (user/pass/pass2/host, port 0x79e), then hand it to the lobby manager.
pub fn lobby_menu_create_account(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S0][0] = sx(p(r(c, S1), 0x8));
        c.g[S3][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p(r(c, S1), 0x8)).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);

        'drain: {
            if empty {
                break 'drain;
            }
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, 0x0017_e068)?;
                c.g[V0][0] = Addr(r(c, S0)).lw(c);
                let more = c.g[V0][0] != 0;
                if !more {
                    break 'drain;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e060);
                }
            }
        }

        c.cycles += 41;
        c.g[A0][0] = sx(p(r(c, S1), 0x1c04));
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0031_84e8, 0x0017_e080)?;
        c.g[A0][0] = sx(p(r(c, S1), 0x1c08));
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x0031_84e8, 0x0017_e08c)?;

        c.g[S0][0] = Addr(gp_5(c, -0xc8)).lw(c);
        c.g[V0][0] = 0x4;
        c.sw(p(r(c, S1), 0x4), c.g[V0][0]);
        c.g[A1][0] = c.g[S3][0];
        c.g[S1][0] = sx(p(r(c, S0), 0x260));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0031_84e8, 0x0017_e0a8)?;
        c.g[A0][0] = sx(p(r(c, S0), 0x264));
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x0031_84e8, 0x0017_e0b4)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = sx(p(r(c, S0), 0x268));
        call_ptr(c, 0x0031_84e8, 0x0017_e0c0)?;
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = sx(p(r(c, S0), 0x26c));
        c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x2db8));
        call_ptr(c, 0x0031_84e8, 0x0017_e0d0)?;

        c.g[A0][0] = Addr(gp_5(c, -0xc8)).lw(c);
        c.g[V1][0] = 1;
        c.g[V0][0] = 0x79e;
        c.sw(p(r(c, S0), 0x29c), 0);
        c.sw(p(r(c, S0), 0x298), 0);
        c.g[A1][0] = c.g[S1][0];
        c.sw(p(r(c, S0), 0x28c), c.g[V1][0]);
        c.sw(p(r(c, S0), 0x290), c.g[V0][0]);
        c.sw(p(r(c, S0), 0x294), c.g[V1][0]);
        c.sw(p(r(c, S0), 0x288), c.g[V1][0]);
        call_ptr(c, 0x0025_b9c8, 0x0017_e0fc)?;

        // The tail of 0x17e030 runs no extra block charge.
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_ctor(self)` (0x0017d298): construct the lobby menu screen and
/// its seven sub-panels, then bind the session and race-manager pointers.
pub fn lobby_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 69;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = sx(0x0046_d350);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        let this = c.g[S0][0];
        sw_2(c, this, 0x4, 0);
        c.g[A0][0] = sx((this as u32).wrapping_add(8));
        sw_2(c, this, 0, c.g[V0][0]);
        call_ptr(c, 0x002c_c018, 0x0017_d2c0)?;

        // `cUIWidget_ctor(widget, parent, ...)` pairs on the stack.
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.g[SP][0] | 4;
        sw_2(c, c.g[SP][0], 0, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x70));
        sw_2(c, c.g[SP][0], 4, this);
        call_ptr(c, 0x001d_a988, 0x0017_d2d8)?;

        c.g[A0][0] = sx((this as u32).wrapping_add(0x604));
        Callee {
            addr: 0x0017_d200,
            f: ee::s02::LobbyMenu_InitMessagePage,
        }
        .call(c, 0x0017_d2e0)?;

        c.g[A1][0] = c.g[SP][0] | 8;
        c.g[A2][0] = c.g[SP][0] | 0xc;
        sw_2(c, c.g[SP][0], 8, this);
        c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        sw_2(c, c.g[SP][0], 0xc, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x74c));
        sw_2(c, c.g[SP][0], 0x10, this);
        call_ptr(c, 0x001d_acb8, 0x0017_d300)?;

        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
        sw_2(c, c.g[SP][0], 0x14, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0xd2c));
        call_ptr(c, 0x001d_ae20, 0x0017_d310)?;

        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x18));
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1c));
        sw_2(c, c.g[SP][0], 0x18, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x1044));
        sw_2(c, c.g[SP][0], 0x1c, this);
        call_ptr(c, 0x001d_af08, 0x0017_d328)?;

        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        sw_2(c, c.g[SP][0], 0x20, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x1534));
        call_ptr(c, 0x001d_b028, 0x0017_d338)?;

        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x24));
        sw_2(c, c.g[SP][0], 0x24, this);
        sw_2(c, c.g[SP][0], 0x28, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x18c0));
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x28));
        call_ptr(c, 0x001d_b148, 0x0017_d350)?;

        sw_2(c, c.g[SP][0], 0x2c, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x1aac));
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2c));
        call_ptr(c, 0x001d_b2b0, 0x0017_d360)?;

        // Session / race-manager bindings.
        c.g[A0][0] = Addr(gp_6(c, 0xda0)).lw(c);
        c.g[V0][0] = this;
        c.g[A1][0] = Addr(gp_6(c, -0x848)).lw(c);
        sw_2(c, this, 0x1c0c, c.g[A0][0]);
        sw_2(c, this, 0x1c04, c.g[A0][0]);
        sw_2(c, this, 0x1c08, c.g[A0][0]);
        frame.restore_ra(c, 0x30);
        c.g[V1][0] = lw_3(c, c.g[A1][0], 0x80);
        c.g[A0][0] = lw_3(c, c.g[V1][0], 0x18);
        sw_2(c, this, 0x60, c.g[A0][0]);
        c.g[V1][0] = lw_3(c, c.g[A1][0], 0xb0);
        sw_2(c, this, 0x64, c.g[V1][0]);
        c.g[A0][0] = lw_3(c, c.g[A1][0], 0x80);
        c.g[V1][0] = lw_3(c, c.g[A0][0], 0x20);
        sw_2(c, this, 0x68, c.g[V1][0]);
        Addr(gp_6(c, 0x1da0)).sw(c, this);
        frame.restore(c, S0, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// The bindings `FEUIInput_IsAnyButton` tests, in the original's order.
pub(crate) const ANY_BUTTON_CODES: [u64; 16] = [
    0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7d, 0x7c, 0x90, 0x92, 0x91, 0x71,
];

/// `int FEUIInput_IsAnyButton(FEUIInput *input)` (0x180300): non-zero when any
/// of the sixteen menu buttons is held.
pub fn fe_ui_input_is_any_button(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;

        let mut last_held = false;
        for (i, code) in ANY_BUTTON_CODES.iter().enumerate() {
            if i > 0 {
                c.cycles += 5;
            }
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x18)).lw(c);
            c.g[A1][0] = *code;
            call_ptr(c, INPUT_BINDING_EVAL_BOOL, 0x0018_0324 + 0x14 * i as u32)?;
            if c.g[V0][0] != 0 {
                c.g[S1][0] = 1;
                last_held = i + 1 == ANY_BUTTON_CODES.len();
                break;
            }
        }
        // A held button costs one extra cycle; the last one costs two (it
        // falls through the tail check as well).
        if last_held {
            c.cycles += 2;
        } else if c.g[S1][0] != 0 {
            c.cycles += 1;
        }
        c.g[V0][0] = c.g[S1][0];

        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int func_0017FBF0(void* self)` (0x17fbf0): render one debug-menu frame.
/// When the module render hook (render-settings slot 17) accepts, reset the
/// cursor trail, set up the 2D view, call the module's own draw and render
/// fade, then let the menu at_4 `self+0x14` draw itself.
pub fn debug_menu_module_render(c: &mut Cpu) {
    let frame = Frame::open(c, 0x50);
    'b_17fdd4: {
        c.cycles += 14;
        c.g[A1][0] = lw_4(c, r32(c, GP).wrapping_sub(0x854));
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.swc1(21, at_4(c, SP, 0x48));
        c.swc1(20, at_4(c, SP, 0x40));
        c.g[V1][0] = lw_4(c, at_4(c, A1, 0x10d8));
        c.g[A0][0] = lh(c, at_4(c, V1, 0x88));
        c.g[V0][0] = lw_4(c, at_4(c, V1, 0x8c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0017fc20);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        if call_ptr(c, t, 0x0017fc20).is_err() {
            return;
        }
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0u64;
        if t {
            break 'b_17fdd4;
        } else {
            'b_17fdbc: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0017fc30);
                c.g[A0][0] = c.g[S0][0];
                if call_ptr(c, 0x00231d60, 0x0017fc30).is_err() {
                    return;
                }
                if c.g[V0][0] != 0u64 {
                    c.g[A1][0] = lw_4(c, r32(c, GP).wrapping_sub(0x854));
                    break 'b_17fdbc;
                } else {
                    c.cycles += 97;
                    c.g[V0][0] = lw_4(c, r32(c, GP).wrapping_sub(0x854));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 21);
                    c.g[V1][0] = lw_4(c, at_4(c, V0, 0x10d8));
                    c.g[A0][0] = lh(c, at_4(c, V1, 0x78));
                    sw_3(c, at_4(c, SP, 0x14), 0u64);
                    c.swc1(21, at_4(c, SP, 0x10));
                    c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.lwc1(20, at_4(c, SP, 0x14));
                    sw_3(c, at_4(c, SP, 0x18), 0u64);
                    sw_3(c, at_4(c, SP, 0x1c), 0u64);
                    c.swc1(20, r32(c, SP));
                    c.swc1(20, at_4(c, SP, 0x4));
                    c.swc1(20, at_4(c, SP, 0x8));
                    c.g[V0][0] = lw_4(c, at_4(c, V1, 0x7c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0017fc7c);
                    c.g[A1][0] = c.g[SP][0];
                    if call_ptr(c, t, 0x0017fc7c).is_err() {
                        return;
                    }
                    c.g[A3][0] = lw_4(c, r32(c, GP).wrapping_sub(0x854));
                    c.mov_s(12, 20);
                    c.g[AT][0] = 0x44200000;
                    c.mtc1(1, 14);
                    c.g[A2][0] = 0u64;
                    c.g[V0][0] = lw_4(c, at_4(c, A3, 0x10d8));
                    c.mov_s(17, 21);
                    c.g[AT][0] = 0x43f00000;
                    c.mtc1(1, 15);
                    c.mov_s(13, 12);
                    c.g[A0][0] = lh(c, at_4(c, V0, 0xd0));
                    c.g[A1][0] = 0u64;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 16);
                    c.g[V1][0] = lw_4(c, at_4(c, V0, 0xd4));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017fcc0);
                    c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x0017fcc0).is_err() {
                        return;
                    }
                    c.g[A2][0] = lw_4(c, r32(c, GP).wrapping_sub(0x854));
                    c.g[A1][0] = 0x500000;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0xe60));
                    c.g[V0][0] = lw_4(c, at_4(c, A2, 0x10d8));
                    c.g[A0][0] = lh(c, at_4(c, V0, 0x110));
                    c.g[V1][0] = lw_4(c, at_4(c, V0, 0x114));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017fce0);
                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x0017fce0).is_err() {
                        return;
                    }
                    c.g[A3][0] = lw_4(c, r32(c, GP).wrapping_sub(0x854));
                    c.g[V1][0] = 0xfffffffffe7f0000;
                    c.g[V1][0] = c.g[V1][0] | 0xffff;
                    c.g[T1][0] = 0x1000000;
                    c.g[A0][0] = lw_4(c, at_4(c, A3, 0xe84));
                    c.g[A1][0] = 0xffffffffffcf0000;
                    c.g[A1][0] = c.g[A1][0] | 0xffff;
                    c.g[T2][0] = 0x300000;
                    c.g[V0][0] = lw_4(c, at_4(c, A0, 0x4));
                    c.g[A2][0] = 0xfffffffffff00000;
                    c.g[A2][0] = c.g[A2][0] | 0xfff;
                    c.g[T3][0] = 0xffffffffffffff83;
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    c.g[T0][0] = 0xffffffffffbf0000;
                    c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                    c.g[T0][0] = c.g[T0][0] | 0xffff;
                    sw_3(c, at_4(c, A0, 0x4), c.g[V0][0]);
                    c.g[T1][0] = 0x400000;
                    c.g[A0][0] = sx(at_4(c, S0, 0x24));
                    c.g[V1][0] = lw_4(c, at_4(c, A3, 0xe84));
                    c.g[V0][0] = lw_4(c, at_4(c, V1, 0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[T2][0];
                    sw_3(c, at_4(c, V1, 0x4), c.g[V0][0]);
                    c.g[A1][0] = lw_4(c, at_4(c, A3, 0xe84));
                    c.g[V0][0] = lw_4(c, at_4(c, A1, 0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                    c.g[V0][0] = c.g[V0][0] | 0xd000;
                    sw_3(c, at_4(c, A1, 0x4), c.g[V0][0]);
                    c.g[V1][0] = lw_4(c, at_4(c, A3, 0xe84));
                    c.g[V0][0] = lw_4(c, at_4(c, V1, 0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
                    c.g[V0][0] = c.g[V0][0] | 0x14;
                    sw_3(c, at_4(c, V1, 0x4), c.g[V0][0]);
                    c.g[A1][0] = lw_4(c, at_4(c, A3, 0xe84));
                    c.g[V0][0] = lw_4(c, at_4(c, A1, 0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                    sw_3(c, at_4(c, A1, 0x4), c.g[V0][0]);
                    c.g[RA][0] = sx(0x0017fd84);
                    c.g[A1][0] = lw_4(c, at_4(c, S0, 0x1c));
                    if callee::C0017F500.call(c, 0x0017fd84).is_err() {
                        return;
                    }
                    c.g[V1][0] = lw_4(c, r32(c, GP).wrapping_sub(0x854));
                    c.g[A1][0] = 0xfffffffffffffc1f;
                    c.g[A0][0] = lw_4(c, at_4(c, V1, 0xe84));
                    c.g[V0][0] = lw_4(c, at_4(c, A0, 0x8));
                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                    c.g[V0][0] = c.g[V0][0] | 0x120;
                    sw_3(c, at_4(c, A0, 0x8), c.g[V0][0]);
                    c.g[A1][0] = lw_4(c, at_4(c, S0, 0x14));
                    c.g[V0][0] = lw_4(c, r32(c, A1));
                    c.g[A0][0] = lh(c, at_4(c, V0, 0x18));
                    c.g[V1][0] = lw_4(c, at_4(c, V0, 0x1c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017fdb8);
                    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x0017fdb8).is_err() {
                        return;
                    }
                    c.g[A1][0] = lw_4(c, r32(c, GP).wrapping_sub(0x854));
                    break 'b_17fdbc;
                }
            }
            c.cycles += 6;
            c.g[V0][0] = lw_4(c, at_4(c, A1, 0x10d8));
            c.g[A0][0] = lh(c, at_4(c, V0, 0xa0));
            c.g[V1][0] = lw_4(c, at_4(c, V0, 0xa4));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0017fdd0);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x0017fdd0).is_err() {
                return;
            }
            c.g[V0][0] = 0x1;
            break 'b_17fdd4;
        }
    }
    c.cycles += 6;
    frame.restore(c, S0, 0x30);
    c.g[RA][0] = ld(c, at_4(c, SP, 0x20));
    c.lwc1(21, at_4(c, SP, 0x48));
    c.lwc1(20, at_4(c, SP, 0x40));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_4(c, SP, 0x50));
    c.pc = t;
    return;
}

/// `void func_0017F2D0(void* p)` (0x17f2d0): advance the debug menu's firefly
/// cursor trail twice per frame: step the phase, wrap both the phase and the
/// position into `[0,1)`, advance the 64-entry ring at_4 `head` and store the
/// screen-space point.
pub fn debug_menu_module_update_cursor_trail(c: &mut Cpu) {
    let frame = Frame::open(c, 0xa0);
    c.cycles += 26;
    c.swc1(28, at_4(c, SP, 0x90));
    c.swc1(27, at_4(c, SP, 0x88));
    c.swc1(26, at_4(c, SP, 0x80));
    c.swc1(25, at_4(c, SP, 0x78));
    c.swc1(24, at_4(c, SP, 0x70));
    c.swc1(23, at_4(c, SP, 0x68));
    c.swc1(22, at_4(c, SP, 0x60));
    c.swc1(21, at_4(c, SP, 0x58));
    frame.save(c, S0, 0x40);
    frame.save(c, S1, 0x30);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0x20);
    c.g[S1][0] = 0x1;
    c.swc1(20, at_4(c, SP, 0x50));
    c.lwc1(28, r32(c, GP).wrapping_sub(0x5a54));
    c.lwc1(27, r32(c, GP).wrapping_sub(0x5a50));
    c.lwc1(26, r32(c, GP).wrapping_sub(0x5a4c));
    c.lwc1(25, r32(c, GP).wrapping_sub(0x5a48));
    c.lwc1(23, r32(c, GP).wrapping_sub(0x5a44));
    c.g[AT][0] = 0x3f000000;
    c.mtc1(1, 22);
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 21);
    c.g[AT][0] = 0x41a00000;
    c.mtc1(1, 24);
    'l_17f338: loop {
        'b_17f3b8: {
            c.cycles += 31;
            c.g[RA][0] = sx(0x0017f340);
            c.lwc1(12, at_4(c, S0, 0x8));
            if call_ptr(c, 0x0031bf60, 0x0017f340).is_err() {
                return;
            }
            c.lwc1(12, at_4(c, S0, 0xc));
            c.g[RA][0] = sx(0x0017f34c);
            c.mov_s(20, 0);
            if call_ptr(c, 0x0031bf60, 0x0017f34c).is_err() {
                return;
            }
            c.lwc1(1, at_4(c, S0, 0x8));
            c.add_s(20, 20, 28);
            c.add_s(0, 0, 28);
            c.lwc1(4, at_4(c, S0, 0xc));
            c.add_s(6, 1, 26);
            c.lwc1(2, r32(c, S0));
            c.mul_s(20, 20, 27);
            c.lwc1(3, at_4(c, S0, 0x4));
            c.mul_s(0, 0, 27);
            c.swc1(26, at_4(c, SP, 0x10));
            c.mul_s(1, 6, 23);
            c.swc1(25, at_4(c, SP, 0x14));
            c.add_s(2, 2, 20);
            c.swc1(20, r32(c, SP));
            c.add_s(3, 3, 0);
            c.swc1(0, at_4(c, SP, 0x4));
            c.add_s(1, 1, 22);
            c.swc1(6, at_4(c, S0, 0x8));
            c.add_s(4, 4, 25);
            c.cvt_w_s(5, 1);
            c.cvt_s_w(5, 5);
            c.swc1(2, r32(c, S0));
            c.c_lt_s(1, 5);
            c.swc1(3, at_4(c, S0, 0x4));
            let t = !c.fcc();
            c.swc1(4, at_4(c, S0, 0xc));
            if t {
                break 'b_17f3b8;
            } else {
                c.cycles += 1;
                c.sub_s(5, 5, 21);
                break 'b_17f3b8;
            }
        }
        'b_17f3e8: {
            c.cycles += 11;
            c.lwc1(7, r32(c, GP).wrapping_sub(0x5a40));
            c.mul_s(0, 4, 23);
            c.mul_s(1, 5, 7);
            c.add_s(0, 0, 22);
            c.cvt_w_s(5, 0);
            c.cvt_s_w(5, 5);
            c.sub_s(1, 6, 1);
            c.c_lt_s(0, 5);
            let t = !c.fcc();
            c.swc1(1, at_4(c, S0, 0x8));
            if t {
                break 'b_17f3e8;
            } else {
                c.cycles += 1;
                c.sub_s(5, 5, 21);
                break 'b_17f3e8;
            }
        }
        'b_17f414: {
            c.cycles += 10;
            c.mul_s(0, 2, 23);
            c.mul_s(1, 5, 7);
            c.add_s(0, 0, 22);
            c.sub_s(1, 4, 1);
            c.cvt_w_s(5, 0);
            c.cvt_s_w(5, 5);
            c.c_lt_s(0, 5);
            let t = !c.fcc();
            c.swc1(1, at_4(c, S0, 0xc));
            if t {
                break 'b_17f414;
            } else {
                c.cycles += 1;
                c.sub_s(5, 5, 21);
                break 'b_17f414;
            }
        }
        'b_17f440: {
            c.cycles += 10;
            c.mul_s(0, 3, 23);
            c.mul_s(1, 5, 7);
            c.add_s(0, 0, 22);
            c.sub_s(12, 2, 1);
            c.cvt_w_s(4, 0);
            c.cvt_s_w(4, 4);
            c.c_lt_s(0, 4);
            let t = !c.fcc();
            c.swc1(12, r32(c, S0));
            if t {
                break 'b_17f440;
            } else {
                c.cycles += 1;
                c.sub_s(4, 4, 21);
                break 'b_17f440;
            }
        }
        c.cycles += 35;
        c.g[V0][0] = lw_4(c, at_4(c, S0, 0x210));
        c.mul_s(0, 4, 7);
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x1));
        c.g[A0][0] = sx(at_4(c, V0, 0x1));
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        c.g[V0][0] = sx(at_4(c, V0, 0x40));
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[A0][0];
        }
        c.sub_s(0, 3, 0);
        c.g[V0][0] = sx((r32(c, V0) as i32 >> 6) as u32);
        c.g[V0][0] = sx(r32(c, V0) << 6);
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V0)));
        c.swc1(0, at_4(c, S0, 0x4));
        c.g[RA][0] = sx(0x0017f47c);
        sw_3(c, at_4(c, S0, 0x210), c.g[A0][0]);
        if call_ptr(c, 0x0031bf60, 0x0017f47c).is_err() {
            return;
        }
        c.mul_s(20, 0, 24);
        c.lwc1(12, at_4(c, S0, 0x4));
        c.lwc1(0, r32(c, GP).wrapping_sub(0x5a3c));
        c.g[RA][0] = sx(0x0017f490);
        c.add_s(20, 20, 0);
        if call_ptr(c, 0x0031bf60, 0x0017f490).is_err() {
            return;
        }
        c.mul_s(0, 0, 24);
        c.g[AT][0] = 0x43d20000;
        c.mtc1(1, 1);
        c.g[V0][0] = lw_4(c, at_4(c, S0, 0x210));
        c.swc1(20, at_4(c, SP, 0x10));
        c.add_s(0, 0, 1);
        c.g[V0][0] = sx(r32(c, V0) << 3);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
        c.swc1(0, at_4(c, SP, 0x14));
        ldl(c, 3, at_4(c, SP, 0x10));
        sdl(c, 3, at_4(c, V0, 0x10));
        let t = (c.g[S1][0] as i64) >= 0;
        c.g[RA][0] = ld(c, at_4(c, SP, 0x20));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x0017f338);
            }
            continue 'l_17f338;
        } else {
            c.cycles += 13;
            frame.restore(c, S0, 0x40);
            frame.restore(c, S1, 0x30);
            c.lwc1(28, at_4(c, SP, 0x90));
            c.lwc1(27, at_4(c, SP, 0x88));
            c.lwc1(26, at_4(c, SP, 0x80));
            c.lwc1(25, at_4(c, SP, 0x78));
            c.lwc1(24, at_4(c, SP, 0x70));
            c.lwc1(23, at_4(c, SP, 0x68));
            c.lwc1(22, at_4(c, SP, 0x60));
            c.lwc1(21, at_4(c, SP, 0x58));
            c.lwc1(20, at_4(c, SP, 0x50));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx(at_4(c, SP, 0xa0));
            c.pc = t;
            return;
        }
    }
}

// --------------------------------------------------- FEUIInput_PollUsbKeyboard

/// `cUsbKeyboard*` at gp+0x300 (0 when no USB keyboard is set up).
pub(crate) const FEUI_USB_KEYBOARD_GP: u32 = 0x300;

/// FEUIInput: key found (+0xc), key code (+0x10), modifier (+0x14).
pub(crate) const FEUI_KEY_FOUND: u32 = 0xc;

pub(crate) const FEUI_KEY_CODE: u32 = 0x10;

pub(crate) const FEUI_KEY_MOD: u32 = 0x14;

/// `void FEUIInput_PollUsbKeyboard(FEUIInput* self)` (0x1804b8): pops the
/// next key of USB keyboard channel 0 (`cUsbKeyboard_GetKey`) into the
/// input's key / modifier, `found` = whether there was one.
pub fn feui_input_poll_usb_keyboard(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let input = Addr(r32(c, S0));
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(FEUI_USB_KEYBOARD_GP)).lw(c);
        input.off(FEUI_KEY_FOUND).sw(c, 0);
        if c.g[A0][0] != 0 {
            c.cycles += 8;
            input.off(FEUI_KEY_MOD).sw(c, 0);
            c.g[A1][0] = 0;
            input.off(FEUI_KEY_CODE).sw(c, 0);
            c.g[A2][0] = sx(input.0.wrapping_add(FEUI_KEY_CODE));
            c.g[A3][0] = sx(input.0.wrapping_add(FEUI_KEY_MOD));
            call_ptr(c, 0x0026_6ba8, 0x0018_04ec)?;
            let found = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if found {
                c.cycles += 1;
                Addr(r32(c, S0).wrapping_add(FEUI_KEY_FOUND)).sw(c, 1);
            }
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FEUIInput_dtor(FEUIInput *self, int flags)` (0x1dd530): resets the
/// vtable (0x46dba8) at `+8`; with `flags & 1` deletes `self`
/// (`__builtin_delete`, 0x317e50).
pub fn fe_ui_input_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x47_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x2458));
        c.g[A1][0] &= 1;
        let keep = c.g[A1][0] == 0;
        Addr(r32(c, A0).wrapping_add(8)).sw(c, c.g[V0][0]);
        if !keep {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x001d_d554)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cMenuItem_activate(item, on)`: the row's activation (0x2ca388).
pub(crate) const CMENU_ITEM_ACTIVATE: u32 = 0x002c_a388;

/// The row array of one of the two lobby lists (rooms, occupants). Both
/// fillers have the same shape; only the callees, the return addresses and
/// the sizes differ.
pub(crate) struct ItemList {
    /// Offset of row 0 from the menu (rows are 0x24 apart).
    items: u32,
    /// Rows the menu holds.
    rows: u32,
    /// Count callee and its return address.
    get_count: (u32, u32),
    /// Name callee and its return address.
    get_name: (u32, u32),
    /// Third argument of the name callee (occupants pass 0).
    name_a2: Option<u64>,
    /// Return address after activating a filled row.
    act_ret: u32,
    /// Return address after activating row 0 for the empty placeholder.
    empty_ret: u32,
    /// Return address after deactivating the rows past the list.
    clear_ret: u32,
    /// Event heads of the fill loop and of the clear loop.
    head_fill: u32,
    head_clear: u32,
    /// Low half of the placeholder string address (`0x460000 + x`).
    empty_text_lo: u32,
    /// Cycles of one fill iteration and of the clear setup.
    fill_cycles: u64,
    mult_cycles: u64,
}

pub(crate) const ROOM_LIST: ItemList = ItemList {
    items: 0x168,
    rows: 12,
    get_count: (0x0025_f120, 0x001d_b558),
    get_name: (0x0025_f150, 0x001d_b574),
    name_a2: None,
    act_ret: 0x001d_b584,
    empty_ret: 0x001d_b5bc,
    clear_ret: 0x001d_b5ec,
    head_fill: 0x001d_b568,
    head_clear: 0x001d_b5e0,
    empty_text_lo: 0x7de8,
    fill_cycles: 12,
    mult_cycles: 4,
};

pub(crate) const OCCUPANT_LIST: ItemList = ItemList {
    items: 0x16c,
    rows: 24,
    get_count: (0x0025_f1b0, 0x001d_b650),
    get_name: (0x0025_f1e0, 0x001d_b670),
    name_a2: Some(0),
    act_ret: 0x001d_b680,
    empty_ret: 0x001d_b6b8,
    clear_ret: 0x001d_b6e4,
    head_fill: 0x001d_b660,
    head_clear: 0x001d_b6d8,
    empty_text_lo: 0x7e00,
    fill_cycles: 13,
    mult_cycles: 3,
};

/// Fills a lobby list from `self` (S4) and `src` (S3): one row per entry of
/// the count callee (name and activation), row 0 shows the placeholder when
/// the list is empty (item ID -1), and the rows past the list are deactivated.
pub(crate) fn fill_list(c: &mut Cpu, l: &ItemList) -> Flow {
    c.cycles += 15;
    let frame = Frame::open(c, 0x60);
    frame.save(c, 19, 0x20);
    frame.save(c, 20, 0x10);
    c.g[S3][0] = c.g[A1][0];
    c.g[S4][0] = c.g[A0][0];
    frame.save(c, 17, 0x40);
    frame.save(c, 18, 0x30);
    c.g[A0][0] = c.g[S3][0];
    frame.save(c, 16, 0x50);
    frame.save_ra(c, 0);
    c.g[S1][0] = 0;
    call_ptr(c, l.get_count.0, l.get_count.1)?;
    c.g[S2][0] = c.g[V0][0];
    if (c.g[S2][0] as i64) > 0 {
        c.cycles += 1;
        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(l.items));
        loop {
            c.cycles += l.fill_cycles;
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S3][0];
            if let Some(a2) = l.name_a2 {
                c.g[A2][0] = a2;
            }
            call_ptr(c, l.get_name.0, l.get_name.1)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            call_ptr(c, CMENU_ITEM_ACTIVATE, l.act_ret)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[S1][0]);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S2][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, l.head_fill);
            }
        }
    }
    // Empty list: row 0 shows the placeholder, ID -1, and counts as one row.
    c.cycles += 2;
    if c.g[S1][0] == 0 {
        c.cycles += 10;
        c.g[V0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(l.items));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(l.empty_text_lo));
        c.g[A1][0] = 1;
        c.sw((c.g[S4][0] as u32).wrapping_add(l.items + 0x14), c.g[V0][0]);
        c.g[S1][0] = 1;
        call_ptr(c, CMENU_ITEM_ACTIVATE, l.empty_ret)?;
        c.g[V0][0] = u64::MAX;
        c.sw((c.g[S4][0] as u32).wrapping_add(l.items + 0x8), c.g[V0][0]);
    }
    c.g[V0][0] = ((c.g[S1][0] as i64) < l.rows as i64) as u64;
    // Deactivate the rows past the list (v0 keeps 0x24 when there are none).
    c.cycles += 2;
    let none_left = c.g[V0][0] == 0;
    c.g[V0][0] = 0x24;
    if !none_left {
        c.cycles += l.mult_cycles;
        c.mult(0, 2, 17, 2); // v0 = s1 * v0: the row offset
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(l.items));
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
        loop {
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            call_ptr(c, CMENU_ITEM_ACTIVATE, l.clear_ret)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S1][0] as i64) < l.rows as i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, l.head_clear);
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
}

/// `LobbyMenu_FillRoomList` (0x1db528), the generated `LobbyMenu_updaterooms`:
/// the room rows (12) from `LobbyMan_getnumrooms` and `LobbyMan_GetRoomName`.
pub fn lobby_menu_fill_room_list(c: &mut Cpu) {
    guest(c, |c| fill_list(c, &ROOM_LIST));
}

/// `LobbyMenu_FillOccupantList` (0x1db620): the occupant rows (24) of the room
/// from `func_0025F1B0` (count) and `func_0025F1E0` (name, third argument 0).
pub fn lobby_menu_fill_occupant_list(c: &mut Cpu) {
    guest(c, |c| fill_list(c, &OCCUPANT_LIST));
}

/// `LobbyMenu_OnAbortPageAction` (0x1dbab8): forwards to `0x17e558` with the
/// menu's screen at +0x130; returns 0.
pub fn lobby_menu_on_abort_page_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x130));
        call_ptr(c, 0x0017_e558, 0x001d_bac8)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `LobbyMenu_OnChallengePageAction` (0x1dbad8): the message's field +8 picks
/// the screen action: 0 runs `0x17e490` (menu at +0x130), 1 runs `0x17e4f0`
/// (menu at +0x134), anything else does nothing. Returns 0.
pub fn lobby_menu_on_challenge_page_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = 1;
        if c.g[V1][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x130));
            call_ptr(c, 0x0017_e490, 0x001d_baf4)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 2;
            frame.restore_ra(c, 0);
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x134));
                call_ptr(c, 0x0017_e4f0, 0x001d_bb0c)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `LobbyMenu_OnChatPageAction` (0x1dbb20): `0x17e418(menu +0x130, msg +0x14)`.
pub fn lobby_menu_on_chat_page_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x130));
        c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
        call_ptr(c, 0x0017_e418, 0x001d_bb34)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `LobbyMenu_OnOccupantsPageAction` (0x1dbb48): the message's field +8 is
/// -2 (`0x17e3b8` with the menu at +0x134), -1 (nothing) or anything else
/// (`0x17e368` with the menu at +0x130). Returns 0.
pub fn lobby_menu_on_occupants_page_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = u64::MAX - 1;
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
        let not_minus_two = c.g[A1][0] != c.g[V0][0];
        c.g[V0][0] = u64::MAX;
        if !not_minus_two {
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x134));
            call_ptr(c, 0x0017_e3b8, 0x001d_bb68)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 2;
            frame.restore_ra(c, 0);
            if c.g[A1][0] != c.g[V0][0] {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x130));
                call_ptr(c, 0x0017_e368, 0x001d_bb80)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `LobbyMenu_OnSelectRoomPageAction` (0x1dbb90): a message whose field +8 is
/// negative does nothing; otherwise `0x17e2f8(menu +0x130, ...)`. Returns 0.
pub fn lobby_menu_on_select_room_page_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
        let negative = (c.g[A1][0] as i64) < 0;
        frame.restore_ra(c, 0);
        if !negative {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x130));
            call_ptr(c, 0x0017_e2f8, 0x001d_bbac)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 3;
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `LobbyMenu_OnSelectPersonaPageAction` (0x1dbbc0):
/// `0x17e290(menu +0x4e0, msg +8)`. Returns 0.
pub fn lobby_menu_on_select_persona_page_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4e0));
        c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
        call_ptr(c, 0x0017_e290, 0x001d_bbd4)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `LobbyMenu_OnCreatePersonaPageAction` (0x1dbbe8):
/// `0x17e1c0(menu +0x130, menu +0x180)`. Returns 0.
pub fn lobby_menu_on_create_persona_page_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        let menu = c.g[A0][0] as u32;
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(menu.wrapping_add(0x130));
        c.g[A1][0] = sx(menu.wrapping_add(0x180));
        call_ptr(c, 0x0017_e1c0, 0x001d_bc00)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `LobbyMenu_OnDeletePersonaPageAction` (0x1dbc10):
/// `0x17e228(menu +0x130, msg +8)`. Returns 0.
pub fn lobby_menu_on_delete_persona_page_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x130));
        c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
        call_ptr(c, 0x0017_e228, 0x001d_bc24)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// The create-account and login page actions: `callee(menu +0x130,
/// menu +0x180, menu +0x1ac)`. Returns 0.
pub(crate) fn persona_form_action(c: &mut Cpu, callee: u32, ret: u32) -> Flow {
    c.cycles += 11;
    let frame = Frame::open(c, 0x10);
    let menu = c.g[A0][0] as u32;
    c.g[A1][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A2][0] = sx(menu.wrapping_add(0x1ac));
    c.g[A0][0] = c.lw(menu.wrapping_add(0x130));
    c.g[A1][0] = sx(menu.wrapping_add(0x180));
    call_ptr(c, callee, ret)?;
    frame.restore_ra(c, 0);
    c.g[V0][0] = 0;
    frame.close(c);
    Ok(())
}

/// `LobbyMenu_OnCreateAccountPageAction` (0x1dbc38).
pub fn lobby_menu_on_create_account_page_action(c: &mut Cpu) {
    guest(c, |c| persona_form_action(c, 0x0017_e030, 0x001d_bc54));
}

/// `LobbyMenu_OnLoginPageAction` (0x1dbc68).
pub fn lobby_menu_on_login_page_action(c: &mut Cpu) {
    guest(c, |c| persona_form_action(c, 0x0017_e118, 0x001d_bc84));
}

/// `sx((base as u32) + off)`: the 32-bit address arithmetic of the generated code.
pub(crate) fn menu_sx_add(base: u64, off: u32) -> u64 {
    sx((base as u32).wrapping_add(off))
}

/// `MenuSysItem_dtor(a0, 2)` (0x2ca280, through the function table), returning to `ret`.
pub(crate) fn menu_item_dtor(c: &mut Cpu, a0: u64, ret: u32) -> Flow {
    c.g[A0][0] = a0;
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002c_a280, ret)
}

/// `MenuSysPage_dtor(a0, a1)` (0x2caa80, through the function table), returning to `ret`.
pub(crate) fn menu_page_dtor(c: &mut Cpu, a0: u64, a1: u64, ret: u32) -> Flow {
    c.g[A0][0] = a0;
    c.g[A1][0] = a1;
    call_ptr(c, 0x002c_aa80, ret)
}

/// The `'l: loop` that destroys the 0x24-byte entries of a page's list: from
/// `S0` down to `S2`, each entry's destructor found through its vtable
/// (`+0x10` holds the vtable, `+0x8` a halfword offset for `a0`, `+0xc` the
/// function). `head` is the loop's first instruction (the event check's pc),
/// `ret` the return address of the destructor call.
pub(crate) fn menu_list_walk(c: &mut Cpu, head: u32, ret: u32) -> Flow {
    loop {
        c.cycles += 8;
        c.g[A1][0] = 0;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x8).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(ret);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, ret)?;
        let again = c.g[S2][0] != c.g[S0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x24));
        if !again {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// Destroys the items at `base + off` (each with a destructor call returning to
/// `ret`), then the page at `base` with `a1`: the common tail of the
/// `LobbyMenu_Destroy*Page` functions.
pub(crate) fn lobby_menu_destroy_items(
    c: &mut Cpu,
    base: u64,
    items: &[(u32, u32)],
    page: (u64, u32),
) -> Flow {
    for &(off, ret) in items {
        menu_item_dtor(c, menu_sx_add(base, off), ret)?;
    }
    menu_page_dtor(c, base, page.0, page.1)
}

/// `LobbyMenu_DestroyLoginPage(this, a1)` (0x1dcda0): six list items, then the page.
pub fn lobby_menu_destroy_login_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let s0 = c.g[S0][0];
        let s1 = c.g[S1][0];
        lobby_menu_destroy_items(
            c,
            s0,
            &[
                (0x1e4, 0x001d_cdc4),
                (0x1c0, 0x001d_cdd0),
                (0x194, 0x001d_cddc),
                (0x168, 0x001d_cde8),
                (0x150, 0x001d_cdf4),
                (0x134, 0x001d_ce00),
            ],
            (s1, 0x001d_ce0c),
        )?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_DestroyCreateAccountPage(this, a1)` (0x1dce20): six list items, then the page.
pub fn lobby_menu_destroy_create_account_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let s0 = c.g[S0][0];
        let s1 = c.g[S1][0];
        lobby_menu_destroy_items(
            c,
            s0,
            &[
                (0x1e4, 0x001d_ce44),
                (0x1c0, 0x001d_ce50),
                (0x194, 0x001d_ce5c),
                (0x168, 0x001d_ce68),
                (0x150, 0x001d_ce74),
                (0x134, 0x001d_ce80),
            ],
            (s1, 0x001d_ce8c),
        )?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_DestroyLoginMethodPage(this, a1)` (0x1dcea0): the largest of the
/// page destructors, a straight run of item and page destructions.
pub fn lobby_menu_destroy_login_method_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 74;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        let s0 = c.g[S0][0];
        menu_item_dtor(c, menu_sx_add(s0, 0x578), 0x001d_cecc)?;
        c.g[S1][0] = menu_sx_add(s0, 0x32c);
        menu_item_dtor(c, menu_sx_add(s0, 0x55c), 0x001d_cedc)?;
        c.g[S2][0] = menu_sx_add(s0, 0x130);
        for (off, ret) in [
            (0x544, 0x001d_ceec),
            (0x528, 0x001d_cef8),
            (0x510, 0x001d_cf04),
            (0x4ec, 0x001d_cf10),
            (0x4c0, 0x001d_cf1c),
            (0x494, 0x001d_cf28),
            (0x47c, 0x001d_cf34),
            (0x460, 0x001d_cf40),
        ] {
            menu_item_dtor(c, menu_sx_add(s0, off), ret)?;
        }
        let s1 = c.g[S1][0];
        menu_page_dtor(c, s1, 0x2, 0x001d_cf4c)?;
        for (off, ret) in [
            (0x314, 0x001d_cf58),
            (0x2f0, 0x001d_cf64),
            (0x2c4, 0x001d_cf70),
            (0x298, 0x001d_cf7c),
            (0x280, 0x001d_cf88),
            (0x264, 0x001d_cf94),
        ] {
            menu_item_dtor(c, menu_sx_add(s0, off), ret)?;
        }
        let s2 = c.g[S2][0];
        menu_page_dtor(c, s2, 0x2, 0x001d_cfa0)?;
        let s3 = c.g[S3][0];
        menu_page_dtor(c, s0, s3, 0x001d_cfac)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_DestroyDeletePersonaPage(this, a1)` (0x1dcfc8): walks the entries
/// from `this + 0x1f8` down to `this + 0x168` (when they exist), then two items and the page.
pub fn lobby_menu_destroy_delete_persona_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        let s1 = c.g[S1][0];
        let v0 = menu_sx_add(s1, 0x168);
        c.g[V0][0] = v0;
        frame.save_ra(c, 0);
        if v0 != 0 {
            c.cycles += 3;
            c.g[S0][0] = menu_sx_add(s1, 0x1f8);
            c.g[S2][0] = v0;
            if v0 != c.g[S0][0] {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x24));
                menu_list_walk(c, 0x001d_d000, 0x001d_d018)?;
            }
        }
        c.cycles += 16;
        menu_item_dtor(c, menu_sx_add(s1, 0x150), 0x001d_d02c)?;
        menu_item_dtor(c, menu_sx_add(s1, 0x134), 0x001d_d038)?;
        let s3 = c.g[S3][0];
        menu_page_dtor(c, s1, s3, 0x001d_d044)?;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S3, 0x10);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_DestroyCreatePersonaPage(this, a1)` (0x1dd060): four items, then the page.
pub fn lobby_menu_destroy_create_persona_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let s0 = c.g[S0][0];
        let s1 = c.g[S1][0];
        lobby_menu_destroy_items(
            c,
            s0,
            &[
                (0x194, 0x001d_d084),
                (0x168, 0x001d_d090),
                (0x150, 0x001d_d09c),
                (0x134, 0x001d_d0a8),
            ],
            (s1, 0x001d_d0b4),
        )?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_DestroySelectPersonaPage(this, a1)` (0x1dd0c8): the largest page
/// destructor. Three entry lists (`+0x518..0x5a8`, `+0x450..0x4e0`, and the
/// items from `+0x264` to `+0x5c4`), then the page with the stored `a1`.
pub fn lobby_menu_destroy_select_persona_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S1, 0x90);
        c.g[S1][0] = c.g[A0][0];
        let a1 = c.g[A1][0];
        let a1_slot = frame.at(c, 0);
        Addr(a1_slot).sw(c, a1);
        frame.save(c, S0, 0xa0);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        let s1 = c.g[S1][0];
        c.g[A1][0] = 0x2;
        c.g[S0][0] = menu_sx_add(s1, 0x5a8);
        menu_item_dtor(c, menu_sx_add(s1, 0x5c4), 0x001d_d10c)?;
        let s0 = c.g[S0][0];
        menu_item_dtor(c, s0, 0x001d_d118)?;

        // Block A: the list at +0x518 (its end is +0x5a8).
        let v0 = menu_sx_add(s1, 0x518);
        c.g[V0][0] = v0;
        c.g[A0][0] = menu_sx_add(s1, 0x500);
        if v0 != 0 {
            c.cycles += 2;
            let same = v0 == c.g[S0][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x24));
            if !same {
                c.cycles += 1;
                c.g[S2][0] = v0;
                menu_list_walk(c, 0x001d_d130, 0x001d_d148)?;
                c.cycles += 1;
                c.g[A0][0] = menu_sx_add(s1, 0x500);
            }
        }

        // Block B: the list at +0x450 (its end is +0x4e0).
        c.cycles += 13;
        menu_item_dtor(c, menu_sx_add(s1, 0x500), 0x001d_d15c)?;
        c.g[S3][0] = menu_sx_add(s1, 0x2e8);
        menu_item_dtor(c, menu_sx_add(s1, 0x4e4), 0x001d_d16c)?;
        c.g[FP][0] = menu_sx_add(s1, 0x130);
        c.g[V0][0] = menu_sx_add(s1, 0x450);
        c.g[S4][0] = menu_sx_add(s1, 0x2c4);
        c.g[S5][0] = menu_sx_add(s1, 0x298);
        c.g[S6][0] = menu_sx_add(s1, 0x280);
        let empty = c.g[V0][0] == 0;
        c.g[S7][0] = menu_sx_add(s1, 0x264);
        if !empty {
            c.cycles += 3;
            c.g[S0][0] = menu_sx_add(s1, 0x4e0);
            let same = c.g[V0][0] == c.g[S0][0];
            c.g[S2][0] = c.g[V0][0];
            if !same {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x24));
                menu_list_walk(c, 0x001d_d198, 0x001d_d1b0)?;
            }
        }

        // Block C: the rest, then the page with the stored a1.
        c.cycles += 39;
        let s3 = c.g[S3][0];
        menu_item_dtor(c, menu_sx_add(s3, 0x150), 0x001d_d1c4)?;
        menu_item_dtor(c, menu_sx_add(s3, 0x134), 0x001d_d1d0)?;
        menu_page_dtor(c, s3, 0x2, 0x001d_d1dc)?;
        let s4 = c.g[S4][0];
        menu_item_dtor(c, s4, 0x001d_d1e8)?;
        let s5 = c.g[S5][0];
        menu_item_dtor(c, s5, 0x001d_d1f4)?;
        let s6 = c.g[S6][0];
        menu_item_dtor(c, s6, 0x001d_d200)?;
        let s7 = c.g[S7][0];
        menu_item_dtor(c, s7, 0x001d_d20c)?;
        let fp = c.g[FP][0];
        menu_page_dtor(c, fp, 0x2, 0x001d_d218)?;
        let stored_a1 = Addr(frame.at(c, 0)).lw(c);
        menu_page_dtor(c, s1, stored_a1, 0x001d_d224)?;

        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_DestroySelectRoomPage(this, a1)` (0x1dd258): one list from
/// `+0x318` down to `+0x168` (when present), two items and the page.
pub fn lobby_menu_destroy_select_room_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        let s1 = c.g[S1][0];
        let v0 = menu_sx_add(s1, 0x168);
        c.g[V0][0] = v0;
        frame.save_ra(c, 0);
        if v0 != 0 {
            c.cycles += 3;
            c.g[S0][0] = menu_sx_add(s1, 0x318);
            let same = v0 == c.g[S0][0];
            c.g[S2][0] = v0;
            if !same {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x24));
                menu_list_walk(c, 0x001d_d290, 0x001d_d2a8)?;
            }
        }
        c.cycles += 16;
        menu_item_dtor(c, menu_sx_add(s1, 0x150), 0x001d_d2bc)?;
        menu_item_dtor(c, menu_sx_add(s1, 0x134), 0x001d_d2c8)?;
        let s3 = c.g[S3][0];
        menu_page_dtor(c, s1, s3, 0x001d_d2d4)?;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S3, 0x10);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_DestroyRoomOccupantsPage(this, a1)` (0x1dd2f0): the item at
/// `+0x4cc`, a list from `+0x4cc` down to `+0x16c` (when present), then two
/// items (`+0x154` before the list's end, `+0x138` after) and the page.
pub fn lobby_menu_destroy_room_occupants_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = menu_sx_add(c.g[S1][0], 0x4cc);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        let s0 = c.g[S0][0];
        menu_item_dtor(c, s0, 0x001d_d320)?;
        let s1 = c.g[S1][0];
        let v0 = menu_sx_add(s1, 0x16c);
        c.g[V0][0] = v0;
        if v0 != 0 {
            c.cycles += 2;
            let same = v0 == c.g[S0][0];
            c.g[S2][0] = v0;
            if !same {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x24));
                menu_list_walk(c, 0x001d_d338, 0x001d_d350)?;
            }
            c.cycles += 1;
        }
        c.cycles += 15;
        menu_item_dtor(c, menu_sx_add(s1, 0x154), 0x001d_d364)?;
        menu_item_dtor(c, menu_sx_add(s1, 0x138), 0x001d_d370)?;
        let s3 = c.g[S3][0];
        menu_page_dtor(c, s1, s3, 0x001d_d37c)?;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S3, 0x10);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_DestroyChatPage(this, a1)` (0x1dd398): a list from `+0x38c` down
/// to `+0x194` (when present), three items and the page.
pub fn lobby_menu_destroy_chat_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        let s1 = c.g[S1][0];
        let v0 = menu_sx_add(s1, 0x194);
        c.g[V0][0] = v0;
        frame.save_ra(c, 0);
        if v0 != 0 {
            c.cycles += 3;
            c.g[S0][0] = menu_sx_add(s1, 0x38c);
            let same = v0 == c.g[S0][0];
            c.g[S2][0] = v0;
            if !same {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x24));
                menu_list_walk(c, 0x001d_d3d0, 0x001d_d3e8)?;
            }
        }
        c.cycles += 19;
        menu_item_dtor(c, menu_sx_add(s1, 0x168), 0x001d_d3fc)?;
        menu_item_dtor(c, menu_sx_add(s1, 0x150), 0x001d_d408)?;
        menu_item_dtor(c, menu_sx_add(s1, 0x134), 0x001d_d414)?;
        let s3 = c.g[S3][0];
        menu_page_dtor(c, s1, s3, 0x001d_d420)?;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S3, 0x10);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_InitAbortPage(page, arg)` (0x1db2b0): calls the page's base
/// constructor (0x2caa58), stores `0x46d378` at `page+0x12c` and the word at
/// `arg` at `page+0x130`, builds the abort item at `page+0x134` (0x2ccf98, with
/// the string at `0x467de0`) and adds it to the page's menu (0x2cbcc8, index
/// -1). Returns the page.
pub fn lobby_menu_init_abort_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        call_ptr(c, 0x002c_aa58, 0x001d_b2d0)?;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x134));
        c.g[V0][0] = 0x470000;
        c.g[A2][0] = 0x460000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2c88));
        c.g[A3][0] = 0;
        let title = c.g[V0][0];
        Addr(c.g[S0][0] as u32).off(0x12c).sw(c, title);
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x7de0));
        c.g[A1][0] = 0;
        c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
        let word = c.g[V0][0];
        Addr(c.g[S0][0] as u32).off(0x130).sw(c, word);
        call_ptr(c, 0x002c_cf98, 0x001d_b300)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_b310)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_FillDeletePersonaPage(page, list)` (0x1db330): one 0x24-byte
/// item per persona of the list (count at `list+0x6c`, persona words at
/// `list+0x70`), from `page+0x168`. Each is activated (`MenuSysItem_SetVisible`,
/// flag 1), gets its persona word at `+0x17c` and the next index at `+0x170`;
/// the items from the count up to the fourth are deactivated (flag 0).
pub fn lobby_menu_fill_delete_persona_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x20);
        frame.save(c, S0, 0x60);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S4][0] as u32).off(0x6c).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = c.g[A0][0];
        if !empty {
            c.cycles += 2;
            c.g[S5][0] = 0x24;
            c.mult(0, V0, S2, S5);
            loop {
                c.cycles += 19;
                c.mult(0, S0, S2, S5);
                c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = 0x1;
                c.g[S2][0] = c.g[S1][0];
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x168));
                c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x70).lw(c);
                let word = c.g[V1][0];
                Addr(c.g[A2][0] as u32).off(0x17c).sw(c, word);
                call_ptr(c, 0x002c_a388, 0x001d_b3a0)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                let next = c.g[S1][0];
                Addr(c.g[S0][0] as u32).off(0x170).sw(c, next);
                c.g[V0][0] = Addr(c.g[S4][0] as u32).off(0x6c).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                let more = (c.g[S2][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.mult(0, V0, S2, S5);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001d_b370);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S2][0] as i64) < 4) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = 0x24;
        if more {
            c.cycles += 4;
            c.mult(0, V0, S2, V0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x168));
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S3][0] as u32));
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x002c_a388, 0x001d_b3e4)?;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S2][0] as i64) < 4) as u64;
                let again = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                if !again {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001d_b3d8);
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_FillPersonaPages(page, list)` (0x1db418): the select-persona
/// items at `page+0x518` (0x24 each, one per persona of the list, count at
/// `list+0x6c`, persona words at `list+0x70`) get their persona word at `+0x14`
/// and index at `+0x8`, and are activated; the items up to the fourth are
/// deactivated. Then the delete-persona page at `page+0x2e8` is filled
/// (`LobbyMenu_FillDeletePersonaPage`), and the two menu items at `page+0x5c4`
/// and `page+0x5a8` are shown when the count is 2 or more (the first) and 4 or
/// more (the second; their visibility flags are `count < 2 ^ 1` and `count < 4`).
pub fn lobby_menu_fill_persona_pages(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S1, 0x60);
        frame.save(c, S3, 0x40);
        c.g[S1][0] = 0;
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x70);
        frame.save(c, S2, 0x50);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S3][0] as u32).off(0x6c).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S4][0] = c.g[A0][0];
        if !empty {
            c.cycles += 2;
            c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x518));
            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x70));
            loop {
                c.cycles += 12;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                let word = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x1;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                Addr(c.g[S0][0] as u32).off(0x14).sw(c, word);
                call_ptr(c, 0x002c_a388, 0x001d_b470)?;
                let index = c.g[S1][0];
                Addr(c.g[S0][0] as u32).off(0x8).sw(c, index);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = Addr(c.g[S3][0] as u32).off(0x6c).lw(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let again = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                if !again {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001d_b458);
                }
            }
        }
        c.cycles += 5;
        c.g[V0][0] = ((c.g[S1][0] as i64) < 4) as u64;
        c.g[S2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x2e8));
        c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5c4));
        let more = c.g[V0][0] != 0;
        c.g[S6][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5a8));
        if more {
            c.cycles += 5;
            c.g[V0][0] = 0x24;
            c.mult(0, V0, S1, V0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x518));
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x002c_a388, 0x001d_b4bc)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 4) as u64;
                let again = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                if !again {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001d_b4b0);
                }
            }
        }
        c.cycles += 22;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S2][0];
        Callee {
            addr: 0x001d_b330,
            f: lobby_menu_fill_delete_persona_page,
        }
        .call(c, 0x001d_b4d8)?;
        c.g[A1][0] = Addr(c.g[S3][0] as u32).off(0x6c).lw(c);
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = ((c.g[A1][0] as i64) < 2) as u64;
        c.g[A1][0] ^= 0x1;
        call_ptr(c, 0x002c_a388, 0x001d_b4ec)?;
        c.g[A1][0] = Addr(c.g[S3][0] as u32).off(0x6c).lw(c);
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = ((c.g[A1][0] as i64) < 4) as u64;
        call_ptr(c, 0x002c_a388, 0x001d_b4fc)?;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_InitChallengePage(page, accept_owner, decline_owner)` (0x1db148):
/// the challenge page's base constructor (0x2caa58), its screen pointer
/// `0x46d3d0`-tagged title, the two owners stored at `+0x130` and `+0x134` (the
/// words they point to), then the six menu items (two label items, a spacer,
/// two command items for accept and decline) added to the page in order.
/// Returns the page.
pub fn lobby_menu_init_challenge_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 90;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x138));
        frame.save(c, S6, 0x20);
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x154));
        frame.save(c, S7, 0x10);
        c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x170));
        frame.save_ra(c, 0);
        c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18c));
        call_ptr(c, 0x002c_aa58, 0x001d_b190)?;
        c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c8));
        c.g[V0][0] = 0x470000;
        c.g[A1][0] = 0x460000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2c30));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7da8));
        let screen = c.g[V0][0];
        Addr(c.g[S0][0] as u32).off(0x12c).sw(c, screen);
        c.g[A0][0] = c.g[S4][0];
        c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1a4));
        let accept = c.g[V1][0];
        Addr(c.g[S0][0] as u32).off(0x130).sw(c, accept);
        c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
        let decline = c.g[V0][0];
        Addr(c.g[S0][0] as u32).off(0x134).sw(c, decline);
        call_ptr(c, 0x002c_d2a0, 0x001d_b1c4)?;
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7dc0));
        call_ptr(c, 0x002c_d2a0, 0x001d_b1d4)?;
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7dc8));
        call_ptr(c, 0x002c_d2a0, 0x001d_b1e4)?;
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x002c_d350, 0x001d_b1f0)?;
        c.g[A2][0] = 0x460000;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x7dd0));
        c.g[A1][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x002c_cf98, 0x001d_b208)?;
        c.g[A2][0] = 0x460000;
        c.g[A3][0] = 0;
        c.g[A0][0] = c.g[S7][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x7dd8));
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x002c_cf98, 0x001d_b220)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_b230)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_b240)?;
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_b250)?;
        c.g[A1][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_b260)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_b270)?;
        c.g[A1][0] = c.g[S7][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x001d_b280)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x70);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMenu_DestroyChallengePage(LobbyMenu* page, int flags)`
/// (0x1dd440): destroy the six menu items of the challenge-response page
/// (`cMenuItem` at `page + 0x138 .. 0x1c8`, flags 2), then the page menu
/// itself at `page` with the caller's flags.
pub fn lobby_menu_destroy_challenge_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        const ITEMS: [(u32, u32); 6] = [
            (0x1c8, 0x001d_d464),
            (0x1a4, 0x001d_d470),
            (0x18c, 0x001d_d47c),
            (0x170, 0x001d_d488),
            (0x154, 0x001d_d494),
            (0x138, 0x001d_d4a0),
        ];
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        for &(off, back) in ITEMS.iter() {
            c.g[A0][0] = sx(oc508_at(c, S0, off));
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x002c_a280, back)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x001d_d4ac)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMenu_DestroyAbortPage(LobbyMenu* page, int flags)` (0x1dd4c0):
/// destroy the abort page's menu item (`page + 0x134`, flags 2), then the
/// page menu at `page` with the caller's flags.
pub fn lobby_menu_destroy_abort_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(oc508_at(c, S0, 0x134));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002c_a280, 0x001d_d4e4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x001d_d4f0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FEUIInput_IsNext(input)` (0x17fec0): asks the pad object at `*(input + 0x18)`
/// whether button 0x7a ("next") is pressed (callee 0x320c48) and returns its answer.
pub fn feui_input_is_next(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x7a;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0032_0c48, 0x0017_fed4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Offset of the input binding in a `FEUIInput` (`+0x18`).
pub(crate) const FEUI_INPUT_BINDING: u32 = 0x18;

/// `bool FEUIInput_IsDown(FEUIInput* this)` (0x180060): evaluates the
/// binding's input 0x73 (`InputBinding_EvalBool`, 0x320c48).
///
/// The generated body also has a resume entry at 0x180070 (after the call's
/// delay slot); resumes go to the generated `__entries` twin.
pub fn feui_input_is_down(c: &mut Cpu) {
    if c.pc != 0x0018_0060 {
        return ee::s02::f_00180060__entries(c);
    }
    feui_input_eval(c, 0x73, 0x0018_0074);
}

/// `bool FEUIInput_IsUp(FEUIInput* this)` (0x180040): evaluates the
/// binding's input 0x72 (`InputBinding_EvalBool`, 0x320c48).
pub fn feui_input_is_up(c: &mut Cpu) {
    feui_input_eval(c, 0x72, 0x0018_0054);
}

/// The shared body of `FEUIInput_IsUp/IsDown`: `InputBinding_EvalBool(
/// this->binding, input)`; `ret` is the call's return address.
pub(crate) fn feui_input_eval(c: &mut Cpu, input: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = input as u64;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(FEUI_INPUT_BINDING));
        call_ptr(c, 0x0032_0c48, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Shared body of the `FEUIInput_IsCam*` wrappers:
/// `InputBinding_EvalBool(this->binding (+0x18), action)`.
pub(crate) fn feui_input_eval_action(c: &mut Cpu, action: u64, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = action;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x18)).lw(c);
        call_ptr(c, 0x0032_0c48, ret_addr)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool FEUIInput::IsCamRotate(FEUIInput* this)` (0x180280): action 0x91.
pub fn feui_input_is_cam_rotate(c: &mut Cpu) {
    feui_input_eval_action(c, 0x91, 0x0018_0294);
}

/// `bool FEUIInput::IsCamPan(FEUIInput* this)` (0x180260): action 0x92.
pub fn feui_input_is_cam_pan(c: &mut Cpu) {
    feui_input_eval_action(c, 0x92, 0x0018_0274);
}

/// `bool FEUIInput::IsCamZoom(FEUIInput* this)` (0x180240): action 0x90.
pub fn feui_input_is_cam_zoom(c: &mut Cpu) {
    feui_input_eval_action(c, 0x90, 0x0018_0254);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    use crate::difftest::SP;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::ai::rider::tests::binding;
    pub(crate) use crate::game::ai::rider::tests::clear_2;

    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::be::beintstat::tests::allocator;

    pub(crate) use crate::game::bx::bxstringctor::tests::W;
    pub(crate) use crate::game::camera::camera::tests::alloc_base;

    pub(crate) use crate::game::camera::camera::tests::W_2 as W_5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::gpf;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::base_8 as base;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::put_vtable;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::zero;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all_2 as check_all;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;

    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_6 as pool_allocator_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_2 as slot;

    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::VT;

    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_10;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all_2;
    pub(crate) use crate::game::fe::fepopup::tests::AUX as AUX_2;

    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9;

    pub(crate) use crate::game::fe::festaterewards::tests::strict as strict_2;
    pub(crate) use crate::game::fe::festaterewards::tests::w as w_7;

    pub(crate) use crate::game::main::debugmenu::tests::w as w_8;
    pub(crate) use crate::game::render::particle::tests::base as base_2;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::belib_2 as belib;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_setup;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_3;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_8;

    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::dummy_world;

    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn keyboard_screen_toggle_caps_matches() {
        check(
            ee::s02::f_001807c8__generated,
            keyboard_screen_toggle_caps,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0018_07c8, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let flags = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => r.next_u32(),
                };
                c.bus.write32(DATA + 0xe4, flags);
            },
        );
    }

    #[test]
    fn lobby_menu_lobby_event_hook_matches() {
        check(
            ee::s02::f_0017d260__generated,
            lobby_menu_lobby_event_hook,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x17d2_60, 0);
                let ev = DATA + 0x600;
                fill(c, &mut r, ev, 0x20);
                c.bus.write32(ev, 0);
                c.g[A1][0] = sx(ev);
                let menu = if seed % 2 == 0 { 0 } else { DATA + 0x400 };
                zero(c, DATA + 0x400, 0x40);
                c.bus.write32(GAME_GP + 0x1da0, menu);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), 0);
            },
        );
    }

    /// A NUL-terminated ASCII string at `addr`.
    pub(crate) fn put_str(c: &mut Cpu, addr: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    #[test]
    fn keyboard_screen_set_text_matches() {
        check(
            ssx3_ee::s02::f_001805d8__generated,
            keyboard_screen_set_text,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x10, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x5000);
                put_str(
                    c,
                    DATA + 0x5000,
                    ["A", "hello", "a longer text value"][seed as usize % 3],
                );
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x17fa60 DebugMenuModule_Enter.

    #[test]
    fn debug_menu_module_enter_matches() {
        check(
            ee::s02::f_0017fa60__generated,
            debug_menu_module_enter,
            8,
            &|c, seed| {
                let _r = machine(c, seed, 0x17fa_60, 1);
                c.bus.write32(GAME_GP.wrapping_sub(0x854), 0);
            },
        );
    }

    // ---------------------------------------------------------- lobby menu

    #[test]
    fn lobby_menu_leave_occupants_matches() {
        check(
            ee::s02::f_0017e368__generated,
            lobby_menu_leave_occupants,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x17e3);
                let menu = DATA;
                let n = if seed % 5 == 0 { 0 } else { 1 + seed % 3 };
                c.bus.write32(menu + 8, n);
                let vt = put_vtable(c, &mut r, DATA + 0x2000, &[0x30, 0x38]);
                for i in 0..3u32 {
                    let item = DATA + 0x1000 + 0x200 * i;
                    c.bus
                        .write32(menu + 0xc + 4 * i, if i < n { item } else { 0 });
                    c.bus.write32(item + 0x12c, vt);
                }
                c.g[A0][0] = sx(menu);
            },
        );
    }

    #[test]
    fn debug_menu_module_v06_matches() {
        check(
            ee::s02::f_0017fb98__generated,
            debug_menu_module_v06,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x17fb_98, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let world = DATA + 0x2000;
                let obj = DATA + 0x800;
                let vt = vtable(c, &mut r, &[0x10]);
                clear(c, this, 0x100);
                clear(c, obj, 0x100);
                c.bus.write32(this + 0x14, obj);
                c.bus.write32(obj, vt);
                dummy_world(c, world);
                if seed % 2 == 0 {
                    c.bus.write32(world + 4, 0);
                } else {
                    c.bus.write32(world + 4, 1);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn lobby_menu_show_occupant_detail_matches() {
        check(
            ee::s02::f_0017e3b8__generated,
            lobby_menu_show_occupant_detail,
            SEEDS_2,
            &|c, seed| {
                let _ = common(c, seed, 0x17e3b8, 2);
                let this = DATA + 0x100;
                clear(c, this, 0x1600 / 4);
                c.bus.write32(this + 8, (seed % 3) as u32);
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }

    #[test]
    fn keyboard_screen_cursor_down_matches() {
        check(
            ee::s02::f_00180768__generated,
            keyboard_screen_cursor_down,
            SEEDS_2,
            &|c, seed| {
                let _ = base_2(c, seed, 0x180768);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                c.bus.write32(this + 0x170, (seed % 48) as u32);
                c.bus.write32(this + 0xd8, ((seed / 48) % 2) as u32);
                c.bus.write32(this + 0xe0, ((seed / 96) % 2) as u32);
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ------------------------------------------------------------------
    // Front-end input.

    #[test]
    fn feui_input_set_port_matches() {
        check(
            ee::s02::f_00180478__generated,
            feui_input_set_port,
            200,
            &|c, seed| {
                let mut r = setup(c, seed, 9);
                c.g[A0][0] = AUX as u64;
                let port = seed % 5;
                c.g[A1][0] = 0xabcd_ff00u64 | port as u64;
                put(c, W + 0xb0 + 4 * port, r.next_u32());
                gpf(c, -0x848, W);
            },
        );
    }

    #[test]
    fn feui_input_has_binding_matches() {
        check(
            ee::s02::f_00180498__generated,
            feui_input_has_binding,
            200,
            &|c, seed| {
                let mut r = setup(c, seed, 10);
                c.g[A0][0] = AUX as u64;
                match seed % 4 {
                    0 => put(c, AUX + 0x18, 0),
                    n => {
                        put(c, AUX + 0x18, AUX + 0x40);
                        put(c, AUX + 0x40, AUX + 0x80);
                        put(c, AUX + 0x80, if n == 1 { 0 } else { r.next_u32() });
                    }
                }
            },
        );
    }

    /// The object most fixtures live at.
    pub(crate) const OBJ: u32 = DATA + 0x1000;

    pub(crate) const OBJ2: u32 = DATA + 0x2000;

    pub(crate) const OBJ3: u32 = DATA + 0x3000;

    pub(crate) fn w(c: &mut Cpu, at: u32, v: u32) {
        c.bus.write32(at, v);
    }

    /// The engine's size-class pool allocator: blocks up to 0x400 bytes come
    /// from per-size free lists hanging off `gp + 0x2a60..0x2a84` when the
    /// pools-ready flag at `gp + 0xdd8` is set.
    pub(crate) fn pool_allocator(c: &mut Cpu) {
        w(c, GAME_GP + 0xdd8, 0); // (re-written below as 1)
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),   // 8-byte blocks
            (0x2a60, 0x2000),  // 16
            (0x2a6c, 0x32000), // 32
            (0x2a7c, 0x25800), // 64
            (0x2a84, 0x25800), // 128
            (0x2a78, 0x20000), // 256
            (0x2a68, 0x20000), // 512
        ];
        let nodes = DATA + 0x28_000;
        for (i, (slot_off, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x100 * i as u32;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                w(c, node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base);
            w(c, GAME_GP + slot_off, pool);
        }
    }

    // ------------------------------------------------------------- keyboard

    #[test]
    fn keyboard_screen_cursor_left_matches() {
        check(
            ssx3_ee::s02::f_00180618__generated,
            keyboard_screen_cursor_left,
            120,
            &|c, seed| {
                fakes::machine(c, seed, 0x180618, 0);
                let cursor = [0u32, 10, 20, 30, 39, 0x25, 0x27, 5][(seed % 8) as usize];
                w(c, OBJ + 0x170, cursor);
                w(c, OBJ + 0xe0, if seed % 3 == 0 { 0 } else { seed });
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn keyboard_screen_cursor_right_matches() {
        check(
            ssx3_ee::s02::f_00180678__generated,
            keyboard_screen_cursor_right,
            120,
            &|c, seed| {
                fakes::machine(c, seed, 0x180678, 0);
                let cursor = [9u32, 19, 29, 39, 0x25, 0x27, 5, 0x1d][(seed % 8) as usize];
                w(c, OBJ + 0x170, cursor);
                w(c, OBJ + 0xe0, if seed % 3 == 0 { 0 } else { seed });
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    // ------------------------------------------------------- menus / audio

    #[test]
    fn lobby_menu_init_message_page_matches() {
        check(
            ssx3_ee::s02::f_0017d200__generated,
            lobby_menu_init_message_page,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x17d200, 0);
                c.bus.write32(OBJ, seed);
                w(c, OBJ + 0x130, seed);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_accept_challenge_matches() {
        check(
            ssx3_ee::s02::f_0017e490__generated,
            lobby_menu_accept_challenge,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x17e490, 0);
                pool_allocator(c);
                let node = OBJ2;
                w(c, node, 0);
                w(c, node + 0x4, OBJ3);
                w(c, OBJ + 0x8, if seed % 2 == 0 { 0 } else { node });
                w(c, OBJ + 0x1c0c, seed);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_update_matches() {
        check(
            ee::s02::f_0017d688__generated,
            lobby_menu_update,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x17d688, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                w(c, OBJ + 8, (seed % 3) as u32);
                w(c, OBJ + 0xc, if seed % 2 == 0 { 0 } else { 1 });
                w(c, OBJ + 4, 0x19);
            },
        );
    }

    pub(crate) fn lobby_persona_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 20, &|c, seed| {
            fakes::machine(c, seed, salt, 0);
            globals(c);
            // Draining the list and `LobbyMan_SelectPersona` build strings and
            // tag fields, so the engine allocator has to be usable.
            pool_allocator_2(c);
            heap_init(c);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { (seed & 7) as u64 };
            if seed % 2 == 0 {
                w(c, OBJ + 8, 0);
            } else {
                w(c, OBJ + 8, 1);
                w(c, OBJ + 0xc, OBJ2);
                w(c, OBJ2 + 0x12c, VT);
                slot(c, VT, 0x38, 0, fakes::FAKE);
            }
        });
    }

    #[test]
    fn lobby_menu_create_persona_matches() {
        lobby_persona_test(
            ee::s02::f_0017e1c0__generated,
            lobby_menu_create_persona,
            0x17e1c0,
        );
    }

    #[test]
    fn lobby_menu_delete_persona_matches() {
        lobby_persona_test(
            ee::s02::f_0017e228__generated,
            lobby_menu_delete_persona,
            0x17e228,
        );
    }

    #[test]
    fn lobby_menu_select_persona_matches() {
        lobby_persona_test(
            ee::s02::f_0017e290__generated,
            lobby_menu_select_persona,
            0x17e290,
        );
    }

    #[test]
    fn lobby_menu_decline_challenge_matches() {
        check(
            ee::s02::f_0017e4f0__generated,
            lobby_menu_decline_challenge,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x17e4f0, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                if seed % 2 == 0 {
                    w(c, OBJ + 8, 0);
                } else {
                    w(c, OBJ + 8, 1);
                    w(c, OBJ + 0xc, OBJ2);
                    w(c, OBJ2 + 0x12c, VT);
                    slot(c, VT, 0x38, 0, fakes::FAKE);
                }
                w(c, OBJ + 0x1044, 0);
                w(c, OBJ + 0x1c0c, 0);
            },
        );
    }

    /// Run one generated function as a setup step, with `args` in a0..t1.
    pub(crate) fn run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = SENTINEL_RA_2 as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, SENTINEL_RA_2, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// The main heap's dlmalloc arena over 256 KiB of test RAM (oc69).
    pub(crate) const MEM_HEAP: u32 = 0x004f_f250;

    pub(crate) const SENTINEL_RA_2: u32 = 0x0012_3450;

    pub(crate) fn heap_init(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x40000) as u64, 0x40000],
        );
    }

    // ------------------------------------------------------------ lobby menu

    #[test]
    fn lobby_menu_enter_room_matches() {
        lobby_menu_test(
            ee::s02::f_0017e2f8__generated,
            lobby_menu_enter_room,
            0x0017_e2f8,
            true,
        );
    }

    #[test]
    fn lobby_menu_cancel_challenge_matches() {
        lobby_menu_test(
            ee::s02::f_0017e558__generated,
            lobby_menu_cancel_challenge,
            0x0017_e558,
            false,
        );
    }

    pub(crate) fn lobby_menu_test(gen: GuestFn, hand: GuestFn, salt: u64, enter: bool) {
        check(gen, hand, 8, &|c, seed| {
            let mut r = base_3(c, seed, salt, 4);
            c.g[A0][0] = OBJ_2 as u64;
            c.g[A1][0] = (r.next_u32() % 8) as u64;
            let count = seed % 3;
            w_2(c, OBJ_2 + 8, count);
            let vt = vtable(c, &mut r, &[0x38]);
            for i in 0..count {
                let item = DATA + 0x3000 + 0x100 * i;
                w_2(c, OBJ_2 + 0xc + 4 * i, item);
                w_2(c, item + 0x12c, vt);
            }
            if !enter {
                let vt2 = vtable(c, &mut r, &[0x30]);
                w_2(c, OBJ_2 + 0x1044 + 0x12c, vt2);
            }
        });
    }

    #[test]
    fn lobby_menu_send_chat_matches() {
        check(
            ee::s02::f_0017e418__generated,
            lobby_menu_send_chat,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x17_e418, 4);
                belib(c);
                heap_setup(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x3000) as u64;
                c.pc = 0x0017_e418;
            },
        );
    }

    #[test]
    fn keyboard_screen_cursor_up_matches() {
        check(
            ee::s02::f_001806d8__generated,
            keyboard_screen_cursor_up,
            200,
            &|c, seed| {
                let _ = base_4(c, seed, 0x18_06d8);
                globals_3(c);
                c.bus.write32(DATA, 0);
                let row = match seed % 8 {
                    0 => 0,
                    1 => 9,
                    2 => 10,
                    3 => 19,
                    4 => 18,
                    5 => 8,
                    6 => 39,
                    _ => (seed * 7) % 45,
                };
                w_4(c, DATA + 0x170, row);
                w_4(c, DATA + 0xd8, if seed % 2 == 0 { 0 } else { 0x2222 });
                w_4(c, DATA + 0xe0, if seed % 3 == 0 { 0 } else { 0x3333 });
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn debug_menu_set_game_mode_matches() {
        check(
            ssx3_ee::s02::f_0017d008__generated,
            debug_menu_set_game_mode,
            8,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x17d008, 3);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 4) as u64;
                let world = DATA + 0x8_0000;
                w(c, world + 0x78, DATA + 0x2000);
                c.bus.write8(0x0053_5bc8 + 0x48, (seed % 4) as u8);
                c.bus.write8(0x0053_5bc8 + 0x49, (seed % 3) as u8);
                for i in 0..4u32 {
                    w(c, 0x004a_6ca8 + 4 * i, r.next_u32() % 4);
                }
                let iface = DATA + 0x3000;
                w(c, DATA + 0x30_1000 + 4, iface);
                w(c, iface + 0xc, DATA + 0x3100);
                w(c, DATA + 0x3100 + 8, 0);
                w(c, DATA + 0x3100 + 0xc, 0);
            },
        );
    }

    /// Fewer seeds for tests whose real callees are heavy.
    pub(crate) const FEW: u32 = 16;

    // ---------------------------------------------------------- lobby login

    #[test]
    fn lobby_menu_login_matches() {
        check(
            ee::s02::f_0017e118__generated,
            lobby_menu_login,
            FEW,
            &|c, seed| {
                let _ = base_6(c, seed, 0x17e1_18, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (obj + 0x1000) as u64;
                c.g[A2][0] = (obj + 0x1100) as u64;
                w_5(c, obj + 8, if seed % 3 == 0 { 0 } else { obj + 0x2000 });
                w_5(c, obj + 0x2000, 0);
                w_5(c, obj + 0x1c04, 0);
                w_5(c, obj + 0x1c08, 0);
                wb(c, obj + 0x1000, b'a');
                wb(c, obj + 0x1100, b'b');
            },
        );
    }

    #[test]
    fn debug_menu_module_shutdown_matches() {
        check(
            ee::s02::f_0017fad0__generated,
            debug_menu_module_shutdown,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x17fa, 4);
                game_globals(c);
                c.bus.write32(GAME_GP - 0x854, DATA + 0x16_0000);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let o1 = DATA + 0x2000;
                let o2 = DATA + 0x2400;
                c.bus.write32(this + 0xc, o1);
                c.bus.write32(this + 0x10, o2);
                let vt1 = vtable(c, &mut r, &[8]);
                let vt2 = vtable(c, &mut r, &[8]);
                c.bus.write32(o1, vt1);
                c.bus.write32(o2, vt2);
                c.bus.write32(
                    this + 0x20,
                    if seed % 2 == 0 { 0xff_ffff } else { seed % 4 },
                );
                c.bus.write32(
                    this + 0x1c,
                    if seed % 3 == 0 {
                        0xff_ffff
                    } else {
                        1 + seed % 4
                    },
                );
                c.bus
                    .write32(this + 0x18, if seed % 4 == 0 { 0 } else { DATA + 0x2800 });
                let vt = vtable(c, &mut r, &[0x190]);
                let m = DATA + 0x16_0000;
                c.bus.write32(m + 0x10d8, vt);
            },
        );
    }

    #[test]
    fn keyboard_screen_get_cursor_object_name_matches() {
        strict(
            ee::s02::f_00180a90__generated,
            keyboard_screen_get_cursor_object_name,
            100,
            &|c, seed| {
                let _r = base_7(c, seed, 0x180a);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 0x50) as u64;
                c.g[A2][0] = (DATA + 0x2000) as u64;
                for i in 0..0x20 {
                    c.bus.write8(DATA + 0x2000 + i, 0);
                }
            },
        );
    }

    #[test]
    fn keyboardscreen_get_key_object_name_matches() {
        check(
            ee::s02::f_001809b0__generated,
            keyboardscreen_get_key_object_name,
            16,
            &|c, seed| {
                let _ = base_8(c, seed, 0x1809b0, 2);
                let dst = DATA;
                let out = DATA + 0x100;
                clear_2(c, dst, 0x100);
                clear_2(c, out, 0x100);
                c.g[A0][0] = dst as u64;
                c.g[A1][0] = match seed % 6 {
                    0 => 0,
                    1 => 5,
                    2 => 10,
                    3 => 0x24,
                    4 => 0x25,
                    _ => (seed % 64) as u64,
                };
                c.g[A2][0] = out as u64;
            },
        );
    }

    // --------------------------------------------------------------- lobby

    #[test]
    fn lobby_menu_create_account_matches() {
        strict_2(
            ee::s02::f_0017e030__generated,
            lobby_menu_create_account,
            60,
            &|c, seed| {
                let _ = base_9(c, seed, 0x17e0, 4);
                allocator(c);
                let menu = DATA;
                let g = DATA + 0x3000;
                let buf = DATA + 0x6000;
                c.g[A0][0] = menu as u64;
                c.g[A1][0] = (DATA + 0x900) as u64;
                c.g[A2][0] = (DATA + 0x940) as u64;
                for (i, b) in b"user5\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x900 + i as u32, *b);
                }
                for (i, b) in b"passw0rd\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x940 + i as u32, *b);
                }
                // Pre-sized BXString buffers (capacity and length large
                // enough) so the assigns take the no-allocation path.
                w_7(c, buf.wrapping_sub(0xc), 0x1000);
                w_7(c, buf.wrapping_sub(4), 0x1000);
                w_7(c, menu + 0x1c04, buf);
                w_7(c, menu + 0x1c08, buf);
                w_7(c, GAME_GP.wrapping_sub(0xc8), g);
                for off in [0x260u32, 0x264, 0x268, 0x26c] {
                    w_7(c, g + off, buf);
                }
                // The page list: empty, or one page that MenuSys_PopPage drains.
                if seed % 3 == 0 {
                    let page = DATA + 0x400;
                    let vt = DATA + 0x1000;
                    w_7(c, menu + 8, 1);
                    w_7(c, menu + 0xc, page);
                    w_7(c, page + 0x12c, vt);
                    w_7(c, vt + 0x38, 0);
                    w_7(c, vt + 0x3c, FAKE);
                }
            },
        );
    }

    #[test]
    fn lobby_menu_ctor_matches() {
        check_all(ee::s02::f_0017d298, lobby_menu_ctor, 40, &|c, seed| {
            let _r = alloc_base(c, seed, 0x17d2);
            c.g[A0][0] = DATA as u64;
            let obj = DATA + 0x10_1000;
            c.bus.write32(W_5 + 0x80, obj);
            c.bus.write32(obj + 0x18, seed);
            c.bus.write32(obj + 0x20, seed * 3 + 1);
            c.bus.write32(W_5 + 0xb0, seed ^ 0x55);
            c.bus.write32(gp_at(0xda0), DATA + 0x10_2000);
        });
    }

    #[test]
    fn fe_ui_input_is_any_button_matches() {
        check(
            ee::s02::f_00180300__generated,
            fe_ui_input_is_any_button,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x18_0300, 16);
                c.g[A0][0] = DATA as u64;
                let b = DATA + 0x2000;
                binding(c, b, DATA + 0x3000, 0);
                w_8(c, DATA + 0x18, b);
                let value = if seed % 2 == 0 { 0.0f32 } else { 1.0f32 };
                w_8(c, SP - 0x208, value.to_bits());
            },
        );
    }

    pub(crate) const DBG: u32 = DATA + 0x1_0000;

    pub(crate) const DBG_MENU: u32 = DATA + 0x1_1000;

    #[test]
    fn debug_menu_module_render_matches() {
        check_all_2(
            ee::s02::f_0017fbf0__generated,
            debug_menu_module_render,
            24,
            &|c, seed| {
                let mut r = base_10(c, seed, 0x17fb_f0, 10);
                c.g[A0][0] = sx(DBG);
                c.bus.write32(DBG + 0x14, DBG_MENU);
                c.bus.write32(DBG + 0x1c, seed % 4);
                c.bus.write32(DBG + 0x24, r.next_u32() % 0x40);
                let vt4 = vtable(c, &mut r, &[0x18, 0x1c]);
                c.bus.write32(DBG_MENU, vt4);
                // The render-settings vtable at_4 `gp-0x854 + 0x10d8` (slot 17/15/26/
                // 34/20) stays empty, so those calls go to the fakes.
                if seed % 2 == 0 {
                    let vt = vtable(c, &mut r, &[0x88, 0x8c]);
                    c.bus.write32(AUX_2 + 0x10d8, vt);
                }
            },
        );
    }

    pub(crate) const FLY: u32 = DATA + 0x1_0000;

    #[test]
    fn debug_menu_module_update_cursor_trail_matches() {
        check_all_2(
            ee::s02::f_0017f2d0__generated,
            debug_menu_module_update_cursor_trail,
            24,
            &|c, seed| {
                let mut r = base_10(c, seed, 0x17f2_d0, 8);
                c.g[A0][0] = sx(FLY);
                c.bus.write32(FLY + 0x0, r.edge_f32()); // phase.x
                c.bus.write32(FLY + 0x4, r.edge_f32()); // phase.y
                c.bus.write32(FLY + 0x8, r.edge_f32()); // pos.x
                c.bus.write32(FLY + 0xc, r.edge_f32()); // pos.y
                c.bus.write32(FLY + 0x10, (seed % 64) as u32); // head
            },
        );
    }

    /// `FEUIInput_PollUsbKeyboard`: no keyboard; a keyboard that is closed,
    /// has an empty channel 0, or has a queued key (zero or non-zero key /
    /// modifier bytes), through the real `cUsbKeyboard_GetKey`.
    #[test]
    fn feui_input_poll_usb_keyboard_matches() {
        crate::difftest::check_lohi(
            ssx3_ee::s02::f_001804b8__generated,
            feui_input_poll_usb_keyboard,
            300,
            &|c, seed| {
                let mut r = Rng(0x0018_04b8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0018_04b8;
                let (input, kb) = (DATA + 0x100, DATA + 0x1000);
                fill(c, &mut r, input, 8);
                c.bus.write32(
                    GAME_GP + FEUI_USB_KEYBOARD_GP,
                    if seed % 5 == 0 { 0 } else { kb },
                );
                // cUsbKeyboard: open flag (+0x460), channel 0 count / read
                // index / key ring; the keyboard limit `UsbKb_GetMaxKeyboards`
                // returns.
                c.bus.write32(0x0052_6010, 1 + r.next_u32() % 4);
                c.bus
                    .write32(kb + 0x460, if seed % 7 == 1 { 0 } else { r.next_u32() | 1 });
                let read = r.next_u32() % 64;
                let count = if seed % 3 == 0 {
                    read
                } else {
                    read + 1 + r.next_u32() % 4
                };
                c.bus.write32(kb, count);
                c.bus.write32(kb + 4, read);
                for b in 0..0x80u32 {
                    let v = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32() as u8
                    };
                    c.bus.write8(kb + 8 + b, v);
                }
                c.g[A0][0] = sx(input);
            },
        );
    }

    use crate::difftest::fakes::covered;
    use std::ptr::addr_of_mut;

    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the callee fakes below: they log the call and run the real
    /// body only when this magic word is not in the machine's RAM.
    pub(crate) const OC505_GATE: u32 = DATA + 0xf_e000;

    pub(crate) const OC505_MAGIC: u32 = 0x0505_6a7e;

    /// A callee fake that logs the call (and unwinds when the machine says
    /// so); the gate off runs the callee body it replaced.
    macro_rules! gated {
        ($fake:ident, $gen:path, $addr:expr) => {
            pub(crate) fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC505_GATE) != OC505_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($fake), $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
            }
        };
    }

    /// Installs a fake in a callee's override slot, once per process.
    macro_rules! install {
        ($slot:path, $fake:ident) => {
            fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake));
        };
    }

    gated!(
        oc505_fake_0025f150,
        ee::s06::f_0025f150__generated,
        0x0025_f150
    );

    gated!(
        oc505_fake_0025f1e0,
        ee::s06::f_0025f1e0__generated,
        0x0025_f1e0
    );

    gated!(
        oc505_fake_002ca388,
        ee::s09::f_002ca388__generated,
        0x002c_a388
    );

    gated!(
        oc505_fake_0017e558,
        ee::s02::f_0017e558__generated,
        0x0017_e558
    );

    gated!(
        oc505_fake_0017e490,
        ee::s02::f_0017e490__generated,
        0x0017_e490
    );

    gated!(
        oc505_fake_0017e4f0,
        ee::s02::f_0017e4f0__generated,
        0x0017_e4f0
    );

    gated!(
        oc505_fake_0017e418,
        ee::s02::f_0017e418__generated,
        0x0017_e418
    );

    gated!(
        oc505_fake_0017e3b8,
        ee::s02::f_0017e3b8__generated,
        0x0017_e3b8
    );

    gated!(
        oc505_fake_0017e368,
        ee::s02::f_0017e368__generated,
        0x0017_e368
    );

    gated!(
        oc505_fake_0017e2f8,
        ee::s02::f_0017e2f8__generated,
        0x0017_e2f8
    );

    gated!(
        oc505_fake_0017e290,
        ee::s02::f_0017e290__generated,
        0x0017_e290
    );

    gated!(
        oc505_fake_0017e228,
        ee::s02::f_0017e228__generated,
        0x0017_e228
    );

    gated!(
        oc505_fake_0017e1c0,
        ee::s02::f_0017e1c0__generated,
        0x0017_e1c0
    );

    gated!(
        oc505_fake_0017e030,
        ee::s02::f_0017e030__generated,
        0x0017_e030
    );

    gated!(
        oc505_fake_0017e118,
        ee::s02::f_0017e118__generated,
        0x0017_e118
    );

    gated!(
        oc505_fake_00317e50,
        ee::s10::f_00317e50__generated,
        0x0031_7e50
    );

    gated!(
        oc505_fake_00318260,
        ee::s10::f_00318260__generated,
        0x0031_8260
    );

    gated!(
        oc505_fake_00391c48,
        ee::s12::f_00391c48__generated,
        0x0039_1c48
    );

    gated!(
        oc505_fake_003929f8,
        ee::s13::f_003929f8__generated,
        0x0039_29f8
    );

    gated!(
        oc505_fake_00392b40,
        ee::s13::f_00392b40__generated,
        0x0039_2b40
    );

    /// The count callees return the test's row count (the fake logs the call
    /// as usual; the result is the count the seed picked, -1 included).
    pub(crate) fn oc505_fake_count_room(c: &mut Cpu) {
        if c.bus.read32(OC505_GATE) != OC505_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc505_fake_count_room),
                ee::s06::f_0025f120__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0025_f120, ra);
        c.g[V0][0] = sx(c.bus.read32(OC505_GATE + 4));
    }

    /// The occupant count callee (see [`oc505_fake_count_room`]).
    pub(crate) fn oc505_fake_count_occupants(c: &mut Cpu) {
        if c.bus.read32(OC505_GATE) != OC505_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc505_fake_count_occupants),
                ee::s06::f_0025f1b0__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0025_f1b0, ra);
        c.g[V0][0] = sx(c.bus.read32(OC505_GATE + 4));
    }

    /// Installs every fake in its callee's override slot.
    pub(crate) fn oc505_install_fakes() {
        install!(ee::s06::OVERRIDE_0025F120, oc505_fake_count_room);
        install!(ee::s06::OVERRIDE_0025F1B0, oc505_fake_count_occupants);
        install!(ee::s06::OVERRIDE_0025F150, oc505_fake_0025f150);
        install!(ee::s06::OVERRIDE_0025F1E0, oc505_fake_0025f1e0);
        install!(ee::s09::OVERRIDE_002CA388, oc505_fake_002ca388);
        install!(ee::s02::OVERRIDE_0017E558, oc505_fake_0017e558);
        install!(ee::s02::OVERRIDE_0017E490, oc505_fake_0017e490);
        install!(ee::s02::OVERRIDE_0017E4F0, oc505_fake_0017e4f0);
        install!(ee::s02::OVERRIDE_0017E418, oc505_fake_0017e418);
        install!(ee::s02::OVERRIDE_0017E3B8, oc505_fake_0017e3b8);
        install!(ee::s02::OVERRIDE_0017E368, oc505_fake_0017e368);
        install!(ee::s02::OVERRIDE_0017E2F8, oc505_fake_0017e2f8);
        install!(ee::s02::OVERRIDE_0017E290, oc505_fake_0017e290);
        install!(ee::s02::OVERRIDE_0017E228, oc505_fake_0017e228);
        install!(ee::s02::OVERRIDE_0017E1C0, oc505_fake_0017e1c0);
        install!(ee::s02::OVERRIDE_0017E030, oc505_fake_0017e030);
        install!(ee::s02::OVERRIDE_0017E118, oc505_fake_0017e118);
        install!(ee::s10::OVERRIDE_00317E50, oc505_fake_00317e50);
        install!(ee::s10::OVERRIDE_00318260, oc505_fake_00318260);
        install!(ee::s12::OVERRIDE_00391C48, oc505_fake_00391c48);
        install!(ee::s13::OVERRIDE_003929F8, oc505_fake_003929f8);
        install!(ee::s13::OVERRIDE_00392B40, oc505_fake_00392b40);
    }

    /// A scrambled machine with the callee gate open and an unwind picked
    /// among `calls` fake calls.
    pub(crate) fn oc505_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC505_GATE, OC505_MAGIC);
        r
    }

    /// A row count in -1..m-1 (negative counts take the empty path).
    pub(crate) fn rows(seed: u32, m: u32) -> u32 {
        ((seed % m) as i32 - 1) as u32
    }

    /// A message whose +8 field is `v`; returns its address.
    pub(crate) fn msg(c: &mut Cpu, v: u32) -> u64 {
        let at = DATA + 0x300;
        c.bus.write32(at + 8, v);
        at as u64
    }

    #[test]
    fn lobby_menu_fill_room_list_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001db528__generated,
            lobby_menu_fill_room_list,
            80,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1db5_28, 8);
                c.pc = 0x001d_b528;
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = (DATA + 0x6000) as u64;
                c.bus.write32(OC505_GATE + 4, rows(seed, 15));
            },
        );
    }

    #[test]
    fn lobby_menu_fill_occupant_list_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001db620__generated,
            lobby_menu_fill_occupant_list,
            80,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1db6_20, 8);
                c.pc = 0x001d_b620;
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = (DATA + 0x6000) as u64;
                c.bus.write32(OC505_GATE + 4, rows(seed, 27));
            },
        );
    }

    #[test]
    fn lobby_menu_on_abort_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbab8__generated,
            lobby_menu_on_abort_page_action,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dba_b8, 1);
                c.pc = 0x001d_bab8;
                c.g[A0][0] = (DATA + 0x100) as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_on_challenge_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbad8__generated,
            lobby_menu_on_challenge_page_action,
            80,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dba_d8, 1);
                c.pc = 0x001d_bad8;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = msg(c, [0u32, 1, 2, 9][(seed % 4) as usize]);
            },
        );
    }

    #[test]
    fn lobby_menu_on_chat_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbb20__generated,
            lobby_menu_on_chat_page_action,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dbb_20, 1);
                c.pc = 0x001d_bb20;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_on_occupants_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbb48__generated,
            lobby_menu_on_occupants_page_action,
            80,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dbb_48, 1);
                c.pc = 0x001d_bb48;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = msg(c, [0u32, u32::MAX - 1, u32::MAX, 3][(seed % 4) as usize]);
            },
        );
    }

    #[test]
    fn lobby_menu_on_select_room_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbb90__generated,
            lobby_menu_on_select_room_page_action,
            80,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dbb_90, 1);
                c.pc = 0x001d_bb90;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = msg(c, [0u32, u32::MAX, 0x7fff_ffff, 5][(seed % 4) as usize]);
            },
        );
    }

    #[test]
    fn lobby_menu_on_select_persona_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbbc0__generated,
            lobby_menu_on_select_persona_page_action,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dbb_c0, 1);
                c.pc = 0x001d_bbc0;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_on_create_persona_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbbe8__generated,
            lobby_menu_on_create_persona_page_action,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dbb_e8, 1);
                c.pc = 0x001d_bbe8;
                c.g[A0][0] = (DATA + 0x100) as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_on_delete_persona_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbc10__generated,
            lobby_menu_on_delete_persona_page_action,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dbc_10, 1);
                c.pc = 0x001d_bc10;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_on_create_account_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbc38__generated,
            lobby_menu_on_create_account_page_action,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dbc_38, 1);
                c.pc = 0x001d_bc38;
                c.g[A0][0] = (DATA + 0x100) as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_on_login_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbc68__generated,
            lobby_menu_on_login_page_action,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dbc_68, 1);
                c.pc = 0x001d_bc68;
                c.g[A0][0] = (DATA + 0x100) as u64;
            },
        );
    }

    /// Gate: the fakes below only log when this word is set in the machine's RAM.
    pub(crate) const OC506_GATE: u32 = DATA + 0x0020_0000;

    pub(crate) const OC506_MAGIC: u32 = 0x0c0c_506a;

    /// The fixture object (a page) and the vtable its list entries point at.
    pub(crate) const OC506_THIS: u32 = DATA + 0x0020_0100;

    pub(crate) const OC506_VT: u32 = DATA + 0x0020_0800;

    /// A gated fake for a callee in the function table: logs the call with
    /// [`fake_call`] when armed, otherwise runs what the slot held before.
    macro_rules! oc506_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC506_GATE) != OC506_MAGIC {
                    return fall_back(c, &$prev, $generated);
                }
                fake_call(c, $target);
            }
        };
    }

    oc506_fake!(
        oc506_fake_item,
        OC506_PREV_ITEM,
        0x002c_a280,
        ee::s09::f_002ca280__generated
    );

    oc506_fake!(
        oc506_fake_page,
        OC506_PREV_PAGE,
        0x002c_aa80,
        ee::s09::f_002caa80__generated
    );

    oc506_fake!(
        oc506_fake_delete,
        OC506_PREV_DELETE,
        0x0031_7e50,
        ee::s10::f_00317e50__generated
    );

    oc506_fake!(
        oc506_fake_flag27,
        OC506_PREV_FLAG27,
        0x001e_1090,
        ee::s04::f_001e1090__generated
    );

    oc506_fake!(
        oc506_fake_free_backgrounds,
        OC506_PREV_FREE_BG,
        0x0017_abe0,
        ee::s02::f_0017abe0__generated
    );

    oc506_fake!(
        oc506_fake_get_human,
        OC506_PREV_GET_HUMAN,
        0x001a_0538,
        ee::s03::f_001a0538__generated
    );

    oc506_fake!(
        oc506_fake_get_human1,
        OC506_PREV_GET_HUMAN1,
        0x001a_0548,
        ee::s03::f_001a0548__generated
    );

    /// Install the fakes in their override slots (once per process).
    pub(crate) fn oc506_hook_all() {
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA280),
            oc506_fake_item,
            &OC506_PREV_ITEM,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CAA80),
            oc506_fake_page,
            &OC506_PREV_PAGE,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc506_fake_delete,
            &OC506_PREV_DELETE,
        );
        hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E1090),
            oc506_fake_flag27,
            &OC506_PREV_FLAG27,
        );
        hook(
            addr_of_mut!(ee::s02::OVERRIDE_0017ABE0),
            oc506_fake_free_backgrounds,
            &OC506_PREV_FREE_BG,
        );
        hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A0538),
            oc506_fake_get_human,
            &OC506_PREV_GET_HUMAN,
        );
        hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A0548),
            oc506_fake_get_human1,
            &OC506_PREV_GET_HUMAN1,
        );
    }

    /// Scrambled machine, the gate armed, and a vtable whose function is the
    /// log fake ([`FAKE`]) for the list entries' destructors.
    pub(crate) fn oc506_arm(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC506_GATE, OC506_MAGIC);
        c.bus.write32(OC506_VT + 0x8, 0);
        c.bus.write32(OC506_VT + 0xc, FAKE);
        r
    }

    /// Mark the list entries from `stop` up to `end` (absolute addresses, a
    /// multiple of 0x24 apart) so their vtable pointer leads to [`OC506_VT`].
    pub(crate) fn oc506_entries(c: &mut Cpu, stop: u32, end: u32) {
        let mut p = stop;
        while p < end {
            c.bus.write32(p + 0x10, OC506_VT);
            p += 0x24;
        }
    }

    /// `this` for a page: the fixture object, or (for the guard of a list
    /// that must be empty) an address that makes `this + off` zero.
    pub(crate) fn oc506_this(seed: u32, empty_off: u32) -> u64 {
        if seed % 4 == 3 {
            0u64.wrapping_sub(empty_off as u64)
        } else {
            OC506_THIS as u64
        }
    }

    #[test]
    fn lobby_menu_destroy_login_page_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dcda0__generated,
            lobby_menu_destroy_login_page,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dcda0, 8);
                c.g[A0][0] = OC506_THIS as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_create_account_page_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dce20__generated,
            lobby_menu_destroy_create_account_page,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dce20, 8);
                c.g[A0][0] = OC506_THIS as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_login_method_page_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dcea0__generated,
            lobby_menu_destroy_login_method_page,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dcea0, 24);
                c.g[A0][0] = OC506_THIS as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_delete_persona_page_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dcfc8__generated,
            lobby_menu_destroy_delete_persona_page,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dcfc8, 8);
                let this = oc506_this(seed, 0x168);
                c.g[A0][0] = this;
                c.g[A1][0] = r.next_u32() as u64;
                if this == OC506_THIS as u64 {
                    oc506_entries(c, OC506_THIS + 0x168, OC506_THIS + 0x1f8);
                }
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_create_persona_page_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dd060__generated,
            lobby_menu_destroy_create_persona_page,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dd060, 6);
                c.g[A0][0] = OC506_THIS as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_select_persona_page_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dd0c8__generated,
            lobby_menu_destroy_select_persona_page,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dd0c8, 40);
                c.g[A0][0] = OC506_THIS as u64;
                c.g[A1][0] = r.next_u32() as u64;
                oc506_entries(c, OC506_THIS + 0x518, OC506_THIS + 0x5a8);
                oc506_entries(c, OC506_THIS + 0x450, OC506_THIS + 0x4e0);
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_select_room_page_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dd258__generated,
            lobby_menu_destroy_select_room_page,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dd258, 8);
                let this = oc506_this(seed, 0x168);
                c.g[A0][0] = this;
                c.g[A1][0] = r.next_u32() as u64;
                if this == OC506_THIS as u64 {
                    oc506_entries(c, OC506_THIS + 0x168, OC506_THIS + 0x318);
                }
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_room_occupants_page_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dd2f0__generated,
            lobby_menu_destroy_room_occupants_page,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dd2f0, 8);
                let this = oc506_this(seed, 0x16c);
                c.g[A0][0] = this;
                c.g[A1][0] = r.next_u32() as u64;
                if this == OC506_THIS as u64 {
                    oc506_entries(c, OC506_THIS + 0x16c, OC506_THIS + 0x4cc);
                }
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_chat_page_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dd398__generated,
            lobby_menu_destroy_chat_page,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dd398, 8);
                let this = oc506_this(seed, 0x194);
                c.g[A0][0] = this;
                c.g[A1][0] = r.next_u32() as u64;
                if this == OC506_THIS as u64 {
                    oc506_entries(c, OC506_THIS + 0x194, OC506_THIS + 0x38c);
                }
            },
        );
    }

    /// Gate: the fakes below only log when this word is set in the machine's RAM.
    pub(crate) const OC507_GATE: u32 = DATA + 0x0020_0000;

    pub(crate) const OC507_MAGIC: u32 = 0x0507_0507;

    /// The fixture page and the argument word read by the function.
    pub(crate) const OC507_PAGE: u32 = DATA + 0x0020_1000;

    pub(crate) const OC507_ARG: u32 = DATA + 0x0020_2000;

    /// The fixture persona list read by the delete-persona fill.
    pub(crate) const OC507_OBJ: u32 = DATA + 0x0020_4000;

    /// A gated fake for a callee in the function table: logs the call with
    /// [`fake_call`] when armed, otherwise runs what the slot held before.
    macro_rules! oc507_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC507_GATE) != OC507_MAGIC {
                    return fall_back(c, &$prev, $generated);
                }
                fake_call(c, $target);
            }
        };
    }

    oc507_fake!(
        oc507_fake_page_ctor,
        OC507_PREV_PAGE_CTOR,
        0x002c_aa58,
        ee::s09::f_002caa58__generated
    );

    oc507_fake!(
        oc507_fake_item_ctor,
        OC507_PREV_ITEM_CTOR,
        0x002c_cf98,
        ee::s09::f_002ccf98__generated
    );

    oc507_fake!(
        oc507_fake_activate,
        OC507_PREV_ACTIVATE,
        0x002c_a388,
        ee::s09::f_002ca388__generated
    );

    oc507_fake!(
        oc507_fake_menu_add,
        OC507_PREV_MENU_ADD,
        0x002c_bcc8,
        ee::s09::f_002cbcc8__generated
    );

    /// Install the fakes in their override slots (once per process).
    pub(crate) fn oc507_hook_all() {
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CAA58),
            oc507_fake_page_ctor,
            &OC507_PREV_PAGE_CTOR,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CCF98),
            oc507_fake_item_ctor,
            &OC507_PREV_ITEM_CTOR,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA388),
            oc507_fake_activate,
            &OC507_PREV_ACTIVATE,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CBCC8),
            oc507_fake_menu_add,
            &OC507_PREV_MENU_ADD,
        );
    }

    /// Scrambled machine with the gate armed.
    pub(crate) fn oc507_arm(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        crate::difftest::fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC507_GATE, OC507_MAGIC);
    }

    #[test]
    fn lobby_menu_fill_delete_persona_page_matches() {
        oc507_hook_all();
        covered(
            ee::s04::f_001db330__generated,
            lobby_menu_fill_delete_persona_page,
            200,
            &|c, seed| {
                oc507_arm(c, seed, 0x1db330, 6);
                c.pc = 0x001d_b330;
                c.g[A0][0] = sx(OC507_PAGE);
                c.g[A1][0] = sx(OC507_OBJ);
                let count = [0u32, 1, 2, 3, 5, 9][(seed % 6) as usize];
                c.bus.write32(OC507_OBJ + 0x6c, count);
                for i in 0..10u32 {
                    c.bus.write32(
                        OC507_OBJ + 0x70 + 4 * i,
                        seed.wrapping_mul(0x2545_f491).rotate_left(i),
                    );
                }
            },
        );
    }

    #[test]
    fn lobby_menu_fill_persona_pages_matches() {
        oc507_hook_all();
        covered(
            ee::s04::f_001db418__generated,
            lobby_menu_fill_persona_pages,
            200,
            &|c, seed| {
                oc507_arm(c, seed, 0x1db418, 12);
                c.pc = 0x001d_b418;
                c.g[A0][0] = sx(OC507_PAGE);
                c.g[A1][0] = sx(OC507_OBJ);
                let count = [0u32, 1, 2, 3, 5, 9][(seed % 6) as usize];
                c.bus.write32(OC507_OBJ + 0x6c, count);
                for i in 0..10u32 {
                    c.bus.write32(
                        OC507_OBJ + 0x70 + 4 * i,
                        seed.wrapping_mul(0x9e37_79b9).rotate_left(i + 3),
                    );
                }
            },
        );
    }

    #[test]
    fn lobby_menu_init_challenge_page_matches() {
        oc507_hook_all();
        covered(
            ee::s04::f_001db148__generated,
            lobby_menu_init_challenge_page,
            200,
            &|c, seed| {
                oc507_arm(c, seed, 0x1db148, 9);
                c.pc = 0x001d_b148;
                c.g[A0][0] = sx(OC507_PAGE);
                c.g[A1][0] = sx(OC507_ARG);
                c.g[A2][0] = sx(OC507_OBJ);
                c.bus
                    .write32(OC507_ARG, seed.wrapping_mul(0x2545_f491) ^ 0x31);
                c.bus
                    .write32(OC507_OBJ, seed.wrapping_mul(0x9e37_79b9) ^ 0x77);
            },
        );
    }

    #[test]
    fn lobby_menu_init_abort_page_matches() {
        oc507_hook_all();
        covered(
            ee::s04::f_001db2b0__generated,
            lobby_menu_init_abort_page,
            200,
            &|c, seed| {
                oc507_arm(c, seed, 0x1db2b0, 3);
                c.pc = 0x001d_b2b0;
                c.g[A0][0] = sx(OC507_PAGE);
                c.g[A1][0] = sx(OC507_ARG);
                c.bus
                    .write32(OC507_ARG, seed.wrapping_mul(0x9e37_79b9) ^ 0x55);
            },
        );
    }

    pub(crate) use crate::game::fe::fepopup::tests::oc508_setup;

    #[test]
    fn lobby_menu_destroy_challenge_page_matches() {
        check(
            ee::s04::f_001dd440__generated,
            lobby_menu_destroy_challenge_page,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1dd440);
                c.g[A1][0] = if seed % 2 == 0 { 2 } else { 0 };
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_abort_page_matches() {
        check(
            ee::s04::f_001dd4c0__generated,
            lobby_menu_destroy_abort_page,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1dd4c0);
                c.g[A1][0] = if seed % 2 == 0 { 2 } else { 0 };
            },
        );
    }

    /// `FEUIInput_IsNext`: the button query and its unwinds.
    #[test]
    fn feui_input_is_next_matches() {
        fakes::covered(
            ee::s02::f_0017fec0__generated,
            feui_input_is_next,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x785a, 1);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// Control word of this module's gated fakes (they only act when the
    /// machine's RAM holds the magic here).
    pub(crate) const OC793_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC793_MAGIC: u32 = 0x4f43_7933;

    macro_rules! oc793_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC793_CTRL) != OC793_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc793_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc793_fakes!(
        (
            oc793_fake_320c48,
            OC793_PREV_320C48,
            ee::s10::OVERRIDE_00320C48,
            ee::s10::f_00320c48__generated,
            0x0032_0c48
        ),
        (
            oc793_fake_263660,
            OC793_PREV_263660,
            ee::s07::OVERRIDE_00263660,
            ee::s07::f_00263660__generated,
            0x0026_3660
        ),
        (
            oc793_fake_263998,
            OC793_PREV_263998,
            ee::s07::OVERRIDE_00263998,
            ee::s07::f_00263998__generated,
            0x0026_3998
        ),
        (
            oc793_fake_28b180,
            OC793_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc793_fake_2a1778,
            OC793_PREV_2A1778,
            ee::s08::OVERRIDE_002A1778,
            ee::s08::f_002a1778__generated,
            0x002a_1778
        ),
        (
            oc793_fake_2a16b0,
            OC793_PREV_2A16B0,
            ee::s08::OVERRIDE_002A16B0,
            ee::s08::f_002a16b0__generated,
            0x002a_16b0
        ),
        (
            oc793_fake_374c90,
            OC793_PREV_374C90,
            ee::s12::OVERRIDE_00374C90,
            ee::s12::f_00374c90__generated,
            0x0037_4c90
        ),
        (
            oc793_fake_372b30,
            OC793_PREV_372B30,
            ee::s12::OVERRIDE_00372B30,
            ee::s12::f_00372b30__generated,
            0x0037_2b30
        ),
        (
            oc793_fake_317d70,
            OC793_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc793_fake_2db9c8,
            OC793_PREV_2DB9C8,
            ee::s09::OVERRIDE_002DB9C8,
            ee::s09::f_002db9c8__generated,
            0x002d_b9c8
        ),
        // Callees of the real `cFE_cFE` (0x1956f8) and `cFE_Init` helper
        // (0x2dbd10) that `FEModule_ctor` runs.
        (
            oc793_fake_231cd0,
            OC793_PREV_231CD0,
            ee::s06::OVERRIDE_00231CD0,
            ee::s06::f_00231cd0__generated,
            0x0023_1cd0
        ),
        (
            oc793_fake_15d698,
            OC793_PREV_15D698,
            ee::s01::OVERRIDE_0015D698,
            ee::s01::f_0015d698__generated,
            0x0015_d698
        ),
        (
            oc793_fake_15d700,
            OC793_PREV_15D700,
            ee::s01::OVERRIDE_0015D700,
            ee::s01::f_0015d700__generated,
            0x0015_d700
        ),
        (
            oc793_fake_19cdf0,
            OC793_PREV_19CDF0,
            ee::s03::OVERRIDE_0019CDF0,
            ee::s03::f_0019cdf0__generated,
            0x0019_cdf0
        ),
        (
            oc793_fake_1a03c0,
            OC793_PREV_1A03C0,
            ee::s03::OVERRIDE_001A03C0,
            ee::s03::f_001a03c0__generated,
            0x001a_03c0
        ),
        (
            oc793_fake_397b08,
            OC793_PREV_397B08,
            ee::s13::OVERRIDE_00397B08,
            ee::s13::f_00397b08__generated,
            0x0039_7b08
        ),
        (
            oc793_fake_3e18c8,
            OC793_PREV_3E18C8,
            ee::s14::OVERRIDE_003E18C8,
            ee::s14::f_003e18c8__generated,
            0x003e_18c8
        ),
    );

    /// A scrambled machine with the gated fakes on, `pc` at `entry`.
    pub(crate) fn oc793_machine(c: &mut Cpu, seed: u32, entry: u32, calls: u32) -> Rng {
        oc793_install_fakes();
        let r = machine(c, seed, entry as u64, calls);
        c.pc = entry;
        c.bus.write32(OC793_CTRL, OC793_MAGIC);
        r
    }

    /// `FEUIInput_IsDown`: one forwarded call, answered or unwound.
    #[test]
    fn feui_input_is_down_matches() {
        covered(
            ee::s02::f_00180060__generated,
            feui_input_is_down,
            60,
            &|c, seed| {
                let mut r = oc793_machine(c, seed, 0x0018_0060, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + FEUI_INPUT_BINDING, r.next_u32());
            },
        );
    }

    /// `FEUIInput_IsUp`: one forwarded call, answered or unwound.
    #[test]
    fn feui_input_is_up_matches() {
        covered(
            ee::s02::f_00180040__generated,
            feui_input_is_up,
            60,
            &|c, seed| {
                let mut r = oc793_machine(c, seed, 0x0018_0040, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + FEUI_INPUT_BINDING, r.next_u32());
            },
        );
    }

    pub(crate) const OC798_CTRL: u32 = DATA + 0x30_0000;

    pub(crate) const OC798_MAGIC: u32 = 0x0798_f4ce;

    pub(crate) const OC798_OBJ: u32 = DATA + 0x1000;

    pub(crate) fn oc798_arm(c: &mut Cpu) {
        c.bus.write32(OC798_CTRL, OC798_MAGIC);
    }

    macro_rules! oc798_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC798_CTRL) != OC798_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc798_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc798_fakes!(
        (
            oc798_fk_320c48,
            OC798_PREV_320C48,
            ee::s10::OVERRIDE_00320C48,
            ee::s10::f_00320c48__generated,
            0x0032_0c48
        ),
        (
            oc798_fk_365e40,
            OC798_PREV_365E40,
            ee::s12::OVERRIDE_00365E40,
            ee::s12::f_00365e40__generated,
            0x0036_5e40
        ),
        (
            oc798_fk_3b5910,
            OC798_PREV_3B5910,
            ee::s13::OVERRIDE_003B5910,
            ee::s13::f_003b5910__generated,
            0x003b_5910
        ),
    );

    pub(crate) fn oc798_feui_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        machine(c, seed, salt, 1);
        oc798_arm(c);
        c.pc = entry;
        c.g[RA][0] = sx(crate::difftest::RA);
        c.g[A0][0] = sx(OC798_OBJ);
        c.bus
            .write32(OC798_OBJ + 0x18, DATA + 0x2000 + 0x10 * (seed & 7));
    }

    /// `FEUIInput_IsCamRotate`.
    #[test]
    fn feui_input_is_cam_rotate_matches() {
        oc798_install_fakes();
        fakes::covered(
            ee::s02::f_00180280__generated,
            feui_input_is_cam_rotate,
            60,
            &|c, seed| oc798_feui_setup(c, seed, 0x798d, 0x0018_0280),
        );
    }

    /// `FEUIInput_IsCamPan`.
    #[test]
    fn feui_input_is_cam_pan_matches() {
        oc798_install_fakes();
        fakes::covered(
            ee::s02::f_00180260__generated,
            feui_input_is_cam_pan,
            60,
            &|c, seed| oc798_feui_setup(c, seed, 0x798e, 0x0018_0260),
        );
    }

    /// `FEUIInput_IsCamZoom`.
    #[test]
    fn feui_input_is_cam_zoom_matches() {
        oc798_install_fakes();
        fakes::covered(
            ee::s02::f_00180240__generated,
            feui_input_is_cam_zoom,
            60,
            &|c, seed| oc798_feui_setup(c, seed, 0x798f, 0x0018_0240),
        );
    }
}
